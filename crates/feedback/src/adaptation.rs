//! Adaptation policy engine for theligi-feedback.
//!
//! Versioned policy rules that govern how telemetry deltas propagate to
//! scores, with automatic rollback on metric regression.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A versioned adaptation policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationPolicy {
    pub version: u64,
    pub created_at_ms: i64,
    pub score_updates: ScoreUpdateRules,
    pub rollback: RollbackPolicy,
}

impl AdaptationPolicy {
    pub fn new(version: u64, created_at_ms: i64) -> Self {
        Self {
            version,
            created_at_ms,
            score_updates: ScoreUpdateRules::default(),
            rollback: RollbackPolicy::default(),
        }
    }
}

/// Rules for how telemetry deltas propagate to affinity scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreUpdateRules {
    pub topic_scores: PropagationRule,
    pub platform_affinity: PropagationRule,
    pub audience_affinity: PropagationRule,
}

impl Default for ScoreUpdateRules {
    fn default() -> Self {
        Self {
            topic_scores: PropagationRule::TelemetryDelta,
            platform_affinity: PropagationRule::TelemetryDelta,
            audience_affinity: PropagationRule::TelemetryDelta,
        }
    }
}

/// Propagation strategy for a score type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PropagationRule {
    TelemetryDelta,
    Direct,
    Disabled,
}

/// Rollback policy for adaptation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackPolicy {
    pub trigger: RollbackTrigger,
    pub revert_to: RollbackTarget,
    pub auto_revert: bool,
}

impl Default for RollbackPolicy {
    fn default() -> Self {
        Self {
            trigger: RollbackTrigger::MetricRegression,
            revert_to: RollbackTarget::LastKnownGood,
            auto_revert: true,
        }
    }
}

/// What triggers a policy rollback.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RollbackTrigger {
    MetricRegression,
    ErrorRateSpike,
    Manual,
}

/// Which policy version to revert to.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RollbackTarget {
    LastKnownGood,
    SpecificVersion(u64),
}

/// Current state of an adaptation policy application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationState {
    pub current_policy_version: u64,
    pub last_good_policy_version: u64,
    pub applied_at_ms: i64,
    pub metrics_since_apply: HashMap<String, f64>,
    pub rolled_back: bool,
}

impl AdaptationState {
    pub fn new(version: u64, last_good: u64, applied_at_ms: i64) -> Self {
        Self {
            current_policy_version: version,
            last_good_policy_version: last_good,
            applied_at_ms,
            metrics_since_apply: HashMap::new(),
            rolled_back: false,
        }
    }
}

/// Check whether the current policy should be rolled back.
pub fn should_rollback(
    state: &AdaptationState,
    policy: &AdaptationPolicy,
    current_metrics: &HashMap<String, f64>,
) -> bool {
    if !policy.rollback.auto_revert {
        return false;
    }
    match policy.rollback.trigger {
        RollbackTrigger::MetricRegression => {
            regression_detected(&state.metrics_since_apply, current_metrics)
        }
        RollbackTrigger::ErrorRateSpike => {
            current_metrics.get("error_rate").is_some_and(|v| *v > 0.05)
        }
        RollbackTrigger::Manual => false,
    }
}

/// Detect metric regression by comparing baseline to current.
fn regression_detected(baseline: &HashMap<String, f64>, current: &HashMap<String, f64>) -> bool {
    for (key, baseline_value) in baseline.iter() {
        if let Some(current_value) = current.get(key) {
            if *current_value < baseline_value * 0.8 {
                return true;
            }
        }
    }
    false
}

/// Compute the version to revert to.
pub fn rollback_target(state: &AdaptationState, policy: &AdaptationPolicy) -> u64 {
    match policy.rollback.revert_to {
        RollbackTarget::LastKnownGood => state.last_good_policy_version,
        RollbackTarget::SpecificVersion(v) => v,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_has_telemetry_delta() {
        let policy = AdaptationPolicy::new(1, 1000);
        assert!(matches!(
            policy.score_updates.topic_scores,
            PropagationRule::TelemetryDelta
        ));
    }

    #[test]
    fn detect_regression() {
        let mut baseline = HashMap::new();
        baseline.insert("engagement".to_string(), 100.0);
        let mut current = HashMap::new();
        current.insert("engagement".to_string(), 70.0);
        assert!(regression_detected(&baseline, &current));
    }

    #[test]
    fn no_regression_when_stable() {
        let mut baseline = HashMap::new();
        baseline.insert("engagement".to_string(), 100.0);
        let mut current = HashMap::new();
        current.insert("engagement".to_string(), 95.0);
        assert!(!regression_detected(&baseline, &current));
    }
}

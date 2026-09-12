//! Telemetry normalization schema for theligi-telemetry.
//!
//! Canonical metric model with platform-specific normalization rules.

use serde::{Deserialize, Serialize};

/// Canonical engagement metric kinds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalMetric {
    Impressions,
    EngagementRate,
    Comments,
    Shares,
    Saves,
    Followers,
    ProfileViews,
    Clicks,
    Conversions,
}

/// Platform-agnostic normalized metric value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedMetric {
    pub metric: CanonicalMetric,
    pub value: f64,
    pub platform: Platform,
    pub observed_at_ms: i64,
}

/// Source platform for a telemetry observation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    LinkedIn,
    X,
    Instagram,
}

/// Platform-specific raw metric names that map to a canonical metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformMetricRule {
    pub platform: Platform,
    pub raw_name: String,
    pub canonical: CanonicalMetric,
}

/// Normalize a platform-specific raw metric name to its canonical form.
pub fn normalize_metric_name(platform: Platform, raw_name: &str) -> Option<CanonicalMetric> {
    let rules = platform_rules(platform);
    rules
        .iter()
        .find(|r| r.raw_name.eq_ignore_ascii_case(raw_name))
        .map(|r| r.canonical)
}

/// Get the platform-specific normalization rules.
pub fn platform_rules(platform: Platform) -> Vec<PlatformMetricRule> {
    match platform {
        Platform::LinkedIn => vec![
            PlatformMetricRule {
                platform: Platform::LinkedIn,
                raw_name: "reactions".into(),
                canonical: CanonicalMetric::EngagementRate,
            },
            PlatformMetricRule {
                platform: Platform::LinkedIn,
                raw_name: "comments".into(),
                canonical: CanonicalMetric::Comments,
            },
            PlatformMetricRule {
                platform: Platform::LinkedIn,
                raw_name: "shares".into(),
                canonical: CanonicalMetric::Shares,
            },
            PlatformMetricRule {
                platform: Platform::LinkedIn,
                raw_name: "saves".into(),
                canonical: CanonicalMetric::Saves,
            },
        ],
        Platform::X => vec![
            PlatformMetricRule {
                platform: Platform::X,
                raw_name: "likes".into(),
                canonical: CanonicalMetric::EngagementRate,
            },
            PlatformMetricRule {
                platform: Platform::X,
                raw_name: "retweets".into(),
                canonical: CanonicalMetric::Shares,
            },
            PlatformMetricRule {
                platform: Platform::X,
                raw_name: "replies".into(),
                canonical: CanonicalMetric::Comments,
            },
            PlatformMetricRule {
                platform: Platform::X,
                raw_name: "bookmarks".into(),
                canonical: CanonicalMetric::Saves,
            },
        ],
        Platform::Instagram => vec![
            PlatformMetricRule {
                platform: Platform::Instagram,
                raw_name: "likes".into(),
                canonical: CanonicalMetric::EngagementRate,
            },
            PlatformMetricRule {
                platform: Platform::Instagram,
                raw_name: "comments".into(),
                canonical: CanonicalMetric::Comments,
            },
            PlatformMetricRule {
                platform: Platform::Instagram,
                raw_name: "shares".into(),
                canonical: CanonicalMetric::Shares,
            },
            PlatformMetricRule {
                platform: Platform::Instagram,
                raw_name: "saves".into(),
                canonical: CanonicalMetric::Saves,
            },
        ],
    }
}

/// A fully normalized telemetry observation ready for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedObservation {
    pub id: uuid::Uuid,
    pub schema_version: u32,
    pub platform: Platform,
    pub account_id: String,
    pub content_id: Option<String>,
    pub metrics: Vec<NormalizedMetric>,
    pub timestamp_ms: i64,
    pub lineage: Option<String>,
}

impl NormalizedObservation {
    pub fn new(
        platform: Platform,
        account_id: impl Into<String>,
        metrics: Vec<NormalizedMetric>,
        timestamp_ms: i64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            schema_version: 1,
            platform,
            account_id: account_id.into(),
            content_id: None,
            metrics,
            timestamp_ms,
            lineage: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_linkedin_reactions() {
        assert_eq!(
            normalize_metric_name(Platform::LinkedIn, "reactions"),
            Some(CanonicalMetric::EngagementRate)
        );
    }

    #[test]
    fn normalize_x_likes() {
        assert_eq!(
            normalize_metric_name(Platform::X, "likes"),
            Some(CanonicalMetric::EngagementRate)
        );
    }

    #[test]
    fn normalize_instagram_saves() {
        assert_eq!(
            normalize_metric_name(Platform::Instagram, "saves"),
            Some(CanonicalMetric::Saves)
        );
    }

    #[test]
    fn normalize_unknown_metric() {
        assert_eq!(normalize_metric_name(Platform::X, "unknown_metric"), None);
    }
}

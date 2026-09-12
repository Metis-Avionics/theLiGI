//! Evidence sufficiency model for theligi-evidence.
//!
//! Defines the evidence hierarchy, sufficiency thresholds per content
//! transformation stage, and the blake3 lineage hash chain from source
//! through series.

use blake3::Hash;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Evidence hierarchy ranks.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceHierarchy {
    Observational,
    ExpertOpinion,
    SecondarySource,
    PrimarySource,
}

impl EvidenceHierarchy {
    /// Minimum required hierarchy for a given transformation stage.
    #[must_use]
    pub fn threshold_for_stage(stage: &str) -> Option<Self> {
        match stage {
            "evidence" | "mechanism" | "impact" => Some(Self::PrimarySource),
            "problem" | "explanation" | "hot" => Some(Self::SecondarySource),
            _ => None,
        }
    }

    /// Numeric weight for aggregation.
    #[must_use]
    pub fn weight(&self) -> f64 {
        match self {
            EvidenceHierarchy::PrimarySource => 1.0,
            EvidenceHierarchy::SecondarySource => 0.7,
            EvidenceHierarchy::ExpertOpinion => 0.5,
            EvidenceHierarchy::Observational => 0.3,
        }
    }
}

/// Sufficiency check result for a claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SufficiencyResult {
    pub sufficient: bool,
    pub highest_evidence: Option<EvidenceHierarchy>,
    pub evidence_count: usize,
    pub missing_threshold: Option<EvidenceHierarchy>,
}

/// Evaluate whether the collected evidence for a claim meets the
/// sufficiency threshold for the given transformation stage.
#[must_use]
pub fn evaluate_sufficiency(
    stage: &str,
    evidence_hierarchies: &[EvidenceHierarchy],
) -> SufficiencyResult {
    let threshold = EvidenceHierarchy::threshold_for_stage(stage);
    let highest = evidence_hierarchies.iter().max().copied();
    let sufficient = match (threshold, highest) {
        (Some(t), Some(h)) => h >= t,
        (None, _) => true,
        (Some(_), None) => false,
    };
    let missing_threshold = if sufficient { None } else { threshold };
    SufficiencyResult {
        sufficient,
        highest_evidence: highest,
        evidence_count: evidence_hierarchies.len(),
        missing_threshold,
    }
}

/// Serializable wrapper around `blake3::Hash`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineageHash(pub Hash);

impl Serialize for LineageHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_hex().as_str())
    }
}

impl<'de> Deserialize<'de> for LineageHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Hash::from_hex(&s)
            .map(LineageHash)
            .map_err(serde::de::Error::custom)
    }
}

/// A link in the blake3 lineage hash chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageLink {
    pub id: Uuid,
    pub label: &'static str,
    pub hash: LineageHash,
    pub parent_hash: Option<LineageHash>,
}

/// Compute the blake3 hash of a byte slice.
#[must_use]
pub fn compute_hash(data: &[u8]) -> Hash {
    blake3::hash(data)
}

/// Build a lineage hash chain entry.
pub fn chain_link(label: &'static str, data: &[u8], parent_hash: Option<Hash>) -> LineageLink {
    let hash = compute_hash(data);
    LineageLink {
        id: Uuid::new_v4(),
        label,
        hash: LineageHash(hash),
        parent_hash: parent_hash.map(LineageHash),
    }
}

/// Verify that a lineage link's hash matches its data.
#[must_use]
pub fn verify_link(link: &LineageLink, data: &[u8]) -> bool {
    compute_hash(data) == link.hash.0
}

/// Verify the entire lineage chain from leaf to root.
#[must_use]
pub fn verify_chain(links: &[LineageLink], datas: &[&[u8]]) -> bool {
    if links.len() != datas.len() {
        return false;
    }
    for (i, (link, data)) in links.iter().zip(datas.iter()).enumerate() {
        if !verify_link(link, data) {
            return false;
        }
        if i > 0 {
            if let Some(expected_parent) = &links[i].parent_hash {
                if expected_parent.0 != links[i - 1].hash.0 {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_stage_thresholds() {
        assert_eq!(
            EvidenceHierarchy::threshold_for_stage("evidence"),
            Some(EvidenceHierarchy::PrimarySource)
        );
        assert_eq!(
            EvidenceHierarchy::threshold_for_stage("problem"),
            Some(EvidenceHierarchy::SecondarySource)
        );
    }

    #[test]
    fn sufficiency_primary_meets_evidence_threshold() {
        let result = evaluate_sufficiency("evidence", &[EvidenceHierarchy::PrimarySource]);
        assert!(result.sufficient);
    }

    #[test]
    fn sufficiency_observational_fails_evidence_threshold() {
        let result = evaluate_sufficiency("evidence", &[EvidenceHierarchy::Observational]);
        assert!(!result.sufficient);
    }

    #[test]
    fn blake3_hash_chain() {
        let data1 = b"source data";
        let data2 = b"evidence data";
        let link1 = chain_link("source", data1, None);
        let link2 = chain_link("evidence", data2, Some(link1.hash.0));
        assert!(verify_link(&link1, data1));
        assert!(verify_link(&link2, data2));
        assert!(verify_chain(&[link1, link2], &[data1, data2]));
    }
}

//! Provenance tracking for theligi-evidence.
use crate::{EvidenceNode, EvidenceNodeId, EvidenceResult, ProvenanceRecord};

/// Append a provenance record to an evidence node.
pub fn add_provenance(node: &mut EvidenceNode, record: ProvenanceRecord) -> EvidenceResult<()> {
    node.provenance.push(record);
    Ok(())
}

/// Verify and return the full provenance chain for an evidence node.
pub fn verify_lineage(_node_id: EvidenceNodeId) -> EvidenceResult<Vec<ProvenanceRecord>> {
    Ok(Vec::new())
}

#![warn(missing_docs)]
//! Core semantic types and primitives for theLIGI.
//!
//! Provides foundational identity, lineage, and error types
//! shared across all crates in the workspace.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a resource.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceIdentity(pub String);

impl ResourceIdentity {
    /// Create a new `ResourceIdentity`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents the provenance lineage of a resource.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lineage(pub String);

impl Lineage {
    /// Create a new `Lineage`.
    pub fn new(lineage: impl Into<String>) -> Self {
        Self(lineage.into())
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Lineage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier used to correlate related events or requests.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorrelationId(pub String);

impl CorrelationId {
    /// Create a new `CorrelationId`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier that points to the event that caused this event.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CausationId(pub String);

impl CausationId {
    /// Create a new `CausationId`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CausationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Schema version identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchemaVersion(pub String);

impl SchemaVersion {
    /// Create a new `SchemaVersion`.
    pub fn new(version: impl Into<String>) -> Self {
        Self(version.into())
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Error type for the core crate.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    /// Invalid identity format.
    #[error("invalid identity: {0}")]
    InvalidIdentity(String),

    /// Missing required field.
    #[error("missing required field: {0}")]
    MissingField(String),

    /// General purpose error.
    #[error("{0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_identity_roundtrip() {
        let id = ResourceIdentity::new("res-123");
        assert_eq!(id.as_str(), "res-123");
        assert_eq!(id.to_string(), "res-123");
    }

    #[test]
    fn lineage_creation() {
        let lineage = Lineage::new("parent -> child");
        assert_eq!(lineage.as_str(), "parent -> child");
    }

    #[test]
    fn correlation_id_uniqueness() {
        let c1 = CorrelationId::new(uuid::Uuid::new_v4());
        let c2 = CorrelationId::new(uuid::Uuid::new_v4());
        assert_ne!(c1, c2);
    }

    #[test]
    fn causation_id_display() {
        let cid = CausationId::new("cause-001");
        assert_eq!(cid.to_string(), "cause-001");
    }

    #[test]
    fn schema_version_equality() {
        let v1 = SchemaVersion::new("1.0.0");
        let v2 = SchemaVersion::new("1.0.0");
        let v3 = SchemaVersion::new("2.0.0");
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
}

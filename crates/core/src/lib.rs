#![warn(missing_docs)]
//! Core semantic types and primitives for theLIGI.
//!
//! Inherits canonical type identities from `themql-core` and `daf-core`,
//! then adds theLiGI-specific lineage, schema versioning, and error types.
//!
//! ## Inheritance
//!
//! From **theMQL** (`themql-core`):
//!   - `MessageId`, `CorrelationId`, `CausationId`, `TraceId`
//!   - `Message`, `Subject`, `Operation`, `Payload`, `Metadata`
//!
//! From **theDAF** (`daf-core`):
//!   - `ResourceId`, `UserId`, `CacheEntry`, `Tier`
//!
//! theLiGI extensions:
//!   - `ResourceIdentity`, `Lineage`, `SchemaVersion`
//!   - `CoreError`

pub use daf_core::{CacheEntry, ResourceId, Tier, UserId};
pub use themql_core::{
    CausationId, CorrelationId, Message, MessageId, Metadata, Operation, Payload, Subject, TraceId,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a resource in theLiGI domain.
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
    use uuid::Uuid;

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
    fn correlation_id_from_themql() {
        let cid = CorrelationId::new();
        assert!(cid.0 != Uuid::default());
    }

    #[test]
    fn resource_id_from_daf() {
        let rid = ResourceId::new("resource-1");
        assert_eq!(rid.to_string(), "resource-1");
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

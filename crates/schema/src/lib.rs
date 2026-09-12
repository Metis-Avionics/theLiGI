#![warn(missing_docs)]
//! Spec-derived type definitions for theLIGI schemas.
//!
//! Inherits canonical schema types from `themql-schema` (theMQL),
//! then adds theLiGI-specific resource schema definitions.

use serde::{Deserialize, Serialize};
use theligi_core::{ResourceIdentity, SchemaVersion};

/// Representation of a schema field definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDefinition {
    /// Name of the field.
    pub name: String,
    /// Type of the field.
    pub field_type: FieldType,
    /// Whether the field is required.
    pub required: bool,
}

/// Supported field types in schemas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    /// String type.
    String,
    /// Integer type.
    Integer,
    /// Boolean type.
    Boolean,
    /// Nested object type.
    Object,
    /// Array of another type.
    Array(Box<FieldType>),
}

/// Schema definition for a resource type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSchema {
    /// Identity of the resource this schema describes.
    pub resource_identity: ResourceIdentity,
    /// Schema version.
    pub version: SchemaVersion,
    /// Field definitions.
    pub fields: Vec<FieldDefinition>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_type_equality() {
        assert_eq!(FieldType::String, FieldType::String);
        assert_eq!(
            FieldType::Array(Box::new(FieldType::String)),
            FieldType::Array(Box::new(FieldType::String))
        );
        assert_ne!(FieldType::String, FieldType::Integer);
    }

    #[test]
    fn resource_schema_creation() {
        let schema = ResourceSchema {
            resource_identity: ResourceIdentity::new("user"),
            version: SchemaVersion::new("1.0.0"),
            fields: vec![
                FieldDefinition {
                    name: "id".into(),
                    field_type: FieldType::String,
                    required: true,
                },
                FieldDefinition {
                    name: "name".into(),
                    field_type: FieldType::String,
                    required: true,
                },
            ],
        };
        assert_eq!(schema.fields.len(), 2);
        assert!(schema.fields[0].required);
    }
}

#![warn(missing_docs)]
//! Typed query model with parse, validate, and authorize pipeline.
//!
//! Provides the query types and pipeline stages for query processing.

use serde::{Deserialize, Serialize};
use theligi_core::{CoreError, ResourceIdentity};

/// A parsed query targeting specific resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Target resource identity.
    pub target: ResourceIdentity,
    /// Query constraints.
    pub constraints: Vec<QueryConstraint>,
    /// Requested output fields.
    pub fields: Vec<String>,
}

impl Query {
    /// Create a new `Query`.
    pub fn new(target: ResourceIdentity) -> Self {
        Self {
            target,
            constraints: Vec::new(),
            fields: Vec::new(),
        }
    }
}

/// A single query constraint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryConstraint {
    /// Field to constrain.
    pub field: String,
    /// Constraint operator.
    pub operator: QueryOperator,
    /// Value to compare against.
    pub value: serde_json::Value,
}

/// Operators available in query constraints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryOperator {
    /// Equals.
    Eq,
    /// Not equals.
    Ne,
    /// Greater than.
    Gt,
    /// Less than.
    Lt,
    /// Contains.
    Contains,
}

/// A validated query ready for execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedQuery {
    /// The original parsed query.
    pub query: Query,
    /// Validation errors, if any.
    pub errors: Vec<ValidationError>,
}

impl ValidatedQuery {
    /// Create a new `ValidatedQuery`.
    pub fn new(query: Query) -> Self {
        Self {
            query,
            errors: Vec::new(),
        }
    }

    /// Returns true if the query is valid.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Validation error for a query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Error message.
    pub message: String,
    /// Field or constraint that failed.
    pub field: Option<String>,
}

/// An authorized query ready for execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizedQuery {
    /// The validated query.
    pub query: ValidatedQuery,
    /// Authorization errors, if any.
    pub errors: Vec<AuthorizationError>,
}

impl AuthorizedQuery {
    /// Create a new `AuthorizedQuery`.
    pub fn new(query: ValidatedQuery) -> Self {
        Self {
            query,
            errors: Vec::new(),
        }
    }

    /// Returns true if the query is authorized.
    pub fn is_authorized(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Authorization error for a query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationError {
    /// Error message.
    pub message: String,
    /// Missing permission, if applicable.
    pub missing_permission: Option<String>,
}

/// Result of parsing a raw query string.
#[derive(Debug)]
pub enum ParseResult {
    /// Query parsed successfully.
    Ok(Query),
    /// Query failed to parse.
    Err(CoreError),
}

/// Result of validating a query.
#[derive(Debug)]
pub enum ValidationResult {
    /// Query is valid.
    Ok(ValidatedQuery),
    /// Query has validation errors.
    Err(Vec<ValidationError>),
}

/// Result of authorizing a query.
#[derive(Debug)]
pub enum AuthorizationResult {
    /// Query is authorized.
    Ok(AuthorizedQuery),
    /// Query is not authorized.
    Err(Vec<AuthorizationError>),
}

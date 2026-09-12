#![warn(missing_docs)]
//! Execution context types for requests and operations.
//!
//! Provides typed context objects that flow through the
//! pipeline, carrying identity, authorization, and correlation data.

use serde::{Deserialize, Serialize};
use theligi_core::{CausationId, CorrelationId, ResourceIdentity};

/// Context describing an incoming request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    /// Correlation id for this request.
    pub correlation_id: CorrelationId,
    /// Causation id for this request.
    pub causation_id: CausationId,
    /// Source resource.
    pub source: ResourceIdentity,
    /// Target resource.
    pub target: ResourceIdentity,
}

impl RequestContext {
    /// Create a new `RequestContext`.
    pub fn new(
        correlation_id: CorrelationId,
        causation_id: CausationId,
        source: ResourceIdentity,
        target: ResourceIdentity,
    ) -> Self {
        Self {
            correlation_id,
            causation_id,
            source,
            target,
        }
    }
}

/// Identity context carrying caller identity information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityContext {
    /// Identity of the caller.
    pub identity: ResourceIdentity,
    /// Acting identity, if different from caller.
    pub acting_identity: Option<ResourceIdentity>,
}

impl IdentityContext {
    /// Create a new `IdentityContext`.
    pub fn new(identity: ResourceIdentity) -> Self {
        Self {
            identity,
            acting_identity: None,
        }
    }

    /// Set the acting identity.
    pub fn with_acting_identity(mut self, acting_identity: ResourceIdentity) -> Self {
        self.acting_identity = Some(acting_identity);
        self
    }
}

/// Authorization context for access control decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationContext {
    /// Identity context of the caller.
    pub identity: IdentityContext,
    /// Resource being accessed.
    pub resource: ResourceIdentity,
    /// Required permissions.
    pub permissions: Vec<String>,
}

impl AuthorizationContext {
    /// Create a new `AuthorizationContext`.
    pub fn new(
        identity: IdentityContext,
        resource: ResourceIdentity,
        permissions: Vec<String>,
    ) -> Self {
        Self {
            identity,
            resource,
            permissions,
        }
    }
}

/// Correlation context for linking related operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationContext {
    /// Correlation id.
    pub correlation_id: CorrelationId,
    /// Causation id.
    pub causation_id: CausationId,
    /// Parent lineage, if any.
    pub parent_lineage: Option<String>,
}

impl CorrelationContext {
    /// Create a new `CorrelationContext`.
    pub fn new(correlation_id: CorrelationId, causation_id: CausationId) -> Self {
        Self {
            correlation_id,
            causation_id,
            parent_lineage: None,
        }
    }
}

/// Execution context for a running operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// Request context.
    pub request: RequestContext,
    /// Identity context.
    pub identity: IdentityContext,
    /// Correlation context.
    pub correlation: CorrelationContext,
    /// Execution metadata.
    pub metadata: std::collections::HashMap<String, String>,
}

impl ExecutionContext {
    /// Create a new `ExecutionContext`.
    pub fn new(
        request: RequestContext,
        identity: IdentityContext,
        correlation: CorrelationContext,
    ) -> Self {
        Self {
            request,
            identity,
            correlation,
            metadata: std::collections::HashMap::new(),
        }
    }
}

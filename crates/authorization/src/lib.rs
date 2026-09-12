use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Fail-closed authorization errors.
#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("access denied: {reason}")]
    Denied { reason: String },

    #[error("missing required claim: {claim}")]
    MissingClaim { claim: String },

    #[error("context isolation violation")]
    IsolationViolation,

    #[error("provider error: {0}")]
    ProviderError(String),
}

/// Authorization context isolated to a specific tenant/session.
#[derive(Debug, Clone, Default)]
pub struct AuthorizationContext {
    pub tenant_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub claims: HashMap<String, String>,
}

impl AuthorizationContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tenant(mut self, tenant_id: Uuid) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }

    pub fn with_session(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_claim(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.claims.insert(key.into(), value.into());
        self
    }

    pub fn is_isolated(&self) -> bool {
        self.tenant_id.is_some() && self.session_id.is_some()
    }
}

/// Authorization decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationDecision {
    Allow,
    Deny,
}

/// Authorization provider trait.
#[async_trait::async_trait]
pub trait AuthorizationProvider: Send + Sync + 'static {
    async fn authorize(
        &self,
        context: &AuthorizationContext,
        resource: &str,
        action: &str,
    ) -> Result<AuthorizationDecision, AuthorizationError>;
}

/// BetterAuth.rs provider stub implementing fail-closed semantics.
#[derive(Debug, Default)]
pub struct BetterAuthProvider;

#[async_trait::async_trait]
impl AuthorizationProvider for BetterAuthProvider {
    async fn authorize(
        &self,
        context: &AuthorizationContext,
        resource: &str,
        action: &str,
    ) -> Result<AuthorizationDecision, AuthorizationError> {
        if !context.is_isolated() {
            return Err(AuthorizationError::IsolationViolation);
        }

        let subject = context
            .claims
            .get("sub")
            .ok_or(AuthorizationError::MissingClaim {
                claim: "sub".to_string(),
            })?;

        tracing::trace!(subject = subject, resource, action, "authorization check");

        Ok(AuthorizationDecision::Allow)
    }
}

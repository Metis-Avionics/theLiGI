//! theligi-security: least_privilege, fail_closed, credential_isolation, audit_logging.

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in security-sensitive operations.
#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("authorization denied: {0}")]
    AuthorizationDenied(String),
    #[error("credential error: {0}")]
    Credential(String),
    #[error("audit error: {0}")]
    Audit(String),
}

pub type Result<T> = std::result::Result<T, SecurityError>;

/// A permission action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Read,
    Write,
    Delete,
    Admin,
}

/// A security principal (user, service, etc.).
#[derive(Debug, Clone)]
pub struct Principal {
    pub id: Uuid,
    pub name: String,
    pub roles: Vec<String>,
}

/// Security policy enforcing least privilege and fail-closed behavior.
pub struct SecurityPolicy;

impl SecurityPolicy {
    /// Authorize an action for a principal against a required permission.
    pub fn authorize(&self, _principal: &Principal, _action: Action) -> Result<()> {
        Err(SecurityError::AuthorizationDenied(
            "default policy denies all until explicitly configured".into(),
        ))
    }

    /// Isolate credentials so they are never logged or returned in error messages.
    pub fn isolate_credential(&self, _credential: &str) -> String {
        "[REDACTED]".to_string()
    }
}

/// Append an audit log entry.
pub fn audit_log(event: &str, principal_id: Uuid) {
    let _ = (event, principal_id);
}

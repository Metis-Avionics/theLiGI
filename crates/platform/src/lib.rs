//! theligi-platform: platform contract for social adapters.

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur while interacting with a platform.
#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("authentication failed: {0}")]
    Authentication(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("publish error: {0}")]
    Publish(String),
    #[error("schedule error: {0}")]
    Schedule(String),
    #[error("delete error: {0}")]
    Delete(String),
    #[error("fetch error: {0}")]
    Fetch(String),
    #[error("internal error: {0}")]
    Internal(String),
}

pub type PlatformResult<T> = std::result::Result<T, PlatformError>;

/// Identity information for a platform account.
#[derive(Debug, Clone)]
pub struct Identity {
    pub id: Uuid,
    pub handle: String,
    pub display_name: String,
}

/// Content to be published to a platform.
#[derive(Debug, Clone)]
pub struct Content {
    pub text: String,
    pub media_urls: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Platform contract: all social adapters must implement these operations.
#[async_trait::async_trait]
pub trait Platform {
    /// Authenticate the platform client.
    async fn authenticate(&self) -> PlatformResult<()>;
    /// Resolve the authenticated identity.
    async fn resolve_identity(&self) -> PlatformResult<Identity>;
    /// Publish content immediately.
    async fn publish(&self, content: &Content) -> PlatformResult<Uuid>;
    /// Schedule content for future publishing.
    async fn schedule(
        &self,
        content: &Content,
        when: chrono::DateTime<chrono::Utc>,
    ) -> PlatformResult<Uuid>;
    /// Delete previously published content.
    async fn delete(&self, content_id: Uuid) -> PlatformResult<()>;
    /// Fetch raw content by ID.
    async fn fetch_content(&self, content_id: Uuid) -> PlatformResult<String>;
    /// Fetch interactions (likes, comments, etc.) for content.
    async fn fetch_interactions(&self, content_id: Uuid) -> PlatformResult<Vec<String>>;
    /// Fetch metrics (impressions, engagement, etc.) for content.
    async fn fetch_metrics(
        &self,
        content_id: Uuid,
    ) -> PlatformResult<std::collections::HashMap<String, f64>>;
}

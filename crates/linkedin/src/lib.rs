//! theligi-linkedin: LinkedIn adapter.

use theligi_platform::{Content, Identity, Platform, PlatformError, PlatformResult};
use thiserror::Error;
use uuid::Uuid;

/// LinkedIn-specific errors.
#[derive(Error, Debug)]
pub enum LinkedInError {
    #[error("platform error: {0}")]
    Platform(#[from] PlatformError),
    #[error("LinkedIn API error: {0}")]
    Api(String),
}

pub type Result<T> = std::result::Result<T, LinkedInError>;

/// Stub LinkedIn adapter.
pub struct LinkedInAdapter;

#[async_trait::async_trait]
impl Platform for LinkedInAdapter {
    async fn authenticate(&self) -> PlatformResult<()> {
        Ok(())
    }

    async fn resolve_identity(&self) -> PlatformResult<Identity> {
        unimplemented!("LinkedIn identity resolution not yet implemented")
    }

    async fn publish(&self, _content: &Content) -> PlatformResult<Uuid> {
        unimplemented!("LinkedIn publish not yet implemented")
    }

    async fn schedule(
        &self,
        _content: &Content,
        _when: chrono::DateTime<chrono::Utc>,
    ) -> PlatformResult<Uuid> {
        unimplemented!("LinkedIn schedule not yet implemented")
    }

    async fn delete(&self, _content_id: Uuid) -> PlatformResult<()> {
        unimplemented!("LinkedIn delete not yet implemented")
    }

    async fn fetch_content(&self, _content_id: Uuid) -> PlatformResult<String> {
        unimplemented!("LinkedIn fetch_content not yet implemented")
    }

    async fn fetch_interactions(&self, _content_id: Uuid) -> PlatformResult<Vec<String>> {
        unimplemented!("LinkedIn fetch_interactions not yet implemented")
    }

    async fn fetch_metrics(
        &self,
        _content_id: Uuid,
    ) -> PlatformResult<std::collections::HashMap<String, f64>> {
        unimplemented!("LinkedIn fetch_metrics not yet implemented")
    }
}

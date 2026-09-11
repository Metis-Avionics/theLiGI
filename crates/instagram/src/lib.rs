//! theligi-instagram: Instagram adapter.

use theligi_platform::{Content, Identity, Platform, PlatformError, PlatformResult};
use thiserror::Error;
use uuid::Uuid;

/// Instagram-specific errors.
#[derive(Error, Debug)]
pub enum InstagramError {
    #[error("platform error: {0}")]
    Platform(#[from] PlatformError),
    #[error("Instagram API error: {0}")]
    Api(String),
}

pub type Result<T> = std::result::Result<T, InstagramError>;

/// Stub Instagram adapter.
pub struct InstagramAdapter;

#[async_trait::async_trait]
impl Platform for InstagramAdapter {
    async fn authenticate(&self) -> PlatformResult<()> {
        Ok(())
    }

    async fn resolve_identity(&self) -> PlatformResult<Identity> {
        unimplemented!("Instagram identity resolution not yet implemented")
    }

    async fn publish(&self, _content: &Content) -> PlatformResult<Uuid> {
        unimplemented!("Instagram publish not yet implemented")
    }

    async fn schedule(
        &self,
        _content: &Content,
        _when: chrono::DateTime<chrono::Utc>,
    ) -> PlatformResult<Uuid> {
        unimplemented!("Instagram schedule not yet implemented")
    }

    async fn delete(&self, _content_id: Uuid) -> PlatformResult<()> {
        unimplemented!("Instagram delete not yet implemented")
    }

    async fn fetch_content(&self, _content_id: Uuid) -> PlatformResult<String> {
        unimplemented!("Instagram fetch_content not yet implemented")
    }

    async fn fetch_interactions(&self, _content_id: Uuid) -> PlatformResult<Vec<String>> {
        unimplemented!("Instagram fetch_interactions not yet implemented")
    }

    async fn fetch_metrics(
        &self,
        _content_id: Uuid,
    ) -> PlatformResult<std::collections::HashMap<String, f64>> {
        unimplemented!("Instagram fetch_metrics not yet implemented")
    }
}

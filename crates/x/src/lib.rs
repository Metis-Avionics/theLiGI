//! theligi-x: X/Twitter adapter.

use theligi_platform::{Content, Identity, Platform, PlatformError};
use thiserror::Error;
use uuid::Uuid;

type PlatformResult<T> = std::result::Result<T, PlatformError>;

/// X/Twitter-specific errors.
#[derive(Error, Debug)]
pub enum XError {
    #[error("platform error: {0}")]
    Platform(#[from] PlatformError),
    #[error("X API error: {0}")]
    Api(String),
}

pub type Result<T> = std::result::Result<T, XError>;

/// Stub X/Twitter adapter.
pub struct XAdapter;

#[async_trait::async_trait]
impl Platform for XAdapter {
    async fn authenticate(&self) -> PlatformResult<()> {
        Ok(())
    }

    async fn resolve_identity(&self) -> PlatformResult<Identity> {
        unimplemented!("X identity resolution not yet implemented")
    }

    async fn publish(&self, _content: &Content) -> PlatformResult<Uuid> {
        unimplemented!("X publish not yet implemented")
    }

    async fn schedule(
        &self,
        _content: &Content,
        _when: chrono::DateTime<chrono::Utc>,
    ) -> PlatformResult<Uuid> {
        unimplemented!("X schedule not yet implemented")
    }

    async fn delete(&self, _content_id: Uuid) -> PlatformResult<()> {
        unimplemented!("X delete not yet implemented")
    }

    async fn fetch_content(&self, _content_id: Uuid) -> PlatformResult<String> {
        unimplemented!("X fetch_content not yet implemented")
    }

    async fn fetch_interactions(&self, _content_id: Uuid) -> PlatformResult<Vec<String>> {
        unimplemented!("X fetch_interactions not yet implemented")
    }

    async fn fetch_metrics(
        &self,
        _content_id: Uuid,
    ) -> PlatformResult<std::collections::HashMap<String, f64>> {
        unimplemented!("X fetch_metrics not yet implemented")
    }
}

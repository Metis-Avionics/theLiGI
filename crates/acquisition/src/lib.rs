//! theligi-acquisition: HTTP, browser, and document methods for content acquisition.

use theligi_document::{DocumentParser, PlainTextParser};
use thiserror::Error;

/// Errors that can occur during content acquisition.
#[derive(Error, Debug)]
pub enum AcquisitionError {
    #[error("HTTP error: {0}")]
    Http(#[from] theligi_http::HttpError),
    #[error("Browser error: {0}")]
    Browser(String),
    #[error("Document error: {0}")]
    Document(#[from] theligi_document::DocumentError),
}

pub type Result<T> = std::result::Result<T, AcquisitionError>;

/// Trait for acquiring content from various sources.
#[async_trait::async_trait]
pub trait Acquirer {
    async fn acquire(&self, url: &str) -> Result<String>;
}

/// Default acquirer that fetches via HTTP and parses as plain text.
pub struct DefaultAcquirer;

#[async_trait::async_trait]
impl Acquirer for DefaultAcquirer {
    async fn acquire(&self, url: &str) -> Result<String> {
        let http_content = theligi_http::fetch(url).await?;
        let doc = PlainTextParser.parse(&http_content).await?;
        Ok(doc.text)
    }
}

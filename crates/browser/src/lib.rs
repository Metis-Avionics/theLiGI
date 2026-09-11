//! theligi-browser: WebDriver abstraction.

use thiserror::Error;

/// Errors returned by browser automation.
#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("WebDriver session error: {0}")]
    Session(String),
    #[error("navigation error: {0}")]
    Navigation(String),
}

pub type Result<T> = std::result::Result<T, BrowserError>;

/// Abstraction over a WebDriver provider.
#[async_trait::async_trait]
pub trait BrowserProvider {
    /// Open a page and return the rendered HTML.
    async fn open(&self, url: &str) -> Result<String>;
}

/// No-op browser provider used as a placeholder.
pub struct NoopBrowser;

#[async_trait::async_trait]
impl BrowserProvider for NoopBrowser {
    async fn open(&self, url: &str) -> Result<String> {
        Err(BrowserError::Session(format!(
            "noop browser cannot open {url}"
        )))
    }
}

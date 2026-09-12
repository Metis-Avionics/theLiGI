//! theligi-document: Document parser trait and text/link/metadata extraction.

use thiserror::Error;

/// Errors returned while parsing a document.
#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, DocumentError>;

/// Extracted document content.
#[derive(Debug, Clone)]
pub struct Document {
    /// Plain text extracted from the document.
    pub text: String,
    /// Hyperlinks found in the document.
    pub links: Vec<String>,
    /// Arbitrary metadata (title, author, etc.).
    pub metadata: std::collections::HashMap<String, String>,
}

/// Trait for parsing raw document bytes into structured data.
#[async_trait::async_trait]
pub trait DocumentParser {
    async fn parse(&self, raw: &str) -> Result<Document>;
}

/// Default parser that treats the input as plain text.
pub struct PlainTextParser;

#[async_trait::async_trait]
impl DocumentParser for PlainTextParser {
    async fn parse(&self, raw: &str) -> Result<Document> {
        Ok(Document {
            text: raw.to_string(),
            links: Vec::new(),
            metadata: std::collections::HashMap::new(),
        })
    }
}

/// Convenience parse function for plain text documents.
pub async fn parse(raw: &str) -> Result<Document> {
    PlainTextParser.parse(raw).await
}

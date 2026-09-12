//! theligi-http: reqwest-based HTTP fetcher.

use thiserror::Error;

/// Errors returned by the HTTP fetcher.
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("request failed: {0}")]
    Request(String),
    #[error("response error: status={status}, body={body}")]
    Response { status: u16, body: String },
}

pub type Result<T> = std::result::Result<T, HttpError>;

/// Fetch the body of a URL as a string.
pub async fn fetch(url: &str) -> Result<String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| HttpError::Request(e.to_string()))?;
    let status = response.status().as_u16();
    let body = response
        .text()
        .await
        .map_err(|e| HttpError::Request(e.to_string()))?;
    if status >= 400 {
        return Err(HttpError::Response { status, body });
    }
    Ok(body)
}

//! theligi-server: server entry point.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("server error: {0}")]
    Start(String),
    #[error("shutdown error: {0}")]
    Shutdown(String),
}

pub type Result<T> = std::result::Result<T, ServerError>;

/// Start the server.
pub async fn run() -> Result<()> {
    tracing::info!("theligi-server starting");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

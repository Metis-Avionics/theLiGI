//! theligi-desktop: desktop entry point.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DesktopError {
    #[error("desktop error: {0}")]
    Start(String),
    #[error("shutdown error: {0}")]
    Shutdown(String),
}

pub type Result<T> = std::result::Result<T, DesktopError>;

/// Start the desktop application.
pub async fn run() -> Result<()> {
    tracing::info!("theligi-desktop starting");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

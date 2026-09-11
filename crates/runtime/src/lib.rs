#![warn(missing_docs)]
//! Async runtime orchestration for theLIGI.
//!
//! Provides the runtime abstractions for executing and orchestrating
//! asynchronous operations across the platform.

use std::future::Future;
use theligi_core::CoreError;

/// Trait for runtime-orchestrated tasks.
pub trait RuntimeTask {
    /// Output type of the task.
    type Output;

    /// Execute the task.
    fn execute(&self) -> impl Future<Output = Result<Self::Output, CoreError>> + Send;
}

/// The main runtime orchestrator.
#[derive(Debug, Clone)]
pub struct Runtime {
    /// Name of the runtime instance.
    pub name: String,
}

impl Runtime {
    /// Create a new `Runtime` with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Start the runtime.
    pub async fn start(&self) -> Result<(), CoreError> {
        tracing::info!("starting runtime: {}", self.name);
        Ok(())
    }

    /// Stop the runtime.
    pub async fn stop(&self) -> Result<(), CoreError> {
        tracing::info!("stopping runtime: {}", self.name);
        Ok(())
    }
}

/// Spawn a task on the runtime.
pub async fn spawn<F, T>(task: F) -> anyhow::Result<T>
where
    F: Future<Output = anyhow::Result<T>> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(task).await?
}

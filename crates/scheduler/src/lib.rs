//! theligi-scheduler: async, timezone-aware, idempotent, retryable, dead-letter, human-approval job scheduling.

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur while scheduling jobs.
#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("schedule error: {0}")]
    Schedule(String),
    #[error("execution error: {0}")]
    Execution(String),
    #[error("dead letter: {0}")]
    DeadLetter(String),
    #[error("approval required: {0}")]
    ApprovalRequired(String),
}

pub type Result<T> = std::result::Result<T, SchedulerError>;

/// Status of a scheduled job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    DeadLettered,
    AwaitingApproval,
}

/// A scheduled job definition.
#[derive(Debug, Clone)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub status: JobStatus,
    pub run_at: chrono::DateTime<chrono::Utc>,
    pub retries: u32,
    pub max_retries: u32,
}

impl Job {
    pub fn new(name: impl Into<String>, run_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            status: JobStatus::Pending,
            run_at,
            retries: 0,
            max_retries: 3,
        }
    }
}

/// Trait for scheduling and managing jobs.
#[async_trait::async_trait]
pub trait Scheduler {
    async fn schedule(&self, job: Job) -> Result<Job>;
    async fn cancel(&self, job_id: Uuid) -> Result<()>;
    async fn retry(&self, job_id: Uuid) -> Result<Job>;
    async fn move_to_dead_letter(&self, job_id: Uuid, reason: String) -> Result<()>;
    async fn request_human_approval(&self, job_id: Uuid) -> Result<()>;
}

use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum WorkerError {
    #[error("Task execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Task cancelled")]
    TaskCancelled,
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Scheduler connection failed: {0}")]
    SchedulerConnectionFailed(String),
}

impl From<WorkerError> for Status {
    fn from(error: WorkerError) -> Self {
        match error {
            WorkerError::ExecutionFailed(msg) => Status::internal(msg),
            WorkerError::TaskCancelled => Status::cancelled("Task was cancelled"),
            WorkerError::Internal(msg) => Status::internal(msg),
            WorkerError::SchedulerConnectionFailed(msg) => Status::unavailable(msg),
        }
    }
}

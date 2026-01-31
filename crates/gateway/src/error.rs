use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    
    #[error("Invalid task specification: {0}")]
    InvalidTaskSpec(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Scheduler unavailable")]
    SchedulerUnavailable,
}

impl From<GatewayError> for Status {
    fn from(error: GatewayError) -> Self {
        match error {
            GatewayError::TaskNotFound(msg) => Status::not_found(msg),
            GatewayError::InvalidTaskSpec(msg) => Status::invalid_argument(msg),
            GatewayError::Internal(msg) => Status::internal(msg),
            GatewayError::SchedulerUnavailable => {
                Status::unavailable("Scheduler service is unavailable")
            }
        }
    }
}

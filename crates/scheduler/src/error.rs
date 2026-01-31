use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("Worker registration failed: {0}")]
    RegistrationFailed(String),
    
    #[error("No available workers")]
    NoAvailableWorkers,
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<SchedulerError> for Status {
    fn from(error: SchedulerError) -> Self {
        match error {
            SchedulerError::RegistrationFailed(msg) => Status::invalid_argument(msg),
            SchedulerError::NoAvailableWorkers => Status::resource_exhausted("No available workers"),
            SchedulerError::Internal(msg) => Status::internal(msg),
        }
    }
}

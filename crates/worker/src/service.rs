use std::pin::Pin;
use tonic::{Request, Response, Status};
use tracing::info;

use crate::orchestrator::v1::{
    worker_server::Worker, HealthCheckRequest, HealthCheckResponse, TaskProgress, TaskSpec,
};

pub struct WorkerService;

impl WorkerService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkerService {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl Worker for WorkerService {
    type ExecuteTaskStream = Pin<Box<dyn futures::Stream<Item = Result<TaskProgress, Status>> + Send>>;

    async fn execute_task(
        &self,
        request: Request<TaskSpec>,
    ) -> Result<Response<Self::ExecuteTaskStream>, Status> {
        let spec = request.into_inner();
        
        info!(task_name = %spec.name, "Executing task");

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<TaskProgress, Status>>(32);

        tokio::spawn(async move {
            // Simulate task execution with progress updates
            for i in (0..=100).step_by(10) {
                let progress = TaskProgress {
                    task_id: String::new(),
                    status: 2, // RUNNING
                    progress_percent: i as i32,
                    message: format!("Task progress: {}%", i),
                    timestamp: chrono::Utc::now().timestamp(),
                };

                if tx.send(Ok(progress)).await.is_err() {
                    break;
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }

            // Send completion
            let result = TaskProgress {
                task_id: String::new(),
                status: 3, // COMPLETED
                progress_percent: 100,
                message: "Task completed".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
            };

            let _ = tx.send(Ok(result)).await;
        });

        let output = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output) as Self::ExecuteTaskStream))
    }

    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            healthy: true,
            version: "0.1.0".to_string(),
        }))
    }
}

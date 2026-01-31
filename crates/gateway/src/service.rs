use futures::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tracing::info;

use crate::error::GatewayError;
use crate::orchestrator::v1::{
    gateway_server::Gateway, HealthCheckRequest, HealthCheckResponse, ProgressRequest,
    TaskAccepted, TaskProgress, TaskSpec,
};
use crate::store::TaskStore;

pub struct GatewayService {
    store: TaskStore,
}

impl GatewayService {
    pub fn new(store: TaskStore) -> Self {
        Self { store }
    }
}

#[tonic::async_trait]
impl Gateway for GatewayService {
    async fn submit_task(
        &self,
        request: Request<TaskSpec>,
    ) -> Result<Response<TaskAccepted>, Status> {
        let spec = request.into_inner();

        if spec.name.is_empty() {
            return Err(GatewayError::InvalidTaskSpec(
                "Task name cannot be empty".to_string(),
            )
            .into());
        }

        let task_id = TaskStore::create_task_id();
        self.store.initialize_task(task_id.clone());

        info!(task_id = %task_id, task_name = %spec.name, "Task submitted");

        let response = TaskAccepted {
            task_id,
            timestamp: chrono::Utc::now().timestamp(),
        };

        Ok(Response::new(response))
    }

    type SubscribeProgressStream =
        Pin<Box<dyn Stream<Item = Result<TaskProgress, Status>> + Send>>;

    async fn subscribe_progress(
        &self,
        request: Request<ProgressRequest>,
    ) -> Result<Response<Self::SubscribeProgressStream>, Status> {
        let req = request.into_inner();
        let task_id = req.task_id.clone();

        if task_id.is_empty() {
            return Err(Status::invalid_argument("Task ID cannot be empty"));
        }

        if !self.store.task_exists(&task_id) {
            return Err(GatewayError::TaskNotFound(format!(
                "Task {} not found",
                task_id
            ))
            .into());
        }

        let store = self.store.clone();
        let (tx, rx) = mpsc::channel(32);

        // Spawn task to send existing progress and stream new updates
        tokio::spawn(async move {
            // Send existing progress
            if let Some(progress_list) = store.get_progress(&task_id) {
                for progress in progress_list {
                    if tx.send(Ok(progress)).await.is_err() {
                        break;
                    }
                }
            }

            // For now, we'll simulate streaming by waiting
            // In production, this would be connected to the scheduler
        });

        let output = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output) as Self::SubscribeProgressStream))
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

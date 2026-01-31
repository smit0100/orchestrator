use futures::stream::StreamExt;
use std::pin::Pin;
use tonic::{Request, Response, Status};
use tonic::Streaming;
use tracing::{info, error};
use uuid::Uuid;

use crate::orchestrator::v1::{
    scheduler_server::Scheduler, HealthCheckRequest, HealthCheckResponse, TaskAssignment,
    WorkerResponse,
};
use crate::worker_registry::{Worker, WorkerRegistry};
use crate::task_queue::TaskQueue;

pub struct SchedulerService {
    workers: WorkerRegistry,
    task_queue: TaskQueue,
}

impl SchedulerService {
    pub fn new() -> Self {
        Self {
            workers: WorkerRegistry::new(),
            task_queue: TaskQueue::new(),
        }
    }
}

impl Default for SchedulerService {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl Scheduler for SchedulerService {
    type RegisterWorkerStream =
        Pin<Box<dyn futures::Stream<Item = Result<TaskAssignment, Status>> + Send>>;

    async fn register_worker(
        &self,
        request: Request<Streaming<WorkerResponse>>,
    ) -> Result<Response<Self::RegisterWorkerStream>, Status> {
        let mut stream = request.into_inner();
        let mut worker_id: Option<String> = None;
        let workers = self.workers.clone();
        let _task_queue = self.task_queue.clone();

        // Create response channel
        let (_tx, rx) = tokio::sync::mpsc::channel::<Result<TaskAssignment, Status>>(32);

        tokio::spawn(async move {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(response) => {
                        if let Some(message) = response.message {
                            match message {
                                crate::orchestrator::v1::worker_response::Message::Heartbeat(hb) => {
                                    if worker_id.is_none() {
                                        let id = Uuid::new_v4().to_string();
                                        let worker = Worker {
                                            id: id.clone(),
                                            address: hb.worker_address.clone(),
                                            available_slots: hb.available_slots,
                                        };
                                        workers.register_worker(worker);
                                        info!(worker_id = %id, "Worker registered");
                                        worker_id = Some(id);
                                    }
                                    
                                    // Update available slots
                                    if let Some(ref id) = worker_id {
                                        workers.update_worker_slots(id, hb.available_slots);
                                    }
                                }
                                crate::orchestrator::v1::worker_response::Message::Progress(_) => {
                                    // Handle progress updates
                                }
                                crate::orchestrator::v1::worker_response::Message::Result(_) => {
                                    // Handle task results
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Stream error: {}", e);
                        break;
                    }
                }
            }

            if let Some(id) = worker_id {
                info!(worker_id = %id, "Worker disconnected");
            }
        });

        let output = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output) as Self::RegisterWorkerStream))
    }

    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let workers_count = self.workers.list_workers().len();
        info!(workers_count = workers_count, "Health check");

        Ok(Response::new(HealthCheckResponse {
            healthy: true,
            version: "0.1.0".to_string(),
        }))
    }
}

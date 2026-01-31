use worker::orchestrator::v1::worker_server::WorkerServer;
use worker::service::WorkerService;
use tonic::transport::Server;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = "127.0.0.1:50053".parse()?;
    let worker_service = WorkerService::new();

    println!("Worker service listening on {}", addr);

    Server::builder()
        .add_service(WorkerServer::new(worker_service))
        .serve(addr)
        .await?;

    Ok(())
}

use scheduler::orchestrator::v1::scheduler_server::SchedulerServer;
use scheduler::service::SchedulerService;
use tonic::transport::Server;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = "127.0.0.1:50052".parse()?;
    let scheduler_service = SchedulerService::new();

    println!("Scheduler service listening on {}", addr);

    Server::builder()
        .add_service(SchedulerServer::new(scheduler_service))
        .serve(addr)
        .await?;

    Ok(())
}

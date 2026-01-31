use gateway::orchestrator::v1::gateway_server::GatewayServer;
use gateway::service::GatewayService;
use gateway::store::TaskStore;
use tonic::transport::Server;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = "127.0.0.1:50051".parse()?;
    let store = TaskStore::new();
    let gateway_service = GatewayService::new(store);

    println!("Gateway service listening on {}", addr);

    Server::builder()
        .add_service(GatewayServer::new(gateway_service))
        .serve(addr)
        .await?;

    Ok(())
}

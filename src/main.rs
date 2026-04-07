use tonic::transport::Server;
use instrument::controller_server::ControllerServer;

pub mod instrument {
    tonic::include_proto!("instrument");
}

mod hardware;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let controller = service::MyController::default();

    println!("Instrument Backbone listening on {}", addr);

    Server::builder()
        .add_service(ControllerServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())
}
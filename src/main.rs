use tonic::transport::Server;
use instrument::controller_server::ControllerServer;
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

pub mod instrument {
    tonic::include_proto!("instrument");
}

mod driver;
mod profile;
mod hardware;
mod service;

struct DummyDriver;

#[async_trait]
impl driver::InstrumentDriver for DummyDriver {
    async fn set_voltage(&self, _volts: f32) -> anyhow::Result<()> {
        Ok(())
    }

    async fn read_telemetry(&self) -> anyhow::Result<instrument::TelemetryData> {
        Ok(instrument::TelemetryData {
            current_draw: 0.0,
            temperature: 0.0,
        })
    }

    fn get_name(&self) -> &str {
        "dummy"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    // Build a minimal EquipmentProfile to pass to the controller
    let profile = profile::EquipmentProfile {
        name: "dummy".into(),
        model: "dummy-1".into(),
        category: profile::EquipmentCategory::PowerSupply,
        capabilities: HashMap::new(),
        driver_settings: HashMap::new(),
    };

    let driver = Arc::new(DummyDriver) as Arc<dyn driver::InstrumentDriver>;
    let controller = service::MyController::new(driver, profile);

    println!("Instrument Backbone listening on {}", addr);

    Server::builder()
        .add_service(ControllerServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())
}
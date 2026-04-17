use tonic::{Request, Response, Status};
use crate::instrument::{VoltageRequest, ActionResponse, TelemetryData, Empty};
use crate::instrument::controller_server::Controller;
use tokio_stream::wrappers::ReceiverStream;
use std::pin::Pin;
use tokio_stream::Stream;
use std::sync::Arc;
use crate::driver::InstrumentDriver;
use crate::profile::EquipmentProfile;

pub struct MyController {
    pub driver: Arc<dyn InstrumentDriver>,
    pub profile: EquipmentProfile,
}

impl MyController {
    pub fn new(driver: Arc<dyn InstrumentDriver>, profile: EquipmentProfile) -> Self {
        Self { driver, profile }
    }
}

#[tonic::async_trait]
impl Controller for MyController {
    // 1. Define the stream type (a Pin'd Boxed stream of TelemetryData)
    type StreamTelemetryStream = Pin<Box<dyn Stream<Item = Result<TelemetryData, Status>> + Send>>;

    // 2. Implement the streaming function
    async fn stream_telemetry(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::StreamTelemetryStream>, Status> {
        // Create a channel with a small buffer
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        // Spawn a background task to "poll" your hardware
        tokio::spawn(async move {
            loop {
                let data = TelemetryData {
                    current_draw: 0.45, // Replace with real hardware read
                    temperature: 22.5,
                };

                if tx.send(Ok(data)).await.is_err() {
                    // Client disconnected
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        // Wrap the receiver in a Stream and box it up
        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream) as Self::StreamTelemetryStream))
    }

    async fn set_voltage(
        &self,
        request: Request<VoltageRequest>,
    ) -> Result<Response<ActionResponse>, Status> {
    let req = request.into_inner();
        
    // Safety Check: Look up the "voltage" capability in the profile
    if let Some(limit) = self.profile.capabilities.get("voltage") {
        if req.volts > limit.max {
            return Ok(Response::new(ActionResponse {
                success: false,
                message: format!("Command rejected: {}V exceeds hardware limit of {}V", req.volts, limit.max),
            }));
        }
    }

    // If safe, pass to driver
    self.driver
        .set_voltage(req.volts)
        .await
        .map_err(|e: anyhow::Error| Status::internal(e.to_string()))?;    
    Ok(Response::new(ActionResponse { success: true, message: "Voltage set".into() }))
}
}
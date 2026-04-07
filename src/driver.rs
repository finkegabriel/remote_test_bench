use async_trait::async_trait;
use crate::instrument::TelemetryData;

#[async_trait]
pub trait InstrumentDriver: Send + Sync {
    // Methods for controlling hardware
    async fn set_voltage(&self, volts: f32) -> anyhow::Result<()>;
    
    // Methods for reading hardware
    async fn read_telemetry(&self) -> anyhow::Result<TelemetryData>;
    
    // Optional: Information about the hardware
    fn get_name(&self) -> &str;
}
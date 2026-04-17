use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EquipmentProfile {
    pub name: String,
    pub model: String,
    pub category: EquipmentCategory,
    pub capabilities: HashMap<String, CapabilityRange>,
    pub driver_settings: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EquipmentCategory {
    PowerSupply,
    Multimeter,
    Oscilloscope,
    SignalGenerator,
    FunctionGenerator,
    ProtocalAnalyzer,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityRange {
    pub min: Option<f32>,
    pub max: f32,
    pub step: Option<f32>,
}

impl EquipmentProfile {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let f = std::fs::File::open(path)?;
        let profile = serde_yaml::from_reader(f)?;
        Ok(profile)
    }
}
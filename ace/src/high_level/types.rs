use std::collections::HashMap;

use crate::low_level::types::BaseSetting;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum SimulationDataType {
    REGULAR,
}
#[derive(Debug, Serialize)]
pub enum SimulationState {
    ON,
    OFF,
    VERIFY,
}

#[derive(Debug, Serialize)]
pub struct SimulationData<'a, 'b> {
    #[serde(rename = "type")]
    pub s_type: SimulationDataType,
    pub settings: SimulationSetting<'a, 'b>,
    pub regular: String,
}

#[derive(Debug, Serialize)]
pub struct SimulationSetting<'a, 'b> {
    #[serde(flatten)]
    pub base_setting: &'a BaseSetting,
    #[serde(flatten)]
    pub simulation_param: &'b SimulationParam,
}
#[derive(Debug, Serialize)]
pub struct SimulationParam {
    pub language: String,
    pub neutralization: String,
    pub decay: u8,
    pub truncation: f32,
    pub pasteurization: SimulationState,
    #[serde(rename = "unitHandling")]
    pub unit_handling: SimulationState,
    #[serde(rename = "nanHandling")]
    pub nan_handling: SimulationState,
    pub visualization: bool,
    #[serde(flatten)]
    pub extra_param: HashMap<String, serde_json::Value>,
}

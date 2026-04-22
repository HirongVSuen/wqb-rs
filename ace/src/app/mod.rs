pub mod runner;
pub use runner::Runner;

use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use tokio_stream::Stream;

/// Executes brain simulations.
pub trait BrainExecutor: Sync {
    // fn simulate_vec<T: Stream>(
    //     &self, data: &[SimulationData],
    // ) -> impl std::future::Future<Output = Result<T>> + Send;
    //
    fn simulate_vec(
        &self, data: &[SimulationData],
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

/// Stores simulation results.
pub trait DB {
    fn get_new_data(
        &self, data: Vec<SimulationData<'_, '_>>,
    ) -> impl std::future::Future<Output = Result<Vec<SimulationData<'_, '_>>>> + Send;
}

/// Generates simulation data.
pub trait Strategy {
    fn generate(
        &self, brain: &impl BrainExecutor,
    ) -> impl std::future::Future<Output = Result<Vec<SimulationData>>> + Send;
}

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

#[derive(Debug, Serialize)]
pub struct BaseSetting {
    pub delay: u8,
    #[serde(rename = "instrumentType")]
    pub instrument_type: String,
    pub region: String,
    pub universe: String,
}

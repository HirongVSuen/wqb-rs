use crate::app::Strategy;

#[derive(Default)]
pub struct SimpleStrategy {}

impl Strategy for SimpleStrategy {
    fn generate(
        &self, brain_executor: &impl crate::app::BrainExecutor,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<crate::app::SimulationData<'_, '_>>>> + Send
    {
        async move {
            brain_executor;
            Ok(Vec::new())
        }
    }
}

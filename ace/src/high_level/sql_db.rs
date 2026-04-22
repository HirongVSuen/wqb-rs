use crate::app::DB;

#[derive(Default)]
pub struct SqlDb;

impl DB for SqlDb {
    fn get_new_data(
        &self, data: Vec<crate::app::SimulationData<'_, '_>>,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<crate::app::SimulationData<'_, '_>>>> + Send
    {
        async move {
            println!("{:?}", data);
            Ok(Vec::new())
        }
    }
}

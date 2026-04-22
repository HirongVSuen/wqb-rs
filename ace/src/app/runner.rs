use super::SimulationData;
use crate::app::{BrainExecutor, DB, Strategy};
use anyhow::{Ok, Result};
use tracing::{Level, event};

pub struct Runner<T: BrainExecutor, E: DB, S: Strategy> {
    pub executor: T,
    pub db: E,
    pub strategy: S,
}

impl<T: BrainExecutor, E: DB, S: Strategy> Runner<T, E, S> {
    pub async fn run(&self) -> Result<()> {
        // 1. 根据策略生成alpha 模拟运算集
        event!(Level::INFO, "生成alpha 模拟运算集");
        let simulation_data_vec: Vec<SimulationData> =
            self.strategy.generate(&self.executor).await?;

        // 2. 筛选出未运算过的运算集
        event!(Level::INFO, "筛选未完成的运算集");
        let pending_data = self.db.get_new_data(simulation_data_vec).await?;

        if pending_data.is_empty() {
            event!(Level::INFO, "未找到未完成的运算集");
            event!(Level::INFO, "结束本次运算");
            return Ok(());
        }
        // 3. 运行模拟运算
        self.executor.simulate_vec(&pending_data).await?;
        // 4. 保存运算结果
        Ok(())
    }
}

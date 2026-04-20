use std::collections::HashMap;

use ace::high_level::BrainClientConfig;
use ace::high_level::types::{SimulationDataType, SimulationParam, SimulationState};
use ace::{high_level::BrainClient, low_level::types::BaseSetting};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建client并自动登录
    let config = BrainClientConfig { auth_file_path: "/home/hirong/test_auth.json".to_string() };
    let client = BrainClient::new(config)?;
    client.auto_login().await?;

    // 获取alpha运行参数
    let (base_setting, param) = get_settings();

    // 获取数据集 数据字段
    let df = client.data_field_df_by_dataset(&base_setting, "fundamental23").await?;
    let field_ids: Vec<String> =
        df.column("id")?.str()?.into_no_null_iter().map(|val| val.to_string()).collect();
    let mut results = Vec::new();

    for val in field_ids {
        println!("simulation:{}", val);

        // 运行alpha
        match client.simulation(SimulationDataType::REGULAR, &base_setting, &param, val).await {
            Ok(alpha_id) => {
                println!("alpha_id: {}", alpha_id);
                results.push(alpha_id);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
    Ok(())
}

fn get_settings() -> (BaseSetting, SimulationParam) {
    // 额外运行参数
    let extra_param = HashMap::from([
        ("maxTrade".to_string(), Value::from("OFF")),
        ("maxPosition".to_string(), Value::from("OFF")),
        ("testPeriod".to_string(), Value::from("P1Y")),
    ]);

    // 运行参数设置
    let param = SimulationParam {
        language: "FASTEXPR".to_string(),
        neutralization: "SUBINDUSTRY".to_string(),
        decay: 0,
        truncation: 0.08,
        pasteurization: SimulationState::ON,
        unit_handling: SimulationState::VERIFY,
        nan_handling: SimulationState::OFF,
        visualization: false,
        extra_param: extra_param,
    };

    // 区域设置
    let base_setting = BaseSetting {
        delay: 1,
        instrument_type: "EQUITY".to_string(),
        region: "USA".to_string(),
        universe: "TOP500".to_string(),
    };
    (base_setting, param)
}

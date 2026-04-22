use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::JsonReader;
use std::io::Cursor;
use tracing::{Level, event};

use crate::app::*;
use crate::err::StatusCodeDisplay;
use crate::err::{ApiClientError, BrainClientError};
use crate::low_level::ApiClient;
use crate::low_level::types::{AuthenticationInfo, SignInInfo};

/// BrainClient 结果类型
pub type BrainClientResult<T> = std::result::Result<T, BrainClientError>;

/// Configuration for the BrainClient.
pub struct BrainClientConfig {
    pub auth_file_path: String,
}

/// A client for interacting with the Brain API.
/// high level client
pub struct BrainClient {
    config: BrainClientConfig,
    api_client: ApiClient,
}

impl BrainClient {
    /// 创建一个新的 BrainClient。
    pub fn new(config: BrainClientConfig) -> BrainClientResult<Self> {
        Ok(Self { config, api_client: ApiClient::new()? })
    }

    /// 从凭据文件中加载凭据并登录。
    pub async fn auto_login(&self) -> BrainClientResult<()> {
        let path = std::path::Path::new(&self.config.auth_file_path);
        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let sign_info: SignInInfo = serde_json::from_str(&contents)?;
            self.api_client.sign_in(&sign_info).await?;
        } else {
            return Err(BrainClientError::CredentialsNotFound());
        }
        Ok(())
    }

    /// 使用用户名和密码登录。 成功登录后会保存凭据。
    pub async fn login(&self, user_name: &str, password: &str) -> BrainClientResult<()> {
        let sign_info = SignInInfo { email: user_name.to_string(), password: password.to_string() };
        self.api_client.sign_in(&sign_info).await.map_err(|err| {
            // 处理登录失败的情况,401 时返回登录失败，提示用户名或密码错误
            if let ApiClientError::ResponseError {
                status: StatusCodeDisplay(Some(reqwest::StatusCode::UNAUTHORIZED)),
                ..
            } = &err
            {
                event!(Level::ERROR, "Login failed: unauthorized");
                return BrainClientError::LoginFailed();
            }
            BrainClientError::ApiClientError(err)
        })?;
        if let Err(err) = self.save_credentials(&sign_info).await {
            event!(Level::ERROR, "Failed to save credentials: {:?}", err);
        }
        Ok(())
    }

    /// 将凭据保存到文件中。
    async fn save_credentials(&self, sign_info: &SignInInfo) -> BrainClientResult<()> {
        let contents = serde_json::to_string(&sign_info)?;
        std::fs::write(&self.config.auth_file_path, contents)?;
        Ok(())
    }

    /// 根据数据集id获取字段
    pub async fn data_field_df_by_dataset(
        &self, base_setting: &BaseSetting, dataset_id: &str,
    ) -> BrainClientResult<DataFrame> {
        let query = &[("dataset.id", dataset_id)];
        let result = self.data_field_df(base_setting, query).await?;
        Ok(result)
    }

    /// 获取数据集字段
    pub async fn data_field_df(
        &self, base_setting: &BaseSetting, query: &[(&str, &str)],
    ) -> BrainClientResult<DataFrame> {
        let limit = 50;
        let offset = 0;

        let mut fields_qry =
            self.api_client.data_fields(base_setting, limit, offset, query).await?;
        let count = fields_qry["count"]
            .as_u64()
            .map(|val| val as u32)
            .ok_or(BrainClientError::NotFoundField("count".to_owned()))?;
        let results = fields_qry["results"]
            .as_array_mut()
            .ok_or(BrainClientError::NotFoundField("results".to_owned()))?;
        for x in (limit..count).step_by(limit as usize) {
            let mut fields_x_qry =
                self.api_client.data_fields(base_setting, limit, x, query).await?;
            let results_x = fields_x_qry["results"]
                .as_array_mut()
                .ok_or(BrainClientError::NotFoundField("results".into()))?;
            results.append(results_x);
        }
        if results.is_empty() {
            return Ok(DataFrame::empty());
        }
        let json_string = serde_json::to_string(results)?;
        let df = JsonReader::new(Cursor::new(json_string)).finish()?;
        Ok(df)
    }

    pub fn build_sumlation_data(
        s_type: SimulationDataType, base: &BaseSetting, param: &SimulationParam, regular: String,
    ) -> BrainClientResult<String> {
        let settings = SimulationSetting { base_setting: base, simulation_param: param };
        let data = SimulationData { s_type, settings, regular };
        let data = serde_json::to_string(&data)?;
        event!(Level::INFO, "create simulation data: {:?}", &data);
        Ok(data)
    }

    /// 进行模拟计算，返回模拟结果的 alpha_id
    pub async fn simulation(
        &self, s_type: SimulationDataType, base: &BaseSetting, param: &SimulationParam,
        regular: String,
    ) -> BrainClientResult<String> {
        let data = Self::build_sumlation_data(s_type, base, param, regular)?;
        let simulation_id = self.api_client.post_simulations(&data).await?;
        let simulation_result;
        loop {
            let (ra, srslt) = self.api_client.get_simulations(&simulation_id).await?;
            if ra == 0 {
                simulation_result = srslt;
                break;
            }
            event!(Level::INFO, "simulation will retry in {:?} s ", ra);
            tokio::time::sleep(tokio::time::Duration::from_secs(ra)).await;
        }
        let alpha_id = simulation_result["alpha"]
            .as_str()
            .ok_or(BrainClientError::NotFoundField("alpha".to_string()))?;
        Ok(alpha_id.to_owned())
    }

    pub async fn auth_info(&self) -> BrainClientResult<AuthenticationInfo> {
        let auth_info = self.api_client.get_authentication().await?;
        Ok(auth_info)
    }
}

impl BrainExecutor for BrainClient {
    fn simulate_vec(
        &self, data: &[SimulationData],
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        async { anyhow::Ok(()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    fn default_settings() -> (BaseSetting, SimulationParam) {
        // 额外运行参数
        let extra_param = HashMap::from([
            ("maxTrade".to_string(), serde_json::Value::from("OFF")),
            ("maxPosition".to_string(), serde_json::Value::from("OFF")),
            ("testPeriod".to_string(), serde_json::Value::from("P1Y")),
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

    fn base_config() -> BrainClientConfig {
        BrainClientConfig { auth_file_path: "/home/hirong/test_auth.json".to_string() }
    }

    async fn new_auto_login_client() -> BrainClientResult<BrainClient> {
        let client = BrainClient::new(base_config())?;
        client.auto_login().await?;
        Ok(client)
    }

    async fn new_client() -> BrainClientResult<BrainClient> {
        let client = BrainClient::new(base_config())?;
        Ok(client)
    }

    #[tokio::test]
    async fn test_login() -> Result<(), BrainClientError> {
        let client_result = new_auto_login_client().await;
        if let Err(_) = client_result {
            println!("票据登录失败！请重新登录");
            let client_result = new_client().await?;
            let login_result = client_result.login("xxxx@xxxx.com", "xxxx").await;
            assert!(login_result.is_ok(), "login should be Ok");
        } else {
            let auth_info = client_result?.auth_info().await;
            assert!(auth_info.is_ok(), "auth_info should be Ok");
        }
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_build_sumlation_data() -> Result<(), BrainClientError> {
        let (base, param) = default_settings();
        let regular = "anl10_cpsff_2371".to_string();
        let client_result = new_auto_login_client().await?;
        let data =
            client_result.simulation(SimulationDataType::REGULAR, &base, &param, regular).await;
        assert!(data.is_ok());
        let data = data.unwrap();
        print!("{:?}", data);
        Ok(())
    }
}

use std::time::Duration;

use super::types::*;
use crate::app::BaseSetting;
use crate::err::ApiClientError;
use reqwest::{Client, Method, Response, StatusCode, Url, header};
use serde_json::Value;
use tracing::{Level, event};

/// API客户端结果类型
pub type ApiClientResult<T> = std::result::Result<T, ApiClientError>;

/// WorldQuant Brain API客户端
pub struct ApiClient {
    client: Client,
    max_retries: usize,
    retry_delay: Duration,
    base_url: Url,
}

impl ApiClient {
    /// 创建一个新的API客户端
    pub fn new() -> ApiClientResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .pool_max_idle_per_host(10)
            .cookie_store(true)
            .build()?;
        let base_url = Url::parse("https://api.worldquantbrain.com")?;
        Ok(Self { client, max_retries: 6, retry_delay: Duration::from_secs(60), base_url })
    }

    /// 执行请求，支持重试
    async fn execute<F, Fut, E>(&self, mut req_factory: F) -> ApiClientResult<Response>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<Response, E>>,
        E: Into<ApiClientError>,
    {
        for _ in 0..=self.max_retries {
            let response = req_factory().await.map_err(Into::into)?;
            if response.status() == StatusCode::TOO_MANY_REQUESTS {
                tokio::time::sleep(self.retry_delay).await;
                continue;
            }
            return Ok(response);
        }
        Err(ApiClientError::MaxRetriesExceeded)
    }

    /// 登录
    pub async fn sign_in(&self, sign_in_info: &SignInInfo) -> ApiClientResult<()> {
        let url = self.base_url.join("authentication")?;
        let _response = self
            .execute(|| {
                self.client
                    .post(url.clone())
                    .basic_auth(&sign_in_info.email, Some(&sign_in_info.password))
                    .send()
            })
            .await?;
        let response =
            _response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "sign_in".to_string(),
                status: err.status().into(),
            })?;
        event!(Level::INFO, "login user:{}", response.text().await?);
        Ok(())
    }

    /// 获取当前登录用户信息
    pub async fn get_authentication(&self) -> ApiClientResult<AuthenticationInfo> {
        let url = self.base_url.join("authentication")?;

        let response = self.execute(|| self.client.get(url.clone()).send()).await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "get_authentication".to_string(),
                status: err.status().into(),
            })?;

        let result: AuthenticationInfo = response.json().await?;
        event!(Level::TRACE, "get_authentication: {:?}", &result);
        Ok(result)
    }

    /// 登出
    pub async fn delete_authentication(&self) -> ApiClientResult<()> {
        let url = self.base_url.join("authentication")?;

        let response = self.execute(|| self.client.delete(url.clone()).send()).await?;
        response.error_for_status().map_err(|err| ApiClientError::ResponseError {
            api_name: "delete_authentication".to_string(),
            status: err.status().into(),
        })?;
        Ok(())
    }

    /// 获取有关可用属性、其类型、要求和允许值的详细信息。
    pub async fn option_simulations(&self) -> ApiClientResult<Value> {
        let url = self.base_url.join("simulations")?;
        let response =
            self.execute(|| self.client.request(Method::OPTIONS, url.clone()).send()).await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "option_simulations".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::TRACE, "option_simulations: {:?}", &result);
        Ok(result)
    }

    /// Post a new simulation
    pub async fn post_simulations(&self, simulation_obj: &str) -> ApiClientResult<String> {
        let url = self.base_url.join("simulations")?;

        let response = self
            .execute(|| {
                self.client
                    .post(url.clone())
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(simulation_obj.to_string())
                    .send()
            })
            .await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "post_simulations".to_string(),
                status: err.status().into(),
            })?;

        let location = response
            .headers()
            .get(header::LOCATION)
            .ok_or_else(|| {
                ApiClientError::BussinessError(
                    "post_simulations".to_string(),
                    "Location header not found".to_string(),
                )
            })?
            .to_str()
            .map_err(|_| {
                ApiClientError::BussinessError(
                    "post_simulations".to_string(),
                    "Location header contains invalid UTF-8".to_string(),
                )
            })?;

        // 解析 URL 并提取最后一段作为 ID
        let simulations_url = Url::parse(location)?;

        let simulations_id = simulations_url
            .path_segments()
            .ok_or_else(|| {
                ApiClientError::BussinessError(
                    "post_simulations".to_string(),
                    "Location URL cannot be a base (no path segments)".to_string(),
                )
            })?
            .rfind(|s| !s.is_empty())
            .ok_or_else(|| {
                ApiClientError::BussinessError(
                    "post_simulations".to_string(),
                    "Location header path is empty".to_string(),
                )
            })?;
        Ok(simulations_id.to_string())
    }

    /// Get a simulation by id
    pub async fn get_simulations(&self, simulation_id: &str) -> ApiClientResult<(u64, Value)> {
        let url = self.base_url.join(&format!("simulations/{}", simulation_id))?;

        let response = self.execute(|| self.client.get(url.clone()).send()).await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "get_simulations".to_string(),
                status: err.status().into(),
            })?;
        let retry_after = response.headers().get("Retry-After");
        if let Some(retry_after) = retry_after {
            let retry_after = retry_after.to_str().unwrap_or("1");
            event!(Level::INFO, "get_simulations: retry_after: {}", retry_after);
            let retry_after = retry_after.parse::<f64>().unwrap_or(10.0);
            event!(Level::INFO, "get_simulations: retry_after: {}", retry_after);
            let result = response.json().await?;
            event!(Level::INFO, "get_simulations: {:?}", &result);
            return Ok((retry_after as u64, result));
        }
        let result = response.json().await?;
        event!(Level::INFO, "get_simulations: {:?}", &result);

        Ok((0, result))
    }

    /// Get an alpha by id
    pub async fn alphas(&self, alpha_id: &str) -> ApiClientResult<Value> {
        let url = self.base_url.join(&format!("alphas/{}", alpha_id))?;
        let response = self.execute(|| self.client.get(url.clone()).send()).await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "get_alphas".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::INFO, "get_alphas: {:?}", &result);
        Ok(result)
    }

    /// Get an alpha recordsets by id
    pub async fn alpha_recordsets(&self, alpha_id: &str) -> ApiClientResult<Value> {
        let url = self.base_url.join(&format!("alphas/{}/recordsets", alpha_id))?;

        let response = self.execute(|| self.client.get(url.clone()).send()).await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "get_alpha_recordsets".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::TRACE, "get_alpha_recordsets: {:?}", &result);
        Ok(result)
    }

    /// Set the name of a recordset by recordset name
    pub async fn alpha_recordsets_name(
        &self, alpha_id: &str, name: &str,
    ) -> ApiClientResult<Value> {
        let url = self.base_url.join(&format!("alphas/{}/recordsets/{}", alpha_id, name))?;

        let response = self.execute(|| self.client.get(url.clone()).send()).await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "alpha_recordsets_setname".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::TRACE, "alpha_recordsets_name: {:?}", &result);
        Ok(result)
    }

    /// Get the diversities of a user's activities
    pub async fn user_activities_diversities(&self) -> ApiClientResult<Value> {
        let authentication_info = self.get_authentication().await?;
        let url = self
            .base_url
            .join(&format!("users/{}/activities/diversity", authentication_info.user.id))?;

        let response = self.execute(|| self.client.get(url.clone()).send()).await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "user_activities_diversities".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::TRACE, "user_activities_diversities: {:?}", &result);
        Ok(result)
    }

    /// Get a list of data sets.
    pub async fn data_sets(
        &self, settings: &BaseSetting, limit: u32, offset: u32, query: &[(&str, &str)],
    ) -> ApiClientResult<Value> {
        let url = self.base_url.join("data-sets")?;

        let response = self
            .execute(|| {
                self.client
                    .get(url.clone())
                    .query(settings)
                    .query(&[("limit", limit), ("offset", offset)])
                    .query(query)
                    .send()
            })
            .await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "data_sets".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::TRACE, "data_sets: {:?}", &result);
        Ok(result)
    }

    /// Get a data set field by data set ID.
    pub async fn data_fields(
        &self, settings: &BaseSetting, limit: u32, offset: u32, query: &[(&str, &str)],
    ) -> ApiClientResult<Value> {
        let url = self.base_url.join("data-fields")?;
        let response = self
            .execute(|| {
                self.client
                    .get(url.clone())
                    .query(settings)
                    .query(query)
                    .query(&[("limit", limit), ("offset", offset)])
                    .send()
            })
            .await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "data_fields".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::TRACE, "data_fields: {:?}", &result);
        Ok(result)
    }

    /// Get operators.
    pub async fn operators(&self) -> ApiClientResult<Value> {
        let url = self.base_url.join("operators")?;

        let response = self.execute(|| self.client.get(url.clone()).send()).await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "operators".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::TRACE, "operators: {:?}", &result);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::ApiClient;
    use super::ApiClientResult;
    use crate::app::BaseSetting;
    use crate::low_level::types::*;

    fn sign_in_info() -> SignInInfo {
        SignInInfo { email: "xxxx@xxxx.com".to_string(), password: "xxxxx".to_string() }
    }

    async fn sign_in_client() -> ApiClient {
        let client = ApiClient::new().unwrap();
        let sign_in_info = sign_in_info();
        client.sign_in(&sign_in_info).await.unwrap();
        client
    }

    #[test]
    fn test_new() {
        assert!(ApiClient::new().is_ok());
    }

    #[tokio::test]
    async fn test_sign_in() {
        let client = ApiClient::new().unwrap();
        let sign_in_info = sign_in_info();
        assert!(client.sign_in(&sign_in_info).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_authentication() {
        let client = sign_in_client().await;
        assert!(client.get_authentication().await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_authentication() {
        let client = sign_in_client().await;
        assert!(client.delete_authentication().await.is_ok());
        assert!(client.get_authentication().await.is_err());
    }

    #[tokio::test]
    async fn test_option_simulations() {
        let client = sign_in_client().await;
        assert!(client.option_simulations().await.is_ok());
    }

    #[tokio::test]
    async fn test_post_simulations() {
        let client = sign_in_client().await;
        let simulation_obj = r#"
               [ {
                "type":"REGULAR",
                "settings":{
                    "maxTrade":"OFF",
                    "nanHandling":"OFF",
                    "instrumentType":"EQUITY",
                    "delay":1,
                    "universe":"TOP500",
                    "truncation":0.08,
                    "unitHandling":"VERIFY",
                    "maxPosition":"OFF",
                    "testPeriod":"P1Y",
                    "pasteurization":"ON",
                    "region":"USA",
                    "language":"FASTEXPR",
                    "decay":0,
                    "neutralization":"SUBINDUSTRY",
                    "visualization":false
                },
                "regular":"zscore(cash_st / debt_st)"
            },
            {
                "type":"REGULAR",
                "settings":{
                    "maxTrade":"OFF",
                    "nanHandling":"OFF",
                    "instrumentType":"EQUITY",
                    "delay":1,
                    "universe":"TOP500",
                    "truncation":0.08,
                    "unitHandling":"VERIFY",
                    "maxPosition":"OFF",
                    "testPeriod":"P1Y",
                    "pasteurization":"ON",
                    "region":"USA",
                    "language":"FASTEXPR",
                    "decay":0,
                    "neutralization":"SUBINDUSTRY",
                    "visualization":false
                },
                "regular":"close"
            }
            ]
            "#;
        assert!(client.post_simulations(simulation_obj).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_alphas() {
        let client = sign_in_client().await;
        assert!(client.alphas("78KkV3oQ").await.is_ok());
    }

    #[tokio::test]
    async fn test_alpha_recordsets() {
        let client = sign_in_client().await;
        assert!(client.alpha_recordsets("78KkV3oQ").await.is_ok());
    }

    #[tokio::test]
    async fn test_alpha_recordsets_name() {
        let client = sign_in_client().await;
        assert!(client.alpha_recordsets_name("78KkV3oQ", "pnl").await.is_ok());
    }

    #[tokio::test]
    async fn test_user_activities_diversities() {
        let client = sign_in_client().await;
        assert!(client.user_activities_diversities().await.is_ok());
    }

    #[tokio::test]
    async fn test_data_set() -> ApiClientResult<()> {
        let client = sign_in_client().await;
        let data_search = BaseSetting {
            delay: 1,
            instrument_type: "EQUITY".to_string(),
            region: "USA".to_string(),
            universe: "TOP3000".to_string(),
        };
        let result = client.data_sets(&data_search, 20, 0, &[]).await?;
        assert!(result.get("count").is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_data_set_field() -> ApiClientResult<()> {
        let client = sign_in_client().await;
        let data_sets_setting = BaseSetting {
            delay: 1,
            instrument_type: "EQUITY".to_string(),
            region: "USA".to_string(),
            universe: "TOP3000".to_string(),
        };
        let result =
            client.data_fields(&data_sets_setting, 20, 0, &[("dataset.id", "analyst10")]).await?;
        assert!(result.get("count").is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_operators() -> ApiClientResult<()> {
        let client = sign_in_client().await;
        let result = client.operators().await?;
        assert!(result.as_array().is_some());
        Ok(())
    }
}

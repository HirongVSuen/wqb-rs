use crate::model::*;
use reqwest::{Client, Method, Url, header};
use serde_json::Value;
use thiserror::Error;
use tracing::{Level, event, instrument};

/// API客户端错误
#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("网路错误： {0}")]
    ReqwestErr(#[from] reqwest::Error),

    #[error("请求响应异常: {api_name} (状态码: {status})")]
    ResponseError { api_name: String, status: StatusCodeDisplay },

    #[error("ApiClient 反序列化错误: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("url 解析异常：{0}")]
    UrlError(#[from] url::ParseError),

    #[error("业务错误: {0} (详情: {1})")]
    BussinessError(String, String),
}

// 自定义一个包装器来优雅处理 Option<StatusCode>
#[derive(Debug)]
pub struct StatusCodeDisplay(Option<reqwest::StatusCode>);

impl From<Option<reqwest::StatusCode>> for StatusCodeDisplay {
    fn from(status: Option<reqwest::StatusCode>) -> Self {
        StatusCodeDisplay(status)
    }
}

impl std::fmt::Display for StatusCodeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(code) => write!(f, "{}", code), // 打印 "400 Bad Request"
            None => write!(f, "无状态码(可能是网络超时)"), // 处理 None 的情况
        }
    }
}

/// API客户端结果类型
pub type ApiClientResult<T> = std::result::Result<T, ApiClientError>;

/// WorldQuant Brain API客户端
pub struct ApiClient {
    client: Client,
    base_url: Url,
}

impl ApiClient {
    /// 创建一个新的API客户端
    pub fn new() -> ApiClientResult<Self> {
        let client = Client::builder().cookie_store(true).build()?;
        let base_url = Url::parse("https://api.worldquantbrain.com")?;
        Ok(Self { client, base_url })
    }

    /// 登录
    pub async fn sign_in(&self, sign_in_info: &SignInInfo) -> ApiClientResult<()> {
        let url = self.base_url.join("authentication")?;

        let response = self
            .client
            .post(url)
            .basic_auth(&sign_in_info.email, Some(&sign_in_info.password))
            .send()
            .await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "sign_in".to_string(),
                status: err.status().into(),
            })?;
        event!(Level::INFO, "login user:{}", response.text().await?);
        Ok(())
    }

    /// 获取当前登录用户信息
    pub async fn get_authentication(&self) -> ApiClientResult<AuthenticationInfo> {
        let url = self.base_url.join("authentication")?;

        let response = self.client.get(url).send().await?;

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

        let response = self.client.delete(url).send().await?;
        response.error_for_status().map_err(|err| ApiClientError::ResponseError {
            api_name: "delete_authentication".to_string(),
            status: err.status().into(),
        })?;
        Ok(())
    }

    /// 获取有关可用属性、其类型、要求和允许值的详细信息。
    pub async fn option_simulations(&self) -> ApiClientResult<Value> {
        let url = self.base_url.join("simulations")?;
        let response = self.client.request(Method::OPTIONS, url).send().await?;

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
    pub async fn post_simulations(&self, simulation_obj: &'static str) -> ApiClientResult<String> {
        let url = self.base_url.join("simulations")?;

        let response = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(simulation_obj)
            .send()
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
    pub async fn get_simulations(&self, simulation_id: &str) -> ApiClientResult<Value> {
        let url = self.base_url.join(&format!("simulations/{}", simulation_id))?;

        let response = self.client.get(url).send().await?;

        let response =
            response.error_for_status().map_err(|err| ApiClientError::ResponseError {
                api_name: "get_simulations".to_string(),
                status: err.status().into(),
            })?;

        let result = response.json().await?;
        event!(Level::INFO, "get_simulations: {:?}", &result);
        Ok(result)
    }

    /// Get an alpha by id
    pub async fn alphas(&self, alpha_id: &str) -> ApiClientResult<Value> {
        let url = self.base_url.join(&format!("alphas/{}", alpha_id))?;
        let response = self.client.get(url).send().await?;

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

        let response = self.client.get(url).send().await?;

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

        let response = self.client.get(url).send().await?;

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

        let response = self.client.get(url).send().await?;

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
    pub async fn data_sets(&self, settings: &DataSetsSetting) -> ApiClientResult<Value> {
        let url = self.base_url.join("data-sets")?;

        let response = self.client.get(url).query(settings).send().await?;

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
    pub async fn data_fields(&self, settings: &DataFieldsSetting) -> ApiClientResult<Value> {
        let url = self.base_url.join("data-fields")?;

        let response = self.client.get(url).query(settings).send().await?;

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

        let response = self.client.get(url).send().await?;

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

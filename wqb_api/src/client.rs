use crate::model::*;
use reqwest::{Client, Method, header};
use thiserror::Error;

/// API客户端错误
#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("网路错误： {0}")]
    ReqwestErr(#[from] reqwest::Error),

    #[error("api请求失败：{0} {1}")]
    BussinessError(String, String),

    #[error("未登录")]
    NotLoggedIn,
}

/// API客户端结果类型
pub type ApiClientResult<T> = std::result::Result<T, ApiClientError>;

/// WorldQuant Brain API客户端
pub struct ApiClient {
    client: Client,
    base_url: String,
    sign_context: Option<SignInConext>,
}

impl ApiClient {
    /// 创建一个新的API客户端
    pub fn new() -> ApiClientResult<Self> {
        let client =
            Client::builder().cookie_store(true).build().map_err(ApiClientError::ReqwestErr)?;
        let base_url = String::from("https://api.worldquantbrain.com");
        Ok(Self { client, base_url, sign_context: None })
    }

    /// 登录
    pub async fn sign_in(&mut self, sign_in_info: &SignInInfo) -> ApiClientResult<()> {
        let url = format!("{}/authentication", self.base_url);
        let response = self
            .client
            .post(&url)
            .basic_auth(&sign_in_info.email, Some(&sign_in_info.password))
            .send()
            .await?;
        if response.status() != reqwest::StatusCode::CREATED {
            eprintln!("sign_in failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "sign_in".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        let sign_context: SignInConext = serde_json::from_str(&text).map_err(|err| {
            ApiClientError::BussinessError("解构SignInConext失败".to_string(), err.to_string())
        })?;
        self.sign_context = Some(sign_context);
        println!("sign_in success: {}", text);
        Ok(())
    }

    /// 获取当前登录用户信息
    pub async fn get_authentication(&self) -> ApiClientResult<()> {
        let url = format!("{}/authentication", self.base_url);
        let response = self.client.get(&url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("get_authentication failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "get_authentication".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        println!("{}", text);
        Ok(())
    }

    /// 登出
    pub async fn delete_authentication(&self) -> ApiClientResult<()> {
        let url = format!("{}/authentication", self.base_url);
        let response = self.client.delete(&url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("delete_authentication failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "delete_authentication".to_string(),
                response.status().to_string(),
            ));
        }
        Ok(())
    }

    /// 获取有关可用属性、其类型、要求和允许值的详细信息。
    pub async fn option_simulations(&self) -> ApiClientResult<()> {
        let url = format!("{}/simulations", self.base_url);
        let response = self.client.request(Method::OPTIONS, &url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("option_simulations failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "option_simulations".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        println!("{}", text);
        Ok(())
    }

    /// Post a new simulation
    pub async fn post_simulations(&self, simulation_obj: &'static str) -> ApiClientResult<()> {
        let url = format!("{}/simulations", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(simulation_obj)
            .send()
            .await?;
        if response.status() != reqwest::StatusCode::CREATED {
            eprintln!("post_simulations failed: {}", &response.status());
            return Err(ApiClientError::BussinessError(
                "post_simulations".to_string(),
                response.status().to_string(),
            ));
        }
        let location = response.headers().get(header::LOCATION).and_then(|v| v.to_str().ok());
        if let None = location {
            eprintln!("post_simulation not found location header");
            return Err(ApiClientError::BussinessError(
                "post_simulations".to_string(),
                "Location header not found".to_string(),
            ));
        }
        Ok(())
    }

    /// Get a simulation by id
    pub async fn get_simulations(&self, simulation_id: &str) -> ApiClientResult<()> {
        let url = format!("{}/simulations/{}", self.base_url, simulation_id);
        let response = self.client.get(&url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("get_simulations failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "get_simulations".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        println!("{}", text);
        Ok(())
    }

    /// Get an alpha by id
    pub async fn alphas(&self, alpha_id: &str) -> ApiClientResult<()> {
        let url = format!("{}/alphas/{}", self.base_url, alpha_id);
        let response = self.client.get(&url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("get_alphas failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "get_alphas".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        println!("{}", text);
        Ok(())
    }

    /// Get an alpha recordsets by id
    pub async fn alpha_recordsets(&self, alpha_id: &str) -> ApiClientResult<()> {
        let url = format!("{}/alphas/{}/recordsets", self.base_url, alpha_id);
        let response = self.client.get(&url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("get_alpha_recordsets failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "get_alpha_recordsets".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        println!("{}", text);
        Ok(())
    }

    /// Set the name of a recordset by recordset name
    pub async fn alpha_recordsets_name(&self, alpha_id: &str, name: &str) -> ApiClientResult<()> {
        let url = format!("{}/alphas/{}/recordsets/{}", self.base_url, alpha_id, name);
        let response = self.client.get(&url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("alpha_recordsets_setname failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "alpha_recordsets_setname".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        println!("{}", text);
        Ok(())
    }

    /// Get the diversities of a user's activities
    pub async fn user_activities_diversities(&self) -> ApiClientResult<()> {
        let sign_context = self.get_sign_context()?;
        let url = format!("{}/users/{}/activities/diversity", self.base_url, &sign_context.user.id);

        let response = self.client.get(&url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("user_activities_diversities failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "user_activities_diversities".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        println!("{}", text);
        Ok(())
    }

    /// Get the sign-in context of the current user
    pub fn get_sign_context(&self) -> ApiClientResult<&SignInConext> {
        let sign_context = self.sign_context.as_ref().ok_or(ApiClientError::NotLoggedIn)?;
        Ok(sign_context)
    }

    pub async fn data_sets(&self) -> ApiClientResult<()> {
        let url = format!("{}/data/data-sets", self.base_url);
        let response = self.client.get(&url).send().await?;
        if response.status() != reqwest::StatusCode::OK {
            eprintln!("data_sets failed: {}", response.status());
            return Err(ApiClientError::BussinessError(
                "data_sets".to_string(),
                response.status().to_string(),
            ));
        }
        let text = response.text().await?;
        println!("{}", text);
        Ok(())
    }
}

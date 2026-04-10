use serde::{Deserialize, Serialize};
/// 登录信息
pub struct SignInInfo {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthenticationInfo {
    pub user: User,
    pub token: Token,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub expiry: f64,
}

#[derive(Debug, Serialize)]
pub struct Data_Sets_Setting {
    pub delay: u8,
    pub instrumentType: String,
    pub limit: u8,
    pub offset: u8,
    pub region: String,
    pub universe: String,
}

#[derive(Debug, Serialize)]
pub struct Data_Fields_Setting {
    pub delay: u8,
    pub instrumentType: String,
    pub limit: u8,
    pub offset: u8,
    pub region: String,
    pub universe: String,
    #[serde(rename = "dataset.id")]
    pub dataSetId: String,
}

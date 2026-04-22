use serde::{Deserialize, Serialize};

/// 登录信息
#[derive(Debug, Deserialize, Serialize)]
pub struct SignInInfo {
    pub email: String,
    pub password: String,
}
/// 认证信息
#[derive(Debug, Deserialize)]
pub struct AuthenticationInfo {
    pub user: User,
    pub token: Token,
    pub permissions: Vec<String>,
}

/// 用户信息
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
}

/// 票据信息
#[derive(Debug, Deserialize)]
pub struct Token {
    pub expiry: f64,
}

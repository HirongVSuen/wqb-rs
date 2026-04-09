use serde::Deserialize;
/// 登录信息
pub struct SignInInfo {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignInConext {
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

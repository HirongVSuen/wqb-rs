use polars::error::PolarsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainClientError {
    #[error("ApiClient 错误: {0}")]
    ApiClientError(#[from] ApiClientError),

    #[error("未找到凭据文件")]
    CredentialsNotFound(),

    #[error("读取凭据文件失败")]
    ReadCredentialsFailed(),

    #[error("保存凭据文件失败")]
    SaveCredentialsFailed(),

    #[error("反序列化失败: {0}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("读取文件失败")]
    FileReadError(#[from] std::io::Error),

    #[error("登录失败: 账号或密码错误")]
    LoginFailed(),

    #[error("创建df失败：{0}")]
    DFERROR(#[from] PolarsError),

    #[error("获取字段失败:{0}")]
    NotFoundField(String),
}

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

    #[error("超过最大重试次数！")]
    MaxRetriesExceeded,
}

// 自定义一个包装器来优雅处理 Option<StatusCode>
#[derive(Debug)]
pub struct StatusCodeDisplay(pub Option<reqwest::StatusCode>);

impl From<Option<reqwest::StatusCode>> for StatusCodeDisplay {
    fn from(status: Option<reqwest::StatusCode>) -> Self {
        StatusCodeDisplay(status)
    }
}

impl From<reqwest::StatusCode> for StatusCodeDisplay {
    fn from(status: reqwest::StatusCode) -> Self {
        StatusCodeDisplay(Some(status))
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

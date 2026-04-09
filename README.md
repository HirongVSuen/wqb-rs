# WorldQuant Brain API Rust Client (wqb-rs)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个用于访问 WorldQuant Brain API 的 Rust 客户端库。提供类型安全的 API 调用，支持异步操作，并包含完整的错误处理。

## 功能特性

- ✅ 完整的认证流程（登录、登出、会话管理）
- ✅ 模拟管理（创建、查询、配置模拟）
- ✅ Alpha 管理（查询 Alpha 信息、记录集）
- ✅ 用户活动多样性查询
- ✅ 数据集查询
- ✅ 类型安全的请求/响应处理
- ✅ 完整的错误处理
- ✅ 异步支持（基于 Tokio）
- ✅ 日志记录支持（基于 tracing）


## 快速开始

### 基本使用

```rust
use wqb_api::{ApiClient, SignInInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 API 客户端
    let mut client = ApiClient::new()?;
    
    // 登录
    let sign_in_info = SignInInfo {
        email: "your-email@example.com".to_string(),
        password: "your-password".to_string(),
    };
    
    client.sign_in(&sign_in_info).await?;
    println!("登录成功！");
    
    // 获取当前认证信息
    client.get_authentication().await?;
    
    // 查询模拟选项
    client.option_simulations().await?;
    
    // 查询数据集
    client.data_sets().await?;
    
    // 登出
    client.delete_authentication().await?;
    
    Ok(())
}
```

### 创建模拟

```rust
use wqb_api::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ApiClient::new()?;
    
    // 先登录
    // ...
    
    // 创建模拟
    let simulation_obj = r#"
    [{
        "type": "REGULAR",
        "settings": {
            "maxTrade": "OFF",
            "nanHandling": "OFF",
            "instrumentType": "EQUITY",
            "delay": 1,
            "universe": "TOP500",
            "truncation": 0.08,
            "unitHandling": "VERIFY",
            "maxPosition": "OFF",
            "testPeriod": "P1Y",
            "pasteurization": "ON",
            "region": "USA",
            "language": "FASTEXPR",
            "decay": 0,
            "neutralization": "SUBINDUSTRY",
            "visualization": false
        },
        "regular": "zscore(cash_st / debt_st)"
    }]
    "#;
    
    client.post_simulations(simulation_obj).await?;
    println!("模拟创建成功！");
    
    Ok(())
}
```

### 查询 Alpha 信息

```rust
use wqb_api::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ApiClient::new()?;
    
    // 先登录
    // ...
    
    // 查询 Alpha 信息
    let alpha_id = "78KkV3oQ";
    client.alphas(alpha_id).await?;
    
    // 查询 Alpha 记录集
    client.alpha_recordsets(alpha_id).await?;
    
    // 查询特定记录集名称
    client.alpha_recordsets_name(alpha_id, "pnl").await?;
    
    Ok(())
}
```

## API 参考

### 主要结构体

#### `ApiClient`
主要的 API 客户端，提供所有 API 方法。

#### `SignInInfo`
登录信息，包含邮箱和密码。

#### `SignInConext`
登录上下文，包含用户信息、令牌和权限。

### 主要方法

#### 认证相关
- `sign_in(&mut self, sign_in_info: &SignInInfo) -> ApiClientResult<()>` - 登录
- `get_authentication(&self) -> ApiClientResult<()>` - 获取当前认证信息
- `delete_authentication(&self) -> ApiClientResult<()>` - 登出

#### 模拟管理
- `option_simulations(&self) -> ApiClientResult<()>` - 获取模拟选项
- `post_simulations(&self, simulation_obj: &'static str) -> ApiClientResult<()>` - 创建模拟
- `get_simulations(&self, simulation_id: &str) -> ApiClientResult<()>` - 查询模拟

#### Alpha 管理
- `alphas(&self, alpha_id: &str) -> ApiClientResult<()>` - 查询 Alpha 信息
- `alpha_recordsets(&self, alpha_id: &str) -> ApiClientResult<()>` - 查询 Alpha 记录集
- `alpha_recordsets_name(&self, alpha_id: &str, name: &str) -> ApiClientResult<()>` - 查询特定名称的记录集

#### 用户活动
- `user_activities_diversities(&self) -> ApiClientResult<()>` - 查询用户活动多样性

#### 数据管理
- `data_sets(&self) -> ApiClientResult<()>` - 查询数据集

### 错误处理

库使用 `ApiClientError` 枚举处理所有错误：

```rust
pub enum ApiClientError {
    #[error("网路错误： {0}")]
    ReqwestErr(#[from] reqwest::Error),
    
    #[error("api请求失败：{0} {1}")]
    BussinessError(String, String),
    
    #[error("未登录")]
    NotLoggedIn,
}
```

## 运行测试

项目包含完整的测试套件：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_sign_in

# 运行测试并显示输出
cargo test -- --nocapture
```

注意：测试需要有效的 WorldQuant Brain 账户凭据。请在运行测试前在 `tests.rs` 文件中更新测试凭据。

## 项目结构

```
wqb-rs/
├── Cargo.toml          # 工作空间配置
├── README.md           # 本文档
├── rust-toolchain.toml # Rust 工具链配置
├── rustfmt.toml        # Rust 格式化配置
└── wqb_api/           # API 客户端库
    ├── Cargo.toml     # 库配置
    └── src/
        ├── lib.rs     # 库入口点
        ├── client.rs  # API 客户端实现
        ├── model.rs   # 数据模型
        ├── main.rs    # 示例主程序
        └── tests.rs   # 测试套件
```

## 开发指南

### 设置开发环境

1. 安装 Rust 工具链（推荐使用 rustup）：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 克隆项目：
   ```bash
   git clone <repository-url>
   cd wqb-rs
   ```

3. 构建项目：
   ```bash
   cargo build
   ```

### 添加新功能

1. 在 `model.rs` 中添加新的数据结构
2. 在 `client.rs` 中添加对应的 API 方法
3. 在 `tests.rs` 中添加测试用例
4. 运行测试确保功能正常

## 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 支持

- 问题跟踪：GitHub Issues
- 文档：查看 [docs.rs/wqb_api](https://docs.rs/wqb_api)
- 讨论：GitHub Discussions

## 相关项目

- [WorldQuant Brain](https://www.worldquantbrain.com/) - 官方平台
- [reqwest](https://crates.io/crates/reqwest) - Rust HTTP 客户端
- [tokio](https://crates.io/crates/tokio) - Rust 异步运行时

---

**注意**：使用此库需要有效的 WorldQuant Brain 账户。请确保遵守 WorldQuant 的服务条款和使用政策。

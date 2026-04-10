# ACE - WorldQuant Brain API Rust 客户端

[![Crates.io](https://img.shields.io/crates/v/ace)](https://crates.io/crates/ace)
[![Documentation](https://docs.rs/ace/badge.svg)](https://docs.rs/ace)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

一个用于访问 WorldQuant Brain API 的 Rust 客户端库，提供类型安全、异步的金融数据和量化研究工具访问。

## 功能特性

- 🔐 **完整认证** - 登录、登出和会话管理
- 📊 **模拟管理** - 创建、查询和配置模拟
- 📈 **Alpha 管理** - 查询 Alpha 信息和记录集
- 👤 **用户分析** - 访问用户活动多样性指标
- 🗃️ **数据探索** - 查询数据集和数据字段
- 🛠️ **操作符发现** - 检索可用操作符
- 🔧 **类型安全** - Rust 类型系统确保数据完整性
- ⚡ **异步支持** - 基于 Tokio 的高性能异步操作
- 📝 **完整日志** - 集成 tracing 支持
- 🍪 **Cookie 管理** - 自动会话 Cookie 处理
- 🎯 **错误处理** - 详细的错误类型和上下文信息

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ace = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 快速开始

### 基本使用

```rust
use ace::low_level::{ApiClient, SignInInfo, DataSetsSetting};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 API 客户端
    let client = ApiClient::new()?;
    
    // 登录
    let sign_in_info = SignInInfo {
        email: "your-email@example.com".to_string(),
        password: "your-password".to_string(),
    };
    
    client.sign_in(&sign_in_info).await?;
    println!("登录成功！");
    
    // 获取认证信息
    let auth_info = client.get_authentication().await?;
    println!("用户 ID: {}", auth_info.user.id);
    
    // 查询数据集
    let dataset_settings = DataSetsSetting {
        delay: 1,
        instrument_type: "EQUITY".to_string(),
        limit: 20,
        offset: 0,
        region: "USA".to_string(),
        universe: "TOP3000".to_string(),
    };
    
    let datasets = client.data_sets(&dataset_settings).await?;
    println!("数据集: {}", datasets);
    
    // 登出
    client.delete_authentication().await?;
    
    Ok(())
}
```

### 创建模拟

```rust
use ace::low_level::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new()?;
    
    // 先登录...
    
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
    
    let simulation_id = client.post_simulations(simulation_obj).await?;
    println!("模拟创建成功，ID: {}", simulation_id);
    
    Ok(())
}
```

## API 概览

### 核心类型

- `ApiClient` - 主要客户端结构，包含所有 API 方法
- `SignInInfo` - 登录凭据（邮箱/密码）
- `AuthenticationInfo` - 认证上下文，包含用户/令牌信息
- `DataSetsSetting` - 数据集查询参数
- `DataFieldsSetting` - 数据字段查询参数

### 主要方法

#### 认证相关
- `sign_in()` - 登录到 WorldQuant Brain
- `get_authentication()` - 获取当前认证信息
- `delete_authentication()` - 登出

#### 模拟管理
- `option_simulations()` - 获取模拟选项和配置
- `post_simulations()` - 创建新模拟
- `get_simulations()` - 根据 ID 获取模拟

#### Alpha 管理
- `alphas()` - 获取 Alpha 信息
- `alpha_recordsets()` - 获取 Alpha 记录集
- `alpha_recordsets_name()` - 根据名称获取特定记录集

#### 数据管理
- `data_sets()` - 查询可用数据集
- `data_fields()` - 查询数据集的数据字段
- `operators()` - 获取可用操作符

#### 用户分析
- `user_activities_diversities()` - 获取用户活动多样性指标

## 错误处理

库使用全面的错误类型 `ApiClientError`：

```rust
pub enum ApiClientError {
    #[error("网络错误： {0}")]
    ReqwestErr(#[from] reqwest::Error),
    
    #[error("请求失败：{api_name} (状态码：{status})")]
    ResponseError { api_name: String, status: StatusCodeDisplay },
    
    #[error("反序列化错误：{0}")]
    SerdeError(#[from] serde_json::Error),
    
    #[error("URL 解析错误：{0}")]
    UrlError(#[from] url::ParseError),
    
    #[error("业务错误：{0} (详情：{1})")]
    BussinessError(String, String),
}
```

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_sign_in

# 运行测试并显示输出
cargo test -- --nocapture
```

**注意：** 测试需要有效的 WorldQuant Brain 账户凭据。在运行测试前，请在 `ace/src/low_level/client.rs` 中更新测试凭据。

## 项目结构

```
wqb-rs/
├── Cargo.toml          # 工作空间配置
├── README.md           # 本文档
├── LICENSE             # MIT 许可证
├── rust-toolchain.toml # Rust 工具链配置
├── rustfmt.toml        # 代码格式化配置
└── ace/               # API 客户端库
    ├── Cargo.toml     # 库配置
    └── src/
        ├── lib.rs     # 库入口点
        ├── high_level/ # 高级 API（计划中）
        └── low_level/ # 低级 API 实现
            ├── mod.rs
            ├── client.rs
            └── model.rs
```

## 开发

### 环境设置

```bash
# 克隆仓库
git clone <repository-url>
cd wqb-rs

# 构建项目
cargo build

# 运行检查
cargo check
cargo clippy
cargo fmt --check
```

### 添加新功能

1. 在 `model.rs` 中添加数据结构
2. 在 `client.rs` 中实现 API 方法
3. 在测试模块中添加测试
4. 运行测试验证功能

## 贡献

欢迎贡献！请：

1. Fork 仓库
2. 创建功能分支
3. 进行更改
4. 添加测试
5. 提交 Pull Request

## 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE)。

## 支持

- 问题跟踪：[GitHub Issues](https://github.com/yourusername/wqb-rs/issues)
- 文档：[docs.rs/ace](https://docs.rs/ace)

## 免责声明

本库需要有效的 WorldQuant Brain 账户。用户必须遵守 WorldQuant 的服务条款和使用政策。这是一个非官方的第三方库，与 WorldQuant 无关。
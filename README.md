# ACE - WorldQuant Brain API Rust 客户端

[![Crates.io](https://img.shields.io/crates/v/ace)](https://crates.io/crates/ace)
[![Documentation](https://docs.rs/ace/badge.svg)](https://docs.rs/ace)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

一个用于访问 WorldQuant Brain API 的 Rust 客户端库，提供类型安全、异步的金融数据和量化研究工具访问。包含低级API和高级API两层抽象，满足不同使用场景需求。

## 功能特性

### 核心功能
- 🔐 **完整认证** - 登录、登出、会话管理和自动凭据存储
- 📊 **模拟管理** - 创建、查询和配置模拟
- 📈 **Alpha 管理** - 查询 Alpha 信息和记录集
- 👤 **用户分析** - 访问用户活动多样性指标
- 🗃️ **数据探索** - 查询数据集和数据字段
- 🛠️ **操作符发现** - 检索可用操作符

### 技术特性
- 🔧 **类型安全** - Rust 类型系统确保数据完整性
- ⚡ **异步支持** - 基于 Tokio 的高性能异步操作
- 📝 **完整日志** - 集成 tracing 支持
- 🍪 **Cookie 管理** - 自动会话 Cookie 处理
- 🎯 **错误处理** - 详细的错误类型和上下文信息
- 📊 **数据框支持** - 集成 Polars 数据框处理
- 🔄 **自动重试** - 网络错误自动重试机制

## 架构设计

### 两层API设计
- **低级API (`low_level`)** - 直接映射 WorldQuant Brain REST API，提供原始控制
- **高级API (`high_level`)** - 简化操作，提供更友好的接口和自动化功能

### 模块结构
```
ace/
├── src/
│   ├── lib.rs              # 库入口点
│   ├── err.rs              # 错误类型定义
│   ├── low_level/          # 低级API实现
│   │   ├── mod.rs
│   │   ├── client.rs       # API客户端
│   │   └── types.rs        # 数据结构
│   └── high_level/         # 高级API实现
│       ├── mod.rs
│       ├── brain_client.rs # 高级客户端
│       └── types.rs        # 高级类型
```

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ace = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 快速开始

### 使用高级API（推荐）

```rust
use ace::high_level::{BrainClient, BrainClientConfig};
use ace::high_level::types::{SimulationDataType, SimulationParam, SimulationState};
use ace::low_level::types::BaseSetting;
use std::collections::HashMap;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端配置（自动保存凭据到文件）
    let config = BrainClientConfig { 
        auth_file_path: "/path/to/auth.json".to_string() 
    };
    
    // 创建客户端并自动登录
    let client = BrainClient::new(config)?;
    client.auto_login().await?;
    
    // 配置模拟参数
    let base_setting = BaseSetting {
        delay: 1,
        instrument_type: "EQUITY".to_string(),
        region: "USA".to_string(),
        universe: "TOP500".to_string(),
    };
    
    let extra_param = HashMap::from([
        ("maxTrade".to_string(), Value::from("OFF")),
        ("maxPosition".to_string(), Value::from("OFF")),
        ("testPeriod".to_string(), Value::from("P1Y")),
    ]);
    
    let param = SimulationParam {
        language: "FASTEXPR".to_string(),
        neutralization: "SUBINDUSTRY".to_string(),
        decay: 0,
        truncation: 0.08,
        pasteurization: SimulationState::ON,
        unit_handling: SimulationState::VERIFY,
        nan_handling: SimulationState::OFF,
        visualization: false,
        extra_param,
    };
    
    // 运行模拟
    let alpha_id = client.simulation(
        SimulationDataType::REGULAR,
        &base_setting,
        &param,
        "cash_st / debt_st".to_string()
    ).await?;
    
    println!("模拟创建成功，Alpha ID: {}", alpha_id);
    
    Ok(())
}
```

### 使用低级API

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

### 命令行工具示例

项目包含一个示例命令行工具 (`src/main.rs`)，展示如何使用高级API进行批量模拟：

```bash
# 构建并运行示例
cargo run --release
```

## API 概览

### 高级API (`high_level`)

#### `BrainClient`
- `new(config: BrainClientConfig)` - 创建新客户端
- `auto_login()` - 自动登录（使用保存的凭据或交互式输入）
- `simulation()` - 创建模拟并返回Alpha ID
- `data_field_df_by_dataset()` - 获取数据集字段为Polars数据框

#### `BrainClientConfig`
- `auth_file_path` - 认证文件存储路径

#### 类型
- `SimulationDataType` - 模拟数据类型（REGULAR, CUSTOM等）
- `SimulationParam` - 模拟参数配置
- `SimulationState` - 模拟状态（ON, OFF, VERIFY）

### 低级API (`low_level`)

#### `ApiClient`
- `sign_in()` - 登录
- `get_authentication()` - 获取认证信息
- `delete_authentication()` - 登出
- `post_simulations()` - 创建模拟
- `get_simulations()` - 获取模拟
- `data_sets()` - 查询数据集
- `data_fields()` - 查询数据字段
- `operators()` - 获取操作符
- `alphas()` - 获取Alpha信息
- `alpha_recordsets()` - 获取Alpha记录集

#### 核心类型
- `SignInInfo` - 登录凭据
- `AuthenticationInfo` - 认证信息
- `DataSetsSetting` - 数据集查询参数
- `DataFieldsSetting` - 数据字段查询参数
- `BaseSetting` - 基础设置（区域、延迟等）

## 错误处理

库使用分层的错误类型系统：

### `BrainClientError` (高级API错误)
```rust
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
```

### `ApiClientError` (低级API错误)
```rust
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
```

## 数据框集成

高级API集成了Polars数据框，便于数据处理：

```rust
// 获取数据集字段为数据框
let df = client.data_field_df_by_dataset(&base_setting, "fundamental23").await?;

// 提取字段ID列表
let field_ids: Vec<String> = df.column("id")?
    .str()?
    .into_no_null_iter()
    .map(|val| val.to_string())
    .collect();

// 批量处理字段
for field_id in field_ids {
    let alpha_id = client.simulation(
        SimulationDataType::REGULAR,
        &base_setting,
        &param,
        field_id
    ).await?;
    println!("字段 {} 的Alpha ID: {}", field_id, alpha_id);
}
```

## 开发指南

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

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_sign_in

# 运行测试并显示输出
cargo test -- --nocapture
```

**注意：** 测试需要有效的 WorldQuant Brain 账户凭据。

### 添加新功能

1. 在 `low_level/types.rs` 中添加数据结构
2. 在 `low_level/client.rs` 中实现API方法
3. 在 `high_level/types.rs` 和 `brain_client.rs` 中添加高级抽象
4. 添加测试用例
5. 更新文档

## 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 代码规范
- 遵循 Rust 官方编码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 为公共API添加文档注释

## 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE)。

## 支持与反馈

- 问题跟踪：[GitHub Issues](https://github.com/yourusername/wqb-rs/issues)
- 文档：[docs.rs/ace](https://docs.rs/ace)
- 邮箱：myant91@126.com

## 版本历史

### v0.1.0
- 初始版本发布
- 低级API完整实现
- 高级API基础功能
- Polars数据框集成
- 完整的错误处理系统

## 免责声明

本库需要有效的 WorldQuant Brain 账户。用户必须遵守 WorldQuant 的服务条款和使用政策。这是一个非官方的第三方库，与 WorldQuant 无关。

## 致谢

- [WorldQuant Brain](https://brain.worldquant.com/) - 提供量化研究平台
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Tokio](https://tokio.rs/) - 异步运行时
- [Polars](https://pola.rs/) - 数据框库
- [Reqwest](https://docs.rs/reqwest) - HTTP客户端

---
**注意：** 使用本库前请确保您已阅读并同意 WorldQuant Brain 的服务条款。
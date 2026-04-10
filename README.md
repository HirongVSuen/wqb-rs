# WorldQuant Brain API Rust Client (wqb-rs)

[![Crates.io](https://img.shields.io/crates/v/wqb_api)](https://crates.io/crates/wqb_api)
[![Documentation](https://docs.rs/wqb_api/badge.svg)](https://docs.rs/wqb_api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

一个用于访问 WorldQuant Brain API 的 Rust 客户端库。提供类型安全的 API 调用，支持异步操作，并包含完整的错误处理。

## ✨ 功能特性

- ✅ **完整的认证流程** - 登录、登出、会话管理
- ✅ **模拟管理** - 创建、查询、配置模拟
- ✅ **Alpha 管理** - 查询 Alpha 信息、记录集
- ✅ **用户活动多样性查询** - 获取用户活动多样性数据
- ✅ **数据集查询** - 访问可用数据集信息
- ✅ **类型安全的请求/响应处理** - 使用 Rust 类型系统确保数据安全
- ✅ **完整的错误处理** - 详细的错误类型和错误消息
- ✅ **异步支持** - 基于 Tokio 的异步运行时
- ✅ **日志记录支持** - 基于 tracing 的日志框架
- ✅ **Cookie 管理** - 自动处理会话 Cookie
- ✅ **HTTP 客户端配置** - 可配置的 HTTP 客户端

## 📦 安装

### 作为库使用

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
wqb_api = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/yourusername/wqb-rs.git
cd wqb-rs

# 构建项目
cargo build --release

# 运行测试
cargo test
```

## 🚀 快速开始

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
    
    // 先登录（省略登录代码）
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
    
    // 先登录（省略登录代码）
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

### 环境配置

可以通过环境变量配置日志级别：

```bash
# 设置日志级别
export RUST_LOG=info
export RUST_LOG=debug  # 更详细的日志
export RUST_LOG=trace  # 最详细的日志
```

## 📚 API 参考

### 主要结构体

#### `ApiClient`
主要的 API 客户端，提供所有 API 方法。

**方法：**
- `new() -> ApiClientResult<Self>` - 创建新的 API 客户端
- `get_sign_context(&self) -> ApiClientResult<&SignInConext>` - 获取当前登录上下文

#### `SignInInfo`
登录信息，包含邮箱和密码。

**字段：**
- `email: String` - 用户邮箱
- `password: String` - 用户密码

#### `SignInConext`
登录上下文，包含用户信息、令牌和权限。

**字段：**
- `user: User` - 用户信息
- `token: Token` - 认证令牌
- `permissions: Vec<String>` - 用户权限列表

### 主要方法

#### 认证相关
- `sign_in(&mut self, sign_in_info: &SignInInfo) -> ApiClientResult<()>` - 登录到 WorldQuant Brain
- `get_authentication(&self) -> ApiClientResult<()>` - 获取当前认证信息
- `delete_authentication(&self) -> ApiClientResult<()>` - 登出

#### 模拟管理
- `option_simulations(&self) -> ApiClientResult<()>` - 获取模拟选项和配置
- `post_simulations(&self, simulation_obj: &'static str) -> ApiClientResult<()>` - 创建新的模拟
- `get_simulations(&self, simulation_id: &str) -> ApiClientResult<()>` - 查询特定模拟

#### Alpha 管理
- `alphas(&self, alpha_id: &str) -> ApiClientResult<()>` - 查询 Alpha 信息
- `alpha_recordsets(&self, alpha_id: &str) -> ApiClientResult<()>` - 查询 Alpha 记录集
- `alpha_recordsets_name(&self, alpha_id: &str, name: &str) -> ApiClientResult<()>` - 查询特定名称的记录集

#### 用户活动
- `user_activities_diversities(&self) -> ApiClientResult<()>` - 查询用户活动多样性

#### 数据管理
- `data_sets(&self) -> ApiClientResult<()>` - 查询可用数据集

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

**类型别名：**
```rust
pub type ApiClientResult<T> = std::result::Result<T, ApiClientError>;
```

## 🧪 运行测试

项目包含完整的测试套件：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_sign_in
cargo test test_get_authentication

# 运行测试并显示输出（用于调试）
cargo test -- --nocapture

# 运行测试并生成测试覆盖率报告（需要安装 tarpaulin）
cargo tarpaulin --ignore-tests
```

**注意：** 测试需要有效的 WorldQuant Brain 账户凭据。请在运行测试前在 `tests.rs` 文件中更新测试凭据：

```rust
fn sign_in_info() -> SignInInfo {
    SignInInfo { 
        email: "your-email@example.com".to_string(), 
        password: "your-password".to_string() 
    }
}
```

## 🏗️ 项目结构

```
wqb-rs/
├── Cargo.toml          # 工作空间配置
├── README.md           # 本文档
├── LICENSE             # MIT 许可证
├── rust-toolchain.toml # Rust 工具链配置
├── rustfmt.toml        # Rust 格式化配置
└── wqb_api/           # API 客户端库
    ├── Cargo.toml     # 库配置
    └── src/
        ├── lib.rs     # 库入口点
        ├── client.rs  # API 客户端实现
        ├── model.rs   # 数据模型定义
        ├── main.rs    # 示例主程序
        └── tests.rs   # 测试套件
```

## 🔧 开发指南

### 设置开发环境

1. **安装 Rust 工具链**（推荐使用 rustup）：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **安装开发工具**：
   ```bash
   # 代码格式化
   rustup component add rustfmt
   
   # 代码检查
   rustup component add clippy
   ```

3. **克隆项目**：
   ```bash
   git clone <repository-url>
   cd wqb-rs
   ```

4. **构建项目**：
   ```bash
   cargo build
   cargo check     # 快速检查语法
   cargo clippy    # 代码质量检查
   ```

### 添加新功能

1. **在 `model.rs` 中添加新的数据结构**
   ```rust
   #[derive(Debug, Deserialize)]
   pub struct NewModel {
       pub field1: String,
       pub field2: i32,
   }
   ```

2. **在 `client.rs` 中添加对应的 API 方法**
   ```rust
   impl ApiClient {
       pub async fn new_method(&self, param: &str) -> ApiClientResult<()> {
           // 实现逻辑
       }
   }
   ```

3. **在 `tests.rs` 中添加测试用例**
   ```rust
   #[tokio::test]
   async fn test_new_method() {
       let client = sign_in_client().await;
       assert!(client.new_method("test").await.is_ok());
   }
   ```

4. **运行测试确保功能正常**
   ```bash
   cargo test test_new_method
   ```

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 命名规范（snake_case 用于函数和变量，CamelCase 用于类型）
- 为公共 API 添加文档注释

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 贡献流程

1. **Fork 项目**
2. **创建功能分支** (`git checkout -b feature/amazing-feature`)
3. **提交更改** (`git commit -m 'Add some amazing feature'`)
4. **推送到分支** (`git push origin feature/amazing-feature`)
5. **打开 Pull Request**

### 贡献指南

- 确保代码通过所有测试
- 添加适当的测试用例
- 更新相关文档
- 遵循项目的代码风格

## 📄 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解详情。

```
MIT License

Copyright (c) [year] [author]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 📞 支持

- **问题跟踪**：[GitHub Issues](https://github.com/yourusername/wqb-rs/issues)
- **文档**：[docs.rs/wqb_api](https://docs.rs/wqb_api)
- **讨论**：[GitHub Discussions](https://github.com/yourusername/wqb-rs/discussions)
- **邮件**：your-email@example.com

## 🔗 相关项目

- [WorldQuant Brain](https://www.worldquantbrain.com/) - 官方平台
- [reqwest](https://crates.io/crates/reqwest) - Rust HTTP 客户端
- [tokio](https://crates.io/crates/tokio) - Rust 异步运行时
- [serde](https://crates.io/crates/serde) - Rust 序列化框架
- [tracing](https://crates.io/crates/tracing) - Rust 应用程序级跟踪

## ⚠️ 注意事项

1. **账户要求**：使用此库需要有效的 WorldQuant Brain 账户
2. **服务条款**：请确保遵守 WorldQuant 的服务条款和使用政策
3. **API 限制**：注意 API 的速率限制和使用限制
4. **安全性**：妥善保管账户凭据，不要在代码中硬编码敏感信息
5. **版本兼容性**：API 可能发生变化，请关注官方文档更新

## 📈 版本历史

- **v0.1.0** (当前) - 初始版本，包含基本 API 功能
- 计划功能：更多 API 端点支持、更好的错误处理、配置管理

---

**Happy Coding!** 🚀

如果您在使用过程中遇到任何问题或有改进建议，请随时提交 Issue 或 Pull Request。
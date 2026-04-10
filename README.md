# ACE - WorldQuant Brain API Client for Rust

[![Crates.io](https://img.shields.io/crates/v/ace)](https://crates.io/crates/ace)
[![Documentation](https://docs.rs/ace/badge.svg)](https://docs.rs/ace)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

A comprehensive Rust client library for the WorldQuant Brain API, providing type-safe, asynchronous access to financial data and quantitative research tools.

## Features

- 🔐 **Full Authentication** - Login, logout, and session management
- 📊 **Simulation Management** - Create, query, and configure simulations
- 📈 **Alpha Management** - Query alpha information and recordsets
- 👤 **User Analytics** - Access user activity diversity metrics
- 🗃️ **Data Exploration** - Query datasets and data fields
- 🛠️ **Operator Discovery** - Retrieve available operators
- 🔧 **Type Safety** - Rust's type system ensures data integrity
- ⚡ **Async Ready** - Built on Tokio for high-performance async operations
- 📝 **Comprehensive Logging** - Integrated tracing support
- 🍪 **Cookie Management** - Automatic session cookie handling
- 🎯 **Error Handling** - Detailed error types with context

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ace = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Basic Usage

```rust
use ace::low_level::{ApiClient, SignInInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create API client
    let client = ApiClient::new()?;
    
    // Login
    let sign_in_info = SignInInfo {
        email: "your-email@example.com".to_string(),
        password: "your-password".to_string(),
    };
    
    client.sign_in(&sign_in_info).await?;
    println!("Login successful!");
    
    // Get authentication info
    let auth_info = client.get_authentication().await?;
    println!("User ID: {}", auth_info.user.id);
    
    // Query datasets
    let dataset_settings = DataSetsSetting {
        delay: 1,
        instrument_type: "EQUITY".to_string(),
        limit: 20,
        offset: 0,
        region: "USA".to_string(),
        universe: "TOP3000".to_string(),
    };
    
    let datasets = client.data_sets(&dataset_settings).await?;
    println!("Datasets: {}", datasets);
    
    // Logout
    client.delete_authentication().await?;
    
    Ok(())
}
```

### Creating a Simulation

```rust
use ace::low_level::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new()?;
    
    // Login first...
    
    // Create simulation
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
    println!("Simulation created with ID: {}", simulation_id);
    
    Ok(())
}
```

## API Overview

### Core Types

- `ApiClient` - Main client struct with all API methods
- `SignInInfo` - Login credentials (email/password)
- `AuthenticationInfo` - Authentication context with user/token info
- `DataSetsSetting` - Dataset query parameters
- `DataFieldsSetting` - Data field query parameters

### Key Methods

#### Authentication
- `sign_in()` - Authenticate with WorldQuant Brain
- `get_authentication()` - Get current authentication info
- `delete_authentication()` - Logout

#### Simulations
- `option_simulations()` - Get simulation options and configurations
- `post_simulations()` - Create new simulation
- `get_simulations()` - Get simulation by ID

#### Alpha Management
- `alphas()` - Get alpha information
- `alpha_recordsets()` - Get alpha recordsets
- `alpha_recordsets_name()` - Get specific recordset by name

#### Data Management
- `data_sets()` - Query available datasets
- `data_fields()` - Query data fields for a dataset
- `operators()` - Get available operators

#### User Analytics
- `user_activities_diversities()` - Get user activity diversity metrics

## Error Handling

The library uses a comprehensive error type `ApiClientError`:

```rust
pub enum ApiClientError {
    #[error("Network error: {0}")]
    ReqwestErr(#[from] reqwest::Error),
    
    #[error("Request failed: {api_name} (status: {status})")]
    ResponseError { api_name: String, status: StatusCodeDisplay },
    
    #[error("Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    
    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),
    
    #[error("Business error: {0} (details: {1})")]
    BussinessError(String, String),
}
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_sign_in

# Run tests with output
cargo test -- --nocapture
```

**Note:** Tests require valid WorldQuant Brain credentials. Update the test credentials in `ace/src/low_level/client.rs` before running tests.

## Project Structure

```
wqb-rs/
├── Cargo.toml          # Workspace configuration
├── README.md           # This document
├── LICENSE             # MIT License
├── rust-toolchain.toml # Rust toolchain
├── rustfmt.toml        # Code formatting
└── ace/               # API client library
    ├── Cargo.toml     # Library configuration
    └── src/
        ├── lib.rs     # Library entry point
        ├── high_level/ # High-level API (planned)
        └── low_level/ # Low-level API implementation
            ├── mod.rs
            ├── client.rs
            └── model.rs
```

## Development

### Setup

```bash
# Clone repository
git clone <repository-url>
cd wqb-rs

# Build project
cargo build

# Run checks
cargo check
cargo clippy
cargo fmt --check
```

### Adding Features

1. Add data structures in `model.rs`
2. Implement API methods in `client.rs`
3. Add tests in the test module
4. Run tests to verify functionality

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- Issues: [GitHub Issues](https://github.com/yourusername/wqb-rs/issues)
- Documentation: [docs.rs/ace](https://docs.rs/ace)

## Disclaimer

This library requires a valid WorldQuant Brain account. Users must comply with WorldQuant's terms of service and usage policies. This is an unofficial third-party library not affiliated with WorldQuant.
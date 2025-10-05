<div align="center">
  <img src="https://raw.githubusercontent.com/glin-ai/glin-sdk-rust/main/assets/glin-coin.svg" alt="GLIN Logo" width="100" height="100">

  # glin-client

[![crates.io](https://img.shields.io/crates/v/glin-client.svg)](https://crates.io/crates/glin-client)
[![docs.rs](https://docs.rs/glin-client/badge.svg)](https://docs.rs/glin-client)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](../LICENSE)

Network connection and RPC operations for GLIN Network.

</div>

## Overview

This crate provides high-level client functionality for interacting with GLIN Network nodes:

- **Connection management**: Connect to GLIN Network RPC endpoints
- **Account management**: Create and manage blockchain accounts
- **Transaction handling**: Sign and submit transactions
- **Event monitoring**: Listen to blockchain events
- **RPC operations**: Query blockchain state and metadata

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
glin-client = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

### Example

```rust
use glin_client::{create_client, get_dev_account};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to local node
    let client = create_client("ws://localhost:9944").await?;

    // Get development account
    let account = get_dev_account("alice")?;

    // Query account balance
    let balance = client.get_balance(&account.public_key()).await?;
    println!("Balance: {}", balance);

    Ok(())
}
```

## Features

- **Async/await support**: Built on Tokio for async operations
- **Type-safe**: Leverages Rust's type system for safe blockchain interactions
- **Error handling**: Comprehensive error types with anyhow integration
- **Flexible connection**: Support for WebSocket RPC endpoints

## Part of GLIN SDK

This crate is part of the [GLIN SDK for Rust](https://github.com/glin-ai/glin-sdk-rust), providing complete blockchain interaction capabilities for GLIN Network.

### Related Crates

- [`glin-types`](https://crates.io/crates/glin-types) - Shared type definitions
- [`glin-contracts`](https://crates.io/crates/glin-contracts) - Contract deployment and interaction

## Documentation

For full SDK documentation and examples, see the [main repository](https://github.com/glin-ai/glin-sdk-rust).

## License

Apache-2.0

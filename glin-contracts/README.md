# glin-contracts

[![crates.io](https://img.shields.io/crates/v/glin-contracts.svg)](https://crates.io/crates/glin-contracts)
[![docs.rs](https://docs.rs/glin-contracts/badge.svg)](https://docs.rs/glin-contracts)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](../LICENSE)

Contract metadata, deployment, and interaction utilities for GLIN Network.

## Overview

This crate provides comprehensive tools for working with ink! smart contracts on GLIN Network:

- **Contract deployment**: Deploy compiled contracts to the blockchain
- **Contract interaction**: Call contract methods and query state
- **Metadata handling**: Parse and work with contract metadata
- **Event decoding**: Decode contract events from blockchain
- **Encoding utilities**: SCALE encoding for contract calls

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
glin-contracts = "0.1.0"
glin-client = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

### Example

```rust
use glin_client::{create_client, get_dev_account};
use glin_contracts::{deploy_contract, call_contract};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to network
    let client = create_client("ws://localhost:9944").await?;
    let signer = get_dev_account("alice")?;

    // Deploy contract
    let result = deploy_contract(
        &client,
        &signer,
        "path/to/contract.contract",
        vec![], // constructor args
        0,      // value
    ).await?;

    println!("Contract deployed at: {}", result.contract_address);

    // Call contract method
    call_contract(
        &client,
        &signer,
        &result.contract_address,
        "transfer",
        vec![], // method args
        0,      // value
    ).await?;

    Ok(())
}
```

## Features

- **Metadata parsing**: Extract ABI and constructor information from .contract files
- **Type-safe encoding**: SCALE encoding for contract arguments
- **Gas estimation**: Automatic gas limit calculation
- **Event monitoring**: Listen and decode contract events
- **Error handling**: Detailed error messages for contract operations

## Contract Metadata

This crate works with ink! contract metadata format (`.contract` files), which includes:

- Contract ABI (messages, constructors, events)
- Contract WASM bytecode
- Type definitions (SCALE info)

## Part of GLIN SDK

This crate is part of the [GLIN SDK for Rust](https://github.com/glin-ai/glin-sdk-rust), providing complete blockchain interaction capabilities for GLIN Network.

### Related Crates

- [`glin-types`](https://crates.io/crates/glin-types) - Shared type definitions
- [`glin-client`](https://crates.io/crates/glin-client) - Network connection and RPC operations

## Documentation

For full SDK documentation, contract examples, and deployment guides, see the [main repository](https://github.com/glin-ai/glin-sdk-rust).

## License

Apache-2.0

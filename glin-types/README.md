# glin-types

[![crates.io](https://img.shields.io/crates/v/glin-types.svg)](https://crates.io/crates/glin-types)
[![docs.rs](https://docs.rs/glin-types/badge.svg)](https://docs.rs/glin-types)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](../LICENSE)

Shared types and data structures for GLIN Network SDK.

## Overview

This crate provides core type definitions used across the GLIN SDK ecosystem. It includes:

- **Serialization support**: Serde for JSON serialization
- **SCALE encoding**: For Substrate blockchain compatibility
- **Substrate types**: Integration with subxt for blockchain interactions
- **Common data structures**: Shared across glin-client and glin-contracts

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
glin-types = "0.1.0"
```

## Features

- Serde-based JSON serialization/deserialization
- SCALE codec support for blockchain encoding
- Hexadecimal encoding/decoding utilities
- Substrate-compatible type definitions

## Part of GLIN SDK

This crate is part of the [GLIN SDK for Rust](https://github.com/glin-ai/glin-sdk-rust), which provides complete blockchain interaction capabilities for GLIN Network.

### Related Crates

- [`glin-client`](https://crates.io/crates/glin-client) - Network connection and RPC operations
- [`glin-contracts`](https://crates.io/crates/glin-contracts) - Contract deployment and interaction

## Documentation

For full SDK documentation, see the [main repository](https://github.com/glin-ai/glin-sdk-rust).

## License

Apache-2.0

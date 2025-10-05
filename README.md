<div align="center">
  <img src="https://raw.githubusercontent.com/glin-ai/glin-sdk-rust/main/assets/glin-coin.svg" alt="GLIN Logo" width="120" height="120">

  # GLIN SDK - Rust

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/glin-sdk-rust.svg)](https://crates.io/crates/glin-sdk-rust)

Official Rust SDK for [GLIN Network](https://glin.ai) - A decentralized AI training platform built on Substrate.

</div>

## 🎯 Overview

Complete Rust SDK for building applications on GLIN Network. Provides **all core blockchain features** (same as TypeScript and Python SDKs) plus **Rust-specific extensions** for high-performance tools.

### ✅ Core Features (Standard across all GLIN SDKs)

- 🌐 Network connection and RPC
- 🔐 Account management and signing
- 📜 Contract deployment and interaction
- 💸 Transaction building and submission
- 📡 Event subscriptions
- 📋 Metadata parsing

### 🚀 Rust-Specific Extensions

- 🎨 CLI tools with colored output and progress bars
- ⚡ High-performance blockchain indexing
- ✅ Contract verification utilities
- 🔧 Type-safe contract code generation

## 📚 Documentation

**[📖 Full Documentation →](https://docs.glin.ai/sdk/rust/setup)**

- **[Getting Started →](https://docs.glin.ai/sdk/getting-started/overview)**
- **[Rust SDK Setup →](https://docs.glin.ai/sdk/rust/setup)**
- **[Examples →](https://docs.glin.ai/sdk/examples/create-cli-tool)**
- **[API Reference →](https://docs.glin.ai/sdk/rust/api-reference)**

## 📦 Workspace Structure

This is a Cargo workspace containing four crates:

- **glin-client**: Network connection, accounts, and RPC operations
- **glin-contracts**: Contract metadata, deployment, interaction, and verification
- **glin-types**: Shared types and data structures
- **glin-indexer**: Blockchain indexing utilities (block streaming, event decoding)

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
glin-client = "0.1.0"
glin-contracts = "0.1.0"

# Or use local path during development
glin-client = { path = "../glin-sdk-rust/glin-client" }
glin-contracts = { path = "../glin-sdk-rust/glin-contracts" }
```

### Network Connection

```rust
use glin_client::{create_client, GlinClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to network
    let client = create_client("wss://testnet.glin.ai").await?;

    // Use client for RPC calls
    let block_hash = client.rpc().finalized_head().await?;
    println!("Latest finalized block: {:?}", block_hash);

    Ok(())
}
```

### Account Management

```rust
use glin_client::{get_dev_account, account_from_seed, get_address};

// Development accounts
let alice = get_dev_account("alice")?;
println!("Alice address: {}", get_address(&alice));

// From seed phrase
let keypair = account_from_seed("//Alice")?;

// From mnemonic
let mnemonic = "word1 word2 word3 ... word12";
let keypair = account_from_seed(mnemonic)?;
```

### Contract Metadata Fetching

```rust
use glin_contracts::{fetch_contract_metadata, MetadataFetchOptions};

let options = MetadataFetchOptions {
    local_path: None,
    explorer_url: Some("https://glincscan.com".to_string()),
    cache_dir: Some("/home/user/.glin/cache".to_string()),
};

let metadata = fetch_contract_metadata(
    &client,
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    options
).await?;
```

### Contract Information

```rust
use glin_contracts::get_contract_info;

let info = get_contract_info(&client, contract_address).await?;
println!("Code hash: 0x{}", hex::encode(info.code_hash));
println!("Storage deposit: {}", info.storage_deposit);
```

## 🛠️ Use Cases

### For Application Developers

Build DApps and services on GLIN Network:

```rust
use glin_client::GlinClient;
use glin_contracts::Contract;

// Connect and interact with contracts
let client = create_client("wss://testnet.glin.ai").await?;
let contract = Contract::new(&client, address, metadata)?;
```

### For CLI Tool Developers

Build developer tools like [glin-forge](https://github.com/glin-ai/glin-forge):

```rust
use colored::Colorize;
use glin_client::create_client;

println!("{} Deploying contract...", "→".cyan());
let client = create_client(rpc_url).await?;
println!("{} Connected!", "✓".green().bold());
```

### For Indexer Developers

Build high-performance blockchain indexers like [glincscan](https://github.com/glin-ai/glincscan):

```rust
use glin_client::create_client;
use glin_indexer::{BlockStream, EventDecoder, ExtrinsicParser};
use futures::StreamExt;

let client = create_client("wss://testnet.glin.ai").await?;
let decoder = EventDecoder::new(&client)?;
let parser = ExtrinsicParser::new();

let mut stream = BlockStream::subscribe_finalized(&client).await?;

while let Some(block) = stream.next().await {
    let block = block?;

    // Parse extrinsics
    for ext in block.extrinsics().await?.iter() {
        let info = parser.parse(&ext?, block.number())?;
        // Store in database...
    }

    // Decode events
    for event in block.events().await?.iter() {
        let decoded = decoder.decode(&event?)?;
        // Store in database...
    }
}
```

See [examples/block_indexer.rs](examples/block_indexer.rs) for a complete example.

## 🏗️ Projects Using This SDK

- **[glin-forge](https://github.com/glin-ai/glin-forge)**: CLI tools for ink! contract development
- **[glincscan](https://github.com/glin-ai/glincscan)** (planned): Blockchain explorer and indexer
- **Your project here!** 🚀

## 📚 Architecture

```
glin-sdk-rust/
├── glin-client/       # Network & RPC
│   ├── Connection management
│   ├── Account utilities
│   ├── Block subscriptions
│   └── Batch operations
│
├── glin-contracts/    # Contract utilities
│   ├── Metadata fetching
│   ├── Chain info queries
│   ├── Encoding/decoding
│   ├── Metadata parsing
│   └── Contract verification
│
├── glin-types/        # Shared types
│   ├── Block types
│   ├── Event types
│   ├── Extrinsic types
│   ├── Account types
│   └── Contract types
│
└── glin-indexer/      # Indexing utilities (NEW in v0.2.0)
    ├── BlockStream - Block subscription helper
    ├── EventDecoder - Event decoding utilities
    └── ExtrinsicParser - Transaction parsing
```

## 🔗 Related SDKs

GLIN Network provides SDKs for multiple languages:

- **[glin-sdk](https://github.com/glin-ai/glin-sdk)**: TypeScript/JavaScript SDK (frontend + backend)
- **glin-sdk-rust** (this repo): Rust SDK (backend + CLI tools)
- **glin-sdk-python** (planned): Python SDK (data science + analytics)

All SDKs share the same **core features**, with language-specific extensions. See **[Rust SDK Documentation →](https://docs.glin.ai/sdk/rust/setup)** for details.

## 🛠️ Development

```bash
# Clone repository
git clone https://github.com/glin-ai/glin-sdk-rust.git
cd glin-sdk-rust

# Build all crates
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Lint
cargo clippy --all-targets --all-features -- -D warnings
```

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📄 License

Apache-2.0 - see [LICENSE](LICENSE) for details

## 🔗 Links

- **Website**: https://glin.ai
- **Documentation**: https://docs.glin.ai
- **GitHub**: https://github.com/glin-ai
- **Discord**: https://discord.gg/glin-ai
- **Twitter**: https://twitter.com/glin_ai

## 📧 Contact

- General: hello@glin.ai
- Technical: dev@glin.ai
- Security: security@glin.ai

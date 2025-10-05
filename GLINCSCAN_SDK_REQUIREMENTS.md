# glincscan SDK Requirements Analysis

> **Document Purpose**: Comprehensive analysis of current glin-sdk-rust capabilities and requirements for building glincscan (blockchain explorer and contract verification platform).

**Date**: October 5, 2025
**SDK Version**: 0.1.2
**Target**: glincscan - Blockchain Explorer, Indexer, and Contract Verifier

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Current SDK Capabilities](#current-sdk-capabilities)
3. [glincscan Architecture Requirements](#glincscan-architecture-requirements)
4. [Gap Analysis](#gap-analysis)
5. [Implementation Guidance](#implementation-guidance)
6. [New SDK Features Needed](#new-sdk-features-needed)
7. [Recommendations](#recommendations)

---

## Executive Summary

### Current State

The glin-sdk-rust provides a **minimal but functional foundation** for blockchain interaction:

- ✅ **Basic RPC connectivity** via subxt 0.44
- ✅ **Account management** with keypair utilities
- ✅ **Contract metadata parsing** using ink_metadata 5.1
- ✅ **Contract info queries** (code hash, storage deposit)
- ✅ **Metadata fetching** with multi-strategy fallback
- ✅ **SCALE encoding/decoding** for contract arguments

### What's Missing for glincscan

The SDK lacks **critical features** needed for a production blockchain explorer:

- ❌ **Block subscription and streaming**
- ❌ **Event decoding utilities**
- ❌ **Extrinsic parsing and indexing**
- ❌ **Transaction history queries**
- ❌ **Contract deployment tracking**
- ❌ **Batch RPC operations**
- ❌ **Database integration helpers**
- ❌ **WASM verification utilities**

**Bottom Line**: The SDK is designed for **application developers** (build DApps), but glincscan needs **indexer-specific features** for blockchain data extraction and verification.

---

## Current SDK Capabilities

### 1. glin-client (Network & RPC)

**File**: `/home/eralp/Projects/glin/glin-sdk-rust/glin-client/src/lib.rs`

#### Features Available

| Feature | Status | Code Location | Usage |
|---------|--------|---------------|-------|
| RPC Connection | ✅ Complete | `create_client()` | Connect to GLIN nodes via WebSocket |
| Legacy RPC Methods | ✅ Complete | `create_rpc_client()` | Direct RPC calls for low-level ops |
| Dev Accounts | ✅ Complete | `get_dev_account()` | Alice, Bob, Charlie, etc. |
| Account from Seed | ✅ Complete | `account_from_seed()` | BIP39 mnemonic + secret URI |
| Address Formatting | ✅ Complete | `get_address()` | SS58 address from keypair |

#### Example: Basic Connection

```rust
use glin_client::{create_client, GlinClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to network
    let client = create_client("wss://testnet.glin.ai").await?;

    // Get finalized block
    let block_hash = client.rpc().finalized_head().await?;
    println!("Latest finalized: {:?}", block_hash);

    Ok(())
}
```

#### Limitations

- ⚠️ **No block subscription helpers** - Must use raw subxt API
- ⚠️ **No event streaming utilities** - Must manually subscribe
- ⚠️ **No transaction history** - Only current state queries
- ⚠️ **No batch operations** - One RPC call at a time

---

### 2. glin-contracts (Contract Utilities)

**Files**:
- `/home/eralp/Projects/glin/glin-sdk-rust/glin-contracts/src/chain_info.rs`
- `/home/eralp/Projects/glin/glin-sdk-rust/glin-contracts/src/metadata.rs`
- `/home/eralp/Projects/glin/glin-sdk-rust/glin-contracts/src/metadata_fetcher.rs`
- `/home/eralp/Projects/glin/glin-sdk-rust/glin-contracts/src/encoding.rs`

#### Features Available

| Feature | Status | Module | Capability |
|---------|--------|--------|------------|
| Contract Info Query | ✅ Complete | `chain_info` | Get code hash & storage deposit |
| Metadata Parsing | ✅ Complete | `metadata` | Parse ink! metadata JSON |
| Metadata Fetching | ✅ Complete | `metadata_fetcher` | Multi-strategy: local/cache/explorer |
| SCALE Encoding | ✅ Complete | `encoding` | Encode contract call arguments |
| SCALE Decoding | ⚠️ Partial | `encoding` | Decode primitives only |
| Constructor Info | ✅ Complete | `metadata` | Get constructor specs |
| Message Info | ✅ Complete | `metadata` | Get message selectors & types |

#### Example: Contract Info Query

```rust
use glin_contracts::get_contract_info;
use glin_client::create_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = create_client("wss://testnet.glin.ai").await?;

    let info = get_contract_info(
        &client,
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    ).await?;

    println!("Code hash: 0x{}", hex::encode(info.code_hash));
    println!("Storage deposit: {}", info.storage_deposit);

    Ok(())
}
```

#### Example: Metadata Fetching Strategy

```rust
use glin_contracts::{fetch_contract_metadata, MetadataFetchOptions};

let options = MetadataFetchOptions {
    local_path: None,
    explorer_url: Some("https://glincscan.com".to_string()),
    cache_dir: Some("/home/user/.glin/cache".to_string()),
};

// Tries: local file → cache → explorer API
let metadata = fetch_contract_metadata(
    &client,
    contract_address,
    options
).await?;
```

#### Limitations

- ⚠️ **Decoding limited to primitives** - Complex types return hex strings
- ⚠️ **No contract event decoding** - Only storage queries
- ⚠️ **No deployment tracking** - Can't watch for Instantiated events
- ⚠️ **No code storage queries** - Can't fetch WASM by code hash

---

### 3. glin-types (Shared Types)

**File**: `/home/eralp/Projects/glin/glin-sdk-rust/glin-types/src/lib.rs`

#### Current State

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
```

⚠️ **Nearly empty** - This crate is a placeholder with no meaningful types yet.

#### What Should Be Here

This crate should contain:
- Block, transaction, and event types
- Account and balance types
- Custom pallet types (TaskRegistry, ProviderStaking, etc.)
- Common error types
- Encoding/decoding traits

---

## glincscan Architecture Requirements

Based on the explorer template and indexer documentation, glincscan needs **three major components**:

### Architecture Overview

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   Indexer   │─────>│  Database   │<─────│  Query API  │
│             │      │ (PostgreSQL)│      │             │
└─────────────┘      └─────────────┘      └─────────────┘
       │                                          │
       │                                          │
       v                                          v
  GLIN Chain                               Web Interface
  (via RPC)                               (Explorer UI)

                    ┌─────────────┐
                    │  Verifier   │
                    │  Service    │
                    └─────────────┘
                           │
                           v
                    Contract Source
                    Verification
```

### Component 1: Indexer

**Purpose**: Real-time blockchain data ingestion

#### Required SDK Features

| Feature | Priority | Description |
|---------|----------|-------------|
| Block Subscription | 🔴 Critical | Subscribe to finalized blocks |
| Event Streaming | 🔴 Critical | Decode and stream events |
| Extrinsic Parsing | 🔴 Critical | Parse transaction details |
| Batch Processing | 🟡 Important | Process multiple blocks in parallel |
| Contract Events | 🟡 Important | Decode contract-specific events |
| Storage Snapshots | 🟢 Nice-to-have | Historical state queries |

#### Data to Index

**Core Blockchain Data:**
- ✅ Blocks (number, hash, parent_hash, timestamp, validator)
- ✅ Extrinsics (hash, signer, call data, success/failure)
- ✅ Events (pallet, method, data, extrinsic_index)
- ✅ Transfers (from, to, amount, block_number)

**Custom Pallet Data:**
- ✅ TaskRegistry: Tasks created, started, completed, cancelled
- ✅ ProviderStaking: Providers registered, stakes updated
- ✅ RewardDistribution: Rewards claimed, distributions made
- ✅ TestnetPoints: Points awarded (already in backend)

**Contract-Specific Data:**
- ✅ Contract deployments (Instantiated events)
- ✅ Contract calls (transaction → contract mapping)
- ✅ Contract events (custom events from contracts)

---

### Component 2: Query API

**Purpose**: Fast data retrieval for explorer UI

#### Required SDK Features

| Feature | Priority | Description |
|---------|----------|-------------|
| Block Queries | 🔴 Critical | Get block by number/hash |
| Transaction Queries | 🔴 Critical | Get tx details with events |
| Account Queries | 🔴 Critical | Balance, nonce, history |
| Contract Queries | 🔴 Critical | Metadata, calls, events |
| Search Support | 🟡 Important | Search by hash, address, ID |
| Pagination | 🟡 Important | Efficient large result sets |

#### Example Queries

```rust
// Block query
GET /api/blocks/12345
{
  "number": 12345,
  "hash": "0x...",
  "timestamp": "2024-10-05T10:30:00Z",
  "extrinsics": [...],
  "events": [...]
}

// Account query
GET /api/accounts/5GrwvaEF...
{
  "address": "5GrwvaEF...",
  "balance": "1000000000000000000",
  "nonce": 42,
  "transactions": [...],
  "contracts": [...]
}

// Contract query
GET /api/contracts/5GrwvaEF.../metadata
{
  "address": "5GrwvaEF...",
  "codeHash": "0x...",
  "metadata": {...},
  "verified": true,
  "sourceCode": "..."
}
```

---

### Component 3: Verifier

**Purpose**: Contract source code verification

#### Required SDK Features

| Feature | Priority | Description |
|---------|----------|-------------|
| Metadata Parsing | ✅ Exists | Already implemented |
| WASM Hash Extraction | 🔴 Critical | Get code hash from compiled WASM |
| Source Compilation | 🔴 Critical | Compile Rust → WASM |
| Hash Comparison | 🔴 Critical | Verify compiled matches deployed |
| Metadata Comparison | 🟡 Important | Ensure metadata consistency |
| ABI Compatibility | 🟢 Nice-to-have | Check interface compatibility |

#### Verification Flow

```
1. User uploads source code + Cargo.toml
2. SDK compiles: cargo contract build
3. Extract WASM hash from bundle
4. Query on-chain code hash: ContractInfoOf
5. Compare hashes → verified ✓ or failed ✗
6. Store source code + metadata in database
```

---

## Gap Analysis

### Legend
- ✅ **Already exists** - Ready to use
- ⚠️ **Partially exists** - Needs extension
- ❌ **Missing** - Must be implemented

---

### Indexer Requirements

| Feature | Status | Current Implementation | What's Missing |
|---------|--------|------------------------|----------------|
| Block Subscription | ❌ Missing | N/A | Helper for `client.blocks().subscribe_finalized()` |
| Event Decoding | ❌ Missing | N/A | Decode events with runtime metadata |
| Extrinsic Parsing | ❌ Missing | N/A | Extract call data, signer, success status |
| Batch Block Fetching | ❌ Missing | N/A | Parallel block processing utilities |
| Contract Event Tracking | ❌ Missing | N/A | Track `Contracts.ContractEmitted` events |
| Transfer Extraction | ❌ Missing | N/A | Parse `Balances.Transfer` events |
| Custom Pallet Events | ❌ Missing | N/A | Decode TaskRegistry, ProviderStaking events |
| Database Integration | ❌ Missing | N/A | sqlx helpers, models, migrations |
| Sync State Management | ❌ Missing | N/A | Track last indexed block |

#### Example: What's Missing

**Current SDK:**
```rust
// ❌ No helper for this common pattern
let client = create_client("wss://testnet.glin.ai").await?;
// Must use raw subxt:
let mut blocks = client.blocks().subscribe_finalized().await?;
while let Some(block) = blocks.next().await { ... }
```

**What We Need:**
```rust
// ✅ Higher-level indexer API
use glin_indexer::BlockStream;

let mut stream = BlockStream::subscribe_finalized(&client).await?;

stream.on_block(|block| async move {
    println!("Block #{}: {} extrinsics", block.number, block.extrinsics.len());
});

stream.on_event(|event| async move {
    if event.pallet == "Balances" && event.method == "Transfer" {
        // Auto-decoded transfer data
        let transfer: Transfer = event.decode()?;
        println!("Transfer: {} -> {} ({})", transfer.from, transfer.to, transfer.amount);
    }
});

stream.start().await?;
```

---

### Query API Requirements

| Feature | Status | Current Implementation | What's Missing |
|---------|--------|------------------------|----------------|
| Block by Number | ⚠️ Partial | `client.rpc().block_hash()` | Wrapper with full block details |
| Block by Hash | ⚠️ Partial | `client.blocks().at()` | Include events & extrinsics |
| Account Balance | ⚠️ Partial | `client.storage().at_latest()` | Higher-level account API |
| Transaction Details | ❌ Missing | N/A | Extrinsic + events together |
| Contract Metadata | ✅ Exists | `fetch_contract_metadata()` | ✅ Ready to use |
| Contract Calls | ❌ Missing | N/A | List all calls to a contract |
| Search Index | ❌ Missing | N/A | Search by hash/address/ID |
| Pagination | ❌ Missing | N/A | Cursor-based pagination |

---

### Verifier Requirements

| Feature | Status | Current Implementation | What's Missing |
|---------|--------|------------------------|----------------|
| Metadata Parsing | ✅ Exists | `glin_contracts::metadata` | ✅ Ready to use |
| Code Hash Query | ✅ Exists | `get_contract_info()` | ✅ Ready to use |
| WASM Compilation | ❌ Missing | N/A | Invoke `cargo contract build` |
| WASM Hash Extraction | ❌ Missing | N/A | Read hash from .contract bundle |
| Code Storage Query | ❌ Missing | N/A | Fetch WASM by code hash |
| Source Code Storage | ❌ Missing | N/A | Database models for verified contracts |
| Metadata Comparison | ⚠️ Partial | Can parse both | Need comparison logic |
| Build Environment | ❌ Missing | N/A | Reproducible build config |

---

## Implementation Guidance

### Using Existing SDK Features

#### 1. Connect to Network

```rust
use glin_client::create_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = create_client("wss://testnet.glin.ai").await?;
    println!("Connected to GLIN Network");
    Ok(())
}
```

#### 2. Subscribe to Blocks (Using Raw Subxt)

```rust
use glin_client::create_client;
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = create_client("wss://testnet.glin.ai").await?;

    // Subscribe to finalized blocks
    let mut blocks = client.blocks().subscribe_finalized().await?;

    while let Some(Ok(block)) = blocks.next().await {
        let number = block.number();
        let hash = block.hash();

        println!("Block #{}: {:?}", number, hash);

        // Get events for this block
        let events = block.events().await?;

        for event in events.iter() {
            let event = event?;
            println!("  Event: {}::{}", event.pallet_name(), event.variant_name());
        }
    }

    Ok(())
}
```

#### 3. Query Contract Info

```rust
use glin_client::create_client;
use glin_contracts::get_contract_info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = create_client("wss://testnet.glin.ai").await?;

    let contract_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let info = get_contract_info(&client, contract_address).await?;

    println!("Code Hash: 0x{}", hex::encode(&info.code_hash));
    println!("Storage Deposit: {}", info.storage_deposit);

    Ok(())
}
```

#### 4. Fetch Contract Metadata

```rust
use glin_client::create_client;
use glin_contracts::{fetch_contract_metadata, MetadataFetchOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = create_client("wss://testnet.glin.ai").await?;

    let options = MetadataFetchOptions {
        local_path: None,
        explorer_url: Some("https://glincscan.com".to_string()),
        cache_dir: Some(dirs::home_dir()
            .unwrap()
            .join(".glin/cache")
            .to_string_lossy()
            .to_string()),
    };

    let metadata = fetch_contract_metadata(
        &client,
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        options
    ).await?;

    println!("Contract version: {}", metadata.version());

    Ok(())
}
```

#### 5. Parse Extrinsics (Manual)

```rust
use glin_client::create_client;
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = create_client("wss://testnet.glin.ai").await?;

    let mut blocks = client.blocks().subscribe_finalized().await?;

    while let Some(Ok(block)) = blocks.next().await {
        let extrinsics = block.extrinsics().await?;

        for ext in extrinsics.iter() {
            let ext = ext?;

            // Get extrinsic index
            let index = ext.index();

            // Check if it's signed
            if let Some(signed_extensions) = ext.signed_extensions() {
                // Extract signer (requires custom logic)
                println!("Extrinsic #{}: signed", index);
            } else {
                println!("Extrinsic #{}: unsigned", index);
            }

            // Get call details (pallet and function name)
            println!("  Call: {}::{}", ext.pallet_name()?, ext.variant_name()?);
        }
    }

    Ok(())
}
```

---

## New SDK Features Needed

### Recommendation: Create `glin-indexer` Crate

Add a new crate to the workspace specifically for indexing utilities:

```toml
# Cargo.toml
[workspace]
members = [
    "glin-client",
    "glin-contracts",
    "glin-types",
    "glin-indexer",  # NEW
]
```

---

### Feature 1: Block Streaming API

**Crate**: `glin-indexer`

```rust
// glin-indexer/src/block_stream.rs

use glin_client::GlinClient;
use futures::Stream;
use std::pin::Pin;

pub struct BlockStream {
    client: GlinClient,
}

impl BlockStream {
    pub async fn subscribe_finalized(client: &GlinClient) -> Result<Self> {
        Ok(Self { client: client.clone() })
    }

    pub async fn subscribe_best(client: &GlinClient) -> Result<Self> {
        Ok(Self { client: client.clone() })
    }

    pub fn with_events(self) -> BlockStreamWithEvents {
        BlockStreamWithEvents { stream: self }
    }

    pub fn with_extrinsics(self) -> BlockStreamWithExtrinsics {
        BlockStreamWithExtrinsics { stream: self }
    }
}

impl Stream for BlockStream {
    type Item = Result<Block>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Implementation using subxt's block subscription
        todo!()
    }
}

// Usage
let mut stream = BlockStream::subscribe_finalized(&client).await?
    .with_events()
    .with_extrinsics();

while let Some(block) = stream.next().await {
    println!("Block #{}: {} events, {} extrinsics",
        block.number,
        block.events.len(),
        block.extrinsics.len()
    );
}
```

---

### Feature 2: Event Decoder

**Crate**: `glin-indexer`

```rust
// glin-indexer/src/event_decoder.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedEvent {
    pub pallet: String,
    pub method: String,
    pub data: serde_json::Value,
    pub block_number: u64,
    pub event_index: u32,
}

pub struct EventDecoder {
    metadata: RuntimeMetadata,
}

impl EventDecoder {
    pub fn new(client: &GlinClient) -> Result<Self> {
        let metadata = client.metadata();
        Ok(Self { metadata })
    }

    pub fn decode(&self, event: &RuntimeEvent) -> Result<DecodedEvent> {
        // Use runtime metadata to decode event data
        let pallet = event.pallet_name();
        let method = event.variant_name();

        // Decode data based on event type
        let data = self.decode_event_data(pallet, method, event.field_bytes())?;

        Ok(DecodedEvent {
            pallet: pallet.to_string(),
            method: method.to_string(),
            data,
            block_number: event.block_number(),
            event_index: event.index(),
        })
    }

    fn decode_event_data(&self, pallet: &str, method: &str, bytes: &[u8]) -> Result<serde_json::Value> {
        // Custom decoders for known events
        match (pallet, method) {
            ("Balances", "Transfer") => self.decode_transfer(bytes),
            ("Contracts", "Instantiated") => self.decode_instantiated(bytes),
            ("Contracts", "ContractEmitted") => self.decode_contract_emitted(bytes),
            ("TaskRegistry", "TaskCreated") => self.decode_task_created(bytes),
            _ => Ok(serde_json::json!({ "raw": hex::encode(bytes) })),
        }
    }

    fn decode_transfer(&self, bytes: &[u8]) -> Result<serde_json::Value> {
        use scale::Decode;

        #[derive(Decode)]
        struct Transfer {
            from: [u8; 32],
            to: [u8; 32],
            amount: u128,
        }

        let transfer = Transfer::decode(&mut &bytes[..])?;

        Ok(serde_json::json!({
            "from": format!("0x{}", hex::encode(transfer.from)),
            "to": format!("0x{}", hex::encode(transfer.to)),
            "amount": transfer.amount.to_string(),
        }))
    }

    // Similar decoders for other events...
}

// Usage
let decoder = EventDecoder::new(&client)?;

for event in block.events {
    let decoded = decoder.decode(&event)?;
    println!("{}", serde_json::to_string_pretty(&decoded)?);
}
```

---

### Feature 3: Database Models & Integration

**Crate**: `glin-indexer`

```rust
// glin-indexer/src/db/models.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Block {
    pub id: i64,
    pub number: i64,
    pub hash: String,
    pub parent_hash: String,
    pub state_root: String,
    pub extrinsics_root: String,
    pub timestamp: i64,
    pub validator: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Extrinsic {
    pub id: i64,
    pub block_id: i64,
    pub index: i32,
    pub hash: String,
    pub signer: Option<String>,
    pub pallet: String,
    pub call: String,
    pub args: serde_json::Value,
    pub success: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Event {
    pub id: i64,
    pub block_id: i64,
    pub extrinsic_id: Option<i64>,
    pub index: i32,
    pub pallet: String,
    pub method: String,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Contract {
    pub id: i64,
    pub address: String,
    pub code_hash: String,
    pub deployer: String,
    pub deployed_at_block: i64,
    pub metadata: Option<serde_json::Value>,
    pub verified: bool,
    pub source_code: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// glin-indexer/src/db/repository.rs

use sqlx::PgPool;

pub struct BlockRepository {
    pool: PgPool,
}

impl BlockRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, block: &Block) -> Result<i64> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO blocks (number, hash, parent_hash, state_root, extrinsics_root, timestamp, validator)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            block.number,
            block.hash,
            block.parent_hash,
            block.state_root,
            block.extrinsics_root,
            block.timestamp,
            block.validator
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn get_by_number(&self, number: i64) -> Result<Option<Block>> {
        let block = sqlx::query_as!(
            Block,
            "SELECT * FROM blocks WHERE number = $1",
            number
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(block)
    }

    pub async fn get_latest(&self) -> Result<Option<Block>> {
        let block = sqlx::query_as!(
            Block,
            "SELECT * FROM blocks ORDER BY number DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(block)
    }
}

// Similar repositories for Extrinsic, Event, Contract...
```

---

### Feature 4: Contract Verifier

**Crate**: `glin-contracts` (extend existing)

```rust
// glin-contracts/src/verifier.rs

use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct ContractVerifier {
    workspace_dir: PathBuf,
}

impl ContractVerifier {
    pub fn new<P: AsRef<Path>>(workspace_dir: P) -> Self {
        Self {
            workspace_dir: workspace_dir.as_ref().to_path_buf(),
        }
    }

    pub async fn verify(
        &self,
        source_code: &str,
        cargo_toml: &str,
        deployed_code_hash: &[u8; 32],
    ) -> Result<VerificationResult> {
        // 1. Create temporary workspace
        let temp_dir = tempfile::tempdir()?;

        // 2. Write source files
        std::fs::write(temp_dir.path().join("lib.rs"), source_code)?;
        std::fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)?;

        // 3. Compile contract
        let output = Command::new("cargo")
            .arg("contract")
            .arg("build")
            .arg("--release")
            .current_dir(&temp_dir)
            .output()?;

        if !output.status.success() {
            return Ok(VerificationResult::CompilationFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        // 4. Extract code hash from bundle
        let bundle_path = temp_dir.path()
            .join("target")
            .join("ink")
            .join("contract.contract");

        let bundle: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(bundle_path)?
        )?;

        let compiled_hash = self.extract_code_hash(&bundle)?;

        // 5. Compare hashes
        if compiled_hash == *deployed_code_hash {
            Ok(VerificationResult::Verified {
                code_hash: hex::encode(compiled_hash),
                metadata: bundle["spec"].clone(),
            })
        } else {
            Ok(VerificationResult::HashMismatch {
                expected: hex::encode(deployed_code_hash),
                actual: hex::encode(compiled_hash),
            })
        }
    }

    fn extract_code_hash(&self, bundle: &serde_json::Value) -> Result<[u8; 32]> {
        // Extract WASM from bundle
        let wasm_hex = bundle["source"]["wasm"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No WASM in bundle"))?
            .trim_start_matches("0x");

        let wasm_bytes = hex::decode(wasm_hex)?;

        // Compute Blake2_256 hash
        use sp_core_hashing::blake2_256;
        let hash = blake2_256(&wasm_bytes);

        Ok(hash)
    }
}

#[derive(Debug)]
pub enum VerificationResult {
    Verified {
        code_hash: String,
        metadata: serde_json::Value,
    },
    HashMismatch {
        expected: String,
        actual: String,
    },
    CompilationFailed(String),
}

// Usage
let verifier = ContractVerifier::new("/tmp/glin-verifier");

let result = verifier.verify(
    source_code,
    cargo_toml,
    &deployed_code_hash
).await?;

match result {
    VerificationResult::Verified { code_hash, .. } => {
        println!("✅ Contract verified! Code hash: {}", code_hash);
    }
    VerificationResult::HashMismatch { expected, actual } => {
        println!("❌ Hash mismatch: {} != {}", expected, actual);
    }
    VerificationResult::CompilationFailed(err) => {
        println!("❌ Compilation failed: {}", err);
    }
}
```

---

### Feature 5: Batch RPC Operations

**Crate**: `glin-client` (extend existing)

```rust
// glin-client/src/batch.rs

use glin_client::GlinClient;
use futures::future::join_all;

pub struct BatchRpc {
    client: GlinClient,
}

impl BatchRpc {
    pub fn new(client: GlinClient) -> Self {
        Self { client }
    }

    pub async fn get_blocks(&self, numbers: Vec<u64>) -> Result<Vec<Block>> {
        let futures = numbers.into_iter().map(|num| {
            let client = self.client.clone();
            async move {
                let hash = client.rpc().block_hash(Some(num.into())).await?;
                let hash = hash.ok_or_else(|| anyhow::anyhow!("Block {} not found", num))?;
                client.blocks().at(hash).await
            }
        });

        let results = join_all(futures).await;

        results.into_iter().collect()
    }

    pub async fn get_balances(&self, addresses: Vec<String>) -> Result<Vec<Balance>> {
        // Similar parallel fetching
        todo!()
    }
}

// Usage
let batch = BatchRpc::new(client);
let blocks = batch.get_blocks(vec![1000, 1001, 1002, 1003]).await?;
```

---

### Feature 6: Enhanced Type System

**Crate**: `glin-types` (replace placeholder)

```rust
// glin-types/src/lib.rs

pub mod block;
pub mod extrinsic;
pub mod event;
pub mod account;
pub mod contract;
pub mod pallets;

// Re-exports
pub use block::{Block, BlockHeader, BlockHash};
pub use extrinsic::{Extrinsic, ExtrinsicHash};
pub use event::{Event, EventData};
pub use account::{Account, AccountId, Balance};
pub use contract::{ContractInfo, ContractMetadata};

// glin-types/src/block.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub hash: BlockHash,
    pub parent_hash: BlockHash,
    pub state_root: String,
    pub extrinsics_root: String,
    pub timestamp: u64,
    pub validator: Option<String>,
}

pub type BlockHash = String;

pub struct BlockHeader {
    pub number: u64,
    pub hash: BlockHash,
    pub parent_hash: BlockHash,
}

// glin-types/src/pallets/task_registry.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub creator: String,
    pub name: String,
    pub bounty: u128,
    pub status: TaskStatus,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Created,
    Recruiting,
    Active,
    Completed,
    Cancelled,
}

// Similar types for other pallets...
```

---

## Recommendations

### 1. Create New `glin-indexer` Crate ✅ RECOMMENDED

**Why**: Separate indexing concerns from general SDK usage

**Structure**:
```
glin-sdk-rust/
├── glin-client/       # Keep as is - connection & accounts
├── glin-contracts/    # Keep as is - contract metadata & calls
├── glin-types/        # Expand with comprehensive types
└── glin-indexer/      # NEW - indexing & database utilities
    ├── src/
    │   ├── lib.rs
    │   ├── block_stream.rs
    │   ├── event_decoder.rs
    │   ├── extrinsic_parser.rs
    │   ├── db/
    │   │   ├── models.rs
    │   │   ├── repositories.rs
    │   │   └── migrations/
    │   └── indexer.rs
    └── Cargo.toml
```

**Dependencies**:
```toml
[dependencies]
glin-client = { version = "0.1.2", path = "../glin-client" }
glin-types = { version = "0.1.2", path = "../glin-types" }
subxt = "0.44"
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio-rustls"] }
tokio = { version = "1.40", features = ["full"] }
futures = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

### 2. Extend `glin-contracts` with Verifier ✅ RECOMMENDED

**Add to existing crate**:
- `src/verifier.rs` - Contract verification logic
- `src/compiler.rs` - WASM compilation utilities
- `src/code_storage.rs` - Query code by hash

---

### 3. Populate `glin-types` with Runtime Types ✅ REQUIRED

**Current state**: Nearly empty placeholder
**Target state**: Comprehensive type system

**Should include**:
- Core Substrate types (Block, Extrinsic, Event, Account)
- Custom pallet types (Task, Provider, Reward, etc.)
- SCALE encoding/decoding implementations
- Conversion traits (to/from JSON, hex, etc.)

---

### 4. Database Schema Helpers ✅ RECOMMENDED

**Option A**: Include in `glin-indexer`
```rust
// Built-in migrations
pub fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await
}
```

**Option B**: Separate `glin-db` crate
```
glin-db/
├── src/
│   ├── lib.rs
│   ├── models.rs
│   ├── repositories.rs
│   └── schema.rs
└── migrations/
    ├── 001_create_blocks.sql
    ├── 002_create_extrinsics.sql
    └── ...
```

---

### 5. Event Decoder Registry ✅ IMPORTANT

**Problem**: New custom pallets need custom decoders

**Solution**: Pluggable decoder system
```rust
use glin_indexer::EventDecoder;

let mut decoder = EventDecoder::new(&client)?;

// Register custom decoders
decoder.register("TaskRegistry", "TaskCreated", |bytes| {
    // Custom decoding logic
    decode_task_created(bytes)
});

decoder.register("ProviderStaking", "ProviderRegistered", |bytes| {
    decode_provider_registered(bytes)
});

// Use decoder
for event in events {
    let decoded = decoder.decode(&event)?;
}
```

---

### 6. Documentation & Examples 📚 CRITICAL

**Add to SDK**:
- `examples/indexer.rs` - Full indexer example
- `examples/block_subscription.rs` - Subscribe to blocks
- `examples/contract_verifier.rs` - Verify contract
- `examples/query_api.rs` - Build query API

**Documentation**:
- Update README with indexer examples
- Add docs.rs documentation
- Create migration guide from subxt to glin-sdk

---

### 7. Testing Infrastructure 🧪 IMPORTANT

**Unit Tests**: Each module should have comprehensive tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_block_subscription() {
        // Mock client
        // Subscribe to blocks
        // Assert events received
    }

    #[test]
    fn test_event_decoder() {
        // Mock event bytes
        // Decode
        // Assert structure
    }
}
```

**Integration Tests**: Test against local node
```bash
# Start local node
./target/release/glin-node --dev

# Run integration tests
cargo test --features integration-tests
```

---

## Summary: Implementation Roadmap

### Phase 1: Foundation (Week 1-2)

1. ✅ Populate `glin-types` with core types
2. ✅ Create `glin-indexer` crate structure
3. ✅ Implement `BlockStream` API
4. ✅ Add database models & migrations

### Phase 2: Indexing (Week 3-4)

5. ✅ Implement `EventDecoder`
6. ✅ Add extrinsic parsing
7. ✅ Build batch RPC operations
8. ✅ Create indexer service

### Phase 3: Verification (Week 5-6)

9. ✅ Add verifier to `glin-contracts`
10. ✅ Implement WASM compilation
11. ✅ Build verification API
12. ✅ Add code storage queries

### Phase 4: API & Testing (Week 7-8)

13. ✅ Build query API helpers
14. ✅ Add comprehensive tests
15. ✅ Write documentation
16. ✅ Create examples

---

## Conclusion

The **glin-sdk-rust** provides a solid foundation for blockchain interaction, but **lacks indexer-specific features** needed for glincscan. The recommended approach is to:

1. **Extend existing crates** with missing features
2. **Create new `glin-indexer` crate** for indexing utilities
3. **Populate `glin-types`** with comprehensive type system
4. **Add contract verifier** to `glin-contracts`

This architecture will provide:
- ✅ **Clean separation** between app SDK and indexer SDK
- ✅ **Reusable components** for other projects
- ✅ **Production-ready** indexing and verification
- ✅ **Easy maintenance** with modular structure

**Next Steps**: Start with Phase 1 (Foundation) to build the core infrastructure, then proceed to indexing and verification features.

---

**Document Version**: 1.0
**Last Updated**: October 5, 2025
**Maintainer**: GLIN SDK Team

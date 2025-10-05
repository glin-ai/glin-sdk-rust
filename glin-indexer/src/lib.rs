//! Blockchain indexing utilities for GLIN Network
//!
//! This crate provides **utilities and helpers** for building blockchain indexers.
//! It follows the Ethereum SDK pattern (like ethers.js) - providing tools without
//! database, migrations, or server code.
//!
//! ## Features
//!
//! - **Block Streaming**: Subscribe to blocks with a clean, ergonomic API
//! - **Event Decoding**: Decode runtime events to structured data
//! - **Extrinsic Parsing**: Extract signer, call info, and arguments from transactions
//!
//! ## Usage
//!
//! ```rust,no_run
//! use glin_client::create_client;
//! use glin_indexer::{BlockStream, EventDecoder};
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Connect to network
//!     let client = create_client("wss://testnet.glin.ai").await?;
//!
//!     // Create decoder
//!     let decoder = EventDecoder::new(&client)?;
//!
//!     // Subscribe to finalized blocks
//!     let mut stream = BlockStream::subscribe_finalized(&client).await?;
//!
//!     while let Some(block) = stream.next().await {
//!         let block = block?;
//!         println!("Block #{}: {}", block.number(), block.hash());
//!
//!         // Decode events
//!         let events = block.events().await?;
//!         for event in events.iter() {
//!             let event = event?;
//!             let decoded = decoder.decode(&event)?;
//!             println!("  Event: {}", serde_json::to_string(&decoded)?);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture Note
//!
//! This crate provides **utilities only**. Database models, migrations, and API servers
//! should be implemented in your indexer application (e.g., glin-explorer).

pub mod block_stream;
pub mod event_decoder;
pub mod extrinsic_parser;

pub use block_stream::BlockStream;
pub use event_decoder::{DecodedEvent, EventDecoder};
pub use extrinsic_parser::ExtrinsicParser;

/// Re-export commonly used types
pub use glin_types::{Block, Event, Extrinsic, ExtrinsicInfo};

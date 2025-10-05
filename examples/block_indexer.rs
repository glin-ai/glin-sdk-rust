//! Block indexer example
//!
//! Demonstrates how to use glin-indexer utilities to subscribe to blocks
//! and decode events.
//!
//! Run with: cargo run --example block_indexer

use anyhow::Result;
use futures::StreamExt;
use glin_client::create_client;
use glin_indexer::{BlockStream, EventDecoder, ExtrinsicParser};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to GLIN Network
    println!("Connecting to GLIN testnet...");
    let client = create_client("wss://testnet.glin.ai").await?;
    println!("✓ Connected!");

    // Create helpers
    let decoder = EventDecoder::new(&client)?;
    let parser = ExtrinsicParser::new();

    // Subscribe to finalized blocks
    println!("\nSubscribing to finalized blocks...\n");
    let mut stream = BlockStream::subscribe_finalized(&client).await?;

    let mut block_count = 0;
    while let Some(block_result) = stream.next().await {
        let block = block_result?;
        let block_number = block.number();

        println!("📦 Block #{}", block_number);
        println!("   Hash: {}", block.hash());

        // Get and decode extrinsics
        let extrinsics = block.extrinsics().await?;
        println!("   Extrinsics: {}", extrinsics.len());

        for ext in extrinsics.iter() {
            let ext = ext?;
            let info = parser.parse(&ext, block_number)?;
            println!("     - {}::{} (signed: {})",
                info.pallet,
                info.call,
                info.signer.is_some()
            );
        }

        // Get and decode events
        let events = block.events().await?;
        println!("   Events: {}", events.iter().count());

        for event in events.iter() {
            let event = event?;
            let decoded = decoder.decode(&event)?;
            println!("     - {}::{}", decoded.pallet, decoded.method);
        }

        println!();

        // Stop after 5 blocks for demonstration
        block_count += 1;
        if block_count >= 5 {
            println!("✓ Indexed 5 blocks, stopping...");
            break;
        }
    }

    Ok(())
}

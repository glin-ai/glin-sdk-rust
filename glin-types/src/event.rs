//! Event-related types

use serde::{Deserialize, Serialize};

/// Simplified event representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Pallet name (e.g., "Balances", "Contracts")
    pub pallet: String,
    /// Event name (e.g., "Transfer", "Instantiated")
    pub method: String,
    /// Block number where event occurred
    pub block_number: u64,
    /// Event index within the block
    pub event_index: u32,
    /// Associated extrinsic index (if any)
    pub extrinsic_index: Option<u32>,
}

/// Decoded event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// Pallet name
    pub pallet: String,
    /// Event name
    pub method: String,
    /// Decoded data as JSON
    pub data: serde_json::Value,
    /// Block number
    pub block_number: u64,
    /// Event index
    pub event_index: u32,
}

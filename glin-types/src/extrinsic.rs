//! Extrinsic (transaction) related types

use serde::{Deserialize, Serialize};

/// Extrinsic hash (hex-encoded)
pub type ExtrinsicHash = String;

/// Simplified extrinsic representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extrinsic {
    /// Extrinsic hash
    pub hash: ExtrinsicHash,
    /// Block number
    pub block_number: u64,
    /// Extrinsic index within block
    pub index: u32,
    /// Signer address (if signed)
    pub signer: Option<String>,
    /// Whether the extrinsic succeeded
    pub success: bool,
}

/// Detailed extrinsic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrinsicInfo {
    /// Extrinsic hash
    pub hash: ExtrinsicHash,
    /// Block number
    pub block_number: u64,
    /// Extrinsic index
    pub index: u32,
    /// Signer address
    pub signer: Option<String>,
    /// Pallet name (e.g., "Balances", "Contracts")
    pub pallet: String,
    /// Call name (e.g., "transfer", "instantiate")
    pub call: String,
    /// Call arguments as JSON
    pub args: serde_json::Value,
    /// Whether execution was successful
    pub success: bool,
}

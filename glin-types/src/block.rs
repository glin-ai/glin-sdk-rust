//! Block-related types

use serde::{Deserialize, Serialize};

/// Block hash (hex-encoded)
pub type BlockHash = String;

/// Simplified block representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block number
    pub number: u64,
    /// Block hash
    pub hash: BlockHash,
    /// Parent block hash
    pub parent_hash: BlockHash,
    /// Timestamp (Unix timestamp in milliseconds)
    pub timestamp: u64,
}

/// Block header information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block number
    pub number: u64,
    /// Block hash
    pub hash: BlockHash,
    /// Parent block hash
    pub parent_hash: BlockHash,
    /// State root
    pub state_root: String,
    /// Extrinsics root
    pub extrinsics_root: String,
}

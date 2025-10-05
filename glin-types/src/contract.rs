//! Contract-related types

use serde::{Deserialize, Serialize};

/// Contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    /// Contract address
    pub address: String,
    /// Code hash
    pub code_hash: String,
    /// Deployer address
    pub deployer: String,
    /// Deploy block number
    pub deploy_block: u64,
    /// Whether contract is verified
    pub verified: bool,
}

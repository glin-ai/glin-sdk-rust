//! Account-related types

use serde::{Deserialize, Serialize};

/// Account identifier (SS58 address)
pub type AccountId = String;

/// Balance amount (u128 as string to avoid precision loss)
pub type Balance = String;

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account address (SS58 format)
    pub address: AccountId,
    /// Free balance
    pub balance: Balance,
    /// Account nonce
    pub nonce: u64,
}

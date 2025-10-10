//! GLIN Contract Utilities
//!
//! Utilities for interacting with ink! smart contracts on GLIN Network.

pub mod chain_info;
pub mod encoding;
pub mod metadata;
pub mod metadata_fetcher;
pub mod verifier;

// Re-export commonly used types
pub use chain_info::{get_contract_info, ContractInfo};
pub use metadata_fetcher::{fetch_contract_metadata, get_default_cache_dir, MetadataFetchOptions};
pub use verifier::{ContractVerifier, VerificationResult};

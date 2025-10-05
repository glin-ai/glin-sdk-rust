//! GLIN Contract Utilities
//!
//! Utilities for interacting with ink! smart contracts on GLIN Network.

pub mod chain_info;
pub mod encoding;
pub mod metadata;
pub mod metadata_fetcher;
pub mod verifier;

// Re-export commonly used types
pub use chain_info::{ContractInfo, get_contract_info};
pub use metadata_fetcher::{
    MetadataFetchOptions, fetch_contract_metadata, get_default_cache_dir,
};
pub use verifier::{ContractVerifier, VerificationResult};

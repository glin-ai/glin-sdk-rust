//! Shared types and data structures for GLIN Network
//!
//! This crate provides common types used across the GLIN SDK ecosystem.
//! Types are designed to be lightweight, serializable, and compatible with
//! Substrate/ink! ecosystems.

pub mod account;
pub mod block;
pub mod contract;
pub mod event;
pub mod extrinsic;

// Re-export main types
pub use account::{AccountId, Balance};
pub use block::{Block, BlockHash, BlockHeader};
pub use contract::ContractInfo;
pub use event::{Event, EventData};
pub use extrinsic::{Extrinsic, ExtrinsicInfo};

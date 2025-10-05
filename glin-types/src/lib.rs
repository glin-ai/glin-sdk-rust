//! Shared types and data structures for GLIN Network
//!
//! This crate provides common types used across the GLIN SDK ecosystem.
//! Types are designed to be lightweight, serializable, and compatible with
//! Substrate/ink! ecosystems.

pub mod block;
pub mod event;
pub mod extrinsic;
pub mod account;
pub mod contract;

// Re-export main types
pub use block::{Block, BlockHash, BlockHeader};
pub use event::{Event, EventData};
pub use extrinsic::{Extrinsic, ExtrinsicInfo};
pub use account::{AccountId, Balance};
pub use contract::ContractInfo;

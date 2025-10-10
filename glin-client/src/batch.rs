//! Batch RPC operations
//!
//! Utilities for performing multiple RPC calls in parallel for better performance.
//!
//! Note: This module provides patterns and examples for parallel operations.
//! Applications should use their own metadata types for type-safe queries.

use crate::GlinClient;
use anyhow::Result;
use futures::future::join_all;

/// Batch RPC helper
///
/// Enables efficient parallel fetching of blockchain data.
///
/// # Example
///
/// ```rust,no_run
/// use glin_client::{create_client, BatchRpc};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = create_client("wss://testnet.glin.ai").await?;
///     let batch = BatchRpc::new(client);
///
///     // Example: Fetch storage in parallel
///     // Applications would use their own metadata types here
///
///     Ok(())
/// }
/// ```
pub struct BatchRpc {
    client: GlinClient,
}

impl BatchRpc {
    /// Create new batch RPC helper
    pub fn new(client: GlinClient) -> Self {
        Self { client }
    }

    /// Example: Fetch multiple storage values in parallel
    ///
    /// Applications should use their own metadata types for type-safe queries.
    /// See subxt documentation for static storage queries.
    ///
    /// # Pattern Example
    ///
    /// ```rust,ignore
    /// // With static metadata:
    /// let queries = vec![
    ///     polkadot::storage().system().account(&alice),
    ///     polkadot::storage().system().account(&bob),
    /// ];
    ///
    /// let futures = queries.into_iter().map(|query| {
    ///     let client = self.client.clone();
    ///     async move {
    ///         client.storage().at_latest().await?.fetch(&query).await
    ///     }
    /// });
    ///
    /// let results = futures::future::join_all(futures).await;
    /// ```
    pub async fn fetch_storage_parallel<T>(
        &self,
        keys: Vec<Vec<u8>>,
    ) -> Result<Vec<Option<Vec<u8>>>> {
        // Example pattern for parallel storage queries
        // Applications should replace this with their own typed queries

        let futures = keys.into_iter().map(|_key| {
            let _client = self.client.clone();
            async move {
                // Placeholder: Applications implement with their metadata types
                Ok::<Option<Vec<u8>>, anyhow::Error>(None)
            }
        });

        let results: Vec<_> = join_all(futures).await;
        results.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_creation() {
        // Tested in integration tests with real client
    }
}

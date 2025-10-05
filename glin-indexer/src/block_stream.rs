//! Block streaming utilities
//!
//! Provides a higher-level API for subscribing to blocks from GLIN Network.

use futures::stream::Stream;
use glin_client::GlinClient;
use std::pin::Pin;
use std::task::{Context, Poll};
use subxt::blocks::Block;
use anyhow::Result;

/// Block streaming helper
///
/// Provides a clean API for subscribing to finalized or best blocks.
///
/// # Example
///
/// ```rust,no_run
/// use glin_client::create_client;
/// use glin_indexer::BlockStream;
/// use futures::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = create_client("wss://testnet.glin.ai").await?;
///     let mut stream = BlockStream::subscribe_finalized(&client).await?;
///
///     while let Some(block) = stream.next().await {
///         let block = block?;
///         println!("Block #{}: {}", block.number(), block.hash());
///     }
///     Ok(())
/// }
/// ```
pub struct BlockStream {
    inner: Pin<Box<dyn Stream<Item = Result<Block<subxt::PolkadotConfig, GlinClient>, subxt::Error>> + Send>>,
}

impl BlockStream {
    /// Subscribe to finalized blocks
    pub async fn subscribe_finalized(client: &GlinClient) -> Result<Self> {
        let subscription = client.blocks().subscribe_finalized().await?;
        Ok(Self {
            inner: Box::pin(subscription),
        })
    }

    /// Subscribe to best blocks (including non-finalized)
    pub async fn subscribe_best(client: &GlinClient) -> Result<Self> {
        let subscription = client.blocks().subscribe_best().await?;
        Ok(Self {
            inner: Box::pin(subscription),
        })
    }
}

impl Stream for BlockStream {
    type Item = Result<Block<subxt::PolkadotConfig, GlinClient>, subxt::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires running node
    async fn test_block_subscription() {
        use glin_client::create_client;

        let client = create_client("ws://localhost:9944").await.unwrap();
        let mut stream = BlockStream::subscribe_finalized(&client).await.unwrap();

        // Get first block
        if let Some(block) = stream.next().await {
            let block = block.unwrap();
            assert!(block.number() > 0);
        }
    }
}

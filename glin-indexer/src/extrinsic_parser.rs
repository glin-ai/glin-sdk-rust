//! Extrinsic parsing utilities
//!
//! Provides helpers to extract information from extrinsics (transactions).

use anyhow::Result;
use glin_client::GlinClient;
use glin_types::ExtrinsicInfo;
use subxt::blocks::ExtrinsicDetails;

/// Extrinsic parser
///
/// Extracts signer, call information, and arguments from extrinsics.
///
/// # Example
///
/// ```rust,no_run
/// use glin_indexer::ExtrinsicParser;
///
/// let parser = ExtrinsicParser::new();
/// // let info = parser.parse(&extrinsic)?;
/// // println!("Signer: {:?}", info.signer);
/// // println!("Call: {}::{}", info.pallet, info.call);
/// ```
pub struct ExtrinsicParser {}

impl ExtrinsicParser {
    /// Create new extrinsic parser
    pub fn new() -> Self {
        Self {}
    }

    /// Parse extrinsic to extract information
    pub fn parse(
        &self,
        extrinsic: &ExtrinsicDetails<subxt::PolkadotConfig, GlinClient>,
        block_number: u64,
    ) -> Result<ExtrinsicInfo> {
        let index = extrinsic.index();

        // Extract signer (if signed)
        let signer = if extrinsic.is_signed() {
            // Try to extract address from signed extensions
            // Note: This is a simplified version; production code may need
            // more sophisticated signer extraction
            extrinsic
                .address_bytes()
                .map(|bytes| format!("0x{}", hex::encode(bytes)))
        } else {
            None
        };

        // Get pallet and call name
        let pallet = extrinsic.pallet_name()?.to_string();
        let call = extrinsic.variant_name()?.to_string();

        // For now, we'll return raw bytes as hex for args
        // Production implementation could decode based on metadata
        let args = serde_json::json!({
            "raw": format!("0x{}", hex::encode(extrinsic.field_bytes()))
        });

        // Determine success (requires checking events in real implementation)
        let success = true; // Placeholder

        Ok(ExtrinsicInfo {
            hash: format!("0x{}", hex::encode(extrinsic.hash())),
            block_number,
            index,
            signer,
            pallet,
            call,
            args,
            success,
        })
    }

    /// Check if extrinsic is signed
    pub fn is_signed(&self, extrinsic: &ExtrinsicDetails<subxt::PolkadotConfig, GlinClient>) -> bool {
        extrinsic.is_signed()
    }
}

impl Default for ExtrinsicParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = ExtrinsicParser::new();
        // Parser is tested in integration tests with real extrinsics
    }
}

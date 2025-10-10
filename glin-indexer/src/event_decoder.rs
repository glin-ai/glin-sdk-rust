//! Event decoding utilities
//!
//! Provides helpers to decode runtime events into structured JSON data.

use anyhow::{anyhow, Result};
use glin_client::GlinClient;
use scale::Decode;
use serde::{Deserialize, Serialize};
use subxt::events::EventDetails;

/// Decoded event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedEvent {
    /// Pallet name
    pub pallet: String,
    /// Event method name
    pub method: String,
    /// Decoded data as JSON
    pub data: serde_json::Value,
    /// Block number
    pub block_number: u64,
    /// Event index
    pub event_index: u32,
}

/// Event decoder
///
/// Decodes runtime events into structured JSON. Supports common events
/// with specific decoders, and falls back to hex encoding for unknown events.
///
/// # Example
///
/// ```rust,no_run
/// use glin_client::create_client;
/// use glin_indexer::EventDecoder;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = create_client("wss://testnet.glin.ai").await?;
///     let decoder = EventDecoder::new(&client)?;
///
///     // Decode event
///     // let decoded = decoder.decode(&event)?;
///     // println!("{}", serde_json::to_string_pretty(&decoded)?);
///     Ok(())
/// }
/// ```
pub struct EventDecoder {
    // Could store metadata for dynamic decoding if needed
    _client: GlinClient,
}

impl EventDecoder {
    /// Create new event decoder
    pub fn new(client: &GlinClient) -> Result<Self> {
        Ok(Self {
            _client: client.clone(),
        })
    }

    /// Decode an event to structured JSON
    pub fn decode(&self, event: &EventDetails<subxt::PolkadotConfig>) -> Result<DecodedEvent> {
        let pallet = event.pallet_name();
        let method = event.variant_name();

        // Get field bytes for custom decoding
        let field_bytes = event.field_bytes();

        // Decode based on event type
        let data = match (pallet, method) {
            ("Balances", "Transfer") => self.decode_transfer(field_bytes)?,
            ("Contracts", "Instantiated") => self.decode_instantiated(field_bytes)?,
            ("Contracts", "ContractEmitted") => self.decode_contract_emitted(field_bytes)?,
            _ => {
                // Fallback: return hex-encoded raw data
                serde_json::json!({
                    "raw": format!("0x{}", hex::encode(field_bytes))
                })
            }
        };

        Ok(DecodedEvent {
            pallet: pallet.to_string(),
            method: method.to_string(),
            data,
            block_number: 0, // Set by caller
            event_index: event.index(),
        })
    }

    fn decode_transfer(&self, bytes: &[u8]) -> Result<serde_json::Value> {
        #[derive(Decode)]
        struct Transfer {
            from: [u8; 32],
            to: [u8; 32],
            amount: u128,
        }

        let transfer = Transfer::decode(&mut &bytes[..])
            .map_err(|e| anyhow!("Failed to decode Transfer event: {}", e))?;

        Ok(serde_json::json!({
            "from": format!("0x{}", hex::encode(transfer.from)),
            "to": format!("0x{}", hex::encode(transfer.to)),
            "amount": transfer.amount.to_string(),
        }))
    }

    fn decode_instantiated(&self, bytes: &[u8]) -> Result<serde_json::Value> {
        #[derive(Decode)]
        struct Instantiated {
            deployer: [u8; 32],
            contract: [u8; 32],
        }

        let inst = Instantiated::decode(&mut &bytes[..])
            .map_err(|e| anyhow!("Failed to decode Instantiated event: {}", e))?;

        Ok(serde_json::json!({
            "deployer": format!("0x{}", hex::encode(inst.deployer)),
            "contract": format!("0x{}", hex::encode(inst.contract)),
        }))
    }

    fn decode_contract_emitted(&self, bytes: &[u8]) -> Result<serde_json::Value> {
        #[derive(Decode)]
        struct ContractEmitted {
            contract: [u8; 32],
            data: Vec<u8>,
        }

        let emitted = ContractEmitted::decode(&mut &bytes[..])
            .map_err(|e| anyhow!("Failed to decode ContractEmitted event: {}", e))?;

        Ok(serde_json::json!({
            "contract": format!("0x{}", hex::encode(emitted.contract)),
            "data": format!("0x{}", hex::encode(emitted.data)),
        }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_decoder_creation() {
        // Decoder creation is tested in integration tests with real client
    }
}

// Query contract and code information from blockchain storage

use anyhow::{Context, Result};
use subxt::dynamic;
use subxt_core::storage;

use glin_client::GlinClient;

/// Contract information stored on-chain
#[derive(Debug, Clone)]
pub struct ContractInfo {
    pub code_hash: [u8; 32],
    pub storage_deposit: u128,
}

/// Get contract info from blockchain storage
pub async fn get_contract_info(
    client: &GlinClient,
    contract_address: &str,
) -> Result<ContractInfo> {
    // Parse contract address to bytes
    let address_bytes = parse_address(contract_address)?;

    // Create dynamic storage query for ContractInfoOf
    let storage_addr = dynamic::storage(
        "Contracts",
        "ContractInfoOf",
        vec![dynamic::Value::from_bytes(address_bytes)],
    );

    // Get the storage key bytes for the address
    let lookup_bytes = storage::get_address_bytes(&storage_addr, &client.metadata())
        .context("Failed to encode storage address")?;

    // Fetch raw SCALE-encoded bytes from storage
    let raw_bytes = client
        .storage()
        .at_latest()
        .await?
        .fetch_raw(lookup_bytes)
        .await
        .context("Failed to fetch contract info")?
        .ok_or_else(|| {
            anyhow::anyhow!("Contract not found at address: {}", contract_address)
        })?;

    // Decode the raw SCALE bytes into ContractInfo
    decode_contract_info_from_bytes(&raw_bytes)
}

/// Decode ContractInfo from raw SCALE-encoded bytes
///
/// Note: For now, we'll extract the code_hash from the raw SCALE-encoded bytes.
/// In a future version, we can use proper SCALE decoding with type registry.
fn decode_contract_info_from_bytes(encoded: &[u8]) -> Result<ContractInfo> {
    use scale::Decode;

    // For ContractInfo structure, we need to decode:
    // struct ContractInfo {
    //     code_hash: H256,         // 32 bytes
    //     storage_deposit: u128,   // 16 bytes (compact encoded)
    //     ...other fields
    // }

    // Simple approach: extract first 32 bytes as code_hash
    if encoded.len() < 32 {
        anyhow::bail!(
            "Encoded ContractInfo too short: {} bytes (expected at least 32)",
            encoded.len()
        );
    }

    let mut code_hash = [0u8; 32];
    code_hash.copy_from_slice(&encoded[0..32]);

    // Decode storage_deposit (u128 after code_hash)
    let mut cursor = &encoded[32..];
    let storage_deposit = u128::decode(&mut cursor)
        .context("Failed to decode storage_deposit from ContractInfo")?;

    Ok(ContractInfo {
        code_hash,
        storage_deposit,
    })
}

/// Parse contract address to bytes
fn parse_address(address: &str) -> Result<Vec<u8>> {
    // Remove "0x" prefix if present
    let address = address.strip_prefix("0x").unwrap_or(address);

    // Try hex decoding first
    if let Ok(bytes) = hex::decode(address) {
        if bytes.len() == 32 {
            return Ok(bytes);
        }
    }

    // Try SS58 decoding (Substrate addresses)
    use subxt::utils::AccountId32;
    use std::str::FromStr;

    let account = AccountId32::from_str(address)
        .context("Invalid contract address format (expected hex or SS58)")?;

    Ok(account.0.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_address() {
        let hex_addr = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let result = parse_address(hex_addr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }

    #[test]
    fn test_parse_address_without_prefix() {
        let hex_addr = "1234567890123456789012345678901234567890123456789012345678901234";
        let result = parse_address(hex_addr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }
}

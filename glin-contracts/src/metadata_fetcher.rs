// Multi-strategy metadata fetching for ink! contracts

use anyhow::{Context, Result};
use ink_metadata::InkProject;
use std::path::Path;

use glin_client::GlinClient;

/// Options for fetching metadata
pub struct MetadataFetchOptions {
    pub local_path: Option<String>,
    pub explorer_url: Option<String>,
    pub cache_dir: Option<String>,
}

/// Fetch contract metadata using cascading fallback strategy
///
/// Strategy priority:
/// 1. Local file (if provided via --metadata flag)
/// 2. Local cache (~/.glin-forge/cache/{address}.json)
/// 3. Get code_hash from chain ContractInfoOf storage
/// 4. Fetch from explorer API using code_hash
/// 5. Fail with helpful error message
pub async fn fetch_contract_metadata(
    client: &GlinClient,
    contract_address: &str,
    options: MetadataFetchOptions,
) -> Result<InkProject> {
    // Strategy 1: Load from local file if provided
    if let Some(path) = options.local_path {
        return load_metadata_from_file(&path);
    }

    // Strategy 2: Try loading from cache
    if let Some(cache_dir) = &options.cache_dir {
        if let Ok(metadata) = try_load_from_cache(cache_dir, contract_address) {
            return Ok(metadata);
        }
    }

    // Strategy 3: Get code hash from blockchain
    let code_hash_hex = match crate::chain_info::get_contract_info(client, contract_address).await {
        Ok(info) => Some(format!("0x{}", hex::encode(info.code_hash))),
        Err(_e) => None,
    };

    // Strategy 4: Fetch from explorer API
    if let Some(explorer_url) = options.explorer_url {
        match fetch_from_explorer(&explorer_url, contract_address, code_hash_hex.as_deref()).await {
            Ok(metadata) => {
                // Cache it for future use
                if let Some(cache_dir) = &options.cache_dir {
                    let _ = save_to_cache(cache_dir, contract_address, &metadata);
                }

                return Ok(metadata);
            }
            Err(_e) => {
                // Continue to error
            }
        }
    }

    // All strategies failed
    Err(anyhow::anyhow!(
        r#"Could not fetch metadata for contract {}

Metadata is not stored on-chain. Please provide it using one of these methods:

1. Specify metadata file via local_path option
2. Use an explorer with verification via explorer_url option
3. Place metadata in cache directory: ~/.glin-forge/cache/{}.json

For more info, see: https://use.ink/basics/metadata
"#,
        contract_address,
        contract_address
    ))
}

/// Load metadata from local file (.json or .contract bundle)
fn load_metadata_from_file(path: &str) -> Result<InkProject> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read metadata file: {}", path))?;

    // Check if it's a .contract bundle file
    if path.ends_with(".contract") {
        let bundle: serde_json::Value =
            serde_json::from_str(&content).context("Invalid .contract bundle format")?;

        // Extract spec from bundle
        let metadata: InkProject = serde_json::from_value(bundle["spec"].clone())
            .context("Invalid metadata in .contract bundle")?;

        Ok(metadata)
    } else {
        // Pure metadata JSON
        let metadata: InkProject =
            serde_json::from_str(&content).context("Invalid metadata JSON format")?;

        Ok(metadata)
    }
}

/// Fetch metadata from explorer API
async fn fetch_from_explorer(
    explorer_url: &str,
    contract_address: &str,
    code_hash: Option<&str>,
) -> Result<InkProject> {
    // Try common explorer API endpoints with both contract address and code hash
    let mut endpoints = vec![
        // Try contract address first
        format!(
            "{}/api/contract/{}/metadata",
            explorer_url, contract_address
        ),
        format!("{}/api/contracts/{}/abi", explorer_url, contract_address),
        format!(
            "{}/api/v1/contracts/{}/metadata",
            explorer_url, contract_address
        ),
    ];

    // If we have code hash, also try those endpoints
    if let Some(hash) = code_hash {
        endpoints.extend(vec![
            format!("{}/api/contract/{}/metadata", explorer_url, hash),
            format!("{}/api/contracts/metadata/{}", explorer_url, hash),
            format!("{}/api/v1/code/{}/abi", explorer_url, hash),
        ]);
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    for url in endpoints {
        match client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                // Try to parse as InkProject directly
                if let Ok(metadata) = response.json::<InkProject>().await {
                    return Ok(metadata);
                }
            }
            Ok(response) => {
                // Log non-success status
                eprintln!(
                    "    Endpoint {} returned status: {}",
                    url,
                    response.status()
                );
            }
            Err(e) => {
                // Log connection errors
                eprintln!("    Failed to connect to {}: {}", url, e);
            }
        }
    }

    Err(anyhow::anyhow!(
        "No explorer endpoint returned valid metadata for contract {}",
        contract_address
    ))
}

/// Try to load metadata from local cache
fn try_load_from_cache(cache_dir: &str, contract_address: &str) -> Result<InkProject> {
    let cache_path = Path::new(cache_dir).join(format!("{}.json", contract_address));

    if !cache_path.exists() {
        anyhow::bail!("No cache found");
    }

    load_metadata_from_file(cache_path.to_str().unwrap())
}

/// Save metadata to local cache
fn save_to_cache(cache_dir: &str, contract_address: &str, metadata: &InkProject) -> Result<()> {
    std::fs::create_dir_all(cache_dir)?;

    let cache_path = Path::new(cache_dir).join(format!("{}.json", contract_address));

    let json = serde_json::to_string_pretty(metadata)?;
    std::fs::write(cache_path, json)?;

    Ok(())
}

/// Get default cache directory
pub fn get_default_cache_dir() -> Result<String> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

    let cache_dir = home.join(".glin-forge").join("cache");

    Ok(cache_dir.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_metadata_from_json() {
        // This test would require a sample metadata file
        // Skipping for now
    }

    #[test]
    fn test_get_default_cache_dir() {
        let cache_dir = get_default_cache_dir();
        assert!(cache_dir.is_ok());
        assert!(cache_dir.unwrap().contains(".glin-forge"));
    }
}

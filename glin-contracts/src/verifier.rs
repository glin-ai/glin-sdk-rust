//! Contract verification utilities
//!
//! Provides tools to verify smart contracts by compiling source code and
//! comparing the resulting WASM hash with the deployed code hash.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use sp_core_hashing::blake2_256;

/// Contract verifier
///
/// Compiles contract source code and verifies that the resulting WASM hash
/// matches the deployed contract's code hash.
///
/// # Example
///
/// ```rust,no_run
/// use glin_contracts::ContractVerifier;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let verifier = ContractVerifier::new("/tmp/verification")?;
///
///     let result = verifier.verify(
///         source_code,
///         cargo_toml,
///         &deployed_code_hash
///     ).await?;
///
///     match result {
///         VerificationResult::Verified { code_hash, .. } => {
///             println!("✅ Contract verified! Code hash: {}", code_hash);
///         }
///         _ => println!("❌ Verification failed"),
///     }
///     Ok(())
/// }
/// ```
pub struct ContractVerifier {
    workspace_dir: PathBuf,
}

impl ContractVerifier {
    /// Create a new contract verifier with a workspace directory
    pub fn new<P: AsRef<Path>>(workspace_dir: P) -> Result<Self> {
        let workspace_dir = workspace_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&workspace_dir)?;

        Ok(Self { workspace_dir })
    }

    /// Verify a contract by compiling source and comparing hashes
    pub async fn verify(
        &self,
        source_code: &str,
        cargo_toml: &str,
        deployed_code_hash: &[u8; 32],
    ) -> Result<VerificationResult> {
        // Create temporary workspace
        let temp_dir = tempfile::tempdir_in(&self.workspace_dir)?;

        // Write source files
        let src_dir = temp_dir.path().join("src");
        std::fs::create_dir_all(&src_dir)?;
        std::fs::write(src_dir.join("lib.rs"), source_code)?;
        std::fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)?;

        // Compile contract
        let output = Command::new("cargo")
            .arg("contract")
            .arg("build")
            .arg("--release")
            .current_dir(temp_dir.path())
            .output()?;

        if !output.status.success() {
            return Ok(VerificationResult::CompilationFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        // Extract code hash from bundle
        let bundle_path = temp_dir
            .path()
            .join("target")
            .join("ink")
            .join("lib.contract");

        if !bundle_path.exists() {
            return Ok(VerificationResult::CompilationFailed(
                "Contract bundle not found".to_string(),
            ));
        }

        let bundle: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(bundle_path)?)?;

        let compiled_hash = self.extract_code_hash(&bundle)?;

        // Compare hashes
        if &compiled_hash == deployed_code_hash {
            Ok(VerificationResult::Verified {
                code_hash: hex::encode(compiled_hash),
                metadata: bundle.get("spec").cloned(),
            })
        } else {
            Ok(VerificationResult::HashMismatch {
                expected: hex::encode(deployed_code_hash),
                actual: hex::encode(compiled_hash),
            })
        }
    }

    /// Extract code hash from contract bundle
    fn extract_code_hash(&self, bundle: &serde_json::Value) -> Result<[u8; 32]> {
        // Extract WASM from bundle
        let wasm_hex = bundle
            .get("source")
            .and_then(|s| s.get("wasm"))
            .and_then(|w| w.as_str())
            .ok_or_else(|| anyhow!("No WASM in bundle"))?
            .trim_start_matches("0x");

        let wasm_bytes = hex::decode(wasm_hex)?;

        // Compute Blake2_256 hash
        let hash = blake2_256(&wasm_bytes);

        Ok(hash)
    }
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum VerificationResult {
    /// Contract successfully verified
    Verified {
        code_hash: String,
        metadata: Option<serde_json::Value>,
    },
    /// Hash mismatch between compiled and deployed code
    HashMismatch { expected: String, actual: String },
    /// Compilation failed
    CompilationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_creation() {
        let verifier = ContractVerifier::new("/tmp/test-verification").unwrap();
        assert!(verifier.workspace_dir.exists());
    }
}

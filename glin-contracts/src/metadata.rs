// Contract metadata parsing utilities using ink_metadata

use anyhow::{Context, Result};
use ink_metadata::{
    InkProject, Selector,
};
use scale_info::form::PortableForm;
use serde_json::Value as JsonValue;

// Type aliases for PortableForm
type ConstructorSpec = ink_metadata::ConstructorSpec<PortableForm>;
type MessageSpec = ink_metadata::MessageSpec<PortableForm>;
type TypeSpec = ink_metadata::TypeSpec<PortableForm>;

/// Parse ink! contract metadata from JSON
pub fn parse_metadata(metadata_json: &str) -> Result<InkProject> {
    let metadata: InkProject = serde_json::from_str(metadata_json)
        .context("Failed to parse ink! contract metadata JSON")?;
    Ok(metadata)
}

/// Get constructor specification by name
pub fn get_constructor_spec<'a>(
    metadata: &'a InkProject,
    name: &str,
) -> Result<&'a ConstructorSpec> {
    metadata
        .spec()
        .constructors()
        .iter()
        .find(|c| c.label() == name)
        .ok_or_else(|| anyhow::anyhow!("Constructor '{}' not found in metadata", name))
}

/// Get default constructor (usually "new")
pub fn get_default_constructor<'a>(metadata: &'a InkProject) -> Result<&'a ConstructorSpec> {
    // Try "new" first
    if let Some(ctor) = metadata
        .spec()
        .constructors()
        .iter()
        .find(|c| c.label() == "new")
    {
        return Ok(ctor);
    }

    // Fall back to first constructor
    metadata
        .spec()
        .constructors()
        .first()
        .ok_or_else(|| anyhow::anyhow!("No constructors found in metadata"))
}

/// Get message specification by name
pub fn get_message_spec<'a>(metadata: &'a InkProject, name: &str) -> Result<&'a MessageSpec> {
    metadata
        .spec()
        .messages()
        .iter()
        .find(|m| m.label() == name)
        .ok_or_else(|| anyhow::anyhow!("Message '{}' not found in metadata", name))
}

/// Get contract name from metadata
pub fn get_contract_name(_metadata: &InkProject) -> String {
    // Note: Contract name not directly accessible in PortableForm
    // Would need to traverse type registry or use different metadata source
    String::from("unknown")
}

/// Get contract version from metadata
pub fn get_contract_version(metadata: &InkProject) -> String {
    metadata.version().to_string()
}

/// List all constructor names
pub fn list_constructors(metadata: &InkProject) -> Vec<String> {
    metadata
        .spec()
        .constructors()
        .iter()
        .map(|c| c.label().to_string())
        .collect()
}

/// List all message names
pub fn list_messages(metadata: &InkProject) -> Vec<String> {
    metadata
        .spec()
        .messages()
        .iter()
        .map(|m| m.label().to_string())
        .collect()
}

/// Get message selector (first 4 bytes of Blake2_256 hash of label)
pub fn get_message_selector(message: &MessageSpec) -> &Selector {
    message.selector()
}

/// Get constructor selector
pub fn get_constructor_selector(constructor: &ConstructorSpec) -> &Selector {
    constructor.selector()
}

/// Check if a message is mutable (changes state)
pub fn is_message_mutable(message: &MessageSpec) -> bool {
    message.mutates()
}

/// Get message return type
pub fn get_message_return_type(message: &MessageSpec) -> &TypeSpec {
    message.return_type().ret_type()
}

/// Parse metadata from JSON value (for backward compatibility)
pub fn parse_metadata_from_json(json: &JsonValue) -> Result<InkProject> {
    let json_str = serde_json::to_string(json)?;
    parse_metadata(&json_str)
}

/// Validate metadata structure
pub fn validate_metadata(metadata: &InkProject) -> Result<()> {
    // Check that we have at least one constructor
    if metadata.spec().constructors().is_empty() {
        anyhow::bail!("Metadata must have at least one constructor");
    }

    // Check that we have at least one message
    if metadata.spec().messages().is_empty() {
        anyhow::bail!("Metadata must have at least one message");
    }

    // Validate type registry is not empty
    if metadata.registry().types.is_empty() {
        anyhow::bail!("Metadata type registry is empty");
    }

    Ok(())
}

/// Get type definition from registry by type ID
pub fn get_type_from_registry<'a>(
    metadata: &'a InkProject,
    type_id: u32,
) -> Option<&'a scale_info::Type<scale_info::form::PortableForm>> {
    metadata.registry().resolve(type_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata_invalid() {
        // Empty JSON should fail
        assert!(parse_metadata("{}").is_err());
    }

    #[test]
    fn test_parse_metadata_from_json() {
        let json = serde_json::json!({});
        assert!(parse_metadata_from_json(&json).is_err());
    }
}

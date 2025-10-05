// SCALE encoding/decoding for contract arguments

use anyhow::{Context, Result};
use ink_metadata::InkProject;
use scale::{Decode, Encode};
use scale_info::{form::PortableForm, TypeDef, TypeDefPrimitive};
use serde_json::Value as JsonValue;
use subxt::utils::AccountId32;

// Type aliases for PortableForm specs
type MessageParamSpec = ink_metadata::MessageParamSpec<PortableForm>;
type TypeSpec = ink_metadata::TypeSpec<PortableForm>;

/// Encode contract arguments based on their types from metadata
pub fn encode_args(
    args: &[String],
    param_specs: &[MessageParamSpec],
    metadata: &InkProject,
) -> Result<Vec<u8>> {
    if args.len() != param_specs.len() {
        anyhow::bail!(
            "Argument count mismatch: expected {}, got {}",
            param_specs.len(),
            args.len()
        );
    }

    let mut encoded = Vec::new();

    for (arg_str, param) in args.iter().zip(param_specs.iter()) {
        // Get type ID from param
        let type_id = param.ty().ty().id;
        let arg_bytes = encode_value_by_id(arg_str, type_id, metadata)?;
        encoded.extend_from_slice(&arg_bytes);
    }

    Ok(encoded)
}

/// Encode a single value based on its type ID
fn encode_value_by_id(value_str: &str, type_id: u32, metadata: &InkProject) -> Result<Vec<u8>> {
    let registry = metadata.registry();

    let ty = registry
        .resolve(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type {} not found in registry", type_id))?;

    // Access type_def field directly (not deprecated)
    match &ty.type_def {
        TypeDef::Primitive(prim) => encode_primitive(value_str, prim),
        TypeDef::Composite(_) => encode_composite(value_str, type_id, metadata),
        TypeDef::Variant(_) => encode_variant(value_str, type_id, metadata),
        TypeDef::Sequence(_) => encode_sequence(value_str, type_id, metadata),
        TypeDef::Array(_) => encode_array(value_str, type_id, metadata),
        TypeDef::Tuple(_) => encode_tuple(value_str, type_id, metadata),
        TypeDef::Compact(_) => {
            // Compact encoding - parse as number and use compact encoding
            let num: u128 = value_str
                .parse()
                .context("Failed to parse compact value as number")?;
            Ok(scale::Compact(num).encode())
        }
        TypeDef::BitSequence(_) => {
            anyhow::bail!("BitSequence encoding not yet supported")
        }
    }
}

/// Encode primitive types
fn encode_primitive(value_str: &str, prim: &TypeDefPrimitive) -> Result<Vec<u8>> {
    match prim {
        TypeDefPrimitive::Bool => {
            let val: bool = value_str.parse().context("Failed to parse boolean")?;
            Ok(val.encode())
        }
        TypeDefPrimitive::Char => {
            let val: char = value_str
                .chars()
                .next()
                .ok_or_else(|| anyhow::anyhow!("Empty char value"))?;
            Ok((val as u32).encode())
        }
        TypeDefPrimitive::Str => Ok(value_str.to_string().encode()),
        TypeDefPrimitive::U8 => {
            let val: u8 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::U16 => {
            let val: u16 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::U32 => {
            let val: u32 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::U64 => {
            let val: u64 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::U128 => {
            let val: u128 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::U256 => {
            anyhow::bail!("U256 encoding not yet supported")
        }
        TypeDefPrimitive::I8 => {
            let val: i8 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::I16 => {
            let val: i16 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::I32 => {
            let val: i32 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::I64 => {
            let val: i64 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::I128 => {
            let val: i128 = value_str.parse()?;
            Ok(val.encode())
        }
        TypeDefPrimitive::I256 => {
            anyhow::bail!("I256 encoding not yet supported")
        }
    }
}

/// Encode composite types (structs)
fn encode_composite(value_str: &str, type_id: u32, metadata: &InkProject) -> Result<Vec<u8>> {
    let registry = metadata.registry();
    let ty = registry
        .resolve(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type {} not found", type_id))?;

    // Check if this is an AccountId32 (special case)
    if ty.path.segments.last().map(|s| s.as_str()) == Some("AccountId32") {
        return encode_account_id(value_str);
    }

    // Try to parse as JSON for complex types
    let json: JsonValue = serde_json::from_str(value_str)
        .context("Failed to parse composite value as JSON")?;

    if let TypeDef::Composite(composite) = &ty.type_def {
        let mut encoded = Vec::new();

        for field in &composite.fields {
            let field_name = field
                .name
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Unnamed field in composite"))?;

            let field_value = json
                .get(field_name)
                .ok_or_else(|| anyhow::anyhow!("Missing field: {}", field_name))?
                .to_string();

            let field_type_id = field.ty.id;
            let field_bytes = encode_value_by_id(&field_value, field_type_id, metadata)?;
            encoded.extend_from_slice(&field_bytes);
        }

        Ok(encoded)
    } else {
        anyhow::bail!("Expected composite type")
    }
}

/// Encode AccountId32
fn encode_account_id(value_str: &str) -> Result<Vec<u8>> {
    use std::str::FromStr;

    // Try parsing as SS58 address
    if let Ok(account_id) = AccountId32::from_str(value_str) {
        return Ok(account_id.0.encode());
    }

    // Try parsing as hex
    if value_str.starts_with("0x") {
        let bytes = hex::decode(value_str.trim_start_matches("0x"))
            .context("Invalid hex address")?;
        if bytes.len() == 32 {
            return Ok(bytes);
        }
    }

    anyhow::bail!("Invalid AccountId32 format: {}", value_str)
}

/// Encode variant types (enums, Option, Result)
fn encode_variant(value_str: &str, type_id: u32, metadata: &InkProject) -> Result<Vec<u8>> {
    let registry = metadata.registry();
    let ty = registry
        .resolve(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type {} not found", type_id))?;

    // Check for common types: Option, Result
    let type_name = ty.path.segments.last().map(|s| s.as_str());

    match type_name {
        Some("Option") => encode_option(value_str, type_id, metadata),
        Some("Result") => encode_result(value_str, type_id, metadata),
        _ => {
            // Generic enum
            let json: JsonValue =
                serde_json::from_str(value_str).context("Failed to parse variant as JSON")?;

            if let Some(variant_name) = json.get("variant").and_then(|v| v.as_str()) {
                if let TypeDef::Variant(variant_def) = &ty.type_def {
                    let variant = variant_def
                        .variants
                        .iter()
                        .find(|v| v.name == variant_name)
                        .ok_or_else(|| anyhow::anyhow!("Variant {} not found", variant_name))?;

                    let mut encoded = Vec::new();
                    encoded.push(variant.index);

                    // Encode variant fields if any
                    if let Some(fields_json) = json.get("fields") {
                        for (i, field) in variant.fields.iter().enumerate() {
                            let field_value = fields_json
                                .get(i)
                                .ok_or_else(|| anyhow::anyhow!("Missing variant field {}", i))?
                                .to_string();

                            let field_type_id = field.ty.id;
                            let field_bytes = encode_value_by_id(&field_value, field_type_id, metadata)?;
                            encoded.extend_from_slice(&field_bytes);
                        }
                    }

                    return Ok(encoded);
                }
            }

            anyhow::bail!("Invalid variant encoding")
        }
    }
}

/// Encode Option type
fn encode_option(value_str: &str, _type_id: u32, _metadata: &InkProject) -> Result<Vec<u8>> {
    if value_str == "null" || value_str.is_empty() {
        // None variant (index 0)
        Ok(vec![0u8])
    } else {
        // Some variant (index 1) + encoded value
        let mut encoded = vec![1u8];
        encoded.extend_from_slice(&value_str.to_string().encode());
        Ok(encoded)
    }
}

/// Encode Result type
fn encode_result(value_str: &str, _type_id: u32, _metadata: &InkProject) -> Result<Vec<u8>> {
    let json: JsonValue =
        serde_json::from_str(value_str).context("Failed to parse Result as JSON")?;

    if json.get("Ok").is_some() {
        // Ok variant (index 0)
        let mut encoded = vec![0u8];
        let ok_value = json.get("Ok").unwrap().to_string();
        encoded.extend_from_slice(&ok_value.encode());
        Ok(encoded)
    } else if json.get("Err").is_some() {
        // Err variant (index 1)
        let mut encoded = vec![1u8];
        let err_value = json.get("Err").unwrap().to_string();
        encoded.extend_from_slice(&err_value.encode());
        Ok(encoded)
    } else {
        anyhow::bail!("Invalid Result format")
    }
}

/// Encode sequence (Vec)
fn encode_sequence(value_str: &str, type_id: u32, metadata: &InkProject) -> Result<Vec<u8>> {
    let json: JsonValue =
        serde_json::from_str(value_str).context("Failed to parse sequence as JSON array")?;

    let array = json
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Expected JSON array"))?;

    let registry = metadata.registry();
    let ty = registry
        .resolve(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type {} not found", type_id))?;

    if let TypeDef::Sequence(seq) = &ty.type_def {
        let element_type_id = seq.type_param.id;

        // Encode length as compact
        let mut encoded = scale::Compact(array.len() as u32).encode();

        // Encode each element
        for element in array {
            let element_str = element.to_string();
            let element_bytes = encode_value_by_id(&element_str, element_type_id, metadata)?;
            encoded.extend_from_slice(&element_bytes);
        }

        Ok(encoded)
    } else {
        anyhow::bail!("Expected sequence type")
    }
}

/// Encode array
fn encode_array(value_str: &str, type_id: u32, metadata: &InkProject) -> Result<Vec<u8>> {
    let json: JsonValue =
        serde_json::from_str(value_str).context("Failed to parse array as JSON")?;

    let array = json
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Expected JSON array"))?;

    let registry = metadata.registry();
    let ty = registry
        .resolve(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type {} not found", type_id))?;

    if let TypeDef::Array(arr_def) = &ty.type_def {
        let element_type_id = arr_def.type_param.id;

        if array.len() != arr_def.len as usize {
            anyhow::bail!(
                "Array length mismatch: expected {}, got {}",
                arr_def.len,
                array.len()
            );
        }

        let mut encoded = Vec::new();

        for element in array {
            let element_str = element.to_string();
            let element_bytes = encode_value_by_id(&element_str, element_type_id, metadata)?;
            encoded.extend_from_slice(&element_bytes);
        }

        Ok(encoded)
    } else {
        anyhow::bail!("Expected array type")
    }
}

/// Encode tuple
fn encode_tuple(value_str: &str, type_id: u32, metadata: &InkProject) -> Result<Vec<u8>> {
    let json: JsonValue =
        serde_json::from_str(value_str).context("Failed to parse tuple as JSON array")?;

    let array = json
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Expected JSON array for tuple"))?;

    let registry = metadata.registry();
    let ty = registry
        .resolve(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type {} not found", type_id))?;

    if let TypeDef::Tuple(tuple_def) = &ty.type_def {
        if array.len() != tuple_def.fields.len() {
            anyhow::bail!("Tuple length mismatch");
        }

        let mut encoded = Vec::new();

        for (element, field_ty) in array.iter().zip(&tuple_def.fields) {
            let element_str = element.to_string();
            let element_bytes = encode_value_by_id(&element_str, field_ty.id, metadata)?;
            encoded.extend_from_slice(&element_bytes);
        }

        Ok(encoded)
    } else {
        anyhow::bail!("Expected tuple type")
    }
}

/// Decode query result based on return type
pub fn decode_result(
    bytes: &[u8],
    type_spec: Option<&TypeSpec>,
    metadata: &InkProject,
) -> Result<JsonValue> {
    if let Some(spec) = type_spec {
        let type_id = spec.ty().id;
        decode_value_by_id(bytes, type_id, metadata)
    } else {
        // No return type (void)
        Ok(JsonValue::Null)
    }
}

/// Decode a value based on its type ID
fn decode_value_by_id(bytes: &[u8], type_id: u32, metadata: &InkProject) -> Result<JsonValue> {
    let registry = metadata.registry();

    let ty = registry
        .resolve(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type {} not found in registry", type_id))?;

    match &ty.type_def {
        TypeDef::Primitive(prim) => decode_primitive(bytes, prim),
        TypeDef::Composite(_) => {
            // For simplicity, return hex for complex types
            Ok(JsonValue::String(format!("0x{}", hex::encode(bytes))))
        }
        _ => {
            // For other types, return as hex
            Ok(JsonValue::String(format!("0x{}", hex::encode(bytes))))
        }
    }
}

/// Decode primitive types
fn decode_primitive(bytes: &[u8], prim: &TypeDefPrimitive) -> Result<JsonValue> {
    match prim {
        TypeDefPrimitive::Bool => {
            let val = bool::decode(&mut &bytes[..])?;
            Ok(JsonValue::Bool(val))
        }
        TypeDefPrimitive::U8 => {
            let val = u8::decode(&mut &bytes[..])?;
            Ok(JsonValue::Number(val.into()))
        }
        TypeDefPrimitive::U16 => {
            let val = u16::decode(&mut &bytes[..])?;
            Ok(JsonValue::Number(val.into()))
        }
        TypeDefPrimitive::U32 => {
            let val = u32::decode(&mut &bytes[..])?;
            Ok(JsonValue::Number(val.into()))
        }
        TypeDefPrimitive::U64 => {
            let val = u64::decode(&mut &bytes[..])?;
            Ok(JsonValue::Number(val.into()))
        }
        TypeDefPrimitive::U128 => {
            let val = u128::decode(&mut &bytes[..])?;
            Ok(JsonValue::String(val.to_string()))
        }
        TypeDefPrimitive::I8 => {
            let val = i8::decode(&mut &bytes[..])?;
            Ok(JsonValue::Number(val.into()))
        }
        TypeDefPrimitive::I16 => {
            let val = i16::decode(&mut &bytes[..])?;
            Ok(JsonValue::Number(val.into()))
        }
        TypeDefPrimitive::I32 => {
            let val = i32::decode(&mut &bytes[..])?;
            Ok(JsonValue::Number(val.into()))
        }
        TypeDefPrimitive::I64 => {
            let val = i64::decode(&mut &bytes[..])?;
            Ok(JsonValue::Number(val.into()))
        }
        TypeDefPrimitive::I128 => {
            let val = i128::decode(&mut &bytes[..])?;
            Ok(JsonValue::String(val.to_string()))
        }
        TypeDefPrimitive::Str => {
            let val = String::decode(&mut &bytes[..])?;
            Ok(JsonValue::String(val))
        }
        _ => Ok(JsonValue::String(format!("0x{}", hex::encode(bytes)))),
    }
}

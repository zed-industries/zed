//! Extracting schema ID.
use serde_json::Value;

pub(crate) fn dollar_id(contents: &Value) -> Option<&str> {
    contents
        .as_object()
        .and_then(|obj| obj.get("$id"))
        .and_then(|id| id.as_str())
}

pub(crate) fn legacy_dollar_id(contents: &Value) -> Option<&str> {
    let object = contents.as_object()?;
    if object.contains_key("$ref") {
        return None;
    }
    if let Some(id) = contents.get("$id").and_then(|id| id.as_str()) {
        if !id.starts_with('#') {
            return Some(id);
        }
    }
    None
}

pub(crate) fn legacy_id(contents: &Value) -> Option<&str> {
    let object = contents.as_object()?;
    if object.contains_key("$ref") {
        return None;
    }
    if let Some(id) = object.get("id").and_then(|id| id.as_str()) {
        if !id.starts_with('#') {
            return Some(id);
        }
    }
    None
}

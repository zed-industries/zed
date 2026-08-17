#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use jsonschema::{self, Draft};
use serde_json::json;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn validates_simple_object_instances() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "age": { "type": "integer", "minimum": 0 }
        },
        "required": ["name"]
    });

    let validator = jsonschema::validator_for(&schema).expect("valid schema");

    assert!(validator.is_valid(&json!({"name": "Ferris", "age": 7})));
    assert!(!validator.is_valid(&json!({"name": 9, "age": 7})));
    assert!(!validator.is_valid(&json!({"age": 7})));
}

#[wasm_bindgen_test]
fn validates_formats_with_options() {
    let options = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true);
    let validator = options
        .build(&json!({"type": "string", "format": "email"}))
        .expect("schema builds");

    assert!(validator.is_valid(&json!("demo@example.com")));
    assert!(!validator.is_valid(&json!("definitely not an email")));
}

#[wasm_bindgen_test]
fn validates_uuid_format() {
    let options = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true);
    let validator = options
        .build(&json!({"type": "string", "format": "uuid"}))
        .expect("schema builds");

    assert!(validator.is_valid(&json!("67e55044-10b1-426f-9247-bb680e5fe0c8")));
    assert!(!validator.is_valid(&json!("67e5504410b1426f9247bb680e5fe0c8"))); // missing hyphens
}

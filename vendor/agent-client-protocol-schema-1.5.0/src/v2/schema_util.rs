use schemars::Schema;
use serde_json::json;

pub(crate) fn reject_known_string_discriminators(
    schema: &mut Schema,
    property_name: &str,
    known_values: &[&str],
) {
    let known_value_schemas: Vec<_> = known_values
        .iter()
        .map(|value| {
            json!({
                "properties": {
                    property_name: {
                        "const": value,
                        "type": "string"
                    }
                },
                "required": [property_name],
                "type": "object"
            })
        })
        .collect();

    schema.insert(
        "not".into(),
        json!({
            "anyOf": known_value_schemas
        }),
    );
}

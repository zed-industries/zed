use agent_client_protocol::schema::{v1, v2};
use anyhow::{Result, bail};
use std::collections::BTreeMap;

pub fn request_from_v1(
    request: v1::CreateElicitationRequest,
) -> Result<v2::CreateElicitationRequest> {
    let mode = match request.mode {
        v1::ElicitationMode::Form(mode) => v2::ElicitationMode::Form(v2::ElicitationFormMode::new(
            scope_from_v1(mode.scope)?,
            schema_from_v1(mode.requested_schema)?,
        )),
        v1::ElicitationMode::Url(mode) => v2::ElicitationMode::Url(v2::ElicitationUrlMode::new(
            scope_from_v1(mode.scope)?,
            v2::ElicitationId::new(mode.elicitation_id.0),
            mode.url,
        )),
        v1::ElicitationMode::Other(mode) => {
            let mut converted = v2::OtherElicitationMode::new(
                mode.mode,
                scope_from_v1(mode.scope)?,
                BTreeMap::new(),
            );
            converted.fields = mode.fields;
            v2::ElicitationMode::Other(converted)
        }
        _ => bail!("unsupported v1 elicitation mode"),
    };
    Ok(v2::CreateElicitationRequest::new(mode, request.message).meta(request.meta))
}

pub fn response_to_v1(
    response: v2::CreateElicitationResponse,
) -> Result<v1::CreateElicitationResponse> {
    let action = match response.action {
        v2::ElicitationAction::Accept(action) => {
            let content = action
                .content
                .map(|content| {
                    content
                        .into_iter()
                        .map(|(name, value)| Ok((name, content_value_to_v1(value)?)))
                        .collect::<Result<BTreeMap<_, _>>>()
                })
                .transpose()?;
            v1::ElicitationAction::Accept(v1::ElicitationAcceptAction::new().content(content))
        }
        v2::ElicitationAction::Decline => v1::ElicitationAction::Decline,
        v2::ElicitationAction::Cancel => v1::ElicitationAction::Cancel,
        v2::ElicitationAction::Other(action) => {
            let mut converted = v1::OtherElicitationAction::new(action.action, BTreeMap::new());
            converted.fields = action.fields;
            v1::ElicitationAction::Other(converted)
        }
        _ => bail!("unsupported v2 elicitation action"),
    };
    Ok(v1::CreateElicitationResponse::new(action).meta(response.meta))
}

pub fn error_to_v1(error: v2::Error) -> v1::Error {
    v1::Error::new(error.code.into(), error.message).data(error.data)
}

fn scope_from_v1(scope: v1::ElicitationScope) -> Result<v2::ElicitationScope> {
    Ok(match scope {
        v1::ElicitationScope::Session(scope) => {
            v2::ElicitationSessionScope::new(v2::SessionId::new(scope.session_id.0))
                .tool_call_id(scope.tool_call_id.map(|id| v2::ToolCallId::new(id.0)))
                .into()
        }
        v1::ElicitationScope::Request(scope) => {
            v2::ElicitationRequestScope::new(scope.request_id).into()
        }
        _ => bail!("unsupported v1 elicitation scope"),
    })
}

fn schema_from_v1(schema: v1::ElicitationSchema) -> Result<v2::ElicitationSchema> {
    let mut converted = v2::ElicitationSchema::new();
    converted.type_ = match schema.type_ {
        v1::ElicitationSchemaType::Object => v2::ElicitationSchemaType::Object,
        _ => bail!("unsupported v1 elicitation schema type"),
    };
    converted.title = schema.title;
    converted.properties = schema
        .properties
        .into_iter()
        .map(|(name, property)| Ok((name, property_from_v1(property)?)))
        .collect::<Result<_>>()?;
    converted.required = schema.required;
    converted.description = schema.description;
    converted.meta = schema.meta;
    Ok(converted)
}

fn property_from_v1(
    property: v1::ElicitationPropertySchema,
) -> Result<v2::ElicitationPropertySchema> {
    Ok(match property {
        v1::ElicitationPropertySchema::String(property) => {
            let mut converted = v2::StringPropertySchema::new();
            converted.title = property.title;
            converted.description = property.description;
            converted.min_length = property.min_length;
            converted.max_length = property.max_length;
            converted.pattern = property.pattern;
            converted.format = property.format.map(string_format_from_v1).transpose()?;
            converted.default = property.default;
            converted.enum_values = property.enum_values;
            converted.one_of = property
                .one_of
                .map(|options| options.into_iter().map(enum_option_from_v1).collect());
            converted.meta = property.meta;
            v2::ElicitationPropertySchema::String(converted)
        }
        v1::ElicitationPropertySchema::Number(property) => {
            let mut converted = v2::NumberPropertySchema::new();
            converted.title = property.title;
            converted.description = property.description;
            converted.minimum = property.minimum;
            converted.maximum = property.maximum;
            converted.default = property.default;
            converted.meta = property.meta;
            v2::ElicitationPropertySchema::Number(converted)
        }
        v1::ElicitationPropertySchema::Integer(property) => {
            let mut converted = v2::IntegerPropertySchema::new();
            converted.title = property.title;
            converted.description = property.description;
            converted.minimum = property.minimum;
            converted.maximum = property.maximum;
            converted.default = property.default;
            converted.meta = property.meta;
            v2::ElicitationPropertySchema::Integer(converted)
        }
        v1::ElicitationPropertySchema::Boolean(property) => {
            let mut converted = v2::BooleanPropertySchema::new();
            converted.title = property.title;
            converted.description = property.description;
            converted.default = property.default;
            converted.meta = property.meta;
            v2::ElicitationPropertySchema::Boolean(converted)
        }
        v1::ElicitationPropertySchema::Array(property) => {
            let mut converted = v2::MultiSelectPropertySchema::new(Vec::new());
            converted.title = property.title;
            converted.description = property.description;
            converted.min_items = property.min_items;
            converted.max_items = property.max_items;
            converted.items = items_from_v1(property.items)?;
            converted.default = property.default;
            converted.meta = property.meta;
            v2::ElicitationPropertySchema::Array(converted)
        }
        v1::ElicitationPropertySchema::Other(property) => {
            let mut converted =
                v2::OtherElicitationPropertySchema::new(property.type_, BTreeMap::new());
            converted.fields = property.fields;
            v2::ElicitationPropertySchema::Other(converted)
        }
        _ => bail!("unsupported v1 elicitation property schema"),
    })
}

fn string_format_from_v1(format: v1::StringFormat) -> Result<v2::StringFormat> {
    Ok(match format {
        v1::StringFormat::Email => v2::StringFormat::Email,
        v1::StringFormat::Uri => v2::StringFormat::Uri,
        v1::StringFormat::Date => v2::StringFormat::Date,
        v1::StringFormat::DateTime => v2::StringFormat::DateTime,
        _ => bail!("unsupported v1 elicitation string format"),
    })
}

fn enum_option_from_v1(option: v1::EnumOption) -> v2::EnumOption {
    v2::EnumOption::new(option.value, option.title)
        .description(option.description)
        .meta(option.meta)
}

fn items_from_v1(items: v1::MultiSelectItems) -> Result<v2::MultiSelectItems> {
    Ok(match items {
        v1::MultiSelectItems::String(items) => v2::MultiSelectItems::String(
            v2::StringMultiSelectItems::new(items.values).meta(items.meta),
        ),
        v1::MultiSelectItems::Titled(items) => v2::MultiSelectItems::Titled(
            v2::TitledMultiSelectItems::new(
                items.options.into_iter().map(enum_option_from_v1).collect(),
            )
            .meta(items.meta),
        ),
        v1::MultiSelectItems::Other(items) => {
            let mut converted = v2::OtherMultiSelectItems::new(items.type_, BTreeMap::new());
            converted.fields = items.fields;
            v2::MultiSelectItems::Other(converted)
        }
        _ => bail!("unsupported v1 elicitation multi-select items"),
    })
}

fn content_value_to_v1(value: v2::ElicitationContentValue) -> Result<v1::ElicitationContentValue> {
    Ok(match value {
        v2::ElicitationContentValue::String(value) => v1::ElicitationContentValue::String(value),
        v2::ElicitationContentValue::Integer(value) => v1::ElicitationContentValue::Integer(value),
        v2::ElicitationContentValue::Number(value) => v1::ElicitationContentValue::Number(value),
        v2::ElicitationContentValue::Boolean(value) => v1::ElicitationContentValue::Boolean(value),
        v2::ElicitationContentValue::StringArray(value) => {
            v1::ElicitationContentValue::StringArray(value)
        }
        _ => bail!("unsupported v2 elicitation content value"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn elicitation_request_adapter_preserves_schema_scopes_and_extensions() -> Result<()> {
        let fixture = json!({
            "mode": "form",
            "sessionId": "session-01",
            "toolCallId": "tool-01",
            "message": "Provide details",
            "_meta": {"request": {"opaque": [null, 1, "value"]}},
            "requestedSchema": {
                "type": "object",
                "title": "Details",
                "description": "Complete this form",
                "required": ["unknown", "name", "choices", "name"],
                "_meta": {"schema": {}},
                "properties": {
                    "name": {
                        "type": "string", "title": "Name", "description": "Display name",
                        "minLength": 1, "maxLength": 64, "pattern": "^[a-z]+$",
                        "format": "email", "default": "ada", "enum": ["ada", "grace"],
                        "oneOf": [
                            {"const": "grace", "title": "Grace", "description": "Second",
                             "_meta": {"option": [2, 1]}},
                            {"const": "ada", "title": "Ada", "_meta": {}}
                        ],
                        "_meta": {"property": {"nested": true}}
                    },
                    "uri": {"type": "string", "format": "uri"},
                    "date": {"type": "string", "format": "date"},
                    "time": {"type": "string", "format": "date-time"},
                    "rating": {
                        "type": "number", "title": "Rating", "description": "Score",
                        "minimum": -1.5, "maximum": 9.5, "default": 2.5, "_meta": {}
                    },
                    "count": {
                        "type": "integer", "title": "Count", "description": "Quantity",
                        "minimum": -4, "maximum": 20, "default": 3, "_meta": {"integer": null}
                    },
                    "enabled": {
                        "type": "boolean", "title": "Enabled", "description": "Toggle",
                        "default": false, "_meta": {"boolean": true}
                    },
                    "choices": {
                        "type": "array", "title": "Choices", "description": "Pick several",
                        "minItems": 1, "maxItems": 3, "default": ["b", "a"],
                        "items": {"type": "string", "enum": ["b", "a", "c"],
                                  "_meta": {"items": [3, 2, 1]}},
                        "_meta": {}
                    },
                    "titled": {
                        "type": "array",
                        "items": {"anyOf": [
                            {"const": "b", "title": "B", "description": "First",
                             "_meta": {"option": true}},
                            {"const": "a", "title": "A"}
                        ], "_meta": {}},
                        "default": []
                    },
                    "unknown_items": {
                        "type": "array", "minItems": 0, "maxItems": 4,
                        "items": {"type": "_token", "$schema": "https://example.com/items",
                                  "additionalProperties": false, "_meta": {"raw": null},
                                  "nested": {"anyOf": [{"const": "a", "title": "A"}]}}
                    },
                    "unknown": {
                        "type": "_location", "$schema": "https://example.com/location",
                        "required": ["longitude", "latitude"], "additionalProperties": true,
                        "nested": {"precision": [1, null, "city"]}, "_meta": {"raw": {}}
                    }
                }
            }
        });
        let request: v1::CreateElicitationRequest = serde_json::from_value(fixture.clone())?;
        let converted = request_from_v1(request)?;
        assert_eq!(serde_json::to_value(&converted)?, fixture);
        let v2::ElicitationMode::Form(mode) = converted.mode else {
            bail!("expected form mode");
        };
        assert!(matches!(
            mode.requested_schema.properties.get("unknown"),
            Some(v2::ElicitationPropertySchema::Other(_))
        ));

        for request_id in [json!(9_007_199_254_740_993_i64), json!("0001")] {
            let fixture = json!({
                "mode": "url", "requestId": request_id,
                "elicitationId": "elicitation-01", "url": "https://example.com/complete",
                "message": "Complete setup", "_meta": {}
            });
            let request: v1::CreateElicitationRequest = serde_json::from_value(fixture.clone())?;
            let request_id = match request.scope() {
                v1::ElicitationScope::Request(scope) => scope.request_id.clone(),
                _ => bail!("expected request scope"),
            };
            let converted = request_from_v1(request)?;
            let v2::ElicitationScope::Request(scope) = converted.scope() else {
                bail!("expected request scope");
            };
            assert_eq!(scope.request_id, request_id);
            assert_eq!(serde_json::to_value(converted)?, fixture);
        }

        for scope in [
            json!({"requestId": "0001"}),
            json!({"sessionId": "session-01", "toolCallId": "tool-01"}),
        ] {
            let mut fixture = json!({
                "mode": "_browser", "message": "Future request",
                "target": {"url": "https://example.com", "options": [null, true]},
                "requestedSchema": {"type": "_future", "additionalProperties": false},
                "_meta": {"raw": [1, 2]}
            });
            fixture
                .as_object_mut()
                .ok_or_else(|| anyhow::anyhow!("expected fixture object"))?
                .extend(
                    scope
                        .as_object()
                        .ok_or_else(|| anyhow::anyhow!("expected scope object"))?
                        .clone(),
                );
            let request: v1::CreateElicitationRequest = serde_json::from_value(fixture.clone())?;
            let converted = request_from_v1(request)?;
            assert!(matches!(converted.mode, v2::ElicitationMode::Other(_)));
            assert_eq!(serde_json::to_value(converted)?, fixture);
        }

        let fixture = json!({
            "mode": "form", "requestId": 0, "message": "",
            "requestedSchema": {"type": "object", "properties": {}, "required": []}
        });
        assert_eq!(
            serde_json::to_value(request_from_v1(serde_json::from_value(fixture.clone())?)?)?,
            fixture
        );
        Ok(())
    }

    #[test]
    fn elicitation_response_and_error_adapter_preserves_payloads() -> Result<()> {
        for fixture in [
            json!({
                "action": "accept",
                "content": {
                    "name": "Ada", "count": 9_007_199_254_740_993_i64, "score": 1.25,
                    "enabled": false, "choices": ["b", "a"], "empty": []
                },
                "_meta": {"response": {"nested": [null, true]}}
            }),
            json!({"action": "accept"}),
            json!({"action": "accept", "content": {}, "_meta": {}}),
            json!({"action": "decline", "_meta": {}}),
            json!({"action": "cancel"}),
            json!({
                "action": "_defer", "content": {"future": {"nested": true}},
                "reason": [null, "later"], "_meta": {"opaque": {}}
            }),
        ] {
            let response: v2::CreateElicitationResponse = serde_json::from_value(fixture.clone())?;
            let converted = response_to_v1(response)?;
            if fixture["action"] == "_defer" {
                assert!(matches!(converted.action, v1::ElicitationAction::Other(_)));
            }
            assert_eq!(serde_json::to_value(converted)?, fixture);
        }

        for (code, message, data) in [
            (-32700, "Parse", None),
            (-32600, "Invalid request", Some(json!(null))),
            (-32601, "Missing method", Some(json!({}))),
            (
                -32602,
                "Invalid parameters",
                Some(json!({"parameter": [1, null]})),
            ),
            (-32603, "Internal", Some(json!("detail"))),
            (-32800, "Canceled", None),
            (-32000, "Authentication", None),
            (-32002, "Missing resource", None),
            (17, "", Some(json!({"opaque": {"nested": [false, 2]}}))),
        ] {
            let converted = error_to_v1(v2::Error::new(code, message).data(data.clone()));
            assert_eq!(i32::from(converted.code), code);
            assert_eq!(converted.message, message);
            assert_eq!(converted.data, data);
        }
        Ok(())
    }
}

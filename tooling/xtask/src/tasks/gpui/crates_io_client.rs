use std::collections::BTreeSet;
use std::fs;

use anyhow::{Context as _, Result, bail};
use serde_json::{Map, Value, json};

use crate::workspace::load_workspace;

/// Everything else is removed from the spec the client is generated from.
const SUPPORTED_ENDPOINTS: &[(&str, HttpMethod)] = &[
    ("/api/v1/crates", HttpMethod::Get),
    ("/api/v1/crates/{name}", HttpMethod::Get),
    ("/api/v1/crates/{name}/owners", HttpMethod::Get),
    ("/api/v1/crates/{name}/owners", HttpMethod::Put),
    ("/api/v1/crates/{name}/{version}", HttpMethod::Get),
    ("/api/v1/trusted_publishing/github_configs", HttpMethod::Get),
    (
        "/api/v1/trusted_publishing/github_configs",
        HttpMethod::Post,
    ),
];

const SPEC_URL: &str = "https://crates.io/api/openapi.json";
/// crates.io requires a user agent that identifies the application.
const USER_AGENT: &str = "Zed GPUI releases (https://github.com/zed-industries/zed)";
const FILTERED_SPEC_PATH: &str = "tooling/crates_io_client/openapi.json";
const OPENAPI_3_0_VERSION: &str = "3.0.3";
const OPENAPI_3_1_DIALECT: &str = "https://spec.openapis.org/oas/3.1/dialect/base";

#[derive(Clone, Copy, Debug, strum::EnumString, strum::IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

fn is_operation(key: &str) -> bool {
    key.parse::<HttpMethod>().is_ok()
}

pub fn run_update_crates_io_client() -> Result<()> {
    eprintln!("Downloading {SPEC_URL}");
    let upstream_spec = tokio::runtime::Runtime::new()
        .context("creating a tokio runtime")?
        .block_on(download_spec())
        .with_context(|| format!("downloading {SPEC_URL}"))?;
    let upstream_spec = serde_json::from_slice(&upstream_spec).context("parsing the spec")?;

    let mut spec = filter_spec(upstream_spec, SUPPORTED_ENDPOINTS)?;
    downgrade_to_openapi_3_0(&mut spec)?;
    // Surfaces parse errors here rather than when compiling the client crate.
    serde_json::from_value::<openapiv3::OpenAPI>(spec.clone())
        .context("the downgraded spec is not a valid OpenAPI 3.0 document")?;

    let path = load_workspace()?
        .workspace_root
        .as_std_path()
        .join(FILTERED_SPEC_PATH);
    fs::write(&path, serde_json::to_string_pretty(&spec)? + "\n")
        .with_context(|| format!("writing {}", path.display()))?;
    eprintln!("Wrote {}", path.display());
    Ok(())
}

async fn download_spec() -> Result<bytes::Bytes> {
    let response = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()?
        .get(SPEC_URL)
        .send()
        .await?
        .error_for_status()?;
    Ok(response.bytes().await?)
}

/// Removes all operations except `endpoints` from `spec`, along with every
/// component that the remaining operations don't reference.
fn filter_spec(mut spec: Value, endpoints: &[(&str, HttpMethod)]) -> Result<Value> {
    // Webhooks describe requests crates.io sends rather than ones a client makes.
    if let Some(spec) = spec.as_object_mut() {
        spec.remove("webhooks");
    }

    let upstream_paths = spec
        .get_mut("paths")
        .and_then(Value::as_object_mut)
        .context("spec has no `paths` object")?;

    let mut paths = Map::new();
    for &(path, method) in endpoints {
        let upstream_path_item = upstream_paths
            .get(path)
            .and_then(Value::as_object)
            .with_context(|| format!("spec has no path {path}"))?;
        let method: &str = method.into();
        let operation = upstream_path_item
            .get(method)
            .with_context(|| format!("spec has no {} operation for {path}", method.to_uppercase()))?
            .clone();

        let path_item = paths
            .entry(path)
            .or_insert_with(|| {
                Value::Object(
                    upstream_path_item
                        .iter()
                        .filter(|(key, _)| !is_operation(key))
                        .map(|(key, value)| (key.clone(), value.clone()))
                        .collect(),
                )
            })
            .as_object_mut()
            .context("path item is not an object")?;
        path_item.insert(method.to_string(), operation);
    }
    *upstream_paths = paths;

    let referenced_components = referenced_components(&spec)?;
    let referenced_security_schemes = referenced_security_schemes(&spec);

    if let Some(components) = spec.get_mut("components").and_then(Value::as_object_mut) {
        for (kind, entries) in components.iter_mut() {
            let Some(entries) = entries.as_object_mut() else {
                continue;
            };
            entries.retain(|name, _| {
                if kind == "securitySchemes" {
                    referenced_security_schemes.contains(name)
                } else {
                    referenced_components.contains(&(kind.clone(), name.clone()))
                }
            });
        }
        components.retain(|_, entries| {
            entries
                .as_object()
                .is_none_or(|entries| !entries.is_empty())
        });
    }

    Ok(spec)
}

/// Returns the `(kind, name)` of every component transitively referenced from
/// outside of `components`.
fn referenced_components(spec: &Value) -> Result<BTreeSet<(String, String)>> {
    let mut pending_references = Vec::new();
    for (key, value) in spec.as_object().into_iter().flatten() {
        if key != "components" {
            collect_references(value, &mut pending_references);
        }
    }

    let mut referenced = BTreeSet::new();
    while let Some(reference) = pending_references.pop() {
        let (kind, name) = reference
            .strip_prefix("#/components/")
            .and_then(|component| component.split_once('/'))
            .with_context(|| format!("unsupported reference {reference:?}"))?;
        if !referenced.insert((kind.to_string(), name.to_string())) {
            continue;
        }
        let component = spec
            .get("components")
            .and_then(|components| components.get(kind))
            .and_then(|entries| entries.get(name))
            .with_context(|| format!("dangling reference {reference:?}"))?;
        collect_references(component, &mut pending_references);
    }
    Ok(referenced)
}

fn collect_references(value: &Value, references: &mut Vec<String>) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                match (key.as_str(), value) {
                    ("$ref", Value::String(reference)) => references.push(reference.clone()),
                    _ => collect_references(value, references),
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_references(value, references);
            }
        }
        _ => {}
    }
}

/// Security schemes are referenced by name from `security` requirements
/// rather than through `$ref`.
fn referenced_security_schemes(spec: &Value) -> BTreeSet<String> {
    let operations = spec
        .get("paths")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|paths| paths.values())
        .filter_map(Value::as_object)
        .flat_map(|path_item| {
            path_item
                .iter()
                .filter(|(key, _)| is_operation(key))
                .map(|(_, operation)| operation)
        });

    std::iter::once(spec)
        .chain(operations)
        .filter_map(|value| value.get("security").and_then(Value::as_array))
        .flatten()
        .filter_map(Value::as_object)
        .flat_map(|requirement| requirement.keys().cloned())
        .collect()
}

const UNSUPPORTED_SCHEMA_KEYWORDS: &[&str] = &[
    "$id",
    "$schema",
    "$anchor",
    "$dynamicRef",
    "$dynamicAnchor",
    "$defs",
    "prefixItems",
    "contains",
    "minContains",
    "maxContains",
    "unevaluatedItems",
    "unevaluatedProperties",
    "patternProperties",
    "dependentRequired",
    "dependentSchemas",
    "if",
    "then",
    "else",
    "contentEncoding",
    "contentMediaType",
    "contentSchema",
];

/// crates.io publishes an OpenAPI 3.1 document, but progenitor only supports
/// 3.0, whose parser fails on or silently ignores 3.1 constructs. Constructs
/// that 3.0 can't express are rejected rather than letting progenitor generate
/// types that don't match the API.
fn downgrade_to_openapi_3_0(spec: &mut Value) -> Result<()> {
    let version = spec
        .get("openapi")
        .and_then(Value::as_str)
        .context("spec has no `openapi` version")?;
    if !version.starts_with("3.1.") {
        bail!("unsupported OpenAPI version {version}");
    }
    let spec = spec.as_object_mut().context("spec is not an object")?;
    spec.insert("openapi".to_string(), json!(OPENAPI_3_0_VERSION));
    if let Some(dialect) = spec.remove("jsonSchemaDialect")
        && dialect != OPENAPI_3_1_DIALECT
    {
        bail!("unsupported JSON Schema dialect {dialect}");
    }
    spec.values_mut().try_for_each(downgrade_nested_schemas)
}

/// Downgrades the Schema Objects nested in a part of the document that isn't a
/// schema itself. Outside of schemas, `schema` (in Parameter, Header and Media
/// Type Objects) and `schemas` (in the Components Object) always hold Schema
/// Objects, whereas examples, links and extensions hold arbitrary values.
fn downgrade_nested_schemas(value: &mut Value) -> Result<()> {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                match key.as_str() {
                    "schema" => downgrade_schema(value)?,
                    "schemas" => downgrade_schemas(value)?,
                    "example" | "examples" | "links" => {}
                    "webhooks" | "pathItems" => bail!("cannot express `{key}` in OpenAPI 3.0"),
                    key if key.starts_with("x-") => {}
                    _ => downgrade_nested_schemas(value)?,
                }
            }
        }
        Value::Array(values) => values.iter_mut().try_for_each(downgrade_nested_schemas)?,
        _ => {}
    }
    Ok(())
}

fn downgrade_schemas(value: &mut Value) -> Result<()> {
    match value {
        Value::Array(schemas) => schemas.iter_mut().try_for_each(downgrade_schema),
        Value::Object(schemas) => schemas.values_mut().try_for_each(downgrade_schema),
        _ => bail!("expected an array or object of schemas, found {value}"),
    }
}

fn downgrade_schema(schema: &mut Value) -> Result<()> {
    let object = match schema {
        Value::Bool(true) => {
            *schema = json!({});
            return Ok(());
        }
        Value::Object(object) => object,
        _ => bail!("cannot express schema {schema} in OpenAPI 3.0"),
    };
    // `nullable` is a no-op annotation in 3.1, but would widen the type in 3.0.
    object.remove("nullable");
    // Runs before recursing so that the null variant is still recognizable.
    downgrade_null_variants(object)?;
    // Runs after merging the null variant's siblings into `object`.
    if let Some(keyword) = UNSUPPORTED_SCHEMA_KEYWORDS
        .iter()
        .find(|keyword| object.contains_key(**keyword))
    {
        bail!("cannot express `{keyword}` in OpenAPI 3.0");
    }

    for (keyword, value) in object.iter_mut() {
        match keyword.as_str() {
            "items" | "not" => downgrade_schema(value)?,
            // Unlike other schemas, `additionalProperties` can be a boolean in 3.0.
            "additionalProperties" if value.is_object() => downgrade_schema(value)?,
            "allOf" | "anyOf" | "oneOf" | "properties" => downgrade_schemas(value)?,
            _ => {}
        }
    }

    if let Some(Value::Array(types)) = object.get("type") {
        let non_null_types = types
            .iter()
            .filter(|type_name| *type_name != "null")
            .collect::<Vec<_>>();
        let [type_name] = non_null_types.as_slice() else {
            bail!("cannot express type {types:?} in OpenAPI 3.0");
        };
        let is_nullable = non_null_types.len() < types.len();
        let type_name = (*type_name).clone();
        object.insert("type".to_string(), type_name);
        if is_nullable {
            object.insert("nullable".to_string(), json!(true));
        }
    }
    if object
        .get("type")
        .is_some_and(|type_name| type_name == "null")
    {
        bail!("cannot express a `null` schema outside of `oneOf` or `anyOf` in OpenAPI 3.0");
    }

    for (exclusive, inclusive) in [
        ("exclusiveMinimum", "minimum"),
        ("exclusiveMaximum", "maximum"),
    ] {
        let Some(bound) = object
            .get(exclusive)
            .filter(|bound| bound.is_number())
            .cloned()
        else {
            continue;
        };
        if object.contains_key(inclusive) {
            bail!("cannot express both `{inclusive}` and `{exclusive}` in OpenAPI 3.0");
        }
        object.insert(inclusive.to_string(), bound);
        object.insert(exclusive.to_string(), json!(true));
    }

    if let Some(value) = object.remove("const") {
        if object.contains_key("enum") {
            bail!("cannot express both `const` and `enum` in OpenAPI 3.0");
        }
        object.insert("enum".to_string(), json!([value]));
    }
    if let Some(Value::Array(examples)) = object.remove("examples")
        && let Some(example) = examples.into_iter().next()
    {
        object.entry("example").or_insert(example);
    }
    // JSON object keys are always strings, so this doesn't constrain anything.
    if let Some(property_names) = object.remove("propertyNames")
        && property_names != json!({ "type": "string" })
    {
        bail!("cannot express `propertyNames` {property_names} in OpenAPI 3.0");
    }
    object.remove("$comment");

    // OpenAPI 3.0 ignores the siblings of `$ref`, whereas 3.1 applies them.
    if object.len() > 1
        && let Some(reference) = object.remove("$ref")
    {
        object
            .entry("allOf")
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .context("`allOf` is not an array")?
            .insert(0, json!({ "$ref": reference }));
    }
    Ok(())
}

/// Rewrites `"oneOf": [{"type": "null"}, schema]` to a nullable `schema`.
///
/// OpenAPI 3.0 has no exact equivalent for a nullable `$ref`, union or `enum`,
/// since `nullable` formally only applies alongside `type` and doesn't bypass
/// `enum`. progenitor however treats `nullable` as "null or the rest of the
/// schema" in all of these cases, which matches the 3.1 semantics.
fn downgrade_null_variants(schema: &mut Map<String, Value>) -> Result<()> {
    for keyword in ["oneOf", "anyOf"] {
        let Some(Value::Array(variants)) = schema.get_mut(keyword) else {
            continue;
        };
        let variant_count = variants.len();
        variants.retain(|variant| *variant != json!({ "type": "null" }));
        if variants.len() == variant_count {
            continue;
        }
        let single_variant = match variants.as_mut_slice() {
            [Value::Object(variant)] => Some(std::mem::take(variant)),
            _ => None,
        };
        // In 3.1, these siblings still reject null, which `nullable` can't express.
        if let Some(sibling) = ["type", "enum", "const"]
            .into_iter()
            .find(|sibling| schema.contains_key(*sibling))
        {
            bail!(
                "cannot express a nullable `{keyword}` with a sibling `{sibling}` in OpenAPI 3.0"
            );
        }
        schema.insert("nullable".to_string(), json!(true));

        let Some(variant) = single_variant else {
            continue;
        };
        schema.remove(keyword);
        for (key, value) in variant {
            match schema.get(&key) {
                Some(existing) if *existing != value => {
                    bail!("conflicting `{key}` in nullable `{keyword}` variant")
                }
                _ => schema.insert(key, value),
            };
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_spec_keeps_only_supported_endpoints_and_their_components() -> Result<()> {
        let spec = json!({
            "openapi": "3.1.0",
            "paths": {
                "/crates/{name}": {
                    "parameters": [{ "$ref": "#/components/parameters/Name" }],
                    "get": {
                        "security": [{ "api_token": [] }],
                        "responses": {
                            "200": { "content": { "application/json": { "schema": {
                                "$ref": "#/components/schemas/Crate"
                            } } } }
                        }
                    },
                    "patch": {
                        "responses": {
                            "200": { "content": { "application/json": { "schema": {
                                "$ref": "#/components/schemas/Unrelated"
                            } } } }
                        }
                    }
                },
                "/users/{id}": {
                    "get": { "responses": {} }
                }
            },
            "components": {
                "parameters": { "Name": { "name": "name", "in": "path" } },
                "schemas": {
                    "Crate": { "properties": { "owner": { "$ref": "#/components/schemas/User" } } },
                    "User": { "type": "object" },
                    "Unrelated": { "type": "object" }
                },
                "responses": { "Unused": {} },
                "securitySchemes": {
                    "api_token": { "type": "apiKey" },
                    "cookie": { "type": "apiKey" }
                }
            }
        });

        let filtered = filter_spec(spec, &[("/crates/{name}", HttpMethod::Get)])?;

        assert_eq!(
            filtered,
            json!({
                "openapi": "3.1.0",
                "paths": {
                    "/crates/{name}": {
                        "parameters": [{ "$ref": "#/components/parameters/Name" }],
                        "get": {
                            "security": [{ "api_token": [] }],
                            "responses": {
                                "200": { "content": { "application/json": { "schema": {
                                    "$ref": "#/components/schemas/Crate"
                                } } } }
                            }
                        }
                    }
                },
                "components": {
                    "parameters": { "Name": { "name": "name", "in": "path" } },
                    "schemas": {
                        "Crate": { "properties": { "owner": { "$ref": "#/components/schemas/User" } } },
                        "User": { "type": "object" }
                    },
                    "securitySchemes": { "api_token": { "type": "apiKey" } }
                }
            })
        );
        Ok(())
    }

    #[test]
    fn filter_spec_rejects_missing_endpoints() {
        let spec = json!({
            "paths": { "/crates/{name}": { "get": {} } },
        });
        assert!(filter_spec(spec.clone(), &[("/crates", HttpMethod::Get)]).is_err());
        assert!(filter_spec(spec, &[("/crates/{name}", HttpMethod::Put)]).is_err());
    }

    #[test]
    fn downgrade_rewrites_openapi_3_1_constructs() -> Result<()> {
        let mut spec = json!({
            "openapi": "3.1.0",
            "jsonSchemaDialect": OPENAPI_3_1_DIALECT,
            "paths": { "/crates": { "get": {
                "parameters": [{
                    "name": "page",
                    "in": "query",
                    "schema": { "type": "integer", "exclusiveMinimum": 0, "examples": [1, 2] },
                    "example": { "type": ["not", "a", "schema"] }
                }],
                "responses": {},
                "x-internal": { "schema": "not a schema" }
            } } },
            "components": { "schemas": { "Crate": { "properties": {
                "description": { "type": ["string", "null"] },
                "published_by": {
                    "oneOf": [
                        { "type": "null" },
                        { "$ref": "#/components/schemas/User", "description": "The publisher." }
                    ]
                },
                "inline": { "oneOf": [{ "type": "null" }, { "type": "integer" }] },
                "either": {
                    "oneOf": [{ "type": "null" }, { "type": "integer" }, { "type": "string" }]
                },
                "links": { "$ref": "#/components/schemas/Links", "description": "Links." },
                "kind": { "const": "crate", "nullable": true },
                "features": {
                    "type": "object",
                    "additionalProperties": { "items": true },
                    "propertyNames": { "type": "string" }
                }
            } } } }
        });

        downgrade_to_openapi_3_0(&mut spec)?;

        assert_eq!(
            spec,
            json!({
                "openapi": "3.0.3",
                "paths": { "/crates": { "get": {
                    "parameters": [{
                        "name": "page",
                        "in": "query",
                        "schema": {
                            "type": "integer",
                            "minimum": 0,
                            "exclusiveMinimum": true,
                            "example": 1
                        },
                        "example": { "type": ["not", "a", "schema"] }
                    }],
                    "responses": {},
                    "x-internal": { "schema": "not a schema" }
                } } },
                "components": { "schemas": { "Crate": { "properties": {
                    "description": { "type": "string", "nullable": true },
                    "published_by": {
                        "allOf": [{ "$ref": "#/components/schemas/User" }],
                        "description": "The publisher.",
                        "nullable": true
                    },
                    "inline": { "type": "integer", "nullable": true },
                    "either": {
                        "oneOf": [{ "type": "integer" }, { "type": "string" }],
                        "nullable": true
                    },
                    "links": {
                        "allOf": [{ "$ref": "#/components/schemas/Links" }],
                        "description": "Links."
                    },
                    "kind": { "enum": ["crate"] },
                    "features": { "type": "object", "additionalProperties": { "items": {} } }
                } } } }
            })
        );
        Ok(())
    }

    #[test]
    fn downgrade_rejects_constructs_openapi_3_0_cannot_express() {
        for schema in [
            json!({ "type": ["string", "integer"] }),
            json!({ "type": "null" }),
            json!({ "prefixItems": [{ "type": "string" }] }),
            json!({ "properties": { "nested": { "if": { "type": "string" } } } }),
            json!({ "propertyNames": { "pattern": "^[a-z]+$" } }),
            json!({ "minimum": 1, "exclusiveMinimum": 0 }),
            json!({ "oneOf": [{ "type": "null" }, { "maxLength": 3 }], "type": "string" }),
            json!({ "oneOf": [{ "type": "null" }, { "patternProperties": {} }] }),
        ] {
            let mut spec = json!({
                "openapi": "3.1.0",
                "components": { "schemas": { "Value": schema.clone() } }
            });
            assert!(
                downgrade_to_openapi_3_0(&mut spec).is_err(),
                "expected an error for {schema}"
            );
        }
    }
}

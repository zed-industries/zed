//! # json_schema_store
use std::{
    str::FromStr,
    sync::{Arc, OnceLock},
};

use anyhow::{Context as _, Result};
use gpui::{App, AsyncApp};
use schemars::{Schema, SchemaGenerator};
use std::collections::HashMap;

static ALL_ACTION_SCHEMAS: OnceLock<HashMap<&'static str, Option<Schema>>> = OnceLock::new();
// Origin: https://github.com/SchemaStore/schemastore
const TSCONFIG_SCHEMA: &str = include_str!("schemas/tsconfig.json");
const PACKAGE_JSON_SCHEMA: &str = include_str!("schemas/package.json");

pub fn init(cx: &mut App) {
    let schema_store = Arc::new(SchemaStore {});
    project::lsp_store::json_language_server_ext::register_schema_handler(schema_store, cx);
}

struct SchemaStore {}

impl project::lsp_store::json_language_server_ext::SchemaHandling for SchemaStore {
    fn handle_schema_request(&self, uri: String, cx: &mut AsyncApp) -> Result<String> {
        let schema = resolve_schema_request(uri, cx)?;
        serde_json::to_string(&schema).context("Failed to serialize schema")
    }
}

fn resolve_schema_request(uri: String, cx: &mut AsyncApp) -> Result<serde_json::Value> {
    let path = uri.strip_prefix("zed://schemas/").context("Invalid URI")?;

    let (family, rest) = path.split_once('/').unzip();
    let family = family.unwrap_or(path);
    let schema = match family {
        "action" => {
            let normalized_action_name = rest.context("No Action name provided")?;
            let action_name = denormalize_action_name(normalized_action_name);
            let mut generator = settings::KeymapFile::action_schema_generator();
            let schema = cx
                .update(|cx| all_action_schemas(&mut generator, cx))?
                .get(action_name.as_str())
                .and_then(Option::clone);
            root_schema_from_action_schema(schema, &mut generator).to_value()
        }
        "package_json" => package_json_schema(),
        "tsconfig" => tsconfig_schema(),
        _ => {
            anyhow::bail!("Unrecognized schema family: {}", family);
        }
    };
    Ok(schema)
}

pub fn all_schema_file_associations(cx: &mut App) -> Vec<serde_json::Value> {
    let mut file_associations = vec![
        serde_json::json!({
            "fileMatch": ["tsconfig.json"],
            "schema": "zed://schemas/tsconfig"
        }),
        serde_json::json!({
            "fileMatch": ["package.json"],
            "schema": "zed://schemas/package_json"
        }),
    ];
    file_associations.extend(
        // PERF: use all_action_schemas() and don't include action schemas with no arguments
        cx.all_action_names()
            .into_iter()
            .map(|&name| url_schema_for_action(name)),
    );

    file_associations
}

fn all_action_schemas(
    generator: &mut SchemaGenerator,
    cx: &mut App,
) -> &'static HashMap<&'static str, Option<Schema>> {
    ALL_ACTION_SCHEMAS.get_or_init(|| {
        let all_schemas = HashMap::from_iter(cx.action_schemas(generator));
        all_schemas
    })
}

fn tsconfig_schema() -> serde_json::Value {
    serde_json::Value::from_str(TSCONFIG_SCHEMA).unwrap()
}

fn package_json_schema() -> serde_json::Value {
    serde_json::Value::from_str(PACKAGE_JSON_SCHEMA).unwrap()
}

pub fn normalize_action_name(action_name: &str) -> String {
    action_name.replace("::", "__")
}

pub fn denormalize_action_name(action_name: &str) -> String {
    action_name.replace("__", "::")
}

pub fn normalized_action_file_name(action_name: &str) -> String {
    normalized_action_name_to_file_name(normalize_action_name(action_name))
}

pub fn normalized_action_name_to_file_name(mut normalized_action_name: String) -> String {
    normalized_action_name.push_str(".json");
    normalized_action_name
}

pub fn url_schema_for_action(action_name: &str) -> serde_json::Value {
    let normalized_name = normalize_action_name(action_name);
    let file_name = normalized_action_name_to_file_name(normalized_name.clone());
    serde_json::json!({
        "fileMatch": [file_name],
        "url": format!("zed://schemas/action/{}", normalized_name)
    })
}

fn root_schema_from_action_schema(
    action_schema: Option<schemars::Schema>,
    generator: &mut schemars::SchemaGenerator,
) -> schemars::Schema {
    let Some(mut action_schema) = action_schema else {
        return schemars::json_schema!(false);
    };
    let meta_schema = generator
        .settings()
        .meta_schema
        .as_ref()
        .expect("meta_schema should be present in schemars settings")
        .to_string();
    let defs = generator.definitions();
    let mut schema = schemars::json_schema!({
        "$schema": meta_schema,
        "allowTrailingCommas": true,
        "$defs": defs,
    });
    schema
        .ensure_object()
        .extend(std::mem::take(action_schema.ensure_object()));
    schema
}

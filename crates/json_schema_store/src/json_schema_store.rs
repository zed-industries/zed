//! # json_schema_store
use std::sync::{Arc, OnceLock};

use anyhow::{Context as _, Result};
use gpui::{App, AsyncApp};
use schemars::Schema;
use std::collections::HashMap;

static ALL_ACTION_SCHEMAS: OnceLock<HashMap<&'static str, Option<Schema>>> = OnceLock::new();

pub fn init(cx: &mut App) {
    let schema_store = Arc::new(SchemaStore {});
    project::lsp_store::json_language_server_ext::register_schema_handler(schema_store, cx);
}

struct SchemaStore {}

impl project::lsp_store::json_language_server_ext::SchemaHandling for SchemaStore {
    fn handle_schema_request(&self, uri: String, cx: &mut AsyncApp) -> Result<String> {
        let normalized_action_name = uri
            .strip_prefix("zed://schemas/action/")
            .context("Invalid URI")?;
        let action_name = denormalize_action_name(normalized_action_name);
        let schema = cx.update(|cx| {
            let mut generator = settings::KeymapFile::action_schema_generator();
            root_schema_from_action_schema(
                all_action_schemas(cx)
                    .get(action_name.as_str())
                    .and_then(Option::as_ref),
                &mut generator,
            )
            .to_value()
        })?;

        serde_json::to_string(&schema).context("Failed to serialize schema")
    }
}

fn all_action_schemas(cx: &mut App) -> &HashMap<&'static str, Option<Schema>> {
    ALL_ACTION_SCHEMAS.get_or_init(|| {
        let mut generator = settings::KeymapFile::action_schema_generator();
        let all_schemas = HashMap::from_iter(cx.action_schemas(&mut generator));
        all_schemas
    })
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
    action_schema: Option<&schemars::Schema>,
    generator: &mut schemars::SchemaGenerator,
) -> schemars::Schema {
    let Some(action_schema) = action_schema else {
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
        .extend(std::mem::take(action_schema.clone().ensure_object()));
    schema
}

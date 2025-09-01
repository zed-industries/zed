use anyhow::Context as _;
use collections::HashMap;
use gpui::WeakEntity;
use lsp::LanguageServer;

use crate::LspStore;
/// https://github.com/Microsoft/vscode/blob/main/extensions/json-language-features/server/README.md#schema-content-request
///
/// Represents a "JSON language server-specific, non-standardized, extension to the LSP" with which the vscode-json-language-server
/// can request the contents of a schema that is associated with a uri scheme it does not support.
/// In our case, we provide the uris for actions on server startup under the `zed://schemas/action/{normalize_action_name}` scheme.
/// We can then respond to this request with the schema content on demand, thereby greatly reducing the total size of the JSON we send to the server on startup
struct SchemaContentRequest {}

impl lsp::request::Request for SchemaContentRequest {
    type Params = Vec<String>;

    type Result = String;

    const METHOD: &'static str = "vscode/content";
}

pub fn register_requests(_lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    language_server
        .on_request::<SchemaContentRequest, _, _>(|params, cx| {
            // PERF: Use a cache (`OnceLock`?) to avoid recomputing the action schemas
            let mut generator = settings::KeymapFile::action_schema_generator();
            let all_schemas = cx.update(|cx| HashMap::from_iter(cx.action_schemas(&mut generator)));
            async move {
                let all_schemas = all_schemas?;
                let Some(uri) = params.get(0) else {
                    anyhow::bail!("No URI");
                };
                let normalized_action_name = uri
                    .strip_prefix("zed://schemas/action/")
                    .context("Invalid URI")?;
                let action_name = denormalize_action_name(normalized_action_name);
                let schema = root_schema_from_action_schema(
                    all_schemas
                        .get(action_name.as_str())
                        .and_then(Option::as_ref),
                    &mut generator,
                )
                .to_value();

                serde_json::to_string(&schema).context("Failed to serialize schema")
            }
        })
        .detach();
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

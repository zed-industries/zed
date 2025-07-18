use ::serde::{Deserialize, Serialize};
use anyhow::Context as _;
use collections::HashMap;
use gpui::{App, Entity, WeakEntity};
use language::Buffer;
use language::{File as _, LocalFile as _};
use lsp::{DidCloseTextDocumentParams, DidOpenTextDocumentParams, LanguageServer};
use util::ResultExt as _;

use crate::{LspStore, Project};

// https://github.com/microsoft/vscode/blob/main/extensions/json-language-features/server/README.md#schema-associations-notification
struct SchemaAssociationsNotification {}

/// interface ISchemaAssociation {
///   /**
///    * The URI of the schema, which is also the identifier of the schema.
///    */
///   uri: string;
///
///   /**
///    * A list of file path patterns that are associated to the schema. The '*' wildcard can be used. Exclusion patterns starting with '!'.
///    * For example '*.schema.json', 'package.json', '!foo*.schema.json'.
///    * A match succeeds when there is at least one pattern matching and last matching pattern does not start with '!'.
///    */
///   fileMatch: string[];
///   /**
///    * If provided, the association is only used if the validated document is located in the given folder (directly or in a subfolder)
///    */
///   folderUri?: string;
///   /*
///    * The schema for the given URI.
///    * If no schema is provided, the schema will be fetched with the schema request service (if available).
///    */
///   schema?: JSONSchema;
/// }
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchemaAssociation {
    pub uri: String,
    pub file_match: Vec<String>,
    pub folder_uri: Option<String>,
    pub schema: Option<serde_json::Value>,
}

impl lsp::notification::Notification for SchemaAssociationsNotification {
    type Params = Vec<SchemaAssociation>;
    const METHOD: &'static str = "json/schemaAssociations";
}

pub fn send_schema_associations_notification(
    project: Entity<Project>,
    buffer: Entity<Buffer>,
    schema_associations: &Vec<SchemaAssociation>,
    cx: &mut App,
) {
    let lsp_store = project.read(cx).lsp_store();
    lsp_store.update(cx, |lsp_store, cx| {
        let Some(local) = lsp_store.as_local_mut() else {
            return;
        };
        buffer.update(cx, |buffer, cx| {
            for (cached_adapter, server) in local
                .language_servers_for_buffer(buffer, cx)
                .map(|(a, b)| (a.clone(), b.clone()))
                .collect::<Vec<_>>()
            {
                if !cached_adapter.adapter.is_primary_zed_json_schema_adapter() {
                    continue;
                }

                server
                    .notify::<SchemaAssociationsNotification>(schema_associations)
                    .log_err(); // todo! don't ignore error
            }
        })
    })
}

struct SchemaContentRequest {}

impl lsp::request::Request for SchemaContentRequest {
    type Params = Vec<String>;

    type Result = String;

    const METHOD: &'static str = "vscode/content";
}

pub fn register_requests(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    language_server
        .on_request::<SchemaContentRequest, _, _>(|params, cx| {
            let mut generator = settings::KeymapFile::action_schema_generator();
            let all_schemas = cx.update(|cx| HashMap::from_iter(cx.action_schemas(&mut generator)));
            async move {
                let all_schemas = all_schemas?;
                eprintln!("Received request for schema {:?}", params);
                let Some(uri) = params.get(0) else {
                    anyhow::bail!("No URI");
                };
                let action_name = uri
                    .strip_prefix("zed://schemas/action/")
                    .context("Invalid URI")?;
                let action_name = action_name.replace("__", "::");
                let schema = root_schema_from_action_schema(
                    all_schemas
                        .get(action_name.as_str())
                        .context("No schema found")?
                        .as_ref(),
                    &mut generator,
                )
                .to_value()
                .to_string();

                Ok(schema)
            }
        })
        .detach();
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
        .extend(std::mem::take(action_schema.clone().ensure_object()).into_iter());
    schema
}

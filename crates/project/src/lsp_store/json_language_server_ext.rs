use ::serde::{Deserialize, Serialize};
use gpui::{App, Entity, WeakEntity};
use language::Buffer;
use lsp::LanguageServer;
use util::ResultExt as _;

use crate::{LspStore, Project};

pub fn register_notifications(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    let name = language_server.name();
}

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
        let Some(local) = lsp_store.as_local() else {
            return;
        };
        buffer.update(cx, |buffer, cx| {
            for (adapter, server) in local.language_servers_for_buffer(buffer, cx) {
                if !adapter.adapter.is_primary_zed_json_schema_adapter() {
                    continue;
                }
                server
                    .notify::<SchemaAssociationsNotification>(schema_associations)
                    .log_err(); // todo! don't ignore error
            }
        })
    })
}

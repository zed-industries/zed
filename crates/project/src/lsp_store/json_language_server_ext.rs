use anyhow::{Context, Result};
use gpui::{App, AsyncApp, Entity, Global, Task, WeakEntity};
use lsp::LanguageServer;

use crate::LspStore;

const LOGGER: zlog::Logger = zlog::scoped!("json-schema");

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

type SchemaRequestHandler = fn(Entity<LspStore>, String, &mut AsyncApp) -> Task<Result<String>>;
pub struct SchemaHandlingImpl(SchemaRequestHandler);

impl Global for SchemaHandlingImpl {}

pub fn register_schema_handler(handler: SchemaRequestHandler, cx: &mut App) {
    debug_assert!(
        !cx.has_global::<SchemaHandlingImpl>(),
        "SchemaHandlingImpl already registered"
    );
    cx.set_global(SchemaHandlingImpl(handler));
}

struct SchemaContentsChanged {}

impl lsp::notification::Notification for SchemaContentsChanged {
    const METHOD: &'static str = "json/schemaContent";
    type Params = String;
}

pub fn notify_schema_changed(lsp_store: Entity<LspStore>, uri: String, cx: &App) {
    zlog::trace!(LOGGER => "Notifying schema changed for URI: {:?}", uri);
    let servers = lsp_store.read_with(cx, |lsp_store, _| {
        let mut servers = Vec::new();
        let Some(local) = lsp_store.as_local() else {
            return servers;
        };

        for states in local.language_servers.values() {
            let json_server = match states {
                super::LanguageServerState::Running {
                    adapter, server, ..
                } if adapter.adapter.is_primary_zed_json_schema_adapter() => server.clone(),
                _ => continue,
            };

            servers.push(json_server);
        }
        servers
    });
    for server in servers {
        zlog::trace!(LOGGER => "Notifying server {:?} of schema change for URI: {:?}", server.server_id(), &uri);
        // TODO: handle errors
        server.notify::<SchemaContentsChanged>(uri.clone()).ok();
    }
}

pub fn register_requests(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    language_server
        .on_request::<SchemaContentRequest, _, _>(move |params, cx| {
            let handler = cx.try_read_global::<SchemaHandlingImpl, _>(|handler, _| handler.0);
            let mut cx = cx.clone();
            let uri = params.clone().pop();
            let lsp_store = lsp_store.clone();
            let resolution = async move {
                let lsp_store = lsp_store.upgrade().context("LSP store has been dropped")?;
                let uri = uri.context("No URI")?;
                let handle_schema_request = handler.context("No schema handler registered")?;
                handle_schema_request(lsp_store, uri, &mut cx).await
            };
            async move {
                zlog::trace!(LOGGER => "Handling schema request for {:?}", &params);
                let result = resolution.await;
                match &result {
                    Ok(content) => {zlog::trace!(LOGGER => "Schema request resolved with {}B schema", content.len());},
                    Err(err) => {zlog::warn!(LOGGER => "Schema request failed: {}", err);},
                }
                result
            }
        })
        .detach();
}

use std::sync::Arc;

use anyhow::{Context, Result};
use gpui::{App, AppContext, AsyncApp, Global, WeakEntity};
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

pub trait SchemaHandling {
    fn handle_schema_request(&self, params: String, cx: &mut AsyncApp) -> Result<String>;
}

pub struct SchemaHandlingImpl(Arc<dyn SchemaHandling>);

impl Global for SchemaHandlingImpl {}

pub fn register_schema_handler(handler: Arc<dyn SchemaHandling>, cx: &mut App) {
    debug_assert!(
        !cx.has_global::<SchemaHandlingImpl>(),
        "SchemaHandlingImpl already registered"
    );
    cx.set_global(SchemaHandlingImpl(handler));
}

pub fn register_requests(_lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    language_server
        .on_request::<SchemaContentRequest, _, _>(|mut params, cx| {
            let handler = cx.try_read_global::<SchemaHandlingImpl, _>(|schema_handling_impl, _| {
                schema_handling_impl.0.clone()
            });
            let mut cx = cx.clone();
            let logger = zlog::scoped!("json-schema");
            let uri = params.clone().pop();
            let resolution = async move {
                let uri = uri.context("No URI")?;
                let handler = handler.context("No schema handler registered")?;
                handler.handle_schema_request(uri, &mut cx)
            };
            async move {
                zlog::trace!(logger => "Handling schema request for {:?}", &params);
                let result = resolution.await;
                match &result {
                    Ok(content) => {zlog::trace!(logger => "Schema request resolved with {}B schema", content.len());},
                    Err(err) => {zlog::warn!(logger => "Schema request failed: {}", err);},
                }
                result
            }
        })
        .detach();
}

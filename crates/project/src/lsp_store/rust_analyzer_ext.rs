use ::serde::{Deserialize, Serialize};
use gpui::{PromptLevel, WeakEntity};
use lsp::LanguageServer;

use crate::{LanguageServerPromptRequest, LspStore, LspStoreEvent};

pub const RUST_ANALYZER_NAME: &str = "rust-analyzer";
pub const CARGO_DIAGNOSTICS_SOURCE_NAME: &str = "rustc";

/// Experimental: Informs the end user about the state of the server
///
/// [Rust Analyzer Specification](https://rust-analyzer.github.io/book/contributing/lsp-extensions.html#server-status)
#[derive(Debug)]
enum ServerStatus {}

/// Other(String) variant to handle unknown values due to this still being experimental
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
enum ServerHealthStatus {
    Ok,
    Warning,
    Error,
    Other(String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ServerStatusParams {
    pub health: ServerHealthStatus,
    pub message: Option<String>,
}

impl lsp::notification::Notification for ServerStatus {
    type Params = ServerStatusParams;
    const METHOD: &'static str = "experimental/serverStatus";
}

pub fn register_notifications(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    let name = language_server.name();
    let server_id = language_server.server_id();

    language_server
        .on_notification::<ServerStatus, _>({
            let name = name.to_string();
            move |params, cx| {
                let name = name.to_string();
                if let Some(ref message) = params.message {
                    let message = message.trim();
                    if !message.is_empty() {
                        let formatted_message = format!(
                            "Language server {name} (id {server_id}) status update: {message}"
                        );
                        match params.health {
                            ServerHealthStatus::Ok => log::info!("{formatted_message}"),
                            ServerHealthStatus::Warning => log::warn!("{formatted_message}"),
                            ServerHealthStatus::Error => {
                                log::error!("{formatted_message}");
                                let (tx, _rx) = smol::channel::bounded(1);
                                let request = LanguageServerPromptRequest {
                                    level: PromptLevel::Critical,
                                    message: params.message.unwrap_or_default(),
                                    actions: Vec::new(),
                                    response_channel: tx,
                                    lsp_name: name.clone(),
                                };
                                lsp_store
                                    .update(cx, |_, cx| {
                                        cx.emit(LspStoreEvent::LanguageServerPrompt(request));
                                    })
                                    .ok();
                            }
                            ServerHealthStatus::Other(status) => {
                                log::info!("Unknown server health: {status}\n{formatted_message}")
                            }
                        }
                    }
                }
            }
        })
        .detach();
}

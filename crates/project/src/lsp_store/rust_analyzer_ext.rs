use ::serde::{Deserialize, Serialize};
use gpui::{PromptLevel, WeakEntity};
use lsp::LanguageServer;

use crate::{LanguageServerPromptRequest, LspStore, LspStoreEvent};

pub const RUST_ANALYZER_NAME: &str = "rust-analyzer";

pub const EXTRA_SUPPORTED_COMMANDS: &[&str] = &[
    "rust-analyzer.runSingle",
    "rust-analyzer.showReferences",
    "rust-analyzer.gotoLocation",
    "rust-analyzer.triggerParameterHints",
    "rust-analyzer.rename",
];

/// Experimental: Informs the end user about the state of the server
///
/// [Rust Analyzer Specification](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/lsp-extensions.md#server-status)
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

    let this = lsp_store;

    language_server
        .on_notification::<ServerStatus, _>({
            let name = name.to_string();
            move |params, mut cx| {
                let this = this.clone();
                let name = name.to_string();
                if let Some(ref message) = params.message {
                    let message = message.trim();
                    if !message.is_empty() {
                        let formatted_message = format!(
                            "Language server {name} (id {server_id}) status update: {message}"
                        );
                        match params.health {
                            ServerHealthStatus::Ok => log::info!("{}", formatted_message),
                            ServerHealthStatus::Warning => log::warn!("{}", formatted_message),
                            ServerHealthStatus::Error => {
                                log::error!("{}", formatted_message);
                                let (tx, _rx) = smol::channel::bounded(1);
                                let request = LanguageServerPromptRequest {
                                    level: PromptLevel::Critical,
                                    message: params.message.unwrap_or_default(),
                                    actions: Vec::new(),
                                    response_channel: tx,
                                    lsp_name: name.clone(),
                                };
                                let _ = this
                                    .update(&mut cx, |_, cx| {
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

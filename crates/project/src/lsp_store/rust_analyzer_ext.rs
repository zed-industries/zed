use std::path::PathBuf;

use ::serde::{Deserialize, Serialize};
use collections::HashMap;
use gpui::{PromptLevel, WeakEntity};
use lsp::LanguageServer;

use crate::{LanguageServerPromptRequest, LspStore, LspStoreEvent};

pub const RUST_ANALYZER_NAME: &str = "rust-analyzer";

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

// https://rust-analyzer.github.io/book/contributing/lsp-extensions.html#runnables
// Taken from https://github.com/rust-lang/rust-analyzer/blob/a73a37a757a58b43a796d3eb86a1f7dfd0036659/crates/rust-analyzer/src/lsp/ext.rs#L425-L489
pub enum Runnables {}

impl lsp::request::Request for Runnables {
    type Params = RunnablesParams;
    type Result = Vec<Runnable>;
    const METHOD: &'static str = "experimental/runnables";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunnablesParams {
    pub text_document: lsp::TextDocumentIdentifier,
    pub position: Option<lsp::Position>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Runnable {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<lsp::LocationLink>,
    pub kind: RunnableKind,
    pub args: RunnableArgs,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum RunnableArgs {
    Cargo(CargoRunnableArgs),
    Shell(ShellRunnableArgs),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RunnableKind {
    Cargo,
    Shell,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CargoRunnableArgs {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub environment: HashMap<String, String>,
    pub cwd: PathBuf,
    /// Command to be executed instead of cargo
    pub override_cargo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_root: Option<PathBuf>,
    // command, --package and --lib stuff
    pub cargo_args: Vec<String>,
    // stuff after --
    pub executable_args: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShellRunnableArgs {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub environment: HashMap<String, String>,
    pub cwd: PathBuf,
    pub program: String,
    pub args: Vec<String>,
}

pub fn register_notifications(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    let name = language_server.name();
    let server_id = language_server.server_id();

    let this = lsp_store;

    language_server
        .on_notification::<ServerStatus, _>({
            let name = name.to_string();
            move |params, cx| {
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

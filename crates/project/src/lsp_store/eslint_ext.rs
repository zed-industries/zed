use lsp::{LanguageServer, LanguageServerName};

/// Use the same static name as `EsLintLspAdapter` in the `languages` crate (`"eslint"`).
pub const ESLINT_SERVER_NAME: LanguageServerName = LanguageServerName::new_static("eslint");

/// Custom notification from the vscode-eslint language server; used for UI status
/// in VS Code. Zed has no use for the payload, but we must register a handler or
/// every message is treated as unhandled and logged.
pub struct EslintStatus;

impl lsp::notification::Notification for EslintStatus {
    type Params = serde_json::Value;
    const METHOD: &'static str = "eslint/status";
}

pub fn register_notifications(language_server: &LanguageServer) {
    if language_server.name() != ESLINT_SERVER_NAME {
        return;
    }

    language_server
        .on_notification::<EslintStatus, _>(|_params, _cx| {})
        .detach();
}

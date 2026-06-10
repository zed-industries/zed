use gpui_shared_string::SharedString;
use serde::{Deserialize, Serialize};

/// Converts a value into an LSP position.
pub trait ToLspPosition {
    /// Converts the value into an LSP position.
    fn to_lsp_position(self) -> lsp::Position;
}

/// Context provided to LSP adapters when a user responds to a ShowMessageRequest prompt.
/// This allows adapters to intercept preference selections (like "Always" or "Never")
/// and potentially persist them to Zed's settings.
#[derive(Debug, Clone)]
pub struct PromptResponseContext {
    /// The original message shown to the user
    pub message: String,
    /// The action (button) the user selected
    pub selected_action: lsp::MessageActionItem,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanguageServerStatusUpdate {
    Binary(BinaryStatus),
    Health(ServerHealth, Option<SharedString>),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ServerHealth {
    Ok,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryStatus {
    None,
    CheckingForUpdate,
    Downloading,
    Starting,
    Stopping,
    Stopped,
    Failed { error: String },
}

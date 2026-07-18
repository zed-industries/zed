use gpui_shared_string::SharedString;
use serde::{Deserialize, Serialize};

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

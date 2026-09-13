use std::sync::Arc;

use anyhow::Result;
use gpui::{App, Task};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The dock in which an extension-owned panel is displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionPanelLocation {
    Right,
    Bottom,
}

/// Identifies one persistent panel owned by an extension.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExtensionPanelId {
    pub extension_id: Arc<str>,
    pub panel_id: Arc<str>,
}

/// Describes the persistent shell Zed creates for an extension panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPanelDescriptor {
    pub id: ExtensionPanelId,
    pub title: String,
    pub location: ExtensionPanelLocation,
    pub actions: Vec<ExtensionPanelActionDescriptor>,
}

/// A button rendered by Zed in an extension-owned panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPanelActionDescriptor {
    pub id: Arc<str>,
    pub label: String,
    /// Whether Zed should collect a single line of text and include it in the
    /// action's JSON payload as `input`.
    pub requires_input: bool,
}

/// A structured event sent by an extension to one of its panels.
///
/// `kind` is a stable event name owned by the extension; `payload` must be a
/// JSON object so consumers can evolve it without parsing presentation text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionPanelEvent {
    pub kind: Arc<str>,
    pub payload: Value,
}

/// A structured action generated in an extension panel and delivered to the
/// extension host.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionPanelAction {
    pub panel: ExtensionPanelId,
    pub action: Arc<str>,
    pub payload: Value,
}

/// Implemented by the application UI to host persistent extension panels.
pub trait ExtensionPanelUiProxy: Send + Sync + 'static {
    fn open_panel(&self, descriptor: ExtensionPanelDescriptor, cx: &mut App) -> Result<()>;
    fn send_panel_event(
        &self,
        panel: ExtensionPanelId,
        event: ExtensionPanelEvent,
        cx: &mut App,
    ) -> Result<()>;

    /// Returns the root path of the first visible worktree in the active
    /// workspace. This intentionally exposes no arbitrary file access.
    fn active_worktree_root(&self, cx: &mut App) -> Result<String>;

    /// Reads one relative file from the active workspace's first visible
    /// worktree. Implementations must reject absolute and traversing paths.
    fn read_active_worktree_file(&self, path: &str, cx: &mut App) -> Result<String>;
}

/// Implemented by the extension host to deliver user actions to the owning
/// extension. A UI must not execute extension-provided code directly.
pub trait ExtensionPanelActionProxy: Send + Sync + 'static {
    fn dispatch_panel_action(&self, action: ExtensionPanelAction, cx: &mut App)
    -> Task<Result<()>>;
}

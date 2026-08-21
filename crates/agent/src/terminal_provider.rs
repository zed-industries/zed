use std::sync::Arc;

use gpui::{App, AppContext, Global};
use serde::{Deserialize, Serialize};

/// A terminal the agent can reference, returned by
/// [`TerminalContentProvider::list_terminals`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalSummary {
    /// Opaque, stable terminal id (a UUID string). Internal-only; never shown
    /// to the user.
    pub id: String,
    /// Human-facing label (the terminal's title), shown in the UI.
    pub title: String,
    /// Current working directory of the terminal, if known.
    pub cwd: Option<String>,
}

/// Provides access to the user's open terminals so the agent can read their
/// contents. Implemented in `agent_ui` (which has workspace access) and
/// registered as a gpui global; the `agent` crate consumes it without taking a
/// dependency on the UI.
pub trait TerminalContentProvider: Send + Sync {
    /// Returns the current buffer of the terminal with the given id, optionally
    /// limited to the first `head` / last `tail` lines. Returns `None` when no
    /// terminal with that id is open.
    fn read_terminal(&self, id: &str, head: Option<u32>, tail: Option<u32>, cx: &App)
    -> Option<String>;

    /// Lists the terminals currently open in the workspace.
    fn list_terminals(&self, cx: &App) -> Vec<TerminalSummary>;
}

struct GlobalTerminalContentProvider(Arc<dyn TerminalContentProvider>);

impl Global for GlobalTerminalContentProvider {}

/// Registers the workspace-backed implementation of [`TerminalContentProvider`].
pub fn set_terminal_content_provider(provider: Arc<dyn TerminalContentProvider>, cx: &mut App) {
    cx.set_global(GlobalTerminalContentProvider(provider));
}

/// Returns the current buffer of the terminal with the given id, if a provider
/// is registered and such a terminal is open.
pub fn read_terminal(
    cx: &App,
    id: &str,
    head: Option<u32>,
    tail: Option<u32>,
) -> Option<String> {
    if !cx.has_global::<GlobalTerminalContentProvider>() {
        return None;
    }
    cx.read_global::<GlobalTerminalContentProvider, _>(|global, app| {
        global.0.read_terminal(id, head, tail, app)
    })
}

/// Lists the terminals currently open in the workspace.
pub fn list_terminals(cx: &App) -> Vec<TerminalSummary> {
    if !cx.has_global::<GlobalTerminalContentProvider>() {
        return Vec::new();
    }
    cx.read_global::<GlobalTerminalContentProvider, _>(|global, app| global.0.list_terminals(app))
}

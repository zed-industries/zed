mod context_server;
mod dap;
mod lsp;
mod slash_command;

use std::{ops::Range, path::PathBuf};

use util::redact::should_redact;

pub use context_server::*;
pub use dap::*;
pub use lsp::*;
pub use slash_command::*;

/// A list of environment variables.
pub type EnvVars = Vec<(String, String)>;

/// A command.
pub struct Command {
    /// The command to execute.
    pub command: PathBuf,
    /// The arguments to pass to the command.
    pub args: Vec<String>,
    /// The environment variables to set for the command.
    pub env: EnvVars,
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filtered_env = self
            .env
            .iter()
            .map(|(k, v)| (k, if should_redact(k) { "[REDACTED]" } else { v }))
            .collect::<Vec<_>>();

        f.debug_struct("Command")
            .field("command", &self.command)
            .field("args", &self.args)
            .field("env", &filtered_env)
            .finish()
    }
}

/// A label containing some code.
#[derive(Debug, Clone)]
pub struct CodeLabel {
    /// The source code to parse with Tree-sitter.
    pub code: String,
    /// The spans to display in the label.
    pub spans: Vec<CodeLabelSpan>,
    /// The range of the displayed label to include when filtering.
    pub filter_range: Range<usize>,
}

/// A span within a code label.
#[derive(Debug, Clone)]
pub enum CodeLabelSpan {
    /// A range into the parsed code.
    CodeRange(Range<usize>),
    /// A span containing a code literal.
    Literal(CodeLabelSpanLiteral),
}

/// A span containing a code literal.
#[derive(Debug, Clone)]
pub struct CodeLabelSpanLiteral {
    /// The literal text.
    pub text: String,
    /// The name of the highlight to use for this literal.
    pub highlight_name: Option<String>,
}

/// A handle to a terminal, represented as its GPUI entity ID (u64).
/// This is intrinsic to Zed and stable for the lifetime of the terminal.
pub type TerminalHandle = u64;

/// Options for creating a terminal.
#[derive(Debug, Clone)]
pub struct TerminalOptions {
    /// The working directory for the terminal.
    pub cwd: Option<PathBuf>,
    /// Environment variables to set.
    pub env: EnvVars,
    /// The command to run instead of the default shell.
    pub command: Option<String>,
    /// Arguments for the command.
    pub args: Vec<String>,
    /// Optional title override for the terminal tab.
    pub title_override: Option<String>,
    /// If set, create the terminal in the pane containing this terminal (by entity_id).
    pub in_pane_of: Option<TerminalHandle>,
    /// Whether to activate (focus) the new terminal. Defaults to true.
    pub activate: bool,
}

impl Default for TerminalOptions {
    fn default() -> Self {
        Self {
            cwd: None,
            env: Vec::new(),
            command: None,
            args: Vec::new(),
            title_override: None,
            in_pane_of: None,
            activate: true,
        }
    }
}

/// Information about a terminal.
#[derive(Debug, Clone)]
pub struct TerminalInfo {
    /// The GPUI entity ID for this terminal (stable identifier).
    pub entity_id: TerminalHandle,
    /// The computed title (what appears in the tab).
    pub title: String,
    /// The user-set title override, if any.
    pub title_override: Option<String>,
    /// Whether this terminal is the active item in its pane.
    pub is_active: bool,
}

/// Information about a workspace containing terminals.
#[derive(Debug, Clone)]
pub struct WorkspaceTerminals {
    /// A stable identifier for this workspace window.
    pub workspace_id: u64,
    /// The display name of this workspace (usually the project root folder name).
    pub name: String,
    /// The terminals in this workspace.
    pub terminals: Vec<TerminalInfo>,
}

/// The content of a terminal screen.
#[derive(Debug, Clone)]
pub struct TerminalContent {
    /// The screen text, line by line.
    pub lines: Vec<String>,
    /// The cursor row position (0-indexed).
    pub cursor_row: u32,
    /// The cursor column position (0-indexed).
    pub cursor_col: u32,
}

/// Direction to split a terminal pane.
#[derive(Debug, Clone, Copy)]
pub enum SplitDirection {
    /// Split up (new pane above).
    Up,
    /// Split down (new pane below).
    Down,
    /// Split left (new pane to the left).
    Left,
    /// Split right (new pane to the right).
    Right,
}

/// Bounding box for a pane in pixel coordinates.
#[derive(Debug, Clone)]
pub struct PaneBounds {
    /// X position from left edge.
    pub x: f32,
    /// Y position from top edge.
    pub y: f32,
    /// Width in pixels.
    pub width: f32,
    /// Height in pixels.
    pub height: f32,
}

/// Information about a terminal within a pane.
#[derive(Debug, Clone)]
pub struct PaneTerminalInfo {
    /// The GPUI entity ID for this terminal (stable identifier).
    pub entity_id: TerminalHandle,
    /// The computed title (what appears in the tab).
    pub title: String,
    /// The user-set title override, if any.
    pub title_override: Option<String>,
    /// Whether this terminal is the active item in its pane.
    pub is_active: bool,
}

/// A member of a pane layout (either a pane with terminals or an axis containing splits).
#[derive(Debug, Clone)]
pub enum PaneLayoutMember {
    /// A pane containing terminals.
    Pane {
        /// A stable identifier for this pane for direct reference.
        pane_id: Option<String>,
        /// The terminals in this pane.
        terminals: Vec<PaneTerminalInfo>,
        /// The bounding box of this pane.
        bounds: Option<PaneBounds>,
    },
    /// An axis containing split panes.
    Axis {
        /// The axis direction (horizontal or vertical).
        axis: AxisDirection,
        /// The child members.
        members: Vec<PaneLayoutMember>,
    },
}

/// The direction of a pane axis.
#[derive(Debug, Clone, Copy)]
pub enum AxisDirection {
    /// Horizontal axis (children are arranged left to right).
    Horizontal,
    /// Vertical axis (children are arranged top to bottom).
    Vertical,
}

/// The complete layout tree of the terminal panel.
#[derive(Debug, Clone)]
pub struct PaneLayout {
    /// The outer bounding box of the entire terminal panel.
    pub panel_bounds: Option<PaneBounds>,
    /// The root member of the layout tree.
    pub root: PaneLayoutMember,
}

pub use cli::LayoutMode;

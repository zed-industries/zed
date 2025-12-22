use collections::HashMap;
pub use ipc_channel::ipc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IpcHandshake {
    pub requests: ipc::IpcSender<CliRequest>,
    pub responses: ipc::IpcReceiver<CliResponse>,
}

/// Layout mode for terminal pane reorganization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LayoutMode {
    /// Arrange all terminals side-by-side in a horizontal row.
    TileVertical,
    /// Stack all terminals vertically in a column.
    TileHorizontal,
    /// Consolidate all terminals into a single pane as tabs.
    Consolidate,
}

/// Commands for terminal management via CLI.
#[derive(Debug, Serialize, Deserialize)]
pub enum TerminalCommand {
    /// Create a new terminal in the active workspace.
    Create {
        cwd: Option<String>,
        command: Option<String>,
        args: Vec<String>,
        env: Vec<(String, String)>,
        /// Optional title override for the terminal tab.
        title: Option<String>,
        /// Optional terminal entity_id whose pane should be used for the new terminal.
        in_pane_of: Option<String>,
        /// Whether to activate the new terminal (make it the active tab). Defaults to true.
        activate: bool,
    },
    /// Send text input to a terminal (by entity_id or title).
    Send { terminal: String, text: String },
    /// Send a special key to a terminal (by entity_id or title).
    Key { terminal: String, key: String },
    /// Read the current screen content of a terminal (by entity_id or title).
    Read { terminal: String },
    /// List all terminals with their entity_id, title, and title_override.
    List,
    /// Get the current working directory of a terminal (by entity_id or title).
    Cwd { terminal: String },
    /// Check if a terminal is idle (by entity_id or title).
    Idle { terminal: String },
    /// Close a terminal (by entity_id or title).
    Close { terminal: String },
    /// Split a terminal pane in a given direction.
    Split {
        terminal: String,
        direction: String,
        /// Optional title override for the new terminal tab.
        title: Option<String>,
    },
    /// Get the full layout tree of the terminal panel with bounding boxes,
    /// or reorganize terminals into a specific layout mode.
    Layout {
        /// Optional layout mode to apply.
        mode: Option<LayoutMode>,
    },
    /// Focus a specific terminal pane.
    Focus { terminal: String },
    /// Set the title override for a terminal tab.
    Title {
        terminal: String,
        /// The title to set, or None to clear the override.
        title: Option<String>,
    },
    /// Move a terminal to the pane of another terminal.
    Move {
        terminal: String,
        to_pane_of: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliRequest {
    Open {
        paths: Vec<String>,
        urls: Vec<String>,
        diff_paths: Vec<[String; 2]>,
        diff_all: bool,
        wsl: Option<String>,
        wait: bool,
        open_new_workspace: Option<bool>,
        reuse: bool,
        env: Option<HashMap<String, String>>,
        user_data_dir: Option<String>,
    },
    /// Terminal management commands.
    Terminal {
        command: TerminalCommand,
        /// The entity_id of the terminal that initiated this command (from ZED_TERM_ID).
        /// Available to all command handlers for caller-aware behavior.
        caller_terminal_id: Option<u64>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliResponse {
    Ping,
    Stdout { message: String },
    Stderr { message: String },
    Exit { status: i32 },
}

/// When Zed started not as an *.app but as a binary (e.g. local development),
/// there's a possibility to tell it to behave "regularly".
pub const FORCE_CLI_MODE_ENV_VAR_NAME: &str = "ZED_FORCE_CLI_MODE";

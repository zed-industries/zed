use dap_types::SteppingGranularity;
use gpui::{App, Global};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsUi};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, SettingsUi)]
#[serde(rename_all = "snake_case")]
pub enum DebugPanelDockPosition {
    Left,
    Bottom,
    Right,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy, SettingsUi)]
#[serde(default)]
#[settings_ui(group = "Debugger", path = "debugger")]
pub struct DebuggerSettings {
    /// Determines the stepping granularity.
    ///
    /// Default: line
    #[settings_ui(skip)]
    pub stepping_granularity: SteppingGranularity,
    /// Whether the breakpoints should be reused across Zed sessions.
    ///
    /// Default: true
    pub save_breakpoints: bool,
    /// Whether to show the debug button in the status bar.
    ///
    /// Default: true
    pub button: bool,
    /// Time in milliseconds until timeout error when connecting to a TCP debug adapter
    ///
    /// Default: 2000ms
    pub timeout: u64,
    /// Whether to log messages between active debug adapters and Zed
    ///
    /// Default: true
    pub log_dap_communications: bool,
    /// Whether to format dap messages in when adding them to debug adapter logger
    ///
    /// Default: true
    pub format_dap_log_messages: bool,
    /// The dock position of the debug panel
    ///
    /// Default: Bottom
    pub dock: DebugPanelDockPosition,
}

impl Default for DebuggerSettings {
    fn default() -> Self {
        Self {
            button: true,
            save_breakpoints: true,
            stepping_granularity: SteppingGranularity::Line,
            timeout: 2000,
            log_dap_communications: true,
            format_dap_log_messages: true,
            dock: DebugPanelDockPosition::Bottom,
        }
    }
}

impl Settings for DebuggerSettings {
    const KEY: Option<&'static str> = Some("debugger");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

impl Global for DebuggerSettings {}

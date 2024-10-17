use dap_types::SteppingGranularity;
use gpui::{AppContext, Global};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(default)]
pub struct DebuggerSettings {
    /// Determines the stepping granularity.
    ///
    /// Default: line
    pub stepping_granularity: SteppingGranularity,
    /// Whether the breakpoints should be reused across Zed sessions.
    ///
    /// Default: true
    pub save_breakpoints: bool,
    /// Whether to show the debug button in the status bar.
    ///
    /// Default: true
    pub button: bool,
}

impl Default for DebuggerSettings {
    fn default() -> Self {
        Self {
            button: true,
            save_breakpoints: true,
            stepping_granularity: SteppingGranularity::Line,
        }
    }
}

impl Settings for DebuggerSettings {
    const KEY: Option<&'static str> = Some("debugger");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

impl Global for DebuggerSettings {}

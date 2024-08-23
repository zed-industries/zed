use gpui::{AppContext, Global};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Clone, Debug)]
pub struct DebuggerSettings {
    pub save_breakpoints: bool,
    pub button: bool,
}

#[derive(Default, Serialize, Deserialize, JsonSchema, Clone)]
pub struct DebuggerSettingsContent {
    /// Whether the breakpoints should be reused across Zed sessions.
    ///
    /// Default: true
    pub save_breakpoints: Option<bool>,
    /// Whether to show the debug button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
}

impl Settings for DebuggerSettings {
    const KEY: Option<&'static str> = Some("debugger");

    type FileContent = DebuggerSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

impl Global for DebuggerSettings {}

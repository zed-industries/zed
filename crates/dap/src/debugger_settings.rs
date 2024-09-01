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

#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SteppingGranularity {
    /// The step should allow the program to run until the current statement has finished executing.
    /// The meaning of a statement is determined by the adapter and it may be considered equivalent to a line.
    /// For example 'for(int i = 0; i < 10; i++)' could be considered to have 3 statements 'int i = 0', 'i < 10', and 'i++'.
    Statement,
    /// The step should allow the program to run until the current source line has executed.
    Line,
    /// The step should allow one instruction to execute (e.g. one x86 instruction).
    Instruction,
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

impl DebuggerSettings {
    pub fn stepping_granularity(&self) -> dap_types::SteppingGranularity {
        match &self.stepping_granularity {
            SteppingGranularity::Statement => dap_types::SteppingGranularity::Statement,
            SteppingGranularity::Line => dap_types::SteppingGranularity::Line,
            SteppingGranularity::Instruction => dap_types::SteppingGranularity::Instruction,
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

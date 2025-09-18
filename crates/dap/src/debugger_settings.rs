use dap_types::SteppingGranularity;
use gpui::App;
use settings::{Settings, SettingsContent};
use util::MergeFrom;

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
    pub dock: settings::DockPosition,
}

impl Settings for DebuggerSettings {
    fn from_settings(content: &SettingsContent, _cx: &mut App) -> Self {
        let content = content.debugger.clone().unwrap();
        Self {
            stepping_granularity: dap_granularity_from_settings(
                content.stepping_granularity.unwrap(),
            ),
            save_breakpoints: content.save_breakpoints.unwrap(),
            button: content.button.unwrap(),
            timeout: content.timeout.unwrap(),
            log_dap_communications: content.log_dap_communications.unwrap(),
            format_dap_log_messages: content.format_dap_log_messages.unwrap(),
            dock: content.dock.unwrap(),
        }
    }

    fn refine(&mut self, content: &SettingsContent, _cx: &mut App) {
        let Some(content) = &content.debugger else {
            return;
        };
        self.stepping_granularity.merge_from(
            &content
                .stepping_granularity
                .map(dap_granularity_from_settings),
        );
        self.save_breakpoints.merge_from(&content.save_breakpoints);
        self.button.merge_from(&content.button);
        self.timeout.merge_from(&content.timeout);
        self.log_dap_communications
            .merge_from(&content.log_dap_communications);
        self.format_dap_log_messages
            .merge_from(&content.format_dap_log_messages);
        self.dock.merge_from(&content.dock);
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut SettingsContent) {}
}

fn dap_granularity_from_settings(
    granularity: settings::SteppingGranularity,
) -> dap_types::SteppingGranularity {
    match granularity {
        settings::SteppingGranularity::Instruction => dap_types::SteppingGranularity::Instruction,
        settings::SteppingGranularity::Line => dap_types::SteppingGranularity::Line,
        settings::SteppingGranularity::Statement => dap_types::SteppingGranularity::Statement,
    }
}

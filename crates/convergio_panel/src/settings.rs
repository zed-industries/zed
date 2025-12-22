//! Settings for the Convergio Panel

use gpui::{App, Pixels};
use serde::Deserialize;
use settings::{
    ConvergioAgentPack, ConvergioEdition, DockPosition,
    PollingInterval, RegisterSetting, Settings,
};
use ui::px;

/// Settings for the Convergio Panel
#[derive(Deserialize, Debug, Clone, PartialEq, RegisterSetting)]
pub struct ConvergioSettings {
    /// Whether the Convergio panel is enabled
    pub enabled: bool,
    /// Whether to show the Convergio button in the status bar
    pub button: bool,
    /// Where to dock the Convergio panel
    pub dock: DockPosition,
    /// Default width in pixels when the panel is docked
    pub default_width: Pixels,
    /// The Convergio edition to use
    pub edition: ConvergioEdition,
    /// Which agent pack to use
    pub agent_pack: ConvergioAgentPack,
    /// Custom list of agent names when using Custom pack
    pub custom_agents: Vec<String>,
    /// How often to poll for new messages
    pub polling_interval: PollingInterval,
    /// Show token usage in chat messages
    pub show_token_usage: bool,
    /// Show cost information in chat messages
    pub show_cost: bool,
    /// Show message timestamps
    pub show_timestamps: bool,
    /// Maximum width of chat messages in rem units
    pub message_max_width: f32,
    /// Play sound when agent responds
    pub play_sound_on_response: bool,
    /// Show notification when agent responds while Zed is not focused
    pub notify_on_response: bool,
    /// Whether to show categories in the agent list
    pub show_categories: bool,
    /// Whether categories are collapsed by default
    pub collapse_categories: bool,
    /// Whether to show agent descriptions in the list
    pub show_agent_descriptions: bool,
}

impl Settings for ConvergioSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let convergio = content.convergio.clone().unwrap_or_default();
        Self {
            enabled: convergio.enabled.unwrap_or(true),
            button: convergio.button.unwrap_or(true),
            dock: convergio.dock.unwrap_or(DockPosition::Left),
            default_width: px(convergio.default_width.unwrap_or(260.0)),
            edition: convergio.edition.unwrap_or_default(),
            agent_pack: convergio.agent_pack.unwrap_or_default(),
            custom_agents: convergio
                .custom_agents
                .iter()
                .map(|s| s.to_string())
                .collect(),
            polling_interval: convergio.polling_interval.unwrap_or_default(),
            show_token_usage: convergio.show_token_usage.unwrap_or(true),
            show_cost: convergio.show_cost.unwrap_or(true),
            show_timestamps: convergio.show_timestamps.unwrap_or(true),
            message_max_width: convergio.message_max_width.unwrap_or(40.0),
            play_sound_on_response: convergio.play_sound_on_response.unwrap_or(false),
            notify_on_response: convergio.notify_on_response.unwrap_or(true),
            show_categories: convergio.show_categories.unwrap_or(true),
            collapse_categories: convergio.collapse_categories.unwrap_or(false),
            show_agent_descriptions: convergio.show_agent_descriptions.unwrap_or(true),
        }
    }
}

impl ConvergioSettings {
    /// Get the current edition from compile-time feature flags
    pub fn detect_edition() -> ConvergioEdition {
        #[cfg(feature = "convergio-education")]
        return ConvergioEdition::Education;

        #[cfg(feature = "convergio-enterprise")]
        return ConvergioEdition::Enterprise;

        #[cfg(not(any(feature = "convergio-education", feature = "convergio-enterprise")))]
        ConvergioEdition::Base
    }

    /// Returns the edition display name
    pub fn edition_display_name(&self) -> &'static str {
        match self.edition {
            ConvergioEdition::Base => "Convergio Studio",
            ConvergioEdition::Education => "Convergio Studio Edu",
            ConvergioEdition::Enterprise => "Convergio Studio Enterprise",
        }
    }

    /// Returns the polling interval in milliseconds, if polling is enabled
    pub fn polling_millis(&self) -> Option<u64> {
        self.polling_interval.to_millis()
    }
}

pub fn init(cx: &mut App) {
    ConvergioSettings::register(cx);
}

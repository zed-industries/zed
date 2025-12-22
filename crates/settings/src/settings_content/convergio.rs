//! Convergio Panel Settings Content
//!
//! This module defines the settings schema for the Convergio panel,
//! which integrates AI agents from MyConvergio into Zed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};
use std::sync::Arc;

use crate::DockPosition;

/// Which agent pack to use by default
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum ConvergioAgentPack {
    /// All 54 agents - full enterprise experience
    #[default]
    Enterprise,
    /// Core agents for startups: Ali, Baccio, Dario, Rex, Amy, Marcello
    Startup,
    /// Developer-focused: Ali, Baccio, Dario, Rex, Paolo, Guardian
    Developer,
    /// Education-focused: accessibility, coaching, and learning agents
    Education,
    /// Minimal: Just Ali as your AI Chief of Staff
    Minimal,
    /// Custom selection using custom_agents list
    Custom,
}

/// Convergio edition determines available features
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum ConvergioEdition {
    /// Base edition with all 54 agents
    #[default]
    Base,
    /// Education edition with accessibility focus
    Education,
    /// Enterprise edition with custom agent sets
    Enterprise,
}

/// Message polling interval options
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum PollingInterval {
    /// Poll every 500ms for near real-time updates
    Fast,
    /// Poll every 2 seconds (default)
    #[default]
    Normal,
    /// Poll every 5 seconds for reduced resource usage
    Slow,
    /// Disable automatic polling
    Manual,
}

impl PollingInterval {
    pub fn to_millis(&self) -> Option<u64> {
        match self {
            Self::Fast => Some(500),
            Self::Normal => Some(2000),
            Self::Slow => Some(5000),
            Self::Manual => None,
        }
    }
}

#[with_fallible_options]
#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, Default)]
pub struct ConvergioSettingsContent {
    /// Whether the Convergio panel is enabled.
    ///
    /// Default: true
    pub enabled: Option<bool>,

    /// Whether to show the Convergio button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,

    /// Where to dock the Convergio panel.
    ///
    /// Default: left
    pub dock: Option<DockPosition>,

    /// Default width in pixels when the panel is docked.
    ///
    /// Default: 260
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_width: Option<f32>,

    /// The Convergio edition to use.
    ///
    /// Default: base
    pub edition: Option<ConvergioEdition>,

    /// Which agent pack to use.
    ///
    /// Default: enterprise
    pub agent_pack: Option<ConvergioAgentPack>,

    /// Custom list of agent names when using Custom pack.
    /// Example: ["ali", "baccio-tech-architect", "dario-debugger"]
    #[serde(default)]
    pub custom_agents: Vec<Arc<str>>,

    /// How often to poll for new messages.
    ///
    /// Default: normal (2 seconds)
    pub polling_interval: Option<PollingInterval>,

    /// Show token usage in chat messages.
    ///
    /// Default: true
    pub show_token_usage: Option<bool>,

    /// Show cost information in chat messages.
    ///
    /// Default: true
    pub show_cost: Option<bool>,

    /// Show message timestamps.
    ///
    /// Default: true
    pub show_timestamps: Option<bool>,

    /// Maximum width of chat messages in rem units.
    ///
    /// Default: 40
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub message_max_width: Option<f32>,

    /// Play sound when agent responds.
    ///
    /// Default: false
    pub play_sound_on_response: Option<bool>,

    /// Show notification when agent responds while Zed is not focused.
    ///
    /// Default: true
    pub notify_on_response: Option<bool>,

    /// Custom path to the Convergio database.
    /// If not set, uses the default location.
    pub database_path: Option<Arc<str>>,

    /// Whether to show categories in the agent list.
    ///
    /// Default: true
    pub show_categories: Option<bool>,

    /// Whether categories are collapsed by default.
    ///
    /// Default: false
    pub collapse_categories: Option<bool>,

    /// Whether to show agent descriptions in the list.
    ///
    /// Default: true
    pub show_agent_descriptions: Option<bool>,
}

impl ConvergioSettingsContent {
    pub fn set_dock(&mut self, dock: DockPosition) {
        self.dock = Some(dock);
    }

    pub fn set_agent_pack(&mut self, pack: ConvergioAgentPack) {
        self.agent_pack = Some(pack);
    }

    pub fn set_polling_interval(&mut self, interval: PollingInterval) {
        self.polling_interval = Some(interval);
    }
}

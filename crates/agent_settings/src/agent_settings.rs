mod agent_profile;

use std::sync::Arc;

use collections::IndexMap;
use gpui::{App, Pixels, px};
use language_model::LanguageModel;
use project::DisableAiSettings;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DefaultAgentView, DockPosition, LanguageModelParameters, LanguageModelSelection,
    NotifyWhenAgentWaiting, Settings,
};

pub use crate::agent_profile::*;

pub const SUMMARIZE_THREAD_PROMPT: &str = include_str!("prompts/summarize_thread_prompt.txt");
pub const SUMMARIZE_THREAD_DETAILED_PROMPT: &str =
    include_str!("prompts/summarize_thread_detailed_prompt.txt");

pub fn init(cx: &mut App) {
    AgentSettings::register(cx);
}

#[derive(Clone, Debug)]
pub struct AgentQuickAction {
    pub id: Arc<str>,
    pub title: Arc<str>,
    pub prompt: Arc<str>,
    pub description: Option<Arc<str>>,
    pub auto_send: bool,
}

impl AgentQuickAction {
    pub fn new(
        id: impl Into<Arc<str>>,
        title: impl Into<Arc<str>>,
        prompt: impl Into<Arc<str>>,
        description: Option<Arc<str>>,
        auto_send: bool,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            prompt: prompt.into(),
            description,
            auto_send,
        }
    }
}

fn default_quick_actions() -> Vec<AgentQuickAction> {
    vec![
        AgentQuickAction::new(
            "explain_selection",
            "Explain Selection",
            "Explain what the selected code does.",
            Some("Describe the highlighted code in plain language.".into()),
            true,
        ),
        AgentQuickAction::new(
            "write_tests",
            "Write Tests",
            "Generate unit tests for the selected code. Prefer idiomatic libraries and keep them concise.",
            Some("Produce focused unit tests for the current selection.".into()),
            true,
        ),
        AgentQuickAction::new(
            "improve_performance",
            "Improve Performance",
            "Suggest performance improvements for the selected code. Explain the trade-offs.",
            Some("Look for faster or more efficient alternatives.".into()),
            true,
        ),
        AgentQuickAction::new(
            "summarize_file",
            "Summarize File",
            "Summarize the main responsibilities and key flows of this file.",
            Some("Capture the high-level purpose of the current file.".into()),
            false,
        ),
    ]
}

#[derive(Clone, Debug)]
pub struct AgentSettings {
    pub enabled: bool,
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub default_model: Option<LanguageModelSelection>,
    pub inline_assistant_model: Option<LanguageModelSelection>,
    pub commit_message_model: Option<LanguageModelSelection>,
    pub thread_summary_model: Option<LanguageModelSelection>,
    pub inline_alternatives: Vec<LanguageModelSelection>,
    pub default_profile: AgentProfileId,
    pub default_view: DefaultAgentView,
    pub profiles: IndexMap<AgentProfileId, AgentProfileSettings>,
    pub always_allow_tool_actions: bool,
    pub notify_when_agent_waiting: NotifyWhenAgentWaiting,
    pub play_sound_when_agent_done: bool,
    pub single_file_review: bool,
    pub model_parameters: Vec<LanguageModelParameters>,
    pub preferred_completion_mode: CompletionMode,
    pub enable_feedback: bool,
    pub expand_edit_card: bool,
    pub expand_terminal_card: bool,
    pub use_modifier_to_send: bool,
    pub message_editor_min_lines: usize,
    pub quick_actions: Vec<AgentQuickAction>,
}

impl AgentSettings {
    pub fn enabled(&self, cx: &App) -> bool {
        self.enabled && !DisableAiSettings::get_global(cx).disable_ai
    }

    pub fn temperature_for_model(model: &Arc<dyn LanguageModel>, cx: &App) -> Option<f32> {
        let settings = Self::get_global(cx);
        for setting in settings.model_parameters.iter().rev() {
            if let Some(provider) = &setting.provider
                && provider.0 != model.provider_id().0
            {
                continue;
            }
            if let Some(setting_model) = &setting.model
                && *setting_model != model.id().0
            {
                continue;
            }
            return setting.temperature;
        }
        return None;
    }

    pub fn set_inline_assistant_model(&mut self, provider: String, model: String) {
        self.inline_assistant_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_commit_message_model(&mut self, provider: String, model: String) {
        self.commit_message_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_thread_summary_model(&mut self, provider: String, model: String) {
        self.thread_summary_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_message_editor_max_lines(&self) -> usize {
        self.message_editor_min_lines * 2
    }

    pub fn quick_action(&self, id: &str) -> Option<&AgentQuickAction> {
        self.quick_actions
            .iter()
            .find(|action| action.id.as_ref() == id)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CompletionMode {
    #[default]
    Normal,
    #[serde(alias = "max")]
    Burn,
}

impl From<CompletionMode> for cloud_llm_client::CompletionMode {
    fn from(value: CompletionMode) -> Self {
        match value {
            CompletionMode::Normal => cloud_llm_client::CompletionMode::Normal,
            CompletionMode::Burn => cloud_llm_client::CompletionMode::Max,
        }
    }
}

impl From<settings::CompletionMode> for CompletionMode {
    fn from(value: settings::CompletionMode) -> Self {
        match value {
            settings::CompletionMode::Normal => CompletionMode::Normal,
            settings::CompletionMode::Burn => CompletionMode::Burn,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentProfileId(pub Arc<str>);

impl AgentProfileId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AgentProfileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for AgentProfileId {
    fn default() -> Self {
        Self("write".into())
    }
}

impl Settings for AgentSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let agent = content.agent.clone().unwrap();
        Self {
            enabled: agent.enabled.unwrap(),
            button: agent.button.unwrap(),
            dock: agent.dock.unwrap(),
            default_width: px(agent.default_width.unwrap()),
            default_height: px(agent.default_height.unwrap()),
            default_model: Some(agent.default_model.unwrap()),
            inline_assistant_model: agent.inline_assistant_model,
            commit_message_model: agent.commit_message_model,
            thread_summary_model: agent.thread_summary_model,
            inline_alternatives: agent.inline_alternatives.unwrap_or_default(),
            default_profile: AgentProfileId(agent.default_profile.unwrap()),
            default_view: agent.default_view.unwrap(),
            profiles: agent
                .profiles
                .unwrap()
                .into_iter()
                .map(|(key, val)| (AgentProfileId(key), val.into()))
                .collect(),
            always_allow_tool_actions: agent.always_allow_tool_actions.unwrap(),
            notify_when_agent_waiting: agent.notify_when_agent_waiting.unwrap(),
            play_sound_when_agent_done: agent.play_sound_when_agent_done.unwrap(),
            single_file_review: agent.single_file_review.unwrap(),
            model_parameters: agent.model_parameters,
            preferred_completion_mode: agent.preferred_completion_mode.unwrap().into(),
            enable_feedback: agent.enable_feedback.unwrap(),
            expand_edit_card: agent.expand_edit_card.unwrap(),
            expand_terminal_card: agent.expand_terminal_card.unwrap(),
            use_modifier_to_send: agent.use_modifier_to_send.unwrap(),
            message_editor_min_lines: agent.message_editor_min_lines.unwrap(),
            quick_actions: default_quick_actions(),
        }
    }
}

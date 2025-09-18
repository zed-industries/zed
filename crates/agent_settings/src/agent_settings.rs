mod agent_profile;

use std::sync::Arc;

use collections::IndexMap;
use gpui::{App, Pixels, px};
use language_model::LanguageModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DefaultAgentView, DockPosition, LanguageModelParameters, LanguageModelSelection,
    NotifyWhenAgentWaiting, Settings, SettingsContent,
};
use util::MergeFrom;

pub use crate::agent_profile::*;

pub const SUMMARIZE_THREAD_PROMPT: &str =
    include_str!("../../agent/src/prompts/summarize_thread_prompt.txt");
pub const SUMMARIZE_THREAD_DETAILED_PROMPT: &str =
    include_str!("../../agent/src/prompts/summarize_thread_detailed_prompt.txt");

pub fn init(cx: &mut App) {
    AgentSettings::register(cx);
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
    pub stream_edits: bool,
    pub single_file_review: bool,
    pub model_parameters: Vec<LanguageModelParameters>,
    pub preferred_completion_mode: CompletionMode,
    pub enable_feedback: bool,
    pub expand_edit_card: bool,
    pub expand_terminal_card: bool,
    pub use_modifier_to_send: bool,
    pub message_editor_min_lines: usize,
}

impl AgentSettings {
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
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
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
            stream_edits: agent.stream_edits.unwrap(),
            single_file_review: agent.single_file_review.unwrap(),
            model_parameters: agent.model_parameters,
            preferred_completion_mode: agent.preferred_completion_mode.unwrap().into(),
            enable_feedback: agent.enable_feedback.unwrap(),
            expand_edit_card: agent.expand_edit_card.unwrap(),
            expand_terminal_card: agent.expand_terminal_card.unwrap(),
            use_modifier_to_send: agent.use_modifier_to_send.unwrap(),
            message_editor_min_lines: agent.message_editor_min_lines.unwrap(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, _: &mut App) {
        let Some(value) = &content.agent else { return };
        self.enabled.merge_from(&value.enabled);
        self.button.merge_from(&value.button);
        self.dock.merge_from(&value.dock);
        self.default_width
            .merge_from(&value.default_width.map(Into::into));
        self.default_height
            .merge_from(&value.default_height.map(Into::into));
        self.default_model = value.default_model.clone().or(self.default_model.take());

        self.inline_assistant_model = value
            .inline_assistant_model
            .clone()
            .or(self.inline_assistant_model.take());
        self.commit_message_model = value
            .clone()
            .commit_message_model
            .or(self.commit_message_model.take());
        self.thread_summary_model = value
            .clone()
            .thread_summary_model
            .or(self.thread_summary_model.take());
        self.inline_alternatives
            .merge_from(&value.inline_alternatives.clone());
        self.default_profile
            .merge_from(&value.default_profile.clone().map(AgentProfileId));
        self.default_view.merge_from(&value.default_view);
        self.always_allow_tool_actions
            .merge_from(&value.always_allow_tool_actions);
        self.notify_when_agent_waiting
            .merge_from(&value.notify_when_agent_waiting);
        self.play_sound_when_agent_done
            .merge_from(&value.play_sound_when_agent_done);
        self.stream_edits.merge_from(&value.stream_edits);
        self.single_file_review
            .merge_from(&value.single_file_review);
        self.preferred_completion_mode
            .merge_from(&value.preferred_completion_mode.map(Into::into));
        self.enable_feedback.merge_from(&value.enable_feedback);
        self.expand_edit_card.merge_from(&value.expand_edit_card);
        self.expand_terminal_card
            .merge_from(&value.expand_terminal_card);
        self.use_modifier_to_send
            .merge_from(&value.use_modifier_to_send);

        self.model_parameters
            .extend_from_slice(&value.model_parameters);
        self.message_editor_min_lines
            .merge_from(&value.message_editor_min_lines);

        if let Some(profiles) = value.profiles.as_ref() {
            self.profiles.extend(
                profiles
                    .into_iter()
                    .map(|(id, profile)| (AgentProfileId(id.clone()), profile.clone().into())),
            );
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut SettingsContent) {
        if let Some(b) = vscode
            .read_value("chat.agent.enabled")
            .and_then(|b| b.as_bool())
        {
            current.agent.get_or_insert_default().enabled = Some(b);
            current.agent.get_or_insert_default().button = Some(b);
        }
    }
}

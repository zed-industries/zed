mod agent_profile;

use std::sync::Arc;

use agent_client_protocol::ModelId;
use collections::{HashSet, IndexMap};
use gpui::{App, Pixels, px};
use language_model::LanguageModel;
use project::DisableAiSettings;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DefaultAgentView, DockPosition, DockSide, LanguageModelParameters, LanguageModelSelection,
    NotifyWhenAgentWaiting, RegisterSetting, Settings, ToolPermissionMode,
};

pub use crate::agent_profile::*;

pub const SUMMARIZE_THREAD_PROMPT: &str = include_str!("prompts/summarize_thread_prompt.txt");
pub const SUMMARIZE_THREAD_DETAILED_PROMPT: &str =
    include_str!("prompts/summarize_thread_detailed_prompt.txt");

#[derive(Clone, Debug, RegisterSetting)]
pub struct AgentSettings {
    pub enabled: bool,
    pub button: bool,
    pub dock: DockPosition,
    pub agents_panel_dock: DockSide,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub default_model: Option<LanguageModelSelection>,
    pub inline_assistant_model: Option<LanguageModelSelection>,
    pub inline_assistant_use_streaming_tools: bool,
    pub commit_message_model: Option<LanguageModelSelection>,
    pub thread_summary_model: Option<LanguageModelSelection>,
    pub inline_alternatives: Vec<LanguageModelSelection>,
    pub favorite_models: Vec<LanguageModelSelection>,
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
    pub tool_permissions: ToolPermissions,
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

    pub fn favorite_model_ids(&self) -> HashSet<ModelId> {
        self.favorite_models
            .iter()
            .map(|sel| ModelId::new(format!("{}/{}", sel.provider.0, sel.model)))
            .collect()
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

#[derive(Clone, Debug, Default)]
pub struct ToolPermissions {
    pub tools: collections::HashMap<Arc<str>, ToolRules>,
}

#[derive(Clone, Debug)]
pub struct ToolRules {
    pub default_mode: ToolPermissionMode,
    pub always_allow: Vec<CompiledRegex>,
    pub always_deny: Vec<CompiledRegex>,
    pub always_confirm: Vec<CompiledRegex>,
}

impl Default for ToolRules {
    fn default() -> Self {
        Self {
            default_mode: ToolPermissionMode::Confirm,
            always_allow: Vec::new(),
            always_deny: Vec::new(),
            always_confirm: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct CompiledRegex {
    pub pattern: String,
    pub regex: regex::Regex,
}

impl std::fmt::Debug for CompiledRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledRegex")
            .field("pattern", &self.pattern)
            .finish()
    }
}

impl CompiledRegex {
    pub fn new(pattern: &str, case_sensitive: bool) -> Option<Self> {
        let regex = regex::RegexBuilder::new(pattern)
            .case_insensitive(!case_sensitive)
            .build()
            .map_err(|e| {
                log::warn!("Invalid regex pattern '{}': {}", pattern, e);
                e
            })
            .ok()?;
        Some(Self {
            pattern: pattern.to_string(),
            regex,
        })
    }

    pub fn is_match(&self, input: &str) -> bool {
        self.regex.is_match(input)
    }
}

impl Settings for AgentSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let agent = content.agent.clone().unwrap();
        Self {
            enabled: agent.enabled.unwrap(),
            button: agent.button.unwrap(),
            dock: agent.dock.unwrap(),
            agents_panel_dock: agent.agents_panel_dock.unwrap(),
            default_width: px(agent.default_width.unwrap()),
            default_height: px(agent.default_height.unwrap()),
            default_model: Some(agent.default_model.unwrap()),
            inline_assistant_model: agent.inline_assistant_model,
            inline_assistant_use_streaming_tools: agent
                .inline_assistant_use_streaming_tools
                .unwrap_or(true),
            commit_message_model: agent.commit_message_model,
            thread_summary_model: agent.thread_summary_model,
            inline_alternatives: agent.inline_alternatives.unwrap_or_default(),
            favorite_models: agent.favorite_models,
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
            tool_permissions: compile_tool_permissions(agent.tool_permissions),
        }
    }
}

fn compile_tool_permissions(content: Option<settings::ToolPermissionsContent>) -> ToolPermissions {
    let Some(content) = content else {
        return ToolPermissions::default();
    };

    let tools = content
        .tools
        .into_iter()
        .map(|(tool_name, rules_content)| {
            let rules = ToolRules {
                default_mode: rules_content.default_mode.unwrap_or_default(),
                always_allow: rules_content
                    .always_allow
                    .map(|v| compile_regex_rules(v.0))
                    .unwrap_or_default(),
                always_deny: rules_content
                    .always_deny
                    .map(|v| compile_regex_rules(v.0))
                    .unwrap_or_default(),
                always_confirm: rules_content
                    .always_confirm
                    .map(|v| compile_regex_rules(v.0))
                    .unwrap_or_default(),
            };
            (tool_name, rules)
        })
        .collect();

    ToolPermissions { tools }
}

fn compile_regex_rules(rules: Vec<settings::ToolRegexRule>) -> Vec<CompiledRegex> {
    rules
        .into_iter()
        .filter_map(|rule| CompiledRegex::new(&rule.pattern, rule.case_sensitive.unwrap_or(false)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use settings::{ToolPermissionsContent, ToolRegexRule};

    #[test]
    fn test_compiled_regex_case_insensitive() {
        let regex = CompiledRegex::new("rm\\s+-rf", false).unwrap();
        assert!(regex.is_match("rm -rf /"));
        assert!(regex.is_match("RM -RF /"));
        assert!(regex.is_match("Rm -Rf /"));
    }

    #[test]
    fn test_compiled_regex_case_sensitive() {
        let regex = CompiledRegex::new("DROP\\s+TABLE", true).unwrap();
        assert!(regex.is_match("DROP TABLE users"));
        assert!(!regex.is_match("drop table users"));
    }

    #[test]
    fn test_invalid_regex_returns_none() {
        let result = CompiledRegex::new("[invalid(regex", false);
        assert!(result.is_none());
    }

    #[test]
    fn test_tool_permissions_parsing() {
        let json = json!({
            "tools": {
                "terminal": {
                    "default_mode": "allow",
                    "always_deny": [
                        { "pattern": "rm\\s+-rf" }
                    ],
                    "always_allow": [
                        { "pattern": "^git\\s" }
                    ]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        let terminal_rules = permissions.tools.get("terminal").unwrap();
        assert_eq!(terminal_rules.default_mode, ToolPermissionMode::Allow);
        assert_eq!(terminal_rules.always_deny.len(), 1);
        assert_eq!(terminal_rules.always_allow.len(), 1);
        assert!(terminal_rules.always_deny[0].is_match("rm -rf /"));
        assert!(terminal_rules.always_allow[0].is_match("git status"));
    }

    #[test]
    fn test_tool_rules_default_mode() {
        let json = json!({
            "tools": {
                "edit_file": {
                    "default_mode": "deny"
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        let rules = permissions.tools.get("edit_file").unwrap();
        assert_eq!(rules.default_mode, ToolPermissionMode::Deny);
    }

    #[test]
    fn test_tool_permissions_empty() {
        let permissions = compile_tool_permissions(None);
        assert!(permissions.tools.is_empty());
    }

    #[test]
    fn test_regex_rule_with_case_sensitivity() {
        let json = json!([
            { "pattern": "DELETE\\s+FROM", "case_sensitive": true },
            { "pattern": "select.*from", "case_sensitive": false }
        ]);

        let rules: Vec<ToolRegexRule> = serde_json::from_value(json).unwrap();
        let compiled = compile_regex_rules(rules);

        assert_eq!(compiled.len(), 2);
        assert!(compiled[0].is_match("DELETE FROM users"));
        assert!(!compiled[0].is_match("delete from users"));
        assert!(compiled[1].is_match("SELECT * FROM users"));
        assert!(compiled[1].is_match("select * from users"));
    }
}

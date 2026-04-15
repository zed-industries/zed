mod agent_profile;

use std::path::{Component, Path};
use std::sync::{Arc, LazyLock};

use agent_client_protocol::ModelId;
use collections::{HashSet, IndexMap};
use fs::Fs;
use futures::channel::oneshot;
use gpui::{App, Pixels, px};
use language_model::LanguageModel;
use project::DisableAiSettings;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DockPosition, DockSide, LanguageModelParameters, LanguageModelSelection, NewThreadLocation,
    NotifyWhenAgentWaiting, PlaySoundWhenAgentDone, RegisterSetting, Settings, SettingsContent,
    SettingsStore, SidebarDockPosition, SidebarSide, ThinkingBlockDisplay, ToolPermissionMode,
    update_settings_file, update_settings_file_with_completion,
};

pub use crate::agent_profile::*;

pub const SUMMARIZE_THREAD_PROMPT: &str = include_str!("prompts/summarize_thread_prompt.txt");
pub const SUMMARIZE_THREAD_DETAILED_PROMPT: &str =
    include_str!("prompts/summarize_thread_detailed_prompt.txt");

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PanelLayout {
    pub(crate) agent_dock: Option<DockPosition>,
    pub(crate) project_panel_dock: Option<DockSide>,
    pub(crate) outline_panel_dock: Option<DockSide>,
    pub(crate) collaboration_panel_dock: Option<DockPosition>,
    pub(crate) git_panel_dock: Option<DockPosition>,
}

impl PanelLayout {
    const AGENT: Self = Self {
        agent_dock: Some(DockPosition::Left),
        project_panel_dock: Some(DockSide::Right),
        outline_panel_dock: Some(DockSide::Right),
        collaboration_panel_dock: Some(DockPosition::Right),
        git_panel_dock: Some(DockPosition::Right),
    };

    const EDITOR: Self = Self {
        agent_dock: Some(DockPosition::Right),
        project_panel_dock: Some(DockSide::Left),
        outline_panel_dock: Some(DockSide::Left),
        collaboration_panel_dock: Some(DockPosition::Left),
        git_panel_dock: Some(DockPosition::Left),
    };

    pub fn is_agent_layout(&self) -> bool {
        *self == Self::AGENT
    }

    pub fn is_editor_layout(&self) -> bool {
        *self == Self::EDITOR
    }

    fn read_from(content: &SettingsContent) -> Self {
        Self {
            agent_dock: content.agent.as_ref().and_then(|a| a.dock),
            project_panel_dock: content.project_panel.as_ref().and_then(|p| p.dock),
            outline_panel_dock: content.outline_panel.as_ref().and_then(|p| p.dock),
            collaboration_panel_dock: content.collaboration_panel.as_ref().and_then(|p| p.dock),
            git_panel_dock: content.git_panel.as_ref().and_then(|p| p.dock),
        }
    }

    fn write_to(&self, settings: &mut SettingsContent) {
        settings.agent.get_or_insert_default().dock = self.agent_dock;
        settings.project_panel.get_or_insert_default().dock = self.project_panel_dock;
        settings.outline_panel.get_or_insert_default().dock = self.outline_panel_dock;
        settings.collaboration_panel.get_or_insert_default().dock = self.collaboration_panel_dock;
        settings.git_panel.get_or_insert_default().dock = self.git_panel_dock;
    }

    fn write_diff_to(&self, current_merged: &PanelLayout, settings: &mut SettingsContent) {
        if self.agent_dock != current_merged.agent_dock {
            settings.agent.get_or_insert_default().dock = self.agent_dock;
        }
        if self.project_panel_dock != current_merged.project_panel_dock {
            settings.project_panel.get_or_insert_default().dock = self.project_panel_dock;
        }
        if self.outline_panel_dock != current_merged.outline_panel_dock {
            settings.outline_panel.get_or_insert_default().dock = self.outline_panel_dock;
        }
        if self.collaboration_panel_dock != current_merged.collaboration_panel_dock {
            settings.collaboration_panel.get_or_insert_default().dock =
                self.collaboration_panel_dock;
        }
        if self.git_panel_dock != current_merged.git_panel_dock {
            settings.git_panel.get_or_insert_default().dock = self.git_panel_dock;
        }
    }

    fn backfill_to(&self, user_layout: &PanelLayout, settings: &mut SettingsContent) {
        if user_layout.agent_dock.is_none() {
            settings.agent.get_or_insert_default().dock = self.agent_dock;
        }
        if user_layout.project_panel_dock.is_none() {
            settings.project_panel.get_or_insert_default().dock = self.project_panel_dock;
        }
        if user_layout.outline_panel_dock.is_none() {
            settings.outline_panel.get_or_insert_default().dock = self.outline_panel_dock;
        }
        if user_layout.collaboration_panel_dock.is_none() {
            settings.collaboration_panel.get_or_insert_default().dock =
                self.collaboration_panel_dock;
        }
        if user_layout.git_panel_dock.is_none() {
            settings.git_panel.get_or_insert_default().dock = self.git_panel_dock;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowLayout {
    Editor(Option<PanelLayout>),
    Agent(Option<PanelLayout>),
    Custom(PanelLayout),
}

impl WindowLayout {
    pub fn agent() -> Self {
        Self::Agent(None)
    }

    pub fn editor() -> Self {
        Self::Editor(None)
    }
}

#[derive(Clone, Debug, RegisterSetting)]
pub struct AgentSettings {
    pub enabled: bool,
    pub button: bool,
    pub dock: DockPosition,
    pub flexible: bool,
    pub sidebar_side: SidebarDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub max_content_width: Pixels,
    pub default_model: Option<LanguageModelSelection>,
    pub inline_assistant_model: Option<LanguageModelSelection>,
    pub inline_assistant_use_streaming_tools: bool,
    pub commit_message_model: Option<LanguageModelSelection>,
    pub thread_summary_model: Option<LanguageModelSelection>,
    pub inline_alternatives: Vec<LanguageModelSelection>,
    pub favorite_models: Vec<LanguageModelSelection>,
    pub default_profile: AgentProfileId,
    pub profiles: IndexMap<AgentProfileId, AgentProfileSettings>,

    pub notify_when_agent_waiting: NotifyWhenAgentWaiting,
    pub play_sound_when_agent_done: PlaySoundWhenAgentDone,
    pub single_file_review: bool,
    pub model_parameters: Vec<LanguageModelParameters>,
    pub enable_feedback: bool,
    pub expand_edit_card: bool,
    pub expand_terminal_card: bool,
    pub thinking_display: ThinkingBlockDisplay,
    pub cancel_generation_on_terminal_stop: bool,
    pub use_modifier_to_send: bool,
    pub message_editor_min_lines: usize,
    pub show_turn_stats: bool,
    pub show_merge_conflict_indicator: bool,
    pub tool_permissions: ToolPermissions,
    pub new_thread_location: NewThreadLocation,
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

    pub fn sidebar_side(&self) -> SidebarSide {
        match self.sidebar_side {
            SidebarDockPosition::Left => SidebarSide::Left,
            SidebarDockPosition::Right => SidebarSide::Right,
        }
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

    pub fn get_layout(cx: &App) -> WindowLayout {
        let store = cx.global::<SettingsStore>();
        let merged = store.merged_settings();
        let user_layout = store
            .raw_user_settings()
            .map(|u| PanelLayout::read_from(u.content.as_ref()))
            .unwrap_or_default();
        let merged_layout = PanelLayout::read_from(merged);

        if merged_layout.is_agent_layout() {
            return WindowLayout::Agent(Some(user_layout));
        }

        if merged_layout.is_editor_layout() {
            return WindowLayout::Editor(Some(user_layout));
        }

        WindowLayout::Custom(user_layout)
    }

    pub fn backfill_editor_layout(fs: Arc<dyn Fs>, cx: &App) {
        let user_layout = cx
            .global::<SettingsStore>()
            .raw_user_settings()
            .map(|u| PanelLayout::read_from(u.content.as_ref()))
            .unwrap_or_default();

        update_settings_file(fs, cx, move |settings, _cx| {
            PanelLayout::EDITOR.backfill_to(&user_layout, settings);
        });
    }

    pub fn set_layout(
        layout: WindowLayout,
        fs: Arc<dyn Fs>,
        cx: &App,
    ) -> oneshot::Receiver<anyhow::Result<()>> {
        let merged = PanelLayout::read_from(cx.global::<SettingsStore>().merged_settings());

        match layout {
            WindowLayout::Agent(None) => {
                update_settings_file_with_completion(fs, cx, move |settings, _cx| {
                    PanelLayout::AGENT.write_diff_to(&merged, settings);
                })
            }
            WindowLayout::Editor(None) => {
                update_settings_file_with_completion(fs, cx, move |settings, _cx| {
                    PanelLayout::EDITOR.write_diff_to(&merged, settings);
                })
            }
            WindowLayout::Agent(Some(saved))
            | WindowLayout::Editor(Some(saved))
            | WindowLayout::Custom(saved) => {
                update_settings_file_with_completion(fs, cx, move |settings, _cx| {
                    saved.write_to(settings);
                })
            }
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
    /// Global default permission when no tool-specific rules or patterns match.
    pub default: ToolPermissionMode,
    pub tools: collections::HashMap<Arc<str>, ToolRules>,
}

impl ToolPermissions {
    /// Returns all invalid regex patterns across all tools.
    pub fn invalid_patterns(&self) -> Vec<&InvalidRegexPattern> {
        self.tools
            .values()
            .flat_map(|rules| rules.invalid_patterns.iter())
            .collect()
    }

    /// Returns true if any tool has invalid regex patterns.
    pub fn has_invalid_patterns(&self) -> bool {
        self.tools
            .values()
            .any(|rules| !rules.invalid_patterns.is_empty())
    }
}

/// Represents a regex pattern that failed to compile.
#[derive(Clone, Debug)]
pub struct InvalidRegexPattern {
    /// The pattern string that failed to compile.
    pub pattern: String,
    /// Which rule list this pattern was in (e.g., "always_deny", "always_allow", "always_confirm").
    pub rule_type: String,
    /// The error message from the regex compiler.
    pub error: String,
}

#[derive(Clone, Debug, Default)]
pub struct ToolRules {
    pub default: Option<ToolPermissionMode>,
    pub always_allow: Vec<CompiledRegex>,
    pub always_deny: Vec<CompiledRegex>,
    pub always_confirm: Vec<CompiledRegex>,
    /// Patterns that failed to compile. If non-empty, tool calls should be blocked.
    pub invalid_patterns: Vec<InvalidRegexPattern>,
}

#[derive(Clone)]
pub struct CompiledRegex {
    pub pattern: String,
    pub case_sensitive: bool,
    pub regex: regex::Regex,
}

impl std::fmt::Debug for CompiledRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledRegex")
            .field("pattern", &self.pattern)
            .field("case_sensitive", &self.case_sensitive)
            .finish()
    }
}

impl CompiledRegex {
    pub fn new(pattern: &str, case_sensitive: bool) -> Option<Self> {
        Self::try_new(pattern, case_sensitive).ok()
    }

    pub fn try_new(pattern: &str, case_sensitive: bool) -> Result<Self, regex::Error> {
        let regex = regex::RegexBuilder::new(pattern)
            .case_insensitive(!case_sensitive)
            .build()?;
        Ok(Self {
            pattern: pattern.to_string(),
            case_sensitive,
            regex,
        })
    }

    pub fn is_match(&self, input: &str) -> bool {
        self.regex.is_match(input)
    }
}

pub const HARDCODED_SECURITY_DENIAL_MESSAGE: &str = "Blocked by built-in security rule. This operation is considered too \
     harmful to be allowed, and cannot be overridden by settings.";

/// Security rules that are always enforced and cannot be overridden by any setting.
/// These protect against catastrophic operations like wiping filesystems.
pub struct HardcodedSecurityRules {
    pub terminal_deny: Vec<CompiledRegex>,
}

pub static HARDCODED_SECURITY_RULES: LazyLock<HardcodedSecurityRules> = LazyLock::new(|| {
    const FLAGS: &str = r"(--[a-zA-Z0-9][-a-zA-Z0-9_]*(=[^\s]*)?\s+|-[a-zA-Z]+\s+)*";
    const TRAILING_FLAGS: &str = r"(\s+--[a-zA-Z0-9][-a-zA-Z0-9_]*(=[^\s]*)?|\s+-[a-zA-Z]+)*\s*";

    HardcodedSecurityRules {
        terminal_deny: vec![
            // Recursive deletion of root - "rm -rf /", "rm -rf /*"
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?/\*?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
            // Recursive deletion of home via tilde - "rm -rf ~", "rm -rf ~/"
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?~/?\*?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
            // Recursive deletion of home via env var - "rm -rf $HOME", "rm -rf ${HOME}"
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?(\$HOME|\$\{{HOME\}})/?(\*)?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
            // Recursive deletion of current directory - "rm -rf .", "rm -rf ./"
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?\./?\*?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
            // Recursive deletion of parent directory - "rm -rf ..", "rm -rf ../"
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?\.\./?\*?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
        ],
    }
});

/// Checks if input matches any hardcoded security rules that cannot be bypassed.
/// Returns the denial reason string if blocked, None otherwise.
///
/// `terminal_tool_name` should be the tool name used for the terminal tool
/// (e.g. `"terminal"`). `extracted_commands` can optionally provide parsed
/// sub-commands for chained command checking; callers with access to a shell
/// parser should extract sub-commands and pass them here.
pub fn check_hardcoded_security_rules(
    tool_name: &str,
    terminal_tool_name: &str,
    input: &str,
    extracted_commands: Option<&[String]>,
) -> Option<String> {
    if tool_name != terminal_tool_name {
        return None;
    }

    let rules = &*HARDCODED_SECURITY_RULES;
    let terminal_patterns = &rules.terminal_deny;

    if matches_hardcoded_patterns(input, terminal_patterns) {
        return Some(HARDCODED_SECURITY_DENIAL_MESSAGE.into());
    }

    if let Some(commands) = extracted_commands {
        for command in commands {
            if matches_hardcoded_patterns(command, terminal_patterns) {
                return Some(HARDCODED_SECURITY_DENIAL_MESSAGE.into());
            }
        }
    }

    None
}

fn matches_hardcoded_patterns(command: &str, patterns: &[CompiledRegex]) -> bool {
    for pattern in patterns {
        if pattern.is_match(command) {
            return true;
        }
    }

    for expanded in expand_rm_to_single_path_commands(command) {
        for pattern in patterns {
            if pattern.is_match(&expanded) {
                return true;
            }
        }
    }

    false
}

fn expand_rm_to_single_path_commands(command: &str) -> Vec<String> {
    let trimmed = command.trim();

    let first_token = trimmed.split_whitespace().next();
    if !first_token.is_some_and(|t| t.eq_ignore_ascii_case("rm")) {
        return vec![];
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let mut flags = Vec::new();
    let mut paths = Vec::new();
    let mut past_double_dash = false;

    for part in parts.iter().skip(1) {
        if !past_double_dash && *part == "--" {
            past_double_dash = true;
            flags.push(*part);
            continue;
        }
        if !past_double_dash && part.starts_with('-') {
            flags.push(*part);
        } else {
            paths.push(*part);
        }
    }

    let flags_str = if flags.is_empty() {
        String::new()
    } else {
        format!("{} ", flags.join(" "))
    };

    let mut results = Vec::new();
    for path in &paths {
        if path.starts_with('$') {
            let home_prefix = if path.starts_with("${HOME}") {
                Some("${HOME}")
            } else if path.starts_with("$HOME") {
                Some("$HOME")
            } else {
                None
            };

            if let Some(prefix) = home_prefix {
                let suffix = &path[prefix.len()..];
                if suffix.is_empty() {
                    results.push(format!("rm {flags_str}{path}"));
                } else if suffix.starts_with('/') {
                    let normalized_suffix = normalize_path(suffix);
                    let reconstructed = if normalized_suffix == "/" {
                        prefix.to_string()
                    } else {
                        format!("{prefix}{normalized_suffix}")
                    };
                    results.push(format!("rm {flags_str}{reconstructed}"));
                } else {
                    results.push(format!("rm {flags_str}{path}"));
                }
            } else {
                results.push(format!("rm {flags_str}{path}"));
            }
            continue;
        }

        let mut normalized = normalize_path(path);
        if normalized.is_empty() && !Path::new(path).has_root() {
            normalized = ".".to_string();
        }

        results.push(format!("rm {flags_str}{normalized}"));
    }

    results
}

pub fn normalize_path(raw: &str) -> String {
    let is_absolute = Path::new(raw).has_root();
    let mut components: Vec<&str> = Vec::new();
    for component in Path::new(raw).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if components.last() == Some(&"..") {
                    components.push("..");
                } else if !components.is_empty() {
                    components.pop();
                } else if !is_absolute {
                    components.push("..");
                }
            }
            Component::Normal(segment) => {
                if let Some(s) = segment.to_str() {
                    components.push(s);
                }
            }
            Component::RootDir | Component::Prefix(_) => {}
        }
    }
    let joined = components.join("/");
    if is_absolute {
        format!("/{joined}")
    } else {
        joined
    }
}

impl Settings for AgentSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let agent = content.agent.clone().unwrap();
        Self {
            enabled: agent.enabled.unwrap(),
            button: agent.button.unwrap(),
            dock: agent.dock.unwrap(),
            sidebar_side: agent.sidebar_side.unwrap(),
            default_width: px(agent.default_width.unwrap()),
            default_height: px(agent.default_height.unwrap()),
            max_content_width: px(agent.max_content_width.unwrap()),
            flexible: agent.flexible.unwrap(),
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
            profiles: agent
                .profiles
                .unwrap()
                .into_iter()
                .map(|(key, val)| (AgentProfileId(key), val.into()))
                .collect(),

            notify_when_agent_waiting: agent.notify_when_agent_waiting.unwrap(),
            play_sound_when_agent_done: agent.play_sound_when_agent_done.unwrap_or_default(),
            single_file_review: agent.single_file_review.unwrap(),
            model_parameters: agent.model_parameters,
            enable_feedback: agent.enable_feedback.unwrap(),
            expand_edit_card: agent.expand_edit_card.unwrap(),
            expand_terminal_card: agent.expand_terminal_card.unwrap(),
            thinking_display: agent.thinking_display.unwrap(),
            cancel_generation_on_terminal_stop: agent.cancel_generation_on_terminal_stop.unwrap(),
            use_modifier_to_send: agent.use_modifier_to_send.unwrap(),
            message_editor_min_lines: agent.message_editor_min_lines.unwrap(),
            show_turn_stats: agent.show_turn_stats.unwrap(),
            show_merge_conflict_indicator: agent.show_merge_conflict_indicator.unwrap(),
            tool_permissions: compile_tool_permissions(agent.tool_permissions),
            new_thread_location: agent.new_thread_location.unwrap_or_default(),
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
            let mut invalid_patterns = Vec::new();

            let (always_allow, allow_errors) = compile_regex_rules(
                rules_content.always_allow.map(|v| v.0).unwrap_or_default(),
                "always_allow",
            );
            invalid_patterns.extend(allow_errors);

            let (always_deny, deny_errors) = compile_regex_rules(
                rules_content.always_deny.map(|v| v.0).unwrap_or_default(),
                "always_deny",
            );
            invalid_patterns.extend(deny_errors);

            let (always_confirm, confirm_errors) = compile_regex_rules(
                rules_content
                    .always_confirm
                    .map(|v| v.0)
                    .unwrap_or_default(),
                "always_confirm",
            );
            invalid_patterns.extend(confirm_errors);

            // Log invalid patterns for debugging. Users will see an error when they
            // attempt to use a tool with invalid patterns in their settings.
            for invalid in &invalid_patterns {
                log::error!(
                    "Invalid regex pattern in tool_permissions for '{}' tool ({}): '{}' - {}",
                    tool_name,
                    invalid.rule_type,
                    invalid.pattern,
                    invalid.error,
                );
            }

            let rules = ToolRules {
                // Preserve tool-specific default; None means fall back to global default at decision time
                default: rules_content.default,
                always_allow,
                always_deny,
                always_confirm,
                invalid_patterns,
            };
            (tool_name, rules)
        })
        .collect();

    ToolPermissions {
        default: content.default.unwrap_or_default(),
        tools,
    }
}

fn compile_regex_rules(
    rules: Vec<settings::ToolRegexRule>,
    rule_type: &str,
) -> (Vec<CompiledRegex>, Vec<InvalidRegexPattern>) {
    let mut compiled = Vec::new();
    let mut errors = Vec::new();

    for rule in rules {
        if rule.pattern.is_empty() {
            errors.push(InvalidRegexPattern {
                pattern: rule.pattern,
                rule_type: rule_type.to_string(),
                error: "empty regex patterns are not allowed".to_string(),
            });
            continue;
        }
        let case_sensitive = rule.case_sensitive.unwrap_or(false);
        match CompiledRegex::try_new(&rule.pattern, case_sensitive) {
            Ok(regex) => compiled.push(regex),
            Err(error) => {
                errors.push(InvalidRegexPattern {
                    pattern: rule.pattern,
                    rule_type: rule_type.to_string(),
                    error: error.to_string(),
                });
            }
        }
    }

    (compiled, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, UpdateGlobal};
    use serde_json::json;
    use settings::ToolPermissionMode;
    use settings::ToolPermissionsContent;

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
                    "default": "allow",
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
        assert_eq!(terminal_rules.default, Some(ToolPermissionMode::Allow));
        assert_eq!(terminal_rules.always_deny.len(), 1);
        assert_eq!(terminal_rules.always_allow.len(), 1);
        assert!(terminal_rules.always_deny[0].is_match("rm -rf /"));
        assert!(terminal_rules.always_allow[0].is_match("git status"));
    }

    #[test]
    fn test_tool_rules_default() {
        let json = json!({
            "tools": {
                "edit_file": {
                    "default": "deny"
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        let rules = permissions.tools.get("edit_file").unwrap();
        assert_eq!(rules.default, Some(ToolPermissionMode::Deny));
    }

    #[test]
    fn test_tool_permissions_empty() {
        let permissions = compile_tool_permissions(None);
        assert!(permissions.tools.is_empty());
        assert_eq!(permissions.default, ToolPermissionMode::Confirm);
    }

    #[test]
    fn test_tool_rules_default_returns_confirm() {
        let default_rules = ToolRules::default();
        assert_eq!(default_rules.default, None);
        assert!(default_rules.always_allow.is_empty());
        assert!(default_rules.always_deny.is_empty());
        assert!(default_rules.always_confirm.is_empty());
    }

    #[test]
    fn test_tool_permissions_with_multiple_tools() {
        let json = json!({
            "tools": {
                "terminal": {
                    "default": "allow",
                    "always_deny": [{ "pattern": "rm\\s+-rf" }]
                },
                "edit_file": {
                    "default": "confirm",
                    "always_deny": [{ "pattern": "\\.env$" }]
                },
                "delete_path": {
                    "default": "deny"
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        assert_eq!(permissions.tools.len(), 3);

        let terminal = permissions.tools.get("terminal").unwrap();
        assert_eq!(terminal.default, Some(ToolPermissionMode::Allow));
        assert_eq!(terminal.always_deny.len(), 1);

        let edit_file = permissions.tools.get("edit_file").unwrap();
        assert_eq!(edit_file.default, Some(ToolPermissionMode::Confirm));
        assert!(edit_file.always_deny[0].is_match("secrets.env"));

        let delete_path = permissions.tools.get("delete_path").unwrap();
        assert_eq!(delete_path.default, Some(ToolPermissionMode::Deny));
    }

    #[test]
    fn test_tool_permissions_with_all_rule_types() {
        let json = json!({
            "tools": {
                "terminal": {
                    "always_deny": [{ "pattern": "rm\\s+-rf" }],
                    "always_confirm": [{ "pattern": "sudo\\s" }],
                    "always_allow": [{ "pattern": "^git\\s+status" }]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        let terminal = permissions.tools.get("terminal").unwrap();
        assert_eq!(terminal.always_deny.len(), 1);
        assert_eq!(terminal.always_confirm.len(), 1);
        assert_eq!(terminal.always_allow.len(), 1);

        assert!(terminal.always_deny[0].is_match("rm -rf /"));
        assert!(terminal.always_confirm[0].is_match("sudo apt install"));
        assert!(terminal.always_allow[0].is_match("git status"));
    }

    #[test]
    fn test_invalid_regex_is_tracked_and_valid_ones_still_compile() {
        let json = json!({
            "tools": {
                "terminal": {
                    "always_deny": [
                        { "pattern": "[invalid(regex" },
                        { "pattern": "valid_pattern" }
                    ],
                    "always_allow": [
                        { "pattern": "[another_bad" }
                    ]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        let terminal = permissions.tools.get("terminal").unwrap();

        // Valid patterns should still be compiled
        assert_eq!(terminal.always_deny.len(), 1);
        assert!(terminal.always_deny[0].is_match("valid_pattern"));

        // Invalid patterns should be tracked (order depends on processing order)
        assert_eq!(terminal.invalid_patterns.len(), 2);

        let deny_invalid = terminal
            .invalid_patterns
            .iter()
            .find(|p| p.rule_type == "always_deny")
            .expect("should have invalid pattern from always_deny");
        assert_eq!(deny_invalid.pattern, "[invalid(regex");
        assert!(!deny_invalid.error.is_empty());

        let allow_invalid = terminal
            .invalid_patterns
            .iter()
            .find(|p| p.rule_type == "always_allow")
            .expect("should have invalid pattern from always_allow");
        assert_eq!(allow_invalid.pattern, "[another_bad");

        // ToolPermissions helper methods should work
        assert!(permissions.has_invalid_patterns());
        assert_eq!(permissions.invalid_patterns().len(), 2);
    }

    #[test]
    fn test_deny_takes_precedence_over_allow_and_confirm() {
        let json = json!({
            "tools": {
                "terminal": {
                    "default": "allow",
                    "always_deny": [{ "pattern": "dangerous" }],
                    "always_confirm": [{ "pattern": "dangerous" }],
                    "always_allow": [{ "pattern": "dangerous" }]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));
        let terminal = permissions.tools.get("terminal").unwrap();

        assert!(
            terminal.always_deny[0].is_match("run dangerous command"),
            "Deny rule should match"
        );
        assert!(
            terminal.always_allow[0].is_match("run dangerous command"),
            "Allow rule should also match (but deny takes precedence at evaluation time)"
        );
        assert!(
            terminal.always_confirm[0].is_match("run dangerous command"),
            "Confirm rule should also match (but deny takes precedence at evaluation time)"
        );
    }

    #[test]
    fn test_confirm_takes_precedence_over_allow() {
        let json = json!({
            "tools": {
                "terminal": {
                    "default": "allow",
                    "always_confirm": [{ "pattern": "risky" }],
                    "always_allow": [{ "pattern": "risky" }]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));
        let terminal = permissions.tools.get("terminal").unwrap();

        assert!(
            terminal.always_confirm[0].is_match("do risky thing"),
            "Confirm rule should match"
        );
        assert!(
            terminal.always_allow[0].is_match("do risky thing"),
            "Allow rule should also match (but confirm takes precedence at evaluation time)"
        );
    }

    #[test]
    fn test_regex_matches_anywhere_in_string_not_just_anchored() {
        let json = json!({
            "tools": {
                "terminal": {
                    "always_deny": [
                        { "pattern": "rm\\s+-rf" },
                        { "pattern": "/etc/passwd" }
                    ]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));
        let terminal = permissions.tools.get("terminal").unwrap();

        assert!(
            terminal.always_deny[0].is_match("echo hello && rm -rf /"),
            "Should match rm -rf in the middle of a command chain"
        );
        assert!(
            terminal.always_deny[0].is_match("cd /tmp; rm -rf *"),
            "Should match rm -rf after semicolon"
        );
        assert!(
            terminal.always_deny[1].is_match("cat /etc/passwd | grep root"),
            "Should match /etc/passwd in a pipeline"
        );
        assert!(
            terminal.always_deny[1].is_match("vim /etc/passwd"),
            "Should match /etc/passwd as argument"
        );
    }

    #[test]
    fn test_fork_bomb_pattern_matches() {
        let fork_bomb_regex = CompiledRegex::new(r":\(\)\{\s*:\|:&\s*\};:", false).unwrap();
        assert!(
            fork_bomb_regex.is_match(":(){ :|:& };:"),
            "Should match the classic fork bomb"
        );
        assert!(
            fork_bomb_regex.is_match(":(){ :|:&};:"),
            "Should match fork bomb without spaces"
        );
    }

    #[test]
    fn test_compiled_regex_stores_case_sensitivity() {
        let case_sensitive = CompiledRegex::new("test", true).unwrap();
        let case_insensitive = CompiledRegex::new("test", false).unwrap();

        assert!(case_sensitive.case_sensitive);
        assert!(!case_insensitive.case_sensitive);
    }

    #[test]
    fn test_invalid_regex_is_skipped_not_fail() {
        let json = json!({
            "tools": {
                "terminal": {
                    "always_deny": [
                        { "pattern": "[invalid(regex" },
                        { "pattern": "valid_pattern" }
                    ]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        let terminal = permissions.tools.get("terminal").unwrap();
        assert_eq!(terminal.always_deny.len(), 1);
        assert!(terminal.always_deny[0].is_match("valid_pattern"));
    }

    #[test]
    fn test_unconfigured_tool_not_in_permissions() {
        let json = json!({
            "tools": {
                "terminal": {
                    "default": "allow"
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        assert!(permissions.tools.contains_key("terminal"));
        assert!(!permissions.tools.contains_key("edit_file"));
        assert!(!permissions.tools.contains_key("fetch"));
    }

    #[test]
    fn test_always_allow_pattern_only_matches_specified_commands() {
        // Reproduces user-reported bug: when always_allow has pattern "^echo\s",
        // only "echo hello" should be allowed, not "git status".
        //
        // User config:
        //   always_allow_tool_actions: false
        //   tool_permissions.tools.terminal.always_allow: [{ pattern: "^echo\\s" }]
        let json = json!({
            "tools": {
                "terminal": {
                    "always_allow": [
                        { "pattern": "^echo\\s" }
                    ]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        let terminal = permissions.tools.get("terminal").unwrap();

        // Verify the pattern was compiled
        assert_eq!(
            terminal.always_allow.len(),
            1,
            "Should have one always_allow pattern"
        );

        // Verify the pattern matches "echo hello"
        assert!(
            terminal.always_allow[0].is_match("echo hello"),
            "Pattern ^echo\\s should match 'echo hello'"
        );

        // Verify the pattern does NOT match "git status"
        assert!(
            !terminal.always_allow[0].is_match("git status"),
            "Pattern ^echo\\s should NOT match 'git status'"
        );

        // Verify the pattern does NOT match "echoHello" (no space)
        assert!(
            !terminal.always_allow[0].is_match("echoHello"),
            "Pattern ^echo\\s should NOT match 'echoHello' (requires whitespace)"
        );

        assert_eq!(
            terminal.default, None,
            "default should be None when not specified"
        );
    }

    #[test]
    fn test_empty_regex_pattern_is_invalid() {
        let json = json!({
            "tools": {
                "terminal": {
                    "always_allow": [
                        { "pattern": "" }
                    ],
                    "always_deny": [
                        { "case_sensitive": true }
                    ],
                    "always_confirm": [
                        { "pattern": "" },
                        { "pattern": "valid_pattern" }
                    ]
                }
            }
        });

        let content: ToolPermissionsContent = serde_json::from_value(json).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        let terminal = permissions.tools.get("terminal").unwrap();

        assert_eq!(terminal.always_allow.len(), 0);
        assert_eq!(terminal.always_deny.len(), 0);
        assert_eq!(terminal.always_confirm.len(), 1);
        assert!(terminal.always_confirm[0].is_match("valid_pattern"));

        assert_eq!(terminal.invalid_patterns.len(), 3);
        for invalid in &terminal.invalid_patterns {
            assert_eq!(invalid.pattern, "");
            assert!(invalid.error.contains("empty"));
        }
    }

    #[test]
    fn test_default_json_tool_permissions_parse() {
        let default_json = include_str!("../../../assets/settings/default.json");
        let value: serde_json_lenient::Value = serde_json_lenient::from_str(default_json).unwrap();
        let agent = value
            .get("agent")
            .expect("default.json should have 'agent' key");
        let tool_permissions_value = agent
            .get("tool_permissions")
            .expect("agent should have 'tool_permissions' key");

        let content: ToolPermissionsContent =
            serde_json_lenient::from_value(tool_permissions_value.clone()).unwrap();
        let permissions = compile_tool_permissions(Some(content));

        assert_eq!(permissions.default, ToolPermissionMode::Confirm);

        assert!(
            permissions.tools.is_empty(),
            "default.json should not have any active tool-specific rules, found: {:?}",
            permissions.tools.keys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_tool_permissions_explicit_global_default() {
        let json_allow = json!({
            "default": "allow"
        });
        let content: ToolPermissionsContent = serde_json::from_value(json_allow).unwrap();
        let permissions = compile_tool_permissions(Some(content));
        assert_eq!(permissions.default, ToolPermissionMode::Allow);

        let json_deny = json!({
            "default": "deny"
        });
        let content: ToolPermissionsContent = serde_json::from_value(json_deny).unwrap();
        let permissions = compile_tool_permissions(Some(content));
        assert_eq!(permissions.default, ToolPermissionMode::Deny);
    }

    #[gpui::test]
    fn test_get_layout(cx: &mut gpui::App) {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        project::DisableAiSettings::register(cx);
        AgentSettings::register(cx);

        // Should be Editor with an empty user layout (user hasn't customized).
        let layout = AgentSettings::get_layout(cx);
        let WindowLayout::Editor(Some(user_layout)) = layout else {
            panic!("expected Editor(Some), got {:?}", layout);
        };
        assert_eq!(user_layout, PanelLayout::default());

        // User explicitly sets agent dock to left. Combined with defaults
        // (agent=right, others=left), merged becomes {agent=left, others=left}
        // which matches neither preset.
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(r#"{ "agent": { "dock": "left" } }"#, cx)
                .unwrap();
        });

        let layout = AgentSettings::get_layout(cx);
        let WindowLayout::Custom(user_layout) = layout else {
            panic!("expected Custom, got {:?}", layout);
        };
        assert_eq!(user_layout.agent_dock, Some(DockPosition::Left));
        assert_eq!(user_layout.project_panel_dock, None);
        assert_eq!(user_layout.outline_panel_dock, None);
        assert_eq!(user_layout.collaboration_panel_dock, None);
        assert_eq!(user_layout.git_panel_dock, None);

        // User sets a combination that doesn't match either preset:
        // agent on the left but project panel also on the left.
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(
                    r#"{
                        "agent": { "dock": "left" },
                        "project_panel": { "dock": "left" }
                    }"#,
                    cx,
                )
                .unwrap();
        });

        let layout = AgentSettings::get_layout(cx);
        let WindowLayout::Custom(user_layout) = layout else {
            panic!("expected Custom, got {:?}", layout);
        };
        assert_eq!(user_layout.agent_dock, Some(DockPosition::Left));
        assert_eq!(user_layout.project_panel_dock, Some(DockSide::Left));
    }

    #[gpui::test]
    fn test_set_layout_round_trip(cx: &mut gpui::App) {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        project::DisableAiSettings::register(cx);
        AgentSettings::register(cx);

        // User has a custom layout: agent on the right with project panel
        // also on the right. This doesn't match either preset.
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(
                    r#"{
                        "agent": { "dock": "right" },
                        "project_panel": { "dock": "right" }
                    }"#,
                    cx,
                )
                .unwrap();
        });

        let original = AgentSettings::get_layout(cx);
        let WindowLayout::Custom(ref original_user_layout) = original else {
            panic!("expected Custom, got {:?}", original);
        };
        assert_eq!(original_user_layout.agent_dock, Some(DockPosition::Right));
        assert_eq!(
            original_user_layout.project_panel_dock,
            Some(DockSide::Right)
        );
        assert_eq!(original_user_layout.outline_panel_dock, None);

        // Switch to the agent layout. This overwrites the user settings.
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                PanelLayout::AGENT.write_to(settings);
            });
        });

        let layout = AgentSettings::get_layout(cx);
        assert!(matches!(layout, WindowLayout::Agent(_)));

        // Restore the original custom layout.
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                original_user_layout.write_to(settings);
            });
        });

        // Should be back to the same custom layout.
        let restored = AgentSettings::get_layout(cx);
        let WindowLayout::Custom(restored_user_layout) = restored else {
            panic!("expected Custom, got {:?}", restored);
        };
        assert_eq!(restored_user_layout.agent_dock, Some(DockPosition::Right));
        assert_eq!(
            restored_user_layout.project_panel_dock,
            Some(DockSide::Right)
        );
        assert_eq!(restored_user_layout.outline_panel_dock, None);
    }

    #[gpui::test]
    async fn test_set_layout_minimal_diff(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        fs.save(
            paths::settings_file().as_path(),
            &serde_json::json!({
                "agent": { "dock": "left" },
                "project_panel": { "dock": "left" }
            })
            .to_string()
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            project::DisableAiSettings::register(cx);
            AgentSettings::register(cx);

            // User has agent=left (matches preset) and project_panel=left (does not)
            SettingsStore::update_global(cx, |store, cx| {
                store
                    .set_user_settings(
                        r#"{
                            "agent": { "dock": "left" },
                            "project_panel": { "dock": "left" }
                        }"#,
                        cx,
                    )
                    .unwrap();
            });

            let layout = AgentSettings::get_layout(cx);
            assert!(matches!(layout, WindowLayout::Custom(_)));

            AgentSettings::set_layout(WindowLayout::agent(), fs.clone(), cx)
        })
        .await
        .ok();

        cx.run_until_parked();

        let written = fs.load(paths::settings_file().as_path()).await.unwrap();
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.set_user_settings(&written, cx).unwrap();
            });

            // The user settings should still have agent=left (preserved)
            // and now project_panel=right (changed to match preset).
            let store = cx.global::<SettingsStore>();
            let user_layout = store
                .raw_user_settings()
                .map(|u| PanelLayout::read_from(u.content.as_ref()))
                .unwrap_or_default();

            assert_eq!(user_layout.agent_dock, Some(DockPosition::Left));
            assert_eq!(user_layout.project_panel_dock, Some(DockSide::Right));
            // With defaults having these panels on the left, the diff to
            // the agent preset also writes outline, collaboration, and git
            // panel positions into user settings.
            assert_eq!(user_layout.outline_panel_dock, Some(DockSide::Right));
            assert_eq!(
                user_layout.collaboration_panel_dock,
                Some(DockPosition::Right)
            );
            assert_eq!(user_layout.git_panel_dock, Some(DockPosition::Right));

            // And the merged result should now match agent.
            let layout = AgentSettings::get_layout(cx);
            assert!(matches!(layout, WindowLayout::Agent(_)));
        });
    }

    #[gpui::test]
    async fn test_backfill_editor_layout(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        // User has only customized project_panel to "right".
        fs.save(
            paths::settings_file().as_path(),
            &serde_json::json!({
                "project_panel": { "dock": "right" }
            })
            .to_string()
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            project::DisableAiSettings::register(cx);
            AgentSettings::register(cx);

            // Simulate pre-migration state: editor defaults (the old world).
            SettingsStore::update_global(cx, |store, cx| {
                store.update_default_settings(cx, |defaults| {
                    PanelLayout::EDITOR.write_to(defaults);
                });
            });

            // User has only customized project_panel to "right".
            SettingsStore::update_global(cx, |store, cx| {
                store
                    .set_user_settings(r#"{ "project_panel": { "dock": "right" } }"#, cx)
                    .unwrap();
            });

            // Run the one-time backfill while still on old defaults.
            AgentSettings::backfill_editor_layout(fs.clone(), cx);
        });

        cx.run_until_parked();

        // Read back the file and apply it.
        let written = fs.load(paths::settings_file().as_path()).await.unwrap();
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.set_user_settings(&written, cx).unwrap();
            });

            // The user's project_panel=right should be preserved (they set it).
            // All other fields should now have the editor preset values
            // written into user settings.
            let store = cx.global::<SettingsStore>();
            let user_layout = store
                .raw_user_settings()
                .map(|u| PanelLayout::read_from(u.content.as_ref()))
                .unwrap_or_default();

            assert_eq!(user_layout.agent_dock, Some(DockPosition::Right));
            assert_eq!(user_layout.project_panel_dock, Some(DockSide::Right));
            assert_eq!(user_layout.outline_panel_dock, Some(DockSide::Left));
            assert_eq!(
                user_layout.collaboration_panel_dock,
                Some(DockPosition::Left)
            );
            assert_eq!(user_layout.git_panel_dock, Some(DockPosition::Left));

            // Even though defaults are now agent, the backfilled user settings
            // keep everything in the editor layout. The user's experience
            // hasn't changed.
            let layout = AgentSettings::get_layout(cx);
            let WindowLayout::Custom(user_layout) = layout else {
                panic!(
                    "expected Custom (editor values override agent defaults), got {:?}",
                    layout
                );
            };
            assert_eq!(user_layout.agent_dock, Some(DockPosition::Right));
            assert_eq!(user_layout.project_panel_dock, Some(DockSide::Right));
        });
    }
}

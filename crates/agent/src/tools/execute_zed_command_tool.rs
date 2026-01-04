use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{Action, App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

/// Execute a Zed command from the command palette.
///
/// This tool allows the agent to execute any command that would be available in Zed's command palette.
/// Commands are actions that can perform various operations like opening files, running tasks, changing settings,
/// toggling panels, and more.
///
/// To find available commands, you can use common Zed commands like:
/// - "editor: go to line" - Navigate to a specific line
/// - "file_finder::Toggle" - Open the file finder
/// - "workspace::Save" - Save the current file
/// - "terminal_panel::ToggleFocus" - Toggle terminal panel focus
/// - "project_panel::ToggleFocus" - Toggle project panel focus
/// - "diagnostics::Deploy" - Show diagnostics panel
/// - "tab_switcher::Toggle" - Open tab switcher
///
/// Use this tool when you need to trigger Zed-specific functionality that isn't covered by other tools.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteCommandToolInput {
    /// The name of the command to execute. This can be either:
    /// - A humanized name (e.g., "go to line", "toggle terminal")
    /// - An action name (e.g., "workspace::Save", "editor::GoToLine")
    ///
    /// The tool will attempt to match the command name against available actions.
    pub command: String,
}

pub struct ExecuteZedCommandTool;

impl ExecuteZedCommandTool {
    pub fn new() -> Self {
        Self
    }

    /// Find an action by name (case-insensitive, fuzzy matching)
    fn find_action(
        command_name: &str,
        available_actions: Vec<Box<dyn Action>>,
    ) -> Option<Box<dyn Action>> {
        let normalized_query = normalize_command_query(command_name);

        // First, try exact match on action name
        for action in &available_actions {
            if action.name().eq_ignore_ascii_case(&normalized_query) {
                return Some(action.boxed_clone());
            }
        }

        // Second, try exact match on humanized name
        for action in &available_actions {
            let humanized = humanize_action_name(action.name());
            if humanized.eq_ignore_ascii_case(&normalized_query) {
                return Some(action.boxed_clone());
            }
        }

        // Third, try substring match on action name
        for action in &available_actions {
            if action
                .name()
                .to_lowercase()
                .contains(&normalized_query.to_lowercase())
            {
                return Some(action.boxed_clone());
            }
        }

        // Finally, try substring match on humanized name
        for action in &available_actions {
            let humanized = humanize_action_name(action.name()).to_lowercase();
            if humanized.contains(&normalized_query.to_lowercase()) {
                return Some(action.boxed_clone());
            }
        }

        None
    }
}

impl Default for ExecuteZedCommandTool {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentTool for ExecuteZedCommandTool {
    type Input = ExecuteCommandToolInput;
    type Output = String;

    fn name() -> &'static str {
        "execute_zed_command"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Execute
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Execute command: {}", MarkdownInlineCode(&input.command)).into()
        } else {
            "Execute command".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let authorize = event_stream.authorize(self.initial_title(Ok(input.clone()), cx), cx);

        // Get available actions from the active window
        let active_window = cx.active_window();
        let available_actions = if let Some(window) = active_window {
            window
                .update(cx, |_, window, cx| window.available_actions(cx))
                .ok()
        } else {
            None
        };

        cx.spawn(async move |cx| {
            authorize.await?;

            let available_actions = available_actions.ok_or_else(|| {
                anyhow!("No active window found. Cannot execute command without an active window.")
            })?;

            // Find the matching action
            let action = Self::find_action(&input.command, available_actions)
                .ok_or_else(|| {
                    anyhow!(
                        "Command '{}' not found. The command may not be available in the current context, \
                        or the command name may be incorrect. Try a different command name or check that \
                        the required features are enabled.",
                        input.command
                    )
                })?;

            let action_name = action.name().to_string();

            // Dispatch the action
            cx.update(|cx| {
                cx.dispatch_action(&*action);
            })?;

            Ok(format!(
                "Successfully executed command: '{}' (action: {})",
                input.command,
                action_name
            ))
        })
    }
}

/// Normalize command query for matching
fn normalize_command_query(input: &str) -> String {
    input
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '_')
        .collect::<String>()
        .to_lowercase()
}

/// Convert an action name to a human-readable format
fn humanize_action_name(action_name: &str) -> String {
    let parts: Vec<&str> = action_name.split("::").collect();

    if parts.len() == 1 {
        // No namespace, just humanize the action name
        return humanize_string(parts[0]);
    }

    // Include namespace for better disambiguation
    let namespace = humanize_string(parts[0]);
    let action = humanize_string(parts[1]);

    format!("{}: {}", namespace, action)
}

/// Helper function to humanize a single string (split on camel case and underscores)
fn humanize_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '_' {
            result.push(' ');
        } else {
            if c.is_uppercase() && !result.is_empty() {
                result.push(' ');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humanize_action_name() {
        assert_eq!(humanize_action_name("workspace::Save"), "workspace: save");
        assert_eq!(
            humanize_action_name("editor::GoToLine"),
            "editor: go to line"
        );
        assert_eq!(
            humanize_action_name("file_finder::Toggle"),
            "file finder: toggle"
        );
        assert_eq!(
            humanize_action_name("terminal_panel::Toggle"),
            "terminal panel: toggle"
        );
        assert_eq!(humanize_action_name("ToggleFocus"), "toggle focus");
    }

    #[test]
    fn test_normalize_command_query() {
        assert_eq!(normalize_command_query("Go To Line"), "gotoline");
        assert_eq!(
            normalize_command_query("workspace::Save"),
            "workspace::save"
        );
        assert_eq!(normalize_command_query("  toggle focus  "), "togglefocus");
    }
}

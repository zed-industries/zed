use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{Action, App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// List all available Zed commands in the current context.
///
/// This tool returns a list of all commands that are currently available in Zed's command palette,
/// along with their humanized names and action identifiers. This is useful for discovering what
/// commands can be executed using the execute_zed_command tool.
///
/// The list includes:
/// - Action name (e.g., "workspace::Save")
/// - Humanized name (e.g., "save")
/// - Whether the command is available in the current context
///
/// Use this tool when you need to discover what commands are available or when you're unsure
/// of the exact command name to use with execute_zed_command.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListCommandsToolInput {
    /// Optional filter to search for commands containing this text (case-insensitive).
    /// If not provided, all available commands will be returned.
    #[serde(default)]
    pub filter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CommandInfo {
    /// The full action name (e.g., "workspace::Save")
    pub action_name: String,
    /// The humanized/friendly name (e.g., "save")
    pub humanized_name: String,
    /// A brief description if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub struct ListZedCommandsTool;

impl ListZedCommandsTool {
    pub fn new() -> Self {
        Self
    }

    /// Get all available commands from the active window
    fn get_available_commands(cx: &mut App) -> Result<Vec<Box<dyn Action>>> {
        if let Some(window) = cx.active_window() {
            window
                .update(cx, |_, window, cx| window.available_actions(cx))
                .map_err(|e| anyhow::anyhow!("Failed to get available actions: {}", e))
        } else {
            Err(anyhow::anyhow!("No active window found"))
        }
    }

    /// Format commands into a readable string
    fn format_commands(commands: Vec<CommandInfo>, filter: Option<&str>) -> String {
        let filtered_commands: Vec<_> = if let Some(filter_text) = filter {
            let filter_lower = filter_text.to_lowercase();
            commands
                .into_iter()
                .filter(|cmd| {
                    cmd.action_name.to_lowercase().contains(&filter_lower)
                        || cmd.humanized_name.to_lowercase().contains(&filter_lower)
                })
                .collect()
        } else {
            commands
        };

        if filtered_commands.is_empty() {
            return if let Some(filter_text) = filter {
                format!("No commands found matching '{}'", filter_text)
            } else {
                "No commands available in the current context".to_string()
            };
        }

        let mut output = String::new();
        output.push_str(&format!(
            "Found {} available command{}:\n\n",
            filtered_commands.len(),
            if filtered_commands.len() == 1 {
                ""
            } else {
                "s"
            }
        ));

        // Group commands by namespace for better readability
        use std::collections::BTreeMap;
        let mut grouped: BTreeMap<String, Vec<CommandInfo>> = BTreeMap::new();

        for cmd in filtered_commands {
            let namespace = if let Some(pos) = cmd.action_name.rfind("::") {
                cmd.action_name[..pos].to_string()
            } else {
                "global".to_string()
            };
            grouped.entry(namespace).or_default().push(cmd);
        }

        for (namespace, cmds) in grouped {
            output.push_str(&format!("## {} ({})\n", namespace, cmds.len()));
            for cmd in cmds {
                output.push_str(&format!(
                    "- **{}** → `{}`\n",
                    cmd.humanized_name, cmd.action_name
                ));
                if let Some(desc) = &cmd.description {
                    output.push_str(&format!("  {}\n", desc));
                }
            }
            output.push('\n');
        }

        output
    }
}

impl Default for ListZedCommandsTool {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentTool for ListZedCommandsTool {
    type Input = ListCommandsToolInput;
    type Output = String;

    fn name() -> &'static str {
        "list_zed_commands"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            if let Some(filter) = &input.filter {
                format!("List commands matching '{}'", filter).into()
            } else {
                "List all available commands".into()
            }
        } else {
            "List commands".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let available_actions = Self::get_available_commands(cx);

        Task::ready((|| {
            let actions = available_actions?;

            let mut commands: Vec<CommandInfo> = actions
                .iter()
                .map(|action| CommandInfo {
                    action_name: action.name().to_string(),
                    humanized_name: humanize_action_name(action.name()),
                    description: None, // Could be enhanced to extract from doc comments
                })
                .collect();

            // Sort by action name for consistency
            commands.sort_by(|a, b| a.action_name.cmp(&b.action_name));

            Ok(Self::format_commands(commands, input.filter.as_deref()))
        })())
    }
}

/// Convert an action name to a human-readable format
/// This should match the implementation in execute_command_tool.rs
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
    fn test_format_commands_empty() {
        let commands = vec![];
        let output = ListZedCommandsTool::format_commands(commands, None);
        assert!(output.contains("No commands available"));
    }

    #[test]
    fn test_format_commands_with_filter_no_match() {
        let commands = vec![CommandInfo {
            action_name: "workspace::Save".to_string(),
            humanized_name: "save".to_string(),
            description: None,
        }];
        let output = ListZedCommandsTool::format_commands(commands, Some("toggle"));
        assert!(output.contains("No commands found matching 'toggle'"));
    }

    #[test]
    fn test_format_commands_with_filter_match() {
        let commands = vec![
            CommandInfo {
                action_name: "workspace::Save".to_string(),
                humanized_name: "save".to_string(),
                description: None,
            },
            CommandInfo {
                action_name: "workspace::SaveAll".to_string(),
                humanized_name: "save all".to_string(),
                description: None,
            },
        ];
        let output = ListZedCommandsTool::format_commands(commands, Some("save"));
        assert!(output.contains("Found 2"));
        assert!(output.contains("workspace::Save"));
        assert!(output.contains("workspace::SaveAll"));
    }
}

use std::fmt::Write;
use std::sync::Arc;

use agent_client_protocol::schema::v1 as acp;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::ResultExt;

use super::symbol_locator::CodeActionStore;
use crate::{AgentTool, ToolCallEventStream, ToolInput};

/// Applies a code action previously retrieved by get_code_actions.
///
/// You must call get_code_actions first to get the list of available actions,
/// then use the number from that list to choose which action to apply.
///
/// After applying a code action, the list is cleared. If you want to apply
/// another action, call get_code_actions again.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ApplyCodeActionToolInput {
    /// The 1-based index of the code action to apply, from the list
    /// returned by get_code_actions.
    pub index: u32,
}

pub struct ApplyCodeActionTool {
    project: Entity<Project>,
    code_action_store: CodeActionStore,
}

impl ApplyCodeActionTool {
    pub fn new(project: Entity<Project>, code_action_store: CodeActionStore) -> Self {
        Self {
            project,
            code_action_store,
        }
    }
}

impl AgentTool for ApplyCodeActionTool {
    type Input = ApplyCodeActionToolInput;
    type Output = String;

    const NAME: &'static str = "apply_code_action";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            let title = self
                .code_action_store
                .read(cx)
                .as_ref()
                .and_then(|pending| {
                    let index = input.index.checked_sub(1)? as usize;
                    Some(pending.actions.get(index)?.lsp_action.title().to_string())
                });
            if let Some(title) = title {
                format!("Apply code action: {title}").into()
            } else {
                format!("Apply code action #{}", input.index).into()
            }
        } else {
            "Apply code action".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        let project = self.project.clone();
        let store = self.code_action_store.clone();
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            let pending = store.update(cx, |store, _cx| store.take()).ok_or_else(|| {
                "No code actions available. Call get_code_actions first.".to_string()
            })?;

            let zero_based_index = input
                .index
                .checked_sub(1)
                .ok_or_else(|| "Index must be 1 or greater.".to_string())?;

            let action = pending
                .actions
                .get(zero_based_index as usize)
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "Index {} is out of range. There were {} code action(s) available.",
                        input.index,
                        pending.actions.len()
                    )
                })?;

            let title = action.lsp_action.title().to_string();
            let buffer = pending.buffer.clone();

            let apply_task = project.update(cx, |project, cx| {
                project.apply_code_action_with_client_command(buffer, action, true, cx)
            });

            let (transaction, client_command) = apply_task
                .await
                .map_err(|e| format!("Failed to apply code action '{title}': {e}"))?;

            if transaction.0.is_empty() {
                if let Some(client_command) = client_command {
                    return Ok(format!(
                        "Code action '{title}' was applied, but {}",
                        client_command_not_applied_message(&client_command),
                    ));
                }

                return Ok(format!(
                    "Code action '{title}' was applied but made no changes.",
                ));
            }

            let mut output = format!(
                "Applied code action '{title}'. Modified {} file(s):\n",
                transaction.0.len()
            );

            for (buffer, _) in &transaction.0 {
                buffer.read_with(cx, |buffer, cx| {
                    let path = buffer
                        .file()
                        .map(|f| f.full_path(cx).display().to_string())
                        .unwrap_or_else(|| "<untitled>".to_string());
                    writeln!(output, "- {path}").log_err();
                });
            }

            if let Some(client_command) = client_command {
                writeln!(
                    output,
                    "\nThe code action also requested an editor command, but {}",
                    client_command_not_applied_message(&client_command),
                )
                .log_err();
            }

            Ok(output)
        })
    }
}

fn client_command_not_applied_message(client_command: &language::ClientCommand) -> String {
    match client_command {
        language::ClientCommand::ShowLocations(_) => {
            "the editor must open the requested locations interactively".to_string()
        }
        language::ClientCommand::ScheduleTask(task_template) => format!(
            "the editor must schedule the task '{}' interactively",
            task_template.label
        ),
    }
}

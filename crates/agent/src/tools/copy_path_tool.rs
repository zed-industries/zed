use super::edit_file_tool::{
    SensitiveSettingsKind, is_sensitive_settings_path, sensitive_settings_kind,
};
use crate::{AgentTool, ToolCallEventStream, ToolPermissionDecision, decide_permission_for_path};
use agent_client_protocol::ToolKind;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, anyhow};
use futures::FutureExt as _;
use gpui::{App, Entity, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::path::Path;
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

/// Copies a file or directory in the project, and returns confirmation that the copy succeeded.
/// Directory contents will be copied recursively.
///
/// This tool should be used when it's desirable to create a copy of a file or directory without modifying the original.
/// It's much more efficient than doing this by separately reading and then writing the file or directory's contents, so this tool should be preferred over that approach whenever copying is the goal.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CopyPathToolInput {
    /// The source path of the file or directory to copy.
    /// If a directory is specified, its contents will be copied recursively.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can copy the first file by providing a source_path of "directory1/a/something.txt"
    /// </example>
    pub source_path: String,
    /// The destination path where the file or directory should be copied to.
    ///
    /// <example>
    /// To copy "directory1/a/something.txt" to "directory2/b/copy.txt", provide a destination_path of "directory2/b/copy.txt"
    /// </example>
    pub destination_path: String,
}

pub struct CopyPathTool {
    project: Entity<Project>,
}

impl CopyPathTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for CopyPathTool {
    type Input = CopyPathToolInput;
    type Output = String;

    const NAME: &'static str = "copy_path";

    fn kind() -> ToolKind {
        ToolKind::Move
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> ui::SharedString {
        if let Ok(input) = input {
            let src = MarkdownInlineCode(&input.source_path);
            let dest = MarkdownInlineCode(&input.destination_path);
            format!("Copy {src} to {dest}").into()
        } else {
            "Copy path".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let settings = AgentSettings::get_global(cx);

        let source_decision = decide_permission_for_path(Self::NAME, &input.source_path, settings);
        if let ToolPermissionDecision::Deny(reason) = source_decision {
            return Task::ready(Err(anyhow!("{}", reason)));
        }

        let dest_decision =
            decide_permission_for_path(Self::NAME, &input.destination_path, settings);
        if let ToolPermissionDecision::Deny(reason) = dest_decision {
            return Task::ready(Err(anyhow!("{}", reason)));
        }

        let needs_confirmation = matches!(source_decision, ToolPermissionDecision::Confirm)
            || matches!(dest_decision, ToolPermissionDecision::Confirm)
            || (!settings.always_allow_tool_actions
                && matches!(source_decision, ToolPermissionDecision::Allow)
                && is_sensitive_settings_path(Path::new(&input.source_path)))
            || (!settings.always_allow_tool_actions
                && matches!(dest_decision, ToolPermissionDecision::Allow)
                && is_sensitive_settings_path(Path::new(&input.destination_path)));

        let authorize = if needs_confirmation {
            let src = MarkdownInlineCode(&input.source_path);
            let dest = MarkdownInlineCode(&input.destination_path);
            let context = crate::ToolPermissionContext {
                tool_name: Self::NAME.to_string(),
                input_value: format!("{}\n{}", input.source_path, input.destination_path),
            };
            let title = format!("Copy {src} to {dest}");
            let sensitive_kind = sensitive_settings_kind(Path::new(&input.source_path))
                .or_else(|| sensitive_settings_kind(Path::new(&input.destination_path)));
            let title = match sensitive_kind {
                Some(SensitiveSettingsKind::Local) => format!("{title} (local settings)"),
                Some(SensitiveSettingsKind::Global) => format!("{title} (settings)"),
                None => title,
            };
            Some(event_stream.authorize(title, context, cx))
        } else {
            None
        };

        let project = self.project.clone();
        cx.spawn(async move |cx| {
            if let Some(authorize) = authorize {
                authorize.await?;
            }

            let copy_task = project.update(cx, |project, cx| {
                match project
                    .find_project_path(&input.source_path, cx)
                    .and_then(|project_path| project.entry_for_path(&project_path, cx))
                {
                    Some(entity) => match project.find_project_path(&input.destination_path, cx) {
                        Some(project_path) => Ok(project.copy_entry(entity.id, project_path, cx)),
                        None => Err(anyhow!(
                            "Destination path {} was outside the project.",
                            input.destination_path
                        )),
                    },
                    None => Err(anyhow!(
                        "Source path {} was not found in the project.",
                        input.source_path
                    )),
                }
            })?;

            let result = futures::select! {
                result = copy_task.fuse() => result,
                _ = event_stream.cancelled_by_user().fuse() => {
                    anyhow::bail!("Copy cancelled by user");
                }
            };
            let _ = result.with_context(|| {
                format!(
                    "Copying {} to {}",
                    input.source_path, input.destination_path
                )
            })?;
            Ok(format!(
                "Copied {} to {}",
                input.source_path, input.destination_path
            ))
        })
    }
}

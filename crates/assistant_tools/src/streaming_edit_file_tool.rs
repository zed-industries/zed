use crate::{
    Templates,
    edit_agent::{EditAgent, EditAgentOutputEvent},
    edit_file_tool::EditFileToolCard,
    schema::json_schema_for,
};
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, AnyToolCard, Tool, ToolResult};
use futures::StreamExt;
use gpui::{AnyWindowHandle, App, AppContext, AsyncApp, Entity, Task};
use language_model::{
    LanguageModelRegistry, LanguageModelRequestMessage, LanguageModelToolSchemaFormat,
};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ui::prelude::*;
use util::ResultExt;

pub struct StreamingEditFileTool;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreamingEditFileToolInput {
    /// A one-line, user-friendly markdown description of the edit. This will be
    /// shown in the UI and also passed to another model to perform the edit.
    ///
    /// Be terse, but also descriptive in what you want to achieve with this
    /// edit. Avoid generic instructions.
    ///
    /// NEVER mention the file path in this description.
    ///
    /// <example>Fix API endpoint URLs</example>
    /// <example>Update copyright year in `page_footer`</example>
    ///
    /// Make sure to include this field before all the others in the input object
    /// so that we can display it immediately.
    pub display_description: String,

    /// The full path of the file to modify in the project.
    ///
    /// WARNING: When specifying which file path need changing, you MUST
    /// start each path with one of the project's root directories.
    ///
    /// The following examples assume we have two root directories in the project:
    /// - backend
    /// - frontend
    ///
    /// <example>
    /// `backend/src/main.rs`
    ///
    /// Notice how the file path starts with root-1. Without that, the path
    /// would be ambiguous and the call would fail!
    /// </example>
    ///
    /// <example>
    /// `frontend/db.js`
    /// </example>
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct PartialInput {
    #[serde(default)]
    path: String,
    #[serde(default)]
    display_description: String,
}

const DEFAULT_UI_TEXT: &str = "Editing file";

impl Tool for StreamingEditFileTool {
    fn name(&self) -> String {
        "edit_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("streaming_edit_file_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Pencil
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<StreamingEditFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<StreamingEditFileToolInput>(input.clone()) {
            Ok(input) => input.display_description,
            Err(_) => "Editing file".to_string(),
        }
    }

    fn still_streaming_ui_text(&self, input: &serde_json::Value) -> String {
        if let Some(input) = serde_json::from_value::<PartialInput>(input.clone()).ok() {
            let description = input.display_description.trim();
            if !description.is_empty() {
                return description.to_string();
            }

            let path = input.path.trim();
            if !path.is_empty() {
                return path.to_string();
            }
        }

        DEFAULT_UI_TEXT.to_string()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<StreamingEditFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!(
                "Path {} not found in project",
                input.path.display()
            )))
            .into();
        };
        let Some(worktree) = project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!("Worktree not found for project path"))).into();
        };
        let exists = worktree.update(cx, |worktree, cx| {
            worktree.file_exists(&project_path.path, cx)
        });

        let card = window.and_then(|window| {
            window
                .update(cx, |_, window, cx| {
                    cx.new(|cx| {
                        EditFileToolCard::new(input.path.clone(), project.clone(), window, cx)
                    })
                })
                .ok()
        });

        let card_clone = card.clone();
        // todo!("read model from settings...")
        let models = LanguageModelRegistry::read_global(cx);
        let model = models
            .available_models(cx)
            .find(|model| model.id().0 == "claude-3-7-sonnet-latest")
            .unwrap();
        let provider = models.provider(&model.provider_id()).unwrap();
        let authenticated = provider.authenticate(cx);
        let messages = messages.to_vec();

        // todo!("reuse templates")
        let edit_agent = EditAgent::new(model, action_log, Templates::new());
        let task = cx.spawn(async move |cx: &mut AsyncApp| {
            authenticated.await?;
            if !exists.await? {
                return Err(anyhow!("{} not found", input.path.display()));
            }

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await?;

            let old_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let old_text = cx
                .background_spawn({
                    let old_snapshot = old_snapshot.clone();
                    async move { old_snapshot.text() }
                })
                .await;

            let (output, mut events) = edit_agent.edit(
                buffer.clone(),
                input.display_description.clone(),
                messages,
                cx,
            );
            while let Some(event) = events.next().await {
                match event {
                    EditAgentOutputEvent::Edited => {
                        if let Some(card) = card_clone.as_ref() {
                            let new_snapshot =
                                buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
                            let new_text = cx
                                .background_spawn({
                                    let new_snapshot = new_snapshot.clone();
                                    async move { new_snapshot.text() }
                                })
                                .await;
                            card.update(cx, |card, cx| {
                                card.set_diff(
                                    project_path.path.clone(),
                                    old_text.clone(),
                                    new_text,
                                    cx,
                                );
                            })
                            .log_err();
                        }
                    }
                    EditAgentOutputEvent::OldTextNotFound(shared_string) => {
                        // todo!()
                    }
                }
            }
            output.await?;

            project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                .await?;

            let new_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let new_text = cx.background_spawn({
                let new_snapshot = new_snapshot.clone();
                async move { new_snapshot.text() }
            });
            let diff = cx.background_spawn(async move {
                language::unified_diff(&old_snapshot.text(), &new_snapshot.text())
            });
            let (new_text, diff) = futures::join!(new_text, diff);

            if let Some(card) = card_clone {
                card.update(cx, |card, cx| {
                    card.set_diff(project_path.path.clone(), old_text, new_text, cx);
                })
                .log_err();
            }

            Ok(format!(
                "Edited {}:\n\n```diff\n{}\n```",
                input.path.display(),
                diff
            ))
        });

        ToolResult {
            output: task,
            card: card.map(AnyToolCard::from),
        }
    }
}

// todo!("add unit tests for failure modes of edit, like file not found, etc.")
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn still_streaming_ui_text_with_path() {
        let input = json!({
            "path": "src/main.rs",
            "display_description": "",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(
            StreamingEditFileTool.still_streaming_ui_text(&input),
            "src/main.rs"
        );
    }

    #[test]
    fn still_streaming_ui_text_with_description() {
        let input = json!({
            "path": "",
            "display_description": "Fix error handling",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(
            StreamingEditFileTool.still_streaming_ui_text(&input),
            "Fix error handling",
        );
    }

    #[test]
    fn still_streaming_ui_text_with_path_and_description() {
        let input = json!({
            "path": "src/main.rs",
            "display_description": "Fix error handling",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(
            StreamingEditFileTool.still_streaming_ui_text(&input),
            "Fix error handling",
        );
    }

    #[test]
    fn still_streaming_ui_text_no_path_or_description() {
        let input = json!({
            "path": "",
            "display_description": "",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(
            StreamingEditFileTool.still_streaming_ui_text(&input),
            DEFAULT_UI_TEXT,
        );
    }

    #[test]
    fn still_streaming_ui_text_with_null() {
        let input = serde_json::Value::Null;

        assert_eq!(
            StreamingEditFileTool.still_streaming_ui_text(&input),
            DEFAULT_UI_TEXT,
        );
    }
}

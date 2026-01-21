use crate::{
    AgentTool, Templates, Thread, ToolCallEventStream, ToolPermissionDecision,
    decide_permission_from_settings, edit_agent::streaming_fuzzy_matcher::StreamingFuzzyMatcher,
};
use acp_thread::Diff;
use agent_client_protocol::{self as acp, ToolCallLocation, ToolCallUpdateFields};
use anyhow::{Context as _, Result, anyhow};
use gpui::{App, AppContext, AsyncApp, Entity, Task, WeakEntity};
use language::{Anchor, LanguageRegistry, ToPoint};
use language_model::LanguageModelToolResultContent;
use paths;
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::ffi::OsStr;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use text::BufferSnapshot;
use ui::SharedString;
use util::rel_path::RelPath;

const DEFAULT_UI_TEXT: &str = "Editing file";

/// This is a tool for creating a new file or editing an existing file. For moving or renaming files, you should generally use the `terminal` tool with the 'mv' command instead.
///
/// Before using this tool:
///
/// 1. Use the `read_file` tool to understand the file's contents and context
///
/// 2. Verify the directory path is correct (only applicable when creating new files):
///    - Use the `list_directory` tool to verify the parent directory exists and is the correct location
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreamingEditFileToolInput {
    /// A one-line, user-friendly markdown description of the edit. This will be shown in the UI.
    ///
    /// Be terse, but also descriptive in what you want to achieve with this edit. Avoid generic instructions.
    ///
    /// NEVER mention the file path in this description.
    ///
    /// <example>Fix API endpoint URLs</example>
    /// <example>Update copyright year in `page_footer`</example>
    ///
    /// Make sure to include this field before all the others in the input object so that we can display it immediately.
    pub display_description: String,

    /// The full path of the file to create or modify in the project.
    ///
    /// WARNING: When specifying which file path need changing, you MUST start each path with one of the project's root directories.
    ///
    /// The following examples assume we have two root directories in the project:
    /// - /a/b/backend
    /// - /c/d/frontend
    ///
    /// <example>
    /// `backend/src/main.rs`
    ///
    /// Notice how the file path starts with `backend`. Without that, the path would be ambiguous and the call would fail!
    /// </example>
    ///
    /// <example>
    /// `frontend/db.js`
    /// </example>
    pub path: PathBuf,

    /// The mode of operation on the file. Possible values:
    /// - 'create': Create a new file if it doesn't exist. Requires 'content' field.
    /// - 'overwrite': Replace the entire contents of an existing file. Requires 'content' field.
    /// - 'edit': Make granular edits to an existing file. Requires 'edits' field.
    ///
    /// When a file already exists or you just created it, prefer editing it as opposed to recreating it from scratch.
    pub mode: StreamingEditFileMode,

    /// The complete content for the new file (required for 'create' and 'overwrite' modes).
    /// This field should contain the entire file content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// List of edit operations to apply sequentially (required for 'edit' mode).
    /// Each edit finds `old_text` in the file and replaces it with `new_text`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edits: Option<Vec<EditOperation>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StreamingEditFileMode {
    /// Create a new file if it doesn't exist
    Create,
    /// Replace the entire contents of an existing file
    Overwrite,
    /// Make granular edits to an existing file
    Edit,
}

/// A single edit operation that replaces old text with new text
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditOperation {
    /// The exact text to find in the file. This will be matched using fuzzy matching
    /// to handle minor differences in whitespace or formatting.
    pub old_text: String,
    /// The text to replace it with
    pub new_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct StreamingEditFileToolPartialInput {
    #[serde(default)]
    path: String,
    #[serde(default)]
    display_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingEditFileToolOutput {
    #[serde(alias = "original_path")]
    input_path: PathBuf,
    new_text: String,
    old_text: Arc<String>,
    #[serde(default)]
    diff: String,
}

impl From<StreamingEditFileToolOutput> for LanguageModelToolResultContent {
    fn from(output: StreamingEditFileToolOutput) -> Self {
        if output.diff.is_empty() {
            "No edits were made.".into()
        } else {
            format!(
                "Edited {}:\n\n```diff\n{}\n```",
                output.input_path.display(),
                output.diff
            )
            .into()
        }
    }
}

pub struct StreamingEditFileTool {
    thread: WeakEntity<Thread>,
    language_registry: Arc<LanguageRegistry>,
    project: Entity<Project>,
    #[allow(dead_code)]
    templates: Arc<Templates>,
}

impl StreamingEditFileTool {
    pub fn new(
        project: Entity<Project>,
        thread: WeakEntity<Thread>,
        language_registry: Arc<LanguageRegistry>,
        templates: Arc<Templates>,
    ) -> Self {
        Self {
            project,
            thread,
            language_registry,
            templates,
        }
    }

    fn authorize(
        &self,
        input: &StreamingEditFileToolInput,
        event_stream: &ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let path_str = input.path.to_string_lossy();
        let settings = agent_settings::AgentSettings::get_global(cx);
        let decision = decide_permission_from_settings(Self::name(), &path_str, settings);

        match decision {
            ToolPermissionDecision::Allow => return Task::ready(Ok(())),
            ToolPermissionDecision::Deny(reason) => {
                return Task::ready(Err(anyhow!("{}", reason)));
            }
            ToolPermissionDecision::Confirm => {}
        }

        let local_settings_folder = paths::local_settings_folder_name();
        let path = Path::new(&input.path);
        if path.components().any(|component| {
            component.as_os_str() == <_ as AsRef<OsStr>>::as_ref(&local_settings_folder)
        }) {
            let context = crate::ToolPermissionContext {
                tool_name: "edit_file".to_string(),
                input_value: path_str.to_string(),
            };
            return event_stream.authorize(
                format!("{} (local settings)", input.display_description),
                context,
                cx,
            );
        }

        if let Ok(canonical_path) = std::fs::canonicalize(&input.path)
            && canonical_path.starts_with(paths::config_dir())
        {
            let context = crate::ToolPermissionContext {
                tool_name: "edit_file".to_string(),
                input_value: path_str.to_string(),
            };
            return event_stream.authorize(
                format!("{} (global settings)", input.display_description),
                context,
                cx,
            );
        }

        let Ok(project_path) = self.thread.read_with(cx, |thread, cx| {
            thread.project().read(cx).find_project_path(&input.path, cx)
        }) else {
            return Task::ready(Err(anyhow!("thread was dropped")));
        };

        if project_path.is_some() {
            Task::ready(Ok(()))
        } else {
            let context = crate::ToolPermissionContext {
                tool_name: "edit_file".to_string(),
                input_value: path_str.to_string(),
            };
            event_stream.authorize(&input.display_description, context, cx)
        }
    }
}

impl AgentTool for StreamingEditFileTool {
    type Input = StreamingEditFileToolInput;
    type Output = StreamingEditFileToolOutput;

    fn name() -> &'static str {
        "streaming_edit_file"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Edit
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => self
                .project
                .read(cx)
                .find_project_path(&input.path, cx)
                .and_then(|project_path| {
                    self.project
                        .read(cx)
                        .short_full_path_for_project_path(&project_path, cx)
                })
                .unwrap_or(input.path.to_string_lossy().into_owned())
                .into(),
            Err(raw_input) => {
                if let Some(input) =
                    serde_json::from_value::<StreamingEditFileToolPartialInput>(raw_input).ok()
                {
                    let path = input.path.trim();
                    if !path.is_empty() {
                        return self
                            .project
                            .read(cx)
                            .find_project_path(&input.path, cx)
                            .and_then(|project_path| {
                                self.project
                                    .read(cx)
                                    .short_full_path_for_project_path(&project_path, cx)
                            })
                            .unwrap_or(input.path)
                            .into();
                    }

                    let description = input.display_description.trim();
                    if !description.is_empty() {
                        return description.to_string().into();
                    }
                }

                DEFAULT_UI_TEXT.into()
            }
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let Ok(project) = self
            .thread
            .read_with(cx, |thread, _cx| thread.project().clone())
        else {
            return Task::ready(Err(anyhow!("thread was dropped")));
        };

        let project_path = match resolve_path(&input, project.clone(), cx) {
            Ok(path) => path,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let abs_path = project.read(cx).absolute_path(&project_path, cx);
        if let Some(abs_path) = abs_path.clone() {
            event_stream.update_fields(
                ToolCallUpdateFields::new().locations(vec![acp::ToolCallLocation::new(abs_path)]),
            );
        }

        let authorize = self.authorize(&input, &event_stream, cx);

        cx.spawn(async move |cx: &mut AsyncApp| {
            authorize.await?;

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })
                .await?;

            if let Some(abs_path) = abs_path.as_ref() {
                let (last_read_mtime, current_mtime, is_dirty, has_save_tool, has_restore_tool) =
                    self.thread.update(cx, |thread, cx| {
                        let last_read = thread.file_read_times.get(abs_path).copied();
                        let current = buffer
                            .read(cx)
                            .file()
                            .and_then(|file| file.disk_state().mtime());
                        let dirty = buffer.read(cx).is_dirty();
                        let has_save = thread.has_tool("save_file");
                        let has_restore = thread.has_tool("restore_file_from_disk");
                        (last_read, current, dirty, has_save, has_restore)
                    })?;

                if is_dirty {
                    let message = match (has_save_tool, has_restore_tool) {
                        (true, true) => {
                            "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                             If they want to keep them, ask for confirmation then use the save_file tool to save the file, then retry this edit. \
                             If they want to discard them, ask for confirmation then use the restore_file_from_disk tool to restore the on-disk contents, then retry this edit."
                        }
                        (true, false) => {
                            "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                             If they want to keep them, ask for confirmation then use the save_file tool to save the file, then retry this edit. \
                             If they want to discard them, ask the user to manually revert the file, then inform you when it's ok to proceed."
                        }
                        (false, true) => {
                            "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                             If they want to keep them, ask the user to manually save the file, then inform you when it's ok to proceed. \
                             If they want to discard them, ask for confirmation then use the restore_file_from_disk tool to restore the on-disk contents, then retry this edit."
                        }
                        (false, false) => {
                            "This file has unsaved changes. Ask the user whether they want to keep or discard those changes, \
                             then ask them to save or revert the file manually and inform you when it's ok to proceed."
                        }
                    };
                    anyhow::bail!("{}", message);
                }

                if let (Some(last_read), Some(current)) = (last_read_mtime, current_mtime) {
                    if current != last_read {
                        anyhow::bail!(
                            "The file {} has been modified since you last read it. \
                             Please read the file again to get the current state before editing it.",
                            input.path.display()
                        );
                    }
                }
            }

            let diff = cx.new(|cx| Diff::new(buffer.clone(), cx));
            event_stream.update_diff(diff.clone());
            let _finalize_diff = util::defer({
                let diff = diff.downgrade();
                let mut cx = cx.clone();
                move || {
                    diff.update(&mut cx, |diff, cx| diff.finalize(cx)).ok();
                }
            });

            let old_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
            let old_text = cx
                .background_spawn({
                    let old_snapshot = old_snapshot.clone();
                    async move { Arc::new(old_snapshot.text()) }
                })
                .await;

            match input.mode {
                StreamingEditFileMode::Create | StreamingEditFileMode::Overwrite => {
                    let content = input.content.ok_or_else(|| {
                        anyhow!("'content' field is required for create and overwrite modes")
                    })?;
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit([(0..buffer.len(), content.as_str())], None, cx);
                    });
                }
                StreamingEditFileMode::Edit => {
                    let edits = input.edits.ok_or_else(|| {
                        anyhow!("'edits' field is required for edit mode")
                    })?;
                    apply_edits(&buffer, &edits, &diff, &event_stream, &abs_path, cx)?;
                }
            }

            let action_log = self.thread.read_with(cx, |thread, _cx| thread.action_log().clone())?;

            action_log.update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), cx);
            });

            project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
                .await?;

            action_log.update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), cx);
            });

            if let Some(abs_path) = abs_path.as_ref() {
                if let Some(new_mtime) = buffer.read_with(cx, |buffer, _| {
                    buffer.file().and_then(|file| file.disk_state().mtime())
                }) {
                    self.thread.update(cx, |thread, _| {
                        thread.file_read_times.insert(abs_path.to_path_buf(), new_mtime);
                    })?;
                }
            }

            let new_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
            let (new_text, unified_diff) = cx
                .background_spawn({
                    let new_snapshot = new_snapshot.clone();
                    let old_text = old_text.clone();
                    async move {
                        let new_text = new_snapshot.text();
                        let diff = language::unified_diff(&old_text, &new_text);
                        (new_text, diff)
                    }
                })
                .await;

            let output = StreamingEditFileToolOutput {
                input_path: input.path,
                new_text,
                old_text,
                diff: unified_diff,
            };

            Ok(output)
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Result<()> {
        event_stream.update_diff(cx.new(|cx| {
            Diff::finalized(
                output.input_path.to_string_lossy().into_owned(),
                Some(output.old_text.to_string()),
                output.new_text,
                self.language_registry.clone(),
                cx,
            )
        }));
        Ok(())
    }
}

fn apply_edits(
    buffer: &Entity<language::Buffer>,
    edits: &[EditOperation],
    diff: &Entity<Diff>,
    event_stream: &ToolCallEventStream,
    abs_path: &Option<PathBuf>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let mut emitted_location = false;
    let mut failed_edits = Vec::new();
    let mut ambiguous_edits = Vec::new();

    for (index, edit) in edits.iter().enumerate() {
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
        let result = apply_single_edit(buffer, &snapshot, edit, diff, cx);

        match result {
            Ok(Some(range)) => {
                if !emitted_location {
                    let line = buffer.update(cx, |buffer, _cx| {
                        range.start.to_point(&buffer.snapshot()).row
                    });
                    if let Some(abs_path) = abs_path.clone() {
                        event_stream.update_fields(
                            ToolCallUpdateFields::new()
                                .locations(vec![ToolCallLocation::new(abs_path).line(Some(line))]),
                        );
                    }
                    emitted_location = true;
                }
            }
            Ok(None) => {
                failed_edits.push(index);
            }
            Err(ranges) => {
                ambiguous_edits.push((index, ranges));
            }
        }
    }

    if !failed_edits.is_empty() {
        let indices = failed_edits
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        anyhow::bail!(
            "Could not find matching text for edit(s) at index(es): {}. \
             The old_text did not match any content in the file. \
             Please read the file again to get the current content.",
            indices
        );
    }

    if !ambiguous_edits.is_empty() {
        let details: Vec<String> = ambiguous_edits
            .iter()
            .map(|(index, ranges)| {
                let lines = ranges
                    .iter()
                    .map(|r| r.start.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("edit {}: matches at lines {}", index, lines)
            })
            .collect();
        anyhow::bail!(
            "Some edits matched multiple locations in the file:\n{}. \
             Please provide more context in old_text to uniquely identify the location.",
            details.join("\n")
        );
    }

    Ok(())
}

fn apply_single_edit(
    buffer: &Entity<language::Buffer>,
    snapshot: &BufferSnapshot,
    edit: &EditOperation,
    diff: &Entity<Diff>,
    cx: &mut AsyncApp,
) -> std::result::Result<Option<Range<Anchor>>, Vec<Range<usize>>> {
    let mut matcher = StreamingFuzzyMatcher::new(snapshot.clone());
    matcher.push(&edit.old_text, None);
    let matches = matcher.finish();

    if matches.is_empty() {
        return Ok(None);
    }

    if matches.len() > 1 {
        return Err(matches);
    }

    let match_range = matches.into_iter().next().expect("checked len above");

    let start_anchor = buffer.read_with(cx, |buffer, _cx| buffer.anchor_before(match_range.start));
    let end_anchor = buffer.read_with(cx, |buffer, _cx| buffer.anchor_after(match_range.end));

    diff.update(cx, |card, cx| {
        card.reveal_range(start_anchor..end_anchor, cx)
    });

    buffer.update(cx, |buffer, cx| {
        buffer.edit([(match_range.clone(), edit.new_text.as_str())], None, cx);
    });

    let new_end = buffer.read_with(cx, |buffer, _cx| {
        buffer.anchor_after(match_range.start + edit.new_text.len())
    });

    Ok(Some(start_anchor..new_end))
}

fn resolve_path(
    input: &StreamingEditFileToolInput,
    project: Entity<Project>,
    cx: &mut App,
) -> Result<ProjectPath> {
    let project = project.read(cx);

    match input.mode {
        StreamingEditFileMode::Edit | StreamingEditFileMode::Overwrite => {
            let path = project
                .find_project_path(&input.path, cx)
                .context("Can't edit file: path not found")?;

            let entry = project
                .entry_for_path(&path, cx)
                .context("Can't edit file: path not found")?;

            anyhow::ensure!(entry.is_file(), "Can't edit file: path is a directory");
            Ok(path)
        }

        StreamingEditFileMode::Create => {
            if let Some(path) = project.find_project_path(&input.path, cx) {
                anyhow::ensure!(
                    project.entry_for_path(&path, cx).is_none(),
                    "Can't create file: file already exists"
                );
            }

            let parent_path = input
                .path
                .parent()
                .context("Can't create file: incorrect path")?;

            let parent_project_path = project.find_project_path(&parent_path, cx);

            let parent_entry = parent_project_path
                .as_ref()
                .and_then(|path| project.entry_for_path(path, cx))
                .context("Can't create file: parent directory doesn't exist")?;

            anyhow::ensure!(
                parent_entry.is_dir(),
                "Can't create file: parent is not a directory"
            );

            let file_name = input
                .path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .and_then(|file_name| RelPath::unix(file_name).ok())
                .context("Can't create file: invalid filename")?;

            let new_file_path = parent_project_path.map(|parent| ProjectPath {
                path: parent.path.join(file_name),
                ..parent
            });

            new_file_path.context("Can't create file")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContextServerRegistry, Templates};
    use gpui::TestAppContext;
    use language_model::fake_provider::FakeLanguageModel;
    use prompt_store::ProjectContext;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_streaming_edit_create_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"dir": {}})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Create new file".into(),
                    path: "root/dir/new_file.txt".into(),
                    mode: StreamingEditFileMode::Create,
                    content: Some("Hello, World!".into()),
                    edits: None,
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.new_text, "Hello, World!");
        assert!(!output.diff.is_empty());
    }

    #[gpui::test]
    async fn test_streaming_edit_overwrite_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"file.txt": "old content"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Overwrite file".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Overwrite,
                    content: Some("new content".into()),
                    edits: None,
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.new_text, "new content");
        assert_eq!(*output.old_text, "old content");
    }

    #[gpui::test]
    async fn test_streaming_edit_granular_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Edit lines".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![EditOperation {
                        old_text: "line 2".into(),
                        new_text: "modified line 2".into(),
                    }]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_edit_nonexistent_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Some edit".into(),
                    path: "root/nonexistent_file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![EditOperation {
                        old_text: "foo".into(),
                        new_text: "bar".into(),
                    }]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project,
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert_eq!(
            result.unwrap_err().to_string(),
            "Can't edit file: path not found"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_failed_match(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"file.txt": "hello world"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Edit file".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![EditOperation {
                        old_text: "nonexistent text that is not in the file".into(),
                        new_text: "replacement".into(),
                    }]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project,
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Could not find matching text")
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }
}

use acp_thread::Diff;
use agent_client_protocol as acp;
use anyhow::{anyhow, Context as _, Result};
use assistant_tools::edit_agent::{EditAgent, EditAgentOutputEvent, EditFormat};
use cloud_llm_client::CompletionIntent;
use collections::HashSet;
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use indoc::formatdoc;
use language::language_settings::{self, FormatOnSave};
use paths;
use project::lsp_store::{FormatTrigger, LspFormatTarget};
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use smol::stream::StreamExt as _;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use ui::SharedString;
use util::ResultExt;

use crate::{AgentTool, Thread, ToolCallEventStream};

/// This is a tool for creating a new file or editing an existing file. For moving or renaming files, you should generally use the `terminal` tool with the 'mv' command instead.
///
/// Before using this tool:
///
/// 1. Use the `read_file` tool to understand the file's contents and context
///
/// 2. Verify the directory path is correct (only applicable when creating new files):
///    - Use the `list_directory` tool to verify the parent directory exists and is the correct location
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFileToolInput {
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

    /// The full path of the file to create or modify in the project.
    ///
    /// WARNING: When specifying which file path need changing, you MUST
    /// start each path with one of the project's root directories.
    ///
    /// The following examples assume we have two root directories in the project:
    /// - /a/b/backend
    /// - /c/d/frontend
    ///
    /// <example>
    /// `backend/src/main.rs`
    ///
    /// Notice how the file path starts with `backend`. Without that, the path
    /// would be ambiguous and the call would fail!
    /// </example>
    ///
    /// <example>
    /// `frontend/db.js`
    /// </example>
    pub path: PathBuf,

    /// The mode of operation on the file. Possible values:
    /// - 'edit': Make granular edits to an existing file.
    /// - 'create': Create a new file if it doesn't exist.
    /// - 'overwrite': Replace the entire contents of an existing file.
    ///
    /// When a file already exists or you just created it, prefer editing
    /// it as opposed to recreating it from scratch.
    pub mode: EditFileMode,

    /// The new content for the file (required for create and overwrite modes)
    /// For edit mode, this field is not used - edits happen through the edit agent
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum EditFileMode {
    Edit,
    Create,
    Overwrite,
}

pub struct EditFileTool {
    project: Entity<Project>,
    thread: Entity<Thread>,
}

impl EditFileTool {
    pub fn new(project: Entity<Project>, thread: Entity<Thread>) -> Self {
        Self { project, thread }
    }

    fn authorize(
        &self,
        input: &EditFileToolInput,
        event_stream: &ToolCallEventStream,
        cx: &App,
    ) -> Task<Result<()>> {
        if agent_settings::AgentSettings::get_global(cx).always_allow_tool_actions {
            return Task::ready(Ok(()));
        }

        // If any path component matches the local settings folder, then this could affect
        // the editor in ways beyond the project source, so prompt.
        let local_settings_folder = paths::local_settings_folder_relative_path();
        let path = Path::new(&input.path);
        if path
            .components()
            .any(|component| component.as_os_str() == local_settings_folder.as_os_str())
        {
            return cx.foreground_executor().spawn(
                event_stream.authorize(format!("{} (local settings)", input.display_description)),
            );
        }

        // It's also possible that the global config dir is configured to be inside the project,
        // so check for that edge case too.
        if let Ok(canonical_path) = std::fs::canonicalize(&input.path) {
            if canonical_path.starts_with(paths::config_dir()) {
                return cx.foreground_executor().spawn(
                    event_stream
                        .authorize(format!("{} (global settings)", input.display_description)),
                );
            }
        }

        // Check if path is inside the global config directory
        // First check if it's already inside project - if not, try to canonicalize
        let project_path = self.project.read(cx).find_project_path(&input.path, cx);

        // If the path is inside the project, and it's not one of the above edge cases,
        // then no confirmation is necessary. Otherwise, confirmation is necessary.
        if project_path.is_some() {
            Task::ready(Ok(()))
        } else {
            cx.foreground_executor()
                .spawn(event_stream.authorize(input.display_description.clone()))
        }
    }
}

impl AgentTool for EditFileTool {
    type Input = EditFileToolInput;

    fn name(&self) -> SharedString {
        "edit_file".into()
    }

    fn kind(&self) -> acp::ToolKind {
        acp::ToolKind::Edit
    }

    fn initial_title(&self, input: Self::Input) -> SharedString {
        input.display_description.into()
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let project_path = match resolve_path(&input, self.project.clone(), cx) {
            Ok(path) => path,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let project = self.project.clone();
        let request = self.thread.update(cx, |thread, cx| {
            thread.build_completion_request(CompletionIntent::ToolResults, cx)
        });
        let thread = self.thread.read(cx);
        let model = thread.selected_model.clone();
        let action_log = thread.action_log().clone();

        let authorize = self.authorize(&input, &event_stream, cx);
        cx.spawn(async move |cx: &mut AsyncApp| {
            authorize.await?;

            let edit_format = EditFormat::from_model(model.clone())?;
            let edit_agent = EditAgent::new(
                model,
                project.clone(),
                action_log.clone(),
                // todo! move edit agent to this crate so we can use our templates?
                assistant_tools::templates::Templates::new(),
                edit_format,
            );

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await?;

            let diff = cx.new(|cx| Diff::new(buffer.clone(), cx))?;
            event_stream.send_update(acp::ToolCallUpdateFields {
                locations: Some(vec![acp::ToolCallLocation {
                    path: project_path.path.to_path_buf(),
                    // todo!
                    line: None
                }]),
                ..Default::default()
            });
            event_stream.send_diff(diff.clone());

            let old_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let old_text = cx
                .background_spawn({
                    let old_snapshot = old_snapshot.clone();
                    async move { Arc::new(old_snapshot.text()) }
                })
                .await;


            let (output, mut events) = if matches!(input.mode, EditFileMode::Edit) {
                edit_agent.edit(
                    buffer.clone(),
                    input.display_description.clone(),
                    &request,
                    cx,
                )
            } else {
                edit_agent.overwrite(
                    buffer.clone(),
                    input.display_description.clone(),
                    &request,
                    cx,
                )
            };

            let mut hallucinated_old_text = false;
            let mut ambiguous_ranges = Vec::new();
            while let Some(event) = events.next().await {
                match event {
                    EditAgentOutputEvent::Edited => {},
                    EditAgentOutputEvent::UnresolvedEditRange => hallucinated_old_text = true,
                    EditAgentOutputEvent::AmbiguousEditRange(ranges) => ambiguous_ranges = ranges,
                    EditAgentOutputEvent::ResolvingEditRange(range) => {
                        diff.update(cx, |card, cx| card.reveal_range(range, cx))?;
                    }
                }
            }

            // If format_on_save is enabled, format the buffer
            let format_on_save_enabled = buffer
                .read_with(cx, |buffer, cx| {
                    let settings = language_settings::language_settings(
                        buffer.language().map(|l| l.name()),
                        buffer.file(),
                        cx,
                    );
                    settings.format_on_save != FormatOnSave::Off
                })
                .unwrap_or(false);

            let _ = output.await?;

            if format_on_save_enabled {
                action_log.update(cx, |log, cx| {
                    log.buffer_edited(buffer.clone(), cx);
                })?;

                let format_task = project.update(cx, |project, cx| {
                    project.format(
                        HashSet::from_iter([buffer.clone()]),
                        LspFormatTarget::Buffers,
                        false, // Don't push to history since the tool did it.
                        FormatTrigger::Save,
                        cx,
                    )
                })?;
                format_task.await.log_err();
            }

            project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                .await?;

            action_log.update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), cx);
            })?;

            let new_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let unified_diff = cx
                .background_spawn({
                    let new_snapshot = new_snapshot.clone();
                    let old_text = old_text.clone();
                    async move {
                        let new_text = new_snapshot.text();
                        language::unified_diff(&old_text, &new_text)
                    }
                })
                .await;

            println!("\n\n{}\n\n", unified_diff);

            diff.update(cx, |diff, cx| {
                diff.finalize(cx);
            }).ok();

            let input_path = input.path.display();
            if unified_diff.is_empty() {
                anyhow::ensure!(
                    !hallucinated_old_text,
                    formatdoc! {"
                        Some edits were produced but none of them could be applied.
                        Read the relevant sections of {input_path} again so that
                        I can perform the requested edits.
                    "}
                );
                anyhow::ensure!(
                    ambiguous_ranges.is_empty(),
                    {
                        let line_numbers = ambiguous_ranges
                            .iter()
                            .map(|range| range.start.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        formatdoc! {"
                            <old_text> matches more than one position in the file (lines: {line_numbers}). Read the
                            relevant sections of {input_path} again and extend <old_text> so
                            that I can perform the requested edits.
                        "}
                    }
                );
                Ok("No edits were made.".into())
            } else {
                Ok(format!(
                    "Edited {}:\n\n```diff\n{}\n```",
                    input_path, unified_diff
                ))
            }
        })
    }
}

/// Validate that the file path is valid, meaning:
///
/// - For `edit` and `overwrite`, the path must point to an existing file.
/// - For `create`, the file must not already exist, but it's parent dir must exist.
fn resolve_path(
    input: &EditFileToolInput,
    project: Entity<Project>,
    cx: &mut App,
) -> Result<ProjectPath> {
    let project = project.read(cx);

    match input.mode {
        EditFileMode::Edit | EditFileMode::Overwrite => {
            let path = project
                .find_project_path(&input.path, cx)
                .context("Can't edit file: path not found")?;

            let entry = project
                .entry_for_path(&path, cx)
                .context("Can't edit file: path not found")?;

            anyhow::ensure!(entry.is_file(), "Can't edit file: path is a directory");
            Ok(path)
        }

        EditFileMode::Create => {
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
                .and_then(|path| project.entry_for_path(&path, cx))
                .context("Can't create file: parent directory doesn't exist")?;

            anyhow::ensure!(
                parent_entry.is_dir(),
                "Can't create file: parent is not a directory"
            );

            let file_name = input
                .path
                .file_name()
                .context("Can't create file: invalid filename")?;

            let new_file_path = parent_project_path.map(|parent| ProjectPath {
                path: Arc::from(parent.path.join(file_name)),
                ..parent
            });

            new_file_path.context("Can't create file")
        }
    }
}

// todo! restore tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ::fs::Fs;
//     use client::TelemetrySettings;
//     use gpui::{TestAppContext, UpdateGlobal};
//     use language_model::fake_provider::FakeLanguageModel;
//     use serde_json::json;
//     use settings::SettingsStore;
//     use std::fs;
//     use util::path;

//     #[gpui::test]
//     async fn test_edit_nonexistent_file(cx: &mut TestAppContext) {
//         init_test(cx);

//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree("/root", json!({})).await;
//         let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
//         let action_log = cx.new(|_| ActionLog::new(project.clone()));
//         let model = Arc::new(FakeLanguageModel::default());
//         let result = cx
//             .update(|cx| {
//                 let input = serde_json::to_value(EditFileToolInput {
//                     display_description: "Some edit".into(),
//                     path: "root/nonexistent_file.txt".into(),
//                     mode: EditFileMode::Edit,
//                 })
//                 .unwrap();
//                 Arc::new(EditFileTool)
//                     .run(
//                         input,
//                         Arc::default(),
//                         project.clone(),
//                         action_log,
//                         model,
//                         None,
//                         cx,
//                     )
//                     .output
//             })
//             .await;
//         assert_eq!(
//             result.unwrap_err().to_string(),
//             "Can't edit file: path not found"
//         );
//     }

//     #[gpui::test]
//     async fn test_resolve_path_for_creating_file(cx: &mut TestAppContext) {
//         let mode = &EditFileMode::Create;

//         let result = test_resolve_path(mode, "root/new.txt", cx);
//         assert_resolved_path_eq(result.await, "new.txt");

//         let result = test_resolve_path(mode, "new.txt", cx);
//         assert_resolved_path_eq(result.await, "new.txt");

//         let result = test_resolve_path(mode, "dir/new.txt", cx);
//         assert_resolved_path_eq(result.await, "dir/new.txt");

//         let result = test_resolve_path(mode, "root/dir/subdir/existing.txt", cx);
//         assert_eq!(
//             result.await.unwrap_err().to_string(),
//             "Can't create file: file already exists"
//         );

//         let result = test_resolve_path(mode, "root/dir/nonexistent_dir/new.txt", cx);
//         assert_eq!(
//             result.await.unwrap_err().to_string(),
//             "Can't create file: parent directory doesn't exist"
//         );
//     }

//     #[gpui::test]
//     async fn test_resolve_path_for_editing_file(cx: &mut TestAppContext) {
//         let mode = &EditFileMode::Edit;

//         let path_with_root = "root/dir/subdir/existing.txt";
//         let path_without_root = "dir/subdir/existing.txt";
//         let result = test_resolve_path(mode, path_with_root, cx);
//         assert_resolved_path_eq(result.await, path_without_root);

//         let result = test_resolve_path(mode, path_without_root, cx);
//         assert_resolved_path_eq(result.await, path_without_root);

//         let result = test_resolve_path(mode, "root/nonexistent.txt", cx);
//         assert_eq!(
//             result.await.unwrap_err().to_string(),
//             "Can't edit file: path not found"
//         );

//         let result = test_resolve_path(mode, "root/dir", cx);
//         assert_eq!(
//             result.await.unwrap_err().to_string(),
//             "Can't edit file: path is a directory"
//         );
//     }

//     async fn test_resolve_path(
//         mode: &EditFileMode,
//         path: &str,
//         cx: &mut TestAppContext,
//     ) -> anyhow::Result<ProjectPath> {
//         init_test(cx);

//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree(
//             "/root",
//             json!({
//                 "dir": {
//                     "subdir": {
//                         "existing.txt": "hello"
//                     }
//                 }
//             }),
//         )
//         .await;
//         let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

//         let input = EditFileToolInput {
//             display_description: "Some edit".into(),
//             path: path.into(),
//             mode: mode.clone(),
//         };

//         let result = cx.update(|cx| resolve_path(&input, project, cx));
//         result
//     }

//     fn assert_resolved_path_eq(path: anyhow::Result<ProjectPath>, expected: &str) {
//         let actual = path
//             .expect("Should return valid path")
//             .path
//             .to_str()
//             .unwrap()
//             .replace("\\", "/"); // Naive Windows paths normalization
//         assert_eq!(actual, expected);
//     }

//     #[test]
//     fn still_streaming_ui_text_with_path() {
//         let input = json!({
//             "path": "src/main.rs",
//             "display_description": "",
//             "old_string": "old code",
//             "new_string": "new code"
//         });

//         assert_eq!(EditFileTool.still_streaming_ui_text(&input), "src/main.rs");
//     }

//     #[test]
//     fn still_streaming_ui_text_with_description() {
//         let input = json!({
//             "path": "",
//             "display_description": "Fix error handling",
//             "old_string": "old code",
//             "new_string": "new code"
//         });

//         assert_eq!(
//             EditFileTool.still_streaming_ui_text(&input),
//             "Fix error handling",
//         );
//     }

//     #[test]
//     fn still_streaming_ui_text_with_path_and_description() {
//         let input = json!({
//             "path": "src/main.rs",
//             "display_description": "Fix error handling",
//             "old_string": "old code",
//             "new_string": "new code"
//         });

//         assert_eq!(
//             EditFileTool.still_streaming_ui_text(&input),
//             "Fix error handling",
//         );
//     }

//     #[test]
//     fn still_streaming_ui_text_no_path_or_description() {
//         let input = json!({
//             "path": "",
//             "display_description": "",
//             "old_string": "old code",
//             "new_string": "new code"
//         });

//         assert_eq!(
//             EditFileTool.still_streaming_ui_text(&input),
//             DEFAULT_UI_TEXT,
//         );
//     }

//     #[test]
//     fn still_streaming_ui_text_with_null() {
//         let input = serde_json::Value::Null;

//         assert_eq!(
//             EditFileTool.still_streaming_ui_text(&input),
//             DEFAULT_UI_TEXT,
//         );
//     }

//     fn init_test(cx: &mut TestAppContext) {
//         cx.update(|cx| {
//             let settings_store = SettingsStore::test(cx);
//             cx.set_global(settings_store);
//             language::init(cx);
//             TelemetrySettings::register(cx);
//             agent_settings::AgentSettings::register(cx);
//             Project::init_settings(cx);
//         });
//     }

//     fn init_test_with_config(cx: &mut TestAppContext, data_dir: &Path) {
//         cx.update(|cx| {
//             // Set custom data directory (config will be under data_dir/config)
//             paths::set_custom_data_dir(data_dir.to_str().unwrap());

//             let settings_store = SettingsStore::test(cx);
//             cx.set_global(settings_store);
//             language::init(cx);
//             TelemetrySettings::register(cx);
//             agent_settings::AgentSettings::register(cx);
//             Project::init_settings(cx);
//         });
//     }

//     #[gpui::test]
//     async fn test_format_on_save(cx: &mut TestAppContext) {
//         init_test(cx);

//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree("/root", json!({"src": {}})).await;

//         let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

//         // Set up a Rust language with LSP formatting support
//         let rust_language = Arc::new(language::Language::new(
//             language::LanguageConfig {
//                 name: "Rust".into(),
//                 matcher: language::LanguageMatcher {
//                     path_suffixes: vec!["rs".to_string()],
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             None,
//         ));

//         // Register the language and fake LSP
//         let language_registry = project.read_with(cx, |project, _| project.languages().clone());
//         language_registry.add(rust_language);

//         let mut fake_language_servers = language_registry.register_fake_lsp(
//             "Rust",
//             language::FakeLspAdapter {
//                 capabilities: lsp::ServerCapabilities {
//                     document_formatting_provider: Some(lsp::OneOf::Left(true)),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         );

//         // Create the file
//         fs.save(
//             path!("/root/src/main.rs").as_ref(),
//             &"initial content".into(),
//             language::LineEnding::Unix,
//         )
//         .await
//         .unwrap();

//         // Open the buffer to trigger LSP initialization
//         let buffer = project
//             .update(cx, |project, cx| {
//                 project.open_local_buffer(path!("/root/src/main.rs"), cx)
//             })
//             .await
//             .unwrap();

//         // Register the buffer with language servers
//         let _handle = project.update(cx, |project, cx| {
//             project.register_buffer_with_language_servers(&buffer, cx)
//         });

//         const UNFORMATTED_CONTENT: &str = "fn main() {println!(\"Hello!\");}\n";
//         const FORMATTED_CONTENT: &str =
//             "This file was formatted by the fake formatter in the test.\n";

//         // Get the fake language server and set up formatting handler
//         let fake_language_server = fake_language_servers.next().await.unwrap();
//         fake_language_server.set_request_handler::<lsp::request::Formatting, _, _>({
//             |_, _| async move {
//                 Ok(Some(vec![lsp::TextEdit {
//                     range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(1, 0)),
//                     new_text: FORMATTED_CONTENT.to_string(),
//                 }]))
//             }
//         });

//         let action_log = cx.new(|_| ActionLog::new(project.clone()));
//         let model = Arc::new(FakeLanguageModel::default());

//         // First, test with format_on_save enabled
//         cx.update(|cx| {
//             SettingsStore::update_global(cx, |store, cx| {
//                 store.update_user_settings::<language::language_settings::AllLanguageSettings>(
//                     cx,
//                     |settings| {
//                         settings.defaults.format_on_save = Some(FormatOnSave::On);
//                         settings.defaults.formatter =
//                             Some(language::language_settings::SelectedFormatter::Auto);
//                     },
//                 );
//             });
//         });

//         // Have the model stream unformatted content
//         let edit_result = {
//             let edit_task = cx.update(|cx| {
//                 let input = serde_json::to_value(EditFileToolInput {
//                     display_description: "Create main function".into(),
//                     path: "root/src/main.rs".into(),
//                     mode: EditFileMode::Overwrite,
//                 })
//                 .unwrap();
//                 Arc::new(EditFileTool)
//                     .run(
//                         input,
//                         Arc::default(),
//                         project.clone(),
//                         action_log.clone(),
//                         model.clone(),
//                         None,
//                         cx,
//                     )
//                     .output
//             });

//             // Stream the unformatted content
//             cx.executor().run_until_parked();
//             model.send_last_completion_stream_text_chunk(UNFORMATTED_CONTENT.to_string());
//             model.end_last_completion_stream();

//             edit_task.await
//         };
//         assert!(edit_result.is_ok());

//         // Wait for any async operations (e.g. formatting) to complete
//         cx.executor().run_until_parked();

//         // Read the file to verify it was formatted automatically
//         let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
//         assert_eq!(
//             // Ignore carriage returns on Windows
//             new_content.replace("\r\n", "\n"),
//             FORMATTED_CONTENT,
//             "Code should be formatted when format_on_save is enabled"
//         );

//         let stale_buffer_count = action_log.read_with(cx, |log, cx| log.stale_buffers(cx).count());

//         assert_eq!(
//             stale_buffer_count, 0,
//             "BUG: Buffer is incorrectly marked as stale after format-on-save. Found {} stale buffers. \
//              This causes the agent to think the file was modified externally when it was just formatted.",
//             stale_buffer_count
//         );

//         // Next, test with format_on_save disabled
//         cx.update(|cx| {
//             SettingsStore::update_global(cx, |store, cx| {
//                 store.update_user_settings::<language::language_settings::AllLanguageSettings>(
//                     cx,
//                     |settings| {
//                         settings.defaults.format_on_save = Some(FormatOnSave::Off);
//                     },
//                 );
//             });
//         });

//         // Stream unformatted edits again
//         let edit_result = {
//             let edit_task = cx.update(|cx| {
//                 let input = serde_json::to_value(EditFileToolInput {
//                     display_description: "Update main function".into(),
//                     path: "root/src/main.rs".into(),
//                     mode: EditFileMode::Overwrite,
//                 })
//                 .unwrap();
//                 Arc::new(EditFileTool)
//                     .run(
//                         input,
//                         Arc::default(),
//                         project.clone(),
//                         action_log.clone(),
//                         model.clone(),
//                         None,
//                         cx,
//                     )
//                     .output
//             });

//             // Stream the unformatted content
//             cx.executor().run_until_parked();
//             model.send_last_completion_stream_text_chunk(UNFORMATTED_CONTENT.to_string());
//             model.end_last_completion_stream();

//             edit_task.await
//         };
//         assert!(edit_result.is_ok());

//         // Wait for any async operations (e.g. formatting) to complete
//         cx.executor().run_until_parked();

//         // Verify the file was not formatted
//         let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
//         assert_eq!(
//             // Ignore carriage returns on Windows
//             new_content.replace("\r\n", "\n"),
//             UNFORMATTED_CONTENT,
//             "Code should not be formatted when format_on_save is disabled"
//         );
//     }

//     #[gpui::test]
//     async fn test_remove_trailing_whitespace(cx: &mut TestAppContext) {
//         init_test(cx);

//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree("/root", json!({"src": {}})).await;

//         // Create a simple file with trailing whitespace
//         fs.save(
//             path!("/root/src/main.rs").as_ref(),
//             &"initial content".into(),
//             language::LineEnding::Unix,
//         )
//         .await
//         .unwrap();

//         let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
//         let action_log = cx.new(|_| ActionLog::new(project.clone()));
//         let model = Arc::new(FakeLanguageModel::default());

//         // First, test with remove_trailing_whitespace_on_save enabled
//         cx.update(|cx| {
//             SettingsStore::update_global(cx, |store, cx| {
//                 store.update_user_settings::<language::language_settings::AllLanguageSettings>(
//                     cx,
//                     |settings| {
//                         settings.defaults.remove_trailing_whitespace_on_save = Some(true);
//                     },
//                 );
//             });
//         });

//         const CONTENT_WITH_TRAILING_WHITESPACE: &str =
//             "fn main() {  \n    println!(\"Hello!\");  \n}\n";

//         // Have the model stream content that contains trailing whitespace
//         let edit_result = {
//             let edit_task = cx.update(|cx| {
//                 let input = serde_json::to_value(EditFileToolInput {
//                     display_description: "Create main function".into(),
//                     path: "root/src/main.rs".into(),
//                     mode: EditFileMode::Overwrite,
//                 })
//                 .unwrap();
//                 Arc::new(EditFileTool)
//                     .run(
//                         input,
//                         Arc::default(),
//                         project.clone(),
//                         action_log.clone(),
//                         model.clone(),
//                         None,
//                         cx,
//                     )
//                     .output
//             });

//             // Stream the content with trailing whitespace
//             cx.executor().run_until_parked();
//             model.send_last_completion_stream_text_chunk(
//                 CONTENT_WITH_TRAILING_WHITESPACE.to_string(),
//             );
//             model.end_last_completion_stream();

//             edit_task.await
//         };
//         assert!(edit_result.is_ok());

//         // Wait for any async operations (e.g. formatting) to complete
//         cx.executor().run_until_parked();

//         // Read the file to verify trailing whitespace was removed automatically
//         assert_eq!(
//             // Ignore carriage returns on Windows
//             fs.load(path!("/root/src/main.rs").as_ref())
//                 .await
//                 .unwrap()
//                 .replace("\r\n", "\n"),
//             "fn main() {\n    println!(\"Hello!\");\n}\n",
//             "Trailing whitespace should be removed when remove_trailing_whitespace_on_save is enabled"
//         );

//         // Next, test with remove_trailing_whitespace_on_save disabled
//         cx.update(|cx| {
//             SettingsStore::update_global(cx, |store, cx| {
//                 store.update_user_settings::<language::language_settings::AllLanguageSettings>(
//                     cx,
//                     |settings| {
//                         settings.defaults.remove_trailing_whitespace_on_save = Some(false);
//                     },
//                 );
//             });
//         });

//         // Stream edits again with trailing whitespace
//         let edit_result = {
//             let edit_task = cx.update(|cx| {
//                 let input = serde_json::to_value(EditFileToolInput {
//                     display_description: "Update main function".into(),
//                     path: "root/src/main.rs".into(),
//                     mode: EditFileMode::Overwrite,
//                 })
//                 .unwrap();
//                 Arc::new(EditFileTool)
//                     .run(
//                         input,
//                         Arc::default(),
//                         project.clone(),
//                         action_log.clone(),
//                         model.clone(),
//                         None,
//                         cx,
//                     )
//                     .output
//             });

//             // Stream the content with trailing whitespace
//             cx.executor().run_until_parked();
//             model.send_last_completion_stream_text_chunk(
//                 CONTENT_WITH_TRAILING_WHITESPACE.to_string(),
//             );
//             model.end_last_completion_stream();

//             edit_task.await
//         };
//         assert!(edit_result.is_ok());

//         // Wait for any async operations (e.g. formatting) to complete
//         cx.executor().run_until_parked();

//         // Verify the file still has trailing whitespace
//         // Read the file again - it should still have trailing whitespace
//         let final_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
//         assert_eq!(
//             // Ignore carriage returns on Windows
//             final_content.replace("\r\n", "\n"),
//             CONTENT_WITH_TRAILING_WHITESPACE,
//             "Trailing whitespace should remain when remove_trailing_whitespace_on_save is disabled"
//         );
//     }

//     #[gpui::test]
//     async fn test_needs_confirmation(cx: &mut TestAppContext) {
//         init_test(cx);
//         let tool = Arc::new(EditFileTool);
//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree("/root", json!({})).await;

//         // Test 1: Path with .zed component should require confirmation
//         let input_with_zed = json!({
//             "display_description": "Edit settings",
//             "path": ".zed/settings.json",
//             "mode": "edit"
//         });
//         let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
//         cx.update(|cx| {
//             assert!(
//                 tool.needs_confirmation(&input_with_zed, &project, cx),
//                 "Path with .zed component should require confirmation"
//             );
//         });

//         // Test 2: Absolute path should require confirmation
//         let input_absolute = json!({
//             "display_description": "Edit file",
//             "path": "/etc/hosts",
//             "mode": "edit"
//         });
//         cx.update(|cx| {
//             assert!(
//                 tool.needs_confirmation(&input_absolute, &project, cx),
//                 "Absolute path should require confirmation"
//             );
//         });

//         // Test 3: Relative path without .zed should not require confirmation
//         let input_relative = json!({
//             "display_description": "Edit file",
//             "path": "root/src/main.rs",
//             "mode": "edit"
//         });
//         cx.update(|cx| {
//             assert!(
//                 !tool.needs_confirmation(&input_relative, &project, cx),
//                 "Relative path without .zed should not require confirmation"
//             );
//         });

//         // Test 4: Path with .zed in the middle should require confirmation
//         let input_zed_middle = json!({
//             "display_description": "Edit settings",
//             "path": "root/.zed/tasks.json",
//             "mode": "edit"
//         });
//         cx.update(|cx| {
//             assert!(
//                 tool.needs_confirmation(&input_zed_middle, &project, cx),
//                 "Path with .zed in any component should require confirmation"
//             );
//         });

//         // Test 5: When always_allow_tool_actions is enabled, no confirmation needed
//         cx.update(|cx| {
//             let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
//             settings.always_allow_tool_actions = true;
//             agent_settings::AgentSettings::override_global(settings, cx);

//             assert!(
//                 !tool.needs_confirmation(&input_with_zed, &project, cx),
//                 "When always_allow_tool_actions is true, no confirmation should be needed"
//             );
//             assert!(
//                 !tool.needs_confirmation(&input_absolute, &project, cx),
//                 "When always_allow_tool_actions is true, no confirmation should be needed for absolute paths"
//             );
//         });
//     }

//     #[gpui::test]
//     async fn test_ui_text_shows_correct_context(cx: &mut TestAppContext) {
//         // Set up a custom config directory for testing
//         let temp_dir = tempfile::tempdir().unwrap();
//         init_test_with_config(cx, temp_dir.path());

//         let tool = Arc::new(EditFileTool);

//         // Test ui_text shows context for various paths
//         let test_cases = vec![
//             (
//                 json!({
//                     "display_description": "Update config",
//                     "path": ".zed/settings.json",
//                     "mode": "edit"
//                 }),
//                 "Update config (local settings)",
//                 ".zed path should show local settings context",
//             ),
//             (
//                 json!({
//                     "display_description": "Fix bug",
//                     "path": "src/.zed/local.json",
//                     "mode": "edit"
//                 }),
//                 "Fix bug (local settings)",
//                 "Nested .zed path should show local settings context",
//             ),
//             (
//                 json!({
//                     "display_description": "Update readme",
//                     "path": "README.md",
//                     "mode": "edit"
//                 }),
//                 "Update readme",
//                 "Normal path should not show additional context",
//             ),
//             (
//                 json!({
//                     "display_description": "Edit config",
//                     "path": "config.zed",
//                     "mode": "edit"
//                 }),
//                 "Edit config",
//                 ".zed as extension should not show context",
//             ),
//         ];

//         for (input, expected_text, description) in test_cases {
//             cx.update(|_cx| {
//                 let ui_text = tool.ui_text(&input);
//                 assert_eq!(ui_text, expected_text, "Failed for case: {}", description);
//             });
//         }
//     }

//     #[gpui::test]
//     async fn test_needs_confirmation_outside_project(cx: &mut TestAppContext) {
//         init_test(cx);
//         let tool = Arc::new(EditFileTool);
//         let fs = project::FakeFs::new(cx.executor());

//         // Create a project in /project directory
//         fs.insert_tree("/project", json!({})).await;
//         let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

//         // Test file outside project requires confirmation
//         let input_outside = json!({
//             "display_description": "Edit file",
//             "path": "/outside/file.txt",
//             "mode": "edit"
//         });
//         cx.update(|cx| {
//             assert!(
//                 tool.needs_confirmation(&input_outside, &project, cx),
//                 "File outside project should require confirmation"
//             );
//         });

//         // Test file inside project doesn't require confirmation
//         let input_inside = json!({
//             "display_description": "Edit file",
//             "path": "project/file.txt",
//             "mode": "edit"
//         });
//         cx.update(|cx| {
//             assert!(
//                 !tool.needs_confirmation(&input_inside, &project, cx),
//                 "File inside project should not require confirmation"
//             );
//         });
//     }

//     #[gpui::test]
//     async fn test_needs_confirmation_config_paths(cx: &mut TestAppContext) {
//         // Set up a custom data directory for testing
//         let temp_dir = tempfile::tempdir().unwrap();
//         init_test_with_config(cx, temp_dir.path());

//         let tool = Arc::new(EditFileTool);
//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree("/home/user/myproject", json!({})).await;
//         let project = Project::test(fs.clone(), [path!("/home/user/myproject").as_ref()], cx).await;

//         // Get the actual local settings folder name
//         let local_settings_folder = paths::local_settings_folder_relative_path();

//         // Test various config path patterns
//         let test_cases = vec![
//             (
//                 format!("{}/settings.json", local_settings_folder.display()),
//                 true,
//                 "Top-level local settings file".to_string(),
//             ),
//             (
//                 format!(
//                     "myproject/{}/settings.json",
//                     local_settings_folder.display()
//                 ),
//                 true,
//                 "Local settings in project path".to_string(),
//             ),
//             (
//                 format!("src/{}/config.toml", local_settings_folder.display()),
//                 true,
//                 "Local settings in subdirectory".to_string(),
//             ),
//             (
//                 ".zed.backup/file.txt".to_string(),
//                 true,
//                 ".zed.backup is outside project".to_string(),
//             ),
//             (
//                 "my.zed/file.txt".to_string(),
//                 true,
//                 "my.zed is outside project".to_string(),
//             ),
//             (
//                 "myproject/src/file.zed".to_string(),
//                 false,
//                 ".zed as file extension".to_string(),
//             ),
//             (
//                 "myproject/normal/path/file.rs".to_string(),
//                 false,
//                 "Normal file without config paths".to_string(),
//             ),
//         ];

//         for (path, should_confirm, description) in test_cases {
//             let input = json!({
//                 "display_description": "Edit file",
//                 "path": path,
//                 "mode": "edit"
//             });
//             cx.update(|cx| {
//                 assert_eq!(
//                     tool.needs_confirmation(&input, &project, cx),
//                     should_confirm,
//                     "Failed for case: {} - path: {}",
//                     description,
//                     path
//                 );
//             });
//         }
//     }

//     #[gpui::test]
//     async fn test_needs_confirmation_global_config(cx: &mut TestAppContext) {
//         // Set up a custom data directory for testing
//         let temp_dir = tempfile::tempdir().unwrap();
//         init_test_with_config(cx, temp_dir.path());

//         let tool = Arc::new(EditFileTool);
//         let fs = project::FakeFs::new(cx.executor());

//         // Create test files in the global config directory
//         let global_config_dir = paths::config_dir();
//         fs::create_dir_all(&global_config_dir).unwrap();
//         let global_settings_path = global_config_dir.join("settings.json");
//         fs::write(&global_settings_path, "{}").unwrap();

//         fs.insert_tree("/project", json!({})).await;
//         let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

//         // Test global config paths
//         let test_cases = vec![
//             (
//                 global_settings_path.to_str().unwrap().to_string(),
//                 true,
//                 "Global settings file should require confirmation",
//             ),
//             (
//                 global_config_dir
//                     .join("keymap.json")
//                     .to_str()
//                     .unwrap()
//                     .to_string(),
//                 true,
//                 "Global keymap file should require confirmation",
//             ),
//             (
//                 "project/normal_file.rs".to_string(),
//                 false,
//                 "Normal project file should not require confirmation",
//             ),
//         ];

//         for (path, should_confirm, description) in test_cases {
//             let input = json!({
//                 "display_description": "Edit file",
//                 "path": path,
//                 "mode": "edit"
//             });
//             cx.update(|cx| {
//                 assert_eq!(
//                     tool.needs_confirmation(&input, &project, cx),
//                     should_confirm,
//                     "Failed for case: {}",
//                     description
//                 );
//             });
//         }
//     }

//     #[gpui::test]
//     async fn test_needs_confirmation_with_multiple_worktrees(cx: &mut TestAppContext) {
//         init_test(cx);
//         let tool = Arc::new(EditFileTool);
//         let fs = project::FakeFs::new(cx.executor());

//         // Create multiple worktree directories
//         fs.insert_tree(
//             "/workspace/frontend",
//             json!({
//                 "src": {
//                     "main.js": "console.log('frontend');"
//                 }
//             }),
//         )
//         .await;
//         fs.insert_tree(
//             "/workspace/backend",
//             json!({
//                 "src": {
//                     "main.rs": "fn main() {}"
//                 }
//             }),
//         )
//         .await;
//         fs.insert_tree(
//             "/workspace/shared",
//             json!({
//                 ".zed": {
//                     "settings.json": "{}"
//                 }
//             }),
//         )
//         .await;

//         // Create project with multiple worktrees
//         let project = Project::test(
//             fs.clone(),
//             [
//                 path!("/workspace/frontend").as_ref(),
//                 path!("/workspace/backend").as_ref(),
//                 path!("/workspace/shared").as_ref(),
//             ],
//             cx,
//         )
//         .await;

//         // Test files in different worktrees
//         let test_cases = vec![
//             ("frontend/src/main.js", false, "File in first worktree"),
//             ("backend/src/main.rs", false, "File in second worktree"),
//             (
//                 "shared/.zed/settings.json",
//                 true,
//                 ".zed file in third worktree",
//             ),
//             ("/etc/hosts", true, "Absolute path outside all worktrees"),
//             (
//                 "../outside/file.txt",
//                 true,
//                 "Relative path outside worktrees",
//             ),
//         ];

//         for (path, should_confirm, description) in test_cases {
//             let input = json!({
//                 "display_description": "Edit file",
//                 "path": path,
//                 "mode": "edit"
//             });
//             cx.update(|cx| {
//                 assert_eq!(
//                     tool.needs_confirmation(&input, &project, cx),
//                     should_confirm,
//                     "Failed for case: {} - path: {}",
//                     description,
//                     path
//                 );
//             });
//         }
//     }

//     #[gpui::test]
//     async fn test_needs_confirmation_edge_cases(cx: &mut TestAppContext) {
//         init_test(cx);
//         let tool = Arc::new(EditFileTool);
//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree(
//             "/project",
//             json!({
//                 ".zed": {
//                     "settings.json": "{}"
//                 },
//                 "src": {
//                     ".zed": {
//                         "local.json": "{}"
//                     }
//                 }
//             }),
//         )
//         .await;
//         let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

//         // Test edge cases
//         let test_cases = vec![
//             // Empty path - find_project_path returns Some for empty paths
//             ("", false, "Empty path is treated as project root"),
//             // Root directory
//             ("/", true, "Root directory should be outside project"),
//             // Parent directory references - find_project_path resolves these
//             (
//                 "project/../other",
//                 false,
//                 "Path with .. is resolved by find_project_path",
//             ),
//             (
//                 "project/./src/file.rs",
//                 false,
//                 "Path with . should work normally",
//             ),
//             // Windows-style paths (if on Windows)
//             #[cfg(target_os = "windows")]
//             ("C:\\Windows\\System32\\hosts", true, "Windows system path"),
//             #[cfg(target_os = "windows")]
//             ("project\\src\\main.rs", false, "Windows-style project path"),
//         ];

//         for (path, should_confirm, description) in test_cases {
//             let input = json!({
//                 "display_description": "Edit file",
//                 "path": path,
//                 "mode": "edit"
//             });
//             cx.update(|cx| {
//                 assert_eq!(
//                     tool.needs_confirmation(&input, &project, cx),
//                     should_confirm,
//                     "Failed for case: {} - path: {}",
//                     description,
//                     path
//                 );
//             });
//         }
//     }

//     #[gpui::test]
//     async fn test_ui_text_with_all_path_types(cx: &mut TestAppContext) {
//         init_test(cx);
//         let tool = Arc::new(EditFileTool);

//         // Test UI text for various scenarios
//         let test_cases = vec![
//             (
//                 json!({
//                     "display_description": "Update config",
//                     "path": ".zed/settings.json",
//                     "mode": "edit"
//                 }),
//                 "Update config (local settings)",
//                 ".zed path should show local settings context",
//             ),
//             (
//                 json!({
//                     "display_description": "Fix bug",
//                     "path": "src/.zed/local.json",
//                     "mode": "edit"
//                 }),
//                 "Fix bug (local settings)",
//                 "Nested .zed path should show local settings context",
//             ),
//             (
//                 json!({
//                     "display_description": "Update readme",
//                     "path": "README.md",
//                     "mode": "edit"
//                 }),
//                 "Update readme",
//                 "Normal path should not show additional context",
//             ),
//             (
//                 json!({
//                     "display_description": "Edit config",
//                     "path": "config.zed",
//                     "mode": "edit"
//                 }),
//                 "Edit config",
//                 ".zed as extension should not show context",
//             ),
//         ];

//         for (input, expected_text, description) in test_cases {
//             cx.update(|_cx| {
//                 let ui_text = tool.ui_text(&input);
//                 assert_eq!(ui_text, expected_text, "Failed for case: {}", description);
//             });
//         }
//     }

//     #[gpui::test]
//     async fn test_needs_confirmation_with_different_modes(cx: &mut TestAppContext) {
//         init_test(cx);
//         let tool = Arc::new(EditFileTool);
//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree(
//             "/project",
//             json!({
//                 "existing.txt": "content",
//                 ".zed": {
//                     "settings.json": "{}"
//                 }
//             }),
//         )
//         .await;
//         let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

//         // Test different EditFileMode values
//         let modes = vec![
//             EditFileMode::Edit,
//             EditFileMode::Create,
//             EditFileMode::Overwrite,
//         ];

//         for mode in modes {
//             // Test .zed path with different modes
//             let input_zed = json!({
//                 "display_description": "Edit settings",
//                 "path": "project/.zed/settings.json",
//                 "mode": mode
//             });
//             cx.update(|cx| {
//                 assert!(
//                     tool.needs_confirmation(&input_zed, &project, cx),
//                     ".zed path should require confirmation regardless of mode: {:?}",
//                     mode
//                 );
//             });

//             // Test outside path with different modes
//             let input_outside = json!({
//                 "display_description": "Edit file",
//                 "path": "/outside/file.txt",
//                 "mode": mode
//             });
//             cx.update(|cx| {
//                 assert!(
//                     tool.needs_confirmation(&input_outside, &project, cx),
//                     "Outside path should require confirmation regardless of mode: {:?}",
//                     mode
//                 );
//             });

//             // Test normal path with different modes
//             let input_normal = json!({
//                 "display_description": "Edit file",
//                 "path": "project/normal.txt",
//                 "mode": mode
//             });
//             cx.update(|cx| {
//                 assert!(
//                     !tool.needs_confirmation(&input_normal, &project, cx),
//                     "Normal path should not require confirmation regardless of mode: {:?}",
//                     mode
//                 );
//             });
//         }
//     }

//     #[gpui::test]
//     async fn test_always_allow_tool_actions_bypasses_all_checks(cx: &mut TestAppContext) {
//         // Set up with custom directories for deterministic testing
//         let temp_dir = tempfile::tempdir().unwrap();
//         init_test_with_config(cx, temp_dir.path());

//         let tool = Arc::new(EditFileTool);
//         let fs = project::FakeFs::new(cx.executor());
//         fs.insert_tree("/project", json!({})).await;
//         let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

//         // Enable always_allow_tool_actions
//         cx.update(|cx| {
//             let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
//             settings.always_allow_tool_actions = true;
//             agent_settings::AgentSettings::override_global(settings, cx);
//         });

//         // Test that all paths that normally require confirmation are bypassed
//         let global_settings_path = paths::config_dir().join("settings.json");
//         fs::create_dir_all(paths::config_dir()).unwrap();
//         fs::write(&global_settings_path, "{}").unwrap();

//         let test_cases = vec![
//             ".zed/settings.json",
//             "project/.zed/config.toml",
//             global_settings_path.to_str().unwrap(),
//             "/etc/hosts",
//             "/absolute/path/file.txt",
//             "../outside/project.txt",
//         ];

//         for path in test_cases {
//             let input = json!({
//                 "display_description": "Edit file",
//                 "path": path,
//                 "mode": "edit"
//             });
//             cx.update(|cx| {
//                 assert!(
//                     !tool.needs_confirmation(&input, &project, cx),
//                     "Path {} should not require confirmation when always_allow_tool_actions is true",
//                     path
//                 );
//             });
//         }

//         // Disable always_allow_tool_actions and verify confirmation is required again
//         cx.update(|cx| {
//             let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
//             settings.always_allow_tool_actions = false;
//             agent_settings::AgentSettings::override_global(settings, cx);
//         });

//         // Verify .zed path requires confirmation again
//         let input = json!({
//             "display_description": "Edit file",
//             "path": ".zed/settings.json",
//             "mode": "edit"
//         });
//         cx.update(|cx| {
//             assert!(
//                 tool.needs_confirmation(&input, &project, cx),
//                 ".zed path should require confirmation when always_allow_tool_actions is false"
//             );
//         });
//     }
// }

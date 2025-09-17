use crate::{AgentTool, Thread, ToolCallEventStream};
use acp_thread::Diff;
use agent_client_protocol::{self as acp, ToolCallLocation, ToolCallUpdateFields};
use anyhow::{Context as _, Result, anyhow};
use assistant_tools::edit_agent::{EditAgent, EditAgentOutput, EditAgentOutputEvent, EditFormat};
use cloud_llm_client::CompletionIntent;
use collections::HashSet;
use gpui::{App, AppContext, AsyncApp, Entity, Task, WeakEntity};
use indoc::formatdoc;
use language::language_settings::{self, FormatOnSave};
use language::{LanguageRegistry, ToPoint};
use language_model::LanguageModelToolResultContent;
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

const DEFAULT_UI_TEXT: &str = "Editing file";

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
    /// A one-line, user-friendly markdown description of the edit. This will be shown in the UI and also passed to another model to perform the edit.
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
    /// - 'edit': Make granular edits to an existing file.
    /// - 'create': Create a new file if it doesn't exist.
    /// - 'overwrite': Replace the entire contents of an existing file.
    ///
    /// When a file already exists or you just created it, prefer editing it as opposed to recreating it from scratch.
    pub mode: EditFileMode,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EditFileToolPartialInput {
    #[serde(default)]
    path: String,
    #[serde(default)]
    display_description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[schemars(inline)]
pub enum EditFileMode {
    Edit,
    Create,
    Overwrite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditFileToolOutput {
    #[serde(alias = "original_path")]
    input_path: PathBuf,
    new_text: String,
    old_text: Arc<String>,
    #[serde(default)]
    diff: String,
    #[serde(alias = "raw_output")]
    edit_agent_output: EditAgentOutput,
}

impl From<EditFileToolOutput> for LanguageModelToolResultContent {
    fn from(output: EditFileToolOutput) -> Self {
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

pub struct EditFileTool {
    thread: WeakEntity<Thread>,
    language_registry: Arc<LanguageRegistry>,
    project: Entity<Project>,
}

impl EditFileTool {
    pub fn new(
        project: Entity<Project>,
        thread: WeakEntity<Thread>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            project,
            thread,
            language_registry,
        }
    }

    fn authorize(
        &self,
        input: &EditFileToolInput,
        event_stream: &ToolCallEventStream,
        cx: &mut App,
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
            return event_stream.authorize(
                format!("{} (local settings)", input.display_description),
                cx,
            );
        }

        // It's also possible that the global config dir is configured to be inside the project,
        // so check for that edge case too.
        if let Ok(canonical_path) = std::fs::canonicalize(&input.path)
            && canonical_path.starts_with(paths::config_dir())
        {
            return event_stream.authorize(
                format!("{} (global settings)", input.display_description),
                cx,
            );
        }

        // Check if path is inside the global config directory
        // First check if it's already inside project - if not, try to canonicalize
        let Ok(project_path) = self.thread.read_with(cx, |thread, cx| {
            thread.project().read(cx).find_project_path(&input.path, cx)
        }) else {
            return Task::ready(Err(anyhow!("thread was dropped")));
        };

        // If the path is inside the project, and it's not one of the above edge cases,
        // then no confirmation is necessary. Otherwise, confirmation is necessary.
        if project_path.is_some() {
            Task::ready(Ok(()))
        } else {
            event_stream.authorize(&input.display_description, cx)
        }
    }
}

impl AgentTool for EditFileTool {
    type Input = EditFileToolInput;
    type Output = EditFileToolOutput;

    fn name() -> &'static str {
        "edit_file"
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
                .unwrap_or(Path::new(&input.path).into())
                .to_string_lossy()
                .to_string()
                .into(),
            Err(raw_input) => {
                if let Some(input) =
                    serde_json::from_value::<EditFileToolPartialInput>(raw_input).ok()
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
                            .unwrap_or(Path::new(&input.path).into())
                            .to_string_lossy()
                            .to_string()
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
            event_stream.update_fields(ToolCallUpdateFields {
                locations: Some(vec![acp::ToolCallLocation {
                    path: abs_path,
                    line: None,
                }]),
                ..Default::default()
            });
        }

        let authorize = self.authorize(&input, &event_stream, cx);
        cx.spawn(async move |cx: &mut AsyncApp| {
            authorize.await?;

            let (request, model, action_log) = self.thread.update(cx, |thread, cx| {
                let request = thread.build_completion_request(CompletionIntent::ToolResults, cx);
                (request, thread.model().cloned(), thread.action_log().clone())
            })?;
            let request = request?;
            let model = model.context("No language model configured")?;

            let edit_format = EditFormat::from_model(model.clone())?;
            let edit_agent = EditAgent::new(
                model,
                project.clone(),
                action_log.clone(),
                // TODO: move edit agent to this crate so we can use our templates
                assistant_tools::templates::Templates::new(),
                edit_format,
            );

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await?;

            let diff = cx.new(|cx| Diff::new(buffer.clone(), cx))?;
            event_stream.update_diff(diff.clone());
            let _finalize_diff = util::defer({
               let diff = diff.downgrade();
               let mut cx = cx.clone();
               move || {
                   diff.update(&mut cx, |diff, cx| diff.finalize(cx)).ok();
               }
            });

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
            let mut emitted_location = false;
            while let Some(event) = events.next().await {
                match event {
                    EditAgentOutputEvent::Edited(range) => {
                        if !emitted_location {
                            let line = buffer.update(cx, |buffer, _cx| {
                                range.start.to_point(&buffer.snapshot()).row
                            }).ok();
                            if let Some(abs_path) = abs_path.clone() {
                                event_stream.update_fields(ToolCallUpdateFields {
                                    locations: Some(vec![ToolCallLocation { path: abs_path, line }]),
                                    ..Default::default()
                                });
                            }
                            emitted_location = true;
                        }
                    },
                    EditAgentOutputEvent::UnresolvedEditRange => hallucinated_old_text = true,
                    EditAgentOutputEvent::AmbiguousEditRange(ranges) => ambiguous_ranges = ranges,
                    EditAgentOutputEvent::ResolvingEditRange(range) => {
                        diff.update(cx, |card, cx| card.reveal_range(range.clone(), cx))?;
                        // if !emitted_location {
                        //     let line = buffer.update(cx, |buffer, _cx| {
                        //         range.start.to_point(&buffer.snapshot()).row
                        //     }).ok();
                        //     if let Some(abs_path) = abs_path.clone() {
                        //         event_stream.update_fields(ToolCallUpdateFields {
                        //             locations: Some(vec![ToolCallLocation { path: abs_path, line }]),
                        //             ..Default::default()
                        //         });
                        //     }
                        // }
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

            let edit_agent_output = output.await?;

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
            }

            Ok(EditFileToolOutput {
                input_path: input.path,
                new_text,
                old_text,
                diff: unified_diff,
                edit_agent_output,
            })
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
                output.input_path,
                Some(output.old_text.to_string()),
                output.new_text,
                self.language_registry.clone(),
                cx,
            )
        }));
        Ok(())
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
                .and_then(|path| project.entry_for_path(path, cx))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContextServerRegistry, Templates};
    use client::TelemetrySettings;
    use fs::Fs;
    use gpui::{TestAppContext, UpdateGlobal};
    use language_model::fake_provider::FakeLanguageModel;
    use prompt_store::ProjectContext;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_edit_nonexistent_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
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
                let input = EditFileToolInput {
                    display_description: "Some edit".into(),
                    path: "root/nonexistent_file.txt".into(),
                    mode: EditFileMode::Edit,
                };
                Arc::new(EditFileTool::new(
                    project,
                    thread.downgrade(),
                    language_registry,
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
    async fn test_resolve_path_for_creating_file(cx: &mut TestAppContext) {
        let mode = &EditFileMode::Create;

        let result = test_resolve_path(mode, "root/new.txt", cx);
        assert_resolved_path_eq(result.await, "new.txt");

        let result = test_resolve_path(mode, "new.txt", cx);
        assert_resolved_path_eq(result.await, "new.txt");

        let result = test_resolve_path(mode, "dir/new.txt", cx);
        assert_resolved_path_eq(result.await, "dir/new.txt");

        let result = test_resolve_path(mode, "root/dir/subdir/existing.txt", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't create file: file already exists"
        );

        let result = test_resolve_path(mode, "root/dir/nonexistent_dir/new.txt", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't create file: parent directory doesn't exist"
        );
    }

    #[gpui::test]
    async fn test_resolve_path_for_editing_file(cx: &mut TestAppContext) {
        let mode = &EditFileMode::Edit;

        let path_with_root = "root/dir/subdir/existing.txt";
        let path_without_root = "dir/subdir/existing.txt";
        let result = test_resolve_path(mode, path_with_root, cx);
        assert_resolved_path_eq(result.await, path_without_root);

        let result = test_resolve_path(mode, path_without_root, cx);
        assert_resolved_path_eq(result.await, path_without_root);

        let result = test_resolve_path(mode, "root/nonexistent.txt", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't edit file: path not found"
        );

        let result = test_resolve_path(mode, "root/dir", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't edit file: path is a directory"
        );
    }

    async fn test_resolve_path(
        mode: &EditFileMode,
        path: &str,
        cx: &mut TestAppContext,
    ) -> anyhow::Result<ProjectPath> {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "dir": {
                    "subdir": {
                        "existing.txt": "hello"
                    }
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        let input = EditFileToolInput {
            display_description: "Some edit".into(),
            path: path.into(),
            mode: mode.clone(),
        };

        cx.update(|cx| resolve_path(&input, project, cx))
    }

    fn assert_resolved_path_eq(path: anyhow::Result<ProjectPath>, expected: &str) {
        let actual = path
            .expect("Should return valid path")
            .path
            .to_str()
            .unwrap()
            .replace("\\", "/"); // Naive Windows paths normalization
        assert_eq!(actual, expected);
    }

    #[gpui::test]
    async fn test_format_on_save(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"src": {}})).await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        // Set up a Rust language with LSP formatting support
        let rust_language = Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "Rust".into(),
                matcher: language::LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        // Register the language and fake LSP
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_language);

        let mut fake_language_servers = language_registry.register_fake_lsp(
            "Rust",
            language::FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    document_formatting_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        // Create the file
        fs.save(
            path!("/root/src/main.rs").as_ref(),
            &"initial content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();

        // Open the buffer to trigger LSP initialization
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();

        // Register the buffer with language servers
        let _handle = project.update(cx, |project, cx| {
            project.register_buffer_with_language_servers(&buffer, cx)
        });

        const UNFORMATTED_CONTENT: &str = "fn main() {println!(\"Hello!\");}\n";
        const FORMATTED_CONTENT: &str =
            "This file was formatted by the fake formatter in the test.\n";

        // Get the fake language server and set up formatting handler
        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.set_request_handler::<lsp::request::Formatting, _, _>({
            |_, _| async move {
                Ok(Some(vec![lsp::TextEdit {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(1, 0)),
                    new_text: FORMATTED_CONTENT.to_string(),
                }]))
            }
        });

        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });

        // First, test with format_on_save enabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<language::language_settings::AllLanguageSettings>(
                    cx,
                    |settings| {
                        settings.defaults.format_on_save = Some(FormatOnSave::On);
                        settings.defaults.formatter =
                            Some(language::language_settings::SelectedFormatter::Auto);
                    },
                );
            });
        });

        // Have the model stream unformatted content
        let edit_result = {
            let edit_task = cx.update(|cx| {
                let input = EditFileToolInput {
                    display_description: "Create main function".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Overwrite,
                };
                Arc::new(EditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry.clone(),
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            });

            // Stream the unformatted content
            cx.executor().run_until_parked();
            model.send_last_completion_stream_text_chunk(UNFORMATTED_CONTENT.to_string());
            model.end_last_completion_stream();

            edit_task.await
        };
        assert!(edit_result.is_ok());

        // Wait for any async operations (e.g. formatting) to complete
        cx.executor().run_until_parked();

        // Read the file to verify it was formatted automatically
        let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            // Ignore carriage returns on Windows
            new_content.replace("\r\n", "\n"),
            FORMATTED_CONTENT,
            "Code should be formatted when format_on_save is enabled"
        );

        let stale_buffer_count = thread
            .read_with(cx, |thread, _cx| thread.action_log.clone())
            .read_with(cx, |log, cx| log.stale_buffers(cx).count());

        assert_eq!(
            stale_buffer_count, 0,
            "BUG: Buffer is incorrectly marked as stale after format-on-save. Found {} stale buffers. \
             This causes the agent to think the file was modified externally when it was just formatted.",
            stale_buffer_count
        );

        // Next, test with format_on_save disabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<language::language_settings::AllLanguageSettings>(
                    cx,
                    |settings| {
                        settings.defaults.format_on_save = Some(FormatOnSave::Off);
                    },
                );
            });
        });

        // Stream unformatted edits again
        let edit_result = {
            let edit_task = cx.update(|cx| {
                let input = EditFileToolInput {
                    display_description: "Update main function".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Overwrite,
                };
                Arc::new(EditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            });

            // Stream the unformatted content
            cx.executor().run_until_parked();
            model.send_last_completion_stream_text_chunk(UNFORMATTED_CONTENT.to_string());
            model.end_last_completion_stream();

            edit_task.await
        };
        assert!(edit_result.is_ok());

        // Wait for any async operations (e.g. formatting) to complete
        cx.executor().run_until_parked();

        // Verify the file was not formatted
        let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            // Ignore carriage returns on Windows
            new_content.replace("\r\n", "\n"),
            UNFORMATTED_CONTENT,
            "Code should not be formatted when format_on_save is disabled"
        );
    }

    #[gpui::test]
    async fn test_remove_trailing_whitespace(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"src": {}})).await;

        // Create a simple file with trailing whitespace
        fs.save(
            path!("/root/src/main.rs").as_ref(),
            &"initial content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });

        // First, test with remove_trailing_whitespace_on_save enabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<language::language_settings::AllLanguageSettings>(
                    cx,
                    |settings| {
                        settings.defaults.remove_trailing_whitespace_on_save = Some(true);
                    },
                );
            });
        });

        const CONTENT_WITH_TRAILING_WHITESPACE: &str =
            "fn main() {  \n    println!(\"Hello!\");  \n}\n";

        // Have the model stream content that contains trailing whitespace
        let edit_result = {
            let edit_task = cx.update(|cx| {
                let input = EditFileToolInput {
                    display_description: "Create main function".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Overwrite,
                };
                Arc::new(EditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry.clone(),
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            });

            // Stream the content with trailing whitespace
            cx.executor().run_until_parked();
            model.send_last_completion_stream_text_chunk(
                CONTENT_WITH_TRAILING_WHITESPACE.to_string(),
            );
            model.end_last_completion_stream();

            edit_task.await
        };
        assert!(edit_result.is_ok());

        // Wait for any async operations (e.g. formatting) to complete
        cx.executor().run_until_parked();

        // Read the file to verify trailing whitespace was removed automatically
        assert_eq!(
            // Ignore carriage returns on Windows
            fs.load(path!("/root/src/main.rs").as_ref())
                .await
                .unwrap()
                .replace("\r\n", "\n"),
            "fn main() {\n    println!(\"Hello!\");\n}\n",
            "Trailing whitespace should be removed when remove_trailing_whitespace_on_save is enabled"
        );

        // Next, test with remove_trailing_whitespace_on_save disabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<language::language_settings::AllLanguageSettings>(
                    cx,
                    |settings| {
                        settings.defaults.remove_trailing_whitespace_on_save = Some(false);
                    },
                );
            });
        });

        // Stream edits again with trailing whitespace
        let edit_result = {
            let edit_task = cx.update(|cx| {
                let input = EditFileToolInput {
                    display_description: "Update main function".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Overwrite,
                };
                Arc::new(EditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                ))
                .run(input, ToolCallEventStream::test().0, cx)
            });

            // Stream the content with trailing whitespace
            cx.executor().run_until_parked();
            model.send_last_completion_stream_text_chunk(
                CONTENT_WITH_TRAILING_WHITESPACE.to_string(),
            );
            model.end_last_completion_stream();

            edit_task.await
        };
        assert!(edit_result.is_ok());

        // Wait for any async operations (e.g. formatting) to complete
        cx.executor().run_until_parked();

        // Verify the file still has trailing whitespace
        // Read the file again - it should still have trailing whitespace
        let final_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            // Ignore carriage returns on Windows
            final_content.replace("\r\n", "\n"),
            CONTENT_WITH_TRAILING_WHITESPACE,
            "Trailing whitespace should remain when remove_trailing_whitespace_on_save is disabled"
        );
    }

    #[gpui::test]
    async fn test_authorize(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });
        let tool = Arc::new(EditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
        ));
        fs.insert_tree("/root", json!({})).await;

        // Test 1: Path with .zed component should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx.update(|cx| {
            tool.authorize(
                &EditFileToolInput {
                    display_description: "test 1".into(),
                    path: ".zed/settings.json".into(),
                    mode: EditFileMode::Edit,
                },
                &stream_tx,
                cx,
            )
        });

        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("test 1 (local settings)".into())
        );

        // Test 2: Path outside project should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx.update(|cx| {
            tool.authorize(
                &EditFileToolInput {
                    display_description: "test 2".into(),
                    path: "/etc/hosts".into(),
                    mode: EditFileMode::Edit,
                },
                &stream_tx,
                cx,
            )
        });

        let event = stream_rx.expect_authorization().await;
        assert_eq!(event.tool_call.fields.title, Some("test 2".into()));

        // Test 3: Relative path without .zed should not require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| {
            tool.authorize(
                &EditFileToolInput {
                    display_description: "test 3".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Edit,
                },
                &stream_tx,
                cx,
            )
        })
        .await
        .unwrap();
        assert!(stream_rx.try_next().is_err());

        // Test 4: Path with .zed in the middle should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx.update(|cx| {
            tool.authorize(
                &EditFileToolInput {
                    display_description: "test 4".into(),
                    path: "root/.zed/tasks.json".into(),
                    mode: EditFileMode::Edit,
                },
                &stream_tx,
                cx,
            )
        });
        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("test 4 (local settings)".into())
        );

        // Test 5: When always_allow_tool_actions is enabled, no confirmation needed
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.always_allow_tool_actions = true;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| {
            tool.authorize(
                &EditFileToolInput {
                    display_description: "test 5.1".into(),
                    path: ".zed/settings.json".into(),
                    mode: EditFileMode::Edit,
                },
                &stream_tx,
                cx,
            )
        })
        .await
        .unwrap();
        assert!(stream_rx.try_next().is_err());

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| {
            tool.authorize(
                &EditFileToolInput {
                    display_description: "test 5.2".into(),
                    path: "/etc/hosts".into(),
                    mode: EditFileMode::Edit,
                },
                &stream_tx,
                cx,
            )
        })
        .await
        .unwrap();
        assert!(stream_rx.try_next().is_err());
    }

    #[gpui::test]
    async fn test_authorize_global_config(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });
        let tool = Arc::new(EditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
        ));

        // Test global config paths - these should require confirmation if they exist and are outside the project
        let test_cases = vec![
            (
                "/etc/hosts",
                true,
                "System file should require confirmation",
            ),
            (
                "/usr/local/bin/script",
                true,
                "System bin file should require confirmation",
            ),
            (
                "project/normal_file.rs",
                false,
                "Normal project file should not require confirmation",
            ),
        ];

        for (path, should_confirm, description) in test_cases {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let auth = cx.update(|cx| {
                tool.authorize(
                    &EditFileToolInput {
                        display_description: "Edit file".into(),
                        path: path.into(),
                        mode: EditFileMode::Edit,
                    },
                    &stream_tx,
                    cx,
                )
            });

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                auth.await.unwrap();
                assert!(
                    stream_rx.try_next().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
            }
        }
    }

    #[gpui::test]
    async fn test_needs_confirmation_with_multiple_worktrees(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());

        // Create multiple worktree directories
        fs.insert_tree(
            "/workspace/frontend",
            json!({
                "src": {
                    "main.js": "console.log('frontend');"
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/workspace/backend",
            json!({
                "src": {
                    "main.rs": "fn main() {}"
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/workspace/shared",
            json!({
                ".zed": {
                    "settings.json": "{}"
                }
            }),
        )
        .await;

        // Create project with multiple worktrees
        let project = Project::test(
            fs.clone(),
            [
                path!("/workspace/frontend").as_ref(),
                path!("/workspace/backend").as_ref(),
                path!("/workspace/shared").as_ref(),
            ],
            cx,
        )
        .await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry.clone(),
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });
        let tool = Arc::new(EditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
        ));

        // Test files in different worktrees
        let test_cases = vec![
            ("frontend/src/main.js", false, "File in first worktree"),
            ("backend/src/main.rs", false, "File in second worktree"),
            (
                "shared/.zed/settings.json",
                true,
                ".zed file in third worktree",
            ),
            ("/etc/hosts", true, "Absolute path outside all worktrees"),
            (
                "../outside/file.txt",
                true,
                "Relative path outside worktrees",
            ),
        ];

        for (path, should_confirm, description) in test_cases {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let auth = cx.update(|cx| {
                tool.authorize(
                    &EditFileToolInput {
                        display_description: "Edit file".into(),
                        path: path.into(),
                        mode: EditFileMode::Edit,
                    },
                    &stream_tx,
                    cx,
                )
            });

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                auth.await.unwrap();
                assert!(
                    stream_rx.try_next().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
            }
        }
    }

    #[gpui::test]
    async fn test_needs_confirmation_edge_cases(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".zed": {
                    "settings.json": "{}"
                },
                "src": {
                    ".zed": {
                        "local.json": "{}"
                    }
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry.clone(),
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });
        let tool = Arc::new(EditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
        ));

        // Test edge cases
        let test_cases = vec![
            // Empty path - find_project_path returns Some for empty paths
            ("", false, "Empty path is treated as project root"),
            // Root directory
            ("/", true, "Root directory should be outside project"),
            // Parent directory references - find_project_path resolves these
            (
                "project/../other",
                false,
                "Path with .. is resolved by find_project_path",
            ),
            (
                "project/./src/file.rs",
                false,
                "Path with . should work normally",
            ),
            // Windows-style paths (if on Windows)
            #[cfg(target_os = "windows")]
            ("C:\\Windows\\System32\\hosts", true, "Windows system path"),
            #[cfg(target_os = "windows")]
            ("project\\src\\main.rs", false, "Windows-style project path"),
        ];

        for (path, should_confirm, description) in test_cases {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let auth = cx.update(|cx| {
                tool.authorize(
                    &EditFileToolInput {
                        display_description: "Edit file".into(),
                        path: path.into(),
                        mode: EditFileMode::Edit,
                    },
                    &stream_tx,
                    cx,
                )
            });

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                auth.await.unwrap();
                assert!(
                    stream_rx.try_next().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
            }
        }
    }

    #[gpui::test]
    async fn test_needs_confirmation_with_different_modes(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "existing.txt": "content",
                ".zed": {
                    "settings.json": "{}"
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry.clone(),
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });
        let tool = Arc::new(EditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
        ));

        // Test different EditFileMode values
        let modes = vec![
            EditFileMode::Edit,
            EditFileMode::Create,
            EditFileMode::Overwrite,
        ];

        for mode in modes {
            // Test .zed path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let _auth = cx.update(|cx| {
                tool.authorize(
                    &EditFileToolInput {
                        display_description: "Edit settings".into(),
                        path: "project/.zed/settings.json".into(),
                        mode: mode.clone(),
                    },
                    &stream_tx,
                    cx,
                )
            });

            stream_rx.expect_authorization().await;

            // Test outside path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let _auth = cx.update(|cx| {
                tool.authorize(
                    &EditFileToolInput {
                        display_description: "Edit file".into(),
                        path: "/outside/file.txt".into(),
                        mode: mode.clone(),
                    },
                    &stream_tx,
                    cx,
                )
            });

            stream_rx.expect_authorization().await;

            // Test normal path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            cx.update(|cx| {
                tool.authorize(
                    &EditFileToolInput {
                        display_description: "Edit file".into(),
                        path: "project/normal.txt".into(),
                        mode: mode.clone(),
                    },
                    &stream_tx,
                    cx,
                )
            })
            .await
            .unwrap();
            assert!(stream_rx.try_next().is_err());
        }
    }

    #[gpui::test]
    async fn test_initial_title_with_partial_input(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });
        let tool = Arc::new(EditFileTool::new(
            project,
            thread.downgrade(),
            language_registry,
        ));

        cx.update(|cx| {
            // ...
            assert_eq!(
                tool.initial_title(
                    Err(json!({
                        "path": "src/main.rs",
                        "display_description": "",
                        "old_string": "old code",
                        "new_string": "new code"
                    })),
                    cx
                ),
                "src/main.rs"
            );
            assert_eq!(
                tool.initial_title(
                    Err(json!({
                        "path": "",
                        "display_description": "Fix error handling",
                        "old_string": "old code",
                        "new_string": "new code"
                    })),
                    cx
                ),
                "Fix error handling"
            );
            assert_eq!(
                tool.initial_title(
                    Err(json!({
                        "path": "src/main.rs",
                        "display_description": "Fix error handling",
                        "old_string": "old code",
                        "new_string": "new code"
                    })),
                    cx
                ),
                "src/main.rs"
            );
            assert_eq!(
                tool.initial_title(
                    Err(json!({
                        "path": "",
                        "display_description": "",
                        "old_string": "old code",
                        "new_string": "new code"
                    })),
                    cx
                ),
                DEFAULT_UI_TEXT
            );
            assert_eq!(
                tool.initial_title(Err(serde_json::Value::Null), cx),
                DEFAULT_UI_TEXT
            );
        });
    }

    #[gpui::test]
    async fn test_diff_finalization(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/", json!({"main.rs": ""})).await;

        let project = Project::test(fs.clone(), [path!("/").as_ref()], cx).await;
        let languages = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry.clone(),
                Templates::new(),
                Some(model.clone()),
                cx,
            )
        });

        // Ensure the diff is finalized after the edit completes.
        {
            let tool = Arc::new(EditFileTool::new(
                project.clone(),
                thread.downgrade(),
                languages.clone(),
            ));
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let edit = cx.update(|cx| {
                tool.run(
                    EditFileToolInput {
                        display_description: "Edit file".into(),
                        path: path!("/main.rs").into(),
                        mode: EditFileMode::Edit,
                    },
                    stream_tx,
                    cx,
                )
            });
            stream_rx.expect_update_fields().await;
            let diff = stream_rx.expect_diff().await;
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Pending(_))));
            cx.run_until_parked();
            model.end_last_completion_stream();
            edit.await.unwrap();
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Finalized(_))));
        }

        // Ensure the diff is finalized if an error occurs while editing.
        {
            model.forbid_requests();
            let tool = Arc::new(EditFileTool::new(
                project.clone(),
                thread.downgrade(),
                languages.clone(),
            ));
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let edit = cx.update(|cx| {
                tool.run(
                    EditFileToolInput {
                        display_description: "Edit file".into(),
                        path: path!("/main.rs").into(),
                        mode: EditFileMode::Edit,
                    },
                    stream_tx,
                    cx,
                )
            });
            stream_rx.expect_update_fields().await;
            let diff = stream_rx.expect_diff().await;
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Pending(_))));
            edit.await.unwrap_err();
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Finalized(_))));
            model.allow_requests();
        }

        // Ensure the diff is finalized if the tool call gets dropped.
        {
            let tool = Arc::new(EditFileTool::new(
                project.clone(),
                thread.downgrade(),
                languages.clone(),
            ));
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let edit = cx.update(|cx| {
                tool.run(
                    EditFileToolInput {
                        display_description: "Edit file".into(),
                        path: path!("/main.rs").into(),
                        mode: EditFileMode::Edit,
                    },
                    stream_tx,
                    cx,
                )
            });
            stream_rx.expect_update_fields().await;
            let diff = stream_rx.expect_diff().await;
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Pending(_))));
            drop(edit);
            cx.run_until_parked();
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Finalized(_))));
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            TelemetrySettings::register(cx);
            agent_settings::AgentSettings::register(cx);
            Project::init_settings(cx);
        });
    }
}

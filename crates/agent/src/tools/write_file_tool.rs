use super::edit_session::{
    EditSession, EditSessionContext, EditSessionMode, EditSessionOutput, EditSessionResult,
    initial_title_from_partial_path, run_session,
};
use crate::{AgentTool, Thread, ToolCallEventStream, ToolInput, ToolInputPayload};
use action_log::ActionLog;
use agent_client_protocol::schema as acp;
use futures::FutureExt as _;
use gpui::{App, AsyncApp, Entity, Task, WeakEntity};
use language::LanguageRegistry;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use ui::SharedString;

const DEFAULT_UI_TEXT: &str = "Writing file";

/// This is a tool for creating a new file or overwriting an existing file with completely new contents.
///
/// To make granular edits to an existing file, prefer the `edit_file` tool instead.
///
/// Before using this tool:
///
/// 1. Verify the directory path is correct (only applicable when creating new files):
///    - Use the `list_directory` tool to verify the parent directory exists and is the correct location
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriteFileToolInput {
    /// The full path of the file to create or overwrite in the project.
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

    /// The complete content for the file.
    /// This field should contain the entire file content.
    pub content: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
struct WriteFileToolPartialInput {
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    content: Option<String>,
}

pub struct WriteFileTool {
    session_context: Arc<EditSessionContext>,
}

impl WriteFileTool {
    pub fn new(
        project: Entity<Project>,
        thread: WeakEntity<Thread>,
        action_log: Entity<ActionLog>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            session_context: Arc::new(EditSessionContext::new(
                project,
                thread,
                action_log,
                language_registry,
            )),
        }
    }

    async fn process_streaming_writes(
        &self,
        input: &mut ToolInput<WriteFileToolInput>,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> EditSessionResult {
        let mut session: Option<EditSession> = None;
        let mut last_path: Option<String> = None;

        loop {
            futures::select! {
                payload = input.next().fuse() => {
                    match payload {
                        Ok(payload) => match payload {
                            ToolInputPayload::Partial(partial) => {
                                if let Ok(parsed) = serde_json::from_value::<WriteFileToolPartialInput>(partial) {
                                    let path_complete = parsed.path.is_some()
                                        && parsed.path.as_ref() == last_path.as_ref();

                                    last_path = parsed.path.clone();

                                    if session.is_none()
                                        && path_complete
                                        && let Some(path) = parsed.path.as_ref()
                                    {
                                        match EditSession::new(
                                            PathBuf::from(path),
                                            EditSessionMode::Write,
                                            Self::NAME,
                                            self.session_context.clone(),
                                            event_stream,
                                            cx,
                                        )
                                        .await
                                        {
                                            Ok(created_session) => session = Some(created_session),
                                            Err(error) => {
                                                log::error!("Failed to create edit session: {}", error);
                                                return EditSessionResult::Failed {
                                                    error,
                                                    session: None,
                                                };
                                            }
                                        }
                                    }

                                    if let Some(current_session) = &mut session
                                        && let Err(error) = current_session.process_write(parsed.content.as_deref(), cx)
                                    {
                                        log::error!("Failed to process write: {}", error);
                                        return EditSessionResult::Failed { error, session };
                                    }
                                }
                            }
                            ToolInputPayload::Full(full_input) => {
                                let mut session = if let Some(session) = session {
                                    session
                                } else {
                                    match EditSession::new(
                                        full_input.path.clone(),
                                        EditSessionMode::Write,
                                        Self::NAME,
                                        self.session_context.clone(),
                                        event_stream,
                                        cx,
                                    )
                                    .await
                                    {
                                        Ok(created_session) => created_session,
                                        Err(error) => {
                                            log::error!("Failed to create edit session: {}", error);
                                            return EditSessionResult::Failed {
                                                error,
                                                session: None,
                                            };
                                        }
                                    }
                                };

                                return match session.finalize_write(&full_input.content, cx).await {
                                    Ok(()) => EditSessionResult::Completed(session),
                                    Err(error) => {
                                        log::error!("Failed to finalize write: {}", error);
                                        EditSessionResult::Failed {
                                            error,
                                            session: Some(session),
                                        }
                                    }
                                };
                            }
                            ToolInputPayload::InvalidJson { error_message } => {
                                log::error!("Received invalid JSON: {error_message}");
                                return EditSessionResult::Failed {
                                    error: error_message,
                                    session,
                                };
                            }
                        },
                        Err(error) => {
                            return EditSessionResult::Failed {
                                error: error.to_string(),
                                session,
                            };
                        }
                    }
                }
                _ = event_stream.cancelled_by_user().fuse() => {
                    return EditSessionResult::Failed {
                        error: "Write cancelled by user".to_string(),
                        session,
                    };
                }
            }
        }
    }
}

impl AgentTool for WriteFileTool {
    type Input = WriteFileToolInput;
    type Output = EditSessionOutput;

    const NAME: &'static str = "write_file";

    fn supports_input_streaming() -> bool {
        true
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
            Ok(input) => {
                self.session_context
                    .initial_title_from_path(&input.path, DEFAULT_UI_TEXT, cx)
            }
            Err(raw_input) => initial_title_from_partial_path::<WriteFileToolPartialInput>(
                &self.session_context,
                raw_input,
                |partial| partial.path.clone(),
                DEFAULT_UI_TEXT,
                cx,
            ),
        }
    }

    fn run(
        self: Arc<Self>,
        mut input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx: &mut AsyncApp| {
            run_session(
                self.process_streaming_writes(&mut input, &event_stream, cx)
                    .await,
                cx,
            )
            .await
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        self.session_context.replay_output(output, event_stream, cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AgentTool, ContextServerRegistry, Templates, Thread, ToolCallEventStream, ToolInput,
        ToolInputSender,
    };
    use acp_thread::Diff;
    use action_log::ActionLog;
    use fs::Fs as _;
    use futures::StreamExt as _;
    use gpui::{AppContext as _, Entity, TestAppContext, UpdateGlobal};
    use language::language_settings::FormatOnSave;
    use language_model::fake_provider::FakeLanguageModel;
    use project::{Project, ProjectPath};
    use prompt_store::ProjectContext;
    use serde_json::json;
    use settings::{Settings, SettingsStore};
    use std::sync::Arc;
    use util::path;
    use util::rel_path::{RelPath, rel_path};

    #[gpui::test]
    async fn test_streaming_write_create_file(cx: &mut TestAppContext) {
        let (write_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"dir": {}})).await;
        let result = cx
            .update(|cx| {
                write_tool.clone().run(
                    ToolInput::resolved(WriteFileToolInput {
                        path: "root/dir/new_file.txt".into(),
                        content: "Hello, World!".into(),
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditSessionOutput::Success { new_text, diff, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "Hello, World!");
        assert!(!diff.is_empty());
    }

    #[gpui::test]
    async fn test_streaming_write_overwrite_file(cx: &mut TestAppContext) {
        let (write_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "old content"})).await;
        let result = cx
            .update(|cx| {
                write_tool.clone().run(
                    ToolInput::resolved(WriteFileToolInput {
                        path: "root/file.txt".into(),
                        content: "new content".into(),
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditSessionOutput::Success {
            new_text, old_text, ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new content");
        assert_eq!(*old_text, "old content");
    }

    #[gpui::test]
    async fn test_streaming_path_completeness_heuristic(cx: &mut TestAppContext) {
        let (write_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world"})).await;
        let (mut sender, input) = ToolInput::<WriteFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| write_tool.clone().run(input, event_stream, cx));

        // Send partial with path but NO mode — path should NOT be treated as complete
        sender.send_partial(json!({
            "path": "root/file"
        }));
        cx.run_until_parked();

        // Now the path grows and mode appears
        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // Send final
        sender.send_full(json!({
            "path": "root/file.txt",
            "content": "new content"
        }));

        let result = task.await;
        let EditSessionOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new content");
    }

    #[gpui::test]
    async fn test_streaming_create_file_with_partials(cx: &mut TestAppContext) {
        let (write_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"dir": {}})).await;
        let (mut sender, input) = ToolInput::<WriteFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| write_tool.clone().run(input, event_stream, cx));

        // Stream partials for create mode
        sender.send_partial(json!({}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/dir/new_file.txt",
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/dir/new_file.txt",
            "content": "Hello, "
        }));
        cx.run_until_parked();

        // Final with full content
        sender.send_full(json!({
            "path": "root/dir/new_file.txt",
            "content": "Hello, World!"
        }));

        let result = task.await;
        let EditSessionOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "Hello, World!");
    }

    #[gpui::test]
    async fn test_streaming_input_recv_drains_partials(cx: &mut TestAppContext) {
        let (write_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"dir": {}})).await;
        // Create a channel and send multiple partials before a final, then use
        // ToolInput::resolved-style immediate delivery to confirm recv() works
        // when partials are already buffered.
        let (mut sender, input): (ToolInputSender, ToolInput<WriteFileToolInput>) =
            ToolInput::test();
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| write_tool.clone().run(input, event_stream, cx));

        // Buffer several partials before sending the final
        sender.send_partial(json!({}));
        sender.send_partial(json!({"path": "root/dir/new.txt"}));
        sender.send_partial(json!({
            "path": "root/dir/new.txt",
        }));
        sender.send_full(json!({
            "path": "root/dir/new.txt",
            "content": "streamed content"
        }));

        let result = task.await;
        let EditSessionOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "streamed content");
    }

    #[gpui::test]
    async fn test_streaming_resolve_path_for_creating_file(cx: &mut TestAppContext) {
        let mode = EditSessionMode::Write;

        let result = test_resolve_path(&mode, "root/new.txt", cx);
        assert_resolved_path_eq(result.await, rel_path("new.txt"));

        let result = test_resolve_path(&mode, "new.txt", cx);
        assert_resolved_path_eq(result.await, rel_path("new.txt"));

        let result = test_resolve_path(&mode, "dir/new.txt", cx);
        assert_resolved_path_eq(result.await, rel_path("dir/new.txt"));

        let result = test_resolve_path(&mode, "root/dir/subdir/existing.txt", cx);
        assert_resolved_path_eq(result.await, rel_path("dir/subdir/existing.txt"));

        let result = test_resolve_path(&mode, "root/dir/subdir", cx);
        assert_eq!(
            result.await.unwrap_err(),
            "Can't write to file: path is a directory"
        );

        let result = test_resolve_path(&mode, "root/dir/nonexistent_dir/new.txt", cx);
        assert_eq!(
            result.await.unwrap_err(),
            "Can't create file: parent directory doesn't exist"
        );
    }

    #[gpui::test]
    async fn test_streaming_format_on_save(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"src": {}})).await;
        let (write_tool, project, action_log, fs, thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

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

        const UNFORMATTED_CONTENT: &str = "fn main() {println!(\"Hello!\");}\
";
        const FORMATTED_CONTENT: &str = "This file was formatted by the fake formatter in the test.\
";

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

        // Test with format_on_save enabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.format_on_save = Some(FormatOnSave::On);
                    settings.project.all_languages.defaults.formatter =
                        Some(language::language_settings::FormatterList::default());
                });
            });
        });

        // Use streaming pattern so executor can pump the LSP request/response
        let (mut sender, input) = ToolInput::<WriteFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let task = cx.update(|cx| write_tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "path": "root/src/main.rs",
        }));
        cx.run_until_parked();

        sender.send_full(json!({
            "path": "root/src/main.rs",
            "content": UNFORMATTED_CONTENT
        }));

        let result = task.await;
        assert!(result.is_ok());

        cx.executor().run_until_parked();

        let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            new_content.replace("\r\n", "\n"),
            FORMATTED_CONTENT,
            "Code should be formatted when format_on_save is enabled"
        );

        let stale_buffer_count = thread
            .read_with(cx, |thread, _cx| thread.action_log.clone())
            .read_with(cx, |log, cx| log.stale_buffers(cx).count());

        assert_eq!(
            stale_buffer_count, 0,
            "BUG: Buffer is incorrectly marked as stale after format-on-save. Found {} stale buffers.",
            stale_buffer_count
        );

        // Test with format_on_save disabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.format_on_save =
                        Some(FormatOnSave::Off);
                });
            });
        });

        let (mut sender, input) = ToolInput::<WriteFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool2 = Arc::new(WriteFileTool::new(
            project.clone(),
            thread.downgrade(),
            action_log.clone(),
            language_registry,
        ));

        let task = cx.update(|cx| tool2.run(input, event_stream, cx));

        sender.send_partial(json!({
            "path": "root/src/main.rs",
        }));
        cx.run_until_parked();

        sender.send_full(json!({
            "path": "root/src/main.rs",
            "content": UNFORMATTED_CONTENT
        }));

        let result = task.await;
        assert!(result.is_ok());

        cx.executor().run_until_parked();

        let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            new_content.replace("\r\n", "\n"),
            UNFORMATTED_CONTENT,
            "Code should not be formatted when format_on_save is disabled"
        );
    }

    #[gpui::test]
    async fn test_streaming_remove_trailing_whitespace(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"src": {}})).await;
        fs.save(
            path!("/root/src/main.rs").as_ref(),
            &"initial content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();
        let (write_tool, project, action_log, fs, thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;
        let language_registry = project.read_with(cx, |p, _cx| p.languages().clone());

        // Test with remove_trailing_whitespace_on_save enabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .remove_trailing_whitespace_on_save = Some(true);
                });
            });
        });

        const CONTENT_WITH_TRAILING_WHITESPACE: &str =
            "fn main() {  \n    println!(\"Hello!\");  \n}\n";

        let result = cx
            .update(|cx| {
                write_tool.clone().run(
                    ToolInput::resolved(WriteFileToolInput {
                        path: "root/src/main.rs".into(),
                        content: CONTENT_WITH_TRAILING_WHITESPACE.into(),
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(result.is_ok());

        cx.executor().run_until_parked();

        assert_eq!(
            fs.load(path!("/root/src/main.rs").as_ref())
                .await
                .unwrap()
                .replace("\r\n", "\n"),
            "fn main() {\n    println!(\"Hello!\");\n}\n",
            "Trailing whitespace should be removed when remove_trailing_whitespace_on_save is enabled"
        );

        // Test with remove_trailing_whitespace_on_save disabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .remove_trailing_whitespace_on_save = Some(false);
                });
            });
        });

        let tool2 = Arc::new(WriteFileTool::new(
            project.clone(),
            thread.downgrade(),
            action_log.clone(),
            language_registry,
        ));

        let result = cx
            .update(|cx| {
                tool2.run(
                    ToolInput::resolved(WriteFileToolInput {
                        path: "root/src/main.rs".into(),
                        content: CONTENT_WITH_TRAILING_WHITESPACE.into(),
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(result.is_ok());

        cx.executor().run_until_parked();

        let final_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            final_content.replace("\r\n", "\n"),
            CONTENT_WITH_TRAILING_WHITESPACE,
            "Trailing whitespace should remain when remove_trailing_whitespace_on_save is disabled"
        );
    }

    #[gpui::test]
    async fn test_streaming_diff_finalization(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/", json!({"main.rs": ""})).await;
        let (write_tool, project, action_log, _fs, thread) =
            setup_test_with_fs(cx, fs, &[path!("/").as_ref()]).await;
        let language_registry = project.read_with(cx, |p, _cx| p.languages().clone());

        // Ensure the diff is finalized after the edit completes.
        {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let edit = cx.update(|cx| {
                write_tool.clone().run(
                    ToolInput::resolved(WriteFileToolInput {
                        path: path!("/main.rs").into(),
                        content: "new content".into(),
                    }),
                    stream_tx,
                    cx,
                )
            });
            stream_rx.expect_update_fields().await;
            let diff = stream_rx.expect_diff().await;
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Pending(_))));
            cx.run_until_parked();
            edit.await.unwrap();
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Finalized(_))));
        }

        // Ensure the diff is finalized if the tool call gets dropped.
        {
            let tool = Arc::new(WriteFileTool::new(
                project.clone(),
                thread.downgrade(),
                action_log,
                language_registry,
            ));
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let edit = cx.update(|cx| {
                tool.run(
                    ToolInput::resolved(WriteFileToolInput {
                        path: path!("/main.rs").into(),
                        content: "dropped content".into(),
                    }),
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

    #[gpui::test]
    async fn test_streaming_create_content_streamed(cx: &mut TestAppContext) {
        let (write_tool, project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"dir": {}})).await;
        let (mut sender, input) = ToolInput::<WriteFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| write_tool.clone().run(input, event_stream, cx));

        // Transition to BufferResolved
        sender.send_partial(json!({
            "path": "root/dir/new_file.txt",
        }));
        cx.run_until_parked();

        // Stream content incrementally
        sender.send_partial(json!({
            "path": "root/dir/new_file.txt",
            "content": "line 1\n"
        }));
        cx.run_until_parked();

        // Verify buffer has partial content
        let buffer = project.update(cx, |project, cx| {
            let path = project
                .find_project_path("root/dir/new_file.txt", cx)
                .unwrap();
            project.get_open_buffer(&path, cx).unwrap()
        });
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), "line 1\n");

        // Stream more content
        sender.send_partial(json!({
            "path": "root/dir/new_file.txt",
            "content": "line 1\nline 2\n"
        }));
        cx.run_until_parked();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), "line 1\nline 2\n");

        // Stream final chunk
        sender.send_partial(json!({
            "path": "root/dir/new_file.txt",
            "content": "line 1\nline 2\nline 3\n"
        }));
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "line 1\nline 2\nline 3\n"
        );

        // Send final input
        sender.send_full(json!({
            "path": "root/dir/new_file.txt",
            "content": "line 1\nline 2\nline 3\n"
        }));

        let result = task.await;
        let EditSessionOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nline 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_overwrite_diff_revealed_during_streaming(cx: &mut TestAppContext) {
        let (write_tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "old line 1\nold line 2\nold line 3\n"}),
        )
        .await;
        let (mut sender, input) = ToolInput::<WriteFileToolInput>::test();
        let (event_stream, mut receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| write_tool.clone().run(input, event_stream, cx));

        // Transition to BufferResolved
        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // Get the diff entity from the event stream
        receiver.expect_update_fields().await;
        let diff = receiver.expect_diff().await;

        // Diff starts pending with no revealed ranges
        diff.read_with(cx, |diff, cx| {
            assert!(matches!(diff, Diff::Pending(_)));
            assert!(!diff.has_revealed_range(cx));
        });

        // Stream first content chunk
        sender.send_partial(json!({
            "path": "root/file.txt",
            "content": "new line 1\n"
        }));
        cx.run_until_parked();

        // Diff should now have revealed ranges showing the new content
        diff.read_with(cx, |diff, cx| {
            assert!(diff.has_revealed_range(cx));
        });

        // Send final input
        sender.send_full(json!({
            "path": "root/file.txt",
            "content": "new line 1\nnew line 2\n"
        }));

        let result = task.await;
        let EditSessionOutput::Success {
            new_text, old_text, ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new line 1\nnew line 2\n");
        assert_eq!(*old_text, "old line 1\nold line 2\nold line 3\n");

        // Diff is finalized after completion
        diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Finalized(_))));
    }

    #[gpui::test]
    async fn test_streaming_overwrite_content_streamed(cx: &mut TestAppContext) {
        let (write_tool, project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "old line 1\nold line 2\nold line 3\n"}),
        )
        .await;
        let (mut sender, input) = ToolInput::<WriteFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| write_tool.clone().run(input, event_stream, cx));

        // Transition to BufferResolved
        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // Verify buffer still has old content (no content partial yet)
        let buffer = project.update(cx, |project, cx| {
            let path = project.find_project_path("root/file.txt", cx).unwrap();
            project.open_buffer(path, cx)
        });
        let buffer = buffer.await.unwrap();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "old line 1\nold line 2\nold line 3\n"
        );

        // First content partial replaces old content
        sender.send_partial(json!({
            "path": "root/file.txt",
            "content": "new line 1\n"
        }));
        cx.run_until_parked();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), "new line 1\n");

        // Subsequent content partials append
        sender.send_partial(json!({
            "path": "root/file.txt",
            "content": "new line 1\nnew line 2\n"
        }));
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "new line 1\nnew line 2\n"
        );

        // Send final input with complete content
        sender.send_full(json!({
            "path": "root/file.txt",
            "content": "new line 1\nnew line 2\nnew line 3\n"
        }));

        let result = task.await;
        let EditSessionOutput::Success {
            new_text, old_text, ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new line 1\nnew line 2\nnew line 3\n");
        assert_eq!(*old_text, "old line 1\nold line 2\nold line 3\n");
    }

    #[gpui::test]
    async fn test_streaming_write_file_tool_registers_changed_buffers(cx: &mut TestAppContext) {
        let (write_tool, _project, action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "original content"})).await;
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (event_stream, _rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            write_tool.clone().run(
                ToolInput::resolved(WriteFileToolInput {
                    path: "root/file.txt".into(),
                    content: "completely new content".into(),
                }),
                event_stream,
                cx,
            )
        });

        let result = task.await;
        assert!(result.is_ok(), "write should succeed: {:?}", result.err());

        cx.run_until_parked();

        let changed = action_log.read_with(cx, |log, cx| log.changed_buffers(cx));
        assert!(
            !changed.is_empty(),
            "action_log.changed_buffers() should be non-empty after streaming write, \
             but no changed buffers were found \u{2014} Accept All / Reject All will not appear"
        );
    }

    #[gpui::test]
    async fn test_streaming_write_file_tool_fields_out_of_order(cx: &mut TestAppContext) {
        let (write_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "old_content"})).await;
        let (mut sender, input) = ToolInput::<WriteFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| write_tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "content": "new_content"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "content": "new_content",
            "path": "root"
        }));
        cx.run_until_parked();

        // Send final.
        sender.send_full(json!({
            "content": "new_content",
            "path": "root/file.txt"
        }));

        let result = task.await;
        let EditSessionOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new_content");
    }

    #[gpui::test]
    async fn test_streaming_reject_created_file_deletes_it(cx: &mut TestAppContext) {
        let (write_tool, _project, action_log, fs, _thread) =
            setup_test(cx, json!({"dir": {}})).await;
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        // Create a new file via the streaming write file tool
        let (event_stream, _rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            write_tool.clone().run(
                ToolInput::resolved(WriteFileToolInput {
                    path: "root/dir/new_file.txt".into(),
                    content: "Hello, World!".into(),
                }),
                event_stream,
                cx,
            )
        });
        let result = task.await;
        assert!(result.is_ok(), "create should succeed: {:?}", result.err());
        cx.run_until_parked();

        assert!(
            fs.is_file(path!("/root/dir/new_file.txt").as_ref()).await,
            "file should exist after creation"
        );

        // Reject all edits — this should delete the newly created file
        let changed = action_log.read_with(cx, |log, cx| log.changed_buffers(cx));
        assert!(
            !changed.is_empty(),
            "action_log should track the created file as changed"
        );

        action_log
            .update(cx, |log, cx| log.reject_all_edits(None, cx))
            .await;
        cx.run_until_parked();

        assert!(
            !fs.is_file(path!("/root/dir/new_file.txt").as_ref()).await,
            "file should be deleted after rejecting creation, but an empty file was left behind"
        );
    }

    async fn setup_test_with_fs(
        cx: &mut TestAppContext,
        fs: Arc<project::FakeFs>,
        worktree_paths: &[&std::path::Path],
    ) -> (
        Arc<WriteFileTool>,
        Entity<Project>,
        Entity<ActionLog>,
        Arc<project::FakeFs>,
        Entity<Thread>,
    ) {
        let project = Project::test(fs.clone(), worktree_paths.iter().copied(), cx).await;
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
        let action_log = thread.read_with(cx, |thread, _| thread.action_log().clone());
        let write_tool = Arc::new(WriteFileTool::new(
            project.clone(),
            thread.downgrade(),
            action_log.clone(),
            language_registry,
        ));
        (write_tool, project, action_log, fs, thread)
    }

    async fn setup_test(
        cx: &mut TestAppContext,
        initial_tree: serde_json::Value,
    ) -> (
        Arc<WriteFileTool>,
        Entity<Project>,
        Entity<ActionLog>,
        Arc<project::FakeFs>,
        Entity<Thread>,
    ) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", initial_tree).await;
        setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await
    }

    async fn test_resolve_path(
        mode: &EditSessionMode,
        path: &str,
        cx: &mut TestAppContext,
    ) -> Result<ProjectPath, String> {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "dir": {
                    "subdir": {
                        "existing.txt": "content"
                    }
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        crate::tools::edit_session::test_resolve_path(mode, path, &project, cx).await
    }

    #[track_caller]
    fn assert_resolved_path_eq(path: Result<ProjectPath, String>, expected: &RelPath) {
        let actual = path.expect("Should return valid path").path;
        assert_eq!(actual.as_ref(), expected);
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .ensure_final_newline_on_save = Some(false);
                });
            });
        });
    }
}

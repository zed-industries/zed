use super::deserialize_maybe_stringified;
pub(crate) use super::edit_session::PartialEdit;
pub use super::edit_session::{Edit, EditSessionOutput as EditFileToolOutput};
use super::edit_session::{
    EditSession, EditSessionContext, EditSessionMode, EditSessionResult,
    initial_title_from_partial_path, run_session,
};
use crate::{AgentTool, Thread, ToolCallEventStream, ToolInput, ToolInputPayload};
use action_log::ActionLog;
use agent_client_protocol::schema as acp;
use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, AsyncApp, Entity, Task, WeakEntity};
use language::LanguageRegistry;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use ui::SharedString;

const DEFAULT_UI_TEXT: &str = "Editing file";

/// This is a tool for applying edits to an existing file.
///
/// Before using this tool:
///
/// 1. Use the `read_file` tool to understand the file's contents and context
///
/// To create a new file or overwrite an existing one with completely new contents, use the `write_file` tool instead.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFileToolInput {
    /// The full path of the file to edit in the project.
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

    /// List of edit operations to apply sequentially.
    /// Each edit finds `old_text` in the file and replaces it with `new_text`.
    #[serde(deserialize_with = "deserialize_maybe_stringified")]
    pub edits: Vec<Edit>,
}

#[derive(Clone, Default, Debug, Deserialize)]
struct EditFileToolPartialInput {
    #[serde(default)]
    path: Option<String>,
    #[serde(default, deserialize_with = "deserialize_maybe_stringified")]
    edits: Option<Vec<PartialEdit>>,
}

pub struct EditFileTool {
    session_context: Arc<EditSessionContext>,
}

impl EditFileTool {
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

    #[cfg(test)]
    fn authorize(
        &self,
        path: &PathBuf,
        event_stream: &ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<()>> {
        self.session_context
            .authorize(Self::NAME, path, event_stream, cx)
    }

    async fn process_streaming_edits(
        &self,
        input: &mut ToolInput<EditFileToolInput>,
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
                                if let Ok(parsed) = serde_json::from_value::<EditFileToolPartialInput>(partial) {
                                    let path_complete = parsed.path.is_some()
                                        && parsed.path.as_ref() == last_path.as_ref();

                                    last_path = parsed.path.clone();

                                    if session.is_none()
                                        && path_complete
                                        && let Some(path) = parsed.path.as_ref()
                                    {
                                        match EditSession::new(
                                            PathBuf::from(path),
                                            EditSessionMode::Edit,
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
                                        && let Err(error) = current_session.process_edit(parsed.edits.as_deref(), event_stream, cx)
                                    {
                                        log::error!("Failed to process edit: {}", error);
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
                                        EditSessionMode::Edit,
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

                                return match session.finalize_edit(full_input.edits, event_stream, cx).await {
                                    Ok(()) => EditSessionResult::Completed(session),
                                    Err(error) => {
                                        log::error!("Failed to finalize edit: {}", error);
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
                        error: "Edit cancelled by user".to_string(),
                        session,
                    };
                }
            }
        }
    }
}

impl AgentTool for EditFileTool {
    type Input = EditFileToolInput;
    type Output = EditFileToolOutput;

    const NAME: &'static str = "edit_file";

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
            Err(raw_input) => initial_title_from_partial_path::<EditFileToolPartialInput>(
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
                self.process_streaming_edits(&mut input, &event_stream, cx)
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
    ) -> Result<()> {
        self.session_context.replay_output(output, event_stream, cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContextServerRegistry, Templates, ToolInputSender};
    use fs::Fs as _;
    use gpui::{AppContext as _, TestAppContext, UpdateGlobal};
    use language_model::fake_provider::FakeLanguageModel;
    use project::ProjectPath;
    use prompt_store::ProjectContext;
    use serde_json::json;
    use settings::Settings;
    use settings::SettingsStore;
    use util::path;
    use util::rel_path::{RelPath, rel_path};

    #[gpui::test]
    async fn test_streaming_edit_granular_edits(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/file.txt".into(),
                        edits: vec![Edit {
                            old_text: "line 2".into(),
                            new_text: "modified line 2".into(),
                        }],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_edit_multiple_edits(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/file.txt".into(),
                        edits: vec![
                            Edit {
                                old_text: "line 5".into(),
                                new_text: "modified line 5".into(),
                            },
                            Edit {
                                old_text: "line 1".into(),
                                new_text: "modified line 1".into(),
                            },
                        ],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(
            new_text,
            "modified line 1\nline 2\nline 3\nline 4\nmodified line 5\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_adjacent_edits(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/file.txt".into(),
                        edits: vec![
                            Edit {
                                old_text: "line 2".into(),
                                new_text: "modified line 2".into(),
                            },
                            Edit {
                                old_text: "line 3".into(),
                                new_text: "modified line 3".into(),
                            },
                        ],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(
            new_text,
            "line 1\nmodified line 2\nmodified line 3\nline 4\nline 5\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_ascending_order_edits(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/file.txt".into(),
                        edits: vec![
                            Edit {
                                old_text: "line 1".into(),
                                new_text: "modified line 1".into(),
                            },
                            Edit {
                                old_text: "line 5".into(),
                                new_text: "modified line 5".into(),
                            },
                        ],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(
            new_text,
            "modified line 1\nline 2\nline 3\nline 4\nmodified line 5\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_nonexistent_file(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) = setup_test(cx, json!({})).await;
        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/nonexistent_file.txt".into(),
                        edits: vec![Edit {
                            old_text: "foo".into(),
                            new_text: "bar".into(),
                        }],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditFileToolOutput::Error {
            error,
            diff,
            input_path,
        } = result.unwrap_err()
        else {
            panic!("expected error");
        };
        assert_eq!(error, "Can't edit file: path not found");
        assert!(diff.is_empty());
        assert_eq!(input_path, None);
    }

    #[gpui::test]
    async fn test_streaming_edit_failed_match(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world"})).await;
        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/file.txt".into(),
                        edits: vec![Edit {
                            old_text: "nonexistent text that is not in the file".into(),
                            new_text: "replacement".into(),
                        }],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditFileToolOutput::Error { error, .. } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("Could not find matching text"),
            "Expected error containing 'Could not find matching text' but got: {error}"
        );
    }

    #[gpui::test]
    async fn test_streaming_early_buffer_open(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Send partials simulating LLM streaming: description first, then path, then mode
        sender.send_partial(json!({}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt"
        }));
        cx.run_until_parked();

        // Path is NOT yet complete because mode hasn't appeared — no buffer open yet
        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // Now send the final complete input
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "line 2", "new_text": "modified line 2"}]
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_cancellation_during_partials(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver, mut cancellation_tx) =
            ToolCallEventStream::test_with_cancellation();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Send a partial
        sender.send_partial(json!({}));
        cx.run_until_parked();

        // Cancel during streaming
        ToolCallEventStream::signal_cancellation_with_sender(&mut cancellation_tx);
        cx.run_until_parked();

        // The sender is still alive so the partial loop should detect cancellation
        // We need to drop the sender to also unblock recv() if the loop didn't catch it
        drop(sender);

        let result = task.await;
        let EditFileToolOutput::Error { error, .. } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("cancelled"),
            "Expected cancellation error but got: {error}"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_with_multiple_partials(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Simulate fine-grained streaming of the JSON
        sender.send_partial(json!({}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "line 1"}]
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "line 1", "new_text": "modified line 1"},
                {"old_text": "line 5"}
            ]
        }));
        cx.run_until_parked();

        // Send final complete input
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "line 1", "new_text": "modified line 1"},
                {"old_text": "line 5", "new_text": "modified line 5"}
            ]
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(
            new_text,
            "modified line 1\nline 2\nline 3\nline 4\nmodified line 5\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_no_partials_direct_final(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Send final immediately with no partials (simulates non-streaming path)
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "line 2", "new_text": "modified line 2"}]
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_incremental_edit_application(cx: &mut TestAppContext) {
        let (edit_tool, project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Stream description, path, mode
        sender.send_partial(json!({}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // First edit starts streaming (old_text only, still in progress)
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "line 1"}]
        }));
        cx.run_until_parked();

        // Buffer should not have changed yet — the first edit is still in progress
        // (no second edit has appeared to prove the first is complete)
        let buffer_text = project.update(cx, |project, cx| {
            let project_path = project.find_project_path(&PathBuf::from("root/file.txt"), cx);
            project_path.and_then(|pp| {
                project
                    .get_open_buffer(&pp, cx)
                    .map(|buffer| buffer.read(cx).text())
            })
        });
        // Buffer is open (from streaming) but edit 1 is still in-progress
        assert_eq!(
            buffer_text.as_deref(),
            Some("line 1\nline 2\nline 3\nline 4\nline 5\n"),
            "Buffer should not be modified while first edit is still in progress"
        );

        // Second edit appears — this proves the first edit is complete, so it
        // should be applied immediately during streaming
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED 1"},
                {"old_text": "line 5"}
            ]
        }));
        cx.run_until_parked();

        // First edit should now be applied to the buffer
        let buffer_text = project.update(cx, |project, cx| {
            let project_path = project.find_project_path(&PathBuf::from("root/file.txt"), cx);
            project_path.and_then(|pp| {
                project
                    .get_open_buffer(&pp, cx)
                    .map(|buffer| buffer.read(cx).text())
            })
        });
        assert_eq!(
            buffer_text.as_deref(),
            Some("MODIFIED 1\nline 2\nline 3\nline 4\nline 5\n"),
            "First edit should be applied during streaming when second edit appears"
        );

        // Send final complete input
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED 1"},
                {"old_text": "line 5", "new_text": "MODIFIED 5"}
            ]
        }));

        let result = task.await;
        let EditFileToolOutput::Success {
            new_text, old_text, ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };
        assert_eq!(new_text, "MODIFIED 1\nline 2\nline 3\nline 4\nMODIFIED 5\n");
        assert_eq!(
            *old_text, "line 1\nline 2\nline 3\nline 4\nline 5\n",
            "old_text should reflect the original file content before any edits"
        );
    }

    #[gpui::test]
    async fn test_streaming_incremental_three_edits(cx: &mut TestAppContext) {
        let (edit_tool, project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "aaa\nbbb\nccc\nddd\neee\n"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Setup: description + path + mode
        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // Edit 1 in progress
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "aaa", "new_text": "AAA"}]
        }));
        cx.run_until_parked();

        // Edit 2 appears — edit 1 is now complete and should be applied
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "aaa", "new_text": "AAA"},
                {"old_text": "ccc", "new_text": "CCC"}
            ]
        }));
        cx.run_until_parked();

        // Verify edit 1 fully applied. Edit 2's new_text is being
        // streamed: "CCC" is inserted but the old "ccc" isn't deleted
        // yet (StreamingDiff::finish runs when edit 3 marks edit 2 done).
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(buffer_text.as_deref(), Some("AAA\nbbb\nCCCccc\nddd\neee\n"));

        // Edit 3 appears — edit 2 is now complete and should be applied
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "aaa", "new_text": "AAA"},
                {"old_text": "ccc", "new_text": "CCC"},
                {"old_text": "eee", "new_text": "EEE"}
            ]
        }));
        cx.run_until_parked();

        // Verify edits 1 and 2 fully applied. Edit 3's new_text is being
        // streamed: "EEE" is inserted but old "eee" isn't deleted yet.
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(buffer_text.as_deref(), Some("AAA\nbbb\nCCC\nddd\nEEEeee\n"));

        // Send final
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "aaa", "new_text": "AAA"},
                {"old_text": "ccc", "new_text": "CCC"},
                {"old_text": "eee", "new_text": "EEE"}
            ]
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "AAA\nbbb\nCCC\nddd\nEEE\n");
    }

    #[gpui::test]
    async fn test_streaming_edit_failure_mid_stream(cx: &mut TestAppContext) {
        let (edit_tool, project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Setup
        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // Edit 1 (valid) in progress — not yet complete (no second edit)
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED"}
            ]
        }));
        cx.run_until_parked();

        // Edit 2 appears (will fail to match) — this makes edit 1 complete.
        // Edit 1 should be applied. Edit 2 is still in-progress (last edit).
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED"},
                {"old_text": "nonexistent text that does not appear anywhere in the file at all", "new_text": "whatever"}
            ]
        }));
        cx.run_until_parked();

        let buffer = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).unwrap()
        });

        // Verify edit 1 was applied
        let buffer_text = buffer.read_with(cx, |buffer, _cx| buffer.text());
        assert_eq!(
            buffer_text, "MODIFIED\nline 2\nline 3\n",
            "First edit should be applied even though second edit will fail"
        );

        // Edit 3 appears — this makes edit 2 "complete", triggering its
        // resolution which should fail (old_text doesn't exist in the file).
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED"},
                {"old_text": "nonexistent text that does not appear anywhere in the file at all", "new_text": "whatever"},
                {"old_text": "line 3", "new_text": "MODIFIED 3"}
            ]
        }));
        cx.run_until_parked();

        // The error from edit 2 should have propagated out of the partial loop.
        // Drop sender to unblock recv() if the loop didn't catch it.
        drop(sender);

        let result = task.await;
        let EditFileToolOutput::Error {
            error,
            diff,
            input_path,
        } = result.unwrap_err()
        else {
            panic!("expected error");
        };

        assert!(
            error.contains("Could not find matching text for edit at index 1"),
            "Expected error about edit 1 failing, got: {error}"
        );
        // Ensure that first edit was applied successfully and that we saved the buffer
        assert_eq!(input_path, Some(PathBuf::from("root/file.txt")));
        assert_eq!(
            diff,
            "@@ -1,3 +1,3 @@\n-line 1\n+MODIFIED\n line 2\n line 3\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_single_edit_no_incremental(cx: &mut TestAppContext) {
        let (edit_tool, project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world\n"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Setup + single edit that stays in-progress (no second edit to prove completion)
        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "hello world", "new_text": "goodbye world"}]
        }));
        cx.run_until_parked();

        // The edit's old_text and new_text both arrived in one partial, so
        // the old_text is resolved and new_text is being streamed via
        // StreamingDiff. The buffer reflects the in-progress diff (new text
        // inserted, old text not yet fully removed until finalization).
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(
            buffer_text.as_deref(),
            Some("goodbye worldhello world\n"),
            "In-progress streaming diff: new text inserted, old text not yet removed"
        );

        // Send final — the edit is applied during finalization
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "hello world", "new_text": "goodbye world"}]
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "goodbye world\n");
    }

    #[gpui::test]
    async fn test_streaming_input_partials_then_final(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let (mut sender, input): (ToolInputSender, ToolInput<EditFileToolInput>) =
            ToolInput::test();
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Send progressively more complete partial snapshots, as the LLM would
        sender.send_partial(json!({}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "line 2", "new_text": "modified line 2"}]
        }));
        cx.run_until_parked();

        // Send the final complete input
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "line 2", "new_text": "modified line 2"}]
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_input_sender_dropped_before_final(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world\n"})).await;
        let (mut sender, input): (ToolInputSender, ToolInput<EditFileToolInput>) =
            ToolInput::test();
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Send a partial then drop the sender without sending final
        sender.send_partial(json!({}));
        cx.run_until_parked();

        drop(sender);

        let result = task.await;
        assert!(
            result.is_err(),
            "Tool should error when sender is dropped without sending final input"
        );
    }

    #[gpui::test]
    async fn test_streaming_resolve_path_for_editing_file(cx: &mut TestAppContext) {
        let mode = EditSessionMode::Edit;

        let path_with_root = "root/dir/subdir/existing.txt";
        let path_without_root = "dir/subdir/existing.txt";
        let result = test_resolve_path(&mode, path_with_root, cx);
        assert_resolved_path_eq(result.await, rel_path(path_without_root));

        let result = test_resolve_path(&mode, path_without_root, cx);
        assert_resolved_path_eq(result.await, rel_path(path_without_root));

        let result = test_resolve_path(&mode, "root/nonexistent.txt", cx);
        assert_eq!(result.await.unwrap_err(), "Can't edit file: path not found");

        let result = test_resolve_path(&mode, "root/dir", cx);
        assert_eq!(
            result.await.unwrap_err(),
            "Can't edit file: path is a directory"
        );
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
                        "existing.txt": "hello"
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

    #[gpui::test]
    async fn test_streaming_authorize(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) = setup_test(cx, json!({})).await;

        // Test 1: Path with .zed component should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx
            .update(|cx| edit_tool.authorize(&PathBuf::from(".zed/settings.json"), &stream_tx, cx));

        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("Edit `.zed/settings.json` (local settings)".into())
        );

        // Test 2: Path outside project should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth =
            cx.update(|cx| edit_tool.authorize(&PathBuf::from("/etc/hosts"), &stream_tx, cx));

        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("Edit `/etc/hosts`".into())
        );

        // Test 3: Relative path without .zed should not require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| edit_tool.authorize(&PathBuf::from("root/src/main.rs"), &stream_tx, cx))
            .await
            .unwrap();
        assert!(stream_rx.try_recv().is_err());

        // Test 4: Path with .zed in the middle should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx.update(|cx| {
            edit_tool.authorize(&PathBuf::from("root/.zed/tasks.json"), &stream_tx, cx)
        });
        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("Edit `root/.zed/tasks.json` (local settings)".into())
        );

        // Test 5: When global default is allow, sensitive and outside-project
        // paths still require confirmation
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        // 5.1: .zed/settings.json is a sensitive path — still prompts
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx
            .update(|cx| edit_tool.authorize(&PathBuf::from(".zed/settings.json"), &stream_tx, cx));
        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("Edit `.zed/settings.json` (local settings)".into())
        );

        // 5.2: /etc/hosts is outside the project, but Allow auto-approves
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| edit_tool.authorize(&PathBuf::from("/etc/hosts"), &stream_tx, cx))
            .await
            .unwrap();
        assert!(stream_rx.try_recv().is_err());

        // 5.3: Normal in-project path with allow — no confirmation needed
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| edit_tool.authorize(&PathBuf::from("root/src/main.rs"), &stream_tx, cx))
            .await
            .unwrap();
        assert!(stream_rx.try_recv().is_err());

        // 5.4: With Confirm default, non-project paths still prompt
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Confirm;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth =
            cx.update(|cx| edit_tool.authorize(&PathBuf::from("/etc/hosts"), &stream_tx, cx));

        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("Edit `/etc/hosts`".into())
        );
    }

    #[gpui::test]
    async fn test_streaming_authorize_create_under_symlink_with_allow(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;
        fs.insert_tree("/outside", json!({})).await;
        fs.insert_symlink("/root/link", PathBuf::from("/outside"))
            .await;
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let authorize_task =
            cx.update(|cx| edit_tool.authorize(&PathBuf::from("link/new.txt"), &stream_tx, cx));

        let event = stream_rx.expect_authorization().await;
        assert!(
            event
                .tool_call
                .fields
                .title
                .as_deref()
                .is_some_and(|title| title.contains("points outside the project")),
            "Expected symlink escape authorization for create under external symlink"
        );

        event
            .response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();
        authorize_task.await.unwrap();
    }

    #[gpui::test]
    async fn test_streaming_edit_file_symlink_escape_requests_authorization(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/outside"),
            json!({
                "config.txt": "old content"
            }),
        )
        .await;
        fs.create_symlink(
            path!("/root/link_to_external").as_ref(),
            PathBuf::from("/outside"),
        )
        .await
        .unwrap();
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _authorize_task = cx.update(|cx| {
            edit_tool.authorize(
                &PathBuf::from("link_to_external/config.txt"),
                &stream_tx,
                cx,
            )
        });

        let auth = stream_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("points outside the project"),
            "title should mention symlink escape, got: {title}"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_file_symlink_escape_denied(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/outside"),
            json!({
                "config.txt": "old content"
            }),
        )
        .await;
        fs.create_symlink(
            path!("/root/link_to_external").as_ref(),
            PathBuf::from("/outside"),
        )
        .await
        .unwrap();
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let authorize_task = cx.update(|cx| {
            edit_tool.authorize(
                &PathBuf::from("link_to_external/config.txt"),
                &stream_tx,
                cx,
            )
        });

        let auth = stream_rx.expect_authorization().await;
        drop(auth); // deny by dropping

        let result = authorize_task.await;
        assert!(result.is_err(), "should fail when denied");
    }

    #[gpui::test]
    async fn test_streaming_edit_file_symlink_escape_honors_deny_policy(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.tools.insert(
                "edit_file".into(),
                agent_settings::ToolRules {
                    default: Some(settings::ToolPermissionMode::Deny),
                    ..Default::default()
                },
            );
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/outside"),
            json!({
                "config.txt": "old content"
            }),
        )
        .await;
        fs.create_symlink(
            path!("/root/link_to_external").as_ref(),
            PathBuf::from("/outside"),
        )
        .await
        .unwrap();
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| {
                edit_tool.authorize(
                    &PathBuf::from("link_to_external/config.txt"),
                    &stream_tx,
                    cx,
                )
            })
            .await;

        assert!(result.is_err(), "Tool should fail when policy denies");
        assert!(
            !matches!(
                stream_rx.try_recv(),
                Ok(Ok(crate::ThreadEvent::ToolCallAuthorization(_)))
            ),
            "Deny policy should not emit symlink authorization prompt",
        );
    }

    #[gpui::test]
    async fn test_streaming_authorize_global_config(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({})).await;
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/project").as_ref()]).await;

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
            let auth = cx.update(|cx| edit_tool.authorize(&PathBuf::from(path), &stream_tx, cx));

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                auth.await.unwrap();
                assert!(
                    stream_rx.try_recv().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
            }
        }
    }

    #[gpui::test]
    async fn test_streaming_needs_confirmation_with_multiple_worktrees(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
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
        let (edit_tool, _project, _action_log, _fs, _thread) = setup_test_with_fs(
            cx,
            fs,
            &[
                path!("/workspace/frontend").as_ref(),
                path!("/workspace/backend").as_ref(),
                path!("/workspace/shared").as_ref(),
            ],
        )
        .await;

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
            let auth = cx.update(|cx| edit_tool.authorize(&PathBuf::from(path), &stream_tx, cx));

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                auth.await.unwrap();
                assert!(
                    stream_rx.try_recv().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
            }
        }
    }

    #[gpui::test]
    async fn test_streaming_needs_confirmation_edge_cases(cx: &mut TestAppContext) {
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
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/project").as_ref()]).await;

        let test_cases = vec![
            ("", false, "Empty path is treated as project root"),
            ("/", true, "Root directory should be outside project"),
            (
                "project/../other",
                true,
                "Path with .. that goes outside of root directory",
            ),
            (
                "project/./src/file.rs",
                false,
                "Path with . should work normally",
            ),
            #[cfg(target_os = "windows")]
            ("C:\\Windows\\System32\\hosts", true, "Windows system path"),
            #[cfg(target_os = "windows")]
            ("project\\src\\main.rs", false, "Windows-style project path"),
        ];

        for (path, should_confirm, description) in test_cases {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let auth = cx.update(|cx| edit_tool.authorize(&PathBuf::from(path), &stream_tx, cx));

            cx.run_until_parked();

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                assert!(
                    stream_rx.try_recv().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
                auth.await.unwrap();
            }
        }
    }

    #[gpui::test]
    async fn test_streaming_needs_confirmation_with_different_modes(cx: &mut TestAppContext) {
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
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/project").as_ref()]).await;

        let modes = vec![EditSessionMode::Edit, EditSessionMode::Write];

        for _mode in modes {
            // Test .zed path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let _auth = cx.update(|cx| {
                edit_tool.authorize(&PathBuf::from("project/.zed/settings.json"), &stream_tx, cx)
            });

            stream_rx.expect_authorization().await;

            // Test outside path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let _auth = cx.update(|cx| {
                edit_tool.authorize(&PathBuf::from("/outside/file.txt"), &stream_tx, cx)
            });

            stream_rx.expect_authorization().await;

            // Test normal path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            cx.update(|cx| {
                edit_tool.authorize(&PathBuf::from("project/normal.txt"), &stream_tx, cx)
            })
            .await
            .unwrap();
            assert!(stream_rx.try_recv().is_err());
        }
    }

    #[gpui::test]
    async fn test_streaming_initial_title_with_partial_input(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({})).await;
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/project").as_ref()]).await;

        cx.update(|cx| {
            assert_eq!(
                edit_tool.initial_title(
                    Err(json!({
                        "path": "src/main.rs",
                    })),
                    cx
                ),
                "src/main.rs"
            );
            assert_eq!(
                edit_tool.initial_title(
                    Err(json!({
                        "path": "",
                    })),
                    cx
                ),
                DEFAULT_UI_TEXT
            );
            assert_eq!(
                edit_tool.initial_title(Err(serde_json::Value::Null), cx),
                DEFAULT_UI_TEXT
            );
        });
    }

    #[gpui::test]
    async fn test_streaming_consecutive_edits_work(cx: &mut TestAppContext) {
        let (edit_tool, project, action_log, _fs, _thread) =
            setup_test(cx, json!({"test.txt": "original content"})).await;
        let read_tool = Arc::new(crate::ReadFileTool::new(
            project.clone(),
            action_log.clone(),
            true,
        ));

        // Read the file first
        cx.update(|cx| {
            read_tool.clone().run(
                ToolInput::resolved(crate::ReadFileToolInput {
                    path: "root/test.txt".to_string(),
                    start_line: None,
                    end_line: None,
                }),
                ToolCallEventStream::test().0,
                cx,
            )
        })
        .await
        .unwrap();

        // First edit should work
        let edit_result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/test.txt".into(),
                        edits: vec![Edit {
                            old_text: "original content".into(),
                            new_text: "modified content".into(),
                        }],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(
            edit_result.is_ok(),
            "First edit should succeed, got error: {:?}",
            edit_result.as_ref().err()
        );

        // Second edit should also work because the edit updated the recorded read time
        let edit_result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/test.txt".into(),
                        edits: vec![Edit {
                            old_text: "modified content".into(),
                            new_text: "further modified content".into(),
                        }],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(
            edit_result.is_ok(),
            "Second consecutive edit should succeed, got error: {:?}",
            edit_result.as_ref().err()
        );
    }

    #[gpui::test]
    async fn test_streaming_external_modification_matching_edit_succeeds(cx: &mut TestAppContext) {
        let (edit_tool, project, action_log, fs, _thread) =
            setup_test(cx, json!({"test.txt": "original content"})).await;
        let read_tool = Arc::new(crate::ReadFileTool::new(
            project.clone(),
            action_log.clone(),
            true,
        ));

        // Read the file first
        cx.update(|cx| {
            read_tool.clone().run(
                ToolInput::resolved(crate::ReadFileToolInput {
                    path: "root/test.txt".to_string(),
                    start_line: None,
                    end_line: None,
                }),
                ToolCallEventStream::test().0,
                cx,
            )
        })
        .await
        .unwrap();

        // Simulate external modification
        cx.background_executor
            .advance_clock(std::time::Duration::from_secs(2));
        fs.save(
            path!("/root/test.txt").as_ref(),
            &"externally modified content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();

        // Reload the buffer to pick up the new mtime
        let project_path = project
            .read_with(cx, |project, cx| {
                project.find_project_path("root/test.txt", cx)
            })
            .expect("Should find project path");
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
            .unwrap();
        buffer
            .update(cx, |buffer, cx| buffer.reload(cx))
            .await
            .unwrap();

        cx.executor().run_until_parked();

        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/test.txt".into(),
                        edits: vec![Edit {
                            old_text: "externally modified content".into(),
                            new_text: "new content".into(),
                        }],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await
            .unwrap();

        let EditFileToolOutput::Success {
            new_text,
            input_path,
            ..
        } = result
        else {
            panic!("expected success");
        };

        assert_eq!(new_text, "new content");
        assert_eq!(input_path, PathBuf::from("root/test.txt"));
    }

    #[gpui::test]
    async fn test_streaming_external_modification_mentioned_when_match_fails(
        cx: &mut TestAppContext,
    ) {
        let (edit_tool, project, action_log, fs, _thread) =
            setup_test(cx, json!({"test.txt": "original content"})).await;
        let read_tool = Arc::new(crate::ReadFileTool::new(
            project.clone(),
            action_log.clone(),
            true,
        ));

        cx.update(|cx| {
            read_tool.clone().run(
                ToolInput::resolved(crate::ReadFileToolInput {
                    path: "root/test.txt".to_string(),
                    start_line: None,
                    end_line: None,
                }),
                ToolCallEventStream::test().0,
                cx,
            )
        })
        .await
        .unwrap();

        cx.background_executor
            .advance_clock(std::time::Duration::from_secs(2));
        fs.save(
            path!("/root/test.txt").as_ref(),
            &"externally modified content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();

        let project_path = project
            .read_with(cx, |project, cx| {
                project.find_project_path("root/test.txt", cx)
            })
            .expect("Should find project path");
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
            .unwrap();
        buffer
            .update(cx, |buffer, cx| buffer.reload(cx))
            .await
            .unwrap();

        cx.executor().run_until_parked();

        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/test.txt".into(),
                        edits: vec![Edit {
                            old_text: "original content".into(),
                            new_text: "new content".into(),
                        }],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditFileToolOutput::Error {
            error,
            diff,
            input_path,
        } = result.unwrap_err()
        else {
            panic!("expected error");
        };

        assert!(
            error.contains("Could not find matching text for edit at index 0"),
            "Error should mention failed match, got: {error}"
        );
        assert!(
            error.contains("has changed on disk since you last read it"),
            "Error should mention possible disk change, got: {error}"
        );
        assert!(diff.is_empty());
        assert_eq!(input_path, Some(PathBuf::from("root/test.txt")));
    }

    #[gpui::test]
    async fn test_streaming_dirty_buffer_detected(cx: &mut TestAppContext) {
        let (edit_tool, project, action_log, _fs, _thread) =
            setup_test(cx, json!({"test.txt": "original content"})).await;
        let read_tool = Arc::new(crate::ReadFileTool::new(
            project.clone(),
            action_log.clone(),
            true,
        ));

        // Read the file first
        cx.update(|cx| {
            read_tool.clone().run(
                ToolInput::resolved(crate::ReadFileToolInput {
                    path: "root/test.txt".to_string(),
                    start_line: None,
                    end_line: None,
                }),
                ToolCallEventStream::test().0,
                cx,
            )
        })
        .await
        .unwrap();

        // Open the buffer and make it dirty
        let project_path = project
            .read_with(cx, |project, cx| {
                project.find_project_path("root/test.txt", cx)
            })
            .expect("Should find project path");
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
            .unwrap();

        buffer.update(cx, |buffer, cx| {
            let end_point = buffer.max_point();
            buffer.edit([(end_point..end_point, " added text")], None, cx);
        });

        let is_dirty = buffer.read_with(cx, |buffer, _| buffer.is_dirty());
        assert!(is_dirty, "Buffer should be dirty after in-memory edit");

        // Try to edit - should fail because buffer has unsaved changes
        let result = cx
            .update(|cx| {
                edit_tool.clone().run(
                    ToolInput::resolved(EditFileToolInput {
                        path: "root/test.txt".into(),
                        edits: vec![Edit {
                            old_text: "original content".into(),
                            new_text: "new content".into(),
                        }],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let EditFileToolOutput::Error {
            error,
            diff,
            input_path,
        } = result.unwrap_err()
        else {
            panic!("expected error");
        };
        assert!(
            error.contains("This file has unsaved changes."),
            "Error should mention unsaved changes, got: {}",
            error
        );
        assert!(
            error.contains("keep or discard"),
            "Error should ask whether to keep or discard changes, got: {}",
            error
        );
        assert!(
            error.contains("save or revert the file manually"),
            "Error should ask user to manually save or revert when tools aren't available, got: {}",
            error
        );
        assert!(diff.is_empty());
        assert!(input_path.is_none());
    }

    #[gpui::test]
    async fn test_streaming_overlapping_edits_resolved_sequentially(cx: &mut TestAppContext) {
        // Edit 1's replacement introduces text that contains edit 2's
        // old_text as a substring. Because edits resolve sequentially
        // against the current buffer, edit 2 finds a unique match in
        // the modified buffer and succeeds.
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "aaa\nbbb\nccc\nddd\neee\n"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        // Setup: resolve the buffer
        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // Edit 1 replaces "bbb\nccc" with "XXX\nccc\nddd", so the
        // buffer becomes "aaa\nXXX\nccc\nddd\nddd\neee\n".
        // Edit 2's old_text "ccc\nddd" matches the first occurrence
        // in the modified buffer and replaces it with "ZZZ".
        // Edit 3 exists only to mark edit 2 as "complete" during streaming.
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "bbb\nccc", "new_text": "XXX\nccc\nddd"},
                {"old_text": "ccc\nddd", "new_text": "ZZZ"},
                {"old_text": "eee", "new_text": "DUMMY"}
            ]
        }));
        cx.run_until_parked();

        // Send the final input with all three edits.
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [
                {"old_text": "bbb\nccc", "new_text": "XXX\nccc\nddd"},
                {"old_text": "ccc\nddd", "new_text": "ZZZ"},
                {"old_text": "eee", "new_text": "DUMMY"}
            ]
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "aaa\nXXX\nZZZ\nddd\nDUMMY\n");
    }

    #[gpui::test]
    async fn test_streaming_edit_json_fixer_escape_corruption(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello\nworld\nfoo\n"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        // Simulate JSON fixer producing a literal backslash when the LLM
        // stream cuts in the middle of a \n escape sequence.
        // The old_text "hello\nworld" would be streamed as:
        //   partial 1: old_text = "hello\\" (fixer closes incomplete \n as \\)
        //   partial 2: old_text = "hello\nworld" (fixer corrected the escape)
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "hello\\"}]
        }));
        cx.run_until_parked();

        // Now the fixer corrects it to the real newline.
        sender.send_partial(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "hello\nworld"}]
        }));
        cx.run_until_parked();

        // Send final.
        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": [{"old_text": "hello\nworld", "new_text": "HELLO\nWORLD"}]
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "HELLO\nWORLD\nfoo\n");
    }

    #[gpui::test]
    async fn test_streaming_final_input_stringified_edits_succeeds(cx: &mut TestAppContext) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello\nworld\n"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        sender.send_full(json!({
            "path": "root/file.txt",
            "edits": "[{\"old_text\": \"hello\\nworld\", \"new_text\": \"HELLO\\nWORLD\"}]"
        }));

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "HELLO\nWORLD\n");
    }

    // Verifies that after streaming_edit_file_tool edits a file, the action log
    // reports changed buffers so that the Accept All / Reject All review UI appears.
    #[gpui::test]
    async fn test_streaming_edit_file_tool_registers_changed_buffers(cx: &mut TestAppContext) {
        let (edit_tool, _project, action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (event_stream, _rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            edit_tool.clone().run(
                ToolInput::resolved(EditFileToolInput {
                    path: "root/file.txt".into(),
                    edits: vec![Edit {
                        old_text: "line 2".into(),
                        new_text: "modified line 2".into(),
                    }],
                }),
                event_stream,
                cx,
            )
        });

        let result = task.await;
        assert!(result.is_ok(), "edit should succeed: {:?}", result.err());

        cx.run_until_parked();

        let changed = action_log.read_with(cx, |log, cx| log.changed_buffers(cx));
        assert!(
            !changed.is_empty(),
            "action_log.changed_buffers() should be non-empty after streaming edit,
             but no changed buffers were found - Accept All / Reject All will not appear"
        );
    }

    // Same test but for Write mode (overwrite entire file).

    #[gpui::test]
    async fn test_streaming_edit_file_tool_fields_out_of_order_in_edit_mode(
        cx: &mut TestAppContext,
    ) {
        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "old_content"})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "edits": [{"old_text": "old_content"}]
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "edits": [{"old_text": "old_content", "new_text": "new_content"}]
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "edits": [{"old_text": "old_content", "new_text": "new_content"}],
            "path": "root"
        }));
        cx.run_until_parked();

        // Send final.
        sender.send_full(json!({
            "edits": [{"old_text": "old_content", "new_text": "new_content"}],
            "path": "root/file.txt"
        }));
        cx.run_until_parked();

        let result = task.await;
        let EditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new_content");
    }

    #[gpui::test]
    async fn test_streaming_edit_partial_last_line(cx: &mut TestAppContext) {
        let file_content = indoc::indoc! {r#"
            fn on_query_change(&mut self, cx: &mut Context<Self>) {
                self.filter(cx);
            }



            fn render_search(&self, cx: &mut Context<Self>) -> Div {
                div()
            }
        "#}
        .to_string();

        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.rs": file_content})).await;

        // The model sends old_text with a PARTIAL last line.
        let old_text = "}\n\n\n\nfn render_search";
        let new_text = "}\n\nfn render_search";

        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        sender.send_full(json!({
            "path": "root/file.rs",
            "edits": [{"old_text": old_text, "new_text": new_text}]
        }));

        let result = task.await;
        let EditFileToolOutput::Success {
            new_text: final_text,
            ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };

        // The edit should reduce 3 blank lines to 1 blank line before
        // fn render_search, without duplicating the function signature.
        let expected = file_content.replace("}\n\n\n\nfn render_search", "}\n\nfn render_search");
        pretty_assertions::assert_eq!(
            final_text,
            expected,
            "Edit should only remove blank lines before render_search"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_preserves_blank_line_after_trailing_newline_replacement(
        cx: &mut TestAppContext,
    ) {
        let file_content = "before\ntarget\n\nafter\n";
        let old_text = "target\n";
        let new_text = "one\ntwo\ntarget\n";
        let expected = "before\none\ntwo\ntarget\n\nafter\n";

        let (edit_tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.rs": file_content})).await;
        let (mut sender, input) = ToolInput::<EditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| edit_tool.clone().run(input, event_stream, cx));

        sender.send_full(json!({
            "path": "root/file.rs",
            "edits": [{"old_text": old_text, "new_text": new_text}]
        }));

        let result = task.await;

        let EditFileToolOutput::Success {
            new_text: final_text,
            ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };

        pretty_assertions::assert_eq!(
            final_text,
            expected,
            "Edit should preserve a single blank line before test_after"
        );
    }

    #[test]
    fn test_input_deserializes_double_encoded_fields() {
        let input = serde_json::from_value::<EditFileToolInput>(json!({
            "path": "root/file.txt",
            "edits": "[{\"old_text\": \"hello\\nworld\", \"new_text\": \"HELLO\\nWORLD\"}]"
        }))
        .expect("input should deserialize");

        assert_eq!(input.edits.len(), 1);
        assert_eq!(input.edits[0].old_text, "hello\nworld");
        assert_eq!(input.edits[0].new_text, "HELLO\nWORLD");

        let input = serde_json::from_value::<EditFileToolPartialInput>(json!({
            "path": "root/file.txt",
            "edits": "[{\"old_text\": \"hello\\nworld\", \"new_text\": \"HELLO\\nWORLD\"}]"
        }))
        .expect("input should deserialize");

        let edits = input.edits.expect("edits should deserialize");
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].old_text.as_deref(), Some("hello\nworld"));
        assert_eq!(edits[0].new_text.as_deref(), Some("HELLO\nWORLD"));

        let input = serde_json::from_value::<EditFileToolPartialInput>(json!({
            "path": "root/file.txt"
        }))
        .expect("input should deserialize");
        assert!(input.edits.is_none());

        let input = serde_json::from_value::<EditFileToolPartialInput>(json!({
            "path": "root/file.txt",
            "edits": null
        }))
        .expect("input should deserialize");
        assert!(input.edits.is_none());
    }

    async fn setup_test_with_fs(
        cx: &mut TestAppContext,
        fs: Arc<project::FakeFs>,
        worktree_paths: &[&std::path::Path],
    ) -> (
        Arc<EditFileTool>,
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
        let edit_tool = Arc::new(EditFileTool::new(
            project.clone(),
            thread.downgrade(),
            action_log.clone(),
            language_registry,
        ));
        (edit_tool, project, action_log, fs, thread)
    }

    async fn setup_test(
        cx: &mut TestAppContext,
        initial_tree: serde_json::Value,
    ) -> (
        Arc<EditFileTool>,
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

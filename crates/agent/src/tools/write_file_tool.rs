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

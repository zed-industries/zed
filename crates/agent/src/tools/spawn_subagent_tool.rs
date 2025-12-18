use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, AsyncApp, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Spawn a subagent (child thread) that can be visited while it runs, and returns a value to the parent.
///
/// Note: This file intentionally defines only the tool surface and streaming updates. The actual
/// spawning/navigation plumbing requires a host capability (session manager + UI) that is not yet
/// present in the native agent tool environment. Until that capability is wired in, this tool will
/// fail with a clear error.
///
/// Expected design (to be implemented in the host):
/// - The tool is constructed with a `SubagentHost` implementation that can:
///   - create a child session/thread
///   - stream child progress updates
///   - complete with a final return value
///   - provide a navigable URI for the UI (e.g. `zed://agent/thread/<session_id>`)
///
/// The tool then:
/// - emits a `ResourceLink` pointing at the child thread so users can open it
/// - streams progress into the tool call card as markdown
/// - resolves with the child's final return value (string)
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct SpawnSubagentToolInput {
    /// A short label/title for the subagent.
    pub title: String,

    /// The instructions to run in the subagent.
    pub prompt: String,

    /// Optional: profile id to use for the subagent.
    #[serde(default)]
    pub profile_id: Option<String>,
}

/// The final return value from the subagent.
pub type SpawnSubagentToolOutput = String;

/// Host interface required to implement spawning + streaming + returning.
///
/// This is intentionally minimal and object-safe to allow injecting a host backed by `NativeAgent`.
pub trait SubagentHost: Send + Sync + 'static {
    /// Start a child subagent session and return a handle containing a navigable URI plus a stream
    /// of progress updates and a final result.
    ///
    /// The returned `SubagentRun` must:
    /// - yield `Progress` updates in-order
    /// - eventually yield exactly one `Final` or `Error`
    fn spawn_subagent(
        &self,
        title: String,
        prompt: String,
        profile_id: Option<String>,
        cx: &mut AsyncApp,
    ) -> Task<Result<SubagentRun>>;
}

/// A handle for a running subagent.
pub struct SubagentRun {
    /// URI that the UI can open to navigate to the child thread.
    pub thread_uri: String,

    /// A human-friendly label for the link.
    pub thread_label: String,

    /// Progress stream for tool UI updates.
    pub updates: futures::channel::mpsc::UnboundedReceiver<SubagentUpdate>,
}

pub enum SubagentUpdate {
    /// A streaming progress chunk (e.g. "thinking…", partial summary, etc).
    Progress(String),

    /// The final return value for the parent.
    Final(String),

    /// Terminal error.
    Error(anyhow::Error),
}

pub struct SpawnSubagentTool {
    host: Option<Arc<dyn SubagentHost>>,
}

impl SpawnSubagentTool {
    pub fn new(host: Option<Arc<dyn SubagentHost>>) -> Self {
        Self { host }
    }
}

impl AgentTool for SpawnSubagentTool {
    type Input = SpawnSubagentToolInput;
    type Output = SpawnSubagentToolOutput;

    fn name() -> &'static str {
        "spawn_subagent"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn description() -> SharedString {
        "Spawns a child Zed Agent thread (subagent), streams its progress, and returns its final value to the parent."
            .into()
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Spawn subagent: {}", input.title).into()
        } else {
            "Spawn subagent".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let Some(host) = self.host.clone() else {
            return Task::ready(Err(anyhow!(
                "spawn_subagent is not available: native agent host capability is not wired into tools yet"
            )));
        };

        let title = input.title;
        let prompt = input.prompt;
        let profile_id = input.profile_id;

        cx.spawn(async move |cx| {
            // Start the child run via host.
            let mut run = host
                .spawn_subagent(title.clone(), prompt, profile_id, cx)
                .await?;

            // Emit a link to the child thread so the user can open/visit it.
            event_stream.update_fields(
                acp::ToolCallUpdateFields::new().content(vec![acp::ToolCallContent::Content(
                    acp::Content::new(acp::ContentBlock::ResourceLink(
                        acp::ResourceLink::new(run.thread_label.clone(), run.thread_uri.clone())
                            .title(run.thread_label.clone()),
                    )),
                )]),
            );

            // Stream progress as markdown appended below the link.
            let mut accumulated_progress = String::new();
            while let Some(update) = run.updates.next().await {
                match update {
                    SubagentUpdate::Progress(chunk) => {
                        if !accumulated_progress.is_empty() {
                            accumulated_progress.push('\n');
                        }
                        accumulated_progress.push_str(&chunk);

                        event_stream.update_fields(
                            acp::ToolCallUpdateFields::new().content(vec![
                                acp::ToolCallContent::Content(acp::Content::new(
                                    acp::ContentBlock::ResourceLink(
                                        acp::ResourceLink::new(
                                            run.thread_label.clone(),
                                            run.thread_uri.clone(),
                                        )
                                        .title(run.thread_label.clone()),
                                    ),
                                )),
                                acp::ToolCallContent::Content(acp::Content::new(
                                    acp::ContentBlock::Text(acp::TextContent::new(
                                        format!("### Subagent progress\n\n{}", accumulated_progress),
                                    )),
                                )),
                            ]),
                        );
                    }
                    SubagentUpdate::Final(value) => {
                        // Final update for UI (optional).
                        event_stream.update_fields(
                            acp::ToolCallUpdateFields::new().content(vec![
                                acp::ToolCallContent::Content(acp::Content::new(
                                    acp::ContentBlock::ResourceLink(
                                        acp::ResourceLink::new(
                                            run.thread_label.clone(),
                                            run.thread_uri.clone(),
                                        )
                                        .title(run.thread_label.clone()),
                                    ),
                                )),
                                acp::ToolCallContent::Content(acp::Content::new(
                                    acp::ContentBlock::Text(acp::TextContent::new(format!(
                                        "### Subagent returned\n\n{}",
                                        value
                                    ))),
                                )),
                            ]),
                        );

                        return Ok(value);
                    }
                    SubagentUpdate::Error(error) => {
                        return Err(error);
                    }
                }
            }

            Err(anyhow!("subagent stream ended without producing a final value"))
        })
    }
}

// futures::StreamExt is only needed in the async run implementation; keep it scoped here.
use futures::StreamExt as _;

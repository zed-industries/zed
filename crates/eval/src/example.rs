use std::{
    error::Error,
    fmt::{self, Debug},
    sync::{Arc, Mutex},
    time::Duration,
    u32,
};

use crate::{
    ToolMetrics,
    assertions::{AssertionsReport, RanAssertion, RanAssertionResult},
};
use acp_thread::UserMessageId;
use agent::{Thread, ThreadEvent, UserMessageContent};
use agent_client_protocol as acp;
use agent_settings::AgentProfileId;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use buffer_diff::DiffHunkStatus;
use collections::HashMap;
use futures::{FutureExt as _, StreamExt, select_biased};
use gpui::{App, AppContext, AsyncApp, Entity};
use language_model::Role;
use util::rel_path::RelPath;

pub const THREAD_EVENT_TIMEOUT: Duration = Duration::from_secs(60 * 2);

#[async_trait(?Send)]
pub trait Example {
    fn meta(&self) -> ExampleMetadata;
    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()>;
    fn diff_assertions(&self) -> Vec<JudgeAssertion> {
        Vec::new()
    }
    fn thread_assertions(&self) -> Vec<JudgeAssertion> {
        Vec::new()
    }
}

#[derive(Clone, Debug)]
pub struct JudgeAssertion {
    pub id: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct ExampleMetadata {
    pub name: String,
    pub url: String,
    pub revision: String,
    pub language_server: Option<LanguageServer>,
    pub max_assertions: Option<usize>,
    pub profile_id: AgentProfileId,
    pub existing_thread_json: Option<String>,
    pub max_turns: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct LanguageServer {
    pub file_extension: String,
    pub allow_preexisting_diagnostics: bool,
}

impl ExampleMetadata {
    pub fn repo_name(&self) -> String {
        self.url
            .split('/')
            .next_back()
            .unwrap_or("")
            .trim_end_matches(".git")
            .into()
    }
}

pub struct FailedAssertion(pub String);

impl fmt::Debug for FailedAssertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assertion failure: {}", self.0)
    }
}

impl fmt::Display for FailedAssertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for FailedAssertion {}

pub struct ExampleContext {
    meta: ExampleMetadata,
    log_prefix: String,
    agent_thread: Entity<agent::Thread>,
    app: AsyncApp,
    pub assertions: AssertionsReport,
    pub tool_metrics: Arc<Mutex<ToolMetrics>>,
}

impl ExampleContext {
    pub fn new(
        meta: ExampleMetadata,
        log_prefix: String,
        agent_thread: Entity<Thread>,
        app: AsyncApp,
    ) -> Self {
        let assertions = AssertionsReport::new(meta.max_assertions);

        Self {
            meta,
            log_prefix,
            agent_thread,
            assertions,
            app,
            tool_metrics: Arc::new(Mutex::new(ToolMetrics::default())),
        }
    }

    pub fn assert(&mut self, expected: bool, message: impl ToString) -> Result<()> {
        let message = message.to_string();
        self.log_assertion(
            if expected {
                Ok(())
            } else {
                Err(anyhow::Error::from(FailedAssertion(message.clone())))
            },
            message,
        )
    }

    pub fn assert_some<T>(&mut self, option: Option<T>, message: impl ToString) -> Result<T> {
        let message = message.to_string();
        self.log_assertion(
            match option {
                Some(value) => Ok(value),
                None => Err(anyhow::Error::from(FailedAssertion(message.clone()))),
            },
            message,
        )
    }

    #[allow(dead_code)]
    pub fn assert_eq<T: PartialEq + Debug>(
        &mut self,
        left: T,
        right: T,
        message: impl ToString,
    ) -> Result<()> {
        let message = message.to_string();
        self.log_assertion(
            if left == right {
                Ok(())
            } else {
                println!(
                    "{}{}",
                    self.log_prefix,
                    pretty_assertions::Comparison::new(&left, &right)
                );
                Err(anyhow::Error::from(FailedAssertion(message.clone())))
            },
            message,
        )
    }

    fn log_assertion<T>(&mut self, result: Result<T>, message: String) -> Result<T> {
        if let Some(max) = self.meta.max_assertions {
            anyhow::ensure!(
                self.assertions.run_count() <= max,
                "More assertions were run than the stated max_assertions of {max}"
            );
        }

        self.assertions.ran.push(RanAssertion {
            id: message.clone(),
            result: Ok(RanAssertionResult {
                analysis: None,
                passed: result.is_ok(),
            }),
        });

        if result.is_ok() {
            println!("{}✅ {}", self.log_prefix, message);
        } else {
            println!("{}❌ {}", self.log_prefix, message);
        }

        result
    }

    pub async fn prompt(&mut self, prompt: impl Into<String>) -> Result<Response> {
        self.prompt_with_max_turns(prompt, u32::MAX).await
    }

    pub async fn prompt_with_max_turns(
        &mut self,
        prompt: impl Into<String>,
        max_turns: u32,
    ) -> Result<Response> {
        let content = vec![UserMessageContent::Text(prompt.into())];
        self.run_turns(Some(content), max_turns).await
    }

    pub async fn proceed_with_max_turns(&mut self, max_turns: u32) -> Result<Response> {
        self.run_turns(None, max_turns).await
    }

    async fn run_turns(
        &mut self,
        prompt: Option<Vec<UserMessageContent>>,
        max_turns: u32,
    ) -> Result<Response> {
        let tool_metrics = self.tool_metrics.clone();
        let log_prefix = self.log_prefix.clone();

        let mut remaining_turns = max_turns;

        let mut event_stream = self.agent_thread.update(&mut self.app, |thread, cx| {
            if let Some(prompt) = prompt {
                let id = UserMessageId::new();
                thread.send(id, prompt, cx)
            } else {
                thread.proceed(cx)
            }
        })?;

        let task = self.app.background_spawn(async move {
            let mut messages = Vec::new();
            let mut tool_uses_by_id = HashMap::default();
            while let Some(event) = event_stream.next().await {
                match event? {
                    ThreadEvent::UserMessage(user_message) => {
                        messages.push(Message {
                            role: Role::User,
                            text: user_message.to_markdown(),
                            tool_use: Vec::new(),
                        });
                    }
                    ThreadEvent::AgentThinking(text) | ThreadEvent::AgentText(text) => {
                        if matches!(
                            messages.last(),
                            Some(Message {
                                role: Role::Assistant,
                                ..
                            })
                        ) {
                            messages.last_mut().unwrap().text.push_str(&text);
                        } else {
                            messages.push(Message {
                                role: Role::Assistant,
                                text,
                                tool_use: Vec::new(),
                            });
                        }
                    }
                    ThreadEvent::ToolCall(tool_call) => {
                        let meta = tool_call.meta.expect("Missing meta field in tool_call");
                        let tool_name = meta
                            .get("tool_name")
                            .expect("Missing tool_name field in meta")
                            .as_str()
                            .expect("Unknown tool_name content in meta");

                        tool_uses_by_id.insert(
                            tool_call.tool_call_id,
                            ToolUse {
                                name: tool_name.to_string(),
                                value: tool_call.raw_input.unwrap_or_default(),
                            },
                        );
                        if matches!(
                            tool_call.status,
                            acp::ToolCallStatus::Completed | acp::ToolCallStatus::Failed
                        ) {
                            panic!("Tool call completed without update");
                        }
                    }
                    ThreadEvent::ToolCallUpdate(tool_call_update) => {
                        if let acp_thread::ToolCallUpdate::UpdateFields(update) = tool_call_update {
                            if let Some(raw_input) = update.fields.raw_input {
                                if let Some(tool_use) =
                                    tool_uses_by_id.get_mut(&update.tool_call_id)
                                {
                                    tool_use.value = raw_input;
                                }
                            }

                            if matches!(
                                update.fields.status,
                                Some(acp::ToolCallStatus::Completed | acp::ToolCallStatus::Failed)
                            ) {
                                let succeeded =
                                    update.fields.status == Some(acp::ToolCallStatus::Completed);

                                let tool_use = tool_uses_by_id
                                    .remove(&update.tool_call_id)
                                    .expect("Unrecognized tool call completed");

                                let log_message = if succeeded {
                                    format!("✔︎ {}", tool_use.name)
                                } else {
                                    format!("✖︎ {}", tool_use.name)
                                };
                                println!("{log_prefix}{log_message}");

                                tool_metrics
                                    .lock()
                                    .unwrap()
                                    .insert(tool_use.name.clone().into(), succeeded);

                                if let Some(message) = messages.last_mut() {
                                    message.tool_use.push(tool_use);
                                } else {
                                    messages.push(Message {
                                        role: Role::Assistant,
                                        text: "".to_string(),
                                        tool_use: vec![tool_use],
                                    });
                                }

                                remaining_turns -= 1;
                                if remaining_turns == 0 {
                                    return Ok(messages);
                                }
                            }
                        }
                    }
                    ThreadEvent::ToolCallAuthorization(_) => panic!(
                        "{}Bug: Tool confirmation should not be required in eval",
                        log_prefix
                    ),
                    ThreadEvent::Retry(status) => {
                        println!("{log_prefix} Got retry: {status:?}");
                    }
                    ThreadEvent::Stop(stop_reason) => match stop_reason {
                        acp::StopReason::EndTurn => {}
                        acp::StopReason::MaxTokens => {
                            return Err(anyhow!("Exceeded maximum tokens"));
                        }
                        acp::StopReason::MaxTurnRequests => {
                            return Err(anyhow!("Exceeded maximum turn requests"));
                        }
                        stop_reason => return Err(anyhow!("{stop_reason:?}")),
                    },
                }
            }
            Ok(messages)
        });

        select_biased! {
            result = task.fuse() => {
                Ok(Response::new(result?))
            }
            _ = self.app.background_executor().timer(THREAD_EVENT_TIMEOUT).fuse() => {
                anyhow::bail!("Agentic loop stalled - waited {THREAD_EVENT_TIMEOUT:?} without any events");
            }
        }
    }

    pub fn edits(&self) -> HashMap<Arc<RelPath>, FileEdits> {
        self.agent_thread.read_with(&self.app, |thread, cx| {
            let action_log = thread.action_log().read(cx);
            HashMap::from_iter(
                action_log
                    .changed_buffers(cx)
                    .into_iter()
                    .map(|(buffer, diff)| {
                        let snapshot = buffer.read(cx).snapshot();

                        let file = snapshot.file().unwrap();
                        let base_text = diff.read(cx).base_text(cx).text();

                        let hunks = diff
                            .read(cx)
                            .snapshot(cx)
                            .hunks(&snapshot)
                            .map(|hunk| FileEditHunk {
                                base_text: base_text[hunk.diff_base_byte_range.clone()].to_string(),
                                text: snapshot
                                    .text_for_range(hunk.range.clone())
                                    .collect::<String>(),
                                status: hunk.status(),
                            })
                            .collect();

                        (file.path().clone(), FileEdits { hunks })
                    }),
            )
        })
    }

    pub fn agent_thread(&self) -> Entity<Thread> {
        self.agent_thread.clone()
    }
}

impl AppContext for ExampleContext {
    fn new<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut gpui::Context<T>) -> T,
    ) -> Entity<T> {
        self.app.new(build_entity)
    }

    fn reserve_entity<T: 'static>(&mut self) -> gpui::Reservation<T> {
        self.app.reserve_entity()
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: gpui::Reservation<T>,
        build_entity: impl FnOnce(&mut gpui::Context<T>) -> T,
    ) -> Entity<T> {
        self.app.insert_entity(reservation, build_entity)
    }

    fn update_entity<T, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut gpui::Context<T>) -> R,
    ) -> R
    where
        T: 'static,
    {
        self.app.update_entity(handle, update)
    }

    fn as_mut<'a, T>(&'a mut self, handle: &Entity<T>) -> gpui::GpuiBorrow<'a, T>
    where
        T: 'static,
    {
        self.app.as_mut(handle)
    }

    fn read_entity<T, R>(&self, handle: &Entity<T>, read: impl FnOnce(&T, &App) -> R) -> R
    where
        T: 'static,
    {
        self.app.read_entity(handle, read)
    }

    fn update_window<T, F>(&mut self, window: gpui::AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(gpui::AnyView, &mut gpui::Window, &mut App) -> T,
    {
        self.app.update_window(window, f)
    }

    fn read_window<T, R>(
        &self,
        window: &gpui::WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        self.app.read_window(window, read)
    }

    fn background_spawn<R>(
        &self,
        future: impl std::future::Future<Output = R> + Send + 'static,
    ) -> gpui::Task<R>
    where
        R: Send + 'static,
    {
        self.app.background_spawn(future)
    }

    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> R
    where
        G: gpui::Global,
    {
        self.app.read_global(callback)
    }
}

#[derive(Debug)]
pub struct Response {
    messages: Vec<Message>,
}

impl Response {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    pub fn expect_tool_call(
        &self,
        tool_name: &'static str,
        cx: &mut ExampleContext,
    ) -> Result<&ToolUse> {
        let result = self.find_tool_call(tool_name);
        cx.assert_some(result, format!("called `{}`", tool_name))
    }

    pub fn find_tool_call(&self, tool_name: &str) -> Option<&ToolUse> {
        self.messages.iter().rev().find_map(|msg| {
            msg.tool_use
                .iter()
                .find(|tool_use| tool_use.name == tool_name)
        })
    }

    pub fn tool_calls(&self) -> impl Iterator<Item = &ToolUse> {
        self.messages.iter().flat_map(|msg| &msg.tool_use)
    }

    pub fn texts(&self) -> impl Iterator<Item = String> {
        self.messages.iter().map(|message| message.text.clone())
    }
}

#[derive(Debug)]
pub struct Message {
    role: Role,
    text: String,
    tool_use: Vec<ToolUse>,
}

#[derive(Debug)]
pub struct ToolUse {
    pub name: String,
    value: serde_json::Value,
}

impl ToolUse {
    pub fn parse_input<Input>(&self) -> Result<Input>
    where
        Input: for<'de> serde::Deserialize<'de>,
    {
        serde_json::from_value::<Input>(self.value.clone()).map_err(|err| anyhow!(err))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FileEdits {
    pub hunks: Vec<FileEditHunk>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FileEditHunk {
    pub base_text: String,
    pub text: String,
    pub status: DiffHunkStatus,
}

impl FileEdits {
    pub fn has_added_line(&self, line: &str) -> bool {
        self.hunks.iter().any(|hunk| {
            hunk.status == DiffHunkStatus::added_none()
                && hunk.base_text.is_empty()
                && hunk.text.contains(line)
        })
    }
}

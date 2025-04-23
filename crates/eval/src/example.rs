use std::{
    error::Error,
    fmt::{self, Debug},
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    ToolMetrics,
    assertions::{Assertion, AssertionsReport, RanAssertionResult},
};
use agent::ThreadEvent;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use buffer_diff::DiffHunkStatus;
use collections::HashMap;
use futures::{FutureExt as _, StreamExt, channel::mpsc, select_biased};
use gpui::{AppContext, AsyncApp, Entity};
use language_model::{LanguageModel, Role, StopReason};
use serde::{Deserialize, Serialize};

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
    pub group_id: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct ExampleMetadata {
    pub name: String,
    pub url: String,
    pub revision: String,
    pub language_server: Option<LanguageServer>,
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
            .unwrap_or(&"")
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
    model: Arc<dyn LanguageModel>,
    pub assertions: AssertionsReport,
    pub tool_metrics: Arc<Mutex<ToolMetrics>>,
}

impl ExampleContext {
    pub fn new(
        meta: ExampleMetadata,
        log_prefix: String,
        agent_thread: Entity<agent::Thread>,
        model: Arc<dyn LanguageModel>,
        app: AsyncApp,
    ) -> Self {
        let assertions = AssertionsReport::default();

        Self {
            meta,
            log_prefix,
            agent_thread,
            assertions,
            model,
            app,
            tool_metrics: Arc::new(Mutex::new(ToolMetrics::default())),
        }
    }

    pub fn push_user_message(&mut self, text: impl ToString) {
        self.app
            .update_entity(&self.agent_thread, |thread, cx| {
                thread.insert_user_message(text.to_string(), vec![], None, cx);
            })
            .unwrap();
    }

    pub fn assertion(&mut self, key: impl Into<String>) -> AssertionGroupId {
        let group_id = AssertionGroupId(key.into());
        self.assertions.groups.insert(group_id.clone());
        group_id
    }

    fn log_assertion<T>(
        &mut self,
        group_id: AssertionGroupId,
        result: Result<T>,
        message: String,
    ) -> Result<T> {
        self.assertions.ran.push(Assertion {
            group_id,
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

    pub async fn run_to_end(&mut self) -> Result<Response> {
        self.run_turns(u32::MAX).await
    }

    pub async fn run_turn(&mut self) -> Result<Response> {
        self.run_turns(1).await
    }

    pub async fn run_turns(&mut self, iterations: u32) -> Result<Response> {
        let (mut tx, mut rx) = mpsc::channel(1);

        let tool_metrics = self.tool_metrics.clone();
        let log_prefix = self.log_prefix.clone();
        let _subscription = self.app.subscribe(
            &self.agent_thread,
            move |thread, event: &ThreadEvent, cx| match event {
                ThreadEvent::ShowError(thread_error) => {
                    tx.try_send(Err(anyhow!(thread_error.clone()))).ok();
                }
                ThreadEvent::Stopped(reason) => match reason {
                    Ok(StopReason::EndTurn) => {
                        tx.close_channel();
                    }
                    Ok(StopReason::ToolUse) => {
                        if thread.read(cx).remaining_turns() == 0 {
                            tx.close_channel();
                        }
                    }
                    Ok(StopReason::MaxTokens) => {
                        tx.try_send(Err(anyhow!("Exceeded maximum tokens"))).ok();
                    }
                    Err(err) => {
                        tx.try_send(Err(anyhow!(err.clone()))).ok();
                    }
                },
                ThreadEvent::StreamedAssistantText(_, _)
                | ThreadEvent::StreamedAssistantThinking(_, _)
                | ThreadEvent::UsePendingTools { .. } => {}
                ThreadEvent::ToolFinished {
                    tool_use_id,
                    pending_tool_use,
                    ..
                } => {
                    thread.update(cx, |thread, _cx| {
                        if let Some(tool_use) = pending_tool_use {
                            let mut tool_metrics = tool_metrics.lock().unwrap();
                            if let Some(tool_result) = thread.tool_result(&tool_use_id) {
                                let message = if tool_result.is_error {
                                    format!("✖︎ {}", tool_use.name)
                                } else {
                                    format!("✔︎ {}", tool_use.name)
                                };
                                println!("{log_prefix}{message}");
                                tool_metrics
                                    .insert(tool_result.tool_name.clone(), !tool_result.is_error);
                            } else {
                                let message =
                                    format!("TOOL FINISHED WITHOUT RESULT: {}", tool_use.name);
                                println!("{log_prefix}{message}");
                                tool_metrics.insert(tool_use.name.clone(), true);
                            }
                        }
                    });
                }
                ThreadEvent::ToolConfirmationNeeded => {
                    panic!(
                        "{}Bug: Tool confirmation should not be required in eval",
                        log_prefix
                    );
                }
                ThreadEvent::StreamedCompletion
                | ThreadEvent::MessageAdded(_)
                | ThreadEvent::MessageEdited(_)
                | ThreadEvent::MessageDeleted(_)
                | ThreadEvent::SummaryChanged
                | ThreadEvent::SummaryGenerated
                | ThreadEvent::ReceivedTextChunk
                | ThreadEvent::StreamedToolUse { .. }
                | ThreadEvent::CheckpointChanged
                | ThreadEvent::UsageUpdated(_) => {
                    tx.try_send(Ok(())).ok();
                    if std::env::var("ZED_EVAL_DEBUG").is_ok() {
                        println!("{}Event: {:#?}", log_prefix, event);
                    }
                }
            },
        );

        let model = self.model.clone();

        let message_count_before = self.app.update_entity(&self.agent_thread, |thread, cx| {
            thread.set_remaining_turns(iterations);
            thread.send_to_model(model, None, cx);
            thread.messages().len()
        })?;

        loop {
            select_biased! {
                result = rx.next() => {
                    if let Some(result) = result {
                        result?;
                    } else {
                        break;
                    }
                }
                _ = self.app.background_executor().timer(THREAD_EVENT_TIMEOUT).fuse() => {
                    return Err(anyhow!("Agentic loop stalled - waited {:?} without any events", THREAD_EVENT_TIMEOUT));
                }
            }
        }

        let messages = self.app.read_entity(&self.agent_thread, |thread, cx| {
            let mut messages = Vec::new();
            for message in thread.messages().skip(message_count_before) {
                messages.push(Message {
                    _role: message.role,
                    _text: message.to_string(),
                    tool_use: thread
                        .tool_uses_for_message(message.id, cx)
                        .into_iter()
                        .map(|tool_use| ToolUse {
                            name: tool_use.name.to_string(),
                            value: tool_use.input,
                        })
                        .collect(),
                });
            }
            messages
        })?;

        let response = Response::new(messages);

        Ok(response)
    }

    pub fn edits(&self) -> HashMap<Arc<Path>, FileEdits> {
        self.app
            .read_entity(&self.agent_thread, |thread, cx| {
                let action_log = thread.action_log().read(cx);
                HashMap::from_iter(action_log.changed_buffers(cx).into_iter().map(
                    |(buffer, diff)| {
                        let snapshot = buffer.read(cx).snapshot();

                        let file = snapshot.file().unwrap();
                        let diff = diff.read(cx);
                        let base_text = diff.base_text().text();

                        let hunks = diff
                            .hunks(&snapshot, cx)
                            .map(|hunk| FileEditHunk {
                                base_text: base_text[hunk.diff_base_byte_range.clone()].to_string(),
                                text: snapshot
                                    .text_for_range(hunk.range.clone())
                                    .collect::<String>(),
                                status: hunk.status(),
                            })
                            .collect();

                        (file.path().clone(), FileEdits { hunks })
                    },
                ))
            })
            .unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct AssertionGroupId(pub String);

impl AssertionGroupId {
    pub fn assert(
        &self,
        expected: bool,
        message: impl ToString,
        cx: &mut ExampleContext,
    ) -> Result<()> {
        let message = message.to_string();
        cx.log_assertion(
            self.clone(),
            if expected {
                Ok(())
            } else {
                Err(anyhow::Error::from(FailedAssertion(message.clone())))
            },
            message,
        )
    }

    pub fn assert_some<T>(
        &self,
        option: Option<T>,
        message: impl ToString,
        cx: &mut ExampleContext,
    ) -> Result<T> {
        let message = message.to_string();
        cx.log_assertion(
            self.clone(),
            match option {
                Some(value) => Ok(value),
                None => Err(anyhow::Error::from(FailedAssertion(message.clone()))),
            },
            message,
        )
    }

    #[allow(dead_code)]
    pub fn assert_eq<T: PartialEq + Debug>(
        &self,
        left: T,
        right: T,
        message: impl ToString,
        cx: &mut ExampleContext,
    ) -> Result<()> {
        let message = message.to_string();
        cx.log_assertion(
            self.clone(),
            if left == right {
                Ok(())
            } else {
                println!("{}{:#?} != {:#?}", cx.log_prefix, left, right);
                Err(anyhow::Error::from(FailedAssertion(message.clone())))
            },
            message,
        )
    }
}

impl fmt::Display for AssertionGroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
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

    // todo! move this to assertion group id method maybe?
    pub fn expect_tool(
        &self,
        group_id: AssertionGroupId,
        tool_name: &'static str,
        cx: &mut ExampleContext,
    ) -> Result<&ToolUse> {
        let result = self.messages.iter().find_map(|msg| {
            msg.tool_use
                .iter()
                .find(|tool_use| tool_use.name == tool_name)
        });
        group_id.assert_some(result, format!("called `{}`", tool_name), cx)
    }

    pub fn tool_uses(&self) -> impl Iterator<Item = &ToolUse> {
        self.messages.iter().flat_map(|msg| &msg.tool_use)
    }
}

#[derive(Debug)]
pub struct Message {
    _role: Role,
    _text: String,
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

#[derive(Debug)]
pub struct FileEdits {
    hunks: Vec<FileEditHunk>,
}

#[derive(Debug)]
struct FileEditHunk {
    base_text: String,
    text: String,
    status: DiffHunkStatus,
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

use std::{
    error::Error,
    fmt::{self, Debug},
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    ToolMetrics,
    assertions::{AssertionsReport, RanAssertion, RanAssertionResult},
};
use agent::ThreadEvent;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use futures::{FutureExt as _, StreamExt, channel::mpsc, select_biased};
use gpui::{AppContext, AsyncApp, Entity};
use language_model::{LanguageModel, Role, StopReason};

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
        let assertions = AssertionsReport::new(meta.max_assertions);

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
                println!("{}{:#?} != {:#?}", self.log_prefix, left, right);
                Err(anyhow::Error::from(FailedAssertion(message.clone())))
            },
            message,
        )
    }

    fn log_assertion<T>(&mut self, result: Result<T>, message: String) -> Result<T> {
        if let Some(max) = self.meta.max_assertions {
            if self.assertions.run_count() > max {
                return Err(anyhow!(
                    "More assertions were run than the stated max_assertions of {}",
                    max
                ));
            }
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
                                    format!("TOOL FAILED: {}", tool_use.name)
                                } else {
                                    format!("TOOL FINISHED: {}", tool_use.name)
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
            thread.send_to_model(model, cx);
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
}

#[derive(Debug)]
pub struct Response {
    messages: Vec<Message>,
}

impl Response {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    pub fn expect_tool(
        &self,
        tool_name: &'static str,
        cx: &mut ExampleContext,
    ) -> Result<&ToolUse> {
        let result = self.messages.iter().find_map(|msg| {
            msg.tool_use
                .iter()
                .find(|tool_use| tool_use.name == tool_name)
        });
        cx.assert_some(result, format!("called `{}`", tool_name))
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
    name: String,
    value: serde_json::Value,
}

impl ToolUse {
    pub fn expect_input<Input>(&self, cx: &mut ExampleContext) -> Result<Input>
    where
        Input: for<'de> serde::Deserialize<'de>,
    {
        let result =
            serde_json::from_value::<Input>(self.value.clone()).map_err(|err| anyhow!(err));
        cx.log_assertion(result, format!("valid `{}` input", &self.name))
    }
}

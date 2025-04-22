use std::{
    error::Error,
    fmt::{self, Debug},
    sync::Arc,
};

use crate::assertions::Assertions;
use agent::ThreadEvent;
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use futures::{StreamExt, channel::mpsc};
use gpui::{AppContext, AsyncApp, Entity};
use language_model::{LanguageModel, Role, StopReason};

#[async_trait(?Send)]
pub trait EvalThread {
    fn meta(&self) -> EvalThreadMetadata;
    async fn conversation(&self, cx: &mut ThreadContext) -> Result<()>;
    fn diff_criteria(&self) -> String {
        "".to_string()
    }
    fn thread_criteria(&self) -> String {
        "".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct EvalThreadMetadata {
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

impl EvalThreadMetadata {
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

pub struct ThreadContext {
    meta: EvalThreadMetadata,
    log_prefix: String,
    agent_thread: Entity<agent::Thread>,
    assertions: Assertions,
    app: AsyncApp,
    model: Arc<dyn LanguageModel>,
}

impl ThreadContext {
    pub fn new(
        meta: EvalThreadMetadata,
        log_prefix: String,
        agent_thread: Entity<agent::Thread>,
        model: Arc<dyn LanguageModel>,
        app: AsyncApp,
    ) -> Self {
        let assertions = Assertions::new(meta.max_assertions);

        Self {
            meta,
            log_prefix,
            agent_thread,
            assertions,
            model,
            app,
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
        self.assertion_result(
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
        self.assertion_result(
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
        self.assertion_result(
            if left == right {
                Ok(())
            } else {
                println!("{}{:#?} != {:#?}", self.log_prefix, left, right);
                Err(anyhow::Error::from(FailedAssertion(message.clone())))
            },
            message,
        )
    }

    fn assertion_result<T>(&mut self, result: Result<T>, message: String) -> Result<T> {
        if let Some(max) = self.meta.max_assertions {
            let run = self.assertions.failure.len() + self.assertions.success.len();

            if run > max {
                return Err(anyhow!(
                    "More assertions were run than the stated max_assertions of {}",
                    max
                ));
            }
        }

        match result {
            Ok(value) => {
                self.assertions.success.push(message.clone());
                println!("{}✅: {}", self.log_prefix, message);
                Ok(value)
            }
            Err(err) => {
                self.assertions.failure.push(message);
                println!("{}❌: {}", self.log_prefix, err);
                Err(err)
            }
        }
    }

    pub async fn run_to_end(&mut self) -> Result<Response> {
        self.run_turns(u32::MAX).await
    }

    pub async fn run_turn(&mut self) -> Result<Response> {
        self.run_turns(1).await
    }

    pub async fn run_turns(&mut self, iterations: u32) -> Result<Response> {
        let (mut tx, mut rx) = mpsc::channel(1);

        let _subscription = self.app.subscribe(
            &self.agent_thread,
            move |thread, event: &ThreadEvent, cx| match event {
                ThreadEvent::Stopped(Ok(StopReason::EndTurn)) => {
                    tx.try_send(Ok(())).ok();
                }
                ThreadEvent::Stopped(Ok(StopReason::ToolUse)) => {
                    if thread.read(cx).remaining_turns() == 0 {
                        tx.try_send(Ok(())).ok();
                    }
                }
                ThreadEvent::Stopped(Ok(StopReason::MaxTokens)) => {
                    tx.try_send(Err(anyhow!("Exceeded maximum tokens"))).ok();
                }
                ThreadEvent::ShowError(thread_error) => {
                    tx.try_send(Err(anyhow!(thread_error.clone()))).ok();
                }
                _ => {}
            },
        );

        let model = self.model.clone();

        let message_count_before = self.app.update_entity(&self.agent_thread, |thread, cx| {
            thread.set_remaining_turns(iterations);
            thread.send_to_model(model, cx);
            thread.messages().len()
        })?;

        rx.next().await.context("Failed to read from channel.")??;

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

    pub fn report(self) -> Assertions {
        self.assertions
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

    pub fn expect_tool(&self, tool_name: &'static str, cx: &mut ThreadContext) -> Result<&ToolUse> {
        let result = self.messages.iter().find_map(|msg| {
            msg.tool_use.iter().find_map(|tool_use| {
                if tool_use.name == tool_name {
                    Some(tool_use)
                } else {
                    None
                }
            })
        });
        cx.assert_some(result, format!("has `{}` tool calls", tool_name))
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
    pub fn expect_input<Input>(&self, cx: &mut ThreadContext) -> Result<Input>
    where
        Input: for<'de> serde::Deserialize<'de>,
    {
        let result =
            serde_json::from_value::<Input>(self.value.clone()).map_err(|err| anyhow!(err));
        cx.assertion_result(result, format!("{} input", &self.name))
    }
}

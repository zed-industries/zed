use std::{error::Error, fmt, sync::Arc};

use agent::ThreadEvent;
use anyhow::{Result, anyhow};
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
    pub max_assertions: Option<u32>,
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
        write!(f, "Assertion failed: {}", self.0)
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
    agent_thread: Entity<agent::Thread>,
    successful_assertions: u32,
    assertions_run: u32,
    app: AsyncApp,
    model: Arc<dyn LanguageModel>,
}

impl ThreadContext {
    pub fn new(
        meta: EvalThreadMetadata,
        agent_thread: Entity<agent::Thread>,
        model: Arc<dyn LanguageModel>,
        app: AsyncApp,
    ) -> Self {
        Self {
            meta,
            agent_thread,
            successful_assertions: 0,
            assertions_run: 0,
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
        self.assertion_result(if expected {
            Ok(())
        } else {
            Err(anyhow::Error::from(FailedAssertion(message.to_string())))
        })
    }

    pub fn assert_some<T>(&mut self, option: Option<T>, message: impl ToString) -> Result<T> {
        self.assertion_result(match option {
            Some(value) => Ok(value),
            None => Err(anyhow::Error::from(FailedAssertion(message.to_string()))),
        })
    }

    fn assertion_result<T>(&mut self, result: Result<T>) -> Result<T> {
        self.assertions_run += 1;

        if let Some(max) = self.meta.max_assertions {
            if self.assertions_run > max {
                return Err(anyhow!(
                    "More assertions were run than the stated max_assertions of {}",
                    max
                ));
            }
        }

        match result {
            Ok(value) => {
                self.successful_assertions += 1;
                Ok(value)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn run_to_end(&mut self) -> Result<Response> {
        let (mut tx, mut rx) = mpsc::channel(1);

        let _subscription = self.app.subscribe(
            &self.agent_thread,
            move |_thread, event: &ThreadEvent, _cx| match event {
                ThreadEvent::Stopped(Ok(StopReason::EndTurn)) => {
                    tx.try_send(()).ok();
                }
                _ => {}
            },
        );

        let model = self.model.clone();

        let message_count_before = self.app.update_entity(&self.agent_thread, |thread, cx| {
            thread.send_to_model(model, cx);
            thread.messages().len()
        })?;

        rx.next().await;

        let messages = self.app.read_entity(&self.agent_thread, |thread, cx| {
            let mut messages = Vec::new();
            for message in thread.messages().skip(message_count_before) {
                messages.push(Message {
                    role: message.role,
                    text: message.to_string(),
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

    async fn run_turn(&self, response: &mut Response) -> Result<Response> {
        todo!()
    }

    pub async fn run_turns(&self, iterations: usize) -> Result<Response> {
        let mut response = Response::new(vec![]);

        for _ in 0..iterations {
            self.run_turn(&mut response).await?;
        }

        Ok(response)
    }

    pub fn report(&self) -> String {
        format!("{}/{}", self.successful_assertions, self.assertions_run)
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
        cx.assert_some(result, format!("No tool calls for {}", tool_name))
    }

    fn extend(&mut self, other: Response) {
        self.messages.extend(other.messages);
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
        cx.assertion_result(result)
    }
}

use agent::{ThreadEvent, ThreadStore};
use anyhow::{Context, Result, anyhow};
use assistant_tool::ToolWorkingSet;
use async_trait::async_trait;
use futures::channel::mpsc;
use futures::{FutureExt as _, StreamExt as _, select_biased};
use gpui::{App, AppContext as _, Task};
use language_model::{LanguageModel, StopReason};
use project::{Project, ProjectPath};
use std::cell::RefCell;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{error::Error, path::PathBuf};
use unindent::Unindent;
use util::ResultExt as _;

use crate::{
    AgentAppState, RequestMarkdown, RunOutput, THREAD_EVENT_TIMEOUT, ToolMetrics, WORKTREES_DIR,
    ZED_REPO_URL, query_lsp_diagnostics, repo_path_for_url, response_events_to_markdown, run_git,
    wait_for_lang_server,
};

#[async_trait]
pub trait EvalThread {
    fn meta(&self) -> EvalThreadMetadata;
    async fn run(&self, cx: &mut ThreadContext) -> Result<()>;
}

pub struct EvalThreadMetadata {
    pub name: &'static str,
    pub url: &'static str,
    pub revision: &'static str,
    pub language_server: Option<LanguageServer>,
    pub max_assertions: u32,
}

pub struct LanguageServer {
    pub file_extension: &'static str,
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
    messages: Vec<Message>,
    successful_assertions: u32,
    assertions_run: u32,
}

impl ThreadContext {
    pub fn push_user_message(&mut self, text: impl ToString) {
        self.messages.push(Message::user(text))
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

        let max = self.meta.max_assertions;
        if self.assertions_run > max {
            return Err(anyhow!(
                "More assertions were run than the stated max_assertions of {}",
                max
            ));
        }

        match result {
            Ok(value) => {
                self.successful_assertions += 1;
                Ok(value)
            }
            Err(err) => Err(err),
        }
    }

    async fn run_turn(&self, response: &mut Response) -> Result<Response> {
        todo!()
    }

    pub async fn run_turns(&self, iterations: usize) -> Result<Response> {
        let mut response = Response::new();

        for _ in 0..iterations {
            self.run_turn(&mut response).await?;
        }

        Ok(response)
    }
}

pub struct Response {
    messages: Vec<Message>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn expect_tool(&self, tool_name: &'static str, cx: &mut ThreadContext) -> Result<&ToolUse> {
        let result = self.messages.iter().find_map(|msg| {
            msg.tool_use.as_ref().and_then(|tool_use| {
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

pub struct Message {
    role: Role,
    text: String,
    tool_use: Option<ToolUse>,
}

impl Message {
    pub fn expect_tool(&self) -> &ToolUse {
        self.tool_use
            .as_ref()
            .expect("Message was expected to have a tool_use, but it had none.")
    }

    pub fn user(text: impl ToString) -> Self {
        Self {
            role: Role::User,
            text: text.to_string().unindent(),
            tool_use: None,
        }
    }
}

pub enum Role {
    Assistant,
    User,
}

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

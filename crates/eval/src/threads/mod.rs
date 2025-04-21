pub mod file_search;

use anyhow::{Result, anyhow};
use std::error::Error;
use std::fmt;

pub enum Threads {
    FileSearch(file_search::Thread),
}

pub trait EvalThread {
    fn meta() -> EvalThreadMetadata;
    async fn run(cx: &mut ThreadContext) -> Result<()>;
}

pub struct EvalThreadMetadata {
    name: &'static str,
    url: &'static str,
    revision: &'static str,
    lang: &'static str,
    max_assertions: usize,
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

struct ThreadContext {
    messages: Vec<Message>,
    successful_assertions: u32,
    assertions_run: u32,
    max_assertions: u32,
}

impl ThreadContext {
    pub fn new(max_assertions: u32) -> Self {
        Self {
            messages: Vec::new(),
            successful_assertions: 0,
            assertions_run: 0,
            max_assertions,
        }
    }

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

        if self.assertions_run > self.max_assertions {
            return Err(anyhow!(
                "More assertions were run than the stated max_assertions of {}",
                self.max_assertions
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

struct Response {
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

struct Message {
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
            text: deindent(text.to_string()),
            tool_use: None,
        }
    }
}

enum Role {
    Assistant,
    User,
}

struct ToolUse {
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

struct Eval {}

struct EvalOutput {
    response: Vec<Message>,
    diff: String,
}

fn deindent(text: String) -> String {
    // Count the number of spaces in the first line of the text
    let leading_spaces = text
        .lines()
        .next()
        .map_or(0, |line| line.chars().take_while(|&c| c == ' ').count());

    // Trim the number of leading spaces from each line, preserving relative indentation
    text.lines()
        .map(|line| {
            let line_spaces = line.chars().take_while(|&c| c == ' ').count();
            let spaces_to_trim = std::cmp::min(leading_spaces, line_spaces);
            &line[spaces_to_trim..]
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deindent_simple() {
        let input = r#"    This is indented
    This is also indented
        This has extra indentation
    Back to normal"#;

        let expected = r#"This is indented
This is also indented
    This has extra indentation
Back to normal"#;

        assert_eq!(deindent(input.to_string()), expected);
    }

    #[test]
    fn test_deindent_mixed() {
        let input = r#"    First line indented
Second line not indented
    Third line indented again
        Fourth line with more indentation"#;

        let expected = r#"First line indented
Second line not indented
Third line indented again
    Fourth line with more indentation"#;

        assert_eq!(deindent(input.to_string()), expected);
    }
}

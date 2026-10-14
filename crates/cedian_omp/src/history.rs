//! A session's history as the session manager shows it: one line per OMP
//! message, and OMP's last assistant text.

use omp_rpc::wire::{AgentMessage, AssistantContent, MessageContent, UserContent};
use serde_json::Value;

/// `get_last_assistant_text`, decoded the way OMP 18.8.6 answers it: with
/// no assistant message OMP sends `{}`, which the generated decoder refuses
/// (it requires `text`).
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct LastAssistantTextCommand {}

impl omp_rpc::Command for LastAssistantTextCommand {
    const NAME: &'static str = "get_last_assistant_text";
    const TIMEOUT_MS: Option<u64> = None;
    type Output = Option<String>;

    fn decode(data: Option<Value>) -> Result<Self::Output, serde_json::Error> {
        Ok(data
            .as_ref()
            .and_then(|d| d.get("text"))
            .and_then(Value::as_str)
            .map(str::to_string))
    }
}

/// One line for `message`: who said it, and its text on one line.
pub fn message_line(message: &AgentMessage) -> String {
    let (who, text) = match message {
        AgentMessage::User(m) => ("you", content(m.content.as_ref())),
        AgentMessage::Developer(m) => ("developer", content(m.content.as_ref())),
        AgentMessage::Assistant(m) => (
            "OMP",
            m.content
                .iter()
                .flatten()
                .filter_map(|c| match c {
                    AssistantContent::Text(t) => t.text.clone(),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" "),
        ),
        AgentMessage::ToolResult(m) => (
            "tool",
            m.tool_name.clone().unwrap_or_else(|| "result".to_string()),
        ),
        AgentMessage::BashExecution(m) => (
            "bash",
            format!(
                "$ {} → {}",
                m.command.as_deref().unwrap_or(""),
                m.output.as_deref().unwrap_or("").trim()
            ),
        ),
        AgentMessage::PythonExecution(m) => ("python", m.code.clone().unwrap_or_default()),
        AgentMessage::Custom(m) => (
            m.custom_type.as_deref().unwrap_or("note"),
            content(m.content.as_ref()),
        ),
        AgentMessage::HookMessage(m) => (
            m.custom_type.as_deref().unwrap_or("hook"),
            content(m.content.as_ref()),
        ),
        AgentMessage::BranchSummary(m) => ("branch summary", m.summary.clone().unwrap_or_default()),
        AgentMessage::CompactionSummary(m) => {
            let who = match m.method.as_deref() {
                Some("handoff") => "handoff summary",
                _ => "summary",
            };
            (who, m.summary.clone().unwrap_or_default())
        }
        AgentMessage::FileMention(m) => (
            "files",
            m.files
                .iter()
                .flatten()
                .filter_map(|f| f.path.clone())
                .collect::<Vec<_>>()
                .join(", "),
        ),
    };
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    format!("{who}: {text}")
}

fn content(content: Option<&MessageContent>) -> String {
    match content {
        Some(MessageContent::String(text)) => text.clone(),
        Some(MessageContent::UserContentList(list)) => list
            .iter()
            .filter_map(|c| match c {
                UserContent::Text(t) => t.text.clone(),
                UserContent::Image(_) => Some("[image]".to_string()),
            })
            .collect::<Vec<_>>()
            .join(" "),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omp_rpc::Command;
    use serde_json::json;

    #[test]
    fn the_pins_empty_answer_is_no_text() {
        assert!(
            omp_rpc::wire::GetLastAssistantTextCommand::decode(Some(json!({}))).is_err(),
            "the generated decoder refuses OMP's empty answer"
        );
        assert_eq!(
            LastAssistantTextCommand::decode(Some(json!({}))).unwrap(),
            None
        );
        assert_eq!(
            LastAssistantTextCommand::decode(Some(json!({"text": "done"}))).unwrap(),
            Some("done".to_string())
        );
    }

    #[test]
    fn a_bash_message_is_its_command_and_output() {
        let message = AgentMessage::from_value(json!({
            "role": "bashExecution", "command": "echo a-1", "output": "a-1\n",
            "exitCode": 0, "cancelled": false, "truncated": false, "timestamp": 1
        }))
        .unwrap();
        assert_eq!(message_line(&message), "bash: $ echo a-1 → a-1");
    }
}

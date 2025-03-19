use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use futures::future::Shared;
use futures::FutureExt as _;
use gpui::{SharedString, Task};
use language_model::{
    LanguageModelRequestMessage, LanguageModelToolResult, LanguageModelToolUse,
    LanguageModelToolUseId, MessageContent, Role,
};

use crate::thread::MessageId;
use crate::thread_store::SerializedMessage;

#[derive(Debug)]
pub struct ToolUse {
    pub id: LanguageModelToolUseId,
    pub name: SharedString,
    pub status: ToolUseStatus,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum ToolUseStatus {
    Pending,
    Running,
    Finished(SharedString),
    Error(SharedString),
}

pub struct ToolUseState {
    tool_uses_by_assistant_message: HashMap<MessageId, Vec<LanguageModelToolUse>>,
    tool_uses_by_user_message: HashMap<MessageId, Vec<LanguageModelToolUseId>>,
    tool_results: HashMap<LanguageModelToolUseId, LanguageModelToolResult>,
    pending_tool_uses_by_id: HashMap<LanguageModelToolUseId, PendingToolUse>,
}

impl ToolUseState {
    pub fn new() -> Self {
        Self {
            tool_uses_by_assistant_message: HashMap::default(),
            tool_uses_by_user_message: HashMap::default(),
            tool_results: HashMap::default(),
            pending_tool_uses_by_id: HashMap::default(),
        }
    }

    /// Constructs a [`ToolUseState`] from the given list of [`SerializedMessage`]s.
    ///
    /// Accepts a function to filter the tools that should be used to populate the state.
    pub fn from_serialized_messages(
        messages: &[SerializedMessage],
        mut filter_by_tool_name: impl FnMut(&str) -> bool,
    ) -> Self {
        let mut this = Self::new();
        let mut tool_names_by_id = HashMap::default();

        for message in messages {
            match message.role {
                Role::Assistant => {
                    if !message.tool_uses.is_empty() {
                        let tool_uses = message
                            .tool_uses
                            .iter()
                            .filter(|tool_use| (filter_by_tool_name)(tool_use.name.as_ref()))
                            .map(|tool_use| LanguageModelToolUse {
                                id: tool_use.id.clone(),
                                name: tool_use.name.clone().into(),
                                input: tool_use.input.clone(),
                            })
                            .collect::<Vec<_>>();

                        tool_names_by_id.extend(
                            tool_uses
                                .iter()
                                .map(|tool_use| (tool_use.id.clone(), tool_use.name.clone())),
                        );

                        this.tool_uses_by_assistant_message
                            .insert(message.id, tool_uses);
                    }
                }
                Role::User => {
                    if !message.tool_results.is_empty() {
                        let tool_uses_by_user_message = this
                            .tool_uses_by_user_message
                            .entry(message.id)
                            .or_default();

                        for tool_result in &message.tool_results {
                            let tool_use_id = tool_result.tool_use_id.clone();
                            let Some(tool_use) = tool_names_by_id.get(&tool_use_id) else {
                                log::warn!("no tool name found for tool use: {tool_use_id:?}");
                                continue;
                            };

                            if !(filter_by_tool_name)(tool_use.as_ref()) {
                                continue;
                            }

                            tool_uses_by_user_message.push(tool_use_id.clone());
                            this.tool_results.insert(
                                tool_use_id.clone(),
                                LanguageModelToolResult {
                                    tool_use_id,
                                    is_error: tool_result.is_error,
                                    content: tool_result.content.clone(),
                                },
                            );
                        }
                    }
                }
                Role::System => {}
            }
        }

        this
    }

    pub fn cancel_pending(&mut self) -> Vec<PendingToolUse> {
        let mut pending_tools = Vec::new();
        for (tool_use_id, tool_use) in self.pending_tool_uses_by_id.drain() {
            self.tool_results.insert(
                tool_use_id.clone(),
                LanguageModelToolResult {
                    tool_use_id,
                    content: "Tool canceled by user".into(),
                    is_error: true,
                },
            );
            pending_tools.push(tool_use.clone());
        }
        pending_tools
    }

    pub fn pending_tool_uses(&self) -> Vec<&PendingToolUse> {
        self.pending_tool_uses_by_id.values().collect()
    }

    pub fn tool_uses_for_message(&self, id: MessageId) -> Vec<ToolUse> {
        let Some(tool_uses_for_message) = &self.tool_uses_by_assistant_message.get(&id) else {
            return Vec::new();
        };

        let mut tool_uses = Vec::new();

        for tool_use in tool_uses_for_message.iter() {
            let tool_result = self.tool_results.get(&tool_use.id);

            let status = (|| {
                if let Some(tool_result) = tool_result {
                    return if tool_result.is_error {
                        ToolUseStatus::Error(tool_result.content.clone().into())
                    } else {
                        ToolUseStatus::Finished(tool_result.content.clone().into())
                    };
                }

                if let Some(pending_tool_use) = self.pending_tool_uses_by_id.get(&tool_use.id) {
                    return match pending_tool_use.status {
                        PendingToolUseStatus::Idle => ToolUseStatus::Pending,
                        PendingToolUseStatus::Running { .. } => ToolUseStatus::Running,
                        PendingToolUseStatus::Error(ref err) => {
                            ToolUseStatus::Error(err.clone().into())
                        }
                    };
                }

                ToolUseStatus::Pending
            })();

            tool_uses.push(ToolUse {
                id: tool_use.id.clone(),
                name: tool_use.name.clone().into(),
                input: tool_use.input.clone(),
                status,
            })
        }

        tool_uses
    }

    pub fn tool_results_for_message(&self, message_id: MessageId) -> Vec<&LanguageModelToolResult> {
        let empty = Vec::new();

        self.tool_uses_by_user_message
            .get(&message_id)
            .unwrap_or(&empty)
            .iter()
            .filter_map(|tool_use_id| self.tool_results.get(&tool_use_id))
            .collect()
    }

    pub fn message_has_tool_results(&self, message_id: MessageId) -> bool {
        self.tool_uses_by_user_message
            .get(&message_id)
            .map_or(false, |results| !results.is_empty())
    }

    pub fn tool_result(
        &self,
        tool_use_id: &LanguageModelToolUseId,
    ) -> Option<&LanguageModelToolResult> {
        self.tool_results.get(tool_use_id)
    }

    pub fn request_tool_use(
        &mut self,
        assistant_message_id: MessageId,
        tool_use: LanguageModelToolUse,
    ) {
        self.tool_uses_by_assistant_message
            .entry(assistant_message_id)
            .or_default()
            .push(tool_use.clone());

        // The tool use is being requested by the Assistant, so we want to
        // attach the tool results to the next user message.
        let next_user_message_id = MessageId(assistant_message_id.0 + 1);
        self.tool_uses_by_user_message
            .entry(next_user_message_id)
            .or_default()
            .push(tool_use.id.clone());

        self.pending_tool_uses_by_id.insert(
            tool_use.id.clone(),
            PendingToolUse {
                assistant_message_id,
                id: tool_use.id,
                name: tool_use.name,
                input: tool_use.input,
                status: PendingToolUseStatus::Idle,
            },
        );
    }

    pub fn run_pending_tool(&mut self, tool_use_id: LanguageModelToolUseId, task: Task<()>) {
        if let Some(tool_use) = self.pending_tool_uses_by_id.get_mut(&tool_use_id) {
            tool_use.status = PendingToolUseStatus::Running {
                _task: task.shared(),
            };
        }
    }

    pub fn insert_tool_output(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        output: Result<String>,
    ) -> Option<PendingToolUse> {
        match output {
            Ok(tool_result) => {
                self.tool_results.insert(
                    tool_use_id.clone(),
                    LanguageModelToolResult {
                        tool_use_id: tool_use_id.clone(),
                        content: tool_result.into(),
                        is_error: false,
                    },
                );
                self.pending_tool_uses_by_id.remove(&tool_use_id)
            }
            Err(err) => {
                self.tool_results.insert(
                    tool_use_id.clone(),
                    LanguageModelToolResult {
                        tool_use_id: tool_use_id.clone(),
                        content: err.to_string().into(),
                        is_error: true,
                    },
                );

                if let Some(tool_use) = self.pending_tool_uses_by_id.get_mut(&tool_use_id) {
                    tool_use.status = PendingToolUseStatus::Error(err.to_string().into());
                }

                self.pending_tool_uses_by_id.get(&tool_use_id).cloned()
            }
        }
    }

    pub fn attach_tool_uses(
        &self,
        message_id: MessageId,
        request_message: &mut LanguageModelRequestMessage,
    ) {
        if let Some(tool_uses) = self.tool_uses_by_assistant_message.get(&message_id) {
            for tool_use in tool_uses {
                request_message
                    .content
                    .push(MessageContent::ToolUse(tool_use.clone()));
            }
        }
    }

    pub fn attach_tool_results(
        &self,
        message_id: MessageId,
        request_message: &mut LanguageModelRequestMessage,
    ) {
        if let Some(tool_uses) = self.tool_uses_by_user_message.get(&message_id) {
            for tool_use_id in tool_uses {
                if let Some(tool_result) = self.tool_results.get(tool_use_id) {
                    request_message
                        .content
                        .push(MessageContent::ToolResult(tool_result.clone()));
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PendingToolUse {
    pub id: LanguageModelToolUseId,
    /// The ID of the Assistant message in which the tool use was requested.
    #[allow(unused)]
    pub assistant_message_id: MessageId,
    pub name: Arc<str>,
    pub input: serde_json::Value,
    pub status: PendingToolUseStatus,
}

#[derive(Debug, Clone)]
pub enum PendingToolUseStatus {
    Idle,
    Running { _task: Shared<Task<()>> },
    Error(#[allow(unused)] Arc<str>),
}

impl PendingToolUseStatus {
    pub fn is_idle(&self) -> bool {
        matches!(self, PendingToolUseStatus::Idle)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, PendingToolUseStatus::Error(_))
    }
}

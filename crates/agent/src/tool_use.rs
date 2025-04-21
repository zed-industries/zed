use std::sync::Arc;

use anyhow::Result;
use assistant_tool::{AnyToolCard, Tool, ToolUseStatus, ToolWorkingSet};
use collections::HashMap;
use futures::FutureExt as _;
use futures::future::Shared;
use gpui::{App, Entity, SharedString, Task};
use language_model::{
    LanguageModelRegistry, LanguageModelRequestMessage, LanguageModelToolResult,
    LanguageModelToolUse, LanguageModelToolUseId, MessageContent, Role,
};
use ui::IconName;
use util::truncate_lines_to_byte_limit;

use crate::thread::MessageId;
use crate::thread_store::SerializedMessage;

#[derive(Debug)]
pub struct ToolUse {
    pub id: LanguageModelToolUseId,
    pub name: SharedString,
    pub ui_text: SharedString,
    pub status: ToolUseStatus,
    pub input: serde_json::Value,
    pub icon: ui::IconName,
    pub needs_confirmation: bool,
}

pub const USING_TOOL_MARKER: &str = "<using_tool>";

pub struct ToolUseState {
    tools: Entity<ToolWorkingSet>,
    tool_uses_by_assistant_message: HashMap<MessageId, Vec<LanguageModelToolUse>>,
    tool_uses_by_user_message: HashMap<MessageId, Vec<LanguageModelToolUseId>>,
    tool_results: HashMap<LanguageModelToolUseId, LanguageModelToolResult>,
    pending_tool_uses_by_id: HashMap<LanguageModelToolUseId, PendingToolUse>,
    tool_result_cards: HashMap<LanguageModelToolUseId, AnyToolCard>,
}

impl ToolUseState {
    pub fn new(tools: Entity<ToolWorkingSet>) -> Self {
        Self {
            tools,
            tool_uses_by_assistant_message: HashMap::default(),
            tool_uses_by_user_message: HashMap::default(),
            tool_results: HashMap::default(),
            pending_tool_uses_by_id: HashMap::default(),
            tool_result_cards: HashMap::default(),
        }
    }

    /// Constructs a [`ToolUseState`] from the given list of [`SerializedMessage`]s.
    ///
    /// Accepts a function to filter the tools that should be used to populate the state.
    pub fn from_serialized_messages(
        tools: Entity<ToolWorkingSet>,
        messages: &[SerializedMessage],
        mut filter_by_tool_name: impl FnMut(&str) -> bool,
    ) -> Self {
        let mut this = Self::new(tools);
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
                                    tool_name: tool_use.clone(),
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
                    tool_name: tool_use.name.clone(),
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

    pub fn tool_uses_for_message(&self, id: MessageId, cx: &App) -> Vec<ToolUse> {
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
                    match pending_tool_use.status {
                        PendingToolUseStatus::Idle => ToolUseStatus::Pending,
                        PendingToolUseStatus::NeedsConfirmation { .. } => {
                            ToolUseStatus::NeedsConfirmation
                        }
                        PendingToolUseStatus::Running { .. } => ToolUseStatus::Running,
                        PendingToolUseStatus::Error(ref err) => {
                            ToolUseStatus::Error(err.clone().into())
                        }
                    }
                } else {
                    ToolUseStatus::Pending
                }
            })();

            let (icon, needs_confirmation) =
                if let Some(tool) = self.tools.read(cx).tool(&tool_use.name, cx) {
                    (tool.icon(), tool.needs_confirmation(&tool_use.input, cx))
                } else {
                    (IconName::Cog, false)
                };

            tool_uses.push(ToolUse {
                id: tool_use.id.clone(),
                name: tool_use.name.clone().into(),
                ui_text: self.tool_ui_label(&tool_use.name, &tool_use.input, cx),
                input: tool_use.input.clone(),
                status,
                icon,
                needs_confirmation,
            })
        }

        tool_uses
    }

    pub fn tool_ui_label(
        &self,
        tool_name: &str,
        input: &serde_json::Value,
        cx: &App,
    ) -> SharedString {
        if let Some(tool) = self.tools.read(cx).tool(tool_name, cx) {
            tool.ui_text(input).into()
        } else {
            format!("Unknown tool {tool_name:?}").into()
        }
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

    pub fn tool_result_card(&self, tool_use_id: &LanguageModelToolUseId) -> Option<&AnyToolCard> {
        self.tool_result_cards.get(tool_use_id)
    }

    pub fn insert_tool_result_card(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        card: AnyToolCard,
    ) {
        self.tool_result_cards.insert(tool_use_id, card);
    }

    pub fn request_tool_use(
        &mut self,
        assistant_message_id: MessageId,
        tool_use: LanguageModelToolUse,
        cx: &App,
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
                name: tool_use.name.clone(),
                ui_text: self
                    .tool_ui_label(&tool_use.name, &tool_use.input, cx)
                    .into(),
                input: tool_use.input,
                status: PendingToolUseStatus::Idle,
            },
        );
    }

    pub fn run_pending_tool(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        ui_text: SharedString,
        task: Task<()>,
    ) {
        if let Some(tool_use) = self.pending_tool_uses_by_id.get_mut(&tool_use_id) {
            tool_use.ui_text = ui_text.into();
            tool_use.status = PendingToolUseStatus::Running {
                _task: task.shared(),
            };
        }
    }

    pub fn confirm_tool_use(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        ui_text: impl Into<Arc<str>>,
        input: serde_json::Value,
        messages: Arc<Vec<LanguageModelRequestMessage>>,
        tool: Arc<dyn Tool>,
    ) {
        if let Some(tool_use) = self.pending_tool_uses_by_id.get_mut(&tool_use_id) {
            let ui_text = ui_text.into();
            tool_use.ui_text = ui_text.clone();
            let confirmation = Confirmation {
                tool_use_id,
                input,
                messages,
                tool,
                ui_text,
            };
            tool_use.status = PendingToolUseStatus::NeedsConfirmation(Arc::new(confirmation));
        }
    }

    pub fn insert_tool_output(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        output: Result<String>,
        cx: &App,
    ) -> Option<PendingToolUse> {
        telemetry::event!("Agent Tool Finished", tool_name, success = output.is_ok());

        match output {
            Ok(tool_result) => {
                let model_registry = LanguageModelRegistry::read_global(cx);

                const BYTES_PER_TOKEN_ESTIMATE: usize = 3;

                // Protect from clearly large output
                let tool_output_limit = model_registry
                    .default_model()
                    .map(|model| model.model.max_token_count() * BYTES_PER_TOKEN_ESTIMATE)
                    .unwrap_or(usize::MAX);

                let tool_result = if tool_result.len() <= tool_output_limit {
                    tool_result
                } else {
                    let truncated = truncate_lines_to_byte_limit(&tool_result, tool_output_limit);

                    format!(
                        "Tool result too long. The first {} bytes:\n\n{}",
                        truncated.len(),
                        truncated
                    )
                };

                self.tool_results.insert(
                    tool_use_id.clone(),
                    LanguageModelToolResult {
                        tool_use_id: tool_use_id.clone(),
                        tool_name,
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
                        tool_name,
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
            let mut found_tool_use = false;

            for tool_use in tool_uses {
                if self.tool_results.contains_key(&tool_use.id) {
                    if !found_tool_use {
                        // The API fails if a message contains a tool use without any (non-whitespace) text around it
                        match request_message.content.last_mut() {
                            Some(MessageContent::Text(txt)) => {
                                if txt.is_empty() {
                                    txt.push_str(USING_TOOL_MARKER);
                                }
                            }
                            None | Some(_) => {
                                request_message
                                    .content
                                    .push(MessageContent::Text(USING_TOOL_MARKER.into()));
                            }
                        };
                    }

                    found_tool_use = true;

                    // Do not send tool uses until they are completed
                    request_message
                        .content
                        .push(MessageContent::ToolUse(tool_use.clone()));
                } else {
                    log::debug!(
                        "skipped tool use {:?} because it is still pending",
                        tool_use
                    );
                }
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
                    request_message.content.push(MessageContent::ToolResult(
                        LanguageModelToolResult {
                            tool_use_id: tool_use_id.clone(),
                            tool_name: tool_result.tool_name.clone(),
                            is_error: tool_result.is_error,
                            content: if tool_result.content.is_empty() {
                                // Surprisingly, the API fails if we return an empty string here.
                                // It thinks we are sending a tool use without a tool result.
                                "<Tool returned an empty string>".into()
                            } else {
                                tool_result.content.clone()
                            },
                        },
                    ));
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
    pub ui_text: Arc<str>,
    pub input: serde_json::Value,
    pub status: PendingToolUseStatus,
}

#[derive(Debug, Clone)]
pub struct Confirmation {
    pub tool_use_id: LanguageModelToolUseId,
    pub input: serde_json::Value,
    pub ui_text: Arc<str>,
    pub messages: Arc<Vec<LanguageModelRequestMessage>>,
    pub tool: Arc<dyn Tool>,
}

#[derive(Debug, Clone)]
pub enum PendingToolUseStatus {
    Idle,
    NeedsConfirmation(Arc<Confirmation>),
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

    pub fn needs_confirmation(&self) -> bool {
        matches!(self, PendingToolUseStatus::NeedsConfirmation { .. })
    }
}

use std::sync::Arc;

use anyhow::Result;
use assistant_tool::{
    AnyToolCard, Tool, ToolResultContent, ToolResultOutput, ToolUseStatus, ToolWorkingSet,
};
use collections::HashMap;
use futures::FutureExt as _;
use futures::future::Shared;
use gpui::{App, Entity, SharedString, Task};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelRequest, LanguageModelToolResult,
    LanguageModelToolResultContent, LanguageModelToolUse, LanguageModelToolUseId, Role,
};
use project::Project;
use ui::{IconName, Window};
use util::truncate_lines_to_byte_limit;

use crate::thread::{MessageId, PromptId, ThreadId};
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

pub struct ToolUseState {
    tools: Entity<ToolWorkingSet>,
    tool_uses_by_assistant_message: HashMap<MessageId, Vec<LanguageModelToolUse>>,
    tool_results: HashMap<LanguageModelToolUseId, LanguageModelToolResult>,
    pending_tool_uses_by_id: HashMap<LanguageModelToolUseId, PendingToolUse>,
    tool_result_cards: HashMap<LanguageModelToolUseId, AnyToolCard>,
    tool_use_metadata_by_id: HashMap<LanguageModelToolUseId, ToolUseMetadata>,
}

impl ToolUseState {
    pub fn new(tools: Entity<ToolWorkingSet>) -> Self {
        Self {
            tools,
            tool_uses_by_assistant_message: HashMap::default(),
            tool_results: HashMap::default(),
            pending_tool_uses_by_id: HashMap::default(),
            tool_result_cards: HashMap::default(),
            tool_use_metadata_by_id: HashMap::default(),
        }
    }

    /// Constructs a [`ToolUseState`] from the given list of [`SerializedMessage`]s.
    ///
    /// Accepts a function to filter the tools that should be used to populate the state.
    ///
    /// If `window` is `None` (e.g., when in headless mode or when running evals),
    /// tool cards won't be deserialized
    pub fn from_serialized_messages(
        tools: Entity<ToolWorkingSet>,
        messages: &[SerializedMessage],
        project: Entity<Project>,
        window: Option<&mut Window>, // None in headless mode
        cx: &mut App,
    ) -> Self {
        let mut this = Self::new(tools);
        let mut tool_names_by_id = HashMap::default();
        let mut window = window;

        for message in messages {
            match message.role {
                Role::Assistant => {
                    if !message.tool_uses.is_empty() {
                        let tool_uses = message
                            .tool_uses
                            .iter()
                            .map(|tool_use| LanguageModelToolUse {
                                id: tool_use.id.clone(),
                                name: tool_use.name.clone().into(),
                                raw_input: tool_use.input.to_string(),
                                input: tool_use.input.clone(),
                                is_input_complete: true,
                            })
                            .collect::<Vec<_>>();

                        tool_names_by_id.extend(
                            tool_uses
                                .iter()
                                .map(|tool_use| (tool_use.id.clone(), tool_use.name.clone())),
                        );

                        this.tool_uses_by_assistant_message
                            .insert(message.id, tool_uses);

                        for tool_result in &message.tool_results {
                            let tool_use_id = tool_result.tool_use_id.clone();
                            let Some(tool_use) = tool_names_by_id.get(&tool_use_id) else {
                                log::warn!("no tool name found for tool use: {tool_use_id:?}");
                                continue;
                            };

                            this.tool_results.insert(
                                tool_use_id.clone(),
                                LanguageModelToolResult {
                                    tool_use_id: tool_use_id.clone(),
                                    tool_name: tool_use.clone(),
                                    is_error: tool_result.is_error,
                                    content: tool_result.content.clone(),
                                    output: tool_result.output.clone(),
                                },
                            );

                            if let Some(window) = &mut window {
                                if let Some(tool) = this.tools.read(cx).tool(tool_use, cx) {
                                    if let Some(output) = tool_result.output.clone() {
                                        if let Some(card) = tool.deserialize_card(
                                            output,
                                            project.clone(),
                                            window,
                                            cx,
                                        ) {
                                            this.tool_result_cards.insert(tool_use_id, card);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Role::System | Role::User => {}
            }
        }

        this
    }

    pub fn cancel_pending(&mut self) -> Vec<PendingToolUse> {
        let mut cancelled_tool_uses = Vec::new();
        self.pending_tool_uses_by_id
            .retain(|tool_use_id, tool_use| {
                if matches!(tool_use.status, PendingToolUseStatus::Error { .. }) {
                    return true;
                }

                let content = "Tool canceled by user".into();
                self.tool_results.insert(
                    tool_use_id.clone(),
                    LanguageModelToolResult {
                        tool_use_id: tool_use_id.clone(),
                        tool_name: tool_use.name.clone(),
                        content,
                        output: None,
                        is_error: true,
                    },
                );
                cancelled_tool_uses.push(tool_use.clone());
                false
            });
        cancelled_tool_uses
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
                    let content = tool_result
                        .content
                        .to_str()
                        .map(|str| str.to_owned().into())
                        .unwrap_or_default();

                    return if tool_result.is_error {
                        ToolUseStatus::Error(content)
                    } else {
                        ToolUseStatus::Finished(content)
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
                        PendingToolUseStatus::InputStillStreaming => {
                            ToolUseStatus::InputStillStreaming
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
                ui_text: self.tool_ui_label(
                    &tool_use.name,
                    &tool_use.input,
                    tool_use.is_input_complete,
                    cx,
                ),
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
        is_input_complete: bool,
        cx: &App,
    ) -> SharedString {
        if let Some(tool) = self.tools.read(cx).tool(tool_name, cx) {
            if is_input_complete {
                tool.ui_text(input).into()
            } else {
                tool.still_streaming_ui_text(input).into()
            }
        } else {
            format!("Unknown tool {tool_name:?}").into()
        }
    }

    pub fn tool_results_for_message(
        &self,
        assistant_message_id: MessageId,
    ) -> Vec<&LanguageModelToolResult> {
        let Some(tool_uses) = self
            .tool_uses_by_assistant_message
            .get(&assistant_message_id)
        else {
            return Vec::new();
        };

        tool_uses
            .iter()
            .filter_map(|tool_use| self.tool_results.get(&tool_use.id))
            .collect()
    }

    pub fn message_has_tool_results(&self, assistant_message_id: MessageId) -> bool {
        self.tool_uses_by_assistant_message
            .get(&assistant_message_id)
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
        metadata: ToolUseMetadata,
        cx: &App,
    ) -> Arc<str> {
        let tool_uses = self
            .tool_uses_by_assistant_message
            .entry(assistant_message_id)
            .or_default();

        let mut existing_tool_use_found = false;

        for existing_tool_use in tool_uses.iter_mut() {
            if existing_tool_use.id == tool_use.id {
                *existing_tool_use = tool_use.clone();
                existing_tool_use_found = true;
            }
        }

        if !existing_tool_use_found {
            tool_uses.push(tool_use.clone());
        }

        let status = if tool_use.is_input_complete {
            self.tool_use_metadata_by_id
                .insert(tool_use.id.clone(), metadata);

            PendingToolUseStatus::Idle
        } else {
            PendingToolUseStatus::InputStillStreaming
        };

        let ui_text: Arc<str> = self
            .tool_ui_label(
                &tool_use.name,
                &tool_use.input,
                tool_use.is_input_complete,
                cx,
            )
            .into();

        self.pending_tool_uses_by_id.insert(
            tool_use.id.clone(),
            PendingToolUse {
                assistant_message_id,
                id: tool_use.id,
                name: tool_use.name.clone(),
                ui_text: ui_text.clone(),
                input: tool_use.input,
                status,
            },
        );

        ui_text
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
        request: Arc<LanguageModelRequest>,
        tool: Arc<dyn Tool>,
    ) {
        if let Some(tool_use) = self.pending_tool_uses_by_id.get_mut(&tool_use_id) {
            let ui_text = ui_text.into();
            tool_use.ui_text = ui_text.clone();
            let confirmation = Confirmation {
                tool_use_id,
                input,
                request,
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
        output: Result<ToolResultOutput>,
        configured_model: Option<&ConfiguredModel>,
    ) -> Option<PendingToolUse> {
        let metadata = self.tool_use_metadata_by_id.remove(&tool_use_id);

        telemetry::event!(
            "Agent Tool Finished",
            model = metadata
                .as_ref()
                .map(|metadata| metadata.model.telemetry_id()),
            model_provider = metadata
                .as_ref()
                .map(|metadata| metadata.model.provider_id().to_string()),
            thread_id = metadata.as_ref().map(|metadata| metadata.thread_id.clone()),
            prompt_id = metadata.as_ref().map(|metadata| metadata.prompt_id.clone()),
            tool_name,
            success = output.is_ok()
        );

        match output {
            Ok(output) => {
                let tool_result = output.content;
                const BYTES_PER_TOKEN_ESTIMATE: usize = 3;

                let old_use = self.pending_tool_uses_by_id.remove(&tool_use_id);

                // Protect from overly large output
                let tool_output_limit = configured_model
                    .map(|model| model.model.max_token_count() * BYTES_PER_TOKEN_ESTIMATE)
                    .unwrap_or(usize::MAX);

                let content = match tool_result {
                    ToolResultContent::Text(text) => {
                        let truncated = truncate_lines_to_byte_limit(&text, tool_output_limit);

                        LanguageModelToolResultContent::Text(
                            format!(
                                "Tool result too long. The first {} bytes:\n\n{}",
                                truncated.len(),
                                truncated
                            )
                            .into(),
                        )
                    }
                    ToolResultContent::Image(language_model_image) => {
                        if language_model_image.estimate_tokens() < tool_output_limit {
                            LanguageModelToolResultContent::Image(language_model_image)
                        } else {
                            self.tool_results.insert(
                                tool_use_id.clone(),
                                LanguageModelToolResult {
                                    tool_use_id: tool_use_id.clone(),
                                    tool_name,
                                    content: "Tool responded with an image that would exceeded the remaining tokens".into(),
                                    is_error: true,
                                    output: None,
                                },
                            );

                            return old_use;
                        }
                    }
                };

                self.tool_results.insert(
                    tool_use_id.clone(),
                    LanguageModelToolResult {
                        tool_use_id: tool_use_id.clone(),
                        tool_name,
                        content,
                        is_error: false,
                        output: output.output,
                    },
                );

                old_use
            }
            Err(err) => {
                self.tool_results.insert(
                    tool_use_id.clone(),
                    LanguageModelToolResult {
                        tool_use_id: tool_use_id.clone(),
                        tool_name,
                        content: LanguageModelToolResultContent::Text(err.to_string().into()),
                        is_error: true,
                        output: None,
                    },
                );

                if let Some(tool_use) = self.pending_tool_uses_by_id.get_mut(&tool_use_id) {
                    tool_use.status = PendingToolUseStatus::Error(err.to_string().into());
                }

                self.pending_tool_uses_by_id.get(&tool_use_id).cloned()
            }
        }
    }

    pub fn has_tool_results(&self, assistant_message_id: MessageId) -> bool {
        self.tool_uses_by_assistant_message
            .contains_key(&assistant_message_id)
    }

    pub fn tool_results(
        &self,
        assistant_message_id: MessageId,
    ) -> impl Iterator<Item = (&LanguageModelToolUse, Option<&LanguageModelToolResult>)> {
        self.tool_uses_by_assistant_message
            .get(&assistant_message_id)
            .into_iter()
            .flatten()
            .map(|tool_use| (tool_use, self.tool_results.get(&tool_use.id)))
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
    pub request: Arc<LanguageModelRequest>,
    pub tool: Arc<dyn Tool>,
}

#[derive(Debug, Clone)]
pub enum PendingToolUseStatus {
    InputStillStreaming,
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

#[derive(Clone)]
pub struct ToolUseMetadata {
    pub model: Arc<dyn LanguageModel>,
    pub thread_id: ThreadId,
    pub prompt_id: PromptId,
}

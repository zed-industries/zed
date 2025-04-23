mod templates;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, Result};
use futures::{channel::mpsc, future};
use gpui::{App, Context, Entity, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelRequestTool, LanguageModelToolResult, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, MessageContent, Role, StopReason,
};
use project::Project;
use schemars::{schema::RootSchema, JsonSchema};
use serde::Deserialize;
use smol::stream::StreamExt;
use std::{collections::BTreeMap, sync::Arc};
use templates::{BaseTemplate, Template, Templates, WorktreeData};
use util::ResultExt;

#[derive(Debug)]
pub struct AgentMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
}

impl AgentMessage {
    fn to_request_message(&self) -> LanguageModelRequestMessage {
        LanguageModelRequestMessage {
            role: self.role,
            content: self.content.clone(),
            cache: false, // TODO: Figure out caching
        }
    }
}

pub type AgentResponseEvent = LanguageModelCompletionEvent;

trait Prompt {
    fn render(&self, prompts: &Templates, cx: &App) -> Result<String>;
}

struct BasePrompt {
    project: Entity<Project>,
}

impl Prompt for BasePrompt {
    fn render(&self, templates: &Templates, cx: &App) -> Result<String> {
        BaseTemplate {
            os: std::env::consts::OS.to_string(),
            shell: util::get_system_shell(),
            worktrees: self
                .project
                .read(cx)
                .worktrees(cx)
                .map(|worktree| WorktreeData {
                    root_name: worktree.read(cx).root_name().to_string(),
                })
                .collect(),
        }
        .render(templates)
    }
}

pub struct Agent {
    messages: Vec<AgentMessage>,
    /// Holds the task that handles agent interaction until the end of the turn.
    /// Survives across multiple requests as the model performs tool calls and
    /// we run tools, report their results.
    running_turn: Option<Task<()>>,
    system_prompts: Vec<Arc<dyn Prompt>>,
    tools: BTreeMap<Arc<str>, Arc<dyn AnyTool>>,
    templates: Arc<Templates>,
    // project: Entity<Project>,
    // action_log: Entity<ActionLog>,
}

impl Agent {
    pub fn new(templates: Arc<Templates>) -> Self {
        Self {
            messages: Vec::new(),
            system_prompts: Vec::new(),
            running_turn: None,
            tools: BTreeMap::default(),
            templates,
        }
    }

    pub fn add_tool(&mut self, tool: Arc<dyn AnyTool>) {
        let name = Arc::from(tool.name());
        self.tools.insert(name, tool);
    }

    pub fn remove_tool(&mut self, name: &str) -> bool {
        self.tools.remove(name).is_some()
    }

    /// Sending a message results in the model streaming a response, which could include tool calls.
    /// After calling tools, the model will stops and waits for any outstanding tool calls to be completed and their results sent.
    /// The returned channel will report all the occurrences in which the model stops before erroring or ending its turn.
    pub fn send(
        &mut self,
        model: Arc<dyn LanguageModel>,
        content: impl Into<MessageContent>,
        cx: &mut Context<Self>,
    ) -> mpsc::UnboundedReceiver<Result<AgentResponseEvent>> {
        cx.notify();
        let (events_tx, events_rx) = mpsc::unbounded();

        let system_message = self.build_system_message(cx);
        self.messages.extend(system_message);

        self.messages.push(AgentMessage {
            role: Role::User,
            content: vec![content.into()],
        });
        self.running_turn = Some(cx.spawn(async move |thread, cx| {
            let turn_result = async {
                // Perform one request, then keep looping if the model makes tool calls.
                loop {
                    let request =
                        thread.update(cx, |thread, _cx| thread.build_completion_request())?;

                    println!(
                        "request: {}",
                        serde_json::to_string_pretty(&request).unwrap()
                    );

                    // Stream events, appending to messages and collecting up tool uses.
                    let mut events = model.stream_completion(request, cx).await?;
                    let mut tool_uses = Vec::new();
                    while let Some(event) = events.next().await {
                        match event {
                            Ok(event) => {
                                thread
                                    .update(cx, |thread, cx| {
                                        tool_uses.extend(thread.handle_response_event(
                                            event,
                                            events_tx.clone(),
                                            cx,
                                        ));
                                    })
                                    .ok();
                            }
                            Err(error) => {
                                events_tx.unbounded_send(Err(error)).ok();
                                break;
                            }
                        }
                    }

                    // If there are no tool uses, the turn is done.
                    if tool_uses.is_empty() {
                        break;
                    }

                    // If there are tool uses, wait for their results to be
                    // computed, then send them together in a single message on
                    // the next loop iteration.
                    let tool_results = future::join_all(tool_uses).await;
                    thread
                        .update(cx, |thread, _cx| {
                            thread.messages.push(AgentMessage {
                                role: Role::User,
                                content: tool_results.into_iter().map(Into::into).collect(),
                            });
                        })
                        .ok();
                }

                anyhow::Ok(())
            }
            .await;

            if let Err(error) = turn_result {
                events_tx.unbounded_send(Err(error)).ok();
            }
        }));
        events_rx
    }

    pub fn build_system_message(&mut self, cx: &App) -> Option<AgentMessage> {
        let mut system_message = AgentMessage {
            role: Role::System,
            content: Vec::new(),
        };

        for prompt in &self.system_prompts {
            if let Some(rendered_prompt) = prompt.render(&self.templates, cx).log_err() {
                system_message
                    .content
                    .push(MessageContent::Text(rendered_prompt));
            }
        }

        (!system_message.content.is_empty()).then_some(system_message)
    }

    fn handle_response_event(
        &mut self,
        event: LanguageModelCompletionEvent,
        events_tx: mpsc::UnboundedSender<Result<AgentResponseEvent>>,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        use LanguageModelCompletionEvent::*;
        events_tx.unbounded_send(Ok(event.clone())).ok();

        match event {
            Text(new_text) => self.handle_text_event(new_text, cx),
            Thinking { text, signature } => {}
            ToolUse(tool_use) => {
                return Some(self.handle_tool_use_event(tool_use, cx));
            }
            StartMessage { message_id, role } => {
                self.messages.push(AgentMessage {
                    role,
                    content: Vec::new(),
                });
            }
            UsageUpdate(token_usage) => {}
            Stop(stop_reason) => self.handle_stop_event(stop_reason),
        }

        None
    }

    fn handle_stop_event(&mut self, stop_reason: StopReason) {
        match stop_reason {
            StopReason::EndTurn | StopReason::ToolUse => {}
            StopReason::MaxTokens => todo!(),
        }
    }

    fn handle_text_event(&mut self, new_text: String, cx: &mut Context<Self>) {
        if let Some(last_message) = self.messages.last_mut() {
            debug_assert!(last_message.role == Role::Assistant);
            if let Some(MessageContent::Text(text)) = last_message.content.last_mut() {
                text.push_str(&new_text);
            } else {
                last_message.content.push(MessageContent::Text(new_text));
            }

            cx.notify();
        } else {
            todo!("does this happen in practice?");
        }
    }

    fn handle_tool_use_event(
        &mut self,
        tool_use: LanguageModelToolUse,
        cx: &mut Context<Self>,
    ) -> Task<LanguageModelToolResult> {
        if let Some(last_message) = self.messages.last_mut() {
            debug_assert!(last_message.role == Role::Assistant);
            last_message.content.push(tool_use.clone().into());
            cx.notify();
        } else {
            todo!("does this happen in practice?");
        }

        if let Some(tool) = self.tools.get(&tool_use.name) {
            let pending_tool_result = tool.clone().run(tool_use.input, cx);

            cx.foreground_executor().spawn(async move {
                match pending_tool_result.await {
                    Ok(tool_output) => LanguageModelToolResult {
                        tool_use_id: tool_use.id,
                        tool_name: tool_use.name,
                        is_error: false,
                        content: Arc::from(tool_output),
                    },
                    Err(error) => LanguageModelToolResult {
                        tool_use_id: tool_use.id,
                        tool_name: tool_use.name,
                        is_error: true,
                        content: Arc::from(error.to_string()),
                    },
                }
            })
        } else {
            Task::ready(LanguageModelToolResult {
                content: Arc::from(format!("No tool named {} exists", tool_use.name)),
                tool_use_id: tool_use.id,
                tool_name: tool_use.name,
                is_error: true,
            })
        }
    }

    fn build_completion_request(&self) -> LanguageModelRequest {
        LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            messages: self.build_request_messages(),
            tools: self
                .tools
                .values()
                .filter_map(|tool| {
                    Some(LanguageModelRequestTool {
                        name: tool.name(),
                        description: tool.description(),
                        input_schema: tool
                            .input_schema(LanguageModelToolSchemaFormat::JsonSchema)
                            .log_err()?,
                    })
                })
                .collect(),
            stop: Vec::new(),
            temperature: None,
        }
    }

    fn build_request_messages(&self) -> Vec<LanguageModelRequestMessage> {
        self.messages
            .iter()
            .map(|message| LanguageModelRequestMessage {
                role: message.role,
                content: message.content.clone(),
                cache: false,
            })
            .collect()
    }
}

pub trait Tool {
    type Input: for<'de> Deserialize<'de> + JsonSchema;

    fn name(&self) -> String;
    fn description(&self) -> String;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> RootSchema {
        assistant_tools::root_schema_for::<Self::Input>(format)
    }

    /// Runs the tool with the provided input.
    fn run(self: Arc<Self>, input: Self::Input, cx: &mut App) -> Task<Result<String>>;
}

pub trait AnyTool {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value>;
    fn run(self: Arc<Self>, input: serde_json::Value, cx: &mut App) -> Task<Result<String>>;
}

impl<T, I> AnyTool for T
where
    T: Tool<Input = I>,
    I: for<'de> Deserialize<'de> + JsonSchema,
{
    fn name(&self) -> String {
        <Self as Tool>::name(self)
    }

    fn description(&self) -> String {
        <Self as Tool>::description(self)
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(<Self as Tool>::input_schema(
            self, format,
        ))?)
    }

    fn run(self: Arc<Self>, input: serde_json::Value, cx: &mut App) -> Task<Result<String>> {
        let parsed_input: Result<I> = serde_json::from_value(input).map_err(Into::into);
        match parsed_input {
            Ok(input) => <Self as Tool>::run(self, input, cx),
            Err(error) => Task::ready(Err(anyhow!(error))),
        }
    }
}

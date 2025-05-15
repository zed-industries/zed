use anyhow::{Result, anyhow};
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Subscription, Task};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelToolChoice, StopReason, LanguageModelToolUse, LanguageModelToolUseId,
};
use language_model::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, RateLimiter, Role,
};
use lmstudio::{
    ChatCompletionRequest, ChatMessage, ModelType, get_models, preload_model,
    stream_chat_completion,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::{collections::BTreeMap, sync::Arc};
use ui::{ButtonLike, Indicator, List, prelude::*};
use util::ResultExt;

use crate::AllLanguageModelSettings;
use crate::ui::InstructionListItem;

const LMSTUDIO_DOWNLOAD_URL: &str = "https://lmstudio.ai/download";
const LMSTUDIO_CATALOG_URL: &str = "https://lmstudio.ai/models";
const LMSTUDIO_SITE: &str = "https://lmstudio.ai/";

const PROVIDER_ID: &str = "lmstudio";
const PROVIDER_NAME: &str = "LM Studio";

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LmStudioSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    /// The model name in the LM Studio API. e.g. qwen2.5-coder-7b, phi-4, etc
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the assistant panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: usize,
}

pub struct LmStudioLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<lmstudio::Model>,
    fetch_model_task: Option<Task<Result<()>>>,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();

        // As a proxy for the server being "authenticated", we'll check if its up by fetching the models
        cx.spawn(async move |this, cx| {
            let models = get_models(http_client.as_ref(), &api_url, None).await?;

            let mut models: Vec<lmstudio::Model> = models
                .into_iter()
                .filter(|model| model.r#type != ModelType::Embeddings)
                .map(|model| lmstudio::Model::new(&model.id, None, None, None))
                .collect();

            models.sort_by(|a, b| a.name.cmp(&b.name));

            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let task = self.fetch_models(cx);
        self.fetch_model_task.replace(task);
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let fetch_models_task = self.fetch_models(cx);
        cx.spawn(async move |_this, _cx| Ok(fetch_models_task.await?))
    }
}

impl LmStudioLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let this = Self {
            http_client: http_client.clone(),
            state: cx.new(|cx| {
                let subscription = cx.observe_global::<SettingsStore>({
                    let mut settings = AllLanguageModelSettings::get_global(cx).lmstudio.clone();
                    move |this: &mut State, cx| {
                        let new_settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
                        if &settings != new_settings {
                            settings = new_settings.clone();
                            this.restart_fetch_models_task(cx);
                            cx.notify();
                        }
                    }
                });

                State {
                    http_client,
                    available_models: Default::default(),
                    fetch_model_task: None,
                    _subscription: subscription,
                }
            }),
        };
        this.state
            .update(cx, |state, cx| state.restart_fetch_models_task(cx));
        this
    }
}

impl LanguageModelProviderState for LmStudioLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for LmStudioLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiLmStudio
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.provided_models(cx).into_iter().next()
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.default_model(cx)
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: BTreeMap<String, lmstudio::Model> = BTreeMap::default();

        // Add models from the LM Studio API
        for model in self.state.read(cx).available_models.iter() {
            models.insert(model.name.clone(), model.clone());
        }

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .lmstudio
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                lmstudio::Model {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    supports_tools: Some(true),
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(LmStudioLanguageModel {
                    id: LanguageModelId::from(model.name.clone()),
                    model: model.clone(),
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn load_model(&self, model: Arc<dyn LanguageModel>, cx: &App) {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();
        let id = model.id().0.to_string();
        cx.spawn(async move |_| preload_model(http_client, &api_url, &id).await)
            .detach_and_log_err(cx);
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, _window: &mut Window, cx: &mut App) -> AnyView {
        let state = self.state.clone();
        cx.new(|cx| ConfigurationView::new(state, cx)).into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.fetch_models(cx))
    }
}

pub struct LmStudioLanguageModel {
    id: LanguageModelId,
    model: lmstudio::Model,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl LmStudioLanguageModel {
    fn to_lmstudio_request(&self, request: LanguageModelRequest) -> ChatCompletionRequest {
        // Make a deep clone of the tools for debugging and to preserve them
        let tools_debug = request.tools.clone();
        
        // Check if tools are empty before moving them
        let has_tools = !request.tools.is_empty();
        
        // Convert tools to LM Studio format
        let tools = request
            .tools
            .into_iter()
            .map(|tool| lmstudio::LmStudioTool::Function {
                function: lmstudio::LmStudioFunctionTool {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect::<Vec<_>>();
        
        // Log the tools for debugging
        if !tools.is_empty() {
            log::debug!("LMStudio: Sending {} tools to model", tools.len());
            for tool in &tools_debug {
                log::debug!("  Tool: {}", tool.name);
            }
        }

        // Convert tool choice to LM Studio format
        let tool_choice = match request.tool_choice {
            Some(choice) => match choice {
                LanguageModelToolChoice::Auto => Some("auto"),
                LanguageModelToolChoice::Any => Some("any"),
                LanguageModelToolChoice::None => Some("none"),
            },
            None => if has_tools { Some("auto") } else { None },
        };

        ChatCompletionRequest {
            model: self.model.name.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|msg| match msg.role {
                    Role::User => ChatMessage::User {
                        content: msg.string_contents(),
                    },
                    Role::Assistant => ChatMessage::Assistant {
                        content: Some(msg.string_contents()),
                        tool_calls: None,
                    },
                    Role::System => ChatMessage::System {
                        content: msg.string_contents(),
                    },
                })
                .collect(),
            stream: true,
            max_tokens: Some(-1),
            stop: Some(request.stop),
            temperature: request.temperature.or(Some(0.0)),
            tools,
            tool_choice,
        }
    }
}

impl LanguageModel for LmStudioLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn supports_tools(&self) -> bool {
        // Return the model's supports_tools flag if available, otherwise default to true
        self.model.supports_tools.unwrap_or(true)
    }

    fn supports_images(&self) -> bool {
        false
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto | LanguageModelToolChoice::Any => self.supports_tools(),
            LanguageModelToolChoice::None => true
        }
    }

    fn telemetry_id(&self) -> String {
        format!("lmstudio/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        // Endpoint for this is coming soon. In the meantime, hacky estimation
        let token_count = request
            .messages
            .iter()
            .map(|msg| msg.string_contents().split_whitespace().count())
            .sum::<usize>();

        let estimated_tokens = (token_count as f64 * 0.75) as usize;
        async move { Ok(estimated_tokens) }.boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
        >,
    > {
        let request = self.to_lmstudio_request(request);

        // Log the full request for debugging
        if let Ok(request_json) = serde_json::to_string_pretty(&request) {
            log::debug!("LMStudio: Request payload:\n{}", request_json);
        }

        let http_client = self.http_client.clone();
        let Ok(api_url) = cx.update(|cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
            settings.api_url.clone()
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let response = stream_chat_completion(http_client.as_ref(), &api_url, request).await?;

            // Create a stream mapper to handle content across multiple deltas
            let stream_mapper = LmStudioStreamMapper::new();
            let stream_mapper = std::sync::Mutex::new(stream_mapper);

            let stream = response
                .map(move |response| {
                    match response {
                        Ok(fragment) => {
                            let mut mapper = stream_mapper.lock().unwrap();
                            mapper.process_fragment(fragment)
                        },
                        Err(e) => {
                            // In case of errors, we need to ensure we reset our state
                            if let Ok(mut mapper) = stream_mapper.lock() {
                                mapper.in_thinking_block = false;
                                mapper.thinking_buffer.clear();
                                mapper.pending_text = None;
                            }
                            Err(e)
                        }
                    }
                })
                .filter_map(|result| async move {
                    match result {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();

            Ok(stream)
        });

        async move {
            Ok(future
                .await?
                .map(|result| {
                    result.map_err(LanguageModelCompletionError::Other)
                })
                .boxed())
        }
        .boxed()
    }
}

struct LmStudioStreamMapper {
    in_thinking_block: bool,
    thinking_buffer: String,
    pending_text: Option<String>,
    // Tool call accumulation state
    accumulating_tool_call: bool,
    tool_call_id: Option<String>,
    tool_call_name: Option<String>,
    tool_call_args_buffer: String,
}

impl LmStudioStreamMapper {
    fn new() -> Self {
        Self {
            in_thinking_block: false,
            thinking_buffer: String::new(),
            pending_text: None,
            // Initialize tool call accumulation fields
            accumulating_tool_call: false,
            tool_call_id: None,
            tool_call_name: None,
            tool_call_args_buffer: String::new(),
        }
    }

    fn process_fragment(&mut self, fragment: lmstudio::ChatResponse) -> Result<Option<LanguageModelCompletionEvent>> {
        // Most of the time, there will be only one choice
        let Some(choice) = fragment.choices.first() else {
            return Ok(None);
        };

        // Check for finish reason first
        if let Some(reason) = choice.finish_reason.as_deref() {
            let stop_reason = match reason {
                "length" => StopReason::MaxTokens,
                "tool_calls" => StopReason::ToolUse,
                _ => StopReason::EndTurn,
            };
            
            // If we were accumulating a tool call, emit it before stopping
            if self.accumulating_tool_call && self.tool_call_name.is_some() {
                // We need to complete the current tool call
                let tool_use = self.create_tool_use_from_buffer();
                
                // Reset accumulation state
                self.reset_tool_call_state();
                
                // Return the tool use and we'll handle the stop in the next iteration
                return Ok(Some(LanguageModelCompletionEvent::ToolUse(tool_use)));
            }
            
            // Reset any state
            self.in_thinking_block = false;
            self.thinking_buffer.clear();
            self.pending_text = None;
            self.reset_tool_call_state();
            
            return Ok(Some(LanguageModelCompletionEvent::Stop(stop_reason)));
        }

        // Extract the delta content
        if let Ok(delta) =
            serde_json::from_value::<lmstudio::ResponseMessageDelta>(choice.delta.clone())
        {
            // Handle tool calls
            if let Some(tool_calls) = delta.tool_calls {
                for tool_call in tool_calls {
                    if let Some(function) = tool_call.function {
                        // Get or update the tool call ID
                        if let Some(id) = tool_call.id {
                            if self.tool_call_id.is_none() {
                                log::debug!("LMStudio: Starting tool call accumulation with ID: {}", id);
                                self.tool_call_id = Some(id);
                                self.accumulating_tool_call = true;
                            }
                        }
                        
                        // Get or update the function name
                        if let Some(name) = function.name {
                            // Don't replace a valid name with an empty one
                            if self.tool_call_name.is_none() && !name.trim().is_empty() {
                                log::debug!("LMStudio: Tool call name: {}", name);
                                self.tool_call_name = Some(name);
                            } else if self.tool_call_name.is_none() && name.trim().is_empty() {
                                // If we get an empty name and don't have one yet, log the warning
                                log::warn!("LMStudio: Received empty function name, ignoring");
                            }
                        }
                        
                        // Accumulate arguments
                        if let Some(args) = function.arguments {
                            log::debug!("LMStudio: Received argument fragment: {}", args);
                            self.tool_call_args_buffer.push_str(&args);
                            
                            // Check if the accumulated arguments form valid JSON
                            if self.is_likely_complete_json(&self.tool_call_args_buffer) {
                                log::debug!("LMStudio: Detected complete JSON arguments, emitting tool use");
                                let tool_use = self.create_tool_use_from_buffer();
                                
                                // Reset accumulation state
                                self.reset_tool_call_state();
                                
                                return Ok(Some(LanguageModelCompletionEvent::ToolUse(tool_use)));
                            }
                        }
                        
                        // We're still accumulating, so don't emit any events yet
                        return Ok(None);
                    }
                }
            }

            // Handle text content (only if we're not accumulating a tool call)
            if let Some(content) = delta.content {
                if !content.is_empty() {
                    // If we're accumulating a tool call, don't emit text events
                    if self.accumulating_tool_call {
                        return Ok(None);
                    }
                    
                    // Process thinking tags in the content
                    if self.in_thinking_block {
                        // Already in a thinking block
                        if content.contains("</think>") {
                            // End of thinking block
                            log::debug!("LMStudio: Ending thinking block");
                            let parts: Vec<&str> = content.split("</think>").collect();
                            let before_closing = parts[0];
                            
                            // Add text before closing tag to thinking buffer
                            let thinking_text = before_closing.to_string();
                            
                            // Return thinking event
                            self.in_thinking_block = false;
                            
                            // Store text after closing tag as pending
                            if parts.len() > 1 && !parts[1].is_empty() {
                                log::debug!("LMStudio: Storing pending text after thinking: {}", parts[1]);
                                self.pending_text = Some(parts[1].to_string());
                            }
                            
                            return Ok(Some(LanguageModelCompletionEvent::Thinking {
                                text: thinking_text,
                                signature: None,
                            }));
                        } else {
                            // Continue thinking block
                            log::debug!("LMStudio: Continuing thinking block: {}", content);
                            return Ok(Some(LanguageModelCompletionEvent::Thinking {
                                text: content,
                                signature: None,
                            }));
                        }
                    } else if content.contains("<think>") {
                        // Start of a thinking block
                        log::debug!("LMStudio: Starting thinking block");
                        self.in_thinking_block = true;
                        
                        // Extract content before the tag
                        let parts: Vec<&str> = content.split("<think>").collect();
                        let before_tag = parts[0];
                        
                        // Handle text before tag if any
                        if !before_tag.is_empty() {
                            self.pending_text = Some(before_tag.to_string());
                            
                            // Process this first to maintain order
                            return Ok(Some(LanguageModelCompletionEvent::Text(
                                before_tag.to_string()
                            )));
                        }
                        
                        if parts.len() > 1 {
                            let after_tag = parts[1];
                            
                            // Check if closing tag is in the same fragment
                            if after_tag.contains("</think>") {
                                // Complete thinking block in a single fragment
                                let thinking_parts: Vec<&str> = after_tag.split("</think>").collect();
                                let thinking_text = thinking_parts[0].trim();
                                
                                self.in_thinking_block = false;
                                
                                // Store text after closing tag as pending
                                if thinking_parts.len() > 1 && !thinking_parts[1].is_empty() {
                                    self.pending_text = Some(thinking_parts[1].to_string());
                                }
                                
                                // Return thinking event
                                return Ok(Some(LanguageModelCompletionEvent::Thinking {
                                    text: thinking_text.to_string(),
                                    signature: None,
                                }));
                            } else if !after_tag.is_empty() {
                                // Beginning of thinking block
                                return Ok(Some(LanguageModelCompletionEvent::Thinking {
                                    text: after_tag.to_string(),
                                    signature: None,
                                }));
                            }
                        }
                        
                        // Just the tag with nothing after it
                        return Ok(None);
                    } else if let Some(pending) = self.pending_text.take() {
                        // Return any pending text first
                        self.pending_text = Some(content);
                        return Ok(Some(LanguageModelCompletionEvent::Text(pending)));
                    } else {
                        // Regular text content
                        return Ok(Some(LanguageModelCompletionEvent::Text(content)));
                    }
                }
            }
        }

        // Check for any pending text
        if let Some(text) = self.pending_text.take() {
            return Ok(Some(LanguageModelCompletionEvent::Text(text)));
        }

        Ok(None)
    }
    
    // Check if JSON is likely to be complete
    fn is_likely_complete_json(&self, json: &str) -> bool {
        // First, simple case - parse it as JSON
        if serde_json::from_str::<serde_json::Value>(json).is_ok() {
            return true;
        }
        
        // If it can't be parsed, do some basic checks
        // Count the number of opening and closing braces
        let mut depth = 0;
        let mut inside_string = false;
        let mut was_escape = false;
        
        for c in json.chars() {
            match c {
                '"' if !was_escape => inside_string = !inside_string,
                '\\' if inside_string => was_escape = !was_escape,
                '{' if !inside_string => depth += 1,
                '}' if !inside_string => depth -= 1,
                _ => was_escape = false,
            }
        }
        
        // If we have no unclosed braces and the JSON starts with { and ends with }, it might be complete
        depth == 0 && json.trim().starts_with('{') && json.trim().ends_with('}')
    }
    
    // Create a tool use from the accumulated state
    fn create_tool_use_from_buffer(&self) -> LanguageModelToolUse {
        let id = self.tool_call_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        
        // Get the function name, ensure it's not empty, and fall back to "unknown_function" if needed
        let name = self.tool_call_name
            .clone()
            .unwrap_or_else(|| "unknown_function".to_string())
            .trim()
            .to_string();
        
        // If somehow we still have an empty name, use the fallback
        let name = if name.is_empty() { 
            log::warn!("LMStudio: Empty function name detected when creating tool use, using fallback name");
            "unknown_function".to_string() 
        } else { 
            name 
        };
            
        let args = self.tool_call_args_buffer.clone();
        
        log::debug!("LMStudio: Creating tool use - Name: {}, Args: {}", name, args);
        
        // Create the tool use with accumulated values
        LanguageModelToolUse {
            id: LanguageModelToolUseId::from(id),
            name: name.into(),
            raw_input: args.clone(),
            input: serde_json::from_str(&args).unwrap_or(serde_json::json!({})),
            is_input_complete: true,
        }
    }
    
    // Reset the tool call accumulation state
    fn reset_tool_call_state(&mut self) {
        self.accumulating_tool_call = false;
        self.tool_call_id = None;
        self.tool_call_name = None;
        self.tool_call_args_buffer.clear();
        log::debug!("LMStudio: Reset tool call accumulation state");
    }
}

struct ConfigurationView {
    state: gpui::Entity<State>,
    loading_models_task: Option<Task<()>>,
}

impl ConfigurationView {
    pub fn new(state: gpui::Entity<State>, cx: &mut Context<Self>) -> Self {
        let loading_models_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    task.await.log_err();
                }
                this.update(cx, |this, cx| {
                    this.loading_models_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            state,
            loading_models_task,
        }
    }

    fn retry_connection(&self, cx: &mut App) {
        self.state
            .update(cx, |state, cx| state.fetch_models(cx))
            .detach_and_log_err(cx);
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        let lmstudio_intro = "Run local LLMs like Llama, Phi, and Qwen.";

        if self.loading_models_task.is_some() {
            div().child(Label::new("Loading models...")).into_any()
        } else {
            v_flex()
                .gap_2()
                .child(
                    v_flex().gap_1().child(Label::new(lmstudio_intro)).child(
                        List::new()
                            .child(InstructionListItem::text_only(
                                "LM Studio needs to be running with at least one model downloaded.",
                            ))
                            .child(InstructionListItem::text_only(
                                "To get your first model, try running `lms get qwen2.5-coder-7b`",
                            )),
                    ),
                )
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .gap_2()
                        .child(
                            h_flex()
                                .w_full()
                                .gap_2()
                                .map(|this| {
                                    if is_authenticated {
                                        this.child(
                                            Button::new("lmstudio-site", "LM Studio")
                                                .style(ButtonStyle::Subtle)
                                                .icon(IconName::ArrowUpRight)
                                                .icon_size(IconSize::XSmall)
                                                .icon_color(Color::Muted)
                                                .on_click(move |_, _window, cx| {
                                                    cx.open_url(LMSTUDIO_SITE)
                                                })
                                                .into_any_element(),
                                        )
                                    } else {
                                        this.child(
                                            Button::new(
                                                "download_lmstudio_button",
                                                "Download LM Studio",
                                            )
                                            .style(ButtonStyle::Subtle)
                                            .icon(IconName::ArrowUpRight)
                                            .icon_size(IconSize::XSmall)
                                            .icon_color(Color::Muted)
                                            .on_click(move |_, _window, cx| {
                                                cx.open_url(LMSTUDIO_DOWNLOAD_URL)
                                            })
                                            .into_any_element(),
                                        )
                                    }
                                })
                                .child(
                                    Button::new("view-models", "Model Catalog")
                                        .style(ButtonStyle::Subtle)
                                        .icon(IconName::ArrowUpRight)
                                        .icon_size(IconSize::XSmall)
                                        .icon_color(Color::Muted)
                                        .on_click(move |_, _window, cx| {
                                            cx.open_url(LMSTUDIO_CATALOG_URL)
                                        }),
                                ),
                        )
                        .map(|this| {
                            if is_authenticated {
                                this.child(
                                    ButtonLike::new("connected")
                                        .disabled(true)
                                        .cursor_style(gpui::CursorStyle::Arrow)
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .child(Indicator::dot().color(Color::Success))
                                                .child(Label::new("Connected"))
                                                .into_any_element(),
                                        ),
                                )
                            } else {
                                this.child(
                                    Button::new("retry_lmstudio_models", "Connect")
                                        .icon_position(IconPosition::Start)
                                        .icon_size(IconSize::XSmall)
                                        .icon(IconName::Play)
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            this.retry_connection(cx)
                                        })),
                                )
                            }
                        }),
                )
                .into_any()
        }
    }
}

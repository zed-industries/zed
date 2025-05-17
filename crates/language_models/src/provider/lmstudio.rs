use anyhow::{Result, anyhow};
use futures::{FutureExt, future::BoxFuture, stream::BoxStream, StreamExt};
use gpui::{AnyView, App, AsyncApp, Context, Subscription, Task, UpdateGlobal};
use fs;
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
    ChatCompletionRequest, ChatMessage, preload_model,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::{collections::{BTreeMap, HashSet}, sync::Arc};
// UI imports
use ui::{ButtonLike, Indicator, List, prelude::*, ListItem, h_flex, v_flex, div, Label, Button, IconButton, LabelSize, Tooltip};
use ui_input::SingleLineInput;
use settings::update_settings_file;
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
    pub servers: Vec<LmStudioServer>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LmStudioServer {
    pub id: String, // UUID or unique identifier
    pub name: String, // User-friendly name
    pub api_url: String, // Server URL
    pub enabled: bool, // Whether this server is enabled
    pub available_models: Option<Vec<AvailableModel>>, // Models available on this server
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    /// The model name in the LM Studio API. e.g. qwen2.5-coder-7b, phi-4, etc
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the assistant panel.
    pub display_name: Option<String>,
    /// The model's context window size from server.
    pub server_max_tokens: usize,
    /// The model's context window size overridden by user (if None, use server_max_tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_max_tokens: Option<usize>,
    /// Which server this model belongs to
    pub server_id: Option<String>,
    /// Whether this model is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// For backward compatibility, max_tokens field is retained but not used directly
    #[serde(default)]
    #[serde(skip_serializing_if = "max_tokens_is_default")]
    #[deprecated]
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
        // Clear existing models
        self.available_models.clear();
        
        // Create a new task to fetch models from all enabled servers
        let http_client = self.http_client.clone();
        let settings = AllLanguageModelSettings::get_global(cx).lmstudio.clone();
        
        cx.spawn({
            let http_client = http_client.clone();
            async move |this, cx| {
                // Get all enabled servers
                let enabled_servers: Vec<LmStudioServer> = settings.servers
                    .into_iter()
                    .filter(|server| server.enabled)
                    .collect();
                
                if enabled_servers.is_empty() {
                    this.update(cx, |this, cx| {
                        this.available_models.clear();
                        cx.notify();
                    })?;
                    return Ok(());
                }
                
                let mut all_models = Vec::new();
                
                // Try to fetch models from each enabled server
                for server in enabled_servers {
                    log::info!("Checking connection to LM Studio server: {} at {}", server.name, server.api_url);
                    
                    // First check if the server is reachable
                    match lmstudio::healthcheck(&*http_client, &server.api_url).await {
                        Ok(true) => {
                            log::info!("LM Studio server {} is reachable, fetching models", server.name);
                        },
                        Ok(false) => {
                            log::warn!("LM Studio server {} is not reachable, skipping", server.name);
                            continue;
                        },
                        Err(e) => {
                            log::warn!("Error checking LM Studio server {}: {}", server.name, e);
                            continue;
                        }
                    }
                    
                    log::info!("Fetching models from LM Studio server: {} at {}", server.name, server.api_url);
                    
                    match lmstudio::get_models(&*http_client, &server.api_url, None).await {
                        Ok(local_models) => {
                            // Log incoming models
                            log::info!("Server {} returned {} models", server.name, local_models.len());
                            
                            for model in &local_models {
                                log::info!("Retrieved model: id={}, type={:?}, state={:?}", 
                                    model.id, model.r#type, model.state);
                            }
                            
                            // Convert LocalModelListing to Model
                            let models = local_models.into_iter()
                                .map(|local_model| {
                                    let id = local_model.id.clone();
                                    log::info!("Converting model {} to internal format", id);
                                    lmstudio::Model {
                                        name: local_model.id,
                                        display_name: Some(format!("{} - {}", id, server.name)),
                                        max_tokens: local_model.max_context_length.unwrap_or(8192),
                                        supports_tools: Some(true),
                                        server_id: Some(server.id.clone()),
                                    }
                                })
                                .collect::<Vec<_>>();
                            
                            log::info!("Converted {} models for server {}", models.len(), server.name);
                            all_models.extend(models.clone());
                            log::info!("All models count after extending: {}", all_models.len());
                            
                            // Store these fetched models so we can update settings later
                            let server_id = server.id.clone();
                            
                            // Get existing models to preserve enabled state
                            let existing_models = if let Some(models) = &server.available_models {
                                models.clone()
                            } else {
                                Vec::new()
                            };
                            
                            let converted_models = models.into_iter()
                                .map(|model| {
                                    // Try to find this model in existing models to preserve settings
                                    let existing_model = existing_models.iter()
                                        .find(|m| m.name == model.name);
                                        
                                    let enabled = existing_model
                                        .map(|m| m.enabled)
                                        .unwrap_or(true); // Default to enabled for new models
                                        
                                    // Get custom max tokens from existing model if available
                                    let custom_max_tokens = existing_model
                                        .and_then(|m| m.custom_max_tokens);
                                    
                                    AvailableModel {
                                        name: model.name.clone(),
                                        display_name: model.display_name.clone()
                                            .or_else(|| {
                                                let model_name = model.name.clone();
                                                let server_name = server.name.clone();
                                                Some(format!("{} - {}", model_name, server_name))
                                            }),
                                        server_max_tokens: model.max_tokens,
                                        custom_max_tokens, // Preserve custom value if it exists
                                        max_tokens: 0, // For backward compatibility
                                        server_id: Some(server_id.clone()),
                                        enabled,
                                    }
                                })
                                .collect::<Vec<_>>();
                                
                            // We'll update settings in the next phase
                            this.update(cx, move |_this, cx| {
                                // Now we have App context, so we can update settings
                                log::info!("Updating settings with {} models for server {}", 
                                    converted_models.len(), server_id);
                                
                                settings::SettingsStore::update_global(cx, |store, cx| {
                                    store.update_settings_file::<crate::AllLanguageModelSettings>(
                                        <dyn fs::Fs>::global(cx), 
                                        move |settings, _| {
                                            if let Some(lmstudio) = &mut settings.lmstudio {
                                                if let Some(servers) = &mut lmstudio.servers {
                                                    // Find the server by ID
                                                    if let Some(server) = servers.iter_mut().find(|s| s.id == server_id) {
                                                        // Update the models
                                                        server.available_models = Some(converted_models.clone());
                                                        log::info!("Updated server settings with {} models", converted_models.len());
                                                    }
                                                }
                                            }
                                        }
                                    );
                                });
                            }).ok();
                        }
                        Err(err) => {
                            log::warn!("Failed to fetch models from server {}: {}", server.name, err);
                        }
                    }
                }
                
                this.update(cx, |this, cx| {
                    this.available_models = all_models;
                    cx.notify();
                })?;
                
                Ok(())
            }
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
        let state = cx.new(|cx| {
            let mut state = State {
                http_client: http_client.clone(),
                available_models: Vec::new(),
                fetch_model_task: None,
                _subscription: cx.observe_global::<AllLanguageModelSettings>(|_, _| {}),
            };
            
            // Fetch models when created
            state.restart_fetch_models_task(cx);
            state
        });

        Self {
            http_client,
            state,
        }
    }
    
    fn create_language_model(&self, model: lmstudio::Model) -> Arc<dyn LanguageModel> {
        Arc::new(LmStudioLanguageModel {
            id: LanguageModelId::from(model.name.clone()),
            model: model.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        }) as Arc<dyn LanguageModel>
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
        log::info!("Processing models from LM Studio API, available models count: {}", self.state.read(cx).available_models.len());
        for model in self.state.read(cx).available_models.iter() {
            log::info!("Adding model from state: {}", model.name);
            models.insert(model.name.clone(), model.clone());
        }

        // Override with available models from settings and filter out disabled models
        let servers = &AllLanguageModelSettings::get_global(cx).lmstudio.servers;
        log::info!("Processing {} servers from settings", servers.len());
        
        // First, filter out any models on disabled servers
        let enabled_servers: Vec<&LmStudioServer> = servers.iter()
            .filter(|server| server.enabled)
            .collect();
            
        log::info!("Found {} enabled servers", enabled_servers.len());
        
        // Then build a list of enabled models
        let mut enabled_models = HashSet::new();
        for server in &enabled_servers {
            if let Some(available_models) = &server.available_models {
                for model in available_models {
                    if model.enabled {
                        // Only include enabled models from enabled servers
                        if let Some(server_id) = &model.server_id {
                            let key = format!("{}:{}", server_id, model.name);
                            log::info!("Marking model as enabled: {}", &key);
                            enabled_models.insert(key);
                        }
                    }
                }
            }
        }
        
        // Now filter out any models that aren't enabled
        let final_models = models.into_iter()
            .filter_map(|(name, model)| {
                let server_id = model.server_id.as_ref()?;
                let key = format!("{}:{}", server_id, name);
                
                if enabled_models.contains(&key) {
                    log::info!("Including enabled model: {}", &key);
                    Some(self.create_language_model(model))
                } else {
                    log::info!("Filtering out disabled model: {}", &key);
                    None
                }
            })
            .collect();

        final_models
    }

    fn load_model(&self, model: Arc<dyn LanguageModel>, cx: &App) {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        let http_client = self.http_client.clone();
        // Get the first enabled server or return if none
        if let Some(server) = settings.first_enabled_server() {
            let api_url = server.api_url.clone();
            let id = model.id().0.to_string();
            cx.spawn(async move |_| preload_model(http_client, &api_url, &id).await)
                .detach_and_log_err(cx);
        }
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
    fn get_server_url(&self, cx: &App) -> Result<String> {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        
        // If the model has a server_id, use that server's URL
        if let Some(server_id) = &self.model.server_id {
            for server in &settings.servers {
                if &server.id == server_id && server.enabled {
                    return Ok(server.api_url.clone());
                }
            }
            
            // If server was found but is disabled
            for server in &settings.servers {
                if &server.id == server_id {
                    return Err(anyhow!("The server '{}' is disabled", server.name));
                }
            }
            
            // Fallback to first enabled server for models with unknown server_id
            if let Some(server) = settings.first_enabled_server() {
                log::warn!("Server ID {} not found for model {}, using first enabled server instead", server_id, self.model.name);
                return Ok(server.api_url.clone());
            }
            
            return Err(anyhow!("Server not found for model {}", self.model.name));
        }
        
        // Fallback to first enabled server
        if let Some(server) = settings.first_enabled_server() {
            return Ok(server.api_url.clone());
        }
        
        // No servers configured
        Err(anyhow!("No enabled LM Studio servers found"))
    }

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
        _request: LanguageModelRequest,
        _cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        // Endpoint for this is coming soon. In the meantime, hacky estimation
        let token_count = _request
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
        // Clone needed variables for the async block
        let http_client = self.http_client.clone();
        let model_name = self.model.name.clone();
        let lmstudio_request = self.to_lmstudio_request(request);
        let model_id = self.id.0.clone();
        
        // Get the server URL upfront before the async block
        let server_url_result = cx.update(|app| {
            self.get_server_url(app)
        });
        
        async move {
            // First get the server URL, which may fail if no servers are available
            let server_url = match server_url_result {
                Ok(url_result) => match url_result {
                    Ok(url) => url,
                    Err(e) => {
                        log::error!("Failed to get server URL for model {}: {}", model_id, e);
                        return Err(anyhow!("No available LM Studio server for model {}: {}", model_id, e));
                    }
                },
                Err(e) => return Err(anyhow!("Failed during server URL lookup: {}", e)),
            };
            
            log::info!("Streaming completion from LM Studio model {} at {}", model_name, server_url);

            // Create stream mapper to handle the response
            let mut stream_mapper = LmStudioStreamMapper::new();

            // Get streaming response from LM Studio
            let stream = match lmstudio::stream_chat_completion(
                &*http_client,
                &server_url,
                lmstudio_request,
            )
            .await {
                Ok(stream) => stream,
                Err(err) => {
                    log::error!("Error streaming from LM Studio: {}", err);
                    return Err(anyhow!("Error connecting to LM Studio: {}", err));
                }
            };

            // Map the stream to LanguageModelCompletionEvent
            let mapped_stream = stream.map(move |fragment| {
                match fragment {
                    Ok(chat_response) => {
                        match stream_mapper.process_fragment(chat_response) {
                            Ok(Some(event)) => Ok(event),
                            Ok(None) => Ok(LanguageModelCompletionEvent::Text(String::new())), // Send empty text for fragments that don't produce events
                            Err(e) => Err(LanguageModelCompletionError::Other(e)),
                        }
                    }
                    Err(e) => Err(LanguageModelCompletionError::Other(anyhow!("{}", e))),
                }
            })
            .boxed();

            Ok(mapped_stream)
        }
        .boxed()
    }
}

#[derive(Clone)]
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
    selected_server_index: Option<usize>,
    editing_server_index: Option<usize>,
    server_edit_name: String,
    server_edit_url: String,
    is_adding_model: bool,
    new_model_name: String,
    new_model_display_name: String,
    new_model_max_tokens: String,
    // Max tokens editing state
    is_editing_max_tokens: bool,
    editing_model_server_id: Option<String>,
    editing_model_name: Option<String>,
    edit_max_tokens_value: String,
    // New server form state
    is_adding_server: bool,
    new_server_name: String,
    new_server_url: String,
    // Text input entities
    server_edit_name_input: Option<gpui::Entity<SingleLineInput>>,
    server_edit_url_input: Option<gpui::Entity<SingleLineInput>>,
    new_model_name_input: Option<gpui::Entity<SingleLineInput>>,
    new_model_display_name_input: Option<gpui::Entity<SingleLineInput>>,
    new_model_max_tokens_input: Option<gpui::Entity<SingleLineInput>>,
    new_server_name_input: Option<gpui::Entity<SingleLineInput>>,
    new_server_url_input: Option<gpui::Entity<SingleLineInput>>,
    edit_max_tokens_input: Option<gpui::Entity<SingleLineInput>>,
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
            selected_server_index: None,
            editing_server_index: None,
            server_edit_name: String::new(),
            server_edit_url: String::new(),
            is_adding_model: false,
            new_model_name: String::new(),
            new_model_display_name: String::new(),
            new_model_max_tokens: String::new(),
            // Max tokens editing state
            is_editing_max_tokens: false,
            editing_model_server_id: None,
            editing_model_name: None,
            edit_max_tokens_value: String::new(),
            // New server form state
            is_adding_server: false,
            new_server_name: String::new(),
            new_server_url: String::new(),
            // Text input entities
            server_edit_name_input: None,
            server_edit_url_input: None,
            new_model_name_input: None,
            new_model_display_name_input: None,
            new_model_max_tokens_input: None,
            new_server_name_input: None,
            new_server_url_input: None,
            edit_max_tokens_input: None,
        }
    }

    // Helper methods for text input creation and updates
    fn create_server_edit_inputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Create server edit name input if it doesn't exist
        if self.server_edit_name_input.is_none() {
            let name_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Server name");
                if !self.server_edit_name.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.server_edit_name.clone(), window, cx);
                    });
                }
                input
            });
            self.server_edit_name_input = Some(name_input);
        }

        // Create server edit URL input if it doesn't exist
        if self.server_edit_url_input.is_none() {
            let url_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Server URL");
                if !self.server_edit_url.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.server_edit_url.clone(), window, cx);
                    });
                }
                input
            });
            self.server_edit_url_input = Some(url_input);
        }
    }

    fn create_new_server_inputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Create new server name input if it doesn't exist
        if self.new_server_name_input.is_none() {
            let name_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Server name");
                if !self.new_server_name.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_server_name.clone(), window, cx);
                    });
                }
                input
            });
            self.new_server_name_input = Some(name_input);
        }

        // Create new server URL input if it doesn't exist
        if self.new_server_url_input.is_none() {
            let url_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Server URL");
                if !self.new_server_url.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_server_url.clone(), window, cx);
                    });
                }
                input
            });
            self.new_server_url_input = Some(url_input);
        }
    }

    fn create_model_inputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Create model name input if it doesn't exist
        if self.new_model_name_input.is_none() {
            let name_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Model name");
                if !self.new_model_name.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_model_name.clone(), window, cx);
                    });
                }
                input
            });
            self.new_model_name_input = Some(name_input);
        }

        // Create model display name input if it doesn't exist
        if self.new_model_display_name_input.is_none() {
            let display_name_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Display name (optional)");
                if !self.new_model_display_name.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_model_display_name.clone(), window, cx);
                    });
                }
                input
            });
            self.new_model_display_name_input = Some(display_name_input);
        }

        // Create model max tokens input if it doesn't exist
        if self.new_model_max_tokens_input.is_none() {
            let max_tokens_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Max tokens");
                if !self.new_model_max_tokens.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_model_max_tokens.clone(), window, cx);
                    });
                } else {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text("8192".to_string(), window, cx);
                    });
                }
                input
            });
            self.new_model_max_tokens_input = Some(max_tokens_input);
        }
    }

    fn update_field_from_input(&mut self, cx: &mut Context<Self>) {
        // Update server edit fields
        if let Some(name_input) = &self.server_edit_name_input {
            self.server_edit_name = name_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        if let Some(url_input) = &self.server_edit_url_input {
            self.server_edit_url = url_input.read(cx).editor.read(cx).text(cx).to_string();
        }

        // Update new server fields
        if let Some(name_input) = &self.new_server_name_input {
            self.new_server_name = name_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        if let Some(url_input) = &self.new_server_url_input {
            self.new_server_url = url_input.read(cx).editor.read(cx).text(cx).to_string();
        }

        // Update model fields
        if let Some(name_input) = &self.new_model_name_input {
            self.new_model_name = name_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        if let Some(display_name_input) = &self.new_model_display_name_input {
            self.new_model_display_name = display_name_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        if let Some(max_tokens_input) = &self.new_model_max_tokens_input {
            self.new_model_max_tokens = max_tokens_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        
        // Update max tokens edit field
        if let Some(max_tokens_input) = &self.edit_max_tokens_input {
            self.edit_max_tokens_value = max_tokens_input.read(cx).editor.read(cx).text(cx).to_string();
        }
    }

    fn retry_connection(&self, cx: &mut App) {
        self.state
            .update(cx, |state, cx| state.fetch_models(cx))
            .detach_and_log_err(cx);
    }
    
    fn edit_server(&mut self, index: usize, cx: &mut Context<Self>) {
        let settings = AllLanguageModelSettings::get_global(cx);
        let servers = &settings.lmstudio.servers;
        
        if servers.is_empty() || index >= servers.len() {
            log::error!("Cannot edit server: invalid index");
            return;
        }
        
        // Store the server information for editing
        self.editing_server_index = Some(index);
        self.server_edit_name = servers[index].name.clone();
        self.server_edit_url = servers[index].api_url.clone();
        
        // Clear any existing inputs to recreate them with new values
        self.server_edit_name_input = None;
        self.server_edit_url_input = None;
        
        cx.notify();
    }
    
    fn save_server_edits(&mut self, cx: &mut Context<Self>) {
        let Some(index) = self.editing_server_index else {
            log::error!("No server being edited");
            return;
        };
        
        // Update field values from inputs
        self.update_field_from_input(cx);
        
        // Make sure we have valid data
        if self.server_edit_name.trim().is_empty() || self.server_edit_url.trim().is_empty() {
            log::error!("Server name and URL cannot be empty");
            return;
        }
        
        let settings = AllLanguageModelSettings::get_global(cx);
        let servers = &settings.lmstudio.servers;
        
        if servers.is_empty() || index >= servers.len() {
            log::error!("Cannot save server edits: invalid index");
            self.editing_server_index = None;
            return;
        }
        
        // Get server ID to preserve it
        let server_id = servers[index].id.clone();
        let enabled = servers[index].enabled;
        let old_url = servers[index].api_url.clone();
        let new_url = self.server_edit_url.trim().to_string();
        
        // Check if URL has changed
        let url_changed = old_url != new_url;
        
        // Create updated server object
        let updated_server = LmStudioServer {
            id: server_id.clone(),
            name: self.server_edit_name.trim().to_string(),
            api_url: new_url.clone(),
            enabled,
            available_models: servers[index].available_models.clone(),
        };
        
        let name_for_log = updated_server.name.clone();
        let url_for_log = updated_server.api_url.clone();
        let id_for_log = updated_server.id.clone();
        let server_index = index;
        
        // Use SettingsStore to update settings
        settings::SettingsStore::update_global(cx, |store, cx| {
            store.update_settings_file::<crate::AllLanguageModelSettings>(
                <dyn fs::Fs>::global(cx), 
                move |settings, _| {
                    if let Some(lmstudio) = &mut settings.lmstudio {
                        if let Some(servers) = &mut lmstudio.servers {
                            if server_index < servers.len() {
                                servers[server_index] = updated_server;
                                log::info!(
                                    "Updated server: {} at {} with ID {}", 
                                    name_for_log, 
                                    url_for_log,
                                    id_for_log
                                );
                            }
                        }
                    }
                }
            );
        });
        
        // Reset edit state
        self.editing_server_index = None;
        self.server_edit_name_input = None;
        self.server_edit_url_input = None;
        
        // If URL changed, fetch models from the new server URL
        if url_changed && enabled {
            log::info!("Server URL changed, fetching models from new URL");
            self.fetch_models_from_server(server_id, new_url, cx);
        }
        
        // Refresh models
        self.state.update(cx, |state, cx| state.restart_fetch_models_task(cx));
        
        cx.notify();
    }
    
    fn cancel_server_edits(&mut self, cx: &mut Context<Self>) {
        self.editing_server_index = None;
        self.server_edit_name_input = None;
        self.server_edit_url_input = None;
        cx.notify();
    }
    
    fn remove_server(&mut self, index: usize, cx: &mut Context<Self>) {
        let settings = AllLanguageModelSettings::get_global(cx);
        let servers = &settings.lmstudio.servers;
        
        if servers.is_empty() || index >= servers.len() {
            log::error!("Cannot remove server: invalid index");
            return;
        }
        
        let server_name = servers[index].name.clone();
        
        // Get the filesystem
        let fs = <dyn fs::Fs>::global(cx);
        
        // Server name for the log message in the closure
        let server_name_clone = server_name.clone();
        
        // Update settings to remove the server
        update_settings_file::<crate::AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if index < servers.len() {
                        servers.remove(index);
                        log::info!("Removed server: {}", server_name_clone);
                    }
                }
            }
        });
        
        // Reset server selection
        self.selected_server_index = None;
        
        // Refresh models
        self.state.update(cx, |state, cx| state.restart_fetch_models_task(cx));
        
        cx.notify();
    }
    
    fn toggle_server(&mut self, index: usize, cx: &mut Context<Self>) {
        let settings = AllLanguageModelSettings::get_global(cx);
        let servers = &settings.lmstudio.servers;
        
        if servers.is_empty() || index >= servers.len() {
            log::error!("Cannot toggle server: invalid index");
            return;
        }
        
        let server_name = servers[index].name.clone();
        let new_enabled_state = !servers[index].enabled;
        
        // Get the filesystem
        let fs = <dyn fs::Fs>::global(cx);
        
        // Variables for the closure
        let server_name_clone = server_name.clone();
        let new_state = new_enabled_state;
        
        // Update settings to toggle the server enabled state
        update_settings_file::<crate::AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if index < servers.len() {
                        servers[index].enabled = new_state;
                        log::info!(
                            "Server '{}' is now {}", 
                            server_name_clone, 
                            if new_state { "enabled" } else { "disabled" }
                        );
                    }
                }
            }
        });
        
        // Refresh models
        self.state.update(cx, |state, cx| state.restart_fetch_models_task(cx));
        
        cx.notify();
    }
    
    fn select_server(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        self.selected_server_index = index;
        cx.notify();
    }

    fn add_model(&mut self, cx: &mut Context<Self>) {
        if let Some(server_idx) = self.selected_server_index {
            let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
            
            if server_idx < settings.servers.len() {
                // Refresh models
                self.state.update(cx, |state, cx| state.restart_fetch_models_task(cx));
                
                // Reset form state
                self.is_adding_model = false;
                self.new_model_name.clear();
                self.new_model_display_name.clear();
                self.new_model_max_tokens.clear();
                
                cx.notify();
            }
        }
    }
    
    fn remove_model(&mut self, model_name: String, server_id: String, cx: &mut Context<Self>) {
        // Get filesystem
        let fs = <dyn fs::Fs>::global(cx);
        
        // Clone values for the closure
        let model_name_clone = model_name.clone();
        let server_id_clone = server_id.clone();
        
        // Update settings
        update_settings_file::<crate::AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    // Find the server with matching ID
                    for server in servers.iter_mut() {
                        if server.id == server_id_clone {
                            if let Some(models) = &mut server.available_models {
                                // Remove model with matching name
                                let before_len = models.len();
                                models.retain(|m| m.name != model_name_clone);
                                let after_len = models.len();
                                
                                if before_len > after_len {
                                    log::info!("Removed model {} from server {}", model_name_clone, server.name);
                                }
                            }
                            break;
                        }
                    }
                }
            }
        });
        
        // Refresh models
        self.state.update(cx, |state, cx| state.restart_fetch_models_task(cx));
        
        cx.notify();
    }

    fn toggle_add_server_form(&mut self, cx: &mut Context<Self>) {
        self.is_adding_server = !self.is_adding_server;
        
        if self.is_adding_server {
            // Initialize with default values when opening the form
            self.new_server_name = "New LM Studio Server".to_string();
            self.new_server_url = "http://localhost:1234/v1".to_string();
            
            // Clear input entities to recreate them with new values
            self.new_server_name_input = None;
            self.new_server_url_input = None;
        } else {
            // Clean up when closing the form
            self.new_server_name_input = None;
            self.new_server_url_input = None;
        }
        
        cx.notify();
    }
    
    fn add_new_server(&mut self, cx: &mut Context<Self>) {
        // Update field values from inputs
        self.update_field_from_input(cx);
        
        // Validate inputs
        if self.new_server_name.trim().is_empty() || self.new_server_url.trim().is_empty() {
            log::error!("Server name and URL cannot be empty");
            return;
        }
        
        // Create new server with user-entered values
        let new_server = LmStudioServer {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.new_server_name.trim().to_string(),
            api_url: self.new_server_url.trim().to_string(),
            enabled: true,
            available_models: None,
        };
        
        // Log server addition
        log::info!(
            "Adding new LM Studio server: {} at {} with ID {}", 
            new_server.name, 
            new_server.api_url,
            new_server.id
        );
        
        // Clone for closure
        let server_clone = new_server.clone();
        let server_id = new_server.id.clone();
        let server_url = new_server.api_url.clone();
        
        // Use SettingsStore to update settings
        settings::SettingsStore::update_global(cx, |store, cx| {
            store.update_settings_file::<crate::AllLanguageModelSettings>(
                <dyn fs::Fs>::global(cx), 
                move |settings, _| {
                    // Get or initialize lmstudio settings
                    if settings.lmstudio.is_none() {
                        settings.lmstudio = Some(Default::default());
                    }
                    
                    // Ensure servers collection exists
                    if let Some(lmstudio) = &mut settings.lmstudio {
                        if lmstudio.servers.is_none() {
                            lmstudio.servers = Some(Vec::new());
                        }
                        
                        // Add server
                        if let Some(servers) = &mut lmstudio.servers {
                            servers.push(server_clone);
                        }
                    }
                }
            );
        });
        
        // Reset form and state
        self.is_adding_server = false;
        self.new_server_name = String::new();
        self.new_server_url = String::new();
        self.new_server_name_input = None;
        self.new_server_url_input = None;
        
        // Fetch models from the new server
        self.fetch_models_from_server(server_id, server_url, cx);
        
        // Refresh models via state
        self.state.update(cx, |state, cx| state.restart_fetch_models_task(cx));
        
        cx.notify();
    }

    // Helper method to fetch models from a specific server
    fn fetch_models_from_server(&self, server_id: String, server_url: String, cx: &mut Context<Self>) {
        log::info!("Fetching models from newly added server at {}", server_url);
        
        let http_client = self.state.read(cx).http_client.clone();
        
        // Spawn a background task to fetch models
        cx.spawn({
            let http_client = http_client.clone();
            async move |this, cx| {
                // Attempt to fetch models from the server
                match lmstudio::get_models(&*http_client, &server_url, None).await {
                    Ok(local_models) => {
                        if local_models.is_empty() {
                            log::info!("No models found on server at {}", server_url);
                            return;
                        }
                        
                        // Convert models to AvailableModel format
                        let models = local_models.into_iter()
                            .map(|local_model| {
                                // Store the ID first
                                let model_id = local_model.id.clone();
                                // Check if this model already exists to preserve custom settings
                                let existing_model_data = {
                                    cx.update(|cx| {
                                        let settings = AllLanguageModelSettings::get_global(cx);
                                        settings.lmstudio.servers.iter()
                                            .find(|s| s.id == server_id)
                                            .and_then(|s| s.available_models.as_ref())
                                            .and_then(|models| models.iter().find(|m| m.name == model_id))
                                            .map(|m| (m.custom_max_tokens, m.enabled))
                                    }).unwrap_or(None)
                                };
                                
                                // Extract the data from the tuple
                                let (custom_max_tokens, enabled) = match existing_model_data {
                                    Some((custom_tokens, is_enabled)) => (custom_tokens, is_enabled),
                                    None => (None, true),
                                };
                                
                                // Get server name for the display
                                let server_name = cx.update(|cx| {
                                    let settings = AllLanguageModelSettings::get_global(cx);
                                    settings.lmstudio.servers.iter()
                                        .find(|s| s.id == server_id)
                                        .map(|s| s.name.clone())
                                        .unwrap_or_else(|| {
                                            // Use URL as fallback if server not found
                                            let url_parts: Vec<&str> = server_url.split('/').collect();
                                            if url_parts.len() >= 3 {
                                                url_parts[2].to_string()
                                            } else {
                                                "Unknown Server".to_string()
                                            }
                                        })
                                }).unwrap_or_else(|_| "Unknown Server".to_string());
                                
                                AvailableModel {
                                    name: model_id.clone(),
                                    display_name: Some(format!("{} - {}", model_id, server_name)),
                                    server_max_tokens: local_model.max_context_length.unwrap_or(8192),
                                    custom_max_tokens,
                                    max_tokens: 0, // For backward compatibility
                                    server_id: Some(server_id.clone()),
                                    enabled,
                                }
                            })
                            .collect::<Vec<_>>();
                        
                        let models_count = models.len();
                        let server_id_clone = server_id.clone();
                        
                        log::info!("Found {} models on newly added server", models_count);
                        
                        // Update settings via SettingsStore instead of using update_global
                        if let Ok(()) = this.update(cx, |this, cx| {
                            // Use SettingsStore to update settings
                            settings::SettingsStore::update_global(cx, |store, cx| {
                                store.update_settings_file::<crate::AllLanguageModelSettings>(
                                    <dyn fs::Fs>::global(cx), 
                                    move |settings, _| {
                                        if let Some(lmstudio) = &mut settings.lmstudio {
                                            if let Some(servers) = &mut lmstudio.servers {
                                                // Find the server by ID
                                                if let Some(server) = servers.iter_mut().find(|s| s.id == server_id) {
                                                    // Update the models
                                                    server.available_models = Some(models.clone());
                                                    log::info!("Updated server with {} models", models.len());
                                                }
                                            }
                                        }
                                    }
                                );
                            });
                            
                            // Refresh the models list
                            this.state.update(cx, |state, cx| {
                                state.restart_fetch_models_task(cx);
                            });
                        }) {
                            log::info!("Successfully updated models for server {}", server_id_clone);
                        } else {
                            log::error!("Failed to update UI state with new models");
                        }
                    },
                    Err(err) => {
                        log::warn!("Failed to fetch models from new server at {}: {}", server_url, err);
                    }
                }
            }
        }).detach();
    }

    fn toggle_add_model_form(&mut self, cx: &mut Context<Self>) {
        self.is_adding_model = !self.is_adding_model;
        
        // Initialize with defaults when opening the form
        if self.is_adding_model {
            self.new_model_name = String::new();
            self.new_model_display_name = String::new();
            self.new_model_max_tokens = "8192".to_string();
            
            // Clear input entities to recreate them with new values
            self.new_model_name_input = None;
            self.new_model_display_name_input = None;
            self.new_model_max_tokens_input = None;
        } else {
            // Clean up when closing the form
            self.new_model_name_input = None;
            self.new_model_display_name_input = None;
            self.new_model_max_tokens_input = None;
        }
        
        cx.notify();
    }

    fn add_custom_model(&mut self, cx: &mut Context<Self>) {
        // Update field values from inputs
        self.update_field_from_input(cx);
        
        // Require server selection
        if self.selected_server_index.is_none() {
            log::error!("No server selected for adding a model");
            return;
        }
        
        // Validate inputs
        if self.new_model_name.trim().is_empty() {
            log::error!("Model name cannot be empty");
            return;
        }
        
        // Parse max tokens
        let max_tokens = match self.new_model_max_tokens.parse::<usize>() {
            Ok(tokens) => tokens,
            Err(_) => {
                log::error!("Invalid max tokens value: {}", self.new_model_max_tokens);
                return;
            }
        };
        
        // Get server ID and index
        let (server_id, server_idx) = {
            let settings = AllLanguageModelSettings::get_global(cx);
            let servers = &settings.lmstudio.servers;
            let server_idx = self.selected_server_index.unwrap();
            
            if server_idx >= servers.len() {
                log::error!("Invalid server index");
                return;
            }
            
            (servers[server_idx].id.clone(), server_idx)
        };
        
        // Create new model
        let new_model = AvailableModel {
            name: self.new_model_name.trim().to_string(),
            display_name: if self.new_model_display_name.trim().is_empty() {
                // If no display name provided, create one with server info
                let settings = AllLanguageModelSettings::get_global(cx);
                let server_name = settings.lmstudio.servers.get(server_idx)
                    .map(|s| s.name.clone())
                    .unwrap_or_default();
                Some(format!("{} - {}", self.new_model_name.trim(), server_name))
            } else {
                Some(self.new_model_display_name.trim().to_string())
            },
            server_max_tokens: max_tokens,
            custom_max_tokens: None,
            max_tokens: 0, // For backward compatibility
            server_id: Some(server_id.clone()),
            enabled: true,
        };
        
        log::info!(
            "Adding new model: {} with max tokens: {} for server: {}", 
            new_model.name, 
            new_model.server_max_tokens,
            server_id
        );
        
        // Clone for closure
        let model_clone = new_model.clone();
        let server_index = server_idx;
        
        // Get filesystem
        let fs = <dyn fs::Fs>::global(cx);
        
        // Update settings
        update_settings_file::<crate::AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if server_index < servers.len() {
                        // Initialize available_models if it's None
                        if servers[server_index].available_models.is_none() {
                            servers[server_index].available_models = Some(Vec::new());
                        }
                        
                        // Add the model to the server's available_models
                        if let Some(models) = &mut servers[server_index].available_models {
                            // Check if model with same name already exists
                            if !models.iter().any(|m| m.name == model_clone.name) {
                                let model_name = model_clone.name.clone();
                                models.push(model_clone);
                                log::info!("Added model {} to server {}", model_name, servers[server_index].name);
                            } else {
                                log::warn!("Model {} already exists for server {}", model_clone.name, servers[server_index].name);
                            }
                        }
                    }
                }
            }
        });
        
        // Reset form and state
        self.is_adding_model = false;
        self.new_model_name = String::new();
        self.new_model_display_name = String::new();
        self.new_model_max_tokens = String::new();
        self.new_model_name_input = None;
        self.new_model_display_name_input = None;
        self.new_model_max_tokens_input = None;
        
        // Refresh models
        self.state.update(cx, |state, cx| state.restart_fetch_models_task(cx));
        
        cx.notify();
    }

    fn render_models_for_server(&self, server: &LmStudioServer, cx: &mut Context<Self>) -> ui::AnyElement {
        // Get the server's models if they exist
        let server_models = if let Some(models) = &server.available_models {
            models.as_slice()
        } else {
            &[] // Empty slice if no models
        };
            
        if server_models.is_empty() {
            div()
                .p_2()
                .child(
                    Label::new("No models configured for this server")
                        .color(Color::Muted)
                        .size(LabelSize::XSmall)
                )
                .into_any_element()
        } else {
            // Create simpler list of models without clickable buttons to avoid borrowing issues
            v_flex()
                .gap_1()
                .children(
                    server_models.iter().map(|model| {
                        let display_name = model.display_name.clone().unwrap_or_else(|| model.name.clone());
                        
                        // Format tokens text differently depending on if custom value is set
                        let tokens_text = if let Some(custom) = model.custom_max_tokens {
                            if custom != model.server_max_tokens {
                                format!("{}k tokens (server: {}k)", 
                                    custom / 1000, 
                                    model.server_max_tokens / 1000)
                            } else {
                                format!("{}k tokens", custom / 1000)
                            }
                        } else {
                            format!("{}k tokens (server default)", model.server_max_tokens / 1000)
                        };
                        
                        let server_id = server.id.clone();
                        let model_name = model.name.clone();
                        let is_enabled = model.enabled;
                        
                        let index_in_list = server_models.iter().position(|m| m.name == model.name).unwrap_or(0);
                        
                        // Create a unique ID for each button using NamedInteger
                        let toggle_button = IconButton::new(
                            ElementId::NamedInteger("toggle".into(), index_in_list as u64),
                            if is_enabled { IconName::Check } else { IconName::Circle }
                        );
                        
                        h_flex()
                            .justify_between()
                            .w_full()
                            .p_2()
                            .gap_2()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(Label::new(display_name))
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                Label::new(&model.name)
                                                    .color(Color::Muted)
                                                    .size(LabelSize::XSmall)
                                            )
                                            .child(
                                                Label::new(tokens_text)
                                                    .color(Color::Muted)
                                                    .size(LabelSize::XSmall)
                                            )
                                    )
                            )
                            .child(
                                h_flex()
                                .gap_1()
                                .child(
                                    // Add edit max tokens button
                                    IconButton::new(
                                        ElementId::NamedInteger("edit-tokens".into(), index_in_list as u64),
                                        IconName::Pencil
                                    )
                                    .tooltip(Tooltip::text("Edit Max Tokens"))
                                    .icon_color(Color::Info)
                                    .on_click(cx.listener({
                                        let server_id = server_id.clone();
                                        let model_name = model_name.clone();
                                        move |this, _, _, cx| {
                                            this.show_edit_max_tokens_dialog(server_id.clone(), model_name.clone(), cx);
                                        }
                                    }))
                                )
                                .child(
                                    // Add the toggle button 
                                    toggle_button
                                        .tooltip(if is_enabled {
                                            Tooltip::text("Disable Model")
                                        } else {
                                            Tooltip::text("Enable Model")
                                        })
                                        .icon_color(if is_enabled { Color::Success } else { Color::Muted })
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            log::info!("Toggle clicked for model: {}", model_name);
                                            this.toggle_model_enabled(server_id.clone(), model_name.clone(), cx);
                                        }))
                                )
                            )
                    })
                )
                .into_any_element()
        }
    }

    fn toggle_model_enabled(&mut self, server_id: String, model_name: String, cx: &mut Context<Self>) {
        log::info!("Toggling model '{}' for server '{}'", model_name, server_id);
        
        // Get a copy of current settings to see what's available
        let settings = AllLanguageModelSettings::get_global(cx);
        let servers = &settings.lmstudio.servers;
        
        // Debug what's available
        log::info!("Current servers: {}", servers.len());
        for (i, server) in servers.iter().enumerate() {
            log::info!("Server {}: id={}, name={}", i, server.id, server.name);
            
            if let Some(models) = &server.available_models {
                log::info!("  Models: {}", models.len());
                for (j, model) in models.iter().enumerate() {
                    log::info!("    Model {}: name={}, enabled={}", j, model.name, model.enabled);
                }
            } else {
                log::info!("  No models");
            }
        }
        
        // Get filesystem
        let fs = <dyn fs::Fs>::global(cx);
        
        // Clone values for the closure
        let model_name_clone = model_name.clone();
        let server_id_clone = server_id.clone();
        
        // Update settings
        update_settings_file::<crate::AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    // Find the server with matching ID
                    for server in servers.iter_mut() {
                        if server.id == server_id_clone {
                            if let Some(models) = &mut server.available_models {
                                // Find the model with the matching name
                                for model in models.iter_mut() {
                                    if model.name == model_name_clone {
                                        // Toggle the enabled state
                                        model.enabled = !model.enabled;
                                        log::info!(
                                            "Model {} is now {}", 
                                            model_name_clone, 
                                            if model.enabled { "enabled" } else { "disabled" }
                                        );
                                        break;
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        });
        
        // Refresh the models list to reflect the changes
        self.state.update(cx, |state, cx| {
            state.restart_fetch_models_task(cx);
        });
        
        cx.notify();
    }

    fn show_edit_max_tokens_dialog(&mut self, server_id: String, model_name: String, cx: &mut Context<Self>) {
        // Find the model to get its current max tokens
        let settings = AllLanguageModelSettings::get_global(cx);
        let servers = &settings.lmstudio.servers;
        
        // Find the server and model
        let mut server_max_tokens = 8192; // Default if not found
        let mut custom_max_tokens = None;
        
        for server in servers {
            if server.id == server_id {
                if let Some(models) = &server.available_models {
                    for model in models {
                        if model.name == model_name {
                            server_max_tokens = model.server_max_tokens;
                            custom_max_tokens = model.custom_max_tokens;
                            break;
                        }
                    }
                }
                break;
            }
        }
        
        // Set dialog state
        self.is_editing_max_tokens = true;
        self.editing_model_server_id = Some(server_id);
        self.editing_model_name = Some(model_name);
        
        // Initialize the dialog with current value or default
        self.edit_max_tokens_value = custom_max_tokens
            .unwrap_or(server_max_tokens)
            .to_string();
        
        // Clear the input entity so it will be recreated
        self.edit_max_tokens_input = None;
        
        cx.notify();
    }
    
    fn create_max_tokens_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.edit_max_tokens_input.is_none() {
            let tokens_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Max tokens");
                if !self.edit_max_tokens_value.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.edit_max_tokens_value.clone(), window, cx);
                    });
                }
                input
            });
            self.edit_max_tokens_input = Some(tokens_input);
        }
    }
    
    fn save_max_tokens_edit(&mut self, cx: &mut Context<Self>) {
        // Get necessary values
        self.update_field_from_input(cx);
        
        // Parse the max tokens value
        let Ok(max_tokens) = self.edit_max_tokens_value.trim().parse::<usize>() else {
            log::error!("Invalid max tokens value: {}", self.edit_max_tokens_value);
            return;
        };
        
        if max_tokens == 0 {
            log::error!("Max tokens value cannot be zero");
            return;
        }
        
        // Get server ID and model name
        let Some(server_id) = self.editing_model_server_id.clone() else {
            log::error!("No server ID for max tokens edit");
            return;
        };
        
        let Some(model_name) = self.editing_model_name.clone() else {
            log::error!("No model name for max tokens edit");
            return;
        };
        
        log::info!("Updating max tokens for model {} to {}", model_name, max_tokens);
        
        // Get filesystem for settings update
        let fs = <dyn fs::Fs>::global(cx);
        
        // Variables for the closure
        let server_id_clone = server_id.clone();
        let model_name_clone = model_name.clone();
        let new_max_tokens = max_tokens;
        
        // Update settings
        update_settings_file::<crate::AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    // Find the server with matching ID
                    for server in servers.iter_mut() {
                        if server.id == server_id_clone {
                            if let Some(models) = &mut server.available_models {
                                // Find the model with the matching name
                                for model in models.iter_mut() {
                                    if model.name == model_name_clone {
                                        // Get the server default
                                        let server_default = model.server_max_tokens;
                                        
                                                                                            // Log current state before update
                                                    log::info!("Before update: model {}, server_max_tokens={}, custom_max_tokens={:?}", 
                                                        model_name_clone, 
                                                        model.server_max_tokens,
                                                        model.custom_max_tokens);
                                                    
                                                    // Update the custom max tokens
                                                    if new_max_tokens == server_default {
                                                        // If value equals server default, remove the custom value
                                                        model.custom_max_tokens = None;
                                                        log::info!("Reset max tokens to server default for model {}", model_name_clone);
                                                    } else {
                                                        model.custom_max_tokens = Some(new_max_tokens);
                                                        log::info!("Updated custom max tokens for model {} to {}", 
                                                            model_name_clone, new_max_tokens);
                                                    }
                                                    
                                                    // Log state after update
                                                    log::info!("After update: model {}, server_max_tokens={}, custom_max_tokens={:?}", 
                                                        model_name_clone, 
                                                        model.server_max_tokens,
                                                        model.custom_max_tokens);
                                        break;
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        });
        
        // Reset dialog state
        self.is_editing_max_tokens = false;
        self.editing_model_server_id = None;
        self.editing_model_name = None;
        self.edit_max_tokens_input = None;
        
        // Refresh the models
        self.state.update(cx, |state, cx| {
            state.restart_fetch_models_task(cx);
        });
        
        cx.notify();
    }
    
    fn cancel_max_tokens_edit(&mut self, cx: &mut Context<Self>) {
        // Reset state
        self.is_editing_max_tokens = false;
        self.editing_model_server_id = None;
        self.editing_model_name = None;
        self.edit_max_tokens_input = None;
        
        cx.notify();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Create inputs for any active forms at the beginning
        if self.editing_server_index.is_some() {
            self.create_server_edit_inputs(window, cx);
        }
        
        if self.is_adding_server {
            self.create_new_server_inputs(window, cx);
        }
        
        if self.is_adding_model {
            self.create_model_inputs(window, cx);
        }
        
        if self.is_editing_max_tokens {
            self.create_max_tokens_input(window, cx);
        }
        
        // Now get settings and continue with rendering
        let settings = AllLanguageModelSettings::get_global(cx);
        let lmstudio_settings = &settings.lmstudio;
        
        let is_authenticated = self.state.read(cx).is_authenticated();
        let servers = &lmstudio_settings.servers;

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
                // Server management section
                .child(
                    v_flex()
                        .gap_2()
                        .child(Label::new("LM Studio Servers").size(LabelSize::Small))
                        // Add server form
                        .child(
                            if self.is_adding_server {
                                // Add server form with SingleLineInput
                                v_flex()
                                    .gap_2()
                                    .p_2()
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .bg(cx.theme().colors().background)
                                    .rounded_md()
                                    .my_2()
                                    .w_full()
                                    .child(
                                        Label::new("Add New Server")
                                    )
                                    .child(
                                        v_flex()
                                            .gap_2()
                                            .w_full()
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(Label::new("Server Name:").size(LabelSize::Small))
                                                    .child(self.new_server_name_input.clone().unwrap())
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(Label::new("Server URL:").size(LabelSize::Small))
                                                    .child(self.new_server_url_input.clone().unwrap())
                                            )
                                        )
                                        .child(
                                            h_flex()
                                                .justify_end()
                                                .gap_2()
                                                .mt_2()
                                                .child(
                                                    Button::new("cancel-add", "Cancel")
                                                        .style(ButtonStyle::Subtle)
                                                        .on_click(cx.listener(move |this, _, _, cx| {
                                                            this.toggle_add_server_form(cx);
                                                        }))
                                                )
                                                .child(
                                                    Button::new("add-new-server", "Add Server")
                                                        .on_click(cx.listener(move |this, _, _, cx| {
                                                            this.add_new_server(cx);
                                                        }))
                                                )
                                        )
                            } else {
                                div()
                            }
                        )
                        // Server list
                        .child(
                            v_flex()
                                .gap_1()
                                .child(
                                    if self.editing_server_index.is_some() {
                                        // Editing form with SingleLineInput
                                        v_flex()
                                            .gap_2()
                                            .p_2()
                                            .border_1()
                                            .border_color(cx.theme().colors().border)
                                            .bg(cx.theme().colors().background)
                                            .rounded_md()
                                            .w_full()
                                            .child(
                                                Label::new("Edit Server")
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_2()
                                                    .w_full()
                                                    .child(
                                                        v_flex()
                                                            .gap_1()
                                                            .child(Label::new("Server Name:").size(LabelSize::Small))
                                                            .child(self.server_edit_name_input.clone().unwrap())
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .gap_1()
                                                            .child(Label::new("Server URL:").size(LabelSize::Small))
                                                            .child(self.server_edit_url_input.clone().unwrap())
                                                    )
                                            )
                                            .child(
                                                h_flex()
                                                    .justify_end()
                                                    .gap_2()
                                                    .mt_2()
                                                    .child(
                                                        Button::new("cancel-edit", "Cancel")
                                                            .style(ButtonStyle::Subtle)
                                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                                this.cancel_server_edits(cx);
                                                            }))
                                                    )
                                                    .child(
                                                        Button::new("save-edit", "Save")
                                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                                this.save_server_edits(cx);
                                                            }))
                                                    )
                                            )
                                            .into_any_element()
                                    } else if servers.is_empty() {
                                        div()
                                            .p_2()
                                            .border_1()
                                            .child(
                                                Label::new("No servers configured")
                                                    .color(Color::Muted)
                                                    .size(LabelSize::XSmall)
                                            )
                                            .into_any_element()
                                    } else {
                                        List::new()
                                            .children(
                                                servers.iter().enumerate().map(|(idx, server)| {
                                                    ListItem::new(idx)
                                                        .child(
                                                            h_flex()
                                                                .justify_between()
                                                                .w_full()
                                                                .gap_2()
                                                                .child(
                                                                    v_flex()
                                                                        .gap_1()
                                                                        .child(Label::new(&server.name))
                                                                        .child(
                                                                            Label::new(&server.api_url)
                                                                                .color(Color::Muted)
                                                                                .size(LabelSize::XSmall)
                                                                        )
                                                                )
                                                                .child(
                                                                    h_flex()
                                                                        .gap_1()
                                                                        .child(
                                                                            IconButton::new("toggle-server", IconName::Circle)
                                                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                                                    this.toggle_server(idx, cx);
                                                                                }))
                                                                                .icon_color(if server.enabled { Color::Success } else { Color::Muted })
                                                                        )
                                                                        .child(
                                                                            IconButton::new("fetch-models", IconName::Update)
                                                                                .tooltip(Tooltip::text("Fetch Models"))
                                                                                .on_click({
                                                                                    let server_enabled = server.enabled;
                                                                                    let state = self.state.clone();
                                                                                    move |_, _, cx| {
                                                                                        if server_enabled {
                                                                                            // Refresh this specific server's models via the state
                                                                                            state.update(cx, |state, cx| {
                                                                                                state.restart_fetch_models_task(cx);
                                                                                            });
                                                                                        }
                                                                                    }
                                                                                })
                                                                                .icon_color(if server.enabled { Color::Info } else { Color::Muted })
                                                                        )
                                                                        .child(
                                                                            IconButton::new("edit-server", IconName::Pencil)
                                                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                                                    this.edit_server(idx, cx);
                                                                                }))
                                                                                .icon_color(Color::Info)
                                                                        )
                                                                        .child(
                                                                            IconButton::new("remove-server", IconName::Trash)
                                                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                                                    this.remove_server(idx, cx);
                                                                                }))
                                                                                .icon_color(Color::Error)
                                                                        )
                                                                )
                                                        )
                                                        .on_click(cx.listener(move |this, _, _, cx| {
                                                            this.select_server(Some(idx), cx);
                                                        }))
                                                })
                                            )
                                            .into_any_element()
                                    }
                                )
                        )
                        // Add server button
                        .child(
                            Button::new("add-server", "Add Server")
                                .icon(IconName::Plus)
                                .icon_position(IconPosition::Start)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.toggle_add_server_form(cx);
                                }))
                        )
                )
                // Model management section - only show if a server is selected
                .child(
                    v_flex()
                        .gap_2()
                        .map(|this| {
                            if let Some(server_idx) = self.selected_server_index {
                                let settings = AllLanguageModelSettings::get_global(cx);
                                let servers = &settings.lmstudio.servers;
                                
                                if server_idx < servers.len() {
                                    let server = &servers[server_idx];
                                    
                                    // Only show for enabled servers
                                    if server.enabled {
                                        this.child(
                                            div()
                                                .border_t_1()
                                                .border_color(cx.theme().colors().border)
                                                .my_2()
                                        )
                                        .child(
                                            h_flex()
                                                .justify_between()
                                                .child(
                                                    Label::new(format!("Models for {}", server.name))
                                                        .size(LabelSize::Small)
                                                )
                                                .child(
                                                    Button::new("add-model", "Add Model")
                                                        .icon(IconName::Plus)
                                                        .icon_position(IconPosition::Start)
                                                        .on_click(cx.listener(move |this, _, _, cx| {
                                                            this.toggle_add_model_form(cx);
                                                        }))
                                                )
                                        )
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .child(
                                                    v_flex()
                                                        .gap_1()
                                                        .child({
                                                            // Create model display element separately to avoid borrowing issue
                                                            let server_id = server.id.clone();
                                                            self.render_models_for_server(&LmStudioServer {
                                                                id: server_id,
                                                                name: server.name.clone(),
                                                                api_url: server.api_url.clone(),
                                                                enabled: server.enabled,
                                                                available_models: server.available_models.clone(),
                                                            }, cx)
                                                        })
                                                )
                                        )
                                        .child(
                                            if self.is_adding_model {
                                                // Add model form with SingleLineInput
                                                v_flex()
                                                    .gap_2()
                                                    .p_2()
                                                    .border_1()
                                                    .border_color(cx.theme().colors().border)
                                                    .bg(cx.theme().colors().background)
                                                    .rounded_md()
                                                    .my_2()
                                                    .w_full()
                                                    .child(
                                                        Label::new("Add Custom Model")
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .gap_2()
                                                            .w_full()
                                                            .child(
                                                                v_flex()
                                                                    .gap_1()
                                                                    .child(Label::new("Model Name:").size(LabelSize::Small))
                                                                    .child(self.new_model_name_input.clone().unwrap())
                                                            )
                                                            .child(
                                                                v_flex()
                                                                    .gap_1()
                                                                    .child(Label::new("Display Name (optional):").size(LabelSize::Small))
                                                                    .child(self.new_model_display_name_input.clone().unwrap())
                                                            )
                                                            .child(
                                                                v_flex()
                                                                    .gap_1()
                                                                    .child(Label::new("Max Tokens:").size(LabelSize::Small))
                                                                    .child(self.new_model_max_tokens_input.clone().unwrap())
                                                            )
                                                    )
                                                    .child(
                                                        h_flex()
                                                            .justify_end()
                                                            .gap_2()
                                                            .mt_2()
                                                            .child(
                                                                Button::new("cancel-model", "Cancel")
                                                                    .style(ButtonStyle::Subtle)
                                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                                        this.toggle_add_model_form(cx);
                                                                    }))
                                                            )
                                                            .child(
                                                                Button::new("add-new-model", "Add Model")
                                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                                        this.add_custom_model(cx);
                                                                    }))
                                                            )
                                                    )
                                                    .into_any_element()
                                            } else {
                                                div().into_any_element()
                                            }
                                        )
                                    } else {
                                        this.child(
                                            div()
                                                .p_2()
                                                .child(
                                                    Label::new(format!("Server {} is disabled. Enable it to manage models.", server.name))
                                                        .color(Color::Muted)
                                                )
                                        )
                                    }
                                } else {
                                    this
                                }
                            } else {
                                // No server selected
                                this.child(
                                    div()
                                        .p_2()
                                        .child(
                                            Label::new("Select a server to manage models")
                                                .color(Color::Muted)
                                        )
                                )
                            }
                        })
                )
                // Add max tokens editing dialog if active
                .child(
                    if self.is_editing_max_tokens {
                        v_flex()
                            .gap_2()
                            .p_2()
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .bg(cx.theme().colors().background)
                            .rounded_md()
                            .my_2()
                            .w_full()
                            .child(
                                Label::new("Edit Max Tokens")
                            )
                            .child(
                                v_flex()
                                    .gap_2()
                                    .w_full()
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(Label::new("Max Tokens:").size(LabelSize::Small))
                                            .child(self.edit_max_tokens_input.clone().unwrap())
                                    )
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .child(
                                                Label::new("Server default:")
                                                    .size(LabelSize::XSmall)
                                                    .color(Color::Muted)
                                            )
                                            .child(
                                                Label::new(format!("{} tokens", 
                                                    // Find current server default for this model
                                                    self.editing_model_server_id.as_ref().and_then(|server_id| {
                                                        self.editing_model_name.as_ref().and_then(|model_name| {
                                                            let settings = AllLanguageModelSettings::get_global(cx);
                                                            settings.lmstudio.servers.iter()
                                                                .find(|s| &s.id == server_id)
                                                                .and_then(|s| s.available_models.as_ref())
                                                                .and_then(|models| models.iter().find(|m| &m.name == model_name))
                                                                .map(|m| m.server_max_tokens)
                                                        })
                                                    }).unwrap_or(8192)
                                                ))
                                                .size(LabelSize::XSmall)
                                                .color(Color::Default)
                                            )
                                    )
                                    .child(
                                        Label::new("Setting to server default will remove custom value")
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted)
                                    )
                            )
                            .child(
                                h_flex()
                                    .justify_between()
                                    .mt_2()
                                    .child(
                                        Button::new("reset-to-default", "Use Server Default")
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                // Get server default value
                                                let server_default = this.editing_model_server_id.as_ref().and_then(|server_id| {
                                                    this.editing_model_name.as_ref().and_then(|model_name| {
                                                        let settings = AllLanguageModelSettings::get_global(cx);
                                                        settings.lmstudio.servers.iter()
                                                            .find(|s| &s.id == server_id)
                                                            .and_then(|s| s.available_models.as_ref())
                                                            .and_then(|models| models.iter().find(|m| &m.name == model_name))
                                                            .map(|m| m.server_max_tokens)
                                                    })
                                                }).unwrap_or(8192);
                                                
                                                // Just update our internal value - the input will be recreated on next UI refresh
                                                this.edit_max_tokens_value = server_default.to_string();
                                                // Clear the input to force recreation
                                                this.edit_max_tokens_input = None;
                                                
                                                // Save with the default value
                                                this.save_max_tokens_edit(cx);
                                            }))
                                    )
                                    .child(
                                        h_flex()
                                        .gap_2()
                                        .child(
                                            Button::new("cancel-tokens-edit", "Cancel")
                                                .style(ButtonStyle::Subtle)
                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                    this.cancel_max_tokens_edit(cx);
                                                }))
                                        )
                                        .child(
                                            Button::new("save-tokens-edit", "Save")
                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                    this.save_max_tokens_edit(cx);
                                                }))
                                        )
                                    )
                            )
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    }
                )
                .into_any()
        }
    }
}

// Add a method to get default settings for backward compatibility
impl LmStudioSettings {
    pub fn default_with_legacy() -> Self {
        Self {
            servers: Vec::new(),
        }
    }
    
    // Migrate from the old api_url format to the new servers format
    pub fn migrate_from_legacy(legacy_api_url: &str) -> Self {
        let mut settings = Self::default();
        
        // Create a default server with the legacy API URL
        if !legacy_api_url.is_empty() {
            let server = LmStudioServer {
                id: uuid::Uuid::new_v4().to_string(),
                name: "Default LM Studio Server".to_string(),
                api_url: legacy_api_url.to_string(),
                enabled: true,
                available_models: None,
            };
            
            settings.servers.push(server);
        }
        
        settings
    }
    
    // Get the first enabled server or None
    pub fn first_enabled_server(&self) -> Option<&LmStudioServer> {
        self.servers.iter().find(|server| server.enabled)
    }
}

fn default_true() -> bool {
    true
}

fn max_tokens_is_default(max_tokens: &usize) -> bool {
    *max_tokens == 0
}

impl AvailableModel {
    /// Gets the effective max tokens, prioritizing custom value if set
    pub fn effective_max_tokens(&self) -> usize {
        self.custom_max_tokens.unwrap_or(self.server_max_tokens)
    }
    
    /// For backward compatibility when loading older settings
    pub fn migrate_max_tokens(&mut self) {
        if self.max_tokens > 0 && self.custom_max_tokens.is_none() {
            self.custom_max_tokens = Some(self.max_tokens);
        }
    }
}

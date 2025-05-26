use anyhow::Result;
use futures::{StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, Subscription, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelAvailability, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, MessageContent,
    RateLimiter, StopReason,
};
use language_model_repository::HuggingFaceModelRepository;
use llama_cpp_2::token::{LlamaToken, data::LlamaTokenData, data_array::LlamaTokenDataArray};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore, update_settings_file};
use std::path::PathBuf;
use std::sync::Arc;
use ui::{IconName, List, ListItem, Label, LabelSize, Button, ButtonStyle, IconButton, prelude::*, Color, Icon};
use ui_input::SingleLineInput;
use walkdir::WalkDir;
use workspace::AppState;
use uuid;

use crate::AllLanguageModelSettings;

const PROVIDER_ID: &str = "llamacpp";
const PROVIDER_NAME: &str = "Llama.cpp";

/// Settings for the llama.cpp provider
#[derive(Debug, Clone, PartialEq)]
pub struct LlamaCppSettings {
    /// Path to the models directory
    pub models_directory: PathBuf,
    /// Available models configuration
    pub available_models: Vec<AvailableModel>,
    /// Custom model configurations
    pub model_configurations: Vec<ModelConfiguration>,
    /// Number of GPU layers to use (0 = CPU only)
    pub gpu_layers: u32,
    /// Number of threads to use
    pub thread_count: Option<usize>,
    /// Default temperature for text generation (0.0 to 2.0)
    pub default_temperature: f32,
    /// Default context size for models (in tokens)
    pub default_context_size: usize,
    /// Default maximum tokens to generate
    pub default_max_tokens: usize,
    /// Top-k sampling parameter (0 = disabled)
    pub default_top_k: i32,
    /// Top-p (nucleus) sampling parameter (0.0 to 1.0)
    pub default_top_p: f32,
    /// Repetition penalty (1.0 = no penalty, >1.0 = penalty)
    pub default_repetition_penalty: f32,
    /// Number of tokens to look back for repetition penalty
    pub default_repetition_penalty_window: usize,
}

impl Default for LlamaCppSettings {
    fn default() -> Self {
        Self {
            models_directory: PathBuf::from("~/.cache/zed/models"),
            available_models: Vec::new(),
            model_configurations: vec![ModelConfiguration::default()],
            gpu_layers: 0,
            thread_count: None,
            default_temperature: 0.3,
            default_context_size: 4096,
            default_max_tokens: 1000,
            default_top_k: 40,
            default_top_p: 0.95,
            default_repetition_penalty: 1.1,
            default_repetition_penalty_window: 64,
        }
    }
}

/// Configuration for an available model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    /// Path to the model file (relative to models_directory)
    pub path: String,
    /// Display name for the model
    pub display_name: String,
    /// Model type/architecture
    pub model_type: String,
    /// Context size (in tokens)
    pub context_size: usize,
    /// Whether this model supports function calling
    pub supports_tools: bool,
    /// Special capabilities of the model
    pub capabilities: Vec<String>,
}

/// Custom model configuration with specific parameters
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ModelConfiguration {
    /// Unique ID for this configuration
    pub id: String,
    /// Display name for this configuration
    pub name: String,
    /// Description of this configuration
    pub description: Option<String>,
    /// Temperature for text generation (0.0 to 2.0)
    pub temperature: f32,
    /// Context size for models (in tokens)
    pub context_size: usize,
    /// Maximum tokens to generate
    pub max_tokens: usize,
    /// Top-k sampling parameter (0 = disabled)
    pub top_k: i32,
    /// Top-p (nucleus) sampling parameter (0.0 to 1.0)
    pub top_p: f32,
    /// Repetition penalty (1.0 = no penalty, >1.0 = penalty)
    pub repetition_penalty: f32,
    /// Number of tokens to look back for repetition penalty
    pub repetition_penalty_window: usize,
}

impl Default for ModelConfiguration {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Configuration".to_string(),
            description: None,
            temperature: 0.3,
            context_size: 4096,
            max_tokens: 1000,
            top_k: 40,
            top_p: 0.95,
            repetition_penalty: 1.1,
            repetition_penalty_window: 64,
        }
    }
}

pub struct State {
    /// Path to models directory
    models_directory: PathBuf,
    /// Available models
    available_models: Vec<ModelInfo>,
    /// Subscription to settings changes
    _subscription: Subscription,
}

/// Information about a model
#[derive(Clone, Debug)]
struct ModelInfo {
    /// Model ID
    id: String,
    /// Model path
    path: PathBuf,
    /// Display name
    display_name: String,
    /// Model type
    #[allow(dead_code)]
    model_type: String,
    /// Context size
    context_size: usize,
    /// Supports tools
    supports_tools: bool,
    /// Model capabilities
    #[allow(dead_code)]
    capabilities: Vec<String>,
}

impl State {
    fn new(cx: &mut Context<Self>) -> Self {
        let subscription = cx.observe_global::<SettingsStore>({
            let mut settings = AllLanguageModelSettings::get_global(cx).llamacpp.clone();
            move |this: &mut State, cx| {
                let new_settings = &AllLanguageModelSettings::get_global(cx).llamacpp;
                if &settings != new_settings {
                    settings = new_settings.clone();
                    this.update_models_directory(new_settings.models_directory.clone());
                    this.scan_for_models(cx);
                    cx.notify();
                }
            }
        });

        let settings = &AllLanguageModelSettings::get_global(cx).llamacpp;
        let mut state = Self {
            models_directory: settings.models_directory.clone(),
            available_models: Vec::new(),
            _subscription: subscription,
        };
        
        // Scan for available models
        state.scan_for_models(cx);
        
        state
    }

    fn update_models_directory(&mut self, directory: PathBuf) {
        self.models_directory = directory;
    }

    fn scan_for_models(&mut self, _cx: &mut Context<Self>) {
        self.available_models.clear();

        if !self.models_directory.exists() {
            return;
        }

        // Recursively scan for GGUF files
        for entry in WalkDir::new(&self.models_directory)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Only process files, not directories
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "gguf" {
                        if let Some(file_name) = path.file_stem() {
                            // Create a unique ID that includes the relative path
                            let relative_path = path.strip_prefix(&self.models_directory)
                                .unwrap_or(path);
                            
                            // Use the relative path without extension as the ID
                            let id = relative_path.with_extension("").display().to_string();
                            
                            // Use just the filename for display, but include parent dir if not in root
                            let display_name = if let Some(parent) = relative_path.parent() {
                                if parent == std::path::Path::new("") {
                                    // File is in root directory, just use filename
                                    file_name.to_string_lossy().to_string()
                                } else {
                                    // File is in subdirectory, include parent path
                                    format!("{}/{}", parent.display(), file_name.to_string_lossy())
                                }
                            } else {
                                file_name.to_string_lossy().to_string()
                            };
                            
                            let info = ModelInfo {
                                id: id.clone(),
                                path: path.to_path_buf(),
                                display_name,
                                model_type: "gguf".to_string(),
                                context_size: 4096, // Default, will be read from model
                                supports_tools: false, // Llama.cpp doesn't support function calling yet
                                capabilities: vec!["text-generation".to_string()],
                            };
                            self.available_models.push(info);
                        }
                    }
                }
            }
        }
        
        // Sort models by display name for consistent ordering
        self.available_models.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    }

    fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
    }
}

pub struct LlamaCppLanguageModelProvider {
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
}

impl LlamaCppLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        Self {
            state: cx.new(State::new),
            http_client,
        }
    }
}

impl LanguageModelProviderState for LlamaCppLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for LlamaCppLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::FileCode
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.provided_models(cx).into_iter().next()
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.default_model(cx)
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        
        state
            .available_models
            .iter()
            .map(|model_info| {
                Arc::new(LlamaCppLanguageModel {
                    id: LanguageModelId::from(model_info.id.clone()),
                    info: model_info.clone(),
                    state: self.state.clone(),
                    #[allow(dead_code)]
                    request_limiter: RateLimiter::new(1), // Single request at a time for local models
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, _cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        // Local models don't need authentication
        Task::ready(Ok(()))
    }

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        let state = self.state.clone();
        cx.new(|cx| ConfigurationView::new(state, self.http_client.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, _cx: &mut App) -> Task<Result<()>> {
        // Local models don't have credentials
        Task::ready(Ok(()))
    }
}

struct LlamaCppLanguageModel {
    id: LanguageModelId,
    info: ModelInfo,
    #[allow(dead_code)]
    state: gpui::Entity<State>,
    #[allow(dead_code)]
    request_limiter: RateLimiter,
}

impl LanguageModel for LlamaCppLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.info.display_name.clone())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn availability(&self) -> LanguageModelAvailability {
        LanguageModelAvailability::Public
    }

    fn supports_tools(&self) -> bool {
        self.info.supports_tools
    }

    fn supports_images(&self) -> bool {
        false
    }

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        false
    }

    fn telemetry_id(&self) -> String {
        format!("llamacpp/{}", self.info.id)
    }

    fn max_token_count(&self) -> usize {
        self.info.context_size
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        // Simple token estimation for now
        // TODO: Implement proper tokenization using llama.cpp tokenizer
        let token_count = request
            .messages
            .iter()
            .map(|msg| {
                msg.content.iter().map(|content| match content {
                    MessageContent::Text(text) => {
                        // Simple estimation: ~4 characters per token
                        text.chars().count() / 4
                    }
                    MessageContent::Image(_) => 0,
                    MessageContent::ToolUse(tool_use) => {
                        tool_use.input.to_string().chars().count() / 4
                    }
                    MessageContent::Thinking { .. } => 0,
                    MessageContent::RedactedThinking(_) => 0,
                    MessageContent::ToolResult(_) => 0,
                }).sum::<usize>()
            })
            .sum();
            
        Box::pin(async move { Ok(token_count) })
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
        let model_path = self.info.path.clone();
        let context_size = self.info.context_size;
        
        let Ok(settings) = cx.read_entity(&self.state, |_state, cx| {
            AllLanguageModelSettings::get_global(cx).llamacpp.clone()
        }) else {
            return Box::pin(async { Err(anyhow::anyhow!("App state dropped")) });
        };
        
        // Create inference configuration
        let config = InferenceConfig {
            temperature: settings.default_temperature,
            max_tokens: settings.default_max_tokens,
            top_k: settings.default_top_k,
            top_p: settings.default_top_p,
            repetition_penalty: settings.default_repetition_penalty,
            repetition_penalty_window: settings.default_repetition_penalty_window,
        };
        
        Box::pin(async move {
            // Create channels for communication with the llama.cpp thread
            let (tx, rx) = futures::channel::mpsc::unbounded();
            
            // Spawn a dedicated thread for llama.cpp operations
            // This avoids the Send/Sync constraints
            std::thread::spawn(move || {
                // Initialize llama.cpp in this thread
                if let Err(e) = run_llama_inference(model_path, context_size, request, config, tx) {
                    log::error!("Llama.cpp inference error: {:?}", e);
                }
            });
            
            // Convert the receiver into a stream
            let stream = rx.boxed();
            Ok(stream)
        })
    }
}

// Configuration for inference
#[derive(Clone, Debug)]
struct InferenceConfig {
    pub temperature: f32,
    pub max_tokens: usize,
    pub top_k: i32,
    pub top_p: f32,
    pub repetition_penalty: f32,
    pub repetition_penalty_window: usize,
}

// Helper function that runs in a dedicated thread
fn run_llama_inference(
    model_path: PathBuf,
    context_size: usize,
    request: LanguageModelRequest,
    config: InferenceConfig,
    tx: futures::channel::mpsc::UnboundedSender<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
) -> Result<()> {
    use llama_cpp_2::{
        context::params::LlamaContextParams,
        llama_backend::LlamaBackend,
        model::{LlamaModel, params::LlamaModelParams, AddBos, Special},
        token::{LlamaToken, data::LlamaTokenData, data_array::LlamaTokenDataArray},
        llama_batch::LlamaBatch,
    };
    
    // Initialize the backend
    let backend = LlamaBackend::init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize backend: {:?}", e))?;
    
    // Load the model
    let model_params = LlamaModelParams::default();
    let model = LlamaModel::load_from_file(&backend, &model_path, &model_params)
        .map_err(|e| anyhow::anyhow!("Failed to load model from {}: {:?}", model_path.display(), e))?;
    
    // Create context
    let ctx_params = LlamaContextParams::default()
        .with_n_ctx(std::num::NonZeroU32::new(context_size as u32));
    
    let mut ctx = model
        .new_context(&backend, ctx_params)
        .map_err(|e| anyhow::anyhow!("Failed to create context: {:?}", e))?;
    
    // Convert messages to prompt using a more standard chat format
    let mut prompt_parts = Vec::new();
    
    // Add system message if present
    let system_messages: Vec<_> = request.messages.iter()
        .filter(|msg| matches!(msg.role, language_model::Role::System))
        .collect();
    
    if !system_messages.is_empty() {
        let system_content = system_messages.iter()
            .flat_map(|msg| msg.content.iter())
            .filter_map(|c| match c {
                MessageContent::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        
        if !system_content.is_empty() {
            prompt_parts.push(format!("System: {}", system_content));
        }
    }
    
    // Add conversation history
    for msg in &request.messages {
        if matches!(msg.role, language_model::Role::System) {
            continue; // Already handled above
        }
        
        let role = match msg.role {
            language_model::Role::User => "Human",
            language_model::Role::Assistant => "Assistant",
            language_model::Role::System => "System", // Won't reach here
        };
        
        let content = msg.content.iter()
            .filter_map(|c| match c {
                MessageContent::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        
        if !content.is_empty() {
            prompt_parts.push(format!("{}: {}", role, content));
        }
    }
    
    // Create the full prompt with proper formatting
    let full_prompt = if prompt_parts.is_empty() {
        "Human: Hello\n\nAssistant:".to_string()
    } else {
        format!("{}\n\nAssistant:", prompt_parts.join("\n\n"))
    };
    
    // Debug: log the prompt being used
    log::info!("Llama.cpp prompt: {}", full_prompt);
    
    // Tokenize the prompt
    let tokens_list = model.str_to_token(&full_prompt, AddBos::Always)
        .map_err(|e| anyhow::anyhow!("Failed to tokenize prompt: {:?}", e))?;
    
    if tokens_list.is_empty() {
        let _ = tx.unbounded_send(Err(LanguageModelCompletionError::Other(
            anyhow::anyhow!("Empty tokenization result")
        )));
        return Ok(());
    }
    
    // Create batch for processing
    let mut batch = LlamaBatch::new(context_size, 1);
    
    // Process the prompt tokens
    for (i, &token) in tokens_list.iter().enumerate() {
        let is_last = i == tokens_list.len() - 1;
        batch.add(token, i as i32, &[0], is_last)
            .map_err(|e| anyhow::anyhow!("Failed to add token to batch: {:?}", e))?;
    }
    
    // Decode the prompt
    ctx.decode(&mut batch)
        .map_err(|e| anyhow::anyhow!("Failed to decode prompt: {:?}", e))?;
    
    // Generation parameters
    let max_tokens = config.max_tokens;
    let temperature = request.temperature.unwrap_or(config.temperature);
    
    // Track current position in the sequence
    let mut n_past = tokens_list.len() as i32;
    
    // Buffer for accumulating partial UTF-8 sequences
    let mut token_buffer = Vec::new();
    
    // Track recent tokens for repetition penalty
    let mut recent_tokens = Vec::new();
    
    // Generation loop
    for generation_step in 0..max_tokens {
        // Get logits from the appropriate position
        let logits = if generation_step == 0 {
            // First generation step: logits are at the last prompt position
            ctx.get_logits_ith((tokens_list.len() - 1) as i32)
        } else {
            // Subsequent steps: logits are at position 0 (single token batch)
            ctx.get_logits_ith(0)
        };
        let n_vocab = model.n_vocab() as usize;
        
        // Create candidates array
        let mut candidates = Vec::with_capacity(n_vocab);
        for token_id in 0..n_vocab {
            candidates.push(LlamaTokenData::new(
                LlamaToken(token_id as i32),
                logits[token_id],
                0.0,
            ));
        }
        
        let mut candidates_p = LlamaTokenDataArray::from_iter(candidates, false);
        
        // Apply repetition penalty to recent tokens
        let repetition_penalty = config.repetition_penalty;
        let penalty_window = config.repetition_penalty_window;
        
        for &recent_token in recent_tokens.iter().rev().take(penalty_window) {
            for candidate in candidates_p.data.iter_mut() {
                if candidate.id() == recent_token {
                    let new_logit = if candidate.logit() > 0.0 {
                        candidate.logit() / repetition_penalty
                    } else {
                        candidate.logit() * repetition_penalty
                    };
                    *candidate = LlamaTokenData::new(candidate.id(), new_logit, candidate.p());
                }
            }
        }
        
        // Use proper probabilistic sampling with temperature, top-k, and top-p
        let new_token_id = sample_token_probabilistic(&mut candidates_p, temperature, config.top_k, config.top_p);
        
        // Check for EOS
        if new_token_id == model.token_eos() {
            break;
        }
        
        // Add token to recent tokens for repetition penalty
        recent_tokens.push(new_token_id);
        if recent_tokens.len() > 128 {
            recent_tokens.remove(0); // Keep only recent 128 tokens
        }
        
        // Convert token to string with proper UTF-8 handling
        match model.token_to_str(new_token_id, Special::Tokenize) {
            Ok(token_str) => {
                // Token decoded successfully, send it
                log::debug!("Generated token: {:?} -> '{}'", new_token_id.0, token_str);
                if tx.unbounded_send(Ok(LanguageModelCompletionEvent::Text(token_str))).is_err() {
                    // Receiver dropped, stop generation
                    break;
                }
            }
            Err(_) => {
                // Token decoding failed, likely a partial UTF-8 sequence
                // Add token to buffer and try to decode accumulated bytes
                token_buffer.push(new_token_id);
                
                // Try to decode the accumulated tokens as a complete string
                let mut accumulated_bytes = Vec::new();
                for &buffered_token in &token_buffer {
                    if let Ok(token_str) = model.token_to_str(buffered_token, Special::Tokenize) {
                        accumulated_bytes.extend_from_slice(token_str.as_bytes());
                    }
                }
                
                // Try to convert accumulated bytes to UTF-8
                match String::from_utf8(accumulated_bytes) {
                    Ok(complete_str) => {
                        // Successfully decoded, send the complete string and clear buffer
                        if tx.unbounded_send(Ok(LanguageModelCompletionEvent::Text(complete_str))).is_err() {
                            break;
                        }
                        token_buffer.clear();
                    }
                    Err(_) => {
                        // Still not valid UTF-8, continue accumulating
                        // But limit buffer size to prevent memory issues
                        if token_buffer.len() > 10 {
                            // Too many tokens in buffer, something is wrong
                            let _ = tx.unbounded_send(Err(LanguageModelCompletionError::Other(
                                anyhow::anyhow!("Failed to decode accumulated tokens as UTF-8")
                            )));
                            break;
                        }
                    }
                }
            }
        }
        
        // Prepare for next token
        batch.clear();
        batch.add(new_token_id, n_past, &[0], true)
            .map_err(|e| anyhow::anyhow!("Failed to add next token: {:?}", e))?;
        
        // Decode the new token
        ctx.decode(&mut batch)
            .map_err(|e| anyhow::anyhow!("Failed to decode next token: {:?}", e))?;
        
        // Increment position
        n_past += 1;
    }
    
    // Flush any remaining tokens in the buffer
    if !token_buffer.is_empty() {
        let mut accumulated_bytes = Vec::new();
        for &buffered_token in &token_buffer {
            if let Ok(token_str) = model.token_to_str(buffered_token, Special::Tokenize) {
                accumulated_bytes.extend_from_slice(token_str.as_bytes());
            }
        }
        
        // Try to send whatever we can decode, even if it's not perfect UTF-8
        match String::from_utf8(accumulated_bytes.clone()) {
            Ok(complete_str) => {
                let _ = tx.unbounded_send(Ok(LanguageModelCompletionEvent::Text(complete_str)));
            }
            Err(_) => {
                // Use lossy conversion as last resort
                let lossy_str = String::from_utf8_lossy(&accumulated_bytes);
                let _ = tx.unbounded_send(Ok(LanguageModelCompletionEvent::Text(lossy_str.to_string())));
            }
        }
    }
    
    // Send completion event
    let _ = tx.unbounded_send(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
    
    Ok(())
}

// Proper probabilistic sampling function
fn sample_token_probabilistic(
    candidates: &mut LlamaTokenDataArray,
    temperature: f32,
    top_k: i32,
    top_p: f32,
) -> LlamaToken {
    use rand::Rng;
    
    if temperature <= 0.0 {
        // Greedy sampling
        return candidates.data.iter()
            .max_by(|a, b| a.logit().partial_cmp(&b.logit()).unwrap_or(std::cmp::Ordering::Equal))
            .map(|token_data| token_data.id())
            .unwrap_or(LlamaToken(0));
    }
    
    // Apply temperature
    for candidate in candidates.data.iter_mut() {
        *candidate = LlamaTokenData::new(
            candidate.id(),
            candidate.logit() / temperature,
            candidate.p(),
        );
    }
    
    // Sort by logit (descending)
    candidates.data.sort_by(|a, b| b.logit().partial_cmp(&a.logit()).unwrap_or(std::cmp::Ordering::Equal));
    
    // Apply top-k filtering
    let k = if top_k > 0 && (top_k as usize) < candidates.data.len() {
        top_k as usize
    } else {
        candidates.data.len()
    };
    candidates.data.truncate(k);
    
    // Convert logits to probabilities using softmax
    let max_logit = candidates.data[0].logit();
    let mut sum = 0.0f32;
    let mut probs = Vec::with_capacity(candidates.data.len());
    
    for candidate in &candidates.data {
        let prob = (candidate.logit() - max_logit).exp();
        probs.push(prob);
        sum += prob;
    }
    
    // Normalize probabilities
    for prob in &mut probs {
        *prob /= sum;
    }
    
    // Apply top-p (nucleus) sampling
    if top_p < 1.0 {
        let mut cumsum = 0.0f32;
        let mut cutoff = probs.len();
        
        for (i, &prob) in probs.iter().enumerate() {
            cumsum += prob;
            if cumsum >= top_p {
                cutoff = i + 1;
                break;
            }
        }
        
        probs.truncate(cutoff);
        candidates.data.truncate(cutoff);
        
        // Renormalize after top-p filtering
        let new_sum: f32 = probs.iter().sum();
        if new_sum > 0.0 {
            for prob in &mut probs {
                *prob /= new_sum;
            }
        }
    }
    
    // Sample from the probability distribution
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0..1.0);
    let mut cumsum = 0.0f32;
    
    for (i, &prob) in probs.iter().enumerate() {
        cumsum += prob;
        if cumsum >= r {
            return candidates.data[i].id();
        }
    }
    
    // Fallback to last token
    candidates.data.last().map(|t| t.id()).unwrap_or(LlamaToken(0))
}

struct ConfigurationView {
    state: gpui::Entity<State>,
    directory_input: Option<Entity<SingleLineInput>>,
    new_directory_path: String,
    is_editing_directory: bool,
    model_config_window: Option<Entity<ModelConfigurationWindow>>,
    model_repository: Option<Entity<HuggingFaceModelRepository>>,
    http_client: Arc<dyn HttpClient>,
}

impl ConfigurationView {
    pub fn new(state: gpui::Entity<State>, http_client: Arc<dyn HttpClient>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let current_path = state.read(_cx).models_directory.to_string_lossy().to_string();
        Self {
            state,
            directory_input: None,
            new_directory_path: current_path,
            is_editing_directory: false,
            model_config_window: None,
            model_repository: None,
            http_client,
        }
    }

    fn create_directory_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let current_path = self.state.read(cx).models_directory.to_string_lossy().to_string();
        self.new_directory_path = current_path.clone();
        
        let input = cx.new(|cx| {
            SingleLineInput::new(window, cx, "Models directory")
        });
        
        // Set the initial text
        input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(current_path, window, cx);
            });
        });
        
        self.directory_input = Some(input);
    }

    fn update_directory_from_input(&mut self, cx: &mut Context<Self>) {
        if let Some(input) = &self.directory_input {
            self.new_directory_path = input.read(cx).editor.read(cx).text(cx).to_string();
        }
    }

    fn start_edit_directory(&mut self, _cx: &mut Context<Self>) {
        self.is_editing_directory = true;
        self.directory_input = None;
    }

    fn cancel_edit_directory(&mut self, _cx: &mut Context<Self>) {
        self.is_editing_directory = false;
        self.directory_input = None;
    }

    fn save_directory(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let new_path = PathBuf::from(&self.new_directory_path);
        
        // Create directory if it doesn't exist
        if !new_path.exists() {
            std::fs::create_dir_all(&new_path)?;
        }
        
        // Get the file system instance from AppState
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        
        // Update settings
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _cx| {
            settings.llamacpp = Some(crate::settings::LlamaCppSettingsContent {
                models_directory: Some(new_path.to_string_lossy().to_string()),
                available_models: None,
                model_configurations: None,
                gpu_layers: None,
                thread_count: None,
                default_temperature: None,
                default_context_size: None,
                default_max_tokens: None,
                default_top_k: None,
                default_top_p: None,
                default_repetition_penalty: None,
                default_repetition_penalty_window: None,
            });
        });
        
        self.is_editing_directory = false;
        self.directory_input = None;
        
        Ok(())
    }

    fn open_model_configuration_window(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let config_window = cx.new(|cx| ModelConfigurationWindow::new(window, cx));
        self.model_config_window = Some(config_window);
    }

    fn open_model_repository(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let models_directory = self.state.read(cx).models_directory.clone();
        let repository = cx.new(|cx| HuggingFaceModelRepository::new(self.http_client.clone(), models_directory, window, cx));
        self.model_repository = Some(repository);
    }

    fn open_model_repository_window(&mut self, cx: &mut Context<Self>) {
        let models_directory = self.state.read(cx).models_directory.clone();
        let http_client = self.http_client.clone();
        
        if let Ok(_window) = cx.open_window(
            gpui::WindowOptions::default(),
            |window, cx| {
                cx.new(|cx| HuggingFaceModelRepository::new(http_client, models_directory, window, cx))
            },
        ) {
            log::info!("Opened model repository in new window");
        } else {
            log::error!("Failed to open model repository window");
        }
    }

    #[allow(dead_code)]
    fn close_model_repository(&mut self, cx: &mut Context<Self>) {
        self.model_repository = None;
        cx.notify();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let models_directory = self.state.read(cx).models_directory.to_string_lossy().to_string();
        let available_models = self.state.read(cx).available_models.clone();
        let model_count = available_models.len();
        
        div()
            .p_4()
            .size_full()
            .child(
                div()
                    .mb_4()
                    .child(
                        Label::new("Llama.cpp Configuration")
                            .size(LabelSize::Large),
                    ),
            )
            .child(
                div()
                    .mb_4()
                    .child(Label::new("Models Directory"))
                    .child(
                        if self.is_editing_directory {
                            div()
                                .flex()
                                .gap_2()
                                .child(
                                    div()
                                        .flex_1()
                                        .child({
                                            if self.directory_input.is_none() {
                                                self.create_directory_input(window, cx);
                                            }
                                            self.directory_input.clone().unwrap()
                                        }),
                                )
                                .child(
                                    Button::new("save", "Save")
                                        .style(ButtonStyle::Filled)
                                        .on_click({
                                            let this = cx.entity().clone();
                                            move |_, _window, cx| {
                                                this.update(cx, |view, cx| {
                                                    view.update_directory_from_input(cx);
                                                    if let Err(e) = view.save_directory(cx) {
                                                        log::error!("Failed to save directory: {}", e);
                                                    }
                                                });
                                            }
                                        }),
                                )
                                .child(
                                    Button::new("cancel", "Cancel")
                                        .style(ButtonStyle::Subtle)
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.cancel_edit_directory(cx);
                                        })),
                                )
                        } else {
                            div()
                                .flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    div()
                                        .flex_1()
                                        .text_color(cx.theme().colors().text_muted)
                                        .child(models_directory),
                                )
                                .child(
                                    IconButton::new("edit", IconName::Pencil)
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.start_edit_directory(cx);
                                        })),
                                )
                        }
                    ),
            )
            .child(
                div()
                    .mb_2()
                    .child(
                        Label::new(format!("Available Models ({})", model_count))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                List::new()
                    .empty_message("No models found. Add GGUF files to the models directory.")
                    .children(
                        available_models
                            .into_iter()
                            .map(|model| {
                                ListItem::new(gpui::SharedString::from(model.id.clone()))
                                    .start_slot(Icon::new(IconName::FileCode))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .child(Label::new(model.display_name))
                                            .child(
                                                Label::new(format!(
                                                    "Context: {} tokens",
                                                    model.context_size
                                                ))
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                            ),
                                    )
                            }),
                    ),
            )
            .child(
                div()
                    .mt_6()
                    .mb_4()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        Label::new("Model Parameters")
                            .size(LabelSize::Large),
                    )
                    .child(
                        IconButton::new("model_config", IconName::Settings)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.open_model_configuration_window(window, cx);
                            }))
                    )
            )
            .child(
                div()
                    .mb_4()
                    .child(
                        Label::new("Click the gear icon to configure custom model parameters for different use cases.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                div()
                    .mt_6()
                    .mb_4()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        Label::new("Model Repository")
                            .size(LabelSize::Large),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                IconButton::new("model_repo", IconName::Download)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_model_repository(window, cx);
                                    }))
                            )
                            .child(
                                Button::new("test_window", "Test New Window")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.open_model_repository_window(cx);
                                    }))
                            )
                    )
            )
            .child(
                div()
                    .mb_4()
                    .child(
                        Label::new("Download models from Hugging Face and other repositories.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .when_some(self.model_config_window.clone(), |this, config_window| {
                this.child(config_window)
            })
            .when_some(self.model_repository.clone(), |this, repository| {
                this.child(repository)
            })
    }
}

struct ModelConfigurationWindow {
    configurations: Vec<ModelConfiguration>,
    selected_config_id: Option<String>,
    is_editing: bool,
    form: Option<Entity<ModelConfigurationForm>>,
}

impl ModelConfigurationWindow {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = AllLanguageModelSettings::get_global(cx);
        Self {
            configurations: settings.llamacpp.model_configurations.clone(),
            selected_config_id: None,
            is_editing: false,
            form: None,
        }
    }

    fn add_new_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let new_config = ModelConfiguration::default();
        self.selected_config_id = Some(new_config.id.clone());
        self.is_editing = true;
        let window_entity = cx.entity().clone();
        self.form = Some(cx.new(|cx| ModelConfigurationForm::new(new_config, window_entity, window, cx)));
    }

    fn edit_configuration(&mut self, config_id: String, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(config) = self.configurations.iter().find(|c| c.id == config_id).cloned() {
            self.selected_config_id = Some(config_id);
            self.is_editing = true;
            let window_entity = cx.entity().clone();
            self.form = Some(cx.new(|cx| ModelConfigurationForm::new(config, window_entity, window, cx)));
        }
    }

    fn delete_configuration(&mut self, config_id: String, cx: &mut Context<Self>) {
        self.configurations.retain(|c| c.id != config_id);
        self.save_configurations(cx);
    }

    fn save_configuration(&mut self, config: ModelConfiguration, cx: &mut Context<Self>) {
        if let Some(index) = self.configurations.iter().position(|c| c.id == config.id) {
            self.configurations[index] = config;
        } else {
            self.configurations.push(config);
        }
        self.save_configurations(cx);
        self.is_editing = false;
        self.form = None;
        cx.notify();
    }

    fn cancel_editing(&mut self, cx: &mut Context<Self>) {
        self.is_editing = false;
        self.form = None;
        self.selected_config_id = None;
        cx.notify();
    }

    fn save_configurations(&self, cx: &mut Context<Self>) {
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        let configurations = self.configurations.clone();
        
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _cx| {
            settings.llamacpp = Some(crate::settings::LlamaCppSettingsContent {
                models_directory: None,
                available_models: None,
                model_configurations: Some(configurations),
                gpu_layers: None,
                thread_count: None,
                default_temperature: None,
                default_context_size: None,
                default_max_tokens: None,
                default_top_k: None,
                default_top_p: None,
                default_repetition_penalty: None,
                default_repetition_penalty_window: None,
            });
        });
    }
}

impl Render for ModelConfigurationWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_4()
            .size_full()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb_4()
                    .child(
                        Label::new("Model Configurations")
                            .size(LabelSize::Large)
                    )
                    .child(
                        Button::new("add_config", "Add Configuration")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.add_new_configuration(_window, cx);
                            }))
                    )
            )
            .child(
                if self.is_editing && self.form.is_some() {
                    self.form.clone().unwrap().into_any_element()
                } else {
                    div()
                        .child(
                            List::new()
                                .empty_message("No model configurations. Click 'Add Configuration' to create one.")
                                .children(
                                    self.configurations
                                        .iter()
                                        .map(|config| {
                                            ListItem::new(gpui::SharedString::from(config.id.clone()))
                                                .start_slot(Icon::new(IconName::Settings))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .justify_between()
                                                        .items_center()
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .gap_1()
                                                                .child(Label::new(config.name.clone()))
                                                                .when_some(config.description.as_ref(), |this, desc| {
                                                                    this.child(
                                                                        Label::new(desc.clone())
                                                                            .size(LabelSize::Small)
                                                                            .color(Color::Muted)
                                                                    )
                                                                })
                                                                .child(
                                                                    Label::new(format!(
                                                                        "Temp: {:.1}, Max Tokens: {}, Top-K: {}, Top-P: {:.2}",
                                                                        config.temperature,
                                                                        config.max_tokens,
                                                                        config.top_k,
                                                                        config.top_p
                                                                    ))
                                                                    .size(LabelSize::Small)
                                                                    .color(Color::Muted)
                                                                )
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .gap_2()
                                                                .child(
                                                                    IconButton::new("edit", IconName::Pencil)
                                                                        .on_click({
                                                                            let config_id = config.id.clone();
                                                                            cx.listener(move |this, _, _window, cx| {
                                                                                this.edit_configuration(config_id.clone(), _window, cx);
                                                                            })
                                                                        })
                                                                )
                                                                .child(
                                                                    IconButton::new("delete", IconName::Trash)
                                                                        .on_click({
                                                                            let config_id = config.id.clone();
                                                                            cx.listener(move |this, _, _window, cx| {
                                                                                this.delete_configuration(config_id.clone(), cx);
                                                                            })
                                                                        })
                                                                )
                                                        )
                                                )
                                        })
                                )
                        )
                        .into_any_element()
                }
            )
    }
}

struct ModelConfigurationForm {
    config: ModelConfiguration,
    parent_window: gpui::Entity<ModelConfigurationWindow>,
    name_input: Entity<SingleLineInput>,
    description_input: Entity<SingleLineInput>,
    temperature_input: Entity<SingleLineInput>,
    context_size_input: Entity<SingleLineInput>,
    max_tokens_input: Entity<SingleLineInput>,
    top_k_input: Entity<SingleLineInput>,
    top_p_input: Entity<SingleLineInput>,
    repetition_penalty_input: Entity<SingleLineInput>,
    repetition_penalty_window_input: Entity<SingleLineInput>,
}

impl ModelConfigurationForm {
    pub fn new(config: ModelConfiguration, parent_window: gpui::Entity<ModelConfigurationWindow>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name_input = cx.new(|cx| SingleLineInput::new(window, cx, "Configuration Name"));
        let description_input = cx.new(|cx| SingleLineInput::new(window, cx, "Description (optional)"));
        let temperature_input = cx.new(|cx| SingleLineInput::new(window, cx, "Temperature"));
        let context_size_input = cx.new(|cx| SingleLineInput::new(window, cx, "Context Size"));
        let max_tokens_input = cx.new(|cx| SingleLineInput::new(window, cx, "Max Tokens"));
        let top_k_input = cx.new(|cx| SingleLineInput::new(window, cx, "Top-K"));
        let top_p_input = cx.new(|cx| SingleLineInput::new(window, cx, "Top-P"));
        let repetition_penalty_input = cx.new(|cx| SingleLineInput::new(window, cx, "Repetition Penalty"));
        let repetition_penalty_window_input = cx.new(|cx| SingleLineInput::new(window, cx, "Penalty Window"));

        // Set initial values
        name_input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(config.name.clone(), window, cx);
            });
        });

        if let Some(description) = &config.description {
            description_input.update(cx, |input, cx| {
                input.editor.update(cx, |editor, cx| {
                    editor.set_text(description.clone(), window, cx);
                });
            });
        }

        temperature_input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(config.temperature.to_string(), window, cx);
            });
        });

        context_size_input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(config.context_size.to_string(), window, cx);
            });
        });

        max_tokens_input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(config.max_tokens.to_string(), window, cx);
            });
        });

        top_k_input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(config.top_k.to_string(), window, cx);
            });
        });

        top_p_input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(config.top_p.to_string(), window, cx);
            });
        });

        repetition_penalty_input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(config.repetition_penalty.to_string(), window, cx);
            });
        });

        repetition_penalty_window_input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text(config.repetition_penalty_window.to_string(), window, cx);
            });
        });

        Self {
            config,
            parent_window,
            name_input,
            description_input,
            temperature_input,
            context_size_input,
            max_tokens_input,
            top_k_input,
            top_p_input,
            repetition_penalty_input,
            repetition_penalty_window_input,
        }
    }

    fn collect_form_data(&self, cx: &Context<Self>) -> Result<ModelConfiguration> {
        let name = self.name_input.read(cx).editor.read(cx).text(cx).to_string();
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Configuration name cannot be empty"));
        }

        let description = self.description_input.read(cx).editor.read(cx).text(cx).to_string();
        let description = if description.trim().is_empty() { None } else { Some(description) };

        let temperature: f32 = self.temperature_input.read(cx).editor.read(cx).text(cx).parse()
            .map_err(|_| anyhow::anyhow!("Invalid temperature value"))?;
        
        let context_size: usize = self.context_size_input.read(cx).editor.read(cx).text(cx).parse()
            .map_err(|_| anyhow::anyhow!("Invalid context size value"))?;
        
        let max_tokens: usize = self.max_tokens_input.read(cx).editor.read(cx).text(cx).parse()
            .map_err(|_| anyhow::anyhow!("Invalid max tokens value"))?;
        
        let top_k: i32 = self.top_k_input.read(cx).editor.read(cx).text(cx).parse()
            .map_err(|_| anyhow::anyhow!("Invalid top-k value"))?;
        
        let top_p: f32 = self.top_p_input.read(cx).editor.read(cx).text(cx).parse()
            .map_err(|_| anyhow::anyhow!("Invalid top-p value"))?;
        
        let repetition_penalty: f32 = self.repetition_penalty_input.read(cx).editor.read(cx).text(cx).parse()
            .map_err(|_| anyhow::anyhow!("Invalid repetition penalty value"))?;
        
        let repetition_penalty_window: usize = self.repetition_penalty_window_input.read(cx).editor.read(cx).text(cx).parse()
            .map_err(|_| anyhow::anyhow!("Invalid repetition penalty window value"))?;

        Ok(ModelConfiguration {
            id: self.config.id.clone(),
            name,
            description,
            temperature,
            context_size,
            max_tokens,
            top_k,
            top_p,
            repetition_penalty,
            repetition_penalty_window,
        })
    }
}

impl Render for ModelConfigurationForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_4()
            .child(
                div()
                    .mb_4()
                    .child(
                        Label::new("Edit Configuration")
                            .size(LabelSize::Large)
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .child(Label::new("Name"))
                            .child(self.name_input.clone())
                    )
                    .child(
                        div()
                            .child(Label::new("Description"))
                            .child(self.description_input.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                div()
                                    .flex_1()
                                    .child(Label::new("Temperature (0.0 - 2.0)"))
                                    .child(self.temperature_input.clone())
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .child(Label::new("Context Size"))
                                    .child(self.context_size_input.clone())
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                div()
                                    .flex_1()
                                    .child(Label::new("Max Tokens"))
                                    .child(self.max_tokens_input.clone())
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .child(Label::new("Top-K"))
                                    .child(self.top_k_input.clone())
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                div()
                                    .flex_1()
                                    .child(Label::new("Top-P (0.0 - 1.0)"))
                                    .child(self.top_p_input.clone())
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .child(Label::new("Repetition Penalty"))
                                    .child(self.repetition_penalty_input.clone())
                            )
                    )
                    .child(
                        div()
                            .child(Label::new("Repetition Penalty Window"))
                            .child(self.repetition_penalty_window_input.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .justify_end()
                            .child(
                                Button::new("cancel", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .on_click({
                                        let parent_window = self.parent_window.clone();
                                        move |_, _window, cx| {
                                            parent_window.update(cx, |window, cx| {
                                                window.cancel_editing(cx);
                                            });
                                        }
                                    })
                            )
                            .child(
                                Button::new("save", "Save")
                                    .style(ButtonStyle::Filled)
                                    .on_click({
                                        let parent_window = self.parent_window.clone();
                                        cx.listener(move |this, _, _window, cx| {
                                            if let Ok(config) = this.collect_form_data(cx) {
                                                parent_window.update(cx, |window, cx| {
                                                    window.save_configuration(config, cx);
                                                });
                                            }
                                        })
                                    })
                            )
                    )
            )
    }
}

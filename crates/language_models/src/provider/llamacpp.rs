use anyhow::{Result, anyhow};
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use futures::{Stream, TryFutureExt, stream};
use gpui::{AnyView, App, AsyncApp, Context, Subscription, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelAvailability, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelRequestTool, LanguageModelToolChoice,
    LanguageModelToolSchemaFormat, LanguageModelToolUse, LanguageModelToolUseId,
    RateLimiter, Role, StopReason, TokenUsage,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use ui::{ButtonLike, IconName, Indicator, List, ListItem, Label, LabelSize, prelude::*};
use util::ResultExt;

use crate::AllLanguageModelSettings;

const PROVIDER_ID: &str = "llamacpp";
const PROVIDER_NAME: &str = "Llama.cpp";

/// Settings for the llama.cpp provider
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LlamaCppSettings {
    /// Path to the models directory
    pub models_directory: PathBuf,
    /// Available models configuration
    pub available_models: Vec<AvailableModel>,
    /// Number of GPU layers to use (0 = CPU only)
    pub gpu_layers: u32,
    /// Number of threads to use
    pub thread_count: Option<usize>,
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

/// Model context protocol - allows AI to define its own interaction patterns
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelContextProtocol {
    /// Protocol name
    pub name: String,
    /// Protocol version
    pub version: String,
    /// Protocol description
    pub description: String,
    /// Interaction patterns
    pub patterns: Vec<InteractionPattern>,
    /// Context management rules
    pub context_rules: ContextRules,
    /// Thinking strategies
    pub thinking_strategies: Vec<ThinkingStrategy>,
}

/// Interaction pattern for model context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractionPattern {
    /// Pattern name
    pub name: String,
    /// Pattern trigger
    pub trigger: String,
    /// Pattern template
    pub template: String,
    /// Expected outputs
    pub outputs: Vec<String>,
}

/// Rules for managing context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextRules {
    /// Maximum context windows
    pub max_windows: usize,
    /// Context prioritization strategy
    pub prioritization: String,
    /// Memory management
    pub memory_strategy: String,
    /// Context compression enabled
    pub compression_enabled: bool,
}

/// Thinking strategy for human-like reasoning
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThinkingStrategy {
    /// Strategy name
    pub name: String,
    /// Strategy type (e.g., "chain-of-thought", "tree-of-thoughts", "graph-of-thoughts")
    pub strategy_type: String,
    /// Strategy parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// State for the llama.cpp provider
pub struct State {
    /// Path to models directory
    models_directory: PathBuf,
    /// Available models
    available_models: Vec<ModelInfo>,
    /// Active model instances
    active_models: Arc<Mutex<HashMap<String, Arc<LlamaModel>>>>,
    /// Model context protocols
    context_protocols: Arc<Mutex<HashMap<String, ModelContextProtocol>>>,
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
    model_type: String,
    /// Context size
    context_size: usize,
    /// Supports tools
    supports_tools: bool,
    /// Model capabilities
    capabilities: Vec<String>,
}

/// Active llama.cpp model instance
struct LlamaModel {
    /// Model info
    info: ModelInfo,
    /// Model instance (placeholder - will be replaced with actual llama-cpp binding)
    model: Arc<Mutex<Option<()>>>, // TODO: Replace with actual llama-cpp model
    /// Usage statistics
    usage_stats: Arc<Mutex<TokenUsage>>,
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
            active_models: Arc::new(Mutex::new(HashMap::new())),
            context_protocols: Arc::new(Mutex::new(HashMap::new())),
            _subscription: subscription,
        };

        // Initialize default context protocols
        state.initialize_default_protocols();
        
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

        // Scan for GGUF files
        if let Ok(entries) = std::fs::read_dir(&self.models_directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "gguf" {
                            if let Some(file_name) = path.file_stem() {
                                let id = file_name.to_string_lossy().to_string();
                                let info = ModelInfo {
                                    id: id.clone(),
                                    path: path.clone(),
                                    display_name: id.clone(),
                                    model_type: "gguf".to_string(),
                                    context_size: 4096, // Default, will be read from model
                                    supports_tools: true,
                                    capabilities: vec![
                                        "text-generation".to_string(),
                                        "reasoning".to_string(),
                                        "context-protocol-creation".to_string(),
                                    ],
                                };
                                self.available_models.push(info);
                            }
                        }
                    }
                }
            }
        }
    }

    fn initialize_default_protocols(&mut self) {
        let mut protocols = self.context_protocols.lock().unwrap();
        
        // Chain of Thought protocol
        protocols.insert(
            "chain-of-thought".to_string(),
            ModelContextProtocol {
                name: "Chain of Thought".to_string(),
                version: "1.0".to_string(),
                description: "Step-by-step reasoning protocol".to_string(),
                patterns: vec![
                    InteractionPattern {
                        name: "problem-decomposition".to_string(),
                        trigger: "complex_problem".to_string(),
                        template: "Let me break this down step by step:\n1. {step1}\n2. {step2}\n...".to_string(),
                        outputs: vec!["reasoning_steps".to_string(), "conclusion".to_string()],
                    },
                ],
                context_rules: ContextRules {
                    max_windows: 5,
                    prioritization: "recency-weighted".to_string(),
                    memory_strategy: "hierarchical".to_string(),
                    compression_enabled: true,
                },
                thinking_strategies: vec![
                    ThinkingStrategy {
                        name: "decomposition".to_string(),
                        strategy_type: "chain-of-thought".to_string(),
                        parameters: HashMap::new(),
                    },
                ],
            },
        );

        // Tree of Thoughts protocol
        protocols.insert(
            "tree-of-thoughts".to_string(),
            ModelContextProtocol {
                name: "Tree of Thoughts".to_string(),
                version: "1.0".to_string(),
                description: "Branching reasoning with multiple paths".to_string(),
                patterns: vec![
                    InteractionPattern {
                        name: "branch-exploration".to_string(),
                        trigger: "multiple_solutions".to_string(),
                        template: "Exploring multiple approaches:\nPath A: {path_a}\nPath B: {path_b}\n...".to_string(),
                        outputs: vec!["paths".to_string(), "evaluation".to_string(), "best_path".to_string()],
                    },
                ],
                context_rules: ContextRules {
                    max_windows: 10,
                    prioritization: "path-weighted".to_string(),
                    memory_strategy: "tree-structured".to_string(),
                    compression_enabled: true,
                },
                thinking_strategies: vec![
                    ThinkingStrategy {
                        name: "branching".to_string(),
                        strategy_type: "tree-of-thoughts".to_string(),
                        parameters: HashMap::from([
                            ("max_branches".to_string(), serde_json::json!(5)),
                            ("pruning_threshold".to_string(), serde_json::json!(0.7)),
                        ]),
                    },
                ],
            },
        );
    }

    fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
    }
}

pub struct LlamaCppLanguageModelProvider {
    state: gpui::Entity<State>,
}

impl LlamaCppLanguageModelProvider {
    pub fn new(_http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        Self {
            state: cx.new(State::new),
        }
    }

    /// Create a new model context protocol
    pub fn create_context_protocol(
        &self,
        name: String,
        description: String,
        cx: &App,
    ) -> Result<ModelContextProtocol> {
        let state = self.state.read(cx);
        let mut protocols = state.context_protocols.lock().unwrap();
        
        let protocol = ModelContextProtocol {
            name: name.clone(),
            version: "1.0".to_string(),
            description,
            patterns: Vec::new(),
            context_rules: ContextRules {
                max_windows: 5,
                prioritization: "adaptive".to_string(),
                memory_strategy: "dynamic".to_string(),
                compression_enabled: true,
            },
            thinking_strategies: Vec::new(),
        };
        
        protocols.insert(name, protocol.clone());
        Ok(protocol)
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
        let settings = &AllLanguageModelSettings::get_global(cx).llamacpp;
        
        state
            .available_models
            .iter()
            .map(|model_info| {
                Arc::new(LlamaCppLanguageModel {
                    id: LanguageModelId::from(model_info.id.clone()),
                    info: model_info.clone(),
                    state: self.state.clone(),
                    gpu_layers: settings.gpu_layers,
                    thread_count: settings.thread_count,
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
        cx.new(|cx| ConfigurationView::new(state, window, cx))
            .into()
    }

    fn reset_credentials(&self, _cx: &mut App) -> Task<Result<()>> {
        // Local models don't have credentials
        Task::ready(Ok(()))
    }
}

pub struct LlamaCppLanguageModel {
    id: LanguageModelId,
    info: ModelInfo,
    state: gpui::Entity<State>,
    gpu_layers: u32,
    thread_count: Option<usize>,
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
        false // Can be extended later
    }

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        false // For now
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
        // Simple approximation - will be replaced with actual tokenizer
        let token_count = request
            .messages
            .iter()
            .map(|msg| msg.string_contents().chars().count())
            .sum::<usize>()
            / 4;

        async move { Ok(token_count) }.boxed()
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
        let gpu_layers = self.gpu_layers;
        let thread_count = self.thread_count.unwrap_or_else(|| num_cpus::get());
        let context_size = self.info.context_size;
        let state = self.state.clone();

        let future = self.request_limiter.stream(async move {
            // This is a placeholder implementation
            // In the real implementation, we would:
            // 1. Load or get the cached model
            // 2. Convert the request to llama.cpp format
            // 3. Stream the completion
            
            let stream = stream::once(async move {
                Ok(LanguageModelCompletionEvent::Text(
                    "This is a placeholder response from llama.cpp. Real implementation would use the actual model.".to_string()
                ))
            }).chain(stream::once(async move {
                Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn))
            }));

            Ok(stream.boxed())
        });

        future.boxed()
    }
}

struct ConfigurationView {
    state: gpui::Entity<State>,
}

impl ConfigurationView {
    pub fn new(state: gpui::Entity<State>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { state }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let models_dir = state.models_directory.display().to_string();
        let model_count = state.available_models.len();

        div()
            .p_4()
            .size_full()
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                Label::new("Llama.cpp Configuration")
                                    .size(LabelSize::Large)
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("Use local AI models with llama.cpp")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(Label::new("Models Directory:").size(LabelSize::Small))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child(models_dir)
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(Label::new("Available Models:").size(LabelSize::Small))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child(format!("{} models found", model_count))
                                    )
                            )
                    )
                    .child(
                        List::new()
                            .empty_message("No models found. Add GGUF files to the models directory.")
                            .children(
                                state.available_models.iter().map(|model| {
                                    ListItem::new(model.id.clone())
                                        .child(
                                            Label::new(model.display_name.clone())
                                                .size(LabelSize::Small)
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(cx.theme().colors().text_muted)
                                                .child(format!("Context: {} tokens", model.context_size))
                                        )
                                })
                            )
                    )
            )
    }
} 
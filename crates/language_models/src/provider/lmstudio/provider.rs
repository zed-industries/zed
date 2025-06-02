use anyhow::Result;
use gpui::{AnyView, App, Context, Entity, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelId, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
};
use settings::Settings;
use std::sync::Arc;
use ui::{prelude::*, IconName};

use crate::AllLanguageModelSettings;

use super::{
    model::LmStudioLanguageModel,
    ui::ConfigurationView,
    PROVIDER_ID, PROVIDER_NAME,
};

pub struct LmStudioLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    pub http_client: Arc<dyn HttpClient>,
    pub available_models: Vec<lmstudio::Model>,
    fetch_model_task: Option<Task<Result<()>>>,
    _subscription: gpui::Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        // Consider authenticated if we have any models available
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
                let enabled_servers: Vec<_> = settings.servers
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
                            all_models.extend(models);
                            log::info!("All models count after extending: {}", all_models.len());
                        },
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

    pub fn public_restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        self.restart_fetch_models_task(cx);
    }

    pub fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
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
                _subscription: cx.observe_global::<AllLanguageModelSettings>(|this: &mut State, cx| {
                    // Restart fetch models task when settings change
                    this.restart_fetch_models_task(cx);
                }),
            };
            
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
        }) as Arc<dyn LanguageModel>
    }
}

impl LanguageModelProviderState for LmStudioLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
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
        let settings = AllLanguageModelSettings::get_global(cx);
        let mut models: std::collections::BTreeMap<String, lmstudio::Model> = std::collections::BTreeMap::default();

        // Add models from the LM Studio API
        log::info!("Processing models from LM Studio API, available models count: {}", self.state.read(cx).available_models.len());
        for model in self.state.read(cx).available_models.iter() {
            log::info!("Adding model from state: {}", model.name);
            models.insert(model.name.clone(), model.clone());
        }

        // Filter models based on server and model enablement settings
        log::info!("Processing {} servers from settings", settings.lmstudio.servers.len());
        
        // Create a set of enabled server IDs for quick lookup
        let enabled_server_ids: std::collections::HashSet<String> = settings.lmstudio.servers.iter()
            .filter(|server| server.enabled)
            .map(|server| server.id.clone())
            .collect();
            
        log::info!("Found {} enabled servers", enabled_server_ids.len());
        
        if enabled_server_ids.is_empty() {
            log::info!("No enabled servers found, returning empty model list");
            return Vec::new();
        }

        // Apply custom max tokens and filter disabled models
        for server in &settings.lmstudio.servers {
            if !server.enabled {
                continue;
            }
            
            if let Some(available_models) = &server.available_models {
                for model_config in available_models {
                    if !model_config.enabled {
                        // Remove disabled models from our list
                        if let Some(server_id) = &model_config.server_id {
                            if server_id == &server.id {
                                models.remove(&model_config.name);
                                log::info!("Removing disabled model: {}", model_config.name);
                            }
                        }
                        continue;
                    }
                    
                    // Apply custom max tokens for enabled models
                    if let Some(custom_max_tokens) = model_config.custom_max_tokens {
                        log::info!("Updating custom max tokens for model {}: {}", model_config.name, custom_max_tokens);
                        lmstudio::update_custom_max_tokens(&server.id, &model_config.name, Some(custom_max_tokens));
                    } else {
                        // Clear any existing custom setting
                        lmstudio::update_custom_max_tokens(&server.id, &model_config.name, None);
                    }
                }
            }
        }
        
        // Filter models to only include those from enabled servers
        let final_models: Vec<Arc<dyn LanguageModel>> = models.into_iter()
            .filter_map(|(name, model)| {
                if let Some(server_id) = &model.server_id {
                    if enabled_server_ids.contains(server_id) {
                        log::info!("Including model from enabled server: {} (server: {})", name, server_id);
                        Some(self.create_language_model(model))
                    } else {
                        log::info!("Filtering out model from disabled server: {} (server: {})", name, server_id);
                        None
                    }
                } else {
                    // Models without server_id are legacy - include them if any server is enabled
                    log::info!("Including legacy model without server_id: {}", name);
                    Some(self.create_language_model(model))
                }
            })
            .collect();

        log::info!("Returning {} final models", final_models.len());
        final_models
    }

    fn load_model(&self, model: Arc<dyn LanguageModel>, cx: &App) {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        let http_client = self.http_client.clone();
        // Get the first enabled server or return if none
        if let Some(server) = settings.first_enabled_server() {
            let api_url = server.api_url.clone();
            let id = model.id().0.to_string();
            cx.spawn(async move |_| lmstudio::preload_model(http_client, &api_url, &id).await)
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
        cx.new(|cx| ConfigurationView::new(self.state.clone(), cx)).into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.fetch_models(cx))
    }
} 
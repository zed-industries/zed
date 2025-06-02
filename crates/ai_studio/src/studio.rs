use gpui::{Context, Entity, Window, FocusHandle, EventEmitter, Focusable, Render, SharedString, FontWeight, UniformListScrollHandle, ListSizingBehavior, uniform_list};
use language_model::{LanguageModelProvider, LanguageModelProviderId};
use std::sync::Arc;
use ui::{prelude::*, IconName, Label, LabelSize, Button, ButtonStyle, Icon, ScrollbarState};
use uuid::Uuid;
use workspace::{Item, WorkspaceId};

use crate::model_manager::ModelManager as OldModelManager;
use crate::{ProviderRegistry, ChatInterface, WorkflowManagerView};
use crate::models::{ModelManager, ModelConfig, ModelCreationModal, ModelProvider};
use crate::agents::{AgentManager, AgentConfig};

/// Main AI Studio component that provides a unified interface for AI model management
pub struct AiStudio {
    provider_registry: Entity<ProviderRegistry>,
    #[allow(dead_code)]
    old_model_manager: Entity<OldModelManager>, // Keep for backwards compatibility
    chat_interface: Option<Entity<ChatInterface>>,
    workflow_manager: Entity<WorkflowManagerView>,
    
    // New AI system managers
    model_manager: Option<Arc<ModelManager>>,
    agent_manager: Option<Arc<AgentManager>>,
    models: Vec<ModelConfig>,
    agents: Vec<AgentConfig>,
    
    // Modals
    model_creation_modal: Option<Entity<ModelCreationModal>>,
    model_edit_modal: Option<Entity<ModelCreationModal>>,
    
    // Scrolling
    models_scroll_handle: UniformListScrollHandle,
    models_scrollbar_state: ScrollbarState,
    
    active_view: StudioView,
    focus_handle: FocusHandle,
}

/// Different views available in the AI Studio
#[derive(Clone, Debug, PartialEq)]
pub enum StudioView {
    Dashboard,
    Models,
    Agents,
    Providers,
    Chat,
    Workflow,
    Settings,
}

impl AiStudio {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let provider_registry = cx.new(ProviderRegistry::new);
        let old_model_manager = cx.new(OldModelManager::new);
        let workflow_manager = cx.new(|cx| WorkflowManagerView::new(window, cx));
        
        let models_scroll_handle = UniformListScrollHandle::new();
        
        let mut studio = Self {
            provider_registry,
            old_model_manager,
            chat_interface: None,
            workflow_manager,
            model_manager: None,
            agent_manager: None,
            models: Vec::new(),
            agents: Vec::new(),
            model_creation_modal: None,
            model_edit_modal: None,
            models_scroll_handle: models_scroll_handle.clone(),
            models_scrollbar_state: ScrollbarState::new(models_scroll_handle),
            active_view: StudioView::Dashboard,
            focus_handle: cx.focus_handle(),
        };

        // Initialize the new AI system
        studio.initialize_ai_system(cx);
        
        studio
    }

    fn initialize_ai_system(&mut self, cx: &mut Context<Self>) {
        // Initialize model manager
        cx.spawn(async move |this, cx| {
            println!("🔄 Initializing model manager...");
            match ModelManager::with_persistence("./ai_studio_models.db").await {
                Ok(manager) => {
                    let manager: Arc<ModelManager> = Arc::new(manager);
                    
                    // Check if models exist and load them
                    let existing_models = manager.get_all_models();
                    println!("📋 Found {} existing models in storage", existing_models.len());
                    
                    // Create default models if none exist
                    if existing_models.is_empty() {
                        println!("🎯 No models found, creating default models...");
                        if let Err(e) = manager.create_default_models().await {
                            eprintln!("❌ Failed to create default models: {}", e);
                        } else {
                            println!("✅ Default models created successfully");
                            // Verify they were created
                            let new_models = manager.get_all_models();
                            println!("📊 Total models after creation: {}", new_models.len());
                        }
                    } else {
                        println!("✅ Using existing models from storage");
                    }

                    this.update(cx, |this, _cx| {
                        let models = manager.get_all_models();
                        this.model_manager = Some(manager);
                        this.models = models;
                        println!("📋 Loaded {} models", this.models.len());
                    }).ok();
                }
                Err(e) => {
                    eprintln!("❌ Failed to initialize model manager: {}", e);
                    eprintln!("   This might be due to missing dependencies or file permissions");
                    eprintln!("   Continuing with in-memory model manager...");
                    
                    // Fallback to in-memory model manager
                    match ModelManager::new().await {
                        Ok(manager) => {
                            let manager: Arc<ModelManager> = Arc::new(manager);
                            this.update(cx, |this, _cx| {
                                this.model_manager = Some(manager);
                                println!("⚠️  Using in-memory model manager (no persistence)");
                            }).ok();
                        }
                        Err(e2) => {
                            eprintln!("❌ Failed to create fallback model manager: {}", e2);
                        }
                    }
                }
            }
        }).detach();

        // Initialize agent manager  
        cx.spawn(async move |this, cx| {
            println!("🤖 Initializing agent manager...");
            match AgentManager::with_persistence("./ai_studio_agents.db").await {
                Ok(manager) => {
                    let manager: Arc<AgentManager> = Arc::new(manager);
                    
                    this.update(cx, |this, _cx| {
                        let agents = manager.get_all_agents();
                        this.agent_manager = Some(manager);
                        this.agents = agents;
                        println!("🤖 Loaded {} agents", this.agents.len());
                    }).ok();
                    println!("✅ Agent manager initialized");
                }
                Err(e) => {
                    eprintln!("❌ Failed to initialize agent manager: {}", e);
                }
            }
        }).detach();
    }

    fn refresh_models(&mut self, cx: &mut Context<Self>) {
        if let Some(ref manager) = self.model_manager {
            self.models = manager.get_all_models();
            println!("📋 Loaded {} models", self.models.len());
        }
        cx.notify();
    }

    fn refresh_agents(&mut self, cx: &mut Context<Self>) {
        if let Some(ref manager) = self.agent_manager {
            self.agents = manager.get_all_agents();
            println!("🤖 Loaded {} agents", self.agents.len());
        }
        cx.notify();
    }

    pub fn set_view(&mut self, view: StudioView, cx: &mut Context<Self>) {
        self.active_view = view.clone();
        
        // Refresh data when switching to Models or Agents views
        match view {
            StudioView::Models => self.refresh_models(cx),
            StudioView::Agents => self.refresh_agents(cx),
            _ => {}
        }
        
        cx.notify();
    }

    pub fn open_chat(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.chat_interface.is_none() {
            self.chat_interface = Some(cx.new(|cx| ChatInterface::new(window, cx)));
        }
        self.set_view(StudioView::Chat, cx);
    }

    pub fn close_chat(&mut self, cx: &mut Context<Self>) {
        self.chat_interface = None;
        self.set_view(StudioView::Dashboard, cx);
    }

    pub fn add_provider(&mut self, provider: Arc<dyn LanguageModelProvider>, cx: &mut Context<Self>) {
        self.provider_registry.update(cx, |registry, cx| {
            registry.add_provider(provider, cx);
        });
    }

    pub fn remove_provider(&mut self, provider_id: &LanguageModelProviderId, cx: &mut Context<Self>) {
        self.provider_registry.update(cx, |registry, cx| {
            registry.remove_provider(provider_id, cx);
        });
    }

    fn create_new_model(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.model_creation_modal.is_none() {
            let modal = cx.new(|cx| ModelCreationModal::new(window, cx));
            cx.subscribe(&modal, |this, _modal, event, cx| {
                if let Some(model_config) = event {
                    println!("📝 Creating new model: {}", model_config.name);
                    // Model created, add it to the manager
                    if let Some(ref manager) = this.model_manager {
                        let manager = manager.clone();
                        let model_config = model_config.clone();
                        cx.spawn(async move |this, cx| {
                            match manager.add_model(model_config.clone()).await {
                                Ok(model_id) => {
                                    println!("✅ Model created successfully with ID: {}", model_id);
                                    this.update(cx, |this, _cx| {
                                        this.models = this.model_manager.as_ref().map(|m| m.get_all_models()).unwrap_or_default();
                                        println!("🔄 Refreshed models list - now have {} models", this.models.len());
                                    }).ok();
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to create model '{}': {}", model_config.name, e);
                                }
                            }
                        }).detach();
                    } else {
                        eprintln!("⚠️  No model manager available to create model");
                    }
                }
                // Close the modal
                this.model_creation_modal = None;
                cx.notify();
            }).detach();
            
            self.model_creation_modal = Some(modal);
            cx.notify();
        }
    }

    fn create_new_agent(&mut self, cx: &mut Context<Self>) {
        if let Some(ref agent_manager) = self.agent_manager {
            // Get the first available model for the agent
            if let Some(first_model) = self.models.first() {
                use crate::agents::{AgentType, AgentContext, AgentCapability, SpawningRules};
                
                let context = AgentContext {
                    system_prompt: "You are a helpful AI assistant specialized in UI/UX design and development. You can create dynamic user interfaces using GPUI.".to_string(),
                    ..Default::default()
                };
                
                let spawning_rules = SpawningRules {
                    can_spawn_agents: true,
                    max_child_agents: 2,
                    allowed_agent_types: vec![
                        AgentType::UIDesigner,
                        AgentType::CodeSpecialist,
                    ],
                    ..Default::default()
                };
                
                let new_agent = AgentConfig::new(
                    "UI Designer Agent".to_string(),
                    AgentType::UIDesigner,
                    first_model.id,
                )
                .with_description("Specialized agent for creating dynamic user interfaces".to_string())
                .with_context(context)
                .with_capabilities(vec![
                    AgentCapability::UICreation,
                    AgentCapability::CodeGeneration,
                    AgentCapability::AgentSpawning,
                    AgentCapability::ToolUsage,
                ])
                .with_tools(vec!["gpui_renderer".to_string(), "file_manager".to_string()])
                .with_spawning_rules(spawning_rules);
                
                let manager = agent_manager.clone();
                cx.spawn(async move |this, cx| {
                    match manager.add_agent(new_agent).await {
                        Ok(_) => {
                            this.update(cx, |this, _cx| {
                                this.agents = this.agent_manager.as_ref().map(|m| m.get_all_agents()).unwrap_or_default();
                            }).ok();
                        }
                        Err(e) => {
                            eprintln!("Failed to create agent: {}", e);
                        }
                    }
                }).detach();
            } else {
                println!("⚠️  No models available to create agent");
            }
        }
    }

    fn delete_model(&mut self, model_id: Uuid, cx: &mut Context<Self>) {
        if let Some(ref manager) = self.model_manager {
            let manager = manager.clone();
            cx.spawn(async move |this, cx| {
                match manager.delete_model(&model_id).await {
                    Ok(deleted) => {
                        if deleted {
                            println!("✅ Model deleted successfully: {}", model_id);
                            this.update(cx, |this, _cx| {
                                this.models = this.model_manager.as_ref().map(|m| m.get_all_models()).unwrap_or_default();
                                println!("🔄 Refreshed models list - now have {} models", this.models.len());
                            }).ok();
                        } else {
                            eprintln!("⚠️  Model not found: {}", model_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete model '{}': {}", model_id, e);
                    }
                }
            }).detach();
        } else {
            eprintln!("⚠️  No model manager available to delete model");
        }
    }

    fn edit_model(&mut self, model_id: Uuid, window: &mut Window, cx: &mut Context<Self>) {
        if self.model_edit_modal.is_none() {
            // Find the model to edit
            if let Some(model) = self.models.iter().find(|m| m.id == model_id) {
                let modal = cx.new(|cx| ModelCreationModal::from_existing_model(model.clone(), window, cx));
                cx.subscribe(&modal, move |this, _modal, event, cx| {
                    if let Some(updated_model_config) = event {
                        println!("📝 Updating model: {}", updated_model_config.name);
                        // Model updated, save it to the manager
                        if let Some(ref manager) = this.model_manager {
                            let manager = manager.clone();
                            let model_config = updated_model_config.clone();
                            cx.spawn(async move |this, cx| {
                                match manager.update_model(model_config.clone()).await {
                                    Ok(()) => {
                                        println!("✅ Model updated successfully: {}", model_config.name);
                                        this.update(cx, |this, _cx| {
                                            this.models = this.model_manager.as_ref().map(|m| m.get_all_models()).unwrap_or_default();
                                            println!("🔄 Refreshed models list - now have {} models", this.models.len());
                                        }).ok();
                                    }
                                    Err(e) => {
                                        eprintln!("❌ Failed to update model '{}': {}", model_config.name, e);
                                    }
                                }
                            }).detach();
                        } else {
                            eprintln!("⚠️  No model manager available to update model");
                        }
                    }
                    // Close the modal
                    this.model_edit_modal = None;
                    cx.notify();
                }).detach();
                
                self.model_edit_modal = Some(modal);
                cx.notify();
            }
        }
    }

    fn render_navigation(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_48()
            .bg(cx.theme().colors().panel_background)
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Label::new("AI Studio")
                            .size(LabelSize::Large)
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_2()
                    .gap_1()
                    .child(self.nav_button("Dashboard", IconName::Cog, StudioView::Dashboard, cx))
                    .child(self.nav_button("Models", IconName::Brain, StudioView::Models, cx))
                    .child(self.nav_button("Agents", IconName::Person, StudioView::Agents, cx))
                    .child(self.nav_button("Providers", IconName::Server, StudioView::Providers, cx))
                    .child(self.nav_button("Chat", IconName::MessageBubbles, StudioView::Chat, cx))
                    .child(self.nav_button("Workflow", IconName::Route, StudioView::Workflow, cx))
                    .child(self.nav_button("Settings", IconName::Settings, StudioView::Settings, cx))
            )
    }

    fn nav_button(
        &self,
        label: &'static str,
        icon: IconName,
        view: StudioView,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.active_view == view;
        
        Button::new(label, label)
            .style(if is_active { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .full_width()
            .icon(Some(icon))
            .icon_position(ui::IconPosition::Start)
            .on_click({
                let view = view.clone();
                cx.listener(move |this, _, _window, cx| {
                    this.set_view(view.clone(), cx);
                })
            })
    }

    fn render_dashboard(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_6()
            .p_6()
            .child(
                Label::new("AI Studio Dashboard")
                    .size(LabelSize::Large)
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .bg(cx.theme().colors().surface_background)
                            .rounded_lg()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .mb_2()
                                    .child(Icon::new(IconName::Brain))
                                    .child(Label::new("Models"))
                            )
                            .child(
                                Label::new(format!("{} configured models", self.models.len()))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .child(
                                        Button::new("view_models", "Manage Models")
                                            .style(ButtonStyle::Filled)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.set_view(StudioView::Models, cx);
                                            }))
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .bg(cx.theme().colors().surface_background)
                            .rounded_lg()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .mb_2()
                                    .child(Icon::new(IconName::Person))
                                    .child(Label::new("Agents"))
                            )
                            .child(
                                Label::new(format!("{} AI agents", self.agents.len()))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .child(
                                        Button::new("view_agents", "Manage Agents")
                                            .style(ButtonStyle::Filled)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.set_view(StudioView::Agents, cx);
                                            }))
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .bg(cx.theme().colors().surface_background)
                            .rounded_lg()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .mb_2()
                                    .child(Icon::new(IconName::Server))
                                    .child(Label::new("Providers"))
                            )
                            .child(
                                Label::new("Manage AI model providers")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .child(
                                        Button::new("view_providers", "View Providers")
                                            .style(ButtonStyle::Filled)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.set_view(StudioView::Providers, cx);
                                            }))
                                    )
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .bg(cx.theme().colors().surface_background)
                            .rounded_lg()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .mb_2()
                                    .child(Icon::new(IconName::MessageBubbles))
                                    .child(Label::new("Chat"))
                            )
                            .child(
                                Label::new("Interactive chat interface")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .child(
                                        Button::new("start_chat", "Start Chat")
                                            .style(ButtonStyle::Filled)
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.open_chat(window, cx);
                                            }))
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .bg(cx.theme().colors().surface_background)
                            .rounded_lg()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .mb_2()
                                    .child(Icon::new(IconName::Route))
                                    .child(Label::new("Workflow"))
                            )
                            .child(
                                Label::new("Visual workflow builder")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .child(
                                        Button::new("open_workflow", "Open Workflow")
                                            .style(ButtonStyle::Filled)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.set_view(StudioView::Workflow, cx);
                                            }))
                                    )
                            )
                    )
            )
            .child(
                div()
                    .child(
                        Label::new("Recent Activity")
                            .size(LabelSize::Default)
                    )
                    .child(
                        div()
                            .mt_2()
                            .p_4()
                            .bg(cx.theme().colors().surface_background)
                            .rounded_lg()
                            .child(
                                Label::new("No recent activity")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                    )
            )
    }

    fn render_models_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mb_4()
                    .child(
                        Label::new("Model Configurations")
                            .size(LabelSize::Large)
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                Button::new("create_model", "Create Model")
                                    .style(ButtonStyle::Filled)
                                    .icon(IconName::Plus)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.create_new_model(window, cx);
                                    }))
                            )
                            .child(
                                Label::new(format!("{} models", self.models.len()))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                    )
            )
            .child(
                if self.models.is_empty() {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .size_full()
                        .gap_4()
                        .child(
                            Icon::new(IconName::Brain)
                                .size(IconSize::XLarge)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("No models configured yet")
                                .size(LabelSize::Large)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("Create your first model configuration to get started")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                        .into_any_element()
                } else {
                    // Use uniform_list for proper scrolling like project panel
                    div()
                        .flex_grow()
                        .min_h_0()
                        .child(
                            uniform_list(
                                cx.entity().clone(),
                                "models",
                                self.models.len(),
                                |this, range, _window, cx| {
                                    let mut items = Vec::with_capacity(range.end - range.start);
                                    for index in range {
                                        if let Some(model) = this.models.get(index) {
                                            items.push(this.render_model_card(model, cx));
                                        }
                                    }
                                    items
                                }
                            )
                            .size_full()
                            .with_sizing_behavior(ListSizingBehavior::Infer)
                            .track_scroll(self.models_scroll_handle.clone())
                        )
                        .into_any_element()
                }
            )
    }

    fn render_model_card(&self, model: &ModelConfig, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let model_id = model.id;
        let edit_id = SharedString::from(format!("edit_{}", model_id));
        let delete_id = SharedString::from(format!("delete_{}", model_id));
        
        div()
            .flex()
            .items_center()
            .justify_between()
            .p_3()
            .bg(cx.theme().colors().surface_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .flex_1()
                    .child(
                        Label::new(model.name.clone())
                            .size(LabelSize::Default)
                            .weight(FontWeight::BOLD)
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Label::new(format!("{:?}", model.model_type))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                Label::new("•")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                Label::new(match &model.provider {
                                    ModelProvider::OpenAI { .. } => "OpenAI".to_string(),
                                    ModelProvider::Anthropic { .. } => "Anthropic".to_string(),
                                    ModelProvider::Local { .. } => "Local".to_string(),
                                    ModelProvider::Ollama { .. } => "Ollama".to_string(),
                                    ModelProvider::HuggingFace { .. } => "HuggingFace".to_string(),
                                    ModelProvider::Custom { name, .. } => name.clone(),
                                })
                                    .size(LabelSize::Small)
                                    .color(Color::Accent)
                            )
                    )
                    .when(!model.description.is_empty(), |div| {
                        div.child(
                            Label::new(model.description.clone())
                                .size(LabelSize::Small)
                                .color(Color::Default)
                        )
                    })
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_end()
                            .gap_1()
                            .child(
                                Label::new(format!("{} capabilities", model.capabilities.len()))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Accent)
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        if model.is_active {
                                            div()
                                                .size_2()
                                                .rounded_full()
                                                .bg(Color::Success.color(cx))
                                        } else {
                                            div()
                                                .size_2()
                                                .rounded_full()
                                                .bg(Color::Muted.color(cx))
                                        }
                                    )
                                    .child(
                                        Label::new(if model.is_active { "Active" } else { "Inactive" })
                                            .size(LabelSize::XSmall)
                                            .color(if model.is_active { Color::Success } else { Color::Muted })
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Button::new(edit_id, "Edit")
                                    .style(ButtonStyle::Subtle)
                                    .icon(IconName::Pencil)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        this.edit_model(model_id, window, cx);
                                    }))
                            )
                            .child(
                                Button::new(delete_id, "Delete")
                                    .style(ButtonStyle::Subtle)
                                    .icon(IconName::Trash)
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.delete_model(model_id, cx);
                                    }))
                            )
                    )
            )
    }

    fn render_agents_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mb_4()
                    .child(
                        Label::new("AI Agents")
                            .size(LabelSize::Large)
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                Button::new("create_agent", "Create Agent")
                                    .style(ButtonStyle::Filled)
                                    .icon(IconName::Plus)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.create_new_agent(cx);
                                    }))
                            )
                            .child(
                                Label::new(format!("{} agents", self.agents.len()))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                    )
            )
            .child(
                if self.agents.is_empty() {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .size_full()
                        .gap_4()
                        .child(
                            Icon::new(IconName::Person)
                                .size(IconSize::XLarge)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("No agents created yet")
                                .size(LabelSize::Large)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("Create your first AI agent to start building intelligent workflows")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                        .into_any_element()
                } else {
                    let mut agent_cards = Vec::new();
                    for agent in &self.agents {
                        agent_cards.push(self.render_agent_card(agent, cx));
                    }
                    
                    v_flex()
                        .gap_2()
                        .children(agent_cards)
                        .into_any_element()
                }
            )
    }

    fn render_agent_card(&self, agent: &AgentConfig, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        // Find the model this agent uses
        let model_name = self.models.iter()
            .find(|m| m.id == agent.model_id)
            .map(|m| m.name.clone())
            .unwrap_or_else(|| "Unknown Model".to_string());

        div()
            .flex()
            .items_center()
            .justify_between()
            .p_3()
            .bg(cx.theme().colors().surface_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        Label::new(agent.name.clone())
                            .size(LabelSize::Default)
                            .weight(FontWeight::BOLD)
                    )
                    .child(
                        Label::new(format!("{:?} • {}", agent.agent_type, model_name))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
                    .when(!agent.description.is_empty(), |div| {
                        div.child(
                            Label::new(agent.description.clone())
                                .size(LabelSize::Small)
                                .color(Color::Default)
                        )
                    })
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Label::new(format!("{} tools", agent.tools.len()))
                            .size(LabelSize::XSmall)
                            .color(Color::Accent)
                    )
                    .child(
                        if agent.can_spawn_agents() {
                            Label::new("🌟 Spawner")
                                .size(LabelSize::XSmall)
                                .color(Color::Success)
                        } else {
                            Label::new("🤖 Worker")
                                .size(LabelSize::XSmall)
                                .color(Color::Info)
                        }
                    )
                    .child(
                        if agent.is_active {
                            div()
                                .size_2()
                                .rounded_full()
                                .bg(Color::Success.color(cx))
                        } else {
                            div()
                                .size_2()
                                .rounded_full()
                                .bg(Color::Muted.color(cx))
                        }
                    )
            )
    }

    fn render_content(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let main_content = match self.active_view {
            StudioView::Dashboard => self.render_dashboard(cx).into_any_element(),
            StudioView::Models => self.render_models_tab(cx).into_any_element(),
            StudioView::Agents => self.render_agents_tab(cx).into_any_element(),
            StudioView::Providers => self.provider_registry.clone().into_any_element(),
            StudioView::Chat => {
                if let Some(chat) = &self.chat_interface {
                    chat.clone().into_any_element()
                } else {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size_full()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_4()
                                .child(
                                    Label::new("No chat session active")
                                        .size(LabelSize::Large)
                                        .color(Color::Muted)
                                )
                                .child(
                                    Button::new("start_chat", "Start New Chat")
                                        .style(ButtonStyle::Filled)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.open_chat(window, cx);
                                        }))
                                )
                        )
                        .into_any_element()
                }
            }
            StudioView::Workflow => self.workflow_manager.clone().into_any_element(),
            StudioView::Settings => {
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .child(
                        Label::new("Settings - Coming Soon")
                            .size(LabelSize::Large)
                            .color(Color::Muted)
                    )
                    .into_any_element()
            }
        };

        div()
            .relative()
            .size_full()
            .child(main_content)
            .when_some(self.model_creation_modal.as_ref(), |element, modal| {
                element.child(
                    div()
                        .absolute()
                        .inset_0()
                        .bg(cx.theme().colors().background.alpha(0.8))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(modal.clone())
                )
            })
            .when_some(self.model_edit_modal.as_ref(), |element, modal| {
                element.child(
                    div()
                        .absolute()
                        .inset_0()
                        .bg(cx.theme().colors().background.alpha(0.8))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(modal.clone())
                )
            })
    }
}

impl Render for AiStudio {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(cx.theme().colors().background)
            .child(self.render_navigation(cx))
            .child(
                div()
                    .flex_1()
                    .child(self.render_content(window, cx))
            )
    }
}

impl EventEmitter<()> for AiStudio {}

impl Focusable for AiStudio {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for AiStudio {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &gpui::App) -> SharedString {
        "AI Studio".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("ai studio")
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| Self::new(window, cx)))
    }

    fn to_item_events(_event: &Self::Event, mut _f: impl FnMut(workspace::item::ItemEvent)) {
        // No events to convert
    }
} 
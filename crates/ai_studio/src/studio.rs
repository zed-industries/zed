use gpui::{Context, Entity, Window, FocusHandle, EventEmitter, Focusable, Render, SharedString};
use language_model::{LanguageModelProvider, LanguageModelProviderId};
use std::sync::Arc;
use ui::{prelude::*, IconName, Label, LabelSize, Button, ButtonStyle, Icon};
use workspace::{Item, WorkspaceId};

use crate::{ModelManager, ProviderRegistry, ChatInterface, WorkflowManagerView};

/// Main AI Studio component that provides a unified interface for AI model management
pub struct AiStudio {
    provider_registry: Entity<ProviderRegistry>,
    model_manager: Entity<ModelManager>,
    chat_interface: Option<Entity<ChatInterface>>,
    workflow_manager: Entity<WorkflowManagerView>,
    active_view: StudioView,
    focus_handle: FocusHandle,
}

/// Different views available in the AI Studio
#[derive(Clone, Debug, PartialEq)]
pub enum StudioView {
    Dashboard,
    Models,
    Providers,
    Chat,
    Workflow,
    Settings,
}

impl AiStudio {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let provider_registry = cx.new(ProviderRegistry::new);
        let model_manager = cx.new(ModelManager::new);
        let workflow_manager = cx.new(|cx| WorkflowManagerView::new(window, cx));
        
        Self {
            provider_registry,
            model_manager,
            chat_interface: None,
            workflow_manager,
            active_view: StudioView::Dashboard,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_view(&mut self, view: StudioView, cx: &mut Context<Self>) {
        self.active_view = view;
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
                    .child(self.nav_button("Models", IconName::FileCode, StudioView::Models, cx))
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
                                    .child(Icon::new(IconName::FileCode))
                                    .child(Label::new("Models"))
                            )
                            .child(
                                Label::new("Browse and manage AI models")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .child(
                                        Button::new("view_models", "View Models")
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

    fn render_content(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_view {
            StudioView::Dashboard => self.render_dashboard(cx).into_any_element(),
            StudioView::Models => self.model_manager.clone().into_any_element(),
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
        }
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
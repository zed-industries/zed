mod add_context_server_modal;
mod configure_context_server_modal;
mod manage_profiles_modal;
mod tool_picker;

use std::sync::Arc;

use assistant_settings::AssistantSettings;
use assistant_tool::{ToolSource, ToolWorkingSet};
use collections::HashMap;
use context_server::manager::ContextServerManager;
use fs::Fs;
use gpui::{
    Action, AnyView, App, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle, Subscription,
};
use language_model::{LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry};
use settings::{Settings, update_settings_file};
use ui::{
    Disclosure, Divider, DividerColor, ElevationIndex, Indicator, Scrollbar, ScrollbarState,
    Switch, SwitchColor, Tooltip, prelude::*,
};
use util::ResultExt as _;
use zed_actions::ExtensionCategoryFilter;

pub(crate) use add_context_server_modal::AddContextServerModal;
pub(crate) use configure_context_server_modal::ConfigureContextServerModal;
pub(crate) use manage_profiles_modal::ManageProfilesModal;

use crate::AddContextServer;

pub struct AssistantConfiguration {
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    configuration_views_by_provider: HashMap<LanguageModelProviderId, AnyView>,
    context_server_manager: Entity<ContextServerManager>,
    expanded_context_server_tools: HashMap<Arc<str>, bool>,
    tools: Entity<ToolWorkingSet>,
    _registry_subscription: Subscription,
    scroll_handle: ScrollHandle,
    scrollbar_state: ScrollbarState,
}

impl AssistantConfiguration {
    pub fn new(
        fs: Arc<dyn Fs>,
        context_server_manager: Entity<ContextServerManager>,
        tools: Entity<ToolWorkingSet>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let registry_subscription = cx.subscribe_in(
            &LanguageModelRegistry::global(cx),
            window,
            |this, _, event: &language_model::Event, window, cx| match event {
                language_model::Event::AddedProvider(provider_id) => {
                    let provider = LanguageModelRegistry::read_global(cx).provider(provider_id);
                    if let Some(provider) = provider {
                        this.add_provider_configuration_view(&provider, window, cx);
                    }
                }
                language_model::Event::RemovedProvider(provider_id) => {
                    this.remove_provider_configuration_view(provider_id);
                }
                _ => {}
            },
        );

        let scroll_handle = ScrollHandle::new();
        let scrollbar_state = ScrollbarState::new(scroll_handle.clone());

        let mut this = Self {
            fs,
            focus_handle,
            configuration_views_by_provider: HashMap::default(),
            context_server_manager,
            expanded_context_server_tools: HashMap::default(),
            tools,
            _registry_subscription: registry_subscription,
            scroll_handle,
            scrollbar_state,
        };
        this.build_provider_configuration_views(window, cx);
        this
    }

    fn build_provider_configuration_views(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let providers = LanguageModelRegistry::read_global(cx).providers();
        for provider in providers {
            self.add_provider_configuration_view(&provider, window, cx);
        }
    }

    fn remove_provider_configuration_view(&mut self, provider_id: &LanguageModelProviderId) {
        self.configuration_views_by_provider.remove(provider_id);
    }

    fn add_provider_configuration_view(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let configuration_view = provider.configuration_view(window, cx);
        self.configuration_views_by_provider
            .insert(provider.id(), configuration_view);
    }
}

impl Focusable for AssistantConfiguration {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub enum AssistantConfigurationEvent {
    NewThread(Arc<dyn LanguageModelProvider>),
}

impl EventEmitter<AssistantConfigurationEvent> for AssistantConfiguration {}

impl AssistantConfiguration {
    fn render_provider_configuration_block(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let provider_id = provider.id().0.clone();
        let provider_name = provider.name().0.clone();
        let configuration_view = self
            .configuration_views_by_provider
            .get(&provider.id())
            .cloned();

        v_flex()
            .pt_3()
            .pb_1()
            .gap_1p5()
            .border_t_1()
            .border_color(cx.theme().colors().border.opacity(0.6))
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Icon::new(provider.icon())
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Label::new(provider_name.clone()).size(LabelSize::Large)),
                    )
                    .when(provider.is_authenticated(cx), |parent| {
                        parent.child(
                            Button::new(
                                SharedString::from(format!("new-thread-{provider_id}")),
                                "Start New Thread",
                            )
                            .icon_position(IconPosition::Start)
                            .icon(IconName::Plus)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::ModalSurface)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener({
                                let provider = provider.clone();
                                move |_this, _event, _window, cx| {
                                    cx.emit(AssistantConfigurationEvent::NewThread(
                                        provider.clone(),
                                    ))
                                }
                            })),
                        )
                    }),
            )
            .map(|parent| match configuration_view {
                Some(configuration_view) => parent.child(configuration_view),
                None => parent.child(div().child(Label::new(format!(
                    "No configuration view for {provider_name}",
                )))),
            })
    }

    fn render_provider_configuration_section(
        &mut self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let providers = LanguageModelRegistry::read_global(cx).providers();

        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .pr(DynamicSpacing::Base20.rems(cx))
            .gap_4()
            .flex_1()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Headline::new("LLM Providers"))
                    .child(
                        Label::new("Add at least one provider to use AI-powered features.")
                            .color(Color::Muted),
                    ),
            )
            .children(
                providers
                    .into_iter()
                    .map(|provider| self.render_provider_configuration_block(&provider, cx)),
            )
    }

    fn render_command_permission(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let always_allow_tool_actions = AssistantSettings::get_global(cx).always_allow_tool_actions;

        const HEADING: &str = "Allow running editing tools without asking for confirmation";

        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .pr(DynamicSpacing::Base20.rems(cx))
            .gap_2()
            .flex_1()
            .child(Headline::new("General Settings"))
            .child(
                h_flex()
                    .gap_4()
                    .justify_between()
                    .flex_wrap()
                    .child(
                        v_flex()
                            .gap_0p5()
                            .max_w_5_6()
                            .child(Label::new(HEADING))
                            .child(Label::new("When enabled, the agent can perform potentially destructive actions without asking for your confirmation.").color(Color::Muted)),
                    )
                    .child(
                        Switch::new(
                            "always-allow-tool-actions-switch",
                            always_allow_tool_actions.into(),
                        )
                        .color(SwitchColor::Accent)
                        .on_click({
                            let fs = self.fs.clone();
                            move |state, _window, cx| {
                                let allow = state == &ToggleState::Selected;
                                update_settings_file::<AssistantSettings>(
                                    fs.clone(),
                                    cx,
                                    move |settings, _| {
                                        settings.set_always_allow_tool_actions(allow);
                                    },
                                );
                            }
                        }),
                    ),
            )
    }

    fn render_context_servers_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let context_servers = self.context_server_manager.read(cx).all_servers().clone();
        let tools_by_source = self.tools.read(cx).tools_by_source(cx);
        let empty = Vec::new();

        const SUBHEADING: &str = "Connect to context servers via the Model Context Protocol either via Zed extensions or directly.";

        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .pr(DynamicSpacing::Base20.rems(cx))
            .gap_2()
            .flex_1()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Headline::new("Model Context Protocol (MCP) Servers"))
                    .child(Label::new(SUBHEADING).color(Color::Muted)),
            )
            .children(context_servers.into_iter().map(|context_server| {
                let is_running = context_server.client().is_some();
                let are_tools_expanded = self
                    .expanded_context_server_tools
                    .get(&context_server.id())
                    .copied()
                    .unwrap_or_default();

                let tools = tools_by_source
                    .get(&ToolSource::ContextServer {
                        id: context_server.id().into(),
                    })
                    .unwrap_or_else(|| &empty);
                let tool_count = tools.len();

                v_flex()
                    .id(SharedString::from(context_server.id()))
                    .border_1()
                    .rounded_md()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().background.opacity(0.25))
                    .child(
                        h_flex()
                            .p_1()
                            .justify_between()
                            .when(are_tools_expanded && tool_count > 1, |element| {
                                element
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                            })
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Disclosure::new("tool-list-disclosure", are_tools_expanded)
                                            .disabled(tool_count == 0)
                                            .on_click(cx.listener({
                                                let context_server_id = context_server.id();
                                                move |this, _event, _window, _cx| {
                                                    let is_open = this
                                                        .expanded_context_server_tools
                                                        .entry(context_server_id.clone())
                                                        .or_insert(false);

                                                    *is_open = !*is_open;
                                                }
                                            })),
                                    )
                                    .child(Indicator::dot().color(if is_running {
                                        Color::Success
                                    } else {
                                        Color::Error
                                    }))
                                    .child(Label::new(context_server.id()))
                                    .child(
                                        Label::new(format!("{tool_count} tools"))
                                            .color(Color::Muted)
                                            .size(LabelSize::Small),
                                    ),
                            )
                            .child(
                                Switch::new("context-server-switch", is_running.into())
                                    .color(SwitchColor::Accent)
                                    .on_click({
                                        let context_server_manager =
                                            self.context_server_manager.clone();
                                        let context_server = context_server.clone();
                                        move |state, _window, cx| match state {
                                            ToggleState::Unselected
                                            | ToggleState::Indeterminate => {
                                                context_server_manager.update(cx, |this, cx| {
                                                    this.stop_server(context_server.clone(), cx)
                                                        .log_err();
                                                });
                                            }
                                            ToggleState::Selected => {
                                                cx.spawn({
                                                    let context_server_manager =
                                                        context_server_manager.clone();
                                                    let context_server = context_server.clone();
                                                    async move |cx| {
                                                        if let Some(start_server_task) =
                                                            context_server_manager
                                                                .update(cx, |this, cx| {
                                                                    this.start_server(
                                                                        context_server,
                                                                        cx,
                                                                    )
                                                                })
                                                                .log_err()
                                                        {
                                                            start_server_task.await.log_err();
                                                        }
                                                    }
                                                })
                                                .detach();
                                            }
                                        }
                                    }),
                            ),
                    )
                    .map(|parent| {
                        if !are_tools_expanded {
                            return parent;
                        }

                        parent.child(v_flex().py_1p5().px_1().gap_1().children(
                            tools.into_iter().enumerate().map(|(ix, tool)| {
                                h_flex()
                                    .id(("tool-item", ix))
                                    .px_1()
                                    .gap_2()
                                    .justify_between()
                                    .hover(|style| style.bg(cx.theme().colors().element_hover))
                                    .rounded_sm()
                                    .child(
                                        Label::new(tool.name())
                                            .buffer_font(cx)
                                            .size(LabelSize::Small),
                                    )
                                    .child(
                                        Icon::new(IconName::Info)
                                            .size(IconSize::Small)
                                            .color(Color::Ignored),
                                    )
                                    .tooltip(Tooltip::text(tool.description()))
                            }),
                        ))
                    })
            }))
            .child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .child(
                        h_flex().w_full().child(
                            Button::new("add-context-server", "Add Custom Server")
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ModalSurface)
                                .full_width()
                                .icon(IconName::Plus)
                                .icon_size(IconSize::Small)
                                .icon_position(IconPosition::Start)
                                .on_click(|_event, window, cx| {
                                    window.dispatch_action(AddContextServer.boxed_clone(), cx)
                                }),
                        ),
                    )
                    .child(
                        h_flex().w_full().child(
                            Button::new(
                                "install-context-server-extensions",
                                "Install MCP Extensions",
                            )
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::ModalSurface)
                            .full_width()
                            .icon(IconName::DatabaseZap)
                            .icon_size(IconSize::Small)
                            .icon_position(IconPosition::Start)
                            .on_click(|_event, window, cx| {
                                window.dispatch_action(
                                    zed_actions::Extensions {
                                        category_filter: Some(
                                            ExtensionCategoryFilter::ContextServers,
                                        ),
                                    }
                                    .boxed_clone(),
                                    cx,
                                )
                            }),
                        ),
                    ),
            )
    }
}

impl Render for AssistantConfiguration {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("assistant-configuration")
            .key_context("AgentConfiguration")
            .track_focus(&self.focus_handle(cx))
            .relative()
            .size_full()
            .pb_8()
            .bg(cx.theme().colors().panel_background)
            .child(
                v_flex()
                    .id("assistant-configuration-content")
                    .track_scroll(&self.scroll_handle)
                    .size_full()
                    .overflow_y_scroll()
                    .child(self.render_command_permission(cx))
                    .child(Divider::horizontal().color(DividerColor::Border))
                    .child(self.render_context_servers_section(cx))
                    .child(Divider::horizontal().color(DividerColor::Border))
                    .child(self.render_provider_configuration_section(cx)),
            )
            .child(
                div()
                    .id("assistant-configuration-scrollbar")
                    .occlude()
                    .absolute()
                    .right(px(3.))
                    .top_0()
                    .bottom_0()
                    .pb_6()
                    .w(px(12.))
                    .cursor_default()
                    .on_mouse_move(cx.listener(|_, _, _window, cx| {
                        cx.notify();
                        cx.stop_propagation()
                    }))
                    .on_hover(|_, _window, cx| {
                        cx.stop_propagation();
                    })
                    .on_any_mouse_down(|_, _window, cx| {
                        cx.stop_propagation();
                    })
                    .on_scroll_wheel(cx.listener(|_, _, _window, cx| {
                        cx.notify();
                    }))
                    .children(Scrollbar::vertical(self.scrollbar_state.clone())),
            )
    }
}

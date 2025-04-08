mod add_context_server_modal;
mod manage_profiles_modal;
mod tool_picker;

use std::sync::Arc;

use assistant_settings::AssistantSettings;
use assistant_tool::{ToolSource, ToolWorkingSet};
use collections::HashMap;
use context_server::manager::ContextServerManager;
use fs::Fs;
use gpui::{Action, AnyView, App, Entity, EventEmitter, FocusHandle, Focusable, Subscription};
use language_model::{LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry};
use settings::{Settings, update_settings_file};
use ui::{Disclosure, Divider, DividerColor, ElevationIndex, Indicator, Switch, prelude::*};
use util::ResultExt as _;
use zed_actions::ExtensionCategoryFilter;

pub(crate) use add_context_server_modal::AddContextServerModal;
pub(crate) use manage_profiles_modal::ManageProfilesModal;

use crate::AddContextServer;

pub struct AssistantConfiguration {
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    configuration_views_by_provider: HashMap<LanguageModelProviderId, AnyView>,
    context_server_manager: Entity<ContextServerManager>,
    expanded_context_server_tools: HashMap<Arc<str>, bool>,
    tools: Arc<ToolWorkingSet>,
    _registry_subscription: Subscription,
}

impl AssistantConfiguration {
    pub fn new(
        fs: Arc<dyn Fs>,
        context_server_manager: Entity<ContextServerManager>,
        tools: Arc<ToolWorkingSet>,
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

        let mut this = Self {
            fs,
            focus_handle,
            configuration_views_by_provider: HashMap::default(),
            context_server_manager,
            expanded_context_server_tools: HashMap::default(),
            tools,
            _registry_subscription: registry_subscription,
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
    fn render_provider_configuration(
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
            .gap_1p5()
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
                            .child(Label::new(provider_name.clone())),
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
            .child(
                div()
                    .p(DynamicSpacing::Base08.rems(cx))
                    .bg(cx.theme().colors().editor_background)
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .rounded_sm()
                    .map(|parent| match configuration_view {
                        Some(configuration_view) => parent.child(configuration_view),
                        None => parent.child(div().child(Label::new(format!(
                            "No configuration view for {provider_name}",
                        )))),
                    }),
            )
    }

    fn render_command_permission(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let always_allow_tool_actions = AssistantSettings::get_global(cx).always_allow_tool_actions;

        const HEADING: &str = "Allow running tools without asking for confirmation";

        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .gap_2()
            .flex_1()
            .child(Headline::new("General Settings").size(HeadlineSize::Small))
            .child(
                h_flex()
                    .p_2p5()
                    .rounded_sm()
                    .bg(cx.theme().colors().editor_background)
                    .border_1()
                    .border_color(cx.theme().colors().border)
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
        let tools_by_source = self.tools.tools_by_source(cx);
        let empty = Vec::new();

        const SUBHEADING: &str = "Connect to context servers via the Model Context Protocol either via Zed extensions or directly.";

        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .gap_2()
            .flex_1()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Headline::new("Context Servers (MCP)").size(HeadlineSize::Small))
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
                    .rounded_sm()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        h_flex()
                            .justify_between()
                            .px_2()
                            .py_1()
                            .when(are_tools_expanded, |element| {
                                element
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                            })
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Disclosure::new("tool-list-disclosure", are_tools_expanded)
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
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(h_flex().child(
                                Switch::new("context-server-switch", is_running.into()).on_click({
                                    let context_server_manager =
                                        self.context_server_manager.clone();
                                    let context_server = context_server.clone();
                                    move |state, _window, cx| match state {
                                        ToggleState::Unselected | ToggleState::Indeterminate => {
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
                            )),
                    )
                    .map(|parent| {
                        if !are_tools_expanded {
                            return parent;
                        }

                        parent.child(v_flex().children(tools.into_iter().enumerate().map(
                            |(ix, tool)| {
                                h_flex()
                                    .px_2()
                                    .py_1()
                                    .when(ix < tool_count - 1, |element| {
                                        element
                                            .border_b_1()
                                            .border_color(cx.theme().colors().border)
                                    })
                                    .child(Label::new(tool.name()))
                            },
                        )))
                    })
            }))
            .child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .child(
                        h_flex().w_full().child(
                            Button::new("add-context-server", "Add Context Server")
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
                                "Install Context Server Extensions",
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
        let providers = LanguageModelRegistry::read_global(cx).providers();

        v_flex()
            .id("assistant-configuration")
            .key_context("AgentConfiguration")
            .track_focus(&self.focus_handle(cx))
            .bg(cx.theme().colors().panel_background)
            .size_full()
            .overflow_y_scroll()
            .child(self.render_command_permission(cx))
            .child(Divider::horizontal().color(DividerColor::Border))
            .child(self.render_context_servers_section(cx))
            .child(Divider::horizontal().color(DividerColor::Border))
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .mt_1()
                    .gap_6()
                    .flex_1()
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(Headline::new("LLM Providers").size(HeadlineSize::Small))
                            .child(
                                Label::new("Add at least one provider to use AI-powered features.")
                                    .color(Color::Muted),
                            ),
                    )
                    .children(
                        providers
                            .into_iter()
                            .map(|provider| self.render_provider_configuration(&provider, cx)),
                    ),
            )
    }
}

mod add_context_server_modal;
mod configure_context_server_modal;
mod manage_profiles_modal;
mod tool_picker;

use std::{sync::Arc, time::Duration};

use assistant_settings::AssistantSettings;
use assistant_tool::{ToolSource, ToolWorkingSet};
use collections::HashMap;
use context_server::ContextServerId;
use fs::Fs;
use gpui::{
    Action, Animation, AnimationExt as _, AnyView, App, Entity, EventEmitter, FocusHandle,
    Focusable, ScrollHandle, Subscription, pulsating_between,
};
use language_model::{LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry};
use project::context_server_store::{ContextServerStatus, ContextServerStore};
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

pub struct AgentConfiguration {
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    configuration_views_by_provider: HashMap<LanguageModelProviderId, AnyView>,
    context_server_store: Entity<ContextServerStore>,
    expanded_context_server_tools: HashMap<ContextServerId, bool>,
    expanded_provider_configurations: HashMap<LanguageModelProviderId, bool>,
    tools: Entity<ToolWorkingSet>,
    _registry_subscription: Subscription,
    scroll_handle: ScrollHandle,
    scrollbar_state: ScrollbarState,
}

impl AgentConfiguration {
    pub fn new(
        fs: Arc<dyn Fs>,
        context_server_store: Entity<ContextServerStore>,
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
            context_server_store,
            expanded_context_server_tools: HashMap::default(),
            expanded_provider_configurations: HashMap::default(),
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
        self.expanded_provider_configurations.remove(provider_id);
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

impl Focusable for AgentConfiguration {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub enum AssistantConfigurationEvent {
    NewThread(Arc<dyn LanguageModelProvider>),
}

impl EventEmitter<AssistantConfigurationEvent> for AgentConfiguration {}

impl AgentConfiguration {
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

        let is_expanded = self
            .expanded_provider_configurations
            .get(&provider.id())
            .copied()
            .unwrap_or(true);

        v_flex()
            .pt_3()
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
                            .child(Label::new(provider_name.clone()).size(LabelSize::Large))
                            .when(provider.is_authenticated(cx) && !is_expanded, |parent| {
                                parent.child(Icon::new(IconName::Check).color(Color::Success))
                            }),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .when(provider.is_authenticated(cx), |parent| {
                                parent.child(
                                    Button::new(
                                        SharedString::from(format!("new-thread-{provider_id}")),
                                        "Start New Thread",
                                    )
                                    .icon_position(IconPosition::Start)
                                    .icon(IconName::Plus)
                                    .icon_size(IconSize::Small)
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
                            })
                            .child(
                                Disclosure::new(
                                    SharedString::from(format!(
                                        "provider-disclosure-{provider_id}"
                                    )),
                                    is_expanded,
                                )
                                .opened_icon(IconName::ChevronUp)
                                .closed_icon(IconName::ChevronDown)
                                .on_click(cx.listener({
                                    let provider_id = provider.id().clone();
                                    move |this, _event, _window, _cx| {
                                        let is_open = this
                                            .expanded_provider_configurations
                                            .entry(provider_id.clone())
                                            .or_insert(true);

                                        *is_open = !*is_open;
                                    }
                                })),
                            ),
                    ),
            )
            .when(is_expanded, |parent| match configuration_view {
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

        h_flex()
            .gap_4()
            .justify_between()
            .flex_wrap()
            .child(
                v_flex()
                    .gap_0p5()
                    .max_w_5_6()
                    .child(Label::new("Allow running editing tools without asking for confirmation"))
                    .child(
                        Label::new(
                            "The agent can perform potentially destructive actions without asking for your confirmation.",
                        )
                        .color(Color::Muted),
                    ),
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
            )
    }

    fn render_single_file_review(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let single_file_review = AssistantSettings::get_global(cx).single_file_review;

        h_flex()
            .gap_4()
            .justify_between()
            .flex_wrap()
            .child(
                v_flex()
                    .gap_0p5()
                    .max_w_5_6()
                    .child(Label::new("Enable single-file agent reviews"))
                    .child(
                        Label::new(
                            "Agent edits are also displayed in single-file editors for review.",
                        )
                        .color(Color::Muted),
                    ),
            )
            .child(
                Switch::new("single-file-review-switch", single_file_review.into())
                    .color(SwitchColor::Accent)
                    .on_click({
                        let fs = self.fs.clone();
                        move |state, _window, cx| {
                            let allow = state == &ToggleState::Selected;
                            update_settings_file::<AssistantSettings>(
                                fs.clone(),
                                cx,
                                move |settings, _| {
                                    settings.set_single_file_review(allow);
                                },
                            );
                        }
                    }),
            )
    }

    fn render_general_settings_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .pr(DynamicSpacing::Base20.rems(cx))
            .gap_2p5()
            .child(Headline::new("General Settings"))
            .child(self.render_command_permission(cx))
            .child(self.render_single_file_review(cx))
    }

    fn render_context_servers_section(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let context_server_ids = self.context_server_store.read(cx).all_server_ids().clone();

        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .pr(DynamicSpacing::Base20.rems(cx))
            .gap_2()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Headline::new("Model Context Protocol (MCP) Servers"))
                    .child(Label::new("Connect to context servers via the Model Context Protocol either via Zed extensions or directly.").color(Color::Muted)),
            )
            .children(
                context_server_ids.into_iter().map(|context_server_id| {
                    self.render_context_server(context_server_id, window, cx)
                }),
            )
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
                            .icon(IconName::Hammer)
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

    fn render_context_server(
        &self,
        context_server_id: ContextServerId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl use<> + IntoElement {
        let tools_by_source = self.tools.read(cx).tools_by_source(cx);
        let server_status = self
            .context_server_store
            .read(cx)
            .status_for_server(&context_server_id)
            .unwrap_or(ContextServerStatus::Stopped);

        let is_running = matches!(server_status, ContextServerStatus::Running);
        let item_id = SharedString::from(context_server_id.0.clone());

        let error = if let ContextServerStatus::Error(error) = server_status.clone() {
            Some(error)
        } else {
            None
        };

        let are_tools_expanded = self
            .expanded_context_server_tools
            .get(&context_server_id)
            .copied()
            .unwrap_or_default();

        let tools = tools_by_source
            .get(&ToolSource::ContextServer {
                id: context_server_id.0.clone().into(),
            })
            .map_or([].as_slice(), |tools| tools.as_slice());
        let tool_count = tools.len();

        let border_color = cx.theme().colors().border.opacity(0.6);
        let success_color = Color::Success.color(cx);

        let (status_indicator, tooltip_text) = match server_status {
            ContextServerStatus::Starting => (
                Indicator::dot()
                    .color(Color::Success)
                    .with_animation(
                        SharedString::from(format!("{}-starting", context_server_id.0.clone(),)),
                        Animation::new(Duration::from_secs(2))
                            .repeat()
                            .with_easing(pulsating_between(0.4, 1.)),
                        move |this, delta| this.color(success_color.alpha(delta).into()),
                    )
                    .into_any_element(),
                "Server is starting.",
            ),
            ContextServerStatus::Running => (
                Indicator::dot().color(Color::Success).into_any_element(),
                "Server is running.",
            ),
            ContextServerStatus::Error(_) => (
                Indicator::dot().color(Color::Error).into_any_element(),
                "Server has an error.",
            ),
            ContextServerStatus::Stopped => (
                Indicator::dot().color(Color::Muted).into_any_element(),
                "Server is stopped.",
            ),
        };

        v_flex()
            .id(item_id.clone())
            .border_1()
            .rounded_md()
            .border_color(border_color)
            .bg(cx.theme().colors().background.opacity(0.2))
            .overflow_hidden()
            .child(
                h_flex()
                    .p_1()
                    .justify_between()
                    .when(
                        error.is_some() || are_tools_expanded && tool_count > 1,
                        |element| element.border_b_1().border_color(border_color),
                    )
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(
                                Disclosure::new(
                                    "tool-list-disclosure",
                                    are_tools_expanded || error.is_some(),
                                )
                                .disabled(tool_count == 0)
                                .on_click(cx.listener({
                                    let context_server_id = context_server_id.clone();
                                    move |this, _event, _window, _cx| {
                                        let is_open = this
                                            .expanded_context_server_tools
                                            .entry(context_server_id.clone())
                                            .or_insert(false);

                                        *is_open = !*is_open;
                                    }
                                })),
                            )
                            .child(
                                div()
                                    .id(item_id.clone())
                                    .tooltip(Tooltip::text(tooltip_text))
                                    .child(status_indicator),
                            )
                            .child(Label::new(context_server_id.0.clone()).ml_0p5())
                            .when(is_running, |this| {
                                this.child(
                                    Label::new(if tool_count == 1 {
                                        SharedString::from("1 tool")
                                    } else {
                                        SharedString::from(format!("{} tools", tool_count))
                                    })
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                                )
                            }),
                    )
                    .child(
                        Switch::new("context-server-switch", is_running.into())
                            .color(SwitchColor::Accent)
                            .on_click({
                                let context_server_manager = self.context_server_store.clone();
                                let context_server_id = context_server_id.clone();
                                move |state, _window, cx| match state {
                                    ToggleState::Unselected | ToggleState::Indeterminate => {
                                        context_server_manager.update(cx, |this, cx| {
                                            this.stop_server(&context_server_id, cx).log_err();
                                        });
                                    }
                                    ToggleState::Selected => {
                                        context_server_manager.update(cx, |this, cx| {
                                            if let Some(server) =
                                                this.get_server(&context_server_id)
                                            {
                                                this.start_server(server, cx).log_err();
                                            }
                                        })
                                    }
                                }
                            }),
                    ),
            )
            .map(|parent| {
                if let Some(error) = error {
                    return parent.child(
                        h_flex()
                            .p_2()
                            .gap_2()
                            .items_start()
                            .child(
                                h_flex()
                                    .flex_none()
                                    .h(window.line_height() / 1.6_f32)
                                    .justify_center()
                                    .child(
                                        Icon::new(IconName::XCircle)
                                            .size(IconSize::XSmall)
                                            .color(Color::Error),
                                    ),
                            )
                            .child(
                                div().w_full().child(
                                    Label::new(error)
                                        .buffer_font(cx)
                                        .color(Color::Muted)
                                        .size(LabelSize::Small),
                                ),
                            ),
                    );
                }

                if !are_tools_expanded || tools.is_empty() {
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
    }
}

impl Render for AgentConfiguration {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(self.render_general_settings_section(cx))
                    .child(Divider::horizontal().color(DividerColor::Border))
                    .child(self.render_context_servers_section(window, cx))
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

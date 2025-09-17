mod add_llm_provider_modal;
mod configure_context_server_modal;
mod manage_profiles_modal;
mod tool_picker;

use std::{ops::Range, sync::Arc};

use agent_settings::AgentSettings;
use anyhow::Result;
use assistant_tool::{ToolSource, ToolWorkingSet};
use cloud_llm_client::{Plan, PlanV1, PlanV2};
use collections::HashMap;
use context_server::ContextServerId;
use editor::{Editor, SelectionEffects, scroll::Autoscroll};
use extension::ExtensionManifest;
use extension_host::ExtensionStore;
use fs::Fs;
use gpui::{
    Action, AnyView, App, AsyncWindowContext, Corner, Entity, EventEmitter, FocusHandle, Focusable,
    Hsla, ScrollHandle, Subscription, Task, WeakEntity,
};
use language::LanguageRegistry;
use language_model::{
    LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID,
};
use notifications::status_toast::{StatusToast, ToastIcon};
use project::{
    agent_server_store::{
        AgentServerCommand, AgentServerStore, AllAgentServersSettings, CLAUDE_CODE_NAME,
        CustomAgentServerSettings, GEMINI_NAME,
    },
    context_server_store::{ContextServerConfiguration, ContextServerStatus, ContextServerStore},
    project_settings::{ContextServerSettings, ProjectSettings},
};
use settings::{Settings, SettingsStore, update_settings_file};
use ui::{
    Chip, CommonAnimationExt, ContextMenu, Disclosure, Divider, DividerColor, ElevationIndex,
    Indicator, PopoverMenu, Switch, SwitchColor, SwitchField, Tooltip, WithScrollbar, prelude::*,
};
use util::ResultExt as _;
use workspace::{Workspace, create_and_open_local_file};
use zed_actions::ExtensionCategoryFilter;

pub(crate) use configure_context_server_modal::ConfigureContextServerModal;
pub(crate) use manage_profiles_modal::ManageProfilesModal;

use crate::{
    AddContextServer, ExternalAgent, NewExternalAgentThread,
    agent_configuration::add_llm_provider_modal::{AddLlmProviderModal, LlmCompatibleProvider},
    placeholder_command,
};

pub struct AgentConfiguration {
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    agent_server_store: Entity<AgentServerStore>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    configuration_views_by_provider: HashMap<LanguageModelProviderId, AnyView>,
    context_server_store: Entity<ContextServerStore>,
    expanded_context_server_tools: HashMap<ContextServerId, bool>,
    expanded_provider_configurations: HashMap<LanguageModelProviderId, bool>,
    tools: Entity<ToolWorkingSet>,
    _registry_subscription: Subscription,
    scroll_handle: ScrollHandle,
    _check_for_gemini: Task<()>,
}

impl AgentConfiguration {
    pub fn new(
        fs: Arc<dyn Fs>,
        agent_server_store: Entity<AgentServerStore>,
        context_server_store: Entity<ContextServerStore>,
        tools: Entity<ToolWorkingSet>,
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakEntity<Workspace>,
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

        cx.subscribe(&context_server_store, |_, _, _, cx| cx.notify())
            .detach();

        let mut this = Self {
            fs,
            language_registry,
            workspace,
            focus_handle,
            configuration_views_by_provider: HashMap::default(),
            agent_server_store,
            context_server_store,
            expanded_context_server_tools: HashMap::default(),
            expanded_provider_configurations: HashMap::default(),
            tools,
            _registry_subscription: registry_subscription,
            scroll_handle: ScrollHandle::new(),
            _check_for_gemini: Task::ready(()),
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
        let configuration_view = provider.configuration_view(
            language_model::ConfigurationViewTargetAgent::ZedAgent,
            window,
            cx,
        );
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
        let provider_id = provider.id().0;
        let provider_name = provider.name().0;
        let provider_id_string = SharedString::from(format!("provider-disclosure-{provider_id}"));

        let configuration_view = self
            .configuration_views_by_provider
            .get(&provider.id())
            .cloned();

        let is_expanded = self
            .expanded_provider_configurations
            .get(&provider.id())
            .copied()
            .unwrap_or(false);

        let is_zed_provider = provider.id() == ZED_CLOUD_PROVIDER_ID;
        let current_plan = if is_zed_provider {
            self.workspace
                .upgrade()
                .and_then(|workspace| workspace.read(cx).user_store().read(cx).plan())
        } else {
            None
        };

        let is_signed_in = self
            .workspace
            .read_with(cx, |workspace, _| {
                !workspace.client().status().borrow().is_signed_out()
            })
            .unwrap_or(false);

        v_flex()
            .w_full()
            .when(is_expanded, |this| this.mb_2())
            .child(
                div()
                    .opacity(0.6)
                    .px_2()
                    .child(Divider::horizontal().color(DividerColor::Border)),
            )
            .child(
                h_flex()
                    .map(|this| {
                        if is_expanded {
                            this.mt_2().mb_1()
                        } else {
                            this.my_2()
                        }
                    })
                    .w_full()
                    .justify_between()
                    .child(
                        h_flex()
                            .id(provider_id_string.clone())
                            .px_2()
                            .py_0p5()
                            .w_full()
                            .justify_between()
                            .rounded_sm()
                            .hover(|hover| hover.bg(cx.theme().colors().element_hover))
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_2()
                                    .child(
                                        Icon::new(provider.icon())
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .gap_1()
                                            .child(Label::new(provider_name.clone()))
                                            .map(|this| {
                                                if is_zed_provider && is_signed_in {
                                                    this.child(
                                                        self.render_zed_plan_info(current_plan, cx),
                                                    )
                                                } else {
                                                    this.when(
                                                        provider.is_authenticated(cx)
                                                            && !is_expanded,
                                                        |parent| {
                                                            parent.child(
                                                                Icon::new(IconName::Check)
                                                                    .color(Color::Success),
                                                            )
                                                        },
                                                    )
                                                }
                                            }),
                                    ),
                            )
                            .child(
                                Disclosure::new(provider_id_string, is_expanded)
                                    .opened_icon(IconName::ChevronUp)
                                    .closed_icon(IconName::ChevronDown),
                            )
                            .on_click(cx.listener({
                                let provider_id = provider.id();
                                move |this, _event, _window, _cx| {
                                    let is_expanded = this
                                        .expanded_provider_configurations
                                        .entry(provider_id.clone())
                                        .or_insert(false);

                                    *is_expanded = !*is_expanded;
                                }
                            })),
                    ),
            )
            .child(
                v_flex()
                    .w_full()
                    .px_2()
                    .gap_1()
                    .when(is_expanded, |parent| match configuration_view {
                        Some(configuration_view) => parent.child(configuration_view),
                        None => parent.child(Label::new(format!(
                            "No configuration view for {provider_name}",
                        ))),
                    })
                    .when(is_expanded && provider.is_authenticated(cx), |parent| {
                        parent.child(
                            Button::new(
                                SharedString::from(format!("new-thread-{provider_id}")),
                                "Start New Thread",
                            )
                            .full_width()
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::ModalSurface)
                            .icon_position(IconPosition::Start)
                            .icon(IconName::Thread)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
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
    }

    fn render_provider_configuration_section(
        &mut self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let providers = LanguageModelRegistry::read_global(cx).providers();

        v_flex()
            .w_full()
            .child(
                h_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .pr(DynamicSpacing::Base20.rems(cx))
                    .pb_0()
                    .mb_2p5()
                    .items_start()
                    .justify_between()
                    .child(
                        v_flex()
                            .w_full()
                            .gap_0p5()
                            .child(
                                h_flex()
                                    .pr_1()
                                    .w_full()
                                    .gap_2()
                                    .justify_between()
                                    .child(Headline::new("LLM Providers"))
                                    .child(
                                        PopoverMenu::new("add-provider-popover")
                                            .trigger(
                                                Button::new("add-provider", "Add Provider")
                                                    .icon_position(IconPosition::Start)
                                                    .icon(IconName::Plus)
                                                    .icon_size(IconSize::Small)
                                                    .icon_color(Color::Muted)
                                                    .label_size(LabelSize::Small),
                                            )
                                            .anchor(gpui::Corner::TopRight)
                                            .menu({
                                                let workspace = self.workspace.clone();
                                                move |window, cx| {
                                                    Some(ContextMenu::build(
                                                        window,
                                                        cx,
                                                        |menu, _window, _cx| {
                                                            menu.header("Compatible APIs").entry(
                                                                "OpenAI",
                                                                None,
                                                                {
                                                                    let workspace =
                                                                        workspace.clone();
                                                                    move |window, cx| {
                                                                        workspace
                                                        .update(cx, |workspace, cx| {
                                                            AddLlmProviderModal::toggle(
                                                                LlmCompatibleProvider::OpenAi,
                                                                workspace,
                                                                window,
                                                                cx,
                                                            );
                                                        })
                                                        .log_err();
                                                                    }
                                                                },
                                                            )
                                                        },
                                                    ))
                                                }
                                            }),
                                    ),
                            )
                            .child(
                                Label::new("Add at least one provider to use AI-powered features with Zed's native agent.")
                                    .color(Color::Muted),
                            ),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .pl(DynamicSpacing::Base08.rems(cx))
                    .pr(DynamicSpacing::Base20.rems(cx))
                    .children(
                        providers.into_iter().map(|provider| {
                            self.render_provider_configuration_block(&provider, cx)
                        }),
                    ),
            )
    }

    fn render_command_permission(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let always_allow_tool_actions = AgentSettings::get_global(cx).always_allow_tool_actions;
        let fs = self.fs.clone();

        SwitchField::new(
            "always-allow-tool-actions-switch",
            "Allow running commands without asking for confirmation",
            Some(
                "The agent can perform potentially destructive actions without asking for your confirmation.".into(),
            ),
            always_allow_tool_actions,
            move |state, _window, cx| {
                let allow = state == &ToggleState::Selected;
                update_settings_file::<AgentSettings>(fs.clone(), cx, move |settings, _| {
                    settings.set_always_allow_tool_actions(allow);
                });
            },
        )
    }

    fn render_single_file_review(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let single_file_review = AgentSettings::get_global(cx).single_file_review;
        let fs = self.fs.clone();

        SwitchField::new(
            "single-file-review",
            "Enable single-file agent reviews",
            Some("Agent edits are also displayed in single-file editors for review.".into()),
            single_file_review,
            move |state, _window, cx| {
                let allow = state == &ToggleState::Selected;
                update_settings_file::<AgentSettings>(fs.clone(), cx, move |settings, _| {
                    settings.set_single_file_review(allow);
                });
            },
        )
    }

    fn render_sound_notification(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let play_sound_when_agent_done = AgentSettings::get_global(cx).play_sound_when_agent_done;
        let fs = self.fs.clone();

        SwitchField::new(
            "sound-notification",
            "Play sound when finished generating",
            Some(
                "Hear a notification sound when the agent is done generating changes or needs your input.".into(),
            ),
            play_sound_when_agent_done,
            move |state, _window, cx| {
                let allow = state == &ToggleState::Selected;
                update_settings_file::<AgentSettings>(fs.clone(), cx, move |settings, _| {
                    settings.set_play_sound_when_agent_done(allow);
                });
            },
        )
    }

    fn render_modifier_to_send(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let use_modifier_to_send = AgentSettings::get_global(cx).use_modifier_to_send;
        let fs = self.fs.clone();

        SwitchField::new(
            "modifier-send",
            "Use modifier to submit a message",
            Some(
                "Make a modifier (cmd-enter on macOS, ctrl-enter on Linux or Windows) required to send messages.".into(),
            ),
            use_modifier_to_send,
            move |state, _window, cx| {
                let allow = state == &ToggleState::Selected;
                update_settings_file::<AgentSettings>(fs.clone(), cx, move |settings, _| {
                    settings.set_use_modifier_to_send(allow);
                });
            },
        )
    }

    fn render_general_settings_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .pr(DynamicSpacing::Base20.rems(cx))
            .gap_2p5()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(Headline::new("General Settings"))
            .child(self.render_command_permission(cx))
            .child(self.render_single_file_review(cx))
            .child(self.render_sound_notification(cx))
            .child(self.render_modifier_to_send(cx))
    }

    fn render_zed_plan_info(&self, plan: Option<Plan>, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(plan) = plan {
            let free_chip_bg = cx
                .theme()
                .colors()
                .editor_background
                .opacity(0.5)
                .blend(cx.theme().colors().text_accent.opacity(0.05));

            let pro_chip_bg = cx
                .theme()
                .colors()
                .editor_background
                .opacity(0.5)
                .blend(cx.theme().colors().text_accent.opacity(0.2));

            let (plan_name, label_color, bg_color) = match plan {
                Plan::V1(PlanV1::ZedFree) | Plan::V2(PlanV2::ZedFree) => {
                    ("Free", Color::Default, free_chip_bg)
                }
                Plan::V1(PlanV1::ZedProTrial) | Plan::V2(PlanV2::ZedProTrial) => {
                    ("Pro Trial", Color::Accent, pro_chip_bg)
                }
                Plan::V1(PlanV1::ZedPro) | Plan::V2(PlanV2::ZedPro) => {
                    ("Pro", Color::Accent, pro_chip_bg)
                }
            };

            Chip::new(plan_name.to_string())
                .bg_color(bg_color)
                .label_color(label_color)
                .into_any_element()
        } else {
            div().into_any_element()
        }
    }

    fn card_item_bg_color(&self, cx: &mut Context<Self>) -> Hsla {
        cx.theme().colors().background.opacity(0.25)
    }

    fn card_item_border_color(&self, cx: &mut Context<Self>) -> Hsla {
        cx.theme().colors().border.opacity(0.6)
    }

    fn render_context_servers_section(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let context_server_ids = self.context_server_store.read(cx).configured_server_ids();

        v_flex()
            .p(DynamicSpacing::Base16.rems(cx))
            .pr(DynamicSpacing::Base20.rems(cx))
            .gap_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Headline::new("Model Context Protocol (MCP) Servers"))
                    .child(
                        Label::new(
                            "All context servers connected through the Model Context Protocol.",
                        )
                        .color(Color::Muted),
                    ),
            )
            .map(|parent| {
                if context_server_ids.is_empty() {
                    parent.child(
                        h_flex()
                            .p_4()
                            .justify_center()
                            .border_1()
                            .border_dashed()
                            .border_color(cx.theme().colors().border.opacity(0.6))
                            .rounded_sm()
                            .child(
                                Label::new("No MCP servers added yet.")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                } else {
                    parent.children(context_server_ids.into_iter().map(|context_server_id| {
                        self.render_context_server(context_server_id, window, cx)
                    }))
                }
            })
            .child(
                h_flex()
                    .justify_between()
                    .gap_1p5()
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
                            .icon(IconName::ToolHammer)
                            .icon_size(IconSize::Small)
                            .icon_position(IconPosition::Start)
                            .on_click(|_event, window, cx| {
                                window.dispatch_action(
                                    zed_actions::Extensions {
                                        category_filter: Some(
                                            ExtensionCategoryFilter::ContextServers,
                                        ),
                                        id: None,
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
        let server_configuration = self
            .context_server_store
            .read(cx)
            .configuration_for_server(&context_server_id);

        let is_running = matches!(server_status, ContextServerStatus::Running);
        let item_id = SharedString::from(context_server_id.0.clone());
        let is_from_extension = server_configuration
            .as_ref()
            .map(|config| {
                matches!(
                    config.as_ref(),
                    ContextServerConfiguration::Extension { .. }
                )
            })
            .unwrap_or(false);

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

        let (source_icon, source_tooltip) = if is_from_extension {
            (
                IconName::ZedMcpExtension,
                "This MCP server was installed from an extension.",
            )
        } else {
            (
                IconName::ZedMcpCustom,
                "This custom MCP server was installed directly.",
            )
        };

        let (status_indicator, tooltip_text) = match server_status {
            ContextServerStatus::Starting => (
                Icon::new(IconName::LoadCircle)
                    .size(IconSize::XSmall)
                    .color(Color::Accent)
                    .with_keyed_rotate_animation(
                        SharedString::from(format!("{}-starting", context_server_id.0)),
                        3,
                    )
                    .into_any_element(),
                "Server is starting.",
            ),
            ContextServerStatus::Running => (
                Indicator::dot().color(Color::Success).into_any_element(),
                "Server is active.",
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

        let context_server_configuration_menu = PopoverMenu::new("context-server-config-menu")
            .trigger_with_tooltip(
                IconButton::new("context-server-config-menu", IconName::Settings)
                    .icon_color(Color::Muted)
                    .icon_size(IconSize::Small),
                Tooltip::text("Open MCP server options"),
            )
            .anchor(Corner::TopRight)
            .menu({
                let fs = self.fs.clone();
                let context_server_id = context_server_id.clone();
                let language_registry = self.language_registry.clone();
                let context_server_store = self.context_server_store.clone();
                let workspace = self.workspace.clone();
                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |menu, _window, _cx| {
                        menu.entry("Configure Server", None, {
                            let context_server_id = context_server_id.clone();
                            let language_registry = language_registry.clone();
                            let workspace = workspace.clone();
                            move |window, cx| {
                                ConfigureContextServerModal::show_modal_for_existing_server(
                                    context_server_id.clone(),
                                    language_registry.clone(),
                                    workspace.clone(),
                                    window,
                                    cx,
                                )
                                .detach_and_log_err(cx);
                            }
                        })
                        .separator()
                        .entry("Uninstall", None, {
                            let fs = fs.clone();
                            let context_server_id = context_server_id.clone();
                            let context_server_store = context_server_store.clone();
                            let workspace = workspace.clone();
                            move |_, cx| {
                                let is_provided_by_extension = context_server_store
                                    .read(cx)
                                    .configuration_for_server(&context_server_id)
                                    .as_ref()
                                    .map(|config| {
                                        matches!(
                                            config.as_ref(),
                                            ContextServerConfiguration::Extension { .. }
                                        )
                                    })
                                    .unwrap_or(false);

                                let uninstall_extension_task = match (
                                    is_provided_by_extension,
                                    resolve_extension_for_context_server(&context_server_id, cx),
                                ) {
                                    (true, Some((id, manifest))) => {
                                        if extension_only_provides_context_server(manifest.as_ref())
                                        {
                                            ExtensionStore::global(cx).update(cx, |store, cx| {
                                                store.uninstall_extension(id, cx)
                                            })
                                        } else {
                                            workspace.update(cx, |workspace, cx| {
                                                show_unable_to_uninstall_extension_with_context_server(workspace, context_server_id.clone(), cx);
                                            }).log_err();
                                            Task::ready(Ok(()))
                                        }
                                    }
                                    _ => Task::ready(Ok(())),
                                };

                                cx.spawn({
                                    let fs = fs.clone();
                                    let context_server_id = context_server_id.clone();
                                    async move |cx| {
                                        uninstall_extension_task.await?;
                                        cx.update(|cx| {
                                            update_settings_file::<ProjectSettings>(
                                                fs.clone(),
                                                cx,
                                                {
                                                    let context_server_id =
                                                        context_server_id.clone();
                                                    move |settings, _| {
                                                        settings
                                                            .context_servers
                                                            .remove(&context_server_id.0);
                                                    }
                                                },
                                            )
                                        })
                                    }
                                })
                                .detach_and_log_err(cx);
                            }
                        })
                    }))
                }
            });

        v_flex()
            .id(item_id.clone())
            .border_1()
            .rounded_md()
            .border_color(self.card_item_border_color(cx))
            .bg(self.card_item_bg_color(cx))
            .overflow_hidden()
            .child(
                h_flex()
                    .p_1()
                    .justify_between()
                    .when(
                        error.is_some() || are_tools_expanded && tool_count >= 1,
                        |element| {
                            element
                                .border_b_1()
                                .border_color(self.card_item_border_color(cx))
                        },
                    )
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w_0()
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
                                h_flex()
                                    .id(SharedString::from(format!("tooltip-{}", item_id)))
                                    .h_full()
                                    .w_3()
                                    .ml_1()
                                    .mr_1p5()
                                    .justify_center()
                                    .tooltip(Tooltip::text(tooltip_text))
                                    .child(status_indicator),
                            )
                            .child(Label::new(item_id).truncate())
                            .child(
                                div()
                                    .id("extension-source")
                                    .mt_0p5()
                                    .mx_1()
                                    .flex_none()
                                    .tooltip(Tooltip::text(source_tooltip))
                                    .child(
                                        Icon::new(source_icon)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
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
                        h_flex()
                            .gap_0p5()
                            .flex_none()
                            .child(context_server_configuration_menu)
                            .child(
                                Switch::new("context-server-switch", is_running.into())
                                    .color(SwitchColor::Accent)
                                    .on_click({
                                        let context_server_manager =
                                            self.context_server_store.clone();
                                        let fs = self.fs.clone();

                                        move |state, _window, cx| {
                                            let is_enabled = match state {
                                                ToggleState::Unselected
                                                | ToggleState::Indeterminate => {
                                                    context_server_manager.update(
                                                        cx,
                                                        |this, cx| {
                                                            this.stop_server(
                                                                &context_server_id,
                                                                cx,
                                                            )
                                                            .log_err();
                                                        },
                                                    );
                                                    false
                                                }
                                                ToggleState::Selected => {
                                                    context_server_manager.update(
                                                        cx,
                                                        |this, cx| {
                                                            if let Some(server) =
                                                                this.get_server(&context_server_id)
                                                            {
                                                                this.start_server(server, cx);
                                                            }
                                                        },
                                                    );
                                                    true
                                                }
                                            };
                                            update_settings_file::<ProjectSettings>(
                                                fs.clone(),
                                                cx,
                                                {
                                                    let context_server_id =
                                                        context_server_id.clone();

                                                    move |settings, _| {
                                                        settings
                                                            .context_servers
                                                            .entry(context_server_id.0)
                                                            .or_insert_with(|| {
                                                                ContextServerSettings::Extension {
                                                                    enabled: is_enabled,
                                                                    settings: serde_json::json!({}),
                                                                }
                                                            })
                                                            .set_enabled(is_enabled);
                                                    }
                                                },
                                            );
                                        }
                                    }),
                            ),
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
                    tools.iter().enumerate().map(|(ix, tool)| {
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

    fn render_agent_servers_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let custom_settings = cx
            .global::<SettingsStore>()
            .get::<AllAgentServersSettings>(None)
            .custom
            .clone();
        let user_defined_agents = self
            .agent_server_store
            .read(cx)
            .external_agents()
            .filter(|name| name.0 != GEMINI_NAME && name.0 != CLAUDE_CODE_NAME)
            .cloned()
            .collect::<Vec<_>>();
        let user_defined_agents = user_defined_agents
            .into_iter()
            .map(|name| {
                self.render_agent_server(
                    IconName::Ai,
                    name.clone(),
                    ExternalAgent::Custom {
                        name: name.clone().into(),
                        command: custom_settings
                            .get(&name.0)
                            .map(|settings| settings.command.clone())
                            .unwrap_or(placeholder_command()),
                    },
                    cx,
                )
                .into_any_element()
            })
            .collect::<Vec<_>>();

        v_flex()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .pr(DynamicSpacing::Base20.rems(cx))
                    .gap_2()
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(
                                h_flex()
                                    .pr_1()
                                    .w_full()
                                    .gap_2()
                                    .justify_between()
                                    .child(Headline::new("External Agents"))
                                    .child(
                                        Button::new("add-agent", "Add Agent")
                                            .icon_position(IconPosition::Start)
                                            .icon(IconName::Plus)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Muted)
                                            .label_size(LabelSize::Small)
                                            .on_click(
                                                move |_, window, cx| {
                                                    if let Some(workspace) = window.root().flatten() {
                                                        let workspace = workspace.downgrade();
                                                        window
                                                            .spawn(cx, async |cx| {
                                                                open_new_agent_servers_entry_in_settings_editor(
                                                                    workspace,
                                                                    cx,
                                                                ).await
                                                            })
                                                            .detach_and_log_err(cx);
                                                    }
                                                }
                                            ),
                                    )
                            )
                            .child(
                                Label::new(
                                    "All agents connected through the Agent Client Protocol.",
                                )
                                .color(Color::Muted),
                            ),
                    )
                    .child(self.render_agent_server(
                        IconName::AiGemini,
                        "Gemini CLI",
                        ExternalAgent::Gemini,
                        cx,
                    ))
                    .child(self.render_agent_server(
                        IconName::AiClaude,
                        "Claude Code",
                        ExternalAgent::ClaudeCode,
                        cx,
                    ))
                    .children(user_defined_agents),
            )
    }

    fn render_agent_server(
        &self,
        icon: IconName,
        name: impl Into<SharedString>,
        agent: ExternalAgent,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let name = name.into();
        h_flex()
            .p_1()
            .pl_2()
            .gap_1p5()
            .justify_between()
            .border_1()
            .rounded_md()
            .border_color(self.card_item_border_color(cx))
            .bg(self.card_item_bg_color(cx))
            .overflow_hidden()
            .child(
                h_flex()
                    .gap_1p5()
                    .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                    .child(Label::new(name.clone())),
            )
            .child(
                Button::new(
                    SharedString::from(format!("start_acp_thread-{name}")),
                    "Start New Thread",
                )
                .layer(ElevationIndex::ModalSurface)
                .label_size(LabelSize::Small)
                .icon(IconName::Thread)
                .icon_position(IconPosition::Start)
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .on_click(move |_, window, cx| {
                    window.dispatch_action(
                        NewExternalAgentThread {
                            agent: Some(agent.clone()),
                        }
                        .boxed_clone(),
                        cx,
                    );
                }),
            )
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
                div()
                    .size_full()
                    .child(
                        v_flex()
                            .id("assistant-configuration-content")
                            .track_scroll(&self.scroll_handle)
                            .size_full()
                            .overflow_y_scroll()
                            .child(self.render_general_settings_section(cx))
                            .child(self.render_agent_servers_section(cx))
                            .child(self.render_context_servers_section(window, cx))
                            .child(self.render_provider_configuration_section(cx)),
                    )
                    .vertical_scrollbar_for(self.scroll_handle.clone(), window, cx),
            )
    }
}

fn extension_only_provides_context_server(manifest: &ExtensionManifest) -> bool {
    manifest.context_servers.len() == 1
        && manifest.themes.is_empty()
        && manifest.icon_themes.is_empty()
        && manifest.languages.is_empty()
        && manifest.grammars.is_empty()
        && manifest.language_servers.is_empty()
        && manifest.slash_commands.is_empty()
        && manifest.snippets.is_none()
        && manifest.debug_locators.is_empty()
}

pub(crate) fn resolve_extension_for_context_server(
    id: &ContextServerId,
    cx: &App,
) -> Option<(Arc<str>, Arc<ExtensionManifest>)> {
    ExtensionStore::global(cx)
        .read(cx)
        .installed_extensions()
        .iter()
        .find(|(_, entry)| entry.manifest.context_servers.contains_key(&id.0))
        .map(|(id, entry)| (id.clone(), entry.manifest.clone()))
}

// This notification appears when trying to delete
// an MCP server extension that not only provides
// the server, but other things, too, like language servers and more.
fn show_unable_to_uninstall_extension_with_context_server(
    workspace: &mut Workspace,
    id: ContextServerId,
    cx: &mut App,
) {
    let workspace_handle = workspace.weak_handle();
    let context_server_id = id.clone();

    let status_toast = StatusToast::new(
        format!(
            "The {} extension provides more than just the MCP server. Proceed to uninstall anyway?",
            id.0
        ),
        cx,
        move |this, _cx| {
            let workspace_handle = workspace_handle.clone();

            this.icon(ToastIcon::new(IconName::Warning).color(Color::Warning))
                .dismiss_button(true)
                .action("Uninstall", move |_, _cx| {
                    if let Some((extension_id, _)) =
                        resolve_extension_for_context_server(&context_server_id, _cx)
                    {
                        ExtensionStore::global(_cx).update(_cx, |store, cx| {
                            store
                                .uninstall_extension(extension_id, cx)
                                .detach_and_log_err(cx);
                        });

                        workspace_handle
                            .update(_cx, |workspace, cx| {
                                let fs = workspace.app_state().fs.clone();
                                cx.spawn({
                                    let context_server_id = context_server_id.clone();
                                    async move |_workspace_handle, cx| {
                                        cx.update(|cx| {
                                            update_settings_file::<ProjectSettings>(
                                                fs,
                                                cx,
                                                move |settings, _| {
                                                    settings
                                                        .context_servers
                                                        .remove(&context_server_id.0);
                                                },
                                            );
                                        })?;
                                        anyhow::Ok(())
                                    }
                                })
                                .detach_and_log_err(cx);
                            })
                            .log_err();
                    }
                })
        },
    );

    workspace.toggle_status_toast(status_toast, cx);
}

async fn open_new_agent_servers_entry_in_settings_editor(
    workspace: WeakEntity<Workspace>,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let settings_editor = workspace
        .update_in(cx, |_, window, cx| {
            create_and_open_local_file(paths::settings_file(), window, cx, || {
                settings::initial_user_settings_content().as_ref().into()
            })
        })?
        .await?
        .downcast::<Editor>()
        .unwrap();

    settings_editor
        .downgrade()
        .update_in(cx, |item, window, cx| {
            let text = item.buffer().read(cx).snapshot(cx).text();

            let settings = cx.global::<SettingsStore>();

            let mut unique_server_name = None;
            let edits = settings.edits_for_update::<AllAgentServersSettings>(&text, |file| {
                let server_name: Option<SharedString> = (0..u8::MAX)
                    .map(|i| {
                        if i == 0 {
                            "your_agent".into()
                        } else {
                            format!("your_agent_{}", i).into()
                        }
                    })
                    .find(|name| !file.custom.contains_key(name));
                if let Some(server_name) = server_name {
                    unique_server_name = Some(server_name.clone());
                    file.custom.insert(
                        server_name,
                        CustomAgentServerSettings {
                            command: AgentServerCommand {
                                path: "path_to_executable".into(),
                                args: vec![],
                                env: Some(HashMap::default()),
                            },
                            default_mode: None,
                        },
                    );
                }
            });

            if edits.is_empty() {
                return;
            }

            let ranges = edits
                .iter()
                .map(|(range, _)| range.clone())
                .collect::<Vec<_>>();

            item.edit(edits, cx);
            if let Some((unique_server_name, buffer)) =
                unique_server_name.zip(item.buffer().read(cx).as_singleton())
            {
                let snapshot = buffer.read(cx).snapshot();
                if let Some(range) =
                    find_text_in_buffer(&unique_server_name, ranges[0].start, &snapshot)
                {
                    item.change_selections(
                        SelectionEffects::scroll(Autoscroll::newest()),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges(vec![range]);
                        },
                    );
                }
            }
        })
}

fn find_text_in_buffer(
    text: &str,
    start: usize,
    snapshot: &language::BufferSnapshot,
) -> Option<Range<usize>> {
    let chars = text.chars().collect::<Vec<char>>();

    let mut offset = start;
    let mut char_offset = 0;
    for c in snapshot.chars_at(start) {
        if char_offset >= chars.len() {
            break;
        }
        offset += 1;

        if c == chars[char_offset] {
            char_offset += 1;
        } else {
            char_offset = 0;
        }
    }

    if char_offset == chars.len() {
        Some(offset.saturating_sub(chars.len())..offset)
    } else {
        None
    }
}

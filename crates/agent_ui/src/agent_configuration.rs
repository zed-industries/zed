mod add_llm_provider_modal;
pub mod configure_context_server_modal;
mod configure_context_server_tools_modal;
mod manage_profiles_modal;
mod tool_picker;

use std::{ops::Range, sync::Arc};

use agent::ContextServerRegistry;
use anyhow::Result;
use client::zed_urls;
use cloud_llm_client::{Plan, PlanV1, PlanV2};
use collections::HashMap;
use context_server::ContextServerId;
use editor::{Editor, MultiBufferOffset, SelectionEffects, scroll::Autoscroll};
use extension::ExtensionManifest;
use extension_host::ExtensionStore;
use fs::Fs;
use gpui::{
    Action, AnyView, App, AsyncWindowContext, Corner, Entity, EventEmitter, FocusHandle, Focusable,
    ScrollHandle, Subscription, Task, WeakEntity,
};
use language::LanguageRegistry;
use language_model::{
    LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID,
};
use language_models::AllLanguageModelSettings;
use notifications::status_toast::{StatusToast, ToastIcon};
use project::{
    agent_server_store::{
        AgentServerStore, CLAUDE_CODE_NAME, CODEX_NAME, ExternalAgentServerName, GEMINI_NAME,
    },
    context_server_store::{ContextServerConfiguration, ContextServerStatus, ContextServerStore},
};
use settings::{Settings, SettingsStore, update_settings_file};
use ui::{
    Button, ButtonStyle, Chip, CommonAnimationExt, ContextMenu, ContextMenuEntry, Disclosure,
    Divider, DividerColor, ElevationIndex, IconName, IconPosition, IconSize, Indicator, LabelSize,
    PopoverMenu, Switch, Tooltip, WithScrollbar, prelude::*,
};
use util::ResultExt as _;
use workspace::{Workspace, create_and_open_local_file};
use zed_actions::{ExtensionCategoryFilter, OpenBrowser};

pub(crate) use configure_context_server_modal::ConfigureContextServerModal;
pub(crate) use configure_context_server_tools_modal::ConfigureContextServerToolsModal;
pub(crate) use manage_profiles_modal::ManageProfilesModal;

use crate::agent_configuration::add_llm_provider_modal::{
    AddLlmProviderModal, LlmCompatibleProvider,
};

pub struct AgentConfiguration {
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    agent_server_store: Entity<AgentServerStore>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    configuration_views_by_provider: HashMap<LanguageModelProviderId, AnyView>,
    context_server_store: Entity<ContextServerStore>,
    expanded_provider_configurations: HashMap<LanguageModelProviderId, bool>,
    context_server_registry: Entity<ContextServerRegistry>,
    _registry_subscription: Subscription,
    scroll_handle: ScrollHandle,
    _check_for_gemini: Task<()>,
}

impl AgentConfiguration {
    pub fn new(
        fs: Arc<dyn Fs>,
        agent_server_store: Entity<AgentServerStore>,
        context_server_store: Entity<ContextServerStore>,
        context_server_registry: Entity<ContextServerRegistry>,
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
            expanded_provider_configurations: HashMap::default(),
            context_server_registry,
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

enum AgentIcon {
    Name(IconName),
    Path(SharedString),
}

impl AgentConfiguration {
    fn render_section_title(
        &mut self,
        title: impl Into<SharedString>,
        description: impl Into<SharedString>,
        menu: AnyElement,
    ) -> impl IntoElement {
        h_flex()
            .p_4()
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
                            .flex_wrap()
                            .child(Headline::new(title.into()))
                            .child(menu),
                    )
                    .child(Label::new(description.into()).color(Color::Muted)),
            )
    }

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
                    .px_2()
                    .child(Divider::horizontal().color(DividerColor::BorderFaded)),
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
                                    .gap_1p5()
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
                            .style(ButtonStyle::Outlined)
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
                    })
                    .when(
                        is_expanded && is_removable_provider(&provider.id(), cx),
                        |this| {
                            this.child(
                                Button::new(
                                    SharedString::from(format!("delete-provider-{provider_id}")),
                                    "Remove Provider",
                                )
                                .full_width()
                                .style(ButtonStyle::Outlined)
                                .icon_position(IconPosition::Start)
                                .icon(IconName::Trash)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let provider = provider.clone();
                                    move |this, _event, window, cx| {
                                        this.delete_provider(provider.clone(), window, cx);
                                    }
                                })),
                            )
                        },
                    ),
            )
    }

    fn delete_provider(
        &mut self,
        provider: Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let fs = self.fs.clone();
        let provider_id = provider.id();

        cx.spawn_in(window, async move |_, cx| {
            cx.update(|_window, cx| {
                update_settings_file(fs.clone(), cx, {
                    let provider_id = provider_id.clone();
                    move |settings, _| {
                        if let Some(ref mut openai_compatible) = settings
                            .language_models
                            .as_mut()
                            .and_then(|lm| lm.openai_compatible.as_mut())
                        {
                            let key_to_remove: Arc<str> = Arc::from(provider_id.0.as_ref());
                            openai_compatible.remove(&key_to_remove);
                        }
                    }
                });
            })
            .log_err();

            cx.update(|_window, cx| {
                LanguageModelRegistry::global(cx).update(cx, {
                    let provider_id = provider_id.clone();
                    move |registry, cx| {
                        registry.unregister_provider(provider_id, cx);
                    }
                })
            })
            .log_err();

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn render_provider_configuration_section(
        &mut self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let providers = LanguageModelRegistry::read_global(cx).providers();

        let popover_menu = PopoverMenu::new("add-provider-popover")
            .trigger(
                Button::new("add-provider", "Add Provider")
                    .style(ButtonStyle::Outlined)
                    .icon_position(IconPosition::Start)
                    .icon(IconName::Plus)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .label_size(LabelSize::Small),
            )
            .menu({
                let workspace = self.workspace.clone();
                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |menu, _window, _cx| {
                        menu.header("Compatible APIs").entry("OpenAI", None, {
                            let workspace = workspace.clone();
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
                        })
                    }))
                }
            })
            .anchor(gpui::Corner::TopRight)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(2.0),
            });

        v_flex()
            .w_full()
            .child(self.render_section_title(
                "LLM Providers",
                "Add at least one provider to use AI-powered features with Zed's native agent.",
                popover_menu.into_any_element(),
            ))
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

    fn render_context_servers_section(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut context_server_ids = self
            .context_server_store
            .read(cx)
            .server_ids(cx)
            .into_iter()
            .collect::<Vec<_>>();

        // Sort context servers: ones without mcp-server- prefix first, then prefixed ones
        context_server_ids.sort_by(|a, b| {
            const MCP_PREFIX: &str = "mcp-server-";
            match (a.0.strip_prefix(MCP_PREFIX), b.0.strip_prefix(MCP_PREFIX)) {
                // If one has mcp-server- prefix and other doesn't, non-mcp comes first
                (Some(_), None) => std::cmp::Ordering::Greater,
                (None, Some(_)) => std::cmp::Ordering::Less,
                // If both have same prefix status, sort by appropriate key
                (Some(a), Some(b)) => a.cmp(b),
                (None, None) => a.0.cmp(&b.0),
            }
        });

        let add_server_popover = PopoverMenu::new("add-server-popover")
            .trigger(
                Button::new("add-server", "Add Server")
                    .style(ButtonStyle::Outlined)
                    .icon_position(IconPosition::Start)
                    .icon(IconName::Plus)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .label_size(LabelSize::Small),
            )
            .menu({
                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |menu, _window, _cx| {
                        menu.entry("Add Custom Server", None, {
                            |window, cx| {
                                window.dispatch_action(crate::AddContextServer.boxed_clone(), cx)
                            }
                        })
                        .entry("Install from Extensions", None, {
                            |window, cx| {
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
                            }
                        })
                    }))
                }
            })
            .anchor(gpui::Corner::TopRight)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(2.0),
            });

        v_flex()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(self.render_section_title(
                "Model Context Protocol (MCP) Servers",
                "All MCP servers connected directly or via a Zed extension.",
                add_server_popover.into_any_element(),
            ))
            .child(
                v_flex()
                    .pl_4()
                    .pb_4()
                    .pr_5()
                    .w_full()
                    .gap_1()
                    .map(|mut parent| {
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
                            for (index, context_server_id) in
                                context_server_ids.into_iter().enumerate()
                            {
                                if index > 0 {
                                    parent = parent.child(
                                        Divider::horizontal()
                                            .color(DividerColor::BorderFaded)
                                            .into_any_element(),
                                    );
                                }
                                parent = parent.child(self.render_context_server(
                                    context_server_id,
                                    window,
                                    cx,
                                ));
                            }
                            parent
                        }
                    }),
            )
    }

    fn render_context_server(
        &self,
        context_server_id: ContextServerId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl use<> + IntoElement {
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
        // Servers without a configuration can only be provided by extensions.
        let provided_by_extension = server_configuration.as_ref().is_none_or(|config| {
            matches!(
                config.as_ref(),
                ContextServerConfiguration::Extension { .. }
            )
        });

        let error = if let ContextServerStatus::Error(error) = server_status.clone() {
            Some(error)
        } else {
            None
        };

        let tool_count = self
            .context_server_registry
            .read(cx)
            .tools_for_server(&context_server_id)
            .count();

        let (source_icon, source_tooltip) = if provided_by_extension {
            (
                IconName::ZedSrcExtension,
                "This MCP server was installed from an extension.",
            )
        } else {
            (
                IconName::ZedSrcCustom,
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
        let is_remote = server_configuration
            .as_ref()
            .map(|config| matches!(config.as_ref(), ContextServerConfiguration::Http { .. }))
            .unwrap_or(false);
        let context_server_configuration_menu = PopoverMenu::new("context-server-config-menu")
            .trigger_with_tooltip(
                IconButton::new("context-server-config-menu", IconName::Settings)
                    .icon_color(Color::Muted)
                    .icon_size(IconSize::Small),
                Tooltip::text("Configure MCP Server"),
            )
            .anchor(Corner::TopRight)
            .menu({
                let fs = self.fs.clone();
                let context_server_id = context_server_id.clone();
                let language_registry = self.language_registry.clone();
                let workspace = self.workspace.clone();
                let context_server_registry = self.context_server_registry.clone();

                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |menu, _window, _cx| {
                        menu.entry("Configure Server", None, {
                            let context_server_id = context_server_id.clone();
                            let language_registry = language_registry.clone();
                            let workspace = workspace.clone();
                            move |window, cx| {
                                if is_remote {
                                    crate::agent_configuration::configure_context_server_modal::ConfigureContextServerModal::show_modal_for_existing_server(
                                        context_server_id.clone(),
                                        language_registry.clone(),
                                        workspace.clone(),
                                        window,
                                        cx,
                                    )
                                    .detach();
                                } else {
                                    ConfigureContextServerModal::show_modal_for_existing_server(
                                        context_server_id.clone(),
                                        language_registry.clone(),
                                        workspace.clone(),
                                        window,
                                        cx,
                                    )
                                    .detach();
                                }
                            }
                        }).when(tool_count > 0, |this| this.entry("View Tools", None, {
                            let context_server_id = context_server_id.clone();
                            let context_server_registry = context_server_registry.clone();
                            let workspace = workspace.clone();
                            move |window, cx| {
                                let context_server_id = context_server_id.clone();
                                workspace.update(cx, |workspace, cx| {
                                    ConfigureContextServerToolsModal::toggle(
                                        context_server_id,
                                        context_server_registry.clone(),
                                        workspace,
                                        window,
                                        cx,
                                    );
                                })
                                .ok();
                            }
                        }))
                        .separator()
                        .entry("Uninstall", None, {
                            let fs = fs.clone();
                            let context_server_id = context_server_id.clone();
                            let workspace = workspace.clone();
                            move |_, cx| {
                                let uninstall_extension_task = match (
                                    provided_by_extension,
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
                                            update_settings_file(
                                                fs.clone(),
                                                cx,
                                                {
                                                    let context_server_id =
                                                        context_server_id.clone();
                                                    move |settings, _| {
                                                        settings.project
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
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w_0()
                            .child(
                                h_flex()
                                    .id(SharedString::from(format!("tooltip-{}", item_id)))
                                    .h_full()
                                    .w_3()
                                    .mr_2()
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
                                .on_click({
                                    let context_server_manager = self.context_server_store.clone();
                                    let fs = self.fs.clone();

                                    move |state, _window, cx| {
                                        let is_enabled = match state {
                                            ToggleState::Unselected
                                            | ToggleState::Indeterminate => {
                                                context_server_manager.update(cx, |this, cx| {
                                                    this.stop_server(&context_server_id, cx)
                                                        .log_err();
                                                });
                                                false
                                            }
                                            ToggleState::Selected => {
                                                context_server_manager.update(cx, |this, cx| {
                                                    if let Some(server) =
                                                        this.get_server(&context_server_id)
                                                    {
                                                        this.start_server(server, cx);
                                                    }
                                                });
                                                true
                                            }
                                        };
                                        update_settings_file(fs.clone(), cx, {
                                            let context_server_id = context_server_id.clone();

                                            move |settings, _| {
                                                settings
                                                    .project
                                                    .context_servers
                                                    .entry(context_server_id.0)
                                                    .or_insert_with(|| {
                                                        settings::ContextServerSettingsContent::Extension {
                                                            enabled: is_enabled,
                                                            settings: serde_json::json!({}),
                                                        }
                                                    })
                                                    .set_enabled(is_enabled);
                                            }
                                        });
                                    }
                                }),
                        ),
                    ),
            )
            .map(|parent| {
                if let Some(error) = error {
                    return parent.child(
                        h_flex()
                            .gap_2()
                            .pr_4()
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
                parent
            })
    }

    fn render_agent_servers_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let agent_server_store = self.agent_server_store.read(cx);

        let user_defined_agents = agent_server_store
            .external_agents()
            .filter(|name| {
                name.0 != GEMINI_NAME && name.0 != CLAUDE_CODE_NAME && name.0 != CODEX_NAME
            })
            .cloned()
            .collect::<Vec<_>>();

        let user_defined_agents: Vec<_> = user_defined_agents
            .into_iter()
            .map(|name| {
                let icon = if let Some(icon_path) = agent_server_store.agent_icon(&name) {
                    AgentIcon::Path(icon_path)
                } else {
                    AgentIcon::Name(IconName::Ai)
                };
                (name, icon)
            })
            .collect();

        let add_agent_popover = PopoverMenu::new("add-agent-server-popover")
            .trigger(
                Button::new("add-agent", "Add Agent")
                    .style(ButtonStyle::Outlined)
                    .icon_position(IconPosition::Start)
                    .icon(IconName::Plus)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .label_size(LabelSize::Small),
            )
            .menu({
                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |menu, _window, _cx| {
                        menu.entry("Install from Extensions", None, {
                            |window, cx| {
                                window.dispatch_action(
                                    zed_actions::Extensions {
                                        category_filter: Some(
                                            ExtensionCategoryFilter::AgentServers,
                                        ),
                                        id: None,
                                    }
                                    .boxed_clone(),
                                    cx,
                                )
                            }
                        })
                        .entry("Add Custom Agent", None, {
                            move |window, cx| {
                                if let Some(workspace) = window.root().flatten() {
                                    let workspace = workspace.downgrade();
                                    window
                                        .spawn(cx, async |cx| {
                                            open_new_agent_servers_entry_in_settings_editor(
                                                workspace, cx,
                                            )
                                            .await
                                        })
                                        .detach_and_log_err(cx);
                                }
                            }
                        })
                        .separator()
                        .header("Learn More")
                        .item(
                            ContextMenuEntry::new("Agent Servers Docs")
                                .icon(IconName::ArrowUpRight)
                                .icon_color(Color::Muted)
                                .icon_position(IconPosition::End)
                                .handler({
                                    move |window, cx| {
                                        window.dispatch_action(
                                            Box::new(OpenBrowser {
                                                url: zed_urls::agent_server_docs(cx),
                                            }),
                                            cx,
                                        );
                                    }
                                }),
                        )
                        .item(
                            ContextMenuEntry::new("ACP Docs")
                                .icon(IconName::ArrowUpRight)
                                .icon_color(Color::Muted)
                                .icon_position(IconPosition::End)
                                .handler({
                                    move |window, cx| {
                                        window.dispatch_action(
                                            Box::new(OpenBrowser {
                                                url: "https://agentclientprotocol.com/".into(),
                                            }),
                                            cx,
                                        );
                                    }
                                }),
                        )
                    }))
                }
            })
            .anchor(gpui::Corner::TopRight)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(2.0),
            });

        v_flex()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                v_flex()
                    .child(self.render_section_title(
                        "External Agents",
                        "All agents connected through the Agent Client Protocol.",
                        add_agent_popover.into_any_element(),
                    ))
                    .child(
                        v_flex()
                            .p_4()
                            .pt_0()
                            .gap_2()
                            .child(self.render_agent_server(
                                AgentIcon::Name(IconName::AiClaude),
                                "Claude Code",
                                false,
                                cx,
                            ))
                            .child(Divider::horizontal().color(DividerColor::BorderFaded))
                            .child(self.render_agent_server(
                                AgentIcon::Name(IconName::AiOpenAi),
                                "Codex CLI",
                                false,
                                cx,
                            ))
                            .child(Divider::horizontal().color(DividerColor::BorderFaded))
                            .child(self.render_agent_server(
                                AgentIcon::Name(IconName::AiGemini),
                                "Gemini CLI",
                                false,
                                cx,
                            ))
                            .map(|mut parent| {
                                for (name, icon) in user_defined_agents {
                                    parent = parent
                                        .child(
                                            Divider::horizontal().color(DividerColor::BorderFaded),
                                        )
                                        .child(self.render_agent_server(icon, name, true, cx));
                                }
                                parent
                            }),
                    ),
            )
    }

    fn render_agent_server(
        &self,
        icon: AgentIcon,
        name: impl Into<SharedString>,
        external: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let name = name.into();
        let icon = match icon {
            AgentIcon::Name(icon_name) => Icon::new(icon_name)
                .size(IconSize::Small)
                .color(Color::Muted),
            AgentIcon::Path(icon_path) => Icon::from_external_svg(icon_path)
                .size(IconSize::Small)
                .color(Color::Muted),
        };

        let tooltip_id = SharedString::new(format!("agent-source-{}", name));
        let tooltip_message = format!("The {} agent was installed from an extension.", name);

        let agent_server_name = ExternalAgentServerName(name.clone());

        let uninstall_btn_id = SharedString::from(format!("uninstall-{}", name));
        let uninstall_button = IconButton::new(uninstall_btn_id, IconName::Trash)
            .icon_color(Color::Muted)
            .icon_size(IconSize::Small)
            .tooltip(Tooltip::text("Uninstall Agent Extension"))
            .on_click(cx.listener(move |this, _, _window, cx| {
                let agent_name = agent_server_name.clone();

                if let Some(ext_id) = this.agent_server_store.update(cx, |store, _cx| {
                    store.get_extension_id_for_agent(&agent_name)
                }) {
                    ExtensionStore::global(cx)
                        .update(cx, |store, cx| store.uninstall_extension(ext_id, cx))
                        .detach_and_log_err(cx);
                }
            }));

        h_flex()
            .gap_1()
            .justify_between()
            .child(
                h_flex()
                    .gap_1p5()
                    .child(icon)
                    .child(Label::new(name))
                    .when(external, |this| {
                        this.child(
                            div()
                                .id(tooltip_id)
                                .flex_none()
                                .tooltip(Tooltip::text(tooltip_message))
                                .child(
                                    Icon::new(IconName::ZedSrcExtension)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                    })
                    .child(
                        Icon::new(IconName::Check)
                            .color(Color::Success)
                            .size(IconSize::Small),
                    ),
            )
            .when(external, |this| this.child(uninstall_button))
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
                            .child(self.render_agent_servers_section(cx))
                            .child(self.render_context_servers_section(window, cx))
                            .child(self.render_provider_configuration_section(cx)),
                    )
                    .vertical_scrollbar_for(&self.scroll_handle, window, cx),
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
                                            update_settings_file(fs, cx, move |settings, _| {
                                                settings
                                                    .project
                                                    .context_servers
                                                    .remove(&context_server_id.0);
                                            });
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
            let edits = settings.edits_for_update(&text, |settings| {
                let server_name: Option<SharedString> = (0..u8::MAX)
                    .map(|i| {
                        if i == 0 {
                            "your_agent".into()
                        } else {
                            format!("your_agent_{}", i).into()
                        }
                    })
                    .find(|name| {
                        !settings
                            .agent_servers
                            .as_ref()
                            .is_some_and(|agent_servers| agent_servers.custom.contains_key(name))
                    });
                if let Some(server_name) = server_name {
                    unique_server_name = Some(server_name.clone());
                    settings
                        .agent_servers
                        .get_or_insert_default()
                        .custom
                        .insert(
                            server_name,
                            settings::CustomAgentServerSettings::Custom {
                                path: "path_to_executable".into(),
                                args: vec![],
                                env: Some(HashMap::default()),
                                default_mode: None,
                                default_model: None,
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

            item.edit(
                edits.into_iter().map(|(range, s)| {
                    (
                        MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                        s,
                    )
                }),
                cx,
            );
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
                            selections.select_ranges(vec![
                                MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                            ]);
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

// OpenAI-compatible providers are user-configured and can be removed,
// whereas built-in providers (like Anthropic, OpenAI, Google, etc.) can't.
//
// If in the future we have more "API-compatible-type" of providers,
// they should be included here as removable providers.
fn is_removable_provider(provider_id: &LanguageModelProviderId, cx: &App) -> bool {
    AllLanguageModelSettings::get_global(cx)
        .openai_compatible
        .contains_key(provider_id.0.as_ref())
}

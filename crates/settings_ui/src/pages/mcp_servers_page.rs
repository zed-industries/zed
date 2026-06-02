use std::sync::Arc;

use context_server::ContextServerId;
use extension_host::ExtensionStore;
use gpui::{Action as _, Entity, ScrollHandle, prelude::*};
use project::context_server_store::{
    ContextServerConfiguration, ContextServerStatus, ContextServerStore,
};
use settings::ContextServerSettingsContent;
use ui::{
    AiSettingItem, AiSettingItemSource, AiSettingItemStatus, ContextMenu, Divider, DividerColor,
    PopoverMenu, Switch, ToggleState, Tooltip, prelude::*,
};
use util::ResultExt as _;

use zed_actions::ExtensionCategoryFilter;

use crate::SettingsWindow;

pub(crate) fn render_mcp_servers_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let context_server_store = get_context_server_store(settings_window, cx);

    let server_list = if let Some(store) = context_server_store.as_ref() {
        let server_ids = store.read(cx).server_ids().to_vec();

        if server_ids.is_empty() {
            render_empty_state(cx)
        } else {
            render_server_list(&server_ids, store, cx)
        }
    } else {
        render_no_project_state(cx)
    };

    let add_server_popover = render_add_server_popover(settings_window, cx);

    v_flex()
        .id("mcp-servers-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(
            h_flex()
                .w_full()
                .justify_between()
                .items_center()
                .mb_4()
                .child(
                    v_flex()
                        .child(Label::new("MCP Servers").size(LabelSize::Large))
                        .child(
                            Label::new("Manage Model Context Protocol servers connected directly or via extensions.")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child(add_server_popover),
        )
        .child(server_list)
        .into_any_element()
}

fn get_context_server_store(
    settings_window: &SettingsWindow,
    cx: &App,
) -> Option<Entity<ContextServerStore>> {
    let original_window = settings_window.original_window.as_ref()?;
    let multi_workspace = original_window.read(cx).ok()?;
    let workspace = multi_workspace.workspaces().next()?;
    let project = workspace.read(cx).project().clone();
    Some(project.read(cx).context_server_store())
}

fn render_empty_state(cx: &App) -> AnyElement {
    h_flex()
        .p_4()
        .justify_center()
        .border_1()
        .border_dashed()
        .border_color(cx.theme().colors().border.opacity(0.6))
        .rounded_sm()
        .child(
            Label::new("No MCP servers added yet. Click \"Add Server\" to get started.")
                .color(Color::Muted)
                .size(LabelSize::Small),
        )
        .into_any_element()
}

fn render_no_project_state(cx: &App) -> AnyElement {
    h_flex()
        .p_4()
        .justify_center()
        .border_1()
        .border_dashed()
        .border_color(cx.theme().colors().border.opacity(0.6))
        .rounded_sm()
        .child(
            Label::new("No active project found. Open a workspace to manage MCP servers.")
                .color(Color::Muted)
                .size(LabelSize::Small),
        )
        .into_any_element()
}

fn render_server_list(
    server_ids: &[ContextServerId],
    store: &Entity<ContextServerStore>,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    v_flex()
        .w_full()
        .gap_1()
        .children(itertools::intersperse_with(
            server_ids.iter().map(|server_id| {
                render_context_server(server_id, store, cx).into_any_element()
            }),
            || {
                Divider::horizontal()
                    .color(DividerColor::BorderFaded)
                    .into_any_element()
            },
        ))
        .into_any_element()
}

fn render_context_server(
    context_server_id: &ContextServerId,
    store: &Entity<ContextServerStore>,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let server_status = store
        .read(cx)
        .status_for_server(context_server_id)
        .unwrap_or(ContextServerStatus::Stopped);
    let server_configuration = store.read(cx).configuration_for_server(context_server_id);

    let is_running = matches!(server_status, ContextServerStatus::Running);
    let item_id = SharedString::from(context_server_id.0.to_string());

    let provided_by_extension = server_configuration.as_ref().is_none_or(|config| {
        matches!(
            config.as_ref(),
            ContextServerConfiguration::Extension { .. }
        )
    });

    let display_name = if provided_by_extension {
        resolve_extension_display_name(context_server_id, cx).unwrap_or_else(|| item_id.clone())
    } else {
        item_id.clone()
    };

    let source = if provided_by_extension {
        AiSettingItemSource::Extension
    } else {
        AiSettingItemSource::Custom
    };

    let status = map_server_status(&server_status);

    let is_remote = server_configuration
        .as_ref()
        .map(|config| matches!(config.as_ref(), ContextServerConfiguration::Http { .. }))
        .unwrap_or(false);

    let should_show_logout = server_configuration.as_ref().is_some_and(|config| {
        matches!(config.as_ref(), ContextServerConfiguration::Http { .. })
            && !config.has_static_auth_header()
    });

    // ContextServerRegistry is per-project (not a global), so we skip tool count
    // in the settings UI for now.
    let tool_count = 0usize;

    let tool_label = if is_running && tool_count > 0 {
        Some(if tool_count == 1 {
            SharedString::from("1 tool")
        } else {
            SharedString::from(format!("{} tools", tool_count))
        })
    } else {
        None
    };

    // Build gear menu
    let gear_menu = render_gear_menu(
        context_server_id,
        store,
        provided_by_extension,
        should_show_logout,
        is_remote,
    );

    // Build toggle switch
    let toggle_switch = render_toggle_switch(context_server_id, store, is_running);

    // Build details (error/auth feedback)
    let details = render_status_details(
        &server_status,
        context_server_id,
        store,
        should_show_logout,
    );

    AiSettingItem::new(item_id, display_name, status, source)
        .action(gear_menu)
        .action(toggle_switch)
        .when_some(tool_label, |this, label| this.detail_label(label))
        .when_some(details, |this, details| this.details(details))
}

fn map_server_status(status: &ContextServerStatus) -> AiSettingItemStatus {
    match status {
        ContextServerStatus::Starting => AiSettingItemStatus::Starting,
        ContextServerStatus::Running => AiSettingItemStatus::Running,
        ContextServerStatus::Stopped => AiSettingItemStatus::Stopped,
        ContextServerStatus::Error(_) => AiSettingItemStatus::Error,
        ContextServerStatus::AuthRequired => AiSettingItemStatus::AuthRequired,
        ContextServerStatus::ClientSecretRequired { .. } => {
            AiSettingItemStatus::ClientSecretRequired
        }
        ContextServerStatus::Authenticating => AiSettingItemStatus::Authenticating,
    }
}

fn resolve_extension_display_name(
    id: &ContextServerId,
    cx: &App,
) -> Option<SharedString> {
    ExtensionStore::global(cx)
        .read(cx)
        .installed_extensions()
        .iter()
        .find(|(_, entry)| entry.manifest.context_servers.contains_key(&id.0))
        .map(|(_, entry)| {
            let name = entry.manifest.name.as_str();
            let stripped = name
                .strip_suffix(" MCP Server")
                .or_else(|| name.strip_suffix(" MCP"))
                .or_else(|| name.strip_suffix(" Context Server"))
                .unwrap_or(name);
            SharedString::from(stripped.to_string())
        })
}

fn render_gear_menu(
    context_server_id: &ContextServerId,
    store: &Entity<ContextServerStore>,
    provided_by_extension: bool,
    should_show_logout: bool,
    _is_remote: bool,
) -> impl IntoElement {
    let context_server_id = context_server_id.clone();
    let store = store.clone();

    PopoverMenu::new(SharedString::from(format!(
        "mcp-gear-{}",
        context_server_id.0
    )))
    .trigger_with_tooltip(
        IconButton::new(
            SharedString::from(format!("mcp-gear-btn-{}", context_server_id.0)),
            IconName::Settings,
        )
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small),
        Tooltip::text("Configure MCP Server"),
    )
    .anchor(gpui::Anchor::TopRight)
    .menu({
        move |window, cx| {
            let context_server_id = context_server_id.clone();
            let store = store.clone();

            Some(ContextMenu::build(window, cx, move |menu, _window, _cx| {
                menu.when(should_show_logout, |this| {
                    this.entry("Log Out", None, {
                        let store = store.clone();
                        let context_server_id = context_server_id.clone();
                        move |_window, cx| {
                            store.update(cx, |s, cx| {
                                s.logout_server(&context_server_id, cx).log_err();
                            });
                        }
                    })
                })
                .separator()
                .entry("Uninstall", None, {
                    let context_server_id = context_server_id.clone();
                    move |_, cx| {
                        uninstall_server(&context_server_id, provided_by_extension, cx);
                    }
                })
            }))
        }
    })
}

fn render_toggle_switch(
    context_server_id: &ContextServerId,
    store: &Entity<ContextServerStore>,
    is_running: bool,
) -> impl IntoElement {
    let context_server_id = context_server_id.clone();
    let store = store.clone();

    Switch::new(
        SharedString::from(format!("mcp-toggle-{}", context_server_id.0)),
        if is_running {
            ToggleState::Selected
        } else {
            ToggleState::Unselected
        },
    )
    .on_click({
        move |state, _window, cx| {
            let is_enabled = match state {
                ToggleState::Unselected | ToggleState::Indeterminate => {
                    store.update(cx, |this, cx| {
                        this.stop_server(&context_server_id, cx).log_err();
                    });
                    false
                }
                ToggleState::Selected => {
                    store.update(cx, |this, cx| {
                        if let Some(server) = this.get_server(&context_server_id) {
                            this.start_server(server, cx);
                        }
                    });
                    true
                }
            };

            let fs = <dyn fs::Fs>::global(cx);
            settings::update_settings_file(fs, cx, {
                let context_server_id = context_server_id.clone();
                move |settings, _| {
                    settings
                        .project
                        .context_servers
                        .entry(context_server_id.0.clone())
                        .or_insert_with(|| ContextServerSettingsContent::Extension {
                            enabled: is_enabled,
                            remote: false,
                            settings: serde_json::json!({}),
                        })
                        .set_enabled(is_enabled);
                }
            });
        }
    })
}

fn render_status_details(
    server_status: &ContextServerStatus,
    context_server_id: &ContextServerId,
    store: &Entity<ContextServerStore>,
    should_show_logout: bool,
) -> Option<AnyElement> {
    let feedback_base = || h_flex().py_1().min_w_0().w_full().gap_1().justify_between();

    match server_status {
        ContextServerStatus::Error(error) => {
            let store = store.clone();
            let context_server_id = context_server_id.clone();
            Some(
                feedback_base()
                    .child(
                        h_flex()
                            .pr_4()
                            .min_w_0()
                            .w_full()
                            .gap_2()
                            .child(
                                Icon::new(IconName::XCircle)
                                    .size(IconSize::XSmall)
                                    .color(Color::Error),
                            )
                            .child(
                                div().min_w_0().flex_1().child(
                                    Label::new(error.to_string())
                                        .color(Color::Muted)
                                        .size(LabelSize::Small),
                                ),
                            ),
                    )
                    .when(should_show_logout, |this| {
                        this.child(
                            Button::new("error-logout", "Log Out")
                                .style(ButtonStyle::Outlined)
                                .label_size(LabelSize::Small)
                                .on_click({
                                    let store = store.clone();
                                    let context_server_id = context_server_id.clone();
                                    move |_event, _window, cx| {
                                        store.update(cx, |s, cx| {
                                            s.logout_server(&context_server_id, cx).log_err();
                                        });
                                    }
                                }),
                        )
                    })
                    .into_any_element(),
            )
        }
        ContextServerStatus::AuthRequired => {
            let store = store.clone();
            let context_server_id = context_server_id.clone();
            Some(
                feedback_base()
                    .child(
                        h_flex()
                            .pr_4()
                            .min_w_0()
                            .w_full()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Info)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .child(
                                Label::new("Authenticate to connect this server")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .child(
                        Button::new("authenticate-server", "Authenticate")
                            .style(ButtonStyle::Outlined)
                            .label_size(LabelSize::Small)
                            .on_click({
                                move |_event, _window, cx| {
                                    store.update(cx, |s, cx| {
                                        s.authenticate_server(&context_server_id, cx).log_err();
                                    });
                                }
                            }),
                    )
                    .into_any_element(),
            )
        }
        ContextServerStatus::ClientSecretRequired { .. } => Some(
            feedback_base()
                .child(
                    h_flex()
                        .pr_4()
                        .min_w_0()
                        .w_full()
                        .gap_2()
                        .child(
                            Icon::new(IconName::Info)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(
                            Label::new("A client secret is required to connect this server")
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                )
                .into_any_element(),
        ),
        ContextServerStatus::Authenticating => Some(
            h_flex()
                .mt_1()
                .pr_4()
                .min_w_0()
                .w_full()
                .gap_2()
                .child(div().size_3().flex_shrink_0())
                .child(
                    Label::new("Authenticating…")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .into_any_element(),
        ),
        _ => None,
    }
}

fn render_add_server_popover(
    settings_window: &SettingsWindow,
    _cx: &App,
) -> impl IntoElement {
    let original_window = settings_window.original_window;

    PopoverMenu::new("add-mcp-server-popover")
        .trigger(
            Button::new("add-mcp-server", "Add Server")
                .style(ButtonStyle::Outlined)
                .start_icon(
                    Icon::new(IconName::Plus)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .label_size(LabelSize::Small),
        )
        .anchor(gpui::Anchor::TopRight)
        .menu({
            move |window, cx| {
                Some(ContextMenu::build(window, cx, move |menu, _window, _cx| {
                    menu.entry("Install from Extensions", None, {
                        move |_window, cx| {
                            if let Some(original_window) = original_window.as_ref() {
                                original_window
                                    .update(cx, |_, window, cx| {
                                        window.activate_window();
                                        window.dispatch_action(
                                            zed_actions::Extensions {
                                                category_filter: Some(
                                                    ExtensionCategoryFilter::ContextServers,
                                                ),
                                                id: None,
                                            }
                                            .boxed_clone(),
                                            cx,
                                        );
                                    })
                                    .log_err();
                            }
                        }
                    })
                }))
            }
        })
}

fn uninstall_server(
    context_server_id: &ContextServerId,
    provided_by_extension: bool,
    cx: &mut App,
) {
    if provided_by_extension {
        if let Some((ext_id, manifest)) = resolve_extension_for_context_server(context_server_id, cx)
        {
            if extension_only_provides_context_server(&manifest) {
                ExtensionStore::global(cx)
                    .update(cx, |store, cx| store.uninstall_extension(ext_id, cx))
                    .detach_and_log_err(cx);
            }
        }
    }

    let fs = <dyn fs::Fs>::global(cx);
    let context_server_id = context_server_id.clone();
    settings::update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .context_servers
            .remove(&context_server_id.0);
    });
}

fn resolve_extension_for_context_server(
    id: &ContextServerId,
    cx: &App,
) -> Option<(Arc<str>, Arc<extension::ExtensionManifest>)> {
    ExtensionStore::global(cx)
        .read(cx)
        .installed_extensions()
        .iter()
        .find(|(_, entry)| entry.manifest.context_servers.contains_key(&id.0))
        .map(|(id, entry)| (id.clone(), entry.manifest.clone()))
}

fn extension_only_provides_context_server(manifest: &extension::ExtensionManifest) -> bool {
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

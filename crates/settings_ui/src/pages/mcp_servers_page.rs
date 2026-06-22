use std::sync::Arc;

use collections::HashMap;
use context_server::ContextServerId;
use editor::Editor;
use extension_host::ExtensionStore;
use gpui::{Action as _, Entity, Focusable as _, ScrollHandle, WeakEntity, prelude::*};
use project::context_server_store::{
    ContextServerConfiguration, ContextServerStatus, ContextServerStore,
};
use project::project_settings::ContextServerSettings;
use settings::{ContextServerCommand, ContextServerSettingsContent, OAuthClientSettings};
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
    window: &mut Window,
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

    let add_server_popover = render_add_server_popover(settings_window, window, cx);

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
            server_ids
                .iter()
                .map(|server_id| render_context_server(server_id, store, cx).into_any_element()),
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

    // Determine the source from the configured settings rather than the runtime
    // configuration: a custom (Stdio/HTTP) server that is disabled or not yet
    // started has no runtime configuration, and must not be mistaken for an
    // extension-provided server.
    let provided_by_extension = store.read(cx).is_extension_provided(context_server_id, cx);

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

    // Build gear menu. Pre-fill "Configure Server" from the raw configured
    // settings (not the resolved runtime configuration) so the form is editable
    // even when the settings contain invalid data (e.g. an unparsable URL) or
    // the server is disabled / not yet started.
    let server_settings = store
        .read(cx)
        .settings_for_server(context_server_id)
        .cloned();
    let gear_menu = render_gear_menu(
        context_server_id,
        store,
        cx.entity().downgrade(),
        server_settings.clone(),
        provided_by_extension,
        should_show_logout,
    );

    // Build toggle switch
    let toggle_switch = render_toggle_switch(context_server_id, store, is_running);

    // Surface invalid settings (which prevent the server from starting at all)
    // ahead of runtime status feedback, so the misconfiguration is visible.
    let details = match settings_validation_error(server_settings.as_ref()) {
        Some(error) => Some(render_form_error(error).into_any_element()),
        None => render_status_details(&server_status, context_server_id, store, should_show_logout),
    };

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

fn resolve_extension_display_name(id: &ContextServerId, cx: &App) -> Option<SharedString> {
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
    settings_window: WeakEntity<SettingsWindow>,
    server_settings: Option<ContextServerSettings>,
    provided_by_extension: bool,
    should_show_logout: bool,
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
        .icon_size(IconSize::Small)
        .tab_index(0isize),
        Tooltip::text("Configure MCP Server"),
    )
    .anchor(gpui::Anchor::TopRight)
    .menu({
        move |window, cx| {
            let context_server_id = context_server_id.clone();
            let store = store.clone();
            let settings_window = settings_window.clone();
            let server_settings = server_settings.clone();

            Some(ContextMenu::build(window, cx, move |menu, _window, _cx| {
                menu.when(!provided_by_extension, |this| {
                    this.entry("Configure Server", None, {
                        let settings_window = settings_window.clone();
                        let context_server_id = context_server_id.clone();
                        let server_settings = server_settings.clone();
                        move |window, cx| {
                            let transport = match &server_settings {
                                Some(ContextServerSettings::Http { .. }) => McpTransport::Http,
                                _ => McpTransport::Stdio,
                            };
                            let existing = server_settings
                                .clone()
                                .map(|settings| (context_server_id.clone(), settings));
                            settings_window
                                .update(cx, |this, cx| {
                                    open_mcp_server_form(this, transport, existing, window, cx);
                                })
                                .log_err();
                        }
                    })
                })
                .when(should_show_logout, |this| {
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
                // Only show a divider when there is an entry above "Uninstall".
                // Extension servers have neither "Configure Server" nor "Log Out".
                .when(!provided_by_extension || should_show_logout, |this| {
                    this.separator()
                })
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
    .tab_index(0isize)
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
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let original_window = settings_window.original_window;
    // Stable handle so the button keeps focus state across renders and can show a
    // focus ring even when the page is opened (and the button auto-focused) via a
    // mouse click, where `focus_visible` styling is suppressed.
    let focus_handle = settings_window
        .mcp_add_server_focus_handle
        .clone()
        .tab_index(0)
        .tab_stop(true);
    let is_focused = focus_handle.is_focused(window);
    let border_color = if is_focused {
        cx.theme().colors().border_focused
    } else {
        gpui::transparent_black()
    };
    let settings_window = cx.entity().downgrade();

    let popover = PopoverMenu::new("add-mcp-server-popover")
        .trigger(
            Button::new("add-mcp-server", "Add Server")
                .style(ButtonStyle::Outlined)
                .track_focus(&focus_handle)
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
                let settings_window = settings_window.clone();
                Some(ContextMenu::build(window, cx, move |menu, _window, _cx| {
                    menu.entry("Add Local Server", None, {
                        let settings_window = settings_window.clone();
                        move |window, cx| {
                            settings_window
                                .update(cx, |this, cx| {
                                    open_mcp_server_form(
                                        this,
                                        McpTransport::Stdio,
                                        None,
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    })
                    .entry("Add Remote Server", None, {
                        let settings_window = settings_window.clone();
                        move |window, cx| {
                            settings_window
                                .update(cx, |this, cx| {
                                    open_mcp_server_form(
                                        this,
                                        McpTransport::Http,
                                        None,
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    })
                    .separator()
                    .entry("Install from Extensions", None, {
                        move |_window, cx| {
                            if let Some(original_window) = original_window.as_ref() {
                                cx.activate(true);
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
        });

    div()
        .rounded_md()
        .border_1()
        .border_color(border_color)
        .child(popover)
}

fn uninstall_server(
    context_server_id: &ContextServerId,
    provided_by_extension: bool,
    cx: &mut App,
) {
    if provided_by_extension {
        if let Some((ext_id, manifest)) =
            resolve_extension_for_context_server(context_server_id, cx)
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

// === Custom (Stdio/HTTP) MCP server add/edit form ===

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum McpTransport {
    /// Local server launched via stdin/stdout.
    Stdio,
    /// Remote server connected over HTTP.
    Http,
}

#[derive(Clone, Copy)]
enum McpKvKind {
    Env,
    Header,
}

impl McpKvKind {
    fn rows_mut(self, form: &mut McpServerForm) -> &mut Vec<KeyValueRow> {
        match self {
            McpKvKind::Env => &mut form.env,
            McpKvKind::Header => &mut form.headers,
        }
    }

    fn remove_id(self) -> &'static str {
        match self {
            McpKvKind::Env => "mcp-env-remove",
            McpKvKind::Header => "mcp-header-remove",
        }
    }

    fn add_id(self) -> &'static str {
        match self {
            McpKvKind::Env => "mcp-env-add",
            McpKvKind::Header => "mcp-header-add",
        }
    }
}

struct KeyValueRow {
    key: Entity<Editor>,
    value: Entity<Editor>,
}

/// Editor-backed state for the custom MCP server add/edit form.
pub(crate) struct McpServerForm {
    transport: McpTransport,
    /// `Some` when editing an existing server (used to remove the old entry on rename).
    original_id: Option<ContextServerId>,
    name: Entity<Editor>,
    command: Entity<Editor>,
    args: Entity<Editor>,
    url: Entity<Editor>,
    timeout: Entity<Editor>,
    oauth_client_id: Entity<Editor>,
    env: Vec<KeyValueRow>,
    headers: Vec<KeyValueRow>,
    error: Option<SharedString>,
}

impl McpServerForm {
    fn new(
        transport: McpTransport,
        existing: Option<(ContextServerId, ContextServerSettings)>,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> Self {
        let original_id = existing.as_ref().map(|(id, _)| id.clone());
        let settings = existing.map(|(_, settings)| settings);
        let name_initial = original_id.as_ref().map(|id| id.0.to_string());

        let mut command_initial = None;
        let mut args_initial = None;
        let mut url_initial = None;
        let mut timeout_initial = None;
        let mut oauth_initial = None;
        let mut env = Vec::new();
        let mut headers = Vec::new();

        // Pre-fill from the raw settings so invalid values (e.g. a malformed URL
        // the user typed directly into settings.json) still load into the form
        // for correction, rather than being dropped during resolution.
        if let Some(settings) = settings.as_ref() {
            match settings {
                ContextServerSettings::Stdio { command, .. } => {
                    command_initial = Some(command.path.to_string_lossy().to_string());
                    if !command.args.is_empty() {
                        args_initial = Some(command.args.join(" "));
                    }
                    timeout_initial = command.timeout.map(|timeout| timeout.to_string());
                    if let Some(env_map) = &command.env {
                        for (key, value) in sorted_pairs(env_map) {
                            env.push(new_kv_row(Some(&key), Some(&value), window, cx));
                        }
                    }
                }
                ContextServerSettings::Http {
                    url,
                    headers: header_map,
                    timeout,
                    oauth,
                    ..
                } => {
                    url_initial = Some(url.clone());
                    timeout_initial = timeout.map(|timeout| timeout.to_string());
                    for (key, value) in sorted_pairs(header_map) {
                        headers.push(new_kv_row(Some(&key), Some(&value), window, cx));
                    }
                    oauth_initial = oauth.as_ref().map(|oauth| oauth.client_id.clone());
                }
                ContextServerSettings::Extension { .. } => {}
            }
        }

        Self {
            transport,
            original_id,
            name: new_input("my-mcp-server", name_initial.as_deref(), window, cx),
            command: new_input("/path/to/server", command_initial.as_deref(), window, cx),
            args: new_input("--flag value", args_initial.as_deref(), window, cx),
            url: new_input(
                "https://example.com/mcp",
                url_initial.as_deref(),
                window,
                cx,
            ),
            timeout: new_input("60", timeout_initial.as_deref(), window, cx),
            oauth_client_id: new_input(
                "Optional OAuth client ID",
                oauth_initial.as_deref(),
                window,
                cx,
            ),
            env,
            headers,
            error: None,
        }
    }
}

fn sorted_pairs(map: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = map
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

fn new_input(
    placeholder: &str,
    initial: Option<&str>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> Entity<Editor> {
    let placeholder = placeholder.to_string();
    let initial = initial.map(|text| text.to_string());
    cx.new(|cx| {
        let mut editor = Editor::single_line(window, cx);
        editor.set_placeholder_text(placeholder.as_str(), window, cx);
        if let Some(text) = initial {
            editor.set_text(text, window, cx);
        }
        editor
    })
}

fn new_kv_row(
    key: Option<&str>,
    value: Option<&str>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> KeyValueRow {
    KeyValueRow {
        key: new_input("Key", key, window, cx),
        value: new_input("Value", value, window, cx),
    }
}

/// Creates the form state and pushes the form sub-page onto the stack.
pub(crate) fn open_mcp_server_form(
    settings_window: &mut SettingsWindow,
    transport: McpTransport,
    existing: Option<(ContextServerId, ContextServerSettings)>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let is_edit = existing.is_some();
    settings_window.mcp_server_form = Some(McpServerForm::new(transport, existing, window, cx));

    let title = if is_edit {
        "Configure MCP Server"
    } else {
        match transport {
            McpTransport::Stdio => "Add Local MCP Server",
            McpTransport::Http => "Add Remote MCP Server",
        }
    };

    settings_window.push_dynamic_sub_page(
        title,
        "Agent Configuration",
        Some("context_servers"),
        false,
        render_mcp_server_form_page,
        window,
        cx,
    );
}

fn render_mcp_server_form_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let Some(form) = settings_window.mcp_server_form.as_ref() else {
        return div().into_any_element();
    };
    let transport = form.transport;
    let error = form.error.clone();

    let fields = v_flex()
        .w_full()
        .gap_4()
        .child(render_form_field(
            settings_window,
            "Server Name",
            "Required. A unique name used to identify this MCP server.",
            &form.name,
            cx,
        ))
        .map(|this| match transport {
            McpTransport::Stdio => this
                .child(render_form_field(
                    settings_window,
                    "Command",
                    "Required. Path to the executable that launches the server.",
                    &form.command,
                    cx,
                ))
                .child(render_form_field(
                    settings_window,
                    "Arguments",
                    "Space-separated arguments passed to the command.",
                    &form.args,
                    cx,
                ))
                .child(render_kv_section(
                    settings_window,
                    "Environment Variables",
                    "Environment variables provided to the server process.",
                    &form.env,
                    McpKvKind::Env,
                    cx,
                ))
                .child(render_form_field(
                    settings_window,
                    "Timeout (seconds)",
                    "How long to wait for the server to respond before timing out.",
                    &form.timeout,
                    cx,
                )),
            McpTransport::Http => this
                .child(render_form_field(
                    settings_window,
                    "URL",
                    "Required. The base URL of the remote MCP server.",
                    &form.url,
                    cx,
                ))
                .child(render_kv_section(
                    settings_window,
                    "Headers",
                    "HTTP headers sent with each request to the server.",
                    &form.headers,
                    McpKvKind::Header,
                    cx,
                ))
                .child(render_form_field(
                    settings_window,
                    "Timeout (seconds)",
                    "How long to wait for the server to respond before timing out.",
                    &form.timeout,
                    cx,
                ))
                .child(render_form_field(
                    settings_window,
                    "OAuth Client ID",
                    "Optional OAuth client ID used to authenticate with the server.",
                    &form.oauth_client_id,
                    cx,
                )),
        })
        .when_some(error, |this, error| this.child(render_form_error(error)))
        .child(render_form_actions(cx));

    v_flex()
        .id("mcp-server-form-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(fields)
        .into_any_element()
}

fn input_box(editor: &Entity<Editor>, cx: &App) -> impl IntoElement {
    let colors = cx.theme().colors();
    // All form inputs share tab index 0, so tab order follows render (insertion)
    // order. Tracking the editor's focus handle makes the field a tab stop and
    // routes keyboard focus into the editor when tabbed to.
    let focus_handle = editor.focus_handle(cx).tab_index(0).tab_stop(true);
    h_flex()
        .min_w_64()
        .py_1()
        .px_2()
        .h_8()
        .rounded_md()
        .border_1()
        .border_color(colors.border)
        .bg(colors.editor_background)
        .track_focus(&focus_handle)
        .focus(|style| style.border_color(colors.border_focused))
        .child(editor.clone())
}

fn render_form_field(
    settings_window: &SettingsWindow,
    title: &'static str,
    description: &'static str,
    editor: &Entity<Editor>,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let control = input_box(editor, cx).into_any_element();
    crate::render_settings_item_layout(
        settings_window,
        title,
        description,
        control,
        None,
        None,
        None,
        false,
        cx,
    )
    .into_any_element()
}

fn render_kv_section(
    settings_window: &SettingsWindow,
    title: &'static str,
    description: &'static str,
    rows: &[KeyValueRow],
    kind: McpKvKind,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let control = v_flex()
        .min_w_64()
        .gap_2()
        .children(rows.iter().enumerate().map(|(ix, row)| {
            v_flex()
                .gap_1()
                .child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .child(input_box(&row.key, cx))
                        .child(
                            IconButton::new((kind.remove_id(), ix), IconName::Close)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .tooltip(Tooltip::text("Remove"))
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    if let Some(form) = this.mcp_server_form.as_mut() {
                                        let rows = kind.rows_mut(form);
                                        if ix < rows.len() {
                                            rows.remove(ix);
                                        }
                                    }
                                    cx.notify();
                                })),
                        ),
                )
                .child(input_box(&row.value, cx))
        }))
        .child(
            Button::new(kind.add_id(), "Add")
                .style(ButtonStyle::Outlined)
                .label_size(LabelSize::Small)
                .start_icon(
                    Icon::new(IconName::Plus)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    let row = new_kv_row(None, None, window, cx);
                    if let Some(form) = this.mcp_server_form.as_mut() {
                        kind.rows_mut(form).push(row);
                    }
                    cx.notify();
                })),
        )
        .into_any_element();

    crate::render_settings_item_layout(
        settings_window,
        title,
        description,
        control,
        None,
        None,
        None,
        false,
        cx,
    )
    .into_any_element()
}

fn render_form_error(error: SharedString) -> impl IntoElement {
    h_flex()
        .w_full()
        .gap_2()
        .items_start()
        .child(
            Icon::new(IconName::XCircle)
                .size(IconSize::Small)
                .color(Color::Error),
        )
        .child(Label::new(error).size(LabelSize::Small).color(Color::Error))
}

fn render_form_actions(cx: &mut Context<SettingsWindow>) -> impl IntoElement {
    h_flex()
        .w_full()
        .gap_2()
        .justify_end()
        .pt_2()
        .child(
            Button::new("mcp-form-cancel", "Cancel")
                .style(ButtonStyle::Subtle)
                .on_click(cx.listener(|this, _, window, cx| {
                    this.mcp_server_form = None;
                    this.pop_sub_page(window, cx);
                })),
        )
        .child(
            Button::new("mcp-form-save", "Save")
                .style(ButtonStyle::Filled)
                .on_click(cx.listener(|this, _, window, cx| {
                    save_mcp_server_form(this, window, cx);
                })),
        )
}

fn save_mcp_server_form(
    settings_window: &mut SettingsWindow,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let built = {
        let Some(form) = settings_window.mcp_server_form.as_ref() else {
            return;
        };
        build_settings_from_form(form, cx)
    };

    let (id, original_id, content) = match built {
        Ok(value) => value,
        Err(error) => {
            if let Some(form) = settings_window.mcp_server_form.as_mut() {
                form.error = Some(error);
            }
            cx.notify();
            return;
        }
    };

    // Reject names that would collide with a *different* existing server. This
    // covers both adding a new server and renaming an existing one (where the new
    // name must not clobber another server's configuration).
    let collides_with_other_server =
        get_context_server_store(settings_window, cx).is_some_and(|store| {
            name_collides_with_other_server(&id, original_id.as_ref(), store.read(cx).server_ids())
        });
    if collides_with_other_server {
        if let Some(form) = settings_window.mcp_server_form.as_mut() {
            form.error = Some(format!("A server named \"{}\" already exists.", id.0).into());
        }
        cx.notify();
        return;
    }

    let fs = <dyn fs::Fs>::global(cx);
    settings::update_settings_file(fs, cx, move |settings, _| {
        if let Some(original_id) = &original_id
            && original_id.0 != id.0
        {
            settings.project.context_servers.remove(&original_id.0);
        }
        settings
            .project
            .context_servers
            .insert(id.0.clone(), content);
    });

    settings_window.mcp_server_form = None;
    settings_window.pop_sub_page(window, cx);
}

/// Plain (editor-free) snapshot of the form's contents, so the validation /
/// build logic can be exercised without a GPUI context.
struct McpServerFormValues {
    transport: McpTransport,
    original_id: Option<ContextServerId>,
    name: String,
    command: String,
    args: String,
    url: String,
    timeout: String,
    oauth_client_id: String,
    env: Vec<(String, String)>,
    headers: Vec<(String, String)>,
}

fn build_settings_from_form(
    form: &McpServerForm,
    cx: &App,
) -> Result<
    (
        ContextServerId,
        Option<ContextServerId>,
        ContextServerSettingsContent,
    ),
    SharedString,
> {
    let values = McpServerFormValues {
        transport: form.transport,
        original_id: form.original_id.clone(),
        name: form.name.read(cx).text(cx),
        command: form.command.read(cx).text(cx),
        args: form.args.read(cx).text(cx),
        url: form.url.read(cx).text(cx),
        timeout: form.timeout.read(cx).text(cx),
        oauth_client_id: form.oauth_client_id.read(cx).text(cx),
        env: read_kv(&form.env, cx),
        headers: read_kv(&form.headers, cx),
    };
    build_settings_from_values(&values)
}

fn read_kv(rows: &[KeyValueRow], cx: &App) -> Vec<(String, String)> {
    rows.iter()
        .map(|row| (row.key.read(cx).text(cx), row.value.read(cx).text(cx)))
        .collect()
}

fn build_settings_from_values(
    values: &McpServerFormValues,
) -> Result<
    (
        ContextServerId,
        Option<ContextServerId>,
        ContextServerSettingsContent,
    ),
    SharedString,
> {
    let name = values.name.trim().to_string();
    if name.is_empty() {
        return Err("Server name is required.".into());
    }

    let timeout = parse_timeout(&values.timeout)?;

    let content = match values.transport {
        McpTransport::Stdio => {
            let command = values.command.trim().to_string();
            if command.is_empty() {
                return Err("Command is required.".into());
            }
            let args = values
                .args
                .split_whitespace()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>();
            let env = collect_kv(&values.env, "environment variable")?;
            ContextServerSettingsContent::Stdio {
                enabled: true,
                remote: false,
                command: ContextServerCommand {
                    path: command.into(),
                    args,
                    env: (!env.is_empty()).then_some(env),
                    timeout,
                },
            }
        }
        McpTransport::Http => {
            let url = values.url.trim().to_string();
            if url.is_empty() {
                return Err("URL is required.".into());
            }
            // Validate the URL on save (a deliberate action) rather than on every
            // render, so a clearly invalid URL is reported to the user instead of
            // being silently written and failing later when the server starts.
            if let Err(error) = url::Url::parse(&url) {
                return Err(format!("Invalid URL: {error}").into());
            }
            let headers = collect_kv(&values.headers, "header")?;
            let oauth_client_id = values.oauth_client_id.trim().to_string();
            let oauth = (!oauth_client_id.is_empty()).then(|| OAuthClientSettings {
                client_id: oauth_client_id,
                client_secret: None,
            });
            ContextServerSettingsContent::Http {
                enabled: true,
                url,
                headers,
                timeout,
                oauth,
            }
        }
    };

    Ok((
        ContextServerId(name.into()),
        values.original_id.clone(),
        content,
    ))
}

/// Returns a human-readable error when a server's configured settings are
/// invalid in a way that prevents it from starting (currently: an HTTP server
/// whose URL cannot be parsed). Used to surface misconfiguration in the list.
fn settings_validation_error(settings: Option<&ContextServerSettings>) -> Option<SharedString> {
    match settings? {
        ContextServerSettings::Http { url, .. } if url::Url::parse(url).is_err() => {
            Some("Invalid URL in settings.".into())
        }
        _ => None,
    }
}

/// Returns whether saving under `id` would overwrite a *different* existing
/// server. Editing a server in place (`id == original_id`) is allowed.
fn name_collides_with_other_server(
    id: &ContextServerId,
    original_id: Option<&ContextServerId>,
    existing_ids: &[ContextServerId],
) -> bool {
    original_id.is_none_or(|original| original.0 != id.0)
        && existing_ids.iter().any(|existing| existing.0 == id.0)
}

fn parse_timeout(text: &str) -> Result<Option<u64>, SharedString> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    text.parse::<u64>()
        .map(Some)
        .map_err(|_| "Timeout must be a positive whole number of seconds.".into())
}

fn collect_kv(
    rows: &[(String, String)],
    label: &str,
) -> Result<HashMap<String, String>, SharedString> {
    let mut map = HashMap::default();
    for (key, value) in rows {
        let key = key.trim().to_string();
        if key.is_empty() {
            continue;
        }
        if map.contains_key(&key) {
            return Err(format!("Duplicate {label} \"{key}\".").into());
        }
        map.insert(key, value.clone());
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(transport: McpTransport) -> McpServerFormValues {
        McpServerFormValues {
            transport,
            original_id: None,
            name: "my-server".into(),
            command: String::new(),
            args: String::new(),
            url: String::new(),
            timeout: String::new(),
            oauth_client_id: String::new(),
            env: Vec::new(),
            headers: Vec::new(),
        }
    }

    fn id(name: &str) -> ContextServerId {
        ContextServerId(name.into())
    }

    #[test]
    fn parse_timeout_handles_empty_and_invalid() {
        assert_eq!(parse_timeout(""), Ok(None));
        assert_eq!(parse_timeout("   "), Ok(None));
        assert_eq!(parse_timeout("60"), Ok(Some(60)));
        assert_eq!(parse_timeout("  90  "), Ok(Some(90)));
        assert!(parse_timeout("abc").is_err());
        assert!(parse_timeout("-5").is_err());
        assert!(parse_timeout("1.5").is_err());
    }

    #[test]
    fn requires_server_name() {
        let mut values = values(McpTransport::Stdio);
        values.name = "   ".into();
        values.command = "/bin/server".into();
        assert_eq!(
            build_settings_from_values(&values).unwrap_err().as_ref(),
            "Server name is required."
        );
    }

    #[test]
    fn requires_command_for_local_server() {
        let values = values(McpTransport::Stdio);
        assert_eq!(
            build_settings_from_values(&values).unwrap_err().as_ref(),
            "Command is required."
        );
    }

    #[test]
    fn requires_url_for_remote_server() {
        let values = values(McpTransport::Http);
        assert_eq!(
            build_settings_from_values(&values).unwrap_err().as_ref(),
            "URL is required."
        );
    }

    #[test]
    fn rejects_invalid_url() {
        let mut values = values(McpTransport::Http);
        values.url = "not a url".into();
        let error = build_settings_from_values(&values).unwrap_err();
        assert!(
            error.starts_with("Invalid URL"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn rejects_invalid_timeout() {
        let mut values = values(McpTransport::Stdio);
        values.command = "/bin/server".into();
        values.timeout = "soon".into();
        assert_eq!(
            build_settings_from_values(&values).unwrap_err().as_ref(),
            "Timeout must be a positive whole number of seconds."
        );
    }

    #[test]
    fn rejects_duplicate_environment_variables() {
        let mut values = values(McpTransport::Stdio);
        values.command = "/bin/server".into();
        values.env = vec![("FOO".into(), "1".into()), ("FOO".into(), "2".into())];
        assert_eq!(
            build_settings_from_values(&values).unwrap_err().as_ref(),
            "Duplicate environment variable \"FOO\"."
        );
    }

    #[test]
    fn rejects_duplicate_headers() {
        let mut values = values(McpTransport::Http);
        values.url = "https://example.com/mcp".into();
        values.headers = vec![
            ("Authorization".into(), "a".into()),
            ("Authorization".into(), "b".into()),
        ];
        assert_eq!(
            build_settings_from_values(&values).unwrap_err().as_ref(),
            "Duplicate header \"Authorization\"."
        );
    }

    #[test]
    fn builds_local_server() {
        let mut values = values(McpTransport::Stdio);
        values.name = "  local  ".into();
        values.command = "/usr/bin/server".into();
        values.args = "--flag  value".into();
        values.timeout = "30".into();
        // Empty values are kept, but rows with a blank key are ignored.
        values.env = vec![
            ("KEY".into(), "VALUE".into()),
            ("EMPTY".into(), String::new()),
            ("   ".into(), "ignored".into()),
        ];

        let (id, original_id, content) = build_settings_from_values(&values).unwrap();
        assert_eq!(id.0.as_ref(), "local");
        assert_eq!(original_id, None);

        let expected_env = HashMap::from_iter([
            ("KEY".to_string(), "VALUE".to_string()),
            ("EMPTY".to_string(), String::new()),
        ]);
        assert_eq!(
            content,
            ContextServerSettingsContent::Stdio {
                enabled: true,
                remote: false,
                command: ContextServerCommand {
                    path: "/usr/bin/server".into(),
                    args: vec!["--flag".into(), "value".into()],
                    env: Some(expected_env),
                    timeout: Some(30),
                },
            }
        );
    }

    #[test]
    fn builds_remote_server() {
        let mut values = values(McpTransport::Http);
        values.name = "remote".into();
        values.url = "https://example.com/mcp".into();
        values.oauth_client_id = "client-123".into();
        values.headers = vec![("Authorization".into(), "Bearer token".into())];

        let (id, _, content) = build_settings_from_values(&values).unwrap();
        assert_eq!(id.0.as_ref(), "remote");

        let expected_headers =
            HashMap::from_iter([("Authorization".to_string(), "Bearer token".to_string())]);
        assert_eq!(
            content,
            ContextServerSettingsContent::Http {
                enabled: true,
                url: "https://example.com/mcp".into(),
                headers: expected_headers,
                timeout: None,
                oauth: Some(OAuthClientSettings {
                    client_id: "client-123".into(),
                    client_secret: None,
                }),
            }
        );
    }

    #[test]
    fn flags_invalid_url_in_settings() {
        let http = |url: &str| ContextServerSettings::Http {
            enabled: true,
            url: url.into(),
            headers: HashMap::default(),
            timeout: None,
            oauth: None,
        };
        assert_eq!(
            settings_validation_error(Some(&http("not a url")))
                .unwrap()
                .as_ref(),
            "Invalid URL in settings."
        );
        assert!(settings_validation_error(Some(&http("https://example.com/mcp"))).is_none());
        assert!(settings_validation_error(None).is_none());
    }

    #[test]
    fn name_collision_covers_new_and_rename() {
        let existing = vec![id("foo"), id("bar")];

        // New server taking an existing name collides.
        assert!(name_collides_with_other_server(&id("foo"), None, &existing));
        // New server with a free name is fine.
        assert!(!name_collides_with_other_server(
            &id("baz"),
            None,
            &existing
        ));
        // Editing a server in place is allowed even though the name "exists".
        assert!(!name_collides_with_other_server(
            &id("foo"),
            Some(&id("foo")),
            &existing
        ));
        // Renaming onto a different server's name collides.
        assert!(name_collides_with_other_server(
            &id("bar"),
            Some(&id("foo")),
            &existing
        ));
        // Renaming to a free name is fine.
        assert!(!name_collides_with_other_server(
            &id("baz"),
            Some(&id("foo")),
            &existing
        ));
    }
}

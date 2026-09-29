use std::{ops::Range, time::Duration};

use collections::{HashMap, HashSet};
use editor::{Editor, EditorElement, EditorStyle};
use feature_flags::FeatureFlagAppExt as _;
use fs::Fs;
use futures::{FutureExt as _, future::Either};
use gpui::{
    App, BackgroundExecutor, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    KeyContext, Render, RenderOnce, ScrollHandle, SharedString, Task, TextStyle,
    UniformListScrollHandle, WeakEntity, Window, point, prelude::*, uniform_list,
};
use project::{
    context_server_store::{
        ContextServerStore, RegistrySettingsOwnership, registry_settings_ownership,
        registry_user_settings_destination_is_active,
    },
    mcp_registry_store::{
        McpRegistryInputDescriptor, McpRegistryInstallationOption, McpRegistryInstallationSource,
        McpRegistryStore, ServerResponse, delete_server_secrets, registry_credential_is_referenced,
        write_server_secrets,
    },
    project_settings::{ContextServerSettings, ProjectSettings},
};
use settings::{
    ContextServerSettingsContent, McpRegistryServerSettings, Settings, SettingsStore,
    update_settings_file_with_completion,
};
use theme_settings::ThemeSettings;
use ui::{
    ButtonStyle, CommonAnimationExt, ContextMenu, ContextMenuEntry, KeyBinding, Modal, ModalFooter,
    ModalHeader, PopoverMenu, ScrollableHandle, Section, ToggleButtonGroup, ToggleButtonGroupSize,
    ToggleButtonGroupStyle, ToggleButtonSimple, Tooltip, WithScrollbar, prelude::*,
};
use util::ResultExt as _;
use uuid::Uuid;
use workspace::{
    DismissDecision, ModalView, Workspace,
    item::{Item, ItemEvent},
    notifications::NotifyResultExt as _,
};

const SEARCH_DEBOUNCE: Duration = Duration::from_millis(300);
const INSTALL_OPERATION_FOREGROUND_GRACE: Duration = Duration::from_secs(2);
const MCP_REGISTRY_ABOUT_URL: &str = "https://modelcontextprotocol.io/registry/about";

enum ForegroundOperation<'a> {
    Completed(anyhow::Result<()>),
    Continuing(futures::future::LocalBoxFuture<'a, anyhow::Result<()>>),
}

async fn wait_for_foreground_operation<'a>(
    operation: futures::future::LocalBoxFuture<'a, anyhow::Result<()>>,
    executor: BackgroundExecutor,
) -> ForegroundOperation<'a> {
    match futures::future::select(
        operation,
        executor
            .timer(INSTALL_OPERATION_FOREGROUND_GRACE)
            .boxed_local(),
    )
    .await
    {
        Either::Left((result, _)) => ForegroundOperation::Completed(result),
        Either::Right(((), operation)) => ForegroundOperation::Continuing(operation),
    }
}

async fn complete_registry_server_operation(
    registry_store: Entity<McpRegistryStore>,
    server_name: String,
    operation: futures::future::LocalBoxFuture<'_, anyhow::Result<()>>,
    cx: &gpui::AsyncApp,
) -> anyhow::Result<()> {
    let result = operation.await;
    cx.update(|cx| {
        registry_store.update(cx, |store, cx| {
            store.finish_server_operation(&server_name, cx);
        });
    });
    result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegistryFilter {
    All,
    Installed,
    NotInstalled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegistryInstallStatus {
    NotInstalled,
    InstalledRegistry,
    InstalledOther,
}

enum InstallationDetailsState {
    Ready {
        server: ServerResponse,
        options: Vec<McpRegistryInstallationOption>,
    },
}

#[derive(IntoElement)]
struct McpRegistryCard {
    children: Vec<AnyElement>,
}

impl McpRegistryCard {
    fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for McpRegistryCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for McpRegistryCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().w_full().child(
            v_flex()
                .p_3()
                .mt_4()
                .w_full()
                .min_h(rems_from_px(94.0_f32))
                .gap_2()
                .bg(cx.theme().colors().elevated_surface_background.opacity(0.5))
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .rounded_md()
                .children(self.children),
        )
    }
}

pub struct McpRegistryPage {
    workspace: WeakEntity<Workspace>,
    registry_store: Entity<McpRegistryStore>,
    context_server_store: Option<Entity<ContextServerStore>>,
    list: UniformListScrollHandle,
    registry_servers: Vec<ServerResponse>,
    filtered_registry_indices: Vec<usize>,
    active_query: Option<String>,
    next_cursor: Option<String>,
    is_fetching: bool,
    fetch_error: Option<SharedString>,
    list_generation: u64,
    pending_list_fetch: Option<Task<()>>,
    installed_statuses: HashMap<String, RegistryInstallStatus>,
    query_editor: Entity<Editor>,
    query_debounce_task: Option<Task<()>>,
    installation_details: HashMap<String, InstallationDetailsState>,
    pending_server_operations: HashSet<String>,
    operation_errors: HashMap<String, SharedString>,
    filter: RegistryFilter,
    _subscriptions: Vec<gpui::Subscription>,
}

impl McpRegistryPage {
    pub fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let weak_workspace = cx.weak_entity();
        let context_server_store = workspace.project().read(cx).context_server_store();
        cx.new(move |cx| {
            let registry_store = McpRegistryStore::global(cx);
            let query_editor = cx.new(|cx| {
                let mut input = Editor::single_line(window, cx);
                input.set_placeholder_text("Search MCP servers...", window, cx);
                input
            });
            cx.subscribe(&query_editor, Self::on_query_change).detach();

            let mut subscriptions = vec![
                cx.observe(&registry_store, |this, _, cx| {
                    this.reload_registry_servers(cx);
                }),
                cx.observe_global::<SettingsStore>(|this, cx| {
                    this.filter_registry_servers(cx);
                }),
            ];
            subscriptions.push(cx.observe(&context_server_store, |this, _, cx| {
                this.filter_registry_servers(cx);
            }));

            let mut this = Self {
                workspace: weak_workspace,
                registry_store,
                context_server_store: Some(context_server_store),
                list: UniformListScrollHandle::new(),
                registry_servers: Vec::new(),
                filtered_registry_indices: Vec::new(),
                active_query: None,
                next_cursor: None,
                is_fetching: false,
                fetch_error: None,
                list_generation: 0,
                pending_list_fetch: None,
                installed_statuses: HashMap::default(),
                query_editor,
                query_debounce_task: None,
                installation_details: HashMap::default(),
                pending_server_operations: HashSet::default(),
                operation_errors: HashMap::default(),
                filter: RegistryFilter::All,
                _subscriptions: subscriptions,
            };

            this.search_registry(None, cx);

            this
        })
    }

    fn reload_registry_servers(&mut self, cx: &mut Context<Self>) {
        self.filter_registry_servers(cx);
    }

    fn refresh_installed_statuses(&mut self, cx: &mut Context<Self>) {
        self.installed_statuses.clear();
        for (id, settings) in &ProjectSettings::get_global(cx).context_servers {
            let status = match settings {
                ContextServerSettings::Registry { .. } => RegistryInstallStatus::InstalledRegistry,
                ContextServerSettings::Stdio { .. }
                | ContextServerSettings::Http { .. }
                | ContextServerSettings::Extension { .. } => RegistryInstallStatus::InstalledOther,
            };
            self.installed_statuses.insert(id.to_string(), status);
        }
        if let Some(context_server_store) = self.context_server_store.as_ref() {
            let context_server_store = context_server_store.read(cx);
            for id in context_server_store.server_ids() {
                if context_server_store.is_server_configured_locally(id) {
                    self.installed_statuses
                        .insert(id.0.to_string(), RegistryInstallStatus::InstalledOther);
                } else {
                    self.installed_statuses
                        .entry(id.0.to_string())
                        .or_insert(RegistryInstallStatus::InstalledOther);
                }
            }
        }
    }

    fn install_status(&self, id: &str) -> RegistryInstallStatus {
        self.installed_statuses
            .get(id)
            .copied()
            .unwrap_or(RegistryInstallStatus::NotInstalled)
    }

    fn search_query(&self, cx: &App) -> Option<String> {
        let search = self.query_editor.read(cx).text(cx);
        let search = search.trim();
        (!search.is_empty()).then(|| search.to_string())
    }

    fn filter_registry_servers(&mut self, cx: &mut Context<Self>) {
        self.refresh_installed_statuses(cx);
        let filter = self.filter;
        let installed_statuses = self.installed_statuses.clone();
        let search = self.search_query(cx).map(|query| query.to_lowercase());
        if filter == RegistryFilter::Installed {
            let mut server_names = self
                .registry_servers
                .iter()
                .map(|server| server.name().to_string())
                .collect::<HashSet<_>>();
            let cached_installed_servers = {
                let store = self.registry_store.read(cx);
                installed_statuses
                    .iter()
                    .filter(|(_, status)| **status != RegistryInstallStatus::NotInstalled)
                    .filter_map(|(name, _)| store.cached_server(name))
                    .filter(|server| {
                        search.as_ref().is_none_or(|query| {
                            server.name().to_lowercase().contains(query.as_str())
                        })
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            };
            for server in cached_installed_servers {
                if server_names.insert(server.name().to_string()) {
                    self.registry_servers.push(server);
                }
            }
        }

        self.filtered_registry_indices = self
            .registry_servers
            .iter()
            .enumerate()
            .filter(|(_, server)| {
                let install_status = installed_statuses
                    .get(server.name())
                    .copied()
                    .unwrap_or(RegistryInstallStatus::NotInstalled);
                match filter {
                    RegistryFilter::All => true,
                    RegistryFilter::Installed => {
                        install_status != RegistryInstallStatus::NotInstalled
                    }
                    RegistryFilter::NotInstalled => {
                        install_status == RegistryInstallStatus::NotInstalled
                    }
                }
            })
            .map(|(index, _)| index)
            .collect();

        cx.notify();
    }

    fn scroll_to_top(&mut self, cx: &mut Context<Self>) {
        self.list.set_offset(point(px(0.), px(0.)));
        cx.notify();
    }

    fn on_query_change(
        &mut self,
        _: Entity<Editor>,
        event: &editor::EditorEvent,
        cx: &mut Context<Self>,
    ) {
        if !matches!(event, editor::EditorEvent::Edited { .. }) {
            return;
        }

        let query = self.search_query(cx);
        self.query_debounce_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SEARCH_DEBOUNCE).await;
            this.update(cx, |this, cx| {
                this.query_debounce_task = None;
                this.scroll_to_top(cx);
                this.search_registry(query, cx);
            })
            .log_err();
        }));
    }

    fn retry(&mut self, cx: &mut Context<Self>) {
        self.refresh(cx);
    }

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        let query = self.search_query(cx);
        self.installation_details.clear();
        self.search_registry(query, cx);
    }

    fn search_registry(&mut self, query: Option<String>, cx: &mut Context<Self>) {
        let query = query.and_then(|query| {
            let query = query.trim();
            (!query.is_empty()).then(|| query.to_owned())
        });
        self.list_generation = self.list_generation.wrapping_add(1);
        self.pending_list_fetch.take();
        self.registry_servers.clear();
        self.filtered_registry_indices.clear();
        self.next_cursor = None;
        self.active_query = query.clone();
        self.is_fetching = false;
        self.fetch_error = None;
        self.fetch_registry_page(query, None, cx);
    }

    fn load_more(&mut self, cx: &mut Context<Self>) {
        if self.is_fetching || self.fetch_error.is_some() {
            return;
        }
        let Some(cursor) = self.next_cursor.clone() else {
            return;
        };
        self.fetch_registry_page(self.active_query.clone(), Some(cursor), cx);
    }

    fn fetch_registry_page(
        &mut self,
        query: Option<String>,
        cursor: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let generation = self.list_generation;
        let task = self
            .registry_store
            .read(cx)
            .fetch_server_list_page(query, cursor, cx);
        self.is_fetching = true;
        self.fetch_error = None;
        cx.notify();

        self.pending_list_fetch = Some(cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                if this.list_generation != generation {
                    return;
                }

                this.pending_list_fetch = None;
                this.is_fetching = false;
                match result {
                    Ok(response) => {
                        let mut known_versions = this
                            .registry_servers
                            .iter()
                            .map(|server| (server.name().to_owned(), server.version().to_owned()))
                            .collect::<HashSet<_>>();
                        for server in response.servers {
                            let key = (server.name().to_owned(), server.version().to_owned());
                            if known_versions.insert(key) {
                                this.registry_servers.push(server);
                            }
                        }
                        this.next_cursor = response
                            .metadata
                            .next_cursor
                            .filter(|cursor| !cursor.is_empty());
                        this.fetch_error = None;
                    }
                    Err(error) => {
                        this.fetch_error = Some(format!("{error:#}").into());
                    }
                }
                this.filter_registry_servers(cx);
            })
            .log_err();
        }));
    }

    fn render_search(&self, cx: &mut Context<Self>) -> Div {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BufferSearchBar");

        h_flex()
            .key_context(key_context)
            .h_8()
            .min_w(rems_from_px(384.0_f32))
            .flex_1()
            .pl_1p5()
            .pr_2()
            .gap_2()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(Icon::new(IconName::MagnifyingGlass).color(Color::Muted))
            .child(self.render_text_input(&self.query_editor, cx))
    }

    fn render_text_input(
        &self,
        editor: &Entity<Editor>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            line_height: relative(1.3),
            ..Default::default()
        };

        EditorElement::new(
            editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_search = self.search_query(cx).is_some();
        let is_fetching = self.is_fetching;
        let fetch_error = self.fetch_error.clone();

        let message = if is_fetching {
            "Loading MCP Registry..."
        } else if fetch_error.is_some() {
            "Failed to load the MCP Registry. Please check your connection and try again."
        } else {
            match self.filter {
                RegistryFilter::All => {
                    if has_search {
                        "No MCP servers match your search."
                    } else {
                        "No MCP servers available."
                    }
                }
                RegistryFilter::Installed => {
                    if has_search {
                        "No installed MCP servers match your search."
                    } else {
                        "No installed MCP servers."
                    }
                }
                RegistryFilter::NotInstalled => {
                    if has_search {
                        "No uninstalled MCP servers match your search."
                    } else {
                        "No uninstalled MCP servers."
                    }
                }
            }
        };

        h_flex()
            .py_4()
            .min_w_0()
            .w_full()
            .gap_1p5()
            .items_start()
            .when(fetch_error.is_some(), |this| {
                this.child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
            })
            .when(is_fetching, |this| {
                this.child(
                    Icon::new(IconName::LoadCircle)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .with_rotate_animation(3),
                )
            })
            .child(
                v_flex()
                    .min_w_0()
                    .flex_1()
                    .gap_1()
                    .child(Label::new(message))
                    .when_some(fetch_error.clone(), |this, fetch_error| {
                        this.child(
                            Label::new(fetch_error)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
            .when_some(fetch_error, |this, _| {
                this.child(
                    Button::new("retry-mcp-registry", "Retry")
                        .style(ButtonStyle::Outlined)
                        .size(ButtonSize::Compact)
                        .on_click(cx.listener(|this, _, _, cx| this.retry(cx))),
                )
            })
    }

    fn render_servers(
        &mut self,
        range: Range<usize>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<McpRegistryCard> {
        let should_load_more = self.filter != RegistryFilter::Installed
            && range.end >= self.filtered_registry_indices.len().saturating_sub(4)
            && self.next_cursor.is_some()
            && !self.is_fetching
            && self.fetch_error.is_none();
        if should_load_more {
            self.load_more(cx);
        }

        range
            .map(|index| {
                let Some(server_index) = self.filtered_registry_indices.get(index).copied() else {
                    return self.render_missing_server();
                };
                let Some(server) = self.registry_servers.get(server_index) else {
                    return self.render_missing_server();
                };
                self.render_registry_server(server, cx)
            })
            .collect()
    }

    fn render_missing_server(&self) -> McpRegistryCard {
        McpRegistryCard::new().child(
            Label::new("Missing registry entry.")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
    }

    fn render_registry_server(
        &self,
        server: &ServerResponse,
        cx: &mut Context<Self>,
    ) -> McpRegistryCard {
        let install_status = self.install_status(server.name());
        let has_server_name = !server.name().trim().is_empty();
        let installation_warning =
            self.operation_errors
                .get(server.name())
                .cloned()
                .or_else(|| {
                    if !has_server_name {
                        return Some(SharedString::from("Missing server ID"));
                    }
                    match self.installation_details.get(server.name()) {
                        Some(InstallationDetailsState::Ready { options, .. })
                            if options.is_empty() =>
                        {
                            Some(SharedString::from("No supported installation method"))
                        }
                        Some(InstallationDetailsState::Ready { .. }) | None => None,
                    }
                });
        let display_name = server
            .title()
            .filter(|title| !title.trim().is_empty())
            .or_else(|| has_server_name.then_some(server.name()))
            .unwrap_or("Unknown MCP server");
        let version: SharedString = if server.version().trim().is_empty() {
            "Unknown version".into()
        } else {
            format!("v{}", server.version()).into()
        };

        let repository_button = server
            .repository()
            .filter(|repository| !repository.url.trim().is_empty())
            .map(|repository| {
                let repository_url = repository.url.clone();
                let repository_for_tooltip = repository_url.clone();
                let icon = match repository.source.as_str() {
                    "github" => IconName::Github,
                    "gitlab" => IconName::Gitlab,
                    "bitbucket" => IconName::Bitbucket,
                    "codeberg" => IconName::Codeberg,
                    "gitea" => IconName::Gitea,
                    "forgejo" => IconName::Forgejo,
                    _ => IconName::Code,
                };
                IconButton::new(
                    SharedString::from(format!("mcp-repository-{}", server.name())),
                    icon,
                )
                .icon_size(IconSize::Small)
                .tooltip(move |_, cx| {
                    Tooltip::with_meta(
                        "Visit Server Repository",
                        None,
                        repository_for_tooltip.clone(),
                        cx,
                    )
                })
                .on_click(move |_, _, cx| cx.open_url(&repository_url))
            });

        let website_button = server
            .website()
            .filter(|website| !website.trim().is_empty())
            .map(|website| {
                let website = website.to_string();
                let website_for_tooltip = website.clone();
                IconButton::new(
                    SharedString::from(format!("mcp-website-{}", server.name())),
                    IconName::Link,
                )
                .icon_size(IconSize::Small)
                .tooltip(move |_, cx| {
                    Tooltip::with_meta(
                        "Visit Server Website",
                        None,
                        website_for_tooltip.clone(),
                        cx,
                    )
                })
                .on_click(move |_, _, cx| cx.open_url(&website))
            });

        McpRegistryCard::new()
            .child(
                h_flex()
                    .min_w_0()
                    .justify_between()
                    .gap_3()
                    .child(
                        h_flex()
                            .min_w_0()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Blocks)
                                    .size(IconSize::Medium)
                                    .color(Color::Muted),
                            )
                            .child(
                                Headline::new(SharedString::from(display_name.to_string()))
                                    .size(HeadlineSize::Small),
                            )
                            .child(Label::new(version).color(Color::Muted))
                            .when_some(installation_warning, |this, warning| {
                                this.child(
                                    Label::new(warning)
                                        .size(LabelSize::Small)
                                        .color(Color::Warning),
                                )
                            }),
                    )
                    .child(self.install_control(server, install_status, cx)),
            )
            .child(
                h_flex()
                    .min_w_0()
                    .gap_2()
                    .justify_between()
                    .child(
                        Label::new(if server.description().trim().is_empty() {
                            SharedString::from("No description provided.")
                        } else {
                            SharedString::from(server.description().to_string())
                        })
                        .size(LabelSize::Small)
                        .truncate(),
                    )
                    .child(
                        h_flex()
                            .min_w_0()
                            .gap_1()
                            .child(
                                Label::new(if has_server_name {
                                    format!("ID: {}", server.name())
                                } else {
                                    "ID unavailable".to_string()
                                })
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .truncate(),
                            )
                            .when_some(repository_button, |this, button| this.child(button))
                            .when_some(website_button, |this, button| this.child(button)),
                    ),
            )
    }

    fn install_control(
        &self,
        server: &ServerResponse,
        install_status: RegistryInstallStatus,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let button_id = SharedString::from(format!("install-mcp-server-{}", server.name()));
        if server.name().trim().is_empty() {
            return Button::new(button_id, "Unavailable")
                .style(ButtonStyle::OutlinedGhost)
                .disabled(true)
                .into_any_element();
        }
        let foreground_operation_pending = self.pending_server_operations.contains(server.name());
        let operation_pending = foreground_operation_pending
            || self
                .registry_store
                .read(cx)
                .is_server_operation_pending(server.name());
        if operation_pending {
            return Button::new(
                button_id,
                if foreground_operation_pending {
                    "Working..."
                } else {
                    "Finishing..."
                },
            )
            .style(ButtonStyle::OutlinedGhost)
            .start_icon(
                Icon::new(IconName::LoadCircle)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
            .disabled(true)
            .into_any_element();
        }

        match install_status {
            RegistryInstallStatus::InstalledRegistry => {
                let server_name = server.name().to_string();
                let configured_in_project =
                    self.context_server_store.as_ref().is_some_and(|store| {
                        store.read(cx).is_server_configured_locally(
                            &context_server::ContextServerId(server_name.clone().into()),
                        )
                    });
                let removable =
                    registry_settings_ownership(&server_name, configured_in_project, cx)
                        == RegistrySettingsOwnership::User;
                Button::new(button_id, "Remove")
                    .style(ButtonStyle::OutlinedGhost)
                    .disabled(!removable)
                    .tooltip(Tooltip::text(if removable {
                        "Remove MCP Registry server"
                    } else {
                        "Remove this server from its active profile, OS, channel, project, or managed settings source"
                    }))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.remove_registry_server(server_name.clone(), cx);
                    }))
                    .into_any_element()
            }
            RegistryInstallStatus::InstalledOther => Button::new(button_id, "Installed")
                .style(ButtonStyle::OutlinedGhost)
                .disabled(true)
                .tooltip(|_, cx| {
                    Tooltip::with_meta(
                        "Already configured",
                        None,
                        "A server with this ID is already configured from another source",
                        cx,
                    )
                })
                .into_any_element(),
            RegistryInstallStatus::NotInstalled => {
                self.install_control_for_not_installed(server, button_id, cx)
            }
        }
    }

    fn install_control_for_not_installed(
        &self,
        list_server: &ServerResponse,
        button_id: SharedString,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match self.installation_details.get(list_server.name()) {
            None => {
                let list_server = list_server.clone();
                Button::new(button_id, "Install")
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .start_icon(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.load_installation_details(list_server.clone(), window, cx);
                    }))
                    .into_any_element()
            }
            Some(InstallationDetailsState::Ready { options, .. }) if options.is_empty() => {
                Button::new(button_id, "Unavailable")
                    .style(ButtonStyle::OutlinedGhost)
                    .disabled(true)
                    .into_any_element()
            }
            Some(InstallationDetailsState::Ready { server, options }) if options.len() == 1 => {
                let server = server.clone();
                let option = options.first().cloned();
                Button::new(button_id, "Install")
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .start_icon(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        if let Some(option) = option.clone() {
                            this.select_installation_option(server.clone(), option, window, cx);
                        }
                    }))
                    .into_any_element()
            }
            Some(InstallationDetailsState::Ready { server, options }) => {
                let weak_self = cx.weak_entity();
                let server = server.clone();
                let options = options.clone();
                PopoverMenu::new(button_id.clone())
                    .trigger(
                        Button::new(button_id, "Install")
                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .start_icon(
                                Icon::new(IconName::Download)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .end_icon(
                                Icon::new(IconName::ChevronDown)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted),
                            ),
                    )
                    .anchor(gpui::Anchor::TopRight)
                    .menu(move |window, cx| {
                        let weak_self = weak_self.clone();
                        let server = server.clone();
                        let options = options.clone();
                        Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                            menu = menu.header("Choose installation method");
                            for option in options {
                                let weak_self = weak_self.clone();
                                let server = server.clone();
                                let label = option.label.clone();
                                menu.push_item(ContextMenuEntry::new(label).handler(
                                    move |window, cx| {
                                        weak_self
                                            .update(cx, |this, cx| {
                                                this.select_installation_option(
                                                    server.clone(),
                                                    option.clone(),
                                                    window,
                                                    cx,
                                                );
                                            })
                                            .log_err();
                                    },
                                ));
                            }
                            menu
                        }))
                    })
                    .into_any_element()
            }
        }
    }

    fn load_installation_details(
        &mut self,
        list_server: ServerResponse,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let server_name = list_server.name().to_string();
        self.registry_store.update(cx, |store, cx| {
            store.remember_server(list_server.clone(), cx);
        });
        let options = list_server.installation_options();
        self.installation_details.insert(
            server_name,
            InstallationDetailsState::Ready {
                server: list_server.clone(),
                options: options.clone(),
            },
        );
        if let [option] = options.as_slice() {
            self.select_installation_option(list_server, option.clone(), window, cx);
        } else {
            cx.notify();
        }
    }

    fn select_installation_option(
        &mut self,
        server: ServerResponse,
        option: McpRegistryInstallationOption,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.operation_errors.remove(server.name());
        if option.inputs.is_empty() {
            self.install_registry_server_without_inputs(server, option.source, cx);
            return;
        }

        let workspace = self.workspace.clone();
        let registry_store = self.registry_store.clone();
        workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    McpRegistryInstallModal::new(server, option, registry_store, window, cx)
                });
            })
            .log_err();
    }

    fn install_registry_server_without_inputs(
        &mut self,
        server: ServerResponse,
        source: McpRegistryInstallationSource,
        cx: &mut Context<Self>,
    ) {
        let server_name = server.name().to_string();
        if ProjectSettings::get_global(cx)
            .context_servers
            .contains_key(server_name.as_str())
        {
            self.operation_errors.insert(
                server_name,
                "A server with this ID is already configured.".into(),
            );
            cx.notify();
            return;
        }

        if !registry_user_settings_destination_is_active(cx) {
            self.operation_errors.insert(
                server_name,
                "The active settings profile excludes user settings. Switch to a profile based on User before installing.".into(),
            );
            cx.notify();
            return;
        }
        let registry_store = self.registry_store.clone();
        let operation_started = registry_store.update(cx, |store, cx| {
            if !store.begin_server_operation(&server_name, cx) {
                return false;
            }
            store.remember_server(server, cx);
            true
        });
        if !operation_started {
            self.operation_errors.insert(
                server_name,
                "An install or removal is already in progress for this server.".into(),
            );
            cx.notify();
            return;
        }

        self.operation_errors.remove(server_name.as_str());
        self.pending_server_operations.insert(server_name.clone());
        cx.notify();

        let task_server_name = server_name.clone();
        let registry_settings = McpRegistryServerSettings {
            credential_id: None,
            inputs: HashMap::default(),
            source: Some(source),
        };
        let settings_completion =
            update_registry_server_settings(server_name.clone(), registry_settings.clone(), cx);
        let task = cx.spawn(async move |this, cx| {
            let operation_cx = cx.clone();
            let result = complete_registry_server_operation(
                registry_store,
                task_server_name.clone(),
                finish_registry_server_installation(
                    &task_server_name,
                    &registry_settings,
                    settings_completion,
                    &operation_cx,
                )
                .boxed_local(),
                &operation_cx,
            )
            .boxed_local();
            let result =
                match wait_for_foreground_operation(result, cx.background_executor().clone()).await
                {
                    ForegroundOperation::Completed(result) => result,
                    ForegroundOperation::Continuing(operation) => {
                        this.update(cx, |this, cx| {
                            this.pending_server_operations
                                .remove(task_server_name.as_str());
                            cx.notify();
                        })
                        .log_err();
                        operation.await
                    }
                };

            let error_message = result
                .as_ref()
                .err()
                .map(|error| format!("Failed to install server: {error:#}"));
            let page_update = this.update(cx, |this, cx| {
                this.pending_server_operations
                    .remove(task_server_name.as_str());
                if let Some(error_message) = error_message.as_ref() {
                    this.operation_errors
                        .insert(task_server_name.clone(), error_message.clone().into());
                } else {
                    this.operation_errors.remove(task_server_name.as_str());
                }
                cx.notify();
            });
            if page_update.is_err()
                && let Some(error_message) = error_message
            {
                cx.update(|cx| {
                    Err::<(), _>(anyhow::anyhow!(error_message)).notify_app_err(cx);
                });
            }
        });
        task.detach();
    }

    fn remove_registry_server(&mut self, server_name: String, cx: &mut Context<Self>) {
        let configured_in_project = self.context_server_store.as_ref().is_some_and(|store| {
            store
                .read(cx)
                .is_server_configured_locally(&context_server::ContextServerId(
                    server_name.clone().into(),
                ))
        });
        if registry_settings_ownership(&server_name, configured_in_project, cx)
            != RegistrySettingsOwnership::User
        {
            self.operation_errors.insert(
                server_name,
                "Remove this server from its active profile, OS, channel, project, or managed settings source instead of user settings.".into(),
            );
            cx.notify();
            return;
        }
        let Some(removed_settings @ ContextServerSettingsContent::Registry { .. }) =
            raw_user_context_server_settings(&server_name, cx)
        else {
            let error_message = format!(
                "MCP Registry server {server_name} is not present in user settings and must be removed from its original settings file"
            );
            self.operation_errors
                .insert(server_name, error_message.clone().into());
            Err::<(), _>(anyhow::anyhow!(error_message)).notify_app_err(cx);
            cx.notify();
            return;
        };
        let registry_store = self.registry_store.clone();
        if !registry_store.update(cx, |store, cx| {
            store.begin_server_operation(&server_name, cx)
        }) {
            self.operation_errors.insert(
                server_name,
                "An install or removal is already in progress for this server.".into(),
            );
            cx.notify();
            return;
        }
        self.operation_errors.remove(server_name.as_str());
        self.installation_details.remove(server_name.as_str());
        self.pending_server_operations.insert(server_name.clone());
        cx.notify();

        let task_server_name = server_name.clone();
        let settings_completion =
            remove_registry_server_settings(server_name.clone(), removed_settings.clone(), cx);
        let task = cx.spawn(async move |this, cx| {
            let operation_cx = cx.clone();
            let operation = complete_registry_server_operation(
                registry_store,
                task_server_name.clone(),
                finish_registry_server_removal(
                    &task_server_name,
                    &removed_settings,
                    settings_completion,
                    &operation_cx,
                )
                .boxed_local(),
                &operation_cx,
            )
            .boxed_local();
            let result =
                match wait_for_foreground_operation(operation, cx.background_executor().clone())
                    .await
                {
                    ForegroundOperation::Completed(result) => result,
                    ForegroundOperation::Continuing(operation) => {
                        this.update(cx, |this, cx| {
                            this.pending_server_operations
                                .remove(task_server_name.as_str());
                            cx.notify();
                        })
                        .log_err();
                        operation.await
                    }
                };
            let error_message = result
                .as_ref()
                .err()
                .map(|error| format!("Failed to remove server or its credentials: {error:#}"));
            if let Err(error) = this.update(cx, |this, cx| {
                this.pending_server_operations
                    .remove(task_server_name.as_str());
                if let Some(error_message) = error_message.as_ref() {
                    this.operation_errors
                        .insert(task_server_name.clone(), error_message.clone().into());
                } else {
                    this.operation_errors.remove(task_server_name.as_str());
                }
                cx.notify();
            }) {
                log::debug!("MCP Registry page closed before removal completed: {error:#}");
            }
            if let Some(error_message) = error_message {
                cx.update(|cx| {
                    Err::<(), _>(anyhow::anyhow!(error_message)).notify_app_err(cx);
                });
            }
        });
        task.detach();
    }
}

impl Render for McpRegistryPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .p_4()
                    .gap_4()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        h_flex()
                            .w_full()
                            .gap_1p5()
                            .justify_between()
                            .child(Headline::new("MCP Registry").size(HeadlineSize::Large))
                            .child(
                                Button::new("learn-more-mcp-registry", "Learn More")
                                    .style(ButtonStyle::Outlined)
                                    .size(ButtonSize::Medium)
                                    .end_icon(
                                        Icon::new(IconName::ArrowUpRight)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .on_click(|_, _, cx| cx.open_url(MCP_REGISTRY_ABOUT_URL)),
                            ),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .flex_wrap()
                            .gap_2()
                            .child(self.render_search(cx))
                            .child(
                                ToggleButtonGroup::single_row(
                                    "mcp-registry-filter-buttons",
                                    [
                                        ToggleButtonSimple::new(
                                            "All",
                                            cx.listener(|this, _, _, cx| {
                                                this.filter = RegistryFilter::All;
                                                this.filter_registry_servers(cx);
                                                this.scroll_to_top(cx);
                                            }),
                                        ),
                                        ToggleButtonSimple::new(
                                            "Installed",
                                            cx.listener(|this, _, _, cx| {
                                                this.filter = RegistryFilter::Installed;
                                                this.filter_registry_servers(cx);
                                                this.scroll_to_top(cx);
                                            }),
                                        ),
                                        ToggleButtonSimple::new(
                                            "Not Installed",
                                            cx.listener(|this, _, _, cx| {
                                                this.filter = RegistryFilter::NotInstalled;
                                                this.filter_registry_servers(cx);
                                                this.scroll_to_top(cx);
                                            }),
                                        ),
                                    ],
                                )
                                .style(ToggleButtonGroupStyle::Outlined)
                                .size(ToggleButtonGroupSize::Custom(rems_from_px(30.0_f32)))
                                .label_size(LabelSize::Default)
                                .auto_width()
                                .selected_index(
                                    match self.filter {
                                        RegistryFilter::All => 0,
                                        RegistryFilter::Installed => 1,
                                        RegistryFilter::NotInstalled => 2,
                                    },
                                ),
                            ),
                    ),
            )
            .child(v_flex().px_4().size_full().overflow_y_hidden().map(|this| {
                let count = self.filtered_registry_indices.len();
                if count == 0 {
                    this.child(self.render_empty_state(cx)).into_any_element()
                } else {
                    let scroll_handle = &self.list;
                    let fetch_error = self.fetch_error.clone();
                    this.when_some(fetch_error, |this, fetch_error| {
                        this.child(
                            h_flex()
                                .py_2()
                                .gap_2()
                                .child(
                                    Icon::new(IconName::Warning)
                                        .size(IconSize::Small)
                                        .color(Color::Warning),
                                )
                                .child(
                                    Label::new(fetch_error)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    Button::new("retry-mcp-registry-page", "Retry")
                                        .style(ButtonStyle::Outlined)
                                        .size(ButtonSize::Compact)
                                        .on_click(cx.listener(|this, _, _, cx| this.retry(cx))),
                                ),
                        )
                    })
                    .child(
                        uniform_list(
                            "mcp-registry-entries",
                            count,
                            cx.processor(Self::render_servers),
                        )
                        .flex_grow_1()
                        .pb_4()
                        .track_scroll(scroll_handle),
                    )
                    .vertical_scrollbar_for(scroll_handle, window, cx)
                    .into_any_element()
                }
            }))
    }
}

impl EventEmitter<ItemEvent> for McpRegistryPage {}

impl Focusable for McpRegistryPage {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.query_editor.read(cx).focus_handle(cx)
    }
}

impl Item for McpRegistryPage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "MCP Registry".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("MCP Registry Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event)
    }
}

struct McpRegistryInputEditor {
    descriptor: McpRegistryInputDescriptor,
    editor: Entity<Editor>,
    error: Option<SharedString>,
}

struct McpRegistryInstallModal {
    server: ServerResponse,
    option: McpRegistryInstallationOption,
    registry_store: Entity<McpRegistryStore>,
    input_editors: Vec<McpRegistryInputEditor>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    form_error: Option<SharedString>,
    installing: bool,
}

impl McpRegistryInstallModal {
    fn new(
        server: ServerResponse,
        option: McpRegistryInstallationOption,
        registry_store: Entity<McpRegistryStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let input_editors = option
            .inputs
            .iter()
            .cloned()
            .map(|descriptor| {
                let editor = cx.new(|cx| {
                    let mut editor = Editor::single_line(window, cx);
                    if let Some(default) = descriptor.default.as_ref() {
                        let default = if descriptor.repeated {
                            serde_json::to_string(std::slice::from_ref(default)).unwrap_or_else(
                                |error| {
                                    log::error!(
                                        "failed to serialize an MCP Registry input default: {error:#}"
                                    );
                                    default.clone()
                                },
                            )
                        } else {
                            default.clone()
                        };
                        editor.set_text(default, window, cx);
                    }
                    if let Some(placeholder) = descriptor.placeholder.as_ref() {
                        editor.set_placeholder_text(placeholder, window, cx);
                    } else if descriptor.repeated {
                        editor.set_placeholder_text("[\"first\", \"second\"]", window, cx);
                    }
                    editor.set_masked(descriptor.secret, cx);
                    editor
                });
                McpRegistryInputEditor {
                    descriptor,
                    editor,
                    error: None,
                }
            })
            .collect::<Vec<_>>();
        let focus_handle = input_editors
            .first()
            .map(|input| input.editor.focus_handle(cx))
            .unwrap_or_else(|| cx.focus_handle());

        Self {
            server,
            option,
            registry_store,
            input_editors,
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            form_error: None,
            installing: false,
        }
    }

    fn confirm(&mut self, cx: &mut Context<Self>) {
        if self.installing {
            return;
        }

        self.form_error = None;
        let mut settings_inputs = HashMap::default();
        let mut secret_inputs = HashMap::default();
        let mut has_errors = false;

        for input in &mut self.input_editors {
            let text = input.editor.read(cx).text(cx);
            match validate_input(&input.descriptor, &text) {
                Ok(Some(values)) => {
                    input.error = None;
                    if input.descriptor.secret {
                        secret_inputs.insert(input.descriptor.id.clone(), values);
                    } else {
                        settings_inputs.insert(input.descriptor.id.clone(), values);
                    }
                }
                Ok(None) => {
                    input.error = None;
                }
                Err(error) => {
                    input.error = Some(error);
                    has_errors = true;
                }
            }
        }

        if has_errors {
            cx.notify();
            return;
        }

        if ProjectSettings::get_global(cx)
            .context_servers
            .contains_key(self.server.name())
        {
            self.form_error = Some(
                "A server with this ID is already configured. Remove it before installing this registry server."
                    .into(),
            );
            cx.notify();
            return;
        }

        if !registry_user_settings_destination_is_active(cx) {
            self.form_error = Some(
                "The active settings profile excludes user settings. Switch to a profile based on User before installing.".into(),
            );
            cx.notify();
            return;
        }
        let server = self.server.clone();
        let server_name = server.name().to_string();
        let registry_store = self.registry_store.clone();
        if !registry_store.update(cx, |store, cx| {
            store.begin_server_operation(&server_name, cx)
        }) {
            self.form_error =
                Some("An install or removal is already in progress for this server.".into());
            cx.notify();
            return;
        }

        self.installing = true;
        let source = self.option.source.clone();
        let credential_id = (!secret_inputs.is_empty()).then(|| Uuid::new_v4().to_string());
        let registry_settings = McpRegistryServerSettings {
            credential_id: credential_id.clone(),
            inputs: settings_inputs,
            source: Some(source),
        };
        let task = cx.spawn(async move |this, cx| {
            let operation_cx = cx.clone();
            let release_cx = operation_cx.clone();
            let cache_registry_store = registry_store.clone();
            let operation_server_name = server_name.clone();
            let operation = async move {
                if let Some(credential_id) = credential_id.as_deref() {
                    write_server_secrets(credential_id, &secret_inputs, &operation_cx)
                        .await
                        .map_err(|error| {
                            anyhow::anyhow!("failed to store server credentials: {error:#}")
                        })?;
                }

                operation_cx.update(|cx| {
                    cache_registry_store.update(cx, |store, cx| {
                        store.remember_server(server, cx);
                    });
                });
                let settings_completion = operation_cx.update(|cx| {
                    update_registry_server_settings(
                        server_name.clone(),
                        registry_settings.clone(),
                        cx,
                    )
                });
                finish_registry_server_installation(
                    &server_name,
                    &registry_settings,
                    settings_completion,
                    &operation_cx,
                )
                .await
            }
            .boxed_local();
            let operation = complete_registry_server_operation(
                registry_store,
                operation_server_name,
                operation,
                &release_cx,
            )
            .boxed_local();

            match wait_for_foreground_operation(operation, cx.background_executor().clone()).await {
                ForegroundOperation::Completed(Ok(())) => {
                    this.update(cx, |this, cx| {
                        this.installing = false;
                        cx.emit(DismissEvent);
                    })
                    .log_err();
                }
                ForegroundOperation::Completed(Err(error)) => {
                    let error_message = format!("Failed to install server: {error:#}");
                    if let Err(update_error) = this.update(cx, |this, cx| {
                        this.installing = false;
                        this.form_error = Some(error_message.clone().into());
                        cx.notify();
                    }) {
                        log::error!(
                            "MCP Registry installation failed after the modal closed: {error_message}; {update_error:#}"
                        );
                        cx.update(|cx| {
                            Err::<(), _>(anyhow::anyhow!(error_message)).notify_app_err(cx);
                        });
                    }
                }
                ForegroundOperation::Continuing(operation) => {
                    this.update(cx, |this, cx| {
                        this.installing = false;
                        cx.emit(DismissEvent);
                    })
                    .log_err();

                    if let Err(error) = operation.await {
                        let error_message = format!("Failed to install server: {error:#}");
                        cx.update(|cx| {
                            Err::<(), _>(anyhow::anyhow!(error_message)).notify_app_err(cx);
                        });
                    }
                }
            }
        });
        task.detach();
        cx.notify();
    }

    fn cancel(&mut self, cx: &mut Context<Self>) {
        if !self.installing {
            cx.emit(DismissEvent);
        }
    }

    fn render_input(input: &McpRegistryInputEditor, cx: &mut Context<Self>) -> AnyElement {
        let label = if input.descriptor.required {
            format!("{} *", input.descriptor.label)
        } else {
            input.descriptor.label.clone()
        };
        let border_color = if input.error.is_some() {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border
        };
        let choices = (!input.descriptor.choices.is_empty())
            .then(|| format!("Allowed values: {}", input.descriptor.choices.join(", ")));

        v_flex()
            .gap_1()
            .child(
                Label::new(label)
                    .size(LabelSize::Small)
                    .when(input.error.is_some(), |label| label.color(Color::Error)),
            )
            .when_some(input.descriptor.description.clone(), |this, description| {
                this.child(
                    Label::new(description)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
            .when(input.descriptor.repeated, |this| {
                this.child(
                    Label::new("Enter a JSON array of strings.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
            .when_some(choices, |this, choices| {
                this.child(
                    Label::new(choices)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
            .child(
                div()
                    .h_8()
                    .w_full()
                    .px_1()
                    .rounded_sm()
                    .border_1()
                    .border_color(border_color)
                    .bg(cx.theme().colors().editor_background)
                    .child(input.editor.clone()),
            )
            .when_some(input.error.clone(), |this, error| {
                this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
            })
            .into_any_element()
    }
}

impl Render for McpRegistryInstallModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display_name = self
            .server
            .title()
            .filter(|title| !title.trim().is_empty())
            .unwrap_or_else(|| self.server.name());
        let focus_handle = self.focus_handle(cx);

        div()
            .elevation_3(cx)
            .w(rems(40.))
            .key_context("McpRegistryInstallModal")
            .track_focus(&focus_handle)
            .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| this.cancel(cx)))
            .on_action(cx.listener(|this, _: &menu::Confirm, _, cx| this.confirm(cx)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window, cx);
            }))
            .child(
                Modal::new("mcp-registry-install-modal", None)
                    .header(
                        ModalHeader::new()
                            .headline(format!("Install {display_name}"))
                            .description(self.option.label.clone()),
                    )
                    .section(
                        Section::new().child(
                            div()
                                .size_full()
                                .child(
                                    v_flex()
                                        .id("mcp-registry-install-inputs")
                                        .max_h(vh(0.7, window))
                                        .overflow_y_scroll()
                                        .track_scroll(&self.scroll_handle)
                                        .gap_3()
                                        .children(
                                            self.input_editors
                                                .iter()
                                                .map(|input| Self::render_input(input, cx)),
                                        )
                                        .when_some(self.form_error.clone(), |this, error| {
                                            this.child(
                                                h_flex()
                                                    .gap_1p5()
                                                    .child(
                                                        Icon::new(IconName::Warning)
                                                            .size(IconSize::Small)
                                                            .color(Color::Warning),
                                                    )
                                                    .child(
                                                        Label::new(error)
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted),
                                                    ),
                                            )
                                        })
                                        .when(self.installing, |this| {
                                            this.child(
                                                h_flex()
                                                    .gap_1p5()
                                                    .child(
                                                        Icon::new(IconName::LoadCircle)
                                                            .size(IconSize::XSmall)
                                                            .color(Color::Muted)
                                                            .with_rotate_animation(3),
                                                    )
                                                    .child(
                                                        Label::new("Installing server...")
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted),
                                                    ),
                                            )
                                        }),
                                )
                                .vertical_scrollbar_for(&self.scroll_handle, window, cx),
                        ),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("cancel-mcp-registry-install", "Cancel")
                                        .disabled(self.installing)
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Cancel,
                                                &focus_handle,
                                                cx,
                                            )
                                            .map(|binding| binding.size(rems_from_px(12.0_f32))),
                                        )
                                        .on_click(cx.listener(|this, _, _, cx| this.cancel(cx))),
                                )
                                .child(
                                    Button::new(
                                        "confirm-mcp-registry-install",
                                        if self.installing {
                                            "Installing..."
                                        } else {
                                            "Install"
                                        },
                                    )
                                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                    .disabled(self.installing)
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &menu::Confirm,
                                            &focus_handle,
                                            cx,
                                        )
                                        .map(|binding| binding.size(rems_from_px(12.0_f32))),
                                    )
                                    .on_click(cx.listener(|this, _, _, cx| this.confirm(cx))),
                                ),
                        ),
                    ),
            )
    }
}

impl Focusable for McpRegistryInstallModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for McpRegistryInstallModal {}

impl ModalView for McpRegistryInstallModal {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> DismissDecision {
        DismissDecision::Dismiss(!self.installing)
    }
}

fn update_registry_server_settings(
    server_name: String,
    registry_settings: McpRegistryServerSettings,
    cx: &mut App,
) -> futures::channel::oneshot::Receiver<anyhow::Result<()>> {
    let fs = <dyn Fs>::global(cx);
    update_settings_file_with_completion(fs, cx, move |settings, _| {
        settings
            .project
            .context_servers
            .entry(server_name.into())
            .or_insert_with(|| ContextServerSettingsContent::Registry {
                enabled: true,
                remote: false,
                registry: registry_settings,
            });
    })
}

fn remove_registry_server_settings(
    server_name: String,
    removed_settings: ContextServerSettingsContent,
    cx: &mut App,
) -> futures::channel::oneshot::Receiver<anyhow::Result<()>> {
    let fs = <dyn Fs>::global(cx);
    update_settings_file_with_completion(fs, cx, move |settings, _| {
        if settings.project.context_servers.get(server_name.as_str()) == Some(&removed_settings) {
            settings
                .project
                .context_servers
                .remove(server_name.as_str());
        }
    })
}

async fn finish_registry_server_installation(
    server_name: &str,
    expected_settings: &McpRegistryServerSettings,
    settings_completion: futures::channel::oneshot::Receiver<anyhow::Result<()>>,
    cx: &gpui::AsyncApp,
) -> anyhow::Result<()> {
    let completion_error = match settings_completion.await {
        Ok(Ok(())) => None,
        Ok(Err(error)) => Some(format!("failed to update settings: {error:#}")),
        Err(error) => Some(format!("settings update was canceled: {error}")),
    };
    let settings_match = cx.update(|cx| {
        registry_server_settings_match(
            raw_user_context_server_settings(server_name, cx).as_ref(),
            expected_settings,
        )
    });
    if settings_match {
        if cx.update(|cx| registry_user_settings_destination_is_active(cx)) {
            return Ok(());
        }
        anyhow::bail!(
            "the active settings profile excludes user settings; switch to a profile based on User"
        );
    }

    let rollback_error =
        delete_unreferenced_registry_credential(expected_settings.credential_id.as_deref(), cx)
            .await
            .err()
            .map(|error| format!("; credential rollback also failed: {error:#}"))
            .unwrap_or_default();
    let installation_error = completion_error
        .unwrap_or_else(|| format!("server `{server_name}` changed while it was being installed"));
    Err(anyhow::anyhow!("{installation_error}{rollback_error}"))
}

async fn finish_registry_server_removal(
    server_name: &str,
    removed_settings: &ContextServerSettingsContent,
    settings_completion: futures::channel::oneshot::Receiver<anyhow::Result<()>>,
    cx: &gpui::AsyncApp,
) -> anyhow::Result<()> {
    let completion_error = match settings_completion.await {
        Ok(Ok(())) => None,
        Ok(Err(error)) => Some(format!("failed to update settings: {error:#}")),
        Err(error) => Some(format!("settings update was canceled: {error}")),
    };
    let current_settings = cx.update(|cx| raw_user_context_server_settings(server_name, cx));
    if current_settings.is_some() {
        return Err(anyhow::anyhow!(completion_error.unwrap_or_else(
            || format!("server `{server_name}` changed while it was being removed")
        )));
    }

    let credential_id = match removed_settings {
        ContextServerSettingsContent::Registry { registry, .. } => {
            registry.credential_id.as_deref()
        }
        _ => None,
    };
    let cleanup_result = delete_unreferenced_registry_credential(credential_id, cx).await;
    if let Err(error) = cleanup_result {
        let restore_completion = cx.update(|cx| {
            let fs = <dyn Fs>::global(cx);
            let server_name = server_name.to_owned();
            let removed_settings = removed_settings.clone();
            update_settings_file_with_completion(fs, cx, move |settings, _| {
                settings
                    .project
                    .context_servers
                    .entry(server_name.into())
                    .or_insert(removed_settings);
            })
        });
        restore_completion
            .await
            .map_err(|restore_error| {
                anyhow::anyhow!("{error:#}; settings restoration was canceled: {restore_error}")
            })?
            .map_err(|restore_error| {
                anyhow::anyhow!("{error:#}; settings restoration failed: {restore_error:#}")
            })?;
        if cx
            .update(|cx| raw_user_context_server_settings(server_name, cx))
            .as_ref()
            != Some(removed_settings)
        {
            anyhow::bail!(
                "{error:#}; settings changed during restoration, so verify the server and credentials before retrying"
            );
        }
        anyhow::bail!("{error:#}; server settings were restored so removal can be retried");
    }
    Ok(())
}

async fn delete_unreferenced_registry_credential(
    credential_id: Option<&str>,
    cx: &gpui::AsyncApp,
) -> anyhow::Result<()> {
    if !cx.update(|cx| cx.has_flag::<feature_flags::McpRegistryFeatureFlag>()) {
        anyhow::bail!("MCP Registry feature disabled during credential removal");
    }
    let Some(credential_id) = credential_id else {
        return Ok(());
    };
    let credential_is_referenced =
        cx.update(|cx| registry_credential_is_referenced(credential_id, cx));
    if !credential_is_referenced {
        delete_server_secrets(credential_id, cx).await?;
    }
    Ok(())
}

fn raw_user_context_server_settings(
    server_name: &str,
    cx: &App,
) -> Option<ContextServerSettingsContent> {
    cx.global::<SettingsStore>()
        .raw_user_settings()
        .and_then(|settings| settings.content.project.context_servers.get(server_name))
        .cloned()
}

fn registry_server_settings_match(
    current_settings: Option<&ContextServerSettingsContent>,
    expected_settings: &McpRegistryServerSettings,
) -> bool {
    matches!(
        current_settings,
        Some(ContextServerSettingsContent::Registry { registry, .. })
            if registry == expected_settings
    )
}

fn validate_input(
    descriptor: &McpRegistryInputDescriptor,
    text: &str,
) -> Result<Option<Vec<String>>, SharedString> {
    if text.trim().is_empty() {
        return if descriptor.required {
            Err(format!("{} is required", descriptor.label).into())
        } else {
            Ok(None)
        };
    }

    let values = if descriptor.repeated {
        serde_json::from_str::<Vec<String>>(text).map_err(|_| {
            SharedString::from(format!(
                "{} must be a JSON array of strings",
                descriptor.label
            ))
        })?
    } else {
        vec![text.to_string()]
    };

    if descriptor.required
        && (values.is_empty() || values.iter().any(|value| value.trim().is_empty()))
    {
        return Err(format!("{} requires at least one value", descriptor.label).into());
    }

    let mut normalized_values = Vec::with_capacity(values.len());
    for value in values {
        normalized_values.push(normalize_input_value(descriptor, &value)?);
    }

    Ok(Some(normalized_values))
}

fn normalize_input_value(
    descriptor: &McpRegistryInputDescriptor,
    value: &str,
) -> Result<String, SharedString> {
    let normalized = normalize_formatted_input(&descriptor.format, value, &descriptor.label)?;
    if !descriptor.choices.is_empty()
        && !descriptor.choices.iter().any(|choice| {
            normalize_formatted_input(&descriptor.format, choice, &descriptor.label)
                .is_ok_and(|choice| choice == normalized)
        })
    {
        return Err(format!("{} must be one of the allowed values", descriptor.label).into());
    }
    Ok(normalized)
}

fn normalize_formatted_input(
    format: &str,
    value: &str,
    label: &str,
) -> Result<String, SharedString> {
    match format {
        "boolean" => value
            .trim()
            .parse::<bool>()
            .map(|value| value.to_string())
            .map_err(|_| SharedString::from(format!("{label} must be true or false"))),
        "number" => {
            let value = value.trim();
            value
                .parse::<f64>()
                .ok()
                .filter(|number| number.is_finite())
                .map(|_| value.to_owned())
                .ok_or_else(|| SharedString::from(format!("{label} must be a finite number")))
        }
        "string" | "filepath" => Ok(value.to_string()),
        _ => Err(format!("{label} uses an unsupported input format ({format})").into()),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        future::Future,
        path::Path,
        pin::Pin,
        sync::{
            Arc, Mutex,
            atomic::{AtomicBool, Ordering},
        },
    };

    use credentials_provider::CredentialsProvider;
    use feature_flags::{FeatureFlag, McpRegistryFeatureFlag};
    use fs::FakeFs;
    use gpui::{AsyncApp, TestAppContext};
    use project::Project;
    use zed_credentials_provider::ZedCredentialsProvider;

    use super::*;

    struct FailingCredentialsProvider {
        delete_started: AtomicBool,
        release: Mutex<Option<futures::channel::oneshot::Receiver<()>>>,
    }

    impl CredentialsProvider for FailingCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<(String, Vec<u8>)>>> + 'a>> {
            Box::pin(async { anyhow::bail!("unexpected credential read") })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            _username: &'a str,
            _password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'a>> {
            Box::pin(async { anyhow::bail!("unexpected credential write") })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'a>> {
            Box::pin(async move {
                self.delete_started.store(true, Ordering::SeqCst);
                let release = self.release.lock().expect("test lock").take();
                if let Some(release) = release {
                    release.await?;
                }
                anyhow::bail!("injected credential deletion failure")
            })
        }
    }

    fn removal_settings() -> ContextServerSettingsContent {
        ContextServerSettingsContent::Registry {
            enabled: false,
            remote: true,
            registry: McpRegistryServerSettings {
                credential_id: Some("registry-credential".to_owned()),
                inputs: HashMap::from_iter([(
                    "argument:0".to_owned(),
                    vec!["visible input".to_owned()],
                )]),
                source: Some(McpRegistryInstallationSource::Remote {
                    url: "https://example.com/mcp".into(),
                }),
            },
        }
    }

    async fn setup_removal(
        cx: &mut TestAppContext,
        provider: Arc<FailingCredentialsProvider>,
    ) -> Arc<FakeFs> {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(paths::settings_file().parent().expect("settings parent"))
            .await
            .expect("create settings directory");
        let settings = serde_json::json!({
            "context_servers": { "registry-server": removal_settings() },
        })
        .to_string();
        fs.insert_file(paths::settings_file(), settings.as_bytes().to_vec())
            .await;
        cx.update(|cx| {
            <dyn Fs>::set_global(fs.clone(), cx);
            let mut store = SettingsStore::test(cx);
            store
                .set_user_settings(&settings, cx)
                .expect("valid user settings");
            cx.set_global(store);
            cx.set_global(ZedCredentialsProvider(provider));
            cx.update_flags(false, vec![McpRegistryFeatureFlag::NAME.to_owned()]);
        });
        fs
    }

    fn start_removal(cx: &mut TestAppContext) -> Task<anyhow::Result<()>> {
        cx.update(|cx| {
            let removed_settings = removal_settings();
            let completion = remove_registry_server_settings(
                "registry-server".into(),
                removed_settings.clone(),
                cx,
            );
            cx.spawn(async move |cx| {
                finish_registry_server_removal("registry-server", &removed_settings, completion, cx)
                    .await
            })
        })
    }

    fn user_server(cx: &TestAppContext) -> Option<ContextServerSettingsContent> {
        cx.update(|cx| raw_user_context_server_settings("registry-server", cx))
    }

    async fn saved_server(fs: &Arc<FakeFs>) -> Option<ContextServerSettingsContent> {
        let text = fs
            .load(paths::settings_file())
            .await
            .expect("settings file");
        let settings: serde_json::Value = serde_json::from_str(&text).expect("settings JSON");
        settings
            .get("context_servers")
            .and_then(|servers| servers.get("registry-server"))
            .map(|server| serde_json::from_value(server.clone()).expect("registry server settings"))
    }

    #[gpui::test]
    async fn failed_credential_deletion_restores_complete_registry_entry(cx: &mut TestAppContext) {
        let provider = Arc::new(FailingCredentialsProvider {
            delete_started: AtomicBool::new(false),
            release: Mutex::new(None),
        });
        let fs = setup_removal(cx, provider.clone()).await;
        let result = start_removal(cx).await;
        assert!(
            result
                .expect_err("deletion must fail")
                .to_string()
                .contains("restored")
        );
        assert!(provider.delete_started.load(Ordering::SeqCst));
        assert_eq!(user_server(cx), Some(removal_settings()));
        assert_eq!(saved_server(&fs).await, Some(removal_settings()));
    }

    #[gpui::test]
    async fn opt_out_during_credential_deletion_restores_registry_entry(cx: &mut TestAppContext) {
        let (release_sender, release_receiver) = futures::channel::oneshot::channel();
        let provider = Arc::new(FailingCredentialsProvider {
            delete_started: AtomicBool::new(false),
            release: Mutex::new(Some(release_receiver)),
        });
        let fs = setup_removal(cx, provider.clone()).await;
        let task = start_removal(cx);
        cx.run_until_parked();
        assert!(provider.delete_started.load(Ordering::SeqCst));
        assert_eq!(user_server(cx), None);
        assert_eq!(saved_server(&fs).await, None);
        cx.update(|cx| cx.update_flags(false, Vec::new()));
        assert!(
            task.await
                .expect_err("opt-out must fail cleanup")
                .to_string()
                .contains("restored")
        );
        assert_eq!(user_server(cx), Some(removal_settings()));
        assert_eq!(saved_server(&fs).await, Some(removal_settings()));
        drop(release_sender);
    }

    #[gpui::test]
    async fn concurrent_registry_edit_is_not_overwritten_by_failed_cleanup(
        cx: &mut TestAppContext,
    ) {
        let (release_sender, release_receiver) = futures::channel::oneshot::channel();
        let provider = Arc::new(FailingCredentialsProvider {
            delete_started: AtomicBool::new(false),
            release: Mutex::new(Some(release_receiver)),
        });
        let fs = setup_removal(cx, provider.clone()).await;
        let task = start_removal(cx);
        cx.run_until_parked();
        assert!(provider.delete_started.load(Ordering::SeqCst));
        assert_eq!(user_server(cx), None);
        let replacement = ContextServerSettingsContent::Registry {
            enabled: true,
            remote: false,
            registry: McpRegistryServerSettings {
                credential_id: Some("replacement-credential".into()),
                ..McpRegistryServerSettings::default()
            },
        };
        let completion = cx.update(|cx| {
            let replacement = replacement.clone();
            update_settings_file_with_completion(<dyn Fs>::global(cx), cx, move |settings, _| {
                settings
                    .project
                    .context_servers
                    .insert("registry-server".into(), replacement);
            })
        });
        completion
            .await
            .expect("settings update completion")
            .expect("settings update");
        release_sender
            .send(())
            .expect("release credential deletion");
        assert!(task.await.is_err());
        assert_eq!(user_server(cx), Some(replacement.clone()));
        assert_eq!(saved_server(&fs).await, Some(replacement));
    }

    #[gpui::test]
    async fn removal_rejects_changed_enabled_and_remote_before_settings_write(
        cx: &mut TestAppContext,
    ) {
        let provider = Arc::new(FailingCredentialsProvider {
            delete_started: AtomicBool::new(false),
            release: Mutex::new(None),
        });
        let fs = setup_removal(cx, provider.clone()).await;
        let task = start_removal(cx);
        let replacement = ContextServerSettingsContent::Registry {
            enabled: true,
            remote: false,
            registry: match removal_settings() {
                ContextServerSettingsContent::Registry { registry, .. } => registry,
                _ => unreachable!("test fixture is a registry entry"),
            },
        };
        let settings = serde_json::json!({
            "context_servers": { "registry-server": replacement },
        })
        .to_string();
        fs.insert_file(paths::settings_file(), settings.as_bytes().to_vec())
            .await;
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(&settings, cx)
                    .expect("valid settings");
            });
        });
        assert!(task.await.is_err());
        assert_eq!(user_server(cx), Some(replacement.clone()));
        assert_eq!(saved_server(&fs).await, Some(replacement));
        assert!(!provider.delete_started.load(Ordering::SeqCst));
    }

    fn descriptor(required: bool, repeated: bool, format: &str) -> McpRegistryInputDescriptor {
        McpRegistryInputDescriptor {
            id: "test".to_string(),
            label: "Test input".to_string(),
            description: None,
            required,
            secret: false,
            repeated,
            format: format.to_string(),
            default: None,
            placeholder: None,
            choices: Vec::new(),
        }
    }

    #[test]
    fn repeated_inputs_require_a_json_string_array() {
        let descriptor = descriptor(false, true, "string");

        assert!(validate_input(&descriptor, "one,two").is_err());
        assert_eq!(
            validate_input(&descriptor, r#"["one","two"]"#),
            Ok(Some(vec!["one".to_string(), "two".to_string()]))
        );
    }

    #[test]
    fn required_repeated_inputs_reject_empty_values() {
        let descriptor = descriptor(true, true, "string");

        assert!(validate_input(&descriptor, "[]").is_err());
        assert!(validate_input(&descriptor, r#"[""]"#).is_err());
    }

    #[test]
    fn formats_and_choices_are_validated() {
        let mut boolean = descriptor(true, false, "boolean");
        boolean.choices = vec!["true".to_string(), "false".to_string()];
        assert!(validate_input(&boolean, "true").is_ok());
        assert!(validate_input(&boolean, "yes").is_err());

        let number = descriptor(true, false, "number");
        assert!(validate_input(&number, "42.5").is_ok());
        assert!(validate_input(&number, "NaN").is_err());
    }

    #[test]
    fn boolean_and_number_inputs_are_stored_in_normalized_form() {
        let mut boolean = descriptor(true, false, "boolean");
        boolean.choices = vec!["true".to_string()];
        assert_eq!(
            validate_input(&boolean, "  true  "),
            Ok(Some(vec!["true".to_string()]))
        );

        let mut number = descriptor(true, true, "number");
        number.choices = vec!["42.500".to_owned(), "2e1".to_owned()];
        assert_eq!(
            validate_input(&number, r#"[" 42.500 ","2e1"]"#),
            Ok(Some(vec!["42.500".to_string(), "2e1".to_string()]))
        );
    }

    #[gpui::test]
    async fn foreground_grace_keeps_the_operation_guarded_until_completion(
        cx: &mut TestAppContext,
    ) {
        let server_name = "io.example/pending-operation";
        let registry_store = cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.update_flags(false, vec![McpRegistryFeatureFlag::NAME.to_owned()]);
            McpRegistryStore::init_test_global(cx, Vec::new())
        });
        assert!(registry_store.update(cx, |store, cx| {
            store.begin_server_operation(server_name, cx)
        }));

        let (operation_sender, operation_receiver) = futures::channel::oneshot::channel();
        let (grace_sender, grace_receiver) = futures::channel::oneshot::channel();
        let task_registry_store = registry_store.clone();
        let task_server_name = server_name.to_owned();
        let task = cx.update(|cx| {
            cx.spawn(async move |cx| {
                let operation_cx = cx.clone();
                let operation = async move {
                    operation_receiver
                        .await
                        .map_err(|error| anyhow::anyhow!("operation was canceled: {error}"))?;
                    anyhow::Ok(())
                }
                .boxed_local();
                let operation = complete_registry_server_operation(
                    task_registry_store,
                    task_server_name,
                    operation,
                    &operation_cx,
                )
                .boxed_local();

                match wait_for_foreground_operation(operation, cx.background_executor().clone())
                    .await
                {
                    ForegroundOperation::Completed(result) => result,
                    ForegroundOperation::Continuing(operation) => {
                        grace_sender
                            .send(())
                            .map_err(|_| anyhow::anyhow!("grace receiver was canceled"))?;
                        operation.await
                    }
                }
            })
        });

        cx.run_until_parked();
        cx.executor()
            .advance_clock(INSTALL_OPERATION_FOREGROUND_GRACE);
        cx.run_until_parked();

        assert!(grace_receiver.await.is_ok());
        registry_store.update(cx, |store, cx| {
            assert!(store.is_server_operation_pending(server_name));
            assert!(!store.begin_server_operation(server_name, cx));
        });
        assert!(operation_sender.send(()).is_ok());
        assert!(task.await.is_ok());
        registry_store.update(cx, |store, cx| {
            assert!(!store.is_server_operation_pending(server_name));
            assert!(store.begin_server_operation(server_name, cx));
            store.finish_server_operation(server_name, cx);
        });
    }

    #[gpui::test]
    async fn removal_cleanup_rejects_feature_opt_out_even_without_credentials(
        cx: &mut TestAppContext,
    ) {
        let task = cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.update_flags(false, vec![McpRegistryFeatureFlag::NAME.to_owned()]);
            cx.spawn(async move |cx| delete_unreferenced_registry_credential(None, cx).await)
        });
        cx.update(|cx| cx.update_flags(false, Vec::new()));
        assert!(task.await.is_err());
    }

    #[test]
    fn identical_no_secret_registry_install_is_idempotent() {
        let expected_settings = McpRegistryServerSettings {
            credential_id: None,
            inputs: HashMap::default(),
            source: Some(McpRegistryInstallationSource::Remote {
                url: "https://example.com/mcp".into(),
            }),
        };
        let current_settings = ContextServerSettingsContent::Registry {
            enabled: false,
            remote: true,
            registry: expected_settings.clone(),
        };

        assert!(registry_server_settings_match(
            Some(&current_settings),
            &expected_settings
        ));
    }

    #[test]
    fn registry_install_match_requires_exact_registry_settings() {
        let expected_settings = McpRegistryServerSettings {
            credential_id: Some("attempt-credential".to_string()),
            inputs: HashMap::from_iter([("argument:0".to_string(), vec!["expected".to_string()])]),
            source: Some(McpRegistryInstallationSource::Remote {
                url: "https://example.com/mcp".into(),
            }),
        };
        let mut different_registry_settings = expected_settings.clone();
        different_registry_settings.credential_id = Some("winner-credential".to_string());
        let current_settings = ContextServerSettingsContent::Registry {
            enabled: true,
            remote: false,
            registry: different_registry_settings,
        };

        assert!(!registry_server_settings_match(
            Some(&current_settings),
            &expected_settings
        ));
        let different_source = ContextServerSettingsContent::Registry {
            enabled: true,
            remote: false,
            registry: McpRegistryServerSettings {
                source: Some(McpRegistryInstallationSource::Remote {
                    url: "https://other.example.com/mcp".into(),
                }),
                ..expected_settings.clone()
            },
        };
        assert!(!registry_server_settings_match(
            Some(&different_source),
            &expected_settings
        ));
    }

    #[gpui::test]
    async fn registry_page_can_be_created_during_workspace_update(cx: &mut TestAppContext) {
        crate::test_support::init_test(cx);
        cx.update(|cx| cx.update_flags(false, vec![McpRegistryFeatureFlag::NAME.to_owned()]));
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None::<&Path>, cx).await;
        cx.update(|cx| {
            McpRegistryStore::init_test_global(cx, Vec::new());
        });
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        let registry_page = workspace.update_in(cx, |workspace, window, cx| {
            McpRegistryPage::new(workspace, window, cx)
        });

        assert_eq!(
            registry_page.read_with(cx, |page, cx| page.tab_content_text(0, cx)),
            "MCP Registry"
        );
    }

    #[gpui::test]
    async fn install_options_use_server_details_from_the_list(cx: &mut TestAppContext) {
        crate::test_support::init_test(cx);
        cx.update(|cx| cx.update_flags(false, vec![McpRegistryFeatureFlag::NAME.to_owned()]));
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None::<&Path>, cx).await;
        let registry_store = cx.update(|cx| McpRegistryStore::init_test_global(cx, Vec::new()));
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let registry_page = workspace.update_in(cx, |workspace, window, cx| {
            McpRegistryPage::new(workspace, window, cx)
        });
        let list_server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": "io.example/list-server",
                "description": "List server",
                "version": "1.0.0",
                "remotes": [
                    {"type": "streamable-http", "url": "https://one.example/mcp"},
                    {"type": "streamable-http", "url": "https://two.example/mcp"}
                ]
            }
        }))
        .expect("list server should parse");

        registry_page.update_in(cx, |page, window, cx| {
            page.load_installation_details(list_server.clone(), window, cx);
        });

        registry_page.read_with(cx, |page, _cx| {
            let Some(InstallationDetailsState::Ready { server, options }) =
                page.installation_details.get(list_server.name())
            else {
                panic!("list details should be ready without another request");
            };
            assert_eq!(server, &list_server);
            assert_eq!(options.len(), 2);
        });
        registry_store.read_with(cx, |store, _cx| {
            assert_eq!(store.cached_server(list_server.name()), Some(&list_server));
        });
    }
}

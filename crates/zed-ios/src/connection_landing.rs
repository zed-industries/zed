use std::path::PathBuf;
use std::sync::Arc;

use askpass::EncryptedPassword;
use editor::{
    Editor,
    actions::{Backtab, Tab},
};
use gpui::{
    AnyWindowHandle, App, AppContext as _, AsyncApp, ClickEvent, Context, Entity, Focusable as _,
    Global, IntoElement, Render, SharedString, Task, Window, WindowOptions, div, prelude::*, px,
};
use remote::SshConnectionOptions;
use serde::{Deserialize, Serialize};
use theme::ActiveTheme;
use ui::{
    ButtonCommon, ButtonLike, ButtonStyle, Clickable, Color, ContextMenu, Divider, DividerColor,
    FixedWidth, Headline, Icon, IconButton, IconName, IconSize, Label, LabelCommon, LabelSize,
    PopoverMenu, Vector, VectorName, h_flex, rems_from_px, v_flex,
};
use util::ResultExt;

// ─── Active connections registry ─────────────────────────────────────────────

/// A single workspace entry within a host connection.
struct WorkspaceEntry {
    workspace: Entity<workspace::Workspace>,
    project: Entity<project::Project>,
    path: String,
}

/// An active SSH connection to a host, potentially with multiple open workspaces
/// (project paths). The `remote_connection` is shared across all workspaces on
/// this host.
struct HostConnection {
    remote_connection: Arc<dyn remote::RemoteConnection>,
    workspaces: Vec<WorkspaceEntry>,
    host: String,
    username: String,
    port: u16,
}

/// Global registry of active SSH connections. Holding entity handles keeps
/// workspaces (and their projects/SSH sessions) alive even when the window
/// root is swapped back to the landing screen.
struct ActiveConnections {
    hosts: Vec<HostConnection>,
    /// Per-project errors that persist across navigation. Key is (host, username, port, path).
    project_errors: Vec<((String, String, u16, String), SharedString)>,
}

impl Global for ActiveConnections {}

impl ActiveConnections {
    fn find_by_host(&self, host: &str, username: &str, port: u16) -> Option<&HostConnection> {
        self.hosts
            .iter()
            .find(|c| c.host == host && c.username == username && c.port == port)
    }

    fn find_by_host_mut(
        &mut self,
        host: &str,
        username: &str,
        port: u16,
    ) -> Option<&mut HostConnection> {
        self.hosts
            .iter_mut()
            .find(|c| c.host == host && c.username == username && c.port == port)
    }

    fn find_workspace_by_path(
        &self,
        host: &str,
        username: &str,
        port: u16,
        path: &str,
    ) -> Option<&WorkspaceEntry> {
        self.find_by_host(host, username, port)
            .and_then(|hc| hc.workspaces.iter().find(|w| w.path == path))
    }

    fn remove_by_project(&mut self, project_id: gpui::EntityId) {
        for host_conn in &mut self.hosts {
            host_conn
                .workspaces
                .retain(|w| w.project.entity_id() != project_id);
        }
        // Remove host connections with no remaining workspaces.
        self.hosts.retain(|hc| !hc.workspaces.is_empty());
    }

    fn set_project_error(
        &mut self,
        host: &str,
        username: &str,
        port: u16,
        path: &str,
        error: SharedString,
    ) {
        let key = (
            host.to_string(),
            username.to_string(),
            port,
            path.to_string(),
        );
        if let Some(entry) = self.project_errors.iter_mut().find(|(k, _)| *k == key) {
            entry.1 = error;
        } else {
            self.project_errors.push((key, error));
        }
    }

    fn clear_project_error(&mut self, host: &str, username: &str, port: u16, path: &str) {
        let key = (
            host.to_string(),
            username.to_string(),
            port,
            path.to_string(),
        );
        self.project_errors.retain(|(k, _)| *k != key);
    }

    fn get_project_error(
        &self,
        host: &str,
        username: &str,
        port: u16,
        path: &str,
    ) -> Option<&SharedString> {
        let key = (
            host.to_string(),
            username.to_string(),
            port,
            path.to_string(),
        );
        self.project_errors
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, msg)| msg)
    }
}

fn active_connections_mut(cx: &mut App) -> &mut ActiveConnections {
    cx.global_mut::<ActiveConnections>()
}

/// Initialize the active connections registry. Call once during app setup.
pub fn init_active_connections(cx: &mut App) {
    cx.set_global(ActiveConnections {
        hosts: Vec::new(),
        project_errors: Vec::new(),
    });
}

// ─── Workspace switcher status bar item ──────────────────────────────────────

/// Status bar item that shows the current project path, a back arrow to return
/// to the landing screen, and a popover menu to switch between open workspaces
/// on the same host.
pub struct WorkspaceSwitcher {
    current_path: SharedString,
    host_label: SharedString,
}

impl WorkspaceSwitcher {
    pub fn new(path: &str, host: &str, username: &str) -> Self {
        let short_path = path
            .strip_prefix("~/")
            .or_else(|| path.strip_prefix('/'))
            .unwrap_or(path);
        Self {
            current_path: SharedString::from(short_path.to_string()),
            host_label: SharedString::from(format!("{username}@{host}")),
        }
    }
}

impl Render for WorkspaceSwitcher {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let current_path = self.current_path.clone();
        let host_label = self.host_label.clone();

        h_flex()
            .gap_1()
            .items_center()
            // Extra left padding to clear the iPad corner resize area
            .pl(px(24.))
            .child(
                IconButton::new("return-to-landing", IconName::ArrowLeft)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .on_click(|_event, window, cx| {
                        return_to_landing(window, cx);
                    }),
            )
            .child(
                PopoverMenu::new("workspace-switcher-menu")
                    .trigger(
                        ButtonLike::new("workspace-switcher-trigger").child(
                            h_flex()
                                .gap_1p5()
                                .items_center()
                                .child(
                                    Icon::new(IconName::Folder)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    Label::new(current_path)
                                        .size(LabelSize::Small)
                                        .color(Color::Default),
                                ),
                        ),
                    )
                    .anchor(gpui::Corner::BottomLeft)
                    .menu({
                        let host_label = host_label.clone();
                        move |window, cx| {
                            // Collect workspace info before borrowing cx mutably.
                            let entries: Vec<(
                                SharedString,
                                Entity<workspace::Workspace>,
                                Vec<Entity<workspace::Workspace>>,
                            )> = cx
                                .try_global::<ActiveConnections>()
                                .map(|active_conns| {
                                    active_conns
                                        .hosts
                                        .iter()
                                        .flat_map(|hc| {
                                            let all: Vec<_> = hc
                                                .workspaces
                                                .iter()
                                                .map(|w| w.workspace.clone())
                                                .collect();
                                            hc.workspaces.iter().map(move |entry| {
                                                (
                                                    SharedString::from(entry.path.clone()),
                                                    entry.workspace.clone(),
                                                    all.clone(),
                                                )
                                            })
                                        })
                                        .collect()
                                })
                                .unwrap_or_default();

                            if entries.is_empty() {
                                return None;
                            }

                            let menu = ContextMenu::build(window, cx, |mut menu, _window, _cx| {
                                menu = menu.header(host_label.clone());
                                for (path_label, workspace, all_workspaces) in entries {
                                    menu = menu.custom_entry(
                                        {
                                            let path_label = path_label.clone();
                                            move |_window, _cx| {
                                                h_flex()
                                                    .gap_2()
                                                    .child(
                                                        Icon::new(IconName::Folder)
                                                            .size(IconSize::Small)
                                                            .color(Color::Muted),
                                                    )
                                                    .child(
                                                        Label::new(path_label.clone())
                                                            .size(LabelSize::Small),
                                                    )
                                                    .into_any_element()
                                            }
                                        },
                                        {
                                            let workspace = workspace.clone();
                                            move |window, cx| {
                                                show_multi_workspace(
                                                    window,
                                                    cx,
                                                    &all_workspaces,
                                                    &workspace,
                                                );
                                            }
                                        },
                                    );
                                }
                                menu
                            });
                            Some(menu)
                        }
                    }),
            )
            .child(Divider::vertical().color(DividerColor::Border))
    }
}

struct StatusBarSuffix;

impl Render for StatusBarSuffix {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().pr(px(24.))
    }
}

/// Navigate back to the landing screen without closing any SSH connections.
/// Workspaces are already stored in ActiveConnections from `do_connect`.
fn return_to_landing(window: &mut Window, cx: &mut App) {
    let landing = window.replace_root(cx, |window, cx| ConnectionLanding::new(window, cx));
    landing.focus_handle(cx).focus(window, cx);
}

/// Create a fresh MultiWorkspace containing all the given workspaces,
/// activate the target one, and set it as the window root.
fn show_multi_workspace(
    window: &mut Window,
    cx: &mut App,
    all_workspaces: &[Entity<workspace::Workspace>],
    active: &Entity<workspace::Workspace>,
) {
    let first = all_workspaces[0].clone();
    let active = active.clone();
    window.replace_root(cx, |window, cx| {
        let mut multi = workspace::MultiWorkspace::new(first, window, cx);
        let mut active_idx = 0;
        for (i, ws) in all_workspaces.iter().enumerate().skip(1) {
            multi.add_workspace(ws.clone(), cx);
            if *ws == active {
                active_idx = i;
            }
        }
        if active_idx > 0 {
            multi.activate_index(active_idx, window, cx);
        }
        multi
    });
}

/// On-disk representation of a saved SSH host.
#[derive(Clone, Serialize, Deserialize)]
struct SavedHostEntry {
    nickname: Option<String>,
    host: String,
    username: String,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default)]
    password: Option<String>,
    #[serde(default)]
    project_path: Option<String>,
}

fn default_port() -> u16 {
    22
}

fn saved_hosts_path() -> PathBuf {
    paths::config_dir().join("ssh_hosts.json")
}

fn load_saved_host_entries() -> Vec<SavedHostEntry> {
    let path = saved_hosts_path();
    log::info!("[zed-ios] loading saved hosts from: {}", path.display());
    match std::fs::read_to_string(&path) {
        Ok(contents) => {
            let entries: Vec<SavedHostEntry> = serde_json::from_str(&contents).unwrap_or_default();
            log::info!("[zed-ios] loaded {} saved hosts", entries.len());
            entries
        }
        Err(error) => {
            log::info!("[zed-ios] no saved hosts file: {error}");
            Vec::new()
        }
    }
}

fn save_host_entries(entries: &[SavedHostEntry]) {
    let path = saved_hosts_path();
    log::info!(
        "[zed-ios] saving {} hosts to: {}",
        entries.len(),
        path.display()
    );
    if let Some(parent) = path.parent() {
        if let Err(error) = std::fs::create_dir_all(parent) {
            log::error!(
                "[zed-ios] failed to create config dir {}: {error}",
                parent.display()
            );
            return;
        }
    }
    match serde_json::to_string_pretty(entries) {
        Ok(json) => {
            if let Err(error) = std::fs::write(&path, &json) {
                log::error!("[zed-ios] failed to write hosts file: {error}");
            } else {
                log::info!("[zed-ios] saved hosts successfully");
            }
        }
        Err(error) => {
            log::error!("[zed-ios] failed to serialize saved hosts: {error}");
        }
    }
}

/// Status of a saved host connection.
enum ConnectionStatus {
    Disconnected,
    Connecting,
    /// This exact host+path has an active workspace.
    Connected,
    /// The host has an active SSH connection, but this path isn't open yet.
    /// Tapping will reuse the existing connection (instant).
    HostConnected,
    Error(SharedString),
}

/// A saved SSH host displayed in the landing list.
struct SavedHost {
    nickname: Option<SharedString>,
    host: SharedString,
    username: SharedString,
    port: u16,
    password: Option<String>,
    project_path: Option<String>,
    status: ConnectionStatus,
}

impl SavedHost {
    fn from_entry(entry: SavedHostEntry) -> Self {
        Self {
            nickname: entry.nickname.map(SharedString::from),
            host: SharedString::from(entry.host),
            username: SharedString::from(entry.username),
            port: entry.port,
            password: entry.password,
            project_path: entry.project_path,
            status: ConnectionStatus::Disconnected,
        }
    }

    fn to_entry(&self) -> SavedHostEntry {
        SavedHostEntry {
            nickname: self.nickname.as_ref().map(|s| s.to_string()),
            host: self.host.to_string(),
            username: self.username.to_string(),
            port: self.port,
            password: self.password.clone(),
            project_path: self.project_path.clone(),
        }
    }

    fn display_name(&self) -> SharedString {
        if let Some(nickname) = &self.nickname {
            nickname.clone()
        } else {
            SharedString::from(format!("{}@{}", self.username, self.host))
        }
    }

    fn address_line(&self) -> SharedString {
        if self.port == 22 {
            SharedString::from(format!("{}@{}", self.username, self.host))
        } else {
            SharedString::from(format!("{}@{}:{}", self.username, self.host, self.port))
        }
    }

    fn is_open(&self) -> bool {
        matches!(self.status, ConnectionStatus::Connected)
    }
}

enum LandingMode {
    Default,
    AddHost,
    /// Editing host-level fields (name, host, username, port, password).
    /// The `usize` is the index of any entry in the group — all entries
    /// sharing the same (host, username, port) will be updated on save.
    EditHost(usize),
    /// Editing only the project path of a single entry.
    EditProject(usize),
    /// Adding a new project path to an existing host. Connection details are
    /// pre-filled from an existing entry via `switch_to_add_project`.
    AddProjectToHost,
}

/// Landing screen shown on iPad launch. Lists saved SSH hosts and provides
/// an "Add Host" entry point. This replaces the desktop welcome page — the
/// thin client has no local filesystem, so the first thing a user does is
/// pick a remote host.
pub struct ConnectionLanding {
    focus_handle: gpui::FocusHandle,
    mode: LandingMode,
    editing_hosts: bool,
    saved_hosts: Vec<SavedHost>,
    name_editor: Entity<Editor>,
    host_editor: Entity<Editor>,
    username_editor: Entity<Editor>,
    port_editor: Entity<Editor>,
    password_editor: Entity<Editor>,
    project_path_editor: Entity<Editor>,
    password_prompt: Option<PasswordPromptState>,
    error_detail: Option<ErrorDetailState>,
}

struct ErrorDetailState {
    host_index: usize,
    message: SharedString,
}

struct PasswordPromptState {
    prompt_text: SharedString,
    editor: Entity<Editor>,
    tx: Option<futures::channel::oneshot::Sender<EncryptedPassword>>,
}

impl ConnectionLanding {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("optional display name", window, cx);
            editor
        });
        let host_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("hostname or IP address", window, cx);
            editor
        });
        let username_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("username", window, cx);
            editor
        });
        let port_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("22", window, cx);
            editor
        });
        let password_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("leave blank for key auth", window, cx);
            editor.set_masked(true, cx);
            editor
        });
        let project_path_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("~/myproject", window, cx);
            editor
        });

        let mut saved_hosts: Vec<SavedHost> = load_saved_host_entries()
            .into_iter()
            .map(SavedHost::from_entry)
            .collect();

        // Restore connection status and errors for hosts with active connections.
        if let Some(active) = cx.try_global::<ActiveConnections>() {
            for host in &mut saved_hosts {
                let path = host.project_path.as_deref().unwrap_or("");
                if active
                    .find_workspace_by_path(&host.host, &host.username, host.port, path)
                    .is_some()
                {
                    host.status = ConnectionStatus::Connected;
                } else if let Some(error_msg) =
                    active.get_project_error(&host.host, &host.username, host.port, path)
                {
                    host.status = ConnectionStatus::Error(error_msg.clone());
                } else if active
                    .find_by_host(&host.host, &host.username, host.port)
                    .is_some()
                {
                    host.status = ConnectionStatus::HostConnected;
                }
            }
        }

        Self {
            focus_handle: cx.focus_handle(),
            mode: LandingMode::Default,
            editing_hosts: false,
            saved_hosts,
            name_editor,
            host_editor,
            username_editor,
            port_editor,
            password_editor,
            project_path_editor,
            password_prompt: None,
            error_detail: None,
        }
    }

    /// Open the connection landing screen in a new window.
    pub fn open(cx: &mut App) -> anyhow::Result<()> {
        cx.open_window(WindowOptions::default(), |window, cx| {
            let landing = cx.new(|cx| Self::new(window, cx));
            landing.focus_handle(cx).focus(window, cx);
            landing
        })?;
        Ok(())
    }

    fn persist_hosts(&self) {
        let entries: Vec<SavedHostEntry> = self.saved_hosts.iter().map(|h| h.to_entry()).collect();
        save_host_entries(&entries);
    }

    fn switch_to_add_host(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = LandingMode::AddHost;
        self.name_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.host_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.username_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.port_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.password_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.project_path_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.name_editor.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn switch_to_edit_host(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(host) = self.saved_hosts.get(index) else {
            return;
        };
        let name_text = host
            .nickname
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_default();
        let host_text = host.host.to_string();
        let username_text = host.username.to_string();
        let port_text = if host.port == 22 {
            String::new()
        } else {
            host.port.to_string()
        };
        let password_text = host.password.clone().unwrap_or_default();

        self.name_editor.update(cx, |editor, cx| {
            editor.set_text(name_text, window, cx);
        });
        self.host_editor.update(cx, |editor, cx| {
            editor.set_text(host_text, window, cx);
        });
        self.username_editor.update(cx, |editor, cx| {
            editor.set_text(username_text, window, cx);
        });
        self.port_editor.update(cx, |editor, cx| {
            editor.set_text(port_text, window, cx);
        });
        self.password_editor.update(cx, |editor, cx| {
            editor.set_text(password_text, window, cx);
        });
        self.mode = LandingMode::EditHost(index);
        self.name_editor.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn switch_to_edit_project(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(host) = self.saved_hosts.get(index) else {
            return;
        };
        let project_path_text = host.project_path.clone().unwrap_or_default();
        self.project_path_editor.update(cx, |editor, cx| {
            editor.set_text(project_path_text, window, cx);
        });
        self.mode = LandingMode::EditProject(index);
        self.project_path_editor.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn switch_to_add_project(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(host) = self.saved_hosts.get(index) else {
            return;
        };
        self.name_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.host_editor.update(cx, |editor, cx| {
            editor.set_text(host.host.to_string(), window, cx);
        });
        self.username_editor.update(cx, |editor, cx| {
            editor.set_text(host.username.to_string(), window, cx);
        });
        let port_text = if host.port == 22 {
            String::new()
        } else {
            host.port.to_string()
        };
        self.port_editor.update(cx, |editor, cx| {
            editor.set_text(port_text, window, cx);
        });
        self.password_editor.update(cx, |editor, cx| {
            editor.set_text(host.password.clone().unwrap_or_default(), window, cx);
        });
        self.project_path_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.mode = LandingMode::AddProjectToHost;
        self.project_path_editor.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn cancel_add_host(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = LandingMode::Default;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn confirm_add_host(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // EditProject only touches the project path — handle it separately.
        if let LandingMode::EditProject(index) = self.mode {
            let project_path_text = self.project_path_editor.read(cx).text(cx);
            let project_path = if project_path_text.is_empty() {
                None
            } else {
                Some(project_path_text.to_string())
            };
            if let Some(entry) = self.saved_hosts.get_mut(index) {
                entry.project_path = project_path;
            }
            self.persist_hosts();
            self.mode = LandingMode::Default;
            self.focus_handle.focus(window, cx);
            cx.notify();
            return;
        }

        let name = self.name_editor.read(cx).text(cx);
        let host = self.host_editor.read(cx).text(cx);
        let username = self.username_editor.read(cx).text(cx);
        let port_text = self.port_editor.read(cx).text(cx);
        let port: u16 = port_text.parse().unwrap_or(22);
        let password_text = self.password_editor.read(cx).text(cx);
        let password = if password_text.is_empty() {
            None
        } else {
            Some(password_text.to_string())
        };
        let project_path_text = self.project_path_editor.read(cx).text(cx);
        let project_path = if project_path_text.is_empty() {
            None
        } else {
            Some(project_path_text.to_string())
        };

        if host.is_empty() || username.is_empty() {
            return;
        }

        let nickname = if name.is_empty() {
            None
        } else {
            Some(SharedString::from(name.clone()))
        };

        match self.mode {
            LandingMode::EditHost(index) if index < self.saved_hosts.len() => {
                // Find the old group key so we can update all entries that share it.
                let old_host = self.saved_hosts[index].host.clone();
                let old_username = self.saved_hosts[index].username.clone();
                let old_port = self.saved_hosts[index].port;

                for entry in &mut self.saved_hosts {
                    if entry.host == old_host
                        && entry.username == old_username
                        && entry.port == old_port
                    {
                        entry.nickname = nickname.clone();
                        entry.host = SharedString::from(host.clone());
                        entry.username = SharedString::from(username.clone());
                        entry.port = port;
                        entry.password = password.clone();
                    }
                }
            }
            _ => {
                let updated = SavedHost {
                    nickname,
                    host: SharedString::from(host),
                    username: SharedString::from(username),
                    port,
                    password,
                    project_path,
                    status: ConnectionStatus::Disconnected,
                };
                self.saved_hosts.push(updated);
            }
        }
        self.persist_hosts();

        self.editing_hosts = false;
        self.mode = LandingMode::Default;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn connect_host(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(host) = self.saved_hosts.get_mut(index) else {
            return;
        };

        let path = host.project_path.clone().unwrap_or_default();
        let error_host = host.host.to_string();
        let error_username = host.username.to_string();
        let error_port = host.port;
        let error_path = path.clone();

        // Clear any previous error for this project.
        active_connections_mut(cx).clear_project_error(
            &error_host,
            &error_username,
            error_port,
            &error_path,
        );

        // Case 1: Exact host+path already open → activate that workspace.
        if let Some(active) = cx
            .try_global::<ActiveConnections>()
            .and_then(|ac| ac.find_by_host(&host.host, &host.username, host.port))
        {
            let target_workspace = active.workspaces.iter().find(|w| w.path == path);
            if let Some(entry) = target_workspace {
                let target = entry.workspace.clone();
                let all_workspaces: Vec<_> = active
                    .workspaces
                    .iter()
                    .map(|w| w.workspace.clone())
                    .collect();
                show_multi_workspace(window, cx, &all_workspaces, &target);
                return;
            }

            // Case 2: Host connected but new path → create workspace on shared connection.
            let remote_connection = active.remote_connection.clone();
            host.status = ConnectionStatus::Connecting;
            cx.notify();

            let app_state = match crate::ios::app_state() {
                Some(state) => state,
                None => {
                    log::error!("[zed-ios] app_state not available for SSH connection");
                    if let Some(host) = self.saved_hosts.get_mut(index) {
                        host.status = ConnectionStatus::Error("App not initialized".into());
                        cx.notify();
                    }
                    return;
                }
            };

            let landing_window = window.window_handle();
            let delegate = Arc::new(IosRemoteClientDelegate {
                window: landing_window,
            });
            let host_str = host.host.to_string();
            let username_str = host.username.to_string();
            let port_val = host.port;

            cx.spawn({
                let error_host = error_host.clone();
                let error_username = error_username.clone();
                let error_path = error_path.clone();
                async move |this, cx| {
                    let result = Self::add_workspace_for_path(
                        landing_window,
                        remote_connection,
                        path.clone(),
                        host_str.clone(),
                        username_str.clone(),
                        port_val,
                        delegate,
                        app_state,
                        cx,
                    )
                    .await;

                    if let Err(error) = result {
                        let error_message = format!("{error:#}");
                        log::error!("[zed-ios] workspace creation failed: {error_message}");
                        let error_shared = SharedString::from(error_message.clone());
                        cx.update(|cx| {
                            active_connections_mut(cx).set_project_error(
                                &error_host,
                                &error_username,
                                error_port,
                                &error_path,
                                error_shared,
                            );
                        });
                        let updated = this
                            .update(cx, |this, cx| {
                                if let Some(host) = this.saved_hosts.get_mut(index) {
                                    host.status = ConnectionStatus::Error(SharedString::from(
                                        error_message.clone(),
                                    ));
                                    cx.notify();
                                }
                            })
                            .is_ok();
                        if !updated {
                            cx.update(|cx| Self::navigate_to_landing(landing_window, cx));
                        }
                    }
                }
            })
            .detach();
            return;
        }

        // Case 3: Host not connected → full connect flow.
        host.status = ConnectionStatus::Connecting;
        cx.notify();

        let connection_options = SshConnectionOptions {
            host: host.host.to_string().into(),
            username: Some(host.username.to_string()),
            port: Some(host.port),
            password: host.password.clone(),
            ..Default::default()
        };

        let app_state = match crate::ios::app_state() {
            Some(state) => state,
            None => {
                log::error!("[zed-ios] app_state not available for SSH connection");
                if let Some(host) = self.saved_hosts.get_mut(index) {
                    host.status = ConnectionStatus::Error("App not initialized".into());
                    cx.notify();
                }
                return;
            }
        };

        let landing_window = window.window_handle();
        let delegate = Arc::new(IosRemoteClientDelegate {
            window: landing_window,
        });

        cx.spawn(async move |this, cx| {
            let result = Self::do_connect(
                landing_window,
                connection_options,
                path,
                delegate.clone(),
                app_state,
                cx,
            )
            .await;

            if let Err(error) = result {
                let error_message = format!("{error:#}");
                log::error!("[zed-ios] SSH connection failed: {error_message}");

                let error_shared = SharedString::from(error_message.clone());
                cx.update(|cx| {
                    active_connections_mut(cx).set_project_error(
                        &error_host,
                        &error_username,
                        error_port,
                        &error_path,
                        error_shared,
                    );
                });

                let updated = this
                    .update(cx, |this, cx| {
                        if let Some(host) = this.saved_hosts.get_mut(index) {
                            host.status =
                                ConnectionStatus::Error(SharedString::from(error_message.clone()));
                            cx.notify();
                        }
                    })
                    .is_ok();

                if !updated {
                    cx.update(|cx| Self::navigate_to_landing(landing_window, cx));
                }
            }
        })
        .detach();
    }

    /// Create a RemoteClient + Project + Workspace for a given path on an
    /// existing SSH connection. Returns the workspace and project entities.
    async fn create_workspace_for_path(
        landing_window: AnyWindowHandle,
        remote_connection: Arc<dyn remote::RemoteConnection>,
        path: &str,
        host: &str,
        username: &str,
        delegate: Arc<dyn remote::RemoteClientDelegate>,
        app_state: Arc<workspace::AppState>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<(Entity<workspace::Workspace>, Entity<project::Project>)> {
        let (cancel_tx, cancel_rx) = futures::channel::oneshot::channel::<()>();
        std::mem::forget(cancel_tx);

        log::info!("[zed-ios] creating remote client session for path: {path}");
        let session = match cx
            .update(|cx| {
                remote::RemoteClient::new(
                    remote::ConnectionIdentifier::setup(),
                    remote_connection,
                    cancel_rx,
                    delegate.clone(),
                    cx,
                )
            })
            .await?
        {
            Some(session) => session,
            None => anyhow::bail!("SSH connection was cancelled"),
        };

        log::info!("[zed-ios] creating remote project");
        let project = cx.update(|cx| {
            project::Project::remote(
                session,
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                true,
                cx,
            )
        });

        log::info!("[zed-ios] resolving project path: {path}");
        let (_worktree, project_path) = cx
            .update(|cx| {
                workspace::Workspace::project_path_for_path(
                    project.clone(),
                    &PathBuf::from(path),
                    true,
                    cx,
                )
            })
            .await
            .map_err(|error| anyhow::anyhow!("Could not open '{path}': {error:#}"))?;

        let workspace_entity = landing_window.update(cx, |_, window, cx| {
            let workspace = cx.new(|cx| {
                workspace::Workspace::new(None, project.clone(), app_state.clone(), window, cx)
            });

            let switcher = cx.new(|_cx| WorkspaceSwitcher::new(path, host, username));
            let suffix = cx.new(|_cx| StatusBarSuffix);
            workspace.update(cx, |ws, cx| {
                ws.set_status_bar_prefix(switcher.into(), cx);
                ws.set_status_bar_suffix(suffix.into(), cx);
            });

            workspace
        })?;

        // Open the resolved project path if it points to a file.
        if !project_path.path.is_empty() {
            log::info!("[zed-ios] opening project path: {:?}", project_path);
            let open_result = landing_window.update(cx, |_, window, cx| {
                workspace_entity.update(cx, |workspace, cx| {
                    workspace.open_path(project_path, None, true, window, cx)
                })
            })?;
            match open_result.await {
                Ok(_) => log::info!("[zed-ios] opened file successfully"),
                Err(error) => log::error!("[zed-ios] failed to open file: {error:#}"),
            }
        }

        Ok((workspace_entity, project))
    }

    /// Subscribe to project close events so we can clean up the active
    /// connection entry. If sibling workspaces remain on the same host,
    /// switch to one of them. Otherwise navigate home.
    fn subscribe_project_closed(
        landing_window: AnyWindowHandle,
        project: &Entity<project::Project>,
        cx: &mut App,
    ) {
        cx.subscribe(project, {
            let landing_window = landing_window;
            move |project_entity, event: &project::Event, cx| {
                if matches!(event, project::Event::Closed) {
                    log::info!("[zed-ios] project closed, cleaning up");
                    let project_id = project_entity.entity_id();
                    let landing_window = landing_window;
                    cx.defer(move |cx| {
                        let active = active_connections_mut(cx);
                        active.remove_by_project(project_id);

                        // Find the host that still has workspaces (sibling).
                        // Collect remaining workspaces to switch to if any exist.
                        let remaining: Option<Vec<Entity<workspace::Workspace>>> = active
                            .hosts
                            .iter()
                            .find(|hc| !hc.workspaces.is_empty())
                            .map(|hc| hc.workspaces.iter().map(|w| w.workspace.clone()).collect());

                        if let Some(workspaces) = remaining {
                            let target = workspaces[0].clone();
                            landing_window
                                .update(cx, |_, window, cx| {
                                    show_multi_workspace(window, cx, &workspaces, &target);
                                })
                                .log_err();
                        } else {
                            Self::navigate_to_landing(landing_window, cx);
                        }
                    });
                }
            }
        })
        .detach();
    }

    /// Case 2: Add a new workspace (path) to an already-connected host.
    async fn add_workspace_for_path(
        landing_window: AnyWindowHandle,
        remote_connection: Arc<dyn remote::RemoteConnection>,
        path: String,
        host: String,
        username: String,
        port: u16,
        delegate: Arc<dyn remote::RemoteClientDelegate>,
        app_state: Arc<workspace::AppState>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        if path.is_empty() {
            anyhow::bail!("No project path specified. Edit the host to add one.");
        }

        let (workspace_entity, project) = Self::create_workspace_for_path(
            landing_window,
            remote_connection,
            &path,
            &host,
            &username,
            delegate,
            app_state,
            cx,
        )
        .await?;

        landing_window.update(cx, |_, window, cx| {
            // Add workspace entry to the existing host connection.
            let active_conns = active_connections_mut(cx);
            if let Some(host_conn) = active_conns.find_by_host_mut(&host, &username, port) {
                host_conn.workspaces.push(WorkspaceEntry {
                    workspace: workspace_entity.clone(),
                    project: project.clone(),
                    path,
                });

                let all_workspaces: Vec<_> = host_conn
                    .workspaces
                    .iter()
                    .map(|w| w.workspace.clone())
                    .collect();
                show_multi_workspace(window, cx, &all_workspaces, &workspace_entity);
            }

            Self::subscribe_project_closed(landing_window, &project, cx);
        })?;

        log::info!("[zed-ios] added workspace to existing host connection");
        Ok(())
    }

    /// Case 3: Full connect flow — establish SSH, create first workspace,
    /// create MultiWorkspace, register host connection, and replace_root.
    async fn do_connect(
        landing_window: AnyWindowHandle,
        connection_options: SshConnectionOptions,
        path: String,
        delegate: Arc<dyn remote::RemoteClientDelegate>,
        app_state: Arc<workspace::AppState>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        if path.is_empty() {
            anyhow::bail!("No project path specified. Edit the host to add one.");
        }

        // 1. Establish SSH connection.
        let connection =
            remote::RusshRemoteConnection::new(connection_options.clone(), delegate.clone(), cx)
                .await?;
        let remote_connection: Arc<dyn remote::RemoteConnection> = Arc::new(connection);

        // 2. Create workspace for the path.
        let host_str = connection_options.host.to_string();
        let username_str = connection_options.username.clone().unwrap_or_default();
        let port_val = connection_options.port.unwrap_or(22);

        let (workspace_entity, project) = Self::create_workspace_for_path(
            landing_window,
            remote_connection.clone(),
            &path,
            &host_str,
            &username_str,
            delegate,
            app_state,
            cx,
        )
        .await?;

        // 3. Create MultiWorkspace, register in ActiveConnections, replace_root.
        landing_window.update(cx, |_, window, cx| {
            window.replace_root(cx, |window, cx| {
                workspace::MultiWorkspace::new(workspace_entity.clone(), window, cx)
            });

            active_connections_mut(cx).hosts.push(HostConnection {
                remote_connection,
                workspaces: vec![WorkspaceEntry {
                    workspace: workspace_entity,
                    project: project.clone(),
                    path,
                }],
                host: host_str,
                username: username_str,
                port: port_val,
            });

            Self::subscribe_project_closed(landing_window, &project, cx);
        })?;

        log::info!("[zed-ios] remote project opened successfully");
        Ok(())
    }

    /// Replace the window root with a fresh ConnectionLanding screen.
    fn navigate_to_landing(window: AnyWindowHandle, cx: &mut App) {
        window
            .update(cx, |_, window, cx| {
                let landing =
                    window.replace_root(cx, |window, cx| ConnectionLanding::new(window, cx));
                landing.focus_handle(cx).focus(window, cx);
            })
            .log_err();
    }

    fn toggle_editing_hosts(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editing_hosts = !self.editing_hosts;
        cx.notify();
    }

    fn show_password_prompt(
        &mut self,
        prompt: String,
        tx: futures::channel::oneshot::Sender<EncryptedPassword>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("enter password", window, cx);
            editor.set_masked(true, cx);
            editor
        });
        editor.focus_handle(cx).focus(window, cx);
        self.password_prompt = Some(PasswordPromptState {
            prompt_text: SharedString::from(prompt),
            editor,
            tx: Some(tx),
        });
        cx.notify();
    }

    fn submit_password(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.password_prompt {
            if let Some(tx) = state.tx.take() {
                let password_text = state.editor.read(cx).text(cx);
                match EncryptedPassword::try_from(password_text.as_ref()) {
                    Ok(encrypted) => {
                        tx.send(encrypted).ok();
                    }
                    Err(error) => {
                        log::error!("[zed-ios] failed to encrypt password: {error}");
                    }
                }
            }
        }
        self.password_prompt = None;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn cancel_password(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.password_prompt {
            state.tx.take();
        }
        self.password_prompt = None;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn render_password_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self
            .password_prompt
            .as_ref()
            .expect("called without prompt");
        let colors = cx.theme().colors();

        div()
            .id("password-overlay")
            .absolute()
            .size_full()
            .bg(colors.background.opacity(0.9))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(Label::new(state.prompt_text.clone()).color(Color::Default))
            .child(
                div()
                    .w_80()
                    .rounded_md()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.editor_background)
                    .px_2()
                    .py_1()
                    .child(state.editor.clone()),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        ui::Button::new("pw-cancel", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.cancel_password(window, cx);
                            })),
                    )
                    .child(
                        ui::Button::new("pw-submit", "Submit")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.submit_password(window, cx);
                            })),
                    ),
            )
    }

    fn render_error_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self
            .error_detail
            .as_ref()
            .expect("called without error detail");
        let colors = cx.theme().colors();
        let host_index = state.host_index;

        div()
            .id("error-overlay")
            .absolute()
            .size_full()
            .bg(colors.background.opacity(0.9))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Icon::new(IconName::Warning)
                    .size(IconSize::Medium)
                    .color(Color::Error),
            )
            .child(
                div()
                    .w(rems_from_px(480.))
                    .px_4()
                    .child(Label::new(state.message.clone()).color(Color::Muted)),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        ui::Button::new("error-dismiss", "Dismiss")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.error_detail = None;
                                cx.notify();
                            })),
                    )
                    .child(
                        ui::Button::new("error-retry", "Retry")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.error_detail = None;
                                this.connect_host(host_index, window, cx);
                            })),
                    ),
            )
    }

    fn remove_host(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        if index < self.saved_hosts.len() {
            self.saved_hosts.remove(index);
            self.persist_hosts();
            cx.notify();
        }
    }

    fn render_header(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .items_center()
            .gap_4()
            .child(
                h_flex()
                    .justify_center()
                    .gap_4()
                    .child(Vector::square(VectorName::ZedLogo, rems_from_px(45.)))
                    .child(
                        v_flex()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Headline::new("Welcome to Zed"))
                                    .child(
                                        Icon::new(IconName::Tablet)
                                            .size(IconSize::Medium)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(
                                Label::new("The editor for what's next")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .italic(),
                            ),
                    ),
            )
            .child(Label::new("Connect to a remote host to start editing").color(Color::Muted))
    }

    fn render_hosts_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let host_count = self.saved_hosts.len();

        let editing = self.editing_hosts;
        let has_hosts = !self.saved_hosts.is_empty();

        let mut list = v_flex().w(rems_from_px(540.)).gap_2().child(
            h_flex()
                .justify_between()
                .items_center()
                .child(
                    Label::new("SAVED HOSTS")
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                )
                .when(has_hosts, |this| {
                    this.child(
                        IconButton::new("edit-hosts-btn", IconName::Pencil)
                            .size(ui::ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(if editing { Color::Accent } else { Color::Muted })
                            .style(ButtonStyle::Transparent)
                            .on_click(cx.listener(Self::toggle_editing_hosts)),
                    )
                }),
        );

        if self.saved_hosts.is_empty() {
            list = list.child(
                div()
                    .rounded_lg()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.surface_background)
                    .p_4()
                    .child(Label::new("No saved hosts yet").color(Color::Muted)),
            );
        } else {
            // Group entries by (host, username, port) to show projects under each host.
            let mut groups: Vec<(String, String, u16, Vec<usize>)> = Vec::new();
            for (index, host) in self.saved_hosts.iter().enumerate() {
                if let Some(group) = groups.iter_mut().find(|(h, u, p, _)| {
                    h == host.host.as_ref() && u == host.username.as_ref() && *p == host.port
                }) {
                    group.3.push(index);
                } else {
                    groups.push((
                        host.host.to_string(),
                        host.username.to_string(),
                        host.port,
                        vec![index],
                    ));
                }
            }

            let border = colors.border;
            let surface_bg = colors.surface_background;
            let hover_bg = colors.ghost_element_hover;
            let mut first_group = true;

            for (_host, _username, _port, indices) in &groups {
                let first_idx = indices[0];
                let host = &self.saved_hosts[first_idx];
                let display_name = host.display_name();
                let address_line = host.address_line();

                // Host header reflects connection-level state only, not project errors.
                let open_count = indices
                    .iter()
                    .filter(|&&i| matches!(self.saved_hosts[i].status, ConnectionStatus::Connected))
                    .count();
                let total_count = indices.len();
                let host_has_connection = indices.iter().any(|&i| {
                    matches!(
                        self.saved_hosts[i].status,
                        ConnectionStatus::Connected | ConnectionStatus::HostConnected
                    )
                });
                let has_connecting = indices
                    .iter()
                    .any(|&i| matches!(self.saved_hosts[i].status, ConnectionStatus::Connecting));

                let (group_icon_color, group_status_label, group_status_color) = if has_connecting {
                    (
                        Color::Warning,
                        "Connecting\u{2026}".to_string(),
                        Color::Default,
                    )
                } else if host_has_connection {
                    (
                        Color::Default,
                        format!("Connected ({open_count}/{total_count})"),
                        Color::Default,
                    )
                } else {
                    (Color::Muted, "Disconnected".to_string(), Color::Muted)
                };

                if !first_group {
                    list = list.child(div().h_2());
                }
                first_group = false;

                let mut group_container = div()
                    .tab_group()
                    .rounded_lg()
                    .border_1()
                    .border_color(border)
                    .bg(surface_bg)
                    .overflow_hidden();

                // Host header row.
                let add_project_index = first_idx;
                let edit_host_index = first_idx;
                group_container = group_container.child(
                    h_flex()
                        .id(SharedString::from(format!("host-header-{first_idx}")))
                        .px_4()
                        .py_2()
                        .gap_3()
                        .items_center()
                        .when(editing, |this| {
                            this.cursor_pointer()
                                .hover(move |style| style.bg(hover_bg))
                                .on_click(cx.listener(move |this, _event, window, cx| {
                                    this.switch_to_edit_host(edit_host_index, window, cx);
                                }))
                        })
                        .child(
                            Icon::new(IconName::Server)
                                .size(IconSize::Small)
                                .color(group_icon_color),
                        )
                        .child(
                            v_flex()
                                .child(Label::new(display_name).color(Color::Default))
                                .child(
                                    Label::new(address_line)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                        .child(div().flex_grow())
                        .child(
                            Label::new(group_status_label)
                                .size(LabelSize::XSmall)
                                .color(group_status_color),
                        )
                        .when(editing, |this| {
                            this.child(
                                IconButton::new(
                                    SharedString::from(format!("add-project-{first_idx}")),
                                    IconName::Plus,
                                )
                                .size(ui::ButtonSize::Compact)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .style(ButtonStyle::Transparent)
                                .on_click(cx.listener(
                                    move |this, _event, window, cx| {
                                        this.switch_to_add_project(add_project_index, window, cx);
                                    },
                                )),
                            )
                        }),
                );

                // Project path sub-entries.
                for (sub_idx, &index) in indices.iter().enumerate() {
                    group_container = group_container.child(div().mx_4().h_px().bg(border));
                    group_container =
                        group_container.child(self.render_project_entry(index, sub_idx, cx));
                }

                list = list.child(group_container);
            }
        }

        let show_add_host = self.saved_hosts.is_empty() || self.editing_hosts;
        list.when(show_add_host, |this| {
            this.child(
                ui::Button::new("add-host-btn", "Add Remote Host")
                    .tab_index(host_count as isize)
                    .start_icon(Icon::new(IconName::Plus))
                    .full_width()
                    .style(ButtonStyle::Filled)
                    .on_click(cx.listener(Self::switch_to_add_host)),
            )
        })
    }

    fn render_project_entry(
        &self,
        index: usize,
        sub_index: usize,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let host = &self.saved_hosts[index];
        let colors = cx.theme().colors();
        let path_label = host.project_path.as_deref().unwrap_or("(no project path)");
        let path_label = SharedString::from(path_label.to_string());
        let is_open = host.is_open();
        let is_error = matches!(host.status, ConnectionStatus::Error(_));
        let error_message = if let ConnectionStatus::Error(msg) = &host.status {
            Some(msg.clone())
        } else {
            None
        };
        let truncated_error = error_message.as_ref().map(|msg| {
            let s = msg.to_string();
            if s.len() > 40 {
                SharedString::from(format!("{}…", &s[..40]))
            } else {
                msg.clone()
            }
        });
        let is_editing = self.editing_hosts;
        let is_connectable = !is_editing
            && !is_error
            && matches!(
                host.status,
                ConnectionStatus::Disconnected
                    | ConnectionStatus::Connected
                    | ConnectionStatus::HostConnected
            );

        let hover_bg = colors.ghost_element_hover;
        let focus_border = colors.border_focused;

        div()
            .id(SharedString::from(format!("project-{index}-{sub_index}")))
            .tab_index(index as isize)
            .w_full()
            .pl_11()
            .pr_4()
            .py_2()
            .flex()
            .items_center()
            .justify_between()
            .cursor_pointer()
            .border_2()
            .border_color(gpui::transparent_black())
            .hover(move |style| style.bg(hover_bg))
            .focus(move |style| style.border_color(focus_border))
            .when(is_connectable, |this| {
                this.on_click(cx.listener(move |this, _event, window, cx| {
                    this.connect_host(index, window, cx);
                }))
            })
            .when(is_error && !is_editing, |this| {
                this.on_click(cx.listener(move |this, _event, _window, cx| {
                    if let Some(ConnectionStatus::Error(msg)) =
                        this.saved_hosts.get(index).map(|h| &h.status)
                    {
                        this.error_detail = Some(ErrorDetailState {
                            host_index: index,
                            message: msg.clone(),
                        });
                        cx.notify();
                    }
                }))
            })
            .when(is_editing, |this| {
                this.on_click(cx.listener(move |this, _event, window, cx| {
                    this.switch_to_edit_project(index, window, cx);
                }))
            })
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Icon::new(if is_open {
                            IconName::FolderOpen
                        } else {
                            IconName::Folder
                        })
                        .size(IconSize::Small)
                        .color(if is_error {
                            Color::Error
                        } else if is_open {
                            Color::Accent
                        } else {
                            Color::Muted
                        }),
                    )
                    .child(
                        Label::new(path_label)
                            .size(LabelSize::Small)
                            .color(Color::Default),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .when_some(truncated_error, |this, msg| {
                        this.child(Label::new(msg).size(LabelSize::XSmall).color(Color::Muted))
                    })
                    .when(is_error && !self.editing_hosts, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!("retry-host-{index}")),
                                IconName::RotateCw,
                            )
                            .size(ui::ButtonSize::Compact)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .style(ButtonStyle::Transparent)
                            .on_click(cx.listener(
                                move |this, _event, window, cx| {
                                    this.connect_host(index, window, cx);
                                },
                            )),
                        )
                    })
                    .when(self.editing_hosts, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!("remove-host-{index}")),
                                IconName::Trash,
                            )
                            .size(ui::ButtonSize::Compact)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .style(ButtonStyle::Transparent)
                            .on_click(cx.listener(
                                move |this, _event, window, cx| {
                                    this.remove_host(index, window, cx);
                                },
                            )),
                        )
                    }),
            )
    }

    fn render_add_host_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let show_host_fields = matches!(self.mode, LandingMode::AddHost | LandingMode::EditHost(_));
        let show_project_field = !matches!(self.mode, LandingMode::EditHost(_));

        let colors = cx.theme().colors();
        let name_focus = self
            .name_editor
            .focus_handle(cx)
            .tab_index(0)
            .tab_stop(true);
        let host_focus = self
            .host_editor
            .focus_handle(cx)
            .tab_index(1)
            .tab_stop(true);
        let username_focus = self
            .username_editor
            .focus_handle(cx)
            .tab_index(2)
            .tab_stop(true);
        let port_focus = self
            .port_editor
            .focus_handle(cx)
            .tab_index(3)
            .tab_stop(true);
        let password_focus = self
            .password_editor
            .focus_handle(cx)
            .tab_index(4)
            .tab_stop(true);
        let project_path_focus = self
            .project_path_editor
            .focus_handle(cx)
            .tab_index(if show_host_fields { 5 } else { 0 })
            .tab_stop(true);

        let (form_title, confirm_label) = match &self.mode {
            LandingMode::AddProjectToHost | LandingMode::EditProject(_) => {
                if matches!(self.mode, LandingMode::EditProject(_)) {
                    ("EDIT PROJECT PATH", "Save")
                } else {
                    ("ADD PROJECT", "Add Project")
                }
            }
            LandingMode::EditHost(_) => ("EDIT HOST", "Save"),
            _ => ("NEW CONNECTION", "Add Host"),
        };

        let border = colors.border;
        let editor_bg = colors.editor_background;

        let mut form_fields = div()
            .tab_group()
            .rounded_lg()
            .border_1()
            .border_color(colors.border)
            .bg(colors.surface_background)
            .p_4()
            .flex()
            .flex_col()
            .gap_3();

        if show_host_fields {
            form_fields = form_fields
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            Label::new("Name")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            div()
                                .id("name-field")
                                .track_focus(&name_focus)
                                .rounded_md()
                                .border_1()
                                .border_color(border)
                                .bg(editor_bg)
                                .px_2()
                                .py_1()
                                .child(self.name_editor.clone()),
                        ),
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            Label::new("Host")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            div()
                                .id("host-field")
                                .track_focus(&host_focus)
                                .rounded_md()
                                .border_1()
                                .border_color(border)
                                .bg(editor_bg)
                                .px_2()
                                .py_1()
                                .child(self.host_editor.clone()),
                        ),
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            Label::new("Username")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            div()
                                .id("username-field")
                                .track_focus(&username_focus)
                                .rounded_md()
                                .border_1()
                                .border_color(border)
                                .bg(editor_bg)
                                .px_2()
                                .py_1()
                                .child(self.username_editor.clone()),
                        ),
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            Label::new("Port")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            div()
                                .id("port-field")
                                .track_focus(&port_focus)
                                .rounded_md()
                                .border_1()
                                .border_color(border)
                                .bg(editor_bg)
                                .px_2()
                                .py_1()
                                .child(self.port_editor.clone()),
                        ),
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            Label::new("Password")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            div()
                                .id("password-field")
                                .track_focus(&password_focus)
                                .rounded_md()
                                .border_1()
                                .border_color(border)
                                .bg(editor_bg)
                                .px_2()
                                .py_1()
                                .child(self.password_editor.clone()),
                        ),
                );
        }

        if show_project_field {
            form_fields = form_fields.child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new("Project Path")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        div()
                            .id("project-path-field")
                            .track_focus(&project_path_focus)
                            .rounded_md()
                            .border_1()
                            .border_color(border)
                            .bg(editor_bg)
                            .px_2()
                            .py_1()
                            .child(self.project_path_editor.clone()),
                    ),
            );
        }

        v_flex()
            .id("add-host-form")
            .w(rems_from_px(540.))
            .gap_4()
            .child(
                Label::new(form_title)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .child(form_fields)
            .child(
                h_flex()
                    .gap_2()
                    .justify_end()
                    .child(
                        ui::Button::new("cancel-btn", "Cancel")
                            .tab_index(6_isize)
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(Self::cancel_add_host)),
                    )
                    .child(
                        ui::Button::new("add-host-confirm-btn", confirm_label)
                            .tab_index(7_isize)
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(Self::confirm_add_host)),
                    ),
            )
    }
}

impl Render for ConnectionLanding {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let insets = window.safe_area_insets();

        let mut content = div()
            .id("connection-landing")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Tab, window, cx| {
                window.focus_next(cx);
            })
            .on_action(|_: &Backtab, window, cx| {
                window.focus_prev(cx);
            })
            .size_full()
            .bg(colors.background)
            .pt(insets.top)
            .pb(insets.bottom)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .child(self.render_header(cx));

        match &self.mode {
            LandingMode::Default => {
                content = content.child(self.render_hosts_list(cx));
            }
            LandingMode::AddHost
            | LandingMode::EditHost(_)
            | LandingMode::EditProject(_)
            | LandingMode::AddProjectToHost => {
                content = content.child(self.render_add_host_form(cx));
            }
        }

        if self.password_prompt.is_some() {
            content = content.child(self.render_password_overlay(cx));
        }

        if self.error_detail.is_some() {
            content = content.child(self.render_error_overlay(cx));
        }

        content
    }
}

impl gpui::Focusable for ConnectionLanding {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

struct IosRemoteClientDelegate {
    window: AnyWindowHandle,
}

impl remote::RemoteClientDelegate for IosRemoteClientDelegate {
    fn ask_password(
        &self,
        prompt: String,
        tx: futures::channel::oneshot::Sender<askpass::EncryptedPassword>,
        cx: &mut AsyncApp,
    ) {
        log::info!("[zed-ios] Password prompt requested: {prompt}");
        let result = self.window.update(cx, |_, window, cx| {
            if let Some(Some(landing)) = window.root::<ConnectionLanding>() {
                landing.update(cx, |landing, cx| {
                    landing.show_password_prompt(prompt, tx, window, cx);
                });
            } else {
                log::error!("[zed-ios] cannot show password prompt: landing screen not active");
            }
        });
        if let Err(error) = result {
            log::error!("[zed-ios] failed to show password prompt: {error:#}");
        }
    }

    fn get_download_url(
        &self,
        _platform: remote::RemotePlatform,
        _release_channel: release_channel::ReleaseChannel,
        _version: Option<semver::Version>,
        _cx: &mut AsyncApp,
    ) -> Task<anyhow::Result<Option<String>>> {
        Task::ready(Ok(None))
    }

    fn download_server_binary_locally(
        &self,
        _platform: remote::RemotePlatform,
        _release_channel: release_channel::ReleaseChannel,
        _version: Option<semver::Version>,
        _cx: &mut AsyncApp,
    ) -> Task<anyhow::Result<PathBuf>> {
        Task::ready(Err(anyhow::anyhow!(
            "server binary download not supported on iOS"
        )))
    }

    fn set_status(&self, status: Option<&str>, _cx: &mut AsyncApp) {
        if let Some(status) = status {
            log::info!("[zed-ios] SSH status: {status}");
        }
    }
}

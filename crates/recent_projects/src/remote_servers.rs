use crate::{
    remote_connections::{
        Connection, RemoteConnectionModal, RemoteConnectionPrompt, SshConnection,
        SshConnectionHeader, SshSettings, connect, determine_paths_with_positions,
        open_remote_project,
    },
    ssh_config::parse_ssh_config_hosts,
};
use editor::Editor;
use file_finder::OpenPathDelegate;
use futures::{FutureExt, channel::oneshot, future::Shared, select};
use gpui::{
    AnyElement, App, ClickEvent, ClipboardItem, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, PromptLevel, ScrollHandle, Subscription, Task, WeakEntity, Window,
    canvas,
};
use language::Point;
use log::info;
use paths::{global_ssh_config_file, user_ssh_config_file};
use picker::Picker;
use project::{Fs, Project};
use remote::{
    RemoteClient, RemoteConnectionOptions, SshConnectionOptions, WslConnectionOptions,
    remote_client::ConnectionIdentifier,
};
use settings::{
    RemoteSettingsContent, Settings as _, SettingsStore, SshProject, update_settings_file,
    watch_config_file,
};
use smol::stream::StreamExt as _;
use std::{
    borrow::Cow,
    collections::BTreeSet,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
};
use ui::{
    IconButtonShape, List, ListItem, ListSeparator, Modal, ModalHeader, Navigable, NavigableEntry,
    Section, Tooltip, WithScrollbar, prelude::*,
};
use util::{
    ResultExt,
    paths::{PathStyle, RemotePathBuf},
};
use workspace::{
    ModalView, OpenOptions, Toast, Workspace,
    notifications::{DetachAndPromptErr, NotificationId},
    open_remote_project_with_existing_connection,
};

pub struct RemoteServerProjects {
    mode: Mode,
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    retained_connections: Vec<Entity<RemoteClient>>,
    ssh_config_updates: Task<()>,
    ssh_config_servers: BTreeSet<SharedString>,
    create_new_window: bool,
    _subscription: Subscription,
}

struct CreateRemoteServer {
    address_editor: Entity<Editor>,
    address_error: Option<SharedString>,
    ssh_prompt: Option<Entity<RemoteConnectionPrompt>>,
    _creating: Option<Task<Option<()>>>,
}

impl CreateRemoteServer {
    fn new(window: &mut Window, cx: &mut App) -> Self {
        let address_editor = cx.new(|cx| Editor::single_line(window, cx));
        address_editor.update(cx, |this, cx| {
            this.focus_handle(cx).focus(window);
        });
        Self {
            address_editor,
            address_error: None,
            ssh_prompt: None,
            _creating: None,
        }
    }
}

#[cfg(target_os = "windows")]
struct AddWslDistro {
    picker: Entity<Picker<crate::wsl_picker::WslPickerDelegate>>,
    connection_prompt: Option<Entity<RemoteConnectionPrompt>>,
    _creating: Option<Task<()>>,
}

#[cfg(target_os = "windows")]
impl AddWslDistro {
    fn new(window: &mut Window, cx: &mut Context<RemoteServerProjects>) -> Self {
        use crate::wsl_picker::{WslDistroSelected, WslPickerDelegate, WslPickerDismissed};

        let delegate = WslPickerDelegate::new();
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));

        cx.subscribe_in(
            &picker,
            window,
            |this, _, _: &WslDistroSelected, window, cx| {
                this.confirm(&menu::Confirm, window, cx);
            },
        )
        .detach();

        cx.subscribe_in(
            &picker,
            window,
            |this, _, _: &WslPickerDismissed, window, cx| {
                this.cancel(&menu::Cancel, window, cx);
            },
        )
        .detach();

        AddWslDistro {
            picker,
            connection_prompt: None,
            _creating: None,
        }
    }
}

enum ProjectPickerData {
    Ssh {
        connection_string: SharedString,
        nickname: Option<SharedString>,
    },
    Wsl {
        distro_name: SharedString,
    },
}

struct ProjectPicker {
    data: ProjectPickerData,
    picker: Entity<Picker<OpenPathDelegate>>,
    _path_task: Shared<Task<Option<()>>>,
}

struct EditNicknameState {
    index: SshServerIndex,
    editor: Entity<Editor>,
}

impl EditNicknameState {
    fn new(index: SshServerIndex, window: &mut Window, cx: &mut App) -> Self {
        let this = Self {
            index,
            editor: cx.new(|cx| Editor::single_line(window, cx)),
        };
        let starting_text = SshSettings::get_global(cx)
            .ssh_connections()
            .nth(index.0)
            .and_then(|state| state.nickname)
            .filter(|text| !text.is_empty());
        this.editor.update(cx, |this, cx| {
            this.set_placeholder_text("Add a nickname for this server", window, cx);
            if let Some(starting_text) = starting_text {
                this.set_text(starting_text, window, cx);
            }
        });
        this.editor.focus_handle(cx).focus(window);
        this
    }
}

impl Focusable for ProjectPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl ProjectPicker {
    fn new(
        create_new_window: bool,
        index: ServerIndex,
        connection: RemoteConnectionOptions,
        project: Entity<Project>,
        home_dir: RemotePathBuf,
        path_style: PathStyle,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<RemoteServerProjects>,
    ) -> Entity<Self> {
        let (tx, rx) = oneshot::channel();
        let lister = project::DirectoryLister::Project(project.clone());
        let delegate = file_finder::OpenPathDelegate::new(tx, lister, false, path_style);

        let picker = cx.new(|cx| {
            let picker = Picker::uniform_list(delegate, window, cx)
                .width(rems(34.))
                .modal(false);
            picker.set_query(home_dir.to_string(), window, cx);
            picker
        });

        let data = match &connection {
            RemoteConnectionOptions::Ssh(connection) => ProjectPickerData::Ssh {
                connection_string: connection.connection_string().into(),
                nickname: connection.nickname.clone().map(|nick| nick.into()),
            },
            RemoteConnectionOptions::Wsl(connection) => ProjectPickerData::Wsl {
                distro_name: connection.distro_name.clone().into(),
            },
        };
        let _path_task = cx
            .spawn_in(window, {
                let workspace = workspace;
                async move |this, cx| {
                    let Ok(Some(paths)) = rx.await else {
                        workspace
                            .update_in(cx, |workspace, window, cx| {
                                let fs = workspace.project().read(cx).fs().clone();
                                let weak = cx.entity().downgrade();
                                workspace.toggle_modal(window, cx, |window, cx| {
                                    RemoteServerProjects::new(
                                        create_new_window,
                                        fs,
                                        window,
                                        weak,
                                        cx,
                                    )
                                });
                            })
                            .log_err()?;
                        return None;
                    };

                    let app_state = workspace
                        .read_with(cx, |workspace, _| workspace.app_state().clone())
                        .ok()?;

                    let remote_connection = project
                        .read_with(cx, |project, cx| {
                            project.remote_client()?.read(cx).connection()
                        })
                        .ok()??;

                    let (paths, paths_with_positions) =
                        determine_paths_with_positions(&remote_connection, paths).await;

                    cx.update(|_, cx| {
                        let fs = app_state.fs.clone();
                        update_settings_file(fs, cx, {
                            let paths = paths
                                .iter()
                                .map(|path| path.to_string_lossy().into_owned())
                                .collect();
                            move |settings, _| match index {
                                ServerIndex::Ssh(index) => {
                                    if let Some(server) = settings
                                        .remote
                                        .ssh_connections
                                        .as_mut()
                                        .and_then(|connections| connections.get_mut(index.0))
                                    {
                                        server.projects.insert(SshProject { paths });
                                    };
                                }
                                ServerIndex::Wsl(index) => {
                                    if let Some(server) = settings
                                        .remote
                                        .wsl_connections
                                        .as_mut()
                                        .and_then(|connections| connections.get_mut(index.0))
                                    {
                                        server.projects.insert(SshProject { paths });
                                    };
                                }
                            }
                        });
                    })
                    .log_err();

                    let options = cx
                        .update(|_, cx| (app_state.build_window_options)(None, cx))
                        .log_err()?;
                    let window = cx
                        .open_window(options, |window, cx| {
                            cx.new(|cx| {
                                telemetry::event!("SSH Project Created");
                                Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                            })
                        })
                        .log_err()?;

                    let items = open_remote_project_with_existing_connection(
                        connection, project, paths, app_state, window, cx,
                    )
                    .await
                    .log_err();

                    if let Some(items) = items {
                        for (item, path) in items.into_iter().zip(paths_with_positions) {
                            let Some(item) = item else {
                                continue;
                            };
                            let Some(row) = path.row else {
                                continue;
                            };
                            if let Some(active_editor) = item.downcast::<Editor>() {
                                window
                                    .update(cx, |_, window, cx| {
                                        active_editor.update(cx, |editor, cx| {
                                            let row = row.saturating_sub(1);
                                            let col = path.column.unwrap_or(0).saturating_sub(1);
                                            editor.go_to_singleton_buffer_point(
                                                Point::new(row, col),
                                                window,
                                                cx,
                                            );
                                        });
                                    })
                                    .ok();
                            }
                        }
                    }

                    this.update(cx, |_, cx| {
                        cx.emit(DismissEvent);
                    })
                    .ok();
                    Some(())
                }
            })
            .shared();
        cx.new(|_| Self {
            _path_task,
            picker,
            data,
        })
    }
}

impl gpui::Render for ProjectPicker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(match &self.data {
                ProjectPickerData::Ssh {
                    connection_string,
                    nickname,
                } => SshConnectionHeader {
                    connection_string: connection_string.clone(),
                    paths: Default::default(),
                    nickname: nickname.clone(),
                    is_wsl: false,
                }
                .render(window, cx),
                ProjectPickerData::Wsl { distro_name } => SshConnectionHeader {
                    connection_string: distro_name.clone(),
                    paths: Default::default(),
                    nickname: None,
                    is_wsl: true,
                }
                .render(window, cx),
            })
            .child(
                div()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.picker.clone()),
            )
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SshServerIndex(usize);
impl std::fmt::Display for SshServerIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct WslServerIndex(usize);
impl std::fmt::Display for WslServerIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ServerIndex {
    Ssh(SshServerIndex),
    Wsl(WslServerIndex),
}
impl From<SshServerIndex> for ServerIndex {
    fn from(index: SshServerIndex) -> Self {
        Self::Ssh(index)
    }
}
impl From<WslServerIndex> for ServerIndex {
    fn from(index: WslServerIndex) -> Self {
        Self::Wsl(index)
    }
}

#[derive(Clone)]
enum RemoteEntry {
    Project {
        open_folder: NavigableEntry,
        projects: Vec<(NavigableEntry, SshProject)>,
        configure: NavigableEntry,
        connection: Connection,
        index: ServerIndex,
    },
    SshConfig {
        open_folder: NavigableEntry,
        host: SharedString,
    },
}

impl RemoteEntry {
    fn is_from_zed(&self) -> bool {
        matches!(self, Self::Project { .. })
    }

    fn connection(&self) -> Cow<'_, Connection> {
        match self {
            Self::Project { connection, .. } => Cow::Borrowed(connection),
            Self::SshConfig { host, .. } => Cow::Owned(
                SshConnection {
                    host: host.clone(),
                    ..SshConnection::default()
                }
                .into(),
            ),
        }
    }
}

#[derive(Clone)]
struct DefaultState {
    scroll_handle: ScrollHandle,
    add_new_server: NavigableEntry,
    add_new_wsl: NavigableEntry,
    servers: Vec<RemoteEntry>,
}

impl DefaultState {
    fn new(ssh_config_servers: &BTreeSet<SharedString>, cx: &mut App) -> Self {
        let handle = ScrollHandle::new();
        let add_new_server = NavigableEntry::new(&handle, cx);
        let add_new_wsl = NavigableEntry::new(&handle, cx);

        let ssh_settings = SshSettings::get_global(cx);
        let read_ssh_config = ssh_settings.read_ssh_config;

        let ssh_servers = ssh_settings
            .ssh_connections()
            .enumerate()
            .map(|(index, connection)| {
                let open_folder = NavigableEntry::new(&handle, cx);
                let configure = NavigableEntry::new(&handle, cx);
                let projects = connection
                    .projects
                    .iter()
                    .map(|project| (NavigableEntry::new(&handle, cx), project.clone()))
                    .collect();
                RemoteEntry::Project {
                    open_folder,
                    configure,
                    projects,
                    index: ServerIndex::Ssh(SshServerIndex(index)),
                    connection: connection.into(),
                }
            });

        let wsl_servers = ssh_settings
            .wsl_connections()
            .enumerate()
            .map(|(index, connection)| {
                let open_folder = NavigableEntry::new(&handle, cx);
                let configure = NavigableEntry::new(&handle, cx);
                let projects = connection
                    .projects
                    .iter()
                    .map(|project| (NavigableEntry::new(&handle, cx), project.clone()))
                    .collect();
                RemoteEntry::Project {
                    open_folder,
                    configure,
                    projects,
                    index: ServerIndex::Wsl(WslServerIndex(index)),
                    connection: connection.into(),
                }
            });

        let mut servers = ssh_servers.chain(wsl_servers).collect::<Vec<RemoteEntry>>();

        if read_ssh_config {
            let mut extra_servers_from_config = ssh_config_servers.clone();
            for server in &servers {
                if let RemoteEntry::Project {
                    connection: Connection::Ssh(ssh_options),
                    ..
                } = server
                {
                    extra_servers_from_config.remove(&SharedString::new(ssh_options.host.clone()));
                }
            }
            servers.extend(extra_servers_from_config.into_iter().map(|host| {
                RemoteEntry::SshConfig {
                    open_folder: NavigableEntry::new(&handle, cx),
                    host,
                }
            }));
        }

        Self {
            scroll_handle: handle,
            add_new_server,
            add_new_wsl,
            servers,
        }
    }
}

#[derive(Clone)]
enum ViewServerOptionsState {
    Ssh {
        connection: SshConnectionOptions,
        server_index: SshServerIndex,
        entries: [NavigableEntry; 4],
    },
    Wsl {
        connection: WslConnectionOptions,
        server_index: WslServerIndex,
        entries: [NavigableEntry; 2],
    },
}

impl ViewServerOptionsState {
    fn entries(&self) -> &[NavigableEntry] {
        match self {
            Self::Ssh { entries, .. } => entries,
            Self::Wsl { entries, .. } => entries,
        }
    }
}

enum Mode {
    Default(DefaultState),
    ViewServerOptions(ViewServerOptionsState),
    EditNickname(EditNicknameState),
    ProjectPicker(Entity<ProjectPicker>),
    CreateRemoteServer(CreateRemoteServer),
    #[cfg(target_os = "windows")]
    AddWslDistro(AddWslDistro),
}

impl Mode {
    fn default_mode(ssh_config_servers: &BTreeSet<SharedString>, cx: &mut App) -> Self {
        Self::Default(DefaultState::new(ssh_config_servers, cx))
    }
}

impl RemoteServerProjects {
    #[cfg(target_os = "windows")]
    pub fn wsl(
        create_new_window: bool,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_inner(
            Mode::AddWslDistro(AddWslDistro::new(window, cx)),
            create_new_window,
            fs,
            window,
            workspace,
            cx,
        )
    }

    pub fn new(
        create_new_window: bool,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_inner(
            Mode::default_mode(&BTreeSet::new(), cx),
            create_new_window,
            fs,
            window,
            workspace,
            cx,
        )
    }

    fn new_inner(
        mode: Mode,
        create_new_window: bool,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let mut read_ssh_config = SshSettings::get_global(cx).read_ssh_config;
        let ssh_config_updates = if read_ssh_config {
            spawn_ssh_config_watch(fs.clone(), cx)
        } else {
            Task::ready(())
        };

        let mut base_style = window.text_style();
        base_style.refine(&gpui::TextStyleRefinement {
            color: Some(cx.theme().colors().editor_foreground),
            ..Default::default()
        });

        let _subscription =
            cx.observe_global_in::<SettingsStore>(window, move |recent_projects, _, cx| {
                let new_read_ssh_config = SshSettings::get_global(cx).read_ssh_config;
                if read_ssh_config != new_read_ssh_config {
                    read_ssh_config = new_read_ssh_config;
                    if read_ssh_config {
                        recent_projects.ssh_config_updates = spawn_ssh_config_watch(fs.clone(), cx);
                    } else {
                        recent_projects.ssh_config_servers.clear();
                        recent_projects.ssh_config_updates = Task::ready(());
                    }
                }
            });

        Self {
            mode,
            focus_handle,
            workspace,
            retained_connections: Vec::new(),
            ssh_config_updates,
            ssh_config_servers: BTreeSet::new(),
            create_new_window,
            _subscription,
        }
    }

    fn project_picker(
        create_new_window: bool,
        index: ServerIndex,
        connection_options: remote::RemoteConnectionOptions,
        project: Entity<Project>,
        home_dir: RemotePathBuf,
        path_style: PathStyle,
        window: &mut Window,
        cx: &mut Context<Self>,
        workspace: WeakEntity<Workspace>,
    ) -> Self {
        let fs = project.read(cx).fs().clone();
        let mut this = Self::new(create_new_window, fs, window, workspace.clone(), cx);
        this.mode = Mode::ProjectPicker(ProjectPicker::new(
            create_new_window,
            index,
            connection_options,
            project,
            home_dir,
            path_style,
            workspace,
            window,
            cx,
        ));
        cx.notify();

        this
    }

    fn create_ssh_server(
        &mut self,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let input = get_text(&editor, cx);
        if input.is_empty() {
            return;
        }

        let connection_options = match SshConnectionOptions::parse_command_line(&input) {
            Ok(c) => c,
            Err(e) => {
                self.mode = Mode::CreateRemoteServer(CreateRemoteServer {
                    address_editor: editor,
                    address_error: Some(format!("could not parse: {:?}", e).into()),
                    ssh_prompt: None,
                    _creating: None,
                });
                return;
            }
        };
        let ssh_prompt = cx.new(|cx| {
            RemoteConnectionPrompt::new(
                connection_options.connection_string(),
                connection_options.nickname.clone(),
                false,
                window,
                cx,
            )
        });

        let connection = connect(
            ConnectionIdentifier::setup(),
            RemoteConnectionOptions::Ssh(connection_options.clone()),
            ssh_prompt.clone(),
            window,
            cx,
        )
        .prompt_err("Failed to connect", window, cx, |_, _, _| None);

        let address_editor = editor.clone();
        let creating = cx.spawn_in(window, async move |this, cx| {
            match connection.await {
                Some(Some(client)) => this
                    .update_in(cx, |this, window, cx| {
                        info!("ssh server created");
                        telemetry::event!("SSH Server Created");
                        this.retained_connections.push(client);
                        this.add_ssh_server(connection_options, cx);
                        this.mode = Mode::default_mode(&this.ssh_config_servers, cx);
                        this.focus_handle(cx).focus(window);
                        cx.notify()
                    })
                    .log_err(),
                _ => this
                    .update(cx, |this, cx| {
                        address_editor.update(cx, |this, _| {
                            this.set_read_only(false);
                        });
                        this.mode = Mode::CreateRemoteServer(CreateRemoteServer {
                            address_editor,
                            address_error: None,
                            ssh_prompt: None,
                            _creating: None,
                        });
                        cx.notify()
                    })
                    .log_err(),
            };
            None
        });

        editor.update(cx, |this, _| {
            this.set_read_only(true);
        });
        self.mode = Mode::CreateRemoteServer(CreateRemoteServer {
            address_editor: editor,
            address_error: None,
            ssh_prompt: Some(ssh_prompt),
            _creating: Some(creating),
        });
    }

    #[cfg(target_os = "windows")]
    fn connect_wsl_distro(
        &mut self,
        picker: Entity<Picker<crate::wsl_picker::WslPickerDelegate>>,
        distro: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let connection_options = WslConnectionOptions {
            distro_name: distro,
            user: None,
        };

        let prompt = cx.new(|cx| {
            RemoteConnectionPrompt::new(
                connection_options.distro_name.clone(),
                None,
                true,
                window,
                cx,
            )
        });
        let connection = connect(
            ConnectionIdentifier::setup(),
            connection_options.clone().into(),
            prompt.clone(),
            window,
            cx,
        )
        .prompt_err("Failed to connect", window, cx, |_, _, _| None);

        let wsl_picker = picker.clone();
        let creating = cx.spawn_in(window, async move |this, cx| {
            match connection.await {
                Some(Some(client)) => this.update_in(cx, |this, window, cx| {
                    telemetry::event!("WSL Distro Added");
                    this.retained_connections.push(client);
                    let Some(fs) = this
                        .workspace
                        .read_with(cx, |workspace, cx| {
                            workspace.project().read(cx).fs().clone()
                        })
                        .log_err()
                    else {
                        return;
                    };

                    crate::add_wsl_distro(fs, &connection_options, cx);
                    this.mode = Mode::default_mode(&BTreeSet::new(), cx);
                    this.focus_handle(cx).focus(window);
                    cx.notify();
                }),
                _ => this.update(cx, |this, cx| {
                    this.mode = Mode::AddWslDistro(AddWslDistro {
                        picker: wsl_picker,
                        connection_prompt: None,
                        _creating: None,
                    });
                    cx.notify();
                }),
            }
            .log_err();
        });

        self.mode = Mode::AddWslDistro(AddWslDistro {
            picker,
            connection_prompt: Some(prompt),
            _creating: Some(creating),
        });
    }

    fn view_server_options(
        &mut self,
        (server_index, connection): (ServerIndex, RemoteConnectionOptions),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = Mode::ViewServerOptions(match (server_index, connection) {
            (ServerIndex::Ssh(server_index), RemoteConnectionOptions::Ssh(connection)) => {
                ViewServerOptionsState::Ssh {
                    connection,
                    server_index,
                    entries: std::array::from_fn(|_| NavigableEntry::focusable(cx)),
                }
            }
            (ServerIndex::Wsl(server_index), RemoteConnectionOptions::Wsl(connection)) => {
                ViewServerOptionsState::Wsl {
                    connection,
                    server_index,
                    entries: std::array::from_fn(|_| NavigableEntry::focusable(cx)),
                }
            }
            _ => {
                log::error!("server index and connection options mismatch");
                self.mode = Mode::default_mode(&BTreeSet::default(), cx);
                return;
            }
        });
        self.focus_handle(cx).focus(window);
        cx.notify();
    }

    fn create_remote_project(
        &mut self,
        index: ServerIndex,
        connection_options: RemoteConnectionOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let create_new_window = self.create_new_window;
        workspace.update(cx, |_, cx| {
            cx.defer_in(window, move |workspace, window, cx| {
                let app_state = workspace.app_state().clone();
                workspace.toggle_modal(window, cx, |window, cx| {
                    RemoteConnectionModal::new(&connection_options, Vec::new(), window, cx)
                });
                let prompt = workspace
                    .active_modal::<RemoteConnectionModal>(cx)
                    .unwrap()
                    .read(cx)
                    .prompt
                    .clone();

                let connect = connect(
                    ConnectionIdentifier::setup(),
                    connection_options.clone(),
                    prompt,
                    window,
                    cx,
                )
                .prompt_err("Failed to connect", window, cx, |_, _, _| None);

                cx.spawn_in(window, async move |workspace, cx| {
                    let session = connect.await;

                    workspace.update(cx, |workspace, cx| {
                        if let Some(prompt) = workspace.active_modal::<RemoteConnectionModal>(cx) {
                            prompt.update(cx, |prompt, cx| prompt.finished(cx))
                        }
                    })?;

                    let Some(Some(session)) = session else {
                        return workspace.update_in(cx, |workspace, window, cx| {
                            let weak = cx.entity().downgrade();
                            let fs = workspace.project().read(cx).fs().clone();
                            workspace.toggle_modal(window, cx, |window, cx| {
                                RemoteServerProjects::new(create_new_window, fs, window, weak, cx)
                            });
                        });
                    };

                    let (path_style, project) = cx.update(|_, cx| {
                        (
                            session.read(cx).path_style(),
                            project::Project::remote(
                                session,
                                app_state.client.clone(),
                                app_state.node_runtime.clone(),
                                app_state.user_store.clone(),
                                app_state.languages.clone(),
                                app_state.fs.clone(),
                                cx,
                            ),
                        )
                    })?;

                    let home_dir = project
                        .read_with(cx, |project, cx| project.resolve_abs_path("~", cx))?
                        .await
                        .and_then(|path| path.into_abs_path())
                        .map(|path| RemotePathBuf::new(path, path_style))
                        .unwrap_or_else(|| match path_style {
                            PathStyle::Posix => RemotePathBuf::from_str("/", PathStyle::Posix),
                            PathStyle::Windows => {
                                RemotePathBuf::from_str("C:\\", PathStyle::Windows)
                            }
                        });

                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            let weak = cx.entity().downgrade();
                            workspace.toggle_modal(window, cx, |window, cx| {
                                RemoteServerProjects::project_picker(
                                    create_new_window,
                                    index,
                                    connection_options,
                                    project,
                                    home_dir,
                                    path_style,
                                    window,
                                    cx,
                                    weak,
                                )
                            });
                        })
                        .ok();
                    Ok(())
                })
                .detach();
            })
        })
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        match &self.mode {
            Mode::Default(_) | Mode::ViewServerOptions(_) => {}
            Mode::ProjectPicker(_) => {}
            Mode::CreateRemoteServer(state) => {
                if let Some(prompt) = state.ssh_prompt.as_ref() {
                    prompt.update(cx, |prompt, cx| {
                        prompt.confirm(window, cx);
                    });
                    return;
                }

                self.create_ssh_server(state.address_editor.clone(), window, cx);
            }
            Mode::EditNickname(state) => {
                let text = Some(state.editor.read(cx).text(cx)).filter(|text| !text.is_empty());
                let index = state.index;
                self.update_settings_file(cx, move |setting, _| {
                    if let Some(connections) = setting.ssh_connections.as_mut()
                        && let Some(connection) = connections.get_mut(index.0)
                    {
                        connection.nickname = text;
                    }
                });
                self.mode = Mode::default_mode(&self.ssh_config_servers, cx);
                self.focus_handle.focus(window);
            }
            #[cfg(target_os = "windows")]
            Mode::AddWslDistro(state) => {
                let delegate = &state.picker.read(cx).delegate;
                let distro = delegate.selected_distro().unwrap();
                self.connect_wsl_distro(state.picker.clone(), distro, window, cx);
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        match &self.mode {
            Mode::Default(_) => cx.emit(DismissEvent),
            Mode::CreateRemoteServer(state) if state.ssh_prompt.is_some() => {
                let new_state = CreateRemoteServer::new(window, cx);
                let old_prompt = state.address_editor.read(cx).text(cx);
                new_state.address_editor.update(cx, |this, cx| {
                    this.set_text(old_prompt, window, cx);
                });

                self.mode = Mode::CreateRemoteServer(new_state);
                cx.notify();
            }
            _ => {
                self.mode = Mode::default_mode(&self.ssh_config_servers, cx);
                self.focus_handle(cx).focus(window);
                cx.notify();
            }
        }
    }

    fn render_ssh_connection(
        &mut self,
        ix: usize,
        ssh_server: RemoteEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let connection = ssh_server.connection().into_owned();

        let (main_label, aux_label, is_wsl) = match &connection {
            Connection::Ssh(connection) => {
                if let Some(nickname) = connection.nickname.clone() {
                    let aux_label = SharedString::from(format!("({})", connection.host));
                    (nickname.into(), Some(aux_label), false)
                } else {
                    (connection.host.clone(), None, false)
                }
            }
            Connection::Wsl(wsl_connection_options) => {
                (wsl_connection_options.distro_name.clone(), None, true)
            }
        };
        v_flex()
            .w_full()
            .child(ListSeparator)
            .child(
                h_flex()
                    .group("ssh-server")
                    .w_full()
                    .pt_0p5()
                    .px_3()
                    .gap_1()
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .gap_1()
                            .max_w_96()
                            .overflow_hidden()
                            .text_ellipsis()
                            .when(is_wsl, |this| {
                                this.child(
                                    Label::new("WSL:")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                            })
                            .child(
                                Label::new(main_label)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .children(
                        aux_label.map(|label| {
                            Label::new(label).size(LabelSize::Small).color(Color::Muted)
                        }),
                    ),
            )
            .child(match &ssh_server {
                RemoteEntry::Project {
                    open_folder,
                    projects,
                    configure,
                    connection,
                    index,
                } => {
                    let index = *index;
                    List::new()
                        .empty_message("No projects.")
                        .children(projects.iter().enumerate().map(|(pix, p)| {
                            v_flex().gap_0p5().child(self.render_ssh_project(
                                index,
                                ssh_server.clone(),
                                pix,
                                p,
                                window,
                                cx,
                            ))
                        }))
                        .child(
                            h_flex()
                                .id(("new-remote-project-container", ix))
                                .track_focus(&open_folder.focus_handle)
                                .anchor_scroll(open_folder.scroll_anchor.clone())
                                .on_action(cx.listener({
                                    let connection = connection.clone();
                                    move |this, _: &menu::Confirm, window, cx| {
                                        this.create_remote_project(
                                            index,
                                            connection.clone().into(),
                                            window,
                                            cx,
                                        );
                                    }
                                }))
                                .child(
                                    ListItem::new(("new-remote-project", ix))
                                        .toggle_state(
                                            open_folder.focus_handle.contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                                        .child(Label::new("Open Folder"))
                                        .on_click(cx.listener({
                                            let connection = connection.clone();
                                            move |this, _, window, cx| {
                                                this.create_remote_project(
                                                    index,
                                                    connection.clone().into(),
                                                    window,
                                                    cx,
                                                );
                                            }
                                        })),
                                ),
                        )
                        .child(
                            h_flex()
                                .id(("server-options-container", ix))
                                .track_focus(&configure.focus_handle)
                                .anchor_scroll(configure.scroll_anchor.clone())
                                .on_action(cx.listener({
                                    let connection = connection.clone();
                                    move |this, _: &menu::Confirm, window, cx| {
                                        this.view_server_options(
                                            (index, connection.clone().into()),
                                            window,
                                            cx,
                                        );
                                    }
                                }))
                                .child(
                                    ListItem::new(("server-options", ix))
                                        .toggle_state(
                                            configure.focus_handle.contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::Settings).color(Color::Muted),
                                        )
                                        .child(Label::new("View Server Options"))
                                        .on_click(cx.listener({
                                            let ssh_connection = connection.clone();
                                            move |this, _, window, cx| {
                                                this.view_server_options(
                                                    (index, ssh_connection.clone().into()),
                                                    window,
                                                    cx,
                                                );
                                            }
                                        })),
                                ),
                        )
                }
                RemoteEntry::SshConfig { open_folder, host } => List::new().child(
                    h_flex()
                        .id(("new-remote-project-container", ix))
                        .track_focus(&open_folder.focus_handle)
                        .anchor_scroll(open_folder.scroll_anchor.clone())
                        .on_action(cx.listener({
                            let connection = connection.clone();
                            let host = host.clone();
                            move |this, _: &menu::Confirm, window, cx| {
                                let new_ix = this.create_host_from_ssh_config(&host, cx);
                                this.create_remote_project(
                                    new_ix.into(),
                                    connection.clone().into(),
                                    window,
                                    cx,
                                );
                            }
                        }))
                        .child(
                            ListItem::new(("new-remote-project", ix))
                                .toggle_state(open_folder.focus_handle.contains_focused(window, cx))
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                                .child(Label::new("Open Folder"))
                                .on_click(cx.listener({
                                    let host = host.clone();
                                    move |this, _, window, cx| {
                                        let new_ix = this.create_host_from_ssh_config(&host, cx);
                                        this.create_remote_project(
                                            new_ix.into(),
                                            connection.clone().into(),
                                            window,
                                            cx,
                                        );
                                    }
                                })),
                        ),
                ),
            })
    }

    fn render_ssh_project(
        &mut self,
        server_ix: ServerIndex,
        server: RemoteEntry,
        ix: usize,
        (navigation, project): &(NavigableEntry, SshProject),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let create_new_window = self.create_new_window;
        let is_from_zed = server.is_from_zed();
        let element_id_base = SharedString::from(format!(
            "remote-project-{}",
            match server_ix {
                ServerIndex::Ssh(index) => format!("ssh-{index}"),
                ServerIndex::Wsl(index) => format!("wsl-{index}"),
            }
        ));
        let container_element_id_base =
            SharedString::from(format!("remote-project-container-{element_id_base}"));

        let callback = Rc::new({
            let project = project.clone();
            move |remote_server_projects: &mut Self,
                  secondary_confirm: bool,
                  window: &mut Window,
                  cx: &mut Context<Self>| {
                let Some(app_state) = remote_server_projects
                    .workspace
                    .read_with(cx, |workspace, _| workspace.app_state().clone())
                    .log_err()
                else {
                    return;
                };
                let project = project.clone();
                let server = server.connection().into_owned();
                cx.emit(DismissEvent);

                let replace_window = match (create_new_window, secondary_confirm) {
                    (true, false) | (false, true) => None,
                    (true, true) | (false, false) => window.window_handle().downcast::<Workspace>(),
                };

                cx.spawn_in(window, async move |_, cx| {
                    let result = open_remote_project(
                        server.into(),
                        project.paths.into_iter().map(PathBuf::from).collect(),
                        app_state,
                        OpenOptions {
                            replace_window,
                            ..OpenOptions::default()
                        },
                        cx,
                    )
                    .await;
                    if let Err(e) = result {
                        log::error!("Failed to connect: {e:#}");
                        cx.prompt(
                            gpui::PromptLevel::Critical,
                            "Failed to connect",
                            Some(&e.to_string()),
                            &["Ok"],
                        )
                        .await
                        .ok();
                    }
                })
                .detach();
            }
        });

        div()
            .id((container_element_id_base, ix))
            .track_focus(&navigation.focus_handle)
            .anchor_scroll(navigation.scroll_anchor.clone())
            .on_action(cx.listener({
                let callback = callback.clone();
                move |this, _: &menu::Confirm, window, cx| {
                    callback(this, false, window, cx);
                }
            }))
            .on_action(cx.listener({
                let callback = callback.clone();
                move |this, _: &menu::SecondaryConfirm, window, cx| {
                    callback(this, true, window, cx);
                }
            }))
            .child(
                ListItem::new((element_id_base, ix))
                    .toggle_state(navigation.focus_handle.contains_focused(window, cx))
                    .inset(true)
                    .spacing(ui::ListItemSpacing::Sparse)
                    .start_slot(
                        Icon::new(IconName::Folder)
                            .color(Color::Muted)
                            .size(IconSize::Small),
                    )
                    .child(Label::new(project.paths.join(", ")))
                    .on_click(cx.listener(move |this, e: &ClickEvent, window, cx| {
                        let secondary_confirm = e.modifiers().platform;
                        callback(this, secondary_confirm, window, cx)
                    }))
                    .when(is_from_zed, |server_list_item| {
                        server_list_item.end_hover_slot::<AnyElement>(Some(
                            div()
                                .mr_2()
                                .child({
                                    let project = project.clone();
                                    // Right-margin to offset it from the Scrollbar
                                    IconButton::new("remove-remote-project", IconName::Trash)
                                        .icon_size(IconSize::Small)
                                        .shape(IconButtonShape::Square)
                                        .size(ButtonSize::Large)
                                        .tooltip(Tooltip::text("Delete Remote Project"))
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.delete_remote_project(server_ix, &project, cx)
                                        }))
                                })
                                .into_any_element(),
                        ))
                    }),
            )
    }

    fn update_settings_file(
        &mut self,
        cx: &mut Context<Self>,
        f: impl FnOnce(&mut RemoteSettingsContent, &App) + Send + Sync + 'static,
    ) {
        let Some(fs) = self
            .workspace
            .read_with(cx, |workspace, _| workspace.app_state().fs.clone())
            .log_err()
        else {
            return;
        };
        update_settings_file(fs, cx, move |setting, cx| f(&mut setting.remote, cx));
    }

    fn delete_ssh_server(&mut self, server: SshServerIndex, cx: &mut Context<Self>) {
        self.update_settings_file(cx, move |setting, _| {
            if let Some(connections) = setting.ssh_connections.as_mut() {
                connections.remove(server.0);
            }
        });
    }

    fn delete_remote_project(
        &mut self,
        server: ServerIndex,
        project: &SshProject,
        cx: &mut Context<Self>,
    ) {
        match server {
            ServerIndex::Ssh(server) => {
                self.delete_ssh_project(server, project, cx);
            }
            ServerIndex::Wsl(server) => {
                self.delete_wsl_project(server, project, cx);
            }
        }
    }

    fn delete_ssh_project(
        &mut self,
        server: SshServerIndex,
        project: &SshProject,
        cx: &mut Context<Self>,
    ) {
        let project = project.clone();
        self.update_settings_file(cx, move |setting, _| {
            if let Some(server) = setting
                .ssh_connections
                .as_mut()
                .and_then(|connections| connections.get_mut(server.0))
            {
                server.projects.remove(&project);
            }
        });
    }

    fn delete_wsl_project(
        &mut self,
        server: WslServerIndex,
        project: &SshProject,
        cx: &mut Context<Self>,
    ) {
        let project = project.clone();
        self.update_settings_file(cx, move |setting, _| {
            if let Some(server) = setting
                .wsl_connections
                .as_mut()
                .and_then(|connections| connections.get_mut(server.0))
            {
                server.projects.remove(&project);
            }
        });
    }

    fn delete_wsl_distro(&mut self, server: WslServerIndex, cx: &mut Context<Self>) {
        self.update_settings_file(cx, move |setting, _| {
            if let Some(connections) = setting.wsl_connections.as_mut() {
                connections.remove(server.0);
            }
        });
    }

    fn add_ssh_server(
        &mut self,
        connection_options: remote::SshConnectionOptions,
        cx: &mut Context<Self>,
    ) {
        self.update_settings_file(cx, move |setting, _| {
            setting
                .ssh_connections
                .get_or_insert(Default::default())
                .push(SshConnection {
                    host: SharedString::from(connection_options.host),
                    username: connection_options.username,
                    port: connection_options.port,
                    projects: BTreeSet::new(),
                    nickname: None,
                    args: connection_options.args.unwrap_or_default(),
                    upload_binary_over_ssh: None,
                    port_forwards: connection_options.port_forwards,
                })
        });
    }

    fn render_create_remote_server(
        &self,
        state: &CreateRemoteServer,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let ssh_prompt = state.ssh_prompt.clone();

        state.address_editor.update(cx, |editor, cx| {
            if editor.text(cx).is_empty() {
                editor.set_placeholder_text("ssh user@example -p 2222", window, cx);
            }
        });

        let theme = cx.theme();

        v_flex()
            .track_focus(&self.focus_handle(cx))
            .id("create-remote-server")
            .overflow_hidden()
            .size_full()
            .flex_1()
            .child(
                div()
                    .p_2()
                    .border_b_1()
                    .border_color(theme.colors().border_variant)
                    .child(state.address_editor.clone()),
            )
            .child(
                h_flex()
                    .bg(theme.colors().editor_background)
                    .rounded_b_sm()
                    .w_full()
                    .map(|this| {
                        if let Some(ssh_prompt) = ssh_prompt {
                            this.child(h_flex().w_full().child(ssh_prompt))
                        } else if let Some(address_error) = &state.address_error {
                            this.child(
                                h_flex().p_2().w_full().gap_2().child(
                                    Label::new(address_error.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Error),
                                ),
                            )
                        } else {
                            this.child(
                                h_flex()
                                    .p_2()
                                    .w_full()
                                    .gap_1()
                                    .child(
                                        Label::new(
                                            "Enter the command you use to SSH into this server.",
                                        )
                                        .color(Color::Muted)
                                        .size(LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("learn-more", "Learn More")
                                            .label_size(LabelSize::Small)
                                            .icon(IconName::ArrowUpRight)
                                            .icon_size(IconSize::XSmall)
                                            .on_click(|_, _, cx| {
                                                cx.open_url(
                                                    "https://zed.dev/docs/remote-development",
                                                );
                                            }),
                                    ),
                            )
                        }
                    }),
            )
    }

    #[cfg(target_os = "windows")]
    fn render_add_wsl_distro(
        &self,
        state: &AddWslDistro,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let connection_prompt = state.connection_prompt.clone();

        state.picker.update(cx, |picker, cx| {
            picker.focus_handle(cx).focus(window);
        });

        v_flex()
            .id("add-wsl-distro")
            .overflow_hidden()
            .size_full()
            .flex_1()
            .map(|this| {
                if let Some(connection_prompt) = connection_prompt {
                    this.child(connection_prompt)
                } else {
                    this.child(state.picker.clone())
                }
            })
    }

    fn render_view_options(
        &mut self,
        options: ViewServerOptionsState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let last_entry = options.entries().last().unwrap();

        let mut view = Navigable::new(
            div()
                .track_focus(&self.focus_handle(cx))
                .size_full()
                .child(match &options {
                    ViewServerOptionsState::Ssh { connection, .. } => SshConnectionHeader {
                        connection_string: connection.host.clone().into(),
                        paths: Default::default(),
                        nickname: connection.nickname.clone().map(|s| s.into()),
                        is_wsl: false,
                    }
                    .render(window, cx)
                    .into_any_element(),
                    ViewServerOptionsState::Wsl { connection, .. } => SshConnectionHeader {
                        connection_string: connection.distro_name.clone().into(),
                        paths: Default::default(),
                        nickname: None,
                        is_wsl: true,
                    }
                    .render(window, cx)
                    .into_any_element(),
                })
                .child(
                    v_flex()
                        .pb_1()
                        .child(ListSeparator)
                        .map(|this| match &options {
                            ViewServerOptionsState::Ssh {
                                connection,
                                entries,
                                server_index,
                            } => this.child(self.render_edit_ssh(
                                connection,
                                *server_index,
                                entries,
                                window,
                                cx,
                            )),
                            ViewServerOptionsState::Wsl {
                                connection,
                                entries,
                                server_index,
                            } => this.child(self.render_edit_wsl(
                                connection,
                                *server_index,
                                entries,
                                window,
                                cx,
                            )),
                        })
                        .child(ListSeparator)
                        .child({
                            div()
                                .id("ssh-options-copy-server-address")
                                .track_focus(&last_entry.focus_handle)
                                .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                                    this.mode = Mode::default_mode(&this.ssh_config_servers, cx);
                                    cx.focus_self(window);
                                    cx.notify();
                                }))
                                .child(
                                    ListItem::new("go-back")
                                        .toggle_state(
                                            last_entry.focus_handle.contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::ArrowLeft).color(Color::Muted),
                                        )
                                        .child(Label::new("Go Back"))
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.mode =
                                                Mode::default_mode(&this.ssh_config_servers, cx);
                                            cx.focus_self(window);
                                            cx.notify()
                                        })),
                                )
                        }),
                )
                .into_any_element(),
        );

        for entry in options.entries() {
            view = view.entry(entry.clone());
        }

        view.render(window, cx).into_any_element()
    }

    fn render_edit_wsl(
        &self,
        connection: &WslConnectionOptions,
        index: WslServerIndex,
        entries: &[NavigableEntry],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let distro_name = SharedString::new(connection.distro_name.clone());

        v_flex().child({
            fn remove_wsl_distro(
                remote_servers: Entity<RemoteServerProjects>,
                index: WslServerIndex,
                distro_name: SharedString,
                window: &mut Window,
                cx: &mut App,
            ) {
                let prompt_message = format!("Remove WSL distro `{}`?", distro_name);

                let confirmation = window.prompt(
                    PromptLevel::Warning,
                    &prompt_message,
                    None,
                    &["Yes, remove it", "No, keep it"],
                    cx,
                );

                cx.spawn(async move |cx| {
                    if confirmation.await.ok() == Some(0) {
                        remote_servers
                            .update(cx, |this, cx| {
                                this.delete_wsl_distro(index, cx);
                            })
                            .ok();
                        remote_servers
                            .update(cx, |this, cx| {
                                this.mode = Mode::default_mode(&this.ssh_config_servers, cx);
                                cx.notify();
                            })
                            .ok();
                    }
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
            div()
                .id("wsl-options-remove-distro")
                .track_focus(&entries[0].focus_handle)
                .on_action(cx.listener({
                    let distro_name = distro_name.clone();
                    move |_, _: &menu::Confirm, window, cx| {
                        remove_wsl_distro(cx.entity(), index, distro_name.clone(), window, cx);
                        cx.focus_self(window);
                    }
                }))
                .child(
                    ListItem::new("remove-distro")
                        .toggle_state(entries[0].focus_handle.contains_focused(window, cx))
                        .inset(true)
                        .spacing(ui::ListItemSpacing::Sparse)
                        .start_slot(Icon::new(IconName::Trash).color(Color::Error))
                        .child(Label::new("Remove Distro").color(Color::Error))
                        .on_click(cx.listener(move |_, _, window, cx| {
                            remove_wsl_distro(cx.entity(), index, distro_name.clone(), window, cx);
                            cx.focus_self(window);
                        })),
                )
        })
    }

    fn render_edit_ssh(
        &self,
        connection: &SshConnectionOptions,
        index: SshServerIndex,
        entries: &[NavigableEntry],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let connection_string = SharedString::new(connection.host.clone());

        v_flex()
            .child({
                let label = if connection.nickname.is_some() {
                    "Edit Nickname"
                } else {
                    "Add Nickname to Server"
                };
                div()
                    .id("ssh-options-add-nickname")
                    .track_focus(&entries[0].focus_handle)
                    .on_action(cx.listener(move |this, _: &menu::Confirm, window, cx| {
                        this.mode = Mode::EditNickname(EditNicknameState::new(index, window, cx));
                        cx.notify();
                    }))
                    .child(
                        ListItem::new("add-nickname")
                            .toggle_state(entries[0].focus_handle.contains_focused(window, cx))
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                            .child(Label::new(label))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.mode =
                                    Mode::EditNickname(EditNicknameState::new(index, window, cx));
                                cx.notify();
                            })),
                    )
            })
            .child({
                let workspace = self.workspace.clone();
                fn callback(
                    workspace: WeakEntity<Workspace>,
                    connection_string: SharedString,
                    cx: &mut App,
                ) {
                    cx.write_to_clipboard(ClipboardItem::new_string(connection_string.to_string()));
                    workspace
                        .update(cx, |this, cx| {
                            struct SshServerAddressCopiedToClipboard;
                            let notification = format!(
                                "Copied server address ({}) to clipboard",
                                connection_string
                            );

                            this.show_toast(
                                Toast::new(
                                    NotificationId::composite::<SshServerAddressCopiedToClipboard>(
                                        connection_string.clone(),
                                    ),
                                    notification,
                                )
                                .autohide(),
                                cx,
                            );
                        })
                        .ok();
                }
                div()
                    .id("ssh-options-copy-server-address")
                    .track_focus(&entries[1].focus_handle)
                    .on_action({
                        let connection_string = connection_string.clone();
                        let workspace = self.workspace.clone();
                        move |_: &menu::Confirm, _, cx| {
                            callback(workspace.clone(), connection_string.clone(), cx);
                        }
                    })
                    .child(
                        ListItem::new("copy-server-address")
                            .toggle_state(entries[1].focus_handle.contains_focused(window, cx))
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Copy).color(Color::Muted))
                            .child(Label::new("Copy Server Address"))
                            .end_hover_slot(
                                Label::new(connection_string.clone()).color(Color::Muted),
                            )
                            .on_click({
                                let connection_string = connection_string.clone();
                                move |_, _, cx| {
                                    callback(workspace.clone(), connection_string.clone(), cx);
                                }
                            }),
                    )
            })
            .child({
                fn remove_ssh_server(
                    remote_servers: Entity<RemoteServerProjects>,
                    index: SshServerIndex,
                    connection_string: SharedString,
                    window: &mut Window,
                    cx: &mut App,
                ) {
                    let prompt_message = format!("Remove server `{}`?", connection_string);

                    let confirmation = window.prompt(
                        PromptLevel::Warning,
                        &prompt_message,
                        None,
                        &["Yes, remove it", "No, keep it"],
                        cx,
                    );

                    cx.spawn(async move |cx| {
                        if confirmation.await.ok() == Some(0) {
                            remote_servers
                                .update(cx, |this, cx| {
                                    this.delete_ssh_server(index, cx);
                                })
                                .ok();
                            remote_servers
                                .update(cx, |this, cx| {
                                    this.mode = Mode::default_mode(&this.ssh_config_servers, cx);
                                    cx.notify();
                                })
                                .ok();
                        }
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                }
                div()
                    .id("ssh-options-copy-server-address")
                    .track_focus(&entries[2].focus_handle)
                    .on_action(cx.listener({
                        let connection_string = connection_string.clone();
                        move |_, _: &menu::Confirm, window, cx| {
                            remove_ssh_server(
                                cx.entity(),
                                index,
                                connection_string.clone(),
                                window,
                                cx,
                            );
                            cx.focus_self(window);
                        }
                    }))
                    .child(
                        ListItem::new("remove-server")
                            .toggle_state(entries[2].focus_handle.contains_focused(window, cx))
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Trash).color(Color::Error))
                            .child(Label::new("Remove Server").color(Color::Error))
                            .on_click(cx.listener(move |_, _, window, cx| {
                                remove_ssh_server(
                                    cx.entity(),
                                    index,
                                    connection_string.clone(),
                                    window,
                                    cx,
                                );
                                cx.focus_self(window);
                            })),
                    )
            })
    }

    fn render_edit_nickname(
        &self,
        state: &EditNicknameState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(connection) = SshSettings::get_global(cx)
            .ssh_connections()
            .nth(state.index.0)
        else {
            return v_flex()
                .id("ssh-edit-nickname")
                .track_focus(&self.focus_handle(cx));
        };

        let connection_string = connection.host.clone();
        let nickname = connection.nickname.map(|s| s.into());

        v_flex()
            .id("ssh-edit-nickname")
            .track_focus(&self.focus_handle(cx))
            .child(
                SshConnectionHeader {
                    connection_string,
                    paths: Default::default(),
                    nickname,
                    is_wsl: false,
                }
                .render(window, cx),
            )
            .child(
                h_flex()
                    .p_2()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(state.editor.clone()),
            )
    }

    fn render_default(
        &mut self,
        mut state: DefaultState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let ssh_settings = SshSettings::get_global(cx);
        let mut should_rebuild = false;

        let ssh_connections_changed = ssh_settings.ssh_connections.0.iter().ne(state
            .servers
            .iter()
            .filter_map(|server| match server {
                RemoteEntry::Project {
                    connection: Connection::Ssh(connection),
                    ..
                } => Some(connection),
                _ => None,
            }));

        let wsl_connections_changed = ssh_settings.wsl_connections.0.iter().ne(state
            .servers
            .iter()
            .filter_map(|server| match server {
                RemoteEntry::Project {
                    connection: Connection::Wsl(connection),
                    ..
                } => Some(connection),
                _ => None,
            }));

        if ssh_connections_changed || wsl_connections_changed {
            should_rebuild = true;
        };

        if !should_rebuild && ssh_settings.read_ssh_config {
            let current_ssh_hosts: BTreeSet<SharedString> = state
                .servers
                .iter()
                .filter_map(|server| match server {
                    RemoteEntry::SshConfig { host, .. } => Some(host.clone()),
                    _ => None,
                })
                .collect();
            let mut expected_ssh_hosts = self.ssh_config_servers.clone();
            for server in &state.servers {
                if let RemoteEntry::Project {
                    connection: Connection::Ssh(connection),
                    ..
                } = server
                {
                    expected_ssh_hosts.remove(&connection.host);
                }
            }
            should_rebuild = current_ssh_hosts != expected_ssh_hosts;
        }

        if should_rebuild {
            self.mode = Mode::default_mode(&self.ssh_config_servers, cx);
            if let Mode::Default(new_state) = &self.mode {
                state = new_state.clone();
            }
        }

        let connect_button = div()
            .id("ssh-connect-new-server-container")
            .track_focus(&state.add_new_server.focus_handle)
            .anchor_scroll(state.add_new_server.scroll_anchor.clone())
            .child(
                ListItem::new("register-remove-server-button")
                    .toggle_state(
                        state
                            .add_new_server
                            .focus_handle
                            .contains_focused(window, cx),
                    )
                    .inset(true)
                    .spacing(ui::ListItemSpacing::Sparse)
                    .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                    .child(Label::new("Connect New Server"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        let state = CreateRemoteServer::new(window, cx);
                        this.mode = Mode::CreateRemoteServer(state);

                        cx.notify();
                    })),
            )
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                let state = CreateRemoteServer::new(window, cx);
                this.mode = Mode::CreateRemoteServer(state);

                cx.notify();
            }));

        #[cfg(target_os = "windows")]
        let wsl_connect_button = div()
            .id("wsl-connect-new-server")
            .track_focus(&state.add_new_wsl.focus_handle)
            .anchor_scroll(state.add_new_wsl.scroll_anchor.clone())
            .child(
                ListItem::new("wsl-add-new-server")
                    .toggle_state(state.add_new_wsl.focus_handle.contains_focused(window, cx))
                    .inset(true)
                    .spacing(ui::ListItemSpacing::Sparse)
                    .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                    .child(Label::new("Add WSL Distro"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        let state = AddWslDistro::new(window, cx);
                        this.mode = Mode::AddWslDistro(state);

                        cx.notify();
                    })),
            )
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                let state = AddWslDistro::new(window, cx);
                this.mode = Mode::AddWslDistro(state);

                cx.notify();
            }));

        let modal_section = v_flex()
            .track_focus(&self.focus_handle(cx))
            .id("ssh-server-list")
            .overflow_y_scroll()
            .track_scroll(&state.scroll_handle)
            .size_full()
            .child(connect_button);

        #[cfg(target_os = "windows")]
        let modal_section = modal_section.child(wsl_connect_button);
        #[cfg(not(target_os = "windows"))]
        let modal_section = modal_section;

        let mut modal_section = Navigable::new(
            modal_section
                .child(
                    List::new()
                        .empty_message(
                            v_flex()
                                .child(
                                    div().px_3().child(
                                        Label::new("No remote servers registered yet.")
                                            .color(Color::Muted),
                                    ),
                                )
                                .into_any_element(),
                        )
                        .children(state.servers.iter().enumerate().map(|(ix, connection)| {
                            self.render_ssh_connection(ix, connection.clone(), window, cx)
                                .into_any_element()
                        })),
                )
                .into_any_element(),
        )
        .entry(state.add_new_server.clone());

        if cfg!(target_os = "windows") {
            modal_section = modal_section.entry(state.add_new_wsl.clone());
        }

        for server in &state.servers {
            match server {
                RemoteEntry::Project {
                    open_folder,
                    projects,
                    configure,
                    ..
                } => {
                    for (navigation_state, _) in projects {
                        modal_section = modal_section.entry(navigation_state.clone());
                    }
                    modal_section = modal_section
                        .entry(open_folder.clone())
                        .entry(configure.clone());
                }
                RemoteEntry::SshConfig { open_folder, .. } => {
                    modal_section = modal_section.entry(open_folder.clone());
                }
            }
        }
        let mut modal_section = modal_section.render(window, cx).into_any_element();

        let (create_window, reuse_window) = if self.create_new_window {
            (
                window.keystroke_text_for(&menu::Confirm),
                window.keystroke_text_for(&menu::SecondaryConfirm),
            )
        } else {
            (
                window.keystroke_text_for(&menu::SecondaryConfirm),
                window.keystroke_text_for(&menu::Confirm),
            )
        };
        let placeholder_text = Arc::from(format!(
            "{reuse_window} reuses this window, {create_window} opens a new one",
        ));

        Modal::new("remote-projects", None)
            .header(
                ModalHeader::new()
                    .child(Headline::new("Remote Projects").size(HeadlineSize::XSmall))
                    .child(
                        Label::new(placeholder_text)
                            .color(Color::Muted)
                            .size(LabelSize::XSmall),
                    ),
            )
            .section(
                Section::new().padded(false).child(
                    v_flex()
                        .min_h(rems(20.))
                        .size_full()
                        .relative()
                        .child(ListSeparator)
                        .child(
                            canvas(
                                |bounds, window, cx| {
                                    modal_section.prepaint_as_root(
                                        bounds.origin,
                                        bounds.size.into(),
                                        window,
                                        cx,
                                    );
                                    modal_section
                                },
                                |_, mut modal_section, window, cx| {
                                    modal_section.paint(window, cx);
                                },
                            )
                            .size_full(),
                        )
                        .vertical_scrollbar_for(&state.scroll_handle, window, cx),
                ),
            )
            .into_any_element()
    }

    fn create_host_from_ssh_config(
        &mut self,
        ssh_config_host: &SharedString,
        cx: &mut Context<'_, Self>,
    ) -> SshServerIndex {
        let new_ix = Arc::new(AtomicUsize::new(0));

        let update_new_ix = new_ix.clone();
        self.update_settings_file(cx, move |settings, _| {
            update_new_ix.store(
                settings
                    .ssh_connections
                    .as_ref()
                    .map_or(0, |connections| connections.len()),
                atomic::Ordering::Release,
            );
        });

        self.add_ssh_server(
            SshConnectionOptions {
                host: ssh_config_host.to_string(),
                ..SshConnectionOptions::default()
            },
            cx,
        );
        self.mode = Mode::default_mode(&self.ssh_config_servers, cx);
        SshServerIndex(new_ix.load(atomic::Ordering::Acquire))
    }
}

fn spawn_ssh_config_watch(fs: Arc<dyn Fs>, cx: &Context<RemoteServerProjects>) -> Task<()> {
    let mut user_ssh_config_watcher =
        watch_config_file(cx.background_executor(), fs.clone(), user_ssh_config_file());
    let mut global_ssh_config_watcher = global_ssh_config_file()
        .map(|it| watch_config_file(cx.background_executor(), fs, it.to_owned()))
        .unwrap_or_else(|| futures::channel::mpsc::unbounded().1);

    cx.spawn(async move |remote_server_projects, cx| {
        let mut global_hosts = BTreeSet::default();
        let mut user_hosts = BTreeSet::default();
        let mut running_receivers = 2;

        loop {
            select! {
                new_global_file_contents = global_ssh_config_watcher.next().fuse() => {
                    match new_global_file_contents {
                        Some(new_global_file_contents) => {
                            global_hosts = parse_ssh_config_hosts(&new_global_file_contents);
                            if remote_server_projects.update(cx, |remote_server_projects, cx| {
                                remote_server_projects.ssh_config_servers = global_hosts.iter().chain(user_hosts.iter()).map(SharedString::from).collect();
                                cx.notify();
                            }).is_err() {
                                return;
                            }
                        },
                        None => {
                            running_receivers -= 1;
                            if running_receivers == 0 {
                                return;
                            }
                        }
                    }
                },
                new_user_file_contents = user_ssh_config_watcher.next().fuse() => {
                    match new_user_file_contents {
                        Some(new_user_file_contents) => {
                            user_hosts = parse_ssh_config_hosts(&new_user_file_contents);
                            if remote_server_projects.update(cx, |remote_server_projects, cx| {
                                remote_server_projects.ssh_config_servers = global_hosts.iter().chain(user_hosts.iter()).map(SharedString::from).collect();
                                cx.notify();
                            }).is_err() {
                                return;
                            }
                        },
                        None => {
                            running_receivers -= 1;
                            if running_receivers == 0 {
                                return;
                            }
                        }
                    }
                },
            }
        }
    })
}

fn get_text(element: &Entity<Editor>, cx: &mut App) -> String {
    element.read(cx).text(cx).trim().to_string()
}

impl ModalView for RemoteServerProjects {}

impl Focusable for RemoteServerProjects {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            Mode::ProjectPicker(picker) => picker.focus_handle(cx),
            _ => self.focus_handle.clone(),
        }
    }
}

impl EventEmitter<DismissEvent> for RemoteServerProjects {}

impl Render for RemoteServerProjects {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("RemoteServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                if matches!(this.mode, Mode::Default(_)) {
                    cx.emit(DismissEvent)
                }
            }))
            .child(match &self.mode {
                Mode::Default(state) => self
                    .render_default(state.clone(), window, cx)
                    .into_any_element(),
                Mode::ViewServerOptions(state) => self
                    .render_view_options(state.clone(), window, cx)
                    .into_any_element(),
                Mode::ProjectPicker(element) => element.clone().into_any_element(),
                Mode::CreateRemoteServer(state) => self
                    .render_create_remote_server(state, window, cx)
                    .into_any_element(),
                Mode::EditNickname(state) => self
                    .render_edit_nickname(state, window, cx)
                    .into_any_element(),
                #[cfg(target_os = "windows")]
                Mode::AddWslDistro(state) => self
                    .render_add_wsl_distro(state, window, cx)
                    .into_any_element(),
            })
    }
}

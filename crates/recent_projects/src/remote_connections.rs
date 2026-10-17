use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use crate::disconnected_overlay::DisconnectedOverlay;
use anyhow::{Context as _, Result};
use askpass::EncryptedPassword;
use editor::Editor;
use futures::{
    FutureExt as _,
    channel::oneshot,
    future::{LocalBoxFuture, Shared, WeakShared, ready, try_join_all},
    select,
};
use gpui::{App, AppContext, AsyncApp, DismissEvent, Entity, PromptLevel, WindowHandle};

use project::trusted_worktrees;
use remote::{
    DockerConnectionOptions, Interactive, RemoteConnection, RemoteConnectionOptions,
    SshConnectionOptions, same_remote_connection_identity,
};
use rpc::proto;
pub use settings::SshConnection;
use settings::{DevContainerConnection, ExtendingVec, RegisterSetting, Settings, WslConnection};
use util::paths::{PathStyle, PathWithPosition, SanitizedPath};
use workspace::{
    AppState, MultiWorkspace, MultiWorkspaceEvent, OpenMode, OpenOptions, OpenVisible,
    SerializedWorkspaceLocation, Workspace, WorkspaceDb, WorkspaceId, WorkspaceIdReservation,
    WorkspaceMatch, WorkspaceMatching, unloaded_workspace_candidates,
};

pub use remote_connection::{
    RemoteClientDelegate, RemoteConnectionModal, RemoteConnectionPrompt, SshConnectionHeader,
    connect,
};

#[derive(RegisterSetting)]
pub struct RemoteSettings {
    pub ssh_connections: ExtendingVec<SshConnection>,
    pub wsl_connections: ExtendingVec<WslConnection>,
    /// Whether to read ~/.ssh/config for ssh connection sources.
    pub read_ssh_config: bool,
}

impl RemoteSettings {
    pub fn ssh_connections(&self) -> impl Iterator<Item = SshConnection> + use<> {
        self.ssh_connections.clone().0.into_iter()
    }

    pub fn wsl_connections(&self) -> impl Iterator<Item = WslConnection> + use<> {
        self.wsl_connections.clone().0.into_iter()
    }

    pub fn fill_connection_options_from_settings(&self, options: &mut SshConnectionOptions) {
        for conn in self.ssh_connections() {
            if conn.host == options.host.to_string()
                && conn.username == options.username
                && conn.port == options.port
            {
                options.nickname = conn.nickname;
                options.upload_binary_over_ssh = conn.upload_binary_over_ssh.unwrap_or_default();
                options.args = Some(conn.args);
                options.port_forwards = conn.port_forwards;
                break;
            }
        }
    }

    pub fn connection_options_for(
        &self,
        host: String,
        port: Option<u16>,
        username: Option<String>,
    ) -> SshConnectionOptions {
        let mut options = SshConnectionOptions {
            host: host.into(),
            port,
            username,
            ..Default::default()
        };
        self.fill_connection_options_from_settings(&mut options);
        options
    }
}

#[derive(Clone, PartialEq)]
pub enum Connection {
    Ssh(SshConnection),
    Wsl(WslConnection),
    DevContainer(DevContainerConnection),
}

impl From<Connection> for RemoteConnectionOptions {
    fn from(val: Connection) -> Self {
        match val {
            Connection::Ssh(conn) => RemoteConnectionOptions::Ssh(conn.into()),
            Connection::Wsl(conn) => RemoteConnectionOptions::Wsl(conn.into()),
            Connection::DevContainer(conn) => {
                RemoteConnectionOptions::Docker(DockerConnectionOptions {
                    name: conn.name,
                    remote_user: conn.remote_user,
                    container_id: conn.container_id,
                    upload_binary_over_docker_exec: false,
                    use_podman: conn.use_podman,
                    remote_env: conn.remote_env,
                })
            }
        }
    }
}

impl From<SshConnection> for Connection {
    fn from(val: SshConnection) -> Self {
        Connection::Ssh(val)
    }
}

impl From<WslConnection> for Connection {
    fn from(val: WslConnection) -> Self {
        Connection::Wsl(val)
    }
}

impl Settings for RemoteSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let remote = &content.remote;
        Self {
            ssh_connections: remote.ssh_connections.clone().unwrap_or_default().into(),
            wsl_connections: remote.wsl_connections.clone().unwrap_or_default().into(),
            read_ssh_config: remote.read_ssh_config.unwrap(),
        }
    }
}

pub async fn open_remote_project(
    connection_options: RemoteConnectionOptions,
    mut paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    mut open_options: workspace::OpenOptions,
    cx: &mut AsyncApp,
) -> Result<WindowHandle<MultiWorkspace>> {
    let focus = open_options.focus.unwrap_or(true);
    if open_options.open_mode == OpenMode::NewWindow {
        open_options.workspace_matching = WorkspaceMatching::None;
        open_options.requesting_window = None;
    }

    let explicit_workspace = open_options.workspace_id.is_some();
    let candidates = cx
        .update(|cx| remote_workspace_candidates(&paths, &open_options, &connection_options, cx))?;
    let loads = cx.update(|cx| {
        candidates
            .into_iter()
            .map(|(owner, candidate)| {
                let workspace_id = match candidate {
                    WorkspaceMatch::Loaded(workspace) => {
                        if remote_workspace_connection(&workspace, cx).is_some() {
                            let load = ready(Ok(OpenedRemoteProject {
                                window: owner.context("Remote workspace has no window")?,
                                workspace: Some(workspace),
                            }))
                            .boxed_local()
                            .shared();
                            return Ok((load, Vec::new()));
                        }
                        workspace
                            .read(cx)
                            .database_id()
                            .context("Remote workspace has no database ID")?
                    }
                    WorkspaceMatch::Unloaded(workspace_id) => workspace_id,
                };
                let mut load_options = open_options.clone();
                load_options.workspace_id = Some(workspace_id);
                load_options.requesting_window = owner;
                if !explicit_workspace {
                    load_options.focus = Some(false);
                }
                load_remote_workspace(
                    workspace_id,
                    connection_options.clone(),
                    app_state.clone(),
                    load_options,
                    cx,
                )
            })
            .collect::<Vec<_>>()
    });
    let mut resolved_paths = None;
    let mut selected = None;
    let mut best_visibility = None;
    let mut candidate_error = None;
    for load in loads {
        let candidate = async {
            let (load, restored_paths) = load?;
            let opened = load.await.map_err(|error| anyhow::anyhow!(error))?;
            let Some(workspace) = &opened.workspace else {
                return Ok((opened, None, OpenVisible::All));
            };
            if explicit_workspace {
                paths.retain(|path| !restored_paths.contains(path));
            }
            if resolved_paths.is_none() {
                resolved_paths = Some(resolve_remote_paths(workspace, &paths, cx).await?);
            }
            if explicit_workspace {
                return Ok((opened, Some(true), OpenVisible::All));
            }
            let scans = cx.update(|cx| {
                let worktrees = workspace
                    .read(cx)
                    .project()
                    .read(cx)
                    .worktrees(cx)
                    .collect::<Vec<_>>();
                worktrees
                    .into_iter()
                    .map(|worktree| {
                        worktree.update(cx, |worktree, _| worktree.wait_for_snapshot(1))
                    })
                    .collect::<Vec<_>>()
            });
            try_join_all(scans).await?;
            let positions = resolved_paths
                .as_ref()
                .context("Remote paths were not resolved")?;
            let (visibility, visible) = opened.window.read_with(cx, |multi_workspace, cx| {
                anyhow::ensure!(
                    multi_workspace
                        .workspaces()
                        .any(|candidate| candidate == workspace),
                    "Workspace was closed while restoring"
                );
                let project = workspace.read(cx).project().read(cx);
                let paths = positions
                    .iter()
                    .map(|position| position.path.clone())
                    .collect::<Vec<_>>();
                let visible = if open_options.wait
                    && !paths.iter().any(|path| {
                        project
                            .find_worktree(SanitizedPath::new(path).as_ref(), cx)
                            .is_some_and(|(worktree, path)| {
                                worktree
                                    .read(cx)
                                    .entry_for_path(&path)
                                    .is_some_and(|entry| entry.is_dir())
                            })
                    }) {
                    OpenVisible::None
                } else {
                    OpenVisible::All
                };
                Ok((
                    open_options
                        .workspace_matching
                        .visibility(project, &paths, cx),
                    visible,
                ))
            })??;
            anyhow::Ok((opened, visibility, visible))
        }
        .await;
        let (opened, visibility, visible) = match candidate {
            Ok(candidate) => candidate,
            Err(error) if explicit_workspace => return Err(error),
            Err(error) => {
                candidate_error.get_or_insert(error);
                continue;
            }
        };
        let Some(workspace) = opened.workspace else {
            return Ok(opened.window);
        };
        if visibility > best_visibility
            || selected.is_none()
                && open_options.workspace_matching == WorkspaceMatching::MatchSubdirectory
        {
            best_visibility = visibility;
            selected = Some((opened.window, workspace, visible));
        }
        if best_visibility == Some(true) {
            break;
        }
    }
    if let Some((mut window, mut workspace, visible)) = selected {
        if !explicit_workspace
            && matches!(visible, OpenVisible::None)
            && let Some((active_window, active_workspace)) =
                cx.update(|cx| active_remote_workspace(&connection_options, cx))
        {
            window = active_window;
            workspace = active_workspace;
        }
        let positions = resolved_paths.context("Remote paths were not resolved")?;
        open_remote_paths(window, workspace, positions, visible, focus, cx).await?;
        return Ok(window);
    }
    if let Some(error) = candidate_error {
        return Err(error);
    }
    Ok(
        connect_remote_project(connection_options, paths, app_state, open_options, None, cx)
            .await?
            .window,
    )
}

async fn connect_remote_project(
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    open_options: OpenOptions,
    reservation: Option<Rc<WorkspaceIdReservation>>,
    cx: &mut AsyncApp,
) -> Result<OpenedRemoteProject> {
    let mut cleanup = RemoteConnectionCleanup {
        action: None,
        cx: cx.clone(),
    };
    let result = connect_remote_project_inner(
        connection_options,
        paths,
        app_state,
        open_options,
        reservation,
        &mut cleanup,
        cx,
    )
    .await;
    cleanup.finish(cx);
    result
}

async fn connect_remote_project_inner(
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    open_options: OpenOptions,
    reservation: Option<Rc<WorkspaceIdReservation>>,
    cleanup: &mut RemoteConnectionCleanup,
    cx: &mut AsyncApp,
) -> Result<OpenedRemoteProject> {
    let focus = open_options.focus.unwrap_or(true);
    let created_new_window = open_options.requesting_window.is_none();
    let (window, initial_workspace) = if let Some(window) = open_options.requesting_window {
        let workspace = window.update(cx, |multi_workspace, _, _| {
            multi_workspace.workspace().clone()
        })?;
        (window, workspace)
    } else {
        let workspace_position = cx
            .update(|cx| {
                workspace::remote_workspace_position_from_db(
                    connection_options.clone(),
                    &paths,
                    open_options.workspace_id,
                    cx,
                )
            })
            .await
            .context("fetching remote workspace position from db")?;

        let mut options =
            cx.update(|cx| (app_state.build_window_options)(workspace_position.display, cx));
        options.window_bounds = workspace_position.window_bounds;
        options.focus = open_options.focus.unwrap_or(options.focus);
        options.show |= !focus;

        let window = cx.open_window(options, |window, cx| {
            let project = project::Project::local(
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                None,
                project::LocalProjectFlags {
                    init_worktree_trust: false,
                    ..Default::default()
                },
                cx,
            );
            let workspace = cx.new(|cx| {
                let mut workspace = Workspace::new(None, project, app_state.clone(), window, cx);
                workspace.centered_layout = workspace_position.centered_layout;
                workspace
            });
            cx.new(|cx| {
                let mut multi_workspace = MultiWorkspace::new(workspace, window, cx);
                if let Some(workspace_id) = open_options.workspace_id {
                    multi_workspace.set_restore_key(workspace_id);
                }
                multi_workspace
            })
        })?;
        let workspace = window.update(cx, |multi_workspace, _, _cx| {
            multi_workspace.workspace().clone()
        })?;
        (window, workspace)
    };

    loop {
        if let Some(reservation) = &reservation {
            reservation.check_cancelled()?;
        }
        let prompt_workspace =
            window.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())?;
        let pending_modal = cx.update(|cx| {
            let modal = prompt_workspace
                .read(cx)
                .active_modal::<RemoteConnectionModal>(cx)?;
            let (dismissed, receiver) = oneshot::channel();
            let dismissed = Rc::new(RefCell::new(Some(dismissed)));
            let subscription = cx.subscribe(&modal, {
                let dismissed = dismissed.clone();
                move |_, _: &DismissEvent, _| {
                    if let Some(dismissed) = dismissed.borrow_mut().take() {
                        dismissed.send(()).ok();
                    }
                }
            });
            let closed = cx.on_window_closed(move |_, closed| {
                if closed == window.window_id() {
                    dismissed.borrow_mut().take();
                }
            });
            Some((receiver, modal, [subscription, closed]))
        });
        if let Some((dismissed, _modal, _subscription)) = pending_modal {
            dismissed
                .await
                .context("Connection window was closed while waiting")?;
            continue;
        }
        let (cancel_tx, mut cancel_rx) = oneshot::channel();
        let delegate = window.update(cx, {
            let paths = paths.clone();
            let connection_options = connection_options.clone();
            let prompt_workspace = prompt_workspace.clone();
            move |_multi_workspace: &mut MultiWorkspace, window, cx| {
                if focus {
                    window.activate_window();
                }
                let delegate = prompt_workspace.update(cx, |workspace, cx| {
                    if workspace.has_active_modal(window, cx) && !workspace.hide_modal(window, cx) {
                        return None;
                    }
                    workspace.toggle_modal(window, cx, |window, cx| {
                        RemoteConnectionModal::new(&connection_options, paths, window, cx)
                    });

                    let modal = workspace.active_modal::<RemoteConnectionModal>(cx)?;
                    let ui = modal.read(cx).prompt.clone();

                    ui.update(cx, |ui, _cx| {
                        ui.set_cancellation_tx(cancel_tx);
                    });

                    Some((
                        Arc::new(RemoteClientDelegate::new(
                            window.window_handle(),
                            ui.downgrade(),
                            if let RemoteConnectionOptions::Ssh(options) = &connection_options {
                                options
                                    .password
                                    .as_deref()
                                    .and_then(|pw| EncryptedPassword::try_from(pw).ok())
                            } else {
                                None
                            },
                        )),
                        modal.downgrade(),
                    ))
                })?;
                let subscription = cx.subscribe(&cx.entity(), {
                    let modal = delegate.1.clone();
                    let prompt_workspace_id = prompt_workspace.entity_id();
                    move |multi_workspace, _, event: &MultiWorkspaceEvent, cx| {
                        let hidden = match event {
                            MultiWorkspaceEvent::ActiveWorkspaceChanged { .. } => {
                                multi_workspace.workspace().entity_id() != prompt_workspace_id
                            }
                            MultiWorkspaceEvent::WorkspaceRemoved(id) => *id == prompt_workspace_id,
                            _ => false,
                        };
                        if hidden {
                            modal.update(cx, |modal, cx| modal.finished(cx)).ok();
                        }
                    }
                });
                Some((delegate, subscription))
            }
        })?;

        let ((delegate, modal), _prompt_subscription) =
            delegate.context("Another modal prevented the remote connection")?;

        cleanup.action = Some(Box::new({
            let initial_workspace = initial_workspace.clone();
            let prompt_workspace = prompt_workspace.clone();
            let modal = modal.clone();
            move |cx: &mut AsyncApp| {
                window
                    .update(cx, |multi_workspace, window, cx| {
                        let active_modal = prompt_workspace
                            .read(cx)
                            .active_modal::<RemoteConnectionModal>(cx);
                        let has_other_modal = active_modal
                            .as_ref()
                            .is_some_and(|active| active.downgrade() != modal);
                        if !has_other_modal {
                            modal.update(cx, |modal, cx| modal.finished(cx)).ok();
                        }
                        let workspace = initial_workspace.read(cx);
                        if created_new_window
                            && !has_other_modal
                            && multi_workspace.workspaces().eq([&initial_workspace])
                            && !multi_workspace.is_workspace_retained(&initial_workspace)
                            && multi_workspace.project_group_keys().is_empty()
                            && workspace.database_id().is_none()
                            && workspace.items(cx).next().is_none()
                            && workspace.worktrees(cx).next().is_none()
                            && !workspace
                                .project()
                                .read(cx)
                                .local_terminal_handles()
                                .iter()
                                .any(|terminal| terminal.is_upgradable())
                        {
                            window.remove_window();
                        }
                    })
                    .ok();
            }
        }));

        let connection = remote::connect(connection_options.clone(), delegate.clone(), cx);
        let connection = select! {
            _ = cancel_rx => {
                modal.update(cx, |modal, cx| modal.finished(cx)).ok();

                break;
            },
            result = connection.fuse() => result,
        };
        if let Some(reservation) = &reservation {
            reservation.check_cancelled()?;
        }
        let remote_connection = match connection {
            Ok(connection) => connection,
            Err(e) => {
                modal.update(cx, |modal, cx| modal.finished(cx)).ok();
                log::error!("Failed to open project: {e:#}");
                let response = window
                    .update(cx, |_, window, cx| {
                        window.prompt(
                            PromptLevel::Critical,
                            match connection_options {
                                RemoteConnectionOptions::Ssh(_) => "Failed to connect over SSH",
                                RemoteConnectionOptions::Wsl(_) => "Failed to connect to WSL",
                                RemoteConnectionOptions::Docker(_) => {
                                    "Failed to connect to Dev Container"
                                }
                                #[cfg(any(test, feature = "test-support"))]
                                RemoteConnectionOptions::Mock(_) => {
                                    "Failed to connect to mock server"
                                }
                            },
                            Some(&format!("{e:#}")),
                            &["Retry", "Cancel"],
                            cx,
                        )
                    })?
                    .await;

                if response == Ok(0) {
                    cleanup.abort();
                    continue;
                }

                return Ok(OpenedRemoteProject {
                    window,
                    workspace: None,
                });
            }
        };

        let (paths, paths_with_positions) = if open_options.workspace_id.is_some() {
            (
                paths.clone(),
                paths
                    .iter()
                    .cloned()
                    .map(PathWithPosition::from_path)
                    .collect::<Vec<_>>(),
            )
        } else {
            determine_paths_with_positions(&remote_connection, paths.clone()).await
        };

        let opened = cx
            .update(|cx| {
                workspace::open_remote_project_with_new_connection(
                    window,
                    remote_connection,
                    cancel_rx,
                    delegate.clone(),
                    app_state.clone(),
                    paths.clone(),
                    open_options.workspace_id,
                    reservation.clone(),
                    focus,
                    cx,
                )
            })
            .await;

        modal.update(cx, |modal, cx| modal.finished(cx)).ok();
        if let Ok((None, _)) = &opened {
            window.read_with(cx, |multi_workspace, _| {
                anyhow::ensure!(
                    multi_workspace
                        .workspaces()
                        .any(|workspace| workspace == &prompt_workspace),
                    "Workspace was removed while connecting"
                );
                Ok(())
            })??;
            break;
        }
        if let Some(reservation) = &reservation {
            reservation.check_cancelled()?;
        }

        match opened {
            Err(e) => {
                log::error!("Failed to open project: {e:#}");
                let response = window
                    .update(cx, |_, window, cx| {
                        window.prompt(
                            PromptLevel::Critical,
                            match connection_options {
                                RemoteConnectionOptions::Ssh(_) => "Failed to connect over SSH",
                                RemoteConnectionOptions::Wsl(_) => "Failed to connect to WSL",
                                RemoteConnectionOptions::Docker(_) => {
                                    "Failed to connect to Dev Container"
                                }
                                #[cfg(any(test, feature = "test-support"))]
                                RemoteConnectionOptions::Mock(_) => {
                                    "Failed to connect to mock server"
                                }
                            },
                            Some(&format!("{e:#}")),
                            &["Retry", "Cancel"],
                            cx,
                        )
                    })?
                    .await;
                if response == Ok(0) {
                    cleanup.abort();
                    continue;
                }

                initial_workspace.update(cx, |workspace, cx| {
                    trusted_worktrees::track_worktree_trust(
                        workspace.project().read(cx).worktree_store(),
                        None,
                        None,
                        None,
                        cx,
                    );
                });
            }

            Ok((workspace, items)) => {
                navigate_to_positions(&window, items, &paths_with_positions, cx);
                return Ok(OpenedRemoteProject { window, workspace });
            }
        }

        break;
    }

    Ok(OpenedRemoteProject {
        window,
        workspace: None,
    })
}

struct RemoteConnectionCleanup {
    action: Option<Box<dyn FnOnce(&mut AsyncApp)>>,
    cx: AsyncApp,
}

impl RemoteConnectionCleanup {
    fn finish(&mut self, cx: &mut AsyncApp) {
        if let Some(action) = self.action.take() {
            action(cx);
        }
    }

    fn abort(&mut self) {
        self.action.take();
    }
}

impl Drop for RemoteConnectionCleanup {
    fn drop(&mut self) {
        if let Some(action) = self.action.take() {
            self.cx.spawn(async move |cx| action(cx)).detach();
        }
    }
}

pub fn navigate_to_positions(
    window: &WindowHandle<MultiWorkspace>,
    items: impl IntoIterator<Item = Option<Box<dyn workspace::item::ItemHandle>>>,
    positions: &[PathWithPosition],
    cx: &mut AsyncApp,
) {
    for (item, path) in items.into_iter().zip(positions) {
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
                        let Some(buffer) = editor.buffer().read(cx).as_singleton() else {
                            return;
                        };
                        let buffer_snapshot = buffer.read(cx).snapshot();
                        let point = buffer_snapshot.point_from_external_input(row, col);
                        editor.go_to_singleton_buffer_point(point, window, cx);
                    });
                })
                .ok();
        }
    }
}

pub(crate) async fn determine_paths_with_positions(
    remote_connection: &Arc<dyn RemoteConnection>,
    mut paths: Vec<PathBuf>,
) -> (Vec<PathBuf>, Vec<PathWithPosition>) {
    let mut paths_with_positions = Vec::<PathWithPosition>::new();
    for path in &mut paths {
        if let Some(position) = position_in_path(path)
            && !path_exists(remote_connection, path).await
        {
            *path = position.path.clone();
            paths_with_positions.push(position);
        } else {
            paths_with_positions.push(PathWithPosition::from_path(path.clone()));
        }
    }
    (paths, paths_with_positions)
}

#[derive(Clone)]
struct OpenedRemoteProject {
    window: WindowHandle<MultiWorkspace>,
    workspace: Option<Entity<Workspace>>,
}

type RemoteWorkspaceFuture =
    LocalBoxFuture<'static, Result<OpenedRemoteProject, Arc<anyhow::Error>>>;
type RemoteWorkspaceLoad = Shared<RemoteWorkspaceFuture>;

struct LoadingRemoteWorkspace {
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    task: WeakShared<RemoteWorkspaceFuture>,
}

#[derive(Default)]
struct RemoteWorkspaceLoads(Rc<RefCell<HashMap<WorkspaceId, LoadingRemoteWorkspace>>>);

impl gpui::Global for RemoteWorkspaceLoads {}

fn remote_workspace_candidates(
    paths: &[PathBuf],
    open_options: &OpenOptions,
    connection_options: &RemoteConnectionOptions,
    cx: &App,
) -> Result<Vec<(Option<WindowHandle<MultiWorkspace>>, WorkspaceMatch)>> {
    if let Some(workspace_id) = open_options.workspace_id {
        return Ok(vec![(
            open_options.requesting_window,
            WorkspaceMatch::Unloaded(workspace_id),
        )]);
    }
    if open_options.workspace_matching == WorkspaceMatching::None {
        return Ok(Vec::new());
    }
    let mut candidate_paths = paths.to_vec();
    candidate_paths.extend(
        paths
            .iter()
            .filter_map(|path| position_in_path(path).map(|position| position.path)),
    );
    candidate_paths.sort();
    candidate_paths.dedup();
    let mut candidates = Vec::new();
    for window in cx
        .windows()
        .into_iter()
        .filter_map(|window| window.downcast::<MultiWorkspace>())
    {
        for workspace in window.read(cx)?.workspaces() {
            let workspace_state = workspace.read(cx);
            let project = workspace_state.project().read(cx);
            if workspace_state.is_restoring()
                || !same_remote_connection_identity(
                    project.remote_connection_options(cx).as_ref(),
                    Some(connection_options),
                )
            {
                continue;
            }
            let priority =
                match open_options
                    .workspace_matching
                    .visibility(project, &candidate_paths, cx)
                {
                    Some(true) if remote_workspace_connection(workspace, cx).is_some() => 0,
                    Some(true) => 2,
                    Some(false) => 3,
                    None if open_options.workspace_matching
                        == WorkspaceMatching::MatchSubdirectory =>
                    {
                        4
                    }
                    None => continue,
                };
            candidates.push((
                priority,
                Some(window),
                WorkspaceMatch::Loaded(workspace.clone()),
            ));
        }
    }
    candidates.extend(
        unloaded_remote_workspace_candidates(
            &candidate_paths,
            open_options,
            connection_options,
            cx,
        )?
        .into_iter()
        .map(|(window, workspace_id)| (2, Some(window), WorkspaceMatch::Unloaded(workspace_id))),
    );
    if let Some(loads) = cx.try_global::<RemoteWorkspaceLoads>() {
        candidates.extend(
            loads
                .0
                .borrow()
                .iter()
                .filter_map(|(workspace_id, loading)| {
                    (same_remote_connection_identity(
                        Some(&loading.connection_options),
                        Some(connection_options),
                    ) && candidate_paths.iter().any(|path| {
                        let path = SanitizedPath::new(path);
                        loading.paths.iter().any(|root| {
                            PathStyle::Unix.strip_prefix(path.as_ref(), root).is_ok_and(
                                |relative| {
                                    open_options.workspace_matching
                                        != WorkspaceMatching::MatchSubpaths
                                        || !relative.as_os_str().is_empty()
                                },
                            )
                        })
                    }))
                    .then_some((
                        1,
                        None,
                        WorkspaceMatch::Unloaded(*workspace_id),
                    ))
                }),
        );
    }
    candidates.sort_by_key(|(priority, _, _)| *priority);
    let mut workspace_ids = HashSet::new();
    candidates.retain(|(_, _, candidate)| {
        let workspace_id = match candidate {
            WorkspaceMatch::Loaded(workspace) => workspace.read(cx).database_id(),
            WorkspaceMatch::Unloaded(workspace_id) => Some(*workspace_id),
        };
        workspace_id.is_none_or(|workspace_id| workspace_ids.insert(workspace_id))
    });
    Ok(candidates
        .into_iter()
        .map(|(_, window, candidate)| (window, candidate))
        .collect())
}

fn active_remote_workspace(
    connection_options: &RemoteConnectionOptions,
    cx: &App,
) -> Option<(WindowHandle<MultiWorkspace>, Entity<Workspace>)> {
    let active_window = cx.active_window().map(|window| window.window_id());
    cx.windows()
        .into_iter()
        .filter_map(|window| {
            let window = window.downcast::<MultiWorkspace>()?;
            let workspace = window.read(cx).ok()?.workspace().clone();
            let workspace_state = workspace.read(cx);
            let project = workspace_state.project().read(cx);
            (!workspace_state.is_restoring()
                && same_remote_connection_identity(
                    project.remote_connection_options(cx).as_ref(),
                    Some(connection_options),
                )
                && remote_workspace_connection(&workspace, cx).is_some())
            .then_some((window, workspace))
        })
        .min_by_key(|(window, _)| Some(window.window_id()) != active_window)
}

fn remote_workspace_connection(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> Option<Arc<dyn RemoteConnection>> {
    workspace
        .read(cx)
        .project()
        .read(cx)
        .remote_client()
        .and_then(|client| client.read(cx).remote_connection())
}

fn position_in_path(path: &Path) -> Option<PathWithPosition> {
    let position = PathWithPosition::parse_str(path.to_str()?);
    position.row.is_some().then_some(position)
}

async fn resolve_remote_paths(
    workspace: &Entity<Workspace>,
    paths: &[PathBuf],
    cx: &mut AsyncApp,
) -> Result<Vec<PathWithPosition>> {
    let (client, mut disconnected, _subscription) = cx.update(|cx| {
        let client = workspace
            .read(cx)
            .project()
            .read(cx)
            .remote_client()
            .context("Remote workspace disconnected while resolving paths")?;
        anyhow::ensure!(
            !client.read(cx).is_disconnected(),
            "Remote workspace disconnected while resolving paths"
        );
        let (disconnected, receiver) = oneshot::channel();
        let mut disconnected = Some(disconnected);
        let subscription = cx.subscribe(&client, move |_, event, _| {
            if matches!(event, remote::RemoteClientEvent::Disconnected { .. })
                && let Some(disconnected) = disconnected.take()
            {
                disconnected.send(()).ok();
            }
        });
        anyhow::Ok((
            client.read(cx).proto_client(),
            receiver.fuse(),
            subscription,
        ))
    })?;
    let mut positions = Vec::with_capacity(paths.len());
    for path in paths {
        let Some(parsed) = position_in_path(path) else {
            positions.push(PathWithPosition::from_path(path.clone()));
            continue;
        };
        let metadata = client.request(proto::GetPathMetadata {
            project_id: proto::REMOTE_SERVER_PROJECT_ID,
            path: path.to_string_lossy().into_owned(),
            canonicalize: false,
        });
        let metadata = select! {
            _ = disconnected => anyhow::bail!("Remote workspace disconnected while resolving paths"),
            metadata = metadata.fuse() => metadata,
        }
        .with_context(|| format!("Failed to resolve remote path {path:?}"))?;
        positions.push(if metadata.exists {
            PathWithPosition::from_path(path.clone())
        } else {
            parsed
        });
    }
    Ok(positions)
}

fn unloaded_remote_workspace_candidates(
    paths: &[PathBuf],
    open_options: &OpenOptions,
    connection_options: &RemoteConnectionOptions,
    cx: &App,
) -> Result<Vec<(WindowHandle<MultiWorkspace>, WorkspaceId)>> {
    let location = SerializedWorkspaceLocation::Remote(connection_options.clone());
    let db = WorkspaceDb::global(cx);
    let mut candidates = Vec::new();
    for (window, workspace_id) in unloaded_workspace_candidates(paths, open_options, &location, cx)?
    {
        let saved = db.session_workspace_for_id(workspace_id)?;
        if matches!(&saved.location, SerializedWorkspaceLocation::Remote(saved)
            if same_remote_connection_identity(Some(saved), Some(connection_options)))
        {
            candidates.push((window, workspace_id));
        }
    }
    Ok(candidates)
}

fn load_remote_workspace(
    workspace_id: WorkspaceId,
    connection_options: RemoteConnectionOptions,
    app_state: Arc<AppState>,
    mut open_options: OpenOptions,
    cx: &mut App,
) -> Result<(RemoteWorkspaceLoad, Vec<PathBuf>)> {
    let loads = cx.default_global::<RemoteWorkspaceLoads>().0.clone();
    if let Some(loading) = loads.borrow().get(&workspace_id)
        && let Some(load) = loading.task.upgrade()
    {
        anyhow::ensure!(
            same_remote_connection_identity(
                Some(&loading.connection_options),
                Some(&connection_options)
            ),
            "Workspace {workspace_id:?} belongs to a different connection"
        );
        return Ok((load, loading.paths.clone()));
    }
    let existing = cx.windows().into_iter().find_map(|window| {
        let window = window.downcast::<MultiWorkspace>()?;
        let workspace = window
            .read(cx)
            .ok()?
            .workspaces()
            .find(|workspace| workspace.read(cx).database_id() == Some(workspace_id))?
            .clone();
        Some((window, workspace))
    });
    let paths = if let Some((window, workspace)) = &existing {
        open_options.requesting_window = Some(*window);
        let remote_workspace = workspace.read(cx);
        anyhow::ensure!(
            same_remote_connection_identity(
                remote_workspace
                    .project()
                    .read(cx)
                    .remote_connection_options(cx)
                    .as_ref(),
                Some(&connection_options),
            ),
            "Workspace {workspace_id:?} belongs to a different connection"
        );
        let paths = remote_workspace
            .root_paths(cx)
            .iter()
            .map(|path| path.to_path_buf())
            .collect::<Vec<_>>();
        if !remote_workspace.is_restoring() && remote_workspace_connection(workspace, cx).is_some()
        {
            return Ok((
                ready(Ok(OpenedRemoteProject {
                    window: *window,
                    workspace: Some(workspace.clone()),
                }))
                .boxed_local()
                .shared(),
                paths,
            ));
        }
        paths
    } else {
        let saved = WorkspaceDb::global(cx).session_workspace_for_id(workspace_id)?;
        anyhow::ensure!(
            matches!(&saved.location, SerializedWorkspaceLocation::Remote(saved)
                if same_remote_connection_identity(Some(saved), Some(&connection_options))),
            "Workspace {workspace_id:?} belongs to a different connection"
        );
        saved.paths.ordered_paths().cloned().collect::<Vec<_>>()
    };
    let reservation = Rc::new(
        WorkspaceIdReservation::new(workspace_id, open_options.requesting_window, cx)
            .with_context(|| format!("Workspace {workspace_id:?} is already open or restoring"))?,
    );
    let remove_load = util::defer({
        let loads = loads.clone();
        move || {
            loads.borrow_mut().remove(&workspace_id);
        }
    });
    let load = {
        let connection_options = connection_options.clone();
        let paths = paths.clone();
        let mut cx = cx.to_async();
        async move {
            let _remove_load = remove_load;
            reservation.check_cancelled()?;
            if let Some((window, workspace)) = existing {
                window
                    .update(&mut cx, |multi_workspace, window, cx| {
                        anyhow::ensure!(
                            multi_workspace
                                .workspaces()
                                .any(|candidate| candidate == &workspace),
                            "Workspace was closed while reconnecting"
                        );
                        Ok(workspace.update(cx, |workspace, cx| {
                            if let Some(overlay) = workspace.active_modal::<DisconnectedOverlay>(cx)
                            {
                                overlay.update(cx, |overlay, cx| overlay.finished(cx));
                            }
                            workspace.flush_serialization_checked(window, cx)
                        }))
                    })??
                    .await?;
            }
            reservation.check_cancelled()?;
            connect_remote_project(
                connection_options,
                paths,
                app_state,
                open_options,
                Some(reservation),
                &mut cx,
            )
            .await
        }
    }
    .map(|result| result.map_err(Arc::new))
    .boxed_local()
    .shared();
    if let Some(task) = load.downgrade() {
        loads.borrow_mut().insert(
            workspace_id,
            LoadingRemoteWorkspace {
                connection_options,
                paths: paths.clone(),
                task,
            },
        );
    }
    Ok((load, paths))
}

async fn open_remote_paths(
    window: WindowHandle<MultiWorkspace>,
    workspace: Entity<Workspace>,
    mut positions: Vec<PathWithPosition>,
    visible: OpenVisible,
    focus: bool,
    cx: &mut AsyncApp,
) -> Result<()> {
    positions.sort_by(|left, right| left.path.cmp(&right.path));
    let paths = positions
        .iter()
        .map(|position| position.path.clone())
        .collect::<Vec<_>>();
    let opened = window
        .update(cx, |multi_workspace, window, cx| {
            anyhow::ensure!(
                multi_workspace
                    .workspaces()
                    .any(|candidate| candidate == &workspace),
                "Workspace was closed while opening paths"
            );
            anyhow::ensure!(
                remote_workspace_connection(&workspace, cx).is_some(),
                "Remote workspace disconnected while opening paths"
            );
            if focus {
                window.activate_window();
                multi_workspace.activate(workspace.clone(), None, window, cx);
            }
            Ok(workspace.update(cx, |workspace, cx| {
                workspace.open_paths(
                    paths,
                    OpenOptions {
                        visible: Some(visible),
                        focus: Some(focus),
                        ..OpenOptions::default()
                    },
                    None,
                    window,
                    cx,
                )
            }))
        })??
        .await;
    workspace.update(cx, |workspace, cx| {
        for error in opened
            .iter()
            .flatten()
            .filter_map(|item| item.as_ref().err())
        {
            workspace.show_error(format!("{error}"), cx);
        }
    });
    let items = opened
        .into_iter()
        .map(|item| item.and_then(Result::ok))
        .collect::<Vec<_>>();
    navigate_to_positions(&window, items, &positions, cx);
    Ok(())
}

async fn path_exists(connection: &Arc<dyn RemoteConnection>, path: &Path) -> bool {
    let Ok(command) = connection.build_command(
        Some("test".to_string()),
        &["-e".to_owned(), path.to_string_lossy().to_string()],
        &Default::default(),
        None,
        None,
        Interactive::No,
    ) else {
        return false;
    };
    let Ok(mut child) = util::command::new_command(command.program)
        .args(command.args)
        .envs(command.env)
        .spawn()
    else {
        return false;
    };
    child.status().await.is_ok_and(|status| status.success())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::init_test;
    use extension::ExtensionHostProxy;
    use fs::{FakeFs, Fs, RemoveOptions};
    use gpui::{AppContext, Focusable as _, FutureExt as _, Task, TestAppContext};
    use http_client::BlockedHttpClient;
    use language::Point;
    use node_runtime::NodeRuntime;
    use project::{Project, ProjectGroupKey};
    use remote::{ConnectionState, RemoteClient, WslConnectionOptions};
    use remote_server::{HeadlessAppState, HeadlessProject};
    use rpc::{AnyProtoClient, proto};
    use serde_json::json;
    use std::{collections::BTreeMap, time::Duration};
    use util::{path, path_list::PathList};
    use workspace::{
        Item, MultiWorkspaceState, RemovalIntent, SerializedMultiWorkspace, SerializedProjectGroup,
        SessionWorkspace, WorkspaceRestorePlan, apply_restored_multiworkspace_state,
        find_existing_workspace,
        item::{item_persistence_id, test::set_item_persistence_id},
    };

    #[gpui::test]
    async fn test_open_remote_project_with_mock_connection(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        let executor = cx.executor();

        cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });

        let (opts, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);

        let remote_fs = FakeFs::new(server_cx.executor());
        remote_fs
            .insert_tree(
                path!("/project"),
                json!({
                    "src": {
                        "main.rs": "fn main() {}",
                    },
                    "README.md": "# Test Project",
                }),
            )
            .await;

        server_cx.update(HeadlessProject::init);
        let _headless = create_headless_project(server_session, remote_fs, server_cx);

        drop(connect_guard);

        let paths = vec![PathBuf::from(path!("/project"))];
        let open_options = workspace::OpenOptions::default();

        let mut async_cx = cx.to_async();
        let result = open_remote_project(opts, paths, app_state, open_options, &mut async_cx).await;

        executor.run_until_parked();

        assert!(result.is_ok(), "open_remote_project should succeed");

        let windows = cx.update(|cx| cx.windows().len());
        assert_eq!(windows, 1, "Should have opened a window");

        let multi_workspace_handle =
            cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());

        multi_workspace_handle
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project().read(cx);
                    assert!(project.is_remote(), "Project should be a remote project");
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_cancel_remote_open_preserves_reused_placeholder(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        server_cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));

        for failure in ["connect", "setup", "cancel", "drop"] {
            for reuse in ["untouched", "edited", "held", "replaced"] {
                let (options, server_session, connect_guard) =
                    RemoteClient::fake_server(cx, server_cx);
                let ping_handler = server_cx.new(|_| ());
                server_session.add_request_handler::<proto::Ping, _, _, _>(
                    ping_handler.downgrade(),
                    |_, _, _| async { anyhow::bail!("mock setup failed") },
                );
                if failure == "connect" {
                    let RemoteConnectionOptions::Mock(options) = &options else {
                        unreachable!();
                    };
                    cx.update(|cx| {
                        assert!(
                            cx.global_mut::<remote::MockConnectionRegistry>()
                                .take(options)
                                .is_some()
                        );
                    });
                }
                if failure != "drop" && failure != "cancel" {
                    drop(connect_guard);
                }
                let opening = cx.spawn({
                    let app_state = app_state.clone();
                    async move |mut cx| {
                        let result = open_remote_project(
                            options,
                            Vec::new(),
                            app_state,
                            OpenOptions::default(),
                            &mut cx,
                        )
                        .await;
                        assert_eq!(
                            cx.update(|cx| cx.windows().len()),
                            usize::from(reuse != "untouched"),
                            "failure={failure}, reuse={reuse}"
                        );
                        result
                    }
                });
                cx.run_until_parked();
                if failure == "drop" || failure == "cancel" {
                    assert_eq!(cx.pending_prompt(), None);
                } else {
                    assert!(cx.has_pending_prompt());
                }
                assert_eq!(cx.windows().len(), 1);
                let window = cx.windows()[0].downcast::<MultiWorkspace>().unwrap();
                let initial_workspace = window
                    .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
                    .unwrap();

                let editor = if reuse == "edited" {
                    let editor = window
                        .update(cx, |_, window, cx| {
                            initial_workspace.update(cx, |workspace, cx| {
                                Editor::new_in_workspace(workspace, window, cx)
                            })
                        })
                        .unwrap()
                        .await
                        .unwrap();
                    window
                        .update(cx, |_, window, cx| {
                            editor.update(cx, |editor, cx| {
                                editor.insert("unsaved local work", window, cx);
                            });
                        })
                        .unwrap();
                    Some(editor)
                } else {
                    None
                };
                if reuse == "held" || reuse == "replaced" {
                    let project = Project::test(app_state.fs.clone(), [], cx).await;
                    window
                        .update(cx, |multi_workspace, window, cx| {
                            let workspace = cx.new(|cx| {
                                Workspace::new(None, project, app_state.clone(), window, cx)
                            });
                            if reuse == "held" {
                                multi_workspace.add(workspace, window, cx);
                            } else {
                                multi_workspace.activate(workspace, None, window, cx);
                            }
                        })
                        .unwrap();
                }
                cx.run_until_parked();
                let expected_workspaces = window
                    .read_with(cx, |multi_workspace, _| {
                        multi_workspace.workspaces().cloned().collect::<Vec<_>>()
                    })
                    .unwrap();

                if failure == "drop" {
                    drop(opening);
                } else {
                    if failure == "cancel" {
                        let modal = initial_workspace.read_with(cx, |workspace, cx| {
                            workspace.active_modal::<RemoteConnectionModal>(cx)
                        });
                        if let Some(modal) = modal {
                            window
                                .update(cx, |_, window, cx| {
                                    modal.read(cx).focus_handle(cx).focus(window, cx);
                                })
                                .unwrap();
                            cx.dispatch_action(window.into(), menu::Cancel);
                        }
                    } else {
                        cx.simulate_prompt_answer("Cancel");
                    }
                    assert_eq!(opening.await.unwrap(), window);
                }
                cx.run_until_parked();
                if reuse == "untouched" {
                    assert!(cx.windows().is_empty());
                } else {
                    assert_eq!(cx.windows().len(), 1);
                    window
                        .update(cx, |multi_workspace, window, cx| {
                            assert_eq!(
                                multi_workspace.workspaces().cloned().collect::<Vec<_>>(),
                                expected_workspaces
                            );
                            if let Some(editor) = editor {
                                assert_eq!(editor.read(cx).text(cx), "unsaved local work");
                            }
                            assert!(
                                initial_workspace
                                    .read(cx)
                                    .active_modal::<RemoteConnectionModal>(cx)
                                    .is_none()
                            );
                            window.remove_window();
                        })
                        .unwrap();
                }
            }
        }
    }

    #[gpui::test]
    async fn test_find_existing_workspace_does_not_reuse_workspace_from_another_remote(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        let executor = cx.executor();

        cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });

        let (opts, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);

        let remote_fs = FakeFs::new(server_cx.executor());
        remote_fs
            .insert_tree(
                path!("/project"),
                json!({
                    "src": {
                        "main.rs": "fn main() {}",
                        "lib.rs": "pub fn hello() {}",
                    },
                    "README.md": "# Test Project",
                }),
            )
            .await;

        server_cx.update(HeadlessProject::init);
        let _headless = create_headless_project(server_session, remote_fs, server_cx);

        drop(connect_guard);

        // First open: create a new window for the remote project.
        let paths = vec![PathBuf::from(path!("/project"))];
        let mut async_cx = cx.to_async();
        open_remote_project(
            opts.clone(),
            paths,
            app_state.clone(),
            workspace::OpenOptions::default(),
            &mut async_cx,
        )
        .await
        .expect("first open_remote_project should succeed");

        executor.run_until_parked();

        assert_eq!(
            cx.update(|cx| cx.windows().len()),
            1,
            "First open should create exactly one window"
        );

        let first_window = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let first_workspace = first_window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();

        let (other_opts, other_server_session, other_connect_guard) =
            RemoteClient::fake_server(cx, server_cx);
        let ping_handler = server_cx.new(|_| ());
        other_server_session.add_request_handler::<proto::Ping, _, _, _>(
            ping_handler.downgrade(),
            |_, _, _| async { Ok(proto::Ack {}) },
        );
        drop(other_connect_guard);
        let other_remote_client = RemoteClient::connect_mock(other_opts.clone(), cx).await;
        let other_project = cx.update(|cx| {
            Project::remote(
                other_remote_client,
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                false,
                cx,
            )
        });
        first_window
            .update(cx, |multi_workspace, window, cx| {
                let other_workspace =
                    cx.new(|cx| Workspace::new(None, other_project, app_state.clone(), window, cx));
                multi_workspace.add(other_workspace, window, cx);
            })
            .unwrap();

        let search_paths = vec![PathBuf::from(path!("/project/src/lib.rs"))];
        let (found, _) = find_existing_workspace(
            &search_paths,
            &workspace::OpenOptions::default(),
            &SerializedWorkspaceLocation::Remote(other_opts),
            &mut async_cx,
        )
        .await
        .unwrap();
        assert!(
            found.is_none(),
            "a matching path on another remote must not satisfy the request"
        );

        // Verify find_existing_workspace discovers the remote workspace.
        let (found, _open_visible) = find_existing_workspace(
            &search_paths,
            &workspace::OpenOptions::default(),
            &SerializedWorkspaceLocation::Remote(opts.clone()),
            &mut async_cx,
        )
        .await
        .unwrap();

        assert!(
            found.is_some(),
            "find_existing_workspace should locate the existing remote workspace"
        );
        let (found_window, WorkspaceMatch::Loaded(found_workspace)) = found.unwrap() else {
            panic!("expected the loaded remote workspace");
        };
        assert_eq!(
            found_window, first_window,
            "find_existing_workspace should return the same window"
        );
        assert_eq!(
            found_workspace, first_workspace,
            "find_existing_workspace should return the workspace for the requested remote"
        );

        // Second open with the same connection options should reuse the window.
        let second_paths = vec![PathBuf::from(path!("/project/src/lib.rs"))];
        open_remote_project(
            opts.clone(),
            second_paths,
            app_state.clone(),
            workspace::OpenOptions::default(),
            &mut async_cx,
        )
        .await
        .expect("second open_remote_project should succeed via reuse");

        executor.run_until_parked();

        assert_eq!(
            cx.update(|cx| cx.windows().len()),
            1,
            "Second open should reuse the existing window, not create a new one"
        );

        let still_first_window =
            cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        assert_eq!(
            still_first_window, first_window,
            "The window handle should be the same after reuse"
        );
    }

    #[gpui::test]
    async fn test_reopen_existing_remote_root_treats_root_as_directory(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        let executor = cx.executor();

        cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });

        let (opts, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);

        let remote_fs = FakeFs::new(server_cx.executor());
        let remote_home = paths::home_dir();
        let canonical_project_path = remote_home.join("remote-reopen-root-project");
        remote_fs
            .insert_tree(
                &canonical_project_path,
                json!({
                    "src": {
                        "main.rs": "fn main() {}",
                    },
                    "README.md": "# Test Project",
                }),
            )
            .await;

        server_cx.update(HeadlessProject::init);
        let _headless = create_headless_project(server_session, remote_fs, server_cx);

        drop(connect_guard);

        let mut async_cx = cx.to_async();
        let window = open_remote_project(
            opts,
            vec![canonical_project_path.clone()],
            app_state,
            workspace::OpenOptions::default(),
            &mut async_cx,
        )
        .await
        .expect("initial open_remote_project should succeed");

        executor.run_until_parked();

        let open_results = window
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    workspace.open_paths(
                        vec![canonical_project_path.clone()],
                        workspace::OpenOptions {
                            visible: Some(workspace::OpenVisible::All),
                            ..Default::default()
                        },
                        None,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await;

        assert_eq!(open_results.len(), 1, "should return one open result");
        assert!(
            open_results[0].is_none(),
            "reopening a remote root directory should not try to open it as a file"
        );
    }

    #[gpui::test]
    async fn test_reconnect_when_server_not_running(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        let executor = cx.executor();

        cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });

        let (opts, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);

        let remote_fs = FakeFs::new(server_cx.executor());
        remote_fs
            .insert_tree(
                path!("/project"),
                json!({
                    "src": {
                        "main.rs": "fn main() {}",
                    },
                }),
            )
            .await;

        server_cx.update(HeadlessProject::init);
        let _headless = create_headless_project(server_session, remote_fs.clone(), server_cx);

        drop(connect_guard);

        // Open the remote project normally.
        let paths = vec![PathBuf::from(path!("/project"))];
        let mut async_cx = cx.to_async();
        open_remote_project(
            opts.clone(),
            paths.clone(),
            app_state.clone(),
            workspace::OpenOptions::default(),
            &mut async_cx,
        )
        .await
        .expect("initial open should succeed");

        executor.run_until_parked();

        assert_eq!(cx.update(|cx| cx.windows().len()), 1);
        let window = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let project = window
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace.workspace().read(cx).project().clone()
            })
            .unwrap();
        let duplicate_id = cx
            .update(|cx| WorkspaceDb::global(cx))
            .next_id()
            .await
            .unwrap();
        let duplicate = cx.add_window(|window, cx| {
            let workspace = cx.new(|cx| {
                Workspace::new(Some(duplicate_id), project, app_state.clone(), window, cx)
            });
            MultiWorkspace::new(workspace, window, cx)
        });
        let (matched, _) = find_existing_workspace(
            &paths,
            &OpenOptions::default(),
            &SerializedWorkspaceLocation::Remote(opts.clone()),
            &mut async_cx,
        )
        .await
        .unwrap();
        let other_window = matched.unwrap().0;
        let window = if window == other_window {
            duplicate
        } else {
            window
        };
        let original_workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let other_workspace = other_window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let other_draft = other_window
            .update(cx, |_, window, cx| {
                other_workspace.update(cx, |workspace, cx| {
                    Editor::new_in_workspace(workspace, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        other_window
            .update(cx, |_, window, cx| {
                other_draft.update(cx, |editor, cx| {
                    editor.insert("untouched draft", window, cx)
                });
            })
            .unwrap();
        let (workspace_id, workspace_ids) = window
            .read_with(cx, |multi_workspace, cx| {
                let mut workspace_ids = multi_workspace
                    .workspaces()
                    .map(|workspace| workspace.read(cx).database_id())
                    .collect::<Vec<_>>();
                workspace_ids.sort();
                (
                    multi_workspace.workspace().read(cx).database_id().unwrap(),
                    workspace_ids,
                )
            })
            .unwrap();
        let items = window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.open_paths(
                        vec![PathBuf::from(path!("/project/src/main.rs"))],
                        OpenOptions::default(),
                        None,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await;
        assert_eq!(items.len(), 1);
        let file = items.into_iter().next().unwrap().unwrap().unwrap();
        let file = file.downcast::<Editor>().unwrap();
        assert_eq!(
            file.read_with(cx, |editor, cx| editor.text(cx)),
            "fn main() {}"
        );
        let draft = window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    Editor::new_in_workspace(workspace, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        window
            .update(cx, |multi_workspace, window, cx| {
                draft.update(cx, |editor, cx| {
                    editor.insert("saved dirty draft", window, cx);
                });
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.flush_serialization_checked(window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();

        // Force the remote client into ServerNotRunning state (simulates the
        // scenario where the remote server died and reconnection failed).
        window
            .update(cx, |multi_workspace, window, cx| {
                draft.update(cx, |editor, cx| {
                    editor.insert(" with pending edits", window, cx);
                });
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    let client = workspace
                        .project()
                        .read(cx)
                        .remote_client()
                        .expect("should have remote client");
                    client.update(cx, |client, cx| {
                        client.force_server_not_running(cx);
                    });
                });
            })
            .unwrap();

        executor.run_until_parked();
        draft.read_with(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "saved dirty draft with pending edits");
            assert!(
                editor
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .unwrap()
                    .read(cx)
                    .has_unsaved_edits()
            );
        });

        // Register a new mock server under the same options so the reconnect
        // path can establish a fresh connection.
        let (server_session_2, connect_guard_2) =
            RemoteClient::fake_server_with_opts(&opts, cx, server_cx);

        let _headless_2 = create_headless_project(server_session_2, remote_fs, server_cx);

        drop(connect_guard_2);

        let result = window
            .update(cx, |_, window, cx| {
                let overlay = original_workspace
                    .read(cx)
                    .active_modal::<DisconnectedOverlay>(cx)
                    .unwrap();
                overlay.update(cx, |overlay, cx| overlay.reconnect(window, cx))
            })
            .unwrap()
            .with_timeout(Duration::from_secs(10), &cx.executor())
            .await;
        assert_eq!(cx.pending_prompt(), None);
        let result = result.expect("remote reconnect timed out");

        executor.run_until_parked();

        assert!(
            result.is_ok(),
            "reconnect should succeed but got: {:?}",
            result.err()
        );

        result.unwrap();

        assert_eq!(cx.update(|cx| cx.windows().len()), 2);
        other_window
            .read_with(cx, |multi_workspace, cx| {
                assert_eq!(multi_workspace.workspace(), &other_workspace);
                assert!(
                    other_workspace
                        .read(cx)
                        .project()
                        .read(cx)
                        .is_disconnected(cx)
                );
                assert_eq!(other_draft.read(cx).text(cx), "untouched draft");
            })
            .unwrap();

        window
            .update(cx, |multi_workspace, _, cx| {
                assert_ne!(multi_workspace.workspace(), &original_workspace);
                let mut restored_workspace_ids = multi_workspace
                    .workspaces()
                    .map(|workspace| workspace.read(cx).database_id())
                    .collect::<Vec<_>>();
                restored_workspace_ids.sort();
                assert_eq!(restored_workspace_ids, workspace_ids);
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert!(
                        workspace.project().read(cx).is_remote(),
                        "project should be remote after reconnect"
                    );
                    assert_eq!(workspace.database_id(), Some(workspace_id));
                    assert!(
                        workspace
                            .project()
                            .read(cx)
                            .remote_client()
                            .unwrap()
                            .read(cx)
                            .remote_connection()
                            .is_some()
                    );
                    let mut restored = workspace
                        .items_of_type::<Editor>(cx)
                        .map(|editor| {
                            let editor = editor.read(cx);
                            (
                                editor.text(cx),
                                editor
                                    .buffer()
                                    .read(cx)
                                    .as_singleton()
                                    .unwrap()
                                    .read(cx)
                                    .is_dirty(),
                            )
                        })
                        .collect::<Vec<_>>();
                    restored.sort();
                    assert_eq!(
                        restored,
                        vec![
                            ("fn main() {}".to_owned(), false),
                            ("saved dirty draft with pending edits".to_owned(), true),
                        ]
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_concurrent_lazy_remote_opens_join_explicit_restore(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
            HeadlessProject::init(cx);
        });
        let (options, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);
        let remote_fs = FakeFs::new(server_cx.executor());
        remote_fs
            .insert_tree(
                path!("/project"),
                json!({"src": {"main.rs": "fn main() {}", "other.rs": "first\nsecond\n"}}),
            )
            .await;
        let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
        drop(connect_guard);

        let mut async_cx = cx.to_async();
        let roots = [
            path!("/project/src/main.rs"),
            path!("/project/src/other.rs"),
        ];
        let remote_id = save_remote_workspace(&options, &roots, &app_state, cx).await;
        drop(headless);
        cx.run_until_parked();

        let window = restore_unloaded_remote_workspaces(&[remote_id], &app_state, cx).await;
        let local_workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        window
            .read_with(cx, |multi_workspace, cx| {
                assert_eq!(multi_workspace.workspace(), &local_workspace);
                assert_eq!(multi_workspace.workspaces().count(), 1);
                assert!(!local_workspace.read(cx).project().read(cx).is_remote());
            })
            .unwrap();

        let (server_session, connect_guard) =
            RemoteClient::fake_server_with_opts(&options, cx, server_cx);
        let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);

        let root_path = vec![PathBuf::from(roots[0])];
        let location = SerializedWorkspaceLocation::Remote(options.clone());
        let candidate = find_existing_workspace(
            &root_path,
            &OpenOptions::default(),
            &location,
            &mut async_cx,
        )
        .await
        .unwrap();
        assert_eq!(
            candidate.0,
            Some((window, WorkspaceMatch::Unloaded(remote_id)))
        );

        let mut restoring = spawn_remote_open(
            &options,
            &roots,
            &app_state,
            OpenOptions {
                workspace_id: Some(remote_id),
                requesting_window: Some(window),
                focus: Some(false),
                ..OpenOptions::default()
            },
            cx,
        );
        cx.run_until_parked();
        let completed = (&mut restoring).now_or_never();
        assert!(
            completed.is_none(),
            "restore completed before connection barrier: {completed:?}"
        );
        assert_eq!(cx.pending_prompt(), None);
        let modal = local_workspace.read_with(cx, |workspace, cx| {
            workspace.active_modal::<RemoteConnectionModal>(cx).unwrap()
        });
        let opening = [
            path!("/project/src/main.rs:1:4"),
            path!("/project/src/other.rs:2:3"),
        ]
        .map(|path| {
            spawn_remote_open(
                &options,
                &[path],
                &app_state,
                OpenOptions {
                    wait: true,
                    ..OpenOptions::default()
                },
                cx,
            )
        });
        cx.run_until_parked();
        local_workspace.read_with(cx, |workspace, cx| {
            assert_eq!(
                workspace.active_modal::<RemoteConnectionModal>(cx),
                Some(modal)
            );
        });
        drop(restoring);
        drop(connect_guard);
        for opening in opening {
            let reopened = opening
                .with_timeout(Duration::from_secs(10), &cx.executor())
                .await;
            assert_eq!(cx.pending_prompt(), None);
            assert_eq!(reopened.expect("remote reopen timed out").unwrap(), window);
        }
        let loaded_match = find_existing_workspace(
            &root_path,
            &OpenOptions::default(),
            &location,
            &mut async_cx,
        )
        .await
        .unwrap();
        let (loaded_window, WorkspaceMatch::Loaded(loaded_workspace)) = loaded_match.0.unwrap()
        else {
            panic!("expected restored remote workspace");
        };
        assert_eq!(loaded_window, window);
        assert_eq!(
            loaded_workspace.read_with(cx, |workspace, _| workspace.database_id()),
            Some(remote_id),
        );
        drop(loaded_workspace);
        cx.run_until_parked();
        cx.update(|cx| assert!(cx.global::<RemoteWorkspaceLoads>().0.borrow().is_empty()));
        assert_eq!(cx.windows().len(), 1);
        window
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().read(cx);
                assert_eq!(workspace.database_id(), Some(remote_id));
                assert_eq!(
                    workspace
                        .root_paths(cx)
                        .iter()
                        .map(|path| path.to_path_buf())
                        .collect::<Vec<_>>(),
                    roots.iter().map(PathBuf::from).collect::<Vec<_>>()
                );
                let editors = workspace.items_of_type::<Editor>(cx).collect::<Vec<_>>();
                let mut restored = editors
                    .into_iter()
                    .map(|editor| {
                        editor.update(cx, |editor, cx| {
                            (
                                editor.text(cx),
                                editor
                                    .selections
                                    .newest::<Point>(&editor.display_snapshot(cx))
                                    .head(),
                            )
                        })
                    })
                    .collect::<Vec<_>>();
                restored.sort_by(|left, right| left.0.cmp(&right.0));
                assert_eq!(
                    restored,
                    vec![
                        ("first\nsecond\n".to_owned(), Point::new(1, 2)),
                        ("fn main() {}".to_owned(), Point::new(0, 3)),
                    ]
                );
                assert_eq!(
                    multi_workspace
                        .workspaces()
                        .filter(|workspace| **workspace == local_workspace)
                        .count(),
                    1
                );
                assert_eq!(multi_workspace.workspaces().count(), 2);
            })
            .unwrap();

        close_remote_window(window, cx);
        drop(headless);
        cx.run_until_parked();

        for cancellation in ["remove", "drop", "close"] {
            let (server_session, connect_guard) =
                RemoteClient::fake_server_with_opts(&options, cx, server_cx);
            let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
            let requesting_window = if cancellation == "remove" {
                Some(restore_unloaded_remote_workspaces(&[remote_id], &app_state, cx).await)
            } else {
                None
            };
            let opening = spawn_remote_open(
                &options,
                &roots,
                &app_state,
                OpenOptions {
                    workspace_id: Some(remote_id),
                    requesting_window,
                    ..OpenOptions::default()
                },
                cx,
            );
            cx.run_until_parked();
            assert_eq!(cx.windows().len(), 1);
            if cancellation == "drop" {
                drop(opening);
            } else if cancellation == "remove" {
                let window = requesting_window.unwrap();
                let removal = window
                    .update(cx, |multi_workspace, window, cx| {
                        multi_workspace.remove_project_group(
                            &ProjectGroupKey::new(
                                Some(options.clone()),
                                PathList::new(&roots.iter().map(PathBuf::from).collect::<Vec<_>>()),
                            ),
                            window,
                            cx,
                        )
                    })
                    .unwrap();
                cx.run_until_parked();
                assert_eq!(
                    cx.pending_prompt(),
                    Some((
                        "Some items have not finished restoring. Close without saving their changes?".to_owned(),
                        String::new(),
                    )),
                );
                cx.simulate_prompt_answer("Close Without Saving");
                assert!(removal.await.unwrap());
                drop(connect_guard);
                assert!(
                    opening
                        .with_timeout(Duration::from_secs(10), &cx.executor())
                        .await
                        .expect("removed remote restore timed out")
                        .is_err()
                );
                assert_eq!(cx.pending_prompt(), None);
                window
                    .update(cx, |multi_workspace, window, cx| {
                        assert_eq!(multi_workspace.workspaces().count(), 1);
                        assert!(
                            !multi_workspace
                                .workspace()
                                .read(cx)
                                .project()
                                .read(cx)
                                .is_remote()
                        );
                        assert!(
                            multi_workspace
                                .project_group_keys()
                                .iter()
                                .all(|key| key.host().is_none())
                        );
                        window.remove_window();
                    })
                    .unwrap();
            } else {
                let file = spawn_remote_open(
                    &options,
                    &[path!("/project/src/main.rs")],
                    &app_state,
                    OpenOptions::default(),
                    cx,
                );
                cx.run_until_parked();
                cx.windows()[0]
                    .update(cx, |_, window, _| window.remove_window())
                    .unwrap();
                drop(connect_guard);
                for opening in [opening, file] {
                    assert!(
                        opening
                            .with_timeout(Duration::from_secs(10), &cx.executor())
                            .await
                            .expect("closed remote restore timed out")
                            .is_err()
                    );
                }
            }
            cx.run_until_parked();
            assert!(cx.windows().is_empty());
            cx.update(|cx| assert!(cx.global::<RemoteWorkspaceLoads>().0.borrow().is_empty()));
            drop(headless);
            cx.run_until_parked();
        }
        remote_fs
            .insert_tree(path!("/missing"), json!({"project": {}}))
            .await;
        let (options, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);
        let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
        drop(connect_guard);
        let roots = [path!("/project"), path!("/missing/project")];
        let remote_id = save_remote_workspace(&options, &roots, &app_state, cx).await;
        drop(headless);
        cx.run_until_parked();
        let root_path = vec![PathBuf::from(roots[0])];
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        for (workspace_id, close_window, stage) in [
            (Some(remote_id), false, "root"),
            (None, false, "root"),
            (Some(remote_id), true, "root"),
            (Some(remote_id), false, "metadata"),
            (Some(remote_id), false, "error"),
        ] {
            let window = restore_unloaded_remote_workspaces(&[remote_id], &app_state, cx).await;
            window
                .update(cx, |multi_workspace, window, cx| {
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .unwrap()
                .await
                .unwrap();
            let initial_workspace = window
                .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
                .unwrap();
            let persisted_state = || {
                (
                    db.select::<(WorkspaceId, Option<String>, Option<String>)>(
                        "SELECT workspace_id, paths, paths_order FROM workspaces
                         WHERE remote_connection_id IS NOT NULL ORDER BY workspace_id",
                    )
                    .unwrap()()
                    .unwrap(),
                    db.select::<(u64, u64, u64, String, i64, i64)>(
                        "SELECT workspace_id, item_id, pane_id, kind, position, active
                         FROM items ORDER BY workspace_id, item_id",
                    )
                    .unwrap()()
                    .unwrap(),
                )
            };
            let saved = persisted_state();
            let remote_project = Rc::new(RefCell::new(None));
            let _subscription = cx.update(|cx| {
                let remote_project = remote_project.clone();
                cx.observe_new::<Project>(move |project, _, cx| {
                    if let Some(client) = project.remote_client() {
                        assert_eq!(
                            client.read(cx).connection_state(),
                            ConnectionState::Connected
                        );
                        *remote_project.borrow_mut() = Some(cx.entity());
                    }
                })
            });
            assert!(!cx.update(|cx| remote::has_active_connection(&options, cx)));
            let (server_session, connect_guard) =
                RemoteClient::fake_server_with_opts(&options, cx, server_cx);
            let (started_tx, started_rx) = oneshot::channel();
            let (release_tx, release_rx) = oneshot::channel();
            let barrier = server_cx.new(|_| Some((started_tx, release_rx)));
            server_session.add_request_handler::<proto::Ping, _, _, _>(
                barrier.downgrade(),
                |_, _, _| async { Ok(proto::Ack {}) },
            );
            server_session.add_request_handler::<proto::AddWorktree, _, _, _>(
                barrier.downgrade(),
                move |barrier, message, mut cx| async move {
                    if stage == "metadata" {
                        if message.payload.path == path!("/missing/project") {
                            return Err(anyhow::Error::from(
                                proto::ErrorCode::DevServerProjectPathDoesNotExist,
                            ));
                        }
                    } else {
                        let (started, release) =
                            barrier.update(&mut cx, |barrier, _| barrier.take().unwrap());
                        started.send(()).unwrap();
                        release.await?;
                        anyhow::ensure!(stage != "error", "remote root failed while cancelling");
                    }
                    Ok(proto::AddWorktreeResponse {
                        worktree_id: 1,
                        canonicalized_path: message.payload.path,
                        root_repo_common_dir: None,
                        root_repo_is_linked_worktree: false,
                    })
                },
            );
            server_session.add_request_handler::<proto::GetPathMetadata, _, _, _>(
                barrier.downgrade(),
                move |barrier, message, mut cx| async move {
                    assert_eq!(stage, "metadata");
                    assert_eq!(message.payload.path, path!("/project"));
                    let (started, release) =
                        barrier.update(&mut cx, |barrier, _| barrier.take().unwrap());
                    started.send(()).unwrap();
                    release.await?;
                    Ok(proto::GetPathMetadataResponse {
                        exists: true,
                        is_dir: true,
                        path: message.payload.path,
                    })
                },
            );
            drop(connect_guard);
            let opening = spawn_remote_open(
                &options,
                &[if workspace_id.is_some() {
                    roots[0]
                } else {
                    path!("/fresh")
                }],
                &app_state,
                OpenOptions {
                    workspace_id,
                    requesting_window: Some(window),
                    ..OpenOptions::default()
                },
                cx,
            );
            started_rx
                .with_timeout(Duration::from_secs(10), &cx.executor())
                .await
                .unwrap_or_else(|error| {
                    panic!("remote {stage} request did not start for {workspace_id:?}: {error:?}; prompt: {:?}", cx.pending_prompt())
                })
                .unwrap();
            assert!(!opening.is_ready());
            if stage == "metadata" {
                cx.executor()
                    .advance_clock(workspace::SERIALIZATION_THROTTLE_TIME);
                cx.run_until_parked();
                assert_eq!(persisted_state(), saved);
            }
            let modal = initial_workspace.read_with(cx, |workspace, cx| {
                workspace.active_modal::<RemoteConnectionModal>(cx).unwrap()
            });
            window
                .update(cx, |_, window, cx| {
                    modal.read(cx).focus_handle(cx).focus(window, cx)
                })
                .unwrap();
            let mut release_tx = Some(release_tx);
            if stage == "error" {
                release_tx.take().unwrap().send(()).unwrap();
            }
            cx.update_window(window.into(), |_, window, cx| {
                modal
                    .read(cx)
                    .focus_handle(cx)
                    .dispatch_action(&menu::Cancel, window, cx);
                if close_window {
                    window.remove_window();
                }
            })
            .unwrap();
            assert!(!opening.is_ready());
            cx.run_until_parked();
            assert!(
                opening.is_ready(),
                "cancel must finish before the root request is released"
            );
            let opened = opening.await;
            if let Some(release_tx) = release_tx {
                release_tx.send(()).ok();
            }
            cx.run_until_parked();
            let remote_project = remote_project.borrow_mut().take().unwrap();
            remote_project
                .read_with(cx, |project, _| project.remote_client().unwrap())
                .update(cx, |client, cx| client.force_server_not_running(cx));
            drop(remote_project);
            drop((barrier, server_session));
            cx.run_until_parked();
            assert!(!cx.update(|cx| remote::has_active_connection(&options, cx)));
            assert_eq!(cx.pending_prompt(), None);
            if close_window {
                assert!(opened.is_err());
                assert!(cx.windows().is_empty());
                assert_eq!(persisted_state(), saved);
                continue;
            }
            assert_eq!(opened.unwrap(), window);
            window
                .update(cx, |multi_workspace, window, cx| {
                    assert_eq!(multi_workspace.workspace(), &initial_workspace);
                    assert!(multi_workspace.workspaces().eq([&initial_workspace]));
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .unwrap()
                .await
                .unwrap();
            assert_eq!(
                cx.read(|cx| {
                    unloaded_remote_workspace_candidates(
                        &root_path,
                        &OpenOptions::default(),
                        &options,
                        cx,
                    )
                    .unwrap()
                }),
                vec![(window, remote_id)],
            );
            assert_eq!(persisted_state(), saved);
            window
                .update(cx, |_, window, _| window.remove_window())
                .unwrap();
            cx.run_until_parked();
        }
    }

    #[gpui::test]
    async fn test_lazy_remote_match_exact_tries_remaining_candidates(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
            HeadlessProject::init(cx);
        });
        let remote_fs = FakeFs::new(server_cx.executor());
        remote_fs
            .insert_tree(
                path!("/project"),
                json!({"src": {"main.rs": "fn main() {}", "main.rs:2:4": "literal file"}}),
            )
            .await;
        let (options, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);
        let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
        drop(connect_guard);
        let parent_id = save_remote_workspace(&options, &[path!("/project")], &app_state, cx).await;
        drop(headless);
        cx.run_until_parked();

        let (server_session, connect_guard) =
            RemoteClient::fake_server_with_opts(&options, cx, server_cx);
        let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
        drop(connect_guard);
        let child_id =
            save_remote_workspace(&options, &[path!("/project/src")], &app_state, cx).await;
        drop(headless);
        cx.run_until_parked();

        for cancel in [true, false] {
            let window =
                restore_unloaded_remote_workspaces(&[parent_id, child_id], &app_state, cx).await;
            let (server_session, connect_guard) =
                RemoteClient::fake_server_with_opts(&options, cx, server_cx);
            let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
            let opening = spawn_remote_open(
                &options,
                &[path!("/project/src/main.rs:1:4")],
                &app_state,
                OpenOptions::default(),
                cx,
            );
            cx.run_until_parked();
            cx.update(|cx| {
                for id in [parent_id, child_id] {
                    assert!(WorkspaceIdReservation::new(id, Some(window), cx).is_none());
                }
            });
            if cancel {
                drop(opening);
                cx.run_until_parked();
            } else {
                drop(connect_guard);
                assert_eq!(opening.await.unwrap(), window);
                window
                    .read_with(cx, |multi_workspace, cx| {
                        assert_eq!(
                            multi_workspace.workspace().read(cx).database_id(),
                            Some(parent_id)
                        );
                        assert_eq!(
                            unloaded_workspace_candidates(
                                &[PathBuf::from(path!("/project/src"))],
                                &OpenOptions::default(),
                                &SerializedWorkspaceLocation::Remote(options.clone()),
                                cx,
                            )
                            .unwrap(),
                            vec![(window, child_id)],
                        );
                    })
                    .unwrap();
                window
                    .update(cx, |multi_workspace, _, cx| {
                        let editor = multi_workspace
                            .workspace()
                            .read(cx)
                            .active_item(cx)
                            .unwrap()
                            .downcast::<Editor>()
                            .unwrap();
                        editor.update(cx, |editor, cx| {
                            assert_eq!(
                                editor
                                    .selections
                                    .newest::<Point>(&editor.display_snapshot(cx))
                                    .head(),
                                Point::new(0, 3)
                            );
                        });
                    })
                    .unwrap();
            }
            cx.update(|cx| {
                assert!(cx.global::<RemoteWorkspaceLoads>().0.borrow().is_empty());
                assert!(WorkspaceIdReservation::new(child_id, Some(window), cx).is_some());
                if cancel {
                    assert!(WorkspaceIdReservation::new(parent_id, Some(window), cx).is_some());
                }
            });
            if cancel {
                window
                    .update(cx, |_, window, _| window.remove_window())
                    .unwrap();
            } else {
                close_remote_window(window, cx);
            }
            drop(headless);
            cx.run_until_parked();
        }

        let window =
            restore_unloaded_remote_workspaces(&[child_id, parent_id], &app_state, cx).await;
        let (server_session, connect_guard) =
            RemoteClient::fake_server_with_opts(&options, cx, server_cx);
        let _parent_headless =
            create_headless_project(server_session, remote_fs.clone(), server_cx);
        let (parent_load, _) = cx.update(|cx| {
            load_remote_workspace(
                parent_id,
                options.clone(),
                app_state.clone(),
                OpenOptions {
                    workspace_id: Some(parent_id),
                    requesting_window: Some(window),
                    focus: Some(false),
                    ..OpenOptions::default()
                },
                cx,
            )
            .unwrap()
        });
        let mut async_cx = cx.to_async();
        let parent_load = async move {
            let opened = parent_load.await?;
            let workspace = opened.workspace.as_ref().unwrap();
            workspace
                .read_with(&async_cx, |workspace, cx| {
                    workspace.project().read(cx).wait_for_initial_scan(cx)
                })
                .await;
            workspace.update(&mut async_cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .remote_client()
                    .unwrap()
                    .update(cx, |client, cx| client.force_server_not_running(cx));
            });
            Ok(opened)
        }
        .boxed_local()
        .shared();
        cx.update(|cx| {
            cx.global::<RemoteWorkspaceLoads>()
                .0
                .borrow_mut()
                .get_mut(&parent_id)
                .unwrap()
                .task = parent_load.downgrade().unwrap();
        });
        let opening = spawn_remote_open(
            &options,
            &[path!("/project/src/main.rs:2:4")],
            &app_state,
            OpenOptions::default(),
            cx,
        );
        cx.run_until_parked();
        let (server_session, child_connect_guard) =
            RemoteClient::fake_server_with_opts(&options, cx, server_cx);
        let _child_headless = create_headless_project(server_session, remote_fs, server_cx);
        drop(child_connect_guard);
        drop(connect_guard);
        let opened = opening
            .with_timeout(Duration::from_secs(10), &cx.executor())
            .await
            .expect("remote candidate search timed out")
            .unwrap();
        assert_eq!(opened, window);
        assert_eq!(cx.windows().len(), 1);
        assert_eq!(cx.pending_prompt(), None);
        window
            .read_with(cx, |multi_workspace, cx| {
                assert_eq!(
                    multi_workspace.workspace().read(cx).database_id(),
                    Some(child_id)
                );
                let restored_ids = multi_workspace
                    .workspaces()
                    .filter(|workspace| workspace.read(cx).project().read(cx).is_remote())
                    .map(|workspace| workspace.read(cx).database_id().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(restored_ids, vec![parent_id, child_id]);
            })
            .unwrap();
        window
            .update(cx, |multi_workspace, _, cx| {
                let editor = multi_workspace
                    .workspace()
                    .read(cx)
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap();
                editor.update(cx, |editor, cx| {
                    assert_eq!(editor.text(cx), "literal file");
                    assert_eq!(
                        editor
                            .selections
                            .newest::<Point>(&editor.display_snapshot(cx))
                            .head(),
                        Point::new(0, 0)
                    );
                });
            })
            .unwrap();
        close_remote_window(window, cx);
    }

    #[gpui::test]
    async fn test_remote_connection_prompt_waiters_recheck_after_dismissal(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        server_cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let window = cx.add_window(|window, cx| {
            let workspace =
                cx.new(|cx| Workspace::new(None, project, app_state.clone(), window, cx));
            let mut multi_workspace = MultiWorkspace::new(workspace, window, cx);
            multi_workspace.retain_active_workspace(cx);
            multi_workspace
        });
        let mut openings = Vec::new();
        let mut guards = Vec::new();
        for _ in 0..4 {
            let (options, _server_session, guard) = RemoteClient::fake_server(cx, server_cx);
            guards.push(guard);
            openings.push(spawn_remote_open(
                &options,
                &[],
                &app_state,
                OpenOptions {
                    requesting_window: Some(window),
                    workspace_matching: WorkspaceMatching::None,
                    ..OpenOptions::default()
                },
                cx,
            ));
            cx.run_until_parked();
        }
        for completed in 0..4 {
            assert_eq!(cx.pending_prompt(), None, "completed: {completed}");
            assert_eq!(
                openings.iter().filter(|opening| opening.is_ready()).count(),
                completed
            );
            let prompt_workspace = window
                .read_with(cx, |multi_workspace, cx| {
                    assert!(
                        multi_workspace
                            .workspace()
                            .read(cx)
                            .active_modal::<RemoteConnectionModal>(cx)
                            .is_some(),
                        "completed: {completed}"
                    );
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            match completed {
                0 => {
                    let project = Project::test(app_state.fs.clone(), [], cx).await;
                    window
                        .update(cx, |multi_workspace, window, cx| {
                            let workspace = cx.new(|cx| {
                                Workspace::new(None, project, app_state.clone(), window, cx)
                            });
                            multi_workspace.activate(workspace, None, window, cx);
                        })
                        .unwrap();
                }
                1 => {
                    let removal = window
                        .update(cx, |multi_workspace, window, cx| {
                            multi_workspace.remove(
                                [prompt_workspace.clone()],
                                RemovalIntent::KeepProject,
                                window,
                                cx,
                            )
                        })
                        .unwrap();
                    assert!(removal.await.unwrap());
                }
                _ => cx.dispatch_action(gpui::AnyWindowHandle::from(window), menu::Cancel),
            }
            cx.run_until_parked();
            if completed < 2 {
                prompt_workspace.read_with(cx, |workspace, cx| {
                    assert!(
                        workspace
                            .active_modal::<RemoteConnectionModal>(cx)
                            .is_none()
                    );
                });
            }
        }
        for opening in openings {
            assert_eq!(
                opening
                    .with_timeout(Duration::from_secs(10), &cx.executor())
                    .await
                    .expect("remote prompt waiter timed out")
                    .unwrap(),
                window,
            );
        }
        assert_eq!(cx.pending_prompt(), None);
        window
            .update(cx, |multi_workspace, window, cx| {
                assert!(
                    multi_workspace
                        .workspace()
                        .read(cx)
                        .active_modal::<RemoteConnectionModal>(cx)
                        .is_none()
                );
                window.remove_window();
            })
            .unwrap();
        drop(guards);
    }

    #[gpui::test]
    async fn test_unloaded_remote_candidates_filter_connection_identity(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        let paths = vec![PathBuf::from(path!("/project"))];
        for kind in ["wsl", "docker"] {
            let saved_paths = PathList::new(&paths).serialize();
            let workspace_ids = db
                .write(move |connection| {
                    ["root", "alice"]
                        .into_iter()
                        .map(|user| {
                            let remote_id = connection.select_row_bound::<_, i64>(
                                "INSERT INTO remote_connections
                                 (kind, user, distro, container_id, name, use_podman, remote_env)
                                 VALUES (?, ?, 'Ubuntu', 'container', 'dev', 0, '{}') RETURNING id",
                            )?((kind, user))?
                            .context("missing remote connection ID")?;
                            connection.select_row_bound::<_, WorkspaceId>(
                                "INSERT INTO workspaces (paths, paths_order, remote_connection_id)
                                 VALUES (?, ?, ?) RETURNING workspace_id",
                            )?((
                                saved_paths.paths.clone(),
                                saved_paths.order.clone(),
                                remote_id,
                            ))?
                            .context("missing workspace ID")
                        })
                        .collect::<Result<Vec<_>>>()
                })
                .await
                .unwrap();
            let window = restore_unloaded_remote_workspaces(&workspace_ids, &app_state, cx).await;
            for (user, expected) in [
                ("root", vec![(window, workspace_ids[0])]),
                ("alice", vec![(window, workspace_ids[1])]),
                ("bob", Vec::new()),
            ] {
                let options = if kind == "wsl" {
                    RemoteConnectionOptions::Wsl(WslConnectionOptions {
                        distro_name: "Ubuntu".to_owned(),
                        user: Some(user.to_owned()),
                    })
                } else {
                    RemoteConnectionOptions::Docker(DockerConnectionOptions {
                        container_id: "container".to_owned(),
                        name: "dev".to_owned(),
                        remote_user: user.to_owned(),
                        upload_binary_over_docker_exec: false,
                        use_podman: false,
                        remote_env: BTreeMap::new(),
                    })
                };
                cx.update(|cx| {
                    assert_eq!(
                        unloaded_workspace_candidates(
                            &paths,
                            &OpenOptions::default(),
                            &SerializedWorkspaceLocation::Remote(options.clone()),
                            cx,
                        )
                        .unwrap(),
                        workspace_ids
                            .iter()
                            .map(|id| (window, *id))
                            .collect::<Vec<_>>()
                    );
                    assert_eq!(
                        unloaded_remote_workspace_candidates(
                            &paths,
                            &OpenOptions::default(),
                            &options,
                            cx,
                        )
                        .unwrap(),
                        expected,
                    );
                    for workspace_id in &workspace_ids {
                        assert!(
                            WorkspaceIdReservation::new(*workspace_id, Some(window), cx).is_some()
                        );
                    }
                });
                if let Some((_, workspace_id)) = expected.first() {
                    let reservation = cx.update(|cx| {
                        WorkspaceIdReservation::new(*workspace_id, Some(window), cx).unwrap()
                    });
                    let error = spawn_remote_open(
                        &options,
                        &[path!("/project")],
                        &app_state,
                        OpenOptions::default(),
                        cx,
                    )
                    .await
                    .unwrap_err();
                    assert_eq!(
                        error.to_string(),
                        format!("Workspace {workspace_id:?} is already open or restoring")
                    );
                    drop(reservation);
                }
            }
            window
                .update(cx, |_, window, _| window.remove_window())
                .unwrap();
            cx.run_until_parked();
        }
    }

    #[gpui::test]
    async fn test_reopen_remote_file_roots_preserves_requested_positions(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
            HeadlessProject::init(cx);
        });
        let (options, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);
        let remote_fs = FakeFs::new(server_cx.executor());
        let alpha = "alpha one\nalpha two\nalpha three\n";
        let beta = "beta one\nbeta two\nbeta three\n";
        let literal = "literal one\nliteral two\nliteral three\n";
        remote_fs
            .insert_tree(
                path!("/project"),
                json!({"a": {"a.txt": alpha, "a.txt:2:4": literal}, "b": {"b.txt": beta}}),
            )
            .await;
        let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
        drop(connect_guard);
        let mut roots = vec![
            PathBuf::from(path!("/project/a/a.txt")),
            PathBuf::from(path!("/project/b/b.txt")),
        ];
        let mut async_cx = cx.to_async();
        let window = open_remote_project(
            options.clone(),
            roots.clone(),
            app_state.clone(),
            OpenOptions::default(),
            &mut async_cx,
        )
        .await
        .unwrap();
        let literal_path = PathBuf::from(path!("/project/a/a.txt:2:4"));
        let opened = window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.open_paths(
                        vec![literal_path.clone()],
                        OpenOptions {
                            visible: Some(OpenVisible::All),
                            ..OpenOptions::default()
                        },
                        None,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await;
        assert_eq!(opened.len(), 1);
        assert!(opened.into_iter().next().unwrap().unwrap().is_ok());
        roots.push(literal_path.clone());
        roots.sort();
        let draft = window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    Editor::new_in_workspace(workspace, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        let (workspace_id, flush) = window
            .update(cx, |multi_workspace, window, cx| {
                draft.update(cx, |editor, cx| {
                    editor.insert("saved dirty draft", window, cx);
                });
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    (
                        workspace.database_id().unwrap(),
                        workspace.flush_serialization_checked(window, cx),
                    )
                })
            })
            .unwrap();
        flush.await.unwrap();
        cx.run_until_parked();
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        assert_eq!(
            db.session_workspace_for_id(workspace_id).unwrap().paths,
            PathList::new(&roots),
        );
        let editor_payload_ids = || {
            db.select_bound::<_, u64>(
                "SELECT item_id FROM editors WHERE workspace_id = ? ORDER BY item_id",
            )
            .unwrap()(workspace_id)
            .unwrap()
        };
        let editor_item_ids = || {
            db.select_bound::<_, u64>(
                "SELECT item_id FROM items WHERE workspace_id = ? AND kind = 'Editor' ORDER BY item_id",
            )
            .unwrap()(workspace_id)
            .unwrap()
        };
        let source_editor_ids = editor_payload_ids();
        assert_eq!(source_editor_ids.len(), 4);
        assert_eq!(source_editor_ids, editor_item_ids());
        close_remote_window(window, cx);
        drop(draft);
        drop(headless);
        cx.run_until_parked();

        let mut window = restore_unloaded_remote_workspaces(&[workspace_id], &app_state, cx).await;
        let (server_session, connect_guard) =
            RemoteClient::fake_server_with_opts(&options, cx, server_cx);
        let mut headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
        drop(connect_guard);
        let (resume_serialization, serialization_resumed) = oneshot::channel();
        let mut resume_serialization = Some(resume_serialization);
        let serialization_resumed = Rc::new(RefCell::new(Some(serialization_resumed)));
        let item_subscription = Rc::new(RefCell::new(None));
        let restore_observer = cx.update(|cx| {
            let serialization_resumed = serialization_resumed.clone();
            let item_subscription = item_subscription.clone();
            cx.observe_new::<Workspace>(move |workspace, _, cx| {
                if workspace.database_id() != Some(workspace_id) {
                    return;
                }
                let serialization_resumed = serialization_resumed.clone();
                *item_subscription.borrow_mut() =
                    Some(cx.subscribe_self(move |_, event: &workspace::Event, cx| {
                        let workspace::Event::ItemAdded { item } = event else {
                            return;
                        };
                        let Some(editor) = item.downcast::<Editor>() else {
                            return;
                        };
                        let Some(resumed) = serialization_resumed.borrow_mut().take() else {
                            return;
                        };
                        let persistence_id = item_persistence_id(&editor, cx);
                        let delayed_id = cx
                            .background_spawn(async move {
                                resumed.await.unwrap();
                                persistence_id.await
                            })
                            .shared();
                        set_item_persistence_id(&editor, delayed_id, cx);
                    }));
            })
        });
        for (alpha_row, beta_row) in [(2, 3), (3, 2)] {
            let reopened = open_remote_project(
                options.clone(),
                vec![
                    PathBuf::from(format!("{}:{beta_row}:2", path!("/project/b/b.txt"))),
                    PathBuf::from(format!("{}:{alpha_row}:5", path!("/project/a/a.txt"))),
                ],
                app_state.clone(),
                OpenOptions::default(),
                &mut async_cx,
            )
            .with_timeout(Duration::from_secs(10), &cx.executor())
            .await;
            assert_eq!(cx.pending_prompt(), None);
            assert_eq!(reopened.expect("remote reopen timed out").unwrap(), window);
            if let Some(resume) = resume_serialization.take() {
                assert!(serialization_resumed.borrow().is_none());
                cx.run_until_parked();
                assert_eq!(editor_item_ids(), source_editor_ids);
                assert_eq!(
                    editor_payload_ids()
                        .into_iter()
                        .filter(|item_id| source_editor_ids.binary_search(item_id).is_ok())
                        .collect::<Vec<_>>(),
                    source_editor_ids,
                );
                let next_flush = window
                    .update(cx, |multi_workspace, window, cx| {
                        multi_workspace.flush_restoration(window, cx)
                    })
                    .unwrap();
                cx.run_until_parked();
                assert!(!next_flush.is_ready());
                resume.send(()).unwrap();
                next_flush
                    .with_timeout(Duration::from_secs(10), &cx.executor())
                    .await
                    .expect("remote post-install flush timed out")
                    .unwrap();
                cx.run_until_parked();
                let restored_editor_ids = editor_item_ids();
                assert_eq!(restored_editor_ids.len(), source_editor_ids.len());
                assert_eq!(editor_payload_ids(), restored_editor_ids);
                assert_eq!(
                    source_editor_ids
                        .iter()
                        .copied()
                        .filter(|item_id| restored_editor_ids.binary_search(item_id).is_ok())
                        .collect::<Vec<_>>(),
                    Vec::<u64>::new(),
                );
            }
            assert_eq!(
                open_remote_project(
                    options.clone(),
                    vec![literal_path.clone()],
                    app_state.clone(),
                    OpenOptions::default(),
                    &mut async_cx
                )
                .await
                .unwrap(),
                window
            );
            assert_eq!(cx.windows().len(), 1);
            window
                .update(cx, |multi_workspace, _, cx| {
                    let workspace = multi_workspace.workspace().read(cx);
                    assert_eq!(workspace.database_id(), Some(workspace_id));
                    let mut restored_roots = workspace
                        .root_paths(cx)
                        .iter()
                        .map(|path| path.to_path_buf())
                        .collect::<Vec<_>>();
                    restored_roots.sort();
                    assert_eq!(restored_roots, roots);
                    let editors = workspace
                        .items(cx)
                        .filter(|item| item.project_path(cx).is_some())
                        .filter_map(|item| item.downcast::<Editor>())
                        .collect::<Vec<_>>();
                    let mut positions = editors
                        .into_iter()
                        .map(|editor| {
                            editor.update(cx, |editor, cx| {
                                (
                                    editor.text(cx),
                                    editor
                                        .selections
                                        .newest::<Point>(&editor.display_snapshot(cx))
                                        .head(),
                                )
                            })
                        })
                        .collect::<Vec<_>>();
                    positions.sort_by(|left, right| left.0.cmp(&right.0));
                    assert_eq!(
                        positions,
                        vec![
                            (alpha.to_owned(), Point::new(alpha_row - 1, 4)),
                            (beta.to_owned(), Point::new(beta_row - 1, 1)),
                            (literal.to_owned(), Point::new(0, 0)),
                        ]
                    );
                })
                .unwrap();
        }
        drop(restore_observer);
        drop(item_subscription);
        let workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let missing = PathBuf::from(path!("/project/missing.txt:2:4"));
        assert_eq!(
            resolve_remote_paths(&workspace, &[literal_path.clone(), missing], &mut async_cx)
                .await
                .unwrap(),
            vec![
                PathWithPosition::from_path(literal_path.clone()),
                PathWithPosition {
                    path: PathBuf::from(path!("/project/missing.txt")),
                    row: Some(2),
                    column: Some(4)
                },
            ],
        );
        workspace.update(cx, |workspace, cx| {
            workspace
                .project()
                .read(cx)
                .remote_client()
                .unwrap()
                .update(cx, |client, cx| client.force_server_not_running(cx));
        });
        cx.run_until_parked();
        let (_server_session, connect_guard) =
            RemoteClient::fake_server_with_opts(&options, cx, server_cx);
        let opening = spawn_remote_open(
            &options,
            &[path!("/project/a/a.txt:2:5")],
            &app_state,
            OpenOptions {
                open_mode: OpenMode::NewWindow,
                requesting_window: Some(window),
                ..OpenOptions::default()
            },
            cx,
        );
        cx.run_until_parked();
        assert_eq!(cx.windows().len(), 2);
        drop(opening);
        cx.run_until_parked();
        assert_eq!(cx.windows().len(), 1);
        drop(connect_guard);
        drop(workspace);

        for (missing_parent, expected_roots) in [
            (path!("/project/b"), vec![roots[0].clone(), literal_path]),
            (path!("/project/a"), Vec::new()),
        ] {
            close_remote_window(window, cx);
            drop(headless);
            cx.run_until_parked();
            remote_fs
                .remove_dir(
                    Path::new(missing_parent),
                    RemoveOptions {
                        recursive: true,
                        ..RemoveOptions::default()
                    },
                )
                .await
                .unwrap();
            let (server_session, connect_guard) =
                RemoteClient::fake_server_with_opts(&options, cx, server_cx);
            headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
            drop(connect_guard);
            let reopened = open_remote_project(
                options.clone(),
                roots.clone(),
                app_state.clone(),
                OpenOptions {
                    workspace_id: Some(workspace_id),
                    ..OpenOptions::default()
                },
                &mut async_cx,
            )
            .with_timeout(Duration::from_secs(10), &cx.executor())
            .await;
            assert_eq!(
                cx.pending_prompt(),
                None,
                "missing parent: {missing_parent}"
            );
            window = reopened.expect("remote reopen timed out").unwrap();
            window
                .read_with(cx, |multi_workspace, cx| {
                    let workspace = multi_workspace.workspace().read(cx);
                    assert_eq!(workspace.database_id(), Some(workspace_id));
                    assert_eq!(
                        workspace
                            .root_paths(cx)
                            .iter()
                            .map(|path| path.to_path_buf())
                            .collect::<Vec<_>>(),
                        expected_roots
                    );
                    assert_eq!(
                        workspace
                            .items(cx)
                            .filter(|item| item.project_path(cx).is_none())
                            .filter_map(|item| item.downcast::<Editor>())
                            .map(|editor| {
                                let editor = editor.read(cx);
                                (editor.text(cx), editor.is_dirty(cx))
                            })
                            .collect::<Vec<_>>(),
                        vec![("saved dirty draft".to_owned(), true)]
                    );
                })
                .unwrap();
        }
    }

    #[gpui::test]
    async fn test_transient_remote_root_failure_preserves_saved_roots(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
            HeadlessProject::init(cx);
        });
        let remote_fs = FakeFs::new(server_cx.executor());
        remote_fs
            .insert_tree(path!("/project"), json!({"a": {}, "b": {}}))
            .await;
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        for roots in [
            vec![path!("/project/a")],
            vec![path!("/project/a"), path!("/project/b")],
        ] {
            let expected_roots = roots.iter().map(PathBuf::from).collect::<Vec<_>>();
            let (options, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);
            let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
            drop(connect_guard);
            let workspace_id = save_remote_workspace(&options, &roots, &app_state, cx).await;
            drop(headless);
            cx.run_until_parked();
            let window = restore_unloaded_remote_workspaces(&[workspace_id], &app_state, cx).await;
            window
                .update(cx, |multi_workspace, window, cx| {
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .unwrap()
                .await
                .unwrap();
            let saved = db.session_workspace_for_id(workspace_id).unwrap();
            assert_eq!(saved.paths, PathList::new(&expected_roots));
            let mut plan = cx.update(WorkspaceRestorePlan::new).unwrap();
            plan.set_workspaces(
                &[SerializedMultiWorkspace {
                    active_workspace: saved.clone(),
                    workspace_ids: vec![workspace_id],
                    scratch_workspaces: Vec::new(),
                    state: MultiWorkspaceState::default(),
                }],
                app_state.session.clone(),
                &mut cx.to_async(),
            )
            .await
            .unwrap();
            drop(plan);

            let (server_session, connect_guard) =
                RemoteClient::fake_server_with_opts(&options, cx, server_cx);
            let requests = server_cx.new(|_| Vec::new());
            server_session.add_request_handler::<proto::Ping, _, _, _>(
                requests.downgrade(),
                |_, _, _| async { Ok(proto::Ack {}) },
            );
            server_session.add_request_handler::<proto::AddWorktree, _, _, _>(
                requests.downgrade(),
                {
                    let denied_path = roots.last().unwrap().to_string();
                    move |requests, message, mut cx| {
                        let denied = message.payload.path == denied_path;
                        async move {
                            requests.update(&mut cx, |requests, _| {
                                requests.push(PathBuf::from(&message.payload.path));
                            });
                            anyhow::ensure!(!denied, "permission denied opening remote root");
                            Ok(proto::AddWorktreeResponse {
                                worktree_id: 1,
                                canonicalized_path: message.payload.path,
                                root_repo_common_dir: None,
                                root_repo_is_linked_worktree: false,
                            })
                        }
                    }
                },
            );
            drop(connect_guard);
            let remote_client = RemoteClient::connect_mock(options.clone(), cx).await;
            let project = cx.update(|cx| {
                Project::remote(
                    remote_client.clone(),
                    app_state.client.clone(),
                    app_state.node_runtime.clone(),
                    app_state.user_store.clone(),
                    app_state.languages.clone(),
                    app_state.fs.clone(),
                    false,
                    cx,
                )
            });
            let result = workspace::open_remote_project_with_existing_connection(
                options.clone(),
                project.clone(),
                expected_roots.clone(),
                app_state.clone(),
                window,
                None,
                None,
                &mut cx.to_async(),
            )
            .with_timeout(Duration::from_secs(10), &cx.executor())
            .await
            .expect("remote root failure timed out");
            cx.run_until_parked();
            assert_eq!(db.session_workspace_for_id(workspace_id).unwrap(), saved);
            assert_eq!(
                db.unfinished_workspace_restores()
                    .unwrap()
                    .into_iter()
                    .map(|workspace| workspace.active_workspace)
                    .collect::<Vec<_>>(),
                vec![saved]
            );
            assert_eq!(
                result
                    .err()
                    .expect("transient root failure must abort restoration")
                    .root_cause()
                    .to_string(),
                "RPC request AddWorktree failed: permission denied opening remote root"
            );
            assert_eq!(
                requests.read_with(server_cx, |requests, _| requests.clone()),
                expected_roots
            );
            cx.read(|cx| {
                assert_eq!(
                    remote_client.read(cx).connection_state(),
                    ConnectionState::Connected
                );
                assert_eq!(
                    project
                        .read(cx)
                        .visible_worktrees(cx)
                        .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
                        .collect::<Vec<_>>(),
                    expected_roots[..expected_roots.len() - 1]
                );
            });
            remote_client
                .read_with(cx, |client, _| client.proto_client())
                .request(proto::Ping {})
                .await
                .unwrap();
            remote_client.update(cx, |client, cx| client.force_server_not_running(cx));
            drop((project, remote_client, requests, server_session));
            cx.run_until_parked();

            let (server_session, connect_guard) =
                RemoteClient::fake_server_with_opts(&options, cx, server_cx);
            let headless = create_headless_project(server_session, remote_fs.clone(), server_cx);
            drop(connect_guard);
            let reopened = spawn_remote_open(
                &options,
                &[],
                &app_state,
                OpenOptions {
                    workspace_id: Some(workspace_id),
                    requesting_window: Some(window),
                    ..OpenOptions::default()
                },
                cx,
            )
            .with_timeout(Duration::from_secs(10), &cx.executor())
            .await
            .expect("remote root retry timed out")
            .unwrap();
            assert_eq!(reopened, window);
            window
                .read_with(cx, |multi_workspace, cx| {
                    let workspace = multi_workspace.workspace().read(cx);
                    assert_eq!(workspace.database_id(), Some(workspace_id));
                    assert_eq!(
                        workspace
                            .root_paths(cx)
                            .iter()
                            .map(|path| path.to_path_buf())
                            .collect::<Vec<_>>(),
                        expected_roots
                    );
                })
                .unwrap();
            assert_eq!(
                db.session_workspace_for_id(workspace_id).unwrap().paths,
                PathList::new(&expected_roots)
            );
            close_remote_window(window, cx);
            drop(headless);
            cx.run_until_parked();
        }
    }

    async fn save_remote_workspace(
        connection_options: &RemoteConnectionOptions,
        paths: &[&str],
        app_state: &Arc<AppState>,
        cx: &mut TestAppContext,
    ) -> WorkspaceId {
        let window = spawn_remote_open(
            connection_options,
            paths,
            app_state,
            OpenOptions::default(),
            cx,
        )
        .await
        .unwrap();
        let (workspace_id, flush) = window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    (
                        workspace.database_id().unwrap(),
                        workspace.flush_serialization_checked(window, cx),
                    )
                })
            })
            .unwrap();
        flush.await.unwrap();
        close_remote_window(window, cx);
        workspace_id
    }

    async fn restore_unloaded_remote_workspaces(
        workspace_ids: &[WorkspaceId],
        app_state: &Arc<AppState>,
        cx: &mut TestAppContext,
    ) -> WindowHandle<MultiWorkspace> {
        let local_fs = FakeFs::new(cx.executor());
        local_fs
            .insert_tree(path!("/local"), json!({"local.txt": "local content"}))
            .await;
        let local_project = Project::test(local_fs.clone(), [path!("/local").as_ref()], cx).await;
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        let local_id = db.next_id().await.unwrap();
        let window = cx.add_window(|window, cx| {
            let workspace = cx.new(|cx| {
                Workspace::new(Some(local_id), local_project, app_state.clone(), window, cx)
            });
            let mut multi_workspace = MultiWorkspace::new(workspace, window, cx);
            multi_workspace.retain_active_workspace(cx);
            multi_workspace
        });
        let project_groups = workspace_ids
            .iter()
            .map(|workspace_id| {
                let saved = db.session_workspace_for_id(*workspace_id).unwrap();
                let SerializedWorkspaceLocation::Remote(connection_options) = saved.location else {
                    panic!("expected a remote workspace");
                };
                SerializedProjectGroup::from_group(
                    &ProjectGroupKey::new(Some(connection_options), saved.paths),
                    true,
                )
            })
            .collect();
        apply_restored_multiworkspace_state(
            window,
            &SerializedMultiWorkspace {
                active_workspace: SessionWorkspace {
                    workspace_id: local_id,
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::new(&[PathBuf::from(path!("/local"))]),
                    window_id: Some(window.window_id()),
                },
                workspace_ids: std::iter::once(local_id)
                    .chain(workspace_ids.iter().copied())
                    .collect(),
                scratch_workspaces: Vec::new(),
                state: MultiWorkspaceState {
                    active_workspace_id: Some(local_id),
                    project_groups,
                    ..MultiWorkspaceState::default()
                },
            },
            local_fs,
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        window
    }

    fn spawn_remote_open(
        connection_options: &RemoteConnectionOptions,
        paths: &[&str],
        app_state: &Arc<AppState>,
        open_options: OpenOptions,
        cx: &mut TestAppContext,
    ) -> Task<Result<WindowHandle<MultiWorkspace>>> {
        let connection_options = connection_options.clone();
        let paths = paths.iter().map(|path| PathBuf::from(*path)).collect();
        let app_state = app_state.clone();
        cx.spawn(async move |mut cx| {
            open_remote_project(connection_options, paths, app_state, open_options, &mut cx).await
        })
    }

    fn close_remote_window(window: WindowHandle<MultiWorkspace>, cx: &mut TestAppContext) {
        window
            .update(cx, |multi_workspace, window, cx| {
                let client = multi_workspace
                    .workspace()
                    .read(cx)
                    .project()
                    .read(cx)
                    .remote_client()
                    .unwrap();
                client.update(cx, |client, cx| client.force_server_not_running(cx));
                window.remove_window();
            })
            .unwrap();
    }

    fn create_headless_project(
        session: AnyProtoClient,
        fs: Arc<FakeFs>,
        cx: &mut TestAppContext,
    ) -> Entity<HeadlessProject> {
        let languages = Arc::new(language::LanguageRegistry::new(cx.executor()));
        cx.new(|cx| {
            HeadlessProject::new(
                HeadlessAppState {
                    session,
                    fs,
                    http_client: Arc::new(BlockedHttpClient),
                    node_runtime: NodeRuntime::unavailable(),
                    languages,
                    extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                    startup_time: std::time::Instant::now(),
                },
                false,
                cx,
            )
        })
    }
}

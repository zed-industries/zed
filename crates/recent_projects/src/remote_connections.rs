use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result};
use editor::Editor;
use futures::{FutureExt as _, channel::oneshot, future::LocalBoxFuture, select};
use gpui::{App, AppContext, AsyncApp, Entity, PromptLevel, Task, WindowHandle};

use project::trusted_worktrees;
use remote::{
    DockerConnectionOptions, Interactive, RemoteConnection, RemoteConnectionOptions,
    SshConnectionOptions, same_remote_connection_target,
};
pub use settings::SshConnection;
use settings::{DevContainerConnection, ExtendingVec, RegisterSetting, Settings, WslConnection};
use util::{ResultExt as _, paths::PathWithPosition};
use workspace::{
    AppState, MultiWorkspace, MultiWorkspaceEvent, OpenOptions, RestoredRemoteWorkspace,
    SerializedWorkspaceLocation, Workspace, WorkspaceDb, WorkspaceMatching,
    find_existing_workspace,
};

pub use remote_connection::{
    KnownPassword, RemoteClientDelegate, RemoteConnectionModal, RemoteConnectionPrompt,
    SshConnectionHeader, connect,
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
        let host = options.host.to_string();
        let mut same_host = self
            .ssh_connections()
            .filter(|conn| {
                conn.host == host && conn.username == options.username && conn.port == options.port
            })
            .collect::<Vec<_>>();
        let conn = match options.args.as_deref() {
            None => same_host.into_iter().next(),
            Some(args) => match same_host.iter().position(|conn| conn.args == args) {
                Some(index) => Some(same_host.swap_remove(index)),
                None if same_host.len() == 1 => same_host.pop(),
                None => None,
            },
        };
        if let Some(conn) = conn {
            options.nickname = conn.nickname;
            options.upload_binary_over_ssh = conn.upload_binary_over_ssh.unwrap_or_default();
            options.args = Some(conn.args);
            options.port_forwards = conn.port_forwards;
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
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    open_options: workspace::OpenOptions,
    cx: &mut AsyncApp,
) -> Result<WindowHandle<MultiWorkspace>> {
    let (window, connected) = open_remote_project_inner(
        connection_options,
        paths,
        app_state,
        open_options,
        None,
        None,
        None,
        cx,
    )
    .await?;
    if connected {
        cx.update(|cx| workspace::focus_requested_window(window.into(), cx));
    }
    Ok(window)
}

pub fn restore_remote_project(
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    reuse_existing_windows: bool,
    connect_after: impl Future<Output = ()> + 'static,
    restored: RestoredRemoteWorkspace,
    workspace_created: oneshot::Sender<()>,
    known_password: KnownPassword,
    cx: &mut App,
) -> (
    oneshot::Receiver<WindowHandle<MultiWorkspace>>,
    Task<Result<(WindowHandle<MultiWorkspace>, bool)>>,
) {
    let (window_ready, window_ready_rx) = oneshot::channel();
    let task = cx.spawn(async move |cx| {
        let mut open_options = OpenOptions::default();
        if !reuse_existing_windows {
            open_options.workspace_matching = WorkspaceMatching::None;
        }
        open_remote_project_inner(
            connection_options,
            paths,
            app_state,
            open_options,
            Some(window_ready),
            Some(connect_after.boxed_local()),
            Some(SessionRestore {
                restored,
                workspace_created,
                known_password,
            }),
            cx,
        )
        .await
    });
    (window_ready_rx, task)
}

struct SessionRestore {
    restored: RestoredRemoteWorkspace,
    workspace_created: oneshot::Sender<()>,
    known_password: KnownPassword,
}

async fn open_remote_project_inner(
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    open_options: workspace::OpenOptions,
    mut window_ready: Option<oneshot::Sender<WindowHandle<MultiWorkspace>>>,
    connect_after: Option<LocalBoxFuture<'static, ()>>,
    session_restore: Option<SessionRestore>,
    cx: &mut AsyncApp,
) -> Result<(WindowHandle<MultiWorkspace>, bool)> {
    let location = SerializedWorkspaceLocation::Remote(connection_options.clone());
    let (mut existing, mut open_visible) =
        find_existing_workspace(&paths, &open_options, &location, cx).await;
    let mut matched_paths_without_positions = None;
    if existing.is_none()
        && let Some(paths_without_positions) = paths_without_positions(&paths)
    {
        (existing, open_visible) =
            find_existing_workspace(&paths_without_positions, &open_options, &location, cx).await;
        matched_paths_without_positions = Some(paths_without_positions);
    }

    if let Some((existing_window, existing_workspace)) = existing {
        let remote_connection = cx.update(|cx| {
            existing_workspace
                .read(cx)
                .project()
                .read(cx)
                .remote_client()
                .and_then(|client| client.read(cx).remote_connection())
        });

        if let Some(remote_connection) = remote_connection {
            let (resolved_paths, paths_with_positions) =
                determine_paths_with_positions(&remote_connection, paths.clone()).await;
            let reuse_existing = matched_paths_without_positions.is_none_or(|matched_paths| {
                matched_paths == resolved_paths
                    && same_remote_connection_target(
                        &remote_connection.connection_options(),
                        &connection_options,
                    )
            });
            if !reuse_existing {
                return open_remote_project_in_new_connection(
                    connection_options,
                    paths,
                    app_state,
                    open_options,
                    window_ready,
                    connect_after,
                    session_restore,
                    cx,
                )
                .await;
            }
            if let Some(window_ready) = window_ready.take() {
                window_ready.send(existing_window).ok();
            }
            if let Some(session_restore) = session_restore {
                session_restore.workspace_created.send(()).ok();
                let restored = session_restore.restored;
                let db = cx.update(|cx| WorkspaceDb::global(cx));
                db.release_restored_workspace(restored.workspace_id, restored.original, None)
                    .await
                    .log_err();
            }
            let restoration = cx.update(|cx| existing_workspace.read(cx).pending_restoration());
            if let Some(restoration) = restoration {
                workspace::wait_for_restoration(existing_window, restoration, cx).await?;
                return Box::pin(open_remote_project_inner(
                    connection_options,
                    paths,
                    app_state,
                    open_options,
                    None,
                    connect_after,
                    None,
                    cx,
                ))
                .await;
            }
            let open_results = existing_window
                .update(cx, |multi_workspace, window, cx| {
                    window.activate_window();
                    multi_workspace.activate(existing_workspace.clone(), None, window, cx);
                    existing_workspace.update(cx, |workspace, cx| {
                        workspace.open_paths(
                            resolved_paths,
                            OpenOptions {
                                visible: Some(open_visible),
                                ..Default::default()
                            },
                            None,
                            window,
                            cx,
                        )
                    })
                })?
                .await;

            _ = existing_window.update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    for item in open_results.iter().flatten() {
                        if let Err(e) = item {
                            workspace.show_error(format!("{e}"), cx);
                        }
                    }
                });
            });

            let items = open_results
                .into_iter()
                .map(|r| r.and_then(|r| r.ok()))
                .collect::<Vec<_>>();
            navigate_to_positions(&existing_window, items, &paths_with_positions, cx);

            return Ok((existing_window, true));
        }
        // If the remote connection is dead (e.g. server not running after failed reconnect),
        // fall through to establish a fresh connection instead of showing an error.
        log::info!(
            "existing remote workspace found but connection is dead, starting fresh connection"
        );
    }

    open_remote_project_in_new_connection(
        connection_options,
        paths,
        app_state,
        open_options,
        window_ready,
        connect_after,
        session_restore,
        cx,
    )
    .await
}

fn paths_without_positions(paths: &[PathBuf]) -> Option<Vec<PathBuf>> {
    let mut has_position = false;
    let paths = paths
        .iter()
        .map(|path| {
            match path
                .to_str()
                .map(PathWithPosition::parse_str)
                .filter(|parsed_path| parsed_path.row.is_some())
            {
                Some(parsed_path) => {
                    has_position = true;
                    parsed_path.path
                }
                None => path.clone(),
            }
        })
        .collect::<Vec<_>>();
    has_position.then_some(paths)
}

async fn open_remote_project_in_new_connection(
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    open_options: workspace::OpenOptions,
    window_ready: Option<oneshot::Sender<WindowHandle<MultiWorkspace>>>,
    mut connect_after: Option<LocalBoxFuture<'static, ()>>,
    session_restore: Option<SessionRestore>,
    cx: &mut AsyncApp,
) -> Result<(WindowHandle<MultiWorkspace>, bool)> {
    let restoring_session = session_restore.is_some();
    let (restored, workspace_created, known_password) = match session_restore {
        Some(SessionRestore {
            restored,
            workspace_created,
            known_password,
        }) => (Some(restored), Some(workspace_created), known_password),
        None => (
            None,
            None,
            KnownPassword::for_connection(&connection_options),
        ),
    };
    let restored_workspace_id = restored.as_ref().map(|restored| restored.workspace_id);
    let created_new_window = open_options.requesting_window.is_none();
    let activate_window = window_ready.is_none();
    let (window, initial_workspace) = if let Some(window) = open_options.requesting_window {
        let workspace = window.update(cx, |multi_workspace, _, _| {
            multi_workspace.workspace().clone()
        })?;
        (window, workspace)
    } else {
        let workspace_position = cx
            .update(|cx| {
                workspace::remote_workspace_position_from_db(connection_options.clone(), &paths, cx)
            })
            .await
            .context("fetching remote workspace position from db")?;

        let mut options =
            cx.update(|cx| (app_state.build_window_options)(workspace_position.display, cx));
        options.window_bounds = workspace_position.window_bounds;

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
                workspace.set_remote_connection_placeholder(true);
                if let Some(restored) = restored {
                    workspace.adopt_restored_remote_workspace(
                        restored,
                        window.window_handle().window_id(),
                        cx,
                    );
                }
                workspace
            });
            cx.new(|cx| MultiWorkspace::new(workspace, window, cx))
        })?;
        let workspace = window.update(cx, |multi_workspace, _, _cx| {
            multi_workspace.workspace().clone()
        })?;
        (window, workspace)
    };
    if let Some(window_ready) = window_ready {
        window_ready.send(window).ok();
    }
    let _notify_workspace_created = workspace_created
        .map(|workspace_created| {
            let multi_workspace = window.entity(cx)?;
            let initial_workspace = initial_workspace.clone();
            let mut workspace_created = Some(workspace_created);
            anyhow::Ok(cx.update(|cx| {
                cx.subscribe(
                    &multi_workspace,
                    move |multi_workspace, event: &MultiWorkspaceEvent, cx| {
                        if let MultiWorkspaceEvent::ActiveWorkspaceChanged { .. } = event
                            && multi_workspace.read(cx).workspace() != &initial_workspace
                            && let Some(workspace_created) = workspace_created.take()
                        {
                            workspace_created.send(()).ok();
                        }
                    },
                )
            }))
        })
        .transpose()?;

    let mut connected = false;
    loop {
        let (cancel_tx, mut cancel_rx) = oneshot::channel();
        let delegate = window.update(cx, {
            let paths = paths.clone();
            let connection_options = connection_options.clone();
            let initial_workspace = initial_workspace.clone();
            let known_password = known_password.clone();
            move |_multi_workspace: &mut MultiWorkspace, window, cx| {
                if activate_window {
                    window.activate_window();
                }
                initial_workspace.update(cx, |workspace, cx| {
                    workspace.hide_modal(window, cx);
                    workspace.toggle_modal(window, cx, |window, cx| {
                        RemoteConnectionModal::new(&connection_options, paths, window, cx)
                    });

                    let ui = workspace
                        .active_modal::<RemoteConnectionModal>(cx)?
                        .read(cx)
                        .prompt
                        .clone();

                    ui.update(cx, |ui, _cx| {
                        ui.set_cancellation_tx(cancel_tx);
                    });

                    let delegate = Arc::new(RemoteClientDelegate::new(
                        window.window_handle(),
                        ui.downgrade(),
                        known_password,
                    ));
                    Some((delegate, ui.downgrade()))
                })
            }
        })?;

        let Some((delegate, prompt)) = delegate else {
            break;
        };
        let window_id = window.window_id();
        let _cancel_on_window_close = cx.update(|cx| {
            cx.on_window_closed(move |cx, closed_window_id| {
                if closed_window_id == window_id {
                    prompt.update(cx, |prompt, _| prompt.cancel()).ok();
                }
            })
        });

        if let Some(connect_after) = connect_after.take() {
            remote::RemoteClientDelegate::set_status(
                delegate.as_ref(),
                Some("Waiting for existing connection attempt"),
                cx,
            );
            let cancelled = select! {
                _ = cancel_rx => true,
                _ = connect_after.fuse() => false,
            };
            if cancelled {
                finish_connection_modal(&initial_workspace, cx);
                break;
            }
            if window.update(cx, |_, _, _| ()).is_err() {
                return Ok((window, false));
            }
            remote::RemoteClientDelegate::set_status(delegate.as_ref(), None, cx);
        }

        let connection = remote::connect(connection_options.clone(), delegate.clone(), cx);
        let connection = select! {
            _ = cancel_rx => {
                finish_connection_modal(&initial_workspace, cx);

                break;
            },
            result = connection.fuse() => result,
        };
        let remote_connection = match connection {
            Ok(connection) => connection,
            Err(e) => {
                finish_connection_modal(&initial_workspace, cx);
                log::error!("Failed to open project: {e:#}");
                let response = window
                    .update(cx, |_, window, cx| {
                        window.request_attention();
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
                    continue;
                }

                release_placeholder_session_workspace(window, &initial_workspace, cx).await;
                if created_new_window {
                    window
                        .update(cx, |_, window, _| window.remove_window())
                        .ok();
                }
                return Ok((window, false));
            }
        };

        let (paths, paths_with_positions) =
            determine_paths_with_positions(&remote_connection, paths.clone()).await;

        let opened = cx
            .update(|cx| {
                workspace::open_remote_project_with_new_connection(
                    window,
                    remote_connection,
                    cancel_rx,
                    delegate.clone(),
                    app_state.clone(),
                    paths.clone(),
                    restoring_session,
                    restored_workspace_id,
                    cx,
                )
            })
            .await;

        finish_connection_modal(&initial_workspace, cx);

        match opened {
            Err(e) => {
                log::error!("Failed to open project: {e:#}");
                let response = window
                    .update(cx, |_, window, cx| {
                        window.request_attention();
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
                    continue;
                }

                release_placeholder_session_workspace(window, &initial_workspace, cx).await;
                if created_new_window {
                    window
                        .update(cx, |_, window, _| window.remove_window())
                        .ok();
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
                connected = workspace.is_some();
                navigate_to_positions(&window, items, &paths_with_positions, cx);
            }
        }

        break;
    }

    if connected {
        initial_workspace.update(cx, |workspace, _| {
            workspace.forget_placeholder_session();
        });
    } else {
        release_placeholder_session_workspace(window, &initial_workspace, cx).await;
    }
    if created_new_window && !connected {
        initial_workspace.update(cx, |workspace, _| {
            workspace.set_remote_connection_placeholder(false);
        });
    }

    Ok((window, connected))
}

async fn release_placeholder_session_workspace(
    window: WindowHandle<MultiWorkspace>,
    workspace: &Entity<Workspace>,
    cx: &mut AsyncApp,
) {
    let release = window.update(cx, |_, _, cx| {
        workspace.update(cx, |workspace, cx| {
            workspace.release_placeholder_session_workspace(cx)
        })
    });
    if let Ok(release) = release {
        release.await;
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
        if let Some(path_str) = path.to_str() {
            let path_with_position = PathWithPosition::parse_str(&path_str);
            if path_with_position.row.is_some() {
                if !path_exists(&remote_connection, &path).await {
                    *path = path_with_position.path.clone();
                    paths_with_positions.push(path_with_position);
                    continue;
                }
            }
        }
        paths_with_positions.push(PathWithPosition::from_path(path.clone()))
    }
    (paths, paths_with_positions)
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

fn finish_connection_modal(workspace: &Entity<Workspace>, cx: &mut AsyncApp) {
    workspace.update(cx, |workspace, cx| {
        if let Some(ui) = workspace.active_modal::<RemoteConnectionModal>(cx) {
            ui.update(cx, |modal, cx| modal.finished(cx))
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use extension::ExtensionHostProxy;
    use fs::FakeFs;
    use futures::future;
    use gpui::{AppContext, Empty, Entity, TestAppContext};
    use http_client::BlockedHttpClient;
    use node_runtime::NodeRuntime;
    use project::Project;
    use remote::RemoteClient;
    use remote_server::{HeadlessAppState, HeadlessProject};
    use rpc::proto;
    use serde_json::json;
    use util::path;
    use workspace::{
        find_existing_workspace,
        item::test::{PendingTestItemDeserialization, TestItem},
    };

    #[test]
    fn test_fill_connection_options_keeps_the_saved_ssh_target() {
        let target_args = |target: &str| vec!["-o".to_string(), format!("Hostname={target}")];
        let entry = |target: &str| SshConnection {
            host: "host".to_string(),
            args: target_args(target),
            nickname: Some(target.to_string()),
            ..SshConnection::default()
        };
        let settings = RemoteSettings {
            ssh_connections: ExtendingVec::from(vec![entry("host-a"), entry("host-b")]),
            wsl_connections: ExtendingVec::from(Vec::new()),
            read_ssh_config: false,
        };
        let filled_nickname = |args: Option<Vec<String>>| {
            let mut options = SshConnectionOptions {
                host: "host".into(),
                args,
                ..SshConnectionOptions::default()
            };
            settings.fill_connection_options_from_settings(&mut options);
            options.nickname
        };

        assert_eq!(
            [
                filled_nickname(None),
                filled_nickname(Some(target_args("host-b"))),
                filled_nickname(Some(target_args("removed"))),
            ],
            [Some("host-a".to_string()), Some("host-b".to_string()), None]
        );
    }

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
        let http_client = Arc::new(BlockedHttpClient);
        let node_runtime = NodeRuntime::unavailable();
        let languages = Arc::new(language::LanguageRegistry::new(server_cx.executor()));
        let proxy = Arc::new(ExtensionHostProxy::new());

        let _headless = server_cx.new(|cx| {
            HeadlessProject::new(
                HeadlessAppState {
                    session: server_session,
                    fs: remote_fs.clone(),
                    http_client,
                    node_runtime,
                    languages,
                    extension_host_proxy: proxy,
                    startup_time: std::time::Instant::now(),
                },
                false,
                cx,
            )
        });

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
        let http_client = Arc::new(BlockedHttpClient);
        let node_runtime = NodeRuntime::unavailable();
        let languages = Arc::new(language::LanguageRegistry::new(server_cx.executor()));
        let proxy = Arc::new(ExtensionHostProxy::new());

        let _headless = server_cx.new(|cx| {
            HeadlessProject::new(
                HeadlessAppState {
                    session: server_session,
                    fs: remote_fs.clone(),
                    http_client,
                    node_runtime,
                    languages,
                    extension_host_proxy: proxy,
                    startup_time: std::time::Instant::now(),
                },
                false,
                cx,
            )
        });

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
        .await;
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
        .await;

        assert!(
            found.is_some(),
            "find_existing_workspace should locate the existing remote workspace"
        );
        let (found_window, found_workspace) = found.unwrap();
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
    async fn test_local_file_open_skips_connecting_remote_window(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
        server_cx.update(|cx| {
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
        let (opts, _server_session, _connect_guard) = RemoteClient::fake_server(cx, server_cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/local"), json!({ "file.txt": "local file" }))
            .await;

        let (window_ready, _restore) = cx.update(|cx| {
            restore_remote_project(
                opts,
                vec![PathBuf::from(path!("/project"))],
                app_state.clone(),
                false,
                future::ready(()),
                unbound_restoration(),
                oneshot::channel().0,
                KnownPassword::default(),
                cx,
            )
        });
        let remote_window = window_ready.await.unwrap();
        let opened = cx
            .update(|cx| {
                workspace::open_paths(
                    &[PathBuf::from(path!("/local/file.txt"))],
                    app_state,
                    OpenOptions::default(),
                    cx,
                )
            })
            .await
            .unwrap();

        assert_ne!(opened.window, remote_window);
        assert_eq!(cx.update(|cx| cx.windows().len()), 2);
    }

    #[gpui::test]
    async fn test_restore_reports_workspace_before_items_finish_loading(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        cx.update(workspace::register_serializable_item::<TestItem>);
        let project_path = paths::home_dir().join("remote-pending-items");
        let (opts, _headless) =
            start_headless_server(&project_path, json!({ "notes.txt": "" }), cx, server_cx).await;

        let mut async_cx = cx.to_async();
        let window = open_remote_project(
            opts.clone(),
            vec![project_path.clone()],
            app_state.clone(),
            workspace::OpenOptions::default(),
            &mut async_cx,
        )
        .await
        .unwrap();
        cx.run_until_parked();
        window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    let item = Box::new(cx.new(TestItem::new));
                    workspace.add_item_to_active_pane(item, None, true, window, cx);
                    workspace.flush_serialization(window, cx)
                })
            })
            .unwrap()
            .await;
        window
            .update(cx, |_, window, _| window.remove_window())
            .unwrap();
        cx.run_until_parked();

        let (unblock_deserialization, pending_deserialization) = oneshot::channel();
        cx.update(|cx| {
            cx.set_global(PendingTestItemDeserialization(
                pending_deserialization.shared(),
            ))
        });
        let (workspace_created, mut workspace_created_rx) = oneshot::channel();
        let (window_ready, restore) = cx.update(|cx| {
            restore_remote_project(
                opts,
                vec![project_path],
                app_state,
                false,
                future::ready(()),
                unbound_restoration(),
                workspace_created,
                KnownPassword::default(),
                cx,
            )
        });
        let restored_window = window_ready.await.unwrap();
        cx.run_until_parked();
        let restoring = restored_window
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .pending_restoration()
                    .is_some()
            })
            .unwrap();
        let created_while_restoring = workspace_created_rx.try_recv();

        drop(unblock_deserialization);
        let (_, connected) = restore.await.unwrap();
        assert_eq!(
            (restoring, created_while_restoring, connected),
            (true, Ok(Some(())), true)
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
        let http_client = Arc::new(BlockedHttpClient);
        let node_runtime = NodeRuntime::unavailable();
        let languages = Arc::new(language::LanguageRegistry::new(server_cx.executor()));
        let proxy = Arc::new(ExtensionHostProxy::new());

        let _headless = server_cx.new(|cx| {
            HeadlessProject::new(
                HeadlessAppState {
                    session: server_session,
                    fs: remote_fs.clone(),
                    http_client,
                    node_runtime,
                    languages,
                    extension_host_proxy: proxy,
                    startup_time: std::time::Instant::now(),
                },
                false,
                cx,
            )
        });

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
    async fn test_reopen_remote_root_reuses_window_with_position(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        let executor = cx.executor();
        let project_path = paths::home_dir().join("remote-position-root");
        let (opts, _headless) = start_headless_server(
            &project_path,
            json!({ "notes.txt": "one\ntwo\n" }),
            cx,
            server_cx,
        )
        .await;
        let file_path = project_path.join("notes.txt");

        let mut async_cx = cx.to_async();
        let first_window = open_remote_project(
            opts.clone(),
            vec![file_path.clone()],
            app_state.clone(),
            workspace::OpenOptions::default(),
            &mut async_cx,
        )
        .await
        .unwrap();
        executor.run_until_parked();

        let mut path_with_position = file_path.into_os_string();
        path_with_position.push(":2");
        let open = cx.spawn(async move |mut cx| {
            open_remote_project(
                opts,
                vec![PathBuf::from(path_with_position)],
                app_state,
                workspace::OpenOptions::default(),
                &mut cx,
            )
            .await
            .unwrap()
        });
        executor.run_until_parked();

        assert_eq!(
            (open.now_or_never(), cx.update(|cx| cx.windows().len())),
            (Some(first_window), 1)
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
        let http_client = Arc::new(BlockedHttpClient);
        let node_runtime = NodeRuntime::unavailable();
        let languages = Arc::new(language::LanguageRegistry::new(server_cx.executor()));
        let proxy = Arc::new(ExtensionHostProxy::new());

        let _headless = server_cx.new(|cx| {
            HeadlessProject::new(
                HeadlessAppState {
                    session: server_session,
                    fs: remote_fs.clone(),
                    http_client: http_client.clone(),
                    node_runtime: node_runtime.clone(),
                    languages: languages.clone(),
                    extension_host_proxy: proxy.clone(),
                    startup_time: std::time::Instant::now(),
                },
                false,
                cx,
            )
        });

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
        let disconnected_workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let database_id =
            disconnected_workspace.read_with(cx, |workspace, _| workspace.database_id());
        window
            .update(cx, |_, window, cx| {
                disconnected_workspace.update(cx, |workspace, cx| {
                    workspace.flush_serialization(window, cx)
                })
            })
            .unwrap()
            .await;

        // Force the remote client into ServerNotRunning state (simulates the
        // scenario where the remote server died and reconnection failed).
        window
            .update(cx, |multi_workspace, _, cx| {
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

        // Register a new mock server under the same options so the reconnect
        // path can establish a fresh connection.
        let (server_session_2, connect_guard_2) =
            RemoteClient::fake_server_with_opts(&opts, cx, server_cx);

        let _headless_2 = server_cx.new(|cx| {
            HeadlessProject::new(
                HeadlessAppState {
                    session: server_session_2,
                    fs: remote_fs.clone(),
                    http_client,
                    node_runtime,
                    languages,
                    extension_host_proxy: proxy,
                    startup_time: std::time::Instant::now(),
                },
                false,
                cx,
            )
        });

        drop(connect_guard_2);
        window
            .update(cx, |_, window, cx| {
                window.dispatch_action(Box::new(menu::Cancel), cx)
            })
            .unwrap();
        executor.run_until_parked();

        // Simulate clicking "Reconnect": calls open_remote_project with
        // replace_window pointing to the existing window.
        let result = open_remote_project(
            opts,
            paths,
            app_state,
            workspace::OpenOptions {
                requesting_window: Some(window),
                ..Default::default()
            },
            &mut async_cx,
        )
        .await;

        executor.run_until_parked();

        assert!(
            result.is_ok(),
            "reconnect should succeed but got: {:?}",
            result.err()
        );

        // Should still be a single window with a working remote project.
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);

        window
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert!(
                        workspace.project().read(cx).is_remote(),
                        "project should be remote after reconnect"
                    );
                });
                assert_eq!(
                    (
                        workspace != disconnected_workspace,
                        workspace.read(cx).database_id(),
                    ),
                    (true, database_id)
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_with_existing_connection_activates_window(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        let (opts, _headless) = start_headless_server(
            Path::new(path!("/project")),
            json!({ "main.rs": "fn main() {}" }),
            cx,
            server_cx,
        )
        .await;
        let remote_client = RemoteClient::connect_mock(opts.clone(), cx).await;
        let project = cx.update(|cx| {
            Project::remote(
                remote_client,
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                false,
                cx,
            )
        });
        let window = cx.add_window({
            let project = project.clone();
            let app_state = app_state.clone();
            |window, cx| {
                let workspace = cx.new(|cx| Workspace::new(None, project, app_state, window, cx));
                MultiWorkspace::new(workspace, window, cx)
            }
        });
        let other_window = cx.add_window(|_, _| Empty);
        other_window
            .update(cx, |_, window, _| window.activate_window())
            .unwrap();
        assert_eq!(cx.read(|cx| cx.active_window()), Some(other_window.into()));

        let mut async_cx = cx.to_async();
        workspace::open_remote_project_with_existing_connection(
            opts,
            project,
            vec![PathBuf::from(path!("/project"))],
            app_state,
            window,
            None,
            None,
            &mut async_cx,
        )
        .await
        .unwrap();
        cx.run_until_parked();

        assert_eq!(cx.read(|cx| cx.active_window()), Some(window.into()));
    }

    fn unbound_restoration() -> RestoredRemoteWorkspace {
        RestoredRemoteWorkspace {
            workspace_id: workspace::WorkspaceId::from_i64(0),
            original: workspace::SessionBinding::default(),
            state: workspace::MultiWorkspaceState::default(),
        }
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            state
        })
    }

    async fn start_headless_server(
        root: &Path,
        tree: serde_json::Value,
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) -> (RemoteConnectionOptions, Entity<HeadlessProject>) {
        cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        server_cx.update(|cx| release_channel::init(semver::Version::new(0, 0, 0), cx));
        let (opts, session, connect_guard) = RemoteClient::fake_server(cx, server_cx);
        let fs = FakeFs::new(server_cx.executor());
        fs.insert_tree(root, tree).await;
        server_cx.update(HeadlessProject::init);
        let languages = Arc::new(language::LanguageRegistry::new(server_cx.executor()));
        let headless = server_cx.new(|cx| {
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
        });
        drop(connect_guard);
        (opts, headless)
    }
}

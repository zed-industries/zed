use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

use anyhow::Context as _;
use askpass::EncryptedPassword;
use dev_container::{
    DevContainerConfig, DevContainerContext, DevContainerHost, find_devcontainer_configs,
};
use gpui::{
    AnyWindowHandle, AsyncApp, AsyncWindowContext, Context, Entity, WeakEntity, Window,
    WindowHandle,
};
use remote::{DockerConnectionOptions, DockerHost, RemoteConnectionOptions};
use workspace::{AppState, MultiWorkspace, OpenOptions, Workspace};

use crate::dev_container_suggest;
use crate::remote_connections::{
    Connection, RemoteClientDelegate, RemoteConnectionModal, RemoteConnectionPrompt,
    open_remote_project,
};

/// Surfaces a lifecycle failure to the user. All of the operations here report
/// errors the same way: a critical modal titled with the operation that
/// failed, detailing the underlying error.
async fn prompt_error(cx: &mut AsyncWindowContext, title: &str, detail: impl std::fmt::Display) {
    cx.prompt(
        gpui::PromptLevel::Critical,
        title,
        Some(&detail.to_string()),
        &["OK"],
    )
    .await
    .ok();
}

/// The machine whose container engine owns `options`' container, as the host
/// type the `dev_container` lifecycle commands take.
///
/// A dev container's engine is not necessarily this machine's: when the
/// container was built on an SSH or WSL host, every lifecycle command has to
/// be routed over a connection to that host, or it reaches this machine's
/// daemon and either finds no such container or, worse, an unrelated one.
///
/// The connection comes from the shared pool, so the one the container itself
/// is using is reused. Resolve it *before* tearing the container's connection
/// down, while that connection is still holding the host's open.
///
/// `window` and `workspace` are where a host that needs credentials asks for
/// them: connecting may require a password or a passphrase, and a lifecycle
/// command is an explicit user action, so it is allowed to prompt.
async fn lifecycle_host(
    options: &DockerConnectionOptions,
    window: Option<AnyWindowHandle>,
    workspace: Option<&WeakEntity<Workspace>>,
    cx: &mut AsyncApp,
) -> anyhow::Result<DevContainerHost> {
    if matches!(options.host, DockerHost::Local) {
        return Ok(DevContainerHost::Local);
    }

    let delegate = host_delegate(options, window, workspace, cx);
    // The modal `host_delegate` may have just opened would otherwise sit blank
    // until the transport reports its own first status.
    delegate.set_status(Some("Connecting to dev container host\u{2026}"), cx);

    let connection = remote::connect_docker_host(&options.host, delegate, cx)
        .await
        .with_context(|| format!("connecting to the host of dev container {}", options.name))?;

    Ok(match connection {
        Some(connection) => DevContainerHost::Remote(connection),
        None => DevContainerHost::Local,
    })
}

/// The delegate the host connection authenticates through.
///
/// Prompts in the connection modal when there is a window to show it in, so a
/// password- or passphrase-protected host can be reached. Falls back to the
/// non-interactive delegate when there is no UI to prompt over (the host then
/// has to authenticate unattended, or be in the pool already).
fn host_delegate(
    options: &DockerConnectionOptions,
    window: Option<AnyWindowHandle>,
    workspace: Option<&WeakEntity<Workspace>>,
    cx: &mut AsyncApp,
) -> Arc<dyn remote::RemoteClientDelegate> {
    let prompt = window
        .zip(workspace)
        .and_then(|(window, workspace)| lifecycle_prompt(window, workspace, options, cx));

    let Some((window, prompt)) = window.zip(prompt) else {
        return remote_connection::background_delegate();
    };

    Arc::new(RemoteClientDelegate::new(
        window,
        prompt.downgrade(),
        saved_password(&options.host),
    ))
}

/// The password already stored on the host's own connection, which is the one
/// the user would otherwise be asked to retype when a lifecycle command
/// reaches that host.
fn saved_password(host: &DockerHost) -> Option<EncryptedPassword> {
    let DockerHost::Ssh(ssh) = host else {
        return None;
    };
    EncryptedPassword::try_from(ssh.password.as_deref()?).ok()
}

/// The connection modal's prompt for `workspace`, opening the modal if it is
/// not already up. Returns `None` if the workspace or window has gone away.
///
/// The modal is the same type and entity [`open_remote_project`] uses for the
/// connect phase, so it is handed over rather than replaced: a password prompt
/// raised while resolving the host appears in the window the user is already
/// looking at, and the spinner carries on into connecting.
fn lifecycle_prompt(
    window: AnyWindowHandle,
    workspace: &WeakEntity<Workspace>,
    options: &DockerConnectionOptions,
    cx: &mut AsyncApp,
) -> Option<Entity<RemoteConnectionPrompt>> {
    let connection_options = RemoteConnectionOptions::Docker(options.clone());
    let workspace = workspace.clone();
    window
        .update(cx, move |_, window, cx| {
            workspace
                .update(cx, |workspace, cx| {
                    if workspace
                        .active_modal::<RemoteConnectionModal>(cx)
                        .is_none()
                    {
                        workspace.toggle_modal(window, cx, |window, cx| {
                            RemoteConnectionModal::new(&connection_options, Vec::new(), window, cx)
                        });
                    }
                    Some(
                        workspace
                            .active_modal::<RemoteConnectionModal>(cx)?
                            .read(cx)
                            .prompt
                            .clone(),
                    )
                })
                .ok()
                .flatten()
        })
        .ok()
        .flatten()
}

/// Cleanly tears down the remote connection currently backing `workspace`,
/// if any, *before* we destroy the dev container on the Docker side.
///
/// Without this, the still-live `RemoteClient` would notice its container
/// disappearing out from under it (its next heartbeat/exec would fail), and
/// its own reconnection logic would kick in and race with us reopening the
/// project - repeatedly retrying against a container id that no longer
/// exists, and eventually surfacing a "Disconnected" prompt. Calling
/// `shutdown_processes` bypasses that reconnection logic entirely (unlike
/// `RemoteClient::force_disconnect`, whose docs note it triggers
/// reconnection).
async fn shutdown_remote_connection(
    workspace_handle: &gpui::WeakEntity<Workspace>,
    cx: &mut AsyncWindowContext,
) {
    let shutdown_task = workspace_handle.update(cx, |workspace, cx| {
        workspace
            .project()
            .read(cx)
            .remote_client()
            .and_then(|client| {
                client.update(cx, |client, cx| {
                    client.shutdown_processes(
                        Some(rpc::proto::ShutdownRemoteServer {}),
                        cx.background_executor().clone(),
                    )
                })
            })
    });

    if let Ok(Some(shutdown_task)) = shutdown_task {
        shutdown_task.await;
    }
}

/// Stops the dev container backing the current project and reopens its
/// local folder in the same window. The container itself is left in place
/// (stopped, not removed), so it can be resumed later by reopening it in a
/// dev container again.
pub(crate) fn stop_dev_container(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(RemoteConnectionOptions::Docker(options)) =
        workspace.project().read(cx).remote_connection_options(cx)
    else {
        cx.propagate();
        return;
    };

    let app_state = Arc::downgrade(workspace.app_state());
    let replace_window = window.window_handle().downcast::<MultiWorkspace>();
    let workspace_handle = cx.entity().downgrade();

    cx.spawn_in(window, async move |_, cx| {
        let host = match lifecycle_host(
            &options,
            Some(cx.window_handle()),
            Some(&workspace_handle),
            cx,
        )
        .await
        {
            Ok(host) => host,
            Err(e) => {
                log::error!("Failed to reach the dev container's host: {e:#}");
                dismiss_lifecycle_status(&workspace_handle, cx);
                prompt_error(cx, "Failed to stop Dev Container", format!("{e:#}")).await;
                return;
            }
        };

        let origin = match dev_container::dev_container_origin(
            &options.container_id,
            host.clone(),
            options.engine,
        )
        .await
        {
            Ok(origin) => origin,
            Err(e) => {
                log::error!("Failed to determine dev container's local folder: {e}");
                prompt_error(cx, "Failed to stop Dev Container", &e).await;
                return;
            }
        };

        shutdown_remote_connection(&workspace_handle, cx).await;

        if let Err(e) =
            dev_container::stop_dev_container(&options.container_id, host, options.engine).await
        {
            log::error!("Failed to stop dev container: {e}");
            prompt_error(cx, "Failed to stop Dev Container", &e).await;
            return;
        }

        let Some(app_state) = app_state.upgrade() else {
            return;
        };

        let open_task = cx.update(|_, cx| {
            workspace::open_paths(
                &[origin.local_folder],
                app_state,
                OpenOptions {
                    requesting_window: replace_window,
                    ..Default::default()
                },
                cx,
            )
        });

        match open_task {
            Ok(task) => {
                if let Err(e) = task.await {
                    log::error!(
                        "Failed to reopen project locally after stopping dev container: {e:#}"
                    );
                }
            }
            Err(e) => {
                log::error!("Failed to reopen project locally after stopping dev container: {e:#}");
            }
        }
    })
    .detach();
}

/// Stops and removes the dev container backing the current project (a
/// `docker rm -f`), then reopens its local folder in the same window. Unlike
/// [`stop_dev_container`], the container is destroyed - its writable layer and
/// any state not on a mounted volume are lost - so this lets users reclaim
/// resources without dropping to the CLI, at the cost of needing a rebuild to
/// use the container again.
///
/// Because the deletion is irreversible it is confirmed with the user first.
/// Like every lifecycle side effect, this only ever runs on an explicit user
/// action.
pub(crate) fn delete_dev_container(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(RemoteConnectionOptions::Docker(options)) =
        workspace.project().read(cx).remote_connection_options(cx)
    else {
        cx.propagate();
        return;
    };

    let app_state = Arc::downgrade(workspace.app_state());
    let replace_window = window.window_handle().downcast::<MultiWorkspace>();
    let workspace_handle = cx.entity().downgrade();

    cx.spawn_in(window, async move |_, cx| {
        let reopen = replace_window.map(|window| (window, app_state));
        delete_dev_container_with_options(
            options,
            workspace_handle.clone(),
            vec![workspace_handle],
            reopen,
            cx,
        )
        .await;
    })
    .detach();
}

/// Shared implementation of the "Delete Dev Container" action, callable both
/// from a connected workspace (title bar / command palette) and from the
/// thread sidebar's project-group menu (which may target a *stopped* container
/// that has no live workspace).
///
/// - `ui_workspace`: the workspace whose window carries the progress and
///   credential prompts. A stopped container has no workspace of its own, so
///   this is the active one rather than one of `connected_workspaces`.
/// - `connected_workspaces`: any live workspaces currently backed by this
///   container; each has its remote connection cleanly shut down before the
///   container is destroyed. Empty for a stopped container.
/// - `reopen`: when `Some`, the container's local folder is reopened in the
///   given window after removal. Callers pass `None` when the container is not
///   backing the active window, so a background/stopped group is never allowed
///   to hijack the user's current window.
pub async fn delete_dev_container_with_options(
    options: DockerConnectionOptions,
    ui_workspace: WeakEntity<Workspace>,
    connected_workspaces: Vec<WeakEntity<Workspace>>,
    reopen: Option<(WindowHandle<MultiWorkspace>, Weak<AppState>)>,
    cx: &mut AsyncWindowContext,
) {
    let confirmed = cx
        .prompt(
            gpui::PromptLevel::Warning,
            "Delete this Dev Container?",
            Some(
                "The container will be stopped and removed. Any changes not on a \
                 mounted volume will be lost, and reconnecting later will require \
                 rebuilding it. Your project files on this machine are not affected.",
            ),
            &["Delete", "Cancel"],
        )
        .await;
    if !matches!(confirmed, Ok(0)) {
        return;
    }

    // Only resolve the local folder when we intend to reopen it. Read it from
    // the container's labels *before* removing it; afterwards it can no longer
    // be inspected.
    let host =
        match lifecycle_host(&options, Some(cx.window_handle()), Some(&ui_workspace), cx).await {
            Ok(host) => host,
            Err(e) => {
                log::error!("Failed to reach the dev container's host: {e:#}");
                dismiss_lifecycle_status(&ui_workspace, cx);
                prompt_error(cx, "Failed to delete Dev Container", format!("{e:#}")).await;
                return;
            }
        };

    let local_folder = if reopen.is_some() {
        match dev_container::dev_container_origin(
            &options.container_id,
            host.clone(),
            options.engine,
        )
        .await
        {
            Ok(origin) => Some(origin.local_folder),
            Err(e) => {
                log::error!("Failed to determine dev container's local folder: {e}");
                prompt_error(cx, "Failed to delete Dev Container", &e).await;
                return;
            }
        }
    } else {
        None
    };

    for workspace_handle in &connected_workspaces {
        shutdown_remote_connection(workspace_handle, cx).await;
    }

    if let Err(e) =
        dev_container::remove_dev_container(&options.container_id, host, options.engine).await
    {
        log::error!("Failed to remove dev container: {e}");
        prompt_error(cx, "Failed to delete Dev Container", &e).await;
        return;
    }

    let (Some(local_folder), Some((replace_window, app_state))) = (local_folder, reopen) else {
        return;
    };
    let Some(app_state) = app_state.upgrade() else {
        return;
    };

    let open_task = cx.update(|_, cx| {
        workspace::open_paths(
            &[local_folder],
            app_state,
            OpenOptions {
                requesting_window: Some(replace_window),
                ..Default::default()
            },
            cx,
        )
    });

    match open_task {
        Ok(task) => {
            if let Err(e) = task.await {
                log::error!("Failed to reopen project locally after deleting dev container: {e:#}");
            }
        }
        Err(e) => {
            log::error!("Failed to reopen project locally after deleting dev container: {e:#}");
        }
    }
}

/// Removes the current dev container, builds it again from scratch, and
/// reconnects to it in the same window. When invoked while the project is
/// local (no dev container currently connected), this instead opens the
/// dev container creation flow for this project in "rebuild" mode: any
/// existing container matching the chosen project/config is torn down and
/// rebuilt from scratch, then connected to, rather than resumed.
pub(crate) fn rebuild_dev_container(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(RemoteConnectionOptions::Docker(options)) =
        workspace.project().read(cx).remote_connection_options(cx)
    else {
        // Not connected to a dev container: fall back to the creation flow in
        // "rebuild" mode, so the chosen project/config is rebuilt from scratch
        // rather than resumed.
        open_dev_container_modal(workspace, true, window, cx);
        return;
    };

    rebuild_connected_dev_container(workspace, options, window, cx);
}

/// Reconnects to the dev container backing the current project, starting the
/// container again if it has stopped. Unlike [`rebuild_dev_container`], the
/// existing container is *resumed* (`docker start` + reconnect) rather than
/// torn down and rebuilt from scratch, so its state is preserved.
///
/// This is the recovery path for a dev container whose connection was lost
/// (e.g. the container exited or was stopped out from under Zed): the raw
/// remote reconnect does not start a stopped container, so a plain
/// "Reconnect" would keep failing against a container that is not running.
pub(crate) fn reconnect_dev_container(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(RemoteConnectionOptions::Docker(options)) =
        workspace.project().read(cx).remote_connection_options(cx)
    else {
        cx.propagate();
        return;
    };

    reconnect_connected_dev_container(workspace, options, ReconnectMode::Resume, window, cx);
}

/// Restarts the dev container backing the current project (stopping then
/// starting it, which kills every in-container process including any wedged
/// `zed-remote-server`) and reconnects. This is the escalation from a plain
/// [`reconnect_dev_container`] for the case where the container is still
/// running but its server is wedged, so resuming would keep reusing the broken
/// server. The container's filesystem is preserved; only its processes die.
///
/// Like every other lifecycle side effect, this must only run in response to an
/// explicit user action.
pub(crate) fn restart_dev_container_and_reconnect(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(RemoteConnectionOptions::Docker(options)) =
        workspace.project().read(cx).remote_connection_options(cx)
    else {
        cx.propagate();
        return;
    };

    reconnect_connected_dev_container(workspace, options, ReconnectMode::Restart, window, cx);
}

/// Opens the dev container config-discovery/selection modal for the current
/// (local) project, shared by the `OpenDevContainer` and `RebuildDevContainer`
/// entry points. With `force_rebuild`, any existing container matching the
/// chosen project/config is removed and rebuilt rather than resumed (see
/// `RemoteServerProjects::force_rebuild`).
pub(crate) fn open_dev_container_modal(
    workspace: &mut Workspace,
    force_rebuild: bool,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    // A remote project's container is built on its own host. A collab
    // project's host belongs to another user, so there is nothing to build it
    // over, and a project already inside a container has nowhere further to go.
    let verb = if force_rebuild { "rebuild" } else { "open" };
    let refusal = if workspace.project().read(cx).is_via_collab() {
        Some(format!("Cannot {verb} Dev Container from a shared project"))
    } else if dev_container_suggest::is_dev_container_project(workspace.project(), cx) {
        Some("This project is already open in a Dev Container".to_string())
    } else {
        None
    };
    if let Some(refusal) = refusal {
        cx.spawn_in(window, async move |_, cx| {
            cx.prompt(gpui::PromptLevel::Critical, &refusal, None, &["OK"])
                .await
                .ok();
        })
        .detach();
        return;
    }

    let fs = workspace.project().read(cx).fs().clone();
    let configs = find_devcontainer_configs(workspace, cx);
    let app_state = workspace.app_state().clone();
    let dev_container_context = DevContainerContext::from_workspace(workspace, cx);
    let handle = cx.entity().downgrade();
    workspace.toggle_modal(window, cx, |window, cx| {
        crate::RemoteServerProjects::new_dev_container(
            fs,
            configs,
            app_state,
            dev_container_context,
            force_rebuild,
            window,
            handle,
            cx,
        )
    });
}

fn rebuild_connected_dev_container(
    workspace: &mut Workspace,
    options: DockerConnectionOptions,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    reconnect_connected_dev_container(workspace, options, ReconnectMode::Rebuild, window, cx);
}

/// How the dev container currently connected to a workspace is (re)established
/// when recovering it in place.
#[derive(Clone, Copy)]
enum ReconnectMode {
    /// Resume the existing container (`docker start` if it has stopped) and
    /// reconnect. Preserves all in-container state.
    Resume,
    /// Stop then start the container, killing every in-container process
    /// (including a wedged `zed-remote-server`) before reconnecting. The
    /// container and its filesystem are kept.
    Restart,
    /// Remove and rebuild the container from scratch before reconnecting.
    Rebuild,
}

impl ReconnectMode {
    /// Title shown on the error modal when this flow fails.
    fn error_title(self) -> &'static str {
        match self {
            ReconnectMode::Resume => "Failed to reconnect to Dev Container",
            ReconnectMode::Restart => "Failed to restart Dev Container",
            ReconnectMode::Rebuild => "Failed to rebuild Dev Container",
        }
    }

    /// Status shown in the connection modal while the container-lifecycle phase
    /// runs, so a slow `docker` step (especially a rebuild, which can take
    /// minutes) doesn't leave the window looking frozen before reconnecting.
    fn status(self) -> &'static str {
        match self {
            ReconnectMode::Resume => "Starting dev container\u{2026}",
            ReconnectMode::Restart => "Restarting dev container\u{2026}",
            ReconnectMode::Rebuild => "Rebuilding dev container\u{2026}",
        }
    }
}

/// Shared implementation for resuming, restarting, or rebuilding the dev
/// container currently connected to `workspace` (see [`ReconnectMode`]). In
/// every case we recover the container's origin (host folder + config) from its
/// identifying labels, tear down the live remote connection cleanly, bring the
/// container to a clean running state per `mode`, and reopen the project against
/// the resulting connection in the same window.
fn reconnect_connected_dev_container(
    workspace: &mut Workspace,
    options: DockerConnectionOptions,
    mode: ReconnectMode,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let app_state = Arc::downgrade(workspace.app_state());
    let replace_window = window.window_handle().downcast::<MultiWorkspace>();
    let workspace_handle = cx.entity().downgrade();

    let error_title = mode.error_title();
    let force_rebuild = matches!(mode, ReconnectMode::Rebuild);

    cx.spawn_in(window, async move |_, cx| {
        let host = match lifecycle_host(
            &options,
            Some(cx.window_handle()),
            Some(&workspace_handle),
            cx,
        )
        .await
        {
            Ok(host) => host,
            Err(e) => {
                log::error!("Failed to reach the dev container's host: {e:#}");
                dismiss_lifecycle_status(&workspace_handle, cx);
                prompt_error(cx, error_title, format!("{e:#}")).await;
                return;
            }
        };

        let origin = match dev_container::dev_container_origin(
            &options.container_id,
            host.clone(),
            options.engine,
        )
        .await
        {
            Ok(origin) => origin,
            Err(e) => {
                log::error!("Failed to determine dev container's local folder: {e}");
                prompt_error(cx, error_title, &e).await;
                return;
            }
        };

        let context = match workspace_handle.update(cx, |workspace, cx| {
            context_on_host(
                Arc::from(origin.local_folder.as_path()),
                host.clone(),
                workspace,
                cx,
            )
        }) {
            Ok(context) => context,
            Err(e) => {
                log::error!("Workspace no longer available to reconnect dev container: {e:#}");
                return;
            }
        };

        let environment = context.environment(cx).await;

        // Surface progress before the potentially slow container-lifecycle work
        // (shutdown/stop/build). The connection modal we open here is a
        // `RemoteConnectionModal`, the same type `open_remote_project` uses for
        // the subsequent connect phase; since it stays open (it only dismisses
        // once marked finished), `open_remote_project` reuses it rather than
        // flashing a new one, so the spinner transitions straight from the
        // container build into connecting.
        show_lifecycle_status(&workspace_handle, &options, mode.status(), cx);

        shutdown_remote_connection(&workspace_handle, cx).await;

        // For a restart, stop the container first so the subsequent start
        // (below) brings up a container with no leftover processes - notably no
        // wedged `zed-remote-server` reusing a still-valid pid file. Rebuild
        // handles teardown itself, and resume intentionally leaves a running
        // container running.
        if matches!(mode, ReconnectMode::Restart) {
            if let Err(e) = dev_container::stop_dev_container(
                &options.container_id,
                host.clone(),
                options.engine,
            )
            .await
            {
                log::error!("Failed to stop dev container before restart: {e}");
                dismiss_lifecycle_status(&workspace_handle, cx);
                prompt_error(cx, error_title, &e).await;
                return;
            }
        }

        let start_result = dev_container::start_dev_container_with_config(
            context,
            Some(origin.config),
            environment,
            force_rebuild,
            cx,
        )
        .await;

        let (connection, docker_host, starting_dir) = match start_result {
            Ok(result) => result,
            Err(e) => {
                log::error!("Failed to start dev container: {e}");
                dismiss_lifecycle_status(&workspace_handle, cx);
                prompt_error(cx, error_title, &e).await;
                return;
            }
        };

        let Some(app_state) = app_state.upgrade() else {
            return;
        };

        let result = open_remote_project(
            Connection::DevContainer(connection, docker_host).into(),
            vec![PathBuf::from(starting_dir)],
            app_state,
            OpenOptions {
                requesting_window: replace_window,
                ..OpenOptions::default()
            },
            cx,
        )
        .await;

        if let Err(e) = result {
            log::error!("Failed to reconnect to dev container: {e:#}");
            prompt_error(cx, "Failed to reconnect", format!("{e:#}")).await;
        }
    })
    .detach();
}

/// A build context for `project_directory` on `host`, for rebuilding a
/// container Zed is (or was) connected to.
///
/// [`DevContainerContext::for_local_directory`] derives the host from the
/// workspace's own connection, which for a dev container workspace is the
/// container - so rebuilding from inside the container's own window would aim
/// at the container rather than the machine whose engine owns it. The
/// container's recorded host is the authority instead.
fn context_on_host(
    project_directory: Arc<Path>,
    host: DevContainerHost,
    workspace: &Workspace,
    cx: &Context<Workspace>,
) -> DevContainerContext {
    let mut context = DevContainerContext::for_local_directory(project_directory, workspace, cx);
    context.host = host;
    // The workspace's remote client talks to the container, not to its host, so
    // it cannot answer host-side requests such as reading the environment.
    context.remote_client = None;
    context
}

/// Shows the connection modal on `workspace` with `status`, giving feedback
/// while a slow container-lifecycle step runs. If the modal is not already
/// open it is toggled on; otherwise its status label is updated in place.
/// Best-effort: does nothing if the workspace has gone away.
fn show_lifecycle_status(
    workspace_handle: &WeakEntity<Workspace>,
    options: &DockerConnectionOptions,
    status: &str,
    cx: &mut AsyncWindowContext,
) {
    let connection_options = RemoteConnectionOptions::Docker(options.clone());
    let status = status.to_string();
    workspace_handle
        .update_in(cx, |workspace, window, cx| {
            if workspace
                .active_modal::<RemoteConnectionModal>(cx)
                .is_none()
            {
                workspace.toggle_modal(window, cx, |window, cx| {
                    RemoteConnectionModal::new(&connection_options, Vec::new(), window, cx)
                });
            }
            if let Some(modal) = workspace.active_modal::<RemoteConnectionModal>(cx) {
                let prompt = modal.read(cx).prompt.clone();
                prompt.update(cx, |prompt, cx| prompt.set_status(Some(status), cx));
            }
        })
        .ok();
}

/// Dismisses the connection modal previously shown by [`show_lifecycle_status`],
/// so a subsequent error prompt isn't left sitting behind the spinner.
fn dismiss_lifecycle_status(workspace_handle: &WeakEntity<Workspace>, cx: &mut AsyncWindowContext) {
    workspace_handle
        .update_in(cx, |workspace, _window, cx| {
            if let Some(modal) = workspace.active_modal::<RemoteConnectionModal>(cx) {
                modal.update(cx, |modal, cx| modal.finished(cx));
            }
        })
        .ok();
}

/// Rebuilds the dev container described by `options` from scratch and returns
/// fresh connection options pointing at the newly built container.
///
/// The build inputs (host project folder + config) are recovered from the
/// connection's persisted `local_folder`/`config_file` labels, so this works
/// even when the old container has been removed. Because a rebuild mints a new
/// `container_id`, the caller must reconnect with the returned options rather
/// than the originals; the persisted connection row is keyed on the stable
/// labels, so it is reused (not duplicated) across the rebuild.
///
/// Only ever invoked on an explicit user action.
pub(crate) async fn rebuild_dev_container_connection(
    workspace: WeakEntity<Workspace>,
    options: &DockerConnectionOptions,
    window: WindowHandle<MultiWorkspace>,
    cx: &mut AsyncApp,
) -> anyhow::Result<RemoteConnectionOptions> {
    let local_folder = options
        .local_folder
        .clone()
        .context("dev container connection is missing its local_folder label")?;
    let config_file = options
        .config_file
        .clone()
        .context("dev container connection is missing its config_file label")?;
    let local_folder: Arc<Path> = Arc::from(PathBuf::from(&local_folder).as_path());
    let config = DevContainerConfig::from_recovered_paths(&local_folder, Path::new(&config_file));

    let host = lifecycle_host(options, Some(window.into()), Some(&workspace), cx).await?;
    let context = workspace.update(cx, |workspace, cx| {
        context_on_host(local_folder.clone(), host, workspace, cx)
    })?;
    let environment = context.environment(cx).await;

    let (connection, docker_host, _starting_dir) = dev_container::start_dev_container_with_config(
        context,
        Some(config),
        environment,
        true,
        cx,
    )
    .await
    .map_err(|e| anyhow::anyhow!("{e}"))?;

    Ok(Connection::DevContainer(connection, docker_host).into())
}

/// Starts the dev container backing `options` (a `docker start`), a no-op if it
/// is already running. Used by the connection-failure modal's "Reconnect Dev
/// Container" to bring a stopped container back up before retrying, without
/// disturbing a container that is merely wedged.
///
/// Like the other lifecycle side effects, this only runs on an explicit user
/// action.
pub(crate) async fn start_dev_container(
    options: &DockerConnectionOptions,
    window: WindowHandle<MultiWorkspace>,
    workspace: WeakEntity<Workspace>,
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    let host = lifecycle_host(options, Some(window.into()), Some(&workspace), cx).await?;
    dev_container::start_dev_container(&options.container_id, host, options.engine)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))
}

/// Restarts the dev container backing `options` in place (a `docker stop`
/// followed by `docker start`), clearing any wedged in-container server state
/// so a subsequent connection can start from a clean slate.
///
/// This must only ever be invoked in response to an explicit user action:
/// restarting a container is side-effecting and the user has to opt into it.
pub(crate) async fn restart_dev_container(
    options: &DockerConnectionOptions,
    window: WindowHandle<MultiWorkspace>,
    workspace: WeakEntity<Workspace>,
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    let host = lifecycle_host(options, Some(window.into()), Some(&workspace), cx).await?;
    dev_container::restart_dev_container(&options.container_id, host, options.engine)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))
}

#[cfg(test)]
mod tests {
    use super::{DevContainerHost, lifecycle_host, saved_password};
    use gpui::TestAppContext;
    use remote::{
        DockerConnectionOptions, DockerHost, RemoteClient, RemoteConnectionOptions,
        SshConnectionOptions,
    };

    fn options(host: DockerHost) -> DockerConnectionOptions {
        DockerConnectionOptions {
            name: "zed-dev".to_string(),
            container_id: "container-123".to_string(),
            remote_user: "root".to_string(),
            host,
            ..Default::default()
        }
    }

    /// A container built on another machine is managed over a connection to
    /// that machine; running the lifecycle commands here would reach the wrong
    /// daemon.
    #[gpui::test]
    async fn lifecycle_commands_target_the_daemons_machine(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let (host_options, _server, connect_guard) = RemoteClient::fake_server(cx, server_cx);
        let RemoteConnectionOptions::Mock(host_options) = host_options else {
            panic!("a fake server is a mock connection");
        };
        drop(connect_guard);

        let local = lifecycle_host(&options(DockerHost::Local), None, None, &mut cx.to_async())
            .await
            .expect("a container on this machine needs no host connection");
        assert!(matches!(local, DevContainerHost::Local));

        let remote = lifecycle_host(
            &options(DockerHost::Mock(host_options)),
            None,
            None,
            &mut cx.to_async(),
        )
        .await
        .expect("connecting to the container's host should succeed");
        assert!(matches!(remote, DevContainerHost::Remote(_)));
    }

    /// A host whose password the user already gave Zed must not ask for it
    /// again just because the connection is being made for a lifecycle command.
    #[test]
    fn a_hosts_saved_password_is_reused() {
        assert!(saved_password(&DockerHost::Local).is_none());

        let host = DockerHost::Ssh(SshConnectionOptions {
            host: "example.com".into(),
            ..Default::default()
        });
        assert!(
            saved_password(&host).is_none(),
            "a host with no saved password has nothing to reuse"
        );

        let host = DockerHost::Ssh(SshConnectionOptions {
            host: "example.com".into(),
            password: Some("hunter2".to_string()),
            ..Default::default()
        });
        assert!(saved_password(&host).is_some());
    }
}

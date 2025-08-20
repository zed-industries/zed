use crate::handle_open_request;
use crate::restorable_workspace_locations;
use anyhow::{Context as _, Result, anyhow};
use cli::{CliRequest, CliResponse, ipc::IpcSender};
use cli::{IpcHandshake, ipc};
use client::parse_zed_link;
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use fs::Fs;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::channel::{mpsc, oneshot};
use futures::future::join_all;
use futures::{FutureExt, SinkExt, StreamExt};
use git_ui::file_diff_view::FileDiffView;
use gpui::{App, AsyncApp, Global, WindowHandle};
use language::Point;
use onboarding::FIRST_OPEN;
use onboarding::show_onboarding_view;
use recent_projects::{SshSettings, open_ssh_project};
use remote::SshConnectionOptions;
use settings::Settings;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use util::ResultExt;
use util::paths::PathWithPosition;
use workspace::item::ItemHandle;
use workspace::{AppState, OpenOptions, SerializedWorkspaceLocation, Workspace};

#[derive(Default, Debug)]
pub struct OpenRequest {
    pub kind: Option<OpenRequestKind>,
    pub open_paths: Vec<String>,
    pub diff_paths: Vec<[String; 2]>,
    pub open_channel_notes: Vec<(u64, Option<String>)>,
    pub join_channel: Option<u64>,
    pub ssh_connection: Option<SshConnectionOptions>,
}

#[derive(Debug)]
pub enum OpenRequestKind {
    CliConnection((mpsc::Receiver<CliRequest>, IpcSender<CliResponse>)),
    Extension { extension_id: String },
    AgentPanel,
    DockMenuAction { index: usize },
}

impl OpenRequest {
    pub fn parse(request: RawOpenRequest, cx: &App) -> Result<Self> {
        let mut this = Self::default();
        for url in request.urls {
            if let Some(server_name) = url.strip_prefix("zed-cli://") {
                this.kind = Some(OpenRequestKind::CliConnection(connect_to_cli(server_name)?));
            } else if let Some(action_index) = url.strip_prefix("zed-dock-action://") {
                this.kind = Some(OpenRequestKind::DockMenuAction {
                    index: action_index.parse()?,
                });
            } else if let Some(file) = url.strip_prefix("file://") {
                this.parse_file_path(file)
            } else if let Some(file) = url.strip_prefix("zed://file") {
                this.parse_file_path(file)
            } else if let Some(file) = url.strip_prefix("zed://ssh") {
                let ssh_url = "ssh:/".to_string() + file;
                this.parse_ssh_file_path(&ssh_url, cx)?
            } else if let Some(extension_id) = url.strip_prefix("zed://extension/") {
                this.kind = Some(OpenRequestKind::Extension {
                    extension_id: extension_id.to_string(),
                });
            } else if url == "zed://agent" {
                this.kind = Some(OpenRequestKind::AgentPanel);
            } else if url.starts_with("ssh://") {
                this.parse_ssh_file_path(&url, cx)?
            } else if let Some(request_path) = parse_zed_link(&url, cx) {
                this.parse_request_path(request_path).log_err();
            } else {
                log::error!("unhandled url: {}", url);
            }
        }

        this.diff_paths = request.diff_paths;

        Ok(this)
    }

    fn parse_file_path(&mut self, file: &str) {
        if let Some(decoded) = urlencoding::decode(file).log_err() {
            self.open_paths.push(decoded.into_owned())
        }
    }

    fn parse_ssh_file_path(&mut self, file: &str, cx: &App) -> Result<()> {
        let url = url::Url::parse(file)?;
        let host = url
            .host()
            .with_context(|| format!("missing host in ssh url: {file}"))?
            .to_string();
        let username = Some(url.username().to_string()).filter(|s| !s.is_empty());
        let port = url.port();
        anyhow::ensure!(
            self.open_paths.is_empty(),
            "cannot open both local and ssh paths"
        );
        let mut connection_options =
            SshSettings::get_global(cx).connection_options_for(host, port, username);
        if let Some(password) = url.password() {
            connection_options.password = Some(password.to_string());
        }
        if let Some(ssh_connection) = &self.ssh_connection {
            anyhow::ensure!(
                *ssh_connection == connection_options,
                "cannot open multiple ssh connections"
            );
        }
        self.ssh_connection = Some(connection_options);
        self.parse_file_path(url.path());
        Ok(())
    }

    fn parse_request_path(&mut self, request_path: &str) -> Result<()> {
        let mut parts = request_path.split('/');
        if parts.next() == Some("channel")
            && let Some(slug) = parts.next()
            && let Some(id_str) = slug.split('-').next_back()
            && let Ok(channel_id) = id_str.parse::<u64>()
        {
            let Some(next) = parts.next() else {
                self.join_channel = Some(channel_id);
                return Ok(());
            };

            if let Some(heading) = next.strip_prefix("notes#") {
                self.open_channel_notes
                    .push((channel_id, Some(heading.to_string())));
                return Ok(());
            }
            if next == "notes" {
                self.open_channel_notes.push((channel_id, None));
                return Ok(());
            }
        }
        anyhow::bail!("invalid zed url: {request_path}")
    }
}

#[derive(Clone)]
pub struct OpenListener(UnboundedSender<RawOpenRequest>);

#[derive(Default)]
pub struct RawOpenRequest {
    pub urls: Vec<String>,
    pub diff_paths: Vec<[String; 2]>,
}

impl Global for OpenListener {}

impl OpenListener {
    pub fn new() -> (Self, UnboundedReceiver<RawOpenRequest>) {
        let (tx, rx) = mpsc::unbounded();
        (OpenListener(tx), rx)
    }

    pub fn open(&self, request: RawOpenRequest) {
        self.0
            .unbounded_send(request)
            .context("no listener for open requests")
            .log_err();
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub fn listen_for_cli_connections(opener: OpenListener) -> Result<()> {
    use release_channel::RELEASE_CHANNEL_NAME;
    use std::os::unix::net::UnixDatagram;

    let sock_path = paths::data_dir().join(format!("zed-{}.sock", *RELEASE_CHANNEL_NAME));
    // remove the socket if the process listening on it has died
    if let Err(e) = UnixDatagram::unbound()?.connect(&sock_path)
        && e.kind() == std::io::ErrorKind::ConnectionRefused
    {
        std::fs::remove_file(&sock_path)?;
    }
    let listener = UnixDatagram::bind(&sock_path)?;
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        while let Ok(len) = listener.recv(&mut buf) {
            opener.open(RawOpenRequest {
                urls: vec![String::from_utf8_lossy(&buf[..len]).to_string()],
                ..Default::default()
            });
        }
    });
    Ok(())
}

fn connect_to_cli(
    server_name: &str,
) -> Result<(mpsc::Receiver<CliRequest>, IpcSender<CliResponse>)> {
    let handshake_tx = cli::ipc::IpcSender::<IpcHandshake>::connect(server_name.to_string())
        .context("error connecting to cli")?;
    let (request_tx, request_rx) = ipc::channel::<CliRequest>()?;
    let (response_tx, response_rx) = ipc::channel::<CliResponse>()?;

    handshake_tx
        .send(IpcHandshake {
            requests: request_tx,
            responses: response_rx,
        })
        .context("error sending ipc handshake")?;

    let (mut async_request_tx, async_request_rx) =
        futures::channel::mpsc::channel::<CliRequest>(16);
    thread::spawn(move || {
        while let Ok(cli_request) = request_rx.recv() {
            if smol::block_on(async_request_tx.send(cli_request)).is_err() {
                break;
            }
        }
        anyhow::Ok(())
    });

    Ok((async_request_rx, response_tx))
}

pub async fn open_paths_with_positions(
    path_positions: &[PathWithPosition],
    diff_paths: &[[String; 2]],
    app_state: Arc<AppState>,
    open_options: workspace::OpenOptions,
    cx: &mut AsyncApp,
) -> Result<(
    WindowHandle<Workspace>,
    Vec<Option<Result<Box<dyn ItemHandle>>>>,
)> {
    let mut caret_positions = HashMap::default();

    let paths = path_positions
        .iter()
        .map(|path_with_position| {
            let path = path_with_position.path.clone();
            if let Some(row) = path_with_position.row
                && path.is_file()
            {
                let row = row.saturating_sub(1);
                let col = path_with_position.column.unwrap_or(0).saturating_sub(1);
                caret_positions.insert(path.clone(), Point::new(row, col));
            }
            path
        })
        .collect::<Vec<_>>();

    let (workspace, mut items) = cx
        .update(|cx| workspace::open_paths(&paths, app_state, open_options, cx))?
        .await?;

    for diff_pair in diff_paths {
        let old_path = Path::new(&diff_pair[0]).canonicalize()?;
        let new_path = Path::new(&diff_pair[1]).canonicalize()?;
        if let Ok(diff_view) = workspace.update(cx, |workspace, window, cx| {
            FileDiffView::open(old_path, new_path, workspace, window, cx)
        }) && let Some(diff_view) = diff_view.await.log_err()
        {
            items.push(Some(Ok(Box::new(diff_view))))
        }
    }

    for (item, path) in items.iter_mut().zip(&paths) {
        if let Some(Err(error)) = item {
            *error = anyhow!("error opening {path:?}: {error}");
            continue;
        }
        let Some(Ok(item)) = item else {
            continue;
        };
        let Some(point) = caret_positions.remove(path) else {
            continue;
        };
        if let Some(active_editor) = item.downcast::<Editor>() {
            workspace
                .update(cx, |_, window, cx| {
                    active_editor.update(cx, |editor, cx| {
                        editor.go_to_singleton_buffer_point(point, window, cx);
                    });
                })
                .log_err();
        }
    }

    Ok((workspace, items))
}

pub async fn handle_cli_connection(
    (mut requests, responses): (mpsc::Receiver<CliRequest>, IpcSender<CliResponse>),
    app_state: Arc<AppState>,
    cx: &mut AsyncApp,
) {
    if let Some(request) = requests.next().await {
        match request {
            CliRequest::Open {
                urls,
                paths,
                diff_paths,
                wait,
                open_new_workspace,
                env,
                user_data_dir: _,
            } => {
                if !urls.is_empty() {
                    cx.update(|cx| {
                        match OpenRequest::parse(RawOpenRequest { urls, diff_paths }, cx) {
                            Ok(open_request) => {
                                handle_open_request(open_request, app_state.clone(), cx);
                                responses.send(CliResponse::Exit { status: 0 }).log_err();
                            }
                            Err(e) => {
                                responses
                                    .send(CliResponse::Stderr {
                                        message: format!("{e}"),
                                    })
                                    .log_err();
                                responses.send(CliResponse::Exit { status: 1 }).log_err();
                            }
                        };
                    })
                    .log_err();
                    return;
                }

                let open_workspace_result = open_workspaces(
                    paths,
                    diff_paths,
                    open_new_workspace,
                    &responses,
                    wait,
                    app_state.clone(),
                    env,
                    cx,
                )
                .await;

                let status = if open_workspace_result.is_err() { 1 } else { 0 };
                responses.send(CliResponse::Exit { status }).log_err();
            }
        }
    }
}

async fn open_workspaces(
    paths: Vec<String>,
    diff_paths: Vec<[String; 2]>,
    open_new_workspace: Option<bool>,
    responses: &IpcSender<CliResponse>,
    wait: bool,
    app_state: Arc<AppState>,
    env: Option<collections::HashMap<String, String>>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let grouped_locations = if paths.is_empty() && diff_paths.is_empty() {
        // If no paths are provided, restore from previous workspaces unless a new workspace is requested with -n
        if open_new_workspace == Some(true) {
            Vec::new()
        } else {
            let locations = restorable_workspace_locations(cx, &app_state).await;
            locations.unwrap_or_default()
        }
    } else {
        vec![SerializedWorkspaceLocation::from_local_paths(
            paths.into_iter().map(PathBuf::from),
        )]
    };

    if grouped_locations.is_empty() {
        // If we have no paths to open, show the welcome screen if this is the first launch
        if matches!(KEY_VALUE_STORE.read_kvp(FIRST_OPEN), Ok(None)) {
            cx.update(|cx| show_onboarding_view(app_state, cx).detach())
                .log_err();
        }
        // If not the first launch, show an empty window with empty editor
        else {
            cx.update(|cx| {
                let open_options = OpenOptions {
                    env,
                    ..Default::default()
                };
                workspace::open_new(open_options, app_state, cx, |workspace, window, cx| {
                    Editor::new_file(workspace, &Default::default(), window, cx)
                })
                .detach();
            })
            .log_err();
        }
    } else {
        // If there are paths to open, open a workspace for each grouping of paths
        let mut errored = false;

        for location in grouped_locations {
            match location {
                SerializedWorkspaceLocation::Local(workspace_paths, _) => {
                    let workspace_paths = workspace_paths
                        .paths()
                        .iter()
                        .map(|path| path.to_string_lossy().to_string())
                        .collect();

                    let workspace_failed_to_open = open_local_workspace(
                        workspace_paths,
                        diff_paths.clone(),
                        open_new_workspace,
                        wait,
                        responses,
                        env.as_ref(),
                        &app_state,
                        cx,
                    )
                    .await;

                    if workspace_failed_to_open {
                        errored = true
                    }
                }
                SerializedWorkspaceLocation::Ssh(ssh) => {
                    let app_state = app_state.clone();
                    let connection_options = cx.update(|cx| {
                        SshSettings::get_global(cx)
                            .connection_options_for(ssh.host, ssh.port, ssh.user)
                    });
                    if let Ok(connection_options) = connection_options {
                        cx.spawn(async move |cx| {
                            open_ssh_project(
                                connection_options,
                                ssh.paths.into_iter().map(PathBuf::from).collect(),
                                app_state,
                                OpenOptions::default(),
                                cx,
                            )
                            .await
                            .log_err();
                        })
                        .detach();
                        // We don't set `errored` here if `open_ssh_project` fails, because for ssh projects, the
                        // error is displayed in the window.
                    } else {
                        errored = false;
                    }
                }
            }
        }

        anyhow::ensure!(!errored, "failed to open a workspace");
    }

    Ok(())
}

async fn open_local_workspace(
    workspace_paths: Vec<String>,
    diff_paths: Vec<[String; 2]>,
    open_new_workspace: Option<bool>,
    wait: bool,
    responses: &IpcSender<CliResponse>,
    env: Option<&HashMap<String, String>>,
    app_state: &Arc<AppState>,
    cx: &mut AsyncApp,
) -> bool {
    let mut errored = false;

    let paths_with_position =
        derive_paths_with_position(app_state.fs.as_ref(), workspace_paths).await;
    match open_paths_with_positions(
        &paths_with_position,
        &diff_paths,
        app_state.clone(),
        workspace::OpenOptions {
            open_new_workspace,
            env: env.cloned(),
            ..Default::default()
        },
        cx,
    )
    .await
    {
        Ok((workspace, items)) => {
            let mut item_release_futures = Vec::new();

            for item in items {
                match item {
                    Some(Ok(item)) => {
                        cx.update(|cx| {
                            let released = oneshot::channel();
                            item.on_release(
                                cx,
                                Box::new(move |_| {
                                    let _ = released.0.send(());
                                }),
                            )
                            .detach();
                            item_release_futures.push(released.1);
                        })
                        .log_err();
                    }
                    Some(Err(err)) => {
                        responses
                            .send(CliResponse::Stderr {
                                message: err.to_string(),
                            })
                            .log_err();
                        errored = true;
                    }
                    None => {}
                }
            }

            if wait {
                let background = cx.background_executor().clone();
                let wait = async move {
                    if paths_with_position.is_empty() && diff_paths.is_empty() {
                        let (done_tx, done_rx) = oneshot::channel();
                        let _subscription = workspace.update(cx, |_, _, cx| {
                            cx.on_release(move |_, _| {
                                let _ = done_tx.send(());
                            })
                        });
                        let _ = done_rx.await;
                    } else {
                        let _ = futures::future::try_join_all(item_release_futures).await;
                    };
                }
                .fuse();

                futures::pin_mut!(wait);

                loop {
                    // Repeatedly check if CLI is still open to avoid wasting resources
                    // waiting for files or workspaces to close.
                    let mut timer = background.timer(Duration::from_secs(1)).fuse();
                    futures::select_biased! {
                        _ = wait => break,
                        _ = timer => {
                            if responses.send(CliResponse::Ping).is_err() {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Err(error) => {
            errored = true;
            responses
                .send(CliResponse::Stderr {
                    message: format!("error opening {paths_with_position:?}: {error}"),
                })
                .log_err();
        }
    }
    errored
}

pub async fn derive_paths_with_position(
    fs: &dyn Fs,
    path_strings: impl IntoIterator<Item = impl AsRef<str>>,
) -> Vec<PathWithPosition> {
    join_all(path_strings.into_iter().map(|path_str| async move {
        let canonicalized = fs.canonicalize(Path::new(path_str.as_ref())).await;
        (path_str, canonicalized)
    }))
    .await
    .into_iter()
    .map(|(original, canonicalized)| match canonicalized {
        Ok(canonicalized) => PathWithPosition::from_path(canonicalized),
        Err(_) => PathWithPosition::parse_str(original.as_ref()),
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zed::{open_listener::open_local_workspace, tests::init_test};
    use cli::{
        CliResponse,
        ipc::{self},
    };
    use editor::Editor;
    use gpui::TestAppContext;
    use serde_json::json;
    use std::sync::Arc;
    use util::path;
    use workspace::{AppState, Workspace};

    #[gpui::test]
    fn test_parse_ssh_url(cx: &mut TestAppContext) {
        let _app_state = init_test(cx);
        cx.update(|cx| {
            SshSettings::register(cx);
        });
        let request = cx.update(|cx| {
            OpenRequest::parse(
                RawOpenRequest {
                    urls: vec!["ssh://me@localhost:/".into()],
                    ..Default::default()
                },
                cx,
            )
            .unwrap()
        });
        assert_eq!(
            request.ssh_connection.unwrap(),
            SshConnectionOptions {
                host: "localhost".into(),
                username: Some("me".into()),
                port: None,
                password: None,
                args: None,
                port_forwards: None,
                nickname: None,
                upload_binary_over_ssh: false,
            }
        );
        assert_eq!(request.open_paths, vec!["/"]);
    }

    #[gpui::test]
    async fn test_open_workspace_with_directory(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "dir1": {
                        "file1.txt": "content1",
                        "file2.txt": "content2",
                    },
                }),
            )
            .await;

        assert_eq!(cx.windows().len(), 0);

        // First open the workspace directory
        open_workspace_file(path!("/root/dir1"), None, app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 1);
        let workspace = cx.windows()[0].downcast::<Workspace>().unwrap();
        workspace
            .update(cx, |workspace, _, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_none())
            })
            .unwrap();

        // Now open a file inside that workspace
        open_workspace_file(path!("/root/dir1/file1.txt"), None, app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 1);
        workspace
            .update(cx, |workspace, _, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_some());
            })
            .unwrap();

        // Now open a file inside that workspace, but tell Zed to open a new window
        open_workspace_file(
            path!("/root/dir1/file1.txt"),
            Some(true),
            app_state.clone(),
            cx,
        )
        .await;

        assert_eq!(cx.windows().len(), 2);

        let workspace_2 = cx.windows()[1].downcast::<Workspace>().unwrap();
        workspace_2
            .update(cx, |workspace, _, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_some());
                let items = workspace.items(cx).collect::<Vec<_>>();
                assert_eq!(items.len(), 1, "Workspace should have two items");
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_workspace_with_nonexistent_files(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/root"), json!({}))
            .await;

        assert_eq!(cx.windows().len(), 0);

        // Test case 1: Open a single file that does not exist yet
        open_workspace_file(path!("/root/file5.txt"), None, app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 1);
        let workspace_1 = cx.windows()[0].downcast::<Workspace>().unwrap();
        workspace_1
            .update(cx, |workspace, _, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_some())
            })
            .unwrap();

        // Test case 2: Open a single file that does not exist yet,
        // but tell Zed to add it to the current workspace
        open_workspace_file(path!("/root/file6.txt"), Some(false), app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 1);
        workspace_1
            .update(cx, |workspace, _, cx| {
                let items = workspace.items(cx).collect::<Vec<_>>();
                assert_eq!(items.len(), 2, "Workspace should have two items");
            })
            .unwrap();

        // Test case 3: Open a single file that does not exist yet,
        // but tell Zed to NOT add it to the current workspace
        open_workspace_file(path!("/root/file7.txt"), Some(true), app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 2);
        let workspace_2 = cx.windows()[1].downcast::<Workspace>().unwrap();
        workspace_2
            .update(cx, |workspace, _, cx| {
                let items = workspace.items(cx).collect::<Vec<_>>();
                assert_eq!(items.len(), 1, "Workspace should have two items");
            })
            .unwrap();
    }

    async fn open_workspace_file(
        path: &str,
        open_new_workspace: Option<bool>,
        app_state: Arc<AppState>,
        cx: &TestAppContext,
    ) {
        let (response_tx, _) = ipc::channel::<CliResponse>().unwrap();

        let workspace_paths = vec![path.to_owned()];

        let errored = cx
            .spawn(|mut cx| async move {
                open_local_workspace(
                    workspace_paths,
                    vec![],
                    open_new_workspace,
                    false,
                    &response_tx,
                    None,
                    &app_state,
                    &mut cx,
                )
                .await
            })
            .await;

        assert!(!errored);
    }
}

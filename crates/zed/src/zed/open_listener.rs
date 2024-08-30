use crate::restorable_workspace_locations;
use crate::{handle_open_request, init_headless, init_ui};
use anyhow::{anyhow, Context, Result};
use assistant::PromptBuilder;
use cli::{ipc, IpcHandshake};
use cli::{ipc::IpcSender, CliRequest, CliResponse};
use client::parse_zed_link;
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use editor::scroll::Autoscroll;
use editor::Editor;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::channel::{mpsc, oneshot};
use futures::{FutureExt, SinkExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, Global, WindowHandle};
use language::{Bias, Point};
use remote::SshConnectionOptions;
use std::sync::Arc;
use std::time::Duration;
use std::{process, thread};
use util::paths::PathWithPosition;
use util::ResultExt;
use welcome::{show_welcome_view, FIRST_OPEN};
use workspace::item::ItemHandle;
use workspace::{AppState, OpenOptions, Workspace};

#[derive(Default, Debug)]
pub struct OpenRequest {
    pub cli_connection: Option<(mpsc::Receiver<CliRequest>, IpcSender<CliResponse>)>,
    pub open_paths: Vec<PathWithPosition>,
    pub open_channel_notes: Vec<(u64, Option<String>)>,
    pub join_channel: Option<u64>,
    pub ssh_connection: Option<SshConnectionOptions>,
}

impl OpenRequest {
    pub fn parse(urls: Vec<String>, cx: &AppContext) -> Result<Self> {
        let mut this = Self::default();
        for url in urls {
            if let Some(server_name) = url.strip_prefix("zed-cli://") {
                this.cli_connection = Some(connect_to_cli(server_name)?);
            } else if let Some(file) = url.strip_prefix("file://") {
                this.parse_file_path(file)
            } else if let Some(file) = url.strip_prefix("zed://file") {
                this.parse_file_path(file)
            } else if url.starts_with("ssh://") {
                this.parse_ssh_file_path(&url)?
            } else if let Some(request_path) = parse_zed_link(&url, cx) {
                this.parse_request_path(request_path).log_err();
            } else {
                log::error!("unhandled url: {}", url);
            }
        }

        Ok(this)
    }

    fn parse_file_path(&mut self, file: &str) {
        if let Some(decoded) = urlencoding::decode(file).log_err() {
            let path_buf = PathWithPosition::parse_str(&decoded);
            self.open_paths.push(path_buf)
        }
    }

    fn parse_ssh_file_path(&mut self, file: &str) -> Result<()> {
        let url = url::Url::parse(file)?;
        let host = url
            .host()
            .ok_or_else(|| anyhow!("missing host in ssh url: {}", file))?
            .to_string();
        let username = Some(url.username().to_string()).filter(|s| !s.is_empty());
        let password = url.password().map(|s| s.to_string());
        let port = url.port();
        if !self.open_paths.is_empty() {
            return Err(anyhow!("cannot open both local and ssh paths"));
        }
        let connection = SshConnectionOptions {
            username,
            password,
            host,
            port,
        };
        if let Some(ssh_connection) = &self.ssh_connection {
            if *ssh_connection != connection {
                return Err(anyhow!("cannot open multiple ssh connections"));
            }
        }
        self.ssh_connection = Some(connection);
        self.parse_file_path(url.path());
        Ok(())
    }

    fn parse_request_path(&mut self, request_path: &str) -> Result<()> {
        let mut parts = request_path.split('/');
        if parts.next() == Some("channel") {
            if let Some(slug) = parts.next() {
                if let Some(id_str) = slug.split('-').last() {
                    if let Ok(channel_id) = id_str.parse::<u64>() {
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
                }
            }
        }
        Err(anyhow!("invalid zed url: {}", request_path))
    }
}

#[derive(Clone)]
pub struct OpenListener(UnboundedSender<Vec<String>>);

impl Global for OpenListener {}

impl OpenListener {
    pub fn new() -> (Self, UnboundedReceiver<Vec<String>>) {
        let (tx, rx) = mpsc::unbounded();
        (OpenListener(tx), rx)
    }

    pub fn open_urls(&self, urls: Vec<String>) {
        self.0
            .unbounded_send(urls)
            .map_err(|_| anyhow!("no listener for open requests"))
            .log_err();
    }
}

#[cfg(target_os = "linux")]
pub fn listen_for_cli_connections(opener: OpenListener) -> Result<()> {
    use release_channel::RELEASE_CHANNEL_NAME;
    use std::os::unix::net::UnixDatagram;

    let sock_path = paths::support_dir().join(format!("zed-{}.sock", *RELEASE_CHANNEL_NAME));
    // remove the socket if the process listening on it has died
    if let Err(e) = UnixDatagram::unbound()?.connect(&sock_path) {
        if e.kind() == std::io::ErrorKind::ConnectionRefused {
            std::fs::remove_file(&sock_path)?;
        }
    }
    let listener = UnixDatagram::bind(&sock_path)?;
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        while let Ok(len) = listener.recv(&mut buf) {
            opener.open_urls(vec![String::from_utf8_lossy(&buf[..len]).to_string()]);
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
        Ok::<_, anyhow::Error>(())
    });

    Ok((async_request_rx, response_tx))
}

pub async fn open_paths_with_positions(
    path_positions: &Vec<PathWithPosition>,
    app_state: Arc<AppState>,
    open_options: workspace::OpenOptions,
    cx: &mut AsyncAppContext,
) -> Result<(
    WindowHandle<Workspace>,
    Vec<Option<Result<Box<dyn ItemHandle>>>>,
)> {
    let mut caret_positions = HashMap::default();

    let paths = path_positions
        .iter()
        .map(|path_with_position| {
            let path = path_with_position.path.clone();
            if let Some(row) = path_with_position.row {
                if path.is_file() {
                    let row = row.saturating_sub(1);
                    let col = path_with_position.column.unwrap_or(0).saturating_sub(1);
                    caret_positions.insert(path.clone(), Point::new(row, col));
                }
            }
            path
        })
        .collect::<Vec<_>>();

    let (workspace, items) = cx
        .update(|cx| workspace::open_paths(&paths, app_state, open_options, cx))?
        .await?;

    for (item, path) in items.iter().zip(&paths) {
        let Some(Ok(item)) = item else {
            continue;
        };
        let Some(point) = caret_positions.remove(path) else {
            continue;
        };
        if let Some(active_editor) = item.downcast::<Editor>() {
            workspace
                .update(cx, |_, cx| {
                    active_editor.update(cx, |editor, cx| {
                        let snapshot = editor.snapshot(cx).display_snapshot;
                        let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
                        editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                            s.select_ranges([point..point])
                        });
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
    prompt_builder: Arc<PromptBuilder>,
    mut cx: AsyncAppContext,
) {
    if let Some(request) = requests.next().await {
        match request {
            CliRequest::Open {
                urls,
                paths,
                wait,
                open_new_workspace,
                dev_server_token,
                env,
            } => {
                if let Some(dev_server_token) = dev_server_token {
                    match cx
                        .update(|cx| {
                            init_headless(client::DevServerToken(dev_server_token), app_state, cx)
                        })
                        .unwrap()
                        .await
                    {
                        Ok(_) => {
                            responses
                                .send(CliResponse::Stdout {
                                    message: format!("zed (pid {}) connected!", process::id()),
                                })
                                .log_err();
                            responses.send(CliResponse::Exit { status: 0 }).log_err();
                        }
                        Err(error) => {
                            responses
                                .send(CliResponse::Stderr {
                                    message: format!("{error}"),
                                })
                                .log_err();
                            responses.send(CliResponse::Exit { status: 1 }).log_err();
                            cx.update(|cx| cx.quit()).log_err();
                        }
                    }
                    return;
                }

                if !urls.is_empty() {
                    cx.update(|cx| {
                        match OpenRequest::parse(urls, cx) {
                            Ok(open_request) => {
                                handle_open_request(
                                    open_request,
                                    app_state.clone(),
                                    prompt_builder.clone(),
                                    cx,
                                );
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

                if let Err(e) = cx
                    .update(|cx| init_ui(app_state.clone(), prompt_builder.clone(), cx))
                    .and_then(|r| r)
                {
                    responses
                        .send(CliResponse::Stderr {
                            message: format!("{e}"),
                        })
                        .log_err();
                    responses.send(CliResponse::Exit { status: 1 }).log_err();
                    return;
                }

                let open_workspace_result = open_workspaces(
                    paths,
                    open_new_workspace,
                    &responses,
                    wait,
                    app_state.clone(),
                    env,
                    &mut cx,
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
    open_new_workspace: Option<bool>,
    responses: &IpcSender<CliResponse>,
    wait: bool,
    app_state: Arc<AppState>,
    env: Option<collections::HashMap<String, String>>,
    mut cx: &mut AsyncAppContext,
) -> Result<()> {
    let grouped_paths = if paths.is_empty() {
        // If no paths are provided, restore from previous workspaces unless a new workspace is requested with -n
        if open_new_workspace == Some(true) {
            Vec::new()
        } else {
            let locations = restorable_workspace_locations(&mut cx, &app_state).await;
            locations
                .into_iter()
                .flat_map(|locations| {
                    locations
                        .into_iter()
                        .map(|location| {
                            location
                                .paths()
                                .iter()
                                .map(|path| PathWithPosition {
                                    path: path.clone(),
                                    row: None,
                                    column: None,
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        }
    } else {
        // If paths are provided, parse them (they include positions)
        let paths_with_position = paths
            .into_iter()
            .map(|path_with_position_string| {
                PathWithPosition::parse_str(&path_with_position_string)
            })
            .collect();
        vec![paths_with_position]
    };

    if grouped_paths.is_empty() {
        // If we have no paths to open, show the welcome screen if this is the first launch
        if matches!(KEY_VALUE_STORE.read_kvp(FIRST_OPEN), Ok(None)) {
            cx.update(|cx| show_welcome_view(app_state, cx).detach())
                .log_err();
        }
        // If not the first launch, show an empty window with empty editor
        else {
            cx.update(|cx| {
                let open_options = OpenOptions {
                    env,
                    ..Default::default()
                };
                workspace::open_new(open_options, app_state, cx, |workspace, cx| {
                    Editor::new_file(workspace, &Default::default(), cx)
                })
                .detach();
            })
            .log_err();
        }
    } else {
        // If there are paths to open, open a workspace for each grouping of paths
        let mut errored = false;

        for workspace_paths in grouped_paths {
            let workspace_failed_to_open = open_workspace(
                workspace_paths,
                open_new_workspace,
                wait,
                responses,
                env.as_ref(),
                &app_state,
                &mut cx,
            )
            .await;

            if workspace_failed_to_open {
                errored = true
            }
        }

        if errored {
            return Err(anyhow!("failed to open a workspace"));
        }
    }

    Ok(())
}

async fn open_workspace(
    workspace_paths: Vec<PathWithPosition>,
    open_new_workspace: Option<bool>,
    wait: bool,
    responses: &IpcSender<CliResponse>,
    env: Option<&HashMap<String, String>>,
    app_state: &Arc<AppState>,
    cx: &mut AsyncAppContext,
) -> bool {
    let mut errored = false;

    match open_paths_with_positions(
        &workspace_paths,
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

            for (item, path) in items.into_iter().zip(&workspace_paths) {
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
                                message: format!("error opening {path:?}: {err}"),
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
                    if workspace_paths.is_empty() {
                        let (done_tx, done_rx) = oneshot::channel();
                        let _subscription = workspace.update(cx, |_, cx| {
                            cx.on_release(move |_, _, _| {
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
                    message: format!("error opening {workspace_paths:?}: {error}"),
                })
                .log_err();
        }
    }
    errored
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use cli::{
        ipc::{self},
        CliResponse,
    };
    use editor::Editor;
    use gpui::TestAppContext;
    use serde_json::json;
    use util::paths::PathWithPosition;
    use workspace::{AppState, Workspace};

    use crate::zed::{open_listener::open_workspace, tests::init_test};

    #[gpui::test]
    async fn test_open_workspace_with_directory(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
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
        open_workspace_file("/root/dir1", None, app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 1);
        let workspace = cx.windows()[0].downcast::<Workspace>().unwrap();
        workspace
            .update(cx, |workspace, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_none())
            })
            .unwrap();

        // Now open a file inside that workspace
        open_workspace_file("/root/dir1/file1.txt", None, app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 1);
        workspace
            .update(cx, |workspace, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_some());
            })
            .unwrap();

        // Now open a file inside that workspace, but tell Zed to open a new window
        open_workspace_file("/root/dir1/file1.txt", Some(true), app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 2);

        let workspace_2 = cx.windows()[1].downcast::<Workspace>().unwrap();
        workspace_2
            .update(cx, |workspace, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_some());
                let items = workspace.items(cx).collect::<Vec<_>>();
                assert_eq!(items.len(), 1, "Workspace should have two items");
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_workspace_with_nonexistent_files(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state.fs.as_fake().insert_tree("/root", json!({})).await;

        assert_eq!(cx.windows().len(), 0);

        // Test case 1: Open a single file that does not exist yet
        open_workspace_file("/root/file5.txt", None, app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 1);
        let workspace_1 = cx.windows()[0].downcast::<Workspace>().unwrap();
        workspace_1
            .update(cx, |workspace, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_some())
            })
            .unwrap();

        // Test case 2: Open a single file that does not exist yet,
        // but tell Zed to add it to the current workspace
        open_workspace_file("/root/file6.txt", Some(false), app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 1);
        workspace_1
            .update(cx, |workspace, cx| {
                let items = workspace.items(cx).collect::<Vec<_>>();
                assert_eq!(items.len(), 2, "Workspace should have two items");
            })
            .unwrap();

        // Test case 3: Open a single file that does not exist yet,
        // but tell Zed to NOT add it to the current workspace
        open_workspace_file("/root/file7.txt", Some(true), app_state.clone(), cx).await;

        assert_eq!(cx.windows().len(), 2);
        let workspace_2 = cx.windows()[1].downcast::<Workspace>().unwrap();
        workspace_2
            .update(cx, |workspace, cx| {
                let items = workspace.items(cx).collect::<Vec<_>>();
                assert_eq!(items.len(), 1, "Workspace should have two items");
            })
            .unwrap();
    }

    async fn open_workspace_file(
        path: &str,
        open_new_workspace: Option<bool>,
        app_state: Arc<AppState>,
        cx: &mut TestAppContext,
    ) {
        let (response_tx, _) = ipc::channel::<CliResponse>().unwrap();

        let path = PathBuf::from(path);
        let workspace_paths = vec![PathWithPosition {
            path,
            row: None,
            column: None,
        }];

        let errored = cx
            .spawn(|mut cx| async move {
                open_workspace(
                    workspace_paths,
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

use anyhow::{anyhow, Context, Result};
use cli::{ipc, IpcHandshake};
use cli::{ipc::IpcSender, CliRequest, CliResponse};
use client::parse_zed_link;
use collections::HashMap;
use editor::scroll::Autoscroll;
use editor::Editor;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::channel::{mpsc, oneshot};
use futures::{FutureExt, SinkExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, Global, WindowHandle};
use language::{Bias, Point};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use util::paths::PathLikeWithPosition;
use util::ResultExt;
use workspace::item::ItemHandle;
use workspace::{AppState, Workspace};

#[derive(Default, Debug)]
pub struct OpenRequest {
    pub cli_connection: Option<(mpsc::Receiver<CliRequest>, IpcSender<CliResponse>)>,
    pub open_paths: Vec<PathLikeWithPosition<PathBuf>>,
    pub open_channel_notes: Vec<(u64, Option<String>)>,
    pub join_channel: Option<u64>,
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
            if let Some(path_buf) =
                PathLikeWithPosition::parse_str(&decoded, |s| PathBuf::try_from(s)).log_err()
            {
                self.open_paths.push(path_buf)
            }
        }
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

pub struct OpenListener {
    tx: UnboundedSender<Vec<String>>,
}

struct GlobalOpenListener(Arc<OpenListener>);

impl Global for GlobalOpenListener {}

impl OpenListener {
    pub fn global(cx: &AppContext) -> Arc<Self> {
        cx.global::<GlobalOpenListener>().0.clone()
    }

    pub fn set_global(listener: Arc<OpenListener>, cx: &mut AppContext) {
        cx.set_global(GlobalOpenListener(listener))
    }

    pub fn new() -> (Self, UnboundedReceiver<Vec<String>>) {
        let (tx, rx) = mpsc::unbounded();
        (OpenListener { tx }, rx)
    }

    pub fn open_urls(&self, urls: Vec<String>) {
        self.tx
            .unbounded_send(urls)
            .map_err(|_| anyhow!("no listener for open requests"))
            .log_err();
    }
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
    path_likes: &Vec<PathLikeWithPosition<PathBuf>>,
    app_state: Arc<AppState>,
    open_options: workspace::OpenOptions,
    cx: &mut AsyncAppContext,
) -> Result<(
    WindowHandle<Workspace>,
    Vec<Option<Result<Box<dyn ItemHandle>>>>,
)> {
    let mut caret_positions = HashMap::default();

    let paths = path_likes
        .iter()
        .map(|path_with_position| {
            let path = path_with_position.path_like.clone();
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
    mut cx: AsyncAppContext,
) {
    if let Some(request) = requests.next().await {
        match request {
            CliRequest::Open {
                paths,
                wait,
                open_new_workspace,
            } => {
                let paths = if paths.is_empty() {
                    if open_new_workspace == Some(true) {
                        vec![]
                    } else {
                        workspace::last_opened_workspace_paths()
                            .await
                            .map(|location| {
                                location
                                    .paths()
                                    .iter()
                                    .map(|path| PathLikeWithPosition {
                                        path_like: path.clone(),
                                        row: None,
                                        column: None,
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    }
                } else {
                    paths
                        .into_iter()
                        .map(|path_with_position_string| {
                            PathLikeWithPosition::parse_str(
                                &path_with_position_string,
                                |path_str| {
                                    Ok::<_, std::convert::Infallible>(
                                        Path::new(path_str).to_path_buf(),
                                    )
                                },
                            )
                            .expect("Infallible")
                        })
                        .collect()
                };

                let mut errored = false;

                match open_paths_with_positions(
                    &paths,
                    app_state,
                    workspace::OpenOptions {
                        open_new_workspace,
                        ..Default::default()
                    },
                    &mut cx,
                )
                .await
                {
                    Ok((workspace, items)) => {
                        let mut item_release_futures = Vec::new();

                        for (item, path) in items.into_iter().zip(&paths) {
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
                                            message: format!("error opening {:?}: {}", path, err),
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
                                if paths.is_empty() {
                                    let (done_tx, done_rx) = oneshot::channel();
                                    let _subscription = workspace.update(&mut cx, |_, cx| {
                                        cx.on_release(move |_, _, _| {
                                            let _ = done_tx.send(());
                                        })
                                    });
                                    let _ = done_rx.await;
                                } else {
                                    let _ =
                                        futures::future::try_join_all(item_release_futures).await;
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
                                message: format!("error opening {:?}: {}", paths, error),
                            })
                            .log_err();
                    }
                }

                responses
                    .send(CliResponse::Exit {
                        status: i32::from(errored),
                    })
                    .log_err();
            }
        }
    }
}

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
use gpui::{AppContext, AsyncAppContext, Global};
use itertools::Itertools;
use language::{Bias, Point};
use std::path::Path;
use std::str::pattern::Pattern;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{path::PathBuf, sync::atomic::AtomicBool};
use util::paths::{PathExt, PathLikeWithPosition};
use util::ResultExt;
use workspace::AppState;

#[derive(Default)]
pub struct OpenDetails {
    cli_connection: Option<(mpsc::Receiver<CliRequest>, IpcSender<CliResponse>)>,
    open_paths: Vec<PathBuf>,
    open_channel_notes: Vec<(u64, Option<String>)>,
    join_channel: Option<u64>,
}

impl OpenRequest {
    pub fn parse(urls: Vec<String>, cx: &AppContext) -> Result<Self> {
        let mut this = Self::default();
        for url in urls {
            if let Some(server_name) = url.strip_prefix("zed-cli://") {
                this.cli_connection = Some(connect_to_cli(server_name)?);
            } else if let Some(file) = url.strip_prefix("file://") {
                let decoded = urlencoding::decode_binary(url.as_bytes());
                if let Some(path_buf) = PathBuf::try_from_bytes(decoded.as_ref()).log_err() {
                    this.open_paths.push(path_buf)
                }
            } else if let Some(file) = url.strip_prefix("zed://file") {
                let decoded = urlencoding::decode_binary(url.as_bytes());
                if let Some(path_buf) = PathBuf::try_from_bytes(decoded.as_ref()).log_err() {
                    this.open_paths.push(path_buf)
                }
            } else if let Some(request_path) = parse_zed_link(&url, cx) {
                this.parse_request_path(request_path).log_err();
            } else {
                log::error!("unhandled url: {}", url);
            }
        }

        Ok(this)
    }

    fn parse_request_path(&mut self, request_path: &str) -> Result<OpenRequest> {
        let mut parts = request_path.split('/');
        if parts.next() == Some("channel") {
            if let Some(slug) = parts.next() {
                if let Some(id_str) = slug.split('-').last() {
                    if let Ok(channel_id) = id_str.parse::<u64>() {
                        let Some(next) = parts.next() else {
                            self.join_channel = Some(channel_id)
                        };

                        if let Some(heading) = next.strip_prefix("notes#") {
                            self.open_channel_notes
                                .push((channel_id, Some(heading.to_string())));
                        } else if next == "notes" {
                            self.open_channel_notes.push((channel_id, None))
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
    pub triggered: AtomicBool,
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
        (
            OpenListener {
                tx,
                triggered: AtomicBool::new(false),
            },
            rx,
        )
    }

    pub fn open_urls(&self, urls: Vec<String>) {
        self.triggered.store(true, Ordering::Release);
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

pub async fn handle_cli_connection(
    (mut requests, responses): (mpsc::Receiver<CliRequest>, IpcSender<CliResponse>),
    app_state: Arc<AppState>,
    mut cx: AsyncAppContext,
) {
    if let Some(request) = requests.next().await {
        match request {
            CliRequest::Open { paths, wait } => {
                let mut caret_positions = HashMap::default();

                let paths = if paths.is_empty() {
                    workspace::last_opened_workspace_paths()
                        .await
                        .map(|location| location.paths().to_vec())
                        .unwrap_or_default()
                } else {
                    paths
                        .into_iter()
                        .map(|path_with_position_string| {
                            let path_with_position = PathLikeWithPosition::parse_str(
                                &path_with_position_string,
                                |path_str| {
                                    Ok::<_, std::convert::Infallible>(
                                        Path::new(path_str).to_path_buf(),
                                    )
                                },
                            )
                            .expect("Infallible");
                            let path = path_with_position.path_like;
                            if let Some(row) = path_with_position.row {
                                if path.is_file() {
                                    let row = row.saturating_sub(1);
                                    let col =
                                        path_with_position.column.unwrap_or(0).saturating_sub(1);
                                    caret_positions.insert(path.clone(), Point::new(row, col));
                                }
                            }
                            path
                        })
                        .collect()
                };

                let mut errored = false;

                match cx.update(|cx| workspace::open_paths(&paths, app_state, None, cx)) {
                    Ok(task) => match task.await {
                        Ok((workspace, items)) => {
                            let mut item_release_futures = Vec::new();

                            for (item, path) in items.into_iter().zip(&paths) {
                                match item {
                                    Some(Ok(item)) => {
                                        if let Some(point) = caret_positions.remove(path) {
                                            if let Some(active_editor) = item.downcast::<Editor>() {
                                                workspace
                                                    .update(&mut cx, |_, cx| {
                                                        active_editor.update(cx, |editor, cx| {
                                                            let snapshot = editor
                                                                .snapshot(cx)
                                                                .display_snapshot;
                                                            let point = snapshot
                                                                .buffer_snapshot
                                                                .clip_point(point, Bias::Left);
                                                            editor.change_selections(
                                                                Some(Autoscroll::center()),
                                                                cx,
                                                                |s| s.select_ranges([point..point]),
                                                            );
                                                        });
                                                    })
                                                    .log_err();
                                            }
                                        }

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
                                                message: format!(
                                                    "error opening {:?}: {}",
                                                    path, err
                                                ),
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
                                        let _ = futures::future::try_join_all(item_release_futures)
                                            .await;
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
                    },
                    Err(_) => errored = true,
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

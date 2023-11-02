mod assets;
pub mod languages;
mod only_instance;
mod open_listener;

pub use assets::*;
use client::{Client, UserStore};
use gpui::{AsyncAppContext, Model};
pub use only_instance::*;
pub use open_listener::*;

use anyhow::{Context, Result};
use cli::{
    ipc::{self, IpcSender},
    CliRequest, CliResponse, IpcHandshake,
};
use futures::{channel::mpsc, SinkExt, StreamExt};
use std::{sync::Arc, thread};

pub fn connect_to_cli(
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

pub struct AppState {
    pub client: Arc<Client>,
    pub user_store: Model<UserStore>,
}

pub async fn handle_cli_connection(
    (mut requests, _responses): (mpsc::Receiver<CliRequest>, IpcSender<CliResponse>),
    _app_state: Arc<AppState>,
    mut _cx: AsyncAppContext,
) {
    if let Some(request) = requests.next().await {
        match request {
            CliRequest::Open { paths: _, wait: _ } => {
                // let mut caret_positions = HashMap::new();

                // let paths = if paths.is_empty() {
                // todo!()
                // workspace::last_opened_workspace_paths()
                //     .await
                //     .map(|location| location.paths().to_vec())
                //     .unwrap_or_default()
                // } else {
                //     paths
                //         .into_iter()
                //         .filter_map(|path_with_position_string| {
                //             let path_with_position = PathLikeWithPosition::parse_str(
                //                 &path_with_position_string,
                //                 |path_str| {
                //                     Ok::<_, std::convert::Infallible>(
                //                         Path::new(path_str).to_path_buf(),
                //                     )
                //                 },
                //             )
                //             .expect("Infallible");
                //             let path = path_with_position.path_like;
                //             if let Some(row) = path_with_position.row {
                //                 if path.is_file() {
                //                     let row = row.saturating_sub(1);
                //                     let col =
                //                         path_with_position.column.unwrap_or(0).saturating_sub(1);
                //                     caret_positions.insert(path.clone(), Point::new(row, col));
                //                 }
                //             }
                //             Some(path)
                //         })
                //         .collect()
                // };

                // let mut errored = false;
                // todo!("workspace")
                // match cx
                //     .update(|cx| workspace::open_paths(&paths, &app_state, None, cx))
                //     .await
                // {
                //     Ok((workspace, items)) => {
                //         let mut item_release_futures = Vec::new();

                //         for (item, path) in items.into_iter().zip(&paths) {
                //             match item {
                //                 Some(Ok(item)) => {
                //                     if let Some(point) = caret_positions.remove(path) {
                //                         if let Some(active_editor) = item.downcast::<Editor>() {
                //                             active_editor
                //                                 .downgrade()
                //                                 .update(&mut cx, |editor, cx| {
                //                                     let snapshot =
                //                                         editor.snapshot(cx).display_snapshot;
                //                                     let point = snapshot
                //                                         .buffer_snapshot
                //                                         .clip_point(point, Bias::Left);
                //                                     editor.change_selections(
                //                                         Some(Autoscroll::center()),
                //                                         cx,
                //                                         |s| s.select_ranges([point..point]),
                //                                     );
                //                                 })
                //                                 .log_err();
                //                         }
                //                     }

                //                     let released = oneshot::channel();
                //                     cx.update(|cx| {
                //                         item.on_release(
                //                             cx,
                //                             Box::new(move |_| {
                //                                 let _ = released.0.send(());
                //                             }),
                //                         )
                //                         .detach();
                //                     });
                //                     item_release_futures.push(released.1);
                //                 }
                //                 Some(Err(err)) => {
                //                     responses
                //                         .send(CliResponse::Stderr {
                //                             message: format!("error opening {:?}: {}", path, err),
                //                         })
                //                         .log_err();
                //                     errored = true;
                //                 }
                //                 None => {}
                //             }
                //         }

                //         if wait {
                //             let background = cx.background();
                //             let wait = async move {
                //                 if paths.is_empty() {
                //                     let (done_tx, done_rx) = oneshot::channel();
                //                     if let Some(workspace) = workspace.upgrade(&cx) {
                //                         let _subscription = cx.update(|cx| {
                //                             cx.observe_release(&workspace, move |_, _| {
                //                                 let _ = done_tx.send(());
                //                             })
                //                         });
                //                         drop(workspace);
                //                         let _ = done_rx.await;
                //                     }
                //                 } else {
                //                     let _ =
                //                         futures::future::try_join_all(item_release_futures).await;
                //                 };
                //             }
                //             .fuse();
                //             futures::pin_mut!(wait);

                //             loop {
                //                 // Repeatedly check if CLI is still open to avoid wasting resources
                //                 // waiting for files or workspaces to close.
                //                 let mut timer = background.timer(Duration::from_secs(1)).fuse();
                //                 futures::select_biased! {
                //                     _ = wait => break,
                //                     _ = timer => {
                //                         if responses.send(CliResponse::Ping).is_err() {
                //                             break;
                //                         }
                //                     }
                //                 }
                //             }
                //         }
                //     }
                //     Err(error) => {
                //         errored = true;
                //         responses
                //             .send(CliResponse::Stderr {
                //                 message: format!("error opening {:?}: {}", paths, error),
                //             })
                //             .log_err();
                //     }
                // }

                // responses
                //     .send(CliResponse::Exit {
                //         status: i32::from(errored),
                //     })
                //     .log_err();
            }
        }
    }
}

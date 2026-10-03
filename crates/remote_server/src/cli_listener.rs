//! Unix socket bridge from remote terminals to the connected workspace.

use crate::cli_client::{CliRequest, CliResponse, read_frame};
use anyhow::{Context as _, Result};
use futures::{AsyncReadExt as _, AsyncWriteExt as _, FutureExt as _};
use gpui::App;
use net::async_net::{UnixListener, UnixStream};
use rpc::AnyProtoClient;
use rpc::proto::{self, REMOTE_SERVER_PROJECT_ID};
use std::path::{Path, PathBuf};
use std::time::Duration;
use util::{ResultExt as _, paths::PathWithPosition};

pub fn start_cli_listener(socket_path: PathBuf, session: AnyProtoClient, cx: &mut App) {
    cx.spawn(async move |_cx| {
        run_cli_listener(&socket_path, &session).await.log_err();
    })
    .detach();
}

async fn run_cli_listener(socket_path: &Path, session: &AnyProtoClient) -> Result<()> {
    if socket_path.exists() {
        std::fs::remove_file(socket_path)?;
    }
    let listener = UnixListener::bind(socket_path).context("failed to bind CLI listener socket")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o600))?;
    }
    loop {
        let (stream, _) = listener.accept().await?;
        let session = session.clone();
        smol::spawn(async move {
            handle_cli_connection(stream, &session).await.log_err();
        })
        .detach();
    }
}

async fn write_response(
    writer: &mut (impl futures::AsyncWrite + Unpin),
    response: CliResponse,
) -> Result<()> {
    let mut message = serde_json::to_vec(&response)?;
    message.push(b'\n');
    let write = async {
        writer.write_all(&message).await?;
        writer.flush().await?;
        anyhow::Ok(())
    }
    .fuse();
    let timeout = smol::Timer::after(Duration::from_secs(5)).fuse();
    futures::pin_mut!(write, timeout);
    futures::select_biased! {
        result = write => result,
        _ = timeout => anyhow::bail!("CLI stopped reading responses"),
    }
}

fn resolve_paths(request: &CliRequest) -> Result<Vec<PathWithPosition>> {
    anyhow::ensure!(
        request.cwd.is_absolute(),
        "CLI working directory must be absolute"
    );
    anyhow::ensure!(!request.paths.is_empty(), "no paths supplied");
    request
        .paths
        .iter()
        .map(|path| {
            let original = request.cwd.join(path);
            let mut parsed = if original.exists() {
                PathWithPosition::from_path(original)
            } else {
                let mut parsed = PathWithPosition::parse_str(path);
                parsed.path = request.cwd.join(parsed.path);
                parsed
            };
            // Canonicalize existing paths on the remote host, never on the GUI host.
            if parsed.path.exists() {
                parsed.path = parsed.path.canonicalize()?;
            }
            Ok(parsed)
        })
        .collect()
}

async fn handle_cli_connection(stream: UnixStream, session: &AnyProtoClient) -> Result<()> {
    let (reader, mut writer) = stream.split();
    let mut reader = smol::io::BufReader::new(reader);
    let request = async {
        let read = read_frame(&mut reader).fuse();
        let timeout = smol::Timer::after(Duration::from_secs(15)).fuse();
        futures::pin_mut!(read, timeout);
        let line = futures::select_biased! {
            result = read => result?,
            _ = timeout => anyhow::bail!("Timed out reading CLI request"),
        };
        let request: CliRequest = serde_json::from_str(&line)?;
        let paths = resolve_paths(&request)?;
        anyhow::Ok((request, paths))
    }
    .await;
    let (request, paths) = match request {
        Ok(request) => request,
        Err(error) => {
            return write_response(&mut writer, CliResponse::Error(error.to_string())).await;
        }
    };
    let request_ids = paths
        .iter()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    let requests = paths
        .into_iter()
        .zip(&request_ids)
        .map(|(path, request_id)| {
            session.request(proto::OpenPathOnClient {
                project_id: REMOTE_SERVER_PROJECT_ID,
                is_directory: path.path.is_dir(),
                path: path.path.to_string_lossy().into_owned(),
                row: path.row,
                column: path.column,
                wait: request.wait,
                request_id: request_id.clone(),
            })
        });
    let result = async {
        let requests = futures::future::try_join_all(requests).fuse();
        let mut disconnect_buffer = [0];
        let disconnected = reader.read(&mut disconnect_buffer).fuse();
        futures::pin_mut!(requests, disconnected);
        loop {
            let heartbeat = smol::Timer::after(Duration::from_secs(1)).fuse();
            futures::pin_mut!(heartbeat);
            futures::select_biased! {
                responses = requests => {
                    for response in responses? {
                        anyhow::ensure!(response.success, "Failed to open path in the connected workspace");
                    }
                    return Ok(());
                },
                _ = disconnected => anyhow::bail!("CLI disconnected"),
                _ = heartbeat => {
                    let ping = session.request(proto::Ping {}).fuse();
                    let timeout = smol::Timer::after(Duration::from_secs(5)).fuse();
                    futures::pin_mut!(ping, timeout);
                    futures::select_biased! {
                        result = ping => { result.context("Remote workspace disconnected")?; },
                        _ = timeout => anyhow::bail!("Remote workspace stopped responding"),
                    }
                    write_response(&mut writer, CliResponse::Ping).await?;
                }
            }
        }
    }.await;
    if result.is_err() {
        for request_id in request_ids {
            session
                .send(proto::CancelOpenPathOnClient {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                    request_id,
                })
                .log_err();
        }
    }
    write_response(
        &mut writer,
        match result {
            Ok(()) => CliResponse::Complete,
            Err(error) => CliResponse::Error(format!("{error:#}")),
        },
    )
    .await
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn resolves_relative_paths_in_client_directory_with_position() {
        let request = CliRequest {
            paths: vec!["src/árvíz file.rs:42:7".into()],
            cwd: PathBuf::from("/remote/project"),
            wait: false,
        };
        let paths = resolve_paths(&request).unwrap();
        assert_eq!(
            paths[0].path,
            Path::new("/remote/project/src/árvíz file.rs")
        );
        assert_eq!((paths[0].row, paths[0].column), (Some(42), Some(7)));
    }

    #[test]
    fn preserves_existing_filenames_that_end_in_numbers() {
        let directory = std::env::temp_dir().join(format!("zed-cli-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("file:42"), "").unwrap();
        let paths = resolve_paths(&CliRequest {
            paths: vec!["file:42".into(), ".".into(), "new file\nλ.rs".into()],
            cwd: directory.clone(),
            wait: false,
        })
        .unwrap();
        assert_eq!(
            paths[0].path,
            directory.join("file:42").canonicalize().unwrap()
        );
        assert_eq!(paths[0].row, None);
        assert_eq!(paths[1].path, directory.canonicalize().unwrap());
        assert_eq!(paths[2].path, directory.join("new file\nλ.rs"));
        std::fs::remove_dir_all(directory).unwrap();
    }
}

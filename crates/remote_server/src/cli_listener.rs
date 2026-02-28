//! CLI socket listener for the remote server.
//!
//! This module provides functionality for accepting file-open requests from a local
//! CLI tool on the remote machine and forwarding them to the connected Zed client.
//! This enables `zed <file>` to work from within remote terminal sessions.

use anyhow::{Context as _, Result};
use futures::AsyncWriteExt;
use gpui::App;
use net::async_net::{UnixListener, UnixStream};
use rpc::AnyProtoClient;
use rpc::proto::{self, REMOTE_SERVER_PROJECT_ID};
use smol::io::AsyncBufReadExt;
use std::path::{Path, PathBuf};

/// Start listening on a Unix socket for CLI open requests.
///
/// When a CLI client connects and sends a path (with optional line:column),
/// this listener sends an `OpenPathOnClient` request over the RPC channel
/// to the local Zed client, which opens the file in the editor.
pub fn start_cli_listener(socket_path: PathBuf, session: AnyProtoClient, cx: &mut App) {
    cx.spawn(async move |_cx| {
        if let Err(e) = run_cli_listener(&socket_path, &session).await {
            log::error!("CLI listener error: {e:#}");
        }
    })
    .detach();
}

async fn run_cli_listener(socket_path: &Path, session: &AnyProtoClient) -> Result<()> {
    if socket_path.exists() {
        std::fs::remove_file(socket_path).ok();
    }

    let listener = UnixListener::bind(socket_path).context("failed to bind CLI listener socket")?;
    log::info!("CLI listener started on {:?}", socket_path);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let session = session.clone();
                smol::spawn(async move {
                    if let Err(e) = handle_cli_connection(stream, &session).await {
                        log::warn!("CLI connection error: {e:#}");
                    }
                })
                .detach();
            }
            Err(e) => {
                log::error!("CLI listener accept error: {e:#}");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_cli_connection(stream: UnixStream, session: &AnyProtoClient) -> Result<()> {
    let (reader, mut writer) = futures::AsyncReadExt::split(stream);
    let mut buf_reader = smol::io::BufReader::new(reader);
    let mut line = String::new();
    buf_reader.read_line(&mut line).await?;
    let line = line.trim();

    if line.is_empty() {
        anyhow::bail!("empty request from CLI client");
    }

    let request = parse_cli_request(line)?;
    log::info!(
        "CLI open request: path={}, row={:?}, column={:?}, wait={}",
        request.path,
        request.row,
        request.column,
        request.wait
    );

    let response = session
        .request(proto::OpenPathOnClient {
            project_id: REMOTE_SERVER_PROJECT_ID,
            path: request.path,
            row: request.row,
            column: request.column,
            wait: request.wait,
        })
        .await;

    match response {
        Ok(resp) => {
            let status = if resp.success { "ok" } else { "error" };
            writer
                .write_all(format!("{status}\n").as_bytes())
                .await
                .ok();
        }
        Err(e) => {
            writer
                .write_all(format!("error: {e}\n").as_bytes())
                .await
                .ok();
        }
    }

    Ok(())
}

struct CliRequest {
    path: String,
    row: Option<u32>,
    column: Option<u32>,
    wait: bool,
}

/// Parse a CLI request line in the format: `[--wait] path[:line[:column]]`
fn parse_cli_request(input: &str) -> Result<CliRequest> {
    let mut wait = false;
    let mut path_str = input;

    if let Some(rest) = input.strip_prefix("--wait ") {
        wait = true;
        path_str = rest.trim();
    }

    let (path, row, column) = parse_path_with_position(path_str);

    // Resolve to absolute path
    let abs_path = if Path::new(&path).is_absolute() {
        path
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(&path).to_string_lossy().into_owned())
            .unwrap_or(path)
    };

    Ok(CliRequest {
        path: abs_path,
        row,
        column,
        wait,
    })
}

/// Parse `path:line:column` syntax. Returns (path, optional_row, optional_column).
fn parse_path_with_position(input: &str) -> (String, Option<u32>, Option<u32>) {
    // Try to split from the right to find :line:column or :line
    // Be careful: on Windows, paths can start with C: so we need to handle that.
    let parts: Vec<&str> = input.rsplitn(3, ':').collect();

    match parts.as_slice() {
        [col_str, line_str, path] => {
            if let (Ok(line), Ok(col)) = (line_str.parse::<u32>(), col_str.parse::<u32>()) {
                if !path.is_empty() {
                    return (path.to_string(), Some(line), Some(col));
                }
            }
            (input.to_string(), None, None)
        }
        [line_str, path] => {
            if let Ok(line) = line_str.parse::<u32>() {
                if !path.is_empty() {
                    return (path.to_string(), Some(line), None);
                }
            }
            (input.to_string(), None, None)
        }
        _ => (input.to_string(), None, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path_with_position() {
        let (path, row, col) = parse_path_with_position("/home/user/file.rs");
        assert_eq!(path, "/home/user/file.rs");
        assert_eq!(row, None);
        assert_eq!(col, None);

        let (path, row, col) = parse_path_with_position("/home/user/file.rs:42");
        assert_eq!(path, "/home/user/file.rs");
        assert_eq!(row, Some(42));
        assert_eq!(col, None);

        let (path, row, col) = parse_path_with_position("/home/user/file.rs:42:10");
        assert_eq!(path, "/home/user/file.rs");
        assert_eq!(row, Some(42));
        assert_eq!(col, Some(10));

        let (path, row, col) = parse_path_with_position(".");
        assert_eq!(path, ".");
        assert_eq!(row, None);
        assert_eq!(col, None);
    }

    #[test]
    fn test_parse_cli_request() {
        let req = parse_cli_request("/home/user/file.rs:42").unwrap();
        assert_eq!(req.path, "/home/user/file.rs");
        assert_eq!(req.row, Some(42));
        assert!(!req.wait);

        let req = parse_cli_request("--wait /home/user/file.rs").unwrap();
        assert_eq!(req.path, "/home/user/file.rs");
        assert!(req.wait);
    }
}

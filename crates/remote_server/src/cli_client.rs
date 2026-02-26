//! CLI client for sending file-open requests to a running remote server.
//!
//! This is invoked when a user types `zed <file>` in a remote terminal session.
//! It connects to the remote server's CLI Unix socket and sends the path to open.

use anyhow::{Context as _, Result, bail};
use std::path::{Path, PathBuf};

/// Send an open request to the running remote server via its CLI socket.
///
/// Finds the CLI socket in one of three ways (in order):
/// 1. `ZED_REMOTE_CLI_SOCKET` environment variable (set by Zed's terminal integration)
/// 2. The `--identifier` argument (used to locate the socket in the server state dir)
/// 3. Auto-discovery: scan server state dir for running servers
pub fn execute_cli(path: String, identifier: String, wait: bool) -> Result<()> {
    let cli_socket = resolve_cli_socket(&identifier)?;

    if !cli_socket.exists() {
        bail!(
            "No running Zed remote server found. \
             Make sure a remote session is active."
        );
    }

    smol::block_on(send_cli_request(&cli_socket, &path, wait))
}

fn resolve_cli_socket(identifier: &str) -> Result<PathBuf> {
    // First, check ZED_REMOTE_CLI_SOCKET env var
    if let Ok(socket_path) = std::env::var("ZED_REMOTE_CLI_SOCKET") {
        let path = PathBuf::from(&socket_path);
        if path.exists() {
            return Ok(path);
        }
        log::warn!(
            "ZED_REMOTE_CLI_SOCKET is set to {socket_path} but socket does not exist, \
             falling back to identifier"
        );
    }

    // Second, use the identifier
    if !identifier.is_empty() {
        return Ok(paths::remote_server_state_dir()
            .join(identifier)
            .join("cli.sock"));
    }

    // Third, try to auto-discover by scanning server state dirs
    let state_dir = paths::remote_server_state_dir();
    if state_dir.exists() {
        for entry in std::fs::read_dir(state_dir)? {
            let entry = entry?;
            let cli_sock = entry.path().join("cli.sock");
            if cli_sock.exists() {
                log::info!("auto-discovered CLI socket at {:?}", cli_sock);
                return Ok(cli_sock);
            }
        }
    }

    bail!(
        "Could not find a Zed remote server CLI socket. \
         Make sure a remote session is active."
    )
}

async fn send_cli_request(socket_path: &Path, path: &str, wait: bool) -> Result<()> {
    use futures::AsyncWriteExt;
    use smol::io::AsyncBufReadExt;

    let stream = net::async_net::UnixStream::connect(socket_path)
        .await
        .context("failed to connect to remote server CLI socket")?;

    let (reader, mut writer) = futures::AsyncReadExt::split(stream);

    let request = if wait {
        format!("--wait {path}\n")
    } else {
        format!("{path}\n")
    };

    writer
        .write_all(request.as_bytes())
        .await
        .context("failed to send request")?;
    writer.flush().await.ok();
    writer.close().await.ok();

    // Read response
    let mut buf_reader = smol::io::BufReader::new(reader);
    let mut response = String::new();
    buf_reader.read_line(&mut response).await.ok();
    let response = response.trim();

    if response == "ok" {
        eprintln!("Opening in Zed...");
        Ok(())
    } else if response.starts_with("error") {
        bail!("Server returned: {response}");
    } else if response.is_empty() {
        Ok(())
    } else {
        bail!("Unexpected response: {response}");
    }
}

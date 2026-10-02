//! CLI client for a remote terminal's connected Zed workspace.

use anyhow::{Context as _, Result, bail};
use futures::{AsyncReadExt as _, AsyncWriteExt as _, FutureExt as _};
use serde::{Deserialize, Serialize};
use smol::io::AsyncBufReadExt as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CliRequest {
    pub paths: Vec<String>,
    pub cwd: PathBuf,
    pub wait: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum CliResponse {
    Ping,
    Complete,
    Error(String),
}

pub fn execute_cli(paths: Vec<String>, identifier: String, wait: bool) -> Result<()> {
    let socket = std::env::var_os("ZED_REMOTE_CLI_SOCKET").map(PathBuf::from);
    let identifier = if identifier.is_empty() {
        std::env::var("ZED_REMOTE_SESSION_ID").unwrap_or_default()
    } else {
        identifier
    };
    let cli_socket = resolve_cli_socket(socket, &identifier)?;
    let request = CliRequest {
        paths,
        cwd: std::env::current_dir().context("failed to read current directory")?,
        wait,
    };
    smol::block_on(send_cli_request(&cli_socket, &request))
}

fn resolve_cli_socket(socket: Option<PathBuf>, identifier: &str) -> Result<PathBuf> {
    let path = if let Some(socket) = socket {
        socket
    } else if !identifier.is_empty() {
        anyhow::ensure!(
            Path::new(identifier).components().count() == 1
                && matches!(
                    Path::new(identifier).components().next(),
                    Some(std::path::Component::Normal(_))
                ),
            "invalid remote session identifier"
        );
        paths::remote_server_state_dir()
            .join(identifier)
            .join("cli.sock")
    } else {
        bail!("No remote session selected. Run zed from a connected Zed terminal.");
    };
    anyhow::ensure!(
        path.is_absolute(),
        "remote CLI socket must be an absolute path"
    );
    anyhow::ensure!(
        path.exists(),
        "Remote session is no longer available: {}",
        path.display()
    );
    Ok(path)
}

pub(crate) async fn read_frame(
    reader: &mut (impl futures::AsyncBufRead + Unpin),
) -> Result<String> {
    let mut line = String::new();
    reader.take(1024 * 1024).read_line(&mut line).await?;
    anyhow::ensure!(
        !line.is_empty(),
        "Remote session closed before completing the request"
    );
    anyhow::ensure!(
        line.ends_with('\n'),
        "Remote CLI message is truncated or too large"
    );
    Ok(line)
}

async fn send_cli_request(socket_path: &Path, request: &CliRequest) -> Result<()> {
    let send = async {
        let stream = net::async_net::UnixStream::connect(socket_path)
            .await
            .context("failed to connect to remote server CLI socket")?;
        let (reader, mut writer) = stream.split();
        let mut message = serde_json::to_vec(request)?;
        message.push(b'\n');
        anyhow::ensure!(
            message.len() <= 1024 * 1024,
            "Remote CLI request is too large"
        );
        writer
            .write_all(&message)
            .await
            .context("failed to send request")?;
        writer.flush().await?;
        anyhow::Ok((reader, writer))
    }
    .fuse();
    let timeout = smol::Timer::after(Duration::from_secs(15)).fuse();
    futures::pin_mut!(send, timeout);
    let (reader, _writer) = futures::select_biased! {
        result = send => result?,
        _ = timeout => bail!("Timed out sending request to remote session"),
    };
    let mut reader = smol::io::BufReader::new(reader);
    loop {
        let read = read_frame(&mut reader).fuse();
        let timeout = smol::Timer::after(Duration::from_secs(15)).fuse();
        futures::pin_mut!(read, timeout);
        let response = futures::select_biased! {
            result = read => result.context("failed to read remote CLI response")?,
            _ = timeout => bail!("Remote session stopped responding"),
        };
        match serde_json::from_str(&response).context("invalid remote CLI response")? {
            CliResponse::Ping => {}
            CliResponse::Complete => return Ok(()),
            CliResponse::Error(error) => bail!("{error}"),
        }
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn rejects_eof_before_response() {
        let path = std::env::temp_dir().join(format!("zed-cli-{}.sock", uuid::Uuid::new_v4()));
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        let server = std::thread::spawn(move || {
            use std::io::BufRead;
            let (stream, _) = listener.accept().unwrap();
            let mut request = String::new();
            std::io::BufReader::new(stream)
                .read_line(&mut request)
                .unwrap();
        });
        let result = smol::block_on(send_cli_request(
            &path,
            &CliRequest {
                paths: vec!["/tmp/file".into()],
                cwd: "/tmp".into(),
                wait: false,
            },
        ));
        server.join().unwrap();
        std::fs::remove_file(path).unwrap();
        assert!(result.is_err(), "an unacknowledged request must fail");
    }
    fn exchange(response: &'static str) -> Result<()> {
        let path = std::env::temp_dir().join(format!("zed-cli-{}.sock", uuid::Uuid::new_v4()));
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        let server = std::thread::spawn(move || {
            use std::io::{BufRead, Write};
            let (mut stream, _) = listener.accept().unwrap();
            let mut line = String::new();
            std::io::BufReader::new(&stream)
                .read_line(&mut line)
                .unwrap();
            let request: CliRequest = serde_json::from_str(&line).unwrap();
            assert_eq!(request.paths, vec!["space λ.rs:2:3", "new\nfile"]);
            assert_eq!(request.cwd, Path::new("/remote/project"));
            assert!(request.wait);
            stream.write_all(response.as_bytes()).unwrap();
        });
        let result = smol::block_on(send_cli_request(
            &path,
            &CliRequest {
                paths: vec!["space λ.rs:2:3".into(), "new\nfile".into()],
                cwd: "/remote/project".into(),
                wait: true,
            },
        ));
        server.join().unwrap();
        std::fs::remove_file(path).unwrap();
        result
    }

    #[test]
    fn forwards_all_arguments_and_waits_past_heartbeats() {
        exchange("\"Ping\"\n\"Complete\"\n").unwrap();
    }

    #[test]
    fn propagates_workspace_failure() {
        let error = exchange("{\"Error\":\"permission denied\"}\n").unwrap_err();
        assert_eq!(error.to_string(), "permission denied");
    }

    #[test]
    fn rejects_truncated_completion() {
        assert!(exchange("\"Complete\"").is_err());
    }

    #[test]
    fn rejects_unrecognized_response() {
        assert!(exchange("ok\n").is_err());
    }

    #[test]
    fn does_not_guess_a_session() {
        assert!(resolve_cli_socket(None, "").is_err());
    }

    #[test]
    fn does_not_fall_back_from_stale_session_socket() {
        let path = std::env::temp_dir().join(uuid::Uuid::new_v4().to_string());
        assert!(resolve_cli_socket(Some(path), "another-session").is_err());
    }

    #[test]
    fn rejects_oversized_response() {
        smol::block_on(async {
            let mut reader = futures::io::Cursor::new(vec![b'x'; 1024 * 1024 + 1]);
            assert!(read_frame(&mut reader).await.is_err());
        });
    }
}

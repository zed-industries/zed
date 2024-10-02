#![cfg_attr(target_os = "windows", allow(unused, dead_code))]

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use fs::RealFs;
use futures::channel::mpsc;
use futures::{select, AsyncRead, AsyncWrite, FutureExt};
use gpui::Context as _;
use remote::ssh_session::ChannelClient;
use remote::{
    json_log::LogRecord,
    protocol::{read_message, write_message},
};
use remote_server::HeadlessProject;
use rpc::proto::Envelope;
use smol::Async;
#[cfg(not(windows))]
use smol::{io::AsyncWriteExt, net::unix::UnixListener, stream::StreamExt as _};
use std::{
    env,
    io::Write,
    mem,
    path::{Path, PathBuf},
    process,
    sync::Arc,
};

#[derive(Parser)]
#[command(disable_version_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(long)]
        log_file: PathBuf,
        #[arg(long)]
        pid_file: PathBuf,
        #[arg(long)]
        stdin_socket: PathBuf,
        #[arg(long)]
        stdout_socket: PathBuf,
    },
    Proxy,
    Version,
}

#[cfg(windows)]
fn main() {
    unimplemented!()
}

#[cfg(not(windows))]
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run {
            log_file,
            pid_file,
            stdin_socket,
            stdout_socket,
        }) => {
            init_logging(Some(log_file))?;
            execute_run(pid_file, stdin_socket, stdout_socket)
        }
        Some(Commands::Proxy) => {
            init_logging(None)?;
            execute_proxy("some-project".to_string())
        }
        Some(Commands::Version) => {
            eprintln!("{}", env!("ZED_PKG_VERSION"));
            Ok(())
        }
        None => {
            eprintln!("usage: remote <run|proxy|version>");
            process::exit(1);
        }
    }
}

#[cfg(not(windows))]
fn init_logging(log_file: Option<PathBuf>) -> Result<()> {
    if let Some(log_file) = log_file {
        let target = Box::new(if log_file.exists() {
            std::fs::OpenOptions::new()
                .append(true)
                .open(&log_file)
                .context("Failed to open log file in append mode")?
        } else {
            std::fs::File::create(&log_file).context("Failed to create log file")?
        });

        env_logger::Builder::from_default_env()
            .target(env_logger::Target::Pipe(target))
            .init();
    } else {
        env_logger::builder()
            .format(|buf, record| {
                serde_json::to_writer(&mut *buf, &LogRecord::new(record))?;
                buf.write_all(b"\n")?;
                Ok(())
            })
            .init();
    }
    Ok(())
}

#[cfg(not(windows))]
fn execute_run(pid_file: PathBuf, stdin_socket: PathBuf, stdout_socket: PathBuf) -> Result<()> {
    write_pid_file(&pid_file)
        .with_context(|| format!("failed to write pid file: {:?}", &pid_file))?;

    let stdin_listener = UnixListener::bind(stdin_socket).context("failed to bind stdin socket")?;
    let stdout_listener =
        UnixListener::bind(stdout_socket).context("failed to bind stdout socket")?;

    gpui::App::headless().run(move |cx| {
        settings::init(cx);
        HeadlessProject::init(cx);

        let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded::<Envelope>();

        let (stdin_failed_tx, mut stdin_failed_rx) = mpsc::unbounded::<()>();

        cx.background_executor()
            .spawn(async move {
                loop {
                    log::info!("server: waiting for a new connection on stdin socket");
                    let (mut stream, _) = stdin_listener
                        .accept()
                        .await
                        .context("accept on stdin socket failed")?;
                    log::info!("server: got new connection on stdin socket");

                    let mut input_buffer = Vec::new();
                    loop {
                        let message = match read_message(&mut stream, &mut input_buffer).await {
                            Ok(message) => message,
                            Err(error) => {
                                log::warn!("server: error reading message on stdin: {}", error);
                                stdin_failed_tx.unbounded_send(()).ok();
                                log::warn!("server: sent stdin failed message");
                                break;
                            }
                        };
                        if let Err(error) = incoming_tx.unbounded_send(message) {
                            return Err::<(), anyhow::Error>(anyhow!(
                                "server: failed to send message to incoming_tx: {:?}",
                                error
                            ));
                        }
                    }
                }
            })
            .detach();

        cx.background_executor()
            .spawn(async move {
                loop {
                    log::info!("server: waiting for a new connection on stdout socket");
                    let Ok((mut stream, _)) = stdout_listener.accept().await else {
                        log::error!("server: accept on stdout socket failed");
                        break;
                    };

                    log::info!("server: got new connection on stdout socket");

                    let mut output_buffer = Vec::new();
                    loop {
                        select! {
                            message = outgoing_rx.next().fuse() => {
                                let Some(message) = message else {
                                    log::error!("server: stdout handler, no message");
                                    break;
                                };

                                if let Err(error) =
                                    write_message(&mut stream, &mut output_buffer, message).await
                                {
                                    log::error!("server: failed to write stdout message: {:?}", error);
                                    break;
                                }
                                if let Err(error) = stream.flush().await {
                                    log::error!("server: failed to flush stdout message: {:?}", error);
                                    break;
                                }
                            }
                            _ = stdin_failed_rx.next().fuse() => {
                                log::error!("server: stdin failed, terminating");
                                break;
                            }
                        }
                    }
                }
            })
            .detach();

        let session = ChannelClient::new(incoming_rx, outgoing_tx, cx);
        let project = cx.new_model(|cx| {
            HeadlessProject::new(
                session.clone(),
                Arc::new(RealFs::new(Default::default(), None)),
                cx,
            )
        });

        mem::forget(project);
    });
    Ok(())
}

#[cfg(not(windows))]
fn execute_proxy(unique_project_id: String) -> Result<()> {
    log::debug!("proxy: starting up. PID: {}", std::process::id());

    let project_dir = ensure_project_dir(&unique_project_id)?;

    let pid_file = project_dir.join("server.pid");
    let stdin_socket = project_dir.join("stdin.sock");
    let stdout_socket = project_dir.join("stdout.sock");
    let log_file = project_dir.join("server.log");

    let server_running = check_pid_file(&pid_file)?;
    if !server_running {
        spawn_server(&log_file, &pid_file, &stdin_socket, &stdout_socket)?;
    };

    let stdin_task = smol::spawn(async move {
        let stdin = Async::new(std::io::stdin())?;
        let stream = smol::net::unix::UnixStream::connect(stdin_socket).await?;
        handle_io(stdin, stream, "stdin").await
    });

    let stdout_task: smol::Task<Result<()>> = smol::spawn(async move {
        let stdout = Async::new(std::io::stdout())?;
        let stream = smol::net::unix::UnixStream::connect(stdout_socket).await?;
        handle_io(stream, stdout, "stdout").await
    });

    if let Err(forwarding_result) =
        smol::block_on(async move { smol::future::race(stdin_task, stdout_task).await })
    {
        log::error!(
            "proxy: failed to forward messages: {:?}, terminating...",
            forwarding_result
        );
        return Err(forwarding_result);
    }

    Ok(())
}

#[cfg(not(windows))]
fn ensure_project_dir(unique_project_id: &str) -> Result<PathBuf> {
    let project_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let project_dir = PathBuf::from(project_dir)
        .join(".local")
        .join("zed")
        .join("server")
        .join(unique_project_id);

    std::fs::create_dir_all(&project_dir)?;

    Ok(project_dir)
}

#[cfg(not(windows))]
fn spawn_server(
    log_file: &Path,
    pid_file: &Path,
    stdin_socket: &Path,
    stdout_socket: &Path,
) -> Result<()> {
    if stdin_socket.exists() {
        std::fs::remove_file(&stdin_socket)?;
    }
    if stdout_socket.exists() {
        std::fs::remove_file(&stdout_socket)?;
    }

    let binary_name = std::env::current_exe()?;
    let server_process = std::process::Command::new(binary_name)
        .arg("run")
        .arg("--log-file")
        .arg(log_file)
        .arg("--pid-file")
        .arg(pid_file)
        .arg("--stdin-socket")
        .arg(stdin_socket)
        .arg("--stdout-socket")
        .arg(stdout_socket)
        .spawn()?;

    log::debug!("proxy: server started. PID: {:?}", server_process.id());

    let mut total_time_waited = std::time::Duration::from_secs(0);
    let wait_duration = std::time::Duration::from_millis(20);
    while !stdout_socket.exists() || !stdin_socket.exists() {
        log::debug!("proxy: waiting for server to be ready to accept connections...");
        std::thread::sleep(wait_duration);
        total_time_waited += wait_duration;
    }

    log::info!(
        "proxy: server ready to accept connections. total time waited: {:?}",
        total_time_waited
    );
    Ok(())
}

#[cfg(not(windows))]
fn check_pid_file(path: &Path) -> Result<bool> {
    let pid = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read PID file at {:?}", path))
        .and_then(|contents| contents.parse::<u32>().context("Failed to parse PID file"))?;

    log::debug!("proxy: Checking if process with PID {} exists...", pid);
    match std::process::Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .output()
    {
        Ok(output) if output.status.success() => {
            log::debug!("proxy: Process with PID {} exists. NOT spawning new server, but attaching to existing one.", pid);
            Ok(true)
        }
        _ => {
            log::debug!("proxy: Found PID file, but process with that PID does not exist. Removing PID file.");
            std::fs::remove_file(&path).context("proxy: Failed to remove PID file")?;
            Ok(false)
        }
    }
}

#[cfg(not(windows))]
fn write_pid_file(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }

    std::fs::write(path, std::process::id().to_string()).context("Failed to write PID file")
}

#[cfg(not(windows))]
async fn handle_io<R, W>(mut reader: R, mut writer: W, socket_name: &str) -> Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    use remote::protocol::read_message_raw;

    let mut buffer = Vec::new();
    loop {
        read_message_raw(&mut reader, &mut buffer)
            .await
            .with_context(|| format!("proxy: failed to read message from {}", socket_name))?;

        write_size_prefixed_buffer(&mut writer, &mut buffer)
            .await
            .with_context(|| format!("proxy: failed to write message to {}", socket_name))?;

        writer.flush().await?;

        buffer.clear();
    }
}

async fn write_size_prefixed_buffer<S: AsyncWrite + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<()> {
    let len = buffer.len() as u32;
    stream.write_all(len.to_le_bytes().as_slice()).await?;
    stream.write_all(buffer).await?;
    Ok(())
}

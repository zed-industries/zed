#![cfg_attr(target_os = "windows", allow(unused, dead_code))]

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use fs::RealFs;
use futures::channel::mpsc;
use futures::AsyncRead;
use gpui::Context as _;
use prost::Message as _;
use remote::protocol::{MessageLen, MESSAGE_LEN_SIZE};
use remote::ssh_session::ChannelClient;
use remote::{
    json_log::LogRecord,
    protocol::{read_message, write_message},
};
use remote_server::HeadlessProject;
use rpc::proto::Envelope;
use smol::Async;
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
            .format(|buf, record| {
                serde_json::to_writer(&mut *buf, &LogRecord::new(record))?;
                buf.write_all(b"\n")?;
                Ok(())
            })
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

fn execute_run(pid_file: PathBuf, stdin_socket: PathBuf, stdout_socket: PathBuf) -> Result<()> {
    write_pid(&pid_file).with_context(|| format!("failed to write pid file: {:?}", &pid_file))?;

    let stdin_listener = UnixListener::bind(stdin_socket).context("failed to bind stdin socket")?;
    let stdout_listener =
        UnixListener::bind(stdout_socket).context("failed to bind stdout socket")?;

    gpui::App::headless().run(move |cx| {
        settings::init(cx);
        HeadlessProject::init(cx);

        let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded::<Envelope>();

        cx.background_executor()
            .spawn(async move {
                'outer: while let Ok((mut stream, _)) = stdin_listener.accept().await {
                    log::debug!("server: got new connection on stdin socket");
                    // read from stream, and send it to message_tx
                    let mut input_buffer = Vec::new();
                    loop {
                        let message = match read_message(&mut stream, &mut input_buffer).await {
                            Ok(message) => {
                                log::debug!("server: got a message on stdin: {:?}", message.id);
                                message
                            }
                            Err(error) => {
                                log::warn!("server: error reading message: {:?}", error);
                                log::warn!("server: Waiting for new connection on stdin socket");
                                break 'outer;
                            }
                        };
                        if let Err(error) = incoming_tx.unbounded_send(message) {
                            return Err(anyhow!(
                                "server: failed to send message to incoming_tx: {:?}",
                                error
                            ));
                        }
                    }
                }
                anyhow::Ok(())
            })
            .detach();

        cx.background_executor()
            .spawn(async move {
                log::debug!("server: waiting for a new connection on stdout socket");
                while let Ok((mut stream, _)) = stdout_listener.accept().await {
                    log::debug!("server: got new connection on stdout socket");

                    let mut output_buffer = Vec::new();
                    while let Some(message) = outgoing_rx.next().await {
                        log::debug!(
                            "server: sending outgoing message to stdout: id={:?}, message.encoded_len={:?}",
                            message.id,
                            message.encoded_len() as u32
                        );
                        write_message(&mut stream, &mut output_buffer, message).await?;
                        stream.flush().await?;
                    }
                }
                anyhow::Ok(())
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

// pid_file_path    = ~/.local/zed/server/<unique_project_id>/server.pid
// stdout_sock_path = ~/.local/zed/server/<unique_project_id>/stdout.sock
// stdin_sock_path  = ~/.local/zed/server/<unique_project_id>/stdin.sock

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

fn execute_proxy(unique_project_id: String) -> Result<()> {
    log::debug!("proxy: forward PID: {}", std::process::id());

    let project_dir = ensure_project_dir(&unique_project_id)?;

    let pid_file = project_dir.join("server.pid");
    let stdin_socket = project_dir.join("stdin.sock");
    let stdout_socket = project_dir.join("stdout.sock");
    let log_file = project_dir.join("server.log");

    let mut spawn_server = true;
    if let Ok(pid_file_contents) = std::fs::read_to_string(&pid_file) {
        let pid: u32 = pid_file_contents.parse()?;
        // check if process with this pid exists
        log::debug!("proxy: Checking if process with PID {} exists...", pid);
        match std::process::Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .output()
        {
            Ok(output) if output.status.success() => {
                log::debug!("proxy: Process with PID {} exists. NOT spawning new server, but attaching to existing one.", pid);
                spawn_server = false;
            }
            _ => {
                log::debug!(
                    "proxy: Found PID file, but process with that PID does not exist. Removing PID file."
                );
                std::fs::remove_file(&pid_file).context("proxy: Failed to remove PID file")?;
            }
        };
    }

    log::debug!("spawn_server: {}", spawn_server);
    if spawn_server && stdin_socket.exists() {
        std::fs::remove_file(&stdin_socket)?;
    }
    if spawn_server && stdout_socket.exists() {
        std::fs::remove_file(&stdout_socket)?;
    }

    let server = if spawn_server {
        let binary_name = std::env::current_exe()?;

        let server = std::process::Command::new(binary_name)
            .arg("run")
            .arg("--log-file")
            .arg(log_file)
            .arg("--pid-file")
            .arg(pid_file)
            .arg("--stdin-socket")
            .arg(stdin_socket.clone())
            .arg("--stdout-socket")
            .arg(stdout_socket.clone())
            .spawn()?;

        log::debug!("proxy: waiting for server to start...");
        // TODO: better way to wait for server to start
        std::thread::sleep(std::time::Duration::from_secs(3));
        log::debug!("proxy: server process id: {:?}", server.id());

        Some(server)
    } else {
        None
    };

    let (quit_tx, quit_rx) = smol::channel::unbounded::<()>();
    let stdin_task = smol::spawn(async move {
        let mut stdin = Async::new(std::io::stdin())?;
        let mut stream = smol::net::unix::UnixStream::connect(stdin_socket).await?;
        log::debug!("proxy: connected to stdin socket");

        let mut read_buffer = Vec::new();
        let mut write_buffer = Vec::new();
        loop {
            match read_message(&mut stdin, &mut read_buffer).await {
                Ok(message) => {
                    log::debug!("proxy: got a message on stdin: {:?}", message.id);
                    if let Err(error) = write_message(&mut stream, &mut write_buffer, message).await
                    {
                        log::error!(
                            "proxy: failed to write message to stdin socket: {:?}, terminating...",
                            error
                        );
                        quit_tx.send(()).await?;
                        return anyhow::Ok(());
                    }
                    stream.flush().await?;
                }
                Err(error) => {
                    log::error!(
                        "proxy: failed to read message from stdin: {:?}, terminating...",
                        error
                    );
                    // TODO: Under which conditions do we exit?
                    // std::process::exit(1);
                    return Err(error.into());
                }
            }
        }
    });
    let stdout_task: smol::Task<Result<()>> = smol::spawn(async move {
        let mut stdout = Async::new(std::io::stdout())?;
        let mut stream = smol::net::unix::UnixStream::connect(stdout_socket).await?;
        let mut read_buffer = Vec::new();
        let mut write_buffer = Vec::new();
        loop {
            smol::future::or(
                async {
                    match read_message(&mut stream, &mut read_buffer).await {
                        Ok(message) => {
                            log::debug!("proxy: got a message on stdout: {:?}", message.id);
                            if let Err(error) = write_message(&mut stdout, &mut write_buffer, message).await {
                                log::error!("proxy: failed to write message to stdout socket: {:?}, terminating...", error);
                                return anyhow::Ok(());
                            }
                            stdout.flush().await?;
                        }
                        Err(error) => {
                            log::error!(
                                "proxy: failed to read message from stdout: {:?}, terminating...",
                                error
                            );
                            return Err(error.into());
                        }
                    }
                    anyhow::Ok(())
                },
                async {
                    // TODO: Under which conditions do we exit?
                    quit_rx.recv().await?;
                    anyhow::Ok(())
                },
            )
            .await?;
        }
    });

    smol::block_on(async move {
        let (stdin_result, stdout_result) = smol::future::zip(stdin_task, stdout_task).await;
        stdin_result?;
        stdout_result?;
        anyhow::Ok(())
    })?;

    Ok(())
}

fn write_pid(pid_file: &Path) -> Result<()> {
    let pid = std::process::id();
    if pid_file.exists() {
        // remove the pid file
        std::fs::remove_file(pid_file)?;
    }
    std::fs::write(pid_file, pid.to_string()).context("Failed to write PID file")?;
    Ok(())
}

use crate::HeadlessProject;
use anyhow::{anyhow, Context, Result};
use fs::RealFs;
use futures::channel::mpsc;
use futures::{select, select_biased, AsyncRead, AsyncWrite, FutureExt, SinkExt};
use gpui::{AppContext, Context as _};
use remote::ssh_session::ChannelClient;
use remote::{
    json_log::LogRecord,
    protocol::{read_message, write_message},
};
use rpc::proto::Envelope;
use smol::Async;
use smol::{io::AsyncWriteExt, net::unix::UnixListener, stream::StreamExt as _};
use std::{
    env,
    io::Write,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn init(log_file: Option<PathBuf>) -> Result<()> {
    init_logging(log_file)?;
    init_panic_hook();
    Ok(())
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

fn init_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "Box<Any>".to_string());

        let backtrace = backtrace::Backtrace::new();
        let mut backtrace = backtrace
            .frames()
            .iter()
            .flat_map(|frame| {
                frame
                    .symbols()
                    .iter()
                    .filter_map(|frame| Some(format!("{:#}", frame.name()?)))
            })
            .collect::<Vec<_>>();

        // Strip out leading stack frames for rust panic-handling.
        if let Some(ix) = backtrace
            .iter()
            .position(|name| name == "rust_begin_unwind")
        {
            backtrace.drain(0..=ix);
        }

        log::error!(
            "server: panic occurred: {}\nBacktrace:\n{}",
            payload,
            backtrace.join("\n")
        );

        std::process::abort();
    }));
}

fn start_server(
    stdin_listener: UnixListener,
    stdout_listener: UnixListener,
    cx: &mut AppContext,
) -> Arc<ChannelClient> {
    // This is the server idle timeout. If no connection comes in in this timeout, the server will shut down.
    const IDLE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10 * 60);

    let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded::<Envelope>();
    let (app_quit_tx, mut app_quit_rx) = mpsc::unbounded::<()>();

    cx.on_app_quit(move |_| {
        let mut app_quit_tx = app_quit_tx.clone();
        async move {
            log::info!("app quitting. sending signal to server main loop");
            app_quit_tx.send(()).await.ok();
        }
    })
    .detach();

    cx.spawn(|cx| async move {
        let mut stdin_incoming = stdin_listener.incoming();
        let mut stdout_incoming = stdout_listener.incoming();

        loop {
            let streams = futures::future::join(stdin_incoming.next(), stdout_incoming.next());

            log::info!("server: accepting new connections");
            let result = select! {
                streams = streams.fuse() => {
                    let (Some(Ok(stdin_stream)), Some(Ok(stdout_stream))) = streams else {
                        break;
                    };
                    anyhow::Ok((stdin_stream, stdout_stream))
                }
                _ = futures::FutureExt::fuse(smol::Timer::after(IDLE_TIMEOUT)) => {
                    log::warn!("server: timed out waiting for new connections after {:?}. exiting.", IDLE_TIMEOUT);
                    cx.update(|cx| {
                        // TODO: This is a hack, because in a headless project, shutdown isn't executed
                        // when calling quit, but it should be.
                        cx.shutdown();
                        cx.quit();
                    })?;
                    break;
                }
                _ = app_quit_rx.next().fuse() => {
                    break;
                }
            };

            let Ok((mut stdin_stream, mut stdout_stream)) = result else {
                break;
            };

            let mut input_buffer = Vec::new();
            let mut output_buffer = Vec::new();
            loop {
                select_biased! {
                    _ = app_quit_rx.next().fuse() => {
                        return anyhow::Ok(());
                    }

                    stdin_message = read_message(&mut stdin_stream, &mut input_buffer).fuse() => {
                        let message = match stdin_message {
                            Ok(message) => message,
                            Err(error) => {
                                log::warn!("server: error reading message on stdin: {}. exiting.", error);
                                break;
                            }
                        };
                        if let Err(error) = incoming_tx.unbounded_send(message) {
                            log::error!("server: failed to send message to application: {:?}. exiting.", error);
                            return Err(anyhow!(error));
                        }
                    }

                    outgoing_message  = outgoing_rx.next().fuse() => {
                        let Some(message) = outgoing_message else {
                            log::error!("server: stdout handler, no message");
                            break;
                        };

                        if let Err(error) =
                            write_message(&mut stdout_stream, &mut output_buffer, message).await
                        {
                            log::error!("server: failed to write stdout message: {:?}", error);
                            break;
                        }
                        if let Err(error) = stdout_stream.flush().await {
                            log::error!("server: failed to flush stdout message: {:?}", error);
                            break;
                        }
                    }
                }
            }
        }
        anyhow::Ok(())
    })
    .detach();

    ChannelClient::new(incoming_rx, outgoing_tx, cx)
}

pub fn execute_run(pid_file: PathBuf, stdin_socket: PathBuf, stdout_socket: PathBuf) -> Result<()> {
    log::info!(
        "server: starting up. pid_file: {:?}, stdin_socket: {:?}, stdout_socket: {:?}",
        pid_file,
        stdin_socket,
        stdout_socket
    );

    write_pid_file(&pid_file)
        .with_context(|| format!("failed to write pid file: {:?}", &pid_file))?;

    let stdin_listener = UnixListener::bind(stdin_socket).context("failed to bind stdin socket")?;
    let stdout_listener =
        UnixListener::bind(stdout_socket).context("failed to bind stdout socket")?;

    log::debug!("server: starting gpui app");
    gpui::App::headless().run(move |cx| {
        settings::init(cx);
        HeadlessProject::init(cx);

        log::info!("server: gpui app started, initializing server");
        let session = start_server(stdin_listener, stdout_listener, cx);
        let project = cx.new_model(|cx| {
            HeadlessProject::new(session, Arc::new(RealFs::new(Default::default(), None)), cx)
        });

        mem::forget(project);
    });
    log::info!("server: gpui app is shut down. quitting.");
    Ok(())
}

pub fn execute_proxy(identifier: String) -> Result<()> {
    log::debug!("proxy: starting up. PID: {}", std::process::id());

    let project_dir = ensure_project_dir(&identifier)?;

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

fn ensure_project_dir(identifier: &str) -> Result<PathBuf> {
    let project_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let project_dir = PathBuf::from(project_dir)
        .join(".local")
        .join("state")
        .join("zed-remote-server")
        .join(identifier);

    std::fs::create_dir_all(&project_dir)?;

    Ok(project_dir)
}

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

fn check_pid_file(path: &Path) -> Result<bool> {
    let Some(pid) = std::fs::read_to_string(&path)
        .ok()
        .and_then(|contents| contents.parse::<u32>().ok())
    else {
        return Ok(false);
    };

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

fn write_pid_file(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    let pid = std::process::id().to_string();
    log::debug!("server: writing PID {} to file {:?}", pid, path);
    std::fs::write(path, pid).context("Failed to write PID file")
}

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

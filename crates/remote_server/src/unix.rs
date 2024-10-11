use crate::HeadlessProject;
use anyhow::{anyhow, Context, Result};
use fs::RealFs;
use futures::channel::mpsc;
use futures::{select, select_biased, AsyncRead, AsyncWrite, AsyncWriteExt, FutureExt, SinkExt};
use gpui::{AppContext, Context as _};
use remote::proxy::ProxyLaunchError;
use remote::ssh_session::ChannelClient;
use remote::{
    json_log::LogRecord,
    protocol::{read_message, write_message},
};
use rpc::proto::Envelope;
use smol::channel::{Receiver, Sender};
use smol::io::AsyncReadExt;
use smol::Async;
use smol::{net::unix::UnixListener, stream::StreamExt as _};
use std::{
    env,
    io::Write,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};

fn init_logging_proxy() {
    env_logger::builder()
        .format(|buf, record| {
            let mut log_record = LogRecord::new(record);
            log_record.message = format!("(remote proxy) {}", log_record.message);
            serde_json::to_writer(&mut *buf, &log_record)?;
            buf.write_all(b"\n")?;
            Ok(())
        })
        .init();
}

fn init_logging_server(log_file_path: PathBuf) -> Result<Receiver<Vec<u8>>> {
    struct MultiWrite {
        file: Box<dyn std::io::Write + Send + 'static>,
        channel: Sender<Vec<u8>>,
        buffer: Vec<u8>,
    }

    impl std::io::Write for MultiWrite {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let written = self.file.write(buf)?;
            self.buffer.extend_from_slice(&buf[..written]);
            Ok(written)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.channel
                .send_blocking(self.buffer.clone())
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
            self.buffer.clear();
            self.file.flush()
        }
    }

    let log_file = Box::new(if log_file_path.exists() {
        std::fs::OpenOptions::new()
            .append(true)
            .open(&log_file_path)
            .context("Failed to open log file in append mode")?
    } else {
        std::fs::File::create(&log_file_path).context("Failed to create log file")?
    });

    let (tx, rx) = smol::channel::unbounded();

    let target = Box::new(MultiWrite {
        file: log_file,
        channel: tx,
        buffer: Vec::new(),
    });

    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(target))
        .format(|buf, record| {
            let mut log_record = LogRecord::new(record);
            log_record.message = format!("(remote server) {}", log_record.message);
            serde_json::to_writer(&mut *buf, &log_record)?;
            buf.write_all(b"\n")?;
            Ok(())
        })
        .init();

    Ok(rx)
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
            "panic occurred: {}\nBacktrace:\n{}",
            payload,
            backtrace.join("\n")
        );

        std::process::abort();
    }));
}

struct ServerListeners {
    stdin: UnixListener,
    stdout: UnixListener,
    stderr: UnixListener,
}

impl ServerListeners {
    pub fn new(stdin_path: PathBuf, stdout_path: PathBuf, stderr_path: PathBuf) -> Result<Self> {
        Ok(Self {
            stdin: UnixListener::bind(stdin_path).context("failed to bind stdin socket")?,
            stdout: UnixListener::bind(stdout_path).context("failed to bind stdout socket")?,
            stderr: UnixListener::bind(stderr_path).context("failed to bind stderr socket")?,
        })
    }
}

fn start_server(
    listeners: ServerListeners,
    mut log_rx: Receiver<Vec<u8>>,
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
        let mut stdin_incoming = listeners.stdin.incoming();
        let mut stdout_incoming = listeners.stdout.incoming();
        let mut stderr_incoming = listeners.stderr.incoming();

        loop {
            let streams = futures::future::join3(stdin_incoming.next(), stdout_incoming.next(), stderr_incoming.next());

            log::info!("accepting new connections");
            let result = select! {
                streams = streams.fuse() => {
                    log::warn!("stdin {:?}, stdout: {:?}, stderr: {:?}", streams.0, streams.1, streams.2);
                    let (Some(Ok(stdin_stream)), Some(Ok(stdout_stream)), Some(Ok(stderr_stream))) = streams else {
                        break;
                    };
                    anyhow::Ok((stdin_stream, stdout_stream, stderr_stream))
                }
                _ = futures::FutureExt::fuse(smol::Timer::after(IDLE_TIMEOUT)) => {
                    log::warn!("timed out waiting for new connections after {:?}. exiting.", IDLE_TIMEOUT);
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

            let Ok((mut stdin_stream, mut stdout_stream, mut stderr_stream)) = result else {
                break;
            };

            log::info!("yep! we got connections");

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
                                log::warn!("error reading message on stdin: {}. exiting.", error);
                                break;
                            }
                        };
                        if let Err(error) = incoming_tx.unbounded_send(message) {
                            log::error!("failed to send message to application: {:?}. exiting.", error);
                            return Err(anyhow!(error));
                        }
                    }

                    outgoing_message  = outgoing_rx.next().fuse() => {
                        let Some(message) = outgoing_message else {
                            log::error!("stdout handler, no message");
                            break;
                        };

                        if let Err(error) =
                            write_message(&mut stdout_stream, &mut output_buffer, message).await
                        {
                            log::error!("failed to write stdout message: {:?}", error);
                            break;
                        }
                        if let Err(error) = stdout_stream.flush().await {
                            log::error!("failed to flush stdout message: {:?}", error);
                            break;
                        }
                    }

                    // // TODO: How do we handle backpressure?
                    log_message = log_rx.next().fuse() => {
                        if let Some(log_message) = log_message {
                            if let Err(error) = stderr_stream.write_all(&log_message).await {
                                log::error!("failed to write log message to stderr: {:?}", error);
                                break;
                            }
                            if let Err(error) = stderr_stream.flush().await {
                                log::error!("failed to flush stderr stream: {:?}", error);
                                break;
                            }
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

pub fn execute_run(
    log_file: PathBuf,
    pid_file: PathBuf,
    stdin_socket: PathBuf,
    stdout_socket: PathBuf,
    stderr_socket: PathBuf,
) -> Result<()> {
    let log_rx = init_logging_server(log_file)?;
    init_panic_hook();

    log::info!(
        "starting up. pid_file: {:?}, stdin_socket: {:?}, stdout_socket: {:?}, stderr_socket: {:?}",
        pid_file,
        stdin_socket,
        stdout_socket,
        stderr_socket
    );

    write_pid_file(&pid_file)
        .with_context(|| format!("failed to write pid file: {:?}", &pid_file))?;

    let listeners = ServerListeners::new(stdin_socket, stdout_socket, stderr_socket)?;

    log::debug!("starting gpui app");
    gpui::App::headless().run(move |cx| {
        settings::init(cx);
        HeadlessProject::init(cx);

        log::info!("gpui app started, initializing server");
        let session = start_server(listeners, log_rx, cx);

        let project = cx.new_model(|cx| {
            HeadlessProject::new(session, Arc::new(RealFs::new(Default::default(), None)), cx)
        });

        mem::forget(project);
    });
    log::info!("gpui app is shut down. quitting.");
    Ok(())
}

#[derive(Clone)]
struct ServerPaths {
    log_file: PathBuf,
    pid_file: PathBuf,
    stdin_socket: PathBuf,
    stdout_socket: PathBuf,
    stderr_socket: PathBuf,
}

impl ServerPaths {
    fn new(identifier: &str) -> Result<Self> {
        let project_dir = create_state_directory(identifier)?;

        let pid_file = project_dir.join("server.pid");
        let stdin_socket = project_dir.join("stdin.sock");
        let stdout_socket = project_dir.join("stdout.sock");
        let stderr_socket = project_dir.join("stderr.sock");
        let log_file = project_dir.join("server.log");

        Ok(Self {
            pid_file,
            stdin_socket,
            stdout_socket,
            stderr_socket,
            log_file,
        })
    }
}

pub fn execute_proxy(identifier: String, is_reconnecting: bool) -> Result<()> {
    init_logging_proxy();
    init_panic_hook();

    log::debug!("starting up. PID: {}", std::process::id());

    let server_paths = ServerPaths::new(&identifier)?;

    let server_pid = check_pid_file(&server_paths.pid_file)?;
    let server_running = server_pid.is_some();
    if is_reconnecting {
        if !server_running {
            log::error!("attempted to reconnect, but no server running");
            return Err(anyhow!(ProxyLaunchError::ServerNotRunning));
        }
    } else {
        if let Some(pid) = server_pid {
            log::debug!("found server already running with PID {}. Killing process and cleaning up files...", pid);
            kill_running_server(pid, &server_paths)?;
        }

        spawn_server(&server_paths)?;
    }

    let stdin_task = smol::spawn(async move {
        let stdin = Async::new(std::io::stdin())?;
        let stream = smol::net::unix::UnixStream::connect(&server_paths.stdin_socket).await?;
        handle_io(stdin, stream, "stdin").await
    });

    let stdout_task: smol::Task<Result<()>> = smol::spawn(async move {
        let stdout = Async::new(std::io::stdout())?;
        let stream = smol::net::unix::UnixStream::connect(&server_paths.stdout_socket).await?;
        handle_io(stream, stdout, "stdout").await
    });

    let stderr_task: smol::Task<Result<()>> = smol::spawn(async move {
        let mut stderr = Async::new(std::io::stderr())?;
        let mut stream = smol::net::unix::UnixStream::connect(&server_paths.stderr_socket).await?;
        let mut stderr_buffer = vec![0; 2048];
        loop {
            match stream.read(&mut stderr_buffer).await {
                Ok(0) => {
                    return anyhow::Ok(());
                }
                Ok(n) => {
                    stderr.write_all(&mut stderr_buffer[..n]).await?;
                    stderr.flush().await?;
                }
                Err(error) => {
                    Err(anyhow!("error reading stderr: {error:?}"))?;
                }
            }
        }
    });

    if let Err(forwarding_result) = smol::block_on(async move {
        futures::select! {
            result = stdin_task.fuse() => result,
            result = stdout_task.fuse() => result,
            result = stderr_task.fuse() => result,
        }
    }) {
        log::error!(
            "failed to forward messages: {:?}, terminating...",
            forwarding_result
        );
        return Err(forwarding_result);
    }

    Ok(())
}

fn create_state_directory(identifier: &str) -> Result<PathBuf> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let server_dir = PathBuf::from(home_dir)
        .join(".local")
        .join("state")
        .join("zed-remote-server")
        .join(identifier);

    std::fs::create_dir_all(&server_dir)?;

    Ok(server_dir)
}

fn kill_running_server(pid: u32, paths: &ServerPaths) -> Result<()> {
    log::info!("killing existing server with PID {}", pid);
    std::process::Command::new("kill")
        .arg(pid.to_string())
        .output()
        .context("failed to kill existing server")?;

    for file in [
        &paths.pid_file,
        &paths.stdin_socket,
        &paths.stdout_socket,
        &paths.stderr_socket,
    ] {
        log::debug!("cleaning up file {:?} before starting new server", file);
        std::fs::remove_file(file).ok();
    }
    Ok(())
}

fn spawn_server(paths: &ServerPaths) -> Result<()> {
    if paths.stdin_socket.exists() {
        std::fs::remove_file(&paths.stdin_socket)?;
    }
    if paths.stdout_socket.exists() {
        std::fs::remove_file(&paths.stdout_socket)?;
    }
    if paths.stderr_socket.exists() {
        std::fs::remove_file(&paths.stderr_socket)?;
    }

    let binary_name = std::env::current_exe()?;
    let server_process = std::process::Command::new(binary_name)
        .arg("run")
        .arg("--log-file")
        .arg(&paths.log_file)
        .arg("--pid-file")
        .arg(&paths.pid_file)
        .arg("--stdin-socket")
        .arg(&paths.stdin_socket)
        .arg("--stdout-socket")
        .arg(&paths.stdout_socket)
        .arg("--stderr-socket")
        .arg(&paths.stderr_socket)
        .spawn()?;

    log::debug!("server started. PID: {:?}", server_process.id());

    let mut total_time_waited = std::time::Duration::from_secs(0);
    let wait_duration = std::time::Duration::from_millis(20);
    while !paths.stdout_socket.exists()
        || !paths.stdin_socket.exists()
        || !paths.stderr_socket.exists()
    {
        log::debug!("waiting for server to be ready to accept connections...");
        std::thread::sleep(wait_duration);
        total_time_waited += wait_duration;
    }

    log::info!(
        "server ready to accept connections. total time waited: {:?}",
        total_time_waited
    );
    Ok(())
}

fn check_pid_file(path: &Path) -> Result<Option<u32>> {
    let Some(pid) = std::fs::read_to_string(&path)
        .ok()
        .and_then(|contents| contents.parse::<u32>().ok())
    else {
        return Ok(None);
    };

    log::debug!("Checking if process with PID {} exists...", pid);
    match std::process::Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .output()
    {
        Ok(output) if output.status.success() => {
            log::debug!("Process with PID {} exists. NOT spawning new server, but attaching to existing one.", pid);
            Ok(Some(pid))
        }
        _ => {
            log::debug!(
                "Found PID file, but process with that PID does not exist. Removing PID file."
            );
            std::fs::remove_file(&path).context("Failed to remove PID file")?;
            Ok(None)
        }
    }
}

fn write_pid_file(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    let pid = std::process::id().to_string();
    log::debug!("writing PID {} to file {:?}", pid, path);
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
            .with_context(|| format!("failed to read message from {}", socket_name))?;

        write_size_prefixed_buffer(&mut writer, &mut buffer)
            .await
            .with_context(|| format!("failed to write message to {}", socket_name))?;

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

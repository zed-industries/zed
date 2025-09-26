use crate::HeadlessProject;
use crate::headless_project::HeadlessAppState;
use anyhow::{Context as _, Result, anyhow};
use client::ProxySettings;
use util::ResultExt;

use extension::ExtensionHostProxy;
use fs::{Fs, RealFs};
use futures::channel::mpsc;
use futures::{AsyncRead, AsyncWrite, AsyncWriteExt, FutureExt, SinkExt, select, select_biased};
use git::GitHostingProviderRegistry;
use gpui::{App, AppContext as _, Context, Entity, SemanticVersion, UpdateGlobal as _};
use gpui_tokio::Tokio;
use http_client::{Url, read_proxy_from_env};
use language::LanguageRegistry;
use node_runtime::{NodeBinaryOptions, NodeRuntime};
use paths::logs_dir;
use project::project_settings::ProjectSettings;

use proto::CrashReport;
use release_channel::{AppVersion, RELEASE_CHANNEL, ReleaseChannel};
use remote::RemoteClient;
use remote::{
    json_log::LogRecord,
    protocol::{read_message, write_message},
    proxy::ProxyLaunchError,
};
use reqwest_client::ReqwestClient;
use rpc::proto::{self, Envelope, REMOTE_SERVER_PROJECT_ID};
use rpc::{AnyProtoClient, TypedEnvelope};
use settings::{Settings, SettingsStore, watch_config_file};
use smol::Async;
use smol::channel::{Receiver, Sender};
use smol::io::AsyncReadExt;
use smol::{net::unix::UnixListener, stream::StreamExt as _};
use std::{
    env,
    ffi::OsStr,
    fs::File,
    io::Write,
    mem,
    ops::ControlFlow,
    path::{Path, PathBuf},
    process::ExitStatus,
    str::FromStr,
    sync::{Arc, LazyLock},
};
use thiserror::Error;

pub static VERSION: LazyLock<&str> = LazyLock::new(|| match *RELEASE_CHANNEL {
    ReleaseChannel::Stable | ReleaseChannel::Preview => env!("ZED_PKG_VERSION"),
    ReleaseChannel::Nightly | ReleaseChannel::Dev => {
        option_env!("ZED_COMMIT_SHA").unwrap_or("missing-zed-commit-sha")
    }
});

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
        file: File,
        channel: Sender<Vec<u8>>,
        buffer: Vec<u8>,
    }

    impl Write for MultiWrite {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let written = self.file.write(buf)?;
            self.buffer.extend_from_slice(&buf[..written]);
            Ok(written)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.channel
                .send_blocking(self.buffer.clone())
                .map_err(std::io::Error::other)?;
            self.buffer.clear();
            self.file.flush()
        }
    }

    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .context("Failed to open log file in append mode")?;

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

fn handle_crash_files_requests(project: &Entity<HeadlessProject>, client: &AnyProtoClient) {
    client.add_request_handler(
        project.downgrade(),
        |_, _: TypedEnvelope<proto::GetCrashFiles>, _cx| async move {
            let mut legacy_panics = Vec::new();
            let mut crashes = Vec::new();
            let mut children = smol::fs::read_dir(paths::logs_dir()).await?;
            while let Some(child) = children.next().await {
                let child = child?;
                let child_path = child.path();

                let extension = child_path.extension();
                if extension == Some(OsStr::new("panic")) {
                    let filename = if let Some(filename) = child_path.file_name() {
                        filename.to_string_lossy()
                    } else {
                        continue;
                    };

                    if !filename.starts_with("zed") {
                        continue;
                    }

                    let file_contents = smol::fs::read_to_string(&child_path)
                        .await
                        .context("error reading panic file")?;

                    legacy_panics.push(file_contents);
                    smol::fs::remove_file(&child_path)
                        .await
                        .context("error removing panic")
                        .log_err();
                } else if extension == Some(OsStr::new("dmp")) {
                    let mut json_path = child_path.clone();
                    json_path.set_extension("json");
                    if let Ok(json_content) = smol::fs::read_to_string(&json_path).await {
                        crashes.push(CrashReport {
                            metadata: json_content,
                            minidump_contents: smol::fs::read(&child_path).await?,
                        });
                        smol::fs::remove_file(&child_path).await.log_err();
                        smol::fs::remove_file(&json_path).await.log_err();
                    } else {
                        log::error!("Couldn't find json metadata for crash: {child_path:?}");
                    }
                }
            }

            anyhow::Ok(proto::GetCrashFilesResponse { crashes })
        },
    );
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
    log_rx: Receiver<Vec<u8>>,
    cx: &mut App,
) -> AnyProtoClient {
    // This is the server idle timeout. If no connection comes in this timeout, the server will shut down.
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

    cx.spawn(async move |cx| {
        let mut stdin_incoming = listeners.stdin.incoming();
        let mut stdout_incoming = listeners.stdout.incoming();
        let mut stderr_incoming = listeners.stderr.incoming();

        loop {
            let streams = futures::future::join3(stdin_incoming.next(), stdout_incoming.next(), stderr_incoming.next());

            log::info!("accepting new connections");
            let result = select! {
                streams = streams.fuse() => {
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

            let mut input_buffer = Vec::new();
            let mut output_buffer = Vec::new();

            let (mut stdin_msg_tx, mut stdin_msg_rx) = mpsc::unbounded::<Envelope>();
            cx.background_spawn(async move {
                while let Ok(msg) = read_message(&mut stdin_stream, &mut input_buffer).await {
                    if (stdin_msg_tx.send(msg).await).is_err() {
                        break;
                    }
                }
            }).detach();

            loop {

                select_biased! {
                    _ = app_quit_rx.next().fuse() => {
                        return anyhow::Ok(());
                    }

                    stdin_message = stdin_msg_rx.next().fuse() => {
                        let Some(message) = stdin_message else {
                            log::warn!("error reading message on stdin. exiting.");
                            break;
                        };
                        if let Err(error) = incoming_tx.unbounded_send(message) {
                            log::error!("failed to send message to application: {error:?}. exiting.");
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

                    log_message = log_rx.recv().fuse() => {
                        if let Ok(log_message) = log_message {
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

    RemoteClient::proto_client_from_channels(incoming_rx, outgoing_tx, cx, "server")
}

fn init_paths() -> anyhow::Result<()> {
    for path in [
        paths::config_dir(),
        paths::extensions_dir(),
        paths::languages_dir(),
        paths::logs_dir(),
        paths::temp_dir(),
        paths::remote_extensions_dir(),
        paths::remote_extensions_uploads_dir(),
    ]
    .iter()
    {
        std::fs::create_dir_all(path).with_context(|| format!("creating directory {path:?}"))?;
    }
    Ok(())
}

pub fn execute_run(
    log_file: PathBuf,
    pid_file: PathBuf,
    stdin_socket: PathBuf,
    stdout_socket: PathBuf,
    stderr_socket: PathBuf,
) -> Result<()> {
    init_paths()?;

    match daemonize()? {
        ControlFlow::Break(_) => return Ok(()),
        ControlFlow::Continue(_) => {}
    }

    let app = gpui::Application::headless();
    let id = std::process::id().to_string();
    app.background_executor()
        .spawn(crashes::init(crashes::InitCrashHandler {
            session_id: id,
            zed_version: VERSION.to_owned(),
            release_channel: release_channel::RELEASE_CHANNEL_NAME.clone(),
            commit_sha: option_env!("ZED_COMMIT_SHA").unwrap_or("no_sha").to_owned(),
        }))
        .detach();
    let log_rx = init_logging_server(log_file)?;
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

    let git_hosting_provider_registry = Arc::new(GitHostingProviderRegistry::new());
    app.run(move |cx| {
        settings::init(cx);
        let app_version = AppVersion::load(env!("ZED_PKG_VERSION"));
        release_channel::init(app_version, cx);
        gpui_tokio::init(cx);

        HeadlessProject::init(cx);

        log::info!("gpui app started, initializing server");
        let session = start_server(listeners, log_rx, cx);

        client::init_settings(cx);

        GitHostingProviderRegistry::set_global(git_hosting_provider_registry, cx);
        git_hosting_providers::init(cx);
        dap_adapters::init(cx);

        extension::init(cx);
        let extension_host_proxy = ExtensionHostProxy::global(cx);

        json_schema_store::init(cx);

        let project = cx.new(|cx| {
            let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
            let node_settings_rx = initialize_settings(session.clone(), fs.clone(), cx);

            let proxy_url = read_proxy_settings(cx);

            let http_client = {
                let _guard = Tokio::handle(cx).enter();
                Arc::new(
                    ReqwestClient::proxy_and_user_agent(
                        proxy_url,
                        &format!(
                            "Zed-Server/{} ({}; {})",
                            env!("CARGO_PKG_VERSION"),
                            std::env::consts::OS,
                            std::env::consts::ARCH
                        ),
                    )
                    .expect("Could not start HTTP client"),
                )
            };

            let node_runtime = NodeRuntime::new(http_client.clone(), None, node_settings_rx);

            let mut languages = LanguageRegistry::new(cx.background_executor().clone());
            languages.set_language_server_download_dir(paths::languages_dir().clone());
            let languages = Arc::new(languages);

            HeadlessProject::new(
                HeadlessAppState {
                    session: session.clone(),
                    fs,
                    http_client,
                    node_runtime,
                    languages,
                    extension_host_proxy,
                },
                cx,
            )
        });

        handle_crash_files_requests(&project, &session);

        cx.background_spawn(async move { cleanup_old_binaries() })
            .detach();

        mem::forget(project);
    });
    log::info!("gpui app is shut down. quitting.");
    Ok(())
}

#[derive(Debug, Error)]
pub(crate) enum ServerPathError {
    #[error("Failed to create server_dir `{path}`")]
    CreateServerDir {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("Failed to create logs_dir `{path}`")]
    CreateLogsDir {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
}

#[derive(Clone, Debug)]
struct ServerPaths {
    log_file: PathBuf,
    pid_file: PathBuf,
    stdin_socket: PathBuf,
    stdout_socket: PathBuf,
    stderr_socket: PathBuf,
}

impl ServerPaths {
    fn new(identifier: &str) -> Result<Self, ServerPathError> {
        let server_dir = paths::remote_server_state_dir().join(identifier);
        std::fs::create_dir_all(&server_dir).map_err(|source| {
            ServerPathError::CreateServerDir {
                source,
                path: server_dir.clone(),
            }
        })?;
        let log_dir = logs_dir();
        std::fs::create_dir_all(log_dir).map_err(|source| ServerPathError::CreateLogsDir {
            source: source,
            path: log_dir.clone(),
        })?;

        let pid_file = server_dir.join("server.pid");
        let stdin_socket = server_dir.join("stdin.sock");
        let stdout_socket = server_dir.join("stdout.sock");
        let stderr_socket = server_dir.join("stderr.sock");
        let log_file = logs_dir().join(format!("server-{}.log", identifier));

        Ok(Self {
            pid_file,
            stdin_socket,
            stdout_socket,
            stderr_socket,
            log_file,
        })
    }
}

#[derive(Debug, Error)]
pub(crate) enum ExecuteProxyError {
    #[error("Failed to init server paths")]
    ServerPath(#[from] ServerPathError),

    #[error(transparent)]
    ServerNotRunning(#[from] ProxyLaunchError),

    #[error("Failed to check PidFile '{path}'")]
    CheckPidFile {
        #[source]
        source: CheckPidError,
        path: PathBuf,
    },

    #[error("Failed to kill existing server with pid '{pid}'")]
    KillRunningServer {
        #[source]
        source: std::io::Error,
        pid: u32,
    },

    #[error("failed to spawn server")]
    SpawnServer(#[source] SpawnServerError),

    #[error("stdin_task failed")]
    StdinTask(#[source] anyhow::Error),
    #[error("stdout_task failed")]
    StdoutTask(#[source] anyhow::Error),
    #[error("stderr_task failed")]
    StderrTask(#[source] anyhow::Error),
}

pub(crate) fn execute_proxy(
    identifier: String,
    is_reconnecting: bool,
) -> Result<(), ExecuteProxyError> {
    init_logging_proxy();

    let server_paths = ServerPaths::new(&identifier)?;

    let id = std::process::id().to_string();
    smol::spawn(crashes::init(crashes::InitCrashHandler {
        session_id: id,
        zed_version: VERSION.to_owned(),
        release_channel: release_channel::RELEASE_CHANNEL_NAME.clone(),
        commit_sha: option_env!("ZED_COMMIT_SHA").unwrap_or("no_sha").to_owned(),
    }))
    .detach();

    log::info!("starting proxy process. PID: {}", std::process::id());
    smol::block_on(async {
        let server_pid = check_pid_file(&server_paths.pid_file)
            .await
            .map_err(|source| ExecuteProxyError::CheckPidFile {
                source,
                path: server_paths.pid_file.clone(),
            })?;
        let server_running = server_pid.is_some();
        if is_reconnecting {
            if !server_running {
                log::error!("attempted to reconnect, but no server running");
                return Err(ExecuteProxyError::ServerNotRunning(
                    ProxyLaunchError::ServerNotRunning,
                ));
            }
        } else {
            if let Some(pid) = server_pid {
                log::info!(
                    "proxy found server already running with PID {}. Killing process and cleaning up files...",
                    pid
                );
                kill_running_server(pid, &server_paths).await?;
            }

            spawn_server(&server_paths)
                .await
                .map_err(ExecuteProxyError::SpawnServer)?;
        };
        Ok(())
    })?;

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
            match stream
                .read(&mut stderr_buffer)
                .await
                .context("reading stderr")?
            {
                0 => {
                    let error =
                        std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "stderr closed");
                    Err(anyhow!(error))?;
                }
                n => {
                    stderr.write_all(&stderr_buffer[..n]).await?;
                    stderr.flush().await?;
                }
            }
        }
    });

    if let Err(forwarding_result) = smol::block_on(async move {
        futures::select! {
            result = stdin_task.fuse() => result.map_err(ExecuteProxyError::StdinTask),
            result = stdout_task.fuse() => result.map_err(ExecuteProxyError::StdoutTask),
            result = stderr_task.fuse() => result.map_err(ExecuteProxyError::StderrTask),
        }
    }) {
        log::error!(
            "encountered error while forwarding messages: {:?}, terminating...",
            forwarding_result
        );
        return Err(forwarding_result);
    }

    Ok(())
}

async fn kill_running_server(pid: u32, paths: &ServerPaths) -> Result<(), ExecuteProxyError> {
    log::info!("killing existing server with PID {}", pid);
    smol::process::Command::new("kill")
        .arg(pid.to_string())
        .output()
        .await
        .map_err(|source| ExecuteProxyError::KillRunningServer { source, pid })?;

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

#[derive(Debug, Error)]
pub(crate) enum SpawnServerError {
    #[error("failed to remove stdin socket")]
    RemoveStdinSocket(#[source] std::io::Error),

    #[error("failed to remove stdout socket")]
    RemoveStdoutSocket(#[source] std::io::Error),

    #[error("failed to remove stderr socket")]
    RemoveStderrSocket(#[source] std::io::Error),

    #[error("failed to get current_exe")]
    CurrentExe(#[source] std::io::Error),

    #[error("failed to launch server process")]
    ProcessStatus(#[source] std::io::Error),

    #[error("failed to launch and detach server process: {status}\n{paths}")]
    LaunchStatus { status: ExitStatus, paths: String },
}

async fn spawn_server(paths: &ServerPaths) -> Result<(), SpawnServerError> {
    if paths.stdin_socket.exists() {
        std::fs::remove_file(&paths.stdin_socket).map_err(SpawnServerError::RemoveStdinSocket)?;
    }
    if paths.stdout_socket.exists() {
        std::fs::remove_file(&paths.stdout_socket).map_err(SpawnServerError::RemoveStdoutSocket)?;
    }
    if paths.stderr_socket.exists() {
        std::fs::remove_file(&paths.stderr_socket).map_err(SpawnServerError::RemoveStderrSocket)?;
    }

    let binary_name = std::env::current_exe().map_err(SpawnServerError::CurrentExe)?;
    let mut server_process = smol::process::Command::new(binary_name);
    server_process
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
        .arg(&paths.stderr_socket);

    let status = server_process
        .status()
        .await
        .map_err(SpawnServerError::ProcessStatus)?;

    if !status.success() {
        return Err(SpawnServerError::LaunchStatus {
            status,
            paths: format!(
                "log file: {:?}, pid file: {:?}",
                paths.log_file, paths.pid_file,
            ),
        });
    }

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

#[derive(Debug, Error)]
#[error("Failed to remove PID file for missing process (pid `{pid}`")]
pub(crate) struct CheckPidError {
    #[source]
    source: std::io::Error,
    pid: u32,
}

async fn check_pid_file(path: &Path) -> Result<Option<u32>, CheckPidError> {
    let Some(pid) = std::fs::read_to_string(&path)
        .ok()
        .and_then(|contents| contents.parse::<u32>().ok())
    else {
        return Ok(None);
    };

    log::debug!("Checking if process with PID {} exists...", pid);
    match smol::process::Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .output()
        .await
    {
        Ok(output) if output.status.success() => {
            log::debug!(
                "Process with PID {} exists. NOT spawning new server, but attaching to existing one.",
                pid
            );
            Ok(Some(pid))
        }
        _ => {
            log::debug!(
                "Found PID file, but process with that PID does not exist. Removing PID file."
            );
            std::fs::remove_file(&path).map_err(|source| CheckPidError { source, pid })?;
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
    use remote::protocol::{read_message_raw, write_size_prefixed_buffer};

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

fn initialize_settings(
    session: AnyProtoClient,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) -> watch::Receiver<Option<NodeBinaryOptions>> {
    let user_settings_file_rx =
        watch_config_file(cx.background_executor(), fs, paths::settings_file().clone());

    handle_settings_file_changes(user_settings_file_rx, cx, {
        move |err, _cx| {
            if let Some(e) = err {
                log::info!("Server settings failed to change: {}", e);

                session
                    .send(proto::Toast {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        notification_id: "server-settings-failed".to_string(),
                        message: format!(
                            "Error in settings on remote host {:?}: {}",
                            paths::settings_file(),
                            e
                        ),
                    })
                    .log_err();
            } else {
                session
                    .send(proto::HideToast {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        notification_id: "server-settings-failed".to_string(),
                    })
                    .log_err();
            }
        }
    });

    let (mut tx, rx) = watch::channel(None);
    let mut node_settings = None;
    cx.observe_global::<SettingsStore>(move |cx| {
        let new_node_settings = &ProjectSettings::get_global(cx).node;
        if Some(new_node_settings) != node_settings.as_ref() {
            log::info!("Got new node settings: {new_node_settings:?}");
            let options = NodeBinaryOptions {
                allow_path_lookup: !new_node_settings.ignore_system_version,
                // TODO: Implement this setting
                allow_binary_download: true,
                use_paths: new_node_settings.path.as_ref().map(|node_path| {
                    let node_path = PathBuf::from(shellexpand::tilde(node_path).as_ref());
                    let npm_path = new_node_settings
                        .npm_path
                        .as_ref()
                        .map(|path| PathBuf::from(shellexpand::tilde(&path).as_ref()));
                    (
                        node_path.clone(),
                        npm_path.unwrap_or_else(|| {
                            let base_path = PathBuf::new();
                            node_path.parent().unwrap_or(&base_path).join("npm")
                        }),
                    )
                }),
            };
            node_settings = Some(new_node_settings.clone());
            tx.send(Some(options)).ok();
        }
    })
    .detach();

    rx
}

pub fn handle_settings_file_changes(
    mut server_settings_file: mpsc::UnboundedReceiver<String>,
    cx: &mut App,
    settings_changed: impl Fn(Option<anyhow::Error>, &mut App) + 'static,
) {
    let server_settings_content = cx
        .background_executor()
        .block(server_settings_file.next())
        .unwrap();
    SettingsStore::update_global(cx, |store, cx| {
        store
            .set_server_settings(&server_settings_content, cx)
            .log_err();
    });
    cx.spawn(async move |cx| {
        while let Some(server_settings_content) = server_settings_file.next().await {
            let result = cx.update_global(|store: &mut SettingsStore, cx| {
                let result = store.set_server_settings(&server_settings_content, cx);
                if let Err(err) = &result {
                    log::error!("Failed to load server settings: {err}");
                }
                settings_changed(result.err(), cx);
                cx.refresh_windows();
            });
            if result.is_err() {
                break; // App dropped
            }
        }
    })
    .detach();
}

fn read_proxy_settings(cx: &mut Context<HeadlessProject>) -> Option<Url> {
    let proxy_str = ProxySettings::get_global(cx).proxy.to_owned();

    proxy_str
        .as_ref()
        .and_then(|input: &String| {
            input
                .parse::<Url>()
                .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                .ok()
        })
        .or_else(read_proxy_from_env)
}

fn daemonize() -> Result<ControlFlow<()>> {
    match fork::fork().map_err(|e| anyhow!("failed to call fork with error code {e}"))? {
        fork::Fork::Parent(_) => {
            return Ok(ControlFlow::Break(()));
        }
        fork::Fork::Child => {}
    }

    // Once we've detached from the parent, we want to close stdout/stderr/stdin
    // so that the outer SSH process is not attached to us in any way anymore.
    unsafe { redirect_standard_streams() }?;

    Ok(ControlFlow::Continue(()))
}

unsafe fn redirect_standard_streams() -> Result<()> {
    let devnull_fd = unsafe { libc::open(b"/dev/null\0" as *const [u8; 10] as _, libc::O_RDWR) };
    anyhow::ensure!(devnull_fd != -1, "failed to open /dev/null");

    let process_stdio = |name, fd| {
        let reopened_fd = unsafe { libc::dup2(devnull_fd, fd) };
        anyhow::ensure!(
            reopened_fd != -1,
            format!("failed to redirect {} to /dev/null", name)
        );
        Ok(())
    };

    process_stdio("stdin", libc::STDIN_FILENO)?;
    process_stdio("stdout", libc::STDOUT_FILENO)?;
    process_stdio("stderr", libc::STDERR_FILENO)?;

    anyhow::ensure!(
        unsafe { libc::close(devnull_fd) != -1 },
        "failed to close /dev/null fd after redirecting"
    );

    Ok(())
}

fn cleanup_old_binaries() -> Result<()> {
    let server_dir = paths::remote_server_dir_relative();
    let release_channel = release_channel::RELEASE_CHANNEL.dev_name();
    let prefix = format!("zed-remote-server-{}-", release_channel);

    for entry in std::fs::read_dir(server_dir.as_std_path())? {
        let path = entry?.path();

        if let Some(file_name) = path.file_name()
            && let Some(version) = file_name.to_string_lossy().strip_prefix(&prefix)
            && !is_new_version(version)
            && !is_file_in_use(file_name)
        {
            log::info!("removing old remote server binary: {:?}", path);
            std::fs::remove_file(&path)?;
        }
    }

    Ok(())
}

fn is_new_version(version: &str) -> bool {
    SemanticVersion::from_str(version)
        .ok()
        .zip(SemanticVersion::from_str(env!("ZED_PKG_VERSION")).ok())
        .is_some_and(|(version, current_version)| version >= current_version)
}

fn is_file_in_use(file_name: &OsStr) -> bool {
    let info =
        sysinfo::System::new_with_specifics(sysinfo::RefreshKind::new().with_processes(
            sysinfo::ProcessRefreshKind::new().with_exe(sysinfo::UpdateKind::Always),
        ));

    for process in info.processes().values() {
        if process
            .exe()
            .is_some_and(|exe| exe.file_name().is_some_and(|name| name == file_name))
        {
            return true;
        }
    }

    false
}

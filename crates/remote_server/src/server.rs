mod headless_project;

#[cfg(test)]
mod remote_editing_tests;

#[cfg(windows)]
pub mod windows;

pub use headless_project::{HeadlessAppState, HeadlessProject};

use anyhow::{Context as _, Result, anyhow};
use clap::Subcommand;
use client::ProxySettings;
use collections::HashMap;
use extension::ExtensionHostProxy;
use fs::{Fs, RealFs};
use futures::{
    AsyncRead, AsyncWrite, AsyncWriteExt, FutureExt, SinkExt,
    channel::{mpsc, oneshot},
    select, select_biased,
};
use git::GitHostingProviderRegistry;
use gpui::{App, AppContext as _, Context, Entity, UpdateGlobal as _};
use gpui_tokio::Tokio;
use http_client::{Url, read_proxy_from_env};
use language::LanguageRegistry;
use net::async_net::{UnixListener, UnixStream};
use node_runtime::{NodeBinaryOptions, NodeRuntime};
use paths::logs_dir;
use project::{project_settings::ProjectSettings, trusted_worktrees};
use proto::CrashReport;
use release_channel::{AppCommitSha, AppVersion, RELEASE_CHANNEL, ReleaseChannel};
use remote::{
    RemoteClient,
    json_log::LogRecord,
    protocol::{read_message, write_message},
    proxy::ProxyLaunchError,
};
use reqwest_client::ReqwestClient;
use rpc::proto::{self, Envelope, REMOTE_SERVER_PROJECT_ID};
use rpc::{AnyProtoClient, TypedEnvelope};
use settings::{Settings, SettingsStore, watch_config_file};
use smol::{
    channel::{Receiver, Sender},
    io::AsyncReadExt,
    stream::StreamExt as _,
};
use std::{
    env,
    ffi::OsStr,
    fs::File,
    io::Write,
    mem,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, LazyLock},
};
use thiserror::Error;
use util::{ResultExt, command::new_smol_command};

#[derive(Subcommand)]
pub enum Commands {
    Run {
        #[arg(long)]
        log_file: PathBuf,
        #[arg(long)]
        pid_file: PathBuf,
        #[arg(long)]
        stdin_socket: PathBuf,
        #[arg(long)]
        stdout_socket: PathBuf,
        #[arg(long)]
        stderr_socket: PathBuf,
    },
    Proxy {
        #[arg(long)]
        reconnect: bool,
        #[arg(long)]
        identifier: String,
    },
    Version,
}

pub fn run(command: Commands) -> anyhow::Result<()> {
    use anyhow::Context;
    use release_channel::{RELEASE_CHANNEL, ReleaseChannel};

    match command {
        Commands::Run {
            log_file,
            pid_file,
            stdin_socket,
            stdout_socket,
            stderr_socket,
        } => execute_run(
            log_file,
            pid_file,
            stdin_socket,
            stdout_socket,
            stderr_socket,
        ),
        Commands::Proxy {
            identifier,
            reconnect,
        } => execute_proxy(identifier, reconnect).context("running proxy on the remote server"),
        Commands::Version => {
            let release_channel = *RELEASE_CHANNEL;
            match release_channel {
                ReleaseChannel::Stable | ReleaseChannel::Preview => {
                    println!("{}", env!("ZED_PKG_VERSION"))
                }
                ReleaseChannel::Nightly | ReleaseChannel::Dev => {
                    let commit_sha =
                        option_env!("ZED_COMMIT_SHA").unwrap_or(release_channel.dev_name());
                    let build_id = option_env!("ZED_BUILD_ID");
                    if let Some(build_id) = build_id {
                        println!("{}+{}", build_id, commit_sha)
                    } else {
                        println!("{commit_sha}");
                    }
                }
            };
            Ok(())
        }
    }
}

pub static VERSION: LazyLock<String> = LazyLock::new(|| match *RELEASE_CHANNEL {
    ReleaseChannel::Stable | ReleaseChannel::Preview => env!("ZED_PKG_VERSION").to_owned(),
    ReleaseChannel::Nightly | ReleaseChannel::Dev => {
        let commit_sha = option_env!("ZED_COMMIT_SHA").unwrap_or("missing-zed-commit-sha");
        let build_identifier = option_env!("ZED_BUILD_ID");
        if let Some(build_id) = build_identifier {
            format!("{build_id}+{commit_sha}")
        } else {
            commit_sha.to_owned()
        }
    }
});

fn init_logging_proxy() {
    env_logger::builder()
        .format(|buf, record| {
            let mut log_record = LogRecord::new(record);
            log_record.message =
                std::borrow::Cow::Owned(format!("(remote proxy) {}", log_record.message));
            serde_json::to_writer(&mut *buf, &log_record)?;
            buf.write_all(b"\n")?;
            Ok(())
        })
        .init();
}

fn init_logging_server(log_file_path: &Path) -> Result<Receiver<Vec<u8>>> {
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
        .open(log_file_path)
        .context("Failed to open log file in append mode")?;

    let (tx, rx) = smol::channel::unbounded();

    let target = Box::new(MultiWrite {
        file: log_file,
        channel: tx,
        buffer: Vec::new(),
    });

    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let backtrace = std::backtrace::Backtrace::force_capture();
        let message = info.payload_as_str().unwrap_or("Box<Any>").to_owned();
        let location = info
            .location()
            .map_or_else(|| "<unknown>".to_owned(), |location| location.to_string());
        let current_thread = std::thread::current();
        let thread_name = current_thread.name().unwrap_or("<unnamed>");

        let msg = format!("thread '{thread_name}' panicked at {location}:\n{message}\n{backtrace}");
        // NOTE: This log never reaches the client, as the communication is handled on a main thread task
        // which will never run once we panic.
        log::error!("{msg}");
        old_hook(info);
    }));
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .target(env_logger::Target::Pipe(target))
        .format(|buf, record| {
            let mut log_record = LogRecord::new(record);
            log_record.message =
                std::borrow::Cow::Owned(format!("(remote server) {}", log_record.message));
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
    is_wsl_interop: bool,
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
        loop {
            let streams = futures::future::join3(
                listeners.stdin.accept(),
                listeners.stdout.accept(),
                listeners.stderr.accept(),
            );

            log::info!("accepting new connections");
            let result = select! {
                streams = streams.fuse() => {
                    let (Ok((stdin_stream, _)), Ok((stdout_stream, _)), Ok((stderr_stream, _))) = streams else {
                        log::error!("failed to accept new connections");
                        break;
                    };
                    log::info!("accepted new connections");
                    anyhow::Ok((stdin_stream, stdout_stream, stderr_stream))
                }
                _ = futures::FutureExt::fuse(cx.background_executor().timer(IDLE_TIMEOUT)) => {
                    log::warn!("timed out waiting for new connections after {:?}. exiting.", IDLE_TIMEOUT);
                    cx.update(|cx| {
                        // TODO: This is a hack, because in a headless project, shutdown isn't executed
                        // when calling quit, but it should be.
                        cx.shutdown();
                        cx.quit();
                    });
                    break;
                }
                _ = app_quit_rx.next().fuse() => {
                    log::info!("app quit requested");
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
                            log::warn!("error reading message on stdin, dropping connection.");
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

    RemoteClient::proto_client_from_channels(incoming_rx, outgoing_tx, cx, "server", is_wsl_interop)
}

fn init_paths() -> anyhow::Result<()> {
    for path in [
        paths::config_dir(),
        paths::extensions_dir(),
        paths::languages_dir(),
        paths::logs_dir(),
        paths::temp_dir(),
        paths::hang_traces_dir(),
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

    let app = gpui::Application::headless();
    let pid = std::process::id();
    let id = pid.to_string();
    app.background_executor()
        .spawn(crashes::init(crashes::InitCrashHandler {
            session_id: id,
            zed_version: VERSION.to_owned(),
            binary: "zed-remote-server".to_string(),
            release_channel: release_channel::RELEASE_CHANNEL_NAME.clone(),
            commit_sha: option_env!("ZED_COMMIT_SHA").unwrap_or("no_sha").to_owned(),
        }))
        .detach();
    let log_rx = init_logging_server(&log_file)?;
    log::info!(
        "starting up with PID {}:\npid_file: {:?}, log_file: {:?}, stdin_socket: {:?}, stdout_socket: {:?}, stderr_socket: {:?}",
        pid,
        pid_file,
        log_file,
        stdin_socket,
        stdout_socket,
        stderr_socket
    );

    write_pid_file(&pid_file, pid)
        .with_context(|| format!("failed to write pid file: {:?}", &pid_file))?;

    let listeners = ServerListeners::new(stdin_socket, stdout_socket, stderr_socket)?;

    rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism().map_or(1, |n| n.get().div_ceil(2)))
        .stack_size(10 * 1024 * 1024)
        .thread_name(|ix| format!("RayonWorker{}", ix))
        .build_global()
        .unwrap();

    #[cfg(unix)]
    let shell_env_loaded_rx = {
        let (shell_env_loaded_tx, shell_env_loaded_rx) = oneshot::channel();
        app.background_executor()
            .spawn(async {
                util::load_login_shell_environment().await.log_err();
                shell_env_loaded_tx.send(()).ok();
            })
            .detach();
        Some(shell_env_loaded_rx)
    };
    #[cfg(windows)]
    let shell_env_loaded_rx: Option<oneshot::Receiver<()>> = None;

    let git_hosting_provider_registry = Arc::new(GitHostingProviderRegistry::new());
    let run = move |cx: &mut _| {
        settings::init(cx);
        let app_commit_sha = option_env!("ZED_COMMIT_SHA").map(|s| AppCommitSha::new(s.to_owned()));
        let app_version = AppVersion::load(
            env!("ZED_PKG_VERSION"),
            option_env!("ZED_BUILD_ID"),
            app_commit_sha,
        );
        release_channel::init(app_version, cx);
        gpui_tokio::init(cx);

        HeadlessProject::init(cx);

        let is_wsl_interop = if cfg!(target_os = "linux") {
            // See: https://learn.microsoft.com/en-us/windows/wsl/filesystems#disable-interoperability
            matches!(std::fs::read_to_string("/proc/sys/fs/binfmt_misc/WSLInterop"), Ok(s) if s.contains("enabled"))
        } else {
            false
        };

        log::info!("gpui app started, initializing server");
        let session = start_server(listeners, log_rx, cx, is_wsl_interop);
        trusted_worktrees::init(HashMap::default(), cx);

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

            let node_runtime =
                NodeRuntime::new(http_client.clone(), shell_env_loaded_rx, node_settings_rx);

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
                true,
                cx,
            )
        });

        handle_crash_files_requests(&project, &session);

        cx.background_spawn(async move {
            cleanup_old_binaries_wsl();
            cleanup_old_binaries()
        })
        .detach();

        mem::forget(project);
    };
    // We do not reuse any of the state after unwinding, so we don't run risk of observing broken invariants.
    let app = std::panic::AssertUnwindSafe(app);
    let run = std::panic::AssertUnwindSafe(run);
    let res = std::panic::catch_unwind(move || { app }.0.run({ run }.0));
    if let Err(_) = res {
        log::error!("app panicked. quitting.");
        Err(anyhow::anyhow!("panicked"))
    } else {
        log::info!("gpui app is shut down. quitting.");
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ServerPathError {
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
            source,
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
pub enum ExecuteProxyError {
    #[error("Failed to init server paths: {0:#}")]
    ServerPath(#[from] ServerPathError),

    #[error(transparent)]
    ServerNotRunning(#[from] ProxyLaunchError),

    #[error("Failed to check PidFile '{path}': {source:#}")]
    CheckPidFile {
        #[source]
        source: CheckPidError,
        path: PathBuf,
    },

    #[error("Failed to kill existing server with pid '{pid}'")]
    KillRunningServer { pid: u32 },

    #[error("failed to spawn server")]
    SpawnServer(#[source] SpawnServerError),

    #[error("stdin_task failed: {0:#}")]
    StdinTask(#[source] anyhow::Error),
    #[error("stdout_task failed: {0:#}")]
    StdoutTask(#[source] anyhow::Error),
    #[error("stderr_task failed: {0:#}")]
    StderrTask(#[source] anyhow::Error),
}

impl ExecuteProxyError {
    pub fn to_exit_code(&self) -> i32 {
        match self {
            ExecuteProxyError::ServerNotRunning(proxy_launch_error) => {
                proxy_launch_error.to_exit_code()
            }
            _ => 1,
        }
    }
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
        binary: "zed-remote-server".to_string(),
        release_channel: release_channel::RELEASE_CHANNEL_NAME.clone(),
        commit_sha: option_env!("ZED_COMMIT_SHA").unwrap_or("no_sha").to_owned(),
    }))
    .detach();

    log::info!("starting proxy process. PID: {}", std::process::id());
    let server_pid = {
        let server_pid = check_pid_file(&server_paths.pid_file).map_err(|source| {
            ExecuteProxyError::CheckPidFile {
                source,
                path: server_paths.pid_file.clone(),
            }
        })?;
        if is_reconnecting {
            match server_pid {
                None => {
                    log::error!("attempted to reconnect, but no server running");
                    return Err(ExecuteProxyError::ServerNotRunning(
                        ProxyLaunchError::ServerNotRunning,
                    ));
                }
                Some(server_pid) => server_pid,
            }
        } else {
            if let Some(pid) = server_pid {
                log::info!(
                    "proxy found server already running with PID {}. Killing process and cleaning up files...",
                    pid
                );
                kill_running_server(pid, &server_paths)?;
            }
            smol::block_on(spawn_server(&server_paths)).map_err(ExecuteProxyError::SpawnServer)?;
            std::fs::read_to_string(&server_paths.pid_file)
                .and_then(|contents| {
                    contents.parse::<u32>().map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid PID file contents",
                        )
                    })
                })
                .map_err(SpawnServerError::ProcessStatus)
                .map_err(ExecuteProxyError::SpawnServer)?
        }
    };

    let stdin_task = smol::spawn(async move {
        let stdin = smol::Unblock::new(std::io::stdin());
        let stream = UnixStream::connect(&server_paths.stdin_socket)
            .await
            .with_context(|| {
                format!(
                    "Failed to connect to stdin socket {}",
                    server_paths.stdin_socket.display()
                )
            })?;
        handle_io(stdin, stream, "stdin").await
    });

    let stdout_task: smol::Task<Result<()>> = smol::spawn(async move {
        let stdout = smol::Unblock::new(std::io::stdout());
        let stream = UnixStream::connect(&server_paths.stdout_socket)
            .await
            .with_context(|| {
                format!(
                    "Failed to connect to stdout socket {}",
                    server_paths.stdout_socket.display()
                )
            })?;
        handle_io(stream, stdout, "stdout").await
    });

    let stderr_task: smol::Task<Result<()>> = smol::spawn(async move {
        let mut stderr = smol::Unblock::new(std::io::stderr());
        let mut stream = UnixStream::connect(&server_paths.stderr_socket)
            .await
            .with_context(|| {
                format!(
                    "Failed to connect to stderr socket {}",
                    server_paths.stderr_socket.display()
                )
            })?;
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
        log::error!("encountered error while forwarding messages: {forwarding_result:#}",);
        if !matches!(smol::block_on(check_server_running(server_pid)), Ok(true)) {
            log::error!("server exited unexpectedly");
            return Err(ExecuteProxyError::ServerNotRunning(
                ProxyLaunchError::ServerNotRunning,
            ));
        }
        return Err(forwarding_result);
    }

    Ok(())
}

fn kill_running_server(pid: u32, paths: &ServerPaths) -> Result<(), ExecuteProxyError> {
    log::info!("killing existing server with PID {}", pid);
    let system = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing()),
    );

    if let Some(process) = system.process(sysinfo::Pid::from_u32(pid)) {
        let killed = process.kill();
        if !killed {
            return Err(ExecuteProxyError::KillRunningServer { pid });
        }
    }

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
pub enum SpawnServerError {
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

    #[error("failed to wait for server to be ready to accept connections")]
    Timeout,
}

async fn spawn_server(paths: &ServerPaths) -> Result<(), SpawnServerError> {
    log::info!("spawning server process",);
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

    #[cfg(windows)]
    {
        spawn_server_windows(&binary_name, paths)?;
    }

    #[cfg(not(windows))]
    {
        spawn_server_normal(&binary_name, paths)?;
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
        if total_time_waited > std::time::Duration::from_secs(10) {
            return Err(SpawnServerError::Timeout);
        }
    }

    log::info!(
        "server ready to accept connections. total time waited: {:?}",
        total_time_waited
    );

    Ok(())
}

#[cfg(windows)]
fn spawn_server_windows(binary_name: &Path, paths: &ServerPaths) -> Result<(), SpawnServerError> {
    let binary_path = binary_name.to_string_lossy().to_string();
    let parameters = format!(
        "run --log-file \"{}\" --pid-file \"{}\" --stdin-socket \"{}\" --stdout-socket \"{}\" --stderr-socket \"{}\"",
        paths.log_file.to_string_lossy(),
        paths.pid_file.to_string_lossy(),
        paths.stdin_socket.to_string_lossy(),
        paths.stdout_socket.to_string_lossy(),
        paths.stderr_socket.to_string_lossy()
    );

    let directory = binary_name
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    crate::windows::shell_execute_from_explorer(&binary_path, &parameters, &directory)
        .map_err(|e| SpawnServerError::ProcessStatus(std::io::Error::other(e)))?;

    Ok(())
}

#[cfg(not(windows))]
fn spawn_server_normal(binary_name: &Path, paths: &ServerPaths) -> Result<(), SpawnServerError> {
    let mut server_process = new_smol_command(binary_name);
    server_process
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
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

    server_process
        .spawn()
        .map_err(SpawnServerError::ProcessStatus)?;

    Ok(())
}

#[derive(Debug, Error)]
#[error("Failed to remove PID file for missing process (pid `{pid}`")]
pub struct CheckPidError {
    #[source]
    source: std::io::Error,
    pid: u32,
}
async fn check_server_running(pid: u32) -> std::io::Result<bool> {
    new_smol_command("kill")
        .arg("-0")
        .arg(pid.to_string())
        .output()
        .await
        .map(|output| output.status.success())
}

fn check_pid_file(path: &Path) -> Result<Option<u32>, CheckPidError> {
    let Some(pid) = std::fs::read_to_string(path)
        .ok()
        .and_then(|contents| contents.parse::<u32>().ok())
    else {
        return Ok(None);
    };

    log::debug!("Checking if process with PID {} exists...", pid);

    let system = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing()),
    );

    if system.process(sysinfo::Pid::from_u32(pid)).is_some() {
        log::debug!(
            "Process with PID {} exists. NOT spawning new server, but attaching to existing one.",
            pid
        );
        Ok(Some(pid))
    } else {
        log::debug!("Found PID file, but process with that PID does not exist. Removing PID file.");
        std::fs::remove_file(path).map_err(|source| CheckPidError { source, pid })?;
        Ok(None)
    }
}

fn write_pid_file(path: &Path, pid: u32) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    log::debug!("writing PID {} to file {:?}", pid, path);
    std::fs::write(path, pid.to_string()).context("Failed to write PID file")
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
    let (user_settings_file_rx, watcher_task) =
        watch_config_file(cx.background_executor(), fs, paths::settings_file().clone());

    handle_settings_file_changes(user_settings_file_rx, watcher_task, cx, {
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
    watcher_task: gpui::Task<()>,
    cx: &mut App,
    settings_changed: impl Fn(Option<anyhow::Error>, &mut App) + 'static,
) {
    let server_settings_content = cx
        .foreground_executor()
        .block_on(server_settings_file.next())
        .unwrap();
    SettingsStore::update_global(cx, |store, cx| {
        store
            .set_server_settings(&server_settings_content, cx)
            .log_err();
    });
    cx.spawn(async move |cx| {
        let _watcher_task = watcher_task;
        while let Some(server_settings_content) = server_settings_file.next().await {
            cx.update_global(|store: &mut SettingsStore, cx| {
                let result = store.set_server_settings(&server_settings_content, cx);
                if let Err(err) = &result {
                    log::error!("Failed to load server settings: {err}");
                }
                settings_changed(result.err(), cx);
                cx.refresh_windows();
            });
        }
    })
    .detach();
}

fn read_proxy_settings(cx: &mut Context<HeadlessProject>) -> Option<Url> {
    let proxy_str = ProxySettings::get_global(cx).proxy.to_owned();

    proxy_str
        .as_deref()
        .map(str::trim)
        .filter(|input| !input.is_empty())
        .and_then(|input| {
            input
                .parse::<Url>()
                .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                .ok()
        })
        .or_else(read_proxy_from_env)
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

// Remove this once 223 goes stable, we only have this to clean up old binaries on WSL
// we no longer download them into this folder, we use the same folder as other remote servers
fn cleanup_old_binaries_wsl() {
    let server_dir = paths::remote_wsl_server_dir_relative();
    if let Ok(()) = std::fs::remove_dir_all(server_dir.as_std_path()) {
        log::info!("removing old wsl remote server folder: {:?}", server_dir);
    }
}

fn is_new_version(version: &str) -> bool {
    semver::Version::from_str(version)
        .ok()
        .zip(semver::Version::from_str(env!("ZED_PKG_VERSION")).ok())
        .is_some_and(|(version, current_version)| version >= current_version)
}

fn is_file_in_use(file_name: &OsStr) -> bool {
    let info = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(
        sysinfo::ProcessRefreshKind::nothing().with_exe(sysinfo::UpdateKind::Always),
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

use crate::{
    RemoteArch, RemoteClientDelegate, RemoteOs, RemotePlatform,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
    transport::{parse_platform, parse_shell},
};
use anyhow::{Context, Result, anyhow, bail};
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext as _, AsyncApp, Task};
use release_channel::{AppVersion, ReleaseChannel};
use rpc::proto::Envelope;
use semver::Version;
use smol::{fs, process};
use std::{
    ffi::OsStr,
    fmt::Write as _,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Instant,
};
use util::{
    paths::{PathStyle, RemotePathBuf},
    rel_path::RelPath,
    shell::{Shell, ShellKind},
    shell_builder::ShellBuilder,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, schemars::JsonSchema)]
pub struct WslConnectionOptions {
    pub distro_name: String,
    pub user: Option<String>,
}

impl From<settings::WslConnection> for WslConnectionOptions {
    fn from(val: settings::WslConnection) -> Self {
        WslConnectionOptions {
            distro_name: val.distro_name.into(),
            user: val.user,
        }
    }
}

#[derive(Debug)]
pub(crate) struct WslRemoteConnection {
    remote_binary_path: Option<Arc<RelPath>>,
    platform: RemotePlatform,
    shell: String,
    shell_kind: ShellKind,
    default_system_shell: String,
    has_wsl_interop: bool,
    connection_options: WslConnectionOptions,
}

impl WslRemoteConnection {
    pub(crate) async fn new(
        connection_options: WslConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        log::info!(
            "Connecting to WSL distro {} with user {:?}",
            connection_options.distro_name,
            connection_options.user
        );
        let (release_channel, version) =
            cx.update(|cx| (ReleaseChannel::global(cx), AppVersion::global(cx)));

        let mut this = Self {
            connection_options,
            remote_binary_path: None,
            platform: RemotePlatform {
                os: RemoteOs::Linux,
                arch: RemoteArch::X86_64,
            },
            shell: String::new(),
            shell_kind: ShellKind::Posix,
            default_system_shell: String::from("/bin/sh"),
            has_wsl_interop: false,
        };
        delegate.set_status(Some("Detecting WSL environment"), cx);
        this.shell = this
            .detect_shell()
            .await
            .context("failed detecting shell")?;
        log::info!("Remote shell discovered: {}", this.shell);
        this.shell_kind = ShellKind::new(&this.shell, false);
        this.has_wsl_interop = this.detect_has_wsl_interop().await.unwrap_or_default();
        log::info!(
            "Remote has wsl interop {}",
            if this.has_wsl_interop {
                "enabled"
            } else {
                "disabled"
            }
        );
        this.platform = this
            .detect_platform()
            .await
            .context("failed detecting platform")?;
        log::info!("Remote platform discovered: {:?}", this.platform);
        this.remote_binary_path = Some(
            this.ensure_server_binary(&delegate, release_channel, version, cx)
                .await
                .context("failed ensuring server binary")?,
        );
        log::debug!("Detected WSL environment: {this:#?}");

        Ok(this)
    }

    async fn detect_platform(&self) -> Result<RemotePlatform> {
        let program = self.shell_kind.prepend_command_prefix("uname");
        let output = self.run_wsl_command_with_output(&program, &["-sm"]).await?;
        parse_platform(&output)
    }

    async fn detect_shell(&self) -> Result<String> {
        const DEFAULT_SHELL: &str = "sh";
        match self
            .run_wsl_command_with_output("sh", &["-c", "echo $SHELL"])
            .await
        {
            Ok(output) => Ok(parse_shell(&output, DEFAULT_SHELL)),
            Err(e) => {
                log::error!("Failed to detect remote shell: {e}");
                Ok(DEFAULT_SHELL.to_owned())
            }
        }
    }

    async fn detect_has_wsl_interop(&self) -> Result<bool> {
        Ok(self
            .run_wsl_command_with_output("cat", &["/proc/sys/fs/binfmt_misc/WSLInterop"])
            .await
            .inspect_err(|err| log::error!("Failed to detect wsl interop: {err}"))?
            .contains("enabled"))
    }

    async fn windows_path_to_wsl_path(&self, source: &Path) -> Result<String> {
        windows_path_to_wsl_path_impl(&self.connection_options, source).await
    }

    async fn run_wsl_command_with_output(&self, program: &str, args: &[&str]) -> Result<String> {
        run_wsl_command_with_output_impl(&self.connection_options, program, args).await
    }

    async fn run_wsl_command(&self, program: &str, args: &[&str]) -> Result<()> {
        run_wsl_command_impl(&self.connection_options, program, args, false)
            .await
            .map(|_| ())
    }

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn RemoteClientDelegate>,
        release_channel: ReleaseChannel,
        version: Version,
        cx: &mut AsyncApp,
    ) -> Result<Arc<RelPath>> {
        let version_str = match release_channel {
            ReleaseChannel::Dev => "build".to_string(),
            _ => version.to_string(),
        };

        let binary_name = format!(
            "zed-remote-server-{}-{}",
            release_channel.dev_name(),
            version_str
        );

        let dst_path =
            paths::remote_wsl_server_dir_relative().join(RelPath::unix(&binary_name).unwrap());

        if let Some(parent) = dst_path.parent() {
            let parent = parent.display(PathStyle::Posix);
            let mkdir = self.shell_kind.prepend_command_prefix("mkdir");
            self.run_wsl_command(&mkdir, &["-p", &parent])
                .await
                .map_err(|e| anyhow!("Failed to create directory: {}", e))?;
        }

        #[cfg(debug_assertions)]
        if let Some(remote_server_path) =
            super::build_remote_server_from_source(&self.platform, delegate.as_ref(), cx).await?
        {
            let tmp_path = paths::remote_wsl_server_dir_relative().join(
                &RelPath::unix(&format!(
                    "download-{}-{}",
                    std::process::id(),
                    remote_server_path.file_name().unwrap().to_string_lossy()
                ))
                .unwrap(),
            );
            self.upload_file(&remote_server_path, &tmp_path, delegate, cx)
                .await?;
            self.extract_and_install(&tmp_path, &dst_path, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .run_wsl_command(&dst_path.display(PathStyle::Posix), &["version"])
            .await
            .is_ok()
        {
            return Ok(dst_path);
        }

        let wanted_version = match release_channel {
            ReleaseChannel::Nightly | ReleaseChannel::Dev => None,
            _ => Some(cx.update(|cx| AppVersion::global(cx))),
        };

        let src_path = delegate
            .download_server_binary_locally(self.platform, release_channel, wanted_version, cx)
            .await?;

        let tmp_path = format!(
            "{}.{}.gz",
            dst_path.display(PathStyle::Posix),
            std::process::id()
        );
        let tmp_path = RelPath::unix(&tmp_path).unwrap();

        self.upload_file(&src_path, &tmp_path, delegate, cx).await?;
        self.extract_and_install(&tmp_path, &dst_path, delegate, cx)
            .await?;

        Ok(dst_path)
    }

    async fn upload_file(
        &self,
        src_path: &Path,
        dst_path: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Uploading remote server"), cx);

        if let Some(parent) = dst_path.parent() {
            let parent = parent.display(PathStyle::Posix);
            let mkdir = self.shell_kind.prepend_command_prefix("mkdir");
            self.run_wsl_command(&mkdir, &["-p", &parent])
                .await
                .context("Failed to create directory when uploading file")?;
        }

        let t0 = Instant::now();
        let src_stat = fs::metadata(&src_path)
            .await
            .with_context(|| format!("source path does not exist: {}", src_path.display()))?;
        let size = src_stat.len();
        log::info!(
            "uploading remote server to WSL {:?} ({}kb)",
            dst_path,
            size / 1024
        );

        let src_path_in_wsl = self.windows_path_to_wsl_path(src_path).await?;
        let cp = self.shell_kind.prepend_command_prefix("cp");
        self.run_wsl_command(
            &cp,
            &["-f", &src_path_in_wsl, &dst_path.display(PathStyle::Posix)],
        )
        .await
        .map_err(|e| {
            anyhow!(
                "Failed to copy file {}({}) to WSL {:?}: {}",
                src_path.display(),
                src_path_in_wsl,
                dst_path,
                e
            )
        })?;

        log::info!("uploaded remote server in {:?}", t0.elapsed());
        Ok(())
    }

    async fn extract_and_install(
        &self,
        tmp_path: &RelPath,
        dst_path: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote server"), cx);

        let tmp_path_str = tmp_path.display(PathStyle::Posix);
        let dst_path_str = dst_path.display(PathStyle::Posix);

        // Build extraction script with proper error handling
        let script = if tmp_path_str.ends_with(".gz") {
            let uncompressed = tmp_path_str.trim_end_matches(".gz");
            format!(
                "set -e; gunzip -f '{}' && chmod 755 '{}' && mv -f '{}' '{}'",
                tmp_path_str, uncompressed, uncompressed, dst_path_str
            )
        } else {
            format!(
                "set -e; chmod 755 '{}' && mv -f '{}' '{}'",
                tmp_path_str, tmp_path_str, dst_path_str
            )
        };

        self.run_wsl_command("sh", &["-c", &script])
            .await
            .map_err(|e| anyhow!("Failed to extract server binary: {}", e))?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl RemoteConnection for WslRemoteConnection {
    fn start_proxy(
        &self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>> {
        delegate.set_status(Some("Starting proxy"), cx);

        let Some(remote_binary_path) = &self.remote_binary_path else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut proxy_args = vec![];
        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                // We don't quote the value here as it seems excessive and may result in invalid envs for the
                // proxy server. For example, `RUST_LOG='debug'` will result in a warning "invalid logging spec 'debug'', ignoring it"
                // in the proxy server. Therefore, we pass the env vars as is.
                proxy_args.push(format!("{}={}", env_var, value));
            }
        }

        proxy_args.push(remote_binary_path.display(PathStyle::Posix).into_owned());
        proxy_args.push("proxy".to_owned());
        proxy_args.push("--identifier".to_owned());
        proxy_args.push(unique_identifier);

        if reconnect {
            proxy_args.push("--reconnect".to_owned());
        }

        let proxy_process =
            match wsl_command_impl(&self.connection_options, "env", &proxy_args, false)
                .kill_on_drop(true)
                .spawn()
            {
                Ok(process) => process,
                Err(error) => {
                    return Task::ready(Err(anyhow!("failed to spawn remote server: {}", error)));
                }
            };

        super::handle_rpc_messages_over_child_process_stdio(
            proxy_process,
            incoming_tx,
            outgoing_rx,
            connection_activity_tx,
            cx,
        )
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        cx.background_spawn({
            let options = self.connection_options.clone();
            async move {
                let wsl_src = windows_path_to_wsl_path_impl(&options, &src_path).await?;

                run_wsl_command_impl(
                    &options,
                    "cp",
                    &["-r", &wsl_src, &dest_path.to_string()],
                    true,
                )
                .await
                .map_err(|e| {
                    anyhow!(
                        "failed to upload directory {} -> {}: {}",
                        src_path.display(),
                        dest_path,
                        e
                    )
                })?;

                Ok(())
            }
        })
    }

    async fn kill(&self) -> Result<()> {
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        false
    }

    fn shares_network_interface(&self) -> bool {
        true
    }

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        if port_forward.is_some() {
            bail!("WSL shares the network interface with the host system");
        }

        let shell_kind = self.shell_kind;
        let working_dir = working_dir
            .map(|working_dir| RemotePathBuf::new(working_dir, PathStyle::Posix).to_string())
            .unwrap_or("~".to_string());

        let mut exec = String::from("exec env ");

        for (k, v) in env.iter() {
            write!(
                exec,
                "{}={} ",
                k,
                shell_kind.try_quote(v).context("shell quoting")?
            )?;
        }

        if let Some(program) = program {
            write!(
                exec,
                "{}",
                shell_kind
                    .try_quote_prefix_aware(&program)
                    .context("shell quoting")?
            )?;
            for arg in args {
                let arg = shell_kind.try_quote(&arg).context("shell quoting")?;
                write!(exec, " {}", &arg)?;
            }
        } else {
            write!(&mut exec, "{} -l", self.shell)?;
        }
        let (command, args) =
            ShellBuilder::new(&Shell::Program(self.shell.clone()), false).build(Some(exec), &[]);

        let mut wsl_args = if let Some(user) = &self.connection_options.user {
            vec![
                "--distribution".to_string(),
                self.connection_options.distro_name.clone(),
                "--user".to_string(),
                user.clone(),
                "--cd".to_string(),
                working_dir,
                "--".to_string(),
                command,
            ]
        } else {
            vec![
                "--distribution".to_string(),
                self.connection_options.distro_name.clone(),
                "--cd".to_string(),
                working_dir,
                "--".to_string(),
                command,
            ]
        };
        wsl_args.extend(args);

        Ok(CommandTemplate {
            program: "wsl.exe".to_string(),
            args: wsl_args,
            env: HashMap::default(),
        })
    }

    fn build_forward_ports_command(
        &self,
        _: Vec<(u16, String, u16)>,
    ) -> anyhow::Result<CommandTemplate> {
        Err(anyhow!("WSL shares a network interface with the host"))
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Wsl(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        PathStyle::Posix
    }

    fn shell(&self) -> String {
        self.shell.clone()
    }

    fn default_system_shell(&self) -> String {
        self.default_system_shell.clone()
    }

    fn has_wsl_interop(&self) -> bool {
        self.has_wsl_interop
    }
}

/// `wslpath` is a executable available in WSL, it's a linux binary.
/// So it doesn't support Windows style paths.
async fn sanitize_path(path: &Path) -> Result<String> {
    let path = smol::fs::canonicalize(path)
        .await
        .with_context(|| format!("Failed to canonicalize path {}", path.display()))?;
    let path_str = path.to_string_lossy();

    let sanitized = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);
    Ok(sanitized.replace('\\', "/"))
}

async fn run_wsl_command_with_output_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[&str],
) -> Result<String> {
    match run_wsl_command_impl(options, program, args, true).await {
        Ok(res) => Ok(res),
        Err(exec_err) => match run_wsl_command_impl(options, program, args, false).await {
            Ok(res) => Ok(res),
            Err(e) => Err(e.context(exec_err)),
        },
    }
}

async fn windows_path_to_wsl_path_impl(
    options: &WslConnectionOptions,
    source: &Path,
) -> Result<String> {
    let source = sanitize_path(source).await?;
    run_wsl_command_with_output_impl(options, "wslpath", &["-u", &source]).await
}

async fn run_wsl_command_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[&str],
    exec: bool,
) -> Result<String> {
    let mut command = wsl_command_impl(options, program, args, exec);
    let output = command
        .output()
        .await
        .with_context(|| format!("Failed to run command '{:?}'", command))?;

    if !output.status.success() {
        return Err(anyhow!(
            "Command '{:?}' failed: {}",
            command,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Creates a new `wsl.exe` command that runs the given program with the given arguments.
///
/// If `exec` is true, the command will be executed in the WSL environment without spawning a new shell.
fn wsl_command_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[impl AsRef<OsStr>],
    exec: bool,
) -> process::Command {
    let mut command = util::command::new_smol_command("wsl.exe");

    if let Some(user) = &options.user {
        command.arg("--user").arg(user);
    }

    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--distribution")
        .arg(&options.distro_name)
        .arg("--cd")
        .arg("~");

    if exec {
        command.arg("--exec");
    }

    command.arg(program).args(args);

    log::debug!("wsl {:?}", command);
    command
}

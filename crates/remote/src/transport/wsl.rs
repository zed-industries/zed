use crate::{
    RemoteClientDelegate, RemotePlatform,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};
use anyhow::{Result, anyhow, bail};
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext as _, AsyncApp, SemanticVersion, Task};
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use rpc::proto::Envelope;
use smol::{fs, process};
use std::{
    fmt::Write as _,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Instant,
};
use util::paths::{PathStyle, RemotePathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

pub(crate) struct WslRemoteConnection {
    remote_binary_path: Option<RemotePathBuf>,
    platform: RemotePlatform,
    shell: String,
    default_system_shell: String,
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
        let (release_channel, version, commit) = cx.update(|cx| {
            (
                ReleaseChannel::global(cx),
                AppVersion::global(cx),
                AppCommitSha::try_global(cx),
            )
        })?;

        let mut this = Self {
            connection_options,
            remote_binary_path: None,
            platform: RemotePlatform { os: "", arch: "" },
            shell: String::new(),
            default_system_shell: String::from("/bin/sh"),
        };
        delegate.set_status(Some("Detecting WSL environment"), cx);
        this.platform = this.detect_platform().await?;
        this.shell = this.detect_shell().await?;
        this.remote_binary_path = Some(
            this.ensure_server_binary(&delegate, release_channel, version, commit, cx)
                .await?,
        );

        Ok(this)
    }

    async fn detect_platform(&self) -> Result<RemotePlatform> {
        let arch_str = self.run_wsl_command("uname", &["-m"]).await?;
        let arch_str = arch_str.trim().to_string();
        let arch = match arch_str.as_str() {
            "x86_64" => "x86_64",
            "aarch64" | "arm64" => "aarch64",
            _ => "x86_64",
        };
        Ok(RemotePlatform { os: "linux", arch })
    }

    async fn detect_shell(&self) -> Result<String> {
        Ok(self
            .run_wsl_command("sh", &["-c", "echo $SHELL"])
            .await
            .ok()
            .and_then(|shell_path| {
                Path::new(shell_path.trim())
                    .file_name()
                    .map(|it| it.to_str().unwrap().to_owned())
            })
            .unwrap_or_else(|| "bash".to_string()))
    }

    async fn windows_path_to_wsl_path(&self, source: &Path) -> Result<String> {
        windows_path_to_wsl_path_impl(&self.connection_options, source).await
    }

    fn wsl_command(&self, program: &str, args: &[&str]) -> process::Command {
        wsl_command_impl(&self.connection_options, program, args)
    }

    async fn run_wsl_command(&self, program: &str, args: &[&str]) -> Result<String> {
        run_wsl_command_impl(&self.connection_options, program, args).await
    }

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn RemoteClientDelegate>,
        release_channel: ReleaseChannel,
        version: SemanticVersion,
        commit: Option<AppCommitSha>,
        cx: &mut AsyncApp,
    ) -> Result<RemotePathBuf> {
        let version_str = match release_channel {
            ReleaseChannel::Nightly => {
                let commit = commit.map(|s| s.full()).unwrap_or_default();
                format!("{}-{}", version, commit)
            }
            ReleaseChannel::Dev => "build".to_string(),
            _ => version.to_string(),
        };

        let binary_name = format!(
            "zed-remote-server-{}-{}",
            release_channel.dev_name(),
            version_str
        );

        let dst_path = RemotePathBuf::new(
            paths::remote_wsl_server_dir_relative().join(binary_name),
            PathStyle::Posix,
        );

        if let Some(parent) = dst_path.parent() {
            self.run_wsl_command("mkdir", &["-p", &parent.to_string()])
                .await
                .map_err(|e| anyhow!("Failed to create directory: {}", e))?;
        }

        #[cfg(debug_assertions)]
        if let Some(remote_server_path) =
            super::build_remote_server_from_source(&self.platform, delegate.as_ref(), cx).await?
        {
            let tmp_path = RemotePathBuf::new(
                paths::remote_wsl_server_dir_relative().join(format!(
                    "download-{}-{}",
                    std::process::id(),
                    remote_server_path.file_name().unwrap().to_string_lossy()
                )),
                PathStyle::Posix,
            );
            self.upload_file(&remote_server_path, &tmp_path, delegate, cx)
                .await?;
            self.extract_and_install(&tmp_path, &dst_path, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .run_wsl_command(&dst_path.to_string(), &["version"])
            .await
            .is_ok()
        {
            return Ok(dst_path);
        }

        delegate.set_status(Some("Installing remote server"), cx);

        let wanted_version = match release_channel {
            ReleaseChannel::Nightly | ReleaseChannel::Dev => None,
            _ => Some(cx.update(|cx| AppVersion::global(cx))?),
        };

        let src_path = delegate
            .download_server_binary_locally(self.platform, release_channel, wanted_version, cx)
            .await?;

        let tmp_path = RemotePathBuf::new(
            PathBuf::from(format!("{}.{}.gz", dst_path, std::process::id())),
            PathStyle::Posix,
        );

        self.upload_file(&src_path, &tmp_path, delegate, cx).await?;
        self.extract_and_install(&tmp_path, &dst_path, delegate, cx)
            .await?;

        Ok(dst_path)
    }

    async fn upload_file(
        &self,
        src_path: &Path,
        dst_path: &RemotePathBuf,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Uploading remote server to WSL"), cx);

        if let Some(parent) = dst_path.parent() {
            self.run_wsl_command("mkdir", &["-p", &parent.to_string()])
                .await
                .map_err(|e| anyhow!("Failed to create directory when uploading file: {}", e))?;
        }

        let t0 = Instant::now();
        let src_stat = fs::metadata(&src_path).await?;
        let size = src_stat.len();
        log::info!(
            "uploading remote server to WSL {:?} ({}kb)",
            dst_path,
            size / 1024
        );

        let src_path_in_wsl = self.windows_path_to_wsl_path(src_path).await?;
        self.run_wsl_command("cp", &["-f", &src_path_in_wsl, &dst_path.to_string()])
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
        tmp_path: &RemotePathBuf,
        dst_path: &RemotePathBuf,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote server"), cx);

        let tmp_path_str = tmp_path.to_string();
        let dst_path_str = dst_path.to_string();

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

        let mut proxy_command = format!(
            "exec {} proxy --identifier {}",
            remote_binary_path, unique_identifier
        );

        if reconnect {
            proxy_command.push_str(" --reconnect");
        }

        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                proxy_command = format!("{}='{}' {}", env_var, value, proxy_command);
            }
        }
        let proxy_process = match self
            .wsl_command("sh", &["-lc", &proxy_command])
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

                run_wsl_command_impl(&options, "cp", &["-r", &wsl_src, &dest_path.to_string()])
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "failed to upload directory {} -> {}: {}",
                            src_path.display(),
                            dest_path.to_string(),
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

        let working_dir = working_dir
            .map(|working_dir| RemotePathBuf::new(working_dir.into(), PathStyle::Posix).to_string())
            .unwrap_or("~".to_string());

        let mut script = String::new();

        for (k, v) in env.iter() {
            write!(&mut script, "{}='{}' ", k, v).unwrap();
        }

        if let Some(program) = program {
            let command = shlex::try_quote(&program)?;
            script.push_str(&command);
            for arg in args {
                let arg = shlex::try_quote(&arg)?;
                script.push_str(" ");
                script.push_str(&arg);
            }
        } else {
            write!(&mut script, "exec {} -l", self.shell).unwrap();
        }

        let wsl_args = if let Some(user) = &self.connection_options.user {
            vec![
                "--distribution".to_string(),
                self.connection_options.distro_name.clone(),
                "--user".to_string(),
                user.clone(),
                "--cd".to_string(),
                working_dir,
                "--".to_string(),
                self.shell.clone(),
                "-c".to_string(),
                script,
            ]
        } else {
            vec![
                "--distribution".to_string(),
                self.connection_options.distro_name.clone(),
                "--cd".to_string(),
                working_dir,
                "--".to_string(),
                self.shell.clone(),
                "-c".to_string(),
                script,
            ]
        };

        Ok(CommandTemplate {
            program: "wsl.exe".to_string(),
            args: wsl_args,
            env: HashMap::default(),
        })
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
}

/// `wslpath` is a executable available in WSL, it's a linux binary.
/// So it doesn't support Windows style paths.
async fn sanitize_path(path: &Path) -> Result<String> {
    let path = smol::fs::canonicalize(path).await?;
    let path_str = path.to_string_lossy();

    let sanitized = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);
    Ok(sanitized.replace('\\', "/"))
}

async fn windows_path_to_wsl_path_impl(
    options: &WslConnectionOptions,
    source: &Path,
) -> Result<String> {
    let source = sanitize_path(source).await?;
    run_wsl_command_impl(options, "wslpath", &["-u", &source]).await
}

fn wsl_command_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[&str],
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
        .arg("~")
        .arg(program)
        .args(args);

    command
}

async fn run_wsl_command_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[&str],
) -> Result<String> {
    let output = wsl_command_impl(options, program, args).output().await?;

    if !output.status.success() {
        return Err(anyhow!(
            "Command '{}' failed: {}",
            program,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

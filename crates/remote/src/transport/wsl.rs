use crate::{
    RemoteClientDelegate, RemotePlatform,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext as _, AsyncApp, SemanticVersion, Task};
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use rpc::proto::Envelope;
use smol::fs;
use std::{
    fmt::Write as _,
    path::{self, Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Instant,
};
use util::paths::{PathStyle, RemotePathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WslConnectionOptions {
    pub distro_name: String,
}

pub(crate) struct WslRemoteConnection {
    remote_binary_path: Option<RemotePathBuf>,
    platform: RemotePlatform,
    shell: String,
    connection_options: WslConnectionOptions,
}

impl WslRemoteConnection {
    pub(crate) async fn new(
        connection_options: WslConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
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
        let arch_str = self.wsl_command("uname", &["-m"]).await?;
        let arch_str = arch_str.trim().to_string();
        let arch = match arch_str.as_str() {
            "x86_64" => "x86_64",
            "aarch64" | "arm64" => "aarch64",
            _ => "x86_64",
        };
        Ok(RemotePlatform { os: "linux", arch })
    }

    async fn detect_shell(&self) -> Result<String> {
        let shell = self
            .wsl_command("sh", &["-c", "echo $SHELL"])
            .await
            .unwrap_or_else(|_| "bash".to_string());
        Ok(shell.trim().split('/').last().unwrap_or("bash").to_string())
    }

    async fn wsl_command(&self, program: &str, args: &[&str]) -> Result<String> {
        Self::wsl_command_impl(&self.connection_options, program, args).await
    }

    async fn wsl_command_impl(
        options: &WslConnectionOptions,
        program: &str,
        args: &[&str],
    ) -> Result<String> {
        let mut command = util::command::new_smol_command("wsl.exe");
        let output = command
            .arg("--distribution")
            .arg(&options.distro_name)
            .arg("--")
            .arg(program)
            .args(args)
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!(
                "Command '{}' failed: {}",
                program,
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
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
            PathBuf::from(format!(".zed/remote-servers/{}", binary_name)),
            PathStyle::Posix,
        );

        if self
            .wsl_command(&dst_path.to_string(), &["version"])
            .await
            .is_ok()
        {
            return Ok(dst_path);
        }

        delegate.set_status(Some("Installing remote server"), cx);

        if let Some(parent) = dst_path.parent() {
            self.wsl_command("mkdir", &["-p", &parent.to_string()])
                .await
                .map_err(|e| anyhow!("Failed to create directory: {}", e))?;
        }

        let wanted_version = match release_channel {
            ReleaseChannel::Nightly => None,
            ReleaseChannel::Dev => {
                return Err(anyhow!("Dev builds require manual installation"));
            }
            _ => Some(cx.update(|cx| AppVersion::global(cx))?),
        };

        let src_path = delegate
            .download_server_binary_locally(self.platform, release_channel, wanted_version, cx)
            .await?;

        let tmp_path = RemotePathBuf::new(
            PathBuf::from(format!("{}.{}.tmp", dst_path, std::process::id())),
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

        let t0 = Instant::now();
        let src_stat = fs::metadata(&src_path).await?;
        let size = src_stat.len();
        log::info!(
            "uploading remote server to WSL {:?} ({}kb)",
            dst_path,
            size / 1024
        );

        let src_path_in_wsl = path_to_wsl(&src_path);
        self.wsl_command("cp", &["-f", &src_path_in_wsl, &dst_path.to_string()])
            .await
            .map_err(|e| anyhow!("Failed to copy file to WSL: {}", e))?;

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

        self.wsl_command("sh", &["-c", &script])
            .await
            .map_err(|e| anyhow!("Failed to extract server binary: {}", e))?;
        Ok(())
    }
}

fn path_to_wsl(path: &Path) -> String {
    let mut components = path.components();

    if let Some(path::Component::Prefix(prefix)) = components.next() {
        if let path::Prefix::Disk(drive_byte) | path::Prefix::VerbatimDisk(drive_byte) =
            prefix.kind()
        {
            let drive_letter = (drive_byte as char).to_ascii_lowercase();
            let mut wsl_path = format!("/mnt/{}", drive_letter);
            for component in components.skip(1) {
                if let path::Component::Normal(part) = component {
                    wsl_path.push('/');
                    wsl_path.push_str(&part.to_string_lossy());
                }
            }

            return wsl_path;
        }
    }
    path.to_string_lossy().replace('\\', "/")
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

        let Some(remote_binary_path) = self.remote_binary_path.clone() else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut proxy_command = format!(
            "exec {} proxy --identifier {}",
            remote_binary_path.to_string(),
            unique_identifier
        );

        if reconnect {
            proxy_command.push_str(" --reconnect");
        }

        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                proxy_command = format!("{}='{}' {}", env_var, value, proxy_command);
            }
        }

        let proxy_process = match util::command::new_smol_command("wsl.exe")
            .arg("--distribution")
            .arg(&self.connection_options.distro_name)
            .arg("--")
            .arg("sh")
            .arg("-lc")
            .arg(&proxy_command)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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
                let wsl_src = path_to_wsl(&src_path);

                Self::wsl_command_impl(&options, "cp", &["-r", &wsl_src, &dest_path.to_string()])
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

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        let mut script = String::new();
        if let Some(working_dir) = working_dir {
            let working_dir = RemotePathBuf::new(working_dir.into(), PathStyle::Posix).to_string();
            if working_dir.starts_with("~/") {
                let working_dir = working_dir.trim_start_matches("~").trim_start_matches("/");
                write!(&mut script, "cd \"$HOME/{}\"; ", working_dir).unwrap();
            } else {
                write!(&mut script, "cd \"{}\"; ", working_dir).unwrap();
            }
        } else {
            write!(&mut script, "cd; ").unwrap();
        }

        for (k, v) in env.iter() {
            write!(&mut script, "{}='{}' ", k, v).unwrap();
        }

        if let Some(program) = program {
            script.push_str(&program);
            for arg in args {
                script.push(' ');
                script.push_str(arg);
            }
        } else {
            write!(&mut script, "exec {} -l", self.shell).unwrap();
        }

        let wsl_args = vec![
            "--distribution".to_string(),
            self.connection_options.distro_name.clone(),
            "--".to_string(),
            self.shell.clone(),
            "-c".to_string(),
            script,
        ];

        // todo(max): port forwarding
        if port_forward.is_some() {
            log::warn!("Port forwarding is not directly supported in WSL transport");
        }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_wsl() {
        let path = Path::new("C:\\Users\\User\\Documents\\file.txt");
        assert_eq!(path_to_wsl(&path), "/mnt/c/Users/User/Documents/file.txt");

        let path = Path::new("F:\\file.txt");
        assert_eq!(path_to_wsl(&path), "/mnt/f/file.txt");
    }
}

use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use fs::{CopyOptions, RealFs, copy_recursive};
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext, AsyncApp, Task};
use libc::SIGTERM;
use rpc::proto::Envelope;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::{
    command::{Stdio, new_command},
    paths::{PathStyle, RemotePathBuf},
};

use crate::{
    CommandTemplate, Interactive, RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct GuixMount {
    pub source: String,
    pub target: Option<String>,
}

impl From<settings::GuixMount> for GuixMount {
    fn from(value: settings::GuixMount) -> Self {
        Self {
            source: value.source,
            target: value.target,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct GuixShellOptions {
    pub allow_network: bool,
    pub nesting: bool,
    pub expose: Vec<GuixMount>,
    pub share: Vec<GuixMount>,
    pub extra_args: Vec<String>,
}

impl From<settings::GuixShellOptions> for GuixShellOptions {
    fn from(value: settings::GuixShellOptions) -> Self {
        Self {
            allow_network: value.allow_network,
            nesting: value.nesting,
            expose: value.expose.into_iter().map(Into::into).collect(),
            share: value.share.into_iter().map(Into::into).collect(),
            extra_args: value.extra_args,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct GuixContainerConnectionOptions {
    pub manifest_path: String,
    pub project_root: String,
    #[serde(default)]
    pub shell_options: GuixShellOptions,
}

pub(crate) struct GuixContainerConnection {
    proxy_process: parking_lot::Mutex<Option<u32>>,
    connection_options: GuixContainerConnectionOptions,
}

impl GuixContainerConnection {
    fn remote_server_exit_error(status: i32) -> anyhow::Error {
        match status {
            127 => anyhow!(
                "The Guix container command chain exited with status 127. This often means \
                 a required command was not available inside the container environment, \
                 for example zed-remote-server."
            ),
            126 => anyhow!(
                "zed-remote-server could not be executed inside the Guix container \
                 environment (exit status 126). Check that it is executable and available \
                 inside the container."
            ),
            _ => anyhow!("Remote server exited with status {status}"),
        }
    }

    pub async fn new(
        connection_options: GuixContainerConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        delegate.set_status(Some("Detecting Guix container environment"), cx);

        Ok(Self {
            proxy_process: parking_lot::Mutex::new(None),
            connection_options,
        })
    }

    fn manifest_path(&self) -> &Path {
        Path::new(&self.connection_options.manifest_path)
    }

    fn project_root(&self) -> PathBuf {
        PathBuf::from(&self.connection_options.project_root)
    }

    fn project_user_data_dir(&self) -> PathBuf {
        self.project_root().join(".zed").join("guix")
    }

    fn mount_arg(flag: &str, mount: &GuixMount) -> String {
        match &mount.target {
            Some(target) => format!("{flag}={}={}", mount.source, target),
            None => format!("{flag}={}", mount.source),
        }
    }

    fn required_mounts(&self) -> (Vec<String>, Vec<String>) {
        let mut expose = Vec::new();
        let mut share = vec![format!("--share={}", self.connection_options.project_root)];

        for mount in &self.connection_options.shell_options.expose {
            expose.push(Self::mount_arg("--expose", mount));
        }
        for mount in &self.connection_options.shell_options.share {
            share.push(Self::mount_arg("--share", mount));
        }

        (expose, share)
    }

    fn base_guix_args(&self) -> Vec<String> {
        let mut args = vec![
            "shell".to_string(),
            "--container".to_string(),
            "--no-cwd".to_string(),
            "-m".to_string(),
            self.manifest_path().display().to_string(),
        ];

        if self.connection_options.shell_options.allow_network {
            args.push("-N".to_string());
        }
        if self.connection_options.shell_options.nesting {
            args.push("--nesting".to_string());
        }

        let (expose, share) = self.required_mounts();
        args.extend(expose);
        args.extend(share);
        args.extend(self.connection_options.shell_options.extra_args.iter().cloned());
        args.push("--".to_string());

        args
    }

    fn current_dir_for_command(&self, working_dir: Option<String>) -> PathBuf {
        match working_dir {
            Some(path) => {
                let path = PathBuf::from(path);
                if path.is_absolute() {
                    path
                } else {
                    self.project_root().join(path)
                }
            }
            None => self.project_root(),
        }
    }

    fn command_inside_container(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut command = self.base_guix_args();

        if env.is_empty() {
            match program {
                Some(program) => {
                    command.push(program);
                    command.extend(args.iter().cloned());
                }
                None => {
                    command.push("bash".to_string());
                    command.push("-l".to_string());
                }
            }
        } else {
            command.push("env".to_string());
            for (key, value) in env {
                command.push(format!("{key}={value}"));
            }
            match program {
                Some(program) => {
                    command.push(program);
                    command.extend(args.iter().cloned());
                }
                None => {
                    command.push("bash".to_string());
                    command.push("-l".to_string());
                }
            }
        }

        command
    }

    fn kill_inner(&self) -> Result<()> {
        if let Some(pid) = self.proxy_process.lock().take() {
            let pid = i32::try_from(pid).context("proxy process id does not fit in i32")?;
            let result = unsafe { libc::kill(pid, SIGTERM) };
            if result == 0 {
                Ok(())
            } else {
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() == Some(libc::ESRCH) {
                    Ok(())
                } else {
                    Err(anyhow!("failed to kill proxy process: {error}"))
                }
            }
        } else {
            Ok(())
        }
    }
}

#[async_trait(?Send)]
impl RemoteConnection for GuixContainerConnection {
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
        if !self.has_been_killed() {
            if let Err(error) = self.kill_inner() {
                return Task::ready(Err(error));
            }
        }

        delegate.set_status(Some("Starting Guix container proxy"), cx);

        let mut env = HashMap::default();
        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Ok(value) = env::var(env_var) {
                env.insert(env_var.to_string(), value);
            }
        }

        let user_data_dir = self.project_user_data_dir();
        if let Err(error) = std::fs::create_dir_all(&user_data_dir) {
            return Task::ready(Err(anyhow!(
                "failed to create Guix remote user data directory {}: {error}",
                user_data_dir.display()
            )));
        }

        let mut args = vec![
            "--user-data-dir".to_string(),
            user_data_dir.display().to_string(),
            "proxy".to_string(),
            "--identifier".to_string(),
            unique_identifier,
        ];
        if reconnect {
            args.push("--reconnect".to_string());
        }

        let command_template = match self.build_command(
            Some("zed-remote-server".to_string()),
            &args,
            &env,
            Some(self.connection_options.project_root.clone()),
            None,
            Interactive::No,
        ) {
            Ok(command) => command,
            Err(error) => return Task::ready(Err(error)),
        };

        let mut command = new_command(&command_template.program);
        command
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(&command_template.args)
            .envs(&command_template.env);
        if let Some(path) = &command_template.cwd {
            command.current_dir(path);
        }

        let Ok(child) = command.spawn() else {
            return Task::ready(Err(anyhow!("failed to start guix remote server process")));
        };

        *self.proxy_process.lock() = Some(child.id());

        cx.spawn(async move |cx| {
            super::handle_rpc_messages_over_child_process_stdio(
                child,
                incoming_tx,
                outgoing_rx,
                connection_activity_tx,
                cx,
            )
            .await
            .and_then(|status| {
                if status != 0 {
                    return Err(Self::remote_server_exit_error(status));
                }
                Ok(0)
            })
        })
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        let dest_path = PathBuf::from(dest_path.to_string());
        let fs = RealFs::new(None, cx.background_executor().clone());
        cx.background_spawn(async move {
            copy_recursive(&fs, &src_path, &dest_path, CopyOptions::default())
                .await
                .with_context(|| {
                    format!(
                        "failed to copy uploaded directory {} to {}",
                        src_path.display(),
                        dest_path.display()
                    )
                })
        })
    }

    async fn kill(&self) -> Result<()> {
        self.kill_inner()
    }

    fn has_been_killed(&self) -> bool {
        self.proxy_process.lock().is_none()
    }

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
        _interactive: Interactive,
    ) -> Result<CommandTemplate> {
        if port_forward.is_some() {
            anyhow::bail!("Guix container transport does not support port forwarding commands");
        }

        Ok(CommandTemplate {
            program: "guix".to_string(),
            args: self.command_inside_container(program, args, env),
            env: HashMap::default(),
            cwd: Some(self.current_dir_for_command(working_dir)),
        })
    }

    fn build_forward_ports_command(
        &self,
        _forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        Err(anyhow!("Guix container transport does not support port forwarding"))
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::GuixContainer(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        PathStyle::Posix
    }

    fn shell(&self) -> String {
        "bash".to_string()
    }

    fn default_system_shell(&self) -> String {
        "bash".to_string()
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Interactive;

    fn test_connection(shell_options: GuixShellOptions) -> GuixContainerConnection {
        GuixContainerConnection {
            proxy_process: parking_lot::Mutex::new(None),
            connection_options: GuixContainerConnectionOptions {
                manifest_path: "/project/manifest.scm".to_string(),
                project_root: "/project".to_string(),
                shell_options,
            },
        }
    }

    #[test]
    fn test_build_command_uses_guix_shell_with_required_mounts() {
        let connection = test_connection(GuixShellOptions::default());
        let command = connection
            .build_command(
                Some("zed-remote-server".to_string()),
                &["proxy".to_string()],
                &HashMap::default(),
                Some("/project".to_string()),
                None,
                Interactive::No,
            )
            .unwrap();

        assert_eq!(command.program, "guix");
        assert_eq!(command.cwd, Some(PathBuf::from("/project")));
        assert_eq!(
            command.args,
            vec![
                "shell",
                "--container",
                "--no-cwd",
                "-m",
                "/project/manifest.scm",
                "--share=/project",
                "--",
                "zed-remote-server",
                "proxy",
            ]
        );
    }

    #[test]
    fn test_build_command_merges_network_mounts_and_env() {
        let connection = test_connection(GuixShellOptions {
            allow_network: true,
            nesting: true,
            expose: vec![GuixMount {
                source: "/nix/store".to_string(),
                target: Some("/gnu/store-alt".to_string()),
            }],
            share: vec![GuixMount {
                source: "/tmp/cache".to_string(),
                target: Some("/cache".to_string()),
            }],
            extra_args: vec!["--pure".to_string()],
        });
        let mut env = HashMap::default();
        env.insert("FOO".to_string(), "BAR".to_string());

        let command = connection
            .build_command(
                Some("env".to_string()),
                &["true".to_string()],
                &env,
                Some("src".to_string()),
                None,
                Interactive::No,
            )
            .unwrap();

        assert_eq!(command.program, "guix");
        assert_eq!(command.cwd, Some(PathBuf::from("/project/src")));
        assert_eq!(
            command.args,
            vec![
                "shell",
                "--container",
                "--no-cwd",
                "-m",
                "/project/manifest.scm",
                "-N",
                "--nesting",
                "--expose=/nix/store=/gnu/store-alt",
                "--share=/project",
                "--share=/tmp/cache=/cache",
                "--pure",
                "--",
                "env",
                "FOO=BAR",
                "env",
                "true",
            ]
        );
    }

    #[test]
    fn test_build_command_preserves_absolute_working_directory() {
        let connection = test_connection(GuixShellOptions::default());
        let command = connection
            .build_command(
                Some("pwd".to_string()),
                &[],
                &HashMap::default(),
                Some("/elsewhere".to_string()),
                None,
                Interactive::No,
            )
            .unwrap();

        assert_eq!(command.cwd, Some(PathBuf::from("/elsewhere")));
    }

    #[test]
    fn test_build_command_without_program_opens_bash_login_shell() {
        let connection = test_connection(GuixShellOptions::default());
        let command = connection
            .build_command(
                None,
                &[],
                &HashMap::default(),
                None,
                None,
                Interactive::No,
            )
            .unwrap();

        assert_eq!(
            command.args,
            vec![
                "shell",
                "--container",
                "--no-cwd",
                "-m",
                "/project/manifest.scm",
                "--share=/project",
                "--",
                "bash",
                "-l",
            ]
        );
    }

    #[test]
    fn test_remote_server_exit_error_for_missing_binary() {
        let error = GuixContainerConnection::remote_server_exit_error(127);
        let message = error.to_string();
        assert!(message.contains("command chain exited with status 127"));
        assert!(message.contains("for example zed-remote-server"));
    }
}

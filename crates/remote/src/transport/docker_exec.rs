use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use collections::HashMap;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
};
use util::{
    paths::{PathStyle, RemotePathBuf},
    rel_path::RelPath,
};

use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext, AsyncApp, SemanticVersion, Task};
use rpc::proto::Envelope;
use std::fmt::Write;

use crate::{
    RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions, RemotePlatform,
    remote_client::CommandTemplate,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct DockerExecConnectionOptions {
    pub name: String,
    // pub username: Option<String>,
    // pub port: Option<u16>,
    // pub password: Option<String>,
    // pub args: Option<Vec<String>>,
    // pub port_forwards: Option<Vec<SshPortForwardOption>>,

    // pub nickname: Option<String>,
    // pub upload_binary_over_ssh: bool,
}

pub(crate) struct DockerExecConnection {
    remote_binary_path: Option<Arc<RelPath>>,
    remote_platform: RemotePlatform,
}

impl DockerExecConnection {
    pub async fn new(
        options: DockerExecConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let mut this = Self {
            remote_binary_path: None,
            remote_platform: RemotePlatform {
                // TODO hard-coded
                os: "linux",
                arch: "aarch64",
            }, // TODO even needed?
        };
        this.remote_binary_path = Some(this.ensure_server_binary(&delegate, cx).await?);

        Ok(this)
    }

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn RemoteClientDelegate>,
        // release_channel: ReleaseChannel,
        // version: SemanticVersion,
        // commit: Option<AppCommitSha>,
        cx: &mut AsyncApp,
    ) -> Result<Arc<RelPath>> {
        let (release_channel, version, commit) = cx.update(|cx| {
            (
                ReleaseChannel::global(cx),
                AppVersion::global(cx),
                AppCommitSha::try_global(cx),
            )
        })?;
        let version_str = match release_channel {
            ReleaseChannel::Nightly => {
                let commit = commit.map(|s| s.full()).unwrap_or_default();
                format!("{}-{}", version, commit)
            }
            ReleaseChannel::Dev => "build".to_string(),
            _ => version.to_string(),
        };
        // let binary_name = format!(
        //     "zed-remote-server-{}-{}",
        //     release_channel.dev_name(),
        //     version_str
        // );
        let binary_name = "zed-remote-server";

        let dst_path =
            paths::remote_server_dir_relative().join(RelPath::unix(&binary_name).unwrap());

        // #[cfg(debug_assertions)]
        // if let Some(remote_server_path) =
        //     super::build_remote_server_from_source(&self.remote_platform, delegate.as_ref(), cx)
        //         .await?
        // {
        //     let tmp_path = paths::remote_server_dir_relative().join(
        //         RelPath::unix(&format!(
        //             "download-{}-{}",
        //             std::process::id(),
        //             remote_server_path.file_name().unwrap().to_string_lossy()
        //         ))
        //         .unwrap(),
        //     );
        //     self.upload_local_server_binary(&remote_server_path, &tmp_path, delegate, cx)
        //         .await?;
        //     self.extract_server_binary(&dst_path, &tmp_path, delegate, cx)
        //         .await?;
        //     return Ok(dst_path);
        // }

        // if self
        //     .socket
        //     .run_command(
        //         self.ssh_shell_kind,
        //         &dst_path.display(self.path_style()),
        //         &["version"],
        //         true,
        //     )
        //     .await
        //     .is_ok()
        // {
        //     return Ok(dst_path);
        // }

        // let wanted_version = cx.update(|cx| match release_channel {
        //     ReleaseChannel::Nightly => Ok(None),
        //     ReleaseChannel::Dev => {
        //         anyhow::bail!(
        //             "ZED_BUILD_REMOTE_SERVER is not set and no remote server exists at ({:?})",
        //             dst_path
        //         )
        //         // anyhow::Ok(Some(AppVersion::global(cx)))
        //     }
        //     _ => Ok(Some(AppVersion::global(cx))),
        // })??;
        // let url = delegate
        //     .get_download_url(self.remote_platform, release_channel, wanted_version, cx)
        //     .await?;

        // println!("Download url: {:?}", url);
        // println!("Dest path: {:?}", dst_path);

        // Let's keep it super simple and just do a docker-exec for this. First, let's figure out what it is?
        Ok(dst_path)
        // #[cfg(debug_assertions)]
        // if let Some(remote_server_path) =
        //     super::build_remote_server_from_source(&self.remote_platform, delegate.as_ref(), cx)
        //         .await?
        // {
        //     // TODO
        //     panic!("Not supporting build server from source yet");
        //     // let tmp_path = paths::remote_server_dir_relative().join(
        //     //     RelPath::unix(&format!(
        //     //         "download-{}-{}",
        //     //         std::process::id(),
        //     //         remote_server_path.file_name().unwrap().to_string_lossy()
        //     //     ))
        //     //     .unwrap(),
        //     // );
        //     // self.upload_local_server_binary(&remote_server_path, &tmp_path, delegate, cx)
        //     //     .await?;
        //     // self.extract_server_binary(&dst_path, &tmp_path, delegate, cx)
        //     //     .await?;
        //     // return Ok(dst_path);
        // }

        // if self
        //     .socket
        //     .run_command(
        //         self.ssh_shell_kind,
        //         &dst_path.display(self.path_style()),
        //         &["version"],
        //         true,
        //     )
        //     .await
        //     .is_ok()
        // {
        //     return Ok(dst_path);
        // }

        // let wanted_version = cx.update(|cx| match release_channel {
        //     ReleaseChannel::Nightly => Ok(None),
        //     ReleaseChannel::Dev => {
        //         anyhow::bail!(
        //             "ZED_BUILD_REMOTE_SERVER is not set and no remote server exists at ({:?})",
        //             dst_path
        //         )
        //     }
        //     _ => Ok(Some(AppVersion::global(cx))),
        // })??;

        // let tmp_path_gz = remote_server_dir_relative().join(
        //     RelPath::unix(&format!(
        //         "{}-download-{}.gz",
        //         binary_name,
        //         std::process::id()
        //     ))
        //     .unwrap(),
        // );
        // if !self.socket.connection_options.upload_binary_over_ssh
        //     && let Some(url) = delegate
        //         .get_download_url(self.ssh_platform, release_channel, wanted_version, cx)
        //         .await?
        // {
        //     match self
        //         .download_binary_on_server(&url, &tmp_path_gz, delegate, cx)
        //         .await
        //     {
        //         Ok(_) => {
        //             self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
        //                 .await
        //                 .context("extracting server binary")?;
        //             return Ok(dst_path);
        //         }
        //         Err(e) => {
        //             log::error!(
        //                 "Failed to download binary on server, attempting to download locally and then upload it the server: {e:#}",
        //             )
        //         }
        //     }
        // }

        // let src_path = delegate
        //     .download_server_binary_locally(self.ssh_platform, release_channel, wanted_version, cx)
        //     .await
        //     .context("downloading server binary locally")?;
        // self.upload_local_server_binary(&src_path, &tmp_path_gz, delegate, cx)
        //     .await
        //     .context("uploading server binary")?;
        // self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
        //     .await
        //     .context("extracting server binary")?;
        // Ok(dst_path)
    }
}

#[async_trait(?Send)]
impl RemoteConnection for DockerExecConnection {
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
        // let path = RelPath::new(Path::new(".zed_server/zed-remote-server"), PathStyle::Posix)?;
        // let remote_binary_path = Arc::new(RelPath::new(
        //     Path::new(".zed_server/zed-remote-server"),
        //     PathStyle::Posix,
        // ));

        let mut docker_args = vec!["exec", "-i", "fa75b942d27c"];
        // TODO not sure how you do this best in an exec context
        // for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
        //     if let Some(value) = std::env::var(env_var).ok() {
        //         docker_args.push(format!("{}='{}'", env_var, value));
        //     }
        // }
        let val = remote_binary_path.display(self.path_style()).into_owned();
        docker_args.push(&val);
        docker_args.push("proxy");
        docker_args.push("--identifier");
        docker_args.push(&unique_identifier);

        // if reconnect {
        //     proxy_args.push("--reconnect".to_owned());
        // }

        // let ssh_proxy_process = match self
        //     .socket
        //     .ssh_command(self.ssh_shell_kind, "env", &proxy_args, false)
        //     // IMPORTANT: we kill this process when we drop the task that uses it.
        //     .kill_on_drop(true)
        //     .spawn()
        // {
        //     Ok(process) => process,
        //     Err(error) => {
        //         return Task::ready(Err(anyhow!("failed to spawn remote server: {}", error)));
        //     }
        // };
        let mut command = util::command::new_smol_command("docker");
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // SO this isn't enough - you need to actually run the server binary with its unique identifier for this to work
            .args(docker_args); // TODO
        // command.args(["exec", /*"-it",*/ "fa75b942d27c", "pwd"]); // TODO - not sure this is really needed, it just sort of proves I can connect
        let child = command.spawn().unwrap(); // TODO

        // So the question is really, why doesn't this have a stdout /in handle?
        super::handle_rpc_messages_over_child_process_stdio(
            child,
            incoming_tx,
            outgoing_rx,
            connection_activity_tx,
            cx,
        )
        // cx.background_spawn(async move {
        //     let out = child.stdout.inspect(f)await;
        //     println!("Stdout: {:?}", out)
        //     match child.status().await {
        //         Ok(exit_status) => Ok(exit_status.code().unwrap_or(1)),
        //         Err(_e) => todo!("Handle unable to connect via docker exec"),
        //     }
        // })
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        let dest_path_str = dest_path.to_string();
        let src_path_display = src_path.display().to_string();

        let mut command = util::command::new_smol_command("docker");
        command.arg("cp");
        command.arg(src_path_display);
        command.arg(format!("{}:{}", "fa75b942d27c", dest_path_str));

        cx.background_spawn(async move {
            let output = command.output().await?;

            // TODO stderr mapping and such
            if output.status.success() {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to upload directory"))
            }
        })
    }

    async fn kill(&self) -> Result<()> {
        // Docker exec is not stateful
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        // Docker exec is not stateful
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
        use std::fmt::Write as _;

        let mut parsed_working_dir = None;

        let path_style = self.path_style();
        // let shell_kind = self.shell_kind();

        let mut exec = String::new();
        if let Some(working_dir) = working_dir {
            let working_dir = RemotePathBuf::new(working_dir, path_style).to_string();

            // shlex will wrap the command in single quotes (''), disabling ~ expansion,
            // replace with with something that works
            const TILDE_PREFIX: &'static str = "~/";
            if working_dir.starts_with(TILDE_PREFIX) {
                let working_dir = working_dir.trim_start_matches("~").trim_start_matches("/");
                parsed_working_dir = Some(format!("$HOME/{working_dir}"));
                // write!(
                //     exec,
                //     "cd \"$HOME/{working_dir}\" {} ",
                //     ssh_shell_kind.sequential_and_commands_separator()
                // )?;
            } else {
                parsed_working_dir = Some(working_dir);
                // write!(
                //     exec,
                //     "cd \"{working_dir}\" {} ",
                //     ssh_shell_kind.sequential_and_commands_separator()
                // )?;
            }
        }

        let mut env_str = String::new();

        for (k, v) in env.iter() {
            write!(env_str, "{}={}", k, v)?; // TODO shell escaping
        }
        write!(exec, "exec env ")?;

        let mut inner_program = Vec::new();

        if let Some(program) = program {
            inner_program.push(program);
            for arg in args {
                // let arg = ssh_shell_kind.try_quote(&arg).context("shell quoting")?;
                inner_program.push(arg.clone());
            }
        } else {
            inner_program.push("/bin/bash".to_string()); // TODO shell extraction
            inner_program.push("-l".to_string());
        };

        let mut the_args = vec!["exec".to_string()];

        if parsed_working_dir.is_some() {
            the_args.push("-w".to_string());
            the_args.push(parsed_working_dir.unwrap());
        }

        if env_str != "" {
            the_args.push("-e".to_string());
            the_args.push(env_str);
        }

        // The hard-coded part
        // TODO this basically assumes that we're using this case to open a terminal window in Zed
        // That seems like a safe assumption, but perhaps warrants a switch?
        the_args.push("-it".to_string());
        the_args.push("fa75b942d27c".to_string());

        the_args.append(&mut inner_program);

        print!("The Args: {:?}", the_args);
        Ok(CommandTemplate {
            program: "docker".to_string(),
            args: the_args,
            // Docker-exec pipes in environment via the "-e" argument
            env: Default::default(),
        })
    }

    fn build_forward_ports_command(
        &self,
        forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        todo!()
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        // RemoteConnectionOptions::DockerExec
        RemoteConnectionOptions::DockerExec(DockerExecConnectionOptions {
            name: "test".to_string(),
        })
    }

    fn path_style(&self) -> PathStyle {
        PathStyle::Posix // TODO inject
    }

    fn shell(&self) -> String {
        "/bin/sh".to_string() // TODO?
    }

    fn default_system_shell(&self) -> String {
        "/bin/sh".to_string() // TODO?
    }
}

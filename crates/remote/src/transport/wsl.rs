use std::{path::PathBuf, process::Stdio, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AsyncApp, Task};
use parking_lot::Mutex;
use rpc::proto::Envelope;
use smol::process::Child;
use util::paths::{PathStyle, RemotePathBuf};

use crate::{
    RemoteClientDelegate, RemotePlatform,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WslConnectionOptions {
    pub distro_name: String,
}

pub(crate) struct WslRemoteConnection {
    wsl_process: Mutex<Option<Child>>,
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
        let mut command = util::command::new_smol_command("wsl.exe");

        let wsl_process = command
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("--distribution")
            .arg(&connection_options.distro_name)
            .spawn()?;

        Ok(Self {
            wsl_process: Mutex::new(Some(wsl_process)),
            remote_binary_path: None,
            platform: RemotePlatform {
                os: "linux",
                arch: "todo",
            },
            shell: "sh".into(),
            connection_options,
        })
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
        todo!()
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        todo!()
    }

    async fn kill(&self) -> Result<()> {
        let Some(mut process) = self.wsl_process.lock().take() else {
            return Ok(());
        };
        process.kill().ok();
        process.status().await?;
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        self.wsl_process.lock().is_none()
    }

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        todo!()
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

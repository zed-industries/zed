use agent_client_protocol as acp;
use std::rc::Rc;
use std::{any::Any, path::Path};

use crate::{AgentServer, AgentServerDelegate, load_proxy_env};
use acp_thread::AgentConnection;
use anyhow::{Context as _, Result};
use gpui::{App, SharedString, Task};

#[derive(Clone)]
pub struct Codex;

impl AgentServer for Codex {
    fn telemetry_id(&self) -> &'static str {
        "codex"
    }

    fn name(&self) -> SharedString {
        "Codex".into()
    }

    fn logo(&self) -> ui::IconName {
        // No dedicated Codex icon yet; use the generic AI icon.
        ui::IconName::Ai
    }

    fn connect(
        &self,
        root_dir: Option<&Path>,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>> {
        let name = self.name();
        let root_dir = root_dir.map(|root_dir| root_dir.to_string_lossy().into_owned());
        let is_remote = delegate.project.read(cx).is_via_remote_server();
        let store = delegate.store.downgrade();
        let extra_env = load_proxy_env(cx);
        // No modes for Codex (yet).
        let default_mode = self.default_mode(cx);

        cx.spawn(async move |cx| {
            // Look up the external agent registered under the "codex" name.
            // The AgentServerStore is responsible for:
            // - Downloading the correct GitHub release tar.gz for the OS/arch
            // - Extracting the binary
            // - Returning an AgentServerCommand to launch the binary
            // - Always reporting "no updates" for now
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&"codex".into())
                        .context("Codex is not registered")?;
                    anyhow::Ok(agent.get_command(
                        root_dir.as_deref(),
                        extra_env,
                        delegate.status_tx,
                        // For now, Codex should report that there are no updates.
                        // The LocalCodex implementation in AgentServerStore should not send any updates.
                        delegate.new_version_available,
                        &mut cx.to_async(),
                    ))
                })??
                .await?;

            let connection = crate::acp::connect(
                name,
                command,
                root_dir.as_ref(),
                default_mode,
                is_remote,
                cx,
            )
            .await?;
            Ok((connection, login))
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

use std::rc::Rc;
use std::{any::Any, path::Path};

use crate::{AgentServer, AgentServerDelegate, load_proxy_env};
use acp_thread::AgentConnection;
use anyhow::{Context as _, Result};
use gpui::{App, SharedString, Task};
use project::agent_server_store::CODEX_NAME;

#[derive(Clone)]
pub struct Codex;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    crate::common_e2e_tests!(async |_, _, _| Codex, allow_option_id = "proceed_once");
}

impl AgentServer for Codex {
    fn telemetry_id(&self) -> &'static str {
        "codex"
    }

    fn name(&self) -> SharedString {
        "Codex".into()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
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
        let default_mode = self.default_mode(cx);

        cx.spawn(async move |cx| {
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&CODEX_NAME.into())
                        .context("Codex is not registered")?;
                    anyhow::Ok(agent.get_command(
                        root_dir.as_deref(),
                        extra_env,
                        delegate.status_tx,
                        // For now, report that there are no updates.
                        // (A future PR will use the GitHub Releases API to fetch them.)
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

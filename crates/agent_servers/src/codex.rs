use std::rc::Rc;
use std::sync::Arc;
use std::{any::Any, path::Path};

use acp_thread::AgentConnection;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use fs::Fs;
use gpui::{App, AppContext as _, SharedString, Task};
use project::agent_server_store::{AllAgentServersSettings, CODEX_NAME};
use settings::{SettingsStore, update_settings_file};

use crate::{AgentServer, AgentServerDelegate, load_proxy_env};

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

    fn default_mode(&self, cx: &mut App) -> Option<acp::SessionModeId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).codex.clone()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_mode.clone().map(|m| acp::SessionModeId(m.into())))
    }

    fn set_default_mode(&self, mode_id: Option<acp::SessionModeId>, fs: Arc<dyn Fs>, cx: &mut App) {
        update_settings_file(fs, cx, |settings, _| {
            settings
                .agent_servers
                .get_or_insert_default()
                .codex
                .get_or_insert_default()
                .default_mode = mode_id.map(|m| m.to_string())
        });
    }

    fn connect(
        &self,
        root_dir: Option<&Path>,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>> {
        let name = self.name();
        let telemetry_id = self.telemetry_id();
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
                        delegate.new_version_available,
                        &mut cx.to_async(),
                    ))
                })??
                .await?;

            let connection = crate::acp::connect(
                name,
                telemetry_id,
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

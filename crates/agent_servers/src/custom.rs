use crate::{AgentServerDelegate, load_proxy_env};
use acp_thread::AgentConnection;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use fs::Fs;
use gpui::{App, AppContext as _, SharedString, Task};
use project::agent_server_store::{AllAgentServersSettings, ExternalAgentServerName};
use settings::{SettingsStore, update_settings_file};
use std::{path::Path, rc::Rc, sync::Arc};
use ui::IconName;

/// A generic agent server implementation for custom user-defined agents
pub struct CustomAgentServer {
    name: SharedString,
}

impl CustomAgentServer {
    pub fn new(name: SharedString) -> Self {
        Self { name }
    }
}

impl crate::AgentServer for CustomAgentServer {
    fn telemetry_id(&self) -> &'static str {
        "custom"
    }

    fn name(&self) -> SharedString {
        self.name.clone()
    }

    fn logo(&self) -> IconName {
        IconName::Terminal
    }

    fn default_mode(&self, cx: &mut App) -> Option<acp::SessionModeId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .custom
                .get(&self.name())
                .cloned()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_mode.clone().map(|m| acp::SessionModeId(m.into())))
    }

    fn set_default_mode(&self, mode_id: Option<acp::SessionModeId>, fs: Arc<dyn Fs>, cx: &mut App) {
        let name = self.name();
        update_settings_file::<AllAgentServersSettings>(fs, cx, move |settings, _| {
            settings.custom.get_mut(&name).unwrap().default_mode = mode_id.map(|m| m.to_string())
        });
    }

    fn connect(
        &self,
        root_dir: Option<&Path>,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>> {
        let name = self.name();
        let root_dir = root_dir.map(|root_dir| root_dir.to_string_lossy().to_string());
        let is_remote = delegate.project.read(cx).is_via_remote_server();
        let default_mode = self.default_mode(cx);
        let store = delegate.store.downgrade();
        let extra_env = load_proxy_env(cx);

        cx.spawn(async move |cx| {
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&ExternalAgentServerName(name.clone()))
                        .with_context(|| {
                            format!("Custom agent server `{}` is not registered", name)
                        })?;
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

    fn into_any(self: Rc<Self>) -> Rc<dyn std::any::Any> {
        self
    }
}

use settings::SettingsStore;
use std::any::Any;
use std::path::Path;
use std::rc::Rc;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, SharedString, Task};
use project::agent_server_store::{AllAgentServersSettings, GITHUB_COPILOT_NAME};

use crate::{AgentServer, AgentServerDelegate, load_proxy_env};
use acp_thread::AgentConnection;

#[derive(Clone)]
pub struct Copilot;

impl AgentServer for Copilot {
    fn name(&self) -> SharedString {
        "GitHub Copilot".into()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::Copilot
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
        let default_model = self.default_model(cx);
        let default_config_options = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .copilot
                .as_ref()
                .map(|s| s.default_config_options.clone())
                .unwrap_or_default()
        });

        cx.spawn(async move |cx| {
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&GITHUB_COPILOT_NAME.into())
                        .context("GitHub Copilot is not registered")?;
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
                default_model,
                default_config_options,
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

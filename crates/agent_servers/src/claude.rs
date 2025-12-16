use agent_client_protocol as acp;
use fs::Fs;
use settings::{SettingsStore, update_settings_file};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::{any::Any, path::PathBuf};

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, SharedString, Task};
use project::agent_server_store::{AllAgentServersSettings, CLAUDE_CODE_NAME};

use crate::{AgentServer, AgentServerDelegate, load_proxy_env};
use acp_thread::AgentConnection;

#[derive(Clone)]
pub struct ClaudeCode;

pub struct AgentServerLoginCommand {
    pub path: PathBuf,
    pub arguments: Vec<String>,
}

impl AgentServer for ClaudeCode {
    fn name(&self) -> SharedString {
        "Claude Code".into()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiClaude
    }

    fn default_mode(&self, cx: &mut App) -> Option<acp::SessionModeId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).claude.clone()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_mode.clone().map(acp::SessionModeId::new))
    }

    fn set_default_mode(&self, mode_id: Option<acp::SessionModeId>, fs: Arc<dyn Fs>, cx: &mut App) {
        update_settings_file(fs, cx, |settings, _| {
            settings
                .agent_servers
                .get_or_insert_default()
                .claude
                .get_or_insert_default()
                .default_mode = mode_id.map(|m| m.to_string())
        });
    }

    fn default_model(&self, cx: &mut App) -> Option<acp::ModelId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).claude.clone()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_model.clone().map(acp::ModelId::new))
    }

    fn set_default_model(&self, model_id: Option<acp::ModelId>, fs: Arc<dyn Fs>, cx: &mut App) {
        update_settings_file(fs, cx, |settings, _| {
            settings
                .agent_servers
                .get_or_insert_default()
                .claude
                .get_or_insert_default()
                .default_model = model_id.map(|m| m.to_string())
        });
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

        cx.spawn(async move |cx| {
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&CLAUDE_CODE_NAME.into())
                        .context("Claude Code is not registered")?;
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

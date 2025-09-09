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

use crate::{AgentServer, AgentServerDelegate};
use acp_thread::AgentConnection;

#[derive(Clone)]
pub struct ClaudeCode;

pub struct AgentServerLoginCommand {
    pub path: PathBuf,
    pub arguments: Vec<String>,
}

impl AgentServer for ClaudeCode {
    fn telemetry_id(&self) -> &'static str {
        "claude-code"
    }

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
            .and_then(|s| s.default_mode.clone().map(|m| acp::SessionModeId(m.into())))
    }

    fn set_default_mode(&self, mode_id: Option<acp::SessionModeId>, fs: Arc<dyn Fs>, cx: &mut App) {
        update_settings_file::<AllAgentServersSettings>(fs, cx, |settings, _| {
            settings.claude.get_or_insert_default().default_mode = mode_id.map(|m| m.to_string())
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
        let store = delegate.store.downgrade();
        let default_mode = self.default_mode(cx);

        cx.spawn(async move |cx| {
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&CLAUDE_CODE_NAME.into())
                        .context("Claude Code is not registered")?;
                    anyhow::Ok(agent.get_command(
                        root_dir.as_deref(),
                        Default::default(),
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

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

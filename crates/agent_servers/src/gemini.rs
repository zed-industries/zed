use std::rc::Rc;
use std::{any::Any, path::Path};

use crate::{AgentServer, AgentServerDelegate};
use acp_thread::AgentConnection;
use anyhow::{Context as _, Result};
use client::ProxySettings;
use collections::HashMap;
use gpui::{App, AppContext, SharedString, Task};
use language_models::provider::google::GoogleLanguageModelProvider;
use project::agent_server_store::GEMINI_NAME;
use settings::SettingsStore;

#[derive(Clone)]
pub struct Gemini;

impl AgentServer for Gemini {
    fn telemetry_id(&self) -> &'static str {
        "gemini-cli"
    }

    fn name(&self) -> SharedString {
        "Gemini CLI".into()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiGemini
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
        let proxy_url = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<ProxySettings>(None).proxy.clone()
        });
        let default_mode = self.default_mode(cx);

        cx.spawn(async move |cx| {
            let mut extra_env = HashMap::default();
            if let Some(api_key) = cx.update(GoogleLanguageModelProvider::api_key)?.await.ok() {
                extra_env.insert("GEMINI_API_KEY".into(), api_key.key);
            }
            let (mut command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&GEMINI_NAME.into())
                        .context("Gemini CLI is not registered")?;
                    anyhow::Ok(agent.get_command(
                        root_dir.as_deref(),
                        extra_env,
                        delegate.status_tx,
                        delegate.new_version_available,
                        &mut cx.to_async(),
                    ))
                })??
                .await?;

            // Add proxy flag if proxy settings are configured in Zed and not in the args
            if let Some(proxy_url_value) = &proxy_url
                && !command.args.iter().any(|arg| arg.contains("--proxy"))
            {
                command.args.push("--proxy".into());
                command.args.push(proxy_url_value.clone());
            }

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

#[cfg(test)]
pub(crate) mod tests {
    use project::agent_server_store::AgentServerCommand;

    use super::*;
    use std::path::Path;

    crate::common_e2e_tests!(async |_, _, _| Gemini, allow_option_id = "proceed_once");

    pub fn local_command() -> AgentServerCommand {
        let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../gemini-cli/packages/cli")
            .to_string_lossy()
            .to_string();

        AgentServerCommand {
            path: "node".into(),
            args: vec![cli_path],
            env: None,
        }
    }
}

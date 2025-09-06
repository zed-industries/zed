use std::rc::Rc;
use std::{any::Any, path::Path};

use crate::acp::AcpConnection;
use crate::{AgentServer, AgentServerDelegate, AgentServerLoginCommand};
use acp_thread::{AgentConnection, LoadError};
use anyhow::Result;
use collections::HashMap;
use gpui::{App, AppContext as _, SharedString, Task};
use language_models::provider::google::GoogleLanguageModelProvider;
use settings::SettingsStore;

#[derive(Clone)]
pub struct Gemini;

const ACP_ARG: &str = "--experimental-acp";

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
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let Some(agent) = delegate
            .store
            .read(cx)
            .get_external_agent(&project::agent_server_store::gemini())
        else {
            return Task::ready(Err(anyhow::anyhow!("Gemini CLI is not registered")));
        };
        let name = self.name();
        let root_dir = root_dir.map(|root_dir| root_dir.to_string_lossy().to_string());
        let is_remote = delegate.project.read(cx).is_via_remote_server();

        cx.spawn(async move |cx| {
            let mut extra_env = HashMap::default();
            if let Some(api_key) = cx.update(GoogleLanguageModelProvider::api_key)?.await.ok() {
                extra_env.insert("GEMINI_API_KEY".into(), api_key.key);
            }
            let (command, root_dir) = agent
                .get_command(root_dir.as_deref(), extra_env, cx)
                .await?;
            crate::acp::connect(name, command, root_dir.as_ref(), is_remote, cx).await
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

impl Gemini {
    const PACKAGE_NAME: &str = "@google/gemini-cli";

    const MINIMUM_VERSION: &str = "0.2.1";

    const BINARY_NAME: &str = "gemini";

    pub fn login_command(
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<AgentServerLoginCommand>> {
        // FIXME
        Task::ready(Ok(AgentServerLoginCommand {
            path: "gemini".into(),
            arguments: vec![],
        }))
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

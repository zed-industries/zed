use language_models::provider::anthropic::AnthropicLanguageModelProvider;
use settings::SettingsStore;
use std::path::Path;
use std::rc::Rc;
use std::{any::Any, path::PathBuf};

use anyhow::Result;
use gpui::{App, AppContext as _, SharedString, Task};

use crate::{AgentServer, AgentServerDelegate, AllAgentServersSettings};
use acp_thread::AgentConnection;

#[derive(Clone)]
pub struct ClaudeCode;

pub struct ClaudeCodeLoginCommand {
    pub path: PathBuf,
    pub arguments: Vec<String>,
}

impl ClaudeCode {
    const BINARY_NAME: &'static str = "claude-code-acp";
    const PACKAGE_NAME: &'static str = "@zed-industries/claude-code-acp";

    pub fn login_command(
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<ClaudeCodeLoginCommand>> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).claude.clone()
        });

        cx.spawn(async move |cx| {
            let mut command = if let Some(settings) = settings {
                settings.command
            } else {
                cx.update(|cx| {
                    delegate.get_or_npm_install_builtin_agent(
                        Self::BINARY_NAME.into(),
                        Self::PACKAGE_NAME.into(),
                        "node_modules/@anthropic-ai/claude-code/cli.js".into(),
                        true,
                        None,
                        cx,
                    )
                })?
                .await?
            };
            command.args.push("/login".into());

            Ok(ClaudeCodeLoginCommand {
                path: command.path,
                arguments: command.args,
            })
        })
    }
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

    fn connect(
        &self,
        root_dir: &Path,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let root_dir = root_dir.to_path_buf();
        let server_name = self.name();
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).claude.clone()
        });

        cx.spawn(async move |cx| {
            let mut command = if let Some(settings) = settings {
                settings.command
            } else {
                cx.update(|cx| {
                    delegate.get_or_npm_install_builtin_agent(
                        Self::BINARY_NAME.into(),
                        Self::PACKAGE_NAME.into(),
                        format!("node_modules/{}/dist/index.js", Self::PACKAGE_NAME).into(),
                        true,
                        None,
                        cx,
                    )
                })?
                .await?
            };

            if let Some(api_key) = cx
                .update(AnthropicLanguageModelProvider::api_key)?
                .await
                .ok()
            {
                command
                    .env
                    .get_or_insert_default()
                    .insert("ANTHROPIC_API_KEY".to_owned(), api_key.key);
            }

            crate::acp::connect(server_name, command.clone(), &root_dir, cx).await
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

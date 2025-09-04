use settings::SettingsStore;
use std::path::Path;
use std::rc::Rc;
use std::{any::Any, path::PathBuf};

use anyhow::{Result, bail};
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
        let custom_command = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .get("claude")
                .cloned()
        });

        cx.spawn(async move |cx| {
            let mut command = if custom_command.is_some() {
                bail!("Cannot construct login command because a custom command was specified for claude-code-acp in settings")
            } else {
                cx.update(|cx| {
                    delegate.get_or_npm_install_builtin_agent(
                        Self::BINARY_NAME.into(),
                        Self::PACKAGE_NAME.into(),
                        "node_modules/@anthropic-ai/claude-code/cli.js".into(),
                        false,
                        Some("0.2.5".parse().unwrap()),
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
        let fs = delegate.project().read(cx).fs().clone();
        let server_name = self.name();
        let custom_command = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .get("claude")
                .cloned()
        });

        cx.spawn(async move |cx| {
            let mut command = if let Some(custom_command) = custom_command {
                custom_command
            } else {
                cx.update(|cx| {
                    delegate.get_or_npm_install_builtin_agent(
                        Self::BINARY_NAME.into(),
                        Self::PACKAGE_NAME.into(),
                        format!("node_modules/{}/dist/index.js", Self::PACKAGE_NAME).into(),
                        false,
                        None,
                        cx,
                    )
                })?
                .await?
            };

            command
                .env
                .get_or_insert_default()
                .insert("ANTHROPIC_API_KEY".to_owned(), "".to_owned());

            let root_dir_exists = fs.is_dir(&root_dir).await;
            anyhow::ensure!(
                root_dir_exists,
                "Session root {} does not exist or is not a directory",
                root_dir.to_string_lossy()
            );

            crate::acp::connect(server_name, command.clone(), &root_dir, cx).await
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

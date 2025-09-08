use std::rc::Rc;
use std::{any::Any, path::Path};

use crate::acp::AcpConnection;
use crate::{AgentServer, AgentServerDelegate};
use acp_thread::{AgentConnection, LoadError};
use anyhow::Result;
use gpui::{App, AppContext as _, SharedString, Task};
use language_models::provider::google::GoogleLanguageModelProvider;
use settings::SettingsStore;

use crate::AllAgentServersSettings;

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
        root_dir: &Path,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let root_dir = root_dir.to_path_buf();
        let fs = delegate.project().read(cx).fs().clone();
        let server_name = self.name();
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).gemini.clone()
        });
        let project = delegate.project().clone();

        cx.spawn(async move |cx| {
            let ignore_system_version = settings
                .as_ref()
                .and_then(|settings| settings.ignore_system_version)
                .unwrap_or(true);
            let mut project_env = project
                .update(cx, |project, cx| {
                    project.directory_environment(root_dir.as_path().into(), cx)
                })?
                .await
                .unwrap_or_default();
            let mut command = if let Some(settings) = settings
                && let Some(command) = settings.custom_command()
            {
                command
            } else {
                cx.update(|cx| {
                    delegate.get_or_npm_install_builtin_agent(
                        Self::BINARY_NAME.into(),
                        Self::PACKAGE_NAME.into(),
                        format!("node_modules/{}/dist/index.js", Self::PACKAGE_NAME).into(),
                        ignore_system_version,
                        Some(Self::MINIMUM_VERSION.parse().unwrap()),
                        cx,
                    )
                })?
                .await?
            };
            if !command.args.contains(&ACP_ARG.into()) {
                command.args.push(ACP_ARG.into());
            }
            if let Some(api_key) = cx.update(GoogleLanguageModelProvider::api_key)?.await.ok() {
                project_env
                    .insert("GEMINI_API_KEY".to_owned(), api_key.key);
            }
            project_env.extend(command.env.take().unwrap_or_default());
            command.env = Some(project_env);

            let root_dir_exists = fs.is_dir(&root_dir).await;
            anyhow::ensure!(
                root_dir_exists,
                "Session root {} does not exist or is not a directory",
                root_dir.to_string_lossy()
            );

            let result = crate::acp::connect(server_name, command.clone(), &root_dir, cx).await;
            match &result {
                Ok(connection) => {
                    if let Some(connection) = connection.clone().downcast::<AcpConnection>()
                        && !connection.prompt_capabilities().image
                    {
                        let version_output = util::command::new_smol_command(&command.path)
                            .args(command.args.iter())
                            .arg("--version")
                            .kill_on_drop(true)
                            .output()
                            .await;
                        let current_version =
                            String::from_utf8(version_output?.stdout)?.trim().to_owned();

                        log::error!("connected to gemini, but missing prompt_capabilities.image (version is {current_version})");
                        return Err(LoadError::Unsupported {
                            current_version: current_version.into(),
                            command: (command.path.to_string_lossy().to_string() + " " + &command.args.join(" ")).into(),
                            minimum_version: Self::MINIMUM_VERSION.into(),
                        }
                        .into());
                    }
                }
                Err(e) => {
                    let version_fut = util::command::new_smol_command(&command.path)
                        .args(command.args.iter())
                        .arg("--version")
                        .kill_on_drop(true)
                        .output();

                    let help_fut = util::command::new_smol_command(&command.path)
                        .args(command.args.iter())
                        .arg("--help")
                        .kill_on_drop(true)
                        .output();

                    let (version_output, help_output) =
                        futures::future::join(version_fut, help_fut).await;
                    let Some(version_output) = version_output.ok().and_then(|output| String::from_utf8(output.stdout).ok()) else {
                        return result;
                    };
                    let Some((help_stdout, help_stderr)) = help_output.ok().and_then(|output| String::from_utf8(output.stdout).ok().zip(String::from_utf8(output.stderr).ok())) else  {
                        return result;
                    };

                    let current_version = version_output.trim().to_string();
                    let supported = help_stdout.contains(ACP_ARG) || current_version.parse::<semver::Version>().is_ok_and(|version| version >= Self::MINIMUM_VERSION.parse::<semver::Version>().unwrap());

                    log::error!("failed to create ACP connection to gemini (version is {current_version}, supported: {supported}): {e}");
                    log::debug!("gemini --help stdout: {help_stdout:?}");
                    log::debug!("gemini --help stderr: {help_stderr:?}");
                    if !supported {
                        return Err(LoadError::Unsupported {
                            current_version: current_version.into(),
                            command: (command.path.to_string_lossy().to_string() + " " + &command.args.join(" ")).into(),
                            minimum_version: Self::MINIMUM_VERSION.into(),
                        }
                        .into());
                    }
                }
            }
            result
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
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
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

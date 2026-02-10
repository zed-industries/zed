use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    path::{Path, PathBuf},
};

use node_runtime::NodeRuntime;
use serde::Deserialize;
use settings::DevContainerConnection;
use smol::{fs, process::Command};
use util::rel_path::RelPath;
use workspace::Workspace;

use crate::{DevContainerContext, DevContainerFeature, DevContainerTemplate};

/// Represents a discovered devcontainer configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevContainerConfig {
    /// Display name for the configuration (subfolder name or "default")
    pub name: String,
    /// Relative path to the devcontainer.json file from the project root
    pub config_path: PathBuf,
}

impl DevContainerConfig {
    pub fn default_config() -> Self {
        Self {
            name: "default".to_string(),
            config_path: PathBuf::from(".devcontainer/devcontainer.json"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerUp {
    _outcome: String,
    container_id: String,
    remote_user: String,
    remote_workspace_folder: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainerApply {
    pub(crate) files: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainerConfiguration {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DevContainerConfigurationOutput {
    configuration: DevContainerConfiguration,
}

pub(crate) struct DevContainerCli {
    pub path: PathBuf,
    node_runtime_path: Option<PathBuf>,
}

impl DevContainerCli {
    fn command(&self, use_podman: bool) -> Command {
        let mut command = if let Some(node_runtime_path) = &self.node_runtime_path {
            let mut command = util::command::new_smol_command(
                node_runtime_path.as_os_str().display().to_string(),
            );
            command.arg(self.path.display().to_string());
            command
        } else {
            util::command::new_smol_command(self.path.display().to_string())
        };

        if use_podman {
            command.arg("--docker-path");
            command.arg("podman");
        }
        command
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevContainerError {
    DockerNotAvailable,
    DevContainerCliNotAvailable,
    DevContainerTemplateApplyFailed(String),
    DevContainerUpFailed(String),
    DevContainerNotFound,
    DevContainerParseFailed,
    NodeRuntimeNotAvailable,
    NotInValidProject,
}

impl Display for DevContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DevContainerError::DockerNotAvailable =>
                    "docker CLI not found on $PATH".to_string(),
                DevContainerError::DevContainerCliNotAvailable =>
                    "devcontainer CLI not found on path".to_string(),
                DevContainerError::DevContainerUpFailed(_) => {
                    "DevContainer creation failed".to_string()
                }
                DevContainerError::DevContainerTemplateApplyFailed(_) => {
                    "DevContainer template apply failed".to_string()
                }
                DevContainerError::DevContainerNotFound =>
                    "No valid dev container definition found in project".to_string(),
                DevContainerError::DevContainerParseFailed =>
                    "Failed to parse file .devcontainer/devcontainer.json".to_string(),
                DevContainerError::NodeRuntimeNotAvailable =>
                    "Cannot find a valid node runtime".to_string(),
                DevContainerError::NotInValidProject => "Not within a valid project".to_string(),
            }
        )
    }
}

/// Finds all available devcontainer configurations in the project.
///
/// This function scans for:
/// 1. `.devcontainer/devcontainer.json` (the default location)
/// 2. `.devcontainer/<subfolder>/devcontainer.json` (named configurations)
///
/// Returns a list of found configurations, or an empty list if none are found.
pub fn find_devcontainer_configs(workspace: &Workspace, cx: &gpui::App) -> Vec<DevContainerConfig> {
    let project = workspace.project().read(cx);

    let worktree = project
        .visible_worktrees(cx)
        .find_map(|tree| tree.read(cx).root_entry()?.is_dir().then_some(tree));

    let Some(worktree) = worktree else {
        log::debug!("find_devcontainer_configs: No worktree found");
        return Vec::new();
    };

    let worktree = worktree.read(cx);
    let mut configs = Vec::new();

    let devcontainer_path = RelPath::unix(".devcontainer").expect("valid path");

    let Some(devcontainer_entry) = worktree.entry_for_path(devcontainer_path) else {
        log::debug!("find_devcontainer_configs: .devcontainer directory not found in worktree");
        return Vec::new();
    };

    if !devcontainer_entry.is_dir() {
        log::debug!("find_devcontainer_configs: .devcontainer is not a directory");
        return Vec::new();
    }

    log::debug!("find_devcontainer_configs: Scanning .devcontainer directory");
    let devcontainer_json_path =
        RelPath::unix(".devcontainer/devcontainer.json").expect("valid path");
    for entry in worktree.child_entries(devcontainer_path) {
        log::debug!(
            "find_devcontainer_configs: Found entry: {:?}, is_file: {}, is_dir: {}",
            entry.path.as_unix_str(),
            entry.is_file(),
            entry.is_dir()
        );

        if entry.is_file() && entry.path.as_ref() == devcontainer_json_path {
            log::debug!("find_devcontainer_configs: Found default devcontainer.json");
            configs.push(DevContainerConfig::default_config());
        } else if entry.is_dir() {
            let subfolder_name = entry
                .path
                .file_name()
                .map(|n| n.to_string())
                .unwrap_or_default();

            let config_json_path = format!("{}/devcontainer.json", entry.path.as_unix_str());
            if let Ok(rel_config_path) = RelPath::unix(&config_json_path) {
                if worktree.entry_for_path(rel_config_path).is_some() {
                    log::debug!(
                        "find_devcontainer_configs: Found config in subfolder: {}",
                        subfolder_name
                    );
                    configs.push(DevContainerConfig {
                        name: subfolder_name,
                        config_path: PathBuf::from(&config_json_path),
                    });
                } else {
                    log::debug!(
                        "find_devcontainer_configs: Subfolder {} has no devcontainer.json",
                        subfolder_name
                    );
                }
            }
        }
    }

    log::info!(
        "find_devcontainer_configs: Found {} configurations",
        configs.len()
    );

    configs.sort_by(|a, b| {
        if a.name == "default" {
            std::cmp::Ordering::Less
        } else if b.name == "default" {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    configs
}

pub async fn start_dev_container_with_config(
    context: DevContainerContext,
    config: Option<DevContainerConfig>,
) -> Result<(DevContainerConnection, String), DevContainerError> {
    check_for_docker(context.use_podman).await?;
    let cli = ensure_devcontainer_cli(&context.node_runtime).await?;
    let config_path = config.map(|c| context.project_directory.join(&c.config_path));

    match devcontainer_up(&context, &cli, config_path.as_deref()).await {
        Ok(DevContainerUp {
            container_id,
            remote_workspace_folder,
            remote_user,
            ..
        }) => {
            let project_name =
                match read_devcontainer_configuration(&context, &cli, config_path.as_deref()).await
                {
                    Ok(DevContainerConfigurationOutput {
                        configuration:
                            DevContainerConfiguration {
                                name: Some(project_name),
                            },
                    }) => project_name,
                    _ => get_backup_project_name(&remote_workspace_folder, &container_id),
                };

            let connection = DevContainerConnection {
                name: project_name,
                container_id,
                use_podman: context.use_podman,
                remote_user,
            };

            Ok((connection, remote_workspace_folder))
        }
        Err(err) => {
            let message = format!("Failed with nested error: {}", err);
            Err(DevContainerError::DevContainerUpFailed(message))
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn dev_container_cli() -> String {
    "devcontainer".to_string()
}

#[cfg(target_os = "windows")]
fn dev_container_cli() -> String {
    "devcontainer.cmd".to_string()
}

fn dev_container_script() -> String {
    "devcontainer.js".to_string()
}

async fn check_for_docker(use_podman: bool) -> Result<(), DevContainerError> {
    let mut command = if use_podman {
        util::command::new_smol_command("podman")
    } else {
        util::command::new_smol_command("docker")
    };
    command.arg("--version");

    match command.output().await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Unable to find docker in $PATH: {:?}", e);
            Err(DevContainerError::DockerNotAvailable)
        }
    }
}

pub(crate) async fn ensure_devcontainer_cli(
    node_runtime: &NodeRuntime,
) -> Result<DevContainerCli, DevContainerError> {
    let mut command = util::command::new_smol_command(&dev_container_cli());
    command.arg("--version");

    if let Err(e) = command.output().await {
        log::error!(
            "Unable to find devcontainer CLI in $PATH. Checking for a zed installed version. Error: {:?}",
            e
        );

        let Ok(node_runtime_path) = node_runtime.binary_path().await else {
            return Err(DevContainerError::NodeRuntimeNotAvailable);
        };

        let datadir_cli_path = paths::devcontainer_dir()
            .join("node_modules")
            .join("@devcontainers")
            .join("cli")
            .join(&dev_container_script());

        log::debug!(
            "devcontainer not found in path, using local location: ${}",
            datadir_cli_path.display()
        );

        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(datadir_cli_path.display().to_string());
        command.arg("--version");

        match command.output().await {
            Err(e) => log::error!(
                "Unable to find devcontainer CLI in Data dir. Will try to install. Error: {:?}",
                e
            ),
            Ok(output) => {
                if output.status.success() {
                    log::info!("Found devcontainer CLI in Data dir");
                    return Ok(DevContainerCli {
                        path: datadir_cli_path.clone(),
                        node_runtime_path: Some(node_runtime_path.clone()),
                    });
                } else {
                    log::error!(
                        "Could not run devcontainer CLI from data_dir. Will try once more to install. Output: {:?}",
                        output
                    );
                }
            }
        }

        if let Err(e) = fs::create_dir_all(paths::devcontainer_dir()).await {
            log::error!("Unable to create devcontainer directory. Error: {:?}", e);
            return Err(DevContainerError::DevContainerCliNotAvailable);
        }

        if let Err(e) = node_runtime
            .npm_install_packages(
                &paths::devcontainer_dir(),
                &[("@devcontainers/cli", "latest")],
            )
            .await
        {
            log::error!(
                "Unable to install devcontainer CLI to data directory. Error: {:?}",
                e
            );
            return Err(DevContainerError::DevContainerCliNotAvailable);
        };

        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(datadir_cli_path.display().to_string());
        command.arg("--version");
        if let Err(e) = command.output().await {
            log::error!(
                "Unable to find devcontainer cli after NPM install. Error: {:?}",
                e
            );
            Err(DevContainerError::DevContainerCliNotAvailable)
        } else {
            Ok(DevContainerCli {
                path: datadir_cli_path,
                node_runtime_path: Some(node_runtime_path),
            })
        }
    } else {
        log::info!("Found devcontainer cli on $PATH, using it");
        Ok(DevContainerCli {
            path: PathBuf::from(&dev_container_cli()),
            node_runtime_path: None,
        })
    }
}

async fn devcontainer_up(
    context: &DevContainerContext,
    cli: &DevContainerCli,
    config_path: Option<&Path>,
) -> Result<DevContainerUp, DevContainerError> {
    let mut command = cli.command(context.use_podman);
    command.arg("up");
    command.arg("--workspace-folder");
    command.arg(context.project_directory.display().to_string());

    if let Some(config) = config_path {
        command.arg("--config");
        command.arg(config.display().to_string());
    }

    log::info!("Running full devcontainer up command: {:?}", command);

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                parse_json_from_cli(&raw)
            } else {
                let message = format!(
                    "Non-success status running devcontainer up for workspace: out: {}, err: {}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );

                log::error!("{}", &message);
                Err(DevContainerError::DevContainerUpFailed(message))
            }
        }
        Err(e) => {
            let message = format!("Error running devcontainer up: {:?}", e);
            log::error!("{}", &message);
            Err(DevContainerError::DevContainerUpFailed(message))
        }
    }
}

pub(crate) async fn read_devcontainer_configuration(
    context: &DevContainerContext,
    cli: &DevContainerCli,
    config_path: Option<&Path>,
) -> Result<DevContainerConfigurationOutput, DevContainerError> {
    let mut command = cli.command(context.use_podman);
    command.arg("read-configuration");
    command.arg("--workspace-folder");
    command.arg(context.project_directory.display().to_string());

    if let Some(config) = config_path {
        command.arg("--config");
        command.arg(config.display().to_string());
    }

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                parse_json_from_cli(&raw)
            } else {
                let message = format!(
                    "Non-success status running devcontainer read-configuration for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                log::error!("{}", &message);
                Err(DevContainerError::DevContainerNotFound)
            }
        }
        Err(e) => {
            let message = format!("Error running devcontainer read-configuration: {:?}", e);
            log::error!("{}", &message);
            Err(DevContainerError::DevContainerNotFound)
        }
    }
}

pub(crate) async fn apply_dev_container_template(
    template: &DevContainerTemplate,
    template_options: &HashMap<String, String>,
    features_selected: &HashSet<DevContainerFeature>,
    context: &DevContainerContext,
    cli: &DevContainerCli,
) -> Result<DevContainerApply, DevContainerError> {
    let mut command = cli.command(context.use_podman);

    let Ok(serialized_options) = serde_json::to_string(template_options) else {
        log::error!("Unable to serialize options for {:?}", template_options);
        return Err(DevContainerError::DevContainerParseFailed);
    };

    command.arg("templates");
    command.arg("apply");
    command.arg("--workspace-folder");
    command.arg(context.project_directory.display().to_string());
    command.arg("--template-id");
    command.arg(format!(
        "{}/{}",
        template
            .source_repository
            .as_ref()
            .unwrap_or(&String::from("")),
        template.id
    ));
    command.arg("--template-args");
    command.arg(serialized_options);
    command.arg("--features");
    command.arg(template_features_to_json(features_selected));

    log::debug!("Running full devcontainer apply command: {:?}", command);

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                parse_json_from_cli(&raw)
            } else {
                let message = format!(
                    "Non-success status running devcontainer templates apply for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );

                log::error!("{}", &message);
                Err(DevContainerError::DevContainerTemplateApplyFailed(message))
            }
        }
        Err(e) => {
            let message = format!("Error running devcontainer templates apply: {:?}", e);
            log::error!("{}", &message);
            Err(DevContainerError::DevContainerTemplateApplyFailed(message))
        }
    }
}
// Try to parse directly first (newer versions output pure JSON)
// If that fails, look for JSON start (older versions have plaintext prefix)
fn parse_json_from_cli<T: serde::de::DeserializeOwned>(raw: &str) -> Result<T, DevContainerError> {
    serde_json::from_str::<T>(&raw)
        .or_else(|e| {
            log::error!("Error parsing json: {} - will try to find json object in larger plaintext", e);
            let json_start = raw
                .find(|c| c == '{')
                .ok_or_else(|| {
                    log::error!("No JSON found in devcontainer up output");
                    DevContainerError::DevContainerParseFailed
                })?;

            serde_json::from_str(&raw[json_start..]).map_err(|e| {
                log::error!(
                    "Unable to parse JSON from devcontainer up output (starting at position {}), error: {:?}",
                    json_start,
                    e
                );
                DevContainerError::DevContainerParseFailed
            })
        })
}

fn get_backup_project_name(remote_workspace_folder: &str, container_id: &str) -> String {
    Path::new(remote_workspace_folder)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|string| string.to_string())
        .unwrap_or_else(|| container_id.to_string())
}

fn template_features_to_json(features_selected: &HashSet<DevContainerFeature>) -> String {
    let features_map = features_selected
        .iter()
        .map(|feature| {
            let mut map = HashMap::new();
            map.insert(
                "id",
                format!(
                    "{}/{}:{}",
                    feature
                        .source_repository
                        .as_ref()
                        .unwrap_or(&String::from("")),
                    feature.id,
                    feature.major_version()
                ),
            );
            map
        })
        .collect::<Vec<HashMap<&str, String>>>();
    serde_json::to_string(&features_map).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::devcontainer_api::{DevContainerUp, find_devcontainer_configs, parse_json_from_cli};
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }

    #[gpui::test]
    async fn test_find_devcontainer_configs_no_devcontainer_dir(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;

        let project = Project::test(fs, ["/project".as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let configs = cx.read(|cx| find_devcontainer_configs(workspace.read(cx), cx));
        assert!(
            configs.is_empty(),
            "Expected no configs when .devcontainer dir is absent, got: {configs:?}"
        );
    }

    #[gpui::test]
    async fn test_find_devcontainer_configs_single_default(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".devcontainer": {
                    "devcontainer.json": r#"{"image": "ubuntu"}"#
                }
            }),
        )
        .await;

        let project = Project::test(fs, ["/project".as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let configs = cx.read(|cx| find_devcontainer_configs(workspace.read(cx), cx));
        assert_eq!(
            configs.len(),
            1,
            "Expected exactly one config, got: {configs:?}"
        );
        assert_eq!(configs[0].name, "default");
        assert_eq!(
            configs[0].config_path.to_str().unwrap(),
            ".devcontainer/devcontainer.json"
        );
    }

    #[gpui::test]
    async fn test_find_devcontainer_configs_multiple_subfolders(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".devcontainer": {
                    "python": { "devcontainer.json": r#"{}"# },
                    "node": { "devcontainer.json": r#"{}"# }
                }
            }),
        )
        .await;

        let project = Project::test(fs, ["/project".as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let configs = cx.read(|cx| find_devcontainer_configs(workspace.read(cx), cx));
        assert_eq!(configs.len(), 2, "Expected two configs, got: {configs:?}");
        assert_eq!(configs[0].name, "node");
        assert_eq!(configs[1].name, "python");
    }

    #[gpui::test]
    async fn test_find_devcontainer_configs_default_plus_subfolders(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".devcontainer": {
                    "devcontainer.json": r#"{}"#,
                    "python": { "devcontainer.json": r#"{}"# },
                    "node": { "devcontainer.json": r#"{}"# }
                }
            }),
        )
        .await;

        let project = Project::test(fs, ["/project".as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let configs = cx.read(|cx| find_devcontainer_configs(workspace.read(cx), cx));
        assert_eq!(configs.len(), 3, "Expected three configs, got: {configs:?}");
        assert_eq!(
            configs[0].name, "default",
            "Default config should be sorted first"
        );
        assert_eq!(configs[1].name, "node");
        assert_eq!(configs[2].name, "python");
    }

    #[gpui::test]
    async fn test_find_devcontainer_configs_subfolder_without_json(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".devcontainer": {
                    "devcontainer.json": r#"{}"#,
                    "has_config": { "devcontainer.json": r#"{}"# },
                    "no_config": { "README.md": "not a devcontainer" }
                }
            }),
        )
        .await;

        let project = Project::test(fs, ["/project".as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let configs = cx.read(|cx| find_devcontainer_configs(workspace.read(cx), cx));
        assert_eq!(
            configs.len(),
            2,
            "Subfolder without devcontainer.json should be skipped, got: {configs:?}"
        );
        assert_eq!(configs[0].name, "default");
        assert_eq!(configs[1].name, "has_config");
    }

    #[test]
    fn should_parse_from_devcontainer_json() {
        let json = r#"{"outcome":"success","containerId":"826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a","remoteUser":"vscode","remoteWorkspaceFolder":"/workspaces/zed"}"#;
        let up: DevContainerUp = parse_json_from_cli(json).unwrap();
        assert_eq!(up._outcome, "success");
        assert_eq!(
            up.container_id,
            "826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a"
        );
        assert_eq!(up.remote_user, "vscode");
        assert_eq!(up.remote_workspace_folder, "/workspaces/zed");

        let json_in_plaintext = r#"[2026-01-22T16:19:08.802Z] @devcontainers/cli 0.80.1. Node.js v22.21.1. darwin 24.6.0 arm64.
            {"outcome":"success","containerId":"826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a","remoteUser":"vscode","remoteWorkspaceFolder":"/workspaces/zed"}"#;
        let up: DevContainerUp = parse_json_from_cli(json_in_plaintext).unwrap();
        assert_eq!(up._outcome, "success");
        assert_eq!(
            up.container_id,
            "826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a"
        );
        assert_eq!(up.remote_user, "vscode");
        assert_eq!(up.remote_workspace_folder, "/workspaces/zed");
    }
}

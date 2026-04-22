use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};

use futures::TryFutureExt;
use gpui::{AsyncWindowContext, Entity};
use project::Worktree;
use serde::Deserialize;
use settings::{DevContainerConnection, infer_json_indent_size, replace_value_in_json_text};
use util::rel_path::RelPath;
use walkdir::WalkDir;
use workspace::Workspace;
use worktree::Snapshot;

use crate::{
    DevContainerContext, DevContainerFeature, DevContainerTemplate,
    devcontainer_json::DevContainer,
    devcontainer_manifest::{read_devcontainer_configuration, spawn_dev_container},
    devcontainer_templates_repository, get_latest_oci_manifest, get_oci_token, ghcr_registry,
    oci::download_oci_tarball,
};

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

    pub fn root_config() -> Self {
        Self {
            name: "root".to_string(),
            config_path: PathBuf::from(".devcontainer.json"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainerUp {
    pub(crate) container_id: String,
    pub(crate) remote_user: String,
    pub(crate) remote_workspace_folder: String,
    #[serde(default)]
    pub(crate) extension_ids: Vec<String>,
    #[serde(default)]
    pub(crate) remote_env: HashMap<String, String>,
}

#[derive(Debug)]
pub(crate) struct DevContainerApply {
    pub(crate) project_files: Vec<Arc<RelPath>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevContainerError {
    CommandFailed(String),
    DockerNotAvailable,
    ContainerNotValid(String),
    DevContainerTemplateApplyFailed(String),
    DevContainerScriptsFailed,
    DevContainerUpFailed(String),
    DevContainerNotFound,
    DevContainerParseFailed,
    DevContainerValidationFailed(String),
    FilesystemError,
    ResourceFetchFailed,
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
                DevContainerError::ContainerNotValid(id) => format!(
                    "docker image {id} did not have expected configuration for a dev container"
                ),
                DevContainerError::DevContainerScriptsFailed =>
                    "lifecycle scripts could not execute for dev container".to_string(),
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
                DevContainerError::NotInValidProject => "Not within a valid project".to_string(),
                DevContainerError::CommandFailed(program) =>
                    format!("Failure running external program {program}"),
                DevContainerError::FilesystemError =>
                    "Error downloading resources locally".to_string(),
                DevContainerError::ResourceFetchFailed =>
                    "Failed to fetch resources from template or feature repository".to_string(),
                DevContainerError::DevContainerValidationFailed(failure) => failure.to_string(),
            }
        )
    }
}

pub(crate) async fn read_default_devcontainer_configuration(
    cx: &DevContainerContext,
    environment: HashMap<String, String>,
) -> Result<DevContainer, DevContainerError> {
    let default_config = DevContainerConfig::default_config();

    read_devcontainer_configuration(default_config, cx, environment)
        .await
        .map_err(|e| {
            log::error!("Default configuration not found: {:?}", e);
            DevContainerError::DevContainerNotFound
        })
}

/// Finds all available devcontainer configurations in the project.
///
/// See [`find_configs_in_snapshot`] for the locations that are scanned.
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
    find_configs_in_snapshot(worktree)
}

/// Scans a worktree snapshot for devcontainer configurations.
///
/// Scans for configurations in these locations:
/// 1. `.devcontainer/devcontainer.json` (the default location)
/// 2. `.devcontainer.json` in the project root
/// 3. `.devcontainer/<subfolder>/devcontainer.json` (named configurations)
///
/// All found configurations are returned so the user can pick between them.
pub fn find_configs_in_snapshot(snapshot: &Snapshot) -> Vec<DevContainerConfig> {
    let mut configs = Vec::new();

    let devcontainer_dir_path = RelPath::unix(".devcontainer").expect("valid path");

    if let Some(devcontainer_entry) = snapshot.entry_for_path(devcontainer_dir_path) {
        if devcontainer_entry.is_dir() {
            log::debug!("find_configs_in_snapshot: Scanning .devcontainer directory");
            let devcontainer_json_path =
                RelPath::unix(".devcontainer/devcontainer.json").expect("valid path");
            for entry in snapshot.child_entries(devcontainer_dir_path) {
                log::debug!(
                    "find_configs_in_snapshot: Found entry: {:?}, is_file: {}, is_dir: {}",
                    entry.path.as_unix_str(),
                    entry.is_file(),
                    entry.is_dir()
                );

                if entry.is_file() && entry.path.as_ref() == devcontainer_json_path {
                    log::debug!("find_configs_in_snapshot: Found default devcontainer.json");
                    configs.push(DevContainerConfig::default_config());
                } else if entry.is_dir() {
                    let subfolder_name = entry
                        .path
                        .file_name()
                        .map(|n| n.to_string())
                        .unwrap_or_default();

                    let config_json_path =
                        format!("{}/devcontainer.json", entry.path.as_unix_str());
                    if let Ok(rel_config_path) = RelPath::unix(&config_json_path) {
                        if snapshot.entry_for_path(rel_config_path).is_some() {
                            log::debug!(
                                "find_configs_in_snapshot: Found config in subfolder: {}",
                                subfolder_name
                            );
                            configs.push(DevContainerConfig {
                                name: subfolder_name,
                                config_path: PathBuf::from(&config_json_path),
                            });
                        } else {
                            log::debug!(
                                "find_configs_in_snapshot: Subfolder {} has no devcontainer.json",
                                subfolder_name
                            );
                        }
                    }
                }
            }
        }
    }

    // Always include `.devcontainer.json` so the user can pick it from the UI
    // even when `.devcontainer/devcontainer.json` also exists.
    let root_config_path = RelPath::unix(".devcontainer.json").expect("valid path");
    if snapshot
        .entry_for_path(root_config_path)
        .is_some_and(|entry| entry.is_file())
    {
        log::debug!("find_configs_in_snapshot: Found .devcontainer.json in project root");
        configs.push(DevContainerConfig::root_config());
    }

    log::info!(
        "find_configs_in_snapshot: Found {} configurations",
        configs.len()
    );

    configs.sort_by(|a, b| {
        let a_is_primary = a.name == "default" || a.name == "root";
        let b_is_primary = b.name == "default" || b.name == "root";
        match (a_is_primary, b_is_primary) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    configs
}

pub async fn start_dev_container_with_config(
    context: DevContainerContext,
    config: Option<DevContainerConfig>,
    environment: HashMap<String, String>,
) -> Result<(DevContainerConnection, String), DevContainerError> {
    check_for_docker(context.use_podman).await?;

    let Some(actual_config) = config.clone() else {
        return Err(DevContainerError::NotInValidProject);
    };

    match spawn_dev_container(
        &context,
        environment.clone(),
        actual_config.clone(),
        context.project_directory.clone().as_ref(),
    )
    .await
    {
        Ok(DevContainerUp {
            container_id,
            remote_workspace_folder,
            remote_user,
            extension_ids,
            remote_env,
            ..
        }) => {
            let project_name =
                match read_devcontainer_configuration(actual_config, &context, environment).await {
                    Ok(DevContainer {
                        name: Some(name), ..
                    }) => name,
                    _ => get_backup_project_name(&remote_workspace_folder, &container_id),
                };

            let connection = DevContainerConnection {
                name: project_name,
                container_id,
                use_podman: context.use_podman,
                remote_user,
                extension_ids,
                remote_env: remote_env.into_iter().collect(),
            };

            Ok((connection, remote_workspace_folder))
        }
        Err(err) => {
            let message = format!("Failed with nested error: {:?}", err);
            Err(DevContainerError::DevContainerUpFailed(message))
        }
    }
}

async fn check_for_docker(use_podman: bool) -> Result<(), DevContainerError> {
    let mut command = if use_podman {
        util::command::new_command("podman")
    } else {
        util::command::new_command("docker")
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

pub(crate) async fn apply_devcontainer_template(
    worktree: Entity<Worktree>,
    template: &DevContainerTemplate,
    template_options: &HashMap<String, String>,
    features_selected: &HashSet<DevContainerFeature>,
    context: &DevContainerContext,
    cx: &mut AsyncWindowContext,
) -> Result<DevContainerApply, DevContainerError> {
    let token = get_oci_token(
        ghcr_registry(),
        devcontainer_templates_repository(),
        &context.http_client,
    )
    .map_err(|e| {
        log::error!("Failed to get OCI auth token: {e}");
        DevContainerError::ResourceFetchFailed
    })
    .await?;
    let manifest = get_latest_oci_manifest(
        &token.token,
        ghcr_registry(),
        devcontainer_templates_repository(),
        &context.http_client,
        Some(&template.id),
    )
    .map_err(|e| {
        log::error!("Failed to fetch template from OCI repository: {e}");
        DevContainerError::ResourceFetchFailed
    })
    .await?;

    let layer = &manifest.layers.get(0).ok_or_else(|| {
        log::error!("Given manifest has no layers to query for blob. Aborting");
        DevContainerError::ResourceFetchFailed
    })?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let extract_dir = std::env::temp_dir()
        .join(&template.id)
        .join(format!("extracted-{timestamp}"));

    context.fs.create_dir(&extract_dir).await.map_err(|e| {
        log::error!("Could not create temporary directory: {e}");
        DevContainerError::FilesystemError
    })?;

    download_oci_tarball(
        &token.token,
        ghcr_registry(),
        devcontainer_templates_repository(),
        &layer.digest,
        "application/vnd.oci.image.manifest.v1+json",
        &extract_dir,
        &context.http_client,
        &context.fs,
        Some(&template.id),
    )
    .map_err(|e| {
        log::error!("Error downloading tarball: {:?}", e);
        DevContainerError::ResourceFetchFailed
    })
    .await?;

    let downloaded_devcontainer_folder = &extract_dir.join(".devcontainer/");
    let mut project_files = Vec::new();
    for entry in WalkDir::new(downloaded_devcontainer_folder) {
        let Ok(entry) = entry else {
            continue;
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let relative_path = entry.path().strip_prefix(&extract_dir).map_err(|e| {
            log::error!("Can't create relative path: {e}");
            DevContainerError::FilesystemError
        })?;
        let rel_path = RelPath::unix(relative_path)
            .map_err(|e| {
                log::error!("Can't create relative path: {e}");
                DevContainerError::FilesystemError
            })?
            .into_arc();
        let content = context.fs.load(entry.path()).await.map_err(|e| {
            log::error!("Unable to read file: {e}");
            DevContainerError::FilesystemError
        })?;

        let mut content = expand_template_options(content, template_options);
        if let Some("devcontainer.json") = &rel_path.file_name() {
            content = insert_features_into_devcontainer_json(&content, features_selected)
        }
        worktree
            .update(cx, |worktree, cx| {
                worktree.create_entry(rel_path.clone(), false, Some(content.into_bytes()), cx)
            })
            .await
            .map_err(|e| {
                log::error!("Unable to create entry in worktree: {e}");
                DevContainerError::NotInValidProject
            })?;
        project_files.push(rel_path);
    }

    Ok(DevContainerApply { project_files })
}

fn insert_features_into_devcontainer_json(
    content: &str,
    features: &HashSet<DevContainerFeature>,
) -> String {
    if features.is_empty() {
        return content.to_string();
    }

    let features_value: serde_json::Value = features
        .iter()
        .map(|f| {
            let key = format!(
                "{}/{}:{}",
                f.source_repository.as_deref().unwrap_or(""),
                f.id,
                f.major_version()
            );
            (key, serde_json::Value::Object(Default::default()))
        })
        .collect::<serde_json::Map<String, serde_json::Value>>()
        .into();

    let tab_size = infer_json_indent_size(content);
    let (range, replacement) = replace_value_in_json_text(
        content,
        &["features"],
        tab_size,
        Some(&features_value),
        None,
    );

    let mut result = content.to_string();
    result.replace_range(range, &replacement);
    result
}

fn expand_template_options(content: String, template_options: &HashMap<String, String>) -> String {
    let mut replaced_content = content;
    for (key, val) in template_options {
        replaced_content = replaced_content.replace(&format!("${{templateOption:{key}}}"), val)
    }
    replaced_content
}

fn get_backup_project_name(remote_workspace_folder: &str, container_id: &str) -> String {
    Path::new(remote_workspace_folder)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|string| string.to_string())
        .unwrap_or_else(|| container_id.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::devcontainer_api::{DevContainerConfig, find_configs_in_snapshot};
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    #[gpui::test]
    async fn test_find_configs_root_devcontainer_json(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".devcontainer.json": "{}"
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();

        let configs = project.read_with(cx, |project, cx| {
            let worktree = project
                .visible_worktrees(cx)
                .next()
                .expect("should have a worktree");
            find_configs_in_snapshot(worktree.read(cx))
        });

        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].name, "root");
        assert_eq!(configs[0].config_path, PathBuf::from(".devcontainer.json"));
    }

    #[gpui::test]
    async fn test_find_configs_default_devcontainer_dir(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".devcontainer": {
                    "devcontainer.json": "{}"
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();

        let configs = project.read_with(cx, |project, cx| {
            let worktree = project
                .visible_worktrees(cx)
                .next()
                .expect("should have a worktree");
            find_configs_in_snapshot(worktree.read(cx))
        });

        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0], DevContainerConfig::default_config());
    }

    #[gpui::test]
    async fn test_find_configs_dir_and_root_both_included(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".devcontainer.json": "{}",
                ".devcontainer": {
                    "devcontainer.json": "{}"
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();

        let configs = project.read_with(cx, |project, cx| {
            let worktree = project
                .visible_worktrees(cx)
                .next()
                .expect("should have a worktree");
            find_configs_in_snapshot(worktree.read(cx))
        });

        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0], DevContainerConfig::default_config());
        assert_eq!(configs[1], DevContainerConfig::root_config());
    }

    #[gpui::test]
    async fn test_find_configs_subfolder_configs(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".devcontainer": {
                    "rust": {
                        "devcontainer.json": "{}"
                    },
                    "python": {
                        "devcontainer.json": "{}"
                    }
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();

        let configs = project.read_with(cx, |project, cx| {
            let worktree = project
                .visible_worktrees(cx)
                .next()
                .expect("should have a worktree");
            find_configs_in_snapshot(worktree.read(cx))
        });

        assert_eq!(configs.len(), 2);
        let names: Vec<&str> = configs.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"python"));
        assert!(names.contains(&"rust"));
    }

    #[gpui::test]
    async fn test_find_configs_default_and_subfolder(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".devcontainer": {
                    "devcontainer.json": "{}",
                    "gpu": {
                        "devcontainer.json": "{}"
                    }
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();

        let configs = project.read_with(cx, |project, cx| {
            let worktree = project
                .visible_worktrees(cx)
                .next()
                .expect("should have a worktree");
            find_configs_in_snapshot(worktree.read(cx))
        });

        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].name, "default");
        assert_eq!(configs[1].name, "gpu");
    }

    #[gpui::test]
    async fn test_find_configs_no_devcontainer(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "src": {
                    "main.rs": "fn main() {}"
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();

        let configs = project.read_with(cx, |project, cx| {
            let worktree = project
                .visible_worktrees(cx)
                .next()
                .expect("should have a worktree");
            find_configs_in_snapshot(worktree.read(cx))
        });

        assert!(configs.is_empty());
    }

    #[gpui::test]
    async fn test_find_configs_root_json_and_subfolder_configs(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".devcontainer.json": "{}",
                ".devcontainer": {
                    "rust": {
                        "devcontainer.json": "{}"
                    }
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();

        let configs = project.read_with(cx, |project, cx| {
            let worktree = project
                .visible_worktrees(cx)
                .next()
                .expect("should have a worktree");
            find_configs_in_snapshot(worktree.read(cx))
        });

        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].name, "root");
        assert_eq!(configs[0].config_path, PathBuf::from(".devcontainer.json"));
        assert_eq!(configs[1].name, "rust");
        assert_eq!(
            configs[1].config_path,
            PathBuf::from(".devcontainer/rust/devcontainer.json")
        );
    }

    #[gpui::test]
    async fn test_find_configs_empty_devcontainer_dir_falls_back_to_root(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".devcontainer.json": "{}",
                ".devcontainer": {}
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        cx.run_until_parked();

        let configs = project.read_with(cx, |project, cx| {
            let worktree = project
                .visible_worktrees(cx)
                .next()
                .expect("should have a worktree");
            find_configs_in_snapshot(worktree.read(cx))
        });

        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0], DevContainerConfig::root_config());
    }
}

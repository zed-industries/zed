use std::{collections::HashSet, path::Path, sync::Arc};

use anyhow::{Context as _, Result, ensure};
use gpui::{App, Global, PromptLevel, Task, Window};
use parking_lot::Mutex;
use remote::{
    Interactive, RemoteConnection, RemoteConnectionOptions, same_remote_connection_identity,
};
use settings::{
    ArcWorkspace, ArcWorkspacesConfig, RegisterSetting, RemoteProject, Settings, SshConnection,
    update_settings_file_with_completion,
};

use crate::{MultiWorkspace, SerializedWorkspaceLocation, WorkspaceDb};

#[derive(RegisterSetting)]
struct ArcSettings {
    connections: Vec<SshConnection>,
}

impl Settings for ArcSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            connections: content.remote.ssh_connections.clone().unwrap_or_default(),
        }
    }
}

pub fn ssh_connection(options: &RemoteConnectionOptions, cx: &App) -> Option<SshConnection> {
    ArcSettings::get_global(cx)
        .connections
        .iter()
        .find(|connection| {
            let candidate = RemoteConnectionOptions::Ssh((*connection).clone().into());
            same_remote_connection_identity(Some(options), Some(&candidate))
        })
        .cloned()
}

pub fn config_for_connection(
    options: &RemoteConnectionOptions,
    cx: &App,
) -> Option<ArcWorkspacesConfig> {
    ssh_connection(options, cx)?
        .arc_workspaces
        .filter(|config| !config.types.is_empty())
}

pub fn managed_workspace(
    options: &RemoteConnectionOptions,
    path: &Path,
    cx: &App,
) -> Option<ArcWorkspace> {
    ssh_connection(options, cx)?
        .projects
        .iter()
        .filter_map(|project| project.arc_workspace.as_ref())
        .find(|workspace| Path::new(&workspace.project_path) == path)
        .cloned()
}

pub fn managed_workspace_for_group(
    key: &crate::ProjectGroupKey,
    cx: &App,
) -> Option<(RemoteConnectionOptions, ArcWorkspace)> {
    let options = key.host()?;
    let paths = key.path_list().paths();
    if paths.len() != 1 {
        return None;
    }
    let workspace = managed_workspace(&options, paths.first()?, cx)?;
    Some((options, workspace))
}

pub fn validate_name(name: &str) -> Result<()> {
    ensure!(
        !name.is_empty()
            && name
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphanumeric)
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"_.-".contains(&byte)),
        "Workspace name must start with a letter or digit and contain only letters, digits, _, . or -"
    );
    Ok(())
}

pub fn validate_relative_path(path: &str) -> Result<()> {
    ensure!(
        !path.is_empty()
            && !path.starts_with('/')
            && !path.contains(['\n', '\r', '\0'])
            && path
                .split('/')
                .all(|component| !component.is_empty() && component != "." && component != ".."),
        "Workspace type must specify a relative path without . or .. components"
    );
    Ok(())
}

const CREATE_SCRIPT: &str = r#"
set -euo pipefail
name="$1"
root="$2"
relative="$3"
case "$root" in '~/'*) root="$HOME/${root:2}" ;; esac
[[ "$root" == /* && "$root" != / && "$root" != "$HOME" ]] || { echo 'Invalid Arc mount root' >&2; exit 1; }
mkdir -p -- "$root"
root="$(cd -- "$root" && pwd -P)"
[[ "$root" != / && "$root" != "$HOME" ]] || { echo 'Invalid Arc mount root' >&2; exit 1; }
mount="$root/$name"
[[ ! -L "$mount" ]] || { echo 'Arc mount path is a symbolic link' >&2; exit 1; }
if [[ -e "$mount" ]]; then
    [[ -d "$mount" && -e "$mount/.arcadia.root" ]] || { echo 'Directory exists but is not an Arc mount' >&2; exit 1; }
else
    arc mount "$mount" >&2
fi
[[ -e "$mount/.arcadia.root" ]] || { echo "Arc mount is missing .arcadia.root: $mount" >&2; exit 1; }
project="$mount/$relative"
[[ -d "$project" ]] || { echo "Project directory does not exist: $project" >&2; exit 1; }
project="$(cd -- "$project" && pwd -P)"
[[ "$project" == "$mount/"* ]] || { echo 'Project directory escapes the Arc mount' >&2; exit 1; }
# Preserve PATH-based bootstrap for installations without git.arc_executable.
# A configured adapter is initialized by Worktree::local before scanning.
bash -lic 'git -C "$1" --version' zed-arc-workspace-git "$project" >&2
printf '%s\n%s\n' "$mount" "$project"
"#;

const DELETE_SCRIPT: &str = r#"
set -euo pipefail
name="$1"
mount="$2"
project="$3"
[[ "$mount" == /* && "$mount" == */"$name" && "$mount" != "$HOME" && "$project" == "$mount/"* ]] || { echo 'Invalid Arc workspace metadata' >&2; exit 1; }
[[ ! -L "$mount" ]] || { echo 'Arc mount path is a symbolic link' >&2; exit 1; }
store="$HOME/.arc/stores/${mount//\//_}"
[[ ! -L "$store" ]] || { echo 'Arc store path is a symbolic link' >&2; exit 1; }
if [[ -e "$mount/.arcadia.root" ]]; then
    if ! arc unmount "$mount" >&2; then
        arc unmount --force "$mount" >&2
    fi
    [[ ! -e "$mount/.arcadia.root" ]] || { echo 'Arc mount is still mounted; refusing to remove it' >&2; exit 1; }
elif [[ -e "$mount" ]]; then
    [[ -d "$mount" && -z "$(find "$mount" -mindepth 1 -maxdepth 1 -print -quit)" ]] || { echo 'Directory is not an Arc mount or an empty unmounted workspace' >&2; exit 1; }
fi
if [[ -e "$mount" ]]; then rm -r -- "$mount"; fi
if [[ -e "$store" ]]; then rm -r -- "$store"; fi
"#;

async fn run_script(
    connection: &Arc<dyn RemoteConnection>,
    script: &str,
    arguments: &[String],
) -> Result<String> {
    let mut args = vec![
        "-c".to_owned(),
        script.to_owned(),
        "zed-arc-workspace".to_owned(),
    ];
    args.extend_from_slice(arguments);
    let command = connection.build_command(
        Some("bash".into()),
        &args,
        &Default::default(),
        None,
        None,
        Interactive::No,
    )?;
    let output = util::command::new_command(command.program)
        .args(command.args)
        .envs(command.env)
        .output()
        .await?;
    ensure!(
        output.status.success(),
        "Arc operation failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    String::from_utf8(output.stdout).context("Arc operation returned invalid UTF-8")
}

pub async fn create_workspace(
    connection: &Arc<dyn RemoteConnection>,
    config: &ArcWorkspacesConfig,
    name: String,
    workspace_type: String,
) -> Result<ArcWorkspace> {
    validate_name(&name)?;
    let relative = config
        .types
        .get(&workspace_type)
        .context("Unknown Arc workspace type")?;
    validate_relative_path(relative)?;
    ensure!(
        !config.mount_root.contains(['\n', '\r', '\0']),
        "Invalid Arc mount root"
    );
    let output = run_script(
        connection,
        CREATE_SCRIPT,
        &[name.clone(), config.mount_root.clone(), relative.clone()],
    )
    .await?;
    let mut lines = output.lines();
    let mount_path = lines
        .next()
        .context("Arc mount path is missing")?
        .to_owned();
    let project_path = lines
        .next()
        .context("Arc project path is missing")?
        .to_owned();
    ensure!(
        lines.next().is_none(),
        "Unexpected output from Arc workspace creation"
    );
    let workspace = ArcWorkspace {
        name,
        workspace_type,
        mount_path,
        project_path,
    };
    validate_metadata(&workspace)?;
    Ok(workspace)
}

fn validate_metadata(workspace: &ArcWorkspace) -> Result<()> {
    validate_name(&workspace.name)?;
    let mount = Path::new(&workspace.mount_path);
    let project = Path::new(&workspace.project_path);
    ensure!(
        mount.is_absolute()
            && mount
                .file_name()
                .is_some_and(|name| name == workspace.name.as_str())
            && project != mount
            && project.starts_with(mount)
            && !workspace.mount_path.contains(['\n', '\r', '\0'])
            && !workspace.project_path.contains(['\n', '\r', '\0'])
            && !mount
                .components()
                .chain(project.components())
                .any(|part| matches!(
                    part,
                    std::path::Component::ParentDir | std::path::Component::CurDir
                )),
        "Invalid Arc workspace paths"
    );
    Ok(())
}

pub async fn delete_workspace(
    connection: &Arc<dyn RemoteConnection>,
    workspace: &ArcWorkspace,
) -> Result<()> {
    validate_metadata(workspace)?;
    run_script(
        connection,
        DELETE_SCRIPT,
        &[
            workspace.name.clone(),
            workspace.mount_path.clone(),
            workspace.project_path.clone(),
        ],
    )
    .await?;
    Ok(())
}

pub fn save_workspace(
    server: SshConnection,
    workspace: ArcWorkspace,
    fs: Arc<dyn fs::Fs>,
    cx: &App,
) -> futures::channel::oneshot::Receiver<Result<()>> {
    update_settings_file_with_completion(fs, cx, move |settings, _| {
        let connections = settings.remote.ssh_connections.get_or_insert_with(Vec::new);
        let index = connections.iter().position(|connection| {
            connection.host == server.host
                && connection.username == server.username
                && connection.port == server.port
        });
        let connection = if let Some(index) = index {
            connections.get_mut(index)
        } else {
            connections.push(server);
            connections.last_mut()
        };
        if let Some(connection) = connection {
            connection.projects.retain(|project| {
                project.paths.as_slice() != std::slice::from_ref(&workspace.project_path)
            });
            connection.projects.insert(RemoteProject {
                paths: vec![workspace.project_path.clone()],
                arc_workspace: Some(workspace),
            });
        }
    })
}

#[derive(Default)]
struct PendingDeletes(Arc<Mutex<HashSet<String>>>);
impl Global for PendingDeletes {}

pub struct DeletionGuard {
    key: String,
    pending: Arc<Mutex<HashSet<String>>>,
}
impl Drop for DeletionGuard {
    fn drop(&mut self) {
        self.pending.lock().remove(&self.key);
    }
}

pub fn begin_deletion(
    options: &RemoteConnectionOptions,
    workspace: &ArcWorkspace,
    cx: &mut App,
) -> Result<DeletionGuard> {
    if !cx.has_global::<PendingDeletes>() {
        cx.set_global(PendingDeletes::default());
    }
    let pending = cx.global::<PendingDeletes>().0.clone();
    let key = format!(
        "{}:{}",
        remote::remote_connection_identity(options).persistence_key(),
        workspace.mount_path
    );
    ensure!(
        pending.lock().insert(key.clone()),
        "This Arc workspace is already being deleted"
    );
    Ok(DeletionGuard { key, pending })
}

pub fn confirm_deletion(
    options: &RemoteConnectionOptions,
    workspace: &ArcWorkspace,
    window: &mut Window,
    cx: &mut App,
) -> futures::channel::oneshot::Receiver<usize> {
    window.prompt(PromptLevel::Warning, &format!("Delete Arc workspace ‘{}’ from the server?", workspace.name), Some(&format!("Server: {}\nMount: {}\n\nThe entire Arc mount and its local store will be permanently deleted, including files outside the opened project folder. Zed will retry unmounting with --force if necessary.", options.display_name(), workspace.mount_path)), &["Cancel", "Delete Workspace"], cx)
}

pub fn remove_open_folder(
    project: gpui::Entity<project::Project>,
    worktree_id: settings::WorktreeId,
    fs: Arc<dyn fs::Fs>,
    window: &mut Window,
    cx: &mut App,
) -> Task<Result<bool>> {
    let Some(client) = project.read(cx).remote_client() else {
        project.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        return Task::ready(Ok(true));
    };
    let options = client.read(cx).connection_options();
    let Some(worktree) = project.read(cx).worktree_for_id(worktree_id, cx) else {
        return Task::ready(Ok(false));
    };
    let Some(workspace) = managed_workspace(&options, &worktree.read(cx).abs_path(), cx) else {
        project.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        return Task::ready(Ok(true));
    };
    let guard = match begin_deletion(&options, &workspace, cx) {
        Ok(guard) => guard,
        Err(error) => return Task::ready(Err(error)),
    };
    let prompt = confirm_deletion(&options, &workspace, window, cx);
    let connection = client.read(cx).remote_connection();
    cx.spawn(async move |cx| {
        let _guard = guard;
        if prompt.await? != 1 {
            return Ok(false);
        }
        let connection =
            connection.context("Reconnect to the remote server before deleting this workspace")?;
        delete_workspace(&connection, &workspace).await?;
        finish_deletion(options, workspace, fs, cx).await?;
        Ok(true)
    })
}

pub async fn finish_deletion(
    options: RemoteConnectionOptions,
    workspace: ArcWorkspace,
    fs: Arc<dyn fs::Fs>,
    cx: &mut gpui::AsyncApp,
) -> Result<()> {
    let mount = std::path::PathBuf::from(&workspace.mount_path);
    let receiver = cx.update(|cx| {
        update_settings_file_with_completion(fs.clone(), cx, {
            let options = options.clone();
            let mount = mount.clone();
            move |settings, _| {
                if let Some(connections) = settings.remote.ssh_connections.as_mut() {
                    for connection in connections {
                        let candidate = RemoteConnectionOptions::Ssh(connection.clone().into());
                        if same_remote_connection_identity(Some(&options), Some(&candidate)) {
                            connection.projects.retain(|project| {
                                !project
                                    .paths
                                    .iter()
                                    .any(|path| Path::new(path).starts_with(&mount))
                            });
                        }
                    }
                }
            }
        })
    });
    receiver.await??;
    cx.update(|cx| {
        for window in cx.windows() {
            if let Some(window) = window.downcast::<MultiWorkspace>() {
                window
                    .update(cx, |multi_workspace, _, cx| {
                        let workspaces: Vec<_> = multi_workspace.workspaces().cloned().collect();
                        for workspace in workspaces {
                            let project = workspace.read(cx).project().clone();
                            let Some(client) = project.read(cx).remote_client() else {
                                continue;
                            };
                            if !same_remote_connection_identity(
                                Some(&options),
                                Some(&client.read(cx).connection_options()),
                            ) {
                                continue;
                            }
                            let ids: Vec<_> = project
                                .read(cx)
                                .visible_worktrees(cx)
                                .filter(|worktree| worktree.read(cx).abs_path().starts_with(&mount))
                                .map(|worktree| worktree.read(cx).id())
                                .collect();
                            project.update(cx, |project, cx| {
                                for id in ids {
                                    project.remove_worktree(id, cx);
                                }
                            });
                        }
                    })
                    .context("Updating windows after Arc workspace deletion")?;
            }
        }
        Ok::<_, anyhow::Error>(())
    })?;
    let db = cx.update(|cx| WorkspaceDb::global(cx));
    for recent in db.recent_project_workspaces_ungrouped(fs.as_ref()).await? {
        if let SerializedWorkspaceLocation::Remote(connection) = &recent.location {
            if same_remote_connection_identity(Some(&options), Some(connection))
                && recent
                    .paths
                    .paths()
                    .iter()
                    .any(|path| path.starts_with(&mount))
            {
                db.delete_workspace_by_id(recent.workspace_id).await?;
            }
        }
    }
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::{
        collections::BTreeMap,
        os::unix::fs::PermissionsExt,
        path::PathBuf,
        process::{Command, Output},
    };

    struct ArcFixture {
        directory: tempfile::TempDir,
        home: PathBuf,
    }

    impl ArcFixture {
        fn new() -> Result<Self> {
            let directory = tempfile::tempdir()?;
            let home = directory.path().join("remote home 'quoted'");
            std::fs::create_dir_all(&home)?;
            let home = std::fs::canonicalize(home)?;
            let binary = directory.path().join("arc");
            std::fs::write(
                &binary,
                r#"#!/bin/bash
set -euo pipefail
printf '%s\n' "$*" >> "$HOME/commands"
operation="$1"; shift
case "$operation" in
mount)
    mount="$1"
    mkdir -p "$mount/custom/project" "$HOME/.arc/stores/${mount//\//_}"
    touch "$mount/.arcadia.root" "$HOME/.arc/stores/${mount//\//_}/data"
    ;;
unmount)
    if [[ "$1" != --force && "${NORMAL_FAIL:-0}" == 1 ]]; then echo 'mount is busy' >&2; exit 1; fi
    if [[ "$1" == --force ]]; then
        shift
        if [[ "${FORCE_FAIL:-0}" == 1 ]]; then echo 'forced unmount failed' >&2; exit 1; fi
    fi
    mount="$1"
    rm -r -- "$mount/.arcadia.root" "$mount/custom"
    ;;
esac
"#,
            )?;
            std::fs::set_permissions(&binary, std::fs::Permissions::from_mode(0o755))?;
            let git_directory = directory.path().join("login-bin");
            std::fs::create_dir(&git_directory)?;
            let git = git_directory.join("git");
            std::fs::write(
                &git,
                r#"#!/bin/bash
set -euo pipefail
printf '%s\n' "$*" >> "$HOME/git-commands"
if [[ "${GIT_FAIL:-0}" == 1 ]]; then echo 'Git shim failed' >&2; exit 1; fi
[[ "$1" == -C && "$3" == --version ]]
root="$2"
while [[ ! -e "$root/.arcadia.root" ]]; do
    [[ "$root" != / ]] || exit 1
    root="$(dirname -- "$root")"
done
mkdir -p -- "$root/.git"
touch "$root/.git/.arc2git-zed"
printf 'git version arc2git-zed\n'
"#,
            )?;
            std::fs::set_permissions(&git, std::fs::Permissions::from_mode(0o755))?;
            // Simulate a shim configured only in the interactive login shell.
            std::fs::write(
                home.join(".bash_profile"),
                format!("export PATH=\"{}:$PATH\"\n", git_directory.display()),
            )?;
            Ok(Self { directory, home })
        }

        fn mount(&self) -> PathBuf {
            self.home.join("arcadia-worktrees/sample")
        }
        fn store(&self) -> PathBuf {
            self.home
                .join(".arc/stores")
                .join(self.mount().to_string_lossy().replace('/', "_"))
        }
        fn run(&self, script: &str, args: &[&str], env: &[(&str, &str)]) -> Result<Output> {
            Ok(Command::new("bash")
                .arg("-c")
                .arg(script)
                .arg("zed-arc-workspace")
                .args(args)
                .env("HOME", &self.home)
                .env(
                    "PATH",
                    format!("{}:/usr/bin:/bin", self.directory.path().display()),
                )
                .envs(env.iter().copied())
                .output()?)
        }
        fn create(&self) -> Result<()> {
            let output = self.run(
                CREATE_SCRIPT,
                &["sample", "~/arcadia-worktrees", "custom/project"],
                &[],
            )?;
            ensure!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            Ok(())
        }
        fn delete(&self, env: &[(&str, &str)]) -> Result<Output> {
            self.run(
                DELETE_SCRIPT,
                &[
                    "sample",
                    &self.mount().to_string_lossy(),
                    &self.mount().join("custom/project").to_string_lossy(),
                ],
                env,
            )
        }
    }

    #[test]
    fn mounts_configured_path_and_reuses_existing_arc_mount() -> Result<()> {
        let fixture = ArcFixture::new()?;
        fixture.create()?;
        assert!(fixture.mount().join(".git/.arc2git-zed").is_file());
        assert!(!fixture.mount().join("custom/project/.git").exists());
        fixture.create()?;
        assert!(!fixture.mount().join("custom/project/.git").exists());
        let commands = std::fs::read_to_string(fixture.home.join("commands"))?;
        assert_eq!(
            commands
                .lines()
                .filter(|command| command.starts_with("mount "))
                .count(),
            1
        );
        assert!(fixture.store().join("data").exists());
        Ok(())
    }

    #[test]
    fn initializes_git_before_returning_project_paths() -> Result<()> {
        let fixture = ArcFixture::new()?;
        let output = fixture.run(
            CREATE_SCRIPT,
            &["sample", "~/arcadia-worktrees", "custom/project"],
            &[],
        )?;
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout)?,
            format!(
                "{}\n{}\n",
                fixture.mount().display(),
                fixture.mount().join("custom/project").display()
            )
        );
        assert!(fixture.mount().join(".git/.arc2git-zed").is_file());
        assert_eq!(
            std::fs::read_to_string(fixture.home.join("git-commands"))?,
            format!(
                "-C {} --version\n",
                fixture.mount().join("custom/project").display()
            )
        );
        Ok(())
    }

    #[test]
    fn failed_git_initialization_does_not_return_project_paths() -> Result<()> {
        let fixture = ArcFixture::new()?;
        let output = fixture.run(
            CREATE_SCRIPT,
            &["sample", "~/arcadia-worktrees", "custom/project"],
            &[("GIT_FAIL", "1")],
        )?;
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Git shim failed"));
        assert!(fixture.mount().join(".arcadia.root").exists());
        Ok(())
    }

    #[test]
    fn removes_mount_and_store_but_preserves_neighbour() -> Result<()> {
        let fixture = ArcFixture::new()?;
        fixture.create()?;
        let neighbour = fixture.home.join("arcadia-worktrees/neighbour");
        std::fs::create_dir(&neighbour)?;
        std::fs::write(neighbour.join("keep"), "keep")?;
        let output = fixture.delete(&[])?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!fixture.mount().exists());
        assert!(!fixture.store().exists());
        assert!(neighbour.join("keep").exists());
        assert!(!std::fs::read_to_string(fixture.home.join("commands"))?.contains("--force"));
        assert!(fixture.delete(&[])?.status.success());
        Ok(())
    }

    #[test]
    fn falls_back_to_force_only_after_unmount_failure() -> Result<()> {
        let fixture = ArcFixture::new()?;
        fixture.create()?;
        let output = fixture.delete(&[("NORMAL_FAIL", "1")])?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!fixture.mount().exists());
        assert!(!fixture.store().exists());
        let commands = std::fs::read_to_string(fixture.home.join("commands"))?;
        let unmounts: Vec<_> = commands
            .lines()
            .filter(|command| command.starts_with("unmount "))
            .collect();
        assert_eq!(unmounts.len(), 2);
        assert!(!unmounts[0].contains("--force"));
        assert!(unmounts[1].contains("--force"));
        Ok(())
    }

    #[test]
    fn failed_forced_unmount_preserves_mount_and_store() -> Result<()> {
        let fixture = ArcFixture::new()?;
        fixture.create()?;
        let output = fixture.delete(&[("NORMAL_FAIL", "1"), ("FORCE_FAIL", "1")])?;
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("forced unmount failed"));
        assert!(fixture.mount().join(".arcadia.root").exists());
        assert!(fixture.store().join("data").exists());
        Ok(())
    }

    #[test]
    fn refuses_existing_non_arc_directory_and_symlink() -> Result<()> {
        let fixture = ArcFixture::new()?;
        std::fs::create_dir_all(fixture.mount())?;
        std::fs::write(fixture.mount().join("keep"), "keep")?;
        assert!(
            !fixture
                .run(
                    CREATE_SCRIPT,
                    &["sample", "~/arcadia-worktrees", "custom/project"],
                    &[]
                )?
                .status
                .success()
        );
        assert!(!fixture.delete(&[])?.status.success());
        assert!(fixture.mount().join("keep").exists());
        std::fs::remove_file(fixture.mount().join("keep"))?;
        std::fs::remove_dir(fixture.mount())?;
        std::os::unix::fs::symlink(&fixture.home, fixture.mount())?;
        assert!(!fixture.delete(&[])?.status.success());
        assert!(fixture.home.exists());
        Ok(())
    }

    #[test]
    fn rejects_invalid_names_and_paths() {
        for name in [
            "",
            "..",
            "-sample",
            "sample/other",
            "sample;touch injected",
            "$(touch injected)",
        ] {
            assert!(validate_name(name).is_err(), "{name}");
        }
        assert!(validate_name("feature-123_clean.web").is_ok());
        for path in [
            "",
            "/absolute",
            "../escape",
            "project/../escape",
            "project//child",
            "project\nchild",
        ] {
            assert!(validate_relative_path(path).is_err(), "{path}");
        }
        assert!(validate_relative_path("arbitrary/project/type").is_ok());
    }

    #[gpui::test]
    async fn saved_metadata_survives_settings_reload_and_is_scoped_to_host(
        cx: &mut gpui::TestAppContext,
    ) {
        let app_state = cx.update(crate::AppState::test);
        let server = SshConnection {
            host: "arc-host".into(),
            arc_workspaces: Some(ArcWorkspacesConfig {
                mount_root: "~/arcadia-worktrees".into(),
                types: BTreeMap::from([("custom".into(), "custom/project".into())]),
            }),
            ..Default::default()
        };
        let metadata = ArcWorkspace {
            name: "sample".into(),
            workspace_type: "custom".into(),
            mount_path: "/home/remote/arcadia-worktrees/sample".into(),
            project_path: "/home/remote/arcadia-worktrees/sample/custom/project".into(),
        };
        let options = RemoteConnectionOptions::Ssh(server.clone().into());
        cx.update(|cx| save_workspace(server.clone(), metadata.clone(), app_state.fs.clone(), cx))
            .await
            .expect("settings completion")
            .expect("save Arc workspace");
        cx.run_until_parked();
        cx.update(|cx| {
            let config = config_for_connection(&options, cx).expect("Arc configuration");
            assert_eq!(
                config.types.get("custom").map(String::as_str),
                Some("custom/project")
            );
            assert_eq!(
                managed_workspace(&options, Path::new(&metadata.project_path), cx),
                Some(metadata.clone())
            );
            let other_host = RemoteConnectionOptions::Ssh(
                SshConnection {
                    host: "other-host".into(),
                    ..Default::default()
                }
                .into(),
            );
            assert!(
                managed_workspace(&other_host, Path::new(&metadata.project_path), cx).is_none()
            );
            let persisted =
                serde_json::to_string(&ssh_connection(&options, cx).expect("saved server"))
                    .expect("serialize");
            let restored: SshConnection = serde_json::from_str(&persisted).expect("deserialize");
            assert_eq!(
                restored
                    .projects
                    .iter()
                    .next()
                    .and_then(|project| project.arc_workspace.clone()),
                Some(metadata.clone())
            );
        });
        cx.update(|cx| save_workspace(server, metadata, app_state.fs.clone(), cx))
            .await
            .expect("settings completion")
            .expect("save Arc workspace");
        cx.run_until_parked();
        cx.update(|cx| {
            assert_eq!(
                ssh_connection(&options, cx)
                    .expect("saved server")
                    .projects
                    .len(),
                1
            )
        });
    }
}

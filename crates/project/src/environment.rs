use anyhow::{anyhow, Context as _, Result};
use futures::{future::Shared, FutureExt};
use paths::local_settings_file_relative_path;
use rpc::proto::{self, AnyProtoClient};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{parse_env_output, ResultExt};

use collections::HashMap;
use gpui::{AppContext, BorrowAppContext, Context, Model, ModelContext, Task};
use settings::{Settings as _, SettingsStore};
use worktree::{PathChange, UpdatedEntriesSet, Worktree, WorktreeId};

use crate::{
    project_settings::{DirenvSettings, ProjectSettings},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

pub struct ProjectEnvironment {
    project_id: u64,
    downstream_client: Option<AnyProtoClient>,
    cli_environment: Option<HashMap<String, String>>,
    get_environment_task: Option<Shared<Task<Option<HashMap<String, String>>>>>,
    cached_shell_environments: HashMap<WorktreeId, HashMap<String, String>>,
}

impl ProjectEnvironment {
    pub fn new(
        worktree_store: &Model<WorktreeStore>,
        downstream_client: Option<AnyProtoClient>,
        project_id: u64,
        cli_environment: Option<HashMap<String, String>>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx| {
            cx.subscribe(worktree_store, Self::on_worktree_store_event)
                .detach();
            Self {
                downstream_client,
                project_id,
                cli_environment,
                get_environment_task: None,
                cached_shell_environments: Default::default(),
            }
        })
    }

    pub fn shared(
        &mut self,
        project_id: u64,
        downstream_client: AnyProtoClient,
        _: &mut ModelContext<Self>,
    ) {
        self.project_id = project_id;
        self.downstream_client = Some(downstream_client);
    }

    pub fn unshared(&mut self, _: &mut ModelContext<Self>) {
        self.downstream_client = None;
    }

    fn on_worktree_store_event(
        &mut self,
        _: Model<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            crate::worktree_store::WorktreeStoreEvent::WorktreeAdded(worktree) => {
                if worktree.read(cx).is_local() {
                    cx.subscribe(worktree, Self::on_worktree_event).detach()
                }
            }
            crate::worktree_store::WorktreeStoreEvent::WorktreeRemoved(_, id) => {
                self.remove_worktree_environment(*id);
            }
            _ => {}
        }
    }

    fn on_worktree_event(
        &mut self,
        worktree: Model<Worktree>,
        event: &worktree::Event,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            worktree::Event::UpdatedEntries(changes) => {
                self.update_local_worktree_settings(&worktree, changes, cx)
            }
            _ => {}
        }
    }

    fn update_local_worktree_settings(
        &mut self,
        worktree: &Model<Worktree>,
        changes: &UpdatedEntriesSet,
        cx: &mut ModelContext<Self>,
    ) {
        let worktree_id = worktree.entity_id();
        let remote_worktree_id = worktree.read(cx).id();
        let Some(fs) = worktree.read(cx).as_local().map(|l| l.fs().clone()) else {
            return;
        };

        let mut settings_contents = Vec::new();
        for (path, _, change) in changes.iter() {
            let removed = change == &PathChange::Removed;
            let abs_path = match worktree.read(cx).absolutize(path) {
                Ok(abs_path) => abs_path,
                Err(e) => {
                    log::warn!("Cannot absolutize {path:?} received as {change:?} FS change: {e}");
                    continue;
                }
            };

            if path.ends_with(local_settings_file_relative_path()) {
                let settings_dir = Arc::from(
                    path.ancestors()
                        .nth(local_settings_file_relative_path().components().count())
                        .unwrap(),
                );
                let fs = fs.clone();
                settings_contents.push(async move {
                    (
                        settings_dir,
                        if removed {
                            None
                        } else {
                            Some(async move { fs.load(&abs_path).await }.await)
                        },
                    )
                });
            }
        }

        if settings_contents.is_empty() {
            return;
        }

        let project_id = self.project_id;
        let downstream_client = self.downstream_client.clone();
        cx.spawn(move |_, cx| async move {
            let settings_contents: Vec<(Arc<Path>, _)> =
                futures::future::join_all(settings_contents).await;
            cx.update(|cx| {
                cx.update_global::<SettingsStore, _>(|store, cx| {
                    for (directory, file_content) in settings_contents {
                        let file_content = file_content.and_then(|content| content.log_err());
                        store
                            .set_local_settings(
                                worktree_id.as_u64() as usize,
                                directory.clone(),
                                file_content.as_deref(),
                                cx,
                            )
                            .log_err();
                        if let Some(downstream_client) = &downstream_client {
                            downstream_client
                                .send(proto::UpdateWorktreeSettings {
                                    project_id,
                                    worktree_id: remote_worktree_id.to_proto(),
                                    path: directory.to_string_lossy().into_owned(),
                                    content: file_content,
                                })
                                .log_err();
                        }
                    }
                });
            })
            .ok();
        })
        .detach();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn set_cached(
        &mut self,
        shell_environments: &[(WorktreeId, HashMap<String, String>)],
    ) {
        self.cached_shell_environments = shell_environments
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
    }

    pub(crate) fn remove_worktree_environment(&mut self, worktree_id: WorktreeId) {
        self.cached_shell_environments.remove(&worktree_id);
    }

    /// Returns the inherited CLI environment, if this project was opened from the Zed CLI.
    pub(crate) fn get_cli_environment(&self) -> Option<HashMap<String, String>> {
        if let Some(mut env) = self.cli_environment.clone() {
            set_origin_marker(&mut env, EnvironmentOrigin::Cli);
            Some(env)
        } else {
            None
        }
    }

    /// Returns the project environment, if possible.
    /// If the project was opened from the CLI, then the inherited CLI environment is returned.
    /// If it wasn't opened from the CLI, and a worktree is given, then a shell is spawned in
    /// the worktree's path, to get environment variables as if the user has `cd`'d into
    /// the worktrees path.
    pub(crate) fn get_environment(
        &mut self,
        worktree_id: Option<WorktreeId>,
        worktree_abs_path: Option<Arc<Path>>,
        cx: &ModelContext<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        if let Some(task) = self.get_environment_task.as_ref() {
            task.clone()
        } else {
            let task = self
                .build_environment_task(worktree_id, worktree_abs_path, cx)
                .shared();

            self.get_environment_task = Some(task.clone());
            task
        }
    }

    fn build_environment_task(
        &mut self,
        worktree_id: Option<WorktreeId>,
        worktree_abs_path: Option<Arc<Path>>,
        cx: &ModelContext<Self>,
    ) -> Task<Option<HashMap<String, String>>> {
        let worktree = worktree_id.zip(worktree_abs_path);

        let cli_environment = self.get_cli_environment();
        if cli_environment.is_some() {
            Task::ready(cli_environment)
        } else if let Some((worktree_id, worktree_abs_path)) = worktree {
            self.get_worktree_env(worktree_id, worktree_abs_path, cx)
        } else {
            Task::ready(None)
        }
    }

    fn get_worktree_env(
        &mut self,
        worktree_id: WorktreeId,
        worktree_abs_path: Arc<Path>,
        cx: &ModelContext<Self>,
    ) -> Task<Option<HashMap<String, String>>> {
        let cached_env = self.cached_shell_environments.get(&worktree_id).cloned();
        if let Some(env) = cached_env {
            Task::ready(Some(env))
        } else {
            let load_direnv = ProjectSettings::get_global(cx).load_direnv.clone();

            cx.spawn(|this, mut cx| async move {
                let mut shell_env = cx
                    .background_executor()
                    .spawn({
                        let cwd = worktree_abs_path.clone();
                        async move { load_shell_environment(&cwd, &load_direnv).await }
                    })
                    .await
                    .ok();

                if let Some(shell_env) = shell_env.as_mut() {
                    this.update(&mut cx, |this, _| {
                        this.cached_shell_environments
                            .insert(worktree_id, shell_env.clone())
                    })
                    .log_err();

                    set_origin_marker(shell_env, EnvironmentOrigin::WorktreeShell);
                }

                shell_env
            })
        }
    }
}

fn set_origin_marker(env: &mut HashMap<String, String>, origin: EnvironmentOrigin) {
    env.insert(ZED_ENVIRONMENT_ORIGIN_MARKER.to_string(), origin.into());
}

const ZED_ENVIRONMENT_ORIGIN_MARKER: &str = "ZED_ENVIRONMENT";

enum EnvironmentOrigin {
    Cli,
    WorktreeShell,
}

impl Into<String> for EnvironmentOrigin {
    fn into(self) -> String {
        match self {
            EnvironmentOrigin::Cli => "cli".into(),
            EnvironmentOrigin::WorktreeShell => "worktree-shell".into(),
        }
    }
}

async fn load_shell_environment(
    dir: &Path,
    load_direnv: &DirenvSettings,
) -> Result<HashMap<String, String>> {
    let direnv_environment = match load_direnv {
        DirenvSettings::ShellHook => None,
        DirenvSettings::Direct => load_direnv_environment(dir).await?,
    }
    .unwrap_or(HashMap::default());

    let marker = "ZED_SHELL_START";
    let shell = std::env::var("SHELL").context(
        "SHELL environment variable is not assigned so we can't source login environment variables",
    )?;

    // What we're doing here is to spawn a shell and then `cd` into
    // the project directory to get the env in there as if the user
    // `cd`'d into it. We do that because tools like direnv, asdf, ...
    // hook into `cd` and only set up the env after that.
    //
    // If the user selects `Direct` for direnv, it would set an environment
    // variable that later uses to know that it should not run the hook.
    // We would include in `.envs` call so it is okay to run the hook
    // even if direnv direct mode is enabled.
    //
    // In certain shells we need to execute additional_command in order to
    // trigger the behavior of direnv, etc.
    //
    //
    // The `exit 0` is the result of hours of debugging, trying to find out
    // why running this command here, without `exit 0`, would mess
    // up signal process for our process so that `ctrl-c` doesn't work
    // anymore.
    //
    // We still don't know why `$SHELL -l -i -c '/usr/bin/env -0'`  would
    // do that, but it does, and `exit 0` helps.
    let additional_command = PathBuf::from(&shell)
        .file_name()
        .and_then(|f| f.to_str())
        .and_then(|shell| match shell {
            "fish" => Some("emit fish_prompt;"),
            _ => None,
        });

    let command = format!(
        "cd '{}';{} printf '%s' {marker}; /usr/bin/env; exit 0;",
        dir.display(),
        additional_command.unwrap_or("")
    );

    let output = smol::process::Command::new(&shell)
        .args(["-i", "-c", &command])
        .envs(direnv_environment)
        .output()
        .await
        .context("failed to spawn login shell to source login environment variables")?;

    anyhow::ensure!(
        output.status.success(),
        "login shell exited with error {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let env_output_start = stdout.find(marker).ok_or_else(|| {
        anyhow!(
            "failed to parse output of `env` command in login shell: {}",
            stdout
        )
    })?;

    let mut parsed_env = HashMap::default();
    let env_output = &stdout[env_output_start + marker.len()..];

    parse_env_output(env_output, |key, value| {
        parsed_env.insert(key, value);
    });

    Ok(parsed_env)
}

async fn load_direnv_environment(dir: &Path) -> Result<Option<HashMap<String, String>>> {
    let Ok(direnv_path) = which::which("direnv") else {
        return Ok(None);
    };

    let direnv_output = smol::process::Command::new(direnv_path)
        .args(["export", "json"])
        .current_dir(dir)
        .output()
        .await
        .context("failed to spawn direnv to get local environment variables")?;

    anyhow::ensure!(
        direnv_output.status.success(),
        "direnv exited with error {:?}",
        direnv_output.status
    );

    let output = String::from_utf8_lossy(&direnv_output.stdout);
    if output.is_empty() {
        return Ok(None);
    }

    Ok(Some(
        serde_json::from_str(&output).context("failed to parse direnv output")?,
    ))
}

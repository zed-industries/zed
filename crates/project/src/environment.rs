use futures::{future::Shared, FutureExt};
use std::{path::Path, sync::Arc};
use util::ResultExt;

use collections::HashMap;
use gpui::{AppContext, Context, Model, ModelContext, Task};
use settings::Settings as _;
use worktree::WorktreeId;

use crate::{
    project_settings::{DirenvSettings, ProjectSettings},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

pub struct ProjectEnvironment {
    cli_environment: Option<HashMap<String, String>>,
    get_environment_task: Option<Shared<Task<Option<HashMap<String, String>>>>>,
    cached_shell_environments: HashMap<WorktreeId, HashMap<String, String>>,
    environment_error_messages: HashMap<WorktreeId, EnvironmentErrorMessage>,
}

impl ProjectEnvironment {
    pub fn new(
        worktree_store: &Model<WorktreeStore>,
        cli_environment: Option<HashMap<String, String>>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx| {
            cx.subscribe(worktree_store, |this: &mut Self, _, event, _| {
                if let WorktreeStoreEvent::WorktreeRemoved(_, id) = event {
                    this.remove_worktree_environment(*id);
                }
            })
            .detach();

            Self {
                cli_environment,
                get_environment_task: None,
                cached_shell_environments: Default::default(),
                environment_error_messages: Default::default(),
            }
        })
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
        self.environment_error_messages.remove(&worktree_id);
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

    /// Returns an iterator over all pairs `(worktree_id, error_message)` of
    /// environment errors associated with this project environment.
    pub(crate) fn environment_errors(
        &self,
    ) -> impl Iterator<Item = (&WorktreeId, &EnvironmentErrorMessage)> {
        self.environment_error_messages.iter()
    }

    pub(crate) fn remove_environment_error(&mut self, worktree_id: WorktreeId) {
        self.environment_error_messages.remove(&worktree_id);
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
        if let Some(environment) = cli_environment {
            cx.spawn(|_, _| async move {
                let path = environment
                    .get("PATH")
                    .map(|path| path.as_str())
                    .unwrap_or_default();
                log::info!(
                    "using project environment variables from CLI. PATH={:?}",
                    path
                );
                Some(environment)
            })
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
                let (mut shell_env, error_message) = cx
                    .background_executor()
                    .spawn({
                        let cwd = worktree_abs_path.clone();
                        async move { load_shell_environment(&cwd, &load_direnv).await }
                    })
                    .await;

                if let Some(shell_env) = shell_env.as_mut() {
                    let path = shell_env
                        .get("PATH")
                        .map(|path| path.as_str())
                        .unwrap_or_default();
                    log::info!(
                        "using project environment variables shell launched in {:?}. PATH={:?}",
                        worktree_abs_path,
                        path
                    );
                    this.update(&mut cx, |this, _| {
                        this.cached_shell_environments
                            .insert(worktree_id, shell_env.clone());
                    })
                    .log_err();

                    set_origin_marker(shell_env, EnvironmentOrigin::WorktreeShell);
                }

                if let Some(error) = error_message {
                    this.update(&mut cx, |this, _| {
                        this.environment_error_messages.insert(worktree_id, error);
                    })
                    .log_err();
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

impl From<EnvironmentOrigin> for String {
    fn from(val: EnvironmentOrigin) -> Self {
        match val {
            EnvironmentOrigin::Cli => "cli".into(),
            EnvironmentOrigin::WorktreeShell => "worktree-shell".into(),
        }
    }
}

pub struct EnvironmentErrorMessage(pub String);

impl EnvironmentErrorMessage {
    #[allow(dead_code)]
    fn from_str(s: &str) -> Self {
        Self(String::from(s))
    }
}

#[cfg(any(test, feature = "test-support"))]
async fn load_shell_environment(
    _dir: &Path,
    _load_direnv: &DirenvSettings,
) -> (
    Option<HashMap<String, String>>,
    Option<EnvironmentErrorMessage>,
) {
    let fake_env = [("ZED_FAKE_TEST_ENV".into(), "true".into())]
        .into_iter()
        .collect();
    (Some(fake_env), None)
}

#[cfg(all(target_os = "windows", not(any(test, feature = "test-support"))))]
async fn load_shell_environment(
    _dir: &Path,
    _load_direnv: &DirenvSettings,
) -> (
    Option<HashMap<String, String>>,
    Option<EnvironmentErrorMessage>,
) {
    // TODO the current code works with Unix $SHELL only, implement environment loading on windows
    (None, None)
}

#[cfg(not(any(target_os = "windows", test, feature = "test-support")))]
async fn load_shell_environment(
    dir: &Path,
    load_direnv: &DirenvSettings,
) -> (
    Option<HashMap<String, String>>,
    Option<EnvironmentErrorMessage>,
) {
    use crate::direnv::{load_direnv_environment, DirenvError};
    use std::path::PathBuf;
    use util::parse_env_output;

    fn message<T>(with: &str) -> (Option<T>, Option<EnvironmentErrorMessage>) {
        let message = EnvironmentErrorMessage::from_str(with);
        (None, Some(message))
    }

    let marker = "ZED_SHELL_START";
    let Some(shell) = std::env::var("SHELL").log_err() else {
        return message("Failed to get login environment. SHELL environment variable is not set");
    };

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

    let Some(output) = smol::process::Command::new(&shell)
        .args(["-l", "-i", "-c", &command])
        .output()
        .await
        .log_err()
    else {
        return message("Failed to spawn login shell to source login environment variables. See logs for details");
    };

    if !output.status.success() {
        log::error!("login shell exited with {}", output.status);
        return message("Login shell exited with nonzero exit code. See logs for details");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(env_output_start) = stdout.find(marker) else {
        log::error!("failed to parse output of `env` command in login shell: {stdout}");
        return message("Failed to parse stdout of env command. See logs for the output");
    };

    let mut parsed_env = HashMap::default();
    let env_output = &stdout[env_output_start + marker.len()..];

    parse_env_output(env_output, |key, value| {
        parsed_env.insert(key, value);
    });

    let (direnv_environment, direnv_error) = match load_direnv {
        DirenvSettings::ShellHook => (None, None),
        DirenvSettings::Direct => match load_direnv_environment(&parsed_env, dir).await {
            Ok(env) => (Some(env), None),
            Err(err) => (
                None,
                <Option<EnvironmentErrorMessage> as From<DirenvError>>::from(err),
            ),
        },
    };

    for (key, value) in direnv_environment.unwrap_or(HashMap::default()) {
        parsed_env.insert(key, value);
    }

    (Some(parsed_env), direnv_error)
}

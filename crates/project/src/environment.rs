use futures::{FutureExt, future::Shared};
use language::Buffer;
use remote::RemoteClient;
use rpc::proto::{self, REMOTE_SERVER_PROJECT_ID};
use std::{collections::VecDeque, path::Path, sync::Arc};
use task::Shell;
use util::ResultExt;
use worktree::Worktree;

use collections::HashMap;
use gpui::{AppContext as _, Context, Entity, EventEmitter, Task};
use settings::Settings as _;

use crate::{
    project_settings::{DirenvSettings, ProjectSettings},
    worktree_store::WorktreeStore,
};

pub struct ProjectEnvironment {
    cli_environment: Option<HashMap<String, String>>,
    local_environments: HashMap<(Shell, Arc<Path>), Shared<Task<Option<HashMap<String, String>>>>>,
    remote_environments: HashMap<(Shell, Arc<Path>), Shared<Task<Option<HashMap<String, String>>>>>,
    environment_error_messages: VecDeque<EnvironmentErrorMessage>,
}

pub enum ProjectEnvironmentEvent {
    ErrorsUpdated,
}

impl EventEmitter<ProjectEnvironmentEvent> for ProjectEnvironment {}

impl ProjectEnvironment {
    pub fn new(cli_environment: Option<HashMap<String, String>>) -> Self {
        Self {
            cli_environment,
            local_environments: Default::default(),
            remote_environments: Default::default(),
            environment_error_messages: Default::default(),
        }
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

    pub(crate) fn get_buffer_environment(
        &mut self,
        buffer: &Entity<Buffer>,
        worktree_store: &Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        if cfg!(any(test, feature = "test-support")) {
            return Task::ready(Some(HashMap::default())).shared();
        }

        if let Some(cli_environment) = self.get_cli_environment() {
            log::debug!("using project environment variables from CLI");
            return Task::ready(Some(cli_environment)).shared();
        }

        let Some(worktree) = buffer
            .read(cx)
            .file()
            .map(|f| f.worktree_id(cx))
            .and_then(|worktree_id| worktree_store.read(cx).worktree_for_id(worktree_id, cx))
        else {
            return Task::ready(None).shared();
        };

        self.get_worktree_environment(worktree, cx)
    }

    pub fn get_worktree_environment(
        &mut self,
        worktree: Entity<Worktree>,
        cx: &mut Context<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        if cfg!(any(test, feature = "test-support")) {
            return Task::ready(Some(HashMap::default())).shared();
        }

        if let Some(cli_environment) = self.get_cli_environment() {
            log::debug!("using project environment variables from CLI");
            return Task::ready(Some(cli_environment)).shared();
        }

        let mut abs_path = worktree.read(cx).abs_path();
        if !worktree.read(cx).is_local() {
            log::error!(
                "attempted to get project environment for a non-local worktree at {abs_path:?}"
            );
            return Task::ready(None).shared();
        } else if worktree.read(cx).is_single_file() {
            let Some(parent) = abs_path.parent() else {
                return Task::ready(None).shared();
            };
            abs_path = parent.into();
        }

        self.get_local_directory_environment(&Shell::System, abs_path, cx)
    }

    /// Returns the project environment, if possible.
    /// If the project was opened from the CLI, then the inherited CLI environment is returned.
    /// If it wasn't opened from the CLI, and an absolute path is given, then a shell is spawned in
    /// that directory, to get environment variables as if the user has `cd`'d there.
    pub fn get_local_directory_environment(
        &mut self,
        shell: &Shell,
        abs_path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        if cfg!(any(test, feature = "test-support")) {
            return Task::ready(Some(HashMap::default())).shared();
        }

        if let Some(cli_environment) = self.get_cli_environment() {
            log::debug!("using project environment variables from CLI");
            return Task::ready(Some(cli_environment)).shared();
        }

        self.local_environments
            .entry((shell.clone(), abs_path.clone()))
            .or_insert_with(|| {
                get_local_directory_environment_impl(shell, abs_path.clone(), cx).shared()
            })
            .clone()
    }

    pub fn get_remote_directory_environment(
        &mut self,
        shell: &Shell,
        abs_path: Arc<Path>,
        remote_client: Entity<RemoteClient>,
        cx: &mut Context<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        if cfg!(any(test, feature = "test-support")) {
            return Task::ready(Some(HashMap::default())).shared();
        }

        self.remote_environments
            .entry((shell.clone(), abs_path.clone()))
            .or_insert_with(|| {
                let response =
                    remote_client
                        .read(cx)
                        .proto_client()
                        .request(proto::GetDirectoryEnvironment {
                            project_id: REMOTE_SERVER_PROJECT_ID,
                            shell: Some(shell.clone().to_proto()),
                            directory: abs_path.to_string_lossy().to_string(),
                        });
                cx.spawn(async move |_, _| {
                    let environment = response.await.log_err()?;
                    Some(environment.environment.into_iter().collect())
                })
                .shared()
            })
            .clone()
    }

    pub fn peek_environment_error(&self) -> Option<&EnvironmentErrorMessage> {
        self.environment_error_messages.front()
    }

    pub fn pop_environment_error(&mut self) -> Option<EnvironmentErrorMessage> {
        self.environment_error_messages.pop_front()
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

#[derive(Debug)]
pub struct EnvironmentErrorMessage(pub String);

impl std::fmt::Display for EnvironmentErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl EnvironmentErrorMessage {
    #[allow(dead_code)]
    fn from_str(s: &str) -> Self {
        Self(String::from(s))
    }
}

async fn load_directory_shell_environment(
    shell: &Shell,
    abs_path: &Path,
    load_direnv: &DirenvSettings,
) -> (
    Option<HashMap<String, String>>,
    Option<EnvironmentErrorMessage>,
) {
    match smol::fs::metadata(abs_path).await {
        Ok(meta) => {
            let dir = if meta.is_dir() {
                abs_path
            } else if let Some(parent) = abs_path.parent() {
                parent
            } else {
                return (
                    None,
                    Some(EnvironmentErrorMessage(format!(
                        "Failed to load shell environment in {}: not a directory",
                        abs_path.display()
                    ))),
                );
            };

            load_shell_environment(shell, dir, load_direnv).await
        }
        Err(err) => (
            None,
            Some(EnvironmentErrorMessage(format!(
                "Failed to load shell environment in {}: {}",
                abs_path.display(),
                err
            ))),
        ),
    }
}

async fn load_shell_environment(
    shell: &Shell,
    dir: &Path,
    load_direnv: &DirenvSettings,
) -> (
    Option<HashMap<String, String>>,
    Option<EnvironmentErrorMessage>,
) {
    use crate::direnv::load_direnv_environment;
    use util::shell_env;

    if cfg!(any(test, feature = "test-support")) {
        let fake_env = [("ZED_FAKE_TEST_ENV".into(), "true".into())]
            .into_iter()
            .collect();
        (Some(fake_env), None)
    } else if cfg!(target_os = "windows",) {
        let (shell, args) = shell.program_and_args();
        let envs = match shell_env::capture(shell, args, dir).await {
            Ok(envs) => envs,
            Err(err) => {
                util::log_err(&err);
                return (
                    None,
                    Some(EnvironmentErrorMessage(format!(
                        "Failed to load environment variables: {}",
                        err
                    ))),
                );
            }
        };

        // Note: direnv is not available on Windows, so we skip direnv processing
        // and just return the shell environment
        (Some(envs), None)
    } else {
        let dir_ = dir.to_owned();
        let (shell, args) = shell.program_and_args();
        let mut envs = match shell_env::capture(shell, args, &dir_).await {
            Ok(envs) => envs,
            Err(err) => {
                util::log_err(&err);
                return (
                    None,
                    Some(EnvironmentErrorMessage::from_str(
                        "Failed to load environment variables. See log for details",
                    )),
                );
            }
        };

        // If the user selects `Direct` for direnv, it would set an environment
        // variable that later uses to know that it should not run the hook.
        // We would include in `.envs` call so it is okay to run the hook
        // even if direnv direct mode is enabled.
        let (direnv_environment, direnv_error) = match load_direnv {
            DirenvSettings::ShellHook => (None, None),
            DirenvSettings::Direct => match load_direnv_environment(&envs, dir).await {
                Ok(env) => (Some(env), None),
                Err(err) => (None, err.into()),
            },
        };
        if let Some(direnv_environment) = direnv_environment {
            for (key, value) in direnv_environment {
                if let Some(value) = value {
                    envs.insert(key, value);
                } else {
                    envs.remove(&key);
                }
            }
        }

        (Some(envs), direnv_error)
    }
}

fn get_local_directory_environment_impl(
    shell: &Shell,
    abs_path: Arc<Path>,
    cx: &Context<ProjectEnvironment>,
) -> Task<Option<HashMap<String, String>>> {
    let load_direnv = ProjectSettings::get_global(cx).load_direnv.clone();

    let shell = shell.clone();
    cx.spawn(async move |this, cx| {
        let (mut shell_env, error_message) = cx
            .background_spawn({
                let abs_path = abs_path.clone();
                async move {
                    load_directory_shell_environment(&shell, &abs_path, &load_direnv).await
                }
            })
            .await;

        if let Some(shell_env) = shell_env.as_mut() {
            let path = shell_env
                .get("PATH")
                .map(|path| path.as_str())
                .unwrap_or_default();
            log::info!(
                "using project environment variables shell launched in {:?}. PATH={:?}",
                abs_path,
                path
            );

            set_origin_marker(shell_env, EnvironmentOrigin::WorktreeShell);
        }

        if let Some(error) = error_message {
            this.update(cx, |this, cx| {
                log::error!("{error}");
                this.environment_error_messages.push_back(error);
                cx.emit(ProjectEnvironmentEvent::ErrorsUpdated)
            })
            .log_err();
        }

        shell_env
    })
}

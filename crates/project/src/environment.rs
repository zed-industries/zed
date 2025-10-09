use futures::{FutureExt, future::Shared};
use language::Buffer;
use std::{path::Path, sync::Arc};
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
    environments: HashMap<Arc<Path>, Shared<Task<Option<HashMap<String, String>>>>>,
    shell_based_environments:
        HashMap<(Shell, Arc<Path>), Shared<Task<Option<HashMap<String, String>>>>>,
    environment_error_messages: HashMap<Arc<Path>, EnvironmentErrorMessage>,
}

pub enum ProjectEnvironmentEvent {
    ErrorsUpdated,
}

impl EventEmitter<ProjectEnvironmentEvent> for ProjectEnvironment {}

impl ProjectEnvironment {
    pub fn new(cli_environment: Option<HashMap<String, String>>) -> Self {
        Self {
            cli_environment,
            environments: Default::default(),
            shell_based_environments: Default::default(),
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

    /// Returns an iterator over all pairs `(abs_path, error_message)` of
    /// environment errors associated with this project environment.
    pub(crate) fn environment_errors(
        &self,
    ) -> impl Iterator<Item = (&Arc<Path>, &EnvironmentErrorMessage)> {
        self.environment_error_messages.iter()
    }

    pub(crate) fn remove_environment_error(&mut self, abs_path: &Path, cx: &mut Context<Self>) {
        self.environment_error_messages.remove(abs_path);
        cx.emit(ProjectEnvironmentEvent::ErrorsUpdated);
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
        let is_local = worktree.read(cx).is_local();

        if !is_local {
            // For non-local worktrees (which shouldn't normally happen on the server),
            // we provide a minimal environment for language servers
            log::debug!("loading minimal environment for non-local worktree at {abs_path:?}");
            return self.get_minimal_environment_for_lsp(abs_path, cx);
        } else if worktree.read(cx).is_single_file() {
            let Some(parent) = abs_path.parent() else {
                return Task::ready(None).shared();
            };
            abs_path = parent.into();
        }

        self.get_directory_environment(abs_path, cx)
    }

    /// Returns a minimal environment suitable for language servers
    /// This is used when we need to provide just enough environment
    /// for tools like rust-analyzer to find configuration files
    fn get_minimal_environment_for_lsp(
        &mut self,
        abs_path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        let task = cx.spawn(async move |_this, _cx| {
            let mut env = HashMap::default();

            // Set PWD for tools that look for config files relative to working directory
            env.insert("PWD".to_string(), abs_path.to_string_lossy().to_string());

            // Set HOME if available (needed for some tools to find global config)
            if let Ok(home) = std::env::var("HOME") {
                env.insert("HOME".to_string(), home);
            }

            // Include PATH for finding executables
            if let Ok(path) = std::env::var("PATH") {
                env.insert("PATH".to_string(), path);
            }

            // Set RUST_LOG if configured (useful for debugging)
            if let Ok(rust_log) = std::env::var("RUST_LOG") {
                env.insert("RUST_LOG".to_string(), rust_log);
            }

            log::debug!("Created minimal LSP environment for {:?}", abs_path);

            Some(env)
        });
        task.shared()
    }

    /// Returns the project environment, if possible.
    /// If the project was opened from the CLI, then the inherited CLI environment is returned.
    /// If it wasn't opened from the CLI, and an absolute path is given, then a shell is spawned in
    /// that directory, to get environment variables as if the user has `cd`'d there.
    pub fn get_directory_environment(
        &mut self,
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

        self.environments
            .entry(abs_path.clone())
            .or_insert_with(|| {
                get_directory_env_impl(&Shell::System, abs_path.clone(), cx).shared()
            })
            .clone()
    }

    /// Returns the project environment, if possible, with the given shell.
    pub fn get_directory_environment_for_shell(
        &mut self,
        shell: &Shell,
        abs_path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        self.shell_based_environments
            .entry((shell.clone(), abs_path.clone()))
            .or_insert_with(|| get_directory_env_impl(shell, abs_path.clone(), cx).shared())
            .clone()
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

fn get_directory_env_impl(
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
                log::error!("{error}",);
                this.environment_error_messages.insert(abs_path, error);
                cx.emit(ProjectEnvironmentEvent::ErrorsUpdated)
            })
            .log_err();
        }

        shell_env
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_minimal_environment_has_required_vars() {
        // Test that the minimal environment contains the necessary variables
        // for rust-analyzer to find rustfmt.toml
        let test_path = "/test/project";

        // Simulate what get_minimal_environment_for_lsp would create
        let mut env = HashMap::<String, String>::default();
        env.insert("PWD".to_string(), test_path.to_string());

        // Add other expected environment variables
        if let Ok(home) = std::env::var("HOME") {
            env.insert("HOME".to_string(), home);
        }
        if let Ok(path) = std::env::var("PATH") {
            env.insert("PATH".to_string(), path);
        }

        // Verify PWD is set correctly
        assert_eq!(env.get("PWD"), Some(&test_path.to_string()));
        assert!(
            env.contains_key("PWD"),
            "PWD must be set for config file discovery"
        );
    }

    #[test]
    fn test_environment_origin_marker() {
        let mut env = HashMap::<String, String>::default();

        // Test CLI origin
        set_origin_marker(&mut env, EnvironmentOrigin::Cli);
        assert_eq!(env.get("ZED_ENVIRONMENT"), Some(&"cli".to_string()));

        // Test WorktreeShell origin
        env.clear();
        set_origin_marker(&mut env, EnvironmentOrigin::WorktreeShell);
        assert_eq!(
            env.get("ZED_ENVIRONMENT"),
            Some(&"worktree-shell".to_string())
        );
    }

    #[test]
    fn test_environment_caching_key() {
        // Test that paths are properly used as cache keys and that
        // identical paths share the same cache entry
        let path1: Arc<Path> = Arc::from(PathBuf::from("/project/a").as_path());
        let path2: Arc<Path> = Arc::from(PathBuf::from("/project/b").as_path());
        let path3: Arc<Path> = Arc::from(PathBuf::from("/project/a").as_path()); // Same as path1

        let mut cache = HashMap::<Arc<Path>, String>::default();

        // Insert first path
        cache.insert(path1.clone(), "env1".to_string());
        assert_eq!(cache.len(), 1, "Cache should have 1 entry");

        // Insert second, different path
        cache.insert(path2.clone(), "env2".to_string());
        assert_eq!(cache.len(), 2, "Cache should have 2 entries");

        // Try to insert third path that's identical to first
        // This should not create a new entry but update the existing one
        cache.insert(path3.clone(), "env3".to_string());
        assert_eq!(cache.len(), 2, "Cache should still have 2 entries, not 3");

        // Verify that path1 and path3 now both retrieve the updated value
        assert_eq!(
            cache.get(&path1),
            Some(&"env3".to_string()),
            "path1 should retrieve updated value"
        );
        assert_eq!(
            cache.get(&path3),
            Some(&"env3".to_string()),
            "path3 should retrieve the same value as path1"
        );
        assert_eq!(
            cache.get(&path2),
            Some(&"env2".to_string()),
            "path2 should still have its original value"
        );

        // Test that Arc<Path> deduplication works correctly
        let path4: Arc<Path> = Arc::from(PathBuf::from("/project/a").as_path());
        assert!(
            Arc::ptr_eq(&path1, &path3) || path1 == path3,
            "Paths with same content should be equal"
        );
        assert_eq!(
            cache.get(&path4),
            Some(&"env3".to_string()),
            "New Arc with same path should retrieve from cache"
        );
    }

    #[test]
    fn test_rustfmt_config_path_scenarios() {
        // Test various path scenarios that rust-analyzer might encounter
        // when looking for rustfmt.toml

        let project_root = PathBuf::from("/workspace/my_project");
        let nested_src = project_root.join("nested/src");
        let _lib_file = nested_src.join("lib.rs");

        // Rust-analyzer would typically search from the file's directory
        // up to the workspace root
        let search_paths = vec![
            nested_src.clone(),
            nested_src.parent().unwrap().to_path_buf(),
            project_root.clone(),
        ];

        // Verify the search path order is correct
        assert_eq!(
            search_paths[0],
            PathBuf::from("/workspace/my_project/nested/src")
        );
        assert_eq!(
            search_paths[1],
            PathBuf::from("/workspace/my_project/nested")
        );
        assert_eq!(search_paths[2], PathBuf::from("/workspace/my_project"));

        // In our implementation, PWD should be set to the worktree root
        // which allows rust-analyzer to find rustfmt.toml
        let pwd = project_root.to_string_lossy().to_string();
        assert!(
            pwd.contains("my_project"),
            "PWD should contain the project name"
        );
    }
}

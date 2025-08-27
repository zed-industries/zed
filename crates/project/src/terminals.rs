use crate::{Project, ProjectPath};
use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity};
use language::LanguageName;
use remote::RemoteClient;
use settings::{Settings, SettingsLocation};
use smol::channel::bounded;
use std::{
    env::{self},
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{Shell, ShellBuilder, SpawnInTerminal};
use terminal::{
    TaskState, TaskStatus, Terminal, TerminalBuilder,
    terminal_settings::{self, ActivateScript, TerminalSettings, VenvSettings},
};
use util::{ResultExt, paths::RemotePathBuf};

/// The directory inside a Python virtual environment that contains executables
const PYTHON_VENV_BIN_DIR: &str = if cfg!(target_os = "windows") {
    "Scripts"
} else {
    "bin"
};

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakEntity<terminal::Terminal>>,
}

/// Terminals are opened either for the users shell, or to run a task.

#[derive(Debug)]
pub enum TerminalKind {
    /// Run a shell at the given path (or $HOME if None)
    Shell(Option<PathBuf>),
    /// Run a task.
    Task(SpawnInTerminal),
}

impl Project {
    pub fn active_project_directory(&self, cx: &App) -> Option<Arc<Path>> {
        self.active_entry()
            .and_then(|entry_id| self.worktree_for_entry(entry_id, cx))
            .into_iter()
            .chain(self.worktrees(cx))
            .find_map(|tree| tree.read(cx).root_dir())
    }

    pub fn first_project_directory(&self, cx: &App) -> Option<PathBuf> {
        let worktree = self.worktrees(cx).next()?;
        let worktree = worktree.read(cx);
        if worktree.root_entry()?.is_dir() {
            Some(worktree.abs_path().to_path_buf())
        } else {
            None
        }
    }

    pub fn create_terminal(
        &mut self,
        kind: TerminalKind,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        let path: Option<Arc<Path>> = match &kind {
            TerminalKind::Shell(path) => path.as_ref().map(|path| Arc::from(path.as_ref())),
            TerminalKind::Task(spawn_task) => {
                if let Some(cwd) = &spawn_task.cwd {
                    Some(Arc::from(cwd.as_ref()))
                } else {
                    self.active_project_directory(cx)
                }
            }
        };

        let mut settings_location = None;
        if let Some(path) = path.as_ref()
            && let Some((worktree, _)) = self.find_worktree(path, cx)
        {
            settings_location = Some(SettingsLocation {
                worktree_id: worktree.read(cx).id(),
                path,
            });
        }
        let venv = TerminalSettings::get(settings_location, cx)
            .detect_venv
            .clone();

        cx.spawn(async move |project, cx| {
            let python_venv_directory = if let Some(path) = path {
                project
                    .update(cx, |this, cx| this.python_venv_directory(path, venv, cx))?
                    .await
            } else {
                None
            };
            project.update(cx, |project, cx| {
                project.create_terminal_with_venv(kind, python_venv_directory, cx)
            })?
        })
    }

    pub fn terminal_settings<'a>(
        &'a self,
        path: &'a Option<PathBuf>,
        cx: &'a App,
    ) -> &'a TerminalSettings {
        let mut settings_location = None;
        if let Some(path) = path.as_ref()
            && let Some((worktree, _)) = self.find_worktree(path, cx)
        {
            settings_location = Some(SettingsLocation {
                worktree_id: worktree.read(cx).id(),
                path,
            });
        }
        TerminalSettings::get(settings_location, cx)
    }

    pub fn exec_in_shell(&self, command: String, cx: &App) -> Result<std::process::Command> {
        let path = self.first_project_directory(cx);
        let remote_client = self.remote_client.as_ref();
        let settings = self.terminal_settings(&path, cx).clone();
        let remote_shell = remote_client
            .as_ref()
            .and_then(|remote_client| remote_client.read(cx).shell());
        let builder = ShellBuilder::new(remote_shell.as_deref(), &settings.shell).non_interactive();
        let (command, args) = builder.build(Some(command), &Vec::new());

        let mut env = self
            .environment
            .read(cx)
            .get_cli_environment()
            .unwrap_or_default();
        env.extend(settings.env);

        match remote_client {
            Some(remote_client) => {
                let command_template =
                    remote_client
                        .read(cx)
                        .build_command(Some(command), &args, &env, None, None)?;
                let mut command = std::process::Command::new(command_template.program);
                command.args(command_template.args);
                command.envs(command_template.env);
                Ok(command)
            }
            None => {
                let mut command = std::process::Command::new(command);
                command.args(args);
                command.envs(env);
                if let Some(path) = path {
                    command.current_dir(path);
                }
                Ok(command)
            }
        }
    }

    pub fn create_terminal_with_venv(
        &mut self,
        kind: TerminalKind,
        python_venv_directory: Option<PathBuf>,
        cx: &mut Context<Self>,
    ) -> Result<Entity<Terminal>> {
        let is_via_remote = self.remote_client.is_some();

        let path: Option<Arc<Path>> = match &kind {
            TerminalKind::Shell(path) => path.as_ref().map(|path| Arc::from(path.as_ref())),
            TerminalKind::Task(spawn_task) => {
                if let Some(cwd) = &spawn_task.cwd {
                    if is_via_remote {
                        Some(Arc::from(cwd.as_ref()))
                    } else {
                        let cwd = cwd.to_string_lossy();
                        let tilde_substituted = shellexpand::tilde(&cwd);
                        Some(Arc::from(Path::new(tilde_substituted.as_ref())))
                    }
                } else {
                    self.active_project_directory(cx)
                }
            }
        };

        let mut settings_location = None;
        if let Some(path) = path.as_ref()
            && let Some((worktree, _)) = self.find_worktree(path, cx)
        {
            settings_location = Some(SettingsLocation {
                worktree_id: worktree.read(cx).id(),
                path,
            });
        }
        let settings = TerminalSettings::get(settings_location, cx).clone();

        let (completion_tx, completion_rx) = bounded(1);

        // Start with the environment that we might have inherited from the Zed CLI.
        let mut env = self
            .environment
            .read(cx)
            .get_cli_environment()
            .unwrap_or_default();
        // Then extend it with the explicit env variables from the settings, so they take
        // precedence.
        env.extend(settings.env);

        let local_path = if is_via_remote { None } else { path.clone() };

        let mut python_venv_activate_command = Task::ready(None);

        let remote_client = self.remote_client.clone();
        let spawn_task;
        let shell;
        match kind {
            TerminalKind::Shell(_) => {
                if let Some(python_venv_directory) = &python_venv_directory {
                    python_venv_activate_command = self.python_activate_command(
                        python_venv_directory,
                        &settings.detect_venv,
                        &settings.shell,
                        cx,
                    );
                }

                spawn_task = None;
                shell = match remote_client {
                    Some(remote_client) => {
                        create_remote_shell(None, &mut env, path, remote_client, cx)?
                    }
                    None => settings.shell,
                };
            }
            TerminalKind::Task(task) => {
                env.extend(task.env);

                if let Some(venv_path) = &python_venv_directory {
                    env.insert(
                        "VIRTUAL_ENV".to_string(),
                        venv_path.to_string_lossy().to_string(),
                    );
                }

                spawn_task = Some(TaskState {
                    id: task.id,
                    full_label: task.full_label,
                    label: task.label,
                    command_label: task.command_label,
                    hide: task.hide,
                    status: TaskStatus::Running,
                    show_summary: task.show_summary,
                    show_command: task.show_command,
                    show_rerun: task.show_rerun,
                    completion_rx,
                });
                shell = match remote_client {
                    Some(remote_client) => {
                        let path_style = remote_client.read(cx).path_style();
                        if let Some(venv_directory) = &python_venv_directory
                            && let Ok(str) =
                                shlex::try_quote(venv_directory.to_string_lossy().as_ref())
                        {
                            let path =
                                RemotePathBuf::new(PathBuf::from(str.to_string()), path_style)
                                    .to_string();
                            env.insert("PATH".into(), format!("{}:$PATH ", path));
                        }

                        create_remote_shell(
                            task.command.as_ref().map(|command| (command, &task.args)),
                            &mut env,
                            path,
                            remote_client,
                            cx,
                        )?
                    }
                    None => {
                        if let Some(venv_path) = &python_venv_directory {
                            add_environment_path(&mut env, &venv_path.join(PYTHON_VENV_BIN_DIR))
                                .log_err();
                        }

                        if let Some(program) = task.command {
                            Shell::WithArguments {
                                program,
                                args: task.args,
                                title_override: None,
                            }
                        } else {
                            Shell::System
                        }
                    }
                };
            }
        };
        TerminalBuilder::new(
            local_path.map(|path| path.to_path_buf()),
            python_venv_directory,
            spawn_task,
            shell,
            env,
            settings.cursor_shape.unwrap_or_default(),
            settings.alternate_scroll,
            settings.max_scroll_history_lines,
            is_via_remote,
            cx.entity_id().as_u64(),
            completion_tx,
            cx,
        )
        .map(|builder| {
            let terminal_handle = cx.new(|cx| builder.subscribe(cx));

            self.terminals
                .local_handles
                .push(terminal_handle.downgrade());

            let id = terminal_handle.entity_id();
            cx.observe_release(&terminal_handle, move |project, _terminal, cx| {
                let handles = &mut project.terminals.local_handles;

                if let Some(index) = handles
                    .iter()
                    .position(|terminal| terminal.entity_id() == id)
                {
                    handles.remove(index);
                    cx.notify();
                }
            })
            .detach();

            self.activate_python_virtual_environment(
                python_venv_activate_command,
                &terminal_handle,
                cx,
            );

            terminal_handle
        })
    }

    fn python_venv_directory(
        &self,
        abs_path: Arc<Path>,
        venv_settings: VenvSettings,
        cx: &Context<Project>,
    ) -> Task<Option<PathBuf>> {
        cx.spawn(async move |this, cx| {
            if let Some((worktree, relative_path)) = this
                .update(cx, |this, cx| this.find_worktree(&abs_path, cx))
                .ok()?
            {
                let toolchain = this
                    .update(cx, |this, cx| {
                        this.active_toolchain(
                            ProjectPath {
                                worktree_id: worktree.read(cx).id(),
                                path: relative_path.into(),
                            },
                            LanguageName::new("Python"),
                            cx,
                        )
                    })
                    .ok()?
                    .await;

                if let Some(toolchain) = toolchain {
                    let toolchain_path = Path::new(toolchain.path.as_ref());
                    return Some(toolchain_path.parent()?.parent()?.to_path_buf());
                }
            }
            let venv_settings = venv_settings.as_option()?;
            this.update(cx, move |this, cx| {
                if let Some(path) = this.find_venv_in_worktree(&abs_path, &venv_settings, cx) {
                    return Some(path);
                }
                this.find_venv_on_filesystem(&abs_path, &venv_settings, cx)
            })
            .ok()
            .flatten()
        })
    }

    fn find_venv_in_worktree(
        &self,
        abs_path: &Path,
        venv_settings: &terminal_settings::VenvSettingsContent,
        cx: &App,
    ) -> Option<PathBuf> {
        venv_settings
            .directories
            .iter()
            .map(|name| abs_path.join(name))
            .find(|venv_path| {
                let bin_path = venv_path.join(PYTHON_VENV_BIN_DIR);
                self.find_worktree(&bin_path, cx)
                    .and_then(|(worktree, relative_path)| {
                        worktree.read(cx).entry_for_path(&relative_path)
                    })
                    .is_some_and(|entry| entry.is_dir())
            })
    }

    fn find_venv_on_filesystem(
        &self,
        abs_path: &Path,
        venv_settings: &terminal_settings::VenvSettingsContent,
        cx: &App,
    ) -> Option<PathBuf> {
        let (worktree, _) = self.find_worktree(abs_path, cx)?;
        let fs = worktree.read(cx).as_local()?.fs();
        venv_settings
            .directories
            .iter()
            .map(|name| abs_path.join(name))
            .find(|venv_path| {
                let bin_path = venv_path.join(PYTHON_VENV_BIN_DIR);
                // One-time synchronous check is acceptable for terminal/task initialization
                smol::block_on(fs.metadata(&bin_path))
                    .ok()
                    .flatten()
                    .is_some_and(|meta| meta.is_dir)
            })
    }

    fn activate_script_kind(shell: Option<&str>) -> ActivateScript {
        let shell_env = std::env::var("SHELL").ok();
        let shell_path = shell.or_else(|| shell_env.as_deref());
        let shell = std::path::Path::new(shell_path.unwrap_or(""))
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        match shell {
            "fish" => ActivateScript::Fish,
            "tcsh" => ActivateScript::Csh,
            "nu" => ActivateScript::Nushell,
            "powershell" | "pwsh" => ActivateScript::PowerShell,
            _ => ActivateScript::Default,
        }
    }

    fn python_activate_command(
        &self,
        venv_base_directory: &Path,
        venv_settings: &VenvSettings,
        shell: &Shell,
        cx: &mut App,
    ) -> Task<Option<String>> {
        let Some(venv_settings) = venv_settings.as_option() else {
            return Task::ready(None);
        };
        let activate_keyword = match venv_settings.activate_script {
            terminal_settings::ActivateScript::Default => match std::env::consts::OS {
                "windows" => ".",
                _ => ".",
            },
            terminal_settings::ActivateScript::Nushell => "overlay use",
            terminal_settings::ActivateScript::PowerShell => ".",
            terminal_settings::ActivateScript::Pyenv => "pyenv",
            _ => "source",
        };
        let script_kind =
            if venv_settings.activate_script == terminal_settings::ActivateScript::Default {
                match shell {
                    Shell::Program(program) => Self::activate_script_kind(Some(program)),
                    Shell::WithArguments {
                        program,
                        args: _,
                        title_override: _,
                    } => Self::activate_script_kind(Some(program)),
                    Shell::System => Self::activate_script_kind(None),
                }
            } else {
                venv_settings.activate_script
            };

        let activate_script_name = match script_kind {
            terminal_settings::ActivateScript::Default
            | terminal_settings::ActivateScript::Pyenv => "activate",
            terminal_settings::ActivateScript::Csh => "activate.csh",
            terminal_settings::ActivateScript::Fish => "activate.fish",
            terminal_settings::ActivateScript::Nushell => "activate.nu",
            terminal_settings::ActivateScript::PowerShell => "activate.ps1",
        };

        let line_ending = match std::env::consts::OS {
            "windows" => "\r",
            _ => "\n",
        };

        if venv_settings.venv_name.is_empty() {
            let path = venv_base_directory
                .join(PYTHON_VENV_BIN_DIR)
                .join(activate_script_name)
                .to_string_lossy()
                .to_string();

            let is_valid_path = self.resolve_abs_path(path.as_ref(), cx);
            cx.background_spawn(async move {
                let quoted = shlex::try_quote(&path).ok()?;
                if is_valid_path.await.is_some_and(|meta| meta.is_file()) {
                    Some(format!(
                        "{} {} ; clear{}",
                        activate_keyword, quoted, line_ending
                    ))
                } else {
                    None
                }
            })
        } else {
            Task::ready(Some(format!(
                "{activate_keyword} {activate_script_name} {name}; clear{line_ending}",
                name = venv_settings.venv_name
            )))
        }
    }

    fn activate_python_virtual_environment(
        &self,
        command: Task<Option<String>>,
        terminal_handle: &Entity<Terminal>,
        cx: &mut App,
    ) {
        terminal_handle.update(cx, |_, cx| {
            cx.spawn(async move |this, cx| {
                if let Some(command) = command.await {
                    this.update(cx, |this, _| {
                        this.input(command.into_bytes());
                    })
                    .ok();
                }
            })
            .detach()
        });
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakEntity<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

fn create_remote_shell(
    spawn_command: Option<(&String, &Vec<String>)>,
    env: &mut HashMap<String, String>,
    working_directory: Option<Arc<Path>>,
    remote_client: Entity<RemoteClient>,
    cx: &mut App,
) -> Result<Shell> {
    // Alacritty sets its terminfo to `alacritty`, this requiring hosts to have it installed
    // to properly display colors.
    // We do not have the luxury of assuming the host has it installed,
    // so we set it to a default that does not break the highlighting via ssh.
    env.entry("TERM".to_string())
        .or_insert_with(|| "xterm-256color".to_string());

    let (program, args) = match spawn_command {
        Some((program, args)) => (Some(program.clone()), args),
        None => (None, &Vec::new()),
    };

    let command = remote_client.read(cx).build_command(
        program,
        args.as_slice(),
        env,
        working_directory.map(|path| path.display().to_string()),
        None,
    )?;
    *env = command.env;

    log::debug!("Connecting to a remote server: {:?}", command.program);
    let host = remote_client.read(cx).connection_options().host;

    Ok(Shell::WithArguments {
        program: command.program,
        args: command.args,
        title_override: Some(format!("{} — Terminal", host).into()),
    })
}

fn add_environment_path(env: &mut HashMap<String, String>, new_path: &Path) -> Result<()> {
    let mut env_paths = vec![new_path.to_path_buf()];
    if let Some(path) = env.get("PATH").or(env::var("PATH").ok().as_ref()) {
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        env_paths.append(&mut paths);
    }

    let paths = std::env::join_paths(env_paths).context("failed to create PATH env variable")?;
    env.insert("PATH".to_string(), paths.to_string_lossy().to_string());

    Ok(())
}

#[cfg(test)]
mod tests {
    use collections::HashMap;

    #[test]
    fn test_add_environment_path_with_existing_path() {
        let tmp_path = std::path::PathBuf::from("/tmp/new");
        let mut env = HashMap::default();
        let old_path = if cfg!(windows) {
            "/usr/bin;/usr/local/bin"
        } else {
            "/usr/bin:/usr/local/bin"
        };
        env.insert("PATH".to_string(), old_path.to_string());
        env.insert("OTHER".to_string(), "aaa".to_string());

        super::add_environment_path(&mut env, &tmp_path).unwrap();
        if cfg!(windows) {
            assert_eq!(env.get("PATH").unwrap(), &format!("/tmp/new;{}", old_path));
        } else {
            assert_eq!(env.get("PATH").unwrap(), &format!("/tmp/new:{}", old_path));
        }
        assert_eq!(env.get("OTHER").unwrap(), "aaa");
    }

    #[test]
    fn test_add_environment_path_with_empty_path() {
        let tmp_path = std::path::PathBuf::from("/tmp/new");
        let mut env = HashMap::default();
        env.insert("OTHER".to_string(), "aaa".to_string());
        let os_path = std::env::var("PATH").unwrap();
        super::add_environment_path(&mut env, &tmp_path).unwrap();
        if cfg!(windows) {
            assert_eq!(env.get("PATH").unwrap(), &format!("/tmp/new;{}", os_path));
        } else {
            assert_eq!(env.get("PATH").unwrap(), &format!("/tmp/new:{}", os_path));
        }
        assert_eq!(env.get("OTHER").unwrap(), "aaa");
    }
}

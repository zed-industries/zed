use crate::{Project, ProjectPath};
use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{AnyWindowHandle, App, AppContext as _, Context, Entity, Task, WeakEntity};
use itertools::Itertools;
use language::LanguageName;
use settings::{Settings, SettingsLocation};
use smol::channel::bounded;
use std::{
    borrow::Cow,
    env::{self},
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{DEFAULT_REMOTE_SHELL, Shell, ShellBuilder, SpawnInTerminal};
use terminal::{
    TaskState, TaskStatus, Terminal, TerminalBuilder,
    terminal_settings::{self, TerminalSettings, VenvSettings},
};
use util::ResultExt;

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

/// SshCommand describes how to connect to a remote server
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshCommand {
    pub arguments: Vec<String>,
}

impl SshCommand {
    pub fn add_port_forwarding(&mut self, local_port: u16, host: String, remote_port: u16) {
        self.arguments.push("-L".to_string());
        self.arguments
            .push(format!("{}:{}:{}", local_port, host, remote_port));
    }
}

impl Project {
    pub fn active_project_directory(&self, cx: &App) -> Option<Arc<Path>> {
        let worktree = self
            .active_entry()
            .and_then(|entry_id| self.worktree_for_entry(entry_id, cx))
            .into_iter()
            .chain(self.worktrees(cx))
            .find_map(|tree| tree.read(cx).root_dir());
        worktree
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

    pub fn ssh_details(&self, cx: &App) -> Option<(String, SshCommand)> {
        if let Some(ssh_client) = &self.ssh_client {
            let ssh_client = ssh_client.read(cx);
            if let Some(args) = ssh_client.ssh_args() {
                return Some((
                    ssh_client.connection_options().host.clone(),
                    SshCommand { arguments: args },
                ));
            }
        }

        return None;
    }

    pub fn create_terminal(
        &mut self,
        kind: TerminalKind,
        window: AnyWindowHandle,
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
        if let Some(path) = path.as_ref() {
            if let Some((worktree, _)) = self.find_worktree(path, cx) {
                settings_location = Some(SettingsLocation {
                    worktree_id: worktree.read(cx).id(),
                    path,
                });
            }
        }
        let settings = TerminalSettings::get(settings_location, cx).clone();

        cx.spawn(async move |project, cx| {
            let python_venv_directory = if let Some(path) = path.clone() {
                project
                    .update(cx, |this, cx| {
                        this.python_venv_directory(path, settings.detect_venv.clone(), cx)
                    })?
                    .await
            } else {
                None
            };
            project.update(cx, |project, cx| {
                project.create_terminal_with_venv(kind, python_venv_directory, window, cx)
            })?
        })
    }

    pub fn terminal_settings<'a>(
        &'a self,
        path: &'a Option<PathBuf>,
        cx: &'a App,
    ) -> &'a TerminalSettings {
        let mut settings_location = None;
        if let Some(path) = path.as_ref() {
            if let Some((worktree, _)) = self.find_worktree(path, cx) {
                settings_location = Some(SettingsLocation {
                    worktree_id: worktree.read(cx).id(),
                    path,
                });
            }
        }
        TerminalSettings::get(settings_location, cx)
    }

    pub fn exec_in_shell(&self, command: String, cx: &App) -> std::process::Command {
        let path = self.first_project_directory(cx);
        let ssh_details = self.ssh_details(cx);
        let settings = self.terminal_settings(&path, cx).clone();

        let builder = ShellBuilder::new(ssh_details.is_none(), &settings.shell);
        let (command, args) = builder.build(command, &Vec::new());

        let mut env = self
            .environment
            .read(cx)
            .get_cli_environment()
            .unwrap_or_default();
        env.extend(settings.env.clone());

        match &self.ssh_details(cx) {
            Some((_, ssh_command)) => {
                let (command, args) = wrap_for_ssh(
                    ssh_command,
                    Some((&command, &args)),
                    path.as_deref(),
                    env,
                    None,
                );
                let mut command = std::process::Command::new(command);
                command.args(args);
                command
            }
            None => {
                let mut command = std::process::Command::new(command);
                command.args(args);
                command.envs(env);
                if let Some(path) = path {
                    command.current_dir(path);
                }
                command
            }
        }
    }

    pub fn create_terminal_with_venv(
        &mut self,
        kind: TerminalKind,
        python_venv_directory: Option<PathBuf>,
        window: AnyWindowHandle,
        cx: &mut Context<Self>,
    ) -> Result<Entity<Terminal>> {
        let this = &mut *self;
        let path: Option<Arc<Path>> = match &kind {
            TerminalKind::Shell(path) => path.as_ref().map(|path| Arc::from(path.as_ref())),
            TerminalKind::Task(spawn_task) => {
                if let Some(cwd) = &spawn_task.cwd {
                    Some(Arc::from(cwd.as_ref()))
                } else {
                    this.active_project_directory(cx)
                }
            }
        };
        let ssh_details = this.ssh_details(cx);

        let mut settings_location = None;
        if let Some(path) = path.as_ref() {
            if let Some((worktree, _)) = this.find_worktree(path, cx) {
                settings_location = Some(SettingsLocation {
                    worktree_id: worktree.read(cx).id(),
                    path,
                });
            }
        }
        let settings = TerminalSettings::get(settings_location, cx).clone();

        let (completion_tx, completion_rx) = bounded(1);

        // Start with the environment that we might have inherited from the Zed CLI.
        let mut env = this
            .environment
            .read(cx)
            .get_cli_environment()
            .unwrap_or_default();
        // Then extend it with the explicit env variables from the settings, so they take
        // precedence.
        env.extend(settings.env.clone());

        let local_path = if ssh_details.is_none() {
            path.clone()
        } else {
            None
        };

        let mut python_venv_activate_command = None;

        let (spawn_task, shell) = match kind {
            TerminalKind::Shell(_) => {
                if let Some(python_venv_directory) = &python_venv_directory {
                    python_venv_activate_command =
                        this.python_activate_command(python_venv_directory, &settings.detect_venv);
                }

                match &ssh_details {
                    Some((host, ssh_command)) => {
                        log::debug!("Connecting to a remote server: {ssh_command:?}");

                        // Alacritty sets its terminfo to `alacritty`, this requiring hosts to have it installed
                        // to properly display colors.
                        // We do not have the luxury of assuming the host has it installed,
                        // so we set it to a default that does not break the highlighting via ssh.
                        env.entry("TERM".to_string())
                            .or_insert_with(|| "xterm-256color".to_string());

                        let (program, args) =
                            wrap_for_ssh(&ssh_command, None, path.as_deref(), env, None);
                        env = HashMap::default();
                        (
                            Option::<TaskState>::None,
                            Shell::WithArguments {
                                program,
                                args,
                                title_override: Some(format!("{} — Terminal", host).into()),
                            },
                        )
                    }
                    None => (None, settings.shell.clone()),
                }
            }
            TerminalKind::Task(spawn_task) => {
                let task_state = Some(TaskState {
                    id: spawn_task.id,
                    full_label: spawn_task.full_label,
                    label: spawn_task.label,
                    command_label: spawn_task.command_label,
                    hide: spawn_task.hide,
                    status: TaskStatus::Running,
                    show_summary: spawn_task.show_summary,
                    show_command: spawn_task.show_command,
                    show_rerun: spawn_task.show_rerun,
                    completion_rx,
                });

                env.extend(spawn_task.env);

                if let Some(venv_path) = &python_venv_directory {
                    env.insert(
                        "VIRTUAL_ENV".to_string(),
                        venv_path.to_string_lossy().to_string(),
                    );
                }

                match &ssh_details {
                    Some((host, ssh_command)) => {
                        log::debug!("Connecting to a remote server: {ssh_command:?}");
                        env.entry("TERM".to_string())
                            .or_insert_with(|| "xterm-256color".to_string());
                        let (program, args) = wrap_for_ssh(
                            &ssh_command,
                            Some((&spawn_task.command, &spawn_task.args)),
                            path.as_deref(),
                            env,
                            python_venv_directory.as_deref(),
                        );
                        env = HashMap::default();
                        (
                            task_state,
                            Shell::WithArguments {
                                program,
                                args,
                                title_override: Some(format!("{} — Terminal", host).into()),
                            },
                        )
                    }
                    None => {
                        if let Some(venv_path) = &python_venv_directory {
                            add_environment_path(&mut env, &venv_path.join("bin")).log_err();
                        }

                        (
                            task_state,
                            Shell::WithArguments {
                                program: spawn_task.command,
                                args: spawn_task.args,
                                title_override: None,
                            },
                        )
                    }
                }
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
            ssh_details.is_some(),
            window,
            completion_tx,
            cx,
        )
        .map(|builder| {
            let terminal_handle = cx.new(|cx| builder.subscribe(cx));

            this.terminals
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

            if let Some(activate_command) = python_venv_activate_command {
                this.activate_python_virtual_environment(activate_command, &terminal_handle, cx);
            }
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
        let bin_dir_name = match std::env::consts::OS {
            "windows" => "Scripts",
            _ => "bin",
        };
        venv_settings
            .directories
            .iter()
            .map(|name| abs_path.join(name))
            .find(|venv_path| {
                let bin_path = venv_path.join(bin_dir_name);
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
        let bin_dir_name = match std::env::consts::OS {
            "windows" => "Scripts",
            _ => "bin",
        };
        venv_settings
            .directories
            .iter()
            .map(|name| abs_path.join(name))
            .find(|venv_path| {
                let bin_path = venv_path.join(bin_dir_name);
                // One-time synchronous check is acceptable for terminal/task initialization
                smol::block_on(fs.metadata(&bin_path))
                    .ok()
                    .flatten()
                    .map_or(false, |meta| meta.is_dir)
            })
    }

    fn python_activate_command(
        &self,
        venv_base_directory: &Path,
        venv_settings: &VenvSettings,
    ) -> Option<String> {
        let venv_settings = venv_settings.as_option()?;
        let activate_keyword = match venv_settings.activate_script {
            terminal_settings::ActivateScript::Default => match std::env::consts::OS {
                "windows" => ".",
                _ => "source",
            },
            terminal_settings::ActivateScript::Nushell => "overlay use",
            terminal_settings::ActivateScript::PowerShell => ".",
            _ => "source",
        };
        let activate_script_name = match venv_settings.activate_script {
            terminal_settings::ActivateScript::Default => "activate",
            terminal_settings::ActivateScript::Csh => "activate.csh",
            terminal_settings::ActivateScript::Fish => "activate.fish",
            terminal_settings::ActivateScript::Nushell => "activate.nu",
            terminal_settings::ActivateScript::PowerShell => "activate.ps1",
        };
        let path = venv_base_directory
            .join(match std::env::consts::OS {
                "windows" => "Scripts",
                _ => "bin",
            })
            .join(activate_script_name)
            .to_string_lossy()
            .to_string();
        let quoted = shlex::try_quote(&path).ok()?;
        let line_ending = match std::env::consts::OS {
            "windows" => "\r",
            _ => "\n",
        };
        smol::block_on(self.fs.metadata(path.as_ref()))
            .ok()
            .flatten()?;

        Some(format!(
            "{} {} ; clear{}",
            activate_keyword, quoted, line_ending
        ))
    }

    fn activate_python_virtual_environment(
        &self,
        command: String,
        terminal_handle: &Entity<Terminal>,
        cx: &mut App,
    ) {
        terminal_handle.update(cx, |terminal, _| terminal.input(command));
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakEntity<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

pub fn wrap_for_ssh(
    ssh_command: &SshCommand,
    command: Option<(&String, &Vec<String>)>,
    path: Option<&Path>,
    env: HashMap<String, String>,
    venv_directory: Option<&Path>,
) -> (String, Vec<String>) {
    let to_run = if let Some((command, args)) = command {
        // DEFAULT_REMOTE_SHELL is '"${SHELL:-sh}"' so must not be escaped
        let command: Option<Cow<str>> = if command == DEFAULT_REMOTE_SHELL {
            Some(command.into())
        } else {
            shlex::try_quote(command).ok()
        };
        let args = args.iter().filter_map(|arg| shlex::try_quote(arg).ok());
        command.into_iter().chain(args).join(" ")
    } else {
        "exec ${SHELL:-sh} -l".to_string()
    };

    let mut env_changes = String::new();
    for (k, v) in env.iter() {
        if let Some((k, v)) = shlex::try_quote(k).ok().zip(shlex::try_quote(v).ok()) {
            env_changes.push_str(&format!("{}={} ", k, v));
        }
    }
    if let Some(venv_directory) = venv_directory {
        if let Ok(str) = shlex::try_quote(venv_directory.to_string_lossy().as_ref()) {
            env_changes.push_str(&format!("PATH={}:$PATH ", str));
        }
    }

    let commands = if let Some(path) = path {
        let path_string = path.to_string_lossy().to_string();
        // shlex will wrap the command in single quotes (''), disabling ~ expansion,
        // replace ith with something that works
        let tilde_prefix = "~/";
        if path.starts_with(tilde_prefix) {
            let trimmed_path = path_string
                .trim_start_matches("/")
                .trim_start_matches("~")
                .trim_start_matches("/");

            format!("cd \"$HOME/{trimmed_path}\"; {env_changes} {to_run}")
        } else {
            format!("cd {path:?}; {env_changes} {to_run}")
        }
    } else {
        format!("cd; {env_changes} {to_run}")
    };
    let shell_invocation = format!("sh -c {}", shlex::try_quote(&commands).unwrap());

    let program = "ssh".to_string();
    let mut args = ssh_command.arguments.clone();

    args.push("-t".to_string());
    args.push(shell_invocation);
    (program, args)
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

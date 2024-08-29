use crate::Project;
use anyhow::Context as _;
use collections::HashMap;
use gpui::{AnyWindowHandle, AppContext, Context, Entity, Model, ModelContext, WeakModel};
use itertools::Itertools;
use settings::{Settings, SettingsLocation};
use smol::channel::bounded;
use std::{
    env::{self},
    iter,
    path::{Path, PathBuf},
};
use task::{Shell, SpawnInTerminal};
use terminal::{
    terminal_settings::{self, TerminalSettings},
    TaskState, TaskStatus, Terminal, TerminalBuilder,
};
use util::ResultExt;

// #[cfg(target_os = "macos")]
// use std::os::unix::ffi::OsStrExt;

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakModel<terminal::Terminal>>,
}

/// Terminals are opened either for the users shell, or to run a task.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum TerminalKind {
    /// Run a shell at the given path (or $HOME if None)
    Shell(Option<PathBuf>),
    /// Run a task.
    Task(SpawnInTerminal),
}

/// SshCommand describes how to connect to a remote server
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SshCommand {
    /// DevServers give a string from the user
    DevServer(String),
    /// Direct ssh has a list of arguments to pass to ssh
    Direct(Vec<String>),
}

impl Project {
    pub fn active_project_directory(&self, cx: &AppContext) -> Option<PathBuf> {
        let worktree = self
            .active_entry()
            .and_then(|entry_id| self.worktree_for_entry(entry_id, cx))
            .or_else(|| self.worktrees(cx).next())?;
        let worktree = worktree.read(cx);
        if !worktree.root_entry()?.is_dir() {
            return None;
        }
        Some(worktree.abs_path().to_path_buf())
    }

    pub fn first_project_directory(&self, cx: &AppContext) -> Option<PathBuf> {
        let worktree = self.worktrees(cx).next()?;
        let worktree = worktree.read(cx);
        if worktree.root_entry()?.is_dir() {
            return Some(worktree.abs_path().to_path_buf());
        } else {
            None
        }
    }

    fn ssh_command(&self, cx: &AppContext) -> Option<SshCommand> {
        if let Some(ssh_session) = self.ssh_session.as_ref() {
            return Some(SshCommand::Direct(ssh_session.ssh_args()));
        }

        let dev_server_project_id = self.dev_server_project_id()?;
        let projects_store = dev_server_projects::Store::global(cx).read(cx);
        let ssh_command = projects_store
            .dev_server_for_project(dev_server_project_id)?
            .ssh_connection_string
            .as_ref()?
            .to_string();
        Some(SshCommand::DevServer(ssh_command))
    }

    pub fn create_terminal(
        &mut self,
        kind: TerminalKind,
        window: AnyWindowHandle,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<Model<Terminal>> {
        let path = match &kind {
            TerminalKind::Shell(path) => path.as_ref().map(|path| path.to_path_buf()),
            TerminalKind::Task(spawn_task) => {
                if let Some(cwd) = &spawn_task.cwd {
                    Some(cwd.clone())
                } else {
                    self.active_project_directory(cx)
                }
            }
        };
        let ssh_command = self.ssh_command(cx);

        let mut settings_location = None;
        if let Some(path) = path.as_ref() {
            if let Some((worktree, _)) = self.find_worktree(path, cx) {
                settings_location = Some(SettingsLocation {
                    worktree_id: worktree.read(cx).id().to_usize(),
                    path,
                });
            }
        }
        let settings = TerminalSettings::get(settings_location, cx);

        let (completion_tx, completion_rx) = bounded(1);

        // Start with the environment that we might have inherited from the Zed CLI.
        let mut env = self
            .environment
            .read(cx)
            .get_cli_environment()
            .unwrap_or_default();
        // Then extend it with the explicit env variables from the settings, so they take
        // precedence.
        env.extend(settings.env.clone());

        let local_path = if ssh_command.is_none() {
            path.clone()
        } else {
            None
        };
        let python_venv_directory = path
            .as_ref()
            .and_then(|path| self.python_venv_directory(path, settings, cx));
        let mut python_venv_activate_command = None;

        let (spawn_task, shell) = match kind {
            TerminalKind::Shell(_) => {
                if let Some(python_venv_directory) = python_venv_directory {
                    python_venv_activate_command =
                        self.python_activate_command(&python_venv_directory, settings);
                }

                match &ssh_command {
                    Some(ssh_command) => {
                        log::debug!("Connecting to a remote server: {ssh_command:?}");

                        // Alacritty sets its terminfo to `alacritty`, this requiring hosts to have it installed
                        // to properly display colors.
                        // We do not have the luxury of assuming the host has it installed,
                        // so we set it to a default that does not break the highlighting via ssh.
                        env.entry("TERM".to_string())
                            .or_insert_with(|| "xterm-256color".to_string());

                        let (program, args) =
                            wrap_for_ssh(ssh_command, None, path.as_deref(), env, None);
                        env = HashMap::default();
                        (None, Shell::WithArguments { program, args })
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
                    completion_rx,
                });

                env.extend(spawn_task.env);

                if let Some(venv_path) = &python_venv_directory {
                    env.insert(
                        "VIRTUAL_ENV".to_string(),
                        venv_path.to_string_lossy().to_string(),
                    );
                }

                match &ssh_command {
                    Some(ssh_command) => {
                        log::debug!("Connecting to a remote server: {ssh_command:?}");
                        env.entry("TERM".to_string())
                            .or_insert_with(|| "xterm-256color".to_string());
                        let (program, args) = wrap_for_ssh(
                            ssh_command,
                            Some((&spawn_task.command, &spawn_task.args)),
                            path.as_deref(),
                            env,
                            python_venv_directory,
                        );
                        env = HashMap::default();
                        (task_state, Shell::WithArguments { program, args })
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
                            },
                        )
                    }
                }
            }
        };

        let terminal = TerminalBuilder::new(
            local_path,
            spawn_task,
            shell,
            env,
            Some(settings.blinking),
            settings.alternate_scroll,
            settings.max_scroll_history_lines,
            window,
            completion_tx,
            cx,
        )
        .map(|builder| {
            let terminal_handle = cx.new_model(|cx| builder.subscribe(cx));

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

            if let Some(activate_command) = python_venv_activate_command {
                self.activate_python_virtual_environment(activate_command, &terminal_handle, cx);
            }
            terminal_handle
        });

        terminal
    }

    pub fn python_venv_directory(
        &self,
        abs_path: &Path,
        settings: &TerminalSettings,
        cx: &AppContext,
    ) -> Option<PathBuf> {
        let venv_settings = settings.detect_venv.as_option()?;
        venv_settings
            .directories
            .into_iter()
            .map(|virtual_environment_name| abs_path.join(virtual_environment_name))
            .find(|venv_path| {
                self.find_worktree(&venv_path, cx)
                    .and_then(|(worktree, relative_path)| {
                        worktree.read(cx).entry_for_path(&relative_path)
                    })
                    .is_some_and(|entry| entry.is_dir())
            })
    }

    fn python_activate_command(
        &self,
        venv_base_directory: &Path,
        settings: &TerminalSettings,
    ) -> Option<String> {
        let venv_settings = settings.detect_venv.as_option()?;
        let activate_script_name = match venv_settings.activate_script {
            terminal_settings::ActivateScript::Default => "activate",
            terminal_settings::ActivateScript::Csh => "activate.csh",
            terminal_settings::ActivateScript::Fish => "activate.fish",
            terminal_settings::ActivateScript::Nushell => "activate.nu",
        };
        let path = venv_base_directory
            .join("bin")
            .join(activate_script_name)
            .to_string_lossy()
            .to_string();
        let quoted = shlex::try_quote(&path).ok()?;

        Some(match venv_settings.activate_script {
            terminal_settings::ActivateScript::Nushell => format!("overlay use {}\n", quoted),
            _ => format!("source {}\n", quoted),
        })
    }

    fn activate_python_virtual_environment(
        &self,
        command: String,
        terminal_handle: &Model<Terminal>,
        cx: &mut ModelContext<Project>,
    ) {
        terminal_handle.update(cx, |this, _| this.input_bytes(command.into_bytes()));
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakModel<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

pub fn wrap_for_ssh(
    ssh_command: &SshCommand,
    command: Option<(&String, &Vec<String>)>,
    path: Option<&Path>,
    env: HashMap<String, String>,
    venv_directory: Option<PathBuf>,
) -> (String, Vec<String>) {
    let to_run = if let Some((command, args)) = command {
        iter::once(command)
            .chain(args)
            .filter_map(|arg| shlex::try_quote(arg).ok())
            .join(" ")
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
        if let Some(str) = shlex::try_quote(venv_directory.to_string_lossy().as_ref()).ok() {
            env_changes.push_str(&format!("PATH={}:$PATH ", str));
        }
    }

    let commands = if let Some(path) = path {
        format!("cd {:?}; {} {}", path, env_changes, to_run)
    } else {
        format!("cd; {env_changes} {to_run}")
    };
    let shell_invocation = format!("sh -c {}", shlex::try_quote(&commands).unwrap());

    let (program, mut args) = match ssh_command {
        SshCommand::DevServer(ssh_command) => {
            let mut args = shlex::split(&ssh_command).unwrap_or_default();
            let program = args.drain(0..1).next().unwrap_or("ssh".to_string());
            (program, args)
        }
        SshCommand::Direct(ssh_args) => ("ssh".to_string(), ssh_args.clone()),
    };

    if command.is_none() {
        args.push("-t".to_string())
    }
    args.push(shell_invocation);
    (program, args)
}

fn add_environment_path(env: &mut HashMap<String, String>, new_path: &Path) -> anyhow::Result<()> {
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

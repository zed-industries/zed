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
    terminal_settings::{self, TerminalSettings, VenvSettingsContent},
    TaskState, TaskStatus, Terminal, TerminalBuilder,
};
use util::ResultExt;

// #[cfg(target_os = "macos")]
// use std::os::unix::ffi::OsStrExt;

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakModel<terminal::Terminal>>,
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
pub enum SshCommand {
    /// DevServers give a string from the user
    DevServer(String),
    /// Direct ssh has a list of arguments to pass to ssh
    Direct(Vec<String>),
}

impl Project {
    pub fn active_project_directory(&self, cx: &AppContext) -> Option<PathBuf> {
        for worktree in self
            .active_entry()
            .and_then(|entry_id| self.worktree_for_entry(entry_id, cx))
            .into_iter()
            .chain(self.worktrees(cx))
        {
            let worktree = worktree.read(cx);
            if worktree.root_entry().is_some_and(|re| re.is_dir()) {
                return Some(worktree.abs_path().to_path_buf());
            }
        }
        None
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

        let python_settings = settings.detect_venv.clone();
        let (completion_tx, completion_rx) = bounded(1);

        let mut env = settings.env.clone();

        let is_terminal = matches!(kind, TerminalKind::Shell(_)) && ssh_command.is_none();
        let local_path = if ssh_command.is_none() {
            path.clone()
        } else {
            None
        };
        let venv_base_directory = local_path.clone().unwrap_or_else(|| PathBuf::new());

        let (spawn_task, shell) = match kind {
            TerminalKind::Shell(_) => match &ssh_command {
                Some(ssh_command) => {
                    log::debug!("Connecting to a remote server: {ssh_command:?}");

                    // Alacritty sets its terminfo to `alacritty`, this requiring hosts to have it installed
                    // to properly display colors.
                    // We do not have the luxury of assuming the host has it installed,
                    // so we set it to a default that does not break the highlighting via ssh.
                    env.entry("TERM".to_string())
                        .or_insert_with(|| "xterm-256color".to_string());

                    let (program, args) = wrap_for_ssh(ssh_command, None, path.as_deref());
                    (None, Shell::WithArguments { program, args })
                }
                None => (None, settings.shell.clone()),
            },
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

                match &ssh_command {
                    Some(ssh_command) => {
                        log::debug!("Connecting to a remote server: {ssh_command:?}");
                        env.entry("TERM".to_string())
                            .or_insert_with(|| "xterm-256color".to_string());
                        let (program, args) = wrap_for_ssh(
                            ssh_command,
                            Some((&spawn_task.command, &spawn_task.args)),
                            path.as_deref(),
                        );
                        (task_state, Shell::WithArguments { program, args })
                    }
                    None => {
                        // todo: this should happen on remotes if ssh command is set
                        env.extend(spawn_task.env);
                        if let Some(python_settings) = &python_settings.as_option() {
                            self.set_python_venv_path_for_tasks(
                                python_settings,
                                &venv_base_directory,
                                &mut env,
                            )
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

            // if the terminal is not a task, activate full Python virtual environment
            if is_terminal {
                if let Some(python_settings) = &python_settings.as_option() {
                    if let Some(activate_script_path) =
                        self.find_activate_script_path(python_settings, &venv_base_directory)
                    {
                        self.activate_python_virtual_environment(
                            Project::get_activate_command(python_settings),
                            activate_script_path,
                            &terminal_handle,
                            cx,
                        );
                    }
                }
            }
            terminal_handle
        });

        terminal
    }

    pub fn find_activate_script_path(
        &mut self,
        settings: &VenvSettingsContent,
        venv_base_directory: &Path,
    ) -> Option<PathBuf> {
        let activate_script_name = match settings.activate_script {
            terminal_settings::ActivateScript::Default => "activate",
            terminal_settings::ActivateScript::Csh => "activate.csh",
            terminal_settings::ActivateScript::Fish => "activate.fish",
            terminal_settings::ActivateScript::Nushell => "activate.nu",
        };

        settings
            .directories
            .into_iter()
            .find_map(|virtual_environment_name| {
                let path = venv_base_directory
                    .join(virtual_environment_name)
                    .join("bin")
                    .join(activate_script_name);
                path.exists().then_some(path)
            })
    }

    pub fn set_python_venv_path_for_tasks(
        &mut self,
        settings: &VenvSettingsContent,
        venv_base_directory: &Path,
        env: &mut HashMap<String, String>,
    ) {
        let activate_path = settings
            .directories
            .into_iter()
            .find_map(|virtual_environment_name| {
                let path = venv_base_directory.join(virtual_environment_name);
                path.exists().then_some(path)
            });

        if let Some(path) = activate_path {
            // Some tools use VIRTUAL_ENV to detect the virtual environment
            env.insert(
                "VIRTUAL_ENV".to_string(),
                path.to_string_lossy().to_string(),
            );

            // We need to set the PATH to include the virtual environment's bin directory
            add_environment_path(env, &path.join("bin")).log_err();
        }
    }

    fn get_activate_command(settings: &VenvSettingsContent) -> &'static str {
        match settings.activate_script {
            terminal_settings::ActivateScript::Nushell => "overlay use",
            _ => "source",
        }
    }

    fn activate_python_virtual_environment(
        &mut self,
        activate_command: &'static str,
        activate_script: PathBuf,
        terminal_handle: &Model<Terminal>,
        cx: &mut ModelContext<Project>,
    ) {
        // Paths are not strings so we need to jump through some hoops to format the command without `format!`
        let mut command = Vec::from(activate_command.as_bytes());
        command.push(b' ');
        // Wrapping path in double quotes to catch spaces in folder name
        command.extend_from_slice(b"\"");
        command.extend_from_slice(activate_script.as_os_str().as_encoded_bytes());
        command.extend_from_slice(b"\"");
        command.push(b'\n');

        terminal_handle.update(cx, |this, _| this.input_bytes(command));
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakModel<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

pub fn wrap_for_ssh(
    ssh_command: &SshCommand,
    command: Option<(&String, &Vec<String>)>,
    path: Option<&Path>,
) -> (String, Vec<String>) {
    let to_run = if let Some((command, args)) = command {
        iter::once(command)
            .chain(args)
            .filter_map(|arg| shlex::try_quote(arg).ok())
            .join(" ")
    } else {
        "exec ${SHELL:-sh} -l".to_string()
    };

    let commands = if let Some(path) = path {
        format!("cd {:?}; {}", path, to_run)
    } else {
        format!("cd; {to_run}")
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

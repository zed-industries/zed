use crate::Project;
use collections::HashMap;
use gpui::{
    AnyWindowHandle, AppContext, Context, Entity, Model, ModelContext, SharedString, WeakModel,
};
use settings::{Settings, SettingsLocation};
use smol::channel::bounded;
use std::path::{Path, PathBuf};
use task::SpawnInTerminal;
use terminal::{
    terminal_settings::{self, Shell, TerminalSettings, VenvSettingsContent},
    TaskState, TaskStatus, Terminal, TerminalBuilder,
};
use util::ResultExt;

// #[cfg(target_os = "macos")]
// use std::os::unix::ffi::OsStrExt;

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakModel<terminal::Terminal>>,
}

#[derive(Debug, Clone)]
pub struct ConnectRemoteTerminal {
    pub ssh_connection_string: SharedString,
    pub project_path: SharedString,
}

impl Project {
    pub fn remote_terminal_connection_data(
        &self,
        cx: &AppContext,
    ) -> Option<ConnectRemoteTerminal> {
        self.dev_server_project_id()
            .and_then(|dev_server_project_id| {
                let projects_store = dev_server_projects::Store::global(cx).read(cx);
                let project_path = projects_store
                    .dev_server_project(dev_server_project_id)?
                    .path
                    .clone();
                let ssh_connection_string = projects_store
                    .dev_server_for_project(dev_server_project_id)?
                    .ssh_connection_string
                    .clone();
                Some(project_path).zip(ssh_connection_string)
            })
            .map(
                |(project_path, ssh_connection_string)| ConnectRemoteTerminal {
                    ssh_connection_string,
                    project_path,
                },
            )
    }

    pub fn create_terminal(
        &mut self,
        working_directory: Option<PathBuf>,
        spawn_task: Option<SpawnInTerminal>,
        window: AnyWindowHandle,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<Model<Terminal>> {
        let remote_connection_data = if self.is_remote() {
            let remote_connection_data = self.remote_terminal_connection_data(cx);
            if remote_connection_data.is_none() {
                anyhow::bail!("Cannot create terminal for remote project without connection data")
            }
            remote_connection_data
        } else {
            None
        };

        // used only for TerminalSettings::get
        let worktree = {
            let terminal_cwd = working_directory.as_deref();
            let task_cwd = spawn_task
                .as_ref()
                .and_then(|spawn_task| spawn_task.cwd.as_deref());

            terminal_cwd
                .and_then(|terminal_cwd| self.find_local_worktree(terminal_cwd, cx))
                .or_else(|| task_cwd.and_then(|spawn_cwd| self.find_local_worktree(spawn_cwd, cx)))
        };

        let settings_location = worktree.as_ref().map(|(worktree, path)| SettingsLocation {
            worktree_id: worktree.read(cx).id().to_usize(),
            path,
        });

        let is_terminal = spawn_task.is_none() && remote_connection_data.is_none();
        let settings = TerminalSettings::get(settings_location, cx);
        let python_settings = settings.detect_venv.clone();
        let (completion_tx, completion_rx) = bounded(1);

        let mut env = settings.env.clone();
        // Alacritty uses parent project's working directory when no working directory is provided
        // https://github.com/alacritty/alacritty/blob/fd1a3cc79192d1d03839f0fd8c72e1f8d0fce42e/extra/man/alacritty.5.scd?plain=1#L47-L52

        let venv_base_directory = working_directory
            .as_deref()
            .unwrap_or_else(|| Path::new(""));

        let (spawn_task, shell) = if let Some(remote_connection_data) = remote_connection_data {
            log::debug!("Connecting to a remote server: {remote_connection_data:?}");
            // Alacritty sets its terminfo to `alacritty`, this requiring hosts to have it installed
            // to properly display colors.
            // We do not have the luxury of assuming the host has it installed,
            // so we set it to a default that does not break the highlighting via ssh.
            env.entry("TERM".to_string())
                .or_insert_with(|| "xterm-256color".to_string());

            (
                None,
                Shell::WithArguments {
                    program: "ssh".to_string(),
                    args: vec![
                        remote_connection_data.ssh_connection_string.to_string(),
                        "-t".to_string(),
                        format!(
                            "cd {} && exec $SHELL -l",
                            escape_path_for_shell(remote_connection_data.project_path.as_ref())
                        ),
                    ],
                },
            )
        } else if let Some(spawn_task) = spawn_task {
            log::debug!("Spawning task: {spawn_task:?}");
            env.extend(spawn_task.env);
            // Activate minimal Python virtual environment
            if let Some(python_settings) = &python_settings.as_option() {
                self.set_python_venv_path_for_tasks(python_settings, venv_base_directory, &mut env);
            }
            (
                Some(TaskState {
                    id: spawn_task.id,
                    full_label: spawn_task.full_label,
                    label: spawn_task.label,
                    command_label: spawn_task.command_label,
                    status: TaskStatus::Running,
                    completion_rx,
                }),
                Shell::WithArguments {
                    program: spawn_task.command,
                    args: spawn_task.args,
                },
            )
        } else {
            (None, settings.shell.clone())
        };

        let terminal = TerminalBuilder::new(
            working_directory.clone(),
            spawn_task,
            shell,
            env,
            Some(settings.blinking.clone()),
            settings.alternate_scroll,
            settings.max_scroll_history_lines,
            window,
            completion_tx,
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
                        self.find_activate_script_path(python_settings, venv_base_directory)
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

            let path_bin = path.join("bin");
            // We need to set the PATH to include the virtual environment's bin directory
            if let Some(paths) = std::env::var_os("PATH") {
                let paths = std::iter::once(path_bin).chain(std::env::split_paths(&paths));
                if let Some(new_path) = std::env::join_paths(paths).log_err() {
                    env.insert("PATH".to_string(), new_path.to_string_lossy().to_string());
                }
            } else {
                env.insert(
                    "PATH".to_string(),
                    path.join("bin").to_string_lossy().to_string(),
                );
            }
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

#[cfg(unix)]
fn escape_path_for_shell(input: &str) -> String {
    input
        .chars()
        .fold(String::with_capacity(input.len()), |mut s, c| {
            match c {
                ' ' | '"' | '\'' | '\\' | '(' | ')' | '{' | '}' | '[' | ']' | '|' | ';' | '&'
                | '<' | '>' | '*' | '?' | '$' | '#' | '!' | '=' | '^' | '%' | ':' => {
                    s.push('\\');
                    s.push('\\');
                    s.push(c);
                }
                _ => s.push(c),
            }
            s
        })
}

#[cfg(windows)]
fn escape_path_for_shell(input: &str) -> String {
    input
        .chars()
        .fold(String::with_capacity(input.len()), |mut s, c| {
            match c {
                '^' | '&' | '|' | '<' | '>' | ' ' | '(' | ')' | '@' | '`' | '=' | ';' | '%' => {
                    s.push('^');
                    s.push(c);
                }
                _ => s.push(c),
            }
            s
        })
}

// TODO: Add a few tests for adding and removing terminal tabs

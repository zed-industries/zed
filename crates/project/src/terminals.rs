use crate::Project;
use collections::HashMap;
use gpui::{
    AnyWindowHandle, AppContext, Context, Entity, Model, ModelContext, SharedString, WeakModel,
};
use itertools::Itertools;
use settings::{Settings, SettingsLocation};
use smol::channel::bounded;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use task::{SpawnInTerminal, TerminalWorkDir};
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
    pub fn terminal_work_dir_for(
        &self,
        pathbuf: Option<&Path>,
        cx: &AppContext,
    ) -> Option<TerminalWorkDir> {
        if self.is_local() {
            return Some(TerminalWorkDir::Local(pathbuf?.to_owned()));
        }
        let dev_server_project_id = self.dev_server_project_id()?;
        let projects_store = dev_server_projects::Store::global(cx).read(cx);
        let ssh_command = projects_store
            .dev_server_for_project(dev_server_project_id)?
            .ssh_connection_string
            .as_ref()?
            .to_string();

        let path = if let Some(pathbuf) = pathbuf {
            pathbuf.to_string_lossy().to_string()
        } else {
            projects_store
                .dev_server_project(dev_server_project_id)?
                .path
                .to_string()
        };

        Some(TerminalWorkDir::Ssh {
            ssh_command,
            path: Some(path),
        })
    }

    pub fn create_terminal(
        &mut self,
        working_directory: Option<TerminalWorkDir>,
        spawn_task: Option<SpawnInTerminal>,
        window: AnyWindowHandle,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<Model<Terminal>> {
        // used only for TerminalSettings::get
        let worktree = {
            let terminal_cwd = working_directory.as_ref().and_then(|cwd| cwd.local_path());
            let task_cwd = spawn_task
                .as_ref()
                .and_then(|spawn_task| spawn_task.cwd.as_ref())
                .and_then(|cwd| cwd.local_path());

            terminal_cwd
                .and_then(|terminal_cwd| self.find_local_worktree(&terminal_cwd, cx))
                .or_else(|| task_cwd.and_then(|spawn_cwd| self.find_local_worktree(&spawn_cwd, cx)))
        };

        let settings_location = worktree.as_ref().map(|(worktree, path)| SettingsLocation {
            worktree_id: worktree.read(cx).id().to_usize(),
            path,
        });

        let is_terminal = spawn_task.is_none()
            && working_directory
                .as_ref()
                .map_or(true, |work_dir| work_dir.is_local());
        let settings = TerminalSettings::get(settings_location, cx);
        let python_settings = settings.detect_venv.clone();
        let (completion_tx, completion_rx) = bounded(1);

        let mut env = settings.env.clone();
        // Alacritty uses parent project's working directory when no working directory is provided
        // https://github.com/alacritty/alacritty/blob/fd1a3cc79192d1d03839f0fd8c72e1f8d0fce42e/extra/man/alacritty.5.scd?plain=1#L47-L52

        let mut retained_script = None;

        let venv_base_directory = working_directory
            .as_ref()
            .and_then(|cwd| cwd.local_path())
            .unwrap_or_else(|| Path::new(""));

        let (spawn_task, shell) = match working_directory.as_ref() {
            Some(TerminalWorkDir::Ssh { ssh_command, path }) => {
                log::debug!("Connecting to a remote server: {ssh_command:?}");
                let tmp_dir = tempfile::tempdir()?;
                let ssh_shell_result = prepare_ssh_shell(
                    &mut env,
                    tmp_dir.path(),
                    spawn_task.as_ref(),
                    ssh_command,
                    path.as_deref(),
                );
                retained_script = Some(tmp_dir);
                let ssh_shell = ssh_shell_result?;

                (
                    spawn_task.map(|spawn_task| TaskState {
                        id: spawn_task.id,
                        full_label: spawn_task.full_label,
                        label: spawn_task.label,
                        command_label: spawn_task.command_label,
                        status: TaskStatus::Running,
                        completion_rx,
                    }),
                    ssh_shell,
                )
            }
            _ => {
                if let Some(spawn_task) = spawn_task {
                    log::debug!("Spawning task: {spawn_task:?}");
                    env.extend(spawn_task.env);
                    // Activate minimal Python virtual environment
                    if let Some(python_settings) = &python_settings.as_option() {
                        self.set_python_venv_path_for_tasks(
                            python_settings,
                            &venv_base_directory,
                            &mut env,
                        );
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
                }
            }
        };

        let terminal = TerminalBuilder::new(
            working_directory
                .as_ref()
                .and_then(|cwd| cwd.local_path())
                .map(ToOwned::to_owned),
            spawn_task,
            shell,
            env,
            Some(settings.blinking),
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
                drop(retained_script);
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

fn prepare_ssh_shell(
    env: &mut HashMap<String, String>,
    tmp_dir: &Path,
    spawn_task: Option<&SpawnInTerminal>,
    ssh_command: &str,
    path: Option<&str>,
) -> anyhow::Result<Shell> {
    // Alacritty sets its terminfo to `alacritty`, this requiring hosts to have it installed
    // to properly display colors.
    // We do not have the luxury of assuming the host has it installed,
    // so we set it to a default that does not break the highlighting via ssh.
    env.entry("TERM".to_string())
        .or_insert_with(|| "xterm-256color".to_string());

    let real_ssh = which::which("ssh")?;
    let ssh_path = tmp_dir.join("ssh");
    let mut ssh_file = File::create(&ssh_path)?;

    let to_run = if let Some(spawn_task) = spawn_task {
        Some(shlex::try_quote(&spawn_task.command)?)
            .into_iter()
            .chain(
                spawn_task
                    .args
                    .iter()
                    .filter_map(|arg| shlex::try_quote(arg).ok()),
            )
            .join(" ")
    } else {
        "exec $SHELL -l".to_string()
    };

    let (port_forward, local_dev_env) =
        if env::var("ZED_RPC_URL").as_deref() == Ok("http://localhost:8080/rpc") {
            (
                "-R 8080:localhost:8080",
                "export ZED_RPC_URL=http://localhost:8080/rpc;",
            )
        } else {
            ("", "")
        };

    let commands = if let Some(path) = path {
        // I've found that `ssh -t dev sh -c 'cd; cd /tmp; pwd'` gives /tmp
        // but `ssh -t dev sh -c 'cd /tmp; pwd'` gives /root
        format!("cd {path}; {local_dev_env} {to_run}")
    } else {
        format!("cd; {local_dev_env} {to_run}")
    };
    let shell_invocation = &format!("sh -c {}", shlex::try_quote(&commands)?);

    // To support things like `gh cs ssh`/`coder ssh`, we run whatever command
    // you have configured, but place our custom script on the path so that it will
    // be run instead.
    write!(
        &mut ssh_file,
        "#!/bin/sh\nexec {} \"$@\" {} {} {}",
        real_ssh.to_string_lossy(),
        if spawn_task.is_none() { "-t" } else { "" },
        port_forward,
        shlex::try_quote(shell_invocation)?,
    )?;

    // todo(windows)
    #[cfg(not(target_os = "windows"))]
    std::fs::set_permissions(ssh_path, smol::fs::unix::PermissionsExt::from_mode(0o755))?;
    let path = format!(
        "{}:{}",
        tmp_dir.to_string_lossy(),
        env.get("PATH")
            .cloned()
            .or(env::var("PATH").ok())
            .unwrap_or_default()
    );
    env.insert("PATH".to_string(), path);

    let mut args = shlex::split(&ssh_command).unwrap_or_default();
    let program = args.drain(0..1).next().unwrap_or("ssh".to_string());
    Ok(Shell::WithArguments { program, args })
}

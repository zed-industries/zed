use anyhow::Result;
use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity};
use itertools::Itertools as _;
use language::LanguageName;
use remote::{SshInfo, ssh_session::SshArgs};
use settings::{Settings, SettingsLocation};
use smol::channel::bounded;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{Shell, ShellBuilder, ShellKind, SpawnInTerminal};
use terminal::{
    TaskState, TaskStatus, Terminal, TerminalBuilder, terminal_settings::TerminalSettings,
};
use util::{
    get_system_shell, maybe,
    paths::{PathStyle, RemotePathBuf},
};

use crate::{Project, ProjectPath};

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakEntity<terminal::Terminal>>,
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

#[derive(Debug)]
pub struct SshDetails {
    pub host: String,
    pub ssh_command: SshCommand,
    pub envs: Option<HashMap<String, String>>,
    pub path_style: PathStyle,
    pub shell: String,
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

    pub fn ssh_details(&self, cx: &App) -> Option<SshDetails> {
        if let Some(ssh_client) = &self.ssh_client {
            let ssh_client = ssh_client.read(cx);
            if let Some(SshInfo {
                args: SshArgs { arguments, envs },
                path_style,
                shell,
            }) = ssh_client.ssh_info()
            {
                return Some(SshDetails {
                    host: ssh_client.connection_options().host,
                    ssh_command: SshCommand { arguments },
                    envs,
                    path_style,
                    shell,
                });
            }
        }

        None
    }

    pub fn create_terminal_task(
        &mut self,
        spawn_task: SpawnInTerminal,
        cx: &mut Context<Self>,
        project_path_context: Option<ProjectPath>,
    ) -> Task<Result<Entity<Terminal>>> {
        let this = &mut *self;
        let ssh_details = this.ssh_details(cx);
        let path: Option<Arc<Path>> = if let Some(cwd) = &spawn_task.cwd {
            if ssh_details.is_some() {
                Some(Arc::from(cwd.as_ref()))
            } else {
                let cwd = cwd.to_string_lossy();
                let tilde_substituted = shellexpand::tilde(&cwd);
                Some(Arc::from(Path::new(tilde_substituted.as_ref())))
            }
        } else {
            this.active_project_directory(cx)
        };

        let is_ssh_terminal = ssh_details.is_some();

        let mut settings_location = None;
        if let Some(path) = path.as_ref()
            && let Some((worktree, _)) = this.find_worktree(path, cx)
        {
            settings_location = Some(SettingsLocation {
                worktree_id: worktree.read(cx).id(),
                path,
            });
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
        env.extend(settings.env);

        let local_path = if is_ssh_terminal { None } else { path.clone() };
        let toolchain =
            project_path_context.map(|p| self.active_toolchain(p, LanguageName::new("Python"), cx));
        cx.spawn(async move |project, cx| {
            let scripts = maybe!(async {
                let toolchain = toolchain?.await?;
                Some(toolchain.activation_script)
            })
            .await;
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

            let shell = {
                env.extend(spawn_task.env);
                // todo(lw): Use shell builder
                let shell = match &ssh_details {
                    Some(ssh) => ssh.shell.clone(),
                    None => match &settings.shell {
                        Shell::Program(program) => program.clone(),
                        Shell::WithArguments {
                            program,
                            args: _,
                            title_override: _,
                        } => program.clone(),
                        Shell::System => get_system_shell(),
                    },
                };
                let shell_kind = ShellKind::new(&shell);
                let activation_script = scripts.as_ref().and_then(|it| it.get(&shell_kind));

                match ssh_details {
                    Some(SshDetails {
                        host,
                        ssh_command,
                        envs,
                        path_style,
                        shell,
                    }) => {
                        log::debug!("Connecting to a remote server: {ssh_command:?}");
                        env.entry("TERM".to_string())
                            .or_insert_with(|| "xterm-256color".to_string());

                        let (program, args) = wrap_for_ssh(
                            &shell,
                            &ssh_command,
                            spawn_task
                                .command
                                .as_ref()
                                .map(|command| (command, &spawn_task.args)),
                            path.as_deref(),
                            env,
                            path_style,
                            activation_script.map(String::as_str),
                        );
                        env = HashMap::default();
                        if let Some(envs) = envs {
                            env.extend(envs);
                        }
                        Shell::WithArguments {
                            program,
                            args,
                            title_override: Some(format!("{} — Terminal", host).into()),
                        }
                    }
                    None => match activation_script {
                        Some(activation_script) => {
                            let to_run = if let Some(command) = spawn_task.command {
                                let command: Option<Cow<str>> = shlex::try_quote(&command).ok();
                                let args = spawn_task
                                    .args
                                    .iter()
                                    .filter_map(|arg| shlex::try_quote(arg).ok());
                                command.into_iter().chain(args).join(" ")
                            } else {
                                format!("exec {shell} -l")
                            };
                            Shell::WithArguments {
                                program: shell,
                                args: vec![
                                    "-c".to_owned(),
                                    format!("{activation_script}; {to_run}",),
                                ],
                                title_override: None,
                            }
                        }
                        None => {
                            if let Some(program) = spawn_task.command {
                                Shell::WithArguments {
                                    program,
                                    args: spawn_task.args,
                                    title_override: None,
                                }
                            } else {
                                Shell::System
                            }
                        }
                    },
                }
            };
            project.update(cx, move |this, cx| {
                TerminalBuilder::new(
                    local_path.map(|path| path.to_path_buf()),
                    task_state,
                    shell,
                    env,
                    settings.cursor_shape.unwrap_or_default(),
                    settings.alternate_scroll,
                    settings.max_scroll_history_lines,
                    is_ssh_terminal,
                    cx.entity_id().as_u64(),
                    Some(completion_tx),
                    cx,
                    None,
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

                    terminal_handle
                })
            })?
        })
    }

    pub fn create_terminal_shell(
        &mut self,
        cwd: Option<PathBuf>,
        cx: &mut Context<Self>,
        project_path_context: Option<ProjectPath>,
    ) -> Task<Result<Entity<Terminal>>> {
        let path = cwd.map(|p| Arc::from(&*p));
        let this = &mut *self;
        let ssh_details = this.ssh_details(cx);

        let is_ssh_terminal = ssh_details.is_some();

        let mut settings_location = None;
        if let Some(path) = path.as_ref()
            && let Some((worktree, _)) = this.find_worktree(path, cx)
        {
            settings_location = Some(SettingsLocation {
                worktree_id: worktree.read(cx).id(),
                path,
            });
        }
        let settings = TerminalSettings::get(settings_location, cx).clone();

        // Start with the environment that we might have inherited from the Zed CLI.
        let mut env = this
            .environment
            .read(cx)
            .get_cli_environment()
            .unwrap_or_default();
        // Then extend it with the explicit env variables from the settings, so they take
        // precedence.
        env.extend(settings.env);

        let local_path = if is_ssh_terminal { None } else { path.clone() };
        let toolchain =
            project_path_context.map(|p| self.active_toolchain(p, LanguageName::new("Python"), cx));
        cx.spawn(async move |project, cx| {
            // todo(lw): Use shell builder
            let shell = match &ssh_details {
                Some(ssh) => ssh.shell.clone(),
                None => match &settings.shell {
                    Shell::Program(program) => program.clone(),
                    Shell::WithArguments {
                        program,
                        args: _,
                        title_override: _,
                    } => program.clone(),
                    Shell::System => get_system_shell(),
                },
            };
            let shell_kind = ShellKind::new(&shell);

            let scripts = maybe!(async {
                let toolchain = toolchain?.await?;
                Some(toolchain.activation_script)
            })
            .await;
            let activation_script = scripts.as_ref().and_then(|it| it.get(&shell_kind));
            let shell = {
                match ssh_details {
                    Some(SshDetails {
                        host,
                        ssh_command,
                        envs,
                        path_style,
                        shell: _,
                    }) => {
                        log::debug!("Connecting to a remote server: {ssh_command:?}");

                        // Alacritty sets its terminfo to `alacritty`, this requiring hosts to have it installed
                        // to properly display colors.
                        // We do not have the luxury of assuming the host has it installed,
                        // so we set it to a default that does not break the highlighting via ssh.
                        env.entry("TERM".to_string())
                            .or_insert_with(|| "xterm-256color".to_string());

                        let (program, args) = wrap_for_ssh(
                            &shell,
                            &ssh_command,
                            None,
                            path.as_deref(),
                            env,
                            path_style,
                            activation_script.map(String::as_str),
                        );
                        env = HashMap::default();
                        if let Some(envs) = envs {
                            env.extend(envs);
                        }
                        Shell::WithArguments {
                            program,
                            args,
                            title_override: Some(format!("{} — Terminal", host).into()),
                        }
                    }
                    None => match activation_script {
                        Some(activation_script) => Shell::WithArguments {
                            program: shell.clone(),
                            args: vec![
                                "-c".to_owned(),
                                format!("{activation_script}; exec {shell} -l",),
                            ],
                            title_override: None,
                        },
                        None => settings.shell,
                    },
                }
            };
            project.update(cx, move |this, cx| {
                TerminalBuilder::new(
                    local_path.map(|path| path.to_path_buf()),
                    None,
                    shell,
                    env,
                    settings.cursor_shape.unwrap_or_default(),
                    settings.alternate_scroll,
                    settings.max_scroll_history_lines,
                    is_ssh_terminal,
                    cx.entity_id().as_u64(),
                    None,
                    cx,
                    None,
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

                    terminal_handle
                })
            })?
        })
    }

    pub fn clone_terminal(
        &mut self,
        terminal: &Entity<Terminal>,
        cx: &mut Context<'_, Project>,
        cwd: impl FnOnce() -> Option<PathBuf>,
    ) -> Result<Entity<Terminal>> {
        terminal.read(cx).clone_builder(cx, cwd).map(|builder| {
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

            terminal_handle
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

    pub fn exec_in_shell(&self, command: String, cx: &App) -> std::process::Command {
        let path = self.first_project_directory(cx);
        let ssh_details = self.ssh_details(cx);
        let settings = self.terminal_settings(&path, cx).clone();

        let builder =
            ShellBuilder::new(ssh_details.as_ref().map(|ssh| &*ssh.shell), &settings.shell)
                .non_interactive();
        let (command, args) = builder.build(Some(command), &Vec::new());

        let mut env = self
            .environment
            .read(cx)
            .get_cli_environment()
            .unwrap_or_default();
        env.extend(settings.env);

        match self.ssh_details(cx) {
            Some(SshDetails {
                ssh_command,
                envs,
                path_style,
                shell,
                ..
            }) => {
                let (command, args) = wrap_for_ssh(
                    &shell,
                    &ssh_command,
                    Some((&command, &args)),
                    path.as_deref(),
                    env,
                    path_style,
                    None,
                );
                let mut command = std::process::Command::new(command);
                command.args(args);
                if let Some(envs) = envs {
                    command.envs(envs);
                }
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

    pub fn local_terminal_handles(&self) -> &Vec<WeakEntity<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

pub fn wrap_for_ssh(
    shell: &str,
    ssh_command: &SshCommand,
    command: Option<(&String, &Vec<String>)>,
    path: Option<&Path>,
    env: HashMap<String, String>,
    path_style: PathStyle,
    activation_script: Option<&str>,
) -> (String, Vec<String>) {
    // todo make this shell aware
    let to_run = if let Some((command, args)) = command {
        let command: Option<Cow<str>> = shlex::try_quote(command).ok();
        let args = args.iter().filter_map(|arg| shlex::try_quote(arg).ok());
        command.into_iter().chain(args).join(" ")
    } else {
        format!("exec {shell} -l")
    };

    let mut env_changes = String::new();
    for (k, v) in env.iter() {
        if let Some((k, v)) = shlex::try_quote(k).ok().zip(shlex::try_quote(v).ok()) {
            env_changes.push_str(&format!("{}={} ", k, v));
        }
    }

    let activation_script = activation_script
        .map(|s| format!(" {s};"))
        .unwrap_or_default();
    let commands = if let Some(path) = path {
        let path = RemotePathBuf::new(path.to_path_buf(), path_style).to_string();
        // shlex will wrap the command in single quotes (''), disabling ~ expansion,
        // replace ith with something that works
        let tilde_prefix = "~/";
        if path.starts_with(tilde_prefix) {
            let trimmed_path = path
                .trim_start_matches("/")
                .trim_start_matches("~")
                .trim_start_matches("/");

            format!("cd \"$HOME/{trimmed_path}\";{activation_script} {env_changes} {to_run}")
        } else {
            format!("cd \"{path}\";{activation_script} {env_changes} {to_run}")
        }
    } else {
        format!("cd;{activation_script} {env_changes} {to_run}")
    };
    let shell_invocation = format!("{shell} -c {}", shlex::try_quote(&commands).unwrap());

    let program = "ssh".to_string();
    let mut args = ssh_command.arguments.clone();

    args.push("-t".to_string());
    args.push(shell_invocation);
    (program, args)
}

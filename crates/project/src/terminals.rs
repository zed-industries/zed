use anyhow::Result;
use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity};

use itertools::Itertools as _;
use language::LanguageName;
use remote::RemoteClient;
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
use util::{get_default_system_shell, get_system_shell, maybe};

use crate::{Project, ProjectPath};

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakEntity<terminal::Terminal>>,
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

    pub fn create_terminal_task(
        &mut self,
        spawn_task: SpawnInTerminal,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        let is_via_remote = self.remote_client.is_some();

        let path: Option<Arc<Path>> = if let Some(cwd) = &spawn_task.cwd {
            if is_via_remote {
                Some(Arc::from(cwd.as_ref()))
            } else {
                let cwd = cwd.to_string_lossy();
                let tilde_substituted = shellexpand::tilde(&cwd);
                Some(Arc::from(Path::new(tilde_substituted.as_ref())))
            }
        } else {
            self.active_project_directory(cx)
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
        let detect_venv = settings.detect_venv.as_option().is_some();

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
        let remote_client = self.remote_client.clone();
        let shell = match &remote_client {
            Some(remote_client) => remote_client
                .read(cx)
                .shell()
                .unwrap_or_else(get_default_system_shell),
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

        let project_path_contexts = self
            .active_entry()
            .and_then(|entry_id| self.path_for_entry(entry_id, cx))
            .into_iter()
            .chain(
                self.visible_worktrees(cx)
                    .map(|wt| wt.read(cx).id())
                    .map(|worktree_id| ProjectPath {
                        worktree_id,
                        path: Arc::from(Path::new("")),
                    }),
            );
        let toolchains = project_path_contexts
            .filter(|_| detect_venv)
            .map(|p| self.active_toolchain(p, LanguageName::new("Python"), cx))
            .collect::<Vec<_>>();
        let lang_registry = self.languages.clone();
        let fs = self.fs.clone();
        cx.spawn(async move |project, cx| {
            let activation_script = maybe!(async {
                for toolchain in toolchains {
                    let Some(toolchain) = toolchain.await else {
                        continue;
                    };
                    let language = lang_registry
                        .language_for_name(&toolchain.language_name.0)
                        .await
                        .ok();
                    let lister = language?.toolchain_lister();
                    return Some(
                        lister?
                            .activation_script(&toolchain, ShellKind::new(&shell), fs.as_ref())
                            .await,
                    );
                }
                None
            })
            .await
            .unwrap_or_default();

            project.update(cx, move |this, cx| {
                let shell = {
                    env.extend(spawn_task.env);
                    match remote_client {
                        Some(remote_client) => match activation_script.clone() {
                            activation_script if !activation_script.is_empty() => {
                                let activation_script = activation_script.join("; ");
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
                                let args = vec![
                                    "-c".to_owned(),
                                    format!("{activation_script}; {to_run}",),
                                ];
                                create_remote_shell(
                                    Some((&shell, &args)),
                                    &mut env,
                                    path,
                                    remote_client,
                                    cx,
                                )?
                            }
                            _ => create_remote_shell(
                                spawn_task
                                    .command
                                    .as_ref()
                                    .map(|command| (command, &spawn_task.args)),
                                &mut env,
                                path,
                                remote_client,
                                cx,
                            )?,
                        },
                        None => match activation_script.clone() {
                            #[cfg(not(target_os = "windows"))]
                            activation_script if !activation_script.is_empty() => {
                                let activation_script = activation_script.join("; ");
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
                            _ => {
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
                TerminalBuilder::new(
                    local_path.map(|path| path.to_path_buf()),
                    task_state,
                    shell,
                    env,
                    settings.cursor_shape.unwrap_or_default(),
                    settings.alternate_scroll,
                    settings.max_scroll_history_lines,
                    is_via_remote,
                    cx.entity_id().as_u64(),
                    Some(completion_tx),
                    cx,
                    activation_script,
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
    ) -> Task<Result<Entity<Terminal>>> {
        let path = cwd.map(|p| Arc::from(&*p));
        let is_via_remote = self.remote_client.is_some();

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
        let detect_venv = settings.detect_venv.as_option().is_some();

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

        let project_path_contexts = self
            .active_entry()
            .and_then(|entry_id| self.path_for_entry(entry_id, cx))
            .into_iter()
            .chain(
                self.visible_worktrees(cx)
                    .map(|wt| wt.read(cx).id())
                    .map(|worktree_id| ProjectPath {
                        worktree_id,
                        path: Arc::from(Path::new("")),
                    }),
            );
        let toolchains = project_path_contexts
            .filter(|_| detect_venv)
            .map(|p| self.active_toolchain(p, LanguageName::new("Python"), cx))
            .collect::<Vec<_>>();
        let remote_client = self.remote_client.clone();
        let shell = match &remote_client {
            Some(remote_client) => remote_client
                .read(cx)
                .shell()
                .unwrap_or_else(get_default_system_shell),
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

        let lang_registry = self.languages.clone();
        let fs = self.fs.clone();
        cx.spawn(async move |project, cx| {
            let activation_script = maybe!(async {
                for toolchain in toolchains {
                    let Some(toolchain) = toolchain.await else {
                        continue;
                    };
                    let language = lang_registry
                        .language_for_name(&toolchain.language_name.0)
                        .await
                        .ok();
                    let lister = language?.toolchain_lister();
                    return Some(
                        lister?
                            .activation_script(&toolchain, ShellKind::new(&shell), fs.as_ref())
                            .await,
                    );
                }
                None
            })
            .await
            .unwrap_or_default();
            project.update(cx, move |this, cx| {
                let shell = {
                    match remote_client {
                        Some(remote_client) => {
                            create_remote_shell(None, &mut env, path, remote_client, cx)?
                        }
                        None => settings.shell,
                    }
                };
                TerminalBuilder::new(
                    local_path.map(|path| path.to_path_buf()),
                    None,
                    shell,
                    env,
                    settings.cursor_shape.unwrap_or_default(),
                    settings.alternate_scroll,
                    settings.max_scroll_history_lines,
                    is_via_remote,
                    cx.entity_id().as_u64(),
                    None,
                    cx,
                    activation_script,
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
    let host = remote_client.read(cx).connection_options().display_name();

    Ok(Shell::WithArguments {
        program: command.program,
        args: command.args,
        title_override: Some(format!("{} — Terminal", host).into()),
    })
}

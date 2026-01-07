use anyhow::Result;
use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity};

use futures::{FutureExt, future::Shared};
use itertools::Itertools as _;
use language::LanguageName;
use remote::RemoteClient;
use settings::{Settings, SettingsLocation};
use smol::channel::bounded;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{Shell, ShellBuilder, ShellKind, SpawnInTerminal};
use terminal::{
    TaskState, TaskStatus, Terminal, TerminalBuilder, insert_zed_terminal_env,
    terminal_settings::TerminalSettings,
};
use util::{command::new_std_command, get_default_system_shell, maybe, rel_path::RelPath};

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
                path: RelPath::empty(),
            });
        }
        let settings = TerminalSettings::get(settings_location, cx).clone();
        let detect_venv = settings.detect_venv.as_option().is_some();

        let (completion_tx, completion_rx) = bounded(1);

        let local_path = if is_via_remote { None } else { path.clone() };
        let task_state = Some(TaskState {
            spawned_task: spawn_task.clone(),
            status: TaskStatus::Running,
            completion_rx,
        });
        let remote_client = self.remote_client.clone();
        let shell = match &remote_client {
            Some(remote_client) => remote_client
                .read(cx)
                .shell()
                .unwrap_or_else(get_default_system_shell),
            None => settings.shell.program(),
        };
        let is_windows = self.path_style(cx).is_windows();
        let shell_kind = ShellKind::new(&shell, is_windows);

        // Prepare a task for resolving the environment
        let env_task =
            self.resolve_directory_environment(&shell, path.clone(), remote_client.clone(), cx);

        let project_path_contexts = self
            .active_entry()
            .and_then(|entry_id| self.path_for_entry(entry_id, cx))
            .into_iter()
            .chain(
                self.visible_worktrees(cx)
                    .map(|wt| wt.read(cx).id())
                    .map(|worktree_id| ProjectPath {
                        worktree_id,
                        path: Arc::from(RelPath::empty()),
                    }),
            );
        let toolchains = project_path_contexts
            .filter(|_| detect_venv)
            .map(|p| self.active_toolchain(p, LanguageName::new_static("Python"), cx))
            .collect::<Vec<_>>();
        let lang_registry = self.languages.clone();
        cx.spawn(async move |project, cx| {
            let mut env = env_task.await.unwrap_or_default();
            env.extend(settings.env);

            let activation_script = maybe!(async {
                for toolchain in toolchains {
                    let Some(toolchain) = toolchain.await else {
                        continue;
                    };
                    let language = lang_registry
                        .language_for_name(&toolchain.language_name.0)
                        .await
                        .ok();
                    let lister = language?.toolchain_lister()?;
                    return cx
                        .update(|cx| lister.activation_script(&toolchain, shell_kind, cx))
                        .ok();
                }
                None
            })
            .await
            .unwrap_or_default();

            let builder = project
                .update(cx, move |_, cx| {
                    let format_to_run = || {
                        if let Some(command) = &spawn_task.command {
                            let command = shell_kind.prepend_command_prefix(command);
                            let command = shell_kind.try_quote_prefix_aware(&command);
                            let args = spawn_task
                                .args
                                .iter()
                                .filter_map(|arg| shell_kind.try_quote(&arg));

                            command.into_iter().chain(args).join(" ")
                        } else {
                            // todo: this breaks for remotes to windows
                            format!("exec {shell} -l")
                        }
                    };

                    let (shell, env) = {
                        env.extend(spawn_task.env);
                        match remote_client {
                            Some(remote_client) => match activation_script.clone() {
                                activation_script if !activation_script.is_empty() => {
                                    let separator = shell_kind.sequential_commands_separator();
                                    let activation_script =
                                        activation_script.join(&format!("{separator} "));
                                    let to_run = format_to_run();

                                    let arg = format!("{activation_script}{separator} {to_run}");
                                    let args = shell_kind.args_for_shell(false, arg);
                                    let shell = remote_client
                                        .read(cx)
                                        .shell()
                                        .unwrap_or_else(get_default_system_shell);

                                    create_remote_shell(
                                        Some((&shell, &args)),
                                        env,
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
                                    env,
                                    path,
                                    remote_client,
                                    cx,
                                )?,
                            },
                            None => match activation_script.clone() {
                                activation_script if !activation_script.is_empty() => {
                                    let separator = shell_kind.sequential_commands_separator();
                                    let activation_script =
                                        activation_script.join(&format!("{separator} "));
                                    let to_run = format_to_run();

                                    let mut arg =
                                        format!("{activation_script}{separator} {to_run}");
                                    if shell_kind == ShellKind::Cmd {
                                        // We need to put the entire command in quotes since otherwise CMD tries to execute them
                                        // as separate commands rather than chaining one after another.
                                        arg = format!("\"{arg}\"");
                                    }

                                    let args = shell_kind.args_for_shell(false, arg);

                                    (
                                        Shell::WithArguments {
                                            program: shell,
                                            args,
                                            title_override: None,
                                        },
                                        env,
                                    )
                                }
                                _ => (
                                    if let Some(program) = spawn_task.command {
                                        Shell::WithArguments {
                                            program,
                                            args: spawn_task.args,
                                            title_override: None,
                                        }
                                    } else {
                                        Shell::System
                                    },
                                    env,
                                ),
                            },
                        }
                    };
                    anyhow::Ok(TerminalBuilder::new(
                        local_path.map(|path| path.to_path_buf()),
                        task_state,
                        shell,
                        env,
                        settings.cursor_shape,
                        settings.alternate_scroll,
                        settings.max_scroll_history_lines,
                        settings.path_hyperlink_regexes,
                        settings.path_hyperlink_timeout_ms,
                        is_via_remote,
                        cx.entity_id().as_u64(),
                        Some(completion_tx),
                        cx,
                        activation_script,
                    ))
                })??
                .await?;
            project.update(cx, move |this, cx| {
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
                path: RelPath::empty(),
            });
        }
        let settings = TerminalSettings::get(settings_location, cx).clone();
        let detect_venv = settings.detect_venv.as_option().is_some();
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
                        path: RelPath::empty().into(),
                    }),
            );
        let toolchains = project_path_contexts
            .filter(|_| detect_venv)
            .map(|p| self.active_toolchain(p, LanguageName::new_static("Python"), cx))
            .collect::<Vec<_>>();
        let remote_client = self.remote_client.clone();
        let shell = match &remote_client {
            Some(remote_client) => remote_client
                .read(cx)
                .shell()
                .unwrap_or_else(get_default_system_shell),
            None => settings.shell.program(),
        };

        let is_windows = self.path_style(cx).is_windows();

        // Prepare a task for resolving the environment
        let env_task =
            self.resolve_directory_environment(&shell, path.clone(), remote_client.clone(), cx);

        let lang_registry = self.languages.clone();
        cx.spawn(async move |project, cx| {
            let shell_kind = ShellKind::new(&shell, is_windows);
            let mut env = env_task.await.unwrap_or_default();
            env.extend(settings.env);

            let activation_script = maybe!(async {
                for toolchain in toolchains {
                    let Some(toolchain) = toolchain.await else {
                        continue;
                    };
                    let language = lang_registry
                        .language_for_name(&toolchain.language_name.0)
                        .await
                        .ok();
                    let lister = language?.toolchain_lister()?;
                    return cx
                        .update(|cx| lister.activation_script(&toolchain, shell_kind, cx))
                        .ok();
                }
                None
            })
            .await
            .unwrap_or_default();

            let builder = project
                .update(cx, move |_, cx| {
                    let (shell, env) = {
                        match remote_client {
                            Some(remote_client) => {
                                create_remote_shell(None, env, path, remote_client, cx)?
                            }
                            None => (settings.shell, env),
                        }
                    };
                    anyhow::Ok(TerminalBuilder::new(
                        local_path.map(|path| path.to_path_buf()),
                        None,
                        shell,
                        env,
                        settings.cursor_shape,
                        settings.alternate_scroll,
                        settings.max_scroll_history_lines,
                        settings.path_hyperlink_regexes,
                        settings.path_hyperlink_timeout_ms,
                        is_via_remote,
                        cx.entity_id().as_u64(),
                        None,
                        cx,
                        activation_script,
                    ))
                })??
                .await?;
            project.update(cx, move |this, cx| {
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
        })
    }

    pub fn clone_terminal(
        &mut self,
        terminal: &Entity<Terminal>,
        cx: &mut Context<'_, Project>,
        cwd: Option<PathBuf>,
    ) -> Task<Result<Entity<Terminal>>> {
        // We cannot clone the task's terminal, as it will effectively re-spawn the task, which might not be desirable.
        // For now, create a new shell instead.
        if terminal.read(cx).task().is_some() {
            return self.create_terminal_shell(cwd, cx);
        }
        let local_path = if self.is_via_remote_server() {
            None
        } else {
            cwd
        };

        let builder = terminal.read(cx).clone_builder(cx, local_path);
        cx.spawn(async |project, cx| {
            let terminal = builder.await?;
            project.update(cx, |project, cx| {
                let terminal_handle = cx.new(|cx| terminal.subscribe(cx));

                project
                    .terminals
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
                path: RelPath::empty(),
            });
        }
        TerminalSettings::get(settings_location, cx)
    }

    pub fn exec_in_shell(
        &self,
        command: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<smol::process::Command>> {
        let path = self.first_project_directory(cx);
        let remote_client = self.remote_client.clone();
        let settings = self.terminal_settings(&path, cx).clone();
        let shell = remote_client
            .as_ref()
            .and_then(|remote_client| remote_client.read(cx).shell())
            .map(Shell::Program)
            .unwrap_or_else(|| settings.shell.clone());
        let is_windows = self.path_style(cx).is_windows();
        let builder = ShellBuilder::new(&shell, is_windows).non_interactive();
        let (command, args) = builder.build(Some(command), &Vec::new());

        let env_task = self.resolve_directory_environment(
            &shell.program(),
            path.as_ref().map(|p| Arc::from(&**p)),
            remote_client.clone(),
            cx,
        );

        cx.spawn(async move |project, cx| {
            let mut env = env_task.await.unwrap_or_default();
            env.extend(settings.env);

            project.update(cx, move |_, cx| {
                match remote_client {
                    Some(remote_client) => {
                        let command_template = remote_client.read(cx).build_command(
                            Some(command),
                            &args,
                            &env,
                            None,
                            None,
                        )?;
                        let mut command = new_std_command(command_template.program);
                        command.args(command_template.args);
                        command.envs(command_template.env);
                        Ok(command)
                    }
                    None => {
                        let mut command = new_std_command(command);
                        command.args(args);
                        command.envs(env);
                        if let Some(path) = path {
                            command.current_dir(path);
                        }
                        Ok(command)
                    }
                }
                .map(|mut process| {
                    util::set_pre_exec_to_start_new_session(&mut process);
                    smol::process::Command::from(process)
                })
            })?
        })
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakEntity<terminal::Terminal>> {
        &self.terminals.local_handles
    }

    fn resolve_directory_environment(
        &self,
        shell: &str,
        path: Option<Arc<Path>>,
        remote_client: Option<Entity<RemoteClient>>,
        cx: &mut App,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        if let Some(path) = &path {
            let shell = Shell::Program(shell.to_string());
            self.environment
                .update(cx, |project_env, cx| match &remote_client {
                    Some(remote_client) => project_env.remote_directory_environment(
                        &shell,
                        path.clone(),
                        remote_client.clone(),
                        cx,
                    ),
                    None => project_env.local_directory_environment(&shell, path.clone(), cx),
                })
        } else {
            Task::ready(None).shared()
        }
    }
}

fn create_remote_shell(
    spawn_command: Option<(&String, &Vec<String>)>,
    mut env: HashMap<String, String>,
    working_directory: Option<Arc<Path>>,
    remote_client: Entity<RemoteClient>,
    cx: &mut App,
) -> Result<(Shell, HashMap<String, String>)> {
    insert_zed_terminal_env(&mut env, &release_channel::AppVersion::global(cx));

    let (program, args) = match spawn_command {
        Some((program, args)) => (Some(program.clone()), args),
        None => (None, &Vec::new()),
    };

    let command = remote_client.read(cx).build_command(
        program,
        args.as_slice(),
        &env,
        working_directory.map(|path| path.display().to_string()),
        None,
    )?;

    log::debug!("Connecting to a remote server: {:?}", command.program);
    let host = remote_client.read(cx).connection_options().display_name();

    Ok((
        Shell::WithArguments {
            program: command.program,
            args: command.args,
            title_override: Some(format!("{} — Terminal", host)),
        },
        command.env,
    ))
}

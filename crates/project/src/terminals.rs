use anyhow::Result;
use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity};

use async_channel::bounded;
use futures::{FutureExt, future::Shared};
use itertools::Itertools as _;
use language::LanguageName;
use remote::{CommandTemplate, Interactive, RemoteClient};
use settings::{Settings, SettingsLocation};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{Shell, ShellBuilder, ShellKind, SpawnInTerminal};
use terminal::{
    TaskState, TaskStatus, Terminal, TerminalBuilder, insert_zed_terminal_env,
    terminal_settings::TerminalSettings,
};
use util::{
    ResultExt as _, command::new_std_command, get_default_system_shell, get_system_shell, maybe,
    rel_path::RelPath,
};

use crate::{Project, ProjectPath, project_settings::ProjectSettings};

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakEntity<terminal::Terminal>>,
}

impl Project {
    pub fn active_entry_directory(&self, cx: &App) -> Option<PathBuf> {
        let entry_id = self.active_entry()?;
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        let worktree = worktree.read(cx);
        let entry = worktree.entry_for_id(entry_id)?;

        let absolute_path = worktree.absolutize(entry.path.as_ref());
        if entry.is_dir() {
            Some(absolute_path)
        } else {
            absolute_path.parent().map(|p| p.to_path_buf())
        }
    }

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
            None => get_system_shell(),
        };
        let path_style = self.path_style(cx);
        let shell_kind = ShellKind::new(&shell, path_style.is_windows());

        // Prepare a task for resolving the environment
        let env_task =
            self.resolve_directory_environment(&shell, path.clone(), remote_client.clone(), cx);

        // Scope the toolchain lookup to the worktree the terminal is being
        // spawned in. Previously this iterated the active editor's worktree
        // and then every visible worktree, so a Python toolchain persisted
        // for worktree A would leak into a terminal opened in worktree B and
        // inject (e.g.) `conda activate base` into a shell that has no
        // business with conda.
        let project_path_contexts: Vec<ProjectPath> = path
            .as_ref()
            .and_then(|p| self.find_worktree(p, cx))
            .map(|(worktree, relative_path)| ProjectPath {
                worktree_id: worktree.read(cx).id(),
                path: relative_path,
            })
            .into_iter()
            .collect();
        let toolchains = project_path_contexts
            .into_iter()
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
                    let future =
                        cx.update(|cx| lister.activation_script(&toolchain, shell_kind, cx));
                    return Some(future.await);
                }
                None
            })
            .await
            .unwrap_or_default();

            let builder = project
                .update(cx, move |_, cx| {
                    let format_to_run = |spawn_task: &SpawnInTerminal| {
                        format_task_for_activation(
                            spawn_task,
                            shell_kind,
                            &shell,
                            path_style.is_windows(),
                        )
                    };

                    let (shell, env) = {
                        let to_run =
                            (!activation_script.is_empty()).then(|| format_to_run(&spawn_task));
                        env.extend(spawn_task.env);
                        match remote_client {
                            Some(remote_client) => match activation_script.clone() {
                                activation_script if !activation_script.is_empty() => {
                                    let separator = shell_kind.sequential_commands_separator();
                                    let activation_script =
                                        activation_script.join(&format!("{separator} "));
                                    let to_run = to_run.expect("activation command was formatted");

                                    let arg = format!("{activation_script}{separator} {to_run}");
                                    let args = shell_kind.args_for_shell(true, arg);
                                    let shell = remote_client
                                        .read(cx)
                                        .shell()
                                        .unwrap_or_else(get_default_system_shell);

                                    create_remote_shell(
                                        Some((&shell, &args)),
                                        env,
                                        path,
                                        remote_client,
                                        None,
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
                                    None,
                                    cx,
                                )?,
                            },
                            None => match activation_script.clone() {
                                activation_script if !activation_script.is_empty() => {
                                    let separator = shell_kind.sequential_commands_separator();
                                    let activation_script =
                                        activation_script.join(&format!("{separator} "));
                                    let to_run = to_run.expect("activation command was formatted");

                                    let arg = format!("{activation_script}{separator} {to_run}");
                                    let args = shell_kind.args_for_shell(true, arg);

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
                        None,
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
                        path_style,
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
        self.create_terminal_shell_internal(cwd, None, false, cx)
    }

    pub fn create_terminal_shell_with_session(
        &mut self,
        cwd: Option<PathBuf>,
        persistent_session: Option<String>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        self.create_terminal_shell_internal(cwd, persistent_session, false, cx)
    }

    /// Creates a local terminal even if the project is remote.
    /// In remote projects: opens in Zed's launch directory (bypasses SSH).
    /// In local projects: opens in the project directory (same as regular terminals).
    pub fn create_local_terminal(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        let working_directory = if self.remote_client.is_some() {
            // Remote project: don't use remote paths, let shell use Zed's cwd
            None
        } else {
            // Local project: use project directory like normal terminals
            self.active_project_directory(cx).map(|p| p.to_path_buf())
        };
        self.create_terminal_shell_internal(working_directory, None, true, cx)
    }

    /// Internal method for creating terminal shells.
    /// If force_local is true, creates a local terminal even if the project has a remote client.
    /// This allows "breaking out" to a local shell in remote projects.
    fn create_terminal_shell_internal(
        &mut self,
        cwd: Option<PathBuf>,
        persistent_session: Option<String>,
        force_local: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        let path = cwd.map(|p| Arc::from(&*p));
        let is_via_remote = !force_local && self.remote_client.is_some();

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

        // See create_terminal_task: scope the toolchain lookup to the
        // worktree the terminal is opened in, not the active editor's
        // worktree or other visible worktrees.
        let project_path_contexts: Vec<ProjectPath> = path
            .as_ref()
            .and_then(|p| self.find_worktree(p, cx))
            .map(|(worktree, relative_path)| ProjectPath {
                worktree_id: worktree.read(cx).id(),
                path: relative_path,
            })
            .into_iter()
            .collect();
        let toolchains = project_path_contexts
            .into_iter()
            .filter(|_| detect_venv)
            .map(|p| self.active_toolchain(p, LanguageName::new_static("Python"), cx))
            .collect::<Vec<_>>();
        let remote_client = if force_local {
            None
        } else {
            self.remote_client.clone()
        };
        let continuity_enabled = ProjectSettings::get_global(cx).remote_session_continuity;
        let connection = remote_client.as_ref().map(|remote_client| {
            let client = remote_client.read(cx);
            (client.is_workspace_connection(), client.unique_identifier())
        });
        let persistent_session =
            resolve_persistent_session(continuity_enabled, persistent_session, connection);
        let shell = match &remote_client {
            Some(remote_client) => remote_client
                .read(cx)
                .shell()
                .unwrap_or_else(get_default_system_shell),
            None => settings.shell.program(),
        };
        let env_shell = match &remote_client {
            Some(_) => shell.clone(),
            None => get_system_shell(),
        };

        let path_style = self.path_style(cx);

        // Prepare a task for resolving the environment
        let env_task =
            self.resolve_directory_environment(&env_shell, path.clone(), remote_client.clone(), cx);

        let lang_registry = self.languages.clone();
        cx.spawn(async move |project, cx| {
            let shell_kind = ShellKind::new(&shell, path_style.is_windows());
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
                    let future =
                        cx.update(|cx| lister.activation_script(&toolchain, shell_kind, cx));
                    return Some(future.await);
                }
                None
            })
            .await
            .unwrap_or_default();

            let builder = project
                .update(cx, move |_, cx| {
                    let (shell, env) = {
                        match remote_client {
                            Some(remote_client) => create_remote_shell(
                                None,
                                env,
                                path,
                                remote_client,
                                persistent_session.as_deref(),
                                cx,
                            )?,
                            None => (settings.shell, env),
                        }
                    };
                    anyhow::Ok(TerminalBuilder::new(
                        local_path.map(|path| path.to_path_buf()),
                        None,
                        shell,
                        persistent_session,
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
                        path_style,
                    ))
                })??
                .await?;
            project.update(cx, move |this, cx| {
                let terminal_handle = cx.new(|cx| builder.subscribe(cx));

                this.terminals
                    .local_handles
                    .push(terminal_handle.downgrade());

                let id = terminal_handle.entity_id();
                cx.observe_release(&terminal_handle, move |project, terminal, cx| {
                    let handles = &mut project.terminals.local_handles;

                    if let Some(index) = handles
                        .iter()
                        .position(|terminal| terminal.entity_id() == id)
                    {
                        handles.remove(index);
                        cx.notify();
                    }

                    // Best-effort only: on a Shut-down close, the awaited
                    // sweep in Workspace::prepare_to_close is the correctness
                    // guarantee, not this detached kill.
                    if project.shutdown_remote_server_on_close
                        && let Some(session) = terminal.persistent_session()
                        && let Some(task) =
                            project.kill_persistent_terminal_session(session.to_string(), cx)
                    {
                        task.detach();
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
        // Terminals wrapped in a persistent tmux session cannot be cloned either: the
        // cloned PTY would re-run `tmux new-session -A` with the same session name and
        // attach to the same session, mirroring the original terminal.
        if terminal.read(cx).task().is_some() || terminal.read(cx).persistent_session().is_some() {
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
            .unwrap_or(Shell::System);
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
                            Interactive::Yes,
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

    /// Kills a single persistent tmux session on the remote host by running
    /// `tmux kill-session` through the same ssh invocation used to attach to it.
    pub fn kill_persistent_terminal_session(&self, session: String, cx: &App) -> Option<Task<()>> {
        let command_template = self
            .remote_client
            .as_ref()?
            .read(cx)
            .build_command(
                Some("tmux".to_string()),
                &tmux_kill_session_args(&session),
                &HashMap::default(),
                None,
                None,
                Interactive::No,
            )
            .log_err()?;
        Some(cx.background_spawn(run_remote_command(command_template)))
    }

    /// Kills every persistent tmux session on the remote host belonging to this
    /// workspace connection, identified by the `zed-{unique_identifier}-` prefix.
    /// Used when the user chooses "Shut down" on disconnect, so no sessions are
    /// left running once the connection is torn down.
    pub fn kill_all_persistent_terminal_sessions(&self, cx: &App) -> Option<Task<()>> {
        let client = self.remote_client.as_ref()?.read(cx);
        let prefix = format!("zed-{}-", client.unique_identifier());
        let command_template = client
            .build_command(
                Some("sh".to_string()),
                &["-c".to_string(), tmux_sweep_script(&prefix)],
                &HashMap::default(),
                None,
                None,
                Interactive::No,
            )
            .log_err()?;
        Some(cx.background_spawn(run_remote_command(command_template)))
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

fn persistent_session_name(identifier: &str) -> String {
    format!("zed-{}-{}", identifier, uuid::Uuid::new_v4().simple())
}

/// Decides which persistent tmux session (if any) a terminal should attach
/// to. `connection` is `Some((is_workspace_connection, unique_identifier))`
/// for a remote terminal, `None` for a local one. A `requested` session name
/// restored from the database only makes sense for a remote terminal, so it
/// is ignored without a connection; this also guards against a stale row
/// attaching a local terminal to a (local) tmux session.
fn resolve_persistent_session(
    continuity_enabled: bool,
    requested: Option<String>,
    connection: Option<(bool, String)>,
) -> Option<String> {
    if !continuity_enabled {
        return None;
    }
    let connection = connection?;
    requested.or_else(|| {
        let (is_workspace_connection, identifier) = connection;
        is_workspace_connection.then(|| persistent_session_name(&identifier))
    })
}

fn tmux_kill_session_args(session: &str) -> Vec<String> {
    vec![
        "-L".to_string(),
        "zed".to_string(),
        "kill-session".to_string(),
        "-t".to_string(),
        session.to_string(),
    ]
}

fn tmux_sweep_script(prefix: &str) -> String {
    // awk's `index() == 1` is an exact literal prefix match, unlike grep,
    // whose patterns would treat regex metacharacters in the prefix specially.
    format!(
        "tmux -L zed list-sessions -F '#S' 2>/dev/null | awk -v p='{prefix}' 'index($0, p) == 1' | xargs -r -n1 tmux -L zed kill-session -t"
    )
}

async fn run_remote_command(command: CommandTemplate) {
    let mut process = new_std_command(command.program);
    process.args(command.args);
    process.envs(command.env);
    smol::process::Command::from(process)
        .output()
        .await
        .log_err();
}

fn tmux_wrapped_shell_args(session: &str) -> (String, Vec<String>) {
    (
        "sh".to_string(),
        vec![
            "-c".to_string(),
            format!(
                "command -v tmux >/dev/null 2>&1 && \
                 exec tmux -L zed new-session -A -s {session} \\; set status off || \
                 exec \"$SHELL\" -l"
            ),
        ],
    )
}

fn create_remote_shell(
    spawn_command: Option<(&String, &Vec<String>)>,
    mut env: HashMap<String, String>,
    working_directory: Option<Arc<Path>>,
    remote_client: Entity<RemoteClient>,
    persistent_session: Option<&str>,
    cx: &mut App,
) -> Result<(Shell, HashMap<String, String>)> {
    insert_zed_terminal_env(&mut env, &release_channel::AppVersion::global(cx));

    let (program, args) = match spawn_command {
        Some((program, args)) => (Some(program.clone()), args.clone()),
        None => match persistent_session {
            Some(session) => {
                let (program, args) = tmux_wrapped_shell_args(session);
                (Some(program), args)
            }
            None => (None, Vec::new()),
        },
    };

    let command = remote_client.read(cx).build_command(
        program,
        args.as_slice(),
        &env,
        working_directory.map(|path| path.display().to_string()),
        None,
        Interactive::Yes,
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

fn format_task_for_activation(
    spawn_task: &SpawnInTerminal,
    shell_kind: ShellKind,
    shell: &str,
    is_windows: bool,
) -> String {
    if let Some(command) = &spawn_task.command {
        let command = shell_kind.prepend_command_prefix(command);
        let command = shell_kind.try_quote_prefix_aware(&command);
        let args = spawn_task
            .args
            .iter()
            .enumerate()
            .filter_map(|(index, arg)| {
                quote_prepared_task_arg_for_activation(
                    spawn_task, shell_kind, arg, index, is_windows,
                )
            });

        command.into_iter().chain(args).join(" ")
    } else {
        // todo: this breaks for remotes to windows
        format!("exec {shell} -l")
    }
}

fn quote_prepared_task_arg_for_activation<'a>(
    spawn_task: &SpawnInTerminal,
    shell_kind: ShellKind,
    arg: &'a str,
    index: usize,
    is_windows: bool,
) -> Option<Cow<'a, str>> {
    if spawn_task.shell.shell_kind(is_windows) == ShellKind::Cmd
        && index >= 2
        && spawn_task
            .args
            .get(index - 2)
            .is_some_and(|arg| arg.eq_ignore_ascii_case("/S"))
        && spawn_task
            .args
            .get(index - 1)
            .is_some_and(|arg| arg.eq_ignore_ascii_case("/C"))
    {
        // The /C argument is already a cmd command string from prepare_task_for_spawn.
        // Quoting it again for venv activation makes cmd see the quotes as literals.
        return quote_cmd_command_arg_for_outer_shell(arg, shell_kind).map(Cow::Owned);
    }

    shell_kind.try_quote(arg)
}

fn quote_cmd_command_arg_for_outer_shell(arg: &str, shell_kind: ShellKind) -> Option<String> {
    match shell_kind {
        ShellKind::PowerShell | ShellKind::Pwsh => Some(format!("'{}'", arg.replace('\'', "''"))),
        ShellKind::Cmd => Some(arg.to_string()),
        ShellKind::Posix
        | ShellKind::Csh
        | ShellKind::Tcsh
        | ShellKind::Fish
        | ShellKind::Nushell
        | ShellKind::Rc
        | ShellKind::Xonsh
        | ShellKind::Elvish => shell_kind.try_quote(arg).map(Cow::into_owned),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn prepared_cmd_task(command_arg: &str) -> SpawnInTerminal {
        SpawnInTerminal {
            command: Some("cmd.exe".to_string()),
            args: vec!["/S".to_string(), "/C".to_string(), command_arg.to_string()],
            shell: Shell::Program("cmd.exe".to_string()),
            ..SpawnInTerminal::default()
        }
    }

    #[test]
    fn formats_prepared_cmd_task_for_powershell_activation() {
        let task = prepared_cmd_task("\"echo Hi there\"");

        assert_eq!(
            format_task_for_activation(&task, ShellKind::PowerShell, "powershell.exe", true),
            "&cmd.exe /S /C '\"echo Hi there\"'"
        );
    }

    #[test]
    fn formats_prepared_cmd_task_for_cmd_activation() {
        let task = prepared_cmd_task("\"echo Hi there\"");

        assert_eq!(
            format_task_for_activation(&task, ShellKind::Cmd, "cmd.exe", true),
            "cmd.exe /S /C \"echo Hi there\""
        );
    }

    #[test]
    fn formats_prepared_cmd_task_with_shell_args_for_activation() {
        let task = SpawnInTerminal {
            command: Some("cmd.exe".to_string()),
            args: vec![
                "/D".to_string(),
                "/S".to_string(),
                "/C".to_string(),
                "\"echo Hi there\"".to_string(),
            ],
            shell: Shell::WithArguments {
                program: "cmd.exe".to_string(),
                args: vec!["/D".to_string()],
                title_override: None,
            },
            ..SpawnInTerminal::default()
        };

        assert_eq!(
            format_task_for_activation(&task, ShellKind::PowerShell, "powershell.exe", true),
            "&cmd.exe /D /S /C '\"echo Hi there\"'"
        );
    }

    #[test]
    fn formats_prepared_cmd_task_with_single_quote_for_powershell_activation() {
        let task = prepared_cmd_task("\"echo It's fine\"");

        assert_eq!(
            format_task_for_activation(&task, ShellKind::PowerShell, "powershell.exe", true),
            "&cmd.exe /S /C '\"echo It''s fine\"'"
        );
    }

    #[test]
    fn tmux_wrapped_shell_args_produces_expected_command() {
        let (program, args) =
            tmux_wrapped_shell_args("zed-dev-workspace-5-9f8c1234abcd4e5f6789abcdef012345");

        assert_eq!(program, "sh");
        assert_eq!(
            args,
            vec![
                "-c".to_string(),
                r#"command -v tmux >/dev/null 2>&1 && exec tmux -L zed new-session -A -s zed-dev-workspace-5-9f8c1234abcd4e5f6789abcdef012345 \; set status off || exec "$SHELL" -l"#
                    .to_string(),
            ]
        );
    }

    #[test]
    fn tmux_kill_session_args_produce_expected_command() {
        assert_eq!(
            tmux_kill_session_args("zed-dev-workspace-abc123-1a2b3c4d5e6f7890abcdef1234567890"),
            vec![
                "-L".to_string(),
                "zed".to_string(),
                "kill-session".to_string(),
                "-t".to_string(),
                "zed-dev-workspace-abc123-1a2b3c4d5e6f7890abcdef1234567890".to_string(),
            ]
        );
    }

    #[test]
    fn tmux_sweep_script_produces_expected_command() {
        assert_eq!(
            tmux_sweep_script("zed-dev-workspace-abc123-"),
            "tmux -L zed list-sessions -F '#S' 2>/dev/null | \
             awk -v p='zed-dev-workspace-abc123-' 'index($0, p) == 1' | \
             xargs -r -n1 tmux -L zed kill-session -t"
        );
    }

    #[test]
    fn persistent_session_name_has_expected_format() {
        let name = persistent_session_name("dev-workspace-5");

        let uuid_part = name
            .strip_prefix("zed-dev-workspace-5-")
            .expect("session name should start with zed-{identifier}-");
        assert_eq!(uuid_part.len(), 32);
        assert!(
            uuid_part
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        );
        assert!(!name.contains('.') && !name.contains(':'));
    }

    #[test]
    fn resolve_persistent_session_ignores_requested_session_when_continuity_disabled() {
        assert_eq!(
            resolve_persistent_session(
                false,
                Some("x".to_string()),
                Some((true, "id".to_string())),
            ),
            None
        );
    }

    #[test]
    fn resolve_persistent_session_does_not_generate_when_continuity_disabled() {
        assert_eq!(
            resolve_persistent_session(false, None, Some((true, "id".to_string()))),
            None
        );
    }

    #[test]
    fn resolve_persistent_session_honors_requested_session_when_connected() {
        assert_eq!(
            resolve_persistent_session(true, Some("x".to_string()), Some((true, "id".to_string())),),
            Some("x".to_string())
        );
        assert_eq!(
            resolve_persistent_session(
                true,
                Some("x".to_string()),
                Some((false, "id".to_string())),
            ),
            Some("x".to_string())
        );
    }

    #[test]
    fn resolve_persistent_session_generates_name_for_workspace_connection() {
        let session = resolve_persistent_session(true, None, Some((true, "id".to_string())))
            .expect("a session name should be generated for a workspace connection");

        assert!(session.starts_with("zed-id-"));
    }

    #[test]
    fn resolve_persistent_session_does_not_generate_for_non_workspace_connection() {
        assert_eq!(
            resolve_persistent_session(true, None, Some((false, "id".to_string()))),
            None
        );
    }

    #[test]
    fn resolve_persistent_session_does_not_generate_without_a_connection() {
        assert_eq!(resolve_persistent_session(true, None, None), None);
    }

    #[test]
    fn formats_non_cmd_task_for_activation() {
        let task = SpawnInTerminal {
            command: Some("cargo".to_string()),
            args: vec!["test".to_string(), "some test".to_string()],
            shell: Shell::System,
            ..SpawnInTerminal::default()
        };

        assert_eq!(
            format_task_for_activation(&task, ShellKind::PowerShell, "powershell.exe", true),
            "&cargo test 'some test'"
        );
    }
}

use crate::Project;
use collections::HashMap;
use futures::{
    channel::mpsc::{self, UnboundedSender},
    StreamExt,
};
use gpui::{AnyWindowHandle, Context, Entity, Model, ModelContext, Task, WeakModel};
use rpc::proto;
use settings::Settings;
use smol::{channel::bounded, io::AsyncReadExt};
use std::{
    io::{ErrorKind, Read},
    num::NonZeroUsize,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};
use task::SpawnInTerminal;
use terminal::{
    alacritty_terminal::tty::EventedReadWrite,
    headless::{polling::Events, portable_pty::CommandBuilder, HeadlessTerminal},
    pty::RemotePty,
    terminal_settings::{self, Shell, TerminalSettings, VenvSettingsContent},
    TaskState, TaskStatus, Terminal, TerminalBuilder,
};
use util::ResultExt;

// #[cfg(target_os = "macos")]
// use std::os::unix::ffi::OsStrExt;

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakModel<terminal::Terminal>>,
    pub(crate) remote_handles: HashMap<u64, UnboundedSender<Vec<u8>>>,
    pub(crate) headless_handles: Vec<WeakModel<terminal::headless::HeadlessTerminal>>,
}

impl Project {
    pub fn create_terminal(
        &mut self,
        working_directory: Option<PathBuf>,
        spawn_task: Option<SpawnInTerminal>,
        window: AnyWindowHandle,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<Model<Terminal>>> {
        if self.is_remote() {
            println!("!!!is remote");

            let rpc = self.client();
            let project_id = self.remote_id().unwrap();
            // let settings = TerminalSettings::get_global(cx);

            cx.spawn(move |project, mut cx| async move {
                let (pty, host_tx, mut input_rx) = RemotePty::new(&cx).await;
                project.update(&mut cx, |this, _| {
                    this.terminals.remote_handles.insert(0, host_tx) // TODO: use real terminal id
                })?;

                let rpc2 = rpc.clone();
                cx.spawn(|_| async move {
                    loop {
                        let input = input_rx.next().await.unwrap();
                        rpc2.send(proto::InputRemoteTerminal {
                            project_id,
                            terminal_id: 0,
                            data: input,
                        })
                        .unwrap();
                    }

                    //
                })
                .detach();

                let response = rpc
                    .request(proto::CreateRemoteTerminal { project_id })
                    .await?;
                println!("!!!request done");

                println!("!!!pty done");

                let terminal = TerminalBuilder::new_remote(
                    None,
                    terminal_settings::AlternateScroll::Off,
                    None,
                    pty,
                )?;

                println!("!!!terminal created");
                let terminal_handle = cx
                    .new_model(|cx| terminal.subscribe(cx))
                    .map(|terminal_handle| terminal_handle);

                let weak_handle = terminal_handle.as_ref().map(|t| t.downgrade());
                if let Ok(terminal_handle) = weak_handle {
                    project.update(&mut cx, |this, cx| {
                        this.terminals.local_handles.push(terminal_handle);
                    })?;
                }

                println!("!!!terminal model created");
                terminal_handle
            })
        } else {
            Task::ready(self.create_local_terminal(working_directory, spawn_task, window, cx))
        }
    }

    pub fn create_local_terminal(
        &mut self,
        working_directory: Option<PathBuf>,
        spawn_task: Option<SpawnInTerminal>,
        window: AnyWindowHandle,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<Model<Terminal>> {
        let is_terminal = spawn_task.is_none();
        let settings = TerminalSettings::get_global(cx);
        let python_settings = settings.detect_venv.clone();
        let (completion_tx, completion_rx) = bounded(1);

        let mut env = settings.env.clone();
        // Alacritty uses parent project's working directory when no working directory is provided
        // https://github.com/alacritty/alacritty/blob/fd1a3cc79192d1d03839f0fd8c72e1f8d0fce42e/extra/man/alacritty.5.scd?plain=1#L47-L52

        let venv_base_directory = working_directory
            .as_deref()
            .unwrap_or_else(|| Path::new(""));

        let (spawn_task, shell) = if let Some(spawn_task) = spawn_task {
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

    pub fn create_headless_terminal(&mut self, cx: &mut ModelContext<Self>) -> anyhow::Result<()> {
        println!("create_headless_terminal");
        let rpc = self.client();

        let project_id = self.remote_id().unwrap();

        let terminal = HeadlessTerminal::new(None, None)?;
        let cmd = CommandBuilder::new("bash");

        // let task: Task<anyhow::Result<()>> = cx.spawn(|_, cx| async move {
        //     let tty = Arc::new(Mutex::new(terminal.tty));
        //     let poll = terminal.poll.clone();
        //     let mut buffer = [0; 1024];

        //     let mut events = Events::with_capacity(NonZeroUsize::new(1024).unwrap());

        //     'event_loop: loop {
        //         // Wakeup the event loop when a synchronized update timeout was reached.
        //         events.clear();
        //         if let Err(err) = poll.wait(&mut events, Some(Duration::from_secs(2))) {
        //             match err.kind() {
        //                 ErrorKind::Interrupted => continue,
        //                 _ => {
        //                     break 'event_loop;
        //                 }
        //             }
        //         }

        //         // Handle synchronized update timeout.
        //         if events.is_empty() {
        //             // state.parser.stop_sync(&mut *self.terminal.lock());
        //             // self.event_proxy.send_event(Event::Wakeup);
        //             continue;
        //         }

        //         for event in events.iter() {
        //             match event.key {
        //                 0 => {
        //                     if event.is_interrupt() {
        //                         // Don't try to do I/O on a dead PTY.
        //                         continue;
        //                     }

        //                     if event.readable {
        //                         let tty = tty.clone();
        //                         let n = smol::unblock(move || {
        //                             tty.lock().unwrap().reader().read(&mut buffer)
        //                         })
        //                         .await;
        //                         if let Err(e) = n {
        //                             if let ErrorKind::Interrupted | ErrorKind::WouldBlock = e.kind()
        //                             {
        //                                 continue;
        //                             } else {
        //                                 break;
        //                             }
        //                         }

        //                         let n = n.unwrap();
        //                         if n == 0 {
        //                             break;
        //                         }

        //                         println!("!!!Read {} bytes", n);
        //                         rpc.send(proto::UpdateRemoteTerminal {
        //                             terminal_id: 0,
        //                             project_id,
        //                             data: buffer[..n].to_vec(),
        //                         })?;
        //                         // if let Err(err) = self.pty_read(&mut state, &mut buf, pipe.as_mut())
        //                         // {
        //                         //     // On Linux, a `read` on the master side of a PTY can fail
        //                         //     // with `EIO` if the client side hangs up.  In that case,
        //                         //     // just loop back round for the inevitable `Exited` event.
        //                         //     // This sucks, but checking the process is either racy or
        //                         //     // blocking.
        //                         //     #[cfg(target_os = "linux")]
        //                         //     if err.raw_os_error() == Some(libc::EIO) {
        //                         //         continue;
        //                         //     }

        //                         //     error!("Error reading from PTY in event loop: {}", err);
        //                         //     break 'event_loop;
        //                         // }
        //                     }
        //                 }
        //                 _ => (),
        //             }
        //         }
        //     }

        //     // loop {
        //     //     let tty = tty.clone();
        //     //     let n = smol::unblock(move || tty.lock().unwrap().reader().read(&mut buffer)).await;
        //     //     if let Err(e) = n {
        //     //         if let ErrorKind::Interrupted | ErrorKind::WouldBlock = e.kind() {
        //     //             continue;
        //     //         } else {
        //     //             break;
        //     //         }
        //     //     }

        //     //     let n = n.unwrap();
        //     //     if n == 0 {
        //     //         break;
        //     //     }

        //     //     println!("!!!Read {} bytes", n);
        //     //     rpc.send(proto::UpdateRemoteTerminal {
        //     //         terminal_id: 0,
        //     //         project_id,
        //     //         data: buffer[..n].to_vec(),
        //     //     })?;
        //     // }
        //     Ok(())
        // });

        let pair = terminal.pair;
        let reader = pair.master.try_clone_reader()?;
        let mut writer = pair.master.take_writer()?;

        let read: Task<anyhow::Result<()>> = cx.spawn(|_, cx| async move {
            let mut child = smol::unblock(move || pair.slave.spawn_command(cmd)).await?;
            let mut reader = smol::Unblock::new(reader);
            let mut buffer = [0; 1024];
            loop {
                let n = reader.read(&mut buffer).await?;
                if n == 0 {
                    break;
                }

                println!("!!!Read {} bytes", n);
                rpc.send(proto::UpdateRemoteTerminal {
                    terminal_id: 0,
                    project_id,
                    data: buffer[..n].to_vec(),
                })?;
            }
            Ok(())
        });
        read.detach_and_log_err(cx);

        let (input_tx, mut input_rx) = mpsc::unbounded();
        let write = cx.spawn(|project, mut cx| async move {
            project
                .update(&mut cx, |project, cx| {
                    project.terminals.remote_handles.insert(0, input_tx);
                })
                .unwrap();

            loop {
                let input = input_rx.next().await.unwrap();
                writer.write_all(&input).unwrap();
            }
        });
        write.detach();
        // let mut reader = pair.master.try_clone_reader()?;
        // writeln!(pair.master.take_writer()?, "ls -l\r\n")?;

        Ok(())
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

// TODO: Add a few tests for adding and removing terminal tabs

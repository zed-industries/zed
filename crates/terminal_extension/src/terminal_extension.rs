use std::borrow::Cow;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use extension::{ExtensionHostProxy, ExtensionTerminalProxy};
use futures::StreamExt;
use gpui::{App, AsyncApp, BackgroundExecutor, Entity, Keystroke, Task, WeakEntity};
use project::Project;
use task::SpawnInTerminal;
use terminal::Terminal;
use terminal::mappings::keys::to_esc_str;

type MainThreadSender = futures::channel::mpsc::UnboundedSender<MainThreadCall>;
type MainThreadCall =
    Box<dyn Send + for<'a> FnOnce(&'a mut AsyncApp) -> futures::future::LocalBoxFuture<'a, ()>>;

/// Provides terminal creation and project access for the terminal extension.
/// Implementations should create terminals that are added to the UI (e.g., terminal panel).
pub trait TerminalProvider: Send + Sync + 'static {
    /// Returns the active project, if any.
    fn active_project(&self, cx: &App) -> Option<Entity<Project>>;

    /// Creates a terminal with the given options and adds it to the UI.
    /// This is called from the main thread with synchronous context.
    /// Returns a task that resolves to the weak terminal entity.
    fn create_terminal(
        &self,
        spawn_task: SpawnInTerminal,
        target_terminal: Option<WeakEntity<Terminal>>,
        activate: bool,
        cx: &mut App,
    ) -> Option<Task<Result<WeakEntity<Terminal>>>>;

    /// Closes a terminal and removes it from the UI.
    /// This is called from the main thread with synchronous context.
    fn close_terminal(&self, terminal: WeakEntity<Terminal>, cx: &mut App) -> Result<()>;

    /// Finds a terminal by its entity ID across all workspaces.
    /// Returns the weak entity if found.
    fn find_terminal_by_id(&self, entity_id: u64, cx: &App) -> Option<WeakEntity<Terminal>>;

    /// Lists all terminals across all workspaces, grouped by workspace.
    fn list_all_workspaces_terminals(&self, cx: &App) -> Vec<extension::WorkspaceTerminals>;

    /// Splits the pane containing the given terminal in the specified direction,
    /// creating a new terminal in the new pane.
    /// Returns the new terminal's weak entity.
    fn split_terminal(
        &self,
        terminal: WeakEntity<Terminal>,
        direction: extension::SplitDirection,
        spawn_task: SpawnInTerminal,
        cx: &mut App,
    ) -> Option<Task<Result<WeakEntity<Terminal>>>>;

    /// Gets the full layout tree of the terminal panel, including bounding boxes.
    /// The terminal handles are resolved using the handle_for_terminal callback.
    fn get_layout(
        &self,
        handle_for_terminal: &dyn Fn(&WeakEntity<Terminal>) -> Option<u64>,
        cx: &App,
    ) -> extension::PaneLayout;

    /// Focuses the pane containing the given terminal and activates it.
    fn focus_terminal(&self, terminal: WeakEntity<Terminal>, cx: &mut App) -> Result<()>;

    /// Reorganizes all terminals according to the specified layout mode.
    /// If caller_terminal_id is provided (from ZED_TERM_ID), the terminal with that entity_id
    /// will be auto-focused after layout changes (used by Consolidate).
    fn set_layout(
        &self,
        mode: extension::LayoutMode,
        caller_terminal_id: Option<u64>,
        cx: &mut App,
    ) -> Result<()>;

    /// Checks if a terminal is the active item in its pane.
    fn is_terminal_active(&self, terminal: &WeakEntity<Terminal>, cx: &App) -> bool;

    /// Moves a terminal to the pane containing another terminal.
    fn move_terminal(
        &self,
        source: WeakEntity<Terminal>,
        destination: WeakEntity<Terminal>,
        cx: &mut App,
    ) -> Result<()>;
}

pub fn init(
    extension_host_proxy: Arc<ExtensionHostProxy>,
    terminal_provider: Arc<dyn TerminalProvider>,
    cx: &mut App,
) {
    let (main_thread_tx, mut main_thread_rx) =
        futures::channel::mpsc::unbounded::<MainThreadCall>();

    cx.spawn(async move |cx| {
        while let Some(call) = main_thread_rx.next().await {
            call(cx).await;
        }
    })
    .detach();

    extension_host_proxy.register_terminal_proxy(TerminalExtensionProxy {
        terminal_provider,
        main_thread_tx,
        executor: cx.background_executor().clone(),
    });
}

struct TerminalExtensionProxy {
    terminal_provider: Arc<dyn TerminalProvider>,
    main_thread_tx: MainThreadSender,
    executor: BackgroundExecutor,
}

impl ExtensionTerminalProxy for TerminalExtensionProxy {
    fn create_terminal(
        &self,
        options: extension::TerminalOptions,
    ) -> Task<Result<extension::TerminalHandle>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();
        let title_override = options.title_override.clone();
        let in_pane_of_entity_id = options.in_pane_of;
        let activate = options.activate;

        let spawn_task = SpawnInTerminal {
            command: options.command,
            args: options.args,
            cwd: options.cwd,
            env: options.env.into_iter().collect(),
            ..Default::default()
        };

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result: Result<extension::TerminalHandle> = async {
                            // Resolve the target terminal by entity_id if specified
                            let target_terminal = cx.update(|cx| {
                                in_pane_of_entity_id.and_then(|entity_id| {
                                    terminal_provider.find_terminal_by_id(entity_id, cx)
                                })
                            });

                            // Get the task from the provider (called synchronously on main thread)
                            let create_task = cx.update(|cx| {
                                terminal_provider
                                    .create_terminal(spawn_task, target_terminal, activate, cx)
                                    .ok_or_else(|| {
                                        anyhow!("No workspace available for terminal creation")
                                    })
                            })?;

                            // Wait for the terminal to be created
                            let weak_terminal = create_task.await?;

                            // Get the entity_id
                            let entity_id = cx.update(|cx| {
                                if let Some(terminal) = weak_terminal.upgrade() {
                                    let entity_id = terminal.entity_id().as_non_zero_u64().get();

                                    // Set title override if provided
                                    if title_override.is_some() {
                                        terminal.update(cx, |terminal, cx| {
                                            terminal.set_title_override(title_override.clone(), cx);
                                        });
                                    }

                                    Ok(entity_id)
                                } else {
                                    Err(anyhow!("Terminal was dropped immediately after creation"))
                                }
                            })?;

                            Ok(entity_id)
                        }
                        .await;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn send_text(
        &self,
        terminal_handle: extension::TerminalHandle,
        text: String,
    ) -> Task<Result<()>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let weak_terminal = terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })?;

                                if let Some(terminal) = weak_terminal.upgrade() {
                                    terminal.update(cx, |terminal, _cx| {
                                        terminal.input(text.into_bytes());
                                    });
                                    Ok(())
                                } else {
                                    Err(anyhow!("Terminal no longer exists"))
                                }
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn send_key(
        &self,
        terminal_handle: extension::TerminalHandle,
        key: String,
    ) -> Task<Result<()>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let weak_terminal = terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })?;

                                if let Some(terminal) = weak_terminal.upgrade() {
                                    let key_normalized = match key.to_lowercase().as_str() {
                                        "esc" => "escape".to_string(),
                                        "return" => "enter".to_string(),
                                        _ => key,
                                    };

                                    let keystroke = match Keystroke::parse(&key_normalized) {
                                        Ok(ks) => ks,
                                        Err(_) => {
                                            terminal.update(cx, |terminal, _cx| {
                                                terminal.input(key_normalized.as_bytes().to_vec());
                                            });
                                            return Ok(());
                                        }
                                    };

                                    let term_mode = terminal.read(cx).last_content().mode;
                                    let esc_seq = to_esc_str(&keystroke, &term_mode, true);

                                    let bytes = if let Some(cow_str) = esc_seq {
                                        match cow_str {
                                            Cow::Borrowed(s) => s.as_bytes().to_vec(),
                                            Cow::Owned(s) => s.into_bytes(),
                                        }
                                    } else {
                                        keystroke.key.as_bytes().to_vec()
                                    };

                                    terminal.update(cx, |terminal, _cx| {
                                        terminal.input(bytes);
                                    });
                                    Ok(())
                                } else {
                                    Err(anyhow!("Terminal no longer exists"))
                                }
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn read_screen(
        &self,
        terminal_handle: extension::TerminalHandle,
    ) -> Task<Result<extension::TerminalContent>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let weak_terminal = terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })?;

                                if let Some(terminal) = weak_terminal.upgrade() {
                                    let term = terminal.read(cx);
                                    let content_str = term.get_content();
                                    let lines: Vec<String> =
                                        content_str.lines().map(String::from).collect();
                                    let last_content = term.last_content();

                                    Ok(extension::TerminalContent {
                                        lines,
                                        cursor_row: last_content.cursor.point.line.0 as u32,
                                        cursor_col: last_content.cursor.point.column.0 as u32,
                                    })
                                } else {
                                    Err(anyhow!("Terminal no longer exists"))
                                }
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn split_terminal(
        &self,
        terminal_handle: extension::TerminalHandle,
        direction: extension::SplitDirection,
        options: extension::TerminalOptions,
    ) -> Task<Result<extension::TerminalHandle>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();
        let title_override = options.title_override.clone();

        let spawn_task = SpawnInTerminal {
            command: options.command,
            args: options.args,
            cwd: options.cwd,
            env: options.env.into_iter().collect(),
            ..Default::default()
        };

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result: Result<extension::TerminalHandle> = async {
                            let weak_terminal = cx.update(|cx| {
                                terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })
                            })?;

                            let create_task = cx.update(|cx| {
                                terminal_provider
                                    .split_terminal(weak_terminal, direction, spawn_task, cx)
                                    .ok_or_else(|| {
                                        anyhow!("No workspace available for terminal split")
                                    })
                            })?;

                            let new_weak_terminal = create_task.await?;

                            // Get the entity_id
                            let entity_id = cx.update(|cx| {
                                if let Some(terminal) = new_weak_terminal.upgrade() {
                                    let entity_id = terminal.entity_id().as_non_zero_u64().get();

                                    // Set title override if provided
                                    if title_override.is_some() {
                                        terminal.update(cx, |terminal, cx| {
                                            terminal.set_title_override(title_override.clone(), cx);
                                        });
                                    }

                                    Ok(entity_id)
                                } else {
                                    Err(anyhow!("Terminal was dropped immediately after creation"))
                                }
                            })?;

                            Ok(entity_id)
                        }
                        .await;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn close_terminal(&self, terminal_handle: extension::TerminalHandle) -> Task<Result<()>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let weak_terminal = terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })?;

                                terminal_provider.close_terminal(weak_terminal, cx)
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn list_terminals(&self) -> Task<Result<Vec<extension::WorkspaceTerminals>>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result: Result<Vec<extension::WorkspaceTerminals>> = cx.update(|cx| {
                            Ok(terminal_provider.list_all_workspaces_terminals(cx))
                        });

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn get_cwd(&self, terminal_handle: extension::TerminalHandle) -> Task<Result<Option<String>>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let weak_terminal = terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })?;

                                if let Some(terminal) = weak_terminal.upgrade() {
                                    let cwd = terminal.read(cx).working_directory();
                                    Ok(cwd.map(|p| p.to_string_lossy().to_string()))
                                } else {
                                    Err(anyhow!("Terminal no longer exists"))
                                }
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn is_idle(&self, terminal_handle: extension::TerminalHandle) -> Task<Result<bool>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let weak_terminal = terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })?;

                                if let Some(terminal) = weak_terminal.upgrade() {
                                    let is_idle = terminal
                                        .read(cx)
                                        .task()
                                        .map(|task_state| {
                                            matches!(
                                                task_state.status,
                                                terminal::TaskStatus::Completed { .. }
                                            )
                                        })
                                        .unwrap_or(true);
                                    Ok(is_idle)
                                } else {
                                    Err(anyhow!("Terminal no longer exists"))
                                }
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn resolve_terminal(&self, identifier: String) -> Task<Result<extension::TerminalHandle>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                // First, try to parse as a numeric entity_id
                                if let Ok(entity_id) = identifier.parse::<u64>() {
                                    if terminal_provider
                                        .find_terminal_by_id(entity_id, cx)
                                        .is_some()
                                    {
                                        return Ok(entity_id);
                                    }
                                }

                                // Otherwise, search all workspaces for matching title_override
                                for workspace in terminal_provider.list_all_workspaces_terminals(cx)
                                {
                                    for terminal_info in workspace.terminals {
                                        if terminal_info.title_override.as_deref()
                                            == Some(identifier.as_str())
                                        {
                                            return Ok(terminal_info.entity_id);
                                        }
                                    }
                                }

                                Err(anyhow!(
                                    "Terminal '{}' not found (not a valid entity_id or title)",
                                    identifier
                                ))
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn get_layout(&self) -> Task<Result<extension::PaneLayout>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result: Result<extension::PaneLayout> = cx.update(|cx| {
                            let handle_for_terminal =
                                |weak: &WeakEntity<Terminal>| -> Option<u64> {
                                    weak.upgrade().map(|terminal| {
                                        terminal.entity_id().as_non_zero_u64().get()
                                    })
                                };

                            Ok(terminal_provider.get_layout(&handle_for_terminal, cx))
                        });

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn focus_terminal(&self, terminal_handle: extension::TerminalHandle) -> Task<Result<()>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let weak_terminal = terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })?;

                                terminal_provider.focus_terminal(weak_terminal, cx)
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn set_layout(
        &self,
        mode: extension::LayoutMode,
        caller_terminal_id: Option<u64>,
    ) -> Task<Result<()>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| terminal_provider.set_layout(mode, caller_terminal_id, cx))
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn set_title(
        &self,
        terminal_handle: extension::TerminalHandle,
        title: Option<String>,
    ) -> Task<Result<()>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let weak_terminal = terminal_provider
                                    .find_terminal_by_id(terminal_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Terminal {} not found", terminal_handle)
                                    })?;

                                if let Some(terminal) = weak_terminal.upgrade() {
                                    terminal.update(cx, |terminal, cx| {
                                        terminal.set_title_override(title, cx);
                                    });
                                    Ok(())
                                } else {
                                    Err(anyhow!("Terminal no longer exists"))
                                }
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }

    fn move_terminal(
        &self,
        source_handle: extension::TerminalHandle,
        destination_handle: extension::TerminalHandle,
    ) -> Task<Result<()>> {
        let terminal_provider = self.terminal_provider.clone();
        let main_thread_tx = self.main_thread_tx.clone();

        self.executor.spawn(async move {
            let (result_tx, result_rx) = futures::channel::oneshot::channel();

            use futures::SinkExt;
            main_thread_tx
                .clone()
                .send(Box::new(move |cx| {
                    Box::pin(async move {
                        let result = cx
                            .update(|cx| {
                                let source = terminal_provider
                                    .find_terminal_by_id(source_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!("Source terminal {} not found", source_handle)
                                    })?;
                                let destination = terminal_provider
                                    .find_terminal_by_id(destination_handle, cx)
                                    .ok_or_else(|| {
                                        anyhow!(
                                            "Destination terminal {} not found",
                                            destination_handle
                                        )
                                    })?;

                                terminal_provider.move_terminal(source, destination, cx)
                            })
                            ;

                        result_tx.send(result).ok();
                    })
                }))
                .await
                .map_err(|_| anyhow!("Main thread channel closed"))?;

            result_rx.await.map_err(|_| anyhow!("Result channel closed"))?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use extension::ExtensionHostProxy;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use serde_json::json;
    use std::sync::Arc;

    #[cfg(test)]
    #[ctor::ctor]
    fn init_logger() {
        zlog::init_test();
    }

    /// Test ProjectProvider that returns a specific project
    struct TestProjectProvider {
        project: parking_lot::RwLock<Option<Entity<Project>>>,
    }

    impl TestProjectProvider {
        fn new() -> Self {
            Self {
                project: parking_lot::RwLock::new(None),
            }
        }

        fn set_project(&self, project: Entity<Project>) {
            *self.project.write() = Some(project);
        }
    }

    impl TerminalProvider for TestProjectProvider {
        fn active_project(&self, _cx: &App) -> Option<Entity<Project>> {
            self.project.read().clone()
        }

        fn create_terminal(
            &self,
            _spawn_task: SpawnInTerminal,
            _target_terminal: Option<WeakEntity<Terminal>>,
            _activate: bool,
            _cx: &mut App,
        ) -> Option<Task<Result<WeakEntity<Terminal>>>> {
            None
        }

        fn close_terminal(&self, _terminal: WeakEntity<Terminal>, _cx: &mut App) -> Result<()> {
            Ok(())
        }

        fn find_terminal_by_id(&self, _entity_id: u64, _cx: &App) -> Option<WeakEntity<Terminal>> {
            None
        }

        fn list_all_workspaces_terminals(&self, _cx: &App) -> Vec<extension::WorkspaceTerminals> {
            Vec::new()
        }

        fn split_terminal(
            &self,
            _terminal: WeakEntity<Terminal>,
            _direction: extension::SplitDirection,
            _spawn_task: SpawnInTerminal,
            _cx: &mut App,
        ) -> Option<Task<Result<WeakEntity<Terminal>>>> {
            None
        }

        fn get_layout(
            &self,
            _handle_for_terminal: &dyn Fn(&WeakEntity<Terminal>) -> Option<u64>,
            _cx: &App,
        ) -> extension::PaneLayout {
            extension::PaneLayout {
                panel_bounds: None,
                root: extension::PaneLayoutMember::Pane {
                    pane_id: None,
                    terminals: Vec::new(),
                    bounds: None,
                },
            }
        }

        fn focus_terminal(&self, _terminal: WeakEntity<Terminal>, _cx: &mut App) -> Result<()> {
            Ok(())
        }

        fn set_layout(
            &self,
            _mode: extension::LayoutMode,
            _caller_terminal_id: Option<u64>,
            _cx: &mut App,
        ) -> Result<()> {
            Ok(())
        }

        fn is_terminal_active(&self, _terminal: &WeakEntity<Terminal>, _cx: &App) -> bool {
            false
        }

        fn move_terminal(
            &self,
            _source: WeakEntity<Terminal>,
            _destination: WeakEntity<Terminal>,
            _cx: &mut App,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[gpui::test]
    async fn test_terminal_proxy_initialization(cx: &mut TestAppContext) {
        // Initialize settings
        cx.update(|cx| {
            settings::init(cx);
        });

        // Create fake filesystem
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/test-project",
            json!({
                "file.txt": "hello world"
            }),
        )
        .await;

        // Create test project
        let project = Project::test(fs, ["/test-project".as_ref()], cx).await;

        // Create project provider
        let project_provider = Arc::new(TestProjectProvider::new());
        project_provider.set_project(project);

        // Create extension host proxy
        let extension_host_proxy = Arc::new(ExtensionHostProxy::default());

        // Initialize terminal extension
        cx.update(|cx| {
            init(extension_host_proxy.clone(), project_provider, cx);
        });

        // Verify proxy was registered by checking that list_terminals works
        // (it should return empty list, not an error about missing proxy)
        let list_result = extension_host_proxy.list_terminals().await;

        assert!(
            list_result.is_ok(),
            "list_terminals should succeed after proxy registration"
        );
        assert!(
            list_result.unwrap().is_empty(),
            "Should have no terminals initially"
        );
    }

    #[gpui::test]
    async fn test_terminal_proxy_close_nonexistent(cx: &mut TestAppContext) {
        cx.update(|cx| {
            settings::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/test-project", json!({})).await;

        let project = Project::test(fs, ["/test-project".as_ref()], cx).await;

        let project_provider = Arc::new(TestProjectProvider::new());
        project_provider.set_project(project);

        let extension_host_proxy = Arc::new(ExtensionHostProxy::default());

        cx.update(|cx| {
            init(extension_host_proxy.clone(), project_provider, cx);
        });

        // Closing a non-existent terminal should fail (terminal not found)
        let close_result = extension_host_proxy.close_terminal(9999).await;

        assert!(
            close_result.is_err(),
            "close_terminal on non-existent handle should fail"
        );
    }

    #[gpui::test]
    async fn test_terminal_proxy_operations_on_invalid_handle(cx: &mut TestAppContext) {
        cx.update(|cx| {
            settings::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/test-project", json!({})).await;

        let project = Project::test(fs, ["/test-project".as_ref()], cx).await;

        let project_provider = Arc::new(TestProjectProvider::new());
        project_provider.set_project(project);

        let extension_host_proxy = Arc::new(ExtensionHostProxy::default());

        cx.update(|cx| {
            init(extension_host_proxy.clone(), project_provider, cx);
        });

        // Operations on invalid handle should return errors
        let invalid_handle = 12345u64;

        let send_result = extension_host_proxy
            .send_text(invalid_handle, "test".to_string())
            .await;
        assert!(
            send_result.is_err(),
            "send_text on invalid handle should fail"
        );

        let read_result = extension_host_proxy.read_screen(invalid_handle).await;
        assert!(
            read_result.is_err(),
            "read_screen on invalid handle should fail"
        );

        let cwd_result = extension_host_proxy.get_cwd(invalid_handle).await;
        assert!(cwd_result.is_err(), "get_cwd on invalid handle should fail");

        let idle_result = extension_host_proxy.is_idle(invalid_handle).await;
        assert!(
            idle_result.is_err(),
            "is_idle on invalid handle should fail"
        );
    }
}

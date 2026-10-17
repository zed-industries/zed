use std::{
    cmp,
    path::PathBuf,
    process::ExitStatus,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};

use crate::{
    TerminalView, default_working_directory,
    persistence::{
        PersistenceTask, SerializedItems, SerializedTerminalPanel, deserialize_terminal_panel,
        serialize_pane_group,
    },
};
use breadcrumbs::Breadcrumbs;
use collections::HashMap;
use db::kvp::KeyValueStore;
use futures::{
    FutureExt as _,
    channel::oneshot,
    future::{BoxFuture, join_all, try_join_all},
};
use gpui::{
    Action, Anchor, App, AsyncApp, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, ParentElement, Pixels, Render, Styled, Task, TaskExt, WeakEntity,
    Window, actions,
};
use itertools::Itertools;
use project::{Fs, Project};

use settings::{Settings, TerminalDockPosition};
use task::{RevealStrategy, RevealTarget, Shell, ShellBuilder, SpawnInTerminal, TaskId};
use terminal::{Terminal, terminal_settings::TerminalSettings};
use ui::{
    ButtonLike, Clickable, CommonAnimationExt, ContextMenu, FluentBuilder, PopoverMenu,
    SplitButton, Toggleable, Tooltip, prelude::*,
};
use util::{ResultExt, defer};
use workspace::{
    ActivateNextPane, ActivatePane, ActivatePaneDown, ActivatePaneLeft, ActivatePaneRight,
    ActivatePaneUp, ActivatePreviousPane, DraggedTab, MoveItemToPane, MoveItemToPaneInDirection,
    MovePaneDown, MovePaneLeft, MovePaneRight, MovePaneUp, Pane, PaneGroup, SplitDirection,
    SplitDown, SplitLeft, SplitMode, SplitRight, SplitUp, SwapPaneDown, SwapPaneLeft,
    SwapPaneRight, SwapPaneUp, ToggleZoom, Workspace,
    dock::{DockPosition, Panel, PanelEvent, PanelHandle},
    invalid_item_view::RestoringItem,
    item::{SerializableItem, item_persistence_id},
    move_active_item,
    notifications::{NotificationId, simple_message_notification::MessageNotification},
    pane,
};

use anyhow::{Result, anyhow};
use zed_actions::assistant::InlineAssist;

const TERMINAL_PANEL_KEY: &str = "TerminalPanel";

actions!(
    terminal_panel,
    [
        /// Toggles the terminal panel.
        Toggle,
        /// Toggles focus on the terminal panel.
        ToggleFocus
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _: &mut Context<Workspace>| {
            workspace.register_action(TerminalPanel::new_terminal);
            workspace.register_action(TerminalPanel::open_terminal);
            workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
                if is_enabled_in_workspace(workspace, cx) {
                    workspace.toggle_panel_focus::<TerminalPanel>(window, cx);
                }
            });
            workspace.register_action(|workspace, _: &Toggle, window, cx| {
                if is_enabled_in_workspace(workspace, cx) {
                    if !workspace.toggle_panel_focus::<TerminalPanel>(window, cx) {
                        workspace.close_panel::<TerminalPanel>(window, cx);
                    }
                }
            });
        },
    )
    .detach();
}

pub struct TerminalPanel {
    pub(crate) active_pane: Entity<Pane>,
    pub(crate) center: PaneGroup,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    workspace: WeakEntity<Workspace>,
    persistence_enabled: Arc<AtomicBool>,
    pending_serialization: Task<Option<()>>,
    serialization: PersistenceTask,
    serialization_revision: Arc<AtomicUsize>,
    serialization_key: Option<String>,
    pub(crate) has_serialized_state: bool,
    pending_terminals_to_add: usize,
    restoring: bool,
    preserve_saved_layout: bool,
    _restoration: Task<()>,
    deferred_tasks: HashMap<TaskId, Task<()>>,
    assistant_enabled: bool,
    active: bool,
}

impl TerminalPanel {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let entity_id = cx.entity_id();
        let panel = cx.weak_entity();
        cx.default_global::<LiveTerminalPanels>()
            .0
            .insert(entity_id, panel);
        cx.on_release(move |_, cx| {
            cx.default_global::<LiveTerminalPanels>()
                .0
                .remove(&entity_id);
        })
        .detach();
        let project = workspace.project();
        let pane = new_terminal_pane(workspace.weak_handle(), project.clone(), false, window, cx);
        let center = PaneGroup::new(pane.clone());

        let terminal_panel = Self {
            center,
            active_pane: pane,
            focus_handle: cx.focus_handle(),
            fs: workspace.app_state().fs.clone(),
            workspace: workspace.weak_handle(),
            persistence_enabled: workspace.persistence_enabled(),
            pending_serialization: Task::ready(None),
            serialization: Task::ready(Ok(())).shared(),
            serialization_revision: Arc::new(AtomicUsize::new(0)),
            serialization_key: Self::serialization_key(workspace),
            has_serialized_state: false,
            pending_terminals_to_add: 0,
            restoring: false,
            preserve_saved_layout: false,
            _restoration: Task::ready(()),
            deferred_tasks: HashMap::default(),
            assistant_enabled: false,
            active: false,
        };
        terminal_panel.apply_tab_bar_buttons(&terminal_panel.active_pane, cx);
        terminal_panel
    }

    pub fn set_assistant_enabled(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.assistant_enabled = enabled;
        for pane in self.center.panes() {
            self.apply_tab_bar_buttons(pane, cx);
        }
    }

    pub(crate) fn apply_tab_bar_buttons(
        &self,
        terminal_pane: &Entity<Pane>,
        cx: &mut Context<Self>,
    ) {
        let assistant_enabled = self.assistant_enabled;
        terminal_pane.update(cx, |pane, cx| {
            pane.set_render_tab_bar_buttons(cx, move |pane, window, cx| {
                let split_context = pane
                    .active_item()
                    .and_then(|item| item.downcast::<TerminalView>())
                    .map(|terminal_view| terminal_view.read(cx).focus_handle.clone());
                let has_focused_rename_editor = pane
                    .active_item()
                    .and_then(|item| item.downcast::<TerminalView>())
                    .is_some_and(|view| view.read(cx).rename_editor_is_focused(window, cx));
                if !pane.has_focus(window, cx)
                    && !pane.context_menu_focused(window, cx)
                    && !has_focused_rename_editor
                {
                    return (None, None);
                }
                let focus_handle = pane.focus_handle(cx);
                let right_children = h_flex()
                    .gap(DynamicSpacing::Base02.rems(cx))
                    .child(
                        PopoverMenu::new("terminal-tab-bar-popover-menu")
                            .trigger_with_tooltip(
                                IconButton::new("plus", IconName::Plus).icon_size(IconSize::Small),
                                Tooltip::text("New…"),
                            )
                            .anchor(Anchor::TopRight)
                            .with_handle(pane.new_item_context_menu_handle.clone())
                            .menu(move |window, cx| {
                                let focus_handle = focus_handle.clone();
                                let menu = ContextMenu::build(window, cx, |menu, _, _| {
                                    menu.context(focus_handle.clone())
                                        .action(
                                            "New Terminal",
                                            workspace::NewTerminal::default().boxed_clone(),
                                        )
                                        // We want the focus to go back to terminal panel once task modal is dismissed,
                                        // hence we focus that first. Otherwise, we'd end up without a focused element, as
                                        // context menu will be gone the moment we spawn the modal.
                                        .action(
                                            "Spawn Task",
                                            zed_actions::Spawn::modal().boxed_clone(),
                                        )
                                });

                                Some(menu)
                            }),
                    )
                    .when(assistant_enabled, |this| {
                        this.when_some(split_context.clone(), |this, focus_handle| {
                            this.child(InlineAssistTabBarButton { focus_handle })
                        })
                    })
                    .child(
                        PopoverMenu::new("terminal-pane-tab-bar-split")
                            .trigger_with_tooltip(
                                IconButton::new("terminal-pane-split", IconName::Split)
                                    .icon_size(IconSize::Small),
                                Tooltip::text("Split Pane"),
                            )
                            .anchor(Anchor::TopRight)
                            .with_handle(pane.split_item_context_menu_handle.clone())
                            .menu({
                                move |window, cx| {
                                    ContextMenu::build(window, cx, |menu, _, _| {
                                        menu.when_some(
                                            split_context.clone(),
                                            |menu, split_context| menu.context(split_context),
                                        )
                                        .action("Split Right", SplitRight::default().boxed_clone())
                                        .action("Split Left", SplitLeft::default().boxed_clone())
                                        .action("Split Up", SplitUp::default().boxed_clone())
                                        .action("Split Down", SplitDown::default().boxed_clone())
                                    })
                                    .into()
                                }
                            }),
                    )
                    .child({
                        let zoomed = pane.is_zoomed();
                        IconButton::new("toggle_zoom", IconName::Maximize)
                            .icon_size(IconSize::Small)
                            .toggle_state(zoomed)
                            .selected_icon(IconName::Minimize)
                            .on_click(cx.listener(|pane, _, window, cx| {
                                pane.toggle_zoom(&workspace::ToggleZoom, window, cx);
                            }))
                            .tooltip(move |_window, cx| {
                                Tooltip::for_action(
                                    if zoomed { "Zoom Out" } else { "Zoom In" },
                                    &ToggleZoom,
                                    cx,
                                )
                            })
                    })
                    .into_any_element()
                    .into();
                (None, right_children)
            });
        });
    }

    fn serialization_key(workspace: &Workspace) -> Option<String> {
        workspace
            .database_id()
            .map(|id| i64::from(id).to_string())
            .or(workspace.session_id())
            .map(|id| format!("{:?}-{:?}", TERMINAL_PANEL_KEY, id))
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        let terminal_panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| TerminalPanel::new(workspace, window, cx));
            workspace.set_terminal_provider(TerminalProvider(panel.clone()));
            panel
        })?;

        let serialized_panel = terminal_panel.update(&mut cx, |panel, cx| {
            panel
                .serialization_key
                .as_ref()
                .map(|key| KeyValueStore::global(cx).read_kvp(key))
                .transpose()?
                .flatten()
                .map(|state| serde_json::from_str::<SerializedTerminalPanel>(&state))
                .transpose()
                .map_err(anyhow::Error::from)
        });
        let serialized_panel = match serialized_panel {
            Ok(serialized_panel) => serialized_panel,
            Err(error) => {
                terminal_panel.update(&mut cx, |panel, cx| panel.fail_restoration(error, cx));
                return Ok(terminal_panel);
            }
        };
        let (project, database_id) = workspace.read_with(&cx, |workspace, _| {
            (workspace.project().clone(), workspace.database_id())
        })?;
        let restore = cx.update(|window, cx| match serialized_panel.zip(database_id) {
            Some((serialized_panel, database_id)) => deserialize_terminal_panel(
                workspace.clone(),
                project,
                database_id,
                serialized_panel,
                terminal_panel.downgrade(),
                window,
                cx,
            ),
            None => Task::ready(Ok(0)),
        })?;

        terminal_panel.update_in(&mut cx, |panel, window, cx| {
            panel.restoring = true;
            panel._restoration = cx.spawn_in(window, {
                let workspace = workspace.clone();
                let persistence_enabled = panel.persistence_enabled.clone();
                async move |terminal_panel, cx| {
                    let restored = match Self::restore_serialized_state(
                        workspace.clone(),
                        terminal_panel.clone(),
                        restore,
                        cx,
                    )
                    .await
                    {
                        Ok(restored) => restored,
                        Err(error) => {
                            terminal_panel
                                .update(cx, |panel, cx| panel.fail_restoration(error, cx))
                                .ok();
                            return;
                        }
                    };
                    let default_shell_task = terminal_panel
                        .update_in(cx, |terminal_panel, window, cx| {
                            terminal_panel.finish_restoration(restored, window, cx)
                        })
                        .ok()
                        .flatten();
                    if let Ok(flush) =
                        terminal_panel.update(cx, |panel, cx| panel.flush_serialization(cx))
                    {
                        flush.await.log_err();
                    }
                    if persistence_enabled.load(Ordering::Relaxed)
                        && let Ok(Some(cleanup)) =
                            workspace.update_in(cx, |workspace, window, cx| {
                                workspace.database_id().map(|workspace_id| {
                                    TerminalView::cleanup(workspace_id, Vec::new(), window, cx)
                                })
                            })
                    {
                        cleanup.await.log_err();
                    }
                    if let Some(task) = default_shell_task {
                        task.await.log_err();
                    }
                }
            });
        })?;

        Ok(terminal_panel)
    }

    fn fail_restoration(&mut self, error: anyhow::Error, cx: &mut Context<Self>) {
        let error = error.context("Failed to restore terminal panel");
        self.restoring = false;
        self.preserve_saved_layout = true;
        let panel = cx.weak_entity();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.show_notification(NotificationId::unique::<TerminalPanel>(), cx, |cx| {
                    cx.new(|cx| {
                        MessageNotification::new(format!("{error:#}"), cx)
                            .secondary_content(
                                "The saved layout has been kept.\nNew terminals will not be saved until you replace it.",
                            )
                            .show_suppress_button(false)
                            .primary_message("Replace Saved Layout")
                            .primary_on_click(move |_, cx| {
                                panel
                                    .update(cx, |panel, cx| panel.replace_failed_layout(cx))
                                    .ok();
                            })
                    })
                });
            })
            .ok();
        cx.notify();
    }

    fn replace_failed_layout(&mut self, cx: &mut Context<Self>) {
        if self.preserve_saved_layout {
            self.preserve_saved_layout = false;
            self.has_serialized_state = true;
            self.serialize(cx);
        }
    }

    fn finish_restoration(
        &mut self,
        restored: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<WeakEntity<Terminal>>>> {
        self.restoring = false;

        let has_terminals = self
            .center
            .panes()
            .into_iter()
            .any(|pane| pane.read(cx).items_len() > 0);
        if restored || has_terminals {
            self.schedule_serialization(cx);
        }
        cx.notify();
        if self.active && self.has_no_terminals(cx) {
            let working_directory = self
                .workspace
                .update(cx, |workspace, cx| default_working_directory(workspace, cx))
                .ok()
                .flatten();
            Some(self.add_terminal_shell(
                false,
                working_directory,
                RevealStrategy::Always,
                window,
                cx,
            ))
        } else {
            None
        }
    }

    async fn restore_serialized_state(
        workspace: WeakEntity<Workspace>,
        terminal_panel: WeakEntity<Self>,
        restore: Task<Result<usize>>,
        cx: &mut AsyncWindowContext,
    ) -> Result<bool> {
        let restored = restore.await? > 0;

        let should_focus = workspace
            .update_in(cx, |workspace, window, cx| {
                !workspace.has_active_modal(window, cx)
                    && terminal_panel.upgrade().is_some_and(|terminal_panel| {
                        workspace.active_item(cx).is_none()
                            && workspace
                                .is_dock_at_position_open(terminal_panel.position(window, cx), cx)
                    })
            })
            .unwrap_or(false);
        if should_focus {
            terminal_panel
                .update_in(cx, |panel, window, cx| {
                    panel.active_pane.update(cx, |pane, cx| {
                        pane.focus_active_item(window, cx);
                    });
                })
                .ok();
        }
        Ok(restored)
    }

    fn handle_pane_event(
        &mut self,
        pane: &Entity<Pane>,
        event: &pane::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            pane::Event::ActivateItem { .. } => self.serialize(cx),
            pane::Event::RemovedItem { .. } => self.serialize(cx),
            pane::Event::Remove { focus_on_pane } => {
                let pane_count_before_removal = self.center.panes().len();
                let _removal_result = self.center.remove(pane, cx);
                if pane_count_before_removal == 1 {
                    self.center.first_pane().update(cx, |pane, cx| {
                        pane.set_zoomed(false, cx);
                    });
                    cx.emit(PanelEvent::Close);
                } else if let Some(focus_on_pane) =
                    focus_on_pane.as_ref().or_else(|| self.center.panes().pop())
                {
                    focus_on_pane.focus_handle(cx).focus(window, cx);
                }
                self.serialize(cx);
            }
            pane::Event::ZoomIn => {
                for pane in self.center.panes() {
                    pane.update(cx, |pane, cx| {
                        pane.set_zoomed(true, cx);
                    })
                }
                cx.emit(PanelEvent::ZoomIn);
                cx.notify();
            }
            pane::Event::ZoomOut => {
                for pane in self.center.panes() {
                    pane.update(cx, |pane, cx| {
                        pane.set_zoomed(false, cx);
                    })
                }
                cx.emit(PanelEvent::ZoomOut);
                cx.notify();
            }
            pane::Event::AddItem { item, restored } => {
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, pane.clone(), *restored, window, cx)
                    })
                }
                if *restored {
                    self.schedule_serialization(cx);
                } else {
                    self.serialize(cx);
                }
            }
            pane::Event::ReplaceItem { item, .. } => {
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, pane.clone(), true, window, cx)
                    });
                }
                self.schedule_serialization(cx);
            }
            &pane::Event::Split { direction, mode } => {
                match mode {
                    SplitMode::ClonePane | SplitMode::EmptyPane => {
                        let clone = matches!(mode, SplitMode::ClonePane);
                        let new_pane = self.new_pane_with_active_terminal(clone, window, cx);
                        let pane = pane.clone();
                        cx.spawn_in(window, async move |panel, cx| {
                            let Some(new_pane) = new_pane.await else {
                                return;
                            };
                            panel
                                .update_in(cx, |panel, window, cx| {
                                    panel.center.split(&pane, &new_pane, direction, cx);
                                    panel.serialize(cx);
                                    window.focus(&new_pane.focus_handle(cx), cx);
                                })
                                .ok();
                        })
                        .detach();
                    }
                    SplitMode::MovePane => {
                        let Some(item) =
                            pane.update(cx, |pane, cx| pane.take_active_item(window, cx))
                        else {
                            return;
                        };
                        let Ok(project) = self
                            .workspace
                            .update(cx, |workspace, _| workspace.project().clone())
                        else {
                            return;
                        };
                        let new_pane =
                            new_terminal_pane(self.workspace.clone(), project, false, window, cx);
                        new_pane.update(cx, |pane, cx| {
                            pane.add_item(item, true, true, None, window, cx);
                        });
                        self.center.split(&pane, &new_pane, direction, cx);
                        self.serialize(cx);
                        window.focus(&new_pane.focus_handle(cx), cx);
                    }
                };
            }
            pane::Event::Focus => {
                if self.active_pane != *pane {
                    self.active_pane = pane.clone();
                    self.serialize(cx);
                }
            }
            pane::Event::ItemPinned | pane::Event::ItemUnpinned => {
                self.serialize(cx);
            }

            _ => {}
        }
    }

    fn new_pane_with_active_terminal(
        &mut self,
        clone: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Pane>>> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(None);
        };
        let workspace = workspace.read(cx);
        let database_id = workspace.database_id();
        let weak_workspace = self.workspace.clone();
        let project = workspace.project().clone();
        let active_pane = &self.active_pane;
        let terminal_view = if clone {
            active_pane
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<TerminalView>())
        } else {
            None
        };
        let working_directory = if clone {
            terminal_view
                .as_ref()
                .and_then(|terminal_view| {
                    terminal_view
                        .read(cx)
                        .terminal()
                        .read(cx)
                        .working_directory()
                })
                .or_else(|| default_working_directory(workspace, cx))
        } else {
            default_working_directory(workspace, cx)
        };

        let is_zoomed = if clone {
            active_pane.read(cx).is_zoomed()
        } else {
            false
        };
        cx.spawn_in(window, async move |panel, cx| {
            let terminal = project
                .update(cx, |project, cx| match terminal_view {
                    Some(view) => project.clone_terminal(
                        &view.read(cx).terminal.clone(),
                        cx,
                        working_directory,
                    ),
                    None => project.create_terminal_shell(working_directory, cx),
                })
                .await
                .log_err()?;

            panel
                .update_in(cx, move |terminal_panel, window, cx| {
                    let terminal_view = Box::new(cx.new(|cx| {
                        TerminalView::new(
                            terminal.clone(),
                            weak_workspace.clone(),
                            database_id,
                            project.downgrade(),
                            window,
                            cx,
                        )
                    }));
                    let pane = new_terminal_pane(weak_workspace, project, is_zoomed, window, cx);
                    terminal_panel.apply_tab_bar_buttons(&pane, cx);
                    pane.update(cx, |pane, cx| {
                        pane.add_item(terminal_view, true, true, None, window, cx);
                    });
                    Some(pane)
                })
                .ok()
                .flatten()
        })
    }

    pub fn open_terminal(
        workspace: &mut Workspace,
        action: &workspace::OpenTerminal,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(terminal_panel) = workspace.panel::<Self>(cx) else {
            return;
        };

        terminal_panel
            .update(cx, |panel, cx| {
                panel.add_terminal_shell(
                    action.local,
                    Some(action.working_directory.clone()),
                    RevealStrategy::Always,
                    window,
                    cx,
                )
            })
            .detach_and_log_err(cx);
    }

    pub fn spawn_task(
        &mut self,
        task: &SpawnInTerminal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<WeakEntity<Terminal>>> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(Err(anyhow!("failed to read workspace")));
        };

        let project = workspace.read(cx).project().read(cx);

        if project.is_via_collab() {
            return Task::ready(Err(anyhow!("cannot spawn tasks as a guest")));
        }

        let remote_client = project.remote_client();
        let is_windows = project.path_style(cx).is_windows();
        let remote_shell = remote_client
            .as_ref()
            .and_then(|remote_client| remote_client.read(cx).shell());

        let shell = if let Some(remote_shell) = remote_shell
            && task.shell == Shell::System
        {
            Shell::Program(remote_shell)
        } else {
            task.shell.clone()
        };

        let task = prepare_task_for_spawn(task, &shell, is_windows);

        if task.allow_concurrent_runs && task.use_new_terminal {
            return self.spawn_in_new_terminal(task, window, cx);
        }

        let mut terminals_for_task = self.terminals_for_task(&task.full_label, cx);
        let Some(existing) = terminals_for_task.pop() else {
            return self.spawn_in_new_terminal(task, window, cx);
        };

        let (existing_item_index, task_pane, existing_terminal) = existing;
        if task.allow_concurrent_runs {
            return self.replace_terminal(
                task,
                task_pane,
                existing_item_index,
                existing_terminal,
                window,
                cx,
            );
        }

        let (tx, rx) = oneshot::channel();

        self.deferred_tasks.insert(
            task.id.clone(),
            cx.spawn_in(window, async move |terminal_panel, cx| {
                wait_for_terminals_tasks(terminals_for_task, cx).await;
                let task = terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                    if task.use_new_terminal {
                        terminal_panel.spawn_in_new_terminal(task, window, cx)
                    } else {
                        terminal_panel.replace_terminal(
                            task,
                            task_pane,
                            existing_item_index,
                            existing_terminal,
                            window,
                            cx,
                        )
                    }
                });
                if let Ok(task) = task {
                    tx.send(task.await).ok();
                }
            }),
        );

        cx.spawn(async move |_, _| rx.await?)
    }

    fn spawn_in_new_terminal(
        &mut self,
        spawn_task: SpawnInTerminal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<WeakEntity<Terminal>>> {
        let reveal = spawn_task.reveal;
        let reveal_target = spawn_task.reveal_target;
        match reveal_target {
            RevealTarget::Center => self
                .workspace
                .update(cx, |workspace, cx| {
                    Self::add_center_terminal(workspace, window, cx, |project, cx| {
                        project.create_terminal_task(spawn_task, cx)
                    })
                })
                .unwrap_or_else(|e| Task::ready(Err(e))),
            RevealTarget::Dock => self.add_terminal_task(spawn_task, reveal, window, cx),
        }
    }

    /// Create a new Terminal in the current working directory or the user's home directory
    fn new_terminal(
        workspace: &mut Workspace,
        action: &workspace::NewTerminal,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let center_pane = workspace.active_pane();
        let center_pane_has_focus = center_pane.focus_handle(cx).contains_focused(window, cx);
        let active_center_item_is_terminal = center_pane
            .read(cx)
            .active_item()
            .is_some_and(|item| item.downcast::<TerminalView>().is_some());

        if center_pane_has_focus && active_center_item_is_terminal {
            let working_directory = default_working_directory(workspace, cx);
            let local = action.local;
            Self::add_center_terminal(workspace, window, cx, move |project, cx| {
                if local {
                    project.create_local_terminal(cx)
                } else {
                    project.create_terminal_shell(working_directory, cx)
                }
            })
            .detach_and_log_err(cx);
            return;
        }

        let Some(terminal_panel) = workspace.panel::<Self>(cx) else {
            return;
        };

        terminal_panel
            .update(cx, |this, cx| {
                this.add_terminal_shell(
                    action.local,
                    default_working_directory(workspace, cx),
                    RevealStrategy::Always,
                    window,
                    cx,
                )
            })
            .detach_and_log_err(cx);
    }

    fn terminals_for_task(
        &self,
        label: &str,
        cx: &mut App,
    ) -> Vec<(usize, Entity<Pane>, Entity<TerminalView>)> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Vec::new();
        };

        let pane_terminal_views = |pane: Entity<Pane>| {
            pane.read(cx)
                .items()
                .enumerate()
                .filter_map(|(index, item)| Some((index, item.act_as::<TerminalView>(cx)?)))
                .filter_map(|(index, terminal_view)| {
                    let task_state = terminal_view.read(cx).terminal().read(cx).task()?;
                    if &task_state.spawned_task.full_label == label {
                        Some((index, terminal_view))
                    } else {
                        None
                    }
                })
                .map(move |(index, terminal_view)| (index, pane.clone(), terminal_view))
        };

        self.center
            .panes()
            .into_iter()
            .cloned()
            .flat_map(pane_terminal_views)
            .chain(
                workspace
                    .read(cx)
                    .panes()
                    .iter()
                    .cloned()
                    .flat_map(pane_terminal_views),
            )
            .sorted_by_key(|(_, _, terminal_view)| terminal_view.entity_id())
            .collect()
    }

    fn activate_terminal_view(
        &self,
        pane: &Entity<Pane>,
        item_index: usize,
        focus: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        pane.update(cx, |pane, cx| {
            pane.activate_item(item_index, true, focus, window, cx)
        })
    }

    pub fn add_center_terminal(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
        create_terminal: impl FnOnce(
            &mut Project,
            &mut Context<Project>,
        ) -> Task<Result<Entity<Terminal>>>
        + 'static,
    ) -> Task<Result<WeakEntity<Terminal>>> {
        if !is_enabled_in_workspace(workspace, cx) {
            return Task::ready(Err(anyhow!(
                "terminal not yet supported for remote projects"
            )));
        }
        let project = workspace.project().downgrade();
        cx.spawn_in(window, async move |workspace, cx| {
            let terminal = project.update(cx, create_terminal)?.await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let terminal_view = cx.new(|cx| {
                    TerminalView::new(
                        terminal.clone(),
                        workspace.weak_handle(),
                        workspace.database_id(),
                        workspace.project().downgrade(),
                        window,
                        cx,
                    )
                });
                // Don't steal focus from an open modal (e.g. the command palette):
                // a background terminal can finish starting up after the user has
                // moved on, and focusing it would dismiss whatever they opened.
                let focus_item = !workspace.has_active_modal(window, cx);
                workspace.add_item_to_active_pane(
                    Box::new(terminal_view),
                    None,
                    focus_item,
                    window,
                    cx,
                );
            })?;
            Ok(terminal.downgrade())
        })
    }

    pub fn add_terminal_task(
        &mut self,
        task: SpawnInTerminal,
        reveal_strategy: RevealStrategy,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<WeakEntity<Terminal>>> {
        let workspace = self.workspace.clone();
        self.spawn_pending_terminal(window, cx, async move |terminal_panel, cx| {
            if workspace.update(cx, |workspace, cx| !is_enabled_in_workspace(workspace, cx))? {
                anyhow::bail!("terminal not yet supported for remote projects");
            }
            let project = workspace.read_with(cx, |workspace, _| workspace.project().clone())?;
            let terminal = project
                .update(cx, |project, cx| project.create_terminal_task(task, cx))
                .await?;
            let pane = terminal_panel
                .read_with(cx, |terminal_panel, _| terminal_panel.active_pane.clone())?;
            workspace.update_in(cx, |workspace, window, cx| {
                let terminal_view = Box::new(cx.new(|cx| {
                    TerminalView::new(
                        terminal.clone(),
                        workspace.weak_handle(),
                        workspace.database_id(),
                        workspace.project().downgrade(),
                        window,
                        cx,
                    )
                }));

                let take_focus = reveal_strategy == RevealStrategy::Always
                    && !workspace.has_active_modal(window, cx);
                match reveal_strategy {
                    RevealStrategy::Always if take_focus => {
                        workspace.focus_panel::<Self>(window, cx);
                    }
                    RevealStrategy::Always | RevealStrategy::NoFocus => {
                        workspace.open_panel::<Self>(window, cx);
                    }
                    RevealStrategy::Never => {}
                }

                pane.update(cx, |pane, cx| {
                    pane.add_item(terminal_view, true, take_focus, None, window, cx);
                });

                terminal.downgrade()
            })
        })
    }

    fn add_terminal_shell(
        &mut self,
        force_local: bool,
        cwd: Option<PathBuf>,
        reveal_strategy: RevealStrategy,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<WeakEntity<Terminal>>> {
        let workspace = self.workspace.clone();
        self.spawn_pending_terminal(window, cx, async move |terminal_panel, cx| {
            if workspace.update(cx, |workspace, cx| !is_enabled_in_workspace(workspace, cx))? {
                anyhow::bail!("terminal not yet supported for collaborative projects");
            }
            let project = workspace.read_with(cx, |workspace, _| workspace.project().clone())?;
            let terminal = if force_local {
                project
                    .update(cx, |project, cx| project.create_local_terminal(cx))
                    .await
            } else {
                project
                    .update(cx, |project, cx| project.create_terminal_shell(cwd, cx))
                    .await
            };

            let pane = terminal_panel
                .read_with(cx, |terminal_panel, _| terminal_panel.active_pane.clone())?;
            match terminal {
                Ok(terminal) => workspace.update_in(cx, |workspace, window, cx| {
                    let terminal_view = Box::new(cx.new(|cx| {
                        TerminalView::new(
                            terminal.clone(),
                            workspace.weak_handle(),
                            workspace.database_id(),
                            workspace.project().downgrade(),
                            window,
                            cx,
                        )
                    }));

                    let take_focus = reveal_strategy == RevealStrategy::Always
                        && !workspace.has_active_modal(window, cx);
                    match reveal_strategy {
                        RevealStrategy::Always if take_focus => {
                            workspace.focus_panel::<Self>(window, cx);
                        }
                        RevealStrategy::Always | RevealStrategy::NoFocus => {
                            workspace.open_panel::<Self>(window, cx);
                        }
                        RevealStrategy::Never => {}
                    }

                    pane.update(cx, |pane, cx| {
                        pane.add_item(terminal_view, true, take_focus, None, window, cx);
                    });

                    terminal.downgrade()
                }),
                Err(error) => {
                    pane.update_in(cx, |pane, window, cx| {
                        let focus = pane.has_focus(window, cx);
                        let failed_to_spawn = cx.new(|cx| FailedToSpawnTerminal {
                            error: error.to_string(),
                            focus_handle: cx.focus_handle(),
                        });
                        pane.add_item(Box::new(failed_to_spawn), true, focus, None, window, cx);
                    })?;
                    Err(error)
                }
            }
        })
    }

    fn spawn_pending_terminal(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        create_terminal: impl AsyncFnOnce(
            WeakEntity<Self>,
            &mut AsyncWindowContext,
        ) -> Result<WeakEntity<Terminal>>
        + 'static,
    ) -> Task<Result<WeakEntity<Terminal>>> {
        self.pending_terminals_to_add += 1;
        cx.notify();
        let decrement_when_cancelled = defer({
            let terminal_panel = cx.weak_entity();
            let cx = cx.to_async();
            move || {
                cx.spawn(async move |cx| {
                    terminal_panel
                        .update(cx, |terminal_panel, cx| {
                            terminal_panel.finish_pending_terminal(cx)
                        })
                        .ok();
                })
                .detach();
            }
        });
        cx.spawn_in(window, async move |terminal_panel, cx| {
            let result = create_terminal(terminal_panel.clone(), cx).await;
            decrement_when_cancelled.abort();
            terminal_panel
                .update(cx, |terminal_panel, cx| {
                    terminal_panel.finish_pending_terminal(cx);
                    if result.is_ok() {
                        terminal_panel.serialize(cx);
                    }
                })
                .ok();
            result
        })
    }

    fn finish_pending_terminal(&mut self, cx: &mut Context<Self>) {
        self.pending_terminals_to_add = self.pending_terminals_to_add.saturating_sub(1);
        cx.notify();
    }

    pub fn flush_serialization(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.pending_serialization = Task::ready(None);
        let revision = self.serialization_revision.clone();
        let captured_revision = revision.load(Ordering::Relaxed);
        let flush = self.serialize_snapshot(cx);
        cx.background_spawn(async move {
            flush.await?;
            anyhow::ensure!(
                revision.load(Ordering::Relaxed) == captured_revision,
                "Terminal panel changed during serialization flush"
            );
            Ok(())
        })
    }

    fn serialize_snapshot(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        if !self.persistence_enabled.load(Ordering::Relaxed) || self.preserve_saved_layout {
            let serialization = self.serialization.clone();
            return cx.background_spawn(async move {
                serialization.await.map_err(|error| anyhow!(error))
            });
        }
        let Some(serialization_key) = self.serialization_key.clone() else {
            return Task::ready(Ok(()));
        };
        let terminals = self
            .center
            .panes()
            .into_iter()
            .flat_map(|pane| pane.read(cx).items())
            .filter_map(|item| item.act_as::<TerminalView>(cx))
            .filter(|terminal| terminal.read(cx).terminal().read(cx).task().is_none())
            .collect::<Vec<_>>();
        if terminals.is_empty()
            && !self.has_serialized_state
            && !self.center.panes().into_iter().any(|pane| {
                pane.read(cx).items().any(|item| {
                    item.downcast::<RestoringItem>().is_some_and(|item| {
                        item.read(cx).serialized_item.kind.as_ref() == "Terminal"
                    })
                })
            })
        {
            return Task::ready(Ok(()));
        }
        let previous = self.serialization.clone();
        let previous = cx
            .background_spawn(async move {
                previous.await.log_err();
            })
            .shared();
        let metadata = terminals
            .into_iter()
            .filter_map(|terminal| {
                let item_id = item_persistence_id(&terminal, cx);
                let previous = previous.clone();
                let item_id = cx
                    .background_spawn(async move {
                        previous.await;
                        item_id.await
                    })
                    .shared();
                terminal.update(cx, |terminal, cx| terminal.serialize_state(item_id, cx))
            })
            .collect::<Vec<_>>();
        let items = serialize_pane_group(&self.center, &self.active_pane, cx);
        let kvp = KeyValueStore::global(cx);
        self.has_serialized_state = true;
        let serialization = cx
            .background_spawn(async move {
                previous.await;
                async move {
                    try_join_all(metadata).await?;
                    let items = items.await?;
                    kvp.write_kvp(
                        serialization_key,
                        serde_json::to_string(&SerializedTerminalPanel {
                            items: SerializedItems::WithSplits(items),
                            active_item_id: None,
                        })?,
                    )
                    .await
                }
                .await
                .map_err(Arc::new)
            })
            .shared();
        self.serialization = serialization.clone();
        cx.background_spawn(async move { serialization.await.map_err(|error| anyhow!(error)) })
    }

    pub(crate) fn cleanup_barrier(
        workspace_id: workspace::WorkspaceId,
        cleanup: BoxFuture<'static, Result<()>>,
        cx: &mut App,
    ) -> BoxFuture<'static, Result<()>> {
        let key = Self::database_serialization_key(workspace_id);
        let panels = cx
            .default_global::<LiveTerminalPanels>()
            .0
            .values()
            .filter_map(WeakEntity::upgrade)
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|panel| panel.read(cx).serialization_key.as_ref() == Some(&key))
            .collect::<Vec<_>>();
        if panels.is_empty() {
            return cleanup;
        }
        let previous = panels
            .iter()
            .map(|panel| panel.read(cx).serialization.clone())
            .collect::<Vec<_>>();
        let (start, started) = oneshot::channel();
        let cleanup = cx
            .background_spawn(async move {
                for previous in previous {
                    previous.await?;
                }
                if started.await.is_ok() {
                    cleanup.await.map_err(Arc::new)
                } else {
                    Ok(())
                }
            })
            .shared();
        for panel in panels {
            panel.update(cx, |panel, _| panel.serialization = cleanup.clone());
        }
        async move {
            start.send(()).ok();
            cleanup.await.map_err(|error| anyhow!(error))
        }
        .boxed()
    }

    pub(crate) fn database_serialization_key(workspace_id: workspace::WorkspaceId) -> String {
        format!(
            "{:?}-{:?}",
            TERMINAL_PANEL_KEY,
            i64::from(workspace_id).to_string()
        )
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        self.serialization_revision.fetch_add(1, Ordering::Relaxed);
        self.schedule_serialization(cx);
    }

    fn schedule_serialization(&mut self, cx: &mut Context<Self>) {
        self.pending_serialization = cx.spawn(async move |terminal_panel, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            let flush = terminal_panel
                .update(cx, |panel, cx| panel.serialize_snapshot(cx))
                .ok()?;
            flush.await.log_err();
            Some(())
        });
    }

    fn replace_terminal(
        &self,
        spawn_task: SpawnInTerminal,
        task_pane: Entity<Pane>,
        terminal_item_index: usize,
        terminal_to_replace: Entity<TerminalView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<WeakEntity<Terminal>>> {
        let reveal = spawn_task.reveal;
        let task_workspace = self.workspace.clone();
        cx.spawn_in(window, async move |terminal_panel, cx| {
            let project = terminal_panel.update(cx, |this, cx| {
                this.workspace
                    .update(cx, |workspace, _| workspace.project().clone())
            })??;
            let new_terminal = project
                .update(cx, |project, cx| {
                    project.create_terminal_task(spawn_task, cx)
                })
                .await?;
            terminal_to_replace.update_in(cx, |terminal_to_replace, window, cx| {
                terminal_to_replace.set_terminal(new_terminal.clone(), window, cx);
            })?;

            let reveal_target = terminal_panel.update(cx, |panel, _| {
                if panel.center.panes().iter().any(|p| **p == task_pane) {
                    RevealTarget::Dock
                } else {
                    RevealTarget::Center
                }
            })?;

            match reveal {
                RevealStrategy::Always => match reveal_target {
                    RevealTarget::Center => {
                        task_workspace.update_in(cx, |workspace, window, cx| {
                            let did_activate = workspace.activate_item(
                                &terminal_to_replace,
                                true,
                                true,
                                window,
                                cx,
                            );

                            anyhow::ensure!(did_activate, "Failed to retrieve terminal pane");

                            anyhow::Ok(())
                        })??;
                    }
                    RevealTarget::Dock => {
                        terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                            terminal_panel.activate_terminal_view(
                                &task_pane,
                                terminal_item_index,
                                true,
                                window,
                                cx,
                            )
                        })?;

                        cx.spawn(async move |cx| {
                            task_workspace
                                .update_in(cx, |workspace, window, cx| {
                                    workspace.focus_panel::<Self>(window, cx)
                                })
                                .ok()
                        })
                        .detach();
                    }
                },
                RevealStrategy::NoFocus => match reveal_target {
                    RevealTarget::Center => {
                        task_workspace.update_in(cx, |workspace, window, cx| {
                            workspace.active_pane().focus_handle(cx).focus(window, cx);
                        })?;
                    }
                    RevealTarget::Dock => {
                        terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                            terminal_panel.activate_terminal_view(
                                &task_pane,
                                terminal_item_index,
                                false,
                                window,
                                cx,
                            )
                        })?;

                        cx.spawn(async move |cx| {
                            task_workspace
                                .update_in(cx, |workspace, window, cx| {
                                    workspace.open_panel::<Self>(window, cx)
                                })
                                .ok()
                        })
                        .detach();
                    }
                },
                RevealStrategy::Never => {}
            }

            Ok(new_terminal.downgrade())
        })
    }

    fn has_no_terminals(&self, cx: &App) -> bool {
        self.active_pane.read(cx).items_len() == 0 && self.pending_terminals_to_add == 0
    }

    pub fn assistant_enabled(&self) -> bool {
        self.assistant_enabled
    }

    /// Returns all panes in the terminal panel.
    pub fn panes(&self) -> Vec<&Entity<Pane>> {
        self.center.panes()
    }

    /// Returns all non-empty terminal selections from all terminal views in all panes.
    pub fn terminal_selections(&self, cx: &App) -> Vec<String> {
        self.center
            .panes()
            .iter()
            .flat_map(|pane| {
                pane.read(cx).items().filter_map(|item| {
                    let terminal_view = item.downcast::<crate::TerminalView>()?;
                    terminal_view
                        .read(cx)
                        .terminal()
                        .read(cx)
                        .last_content
                        .selection_text
                        .clone()
                        .filter(|text| !text.is_empty())
                })
            })
            .collect()
    }

    fn is_enabled(&self, cx: &App) -> bool {
        self.workspace
            .upgrade()
            .is_some_and(|workspace| is_enabled_in_workspace(workspace.read(cx), cx))
    }

    fn activate_pane_in_direction(
        &mut self,
        direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(pane) = self
            .center
            .find_pane_in_direction(&self.active_pane, direction, cx)
        {
            window.focus(&pane.focus_handle(cx), cx);
        } else {
            self.workspace
                .update(cx, |workspace, cx| {
                    workspace.activate_pane_in_direction(direction, window, cx)
                })
                .ok();
        }
    }

    fn swap_pane_in_direction(&mut self, direction: SplitDirection, cx: &mut Context<Self>) {
        if let Some(to) = self
            .center
            .find_pane_in_direction(&self.active_pane, direction, cx)
            .cloned()
        {
            self.center.swap(&self.active_pane, &to, cx);
            self.serialize(cx);
            cx.notify();
        }
    }

    fn move_pane_to_border(&mut self, direction: SplitDirection, cx: &mut Context<Self>) {
        if self
            .center
            .move_to_border(&self.active_pane, direction, cx)
            .unwrap()
        {
            self.serialize(cx);
            cx.notify();
        }
    }
}

/// Prepares a `SpawnInTerminal` by computing the command, args, and command_label
/// based on the shell configuration. This is a pure function that can be tested
/// without spawning actual terminals.
pub fn prepare_task_for_spawn(
    task: &SpawnInTerminal,
    shell: &Shell,
    is_windows: bool,
) -> SpawnInTerminal {
    let builder = ShellBuilder::new(shell, is_windows);
    let command_label = builder.command_label(task.command.as_deref().unwrap_or(""));
    let (command, args) = builder.build_no_quote(task.command.clone(), &task.args);

    SpawnInTerminal {
        command_label,
        command: Some(command),
        args,
        ..task.clone()
    }
}

fn is_enabled_in_workspace(workspace: &Workspace, cx: &App) -> bool {
    workspace.project().read(cx).supports_terminal(cx)
}

pub fn new_terminal_pane(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    zoomed: bool,
    window: &mut Window,
    cx: &mut Context<TerminalPanel>,
) -> Entity<Pane> {
    let terminal_panel = cx.entity();
    let pane = cx.new(|cx| {
        let mut pane = Pane::new(
            workspace.clone(),
            project.clone(),
            Default::default(),
            Some(Arc::new(|dragged, _, cx| {
                dragged
                    .downcast_ref::<DraggedTab>()
                    .and_then(|tab| tab.item.downcast::<RestoringItem>())
                    .is_none_or(|item| item.read(cx).serialized_item.kind.as_ref() == "Terminal")
            })),
            workspace::NewTerminal::default().boxed_clone(),
            false,
            window,
            cx,
        );
        pane.set_zoomed(zoomed, cx);
        pane.set_can_navigate(false, cx);
        pane.display_nav_history_buttons(None);
        pane.set_should_display_tab_bar(|_, _| true);
        pane.set_zoom_out_on_close(false);

        let split_closure_terminal_panel = terminal_panel.downgrade();
        pane.set_can_split(Some(Arc::new(move |pane, dragged_item, _window, cx| {
            if let Some(tab) = dragged_item.downcast_ref::<DraggedTab>() {
                let is_current_pane = tab.pane == cx.entity();
                let Some(can_drag_away) = split_closure_terminal_panel
                    .read_with(cx, |terminal_panel, _| {
                        let current_panes = terminal_panel.center.panes();
                        !current_panes.contains(&&tab.pane)
                            || current_panes.len() > 1
                            || (!is_current_pane || pane.items_len() > 1)
                    })
                    .ok()
                else {
                    return false;
                };
                if can_drag_away {
                    let item = if is_current_pane {
                        pane.item_for_index(tab.ix)
                    } else {
                        tab.pane.read(cx).item_for_index(tab.ix)
                    };
                    if let Some(item) = item {
                        return item.downcast::<TerminalView>().is_some()
                            || item.downcast::<RestoringItem>().is_some_and(|item| {
                                item.read(cx).serialized_item.kind.as_ref() == "Terminal"
                            });
                    }
                }
            }
            false
        })));

        let toolbar = pane.toolbar().clone();
        if let Some(callbacks) = cx.try_global::<workspace::PaneSearchBarCallbacks>() {
            let languages = Some(project.read(cx).languages().clone());
            (callbacks.setup_search_bar)(languages, &toolbar, window, cx);
        }
        let breadcrumbs = cx.new(|_| Breadcrumbs::new());
        toolbar.update(cx, |toolbar, cx| {
            toolbar.add_item(breadcrumbs, window, cx);
        });

        pane
    });

    cx.subscribe_in(&pane, window, TerminalPanel::handle_pane_event)
        .detach();
    cx.observe(&pane, |_, _, cx| cx.notify()).detach();

    pane
}

async fn wait_for_terminals_tasks(
    terminals_for_task: Vec<(usize, Entity<Pane>, Entity<TerminalView>)>,
    cx: &mut AsyncApp,
) {
    let pending_tasks = terminals_for_task.iter().map(|(_, _, terminal)| {
        terminal.update(cx, |terminal_view, cx| {
            terminal_view
                .terminal()
                .update(cx, |terminal, cx| terminal.wait_for_completed_task(cx))
        })
    });
    join_all(pending_tasks).await;
}

struct FailedToSpawnTerminal {
    error: String,
    focus_handle: FocusHandle,
}

impl Focusable for FailedToSpawnTerminal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for FailedToSpawnTerminal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let popover_menu = PopoverMenu::new("settings-popover")
            .trigger(
                IconButton::new("icon-button-popover", IconName::ChevronDown)
                    .icon_size(IconSize::XSmall),
            )
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, |context_menu, _, _| {
                    context_menu
                        .action("Open Settings", zed_actions::OpenSettings.boxed_clone())
                        .action(
                            "Edit settings.json",
                            zed_actions::OpenSettingsFile.boxed_clone(),
                        )
                }))
            })
            .anchor(Anchor::TopRight)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(2.0),
            });

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .p_4()
            .items_center()
            .justify_center()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .max_w_112()
                    .items_center()
                    .justify_center()
                    .text_center()
                    .child(Label::new("Failed to spawn terminal"))
                    .child(
                        Label::new(self.error.to_string())
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .mb_4(),
                    )
                    .child(SplitButton::new(
                        ButtonLike::new("open-settings-ui")
                            .child(Label::new("Edit Settings").size(LabelSize::Small))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(zed_actions::OpenSettings.boxed_clone(), cx);
                            }),
                        popover_menu.into_any_element(),
                    )),
            )
    }
}

impl EventEmitter<()> for FailedToSpawnTerminal {}

impl workspace::Item for FailedToSpawnTerminal {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        SharedString::new_static("Failed to spawn terminal")
    }
}

impl EventEmitter<PanelEvent> for TerminalPanel {}

impl Render for TerminalPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let registrar = cx
            .try_global::<workspace::PaneSearchBarCallbacks>()
            .map(|callbacks| {
                (callbacks.wrap_div_with_search_actions)(div(), self.active_pane.clone())
            })
            .unwrap_or_else(div);
        let no_items_in_panes = self
            .center
            .panes()
            .into_iter()
            .all(|pane| pane.read(cx).items_len() == 0);
        let waiting_for_terminals = self.restoring || self.pending_terminals_to_add > 0;
        let restoring_placeholder = (waiting_for_terminals && no_items_in_panes).then(|| {
            let label = if self.restoring {
                "Restoring terminals…"
            } else {
                "Starting terminal…"
            };
            h_flex()
                .absolute()
                .inset_0()
                .justify_center()
                .gap_2()
                .child(
                    Icon::new(IconName::ArrowCircle)
                        .color(Color::Muted)
                        .size(IconSize::Small)
                        .with_rotate_animation(2),
                )
                .child(Label::new(label).color(Color::Muted))
        });
        self.workspace
            .update(cx, |workspace, cx| {
                registrar
                    .track_focus(&self.focus_handle)
                    .size_full()
                    .relative()
                    .child(self.center.render(
                        workspace.zoomed_item(),
                        None,
                        &workspace::PaneRenderContext {
                            follower_states: &HashMap::default(),
                            active_call: workspace.active_call(),
                            active_pane: &self.active_pane,
                            app_state: workspace.app_state(),
                            project: workspace.project(),
                            workspace: &workspace.weak_handle(),
                        },
                        window,
                        cx,
                    ))
                    .children(restoring_placeholder)
            })
            .ok()
            .map(|div| {
                div.on_action({
                    cx.listener(|terminal_panel, _: &ActivatePaneLeft, window, cx| {
                        terminal_panel.activate_pane_in_direction(SplitDirection::Left, window, cx);
                    })
                })
                .on_action({
                    cx.listener(|terminal_panel, _: &ActivatePaneRight, window, cx| {
                        terminal_panel.activate_pane_in_direction(
                            SplitDirection::Right,
                            window,
                            cx,
                        );
                    })
                })
                .on_action({
                    cx.listener(|terminal_panel, _: &ActivatePaneUp, window, cx| {
                        terminal_panel.activate_pane_in_direction(SplitDirection::Up, window, cx);
                    })
                })
                .on_action({
                    cx.listener(|terminal_panel, _: &ActivatePaneDown, window, cx| {
                        terminal_panel.activate_pane_in_direction(SplitDirection::Down, window, cx);
                    })
                })
                .on_action(
                    cx.listener(|terminal_panel, _action: &ActivateNextPane, window, cx| {
                        let panes = terminal_panel.center.panes();
                        if let Some(ix) = panes
                            .iter()
                            .position(|pane| **pane == terminal_panel.active_pane)
                        {
                            let next_ix = (ix + 1) % panes.len();
                            window.focus(&panes[next_ix].focus_handle(cx), cx);
                        }
                    }),
                )
                .on_action(cx.listener(
                    |terminal_panel, _action: &ActivatePreviousPane, window, cx| {
                        let panes = terminal_panel.center.panes();
                        if let Some(ix) = panes
                            .iter()
                            .position(|pane| **pane == terminal_panel.active_pane)
                        {
                            let prev_ix = cmp::min(ix.wrapping_sub(1), panes.len() - 1);
                            window.focus(&panes[prev_ix].focus_handle(cx), cx);
                        }
                    },
                ))
                .on_action(
                    cx.listener(|terminal_panel, action: &ActivatePane, window, cx| {
                        let panes = terminal_panel.center.panes();
                        if let Some(&pane) = panes.get(action.0) {
                            window.focus(&pane.read(cx).focus_handle(cx), cx);
                        } else {
                            let future =
                                terminal_panel.new_pane_with_active_terminal(true, window, cx);
                            cx.spawn_in(window, async move |terminal_panel, cx| {
                                if let Some(new_pane) = future.await {
                                    _ = terminal_panel.update_in(
                                        cx,
                                        |terminal_panel, window, cx| {
                                            terminal_panel.center.split(
                                                &terminal_panel.active_pane,
                                                &new_pane,
                                                SplitDirection::Right,
                                                cx,
                                            );
                                            terminal_panel.serialize(cx);
                                            let new_pane = new_pane.read(cx);
                                            window.focus(&new_pane.focus_handle(cx), cx);
                                        },
                                    );
                                }
                            })
                            .detach();
                        }
                    }),
                )
                .on_action(cx.listener(|terminal_panel, _: &SwapPaneLeft, _, cx| {
                    terminal_panel.swap_pane_in_direction(SplitDirection::Left, cx);
                }))
                .on_action(cx.listener(|terminal_panel, _: &SwapPaneRight, _, cx| {
                    terminal_panel.swap_pane_in_direction(SplitDirection::Right, cx);
                }))
                .on_action(cx.listener(|terminal_panel, _: &SwapPaneUp, _, cx| {
                    terminal_panel.swap_pane_in_direction(SplitDirection::Up, cx);
                }))
                .on_action(cx.listener(|terminal_panel, _: &SwapPaneDown, _, cx| {
                    terminal_panel.swap_pane_in_direction(SplitDirection::Down, cx);
                }))
                .on_action(cx.listener(|terminal_panel, _: &MovePaneLeft, _, cx| {
                    terminal_panel.move_pane_to_border(SplitDirection::Left, cx);
                }))
                .on_action(cx.listener(|terminal_panel, _: &MovePaneRight, _, cx| {
                    terminal_panel.move_pane_to_border(SplitDirection::Right, cx);
                }))
                .on_action(cx.listener(|terminal_panel, _: &MovePaneUp, _, cx| {
                    terminal_panel.move_pane_to_border(SplitDirection::Up, cx);
                }))
                .on_action(cx.listener(|terminal_panel, _: &MovePaneDown, _, cx| {
                    terminal_panel.move_pane_to_border(SplitDirection::Down, cx);
                }))
                .on_action(
                    cx.listener(|terminal_panel, action: &MoveItemToPane, window, cx| {
                        let Some(&target_pane) =
                            terminal_panel.center.panes().get(action.destination)
                        else {
                            return;
                        };
                        move_active_item(
                            &terminal_panel.active_pane,
                            target_pane,
                            action.focus,
                            true,
                            window,
                            cx,
                        );
                    }),
                )
                .on_action(cx.listener(
                    |terminal_panel, action: &MoveItemToPaneInDirection, window, cx| {
                        let source_pane = &terminal_panel.active_pane;
                        if let Some(destination_pane) = terminal_panel
                            .center
                            .find_pane_in_direction(source_pane, action.direction, cx)
                        {
                            move_active_item(
                                source_pane,
                                destination_pane,
                                action.focus,
                                true,
                                window,
                                cx,
                            );
                        };
                    },
                ))
            })
            .unwrap_or_else(|| div())
    }
}

impl Focusable for TerminalPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for TerminalPanel {
    fn activation_focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane.focus_handle(cx)
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        TerminalSettings::get_global(cx).dock.into()
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn starts_open(&self, _: &Window, cx: &App) -> bool {
        TerminalSettings::get_global(cx).starts_open
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            let dock = match position {
                DockPosition::Left => TerminalDockPosition::Left,
                DockPosition::Bottom => TerminalDockPosition::Bottom,
                DockPosition::Right => TerminalDockPosition::Right,
            };
            settings.terminal.get_or_insert_default().dock = Some(dock);
        });
    }

    fn default_size(&self, window: &Window, cx: &App) -> Pixels {
        let settings = TerminalSettings::get_global(cx);
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => settings.default_width,
            DockPosition::Bottom => settings.default_height,
        }
    }

    fn supports_flexible_size(&self) -> bool {
        true
    }

    fn has_flexible_size(&self, _window: &Window, cx: &App) -> bool {
        TerminalSettings::get_global(cx).flexible
    }

    fn set_flexible_size(&mut self, flexible: bool, _window: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.terminal.get_or_insert_default().flexible = Some(flexible);
        });
    }

    fn is_zoomed(&self, _window: &Window, cx: &App) -> bool {
        self.active_pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, _: &mut Window, cx: &mut Context<Self>) {
        for pane in self.center.panes() {
            pane.update(cx, |pane, cx| {
                pane.set_zoomed(zoomed, cx);
            })
        }
        cx.notify();
    }

    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {
        let old_active = self.active;
        self.active = active;
        if !active || old_active == active || self.restoring || !self.has_no_terminals(cx) {
            return;
        }
        cx.defer_in(window, |this, window, cx| {
            let Ok(kind) = this
                .workspace
                .update(cx, |workspace, cx| default_working_directory(workspace, cx))
            else {
                return;
            };

            this.add_terminal_shell(false, kind, RevealStrategy::Always, window, cx)
                .detach_and_log_err(cx)
        })
    }

    fn icon_label(&self, _window: &Window, cx: &App) -> Option<String> {
        if !TerminalSettings::get_global(cx).show_count_badge {
            return None;
        }
        let count = self
            .center
            .panes()
            .into_iter()
            .map(|pane| pane.read(cx).items_len())
            .sum::<usize>();
        if count == 0 {
            None
        } else {
            Some(count.to_string())
        }
    }

    fn persistent_name() -> &'static str {
        "TerminalPanel"
    }

    fn panel_key() -> &'static str {
        TERMINAL_PANEL_KEY
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        if (self.is_enabled(cx) || !self.has_no_terminals(cx))
            && TerminalSettings::get_global(cx).button
        {
            Some(IconName::TerminalAlt)
        } else {
            None
        }
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Terminal Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(Toggle)
    }

    fn pane(&self) -> Option<Entity<Pane>> {
        Some(self.active_pane.clone())
    }

    fn activation_priority(&self) -> u32 {
        2
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.terminal.get_or_insert_default().button = Some(false);
        }))
    }
}

struct TerminalProvider(Entity<TerminalPanel>);

impl workspace::TerminalProvider for TerminalProvider {
    fn serialization_revision(&self, cx: &App) -> Arc<AtomicUsize> {
        self.0.read(cx).serialization_revision.clone()
    }

    fn flush_serialization(&self, cx: &mut App) -> Task<Result<()>> {
        self.0.update(cx, |panel, cx| panel.flush_serialization(cx))
    }

    fn spawn(
        &self,
        task: SpawnInTerminal,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Option<Result<ExitStatus>>> {
        let terminal_panel = self.0.clone();
        window.spawn(cx, async move |cx| {
            let terminal = terminal_panel
                .update_in(cx, |terminal_panel, window, cx| {
                    terminal_panel.spawn_task(&task, window, cx)
                })
                .ok()?
                .await;
            match terminal {
                Ok(terminal) => {
                    let exit_status = terminal
                        .read_with(cx, |terminal, cx| terminal.wait_for_completed_task(cx))
                        .ok()?
                        .await?;
                    Some(Ok(exit_status))
                }
                Err(e) => Some(Err(e)),
            }
        })
    }
}

#[derive(IntoElement)]
struct InlineAssistTabBarButton {
    focus_handle: FocusHandle,
}

impl RenderOnce for InlineAssistTabBarButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let focus_handle = self.focus_handle;
        IconButton::new("terminal_inline_assistant", IconName::ZedAssistant)
            .icon_size(IconSize::Small)
            .on_click({
                let focus_handle = focus_handle.clone();
                move |_, window, cx| {
                    focus_handle.dispatch_action(&InlineAssist::default(), window, cx);
                }
            })
            .tooltip(move |_window, cx| {
                Tooltip::for_action_in("Inline Assist", &InlineAssist::default(), &focus_handle, cx)
            })
    }
}

#[derive(Default)]
struct LiveTerminalPanels(HashMap<gpui::EntityId, WeakEntity<TerminalPanel>>);

impl gpui::Global for LiveTerminalPanels {}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use super::*;
    use crate::persistence::{
        SerializedPane, SerializedPaneGroup, hydrate_terminal, initialize_terminal_panel,
    };
    use editor::Editor;
    use gpui::{EntityId, Modifiers, TestAppContext, UpdateGlobal as _, VisualTestContext};
    use language::Buffer;
    use pretty_assertions::assert_eq;
    use project::FakeFs;
    use settings::SettingsStore;
    use workspace::{ItemId, MultiWorkspace, SerializedItem, WorkspaceId};

    #[test]
    fn test_prepare_empty_task() {
        let input = SpawnInTerminal::default();
        let shell = Shell::System;

        let result = prepare_task_for_spawn(&input, &shell, false);

        let expected_shell = util::get_system_shell();
        assert_eq!(result.env, HashMap::default());
        assert_eq!(result.cwd, None);
        assert_eq!(result.shell, Shell::System);
        assert_eq!(
            result.command,
            Some(expected_shell.clone()),
            "Empty tasks should spawn a -i shell"
        );
        assert_eq!(result.args, Vec::<String>::new());
        assert_eq!(
            result.command_label, expected_shell,
            "We show the shell launch for empty commands"
        );
    }

    #[gpui::test]
    async fn test_bypass_max_tabs_limit(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        set_max_tabs(cx, Some(3));

        for _ in 0..5 {
            let task = window_handle
                .update(cx, |_, window, cx| {
                    terminal_panel.update(cx, |panel, cx| {
                        panel.add_terminal_shell(false, None, RevealStrategy::Always, window, cx)
                    })
                })
                .unwrap();
            task.await.unwrap();
        }

        cx.run_until_parked();

        let item_count =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());

        assert_eq!(
            item_count, 5,
            "Terminal panel should bypass max_tabs limit and have all 5 terminals"
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_prepare_script_like_task() {
        let user_command = r#"REPO_URL=$(git remote get-url origin | sed -e \"s/^git@\\(.*\\):\\(.*\\)\\.git$/https:\\/\\/\\1\\/\\2/\"); COMMIT_SHA=$(git log -1 --format=\"%H\" -- \"${ZED_RELATIVE_FILE}\"); echo \"${REPO_URL}/blob/${COMMIT_SHA}/${ZED_RELATIVE_FILE}#L${ZED_ROW}-$(echo $(($(wc -l <<< \"$ZED_SELECTED_TEXT\") + $ZED_ROW - 1)))\" | xclip -selection clipboard"#.to_string();
        let expected_cwd = PathBuf::from("/some/work");

        let input = SpawnInTerminal {
            command: Some(user_command.clone()),
            cwd: Some(expected_cwd.clone()),
            ..SpawnInTerminal::default()
        };
        let shell = Shell::System;

        let result = prepare_task_for_spawn(&input, &shell, false);

        let system_shell = util::get_system_shell();
        assert_eq!(result.env, HashMap::default());
        assert_eq!(result.cwd, Some(expected_cwd));
        assert_eq!(result.shell, Shell::System);
        assert_eq!(result.command, Some(system_shell.clone()));
        assert_eq!(
            result.args,
            vec!["-i".to_string(), "-c".to_string(), user_command.clone()],
            "User command should have been moved into the arguments, as we're spawning a new -i shell",
        );
        assert_eq!(
            result.command_label,
            format!(
                "{system_shell} {interactive}-c '{user_command}'",
                interactive = if cfg!(windows) { "" } else { "-i " }
            ),
            "We want to show to the user the entire command spawned"
        );
    }

    #[gpui::test]
    async fn renders_error_if_default_shell_fails(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.terminal.get_or_insert_default().project.shell =
                        Some(settings::Shell::Program("__nonexistent_shell__".to_owned()));
                });
            });
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    terminal_panel.add_terminal_shell(
                        false,
                        None,
                        RevealStrategy::Always,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await
            .unwrap_err();

        window_handle
            .update(cx, |_, _, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    assert!(
                        terminal_panel
                            .active_pane
                            .read(cx)
                            .items()
                            .any(|item| item.downcast::<FailedToSpawnTerminal>().is_some()),
                        "should spawn `FailedToSpawnTerminal` pane"
                    );
                    assert_eq!(terminal_panel.pending_terminals_to_add, 0);
                })
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_failed_task_spawn_does_not_leak_pending_terminal_count(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    terminal_panel.add_terminal_task(
                        SpawnInTerminal {
                            command: Some("__nonexistent_program__".to_owned()),
                            ..SpawnInTerminal::default()
                        },
                        RevealStrategy::Never,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await
            .unwrap_err();

        cx.run_until_parked();
        terminal_panel.read_with(cx, |terminal_panel, cx| {
            assert_eq!(terminal_panel.pending_terminals_to_add, 0);
            assert_eq!(terminal_panel.active_pane.read(cx).items_len(), 0);
        });
    }

    #[gpui::test]
    async fn test_pending_terminal_count_tracks_spawn_lifecycle(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        let task = window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    let task =
                        terminal_panel.add_terminal_shell(false, None, RevealStrategy::Never, window, cx);
                    assert_eq!(
                        terminal_panel.pending_terminals_to_add, 1,
                        "pending count should be incremented synchronously to avoid double default terminal spawns"
                    );
                    assert!(!terminal_panel.has_no_terminals(cx));
                    task
                })
            })
            .unwrap();
        task.await.unwrap();

        cx.run_until_parked();
        terminal_panel.read_with(cx, |terminal_panel, cx| {
            assert_eq!(terminal_panel.pending_terminals_to_add, 0);
            assert_eq!(terminal_panel.active_pane.read(cx).items_len(), 1);
        });
    }

    #[gpui::test]
    async fn test_pending_terminal_count_resets_when_spawn_cancelled(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    let task = terminal_panel.add_terminal_shell(
                        false,
                        None,
                        RevealStrategy::Never,
                        window,
                        cx,
                    );
                    assert_eq!(terminal_panel.pending_terminals_to_add, 1);
                    drop(task);
                })
            })
            .unwrap();

        cx.run_until_parked();
        terminal_panel.read_with(cx, |terminal_panel, _| {
            assert_eq!(terminal_panel.pending_terminals_to_add, 0);
        });
    }

    #[gpui::test]
    async fn test_load_without_serialized_state_does_not_persist_empty_state(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let serialization_key = window_handle
            .update(cx, |multi_workspace, _, cx| {
                TerminalPanel::serialization_key(multi_workspace.workspace().read(cx)).unwrap()
            })
            .unwrap();

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().downgrade();
                window.spawn(cx, async move |cx| {
                    TerminalPanel::load(workspace, cx.clone()).await
                })
            })
            .unwrap()
            .await
            .unwrap();

        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();

        terminal_panel.read_with(cx, |terminal_panel, cx| {
            assert!(!terminal_panel.restoring);
            assert_eq!(terminal_panel.active_pane.read(cx).items_len(), 0);
        });
        let serialized_state = cx
            .update(|cx| KeyValueStore::global(cx))
            .read_kvp(&serialization_key)
            .unwrap();
        assert_eq!(serialized_state, None);
    }

    #[gpui::test]
    async fn test_malformed_layout_keeps_panel_usable_and_preserves_saved_state(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings.terminal.get_or_insert_default().starts_open = Some(true);
            });
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window_handle
            .update(cx, |multi_workspace, _, cx| {
                multi_workspace.set_random_database_id(cx);
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);
        let (workspace_id, serialization_key) = workspace.read_with(cx, |workspace, _| {
            (
                workspace.database_id().unwrap(),
                TerminalPanel::serialization_key(workspace).unwrap(),
            )
        });
        let db = cx.update(|_, cx| crate::persistence::TerminalDb::global(cx));
        db.write(move |connection| {
            connection.exec_bound("INSERT INTO workspaces (workspace_id) VALUES (?)")?(workspace_id)
        })
        .await
        .unwrap();
        db.save_custom_title(12345, workspace_id, Some("saved".to_string()))
            .await
            .unwrap();
        let saved_directory = PathBuf::from("/saved/terminal");
        db.save_working_directory(12345, workspace_id, saved_directory.clone())
            .await
            .unwrap();
        let editor = workspace.update_in(cx, |workspace, window, cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx));
            let editor = cx.new(|cx| {
                Editor::for_buffer(buffer, Some(workspace.project().clone()), window, cx)
            });
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, false, window, cx);
            editor
        });
        let editor_id = cx
            .update(|_, cx| item_persistence_id(&editor, cx))
            .await
            .unwrap();
        let kvp = cx.update(|_, cx| KeyValueStore::global(cx));
        let malformed_layout = "{";
        kvp.write_kvp(serialization_key.clone(), malformed_layout.to_string())
            .await
            .unwrap();

        for contents in ["unsaved editor", "editor updated during recovery"] {
            workspace.update_in(cx, |workspace, window, cx| {
                if let Some(panel) = workspace.panel::<TerminalPanel>(cx) {
                    workspace.remove_panel(&panel, window, cx);
                }
                workspace.dismiss_notification(&NotificationId::unique::<TerminalPanel>(), cx);
            });
            editor.update_in(cx, |editor, window, cx| {
                editor.set_text(contents, window, cx);
            });
            let terminal_panel = cx
                .update(|window, cx| {
                    let workspace = workspace.downgrade();
                    window.spawn(cx, async move |cx| {
                        TerminalPanel::load(workspace, cx.clone()).await
                    })
                })
                .await
                .unwrap();
            workspace.update_in(cx, |workspace, window, cx| {
                workspace.add_panel(terminal_panel.clone(), window, cx);
                assert_eq!(
                    workspace.panel::<TerminalPanel>(cx),
                    Some(terminal_panel.clone())
                );
            });
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
            cx.run_until_parked();
            terminal_panel.read_with(cx, |panel, cx| {
                assert!(!panel.restoring);
                assert_eq!(panel.active_pane.read(cx).items_len(), 1);
            });
            workspace.update_in(cx, |workspace, window, cx| {
                TerminalPanel::new_terminal(
                    workspace,
                    &workspace::NewTerminal::default(),
                    window,
                    cx,
                );
            });
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
            cx.run_until_parked();
            terminal_panel.read_with(cx, |panel, cx| {
                assert_eq!(panel.active_pane.read(cx).items_len(), 2);
            });
            cx.update(|window, cx| TerminalView::cleanup(workspace_id, Vec::new(), window, cx))
                .await
                .unwrap();
            window_handle
                .update(cx, |multi_workspace, window, cx| {
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .unwrap()
                .await
                .unwrap();
            assert_eq!(
                db.select_bound::<_, (ItemId, String)>(
                    "SELECT items.item_id, editors.contents FROM items
                     JOIN editors USING (workspace_id, item_id)
                     WHERE items.workspace_id = ? AND items.kind = 'Editor'",
                )
                .unwrap()(workspace_id)
                .unwrap(),
                vec![(editor_id, contents.to_string())]
            );
            assert_eq!(
                kvp.read_kvp(&serialization_key).unwrap().as_deref(),
                Some(malformed_layout)
            );
            assert_eq!(
                db.get_custom_title(12345, workspace_id).unwrap().as_deref(),
                Some("saved")
            );
            assert_eq!(
                db.get_working_directory(12345, workspace_id)
                    .unwrap()
                    .as_ref(),
                Some(&saved_directory)
            );
            workspace.read_with(cx, |workspace, _| {
                assert!(workspace.has_notification(&NotificationId::unique::<TerminalPanel>()));
            });
        }

        let terminal_panel = workspace.read_with(cx, |workspace, cx| {
            workspace.panel::<TerminalPanel>(cx).unwrap()
        });
        let item_ids = cx.update(|_, cx| {
            let terminals = terminal_panel
                .read(cx)
                .active_pane
                .read(cx)
                .items()
                .map(|item| item.downcast::<TerminalView>().unwrap())
                .collect::<Vec<_>>();
            terminals
                .iter()
                .map(|terminal| item_persistence_id(terminal, cx))
                .collect::<Vec<_>>()
        });
        let item_ids = try_join_all(item_ids).await.unwrap();
        db.write(|connection| {
            connection.exec(
                "CREATE TRIGGER reject_terminal_write BEFORE INSERT ON terminals
                 BEGIN SELECT RAISE(ABORT, 'terminal write rejected'); END",
            )?()
        })
        .await
        .unwrap();
        terminal_panel.update(cx, |panel, cx| panel.replace_failed_layout(cx));
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization_checked(window, cx)
            })
            .await
            .unwrap_err();
        assert_eq!(
            kvp.read_kvp(&serialization_key).unwrap().as_deref(),
            Some(malformed_layout)
        );
        assert_eq!(
            db.get_custom_title(12345, workspace_id).unwrap().as_deref(),
            Some("saved")
        );
        db.write(|connection| connection.exec("DROP TRIGGER reject_terminal_write")?())
            .await
            .unwrap();
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization_checked(window, cx)
            })
            .await
            .unwrap();
        let saved = kvp.read_kvp(&serialization_key).unwrap().unwrap();
        assert_eq!(
            serde_json::from_str::<SerializedTerminalPanel>(&saved)
                .unwrap()
                .into_pane_group()
                .item_ids(),
            item_ids
        );
    }

    #[gpui::test]
    async fn test_terminal_added_during_restore_is_flushed_before_restore_finishes(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let (workspace_id, serialization_key) = window_handle
            .update(cx, |multi_workspace, _, cx| {
                multi_workspace.set_random_database_id(cx);
                let workspace = multi_workspace.workspace().read(cx);
                (
                    workspace.database_id().unwrap(),
                    TerminalPanel::serialization_key(workspace).unwrap(),
                )
            })
            .unwrap();
        let db = cx.update(|cx| crate::persistence::TerminalDb::global(cx));
        db.write(move |connection| {
            connection.exec_bound("INSERT INTO workspaces (workspace_id) VALUES (?)")?(workspace_id)
        })
        .await
        .unwrap();
        db.save_custom_title(12345, workspace_id, Some("saved".to_string()))
            .await
            .unwrap();
        for (item_id, title) in [(12346, "pending"), (12347, "closed"), (12348, "orphan")] {
            db.save_custom_title(item_id, workspace_id, Some(title.to_string()))
                .await
                .unwrap();
        }
        db.save_working_directory(12346, workspace_id, PathBuf::from("/saved/pending"))
            .await
            .unwrap();
        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::add_center_terminal(workspace, window, cx, |project, cx| {
                        project.create_terminal_shell(None, cx)
                    })
                })
            })
            .unwrap()
            .await
            .unwrap();
        let center_terminal = window_handle
            .update(cx, |multi_workspace, _, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_item(cx)
                    .unwrap()
                    .downcast::<TerminalView>()
                    .unwrap()
            })
            .unwrap();
        let center_id = cx
            .update(|cx| item_persistence_id(&center_terminal, cx))
            .await
            .unwrap();
        center_terminal
            .update(cx, |terminal, cx| {
                terminal.set_custom_title(Some("center".to_string()), cx);
                terminal
                    .serialize_state(Task::ready(Ok(center_id)).shared(), cx)
                    .unwrap()
            })
            .await
            .unwrap();

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    terminal_panel.restoring = true;

                    terminal_panel.add_terminal_shell(
                        false,
                        None,
                        RevealStrategy::Never,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await
            .unwrap();

        let revision = cx.update(|cx| {
            workspace::TerminalProvider::serialization_revision(
                &TerminalProvider(terminal_panel.clone()),
                cx,
            )
        });
        let before_restoration = revision.load(Ordering::Relaxed);
        let (mut senders, placeholder_ids) = window_handle
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                let project = workspace.read(cx).project().clone();
                let placeholders = initialize_terminal_panel(
                    workspace.downgrade(),
                    project,
                    SerializedTerminalPanel {
                        items: SerializedItems::NoSplits(vec![12345, 12346, 12347]),
                        active_item_id: Some(12345),
                    },
                    &terminal_panel.downgrade(),
                    window,
                    cx,
                )
                .unwrap();
                let placeholder_ids = placeholders
                    .iter()
                    .map(Entity::entity_id)
                    .collect::<Vec<_>>();
                let mut senders = Vec::new();
                let mut tasks = Vec::new();
                for placeholder in placeholders {
                    let (sender, receiver) = oneshot::channel();
                    senders.push(sender);
                    let deserialize = cx.background_spawn(async move { receiver.await.unwrap() });
                    tasks.push(hydrate_terminal(
                        workspace.downgrade(),
                        placeholder,
                        deserialize,
                        window,
                        cx,
                    ));
                }
                terminal_panel.update(cx, |panel, cx| {
                    panel._restoration = cx.background_spawn(async move {
                        join_all(tasks).await;
                    });
                });
                (senders, placeholder_ids)
            })
            .unwrap();
        assert_eq!(revision.load(Ordering::Relaxed), before_restoration);
        let terminal_view = terminal_panel.read_with(cx, |panel, cx| {
            panel
                .active_pane
                .read(cx)
                .active_item()
                .unwrap()
                .downcast::<TerminalView>()
                .unwrap()
        });
        terminal_view.update(cx, |view, cx| {
            view.set_custom_title(Some("interim".to_string()), cx);
        });
        window_handle
            .update(cx, |_, window, cx| {
                let pane = terminal_panel.read(cx).active_pane.clone();
                pane.update(cx, |pane, cx| {
                    let item = pane
                        .items()
                        .find(|item| item.item_id() == placeholder_ids[0])
                        .unwrap()
                        .boxed_clone();
                    pane.add_item_inner(item, false, false, false, Some(0), window, cx);
                    pane.remove_item(placeholder_ids[2], false, false, window, cx);
                    let index = pane.index_for_item(&terminal_view).unwrap();
                    pane.activate_item(index, false, false, window, cx);
                });
            })
            .unwrap();
        cx.update(|cx| {
            assert_eq!(
                terminal_panel
                    .read(cx)
                    .active_pane
                    .read(cx)
                    .items()
                    .map(|item| item.item_id())
                    .collect::<Vec<_>>(),
                vec![
                    placeholder_ids[0],
                    terminal_view.entity_id(),
                    placeholder_ids[1]
                ]
            );
        });
        cx.run_until_parked();
        assert!(senders.last().unwrap().is_canceled());
        let restored_terminal = window_handle
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                TerminalView::deserialize(
                    workspace.read(cx).project().clone(),
                    workspace.downgrade(),
                    workspace_id,
                    12345,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();
        let restored_id = cx
            .update(|cx| item_persistence_id(&restored_terminal, cx))
            .await
            .unwrap();
        assert_ne!(restored_id, 12345);
        assert_eq!(
            restored_terminal
                .read_with(cx, |terminal, _| terminal.custom_title().map(str::to_owned)),
            Some("saved".to_string())
        );
        let before_hydration = revision.load(Ordering::Relaxed);
        senders
            .remove(0)
            .send(Ok(restored_terminal.clone()))
            .unwrap();
        cx.run_until_parked();
        assert_eq!(revision.load(Ordering::Relaxed), before_hydration);
        terminal_panel.read_with(cx, |panel, cx| {
            let pane = panel.active_pane.read(cx);
            assert_eq!(
                pane.item_for_index(0).unwrap().item_id(),
                restored_terminal.entity_id()
            );
            assert_eq!(
                pane.active_item().unwrap().item_id(),
                terminal_view.entity_id()
            );
        });
        restored_terminal.update(cx, |terminal, cx| {
            terminal.set_custom_title(Some("edited saved".to_string()), cx);
        });
        let flush = cx.update(|cx| {
            workspace::TerminalProvider::flush_serialization(
                &TerminalProvider(terminal_panel.clone()),
                cx,
            )
        });
        flush.await.unwrap();
        let interim_id = cx
            .update(|cx| item_persistence_id(&terminal_view, cx))
            .await
            .unwrap();
        assert_ne!(interim_id, terminal_view.entity_id().as_u64());
        let saved = cx
            .update(|cx| KeyValueStore::global(cx))
            .read_kvp(&serialization_key)
            .unwrap()
            .unwrap();
        let saved = serde_json::from_str::<SerializedTerminalPanel>(&saved)
            .unwrap()
            .into_pane_group();
        assert_eq!(saved.item_ids(), vec![restored_id, interim_id, 12346]);
        assert_eq!(
            db.get_custom_title(interim_id, workspace_id)
                .unwrap()
                .as_deref(),
            Some("interim")
        );
        assert_eq!(
            db.get_custom_title(restored_id, workspace_id)
                .unwrap()
                .as_deref(),
            Some("edited saved")
        );
        assert_eq!(
            db.get_custom_title(12345, workspace_id).unwrap().as_deref(),
            Some("saved")
        );
        assert_eq!(
            db.get_custom_title(12346, workspace_id).unwrap().as_deref(),
            Some("pending")
        );
        assert_eq!(
            db.get_working_directory(12346, workspace_id).unwrap(),
            Some(PathBuf::from("/saved/pending"))
        );
        terminal_panel.read_with(cx, |panel, _| assert!(panel.restoring));
        window_handle
            .update(cx, |_, window, cx| {
                TerminalView::cleanup(workspace_id, Vec::new(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        let mut retained_ids = db.item_ids(workspace_id).unwrap();
        retained_ids.sort_unstable();
        let mut expected_ids = vec![restored_id, 12346, center_id, interim_id];
        expected_ids.sort_unstable();
        assert_eq!(retained_ids, expected_ids);
        assert_eq!(
            db.get_custom_title(center_id, workspace_id)
                .unwrap()
                .as_deref(),
            Some("center")
        );

        let pending = senders.remove(0);
        pending
            .send(Err(anyhow!("terminal startup failed")))
            .unwrap();
        cx.run_until_parked();
        terminal_panel
            .update(cx, |panel, cx| panel.flush_serialization(cx))
            .await
            .unwrap();
        let serialized_state = cx
            .update(|cx| KeyValueStore::global(cx))
            .read_kvp(&serialization_key)
            .unwrap()
            .unwrap();
        let serialized =
            serde_json::from_str::<SerializedTerminalPanel>(&serialized_state).unwrap();
        assert_eq!(
            serialized.into_pane_group().item_ids(),
            vec![restored_id, interim_id, 12346]
        );
        assert_eq!(
            db.get_custom_title(12346, workspace_id).unwrap().as_deref(),
            Some("pending")
        );
        let captured_revision = revision.load(Ordering::Relaxed);
        restored_terminal.update(cx, |terminal, _| {
            terminal.custom_title = Some("captured".to_string());
        });
        let (release, wait) = oneshot::channel();
        let flush = terminal_panel.update(cx, |panel, cx| {
            let db = db.clone();
            panel.serialization = cx
                .background_spawn(async move {
                    wait.await.unwrap();
                    db.save_custom_title(restored_id, workspace_id, Some("older".to_string()))
                        .await
                        .map_err(Arc::new)
                })
                .shared();
            panel.flush_serialization(cx)
        });
        restored_terminal.update(cx, |terminal, _| {
            terminal.custom_title = Some("uncaptured".to_string());
        });
        cx.run_until_parked();
        assert_eq!(
            db.get_custom_title(restored_id, workspace_id)
                .unwrap()
                .as_deref(),
            Some("edited saved")
        );
        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel
                    .read(cx)
                    .active_pane
                    .clone()
                    .update(cx, |pane, cx| {
                        pane.remove_item(placeholder_ids[1], false, false, window, cx);
                    });
            })
            .unwrap();
        cx.run_until_parked();
        assert!(revision.load(Ordering::Relaxed) > captured_revision);
        release.send(()).unwrap();
        assert_eq!(
            flush.await.unwrap_err().to_string(),
            "Terminal panel changed during serialization flush"
        );
        assert_eq!(
            db.get_custom_title(restored_id, workspace_id)
                .unwrap()
                .as_deref(),
            Some("captured")
        );
        let flush = terminal_panel.update(cx, |panel, cx| panel.flush_serialization(cx));
        let weak_panel = terminal_panel.downgrade();
        drop(terminal_panel);
        assert!(weak_panel.upgrade().is_none());
        flush.await.unwrap();
        assert_eq!(
            cx.update(|cx| stored_item_ids(workspace_id, cx)).unwrap(),
            vec![restored_id, interim_id]
        );
        assert_eq!(
            db.get_custom_title(restored_id, workspace_id)
                .unwrap()
                .as_deref(),
            Some("uncaptured")
        );
    }

    #[gpui::test]
    async fn test_terminal_restore_moves_preserve_persistence(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;
        let other_project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let (workspace, other_workspace, panel, other_panel) = window_handle
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                let other_workspace = multi_workspace.test_add_workspace(other_project, window, cx);
                let panels = [&workspace, &other_workspace].map(|workspace| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.set_random_database_id();
                        let panel = cx.new(|cx| TerminalPanel::new(workspace, window, cx));
                        workspace.set_terminal_provider(TerminalProvider(panel.clone()));
                        panel
                    })
                });
                let [panel, other_panel] = panels;
                (workspace, other_workspace, panel, other_panel)
            })
            .unwrap();
        let workspace_id = workspace.read_with(cx, |workspace, _| workspace.database_id().unwrap());
        let other_workspace_id =
            other_workspace.read_with(cx, |workspace, _| workspace.database_id().unwrap());
        let db = cx.update(|cx| crate::persistence::TerminalDb::global(cx));
        db.write(move |connection| {
            connection.exec_bound("INSERT INTO workspaces (workspace_id) VALUES (?), (?)")?((
                workspace_id,
                other_workspace_id,
            ))
        })
        .await
        .unwrap();
        let cwd = std::env::current_dir().unwrap();
        db.save_working_directory(12345, workspace_id, cwd.clone())
            .await
            .unwrap();
        db.save_custom_title(12345, workspace_id, Some("saved".to_string()))
            .await
            .unwrap();
        db.save_custom_title(12347, workspace_id, Some("orphan".to_string()))
            .await
            .unwrap();
        let center = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
        let panel_pane = panel.read_with(cx, |panel, _| panel.active_pane.clone());
        let other_pane = other_panel.read_with(cx, |panel, _| panel.active_pane.clone());
        let (placeholder_id, editor_id, sender, hydration) = window_handle
            .update(cx, |_, window, cx| {
                let placeholder = cx.new(|cx| {
                    RestoringItem::new(SerializedItem::new("Terminal", 12345, false, false), cx)
                });
                let editor = cx.new(|cx| {
                    RestoringItem::new(SerializedItem::new("Editor", 12346, false, false), cx)
                });
                center.update(cx, |pane, cx| {
                    pane.add_item(
                        Box::new(placeholder.clone()),
                        false,
                        false,
                        None,
                        window,
                        cx,
                    );
                    pane.add_item(Box::new(editor.clone()), false, false, None, window, cx);
                });
                let placeholder_id = placeholder.entity_id();
                let (sender, receiver) = oneshot::channel();
                let deserialize = cx.background_spawn(async move { receiver.await.unwrap() });
                let hydration =
                    hydrate_terminal(workspace.downgrade(), placeholder, deserialize, window, cx);
                (placeholder_id, editor.entity_id(), sender, hydration)
            })
            .unwrap();
        window_handle
            .update(cx, |_, window, cx| {
                drop_tab(editor_id, &center, &panel_pane, window, cx);
                drop_tab(placeholder_id, &center, &other_pane, window, cx);
            })
            .unwrap();
        cx.run_until_parked();
        center.read_with(cx, |pane, _| {
            assert_eq!(
                pane.items().map(|item| item.item_id()).collect::<Vec<_>>(),
                vec![placeholder_id, editor_id]
            );
        });
        panel_pane.read_with(cx, |pane, _| assert_eq!(pane.items_len(), 0));
        other_pane.read_with(cx, |pane, _| assert_eq!(pane.items_len(), 0));
        for (source, destination) in [
            (&center, &panel_pane),
            (&panel_pane, &center),
            (&center, &panel_pane),
        ] {
            window_handle
                .update(cx, |_, window, cx| {
                    drop_tab(placeholder_id, source, destination, window, cx);
                })
                .unwrap();
            cx.run_until_parked();
            destination.read_with(cx, |pane, _| {
                assert_eq!(pane.active_item().unwrap().item_id(), placeholder_id);
            });
        }
        window_handle
            .update(cx, |_, window, cx| {
                workspace.update(cx, |workspace, cx| {
                    workspace.flush_serialization_checked(window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            cx.update(|cx| stored_item_ids(workspace_id, cx)).unwrap(),
            vec![12345]
        );
        assert_eq!(
            db.get_working_directory(12345, workspace_id).unwrap(),
            Some(cwd.clone())
        );
        assert_eq!(
            db.get_custom_title(12345, workspace_id).unwrap().as_deref(),
            Some("saved")
        );
        let terminal = window_handle
            .update(cx, |_, window, cx| {
                TerminalView::deserialize(
                    workspace.read(cx).project().clone(),
                    workspace.downgrade(),
                    workspace_id,
                    12345,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();
        let restored_id = cx
            .update(|cx| item_persistence_id(&terminal, cx))
            .await
            .unwrap();
        assert_ne!(restored_id, 12345);
        let (release, wait) = oneshot::channel();
        panel.update(cx, |panel, cx| {
            let previous = panel.serialization.clone();
            panel.serialization = cx
                .background_spawn(async move {
                    previous.await?;
                    wait.await.unwrap();
                    Ok(())
                })
                .shared();
        });
        sender.send(Ok(terminal.clone())).unwrap();
        assert!(hydration.await);
        cx.run_until_parked();
        panel_pane.read_with(cx, |pane, _| {
            assert_eq!(pane.active_item().unwrap().item_id(), terminal.entity_id());
        });
        let cleanup = window_handle
            .update(cx, |_, window, cx| {
                TerminalView::cleanup(workspace_id, Vec::new(), window, cx)
            })
            .unwrap();
        let flush = panel.update(cx, |panel, cx| panel.flush_serialization(cx));
        release.send(()).unwrap();
        cx.run_until_parked();
        assert_eq!(
            db.get_custom_title(12347, workspace_id).unwrap().as_deref(),
            Some("orphan")
        );
        assert_eq!(
            cx.update(|cx| stored_item_ids(workspace_id, cx)).unwrap(),
            vec![12345]
        );
        assert_eq!(
            db.get_custom_title(12345, workspace_id).unwrap().as_deref(),
            Some("saved")
        );
        assert_eq!(
            db.get_working_directory(12345, workspace_id).unwrap(),
            Some(cwd.clone())
        );
        cleanup.await.unwrap();
        flush.await.unwrap();
        assert_eq!(db.get_custom_title(12347, workspace_id).unwrap(), None);
        assert_eq!(
            cx.update(|cx| stored_item_ids(workspace_id, cx)).unwrap(),
            vec![restored_id]
        );
        assert_eq!(
            db.get_custom_title(restored_id, workspace_id)
                .unwrap()
                .as_deref(),
            Some("saved")
        );
        assert_eq!(
            db.get_working_directory(restored_id, workspace_id).unwrap(),
            Some(cwd.clone())
        );
        window_handle
            .update(cx, |_, window, cx| {
                drop_tab(terminal.entity_id(), &panel_pane, &other_pane, window, cx);
            })
            .unwrap();
        cx.run_until_parked();
        terminal.update(cx, |terminal, cx| {
            terminal.set_custom_title(Some("moved".to_string()), cx)
        });
        for workspace in [&workspace, &other_workspace] {
            window_handle
                .update(cx, |_, window, cx| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.flush_serialization_checked(window, cx)
                    })
                })
                .unwrap()
                .await
                .unwrap();
        }
        assert_eq!(
            cx.update(|cx| item_persistence_id(&terminal, cx))
                .await
                .unwrap(),
            restored_id
        );
        assert_eq!(
            cx.update(|cx| stored_item_ids(workspace_id, cx)).unwrap(),
            Vec::<ItemId>::new()
        );
        assert_eq!(
            cx.update(|cx| stored_item_ids(other_workspace_id, cx))
                .unwrap(),
            vec![restored_id]
        );
        assert_eq!(
            db.get_custom_title(restored_id, workspace_id).unwrap(),
            None
        );
        assert_eq!(
            db.get_custom_title(12345, workspace_id).unwrap().as_deref(),
            Some("saved")
        );
        assert_eq!(
            db.get_working_directory(restored_id, other_workspace_id)
                .unwrap(),
            Some(cwd)
        );
        assert_eq!(
            db.get_custom_title(restored_id, other_workspace_id)
                .unwrap()
                .as_deref(),
            Some("moved")
        );
    }

    #[gpui::test]
    async fn test_legacy_serialized_restore_keeps_interim_terminal_active(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    terminal_panel.add_terminal_shell(
                        false,
                        None,
                        RevealStrategy::Never,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await
            .unwrap();
        let interim_item_id = terminal_panel.read_with(cx, |terminal_panel, cx| {
            let pane = terminal_panel.active_pane.read(cx);
            assert_eq!(pane.items_len(), 1);
            pane.active_item().unwrap().item_id()
        });

        let restored_items = window_handle
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                let project = workspace.read(cx).project().clone();
                deserialize_terminal_panel(
                    workspace.downgrade(),
                    project,
                    WorkspaceId::default(),
                    SerializedTerminalPanel {
                        items: SerializedItems::NoSplits(vec![12345]),
                        active_item_id: Some(12345),
                    },
                    terminal_panel.downgrade(),
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();
        cx.run_until_parked();

        assert_eq!(restored_items, 1);
        terminal_panel.read_with(cx, |terminal_panel, cx| {
            let pane = terminal_panel.active_pane.read(cx);
            assert_eq!(pane.items_len(), 2);
            assert_eq!(
                pane.active_item().map(|item| item.item_id()),
                Some(interim_item_id),
                "interim terminal must stay active after a legacy format restore"
            );
        });
    }

    #[gpui::test]
    async fn test_split_restore_grafts_interim_pane(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    terminal_panel.add_terminal_shell(
                        false,
                        None,
                        RevealStrategy::Never,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await
            .unwrap();
        let interim_pane =
            terminal_panel.read_with(cx, |terminal_panel, _| terminal_panel.active_pane.clone());

        let restored_items = window_handle
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                let project = workspace.read(cx).project().clone();
                deserialize_terminal_panel(
                    workspace.downgrade(),
                    project,
                    WorkspaceId::default(),
                    SerializedTerminalPanel {
                        items: SerializedItems::WithSplits(SerializedPaneGroup::Pane(
                            SerializedPane {
                                active: true,
                                children: vec![12345, 12346],
                                active_item: Some(12345),
                                pinned_count: 0,
                            },
                        )),
                        active_item_id: None,
                    },
                    terminal_panel.downgrade(),
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();
        cx.run_until_parked();

        assert_eq!(restored_items, 2);
        let active_terminal = terminal_panel.read_with(cx, |panel, cx| {
            panel
                .active_pane
                .read(cx)
                .active_item()
                .unwrap()
                .downcast::<TerminalView>()
                .unwrap()
        });
        assert_ne!(active_terminal.entity_id().as_u64(), 12345);
        let active_id = cx
            .update(|cx| item_persistence_id(&active_terminal, cx))
            .await
            .unwrap();
        assert_ne!(active_id, 12345);
        let restored_ids = cx.update(|cx| {
            let terminals = terminal_panel
                .read(cx)
                .active_pane
                .read(cx)
                .items()
                .map(|item| item.downcast::<TerminalView>().unwrap())
                .collect::<Vec<_>>();
            terminals
                .iter()
                .map(|terminal| item_persistence_id(terminal, cx))
                .collect::<Vec<_>>()
        });
        let restored_ids = futures::future::try_join_all(restored_ids).await.unwrap();
        assert_eq!(restored_ids.len(), 2);
        assert_ne!(restored_ids[0], 12345);
        assert_ne!(restored_ids[1], 12346);
        assert_ne!(restored_ids[0], restored_ids[1]);
        assert_eq!(active_id, restored_ids[0]);
        let serialized = terminal_panel
            .update(cx, |panel, cx| {
                serialize_pane_group(&panel.center, &panel.active_pane, cx)
            })
            .await
            .unwrap();
        let SerializedPaneGroup::Group { children, .. } = serialized else {
            panic!("missing split")
        };
        let Some(SerializedPaneGroup::Pane(restored_pane)) = children.first() else {
            panic!("missing restored pane")
        };
        assert_eq!(restored_pane.children, restored_ids);
        assert_eq!(restored_pane.active_item, Some(active_id));
        terminal_panel.read_with(cx, |terminal_panel, cx| {
            let panes = terminal_panel.center.panes();
            assert_eq!(
                panes.len(),
                2,
                "both the restored pane and the interim pane must survive"
            );
            let interim_panes_kept = panes.iter().filter(|pane| **pane == &interim_pane).count();
            assert_eq!(
                interim_panes_kept, 1,
                "interim pane must be grafted into the restored center"
            );
            assert_eq!(interim_pane.read(cx).items_len(), 1);
            assert_ne!(
                terminal_panel.active_pane, interim_pane,
                "unfocused interim pane must not become the active pane"
            );
        });
        let flush = terminal_panel.update(cx, |panel, cx| {
            let flush = panel.flush_serialization(cx);
            panel.move_pane_to_border(SplitDirection::Right, cx);
            assert_eq!(panel.center.first_pane(), interim_pane);
            flush
        });
        assert_eq!(
            flush.await.unwrap_err().to_string(),
            "Terminal panel changed during serialization flush"
        );
    }

    #[gpui::test]
    async fn test_local_terminal_in_local_project(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| TerminalPanel::new(workspace, window, cx))
                })
            })
            .unwrap();

        let result = window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    terminal_panel.add_terminal_shell(
                        true,
                        None,
                        RevealStrategy::Always,
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await;

        assert!(
            result.is_ok(),
            "local terminal should successfully create in local project"
        );
    }

    struct FocusOnlyModal {
        focus_handle: gpui::FocusHandle,
    }
    impl gpui::EventEmitter<gpui::DismissEvent> for FocusOnlyModal {}
    impl gpui::Focusable for FocusOnlyModal {
        fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
            self.focus_handle.clone()
        }
    }
    impl Render for FocusOnlyModal {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::div().track_focus(&self.focus_handle)
        }
    }
    impl workspace::ModalView for FocusOnlyModal {}

    async fn open_center_display_terminal(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) {
        workspace
            .update_in(cx, |workspace, window, cx| {
                TerminalPanel::add_center_terminal(workspace, window, cx, |_, cx| {
                    let terminal = cx.new(|cx| {
                        terminal::TerminalBuilder::new_display_only(
                            terminal::terminal_settings::CursorShape::default(),
                            terminal::terminal_settings::AlternateScroll::On,
                            None,
                            0,
                            cx.background_executor(),
                            util::paths::PathStyle::local(),
                        )
                        .subscribe(cx)
                    });
                    gpui::Task::ready(Ok(terminal))
                })
            })
            .await
            .unwrap();
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_center_terminal_keeps_focus_on_active_modal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        let modal_focus_handle = workspace.update_in(cx, |workspace, window, cx| {
            let focus_handle = cx.focus_handle();
            workspace.toggle_modal(window, cx, {
                let focus_handle = focus_handle.clone();
                move |_, _| FocusOnlyModal { focus_handle }
            });
            focus_handle
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.has_active_modal(window, cx));
            assert!(
                modal_focus_handle.is_focused(window),
                "the modal should hold focus before the terminal is created"
            );
        });

        open_center_display_terminal(&workspace, cx).await;

        workspace.update_in(cx, |_, window, _| {
            assert!(
                modal_focus_handle.is_focused(window),
                "a background center terminal must not steal focus from an active modal"
            );
        });
    }

    #[gpui::test]
    async fn test_center_terminal_takes_focus_without_modal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        open_center_display_terminal(&workspace, cx).await;

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(!workspace.has_active_modal(window, cx));
            let terminal_view = workspace
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<TerminalView>())
                .expect("the new center terminal should be the active item");
            assert!(
                terminal_view.focus_handle(cx).contains_focused(window, cx),
                "with no modal open, a new center terminal should take focus"
            );
        });
    }

    #[gpui::test]
    async fn test_panel_terminal_keeps_focus_on_active_modal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .expect("Failed to read workspace");
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        let modal_focus_handle = workspace.update_in(cx, |workspace, window, cx| {
            let focus_handle = cx.focus_handle();
            workspace.toggle_modal(window, cx, {
                let focus_handle = focus_handle.clone();
                move |_, _| FocusOnlyModal { focus_handle }
            });
            focus_handle
        });

        terminal_panel
            .update_in(cx, |terminal_panel, window, cx| {
                terminal_panel.add_terminal_shell(false, None, RevealStrategy::Always, window, cx)
            })
            .await
            .expect("Failed to spawn a panel terminal");
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(
                workspace.has_active_modal(window, cx),
                "a panel terminal that finishes spawning must not dismiss an active modal"
            );
            assert!(
                modal_focus_handle.is_focused(window),
                "a panel terminal that finishes spawning must not steal focus from an active modal"
            );
        });
        terminal_panel.read_with(cx, |terminal_panel, cx| {
            assert_eq!(
                terminal_panel.active_pane.read(cx).items_len(),
                1,
                "the terminal should still be added to the panel"
            );
        });
    }

    #[gpui::test]
    async fn test_panel_terminal_takes_focus_without_modal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        terminal_panel
            .update_in(cx, |terminal_panel, window, cx| {
                terminal_panel.add_terminal_shell(false, None, RevealStrategy::Always, window, cx)
            })
            .await
            .expect("Failed to spawn a panel terminal");
        cx.run_until_parked();

        terminal_panel.update_in(cx, |terminal_panel, window, cx| {
            let terminal_view = terminal_panel
                .active_pane
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<TerminalView>())
                .expect("the new terminal should be the active panel item");
            assert!(
                terminal_view.focus_handle(cx).contains_focused(window, cx),
                "with no modal open, a new panel terminal should take focus"
            );
        });
    }

    #[gpui::test]
    async fn test_task_terminal_keeps_focus_on_active_modal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .expect("Failed to read workspace");
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        let modal_focus_handle = workspace.update_in(cx, |workspace, window, cx| {
            let focus_handle = cx.focus_handle();
            workspace.toggle_modal(window, cx, {
                let focus_handle = focus_handle.clone();
                move |_, _| FocusOnlyModal { focus_handle }
            });
            focus_handle
        });

        terminal_panel
            .update_in(cx, |terminal_panel, window, cx| {
                terminal_panel.add_terminal_task(echo_task(), RevealStrategy::Always, window, cx)
            })
            .await
            .expect("Failed to spawn a task terminal");
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(
                workspace.has_active_modal(window, cx),
                "a task terminal that finishes spawning must not dismiss an active modal"
            );
            assert!(
                modal_focus_handle.is_focused(window),
                "a task terminal that finishes spawning must not steal focus from an active modal"
            );
        });
        terminal_panel.read_with(cx, |terminal_panel, cx| {
            assert_eq!(
                terminal_panel.active_pane.read(cx).items_len(),
                1,
                "the task terminal should still be added to the panel"
            );
        });
    }

    #[gpui::test]
    async fn test_task_terminal_takes_focus_without_modal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        terminal_panel
            .update_in(cx, |terminal_panel, window, cx| {
                terminal_panel.add_terminal_task(echo_task(), RevealStrategy::Always, window, cx)
            })
            .await
            .expect("Failed to spawn a task terminal");
        cx.run_until_parked();

        terminal_panel.update_in(cx, |terminal_panel, window, cx| {
            let terminal_view = terminal_panel
                .active_pane
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<TerminalView>())
                .expect("the new task terminal should be the active panel item");
            assert!(
                terminal_view.focus_handle(cx).contains_focused(window, cx),
                "with no modal open, a new task terminal should take focus"
            );
        });
    }

    #[gpui::test]
    async fn test_finished_restoration_keeps_focus_on_active_modal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .expect("Failed to read workspace");
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        terminal_panel
            .update_in(cx, |terminal_panel, window, cx| {
                terminal_panel.add_terminal_shell(false, None, RevealStrategy::Never, window, cx)
            })
            .await
            .expect("Failed to spawn a panel terminal");
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.open_panel::<TerminalPanel>(window, cx);
        });
        cx.run_until_parked();

        let modal_focus_handle = workspace.update_in(cx, |workspace, window, cx| {
            let focus_handle = cx.focus_handle();
            workspace.toggle_modal(window, cx, {
                let focus_handle = focus_handle.clone();
                move |_, _| FocusOnlyModal { focus_handle }
            });
            focus_handle
        });
        workspace.update_in(cx, |workspace, window, cx| {
            assert!(
                workspace.active_item(cx).is_none()
                    && workspace
                        .is_dock_at_position_open(terminal_panel.read(cx).position(window, cx), cx),
                "the restoration focus conditions should hold, otherwise this test is vacuous"
            );
        });

        window_handle
            .update(cx, |_, window, cx| {
                let workspace = workspace.downgrade();
                let terminal_panel = terminal_panel.downgrade();
                window.spawn(cx, async move |cx| {
                    TerminalPanel::restore_serialized_state(
                        workspace,
                        terminal_panel,
                        Task::ready(Ok(0)),
                        cx,
                    )
                    .await
                })
            })
            .expect("Failed to restore serialized state")
            .await
            .expect("Failed to restore serialized state");
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(
                workspace.has_active_modal(window, cx),
                "finishing restoration must not dismiss an active modal"
            );
            assert!(
                modal_focus_handle.is_focused(window),
                "finishing restoration must not steal focus from an active modal"
            );
        });
    }

    #[gpui::test]
    async fn test_inline_assist_tooltip_shows_keybinding_of_active_terminal(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);

        cx.update(|cx| {
            cx.bind_keys([gpui::KeyBinding::new(
                "ctrl-enter",
                InlineAssist::default(),
                Some("Terminal"),
            )])
        });

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        terminal_panel.update(cx, |panel, cx| panel.set_assistant_enabled(true, cx));
        terminal_panel
            .update_in(cx, |panel, window, cx| {
                panel.add_terminal_shell(false, None, RevealStrategy::Always, window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();

        let button_bounds = cx
            .debug_bounds("ICON-ZedAssistant")
            .expect("inline assist button should be rendered in the terminal tab bar");
        cx.simulate_mouse_move(button_bounds.center(), None, Modifiers::default());

        cx.executor().advance_clock(Duration::from_millis(600));
        cx.run_until_parked();

        assert!(
            cx.debug_bounds("KEY_BINDING-enter").is_some(),
            "tooltip should show the InlineAssist keybinding resolved in the terminal's context"
        );
    }

    // On Windows `echo` is a shell builtin rather than an executable, so spawning it directly fails.
    fn echo_task() -> SpawnInTerminal {
        let (command, args) = if cfg!(windows) {
            ("cmd.exe", vec!["/C".to_owned(), "echo".to_owned()])
        } else {
            ("echo", Vec::new())
        };
        SpawnInTerminal {
            command: Some(command.to_owned()),
            args,
            ..SpawnInTerminal::default()
        }
    }

    async fn init_workspace_with_panel(
        cx: &mut TestAppContext,
    ) -> (gpui::WindowHandle<MultiWorkspace>, Entity<TerminalPanel>) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    let panel = cx.new(|cx| TerminalPanel::new(workspace, window, cx));
                    workspace.add_panel(panel.clone(), window, cx);
                    panel
                })
            })
            .expect("Failed to initialize workspace with terminal panel");

        (window_handle, terminal_panel)
    }

    #[gpui::test]
    async fn test_terminal_panel_starts_open_follows_setting(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    assert!(
                        !terminal_panel.starts_open(window, cx),
                        "terminal panel should not start open by default"
                    );
                });
            })
            .expect("Failed to read terminal panel starts_open default");

        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings.terminal.get_or_insert_default().starts_open = Some(true);
            });
        });

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    assert!(
                        terminal_panel.starts_open(window, cx),
                        "terminal panel should start open when configured"
                    );
                });
            })
            .expect("Failed to read configured terminal panel starts_open");
    }

    #[gpui::test]
    async fn test_new_terminal_opens_in_panel_by_default(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;

        let panel_items_before =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());
        let center_items_before = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::new_terminal(
                        workspace,
                        &workspace::NewTerminal::default(),
                        window,
                        cx,
                    );
                })
            })
            .expect("Failed to dispatch new_terminal");

        cx.run_until_parked();

        let panel_items_after =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());
        let center_items_after = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");

        assert_eq!(
            panel_items_after,
            panel_items_before + 1,
            "Terminal should be added to the panel when no center terminal is focused"
        );
        assert_eq!(
            center_items_after, center_items_before,
            "Center pane should not gain a new terminal"
        );
    }

    #[gpui::test]
    async fn test_new_terminal_opens_in_center_when_center_terminal_focused(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::add_center_terminal(workspace, window, cx, |project, cx| {
                        project.create_terminal_shell(None, cx)
                    })
                })
            })
            .expect("Failed to update workspace")
            .await
            .expect("Failed to create center terminal");
        cx.run_until_parked();

        let center_items_before = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");
        assert_eq!(center_items_before, 1, "Center pane should have 1 terminal");

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    let active_item = workspace
                        .active_pane()
                        .read(cx)
                        .active_item()
                        .expect("Center pane should have an active item");
                    let terminal_view = active_item
                        .downcast::<TerminalView>()
                        .expect("Active center item should be a TerminalView");
                    window.focus(&terminal_view.focus_handle(cx), cx);
                })
            })
            .expect("Failed to focus terminal view");
        cx.run_until_parked();

        let panel_items_before =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::new_terminal(
                        workspace,
                        &workspace::NewTerminal::default(),
                        window,
                        cx,
                    );
                })
            })
            .expect("Failed to dispatch new_terminal");
        cx.run_until_parked();

        let center_items_after = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");
        let panel_items_after =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());

        assert_eq!(
            center_items_after,
            center_items_before + 1,
            "New terminal should be added to the center pane"
        );
        assert_eq!(
            panel_items_after, panel_items_before,
            "Terminal panel should not gain a new terminal"
        );
    }

    #[gpui::test]
    async fn test_new_terminal_opens_in_panel_when_panel_focused(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |panel, cx| {
                    panel.add_terminal_shell(false, None, RevealStrategy::Always, window, cx)
                })
            })
            .expect("Failed to update workspace")
            .await
            .expect("Failed to create panel terminal");
        cx.run_until_parked();

        window_handle
            .update(cx, |_, window, cx| {
                window.focus(&terminal_panel.read(cx).focus_handle(cx), cx);
            })
            .expect("Failed to focus terminal panel");
        cx.run_until_parked();

        let panel_items_before =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());

        let center_items_before = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::new_terminal(
                        workspace,
                        &workspace::NewTerminal::default(),
                        window,
                        cx,
                    );
                })
            })
            .expect("Failed to dispatch new_terminal");
        cx.run_until_parked();

        let panel_items_after =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());
        let center_items_after = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");

        assert_eq!(
            panel_items_after,
            panel_items_before + 1,
            "New terminal should be added to the panel when panel is focused"
        );
        assert_eq!(
            center_items_after, center_items_before,
            "Center pane should not gain a new terminal"
        );
    }

    #[gpui::test]
    async fn test_new_local_terminal_opens_in_center_when_center_terminal_focused(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::add_center_terminal(workspace, window, cx, |project, cx| {
                        project.create_terminal_shell(None, cx)
                    })
                })
            })
            .expect("Failed to update workspace")
            .await
            .expect("Failed to create center terminal");
        cx.run_until_parked();

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    let active_item = workspace
                        .active_pane()
                        .read(cx)
                        .active_item()
                        .expect("Center pane should have an active item");
                    let terminal_view = active_item
                        .downcast::<TerminalView>()
                        .expect("Active center item should be a TerminalView");
                    window.focus(&terminal_view.focus_handle(cx), cx);
                })
            })
            .expect("Failed to focus terminal view");
        cx.run_until_parked();

        let center_items_before = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");
        let panel_items_before =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::new_terminal(
                        workspace,
                        &workspace::NewTerminal { local: true },
                        window,
                        cx,
                    );
                })
            })
            .expect("Failed to dispatch new_terminal with local=true");
        cx.run_until_parked();

        let center_items_after = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");
        let panel_items_after =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());

        assert_eq!(
            center_items_after,
            center_items_before + 1,
            "New local terminal should be added to the center pane"
        );
        assert_eq!(
            panel_items_after, panel_items_before,
            "Terminal panel should not gain a new terminal"
        );
    }

    #[gpui::test]
    async fn test_new_terminal_opens_in_panel_when_panel_focused_and_center_has_terminal(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);

        let (window_handle, terminal_panel) = init_workspace_with_panel(cx).await;

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::add_center_terminal(workspace, window, cx, |project, cx| {
                        project.create_terminal_shell(None, cx)
                    })
                })
            })
            .expect("Failed to update workspace")
            .await
            .expect("Failed to create center terminal");
        cx.run_until_parked();

        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |panel, cx| {
                    panel.add_terminal_shell(false, None, RevealStrategy::Always, window, cx)
                })
            })
            .expect("Failed to update workspace")
            .await
            .expect("Failed to create panel terminal");
        cx.run_until_parked();

        window_handle
            .update(cx, |_, window, cx| {
                window.focus(&terminal_panel.read(cx).focus_handle(cx), cx);
            })
            .expect("Failed to focus terminal panel");
        cx.run_until_parked();

        let panel_items_before =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());
        let center_items_before = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");

        window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    TerminalPanel::new_terminal(
                        workspace,
                        &workspace::NewTerminal::default(),
                        window,
                        cx,
                    );
                })
            })
            .expect("Failed to dispatch new_terminal");
        cx.run_until_parked();

        let panel_items_after =
            terminal_panel.read_with(cx, |panel, cx| panel.active_pane.read(cx).items_len());
        let center_items_after = window_handle
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .items_len()
            })
            .expect("Failed to read center pane items");

        assert_eq!(
            panel_items_after,
            panel_items_before + 1,
            "New terminal should go to panel when panel is focused, even if center has a terminal"
        );
        assert_eq!(
            center_items_after, center_items_before,
            "Center pane should not gain a new terminal when panel is focused"
        );
    }

    fn stored_item_ids(workspace_id: WorkspaceId, cx: &App) -> Result<Vec<ItemId>> {
        let key = TerminalPanel::database_serialization_key(workspace_id);
        KeyValueStore::global(cx)
            .read_kvp(&key)?
            .map(|state| {
                serde_json::from_str::<SerializedTerminalPanel>(&state)
                    .map(|panel| panel.into_pane_group().item_ids())
            })
            .transpose()
            .map(|items| items.unwrap_or_default())
            .map_err(anyhow::Error::from)
    }

    fn drop_tab(
        item_id: EntityId,
        source: &Entity<Pane>,
        destination: &Entity<Pane>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let pane = source.read(cx);
        let index = pane
            .items()
            .position(|item| item.item_id() == item_id)
            .unwrap();
        let dragged = DraggedTab {
            pane: source.clone(),
            item: pane.item_for_index(index).unwrap().boxed_clone(),
            ix: index,
            detail: 0,
            is_active: pane
                .active_item()
                .is_some_and(|item| item.item_id() == item_id),
        };
        destination.update(cx, |pane, cx| {
            pane.handle_tab_drop(&dragged, pane.items_len(), false, window, cx);
        });
    }

    fn set_max_tabs(cx: &mut TestAppContext, value: Option<usize>) {
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings.workspace.max_tabs = value.map(|v| NonZero::new(v).unwrap())
            });
        });
    }

    pub fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }
}

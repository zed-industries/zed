use std::{cmp, ops::ControlFlow, path::PathBuf, sync::Arc, time::Duration};

use crate::{
    TerminalView, default_working_directory,
    persistence::{
        SerializedItems, SerializedTerminalPanel, deserialize_terminal_panel, serialize_pane_group,
    },
};
use breadcrumbs::Breadcrumbs;
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use futures::future::join_all;
use gpui::{
    Action, AnyView, App, AsyncApp, AsyncWindowContext, Context, Corner, Entity, EventEmitter,
    ExternalPaths, FocusHandle, Focusable, IntoElement, ParentElement, Pixels, Render, Styled,
    Task, WeakEntity, Window, actions,
};
use itertools::Itertools;
use project::{Fs, Project, ProjectEntryId, terminals::TerminalKind};
use search::{BufferSearchBar, buffer_search::DivRegistrar};
use settings::Settings;
use task::{RevealStrategy, RevealTarget, ShellBuilder, SpawnInTerminal, TaskId};
use terminal::{
    Terminal,
    terminal_settings::{TerminalDockPosition, TerminalSettings},
};
use ui::{
    ButtonCommon, Clickable, ContextMenu, FluentBuilder, PopoverMenu, Toggleable, Tooltip,
    prelude::*,
};
use util::{ResultExt, TryFutureExt};
use workspace::{
    ActivateNextPane, ActivatePane, ActivatePaneDown, ActivatePaneLeft, ActivatePaneRight,
    ActivatePaneUp, ActivatePreviousPane, DraggedSelection, DraggedTab, ItemId, MoveItemToPane,
    MoveItemToPaneInDirection, NewTerminal, Pane, PaneGroup, SplitDirection, SplitDown, SplitLeft,
    SplitRight, SplitUp, SwapPaneDown, SwapPaneLeft, SwapPaneRight, SwapPaneUp, ToggleZoom,
    Workspace,
    dock::{DockPosition, Panel, PanelEvent, PanelHandle},
    item::SerializableItem,
    move_active_item, move_item, pane,
    ui::IconName,
};

use anyhow::{Context as _, Result, anyhow};
use zed_actions::assistant::InlineAssist;

const TERMINAL_PANEL_KEY: &str = "TerminalPanel";

actions!(terminal_panel, [ToggleFocus]);

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
        },
    )
    .detach();
}

pub struct TerminalPanel {
    pub(crate) active_pane: Entity<Pane>,
    pub(crate) center: PaneGroup,
    fs: Arc<dyn Fs>,
    workspace: WeakEntity<Workspace>,
    pub(crate) width: Option<Pixels>,
    pub(crate) height: Option<Pixels>,
    pending_serialization: Task<Option<()>>,
    pending_terminals_to_add: usize,
    deferred_tasks: HashMap<TaskId, Task<()>>,
    assistant_enabled: bool,
    assistant_tab_bar_button: Option<AnyView>,
    active: bool,
}

impl TerminalPanel {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let project = workspace.project();
        let pane = new_terminal_pane(workspace.weak_handle(), project.clone(), false, window, cx);
        let center = PaneGroup::new(pane.clone());
        let terminal_panel = Self {
            center,
            active_pane: pane,
            fs: workspace.app_state().fs.clone(),
            workspace: workspace.weak_handle(),
            pending_serialization: Task::ready(None),
            width: None,
            height: None,
            pending_terminals_to_add: 0,
            deferred_tasks: HashMap::default(),
            assistant_enabled: false,
            assistant_tab_bar_button: None,
            active: false,
        };
        terminal_panel.apply_tab_bar_buttons(&terminal_panel.active_pane, cx);
        terminal_panel
    }

    pub fn set_assistant_enabled(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.assistant_enabled = enabled;
        if enabled {
            let focus_handle = self
                .active_pane
                .read(cx)
                .active_item()
                .map(|item| item.item_focus_handle(cx))
                .unwrap_or(self.focus_handle(cx));
            self.assistant_tab_bar_button = Some(
                cx.new(move |_| InlineAssistTabBarButton { focus_handle })
                    .into(),
            );
        } else {
            self.assistant_tab_bar_button = None;
        }
        for pane in self.center.panes() {
            self.apply_tab_bar_buttons(pane, cx);
        }
    }

    fn apply_tab_bar_buttons(&self, terminal_pane: &Entity<Pane>, cx: &mut Context<Self>) {
        let assistant_tab_bar_button = self.assistant_tab_bar_button.clone();
        terminal_pane.update(cx, |pane, cx| {
            pane.set_render_tab_bar_buttons(cx, move |pane, window, cx| {
                let split_context = pane
                    .active_item()
                    .and_then(|item| item.downcast::<TerminalView>())
                    .map(|terminal_view| terminal_view.read(cx).focus_handle.clone());
                if !pane.has_focus(window, cx) && !pane.context_menu_focused(window, cx) {
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
                            .anchor(Corner::TopRight)
                            .with_handle(pane.new_item_context_menu_handle.clone())
                            .menu(move |window, cx| {
                                let focus_handle = focus_handle.clone();
                                let menu = ContextMenu::build(window, cx, |menu, _, _| {
                                    menu.context(focus_handle.clone())
                                        .action(
                                            "New Terminal",
                                            workspace::NewTerminal.boxed_clone(),
                                        )
                                        // We want the focus to go back to terminal panel once task modal is dismissed,
                                        // hence we focus that first. Otherwise, we'd end up without a focused element, as
                                        // context menu will be gone the moment we spawn the modal.
                                        .action(
                                            "Spawn task",
                                            zed_actions::Spawn::modal().boxed_clone(),
                                        )
                                });

                                Some(menu)
                            }),
                    )
                    .children(assistant_tab_bar_button.clone())
                    .child(
                        PopoverMenu::new("terminal-pane-tab-bar-split")
                            .trigger_with_tooltip(
                                IconButton::new("terminal-pane-split", IconName::Split)
                                    .icon_size(IconSize::Small),
                                Tooltip::text("Split Pane"),
                            )
                            .anchor(Corner::TopRight)
                            .with_handle(pane.split_item_context_menu_handle.clone())
                            .menu({
                                let split_context = split_context.clone();
                                move |window, cx| {
                                    ContextMenu::build(window, cx, |menu, _, _| {
                                        menu.when_some(
                                            split_context.clone(),
                                            |menu, split_context| menu.context(split_context),
                                        )
                                        .action("Split Right", SplitRight.boxed_clone())
                                        .action("Split Left", SplitLeft.boxed_clone())
                                        .action("Split Up", SplitUp.boxed_clone())
                                        .action("Split Down", SplitDown.boxed_clone())
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
                            .tooltip(move |window, cx| {
                                Tooltip::for_action(
                                    if zoomed { "Zoom Out" } else { "Zoom In" },
                                    &ToggleZoom,
                                    window,
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
        let mut terminal_panel = None;

        match workspace
            .read_with(&cx, |workspace, _| {
                workspace
                    .database_id()
                    .zip(TerminalPanel::serialization_key(workspace))
            })
            .ok()
            .flatten()
        {
            Some((database_id, serialization_key)) => {
                if let Some(serialized_panel) = cx
                    .background_spawn(async move { KEY_VALUE_STORE.read_kvp(&serialization_key) })
                    .await
                    .log_err()
                    .flatten()
                    .map(|panel| serde_json::from_str::<SerializedTerminalPanel>(&panel))
                    .transpose()
                    .log_err()
                    .flatten()
                {
                    if let Ok(serialized) = workspace
                        .update_in(&mut cx, |workspace, window, cx| {
                            deserialize_terminal_panel(
                                workspace.weak_handle(),
                                workspace.project().clone(),
                                database_id,
                                serialized_panel,
                                window,
                                cx,
                            )
                        })?
                        .await
                    {
                        terminal_panel = Some(serialized);
                    }
                }
            }
            _ => {}
        }

        let terminal_panel = if let Some(panel) = terminal_panel {
            panel
        } else {
            workspace.update_in(&mut cx, |workspace, window, cx| {
                cx.new(|cx| TerminalPanel::new(workspace, window, cx))
            })?
        };

        if let Some(workspace) = workspace.upgrade() {
            terminal_panel
                .update_in(&mut cx, |_, window, cx| {
                    cx.subscribe_in(&workspace, window, |terminal_panel, _, e, window, cx| {
                        if let workspace::Event::SpawnTask {
                            action: spawn_in_terminal,
                        } = e
                        {
                            terminal_panel.spawn_task(spawn_in_terminal, window, cx);
                        };
                    })
                    .detach();
                })
                .ok();
        }

        // Since panels/docks are loaded outside from the workspace, we cleanup here, instead of through the workspace.
        if let Some(workspace) = workspace.upgrade() {
            let cleanup_task = workspace.update_in(&mut cx, |workspace, window, cx| {
                let alive_item_ids = terminal_panel
                    .read(cx)
                    .center
                    .panes()
                    .into_iter()
                    .flat_map(|pane| pane.read(cx).items())
                    .map(|item| item.item_id().as_u64() as ItemId)
                    .collect();
                workspace.database_id().map(|workspace_id| {
                    TerminalView::cleanup(workspace_id, alive_item_ids, window, cx)
                })
            })?;
            if let Some(task) = cleanup_task {
                task.await.log_err();
            }
        }

        if let Some(workspace) = workspace.upgrade() {
            let should_focus = workspace
                .update_in(&mut cx, |workspace, window, cx| {
                    workspace.active_item(cx).is_none()
                        && workspace
                            .is_dock_at_position_open(terminal_panel.position(window, cx), cx)
                })
                .unwrap_or(false);

            if should_focus {
                terminal_panel
                    .update_in(&mut cx, |panel, window, cx| {
                        panel.active_pane.update(cx, |pane, cx| {
                            pane.focus_active_item(window, cx);
                        });
                    })
                    .ok();
            }
        }

        Ok(terminal_panel)
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
                let _removal_result = self.center.remove(&pane);
                if pane_count_before_removal == 1 {
                    self.center.first_pane().update(cx, |pane, cx| {
                        pane.set_zoomed(false, cx);
                    });
                    cx.emit(PanelEvent::Close);
                } else {
                    if let Some(focus_on_pane) =
                        focus_on_pane.as_ref().or_else(|| self.center.panes().pop())
                    {
                        focus_on_pane.focus_handle(cx).focus(window);
                    }
                }
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
            pane::Event::AddItem { item } => {
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, pane.clone(), window, cx)
                    })
                }
                self.serialize(cx);
            }
            pane::Event::Split(direction) => {
                let Some(new_pane) = self.new_pane_with_cloned_active_terminal(window, cx) else {
                    return;
                };
                let pane = pane.clone();
                let direction = *direction;
                self.center.split(&pane, &new_pane, direction).log_err();
                window.focus(&new_pane.focus_handle(cx));
            }
            pane::Event::Focus => {
                self.active_pane = pane.clone();
            }

            _ => {}
        }
    }

    fn new_pane_with_cloned_active_terminal(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Pane>> {
        let workspace = self.workspace.upgrade()?;
        let workspace = workspace.read(cx);
        let database_id = workspace.database_id();
        let weak_workspace = self.workspace.clone();
        let project = workspace.project().clone();
        let (working_directory, python_venv_directory) = self
            .active_pane
            .read(cx)
            .active_item()
            .and_then(|item| item.downcast::<TerminalView>())
            .map(|terminal_view| {
                let terminal = terminal_view.read(cx).terminal().read(cx);
                (
                    terminal
                        .working_directory()
                        .or_else(|| default_working_directory(workspace, cx)),
                    terminal.python_venv_directory.clone(),
                )
            })
            .unwrap_or((None, None));
        let kind = TerminalKind::Shell(working_directory);
        let window_handle = window.window_handle();
        let terminal = project
            .update(cx, |project, cx| {
                project.create_terminal_with_venv(kind, python_venv_directory, window_handle, cx)
            })
            .ok()?;

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
        let pane = new_terminal_pane(
            weak_workspace,
            project,
            self.active_pane.read(cx).is_zoomed(),
            window,
            cx,
        );
        self.apply_tab_bar_buttons(&pane, cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(terminal_view, true, true, None, window, cx);
        });

        Some(pane)
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
                panel.add_terminal(
                    TerminalKind::Shell(Some(action.working_directory.clone())),
                    RevealStrategy::Always,
                    window,
                    cx,
                )
            })
            .detach_and_log_err(cx);
    }

    fn spawn_task(&mut self, task: &SpawnInTerminal, window: &mut Window, cx: &mut Context<Self>) {
        let Ok(is_local) = self
            .workspace
            .update(cx, |workspace, cx| workspace.project().read(cx).is_local())
        else {
            return;
        };

        let builder = ShellBuilder::new(is_local, &task.shell);
        let command_label = builder.command_label(&task.command_label);
        let (command, args) = builder.build(task.command.clone(), &task.args);

        let task = SpawnInTerminal {
            command_label,
            command,
            args,
            ..task.clone()
        };

        if task.allow_concurrent_runs && task.use_new_terminal {
            self.spawn_in_new_terminal(task, window, cx)
                .detach_and_log_err(cx);
            return;
        }

        let mut terminals_for_task = self.terminals_for_task(&task.full_label, cx);
        let Some(existing) = terminals_for_task.pop() else {
            self.spawn_in_new_terminal(task, window, cx)
                .detach_and_log_err(cx);
            return;
        };

        let (existing_item_index, task_pane, existing_terminal) = existing;
        if task.allow_concurrent_runs {
            self.replace_terminal(
                task,
                task_pane,
                existing_item_index,
                existing_terminal,
                window,
                cx,
            )
            .detach();
            return;
        }

        self.deferred_tasks.insert(
            task.id.clone(),
            cx.spawn_in(window, async move |terminal_panel, cx| {
                wait_for_terminals_tasks(terminals_for_task, cx).await;
                let task = terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                    if task.use_new_terminal {
                        terminal_panel
                            .spawn_in_new_terminal(task, window, cx)
                            .detach_and_log_err(cx);
                        None
                    } else {
                        Some(terminal_panel.replace_terminal(
                            task,
                            task_pane,
                            existing_item_index,
                            existing_terminal,
                            window,
                            cx,
                        ))
                    }
                });
                if let Ok(Some(task)) = task {
                    task.await;
                }
            }),
        );
    }

    pub fn spawn_in_new_terminal(
        &mut self,
        spawn_task: SpawnInTerminal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        let reveal = spawn_task.reveal;
        let reveal_target = spawn_task.reveal_target;
        let kind = TerminalKind::Task(spawn_task);
        match reveal_target {
            RevealTarget::Center => self
                .workspace
                .update(cx, |workspace, cx| {
                    Self::add_center_terminal(workspace, kind, window, cx)
                })
                .unwrap_or_else(|e| Task::ready(Err(e))),
            RevealTarget::Dock => self.add_terminal(kind, reveal, window, cx),
        }
    }

    /// Create a new Terminal in the current working directory or the user's home directory
    fn new_terminal(
        workspace: &mut Workspace,
        _: &workspace::NewTerminal,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(terminal_panel) = workspace.panel::<Self>(cx) else {
            return;
        };

        let kind = TerminalKind::Shell(default_working_directory(workspace, cx));

        terminal_panel
            .update(cx, |this, cx| {
                this.add_terminal(kind, RevealStrategy::Always, window, cx)
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
                    if &task_state.full_label == label {
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
                    .into_iter()
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
        kind: TerminalKind,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<Entity<Terminal>>> {
        if !is_enabled_in_workspace(workspace, cx) {
            return Task::ready(Err(anyhow!(
                "terminal not yet supported for remote projects"
            )));
        }
        let window_handle = window.window_handle();
        let project = workspace.project().downgrade();
        cx.spawn_in(window, async move |workspace, cx| {
            let terminal = project
                .update(cx, |project, cx| {
                    project.create_terminal(kind, window_handle, cx)
                })?
                .await?;

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
                workspace.add_item_to_active_pane(Box::new(terminal_view), None, true, window, cx);
            })?;
            Ok(terminal)
        })
    }

    pub fn add_terminal(
        &mut self,
        kind: TerminalKind,
        reveal_strategy: RevealStrategy,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |terminal_panel, cx| {
            if workspace.update(cx, |workspace, cx| !is_enabled_in_workspace(workspace, cx))? {
                anyhow::bail!("terminal not yet supported for remote projects");
            }
            let pane = terminal_panel.update(cx, |terminal_panel, _| {
                terminal_panel.pending_terminals_to_add += 1;
                terminal_panel.active_pane.clone()
            })?;
            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;
            let window_handle = cx.window_handle();
            let terminal = project
                .update(cx, |project, cx| {
                    project.create_terminal(kind, window_handle, cx)
                })?
                .await?;
            let result = workspace.update_in(cx, |workspace, window, cx| {
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

                match reveal_strategy {
                    RevealStrategy::Always => {
                        workspace.focus_panel::<Self>(window, cx);
                    }
                    RevealStrategy::NoFocus => {
                        workspace.open_panel::<Self>(window, cx);
                    }
                    RevealStrategy::Never => {}
                }

                pane.update(cx, |pane, cx| {
                    let focus = pane.has_focus(window, cx)
                        || matches!(reveal_strategy, RevealStrategy::Always);
                    pane.add_item(terminal_view, true, focus, None, window, cx);
                });

                Ok(terminal)
            })?;
            terminal_panel.update(cx, |this, cx| {
                this.pending_terminals_to_add = this.pending_terminals_to_add.saturating_sub(1);
                this.serialize(cx)
            })?;
            result
        })
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let height = self.height;
        let width = self.width;
        let Some(serialization_key) = self
            .workspace
            .update(cx, |workspace, _| {
                TerminalPanel::serialization_key(workspace)
            })
            .ok()
            .flatten()
        else {
            return;
        };
        self.pending_serialization = cx.spawn(async move |terminal_panel, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            let terminal_panel = terminal_panel.upgrade()?;
            let items = terminal_panel
                .update(cx, |terminal_panel, cx| {
                    SerializedItems::WithSplits(serialize_pane_group(
                        &terminal_panel.center,
                        &terminal_panel.active_pane,
                        cx,
                    ))
                })
                .ok()?;
            cx.background_spawn(
                async move {
                    KEY_VALUE_STORE
                        .write_kvp(
                            serialization_key,
                            serde_json::to_string(&SerializedTerminalPanel {
                                items,
                                active_item_id: None,
                                height,
                                width,
                            })?,
                        )
                        .await?;
                    anyhow::Ok(())
                }
                .log_err(),
            )
            .await;
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
    ) -> Task<Option<()>> {
        let reveal = spawn_task.reveal;
        let reveal_target = spawn_task.reveal_target;
        let window_handle = window.window_handle();
        let task_workspace = self.workspace.clone();
        cx.spawn_in(window, async move |terminal_panel, cx| {
            let project = terminal_panel
                .update(cx, |this, cx| {
                    this.workspace
                        .update(cx, |workspace, _| workspace.project().clone())
                        .ok()
                })
                .ok()
                .flatten()?;
            let new_terminal = project
                .update(cx, |project, cx| {
                    project.create_terminal(TerminalKind::Task(spawn_task), window_handle, cx)
                })
                .ok()?
                .await
                .log_err()?;
            terminal_to_replace
                .update_in(cx, |terminal_to_replace, window, cx| {
                    terminal_to_replace.set_terminal(new_terminal, window, cx);
                })
                .ok()?;

            match reveal {
                RevealStrategy::Always => match reveal_target {
                    RevealTarget::Center => {
                        task_workspace
                            .update_in(cx, |workspace, window, cx| {
                                workspace
                                    .active_item(cx)
                                    .context("retrieving active terminal item in the workspace")
                                    .log_err()?
                                    .item_focus_handle(cx)
                                    .focus(window);
                                Some(())
                            })
                            .ok()??;
                    }
                    RevealTarget::Dock => {
                        terminal_panel
                            .update_in(cx, |terminal_panel, window, cx| {
                                terminal_panel.activate_terminal_view(
                                    &task_pane,
                                    terminal_item_index,
                                    true,
                                    window,
                                    cx,
                                )
                            })
                            .ok()?;

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
                        task_workspace
                            .update_in(cx, |workspace, window, cx| {
                                workspace.active_pane().focus_handle(cx).focus(window);
                            })
                            .ok()?;
                    }
                    RevealTarget::Dock => {
                        terminal_panel
                            .update_in(cx, |terminal_panel, window, cx| {
                                terminal_panel.activate_terminal_view(
                                    &task_pane,
                                    terminal_item_index,
                                    false,
                                    window,
                                    cx,
                                )
                            })
                            .ok()?;

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

            Some(())
        })
    }

    fn has_no_terminals(&self, cx: &App) -> bool {
        self.active_pane.read(cx).items_len() == 0 && self.pending_terminals_to_add == 0
    }

    pub fn assistant_enabled(&self) -> bool {
        self.assistant_enabled
    }

    fn is_enabled(&self, cx: &App) -> bool {
        self.workspace.upgrade().map_or(false, |workspace| {
            is_enabled_in_workspace(workspace.read(cx), cx)
        })
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
            window.focus(&pane.focus_handle(cx));
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
            self.center.swap(&self.active_pane, &to);
            cx.notify();
        }
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
    let is_local = project.read(cx).is_local();
    let terminal_panel = cx.entity().clone();
    let pane = cx.new(|cx| {
        let mut pane = Pane::new(
            workspace.clone(),
            project.clone(),
            Default::default(),
            None,
            NewTerminal.boxed_clone(),
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
                    .update(cx, |terminal_panel, _| {
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
                        return item.downcast::<TerminalView>().is_some();
                    }
                }
            }
            false
        })));

        let buffer_search_bar = cx.new(|cx| {
            search::BufferSearchBar::new(Some(project.read(cx).languages().clone()), window, cx)
        });
        let breadcrumbs = cx.new(|_| Breadcrumbs::new());
        pane.toolbar().update(cx, |toolbar, cx| {
            toolbar.add_item(buffer_search_bar, window, cx);
            toolbar.add_item(breadcrumbs, window, cx);
        });

        let drop_closure_project = project.downgrade();
        let drop_closure_terminal_panel = terminal_panel.downgrade();
        pane.set_custom_drop_handle(cx, move |pane, dropped_item, window, cx| {
            let Some(project) = drop_closure_project.upgrade() else {
                return ControlFlow::Break(());
            };
            if let Some(tab) = dropped_item.downcast_ref::<DraggedTab>() {
                let this_pane = cx.entity().clone();
                let item = if tab.pane == this_pane {
                    pane.item_for_index(tab.ix)
                } else {
                    tab.pane.read(cx).item_for_index(tab.ix)
                };
                if let Some(item) = item {
                    if item.downcast::<TerminalView>().is_some() {
                        let source = tab.pane.clone();
                        let item_id_to_move = item.item_id();

                        let Ok(new_split_pane) = pane
                            .drag_split_direction()
                            .map(|split_direction| {
                                drop_closure_terminal_panel.update(cx, |terminal_panel, cx| {
                                    let is_zoomed = if terminal_panel.active_pane == this_pane {
                                        pane.is_zoomed()
                                    } else {
                                        terminal_panel.active_pane.read(cx).is_zoomed()
                                    };
                                    let new_pane = new_terminal_pane(
                                        workspace.clone(),
                                        project.clone(),
                                        is_zoomed,
                                        window,
                                        cx,
                                    );
                                    terminal_panel.apply_tab_bar_buttons(&new_pane, cx);
                                    terminal_panel.center.split(
                                        &this_pane,
                                        &new_pane,
                                        split_direction,
                                    )?;
                                    anyhow::Ok(new_pane)
                                })
                            })
                            .transpose()
                        else {
                            return ControlFlow::Break(());
                        };

                        match new_split_pane.transpose() {
                            // Source pane may be the one currently updated, so defer the move.
                            Ok(Some(new_pane)) => cx
                                .spawn_in(window, async move |_, cx| {
                                    cx.update(|window, cx| {
                                        move_item(
                                            &source,
                                            &new_pane,
                                            item_id_to_move,
                                            new_pane.read(cx).active_item_index(),
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                                })
                                .detach(),
                            // If we drop into existing pane or current pane,
                            // regular pane drop handler will take care of it,
                            // using the right tab index for the operation.
                            Ok(None) => return ControlFlow::Continue(()),
                            err @ Err(_) => {
                                err.log_err();
                                return ControlFlow::Break(());
                            }
                        };
                    } else if let Some(project_path) = item.project_path(cx) {
                        if let Some(entry_path) = project.read(cx).absolute_path(&project_path, cx)
                        {
                            add_paths_to_terminal(pane, &[entry_path], window, cx);
                        }
                    }
                }
            } else if let Some(selection) = dropped_item.downcast_ref::<DraggedSelection>() {
                let project = project.read(cx);
                let paths_to_add = selection
                    .items()
                    .map(|selected_entry| selected_entry.entry_id)
                    .filter_map(|entry_id| project.path_for_entry(entry_id, cx))
                    .filter_map(|project_path| project.absolute_path(&project_path, cx))
                    .collect::<Vec<_>>();
                if !paths_to_add.is_empty() {
                    add_paths_to_terminal(pane, &paths_to_add, window, cx);
                }
            } else if let Some(&entry_id) = dropped_item.downcast_ref::<ProjectEntryId>() {
                if let Some(entry_path) = project
                    .read(cx)
                    .path_for_entry(entry_id, cx)
                    .and_then(|project_path| project.read(cx).absolute_path(&project_path, cx))
                {
                    add_paths_to_terminal(pane, &[entry_path], window, cx);
                }
            } else if is_local {
                if let Some(paths) = dropped_item.downcast_ref::<ExternalPaths>() {
                    add_paths_to_terminal(pane, paths.paths(), window, cx);
                }
            }

            ControlFlow::Break(())
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
    let pending_tasks = terminals_for_task.iter().filter_map(|(_, _, terminal)| {
        terminal
            .update(cx, |terminal_view, cx| {
                terminal_view
                    .terminal()
                    .update(cx, |terminal, cx| terminal.wait_for_completed_task(cx))
            })
            .ok()
    });
    let _: Vec<()> = join_all(pending_tasks).await;
}

fn add_paths_to_terminal(
    pane: &mut Pane,
    paths: &[PathBuf],
    window: &mut Window,
    cx: &mut Context<Pane>,
) {
    if let Some(terminal_view) = pane
        .active_item()
        .and_then(|item| item.downcast::<TerminalView>())
    {
        window.focus(&terminal_view.focus_handle(cx));
        let mut new_text = paths.iter().map(|path| format!(" {path:?}")).join("");
        new_text.push(' ');
        terminal_view.update(cx, |terminal_view, cx| {
            terminal_view.terminal().update(cx, |terminal, _| {
                terminal.paste(&new_text);
            });
        });
    }
}

impl EventEmitter<PanelEvent> for TerminalPanel {}

impl Render for TerminalPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut registrar = DivRegistrar::new(
            |panel, _, cx| {
                panel
                    .active_pane
                    .read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<BufferSearchBar>()
            },
            cx,
        );
        BufferSearchBar::register(&mut registrar);
        let registrar = registrar.into_div();
        self.workspace
            .update(cx, |workspace, cx| {
                registrar.size_full().child(self.center.render(
                    workspace.zoomed_item(),
                    &workspace::PaneRenderContext {
                        follower_states: &&HashMap::default(),
                        active_call: workspace.active_call(),
                        active_pane: &self.active_pane,
                        app_state: &workspace.app_state(),
                        project: workspace.project(),
                        workspace: &workspace.weak_handle(),
                    },
                    window,
                    cx,
                ))
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
                            window.focus(&panes[next_ix].focus_handle(cx));
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
                            window.focus(&panes[prev_ix].focus_handle(cx));
                        }
                    },
                ))
                .on_action(
                    cx.listener(|terminal_panel, action: &ActivatePane, window, cx| {
                        let panes = terminal_panel.center.panes();
                        if let Some(&pane) = panes.get(action.0) {
                            window.focus(&pane.read(cx).focus_handle(cx));
                        } else {
                            if let Some(new_pane) =
                                terminal_panel.new_pane_with_cloned_active_terminal(window, cx)
                            {
                                terminal_panel
                                    .center
                                    .split(
                                        &terminal_panel.active_pane,
                                        &new_pane,
                                        SplitDirection::Right,
                                    )
                                    .log_err();
                                window.focus(&new_pane.focus_handle(cx));
                            }
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
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane.focus_handle(cx)
    }
}

impl Panel for TerminalPanel {
    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        match TerminalSettings::get_global(cx).dock {
            TerminalDockPosition::Left => DockPosition::Left,
            TerminalDockPosition::Bottom => DockPosition::Bottom,
            TerminalDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        settings::update_settings_file::<TerminalSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left => TerminalDockPosition::Left,
                    DockPosition::Bottom => TerminalDockPosition::Bottom,
                    DockPosition::Right => TerminalDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, window: &Window, cx: &App) -> Pixels {
        let settings = TerminalSettings::get_global(cx);
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        cx.notify();
        cx.defer_in(window, |this, _, cx| {
            this.serialize(cx);
        })
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
        if !active || old_active == active || !self.has_no_terminals(cx) {
            return;
        }
        cx.defer_in(window, |this, window, cx| {
            let Ok(kind) = this.workspace.update(cx, |workspace, cx| {
                TerminalKind::Shell(default_working_directory(workspace, cx))
            }) else {
                return;
            };

            this.add_terminal(kind, RevealStrategy::Always, window, cx)
                .detach_and_log_err(cx)
        })
    }

    fn icon_label(&self, _window: &Window, cx: &App) -> Option<String> {
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

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        if (self.is_enabled(cx) || !self.has_no_terminals(cx))
            && TerminalSettings::get_global(cx).button
        {
            Some(IconName::Terminal)
        } else {
            None
        }
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Terminal Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn pane(&self) -> Option<Entity<Pane>> {
        Some(self.active_pane.clone())
    }

    fn activation_priority(&self) -> u32 {
        1
    }
}

struct InlineAssistTabBarButton {
    focus_handle: FocusHandle,
}

impl Render for InlineAssistTabBarButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        IconButton::new("terminal_inline_assistant", IconName::ZedAssistant)
            .icon_size(IconSize::Small)
            .on_click(cx.listener(|_, _, window, cx| {
                window.dispatch_action(InlineAssist::default().boxed_clone(), cx);
            }))
            .tooltip(move |window, cx| {
                Tooltip::for_action_in(
                    "Inline Assist",
                    &InlineAssist::default(),
                    &focus_handle,
                    window,
                    cx,
                )
            })
    }
}

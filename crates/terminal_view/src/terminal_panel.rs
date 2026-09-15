use std::{
    cmp,
    path::PathBuf,
    process::ExitStatus,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::{
    TerminalView, default_working_directory,
    persistence::{
        SerializedItems, SerializedTerminalPanel, TerminalDb, TerminalSerializationAdmission,
        deserialize_terminal_panel, serialize_pane_group,
    },
};
use breadcrumbs::Breadcrumbs;
use collections::{HashMap, HashSet};
use db::kvp::KeyValueStore;
use futures::{
    FutureExt as _,
    channel::oneshot,
    future::{Shared, join_all},
};
use gpui::{
    Action, Anchor, App, AsyncApp, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, ParentElement, Pixels, Render, Styled, Subscription, Task, TaskExt,
    WeakEntity, Window, actions,
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
    SwapPaneRight, SwapPaneUp, ToggleZoom, Workspace, WorkspaceId,
    dock::{DockPosition, Panel, PanelEvent, PanelHandle},
    move_active_item, pane,
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
    pub(crate) primary_item_ids: HashSet<workspace::ItemId>,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    workspace: WeakEntity<Workspace>,
    pending_serialization: Task<Option<()>>,
    pending_publication: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
    known_item_ids: Arc<Mutex<HashSet<workspace::ItemId>>>,
    needs_cleanup: Arc<AtomicBool>,
    pending_terminals_to_add: usize,
    restoring: bool,
    primary_loaded: bool,
    recovery_loaded: bool,
    published_recovery_item_ids: Arc<Mutex<HashSet<workspace::ItemId>>>,
    restoration_error: Option<SharedString>,
    publication_error: Option<SharedString>,
    publication_token: Arc<()>,
    _restoration: Task<()>,
    _quit_subscription: Subscription,
    deferred_tasks: HashMap<TaskId, Task<()>>,
    assistant_enabled: bool,
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
            focus_handle: cx.focus_handle(),
            fs: workspace.app_state().fs.clone(),
            workspace: workspace.weak_handle(),
            pending_serialization: Task::ready(None),
            pending_publication: None,
            known_item_ids: Arc::new(Mutex::new(HashSet::default())),
            needs_cleanup: Arc::new(AtomicBool::new(true)),
            pending_terminals_to_add: 0,
            restoring: false,
            primary_loaded: true,
            primary_item_ids: HashSet::default(),
            recovery_loaded: true,
            published_recovery_item_ids: Arc::new(Mutex::new(HashSet::default())),
            restoration_error: None,
            publication_error: None,
            publication_token: Arc::new(()),
            _restoration: Task::ready(()),
            _quit_subscription: cx.on_app_quit(Self::app_will_quit),
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

    pub(crate) fn serialization_key_for_workspace_id(workspace_id: WorkspaceId) -> String {
        let id = i64::from(workspace_id).to_string();
        format!("{TERMINAL_PANEL_KEY:?}-{id:?}")
    }

    pub(crate) fn recovery_key_for_workspace_id(workspace_id: WorkspaceId) -> String {
        format!(
            "{}-recovery",
            Self::serialization_key_for_workspace_id(workspace_id)
        )
    }

    #[cfg(test)]
    fn serialization_key(workspace: &Workspace) -> Option<String> {
        workspace
            .database_id()
            .map(Self::serialization_key_for_workspace_id)
            .or_else(|| {
                workspace
                    .session_id()
                    .map(|id| format!("{TERMINAL_PANEL_KEY:?}-{id:?}"))
            })
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        let terminal_panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            cx.new(|cx| TerminalPanel::new(workspace, window, cx))
        })?;

        workspace
            .update(&mut cx, |workspace, _| {
                workspace.set_terminal_provider(TerminalProvider(terminal_panel.clone()))
            })
            .ok();

        terminal_panel.update_in(&mut cx, |panel, window, cx| {
            panel.primary_loaded = false;
            panel.recovery_loaded = false;
            panel.retry_restoration(window, cx);
        })?;

        Ok(terminal_panel)
    }

    fn retry_restoration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.restoring {
            return;
        }
        self.restoring = true;
        let workspace = self.workspace.clone();
        self._restoration = cx.spawn_in(window, async move |panel, cx| {
            let result = Self::restore_serialized_state(workspace, panel.clone(), cx).await;
            let default_shell = panel
                .update_in(cx, |panel, window, cx| {
                    let restored = match result {
                        Ok(restored) => {
                            panel.restoration_error = None;
                            restored
                        }
                        Err(error) => {
                            log::error!("Terminal panel restoration failed: {error:#}");
                            panel.restoration_error =
                                Some(SharedString::from(format!("{error:#}")));
                            false
                        }
                    };
                    panel.finish_restoration(restored, window, cx)
                })
                .log_err()
                .flatten();
            if let Some(task) = default_shell {
                task.await.log_err();
            }
        });
        cx.notify();
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
        if restored
            || has_terminals
            || !self.primary_item_ids.is_empty()
            || self.publication_error.is_some()
        {
            self.serialize(cx);
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
        cx: &mut AsyncWindowContext,
    ) -> Result<bool> {
        let Some((database_id, kvp)) = workspace.read_with(cx, |workspace, cx| {
            workspace
                .database_id()
                .map(|id| (id, KeyValueStore::global(cx)))
        })?
        else {
            terminal_panel.update(cx, |panel, _| {
                panel.primary_loaded = true;
                panel.recovery_loaded = true;
            })?;
            return Ok(false);
        };
        let mut restored = false;
        let mut errors = Vec::new();
        for recovery in [true, false] {
            let loaded = terminal_panel.read_with(cx, |panel, _| {
                if recovery {
                    panel.recovery_loaded
                } else {
                    panel.primary_loaded
                }
            })?;
            if loaded {
                continue;
            }
            let key = if recovery {
                Self::recovery_key_for_workspace_id(database_id)
            } else {
                Self::serialization_key_for_workspace_id(database_id)
            };
            let result: Result<usize> = async {
                let raw = cx
                    .background_spawn({
                        let kvp = kvp.clone();
                        async move { kvp.read_kvp(&key) }
                    })
                    .await?;
                let Some(raw) = raw else {
                    return Ok(0);
                };
                let serialized = serde_json::from_str::<SerializedTerminalPanel>(&raw)?;
                serialized.validate_child_item_ids()?;
                let saved_ids = serialized.item_ids().into_iter().collect::<HashSet<_>>();
                workspace.update(cx, |workspace, cx| {
                    let mut item_ids = serialized.item_ids();
                    item_ids.extend(&serialized.primary_item_ids);
                    workspace.reserve_serialized_item_ids(database_id, "Terminal", &item_ids, cx)
                })??;
                let known = terminal_panel.read_with(cx, |panel, cx| {
                    let known = if recovery {
                        let mut known = panel
                            .published_recovery_item_ids
                            .lock()
                            .map_err(|error| anyhow!("Failed to read recovery publication state: {error}"))?
                            .clone();
                        known.extend(
                            serialized
                                .primary_item_ids
                                .iter()
                                .filter(|item_id| panel.primary_item_ids.contains(item_id)),
                        );
                        known
                    } else {
                        panel.primary_item_ids.clone()
                    };
                    let mut conflicts = panel
                        .center
                        .panes()
                        .into_iter()
                        .flat_map(|pane| {
                            pane.read(cx)
                                .items_of_type::<TerminalView>()
                                .filter_map(|view| view.read(cx).serialization_identity())
                                .filter_map(|(workspace_id, item_id)| {
                                    (workspace_id == database_id
                                        && saved_ids.contains(&item_id)
                                        && !known.contains(&item_id))
                                    .then_some(item_id)
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    conflicts.sort_unstable();
                    let layout = if recovery { "Recovery layout" } else { "Saved layout" };
                    anyhow::ensure!(
                        conflicts.is_empty(),
                        "{layout} references conflict with new terminal IDs {conflicts:?}; repair the saved layout before retrying"
                    );
                    anyhow::Ok(known)
                })??;
                terminal_panel.update(cx, |panel, _| {
                    panel.known_item_ids.lock().map_err(|error| anyhow!("Failed to read terminal publication state: {error}"))?.extend(&saved_ids);
                    anyhow::Ok(())
                })??;
                let primary_item_ids = if recovery {
                    serialized.primary_item_ids.clone()
                } else {
                    saved_ids.into_iter().collect()
                };
                let Some(mut serialized) = serialized.without_items(&known) else {
                    terminal_panel.update(cx, |panel, _| {
                        panel.primary_item_ids.extend(primary_item_ids);
                    })?;
                    return Ok(0);
                };
                serialized.primary_item_ids = primary_item_ids;
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        deserialize_terminal_panel(
                            workspace.weak_handle(),
                            workspace.project().clone(),
                            database_id,
                            serialized,
                            terminal_panel.clone(),
                            window,
                            cx,
                        )
                    })?
                    .await
            }
            .await;
            match result {
                Ok(count) => {
                    restored |= count > 0;
                    terminal_panel.update(cx, |panel, _| {
                        if recovery {
                            panel.recovery_loaded = true;
                        } else {
                            panel.primary_loaded = true;
                        }
                    })?;
                }
                Err(error) => errors.push(format!(
                    "{}: {error:#}",
                    if recovery {
                        "Recovery layout"
                    } else {
                        "Saved layout"
                    }
                )),
            }
        }
        if !errors.is_empty() {
            return Err(anyhow!("{}", errors.join("\n")));
        }

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
                        window.focus(&new_pane.focus_handle(cx), cx);
                    }
                };
            }
            pane::Event::Focus => {
                self.active_pane = pane.clone();
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

    fn serialize(&mut self, cx: &mut Context<Self>) {
        self.pending_serialization = cx.spawn(async move |terminal_panel, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            terminal_panel
                .update(cx, |terminal_panel, cx| terminal_panel.serialize_now(cx))
                .log_err()?;
            Some(())
        });
    }

    pub(crate) fn serialization_admission(
        &self,
        workspace: &mut Workspace,
        workspace_id: WorkspaceId,
        cx: &App,
    ) -> Result<TerminalSerializationAdmission> {
        let mut known_item_ids = self
            .known_item_ids
            .lock()
            .map_err(|error| anyhow!("Failed to read terminal publication state: {error}"))?
            .clone();
        known_item_ids.extend(&self.primary_item_ids);
        let recovery = self.restoring
            || !self.primary_loaded
            || !self.recovery_loaded
            || workspace.is_restoring();
        TerminalSerializationAdmission::new(workspace, workspace_id, recovery, known_item_ids, cx)
    }

    fn validate_serialized_item_ids(
        &self,
        workspace: &mut Workspace,
        workspace_id: WorkspaceId,
        cx: &App,
    ) -> Result<TerminalSerializationAdmission> {
        let admission = self.serialization_admission(workspace, workspace_id, cx)?;
        let mut live_ids = HashSet::default();
        for pane in self.center.panes() {
            for terminal in pane.read(cx).items_of_type::<TerminalView>() {
                if terminal.read(cx).terminal().read(cx).task().is_some() {
                    continue;
                }
                let item_id = workspace.serialization_id("Terminal", terminal.entity_id(), cx)?;
                anyhow::ensure!(
                    terminal
                        .read(cx)
                        .serialization_identity()
                        .is_none_or(|identity| identity == (workspace_id, item_id)),
                    "Terminal serialization identity does not match its workspace assignment"
                );
                live_ids.insert(item_id);
            }
        }
        admission.validate(&live_ids)?;
        Ok(admission)
    }

    fn serialize_now(&mut self, cx: &mut Context<Self>) {
        self.publication_token = Arc::new(());
        let mut recovery = self.restoring || !self.primary_loaded || !self.recovery_loaded;
        let capture = self
            .workspace
            .update(cx, |workspace, cx| {
                recovery |= workspace.is_restoring();
                let Some(workspace_id) = workspace.database_id() else {
                    return Ok(None);
                };
                let admission = self.validate_serialized_item_ids(workspace, workspace_id, cx)?;
                let (group, tasks) = serialize_pane_group(
                    &self.center,
                    &self.active_pane,
                    workspace,
                    &admission,
                    cx,
                )?;
                anyhow::Ok(Some((
                    workspace_id,
                    SerializedTerminalPanel {
                        items: SerializedItems::WithSplits(group),
                        active_item_id: None,
                        primary_item_ids: self.primary_item_ids.iter().copied().collect(),
                    },
                    tasks,
                    TerminalDb::global(cx),
                )))
            })
            .and_then(|capture| capture);
        let capture = capture.and_then(|capture| {
            capture
                .map(|(workspace_id, items, tasks, db)| {
                    let cleanup = if !recovery && self.needs_cleanup.load(Ordering::Relaxed) {
                        Some(self.cleanup(workspace_id, items.item_ids(), cx)?)
                    } else {
                        None
                    };
                    anyhow::Ok((workspace_id, items, tasks, db, cleanup))
                })
                .transpose()
        });
        if let Err(error) = &capture {
            self.publication_error = Some(SharedString::from(format!("{error:#}")));
            cx.notify();
        }
        let needs_cleanup = self.needs_cleanup.clone();
        let previous = self.pending_publication.take();
        let recovery_loaded = self.recovery_loaded;
        let known_item_ids = self.known_item_ids.clone();
        let published_recovery_item_ids = self.published_recovery_item_ids.clone();
        let (completion, completed) = oneshot::channel();
        self.pending_publication = Some(
            cx.background_spawn(async move {
                if let Some(previous) = previous {
                    previous.await.log_err();
                }
                let result: Result<()> = async {
                    let Some((workspace_id, items, tasks, db, cleanup)) = capture? else {
                        return Ok(());
                    };
                    for result in join_all(tasks).await {
                        result?;
                    }
                    let previous_live_ids = published_recovery_item_ids
                        .lock()
                        .map_err(|error| {
                            anyhow!("Failed to read recovery publication state: {error}")
                        })?
                        .clone();
                    let item_ids = items.item_ids();
                    let live_ids = if recovery {
                        item_ids.iter().copied().collect::<HashSet<_>>()
                    } else {
                        HashSet::default()
                    };
                    db.save_panel(
                        workspace_id,
                        items,
                        recovery,
                        recovery_loaded,
                        previous_live_ids,
                    )
                    .await?;
                    known_item_ids
                        .lock()
                        .map_err(|error| {
                            anyhow!("Failed to update terminal publication state: {error}")
                        })?
                        .extend(item_ids);
                    *published_recovery_item_ids.lock().map_err(|error| {
                        anyhow!("Failed to update recovery publication state: {error}")
                    })? = live_ids;
                    if needs_cleanup.load(Ordering::Relaxed)
                        && let Some(cleanup) = cleanup
                    {
                        cleanup.await?;
                        needs_cleanup.store(false, Ordering::Relaxed);
                    }
                    Ok(())
                }
                .await;
                let error = result
                    .as_ref()
                    .err()
                    .map(|error| SharedString::from(format!("{error:#}")));
                if completion.send(error).is_err() {
                    log::debug!("Terminal panel publication observer was dropped");
                }
                if let Err(error) = &result {
                    log::error!("Failed to publish terminal panel: {error:#}");
                }
                result.map_err(Arc::new)
            })
            .shared(),
        );
        let publication_token = self.publication_token.clone();
        cx.spawn(async move |panel, cx| {
            if let Ok(error) = completed.await {
                panel
                    .update(cx, |panel, cx| {
                        if Arc::ptr_eq(&panel.publication_token, &publication_token) {
                            panel.publication_error = error;
                            cx.notify();
                        }
                    })
                    .log_err();
            }
        })
        .detach();
    }

    fn flush_serialization(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.pending_serialization = Task::ready(None);
        self.serialize_now(cx);
        let publication = self.pending_publication.clone();
        cx.background_spawn(async move {
            if let Some(publication) = publication {
                publication.await.map_err(|error| anyhow!(error))?;
            }
            Ok(())
        })
    }

    fn app_will_quit(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let flush = self.flush_serialization(cx);
        cx.background_spawn(async move {
            flush.await.log_err();
        })
    }

    fn cleanup(
        &self,
        workspace_id: WorkspaceId,
        mut item_ids: Vec<workspace::ItemId>,
        cx: &mut Context<Self>,
    ) -> Result<impl Future<Output = Result<()>> + use<>> {
        self.workspace.update(cx, |workspace, cx| {
            item_ids.extend(workspace.live_serialized_item_ids("Terminal", cx));
            TerminalDb::global(cx).prepare_cleanup(workspace_id, item_ids)
        })
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
            cx.notify();
        }
    }

    fn move_pane_to_border(&mut self, direction: SplitDirection, cx: &mut Context<Self>) {
        if self
            .center
            .move_to_border(&self.active_pane, direction, cx)
            .unwrap()
        {
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
            None,
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
                        return item.downcast::<TerminalView>().is_some();
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
        let recovery_notice = self
            .publication_error
            .clone()
            .or_else(|| self.restoration_error.clone())
            .map(|error| {
                h_flex()
                    .p_2()
                    .gap_2()
                    .child(
                        v_flex()
                            .flex_1()
                            .child(Label::new(if self.publication_error.is_some() {
                                "Terminal changes could not be saved"
                            } else {
                                "Saved terminal layout could not be loaded"
                            }))
                            .child(Label::new(error).size(LabelSize::Small).color(Color::Muted))
                            .when(self.publication_error.is_some(), |notice| {
                                notice.child(
                                    Label::new("Keep this window open and retry before closing to avoid losing unsaved terminal changes")
                                        .size(LabelSize::Small),
                                )
                            }),
                    )
                    .child(
                        Button::new("retry-terminal-panel-restoration", "Retry")
                            .disabled(self.restoring)
                            .on_click(cx.listener(|panel, _, window, cx| {
                                panel.retry_restoration(window, cx)
                            })),
                    )
            });
        self.workspace
            .update(cx, |workspace, cx| {
                registrar
                    .track_focus(&self.focus_handle)
                    .size_full()
                    .relative()
                    .flex()
                    .flex_col()
                    .children(recovery_notice)
                    .child(div().flex_1().min_h_0().child(self.center.render(
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
                    )))
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
    fn flush_serialization(&self, cx: &mut App) -> Task<Result<()>> {
        let panel = self.0.clone();
        let (completion, completed) = oneshot::channel();
        cx.spawn(async move |cx| {
            let result = panel
                .update(cx, |panel, cx| panel.flush_serialization(cx))
                .await;
            if let Err(result) = completion.send(result) {
                result.log_err();
            }
        })
        .detach();
        cx.background_spawn(async move { completed.await? })
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

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use super::*;
    use crate::persistence::{SerializedPane, SerializedPaneGroup};
    use db::AppDatabase;
    use gpui::{Modifiers, TestAppContext, UpdateGlobal as _, VisualTestContext};
    use pretty_assertions::assert_eq;
    use project::FakeFs;
    use settings::SettingsStore;
    use workspace::{
        ItemId, MultiWorkspace, SaveIntent, SerializableItem as _,
        item::{Item as _, ItemEvent, SaveDisposition},
    };

    #[gpui::test]
    async fn test_panel_fallible_flush_retries_storage_failures(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        for payload_failure in [true, false] {
            let (window, panel) = init_workspace_with_panel(cx).await;
            let workspace = window
                .update(cx, |multi_workspace, _, _| {
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
            let db = cx.update(|cx| TerminalDb::global(cx));
            let kvp = cx.update(|cx| KeyValueStore::global(cx));
            let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
            db.write(move |connection| {
                connection.exec(if payload_failure {
                    "CREATE TRIGGER fail_panel_flush BEFORE INSERT ON terminals
                     BEGIN SELECT RAISE(FAIL, 'injected terminal payload failure'); END"
                } else {
                    "CREATE TRIGGER fail_panel_flush BEFORE INSERT ON kv_store
                     WHEN NEW.key LIKE '%TerminalPanel%'
                     BEGIN SELECT RAISE(FAIL, 'injected terminal graph failure'); END"
                })?()
            })
            .await
            .unwrap();
            let (_, item_id) = window
                .update(cx, |_, window, cx| {
                    let pane = panel.read(cx).active_pane.clone();
                    let terminal =
                        add_panel_display_terminal(&workspace, &pane, "retained", window, cx);
                    panel.update(cx, |panel, _| {
                        panel.pending_serialization = Task::ready(None)
                    });
                    terminal
                })
                .unwrap();
            assert!(
                panel
                    .update(cx, |panel, cx| panel.flush_serialization(cx))
                    .await
                    .is_err()
            );
            assert_eq!(kvp.read_kvp(&key).unwrap(), None);
            assert_eq!(panel_terminal_ids(&panel, cx), vec![item_id]);
            db.write(|connection| connection.exec("DROP TRIGGER fail_panel_flush")?())
                .await
                .unwrap();
            panel
                .update(cx, |panel, cx| panel.flush_serialization(cx))
                .await
                .unwrap();
            assert_eq!(saved_panel_terminal_ids(&kvp, &key), vec![item_id]);
            assert_eq!(
                db.get_terminal(item_id, workspace_id).unwrap(),
                (None, Some(String::from("retained")))
            );
        }
    }

    #[gpui::test]
    async fn test_panel_flush_survives_dropped_waiters(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, panel) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
        let (release, wait) = oneshot::channel();
        let (_, item_id) = window
            .update(cx, |_, window, cx| {
                let pane = panel.read(cx).active_pane.clone();
                let terminal = add_panel_display_terminal(&workspace, &pane, "queued", window, cx);
                panel.update(cx, |panel, cx| {
                    panel.pending_serialization = Task::ready(None);
                    panel.pending_publication = Some(
                        cx.background_spawn(async move {
                            wait.await.map_err(|error| Arc::new(anyhow!(error)))
                        })
                        .shared(),
                    );
                    drop(panel.flush_serialization(cx));
                });
                terminal
            })
            .unwrap();
        drop(cx.update(|cx| {
            workspace::TerminalProvider::flush_serialization(&TerminalProvider(panel.clone()), cx)
        }));
        cx.run_until_parked();
        assert_eq!(kvp.read_kvp(&key).unwrap(), None);
        release.send(()).unwrap();
        panel
            .update(cx, |panel, _| panel.pending_publication.clone())
            .unwrap()
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(saved_panel_terminal_ids(&kvp, &key), vec![item_id]);
        let db = cx.update(|cx| TerminalDb::global(cx));
        assert_eq!(
            db.get_terminal(item_id, workspace_id).unwrap(),
            (None, Some(String::from("queued")))
        );
    }

    #[gpui::test]
    async fn test_panel_flush_replaces_completed_success_with_capture_failure(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        for unreadable_sidecar in [false, true] {
            let (window, panel) = init_workspace_with_panel(cx).await;
            let workspace = window
                .update(cx, |multi_workspace, _, _| {
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
            let db = cx.update(|cx| TerminalDb::global(cx));
            let kvp = cx.update(|cx| KeyValueStore::global(cx));
            let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
            let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
            let (first, first_id) = window
                .update(cx, |_, window, cx| {
                    let pane = panel.read(cx).active_pane.clone();
                    add_panel_display_terminal(&workspace, &pane, "first", window, cx)
                })
                .unwrap();
            panel
                .update(cx, |panel, cx| panel.flush_serialization(cx))
                .await
                .unwrap();
            let previous = panel
                .update(cx, |panel, _| panel.pending_publication.clone())
                .unwrap();
            let saved = kvp.read_kvp(&key).unwrap();
            let (_, second_id) = window
                .update(cx, |_, window, cx| {
                    first.update(cx, |first, cx| {
                        first.set_custom_title(Some(String::from("changed")), cx)
                    });
                    let pane = panel.read(cx).active_pane.clone();
                    let terminal =
                        add_panel_display_terminal(&workspace, &pane, "second", window, cx);
                    panel.update(cx, |panel, _| {
                        panel.pending_serialization = Task::ready(None)
                    });
                    terminal
                })
                .unwrap();
            cx.update(|cx| {
                cx.foreground_executor()
                    .block_with_timeout(Duration::from_secs(1), async {
                        if unreadable_sidecar {
                            kvp.write_kvp(recovery_key.clone(), String::from("{")).await
                        } else {
                            db.write(|connection| {
                                connection
                                    .exec("ALTER TABLE kv_store RENAME TO unavailable_kv_store")?(
                                )
                            })
                            .await
                        }
                    })
                    .unwrap_or_else(|_| panic!("storage failure requires foreground work"))
                    .unwrap();
            });
            let flush = panel.update(cx, |panel, cx| panel.flush_serialization(cx));
            assert!(flush.await.is_err());
            previous.await.unwrap();
            assert!(
                panel
                    .update(cx, |panel, _| panel.pending_publication.clone())
                    .unwrap()
                    .await
                    .is_err()
            );
            assert_eq!(db.item_ids(workspace_id).unwrap(), vec![first_id]);
            assert_eq!(
                db.get_terminal(first_id, workspace_id).unwrap(),
                (None, Some(String::from("first")))
            );
            if unreadable_sidecar {
                assert_eq!(kvp.read_kvp(&recovery_key).unwrap().as_deref(), Some("{"));
                kvp.delete_kvp(recovery_key).await.unwrap();
            } else {
                db.write(|connection| {
                    connection.exec("ALTER TABLE unavailable_kv_store RENAME TO kv_store")?()
                })
                .await
                .unwrap();
            }
            assert_eq!(kvp.read_kvp(&key).unwrap(), saved);
            panel
                .update(cx, |panel, cx| panel.flush_serialization(cx))
                .await
                .unwrap();
            assert_eq!(
                saved_panel_terminal_ids(&kvp, &key),
                vec![first_id, second_id]
            );
            assert_eq!(
                db.get_terminal(first_id, workspace_id).unwrap(),
                (None, Some(String::from("changed")))
            );
        }
    }

    #[gpui::test]
    async fn test_panel_late_graph_collision_precedes_all_payload_writes(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        for saved_recovery in [false, true] {
            for loaded in [false, true] {
                let (window, panel) = init_workspace_with_panel(cx).await;
                let workspace = window
                    .update(cx, |multi_workspace, _, _| {
                        multi_workspace.workspace().clone()
                    })
                    .unwrap();
                let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
                let db = cx.update(|cx| TerminalDb::global(cx));
                let kvp = cx.update(|cx| KeyValueStore::global(cx));
                let (_, collision_id) = window
                    .update(cx, |_, window, cx| {
                        let pane = panel.read(cx).active_pane.clone();
                        add_panel_display_terminal(
                            &workspace,
                            &pane,
                            "earlier new terminal",
                            window,
                            cx,
                        );
                        let collision = add_panel_display_terminal(
                            &workspace,
                            &pane,
                            "must not overwrite",
                            window,
                            cx,
                        );
                        panel.update(cx, |panel, _| {
                            panel.pending_serialization = Task::ready(None);
                            panel.primary_loaded = loaded;
                            panel.recovery_loaded = loaded;
                        });
                        collision
                    })
                    .unwrap();
                if loaded {
                    db.write(|connection| {
                        connection.exec(
                            "CREATE TRIGGER fail_late_panel_payload BEFORE INSERT ON terminals
                             BEGIN SELECT RAISE(FAIL, 'injected terminal payload failure'); END",
                        )?()
                    })
                    .await
                    .unwrap();
                    assert!(
                        panel
                            .update(cx, |panel, cx| panel.flush_serialization(cx))
                            .await
                            .is_err()
                    );
                    db.write(|connection| {
                        connection.exec("DROP TRIGGER fail_late_panel_payload")?()
                    })
                    .await
                    .unwrap();
                }
                let key = if saved_recovery {
                    TerminalPanel::recovery_key_for_workspace_id(workspace_id)
                } else {
                    TerminalPanel::serialization_key_for_workspace_id(workspace_id)
                };
                let raw =
                    serde_json::json!({"items": [collision_id], "active_item_id": collision_id})
                        .to_string();
                kvp.write_kvp(key.clone(), raw.clone()).await.unwrap();
                db.save_terminal(
                    collision_id,
                    workspace_id,
                    None,
                    Some(String::from("saved payload")),
                )
                .await
                .unwrap();
                let error = panel
                    .update(cx, |panel, cx| panel.flush_serialization(cx))
                    .await
                    .unwrap_err();
                let layout = if saved_recovery {
                    "Recovery layout"
                } else {
                    "Saved layout"
                };
                assert_eq!(
                    error.to_string(),
                    format!(
                        "{layout} references conflict with new terminal IDs [{collision_id}]; repair the saved references before retrying"
                    )
                );
                assert_eq!(kvp.read_kvp(&key).unwrap(), Some(raw));
                assert_eq!(db.item_ids(workspace_id).unwrap(), vec![collision_id]);
                assert_eq!(
                    db.get_terminal(collision_id, workspace_id).unwrap(),
                    (None, Some(String::from("saved payload")))
                );
            }
        }
    }

    #[gpui::test]
    async fn test_queued_terminal_payload_rejects_repaired_graph_collision(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        for recovery in [false, true] {
            for in_center in [false, true] {
                let (window, panel) = init_workspace_with_panel(cx).await;
                let workspace = window
                    .update(cx, |multi_workspace, _, _| {
                        multi_workspace.workspace().clone()
                    })
                    .unwrap();
                let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
                let db = cx.update(|cx| TerminalDb::global(cx));
                let kvp = cx.update(|cx| KeyValueStore::global(cx));
                let primary_key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
                kvp.write_kvp(primary_key.clone(), String::from("{"))
                    .await
                    .unwrap();
                let (live, item_id) = window
                    .update(cx, |_, window, cx| {
                        let pane = if in_center {
                            workspace.read(cx).active_pane().clone()
                        } else {
                            panel.read(cx).active_pane.clone()
                        };
                        add_panel_display_terminal(&workspace, &pane, "LIVE_λ", window, cx)
                    })
                    .unwrap();
                panel.update(cx, |panel, _| {
                    panel.primary_loaded = false;
                    panel.pending_serialization = Task::ready(None);
                });
                assert_eq!(db.item_ids(workspace_id).unwrap(), Vec::<ItemId>::new());
                let key = if recovery {
                    TerminalPanel::recovery_key_for_workspace_id(workspace_id)
                } else {
                    primary_key
                };
                let saved =
                    serde_json::json!({"items": [item_id], "active_item_id": item_id}).to_string();
                cx.update(|cx| {
                    cx.foreground_executor()
                        .block_with_timeout(Duration::from_secs(1), async {
                            kvp.write_kvp(key.clone(), saved.clone()).await?;
                            db.save_terminal(
                                item_id,
                                workspace_id,
                                Some(PathBuf::from("saved_λ")),
                                Some(String::from("SAVE_λ")),
                            )
                            .await
                        })
                        .unwrap_or_else(|_| panic!("repair requires foreground work"))
                        .unwrap();
                });
                cx.run_until_parked();
                cx.executor()
                    .advance_clock(workspace::SERIALIZATION_THROTTLE_TIME);
                cx.run_until_parked();
                assert_eq!(kvp.read_kvp(&key).unwrap(), Some(saved));
                assert_eq!(
                    db.get_terminal(item_id, workspace_id).unwrap(),
                    (Some(PathBuf::from("saved_λ")), Some(String::from("SAVE_λ")))
                );
                assert_eq!(db.item_ids(workspace_id).unwrap(), vec![item_id]);
                live.read_with(cx, |live, _| {
                    assert_eq!(live.serialization_identity(), Some((workspace_id, item_id)));
                    assert_eq!(live.custom_title(), Some("LIVE_λ"));
                    assert!(live.needs_serialize);
                    assert!(live.pending_serialization.is_none());
                });
                assert!(panel.read_with(cx, |panel, _| panel.pending_publication.is_none()));
            }
        }
    }

    #[gpui::test]
    async fn test_stale_source_queue_preserves_moved_terminal_metadata(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        let (source_window, source_panel) = init_workspace_with_panel(cx).await;
        let source = source_window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let source_id = initialize_terminal_persistence(&source, &[], cx).await;
        let (destination_window, destination_panel) = init_workspace_with_panel(cx).await;
        let destination = destination_window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let destination_id = initialize_terminal_persistence(&destination, &[], cx).await;
        let (view, source_item_id) = source_window
            .update(cx, |_, window, cx| {
                let pane = source_panel.read(cx).active_pane.clone();
                add_panel_display_terminal(&source, &pane, "SAVE_λ", window, cx)
            })
            .unwrap();
        source_panel
            .update(cx, |panel, cx| panel.flush_serialization(cx))
            .await
            .unwrap();
        cx.run_until_parked();
        view.update(cx, |view, cx| {
            view.set_custom_title(Some(String::from("queued_λ")), cx)
        });
        cx.run_until_parked();
        let db = cx.update(|cx| TerminalDb::global(cx));
        let saved_cwd = PathBuf::from("source_saved_λ");
        cx.update(|cx| {
            cx.foreground_executor()
                .block_with_timeout(
                    Duration::from_secs(1),
                    db.save_terminal(
                        source_item_id,
                        source_id,
                        Some(saved_cwd.clone()),
                        Some(String::from("SAVE_λ")),
                    ),
                )
                .unwrap_or_else(|_| panic!("source sentinel requires foreground work"))
                .unwrap();
        });
        view.update(cx, |view, cx| {
            view.set_custom_title(Some(String::from("LIVE_λ")), cx)
        });
        cx.run_until_parked();
        let source_pane = source_panel.read_with(cx, |panel, _| panel.active_pane.clone());
        let destination_pane =
            destination_panel.read_with(cx, |panel, _| panel.active_pane.clone());
        destination_window
            .update(cx, |_, window, cx| {
                workspace::move_item(
                    &source_pane,
                    &destination_pane,
                    view.entity_id(),
                    0,
                    true,
                    window,
                    cx,
                );
            })
            .unwrap();
        for panel in [&source_panel, &destination_panel] {
            panel.update(cx, |panel, _| {
                panel.pending_serialization = Task::ready(None)
            });
        }
        let destination_item_id = destination
            .update(cx, |workspace, cx| {
                workspace.serialization_id("Terminal", view.entity_id(), cx)
            })
            .unwrap();
        cx.run_until_parked();
        cx.executor()
            .advance_clock(workspace::SERIALIZATION_THROTTLE_TIME);
        cx.run_until_parked();
        let live_cwd = view.read_with(cx, |view, cx| {
            assert_eq!(view.workspace, destination.downgrade());
            assert_eq!(
                view.serialization_identity(),
                Some((destination_id, destination_item_id))
            );
            assert_eq!(view.custom_title(), Some("LIVE_λ"));
            view.terminal().read(cx).working_directory()
        });
        assert_eq!(live_cwd, None);
        assert_eq!(
            db.get_terminal(source_item_id, source_id).unwrap(),
            (Some(saved_cwd.clone()), Some(String::from("SAVE_λ")))
        );
        let flush = cx.update(|cx| {
            workspace::TerminalProvider::flush_serialization(
                &TerminalProvider(destination_panel.clone()),
                cx,
            )
        });
        flush.await.unwrap();
        assert_eq!(
            db.get_terminal(destination_item_id, destination_id)
                .unwrap(),
            (live_cwd, Some(String::from("LIVE_λ")))
        );
        assert_eq!(
            db.get_terminal(source_item_id, source_id).unwrap(),
            (Some(saved_cwd), Some(String::from("SAVE_λ")))
        );
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        assert_eq!(
            saved_panel_terminal_ids(
                &kvp,
                &TerminalPanel::serialization_key_for_workspace_id(destination_id)
            ),
            vec![destination_item_id]
        );
    }

    #[gpui::test]
    async fn test_duplicate_terminal_children_reject_before_spawn_and_retry(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        for recovery in [false, true] {
            for items in [
                serde_json::json!([701, 701]),
                serde_json::json!({"Pane": {"active": true, "children": [701, 701], "active_item": 701}}),
                serde_json::json!({"Group": {"axis": "horizontal", "flexes": null, "children": [
                    {"Pane": {"active": true, "children": [701], "active_item": 701}},
                    {"Group": {"axis": "vertical", "flexes": null, "children": [
                        {"Pane": {"active": false, "children": [701], "active_item": 701}}
                    ]}}
                ]}}),
            ] {
                let (window, panel) = init_workspace_with_panel(cx).await;
                let workspace = window
                    .update(cx, |multi_workspace, _, _| {
                        multi_workspace.workspace().clone()
                    })
                    .unwrap();
                let workspace_id = initialize_terminal_persistence(&workspace, &[701], cx).await;
                let db = cx.update(|cx| TerminalDb::global(cx));
                db.save_terminal(701, workspace_id, None, Some(String::from("SAVE_λ")))
                    .await
                    .unwrap();
                let kvp = cx.update(|cx| KeyValueStore::global(cx));
                let key = if recovery {
                    TerminalPanel::recovery_key_for_workspace_id(workspace_id)
                } else {
                    TerminalPanel::serialization_key_for_workspace_id(workspace_id)
                };
                let saved = serde_json::json!({"items": items, "active_item_id": 701}).to_string();
                kvp.write_kvp(key.clone(), saved.clone()).await.unwrap();
                let (live, live_id) = window
                    .update(cx, |_, window, cx| {
                        let pane = panel.read(cx).active_pane.clone();
                        let live =
                            add_panel_display_terminal(&workspace, &pane, "LIVE_λ", window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.primary_loaded = recovery;
                            panel.recovery_loaded = !recovery;
                            panel.retry_restoration(window, cx);
                        });
                        live
                    })
                    .unwrap();
                panel
                    .update(cx, |panel, _| {
                        std::mem::replace(&mut panel._restoration, Task::ready(()))
                    })
                    .await;
                cx.run_until_parked();
                let layout = if recovery {
                    "Recovery layout"
                } else {
                    "Saved layout"
                };
                assert_eq!(
                    panel.read_with(cx, |panel, _| panel
                        .restoration_error
                        .as_ref()
                        .map(ToString::to_string)),
                    Some(format!(
                        "{layout}: Terminal layout contains duplicate child ID 701"
                    ))
                );
                assert_eq!(panel_terminal_ids(&panel, cx), vec![live_id]);
                assert_eq!(
                    workspace.read_with(cx, |workspace, _| workspace
                        .assigned_serialized_item_ids("Terminal")),
                    vec![live_id]
                );
                assert_eq!(
                    db.get_terminal(701, workspace_id).unwrap(),
                    (None, Some(String::from("SAVE_λ")))
                );
                assert_eq!(kvp.read_kvp(&key).unwrap(), Some(saved));
                kvp.write_kvp(
                    key.clone(),
                    String::from(r#"{"items":[701],"active_item_id":701}"#),
                )
                .await
                .unwrap();
                window
                    .update(cx, |_, window, cx| {
                        panel.update(cx, |panel, cx| panel.retry_restoration(window, cx))
                    })
                    .unwrap();
                panel
                    .update(cx, |panel, _| {
                        std::mem::replace(&mut panel._restoration, Task::ready(()))
                    })
                    .await;
                assert_eq!(
                    panel.read_with(cx, |panel, _| panel.restoration_error.clone()),
                    None
                );
                assert_eq!(panel_terminal_ids(&panel, cx), vec![701, live_id]);
                assert_eq!(panel_terminal(&panel, live_id, cx), live);
                let restored = panel_terminal(&panel, 701, cx);
                assert_eq!(
                    restored.read_with(cx, |view, _| view.custom_title().map(String::from)),
                    Some(String::from("SAVE_λ"))
                );
                panel
                    .update(cx, |panel, cx| panel.flush_serialization(cx))
                    .await
                    .unwrap();
                assert_eq!(
                    db.get_custom_title(701, workspace_id).unwrap(),
                    Some(String::from("SAVE_λ"))
                );
                assert_eq!(panel_terminal(&panel, 701, cx), restored);
            }
        }
    }

    #[gpui::test]
    async fn test_failed_terminal_batch_releases_registered_owners(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        for (split, visible) in [(false, false), (true, false), (false, true), (true, true)] {
            let (window, panel) = init_workspace_with_panel(cx).await;
            let workspace = window
                .update(cx, |multi_workspace, _, _| {
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            let workspace_id = initialize_terminal_persistence(&workspace, &[701, 702], cx).await;
            let db = cx.update(|cx| TerminalDb::global(cx));
            db.save_terminal(701, workspace_id, None, Some(String::from("SAVE_λ")))
                .await
                .unwrap();
            let kvp = cx.update(|cx| KeyValueStore::global(cx));
            let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
            let saved = if split {
                serde_json::json!({"items": {"Group": {"axis": "horizontal", "flexes": null, "children": [
                {"Pane": {"active": true, "children": [701], "active_item": 701}},
                {"Pane": {"active": false, "children": [702], "active_item": 702}}
            ]}}, "active_item_id": 701}).to_string()
            } else {
                String::from(r#"{"items":[701,702],"active_item_id":701}"#)
            };
            kvp.write_kvp(key.clone(), saved.clone()).await.unwrap();

            let (original, live) = window
                .update(cx, |_, window, cx| {
                    let pane = workspace.read(cx).active_pane().clone();
                    let original =
                        add_failed_terminal(&workspace, &pane, workspace_id, 702, window, cx);
                    let live = visible.then(|| {
                        let pane = panel.read(cx).active_pane.clone();
                        let live =
                            add_panel_display_terminal(&workspace, &pane, "LIVE_λ", window, cx);
                        workspace.update(cx, |workspace, cx| {
                            workspace.focus_panel::<TerminalPanel>(window, cx);
                        });
                        live
                    });
                    (original, live)
                })
                .unwrap();
            cx.run_until_parked();
            let active_pane = panel.read_with(cx, |panel, _| panel.active_pane.clone());
            let pane_state = terminal_pane_state(&active_pane, cx);
            let status_item = window
                .update(cx, |_, window, cx| {
                    observe_terminal_status(&workspace, window, cx)
                })
                .unwrap();
            let status_item_id = live
                .as_ref()
                .map_or(original.entity_id(), |(live, _)| live.entity_id());
            assert_eq!(
                status_item.read_with(cx, |item, _| item.item_ids.clone()),
                vec![Some(status_item_id)]
            );
            window
                .update(cx, |_, window, cx| {
                    panel.update(cx, |panel, cx| {
                        panel.primary_loaded = false;
                        panel.retry_restoration(window, cx);
                    });
                })
                .unwrap();
            panel
                .update(cx, |panel, _| {
                    std::mem::replace(&mut panel._restoration, Task::ready(()))
                })
                .await;
            cx.run_until_parked();
            assert_eq!(
                panel.read_with(cx, |panel, _| panel
                    .restoration_error
                    .as_ref()
                    .map(ToString::to_string)),
                Some(String::from(
                    "Saved layout: serialized item ID 702 already belongs to another item"
                ))
            );
            let live_panel_ids = live.iter().map(|(_, item_id)| *item_id).collect::<Vec<_>>();
            assert_eq!(panel_terminal_ids(&panel, cx), live_panel_ids);
            assert_eq!(
                panel.read_with(cx, |panel, _| panel.active_pane.clone()),
                active_pane
            );
            assert_eq!(
                panel.read_with(cx, |panel, _| panel
                    .center
                    .panes()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<_>>()),
                vec![active_pane.clone()]
            );
            assert_eq!(terminal_pane_state(&active_pane, cx), pane_state);
            assert_eq!(
                status_item.read_with(cx, |item, _| item.item_ids.clone()),
                vec![Some(status_item_id)]
            );
            if let Some((live, _)) = &live {
                window
                    .update(cx, |_, window, cx| {
                        workspace.update(cx, |workspace, cx| {
                            assert!(
                                workspace.is_dock_at_position_open(
                                    panel.read(cx).position(window, cx),
                                    cx
                                )
                            );
                        });
                        assert!(live.focus_handle(cx).contains_focused(window, cx));
                    })
                    .unwrap();
            }
            cx.executor()
                .advance_clock(workspace::SERIALIZATION_THROTTLE_TIME);
            cx.run_until_parked();
            db.write(|_| ()).await;
            cx.run_until_parked();

            let mut live_owner_ids = vec![702];
            live_owner_ids.extend(&live_panel_ids);
            live_owner_ids.sort_unstable();
            assert_eq!(
                workspace.read_with(cx, |workspace, cx| workspace
                    .live_serialized_item_ids("Terminal", cx)),
                live_owner_ids,
                "split: {split}, visible: {visible}"
            );
            assert_eq!(kvp.read_kvp(&key).unwrap(), Some(saved));
            assert_eq!(
                db.get_custom_title(701, workspace_id).unwrap(),
                Some(String::from("SAVE_λ"))
            );
            kvp.write_kvp(key, String::from(r#"{"items":[701],"active_item_id":701}"#))
                .await
                .unwrap();
            window
                .update(cx, |_, window, cx| {
                    panel.update(cx, |panel, cx| panel.retry_restoration(window, cx))
                })
                .unwrap();
            panel
                .update(cx, |panel, _| {
                    std::mem::replace(&mut panel._restoration, Task::ready(()))
                })
                .await;
            assert_eq!(
                panel.read_with(cx, |panel, _| panel.restoration_error.clone()),
                None
            );
            let mut restored_panel_ids = vec![701];
            restored_panel_ids.extend(live_panel_ids);
            restored_panel_ids.sort_unstable();
            assert_eq!(panel_terminal_ids(&panel, cx), restored_panel_ids);
            live_owner_ids.push(701);
            live_owner_ids.sort_unstable();
            assert_eq!(
                workspace.read_with(cx, |workspace, cx| workspace
                    .live_serialized_item_ids("Terminal", cx)),
                live_owner_ids
            );
            assert_eq!(
                workspace.read_with(cx, |workspace, _| {
                    let mut ids = workspace.assigned_serialized_item_ids("Terminal");
                    ids.sort_unstable();
                    ids
                }),
                live_owner_ids
            );
            assert_eq!(
                workspace.read_with(cx, |workspace, cx| workspace
                    .active_item(cx)
                    .unwrap()
                    .item_id()),
                original.entity_id()
            );
            if let Some((live, live_id)) = live {
                assert_eq!(panel_terminal(&panel, live_id, cx), live);
                assert_eq!(
                    active_pane.read_with(cx, |pane, _| pane.active_item().unwrap().item_id()),
                    live.entity_id()
                );
            }
            panel
                .update(cx, |panel, cx| panel.flush_serialization(cx))
                .await
                .unwrap();
            assert_eq!(
                db.get_custom_title(701, workspace_id).unwrap(),
                Some(String::from("SAVE_λ"))
            );
            assert_eq!(
                db.get_terminal(702, workspace_id).unwrap(),
                (None, Some(String::from("terminal-702")))
            );
        }
    }

    #[gpui::test]
    async fn test_failed_terminal_batch_prepares_empty_pane_shell_before_activation(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, panel) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[701], cx).await;
        let db = cx.update(|cx| TerminalDb::global(cx));
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
        let saved = serde_json::json!({"items": {"Group": {
            "axis": "horizontal", "flexes": [0.5, 0.5], "children": [
                {"Pane": {"active": true, "children": [701], "active_item": 701, "pinned_count": 1}},
                {"Group": {"axis": "vertical", "flexes": null, "children": [
                    {"Pane": {"active": false, "children": [], "active_item": null}}
                ]}}
            ]
        }}, "active_item_id": 701}).to_string();
        kvp.write_kvp(key.clone(), saved.clone()).await.unwrap();
        let (live, live_id, active_pane) = window
            .update(cx, |_, window, cx| {
                let pane = panel.read(cx).active_pane.clone();
                let (live, live_id) =
                    add_panel_display_terminal(&workspace, &pane, "LIVE_λ", window, cx);
                workspace.update(cx, |workspace, cx| {
                    workspace.focus_panel::<TerminalPanel>(window, cx);
                });
                (live, live_id, pane)
            })
            .unwrap();
        cx.run_until_parked();
        let pane_state = terminal_pane_state(&active_pane, cx);
        let status_item = window
            .update(cx, |_, window, cx| {
                observe_terminal_status(&workspace, window, cx)
            })
            .unwrap();
        assert_eq!(
            status_item.read_with(cx, |item, _| item.item_ids.clone()),
            vec![Some(live.entity_id())]
        );
        let previous_shell = cx.update(|cx| {
            let mut previous_shell = None;
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    previous_shell = settings
                        .terminal
                        .get_or_insert_default()
                        .project
                        .shell
                        .replace(settings::Shell::Program(String::from(
                            "__nonexistent_shell__",
                        )));
                });
            });
            previous_shell
        });
        window
            .update(cx, |_, window, cx| {
                panel.update(cx, |panel, cx| {
                    panel.primary_loaded = false;
                    panel.retry_restoration(window, cx);
                });
            })
            .unwrap();
        panel
            .update(cx, |panel, _| {
                std::mem::replace(&mut panel._restoration, Task::ready(()))
            })
            .await;
        cx.run_until_parked();
        assert!(panel.read_with(cx, |panel, _| panel.restoration_error.is_some()));
        assert_eq!(panel_terminal_ids(&panel, cx), vec![live_id]);
        assert_eq!(
            panel.read_with(cx, |panel, _| panel.active_pane.clone()),
            active_pane
        );
        assert_eq!(
            panel.read_with(cx, |panel, _| panel
                .center
                .panes()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()),
            vec![active_pane.clone()]
        );
        assert_eq!(terminal_pane_state(&active_pane, cx), pane_state);
        assert_eq!(
            status_item.read_with(cx, |item, _| item.item_ids.clone()),
            vec![Some(live.entity_id())]
        );
        assert_eq!(
            workspace.read_with(cx, |workspace, cx| workspace
                .live_serialized_item_ids("Terminal", cx)),
            vec![live_id]
        );
        let mut failed_owner_ids = vec![701, live_id];
        failed_owner_ids.sort_unstable();
        assert_eq!(
            workspace.read_with(cx, |workspace, _| {
                let mut ids = workspace.assigned_serialized_item_ids("Terminal");
                ids.sort_unstable();
                ids
            }),
            failed_owner_ids
        );
        assert_eq!(kvp.read_kvp(&key).unwrap(), Some(saved));
        assert_eq!(
            db.get_terminal(701, workspace_id).unwrap(),
            (None, Some(String::from("terminal-701")))
        );
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.terminal.get_or_insert_default().project.shell = previous_shell;
                });
            });
        });
        window
            .update(cx, |_, window, cx| {
                panel.update(cx, |panel, cx| panel.retry_restoration(window, cx));
            })
            .unwrap();
        panel
            .update(cx, |panel, _| {
                std::mem::replace(&mut panel._restoration, Task::ready(()))
            })
            .await;
        cx.run_until_parked();
        assert_eq!(
            panel.read_with(cx, |panel, _| panel.restoration_error.clone()),
            None
        );
        panel
            .update(cx, |panel, cx| panel.flush_serialization(cx))
            .await
            .unwrap();
        let panes = panel.read_with(cx, |panel, _| {
            panel
                .center
                .panes()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        });
        assert_eq!(panes.len(), 3);
        let restored = panel_terminal(&panel, 701, cx);
        assert_eq!(
            terminal_pane_state(&panes[0], cx),
            (vec![restored.entity_id()], 1, 0, None)
        );
        let default_terminal = panes[2].read_with(cx, |pane, _| {
            pane.items_of_type::<TerminalView>().next().unwrap()
        });
        assert_eq!(
            terminal_pane_state(&panes[2], cx),
            (vec![default_terminal.entity_id()], 0, 0, None)
        );
        assert_eq!(panes[1], active_pane);
        assert_eq!(terminal_pane_state(&active_pane, cx), pane_state);
        assert_eq!(panel_terminal(&panel, live_id, cx), live);
        let default_item_id = default_terminal.read_with(cx, |terminal, _| {
            terminal.serialization_identity().unwrap().1
        });
        let mut owner_ids = vec![701, live_id, default_item_id];
        owner_ids.sort_unstable();
        assert_eq!(panel_terminal_ids(&panel, cx), owner_ids);
        assert_eq!(
            workspace.read_with(cx, |workspace, cx| workspace
                .live_serialized_item_ids("Terminal", cx)),
            owner_ids
        );
        assert_eq!(
            workspace.read_with(cx, |workspace, _| {
                let mut ids = workspace.assigned_serialized_item_ids("Terminal");
                ids.sort_unstable();
                ids
            }),
            owner_ids
        );
    }

    #[gpui::test]
    async fn test_panel_close_failure_preserves_session_membership(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, panel) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
        let (_other_window, _other_panel) = init_workspace_with_panel(cx).await;
        join_all(
            window
                .update(cx, |multi_workspace, window, cx| {
                    multi_workspace.flush_all_serialization(window, cx)
                })
                .unwrap(),
        )
        .await;
        let db = cx.update(|cx| TerminalDb::global(cx));
        let binding = terminal_session_binding(&db, workspace_id);
        assert!(binding.0.is_some());
        assert!(binding.1.is_some());
        workspace.update(cx, |workspace, cx| {
            workspace
                .reserve_serialized_item_ids(workspace_id, "Terminal", &[], cx)
                .unwrap();
        });
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
        kvp.write_kvp(recovery_key.clone(), String::from("{"))
            .await
            .unwrap();
        let (_, item_id) = window
            .update(cx, |_, window, cx| {
                let pane = panel.read(cx).active_pane.clone();
                let terminal = add_panel_display_terminal(&workspace, &pane, "unsaved", window, cx);
                panel.update(cx, |panel, _| {
                    panel.pending_serialization = Task::ready(None)
                });
                terminal
            })
            .unwrap();
        let quitting = cx.spawn(async move |mut cx| {
            workspace::prepare_windows_to_quit(&[window], &mut cx).await
        });
        assert!(!quitting.await);
        assert_eq!(terminal_session_binding(&db, workspace_id), binding);
        window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.close_window(&workspace::CloseWindow, window, cx)
            })
            .unwrap();
        cx.run_until_parked();
        assert!(window.read_with(cx, |_, _| ()).is_ok());
        assert_eq!(terminal_session_binding(&db, workspace_id), binding);
        assert_eq!(panel_terminal_ids(&panel, cx), vec![item_id]);
        assert_eq!(kvp.read_kvp(&recovery_key).unwrap().as_deref(), Some("{"));
        assert_eq!(db.item_ids(workspace_id).unwrap(), Vec::<ItemId>::new());
        kvp.delete_kvp(recovery_key).await.unwrap();
        let quitting = cx.spawn(async move |mut cx| {
            workspace::prepare_windows_to_quit(&[window], &mut cx).await
        });
        assert!(quitting.await);
        assert_eq!(terminal_session_binding(&db, workspace_id), binding);
        assert_eq!(db.item_ids(workspace_id).unwrap(), vec![item_id]);
    }

    #[gpui::test]
    async fn test_panel_flush_does_not_override_close_cancel(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, panel) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[121], cx).await;
        let (_other_window, _other_panel) = init_workspace_with_panel(cx).await;
        let failed = window
            .update(cx, |_, window, cx| {
                let pane = workspace.read(cx).active_pane().clone();
                add_failed_terminal(&workspace, &pane, workspace_id, 121, window, cx)
            })
            .unwrap();
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
        kvp.write_kvp(recovery_key.clone(), String::from("{"))
            .await
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let closing = workspace.update_in(cx, |workspace, window, cx| {
            workspace.prepare_to_close(workspace::CloseIntent::CloseWindow, window, cx)
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("Cancel");
        assert!(!closing.await.unwrap());
        assert!(panel.read_with(cx, |panel, _| panel.pending_publication.is_none()));
        assert_eq!(kvp.read_kvp(&recovery_key).unwrap().as_deref(), Some("{"));
        assert_eq!(
            failed.read_with(cx, |failed, _| failed.serialization_identity()),
            Some((workspace_id, 121))
        );
    }

    #[gpui::test]
    async fn test_failed_terminal_cross_workspace_tabbar_drop_preserves_pins(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, panel) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[121], cx).await;
        let source = panel.read_with(cx, |panel, _| panel.active_pane.clone());
        let failed = window
            .update(cx, |_, window, cx| {
                add_failed_terminal(&workspace, &source, workspace_id, 121, window, cx)
            })
            .unwrap();
        let (other_window, other_panel) = init_workspace_with_panel(cx).await;
        let other_workspace = other_window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        initialize_terminal_persistence(&other_workspace, &[], cx).await;
        let destinations = [
            other_panel.read_with(cx, |panel, _| panel.active_pane.clone()),
            other_workspace.read_with(cx, |workspace, _| workspace.active_pane().clone()),
        ];
        let cx = &mut VisualTestContext::from_window(other_window.into(), cx);
        for destination in destinations {
            cx.update(|window, cx| {
                add_panel_display_terminal(
                    &other_workspace,
                    &destination,
                    "destination",
                    window,
                    cx,
                );
            });
            for source_pinned in [0, 1] {
                for destination_pinned in [0, 1] {
                    source.update(cx, |pane, _| pane.set_pinned_count(source_pinned));
                    destination.update(cx, |pane, _| pane.set_pinned_count(destination_pinned));
                    let before_source = terminal_pane_state(&source, cx);
                    let before_destination = terminal_pane_state(&destination, cx);
                    destination.update_in(cx, |pane, window, cx| {
                        pane.handle_tab_drop(
                            &DraggedTab {
                                pane: source.clone(),
                                item: Box::new(failed.clone()),
                                ix: 0,
                                detail: 0,
                                is_active: true,
                            },
                            0,
                            false,
                            window,
                            cx,
                        );
                    });
                    cx.run_until_parked();
                    assert!(cx.has_pending_prompt());
                    cx.simulate_prompt_answer("OK");
                    cx.run_until_parked();
                    assert_eq!(terminal_pane_state(&source, cx), before_source);
                    assert_eq!(terminal_pane_state(&destination, cx), before_destination);
                    assert_eq!(
                        failed.read_with(cx, |failed, _| failed.serialization_identity()),
                        Some((workspace_id, 121))
                    );
                }
            }
        }
        let db = cx.update(|_, cx| TerminalDb::global(cx));
        assert_eq!(
            db.get_terminal(121, workspace_id).unwrap(),
            (None, Some(String::from("terminal-121")))
        );
    }

    #[gpui::test]
    async fn test_terminal_cleanup_retains_both_committed_graphs_in_either_order(
        cx: &mut TestAppContext,
    ) {
        let db = cx.update(|cx| {
            cx.set_global(AppDatabase::test_new());
            TerminalDb::global(cx)
        });
        let workspace_id = WorkspaceId::from_i64(1);
        let other_workspace_id = WorkspaceId::from_i64(2);
        for panel in [
            r#"{"items":[22],"active_item_id":22}"#,
            r#"{"items":{"Group":{"axis":"horizontal","flexes":null,"children":[{"Pane":{"active":true,"children":[22],"active_item":22,"pinned_count":0}}]}},"active_item_id":null}"#,
        ] {
            for cleanup_order in [[11, 22], [22, 11]] {
                db.write(move |connection| {
                    connection.exec(
                        "DELETE FROM items;
                         DELETE FROM panes;
                         DELETE FROM terminals;
                         DELETE FROM kv_store;
                         INSERT OR IGNORE INTO workspaces (workspace_id) VALUES (1), (2);
                         INSERT INTO panes (pane_id, workspace_id, active) VALUES (1, 1, 1);
                         INSERT INTO items (item_id, workspace_id, pane_id, kind, position, active)
                         VALUES (11, 1, 1, 'Terminal', 0, 1), (44, 1, 1, 'Editor', 1, 0);
                         INSERT INTO terminals (workspace_id, item_id, working_directory)
                         VALUES (1, 11, NULL), (1, 22, NULL), (1, 33, X'ff'),
                                (1, 44, NULL), (2, 55, NULL);",
                    )?()?;
                    connection.exec_bound::<(String, &str)>(
                        "INSERT INTO kv_store (key, value) VALUES (?, ?)",
                    )?((
                        TerminalPanel::serialization_key_for_workspace_id(workspace_id),
                        panel,
                    ))
                })
                .await
                .unwrap();
                assert_eq!(db.item_ids(workspace_id).unwrap(), vec![11, 22, 33, 44]);
                for item_id in cleanup_order {
                    db.cleanup(workspace_id, vec![item_id]).await.unwrap();
                    assert_eq!(db.item_ids(workspace_id).unwrap(), vec![11, 22]);
                    assert_eq!(db.item_ids(other_workspace_id).unwrap(), vec![55]);
                }
                db.cleanup(workspace_id, Vec::new()).await.unwrap();
                assert_eq!(db.item_ids(workspace_id).unwrap(), vec![11, 22]);
                db.write(|connection| {
                    connection.exec("DELETE FROM items WHERE kind = 'Terminal'")?()
                })
                .await
                .unwrap();
                db.cleanup(workspace_id, vec![22]).await.unwrap();
                assert_eq!(db.item_ids(workspace_id).unwrap(), vec![22]);
                db.write(|connection| connection.exec("DELETE FROM kv_store")?())
                    .await
                    .unwrap();
                db.cleanup(workspace_id, Vec::new()).await.unwrap();
                assert_eq!(db.item_ids(workspace_id).unwrap(), Vec::<ItemId>::new());
            }
        }
    }

    #[gpui::test]
    async fn test_terminal_cleanup_protects_unpublished_and_later_payloads(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window_handle, panel) = init_workspace_with_panel(cx).await;
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        workspace.update(cx, |workspace, _| workspace.set_restoring_workspace(true));
        let workspace_id = initialize_terminal_persistence(&workspace, &[22, 33], cx).await;
        let db = cx.update(|cx| TerminalDb::global(cx));
        cx.update(|cx| KeyValueStore::global(cx))
            .write_kvp(
                TerminalPanel::serialization_key_for_workspace_id(workspace_id),
                String::from(r#"{"items":[22],"active_item_id":22}"#),
            )
            .await
            .unwrap();
        let historical = cx.new(|_| ());
        let historical_id = workspace.update(cx, |workspace, cx| {
            workspace
                .serialization_id("Terminal", historical.entity_id(), cx)
                .unwrap()
        });
        db.save_terminal(historical_id, workspace_id, None, None)
            .await
            .unwrap();
        drop(historical);
        let (earlier, earlier_id) = window_handle
            .update(cx, |_, window, cx| {
                let pane = workspace.read(cx).active_pane().clone();
                add_panel_display_terminal(&workspace, &pane, "unpublished", window, cx)
            })
            .unwrap();
        let earlier_write = db.save_terminal(earlier_id, workspace_id, None, None);
        let cleanup = panel
            .update(cx, |panel, cx| panel.cleanup(workspace_id, vec![22], cx))
            .unwrap();
        let later = cx.new(|_| ());
        let (later_id, later_write) = workspace.update(cx, |workspace, cx| {
            let item_id = workspace
                .serialization_id("Terminal", later.entity_id(), cx)
                .unwrap();
            (item_id, db.save_terminal(item_id, workspace_id, None, None))
        });
        later_write.await.unwrap();
        cleanup.await.unwrap();
        earlier_write.await.unwrap();
        let mut expected = vec![22, earlier_id, later_id];
        expected.sort_unstable();
        assert_eq!(db.item_ids(workspace_id).unwrap(), expected);
        assert_eq!(
            earlier.read_with(cx, |terminal, _| terminal.serialization_identity()),
            Some((workspace_id, earlier_id))
        );
    }

    #[gpui::test]
    async fn test_terminal_serialization_completes_and_retries_without_foreground_tasks(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window_handle, _) = init_workspace_with_panel(cx).await;
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        workspace.update(cx, |workspace, _| workspace.set_restoring_workspace(true));
        let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);
        open_center_display_terminal(&workspace, cx).await;
        let (terminal_view, item_id) = workspace.update(cx, |workspace, cx| {
            let terminal_view = workspace
                .active_item(cx)
                .unwrap()
                .downcast::<TerminalView>()
                .unwrap();
            let item_id = workspace
                .serialization_id("Terminal", terminal_view.entity_id(), cx)
                .unwrap();
            (terminal_view, item_id)
        });
        let db = cx.update(|_, cx| TerminalDb::global(cx));
        for (title, fail) in [
            ("first", false),
            ("first", false),
            ("retry", true),
            ("retry", false),
        ] {
            db.write(move |connection| {
                connection.exec("DROP TRIGGER IF EXISTS fail_terminal_payload")?()?;
                if fail {
                    connection.exec(
                        "CREATE TRIGGER fail_terminal_payload BEFORE INSERT ON terminals
                         BEGIN SELECT RAISE(FAIL, 'terminal payload failure'); END",
                    )?()?;
                }
                anyhow::Ok(())
            })
            .await
            .unwrap();
            terminal_view.update(cx, |terminal_view, cx| {
                terminal_view.set_custom_title(Some(String::from(title)), cx);
            });
            let results = workspace.update(cx, |workspace, cx| {
                let tasks = [false, true].map(|closing| {
                    terminal_view.update(cx, |terminal_view, cx| {
                        terminal_view
                            .serialize(workspace, item_id, closing, cx)
                            .unwrap()
                    })
                });
                cx.foreground_executor()
                    .block_with_timeout(Duration::from_secs(1), join_all(tasks))
                    .unwrap_or_else(|_| panic!("terminal serialization requires foreground work"))
            });
            for result in results {
                if fail {
                    assert!(result.is_err());
                } else {
                    result.unwrap();
                }
            }
            if fail {
                assert!(terminal_view.read_with(cx, |terminal_view, _| {
                    terminal_view.should_serialize(&ItemEvent::UpdateTab)
                }));
                assert_eq!(
                    db.get_custom_title(item_id, workspace_id)
                        .unwrap()
                        .as_deref(),
                    Some("first")
                );
            } else {
                assert_eq!(
                    db.get_custom_title(item_id, workspace_id)
                        .unwrap()
                        .as_deref(),
                    Some(title)
                );
            }
        }
    }

    #[gpui::test]
    async fn test_terminal_cleanup_preserves_rows_when_panel_graph_is_unreadable(
        cx: &mut TestAppContext,
    ) {
        let db = cx.update(|cx| {
            cx.set_global(AppDatabase::test_new());
            TerminalDb::global(cx)
        });
        let workspace_id = WorkspaceId::from_i64(1);
        db.write(move |connection| {
            connection.exec("INSERT INTO workspaces (workspace_id) VALUES (1)")?()?;
            connection
                .exec_bound::<(String, &str)>("INSERT INTO kv_store (key, value) VALUES (?, ?)")?(
                (
                TerminalPanel::serialization_key_for_workspace_id(workspace_id),
                "{",
            )
            )
        })
        .await
        .unwrap();
        db.save_terminal(11, workspace_id, None, None)
            .await
            .unwrap();
        assert!(db.cleanup(workspace_id, Vec::new()).await.is_err());
        assert_eq!(db.item_ids(workspace_id).unwrap(), vec![11]);
    }

    #[gpui::test]
    async fn test_panel_serialization_counts_only_retained_pinned_terminals(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window_handle, panel) = init_workspace_with_panel(cx).await;
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        initialize_terminal_persistence(&workspace, &[], cx).await;
        workspace.update(cx, |workspace, _| workspace.set_restoring_workspace(true));
        let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());
        let pane = panel.read_with(cx, |panel, _| panel.active_pane.clone());
        let mut retained_ids = Vec::new();
        for (index, is_task) in [true, false, true, false].into_iter().enumerate() {
            if is_task {
                let terminal = project
                    .update(cx, |project, cx| {
                        project.create_terminal_task(echo_task(), cx)
                    })
                    .await
                    .unwrap();
                window_handle
                    .update(cx, |_, window, cx| {
                        let view = cx.new(|cx| {
                            TerminalView::new(
                                terminal,
                                workspace.downgrade(),
                                project.downgrade(),
                                window,
                                cx,
                            )
                        });
                        pane.update(cx, |pane, cx| {
                            pane.add_item(Box::new(view), true, false, None, window, cx)
                        });
                    })
                    .unwrap();
            } else {
                let item_id = window_handle
                    .update(cx, |_, window, cx| {
                        add_panel_display_terminal(
                            &workspace,
                            &pane,
                            &format!("shell-{index}"),
                            window,
                            cx,
                        )
                        .1
                    })
                    .unwrap();
                retained_ids.push(item_id);
            }
        }
        for (pinned_count, expected_pinned) in [(0, 0), (1, 0), (2, 1), (3, 1), (4, 2)] {
            window_handle
                .update(cx, |_, window, cx| {
                    pane.update(cx, |pane, cx| {
                        pane.set_pinned_count(pinned_count);
                        pane.activate_item(2, false, false, window, cx);
                    });
                })
                .unwrap();
            let (serialized, tasks) = workspace.update(cx, |workspace, cx| {
                panel.update(cx, |panel, cx| {
                    let workspace_id = workspace.database_id().unwrap();
                    let admission = panel
                        .validate_serialized_item_ids(workspace, workspace_id, cx)
                        .unwrap();
                    serialize_pane_group(
                        &panel.center,
                        &panel.active_pane,
                        workspace,
                        &admission,
                        cx,
                    )
                    .unwrap()
                })
            });
            for result in join_all(tasks).await {
                result.unwrap();
            }
            let SerializedPaneGroup::Pane(serialized) = serialized else {
                panic!("expected a single pane");
            };
            assert_eq!(serialized.children, retained_ids);
            assert_eq!(serialized.pinned_count, expected_pinned);
            assert_eq!(serialized.active_item, None);
        }
    }

    #[gpui::test]
    async fn test_panel_recovery_merge_is_stable_and_reconciliation_is_atomic(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let db = cx.update(|cx| TerminalDb::global(cx));
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let workspace_id = WorkspaceId::from_i64(1);
        db.write(|connection| {
            connection.exec("INSERT INTO workspaces (workspace_id) VALUES (1)")?()
        })
        .await
        .unwrap();
        db.save_terminal(11, workspace_id, None, Some(String::from("saved")))
            .await
            .unwrap();
        db.save_terminal(22, workspace_id, None, Some(String::from("new")))
            .await
            .unwrap();
        let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
        let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
        kvp.write_kvp(key.clone(), String::from("{")).await.unwrap();
        let previous = r#"{"items":{"Pane":{"active":true,"children":[11],"active_item":11,"pinned_count":1}},"active_item_id":null}"#;
        let live = r#"{"items":{"Pane":{"active":true,"children":[22],"active_item":22,"pinned_count":0}},"active_item_id":null}"#;
        kvp.write_kvp(recovery_key.clone(), String::from(previous))
            .await
            .unwrap();
        let expected = serde_json::json!({
            "items": { "Group": {
                "axis": "horizontal", "flexes": null,
                "children": [
                    { "Pane": { "active": true, "children": [22], "active_item": 22, "pinned_count": 0 } },
                    { "Pane": { "active": true, "children": [11], "active_item": 11, "pinned_count": 1 } },
                ],
            } },
            "active_item_id": null,
        });
        for _ in 0..8 {
            db.save_panel(
                workspace_id,
                serde_json::from_str(live).unwrap(),
                true,
                false,
                HashSet::from_iter([22]),
            )
            .await
            .unwrap();
            assert_eq!(kvp.read_kvp(&key).unwrap().as_deref(), Some("{"));
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(
                    &kvp.read_kvp(&recovery_key).unwrap().unwrap()
                )
                .unwrap(),
                expected
            );
            assert!(db.cleanup(workspace_id, Vec::new()).await.is_err());
            assert_eq!(db.item_ids(workspace_id).unwrap(), vec![11, 22]);
        }
        kvp.write_kvp(key.clone(), String::from(previous))
            .await
            .unwrap();
        db.write(|connection| {
            connection.exec(
                "CREATE TRIGGER fail_recovery_reconciliation BEFORE DELETE ON kv_store
             BEGIN SELECT RAISE(FAIL, 'recovery reconciliation failure'); END",
            )?()
        })
        .await
        .unwrap();
        assert!(
            db.save_panel(
                workspace_id,
                serde_json::from_value(expected.clone()).unwrap(),
                false,
                true,
                HashSet::default(),
            )
            .await
            .is_err()
        );
        assert_eq!(kvp.read_kvp(&key).unwrap().as_deref(), Some(previous));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(
                &kvp.read_kvp(&recovery_key).unwrap().unwrap()
            )
            .unwrap(),
            expected
        );
        db.write(|connection| connection.exec("DROP TRIGGER fail_recovery_reconciliation")?())
            .await
            .unwrap();
        db.save_panel(
            workspace_id,
            serde_json::from_value(expected.clone()).unwrap(),
            false,
            true,
            HashSet::default(),
        )
        .await
        .unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&kvp.read_kvp(&key).unwrap().unwrap())
                .unwrap(),
            expected
        );
        assert_eq!(kvp.read_kvp(&recovery_key).unwrap(), None);
        assert_eq!(
            db.get_terminal(11, workspace_id).unwrap(),
            (None, Some(String::from("saved")))
        );
        assert_eq!(
            db.get_terminal(22, workspace_id).unwrap(),
            (None, Some(String::from("new")))
        );
    }

    #[gpui::test]
    async fn test_panel_load_failure_preserves_saved_and_new_terminals_through_reopen(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        cx.executor().set_block_on_ticks(10_000..=10_000);
        init_test(cx);
        for (saved, failed_item, failed_read) in [
            ("{\n  \"items\": [121, 122]", None, false),
            (r#"{"items":[121,122],"active_item_id":122}"#, None, true),
            (
                r#"{"items":[121,122],"active_item_id":122}"#,
                Some(121),
                false,
            ),
            (
                r#"{"items":[121,122],"active_item_id":122}"#,
                Some(122),
                false,
            ),
            (
                r#"{"items":{"Pane":{"active":true,"children":[121,122],"active_item":122,"pinned_count":1}},"active_item_id":null}"#,
                Some(121),
                false,
            ),
            (
                r#"{"items":{"Pane":{"active":true,"children":[121,122],"active_item":122,"pinned_count":1}},"active_item_id":null}"#,
                Some(122),
                false,
            ),
        ] {
            let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
            let window_handle =
                cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
            let workspace = window_handle
                .update(cx, |multi_workspace, _, _| {
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            let workspace_id = initialize_terminal_persistence(&workspace, &[121, 122], cx).await;
            let db = cx.update(|cx| TerminalDb::global(cx));
            if let Some(item_id) = failed_item {
                db.write(move |connection| {
                    connection.exec_bound::<(WorkspaceId, ItemId)>(
                        "UPDATE terminals SET custom_title = CAST(X'ff' AS TEXT) WHERE workspace_id = ? AND item_id = ?",
                    )?((workspace_id, item_id))
                }).await.unwrap();
            }
            let kvp = cx.update(|cx| KeyValueStore::global(cx));
            let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
            kvp.write_kvp(key.clone(), String::from(saved))
                .await
                .unwrap();
            if failed_read {
                db.write(|connection| {
                    connection.exec("ALTER TABLE kv_store RENAME TO unavailable_kv_store")?()
                })
                .await
                .unwrap();
            }
            let panel = window_handle
                .update(cx, |_, window, cx| {
                    let workspace = workspace.downgrade();
                    window.spawn(cx, async move |cx| {
                        TerminalPanel::load(workspace, cx.clone()).await
                    })
                })
                .unwrap()
                .await
                .unwrap();
            panel
                .update(cx, |panel, _| {
                    std::mem::replace(&mut panel._restoration, Task::ready(()))
                })
                .await;
            if failed_read {
                db.write(|connection| {
                    connection.exec("ALTER TABLE unavailable_kv_store RENAME TO kv_store")?()
                })
                .await
                .unwrap();
            }
            panel.read_with(cx, |panel, cx| {
                assert!(!panel.restoring);
                assert_eq!(panel.restoration_error.is_some(), failed_item.is_none());
                let titles = panel
                    .center
                    .panes()
                    .into_iter()
                    .flat_map(|pane| {
                        pane.read(cx)
                            .items()
                            .map(|item| item.tab_content_text(0, cx).to_string())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    titles,
                    match failed_item {
                        Some(121) => vec![
                            String::from("Failed terminal 121"),
                            String::from("terminal-122")
                        ],
                        Some(122) => vec![
                            String::from("terminal-121"),
                            String::from("Failed terminal 122")
                        ],
                        _ => Vec::new(),
                    }
                );
                if saved.starts_with(r#"{"items":{"Pane"#) {
                    assert_eq!(panel.active_pane.read(cx).pinned_count(), 1);
                }
            });
            let new_id = window_handle
                .update(cx, |_, window, cx| {
                    let pane = panel.read(cx).active_pane.clone();
                    let (_, item_id) =
                        add_panel_display_terminal(&workspace, &pane, "interim", window, cx);
                    panel.update(cx, |panel, cx| panel.serialize(cx));
                    item_id
                })
                .unwrap();
            cx.executor().advance_clock(Duration::from_millis(100));
            cx.run_until_parked();
            cx.update(|cx| cx.shutdown());
            let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
            let expected = serde_json::json!({
                "items": { "Pane": {
                    "active": true,
                    "children": if failed_item.is_some() { vec![121, 122, new_id] } else { vec![new_id] },
                    "active_item": new_id,
                    "pinned_count": usize::from(saved.starts_with(r#"{"items":{"Pane"#)),
                } },
                "active_item_id": null,
            });
            if failed_item.is_some() {
                assert_eq!(
                    serde_json::from_str::<serde_json::Value>(
                        &kvp.read_kvp(&key).unwrap().unwrap()
                    )
                    .unwrap(),
                    expected
                );
                assert_eq!(kvp.read_kvp(&recovery_key).unwrap(), None);
            } else {
                assert_eq!(kvp.read_kvp(&key).unwrap(), Some(String::from(saved)));
                assert_eq!(
                    serde_json::from_str::<serde_json::Value>(
                        &kvp.read_kvp(&recovery_key).unwrap().unwrap()
                    )
                    .unwrap(),
                    expected
                );
            }
            assert_eq!(db.item_ids(workspace_id).unwrap(), vec![121, 122, new_id]);
            let payloads = db.select_bound::<WorkspaceId, (ItemId, Option<PathBuf>, Option<String>, String)>(
                "SELECT item_id, working_directory, working_directory_path, hex(custom_title) FROM terminals WHERE workspace_id = ? ORDER BY item_id",
            ).unwrap()(workspace_id).unwrap();
            let expected = vec![
                (
                    121,
                    None,
                    None,
                    String::from(if failed_item == Some(121) {
                        "FF"
                    } else {
                        "7465726D696E616C2D313231"
                    }),
                ),
                (
                    122,
                    None,
                    None,
                    String::from(if failed_item == Some(122) {
                        "FF"
                    } else {
                        "7465726D696E616C2D313232"
                    }),
                ),
                (new_id, None, None, String::from("696E746572696D")),
            ];
            assert_eq!(
                payloads
                    .iter()
                    .map(|(item_id, _, _, title)| (*item_id, title))
                    .collect::<Vec<_>>(),
                expected
                    .iter()
                    .map(|(item_id, _, _, title)| (*item_id, title))
                    .collect::<Vec<_>>(),
            );
            let preserved = |item_id: ItemId| {
                failed_item.is_none() || failed_item == Some(item_id) || item_id == new_id
            };
            assert_eq!(
                payloads
                    .into_iter()
                    .filter(|(item_id, _, _, _)| preserved(*item_id))
                    .collect::<Vec<_>>(),
                expected
                    .into_iter()
                    .filter(|(item_id, _, _, _)| preserved(*item_id))
                    .collect::<Vec<_>>(),
            );
            drop(panel);
            drop(workspace);
            let (_, _, reopened) = reopen_terminal_panel(workspace_id, cx).await;
            let expected_ids = if failed_item.is_some() || failed_read {
                vec![121, 122, new_id]
            } else {
                vec![new_id]
            };
            assert_eq!(panel_terminal_ids(&reopened, cx), expected_ids);
            reopened.read_with(cx, |panel, cx| {
                let failures = panel
                    .center
                    .panes()
                    .into_iter()
                    .flat_map(|pane| {
                        pane.read(cx)
                            .items_of_type::<TerminalView>()
                            .filter_map(|terminal| {
                                let terminal = terminal.read(cx);
                                terminal
                                    .restoration_error
                                    .as_ref()
                                    .map(|_| terminal.serialization_identity().unwrap().1)
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                assert_eq!(failures, failed_item.into_iter().collect::<Vec<_>>());
                assert!(!panel.restoring);
            });
            cx.update(|cx| cx.shutdown());
        }
    }

    #[gpui::test]
    async fn test_panel_retries_repaired_state_without_recreating_live_terminals(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        cx.executor().set_block_on_ticks(10_000..=10_000);
        init_test(cx);
        for split in [false, true] {
            for failure in ["missing", "payload", "graph", "query"] {
                for addition in ["before", "during", "after"] {
                    let (window, panel) = init_workspace_with_panel(cx).await;
                    let workspace = window
                        .update(cx, |multi_workspace, _, _| {
                            multi_workspace.workspace().clone()
                        })
                        .unwrap();
                    let workspace_id =
                        initialize_terminal_persistence(&workspace, &[121, 122], cx).await;
                    let db = cx.update(|cx| TerminalDb::global(cx));
                    let kvp = cx.update(|cx| KeyValueStore::global(cx));
                    let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
                    let saved = if split {
                        r#"{"items":{"Pane":{"active":true,"children":[121,122],"active_item":122,"pinned_count":1}},"active_item_id":null}"#
                    } else {
                        r#"{"items":[121,122],"active_item_id":122}"#
                    };
                    kvp.write_kvp(
                        key.clone(),
                        String::from(if failure == "graph" { "{" } else { saved }),
                    )
                    .await
                    .unwrap();
                    if failure == "missing" {
                        db.write(move |connection| {
                            connection.exec_bound::<WorkspaceId>(
                                "DELETE FROM terminals WHERE workspace_id = ? AND item_id = 122",
                            )?(workspace_id)
                        })
                        .await
                        .unwrap();
                        let error = window
                            .update(cx, |_, window, cx| {
                                TerminalView::deserialize(
                                    workspace.read(cx).project().clone(),
                                    workspace.downgrade(),
                                    workspace_id,
                                    122,
                                    window,
                                    cx,
                                )
                            })
                            .unwrap()
                            .await
                            .err()
                            .unwrap();
                        assert_eq!(error.to_string(), "Saved terminal 122 has no payload");
                    } else if failure == "payload" {
                        db.write(move |connection| connection.exec_bound::<WorkspaceId>(
                            "UPDATE terminals SET custom_title = CAST(X'ff' AS TEXT) WHERE workspace_id = ? AND item_id = 122",
                        )?(workspace_id)).await.unwrap();
                    }
                    panel.update(cx, |panel, _| {
                        panel.primary_loaded = false;
                        panel.recovery_loaded = false;
                    });
                    let mut added = None;
                    if addition == "before" {
                        added = Some(
                            window
                                .update(cx, |_, window, cx| {
                                    let pane = panel.read(cx).active_pane.clone();
                                    add_panel_display_terminal(&workspace, &pane, "new", window, cx)
                                })
                                .unwrap(),
                        );
                    }
                    if failure == "query" {
                        workspace.update(cx, |workspace, cx| {
                            workspace
                                .reserve_serialized_item_ids(
                                    workspace_id,
                                    "Terminal",
                                    &[121, 122],
                                    cx,
                                )
                                .unwrap();
                        });
                        db.write(|connection| {
                            connection
                                .exec("ALTER TABLE kv_store RENAME TO unavailable_kv_store")?(
                            )
                        })
                        .await
                        .unwrap();
                    }
                    window
                        .update(cx, |_, window, cx| {
                            panel.update(cx, |panel, cx| panel.retry_restoration(window, cx));
                            if addition == "during" {
                                let pane = panel.read(cx).active_pane.clone();
                                added = Some(add_panel_display_terminal(
                                    &workspace, &pane, "new", window, cx,
                                ));
                            }
                        })
                        .unwrap();
                    panel
                        .update(cx, |panel, _| {
                            std::mem::replace(&mut panel._restoration, Task::ready(()))
                        })
                        .await;
                    if failure == "query" {
                        db.write(|connection| {
                            connection
                                .exec("ALTER TABLE unavailable_kv_store RENAME TO kv_store")?(
                            )
                        })
                        .await
                        .unwrap();
                    }
                    if addition == "after" {
                        added = Some(
                            window
                                .update(cx, |_, window, cx| {
                                    let pane = panel.read(cx).active_pane.clone();
                                    add_panel_display_terminal(&workspace, &pane, "new", window, cx)
                                })
                                .unwrap(),
                        );
                    }
                    let (added, added_id) = added.unwrap();
                    let decoded = failure == "missing" || failure == "payload";
                    let healthy = decoded.then(|| panel_terminal(&panel, 121, cx));
                    let failed = decoded.then(|| panel_terminal(&panel, 122, cx));
                    let failed_backing = failed
                        .as_ref()
                        .map(|failed| failed.read_with(cx, |view, _| view.terminal.clone()));
                    if let Some(failed) = &failed {
                        window
                            .update(cx, |_, window, cx| {
                                failed.update(cx, |failed, cx| failed.retry_restoration(window, cx))
                            })
                            .unwrap();
                        failed
                            .update(cx, |failed, _| failed.restoration_task.take())
                            .unwrap()
                            .await;
                        assert!(
                            failed.read_with(cx, |failed, _| failed.restoration_error.is_some())
                        );
                        assert_eq!(panel_terminal(&panel, 121, cx), healthy.clone().unwrap());
                        assert_eq!(panel_terminal(&panel, added_id, cx), added);
                    } else {
                        assert!(panel.read_with(cx, |panel, _| panel.restoration_error.is_some()));
                    }
                    if failure == "graph" {
                        let conflicting = serde_json::json!({ "items": [121, added_id], "active_item_id": added_id }).to_string();
                        kvp.write_kvp(key.clone(), conflicting.clone())
                            .await
                            .unwrap();
                        window
                            .update(cx, |_, window, cx| {
                                panel.update(cx, |panel, cx| panel.retry_restoration(window, cx))
                            })
                            .unwrap();
                        panel
                            .update(cx, |panel, _| {
                                std::mem::replace(&mut panel._restoration, Task::ready(()))
                            })
                            .await;
                        assert_eq!(
                            panel.read_with(cx, |panel, _| panel
                                .restoration_error
                                .as_ref()
                                .map(ToString::to_string)),
                            Some(format!(
                                "Saved layout: Saved layout references conflict with new terminal IDs [{added_id}]; repair the saved layout before retrying"
                            ))
                        );
                        assert_eq!(panel_terminal_ids(&panel, cx), vec![added_id]);
                        assert_eq!(kvp.read_kvp(&key).unwrap(), Some(conflicting));
                    }
                    let restored_second_id = if failure == "graph" {
                        added_id + 1
                    } else {
                        122
                    };
                    if decoded || failure == "graph" {
                        db.save_terminal(
                            restored_second_id,
                            workspace_id,
                            None,
                            Some(String::from("terminal-122")),
                        )
                        .await
                        .unwrap();
                    }
                    if !decoded {
                        kvp.write_kvp(
                            key.clone(),
                            saved.replace("122", &restored_second_id.to_string()),
                        )
                        .await
                        .unwrap();
                    }
                    if let Some(failed) = &failed {
                        window
                            .update(cx, |_, window, cx| {
                                failed.update(cx, |failed, cx| failed.retry_restoration(window, cx))
                            })
                            .unwrap();
                        failed
                            .update(cx, |failed, _| failed.restoration_task.take())
                            .unwrap()
                            .await;
                        assert!(
                            failed.read_with(cx, |failed, _| failed.restoration_error.is_none())
                        );
                        assert_eq!(panel_terminal(&panel, 122, cx), *failed);
                        assert_ne!(
                            failed.read_with(cx, |view, _| view.terminal.clone()),
                            failed_backing.unwrap()
                        );
                        assert_eq!(panel_terminal(&panel, 121, cx), healthy.unwrap());
                    } else {
                        window
                            .update(cx, |_, window, cx| {
                                panel.update(cx, |panel, cx| panel.retry_restoration(window, cx))
                            })
                            .unwrap();
                        panel
                            .update(cx, |panel, _| {
                                std::mem::replace(&mut panel._restoration, Task::ready(()))
                            })
                            .await;
                        assert!(panel.read_with(cx, |panel, _| panel.restoration_error.is_none()));
                    }
                    assert_eq!(panel_terminal(&panel, added_id, cx), added);
                    let mut expected_ids = vec![121, restored_second_id, added_id];
                    expected_ids.sort_unstable();
                    assert_eq!(panel_terminal_ids(&panel, cx), expected_ids);
                    let first_cwd = panel_terminal(&panel, 121, cx)
                        .read_with(cx, |view, cx| view.terminal().read(cx).working_directory());
                    let second_cwd = panel_terminal(&panel, restored_second_id, cx)
                        .read_with(cx, |view, cx| view.terminal().read(cx).working_directory());
                    cx.update(|cx| cx.shutdown());
                    assert_eq!(db.item_ids(workspace_id).unwrap(), expected_ids);
                    assert_eq!(saved_panel_terminal_ids(&kvp, &key), expected_ids);
                    assert_eq!(
                        kvp.read_kvp(&TerminalPanel::recovery_key_for_workspace_id(workspace_id))
                            .unwrap(),
                        None
                    );
                    assert_eq!(
                        db.get_terminal(121, workspace_id).unwrap(),
                        (first_cwd, Some(String::from("terminal-121")))
                    );
                    assert_eq!(
                        db.get_terminal(restored_second_id, workspace_id).unwrap(),
                        (second_cwd, Some(String::from("terminal-122")))
                    );
                    assert_eq!(
                        db.get_terminal(added_id, workspace_id).unwrap(),
                        (None, Some(String::from("new")))
                    );
                }
            }
        }
    }

    #[test]
    fn test_panel_recovery_merge_preserves_primary_tombstones() {
        for (current, previous, expected) in [
            (
                r#"{"items":[22],"active_item_id":22,"primary_item_ids":[11]}"#,
                r#"{"items":[11],"active_item_id":11,"primary_item_ids":[11]}"#,
                r#"{"items":[22],"active_item_id":22,"primary_item_ids":[11]}"#,
            ),
            (
                r#"{"items":[],"active_item_id":null,"primary_item_ids":[11]}"#,
                r#"{"items":[11],"active_item_id":11,"primary_item_ids":[11]}"#,
                r#"{"items":[],"active_item_id":null,"primary_item_ids":[11]}"#,
            ),
            (
                r#"{"items":[],"active_item_id":null}"#,
                r#"{"items":[11],"active_item_id":11,"primary_item_ids":[11]}"#,
                r#"{"items":[11],"active_item_id":11,"primary_item_ids":[11]}"#,
            ),
            (
                r#"{"items":[22],"active_item_id":22}"#,
                r#"{"items":[22],"active_item_id":22,"primary_item_ids":[11]}"#,
                r#"{"items":[22],"active_item_id":22,"primary_item_ids":[11]}"#,
            ),
        ] {
            let current = serde_json::from_str::<SerializedTerminalPanel>(current).unwrap();
            let previous = serde_json::from_str::<SerializedTerminalPanel>(previous).unwrap();
            assert_eq!(
                serde_json::to_value(current.merge(previous, &HashSet::from_iter([22])).unwrap())
                    .unwrap(),
                serde_json::from_str::<serde_json::Value>(expected).unwrap(),
            );
        }
    }

    #[test]
    fn test_panel_recovery_merge_discards_closed_live_items_and_rejects_unknown_overlap() {
        let previous = r#"{"items":[11,22],"active_item_id":22}"#;
        for current in [
            r#"{"items":[22],"active_item_id":22}"#,
            r#"{"items":[],"active_item_id":null}"#,
        ] {
            let merged = serde_json::from_str::<SerializedTerminalPanel>(current)
                .unwrap()
                .merge(
                    serde_json::from_str(previous).unwrap(),
                    &HashSet::from_iter([11, 22]),
                )
                .unwrap();
            assert_eq!(
                serde_json::to_value(merged).unwrap(),
                serde_json::from_str::<serde_json::Value>(current).unwrap()
            );
        }
        let current = r#"{"items":[22],"active_item_id":22}"#;
        let result = serde_json::from_str::<SerializedTerminalPanel>(current)
            .unwrap()
            .merge(serde_json::from_str(previous).unwrap(), &HashSet::default());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Recovery layout references conflict with new terminal IDs [22]; repair the saved references before retrying"
        );
    }

    #[gpui::test]
    async fn test_failed_terminal_save_close_and_recovery_tombstones(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        cx.executor().set_block_on_ticks(10_000..=10_000);
        init_test(cx);
        for missing_payload in [false, true] {
            for recovery in [false, true] {
                let (window, panel) = init_workspace_with_panel(cx).await;
                let workspace = window
                    .update(cx, |multi_workspace, _, _| {
                        multi_workspace.workspace().clone()
                    })
                    .unwrap();
                let workspace_id =
                    initialize_terminal_persistence(&workspace, &[121, 122], cx).await;
                let db = cx.update(|cx| TerminalDb::global(cx));
                let kvp = cx.update(|cx| KeyValueStore::global(cx));
                let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
                let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
                let saved = r#"{"items":[121,122],"active_item_id":121}"#;
                kvp.write_kvp(key.clone(), String::from(saved))
                    .await
                    .unwrap();
                db.write(move |connection| {
                    connection.exec_bound::<WorkspaceId>(if missing_payload {
                        "DELETE FROM terminals WHERE workspace_id = ? AND item_id = 121"
                    } else {
                        "UPDATE terminals SET custom_title = CAST(X'ff' AS TEXT) WHERE workspace_id = ? AND item_id = 121"
                    })?(workspace_id)
                }).await.unwrap();
                window
                    .update(cx, |_, window, cx| {
                        panel.update(cx, |panel, cx| {
                            panel.primary_loaded = false;
                            panel.recovery_loaded = false;
                            panel.retry_restoration(window, cx);
                        });
                    })
                    .unwrap();
                panel
                    .update(cx, |panel, _| {
                        std::mem::replace(&mut panel._restoration, Task::ready(()))
                    })
                    .await;
                let failed = panel_terminal(&panel, 121, cx);
                let pane = panel.read_with(cx, |panel, _| panel.active_pane.clone());
                failed.read_with(cx, |failed, cx| {
                    assert_eq!(failed.save_disposition(cx), SaveDisposition::DiscardOnly);
                    assert!(failed.is_dirty(cx));
                    assert!(!failed.can_split());
                    assert!(!failed.can_save(cx));
                    assert!(!failed.can_save_as(cx));
                });
                let publication = workspace.update(cx, |workspace, cx| {
                    failed.update(cx, |failed, cx| {
                        failed.serialize(workspace, 121, true, cx).unwrap()
                    })
                });
                publication.await.unwrap();
                assert_eq!(
                    db.select_row_bound::<WorkspaceId, String>(
                        "SELECT hex(custom_title) FROM terminals WHERE workspace_id = ? AND item_id = 121",
                    ).unwrap()(workspace_id).unwrap(),
                    (!missing_payload).then(|| String::from("FF")),
                );
                workspace.update(cx, |workspace, _| {
                    workspace.set_restoring_workspace(recovery)
                });
                {
                    let cx = &mut VisualTestContext::from_window(window.into(), cx);
                    for intent in [SaveIntent::Save, SaveIntent::SaveAs, SaveIntent::SaveAll] {
                        let closing = pane.update_in(cx, |pane, window, cx| {
                            pane.close_item_by_id(failed.entity_id(), intent, window, cx)
                        });
                        cx.run_until_parked();
                        assert!(cx.has_pending_prompt());
                        cx.simulate_prompt_answer("OK");
                        closing.await.unwrap();
                        assert_eq!(pane.read_with(cx, |pane, _| pane.items_len()), 2);
                    }
                    for answer in ["Cancel", "Discard"] {
                        let closing = pane.update_in(cx, |pane, window, cx| {
                            pane.close_item_by_id(failed.entity_id(), SaveIntent::Close, window, cx)
                        });
                        cx.run_until_parked();
                        assert!(cx.has_pending_prompt());
                        cx.simulate_prompt_answer(answer);
                        closing.await.unwrap();
                        assert_eq!(
                            pane.read_with(cx, |pane, _| pane.items_len()),
                            if answer == "Cancel" { 2 } else { 1 }
                        );
                    }
                }
                workspace.update(cx, |workspace, _| {
                    workspace.set_restoring_workspace(recovery)
                });
                panel.update(cx, |panel, cx| {
                    panel.pending_serialization = Task::ready(None);
                    panel.serialize_now(cx);
                });
                panel
                    .update(cx, |panel, _| panel.pending_publication.take())
                    .unwrap()
                    .await
                    .unwrap();
                if recovery {
                    assert_eq!(saved_panel_terminal_ids(&kvp, &key), vec![121, 122]);
                    assert_eq!(saved_panel_terminal_ids(&kvp, &recovery_key), vec![122]);
                    let saved = serde_json::from_str::<serde_json::Value>(
                        &kvp.read_kvp(&recovery_key).unwrap().unwrap(),
                    )
                    .unwrap();
                    assert_eq!(saved["primary_item_ids"], serde_json::json!([121, 122]));
                } else {
                    assert_eq!(saved_panel_terminal_ids(&kvp, &key), vec![122]);
                    assert_eq!(kvp.read_kvp(&recovery_key).unwrap(), None);
                }
                cx.update(|cx| cx.shutdown());
                drop(failed);
                drop(panel);
                drop(workspace);
                let (_, _, reopened) = reopen_terminal_panel(workspace_id, cx).await;
                assert_eq!(panel_terminal_ids(&reopened, cx), vec![122]);
                cx.update(|cx| cx.shutdown());
                assert_eq!(saved_panel_terminal_ids(&kvp, &key), vec![122]);
                assert_eq!(kvp.read_kvp(&recovery_key).unwrap(), None);
            }
        }
    }

    #[gpui::test]
    async fn test_panel_unreadable_recovery_reports_unsaved_changes_without_overwrite(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, panel) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[121], cx).await;
        let db = cx.update(|cx| TerminalDb::global(cx));
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
        let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
        workspace.update(cx, |workspace, cx| {
            workspace
                .reserve_serialized_item_ids(workspace_id, "Terminal", &[121], cx)
                .unwrap();
        });
        for key in [&key, &recovery_key] {
            kvp.write_kvp(key.clone(), String::from("{")).await.unwrap();
        }
        window
            .update(cx, |_, window, cx| {
                panel.update(cx, |panel, cx| {
                    panel.primary_loaded = false;
                    panel.recovery_loaded = false;
                    panel.retry_restoration(window, cx);
                });
            })
            .unwrap();
        panel
            .update(cx, |panel, _| {
                std::mem::replace(&mut panel._restoration, Task::ready(()))
            })
            .await;
        let (live, live_id) = window
            .update(cx, |_, window, cx| {
                let pane = panel.read(cx).active_pane.clone();
                add_panel_display_terminal(&workspace, &pane, "unsaved", window, cx)
            })
            .unwrap();
        for _ in 0..3 {
            panel.update(cx, |panel, cx| {
                panel.pending_serialization = Task::ready(None);
                panel.serialize_now(cx);
            });
            panel
                .update(cx, |panel, _| panel.pending_publication.take())
                .unwrap()
                .await
                .unwrap_err();
            cx.run_until_parked();
            assert_eq!(
                panel.read_with(cx, |panel, _| panel
                    .publication_error
                    .as_ref()
                    .map(ToString::to_string)),
                Some(String::from(
                    "Cannot reserve terminal IDs because the recovery layout is unreadable: EOF while parsing an object at line 1 column 1"
                )),
            );
            assert_eq!(kvp.read_kvp(&key).unwrap().as_deref(), Some("{"));
            assert_eq!(kvp.read_kvp(&recovery_key).unwrap().as_deref(), Some("{"));
            assert_eq!(db.item_ids(workspace_id).unwrap(), vec![121]);
        }
        kvp.write_kvp(
            recovery_key.clone(),
            String::from(r#"{"items":[121],"active_item_id":121}"#),
        )
        .await
        .unwrap();
        window
            .update(cx, |_, window, cx| {
                panel.update(cx, |panel, cx| panel.retry_restoration(window, cx));
            })
            .unwrap();
        panel
            .update(cx, |panel, _| {
                std::mem::replace(&mut panel._restoration, Task::ready(()))
            })
            .await;
        panel.update(cx, |panel, cx| {
            panel.pending_serialization = Task::ready(None);
            panel.serialize_now(cx);
        });
        panel
            .update(cx, |panel, _| panel.pending_publication.take())
            .unwrap()
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(
            panel.read_with(cx, |panel, _| panel.publication_error.clone()),
            None
        );
        assert_eq!(panel_terminal(&panel, live_id, cx), live);
        assert_eq!(
            saved_panel_terminal_ids(&kvp, &recovery_key),
            vec![121, live_id]
        );
        assert_eq!(kvp.read_kvp(&key).unwrap().as_deref(), Some("{"));
    }

    #[gpui::test]
    async fn test_terminal_ids_require_a_readable_recovery_graph(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, _) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
        let db = cx.update(|cx| TerminalDb::global(cx));
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
        let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
        kvp.write_kvp(key, String::from("{")).await.unwrap();
        kvp.write_kvp(recovery_key.clone(), String::from("{"))
            .await
            .unwrap();
        let live = cx.new(|_| ());
        assert!(
            workspace
                .update(cx, |workspace, cx| workspace.serialization_id(
                    "Terminal",
                    live.entity_id(),
                    cx
                ))
                .is_err()
        );
        let saved_id = live.entity_id().as_u64();
        kvp.write_kvp(
            recovery_key,
            serde_json::json!({"items": [saved_id], "active_item_id": saved_id}).to_string(),
        )
        .await
        .unwrap();
        db.write(|connection| {
            connection.exec("ALTER TABLE kv_store RENAME TO unavailable_kv_store")?()
        })
        .await
        .unwrap();
        assert!(
            workspace
                .update(cx, |workspace, cx| workspace.serialization_id(
                    "Terminal",
                    live.entity_id(),
                    cx
                ))
                .is_err()
        );
        db.write(|connection| {
            connection.exec("ALTER TABLE unavailable_kv_store RENAME TO kv_store")?()
        })
        .await
        .unwrap();
        assert_eq!(
            db.serialized_item_ids(workspace_id).unwrap(),
            vec![saved_id]
        );
        let live_id = workspace
            .update(cx, |workspace, cx| {
                workspace.serialization_id("Terminal", live.entity_id(), cx)
            })
            .unwrap();
        assert!(live_id > saved_id);
        assert_eq!(db.item_ids(workspace_id).unwrap(), Vec::<ItemId>::new());
    }

    #[gpui::test]
    async fn test_panel_rejects_unloaded_recovery_id_collision(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, panel) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
        let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
        kvp.write_kvp(key.clone(), String::from("{")).await.unwrap();
        let (live, live_id) = window
            .update(cx, |_, window, cx| {
                let pane = panel.read(cx).active_pane.clone();
                let live = add_panel_display_terminal(&workspace, &pane, "new", window, cx);
                panel.update(cx, |panel, _| {
                    panel.primary_loaded = false;
                    panel.recovery_loaded = false;
                    panel.pending_serialization = Task::ready(None);
                });
                live
            })
            .unwrap();
        let saved = serde_json::json!({"items": [live_id], "active_item_id": live_id}).to_string();
        kvp.write_kvp(recovery_key.clone(), saved.clone())
            .await
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                panel.update(cx, |panel, cx| panel.retry_restoration(window, cx));
            })
            .unwrap();
        panel
            .update(cx, |panel, _| {
                std::mem::replace(&mut panel._restoration, Task::ready(()))
            })
            .await;
        assert!(!panel.read_with(cx, |panel, _| panel.recovery_loaded));
        assert_eq!(panel_terminal(&panel, live_id, cx), live);
        assert_eq!(panel_terminal_ids(&panel, cx), vec![live_id]);
        panel.update(cx, |panel, cx| {
            panel.pending_serialization = Task::ready(None);
            panel.serialize_now(cx);
        });
        panel
            .update(cx, |panel, _| panel.pending_publication.take())
            .unwrap()
            .await
            .unwrap_err();
        cx.run_until_parked();
        assert_eq!(
            panel.read_with(cx, |panel, _| panel
                .publication_error
                .as_ref()
                .map(ToString::to_string)),
            Some(format!(
                "Recovery layout references conflict with new terminal IDs [{live_id}]; repair the saved references before retrying"
            )),
        );
        assert_eq!(kvp.read_kvp(&recovery_key).unwrap(), Some(saved));
        assert_eq!(kvp.read_kvp(&key).unwrap().as_deref(), Some("{"));
    }

    #[gpui::test]
    async fn test_panel_recovery_does_not_resurrect_all_discarded_terminals(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        cx.executor().set_block_on_ticks(10_000..=10_000);
        init_test(cx);
        let db = cx.update(|cx| TerminalDb::global(cx));
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        let workspace_id = WorkspaceId::from_i64(1);
        db.write(|connection| {
            connection.exec("INSERT INTO workspaces (workspace_id) VALUES (1)")?()
        })
        .await
        .unwrap();
        db.save_terminal(121, workspace_id, None, Some(String::from("discarded")))
            .await
            .unwrap();
        let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
        let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
        kvp.write_kvp(
            key.clone(),
            String::from(r#"{"items":[121],"active_item_id":121}"#),
        )
        .await
        .unwrap();
        kvp.write_kvp(
            recovery_key.clone(),
            String::from(r#"{"items":[],"active_item_id":null,"primary_item_ids":[121]}"#),
        )
        .await
        .unwrap();
        let (_, _, panel) = reopen_terminal_panel(workspace_id, cx).await;
        assert_eq!(panel_terminal_ids(&panel, cx), Vec::<ItemId>::new());
        cx.update(|cx| cx.shutdown());
        assert_eq!(saved_panel_terminal_ids(&kvp, &key), Vec::<ItemId>::new());
        assert_eq!(kvp.read_kvp(&recovery_key).unwrap(), None);
        assert_eq!(db.item_ids(workspace_id).unwrap(), Vec::<ItemId>::new());
    }

    #[gpui::test]
    async fn test_failed_terminal_keeps_its_identity_when_moved(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        let (window, panel) = init_workspace_with_panel(cx).await;
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[121], cx).await;
        let failed = window
            .update(cx, |_, window, cx| {
                let pane = panel.read(cx).active_pane.clone();
                let failed = workspace.update(cx, |workspace, cx| {
                    let failed = cx.new(|cx| {
                        TerminalView::failed_restoration(
                            workspace.weak_handle(),
                            workspace.project().downgrade(),
                            workspace_id,
                            121,
                            anyhow!("terminal could not be restored"),
                            window,
                            cx,
                        )
                    });
                    workspace
                        .register_serialized_item_id("Terminal", failed.entity_id(), 121, cx)
                        .unwrap();
                    failed
                });
                pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(failed.clone()), true, false, None, window, cx)
                });
                let destination = workspace.read(cx).active_pane().clone();
                workspace::move_item(
                    &pane,
                    &destination,
                    failed.entity_id(),
                    0,
                    false,
                    window,
                    cx,
                );
                assert_eq!(pane.read(cx).items_len(), 0);
                assert_eq!(destination.read(cx).items_len(), 1);
                failed
            })
            .unwrap();
        workspace
            .update(cx, |workspace, cx| {
                failed.update(cx, |failed, cx| {
                    failed.serialize(workspace, 121, false, cx).unwrap()
                })
            })
            .await
            .unwrap();
        assert_eq!(
            failed.read_with(cx, |failed, _| failed.serialization_identity()),
            Some((workspace_id, 121))
        );
        let (other_window, _) = init_workspace_with_panel(cx).await;
        let other_workspace = other_window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let other_id = initialize_terminal_persistence(&other_workspace, &[], cx).await;
        let result = other_window
            .update(cx, |_, window, cx| {
                other_workspace.update(cx, |workspace, cx| {
                    failed.update(cx, |failed, cx| {
                        failed.added_to_workspace(workspace, window, cx);
                        failed.serialize(workspace, 121, false, cx).unwrap()
                    })
                })
            })
            .unwrap()
            .await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "Retry this failed terminal in its original workspace before moving it"
        );
        assert_eq!(
            failed.read_with(cx, |failed, _| failed.serialization_identity()),
            Some((workspace_id, 121))
        );
        let db = cx.update(|cx| TerminalDb::global(cx));
        assert_eq!(db.item_ids(workspace_id).unwrap(), vec![121]);
        assert_eq!(db.item_ids(other_id).unwrap(), Vec::<ItemId>::new());
    }

    #[gpui::test]
    async fn test_panel_restores_and_serializes_saved_active_terminal_id(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init_test(cx);
        for items in [
            SerializedItems::NoSplits(vec![121, 122]),
            SerializedItems::WithSplits(SerializedPaneGroup::Pane(SerializedPane {
                active: true,
                children: vec![121, 122],
                active_item: Some(122),
                pinned_count: 0,
            })),
        ] {
            let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
            let window_handle =
                cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
            let workspace = window_handle
                .update(cx, |multi_workspace, _, _| {
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            let workspace_id = initialize_terminal_persistence(&workspace, &[121, 122], cx).await;
            let panel = window_handle
                .update(cx, |_, window, cx| {
                    workspace.update(cx, |workspace, cx| {
                        cx.new(|cx| {
                            let mut panel = TerminalPanel::new(workspace, window, cx);
                            panel.restoring = true;
                            panel
                        })
                    })
                })
                .unwrap();
            let restored = window_handle
                .update(cx, |_, window, cx| {
                    deserialize_terminal_panel(
                        workspace.downgrade(),
                        workspace.read(cx).project().clone(),
                        workspace_id,
                        SerializedTerminalPanel {
                            items,
                            active_item_id: Some(122),
                            primary_item_ids: Vec::new(),
                        },
                        panel.downgrade(),
                        window,
                        cx,
                    )
                })
                .unwrap()
                .await
                .unwrap();
            assert_eq!(restored, 2);
            panel.read_with(cx, |panel, cx| {
                let active = panel.active_pane.read(cx).active_item().unwrap();
                assert_eq!(active.tab_content_text(0, cx).as_ref(), "terminal-122");
                assert_ne!(active.item_id().as_u64(), 122);
            });
            let (serialized, tasks) = workspace.update(cx, |workspace, cx| {
                panel.update(cx, |panel, cx| {
                    let workspace_id = workspace.database_id().unwrap();
                    let admission = panel
                        .validate_serialized_item_ids(workspace, workspace_id, cx)
                        .unwrap();
                    serialize_pane_group(
                        &panel.center,
                        &panel.active_pane,
                        workspace,
                        &admission,
                        cx,
                    )
                    .unwrap()
                })
            });
            for result in join_all(tasks).await {
                result.unwrap();
            }
            let SerializedPaneGroup::Pane(serialized) = serialized else {
                panic!("expected a single restored pane");
            };
            assert_eq!(serialized.children, vec![121, 122]);
            assert_eq!(serialized.active_item, Some(122));
            let db = cx.update(|cx| TerminalDb::global(cx));
            assert_eq!(db.item_ids(workspace_id).unwrap(), vec![121, 122]);
        }
    }

    #[gpui::test]
    async fn test_panel_shutdown_captures_debounced_graph_and_payloads(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        cx.executor().set_block_on_ticks(10_000..=10_000);
        init_test(cx);
        for prior_publication in [false, true] {
            let (window_handle, panel) = init_workspace_with_panel(cx).await;
            let workspace = window_handle
                .update(cx, |multi_workspace, _, _| {
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            workspace.update(cx, |workspace, _| workspace.set_restoring_workspace(true));
            let workspace_id = initialize_terminal_persistence(&workspace, &[11, 22], cx).await;
            let db = cx.update(|cx| TerminalDb::global(cx));
            db.write(move |connection| {
                connection.exec_bound::<WorkspaceId>(
                    "INSERT INTO panes (workspace_id, active) VALUES (?, 1)",
                )?(workspace_id)?;
                connection.exec_bound::<WorkspaceId>(
                    "INSERT INTO items (item_id, workspace_id, pane_id, kind, position, active)
                     VALUES (22, ?, last_insert_rowid(), 'Terminal', 0, 1)",
                )?(workspace_id)
            })
            .await
            .unwrap();
            let kvp = cx.update(|cx| KeyValueStore::global(cx));
            let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
            let saved = String::from(r#"{"items":[11],"active_item_id":11}"#);
            kvp.write_kvp(key.clone(), saved.clone()).await.unwrap();
            cx.run_until_parked();

            let (first_id, second_id, third_id) = window_handle
                .update(cx, |_, window, cx| {
                    workspace.update(cx, |workspace, _| workspace.set_restoring_workspace(false));
                    let first_pane = panel.read(cx).active_pane.clone();
                    let (first, first_id) =
                        add_panel_display_terminal(&workspace, &first_pane, "before", window, cx);
                    if prior_publication {
                        panel.update(cx, |panel, cx| {
                            panel.serialize_now(cx);
                            assert!(panel.pending_publication.is_some());
                        });
                    }
                    first.update(cx, |terminal, cx| {
                        terminal.set_custom_title(Some(String::from("first")), cx);
                    });
                    let (_, second_id) =
                        add_panel_display_terminal(&workspace, &first_pane, "second", window, cx);
                    first_pane.update(cx, |pane, cx| {
                        pane.set_pinned_count(1);
                        pane.activate_item(0, false, false, window, cx);
                    });
                    let second_pane = panel.update(cx, |panel, cx| {
                        new_terminal_pane(
                            panel.workspace.clone(),
                            workspace.read(cx).project().clone(),
                            false,
                            window,
                            cx,
                        )
                    });
                    let (_, third_id) =
                        add_panel_display_terminal(&workspace, &second_pane, "third", window, cx);
                    panel.update(cx, |panel, cx| {
                        panel
                            .center
                            .split(&first_pane, &second_pane, SplitDirection::Right, cx);
                        let workspace::Member::Axis(axis) = &panel.center.root else {
                            panic!("expected a split panel");
                        };
                        *axis.flexes.lock() = vec![0.75, 1.25];
                        panel.active_pane = second_pane;
                        panel.serialize(cx);
                    });
                    (first_id, second_id, third_id)
                })
                .unwrap();
            assert_eq!(kvp.read_kvp(&key).unwrap(), Some(saved));
            let weak_panel = panel.downgrade();
            let weak_workspace = workspace.downgrade();
            drop(panel);
            drop(workspace);
            let foreground_ran = Arc::new(AtomicBool::new(false));
            cx.update(|cx| {
                cx.spawn({
                    let foreground_ran = foreground_ran.clone();
                    async move |_| foreground_ran.store(true, Ordering::Relaxed)
                })
                .detach();
                cx.shutdown();
            });
            assert!(!foreground_ran.load(Ordering::Relaxed));
            assert!(weak_panel.upgrade().is_none());
            assert!(weak_workspace.upgrade().is_none());
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&kvp.read_kvp(&key).unwrap().unwrap())
                    .unwrap(),
                serde_json::json!({
                    "items": {
                        "Group": {
                            "axis": "horizontal",
                            "flexes": [0.75, 1.25],
                            "children": [
                                {"Pane": {
                                    "active": false,
                                    "children": [first_id, second_id],
                                    "active_item": first_id,
                                    "pinned_count": 1
                                }},
                                {"Pane": {
                                    "active": true,
                                    "children": [third_id],
                                    "active_item": third_id,
                                    "pinned_count": 0
                                }}
                            ]
                        }
                    },
                    "active_item_id": null
                })
            );
            let mut expected_ids = vec![22, first_id, second_id, third_id];
            expected_ids.sort_unstable();
            assert_eq!(db.item_ids(workspace_id).unwrap(), expected_ids);
            for (item_id, title) in [
                (22, "terminal-22"),
                (first_id, "first"),
                (second_id, "second"),
                (third_id, "third"),
            ] {
                assert_eq!(
                    db.select_row_bound::<
                        (ItemId, WorkspaceId),
                        (Option<PathBuf>, Option<String>, Option<String>),
                    >(
                        "SELECT working_directory, working_directory_path, custom_title
                         FROM terminals WHERE item_id = ? AND workspace_id = ?",
                    )
                    .unwrap()((item_id, workspace_id))
                    .unwrap(),
                    Some((None, None, Some(String::from(title))))
                );
            }
        }
    }

    #[gpui::test]
    async fn test_panel_shutdown_preserves_state_during_restoration(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        cx.executor().set_block_on_ticks(10_000..=10_000);
        init_test(cx);
        for (panel_restoring, workspace_restoring) in [(true, false), (false, true), (true, true)] {
            let (window_handle, panel) = init_workspace_with_panel(cx).await;
            let workspace = window_handle
                .update(cx, |multi_workspace, _, _| {
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            workspace.update(cx, |workspace, _| workspace.set_restoring_workspace(true));
            let workspace_id = initialize_terminal_persistence(&workspace, &[11], cx).await;
            let db = cx.update(|cx| TerminalDb::global(cx));
            let kvp = cx.update(|cx| KeyValueStore::global(cx));
            let key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
            let saved = String::from(r#"{"items":[11],"active_item_id":11}"#);
            kvp.write_kvp(key.clone(), saved.clone()).await.unwrap();
            cx.run_until_parked();
            let new_id = window_handle
                .update(cx, |_, window, cx| {
                    let pane = panel.read(cx).active_pane.clone();
                    let (_, new_id) =
                        add_panel_display_terminal(&workspace, &pane, "interim", window, cx);
                    panel.update(cx, |panel, cx| {
                        panel.serialize(cx);
                        panel.restoring = panel_restoring;
                    });
                    workspace.update(cx, |workspace, _| {
                        workspace.set_restoring_workspace(workspace_restoring)
                    });
                    new_id
                })
                .unwrap();
            cx.update(|cx| cx.shutdown());
            assert_eq!(kvp.read_kvp(&key).unwrap(), Some(saved));
            assert_eq!(
                saved_panel_terminal_ids(
                    &kvp,
                    &TerminalPanel::recovery_key_for_workspace_id(workspace_id)
                ),
                vec![new_id]
            );
            assert_eq!(db.item_ids(workspace_id).unwrap(), vec![11, new_id]);
            assert_eq!(
                db.get_terminal(new_id, workspace_id).unwrap(),
                (None, Some(String::from("interim")))
            );
            assert_eq!(
                db.get_custom_title(11, workspace_id).unwrap().as_deref(),
                Some("terminal-11")
            );
            assert!(panel.read_with(cx, |panel, _| {
                panel.needs_cleanup.load(Ordering::Relaxed)
            }));
            drop(panel);
            drop(workspace);
            let (_, _, reopened) = reopen_terminal_panel(workspace_id, cx).await;
            assert_eq!(panel_terminal_ids(&reopened, cx), vec![11, new_id]);
            cx.update(|cx| cx.shutdown());
        }
    }

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
    async fn test_empty_inactive_panel_restores_without_spawning_shell(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings.terminal.get_or_insert_default().project.shell = Some(
                    settings::Shell::Program(String::from("__nonexistent_shell__")),
                );
            });
        });
        for recovery in [false, true] {
            for items in [
                serde_json::json!([]),
                serde_json::json!({"Pane": {
                    "active": true, "children": [], "active_item": null
                }}),
                serde_json::json!({"Group": {
                    "axis": "horizontal", "flexes": [0.5, 0.5], "children": [
                        {"Pane": {"active": false, "children": [], "active_item": null}},
                        {"Group": {"axis": "vertical", "flexes": null, "children": [
                            {"Pane": {"active": true, "children": [], "active_item": null}}
                        ]}}
                    ]
                }}),
            ] {
                let (window, panel) = init_workspace_with_panel(cx).await;
                let workspace = window
                    .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
                    .expect("workspace window");
                let workspace_id = initialize_terminal_persistence(&workspace, &[], cx).await;
                let kvp = cx.read(|cx| KeyValueStore::global(cx));
                let key = if recovery {
                    TerminalPanel::recovery_key_for_workspace_id(workspace_id)
                } else {
                    TerminalPanel::serialization_key_for_workspace_id(workspace_id)
                };
                let saved = serde_json::json!({
                    "items": items, "active_item_id": null, "primary_item_ids": []
                })
                .to_string();
                kvp.write_kvp(key.clone(), saved.clone())
                    .await
                    .expect("save empty graph");
                window
                    .update(cx, |_, window, cx| {
                        panel.update(cx, |panel, cx| {
                            assert!(!panel.active);
                            panel.primary_loaded = false;
                            panel.recovery_loaded = false;
                            panel.retry_restoration(window, cx);
                        });
                    })
                    .expect("restore empty panel");
                panel
                    .update(cx, |panel, _| {
                        std::mem::replace(&mut panel._restoration, Task::ready(()))
                    })
                    .await;
                cx.run_until_parked();
                panel.read_with(cx, |panel, cx| {
                    assert!(!panel.active);
                    assert!(!panel.restoring);
                    assert_eq!(panel.restoration_error, None);
                    assert!(panel.primary_loaded);
                    assert!(panel.recovery_loaded);
                    assert_eq!(panel.pending_terminals_to_add, 0);
                    assert_eq!(
                        panel
                            .center
                            .panes()
                            .into_iter()
                            .map(|pane| pane.read(cx).items_len())
                            .collect::<Vec<_>>(),
                        vec![0]
                    );
                });
                assert_eq!(kvp.read_kvp(&key).expect("read empty graph"), Some(saved));
            }
        }
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
    async fn test_terminal_added_during_restore_is_serialized_after_restore(
        cx: &mut TestAppContext,
    ) {
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
        initialize_terminal_persistence(&workspace, &[], cx).await;

        let serialization_key = window_handle
            .update(cx, |multi_workspace, _, cx| {
                TerminalPanel::serialization_key(multi_workspace.workspace().read(cx)).unwrap()
            })
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

        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        let suppressed_state = cx
            .update(|cx| KeyValueStore::global(cx))
            .read_kvp(&serialization_key)
            .unwrap();
        assert_eq!(
            suppressed_state, None,
            "the primary graph must not change while the panel is restoring"
        );
        let db = cx.update(|cx| TerminalDb::global(cx));
        db.write(|connection| {
            connection.exec(
                "CREATE TRIGGER fail_terminal_payload BEFORE INSERT ON terminals
                 BEGIN SELECT RAISE(FAIL, 'terminal payload failure'); END",
            )?()
        })
        .await
        .unwrap();
        let terminal_view = terminal_panel.read_with(cx, |panel, cx| {
            panel
                .active_pane
                .read(cx)
                .active_item()
                .unwrap()
                .downcast::<TerminalView>()
                .unwrap()
        });
        let (workspace_id, item_id) =
            terminal_view.read_with(cx, |view, _| view.serialization_identity().unwrap());
        assert_eq!(
            saved_panel_terminal_ids(
                &cx.update(|cx| KeyValueStore::global(cx)),
                &TerminalPanel::recovery_key_for_workspace_id(workspace_id)
            ),
            vec![item_id]
        );
        assert_eq!(db.item_ids(workspace_id).unwrap(), vec![item_id]);
        terminal_view.update(cx, |terminal_view, cx| {
            terminal_view.set_custom_title(Some(String::from("during-restore")), cx);
        });

        let default_shell_task = window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |terminal_panel, cx| {
                    terminal_panel.finish_restoration(false, window, cx)
                })
            })
            .unwrap();
        assert!(
            default_shell_task.is_none(),
            "no default terminal should spawn when a terminal already exists"
        );

        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        let failed_state = cx
            .update(|cx| KeyValueStore::global(cx))
            .read_kvp(&serialization_key)
            .unwrap();
        assert_eq!(failed_state, None);
        assert!(terminal_panel.read_with(cx, |panel, _| panel.publication_error.is_some()));
        assert!(terminal_panel.read_with(cx, |panel, _| {
            panel.needs_cleanup.load(Ordering::Relaxed)
        }));
        db.write(|connection| connection.exec("DROP TRIGGER fail_terminal_payload")?())
            .await
            .unwrap();
        window_handle
            .update(cx, |_, window, cx| {
                terminal_panel.update(cx, |panel, cx| panel.retry_restoration(window, cx))
            })
            .unwrap();
        terminal_panel
            .update(cx, |panel, _| {
                std::mem::replace(&mut panel._restoration, Task::ready(()))
            })
            .await;
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        assert!(terminal_panel.read_with(cx, |panel, _| panel.publication_error.is_none()));
        let serialized_state = cx
            .update(|cx| KeyValueStore::global(cx))
            .read_kvp(&serialization_key)
            .unwrap();
        assert!(
            serialized_state.is_some(),
            "terminal added during restore must be serialized once its payload succeeds"
        );
        assert!(!terminal_panel.read_with(cx, |panel, _| {
            panel.needs_cleanup.load(Ordering::Relaxed)
        }));
    }

    #[gpui::test]
    async fn test_legacy_serialized_restore_keeps_interim_terminal_active(cx: &mut TestAppContext) {
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
        let workspace_id = initialize_terminal_persistence(&workspace, &[12345], cx).await;

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| {
                        let mut panel = TerminalPanel::new(workspace, window, cx);
                        panel.restoring = true;
                        panel
                    })
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
                    workspace_id,
                    SerializedTerminalPanel {
                        items: SerializedItems::NoSplits(vec![12345]),
                        active_item_id: Some(12345),
                        primary_item_ids: Vec::new(),
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
        let workspace = window_handle
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_id = initialize_terminal_persistence(&workspace, &[12345], cx).await;

        let terminal_panel = window_handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    cx.new(|cx| {
                        let mut panel = TerminalPanel::new(workspace, window, cx);
                        panel.restoring = true;
                        panel
                    })
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
                    workspace_id,
                    SerializedTerminalPanel {
                        items: SerializedItems::WithSplits(SerializedPaneGroup::Pane(
                            SerializedPane {
                                active: true,
                                children: vec![12345],
                                active_item: Some(12345),
                                pinned_count: 0,
                            },
                        )),
                        active_item_id: None,
                        primary_item_ids: Vec::new(),
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
                    TerminalPanel::restore_serialized_state(workspace, terminal_panel, cx).await
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
                    workspace.set_terminal_provider(TerminalProvider(panel.clone()));
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

    struct TerminalStatusItem {
        item_ids: Vec<Option<gpui::EntityId>>,
    }

    impl Render for TerminalStatusItem {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
        }
    }

    impl workspace::StatusItemView for TerminalStatusItem {
        fn set_active_pane_item(
            &mut self,
            item: Option<&dyn workspace::ItemHandle>,
            _: &mut Window,
            _: &mut Context<Self>,
        ) {
            let item_id = item.map(|item| item.item_id());
            if self.item_ids.last() != Some(&item_id) {
                self.item_ids.push(item_id);
            }
        }

        fn hide_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
            None
        }
    }

    fn observe_terminal_status(
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<TerminalStatusItem> {
        let item = cx.new(|_| TerminalStatusItem {
            item_ids: Vec::new(),
        });
        workspace
            .read(cx)
            .status_bar()
            .clone()
            .update(cx, |status_bar, cx| {
                status_bar.add_right_item(item.clone(), window, cx);
            });
        item
    }

    async fn reopen_terminal_panel(
        workspace_id: WorkspaceId,
        cx: &mut TestAppContext,
    ) -> (
        gpui::WindowHandle<MultiWorkspace>,
        Entity<Workspace>,
        Entity<TerminalPanel>,
    ) {
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let window = cx.add_window(|window, cx| {
            let template = cx.new(|cx| Workspace::test_new(project.clone(), window, cx));
            let app_state = template.read(cx).app_state().clone();
            let workspace =
                cx.new(|cx| Workspace::new(Some(workspace_id), project, app_state, window, cx));
            MultiWorkspace::test_from_workspace(workspace, window, cx)
        });
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let panel = load_terminal_panel(&workspace, window, cx).await;
        panel
            .update(cx, |panel, _| {
                std::mem::replace(&mut panel._restoration, Task::ready(()))
            })
            .await;
        (window, workspace, panel)
    }

    async fn load_terminal_panel(
        workspace: &Entity<Workspace>,
        window: gpui::WindowHandle<MultiWorkspace>,
        cx: &mut TestAppContext,
    ) -> Entity<TerminalPanel> {
        window
            .update(cx, |_, window, cx| {
                let workspace = workspace.downgrade();
                window.spawn(cx, async move |cx| {
                    TerminalPanel::load(workspace, cx.clone()).await
                })
            })
            .unwrap()
            .await
            .unwrap()
    }

    fn terminal_session_binding(
        db: &TerminalDb,
        workspace_id: WorkspaceId,
    ) -> (Option<String>, Option<u64>) {
        db.select_row_bound::<WorkspaceId, (Option<String>, Option<u64>)>(
            "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
        )
        .unwrap()(workspace_id)
        .unwrap()
        .unwrap()
    }

    fn terminal_pane_state(
        pane: &Entity<Pane>,
        cx: &TestAppContext,
    ) -> (Vec<gpui::EntityId>, usize, usize, Option<gpui::EntityId>) {
        pane.read_with(cx, |pane, _| {
            (
                pane.items().map(|item| item.item_id()).collect::<Vec<_>>(),
                pane.pinned_count(),
                pane.active_item_index(),
                pane.preview_item_id(),
            )
        })
    }

    fn add_failed_terminal(
        workspace: &Entity<Workspace>,
        pane: &Entity<Pane>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<TerminalView> {
        let failed = workspace.update(cx, |workspace, cx| {
            let failed = cx.new(|cx| {
                TerminalView::failed_restoration(
                    workspace.weak_handle(),
                    workspace.project().downgrade(),
                    workspace_id,
                    item_id,
                    anyhow!("terminal could not be restored"),
                    window,
                    cx,
                )
            });
            workspace
                .register_serialized_item_id("Terminal", failed.entity_id(), item_id, cx)
                .unwrap();
            failed
        });
        pane.update(cx, |pane, cx| {
            pane.add_item(Box::new(failed.clone()), true, false, None, window, cx)
        });
        failed
    }

    fn panel_terminal(
        panel: &Entity<TerminalPanel>,
        item_id: ItemId,
        cx: &TestAppContext,
    ) -> Entity<TerminalView> {
        panel.read_with(cx, |panel, cx| {
            panel
                .center
                .panes()
                .into_iter()
                .find_map(|pane| {
                    pane.read(cx).items_of_type::<TerminalView>().find(|view| {
                        view.read(cx)
                            .serialization_identity()
                            .is_some_and(|(_, id)| id == item_id)
                    })
                })
                .unwrap()
        })
    }

    fn saved_panel_terminal_ids(kvp: &KeyValueStore, key: &str) -> Vec<ItemId> {
        fn collect(value: &serde_json::Value, ids: &mut Vec<ItemId>) {
            if let Some(items) = value.as_array() {
                ids.extend(items.iter().map(|item| item.as_u64().unwrap()));
            } else if let Some(pane) = value.get("Pane") {
                collect(&pane["children"], ids);
            } else {
                for child in value["Group"]["children"].as_array().unwrap() {
                    collect(child, ids);
                }
            }
        }
        let value = serde_json::from_str::<serde_json::Value>(&kvp.read_kvp(key).unwrap().unwrap())
            .unwrap();
        let mut ids = Vec::new();
        collect(&value["items"], &mut ids);
        ids.sort_unstable();
        ids
    }

    fn panel_terminal_ids(panel: &Entity<TerminalPanel>, cx: &TestAppContext) -> Vec<ItemId> {
        panel.read_with(cx, |panel, cx| {
            let mut ids = panel
                .center
                .panes()
                .into_iter()
                .flat_map(|pane| {
                    pane.read(cx)
                        .items_of_type::<TerminalView>()
                        .map(|view| view.read(cx).serialization_identity().unwrap().1)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            ids.sort_unstable();
            ids
        })
    }

    fn add_panel_display_terminal(
        workspace: &Entity<Workspace>,
        pane: &Entity<Pane>,
        title: &str,
        window: &mut Window,
        cx: &mut App,
    ) -> (Entity<TerminalView>, ItemId) {
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
        let (terminal_view, item_id) = workspace.update(cx, |workspace, cx| {
            let terminal_view = cx.new(|cx| {
                let mut view = TerminalView::new(
                    terminal,
                    workspace.weak_handle(),
                    workspace.project().downgrade(),
                    window,
                    cx,
                );
                view.set_custom_title(Some(String::from(title)), cx);
                view
            });
            let item_id = workspace
                .serialization_id("Terminal", terminal_view.entity_id(), cx)
                .unwrap();
            (terminal_view, item_id)
        });
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(terminal_view.clone()),
                true,
                false,
                None,
                window,
                cx,
            );
        });
        (terminal_view, item_id)
    }

    async fn initialize_terminal_persistence(
        workspace: &Entity<Workspace>,
        item_ids: &[ItemId],
        cx: &mut TestAppContext,
    ) -> WorkspaceId {
        let db = cx.update(|cx| TerminalDb::global(cx));
        let workspace_id = workspace.update(cx, |workspace, _| {
            workspace.set_random_database_id();
            workspace.database_id().unwrap()
        });
        db.write(move |connection| {
            connection.exec_bound("INSERT INTO workspaces (workspace_id) VALUES (?)")?(workspace_id)
        })
        .await
        .unwrap();
        for item_id in item_ids {
            db.save_terminal(
                *item_id,
                workspace_id,
                None,
                Some(format!("terminal-{item_id}")),
            )
            .await
            .unwrap();
        }
        workspace_id
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
            cx.set_global(AppDatabase::test_new());
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }
}

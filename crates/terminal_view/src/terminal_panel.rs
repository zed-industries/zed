use std::{ops::ControlFlow, path::PathBuf, sync::Arc};

use crate::TerminalView;
use collections::{HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use futures::future::join_all;
use gpui::{
    actions, AppContext, AsyncWindowContext, Entity, EventEmitter, ExternalPaths, FocusHandle,
    FocusableView, IntoElement, ParentElement, Pixels, Render, Styled, Subscription, Task, View,
    ViewContext, VisualContext, WeakView, WindowContext,
};
use itertools::Itertools;
use project::{Fs, ProjectEntryId};
use search::{buffer_search::DivRegistrar, BufferSearchBar};
use serde::{Deserialize, Serialize};
use settings::Settings;
use task::{SpawnInTerminal, TaskId};
use terminal::{
    terminal_settings::{Shell, TerminalDockPosition, TerminalSettings},
    SpawnTask,
};
use ui::{h_flex, ButtonCommon, Clickable, IconButton, IconSize, Selectable, Tooltip};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::Item,
    pane,
    ui::IconName,
    DraggedTab, Pane, Workspace,
};

use anyhow::Result;

const TERMINAL_PANEL_KEY: &str = "TerminalPanel";

actions!(terminal_panel, [ToggleFocus]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace.register_action(TerminalPanel::new_terminal);
            workspace.register_action(TerminalPanel::open_terminal);
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<TerminalPanel>(cx);
            });
        },
    )
    .detach();
}

pub struct TerminalPanel {
    pane: View<Pane>,
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    pending_serialization: Task<Option<()>>,
    pending_terminals_to_add: usize,
    _subscriptions: Vec<Subscription>,
    deferred_tasks: HashMap<TaskId, Task<()>>,
}

impl TerminalPanel {
    fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let terminal_panel = cx.view().downgrade();
        let pane = cx.new_view(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.project().clone(),
                Default::default(),
                None,
                cx,
            );
            pane.set_can_split(false, cx);
            pane.set_can_navigate(false, cx);
            pane.display_nav_history_buttons(false);
            pane.set_render_tab_bar_buttons(cx, move |pane, cx| {
                let terminal_panel = terminal_panel.clone();
                h_flex()
                    .gap_2()
                    .child(
                        IconButton::new("plus", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .on_click(move |_, cx| {
                                terminal_panel
                                    .update(cx, |panel, cx| panel.add_terminal(None, None, cx))
                                    .log_err();
                            })
                            .tooltip(|cx| Tooltip::text("New Terminal", cx)),
                    )
                    .child({
                        let zoomed = pane.is_zoomed();
                        IconButton::new("toggle_zoom", IconName::Maximize)
                            .icon_size(IconSize::Small)
                            .selected(zoomed)
                            .selected_icon(IconName::Minimize)
                            .on_click(cx.listener(|pane, _, cx| {
                                pane.toggle_zoom(&workspace::ToggleZoom, cx);
                            }))
                            .tooltip(move |cx| {
                                Tooltip::text(if zoomed { "Zoom Out" } else { "Zoom In" }, cx)
                            })
                    })
                    .into_any_element()
            });

            let workspace = workspace.weak_handle();
            pane.set_custom_drop_handle(cx, move |pane, dropped_item, cx| {
                if let Some(tab) = dropped_item.downcast_ref::<DraggedTab>() {
                    let item = if &tab.pane == cx.view() {
                        pane.item_for_index(tab.ix)
                    } else {
                        tab.pane.read(cx).item_for_index(tab.ix)
                    };
                    if let Some(item) = item {
                        if item.downcast::<TerminalView>().is_some() {
                            return ControlFlow::Continue(());
                        } else if let Some(project_path) = item.project_path(cx) {
                            if let Some(entry_path) = workspace
                                .update(cx, |workspace, cx| {
                                    workspace
                                        .project()
                                        .read(cx)
                                        .absolute_path(&project_path, cx)
                                })
                                .log_err()
                                .flatten()
                            {
                                add_paths_to_terminal(pane, &[entry_path], cx);
                            }
                        }
                    }
                } else if let Some(&entry_id) = dropped_item.downcast_ref::<ProjectEntryId>() {
                    if let Some(entry_path) = workspace
                        .update(cx, |workspace, cx| {
                            let project = workspace.project().read(cx);
                            project
                                .path_for_entry(entry_id, cx)
                                .and_then(|project_path| project.absolute_path(&project_path, cx))
                        })
                        .log_err()
                        .flatten()
                    {
                        add_paths_to_terminal(pane, &[entry_path], cx);
                    }
                } else if let Some(paths) = dropped_item.downcast_ref::<ExternalPaths>() {
                    add_paths_to_terminal(pane, paths.paths(), cx);
                }

                ControlFlow::Break(())
            });
            let buffer_search_bar = cx.new_view(search::BufferSearchBar::new);
            pane.toolbar()
                .update(cx, |toolbar, cx| toolbar.add_item(buffer_search_bar, cx));
            pane
        });
        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe(&pane, Self::handle_pane_event),
        ];
        let this = Self {
            pane,
            fs: workspace.app_state().fs.clone(),
            workspace: workspace.weak_handle(),
            pending_serialization: Task::ready(None),
            width: None,
            height: None,
            pending_terminals_to_add: 0,
            deferred_tasks: HashMap::default(),
            _subscriptions: subscriptions,
        };
        this
    }

    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(TERMINAL_PANEL_KEY) })
            .await
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedTerminalPanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

        let (panel, pane, items) = workspace.update(&mut cx, |workspace, cx| {
            let panel = cx.new_view(|cx| TerminalPanel::new(workspace, cx));
            let items = if let Some(serialized_panel) = serialized_panel.as_ref() {
                panel.update(cx, |panel, cx| {
                    cx.notify();
                    panel.height = serialized_panel.height.map(|h| h.round());
                    panel.width = serialized_panel.width.map(|w| w.round());
                    panel.pane.update(cx, |_, cx| {
                        serialized_panel
                            .items
                            .iter()
                            .map(|item_id| {
                                TerminalView::deserialize(
                                    workspace.project().clone(),
                                    workspace.weak_handle(),
                                    workspace.database_id(),
                                    *item_id,
                                    cx,
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                })
            } else {
                Vec::new()
            };
            let pane = panel.read(cx).pane.clone();
            (panel, pane, items)
        })?;

        if let Some(workspace) = workspace.upgrade() {
            panel
                .update(&mut cx, |panel, cx| {
                    panel._subscriptions.push(cx.subscribe(
                        &workspace,
                        |terminal_panel, _, e, cx| {
                            if let workspace::Event::SpawnTask(spawn_in_terminal) = e {
                                terminal_panel.spawn_task(spawn_in_terminal, cx);
                            };
                        },
                    ))
                })
                .ok();
        }

        let pane = pane.downgrade();
        let items = futures::future::join_all(items).await;
        pane.update(&mut cx, |pane, cx| {
            let active_item_id = serialized_panel
                .as_ref()
                .and_then(|panel| panel.active_item_id);
            let mut active_ix = None;
            for item in items {
                if let Some(item) = item.log_err() {
                    let item_id = item.entity_id().as_u64();
                    pane.add_item(Box::new(item), false, false, None, cx);
                    if Some(item_id) == active_item_id {
                        active_ix = Some(pane.items_len() - 1);
                    }
                }
            }

            if let Some(active_ix) = active_ix {
                pane.activate_item(active_ix, false, false, cx)
            }
        })?;

        Ok(panel)
    }

    fn handle_pane_event(
        &mut self,
        _pane: View<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::ActivateItem { .. } => self.serialize(cx),
            pane::Event::RemoveItem { .. } => self.serialize(cx),
            pane::Event::Remove => cx.emit(PanelEvent::Close),
            pane::Event::ZoomIn => cx.emit(PanelEvent::ZoomIn),
            pane::Event::ZoomOut => cx.emit(PanelEvent::ZoomOut),

            pane::Event::AddItem { item } => {
                if let Some(workspace) = self.workspace.upgrade() {
                    let pane = self.pane.clone();
                    workspace.update(cx, |workspace, cx| item.added_to_pane(workspace, pane, cx))
                }
            }

            _ => {}
        }
    }

    pub fn open_terminal(
        workspace: &mut Workspace,
        action: &workspace::OpenTerminal,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(this) = workspace.focus_panel::<Self>(cx) else {
            return;
        };

        this.update(cx, |this, cx| {
            this.add_terminal(Some(action.working_directory.clone()), None, cx)
        })
    }

    pub fn spawn_task(&mut self, spawn_in_terminal: &SpawnInTerminal, cx: &mut ViewContext<Self>) {
        let mut spawn_task = SpawnTask {
            id: spawn_in_terminal.id.clone(),
            label: spawn_in_terminal.label.clone(),
            command: spawn_in_terminal.command.clone(),
            args: spawn_in_terminal.args.clone(),
            env: spawn_in_terminal.env.clone(),
        };
        // Set up shell args unconditionally, as tasks are always spawned inside of a shell.
        let Some((shell, mut user_args)) = (match TerminalSettings::get_global(cx).shell.clone() {
            Shell::System => std::env::var("SHELL").ok().map(|shell| (shell, vec![])),
            Shell::Program(shell) => Some((shell, vec![])),
            Shell::WithArguments { program, args } => Some((program, args)),
        }) else {
            return;
        };

        let mut command = std::mem::take(&mut spawn_task.command);
        let args = std::mem::take(&mut spawn_task.args);
        for arg in args {
            command.push(' ');
            command.push_str(&arg);
        }
        spawn_task.command = shell;
        user_args.extend(["-i".to_owned(), "-c".to_owned(), command]);
        spawn_task.args = user_args;

        let working_directory = spawn_in_terminal.cwd.clone();
        let allow_concurrent_runs = spawn_in_terminal.allow_concurrent_runs;
        let use_new_terminal = spawn_in_terminal.use_new_terminal;

        if allow_concurrent_runs && use_new_terminal {
            self.spawn_in_new_terminal(spawn_task, working_directory, cx);
            return;
        }

        let terminals_for_task = self.terminals_for_task(&spawn_in_terminal.id, cx);
        if terminals_for_task.is_empty() {
            self.spawn_in_new_terminal(spawn_task, working_directory, cx);
            return;
        }
        let (existing_item_index, existing_terminal) = terminals_for_task
            .last()
            .expect("covered no terminals case above")
            .clone();
        if allow_concurrent_runs {
            debug_assert!(
                !use_new_terminal,
                "Should have handled 'allow_concurrent_runs && use_new_terminal' case above"
            );
            self.replace_terminal(
                working_directory,
                spawn_task,
                existing_item_index,
                existing_terminal,
                cx,
            );
        } else {
            self.deferred_tasks.insert(
                spawn_in_terminal.id.clone(),
                cx.spawn(|terminal_panel, mut cx| async move {
                    wait_for_terminals_tasks(terminals_for_task, &mut cx).await;
                    terminal_panel
                        .update(&mut cx, |terminal_panel, cx| {
                            if use_new_terminal {
                                terminal_panel.spawn_in_new_terminal(
                                    spawn_task,
                                    working_directory,
                                    cx,
                                );
                            } else {
                                terminal_panel.replace_terminal(
                                    working_directory,
                                    spawn_task,
                                    existing_item_index,
                                    existing_terminal,
                                    cx,
                                );
                            }
                        })
                        .ok();
                }),
            );
        }
    }

    fn spawn_in_new_terminal(
        &mut self,
        spawn_task: SpawnTask,
        working_directory: Option<PathBuf>,
        cx: &mut ViewContext<Self>,
    ) {
        self.add_terminal(working_directory, Some(spawn_task), cx);
        let task_workspace = self.workspace.clone();
        cx.spawn(|_, mut cx| async move {
            task_workspace
                .update(&mut cx, |workspace, cx| workspace.focus_panel::<Self>(cx))
                .ok()
        })
        .detach();
    }

    ///Create a new Terminal in the current working directory or the user's home directory
    fn new_terminal(
        workspace: &mut Workspace,
        _: &workspace::NewTerminal,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(this) = workspace.focus_panel::<Self>(cx) else {
            return;
        };

        this.update(cx, |this, cx| this.add_terminal(None, None, cx))
    }

    fn terminals_for_task(
        &self,
        id: &TaskId,
        cx: &mut AppContext,
    ) -> Vec<(usize, View<TerminalView>)> {
        self.pane
            .read(cx)
            .items()
            .enumerate()
            .filter_map(|(index, item)| Some((index, item.act_as::<TerminalView>(cx)?)))
            .filter_map(|(index, terminal_view)| {
                let task_state = terminal_view.read(cx).terminal().read(cx).task()?;
                if &task_state.id == id {
                    Some((index, terminal_view))
                } else {
                    None
                }
            })
            .collect()
    }

    fn activate_terminal_view(&self, item_index: usize, cx: &mut WindowContext) {
        self.pane.update(cx, |pane, cx| {
            pane.activate_item(item_index, true, true, cx)
        })
    }

    fn add_terminal(
        &mut self,
        working_directory: Option<PathBuf>,
        spawn_task: Option<SpawnTask>,
        cx: &mut ViewContext<Self>,
    ) {
        let workspace = self.workspace.clone();
        self.pending_terminals_to_add += 1;
        cx.spawn(|terminal_panel, mut cx| async move {
            let pane = terminal_panel.update(&mut cx, |this, _| this.pane.clone())?;
            workspace.update(&mut cx, |workspace, cx| {
                let working_directory = if let Some(working_directory) = working_directory {
                    Some(working_directory)
                } else {
                    let working_directory_strategy =
                        TerminalSettings::get_global(cx).working_directory.clone();
                    crate::get_working_directory(workspace, cx, working_directory_strategy)
                };

                let window = cx.window_handle();
                if let Some(terminal) = workspace.project().update(cx, |project, cx| {
                    project
                        .create_terminal(working_directory, spawn_task, window, cx)
                        .log_err()
                }) {
                    let terminal = Box::new(cx.new_view(|cx| {
                        TerminalView::new(
                            terminal,
                            workspace.weak_handle(),
                            workspace.database_id(),
                            cx,
                        )
                    }));
                    pane.update(cx, |pane, cx| {
                        let focus = pane.has_focus(cx);
                        pane.add_item(terminal, true, focus, None, cx);
                    });
                }
            })?;
            terminal_panel.update(&mut cx, |this, cx| {
                this.pending_terminals_to_add = this.pending_terminals_to_add.saturating_sub(1);
                this.serialize(cx)
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let mut items_to_serialize = HashSet::default();
        let items = self
            .pane
            .read(cx)
            .items()
            .filter_map(|item| {
                let terminal_view = item.act_as::<TerminalView>(cx)?;
                if terminal_view.read(cx).terminal().read(cx).task().is_some() {
                    None
                } else {
                    let id = item.item_id().as_u64();
                    items_to_serialize.insert(id);
                    Some(id)
                }
            })
            .collect::<Vec<_>>();
        let active_item_id = self
            .pane
            .read(cx)
            .active_item()
            .map(|item| item.item_id().as_u64())
            .filter(|active_id| items_to_serialize.contains(active_id));
        let height = self.height;
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        TERMINAL_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedTerminalPanel {
                            items,
                            active_item_id,
                            height,
                            width,
                        })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn replace_terminal(
        &self,
        working_directory: Option<PathBuf>,
        spawn_task: SpawnTask,
        terminal_item_index: usize,
        terminal_to_replace: View<TerminalView>,
        cx: &mut ViewContext<'_, Self>,
    ) -> Option<()> {
        let project = self
            .workspace
            .update(cx, |workspace, _| workspace.project().clone())
            .ok()?;
        let window = cx.window_handle();
        let new_terminal = project.update(cx, |project, cx| {
            project
                .create_terminal(working_directory, Some(spawn_task), window, cx)
                .log_err()
        })?;
        terminal_to_replace.update(cx, |terminal_to_replace, cx| {
            terminal_to_replace.set_terminal(new_terminal, cx);
        });
        self.activate_terminal_view(terminal_item_index, cx);
        let task_workspace = self.workspace.clone();
        cx.spawn(|_, mut cx| async move {
            task_workspace
                .update(&mut cx, |workspace, cx| workspace.focus_panel::<Self>(cx))
                .ok()
        })
        .detach();
        Some(())
    }
}

async fn wait_for_terminals_tasks(
    terminals_for_task: Vec<(usize, View<TerminalView>)>,
    cx: &mut AsyncWindowContext,
) {
    let pending_tasks = terminals_for_task.iter().filter_map(|(_, terminal)| {
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

fn add_paths_to_terminal(pane: &mut Pane, paths: &[PathBuf], cx: &mut ViewContext<'_, Pane>) {
    if let Some(terminal_view) = pane
        .active_item()
        .and_then(|item| item.downcast::<TerminalView>())
    {
        cx.focus_view(&terminal_view);
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut registrar = DivRegistrar::new(
            |panel, cx| {
                panel
                    .pane
                    .read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<BufferSearchBar>()
            },
            cx,
        );
        BufferSearchBar::register(&mut registrar);
        registrar.into_div().size_full().child(self.pane.clone())
    }
}

impl FocusableView for TerminalPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

impl Panel for TerminalPanel {
    fn position(&self, cx: &WindowContext) -> DockPosition {
        match TerminalSettings::get_global(cx).dock {
            TerminalDockPosition::Left => DockPosition::Left,
            TerminalDockPosition::Bottom => DockPosition::Bottom,
            TerminalDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<TerminalSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                DockPosition::Left => TerminalDockPosition::Left,
                DockPosition::Bottom => TerminalDockPosition::Bottom,
                DockPosition::Right => TerminalDockPosition::Right,
            };
            settings.dock = Some(dock);
        });
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        let settings = TerminalSettings::get_global(cx);
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or_else(|| settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or_else(|| settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        self.serialize(cx);
        cx.notify();
    }

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active && self.pane.read(cx).items_len() == 0 && self.pending_terminals_to_add == 0 {
            self.add_terminal(None, None, cx)
        }
    }

    fn icon_label(&self, cx: &WindowContext) -> Option<String> {
        let count = self.pane.read(cx).items_len();
        if count == 0 {
            None
        } else {
            Some(count.to_string())
        }
    }

    fn persistent_name() -> &'static str {
        "TerminalPanel"
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        Some(IconName::Terminal)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Terminal Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedTerminalPanel {
    items: Vec<u64>,
    active_item_id: Option<u64>,
    width: Option<Pixels>,
    height: Option<Pixels>,
}

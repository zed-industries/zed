pub(crate) mod breakpoint_list;
pub(crate) mod console;
pub(crate) mod loaded_source_list;
pub(crate) mod module_list;
pub mod stack_frame_list;
pub mod variable_list;

use std::{any::Any, ops::ControlFlow, path::PathBuf, sync::Arc, time::Duration};

use crate::persistence::{self, DebuggerPaneItem, SerializedLayout};

use super::DebugPanelItemEvent;
use anyhow::{Result, anyhow};
use breakpoint_list::BreakpointList;
use collections::{HashMap, IndexMap};
use console::Console;
use dap::{
    Capabilities, RunInTerminalRequestArguments, Thread,
    adapters::{DebugAdapterName, DebugTaskDefinition},
    client::SessionId,
    debugger_settings::DebuggerSettings,
};
use futures::{SinkExt, channel::mpsc};
use gpui::{
    Action as _, AnyView, AppContext, Axis, Entity, EntityId, EventEmitter, FocusHandle, Focusable,
    NoAction, Pixels, Point, Subscription, Task, WeakEntity,
};
use language::Buffer;
use loaded_source_list::LoadedSourceList;
use module_list::ModuleList;
use project::{
    Project, WorktreeId,
    debugger::session::{Session, SessionEvent, ThreadId, ThreadStatus},
    terminals::TerminalKind,
};
use rpc::proto::ViewId;
use serde_json::Value;
use settings::Settings;
use stack_frame_list::StackFrameList;
use task::{
    BuildTaskDefinition, DebugScenario, LaunchRequest, TaskContext, substitute_variables_in_map,
    substitute_variables_in_str,
};
use terminal_view::TerminalView;
use ui::{
    ActiveTheme, AnyElement, App, ButtonCommon as _, Clickable as _, Context, ContextMenu,
    DropdownMenu, FluentBuilder, IconButton, IconName, IconSize, InteractiveElement, IntoElement,
    Label, LabelCommon as _, ParentElement, Render, SharedString, StatefulInteractiveElement,
    Styled, Tab, Tooltip, VisibleOnHover, VisualContext, Window, div, h_flex, v_flex,
};
use util::ResultExt;
use variable_list::VariableList;
use workspace::{
    ActivePaneDecorator, DraggedTab, Item, ItemHandle, Member, Pane, PaneGroup, SplitDirection,
    Workspace, item::TabContentParams, move_item, pane::Event,
};

pub struct RunningState {
    session: Entity<Session>,
    thread_id: Option<ThreadId>,
    focus_handle: FocusHandle,
    _remote_id: Option<ViewId>,
    workspace: WeakEntity<Workspace>,
    session_id: SessionId,
    variable_list: Entity<variable_list::VariableList>,
    _subscriptions: Vec<Subscription>,
    stack_frame_list: Entity<stack_frame_list::StackFrameList>,
    loaded_sources_list: Entity<LoadedSourceList>,
    pub debug_terminal: Entity<DebugTerminal>,
    module_list: Entity<module_list::ModuleList>,
    console: Entity<Console>,
    breakpoint_list: Entity<BreakpointList>,
    panes: PaneGroup,
    active_pane: Option<Entity<Pane>>,
    pane_close_subscriptions: HashMap<EntityId, Subscription>,
    dock_axis: Axis,
    _schedule_serialize: Option<Task<()>>,
}

impl Render for RunningState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let zoomed_pane = self
            .panes
            .panes()
            .into_iter()
            .find(|pane| pane.read(cx).is_zoomed());

        let active = self.panes.panes().into_iter().next();
        let x = if let Some(ref zoomed_pane) = zoomed_pane {
            zoomed_pane.update(cx, |pane, cx| pane.render(window, cx).into_any_element())
        } else if let Some(active) = active {
            self.panes
                .render(
                    None,
                    &ActivePaneDecorator::new(active, &self.workspace),
                    window,
                    cx,
                )
                .into_any_element()
        } else {
            div().into_any_element()
        };
        let thread_status = self
            .thread_id
            .map(|thread_id| self.session.read(cx).thread_status(thread_id))
            .unwrap_or(ThreadStatus::Exited);

        self.variable_list.update(cx, |this, cx| {
            this.disabled(thread_status != ThreadStatus::Stopped, cx);
        });
        v_flex()
            .size_full()
            .key_context("DebugSessionItem")
            .track_focus(&self.focus_handle(cx))
            .child(h_flex().flex_1().child(x))
    }
}

pub(crate) struct SubView {
    inner: AnyView,
    pane_focus_handle: FocusHandle,
    kind: DebuggerPaneItem,
    show_indicator: Box<dyn Fn(&App) -> bool>,
    hovered: bool,
}

impl SubView {
    pub(crate) fn new(
        pane_focus_handle: FocusHandle,
        view: AnyView,
        kind: DebuggerPaneItem,
        show_indicator: Option<Box<dyn Fn(&App) -> bool>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            kind,
            inner: view,
            pane_focus_handle,
            show_indicator: show_indicator.unwrap_or(Box::new(|_| false)),
            hovered: false,
        })
    }

    pub(crate) fn view_kind(&self) -> DebuggerPaneItem {
        self.kind
    }
}
impl Focusable for SubView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.pane_focus_handle.clone()
    }
}
impl EventEmitter<()> for SubView {}
impl Item for SubView {
    type Event = ();

    /// This is used to serialize debugger pane layouts
    /// A SharedString gets converted to a enum and back during serialization/deserialization.
    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.kind.to_shared_string()
    }

    fn tab_content(
        &self,
        params: workspace::item::TabContentParams,
        _: &Window,
        cx: &App,
    ) -> AnyElement {
        let label = Label::new(self.kind.to_shared_string())
            .size(ui::LabelSize::Small)
            .color(params.text_color())
            .line_height_style(ui::LineHeightStyle::UiLabel);

        if !params.selected && self.show_indicator.as_ref()(cx) {
            return h_flex()
                .justify_between()
                .child(ui::Indicator::dot())
                .gap_2()
                .child(label)
                .into_any_element();
        }

        label.into_any_element()
    }
}

impl Render for SubView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id(SharedString::from(format!(
                "subview-container-{}",
                self.kind.to_shared_string()
            )))
            .on_hover(cx.listener(|this, hovered, _, cx| {
                this.hovered = *hovered;
                cx.notify();
            }))
            .size_full()
            // Add border unconditionally to prevent layout shifts on focus changes.
            .border_1()
            .when(self.pane_focus_handle.contains_focused(window, cx), |el| {
                el.border_color(cx.theme().colors().pane_focused_border)
            })
            .child(self.inner.clone())
    }
}

pub(crate) fn new_debugger_pane(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    window: &mut Window,
    cx: &mut Context<RunningState>,
) -> Entity<Pane> {
    let weak_running = cx.weak_entity();
    let custom_drop_handle = {
        let workspace = workspace.clone();
        let project = project.downgrade();
        let weak_running = weak_running.clone();
        move |pane: &mut Pane, any: &dyn Any, window: &mut Window, cx: &mut Context<Pane>| {
            let Some(tab) = any.downcast_ref::<DraggedTab>() else {
                return ControlFlow::Break(());
            };
            let Some(project) = project.upgrade() else {
                return ControlFlow::Break(());
            };
            let this_pane = cx.entity().clone();
            let item = if tab.pane == this_pane {
                pane.item_for_index(tab.ix)
            } else {
                tab.pane.read(cx).item_for_index(tab.ix)
            };
            let Some(item) = item.filter(|item| item.downcast::<SubView>().is_some()) else {
                return ControlFlow::Break(());
            };

            let source = tab.pane.clone();
            let item_id_to_move = item.item_id();

            let Ok(new_split_pane) = pane
                .drag_split_direction()
                .map(|split_direction| {
                    weak_running.update(cx, |running, cx| {
                        let new_pane =
                            new_debugger_pane(workspace.clone(), project.clone(), window, cx);
                        let _previous_subscription = running.pane_close_subscriptions.insert(
                            new_pane.entity_id(),
                            cx.subscribe_in(&new_pane, window, RunningState::handle_pane_event),
                        );
                        debug_assert!(_previous_subscription.is_none());
                        running
                            .panes
                            .split(&this_pane, &new_pane, split_direction)?;
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

            ControlFlow::Break(())
        }
    };

    let ret = cx.new(move |cx| {
        let mut pane = Pane::new(
            workspace.clone(),
            project.clone(),
            Default::default(),
            None,
            NoAction.boxed_clone(),
            window,
            cx,
        );
        let focus_handle = pane.focus_handle(cx);
        pane.set_can_split(Some(Arc::new({
            let weak_running = weak_running.clone();
            move |pane, dragged_item, _window, cx| {
                if let Some(tab) = dragged_item.downcast_ref::<DraggedTab>() {
                    let is_current_pane = tab.pane == cx.entity();
                    let Some(can_drag_away) = weak_running
                        .update(cx, |running_state, _| {
                            let current_panes = running_state.panes.panes();
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
                            return item.downcast::<SubView>().is_some();
                        }
                    }
                }
                false
            }
        })));
        pane.display_nav_history_buttons(None);
        pane.set_custom_drop_handle(cx, custom_drop_handle);
        pane.set_should_display_tab_bar(|_, _| true);
        pane.set_render_tab_bar_buttons(cx, |_, _, _| (None, None));
        pane.set_render_tab_bar(cx, {
            move |pane, window, cx| {
                let active_pane_item = pane.active_item();
                let pane_group_id: SharedString =
                    format!("pane-zoom-button-hover-{}", cx.entity_id()).into();
                let is_hovered = active_pane_item.as_ref().map_or(false, |item| {
                    item.downcast::<SubView>()
                        .map_or(false, |this| this.read(cx).hovered)
                });
                h_flex()
                    .group(pane_group_id.clone())
                    .justify_between()
                    .bg(cx.theme().colors().tab_bar_background)
                    .border_b_1()
                    .px_2()
                    .border_color(cx.theme().colors().border)
                    .track_focus(&focus_handle)
                    .child(
                        h_flex()
                            .w_full()
                            .gap_1()
                            .h(Tab::container_height(cx))
                            .drag_over::<DraggedTab>(|bar, _, _, cx| {
                                bar.bg(cx.theme().colors().drop_target_background)
                            })
                            .on_drop(cx.listener(
                                move |this, dragged_tab: &DraggedTab, window, cx| {
                                    this.drag_split_direction = None;
                                    this.handle_tab_drop(dragged_tab, this.items_len(), window, cx)
                                },
                            ))
                            .children(pane.items().enumerate().map(|(ix, item)| {
                                let selected = active_pane_item
                                    .as_ref()
                                    .map_or(false, |active| active.item_id() == item.item_id());
                                let deemphasized = !pane.has_focus(window, cx);
                                let item_ = item.boxed_clone();
                                div()
                                    .id(SharedString::from(format!(
                                        "debugger_tab_{}",
                                        item.item_id().as_u64()
                                    )))
                                    .p_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .map(|this| {
                                        let theme = cx.theme();
                                        if selected {
                                            let color = theme.colors().tab_active_background;
                                            let color = if deemphasized {
                                                color.opacity(0.5)
                                            } else {
                                                color
                                            };
                                            this.bg(color)
                                        } else {
                                            let hover_color = theme.colors().element_hover;
                                            this.hover(|style| style.bg(hover_color))
                                        }
                                    })
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        let index = this.index_for_item(&*item_);
                                        if let Some(index) = index {
                                            this.activate_item(index, true, true, window, cx);
                                        }
                                    }))
                                    .child(item.tab_content(
                                        TabContentParams {
                                            selected,
                                            deemphasized,
                                            ..Default::default()
                                        },
                                        window,
                                        cx,
                                    ))
                                    .on_drop(cx.listener(
                                        move |this, dragged_tab: &DraggedTab, window, cx| {
                                            this.drag_split_direction = None;
                                            this.handle_tab_drop(dragged_tab, ix, window, cx)
                                        },
                                    ))
                                    .on_drag(
                                        DraggedTab {
                                            item: item.boxed_clone(),
                                            pane: cx.entity().clone(),
                                            detail: 0,
                                            is_active: selected,
                                            ix,
                                        },
                                        |tab, _, _, cx| cx.new(|_| tab.clone()),
                                    )
                            })),
                    )
                    .child({
                        let zoomed = pane.is_zoomed();
                        div()
                            .visible_on_hover(pane_group_id)
                            .when(is_hovered, |this| this.visible())
                            .child(
                                IconButton::new(
                                    SharedString::from(format!(
                                        "debug-toggle-zoom-{}",
                                        cx.entity_id()
                                    )),
                                    if zoomed {
                                        IconName::Minimize
                                    } else {
                                        IconName::Maximize
                                    },
                                )
                                .icon_size(IconSize::XSmall)
                                .on_click(cx.listener(move |pane, _, window, cx| {
                                    pane.toggle_zoom(&workspace::ToggleZoom, window, cx);
                                }))
                                .tooltip({
                                    let focus_handle = focus_handle.clone();
                                    move |window, cx| {
                                        let zoomed_text =
                                            if zoomed { "Zoom Out" } else { "Zoom In" };
                                        Tooltip::for_action_in(
                                            zoomed_text,
                                            &workspace::ToggleZoom,
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                    }
                                }),
                            )
                    })
                    .into_any_element()
            }
        });
        pane
    });

    ret
}

pub struct DebugTerminal {
    pub terminal: Option<Entity<TerminalView>>,
    focus_handle: FocusHandle,
}

impl DebugTerminal {
    fn empty(cx: &mut Context<Self>) -> Self {
        Self {
            terminal: None,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl gpui::Render for DebugTerminal {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        if let Some(terminal) = self.terminal.clone() {
            terminal.into_any_element()
        } else {
            div().track_focus(&self.focus_handle).into_any_element()
        }
    }
}
impl Focusable for DebugTerminal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if let Some(terminal) = self.terminal.as_ref() {
            return terminal.focus_handle(cx);
        } else {
            self.focus_handle.clone()
        }
    }
}

impl RunningState {
    pub fn new(
        session: Entity<Session>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        serialized_pane_layout: Option<SerializedLayout>,
        dock_axis: Axis,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let session_id = session.read(cx).session_id();
        let weak_state = cx.weak_entity();
        let stack_frame_list = cx.new(|cx| {
            StackFrameList::new(workspace.clone(), session.clone(), weak_state, window, cx)
        });

        let debug_terminal = cx.new(DebugTerminal::empty);

        let variable_list =
            cx.new(|cx| VariableList::new(session.clone(), stack_frame_list.clone(), window, cx));

        let module_list = cx.new(|cx| ModuleList::new(session.clone(), workspace.clone(), cx));

        let loaded_source_list = cx.new(|cx| LoadedSourceList::new(session.clone(), cx));

        let console = cx.new(|cx| {
            Console::new(
                session.clone(),
                stack_frame_list.clone(),
                variable_list.clone(),
                window,
                cx,
            )
        });

        let breakpoint_list = BreakpointList::new(session.clone(), workspace.clone(), &project, cx);

        let _subscriptions = vec![
            cx.observe(&module_list, |_, _, cx| cx.notify()),
            cx.subscribe_in(&session, window, |this, _, event, window, cx| {
                match event {
                    SessionEvent::Stopped(thread_id) => {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                workspace.open_panel::<crate::DebugPanel>(window, cx);
                            })
                            .log_err();

                        if let Some(thread_id) = thread_id {
                            this.select_thread(*thread_id, window, cx);
                        }
                    }
                    SessionEvent::Threads => {
                        let threads = this.session.update(cx, |this, cx| this.threads(cx));
                        this.select_current_thread(&threads, window, cx);
                    }
                    SessionEvent::CapabilitiesLoaded => {
                        let capabilities = this.capabilities(cx);
                        if !capabilities.supports_modules_request.unwrap_or(false) {
                            this.remove_pane_item(DebuggerPaneItem::Modules, window, cx);
                        }
                        if !capabilities
                            .supports_loaded_sources_request
                            .unwrap_or(false)
                        {
                            this.remove_pane_item(DebuggerPaneItem::LoadedSources, window, cx);
                        }
                    }
                    SessionEvent::RunInTerminal { request, sender } => this
                        .handle_run_in_terminal(request, sender.clone(), window, cx)
                        .detach_and_log_err(cx),

                    _ => {}
                }
                cx.notify()
            }),
            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                this.serialize_layout(window, cx);
            }),
        ];

        let mut pane_close_subscriptions = HashMap::default();
        let panes = if let Some(root) = serialized_pane_layout.and_then(|serialized_layout| {
            persistence::deserialize_pane_layout(
                serialized_layout.panes,
                dock_axis != serialized_layout.dock_axis,
                &workspace,
                &project,
                &stack_frame_list,
                &variable_list,
                &module_list,
                &console,
                &breakpoint_list,
                &loaded_source_list,
                &debug_terminal,
                &mut pane_close_subscriptions,
                window,
                cx,
            )
        }) {
            workspace::PaneGroup::with_root(root)
        } else {
            pane_close_subscriptions.clear();

            let root = Self::default_pane_layout(
                project,
                &workspace,
                &stack_frame_list,
                &variable_list,
                &module_list,
                &loaded_source_list,
                &console,
                &breakpoint_list,
                dock_axis,
                &mut pane_close_subscriptions,
                window,
                cx,
            );

            workspace::PaneGroup::with_root(root)
        };

        Self {
            session,
            workspace,
            focus_handle,
            variable_list,
            _subscriptions,
            thread_id: None,
            _remote_id: None,
            stack_frame_list,
            session_id,
            panes,
            active_pane: None,
            module_list,
            console,
            breakpoint_list,
            loaded_sources_list: loaded_source_list,
            pane_close_subscriptions,
            debug_terminal,
            dock_axis,
            _schedule_serialize: None,
        }
    }

    pub(crate) fn remove_pane_item(
        &mut self,
        item_kind: DebuggerPaneItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((pane, item_id)) = self.panes.panes().iter().find_map(|pane| {
            Some(pane).zip(
                pane.read(cx)
                    .items()
                    .find(|item| {
                        item.act_as::<SubView>(cx)
                            .is_some_and(|view| view.read(cx).kind == item_kind)
                    })
                    .map(|item| item.item_id()),
            )
        }) {
            pane.update(cx, |pane, cx| {
                pane.remove_item(item_id, false, true, window, cx)
            })
        }
    }

    pub(crate) fn has_pane_at_position(&self, position: Point<Pixels>) -> bool {
        self.panes.pane_at_pixel_position(position).is_some()
    }

    pub(crate) fn resolve_scenario(
        &self,
        scenario: DebugScenario,
        task_context: TaskContext,
        buffer: Option<Entity<Buffer>>,
        worktree_id: Option<WorktreeId>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<DebugTaskDefinition>> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(Err(anyhow!("no workspace")));
        };
        let project = workspace.read(cx).project().clone();
        let dap_store = project.read(cx).dap_store().downgrade();
        let task_store = project.read(cx).task_store().downgrade();
        let weak_project = project.downgrade();
        let weak_workspace = workspace.downgrade();
        cx.spawn_in(window, async move |this, cx| {
            let DebugScenario {
                adapter,
                label,
                build,
                request,
                initialize_args,
                tcp_connection,
                stop_on_entry,
            } = scenario;
            let build_output = if let Some(build) = build {
                let (task, locator_name) = match build {
                    BuildTaskDefinition::Template {
                        task_template,
                        locator_name,
                    } => (task_template, locator_name),
                    BuildTaskDefinition::ByName(ref label) => {
                        let Some(task) = task_store.update(cx, |this, cx| {
                            this.task_inventory().and_then(|inventory| {
                                inventory.read(cx).task_template_by_label(
                                    buffer,
                                    worktree_id,
                                    &label,
                                    cx,
                                )
                            })
                        })?
                        else {
                            anyhow::bail!("Couldn't find task template for {:?}", build)
                        };
                        (task, None)
                    }
                };
                let locator_name = if let Some(locator_name) = locator_name {
                    debug_assert!(request.is_none());
                    Some(locator_name)
                } else if request.is_none() {
                    dap_store
                        .update(cx, |this, cx| {
                            this.debug_scenario_for_build_task(task.clone(), adapter.clone(), cx)
                                .and_then(|scenario| match scenario.build {
                                    Some(BuildTaskDefinition::Template {
                                        locator_name, ..
                                    }) => locator_name,
                                    _ => None,
                                })
                        })
                        .ok()
                        .flatten()
                } else {
                    None
                };
                let Some(task) = task.resolve_task("debug-build-task", &task_context) else {
                    anyhow::bail!("Could not resolve task variables within a debug scenario");
                };
                let terminal = project
                    .update_in(cx, |project, window, cx| {
                        project.create_terminal(
                            TerminalKind::Task(task.resolved.clone()),
                            window.window_handle(),
                            cx,
                        )
                    })?
                    .await?;

                let terminal_view = cx.new_window_entity(|window, cx| {
                    TerminalView::new(
                        terminal.clone(),
                        weak_workspace,
                        None,
                        weak_project,
                        false,
                        window,
                        cx,
                    )
                })?;

                this.update_in(cx, |this, window, cx| {
                    this.ensure_pane_item(DebuggerPaneItem::Terminal, window, cx);
                    this.debug_terminal.update(cx, |debug_terminal, cx| {
                        debug_terminal.terminal = Some(terminal_view);
                        cx.notify();
                    });
                })?;

                let exit_status = terminal
                    .read_with(cx, |terminal, cx| terminal.wait_for_completed_task(cx))?
                    .await
                    .ok_or_else(|| anyhow!("Failed to wait for completed task"))?;

                if !exit_status.success() {
                    anyhow::bail!("Build failed");
                }
                Some((task, locator_name))
            } else {
                None
            };
            let request = if let Some(request) = request {
                request
            } else if let Some((task, locator_name)) = build_output {
                let locator_name = locator_name
                    .ok_or_else(|| anyhow!("Could not find a valid locator for a build task"))?;
                dap_store
                    .update(cx, |this, cx| {
                        this.run_debug_locator(&locator_name, task.resolved, cx)
                    })?
                    .await?
            } else {
                return Err(anyhow!("No request or build provided"));
            };
            let request = match request {
                dap::DebugRequest::Launch(launch_request) => {
                    let cwd = match launch_request.cwd.as_deref().and_then(|path| path.to_str()) {
                        Some(cwd) => {
                            let substituted_cwd = substitute_variables_in_str(&cwd, &task_context)
                                .ok_or_else(|| anyhow!("Failed to substitute variables in cwd"))?;
                            Some(PathBuf::from(substituted_cwd))
                        }
                        None => None,
                    };

                    let env = substitute_variables_in_map(
                        &launch_request.env.into_iter().collect(),
                        &task_context,
                    )
                    .ok_or_else(|| anyhow!("Failed to substitute variables in env"))?
                    .into_iter()
                    .collect();
                    let new_launch_request = LaunchRequest {
                        program: substitute_variables_in_str(
                            &launch_request.program,
                            &task_context,
                        )
                        .ok_or_else(|| anyhow!("Failed to substitute variables in program"))?,
                        args: launch_request
                            .args
                            .into_iter()
                            .map(|arg| substitute_variables_in_str(&arg, &task_context))
                            .collect::<Option<Vec<_>>>()
                            .ok_or_else(|| anyhow!("Failed to substitute variables in args"))?,
                        cwd,
                        env,
                    };

                    dap::DebugRequest::Launch(new_launch_request)
                }
                request @ dap::DebugRequest::Attach(_) => request,
            };
            Ok(DebugTaskDefinition {
                label,
                adapter: DebugAdapterName(adapter),
                request,
                initialize_args,
                stop_on_entry,
                tcp_connection,
            })
        })
    }

    fn handle_run_in_terminal(
        &self,
        request: &RunInTerminalRequestArguments,
        mut sender: mpsc::Sender<Result<u32>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let running = cx.entity();
        let Ok(project) = self
            .workspace
            .update(cx, |workspace, _| workspace.project().clone())
        else {
            return Task::ready(Err(anyhow!("no workspace")));
        };
        let session = self.session.read(cx);

        let cwd = Some(&request.cwd)
            .filter(|cwd| cwd.len() > 0)
            .map(PathBuf::from)
            .or_else(|| session.binary().cwd.clone());

        let mut args = request.args.clone();

        // Handle special case for NodeJS debug adapter
        // If only the Node binary path is provided, we set the command to None
        // This prevents the NodeJS REPL from appearing, which is not the desired behavior
        // The expected usage is for users to provide their own Node command, e.g., `node test.js`
        // This allows the NodeJS debug client to attach correctly
        let command = if args.len() > 1 {
            Some(args.remove(0))
        } else {
            None
        };

        let mut envs: HashMap<String, String> = Default::default();
        if let Some(Value::Object(env)) = &request.env {
            for (key, value) in env {
                let value_str = match (key.as_str(), value) {
                    (_, Value::String(value)) => value,
                    _ => continue,
                };

                envs.insert(key.clone(), value_str.clone());
            }
        }

        let shell = project.read(cx).terminal_settings(&cwd, cx).shell.clone();
        let kind = if let Some(command) = command {
            let title = request.title.clone().unwrap_or(command.clone());
            TerminalKind::Task(task::SpawnInTerminal {
                id: task::TaskId("debug".to_string()),
                full_label: title.clone(),
                label: title.clone(),
                command: command.clone(),
                args,
                command_label: title.clone(),
                cwd,
                env: envs,
                use_new_terminal: true,
                allow_concurrent_runs: true,
                reveal: task::RevealStrategy::NoFocus,
                reveal_target: task::RevealTarget::Dock,
                hide: task::HideStrategy::Never,
                shell,
                show_summary: false,
                show_command: false,
                show_rerun: false,
            })
        } else {
            TerminalKind::Shell(cwd.map(|c| c.to_path_buf()))
        };

        let workspace = self.workspace.clone();
        let weak_project = project.downgrade();

        let terminal_task = project.update(cx, |project, cx| {
            project.create_terminal(kind, window.window_handle(), cx)
        });
        let terminal_task = cx.spawn_in(window, async move |_, cx| {
            let terminal = terminal_task.await?;

            let terminal_view = cx.new_window_entity(|window, cx| {
                TerminalView::new(
                    terminal.clone(),
                    workspace,
                    None,
                    weak_project,
                    false,
                    window,
                    cx,
                )
            })?;

            running.update_in(cx, |running, window, cx| {
                running.ensure_pane_item(DebuggerPaneItem::Terminal, window, cx);
                running.debug_terminal.update(cx, |debug_terminal, cx| {
                    debug_terminal.terminal = Some(terminal_view);
                    cx.notify();
                });
            })?;

            terminal.read_with(cx, |terminal, _| {
                terminal
                    .pty_info
                    .pid()
                    .map(|pid| pid.as_u32())
                    .ok_or_else(|| anyhow!("Terminal was spawned but PID was not available"))
            })?
        });

        cx.background_spawn(async move { anyhow::Ok(sender.send(terminal_task.await).await?) })
    }

    fn create_sub_view(
        &self,
        item_kind: DebuggerPaneItem,
        _pane: &Entity<Pane>,
        cx: &mut Context<Self>,
    ) -> Box<dyn ItemHandle> {
        match item_kind {
            DebuggerPaneItem::Console => {
                let weak_console = self.console.clone().downgrade();

                Box::new(SubView::new(
                    self.console.focus_handle(cx),
                    self.console.clone().into(),
                    item_kind,
                    Some(Box::new(move |cx| {
                        weak_console
                            .read_with(cx, |console, cx| console.show_indicator(cx))
                            .unwrap_or_default()
                    })),
                    cx,
                ))
            }
            DebuggerPaneItem::Variables => Box::new(SubView::new(
                self.variable_list.focus_handle(cx),
                self.variable_list.clone().into(),
                item_kind,
                None,
                cx,
            )),
            DebuggerPaneItem::BreakpointList => Box::new(SubView::new(
                self.breakpoint_list.focus_handle(cx),
                self.breakpoint_list.clone().into(),
                item_kind,
                None,
                cx,
            )),
            DebuggerPaneItem::Frames => Box::new(SubView::new(
                self.stack_frame_list.focus_handle(cx),
                self.stack_frame_list.clone().into(),
                item_kind,
                None,
                cx,
            )),
            DebuggerPaneItem::Modules => Box::new(SubView::new(
                self.module_list.focus_handle(cx),
                self.module_list.clone().into(),
                item_kind,
                None,
                cx,
            )),
            DebuggerPaneItem::LoadedSources => Box::new(SubView::new(
                self.loaded_sources_list.focus_handle(cx),
                self.loaded_sources_list.clone().into(),
                item_kind,
                None,
                cx,
            )),
            DebuggerPaneItem::Terminal => Box::new(SubView::new(
                self.debug_terminal.focus_handle(cx),
                self.debug_terminal.clone().into(),
                item_kind,
                None,
                cx,
            )),
        }
    }

    pub(crate) fn ensure_pane_item(
        &mut self,
        item_kind: DebuggerPaneItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.pane_items_status(cx).get(&item_kind) == Some(&true) {
            return;
        };
        let pane = self.panes.last_pane();
        let sub_view = self.create_sub_view(item_kind, &pane, cx);

        pane.update(cx, |pane, cx| {
            pane.add_item_inner(sub_view, false, false, false, None, window, cx);
        })
    }

    pub(crate) fn add_pane_item(
        &mut self,
        item_kind: DebuggerPaneItem,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        debug_assert!(
            item_kind.is_supported(self.session.read(cx).capabilities()),
            "We should only allow adding supported item kinds"
        );

        if let Some(pane) = self.panes.pane_at_pixel_position(position) {
            let sub_view = self.create_sub_view(item_kind, pane, cx);

            pane.update(cx, |pane, cx| {
                pane.add_item(sub_view, false, false, None, window, cx);
            })
        }
    }

    pub(crate) fn pane_items_status(&self, cx: &App) -> IndexMap<DebuggerPaneItem, bool> {
        let caps = self.session.read(cx).capabilities();
        let mut pane_item_status = IndexMap::from_iter(
            DebuggerPaneItem::all()
                .iter()
                .filter(|kind| kind.is_supported(&caps))
                .map(|kind| (*kind, false)),
        );
        self.panes.panes().iter().for_each(|pane| {
            pane.read(cx)
                .items()
                .filter_map(|item| item.act_as::<SubView>(cx))
                .for_each(|view| {
                    pane_item_status.insert(view.read(cx).kind, true);
                });
        });

        pane_item_status
    }

    pub(crate) fn serialize_layout(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self._schedule_serialize.is_none() {
            self._schedule_serialize = Some(cx.spawn_in(window, async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;

                let Some((adapter_name, pane_layout)) = this
                    .read_with(cx, |this, cx| {
                        let adapter_name = this.session.read(cx).adapter();
                        (
                            adapter_name,
                            persistence::build_serialized_layout(
                                &this.panes.root,
                                this.dock_axis,
                                cx,
                            ),
                        )
                    })
                    .ok()
                else {
                    return;
                };

                persistence::serialize_pane_layout(adapter_name, pane_layout)
                    .await
                    .log_err();

                this.update(cx, |this, _| {
                    this._schedule_serialize.take();
                })
                .ok();
            }));
        }
    }

    pub(crate) fn handle_pane_event(
        this: &mut RunningState,
        source_pane: &Entity<Pane>,
        event: &Event,
        window: &mut Window,
        cx: &mut Context<RunningState>,
    ) {
        this.serialize_layout(window, cx);
        match event {
            Event::Remove { .. } => {
                let _did_find_pane = this.panes.remove(&source_pane).is_ok();
                debug_assert!(_did_find_pane);
                cx.notify();
            }
            Event::Focus => {
                this.active_pane = Some(source_pane.clone());
            }
            Event::ZoomIn => {
                source_pane.update(cx, |pane, cx| {
                    pane.set_zoomed(true, cx);
                });
                cx.notify();
            }
            Event::ZoomOut => {
                source_pane.update(cx, |pane, cx| {
                    pane.set_zoomed(false, cx);
                });
                cx.notify();
            }
            _ => {}
        }
    }

    pub(crate) fn activate_pane_in_direction(
        &mut self,
        direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(pane) = self
            .active_pane
            .as_ref()
            .and_then(|pane| self.panes.find_pane_in_direction(pane, direction, cx))
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

    pub(crate) fn go_to_selected_stack_frame(&self, window: &Window, cx: &mut Context<Self>) {
        if self.thread_id.is_some() {
            self.stack_frame_list
                .update(cx, |list, cx| list.go_to_selected_stack_frame(window, cx));
        }
    }

    pub(crate) fn has_open_context_menu(&self, cx: &App) -> bool {
        self.variable_list.read(cx).has_open_context_menu()
    }

    pub fn session(&self) -> &Entity<Session> {
        &self.session
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub(crate) fn selected_stack_frame_id(&self, cx: &App) -> Option<dap::StackFrameId> {
        self.stack_frame_list.read(cx).selected_stack_frame_id()
    }

    #[cfg(test)]
    pub fn stack_frame_list(&self) -> &Entity<StackFrameList> {
        &self.stack_frame_list
    }

    #[cfg(test)]
    pub fn console(&self) -> &Entity<Console> {
        &self.console
    }

    #[cfg(test)]
    pub(crate) fn module_list(&self) -> &Entity<ModuleList> {
        &self.module_list
    }

    pub(crate) fn activate_item(&self, item: DebuggerPaneItem, window: &mut Window, cx: &mut App) {
        let (variable_list_position, pane) = self
            .panes
            .panes()
            .into_iter()
            .find_map(|pane| {
                pane.read(cx)
                    .items_of_type::<SubView>()
                    .position(|view| view.read(cx).view_kind() == item)
                    .map(|view| (view, pane))
            })
            .unwrap();
        pane.update(cx, |this, cx| {
            this.activate_item(variable_list_position, true, true, window, cx);
        })
    }

    #[cfg(test)]
    pub(crate) fn variable_list(&self) -> &Entity<VariableList> {
        &self.variable_list
    }

    #[cfg(test)]
    pub(crate) fn serialized_layout(&self, cx: &App) -> SerializedLayout {
        persistence::build_serialized_layout(&self.panes.root, self.dock_axis, cx)
    }

    pub fn capabilities(&self, cx: &App) -> Capabilities {
        self.session().read(cx).capabilities().clone()
    }

    pub fn select_current_thread(
        &mut self,
        threads: &Vec<(Thread, ThreadStatus)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected_thread = self
            .thread_id
            .and_then(|thread_id| threads.iter().find(|(thread, _)| thread.id == thread_id.0))
            .or_else(|| threads.first());

        let Some((selected_thread, _)) = selected_thread else {
            return;
        };

        if Some(ThreadId(selected_thread.id)) != self.thread_id {
            self.select_thread(ThreadId(selected_thread.id), window, cx);
        }
    }

    pub(crate) fn selected_thread_id(&self) -> Option<ThreadId> {
        self.thread_id
    }

    pub fn thread_status(&self, cx: &App) -> Option<ThreadStatus> {
        self.thread_id
            .map(|id| self.session().read(cx).thread_status(id))
    }

    fn select_thread(&mut self, thread_id: ThreadId, window: &mut Window, cx: &mut Context<Self>) {
        if self.thread_id.is_some_and(|id| id == thread_id) {
            return;
        }

        self.thread_id = Some(thread_id);

        self.stack_frame_list
            .update(cx, |list, cx| list.schedule_refresh(true, window, cx));
    }

    pub fn continue_thread(&mut self, cx: &mut Context<Self>) {
        let Some(thread_id) = self.thread_id else {
            return;
        };

        self.session().update(cx, |state, cx| {
            state.continue_thread(thread_id, cx);
        });
    }

    pub fn step_over(&mut self, cx: &mut Context<Self>) {
        let Some(thread_id) = self.thread_id else {
            return;
        };

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.session().update(cx, |state, cx| {
            state.step_over(thread_id, granularity, cx);
        });
    }

    pub(crate) fn step_in(&mut self, cx: &mut Context<Self>) {
        let Some(thread_id) = self.thread_id else {
            return;
        };

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.session().update(cx, |state, cx| {
            state.step_in(thread_id, granularity, cx);
        });
    }

    pub(crate) fn step_out(&mut self, cx: &mut Context<Self>) {
        let Some(thread_id) = self.thread_id else {
            return;
        };

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.session().update(cx, |state, cx| {
            state.step_out(thread_id, granularity, cx);
        });
    }

    pub(crate) fn step_back(&mut self, cx: &mut Context<Self>) {
        let Some(thread_id) = self.thread_id else {
            return;
        };

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.session().update(cx, |state, cx| {
            state.step_back(thread_id, granularity, cx);
        });
    }

    pub fn restart_session(&self, cx: &mut Context<Self>) {
        self.session().update(cx, |state, cx| {
            state.restart(None, cx);
        });
    }

    pub fn pause_thread(&self, cx: &mut Context<Self>) {
        let Some(thread_id) = self.thread_id else {
            return;
        };

        self.session().update(cx, |state, cx| {
            state.pause_thread(thread_id, cx);
        });
    }

    pub(crate) fn shutdown(&mut self, cx: &mut Context<Self>) {
        self.workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .breakpoint_store()
                    .update(cx, |store, cx| {
                        store.remove_active_position(Some(self.session_id), cx)
                    })
            })
            .log_err();

        self.session.update(cx, |session, cx| {
            session.shutdown(cx).detach();
        })
    }

    pub fn stop_thread(&self, cx: &mut Context<Self>) {
        let Some(thread_id) = self.thread_id else {
            return;
        };

        self.workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .breakpoint_store()
                    .update(cx, |store, cx| {
                        store.remove_active_position(Some(self.session_id), cx)
                    })
            })
            .log_err();

        self.session().update(cx, |state, cx| {
            state.terminate_threads(Some(vec![thread_id; 1]), cx);
        });
    }

    #[expect(
        unused,
        reason = "Support for disconnecting a client is not wired through yet"
    )]
    pub fn disconnect_client(&self, cx: &mut Context<Self>) {
        self.session().update(cx, |state, cx| {
            state.disconnect_client(cx);
        });
    }

    pub fn toggle_ignore_breakpoints(&mut self, cx: &mut Context<Self>) {
        self.session.update(cx, |session, cx| {
            session.toggle_ignore_breakpoints(cx).detach();
        });
    }

    pub(crate) fn thread_dropdown(
        &self,
        window: &mut Window,
        cx: &mut Context<'_, RunningState>,
    ) -> DropdownMenu {
        let state = cx.entity();
        let threads = self.session.update(cx, |this, cx| this.threads(cx));
        let selected_thread_name = threads
            .iter()
            .find(|(thread, _)| self.thread_id.map(|id| id.0) == Some(thread.id))
            .map(|(thread, _)| thread.name.clone())
            .unwrap_or("Threads".to_owned());
        DropdownMenu::new(
            ("thread-list", self.session_id.0),
            selected_thread_name,
            ContextMenu::build_eager(window, cx, move |mut this, _, _| {
                for (thread, _) in threads {
                    let state = state.clone();
                    let thread_id = thread.id;
                    this = this.entry(thread.name, None, move |window, cx| {
                        state.update(cx, |state, cx| {
                            state.select_thread(ThreadId(thread_id), window, cx);
                        });
                    });
                }
                this
            }),
        )
    }

    fn default_pane_layout(
        project: Entity<Project>,
        workspace: &WeakEntity<Workspace>,
        stack_frame_list: &Entity<StackFrameList>,
        variable_list: &Entity<VariableList>,
        module_list: &Entity<ModuleList>,
        loaded_source_list: &Entity<LoadedSourceList>,
        console: &Entity<Console>,
        breakpoints: &Entity<BreakpointList>,
        dock_axis: Axis,
        subscriptions: &mut HashMap<EntityId, Subscription>,
        window: &mut Window,
        cx: &mut Context<'_, RunningState>,
    ) -> Member {
        let leftmost_pane = new_debugger_pane(workspace.clone(), project.clone(), window, cx);
        leftmost_pane.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    this.focus_handle(cx),
                    stack_frame_list.clone().into(),
                    DebuggerPaneItem::Frames,
                    None,
                    cx,
                )),
                true,
                false,
                None,
                window,
                cx,
            );
            this.add_item(
                Box::new(SubView::new(
                    breakpoints.focus_handle(cx),
                    breakpoints.clone().into(),
                    DebuggerPaneItem::BreakpointList,
                    None,
                    cx,
                )),
                true,
                false,
                None,
                window,
                cx,
            );
            this.activate_item(0, false, false, window, cx);
        });
        let center_pane = new_debugger_pane(workspace.clone(), project.clone(), window, cx);

        center_pane.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    variable_list.focus_handle(cx),
                    variable_list.clone().into(),
                    DebuggerPaneItem::Variables,
                    None,
                    cx,
                )),
                true,
                false,
                None,
                window,
                cx,
            );
            this.add_item(
                Box::new(SubView::new(
                    module_list.focus_handle(cx),
                    module_list.clone().into(),
                    DebuggerPaneItem::Modules,
                    None,
                    cx,
                )),
                false,
                false,
                None,
                window,
                cx,
            );

            this.add_item(
                Box::new(SubView::new(
                    loaded_source_list.focus_handle(cx),
                    loaded_source_list.clone().into(),
                    DebuggerPaneItem::LoadedSources,
                    None,
                    cx,
                )),
                false,
                false,
                None,
                window,
                cx,
            );
            this.activate_item(0, false, false, window, cx);
        });

        let rightmost_pane = new_debugger_pane(workspace.clone(), project.clone(), window, cx);
        rightmost_pane.update(cx, |this, cx| {
            let weak_console = console.downgrade();
            this.add_item(
                Box::new(SubView::new(
                    this.focus_handle(cx),
                    console.clone().into(),
                    DebuggerPaneItem::Console,
                    Some(Box::new(move |cx| {
                        weak_console
                            .read_with(cx, |console, cx| console.show_indicator(cx))
                            .unwrap_or_default()
                    })),
                    cx,
                )),
                true,
                false,
                None,
                window,
                cx,
            );
        });

        subscriptions.extend(
            [&leftmost_pane, &center_pane, &rightmost_pane]
                .into_iter()
                .map(|entity| {
                    (
                        entity.entity_id(),
                        cx.subscribe_in(entity, window, Self::handle_pane_event),
                    )
                }),
        );

        let group_root = workspace::PaneAxis::new(
            dock_axis.invert(),
            [leftmost_pane, center_pane, rightmost_pane]
                .into_iter()
                .map(workspace::Member::Pane)
                .collect(),
        );

        Member::Axis(group_root)
    }

    pub(crate) fn invert_axies(&mut self) {
        self.dock_axis = self.dock_axis.invert();
        self.panes.invert_axies();
    }
}

impl EventEmitter<DebugPanelItemEvent> for RunningState {}

impl Focusable for RunningState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub(crate) mod breakpoint_list;
pub(crate) mod console;
pub(crate) mod loaded_source_list;
pub(crate) mod memory_view;
pub(crate) mod module_list;
pub mod stack_frame_list;
pub mod variable_list;
use std::{
    any::Any,
    ops::ControlFlow,
    path::PathBuf,
    sync::{Arc, LazyLock},
    time::Duration,
};

use crate::{
    ToggleExpandItem,
    attach_modal::{AttachModal, ModalIntent},
    new_process_modal::resolve_path,
    persistence::{self, DebuggerPaneItem, SerializedLayout},
    session::running::memory_view::MemoryView,
};

use anyhow::{Context as _, Result, anyhow, bail};
use breakpoint_list::BreakpointList;
use collections::{HashMap, IndexMap};
use console::Console;
use dap::{
    Capabilities, DapRegistry, RunInTerminalRequestArguments, Thread,
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
    DebugScenarioContext, Project, WorktreeId,
    debugger::session::{self, Session, SessionEvent, SessionStateEvent, ThreadId, ThreadStatus},
};
use rpc::proto::ViewId;
use serde_json::Value;
use settings::Settings;
use stack_frame_list::StackFrameList;
use task::{
    BuildTaskDefinition, DebugScenario, Shell, ShellBuilder, SpawnInTerminal, TaskContext,
    ZedDebugConfig, substitute_variables_in_str,
};
use terminal_view::TerminalView;
use ui::{
    FluentBuilder, IntoElement, Render, StatefulInteractiveElement, Tab, Tooltip, VisibleOnHover,
    VisualContext, prelude::*,
};
use util::ResultExt;
use variable_list::VariableList;
use workspace::{
    ActivePaneDecorator, DraggedTab, Item, ItemHandle, Member, Pane, PaneGroup, SplitDirection,
    Workspace, item::TabContentParams, move_item, pane::Event,
};

static PROCESS_ID_PLACEHOLDER: LazyLock<String> =
    LazyLock::new(|| task::VariableName::PickProcessId.template_value());

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
    active_pane: Entity<Pane>,
    pane_close_subscriptions: HashMap<EntityId, Subscription>,
    dock_axis: Axis,
    _schedule_serialize: Option<Task<()>>,
    pub(crate) scenario: Option<DebugScenario>,
    pub(crate) scenario_context: Option<DebugScenarioContext>,
    memory_view: Entity<MemoryView>,
}

impl RunningState {
    pub(crate) fn thread_id(&self) -> Option<ThreadId> {
        self.thread_id
    }

    pub(crate) fn active_pane(&self) -> &Entity<Pane> {
        &self.active_pane
    }
}

impl Render for RunningState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let zoomed_pane = self
            .panes
            .panes()
            .into_iter()
            .find(|pane| pane.read(cx).is_zoomed());

        let active = self.panes.panes().into_iter().next();
        let pane = if let Some(zoomed_pane) = zoomed_pane {
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
            .child(h_flex().flex_1().child(pane))
    }
}

pub(crate) struct SubView {
    inner: AnyView,
    item_focus_handle: FocusHandle,
    kind: DebuggerPaneItem,
    show_indicator: Box<dyn Fn(&App) -> bool>,
    actions: Option<Box<dyn FnMut(&mut Window, &mut App) -> AnyElement>>,
    hovered: bool,
}

impl SubView {
    pub(crate) fn new(
        item_focus_handle: FocusHandle,
        view: AnyView,
        kind: DebuggerPaneItem,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            kind,
            inner: view,
            item_focus_handle,
            show_indicator: Box::new(|_| false),
            actions: None,
            hovered: false,
        })
    }

    pub(crate) fn stack_frame_list(
        stack_frame_list: Entity<StackFrameList>,
        cx: &mut App,
    ) -> Entity<Self> {
        let weak_list = stack_frame_list.downgrade();
        let this = Self::new(
            stack_frame_list.focus_handle(cx),
            stack_frame_list.into(),
            DebuggerPaneItem::Frames,
            cx,
        );

        this.update(cx, |this, _| {
            this.with_actions(Box::new(move |_, cx| {
                weak_list
                    .update(cx, |this, _| this.render_control_strip())
                    .unwrap_or_else(|_| div().into_any_element())
            }));
        });

        this
    }

    pub(crate) fn console(console: Entity<Console>, cx: &mut App) -> Entity<Self> {
        let weak_console = console.downgrade();
        let this = Self::new(
            console.focus_handle(cx),
            console.into(),
            DebuggerPaneItem::Console,
            cx,
        );
        this.update(cx, |this, _| {
            this.with_indicator(Box::new(move |cx| {
                weak_console
                    .read_with(cx, |console, cx| console.show_indicator(cx))
                    .unwrap_or_default()
            }))
        });
        this
    }

    pub(crate) fn breakpoint_list(list: Entity<BreakpointList>, cx: &mut App) -> Entity<Self> {
        let weak_list = list.downgrade();
        let focus_handle = list.focus_handle(cx);
        let this = Self::new(
            focus_handle,
            list.into(),
            DebuggerPaneItem::BreakpointList,
            cx,
        );

        this.update(cx, |this, _| {
            this.with_actions(Box::new(move |_, cx| {
                weak_list
                    .update(cx, |this, _| this.render_control_strip())
                    .unwrap_or_else(|_| div().into_any_element())
            }));
        });
        this
    }

    pub(crate) fn view_kind(&self) -> DebuggerPaneItem {
        self.kind
    }
    pub(crate) fn with_indicator(&mut self, indicator: Box<dyn Fn(&App) -> bool>) {
        self.show_indicator = indicator;
    }
    pub(crate) fn with_actions(
        &mut self,
        actions: Box<dyn FnMut(&mut Window, &mut App) -> AnyElement>,
    ) {
        self.actions = Some(actions);
    }
}
impl Focusable for SubView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.item_focus_handle.clone()
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

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some(self.kind.tab_tooltip())
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
            .id(format!(
                "subview-container-{}",
                self.kind.to_shared_string()
            ))
            .on_hover(cx.listener(|this, hovered, _, cx| {
                this.hovered = *hovered;
                cx.notify();
            }))
            .size_full()
            // Add border unconditionally to prevent layout shifts on focus changes.
            .border_1()
            .when(self.item_focus_handle.contains_focused(window, cx), |el| {
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
            let this_pane = cx.entity();
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
                            .split(&this_pane, &new_pane, split_direction, cx)?;
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
                                true,
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

    cx.new(move |cx| {
        let mut pane = Pane::new(
            workspace.clone(),
            project.clone(),
            Default::default(),
            None,
            NoAction.boxed_clone(),
            true,
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
                        .read_with(cx, |running_state, _| {
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
        pane.set_can_toggle_zoom(false, cx);
        pane.display_nav_history_buttons(None);
        pane.set_custom_drop_handle(cx, custom_drop_handle);
        pane.set_should_display_tab_bar(|_, _| true);
        pane.set_render_tab_bar_buttons(cx, |_, _, _| (None, None));
        pane.set_render_tab_bar(cx, {
            move |pane, window, cx| {
                let active_pane_item = pane.active_item();
                let pane_group_id: SharedString =
                    format!("pane-zoom-button-hover-{}", cx.entity_id()).into();
                let as_subview = active_pane_item
                    .as_ref()
                    .and_then(|item| item.downcast::<SubView>());
                let is_hovered = as_subview
                    .as_ref()
                    .is_some_and(|item| item.read(cx).hovered);

                h_flex()
                    .track_focus(&focus_handle)
                    .group(pane_group_id.clone())
                    .pl_1p5()
                    .pr_1()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().tab_bar_background)
                    .on_action(|_: &menu::Cancel, window, cx| {
                        if cx.stop_active_drag(window) {
                        } else {
                            cx.propagate();
                        }
                    })
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
                                    .is_some_and(|active| active.item_id() == item.item_id());
                                let deemphasized = !pane.has_focus(window, cx);
                                let item_ = item.boxed_clone();
                                div()
                                    .id(format!("debugger_tab_{}", item.item_id().as_u64()))
                                    .p_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .when_some(item.tab_tooltip_text(cx), |this, tooltip| {
                                        this.tooltip(Tooltip::text(tooltip))
                                    })
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
                                            pane: cx.entity(),
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

                        h_flex()
                            .visible_on_hover(pane_group_id)
                            .when(is_hovered, |this| this.visible())
                            .when_some(as_subview.as_ref(), |this, subview| {
                                subview.update(cx, |view, cx| {
                                    let Some(additional_actions) = view.actions.as_mut() else {
                                        return this;
                                    };
                                    this.child(additional_actions(window, cx))
                                })
                            })
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
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(move |pane, _, _, cx| {
                                    let is_zoomed = pane.is_zoomed();
                                    pane.set_zoomed(!is_zoomed, cx);
                                    cx.notify();
                                }))
                                .tooltip({
                                    let focus_handle = focus_handle.clone();
                                    move |_window, cx| {
                                        let zoomed_text =
                                            if zoomed { "Minimize" } else { "Expand" };
                                        Tooltip::for_action_in(
                                            zoomed_text,
                                            &ToggleExpandItem,
                                            &focus_handle,
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
    })
}

pub struct DebugTerminal {
    pub terminal: Option<Entity<TerminalView>>,
    focus_handle: FocusHandle,
    _subscriptions: [Subscription; 1],
}

impl DebugTerminal {
    fn empty(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let focus_subscription = cx.on_focus(&focus_handle, window, |this, window, cx| {
            if let Some(terminal) = this.terminal.as_ref() {
                terminal.focus_handle(cx).focus(window);
            }
        });

        Self {
            terminal: None,
            focus_handle,
            _subscriptions: [focus_subscription],
        }
    }
}

impl gpui::Render for DebugTerminal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .children(self.terminal.clone())
    }
}
impl Focusable for DebugTerminal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl RunningState {
    // todo(debugger) move this to util and make it so you pass a closure to it that converts a string
    pub(crate) fn substitute_variables_in_config(
        config: &mut serde_json::Value,
        context: &TaskContext,
    ) {
        match config {
            serde_json::Value::Object(obj) => {
                obj.values_mut()
                    .for_each(|value| Self::substitute_variables_in_config(value, context));
            }
            serde_json::Value::Array(array) => {
                array
                    .iter_mut()
                    .for_each(|value| Self::substitute_variables_in_config(value, context));
            }
            serde_json::Value::String(s) => {
                // Some built-in zed tasks wrap their arguments in quotes as they might contain spaces.
                if s.starts_with("\"$ZED_") && s.ends_with('"') {
                    *s = s[1..s.len() - 1].to_string();
                }
                if let Some(substituted) = substitute_variables_in_str(s, context) {
                    *s = substituted;
                }
            }
            _ => {}
        }
    }

    pub(crate) fn contains_substring(config: &serde_json::Value, substring: &str) -> bool {
        match config {
            serde_json::Value::Object(obj) => obj
                .values()
                .any(|value| Self::contains_substring(value, substring)),
            serde_json::Value::Array(array) => array
                .iter()
                .any(|value| Self::contains_substring(value, substring)),
            serde_json::Value::String(s) => s.contains(substring),
            _ => false,
        }
    }

    pub(crate) fn substitute_process_id_in_config(config: &mut serde_json::Value, process_id: i32) {
        match config {
            serde_json::Value::Object(obj) => {
                obj.values_mut().for_each(|value| {
                    Self::substitute_process_id_in_config(value, process_id);
                });
            }
            serde_json::Value::Array(array) => {
                array.iter_mut().for_each(|value| {
                    Self::substitute_process_id_in_config(value, process_id);
                });
            }
            serde_json::Value::String(s) => {
                if s.contains(PROCESS_ID_PLACEHOLDER.as_str()) {
                    *s = s.replace(PROCESS_ID_PLACEHOLDER.as_str(), &process_id.to_string());
                }
            }
            _ => {}
        }
    }

    pub(crate) fn relativize_paths(
        key: Option<&str>,
        config: &mut serde_json::Value,
        context: &TaskContext,
    ) {
        match config {
            serde_json::Value::Object(obj) => {
                obj.iter_mut()
                    .for_each(|(key, value)| Self::relativize_paths(Some(key), value, context));
            }
            serde_json::Value::Array(array) => {
                array
                    .iter_mut()
                    .for_each(|value| Self::relativize_paths(None, value, context));
            }
            serde_json::Value::String(s) if key == Some("program") || key == Some("cwd") => {
                // Some built-in zed tasks wrap their arguments in quotes as they might contain spaces.
                if s.starts_with("\"$ZED_") && s.ends_with('"') {
                    *s = s[1..s.len() - 1].to_string();
                }
                resolve_path(s);

                if let Some(substituted) = substitute_variables_in_str(s, context) {
                    *s = substituted;
                }
            }
            _ => {}
        }
    }

    pub(crate) fn new(
        session: Entity<Session>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        parent_terminal: Option<Entity<DebugTerminal>>,
        serialized_pane_layout: Option<SerializedLayout>,
        dock_axis: Axis,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let session_id = session.read(cx).session_id();
        let weak_state = cx.weak_entity();
        let stack_frame_list = cx.new(|cx| {
            StackFrameList::new(
                workspace.clone(),
                session.clone(),
                weak_state.clone(),
                window,
                cx,
            )
        });

        let debug_terminal =
            parent_terminal.unwrap_or_else(|| cx.new(|cx| DebugTerminal::empty(window, cx)));
        let memory_view = cx.new(|cx| {
            MemoryView::new(
                session.clone(),
                workspace.clone(),
                stack_frame_list.downgrade(),
                window,
                cx,
            )
        });
        let variable_list = cx.new(|cx| {
            VariableList::new(
                session.clone(),
                stack_frame_list.clone(),
                memory_view.clone(),
                weak_state.clone(),
                window,
                cx,
            )
        });

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

        let breakpoint_list = BreakpointList::new(
            Some(session.clone()),
            workspace.clone(),
            &project,
            window,
            cx,
        );

        let _subscriptions = vec![
            cx.on_app_quit(move |this, cx| {
                let shutdown = this
                    .session
                    .update(cx, |session, cx| session.on_app_quit(cx));
                let terminal = this.debug_terminal.clone();
                async move {
                    shutdown.await;
                    drop(terminal)
                }
            }),
            cx.observe(&module_list, |_, _, cx| cx.notify()),
            cx.subscribe_in(&session, window, |this, _, event, window, cx| {
                match event {
                    SessionEvent::Stopped(thread_id) => {
                        let panel = this
                            .workspace
                            .update(cx, |workspace, cx| {
                                workspace.open_panel::<crate::DebugPanel>(window, cx);
                                workspace.panel::<crate::DebugPanel>(cx)
                            })
                            .log_err()
                            .flatten();

                        if let Some(thread_id) = thread_id {
                            this.select_thread(*thread_id, window, cx);
                        }
                        if let Some(panel) = panel {
                            let id = this.session_id;
                            window.defer(cx, move |window, cx| {
                                panel.update(cx, |this, cx| {
                                    this.activate_session_by_id(id, window, cx);
                                })
                            })
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
            cx.subscribe(
                &session,
                |this, session, event: &SessionStateEvent, cx| match event {
                    SessionStateEvent::Shutdown if session.read(cx).is_building() => {
                        this.shutdown(cx);
                    }
                    _ => {}
                },
            ),
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
                &memory_view,
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
                &console,
                &breakpoint_list,
                &debug_terminal,
                dock_axis,
                &mut pane_close_subscriptions,
                window,
                cx,
            );

            workspace::PaneGroup::with_root(root)
        };
        let active_pane = panes.first_pane();

        Self {
            memory_view,
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
            active_pane,
            module_list,
            console,
            breakpoint_list,
            loaded_sources_list: loaded_source_list,
            pane_close_subscriptions,
            debug_terminal,
            dock_axis,
            _schedule_serialize: None,
            scenario: None,
            scenario_context: None,
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
        let dap_registry = cx.global::<DapRegistry>().clone();
        let task_store = project.read(cx).task_store().downgrade();
        let weak_project = project.downgrade();
        let weak_workspace = workspace.downgrade();
        let is_windows = project.read(cx).path_style(cx).is_windows();
        let remote_shell = project
            .read(cx)
            .remote_client()
            .as_ref()
            .and_then(|remote| remote.read(cx).shell());

        cx.spawn_in(window, async move |this, cx| {
            let DebugScenario {
                adapter,
                label,
                build,
                mut config,
                tcp_connection,
            } = scenario;
            Self::relativize_paths(None, &mut config, &task_context);
            Self::substitute_variables_in_config(&mut config, &task_context);

            if Self::contains_substring(&config, PROCESS_ID_PLACEHOLDER.as_str()) || label.as_ref().contains(PROCESS_ID_PLACEHOLDER.as_str()) {
                let (tx, rx) = futures::channel::oneshot::channel::<Option<i32>>();

                let weak_workspace_clone = weak_workspace.clone();
                weak_workspace.update_in(cx, |workspace, window, cx| {
                    let project = workspace.project().clone();
                    workspace.toggle_modal(window, cx, |window, cx| {
                        AttachModal::new(
                            ModalIntent::ResolveProcessId(Some(tx)),
                            weak_workspace_clone,
                            project,
                            true,
                            window,
                            cx,
                        )
                    });
                }).ok();

                let Some(process_id) = rx.await.ok().flatten() else {
                    bail!("No process selected with config that contains {}", PROCESS_ID_PLACEHOLDER.as_str())
                };

                Self::substitute_process_id_in_config(&mut config, process_id);
            }

            let request_type = match dap_registry
                .adapter(&adapter)
                .with_context(|| format!("{}: is not a valid adapter name", &adapter)) {
                    Ok(adapter) => adapter.request_kind(&config).await,
                    Err(e) => Err(e)
                };


            let config_is_valid = request_type.is_ok();
            let mut extra_config = Value::Null;
            let build_output = if let Some(build) = build {
                let (task_template, locator_name) = match build {
                    BuildTaskDefinition::Template {
                        task_template,
                        locator_name,
                    } => (task_template, locator_name),
                    BuildTaskDefinition::ByName(ref label) => {
                        let task = task_store.update(cx, |this, cx| {
                            this.task_inventory().map(|inventory| {
                                inventory.read(cx).task_template_by_label(
                                    buffer,
                                    worktree_id,
                                    label,
                                    cx,
                                )
                            })
                        })?;
                        let task = match task {
                            Some(task) => task.await,
                            None => None,
                        }.with_context(|| format!("Couldn't find task template for {build:?}"))?;
                        (task, None)
                    }
                };
                let Some(mut task) = task_template.resolve_task("debug-build-task", &task_context) else {
                    anyhow::bail!("Could not resolve task variables within a debug scenario");
                };

                let locator_name = if let Some(locator_name) = locator_name {
                    extra_config = config.clone();
                    debug_assert!(!config_is_valid);
                    Some(locator_name)
                } else if !config_is_valid {
                    let task = dap_store
                        .update(cx, |this, cx| {
                            this.debug_scenario_for_build_task(
                                task.original_task().clone(),
                                adapter.clone().into(),
                                task.display_label().to_owned().into(),
                                cx,
                            )

                        });
                    if let Ok(t) = task {
                        t.await.and_then(|scenario| {
                            extra_config = scenario.config;
                            match scenario.build {
                                Some(BuildTaskDefinition::Template {
                                    locator_name, ..
                                }) => locator_name,
                                _ => None,
                            }
                        })
                    } else {
                        None
                    }

                } else {
                    None
                };

                if let Some(remote_shell) = remote_shell && task.resolved.shell == Shell::System {
                    task.resolved.shell = Shell::Program(remote_shell);
                }

                let builder = ShellBuilder::new(&task.resolved.shell, is_windows);
                let command_label = builder.command_label(task.resolved.command.as_deref().unwrap_or(""));
                let (command, args) =
                    builder.build(task.resolved.command.clone(), &task.resolved.args);

                let task_with_shell = SpawnInTerminal {
                    command_label,
                    command: Some(command),
                    args,
                    ..task.resolved.clone()
                };
                let terminal = project
                    .update(cx, |project, cx| {
                        project.create_terminal_task(
                            task_with_shell.clone(),
                            cx,
                        )
                    })?.await?;

                let terminal_view = cx.new_window_entity(|window, cx| {
                    TerminalView::new(
                        terminal.clone(),
                        weak_workspace,
                        None,
                        weak_project,
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
                    .context("Failed to wait for completed task")?;

                if !exit_status.success() {
                    anyhow::bail!("Build failed");
                }
                Some((task.resolved.clone(), locator_name, extra_config))
            } else {
                None
            };

            if config_is_valid {
            } else if let Some((task, locator_name, extra_config)) = build_output {
                let locator_name =
                    locator_name.with_context(|| {
                        format!("Could not find a valid locator for a build task and configure is invalid with error: {}", request_type.err()
                            .map(|err| err.to_string())
                            .unwrap_or_default())
                    })?;
                let request = dap_store
                    .update(cx, |this, cx| {
                        this.run_debug_locator(&locator_name, task, cx)
                    })?
                    .await?;

                let zed_config = ZedDebugConfig {
                    label: label.clone(),
                    adapter: adapter.clone(),
                    request,
                    stop_on_entry: None,
                };

                let scenario = dap_registry
                    .adapter(&adapter)
                    .with_context(|| anyhow!("{}: is not a valid adapter name", &adapter))?.config_from_zed_format(zed_config)
                    .await?;
                config = scenario.config;
                util::merge_non_null_json_value_into(extra_config, &mut config);

                Self::substitute_variables_in_config(&mut config, &task_context);
            } else {
                let Err(e) = request_type else {
                    unreachable!();
                };
                anyhow::bail!("Zed cannot determine how to run this debug scenario. `build` field was not provided and Debug Adapter won't accept provided configuration because: {e}");
            };

            Ok(DebugTaskDefinition {
                label,
                adapter: DebugAdapterName(adapter),
                config,
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
            .read_with(cx, |workspace, _| workspace.project().clone())
        else {
            return Task::ready(Err(anyhow!("no workspace")));
        };
        let session = self.session.read(cx);

        let cwd = (!request.cwd.is_empty())
            .then(|| PathBuf::from(&request.cwd))
            .or_else(|| session.binary().unwrap().cwd.clone());

        let mut envs: HashMap<String, String> =
            self.session.read(cx).task_context().project_env.clone();
        if let Some(Value::Object(env)) = &request.env {
            for (key, value) in env {
                let value_str = match (key.as_str(), value) {
                    (_, Value::String(value)) => value,
                    _ => continue,
                };

                envs.insert(key.clone(), value_str.clone());
            }
        }

        let mut args = request.args.clone();
        let command = if envs.contains_key("VSCODE_INSPECTOR_OPTIONS") {
            // Handle special case for NodeJS debug adapter
            // If the Node binary path is provided (possibly with arguments like --experimental-network-inspection),
            // we set the command to None
            // This prevents the NodeJS REPL from appearing, which is not the desired behavior
            // The expected usage is for users to provide their own Node command, e.g., `node test.js`
            // This allows the NodeJS debug client to attach correctly
            if args
                .iter()
                .filter(|arg| !arg.starts_with("--"))
                .collect::<Vec<_>>()
                .len()
                > 1
            {
                Some(args.remove(0))
            } else {
                None
            }
        } else if !args.is_empty() {
            Some(args.remove(0))
        } else {
            None
        };

        let shell = project.read(cx).terminal_settings(&cwd, cx).shell.clone();
        let title = request
            .title
            .clone()
            .filter(|title| !title.is_empty())
            .or_else(|| command.clone())
            .unwrap_or_else(|| "Debug terminal".to_string());
        let kind = task::SpawnInTerminal {
            id: task::TaskId("debug".to_string()),
            full_label: title.clone(),
            label: title.clone(),
            command,
            args,
            command_label: title,
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
        };

        let workspace = self.workspace.clone();
        let weak_project = project.downgrade();

        let terminal_task =
            project.update(cx, |project, cx| project.create_terminal_task(kind, cx));
        let terminal_task = cx.spawn_in(window, async move |_, cx| {
            let terminal = terminal_task.await?;

            let terminal_view = cx.new_window_entity(|window, cx| {
                TerminalView::new(terminal.clone(), workspace, None, weak_project, window, cx)
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
                    .pid()
                    .map(|pid| pid.as_u32())
                    .context("Terminal was spawned but PID was not available")
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
            DebuggerPaneItem::Console => Box::new(SubView::console(self.console.clone(), cx)),
            DebuggerPaneItem::Variables => Box::new(SubView::new(
                self.variable_list.focus_handle(cx),
                self.variable_list.clone().into(),
                item_kind,
                cx,
            )),
            DebuggerPaneItem::BreakpointList => {
                Box::new(SubView::breakpoint_list(self.breakpoint_list.clone(), cx))
            }
            DebuggerPaneItem::Frames => Box::new(SubView::new(
                self.stack_frame_list.focus_handle(cx),
                self.stack_frame_list.clone().into(),
                item_kind,
                cx,
            )),
            DebuggerPaneItem::Modules => Box::new(SubView::new(
                self.module_list.focus_handle(cx),
                self.module_list.clone().into(),
                item_kind,
                cx,
            )),
            DebuggerPaneItem::LoadedSources => Box::new(SubView::new(
                self.loaded_sources_list.focus_handle(cx),
                self.loaded_sources_list.clone().into(),
                item_kind,
                cx,
            )),
            DebuggerPaneItem::Terminal => Box::new(SubView::new(
                self.debug_terminal.focus_handle(cx),
                self.debug_terminal.clone().into(),
                item_kind,
                cx,
            )),
            DebuggerPaneItem::MemoryView => Box::new(SubView::new(
                self.memory_view.focus_handle(cx),
                self.memory_view.clone().into(),
                item_kind,
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
                .filter(|kind| kind.is_supported(caps))
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
                let _did_find_pane = this.panes.remove(source_pane, cx).is_ok();
                debug_assert!(_did_find_pane);
                cx.notify();
            }
            Event::Focus => {
                this.active_pane = source_pane.clone();
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
        let active_pane = self.active_pane.clone();
        if let Some(pane) = self
            .panes
            .find_pane_in_direction(&active_pane, direction, cx)
        {
            pane.update(cx, |pane, cx| {
                pane.focus_active_item(window, cx);
            })
        } else {
            self.workspace
                .update(cx, |workspace, cx| {
                    workspace.activate_pane_in_direction(direction, window, cx)
                })
                .ok();
        }
    }

    pub(crate) fn go_to_selected_stack_frame(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.thread_id.is_some() {
            self.stack_frame_list
                .update(cx, |list, cx| {
                    let Some(stack_frame_id) = list.opened_stack_frame_id() else {
                        return Task::ready(Ok(()));
                    };
                    list.go_to_stack_frame(stack_frame_id, window, cx)
                })
                .detach();
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
        self.stack_frame_list.read(cx).opened_stack_frame_id()
    }

    pub(crate) fn stack_frame_list(&self) -> &Entity<StackFrameList> {
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

    pub(crate) fn activate_item(
        &mut self,
        item: DebuggerPaneItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.ensure_pane_item(item, window, cx);

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
        });
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

    pub fn selected_thread_id(&self) -> Option<ThreadId> {
        self.thread_id
    }

    pub fn thread_status(&self, cx: &App) -> Option<ThreadStatus> {
        self.thread_id
            .map(|id| self.session().read(cx).thread_status(id))
    }

    pub(crate) fn select_thread(
        &mut self,
        thread_id: ThreadId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

    pub fn rerun_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((scenario, context)) = self.scenario.take().zip(self.scenario_context.take())
            && scenario.build.is_some()
        {
            let DebugScenarioContext {
                task_context,
                active_buffer,
                worktree_id,
            } = context;
            let active_buffer = active_buffer.and_then(|buffer| buffer.upgrade());

            self.workspace
                .update(cx, |workspace, cx| {
                    workspace.start_debug_session(
                        scenario,
                        task_context,
                        active_buffer,
                        worktree_id,
                        window,
                        cx,
                    )
                })
                .ok();
        } else {
            self.restart_session(cx);
        }
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

        let is_building = self.session.update(cx, |session, cx| {
            session.shutdown(cx).detach();
            matches!(session.state, session::SessionState::Booting(_))
        });

        if is_building {
            self.debug_terminal.update(cx, |terminal, cx| {
                if let Some(view) = terminal.terminal.as_ref() {
                    view.update(cx, |view, cx| {
                        view.terminal()
                            .update(cx, |terminal, _| terminal.kill_active_task())
                    })
                }
            })
        }
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

    pub fn detach_client(&self, cx: &mut Context<Self>) {
        self.session().update(cx, |state, cx| {
            state.disconnect_client(cx);
        });
    }

    pub fn toggle_ignore_breakpoints(&mut self, cx: &mut Context<Self>) {
        self.session.update(cx, |session, cx| {
            session.toggle_ignore_breakpoints(cx).detach();
        });
    }

    fn default_pane_layout(
        project: Entity<Project>,
        workspace: &WeakEntity<Workspace>,
        stack_frame_list: &Entity<StackFrameList>,
        variable_list: &Entity<VariableList>,
        console: &Entity<Console>,
        breakpoints: &Entity<BreakpointList>,
        debug_terminal: &Entity<DebugTerminal>,
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
                    cx,
                )),
                true,
                false,
                None,
                window,
                cx,
            );
            this.add_item(
                Box::new(SubView::breakpoint_list(breakpoints.clone(), cx)),
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
            let view = SubView::console(console.clone(), cx);

            this.add_item(Box::new(view), true, false, None, window, cx);

            this.add_item(
                Box::new(SubView::new(
                    variable_list.focus_handle(cx),
                    variable_list.clone().into(),
                    DebuggerPaneItem::Variables,
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

        let rightmost_pane = new_debugger_pane(workspace.clone(), project, window, cx);
        rightmost_pane.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    debug_terminal.focus_handle(cx),
                    debug_terminal.clone().into(),
                    DebuggerPaneItem::Terminal,
                    cx,
                )),
                false,
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

    pub(crate) fn invert_axies(&mut self, cx: &mut App) {
        self.dock_axis = self.dock_axis.invert();
        self.panes.invert_axies(cx);
    }
}

impl Focusable for RunningState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

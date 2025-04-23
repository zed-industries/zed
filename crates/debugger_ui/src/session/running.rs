pub(crate) mod breakpoint_list;
pub(crate) mod console;
pub(crate) mod loaded_source_list;
pub(crate) mod module_list;
pub mod stack_frame_list;
pub mod variable_list;

use std::{any::Any, ops::ControlFlow, sync::Arc, time::Duration};

use crate::persistence::{self, DebuggerPaneItem, SerializedPaneLayout};

use super::DebugPanelItemEvent;
use breakpoint_list::BreakpointList;
use collections::{HashMap, IndexMap};
use console::Console;
use dap::{Capabilities, Thread, client::SessionId, debugger_settings::DebuggerSettings};
use gpui::{
    Action as _, AnyView, AppContext, Entity, EntityId, EventEmitter, FocusHandle, Focusable,
    NoAction, Pixels, Point, Subscription, Task, WeakEntity,
};
use loaded_source_list::LoadedSourceList;
use module_list::ModuleList;
use project::{
    Project,
    debugger::session::{Session, SessionEvent, ThreadId, ThreadStatus},
};
use rpc::proto::ViewId;
use settings::Settings;
use stack_frame_list::StackFrameList;
use ui::{
    ActiveTheme, AnyElement, App, Context, ContextMenu, DropdownMenu, FluentBuilder,
    InteractiveElement, IntoElement, Label, LabelCommon as _, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Tab, Window, div, h_flex, v_flex,
};
use util::ResultExt;
use variable_list::VariableList;
use workspace::{
    ActivePaneDecorator, DraggedTab, Item, Member, Pane, PaneGroup, Workspace,
    item::TabContentParams, move_item, pane::Event,
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
    module_list: Entity<module_list::ModuleList>,
    _console: Entity<Console>,
    breakpoint_list: Entity<BreakpointList>,
    panes: PaneGroup,
    pane_close_subscriptions: HashMap<EntityId, Subscription>,
    _schedule_serialize: Option<Task<()>>,
}

impl Render for RunningState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.panes.panes().into_iter().next();
        let x = if let Some(active) = active {
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
    fn tab_content_text(&self, _window: &Window, _cx: &App) -> Option<SharedString> {
        Some(self.kind.to_shared_string())
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
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().child(self.inner.clone())
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
        pane.set_can_split(Some(Arc::new(move |pane, dragged_item, _window, cx| {
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
        })));
        pane.display_nav_history_buttons(None);
        pane.set_custom_drop_handle(cx, custom_drop_handle);
        pane.set_should_display_tab_bar(|_, _| true);
        pane.set_render_tab_bar_buttons(cx, |_, _, _| (None, None));
        pane.set_render_tab_bar(cx, |pane, window, cx| {
            let active_pane_item = pane.active_item();
            h_flex()
                .w_full()
                .px_2()
                .gap_1()
                .h(Tab::container_height(cx))
                .drag_over::<DraggedTab>(|bar, _, _, cx| {
                    bar.bg(cx.theme().colors().drop_target_background)
                })
                .on_drop(
                    cx.listener(move |this, dragged_tab: &DraggedTab, window, cx| {
                        this.drag_split_direction = None;
                        this.handle_tab_drop(dragged_tab, this.items_len(), window, cx)
                    }),
                )
                .bg(cx.theme().colors().tab_bar_background)
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .children(pane.items().enumerate().map(|(ix, item)| {
                    let selected = active_pane_item
                        .as_ref()
                        .map_or(false, |active| active.item_id() == item.item_id());
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
                            if selected {
                                this.bg(cx.theme().colors().tab_active_background)
                            } else {
                                let hover_color = cx.theme().colors().element_hover;
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
                                ..Default::default()
                            },
                            window,
                            cx,
                        ))
                        .on_drop(
                            cx.listener(move |this, dragged_tab: &DraggedTab, window, cx| {
                                this.drag_split_direction = None;
                                this.handle_tab_drop(dragged_tab, ix, window, cx)
                            }),
                        )
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
                }))
                .into_any_element()
        });
        pane
    });

    ret
}
impl RunningState {
    pub fn new(
        session: Entity<Session>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        serialized_pane_layout: Option<SerializedPaneLayout>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let session_id = session.read(cx).session_id();
        let weak_state = cx.weak_entity();
        let stack_frame_list = cx.new(|cx| {
            StackFrameList::new(workspace.clone(), session.clone(), weak_state, window, cx)
        });

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
                            this.select_thread(*thread_id, cx);
                        }
                    }
                    SessionEvent::Threads => {
                        let threads = this.session.update(cx, |this, cx| this.threads(cx));
                        this.select_current_thread(&threads, cx);
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
                serialized_layout,
                &workspace,
                &project,
                &stack_frame_list,
                &variable_list,
                &module_list,
                &console,
                &breakpoint_list,
                &loaded_source_list,
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
            module_list,
            _console: console,
            breakpoint_list,
            loaded_sources_list: loaded_source_list,
            pane_close_subscriptions,
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
            let sub_view = match item_kind {
                DebuggerPaneItem::Console => {
                    let weak_console = self._console.clone().downgrade();

                    Box::new(SubView::new(
                        pane.focus_handle(cx),
                        self._console.clone().into(),
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
            };

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

                let Some((adapter_name, pane_group)) = this
                    .update(cx, |this, cx| {
                        let adapter_name = this.session.read(cx).adapter_name();
                        (
                            adapter_name,
                            persistence::build_serialized_pane_layout(&this.panes.root, cx),
                        )
                    })
                    .ok()
                else {
                    return;
                };

                persistence::serialize_pane_layout(adapter_name, pane_group)
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
        if let Event::Remove { .. } = event {
            let _did_find_pane = this.panes.remove(&source_pane).is_ok();
            debug_assert!(_did_find_pane);
            cx.notify();
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
        &self._console
    }

    #[cfg(test)]
    pub(crate) fn module_list(&self) -> &Entity<ModuleList> {
        &self.module_list
    }

    #[cfg(test)]
    pub(crate) fn activate_modules_list(&self, window: &mut Window, cx: &mut App) {
        let (variable_list_position, pane) = self
            .panes
            .panes()
            .into_iter()
            .find_map(|pane| {
                pane.read(cx)
                    .items_of_type::<SubView>()
                    .position(|view| view.read(cx).view_kind().to_shared_string() == *"Modules")
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

    pub fn capabilities(&self, cx: &App) -> Capabilities {
        self.session().read(cx).capabilities().clone()
    }

    pub fn select_current_thread(
        &mut self,
        threads: &Vec<(Thread, ThreadStatus)>,
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
            self.select_thread(ThreadId(selected_thread.id), cx);
        }
    }

    pub(crate) fn selected_thread_id(&self) -> Option<ThreadId> {
        self.thread_id
    }

    pub fn thread_status(&self, cx: &App) -> Option<ThreadStatus> {
        self.thread_id
            .map(|id| self.session().read(cx).thread_status(id))
    }

    fn select_thread(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) {
        if self.thread_id.is_some_and(|id| id == thread_id) {
            return;
        }

        self.thread_id = Some(thread_id);

        self.stack_frame_list
            .update(cx, |list, cx| list.refresh(cx));
        cx.notify();
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
                    this = this.entry(thread.name, None, move |_, cx| {
                        state.update(cx, |state, cx| {
                            state.select_thread(ThreadId(thread_id), cx);
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
            gpui::Axis::Horizontal,
            [leftmost_pane, center_pane, rightmost_pane]
                .into_iter()
                .map(workspace::Member::Pane)
                .collect(),
        );

        Member::Axis(group_root)
    }
}

impl EventEmitter<DebugPanelItemEvent> for RunningState {}

impl Focusable for RunningState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

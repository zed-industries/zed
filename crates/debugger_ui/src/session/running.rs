mod console;
mod loaded_source_list;
mod module_list;
pub mod stack_frame_list;
pub mod variable_list;

use std::{any::Any, ops::ControlFlow, sync::Arc};

use super::DebugPanelItemEvent;
use collections::HashMap;
use console::Console;
use dap::{Capabilities, Thread, client::SessionId, debugger_settings::DebuggerSettings};
use gpui::{
    Action as _, AnyView, AppContext, Entity, EntityId, EventEmitter, FocusHandle, Focusable,
    NoAction, Subscription, WeakEntity,
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
    App, Context, ContextMenu, DropdownMenu, InteractiveElement, IntoElement, ParentElement,
    Render, SharedString, Styled, Window, div, h_flex, v_flex,
};
use util::ResultExt;
use variable_list::VariableList;
use workspace::{
    ActivePaneDecorator, DraggedTab, Item, Pane, PaneGroup, Workspace, move_item, pane::Event,
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
    _module_list: Entity<module_list::ModuleList>,
    _console: Entity<Console>,
    panes: PaneGroup,
    pane_close_subscriptions: HashMap<EntityId, Subscription>,
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

struct SubView {
    inner: AnyView,
    pane_focus_handle: FocusHandle,
    tab_name: SharedString,
}

impl SubView {
    fn new(
        pane_focus_handle: FocusHandle,
        view: AnyView,
        tab_name: SharedString,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            tab_name,
            inner: view,
            pane_focus_handle,
        })
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
    fn tab_content_text(&self, _window: &Window, _cx: &App) -> Option<SharedString> {
        Some(self.tab_name.clone())
    }
}

impl Render for SubView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().child(self.inner.clone())
    }
}

fn new_debugger_pane(
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
                            cx.subscribe(&new_pane, RunningState::handle_pane_event),
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
        pane
    });

    ret
}
impl RunningState {
    pub fn new(
        session: Entity<Session>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
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

        #[expect(unused)]
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
                    _ => {}
                }
                cx.notify()
            }),
        ];

        let leftmost_pane = new_debugger_pane(workspace.clone(), project.clone(), window, cx);
        leftmost_pane.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    this.focus_handle(cx),
                    stack_frame_list.clone().into(),
                    SharedString::new_static("Frames"),
                    cx,
                )),
                true,
                false,
                None,
                window,
                cx,
            );
        });
        let center_pane = new_debugger_pane(workspace.clone(), project.clone(), window, cx);
        center_pane.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    variable_list.focus_handle(cx),
                    variable_list.clone().into(),
                    SharedString::new_static("Variables"),
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
                    this.focus_handle(cx),
                    module_list.clone().into(),
                    SharedString::new_static("Modules"),
                    cx,
                )),
                true,
                false,
                None,
                window,
                cx,
            );
        });
        let rightmost_pane = new_debugger_pane(workspace.clone(), project.clone(), window, cx);
        rightmost_pane.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    this.focus_handle(cx),
                    console.clone().into(),
                    SharedString::new_static("Console"),
                    cx,
                )),
                true,
                false,
                None,
                window,
                cx,
            );
        });
        let pane_close_subscriptions = HashMap::from_iter(
            [&leftmost_pane, &center_pane, &rightmost_pane]
                .into_iter()
                .map(|entity| {
                    (
                        entity.entity_id(),
                        cx.subscribe(entity, Self::handle_pane_event),
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

        let panes = PaneGroup::with_root(workspace::Member::Axis(group_root));

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
            _module_list: module_list,
            _console: console,
            pane_close_subscriptions,
        }
    }

    fn handle_pane_event(
        this: &mut RunningState,
        source_pane: Entity<Pane>,
        event: &Event,
        cx: &mut Context<RunningState>,
    ) {
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

    pub fn session(&self) -> &Entity<Session> {
        &self.session
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
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
        &self._module_list
    }

    #[cfg(test)]
    pub(crate) fn activate_variable_list(&self, window: &mut Window, cx: &mut App) {
        let (variable_list_position, pane) = self
            .panes
            .panes()
            .into_iter()
            .find_map(|pane| {
                pane.read(cx)
                    .items_of_type::<SubView>()
                    .position(|view| view.read(cx).tab_name == *"Variables")
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

    #[cfg(test)]
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
            ContextMenu::build(window, cx, move |mut this, _, _| {
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
}

impl EventEmitter<DebugPanelItemEvent> for RunningState {}

impl Focusable for RunningState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

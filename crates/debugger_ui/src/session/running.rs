mod console;
mod loaded_source_list;
mod module_list;
pub mod stack_frame_list;
pub mod variable_list;

use std::{any::Any, ops::ControlFlow, sync::Arc};

use super::{DebugPanelItemEvent, ThreadItem};
use console::Console;
use dap::{Capabilities, Thread, client::SessionId, debugger_settings::DebuggerSettings};
use gpui::{
    Action as _, AnyView, AppContext, Entity, EventEmitter, FocusHandle, Focusable, NoAction,
    Subscription, WeakEntity,
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
    ActiveTheme, AnyElement, App, Button, Context, ContextMenu, DropdownMenu, FluentBuilder,
    Indicator, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, div, h_flex, v_flex,
};
use util::ResultExt;
use variable_list::VariableList;
use workspace::{ActivePaneDecorator, DraggedTab, Item, Pane, PaneGroup, Workspace, move_item};

pub struct RunningState {
    session: Entity<Session>,
    thread_id: Option<ThreadId>,
    console: Entity<console::Console>,
    focus_handle: FocusHandle,
    _remote_id: Option<ViewId>,
    show_console_indicator: bool,
    module_list: Entity<module_list::ModuleList>,
    active_thread_item: ThreadItem,
    workspace: WeakEntity<Workspace>,
    session_id: SessionId,
    variable_list: Entity<variable_list::VariableList>,
    _subscriptions: Vec<Subscription>,
    stack_frame_list: Entity<stack_frame_list::StackFrameList>,
    loaded_source_list: Entity<loaded_source_list::LoadedSourceList>,
    panes: PaneGroup,
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

        v_flex()
            .size_full()
            .key_context("DebugSessionItem")
            .track_focus(&self.focus_handle(cx))
            .child(h_flex().flex_1().child(x))

        // let threads = self.session.update(cx, |this, cx| this.threads(cx));
        // self.select_current_thread(&threads, cx);

        // let thread_status = self
        //     .thread_id
        //     .map(|thread_id| self.session.read(cx).thread_status(thread_id))
        //     .unwrap_or(ThreadStatus::Exited);

        // self.variable_list.update(cx, |this, cx| {
        //     this.disabled(thread_status != ThreadStatus::Stopped, cx);
        // });

        // let active_thread_item = &self.active_thread_item;

        // let capabilities = self.capabilities(cx);
        // h_flex()
        //     .key_context("DebugPanelItem")
        //     .track_focus(&self.focus_handle(cx))
        //     .size_full()
        //     .items_start()
        //     .child(
        //         v_flex().size_full().items_start().child(
        //             h_flex()
        //                 .size_full()
        //                 .items_start()
        //                 .p_1()
        //                 .gap_4()
        //                 .child(self.stack_frame_list.clone()),
        //         ),
        //     )
        //     .child(
        //         v_flex()
        //             .border_l_1()
        //             .border_color(cx.theme().colors().border_variant)
        //             .size_full()
        //             .items_start()
        //             .child(
        //                 h_flex()
        //                     .border_b_1()
        //                     .w_full()
        //                     .border_color(cx.theme().colors().border_variant)
        //                     .child(self.render_entry_button(
        //                         &SharedString::from("Variables"),
        //                         ThreadItem::Variables,
        //                         cx,
        //                     ))
        //                     .when(
        //                         capabilities.supports_modules_request.unwrap_or_default(),
        //                         |this| {
        //                             this.child(self.render_entry_button(
        //                                 &SharedString::from("Modules"),
        //                                 ThreadItem::Modules,
        //                                 cx,
        //                             ))
        //                         },
        //                     )
        //                     .when(
        //                         capabilities
        //                             .supports_loaded_sources_request
        //                             .unwrap_or_default(),
        //                         |this| {
        //                             this.child(self.render_entry_button(
        //                                 &SharedString::from("Loaded Sources"),
        //                                 ThreadItem::LoadedSource,
        //                                 cx,
        //                             ))
        //                         },
        //                     )
        //                     .child(self.render_entry_button(
        //                         &SharedString::from("Console"),
        //                         ThreadItem::Console,
        //                         cx,
        //                     )),
        //             )
        //             .when(*active_thread_item == ThreadItem::Variables, |this| {
        //                 this.child(self.variable_list.clone())
        //             })
        //             .when(*active_thread_item == ThreadItem::Modules, |this| {
        //                 this.size_full().child(self.module_list.clone())
        //             })
        //             .when(*active_thread_item == ThreadItem::LoadedSource, |this| {
        //                 this.size_full().child(self.loaded_source_list.clone())
        //             })
        //             .when(*active_thread_item == ThreadItem::Console, |this| {
        //                 this.child(self.console.clone())
        //             }),
        //     )
    }
}

struct SubView {
    inner: AnyView,
    owning_pane: WeakEntity<Pane>,
    pane_focus_handle: FocusHandle,
    tab_name: SharedString,
}

impl SubView {
    fn new(
        owning_pane: WeakEntity<Pane>,
        pane_focus_handle: FocusHandle,
        view: AnyView,
        tab_name: SharedString,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            tab_name,
            inner: view,
            owning_pane,
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

        let root = cx.new(|cx| {
            let mut r = Pane::new(
                workspace.clone(),
                project.clone(),
                Default::default(),
                None,
                NoAction.boxed_clone(),
                window,
                cx,
            );
            r.display_nav_history_buttons(None);
            r.set_can_split(Some(Arc::new(|_, _, _, _| true)));
            r
        });

        let mut panes = PaneGroup::new(root.clone());
        root.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    root.downgrade(),
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
            this.set_custom_drop_handle(cx, Self::custom_drop_handler);
        });
        let second = cx.new(|cx| {
            let mut r = Pane::new(
                workspace.clone(),
                project.clone(),
                Default::default(),
                None,
                NoAction.boxed_clone(),
                window,
                cx,
            );
            r.set_can_split(Some(Arc::new(|_, _, _, _| true)));
            r.display_nav_history_buttons(None);
            r
        });
        second.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    root.downgrade(),
                    this.focus_handle(cx),
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
                    root.downgrade(),
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
            this.set_custom_drop_handle(cx, Self::custom_drop_handler);
        });
        panes
            .split(&root, &second.clone(), workspace::SplitDirection::Right)
            .log_err();
        let third_column = cx.new(|cx| {
            let mut r = Pane::new(
                workspace.clone(),
                project.clone(),
                Default::default(),
                None,
                NoAction.boxed_clone(),
                window,
                cx,
            );
            r.set_can_split(Some(Arc::new(|_, _, _, _| true)));
            r.display_nav_history_buttons(None);
            r
        });
        third_column.update(cx, |this, cx| {
            this.add_item(
                Box::new(SubView::new(
                    root.downgrade(),
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
            this.set_custom_drop_handle(cx, Self::custom_drop_handler);
        });
        panes
            .split(
                &second,
                &third_column.clone(),
                workspace::SplitDirection::Right,
            )
            .log_err();

        Self {
            session,
            console,
            workspace,
            module_list,
            focus_handle,
            variable_list,
            _subscriptions,
            thread_id: None,
            _remote_id: None,
            stack_frame_list,
            loaded_source_list,
            session_id,
            show_console_indicator: false,
            active_thread_item: ThreadItem::Variables,
            panes,
        }
    }

    fn custom_drop_handler(
        pane: &mut Pane,
        any: &dyn Any,
        window: &mut Window,
        cx: &mut Context<Pane>,
    ) -> ControlFlow<()> {
        let Some(tab) = any.downcast_ref::<DraggedTab>() else {
            return ControlFlow::Break(());
        };

        if cx.entity_id() != tab.pane.entity_id() {
            let source_pane = tab.pane.clone();
            let target_pane = cx.entity();
            let target_ix = pane.items_len();

            let item_id = tab.item.item_id();

            window.defer(cx, move |window, cx| {
                move_item(&source_pane, &target_pane, item_id, target_ix, window, cx);
            });
        }
        ControlFlow::Break(())
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
    pub fn set_thread_item(&mut self, thread_item: ThreadItem, cx: &mut Context<Self>) {
        self.active_thread_item = thread_item;
        cx.notify()
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

    fn render_entry_button(
        &self,
        label: &SharedString,
        thread_item: ThreadItem,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let has_indicator =
            matches!(thread_item, ThreadItem::Console) && self.show_console_indicator;

        div()
            .id(label.clone())
            .px_2()
            .py_1()
            .cursor_pointer()
            .border_b_2()
            .when(self.active_thread_item == thread_item, |this| {
                this.border_color(cx.theme().colors().border)
            })
            .child(
                h_flex()
                    .child(Button::new(label.clone(), label.clone()))
                    .when(has_indicator, |this| this.child(Indicator::dot())),
            )
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.active_thread_item = thread_item;

                if matches!(this.active_thread_item, ThreadItem::Console) {
                    this.show_console_indicator = false;
                }

                cx.notify();
            }))
            .into_any_element()
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

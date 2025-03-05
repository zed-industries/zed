mod console;
mod loaded_source_list;
mod module_list;
pub mod stack_frame_list;
pub mod variable_list;

use super::{DebugPanelItemEvent, ThreadItem};
use console::Console;
use dap::{client::SessionId, debugger_settings::DebuggerSettings, Capabilities, Thread};
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, Subscription, WeakEntity};
use loaded_source_list::LoadedSourceList;
use module_list::ModuleList;
use project::debugger::session::{Session, SessionEvent, ThreadId, ThreadStatus};
use rpc::proto::ViewId;
use settings::Settings;
use stack_frame_list::{StackFrameList, StackFrameListEvent};
use ui::{
    div, h_flex, v_flex, ActiveTheme, AnyElement, App, Button, ButtonCommon, Clickable, Context,
    ContextMenu, Disableable, DropdownMenu, FluentBuilder, IconButton, IconName, IconSize,
    Indicator, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Tooltip, Window,
};
use util::ResultExt;
use variable_list::VariableList;
use workspace::{item::ItemEvent, Item, Workspace};

pub struct RunningState {
    session: Entity<Session>,
    thread: Option<(ThreadId, String)>,
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
}

impl Render for RunningState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let threads = self.session.update(cx, |this, cx| this.threads(cx));
        self.select_first_thread(&threads, window, cx);

        let thread_status = self
            .thread
            .as_ref()
            .map(|(thread_id, _)| self.session.read(cx).thread_status(*thread_id))
            .unwrap_or(ThreadStatus::Exited);

        self.variable_list.update(cx, |this, cx| {
            this.disabled(thread_status != ThreadStatus::Stopped, cx);
        });

        let is_terminated = self.session.read(cx).is_terminated();
        let active_thread_item = &self.active_thread_item;

        let has_no_threads = threads.is_empty();
        let capabilities = self.capabilities(cx);
        let state = cx.entity();
        h_flex()
            .when(is_terminated, |this| this.bg(gpui::red()))
            .key_context("DebugPanelItem")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .items_start()
            .child(
                v_flex()
                    .size_full()
                    .items_start()
                    .child(
                        h_flex()
                            .w_full()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .justify_between()
                            .child(
                                h_flex()
                                    .p_1()
                                    .w_full()
                                    .gap_2()
                                    .map(|this| {
                                        if thread_status == ThreadStatus::Running {
                                            this.child(
                                                IconButton::new(
                                                    "debug-pause",
                                                    IconName::DebugPause,
                                                )
                                                .icon_size(IconSize::Small)
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.pause_thread(cx);
                                                }))
                                                .tooltip(move |window, cx| {
                                                    Tooltip::text("Pause program")(window, cx)
                                                }),
                                            )
                                        } else {
                                            this.child(
                                                IconButton::new(
                                                    "debug-continue",
                                                    IconName::DebugContinue,
                                                )
                                                .icon_size(IconSize::Small)
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.continue_thread(cx)
                                                }))
                                                .disabled(thread_status != ThreadStatus::Stopped)
                                                .tooltip(move |window, cx| {
                                                    Tooltip::text("Continue program")(window, cx)
                                                }),
                                            )
                                        }
                                    })
                                    .when(
                                        capabilities.supports_step_back.unwrap_or(false),
                                        |this| {
                                            this.child(
                                                IconButton::new(
                                                    "debug-step-back",
                                                    IconName::DebugStepBack,
                                                )
                                                .icon_size(IconSize::Small)
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.step_back(cx);
                                                }))
                                                .disabled(thread_status != ThreadStatus::Stopped)
                                                .tooltip(move |window, cx| {
                                                    Tooltip::text("Step back")(window, cx)
                                                }),
                                            )
                                        },
                                    )
                                    .child(
                                        IconButton::new("debug-step-over", IconName::DebugStepOver)
                                            .icon_size(IconSize::Small)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.step_over(cx);
                                            }))
                                            .disabled(thread_status != ThreadStatus::Stopped)
                                            .tooltip(move |window, cx| {
                                                Tooltip::text("Step over")(window, cx)
                                            }),
                                    )
                                    .child(
                                        IconButton::new("debug-step-in", IconName::DebugStepInto)
                                            .icon_size(IconSize::Small)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.step_in(cx);
                                            }))
                                            .disabled(thread_status != ThreadStatus::Stopped)
                                            .tooltip(move |window, cx| {
                                                Tooltip::text("Step in")(window, cx)
                                            }),
                                    )
                                    .child(
                                        IconButton::new("debug-step-out", IconName::DebugStepOut)
                                            .icon_size(IconSize::Small)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.step_out(cx);
                                            }))
                                            .disabled(thread_status != ThreadStatus::Stopped)
                                            .tooltip(move |window, cx| {
                                                Tooltip::text("Step out")(window, cx)
                                            }),
                                    )
                                    .child(
                                        IconButton::new("debug-restart", IconName::DebugRestart)
                                            .icon_size(IconSize::Small)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.restart_client(cx);
                                            }))
                                            .disabled(
                                                !capabilities
                                                    .supports_restart_request
                                                    .unwrap_or_default(),
                                            )
                                            .tooltip(move |window, cx| {
                                                Tooltip::text("Restart")(window, cx)
                                            }),
                                    )
                                    .child(
                                        IconButton::new("debug-stop", IconName::DebugStop)
                                            .icon_size(IconSize::Small)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.stop_thread(cx);
                                            }))
                                            .disabled(
                                                thread_status != ThreadStatus::Stopped
                                                    && thread_status != ThreadStatus::Running,
                                            )
                                            .tooltip({
                                                let label = if capabilities
                                                    .supports_terminate_threads_request
                                                    .unwrap_or_default()
                                                {
                                                    "Terminate Thread"
                                                } else {
                                                    "Terminate all Threads"
                                                };
                                                move |window, cx| Tooltip::text(label)(window, cx)
                                            }),
                                    )
                                    .child(
                                        IconButton::new(
                                            "debug-disconnect",
                                            IconName::DebugDisconnect,
                                        )
                                        .icon_size(IconSize::Small)
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.disconnect_client(cx);
                                        }))
                                        .disabled(
                                            thread_status == ThreadStatus::Exited
                                                || thread_status == ThreadStatus::Ended,
                                        )
                                        .tooltip(
                                            move |window, cx| {
                                                Tooltip::text("Disconnect")(window, cx)
                                            },
                                        ),
                                    )
                                    .child(
                                        IconButton::new(
                                            "debug-ignore-breakpoints",
                                            if self.session.read(cx).breakpoints_enabled() {
                                                IconName::DebugBreakpoint
                                            } else {
                                                IconName::DebugIgnoreBreakpoints
                                            },
                                        )
                                        .icon_size(IconSize::Small)
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_ignore_breakpoints(cx);
                                        }))
                                        .disabled(
                                            thread_status == ThreadStatus::Exited
                                                || thread_status == ThreadStatus::Ended,
                                        )
                                        .tooltip(
                                            move |window, cx| {
                                                Tooltip::text("Ignore breakpoints")(window, cx)
                                            },
                                        ),
                                    ),
                            )
                            //.child(h_flex())
                            .child(
                                h_flex().p_1().mx_2().w_3_4().justify_end().child(
                                    DropdownMenu::new(
                                        ("thread-list", self.session_id.0),
                                        self.thread
                                            .as_ref()
                                            .map(|(_, name)| format!("Thread {name}"))
                                            .unwrap_or_else(|| "Threads".into()),
                                        ContextMenu::build(window, cx, move |mut this, _, _| {
                                            for (thread, _) in threads {
                                                let state = state.clone();
                                                let thread_id = thread.id;
                                                let thread_name = SharedString::from(&thread.name);
                                                this = this.entry(
                                                    thread.name,
                                                    None,
                                                    move |window, cx| {
                                                        state.update(cx, |state, cx| {
                                                            state.select_thread(
                                                                ThreadId(thread_id),
                                                                String::from(thread_name.as_ref()),
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    },
                                                );
                                            }
                                            this
                                        }),
                                    )
                                    .disabled(
                                        has_no_threads || thread_status != ThreadStatus::Stopped,
                                    ),
                                ),
                            ),
                    )
                    .child(
                        h_flex()
                            .size_full()
                            .items_start()
                            .p_1()
                            .gap_4()
                            .child(self.stack_frame_list.clone()),
                    ),
            )
            .child(
                v_flex()
                    .border_l_1()
                    .border_color(cx.theme().colors().border_variant)
                    .size_full()
                    .items_start()
                    .child(
                        h_flex()
                            .border_b_1()
                            .w_full()
                            .border_color(cx.theme().colors().border_variant)
                            .child(self.render_entry_button(
                                &SharedString::from("Variables"),
                                ThreadItem::Variables,
                                cx,
                            ))
                            .when(
                                capabilities.supports_modules_request.unwrap_or_default(),
                                |this| {
                                    this.child(self.render_entry_button(
                                        &SharedString::from("Modules"),
                                        ThreadItem::Modules,
                                        cx,
                                    ))
                                },
                            )
                            .when(
                                capabilities
                                    .supports_loaded_sources_request
                                    .unwrap_or_default(),
                                |this| {
                                    this.child(self.render_entry_button(
                                        &SharedString::from("Loaded Sources"),
                                        ThreadItem::LoadedSource,
                                        cx,
                                    ))
                                },
                            )
                            .child(self.render_entry_button(
                                &SharedString::from("Console"),
                                ThreadItem::Console,
                                cx,
                            )),
                    )
                    .when(*active_thread_item == ThreadItem::Variables, |this| {
                        this.size_full().child(self.variable_list.clone())
                    })
                    .when(*active_thread_item == ThreadItem::Modules, |this| {
                        this.size_full().child(self.module_list.clone())
                    })
                    .when(*active_thread_item == ThreadItem::LoadedSource, |this| {
                        this.size_full().child(self.loaded_source_list.clone())
                    })
                    .when(*active_thread_item == ThreadItem::Console, |this| {
                        this.child(self.console.clone())
                    }),
            )
    }
}

impl RunningState {
    pub fn new(
        session: Entity<Session>,
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

        let module_list = cx.new(|cx| ModuleList::new(session.clone(), cx));

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
            cx.subscribe(
                &stack_frame_list,
                move |this: &mut Self, _, event: &StackFrameListEvent, cx| match event {
                    StackFrameListEvent::SelectedStackFrameChanged(_) => this.clear_highlights(cx),
                },
            ),
            cx.subscribe(
                &session,
                move |this: &mut Self, _, event: &SessionEvent, cx| match event {
                    SessionEvent::Stopped => {
                        this.thread.take();
                        cx.notify();
                    }
                    _ => {}
                },
            ),
            cx.observe(&module_list, |_, _, cx| cx.notify()),
            cx.observe(&session, |_, _, cx| cx.notify()),
        ];

        Self {
            session,
            console,
            workspace,
            module_list,
            focus_handle,
            variable_list,
            _subscriptions,
            thread: None,
            _remote_id: None,
            stack_frame_list,
            loaded_source_list,
            session_id,
            show_console_indicator: false,
            active_thread_item: ThreadItem::Variables,
        }
    }

    // pub(crate) fn update_adapter(
    //     &mut self,
    //     update: &UpdateDebugAdapter,
    //     window: &mut Window,
    //     cx: &mut Context<Self>,
    // ) {
    //     if let Some(update_variant) = update.variant.as_ref() {
    //         match update_variant {
    //             proto::update_debug_adapter::Variant::StackFrameList(stack_frame_list) => {
    //                 self.stack_frame_list.update(cx, |this, cx| {
    //                     this.set_from_proto(stack_frame_list.clone(), cx);
    //                 })
    //             }
    //             proto::update_debug_adapter::Variant::ThreadState(thread_state) => {
    //                 self.thread_state.update(cx, |this, _| {
    //                     *this = ThreadState::from_proto(thread_state.clone());
    //                 })
    //             }
    //             proto::update_debug_adapter::Variant::VariableList(variable_list) => self
    //                 .variable_list
    //                 .update(cx, |this, cx| this.set_from_proto(variable_list, cx)),
    //             proto::update_debug_adapter::Variant::AddToVariableList(variables_to_add) => self
    //                 .variable_list
    //                 .update(cx, |this, _| this.add_variables(variables_to_add.clone())),
    //             proto::update_debug_adapter::Variant::Modules(_) => {}
    //             proto::update_debug_adapter::Variant::OutputEvent(output_event) => {
    //                 self.console.update(cx, |this, cx| {
    //                     this.add_message(OutputEvent::from_proto(output_event.clone()), window, cx);
    //                 })
    //             }
    //         }
    //     }
    // }

    pub fn session(&self) -> &Entity<Session> {
        &self.session
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_thread_item(&mut self, thread_item: ThreadItem, cx: &mut Context<Self>) {
        self.active_thread_item = thread_item;
        cx.notify()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn stack_frame_list(&self) -> &Entity<StackFrameList> {
        &self.stack_frame_list
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn console(&self) -> &Entity<Console> {
        &self.console
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn module_list(&self) -> &Entity<ModuleList> {
        &self.module_list
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn variable_list(&self) -> &Entity<VariableList> {
        &self.variable_list
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn are_breakpoints_ignored(&self, cx: &App) -> bool {
        self.session.read(cx).ignore_breakpoints()
    }

    pub fn capabilities(&self, cx: &mut Context<Self>) -> Capabilities {
        self.session().read(cx).capabilities().clone()
    }

    pub fn select_first_thread(
        &mut self,
        threads: &Vec<(Thread, ThreadStatus)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.thread.is_some() {
            return;
        }

        if let Some((thread, _)) = threads.first() {
            self.select_thread(ThreadId(thread.id), thread.name.clone(), window, cx);
        }
    }

    fn select_thread(
        &mut self,
        thread_id: ThreadId,
        thread_name: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.thread = Some((thread_id, thread_name));

        self.stack_frame_list
            .update(cx, |list, cx| list.refresh(window, cx));
        cx.notify();
    }

    fn clear_highlights(&self, _cx: &mut Context<Self>) {
        // TODO(debugger): make this work again
        // if let Some((_, project_path, _)) = self.dap_store.read(cx).active_debug_line() {
        //     self.workspace
        //         .update(cx, |workspace, cx| {
        //             let editor = workspace
        //                 .items_of_type::<Editor>(cx)
        //                 .find(|editor| Some(project_path.clone()) == editor.project_path(cx));

        //             if let Some(editor) = editor {
        //                 editor.update(cx, |editor, cx| {
        //                     editor.clear_row_highlights::<editor::DebugCurrentRowHighlight>();

        //                     cx.notify();
        //                 });
        //             }
        //         })
        //         .ok();
        // }
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
                this.active_thread_item = thread_item.clone();

                if matches!(this.active_thread_item, ThreadItem::Console) {
                    this.show_console_indicator = false;
                }

                cx.notify();
            }))
            .into_any_element()
    }

    pub fn continue_thread(&mut self, cx: &mut Context<Self>) {
        let Some((thread_id, _)) = self.thread else {
            return;
        };

        self.session().update(cx, |state, cx| {
            state.continue_thread(thread_id, cx);
        });
    }

    pub fn step_over(&mut self, cx: &mut Context<Self>) {
        let Some((thread_id, _)) = self.thread else {
            return;
        };

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.session().update(cx, |state, cx| {
            state.step_over(thread_id, granularity, cx);
        });
    }

    pub fn step_in(&mut self, cx: &mut Context<Self>) {
        let Some((thread_id, _)) = self.thread else {
            return;
        };

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.session().update(cx, |state, cx| {
            state.step_in(thread_id, granularity, cx);
        });
    }

    pub fn step_out(&mut self, cx: &mut Context<Self>) {
        let Some((thread_id, _)) = self.thread else {
            return;
        };

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.session().update(cx, |state, cx| {
            state.step_out(thread_id, granularity, cx);
        });
    }

    pub fn step_back(&mut self, cx: &mut Context<Self>) {
        let Some((thread_id, _)) = self.thread else {
            return;
        };

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.session().update(cx, |state, cx| {
            state.step_back(thread_id, granularity, cx);
        });
    }

    pub fn restart_client(&self, cx: &mut Context<Self>) {
        self.session().update(cx, |state, cx| {
            state.restart(None, cx);
        });
    }

    pub fn pause_thread(&self, cx: &mut Context<Self>) {
        let Some((thread_id, _)) = self.thread else {
            return;
        };

        self.session().update(cx, |state, cx| {
            state.pause_thread(thread_id, cx);
        });
    }

    pub(crate) fn shutdown(&mut self, cx: &mut Context<Self>) {
        // todo(debugger): We should check that this debug line is associated with this item
        self.workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .breakpoint_store()
                    .update(cx, |store, cx| store.set_active_position(None, cx))
            })
            .log_err();

        self.session.update(cx, |session, cx| {
            session.shutdown(cx).detach();
        })
    }

    pub fn stop_thread(&self, cx: &mut Context<Self>) {
        let Some((thread_id, _)) = self.thread else {
            return;
        };

        self.workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .breakpoint_store()
                    .update(cx, |store, cx| store.set_active_position(None, cx))
            })
            .log_err();

        self.session().update(cx, |state, cx| {
            state.terminate_threads(Some(vec![thread_id; 1]), cx);
        });
    }

    pub fn disconnect_client(&self, cx: &mut Context<Self>) {
        self.session().update(cx, |state, cx| {
            state.disconnect_client(cx);
        });
    }

    pub fn toggle_ignore_breakpoints(&mut self, cx: &mut Context<Self>) {
        self.session
            .update(cx, |session, cx| session.toggle_ignore_breakpoints(cx))
            .detach_and_log_err(cx);
    }
}

impl EventEmitter<DebugPanelItemEvent> for RunningState {}

impl Focusable for RunningState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for RunningState {
    type Event = DebugPanelItemEvent;

    fn tab_content(
        &self,
        _params: workspace::item::TabContentParams,
        _window: &Window,
        _cx: &App,
    ) -> AnyElement {
        todo!()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        todo!()
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            DebugPanelItemEvent::Close => f(ItemEvent::CloseItem),
            DebugPanelItemEvent::Stopped { .. } => {}
        }
    }
}

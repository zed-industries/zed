use crate::console::Console;
use crate::debugger_panel::{DebugPanel, DebugPanelEvent, ThreadState};
use crate::loaded_source_list::LoadedSourceList;
use crate::module_list::ModuleList;
use crate::stack_frame_list::{StackFrameList, StackFrameListEvent};
use crate::variable_list::VariableList;

use dap::client::{DebugAdapterClientId, ThreadStatus};
use dap::debugger_settings::DebuggerSettings;
use dap::{
    Capabilities, ContinuedEvent, LoadedSourceEvent, ModuleEvent, OutputEvent, OutputEventCategory,
    StoppedEvent, ThreadEvent,
};
use editor::Editor;
use gpui::{
    AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, Model, Subscription, View,
    WeakView,
};
use project::dap_store::DapStore;
use settings::Settings;
use task::DebugAdapterKind;
use ui::WindowContext;
use ui::{prelude::*, Tooltip};
use workspace::item::{Item, ItemEvent};
use workspace::Workspace;

#[derive(Debug)]
pub enum DebugPanelItemEvent {
    Close,
    Stopped { go_to_stack_frame: bool },
}

#[derive(Clone, PartialEq, Eq)]
enum ThreadItem {
    Console,
    LoadedSource,
    Modules,
    Output,
    Variables,
}

pub struct DebugPanelItem {
    thread_id: u64,
    console: View<Console>,
    focus_handle: FocusHandle,
    dap_store: Model<DapStore>,
    output_editor: View<Editor>,
    module_list: View<ModuleList>,
    client_kind: DebugAdapterKind,
    active_thread_item: ThreadItem,
    workspace: WeakView<Workspace>,
    client_id: DebugAdapterClientId,
    thread_state: Model<ThreadState>,
    variable_list: View<VariableList>,
    _subscriptions: Vec<Subscription>,
    stack_frame_list: View<StackFrameList>,
    loaded_source_list: View<LoadedSourceList>,
}

impl DebugPanelItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        debug_panel: View<DebugPanel>,
        workspace: WeakView<Workspace>,
        dap_store: Model<DapStore>,
        thread_state: Model<ThreadState>,
        client_id: &DebugAdapterClientId,
        client_kind: &DebugAdapterKind,
        thread_id: u64,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let this = cx.view().clone();
        let stack_frame_list = cx.new_view(|cx| {
            StackFrameList::new(&workspace, &this, &dap_store, client_id, thread_id, cx)
        });

        let variable_list = cx
            .new_view(|cx| VariableList::new(&stack_frame_list, dap_store.clone(), &client_id, cx));

        let module_list = cx.new_view(|cx| ModuleList::new(dap_store.clone(), &client_id, cx));

        let loaded_source_list =
            cx.new_view(|cx| LoadedSourceList::new(&this, dap_store.clone(), &client_id, cx));

        let console = cx.new_view(|cx| {
            Console::new(
                &stack_frame_list,
                client_id,
                variable_list.clone(),
                dap_store.clone(),
                cx,
            )
        });

        let _subscriptions = vec![
            cx.subscribe(&debug_panel, {
                move |this: &mut Self, _, event: &DebugPanelEvent, cx| {
                    match event {
                        DebugPanelEvent::Stopped {
                            client_id,
                            event,
                            go_to_stack_frame,
                        } => this.handle_stopped_event(client_id, event, *go_to_stack_frame, cx),
                        DebugPanelEvent::Thread((client_id, event)) => {
                            this.handle_thread_event(client_id, event, cx)
                        }
                        DebugPanelEvent::Output((client_id, event)) => {
                            this.handle_output_event(client_id, event, cx)
                        }
                        DebugPanelEvent::Module((client_id, event)) => {
                            this.handle_module_event(client_id, event, cx)
                        }
                        DebugPanelEvent::LoadedSource((client_id, event)) => {
                            this.handle_loaded_source_event(client_id, event, cx)
                        }
                        DebugPanelEvent::ClientStopped(client_id) => {
                            this.handle_client_stopped_event(client_id, cx)
                        }
                        DebugPanelEvent::Continued((client_id, event)) => {
                            this.handle_thread_continued_event(client_id, event, cx);
                        }
                        DebugPanelEvent::Exited(client_id)
                        | DebugPanelEvent::Terminated(client_id) => {
                            this.handle_client_exited_and_terminated_event(client_id, cx);
                        }
                        DebugPanelEvent::CapabilitiesChanged(client_id) => {
                            this.handle_capabilities_changed_event(client_id, cx);
                        }
                    };
                }
            }),
            cx.subscribe(
                &stack_frame_list,
                move |this: &mut Self, _, event: &StackFrameListEvent, cx| match event {
                    StackFrameListEvent::SelectedStackFrameChanged => this.clear_highlights(cx),
                    _ => {}
                },
            ),
        ];

        let output_editor = cx.new_view(|cx| {
            let mut editor = Editor::multi_line(cx);
            editor.set_placeholder_text("Debug adapter and script output", cx);
            editor.set_read_only(true);
            editor.set_show_inline_completions(Some(false), cx);
            editor.set_searchable(false);
            editor.set_auto_replace_emoji_shortcode(false);
            editor.set_show_indent_guides(false, cx);
            editor.set_autoindent(false);
            editor.set_show_gutter(false, cx);
            editor.set_show_line_numbers(false, cx);
            editor
        });

        Self {
            console,
            thread_id,
            dap_store,
            workspace,
            module_list,
            thread_state,
            focus_handle,
            output_editor,
            variable_list,
            _subscriptions,
            stack_frame_list,
            loaded_source_list,
            client_id: *client_id,
            client_kind: client_kind.clone(),
            active_thread_item: ThreadItem::Variables,
        }
    }

    pub fn update_thread_state_status(&mut self, status: ThreadStatus, cx: &mut ViewContext<Self>) {
        self.thread_state.update(cx, |thread_state, cx| {
            thread_state.status = status;

            cx.notify();
        });

        if status == ThreadStatus::Exited
            || status == ThreadStatus::Ended
            || status == ThreadStatus::Stopped
        {
            self.clear_highlights(cx);
        }
    }

    fn should_skip_event(&self, client_id: &DebugAdapterClientId, thread_id: u64) -> bool {
        thread_id != self.thread_id || *client_id != self.client_id
    }

    fn handle_thread_continued_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ContinuedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if self.should_skip_event(client_id, event.thread_id) {
            return;
        }

        self.update_thread_state_status(ThreadStatus::Running, cx);
    }

    fn handle_stopped_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &StoppedEvent,
        go_to_stack_frame: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if self.should_skip_event(client_id, event.thread_id.unwrap_or(self.thread_id)) {
            return;
        }

        cx.emit(DebugPanelItemEvent::Stopped { go_to_stack_frame });
    }

    fn handle_thread_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ThreadEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if self.should_skip_event(client_id, event.thread_id) {
            return;
        }

        self.update_thread_state_status(ThreadStatus::Running, cx);
    }

    fn handle_output_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &OutputEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        // The default value of an event category is console
        // so we assume that is the output type if it doesn't exist
        let output_category = event
            .category
            .as_ref()
            .unwrap_or(&OutputEventCategory::Console);

        match output_category {
            OutputEventCategory::Console => {
                self.console.update(cx, |console, cx| {
                    console.add_message(&event.output, cx);
                });
            }
            _ => {
                self.output_editor.update(cx, |editor, cx| {
                    editor.set_read_only(false);
                    editor.move_to_end(&editor::actions::MoveToEnd, cx);
                    editor.insert(format!("{}\n", &event.output.trim_end()).as_str(), cx);
                    editor.set_read_only(true);

                    cx.notify();
                });
            }
        }
    }

    fn handle_module_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ModuleEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        self.module_list.update(cx, |variable_list, cx| {
            variable_list.on_module_event(event, cx);
        });
    }

    fn handle_loaded_source_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &LoadedSourceEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        self.loaded_source_list
            .update(cx, |loaded_source_list, cx| {
                loaded_source_list.on_loaded_source_event(event, cx);
            });
    }

    fn handle_client_stopped_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ViewContext<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        self.update_thread_state_status(ThreadStatus::Stopped, cx);

        cx.emit(DebugPanelItemEvent::Close);
    }

    fn handle_client_exited_and_terminated_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ViewContext<Self>,
    ) {
        if Self::should_skip_event(self, client_id, self.thread_id) {
            return;
        }

        self.update_thread_state_status(ThreadStatus::Exited, cx);

        cx.emit(DebugPanelItemEvent::Close);
    }

    fn handle_capabilities_changed_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ViewContext<Self>,
    ) {
        if Self::should_skip_event(self, client_id, self.thread_id) {
            return;
        }

        cx.notify();
    }

    pub fn client_id(&self) -> DebugAdapterClientId {
        self.client_id
    }

    pub fn thread_id(&self) -> u64 {
        self.thread_id
    }

    pub fn capabilities(&self, cx: &mut ViewContext<Self>) -> Capabilities {
        self.dap_store
            .read_with(cx, |store, _| store.capabilities_by_id(&self.client_id))
    }

    fn clear_highlights(&self, cx: &mut ViewContext<Self>) {
        self.workspace
            .update(cx, |workspace, cx| {
                let editor_views = workspace
                    .items_of_type::<Editor>(cx)
                    .collect::<Vec<View<Editor>>>();

                for editor_view in editor_views {
                    editor_view.update(cx, |editor, _| {
                        editor.clear_row_highlights::<editor::DebugCurrentRowHighlight>();
                    });
                }
            })
            .ok();
    }

    pub fn go_to_current_stack_frame(&self, cx: &mut ViewContext<Self>) {
        self.stack_frame_list.update(cx, |stack_frame_list, cx| {
            stack_frame_list
                .go_to_stack_frame(cx)
                .detach_and_log_err(cx);
        });
    }

    fn render_entry_button(
        &self,
        label: &SharedString,
        thread_item: ThreadItem,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
        div()
            .id(label.clone())
            .px_2()
            .py_1()
            .cursor_pointer()
            .border_b_2()
            .when(self.active_thread_item == thread_item, |this| {
                this.border_color(cx.theme().colors().border)
            })
            .child(Button::new(label.clone(), label.clone()))
            .on_click(cx.listener(move |this, _, cx| {
                this.active_thread_item = thread_item.clone();

                cx.notify();
            }))
            .into_any_element()
    }

    pub fn continue_thread(&mut self, cx: &mut ViewContext<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);

        self.dap_store.update(cx, |store, cx| {
            store
                .continue_thread(&self.client_id, self.thread_id, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn step_over(&mut self, cx: &mut ViewContext<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.dap_store.update(cx, |store, cx| {
            store
                .step_over(&self.client_id, self.thread_id, granularity, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn step_in(&mut self, cx: &mut ViewContext<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.dap_store.update(cx, |store, cx| {
            store
                .step_in(&self.client_id, self.thread_id, granularity, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn step_out(&mut self, cx: &mut ViewContext<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        self.dap_store.update(cx, |store, cx| {
            store
                .step_out(&self.client_id, self.thread_id, granularity, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn restart_client(&self, cx: &mut ViewContext<Self>) {
        self.dap_store.update(cx, |store, cx| {
            store
                .restart(&self.client_id, None, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn pause_thread(&self, cx: &mut ViewContext<Self>) {
        self.dap_store.update(cx, |store, cx| {
            store
                .pause_thread(&self.client_id, self.thread_id, cx)
                .detach_and_log_err(cx)
        });
    }

    pub fn stop_thread(&self, cx: &mut ViewContext<Self>) {
        self.dap_store.update(cx, |store, cx| {
            store
                .terminate_threads(&self.client_id, Some(vec![self.thread_id; 1]), cx)
                .detach_and_log_err(cx)
        });
    }

    pub fn disconnect_client(&self, cx: &mut ViewContext<Self>) {
        self.dap_store.update(cx, |store, cx| {
            store
                .disconnect_client(&self.client_id, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn toggle_ignore_breakpoints(&mut self, cx: &mut ViewContext<Self>) {
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    project
                        .toggle_ignore_breakpoints(&self.client_id, cx)
                        .detach_and_log_err(cx);
                })
            })
            .ok();
    }
}

impl EventEmitter<DebugPanelItemEvent> for DebugPanelItem {}

impl FocusableView for DebugPanelItem {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for DebugPanelItem {
    type Event = DebugPanelItemEvent;

    fn tab_content(
        &self,
        params: workspace::item::TabContentParams,
        _: &WindowContext,
    ) -> AnyElement {
        Label::new(format!(
            "{} - Thread {}",
            self.client_kind.display_name(),
            self.thread_id
        ))
        .color(if params.selected {
            Color::Default
        } else {
            Color::Muted
        })
        .into_any_element()
    }

    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString> {
        Some(SharedString::from(format!(
            "{} Thread {} - {:?}",
            self.client_kind.display_name(),
            self.thread_id,
            self.thread_state.read(cx).status,
        )))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            DebugPanelItemEvent::Close => f(ItemEvent::CloseItem),
            DebugPanelItemEvent::Stopped { .. } => {}
        }
    }
}

impl Render for DebugPanelItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let thread_status = self.thread_state.read(cx).status;
        let active_thread_item = &self.active_thread_item;

        let capabilities = self.capabilities(cx);

        h_flex()
            .key_context("DebugPanelItem")
            .track_focus(&self.focus_handle)
            .size_full()
            .items_start()
            .child(
                v_flex()
                    .size_full()
                    .items_start()
                    .child(
                        h_flex()
                            .p_1()
                            .border_b_1()
                            .w_full()
                            .border_color(cx.theme().colors().border_variant)
                            .gap_2()
                            .map(|this| {
                                if thread_status == ThreadStatus::Running {
                                    this.child(
                                        IconButton::new("debug-pause", IconName::DebugPause)
                                            .icon_size(IconSize::Small)
                                            .on_click(cx.listener(|this, _, cx| {
                                                this.pause_thread(cx);
                                            }))
                                            .tooltip(move |cx| Tooltip::text("Pause program", cx)),
                                    )
                                } else {
                                    this.child(
                                        IconButton::new("debug-continue", IconName::DebugContinue)
                                            .icon_size(IconSize::Small)
                                            .on_click(
                                                cx.listener(|this, _, cx| this.continue_thread(cx)),
                                            )
                                            .disabled(thread_status != ThreadStatus::Stopped)
                                            .tooltip(move |cx| {
                                                Tooltip::text("Continue program", cx)
                                            }),
                                    )
                                }
                            })
                            .child(
                                IconButton::new("debug-step-over", IconName::DebugStepOver)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.step_over(cx);
                                    }))
                                    .disabled(thread_status != ThreadStatus::Stopped)
                                    .tooltip(move |cx| Tooltip::text("Step over", cx)),
                            )
                            .child(
                                IconButton::new("debug-step-in", IconName::DebugStepInto)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.step_in(cx);
                                    }))
                                    .disabled(thread_status != ThreadStatus::Stopped)
                                    .tooltip(move |cx| Tooltip::text("Step in", cx)),
                            )
                            .child(
                                IconButton::new("debug-step-out", IconName::DebugStepOut)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.step_out(cx);
                                    }))
                                    .disabled(thread_status != ThreadStatus::Stopped)
                                    .tooltip(move |cx| Tooltip::text("Step out", cx)),
                            )
                            .child(
                                IconButton::new("debug-restart", IconName::DebugRestart)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.restart_client(cx);
                                    }))
                                    .disabled(
                                        !capabilities.supports_restart_request.unwrap_or_default(),
                                    )
                                    .tooltip(move |cx| Tooltip::text("Restart", cx)),
                            )
                            .child(
                                IconButton::new("debug-stop", IconName::DebugStop)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.stop_thread(cx);
                                    }))
                                    .disabled(
                                        thread_status != ThreadStatus::Stopped
                                            && thread_status != ThreadStatus::Running,
                                    )
                                    .tooltip(move |cx| Tooltip::text("Stop", cx)),
                            )
                            .child(
                                IconButton::new("debug-disconnect", IconName::DebugDisconnect)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.disconnect_client(cx);
                                    }))
                                    .disabled(
                                        thread_status == ThreadStatus::Exited
                                            || thread_status == ThreadStatus::Ended,
                                    )
                                    .tooltip(move |cx| Tooltip::text("Disconnect", cx)),
                            )
                            .child(
                                IconButton::new(
                                    "debug-ignore-breakpoints",
                                    if self.dap_store.read(cx).ignore_breakpoints(&self.client_id) {
                                        IconName::DebugIgnoreBreakpoints
                                    } else {
                                        IconName::DebugBreakpoint
                                    },
                                )
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(|this, _, cx| {
                                    this.toggle_ignore_breakpoints(cx);
                                }))
                                .disabled(
                                    thread_status == ThreadStatus::Exited
                                        || thread_status == ThreadStatus::Ended,
                                )
                                .tooltip(move |cx| Tooltip::text("Ignore breakpoints", cx)),
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
                            ))
                            .child(self.render_entry_button(
                                &SharedString::from("Output"),
                                ThreadItem::Output,
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
                    .when(*active_thread_item == ThreadItem::Output, |this| {
                        this.child(self.output_editor.clone())
                    })
                    .when(*active_thread_item == ThreadItem::Console, |this| {
                        this.child(self.console.clone())
                    }),
            )
            .into_any()
    }
}

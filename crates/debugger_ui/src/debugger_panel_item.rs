use std::path::Path;

use crate::console::Console;
use crate::debugger_panel::{DebugPanel, DebugPanelEvent, ThreadState};
use crate::variable_list::VariableList;

use dap::client::{DebugAdapterClientId, ThreadStatus};
use dap::debugger_settings::DebuggerSettings;
use dap::{
    Capabilities, ContinuedEvent, OutputEvent, OutputEventCategory, StackFrame, StoppedEvent,
    ThreadEvent,
};
use editor::Editor;
use gpui::{
    list, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, ListState, Model,
    Subscription, View, WeakView,
};
use project::dap_store::DapStore;
use project::ProjectPath;
use settings::Settings;
use task::DebugAdapterKind;
use ui::WindowContext;
use ui::{prelude::*, Tooltip};
use workspace::item::{Item, ItemEvent};
use workspace::Workspace;

pub enum Event {
    Close,
}

#[derive(PartialEq, Eq)]
enum ThreadItem {
    Variables,
    Console,
    Output,
}

pub struct DebugPanelItem {
    thread_id: u64,
    console: View<Console>,
    focus_handle: FocusHandle,
    dap_store: Model<DapStore>,
    stack_frame_list: ListState,
    output_editor: View<Editor>,
    current_stack_frame_id: u64,
    client_kind: DebugAdapterKind,
    active_thread_item: ThreadItem,
    workspace: WeakView<Workspace>,
    client_id: DebugAdapterClientId,
    thread_state: Model<ThreadState>,
    variable_list: View<VariableList>,
    _subscriptions: Vec<Subscription>,
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
        current_stack_frame_id: u64,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let capabilities = dap_store.read(cx).capabilities_by_id(&client_id);

        let variable_list = cx.new_view(|cx| {
            VariableList::new(
                dap_store.clone(),
                &client_id,
                &thread_state,
                &capabilities,
                current_stack_frame_id,
                cx,
            )
        });
        let console = cx.new_view(Console::new);

        let weakview = cx.view().downgrade();
        let stack_frame_list =
            ListState::new(0, gpui::ListAlignment::Top, px(1000.), move |ix, cx| {
                if let Some(view) = weakview.upgrade() {
                    view.update(cx, |view, cx| {
                        view.render_stack_frame(ix, cx).into_any_element()
                    })
                } else {
                    div().into_any()
                }
            });

        let _subscriptions = vec![cx.subscribe(&debug_panel, {
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
                    DebugPanelEvent::ClientStopped(client_id) => {
                        this.handle_client_stopped_event(client_id, cx)
                    }
                    DebugPanelEvent::Continued((client_id, event)) => {
                        this.handle_thread_continued_event(client_id, event, cx);
                    }
                    DebugPanelEvent::Exited(client_id) | DebugPanelEvent::Terminated(client_id) => {
                        this.handle_client_exited_and_terminated_event(client_id, cx);
                    }
                };
            }
        })];

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
            thread_state,
            focus_handle,
            output_editor,
            variable_list,
            _subscriptions,
            stack_frame_list,
            client_id: *client_id,
            current_stack_frame_id,
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

        cx.notify();
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

        let thread_state = self.thread_state.read(cx);

        self.stack_frame_list.reset(thread_state.stack_frames.len());
        if let Some(stack_frame) = thread_state.stack_frames.first() {
            self.update_stack_frame_id(stack_frame.id, go_to_stack_frame, cx);
        };

        cx.notify();
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
            // OutputEventCategory::Stderr => {}
            OutputEventCategory::Stdout => {
                self.output_editor.update(cx, |editor, cx| {
                    editor.set_read_only(false);
                    editor.move_to_end(&editor::actions::MoveToEnd, cx);
                    editor.insert(format!("{}\n", &event.output.trim_end()).as_str(), cx);
                    editor.set_read_only(true);

                    cx.notify();
                });
            }
            // OutputEventCategory::Unknown => {}
            // OutputEventCategory::Important => {}
            OutputEventCategory::Telemetry => {}
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

    fn handle_client_stopped_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ViewContext<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        self.update_thread_state_status(ThreadStatus::Stopped, cx);

        cx.emit(Event::Close);
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

        cx.emit(Event::Close);
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

    fn stack_frame_for_index(&self, ix: usize, cx: &mut ViewContext<Self>) -> StackFrame {
        self.thread_state.read(cx).stack_frames[ix].clone()
    }

    fn update_stack_frame_id(
        &mut self,
        stack_frame_id: u64,
        go_to_stack_frame: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.current_stack_frame_id = stack_frame_id;

        self.variable_list.update(cx, |variable_list, cx| {
            variable_list.update_stack_frame_id(stack_frame_id, cx);
            variable_list.build_entries(true, false, cx);
        });

        if go_to_stack_frame {
            self.go_to_stack_frame(cx);
        }

        cx.notify();
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

    pub fn project_path_from_stack_frame(
        &self,
        stack_frame: &StackFrame,
        cx: &mut ViewContext<Self>,
    ) -> Option<ProjectPath> {
        let path = stack_frame.source.as_ref().and_then(|s| s.path.as_ref())?;

        self.workspace
            .update(cx, |workspace, cx| {
                workspace.project().read_with(cx, |project, cx| {
                    project.project_path_for_absolute_path(&Path::new(path), cx)
                })
            })
            .ok()?
    }

    pub fn go_to_stack_frame(&mut self, cx: &mut ViewContext<Self>) {
        self.clear_highlights(cx);

        let stack_frame = self
            .thread_state
            .read(cx)
            .stack_frames
            .iter()
            .find(|s| s.id == self.current_stack_frame_id)
            .cloned();

        let Some(stack_frame) = stack_frame else {
            return; // this could never happen
        };

        let row = (stack_frame.line.saturating_sub(1)) as u32;
        let column = (stack_frame.column.saturating_sub(1)) as u32;

        let Some(project_path) = self.project_path_from_stack_frame(&stack_frame, cx) else {
            return;
        };

        self.dap_store.update(cx, |store, cx| {
            store.set_active_debug_line(&project_path, row, column, cx);
        });

        cx.spawn({
            let workspace = self.workspace.clone();
            move |_, mut cx| async move {
                let task = workspace.update(&mut cx, |workspace, cx| {
                    workspace.open_path_preview(project_path, None, false, true, cx)
                })?;

                let editor = task.await?.downcast::<Editor>().unwrap();

                workspace.update(&mut cx, |_, cx| {
                    editor.update(cx, |editor, cx| editor.go_to_active_debug_line(cx))
                })
            }
        })
        .detach_and_log_err(cx);
    }

    fn render_stack_frames(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(list(self.stack_frame_list.clone()).size_full())
            .into_any()
    }

    fn render_stack_frame(&self, ix: usize, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let stack_frame = self.stack_frame_for_index(ix, cx);

        let source = stack_frame.source.clone();
        let is_selected_frame = stack_frame.id == self.current_stack_frame_id;

        let formatted_path = format!(
            "{}:{}",
            source.clone().and_then(|s| s.name).unwrap_or_default(),
            stack_frame.line,
        );

        v_flex()
            .rounded_md()
            .w_full()
            .group("")
            .id(("stack-frame", stack_frame.id))
            .tooltip({
                let formatted_path = formatted_path.clone();
                move |cx| Tooltip::text(formatted_path.clone(), cx)
            })
            .p_1()
            .when(is_selected_frame, |this| {
                this.bg(cx.theme().colors().element_hover)
            })
            .on_click(cx.listener({
                let stack_frame_id = stack_frame.id;
                move |this, _, cx| {
                    this.update_stack_frame_id(stack_frame_id, true, cx);
                }
            }))
            .hover(|s| s.bg(cx.theme().colors().element_hover).cursor_pointer())
            .child(
                h_flex()
                    .gap_0p5()
                    .text_ui_sm(cx)
                    .child(stack_frame.name.clone())
                    .child(formatted_path),
            )
            .child(
                h_flex()
                    .text_ui_xs(cx)
                    .text_color(cx.theme().colors().text_muted)
                    .when_some(source.and_then(|s| s.path), |this, path| this.child(path)),
            )
            .into_any()
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

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity();

        self.dap_store.update(cx, |store, cx| {
            store
                .step_over(&self.client_id, self.thread_id, granularity, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn step_in(&mut self, cx: &mut ViewContext<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity();

        self.dap_store.update(cx, |store, cx| {
            store
                .step_in(&self.client_id, self.thread_id, granularity, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn step_out(&mut self, cx: &mut ViewContext<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);

        let granularity = DebuggerSettings::get_global(cx).stepping_granularity();

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
}

impl EventEmitter<Event> for DebugPanelItem {}

impl FocusableView for DebugPanelItem {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for DebugPanelItem {
    type Event = Event;

    fn tab_content(
        &self,
        params: workspace::item::TabContentParams,
        _: &WindowContext,
    ) -> AnyElement {
        Label::new(format!(
            "{:?} - Thread {}",
            self.client_kind, self.thread_id
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
            "{:?} Thread {} - {:?}",
            self.client_kind,
            self.thread_id,
            self.thread_state.read(cx).status,
        )))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            Event::Close => f(ItemEvent::CloseItem),
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
                            ),
                    )
                    .child(
                        h_flex()
                            .size_full()
                            .items_start()
                            .p_1()
                            .gap_4()
                            .child(self.render_stack_frames(cx)),
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
                            .child(
                                div()
                                    .id("variables")
                                    .px_2()
                                    .py_1()
                                    .cursor_pointer()
                                    .border_b_2()
                                    .when(*active_thread_item == ThreadItem::Variables, |this| {
                                        this.border_color(cx.theme().colors().border)
                                    })
                                    .child(Button::new("variables-button", "Variables"))
                                    .on_click(cx.listener(|this, _, _| {
                                        this.active_thread_item = ThreadItem::Variables;
                                    })),
                            )
                            .child(
                                div()
                                    .id("console")
                                    .px_2()
                                    .py_1()
                                    .cursor_pointer()
                                    .border_b_2()
                                    .when(*active_thread_item == ThreadItem::Console, |this| {
                                        this.border_color(cx.theme().colors().border)
                                    })
                                    .child(Button::new("console-button", "Console"))
                                    .on_click(cx.listener(|this, _, _| {
                                        this.active_thread_item = ThreadItem::Console;
                                    })),
                            )
                            .child(
                                div()
                                    .id("output")
                                    .px_2()
                                    .py_1()
                                    .cursor_pointer()
                                    .border_b_2()
                                    .when(*active_thread_item == ThreadItem::Output, |this| {
                                        this.border_color(cx.theme().colors().border)
                                    })
                                    .child(Button::new("output", "Output"))
                                    .on_click(cx.listener(|this, _, _| {
                                        this.active_thread_item = ThreadItem::Output;
                                    })),
                            ),
                    )
                    .when(*active_thread_item == ThreadItem::Variables, |this| {
                        this.size_full().child(self.variable_list.clone())
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

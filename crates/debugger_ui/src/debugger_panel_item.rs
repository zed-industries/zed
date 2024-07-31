use crate::debugger_panel::{DebugPanel, DebugPanelEvent};
use anyhow::Result;
use dap::client::{DebugAdapterClient, DebugAdapterClientId, ThreadState, ThreadStatus};
use dap::{Scope, StackFrame, StoppedEvent, ThreadEvent, Variable};
use gpui::{
    actions, list, AnyElement, AppContext, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, ListState, Subscription, View, WeakView,
};
use std::sync::Arc;
use ui::WindowContext;
use ui::{prelude::*, Tooltip};
use workspace::item::{Item, ItemEvent};

pub struct DebugPanelItem {
    thread_id: u64,
    focus_handle: FocusHandle,
    stack_frame_list: ListState,
    client: Arc<DebugAdapterClient>,
    _subscriptions: Vec<Subscription>,
    current_stack_frame_id: Option<u64>,
}

actions!(
    debug_panel_item,
    [Continue, StepOver, StepIn, StepOut, Restart, Pause, Stop, Disconnect]
);

impl DebugPanelItem {
    pub fn new(
        debug_panel: View<DebugPanel>,
        client: Arc<DebugAdapterClient>,
        thread_id: u64,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
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
                    DebugPanelEvent::Stopped((client_id, event)) => {
                        Self::handle_stopped_event(this, client_id, event, cx)
                    }
                    DebugPanelEvent::Thread((client_id, event)) => {
                        Self::handle_thread_event(this, client_id, event, cx)
                    }
                };
            }
        })];

        Self {
            client,
            thread_id,
            focus_handle,
            _subscriptions,
            stack_frame_list,
            current_stack_frame_id: None,
        }
    }

    fn should_skip_event(
        this: &mut Self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
    ) -> bool {
        thread_id != this.thread_id || *client_id != this.client.id()
    }

    fn handle_stopped_event(
        this: &mut Self,
        client_id: &DebugAdapterClientId,
        event: &StoppedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if Self::should_skip_event(this, client_id, event.thread_id.unwrap_or_default()) {
            return;
        }

        this.stack_frame_list
            .reset(this.current_thread_state().stack_frames.len());
        cx.notify();
    }

    fn handle_thread_event(
        this: &mut Self,
        client_id: &DebugAdapterClientId,
        event: &ThreadEvent,
        _: &mut ViewContext<Self>,
    ) {
        if Self::should_skip_event(this, client_id, event.thread_id) {
            return;
        }

        // TODO: handle thread event
    }
}

impl EventEmitter<ItemEvent> for DebugPanelItem {}

impl FocusableView for DebugPanelItem {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for DebugPanelItem {
    type Event = ItemEvent;

    fn tab_content(
        &self,
        params: workspace::item::TabContentParams,
        _: &WindowContext,
    ) -> AnyElement {
        Label::new(format!(
            "{} - Thread {}",
            self.client.config().id,
            self.thread_id
        ))
        .color(if params.selected {
            Color::Default
        } else {
            Color::Muted
        })
        .into_any_element()
    }

    fn tab_tooltip_text(&self, _: &AppContext) -> Option<SharedString> {
        Some(SharedString::from(format!(
            "{} Thread {} - {:?}",
            self.client.config().id,
            self.thread_id,
            self.current_thread_state().status
        )))
    }
}

impl DebugPanelItem {
    pub fn client(&self) -> Arc<DebugAdapterClient> {
        self.client.clone()
    }

    pub fn thread_id(&self) -> u64 {
        self.thread_id
    }

    fn stack_frame_for_index(&self, ix: usize) -> StackFrame {
        self.client
            .thread_state_by_id(self.thread_id)
            .stack_frames
            .get(ix)
            .cloned()
            .unwrap()
    }

    fn current_thread_state(&self) -> ThreadState {
        self.client
            .thread_states()
            .get(&self.thread_id)
            .cloned()
            .unwrap()
    }

    fn render_stack_frames(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w_1_3()
            .gap_3()
            .h_full()
            .child(list(self.stack_frame_list.clone()).size_full())
            .into_any()
    }

    fn render_stack_frame(&self, ix: usize, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let stack_frame = self.stack_frame_for_index(ix);

        let source = stack_frame.source.clone();
        let selected_frame_id = self.current_stack_frame_id;
        let is_selected_frame = Some(stack_frame.id) == selected_frame_id;

        let formatted_path = format!(
            "{}:{}",
            source.clone().and_then(|s| s.name).unwrap_or_default(),
            stack_frame.line,
        );

        v_flex()
            .rounded_md()
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
                let stack_frame = stack_frame.clone();
                move |this, _, _| {
                    this.current_stack_frame_id = Some(stack_frame.id);

                    // let client = this.client();
                    // DebugPanel::go_to_stack_frame(&stack_frame, client, true, cx)
                    //     .detach_and_log_err(cx);

                    // TODO:
                    // this.go_to_stack_frame(&stack_frame, this.client.clone(), false, cx)
                    //     .detach_and_log_err(cx);
                    // cx.notify();
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

    fn render_scopes(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let thread_state = self.current_thread_state();
        let Some(scopes) = thread_state
            .current_stack_frame_id
            .and_then(|id| thread_state.scopes.get(&id))
        else {
            return div().child("No scopes for this thread yet").into_any();
        };

        div()
            .w_3_4()
            .gap_3()
            .text_ui_sm(cx)
            .children(
                scopes
                    .iter()
                    .map(|scope| self.render_scope(&thread_state, scope, cx)),
            )
            .into_any()
    }

    fn render_scope(
        &self,
        thread_state: &ThreadState,
        scope: &Scope,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        div()
            .id(("scope", scope.variables_reference))
            .p_1()
            .text_ui_sm(cx)
            .hover(|s| s.bg(cx.theme().colors().element_hover).cursor_pointer())
            .child(scope.name.clone())
            .child(
                div()
                    .ml_2()
                    .child(self.render_variables(thread_state, scope, cx)),
            )
            .into_any()
    }

    fn render_variables(
        &self,
        thread_state: &ThreadState,
        scope: &Scope,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let Some(variables) = thread_state.variables.get(&scope.variables_reference) else {
            return div().child("No variables for this thread yet").into_any();
        };

        div()
            .gap_3()
            .text_ui_sm(cx)
            .children(
                variables
                    .iter()
                    .map(|variable| self.render_variable(variable, cx)),
            )
            .into_any()
    }

    fn render_variable(&self, variable: &Variable, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .id(("variable", variable.variables_reference))
            .p_1()
            .gap_1()
            .text_ui_sm(cx)
            .hover(|s| s.bg(cx.theme().colors().element_hover).cursor_pointer())
            .child(variable.name.clone())
            .child(
                div()
                    .text_ui_xs(cx)
                    .text_color(cx.theme().colors().text_muted)
                    .child(variable.value.clone()),
            )
            .into_any()
    }

    // if the debug adapter does not send the continued event,
    // and the status of the thread did not change we have to assume the thread is running
    // so we have to update the thread state status to running
    fn update_thread_state(
        this: WeakView<Self>,
        previous_status: ThreadStatus,
        all_threads_continued: Option<bool>,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            if previous_status == this.current_thread_state().status {
                if all_threads_continued.unwrap_or(false) {
                    for thread in this.client.thread_states().values_mut() {
                        thread.status = ThreadStatus::Running;
                    }
                } else {
                    this.client
                        .update_thread_state_status(this.thread_id, ThreadStatus::Running);
                }

                cx.notify();
            }
        })
    }

    fn handle_continue_action(&mut self, _: &Continue, cx: &mut ViewContext<Self>) {
        let client = self.client.clone();
        let thread_id = self.thread_id;
        let previous_status = self.current_thread_state().status;

        cx.spawn(|this, cx| async move {
            let response = client.resume(thread_id).await?;

            Self::update_thread_state(this, previous_status, response.all_threads_continued, cx)
        })
        .detach_and_log_err(cx);
    }

    fn handle_step_over_action(&mut self, _: &StepOver, cx: &mut ViewContext<Self>) {
        let client = self.client.clone();
        let thread_id = self.thread_id;
        let previous_status = self.current_thread_state().status;

        cx.spawn(|this, cx| async move {
            client.step_over(thread_id).await?;

            Self::update_thread_state(this, previous_status, None, cx)
        })
        .detach_and_log_err(cx);
    }

    fn handle_step_in_action(&mut self, _: &StepIn, cx: &mut ViewContext<Self>) {
        let client = self.client.clone();
        let thread_id = self.thread_id;
        let previous_status = self.current_thread_state().status;

        cx.spawn(|this, cx| async move {
            client.step_in(thread_id).await?;

            Self::update_thread_state(this, previous_status, None, cx)
        })
        .detach_and_log_err(cx);
    }

    fn handle_step_out_action(&mut self, _: &StepOut, cx: &mut ViewContext<Self>) {
        let client = self.client.clone();
        let thread_id = self.thread_id;
        let previous_status = self.current_thread_state().status;

        cx.spawn(|this, cx| async move {
            client.step_out(thread_id).await?;

            Self::update_thread_state(this, previous_status, None, cx)
        })
        .detach_and_log_err(cx);
    }

    fn handle_restart_action(&mut self, _: &Restart, cx: &mut ViewContext<Self>) {
        let client = self.client.clone();

        cx.background_executor()
            .spawn(async move { client.restart().await })
            .detach_and_log_err(cx);
    }

    fn handle_pause_action(&mut self, _: &Pause, cx: &mut ViewContext<Self>) {
        let client = self.client.clone();
        let thread_id = self.thread_id;
        cx.background_executor()
            .spawn(async move { client.pause(thread_id).await })
            .detach_and_log_err(cx);
    }

    fn handle_stop_action(&mut self, _: &Stop, cx: &mut ViewContext<Self>) {
        let client = self.client.clone();
        let thread_ids = vec![self.thread_id; 1];

        cx.background_executor()
            .spawn(async move { client.terminate_threads(Some(thread_ids)).await })
            .detach_and_log_err(cx);
    }

    fn handle_disconnect_action(&mut self, _: &Disconnect, cx: &mut ViewContext<Self>) {
        let client = self.client.clone();
        cx.background_executor()
            .spawn(async move { client.disconnect(None, Some(true), None).await })
            .detach_and_log_err(cx);
    }
}

impl Render for DebugPanelItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let thread_status = self.current_thread_state().status;

        v_flex()
            .key_context("DebugPanelItem")
            .track_focus(&self.focus_handle)
            .capture_action(cx.listener(Self::handle_continue_action))
            .capture_action(cx.listener(Self::handle_step_over_action))
            .capture_action(cx.listener(Self::handle_step_in_action))
            .capture_action(cx.listener(Self::handle_step_out_action))
            .capture_action(cx.listener(Self::handle_restart_action))
            .capture_action(cx.listener(Self::handle_pause_action))
            .capture_action(cx.listener(Self::handle_stop_action))
            .capture_action(cx.listener(Self::handle_disconnect_action))
            .p_2()
            .size_full()
            .items_start()
            .child(
                h_flex()
                    .gap_2()
                    .map(|this| {
                        if thread_status == ThreadStatus::Running {
                            this.child(
                                IconButton::new("debug-pause", IconName::DebugPause)
                                    .on_click(
                                        cx.listener(|_, _, cx| cx.dispatch_action(Box::new(Pause))),
                                    )
                                    .tooltip(move |cx| Tooltip::text("Pause program", cx)),
                            )
                        } else {
                            this.child(
                                IconButton::new("debug-continue", IconName::DebugContinue)
                                    .on_click(cx.listener(|_, _, cx| {
                                        cx.dispatch_action(Box::new(Continue))
                                    }))
                                    .disabled(thread_status != ThreadStatus::Stopped)
                                    .tooltip(move |cx| Tooltip::text("Continue program", cx)),
                            )
                        }
                    })
                    .child(
                        IconButton::new("debug-step-over", IconName::DebugStepOver)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(Box::new(StepOver))),
                            )
                            .disabled(thread_status != ThreadStatus::Stopped)
                            .tooltip(move |cx| Tooltip::text("Step over", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-in", IconName::DebugStepInto)
                            .on_click(cx.listener(|_, _, cx| cx.dispatch_action(Box::new(StepIn))))
                            .disabled(thread_status != ThreadStatus::Stopped)
                            .tooltip(move |cx| Tooltip::text("Step in", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-out", IconName::DebugStepOut)
                            .on_click(cx.listener(|_, _, cx| cx.dispatch_action(Box::new(StepOut))))
                            .disabled(thread_status != ThreadStatus::Stopped)
                            .tooltip(move |cx| Tooltip::text("Step out", cx)),
                    )
                    .child(
                        IconButton::new("debug-restart", IconName::DebugRestart)
                            .on_click(cx.listener(|_, _, cx| cx.dispatch_action(Box::new(Restart))))
                            .disabled(
                                !self
                                    .client
                                    .capabilities()
                                    .supports_restart_request
                                    .unwrap_or_default()
                                    || thread_status != ThreadStatus::Stopped
                                        && thread_status != ThreadStatus::Running,
                            )
                            .tooltip(move |cx| Tooltip::text("Restart", cx)),
                    )
                    .child(
                        IconButton::new("debug-stop", IconName::DebugStop)
                            .on_click(cx.listener(|_, _, cx| cx.dispatch_action(Box::new(Stop))))
                            .disabled(
                                thread_status != ThreadStatus::Stopped
                                    && thread_status != ThreadStatus::Running,
                            )
                            .tooltip(move |cx| Tooltip::text("Stop", cx)),
                    )
                    .child(
                        IconButton::new("debug-disconnect", IconName::DebugDisconnect)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(Box::new(Disconnect))),
                            )
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
                    .child(self.render_stack_frames(cx))
                    .child(self.render_scopes(cx)),
            )
            .into_any()
    }
}

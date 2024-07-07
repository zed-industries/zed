use anyhow::Result;
use dap::client::{DebugAdapterClientId, ThreadState, ThreadStatus};
use dap::requests::{Disconnect, Scopes, StackTrace, Variables};
use dap::{client::DebugAdapterClient, transport::Events};
use dap::{
    DisconnectArguments, Scope, ScopesArguments, StackFrame, StackTraceArguments, StoppedEvent,
    TerminatedEvent, ThreadEvent, ThreadEventReason, Variable, VariablesArguments,
};
use editor::Editor;
use futures::future::try_join_all;
use gpui::{
    actions, list, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, ListState, Subscription, Task, View, ViewContext, WeakView,
};
use std::path::Path;
use std::{collections::HashMap, sync::Arc};
use task::DebugRequestType;
use ui::{prelude::*, Tooltip};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

enum DebugCurrentRowHighlight {}

actions!(
    debug_panel,
    [
        TogglePanel,
        Continue,
        StepOver,
        StepIn,
        StepOut,
        Restart,
        Pause
    ]
);

pub struct DebugPanel {
    pub focus_handle: FocusHandle,
    pub size: Pixels,
    _subscriptions: Vec<Subscription>,
    pub workspace: WeakView<Workspace>,
    pub stack_frame_list: ListState,
}

impl DebugPanel {
    pub fn new(workspace: WeakView<Workspace>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let project = workspace
                .update(cx, |workspace, _| workspace.project().clone())
                .unwrap();

            let _subscriptions = vec![cx.subscribe(&project, {
                move |this: &mut Self, _, event, cx| {
                    if let project::Event::DebugClientEvent { client_id, event } = event {
                        Self::handle_debug_client_events(this, client_id, event, cx);
                    }
                }
            })];

            let view = cx.view().downgrade();
            let stack_frame_list =
                ListState::new(0, gpui::ListAlignment::Top, px(1000.), move |ix, cx| {
                    if let Some(view) = view.upgrade() {
                        view.update(cx, |view, cx| {
                            view.render_stack_frame(ix, cx).into_any_element()
                        })
                    } else {
                        div().into_any()
                    }
                });

            Self {
                focus_handle: cx.focus_handle(),
                size: px(300.),
                _subscriptions,
                workspace: workspace.clone(),
                stack_frame_list,
            }
        })
    }

    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move { cx.update(|cx| DebugPanel::new(workspace, cx)) })
    }

    fn stack_frame_for_index(&self, ix: usize, cx: &mut ViewContext<Self>) -> Option<StackFrame> {
        self.debug_client(cx).and_then(|c| {
            c.current_thread_state()
                .and_then(|f| f.stack_frames.get(ix).cloned())
        })
    }

    fn debug_client(&self, cx: &mut ViewContext<Self>) -> Option<Arc<DebugAdapterClient>> {
        self.workspace
            .update(cx, |this, cx| {
                this.project().read(cx).running_debug_adapters().next()
            })
            .ok()
            .flatten()
    }

    fn debug_client_by_id(
        &self,
        client_id: DebugAdapterClientId,
        cx: &mut ViewContext<Self>,
    ) -> Arc<DebugAdapterClient> {
        self.workspace
            .update(cx, |this, cx| {
                this.project()
                    .read(cx)
                    .debug_adapter_by_id(client_id)
                    .unwrap()
            })
            .unwrap()
    }

    fn render_stack_frames(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_3()
            .h_full()
            .flex_grow()
            .flex_shrink_0()
            .child(list(self.stack_frame_list.clone()).size_full())
            .into_any()
    }

    fn render_stack_frame(&self, ix: usize, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let stack_frame = self.stack_frame_for_index(ix, cx).unwrap();

        let source = stack_frame.source.clone();
        let selected_frame_id = self
            .debug_client(cx)
            .and_then(|c| c.current_stack_frame_id());
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
                move |this, _, cx| {
                    if let Some(client) = this.debug_client(cx) {
                        client.update_current_stack_frame_id(stack_frame.id);
                        this.go_to_stack_frame(&stack_frame, client.clone(), false, cx)
                            .detach_and_log_err(cx);
                        cx.notify();
                    };
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

    fn render_scopes(
        &self,
        thread_state: &ThreadState,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let Some(scopes) = thread_state
            .current_stack_frame_id
            .and_then(|id| thread_state.scopes.get(&id))
        else {
            return div().child("No scopes for this thread yet").into_any();
        };

        div()
            .gap_3()
            .text_ui_sm(cx)
            .children(
                scopes
                    .iter()
                    .map(|scope| self.render_scope(thread_state, scope, cx)),
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

    fn handle_continue_action(&mut self, _: &Continue, cx: &mut ViewContext<Self>) {
        if let Some(client) = self.debug_client(cx) {
            if let Some(thread_id) = client.current_thread_id() {
                cx.background_executor()
                    .spawn(async move { client.resume(thread_id).await })
                    .detach();
            }
        }
    }

    fn handle_step_over_action(&mut self, _: &StepOver, cx: &mut ViewContext<Self>) {
        if let Some(client) = self.debug_client(cx) {
            if let Some(thread_id) = client.current_thread_id() {
                cx.background_executor()
                    .spawn(async move { client.step_over(thread_id).await })
                    .detach();
            }
        }
    }

    fn handle_step_in_action(&mut self, _: &StepIn, cx: &mut ViewContext<Self>) {
        if let Some(client) = self.debug_client(cx) {
            if let Some(thread_id) = client.current_thread_id() {
                cx.background_executor()
                    .spawn(async move { client.step_in(thread_id).await })
                    .detach();
            }
        }
    }

    fn handle_step_out_action(&mut self, _: &StepOut, cx: &mut ViewContext<Self>) {
        if let Some(client) = self.debug_client(cx) {
            if let Some(thread_id) = client.current_thread_id() {
                cx.background_executor()
                    .spawn(async move { client.step_out(thread_id).await })
                    .detach();
            }
        }
    }

    fn handle_restart_action(&mut self, _: &Restart, cx: &mut ViewContext<Self>) {
        if let Some(client) = self.debug_client(cx) {
            if let Some(thread_id) = client.current_thread_id() {
                cx.background_executor()
                    .spawn(async move { client.restart(thread_id).await })
                    .detach();
            }
        }
    }

    fn handle_pause_action(&mut self, _: &Pause, cx: &mut ViewContext<Self>) {
        if let Some(client) = self.debug_client(cx) {
            if let Some(thread_id) = client.current_thread_id() {
                cx.background_executor()
                    .spawn(async move { client.pause(thread_id).await })
                    .detach();
            }
        }
    }

    fn handle_debug_client_events(
        this: &mut Self,
        client_id: &DebugAdapterClientId,
        event: &Events,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            Events::Initialized(_) => {
                let client = this.debug_client_by_id(*client_id, cx);
                cx.spawn(|_, _| async move {
                    // TODO: send all the current breakpoints
                    client.configuration_done().await
                })
                .detach_and_log_err(cx);
            }
            Events::Stopped(event) => Self::handle_stopped_event(this, client_id, event, cx),
            Events::Continued(_) => {}
            Events::Exited(_) => {}
            Events::Terminated(event) => Self::handle_terminated_event(this, client_id, event, cx),
            Events::Thread(event) => Self::handle_thread_event(this, client_id, event, cx),
            Events::Output(_) => {}
            Events::Breakpoint(_) => {}
            Events::Module(_) => {}
            Events::LoadedSource(_) => {}
            Events::Capabilities(_) => {}
            Events::Memory(_) => {}
            Events::Process(_) => {}
            Events::ProgressEnd(_) => {}
            Events::ProgressStart(_) => {}
            Events::ProgressUpdate(_) => {}
            Events::Invalidated(_) => {}
        }
    }

    fn remove_highlights(
        &self,
        client: Arc<DebugAdapterClient>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let mut tasks = Vec::new();
        for (_, thread_state) in client.thread_state().clone() {
            for stack_frame in thread_state.stack_frames {
                tasks.push(self.remove_editor_highlight(&stack_frame, cx));
            }
        }

        cx.spawn(|_, _| async move {
            try_join_all(tasks).await?;

            anyhow::Ok(())
        })
    }

    fn remove_editor_highlight(
        &self,
        stack_frame: &StackFrame,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let path = stack_frame.clone().source.unwrap().path.unwrap().clone();

        cx.spawn(|this, mut cx| async move {
            let task = this.update(&mut cx, |this, cx| {
                this.workspace.update(cx, |workspace, cx| {
                    let project_path = workspace.project().read_with(cx, |project, cx| {
                        project.project_path_for_absolute_path(&Path::new(&path), cx)
                    });

                    if let Some(project_path) = project_path {
                        workspace.open_path(project_path, None, false, cx)
                    } else {
                        Task::ready(Err(anyhow::anyhow!(
                            "No project path found for path: {}",
                            path
                        )))
                    }
                })
            })??;

            let editor = task.await?.downcast::<Editor>().unwrap();

            editor.update(&mut cx, |editor, _| {
                editor.clear_row_highlights::<DebugCurrentRowHighlight>();
            })
        })
    }

    fn go_to_stack_frame(
        &self,
        stack_frame: &StackFrame,
        client: Arc<DebugAdapterClient>,
        clear_highlights: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let path = stack_frame.clone().source.unwrap().path.unwrap().clone();
        let row = (stack_frame.line.saturating_sub(1)) as u32;
        let column = (stack_frame.column.saturating_sub(1)) as u32;

        cx.spawn(move |this, mut cx| async move {
            if clear_highlights {
                this.update(&mut cx, |this, cx| this.remove_highlights(client, cx))?
                    .await?;
            }

            let task = this.update(&mut cx, |this, cx| {
                this.workspace.update(cx, |workspace, cx| {
                    let project_path = workspace.project().read_with(cx, |project, cx| {
                        project.project_path_for_absolute_path(&Path::new(&path), cx)
                    });

                    if let Some(project_path) = project_path {
                        workspace.open_path_preview(project_path, None, false, true, cx)
                    } else {
                        Task::ready(Err(anyhow::anyhow!(
                            "No project path found for path: {}",
                            path
                        )))
                    }
                })
            })??;

            let editor = task.await?.downcast::<Editor>().unwrap();

            this.update(&mut cx, |this, cx| {
                this.workspace.update(cx, |_, cx| {
                    editor.update(cx, |editor, cx| {
                        editor.go_to_line::<DebugCurrentRowHighlight>(
                            row,
                            column,
                            Some(cx.theme().colors().editor_highlighted_line_background),
                            cx,
                        );
                    })
                })
            })??;

            anyhow::Ok(())
        })
    }

    fn handle_stopped_event(
        this: &mut Self,
        client_id: &DebugAdapterClientId,
        event: &StoppedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(thread_id) = event.thread_id else {
            return;
        };

        let client = this.debug_client_by_id(*client_id, cx);

        cx.spawn(|this, mut cx| async move {
            this.update(&mut cx, |this, cx| {
                this.remove_highlights(client.clone(), cx)
            })?
            .await?;

            let stack_trace_response = client
                .request::<StackTrace>(StackTraceArguments {
                    thread_id,
                    start_frame: None,
                    levels: None,
                    format: None,
                })
                .await?;

            let current_stack_frame = stack_trace_response.stack_frames.first().unwrap().clone();
            let mut scope_tasks = Vec::new();
            for stack_frame in stack_trace_response.stack_frames.clone().into_iter() {
                let frame_id = stack_frame.id.clone();
                let client = client.clone();
                scope_tasks.push(async move {
                    anyhow::Ok((
                        frame_id.clone(),
                        client
                            .request::<Scopes>(ScopesArguments { frame_id })
                            .await?,
                    ))
                });
            }

            let mut scopes: HashMap<u64, Vec<Scope>> = HashMap::new();
            let mut variables: HashMap<u64, Vec<Variable>> = HashMap::new();

            let mut variable_tasks = Vec::new();
            for (thread_id, response) in try_join_all(scope_tasks).await? {
                scopes.insert(thread_id, response.scopes.clone());

                for scope in response.scopes {
                    let scope_reference = scope.variables_reference.clone();
                    let client = client.clone();
                    variable_tasks.push(async move {
                        anyhow::Ok((
                            scope_reference.clone(),
                            client
                                .request::<Variables>(VariablesArguments {
                                    variables_reference: scope_reference,
                                    filter: None,
                                    start: None,
                                    count: None,
                                    format: None,
                                })
                                .await?,
                        ))
                    });
                }
            }

            for (scope_reference, response) in try_join_all(variable_tasks).await? {
                variables.insert(scope_reference, response.variables.clone());
            }

            this.update(&mut cx, |this, cx| {
                if let Some(thread_state) = client.thread_state().get_mut(&thread_id) {
                    client.update_current_thread_id(thread_id);

                    thread_state.current_stack_frame_id = Some(current_stack_frame.clone().id);
                    thread_state.stack_frames = stack_trace_response.stack_frames.clone();
                    thread_state.scopes = scopes;
                    thread_state.variables = variables;
                    thread_state.status = ThreadStatus::Stopped;

                    if Some(client.id()) == this.debug_client(cx).map(|c| c.id()) {
                        this.stack_frame_list.reset(thread_state.stack_frames.len());
                        cx.notify();

                        return this.go_to_stack_frame(
                            &current_stack_frame,
                            client.clone(),
                            true,
                            cx,
                        );
                    }
                }

                Task::ready(anyhow::Ok(()))
            })?
            .await
        })
        .detach_and_log_err(cx);
    }

    fn handle_thread_event(
        this: &mut Self,
        client_id: &DebugAdapterClientId,
        event: &ThreadEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let client = this.debug_client_by_id(*client_id, cx);

        let current_thread_id = client.current_thread_id();

        if event.reason == ThreadEventReason::Started {
            client
                .thread_state()
                .insert(event.thread_id, ThreadState::default());

            if current_thread_id.is_none() {
                client.update_current_thread_id(event.thread_id);
            }
        } else {
            if current_thread_id == Some(event.thread_id) {
                client.update_thread_state_status(event.thread_id, ThreadStatus::Ended);

                cx.spawn({
                    let client = client.clone();
                    |this, mut cx| async move {
                        this.update(&mut cx, |this, cx| this.remove_highlights(client, cx))?
                            .await
                    }
                })
                .detach_and_log_err(cx);
            }
        }

        cx.notify();
    }

    fn handle_terminated_event(
        this: &mut Self,
        client_id: &DebugAdapterClientId,
        event: &Option<TerminatedEvent>,
        cx: &mut ViewContext<Self>,
    ) {
        let restart_args = event.clone().and_then(|e| e.restart);
        let client = this.debug_client_by_id(*client_id, cx);

        cx.spawn(|_, _| async move {
            let should_restart = restart_args.is_some();

            client
                .request::<Disconnect>(DisconnectArguments {
                    restart: Some(should_restart),
                    terminate_debuggee: None,
                    suspend_debuggee: None,
                })
                .await?;

            if should_restart {
                match client.request_type() {
                    DebugRequestType::Launch => client.launch(restart_args).await,
                    DebugRequestType::Attach => client.attach(restart_args).await,
                }
            } else {
                anyhow::Ok(())
            }
        })
        .detach_and_log_err(cx);
    }

    fn disable_button(&self, cx: &mut ViewContext<Self>) -> bool {
        let thread_state = self.debug_client(cx).and_then(|c| c.current_thread_state());
        thread_state
            .and_then(|s| Some(s.status != ThreadStatus::Stopped))
            .unwrap_or(true)
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}

impl FocusableView for DebugPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for DebugPanel {
    fn persistent_name() -> &'static str {
        "DebugPanel"
    }

    fn position(&self, _cx: &WindowContext) -> DockPosition {
        DockPosition::Bottom
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        position == DockPosition::Bottom
    }

    fn set_position(&mut self, _position: DockPosition, _cx: &mut ViewContext<Self>) {}

    fn size(&self, _cx: &WindowContext) -> Pixels {
        self.size
    }

    fn set_size(&mut self, size: Option<Pixels>, _cx: &mut ViewContext<Self>) {
        self.size = size.unwrap();
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        None
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        None
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(TogglePanel)
    }

    fn icon_label(&self, _: &WindowContext) -> Option<String> {
        None
    }

    fn is_zoomed(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn starts_open(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn set_zoomed(&mut self, _zoomed: bool, _cx: &mut ViewContext<Self>) {}

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}
}

impl Render for DebugPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let disable_button = self.disable_button(cx);

        v_flex()
            .key_context("DebugPanel")
            .track_focus(&self.focus_handle)
            .capture_action(cx.listener(Self::handle_continue_action))
            .capture_action(cx.listener(Self::handle_step_over_action))
            .capture_action(cx.listener(Self::handle_step_in_action))
            .capture_action(cx.listener(Self::handle_step_out_action))
            .capture_action(cx.listener(Self::handle_restart_action))
            .capture_action(cx.listener(Self::handle_pause_action))
            .items_start()
            .child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .child(
                        IconButton::new("debug-continue", IconName::DebugContinue)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(Continue.boxed_clone())),
                            )
                            .disabled(disable_button)
                            .tooltip(move |cx| Tooltip::text("Continue debug", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-over", IconName::DebugStepOver)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(StepOver.boxed_clone())),
                            )
                            .disabled(disable_button)
                            .tooltip(move |cx| Tooltip::text("Step over", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-in", IconName::DebugStepInto)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(StepIn.boxed_clone())),
                            )
                            .disabled(disable_button)
                            .tooltip(move |cx| Tooltip::text("Go in", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-out", IconName::DebugStepOut)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(StepOut.boxed_clone())),
                            )
                            .disabled(disable_button)
                            .tooltip(move |cx| Tooltip::text("Go out", cx)),
                    )
                    .child(
                        IconButton::new("debug-restart", IconName::DebugRestart)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(Restart.boxed_clone())),
                            )
                            .disabled(disable_button)
                            .tooltip(move |cx| Tooltip::text("Restart", cx)),
                    )
                    .child(
                        IconButton::new("debug-pause", IconName::DebugStop)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(Pause.boxed_clone())),
                            )
                            .disabled(disable_button)
                            .tooltip(move |cx| Tooltip::text("Pause", cx)),
                    ),
            )
            .child(h_flex().size_full().items_start().p_1().gap_4().when_some(
                self.debug_client(cx).and_then(|c| c.current_thread_state()),
                |this, thread_state| {
                    this.child(self.render_stack_frames(cx))
                        .child(self.render_scopes(&thread_state, cx))
                },
            ))
            .into_any()
    }
}

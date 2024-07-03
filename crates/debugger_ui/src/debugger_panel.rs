use anyhow::Result;
use dap::client::DebugAdapterClientId;
use dap::requests::{BreakpointLocations, Scopes, StackTrace, Variables};
use dap::{client::DebugAdapterClient, transport::Events};
use dap::{
    BreakpointLocationsArguments, Scope, ScopesArguments, StackFrame, StackTraceArguments,
    StoppedEvent, ThreadEventReason, Variable, VariablesArguments,
};
use gpui::{
    actions, list, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, ListState, Model, Subscription, Task, View, ViewContext, WeakView,
};
use project::Event::DebugClientEvent;
use project::Project;
use std::{collections::HashMap, sync::Arc};
use ui::{prelude::*, Tooltip};
use workspace::pane::Event;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

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

#[derive(Default)]
struct ThreadState {
    pub stack_frames: Vec<StackFrame>,
    pub scopes: HashMap<u64, Vec<Scope>>, // stack_frame_id -> scopes
    pub variables: HashMap<u64, Vec<Variable>>, // scope.variable_reference -> variables
}

pub struct DebugPanel {
    pub position: DockPosition,
    pub zoomed: bool,
    pub active: bool,
    pub focus_handle: FocusHandle,
    pub size: Pixels,
    _subscriptions: Vec<Subscription>,
    pub current_thread_id: Option<u64>,
    pub current_stack_frame_id: Option<u64>,
    pub workspace: WeakView<Workspace>,
    thread_state: HashMap<u64, ThreadState>,
    pub stack_frame_list: ListState,
}

impl DebugPanel {
    pub fn new(workspace: WeakView<Workspace>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let project = workspace
                .update(cx, |workspace, _| workspace.project().clone())
                .unwrap();

            let _subscriptions = vec![cx.subscribe(&project, {
                move |this: &mut Self, model, event, cx| {
                    if let project::Event::DebugClientEvent { client_id, event } = event {
                        Self::handle_debug_client_events(this, model, client_id, event, cx);
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
                position: DockPosition::Bottom,
                zoomed: false,
                active: false,
                focus_handle: cx.focus_handle(),
                size: px(300.),
                _subscriptions,
                current_thread_id: None,
                current_stack_frame_id: None,
                workspace: workspace.clone(),
                thread_state: Default::default(),
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

    fn stack_frame_for_index(&self, ix: usize) -> &StackFrame {
        &self
            .current_thread_id
            .and_then(|id| {
                self.thread_state
                    .get(&id)
                    .and_then(|state| state.stack_frames.get(ix))
            })
            .unwrap()
    }

    fn debug_adapter(&self, cx: &mut ViewContext<Self>) -> Arc<DebugAdapterClient> {
        self.workspace
            .update(cx, |this, cx| {
                this.project()
                    .read(cx)
                    .running_debug_adapters()
                    .next()
                    .unwrap()
            })
            .unwrap()
    }

    fn debug_adapter_by_id(
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
        let stack_frame = self.stack_frame_for_index(ix);

        let source = stack_frame.source.clone();

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
        let Some(scopes) = self
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
        let client = self.debug_adapter(cx);
        if let Some(thread_id) = self.current_thread_id {
            cx.background_executor()
                .spawn(async move { client.resume(thread_id).await })
                .detach();
        }
    }

    fn handle_step_over_action(&mut self, _: &StepOver, cx: &mut ViewContext<Self>) {
        let client = self.debug_adapter(cx);
        if let Some(thread_id) = self.current_thread_id {
            cx.background_executor()
                .spawn(async move { client.step_over(thread_id).await })
                .detach();
        }
    }

    fn handle_step_in_action(&mut self, _: &StepIn, cx: &mut ViewContext<Self>) {
        let client = self.debug_adapter(cx);
        if let Some(thread_id) = self.current_thread_id {
            cx.background_executor()
                .spawn(async move { client.step_in(thread_id).await })
                .detach();
        }
    }

    fn handle_step_out_action(&mut self, _: &StepOut, cx: &mut ViewContext<Self>) {
        let client = self.debug_adapter(cx);
        if let Some(thread_id) = self.current_thread_id {
            cx.background_executor()
                .spawn(async move { client.step_out(thread_id).await })
                .detach();
        }
    }

    fn handle_restart_action(&mut self, _: &Restart, cx: &mut ViewContext<Self>) {
        let client = self.debug_adapter(cx);
        if let Some(thread_id) = self.current_thread_id {
            cx.background_executor()
                .spawn(async move { client.restart(thread_id).await })
                .detach();
        }
    }

    fn handle_pause_action(&mut self, _: &Pause, cx: &mut ViewContext<Self>) {
        let client = self.debug_adapter(cx);
        if let Some(thread_id) = self.current_thread_id {
            cx.background_executor()
                .spawn(async move { client.pause(thread_id).await })
                .detach();
        }
    }

    fn handle_debug_client_events(
        this: &mut Self,
        model: Model<Project>,
        client_id: &DebugAdapterClientId,
        event: &Events,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            Events::Initialized(_) => {
                let client = this.debug_adapter_by_id(*client_id, cx);
                cx.spawn(|_, _| async move {
                    // TODO: send all the current breakpoints
                    client.configuration_done().await
                })
                .detach_and_log_err(cx);
            }
            Events::Stopped(event) => Self::handle_stopped_event(this, model, client_id, event, cx),
            Events::Continued(_) => {}
            Events::Exited(_) => {}
            Events::Terminated(_) => {}
            Events::Thread(event) => {
                if event.reason == ThreadEventReason::Started {
                    this.thread_state
                        .insert(event.thread_id, ThreadState::default());
                    this.current_thread_id = Some(event.thread_id);
                } else {
                    if this.current_thread_id == Some(event.thread_id) {
                        this.current_thread_id = None;
                    }
                    this.stack_frame_list.reset(0);
                    this.thread_state.remove(&event.thread_id);
                }

                cx.notify();
            }
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

    fn handle_stopped_event(
        this: &mut Self,
        model: Model<Project>,
        client_id: &DebugAdapterClientId,
        event: &StoppedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(thread_id) = event.thread_id else {
            return;
        };

        let client = this.debug_adapter_by_id(*client_id, cx);

        cx.spawn(|this, mut cx| async move {
            let stack_trace_response = client
                .request::<StackTrace>(StackTraceArguments {
                    thread_id,
                    start_frame: None,
                    levels: None,
                    format: None,
                })
                .await?;

            let mut scopes: HashMap<u64, Vec<Scope>> = HashMap::new();
            let mut variables: HashMap<u64, Vec<Variable>> = HashMap::new();

            for stack_frame in stack_trace_response.stack_frames.clone().into_iter() {
                let scope_response = client
                    .request::<Scopes>(ScopesArguments {
                        frame_id: stack_frame.id,
                    })
                    .await?;

                scopes.insert(stack_frame.id, scope_response.scopes.clone());

                for scope in scope_response.scopes {
                    variables.insert(
                        scope.variables_reference,
                        client
                            .request::<Variables>(VariablesArguments {
                                variables_reference: scope.variables_reference,
                                filter: None,
                                start: None,
                                count: None,
                                format: None,
                            })
                            .await?
                            .variables,
                    );
                }
            }

            this.update(&mut cx, |this, cx| {
                if let Some(entry) = this.thread_state.get_mut(&thread_id) {
                    this.current_thread_id = Some(thread_id);

                    this.current_stack_frame_id = stack_trace_response
                        .stack_frames
                        .clone()
                        .first()
                        .map(|f| f.id);

                    entry.stack_frames = stack_trace_response.stack_frames.clone();
                    entry.scopes = scopes;
                    entry.variables = variables;

                    this.stack_frame_list.reset(entry.stack_frames.len());

                    cx.notify();
                }

                anyhow::Ok(())
            })
        })
        .detach();
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
        self.position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _cx: &mut ViewContext<Self>) {
        self.position = position;
        // TODO:
        // cx.update_global::<SettingsStore>(f)
    }

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
                            .tooltip(move |cx| Tooltip::text("Continue debug", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-over", IconName::DebugStepOver)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(StepOver.boxed_clone())),
                            )
                            .tooltip(move |cx| Tooltip::text("Step over", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-in", IconName::DebugStepInto)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(StepIn.boxed_clone())),
                            )
                            .tooltip(move |cx| Tooltip::text("Go in", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-out", IconName::DebugStepOut)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(StepOut.boxed_clone())),
                            )
                            .tooltip(move |cx| Tooltip::text("Go out", cx)),
                    )
                    .child(
                        IconButton::new("debug-restart", IconName::DebugRestart)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(Restart.boxed_clone())),
                            )
                            .tooltip(move |cx| Tooltip::text("Restart", cx)),
                    )
                    .child(
                        IconButton::new("debug-pause", IconName::DebugStop)
                            .on_click(
                                cx.listener(|_, _, cx| cx.dispatch_action(Pause.boxed_clone())),
                            )
                            .tooltip(move |cx| Tooltip::text("Pause", cx)),
                    ),
            )
            .child(
                h_flex().size_full().items_start().p_1().gap_4().when_some(
                    self.current_thread_id
                        .and_then(|t| self.thread_state.get(&t)),
                    |this, thread_state| {
                        this.child(self.render_stack_frames(cx))
                            .child(self.render_scopes(thread_state, cx))
                    },
                ),
            )
            .into_any()
    }
}

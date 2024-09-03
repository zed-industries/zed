use crate::debugger_panel_item::DebugPanelItem;
use anyhow::Result;
use dap::client::{DebugAdapterClientId, ThreadState, ThreadStatus};
use dap::debugger_settings::DebuggerSettings;
use dap::requests::{Request, Scopes, StackTrace, StartDebugging};
use dap::transport::Payload;
use dap::{client::DebugAdapterClient, transport::Events};
use dap::{
    Capabilities, ContinuedEvent, ExitedEvent, OutputEvent, ScopesArguments, StackFrame,
    StackTraceArguments, StartDebuggingRequestArguments, StoppedEvent, TerminatedEvent,
    ThreadEvent, ThreadEventReason, Variable,
};
use editor::Editor;
use futures::future::try_join_all;
use gpui::{
    actions, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle, FocusableView,
    FontWeight, Subscription, Task, View, ViewContext, WeakView,
};
use serde_json::json;
use settings::Settings;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use task::DebugRequestType;
use ui::prelude::*;
use util::{merge_json_value_into, ResultExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};
use workspace::{pane, Pane, StartDebugger};

enum DebugCurrentRowHighlight {}

pub enum DebugPanelEvent {
    Stopped((DebugAdapterClientId, StoppedEvent)),
    Thread((DebugAdapterClientId, ThreadEvent)),
    Output((DebugAdapterClientId, OutputEvent)),
    ClientStopped(DebugAdapterClientId),
}

actions!(debug_panel, [ToggleFocus]);

pub struct DebugPanel {
    size: Pixels,
    pane: View<Pane>,
    focus_handle: FocusHandle,
    workspace: WeakView<Workspace>,
    _subscriptions: Vec<Subscription>,
    show_did_not_stop_warning: bool,
}

impl DebugPanel {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx| {
            let pane = cx.new_view(|cx| {
                let mut pane = Pane::new(
                    workspace.weak_handle(),
                    workspace.project().clone(),
                    Default::default(),
                    None,
                    None,
                    cx,
                );
                pane.set_can_split(false, cx);
                pane.set_can_navigate(true, cx);
                pane.display_nav_history_buttons(None);
                pane.set_should_display_tab_bar(|_| true);
                pane.set_close_pane_if_empty(false, cx);

                pane
            });

            let project = workspace.project().clone();

            let _subscriptions = vec![
                cx.observe(&pane, |_, _, cx| cx.notify()),
                cx.subscribe(&pane, Self::handle_pane_event),
                cx.subscribe(&project, {
                    move |this: &mut Self, _, event, cx| match event {
                        project::Event::DebugClientEvent { payload, client_id } => {
                            let Some(client) = this.debug_client_by_id(*client_id, cx) else {
                                return cx.emit(DebugPanelEvent::ClientStopped(*client_id));
                            };

                            match payload {
                                Payload::Event(event) => {
                                    Self::handle_debug_client_events(this, client, event, cx);
                                }
                                Payload::Request(request) => {
                                    if StartDebugging::COMMAND == request.command {
                                        Self::handle_start_debugging_request(
                                            this, client, request, cx,
                                        )
                                        .log_err();
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        project::Event::DebugClientStarted(client_id) => {
                            let Some(client) = this.debug_client_by_id(*client_id, cx) else {
                                return cx.emit(DebugPanelEvent::ClientStopped(*client_id));
                            };

                            cx.background_executor()
                                .spawn(async move {
                                    client.initialize().await?;

                                    // send correct request based on adapter config
                                    match client.config().request {
                                        DebugRequestType::Launch => {
                                            client.launch(client.request_args()).await
                                        }
                                        DebugRequestType::Attach => {
                                            client.attach(client.request_args()).await
                                        }
                                    }
                                })
                                .detach_and_log_err(cx);
                        }
                        project::Event::DebugClientStopped(client_id) => {
                            cx.emit(DebugPanelEvent::ClientStopped(*client_id));
                        }
                        _ => {}
                    }
                }),
            ];

            Self {
                pane,
                size: px(300.),
                _subscriptions,
                focus_handle: cx.focus_handle(),
                show_did_not_stop_warning: false,
                workspace: workspace.weak_handle(),
            }
        })
    }

    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            workspace.update(&mut cx, |workspace, cx| DebugPanel::new(workspace, cx))
        })
    }

    fn debug_client_by_id(
        &self,
        client_id: DebugAdapterClientId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Arc<DebugAdapterClient>> {
        self.workspace
            .update(cx, |this, cx| {
                this.project().read(cx).debug_adapter_by_id(client_id)
            })
            .ok()
            .flatten()
    }

    fn handle_pane_event(
        &mut self,
        _: View<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::RemovedItem { item } => {
                let thread_panel = item.downcast::<DebugPanelItem>().unwrap();

                thread_panel.update(cx, |pane, cx| {
                    let thread_id = pane.thread_id();
                    let client = pane.client();
                    let thread_status = client.thread_state_by_id(thread_id).status;

                    // only terminate thread if the thread has not yet ended
                    if thread_status != ThreadStatus::Ended && thread_status != ThreadStatus::Exited
                    {
                        let client = client.clone();
                        cx.background_executor()
                            .spawn(async move {
                                client.terminate_threads(Some(vec![thread_id; 1])).await
                            })
                            .detach_and_log_err(cx);
                    }
                });
            }
            pane::Event::Remove { .. } => cx.emit(PanelEvent::Close),
            pane::Event::ZoomIn => cx.emit(PanelEvent::ZoomIn),
            pane::Event::ZoomOut => cx.emit(PanelEvent::ZoomOut),
            pane::Event::AddItem { item } => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, self.pane.clone(), cx)
                    })
                    .ok();
            }
            _ => {}
        }
    }

    fn handle_start_debugging_request(
        this: &mut Self,
        client: Arc<DebugAdapterClient>,
        request: &dap::transport::Request,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        let arguments: StartDebuggingRequestArguments =
            serde_json::from_value(request.arguments.clone().unwrap_or_default())?;

        let mut json = json!({});
        if let Some(args) = client
            .config()
            .request_args
            .as_ref()
            .map(|a| a.args.clone())
        {
            merge_json_value_into(args, &mut json);
        }
        merge_json_value_into(arguments.configuration, &mut json);

        this.workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.start_debug_adapter_client(
                    client.config(),
                    client.command.clone(),
                    client.args.clone(),
                    client.cwd.clone(),
                    Some(json),
                    cx,
                );
            })
        })
    }

    fn handle_debug_client_events(
        this: &mut Self,
        client: Arc<DebugAdapterClient>,
        event: &Events,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            Events::Initialized(event) => Self::handle_initialized_event(client, event, cx),
            Events::Stopped(event) => Self::handle_stopped_event(client, event, cx),
            Events::Continued(event) => Self::handle_continued_event(client, event, cx),
            Events::Exited(event) => Self::handle_exited_event(client, event, cx),
            Events::Terminated(event) => Self::handle_terminated_event(this, client, event, cx),
            Events::Thread(event) => Self::handle_thread_event(this, client, event, cx),
            Events::Output(event) => Self::handle_output_event(client, event, cx),
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
            Events::Other(_) => {}
        }
    }

    pub async fn go_to_stack_frame(
        workspace: WeakView<Workspace>,
        stack_frame: StackFrame,
        clear_highlights: bool,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        let Some(path) = &stack_frame.source.and_then(|s| s.path) else {
            return Err(anyhow::anyhow!(
                "Cannot go to stack frame, path doesn't exist"
            ));
        };

        let row = (stack_frame.line.saturating_sub(1)) as u32;
        let column = (stack_frame.column.saturating_sub(1)) as u32;

        if clear_highlights {
            Self::remove_highlights(workspace.clone(), cx.clone())?;
        }

        let task = workspace.update(&mut cx, |workspace, cx| {
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
        })?;

        let editor = task.await?.downcast::<Editor>().unwrap();

        workspace.update(&mut cx, |_, cx| {
            editor.update(cx, |editor, cx| {
                editor.go_to_line::<DebugCurrentRowHighlight>(
                    row,
                    column,
                    Some(cx.theme().colors().editor_debugger_active_line_background),
                    cx,
                );
            })
        })
    }

    fn remove_highlights(workspace: WeakView<Workspace>, mut cx: AsyncWindowContext) -> Result<()> {
        workspace.update(&mut cx, |workspace, cx| {
            let editor_views = workspace
                .items_of_type::<Editor>(cx)
                .collect::<Vec<View<Editor>>>();

            for editor_view in editor_views {
                editor_view.update(cx, |editor, _| {
                    editor.clear_row_highlights::<DebugCurrentRowHighlight>();
                });
            }
        })
    }

    async fn remove_highlights_for_thread(
        workspace: WeakView<Workspace>,
        client: Arc<DebugAdapterClient>,
        thread_id: u64,
        cx: AsyncWindowContext,
    ) -> Result<()> {
        let mut tasks = Vec::new();
        let mut paths: HashSet<String> = HashSet::new();
        let thread_state = client.thread_state_by_id(thread_id);

        for stack_frame in thread_state.stack_frames.into_iter() {
            let Some(path) = stack_frame.source.clone().and_then(|s| s.path.clone()) else {
                continue;
            };

            if paths.contains(&path) {
                continue;
            }

            paths.insert(path.clone());
            tasks.push(Self::remove_editor_highlight(
                workspace.clone(),
                path,
                cx.clone(),
            ));
        }

        if !tasks.is_empty() {
            try_join_all(tasks).await?;
        }

        anyhow::Ok(())
    }

    async fn remove_editor_highlight(
        workspace: WeakView<Workspace>,
        path: String,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        let task = workspace.update(&mut cx, |workspace, cx| {
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
        })?;

        let editor = task.await?.downcast::<Editor>().unwrap();

        editor.update(&mut cx, |editor, _| {
            editor.clear_row_highlights::<DebugCurrentRowHighlight>();
        })
    }

    fn handle_initialized_event(
        client: Arc<DebugAdapterClient>,
        _: &Option<Capabilities>,
        cx: &mut ViewContext<Self>,
    ) {
        cx.spawn(|this, mut cx| async move {
            let task = this.update(&mut cx, |this, cx| {
                this.workspace.update(cx, |workspace, cx| {
                    workspace.project().update(cx, |project, cx| {
                        project.send_breakpoints(client.clone(), cx)
                    })
                })
            })??;

            task.await?;

            client.configuration_done().await
        })
        .detach_and_log_err(cx);
    }

    fn handle_continued_event(
        client: Arc<DebugAdapterClient>,
        event: &ContinuedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let all_threads = event.all_threads_continued.unwrap_or(false);

        if all_threads {
            for thread in client.thread_states().values_mut() {
                thread.status = ThreadStatus::Running;
            }
        } else {
            client.update_thread_state_status(event.thread_id, ThreadStatus::Running);
        }

        cx.notify();
    }

    fn handle_stopped_event(
        client: Arc<DebugAdapterClient>,
        event: &StoppedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(thread_id) = event.thread_id else {
            return;
        };

        let client_id = client.id();
        cx.spawn({
            let event = event.clone();
            |this, mut cx| async move {
                let stack_trace_response = client
                    .request::<StackTrace>(StackTraceArguments {
                        thread_id,
                        start_frame: None,
                        levels: None,
                        format: None,
                    })
                    .await?;

                let mut thread_state = ThreadState::default();

                let current_stack_frame =
                    stack_trace_response.stack_frames.first().unwrap().clone();
                let mut scope_tasks = Vec::new();
                for stack_frame in stack_trace_response.stack_frames.clone().into_iter() {
                    let client = client.clone();
                    scope_tasks.push(async move {
                        anyhow::Ok((
                            stack_frame.id,
                            client
                                .request::<Scopes>(ScopesArguments {
                                    frame_id: stack_frame.id,
                                })
                                .await?,
                        ))
                    });
                }

                let mut stack_frame_tasks = Vec::new();
                for (stack_frame_id, response) in try_join_all(scope_tasks).await? {
                    let client = client.clone();
                    stack_frame_tasks.push(async move {
                        let mut variable_tasks = Vec::new();

                        for scope in response.scopes {
                            let scope_reference = scope.variables_reference;

                            let client = client.clone();
                            variable_tasks.push(async move {
                                anyhow::Ok((scope, client.variables(scope_reference).await?))
                            });
                        }

                        anyhow::Ok((stack_frame_id, try_join_all(variable_tasks).await?))
                    });
                }

                for (stack_frame_id, scopes) in try_join_all(stack_frame_tasks).await? {
                    let stack_frame_state = thread_state
                        .variables
                        .entry(stack_frame_id)
                        .or_insert_with(BTreeMap::default);

                    for (scope, variables) in scopes {
                        thread_state
                            .vars
                            .insert(scope.variables_reference, variables.clone());

                        stack_frame_state.insert(
                            scope,
                            variables
                                .into_iter()
                                .map(|v| (1, v))
                                .collect::<Vec<(usize, Variable)>>(),
                        );
                    }
                }

                this.update(&mut cx, |this, cx| {
                    thread_state.current_stack_frame_id = current_stack_frame.clone().id;
                    thread_state.stack_frames = stack_trace_response.stack_frames;
                    thread_state.status = ThreadStatus::Stopped;
                    thread_state.stopped = true;

                    client.thread_states().insert(thread_id, thread_state);

                    let existing_item = this
                        .pane
                        .read(cx)
                        .items()
                        .filter_map(|item| item.downcast::<DebugPanelItem>())
                        .any(|item| {
                            let item = item.read(cx);

                            item.client().id() == client_id && item.thread_id() == thread_id
                        });

                    if !existing_item {
                        let debug_panel = cx.view().clone();
                        this.pane.update(cx, |pane, cx| {
                            let tab = cx.new_view(|cx| {
                                DebugPanelItem::new(
                                    debug_panel,
                                    this.workspace.clone(),
                                    client.clone(),
                                    thread_id,
                                    cx,
                                )
                            });

                            pane.add_item(Box::new(tab), true, true, None, cx);
                        });
                    }

                    cx.emit(DebugPanelEvent::Stopped((client_id, event)));

                    cx.notify();

                    if let Some(item) = this.pane.read(cx).active_item() {
                        if let Some(pane) = item.downcast::<DebugPanelItem>() {
                            let pane = pane.read(cx);
                            if pane.thread_id() == thread_id && pane.client().id() == client_id {
                                let workspace = this.workspace.clone();
                                return cx.spawn(|_, cx| async move {
                                    Self::go_to_stack_frame(
                                        workspace,
                                        current_stack_frame.clone(),
                                        true,
                                        cx,
                                    )
                                    .await
                                });
                            }
                        }
                    }

                    Task::ready(anyhow::Ok(()))
                })?
                .await
            }
        })
        .detach_and_log_err(cx);
    }

    fn handle_thread_event(
        this: &mut Self,
        client: Arc<DebugAdapterClient>,
        event: &ThreadEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let thread_id = event.thread_id;

        if let Some(thread_state) = client.thread_states().get(&thread_id) {
            if !thread_state.stopped && event.reason == ThreadEventReason::Exited {
                this.show_did_not_stop_warning = true;
                cx.notify();
            };
        }

        if event.reason == ThreadEventReason::Started {
            client
                .thread_states()
                .insert(thread_id, ThreadState::default());
        } else {
            client.update_thread_state_status(thread_id, ThreadStatus::Ended);

            cx.notify();

            // TODO: we want to figure out for witch clients/threads we should remove the highlights
            cx.spawn({
                let client = client.clone();
                |this, mut cx| async move {
                    let workspace = this.update(&mut cx, |this, _| this.workspace.clone())?;

                    Self::remove_highlights_for_thread(workspace, client, thread_id, cx).await?;

                    anyhow::Ok(())
                }
            })
            .detach_and_log_err(cx);
        }

        cx.emit(DebugPanelEvent::Thread((client.id(), event.clone())));
    }

    fn handle_exited_event(
        client: Arc<DebugAdapterClient>,
        _: &ExitedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.spawn(|this, mut cx| async move {
            for thread_state in client.thread_states().values_mut() {
                thread_state.status = ThreadStatus::Exited;
            }

            this.update(&mut cx, |_, cx| cx.notify())
        })
        .detach_and_log_err(cx);
    }

    fn handle_terminated_event(
        this: &mut Self,
        client: Arc<DebugAdapterClient>,
        event: &Option<TerminatedEvent>,
        cx: &mut ViewContext<Self>,
    ) {
        let restart_args = event.clone().and_then(|e| e.restart);
        let workspace = this.workspace.clone();

        cx.spawn(|_, mut cx| async move {
            Self::remove_highlights(workspace.clone(), cx.clone())?;

            if restart_args.is_some() {
                client.disconnect(Some(true), None, None).await?;

                match client.request_type() {
                    DebugRequestType::Launch => client.launch(restart_args).await,
                    DebugRequestType::Attach => client.attach(restart_args).await,
                }
            } else {
                cx.update(|cx| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.project().update(cx, |project, cx| {
                            project.stop_debug_adapter_client(client.id(), false, cx)
                        })
                    })
                })?
            }
        })
        .detach_and_log_err(cx);
    }

    fn handle_output_event(
        client: Arc<DebugAdapterClient>,
        event: &OutputEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(DebugPanelEvent::Output((client.id(), event.clone())));
    }

    fn render_did_not_stop_warning(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        const TITLE: &str = "Debug session exited without hitting any breakpoints";
        const DESCRIPTION: &str =
            "Try adding a breakpoint, or define the correct path mapping for your debugger.";

        div()
            .absolute()
            .right_3()
            .bottom_12()
            .max_w_96()
            .py_2()
            .px_3()
            .elevation_2(cx)
            .occlude()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .items_center()
                            .child(Icon::new(IconName::ExclamationTriangle).color(Color::Conflict))
                            .child(Label::new(TITLE).weight(FontWeight::MEDIUM)),
                    )
                    .child(
                        Label::new(DESCRIPTION)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        h_flex().justify_end().mt_1().child(
                            Button::new("dismiss", "Dismiss")
                                .color(Color::Muted)
                                .on_click(cx.listener(|this, _, cx| {
                                    this.show_did_not_stop_warning = false;
                                    cx.notify();
                                })),
                        ),
                    ),
            )
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}
impl EventEmitter<DebugPanelEvent> for DebugPanel {}
impl EventEmitter<project::Event> for DebugPanel {}

impl FocusableView for DebugPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for DebugPanel {
    fn pane(&self) -> Option<View<Pane>> {
        Some(self.pane.clone())
    }

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
        Some(IconName::Debug)
    }

    fn icon_tooltip(&self, cx: &WindowContext) -> Option<&'static str> {
        if DebuggerSettings::get_global(cx).button {
            Some("Debug Panel")
        } else {
            None
        }
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

impl Render for DebugPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DebugPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .when(self.show_did_not_stop_warning, |this| {
              this.child(self.render_did_not_stop_warning(cx))
            })
            .map(|this| {
                if self.pane.read(cx).items_len() == 0 {
                    this.child(
                        h_flex().size_full().items_center().justify_center().child(
                            v_flex()
                                .gap_2()
                                .rounded_md()
                                .max_w_64()
                                .items_start()
                                .child(
                                    Label::new("You can create a debug task by creating a new task and setting the `type` key to `debug`")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    h_flex().w_full().justify_end().child(
                                        Button::new(
                                            "start-debugger",
                                            "Choose a debugger",
                                        )
                                        .label_size(LabelSize::Small)
                                        .on_click(move |_, cx| {
                                            cx.dispatch_action(StartDebugger.boxed_clone());
                                        })
                                    ),
                                ),
                        ),
                    )
                } else {
                    this.child(self.pane.clone())
                }
            })
            .into_any()
    }
}

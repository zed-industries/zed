use anyhow::Result;
use dap::client::{DebugAdapterClientId, ThreadState, ThreadStatus};
use dap::requests::{Disconnect, Scopes, StackTrace, Variables};
use dap::{client::DebugAdapterClient, transport::Events};
use dap::{
    Capabilities, ContinuedEvent, DisconnectArguments, ExitedEvent, Scope, ScopesArguments,
    StackFrame, StackTraceArguments, StoppedEvent, TerminatedEvent, ThreadEvent, ThreadEventReason,
    Variable, VariablesArguments,
};
use editor::Editor;
use futures::future::try_join_all;
use gpui::{
    actions, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle, FocusableView,
    Subscription, Task, View, ViewContext, WeakView,
};
use std::path::Path;
use std::{collections::HashMap, sync::Arc};
use task::DebugRequestType;
use ui::prelude::*;
use workspace::Pane;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

use crate::debugger_panel_item::DebugPanelItem;

enum DebugCurrentRowHighlight {}

#[derive(Debug)]
pub enum DebugPanelEvent {
    Stopped((DebugAdapterClientId, StoppedEvent)),
    Thread((DebugAdapterClientId, ThreadEvent)),
}

actions!(debug_panel, [ToggleFocus]);

pub struct DebugPanel {
    size: Pixels,
    pane: View<Pane>,
    focus_handle: FocusHandle,
    workspace: WeakView<Workspace>,
    _subscriptions: Vec<Subscription>,
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

                pane
            });

            let project = workspace.project().clone();

            let _subscriptions = vec![cx.subscribe(&project, {
                move |this: &mut Self, _, event, cx| match event {
                    project::Event::DebugClientEvent { event, client_id } => {
                        Self::handle_debug_client_events(
                            this,
                            this.debug_client_by_id(*client_id, cx),
                            event,
                            cx,
                        );
                    }
                    project::Event::DebugClientStarted(client_id) => {
                        let client = this.debug_client_by_id(*client_id, cx);
                        cx.spawn(|_, _| async move {
                            client.initialize().await?;
                            let request_args = client.config().request_args.map(|a| a.args);

                            // send correct request based on adapter config
                            match client.config().request {
                                DebugRequestType::Launch => client.launch(request_args).await,
                                DebugRequestType::Attach => client.attach(request_args).await,
                            }
                        })
                        .detach_and_log_err(cx);
                    }
                    _ => {}
                }
            })];

            Self {
                pane,
                size: px(300.),
                _subscriptions,
                focus_handle: cx.focus_handle(),
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
            Events::Thread(event) => Self::handle_thread_event(client, event, cx),
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
            Events::Other(_) => {}
        }
    }

    pub async fn go_to_stack_frame(
        workspace: WeakView<Workspace>,
        stack_frame: StackFrame,
        client: Arc<DebugAdapterClient>,
        clear_highlights: bool,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        let path = stack_frame.clone().source.unwrap().path.unwrap().clone();
        let row = (stack_frame.line.saturating_sub(1)) as u32;
        let column = (stack_frame.column.saturating_sub(1)) as u32;

        if clear_highlights {
            Self::remove_highlights(workspace.clone(), client, cx.clone()).await?;
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

    async fn remove_highlights(
        workspace: WeakView<Workspace>,
        client: Arc<DebugAdapterClient>,
        cx: AsyncWindowContext,
    ) -> Result<()> {
        let mut tasks = Vec::new();
        for thread_state in client.thread_states().values() {
            for stack_frame in thread_state.stack_frames.clone() {
                tasks.push(Self::remove_editor_highlight(
                    workspace.clone(),
                    stack_frame,
                    cx.clone(),
                ));
            }
        }

        if !tasks.is_empty() {
            try_join_all(tasks).await?;
        }

        anyhow::Ok(())
    }

    async fn remove_highlights_for_thread(
        workspace: WeakView<Workspace>,
        client: Arc<DebugAdapterClient>,
        thread_id: u64,
        cx: AsyncWindowContext,
    ) -> Result<()> {
        let mut tasks = Vec::new();
        if let Some(thread_state) = client.thread_states().get(&thread_id) {
            for stack_frame in thread_state.stack_frames.clone() {
                tasks.push(Self::remove_editor_highlight(
                    workspace.clone(),
                    stack_frame.clone(),
                    cx.clone(),
                ));
            }
        }

        if !tasks.is_empty() {
            try_join_all(tasks).await?;
        }

        anyhow::Ok(())
    }

    async fn remove_editor_highlight(
        workspace: WeakView<Workspace>,
        stack_frame: StackFrame,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        let path = stack_frame.clone().source.unwrap().path.unwrap().clone();

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
                        let client = client.clone();

                        project.send_breakpoints(client, cx)
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

                let current_stack_frame =
                    stack_trace_response.stack_frames.first().unwrap().clone();
                let mut scope_tasks = Vec::new();
                for stack_frame in stack_trace_response.stack_frames.clone().into_iter() {
                    let frame_id = stack_frame.id;
                    let client = client.clone();
                    scope_tasks.push(async move {
                        anyhow::Ok((
                            frame_id,
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
                        let scope_reference = scope.variables_reference;
                        let client = client.clone();
                        variable_tasks.push(async move {
                            anyhow::Ok((
                                scope_reference,
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
                    let mut thread_state = client.thread_states();
                    let thread_state = thread_state
                        .entry(thread_id)
                        .or_insert(ThreadState::default());

                    thread_state.current_stack_frame_id = Some(current_stack_frame.clone().id);
                    thread_state.stack_frames = stack_trace_response.stack_frames;
                    thread_state.scopes = scopes;
                    thread_state.variables = variables;
                    thread_state.status = ThreadStatus::Stopped;

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
                        this.pane.update(cx, |this, cx| {
                            let tab = cx.new_view(|cx| {
                                DebugPanelItem::new(debug_panel, client.clone(), thread_id, cx)
                            });

                            this.add_item(Box::new(tab.clone()), false, false, None, cx)
                        });
                    }

                    cx.emit(DebugPanelEvent::Stopped((client_id, event)));

                    if let Some(item) = this.pane.read(cx).active_item() {
                        if let Some(pane) = item.downcast::<DebugPanelItem>() {
                            let pane = pane.read(cx);
                            if pane.thread_id() == thread_id && pane.client().id() == client_id {
                                let workspace = this.workspace.clone();
                                let client = client.clone();
                                return cx.spawn(|_, cx| async move {
                                    Self::go_to_stack_frame(
                                        workspace,
                                        current_stack_frame.clone(),
                                        client,
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
        client: Arc<DebugAdapterClient>,
        event: &ThreadEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let thread_id = event.thread_id;

        if event.reason == ThreadEventReason::Started {
            client
                .thread_states()
                .insert(thread_id, ThreadState::default());
        } else {
            client.update_thread_state_status(thread_id, ThreadStatus::Ended);

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
        cx.spawn(|_, _| async move {
            for thread_state in client.thread_states().values_mut() {
                thread_state.status = ThreadStatus::Exited;
            }
        })
        .detach();
    }

    fn handle_terminated_event(
        this: &mut Self,
        client: Arc<DebugAdapterClient>,
        event: &Option<TerminatedEvent>,
        cx: &mut ViewContext<Self>,
    ) {
        let restart_args = event.clone().and_then(|e| e.restart);
        let workspace = this.workspace.clone();

        cx.spawn(|_, cx| async move {
            let should_restart = restart_args.is_some();

            Self::remove_highlights(workspace, client.clone(), cx).await?;

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
        Box::new(ToggleFocus)
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
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DebugPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .child(self.pane.clone())
            .into_any()
    }
}

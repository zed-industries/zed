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
    actions, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle, FocusableView,
    Subscription, Task, View, ViewContext, WeakView,
};
use std::path::Path;
use std::{collections::HashMap, sync::Arc};
use task::DebugRequestType;
use ui::prelude::*;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};
use workspace::{NewFile, Pane};

use crate::debugger_panel_item::DebugPanelItem;

enum DebugCurrentRowHighlight {}

#[derive(Debug)]
pub enum DebugPanelEvent {
    Stopped((DebugAdapterClientId, StoppedEvent)),
    Thread((DebugAdapterClientId, ThreadEvent)),
}

actions!(debug_panel, [TogglePanel]);

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
                    NewFile.boxed_clone(),
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
                move |this: &mut Self, _, event, cx| {
                    if let project::Event::DebugClientEvent { client_id, event } = event {
                        Self::handle_debug_client_events(this, client_id, event, cx);
                    }
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

    fn remove_highlights_for_thread(
        &self,
        client: Arc<DebugAdapterClient>,
        thread_id: u64,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let mut tasks = Vec::new();

        if let Some(thread_state) = client.thread_state().get(&thread_id) {
            for stack_frame in thread_state.stack_frames.clone() {
                tasks.push(self.remove_editor_highlight(&stack_frame, cx));
            }
        }

        if tasks.is_empty() {
            return Task::ready(Ok(()));
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

        let client_id = client_id.clone();
        let client = this.debug_client_by_id(client_id.clone(), cx);

        let client_id = client_id.clone();
        cx.spawn({
            let event = event.clone();
            |this, mut cx| async move {
                this.update(&mut cx, |this, cx| {
                    this.remove_highlights_for_thread(client.clone(), thread_id, cx)
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

                let current_stack_frame =
                    stack_trace_response.stack_frames.first().unwrap().clone();
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
                    let mut thread_state = client.thread_state();
                    let thread_state = thread_state
                        .entry(thread_id)
                        .or_insert(ThreadState::default());

                    thread_state.current_stack_frame_id = Some(current_stack_frame.clone().id);
                    thread_state.stack_frames = stack_trace_response.stack_frames.clone();
                    thread_state.scopes = scopes;
                    thread_state.variables = variables;
                    thread_state.status = ThreadStatus::Stopped;

                    let focus = this.focus_handle(cx).contains_focused(cx);

                    let existing_item = this
                        .pane
                        .read(cx)
                        .items()
                        .filter_map(|item| item.downcast::<DebugPanelItem>())
                        .find(|item| {
                            let item = item.read(cx);

                            item.client().id() == client_id && item.thread_id() == thread_id
                        });

                    if let None = existing_item {
                        let debug_panel = cx.view().clone();
                        this.pane.update(cx, |this, cx| {
                            let tab = cx.new_view(|cx| {
                                DebugPanelItem::new(debug_panel, client.clone(), thread_id, cx)
                            });

                            this.add_item(Box::new(tab.clone()), focus, focus, None, cx)
                        });
                    }

                    cx.emit(DebugPanelEvent::Stopped((client_id, event)));

                    // if Some(client.id()) == this.debug_client(cx).map(|c| c.id()) {
                    //     // this.stack_frame_list.reset(thread_state.stack_frames.len());
                    //     // cx.notify();

                    //     return this.go_to_stack_frame(&current_stack_frame, client.clone(), true, cx);
                    // }

                    Task::ready(anyhow::Ok(()))
                })?
                .await
            }
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
        let thread_id = event.thread_id;

        if event.reason == ThreadEventReason::Started {
            client
                .thread_state()
                .insert(thread_id, ThreadState::default());
        } else {
            client.update_thread_state_status(thread_id.clone(), ThreadStatus::Ended);

            cx.spawn({
                let client = client.clone();
                |this, mut cx| async move {
                    this.update(&mut cx, |this, cx| {
                        this.remove_highlights_for_thread(client, thread_id, cx)
                    })?
                    .await
                }
            })
            .detach_and_log_err(cx);
        }

        cx.emit(DebugPanelEvent::Thread((*client_id, event.clone())));
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
            // .capture_action(cx.listener(Self::handle_continue_action))
            // .capture_action(cx.listener(Self::handle_step_over_action))
            // .capture_action(cx.listener(Self::handle_step_in_action))
            // .capture_action(cx.listener(Self::handle_step_out_action))
            // .capture_action(cx.listener(Self::handle_restart_action))
            // .capture_action(cx.listener(Self::handle_pause_action))
            .size_full()
            .child(self.pane.clone())
            .into_any()
    }
}

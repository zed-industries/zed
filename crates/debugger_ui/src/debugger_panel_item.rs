use crate::console::Console;
use crate::debugger_panel::{DebugPanel, DebugPanelEvent, ThreadState, ThreadStatus};
use crate::loaded_source_list::LoadedSourceList;
use crate::module_list::ModuleList;
use crate::stack_frame_list::{StackFrameList, StackFrameListEvent};
use crate::variable_list::VariableList;

use dap::proto_conversions::{self, ProtoConversion};
use dap::session::DebugSession;
use dap::{
    client::DebugAdapterClientId, debugger_settings::DebuggerSettings, Capabilities,
    ContinuedEvent, LoadedSourceEvent, ModuleEvent, OutputEvent, OutputEventCategory, StoppedEvent,
    ThreadEvent,
};
use editor::Editor;
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity,
};
use project::dap_store::DapStore;
use rpc::proto::{self, DebuggerThreadStatus, PeerId, SetDebuggerPanelItem, UpdateDebugAdapter};
use settings::Settings;
use ui::{prelude::*, Indicator, Tooltip};
use util::ResultExt as _;
use workspace::{
    item::{self, Item, ItemEvent},
    FollowableItem, ItemHandle, ViewId, Workspace,
};

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
    Variables,
}

impl ThreadItem {
    fn to_proto(&self) -> proto::DebuggerThreadItem {
        match self {
            ThreadItem::Console => proto::DebuggerThreadItem::Console,
            ThreadItem::LoadedSource => proto::DebuggerThreadItem::LoadedSource,
            ThreadItem::Modules => proto::DebuggerThreadItem::Modules,
            ThreadItem::Variables => proto::DebuggerThreadItem::Variables,
        }
    }

    fn from_proto(active_thread_item: proto::DebuggerThreadItem) -> Self {
        match active_thread_item {
            proto::DebuggerThreadItem::Console => ThreadItem::Console,
            proto::DebuggerThreadItem::LoadedSource => ThreadItem::LoadedSource,
            proto::DebuggerThreadItem::Modules => ThreadItem::Modules,
            proto::DebuggerThreadItem::Variables => ThreadItem::Variables,
        }
    }
}

pub struct DebugPanelItem {
    thread_id: u64,
    console: Entity<Console>,
    focus_handle: FocusHandle,
    remote_id: Option<ViewId>,
    dap_store: Entity<DapStore>,
    session: Entity<DebugSession>,
    show_console_indicator: bool,
    module_list: Entity<ModuleList>,
    active_thread_item: ThreadItem,
    workspace: WeakEntity<Workspace>,
    client_id: DebugAdapterClientId,
    thread_state: Entity<ThreadState>,
    variable_list: Entity<VariableList>,
    _subscriptions: Vec<Subscription>,
    stack_frame_list: Entity<StackFrameList>,
    loaded_source_list: Entity<LoadedSourceList>,
}

impl DebugPanelItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session: Entity<DebugSession>,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        thread_state: Entity<ThreadState>,
        dap_store: Entity<DapStore>,
        debug_panel: &Entity<DebugPanel>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let this = cx.entity();

        let stack_frame_list = cx.new(|cx| {
            StackFrameList::new(
                workspace.clone(),
                &this,
                dap_store.clone(),
                session.clone(),
                client_id,
                thread_id,
                window,
                cx,
            )
        });

        let variable_list = cx.new(|cx| {
            VariableList::new(
                session.clone(),
                client_id,
                dap_store.clone(),
                stack_frame_list.clone(),
                window,
                cx,
            )
        });

        let module_list =
            cx.new(|cx| ModuleList::new(dap_store.clone(), session.clone(), &client_id, cx));

        let loaded_source_list =
            cx.new(|cx| LoadedSourceList::new(&this, dap_store.clone(), &client_id, cx));

        let console = cx.new(|cx| {
            Console::new(
                session.clone(),
                client_id,
                dap_store.clone(),
                stack_frame_list.clone(),
                variable_list.clone(),
                window,
                cx,
            )
        });

        cx.observe(&module_list, |_, _, cx| cx.notify()).detach();

        let _subscriptions = vec![
            cx.subscribe_in(debug_panel, window, {
                move |this: &mut Self, _, event: &DebugPanelEvent, window, cx| {
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
                            this.handle_output_event(client_id, event, window, cx)
                        }
                        DebugPanelEvent::Module((client_id, event)) => {
                            this.handle_module_event(client_id, event, cx)
                        }
                        DebugPanelEvent::LoadedSource((client_id, event)) => {
                            this.handle_loaded_source_event(client_id, event, cx)
                        }
                        DebugPanelEvent::ClientShutdown(client_id) => {
                            this.handle_client_shutdown_event(client_id, cx)
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
                    StackFrameListEvent::SelectedStackFrameChanged(_)
                    | StackFrameListEvent::StackFramesUpdated => this.clear_highlights(cx),
                },
            ),
        ];

        Self {
            session,
            console,
            thread_id,
            dap_store,
            workspace,
            module_list,
            thread_state,
            focus_handle,
            variable_list,
            _subscriptions,
            remote_id: None,
            stack_frame_list,
            loaded_source_list,
            client_id: *client_id,
            show_console_indicator: false,
            active_thread_item: ThreadItem::Variables,
        }
    }

    pub(crate) fn to_proto(&self, project_id: u64, cx: &App) -> SetDebuggerPanelItem {
        let thread_state = Some(self.thread_state.read(cx).to_proto());
        let variable_list = Some(self.variable_list.read(cx).to_proto());
        let stack_frame_list = Some(self.stack_frame_list.read(cx).to_proto());

        SetDebuggerPanelItem {
            project_id,
            session_id: self.session.read(cx).id().to_proto(),
            client_id: self.client_id.to_proto(),
            thread_id: self.thread_id,
            console: None,
            module_list: None,
            active_thread_item: self.active_thread_item.to_proto().into(),
            thread_state,
            variable_list,
            stack_frame_list,
            loaded_source_list: None,
            session_name: self.session.read(cx).name(),
        }
    }

    pub(crate) fn from_proto(&mut self, state: &SetDebuggerPanelItem, cx: &mut Context<Self>) {
        self.thread_state.update(cx, |thread_state, _| {
            let (status, stopped) = state
                .thread_state
                .as_ref()
                .map_or((DebuggerThreadStatus::Stopped, true), |thread_state| {
                    (thread_state.thread_status(), true)
                });

            thread_state.status = ThreadStatus::from_proto(status);
            thread_state.stopped = stopped;
        });

        self.active_thread_item = ThreadItem::from_proto(state.active_thread_item());

        if let Some(stack_frame_list) = state.stack_frame_list.as_ref() {
            self.stack_frame_list.update(cx, |this, cx| {
                this.set_from_proto(stack_frame_list.clone(), cx);
            });
        }

        if let Some(variable_list_state) = state.variable_list.as_ref() {
            self.variable_list
                .update(cx, |this, cx| this.set_from_proto(variable_list_state, cx));
        }

        if let Some(module_list_state) = state.module_list.as_ref() {
            self.module_list
                .update(cx, |this, cx| this.set_from_proto(module_list_state, cx));
        }

        cx.notify();
    }

    pub fn update_thread_state_status(&mut self, status: ThreadStatus, cx: &mut Context<Self>) {
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
        cx: &mut Context<Self>,
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
        cx: &mut Context<Self>,
    ) {
        if self.should_skip_event(client_id, event.thread_id.unwrap_or(self.thread_id)) {
            return;
        }

        cx.emit(DebugPanelItemEvent::Stopped { go_to_stack_frame });

        if let Some((downstream_client, project_id)) = self.dap_store.read(cx).downstream_client() {
            downstream_client
                .send(self.to_proto(*project_id, cx))
                .log_err();
        }
    }

    fn handle_thread_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ThreadEvent,
        cx: &mut Context<Self>,
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        // skip telemetry output as it pollutes the users output view
        let output_category = event
            .category
            .as_ref()
            .unwrap_or(&OutputEventCategory::Console);

        // skip telemetry output as it pollutes the users output view
        if output_category == &OutputEventCategory::Telemetry {
            return;
        }

        self.console.update(cx, |console, cx| {
            console.add_message(event.clone(), window, cx);
        });
        self.show_console_indicator = true;
        cx.notify();
    }

    fn handle_module_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ModuleEvent,
        cx: &mut Context<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        self.module_list.update(cx, |module_list, cx| {
            module_list.on_module_event(event, cx);
        });
    }

    fn handle_loaded_source_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &LoadedSourceEvent,
        cx: &mut Context<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        self.loaded_source_list
            .update(cx, |loaded_source_list, cx| {
                loaded_source_list.on_loaded_source_event(event, cx);
            });
    }

    fn handle_client_shutdown_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut Context<Self>,
    ) {
        if self.should_skip_event(client_id, self.thread_id) {
            return;
        }

        self.update_thread_state_status(ThreadStatus::Stopped, cx);

        self.dap_store.update(cx, |store, cx| {
            store.remove_active_debug_line_for_client(client_id, cx);
        });

        cx.emit(DebugPanelItemEvent::Close);
    }

    fn handle_client_exited_and_terminated_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut Context<Self>,
    ) {
        if Self::should_skip_event(self, client_id, self.thread_id) {
            return;
        }

        self.update_thread_state_status(ThreadStatus::Exited, cx);

        self.dap_store.update(cx, |store, cx| {
            store.remove_active_debug_line_for_client(client_id, cx);
        });

        cx.emit(DebugPanelItemEvent::Close);
    }

    fn handle_capabilities_changed_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut Context<Self>,
    ) {
        if Self::should_skip_event(self, client_id, self.thread_id) {
            return;
        }

        // notify the view that the capabilities have changed
        cx.notify();

        if let Some((downstream_client, project_id)) = self.dap_store.read(cx).downstream_client() {
            let message = proto_conversions::capabilities_to_proto(
                &self.dap_store.read(cx).capabilities_by_id(client_id),
                *project_id,
                self.session.read(cx).id().to_proto(),
                self.client_id.to_proto(),
            );

            downstream_client.send(message).log_err();
        }
    }

    pub(crate) fn update_adapter(
        &mut self,
        update: &UpdateDebugAdapter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(update_variant) = update.variant.as_ref() {
            match update_variant {
                proto::update_debug_adapter::Variant::StackFrameList(stack_frame_list) => {
                    self.stack_frame_list.update(cx, |this, cx| {
                        this.set_from_proto(stack_frame_list.clone(), cx);
                    })
                }
                proto::update_debug_adapter::Variant::ThreadState(thread_state) => {
                    self.thread_state.update(cx, |this, _| {
                        *this = ThreadState::from_proto(thread_state.clone());
                    })
                }
                proto::update_debug_adapter::Variant::VariableList(variable_list) => self
                    .variable_list
                    .update(cx, |this, cx| this.set_from_proto(variable_list, cx)),
                proto::update_debug_adapter::Variant::AddToVariableList(variables_to_add) => self
                    .variable_list
                    .update(cx, |this, _| this.add_variables(variables_to_add.clone())),
                proto::update_debug_adapter::Variant::Modules(module_list) => {
                    self.module_list.update(cx, |this, cx| {
                        this.set_from_proto(module_list, cx);
                    })
                }
                proto::update_debug_adapter::Variant::OutputEvent(output_event) => {
                    self.console.update(cx, |this, cx| {
                        this.add_message(OutputEvent::from_proto(output_event.clone()), window, cx);
                    })
                }
            }
        }
    }

    pub fn session(&self) -> &Entity<DebugSession> {
        &self.session
    }

    pub fn client_id(&self) -> DebugAdapterClientId {
        self.client_id
    }

    pub fn thread_id(&self) -> u64 {
        self.thread_id
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
    pub fn thread_state(&self) -> &Entity<ThreadState> {
        &self.thread_state
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn are_breakpoints_ignored(&self, cx: &App) -> bool {
        self.session.read(cx).ignore_breakpoints()
    }

    pub fn capabilities(&self, cx: &mut Context<Self>) -> Capabilities {
        self.dap_store.read(cx).capabilities_by_id(&self.client_id)
    }

    fn clear_highlights(&self, cx: &mut Context<Self>) {
        if let Some((_, project_path, _)) = self.dap_store.read(cx).active_debug_line() {
            self.workspace
                .update(cx, |workspace, cx| {
                    let editor = workspace
                        .items_of_type::<Editor>(cx)
                        .find(|editor| Some(project_path.clone()) == editor.project_path(cx));

                    if let Some(editor) = editor {
                        editor.update(cx, |editor, cx| {
                            editor.clear_row_highlights::<editor::DebugCurrentRowHighlight>();

                            cx.notify();
                        });
                    }
                })
                .ok();
        }
    }

    pub fn go_to_current_stack_frame(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.stack_frame_list.update(cx, |stack_frame_list, cx| {
            if let Some(stack_frame) = stack_frame_list
                .stack_frames()
                .iter()
                .find(|frame| frame.id == stack_frame_list.current_stack_frame_id())
                .cloned()
            {
                stack_frame_list
                    .select_stack_frame(&stack_frame, true, window, cx)
                    .detach_and_log_err(cx);
            }
        });
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
        self.update_thread_state_status(ThreadStatus::Running, cx);

        let task = self.dap_store.update(cx, |store, cx| {
            store.continue_thread(&self.client_id, self.thread_id, cx)
        });

        cx.spawn(|this, mut cx| async move {
            if task.await.log_err().is_none() {
                this.update(&mut cx, |debug_panel_item, cx| {
                    debug_panel_item.update_thread_state_status(ThreadStatus::Stopped, cx);
                })
                .log_err();
            }
        })
        .detach();
    }

    pub fn step_over(&mut self, cx: &mut Context<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);
        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        let task = self.dap_store.update(cx, |store, cx| {
            store.step_over(&self.client_id, self.thread_id, granularity, cx)
        });

        cx.spawn(|this, mut cx| async move {
            if task.await.log_err().is_none() {
                this.update(&mut cx, |debug_panel_item, cx| {
                    debug_panel_item.update_thread_state_status(ThreadStatus::Stopped, cx);
                })
                .log_err();
            }
        })
        .detach();
    }

    pub fn step_in(&mut self, cx: &mut Context<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);
        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        let task = self.dap_store.update(cx, |store, cx| {
            store.step_in(&self.client_id, self.thread_id, granularity, cx)
        });

        cx.spawn(|this, mut cx| async move {
            if task.await.log_err().is_none() {
                this.update(&mut cx, |debug_panel_item, cx| {
                    debug_panel_item.update_thread_state_status(ThreadStatus::Stopped, cx);
                })
                .log_err();
            }
        })
        .detach();
    }

    pub fn step_out(&mut self, cx: &mut Context<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);
        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        let task = self.dap_store.update(cx, |store, cx| {
            store.step_out(&self.client_id, self.thread_id, granularity, cx)
        });

        cx.spawn(|this, mut cx| async move {
            if task.await.log_err().is_none() {
                this.update(&mut cx, |debug_panel_item, cx| {
                    debug_panel_item.update_thread_state_status(ThreadStatus::Stopped, cx);
                })
                .log_err();
            }
        })
        .detach();
    }

    pub fn step_back(&mut self, cx: &mut Context<Self>) {
        self.update_thread_state_status(ThreadStatus::Running, cx);
        let granularity = DebuggerSettings::get_global(cx).stepping_granularity;

        let task = self.dap_store.update(cx, |store, cx| {
            store.step_back(&self.client_id, self.thread_id, granularity, cx)
        });

        cx.spawn(|this, mut cx| async move {
            if task.await.log_err().is_none() {
                this.update(&mut cx, |debug_panel_item, cx| {
                    debug_panel_item.update_thread_state_status(ThreadStatus::Stopped, cx);
                })
                .log_err();
            }
        })
        .detach();
    }

    pub fn restart_client(&self, cx: &mut Context<Self>) {
        self.dap_store.update(cx, |store, cx| {
            store
                .restart(&self.client_id, None, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn pause_thread(&self, cx: &mut Context<Self>) {
        self.dap_store.update(cx, |store, cx| {
            store
                .pause_thread(&self.client_id, self.thread_id, cx)
                .detach_and_log_err(cx)
        });
    }

    pub fn stop_thread(&self, cx: &mut Context<Self>) {
        self.dap_store.update(cx, |store, cx| {
            store
                .terminate_threads(
                    &self.session.read(cx).id(),
                    &self.client_id,
                    Some(vec![self.thread_id; 1]),
                    cx,
                )
                .detach_and_log_err(cx)
        });
    }

    pub fn disconnect_client(&self, cx: &mut Context<Self>) {
        self.dap_store.update(cx, |store, cx| {
            store
                .disconnect_client(&self.client_id, cx)
                .detach_and_log_err(cx);
        });
    }

    pub fn toggle_ignore_breakpoints(&mut self, cx: &mut Context<Self>) {
        self.session.update(cx, |session, cx| {
            session.set_ignore_breakpoints(!session.ignore_breakpoints(), cx);
        });
    }
}

impl EventEmitter<DebugPanelItemEvent> for DebugPanelItem {}

impl Focusable for DebugPanelItem {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for DebugPanelItem {
    type Event = DebugPanelItemEvent;

    fn tab_content(
        &self,
        params: workspace::item::TabContentParams,
        _window: &Window,
        cx: &App,
    ) -> AnyElement {
        Label::new(format!(
            "{} - Thread {}",
            self.session.read(cx).name(),
            self.thread_id
        ))
        .color(if params.selected {
            Color::Default
        } else {
            Color::Muted
        })
        .into_any_element()
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        Some(SharedString::from(format!(
            "{} Thread {} - {:?}",
            self.session.read(cx).name(),
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

impl FollowableItem for DebugPanelItem {
    fn remote_id(&self) -> Option<workspace::ViewId> {
        self.remote_id
    }

    fn to_state_proto(&self, _window: &Window, _cx: &App) -> Option<proto::view::Variant> {
        None
    }

    fn from_state_proto(
        _workspace: Entity<Workspace>,
        _remote_id: ViewId,
        _state: &mut Option<proto::view::Variant>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<gpui::Task<gpui::Result<Entity<Self>>>> {
        None
    }

    fn add_event_to_update_proto(
        &self,
        _event: &Self::Event,
        _update: &mut Option<proto::update_view::Variant>,
        _window: &Window,
        _cx: &App,
    ) -> bool {
        // update.get_or_insert_with(|| proto::update_view::Variant::DebugPanel(Default::default()));

        true
    }

    fn apply_update_proto(
        &mut self,
        _project: &Entity<project::Project>,
        _message: proto::update_view::Variant,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> gpui::Task<gpui::Result<()>> {
        Task::ready(Ok(()))
    }

    fn set_leader_peer_id(
        &mut self,
        _leader_peer_id: Option<PeerId>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn to_follow_event(_event: &Self::Event) -> Option<workspace::item::FollowEvent> {
        None
    }

    fn dedup(
        &self,
        existing: &Self,
        _window: &Window,
        _cx: &App,
    ) -> Option<workspace::item::Dedup> {
        if existing.client_id == self.client_id && existing.thread_id == self.thread_id {
            Some(item::Dedup::KeepExisting)
        } else {
            None
        }
    }

    fn is_project_item(&self, _window: &Window, _cx: &App) -> bool {
        true
    }
}

impl Render for DebugPanelItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let thread_status = self.thread_state.read(cx).status;
        let active_thread_item = &self.active_thread_item;

        let capabilities = self.capabilities(cx);

        h_flex()
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
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.pause_thread(cx);
                                            }))
                                            .tooltip(move |window, cx| {
                                                Tooltip::text("Pause program")(window, cx)
                                            }),
                                    )
                                } else {
                                    this.child(
                                        IconButton::new("debug-continue", IconName::DebugContinue)
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
                            .when(capabilities.supports_step_back.unwrap_or(false), |this| {
                                this.child(
                                    IconButton::new("debug-step-back", IconName::DebugStepBack)
                                        .icon_size(IconSize::Small)
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.step_back(cx);
                                        }))
                                        .disabled(thread_status != ThreadStatus::Stopped)
                                        .tooltip(move |window, cx| {
                                            Tooltip::text("Step back")(window, cx)
                                        }),
                                )
                            })
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
                                        !capabilities.supports_restart_request.unwrap_or_default(),
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
                                    .tooltip(move |window, cx| Tooltip::text("Stop")(window, cx)),
                            )
                            .child(
                                IconButton::new("debug-disconnect", IconName::DebugDisconnect)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.disconnect_client(cx);
                                    }))
                                    .disabled(
                                        thread_status == ThreadStatus::Exited
                                            || thread_status == ThreadStatus::Ended,
                                    )
                                    .tooltip(move |window, cx| {
                                        Tooltip::text("Disconnect")(window, cx)
                                    }),
                            )
                            .child(
                                IconButton::new(
                                    "debug-ignore-breakpoints",
                                    if self.session.read(cx).ignore_breakpoints() {
                                        IconName::DebugIgnoreBreakpoints
                                    } else {
                                        IconName::DebugBreakpoint
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
                                .tooltip(move |window, cx| {
                                    Tooltip::text("Ignore breakpoints")(window, cx)
                                }),
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
            .into_any()
    }
}

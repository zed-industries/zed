mod inert;
mod running;
mod starting;

use crate::debugger_panel::{DebugPanel, DebugPanelEvent};

use dap::{
    client::SessionId, debugger_settings::DebuggerSettings, Capabilities, ContinuedEvent,
    LoadedSourceEvent, ModuleEvent, OutputEvent, OutputEventCategory, StoppedEvent, ThreadEvent,
};
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity,
};
use inert::{InertEvent, InertState};
use project::debugger::session::Session;
use project::debugger::session::{ThreadId, ThreadStatus};

use rpc::proto::{self, PeerId};
use settings::Settings;
use starting::StartingState;
use ui::{prelude::*, ContextMenu, DropdownMenu, Indicator, Tooltip};
use workspace::{
    item::{self, Item, ItemEvent},
    FollowableItem, ViewId, Workspace,
};

enum DebugSessionState {
    Inert(Entity<InertState>),
    Starting(Entity<StartingState>),
    Running(Entity<running::RunningState>),
}

pub struct DebugSession {
    remote_id: Option<workspace::ViewId>,
    mode: DebugSessionState,
    _subscriptions: [Subscription; 1],
}
#[derive(Debug)]
pub enum DebugPanelItemEvent {
    Close,
    Stopped { go_to_stack_frame: bool },
}

#[derive(Clone, PartialEq, Eq)]
pub enum ThreadItem {
    Console,
    LoadedSource,
    Modules,
    Variables,
}

impl ThreadItem {
    fn _to_proto(&self) -> proto::DebuggerThreadItem {
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

impl DebugSession {
    pub(super) fn inert(cx: &mut App) -> Entity<Self> {
        let inert = cx.new(|cx| InertState::new(cx));

        cx.new(|cx| {
            let _subscriptions = [cx.subscribe(&inert, Self::on_inert_event)];
            Self {
                remote_id: None,
                mode: DebugSessionState::Inert(inert),
                _subscriptions,
            }
        })
    }
    pub(crate) fn session_id(&self, cx: &App) -> Option<SessionId> {
        match &self.mode {
            DebugSessionState::Inert(_) => None,
            DebugSessionState::Starting(_entity) => unimplemented!(),
            DebugSessionState::Running(entity) => Some(entity.read(cx).client_id()),
        }
    }
    fn on_inert_event(
        &mut self,
        _: Entity<InertState>,
        _: &InertEvent,
        cx: &mut Context<'_, Self>,
    ) {
        self.mode = DebugSessionState::Starting(cx.new(|cx| {
            let task = cx.background_executor().spawn(async move {
                std::future::pending::<()>().await;
                Ok(())
            });
            StartingState::new(task, cx)
        }));
    }
}
impl EventEmitter<DebugPanelItemEvent> for DebugSession {}

impl Focusable for DebugSession {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            DebugSessionState::Inert(inert_state) => inert_state.focus_handle(cx),
            DebugSessionState::Starting(starting_state) => starting_state.focus_handle(cx),
            DebugSessionState::Running(running_state) => running_state.focus_handle(cx),
        }
    }
}

impl Item for DebugSession {
    type Event = DebugPanelItemEvent;
    fn tab_content(&self, _: item::TabContentParams, _: &Window, _: &App) -> AnyElement {
        let label = match &self.mode {
            DebugSessionState::Inert(_) => "New Session",
            DebugSessionState::Starting(_) => "Starting",
            DebugSessionState::Running(_) => "Running",
        };
        div().child(Label::new(label)).into_any_element()
    }
}

impl FollowableItem for DebugSession {
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

    fn dedup(&self, existing: &Self, _window: &Window, cx: &App) -> Option<workspace::item::Dedup> {
        if existing.session_id(cx) == self.session_id(cx) {
            Some(item::Dedup::KeepExisting)
        } else {
            None
        }
    }

    fn is_project_item(&self, _window: &Window, _cx: &App) -> bool {
        true
    }
}

impl Render for DebugSession {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        match &self.mode {
            DebugSessionState::Inert(inert_state) => {
                inert_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
            DebugSessionState::Starting(starting_state) => {
                starting_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
            DebugSessionState::Running(running_state) => {
                running_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
        }
    }
}

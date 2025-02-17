mod inert;
mod running;
mod starting;

use crate::debugger_panel::{DebugPanel, DebugPanelEvent};

use dap::{
    client::DebugAdapterClientId, debugger_settings::DebuggerSettings, Capabilities,
    ContinuedEvent, LoadedSourceEvent, ModuleEvent, OutputEvent, OutputEventCategory, StoppedEvent,
    ThreadEvent,
};
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity,
};
use inert::InertState;
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

pub enum DebugSession {
    Inert(Entity<InertState>),
    Starting(Entity<StartingState>),
    Running(Entity<running::RunningState>),
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

impl EventEmitter<DebugPanelItemEvent> for DebugSession {}

impl Focusable for DebugSession {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self {
            DebugSession::Inert(inert_state) => inert_state.focus_handle(cx),
            DebugSession::Starting(starting_state) => starting_state.focus_handle(cx),
            DebugSession::Running(running_state) => running_state.focus_handle(cx),
        }
    }
}

impl Item for DebugSession {
    type Event = DebugPanelItemEvent;
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

impl Render for DebugSession {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        match self {
            DebugSession::Inert(inert_state) => {
                inert_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
            DebugSession::Starting(starting_state) => {
                starting_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
            DebugSession::Running(running_state) => {
                running_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
        }
    }
}

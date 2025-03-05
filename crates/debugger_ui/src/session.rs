mod inert;
pub mod running;
mod starting;

use dap::client::SessionId;
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity,
};
use inert::{InertEvent, InertState};
use project::debugger::{dap_store::DapStore, session::Session};
use project::worktree_store::WorktreeStore;
use project::Project;
use rpc::proto::{self, PeerId};
use running::RunningState;
use starting::{StartingEvent, StartingState};
use ui::prelude::*;
use workspace::{
    item::{self, Item},
    FollowableItem, ViewId, Workspace,
};

pub(crate) enum DebugSessionState {
    Inert(Entity<InertState>),
    Starting(Entity<StartingState>),
    Running(Entity<running::RunningState>),
}

impl DebugSessionState {
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn as_running(&self) -> Option<&Entity<running::RunningState>> {
        match &self {
            DebugSessionState::Running(entity) => Some(entity),
            _ => None,
        }
    }
}

pub struct DebugSession {
    remote_id: Option<workspace::ViewId>,
    mode: DebugSessionState,
    dap_store: WeakEntity<DapStore>,
    worktree_store: WeakEntity<WorktreeStore>,
    workspace: WeakEntity<Workspace>,
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

impl DebugSession {
    pub(super) fn inert(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let inert = cx.new(|cx| InertState::new(window, cx));

        let project = project.read(cx);
        let dap_store = project.dap_store().downgrade();
        let worktree_store = project.worktree_store().downgrade();
        cx.new(|cx| {
            let _subscriptions = [cx.subscribe_in(&inert, window, Self::on_inert_event)];
            Self {
                remote_id: None,
                mode: DebugSessionState::Inert(inert),
                dap_store,
                worktree_store,
                workspace,
                _subscriptions,
            }
        })
    }

    pub(crate) fn running(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        session: Entity<Session>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let mode = DebugSessionState::Running(
            cx.new(|cx| RunningState::new(session.clone(), workspace.clone(), window, cx)),
        );

        cx.new(|cx| Self {
            remote_id: None,
            mode,
            dap_store: project.read(cx).dap_store().downgrade(),
            worktree_store: project.read(cx).worktree_store().downgrade(),
            workspace,
            _subscriptions: [cx.subscribe(&project, |_, _, _, _| {})], // todo(debugger) We don't need this subscription
        })
    }

    pub(crate) fn session_id(&self, cx: &App) -> Option<SessionId> {
        match &self.mode {
            DebugSessionState::Inert(_) => None,
            DebugSessionState::Starting(_entity) => unimplemented!(),
            DebugSessionState::Running(entity) => Some(entity.read(cx).session_id()),
        }
    }

    pub(crate) fn shutdown(&mut self, cx: &mut Context<Self>) {
        match &self.mode {
            DebugSessionState::Inert(_) => {}
            DebugSessionState::Starting(_entity) => {} // todo(debugger): we need to shutdown the starting process in this case (or recreate it on a breakpoint being hit)
            DebugSessionState::Running(state) => state.update(cx, |state, cx| state.shutdown(cx)),
        }
    }

    #[cfg(any(test, feature = "test-feature"))]
    pub(crate) fn mode(&self) -> &DebugSessionState {
        &self.mode
    }

    fn on_inert_event(
        &mut self,
        _: &Entity<InertState>,
        event: &InertEvent,
        window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) {
        let dap_store = self.dap_store.clone();
        let InertEvent::Spawned { config } = event;
        let config = config.clone();
        let worktree = self
            .worktree_store
            .update(cx, |this, _| this.worktrees().next())
            .ok()
            .flatten()
            .expect("worktree-less project");
        let Ok(task) = dap_store.update(cx, |store, cx| {
            store.new_session(config, &worktree, None, cx)
        }) else {
            return;
        };
        let starting = cx.new(|cx| StartingState::new(task, cx));

        self._subscriptions = [cx.subscribe_in(&starting, window, Self::on_starting_event)];
        self.mode = DebugSessionState::Starting(starting);
    }

    fn on_starting_event(
        &mut self,
        _: &Entity<StartingState>,
        event: &StartingEvent,
        window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) {
        let StartingEvent::Finished(Ok(session)) = event else {
            return;
        };

        let mode =
            cx.new(|cx| RunningState::new(session.clone(), self.workspace.clone(), window, cx));

        self.mode = DebugSessionState::Running(mode);
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

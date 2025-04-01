mod failed;
mod inert;
pub mod running;
mod starting;

use std::time::Duration;

use dap::client::SessionId;
use failed::FailedState;
use gpui::{
    Animation, AnimationExt, AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, Task, Transformation, WeakEntity, percentage,
};
use inert::{InertEvent, InertState};
use project::Project;
use project::debugger::{dap_store::DapStore, session::Session};
use project::worktree_store::WorktreeStore;
use rpc::proto::{self, PeerId};
use running::RunningState;
use starting::{StartingEvent, StartingState};
use task::DebugTaskDefinition;
use ui::{Indicator, prelude::*};
use util::ResultExt;
use workspace::{
    FollowableItem, ViewId, Workspace,
    item::{self, Item},
};

use crate::debugger_panel::DebugPanel;

pub(crate) enum DebugSessionState {
    Inert(Entity<InertState>),
    Starting(Entity<StartingState>),
    Failed(Entity<FailedState>),
    Running(Entity<running::RunningState>),
}

impl DebugSessionState {
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
    debug_panel: WeakEntity<DebugPanel>,
    worktree_store: WeakEntity<WorktreeStore>,
    workspace: WeakEntity<Workspace>,
    _subscriptions: [Subscription; 1],
}

#[derive(Debug)]
pub enum DebugPanelItemEvent {
    Close,
    Stopped { go_to_stack_frame: bool },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        debug_panel: WeakEntity<DebugPanel>,
        config: Option<DebugTaskDefinition>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let default_cwd = project
            .read(cx)
            .worktrees(cx)
            .next()
            .and_then(|tree| tree.read(cx).abs_path().to_str().map(|str| str.to_string()))
            .unwrap_or_default();

        let inert =
            cx.new(|cx| InertState::new(workspace.clone(), &default_cwd, config, window, cx));

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
                debug_panel,
                workspace,
                _subscriptions,
            }
        })
    }

    pub(crate) fn running(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        session: Entity<Session>,
        debug_panel: WeakEntity<DebugPanel>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let mode = cx.new(|cx| RunningState::new(session.clone(), workspace.clone(), window, cx));

        cx.new(|cx| Self {
            _subscriptions: [cx.subscribe(&mode, |_, _, _, cx| {
                cx.notify();
            })],
            remote_id: None,
            mode: DebugSessionState::Running(mode),
            dap_store: project.read(cx).dap_store().downgrade(),
            debug_panel,
            worktree_store: project.read(cx).worktree_store().downgrade(),
            workspace,
        })
    }

    pub(crate) fn session_id(&self, cx: &App) -> Option<SessionId> {
        match &self.mode {
            DebugSessionState::Inert(_) => None,
            DebugSessionState::Starting(entity) => Some(entity.read(cx).session_id),
            DebugSessionState::Failed(_) => None,
            DebugSessionState::Running(entity) => Some(entity.read(cx).session_id()),
        }
    }

    pub(crate) fn shutdown(&mut self, cx: &mut Context<Self>) {
        match &self.mode {
            DebugSessionState::Inert(_) => {}
            DebugSessionState::Starting(_entity) => {} // todo(debugger): we need to shutdown the starting process in this case (or recreate it on a breakpoint being hit)
            DebugSessionState::Failed(_) => {}
            DebugSessionState::Running(state) => state.update(cx, |state, cx| state.shutdown(cx)),
        }
    }

    pub(crate) fn mode(&self) -> &DebugSessionState {
        &self.mode
    }

    fn on_inert_event(
        &mut self,
        _: &Entity<InertState>,
        event: &InertEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let dap_store = self.dap_store.clone();
        let InertEvent::Spawned { config } = event;
        let config = config.clone();

        self.debug_panel
            .update(cx, |this, _| this.last_inert_config = Some(config.clone()))
            .log_err();

        let worktree = self
            .worktree_store
            .update(cx, |this, _| this.worktrees().next())
            .ok()
            .flatten()
            .expect("worktree-less project");
        let Ok((new_session_id, task)) = dap_store.update(cx, |store, cx| {
            store.new_session(config.into(), &worktree, None, cx)
        }) else {
            return;
        };
        let starting = cx.new(|cx| StartingState::new(new_session_id, task, cx));

        self._subscriptions = [cx.subscribe_in(&starting, window, Self::on_starting_event)];
        self.mode = DebugSessionState::Starting(starting);
    }

    fn on_starting_event(
        &mut self,
        _: &Entity<StartingState>,
        event: &StartingEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let StartingEvent::Finished(session) = event {
            let mode =
                cx.new(|cx| RunningState::new(session.clone(), self.workspace.clone(), window, cx));
            self.mode = DebugSessionState::Running(mode);
        } else if let StartingEvent::Failed = event {
            self.mode = DebugSessionState::Failed(cx.new(FailedState::new));
        };
        cx.notify();
    }
}
impl EventEmitter<DebugPanelItemEvent> for DebugSession {}

impl Focusable for DebugSession {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            DebugSessionState::Inert(inert_state) => inert_state.focus_handle(cx),
            DebugSessionState::Starting(starting_state) => starting_state.focus_handle(cx),
            DebugSessionState::Failed(failed_state) => failed_state.focus_handle(cx),
            DebugSessionState::Running(running_state) => running_state.focus_handle(cx),
        }
    }
}

impl Item for DebugSession {
    type Event = DebugPanelItemEvent;
    fn tab_content(&self, _: item::TabContentParams, _: &Window, cx: &App) -> AnyElement {
        let (icon, label, color) = match &self.mode {
            DebugSessionState::Inert(_) => (None, "New Session", Color::Default),
            DebugSessionState::Starting(_) => (None, "Starting", Color::Default),
            DebugSessionState::Failed(_) => (
                Some(Indicator::dot().color(Color::Error)),
                "Failed",
                Color::Error,
            ),
            DebugSessionState::Running(state) => {
                if state.read(cx).session().read(cx).is_terminated() {
                    (
                        Some(Indicator::dot().color(Color::Error)),
                        "Terminated",
                        Color::Error,
                    )
                } else {
                    match state.read(cx).thread_status(cx).unwrap_or_default() {
                        project::debugger::session::ThreadStatus::Stopped => (
                            Some(Indicator::dot().color(Color::Conflict)),
                            state
                                .read_with(cx, |state, cx| state.thread_status(cx))
                                .map(|status| status.label())
                                .unwrap_or("Stopped"),
                            Color::Conflict,
                        ),
                        _ => (
                            Some(Indicator::dot().color(Color::Success)),
                            state
                                .read_with(cx, |state, cx| state.thread_status(cx))
                                .map(|status| status.label())
                                .unwrap_or("Running"),
                            Color::Success,
                        ),
                    }
                }
            }
        };

        let is_starting = matches!(self.mode, DebugSessionState::Starting(_));

        h_flex()
            .gap_2()
            .children(is_starting.then(|| {
                Icon::new(IconName::ArrowCircle).with_animation(
                    "starting-debug-session",
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |this, delta| this.transform(Transformation::rotate(percentage(delta))),
                )
            }))
            .when_some(icon, |this, indicator| this.child(indicator))
            .justify_between()
            .child(Label::new(label).color(color))
            .into_any_element()
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.mode {
            DebugSessionState::Inert(inert_state) => {
                inert_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
            DebugSessionState::Starting(starting_state) => {
                starting_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
            DebugSessionState::Failed(failed_state) => {
                failed_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
            DebugSessionState::Running(running_state) => {
                running_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
        }
    }
}

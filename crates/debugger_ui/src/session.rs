pub mod running;

use std::sync::OnceLock;

use dap::client::SessionId;
use gpui::{App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity};
use project::Project;
use project::debugger::{dap_store::DapStore, session::Session};
use project::worktree_store::WorktreeStore;
use rpc::proto::{self, PeerId};
use running::RunningState;
use ui::{Indicator, prelude::*};
use workspace::{
    FollowableItem, ViewId, Workspace,
    item::{self, Item},
};

use crate::debugger_panel::DebugPanel;
use crate::persistence::SerializedPaneLayout;

pub(crate) enum DebugSessionState {
    Running(Entity<running::RunningState>),
}

impl DebugSessionState {
    pub(crate) fn as_running(&self) -> Option<&Entity<running::RunningState>> {
        match &self {
            DebugSessionState::Running(entity) => Some(entity),
        }
    }
}

pub struct DebugSession {
    remote_id: Option<workspace::ViewId>,
    mode: DebugSessionState,
    label: OnceLock<String>,
    dap_store: WeakEntity<DapStore>,
    _debug_panel: WeakEntity<DebugPanel>,
    _worktree_store: WeakEntity<WorktreeStore>,
    _workspace: WeakEntity<Workspace>,
    _subscriptions: [Subscription; 1],
}

#[derive(Debug)]
pub enum DebugPanelItemEvent {
    Close,
    Stopped { go_to_stack_frame: bool },
}

impl DebugSession {
    pub(crate) fn running(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        session: Entity<Session>,
        _debug_panel: WeakEntity<DebugPanel>,
        serialized_pane_layout: Option<SerializedPaneLayout>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let mode = cx.new(|cx| {
            RunningState::new(
                session.clone(),
                project.clone(),
                workspace.clone(),
                serialized_pane_layout,
                window,
                cx,
            )
        });

        cx.new(|cx| Self {
            _subscriptions: [cx.subscribe(&mode, |_, _, _, cx| {
                cx.notify();
            })],
            remote_id: None,
            mode: DebugSessionState::Running(mode),
            label: OnceLock::new(),
            dap_store: project.read(cx).dap_store().downgrade(),
            _debug_panel,
            _worktree_store: project.read(cx).worktree_store().downgrade(),
            _workspace: workspace,
        })
    }

    pub(crate) fn session_id(&self, cx: &App) -> SessionId {
        match &self.mode {
            DebugSessionState::Running(entity) => entity.read(cx).session_id(),
        }
    }

    pub fn session(&self, cx: &App) -> Entity<Session> {
        match &self.mode {
            DebugSessionState::Running(entity) => entity.read(cx).session().clone(),
        }
    }

    pub(crate) fn shutdown(&mut self, cx: &mut Context<Self>) {
        match &self.mode {
            DebugSessionState::Running(state) => state.update(cx, |state, cx| state.shutdown(cx)),
        }
    }

    pub(crate) fn mode(&self) -> &DebugSessionState {
        &self.mode
    }

    pub(crate) fn running_state(&self) -> Entity<RunningState> {
        match &self.mode {
            DebugSessionState::Running(running_state) => running_state.clone(),
        }
    }

    pub(crate) fn label(&self, cx: &App) -> String {
        if let Some(label) = self.label.get() {
            return label.to_owned();
        }

        let session_id = match &self.mode {
            DebugSessionState::Running(running_state) => running_state.read(cx).session_id(),
        };

        let Ok(Some(session)) = self
            .dap_store
            .read_with(cx, |store, _| store.session_by_id(session_id))
        else {
            return "".to_owned();
        };

        self.label
            .get_or_init(|| session.read(cx).label())
            .to_owned()
    }

    pub(crate) fn label_element(&self, cx: &App) -> AnyElement {
        let label = self.label(cx);

        let icon = match &self.mode {
            DebugSessionState::Running(state) => {
                if state.read(cx).session().read(cx).is_terminated() {
                    Some(Indicator::dot().color(Color::Error))
                } else {
                    match state.read(cx).thread_status(cx).unwrap_or_default() {
                        project::debugger::session::ThreadStatus::Stopped => {
                            Some(Indicator::dot().color(Color::Conflict))
                        }
                        _ => Some(Indicator::dot().color(Color::Success)),
                    }
                }
            }
        };

        h_flex()
            .gap_2()
            .when_some(icon, |this, indicator| this.child(indicator))
            .justify_between()
            .child(Label::new(label))
            .into_any_element()
    }
}

impl EventEmitter<DebugPanelItemEvent> for DebugSession {}

impl Focusable for DebugSession {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            DebugSessionState::Running(running_state) => running_state.focus_handle(cx),
        }
    }
}

impl Item for DebugSession {
    type Event = DebugPanelItemEvent;
    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Debugger".into()
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
            DebugSessionState::Running(running_state) => {
                running_state.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
        }
    }
}

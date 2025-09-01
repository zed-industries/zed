pub mod running;

use crate::{StackTraceView, persistence::SerializedLayout, session::running::DebugTerminal};
use dap::client::SessionId;
use gpui::{
    App, Axis, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity,
};
use project::debugger::session::Session;
use project::worktree_store::WorktreeStore;
use project::{Project, debugger::session::SessionQuirks};
use rpc::proto;
use running::RunningState;
use std::cell::OnceCell;
use ui::prelude::*;
use workspace::{
    CollaboratorId, FollowableItem, ViewId, Workspace,
    item::{self, Item},
};

pub struct DebugSession {
    remote_id: Option<workspace::ViewId>,
    pub(crate) running_state: Entity<RunningState>,
    pub(crate) quirks: SessionQuirks,
    stack_trace_view: OnceCell<Entity<StackTraceView>>,
    _worktree_store: WeakEntity<WorktreeStore>,
    workspace: WeakEntity<Workspace>,
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
        parent_terminal: Option<Entity<DebugTerminal>>,
        session: Entity<Session>,
        serialized_layout: Option<SerializedLayout>,
        dock_axis: Axis,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let running_state = cx.new(|cx| {
            RunningState::new(
                session.clone(),
                project.clone(),
                workspace.clone(),
                parent_terminal,
                serialized_layout,
                dock_axis,
                window,
                cx,
            )
        });
        let quirks = session.read(cx).quirks();

        cx.new(|cx| Self {
            _subscriptions: [cx.subscribe(&running_state, |_, _, _, cx| {
                cx.notify();
            })],
            remote_id: None,
            running_state,
            quirks,
            stack_trace_view: OnceCell::new(),
            _worktree_store: project.read(cx).worktree_store().downgrade(),
            workspace,
        })
    }

    pub(crate) fn session_id(&self, cx: &App) -> SessionId {
        self.running_state.read(cx).session_id()
    }

    pub(crate) fn stack_trace_view(
        &mut self,
        project: &Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> &Entity<StackTraceView> {
        let workspace = self.workspace.clone();
        let running_state = self.running_state.clone();

        self.stack_trace_view.get_or_init(|| {
            let stackframe_list = running_state.read(cx).stack_frame_list().clone();

            cx.new(|cx| {
                StackTraceView::new(
                    workspace.clone(),
                    project.clone(),
                    stackframe_list,
                    window,
                    cx,
                )
            })
        })
    }

    pub fn session(&self, cx: &App) -> Entity<Session> {
        self.running_state.read(cx).session().clone()
    }

    pub(crate) fn shutdown(&mut self, cx: &mut Context<Self>) {
        self.running_state
            .update(cx, |state, cx| state.shutdown(cx));
    }

    pub(crate) fn label(&self, cx: &mut App) -> Option<SharedString> {
        let session = self.running_state.read(cx).session().clone();
        session.update(cx, |session, cx| {
            let session_label = session.label();
            let quirks = session.quirks();
            let mut single_thread_name = || {
                let threads = session.threads(cx);
                match threads.as_slice() {
                    [(thread, _)] => Some(SharedString::from(&thread.name)),
                    _ => None,
                }
            };
            if quirks.prefer_thread_name {
                single_thread_name().or(session_label)
            } else {
                session_label.or_else(single_thread_name)
            }
        })
    }

    pub fn running_state(&self) -> &Entity<RunningState> {
        &self.running_state
    }
}

impl EventEmitter<DebugPanelItemEvent> for DebugSession {}

impl Focusable for DebugSession {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.running_state.focus_handle(cx)
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
    ) -> Option<gpui::Task<anyhow::Result<Entity<Self>>>> {
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
    ) -> gpui::Task<anyhow::Result<()>> {
        Task::ready(Ok(()))
    }

    fn set_leader_id(
        &mut self,
        _leader_id: Option<CollaboratorId>,
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
        self.running_state
            .update(cx, |this, cx| this.render(window, cx).into_any_element())
    }
}

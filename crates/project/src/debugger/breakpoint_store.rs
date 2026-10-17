//! Module for managing breakpoints in a project.
//!
//! Breakpoints are separate from a session because they're not associated with any particular debug session. They can also be set up without a session running.
use anyhow::{Context as _, Result};
pub use breakpoints_in_file::{BreakpointSessionState, BreakpointWithPosition};
use breakpoints_in_file::{BreakpointsInFile, StatefulBreakpoint};
use collections::{BTreeMap, HashMap, btree_map};
use dap::{StackFrameId, client::SessionId};
use futures::future::join_all;
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EntityId, EventEmitter, Subscription, Task,
};
use language::{
    Buffer, BufferEvent, BufferSnapshot, proto::serialize_anchor as serialize_text_anchor,
};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self},
};
use std::{hash::Hash, ops::Range, path::Path, sync::Arc};
use text::{Bias, Point, PointUtf16, Unclipped};
use util::maybe;

use crate::{
    ProjectPath,
    buffer_store::{BufferStore, BufferStoreEvent},
    worktree_store::WorktreeStore,
};

use super::session::ThreadId;

mod breakpoints_in_file {
    use collections::HashMap;

    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BreakpointWithPosition {
        pub position: text::Anchor,
        pub bp: Breakpoint,
    }

    /// A breakpoint with per-session data about it's state (as seen by the Debug Adapter).
    #[derive(Clone, Debug)]
    pub struct StatefulBreakpoint {
        pub bp: BreakpointWithPosition,
        pub session_state: HashMap<SessionId, BreakpointSessionState>,
    }

    impl StatefulBreakpoint {
        pub(super) fn new(bp: BreakpointWithPosition) -> Self {
            Self {
                bp,
                session_state: Default::default(),
            }
        }
        pub(super) fn position(&self) -> &text::Anchor {
            &self.bp.position
        }
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct BreakpointSessionState {
        /// Session-specific identifier for the breakpoint, as assigned by Debug Adapter.
        pub id: u64,
        pub verified: bool,
    }
    #[derive(Default)]
    pub(super) struct BreakpointsInFile {
        pub(super) buffer: Option<Entity<Buffer>>,
        // TODO: This is.. less than ideal, as it's O(n) and does not return entries in order. We'll have to change TreeMap to support passing in the context for comparisons
        pub(super) breakpoints: Vec<StatefulBreakpoint>,
        pub(super) pending: Vec<SourceBreakpoint>,
        _subscription: Option<Subscription>,
    }

    impl BreakpointsInFile {
        pub(super) fn set_buffer(
            &mut self,
            buffer: Entity<Buffer>,
            cx: &mut Context<BreakpointStore>,
        ) -> bool {
            if self.buffer.as_ref() == Some(&buffer) {
                return false;
            }
            if let Some(previous) = &self.buffer {
                let previous = previous.read(cx).snapshot();
                let snapshot = buffer.read(cx).snapshot();
                for breakpoint in &mut self.breakpoints {
                    let row = previous
                        .summary_for_anchor::<PointUtf16>(breakpoint.position())
                        .row;
                    let position =
                        snapshot.clip_point_utf16(Unclipped(PointUtf16::new(row, 0)), Bias::Left);
                    breakpoint.bp.position = snapshot.anchor_after(position);
                }
            }
            self._subscription = Some(cx.subscribe(&buffer, BreakpointStore::on_buffer_event));
            self.buffer = Some(buffer);
            true
        }

        pub(super) fn resolve_pending(&mut self, cx: &App) -> bool {
            if self.pending.is_empty() {
                return false;
            }
            let Some(buffer) = &self.buffer else {
                return false;
            };
            let snapshot = buffer.read(cx).snapshot();
            let previous_count = self.pending.len();
            self.pending.retain(|breakpoint| {
                let point = PointUtf16::new(breakpoint.row, 0);
                if point > snapshot.max_point_utf16() {
                    return true;
                }
                self.breakpoints
                    .push(StatefulBreakpoint::new(BreakpointWithPosition {
                        position: snapshot.anchor_after(point),
                        bp: Breakpoint {
                            message: breakpoint.message.clone(),
                            state: breakpoint.state,
                            condition: breakpoint.condition.clone(),
                            hit_condition: breakpoint.hit_condition.clone(),
                        },
                    }));
                false
            });
            self.pending.len() != previous_count
        }

        pub(super) fn is_empty(&self) -> bool {
            self.breakpoints.is_empty() && self.pending.is_empty()
        }
    }
}

#[derive(Clone)]
struct RemoteBreakpointStore {
    upstream_client: AnyProtoClient,
    upstream_project_id: u64,
}

#[derive(Clone)]
enum BreakpointStoreMode {
    Local,
    Remote(RemoteBreakpointStore),
}

#[derive(Clone, PartialEq)]
pub struct ActiveStackFrame {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub stack_frame_id: StackFrameId,
    pub path: Arc<Path>,
    pub position: text::Anchor,
}

pub struct BreakpointStore {
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    breakpoints: BTreeMap<Arc<Path>, BreakpointsInFile>,
    buffer_subscription: Option<Subscription>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    active_stack_frame: Option<ActiveStackFrame>,
    active_debug_line_pane_id: Option<EntityId>,
    // E.g ssh
    mode: BreakpointStoreMode,
}

impl BreakpointStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_toggle_breakpoint);
        client.add_entity_message_handler(Self::handle_breakpoints_for_file);
    }
    pub fn local(worktree_store: Entity<WorktreeStore>, buffer_store: Entity<BufferStore>) -> Self {
        BreakpointStore {
            breakpoints: BTreeMap::new(),
            buffer_subscription: None,
            mode: BreakpointStoreMode::Local,
            buffer_store,
            worktree_store,
            downstream_client: None,
            active_stack_frame: Default::default(),
            active_debug_line_pane_id: None,
        }
    }

    pub(crate) fn remote(
        upstream_project_id: u64,
        upstream_client: AnyProtoClient,
        buffer_store: Entity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
    ) -> Self {
        BreakpointStore {
            breakpoints: BTreeMap::new(),
            buffer_subscription: None,
            mode: BreakpointStoreMode::Remote(RemoteBreakpointStore {
                upstream_client,
                upstream_project_id,
            }),
            buffer_store,
            worktree_store,
            downstream_client: None,
            active_stack_frame: Default::default(),
            active_debug_line_pane_id: None,
        }
    }

    pub fn shared(&mut self, project_id: u64, downstream_client: AnyProtoClient) {
        self.downstream_client = Some((downstream_client, project_id));
    }

    pub(crate) fn unshared(&mut self, cx: &mut Context<Self>) {
        self.downstream_client.take();

        cx.notify();
    }

    async fn handle_breakpoints_for_file(
        this: Entity<Self>,
        message: TypedEnvelope<proto::BreakpointsForFile>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        if message.payload.breakpoints.is_empty() {
            return Ok(());
        }

        let buffer = this
            .update(&mut cx, |this, cx| {
                let path = this
                    .worktree_store
                    .read(cx)
                    .project_path_for_absolute_path(message.payload.path.as_ref(), cx)?;
                Some(
                    this.buffer_store
                        .update(cx, |this, cx| this.open_buffer(path, cx)),
                )
            })
            .context("Invalid project path")?
            .await?;

        this.update(&mut cx, move |this, cx| {
            let bps = this
                .breakpoints
                .entry(Arc::<Path>::from(message.payload.path.as_ref()))
                .or_default();
            bps.set_buffer(buffer, cx);

            bps.breakpoints = message
                .payload
                .breakpoints
                .into_iter()
                .filter_map(|breakpoint| {
                    let position = language::proto::deserialize_anchor(breakpoint.position?)?;
                    let session_state = breakpoint
                        .session_state
                        .iter()
                        .map(|(session_id, state)| {
                            let state = BreakpointSessionState {
                                id: state.id,
                                verified: state.verified,
                            };
                            (SessionId::from_proto(*session_id), state)
                        })
                        .collect();
                    let breakpoint = Breakpoint::from_proto(breakpoint)?;
                    let bp = BreakpointWithPosition {
                        position,
                        bp: breakpoint,
                    };

                    Some(StatefulBreakpoint { bp, session_state })
                })
                .collect();

            cx.notify();
        });

        Ok(())
    }

    async fn handle_toggle_breakpoint(
        this: Entity<Self>,
        message: TypedEnvelope<proto::ToggleBreakpoint>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let path = this
            .update(&mut cx, |this, cx| {
                this.worktree_store
                    .read(cx)
                    .project_path_for_absolute_path(message.payload.path.as_ref(), cx)
            })
            .context("Could not resolve provided abs path")?;
        let buffer = this
            .update(&mut cx, |this, cx| {
                this.buffer_store.read(cx).get_by_path(&path)
            })
            .context("Could not find buffer for a given path")?;
        let breakpoint = message
            .payload
            .breakpoint
            .context("Breakpoint not present in RPC payload")?;
        let position = language::proto::deserialize_anchor(
            breakpoint
                .position
                .context("Anchor not present in RPC payload")?,
        )
        .context("Anchor deserialization failed")?;
        let breakpoint =
            Breakpoint::from_proto(breakpoint).context("Could not deserialize breakpoint")?;

        this.update(&mut cx, |this, cx| {
            this.toggle_breakpoint(
                buffer,
                BreakpointWithPosition {
                    position,
                    bp: breakpoint,
                },
                BreakpointEditAction::Toggle,
                cx,
            );
        });
        Ok(proto::Ack {})
    }

    pub(crate) fn broadcast(&self) {
        if let Some((client, project_id)) = &self.downstream_client {
            for (path, breakpoint_set) in &self.breakpoints {
                let _ = client.send(proto::BreakpointsForFile {
                    project_id: *project_id,
                    path: path.to_string_lossy().into_owned(),
                    breakpoints: breakpoint_set
                        .breakpoints
                        .iter()
                        .filter_map(|breakpoint| {
                            breakpoint.bp.bp.to_proto(
                                path,
                                breakpoint.position(),
                                &breakpoint.session_state,
                            )
                        })
                        .collect(),
                });
            }
        }
    }

    pub(crate) fn update_session_breakpoint(
        &mut self,
        session_id: SessionId,
        _: dap::BreakpointEventReason,
        breakpoint: dap::Breakpoint,
    ) {
        maybe!({
            let event_id = breakpoint.id?;

            let state = self
                .breakpoints
                .values_mut()
                .find_map(|breakpoints_in_file| {
                    breakpoints_in_file
                        .breakpoints
                        .iter_mut()
                        .find_map(|state| {
                            let state = state.session_state.get_mut(&session_id)?;

                            if state.id == event_id {
                                Some(state)
                            } else {
                                None
                            }
                        })
                })?;

            state.verified = breakpoint.verified;
            Some(())
        });
    }

    pub(super) fn mark_breakpoints_verified(
        &mut self,
        session_id: SessionId,
        abs_path: &Path,

        it: impl Iterator<Item = (BreakpointWithPosition, BreakpointSessionState)>,
    ) {
        maybe!({
            let breakpoints = self.breakpoints.get_mut(abs_path)?;
            for (breakpoint, state) in it {
                if let Some(to_update) = breakpoints
                    .breakpoints
                    .iter_mut()
                    .find(|bp| *bp.position() == breakpoint.position)
                {
                    to_update
                        .session_state
                        .entry(session_id)
                        .insert_entry(state);
                }
            }
            Some(())
        });
    }

    pub fn abs_path_from_buffer(buffer: &Entity<Buffer>, cx: &App) -> Option<Arc<Path>> {
        worktree::File::from_dyn(buffer.read(cx).file())
            .map(|file| file.worktree.read(cx).absolutize(&file.path))
            .map(Arc::<Path>::from)
    }

    pub fn edit_source_breakpoint(
        &mut self,
        path: Arc<Path>,
        row: u32,
        action: BreakpointEditAction,
        cx: &mut Context<Self>,
    ) {
        if let Some((buffer, breakpoint)) = self.breakpoint_at_row(&path, row, cx) {
            self.toggle_breakpoint(buffer, breakpoint, action, cx);
            return;
        }
        let Some(file) = self.breakpoints.get_mut(&path) else {
            return;
        };
        let pending = &mut file.pending;
        let Some(index) = pending.iter().position(|breakpoint| breakpoint.row == row) else {
            return;
        };
        let breakpoint = &mut pending[index];
        let remove_duplicates = matches!(action, BreakpointEditAction::Toggle);
        let mut remove = false;
        let property = match &action {
            BreakpointEditAction::Toggle => {
                remove = true;
                None
            }
            BreakpointEditAction::InvertState => {
                breakpoint.state = if breakpoint.state.is_enabled() {
                    BreakpointState::Disabled
                } else {
                    BreakpointState::Enabled
                };
                None
            }
            action => action.property(
                &mut breakpoint.message,
                &mut breakpoint.condition,
                &mut breakpoint.hit_condition,
            ),
        };
        if let Some((property, value)) = property {
            if value.is_empty() {
                remove = property.is_some();
            } else {
                *property = Some(value.clone());
            }
        }
        if remove {
            let removed = pending.remove(index);
            if remove_duplicates {
                pending.retain(|breakpoint| breakpoint != &removed);
            }
        }
        if file.is_empty() {
            self.breakpoints.remove(&path);
        }
        cx.emit(BreakpointStoreEvent::BreakpointsUpdated(
            path,
            BreakpointUpdatedReason::Toggled,
        ));
        cx.notify();
    }

    pub fn toggle_breakpoint(
        &mut self,
        buffer: Entity<Buffer>,
        mut breakpoint: BreakpointWithPosition,
        edit_action: BreakpointEditAction,
        cx: &mut Context<Self>,
    ) {
        let Some(abs_path) = Self::abs_path_from_buffer(&buffer, cx) else {
            return;
        };

        let breakpoint_set = self.breakpoints.entry(abs_path.clone()).or_default();
        breakpoint_set.set_buffer(buffer, cx);
        breakpoint_set.resolve_pending(cx);

        match edit_action {
            BreakpointEditAction::Toggle => {
                let len_before = breakpoint_set.breakpoints.len();
                breakpoint_set
                    .breakpoints
                    .retain(|value| breakpoint != value.bp);
                if len_before == breakpoint_set.breakpoints.len() {
                    // We did not remove any breakpoint, hence let's toggle one.
                    breakpoint_set
                        .breakpoints
                        .push(StatefulBreakpoint::new(breakpoint.clone()));
                }
            }
            BreakpointEditAction::InvertState => {
                if let Some(bp) = breakpoint_set
                    .breakpoints
                    .iter_mut()
                    .find(|value| breakpoint == value.bp)
                {
                    let bp = &mut bp.bp.bp;
                    if bp.is_enabled() {
                        bp.state = BreakpointState::Disabled;
                    } else {
                        bp.state = BreakpointState::Enabled;
                    }
                } else {
                    breakpoint.bp.state = BreakpointState::Disabled;
                    breakpoint_set
                        .breakpoints
                        .push(StatefulBreakpoint::new(breakpoint.clone()));
                }
            }
            action => {
                if let Some((property, value)) = action.property(
                    &mut breakpoint.bp.message,
                    &mut breakpoint.bp.condition,
                    &mut breakpoint.bp.hit_condition,
                ) {
                    if !value.is_empty() {
                        let found = breakpoint_set
                            .breakpoints
                            .iter_mut()
                            .find(|other| breakpoint.position == *other.position());
                        if let Some(found) = found {
                            let found = &mut found.bp.bp;
                            if let Some((property, value)) = action.property(
                                &mut found.message,
                                &mut found.condition,
                                &mut found.hit_condition,
                            ) {
                                *property = Some(value.clone());
                            }
                        } else {
                            *property = Some(value.clone());
                            breakpoint_set
                                .breakpoints
                                .push(StatefulBreakpoint::new(breakpoint.clone()));
                        }
                    } else if property.is_some() {
                        if let Some(position) = breakpoint_set
                            .breakpoints
                            .iter()
                            .position(|other| breakpoint == other.bp)
                        {
                            breakpoint_set.breakpoints.remove(position);
                        } else {
                            log::error!("Failed to find position of breakpoint to delete")
                        }
                    }
                }
            }
        }

        if breakpoint_set.is_empty() {
            self.breakpoints.remove(&abs_path);
        }
        if let BreakpointStoreMode::Remote(remote) = &self.mode {
            if let Some(breakpoint) =
                breakpoint
                    .bp
                    .to_proto(&abs_path, &breakpoint.position, &HashMap::default())
            {
                cx.background_spawn(remote.upstream_client.request(proto::ToggleBreakpoint {
                    project_id: remote.upstream_project_id,
                    path: abs_path.to_string_lossy().into_owned(),
                    breakpoint: Some(breakpoint),
                }))
                .detach();
            }
        } else if let Some((client, project_id)) = &self.downstream_client {
            let breakpoints = self
                .breakpoints
                .get(&abs_path)
                .map(|breakpoint_set| {
                    breakpoint_set
                        .breakpoints
                        .iter()
                        .filter_map(|bp| {
                            bp.bp
                                .bp
                                .to_proto(&abs_path, bp.position(), &bp.session_state)
                        })
                        .collect()
                })
                .unwrap_or_default();

            let _ = client.send(proto::BreakpointsForFile {
                project_id: *project_id,
                path: abs_path.to_string_lossy().into_owned(),
                breakpoints,
            });
        }

        cx.emit(BreakpointStoreEvent::BreakpointsUpdated(
            abs_path,
            BreakpointUpdatedReason::Toggled,
        ));
        cx.notify();
    }

    pub fn on_file_rename(
        &mut self,
        old_path: Arc<Path>,
        new_path: Arc<Path>,
        cx: &mut Context<Self>,
    ) {
        if let Some(mut breakpoints) = self.breakpoints.remove(&old_path) {
            for breakpoint in &mut breakpoints.pending {
                breakpoint.path = new_path.clone();
            }
            self.breakpoints.insert(new_path, breakpoints);
            cx.notify();
        }
    }

    pub fn clear_breakpoints(&mut self, cx: &mut Context<Self>) {
        let breakpoint_paths = self.breakpoints.keys().cloned().collect();
        self.breakpoints.clear();
        cx.emit(BreakpointStoreEvent::BreakpointsCleared(breakpoint_paths));
        cx.notify();
    }

    pub fn breakpoints<'a>(
        &'a self,
        buffer: &'a Entity<Buffer>,
        range: Option<Range<text::Anchor>>,
        buffer_snapshot: &'a BufferSnapshot,
        cx: &App,
    ) -> impl Iterator<Item = (&'a BreakpointWithPosition, Option<BreakpointSessionState>)> + 'a
    {
        let abs_path = Self::abs_path_from_buffer(buffer, cx);
        let active_session_id = self
            .active_stack_frame
            .as_ref()
            .map(|frame| frame.session_id);
        abs_path
            .and_then(|path| self.breakpoints.get(&path))
            .into_iter()
            .flat_map(move |file_breakpoints| {
                file_breakpoints.breakpoints.iter().filter_map({
                    let range = range.clone();
                    move |bp| {
                        if !buffer_snapshot.can_resolve(bp.position()) {
                            return None;
                        }

                        if let Some(range) = &range
                            && (bp.position().cmp(&range.start, buffer_snapshot).is_lt()
                                || bp.position().cmp(&range.end, buffer_snapshot).is_gt())
                        {
                            return None;
                        }
                        let session_state = active_session_id
                            .and_then(|id| bp.session_state.get(&id))
                            .copied();
                        Some((&bp.bp, session_state))
                    }
                })
            })
    }

    pub fn active_position(&self) -> Option<&ActiveStackFrame> {
        self.active_stack_frame.as_ref()
    }

    pub fn active_debug_line_pane_id(&self) -> Option<EntityId> {
        self.active_debug_line_pane_id
    }

    pub fn set_active_debug_pane_id(&mut self, pane_id: EntityId) {
        self.active_debug_line_pane_id = Some(pane_id);
    }

    pub fn remove_active_position(
        &mut self,
        session_id: Option<SessionId>,
        cx: &mut Context<Self>,
    ) {
        if let Some(session_id) = session_id {
            if self
                .active_stack_frame
                .take_if(|active_stack_frame| active_stack_frame.session_id == session_id)
                .is_some()
            {
                self.active_debug_line_pane_id = None;
            }
        } else {
            self.active_stack_frame.take();
            self.active_debug_line_pane_id = None;
        }

        cx.emit(BreakpointStoreEvent::ClearDebugLines);
        cx.notify();
    }

    pub fn set_active_position(&mut self, position: ActiveStackFrame, cx: &mut Context<Self>) {
        if self
            .active_stack_frame
            .as_ref()
            .is_some_and(|active_position| active_position == &position)
        {
            cx.emit(BreakpointStoreEvent::SetDebugLine);
            return;
        }

        if self.active_stack_frame.is_some() {
            cx.emit(BreakpointStoreEvent::ClearDebugLines);
        }

        self.active_stack_frame = Some(position);

        cx.emit(BreakpointStoreEvent::SetDebugLine);
        cx.notify();
    }

    pub fn breakpoint_at_row(
        &self,
        path: &Path,
        row: u32,
        cx: &App,
    ) -> Option<(Entity<Buffer>, BreakpointWithPosition)> {
        self.breakpoints.get(path).and_then(|breakpoints| {
            let buffer = breakpoints.buffer.as_ref()?;
            let snapshot = buffer.read(cx).text_snapshot();

            breakpoints
                .breakpoints
                .iter()
                .find(|bp| bp.position().summary::<Point>(&snapshot).row == row)
                .map(|breakpoint| (buffer.clone(), breakpoint.bp.clone()))
        })
    }

    pub fn breakpoints_from_path(&self, path: &Arc<Path>) -> Vec<BreakpointWithPosition> {
        self.breakpoints
            .get(path)
            .map(|bp| bp.breakpoints.iter().map(|bp| bp.bp.clone()).collect())
            .unwrap_or_default()
    }

    pub fn source_breakpoints_from_path(
        &self,
        path: &Arc<Path>,
        cx: &App,
    ) -> Vec<SourceBreakpoint> {
        let Some(file) = self.breakpoints.get(path) else {
            return Vec::new();
        };
        file.buffer
            .iter()
            .flat_map(|buffer| {
                let snapshot = buffer.read(cx).snapshot();
                file.breakpoints
                    .iter()
                    .map(move |breakpoint| SourceBreakpoint {
                        row: snapshot
                            .summary_for_anchor::<PointUtf16>(breakpoint.position())
                            .row,
                        path: path.clone(),
                        state: breakpoint.bp.bp.state,
                        message: breakpoint.bp.bp.message.clone(),
                        condition: breakpoint.bp.bp.condition.clone(),
                        hit_condition: breakpoint.bp.bp.hit_condition.clone(),
                    })
            })
            .chain(file.pending.iter().cloned())
            .collect()
    }

    pub fn all_breakpoints(&self) -> BTreeMap<Arc<Path>, Vec<BreakpointWithPosition>> {
        self.breakpoints
            .keys()
            .map(|path| (path.clone(), self.breakpoints_from_path(path)))
            .collect()
    }
    pub fn all_source_breakpoints(&self, cx: &App) -> BTreeMap<Arc<Path>, Vec<SourceBreakpoint>> {
        self.breakpoints
            .keys()
            .map(|path| (path.clone(), self.source_breakpoints_from_path(path, cx)))
            .collect()
    }

    pub fn with_serialized_breakpoints(
        &mut self,
        breakpoints: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>,
        cx: &mut Context<BreakpointStore>,
    ) -> Task<Result<()>> {
        if let BreakpointStoreMode::Local = &self.mode {
            if self.buffer_subscription.is_none() {
                self.buffer_subscription =
                    Some(cx.subscribe(&self.buffer_store, |store, _, event, cx| {
                        if let BufferStoreEvent::BufferAdded(buffer) = event {
                            store.on_buffer_added(buffer.clone(), cx);
                        }
                    }));
            }
            for (path, pending) in breakpoints {
                if !pending.is_empty()
                    && let btree_map::Entry::Vacant(entry) = self.breakpoints.entry(path)
                {
                    entry.insert(BreakpointsInFile::default()).pending = pending;
                }
            }
            cx.notify();
            let tasks = self
                .breakpoints
                .iter()
                .filter(|(_, file)| file.buffer.is_none())
                .map(|(path, _)| {
                    let path = path.clone();
                    let worktree_store = self.worktree_store.downgrade();
                    let buffer_store = self.buffer_store.downgrade();
                    cx.spawn(async move |store, cx| {
                        let (worktree, relative_path) = worktree_store
                            .update(cx, |store, cx| {
                                store.find_or_create_worktree(&path, false, cx)
                            })?
                            .await?;
                        worktree
                            .update(cx, |worktree, _| worktree.wait_for_snapshot(1))
                            .await?;
                        let buffer = buffer_store
                            .update(cx, |store, cx| {
                                let path = ProjectPath {
                                    worktree_id: worktree.read(cx).id(),
                                    path: relative_path,
                                };
                                store.open_buffer(path, cx)
                            })?
                            .await;
                        let buffer = match buffer {
                            Ok(buffer) => buffer,
                            Err(error) => {
                                log::error!(
                                    "Failed to restore breakpoints for {path:?}: {error:#}"
                                );
                                return Ok(());
                            }
                        };
                        store.update(cx, |store, cx| {
                            if store
                                .breakpoints
                                .get(&path)
                                .is_some_and(|file| file.buffer.is_none())
                                && Self::abs_path_from_buffer(&buffer, cx).as_ref() == Some(&path)
                            {
                                store.on_buffer_added(buffer, cx);
                            }
                        })?;
                        Ok::<_, anyhow::Error>(())
                    })
                })
                .collect::<Vec<_>>();
            cx.background_spawn(async move {
                for result in join_all(tasks).await {
                    result?;
                }
                Ok(())
            })
        } else {
            Task::ready(Ok(()))
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn breakpoint_paths(&self) -> Vec<Arc<Path>> {
        self.breakpoints.keys().cloned().collect()
    }

    fn on_buffer_added(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let Some(path) = Self::abs_path_from_buffer(&buffer, cx) else {
            return;
        };
        let Some(file) = self.breakpoints.get_mut(&path) else {
            return;
        };
        if !file.set_buffer(buffer.clone(), cx) {
            return;
        }
        let buffer = buffer.read(cx);
        if (buffer.has_unsaved_edits() || buffer.saved_mtime().is_some())
            && file.resolve_pending(cx)
        {
            self.broadcast();
            cx.emit(BreakpointStoreEvent::BreakpointsUpdated(
                path,
                BreakpointUpdatedReason::Toggled,
            ));
            cx.notify();
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            BufferEvent::Edited { .. } | BufferEvent::Reloaded | BufferEvent::Saved => {
                let Some(path) = Self::abs_path_from_buffer(&buffer, cx) else {
                    return;
                };
                let Some(file) = self.breakpoints.get_mut(&path) else {
                    return;
                };
                if file.buffer.as_ref() != Some(&buffer) {
                    return;
                }
                let resolved = file.resolve_pending(cx);
                if resolved {
                    self.broadcast();
                    cx.notify();
                }
                if resolved || event == &BufferEvent::Saved {
                    cx.emit(BreakpointStoreEvent::BreakpointsUpdated(
                        path,
                        if event == &BufferEvent::Saved {
                            BreakpointUpdatedReason::FileSaved
                        } else {
                            BreakpointUpdatedReason::Toggled
                        },
                    ));
                }
            }
            BufferEvent::FileHandleChanged => {
                if buffer
                    .read(cx)
                    .file()
                    .is_none_or(|file| file.disk_state().is_deleted())
                {
                    self.breakpoints.retain(|_, file| {
                        if file.buffer.as_ref() == Some(&buffer) {
                            file.breakpoints.clear();
                        }
                        !file.is_empty()
                    });
                    cx.notify();
                    return;
                }
                let Some(path) = Self::abs_path_from_buffer(&buffer, cx) else {
                    return;
                };
                if self
                    .breakpoints
                    .get(&path)
                    .is_some_and(|file| !file.breakpoints.is_empty())
                {
                    return;
                }
                if let Some(old_path) = self.breakpoints.iter().find_map(|(old_path, file)| {
                    (file.buffer.as_ref() == Some(&buffer) && old_path != &path)
                        .then(|| old_path.clone())
                }) {
                    self.on_file_rename(old_path, path, cx);
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
pub enum BreakpointUpdatedReason {
    Toggled,
    FileSaved,
}

pub enum BreakpointStoreEvent {
    SetDebugLine,
    ClearDebugLines,
    BreakpointsUpdated(Arc<Path>, BreakpointUpdatedReason),
    BreakpointsCleared(Vec<Arc<Path>>),
}

impl EventEmitter<BreakpointStoreEvent> for BreakpointStore {}

type BreakpointMessage = Arc<str>;

#[derive(Clone, Debug)]
pub enum BreakpointEditAction {
    Toggle,
    InvertState,
    EditLogMessage(BreakpointMessage),
    EditCondition(BreakpointMessage),
    EditHitCondition(BreakpointMessage),
}

impl BreakpointEditAction {
    fn property<'a>(
        &self,
        message: &'a mut Option<BreakpointMessage>,
        condition: &'a mut Option<BreakpointMessage>,
        hit_condition: &'a mut Option<BreakpointMessage>,
    ) -> Option<(&'a mut Option<BreakpointMessage>, &BreakpointMessage)> {
        match self {
            Self::EditLogMessage(value) => Some((message, value)),
            Self::EditCondition(value) => Some((condition, value)),
            Self::EditHitCondition(value) => Some((hit_condition, value)),
            Self::Toggle | Self::InvertState => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum BreakpointState {
    Enabled,
    Disabled,
}

impl BreakpointState {
    #[inline]
    pub fn is_enabled(&self) -> bool {
        matches!(self, BreakpointState::Enabled)
    }

    #[inline]
    pub fn is_disabled(&self) -> bool {
        matches!(self, BreakpointState::Disabled)
    }

    #[inline]
    pub fn to_int(self) -> i32 {
        match self {
            BreakpointState::Enabled => 0,
            BreakpointState::Disabled => 1,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Breakpoint {
    pub message: Option<BreakpointMessage>,
    /// How many times do we hit the breakpoint until we actually stop at it e.g. (2 = 2 times of the breakpoint action)
    pub hit_condition: Option<Arc<str>>,
    pub condition: Option<BreakpointMessage>,
    pub state: BreakpointState,
}

impl Breakpoint {
    pub fn new_standard() -> Self {
        Self {
            state: BreakpointState::Enabled,
            hit_condition: None,
            condition: None,
            message: None,
        }
    }

    pub fn new_condition(hit_condition: &str) -> Self {
        Self {
            state: BreakpointState::Enabled,
            condition: None,
            hit_condition: Some(hit_condition.into()),
            message: None,
        }
    }

    pub fn new_log(log_message: &str) -> Self {
        Self {
            state: BreakpointState::Enabled,
            hit_condition: None,
            condition: None,
            message: Some(log_message.into()),
        }
    }

    fn to_proto(
        &self,
        _path: &Path,
        position: &text::Anchor,
        session_states: &HashMap<SessionId, BreakpointSessionState>,
    ) -> Option<client::proto::Breakpoint> {
        Some(client::proto::Breakpoint {
            position: Some(serialize_text_anchor(position)),
            state: match self.state {
                BreakpointState::Enabled => proto::BreakpointState::Enabled.into(),
                BreakpointState::Disabled => proto::BreakpointState::Disabled.into(),
            },
            message: self.message.as_ref().map(|s| String::from(s.as_ref())),
            condition: self.condition.as_ref().map(|s| String::from(s.as_ref())),
            hit_condition: self
                .hit_condition
                .as_ref()
                .map(|s| String::from(s.as_ref())),
            session_state: session_states
                .iter()
                .map(|(session_id, state)| {
                    (
                        session_id.to_proto(),
                        proto::BreakpointSessionState {
                            id: state.id,
                            verified: state.verified,
                        },
                    )
                })
                .collect(),
        })
    }

    fn from_proto(breakpoint: client::proto::Breakpoint) -> Option<Self> {
        Some(Self {
            state: match proto::BreakpointState::try_from(breakpoint.state).ok() {
                Some(proto::BreakpointState::Disabled) => BreakpointState::Disabled,
                None | Some(proto::BreakpointState::Enabled) => BreakpointState::Enabled,
            },
            message: breakpoint.message.map(Into::into),
            condition: breakpoint.condition.map(Into::into),
            hit_condition: breakpoint.hit_condition.map(Into::into),
        })
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.state.is_enabled()
    }

    #[inline]
    pub fn is_disabled(&self) -> bool {
        self.state.is_disabled()
    }
}

/// Breakpoint for location within source code.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SourceBreakpoint {
    pub row: u32,
    pub path: Arc<Path>,
    pub message: Option<Arc<str>>,
    pub condition: Option<Arc<str>>,
    pub hit_condition: Option<Arc<str>>,
    pub state: BreakpointState,
}

impl From<SourceBreakpoint> for dap::SourceBreakpoint {
    fn from(bp: SourceBreakpoint) -> Self {
        Self {
            line: bp.row as u64 + 1,
            column: None,
            condition: bp
                .condition
                .map(|condition| String::from(condition.as_ref())),
            hit_condition: bp
                .hit_condition
                .map(|hit_condition| String::from(hit_condition.as_ref())),
            log_message: bp.message.map(|message| String::from(message.as_ref())),
            mode: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Project;
    use fs::FakeFs;
    use futures::{
        FutureExt as _, StreamExt as _,
        channel::{mpsc, oneshot},
    };
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::SettingsStore;
    use std::iter;
    use util::path;

    #[gpui::test]
    async fn test_restore_preserves_pending_breakpoints_and_user_edits(cx: &mut TestAppContext) {
        for (clear, remove_pending) in [(false, false), (false, true), (true, false)] {
            let cx = &mut cx.new_app();
            cx.update(|cx| {
                let mut settings = SettingsStore::test(cx);
                settings.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_depth = Some(1);
                });
                cx.set_global(settings);
            });
            let fs = FakeFs::new(cx.executor());
            fs.insert_tree(
                path!("/project"),
                json!({
                    "a_stalled": {"file.txt": "first\nsecond\n"},
                    "b_ready.txt": "first\nsecond\nthird\n",
                }),
            )
            .await;
            let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
            let buffer = project
                .update(cx, |project, cx| {
                    project.open_local_buffer(path!("/project/b_ready.txt"), cx)
                })
                .await
                .unwrap();
            let store = project.read_with(cx, |project, _| project.breakpoint_store());
            let stalled = Arc::<Path>::from(Path::new(path!("/project/a_stalled/file.txt")));
            let ready = Arc::<Path>::from(Path::new(path!("/project/b_ready.txt")));
            let missing = Arc::<Path>::from(Path::new(path!("/project/c_missing.txt")));
            let mut saved = [
                (stalled.clone(), [0, 1]),
                (ready.clone(), [0, 1]),
                (missing.clone(), [9, 10]),
            ]
            .into_iter()
            .map(|(path, rows)| {
                let breakpoints = rows
                    .into_iter()
                    .map(|row| source_breakpoint(path.clone(), row))
                    .collect::<Vec<_>>();
                (path, breakpoints)
            })
            .collect::<BTreeMap<_, _>>();
            if remove_pending {
                let mut duplicate = source_breakpoint(stalled.clone(), 0);
                duplicate.condition = Some(Arc::from("pending condition"));
                saved.insert(
                    stalled.clone(),
                    vec![
                        duplicate.clone(),
                        duplicate,
                        source_breakpoint(stalled.clone(), 1),
                    ],
                );
            }
            let (started, entered) = oneshot::channel();
            let (resume, resumed) = oneshot::channel();
            fs.before_read_dir(
                path!("/project/a_stalled"),
                async move {
                    started.send(()).unwrap();
                    resumed.await.unwrap();
                }
                .boxed(),
            );
            let restore = store.update(cx, |store, cx| {
                let restore = store.with_serialized_breakpoints(saved.clone(), cx);
                assert_eq!(store.all_source_breakpoints(cx), saved);
                let snapshot = buffer.read(cx).snapshot();
                store.toggle_breakpoint(
                    buffer.clone(),
                    BreakpointWithPosition {
                        position: snapshot.anchor_after(PointUtf16::new(0, 0)),
                        bp: Breakpoint::new_standard(),
                    },
                    BreakpointEditAction::EditCondition(Arc::from("live condition")),
                    cx,
                );
                restore
            });
            entered.await.unwrap();
            cx.run_until_parked();
            let mut expected = saved.clone();
            expected
                .get_mut(&ready)
                .unwrap()
                .first_mut()
                .unwrap()
                .condition = Some(Arc::from("live condition"));
            store.read_with(cx, |store, cx| {
                assert_eq!(store.all_source_breakpoints(cx), expected);
                assert_eq!(
                    store
                        .breakpoints
                        .iter()
                        .filter(|(_, file)| !file.pending.is_empty())
                        .map(|(path, _)| path.clone())
                        .collect::<Vec<_>>(),
                    vec![stalled.clone(), missing.clone()]
                );
                assert_eq!(
                    store.all_breakpoints().into_keys().collect::<Vec<_>>(),
                    saved.keys().cloned().collect::<Vec<_>>()
                );
            });
            let (updates, mut updated_paths) = mpsc::unbounded();
            let _subscription = cx.update(|cx| {
                cx.subscribe(&store, move |_, event, _| {
                    if let BreakpointStoreEvent::BreakpointsUpdated(
                        path,
                        BreakpointUpdatedReason::Toggled,
                    ) = event
                    {
                        updates.unbounded_send(path.clone()).unwrap();
                    }
                })
            });
            let mut notifications = cx.notifications(&store);
            let mut expected_updates = Vec::new();
            for (path, row, remove_action) in [
                (
                    stalled.clone(),
                    1,
                    BreakpointEditAction::EditLogMessage(Arc::from("")),
                ),
                (
                    ready.clone(),
                    1,
                    BreakpointEditAction::EditCondition(Arc::from("")),
                ),
                (
                    missing.clone(),
                    10,
                    BreakpointEditAction::EditHitCondition(Arc::from("")),
                ),
            ] {
                for action in [
                    BreakpointEditAction::EditLogMessage(Arc::from("")),
                    BreakpointEditAction::EditCondition(Arc::from("")),
                    BreakpointEditAction::EditHitCondition(Arc::from("")),
                ] {
                    store.update(cx, |store, cx| {
                        store.edit_source_breakpoint(path.clone(), row, action, cx);
                        assert_eq!(store.all_source_breakpoints(cx), expected);
                    });
                }
                for action in [
                    BreakpointEditAction::InvertState,
                    BreakpointEditAction::EditLogMessage(Arc::from("log")),
                    BreakpointEditAction::EditCondition(Arc::from("condition")),
                    BreakpointEditAction::EditHitCondition(Arc::from("3")),
                ] {
                    store.update(cx, |store, cx| {
                        store.edit_source_breakpoint(path.clone(), row, action, cx);
                    });
                }
                let breakpoint = expected.get_mut(&path).unwrap().last_mut().unwrap();
                *breakpoint = SourceBreakpoint {
                    row,
                    path: path.clone(),
                    state: BreakpointState::Disabled,
                    message: Some(Arc::from("log")),
                    condition: Some(Arc::from("condition")),
                    hit_condition: Some(Arc::from("3")),
                };
                store.read_with(cx, |store, cx| {
                    assert_eq!(store.all_source_breakpoints(cx), expected);
                });
                expected.get_mut(&path).unwrap().last_mut().unwrap().state =
                    BreakpointState::Enabled;
                store.update(cx, |store, cx| {
                    store.edit_source_breakpoint(
                        path.clone(),
                        row,
                        BreakpointEditAction::InvertState,
                        cx,
                    );
                    assert_eq!(store.all_source_breakpoints(cx), expected);
                });
                expected.get_mut(&path).unwrap().pop().unwrap();
                expected.retain(|_, breakpoints| !breakpoints.is_empty());
                store.update(cx, |store, cx| {
                    store.edit_source_breakpoint(path.clone(), row, remove_action, cx);
                    assert_eq!(store.all_source_breakpoints(cx), expected);
                });
                expected_updates.extend(iter::repeat_n(path, 9));
            }
            expected
                .get_mut(&stalled)
                .unwrap()
                .first_mut()
                .unwrap()
                .condition = Some(Arc::from("pending condition"));
            store.update(cx, |store, cx| {
                store.edit_source_breakpoint(
                    stalled.clone(),
                    0,
                    BreakpointEditAction::EditCondition(Arc::from("pending condition")),
                    cx,
                );
                assert_eq!(store.all_source_breakpoints(cx), expected);
            });
            expected_updates.push(stalled.clone());
            if remove_pending {
                expected.remove(&stalled);
                store.update(cx, |store, cx| {
                    store.edit_source_breakpoint(
                        stalled.clone(),
                        0,
                        BreakpointEditAction::Toggle,
                        cx,
                    );
                    assert_eq!(store.all_source_breakpoints(cx), expected);
                });
                expected_updates.push(stalled.clone());
            }
            assert_eq!(
                iter::from_fn(|| updated_paths.next().now_or_never().flatten()).collect::<Vec<_>>(),
                expected_updates,
            );
            assert_eq!(
                iter::from_fn(|| notifications.next().now_or_never().flatten()).count(),
                expected_updates.len(),
            );
            if clear {
                store.update(cx, |store, cx| {
                    store.clear_breakpoints(cx);
                    assert_eq!(store.all_source_breakpoints(cx), BTreeMap::new());
                    let snapshot = buffer.read(cx).snapshot();
                    store.toggle_breakpoint(
                        buffer.clone(),
                        BreakpointWithPosition {
                            position: snapshot.anchor_after(PointUtf16::new(2, 0)),
                            bp: Breakpoint::new_standard(),
                        },
                        BreakpointEditAction::Toggle,
                        cx,
                    );
                });
                expected =
                    BTreeMap::from([(ready.clone(), vec![source_breakpoint(ready.clone(), 2)])]);
            }
            resume.send(()).unwrap();
            restore.await.unwrap();
            store.read_with(cx, |store, cx| {
                assert_eq!(store.all_source_breakpoints(cx), expected);
                assert_eq!(
                    store
                        .breakpoints
                        .iter()
                        .filter(|(_, file)| !file.pending.is_empty())
                        .map(|(path, _)| path.clone())
                        .collect::<Vec<_>>(),
                    if clear { Vec::new() } else { vec![missing] }
                );
            });
        }
    }

    #[gpui::test]
    async fn test_restore_breakpoints_for_named_unsaved_buffers(cx: &mut TestAppContext) {
        for restore_contents_first in [false, true] {
            let cx = &mut cx.new_app();
            cx.update(|cx| {
                let settings = SettingsStore::test(cx);
                cx.set_global(settings);
            });
            let fs = FakeFs::new(cx.executor());
            fs.insert_tree(path!("/project"), json!({})).await;
            let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
            let path = Arc::<Path>::from(Path::new(path!("/project/unsaved.txt")));
            let missing = Arc::<Path>::from(Path::new(path!("/project/missing.txt")));
            let buffer = if restore_contents_first {
                let buffer = project
                    .update(cx, |project, cx| project.open_local_buffer(&path, cx))
                    .await
                    .unwrap();
                buffer.update(cx, |buffer, cx| buffer.set_text("first\nsecond\n", cx));
                Some(buffer)
            } else {
                None
            };
            let store = project.read_with(cx, |project, _| project.breakpoint_store());
            let mut expected = BTreeMap::from([
                (
                    path.clone(),
                    vec![
                        source_breakpoint(path.clone(), 0),
                        source_breakpoint(path.clone(), 1),
                    ],
                ),
                (
                    missing.clone(),
                    vec![
                        source_breakpoint(missing.clone(), 0),
                        source_breakpoint(missing.clone(), 9),
                    ],
                ),
            ]);
            store
                .update(cx, |store, cx| {
                    store.with_serialized_breakpoints(expected.clone(), cx)
                })
                .await
                .unwrap();
            let buffer = match buffer {
                Some(buffer) => buffer,
                None => project
                    .update(cx, |project, cx| project.open_local_buffer(&path, cx))
                    .await
                    .unwrap(),
            };
            expected
                .get_mut(&path)
                .unwrap()
                .last_mut()
                .unwrap()
                .condition = Some(Arc::from("pending condition"));
            store.update(cx, |store, cx| {
                assert_eq!(
                    store.breakpoints_from_path(&path).len(),
                    if restore_contents_first { 2 } else { 0 }
                );
                assert_eq!(store.breakpoints_from_path(&missing), Vec::new());
                store.edit_source_breakpoint(
                    path.clone(),
                    1,
                    BreakpointEditAction::EditCondition(Arc::from("pending condition")),
                    cx,
                );
                assert_eq!(store.all_source_breakpoints(cx), expected);
            });
            if !restore_contents_first {
                buffer.update(cx, |buffer, cx| buffer.set_text("first\nsecond\n", cx));
            }
            cx.run_until_parked();
            store.read_with(cx, |store, cx| {
                assert!(!buffer.read(cx).file().unwrap().disk_state().exists());
                assert_eq!(store.breakpoints_from_path(&path).len(), 2);
                assert_eq!(store.all_source_breakpoints(cx), expected);
            });
            buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "\n")], None, cx));
            for breakpoint in expected.get_mut(&path).unwrap() {
                breakpoint.row += 1;
            }
            project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
                .await
                .unwrap();
            store.read_with(cx, |store, cx| {
                assert_eq!(store.all_source_breakpoints(cx), expected)
            });
            fs.insert_tree(
                path!("/project"),
                json!({"unsaved.txt": "\n\nfirst\nsecond\n"}),
            )
            .await;
            cx.run_until_parked();
            for breakpoint in expected.get_mut(&path).unwrap() {
                breakpoint.row += 1;
            }
            let saved = store.read_with(cx, |store, cx| {
                assert_eq!(buffer.read(cx).text(), "\n\nfirst\nsecond\n");
                let saved = store.all_source_breakpoints(cx);
                assert_eq!(saved, expected);
                saved
            });
            let released_store = store.downgrade();
            cx.update(|_| drop((buffer, store, project)));
            cx.run_until_parked();
            assert!(released_store.upgrade().is_none());

            let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
            let store = project.read_with(cx, |project, _| project.breakpoint_store());
            store
                .update(cx, |store, cx| store.with_serialized_breakpoints(saved, cx))
                .await
                .unwrap();
            store.read_with(cx, |store, cx| {
                assert_eq!(store.breakpoints_from_path(&missing), Vec::new());
                assert_eq!(store.all_source_breakpoints(cx), expected);
            });
            fs.insert_tree(path!("/project"), json!({"missing.txt": "first\nsecond\n"}))
                .await;
            cx.run_until_parked();
            store.read_with(cx, |store, cx| {
                assert_eq!(store.breakpoints_from_path(&missing).len(), 1);
                assert_eq!(store.all_source_breakpoints(cx), expected);
            });
            let buffer = project
                .update(cx, |project, cx| project.open_local_buffer(&missing, cx))
                .await
                .unwrap();
            buffer.update(cx, |buffer, cx| {
                let end = buffer.len();
                buffer.edit([(end..end, "\n\n\n\n\n\n\nlast")], None, cx);
            });
            cx.run_until_parked();
            store.read_with(cx, |store, cx| {
                assert_eq!(store.breakpoints_from_path(&missing).len(), 2);
                assert_eq!(store.all_source_breakpoints(cx), expected);
            });
        }
    }

    fn source_breakpoint(path: Arc<Path>, row: u32) -> SourceBreakpoint {
        SourceBreakpoint {
            row,
            path,
            message: None,
            condition: None,
            hit_condition: None,
            state: BreakpointState::Enabled,
        }
    }
}

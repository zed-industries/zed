//! Module for managing breakpoints in a project.
//!
//! Breakpoints are separate from a session because they're not associated with any particular debug session. They can also be set up without a session running.
use anyhow::Result;
use collections::BTreeMap;
use gpui::{App, Context, Entity, EventEmitter, Task};
use language::{proto::serialize_anchor as serialize_text_anchor, Buffer, BufferSnapshot};
use rpc::{proto, AnyProtoClient};
use std::{
    hash::{Hash, Hasher},
    ops::Range,
    path::Path,
    sync::Arc,
};
use text::Point;

use crate::{buffer_store::BufferStore, worktree_store::WorktreeStore, ProjectPath};

#[derive(Clone)]
struct RemoteBreakpointStore {
    _upstream_client: Option<AnyProtoClient>,
    _upstream_project_id: u64,
}

#[derive(Clone)]
struct BreakpointsInFile {
    buffer: Entity<Buffer>,
    // TODO: This is.. less than ideal, as it's O(n) and does not return entries in order. We'll have to change TreeMap to support passing in the context for comparisons
    breakpoints: Vec<(text::Anchor, Breakpoint)>,
}

#[derive(Clone)]
struct LocalBreakpointStore {
    worktree_store: Entity<WorktreeStore>,
    buffer_store: Entity<BufferStore>,
}

#[derive(Clone)]
enum BreakpointStoreMode {
    Local(LocalBreakpointStore),
    #[expect(dead_code)]
    Remote(RemoteBreakpointStore),
}
pub struct BreakpointStore {
    breakpoints: BTreeMap<Arc<Path>, BreakpointsInFile>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    active_stack_frame: Option<(Arc<Path>, text::Anchor)>,
    // E.g ssh
    mode: BreakpointStoreMode,
}

impl BreakpointStore {
    pub fn local(worktree_store: Entity<WorktreeStore>, buffer_store: Entity<BufferStore>) -> Self {
        BreakpointStore {
            breakpoints: BTreeMap::new(),
            mode: BreakpointStoreMode::Local(LocalBreakpointStore {
                worktree_store,
                buffer_store,
            }),
            downstream_client: None,
            active_stack_frame: Default::default(),
        }
    }

    pub(crate) fn remote(upstream_project_id: u64, upstream_client: AnyProtoClient) -> Self {
        BreakpointStore {
            breakpoints: BTreeMap::new(),
            mode: BreakpointStoreMode::Remote(RemoteBreakpointStore {
                _upstream_client: Some(upstream_client),
                _upstream_project_id: upstream_project_id,
            }),
            downstream_client: None,
            active_stack_frame: Default::default(),
        }
    }

    pub(crate) fn shared(&mut self, project_id: u64, downstream_client: AnyProtoClient) {
        self.downstream_client = Some((downstream_client.clone(), project_id));
    }

    pub(crate) fn unshared(&mut self, cx: &mut Context<Self>) {
        self.downstream_client.take();

        cx.notify();
    }

    fn abs_path_from_buffer(buffer: &Entity<Buffer>, cx: &App) -> Option<Arc<Path>> {
        worktree::File::from_dyn(buffer.read(cx).file())
            .and_then(|file| file.worktree.read(cx).absolutize(&file.path).ok())
            .map(|path_buf| Arc::<Path>::from(path_buf))
    }

    pub fn toggle_breakpoint(
        &mut self,
        buffer: Entity<Buffer>,
        mut breakpoint: (text::Anchor, Breakpoint),
        edit_action: BreakpointEditAction,
        cx: &mut Context<Self>,
    ) {
        let Some(abs_path) = Self::abs_path_from_buffer(&buffer, cx) else {
            return;
        };

        let breakpoint_set =
            self.breakpoints
                .entry(abs_path.clone())
                .or_insert_with(|| BreakpointsInFile {
                    breakpoints: Default::default(),
                    buffer,
                });

        match edit_action {
            BreakpointEditAction::Toggle => {
                let len_before = breakpoint_set.breakpoints.len();
                breakpoint_set
                    .breakpoints
                    .retain(|value| &breakpoint != value);
                if len_before == breakpoint_set.breakpoints.len() {
                    // We did not remove any breakpoint, hence let's toggle one.
                    breakpoint_set.breakpoints.push(breakpoint);
                }
            }
            BreakpointEditAction::EditLogMessage(log_message) => {
                if !log_message.is_empty() {
                    breakpoint.1.kind = BreakpointKind::Log(log_message.clone());
                    let len_before = breakpoint_set.breakpoints.len();
                    breakpoint_set
                        .breakpoints
                        .retain(|value| &breakpoint != value);
                    if len_before == breakpoint_set.breakpoints.len() {
                        // We did not remove any breakpoint, hence let's toggle one.
                        breakpoint_set.breakpoints.push(breakpoint);
                    }
                } else if matches!(&breakpoint.1.kind, BreakpointKind::Log(_)) {
                    breakpoint_set
                        .breakpoints
                        .retain(|value| &breakpoint != value);
                }
            }
        }

        if breakpoint_set.breakpoints.is_empty() {
            self.breakpoints.remove(&abs_path);
        }

        cx.emit(BreakpointStoreEvent::BreakpointsUpdated(abs_path));
        cx.notify();
    }

    pub fn on_file_rename(
        &mut self,
        old_path: Arc<Path>,
        new_path: Arc<Path>,
        cx: &mut Context<Self>,
    ) {
        if let Some(breakpoints) = self.breakpoints.remove(&old_path) {
            self.breakpoints.insert(new_path.clone(), breakpoints);

            cx.notify();
        }
    }

    pub fn breakpoints<'a>(
        &'a self,
        buffer: &'a Entity<Buffer>,
        range: Option<Range<text::Anchor>>,
        buffer_snapshot: BufferSnapshot,
        cx: &App,
    ) -> impl Iterator<Item = &'a (text::Anchor, Breakpoint)> + 'a {
        let abs_path = Self::abs_path_from_buffer(buffer, cx);
        abs_path
            .and_then(|path| self.breakpoints.get(&path))
            .into_iter()
            .flat_map(move |file_breakpoints| {
                file_breakpoints.breakpoints.iter().filter({
                    let range = range.clone();
                    let buffer_snapshot = buffer_snapshot.clone();
                    move |(position, _)| {
                        if let Some(range) = &range {
                            position.cmp(&range.start, &buffer_snapshot).is_ge()
                                && position.cmp(&range.end, &buffer_snapshot).is_le()
                        } else {
                            true
                        }
                    }
                })
            })
    }

    pub fn active_position(&self) -> Option<&(Arc<Path>, text::Anchor)> {
        self.active_stack_frame.as_ref()
    }

    pub fn set_active_position(
        &mut self,
        position: Option<(Arc<Path>, text::Anchor)>,
        cx: &mut Context<Self>,
    ) {
        self.active_stack_frame = position;
        cx.emit(BreakpointStoreEvent::ActiveDebugLineChanged);
        cx.notify();
    }

    pub fn all_breakpoints(&self, cx: &App) -> BTreeMap<Arc<Path>, Vec<SerializedBreakpoint>> {
        self.breakpoints
            .iter()
            .map(|(path, bp)| {
                let snapshot = bp.buffer.read(cx).snapshot();
                (
                    path.clone(),
                    bp.breakpoints
                        .iter()
                        .map(|(position, breakpoint)| {
                            let position = snapshot.summary_for_anchor::<Point>(position).row;
                            SerializedBreakpoint {
                                position,
                                path: path.clone(),
                                kind: breakpoint.kind.clone(),
                            }
                        })
                        .collect(),
                )
            })
            .collect()
    }

    pub fn with_serialized_breakpoints(
        &self,
        breakpoints: BTreeMap<Arc<Path>, Vec<SerializedBreakpoint>>,
        cx: &mut Context<'_, BreakpointStore>,
    ) -> Task<Result<()>> {
        if let BreakpointStoreMode::Local(mode) = &self.mode {
            let mode = mode.clone();
            cx.spawn(move |this, mut cx| async move {
                let mut new_breakpoints = BTreeMap::default();
                for (path, bps) in breakpoints {
                    if bps.is_empty() {
                        continue;
                    }
                    let (worktree, relative_path) = mode
                        .worktree_store
                        .update(&mut cx, |this, cx| {
                            this.find_or_create_worktree(&path, false, cx)
                        })?
                        .await?;
                    let buffer = mode
                        .buffer_store
                        .update(&mut cx, |this, cx| {
                            let path = ProjectPath {
                                worktree_id: worktree.read(cx).id(),
                                path: relative_path.into(),
                            };
                            this.open_buffer(path, cx)
                        })?
                        .await;
                    let Ok(buffer) = buffer else {
                        log::error!("Todo: Serialized breakpoints which do not have buffer (yet)");
                        continue;
                    };
                    let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
                    let mut breakpoints_for_file = BreakpointsInFile {
                        buffer,
                        breakpoints: vec![],
                    };
                    for bp in bps {
                        let position = snapshot.anchor_before(Point::new(bp.position, 0));
                        breakpoints_for_file
                            .breakpoints
                            .push((position, Breakpoint { kind: bp.kind }))
                    }
                    new_breakpoints.insert(path, breakpoints_for_file);
                }
                this.update(&mut cx, |this, cx| {
                    this.breakpoints = new_breakpoints;
                    cx.notify();
                })?;

                Ok(())
            })
        } else {
            Task::ready(Ok(()))
        }
    }
}

pub enum BreakpointStoreEvent {
    ActiveDebugLineChanged,
    BreakpointsUpdated(Arc<Path>),
}

impl EventEmitter<BreakpointStoreEvent> for BreakpointStore {}

type LogMessage = Arc<str>;

#[derive(Clone, Debug)]
pub enum BreakpointEditAction {
    Toggle,
    EditLogMessage(LogMessage),
}

#[derive(Clone, Debug)]
pub enum BreakpointKind {
    Standard,
    Log(LogMessage),
}

impl BreakpointKind {
    pub fn to_int(&self) -> i32 {
        match self {
            BreakpointKind::Standard => 0,
            BreakpointKind::Log(_) => 1,
        }
    }

    pub fn log_message(&self) -> Option<LogMessage> {
        match self {
            BreakpointKind::Standard => None,
            BreakpointKind::Log(message) => Some(message.clone()),
        }
    }
}

impl PartialEq for BreakpointKind {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for BreakpointKind {}

impl Hash for BreakpointKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Breakpoint {
    pub kind: BreakpointKind,
}

impl Breakpoint {
    fn _to_proto(
        &self,
        _path: &Path,
        position: &text::Anchor,
    ) -> Option<client::proto::Breakpoint> {
        Some(client::proto::Breakpoint {
            position: Some(serialize_text_anchor(position)),

            kind: match self.kind {
                BreakpointKind::Standard => proto::BreakpointKind::Standard.into(),
                BreakpointKind::Log(_) => proto::BreakpointKind::Log.into(),
            },
            message: if let BreakpointKind::Log(message) = &self.kind {
                Some(message.to_string())
            } else {
                None
            },
        })
    }

    fn _from_proto(_breakpoint: client::proto::Breakpoint) -> Option<Self> {
        None
        // Some(Self {
        //     position: deserialize_anchor(breakpoint.position?)?,
        //     kind: match proto::BreakpointKind::from_i32(breakpoint.kind) {
        //         Some(proto::BreakpointKind::Log) => {
        //             BreakpointKind::Log(breakpoint.message.clone().unwrap_or_default().into())
        //         }
        //         None | Some(proto::BreakpointKind::Standard) => BreakpointKind::Standard,
        //     },
        // })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SerializedBreakpoint {
    pub position: u32,
    pub path: Arc<Path>,
    pub kind: BreakpointKind,
}

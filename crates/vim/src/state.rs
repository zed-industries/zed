use crate::command::command_interceptor;
use crate::motion::MotionKind;
use crate::normal::repeat::Replayer;
use crate::surrounds::SurroundsType;
use crate::{ToggleMarksView, ToggleRegistersView, UseSystemClipboard, Vim, VimAddon, VimSettings};
use crate::{motion::Motion, object::Object};
use anyhow::Result;
use collections::HashMap;
use command_palette_hooks::{CommandPaletteFilter, GlobalCommandPaletteInterceptor};
use db::{
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use editor::display_map::{is_invisible, replacement};
use editor::{
    Anchor, ClipboardAccess, ClipboardSelection, Editor, MultiBuffer, NativeInput,
    ToPoint as EditorToPoint,
};
use futures::{
    FutureExt,
    channel::oneshot,
    future::{LocalBoxFuture, Shared},
};
use gpui::{
    Action, App, AppContext, BorrowAppContext, ClipboardEntry, ClipboardItem, DismissEvent, Entity,
    EntityId, FocusHandle, Global, HighlightStyle, StyledText, Subscription, Task, TaskExt,
    TextStyle, WeakEntity,
};
use language::{Buffer, BufferEvent, BufferId, Chunk, LanguageAwareStyling, Point};

use multi_buffer::MultiBufferRow;
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectItem, ProjectPath};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut, Range},
    sync::Arc,
};
use text::{Bias, ToPoint};
use theme_settings::ThemeSettings;
use ui::{
    ActiveTheme, Context, Div, FluentBuilder, KeyBinding, ParentElement, SharedString, Styled,
    StyledTypography, Window, h_flex, rems,
};
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::searchable::Direction;
use workspace::{MultiWorkspace, Workspace, WorkspaceDb, WorkspaceId};

#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
    HelixNormal,
    HelixSelect,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::Replace => write!(f, "REPLACE"),
            Mode::Visual => write!(f, "VISUAL"),
            Mode::VisualLine => write!(f, "VISUAL LINE"),
            Mode::VisualBlock => write!(f, "VISUAL BLOCK"),
            Mode::HelixNormal => write!(f, "NORMAL"),
            Mode::HelixSelect => write!(f, "SELECT"),
        }
    }
}

impl Mode {
    pub fn is_visual(&self) -> bool {
        match self {
            Self::Visual | Self::VisualLine | Self::VisualBlock | Self::HelixSelect => true,
            Self::Normal | Self::Insert | Self::Replace | Self::HelixNormal => false,
        }
    }

    pub fn is_helix(&self) -> bool {
        matches!(self, Self::HelixNormal | Self::HelixSelect)
    }

    /// `HelixNormal` qualifies because its cursor is itself a one-character selection.
    pub fn has_selection(&self) -> bool {
        self.is_visual() || matches!(self, Self::HelixNormal)
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal | Self::HelixNormal)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Change,
    Delete,
    Yank,
    Replace,
    Object {
        around: bool,
    },
    FindForward {
        before: bool,
        multiline: bool,
    },
    FindBackward {
        after: bool,
        multiline: bool,
    },
    Sneak {
        first_char: Option<char>,
    },
    SneakBackward {
        first_char: Option<char>,
    },
    AddSurrounds {
        // Typically no need to configure this as `SendKeystrokes` can be used - see #23088.
        target: Option<SurroundsType>,
    },
    ChangeSurrounds {
        target: Option<Object>,
        /// Represents whether the opening bracket was used for the target
        /// object.
        opening: bool,
        /// Computed anchors for the opening and closing bracket characters,
        bracket_anchors: Vec<Option<(Anchor, Anchor)>>,
    },
    DeleteSurrounds,
    Mark,
    Jump {
        line: bool,
    },
    Indent,
    Outdent,
    AutoIndent,
    Rewrap,
    ShellCommand,
    Lowercase,
    Uppercase,
    OppositeCase,
    Rot13,
    Rot47,
    Digraph {
        first_char: Option<char>,
    },
    Literal {
        prefix: Option<String>,
    },
    Register {
        recording: RecordingCheckpoint,
    },
    RecordRegister,
    ReplayRegister,
    ToggleComments,
    ToggleBlockComments,
    ReplaceWithRegister,
    Exchange,
    HelixMatch,
    HelixNext {
        around: bool,
    },
    HelixPrevious {
        around: bool,
    },
    HelixSurroundAdd,
    HelixSurroundReplace {
        replaced_char: Option<char>,
    },
    HelixSurroundDelete,
    HelixJump {
        behaviour: HelixJumpBehaviour,
        first_char: Option<char>,
        labels: Vec<HelixJumpLabel>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordingCheckpoint {
    dot: Option<usize>,
    macro_action: Option<(char, usize, usize, Rc<()>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HelixJumpLabel {
    pub label: [char; 2],
    pub range: Range<Anchor>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelixJumpBehaviour {
    Move,
    MoveToWordStart,
    Extend,
    ExtendToWordStart,
}

#[derive(Default, Clone, Debug)]
pub enum RecordedSelection {
    #[default]
    None,
    Visual {
        rows: u32,
        cols: u32,
    },
    SingleLine {
        cols: u32,
    },
    VisualBlock {
        rows: u32,
        cols: u32,
    },
    VisualLine {
        rows: u32,
    },
}

#[derive(Default, Clone, Debug)]
pub struct Register {
    pub(crate) text: SharedString,
    pub(crate) clipboard_selections: Option<Vec<ClipboardSelection>>,
}

impl From<Register> for ClipboardItem {
    fn from(register: Register) -> Self {
        if let Some(clipboard_selections) = register.clipboard_selections {
            ClipboardItem::new_string_with_json_metadata(register.text.into(), clipboard_selections)
        } else {
            ClipboardItem::new_string(register.text.into())
        }
    }
}

impl From<ClipboardItem> for Register {
    fn from(item: ClipboardItem) -> Self {
        match item.entries().iter().find_map(|entry| match entry {
            ClipboardEntry::String(value) => Some(value),
            _ => None,
        }) {
            Some(value) => Register {
                text: value.text().to_owned().into(),
                clipboard_selections: value.metadata_json::<Vec<ClipboardSelection>>(),
            },
            None => Register::default(),
        }
    }
}

impl From<String> for Register {
    fn from(text: String) -> Self {
        Register {
            text: text.into(),
            clipboard_selections: None,
        }
    }
}

#[derive(Default)]
pub struct VimGlobals {
    pub last_find: Option<Motion>,

    pub dot_replaying: bool,
    pub recorded_count: Option<usize>,
    pub recorded_actions: Vec<ReplayableAction>,
    pub recorded_selection: RecordedSelection,

    /// The register being written to by the active `q{register}` macro
    /// recording.
    pub recording_register: Option<char>,
    /// The register from the last completed dot-recording. Used when replaying
    /// with `.`.
    pub recorded_register_for_dot: Option<char>,
    pub last_recorded_register: Option<char>,
    pub last_replayed_register: Option<char>,
    pub replayer: Option<Replayer>,

    pub last_yank: Option<SharedString>,
    pub registers: HashMap<char, Register>,
    pub recordings: HashMap<char, Vec<ReplayableAction>>,

    pub focused_vim: Option<WeakEntity<Vim>>,

    pub marks: HashMap<EntityId, Entity<MarksState>>,

    pub(crate) keystroke_observers: Vec<(FocusHandle, WeakEntity<Vim>)>,
    pub(crate) input: Rc<RefCell<VimInputState>>,
    pub(crate) recording_input: Weak<RefCell<VimInputState>>,
    focused_input: Rc<RefCell<VimInputState>>,
    input_depth: usize,
    command: VimCommandState,
}

#[derive(Default)]
pub struct VimCommandState {
    pub dot_recording: bool,
    /// pre_count is the number before an operator is specified (3 in 3d2d)
    pub pre_count: Option<usize>,
    /// post_count is the number after an operator is specified (2 in 3d2d)
    pub post_count: Option<usize>,
    pub forced_motion: bool,
    pub stop_recording_after_next_action: bool,
    pub ignore_current_insertion: bool,
    pub recording_count: Option<usize>,
    pub recording_actions: Vec<ReplayableAction>,
    pub recording_selection: RecordedSelection,
    /// The register that was selected at the start of the current
    /// dot-recording, for example, `"ap`.
    pub recording_register_for_dot: Option<char>,
    pub(crate) execution: Option<ReplayExecution>,
}

#[derive(Clone)]
pub(crate) struct ReplayExecution {
    pub(crate) identity: Rc<()>,
    pub(crate) count: Option<usize>,
    pub(crate) dot: bool,
    pub(crate) forced_motion: Option<bool>,
}

#[derive(Clone)]
pub(crate) struct VimCommandContext {
    input: Rc<RefCell<VimInputState>>,
    execution: Option<ReplayExecution>,
}

impl VimCommandContext {
    pub(crate) fn capture(input: Rc<RefCell<VimInputState>>, cx: &App) -> Self {
        let globals = cx.global::<VimGlobals>();
        let (execution, forced_motion) = if Rc::ptr_eq(&input, &globals.input) {
            (globals.execution.clone(), globals.forced_motion)
        } else {
            let input = input.borrow();
            (input.command.execution.clone(), input.command.forced_motion)
        };
        let execution = execution.map(|mut execution| {
            execution.forced_motion = Some(execution.forced_motion.unwrap_or(forced_motion));
            execution
        });
        Self { input, execution }
    }

    pub(crate) fn run<C: BorrowMut<App>, R>(self, cx: &mut C, run: impl FnOnce(&mut C) -> R) -> R {
        VimGlobals::with_input(self.input, cx, |cx| {
            VimGlobals::with_execution(self.execution, cx, run)
        })
    }
}

impl Deref for VimGlobals {
    type Target = VimCommandState;

    fn deref(&self) -> &Self::Target {
        &self.command
    }
}

impl DerefMut for VimGlobals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.command
    }
}

#[derive(Default)]
pub(crate) struct VimInputState {
    pub editor: Option<WeakEntity<Editor>>,
    pub command: VimCommandState,
    macro_input: Option<Rc<MacroInputScopeState>>,
}

impl VimInputState {
    pub(crate) fn new(editor: WeakEntity<Editor>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            editor: Some(editor),
            ..Self::default()
        }))
    }

    fn swap(&mut self, globals: &mut VimGlobals) {
        std::mem::swap(&mut self.command, &mut globals.command);
    }
}

pub struct MarksState {
    workspace: WeakEntity<Workspace>,

    multibuffer_marks: HashMap<EntityId, HashMap<String, Vec<Anchor>>>,
    buffer_marks: HashMap<BufferId, HashMap<String, Vec<text::Anchor>>>,
    watched_buffers: HashMap<BufferId, (MarkLocation, Subscription, Subscription)>,

    serialized_marks: HashMap<Arc<Path>, HashMap<String, Vec<Point>>>,
    global_marks: HashMap<String, MarkLocation>,

    _subscription: Subscription,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MarkLocation {
    Buffer(EntityId),
    Path(Arc<Path>),
}

pub enum Mark {
    Local(Vec<Anchor>),
    Buffer(EntityId, Vec<Anchor>),
    Path(Arc<Path>, Vec<Point>),
}

impl MarksState {
    pub fn new(workspace: &Workspace, cx: &mut App) -> Entity<MarksState> {
        cx.new(|cx| {
            let buffer_store = workspace.project().read(cx).buffer_store().clone();
            let subscription = cx.subscribe(&buffer_store, move |this: &mut Self, _, event, cx| {
                if let project::buffer_store::BufferStoreEvent::BufferAdded(buffer) = event {
                    this.on_buffer_loaded(buffer, cx);
                }
            });

            let mut this = Self {
                workspace: workspace.weak_handle(),
                multibuffer_marks: HashMap::default(),
                buffer_marks: HashMap::default(),
                watched_buffers: HashMap::default(),
                serialized_marks: HashMap::default(),
                global_marks: HashMap::default(),
                _subscription: subscription,
            };

            this.load(cx);
            this
        })
    }

    fn workspace_id(&self, cx: &App) -> Option<WorkspaceId> {
        self.workspace
            .read_with(cx, |workspace, _| workspace.database_id())
            .ok()
            .flatten()
    }

    fn project(&self, cx: &App) -> Option<Entity<Project>> {
        self.workspace
            .read_with(cx, |workspace, _| workspace.project().clone())
            .ok()
    }

    fn load(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let Some(workspace_id) = this.update(cx, |this, cx| this.workspace_id(cx)).ok()? else {
                return None;
            };
            let db = cx.update(|cx| VimDb::global(cx));
            let (marks, paths) = cx
                .background_spawn(async move {
                    let marks = db.get_marks(workspace_id)?;
                    let paths = db.get_global_marks_paths(workspace_id)?;
                    anyhow::Ok((marks, paths))
                })
                .await
                .log_err()?;
            this.update(cx, |this, cx| this.loaded(marks, paths, cx))
                .ok()
        })
        .detach();
    }

    fn loaded(
        &mut self,
        marks: Vec<SerializedMark>,
        global_mark_paths: Vec<(String, Arc<Path>)>,
        cx: &mut Context<Self>,
    ) {
        let Some(project) = self.project(cx) else {
            return;
        };

        for mark in marks {
            self.serialized_marks
                .entry(mark.path)
                .or_default()
                .insert(mark.name, mark.points);
        }

        for (name, path) in global_mark_paths {
            self.global_marks
                .insert(name, MarkLocation::Path(path.clone()));

            let project_path = project
                .read(cx)
                .worktrees(cx)
                .filter_map(|worktree| {
                    let relative = path.strip_prefix(worktree.read(cx).abs_path()).ok()?;
                    let path = RelPath::new(relative, worktree.read(cx).path_style()).log_err()?;
                    Some(ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: path.into_arc(),
                    })
                })
                .next();
            if let Some(buffer) = project_path
                .and_then(|project_path| project.read(cx).get_open_buffer(&project_path, cx))
            {
                self.on_buffer_loaded(&buffer, cx)
            }
        }
    }

    pub fn on_buffer_loaded(&mut self, buffer_handle: &Entity<Buffer>, cx: &mut Context<Self>) {
        let Some(project) = self.project(cx) else {
            return;
        };
        let Some(project_path) = buffer_handle.read(cx).project_path(cx) else {
            return;
        };
        let Some(abs_path) = project.read(cx).absolute_path(&project_path, cx) else {
            return;
        };
        let abs_path: Arc<Path> = abs_path.into();

        let Some(serialized_marks) = self.serialized_marks.get(&abs_path) else {
            return;
        };

        let mut loaded_marks = HashMap::default();
        let buffer = buffer_handle.read(cx);
        for (name, points) in serialized_marks.iter() {
            loaded_marks.insert(
                name.clone(),
                points
                    .iter()
                    .map(|point| buffer.anchor_before(buffer.clip_point(*point, Bias::Left)))
                    .collect(),
            );
        }
        self.buffer_marks.insert(buffer.remote_id(), loaded_marks);
        self.watch_buffer(MarkLocation::Path(abs_path), buffer_handle, cx)
    }

    fn serialize_buffer_marks(
        &mut self,
        path: Arc<Path>,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let new_points: HashMap<String, Vec<Point>> = if let Some(anchors) =
            self.buffer_marks.get(&buffer.read(cx).remote_id())
        {
            anchors
                .iter()
                .map(|(name, anchors)| {
                    let snapshot = buffer.read(cx);
                    (
                        name.clone(),
                        snapshot
                            .summaries_for_anchors_unordered::<Point, _>(anchors.iter().copied())
                            .collect(),
                    )
                })
                .collect()
        } else {
            HashMap::default()
        };
        let old_points = self.serialized_marks.get(&path);
        if old_points == Some(&new_points) {
            return;
        }
        let mut to_write = HashMap::default();

        for (key, value) in &new_points {
            if self.is_global_mark(key)
                && self.global_marks.get(key) != Some(&MarkLocation::Path(path.clone()))
            {
                if let Some(workspace_id) = self.workspace_id(cx) {
                    let path = path.clone();
                    let key = key.clone();
                    let db = VimDb::global(cx);
                    cx.background_spawn(async move {
                        db.set_global_mark_path(workspace_id, key, path).await
                    })
                    .detach_and_log_err(cx);
                }

                self.global_marks
                    .insert(key.clone(), MarkLocation::Path(path.clone()));
            }
            if old_points.and_then(|o| o.get(key)) != Some(value) {
                to_write.insert(key.clone(), value.clone());
            }
        }

        self.serialized_marks.insert(path.clone(), new_points);

        if let Some(workspace_id) = self.workspace_id(cx) {
            let db = VimDb::global(cx);
            cx.background_spawn(async move {
                db.set_marks(workspace_id, path.clone(), to_write).await?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn is_global_mark(&self, key: &str) -> bool {
        key.chars()
            .next()
            .is_some_and(|c| c.is_uppercase() || c.is_digit(10))
    }

    fn rename_buffer(
        &mut self,
        old_path: MarkLocation,
        new_path: Arc<Path>,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        if let MarkLocation::Buffer(entity_id) = old_path
            && let Some(old_marks) = self.multibuffer_marks.remove(&entity_id)
        {
            let buffer_marks = old_marks
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .filter_map(|anchor| anchor.raw_text_anchor())
                            .collect(),
                    )
                })
                .collect();
            self.buffer_marks
                .insert(buffer.read(cx).remote_id(), buffer_marks);
        }
        self.watch_buffer(MarkLocation::Path(new_path.clone()), buffer, cx);
        self.serialize_buffer_marks(new_path, buffer, cx);
    }

    fn path_for_buffer(&self, buffer: &Entity<Buffer>, cx: &App) -> Option<Arc<Path>> {
        let project_path = buffer.read(cx).project_path(cx)?;
        let project = self.project(cx)?;
        let abs_path = project.read(cx).absolute_path(&project_path, cx)?;
        Some(abs_path.into())
    }

    fn points_at(
        &self,
        location: &MarkLocation,
        multi_buffer: &Entity<MultiBuffer>,
        cx: &App,
    ) -> bool {
        match location {
            MarkLocation::Buffer(entity_id) => entity_id == &multi_buffer.entity_id(),
            MarkLocation::Path(path) => {
                let Some(singleton) = multi_buffer.read(cx).as_singleton() else {
                    return false;
                };
                self.path_for_buffer(&singleton, cx).as_ref() == Some(path)
            }
        }
    }

    pub fn watch_buffer(
        &mut self,
        mark_location: MarkLocation,
        buffer_handle: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let on_change = cx.subscribe(buffer_handle, move |this, buffer, event, cx| match event {
            BufferEvent::Edited { .. } => {
                if let Some(path) = this.path_for_buffer(&buffer, cx) {
                    this.serialize_buffer_marks(path, &buffer, cx);
                }
            }
            BufferEvent::FileHandleChanged => {
                let buffer_id = buffer.read(cx).remote_id();
                if let Some(old_path) = this
                    .watched_buffers
                    .get(&buffer_id.clone())
                    .map(|(path, _, _)| path.clone())
                    && let Some(new_path) = this.path_for_buffer(&buffer, cx)
                {
                    this.rename_buffer(old_path, new_path, &buffer, cx)
                }
            }
            _ => {}
        });

        let on_release = cx.observe_release(buffer_handle, |this, buffer, _| {
            this.watched_buffers.remove(&buffer.remote_id());
            this.buffer_marks.remove(&buffer.remote_id());
        });

        self.watched_buffers.insert(
            buffer_handle.read(cx).remote_id(),
            (mark_location, on_change, on_release),
        );
    }

    pub fn set_mark(
        &mut self,
        name: String,
        multibuffer: &Entity<MultiBuffer>,
        anchors: Vec<Anchor>,
        cx: &mut Context<Self>,
    ) {
        let multibuffer_snapshot = multibuffer.read(cx).snapshot(cx);
        let buffer = multibuffer.read(cx).as_singleton();
        let abs_path = buffer.as_ref().and_then(|b| self.path_for_buffer(b, cx));

        if self.is_global_mark(&name) && self.global_marks.contains_key(&name) {
            self.delete_mark(name.clone(), multibuffer, cx);
        }

        let Some(abs_path) = abs_path else {
            self.multibuffer_marks
                .entry(multibuffer.entity_id())
                .or_default()
                .insert(name.clone(), anchors);
            if self.is_global_mark(&name) {
                self.global_marks
                    .insert(name, MarkLocation::Buffer(multibuffer.entity_id()));
            }
            if let Some(buffer) = buffer {
                let buffer_id = buffer.read(cx).remote_id();
                if !self.watched_buffers.contains_key(&buffer_id) {
                    self.watch_buffer(MarkLocation::Buffer(multibuffer.entity_id()), &buffer, cx)
                }
            }
            return;
        };
        let Some(buffer) = buffer else {
            return;
        };

        let buffer_id = buffer.read(cx).remote_id();
        self.buffer_marks.entry(buffer_id).or_default().insert(
            name.clone(),
            anchors
                .into_iter()
                .filter_map(|anchor| Some(multibuffer_snapshot.anchor_to_buffer_anchor(anchor)?.0))
                .collect(),
        );
        if !self.watched_buffers.contains_key(&buffer_id) {
            self.watch_buffer(MarkLocation::Path(abs_path.clone()), &buffer, cx)
        }
        if self.is_global_mark(&name) {
            self.global_marks
                .insert(name, MarkLocation::Path(abs_path.clone()));
        }
        self.serialize_buffer_marks(abs_path, &buffer, cx)
    }

    pub fn get_mark(
        &self,
        name: &str,
        multi_buffer: &Entity<MultiBuffer>,
        cx: &App,
    ) -> Option<Mark> {
        let target = self.global_marks.get(name);

        if !self.is_global_mark(name) || target.is_some_and(|t| self.points_at(t, multi_buffer, cx))
        {
            if let Some(anchors) = self.multibuffer_marks.get(&multi_buffer.entity_id()) {
                return Some(Mark::Local(anchors.get(name)?.clone()));
            }

            let multibuffer_snapshot = multi_buffer.read(cx).snapshot(cx);
            let buffer_snapshot = multibuffer_snapshot.as_singleton()?;
            if let Some(anchors) = self.buffer_marks.get(&buffer_snapshot.remote_id()) {
                let text_anchors = anchors.get(name)?;
                let anchors = text_anchors
                    .iter()
                    .filter_map(|anchor| multibuffer_snapshot.anchor_in_excerpt(*anchor))
                    .collect();
                return Some(Mark::Local(anchors));
            }
        }

        match target? {
            MarkLocation::Buffer(entity_id) => {
                let anchors = self.multibuffer_marks.get(entity_id)?;
                Some(Mark::Buffer(*entity_id, anchors.get(name)?.clone()))
            }
            MarkLocation::Path(path) => {
                let points = self.serialized_marks.get(path)?;
                Some(Mark::Path(path.clone(), points.get(name)?.clone()))
            }
        }
    }
    pub fn delete_mark(
        &mut self,
        mark_name: String,
        multi_buffer: &Entity<MultiBuffer>,
        cx: &mut Context<Self>,
    ) {
        let path = if let Some(target) = self.global_marks.get(&mark_name.clone()) {
            let name = mark_name.clone();
            if let Some(workspace_id) = self.workspace_id(cx) {
                let db = VimDb::global(cx);
                cx.background_spawn(async move {
                    db.delete_global_marks_path(workspace_id, name).await
                })
                .detach_and_log_err(cx);
            }
            self.buffer_marks.iter_mut().for_each(|(_, m)| {
                m.remove(&mark_name.clone());
            });

            match target {
                MarkLocation::Buffer(entity_id) => {
                    self.multibuffer_marks
                        .get_mut(entity_id)
                        .map(|m| m.remove(&mark_name.clone()));
                    return;
                }
                MarkLocation::Path(path) => path.clone(),
            }
        } else {
            self.multibuffer_marks
                .get_mut(&multi_buffer.entity_id())
                .map(|m| m.remove(&mark_name.clone()));

            if let Some(singleton) = multi_buffer.read(cx).as_singleton() {
                let buffer_id = singleton.read(cx).remote_id();
                self.buffer_marks
                    .get_mut(&buffer_id)
                    .map(|m| m.remove(&mark_name.clone()));
                let Some(path) = self.path_for_buffer(&singleton, cx) else {
                    return;
                };
                path
            } else {
                return;
            }
        };
        self.global_marks.remove(&mark_name);
        self.serialized_marks
            .get_mut(&path)
            .map(|m| m.remove(&mark_name.clone()));
        if let Some(workspace_id) = self.workspace_id(cx) {
            let db = VimDb::global(cx);
            cx.background_spawn(async move { db.delete_mark(workspace_id, path, mark_name).await })
                .detach_and_log_err(cx);
        }
    }
}

impl Global for VimGlobals {}

impl VimGlobals {
    pub(crate) fn with_execution<C: BorrowMut<App>, R>(
        execution: Option<ReplayExecution>,
        cx: &mut C,
        run: impl FnOnce(&mut C) -> R,
    ) -> R {
        let previous = std::mem::replace(
            &mut cx.borrow_mut().global_mut::<Self>().execution,
            execution,
        );
        let result = run(cx);
        cx.borrow_mut().global_mut::<Self>().execution = previous;
        result
    }

    pub(crate) fn is_dot_replaying(&self) -> bool {
        self.execution
            .as_ref()
            .is_some_and(|execution| execution.dot)
    }

    pub(crate) fn input_execution(&self) -> Option<ReplayExecution> {
        self.input
            .borrow()
            .macro_input
            .as_ref()
            .map_or_else(|| self.execution.clone(), |scope| scope.execution.clone())
    }

    pub(crate) fn forced_motion(&self) -> bool {
        self.execution
            .as_ref()
            .and_then(|execution| execution.forced_motion)
            .unwrap_or(self.forced_motion)
    }

    pub(crate) fn execution_is_current(&self) -> bool {
        self.execution.as_ref().is_none_or(|execution| {
            self.replayer
                .as_ref()
                .is_some_and(|replayer| replayer.matches_execution(execution))
        })
    }

    pub(crate) fn with_input<C, R>(
        input: Rc<RefCell<VimInputState>>,
        cx: &mut C,
        run: impl FnOnce(&mut C) -> R,
    ) -> R
    where
        C: BorrowMut<App>,
    {
        let globals = cx.borrow_mut().global_mut::<Self>();
        let previous = globals.input.clone();
        globals.input_depth += 1;
        globals.activate_input(input);
        let result = run(cx);
        let globals = cx.borrow_mut().global_mut::<Self>();
        globals.input_depth -= 1;
        let input = if globals.input_depth == 0 {
            globals.focused_input.clone()
        } else {
            previous
        };
        globals.activate_input(input);
        result
    }

    pub(crate) fn focus_input(&mut self, input: Rc<RefCell<VimInputState>>) {
        self.focused_input = input.clone();
        if self.input_depth == 0 {
            self.activate_input(input);
        }
    }

    pub(crate) fn blur_input(&mut self, input: &Rc<RefCell<VimInputState>>) {
        if Rc::ptr_eq(&self.focused_input, input) {
            self.focus_input(Rc::default());
        }
    }

    pub(crate) fn abort_recording(&mut self) {
        self.dot_recording = false;
        self.stop_recording_after_next_action = false;
        self.recording_actions.clear();
        self.recording_count = None;
        self.recording_selection = RecordedSelection::None;
        self.recording_register_for_dot = None;
    }

    pub(crate) fn finish_recording(&mut self) {
        self.recorded_actions = std::mem::take(&mut self.recording_actions);
        self.recorded_count = self.recording_count.take();
        self.recorded_selection = std::mem::take(&mut self.recording_selection);
        self.recorded_register_for_dot = self.recording_register_for_dot.take();
        self.dot_recording = false;
        self.stop_recording_after_next_action = false;
    }

    pub(crate) fn register(cx: &mut App) {
        cx.set_global(VimGlobals::default());

        cx.observe_keystrokes(|event, window, cx| {
            let vim = Vim::globals(cx)
                .keystroke_observers
                .iter()
                .find(|(focus, _)| window.is_key_dispatching_to(focus))
                .and_then(|(_, vim)| vim.upgrade());
            if let Some(vim) = vim {
                vim.update(cx, |vim, cx| vim.observe_keystrokes(event, window, cx));
            } else if let Some(action) = event.action.as_ref() {
                Self::observe_action_after_input(action.boxed_clone(), cx);
            }
        })
        .detach();

        cx.observe_new(|workspace: &mut Workspace, window, _| {
            RegistersView::register(workspace, window);
        })
        .detach();

        cx.observe_new(move |workspace: &mut Workspace, window, _| {
            MarksView::register(workspace, window);
        })
        .detach();

        let mut was_enabled = None;

        cx.observe_global::<SettingsStore>(move |cx| {
            let is_enabled = Vim::enabled(cx);
            if was_enabled == Some(is_enabled) {
                return;
            }
            was_enabled = Some(is_enabled);
            if is_enabled {
                KeyBinding::set_vim_mode(cx, true);
                CommandPaletteFilter::update_global(cx, |filter, _| {
                    filter.show_namespace(Vim::NAMESPACE);
                });
                GlobalCommandPaletteInterceptor::set(cx, command_interceptor);
                for window in cx.windows() {
                    if let Some(multi_workspace) = window.downcast::<MultiWorkspace>() {
                        multi_workspace
                            .update(cx, |multi_workspace, _, cx| {
                                for workspace in multi_workspace.workspaces() {
                                    workspace.update(cx, |workspace, cx| {
                                        Vim::update_globals(cx, |globals, cx| {
                                            globals.register_workspace(workspace, cx)
                                        });
                                    });
                                }
                            })
                            .ok();
                    }
                }
            } else {
                KeyBinding::set_vim_mode(cx, false);
                *Vim::globals(cx) = VimGlobals::default();
                GlobalCommandPaletteInterceptor::clear(cx);
                CommandPaletteFilter::update_global(cx, |filter, _| {
                    filter.hide_namespace(Vim::NAMESPACE);
                });
            }
        })
        .detach();
        cx.observe_new(|workspace: &mut Workspace, _, cx| {
            Vim::update_globals(cx, |globals, cx| globals.register_workspace(workspace, cx));
        })
        .detach()
    }

    fn register_workspace(&mut self, workspace: &Workspace, cx: &mut Context<Workspace>) {
        let entity_id = cx.entity_id();
        self.marks.insert(entity_id, MarksState::new(workspace, cx));
        cx.observe_release(&cx.entity(), move |_, _, cx| {
            Vim::update_globals(cx, |globals, _| {
                globals.marks.remove(&entity_id);
            })
        })
        .detach();
    }

    pub(crate) fn register_read_access(register: Option<char>, cx: &App) -> Option<NativeInput> {
        if matches!(register, Some('_' | ':' | '.' | '#' | '=' | '%')) {
            return None;
        }
        let access = match register {
            Some('+' | '*') => Some(ClipboardAccess::Read),
            None | Some('"') => match VimSettings::get_global(cx).use_system_clipboard {
                UseSystemClipboard::Always => Some(ClipboardAccess::Read),
                UseSystemClipboard::OnYank => Some(ClipboardAccess::Observe),
                UseSystemClipboard::Never => None,
            },
            _ => None,
        };
        Some(access.map_or_else(NativeInput::default, |access| {
            Self::native_register_input(register.unwrap_or('+'), access)
        }))
    }

    pub(crate) fn register_write_access(
        register: Option<char>,
        is_yank: bool,
        cx: &App,
    ) -> Option<NativeInput> {
        if matches!(register, Some('_' | ':' | '.' | '%' | '#' | '=' | '/')) {
            return None;
        }
        let access = match register {
            Some('+' | '*') => Some(ClipboardAccess::Write),
            None => match VimSettings::get_global(cx).use_system_clipboard {
                UseSystemClipboard::Always => Some(ClipboardAccess::Write),
                UseSystemClipboard::OnYank if is_yank => Some(ClipboardAccess::Write),
                UseSystemClipboard::OnYank => Some(ClipboardAccess::Observe),
                UseSystemClipboard::Never => None,
            },
            _ => None,
        };
        Some(access.map_or_else(NativeInput::default, |access| {
            Self::native_register_input(register.unwrap_or('+'), access)
        }))
    }

    pub(crate) fn native_register_input(register: char, access: ClipboardAccess) -> NativeInput {
        if register == '*' && cfg!(any(target_os = "linux", target_os = "freebsd")) {
            NativeInput {
                primary: Some(access),
                ..NativeInput::default()
            }
        } else {
            NativeInput {
                clipboard: Some(access),
                ..NativeInput::default()
            }
        }
    }

    pub(crate) fn write_registers(
        &mut self,
        editor: &mut Editor,
        content: Register,
        register: Option<char>,
        is_yank: bool,
        kind: MotionKind,
        cx: &mut Context<Editor>,
    ) -> Result<()> {
        if let Some(register) = register {
            let lower = register.to_lowercase().next().unwrap_or(register);
            if lower != register {
                let current = self.registers.entry(lower).or_default();
                current.text = (current.text.to_string() + &content.text).into();
                // not clear how to support appending to registers with multiple cursors
                current.clipboard_selections.take();
                let yanked = current.clone();
                self.registers.insert('"', yanked);
            } else {
                match lower {
                    '_' | ':' | '.' | '%' | '#' | '=' | '/' => {}
                    '+' => {
                        editor.write_to_clipboard(ClipboardItem::from(content.clone()), cx)?;
                        self.registers.insert('"', content);
                    }
                    '*' => {
                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                        editor.write_to_primary(ClipboardItem::from(content.clone()), cx)?;
                        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                        editor.write_to_clipboard(ClipboardItem::from(content.clone()), cx)?;
                        self.registers.insert('"', content);
                    }
                    '"' => {
                        self.registers.insert('"', content.clone());
                        self.registers.insert('0', content);
                    }
                    _ => {
                        self.registers.insert('"', content.clone());
                        self.registers.insert(lower, content);
                    }
                }
            }
        } else {
            match Self::register_write_access(None, is_yank, cx)
                .and_then(|required| required.clipboard)
            {
                Some(ClipboardAccess::Write) => {
                    editor.write_to_clipboard(ClipboardItem::from(content.clone()), cx)?;
                    self.last_yank.replace(content.text.clone());
                }
                Some(ClipboardAccess::Observe) => {
                    if let Ok(Some(item)) = editor.read_from_clipboard(cx)
                        && let Some(text) = item.text()
                    {
                        self.last_yank.replace(SharedString::from(text));
                    }
                }
                Some(ClipboardAccess::Read) | None => {}
            }
            self.registers.insert('"', content.clone());
            if is_yank {
                self.registers.insert('0', content);
            } else {
                let contains_newline = content.text.contains('\n');
                if !contains_newline {
                    self.registers.insert('-', content.clone());
                }
                if kind.linewise() || contains_newline {
                    let mut content = content;
                    for i in '1'..='9' {
                        if let Some(moved) = self.registers.insert(i, content) {
                            content = moved;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn read_register(
        &self,
        register: Option<char>,
        editor: Option<&mut Editor>,
        cx: &mut App,
    ) -> Result<Option<Register>> {
        let read_clipboard = |cx: &App| {
            editor.as_ref().map_or_else(
                || Ok(cx.read_from_clipboard()),
                |editor| editor.read_from_clipboard(cx),
            )
        };
        let Some(register) = register.filter(|reg| *reg != '"') else {
            let setting = VimSettings::get_global(cx).use_system_clipboard;
            return Ok(match setting {
                UseSystemClipboard::Always => read_clipboard(cx)?.map(Register::from),
                UseSystemClipboard::OnYank => {
                    let item = read_clipboard(cx).ok().flatten();
                    if self.system_clipboard_is_newer(item.as_ref()) {
                        item.map(Register::from)
                    } else {
                        self.registers.get(&'"').cloned()
                    }
                }
                UseSystemClipboard::Never => self.registers.get(&'"').cloned(),
            });
        };
        let lower = register.to_lowercase().next().unwrap_or(register);
        Ok(match lower {
            '_' | ':' | '.' | '#' | '=' => None,
            '+' => read_clipboard(cx)?.map(Register::from),
            '*' => {
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                {
                    editor
                        .as_ref()
                        .map_or_else(
                            || Ok(cx.read_from_primary()),
                            |editor| editor.read_from_primary(cx),
                        )?
                        .map(Register::from)
                }
                #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                {
                    read_clipboard(cx)?.map(Register::from)
                }
            }
            '%' => editor.and_then(|editor| {
                let multibuffer = editor.buffer().read(cx);
                let snapshot = multibuffer.snapshot(cx);
                let selection = editor.selections.newest_anchor();
                let buffer = snapshot
                    .anchor_to_buffer_anchor(selection.head())
                    .and_then(|(text_anchor, _)| multibuffer.buffer(text_anchor.buffer_id));
                if let Some(buffer) = buffer {
                    buffer
                        .read(cx)
                        .file()
                        .map(|file| file.path().display(file.path_style(cx)).into_owned().into())
                } else {
                    None
                }
            }),
            _ => self.registers.get(&lower).cloned(),
        })
    }

    fn system_clipboard_is_newer(&self, item: Option<&ClipboardItem>) -> bool {
        item.is_some_and(|item| match (item.text().as_deref(), &self.last_yank) {
            (Some(new), Some(last)) => last.as_ref() != new,
            (Some(_), None) => true,
            (None, _) => false,
        })
    }

    pub(crate) fn observe_action_after_input(action: Box<dyn Action>, cx: &mut App) {
        if !Vim::globals(cx).execution_is_current() {
            return;
        }
        Vim::globals(cx).observe_macro_action(Some(action.as_ref()));
        let deferred_action = action.boxed_clone();
        if Self::defer_input(
            move |_, cx| {
                let globals = Vim::globals(cx);
                if globals.execution_is_current() {
                    globals.observe_action(deferred_action);
                }
            },
            cx,
        ) {
            return;
        }
        Vim::globals(cx).observe_action(action);
    }

    pub(crate) fn defer_input(
        continuation: impl FnOnce(&mut Window, &mut App) + 'static,
        cx: &mut App,
    ) -> bool {
        let input = Vim::globals(cx).input.clone();
        let command = VimCommandContext::capture(input.clone(), cx);
        let editor = input.borrow().editor.as_ref().and_then(WeakEntity::upgrade);
        editor.is_some_and(|editor| {
            editor.update(cx, |editor, cx| {
                editor.defer_external_input(
                    move |window, cx| command.run(cx, |cx| continuation(window, cx)),
                    cx,
                )
            })
        })
    }

    pub fn observe_action(&mut self, action: Box<dyn Action>) {
        if self.dot_recording {
            self.recording_actions
                .push(ReplayableAction::Action(action.boxed_clone()));

            if self.stop_recording_after_next_action {
                self.finish_recording();
            }
        }
    }

    pub(crate) fn recording_checkpoint(&mut self) -> RecordingCheckpoint {
        let input = self
            .input
            .borrow()
            .macro_input
            .as_ref()
            .and_then(|scope| scope.recording.clone());
        let macro_action = input.and_then(|input| {
            let input = input.0.borrow();
            let action_index = input.submitted_action?;
            let ReplayableAction::Pending(pending) = self
                .recordings
                .get_mut(&input.register)?
                .get_mut(input.index)?
            else {
                return None;
            };
            if !Rc::ptr_eq(&pending.identity, &input.identity) {
                return None;
            }
            let identity = Rc::new(());
            pending.identity = identity.clone();
            Some((input.register, input.index, action_index, identity))
        });
        RecordingCheckpoint {
            dot: self.dot_recording.then_some(self.recording_actions.len()),
            macro_action,
        }
    }

    pub(crate) fn discard_recorded_action(
        &mut self,
        checkpoint: RecordingCheckpoint,
        action: &dyn Action,
    ) {
        if let Some(index) = checkpoint.dot
            && self.recording_actions.get(index).is_some_and(|recorded| {
                matches!(recorded, ReplayableAction::Action(recorded) if recorded.partial_eq(action))
            })
        {
            self.recording_actions.remove(index);
        }
        let Some((register, index, action_index, identity)) = checkpoint.macro_action else {
            return;
        };
        let Some(ReplayableAction::Pending(pending)) = self
            .recordings
            .get_mut(&register)
            .and_then(|recording| recording.get_mut(index))
        else {
            return;
        };
        if !Rc::ptr_eq(&pending.identity, &identity) {
            return;
        }
        let completion = pending.completion.clone();
        let action = action.boxed_clone();
        pending.completion = async move {
            let mut actions = completion.await?;
            if actions.get(action_index).is_some_and(|recorded| {
                matches!(recorded, ReplayableAction::Action(recorded) if recorded.partial_eq(action.as_ref()))
            }) {
                actions.remove(action_index);
            }
            Some(actions)
        }.boxed_local().shared();
    }

    pub(crate) fn stop_macro_recording(&mut self) -> bool {
        if let Some(scope) = &self.input.borrow().macro_input
            && let Some(input) = &scope.recording
        {
            let input = input.0.borrow();
            if self.recording_register != Some(input.register)
                || !self
                    .recordings
                    .get(&input.register)
                    .and_then(|recording| recording.get(input.index))
                    .is_some_and(|action| match action {
                        ReplayableAction::Pending(pending) => {
                            Rc::ptr_eq(&pending.identity, &input.identity)
                        }
                        _ => false,
                    })
            {
                return true;
            }
        }
        if let Some(register) = self.recording_register.take() {
            self.last_recorded_register = Some(register);
            true
        } else {
            false
        }
    }

    pub(crate) fn submit_macro_action(&mut self, action: &dyn Action) {
        if !action.name().starts_with("vim::")
            || action.as_any().is::<crate::normal::repeat::ToggleRecord>()
        {
            return;
        }
        if let Some(scope) = &self.input.borrow().macro_input
            && let Some(input) = &scope.recording
        {
            let mut input = input.0.as_ref().borrow_mut();
            let action = ReplayableAction::Action(action.boxed_clone());
            if let Some(index) = input.submitted_action
                && let Some(recorded) = input.actions.get_mut(index)
            {
                *recorded = action;
            } else {
                input.submitted_action = Some(input.actions.len());
                input.actions.push(action);
            }
        }
    }

    pub(crate) fn observe_macro_action(&mut self, action: Option<&dyn Action>) {
        let action =
            action.filter(|action| !action.as_any().is::<crate::normal::repeat::ToggleRecord>());
        let scope = self.input.borrow().macro_input.clone();
        if let Some(scope) = scope {
            if let Some(input) = &scope.recording {
                let mut input = input.0.as_ref().borrow_mut();
                if let Some(index) = input.submitted_action.take() {
                    if let Some(action) = action {
                        if let Some(recorded) = input.actions.get_mut(index) {
                            *recorded = ReplayableAction::Action(action.boxed_clone());
                        }
                    } else if index < input.actions.len() {
                        input.actions.remove(index);
                    }
                } else if let Some(action) = action {
                    input
                        .actions
                        .push(ReplayableAction::Action(action.boxed_clone()));
                }
            }
        } else if self.execution.is_none()
            && let Some(register) = self.recording_register
            && let Some(action) = action
        {
            self.recordings
                .entry(register)
                .or_default()
                .push(ReplayableAction::Action(action.boxed_clone()));
        }
    }

    pub fn observe_insertion(&mut self, text: &Arc<str>, range_to_replace: Option<Range<isize>>) {
        if self.ignore_current_insertion {
            self.ignore_current_insertion = false;
            return;
        }
        if self.dot_recording {
            self.recording_actions.push(ReplayableAction::Insertion {
                text: text.clone(),
                utf16_range_to_replace: range_to_replace.clone(),
            });
            if self.stop_recording_after_next_action {
                self.finish_recording();
            }
        }
        let scope = self.input.borrow().macro_input.clone();
        let action = ReplayableAction::Insertion {
            text: text.clone(),
            utf16_range_to_replace: range_to_replace,
        };
        if let Some(scope) = scope {
            if let Some(input) = &scope.recording {
                input.0.as_ref().borrow_mut().actions.push(action);
            }
        } else if self.execution.is_none()
            && let Some(register) = self.recording_register
        {
            self.recordings.entry(register).or_default().push(action);
        }
    }

    pub fn focused_vim(&self) -> Option<Entity<Vim>> {
        self.focused_vim.as_ref().and_then(|vim| vim.upgrade())
    }

    fn activate_input(&mut self, input: Rc<RefCell<VimInputState>>) {
        if !Rc::ptr_eq(&self.input, &input) {
            self.input.clone().as_ref().borrow_mut().swap(self);
            input.as_ref().borrow_mut().swap(self);
            self.input = input;
        }
    }
}

impl Vim {
    pub fn globals(cx: &mut App) -> &mut VimGlobals {
        cx.global_mut::<VimGlobals>()
    }

    pub fn update_globals<C, R>(cx: &mut C, f: impl FnOnce(&mut VimGlobals, &mut C) -> R) -> R
    where
        C: BorrowMut<App>,
    {
        cx.update_global(f)
    }
}

#[derive(Clone)]
pub struct PendingMacroInput {
    pub(crate) completion: Shared<LocalBoxFuture<'static, Option<Vec<ReplayableAction>>>>,
    identity: Rc<()>,
}

impl std::fmt::Debug for PendingMacroInput {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PendingMacroInput")
            .finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub(crate) struct MacroInput(Rc<RefCell<MacroInputState>>);

pub(crate) struct MacroInputScope {
    input: Rc<RefCell<VimInputState>>,
    state: Rc<MacroInputScopeState>,
}

struct MacroInputScopeState {
    previous: Option<Rc<MacroInputScopeState>>,
    recording: Option<MacroInput>,
    execution: Option<ReplayExecution>,
    closed: Cell<bool>,
}

struct MacroInputState {
    register: char,
    index: usize,
    identity: Rc<()>,
    actions: Vec<ReplayableAction>,
    submitted_action: Option<usize>,
    sender: Option<oneshot::Sender<Vec<ReplayableAction>>>,
}

impl MacroInput {
    pub(crate) fn capture(cx: &mut App) -> Option<Self> {
        let globals = Vim::globals(cx);
        let register = globals.recording_register?;
        let recording = globals.recordings.entry(register).or_default();
        let index = recording.len();
        let identity = Rc::new(());
        let (sender, receiver) = oneshot::channel();
        recording.push(ReplayableAction::Pending(PendingMacroInput {
            identity: identity.clone(),
            completion: receiver.map(|result| result.ok()).boxed_local().shared(),
        }));
        Some(Self(Rc::new(RefCell::new(MacroInputState {
            register,
            index,
            identity,
            actions: Vec::new(),
            submitted_action: None,
            sender: Some(sender),
        }))))
    }

    fn finish(&self, cx: &mut App) {
        let mut input = self.0.as_ref().borrow_mut();
        let actions = std::mem::take(&mut input.actions);
        if let Some(recording) = Vim::globals(cx).recordings.get_mut(&input.register)
            && let Some(ReplayableAction::Pending(pending)) = recording.get(input.index)
            && Rc::ptr_eq(&pending.identity, &input.identity)
        {
            if let [action] = actions.as_slice() {
                if let Some(recorded) = recording.get_mut(input.index) {
                    *recorded = action.clone();
                }
            } else if input.index + 1 == recording.len() {
                recording.pop();
                recording.extend(actions.iter().cloned());
            }
        }
        if let Some(sender) = input.sender.take() {
            sender.send(actions).ok();
        }
    }
}

impl MacroInputScope {
    pub(crate) fn new(
        input: Rc<RefCell<VimInputState>>,
        mut recording: Option<MacroInput>,
        cx: &mut App,
    ) -> Self {
        if recording.is_none()
            && Vim::globals(cx)
                .recording_input
                .ptr_eq(&Rc::downgrade(&input))
        {
            recording = MacroInput::capture(cx);
        }
        Self::start(input, recording, None)
    }

    pub(crate) fn replay(input: Rc<RefCell<VimInputState>>, execution: ReplayExecution) -> Self {
        Self::start(input, None, Some(execution))
    }

    pub(crate) fn finish_after_input(self, cx: &mut App) {
        cx.defer(move |cx| {
            cx.defer(move |cx| {
                let editor = self
                    .input
                    .borrow()
                    .editor
                    .as_ref()
                    .and_then(WeakEntity::upgrade);
                if let Some(editor) = editor {
                    let scope = Rc::new(RefCell::new(Some(self)));
                    let deferred_scope = scope.clone();
                    let deferred = editor.update(cx, |editor, cx| {
                        editor.defer_external_input(
                            move |_, cx| {
                                if let Some(scope) = deferred_scope.as_ref().borrow_mut().take() {
                                    scope.finish(cx);
                                }
                            },
                            cx,
                        )
                    });
                    if !deferred && let Some(scope) = scope.as_ref().borrow_mut().take() {
                        scope.finish(cx);
                    }
                }
            });
        });
    }

    fn start(
        input: Rc<RefCell<VimInputState>>,
        recording: Option<MacroInput>,
        execution: Option<ReplayExecution>,
    ) -> Self {
        let state = Rc::new(MacroInputScopeState {
            previous: input.as_ref().borrow_mut().macro_input.take(),
            recording,
            execution,
            closed: Cell::new(false),
        });
        input.as_ref().borrow_mut().macro_input = Some(state.clone());
        Self { input, state }
    }

    fn finish(self, cx: &mut App) {
        if let Some(recording) = &self.state.recording {
            recording.finish(cx);
        }
    }
}

impl Drop for MacroInputScope {
    fn drop(&mut self) {
        self.state.closed.set(true);
        let mut input = self.input.as_ref().borrow_mut();
        while let Some(scope) = &input.macro_input
            && scope.closed.get()
        {
            input.macro_input = scope.previous.clone();
        }
    }
}

impl Drop for MacroInputState {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            sender.send(std::mem::take(&mut self.actions)).ok();
        }
    }
}

#[derive(Debug)]
pub enum ReplayableAction {
    Action(Box<dyn Action>),
    Pending(PendingMacroInput),
    Insertion {
        text: Arc<str>,
        utf16_range_to_replace: Option<Range<isize>>,
    },
}

impl Clone for ReplayableAction {
    fn clone(&self) -> Self {
        match self {
            Self::Action(action) => Self::Action(action.boxed_clone()),
            Self::Pending(input) => Self::Pending(input.clone()),
            Self::Insertion {
                text,
                utf16_range_to_replace,
            } => Self::Insertion {
                text: text.clone(),
                utf16_range_to_replace: utf16_range_to_replace.clone(),
            },
        }
    }
}

#[derive(Default, Debug)]
pub struct SearchState {
    pub direction: Direction,
    pub count: usize,
    pub cmd_f_search: bool,

    pub prior_selections: Vec<Range<Anchor>>,
    pub prior_operator: Option<Operator>,
    pub prior_mode: Mode,
    pub helix_select: bool,
    pub _dismiss_subscription: Option<gpui::Subscription>,
}

impl Operator {
    pub fn id(&self) -> &'static str {
        match self {
            Operator::Object { around: false } => "i",
            Operator::Object { around: true } => "a",
            Operator::Change => "c",
            Operator::Delete => "d",
            Operator::Yank => "y",
            Operator::Replace => "r",
            Operator::Digraph { .. } => "^K",
            Operator::Literal { .. } => "^V",
            Operator::FindForward { before: false, .. } => "f",
            Operator::FindForward { before: true, .. } => "t",
            Operator::Sneak { .. } => "s",
            Operator::SneakBackward { .. } => "S",
            Operator::FindBackward { after: false, .. } => "F",
            Operator::FindBackward { after: true, .. } => "T",
            Operator::AddSurrounds { .. } => "ys",
            Operator::ChangeSurrounds { .. } => "cs",
            Operator::DeleteSurrounds => "ds",
            Operator::Mark => "m",
            Operator::Jump { line: true } => "'",
            Operator::Jump { line: false } => "`",
            Operator::Indent => ">",
            Operator::AutoIndent => "eq",
            Operator::ShellCommand => "sh",
            Operator::Rewrap => "gq",
            Operator::ReplaceWithRegister => "gR",
            Operator::Exchange => "cx",
            Operator::Outdent => "<",
            Operator::Uppercase => "gU",
            Operator::Lowercase => "gu",
            Operator::OppositeCase => "g~",
            Operator::Rot13 => "g?",
            Operator::Rot47 => "g?",
            Operator::Register { .. } => "\"",
            Operator::RecordRegister => "q",
            Operator::ReplayRegister => "@",
            Operator::ToggleComments => "gc",
            Operator::ToggleBlockComments => "gb",
            Operator::HelixMatch => "helix_m",
            Operator::HelixNext { .. } => "helix_next",
            Operator::HelixPrevious { .. } => "helix_previous",
            Operator::HelixJump { .. } => "gw",
            Operator::HelixSurroundAdd => "helix_ms",
            Operator::HelixSurroundReplace { .. } => "helix_mr",
            Operator::HelixSurroundDelete => "helix_md",
        }
    }

    pub fn status(&self) -> String {
        fn make_visible(c: &str) -> &str {
            match c {
                "\n" => "enter",
                "\t" => "tab",
                " " => "space",
                c => c,
            }
        }
        match self {
            Operator::Digraph {
                first_char: Some(first_char),
            } => format!("^K{}", make_visible(&first_char.to_string())),
            Operator::Literal {
                prefix: Some(prefix),
            } => format!("^V{}", make_visible(prefix)),
            Operator::AutoIndent => "=".to_string(),
            Operator::ShellCommand => "=".to_string(),
            Operator::HelixMatch => "m".to_string(),
            Operator::HelixNext { .. } => "]".to_string(),
            Operator::HelixPrevious { .. } => "[".to_string(),
            Operator::HelixJump { .. } => "gw".to_string(),
            Operator::HelixSurroundAdd => "ms".to_string(),
            Operator::HelixSurroundReplace {
                replaced_char: None,
            } => "mr".to_string(),
            Operator::HelixSurroundReplace {
                replaced_char: Some(c),
            } => format!("mr{}", c),
            Operator::HelixSurroundDelete => "md".to_string(),
            _ => self.id().to_string(),
        }
    }

    pub fn is_waiting(&self, mode: Mode) -> bool {
        match self {
            Operator::AddSurrounds { target } => target.is_some() || mode.is_visual(),
            Operator::FindForward { .. }
            | Operator::Mark
            | Operator::Jump { .. }
            | Operator::FindBackward { .. }
            | Operator::Sneak { .. }
            | Operator::SneakBackward { .. }
            | Operator::Register { .. }
            | Operator::RecordRegister
            | Operator::ReplayRegister
            | Operator::Replace
            | Operator::Digraph { .. }
            | Operator::Literal { .. }
            | Operator::ChangeSurrounds {
                target: Some(_), ..
            }
            | Operator::DeleteSurrounds
            | Operator::HelixJump { .. } => true,
            Operator::Change
            | Operator::Delete
            | Operator::Yank
            | Operator::Rewrap
            | Operator::Indent
            | Operator::Outdent
            | Operator::AutoIndent
            | Operator::ShellCommand
            | Operator::Lowercase
            | Operator::Uppercase
            | Operator::Rot13
            | Operator::Rot47
            | Operator::ReplaceWithRegister
            | Operator::Exchange
            | Operator::Object { .. }
            | Operator::ChangeSurrounds { target: None, .. }
            | Operator::OppositeCase
            | Operator::ToggleComments
            | Operator::ToggleBlockComments
            | Operator::HelixMatch
            | Operator::HelixNext { .. }
            | Operator::HelixPrevious { .. } => false,
            Operator::HelixSurroundAdd
            | Operator::HelixSurroundReplace { .. }
            | Operator::HelixSurroundDelete => true,
        }
    }

    pub fn starts_dot_recording(&self) -> bool {
        match self {
            Operator::Change
            | Operator::Delete
            | Operator::Replace
            | Operator::Indent
            | Operator::Outdent
            | Operator::AutoIndent
            | Operator::Lowercase
            | Operator::Uppercase
            | Operator::OppositeCase
            | Operator::Rot13
            | Operator::Rot47
            | Operator::ToggleComments
            | Operator::ToggleBlockComments
            | Operator::ReplaceWithRegister
            | Operator::Rewrap
            | Operator::ShellCommand
            | Operator::AddSurrounds { target: None }
            | Operator::ChangeSurrounds { target: None, .. }
            | Operator::DeleteSurrounds
            | Operator::Exchange
            | Operator::HelixNext { .. }
            | Operator::HelixPrevious { .. }
            | Operator::HelixSurroundAdd
            | Operator::HelixSurroundReplace { .. }
            | Operator::HelixSurroundDelete => true,
            Operator::Yank
            | Operator::Object { .. }
            | Operator::FindForward { .. }
            | Operator::FindBackward { .. }
            | Operator::Sneak { .. }
            | Operator::SneakBackward { .. }
            | Operator::Mark
            | Operator::Digraph { .. }
            | Operator::Literal { .. }
            | Operator::AddSurrounds { .. }
            | Operator::ChangeSurrounds { .. }
            | Operator::Jump { .. }
            | Operator::Register { .. }
            | Operator::RecordRegister
            | Operator::ReplayRegister
            | Operator::HelixMatch
            | Operator::HelixJump { .. } => false,
        }
    }
}

struct RegisterMatch {
    name: char,
    contents: SharedString,
}

pub struct RegistersViewDelegate {
    selected_index: usize,
    matches: Vec<RegisterMatch>,
}

impl PickerDelegate for RegistersViewDelegate {
    type ListItem = Div;

    fn name() -> &'static str {
        "registers view"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::default()
    }

    fn update_matches(
        &mut self,
        _: String,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        Task::ready(())
    }

    fn confirm(&mut self, _: bool, _: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn dismissed(&mut self, _: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let register_match = self.matches.get(ix)?;

        let mut output = String::new();
        let mut runs = Vec::new();
        output.push('"');
        output.push(register_match.name);
        runs.push((
            0..output.len(),
            HighlightStyle::color(cx.theme().colors().text_accent),
        ));
        output.push(' ');
        output.push(' ');
        let mut base = output.len();
        for (ix, c) in register_match.contents.char_indices() {
            if ix > 100 {
                break;
            }
            let replace = match c {
                '\t' => Some("\\t".to_string()),
                '\n' => Some("\\n".to_string()),
                '\r' => Some("\\r".to_string()),
                c if is_invisible(c) => {
                    if c <= '\x1f' {
                        replacement(c).map(|s| s.to_string())
                    } else {
                        Some(format!("\\u{:04X}", c as u32))
                    }
                }
                _ => None,
            };
            let Some(replace) = replace else {
                output.push(c);
                continue;
            };
            output.push_str(&replace);
            runs.push((
                base + ix..base + ix + replace.len(),
                HighlightStyle::color(cx.theme().colors().text_muted),
            ));
            base += replace.len() - c.len_utf8();
        }

        let theme = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().editor_foreground,
            font_family: theme.buffer_font.family.clone(),
            font_features: theme.buffer_font.features.clone(),
            font_fallbacks: theme.buffer_font.fallbacks.clone(),
            font_size: theme.buffer_font_size(cx).into(),
            line_height: (theme.line_height() * theme.buffer_font_size(cx)).into(),
            font_weight: theme.buffer_font.weight,
            font_style: theme.buffer_font.style,
            ..Default::default()
        };

        Some(
            h_flex()
                .when(selected, |el| el.bg(cx.theme().colors().element_selected))
                .font_buffer(cx)
                .text_buffer(cx)
                .h(theme.buffer_font_size(cx) * theme.line_height())
                .px_2()
                .gap_1()
                .child(StyledText::new(output).with_default_highlights(&text_style, runs)),
        )
    }
}

pub struct RegistersView {}

impl RegistersView {
    fn register(workspace: &mut Workspace, _window: Option<&mut Window>) {
        workspace.register_action(|workspace, _: &ToggleRegistersView, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let editor = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx));
        workspace.toggle_modal(window, cx, move |window, cx| {
            RegistersView::new(editor, window, cx)
        });
    }

    fn new(
        editor: Option<Entity<Editor>>,
        window: &mut Window,
        cx: &mut Context<Picker<RegistersViewDelegate>>,
    ) -> Picker<RegistersViewDelegate> {
        let mut matches = Vec::default();
        cx.update_global(|globals: &mut VimGlobals, cx| {
            for name in ['"', '+', '*'] {
                if let Some(register) = globals
                    .read_register(Some(name), None, cx)
                    .log_err()
                    .flatten()
                {
                    matches.push(RegisterMatch {
                        name,
                        contents: register.text.clone(),
                    })
                }
            }
            if let Some(editor) = editor {
                let register = editor.update(cx, |editor, cx| {
                    globals.read_register(Some('%'), Some(editor), cx)
                });
                if let Some(register) = register.log_err().flatten() {
                    matches.push(RegisterMatch {
                        name: '%',
                        contents: register.text,
                    })
                }
            }
            for (name, register) in globals.registers.iter() {
                if ['"', '+', '*', '%'].contains(name) {
                    continue;
                };
                matches.push(RegisterMatch {
                    name: *name,
                    contents: register.text.clone(),
                })
            }
        });
        matches.sort_by_key(|m| m.name);
        let delegate = RegistersViewDelegate {
            selected_index: 0,
            matches,
        };

        Picker::nonsearchable_uniform_list(delegate, window, cx).initial_width(rems(36.))
    }
}

enum MarksMatchInfo {
    Path(Arc<Path>),
    Title(String),
    Content {
        line: String,
        highlights: Vec<(Range<usize>, HighlightStyle)>,
    },
}

impl MarksMatchInfo {
    fn from_chunks<'a>(chunks: impl Iterator<Item = Chunk<'a>>, cx: &App) -> Self {
        let mut line = String::new();
        let mut highlights = Vec::new();
        let mut offset = 0;
        for chunk in chunks {
            line.push_str(chunk.text);
            if let Some(highlight_id) = chunk.syntax_highlight_id
                && let Some(highlight) = cx.theme().syntax().get(highlight_id).cloned()
            {
                highlights.push((offset..offset + chunk.text.len(), highlight))
            }
            offset += chunk.text.len();
        }
        MarksMatchInfo::Content { line, highlights }
    }
}

struct MarksMatch {
    name: String,
    position: Point,
    info: MarksMatchInfo,
}

pub struct MarksViewDelegate {
    selected_index: usize,
    matches: Vec<MarksMatch>,
    point_column_width: usize,
    workspace: WeakEntity<Workspace>,
}

impl PickerDelegate for MarksViewDelegate {
    type ListItem = Div;

    fn name() -> &'static str {
        "marks view"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::default()
    }

    fn update_matches(
        &mut self,
        _: String,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(());
        };
        cx.spawn(async move |picker, cx| {
            let mut matches = Vec::new();
            let _ = workspace.update(cx, |workspace, cx| {
                let entity_id = cx.entity_id();
                let Some(editor) = workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
                else {
                    return;
                };
                let editor = editor.read(cx);
                let mut has_seen = HashSet::new();
                let Some(marks_state) = cx.global::<VimGlobals>().marks.get(&entity_id) else {
                    return;
                };
                let marks_state = marks_state.read(cx);

                if let Some(map) = marks_state
                    .multibuffer_marks
                    .get(&editor.buffer().entity_id())
                {
                    for (name, anchors) in map {
                        if has_seen.contains(name) {
                            continue;
                        }
                        has_seen.insert(name.clone());
                        let Some(anchor) = anchors.first() else {
                            continue;
                        };

                        let snapshot = editor.buffer().read(cx).snapshot(cx);
                        let position = anchor.to_point(&snapshot);

                        let chunks = snapshot.chunks(
                            Point::new(position.row, 0)
                                ..Point::new(
                                    position.row,
                                    snapshot.line_len(MultiBufferRow(position.row)),
                                ),
                            LanguageAwareStyling {
                                tree_sitter: true,
                                diagnostics: true,
                            },
                        );
                        matches.push(MarksMatch {
                            name: name.clone(),
                            position,
                            info: MarksMatchInfo::from_chunks(chunks, cx),
                        })
                    }
                }

                if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                    let buffer = buffer.read(cx);
                    if let Some(map) = marks_state.buffer_marks.get(&buffer.remote_id()) {
                        for (name, anchors) in map {
                            if has_seen.contains(name) {
                                continue;
                            }
                            has_seen.insert(name.clone());
                            let Some(anchor) = anchors.first() else {
                                continue;
                            };
                            let snapshot = buffer.snapshot();
                            let position = anchor.to_point(&snapshot);
                            let chunks = snapshot.chunks(
                                Point::new(position.row, 0)
                                    ..Point::new(position.row, snapshot.line_len(position.row)),
                                LanguageAwareStyling {
                                    tree_sitter: true,
                                    diagnostics: true,
                                },
                            );

                            matches.push(MarksMatch {
                                name: name.clone(),
                                position,
                                info: MarksMatchInfo::from_chunks(chunks, cx),
                            })
                        }
                    }
                }

                for (name, mark_location) in marks_state.global_marks.iter() {
                    if has_seen.contains(name) {
                        continue;
                    }
                    has_seen.insert(name.clone());

                    match mark_location {
                        MarkLocation::Buffer(entity_id) => {
                            if let Some(&anchor) = marks_state
                                .multibuffer_marks
                                .get(entity_id)
                                .and_then(|map| map.get(name))
                                .and_then(|anchors| anchors.first())
                            {
                                let Some((info, snapshot)) = workspace
                                    .items(cx)
                                    .filter_map(|item| item.act_as::<Editor>(cx))
                                    .map(|entity| entity.read(cx).buffer())
                                    .find(|buffer| buffer.entity_id().eq(entity_id))
                                    .map(|buffer| {
                                        (
                                            MarksMatchInfo::Title(
                                                buffer.read(cx).title(cx).to_string(),
                                            ),
                                            buffer.read(cx).snapshot(cx),
                                        )
                                    })
                                else {
                                    continue;
                                };
                                matches.push(MarksMatch {
                                    name: name.clone(),
                                    position: anchor.to_point(&snapshot),
                                    info,
                                });
                            }
                        }
                        MarkLocation::Path(path) => {
                            if let Some(&position) = marks_state
                                .serialized_marks
                                .get(path.as_ref())
                                .and_then(|map| map.get(name))
                                .and_then(|points| points.first())
                            {
                                let info = MarksMatchInfo::Path(path.clone());
                                matches.push(MarksMatch {
                                    name: name.clone(),
                                    position,
                                    info,
                                });
                            }
                        }
                    }
                }
            });
            let _ = picker.update(cx, |picker, cx| {
                matches.sort_by_key(|a| {
                    (
                        a.name.chars().next().map(|c| c.is_ascii_uppercase()),
                        a.name.clone(),
                    )
                });
                let digits = matches
                    .iter()
                    .map(|m| (m.position.row + 1).ilog10() + (m.position.column + 1).ilog10())
                    .max()
                    .unwrap_or_default();
                picker.delegate.matches = matches;
                picker.delegate.point_column_width = (digits + 4) as usize;
                cx.notify();
            });
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(vim) = self
            .workspace
            .upgrade()
            .map(|w| w.read(cx))
            .and_then(|w| w.focused_pane(window, cx).read(cx).active_item())
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor| editor.read(cx).addon::<VimAddon>().cloned())
            .map(|addon| addon.entity)
        else {
            return;
        };
        let Some(text): Option<Arc<str>> = self
            .matches
            .get(self.selected_index)
            .map(|m| Arc::from(m.name.to_string().into_boxed_str()))
        else {
            return;
        };
        vim.update(cx, |vim, cx| {
            vim.jump(text, false, false, window, cx);
        });

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mark_match = self.matches.get(ix)?;

        let mut left_output = String::new();
        let mut left_runs = Vec::new();
        left_output.push('`');
        left_output.push_str(&mark_match.name);
        left_runs.push((
            0..left_output.len(),
            HighlightStyle::color(cx.theme().colors().text_accent),
        ));
        left_output.push(' ');
        left_output.push(' ');
        let point_column = format!(
            "{},{}",
            mark_match.position.row + 1,
            mark_match.position.column + 1
        );
        left_output.push_str(&point_column);
        if let Some(padding) = self.point_column_width.checked_sub(point_column.len()) {
            left_output.push_str(&" ".repeat(padding));
        }

        let (right_output, right_runs): (String, Vec<_>) = match &mark_match.info {
            MarksMatchInfo::Path(path) => {
                let s = path.to_string_lossy().into_owned();
                (
                    s.clone(),
                    vec![(0..s.len(), HighlightStyle::color(cx.theme().colors().text))],
                )
            }
            MarksMatchInfo::Title(title) => (
                title.clone(),
                vec![(
                    0..title.len(),
                    HighlightStyle::color(cx.theme().colors().text),
                )],
            ),
            MarksMatchInfo::Content { line, highlights } => (line.clone(), highlights.clone()),
        };

        let theme = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().editor_foreground,
            font_family: theme.buffer_font.family.clone(),
            font_features: theme.buffer_font.features.clone(),
            font_fallbacks: theme.buffer_font.fallbacks.clone(),
            font_size: theme.buffer_font_size(cx).into(),
            line_height: (theme.line_height() * theme.buffer_font_size(cx)).into(),
            font_weight: theme.buffer_font.weight,
            font_style: theme.buffer_font.style,
            ..Default::default()
        };

        Some(
            h_flex()
                .when(selected, |el| el.bg(cx.theme().colors().element_selected))
                .font_buffer(cx)
                .text_buffer(cx)
                .h(theme.buffer_font_size(cx) * theme.line_height())
                .px_2()
                .child(StyledText::new(left_output).with_default_highlights(&text_style, left_runs))
                .child(
                    StyledText::new(right_output).with_default_highlights(&text_style, right_runs),
                ),
        )
    }
}

pub struct MarksView {}

impl MarksView {
    fn register(workspace: &mut Workspace, _window: Option<&mut Window>) {
        workspace.register_action(|workspace, _: &ToggleMarksView, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let handle = cx.weak_entity();
        workspace.toggle_modal(window, cx, move |window, cx| {
            MarksView::new(handle, window, cx)
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Picker<MarksViewDelegate>>,
    ) -> Picker<MarksViewDelegate> {
        let matches = Vec::default();
        let delegate = MarksViewDelegate {
            selected_index: 0,
            point_column_width: 0,
            matches,
            workspace,
        };
        Picker::nonsearchable_uniform_list(delegate, window, cx).initial_width(rems(36.))
    }
}

pub struct VimDb(ThreadSafeConnection);

impl Domain for VimDb {
    const NAME: &str = stringify!(VimDb);

    const MIGRATIONS: &[&str] = &[
        sql! (
            CREATE TABLE vim_marks (
              workspace_id INTEGER,
              mark_name TEXT,
              path BLOB,
              value TEXT
            );
            CREATE UNIQUE INDEX idx_vim_marks ON vim_marks (workspace_id, mark_name, path);
        ),
        sql! (
            CREATE TABLE vim_global_marks_paths(
                workspace_id INTEGER,
                mark_name TEXT,
                path BLOB
            );
            CREATE UNIQUE INDEX idx_vim_global_marks_paths
            ON vim_global_marks_paths(workspace_id, mark_name);
        ),
    ];
}

db::static_connection!(VimDb, [WorkspaceDb]);

struct SerializedMark {
    path: Arc<Path>,
    name: String,
    points: Vec<Point>,
}

impl VimDb {
    pub(crate) async fn set_marks(
        &self,
        workspace_id: WorkspaceId,
        path: Arc<Path>,
        marks: HashMap<String, Vec<Point>>,
    ) -> Result<()> {
        log::debug!("Setting path {path:?} for {} marks", marks.len());

        self.write(move |conn| {
            let mut query = conn.exec_bound(sql!(
                INSERT OR REPLACE INTO vim_marks
                    (workspace_id, mark_name, path, value)
                VALUES
                    (?, ?, ?, ?)
            ))?;
            for (mark_name, value) in marks {
                let pairs: Vec<(u32, u32)> = value
                    .into_iter()
                    .map(|point| (point.row, point.column))
                    .collect();
                let serialized = serde_json::to_string(&pairs)?;
                query((workspace_id, mark_name, path.clone(), serialized))?;
            }
            Ok(())
        })
        .await
    }

    fn get_marks(&self, workspace_id: WorkspaceId) -> Result<Vec<SerializedMark>> {
        let result: Vec<(Arc<Path>, String, String)> = self.select_bound(sql!(
            SELECT path, mark_name, value FROM vim_marks
                WHERE workspace_id = ?
        ))?(workspace_id)?;

        Ok(result
            .into_iter()
            .filter_map(|(path, name, value)| {
                let pairs: Vec<(u32, u32)> = serde_json::from_str(&value).log_err()?;
                Some(SerializedMark {
                    path,
                    name,
                    points: pairs
                        .into_iter()
                        .map(|(row, column)| Point { row, column })
                        .collect(),
                })
            })
            .collect())
    }

    pub(crate) async fn delete_mark(
        &self,
        workspace_id: WorkspaceId,
        path: Arc<Path>,
        mark_name: String,
    ) -> Result<()> {
        self.write(move |conn| {
            conn.exec_bound(sql!(
                DELETE FROM vim_marks
                WHERE workspace_id = ? AND mark_name = ? AND path = ?
            ))?((workspace_id, mark_name, path))
        })
        .await
    }

    pub(crate) async fn set_global_mark_path(
        &self,
        workspace_id: WorkspaceId,
        mark_name: String,
        path: Arc<Path>,
    ) -> Result<()> {
        log::debug!("Setting global mark path {path:?} for {mark_name}");
        self.write(move |conn| {
            conn.exec_bound(sql!(
                INSERT OR REPLACE INTO vim_global_marks_paths
                    (workspace_id, mark_name, path)
                VALUES
                    (?, ?, ?)
            ))?((workspace_id, mark_name, path))
        })
        .await
    }

    pub fn get_global_marks_paths(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<(String, Arc<Path>)>> {
        self.select_bound(sql!(
        SELECT mark_name, path FROM vim_global_marks_paths
            WHERE workspace_id = ?
        ))?(workspace_id)
    }

    pub(crate) async fn delete_global_marks_path(
        &self,
        workspace_id: WorkspaceId,
        mark_name: String,
    ) -> Result<()> {
        self.write(move |conn| {
            conn.exec_bound(sql!(
                DELETE FROM vim_global_marks_paths
                WHERE workspace_id = ? AND mark_name = ?
            ))?((workspace_id, mark_name))
        })
        .await
    }
}

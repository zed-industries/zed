use crate::command::command_interceptor;
use crate::normal::repeat::Replayer;
use crate::surrounds::SurroundsType;
use crate::{motion::Motion, object::Object};
use crate::{ToggleRegistersView, UseSystemClipboard, Vim, VimSettings};
use anyhow::Result;
use collections::HashMap;
use command_palette_hooks::{CommandPaletteFilter, CommandPaletteInterceptor};
use db::define_connection;
use db::sqlez_macros::sql;
use editor::display_map::{is_invisible, replacement};
use editor::{Anchor, ClipboardSelection, Editor, ExcerptId, MultiBuffer};
use gpui::{
    Action, App, AppContext, BorrowAppContext, ClipboardEntry, ClipboardItem, Entity, EntityId,
    Global, HighlightStyle, StyledText, Subscription, Task, TextStyle, WeakEntity,
};
use language::{Buffer, BufferEvent, BufferId, Point};
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectItem, ProjectPath};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::borrow::BorrowMut;
use std::path::Path;
use std::{fmt::Display, ops::Range, sync::Arc};
use text::Bias;
use theme::ThemeSettings;
use ui::{
    h_flex, rems, ActiveTheme, Context, Div, FluentBuilder, KeyBinding, ParentElement,
    SharedString, Styled, StyledTypography, Window,
};
use util::ResultExt;
use workspace::searchable::Direction;
use workspace::{Workspace, WorkspaceDb, WorkspaceId};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
    HelixNormal,
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
            Mode::HelixNormal => write!(f, "HELIX NORMAL"),
        }
    }
}

impl Mode {
    pub fn is_visual(&self) -> bool {
        match self {
            Self::Visual | Self::VisualLine | Self::VisualBlock => true,
            Self::Normal | Self::Insert | Self::Replace | Self::HelixNormal => false,
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
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
    },
    FindBackward {
        after: bool,
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
    Digraph {
        first_char: Option<char>,
    },
    Literal {
        prefix: Option<String>,
    },
    Register,
    RecordRegister,
    ReplayRegister,
    ToggleComments,
    ReplaceWithRegister,
    Exchange,
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
        // For now, we don't store metadata for multiple entries.
        match item.entries().first() {
            Some(ClipboardEntry::String(value)) if item.entries().len() == 1 => Register {
                text: value.text().to_owned().into(),
                clipboard_selections: value.metadata_json::<Vec<ClipboardSelection>>(),
            },
            // For now, registers can't store images. This could change in the future.
            _ => Register::default(),
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

    pub dot_recording: bool,
    pub dot_replaying: bool,

    /// pre_count is the number before an operator is specified (3 in 3d2d)
    pub pre_count: Option<usize>,
    /// post_count is the number after an operator is specified (2 in 3d2d)
    pub post_count: Option<usize>,

    pub stop_recording_after_next_action: bool,
    pub ignore_current_insertion: bool,
    pub recorded_count: Option<usize>,
    pub recording_actions: Vec<ReplayableAction>,
    pub recorded_actions: Vec<ReplayableAction>,
    pub recorded_selection: RecordedSelection,

    pub recording_register: Option<char>,
    pub last_recorded_register: Option<char>,
    pub last_replayed_register: Option<char>,
    pub replayer: Option<Replayer>,

    pub last_yank: Option<SharedString>,
    pub registers: HashMap<char, Register>,
    pub recordings: HashMap<char, Vec<ReplayableAction>>,

    pub focused_vim: Option<WeakEntity<Vim>>,

    pub marks: HashMap<EntityId, Entity<MarksState>>,
}

// #[derive(Clone)]
// pub enum MarksCollection {
//     Loaded {
//         marks: HashMap<String, Vec<text::Anchor>>,
//         buffer: WeakEntity<Buffer>,
//     },
//     Unloaded(HashMap<String, Vec<Point>>),
// }

// impl MarksCollection {
//     pub fn load(&mut self, entity: &Entity<Buffer>, cx: &App) {
//         let buffer = entity.read(cx);
//         let buffer_snapshot = buffer.snapshot();
//         match self {
//             MarksCollection::Unloaded(marks) => {
//                 let mut new_marks = HashMap::<String, Vec<text::Anchor>>::default();
//                 for (name, points) in marks.iter() {
//                     let anchors = points
//                         .iter()
//                         .map(|&p| {
//                             let p = buffer_snapshot.clip_point(p, editor::Bias::Left);
//                             buffer_snapshot.anchor_before(p)
//                         })
//                         .collect();
//                     new_marks.insert(name.clone(), anchors);
//                 }
//                 *self = MarksCollection::Loaded {
//                     marks: new_marks,
//                     buffer: entity.downgrade(),
//                 };
//             }
//             _ => {}
//         };
//     }
//     fn get_points_for_marks(&self, cx: &App) -> HashMap<String, Vec<Point>> {
//         match self {
//             MarksCollection::Loaded { marks, buffer } => {
//                 let mut new_marks = HashMap::<String, Vec<Point>>::default();
//                 let Some(buffer) = buffer.upgrade() else {
//                     return Default::default();
//                 };
//                 let snapshot = buffer.read(cx).snapshot();

//                 for (name, anchors) in marks.iter() {
//                     let points: Vec<Point> = anchors
//                         .iter()
//                         .map(|anchor| anchor.to_point(&snapshot))
//                         .collect();
//                     new_marks.insert(name.clone(), points);
//                 }
//                 new_marks
//             }
//             MarksCollection::Unloaded(marks) => marks.clone(),
//         }
//     }

//     pub fn unload(&mut self, cx: &App) {
//         let new_marks = self.get_points_for_marks(cx);
//         *self = MarksCollection::Unloaded(new_marks)
//     }

//     // This method is for unloading when the intenal buffer entity has been destroyed
//     pub fn unload_without_internal_buffer(&mut self, buffer: &Buffer) {
//         match self {
//             MarksCollection::Loaded { marks, buffer: _ } => {
//                 let mut new_marks = HashMap::<String, Vec<Point>>::default();
//                 let snapshot = buffer.snapshot();

//                 for (name, anchors) in marks.iter() {
//                     let points: Vec<Point> = anchors
//                         .iter()
//                         .map(|anchor| anchor.to_point(&snapshot))
//                         .collect();
//                     new_marks.insert(name.clone(), points);
//                 }
//                 *self = MarksCollection::Unloaded(new_marks);
//             }
//             _ => {}
//         }
//     }

//     pub fn add_mark_by_points(&mut self, name: String, points: Vec<Point>) {
//         match self {
//             MarksCollection::Loaded {
//                 marks: _,
//                 buffer: _,
//             } => {}
//             MarksCollection::Unloaded(marks) => {
//                 marks.insert(name, points);
//             }
//         }
//     }

//     pub fn add_mark_by_anchors(&mut self, name: String, anchors: Vec<text::Anchor>) {
//         match self {
//             MarksCollection::Loaded { marks, buffer: _ } => {
//                 marks.insert(name, anchors);
//             }
//             MarksCollection::Unloaded(_) => {}
//         }
//     }

//     pub fn get_anchors(
//         &mut self,
//         name: String,
//         buffer: &Entity<Buffer>,
//         multi_buffer: &Entity<MultiBuffer>,
//         cx: &App,
//     ) -> Option<Vec<Anchor>> {
//         self.load(buffer, cx);
//         match self {
//             MarksCollection::Loaded { marks, buffer: _ } => {
//                 let snapshot = buffer.read(cx).snapshot();
//                 let marks = marks.get(&name)?;
//                 let multi_buffer = multi_buffer.read(cx);
//                 Some(
//                     marks
//                         .iter()
//                         .flat_map(|anchor| {
//                             multi_buffer.buffer_point_to_anchor(
//                                 buffer,
//                                 anchor.to_point(&snapshot),
//                                 cx,
//                             )
//                         })
//                         .collect(),
//                 )
//             }
//             _ => None,
//         }
//     }

//     pub fn get_json(&self, name: String, cx: &App) -> Option<String> {
//         match self {
//             MarksCollection::Loaded { marks, buffer } => {
//                 let snapshot = buffer.upgrade()?.read(cx).snapshot();
//                 let anchors = marks.get(&name)?;
//                 let points: Vec<Point> = anchors
//                     .iter()
//                     .map(|anchor| anchor.to_point(&snapshot))
//                     .collect();
//                 let locations: Vec<(u32, u32)> = points
//                     .iter()
//                     .map(|point| (point.row, point.column))
//                     .collect();
//                 serde_json::to_string(&locations).ok()
//             }
//             MarksCollection::Unloaded(marks) => marks.get(&name).and_then(|points| {
//                 let locations: Vec<(u32, u32)> = points
//                     .iter()
//                     .map(|point| (point.row, point.column))
//                     .collect();
//                 serde_json::to_string(&locations).ok()
//             }),
//         }
//     }

//     pub fn write_all_to_db(&mut self, workspace_id: WorkspaceId, path: Arc<Path>, cx: &App) {
//         let marks = self.get_points_for_marks(cx);
//         for (name, points) in marks.iter() {
//             let locations: Vec<(u32, u32)> = points
//                 .iter()
//                 .map(|point| (point.row, point.column))
//                 .collect();
//             let Some(value) = serde_json::to_string(&locations).ok() else {
//                 return;
//             };
//             cx.background_executor()
//                 .spawn(DB.set_mark(
//                     workspace_id,
//                     name.clone(),
//                     path.as_os_str().as_encoded_bytes().to_vec(),
//                     value,
//                 ))
//                 .detach_and_log_err(cx);
//         }
//     }
// }

pub struct MarksState {
    workspace: WeakEntity<Workspace>,

    multibuffer_marks: HashMap<EntityId, HashMap<String, Vec<Anchor>>>,
    buffer_marks: HashMap<BufferId, HashMap<String, Vec<text::Anchor>>>,
    watched_buffers: HashMap<BufferId, (Option<Arc<Path>>, Subscription, Subscription)>,

    serialized_marks: HashMap<Arc<Path>, HashMap<String, Vec<Point>>>,
    global_marks: HashMap<String, MarkLocation>,

    _subscription: Subscription,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarkLocation {
    MultiBuffer(EntityId),
    Buffer(BufferId),
    Path(Arc<Path>),
}

pub enum Mark {
    Local(Vec<Anchor>),
    MultiBuffer(EntityId, Vec<Anchor>),
    Buffer(BufferId, Vec<Anchor>), // for singleton buffers with no file
    Path(Arc<Path>, Vec<Point>),
}

impl MarksState {
    pub fn new(workspace: &Workspace, cx: &mut App) -> Entity<MarksState> {
        cx.new(|cx| {
            let buffer_store = workspace.project().read(cx).buffer_store().clone();
            let subscription =
                cx.subscribe(
                    &buffer_store,
                    move |this: &mut Self, _, event, cx| match event {
                        project::buffer_store::BufferStoreEvent::BufferAdded(buffer) => {
                            this.on_buffer_loaded(buffer, cx);
                        }
                        _ => {}
                    },
                );

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
        cx.spawn(|this, mut cx| async move {
            let Some(workspace_id) = this.update(&mut cx, |this, cx| this.workspace_id(cx))? else {
                return Ok(());
            };
            let (marks, paths) = cx
                .background_spawn(async move {
                    let marks = DB.get_marks(workspace_id)?;
                    let paths = DB.get_global_marks_paths(workspace_id)?;
                    anyhow::Ok((marks, paths))
                })
                .await?;
            this.update(&mut cx, |this, cx| this.loaded(marks, paths, cx))
        })
        .detach_and_log_err(cx);
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
                    Some(ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: relative.into(),
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
            dbg!("loaded", &name, &points, &buffer.text());
            loaded_marks.insert(
                name.clone(),
                points
                    .iter()
                    .map(|point| buffer.anchor_before(buffer.clip_point(*point, Bias::Left)))
                    .collect(),
            );
        }
        self.buffer_marks.insert(buffer.remote_id(), loaded_marks);
        self.watch_buffer(Some(abs_path), buffer_handle, cx)
    }

    fn serialize_buffer_marks(
        &mut self,
        path: Arc<Path>,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let new_points = if let Some(anchors) = self.buffer_marks.get(&buffer.read(cx).remote_id())
        {
            anchors
                .iter()
                .map(|(name, anchors)| {
                    (
                        name.clone(),
                        buffer
                            .read(cx)
                            .summaries_for_anchors::<Point, _>(anchors)
                            .collect(),
                    )
                })
                .collect()
        } else {
            HashMap::default()
        };
        let old_points = self
            .serialized_marks
            .get(&path)
            .cloned()
            .unwrap_or_default();
        if old_points == new_points {
            return;
        }
        dbg!(std::backtrace::Backtrace::force_capture());
        dbg!(&old_points, &new_points);

        for (key, _) in &new_points {
            if self.is_global_mark(key) {
                if self.global_marks.get(key) != Some(&MarkLocation::Path(path.clone())) {
                    if let Some(workspace_id) = self.workspace_id(cx) {
                        let path = path.clone();
                        let key = key.clone();
                        cx.background_spawn(async move {
                            DB.set_global_mark_path(workspace_id, key, path).await
                        })
                        .detach_and_log_err(cx);
                    }

                    self.global_marks
                        .insert(key.clone(), MarkLocation::Path(path.clone()));
                }
            }
        }

        self.serialized_marks
            .insert(path.clone(), new_points.clone());

        if let Some(workspace_id) = self.workspace_id(cx) {
            cx.background_spawn(async move {
                for (key, value) in new_points {
                    DB.set_mark(workspace_id, path.clone(), key, value).await?;
                }
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
        old_path: Option<Arc<Path>>,
        new_path: Arc<Path>,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        if let Some(old_path) = old_path {
            self.serialized_marks.remove(&old_path);
        }
        // if there is no old path we assume that
        // we are giving a previously unnamed buffer a name
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
            MarkLocation::MultiBuffer(entity_id) => entity_id == &multi_buffer.entity_id(),
            MarkLocation::Buffer(buffer_id) => {
                let Some(buffer) = multi_buffer.read(cx).as_singleton() else {
                    return false;
                };
                buffer_id == &buffer.read(cx).remote_id()
            }
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
        abs_path: Option<Arc<Path>>,
        buffer_handle: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let on_change = cx.subscribe(buffer_handle, move |this, buffer, event, cx| match event {
            BufferEvent::Edited => {
                if let Some(path) = this.path_for_buffer(&buffer, cx) {
                    dbg!(&buffer.read(cx).text());
                    this.serialize_buffer_marks(path, &buffer, cx);
                }
            }
            BufferEvent::FileHandleChanged => {
                let buffer_id = buffer.read(cx).remote_id();
                if let Some(old_path) = this
                    .watched_buffers
                    .get(&buffer_id.clone())
                    .map(|(path, _, _)| path.clone())
                {
                    if let Some(new_path) = this.path_for_buffer(&buffer, cx) {
                        this.rename_buffer(old_path, new_path, &buffer, cx)
                    }
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
            (abs_path, on_change, on_release),
        );
    }

    pub fn set_mark(
        &mut self,
        name: String,
        buffer_handle: &Entity<MultiBuffer>,
        anchors: Vec<Anchor>,
        cx: &mut Context<Self>,
    ) {
        let Some(buffer_handle) = buffer_handle.read(cx).as_singleton() else {
            self.multibuffer_marks
                .entry(buffer_handle.entity_id())
                .or_default()
                .insert(name.clone(), anchors);
            if self.is_global_mark(&name) {
                self.global_marks.insert(
                    name.clone(),
                    MarkLocation::MultiBuffer(buffer_handle.entity_id()),
                );
            }
            return;
        };

        let buffer_id = buffer_handle.read(cx).remote_id();
        dbg!(
            "set_mark",
            anchors
                .iter()
                .map(|anchor| anchor.text_anchor)
                .collect::<Vec<_>>()
        );
        self.buffer_marks
            .entry(buffer_id.clone())
            .or_default()
            .insert(
                name.clone(),
                anchors
                    .into_iter()
                    .map(|anchor| anchor.text_anchor)
                    .collect(),
            );
        self.global_marks.insert(
            // this will be overwritten by serialize if there is an associated file
            name.clone(),
            MarkLocation::Buffer(buffer_id),
        );
        let abs_path = self.path_for_buffer(&buffer_handle, cx);
        if !self.watched_buffers.contains_key(&buffer_id) {
            self.watch_buffer(abs_path.clone(), &buffer_handle, cx)
        }
        if let Some(abs_path) = abs_path {
            self.serialize_buffer_marks(abs_path, &buffer_handle, cx)
        }
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

            let singleton = multi_buffer.read(cx).as_singleton()?;
            let excerpt_id = multi_buffer.read(cx).excerpt_ids().first().unwrap().clone();
            let buffer_id = singleton.read(cx).remote_id();
            if let Some(anchors) = self.buffer_marks.get(&buffer_id) {
                let text_anchors = anchors.get(name)?;
                let anchors = text_anchors
                    .into_iter()
                    .map(|anchor| Anchor::in_buffer(excerpt_id, buffer_id, anchor.clone()))
                    .collect();
                return Some(Mark::Local(anchors));
            }
        }

        match target? {
            MarkLocation::MultiBuffer(entity_id) => {
                let anchors = self.multibuffer_marks.get(&entity_id)?;
                return Some(Mark::MultiBuffer(*entity_id, anchors.get(name)?.clone()));
            }
            MarkLocation::Buffer(buffer_id) => {
                let text_anchors = self.buffer_marks.get(&buffer_id)?.get(name)?;
                let anchors: Vec<_> = text_anchors
                    .into_iter()
                    .map(|anchor| Anchor::in_buffer(ExcerptId::min(), *buffer_id, anchor.clone()))
                    .collect();
                return Some(Mark::Buffer(*buffer_id, anchors));
            }
            MarkLocation::Path(path) => {
                let points = self.serialized_marks.get(path)?;
                return Some(Mark::Path(path.clone(), points.get(name)?.clone()));
            }
        }
    }
}

impl Global for VimGlobals {}

impl VimGlobals {
    pub(crate) fn register(cx: &mut App) {
        cx.set_global(VimGlobals::default());

        cx.observe_keystrokes(|event, _, cx| {
            let Some(action) = event.action.as_ref().map(|action| action.boxed_clone()) else {
                return;
            };
            Vim::globals(cx).observe_action(action.boxed_clone())
        })
        .detach();

        cx.observe_new(|workspace: &mut Workspace, window, _| {
            RegistersView::register(workspace, window);
        })
        .detach();

        cx.observe_global::<SettingsStore>(move |cx| {
            if Vim::enabled(cx) {
                KeyBinding::set_vim_mode(cx, true);
                CommandPaletteFilter::update_global(cx, |filter, _| {
                    filter.show_namespace(Vim::NAMESPACE);
                });
                CommandPaletteInterceptor::update_global(cx, |interceptor, _| {
                    interceptor.set(Box::new(command_interceptor));
                });
            } else {
                KeyBinding::set_vim_mode(cx, false);
                *Vim::globals(cx) = VimGlobals::default();
                CommandPaletteInterceptor::update_global(cx, |interceptor, _| {
                    interceptor.clear();
                });
                CommandPaletteFilter::update_global(cx, |filter, _| {
                    filter.hide_namespace(Vim::NAMESPACE);
                });
            }
        })
        .detach();
        cx.observe_new(|workspace: &mut Workspace, _, cx| {
            let entity_id = cx.entity_id();
            Vim::update_globals(cx, |globals, cx| {
                globals
                    .marks
                    .insert(entity_id, MarksState::new(workspace, cx))
            });
            cx.observe_release(
                &workspace.weak_handle().upgrade().unwrap(),
                move |_, _, cx| {
                    Vim::update_globals(cx, |globals, _| {
                        globals.marks.remove(&entity_id);
                    })
                },
            )
            .detach();
        })
        .detach()
    }

    pub(crate) fn write_registers(
        &mut self,
        content: Register,
        register: Option<char>,
        is_yank: bool,
        linewise: bool,
        cx: &mut Context<Editor>,
    ) {
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
                        self.registers.insert('"', content.clone());
                        cx.write_to_clipboard(content.into());
                    }
                    '*' => {
                        self.registers.insert('"', content.clone());
                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                        cx.write_to_primary(content.into());
                        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                        cx.write_to_clipboard(content.into());
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
            let setting = VimSettings::get_global(cx).use_system_clipboard;
            if setting == UseSystemClipboard::Always
                || setting == UseSystemClipboard::OnYank && is_yank
            {
                self.last_yank.replace(content.text.clone());
                cx.write_to_clipboard(content.clone().into());
            } else {
                self.last_yank = cx
                    .read_from_clipboard()
                    .and_then(|item| item.text().map(|string| string.into()));
            }

            self.registers.insert('"', content.clone());
            if is_yank {
                self.registers.insert('0', content);
            } else {
                let contains_newline = content.text.contains('\n');
                if !contains_newline {
                    self.registers.insert('-', content.clone());
                }
                if linewise || contains_newline {
                    let mut content = content;
                    for i in '1'..'8' {
                        if let Some(moved) = self.registers.insert(i, content) {
                            content = moved;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn read_register(
        &self,
        register: Option<char>,
        editor: Option<&mut Editor>,
        cx: &mut App,
    ) -> Option<Register> {
        let Some(register) = register.filter(|reg| *reg != '"') else {
            let setting = VimSettings::get_global(cx).use_system_clipboard;
            return match setting {
                UseSystemClipboard::Always => cx.read_from_clipboard().map(|item| item.into()),
                UseSystemClipboard::OnYank if self.system_clipboard_is_newer(cx) => {
                    cx.read_from_clipboard().map(|item| item.into())
                }
                _ => self.registers.get(&'"').cloned(),
            };
        };
        let lower = register.to_lowercase().next().unwrap_or(register);
        match lower {
            '_' | ':' | '.' | '#' | '=' => None,
            '+' => cx.read_from_clipboard().map(|item| item.into()),
            '*' => {
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                {
                    cx.read_from_primary().map(|item| item.into())
                }
                #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                {
                    cx.read_from_clipboard().map(|item| item.into())
                }
            }
            '%' => editor.and_then(|editor| {
                let selection = editor.selections.newest::<Point>(cx);
                if let Some((_, buffer, _)) = editor
                    .buffer()
                    .read(cx)
                    .excerpt_containing(selection.head(), cx)
                {
                    buffer
                        .read(cx)
                        .file()
                        .map(|file| file.path().to_string_lossy().to_string().into())
                } else {
                    None
                }
            }),
            _ => self.registers.get(&lower).cloned(),
        }
    }

    fn system_clipboard_is_newer(&self, cx: &App) -> bool {
        cx.read_from_clipboard().is_some_and(|item| {
            if let Some(last_state) = &self.last_yank {
                Some(last_state.as_ref()) != item.text().as_deref()
            } else {
                true
            }
        })
    }

    pub fn observe_action(&mut self, action: Box<dyn Action>) {
        if self.dot_recording {
            self.recording_actions
                .push(ReplayableAction::Action(action.boxed_clone()));

            if self.stop_recording_after_next_action {
                self.dot_recording = false;
                self.recorded_actions = std::mem::take(&mut self.recording_actions);
                self.stop_recording_after_next_action = false;
            }
        }
        if self.replayer.is_none() {
            if let Some(recording_register) = self.recording_register {
                self.recordings
                    .entry(recording_register)
                    .or_default()
                    .push(ReplayableAction::Action(action));
            }
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
                self.dot_recording = false;
                self.recorded_actions = std::mem::take(&mut self.recording_actions);
                self.stop_recording_after_next_action = false;
            }
        }
        if let Some(recording_register) = self.recording_register {
            self.recordings.entry(recording_register).or_default().push(
                ReplayableAction::Insertion {
                    text: text.clone(),
                    utf16_range_to_replace: range_to_replace,
                },
            );
        }
    }

    pub fn focused_vim(&self) -> Option<Entity<Vim>> {
        self.focused_vim.as_ref().and_then(|vim| vim.upgrade())
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

#[derive(Debug)]
pub enum ReplayableAction {
    Action(Box<dyn Action>),
    Insertion {
        text: Arc<str>,
        utf16_range_to_replace: Option<Range<isize>>,
    },
}

impl Clone for ReplayableAction {
    fn clone(&self) -> Self {
        match self {
            Self::Action(action) => Self::Action(action.boxed_clone()),
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

#[derive(Clone, Default, Debug)]
pub struct SearchState {
    pub direction: Direction,
    pub count: usize,

    pub prior_selections: Vec<Range<Anchor>>,
    pub prior_operator: Option<Operator>,
    pub prior_mode: Mode,
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
            Operator::FindForward { before: false } => "f",
            Operator::FindForward { before: true } => "t",
            Operator::Sneak { .. } => "s",
            Operator::SneakBackward { .. } => "S",
            Operator::FindBackward { after: false } => "F",
            Operator::FindBackward { after: true } => "T",
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
            Operator::ReplaceWithRegister => "gr",
            Operator::Exchange => "cx",
            Operator::Outdent => "<",
            Operator::Uppercase => "gU",
            Operator::Lowercase => "gu",
            Operator::OppositeCase => "g~",
            Operator::Register => "\"",
            Operator::RecordRegister => "q",
            Operator::ReplayRegister => "@",
            Operator::ToggleComments => "gc",
        }
    }

    pub fn status(&self) -> String {
        match self {
            Operator::Digraph {
                first_char: Some(first_char),
            } => format!("^K{first_char}"),
            Operator::Literal {
                prefix: Some(prefix),
            } => format!("^V{prefix}"),
            Operator::AutoIndent => "=".to_string(),
            Operator::ShellCommand => "=".to_string(),
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
            | Operator::Register
            | Operator::RecordRegister
            | Operator::ReplayRegister
            | Operator::Replace
            | Operator::Digraph { .. }
            | Operator::Literal { .. }
            | Operator::ChangeSurrounds { target: Some(_) }
            | Operator::DeleteSurrounds => true,
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
            | Operator::ReplaceWithRegister
            | Operator::Exchange
            | Operator::Object { .. }
            | Operator::ChangeSurrounds { target: None }
            | Operator::OppositeCase
            | Operator::ToggleComments => false,
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
            | Operator::ToggleComments
            | Operator::ReplaceWithRegister
            | Operator::Rewrap
            | Operator::ShellCommand
            | Operator::AddSurrounds { target: None }
            | Operator::ChangeSurrounds { target: None }
            | Operator::DeleteSurrounds
            | Operator::Exchange => true,
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
            | Operator::Register
            | Operator::RecordRegister
            | Operator::ReplayRegister => false,
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
        let register_match = self
            .matches
            .get(ix)
            .expect("Invalid matches state: no element for index {ix}");

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
                if let Some(register) = globals.read_register(Some(name), None, cx) {
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
                if let Some(register) = register {
                    matches.push(RegisterMatch {
                        name: '%',
                        contents: register.text.clone(),
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
        matches.sort_by(|a, b| a.name.cmp(&b.name));
        let delegate = RegistersViewDelegate {
            selected_index: 0,
            matches,
        };

        Picker::nonsearchable_uniform_list(delegate, window, cx)
            .width(rems(36.))
            .modal(true)
    }
}

define_connection! (
    pub static ref DB: VimDb<WorkspaceDb> = &[
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
);

struct SerializedMark {
    path: Arc<Path>,
    name: String,
    points: Vec<Point>,
}

impl VimDb {
    pub(crate) async fn set_mark(
        &self,
        workspace_id: WorkspaceId,
        path: Arc<Path>,
        mark_name: String,
        value: Vec<Point>,
    ) -> Result<()> {
        let pairs: Vec<(u32, u32)> = value
            .into_iter()
            .map(|point| (point.row, point.column))
            .collect();
        let serialized = serde_json::to_string(&pairs)?;
        dbg!(&path, &mark_name, &serialized);
        self.write(move |conn| {
            conn.exec_bound(sql!(
                INSERT OR REPLACE INTO vim_marks
                    (workspace_id, mark_name, path, value)
                VALUES
                    (?, ?, ?, ?)
            ))?((workspace_id, mark_name, path, serialized))
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

    pub(crate) async fn set_global_mark_path(
        &self,
        workspace_id: WorkspaceId,
        mark_name: String,
        path: Arc<Path>,
    ) -> Result<()> {
        dbg!(&mark_name, &path);
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
}

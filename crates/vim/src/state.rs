use crate::command::command_interceptor;
use crate::motion::MotionKind;
use crate::normal::repeat::Replayer;
use crate::surrounds::SurroundsType;
use crate::{ToggleMarksView, ToggleRegistersView, UseSystemClipboard, Vim, VimAddon, VimSettings};
use crate::{motion::Motion, object::Object};
use anyhow::Result;
use collections::HashMap;
use command_palette_hooks::{CommandPaletteFilter, CommandPaletteInterceptor};
use db::{
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use editor::display_map::{is_invisible, replacement};
use editor::{Anchor, ClipboardSelection, Editor, MultiBuffer, ToPoint as EditorToPoint};
use gpui::{
    Action, App, AppContext, BorrowAppContext, ClipboardEntry, ClipboardItem, DismissEvent, Entity,
    EntityId, Global, HighlightStyle, StyledText, Subscription, Task, TextStyle, WeakEntity,
};
use language::{Buffer, BufferEvent, BufferId, Chunk, Point};
use multi_buffer::MultiBufferRow;
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectItem, ProjectPath};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::path::Path;
use std::{fmt::Display, ops::Range, sync::Arc};
use text::{Bias, ToPoint};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Context, Div, FluentBuilder, KeyBinding, ParentElement, SharedString, Styled,
    StyledTypography, Window, h_flex, rems,
};
use util::ResultExt;
use util::rel_path::RelPath;
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
    Register,
    RecordRegister,
    ReplayRegister,
    ToggleComments,
    ReplaceWithRegister,
    Exchange,
    HelixMatch,
    HelixNext {
        around: bool,
    },
    HelixPrevious {
        around: bool,
    },
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
    pub forced_motion: bool,
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
            let Some(workspace_id) = this.update(cx, |this, cx| this.workspace_id(cx))? else {
                return Ok(());
            };
            let (marks, paths) = cx
                .background_spawn(async move {
                    let marks = DB.get_marks(workspace_id)?;
                    let paths = DB.get_global_marks_paths(workspace_id)?;
                    anyhow::Ok((marks, paths))
                })
                .await?;
            this.update(cx, |this, cx| this.loaded(marks, paths, cx))
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
        let new_points: HashMap<String, Vec<Point>> =
            if let Some(anchors) = self.buffer_marks.get(&buffer.read(cx).remote_id()) {
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
                    cx.background_spawn(async move {
                        DB.set_global_mark_path(workspace_id, key, path).await
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
            cx.background_spawn(async move {
                DB.set_marks(workspace_id, path.clone(), to_write).await?;
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
                .map(|(k, v)| (k, v.into_iter().map(|anchor| anchor.text_anchor).collect()))
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
            BufferEvent::Edited => {
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
        let buffer = multibuffer.read(cx).as_singleton();
        let abs_path = buffer.as_ref().and_then(|b| self.path_for_buffer(b, cx));

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
            name,
            anchors
                .into_iter()
                .map(|anchor| anchor.text_anchor)
                .collect(),
        );
        if !self.watched_buffers.contains_key(&buffer_id) {
            self.watch_buffer(MarkLocation::Path(abs_path.clone()), &buffer, cx)
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

            let singleton = multi_buffer.read(cx).as_singleton()?;
            let excerpt_id = *multi_buffer.read(cx).excerpt_ids().first()?;
            let buffer_id = singleton.read(cx).remote_id();
            if let Some(anchors) = self.buffer_marks.get(&buffer_id) {
                let text_anchors = anchors.get(name)?;
                let anchors = text_anchors
                    .iter()
                    .map(|anchor| Anchor::in_buffer(excerpt_id, buffer_id, *anchor))
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
                cx.background_spawn(async move {
                    DB.delete_global_marks_path(workspace_id, name).await
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
            cx.background_spawn(async move { DB.delete_mark(workspace_id, path, mark_name).await })
                .detach_and_log_err(cx);
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
                CommandPaletteInterceptor::update_global(cx, |interceptor, _| {
                    interceptor.set(Box::new(command_interceptor));
                });
                for window in cx.windows() {
                    if let Some(workspace) = window.downcast::<Workspace>() {
                        workspace
                            .update(cx, |workspace, _, cx| {
                                Vim::update_globals(cx, |globals, cx| {
                                    globals.register_workspace(workspace, cx)
                                });
                            })
                            .ok();
                    }
                }
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

    pub(crate) fn write_registers(
        &mut self,
        content: Register,
        register: Option<char>,
        is_yank: bool,
        kind: MotionKind,
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
                        .map(|file| file.path().display(file.path_style(cx)).into_owned().into())
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
        if self.replayer.is_none()
            && let Some(recording_register) = self.recording_register
        {
            self.recordings
                .entry(recording_register)
                .or_default()
                .push(ReplayableAction::Action(action));
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
    pub helix_select: bool,
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
            Operator::Register => "\"",
            Operator::RecordRegister => "q",
            Operator::ReplayRegister => "@",
            Operator::ToggleComments => "gc",
            Operator::HelixMatch => "helix_m",
            Operator::HelixNext { .. } => "helix_next",
            Operator::HelixPrevious { .. } => "helix_previous",
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
            | Operator::Rot13
            | Operator::Rot47
            | Operator::ReplaceWithRegister
            | Operator::Exchange
            | Operator::Object { .. }
            | Operator::ChangeSurrounds { target: None }
            | Operator::OppositeCase
            | Operator::ToggleComments
            | Operator::HelixMatch
            | Operator::HelixNext { .. }
            | Operator::HelixPrevious { .. } => false,
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
            | Operator::ReplaceWithRegister
            | Operator::Rewrap
            | Operator::ShellCommand
            | Operator::AddSurrounds { target: None }
            | Operator::ChangeSurrounds { target: None }
            | Operator::DeleteSurrounds
            | Operator::Exchange
            | Operator::HelixNext { .. }
            | Operator::HelixPrevious { .. } => true,
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
            | Operator::ReplayRegister
            | Operator::HelixMatch => false,
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
            if let Some(highlight_style) = chunk.syntax_highlight_id
                && let Some(highlight) = highlight_style.style(cx.theme().syntax())
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
                            true,
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
                                true,
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
                let s = path.to_string_lossy().to_string();
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
        Picker::nonsearchable_uniform_list(delegate, window, cx)
            .width(rems(36.))
            .modal(true)
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

db::static_connection!(DB, VimDb, [WorkspaceDb]);

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

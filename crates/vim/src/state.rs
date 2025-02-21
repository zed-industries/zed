use crate::command::command_interceptor;
use crate::normal::repeat::Replayer;
use crate::surrounds::SurroundsType;
use crate::{motion::Motion, object::Object};
use crate::{UseSystemClipboard, Vim, VimSettings};
use anyhow::Result;
use collections::HashMap;
use command_palette_hooks::{CommandPaletteFilter, CommandPaletteInterceptor};
use db::sqlez_macros::sql;
use db::{define_connection, query};
use editor::{Anchor, ClipboardSelection, Editor, ExcerptId, MultiBuffer};
use gpui::{
    Action, App, AppContext, BorrowAppContext, ClipboardEntry, ClipboardItem, Entity, Global,
    WeakEntity,
};
use language::{Buffer, BufferEvent, Point, ToOffset, ToPoint};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::borrow::BorrowMut;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::{fmt::Display, ops::Range, sync::Arc};
use ui::{Context, KeyBinding, SharedString};
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

    pub marks: HashMap<WorkspaceId, Entity<MarksState>>,
}

#[derive(Clone)]
enum MarksCollection {
    Loaded {
        marks: HashMap<String, Vec<Anchor>>,
        buffer: WeakEntity<Buffer>,
    },
    Unloaded(HashMap<String, Vec<Point>>),
}

impl MarksCollection {
    pub fn load(&mut self, entity: &Entity<Buffer>, multi_buffer: &Entity<MultiBuffer>, cx: &App) {
        let buffer = entity.read(cx);
        let buffer_snapshot = buffer.snapshot();
        let multi_buffer = multi_buffer.read(cx);
        let multi_buffer_snapshot = multi_buffer.snapshot(cx);
        match self {
            MarksCollection::Unloaded(marks) => {
                let mut new_marks = HashMap::<String, Vec<Anchor>>::default();
                for (name, points) in marks.iter() {
                    println!("LOAD| name: {}, points: {:?}", name.clone(), points.clone());
                    let anchors = points
                        .iter()
                        .map(|&p| {
                            multi_buffer_snapshot.anchor_after(p.to_offset(&buffer_snapshot))

                            // Anchor::in_buffer(ExcerptId::min(), id, snapshot.anchor_before(p))
                        })
                        .collect();
                    new_marks.insert(name.clone(), anchors);
                }
                *self = MarksCollection::Loaded {
                    marks: new_marks,
                    buffer: entity.downgrade(),
                };
            }
            _ => {}
        };
    }
    pub fn unload(&mut self, cx: &App) {
        match self {
            MarksCollection::Loaded { marks, buffer } => {
                let mut new_marks = HashMap::<String, Vec<Point>>::default();
                let Some(buffer) = buffer.upgrade() else {
                    return;
                };
                let snapshot = buffer.read(cx).snapshot();

                for (name, anchors) in marks.iter() {
                    let points: Vec<Point> = anchors
                        .iter()
                        .map(|anchor| anchor.text_anchor.to_point(&snapshot))
                        .collect();
                    new_marks.insert(name.clone(), points);
                }

                *self = MarksCollection::Unloaded(new_marks)
            }
            _ => {}
        };
    }

    pub fn add_mark_by_points(&mut self, name: String, points: Vec<Point>) {
        match self {
            MarksCollection::Loaded { marks, buffer } => todo!(),
            MarksCollection::Unloaded(marks) => {
                marks.insert(name, points);
            }
        }
    }

    pub fn add_mark_by_anchors(&mut self, name: String, anchors: Vec<Anchor>) {
        match self {
            MarksCollection::Loaded { marks, buffer } => {
                marks.insert(name, anchors);
            }
            MarksCollection::Unloaded(marks) => todo!(),
        }
    }

    pub fn get_anchors(
        &mut self,
        name: String,
        buffer: &Entity<Buffer>,
        multi_buffer: &Entity<MultiBuffer>,
        cx: &App,
    ) -> Option<Vec<Anchor>> {
        self.load(buffer, multi_buffer, cx);
        match self {
            MarksCollection::Loaded { marks, buffer } => marks.get(&name).cloned(),
            _ => None,
        }
    }

    pub fn get_json(&self, name: String, cx: &App) -> Option<String> {
        match self {
            MarksCollection::Loaded { marks, buffer } => {
                let snapshot = buffer.upgrade()?.read(cx).snapshot();
                let anchors = marks.get(&name)?;
                let points: Vec<Point> = anchors
                    .iter()
                    .map(|anchor| anchor.text_anchor.to_point(&snapshot))
                    .collect();
                let locations: Vec<(u32, u32)> = points
                    .iter()
                    .map(|point| (point.row, point.column))
                    .collect();
                serde_json::to_string(&locations).ok()
            }
            MarksCollection::Unloaded(marks) => marks.get(&name).and_then(|points| {
                let locations: Vec<(u32, u32)> = points
                    .iter()
                    .map(|point| (point.row, point.column))
                    .collect();
                serde_json::to_string(&locations).ok()
            }),
        }
    }
}

#[derive(Clone)]
pub struct MarksState {
    pub workspace_id: WorkspaceId,
    // this allows for buffers that are not files to have global marks and for storing anchors in general
    // pub loaded_marks: HashMap<BufferId, HashMap<String, Vec<Anchor>>>,
    // pub path_buf_id: HashMap<Arc<Path>, BufferId>,
    // pub marks: HashMap<Arc<Path>, HashMap<String, Vec<Point>>>,
    pub marks: HashMap<Arc<Path>, MarksCollection>,
    pub global_marks: HashMap<String, Arc<Path>>,
}

impl MarksState {
    pub fn new(workspace_id: WorkspaceId, cx: &mut App) -> Entity<MarksState> {
        cx.new(|_| Self {
            workspace_id,
            // loaded_marks: HashMap::default(),
            // path_buf_id: HashMap::default(),
            marks: HashMap::default(),
            global_marks: HashMap::default(),
        })
    }

    pub fn load(
        &mut self,
        workspace_id: WorkspaceId,
        marks: Vec<(Vec<u8>, String, String)>,
        global_mark_paths: Vec<(Vec<u8>, String)>,
        cx: &App,
    ) {
        self.workspace_id = workspace_id;
        for (path, name, values) in marks {
            let Some(value) = serde_json::from_str::<Vec<(u32, u32)>>(&values).log_err() else {
                continue;
            };
            let path = Arc::from(PathBuf::from(OsString::from_vec(path)));
            println!(
                "wid {:?}, path {:?}, name {}, values {}",
                workspace_id.clone(),
                path,
                name,
                values
            );

            let points: Vec<Point> = value
                .into_iter()
                .map(|(row, col)| Point::new(row, col))
                .collect();
            if let Some(marks_collection) = self.marks.get_mut(&path) {
                marks_collection.unload(cx);
                marks_collection.add_mark_by_points(name, points);
            } else {
                let mut marks = HashMap::<String, Vec<Point>>::default();
                marks.insert(name, points);
                let marks_collection = MarksCollection::Unloaded(marks);
                self.marks.insert(path, marks_collection);
            }
        }

        for (path, name) in global_mark_paths {
            let path: Arc<Path> = Arc::from(PathBuf::from(OsString::from_vec(path)));
            self.global_marks.insert(name, path);
        }
    }

    pub fn on_buffer_loaded(
        &mut self,
        buffer_handle: &Entity<Buffer>,
        // multi_buffer_handle: &Entity<MultiBuffer>,
        cx: &mut Context<Self>,
    ) {
        let buffer = buffer_handle.read(cx);
        let id = buffer.remote_id();
        let Some(path) = buffer.file().map(|file| file.path().clone()) else {
            return;
        };

        let Some(marks_collection) = self.marks.get_mut(&path) else {
            return;
        };

        // marks_collection.load(buffer_handle, multi_buffer_handle, cx);

        cx.subscribe(buffer_handle, move |this, buffer, event, cx| {
            match event {
                BufferEvent::Edited => {
                    // if let Some(m) = this.loaded_marks.get(&id) {
                    //     let mut updates = Vec::new();

                    //     for (name, anchors) in m.iter() {
                    //         let points_from_anchors: Vec<Point> = anchors
                    //             .iter()
                    //             .map(|anchor| anchor.text_anchor.to_point(&snapshot))
                    //             .collect();

                    //         if let Some(map) = this.marks.get(&path) {
                    //             if let Some(old_points) = map.get(name) {
                    //                 if points_from_anchors
                    //                     .iter()
                    //                     .zip(old_points.iter())
                    //                     .any(|(&p1, &p2)| p1 != p2)
                    //                 {
                    //                     updates.push((name.clone(), anchors.clone()));
                    //                 }
                    //             }
                    //         }
                    //     }
                    //     println!("here");
                    //     for (name, anchors) in updates {
                    //         println!("updating: {}", name.clone());
                    //         this.set_mark(name, &buffer, anchors, cx);
                    //     }
                    // }
                    // recalculate marks for this buffer, and if they've changed update SQLite
                }
                // BufferEvent::Operation { operation, is_local } => todo!(),
                // BufferEvent::DirtyChanged => todo!(),
                // BufferEvent::Saved => todo!(),
                BufferEvent::FileHandleChanged => {
                    // I am not sure how to see what the name was previously in order to remove those marks
                }
                // BufferEvent::Reloaded => todo!(),
                // BufferEvent::ReloadNeeded => todo!(),
                // BufferEvent::LanguageChanged => todo!(),
                // BufferEvent::Reparsed => todo!(),
                // BufferEvent::DiagnosticsUpdated => todo!(),
                // BufferEvent::CapabilityChanged => todo!(),
                BufferEvent::Closed => {}
                // BufferEvent::Discarded => todo!(),
                _ => {}
            }
        });
        // cx.observe_release(buffer_handle, |buffer, cx| self.loaded_marks.remove(&id))
        // I think ^ is covered by BufferEvent::Closed event
    }

    pub fn set_mark(
        &mut self,
        name: String,
        buffer_handle: &Entity<Buffer>,
        multi_buffer_handle: &Entity<MultiBuffer>,
        anchors: Vec<Anchor>,
        cx: &mut Context<Self>,
    ) {
        // the catch here is we may need to subscribe to the buffer.

        let Some(path) = buffer_handle
            .read(cx)
            .file()
            .map(|file| file.path().clone())
        else {
            return;
        };

        let Some(marks_collection) = self.marks.get_mut(&path) else {
            return;
        };

        marks_collection.load(buffer_handle, multi_buffer_handle, cx);

        if name.starts_with(|c: char| c.is_uppercase())
            || name.starts_with(|c: char| c.is_digit(10))
        {
            self.global_marks.insert(name.clone(), path.clone());
            cx.background_executor()
                .spawn(DB.set_global_mark_path(
                    self.workspace_id,
                    name.clone(),
                    path.to_path_buf().into_os_string().into_vec(),
                ))
                .detach_and_log_err(cx);
        }

        marks_collection.add_mark_by_anchors(name.clone(), anchors);

        let Some(value) = marks_collection.get_json(name.clone(), cx) else {
            return;
        };

        println!(
            "wid: {:?}, name: {}, path: {:?}, json: {}",
            self.workspace_id.clone(),
            name.clone(),
            path.clone(),
            value.clone()
        );

        cx.background_executor()
            .spawn(DB.set_mark(
                self.workspace_id,
                name,
                path.to_path_buf().into_os_string().into_vec(),
                value,
            ))
            .detach_and_log_err(cx);
    }

    pub fn get_path_for_mark(&self, name: String) -> Option<Arc<Path>> {
        self.global_marks.get(&name).cloned()
    }

    pub fn get_mark(
        &mut self,
        name: String,
        entity: &Entity<Buffer>,
        multi_buffer: &Entity<MultiBuffer>,
        cx: &App,
    ) -> Option<Vec<Anchor>> {
        let path: Arc<Path> = if name.starts_with(|c: char| c.is_uppercase())
            || name.starts_with(|c: char| c.is_digit(10))
        {
            self.get_path_for_mark(name.clone())?
        } else {
            let buffer = entity.read(cx);
            entity.read(cx).file().map(|file| file.path().clone())?
        };
        self.marks
            .get_mut(&path)?
            .get_anchors(name.clone(), entity, multi_buffer, cx)
    }
}

// enum MarkCollection {
//     Loaded {
//         anchor: HashMap<String, Vec<Anchor>>,
//         buffer: WeakEntity<Buffer>,
//         subscription: Subscription,
//     },
//     Unloaded(HashMap<String, Vec<Point>>),
// }

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
            let Some(workspace_id) = workspace.database_id() else {
                return;
            };

            cx.spawn(|_, cx| async move {
                let marks = cx
                    .background_executor()
                    .spawn(async move { DB.get_marks(workspace_id) })
                    .await?;
                let global_marks_paths = cx
                    .background_executor()
                    .spawn(async move { DB.get_global_marks_paths(workspace_id) })
                    .await?;
                cx.update_global(|g: &mut VimGlobals, cx: &mut App| {
                    g.marks
                        .insert(workspace_id, MarksState::new(workspace_id.clone(), cx));
                    if let Some(marks_state) = g.marks.get(&workspace_id) {
                        marks_state.update(cx, |ms, cx| {
                            ms.load(workspace_id, marks, global_marks_paths, cx);
                        });
                    }
                    // g.load_local_marks(workspace_id, local_marks);
                    // g.load_global_marks(workspace_id, global_marks);
                })
            })
            .detach_and_log_err(cx);

            let buffer_store = workspace.project().read(cx).buffer_store().clone();
            cx.subscribe(&buffer_store, move |this, _, event, cx| {
                match event {
                    project::buffer_store::BufferStoreEvent::BufferAdded(buffer) => {
                        // if we have marks for this buffer, upgrade to anchors,
                        // watch for changes, and when the buffer is closed, convert back
                        Vim::update_globals(cx, |globals, cx| {
                            if let Some(marks_state) = globals.marks.get(&workspace_id) {
                                marks_state.update(cx, |ms, cx| {
                                    ms.on_buffer_loaded(buffer, cx);
                                });
                            }
                        })
                    }
                    _ => {}
                }
            })
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
        &mut self,
        register: Option<char>,
        editor: Option<&mut Editor>,
        cx: &mut Context<Editor>,
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

    // pub fn set_local_mark(editor: &Editor, name: String, positions: &Vec<Anchor>, cx: &mut App) {
    //     let name = name.clone();
    //     let snapshot = editor.buffer().read(cx).snapshot(cx);
    //     let points: Vec<Point> = positions
    //         .iter()
    //         .map(|anchor| anchor.to_point(&snapshot))
    //         .collect();
    //     let locations: Vec<(u32, u32)> = points
    //         .iter()
    //         .map(|point| (point.row, point.column))
    //         .collect();
    //     let Ok(value) = serde_json::to_string(&locations) else {
    //         return;
    //     };

    //     let Some(workspace) = editor.workspace() else {
    //         return;
    //     };
    //     let workspace = workspace.read(cx);
    //     let Some(workspace_id) = workspace.database_id() else {
    //         return;
    //     };

    //     let Some(file) = editor
    //         .buffer()
    //         .read(cx)
    //         .as_singleton()
    //         .and_then(|buffer| buffer.read(cx).file())
    //     else {
    //         return;
    //     };
    //     let path = file.path().to_path_buf();

    //     cx.update_global(|g: &mut VimGlobals, _cx: &mut App| {
    //         if !g
    //             .local_marks
    //             .contains_key(&(workspace_id.clone(), path.clone().into()))
    //         {
    //             g.local_marks.insert(
    //                 (workspace_id.clone(), path.clone().into()),
    //                 HashMap::<String, Vec<Point>>::default(),
    //             );
    //         }
    //         let _ = g
    //             .local_marks
    //             .get_mut(&(workspace_id.clone(), path.clone().into()))
    //             .and_then(|map| map.insert(name.clone(), points));
    //     });

    //     cx.background_executor()
    //         .spawn(DB.set_mark(workspace_id, name, path.into_os_string().into_vec(), value))
    //         .detach_and_log_err(cx);
    // }

    // pub fn set_global_mark(editor: &Editor, name: String, positions: &Vec<Anchor>, cx: &mut App) {
    //     let name = name.clone();
    //     let snapshot = editor.buffer().read(cx).snapshot(cx);
    //     let points: Vec<Point> = positions
    //         .iter()
    //         .map(|anchor| anchor.to_point(&snapshot))
    //         .collect();
    //     let locations: Vec<(u32, u32)> = points
    //         .iter()
    //         .map(|point| (point.row, point.column))
    //         .collect();
    //     let Ok(value) = serde_json::to_string(&locations) else {
    //         return;
    //     };

    //     let Some(workspace) = editor.workspace() else {
    //         return;
    //     };
    //     let workspace = workspace.read(cx);
    //     let Some(workspace_id) = workspace.database_id() else {
    //         return;
    //     };

    //     let Some(file) = editor
    //         .buffer()
    //         .read(cx)
    //         .as_singleton()
    //         .and_then(|buffer| buffer.read(cx).file())
    //     else {
    //         return;
    //     };
    //     let path = file.path().to_path_buf();

    //     cx.update_global(|g: &mut VimGlobals, _cx: &mut App| {
    //         if !g.global_marks.contains_key(&workspace_id.clone()) {
    //             g.global_marks.insert(
    //                 workspace_id.clone(),
    //                 HashMap::<String, (Arc<Path>, Vec<Point>)>::default(),
    //             );
    //         }
    //         let _ = g
    //             .global_marks
    //             .get_mut(&workspace_id.clone())
    //             .and_then(|map| map.insert(name.clone(), (path.clone().into(), points)));
    //     });
    //     cx.background_executor()
    //         .spawn(DB.set_global_mark_path(
    //             workspace_id,
    //             name,
    //             path.into_os_string().into_vec(),
    //             value,
    //         ))
    //         .detach_and_log_err(cx);
    // }

    // fn load_local_marks(
    //     &mut self,
    //     workspace_id: WorkspaceId,
    //     marks: Vec<(Vec<u8>, String, String)>,
    // ) {
    //     for (abs_path, name, values) in marks {
    //         let Some(value) = serde_json::from_str::<Vec<(u32, u32)>>(&values).log_err() else {
    //             continue;
    //         };
    //         let path = PathBuf::from(OsString::from_vec(abs_path));
    //         let marks = self
    //             .local_marks
    //             .entry((workspace_id, Arc::from(path)))
    //             .or_default();
    //         let points: Vec<Point> = value
    //             .into_iter()
    //             .map(|(row, col)| Point::new(row, col))
    //             .collect();
    //         marks.insert(name, points);
    //     }
    // }

    // fn load_global_marks(
    //     &mut self,
    //     workspace_id: WorkspaceId,
    //     marks: Vec<(Vec<u8>, String, String)>,
    // ) {
    //     for (abs_path, name, values) in marks {
    //         let Some(value) = serde_json::from_str::<Vec<(u32, u32)>>(&values).log_err() else {
    //             continue;
    //         };
    //         let path = PathBuf::from(OsString::from_vec(abs_path));
    //         let global_marks = self.global_marks.entry(workspace_id).or_default();
    //         let points = value
    //             .into_iter()
    //             .map(|(row, col)| Point::new(row, col))
    //             .collect();
    //         global_marks.insert(name, (Arc::from(path), points));
    //     }
    // }

    fn system_clipboard_is_newer(&self, cx: &mut Context<Editor>) -> bool {
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

impl VimDb {
    pub(crate) async fn set_mark(
        &self,
        workspace_id: WorkspaceId,
        mark_name: String,
        path: Vec<u8>,
        value: String,
    ) -> Result<()> {
        self.write(move |conn| {
            conn.exec_bound(sql!(
                INSERT OR REPLACE INTO vim_marks
                    (workspace_id, mark_name, path, value)
                VALUES
                    (?, ?, ?, ?)
            ))?((workspace_id, mark_name, path, value))
        })
        .await
    }

    query! {
        pub fn get_marks(workspace_id: WorkspaceId) -> Result<Vec<(Vec<u8>, String, String)>> {
            SELECT path, mark_name, value FROM vim_marks
                WHERE workspace_id = ?
        }
    }

    pub(crate) async fn set_global_mark_path(
        &self,
        workspace_id: WorkspaceId,
        mark_name: String,
        path: Vec<u8>,
    ) -> Result<()> {
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

    query! {
        pub fn get_global_marks_paths(workspace_id: WorkspaceId) -> Result<Vec<(Vec<u8>, String)>> {
            SELECT path, mark_name FROM vim_global_marks_paths
                WHERE workspace_id = ?
        }
    }
}

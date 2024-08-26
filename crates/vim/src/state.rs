use std::borrow::BorrowMut;
use std::{fmt::Display, ops::Range, sync::Arc};

use crate::command::command_interceptor;
use crate::normal::repeat::Replayer;
use crate::surrounds::SurroundsType;
use crate::{motion::Motion, object::Object};
use crate::{UseSystemClipboard, Vim, VimSettings};
use collections::HashMap;
use command_palette_hooks::{CommandPaletteFilter, CommandPaletteInterceptor};
use editor::{Anchor, ClipboardSelection, Editor};
use gpui::{
    Action, AppContext, BorrowAppContext, ClipboardEntry, ClipboardItem, Global, View, WeakView,
};
use language::Point;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use ui::{SharedString, ViewContext};
use workspace::searchable::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Mode {
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
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
        }
    }
}

impl Mode {
    pub fn is_visual(&self) -> bool {
        match self {
            Mode::Normal | Mode::Insert | Mode::Replace => false,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => true,
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum Operator {
    Change,
    Delete,
    Yank,
    Replace,
    Object { around: bool },
    FindForward { before: bool },
    FindBackward { after: bool },
    AddSurrounds { target: Option<SurroundsType> },
    ChangeSurrounds { target: Option<Object> },
    DeleteSurrounds,
    Mark,
    Jump { line: bool },
    Indent,
    Outdent,
    Lowercase,
    Uppercase,
    OppositeCase,
    Digraph { first_char: Option<char> },
    Register,
    RecordRegister,
    ReplayRegister,
    ToggleComments,
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

#[derive(Default, Clone)]
pub struct VimGlobals {
    pub last_find: Option<Motion>,

    pub dot_recording: bool,
    pub dot_replaying: bool,

    pub stop_recording_after_next_action: bool,
    pub ignore_current_insertion: bool,
    pub recorded_count: Option<usize>,
    pub recorded_actions: Vec<ReplayableAction>,
    pub recorded_selection: RecordedSelection,

    pub recording_register: Option<char>,
    pub last_recorded_register: Option<char>,
    pub last_replayed_register: Option<char>,
    pub replayer: Option<Replayer>,

    pub last_yank: Option<SharedString>,
    pub registers: HashMap<char, Register>,
    pub recordings: HashMap<char, Vec<ReplayableAction>>,

    pub focused_vim: Option<WeakView<Vim>>,
}
impl Global for VimGlobals {}

impl VimGlobals {
    pub(crate) fn register(cx: &mut AppContext) {
        cx.set_global(VimGlobals::default());

        cx.observe_keystrokes(|event, cx| {
            let Some(action) = event.action.as_ref().map(|action| action.boxed_clone()) else {
                return;
            };
            Vim::globals(cx).observe_action(action.boxed_clone())
        })
        .detach();

        cx.observe_global::<SettingsStore>(move |cx| {
            if Vim::enabled(cx) {
                CommandPaletteFilter::update_global(cx, |filter, _| {
                    filter.show_namespace(Vim::NAMESPACE);
                });
                CommandPaletteInterceptor::update_global(cx, |interceptor, _| {
                    interceptor.set(Box::new(command_interceptor));
                });
            } else {
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
    }

    pub(crate) fn write_registers(
        &mut self,
        content: Register,
        register: Option<char>,
        is_yank: bool,
        linewise: bool,
        cx: &mut ViewContext<Editor>,
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
                self.registers.insert('"', content.clone());
                match lower {
                    '_' | ':' | '.' | '%' | '#' | '=' | '/' => {}
                    '+' => {
                        cx.write_to_clipboard(content.into());
                    }
                    '*' => {
                        #[cfg(target_os = "linux")]
                        cx.write_to_primary(content.into());
                        #[cfg(not(target_os = "linux"))]
                        cx.write_to_clipboard(content.into());
                    }
                    '"' => {
                        self.registers.insert('0', content.clone());
                        self.registers.insert('"', content);
                    }
                    _ => {
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
        cx: &ViewContext<Editor>,
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
                #[cfg(target_os = "linux")]
                {
                    cx.read_from_primary().map(|item| item.into())
                }
                #[cfg(not(target_os = "linux"))]
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

    fn system_clipboard_is_newer(&self, cx: &ViewContext<Editor>) -> bool {
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
            self.recorded_actions
                .push(ReplayableAction::Action(action.boxed_clone()));

            if self.stop_recording_after_next_action {
                self.dot_recording = false;
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
            self.recorded_actions.push(ReplayableAction::Insertion {
                text: text.clone(),
                utf16_range_to_replace: range_to_replace.clone(),
            });
            if self.stop_recording_after_next_action {
                self.dot_recording = false;
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

    pub fn focused_vim(&self) -> Option<View<Vim>> {
        self.focused_vim.as_ref().and_then(|vim| vim.upgrade())
    }
}

impl Vim {
    pub fn globals(cx: &mut AppContext) -> &mut VimGlobals {
        cx.global_mut::<VimGlobals>()
    }

    pub fn update_globals<C, R>(cx: &mut C, f: impl FnOnce(&mut VimGlobals, &mut C) -> R) -> R
    where
        C: BorrowMut<AppContext>,
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
    pub initial_query: String,

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
            Operator::FindForward { before: false } => "f",
            Operator::FindForward { before: true } => "t",
            Operator::FindBackward { after: false } => "F",
            Operator::FindBackward { after: true } => "T",
            Operator::AddSurrounds { .. } => "ys",
            Operator::ChangeSurrounds { .. } => "cs",
            Operator::DeleteSurrounds => "ds",
            Operator::Mark => "m",
            Operator::Jump { line: true } => "'",
            Operator::Jump { line: false } => "`",
            Operator::Indent => ">",
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

    pub fn is_waiting(&self, mode: Mode) -> bool {
        match self {
            Operator::AddSurrounds { target } => target.is_some() || mode.is_visual(),
            Operator::FindForward { .. }
            | Operator::Mark
            | Operator::Jump { .. }
            | Operator::FindBackward { .. }
            | Operator::Register
            | Operator::RecordRegister
            | Operator::ReplayRegister
            | Operator::Replace
            | Operator::Digraph { .. }
            | Operator::ChangeSurrounds { target: Some(_) }
            | Operator::DeleteSurrounds => true,
            Operator::Change
            | Operator::Delete
            | Operator::Yank
            | Operator::Indent
            | Operator::Outdent
            | Operator::Lowercase
            | Operator::Uppercase
            | Operator::Object { .. }
            | Operator::ChangeSurrounds { target: None }
            | Operator::OppositeCase
            | Operator::ToggleComments => false,
        }
    }
}

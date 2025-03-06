use crate::command::command_interceptor;
use crate::normal::repeat::Replayer;
use crate::surrounds::SurroundsType;
use crate::{motion::Motion, object::Object};
use crate::{ToggleRegistersView, UseSystemClipboard, Vim, VimSettings};
use collections::HashMap;
use command_palette_hooks::{CommandPaletteFilter, CommandPaletteInterceptor};
use editor::display_map::{is_invisible, replacement};
use editor::{Anchor, ClipboardSelection, Editor};
use gpui::{
    Action, App, BorrowAppContext, ClipboardEntry, ClipboardItem, Entity, Global, HighlightStyle,
    StyledText, Task, TextStyle, WeakEntity,
};
use language::Point;
use picker::{Picker, PickerDelegate};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::borrow::BorrowMut;
use std::{fmt::Display, ops::Range, sync::Arc};
use theme::ThemeSettings;
use ui::{
    h_flex, rems, ActiveTheme, Context, Div, FluentBuilder, KeyBinding, ParentElement,
    SharedString, Styled, StyledTypography, Window,
};
use workspace::searchable::Direction;
use workspace::Workspace;

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

#[derive(Default, Clone)]
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

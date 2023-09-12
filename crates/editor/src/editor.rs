mod blink_manager;
pub mod display_map;
mod editor_settings;
mod element;
mod inlay_hint_cache;

mod git;
mod highlight_matching_bracket;
mod hover_popover;
pub mod items;
mod link_go_to_definition;
mod mouse_context_menu;
pub mod movement;
pub mod multi_buffer;
mod persistence;
pub mod scroll;
pub mod selections_collection;

#[cfg(test)]
mod editor_tests;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

use ::git::diff::DiffHunk;
use aho_corasick::AhoCorasick;
use anyhow::{anyhow, Context, Result};
use blink_manager::BlinkManager;
use client::{ClickhouseEvent, TelemetrySettings};
use clock::{Global, ReplicaId};
use collections::{BTreeMap, Bound, HashMap, HashSet, VecDeque};
use convert_case::{Case, Casing};
use copilot::Copilot;
pub use display_map::DisplayPoint;
use display_map::*;
pub use editor_settings::EditorSettings;
pub use element::{
    Cursor, EditorElement, HighlightedRange, HighlightedRangeLine, LineWithInvisibles,
};
use futures::FutureExt;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    color::Color,
    elements::*,
    executor,
    fonts::{self, HighlightStyle, TextStyle},
    geometry::vector::{vec2f, Vector2F},
    impl_actions,
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, MouseButton},
    serde_json, AnyElement, AnyViewHandle, AppContext, AsyncAppContext, ClipboardItem, Element,
    Entity, ModelHandle, Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
    WindowContext,
};
use highlight_matching_bracket::refresh_matching_bracket_highlights;
use hover_popover::{hide_hover, HoverState};
use inlay_hint_cache::{InlayHintCache, InlaySplice, InvalidationStrategy};
pub use items::MAX_TAB_TITLE_LEN;
use itertools::Itertools;
pub use language::{char_kind, CharKind};
use language::{
    language_settings::{self, all_language_settings, InlayHintSettings},
    point_from_lsp, AutoindentMode, BracketPair, Buffer, CodeAction, CodeLabel, Completion,
    CursorShape, Diagnostic, DiagnosticSeverity, File, IndentKind, IndentSize, Language,
    LanguageServerName, OffsetRangeExt, OffsetUtf16, Point, Selection, SelectionGoal,
    TransactionId,
};
use link_go_to_definition::{
    hide_link_definition, show_link_definition, DocumentRange, GoToDefinitionLink, InlayRange,
    LinkGoToDefinitionState,
};
use log::error;
use lsp::LanguageServerId;
use multi_buffer::ToOffsetUtf16;
pub use multi_buffer::{
    Anchor, AnchorRangeExt, ExcerptId, ExcerptRange, MultiBuffer, MultiBufferSnapshot, ToOffset,
    ToPoint,
};
use ordered_float::OrderedFloat;
use project::{FormatTrigger, Location, Project, ProjectPath, ProjectTransaction};
use rand::{seq::SliceRandom, thread_rng};
use scroll::{
    autoscroll::Autoscroll, OngoingScroll, ScrollAnchor, ScrollManager, ScrollbarAutoHide,
};
use selections_collection::{resolve_multiple, MutableSelectionsCollection, SelectionsCollection};
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use smallvec::SmallVec;
use snippet::Snippet;
use std::{
    any::TypeId,
    borrow::Cow,
    cmp::{self, Ordering, Reverse},
    mem,
    num::NonZeroU32,
    ops::{ControlFlow, Deref, DerefMut, Range, RangeInclusive},
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
pub use sum_tree::Bias;
use text::Rope;
use theme::{DiagnosticStyle, Theme, ThemeSettings};
use util::{post_inc, RangeExt, ResultExt, TryFutureExt};
use workspace::{ItemNavHistory, ViewId, Workspace};

use crate::git::diff_hunk_to_display;

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);
const MAX_LINE_LEN: usize = 1024;
const MIN_NAVIGATION_HISTORY_ROW_DELTA: i64 = 10;
const MAX_SELECTION_HISTORY_LEN: usize = 1024;
const COPILOT_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);
pub const CODE_ACTIONS_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(250);
pub const DOCUMENT_HIGHLIGHTS_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub const FORMAT_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Deserialize, PartialEq, Default)]
pub struct SelectNext {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(Clone, Deserialize, PartialEq, Default)]
pub struct SelectPrevious {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct SelectToBeginningOfLine {
    #[serde(default)]
    stop_at_soft_wraps: bool,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct MovePageUp {
    #[serde(default)]
    center_cursor: bool,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct MovePageDown {
    #[serde(default)]
    center_cursor: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct SelectToEndOfLine {
    #[serde(default)]
    stop_at_soft_wraps: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct ToggleCodeActions {
    #[serde(default)]
    pub deployed_from_indicator: bool,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct ConfirmCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct ConfirmCodeAction {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct ToggleComments {
    #[serde(default)]
    pub advance_downwards: bool,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct FoldAt {
    pub buffer_row: u32,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct UnfoldAt {
    pub buffer_row: u32,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct GutterHover {
    pub hovered: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlayId {
    Suggestion(usize),
    Hint(usize),
}

impl InlayId {
    fn id(&self) -> usize {
        match self {
            Self::Suggestion(id) => *id,
            Self::Hint(id) => *id,
        }
    }
}

actions!(
    editor,
    [
        Cancel,
        Backspace,
        Delete,
        Newline,
        NewlineAbove,
        NewlineBelow,
        GoToDiagnostic,
        GoToPrevDiagnostic,
        GoToHunk,
        GoToPrevHunk,
        Indent,
        Outdent,
        DeleteLine,
        DeleteToPreviousWordStart,
        DeleteToPreviousSubwordStart,
        DeleteToNextWordEnd,
        DeleteToNextSubwordEnd,
        DeleteToBeginningOfLine,
        DeleteToEndOfLine,
        CutToEndOfLine,
        DuplicateLine,
        MoveLineUp,
        MoveLineDown,
        JoinLines,
        SortLinesCaseSensitive,
        SortLinesCaseInsensitive,
        ReverseLines,
        ShuffleLines,
        ConvertToUpperCase,
        ConvertToLowerCase,
        ConvertToTitleCase,
        ConvertToSnakeCase,
        ConvertToKebabCase,
        ConvertToUpperCamelCase,
        ConvertToLowerCamelCase,
        Transpose,
        Cut,
        Copy,
        Paste,
        Undo,
        Redo,
        MoveUp,
        PageUp,
        MoveDown,
        PageDown,
        MoveLeft,
        MoveRight,
        MoveToPreviousWordStart,
        MoveToPreviousSubwordStart,
        MoveToNextWordEnd,
        MoveToNextSubwordEnd,
        MoveToBeginningOfLine,
        MoveToEndOfLine,
        MoveToStartOfParagraph,
        MoveToEndOfParagraph,
        MoveToBeginning,
        MoveToEnd,
        SelectUp,
        SelectDown,
        SelectLeft,
        SelectRight,
        SelectToPreviousWordStart,
        SelectToPreviousSubwordStart,
        SelectToNextWordEnd,
        SelectToNextSubwordEnd,
        SelectToStartOfParagraph,
        SelectToEndOfParagraph,
        SelectToBeginning,
        SelectToEnd,
        SelectAll,
        SelectLine,
        SplitSelectionIntoLines,
        AddSelectionAbove,
        AddSelectionBelow,
        Tab,
        TabPrev,
        ShowCharacterPalette,
        SelectLargerSyntaxNode,
        SelectSmallerSyntaxNode,
        GoToDefinition,
        GoToDefinitionSplit,
        GoToTypeDefinition,
        GoToTypeDefinitionSplit,
        MoveToEnclosingBracket,
        UndoSelection,
        RedoSelection,
        FindAllReferences,
        Rename,
        ConfirmRename,
        Fold,
        UnfoldLines,
        FoldSelectedRanges,
        ShowCompletions,
        OpenExcerpts,
        RestartLanguageServer,
        Hover,
        Format,
        ToggleSoftWrap,
        ToggleInlayHints,
        RevealInFinder,
        CopyPath,
        CopyRelativePath,
        CopyHighlightJson,
        ContextMenuFirst,
        ContextMenuPrev,
        ContextMenuNext,
        ContextMenuLast,
    ]
);

impl_actions!(
    editor,
    [
        SelectNext,
        SelectPrevious,
        SelectToBeginningOfLine,
        SelectToEndOfLine,
        ToggleCodeActions,
        MovePageUp,
        MovePageDown,
        ConfirmCompletion,
        ConfirmCodeAction,
        ToggleComments,
        FoldAt,
        UnfoldAt,
        GutterHover
    ]
);

enum DocumentHighlightRead {}
enum DocumentHighlightWrite {}
enum InputComposition {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

pub fn init_settings(cx: &mut AppContext) {
    settings::register::<EditorSettings>(cx);
}

pub fn init(cx: &mut AppContext) {
    init_settings(cx);
    cx.add_action(Editor::new_file);
    cx.add_action(Editor::cancel);
    cx.add_action(Editor::newline);
    cx.add_action(Editor::newline_above);
    cx.add_action(Editor::newline_below);
    cx.add_action(Editor::backspace);
    cx.add_action(Editor::delete);
    cx.add_action(Editor::tab);
    cx.add_action(Editor::tab_prev);
    cx.add_action(Editor::indent);
    cx.add_action(Editor::outdent);
    cx.add_action(Editor::delete_line);
    cx.add_action(Editor::join_lines);
    cx.add_action(Editor::sort_lines_case_sensitive);
    cx.add_action(Editor::sort_lines_case_insensitive);
    cx.add_action(Editor::reverse_lines);
    cx.add_action(Editor::shuffle_lines);
    cx.add_action(Editor::convert_to_upper_case);
    cx.add_action(Editor::convert_to_lower_case);
    cx.add_action(Editor::convert_to_title_case);
    cx.add_action(Editor::convert_to_snake_case);
    cx.add_action(Editor::convert_to_kebab_case);
    cx.add_action(Editor::convert_to_upper_camel_case);
    cx.add_action(Editor::convert_to_lower_camel_case);
    cx.add_action(Editor::delete_to_previous_word_start);
    cx.add_action(Editor::delete_to_previous_subword_start);
    cx.add_action(Editor::delete_to_next_word_end);
    cx.add_action(Editor::delete_to_next_subword_end);
    cx.add_action(Editor::delete_to_beginning_of_line);
    cx.add_action(Editor::delete_to_end_of_line);
    cx.add_action(Editor::cut_to_end_of_line);
    cx.add_action(Editor::duplicate_line);
    cx.add_action(Editor::move_line_up);
    cx.add_action(Editor::move_line_down);
    cx.add_action(Editor::transpose);
    cx.add_action(Editor::cut);
    cx.add_action(Editor::copy);
    cx.add_action(Editor::paste);
    cx.add_action(Editor::undo);
    cx.add_action(Editor::redo);
    cx.add_action(Editor::move_up);
    cx.add_action(Editor::move_page_up);
    cx.add_action(Editor::move_down);
    cx.add_action(Editor::move_page_down);
    cx.add_action(Editor::next_screen);
    cx.add_action(Editor::move_left);
    cx.add_action(Editor::move_right);
    cx.add_action(Editor::move_to_previous_word_start);
    cx.add_action(Editor::move_to_previous_subword_start);
    cx.add_action(Editor::move_to_next_word_end);
    cx.add_action(Editor::move_to_next_subword_end);
    cx.add_action(Editor::move_to_beginning_of_line);
    cx.add_action(Editor::move_to_end_of_line);
    cx.add_action(Editor::move_to_start_of_paragraph);
    cx.add_action(Editor::move_to_end_of_paragraph);
    cx.add_action(Editor::move_to_beginning);
    cx.add_action(Editor::move_to_end);
    cx.add_action(Editor::select_up);
    cx.add_action(Editor::select_down);
    cx.add_action(Editor::select_left);
    cx.add_action(Editor::select_right);
    cx.add_action(Editor::select_to_previous_word_start);
    cx.add_action(Editor::select_to_previous_subword_start);
    cx.add_action(Editor::select_to_next_word_end);
    cx.add_action(Editor::select_to_next_subword_end);
    cx.add_action(Editor::select_to_beginning_of_line);
    cx.add_action(Editor::select_to_end_of_line);
    cx.add_action(Editor::select_to_start_of_paragraph);
    cx.add_action(Editor::select_to_end_of_paragraph);
    cx.add_action(Editor::select_to_beginning);
    cx.add_action(Editor::select_to_end);
    cx.add_action(Editor::select_all);
    cx.add_action(Editor::select_line);
    cx.add_action(Editor::split_selection_into_lines);
    cx.add_action(Editor::add_selection_above);
    cx.add_action(Editor::add_selection_below);
    cx.add_action(Editor::select_next);
    cx.add_action(Editor::select_previous);
    cx.add_action(Editor::toggle_comments);
    cx.add_action(Editor::select_larger_syntax_node);
    cx.add_action(Editor::select_smaller_syntax_node);
    cx.add_action(Editor::move_to_enclosing_bracket);
    cx.add_action(Editor::undo_selection);
    cx.add_action(Editor::redo_selection);
    cx.add_action(Editor::go_to_diagnostic);
    cx.add_action(Editor::go_to_prev_diagnostic);
    cx.add_action(Editor::go_to_hunk);
    cx.add_action(Editor::go_to_prev_hunk);
    cx.add_action(Editor::go_to_definition);
    cx.add_action(Editor::go_to_definition_split);
    cx.add_action(Editor::go_to_type_definition);
    cx.add_action(Editor::go_to_type_definition_split);
    cx.add_action(Editor::fold);
    cx.add_action(Editor::fold_at);
    cx.add_action(Editor::unfold_lines);
    cx.add_action(Editor::unfold_at);
    cx.add_action(Editor::gutter_hover);
    cx.add_action(Editor::fold_selected_ranges);
    cx.add_action(Editor::show_completions);
    cx.add_action(Editor::toggle_code_actions);
    cx.add_action(Editor::open_excerpts);
    cx.add_action(Editor::toggle_soft_wrap);
    cx.add_action(Editor::toggle_inlay_hints);
    cx.add_action(Editor::reveal_in_finder);
    cx.add_action(Editor::copy_path);
    cx.add_action(Editor::copy_relative_path);
    cx.add_action(Editor::copy_highlight_json);
    cx.add_async_action(Editor::format);
    cx.add_action(Editor::restart_language_server);
    cx.add_action(Editor::show_character_palette);
    cx.add_async_action(Editor::confirm_completion);
    cx.add_async_action(Editor::confirm_code_action);
    cx.add_async_action(Editor::rename);
    cx.add_async_action(Editor::confirm_rename);
    cx.add_async_action(Editor::find_all_references);
    cx.add_action(Editor::next_copilot_suggestion);
    cx.add_action(Editor::previous_copilot_suggestion);
    cx.add_action(Editor::copilot_suggest);
    cx.add_action(Editor::context_menu_first);
    cx.add_action(Editor::context_menu_prev);
    cx.add_action(Editor::context_menu_next);
    cx.add_action(Editor::context_menu_last);

    hover_popover::init(cx);
    scroll::actions::init(cx);

    workspace::register_project_item::<Editor>(cx);
    workspace::register_followable_item::<Editor>(cx);
    workspace::register_deserializable_item::<Editor>(cx);
}

trait InvalidationRegion {
    fn ranges(&self) -> &[Range<Anchor>];
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectPhase {
    Begin {
        position: DisplayPoint,
        add: bool,
        click_count: usize,
    },
    BeginColumnar {
        position: DisplayPoint,
        goal_column: u32,
    },
    Extend {
        position: DisplayPoint,
        click_count: usize,
    },
    Update {
        position: DisplayPoint,
        goal_column: u32,
        scroll_position: Vector2F,
    },
    End,
}

#[derive(Clone, Debug)]
pub enum SelectMode {
    Character,
    Word(Range<Anchor>),
    Line(Range<Anchor>),
    All,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EditorMode {
    SingleLine,
    AutoHeight { max_lines: usize },
    Full,
}

#[derive(Clone, Debug)]
pub enum SoftWrap {
    None,
    EditorWidth,
    Column(u32),
}

#[derive(Clone)]
pub struct EditorStyle {
    pub text: TextStyle,
    pub line_height_scalar: f32,
    pub placeholder_text: Option<TextStyle>,
    pub theme: theme::Editor,
    pub theme_id: usize,
}

type CompletionId = usize;

type GetFieldEditorTheme = dyn Fn(&theme::Theme) -> theme::FieldEditor;
type OverrideTextStyle = dyn Fn(&EditorStyle) -> Option<HighlightStyle>;

type BackgroundHighlight = (fn(&Theme) -> Color, Vec<DocumentRange>);

pub struct Editor {
    handle: WeakViewHandle<Self>,
    buffer: ModelHandle<MultiBuffer>,
    display_map: ModelHandle<DisplayMap>,
    pub selections: SelectionsCollection,
    pub scroll_manager: ScrollManager,
    columnar_selection_tail: Option<Anchor>,
    add_selections_state: Option<AddSelectionsState>,
    select_next_state: Option<SelectNextState>,
    select_prev_state: Option<SelectNextState>,
    selection_history: SelectionHistory,
    autoclose_regions: Vec<AutocloseRegion>,
    snippet_stack: InvalidationStack<SnippetState>,
    select_larger_syntax_node_stack: Vec<Box<[Selection<usize>]>>,
    ime_transaction: Option<TransactionId>,
    active_diagnostics: Option<ActiveDiagnosticGroup>,
    soft_wrap_mode_override: Option<language_settings::SoftWrap>,
    get_field_editor_theme: Option<Arc<GetFieldEditorTheme>>,
    override_text_style: Option<Box<OverrideTextStyle>>,
    project: Option<ModelHandle<Project>>,
    focused: bool,
    blink_manager: ModelHandle<BlinkManager>,
    pub show_local_selections: bool,
    mode: EditorMode,
    replica_id_mapping: Option<HashMap<ReplicaId, ReplicaId>>,
    show_gutter: bool,
    show_wrap_guides: Option<bool>,
    placeholder_text: Option<Arc<str>>,
    highlighted_rows: Option<Range<u32>>,
    background_highlights: BTreeMap<TypeId, BackgroundHighlight>,
    nav_history: Option<ItemNavHistory>,
    context_menu: Option<ContextMenu>,
    mouse_context_menu: ViewHandle<context_menu::ContextMenu>,
    completion_tasks: Vec<(CompletionId, Task<Option<()>>)>,
    next_completion_id: CompletionId,
    available_code_actions: Option<(ModelHandle<Buffer>, Arc<[CodeAction]>)>,
    code_actions_task: Option<Task<()>>,
    document_highlights_task: Option<Task<()>>,
    pending_rename: Option<RenameState>,
    searchable: bool,
    cursor_shape: CursorShape,
    collapse_matches: bool,
    autoindent_mode: Option<AutoindentMode>,
    workspace: Option<(WeakViewHandle<Workspace>, i64)>,
    keymap_context_layers: BTreeMap<TypeId, KeymapContext>,
    input_enabled: bool,
    read_only: bool,
    leader_replica_id: Option<u16>,
    remote_id: Option<ViewId>,
    hover_state: HoverState,
    gutter_hovered: bool,
    link_go_to_definition_state: LinkGoToDefinitionState,
    copilot_state: CopilotState,
    inlay_hint_cache: InlayHintCache,
    next_inlay_id: usize,
    _subscriptions: Vec<Subscription>,
    pixel_position_of_newest_cursor: Option<Vector2F>,
}

pub struct EditorSnapshot {
    pub mode: EditorMode,
    pub show_gutter: bool,
    pub display_snapshot: DisplaySnapshot,
    pub placeholder_text: Option<Arc<str>>,
    is_focused: bool,
    scroll_anchor: ScrollAnchor,
    ongoing_scroll: OngoingScroll,
}

#[derive(Clone, Debug)]
struct SelectionHistoryEntry {
    selections: Arc<[Selection<Anchor>]>,
    select_next_state: Option<SelectNextState>,
    select_prev_state: Option<SelectNextState>,
    add_selections_state: Option<AddSelectionsState>,
}

enum SelectionHistoryMode {
    Normal,
    Undoing,
    Redoing,
}

impl Default for SelectionHistoryMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
struct SelectionHistory {
    #[allow(clippy::type_complexity)]
    selections_by_transaction:
        HashMap<TransactionId, (Arc<[Selection<Anchor>]>, Option<Arc<[Selection<Anchor>]>>)>,
    mode: SelectionHistoryMode,
    undo_stack: VecDeque<SelectionHistoryEntry>,
    redo_stack: VecDeque<SelectionHistoryEntry>,
}

impl SelectionHistory {
    fn insert_transaction(
        &mut self,
        transaction_id: TransactionId,
        selections: Arc<[Selection<Anchor>]>,
    ) {
        self.selections_by_transaction
            .insert(transaction_id, (selections, None));
    }

    #[allow(clippy::type_complexity)]
    fn transaction(
        &self,
        transaction_id: TransactionId,
    ) -> Option<&(Arc<[Selection<Anchor>]>, Option<Arc<[Selection<Anchor>]>>)> {
        self.selections_by_transaction.get(&transaction_id)
    }

    #[allow(clippy::type_complexity)]
    fn transaction_mut(
        &mut self,
        transaction_id: TransactionId,
    ) -> Option<&mut (Arc<[Selection<Anchor>]>, Option<Arc<[Selection<Anchor>]>>)> {
        self.selections_by_transaction.get_mut(&transaction_id)
    }

    fn push(&mut self, entry: SelectionHistoryEntry) {
        if !entry.selections.is_empty() {
            match self.mode {
                SelectionHistoryMode::Normal => {
                    self.push_undo(entry);
                    self.redo_stack.clear();
                }
                SelectionHistoryMode::Undoing => self.push_redo(entry),
                SelectionHistoryMode::Redoing => self.push_undo(entry),
            }
        }
    }

    fn push_undo(&mut self, entry: SelectionHistoryEntry) {
        if self
            .undo_stack
            .back()
            .map_or(true, |e| e.selections != entry.selections)
        {
            self.undo_stack.push_back(entry);
            if self.undo_stack.len() > MAX_SELECTION_HISTORY_LEN {
                self.undo_stack.pop_front();
            }
        }
    }

    fn push_redo(&mut self, entry: SelectionHistoryEntry) {
        if self
            .redo_stack
            .back()
            .map_or(true, |e| e.selections != entry.selections)
        {
            self.redo_stack.push_back(entry);
            if self.redo_stack.len() > MAX_SELECTION_HISTORY_LEN {
                self.redo_stack.pop_front();
            }
        }
    }
}

#[derive(Clone, Debug)]
struct AddSelectionsState {
    above: bool,
    stack: Vec<usize>,
}

#[derive(Clone, Debug)]
struct SelectNextState {
    query: AhoCorasick,
    wordwise: bool,
    done: bool,
}

#[derive(Debug)]
struct AutocloseRegion {
    selection_id: usize,
    range: Range<Anchor>,
    pair: BracketPair,
}

#[derive(Debug)]
struct SnippetState {
    ranges: Vec<Vec<Range<Anchor>>>,
    active_index: usize,
}

pub struct RenameState {
    pub range: Range<Anchor>,
    pub old_name: Arc<str>,
    pub editor: ViewHandle<Editor>,
    block_id: BlockId,
}

struct InvalidationStack<T>(Vec<T>);

enum ContextMenu {
    Completions(CompletionsMenu),
    CodeActions(CodeActionsMenu),
}

impl ContextMenu {
    fn select_first(&mut self, cx: &mut ViewContext<Editor>) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_first(cx),
                ContextMenu::CodeActions(menu) => menu.select_first(cx),
            }
            true
        } else {
            false
        }
    }

    fn select_prev(&mut self, cx: &mut ViewContext<Editor>) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_prev(cx),
                ContextMenu::CodeActions(menu) => menu.select_prev(cx),
            }
            true
        } else {
            false
        }
    }

    fn select_next(&mut self, cx: &mut ViewContext<Editor>) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_next(cx),
                ContextMenu::CodeActions(menu) => menu.select_next(cx),
            }
            true
        } else {
            false
        }
    }

    fn select_last(&mut self, cx: &mut ViewContext<Editor>) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_last(cx),
                ContextMenu::CodeActions(menu) => menu.select_last(cx),
            }
            true
        } else {
            false
        }
    }

    fn visible(&self) -> bool {
        match self {
            ContextMenu::Completions(menu) => menu.visible(),
            ContextMenu::CodeActions(menu) => menu.visible(),
        }
    }

    fn render(
        &self,
        cursor_position: DisplayPoint,
        style: EditorStyle,
        cx: &mut ViewContext<Editor>,
    ) -> (DisplayPoint, AnyElement<Editor>) {
        match self {
            ContextMenu::Completions(menu) => (cursor_position, menu.render(style, cx)),
            ContextMenu::CodeActions(menu) => menu.render(cursor_position, style, cx),
        }
    }
}

struct CompletionsMenu {
    id: CompletionId,
    initial_position: Anchor,
    buffer: ModelHandle<Buffer>,
    project: Option<ModelHandle<Project>>,
    completions: Arc<[Completion]>,
    match_candidates: Vec<StringMatchCandidate>,
    matches: Arc<[StringMatch]>,
    selected_item: usize,
    list: UniformListState,
}

impl CompletionsMenu {
    fn select_first(&mut self, cx: &mut ViewContext<Editor>) {
        self.selected_item = 0;
        self.list.scroll_to(ScrollTarget::Show(self.selected_item));
        cx.notify();
    }

    fn select_prev(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
            self.list.scroll_to(ScrollTarget::Show(self.selected_item));
        }
        cx.notify();
    }

    fn select_next(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item + 1 < self.matches.len() {
            self.selected_item += 1;
            self.list.scroll_to(ScrollTarget::Show(self.selected_item));
        }
        cx.notify();
    }

    fn select_last(&mut self, cx: &mut ViewContext<Editor>) {
        self.selected_item = self.matches.len() - 1;
        self.list.scroll_to(ScrollTarget::Show(self.selected_item));
        cx.notify();
    }

    fn visible(&self) -> bool {
        !self.matches.is_empty()
    }

    fn render(&self, style: EditorStyle, cx: &mut ViewContext<Editor>) -> AnyElement<Editor> {
        enum CompletionTag {}

        let language_servers = self.project.as_ref().map(|project| {
            project
                .read(cx)
                .language_servers_for_buffer(self.buffer.read(cx), cx)
                .filter(|(_, server)| server.capabilities().completion_provider.is_some())
                .map(|(adapter, server)| (server.server_id(), adapter.short_name))
                .collect::<Vec<_>>()
        });
        let needs_server_name = language_servers
            .as_ref()
            .map_or(false, |servers| servers.len() > 1);

        let get_server_name =
            move |lookup_server_id: lsp::LanguageServerId| -> Option<&'static str> {
                language_servers
                    .iter()
                    .flatten()
                    .find_map(|(server_id, server_name)| {
                        if *server_id == lookup_server_id {
                            Some(*server_name)
                        } else {
                            None
                        }
                    })
            };

        let widest_completion_ix = self
            .matches
            .iter()
            .enumerate()
            .max_by_key(|(_, mat)| {
                let completion = &self.completions[mat.candidate_id];
                let mut len = completion.label.text.chars().count();

                if let Some(server_name) = get_server_name(completion.server_id) {
                    len += server_name.chars().count();
                }

                len
            })
            .map(|(ix, _)| ix);

        let completions = self.completions.clone();
        let matches = self.matches.clone();
        let selected_item = self.selected_item;
        let container_style = style.autocomplete.container;
        UniformList::new(
            self.list.clone(),
            matches.len(),
            cx,
            move |_, range, items, cx| {
                let start_ix = range.start;
                for (ix, mat) in matches[range].iter().enumerate() {
                    let completion = &completions[mat.candidate_id];
                    let item_ix = start_ix + ix;
                    items.push(
                        MouseEventHandler::new::<CompletionTag, _>(
                            mat.candidate_id,
                            cx,
                            |state, _| {
                                let item_style = if item_ix == selected_item {
                                    style.autocomplete.selected_item
                                } else if state.hovered() {
                                    style.autocomplete.hovered_item
                                } else {
                                    style.autocomplete.item
                                };

                                let completion_label =
                                    Text::new(completion.label.text.clone(), style.text.clone())
                                        .with_soft_wrap(false)
                                        .with_highlights(
                                            combine_syntax_and_fuzzy_match_highlights(
                                                &completion.label.text,
                                                style.text.color.into(),
                                                styled_runs_for_code_label(
                                                    &completion.label,
                                                    &style.syntax,
                                                ),
                                                &mat.positions,
                                            ),
                                        );

                                if let Some(server_name) = get_server_name(completion.server_id) {
                                    Flex::row()
                                        .with_child(completion_label)
                                        .with_children((|| {
                                            if !needs_server_name {
                                                return None;
                                            }

                                            let text_style = TextStyle {
                                                color: style.autocomplete.server_name_color,
                                                font_size: style.text.font_size
                                                    * style.autocomplete.server_name_size_percent,
                                                ..style.text.clone()
                                            };

                                            let label = Text::new(server_name, text_style)
                                                .aligned()
                                                .constrained()
                                                .dynamically(move |constraint, _, _| {
                                                    gpui::SizeConstraint {
                                                        min: constraint.min,
                                                        max: vec2f(
                                                            constraint.max.x(),
                                                            constraint.min.y(),
                                                        ),
                                                    }
                                                });

                                            if Some(item_ix) == widest_completion_ix {
                                                Some(
                                                    label
                                                        .contained()
                                                        .with_style(
                                                            style
                                                                .autocomplete
                                                                .server_name_container,
                                                        )
                                                        .into_any(),
                                                )
                                            } else {
                                                Some(label.flex_float().into_any())
                                            }
                                        })())
                                        .into_any()
                                } else {
                                    completion_label.into_any()
                                }
                                .contained()
                                .with_style(item_style)
                                .constrained()
                                .dynamically(
                                    move |constraint, _, _| {
                                        if Some(item_ix) == widest_completion_ix {
                                            constraint
                                        } else {
                                            gpui::SizeConstraint {
                                                min: constraint.min,
                                                max: constraint.min,
                                            }
                                        }
                                    },
                                )
                            },
                        )
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_down(MouseButton::Left, move |_, this, cx| {
                            this.confirm_completion(
                                &ConfirmCompletion {
                                    item_ix: Some(item_ix),
                                },
                                cx,
                            );
                        })
                        .into_any(),
                    );
                }
            },
        )
        .with_width_from_item(widest_completion_ix)
        .contained()
        .with_style(container_style)
        .into_any()
    }

    pub async fn filter(&mut self, query: Option<&str>, executor: Arc<executor::Background>) {
        let mut matches = if let Some(query) = query {
            fuzzy::match_strings(
                &self.match_candidates,
                query,
                query.chars().any(|c| c.is_uppercase()),
                100,
                &Default::default(),
                executor,
            )
            .await
        } else {
            self.match_candidates
                .iter()
                .enumerate()
                .map(|(candidate_id, candidate)| StringMatch {
                    candidate_id,
                    score: Default::default(),
                    positions: Default::default(),
                    string: candidate.string.clone(),
                })
                .collect()
        };

        //Remove all candidates where the query's start does not match the start of any word in the candidate
        if let Some(query) = query {
            if let Some(query_start) = query.chars().next() {
                matches.retain(|string_match| {
                    split_words(&string_match.string).any(|word| {
                        //Check that the first codepoint of the word as lowercase matches the first
                        //codepoint of the query as lowercase
                        word.chars()
                            .flat_map(|codepoint| codepoint.to_lowercase())
                            .zip(query_start.to_lowercase())
                            .all(|(word_cp, query_cp)| word_cp == query_cp)
                    })
                });
            }
        }

        matches.sort_unstable_by_key(|mat| {
            let completion = &self.completions[mat.candidate_id];
            (
                completion.lsp_completion.sort_text.as_ref(),
                Reverse(OrderedFloat(mat.score)),
                completion.sort_key(),
            )
        });

        for mat in &mut matches {
            let filter_start = self.completions[mat.candidate_id].label.filter_range.start;
            for position in &mut mat.positions {
                *position += filter_start;
            }
        }

        self.matches = matches.into();
    }
}

#[derive(Clone)]
struct CodeActionsMenu {
    actions: Arc<[CodeAction]>,
    buffer: ModelHandle<Buffer>,
    selected_item: usize,
    list: UniformListState,
    deployed_from_indicator: bool,
}

impl CodeActionsMenu {
    fn select_first(&mut self, cx: &mut ViewContext<Editor>) {
        self.selected_item = 0;
        cx.notify()
    }

    fn select_prev(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
            cx.notify()
        }
    }

    fn select_next(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item + 1 < self.actions.len() {
            self.selected_item += 1;
            cx.notify()
        }
    }

    fn select_last(&mut self, cx: &mut ViewContext<Editor>) {
        self.selected_item = self.actions.len() - 1;
        cx.notify()
    }

    fn visible(&self) -> bool {
        !self.actions.is_empty()
    }

    fn render(
        &self,
        mut cursor_position: DisplayPoint,
        style: EditorStyle,
        cx: &mut ViewContext<Editor>,
    ) -> (DisplayPoint, AnyElement<Editor>) {
        enum ActionTag {}

        let container_style = style.autocomplete.container;
        let actions = self.actions.clone();
        let selected_item = self.selected_item;
        let element = UniformList::new(
            self.list.clone(),
            actions.len(),
            cx,
            move |_, range, items, cx| {
                let start_ix = range.start;
                for (ix, action) in actions[range].iter().enumerate() {
                    let item_ix = start_ix + ix;
                    items.push(
                        MouseEventHandler::new::<ActionTag, _>(item_ix, cx, |state, _| {
                            let item_style = if item_ix == selected_item {
                                style.autocomplete.selected_item
                            } else if state.hovered() {
                                style.autocomplete.hovered_item
                            } else {
                                style.autocomplete.item
                            };

                            Text::new(action.lsp_action.title.clone(), style.text.clone())
                                .with_soft_wrap(false)
                                .contained()
                                .with_style(item_style)
                        })
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_down(MouseButton::Left, move |_, this, cx| {
                            let workspace = this
                                .workspace
                                .as_ref()
                                .and_then(|(workspace, _)| workspace.upgrade(cx));
                            cx.window_context().defer(move |cx| {
                                if let Some(workspace) = workspace {
                                    workspace.update(cx, |workspace, cx| {
                                        if let Some(task) = Editor::confirm_code_action(
                                            workspace,
                                            &Default::default(),
                                            cx,
                                        ) {
                                            task.detach_and_log_err(cx);
                                        }
                                    });
                                }
                            });
                        })
                        .into_any(),
                    );
                }
            },
        )
        .with_width_from_item(
            self.actions
                .iter()
                .enumerate()
                .max_by_key(|(_, action)| action.lsp_action.title.chars().count())
                .map(|(ix, _)| ix),
        )
        .contained()
        .with_style(container_style)
        .into_any();

        if self.deployed_from_indicator {
            *cursor_position.column_mut() = 0;
        }

        (cursor_position, element)
    }
}

pub struct CopilotState {
    excerpt_id: Option<ExcerptId>,
    pending_refresh: Task<Option<()>>,
    pending_cycling_refresh: Task<Option<()>>,
    cycled: bool,
    completions: Vec<copilot::Completion>,
    active_completion_index: usize,
    suggestion: Option<Inlay>,
}

impl Default for CopilotState {
    fn default() -> Self {
        Self {
            excerpt_id: None,
            pending_cycling_refresh: Task::ready(Some(())),
            pending_refresh: Task::ready(Some(())),
            completions: Default::default(),
            active_completion_index: 0,
            cycled: false,
            suggestion: None,
        }
    }
}

impl CopilotState {
    fn active_completion(&self) -> Option<&copilot::Completion> {
        self.completions.get(self.active_completion_index)
    }

    fn text_for_active_completion(
        &self,
        cursor: Anchor,
        buffer: &MultiBufferSnapshot,
    ) -> Option<&str> {
        use language::ToOffset as _;

        let completion = self.active_completion()?;
        let excerpt_id = self.excerpt_id?;
        let completion_buffer = buffer.buffer_for_excerpt(excerpt_id)?;
        if excerpt_id != cursor.excerpt_id
            || !completion.range.start.is_valid(completion_buffer)
            || !completion.range.end.is_valid(completion_buffer)
        {
            return None;
        }

        let mut completion_range = completion.range.to_offset(&completion_buffer);
        let prefix_len = Self::common_prefix(
            completion_buffer.chars_for_range(completion_range.clone()),
            completion.text.chars(),
        );
        completion_range.start += prefix_len;
        let suffix_len = Self::common_prefix(
            completion_buffer.reversed_chars_for_range(completion_range.clone()),
            completion.text[prefix_len..].chars().rev(),
        );
        completion_range.end = completion_range.end.saturating_sub(suffix_len);

        if completion_range.is_empty()
            && completion_range.start == cursor.text_anchor.to_offset(&completion_buffer)
        {
            Some(&completion.text[prefix_len..completion.text.len() - suffix_len])
        } else {
            None
        }
    }

    fn cycle_completions(&mut self, direction: Direction) {
        match direction {
            Direction::Prev => {
                self.active_completion_index = if self.active_completion_index == 0 {
                    self.completions.len().saturating_sub(1)
                } else {
                    self.active_completion_index - 1
                };
            }
            Direction::Next => {
                if self.completions.len() == 0 {
                    self.active_completion_index = 0
                } else {
                    self.active_completion_index =
                        (self.active_completion_index + 1) % self.completions.len();
                }
            }
        }
    }

    fn push_completion(&mut self, new_completion: copilot::Completion) {
        for completion in &self.completions {
            if completion.text == new_completion.text && completion.range == new_completion.range {
                return;
            }
        }
        self.completions.push(new_completion);
    }

    fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
        a.zip(b)
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a.len_utf8())
            .sum()
    }
}

#[derive(Debug)]
struct ActiveDiagnosticGroup {
    primary_range: Range<Anchor>,
    primary_message: String,
    blocks: HashMap<BlockId, Diagnostic>,
    is_valid: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ClipboardSelection {
    pub len: usize,
    pub is_entire_line: bool,
    pub first_line_indent: u32,
}

#[derive(Debug)]
pub struct NavigationData {
    cursor_anchor: Anchor,
    cursor_position: Point,
    scroll_anchor: ScrollAnchor,
    scroll_top_row: u32,
}

pub struct EditorCreated(pub ViewHandle<Editor>);

enum GotoDefinitionKind {
    Symbol,
    Type,
}

#[derive(Debug, Clone)]
enum InlayHintRefreshReason {
    Toggle(bool),
    SettingsChange(InlayHintSettings),
    NewLinesShown,
    BufferEdited(HashSet<Arc<Language>>),
    RefreshRequested,
    ExcerptsRemoved(Vec<ExcerptId>),
}
impl InlayHintRefreshReason {
    fn description(&self) -> &'static str {
        match self {
            Self::Toggle(_) => "toggle",
            Self::SettingsChange(_) => "settings change",
            Self::NewLinesShown => "new lines shown",
            Self::BufferEdited(_) => "buffer edited",
            Self::RefreshRequested => "refresh requested",
            Self::ExcerptsRemoved(_) => "excerpts removed",
        }
    }
}

impl Editor {
    pub fn single_line(
        field_editor_style: Option<Arc<GetFieldEditorTheme>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, cx.model_id() as u64, String::new()));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(EditorMode::SingleLine, buffer, None, field_editor_style, cx)
    }

    pub fn multi_line(
        field_editor_style: Option<Arc<GetFieldEditorTheme>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, cx.model_id() as u64, String::new()));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(EditorMode::Full, buffer, None, field_editor_style, cx)
    }

    pub fn auto_height(
        max_lines: usize,
        field_editor_style: Option<Arc<GetFieldEditorTheme>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, cx.model_id() as u64, String::new()));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(
            EditorMode::AutoHeight { max_lines },
            buffer,
            None,
            field_editor_style,
            cx,
        )
    }

    pub fn for_buffer(
        buffer: ModelHandle<Buffer>,
        project: Option<ModelHandle<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(EditorMode::Full, buffer, project, None, cx)
    }

    pub fn for_multibuffer(
        buffer: ModelHandle<MultiBuffer>,
        project: Option<ModelHandle<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::new(EditorMode::Full, buffer, project, None, cx)
    }

    pub fn clone(&self, cx: &mut ViewContext<Self>) -> Self {
        let mut clone = Self::new(
            self.mode,
            self.buffer.clone(),
            self.project.clone(),
            self.get_field_editor_theme.clone(),
            cx,
        );
        self.display_map.update(cx, |display_map, cx| {
            let snapshot = display_map.snapshot(cx);
            clone.display_map.update(cx, |display_map, cx| {
                display_map.set_state(&snapshot, cx);
            });
        });
        clone.selections.clone_state(&self.selections);
        clone.scroll_manager.clone_state(&self.scroll_manager);
        clone.searchable = self.searchable;
        clone
    }

    fn new(
        mode: EditorMode,
        buffer: ModelHandle<MultiBuffer>,
        project: Option<ModelHandle<Project>>,
        get_field_editor_theme: Option<Arc<GetFieldEditorTheme>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let editor_view_id = cx.view_id();
        let display_map = cx.add_model(|cx| {
            let settings = settings::get::<ThemeSettings>(cx);
            let style = build_style(settings, get_field_editor_theme.as_deref(), None, cx);
            DisplayMap::new(
                buffer.clone(),
                style.text.font_id,
                style.text.font_size,
                None,
                2,
                1,
                cx,
            )
        });

        let selections = SelectionsCollection::new(display_map.clone(), buffer.clone());

        let blink_manager = cx.add_model(|cx| BlinkManager::new(CURSOR_BLINK_INTERVAL, cx));

        let soft_wrap_mode_override =
            (mode == EditorMode::SingleLine).then(|| language_settings::SoftWrap::None);

        let mut project_subscriptions = Vec::new();
        if mode == EditorMode::Full {
            if let Some(project) = project.as_ref() {
                if buffer.read(cx).is_singleton() {
                    project_subscriptions.push(cx.observe(project, |_, _, cx| {
                        cx.emit(Event::TitleChanged);
                    }));
                }
                project_subscriptions.push(cx.subscribe(project, |editor, _, event, cx| {
                    if let project::Event::RefreshInlayHints = event {
                        editor.refresh_inlay_hints(InlayHintRefreshReason::RefreshRequested, cx);
                    };
                }));
            }
        }

        let inlay_hint_settings = inlay_hint_settings(
            selections.newest_anchor().head(),
            &buffer.read(cx).snapshot(cx),
            cx,
        );

        let mut this = Self {
            handle: cx.weak_handle(),
            buffer: buffer.clone(),
            display_map: display_map.clone(),
            selections,
            scroll_manager: ScrollManager::new(),
            columnar_selection_tail: None,
            add_selections_state: None,
            select_next_state: None,
            select_prev_state: None,
            selection_history: Default::default(),
            autoclose_regions: Default::default(),
            snippet_stack: Default::default(),
            select_larger_syntax_node_stack: Vec::new(),
            ime_transaction: Default::default(),
            active_diagnostics: None,
            soft_wrap_mode_override,
            get_field_editor_theme,
            project,
            focused: false,
            blink_manager: blink_manager.clone(),
            show_local_selections: true,
            mode,
            replica_id_mapping: None,
            show_gutter: mode == EditorMode::Full,
            show_wrap_guides: None,
            placeholder_text: None,
            highlighted_rows: None,
            background_highlights: Default::default(),
            nav_history: None,
            context_menu: None,
            mouse_context_menu: cx
                .add_view(|cx| context_menu::ContextMenu::new(editor_view_id, cx)),
            completion_tasks: Default::default(),
            next_completion_id: 0,
            next_inlay_id: 0,
            available_code_actions: Default::default(),
            code_actions_task: Default::default(),
            document_highlights_task: Default::default(),
            pending_rename: Default::default(),
            searchable: true,
            override_text_style: None,
            cursor_shape: Default::default(),
            autoindent_mode: Some(AutoindentMode::EachLine),
            collapse_matches: false,
            workspace: None,
            keymap_context_layers: Default::default(),
            input_enabled: true,
            read_only: false,
            leader_replica_id: None,
            remote_id: None,
            hover_state: Default::default(),
            link_go_to_definition_state: Default::default(),
            copilot_state: Default::default(),
            inlay_hint_cache: InlayHintCache::new(inlay_hint_settings),
            gutter_hovered: false,
            pixel_position_of_newest_cursor: None,
            _subscriptions: vec![
                cx.observe(&buffer, Self::on_buffer_changed),
                cx.subscribe(&buffer, Self::on_buffer_event),
                cx.observe(&display_map, Self::on_display_map_changed),
                cx.observe(&blink_manager, |_, _, cx| cx.notify()),
                cx.observe_global::<SettingsStore, _>(Self::settings_changed),
                cx.observe_window_activation(|editor, active, cx| {
                    editor.blink_manager.update(cx, |blink_manager, cx| {
                        if active {
                            blink_manager.enable(cx);
                        } else {
                            blink_manager.show_cursor(cx);
                            blink_manager.disable(cx);
                        }
                    });
                }),
            ],
        };

        this._subscriptions.extend(project_subscriptions);

        this.end_selection(cx);
        this.scroll_manager.show_scrollbar(cx);

        let editor_created_event = EditorCreated(cx.handle());
        cx.emit_global(editor_created_event);

        if mode == EditorMode::Full {
            let should_auto_hide_scrollbars = cx.platform().should_auto_hide_scrollbars();
            cx.set_global(ScrollbarAutoHide(should_auto_hide_scrollbars));
        }

        this.report_editor_event("open", None, cx);
        this
    }

    pub fn new_file(
        workspace: &mut Workspace,
        _: &workspace::NewFile,
        cx: &mut ViewContext<Workspace>,
    ) {
        let project = workspace.project().clone();
        if project.read(cx).is_remote() {
            cx.propagate_action();
        } else if let Some(buffer) = project
            .update(cx, |project, cx| project.create_buffer("", None, cx))
            .log_err()
        {
            workspace.add_item(
                Box::new(cx.add_view(|cx| Editor::for_buffer(buffer, Some(project.clone()), cx))),
                cx,
            );
        }
    }

    pub fn replica_id(&self, cx: &AppContext) -> ReplicaId {
        self.buffer.read(cx).replica_id()
    }

    pub fn leader_replica_id(&self) -> Option<ReplicaId> {
        self.leader_replica_id
    }

    pub fn buffer(&self) -> &ModelHandle<MultiBuffer> {
        &self.buffer
    }

    fn workspace(&self, cx: &AppContext) -> Option<ViewHandle<Workspace>> {
        self.workspace.as_ref()?.0.upgrade(cx)
    }

    pub fn title<'a>(&self, cx: &'a AppContext) -> Cow<'a, str> {
        self.buffer().read(cx).title(cx)
    }

    pub fn snapshot(&mut self, cx: &mut WindowContext) -> EditorSnapshot {
        EditorSnapshot {
            mode: self.mode,
            show_gutter: self.show_gutter,
            display_snapshot: self.display_map.update(cx, |map, cx| map.snapshot(cx)),
            scroll_anchor: self.scroll_manager.anchor(),
            ongoing_scroll: self.scroll_manager.ongoing_scroll(),
            placeholder_text: self.placeholder_text.clone(),
            is_focused: self
                .handle
                .upgrade(cx)
                .map_or(false, |handle| handle.is_focused(cx)),
        }
    }

    pub fn language_at<'a, T: ToOffset>(
        &self,
        point: T,
        cx: &'a AppContext,
    ) -> Option<Arc<Language>> {
        self.buffer.read(cx).language_at(point, cx)
    }

    pub fn file_at<'a, T: ToOffset>(&self, point: T, cx: &'a AppContext) -> Option<Arc<dyn File>> {
        self.buffer.read(cx).read(cx).file_at(point).cloned()
    }

    pub fn active_excerpt(
        &self,
        cx: &AppContext,
    ) -> Option<(ExcerptId, ModelHandle<Buffer>, Range<text::Anchor>)> {
        self.buffer
            .read(cx)
            .excerpt_containing(self.selections.newest_anchor().head(), cx)
    }

    pub fn style(&self, cx: &AppContext) -> EditorStyle {
        build_style(
            settings::get::<ThemeSettings>(cx),
            self.get_field_editor_theme.as_deref(),
            self.override_text_style.as_deref(),
            cx,
        )
    }

    pub fn mode(&self) -> EditorMode {
        self.mode
    }

    pub fn set_placeholder_text(
        &mut self,
        placeholder_text: impl Into<Arc<str>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.placeholder_text = Some(placeholder_text.into());
        cx.notify();
    }

    pub fn set_cursor_shape(&mut self, cursor_shape: CursorShape, cx: &mut ViewContext<Self>) {
        self.cursor_shape = cursor_shape;
        cx.notify();
    }

    pub fn set_collapse_matches(&mut self, collapse_matches: bool) {
        self.collapse_matches = collapse_matches;
    }

    pub fn range_for_match<T: std::marker::Copy>(&self, range: &Range<T>) -> Range<T> {
        if self.collapse_matches {
            return range.start..range.start;
        }
        range.clone()
    }

    pub fn set_clip_at_line_ends(&mut self, clip: bool, cx: &mut ViewContext<Self>) {
        if self.display_map.read(cx).clip_at_line_ends != clip {
            self.display_map
                .update(cx, |map, _| map.clip_at_line_ends = clip);
        }
    }

    pub fn set_keymap_context_layer<Tag: 'static>(
        &mut self,
        context: KeymapContext,
        cx: &mut ViewContext<Self>,
    ) {
        self.keymap_context_layers
            .insert(TypeId::of::<Tag>(), context);
        cx.notify();
    }

    pub fn remove_keymap_context_layer<Tag: 'static>(&mut self, cx: &mut ViewContext<Self>) {
        self.keymap_context_layers.remove(&TypeId::of::<Tag>());
        cx.notify();
    }

    pub fn set_input_enabled(&mut self, input_enabled: bool) {
        self.input_enabled = input_enabled;
    }

    pub fn set_autoindent(&mut self, autoindent: bool) {
        if autoindent {
            self.autoindent_mode = Some(AutoindentMode::EachLine);
        } else {
            self.autoindent_mode = None;
        }
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    pub fn set_field_editor_style(
        &mut self,
        style: Option<Arc<GetFieldEditorTheme>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.get_field_editor_theme = style;
        cx.notify();
    }

    pub fn replica_id_map(&self) -> Option<&HashMap<ReplicaId, ReplicaId>> {
        self.replica_id_mapping.as_ref()
    }

    pub fn set_replica_id_map(
        &mut self,
        mapping: Option<HashMap<ReplicaId, ReplicaId>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.replica_id_mapping = mapping;
        cx.notify();
    }

    fn selections_did_change(
        &mut self,
        local: bool,
        old_cursor_position: &Anchor,
        cx: &mut ViewContext<Self>,
    ) {
        if self.focused && self.leader_replica_id.is_none() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.set_active_selections(
                    &self.selections.disjoint_anchors(),
                    self.selections.line_mode,
                    self.cursor_shape,
                    cx,
                )
            });
        }

        let display_map = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        self.add_selections_state = None;
        self.select_next_state = None;
        self.select_prev_state = None;
        self.select_larger_syntax_node_stack.clear();
        self.invalidate_autoclose_regions(&self.selections.disjoint_anchors(), buffer);
        self.snippet_stack
            .invalidate(&self.selections.disjoint_anchors(), buffer);
        self.take_rename(false, cx);

        let new_cursor_position = self.selections.newest_anchor().head();

        self.push_to_nav_history(
            old_cursor_position.clone(),
            Some(new_cursor_position.to_point(buffer)),
            cx,
        );

        if local {
            let new_cursor_position = self.selections.newest_anchor().head();
            let completion_menu = match self.context_menu.as_mut() {
                Some(ContextMenu::Completions(menu)) => Some(menu),
                _ => {
                    self.context_menu.take();
                    None
                }
            };

            if let Some(completion_menu) = completion_menu {
                let cursor_position = new_cursor_position.to_offset(buffer);
                let (word_range, kind) =
                    buffer.surrounding_word(completion_menu.initial_position.clone());
                if kind == Some(CharKind::Word)
                    && word_range.to_inclusive().contains(&cursor_position)
                {
                    let query = Self::completion_query(buffer, cursor_position);
                    cx.background()
                        .block(completion_menu.filter(query.as_deref(), cx.background().clone()));
                    self.show_completions(&ShowCompletions, cx);
                } else {
                    self.hide_context_menu(cx);
                }
            }

            hide_hover(self, cx);

            if old_cursor_position.to_display_point(&display_map).row()
                != new_cursor_position.to_display_point(&display_map).row()
            {
                self.available_code_actions.take();
            }
            self.refresh_code_actions(cx);
            self.refresh_document_highlights(cx);
            refresh_matching_bracket_highlights(self, cx);
            self.discard_copilot_suggestion(cx);
        }

        self.blink_manager.update(cx, BlinkManager::pause_blinking);
        cx.emit(Event::SelectionsChanged { local });
        cx.notify();
    }

    pub fn change_selections<R>(
        &mut self,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
        change: impl FnOnce(&mut MutableSelectionsCollection<'_>) -> R,
    ) -> R {
        let old_cursor_position = self.selections.newest_anchor().head();
        self.push_to_selection_history();

        let (changed, result) = self.selections.change_with(cx, change);

        if changed {
            if let Some(autoscroll) = autoscroll {
                self.request_autoscroll(autoscroll, cx);
            }
            self.selections_did_change(true, &old_cursor_position, cx);
        }

        result
    }

    pub fn edit<I, S, T>(&mut self, edits: I, cx: &mut ViewContext<Self>)
    where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        if self.read_only {
            return;
        }

        self.buffer
            .update(cx, |buffer, cx| buffer.edit(edits, None, cx));
    }

    pub fn edit_with_autoindent<I, S, T>(&mut self, edits: I, cx: &mut ViewContext<Self>)
    where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        if self.read_only {
            return;
        }

        self.buffer.update(cx, |buffer, cx| {
            buffer.edit(edits, self.autoindent_mode.clone(), cx)
        });
    }

    pub fn edit_with_block_indent<I, S, T>(
        &mut self,
        edits: I,
        original_indent_columns: Vec<u32>,
        cx: &mut ViewContext<Self>,
    ) where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        if self.read_only {
            return;
        }

        self.buffer.update(cx, |buffer, cx| {
            buffer.edit(
                edits,
                Some(AutoindentMode::Block {
                    original_indent_columns,
                }),
                cx,
            )
        });
    }

    fn select(&mut self, phase: SelectPhase, cx: &mut ViewContext<Self>) {
        self.hide_context_menu(cx);

        match phase {
            SelectPhase::Begin {
                position,
                add,
                click_count,
            } => self.begin_selection(position, add, click_count, cx),
            SelectPhase::BeginColumnar {
                position,
                goal_column,
            } => self.begin_columnar_selection(position, goal_column, cx),
            SelectPhase::Extend {
                position,
                click_count,
            } => self.extend_selection(position, click_count, cx),
            SelectPhase::Update {
                position,
                goal_column,
                scroll_position,
            } => self.update_selection(position, goal_column, scroll_position, cx),
            SelectPhase::End => self.end_selection(cx),
        }
    }

    fn extend_selection(
        &mut self,
        position: DisplayPoint,
        click_count: usize,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let tail = self.selections.newest::<usize>(cx).tail();
        self.begin_selection(position, false, click_count, cx);

        let position = position.to_offset(&display_map, Bias::Left);
        let tail_anchor = display_map.buffer_snapshot.anchor_before(tail);

        let mut pending_selection = self
            .selections
            .pending_anchor()
            .expect("extend_selection not called with pending selection");
        if position >= tail {
            pending_selection.start = tail_anchor;
        } else {
            pending_selection.end = tail_anchor;
            pending_selection.reversed = true;
        }

        let mut pending_mode = self.selections.pending_mode().unwrap();
        match &mut pending_mode {
            SelectMode::Word(range) | SelectMode::Line(range) => *range = tail_anchor..tail_anchor,
            _ => {}
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.set_pending(pending_selection, pending_mode)
        });
    }

    fn begin_selection(
        &mut self,
        position: DisplayPoint,
        add: bool,
        click_count: usize,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.focused {
            cx.focus_self();
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let newest_selection = self.selections.newest_anchor().clone();
        let position = display_map.clip_point(position, Bias::Left);

        let start;
        let end;
        let mode;
        let auto_scroll;
        match click_count {
            1 => {
                start = buffer.anchor_before(position.to_point(&display_map));
                end = start.clone();
                mode = SelectMode::Character;
                auto_scroll = true;
            }
            2 => {
                let range = movement::surrounding_word(&display_map, position);
                start = buffer.anchor_before(range.start.to_point(&display_map));
                end = buffer.anchor_before(range.end.to_point(&display_map));
                mode = SelectMode::Word(start.clone()..end.clone());
                auto_scroll = true;
            }
            3 => {
                let position = display_map
                    .clip_point(position, Bias::Left)
                    .to_point(&display_map);
                let line_start = display_map.prev_line_boundary(position).0;
                let next_line_start = buffer.clip_point(
                    display_map.next_line_boundary(position).0 + Point::new(1, 0),
                    Bias::Left,
                );
                start = buffer.anchor_before(line_start);
                end = buffer.anchor_before(next_line_start);
                mode = SelectMode::Line(start.clone()..end.clone());
                auto_scroll = true;
            }
            _ => {
                start = buffer.anchor_before(0);
                end = buffer.anchor_before(buffer.len());
                mode = SelectMode::All;
                auto_scroll = false;
            }
        }

        self.change_selections(auto_scroll.then(|| Autoscroll::newest()), cx, |s| {
            if !add {
                s.clear_disjoint();
            } else if click_count > 1 {
                s.delete(newest_selection.id)
            }

            s.set_pending_anchor_range(start..end, mode);
        });
    }

    fn begin_columnar_selection(
        &mut self,
        position: DisplayPoint,
        goal_column: u32,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.focused {
            cx.focus_self();
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let tail = self.selections.newest::<Point>(cx).tail();
        self.columnar_selection_tail = Some(display_map.buffer_snapshot.anchor_before(tail));

        self.select_columns(
            tail.to_display_point(&display_map),
            position,
            goal_column,
            &display_map,
            cx,
        );
    }

    fn update_selection(
        &mut self,
        position: DisplayPoint,
        goal_column: u32,
        scroll_position: Vector2F,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if let Some(tail) = self.columnar_selection_tail.as_ref() {
            let tail = tail.to_display_point(&display_map);
            self.select_columns(tail, position, goal_column, &display_map, cx);
        } else if let Some(mut pending) = self.selections.pending_anchor() {
            let buffer = self.buffer.read(cx).snapshot(cx);
            let head;
            let tail;
            let mode = self.selections.pending_mode().unwrap();
            match &mode {
                SelectMode::Character => {
                    head = position.to_point(&display_map);
                    tail = pending.tail().to_point(&buffer);
                }
                SelectMode::Word(original_range) => {
                    let original_display_range = original_range.start.to_display_point(&display_map)
                        ..original_range.end.to_display_point(&display_map);
                    let original_buffer_range = original_display_range.start.to_point(&display_map)
                        ..original_display_range.end.to_point(&display_map);
                    if movement::is_inside_word(&display_map, position)
                        || original_display_range.contains(&position)
                    {
                        let word_range = movement::surrounding_word(&display_map, position);
                        if word_range.start < original_display_range.start {
                            head = word_range.start.to_point(&display_map);
                        } else {
                            head = word_range.end.to_point(&display_map);
                        }
                    } else {
                        head = position.to_point(&display_map);
                    }

                    if head <= original_buffer_range.start {
                        tail = original_buffer_range.end;
                    } else {
                        tail = original_buffer_range.start;
                    }
                }
                SelectMode::Line(original_range) => {
                    let original_range = original_range.to_point(&display_map.buffer_snapshot);

                    let position = display_map
                        .clip_point(position, Bias::Left)
                        .to_point(&display_map);
                    let line_start = display_map.prev_line_boundary(position).0;
                    let next_line_start = buffer.clip_point(
                        display_map.next_line_boundary(position).0 + Point::new(1, 0),
                        Bias::Left,
                    );

                    if line_start < original_range.start {
                        head = line_start
                    } else {
                        head = next_line_start
                    }

                    if head <= original_range.start {
                        tail = original_range.end;
                    } else {
                        tail = original_range.start;
                    }
                }
                SelectMode::All => {
                    return;
                }
            };

            if head < tail {
                pending.start = buffer.anchor_before(head);
                pending.end = buffer.anchor_before(tail);
                pending.reversed = true;
            } else {
                pending.start = buffer.anchor_before(tail);
                pending.end = buffer.anchor_before(head);
                pending.reversed = false;
            }

            self.change_selections(None, cx, |s| {
                s.set_pending(pending, mode);
            });
        } else {
            error!("update_selection dispatched with no pending selection");
            return;
        }

        self.set_scroll_position(scroll_position, cx);
        cx.notify();
    }

    fn end_selection(&mut self, cx: &mut ViewContext<Self>) {
        self.columnar_selection_tail.take();
        if self.selections.pending_anchor().is_some() {
            let selections = self.selections.all::<usize>(cx);
            self.change_selections(None, cx, |s| {
                s.select(selections);
                s.clear_pending();
            });
        }
    }

    fn select_columns(
        &mut self,
        tail: DisplayPoint,
        head: DisplayPoint,
        goal_column: u32,
        display_map: &DisplaySnapshot,
        cx: &mut ViewContext<Self>,
    ) {
        let start_row = cmp::min(tail.row(), head.row());
        let end_row = cmp::max(tail.row(), head.row());
        let start_column = cmp::min(tail.column(), goal_column);
        let end_column = cmp::max(tail.column(), goal_column);
        let reversed = start_column < tail.column();

        let selection_ranges = (start_row..=end_row)
            .filter_map(|row| {
                if start_column <= display_map.line_len(row) && !display_map.is_block_line(row) {
                    let start = display_map
                        .clip_point(DisplayPoint::new(row, start_column), Bias::Left)
                        .to_point(display_map);
                    let end = display_map
                        .clip_point(DisplayPoint::new(row, end_column), Bias::Right)
                        .to_point(display_map);
                    if reversed {
                        Some(end..start)
                    } else {
                        Some(start..end)
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        self.change_selections(None, cx, |s| {
            s.select_ranges(selection_ranges);
        });
        cx.notify();
    }

    pub fn has_pending_nonempty_selection(&self) -> bool {
        let pending_nonempty_selection = match self.selections.pending_anchor() {
            Some(Selection { start, end, .. }) => start != end,
            None => false,
        };
        pending_nonempty_selection || self.columnar_selection_tail.is_some()
    }

    pub fn has_pending_selection(&self) -> bool {
        self.selections.pending_anchor().is_some() || self.columnar_selection_tail.is_some()
    }

    pub fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if self.take_rename(false, cx).is_some() {
            return;
        }

        if hide_hover(self, cx) {
            return;
        }

        if self.hide_context_menu(cx).is_some() {
            return;
        }

        if self.discard_copilot_suggestion(cx) {
            return;
        }

        if self.snippet_stack.pop().is_some() {
            return;
        }

        if self.mode == EditorMode::Full {
            if self.active_diagnostics.is_some() {
                self.dismiss_diagnostics(cx);
                return;
            }

            if self.change_selections(Some(Autoscroll::fit()), cx, |s| s.try_cancel()) {
                return;
            }
        }

        cx.propagate_action();
    }

    pub fn handle_input(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        let text: Arc<str> = text.into();

        if self.read_only {
            return;
        }

        let selections = self.selections.all_adjusted(cx);
        let mut brace_inserted = false;
        let mut edits = Vec::new();
        let mut new_selections = Vec::with_capacity(selections.len());
        let mut new_autoclose_regions = Vec::new();
        let snapshot = self.buffer.read(cx).read(cx);

        for (selection, autoclose_region) in
            self.selections_with_autoclose_regions(selections, &snapshot)
        {
            if let Some(scope) = snapshot.language_scope_at(selection.head()) {
                // Determine if the inserted text matches the opening or closing
                // bracket of any of this language's bracket pairs.
                let mut bracket_pair = None;
                let mut is_bracket_pair_start = false;
                for (pair, enabled) in scope.brackets() {
                    if enabled && pair.close && pair.start.ends_with(text.as_ref()) {
                        bracket_pair = Some(pair.clone());
                        is_bracket_pair_start = true;
                        break;
                    } else if pair.end.as_str() == text.as_ref() {
                        bracket_pair = Some(pair.clone());
                        break;
                    }
                }

                if let Some(bracket_pair) = bracket_pair {
                    if selection.is_empty() {
                        if is_bracket_pair_start {
                            let prefix_len = bracket_pair.start.len() - text.len();

                            // If the inserted text is a suffix of an opening bracket and the
                            // selection is preceded by the rest of the opening bracket, then
                            // insert the closing bracket.
                            let following_text_allows_autoclose = snapshot
                                .chars_at(selection.start)
                                .next()
                                .map_or(true, |c| scope.should_autoclose_before(c));
                            let preceding_text_matches_prefix = prefix_len == 0
                                || (selection.start.column >= (prefix_len as u32)
                                    && snapshot.contains_str_at(
                                        Point::new(
                                            selection.start.row,
                                            selection.start.column - (prefix_len as u32),
                                        ),
                                        &bracket_pair.start[..prefix_len],
                                    ));
                            if following_text_allows_autoclose && preceding_text_matches_prefix {
                                let anchor = snapshot.anchor_before(selection.end);
                                new_selections.push((selection.map(|_| anchor), text.len()));
                                new_autoclose_regions.push((
                                    anchor,
                                    text.len(),
                                    selection.id,
                                    bracket_pair.clone(),
                                ));
                                edits.push((
                                    selection.range(),
                                    format!("{}{}", text, bracket_pair.end).into(),
                                ));
                                brace_inserted = true;
                                continue;
                            }
                        }

                        if let Some(region) = autoclose_region {
                            // If the selection is followed by an auto-inserted closing bracket,
                            // then don't insert that closing bracket again; just move the selection
                            // past the closing bracket.
                            let should_skip = selection.end == region.range.end.to_point(&snapshot)
                                && text.as_ref() == region.pair.end.as_str();
                            if should_skip {
                                let anchor = snapshot.anchor_after(selection.end);
                                new_selections
                                    .push((selection.map(|_| anchor), region.pair.end.len()));
                                continue;
                            }
                        }
                    }
                    // If an opening bracket is 1 character long and is typed while
                    // text is selected, then surround that text with the bracket pair.
                    else if is_bracket_pair_start && bracket_pair.start.chars().count() == 1 {
                        edits.push((selection.start..selection.start, text.clone()));
                        edits.push((
                            selection.end..selection.end,
                            bracket_pair.end.as_str().into(),
                        ));
                        brace_inserted = true;
                        new_selections.push((
                            Selection {
                                id: selection.id,
                                start: snapshot.anchor_after(selection.start),
                                end: snapshot.anchor_before(selection.end),
                                reversed: selection.reversed,
                                goal: selection.goal,
                            },
                            0,
                        ));
                        continue;
                    }
                }
            }

            // If not handling any auto-close operation, then just replace the selected
            // text with the given input and move the selection to the end of the
            // newly inserted text.
            let anchor = snapshot.anchor_after(selection.end);
            new_selections.push((selection.map(|_| anchor), 0));
            edits.push((selection.start..selection.end, text.clone()));
        }

        drop(snapshot);
        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, this.autoindent_mode.clone(), cx);
            });

            let new_anchor_selections = new_selections.iter().map(|e| &e.0);
            let new_selection_deltas = new_selections.iter().map(|e| e.1);
            let snapshot = this.buffer.read(cx).read(cx);
            let new_selections = resolve_multiple::<usize, _>(new_anchor_selections, &snapshot)
                .zip(new_selection_deltas)
                .map(|(selection, delta)| selection.map(|e| e + delta))
                .collect::<Vec<_>>();

            let mut i = 0;
            for (position, delta, selection_id, pair) in new_autoclose_regions {
                let position = position.to_offset(&snapshot) + delta;
                let start = snapshot.anchor_before(position);
                let end = snapshot.anchor_after(position);
                while let Some(existing_state) = this.autoclose_regions.get(i) {
                    match existing_state.range.start.cmp(&start, &snapshot) {
                        Ordering::Less => i += 1,
                        Ordering::Greater => break,
                        Ordering::Equal => match end.cmp(&existing_state.range.end, &snapshot) {
                            Ordering::Less => i += 1,
                            Ordering::Equal => break,
                            Ordering::Greater => break,
                        },
                    }
                }
                this.autoclose_regions.insert(
                    i,
                    AutocloseRegion {
                        selection_id,
                        range: start..end,
                        pair,
                    },
                );
            }

            drop(snapshot);
            let had_active_copilot_suggestion = this.has_active_copilot_suggestion(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(new_selections));

            if !brace_inserted && settings::get::<EditorSettings>(cx).use_on_type_format {
                if let Some(on_type_format_task) =
                    this.trigger_on_type_formatting(text.to_string(), cx)
                {
                    on_type_format_task.detach_and_log_err(cx);
                }
            }

            if had_active_copilot_suggestion {
                this.refresh_copilot_suggestions(true, cx);
                if !this.has_active_copilot_suggestion(cx) {
                    this.trigger_completion_on_input(&text, cx);
                }
            } else {
                this.trigger_completion_on_input(&text, cx);
                this.refresh_copilot_suggestions(true, cx);
            }
        });
    }

    pub fn newline(&mut self, _: &Newline, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            let (edits, selection_fixup_info): (Vec<_>, Vec<_>) = {
                let selections = this.selections.all::<usize>(cx);
                let multi_buffer = this.buffer.read(cx);
                let buffer = multi_buffer.snapshot(cx);
                selections
                    .iter()
                    .map(|selection| {
                        let start_point = selection.start.to_point(&buffer);
                        let mut indent = buffer.indent_size_for_line(start_point.row);
                        indent.len = cmp::min(indent.len, start_point.column);
                        let start = selection.start;
                        let end = selection.end;
                        let is_cursor = start == end;
                        let language_scope = buffer.language_scope_at(start);
                        let (comment_delimiter, insert_extra_newline) = if let Some(language) =
                            &language_scope
                        {
                            let leading_whitespace_len = buffer
                                .reversed_chars_at(start)
                                .take_while(|c| c.is_whitespace() && *c != '\n')
                                .map(|c| c.len_utf8())
                                .sum::<usize>();

                            let trailing_whitespace_len = buffer
                                .chars_at(end)
                                .take_while(|c| c.is_whitespace() && *c != '\n')
                                .map(|c| c.len_utf8())
                                .sum::<usize>();

                            let insert_extra_newline =
                                language.brackets().any(|(pair, enabled)| {
                                    let pair_start = pair.start.trim_end();
                                    let pair_end = pair.end.trim_start();

                                    enabled
                                        && pair.newline
                                        && buffer.contains_str_at(
                                            end + trailing_whitespace_len,
                                            pair_end,
                                        )
                                        && buffer.contains_str_at(
                                            (start - leading_whitespace_len)
                                                .saturating_sub(pair_start.len()),
                                            pair_start,
                                        )
                                });
                            // Comment extension on newline is allowed only for cursor selections
                            let comment_delimiter = language.line_comment_prefix().filter(|_| {
                                let is_comment_extension_enabled =
                                    multi_buffer.settings_at(0, cx).extend_comment_on_newline;
                                is_cursor && is_comment_extension_enabled
                            });
                            let comment_delimiter = if let Some(delimiter) = comment_delimiter {
                                buffer
                                    .buffer_line_for_row(start_point.row)
                                    .is_some_and(|(snapshot, range)| {
                                        let mut index_of_first_non_whitespace = 0;
                                        let line_starts_with_comment = snapshot
                                            .chars_for_range(range)
                                            .skip_while(|c| {
                                                let should_skip = c.is_whitespace();
                                                if should_skip {
                                                    index_of_first_non_whitespace += 1;
                                                }
                                                should_skip
                                            })
                                            .take(delimiter.len())
                                            .eq(delimiter.chars());
                                        let cursor_is_placed_after_comment_marker =
                                            index_of_first_non_whitespace + delimiter.len()
                                                <= start_point.column as usize;
                                        line_starts_with_comment
                                            && cursor_is_placed_after_comment_marker
                                    })
                                    .then(|| delimiter.clone())
                            } else {
                                None
                            };
                            (comment_delimiter, insert_extra_newline)
                        } else {
                            (None, false)
                        };

                        let capacity_for_delimiter = comment_delimiter
                            .as_deref()
                            .map(str::len)
                            .unwrap_or_default();
                        let mut new_text =
                            String::with_capacity(1 + capacity_for_delimiter + indent.len as usize);
                        new_text.push_str("\n");
                        new_text.extend(indent.chars());
                        if let Some(delimiter) = &comment_delimiter {
                            new_text.push_str(&delimiter);
                        }
                        if insert_extra_newline {
                            new_text = new_text.repeat(2);
                        }

                        let anchor = buffer.anchor_after(end);
                        let new_selection = selection.map(|_| anchor);
                        (
                            (start..end, new_text),
                            (insert_extra_newline, new_selection),
                        )
                    })
                    .unzip()
            };

            this.edit_with_autoindent(edits, cx);
            let buffer = this.buffer.read(cx).snapshot(cx);
            let new_selections = selection_fixup_info
                .into_iter()
                .map(|(extra_newline_inserted, new_selection)| {
                    let mut cursor = new_selection.end.to_point(&buffer);
                    if extra_newline_inserted {
                        cursor.row -= 1;
                        cursor.column = buffer.line_len(cursor.row);
                    }
                    new_selection.map(|_| cursor)
                })
                .collect();

            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(new_selections));
            this.refresh_copilot_suggestions(true, cx);
        });
    }

    pub fn newline_above(&mut self, _: &NewlineAbove, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);

        let mut edits = Vec::new();
        let mut rows = Vec::new();
        let mut rows_inserted = 0;

        for selection in self.selections.all_adjusted(cx) {
            let cursor = selection.head();
            let row = cursor.row;

            let start_of_line = snapshot.clip_point(Point::new(row, 0), Bias::Left);

            let newline = "\n".to_string();
            edits.push((start_of_line..start_of_line, newline));

            rows.push(row + rows_inserted);
            rows_inserted += 1;
        }

        self.transact(cx, |editor, cx| {
            editor.edit(edits, cx);

            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let mut index = 0;
                s.move_cursors_with(|map, _, _| {
                    let row = rows[index];
                    index += 1;

                    let point = Point::new(row, 0);
                    let boundary = map.next_line_boundary(point).1;
                    let clipped = map.clip_point(boundary, Bias::Left);

                    (clipped, SelectionGoal::None)
                });
            });

            let mut indent_edits = Vec::new();
            let multibuffer_snapshot = editor.buffer.read(cx).snapshot(cx);
            for row in rows {
                let indents = multibuffer_snapshot.suggested_indents(row..row + 1, cx);
                for (row, indent) in indents {
                    if indent.len == 0 {
                        continue;
                    }

                    let text = match indent.kind {
                        IndentKind::Space => " ".repeat(indent.len as usize),
                        IndentKind::Tab => "\t".repeat(indent.len as usize),
                    };
                    let point = Point::new(row, 0);
                    indent_edits.push((point..point, text));
                }
            }
            editor.edit(indent_edits, cx);
        });
    }

    pub fn newline_below(&mut self, _: &NewlineBelow, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);

        let mut edits = Vec::new();
        let mut rows = Vec::new();
        let mut rows_inserted = 0;

        for selection in self.selections.all_adjusted(cx) {
            let cursor = selection.head();
            let row = cursor.row;

            let point = Point::new(row + 1, 0);
            let start_of_line = snapshot.clip_point(point, Bias::Left);

            let newline = "\n".to_string();
            edits.push((start_of_line..start_of_line, newline));

            rows_inserted += 1;
            rows.push(row + rows_inserted);
        }

        self.transact(cx, |editor, cx| {
            editor.edit(edits, cx);

            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let mut index = 0;
                s.move_cursors_with(|map, _, _| {
                    let row = rows[index];
                    index += 1;

                    let point = Point::new(row, 0);
                    let boundary = map.next_line_boundary(point).1;
                    let clipped = map.clip_point(boundary, Bias::Left);

                    (clipped, SelectionGoal::None)
                });
            });

            let mut indent_edits = Vec::new();
            let multibuffer_snapshot = editor.buffer.read(cx).snapshot(cx);
            for row in rows {
                let indents = multibuffer_snapshot.suggested_indents(row..row + 1, cx);
                for (row, indent) in indents {
                    if indent.len == 0 {
                        continue;
                    }

                    let text = match indent.kind {
                        IndentKind::Space => " ".repeat(indent.len as usize),
                        IndentKind::Tab => "\t".repeat(indent.len as usize),
                    };
                    let point = Point::new(row, 0);
                    indent_edits.push((point..point, text));
                }
            }
            editor.edit(indent_edits, cx);
        });
    }

    pub fn insert(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.insert_with_autoindent_mode(
            text,
            Some(AutoindentMode::Block {
                original_indent_columns: Vec::new(),
            }),
            cx,
        );
    }

    fn insert_with_autoindent_mode(
        &mut self,
        text: &str,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut ViewContext<Self>,
    ) {
        if self.read_only {
            return;
        }

        let text: Arc<str> = text.into();
        self.transact(cx, |this, cx| {
            let old_selections = this.selections.all_adjusted(cx);
            let selection_anchors = this.buffer.update(cx, |buffer, cx| {
                let anchors = {
                    let snapshot = buffer.read(cx);
                    old_selections
                        .iter()
                        .map(|s| {
                            let anchor = snapshot.anchor_after(s.head());
                            s.map(|_| anchor)
                        })
                        .collect::<Vec<_>>()
                };
                buffer.edit(
                    old_selections
                        .iter()
                        .map(|s| (s.start..s.end, text.clone())),
                    autoindent_mode,
                    cx,
                );
                anchors
            });

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_anchors(selection_anchors);
            })
        });
    }

    fn trigger_completion_on_input(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        if !settings::get::<EditorSettings>(cx).show_completions_on_input {
            return;
        }

        let selection = self.selections.newest_anchor();
        if self
            .buffer
            .read(cx)
            .is_completion_trigger(selection.head(), text, cx)
        {
            self.show_completions(&ShowCompletions, cx);
        } else {
            self.hide_context_menu(cx);
        }
    }

    /// If any empty selections is touching the start of its innermost containing autoclose
    /// region, expand it to select the brackets.
    fn select_autoclose_pair(&mut self, cx: &mut ViewContext<Self>) {
        let selections = self.selections.all::<usize>(cx);
        let buffer = self.buffer.read(cx).read(cx);
        let mut new_selections = Vec::new();
        for (mut selection, region) in self.selections_with_autoclose_regions(selections, &buffer) {
            if let (Some(region), true) = (region, selection.is_empty()) {
                let mut range = region.range.to_offset(&buffer);
                if selection.start == range.start {
                    if range.start >= region.pair.start.len() {
                        range.start -= region.pair.start.len();
                        if buffer.contains_str_at(range.start, &region.pair.start) {
                            if buffer.contains_str_at(range.end, &region.pair.end) {
                                range.end += region.pair.end.len();
                                selection.start = range.start;
                                selection.end = range.end;
                            }
                        }
                    }
                }
            }
            new_selections.push(selection);
        }

        drop(buffer);
        self.change_selections(None, cx, |selections| selections.select(new_selections));
    }

    /// Iterate the given selections, and for each one, find the smallest surrounding
    /// autoclose region. This uses the ordering of the selections and the autoclose
    /// regions to avoid repeated comparisons.
    fn selections_with_autoclose_regions<'a, D: ToOffset + Clone>(
        &'a self,
        selections: impl IntoIterator<Item = Selection<D>>,
        buffer: &'a MultiBufferSnapshot,
    ) -> impl Iterator<Item = (Selection<D>, Option<&'a AutocloseRegion>)> {
        let mut i = 0;
        let mut regions = self.autoclose_regions.as_slice();
        selections.into_iter().map(move |selection| {
            let range = selection.start.to_offset(buffer)..selection.end.to_offset(buffer);

            let mut enclosing = None;
            while let Some(pair_state) = regions.get(i) {
                if pair_state.range.end.to_offset(buffer) < range.start {
                    regions = &regions[i + 1..];
                    i = 0;
                } else if pair_state.range.start.to_offset(buffer) > range.end {
                    break;
                } else if pair_state.selection_id == selection.id {
                    enclosing = Some(pair_state);
                    i += 1;
                }
            }

            (selection.clone(), enclosing)
        })
    }

    /// Remove any autoclose regions that no longer contain their selection.
    fn invalidate_autoclose_regions(
        &mut self,
        mut selections: &[Selection<Anchor>],
        buffer: &MultiBufferSnapshot,
    ) {
        self.autoclose_regions.retain(|state| {
            let mut i = 0;
            while let Some(selection) = selections.get(i) {
                if selection.end.cmp(&state.range.start, buffer).is_lt() {
                    selections = &selections[1..];
                    continue;
                }
                if selection.start.cmp(&state.range.end, buffer).is_gt() {
                    break;
                }
                if selection.id == state.selection_id {
                    return true;
                } else {
                    i += 1;
                }
            }
            false
        });
    }
    fn completion_query(buffer: &MultiBufferSnapshot, position: impl ToOffset) -> Option<String> {
        let offset = position.to_offset(buffer);
        let (word_range, kind) = buffer.surrounding_word(offset);
        if offset > word_range.start && kind == Some(CharKind::Word) {
            Some(
                buffer
                    .text_for_range(word_range.start..offset)
                    .collect::<String>(),
            )
        } else {
            None
        }
    }

    pub fn toggle_inlay_hints(&mut self, _: &ToggleInlayHints, cx: &mut ViewContext<Self>) {
        self.refresh_inlay_hints(
            InlayHintRefreshReason::Toggle(!self.inlay_hint_cache.enabled),
            cx,
        );
    }

    pub fn inlay_hints_enabled(&self) -> bool {
        self.inlay_hint_cache.enabled
    }

    fn refresh_inlay_hints(&mut self, reason: InlayHintRefreshReason, cx: &mut ViewContext<Self>) {
        if self.project.is_none() || self.mode != EditorMode::Full {
            return;
        }

        let reason_description = reason.description();
        let (invalidate_cache, required_languages) = match reason {
            InlayHintRefreshReason::Toggle(enabled) => {
                self.inlay_hint_cache.enabled = enabled;
                if enabled {
                    (InvalidationStrategy::RefreshRequested, None)
                } else {
                    self.inlay_hint_cache.clear();
                    self.splice_inlay_hints(
                        self.visible_inlay_hints(cx)
                            .iter()
                            .map(|inlay| inlay.id)
                            .collect(),
                        Vec::new(),
                        cx,
                    );
                    return;
                }
            }
            InlayHintRefreshReason::SettingsChange(new_settings) => {
                match self.inlay_hint_cache.update_settings(
                    &self.buffer,
                    new_settings,
                    self.visible_inlay_hints(cx),
                    cx,
                ) {
                    ControlFlow::Break(Some(InlaySplice {
                        to_remove,
                        to_insert,
                    })) => {
                        self.splice_inlay_hints(to_remove, to_insert, cx);
                        return;
                    }
                    ControlFlow::Break(None) => return,
                    ControlFlow::Continue(()) => (InvalidationStrategy::RefreshRequested, None),
                }
            }
            InlayHintRefreshReason::ExcerptsRemoved(excerpts_removed) => {
                if let Some(InlaySplice {
                    to_remove,
                    to_insert,
                }) = self.inlay_hint_cache.remove_excerpts(excerpts_removed)
                {
                    self.splice_inlay_hints(to_remove, to_insert, cx);
                }
                return;
            }
            InlayHintRefreshReason::NewLinesShown => (InvalidationStrategy::None, None),
            InlayHintRefreshReason::BufferEdited(buffer_languages) => {
                (InvalidationStrategy::BufferEdited, Some(buffer_languages))
            }
            InlayHintRefreshReason::RefreshRequested => {
                (InvalidationStrategy::RefreshRequested, None)
            }
        };

        if let Some(InlaySplice {
            to_remove,
            to_insert,
        }) = self.inlay_hint_cache.spawn_hint_refresh(
            reason_description,
            self.excerpt_visible_offsets(required_languages.as_ref(), cx),
            invalidate_cache,
            cx,
        ) {
            self.splice_inlay_hints(to_remove, to_insert, cx);
        }
    }

    fn visible_inlay_hints(&self, cx: &ViewContext<'_, '_, Editor>) -> Vec<Inlay> {
        self.display_map
            .read(cx)
            .current_inlays()
            .filter(move |inlay| {
                Some(inlay.id) != self.copilot_state.suggestion.as_ref().map(|h| h.id)
            })
            .cloned()
            .collect()
    }

    pub fn excerpt_visible_offsets(
        &self,
        restrict_to_languages: Option<&HashSet<Arc<Language>>>,
        cx: &mut ViewContext<'_, '_, Editor>,
    ) -> HashMap<ExcerptId, (ModelHandle<Buffer>, Global, Range<usize>)> {
        let multi_buffer = self.buffer().read(cx);
        let multi_buffer_snapshot = multi_buffer.snapshot(cx);
        let multi_buffer_visible_start = self
            .scroll_manager
            .anchor()
            .anchor
            .to_point(&multi_buffer_snapshot);
        let multi_buffer_visible_end = multi_buffer_snapshot.clip_point(
            multi_buffer_visible_start
                + Point::new(self.visible_line_count().unwrap_or(0.).ceil() as u32, 0),
            Bias::Left,
        );
        let multi_buffer_visible_range = multi_buffer_visible_start..multi_buffer_visible_end;
        multi_buffer
            .range_to_buffer_ranges(multi_buffer_visible_range, cx)
            .into_iter()
            .filter(|(_, excerpt_visible_range, _)| !excerpt_visible_range.is_empty())
            .filter_map(|(buffer_handle, excerpt_visible_range, excerpt_id)| {
                let buffer = buffer_handle.read(cx);
                let language = buffer.language()?;
                if let Some(restrict_to_languages) = restrict_to_languages {
                    if !restrict_to_languages.contains(language) {
                        return None;
                    }
                }
                Some((
                    excerpt_id,
                    (
                        buffer_handle,
                        buffer.version().clone(),
                        excerpt_visible_range,
                    ),
                ))
            })
            .collect()
    }

    fn splice_inlay_hints(
        &self,
        to_remove: Vec<InlayId>,
        to_insert: Vec<Inlay>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map.update(cx, |display_map, cx| {
            display_map.splice_inlays(to_remove, to_insert, cx);
        });
        cx.notify();
    }

    fn trigger_on_type_formatting(
        &self,
        input: String,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if input.len() != 1 {
            return None;
        }

        let project = self.project.as_ref()?;
        let position = self.selections.newest_anchor().head();
        let (buffer, buffer_position) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(position.clone(), cx)?;

        // OnTypeFormatting returns a list of edits, no need to pass them between Zed instances,
        // hence we do LSP request & edit on host side only — add formats to host's history.
        let push_to_lsp_host_history = true;
        // If this is not the host, append its history with new edits.
        let push_to_client_history = project.read(cx).is_remote();

        let on_type_formatting = project.update(cx, |project, cx| {
            project.on_type_format(
                buffer.clone(),
                buffer_position,
                input,
                push_to_lsp_host_history,
                cx,
            )
        });
        Some(cx.spawn(|editor, mut cx| async move {
            if let Some(transaction) = on_type_formatting.await? {
                if push_to_client_history {
                    buffer.update(&mut cx, |buffer, _| {
                        buffer.push_transaction(transaction, Instant::now());
                    });
                }
                editor.update(&mut cx, |editor, cx| {
                    editor.refresh_document_highlights(cx);
                })?;
            }
            Ok(())
        }))
    }

    fn show_completions(&mut self, _: &ShowCompletions, cx: &mut ViewContext<Self>) {
        if self.pending_rename.is_some() {
            return;
        }

        let project = if let Some(project) = self.project.clone() {
            project
        } else {
            return;
        };

        let position = self.selections.newest_anchor().head();
        let (buffer, buffer_position) = if let Some(output) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(position.clone(), cx)
        {
            output
        } else {
            return;
        };

        let query = Self::completion_query(&self.buffer.read(cx).read(cx), position.clone());
        let completions = project.update(cx, |project, cx| {
            project.completions(&buffer, buffer_position, cx)
        });

        let id = post_inc(&mut self.next_completion_id);
        let project = self.project.clone();
        let task = cx.spawn(|this, mut cx| {
            async move {
                let menu = if let Some(completions) = completions.await.log_err() {
                    let mut menu = CompletionsMenu {
                        id,
                        initial_position: position,
                        match_candidates: completions
                            .iter()
                            .enumerate()
                            .map(|(id, completion)| {
                                StringMatchCandidate::new(
                                    id,
                                    completion.label.text[completion.label.filter_range.clone()]
                                        .into(),
                                )
                            })
                            .collect(),
                        buffer,
                        project,
                        completions: completions.into(),
                        matches: Vec::new().into(),
                        selected_item: 0,
                        list: Default::default(),
                    };
                    menu.filter(query.as_deref(), cx.background()).await;
                    if menu.matches.is_empty() {
                        None
                    } else {
                        Some(menu)
                    }
                } else {
                    None
                };

                this.update(&mut cx, |this, cx| {
                    this.completion_tasks.retain(|(task_id, _)| *task_id > id);

                    match this.context_menu.as_ref() {
                        None => {}
                        Some(ContextMenu::Completions(prev_menu)) => {
                            if prev_menu.id > id {
                                return;
                            }
                        }
                        _ => return,
                    }

                    if this.focused && menu.is_some() {
                        let menu = menu.unwrap();
                        this.show_context_menu(ContextMenu::Completions(menu), cx);
                    } else if this.completion_tasks.is_empty() {
                        // If there are no more completion tasks and the last menu was
                        // empty, we should hide it. If it was already hidden, we should
                        // also show the copilot suggestion when available.
                        if this.hide_context_menu(cx).is_none() {
                            this.update_visible_copilot_suggestion(cx);
                        }
                    }
                })?;

                Ok::<_, anyhow::Error>(())
            }
            .log_err()
        });
        self.completion_tasks.push((id, task));
    }

    pub fn confirm_completion(
        &mut self,
        action: &ConfirmCompletion,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        use language::ToOffset as _;

        let completions_menu = if let ContextMenu::Completions(menu) = self.hide_context_menu(cx)? {
            menu
        } else {
            return None;
        };

        let mat = completions_menu
            .matches
            .get(action.item_ix.unwrap_or(completions_menu.selected_item))?;
        let buffer_handle = completions_menu.buffer;
        let completion = completions_menu.completions.get(mat.candidate_id)?;

        let snippet;
        let text;
        if completion.is_snippet() {
            snippet = Some(Snippet::parse(&completion.new_text).log_err()?);
            text = snippet.as_ref().unwrap().text.clone();
        } else {
            snippet = None;
            text = completion.new_text.clone();
        };
        let selections = self.selections.all::<usize>(cx);
        let buffer = buffer_handle.read(cx);
        let old_range = completion.old_range.to_offset(buffer);
        let old_text = buffer.text_for_range(old_range.clone()).collect::<String>();

        let newest_selection = self.selections.newest_anchor();
        if newest_selection.start.buffer_id != Some(buffer_handle.read(cx).remote_id()) {
            return None;
        }

        let lookbehind = newest_selection
            .start
            .text_anchor
            .to_offset(buffer)
            .saturating_sub(old_range.start);
        let lookahead = old_range
            .end
            .saturating_sub(newest_selection.end.text_anchor.to_offset(buffer));
        let mut common_prefix_len = old_text
            .bytes()
            .zip(text.bytes())
            .take_while(|(a, b)| a == b)
            .count();

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let mut range_to_replace: Option<Range<isize>> = None;
        let mut ranges = Vec::new();
        for selection in &selections {
            if snapshot.contains_str_at(selection.start.saturating_sub(lookbehind), &old_text) {
                let start = selection.start.saturating_sub(lookbehind);
                let end = selection.end + lookahead;
                if selection.id == newest_selection.id {
                    range_to_replace = Some(
                        ((start + common_prefix_len) as isize - selection.start as isize)
                            ..(end as isize - selection.start as isize),
                    );
                }
                ranges.push(start + common_prefix_len..end);
            } else {
                common_prefix_len = 0;
                ranges.clear();
                ranges.extend(selections.iter().map(|s| {
                    if s.id == newest_selection.id {
                        range_to_replace = Some(
                            old_range.start.to_offset_utf16(&snapshot).0 as isize
                                - selection.start as isize
                                ..old_range.end.to_offset_utf16(&snapshot).0 as isize
                                    - selection.start as isize,
                        );
                        old_range.clone()
                    } else {
                        s.start..s.end
                    }
                }));
                break;
            }
        }
        let text = &text[common_prefix_len..];

        cx.emit(Event::InputHandled {
            utf16_range_to_replace: range_to_replace,
            text: text.into(),
        });

        self.transact(cx, |this, cx| {
            if let Some(mut snippet) = snippet {
                snippet.text = text.to_string();
                for tabstop in snippet.tabstops.iter_mut().flatten() {
                    tabstop.start -= common_prefix_len as isize;
                    tabstop.end -= common_prefix_len as isize;
                }

                this.insert_snippet(&ranges, snippet, cx).log_err();
            } else {
                this.buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        ranges.iter().map(|range| (range.clone(), text)),
                        this.autoindent_mode.clone(),
                        cx,
                    );
                });
            }

            this.refresh_copilot_suggestions(true, cx);
        });

        let project = self.project.clone()?;
        let apply_edits = project.update(cx, |project, cx| {
            project.apply_additional_edits_for_completion(
                buffer_handle,
                completion.clone(),
                true,
                cx,
            )
        });
        Some(cx.foreground().spawn(async move {
            apply_edits.await?;
            Ok(())
        }))
    }

    pub fn toggle_code_actions(&mut self, action: &ToggleCodeActions, cx: &mut ViewContext<Self>) {
        if matches!(
            self.context_menu.as_ref(),
            Some(ContextMenu::CodeActions(_))
        ) {
            self.context_menu.take();
            cx.notify();
            return;
        }

        let deployed_from_indicator = action.deployed_from_indicator;
        let mut task = self.code_actions_task.take();
        cx.spawn(|this, mut cx| async move {
            while let Some(prev_task) = task {
                prev_task.await;
                task = this.update(&mut cx, |this, _| this.code_actions_task.take())?;
            }

            this.update(&mut cx, |this, cx| {
                if this.focused {
                    if let Some((buffer, actions)) = this.available_code_actions.clone() {
                        this.show_context_menu(
                            ContextMenu::CodeActions(CodeActionsMenu {
                                buffer,
                                actions,
                                selected_item: Default::default(),
                                list: Default::default(),
                                deployed_from_indicator,
                            }),
                            cx,
                        );
                    }
                }
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }

    pub fn confirm_code_action(
        workspace: &mut Workspace,
        action: &ConfirmCodeAction,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        let editor = workspace.active_item(cx)?.act_as::<Editor>(cx)?;
        let actions_menu = if let ContextMenu::CodeActions(menu) =
            editor.update(cx, |editor, cx| editor.hide_context_menu(cx))?
        {
            menu
        } else {
            return None;
        };
        let action_ix = action.item_ix.unwrap_or(actions_menu.selected_item);
        let action = actions_menu.actions.get(action_ix)?.clone();
        let title = action.lsp_action.title.clone();
        let buffer = actions_menu.buffer;

        let apply_code_actions = workspace.project().clone().update(cx, |project, cx| {
            project.apply_code_action(buffer, action, true, cx)
        });
        let editor = editor.downgrade();
        Some(cx.spawn(|workspace, cx| async move {
            let project_transaction = apply_code_actions.await?;
            Self::open_project_transaction(&editor, workspace, project_transaction, title, cx).await
        }))
    }

    async fn open_project_transaction(
        this: &WeakViewHandle<Editor>,
        workspace: WeakViewHandle<Workspace>,
        transaction: ProjectTransaction,
        title: String,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let replica_id = this.read_with(&cx, |this, cx| this.replica_id(cx))?;

        let mut entries = transaction.0.into_iter().collect::<Vec<_>>();
        entries.sort_unstable_by_key(|(buffer, _)| {
            buffer.read_with(&cx, |buffer, _| buffer.file().map(|f| f.path().clone()))
        });

        // If the project transaction's edits are all contained within this editor, then
        // avoid opening a new editor to display them.

        if let Some((buffer, transaction)) = entries.first() {
            if entries.len() == 1 {
                let excerpt = this.read_with(&cx, |editor, cx| {
                    editor
                        .buffer()
                        .read(cx)
                        .excerpt_containing(editor.selections.newest_anchor().head(), cx)
                })?;
                if let Some((_, excerpted_buffer, excerpt_range)) = excerpt {
                    if excerpted_buffer == *buffer {
                        let all_edits_within_excerpt = buffer.read_with(&cx, |buffer, _| {
                            let excerpt_range = excerpt_range.to_offset(buffer);
                            buffer
                                .edited_ranges_for_transaction::<usize>(transaction)
                                .all(|range| {
                                    excerpt_range.start <= range.start
                                        && excerpt_range.end >= range.end
                                })
                        });

                        if all_edits_within_excerpt {
                            return Ok(());
                        }
                    }
                }
            }
        } else {
            return Ok(());
        }

        let mut ranges_to_highlight = Vec::new();
        let excerpt_buffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(replica_id).with_title(title);
            for (buffer_handle, transaction) in &entries {
                let buffer = buffer_handle.read(cx);
                ranges_to_highlight.extend(
                    multibuffer.push_excerpts_with_context_lines(
                        buffer_handle.clone(),
                        buffer
                            .edited_ranges_for_transaction::<usize>(transaction)
                            .collect(),
                        1,
                        cx,
                    ),
                );
            }
            multibuffer.push_transaction(entries.iter().map(|(b, t)| (b, t)), cx);
            multibuffer
        });

        workspace.update(&mut cx, |workspace, cx| {
            let project = workspace.project().clone();
            let editor =
                cx.add_view(|cx| Editor::for_multibuffer(excerpt_buffer, Some(project), cx));
            workspace.add_item(Box::new(editor.clone()), cx);
            editor.update(cx, |editor, cx| {
                editor.highlight_background::<Self>(
                    ranges_to_highlight,
                    |theme| theme.editor.highlighted_line_background,
                    cx,
                );
            });
        })?;

        Ok(())
    }

    fn refresh_code_actions(&mut self, cx: &mut ViewContext<Self>) -> Option<()> {
        let project = self.project.clone()?;
        let buffer = self.buffer.read(cx);
        let newest_selection = self.selections.newest_anchor().clone();
        let (start_buffer, start) = buffer.text_anchor_for_position(newest_selection.start, cx)?;
        let (end_buffer, end) = buffer.text_anchor_for_position(newest_selection.end, cx)?;
        if start_buffer != end_buffer {
            return None;
        }

        self.code_actions_task = Some(cx.spawn(|this, mut cx| async move {
            cx.background().timer(CODE_ACTIONS_DEBOUNCE_TIMEOUT).await;

            let actions = project
                .update(&mut cx, |project, cx| {
                    project.code_actions(&start_buffer, start..end, cx)
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.available_code_actions = actions.log_err().and_then(|actions| {
                    if actions.is_empty() {
                        None
                    } else {
                        Some((start_buffer, actions.into()))
                    }
                });
                cx.notify();
            })
            .log_err();
        }));
        None
    }

    fn refresh_document_highlights(&mut self, cx: &mut ViewContext<Self>) -> Option<()> {
        if self.pending_rename.is_some() {
            return None;
        }

        let project = self.project.clone()?;
        let buffer = self.buffer.read(cx);
        let newest_selection = self.selections.newest_anchor().clone();
        let cursor_position = newest_selection.head();
        let (cursor_buffer, cursor_buffer_position) =
            buffer.text_anchor_for_position(cursor_position.clone(), cx)?;
        let (tail_buffer, _) = buffer.text_anchor_for_position(newest_selection.tail(), cx)?;
        if cursor_buffer != tail_buffer {
            return None;
        }

        self.document_highlights_task = Some(cx.spawn(|this, mut cx| async move {
            cx.background()
                .timer(DOCUMENT_HIGHLIGHTS_DEBOUNCE_TIMEOUT)
                .await;

            let highlights = project
                .update(&mut cx, |project, cx| {
                    project.document_highlights(&cursor_buffer, cursor_buffer_position, cx)
                })
                .await
                .log_err();

            if let Some(highlights) = highlights {
                this.update(&mut cx, |this, cx| {
                    if this.pending_rename.is_some() {
                        return;
                    }

                    let buffer_id = cursor_position.buffer_id;
                    let buffer = this.buffer.read(cx);
                    if !buffer
                        .text_anchor_for_position(cursor_position, cx)
                        .map_or(false, |(buffer, _)| buffer == cursor_buffer)
                    {
                        return;
                    }

                    let cursor_buffer_snapshot = cursor_buffer.read(cx);
                    let mut write_ranges = Vec::new();
                    let mut read_ranges = Vec::new();
                    for highlight in highlights {
                        for (excerpt_id, excerpt_range) in
                            buffer.excerpts_for_buffer(&cursor_buffer, cx)
                        {
                            let start = highlight
                                .range
                                .start
                                .max(&excerpt_range.context.start, cursor_buffer_snapshot);
                            let end = highlight
                                .range
                                .end
                                .min(&excerpt_range.context.end, cursor_buffer_snapshot);
                            if start.cmp(&end, cursor_buffer_snapshot).is_ge() {
                                continue;
                            }

                            let range = Anchor {
                                buffer_id,
                                excerpt_id: excerpt_id.clone(),
                                text_anchor: start,
                            }..Anchor {
                                buffer_id,
                                excerpt_id,
                                text_anchor: end,
                            };
                            if highlight.kind == lsp::DocumentHighlightKind::WRITE {
                                write_ranges.push(range);
                            } else {
                                read_ranges.push(range);
                            }
                        }
                    }

                    this.highlight_background::<DocumentHighlightRead>(
                        read_ranges,
                        |theme| theme.editor.document_highlight_read_background,
                        cx,
                    );
                    this.highlight_background::<DocumentHighlightWrite>(
                        write_ranges,
                        |theme| theme.editor.document_highlight_write_background,
                        cx,
                    );
                    cx.notify();
                })
                .log_err();
            }
        }));
        None
    }

    fn refresh_copilot_suggestions(
        &mut self,
        debounce: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<()> {
        let copilot = Copilot::global(cx)?;
        if self.mode != EditorMode::Full || !copilot.read(cx).status().is_authorized() {
            self.clear_copilot_suggestions(cx);
            return None;
        }
        self.update_visible_copilot_suggestion(cx);

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let cursor = self.selections.newest_anchor().head();
        if !self.is_copilot_enabled_at(cursor, &snapshot, cx) {
            self.clear_copilot_suggestions(cx);
            return None;
        }

        let (buffer, buffer_position) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)?;
        self.copilot_state.pending_refresh = cx.spawn(|this, mut cx| async move {
            if debounce {
                cx.background().timer(COPILOT_DEBOUNCE_TIMEOUT).await;
            }

            let completions = copilot
                .update(&mut cx, |copilot, cx| {
                    copilot.completions(&buffer, buffer_position, cx)
                })
                .await
                .log_err()
                .into_iter()
                .flatten()
                .collect_vec();

            this.update(&mut cx, |this, cx| {
                if !completions.is_empty() {
                    this.copilot_state.cycled = false;
                    this.copilot_state.pending_cycling_refresh = Task::ready(None);
                    this.copilot_state.completions.clear();
                    this.copilot_state.active_completion_index = 0;
                    this.copilot_state.excerpt_id = Some(cursor.excerpt_id);
                    for completion in completions {
                        this.copilot_state.push_completion(completion);
                    }
                    this.update_visible_copilot_suggestion(cx);
                }
            })
            .log_err()?;
            Some(())
        });

        Some(())
    }

    fn cycle_copilot_suggestions(
        &mut self,
        direction: Direction,
        cx: &mut ViewContext<Self>,
    ) -> Option<()> {
        let copilot = Copilot::global(cx)?;
        if self.mode != EditorMode::Full || !copilot.read(cx).status().is_authorized() {
            return None;
        }

        if self.copilot_state.cycled {
            self.copilot_state.cycle_completions(direction);
            self.update_visible_copilot_suggestion(cx);
        } else {
            let cursor = self.selections.newest_anchor().head();
            let (buffer, buffer_position) =
                self.buffer.read(cx).text_anchor_for_position(cursor, cx)?;
            self.copilot_state.pending_cycling_refresh = cx.spawn(|this, mut cx| async move {
                let completions = copilot
                    .update(&mut cx, |copilot, cx| {
                        copilot.completions_cycling(&buffer, buffer_position, cx)
                    })
                    .await;

                this.update(&mut cx, |this, cx| {
                    this.copilot_state.cycled = true;
                    for completion in completions.log_err().into_iter().flatten() {
                        this.copilot_state.push_completion(completion);
                    }
                    this.copilot_state.cycle_completions(direction);
                    this.update_visible_copilot_suggestion(cx);
                })
                .log_err()?;

                Some(())
            });
        }

        Some(())
    }

    fn copilot_suggest(&mut self, _: &copilot::Suggest, cx: &mut ViewContext<Self>) {
        if !self.has_active_copilot_suggestion(cx) {
            self.refresh_copilot_suggestions(false, cx);
            return;
        }

        self.update_visible_copilot_suggestion(cx);
    }

    fn next_copilot_suggestion(&mut self, _: &copilot::NextSuggestion, cx: &mut ViewContext<Self>) {
        if self.has_active_copilot_suggestion(cx) {
            self.cycle_copilot_suggestions(Direction::Next, cx);
        } else {
            self.refresh_copilot_suggestions(false, cx);
        }
    }

    fn previous_copilot_suggestion(
        &mut self,
        _: &copilot::PreviousSuggestion,
        cx: &mut ViewContext<Self>,
    ) {
        if self.has_active_copilot_suggestion(cx) {
            self.cycle_copilot_suggestions(Direction::Prev, cx);
        } else {
            self.refresh_copilot_suggestions(false, cx);
        }
    }

    fn accept_copilot_suggestion(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if let Some(suggestion) = self.take_active_copilot_suggestion(cx) {
            if let Some((copilot, completion)) =
                Copilot::global(cx).zip(self.copilot_state.active_completion())
            {
                copilot
                    .update(cx, |copilot, cx| copilot.accept_completion(completion, cx))
                    .detach_and_log_err(cx);

                self.report_copilot_event(Some(completion.uuid.clone()), true, cx)
            }
            cx.emit(Event::InputHandled {
                utf16_range_to_replace: None,
                text: suggestion.text.to_string().into(),
            });
            self.insert_with_autoindent_mode(&suggestion.text.to_string(), None, cx);
            cx.notify();
            true
        } else {
            false
        }
    }

    fn discard_copilot_suggestion(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if let Some(suggestion) = self.take_active_copilot_suggestion(cx) {
            if let Some(copilot) = Copilot::global(cx) {
                copilot
                    .update(cx, |copilot, cx| {
                        copilot.discard_completions(&self.copilot_state.completions, cx)
                    })
                    .detach_and_log_err(cx);

                self.report_copilot_event(None, false, cx)
            }

            self.display_map.update(cx, |map, cx| {
                map.splice_inlays(vec![suggestion.id], Vec::new(), cx)
            });
            cx.notify();
            true
        } else {
            false
        }
    }

    fn is_copilot_enabled_at(
        &self,
        location: Anchor,
        snapshot: &MultiBufferSnapshot,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        let file = snapshot.file_at(location);
        let language = snapshot.language_at(location);
        let settings = all_language_settings(file, cx);
        settings.copilot_enabled(language, file.map(|f| f.path().as_ref()))
    }

    fn has_active_copilot_suggestion(&self, cx: &AppContext) -> bool {
        if let Some(suggestion) = self.copilot_state.suggestion.as_ref() {
            let buffer = self.buffer.read(cx).read(cx);
            suggestion.position.is_valid(&buffer)
        } else {
            false
        }
    }

    fn take_active_copilot_suggestion(&mut self, cx: &mut ViewContext<Self>) -> Option<Inlay> {
        let suggestion = self.copilot_state.suggestion.take()?;
        self.display_map.update(cx, |map, cx| {
            map.splice_inlays(vec![suggestion.id], Default::default(), cx);
        });
        let buffer = self.buffer.read(cx).read(cx);

        if suggestion.position.is_valid(&buffer) {
            Some(suggestion)
        } else {
            None
        }
    }

    fn update_visible_copilot_suggestion(&mut self, cx: &mut ViewContext<Self>) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let selection = self.selections.newest_anchor();
        let cursor = selection.head();

        if self.context_menu.is_some()
            || !self.completion_tasks.is_empty()
            || selection.start != selection.end
        {
            self.discard_copilot_suggestion(cx);
        } else if let Some(text) = self
            .copilot_state
            .text_for_active_completion(cursor, &snapshot)
        {
            let text = Rope::from(text);
            let mut to_remove = Vec::new();
            if let Some(suggestion) = self.copilot_state.suggestion.take() {
                to_remove.push(suggestion.id);
            }

            let suggestion_inlay =
                Inlay::suggestion(post_inc(&mut self.next_inlay_id), cursor, text);
            self.copilot_state.suggestion = Some(suggestion_inlay.clone());
            self.display_map.update(cx, move |map, cx| {
                map.splice_inlays(to_remove, vec![suggestion_inlay], cx)
            });
            cx.notify();
        } else {
            self.discard_copilot_suggestion(cx);
        }
    }

    fn clear_copilot_suggestions(&mut self, cx: &mut ViewContext<Self>) {
        self.copilot_state = Default::default();
        self.discard_copilot_suggestion(cx);
    }

    pub fn render_code_actions_indicator(
        &self,
        style: &EditorStyle,
        is_active: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement<Self>> {
        if self.available_code_actions.is_some() {
            enum CodeActions {}
            Some(
                MouseEventHandler::new::<CodeActions, _>(0, cx, |state, _| {
                    Svg::new("icons/bolt_8.svg").with_color(
                        style
                            .code_actions
                            .indicator
                            .in_state(is_active)
                            .style_for(state)
                            .color,
                    )
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .with_padding(Padding::uniform(3.))
                .on_down(MouseButton::Left, |_, this, cx| {
                    this.toggle_code_actions(
                        &ToggleCodeActions {
                            deployed_from_indicator: true,
                        },
                        cx,
                    );
                })
                .into_any(),
            )
        } else {
            None
        }
    }

    pub fn render_fold_indicators(
        &self,
        fold_data: Vec<Option<(FoldStatus, u32, bool)>>,
        style: &EditorStyle,
        gutter_hovered: bool,
        line_height: f32,
        gutter_margin: f32,
        cx: &mut ViewContext<Self>,
    ) -> Vec<Option<AnyElement<Self>>> {
        enum FoldIndicators {}

        let style = style.folds.clone();

        fold_data
            .iter()
            .enumerate()
            .map(|(ix, fold_data)| {
                fold_data
                    .map(|(fold_status, buffer_row, active)| {
                        (active || gutter_hovered || fold_status == FoldStatus::Folded).then(|| {
                            MouseEventHandler::new::<FoldIndicators, _>(
                                ix as usize,
                                cx,
                                |mouse_state, _| {
                                    Svg::new(match fold_status {
                                        FoldStatus::Folded => style.folded_icon.clone(),
                                        FoldStatus::Foldable => style.foldable_icon.clone(),
                                    })
                                    .with_color(
                                        style
                                            .indicator
                                            .in_state(fold_status == FoldStatus::Folded)
                                            .style_for(mouse_state)
                                            .color,
                                    )
                                    .constrained()
                                    .with_width(gutter_margin * style.icon_margin_scale)
                                    .aligned()
                                    .constrained()
                                    .with_height(line_height)
                                    .with_width(gutter_margin)
                                    .aligned()
                                },
                            )
                            .with_cursor_style(CursorStyle::PointingHand)
                            .with_padding(Padding::uniform(3.))
                            .on_click(MouseButton::Left, {
                                move |_, editor, cx| match fold_status {
                                    FoldStatus::Folded => {
                                        editor.unfold_at(&UnfoldAt { buffer_row }, cx);
                                    }
                                    FoldStatus::Foldable => {
                                        editor.fold_at(&FoldAt { buffer_row }, cx);
                                    }
                                }
                            })
                            .into_any()
                        })
                    })
                    .flatten()
            })
            .collect()
    }

    pub fn context_menu_visible(&self) -> bool {
        self.context_menu
            .as_ref()
            .map_or(false, |menu| menu.visible())
    }

    pub fn render_context_menu(
        &self,
        cursor_position: DisplayPoint,
        style: EditorStyle,
        cx: &mut ViewContext<Editor>,
    ) -> Option<(DisplayPoint, AnyElement<Editor>)> {
        self.context_menu
            .as_ref()
            .map(|menu| menu.render(cursor_position, style, cx))
    }

    fn show_context_menu(&mut self, menu: ContextMenu, cx: &mut ViewContext<Self>) {
        if !matches!(menu, ContextMenu::Completions(_)) {
            self.completion_tasks.clear();
        }
        self.context_menu = Some(menu);
        self.discard_copilot_suggestion(cx);
        cx.notify();
    }

    fn hide_context_menu(&mut self, cx: &mut ViewContext<Self>) -> Option<ContextMenu> {
        cx.notify();
        self.completion_tasks.clear();
        let context_menu = self.context_menu.take();
        if context_menu.is_some() {
            self.update_visible_copilot_suggestion(cx);
        }
        context_menu
    }

    pub fn insert_snippet(
        &mut self,
        insertion_ranges: &[Range<usize>],
        snippet: Snippet,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        let tabstops = self.buffer.update(cx, |buffer, cx| {
            let snippet_text: Arc<str> = snippet.text.clone().into();
            buffer.edit(
                insertion_ranges
                    .iter()
                    .cloned()
                    .map(|range| (range, snippet_text.clone())),
                Some(AutoindentMode::EachLine),
                cx,
            );

            let snapshot = &*buffer.read(cx);
            let snippet = &snippet;
            snippet
                .tabstops
                .iter()
                .map(|tabstop| {
                    let mut tabstop_ranges = tabstop
                        .iter()
                        .flat_map(|tabstop_range| {
                            let mut delta = 0_isize;
                            insertion_ranges.iter().map(move |insertion_range| {
                                let insertion_start = insertion_range.start as isize + delta;
                                delta +=
                                    snippet.text.len() as isize - insertion_range.len() as isize;

                                let start = snapshot.anchor_before(
                                    (insertion_start + tabstop_range.start) as usize,
                                );
                                let end = snapshot
                                    .anchor_after((insertion_start + tabstop_range.end) as usize);
                                start..end
                            })
                        })
                        .collect::<Vec<_>>();
                    tabstop_ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start, snapshot));
                    tabstop_ranges
                })
                .collect::<Vec<_>>()
        });

        if let Some(tabstop) = tabstops.first() {
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_ranges(tabstop.iter().cloned());
            });
            self.snippet_stack.push(SnippetState {
                active_index: 0,
                ranges: tabstops,
            });
        }

        Ok(())
    }

    pub fn move_to_next_snippet_tabstop(&mut self, cx: &mut ViewContext<Self>) -> bool {
        self.move_to_snippet_tabstop(Bias::Right, cx)
    }

    pub fn move_to_prev_snippet_tabstop(&mut self, cx: &mut ViewContext<Self>) -> bool {
        self.move_to_snippet_tabstop(Bias::Left, cx)
    }

    pub fn move_to_snippet_tabstop(&mut self, bias: Bias, cx: &mut ViewContext<Self>) -> bool {
        if let Some(mut snippet) = self.snippet_stack.pop() {
            match bias {
                Bias::Left => {
                    if snippet.active_index > 0 {
                        snippet.active_index -= 1;
                    } else {
                        self.snippet_stack.push(snippet);
                        return false;
                    }
                }
                Bias::Right => {
                    if snippet.active_index + 1 < snippet.ranges.len() {
                        snippet.active_index += 1;
                    } else {
                        self.snippet_stack.push(snippet);
                        return false;
                    }
                }
            }
            if let Some(current_ranges) = snippet.ranges.get(snippet.active_index) {
                self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_anchor_ranges(current_ranges.iter().cloned())
                });
                // If snippet state is not at the last tabstop, push it back on the stack
                if snippet.active_index + 1 < snippet.ranges.len() {
                    self.snippet_stack.push(snippet);
                }
                return true;
            }
        }

        false
    }

    pub fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.select_all(&SelectAll, cx);
            this.insert("", cx);
        });
    }

    pub fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.select_autoclose_pair(cx);
            let mut selections = this.selections.all::<Point>(cx);
            if !this.selections.line_mode {
                let display_map = this.display_map.update(cx, |map, cx| map.snapshot(cx));
                for selection in &mut selections {
                    if selection.is_empty() {
                        let old_head = selection.head();
                        let mut new_head =
                            movement::left(&display_map, old_head.to_display_point(&display_map))
                                .to_point(&display_map);
                        if let Some((buffer, line_buffer_range)) = display_map
                            .buffer_snapshot
                            .buffer_line_for_row(old_head.row)
                        {
                            let indent_size =
                                buffer.indent_size_for_line(line_buffer_range.start.row);
                            let indent_len = match indent_size.kind {
                                IndentKind::Space => {
                                    buffer.settings_at(line_buffer_range.start, cx).tab_size
                                }
                                IndentKind::Tab => NonZeroU32::new(1).unwrap(),
                            };
                            if old_head.column <= indent_size.len && old_head.column > 0 {
                                let indent_len = indent_len.get();
                                new_head = cmp::min(
                                    new_head,
                                    Point::new(
                                        old_head.row,
                                        ((old_head.column - 1) / indent_len) * indent_len,
                                    ),
                                );
                            }
                        }

                        selection.set_head(new_head, SelectionGoal::None);
                    }
                }
            }

            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
            this.insert("", cx);
            this.refresh_copilot_suggestions(true, cx);
        });
    }

    pub fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::right(map, selection.head());
                        selection.end = cursor;
                        selection.reversed = true;
                        selection.goal = SelectionGoal::None;
                    }
                })
            });
            this.insert("", cx);
            this.refresh_copilot_suggestions(true, cx);
        });
    }

    pub fn tab_prev(&mut self, _: &TabPrev, cx: &mut ViewContext<Self>) {
        if self.move_to_prev_snippet_tabstop(cx) {
            return;
        }

        self.outdent(&Outdent, cx);
    }

    pub fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        if self.move_to_next_snippet_tabstop(cx) {
            return;
        }

        let mut selections = self.selections.all_adjusted(cx);
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);
        let rows_iter = selections.iter().map(|s| s.head().row);
        let suggested_indents = snapshot.suggested_indents(rows_iter, cx);

        let mut edits = Vec::new();
        let mut prev_edited_row = 0;
        let mut row_delta = 0;
        for selection in &mut selections {
            if selection.start.row != prev_edited_row {
                row_delta = 0;
            }
            prev_edited_row = selection.end.row;

            // If the selection is non-empty, then increase the indentation of the selected lines.
            if !selection.is_empty() {
                row_delta =
                    Self::indent_selection(buffer, &snapshot, selection, &mut edits, row_delta, cx);
                continue;
            }

            // If the selection is empty and the cursor is in the leading whitespace before the
            // suggested indentation, then auto-indent the line.
            let cursor = selection.head();
            let current_indent = snapshot.indent_size_for_line(cursor.row);
            if let Some(suggested_indent) = suggested_indents.get(&cursor.row).copied() {
                if cursor.column < suggested_indent.len
                    && cursor.column <= current_indent.len
                    && current_indent.len <= suggested_indent.len
                {
                    selection.start = Point::new(cursor.row, suggested_indent.len);
                    selection.end = selection.start;
                    if row_delta == 0 {
                        edits.extend(Buffer::edit_for_indent_size_adjustment(
                            cursor.row,
                            current_indent,
                            suggested_indent,
                        ));
                        row_delta = suggested_indent.len - current_indent.len;
                    }
                    continue;
                }
            }

            // Accept copilot suggestion if there is only one selection and the cursor is not
            // in the leading whitespace.
            if self.selections.count() == 1
                && cursor.column >= current_indent.len
                && self.has_active_copilot_suggestion(cx)
            {
                self.accept_copilot_suggestion(cx);
                return;
            }

            // Otherwise, insert a hard or soft tab.
            let settings = buffer.settings_at(cursor, cx);
            let tab_size = if settings.hard_tabs {
                IndentSize::tab()
            } else {
                let tab_size = settings.tab_size.get();
                let char_column = snapshot
                    .text_for_range(Point::new(cursor.row, 0)..cursor)
                    .flat_map(str::chars)
                    .count()
                    + row_delta as usize;
                let chars_to_next_tab_stop = tab_size - (char_column as u32 % tab_size);
                IndentSize::spaces(chars_to_next_tab_stop)
            };
            selection.start = Point::new(cursor.row, cursor.column + row_delta + tab_size.len);
            selection.end = selection.start;
            edits.push((cursor..cursor, tab_size.chars().collect::<String>()));
            row_delta += tab_size.len;
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |b, cx| b.edit(edits, None, cx));
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
            this.refresh_copilot_suggestions(true, cx);
        });
    }

    pub fn indent(&mut self, _: &Indent, cx: &mut ViewContext<Self>) {
        let mut selections = self.selections.all::<Point>(cx);
        let mut prev_edited_row = 0;
        let mut row_delta = 0;
        let mut edits = Vec::new();
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);
        for selection in &mut selections {
            if selection.start.row != prev_edited_row {
                row_delta = 0;
            }
            prev_edited_row = selection.end.row;

            row_delta =
                Self::indent_selection(buffer, &snapshot, selection, &mut edits, row_delta, cx);
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |b, cx| b.edit(edits, None, cx));
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
        });
    }

    fn indent_selection(
        buffer: &MultiBuffer,
        snapshot: &MultiBufferSnapshot,
        selection: &mut Selection<Point>,
        edits: &mut Vec<(Range<Point>, String)>,
        delta_for_start_row: u32,
        cx: &AppContext,
    ) -> u32 {
        let settings = buffer.settings_at(selection.start, cx);
        let tab_size = settings.tab_size.get();
        let indent_kind = if settings.hard_tabs {
            IndentKind::Tab
        } else {
            IndentKind::Space
        };
        let mut start_row = selection.start.row;
        let mut end_row = selection.end.row + 1;

        // If a selection ends at the beginning of a line, don't indent
        // that last line.
        if selection.end.column == 0 {
            end_row -= 1;
        }

        // Avoid re-indenting a row that has already been indented by a
        // previous selection, but still update this selection's column
        // to reflect that indentation.
        if delta_for_start_row > 0 {
            start_row += 1;
            selection.start.column += delta_for_start_row;
            if selection.end.row == selection.start.row {
                selection.end.column += delta_for_start_row;
            }
        }

        let mut delta_for_end_row = 0;
        for row in start_row..end_row {
            let current_indent = snapshot.indent_size_for_line(row);
            let indent_delta = match (current_indent.kind, indent_kind) {
                (IndentKind::Space, IndentKind::Space) => {
                    let columns_to_next_tab_stop = tab_size - (current_indent.len % tab_size);
                    IndentSize::spaces(columns_to_next_tab_stop)
                }
                (IndentKind::Tab, IndentKind::Space) => IndentSize::spaces(tab_size),
                (_, IndentKind::Tab) => IndentSize::tab(),
            };

            let row_start = Point::new(row, 0);
            edits.push((
                row_start..row_start,
                indent_delta.chars().collect::<String>(),
            ));

            // Update this selection's endpoints to reflect the indentation.
            if row == selection.start.row {
                selection.start.column += indent_delta.len;
            }
            if row == selection.end.row {
                selection.end.column += indent_delta.len;
                delta_for_end_row = indent_delta.len;
            }
        }

        if selection.start.row == selection.end.row {
            delta_for_start_row + delta_for_end_row
        } else {
            delta_for_end_row
        }
    }

    pub fn outdent(&mut self, _: &Outdent, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);
        let mut deletion_ranges = Vec::new();
        let mut last_outdent = None;
        {
            let buffer = self.buffer.read(cx);
            let snapshot = buffer.snapshot(cx);
            for selection in &selections {
                let settings = buffer.settings_at(selection.start, cx);
                let tab_size = settings.tab_size.get();
                let mut rows = selection.spanned_rows(false, &display_map);

                // Avoid re-outdenting a row that has already been outdented by a
                // previous selection.
                if let Some(last_row) = last_outdent {
                    if last_row == rows.start {
                        rows.start += 1;
                    }
                }

                for row in rows {
                    let indent_size = snapshot.indent_size_for_line(row);
                    if indent_size.len > 0 {
                        let deletion_len = match indent_size.kind {
                            IndentKind::Space => {
                                let columns_to_prev_tab_stop = indent_size.len % tab_size;
                                if columns_to_prev_tab_stop == 0 {
                                    tab_size
                                } else {
                                    columns_to_prev_tab_stop
                                }
                            }
                            IndentKind::Tab => 1,
                        };
                        deletion_ranges.push(Point::new(row, 0)..Point::new(row, deletion_len));
                        last_outdent = Some(row);
                    }
                }
            }
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                let empty_str: Arc<str> = "".into();
                buffer.edit(
                    deletion_ranges
                        .into_iter()
                        .map(|range| (range, empty_str.clone())),
                    None,
                    cx,
                );
            });
            let selections = this.selections.all::<usize>(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
        });
    }

    pub fn delete_line(&mut self, _: &DeleteLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);

        let mut new_cursors = Vec::new();
        let mut edit_ranges = Vec::new();
        let mut selections = selections.iter().peekable();
        while let Some(selection) = selections.next() {
            let mut rows = selection.spanned_rows(false, &display_map);
            let goal_display_column = selection.head().to_display_point(&display_map).column();

            // Accumulate contiguous regions of rows that we want to delete.
            while let Some(next_selection) = selections.peek() {
                let next_rows = next_selection.spanned_rows(false, &display_map);
                if next_rows.start <= rows.end {
                    rows.end = next_rows.end;
                    selections.next().unwrap();
                } else {
                    break;
                }
            }

            let buffer = &display_map.buffer_snapshot;
            let mut edit_start = Point::new(rows.start, 0).to_offset(buffer);
            let edit_end;
            let cursor_buffer_row;
            if buffer.max_point().row >= rows.end {
                // If there's a line after the range, delete the \n from the end of the row range
                // and position the cursor on the next line.
                edit_end = Point::new(rows.end, 0).to_offset(buffer);
                cursor_buffer_row = rows.end;
            } else {
                // If there isn't a line after the range, delete the \n from the line before the
                // start of the row range and position the cursor there.
                edit_start = edit_start.saturating_sub(1);
                edit_end = buffer.len();
                cursor_buffer_row = rows.start.saturating_sub(1);
            }

            let mut cursor = Point::new(cursor_buffer_row, 0).to_display_point(&display_map);
            *cursor.column_mut() =
                cmp::min(goal_display_column, display_map.line_len(cursor.row()));

            new_cursors.push((
                selection.id,
                buffer.anchor_after(cursor.to_point(&display_map)),
            ));
            edit_ranges.push(edit_start..edit_end);
        }

        self.transact(cx, |this, cx| {
            let buffer = this.buffer.update(cx, |buffer, cx| {
                let empty_str: Arc<str> = "".into();
                buffer.edit(
                    edit_ranges
                        .into_iter()
                        .map(|range| (range, empty_str.clone())),
                    None,
                    cx,
                );
                buffer.snapshot(cx)
            });
            let new_selections = new_cursors
                .into_iter()
                .map(|(id, cursor)| {
                    let cursor = cursor.to_point(&buffer);
                    Selection {
                        id,
                        start: cursor,
                        end: cursor,
                        reversed: false,
                        goal: SelectionGoal::None,
                    }
                })
                .collect();

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });
        });
    }

    pub fn join_lines(&mut self, _: &JoinLines, cx: &mut ViewContext<Self>) {
        let mut row_ranges = Vec::<Range<u32>>::new();
        for selection in self.selections.all::<Point>(cx) {
            let start = selection.start.row;
            let end = if selection.start.row == selection.end.row {
                selection.start.row + 1
            } else {
                selection.end.row
            };

            if let Some(last_row_range) = row_ranges.last_mut() {
                if start <= last_row_range.end {
                    last_row_range.end = end;
                    continue;
                }
            }
            row_ranges.push(start..end);
        }

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let mut cursor_positions = Vec::new();
        for row_range in &row_ranges {
            let anchor = snapshot.anchor_before(Point::new(
                row_range.end - 1,
                snapshot.line_len(row_range.end - 1),
            ));
            cursor_positions.push(anchor.clone()..anchor);
        }

        self.transact(cx, |this, cx| {
            for row_range in row_ranges.into_iter().rev() {
                for row in row_range.rev() {
                    let end_of_line = Point::new(row, snapshot.line_len(row));
                    let indent = snapshot.indent_size_for_line(row + 1);
                    let start_of_next_line = Point::new(row + 1, indent.len);

                    let replace = if snapshot.line_len(row + 1) > indent.len {
                        " "
                    } else {
                        ""
                    };

                    this.buffer.update(cx, |buffer, cx| {
                        buffer.edit([(end_of_line..start_of_next_line, replace)], None, cx)
                    });
                }
            }

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_anchor_ranges(cursor_positions)
            });
        });
    }

    pub fn sort_lines_case_sensitive(
        &mut self,
        _: &SortLinesCaseSensitive,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_lines(cx, |lines| lines.sort())
    }

    pub fn sort_lines_case_insensitive(
        &mut self,
        _: &SortLinesCaseInsensitive,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_lines(cx, |lines| lines.sort_by_key(|line| line.to_lowercase()))
    }

    pub fn reverse_lines(&mut self, _: &ReverseLines, cx: &mut ViewContext<Self>) {
        self.manipulate_lines(cx, |lines| lines.reverse())
    }

    pub fn shuffle_lines(&mut self, _: &ShuffleLines, cx: &mut ViewContext<Self>) {
        self.manipulate_lines(cx, |lines| lines.shuffle(&mut thread_rng()))
    }

    fn manipulate_lines<Fn>(&mut self, cx: &mut ViewContext<Self>, mut callback: Fn)
    where
        Fn: FnMut(&mut [&str]),
    {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut edits = Vec::new();

        let selections = self.selections.all::<Point>(cx);
        let mut selections = selections.iter().peekable();
        let mut contiguous_row_selections = Vec::new();
        let mut new_selections = Vec::new();

        while let Some(selection) = selections.next() {
            let (start_row, end_row) = consume_contiguous_rows(
                &mut contiguous_row_selections,
                selection,
                &display_map,
                &mut selections,
            );

            let start_point = Point::new(start_row, 0);
            let end_point = Point::new(end_row - 1, buffer.line_len(end_row - 1));
            let text = buffer
                .text_for_range(start_point..end_point)
                .collect::<String>();
            let mut lines = text.split("\n").collect_vec();

            let lines_len = lines.len();
            callback(&mut lines);

            // This is a current limitation with selections.
            // If we wanted to support removing or adding lines, we'd need to fix the logic associated with selections.
            debug_assert!(
                lines.len() == lines_len,
                "callback should not change the number of lines"
            );

            edits.push((start_point..end_point, lines.join("\n")));
            let start_anchor = buffer.anchor_after(start_point);
            let end_anchor = buffer.anchor_before(end_point);

            // Make selection and push
            new_selections.push(Selection {
                id: selection.id,
                start: start_anchor.to_offset(&buffer),
                end: end_anchor.to_offset(&buffer),
                goal: SelectionGoal::None,
                reversed: selection.reversed,
            });
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });

            this.request_autoscroll(Autoscroll::fit(), cx);
        });
    }

    pub fn convert_to_upper_case(&mut self, _: &ConvertToUpperCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_uppercase())
    }

    pub fn convert_to_lower_case(&mut self, _: &ConvertToLowerCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_lowercase())
    }

    pub fn convert_to_title_case(&mut self, _: &ConvertToTitleCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_case(Case::Title))
    }

    pub fn convert_to_snake_case(&mut self, _: &ConvertToSnakeCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_case(Case::Snake))
    }

    pub fn convert_to_kebab_case(&mut self, _: &ConvertToKebabCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_case(Case::Kebab))
    }

    pub fn convert_to_upper_camel_case(
        &mut self,
        _: &ConvertToUpperCamelCase,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_text(cx, |text| text.to_case(Case::UpperCamel))
    }

    pub fn convert_to_lower_camel_case(
        &mut self,
        _: &ConvertToLowerCamelCase,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_text(cx, |text| text.to_case(Case::Camel))
    }

    fn manipulate_text<Fn>(&mut self, cx: &mut ViewContext<Self>, mut callback: Fn)
    where
        Fn: FnMut(&str) -> String,
    {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut new_selections = Vec::new();
        let mut edits = Vec::new();
        let mut selection_adjustment = 0i32;

        for selection in self.selections.all::<usize>(cx) {
            let selection_is_empty = selection.is_empty();

            let (start, end) = if selection_is_empty {
                let word_range = movement::surrounding_word(
                    &display_map,
                    selection.start.to_display_point(&display_map),
                );
                let start = word_range.start.to_offset(&display_map, Bias::Left);
                let end = word_range.end.to_offset(&display_map, Bias::Left);
                (start, end)
            } else {
                (selection.start, selection.end)
            };

            let text = buffer.text_for_range(start..end).collect::<String>();
            let old_length = text.len() as i32;
            let text = callback(&text);

            new_selections.push(Selection {
                start: (start as i32 - selection_adjustment) as usize,
                end: ((start + text.len()) as i32 - selection_adjustment) as usize,
                goal: SelectionGoal::None,
                ..selection
            });

            selection_adjustment += old_length - text.len() as i32;

            edits.push((start..end, text));
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });

            this.request_autoscroll(Autoscroll::fit(), cx);
        });
    }

    pub fn duplicate_line(&mut self, _: &DuplicateLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let selections = self.selections.all::<Point>(cx);

        let mut edits = Vec::new();
        let mut selections_iter = selections.iter().peekable();
        while let Some(selection) = selections_iter.next() {
            // Avoid duplicating the same lines twice.
            let mut rows = selection.spanned_rows(false, &display_map);

            while let Some(next_selection) = selections_iter.peek() {
                let next_rows = next_selection.spanned_rows(false, &display_map);
                if next_rows.start < rows.end {
                    rows.end = next_rows.end;
                    selections_iter.next().unwrap();
                } else {
                    break;
                }
            }

            // Copy the text from the selected row region and splice it at the start of the region.
            let start = Point::new(rows.start, 0);
            let end = Point::new(rows.end - 1, buffer.line_len(rows.end - 1));
            let text = buffer
                .text_for_range(start..end)
                .chain(Some("\n"))
                .collect::<String>();
            edits.push((start..start, text));
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            this.request_autoscroll(Autoscroll::fit(), cx);
        });
    }

    pub fn move_line_up(&mut self, _: &MoveLineUp, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut edits = Vec::new();
        let mut unfold_ranges = Vec::new();
        let mut refold_ranges = Vec::new();

        let selections = self.selections.all::<Point>(cx);
        let mut selections = selections.iter().peekable();
        let mut contiguous_row_selections = Vec::new();
        let mut new_selections = Vec::new();

        while let Some(selection) = selections.next() {
            // Find all the selections that span a contiguous row range
            let (start_row, end_row) = consume_contiguous_rows(
                &mut contiguous_row_selections,
                selection,
                &display_map,
                &mut selections,
            );

            // Move the text spanned by the row range to be before the line preceding the row range
            if start_row > 0 {
                let range_to_move = Point::new(start_row - 1, buffer.line_len(start_row - 1))
                    ..Point::new(end_row - 1, buffer.line_len(end_row - 1));
                let insertion_point = display_map
                    .prev_line_boundary(Point::new(start_row - 1, 0))
                    .0;

                // Don't move lines across excerpts
                if buffer
                    .excerpt_boundaries_in_range((
                        Bound::Excluded(insertion_point),
                        Bound::Included(range_to_move.end),
                    ))
                    .next()
                    .is_none()
                {
                    let text = buffer
                        .text_for_range(range_to_move.clone())
                        .flat_map(|s| s.chars())
                        .skip(1)
                        .chain(['\n'])
                        .collect::<String>();

                    edits.push((
                        buffer.anchor_after(range_to_move.start)
                            ..buffer.anchor_before(range_to_move.end),
                        String::new(),
                    ));
                    let insertion_anchor = buffer.anchor_after(insertion_point);
                    edits.push((insertion_anchor..insertion_anchor, text));

                    let row_delta = range_to_move.start.row - insertion_point.row + 1;

                    // Move selections up
                    new_selections.extend(contiguous_row_selections.drain(..).map(
                        |mut selection| {
                            selection.start.row -= row_delta;
                            selection.end.row -= row_delta;
                            selection
                        },
                    ));

                    // Move folds up
                    unfold_ranges.push(range_to_move.clone());
                    for fold in display_map.folds_in_range(
                        buffer.anchor_before(range_to_move.start)
                            ..buffer.anchor_after(range_to_move.end),
                    ) {
                        let mut start = fold.start.to_point(&buffer);
                        let mut end = fold.end.to_point(&buffer);
                        start.row -= row_delta;
                        end.row -= row_delta;
                        refold_ranges.push(start..end);
                    }
                }
            }

            // If we didn't move line(s), preserve the existing selections
            new_selections.append(&mut contiguous_row_selections);
        }

        self.transact(cx, |this, cx| {
            this.unfold_ranges(unfold_ranges, true, true, cx);
            this.buffer.update(cx, |buffer, cx| {
                for (range, text) in edits {
                    buffer.edit([(range, text)], None, cx);
                }
            });
            this.fold_ranges(refold_ranges, true, cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            })
        });
    }

    pub fn move_line_down(&mut self, _: &MoveLineDown, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut edits = Vec::new();
        let mut unfold_ranges = Vec::new();
        let mut refold_ranges = Vec::new();

        let selections = self.selections.all::<Point>(cx);
        let mut selections = selections.iter().peekable();
        let mut contiguous_row_selections = Vec::new();
        let mut new_selections = Vec::new();

        while let Some(selection) = selections.next() {
            // Find all the selections that span a contiguous row range
            let (start_row, end_row) = consume_contiguous_rows(
                &mut contiguous_row_selections,
                selection,
                &display_map,
                &mut selections,
            );

            // Move the text spanned by the row range to be after the last line of the row range
            if end_row <= buffer.max_point().row {
                let range_to_move = Point::new(start_row, 0)..Point::new(end_row, 0);
                let insertion_point = display_map.next_line_boundary(Point::new(end_row, 0)).0;

                // Don't move lines across excerpt boundaries
                if buffer
                    .excerpt_boundaries_in_range((
                        Bound::Excluded(range_to_move.start),
                        Bound::Included(insertion_point),
                    ))
                    .next()
                    .is_none()
                {
                    let mut text = String::from("\n");
                    text.extend(buffer.text_for_range(range_to_move.clone()));
                    text.pop(); // Drop trailing newline
                    edits.push((
                        buffer.anchor_after(range_to_move.start)
                            ..buffer.anchor_before(range_to_move.end),
                        String::new(),
                    ));
                    let insertion_anchor = buffer.anchor_after(insertion_point);
                    edits.push((insertion_anchor..insertion_anchor, text));

                    let row_delta = insertion_point.row - range_to_move.end.row + 1;

                    // Move selections down
                    new_selections.extend(contiguous_row_selections.drain(..).map(
                        |mut selection| {
                            selection.start.row += row_delta;
                            selection.end.row += row_delta;
                            selection
                        },
                    ));

                    // Move folds down
                    unfold_ranges.push(range_to_move.clone());
                    for fold in display_map.folds_in_range(
                        buffer.anchor_before(range_to_move.start)
                            ..buffer.anchor_after(range_to_move.end),
                    ) {
                        let mut start = fold.start.to_point(&buffer);
                        let mut end = fold.end.to_point(&buffer);
                        start.row += row_delta;
                        end.row += row_delta;
                        refold_ranges.push(start..end);
                    }
                }
            }

            // If we didn't move line(s), preserve the existing selections
            new_selections.append(&mut contiguous_row_selections);
        }

        self.transact(cx, |this, cx| {
            this.unfold_ranges(unfold_ranges, true, true, cx);
            this.buffer.update(cx, |buffer, cx| {
                for (range, text) in edits {
                    buffer.edit([(range, text)], None, cx);
                }
            });
            this.fold_ranges(refold_ranges, true, cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(new_selections));
        });
    }

    pub fn transpose(&mut self, _: &Transpose, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            let edits = this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let mut edits: Vec<(Range<usize>, String)> = Default::default();
                let line_mode = s.line_mode;
                s.move_with(|display_map, selection| {
                    if !selection.is_empty() || line_mode {
                        return;
                    }

                    let mut head = selection.head();
                    let mut transpose_offset = head.to_offset(display_map, Bias::Right);
                    if head.column() == display_map.line_len(head.row()) {
                        transpose_offset = display_map
                            .buffer_snapshot
                            .clip_offset(transpose_offset.saturating_sub(1), Bias::Left);
                    }

                    if transpose_offset == 0 {
                        return;
                    }

                    *head.column_mut() += 1;
                    head = display_map.clip_point(head, Bias::Right);
                    selection.collapse_to(head, SelectionGoal::Column(head.column()));

                    let transpose_start = display_map
                        .buffer_snapshot
                        .clip_offset(transpose_offset.saturating_sub(1), Bias::Left);
                    if edits.last().map_or(true, |e| e.0.end <= transpose_start) {
                        let transpose_end = display_map
                            .buffer_snapshot
                            .clip_offset(transpose_offset + 1, Bias::Right);
                        if let Some(ch) =
                            display_map.buffer_snapshot.chars_at(transpose_start).next()
                        {
                            edits.push((transpose_start..transpose_offset, String::new()));
                            edits.push((transpose_end..transpose_end, ch.to_string()));
                        }
                    }
                });
                edits
            });
            this.buffer
                .update(cx, |buffer, cx| buffer.edit(edits, None, cx));
            let selections = this.selections.all::<usize>(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(selections);
            });
        });
    }

    pub fn cut(&mut self, _: &Cut, cx: &mut ViewContext<Self>) {
        let mut text = String::new();
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selections = self.selections.all::<Point>(cx);
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let max_point = buffer.max_point();
            let mut is_first = true;
            for selection in &mut selections {
                let is_entire_line = selection.is_empty() || self.selections.line_mode;
                if is_entire_line {
                    selection.start = Point::new(selection.start.row, 0);
                    selection.end = cmp::min(max_point, Point::new(selection.end.row + 1, 0));
                    selection.goal = SelectionGoal::None;
                }
                if is_first {
                    is_first = false;
                } else {
                    text += "\n";
                }
                let mut len = 0;
                for chunk in buffer.text_for_range(selection.start..selection.end) {
                    text.push_str(chunk);
                    len += chunk.len();
                }
                clipboard_selections.push(ClipboardSelection {
                    len,
                    is_entire_line,
                    first_line_indent: buffer.indent_size_for_line(selection.start.row).len,
                });
            }
        }

        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(selections);
            });
            this.insert("", cx);
            cx.write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
        });
    }

    pub fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        let selections = self.selections.all::<Point>(cx);
        let buffer = self.buffer.read(cx).read(cx);
        let mut text = String::new();

        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let max_point = buffer.max_point();
            let mut is_first = true;
            for selection in selections.iter() {
                let mut start = selection.start;
                let mut end = selection.end;
                let is_entire_line = selection.is_empty() || self.selections.line_mode;
                if is_entire_line {
                    start = Point::new(start.row, 0);
                    end = cmp::min(max_point, Point::new(end.row + 1, 0));
                }
                if is_first {
                    is_first = false;
                } else {
                    text += "\n";
                }
                let mut len = 0;
                for chunk in buffer.text_for_range(start..end) {
                    text.push_str(chunk);
                    len += chunk.len();
                }
                clipboard_selections.push(ClipboardSelection {
                    len,
                    is_entire_line,
                    first_line_indent: buffer.indent_size_for_line(start.row).len,
                });
            }
        }

        cx.write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            if let Some(item) = cx.read_from_clipboard() {
                let clipboard_text = Cow::Borrowed(item.text());
                if let Some(mut clipboard_selections) = item.metadata::<Vec<ClipboardSelection>>() {
                    let old_selections = this.selections.all::<usize>(cx);
                    let all_selections_were_entire_line =
                        clipboard_selections.iter().all(|s| s.is_entire_line);
                    let first_selection_indent_column =
                        clipboard_selections.first().map(|s| s.first_line_indent);
                    if clipboard_selections.len() != old_selections.len() {
                        clipboard_selections.drain(..);
                    }

                    this.buffer.update(cx, |buffer, cx| {
                        let snapshot = buffer.read(cx);
                        let mut start_offset = 0;
                        let mut edits = Vec::new();
                        let mut original_indent_columns = Vec::new();
                        let line_mode = this.selections.line_mode;
                        for (ix, selection) in old_selections.iter().enumerate() {
                            let to_insert;
                            let entire_line;
                            let original_indent_column;
                            if let Some(clipboard_selection) = clipboard_selections.get(ix) {
                                let end_offset = start_offset + clipboard_selection.len;
                                to_insert = &clipboard_text[start_offset..end_offset];
                                entire_line = clipboard_selection.is_entire_line;
                                start_offset = end_offset + 1;
                                original_indent_column =
                                    Some(clipboard_selection.first_line_indent);
                            } else {
                                to_insert = clipboard_text.as_str();
                                entire_line = all_selections_were_entire_line;
                                original_indent_column = first_selection_indent_column
                            }

                            // If the corresponding selection was empty when this slice of the
                            // clipboard text was written, then the entire line containing the
                            // selection was copied. If this selection is also currently empty,
                            // then paste the line before the current line of the buffer.
                            let range = if selection.is_empty() && !line_mode && entire_line {
                                let column = selection.start.to_point(&snapshot).column as usize;
                                let line_start = selection.start - column;
                                line_start..line_start
                            } else {
                                selection.range()
                            };

                            edits.push((range, to_insert));
                            original_indent_columns.extend(original_indent_column);
                        }
                        drop(snapshot);

                        buffer.edit(
                            edits,
                            Some(AutoindentMode::Block {
                                original_indent_columns,
                            }),
                            cx,
                        );
                    });

                    let selections = this.selections.all::<usize>(cx);
                    this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
                } else {
                    this.insert(&clipboard_text, cx);
                }
            }
        });
    }

    pub fn undo(&mut self, _: &Undo, cx: &mut ViewContext<Self>) {
        if let Some(tx_id) = self.buffer.update(cx, |buffer, cx| buffer.undo(cx)) {
            if let Some((selections, _)) = self.selection_history.transaction(tx_id).cloned() {
                self.change_selections(None, cx, |s| {
                    s.select_anchors(selections.to_vec());
                });
            }
            self.request_autoscroll(Autoscroll::fit(), cx);
            self.unmark_text(cx);
            self.refresh_copilot_suggestions(true, cx);
            cx.emit(Event::Edited);
            cx.emit(Event::TransactionUndone {
                transaction_id: tx_id,
            });
        }
    }

    pub fn redo(&mut self, _: &Redo, cx: &mut ViewContext<Self>) {
        if let Some(tx_id) = self.buffer.update(cx, |buffer, cx| buffer.redo(cx)) {
            if let Some((_, Some(selections))) = self.selection_history.transaction(tx_id).cloned()
            {
                self.change_selections(None, cx, |s| {
                    s.select_anchors(selections.to_vec());
                });
            }
            self.request_autoscroll(Autoscroll::fit(), cx);
            self.unmark_text(cx);
            self.refresh_copilot_suggestions(true, cx);
            cx.emit(Event::Edited);
        }
    }

    pub fn finalize_last_transaction(&mut self, cx: &mut ViewContext<Self>) {
        self.buffer
            .update(cx, |buffer, cx| buffer.finalize_last_transaction(cx));
    }

    pub fn move_left(&mut self, _: &MoveLeft, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                let cursor = if selection.is_empty() && !line_mode {
                    movement::left(map, selection.start)
                } else {
                    selection.start
                };
                selection.collapse_to(cursor, SelectionGoal::None);
            });
        })
    }

    pub fn select_left(&mut self, _: &SelectLeft, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| (movement::left(map, head), SelectionGoal::None));
        })
    }

    pub fn move_right(&mut self, _: &MoveRight, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                let cursor = if selection.is_empty() && !line_mode {
                    movement::right(map, selection.end)
                } else {
                    selection.end
                };
                selection.collapse_to(cursor, SelectionGoal::None)
            });
        })
    }

    pub fn select_right(&mut self, _: &SelectRight, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| (movement::right(map, head), SelectionGoal::None));
        })
    }

    pub fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::up(map, selection.start, selection.goal, false);
                selection.collapse_to(cursor, goal);
            });
        })
    }

    pub fn move_page_up(&mut self, action: &MovePageUp, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let row_count = if let Some(row_count) = self.visible_line_count() {
            row_count as u32 - 1
        } else {
            return;
        };

        let autoscroll = if action.center_cursor {
            Autoscroll::center()
        } else {
            Autoscroll::fit()
        };

        self.change_selections(Some(autoscroll), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) =
                    movement::up_by_rows(map, selection.end, row_count, selection.goal, false);
                selection.collapse_to(cursor, goal);
            });
        });
    }

    pub fn select_up(&mut self, _: &SelectUp, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, goal| movement::up(map, head, goal, false))
        })
    }

    pub fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        self.take_rename(true, cx);

        if self.mode == EditorMode::SingleLine {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::down(map, selection.end, selection.goal, false);
                selection.collapse_to(cursor, goal);
            });
        });
    }

    pub fn move_page_down(&mut self, action: &MovePageDown, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if self
            .context_menu
            .as_mut()
            .map(|menu| menu.select_last(cx))
            .unwrap_or(false)
        {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let row_count = if let Some(row_count) = self.visible_line_count() {
            row_count as u32 - 1
        } else {
            return;
        };

        let autoscroll = if action.center_cursor {
            Autoscroll::center()
        } else {
            Autoscroll::fit()
        };

        self.change_selections(Some(autoscroll), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) =
                    movement::down_by_rows(map, selection.end, row_count, selection.goal, false);
                selection.collapse_to(cursor, goal);
            });
        });
    }

    pub fn select_down(&mut self, _: &SelectDown, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, goal| movement::down(map, head, goal, false))
        });
    }

    pub fn context_menu_first(&mut self, _: &ContextMenuFirst, cx: &mut ViewContext<Self>) {
        if let Some(context_menu) = self.context_menu.as_mut() {
            context_menu.select_first(cx);
        }
    }

    pub fn context_menu_prev(&mut self, _: &ContextMenuPrev, cx: &mut ViewContext<Self>) {
        if let Some(context_menu) = self.context_menu.as_mut() {
            context_menu.select_prev(cx);
        }
    }

    pub fn context_menu_next(&mut self, _: &ContextMenuNext, cx: &mut ViewContext<Self>) {
        if let Some(context_menu) = self.context_menu.as_mut() {
            context_menu.select_next(cx);
        }
    }

    pub fn context_menu_last(&mut self, _: &ContextMenuLast, cx: &mut ViewContext<Self>) {
        if let Some(context_menu) = self.context_menu.as_mut() {
            context_menu.select_last(cx);
        }
    }

    pub fn move_to_previous_word_start(
        &mut self,
        _: &MoveToPreviousWordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (
                    movement::previous_word_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_previous_subword_start(
        &mut self,
        _: &MoveToPreviousSubwordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (
                    movement::previous_subword_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_previous_word_start(
        &mut self,
        _: &SelectToPreviousWordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::previous_word_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_previous_subword_start(
        &mut self,
        _: &SelectToPreviousSubwordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::previous_subword_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn delete_to_previous_word_start(
        &mut self,
        _: &DeleteToPreviousWordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.transact(cx, |this, cx| {
            this.select_autoclose_pair(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::previous_word_start(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", cx);
        });
    }

    pub fn delete_to_previous_subword_start(
        &mut self,
        _: &DeleteToPreviousSubwordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.transact(cx, |this, cx| {
            this.select_autoclose_pair(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::previous_subword_start(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", cx);
        });
    }

    pub fn move_to_next_word_end(&mut self, _: &MoveToNextWordEnd, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (movement::next_word_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn move_to_next_subword_end(
        &mut self,
        _: &MoveToNextSubwordEnd,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (movement::next_subword_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn select_to_next_word_end(&mut self, _: &SelectToNextWordEnd, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (movement::next_word_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn select_to_next_subword_end(
        &mut self,
        _: &SelectToNextSubwordEnd,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (movement::next_subword_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn delete_to_next_word_end(&mut self, _: &DeleteToNextWordEnd, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::next_word_end(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", cx);
        });
    }

    pub fn delete_to_next_subword_end(
        &mut self,
        _: &DeleteToNextSubwordEnd,
        cx: &mut ViewContext<Self>,
    ) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    if selection.is_empty() {
                        let cursor = movement::next_subword_end(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", cx);
        });
    }

    pub fn move_to_beginning_of_line(
        &mut self,
        _: &MoveToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (
                    movement::indented_line_beginning(map, head, true),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_beginning_of_line(
        &mut self,
        action: &SelectToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::indented_line_beginning(map, head, action.stop_at_soft_wraps),
                    SelectionGoal::None,
                )
            });
        });
    }

    pub fn delete_to_beginning_of_line(
        &mut self,
        _: &DeleteToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|_, selection| {
                    selection.reversed = true;
                });
            });

            this.select_to_beginning_of_line(
                &SelectToBeginningOfLine {
                    stop_at_soft_wraps: false,
                },
                cx,
            );
            this.backspace(&Backspace, cx);
        });
    }

    pub fn move_to_end_of_line(&mut self, _: &MoveToEndOfLine, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (movement::line_end(map, head, true), SelectionGoal::None)
            });
        })
    }

    pub fn select_to_end_of_line(
        &mut self,
        action: &SelectToEndOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::line_end(map, head, action.stop_at_soft_wraps),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn delete_to_end_of_line(&mut self, _: &DeleteToEndOfLine, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.select_to_end_of_line(
                &SelectToEndOfLine {
                    stop_at_soft_wraps: false,
                },
                cx,
            );
            this.delete(&Delete, cx);
        });
    }

    pub fn cut_to_end_of_line(&mut self, _: &CutToEndOfLine, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.select_to_end_of_line(
                &SelectToEndOfLine {
                    stop_at_soft_wraps: false,
                },
                cx,
            );
            this.cut(&Cut, cx);
        });
    }

    pub fn move_to_start_of_paragraph(
        &mut self,
        _: &MoveToStartOfParagraph,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_with(|map, selection| {
                selection.collapse_to(
                    movement::start_of_paragraph(map, selection.head(), 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_end_of_paragraph(
        &mut self,
        _: &MoveToEndOfParagraph,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_with(|map, selection| {
                selection.collapse_to(
                    movement::end_of_paragraph(map, selection.head(), 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_start_of_paragraph(
        &mut self,
        _: &SelectToStartOfParagraph,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::start_of_paragraph(map, head, 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_end_of_paragraph(
        &mut self,
        _: &SelectToEndOfParagraph,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::end_of_paragraph(map, head, 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_beginning(&mut self, _: &MoveToBeginning, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select_ranges(vec![0..0]);
        });
    }

    pub fn select_to_beginning(&mut self, _: &SelectToBeginning, cx: &mut ViewContext<Self>) {
        let mut selection = self.selections.last::<Point>(cx);
        selection.set_head(Point::zero(), SelectionGoal::None);

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select(vec![selection]);
        });
    }

    pub fn move_to_end(&mut self, _: &MoveToEnd, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let cursor = self.buffer.read(cx).read(cx).len();
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select_ranges(vec![cursor..cursor])
        });
    }

    pub fn set_nav_history(&mut self, nav_history: Option<ItemNavHistory>) {
        self.nav_history = nav_history;
    }

    pub fn nav_history(&self) -> Option<&ItemNavHistory> {
        self.nav_history.as_ref()
    }

    fn push_to_nav_history(
        &mut self,
        cursor_anchor: Anchor,
        new_position: Option<Point>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(nav_history) = self.nav_history.as_mut() {
            let buffer = self.buffer.read(cx).read(cx);
            let cursor_position = cursor_anchor.to_point(&buffer);
            let scroll_state = self.scroll_manager.anchor();
            let scroll_top_row = scroll_state.top_row(&buffer);
            drop(buffer);

            if let Some(new_position) = new_position {
                let row_delta = (new_position.row as i64 - cursor_position.row as i64).abs();
                if row_delta < MIN_NAVIGATION_HISTORY_ROW_DELTA {
                    return;
                }
            }

            nav_history.push(
                Some(NavigationData {
                    cursor_anchor,
                    cursor_position,
                    scroll_anchor: scroll_state,
                    scroll_top_row,
                }),
                cx,
            );
        }
    }

    pub fn select_to_end(&mut self, _: &SelectToEnd, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selection = self.selections.first::<usize>(cx);
        selection.set_head(buffer.len(), SelectionGoal::None);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select(vec![selection]);
        });
    }

    pub fn select_all(&mut self, _: &SelectAll, cx: &mut ViewContext<Self>) {
        let end = self.buffer.read(cx).read(cx).len();
        self.change_selections(None, cx, |s| {
            s.select_ranges(vec![0..end]);
        });
    }

    pub fn select_line(&mut self, _: &SelectLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections.all::<Point>(cx);
        let max_point = display_map.buffer_snapshot.max_point();
        for selection in &mut selections {
            let rows = selection.spanned_rows(true, &display_map);
            selection.start = Point::new(rows.start, 0);
            selection.end = cmp::min(max_point, Point::new(rows.end, 0));
            selection.reversed = false;
        }
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select(selections);
        });
    }

    pub fn split_selection_into_lines(
        &mut self,
        _: &SplitSelectionIntoLines,
        cx: &mut ViewContext<Self>,
    ) {
        let mut to_unfold = Vec::new();
        let mut new_selection_ranges = Vec::new();
        {
            let selections = self.selections.all::<Point>(cx);
            let buffer = self.buffer.read(cx).read(cx);
            for selection in selections {
                for row in selection.start.row..selection.end.row {
                    let cursor = Point::new(row, buffer.line_len(row));
                    new_selection_ranges.push(cursor..cursor);
                }
                new_selection_ranges.push(selection.end..selection.end);
                to_unfold.push(selection.start..selection.end);
            }
        }
        self.unfold_ranges(to_unfold, true, true, cx);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select_ranges(new_selection_ranges);
        });
    }

    pub fn add_selection_above(&mut self, _: &AddSelectionAbove, cx: &mut ViewContext<Self>) {
        self.add_selection(true, cx);
    }

    pub fn add_selection_below(&mut self, _: &AddSelectionBelow, cx: &mut ViewContext<Self>) {
        self.add_selection(false, cx);
    }

    fn add_selection(&mut self, above: bool, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections.all::<Point>(cx);
        let mut state = self.add_selections_state.take().unwrap_or_else(|| {
            let oldest_selection = selections.iter().min_by_key(|s| s.id).unwrap().clone();
            let range = oldest_selection.display_range(&display_map).sorted();
            let columns = cmp::min(range.start.column(), range.end.column())
                ..cmp::max(range.start.column(), range.end.column());

            selections.clear();
            let mut stack = Vec::new();
            for row in range.start.row()..=range.end.row() {
                if let Some(selection) = self.selections.build_columnar_selection(
                    &display_map,
                    row,
                    &columns,
                    oldest_selection.reversed,
                ) {
                    stack.push(selection.id);
                    selections.push(selection);
                }
            }

            if above {
                stack.reverse();
            }

            AddSelectionsState { above, stack }
        });

        let last_added_selection = *state.stack.last().unwrap();
        let mut new_selections = Vec::new();
        if above == state.above {
            let end_row = if above {
                0
            } else {
                display_map.max_point().row()
            };

            'outer: for selection in selections {
                if selection.id == last_added_selection {
                    let range = selection.display_range(&display_map).sorted();
                    debug_assert_eq!(range.start.row(), range.end.row());
                    let mut row = range.start.row();
                    let columns = if let SelectionGoal::ColumnRange { start, end } = selection.goal
                    {
                        start..end
                    } else {
                        cmp::min(range.start.column(), range.end.column())
                            ..cmp::max(range.start.column(), range.end.column())
                    };

                    while row != end_row {
                        if above {
                            row -= 1;
                        } else {
                            row += 1;
                        }

                        if let Some(new_selection) = self.selections.build_columnar_selection(
                            &display_map,
                            row,
                            &columns,
                            selection.reversed,
                        ) {
                            state.stack.push(new_selection.id);
                            if above {
                                new_selections.push(new_selection);
                                new_selections.push(selection);
                            } else {
                                new_selections.push(selection);
                                new_selections.push(new_selection);
                            }

                            continue 'outer;
                        }
                    }
                }

                new_selections.push(selection);
            }
        } else {
            new_selections = selections;
            new_selections.retain(|s| s.id != last_added_selection);
            state.stack.pop();
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select(new_selections);
        });
        if state.stack.len() > 1 {
            self.add_selections_state = Some(state);
        }
    }

    pub fn select_next(&mut self, action: &SelectNext, cx: &mut ViewContext<Self>) {
        self.push_to_selection_history();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let mut selections = self.selections.all::<usize>(cx);
        if let Some(mut select_next_state) = self.select_next_state.take() {
            let query = &select_next_state.query;
            if !select_next_state.done {
                let first_selection = selections.iter().min_by_key(|s| s.id).unwrap();
                let last_selection = selections.iter().max_by_key(|s| s.id).unwrap();
                let mut next_selected_range = None;

                let bytes_after_last_selection =
                    buffer.bytes_in_range(last_selection.end..buffer.len());
                let bytes_before_first_selection = buffer.bytes_in_range(0..first_selection.start);
                let query_matches = query
                    .stream_find_iter(bytes_after_last_selection)
                    .map(|result| (last_selection.end, result))
                    .chain(
                        query
                            .stream_find_iter(bytes_before_first_selection)
                            .map(|result| (0, result)),
                    );
                for (start_offset, query_match) in query_matches {
                    let query_match = query_match.unwrap(); // can only fail due to I/O
                    let offset_range =
                        start_offset + query_match.start()..start_offset + query_match.end();
                    let display_range = offset_range.start.to_display_point(&display_map)
                        ..offset_range.end.to_display_point(&display_map);

                    if !select_next_state.wordwise
                        || (!movement::is_inside_word(&display_map, display_range.start)
                            && !movement::is_inside_word(&display_map, display_range.end))
                    {
                        next_selected_range = Some(offset_range);
                        break;
                    }
                }

                if let Some(next_selected_range) = next_selected_range {
                    self.unfold_ranges([next_selected_range.clone()], false, true, cx);
                    self.change_selections(Some(Autoscroll::newest()), cx, |s| {
                        if action.replace_newest {
                            s.delete(s.newest_anchor().id);
                        }
                        s.insert_range(next_selected_range);
                    });
                } else {
                    select_next_state.done = true;
                }
            }

            self.select_next_state = Some(select_next_state);
        } else if selections.len() == 1 {
            let selection = selections.last_mut().unwrap();
            if selection.start == selection.end {
                let word_range = movement::surrounding_word(
                    &display_map,
                    selection.start.to_display_point(&display_map),
                );
                selection.start = word_range.start.to_offset(&display_map, Bias::Left);
                selection.end = word_range.end.to_offset(&display_map, Bias::Left);
                selection.goal = SelectionGoal::None;
                selection.reversed = false;

                let query = buffer
                    .text_for_range(selection.start..selection.end)
                    .collect::<String>();
                let select_state = SelectNextState {
                    query: AhoCorasick::new_auto_configured(&[query]),
                    wordwise: true,
                    done: false,
                };
                self.unfold_ranges([selection.start..selection.end], false, true, cx);
                self.change_selections(Some(Autoscroll::newest()), cx, |s| {
                    s.select(selections);
                });
                self.select_next_state = Some(select_state);
            } else {
                let query = buffer
                    .text_for_range(selection.start..selection.end)
                    .collect::<String>();
                self.select_next_state = Some(SelectNextState {
                    query: AhoCorasick::new_auto_configured(&[query]),
                    wordwise: false,
                    done: false,
                });
                self.select_next(action, cx);
            }
        }
    }

    pub fn select_previous(&mut self, action: &SelectPrevious, cx: &mut ViewContext<Self>) {
        self.push_to_selection_history();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let mut selections = self.selections.all::<usize>(cx);
        if let Some(mut select_prev_state) = self.select_prev_state.take() {
            let query = &select_prev_state.query;
            if !select_prev_state.done {
                let first_selection = selections.iter().min_by_key(|s| s.id).unwrap();
                let last_selection = selections.iter().max_by_key(|s| s.id).unwrap();
                let mut next_selected_range = None;
                // When we're iterating matches backwards, the oldest match will actually be the furthest one in the buffer.
                let bytes_before_last_selection =
                    buffer.reversed_bytes_in_range(0..last_selection.start);
                let bytes_after_first_selection =
                    buffer.reversed_bytes_in_range(first_selection.end..buffer.len());
                let query_matches = query
                    .stream_find_iter(bytes_before_last_selection)
                    .map(|result| (last_selection.start, result))
                    .chain(
                        query
                            .stream_find_iter(bytes_after_first_selection)
                            .map(|result| (buffer.len(), result)),
                    );
                for (end_offset, query_match) in query_matches {
                    let query_match = query_match.unwrap(); // can only fail due to I/O
                    let offset_range =
                        end_offset - query_match.end()..end_offset - query_match.start();
                    let display_range = offset_range.start.to_display_point(&display_map)
                        ..offset_range.end.to_display_point(&display_map);

                    if !select_prev_state.wordwise
                        || (!movement::is_inside_word(&display_map, display_range.start)
                            && !movement::is_inside_word(&display_map, display_range.end))
                    {
                        next_selected_range = Some(offset_range);
                        break;
                    }
                }

                if let Some(next_selected_range) = next_selected_range {
                    self.unfold_ranges([next_selected_range.clone()], false, true, cx);
                    self.change_selections(Some(Autoscroll::newest()), cx, |s| {
                        if action.replace_newest {
                            s.delete(s.newest_anchor().id);
                        }
                        s.insert_range(next_selected_range);
                    });
                } else {
                    select_prev_state.done = true;
                }
            }

            self.select_prev_state = Some(select_prev_state);
        } else if selections.len() == 1 {
            let selection = selections.last_mut().unwrap();
            if selection.start == selection.end {
                let word_range = movement::surrounding_word(
                    &display_map,
                    selection.start.to_display_point(&display_map),
                );
                selection.start = word_range.start.to_offset(&display_map, Bias::Left);
                selection.end = word_range.end.to_offset(&display_map, Bias::Left);
                selection.goal = SelectionGoal::None;
                selection.reversed = false;

                let query = buffer
                    .text_for_range(selection.start..selection.end)
                    .collect::<String>();
                let query = query.chars().rev().collect::<String>();
                let select_state = SelectNextState {
                    query: AhoCorasick::new_auto_configured(&[query]),
                    wordwise: true,
                    done: false,
                };
                self.unfold_ranges([selection.start..selection.end], false, true, cx);
                self.change_selections(Some(Autoscroll::newest()), cx, |s| {
                    s.select(selections);
                });
                self.select_prev_state = Some(select_state);
            } else {
                let query = buffer
                    .text_for_range(selection.start..selection.end)
                    .collect::<String>();
                let query = query.chars().rev().collect::<String>();
                self.select_prev_state = Some(SelectNextState {
                    query: AhoCorasick::new_auto_configured(&[query]),
                    wordwise: false,
                    done: false,
                });
                self.select_previous(action, cx);
            }
        }
    }

    pub fn toggle_comments(&mut self, action: &ToggleComments, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            let mut selections = this.selections.all::<Point>(cx);
            let mut edits = Vec::new();
            let mut selection_edit_ranges = Vec::new();
            let mut last_toggled_row = None;
            let snapshot = this.buffer.read(cx).read(cx);
            let empty_str: Arc<str> = "".into();
            let mut suffixes_inserted = Vec::new();

            fn comment_prefix_range(
                snapshot: &MultiBufferSnapshot,
                row: u32,
                comment_prefix: &str,
                comment_prefix_whitespace: &str,
            ) -> Range<Point> {
                let start = Point::new(row, snapshot.indent_size_for_line(row).len);

                let mut line_bytes = snapshot
                    .bytes_in_range(start..snapshot.max_point())
                    .flatten()
                    .copied();

                // If this line currently begins with the line comment prefix, then record
                // the range containing the prefix.
                if line_bytes
                    .by_ref()
                    .take(comment_prefix.len())
                    .eq(comment_prefix.bytes())
                {
                    // Include any whitespace that matches the comment prefix.
                    let matching_whitespace_len = line_bytes
                        .zip(comment_prefix_whitespace.bytes())
                        .take_while(|(a, b)| a == b)
                        .count() as u32;
                    let end = Point::new(
                        start.row,
                        start.column + comment_prefix.len() as u32 + matching_whitespace_len,
                    );
                    start..end
                } else {
                    start..start
                }
            }

            fn comment_suffix_range(
                snapshot: &MultiBufferSnapshot,
                row: u32,
                comment_suffix: &str,
                comment_suffix_has_leading_space: bool,
            ) -> Range<Point> {
                let end = Point::new(row, snapshot.line_len(row));
                let suffix_start_column = end.column.saturating_sub(comment_suffix.len() as u32);

                let mut line_end_bytes = snapshot
                    .bytes_in_range(Point::new(end.row, suffix_start_column.saturating_sub(1))..end)
                    .flatten()
                    .copied();

                let leading_space_len = if suffix_start_column > 0
                    && line_end_bytes.next() == Some(b' ')
                    && comment_suffix_has_leading_space
                {
                    1
                } else {
                    0
                };

                // If this line currently begins with the line comment prefix, then record
                // the range containing the prefix.
                if line_end_bytes.by_ref().eq(comment_suffix.bytes()) {
                    let start = Point::new(end.row, suffix_start_column - leading_space_len);
                    start..end
                } else {
                    end..end
                }
            }

            // TODO: Handle selections that cross excerpts
            for selection in &mut selections {
                let start_column = snapshot.indent_size_for_line(selection.start.row).len;
                let language = if let Some(language) =
                    snapshot.language_scope_at(Point::new(selection.start.row, start_column))
                {
                    language
                } else {
                    continue;
                };

                selection_edit_ranges.clear();

                // If multiple selections contain a given row, avoid processing that
                // row more than once.
                let mut start_row = selection.start.row;
                if last_toggled_row == Some(start_row) {
                    start_row += 1;
                }
                let end_row =
                    if selection.end.row > selection.start.row && selection.end.column == 0 {
                        selection.end.row - 1
                    } else {
                        selection.end.row
                    };
                last_toggled_row = Some(end_row);

                if start_row > end_row {
                    continue;
                }

                // If the language has line comments, toggle those.
                if let Some(full_comment_prefix) = language.line_comment_prefix() {
                    // Split the comment prefix's trailing whitespace into a separate string,
                    // as that portion won't be used for detecting if a line is a comment.
                    let comment_prefix = full_comment_prefix.trim_end_matches(' ');
                    let comment_prefix_whitespace = &full_comment_prefix[comment_prefix.len()..];
                    let mut all_selection_lines_are_comments = true;

                    for row in start_row..=end_row {
                        if snapshot.is_line_blank(row) && start_row < end_row {
                            continue;
                        }

                        let prefix_range = comment_prefix_range(
                            snapshot.deref(),
                            row,
                            comment_prefix,
                            comment_prefix_whitespace,
                        );
                        if prefix_range.is_empty() {
                            all_selection_lines_are_comments = false;
                        }
                        selection_edit_ranges.push(prefix_range);
                    }

                    if all_selection_lines_are_comments {
                        edits.extend(
                            selection_edit_ranges
                                .iter()
                                .cloned()
                                .map(|range| (range, empty_str.clone())),
                        );
                    } else {
                        let min_column = selection_edit_ranges
                            .iter()
                            .map(|r| r.start.column)
                            .min()
                            .unwrap_or(0);
                        edits.extend(selection_edit_ranges.iter().map(|range| {
                            let position = Point::new(range.start.row, min_column);
                            (position..position, full_comment_prefix.clone())
                        }));
                    }
                } else if let Some((full_comment_prefix, comment_suffix)) =
                    language.block_comment_delimiters()
                {
                    let comment_prefix = full_comment_prefix.trim_end_matches(' ');
                    let comment_prefix_whitespace = &full_comment_prefix[comment_prefix.len()..];
                    let prefix_range = comment_prefix_range(
                        snapshot.deref(),
                        start_row,
                        comment_prefix,
                        comment_prefix_whitespace,
                    );
                    let suffix_range = comment_suffix_range(
                        snapshot.deref(),
                        end_row,
                        comment_suffix.trim_start_matches(' '),
                        comment_suffix.starts_with(' '),
                    );

                    if prefix_range.is_empty() || suffix_range.is_empty() {
                        edits.push((
                            prefix_range.start..prefix_range.start,
                            full_comment_prefix.clone(),
                        ));
                        edits.push((suffix_range.end..suffix_range.end, comment_suffix.clone()));
                        suffixes_inserted.push((end_row, comment_suffix.len()));
                    } else {
                        edits.push((prefix_range, empty_str.clone()));
                        edits.push((suffix_range, empty_str.clone()));
                    }
                } else {
                    continue;
                }
            }

            drop(snapshot);
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            // Adjust selections so that they end before any comment suffixes that
            // were inserted.
            let mut suffixes_inserted = suffixes_inserted.into_iter().peekable();
            let mut selections = this.selections.all::<Point>(cx);
            let snapshot = this.buffer.read(cx).read(cx);
            for selection in &mut selections {
                while let Some((row, suffix_len)) = suffixes_inserted.peek().copied() {
                    match row.cmp(&selection.end.row) {
                        Ordering::Less => {
                            suffixes_inserted.next();
                            continue;
                        }
                        Ordering::Greater => break,
                        Ordering::Equal => {
                            if selection.end.column == snapshot.line_len(row) {
                                if selection.is_empty() {
                                    selection.start.column -= suffix_len as u32;
                                }
                                selection.end.column -= suffix_len as u32;
                            }
                            break;
                        }
                    }
                }
            }

            drop(snapshot);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));

            let selections = this.selections.all::<Point>(cx);
            let selections_on_single_row = selections.windows(2).all(|selections| {
                selections[0].start.row == selections[1].start.row
                    && selections[0].end.row == selections[1].end.row
                    && selections[0].start.row == selections[0].end.row
            });
            let selections_selecting = selections
                .iter()
                .any(|selection| selection.start != selection.end);
            let advance_downwards = action.advance_downwards
                && selections_on_single_row
                && !selections_selecting
                && this.mode != EditorMode::SingleLine;

            if advance_downwards {
                let snapshot = this.buffer.read(cx).snapshot(cx);

                this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_cursors_with(|display_snapshot, display_point, _| {
                        let mut point = display_point.to_point(display_snapshot);
                        point.row += 1;
                        point = snapshot.clip_point(point, Bias::Left);
                        let display_point = point.to_display_point(display_snapshot);
                        (display_point, SelectionGoal::Column(display_point.column()))
                    })
                });
            }
        });
    }

    pub fn select_larger_syntax_node(
        &mut self,
        _: &SelectLargerSyntaxNode,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);
        let old_selections = self.selections.all::<usize>(cx).into_boxed_slice();

        let mut stack = mem::take(&mut self.select_larger_syntax_node_stack);
        let mut selected_larger_node = false;
        let new_selections = old_selections
            .iter()
            .map(|selection| {
                let old_range = selection.start..selection.end;
                let mut new_range = old_range.clone();
                while let Some(containing_range) =
                    buffer.range_for_syntax_ancestor(new_range.clone())
                {
                    new_range = containing_range;
                    if !display_map.intersects_fold(new_range.start)
                        && !display_map.intersects_fold(new_range.end)
                    {
                        break;
                    }
                }

                selected_larger_node |= new_range != old_range;
                Selection {
                    id: selection.id,
                    start: new_range.start,
                    end: new_range.end,
                    goal: SelectionGoal::None,
                    reversed: selection.reversed,
                }
            })
            .collect::<Vec<_>>();

        if selected_larger_node {
            stack.push(old_selections);
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });
        }
        self.select_larger_syntax_node_stack = stack;
    }

    pub fn select_smaller_syntax_node(
        &mut self,
        _: &SelectSmallerSyntaxNode,
        cx: &mut ViewContext<Self>,
    ) {
        let mut stack = mem::take(&mut self.select_larger_syntax_node_stack);
        if let Some(selections) = stack.pop() {
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(selections.to_vec());
            });
        }
        self.select_larger_syntax_node_stack = stack;
    }

    pub fn move_to_enclosing_bracket(
        &mut self,
        _: &MoveToEnclosingBracket,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_offsets_with(|snapshot, selection| {
                let Some(enclosing_bracket_ranges) =
                    snapshot.enclosing_bracket_ranges(selection.start..selection.end)
                else {
                    return;
                };

                let mut best_length = usize::MAX;
                let mut best_inside = false;
                let mut best_in_bracket_range = false;
                let mut best_destination = None;
                for (open, close) in enclosing_bracket_ranges {
                    let close = close.to_inclusive();
                    let length = close.end() - open.start;
                    let inside = selection.start >= open.end && selection.end <= *close.start();
                    let in_bracket_range = open.to_inclusive().contains(&selection.head())
                        || close.contains(&selection.head());

                    // If best is next to a bracket and current isn't, skip
                    if !in_bracket_range && best_in_bracket_range {
                        continue;
                    }

                    // Prefer smaller lengths unless best is inside and current isn't
                    if length > best_length && (best_inside || !inside) {
                        continue;
                    }

                    best_length = length;
                    best_inside = inside;
                    best_in_bracket_range = in_bracket_range;
                    best_destination = Some(
                        if close.contains(&selection.start) && close.contains(&selection.end) {
                            if inside {
                                open.end
                            } else {
                                open.start
                            }
                        } else {
                            if inside {
                                *close.start()
                            } else {
                                *close.end()
                            }
                        },
                    );
                }

                if let Some(destination) = best_destination {
                    selection.collapse_to(destination, SelectionGoal::None);
                }
            })
        });
    }

    pub fn undo_selection(&mut self, _: &UndoSelection, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        self.selection_history.mode = SelectionHistoryMode::Undoing;
        if let Some(entry) = self.selection_history.undo_stack.pop_back() {
            self.change_selections(None, cx, |s| s.select_anchors(entry.selections.to_vec()));
            self.select_next_state = entry.select_next_state;
            self.select_prev_state = entry.select_prev_state;
            self.add_selections_state = entry.add_selections_state;
            self.request_autoscroll(Autoscroll::newest(), cx);
        }
        self.selection_history.mode = SelectionHistoryMode::Normal;
    }

    pub fn redo_selection(&mut self, _: &RedoSelection, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        self.selection_history.mode = SelectionHistoryMode::Redoing;
        if let Some(entry) = self.selection_history.redo_stack.pop_back() {
            self.change_selections(None, cx, |s| s.select_anchors(entry.selections.to_vec()));
            self.select_next_state = entry.select_next_state;
            self.select_prev_state = entry.select_prev_state;
            self.add_selections_state = entry.add_selections_state;
            self.request_autoscroll(Autoscroll::newest(), cx);
        }
        self.selection_history.mode = SelectionHistoryMode::Normal;
    }

    fn go_to_diagnostic(&mut self, _: &GoToDiagnostic, cx: &mut ViewContext<Self>) {
        self.go_to_diagnostic_impl(Direction::Next, cx)
    }

    fn go_to_prev_diagnostic(&mut self, _: &GoToPrevDiagnostic, cx: &mut ViewContext<Self>) {
        self.go_to_diagnostic_impl(Direction::Prev, cx)
    }

    pub fn go_to_diagnostic_impl(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let selection = self.selections.newest::<usize>(cx);

        // If there is an active Diagnostic Popover. Jump to it's diagnostic instead.
        if direction == Direction::Next {
            if let Some(popover) = self.hover_state.diagnostic_popover.as_ref() {
                let (group_id, jump_to) = popover.activation_info();
                if self.activate_diagnostics(group_id, cx) {
                    self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        let mut new_selection = s.newest_anchor().clone();
                        new_selection.collapse_to(jump_to, SelectionGoal::None);
                        s.select_anchors(vec![new_selection.clone()]);
                    });
                }
                return;
            }
        }

        let mut active_primary_range = self.active_diagnostics.as_ref().map(|active_diagnostics| {
            active_diagnostics
                .primary_range
                .to_offset(&buffer)
                .to_inclusive()
        });
        let mut search_start = if let Some(active_primary_range) = active_primary_range.as_ref() {
            if active_primary_range.contains(&selection.head()) {
                *active_primary_range.end()
            } else {
                selection.head()
            }
        } else {
            selection.head()
        };

        loop {
            let mut diagnostics = if direction == Direction::Prev {
                buffer.diagnostics_in_range::<_, usize>(0..search_start, true)
            } else {
                buffer.diagnostics_in_range::<_, usize>(search_start..buffer.len(), false)
            };
            let group = diagnostics.find_map(|entry| {
                if entry.diagnostic.is_primary
                    && entry.diagnostic.severity <= DiagnosticSeverity::WARNING
                    && !entry.range.is_empty()
                    && Some(entry.range.end) != active_primary_range.as_ref().map(|r| *r.end())
                {
                    Some((entry.range, entry.diagnostic.group_id))
                } else {
                    None
                }
            });

            if let Some((primary_range, group_id)) = group {
                if self.activate_diagnostics(group_id, cx) {
                    self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select(vec![Selection {
                            id: selection.id,
                            start: primary_range.start,
                            end: primary_range.start,
                            reversed: false,
                            goal: SelectionGoal::None,
                        }]);
                    });
                }
                break;
            } else {
                // Cycle around to the start of the buffer, potentially moving back to the start of
                // the currently active diagnostic.
                active_primary_range.take();
                if direction == Direction::Prev {
                    if search_start == buffer.len() {
                        break;
                    } else {
                        search_start = buffer.len();
                    }
                } else if search_start == 0 {
                    break;
                } else {
                    search_start = 0;
                }
            }
        }
    }

    fn go_to_hunk(&mut self, _: &GoToHunk, cx: &mut ViewContext<Self>) {
        let snapshot = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let selection = self.selections.newest::<Point>(cx);

        if !self.seek_in_direction(
            &snapshot,
            selection.head(),
            false,
            snapshot
                .buffer_snapshot
                .git_diff_hunks_in_range((selection.head().row + 1)..u32::MAX),
            cx,
        ) {
            let wrapped_point = Point::zero();
            self.seek_in_direction(
                &snapshot,
                wrapped_point,
                true,
                snapshot
                    .buffer_snapshot
                    .git_diff_hunks_in_range((wrapped_point.row + 1)..u32::MAX),
                cx,
            );
        }
    }

    fn go_to_prev_hunk(&mut self, _: &GoToPrevHunk, cx: &mut ViewContext<Self>) {
        let snapshot = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let selection = self.selections.newest::<Point>(cx);

        if !self.seek_in_direction(
            &snapshot,
            selection.head(),
            false,
            snapshot
                .buffer_snapshot
                .git_diff_hunks_in_range_rev(0..selection.head().row),
            cx,
        ) {
            let wrapped_point = snapshot.buffer_snapshot.max_point();
            self.seek_in_direction(
                &snapshot,
                wrapped_point,
                true,
                snapshot
                    .buffer_snapshot
                    .git_diff_hunks_in_range_rev(0..wrapped_point.row),
                cx,
            );
        }
    }

    fn seek_in_direction(
        &mut self,
        snapshot: &DisplaySnapshot,
        initial_point: Point,
        is_wrapped: bool,
        hunks: impl Iterator<Item = DiffHunk<u32>>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        let display_point = initial_point.to_display_point(snapshot);
        let mut hunks = hunks
            .map(|hunk| diff_hunk_to_display(hunk, &snapshot))
            .skip_while(|hunk| {
                if is_wrapped {
                    false
                } else {
                    hunk.contains_display_row(display_point.row())
                }
            })
            .dedup();

        if let Some(hunk) = hunks.next() {
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let row = hunk.start_display_row();
                let point = DisplayPoint::new(row, 0);
                s.select_display_ranges([point..point]);
            });

            true
        } else {
            false
        }
    }

    pub fn go_to_definition(&mut self, _: &GoToDefinition, cx: &mut ViewContext<Self>) {
        self.go_to_definition_of_kind(GotoDefinitionKind::Symbol, false, cx);
    }

    pub fn go_to_type_definition(&mut self, _: &GoToTypeDefinition, cx: &mut ViewContext<Self>) {
        self.go_to_definition_of_kind(GotoDefinitionKind::Type, false, cx);
    }

    pub fn go_to_definition_split(&mut self, _: &GoToDefinitionSplit, cx: &mut ViewContext<Self>) {
        self.go_to_definition_of_kind(GotoDefinitionKind::Symbol, true, cx);
    }

    pub fn go_to_type_definition_split(
        &mut self,
        _: &GoToTypeDefinitionSplit,
        cx: &mut ViewContext<Self>,
    ) {
        self.go_to_definition_of_kind(GotoDefinitionKind::Type, true, cx);
    }

    fn go_to_definition_of_kind(
        &mut self,
        kind: GotoDefinitionKind,
        split: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(workspace) = self.workspace(cx) else {
            return;
        };
        let buffer = self.buffer.read(cx);
        let head = self.selections.newest::<usize>(cx).head();
        let (buffer, head) = if let Some(text_anchor) = buffer.text_anchor_for_position(head, cx) {
            text_anchor
        } else {
            return;
        };

        let project = workspace.read(cx).project().clone();
        let definitions = project.update(cx, |project, cx| match kind {
            GotoDefinitionKind::Symbol => project.definition(&buffer, head, cx),
            GotoDefinitionKind::Type => project.type_definition(&buffer, head, cx),
        });

        cx.spawn_labeled("Fetching Definition...", |editor, mut cx| async move {
            let definitions = definitions.await?;
            editor.update(&mut cx, |editor, cx| {
                editor.navigate_to_definitions(
                    definitions
                        .into_iter()
                        .map(GoToDefinitionLink::Text)
                        .collect(),
                    split,
                    cx,
                );
            })?;
            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }

    pub fn navigate_to_definitions(
        &mut self,
        mut definitions: Vec<GoToDefinitionLink>,
        split: bool,
        cx: &mut ViewContext<Editor>,
    ) {
        let Some(workspace) = self.workspace(cx) else {
            return;
        };
        let pane = workspace.read(cx).active_pane().clone();
        // If there is one definition, just open it directly
        if definitions.len() == 1 {
            let definition = definitions.pop().unwrap();
            let target_task = match definition {
                GoToDefinitionLink::Text(link) => Task::Ready(Some(Ok(Some(link.target)))),
                GoToDefinitionLink::InlayHint(lsp_location, server_id) => {
                    self.compute_target_location(lsp_location, server_id, cx)
                }
            };
            cx.spawn(|editor, mut cx| async move {
                let target = target_task.await.context("target resolution task")?;
                if let Some(target) = target {
                    editor.update(&mut cx, |editor, cx| {
                        let range = target.range.to_offset(target.buffer.read(cx));
                        let range = editor.range_for_match(&range);
                        if Some(&target.buffer) == editor.buffer.read(cx).as_singleton().as_ref() {
                            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                                s.select_ranges([range]);
                            });
                        } else {
                            cx.window_context().defer(move |cx| {
                                let target_editor: ViewHandle<Self> =
                                    workspace.update(cx, |workspace, cx| {
                                        if split {
                                            workspace.split_project_item(target.buffer.clone(), cx)
                                        } else {
                                            workspace.open_project_item(target.buffer.clone(), cx)
                                        }
                                    });
                                target_editor.update(cx, |target_editor, cx| {
                                    // When selecting a definition in a different buffer, disable the nav history
                                    // to avoid creating a history entry at the previous cursor location.
                                    pane.update(cx, |pane, _| pane.disable_history());
                                    target_editor.change_selections(
                                        Some(Autoscroll::fit()),
                                        cx,
                                        |s| {
                                            s.select_ranges([range]);
                                        },
                                    );
                                    pane.update(cx, |pane, _| pane.enable_history());
                                });
                            });
                        }
                    })
                } else {
                    Ok(())
                }
            })
            .detach_and_log_err(cx);
        } else if !definitions.is_empty() {
            let replica_id = self.replica_id(cx);
            cx.spawn(|editor, mut cx| async move {
                let (title, location_tasks) = editor
                    .update(&mut cx, |editor, cx| {
                        let title = definitions
                            .iter()
                            .find_map(|definition| match definition {
                                GoToDefinitionLink::Text(link) => {
                                    link.origin.as_ref().map(|origin| {
                                        let buffer = origin.buffer.read(cx);
                                        format!(
                                            "Definitions for {}",
                                            buffer
                                                .text_for_range(origin.range.clone())
                                                .collect::<String>()
                                        )
                                    })
                                }
                                GoToDefinitionLink::InlayHint(_, _) => None,
                            })
                            .unwrap_or("Definitions".to_string());
                        let location_tasks = definitions
                            .into_iter()
                            .map(|definition| match definition {
                                GoToDefinitionLink::Text(link) => {
                                    Task::Ready(Some(Ok(Some(link.target))))
                                }
                                GoToDefinitionLink::InlayHint(lsp_location, server_id) => {
                                    editor.compute_target_location(lsp_location, server_id, cx)
                                }
                            })
                            .collect::<Vec<_>>();
                        (title, location_tasks)
                    })
                    .context("location tasks preparation")?;

                let locations = futures::future::join_all(location_tasks)
                    .await
                    .into_iter()
                    .filter_map(|location| location.transpose())
                    .collect::<Result<_>>()
                    .context("location tasks")?;
                workspace.update(&mut cx, |workspace, cx| {
                    Self::open_locations_in_multibuffer(
                        workspace, locations, replica_id, title, split, cx,
                    )
                });

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn compute_target_location(
        &self,
        lsp_location: lsp::Location,
        server_id: LanguageServerId,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<Option<Location>>> {
        let Some(project) = self.project.clone() else {
            return Task::Ready(Some(Ok(None)));
        };

        cx.spawn(move |editor, mut cx| async move {
            let location_task = editor.update(&mut cx, |editor, cx| {
                project.update(cx, |project, cx| {
                    let language_server_name =
                        editor.buffer.read(cx).as_singleton().and_then(|buffer| {
                            project
                                .language_server_for_buffer(buffer.read(cx), server_id, cx)
                                .map(|(_, lsp_adapter)| {
                                    LanguageServerName(Arc::from(lsp_adapter.name()))
                                })
                        });
                    language_server_name.map(|language_server_name| {
                        project.open_local_buffer_via_lsp(
                            lsp_location.uri.clone(),
                            server_id,
                            language_server_name,
                            cx,
                        )
                    })
                })
            })?;
            let location = match location_task {
                Some(task) => Some({
                    let target_buffer_handle = task.await.context("open local buffer")?;
                    let range = {
                        target_buffer_handle.update(&mut cx, |target_buffer, _| {
                            let target_start = target_buffer.clip_point_utf16(
                                point_from_lsp(lsp_location.range.start),
                                Bias::Left,
                            );
                            let target_end = target_buffer.clip_point_utf16(
                                point_from_lsp(lsp_location.range.end),
                                Bias::Left,
                            );
                            target_buffer.anchor_after(target_start)
                                ..target_buffer.anchor_before(target_end)
                        })
                    };
                    Location {
                        buffer: target_buffer_handle,
                        range,
                    }
                }),
                None => None,
            };
            Ok(location)
        })
    }

    pub fn find_all_references(
        workspace: &mut Workspace,
        _: &FindAllReferences,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        let active_item = workspace.active_item(cx)?;
        let editor_handle = active_item.act_as::<Self>(cx)?;

        let editor = editor_handle.read(cx);
        let buffer = editor.buffer.read(cx);
        let head = editor.selections.newest::<usize>(cx).head();
        let (buffer, head) = buffer.text_anchor_for_position(head, cx)?;
        let replica_id = editor.replica_id(cx);

        let project = workspace.project().clone();
        let references = project.update(cx, |project, cx| project.references(&buffer, head, cx));
        Some(cx.spawn_labeled(
            "Finding All References...",
            |workspace, mut cx| async move {
                let locations = references.await?;
                if locations.is_empty() {
                    return Ok(());
                }

                workspace.update(&mut cx, |workspace, cx| {
                    let title = locations
                        .first()
                        .as_ref()
                        .map(|location| {
                            let buffer = location.buffer.read(cx);
                            format!(
                                "References to `{}`",
                                buffer
                                    .text_for_range(location.range.clone())
                                    .collect::<String>()
                            )
                        })
                        .unwrap();
                    Self::open_locations_in_multibuffer(
                        workspace, locations, replica_id, title, false, cx,
                    );
                })?;

                Ok(())
            },
        ))
    }

    /// Opens a multibuffer with the given project locations in it
    pub fn open_locations_in_multibuffer(
        workspace: &mut Workspace,
        mut locations: Vec<Location>,
        replica_id: ReplicaId,
        title: String,
        split: bool,
        cx: &mut ViewContext<Workspace>,
    ) {
        // If there are multiple definitions, open them in a multibuffer
        locations.sort_by_key(|location| location.buffer.read(cx).remote_id());
        let mut locations = locations.into_iter().peekable();
        let mut ranges_to_highlight = Vec::new();

        let excerpt_buffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(replica_id);
            while let Some(location) = locations.next() {
                let buffer = location.buffer.read(cx);
                let mut ranges_for_buffer = Vec::new();
                let range = location.range.to_offset(buffer);
                ranges_for_buffer.push(range.clone());

                while let Some(next_location) = locations.peek() {
                    if next_location.buffer == location.buffer {
                        ranges_for_buffer.push(next_location.range.to_offset(buffer));
                        locations.next();
                    } else {
                        break;
                    }
                }

                ranges_for_buffer.sort_by_key(|range| (range.start, Reverse(range.end)));
                ranges_to_highlight.extend(multibuffer.push_excerpts_with_context_lines(
                    location.buffer.clone(),
                    ranges_for_buffer,
                    1,
                    cx,
                ))
            }

            multibuffer.with_title(title)
        });

        let editor = cx.add_view(|cx| {
            Editor::for_multibuffer(excerpt_buffer, Some(workspace.project().clone()), cx)
        });
        editor.update(cx, |editor, cx| {
            editor.highlight_background::<Self>(
                ranges_to_highlight,
                |theme| theme.editor.highlighted_line_background,
                cx,
            );
        });
        if split {
            workspace.split_item(Box::new(editor), cx);
        } else {
            workspace.add_item(Box::new(editor), cx);
        }
    }

    pub fn rename(&mut self, _: &Rename, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        use language::ToOffset as _;

        let project = self.project.clone()?;
        let selection = self.selections.newest_anchor().clone();
        let (cursor_buffer, cursor_buffer_position) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(selection.head(), cx)?;
        let (tail_buffer, _) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(selection.tail(), cx)?;
        if tail_buffer != cursor_buffer {
            return None;
        }

        let snapshot = cursor_buffer.read(cx).snapshot();
        let cursor_buffer_offset = cursor_buffer_position.to_offset(&snapshot);
        let prepare_rename = project.update(cx, |project, cx| {
            project.prepare_rename(cursor_buffer, cursor_buffer_offset, cx)
        });

        Some(cx.spawn(|this, mut cx| async move {
            let rename_range = if let Some(range) = prepare_rename.await? {
                Some(range)
            } else {
                this.update(&mut cx, |this, cx| {
                    let buffer = this.buffer.read(cx).snapshot(cx);
                    let display_snapshot = this
                        .display_map
                        .update(cx, |display_map, cx| display_map.snapshot(cx));
                    let mut buffer_highlights = this
                        .document_highlights_for_position(
                            selection.head(),
                            &buffer,
                            &display_snapshot,
                        )
                        .filter_map(|highlight| highlight.as_text_range())
                        .filter(|highlight| {
                            highlight.start.excerpt_id() == selection.head().excerpt_id()
                                && highlight.end.excerpt_id() == selection.head().excerpt_id()
                        });
                    buffer_highlights
                        .next()
                        .map(|highlight| highlight.start.text_anchor..highlight.end.text_anchor)
                })?
            };
            if let Some(rename_range) = rename_range {
                let rename_buffer_range = rename_range.to_offset(&snapshot);
                let cursor_offset_in_rename_range =
                    cursor_buffer_offset.saturating_sub(rename_buffer_range.start);

                this.update(&mut cx, |this, cx| {
                    this.take_rename(false, cx);
                    let style = this.style(cx);
                    let buffer = this.buffer.read(cx).read(cx);
                    let cursor_offset = selection.head().to_offset(&buffer);
                    let rename_start = cursor_offset.saturating_sub(cursor_offset_in_rename_range);
                    let rename_end = rename_start + rename_buffer_range.len();
                    let range = buffer.anchor_before(rename_start)..buffer.anchor_after(rename_end);
                    let mut old_highlight_id = None;
                    let old_name: Arc<str> = buffer
                        .chunks(rename_start..rename_end, true)
                        .map(|chunk| {
                            if old_highlight_id.is_none() {
                                old_highlight_id = chunk.syntax_highlight_id;
                            }
                            chunk.text
                        })
                        .collect::<String>()
                        .into();

                    drop(buffer);

                    // Position the selection in the rename editor so that it matches the current selection.
                    this.show_local_selections = false;
                    let rename_editor = cx.add_view(|cx| {
                        let mut editor = Editor::single_line(None, cx);
                        if let Some(old_highlight_id) = old_highlight_id {
                            editor.override_text_style =
                                Some(Box::new(move |style| old_highlight_id.style(&style.syntax)));
                        }
                        editor.buffer.update(cx, |buffer, cx| {
                            buffer.edit([(0..0, old_name.clone())], None, cx)
                        });
                        editor.select_all(&SelectAll, cx);
                        editor
                    });

                    let ranges = this
                        .clear_background_highlights::<DocumentHighlightWrite>(cx)
                        .into_iter()
                        .flat_map(|(_, ranges)| {
                            ranges.into_iter().filter_map(|range| range.as_text_range())
                        })
                        .chain(
                            this.clear_background_highlights::<DocumentHighlightRead>(cx)
                                .into_iter()
                                .flat_map(|(_, ranges)| {
                                    ranges.into_iter().filter_map(|range| range.as_text_range())
                                }),
                        )
                        .collect();

                    this.highlight_text::<Rename>(
                        ranges,
                        HighlightStyle {
                            fade_out: Some(style.rename_fade),
                            ..Default::default()
                        },
                        cx,
                    );
                    cx.focus(&rename_editor);
                    let block_id = this.insert_blocks(
                        [BlockProperties {
                            style: BlockStyle::Flex,
                            position: range.start.clone(),
                            height: 1,
                            render: Arc::new({
                                let editor = rename_editor.clone();
                                move |cx: &mut BlockContext| {
                                    ChildView::new(&editor, cx)
                                        .contained()
                                        .with_padding_left(cx.anchor_x)
                                        .into_any()
                                }
                            }),
                            disposition: BlockDisposition::Below,
                        }],
                        Some(Autoscroll::fit()),
                        cx,
                    )[0];
                    this.pending_rename = Some(RenameState {
                        range,
                        old_name,
                        editor: rename_editor,
                        block_id,
                    });
                })?;
            }

            Ok(())
        }))
    }

    pub fn confirm_rename(
        workspace: &mut Workspace,
        _: &ConfirmRename,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        let editor = workspace.active_item(cx)?.act_as::<Editor>(cx)?;

        let (buffer, range, old_name, new_name) = editor.update(cx, |editor, cx| {
            let rename = editor.take_rename(false, cx)?;
            let buffer = editor.buffer.read(cx);
            let (start_buffer, start) =
                buffer.text_anchor_for_position(rename.range.start.clone(), cx)?;
            let (end_buffer, end) =
                buffer.text_anchor_for_position(rename.range.end.clone(), cx)?;
            if start_buffer == end_buffer {
                let new_name = rename.editor.read(cx).text(cx);
                Some((start_buffer, start..end, rename.old_name, new_name))
            } else {
                None
            }
        })?;

        let rename = workspace.project().clone().update(cx, |project, cx| {
            project.perform_rename(buffer.clone(), range.start, new_name.clone(), true, cx)
        });

        let editor = editor.downgrade();
        Some(cx.spawn(|workspace, mut cx| async move {
            let project_transaction = rename.await?;
            Self::open_project_transaction(
                &editor,
                workspace,
                project_transaction,
                format!("Rename: {} → {}", old_name, new_name),
                cx.clone(),
            )
            .await?;

            editor.update(&mut cx, |editor, cx| {
                editor.refresh_document_highlights(cx);
            })?;
            Ok(())
        }))
    }

    fn take_rename(
        &mut self,
        moving_cursor: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<RenameState> {
        let rename = self.pending_rename.take()?;
        self.remove_blocks(
            [rename.block_id].into_iter().collect(),
            Some(Autoscroll::fit()),
            cx,
        );
        self.clear_text_highlights::<Rename>(cx);
        self.show_local_selections = true;

        if moving_cursor {
            let rename_editor = rename.editor.read(cx);
            let cursor_in_rename_editor = rename_editor.selections.newest::<usize>(cx).head();

            // Update the selection to match the position of the selection inside
            // the rename editor.
            let snapshot = self.buffer.read(cx).read(cx);
            let rename_range = rename.range.to_offset(&snapshot);
            let cursor_in_editor = snapshot
                .clip_offset(rename_range.start + cursor_in_rename_editor, Bias::Left)
                .min(rename_range.end);
            drop(snapshot);

            self.change_selections(None, cx, |s| {
                s.select_ranges(vec![cursor_in_editor..cursor_in_editor])
            });
        } else {
            self.refresh_document_highlights(cx);
        }

        Some(rename)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn pending_rename(&self) -> Option<&RenameState> {
        self.pending_rename.as_ref()
    }

    fn format(&mut self, _: &Format, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let project = match &self.project {
            Some(project) => project.clone(),
            None => return None,
        };

        Some(self.perform_format(project, FormatTrigger::Manual, cx))
    }

    fn perform_format(
        &mut self,
        project: ModelHandle<Project>,
        trigger: FormatTrigger,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self.buffer().clone();
        let buffers = buffer.read(cx).all_buffers();

        let mut timeout = cx.background().timer(FORMAT_TIMEOUT).fuse();
        let format = project.update(cx, |project, cx| project.format(buffers, true, trigger, cx));

        cx.spawn(|_, mut cx| async move {
            let transaction = futures::select_biased! {
                _ = timeout => {
                    log::warn!("timed out waiting for formatting");
                    None
                }
                transaction = format.log_err().fuse() => transaction,
            };

            buffer.update(&mut cx, |buffer, cx| {
                if let Some(transaction) = transaction {
                    if !buffer.is_singleton() {
                        buffer.push_transaction(&transaction.0, cx);
                    }
                }

                cx.notify();
            });

            Ok(())
        })
    }

    fn restart_language_server(&mut self, _: &RestartLanguageServer, cx: &mut ViewContext<Self>) {
        if let Some(project) = self.project.clone() {
            self.buffer.update(cx, |multi_buffer, cx| {
                project.update(cx, |project, cx| {
                    project.restart_language_servers_for_buffers(multi_buffer.all_buffers(), cx);
                });
            })
        }
    }

    fn show_character_palette(&mut self, _: &ShowCharacterPalette, cx: &mut ViewContext<Self>) {
        cx.show_character_palette();
    }

    fn refresh_active_diagnostics(&mut self, cx: &mut ViewContext<Editor>) {
        if let Some(active_diagnostics) = self.active_diagnostics.as_mut() {
            let buffer = self.buffer.read(cx).snapshot(cx);
            let primary_range_start = active_diagnostics.primary_range.start.to_offset(&buffer);
            let is_valid = buffer
                .diagnostics_in_range::<_, usize>(active_diagnostics.primary_range.clone(), false)
                .any(|entry| {
                    entry.diagnostic.is_primary
                        && !entry.range.is_empty()
                        && entry.range.start == primary_range_start
                        && entry.diagnostic.message == active_diagnostics.primary_message
                });

            if is_valid != active_diagnostics.is_valid {
                active_diagnostics.is_valid = is_valid;
                let mut new_styles = HashMap::default();
                for (block_id, diagnostic) in &active_diagnostics.blocks {
                    new_styles.insert(
                        *block_id,
                        diagnostic_block_renderer(diagnostic.clone(), is_valid),
                    );
                }
                self.display_map
                    .update(cx, |display_map, _| display_map.replace_blocks(new_styles));
            }
        }
    }

    fn activate_diagnostics(&mut self, group_id: usize, cx: &mut ViewContext<Self>) -> bool {
        self.dismiss_diagnostics(cx);
        self.active_diagnostics = self.display_map.update(cx, |display_map, cx| {
            let buffer = self.buffer.read(cx).snapshot(cx);

            let mut primary_range = None;
            let mut primary_message = None;
            let mut group_end = Point::zero();
            let diagnostic_group = buffer
                .diagnostic_group::<Point>(group_id)
                .map(|entry| {
                    if entry.range.end > group_end {
                        group_end = entry.range.end;
                    }
                    if entry.diagnostic.is_primary {
                        primary_range = Some(entry.range.clone());
                        primary_message = Some(entry.diagnostic.message.clone());
                    }
                    entry
                })
                .collect::<Vec<_>>();
            let primary_range = primary_range?;
            let primary_message = primary_message?;
            let primary_range =
                buffer.anchor_after(primary_range.start)..buffer.anchor_before(primary_range.end);

            let blocks = display_map
                .insert_blocks(
                    diagnostic_group.iter().map(|entry| {
                        let diagnostic = entry.diagnostic.clone();
                        let message_height = diagnostic.message.lines().count() as u8;
                        BlockProperties {
                            style: BlockStyle::Fixed,
                            position: buffer.anchor_after(entry.range.start),
                            height: message_height,
                            render: diagnostic_block_renderer(diagnostic, true),
                            disposition: BlockDisposition::Below,
                        }
                    }),
                    cx,
                )
                .into_iter()
                .zip(diagnostic_group.into_iter().map(|entry| entry.diagnostic))
                .collect();

            Some(ActiveDiagnosticGroup {
                primary_range,
                primary_message,
                blocks,
                is_valid: true,
            })
        });
        self.active_diagnostics.is_some()
    }

    fn dismiss_diagnostics(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_diagnostic_group) = self.active_diagnostics.take() {
            self.display_map.update(cx, |display_map, cx| {
                display_map.remove_blocks(active_diagnostic_group.blocks.into_keys().collect(), cx);
            });
            cx.notify();
        }
    }

    pub fn set_selections_from_remote(
        &mut self,
        selections: Vec<Selection<Anchor>>,
        pending_selection: Option<Selection<Anchor>>,
        cx: &mut ViewContext<Self>,
    ) {
        let old_cursor_position = self.selections.newest_anchor().head();
        self.selections.change_with(cx, |s| {
            s.select_anchors(selections);
            if let Some(pending_selection) = pending_selection {
                s.set_pending(pending_selection, SelectMode::Character);
            } else {
                s.clear_pending();
            }
        });
        self.selections_did_change(false, &old_cursor_position, cx);
    }

    fn push_to_selection_history(&mut self) {
        self.selection_history.push(SelectionHistoryEntry {
            selections: self.selections.disjoint_anchors(),
            select_next_state: self.select_next_state.clone(),
            select_prev_state: self.select_prev_state.clone(),
            add_selections_state: self.add_selections_state.clone(),
        });
    }

    pub fn transact(
        &mut self,
        cx: &mut ViewContext<Self>,
        update: impl FnOnce(&mut Self, &mut ViewContext<Self>),
    ) -> Option<TransactionId> {
        self.start_transaction_at(Instant::now(), cx);
        update(self, cx);
        self.end_transaction_at(Instant::now(), cx)
    }

    fn start_transaction_at(&mut self, now: Instant, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        if let Some(tx_id) = self
            .buffer
            .update(cx, |buffer, cx| buffer.start_transaction_at(now, cx))
        {
            self.selection_history
                .insert_transaction(tx_id, self.selections.disjoint_anchors());
        }
    }

    fn end_transaction_at(
        &mut self,
        now: Instant,
        cx: &mut ViewContext<Self>,
    ) -> Option<TransactionId> {
        if let Some(tx_id) = self
            .buffer
            .update(cx, |buffer, cx| buffer.end_transaction_at(now, cx))
        {
            if let Some((_, end_selections)) = self.selection_history.transaction_mut(tx_id) {
                *end_selections = Some(self.selections.disjoint_anchors());
            } else {
                error!("unexpectedly ended a transaction that wasn't started by this editor");
            }

            cx.emit(Event::Edited);
            Some(tx_id)
        } else {
            None
        }
    }

    pub fn fold(&mut self, _: &Fold, cx: &mut ViewContext<Self>) {
        let mut fold_ranges = Vec::new();

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        let selections = self.selections.all_adjusted(cx);
        for selection in selections {
            let range = selection.range().sorted();
            let buffer_start_row = range.start.row;

            for row in (0..=range.end.row).rev() {
                let fold_range = display_map.foldable_range(row);

                if let Some(fold_range) = fold_range {
                    if fold_range.end.row >= buffer_start_row {
                        fold_ranges.push(fold_range);
                        if row <= range.start.row {
                            break;
                        }
                    }
                }
            }
        }

        self.fold_ranges(fold_ranges, true, cx);
    }

    pub fn fold_at(&mut self, fold_at: &FoldAt, cx: &mut ViewContext<Self>) {
        let buffer_row = fold_at.buffer_row;
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if let Some(fold_range) = display_map.foldable_range(buffer_row) {
            let autoscroll = self
                .selections
                .all::<Point>(cx)
                .iter()
                .any(|selection| fold_range.overlaps(&selection.range()));

            self.fold_ranges(std::iter::once(fold_range), autoscroll, cx);
        }
    }

    pub fn unfold_lines(&mut self, _: &UnfoldLines, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let selections = self.selections.all::<Point>(cx);
        let ranges = selections
            .iter()
            .map(|s| {
                let range = s.display_range(&display_map).sorted();
                let mut start = range.start.to_point(&display_map);
                let mut end = range.end.to_point(&display_map);
                start.column = 0;
                end.column = buffer.line_len(end.row);
                start..end
            })
            .collect::<Vec<_>>();

        self.unfold_ranges(ranges, true, true, cx);
    }

    pub fn unfold_at(&mut self, unfold_at: &UnfoldAt, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        let intersection_range = Point::new(unfold_at.buffer_row, 0)
            ..Point::new(
                unfold_at.buffer_row,
                display_map.buffer_snapshot.line_len(unfold_at.buffer_row),
            );

        let autoscroll = self
            .selections
            .all::<Point>(cx)
            .iter()
            .any(|selection| selection.range().overlaps(&intersection_range));

        self.unfold_ranges(std::iter::once(intersection_range), true, autoscroll, cx)
    }

    pub fn fold_selected_ranges(&mut self, _: &FoldSelectedRanges, cx: &mut ViewContext<Self>) {
        let selections = self.selections.all::<Point>(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let line_mode = self.selections.line_mode;
        let ranges = selections.into_iter().map(|s| {
            if line_mode {
                let start = Point::new(s.start.row, 0);
                let end = Point::new(s.end.row, display_map.buffer_snapshot.line_len(s.end.row));
                start..end
            } else {
                s.start..s.end
            }
        });
        self.fold_ranges(ranges, true, cx);
    }

    pub fn fold_ranges<T: ToOffset + Clone>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        auto_scroll: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let mut ranges = ranges.into_iter().peekable();
        if ranges.peek().is_some() {
            self.display_map.update(cx, |map, cx| map.fold(ranges, cx));

            if auto_scroll {
                self.request_autoscroll(Autoscroll::fit(), cx);
            }

            cx.notify();
        }
    }

    pub fn unfold_ranges<T: ToOffset + Clone>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        inclusive: bool,
        auto_scroll: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let mut ranges = ranges.into_iter().peekable();
        if ranges.peek().is_some() {
            self.display_map
                .update(cx, |map, cx| map.unfold(ranges, inclusive, cx));
            if auto_scroll {
                self.request_autoscroll(Autoscroll::fit(), cx);
            }

            cx.notify();
        }
    }

    pub fn gutter_hover(
        &mut self,
        GutterHover { hovered }: &GutterHover,
        cx: &mut ViewContext<Self>,
    ) {
        self.gutter_hovered = *hovered;
        cx.notify();
    }

    pub fn insert_blocks(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<Anchor>>,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<BlockId> {
        let blocks = self
            .display_map
            .update(cx, |display_map, cx| display_map.insert_blocks(blocks, cx));
        if let Some(autoscroll) = autoscroll {
            self.request_autoscroll(autoscroll, cx);
        }
        blocks
    }

    pub fn replace_blocks(
        &mut self,
        blocks: HashMap<BlockId, RenderBlock>,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map
            .update(cx, |display_map, _| display_map.replace_blocks(blocks));
        if let Some(autoscroll) = autoscroll {
            self.request_autoscroll(autoscroll, cx);
        }
    }

    pub fn remove_blocks(
        &mut self,
        block_ids: HashSet<BlockId>,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map.update(cx, |display_map, cx| {
            display_map.remove_blocks(block_ids, cx)
        });
        if let Some(autoscroll) = autoscroll {
            self.request_autoscroll(autoscroll, cx);
        }
    }

    pub fn longest_row(&self, cx: &mut AppContext) -> u32 {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .longest_row()
    }

    pub fn max_point(&self, cx: &mut AppContext) -> DisplayPoint {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .max_point()
    }

    pub fn text(&self, cx: &AppContext) -> String {
        self.buffer.read(cx).read(cx).text()
    }

    pub fn set_text(&mut self, text: impl Into<Arc<str>>, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.buffer
                .read(cx)
                .as_singleton()
                .expect("you can only call set_text on editors for singleton buffers")
                .update(cx, |buffer, cx| buffer.set_text(text, cx));
        });
    }

    pub fn display_text(&self, cx: &mut AppContext) -> String {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .text()
    }

    pub fn wrap_guides(&self, cx: &AppContext) -> SmallVec<[(usize, bool); 2]> {
        let mut wrap_guides = smallvec::smallvec![];

        if self.show_wrap_guides == Some(false) {
            return wrap_guides;
        }

        let settings = self.buffer.read(cx).settings_at(0, cx);
        if settings.show_wrap_guides {
            if let SoftWrap::Column(soft_wrap) = self.soft_wrap_mode(cx) {
                wrap_guides.push((soft_wrap as usize, true));
            }
            wrap_guides.extend(settings.wrap_guides.iter().map(|guide| (*guide, false)))
        }

        wrap_guides
    }

    pub fn soft_wrap_mode(&self, cx: &AppContext) -> SoftWrap {
        let settings = self.buffer.read(cx).settings_at(0, cx);
        let mode = self
            .soft_wrap_mode_override
            .unwrap_or_else(|| settings.soft_wrap);
        match mode {
            language_settings::SoftWrap::None => SoftWrap::None,
            language_settings::SoftWrap::EditorWidth => SoftWrap::EditorWidth,
            language_settings::SoftWrap::PreferredLineLength => {
                SoftWrap::Column(settings.preferred_line_length)
            }
        }
    }

    pub fn set_soft_wrap_mode(
        &mut self,
        mode: language_settings::SoftWrap,
        cx: &mut ViewContext<Self>,
    ) {
        self.soft_wrap_mode_override = Some(mode);
        cx.notify();
    }

    pub fn set_wrap_width(&self, width: Option<f32>, cx: &mut AppContext) -> bool {
        self.display_map
            .update(cx, |map, cx| map.set_wrap_width(width, cx))
    }

    pub fn toggle_soft_wrap(&mut self, _: &ToggleSoftWrap, cx: &mut ViewContext<Self>) {
        if self.soft_wrap_mode_override.is_some() {
            self.soft_wrap_mode_override.take();
        } else {
            let soft_wrap = match self.soft_wrap_mode(cx) {
                SoftWrap::None => language_settings::SoftWrap::EditorWidth,
                SoftWrap::EditorWidth | SoftWrap::Column(_) => language_settings::SoftWrap::None,
            };
            self.soft_wrap_mode_override = Some(soft_wrap);
        }
        cx.notify();
    }

    pub fn set_show_gutter(&mut self, show_gutter: bool, cx: &mut ViewContext<Self>) {
        self.show_gutter = show_gutter;
        cx.notify();
    }

    pub fn set_show_wrap_guides(&mut self, show_gutter: bool, cx: &mut ViewContext<Self>) {
        self.show_wrap_guides = Some(show_gutter);
        cx.notify();
    }

    pub fn reveal_in_finder(&mut self, _: &RevealInFinder, cx: &mut ViewContext<Self>) {
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            if let Some(file) = buffer.read(cx).file().and_then(|f| f.as_local()) {
                cx.reveal_path(&file.abs_path(cx));
            }
        }
    }

    pub fn copy_path(&mut self, _: &CopyPath, cx: &mut ViewContext<Self>) {
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            if let Some(file) = buffer.read(cx).file().and_then(|f| f.as_local()) {
                if let Some(path) = file.abs_path(cx).to_str() {
                    cx.write_to_clipboard(ClipboardItem::new(path.to_string()));
                }
            }
        }
    }

    pub fn copy_relative_path(&mut self, _: &CopyRelativePath, cx: &mut ViewContext<Self>) {
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            if let Some(file) = buffer.read(cx).file().and_then(|f| f.as_local()) {
                if let Some(path) = file.path().to_str() {
                    cx.write_to_clipboard(ClipboardItem::new(path.to_string()));
                }
            }
        }
    }

    pub fn highlight_rows(&mut self, rows: Option<Range<u32>>) {
        self.highlighted_rows = rows;
    }

    pub fn highlighted_rows(&self) -> Option<Range<u32>> {
        self.highlighted_rows.clone()
    }

    pub fn highlight_background<T: 'static>(
        &mut self,
        ranges: Vec<Range<Anchor>>,
        color_fetcher: fn(&Theme) -> Color,
        cx: &mut ViewContext<Self>,
    ) {
        self.background_highlights.insert(
            TypeId::of::<T>(),
            (
                color_fetcher,
                ranges.into_iter().map(DocumentRange::Text).collect(),
            ),
        );
        cx.notify();
    }

    pub fn highlight_inlay_background<T: 'static>(
        &mut self,
        ranges: Vec<InlayRange>,
        color_fetcher: fn(&Theme) -> Color,
        cx: &mut ViewContext<Self>,
    ) {
        self.background_highlights.insert(
            TypeId::of::<T>(),
            (
                color_fetcher,
                ranges.into_iter().map(DocumentRange::Inlay).collect(),
            ),
        );
        cx.notify();
    }

    pub fn clear_background_highlights<T: 'static>(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<BackgroundHighlight> {
        let highlights = self.background_highlights.remove(&TypeId::of::<T>());
        if highlights.is_some() {
            cx.notify();
        }
        highlights
    }

    #[cfg(feature = "test-support")]
    pub fn all_background_highlights(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Vec<(Range<DisplayPoint>, Color)> {
        let snapshot = self.snapshot(cx);
        let buffer = &snapshot.buffer_snapshot;
        let start = buffer.anchor_before(0);
        let end = buffer.anchor_after(buffer.len());
        let theme = theme::current(cx);
        self.background_highlights_in_range(start..end, &snapshot, theme.as_ref())
    }

    fn document_highlights_for_position<'a>(
        &'a self,
        position: Anchor,
        buffer: &'a MultiBufferSnapshot,
        display_snapshot: &'a DisplaySnapshot,
    ) -> impl 'a + Iterator<Item = &DocumentRange> {
        let read_highlights = self
            .background_highlights
            .get(&TypeId::of::<DocumentHighlightRead>())
            .map(|h| &h.1);
        let write_highlights = self
            .background_highlights
            .get(&TypeId::of::<DocumentHighlightWrite>())
            .map(|h| &h.1);
        let left_position = display_snapshot.anchor_to_inlay_offset(position.bias_left(buffer));
        let right_position = display_snapshot.anchor_to_inlay_offset(position.bias_right(buffer));
        read_highlights
            .into_iter()
            .chain(write_highlights)
            .flat_map(move |ranges| {
                let start_ix = match ranges.binary_search_by(|probe| {
                    let cmp = document_to_inlay_range(probe, display_snapshot)
                        .end
                        .cmp(&left_position);
                    if cmp.is_ge() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }) {
                    Ok(i) | Err(i) => i,
                };

                let right_position = right_position.clone();
                ranges[start_ix..].iter().take_while(move |range| {
                    document_to_inlay_range(range, display_snapshot)
                        .start
                        .cmp(&right_position)
                        .is_le()
                })
            })
    }

    pub fn background_highlights_in_range(
        &self,
        search_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
        theme: &Theme,
    ) -> Vec<(Range<DisplayPoint>, Color)> {
        let search_range = display_snapshot.anchor_to_inlay_offset(search_range.start)
            ..display_snapshot.anchor_to_inlay_offset(search_range.end);
        let mut results = Vec::new();
        for (color_fetcher, ranges) in self.background_highlights.values() {
            let color = color_fetcher(theme);
            let start_ix = match ranges.binary_search_by(|probe| {
                let cmp = document_to_inlay_range(probe, display_snapshot)
                    .end
                    .cmp(&search_range.start);
                if cmp.is_gt() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }) {
                Ok(i) | Err(i) => i,
            };
            for range in &ranges[start_ix..] {
                let range = document_to_inlay_range(range, display_snapshot);
                if range.start.cmp(&search_range.end).is_ge() {
                    break;
                }

                let start = display_snapshot.inlay_offset_to_display_point(range.start, Bias::Left);
                let end = display_snapshot.inlay_offset_to_display_point(range.end, Bias::Right);
                results.push((start..end, color))
            }
        }
        results
    }

    pub fn background_highlight_row_ranges<T: 'static>(
        &self,
        search_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
        count: usize,
    ) -> Vec<RangeInclusive<DisplayPoint>> {
        let search_range = display_snapshot.anchor_to_inlay_offset(search_range.start)
            ..display_snapshot.anchor_to_inlay_offset(search_range.end);
        let mut results = Vec::new();
        let Some((_, ranges)) = self.background_highlights.get(&TypeId::of::<T>()) else {
            return vec![];
        };

        let start_ix = match ranges.binary_search_by(|probe| {
            let cmp = document_to_inlay_range(probe, display_snapshot)
                .end
                .cmp(&search_range.start);
            if cmp.is_gt() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }) {
            Ok(i) | Err(i) => i,
        };
        let mut push_region = |start: Option<Point>, end: Option<Point>| {
            if let (Some(start_display), Some(end_display)) = (start, end) {
                results.push(
                    start_display.to_display_point(display_snapshot)
                        ..=end_display.to_display_point(display_snapshot),
                );
            }
        };
        let mut start_row: Option<Point> = None;
        let mut end_row: Option<Point> = None;
        if ranges.len() > count {
            return Vec::new();
        }
        for range in &ranges[start_ix..] {
            let range = document_to_inlay_range(range, display_snapshot);
            if range.start.cmp(&search_range.end).is_ge() {
                break;
            }
            let end = display_snapshot
                .inlay_offset_to_display_point(range.end, Bias::Right)
                .to_point(display_snapshot);
            if let Some(current_row) = &end_row {
                if end.row == current_row.row {
                    continue;
                }
            }
            let start = display_snapshot
                .inlay_offset_to_display_point(range.start, Bias::Left)
                .to_point(display_snapshot);

            if start_row.is_none() {
                assert_eq!(end_row, None);
                start_row = Some(start);
                end_row = Some(end);
                continue;
            }
            if let Some(current_end) = end_row.as_mut() {
                if start.row > current_end.row + 1 {
                    push_region(start_row, end_row);
                    start_row = Some(start);
                    end_row = Some(end);
                } else {
                    // Merge two hunks.
                    *current_end = end;
                }
            } else {
                unreachable!();
            }
        }
        // We might still have a hunk that was not rendered (if there was a search hit on the last line)
        push_region(start_row, end_row);
        results
    }

    pub fn highlight_text<T: 'static>(
        &mut self,
        ranges: Vec<Range<Anchor>>,
        style: HighlightStyle,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map.update(cx, |map, _| {
            map.highlight_text(TypeId::of::<T>(), ranges, style)
        });
        cx.notify();
    }

    pub fn highlight_inlays<T: 'static>(
        &mut self,
        ranges: Vec<InlayRange>,
        style: HighlightStyle,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map.update(cx, |map, _| {
            map.highlight_inlays(TypeId::of::<T>(), ranges, style)
        });
        cx.notify();
    }

    pub fn text_highlights<'a, T: 'static>(
        &'a self,
        cx: &'a AppContext,
    ) -> Option<(HighlightStyle, &'a [DocumentRange])> {
        self.display_map.read(cx).text_highlights(TypeId::of::<T>())
    }

    pub fn clear_text_highlights<T: 'static>(&mut self, cx: &mut ViewContext<Self>) {
        let text_highlights = self
            .display_map
            .update(cx, |map, _| map.clear_text_highlights(TypeId::of::<T>()));
        if text_highlights.is_some() {
            cx.notify();
        }
    }

    pub fn show_local_cursors(&self, cx: &AppContext) -> bool {
        self.blink_manager.read(cx).visible() && self.focused
    }

    fn on_buffer_changed(&mut self, _: ModelHandle<MultiBuffer>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn on_buffer_event(
        &mut self,
        multibuffer: ModelHandle<MultiBuffer>,
        event: &multi_buffer::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            multi_buffer::Event::Edited {
                sigleton_buffer_edited,
            } => {
                self.refresh_active_diagnostics(cx);
                self.refresh_code_actions(cx);
                if self.has_active_copilot_suggestion(cx) {
                    self.update_visible_copilot_suggestion(cx);
                }
                cx.emit(Event::BufferEdited);

                if *sigleton_buffer_edited {
                    if let Some(project) = &self.project {
                        let project = project.read(cx);
                        let languages_affected = multibuffer
                            .read(cx)
                            .all_buffers()
                            .into_iter()
                            .filter_map(|buffer| {
                                let buffer = buffer.read(cx);
                                let language = buffer.language()?;
                                if project.is_local()
                                    && project.language_servers_for_buffer(buffer, cx).count() == 0
                                {
                                    None
                                } else {
                                    Some(language)
                                }
                            })
                            .cloned()
                            .collect::<HashSet<_>>();
                        if !languages_affected.is_empty() {
                            self.refresh_inlay_hints(
                                InlayHintRefreshReason::BufferEdited(languages_affected),
                                cx,
                            );
                        }
                    }
                }
            }
            multi_buffer::Event::ExcerptsAdded {
                buffer,
                predecessor,
                excerpts,
            } => {
                cx.emit(Event::ExcerptsAdded {
                    buffer: buffer.clone(),
                    predecessor: *predecessor,
                    excerpts: excerpts.clone(),
                });
                self.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            }
            multi_buffer::Event::ExcerptsRemoved { ids } => {
                self.refresh_inlay_hints(InlayHintRefreshReason::ExcerptsRemoved(ids.clone()), cx);
                cx.emit(Event::ExcerptsRemoved { ids: ids.clone() })
            }
            multi_buffer::Event::Reparsed => cx.emit(Event::Reparsed),
            multi_buffer::Event::DirtyChanged => cx.emit(Event::DirtyChanged),
            multi_buffer::Event::Saved => cx.emit(Event::Saved),
            multi_buffer::Event::FileHandleChanged => cx.emit(Event::TitleChanged),
            multi_buffer::Event::Reloaded => cx.emit(Event::TitleChanged),
            multi_buffer::Event::DiffBaseChanged => cx.emit(Event::DiffBaseChanged),
            multi_buffer::Event::Closed => cx.emit(Event::Closed),
            multi_buffer::Event::DiagnosticsUpdated => {
                self.refresh_active_diagnostics(cx);
            }
            _ => {}
        };
    }

    fn on_display_map_changed(&mut self, _: ModelHandle<DisplayMap>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn settings_changed(&mut self, cx: &mut ViewContext<Self>) {
        self.refresh_copilot_suggestions(true, cx);
        self.refresh_inlay_hints(
            InlayHintRefreshReason::SettingsChange(inlay_hint_settings(
                self.selections.newest_anchor().head(),
                &self.buffer.read(cx).snapshot(cx),
                cx,
            )),
            cx,
        );
    }

    pub fn set_searchable(&mut self, searchable: bool) {
        self.searchable = searchable;
    }

    pub fn searchable(&self) -> bool {
        self.searchable
    }

    fn open_excerpts(workspace: &mut Workspace, _: &OpenExcerpts, cx: &mut ViewContext<Workspace>) {
        let active_item = workspace.active_item(cx);
        let editor_handle = if let Some(editor) = active_item
            .as_ref()
            .and_then(|item| item.act_as::<Self>(cx))
        {
            editor
        } else {
            cx.propagate_action();
            return;
        };

        let editor = editor_handle.read(cx);
        let buffer = editor.buffer.read(cx);
        if buffer.is_singleton() {
            cx.propagate_action();
            return;
        }

        let mut new_selections_by_buffer = HashMap::default();
        for selection in editor.selections.all::<usize>(cx) {
            for (buffer, mut range, _) in
                buffer.range_to_buffer_ranges(selection.start..selection.end, cx)
            {
                if selection.reversed {
                    mem::swap(&mut range.start, &mut range.end);
                }
                new_selections_by_buffer
                    .entry(buffer)
                    .or_insert(Vec::new())
                    .push(range)
            }
        }

        editor_handle.update(cx, |editor, cx| {
            editor.push_to_nav_history(editor.selections.newest_anchor().head(), None, cx);
        });
        let pane = workspace.active_pane().clone();
        pane.update(cx, |pane, _| pane.disable_history());

        // We defer the pane interaction because we ourselves are a workspace item
        // and activating a new item causes the pane to call a method on us reentrantly,
        // which panics if we're on the stack.
        cx.defer(move |workspace, cx| {
            for (buffer, ranges) in new_selections_by_buffer.into_iter() {
                let editor = workspace.open_project_item::<Self>(buffer, cx);
                editor.update(cx, |editor, cx| {
                    editor.change_selections(Some(Autoscroll::newest()), cx, |s| {
                        s.select_ranges(ranges);
                    });
                });
            }

            pane.update(cx, |pane, _| pane.enable_history());
        });
    }

    fn jump(
        workspace: &mut Workspace,
        path: ProjectPath,
        position: Point,
        anchor: language::Anchor,
        cx: &mut ViewContext<Workspace>,
    ) {
        let editor = workspace.open_path(path, None, true, cx);
        cx.spawn(|_, mut cx| async move {
            let editor = editor
                .await?
                .downcast::<Editor>()
                .ok_or_else(|| anyhow!("opened item was not an editor"))?
                .downgrade();
            editor.update(&mut cx, |editor, cx| {
                let buffer = editor
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .ok_or_else(|| anyhow!("cannot jump in a multi-buffer"))?;
                let buffer = buffer.read(cx);
                let cursor = if buffer.can_resolve(&anchor) {
                    language::ToPoint::to_point(&anchor, buffer)
                } else {
                    buffer.clip_point(position, Bias::Left)
                };

                let nav_history = editor.nav_history.take();
                editor.change_selections(Some(Autoscroll::newest()), cx, |s| {
                    s.select_ranges([cursor..cursor]);
                });
                editor.nav_history = nav_history;

                anyhow::Ok(())
            })??;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn marked_text_ranges(&self, cx: &AppContext) -> Option<Vec<Range<OffsetUtf16>>> {
        let snapshot = self.buffer.read(cx).read(cx);
        let (_, ranges) = self.text_highlights::<InputComposition>(cx)?;
        Some(
            ranges
                .iter()
                .filter_map(|range| range.as_text_range())
                .map(move |range| {
                    range.start.to_offset_utf16(&snapshot)..range.end.to_offset_utf16(&snapshot)
                })
                .collect(),
        )
    }

    fn selection_replacement_ranges(
        &self,
        range: Range<OffsetUtf16>,
        cx: &AppContext,
    ) -> Vec<Range<OffsetUtf16>> {
        let selections = self.selections.all::<OffsetUtf16>(cx);
        let newest_selection = selections
            .iter()
            .max_by_key(|selection| selection.id)
            .unwrap();
        let start_delta = range.start.0 as isize - newest_selection.start.0 as isize;
        let end_delta = range.end.0 as isize - newest_selection.end.0 as isize;
        let snapshot = self.buffer.read(cx).read(cx);
        selections
            .into_iter()
            .map(|mut selection| {
                selection.start.0 =
                    (selection.start.0 as isize).saturating_add(start_delta) as usize;
                selection.end.0 = (selection.end.0 as isize).saturating_add(end_delta) as usize;
                snapshot.clip_offset_utf16(selection.start, Bias::Left)
                    ..snapshot.clip_offset_utf16(selection.end, Bias::Right)
            })
            .collect()
    }

    fn report_copilot_event(
        &self,
        suggestion_id: Option<String>,
        suggestion_accepted: bool,
        cx: &AppContext,
    ) {
        let Some(project) = &self.project else { return };

        // If None, we are either getting suggestions in a new, unsaved file, or in a file without an extension
        let file_extension = self
            .buffer
            .read(cx)
            .as_singleton()
            .and_then(|b| b.read(cx).file())
            .and_then(|file| Path::new(file.file_name(cx)).extension())
            .and_then(|e| e.to_str())
            .map(|a| a.to_string());

        let telemetry = project.read(cx).client().telemetry().clone();
        let telemetry_settings = *settings::get::<TelemetrySettings>(cx);

        let event = ClickhouseEvent::Copilot {
            suggestion_id,
            suggestion_accepted,
            file_extension,
        };
        telemetry.report_clickhouse_event(event, telemetry_settings);
    }

    fn report_editor_event(
        &self,
        operation: &'static str,
        file_extension: Option<String>,
        cx: &AppContext,
    ) {
        let Some(project) = &self.project else { return };

        // If None, we are in a file without an extension
        let file = self
            .buffer
            .read(cx)
            .as_singleton()
            .and_then(|b| b.read(cx).file());
        let file_extension = file_extension.or(file
            .as_ref()
            .and_then(|file| Path::new(file.file_name(cx)).extension())
            .and_then(|e| e.to_str())
            .map(|a| a.to_string()));

        let vim_mode = cx
            .global::<SettingsStore>()
            .raw_user_settings()
            .get("vim_mode")
            == Some(&serde_json::Value::Bool(true));
        let telemetry_settings = *settings::get::<TelemetrySettings>(cx);
        let copilot_enabled = all_language_settings(file, cx).copilot_enabled(None, None);
        let copilot_enabled_for_language = self
            .buffer
            .read(cx)
            .settings_at(0, cx)
            .show_copilot_suggestions;

        let telemetry = project.read(cx).client().telemetry().clone();
        let event = ClickhouseEvent::Editor {
            file_extension,
            vim_mode,
            operation,
            copilot_enabled,
            copilot_enabled_for_language,
        };
        telemetry.report_clickhouse_event(event, telemetry_settings)
    }

    /// Copy the highlighted chunks to the clipboard as JSON. The format is an array of lines,
    /// with each line being an array of {text, highlight} objects.
    fn copy_highlight_json(&mut self, _: &CopyHighlightJson, cx: &mut ViewContext<Self>) {
        let Some(buffer) = self.buffer.read(cx).as_singleton() else {
            return;
        };

        #[derive(Serialize)]
        struct Chunk<'a> {
            text: String,
            highlight: Option<&'a str>,
        }

        let snapshot = buffer.read(cx).snapshot();
        let range = self
            .selected_text_range(cx)
            .and_then(|selected_range| {
                if selected_range.is_empty() {
                    None
                } else {
                    Some(selected_range)
                }
            })
            .unwrap_or_else(|| 0..snapshot.len());

        let chunks = snapshot.chunks(range, true);
        let mut lines = Vec::new();
        let mut line: VecDeque<Chunk> = VecDeque::new();

        let theme = &theme::current(cx).editor.syntax;

        for chunk in chunks {
            let highlight = chunk.syntax_highlight_id.and_then(|id| id.name(theme));
            let mut chunk_lines = chunk.text.split("\n").peekable();
            while let Some(text) = chunk_lines.next() {
                let mut merged_with_last_token = false;
                if let Some(last_token) = line.back_mut() {
                    if last_token.highlight == highlight {
                        last_token.text.push_str(text);
                        merged_with_last_token = true;
                    }
                }

                if !merged_with_last_token {
                    line.push_back(Chunk {
                        text: text.into(),
                        highlight,
                    });
                }

                if chunk_lines.peek().is_some() {
                    if line.len() > 1 && line.front().unwrap().text.is_empty() {
                        line.pop_front();
                    }
                    if line.len() > 1 && line.back().unwrap().text.is_empty() {
                        line.pop_back();
                    }

                    lines.push(mem::take(&mut line));
                }
            }
        }

        let Some(lines) = serde_json::to_string_pretty(&lines).log_err() else {
            return;
        };
        cx.write_to_clipboard(ClipboardItem::new(lines));
    }

    pub fn inlay_hint_cache(&self) -> &InlayHintCache {
        &self.inlay_hint_cache
    }

    pub fn replay_insert_event(
        &mut self,
        text: &str,
        relative_utf16_range: Option<Range<isize>>,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.input_enabled {
            cx.emit(Event::InputIgnored { text: text.into() });
            return;
        }
        if let Some(relative_utf16_range) = relative_utf16_range {
            let selections = self.selections.all::<OffsetUtf16>(cx);
            self.change_selections(None, cx, |s| {
                let new_ranges = selections.into_iter().map(|range| {
                    let start = OffsetUtf16(
                        range
                            .head()
                            .0
                            .saturating_add_signed(relative_utf16_range.start),
                    );
                    let end = OffsetUtf16(
                        range
                            .head()
                            .0
                            .saturating_add_signed(relative_utf16_range.end),
                    );
                    start..end
                });
                s.select_ranges(new_ranges);
            });
        }

        self.handle_input(text, cx);
    }
}

fn document_to_inlay_range(
    range: &DocumentRange,
    snapshot: &DisplaySnapshot,
) -> Range<InlayOffset> {
    match range {
        DocumentRange::Text(text_range) => {
            snapshot.anchor_to_inlay_offset(text_range.start)
                ..snapshot.anchor_to_inlay_offset(text_range.end)
        }
        DocumentRange::Inlay(inlay_range) => inlay_range.highlight_start..inlay_range.highlight_end,
    }
}

fn inlay_hint_settings(
    location: Anchor,
    snapshot: &MultiBufferSnapshot,
    cx: &mut ViewContext<'_, '_, Editor>,
) -> InlayHintSettings {
    let file = snapshot.file_at(location);
    let language = snapshot.language_at(location);
    let settings = all_language_settings(file, cx);
    settings
        .language(language.map(|l| l.name()).as_deref())
        .inlay_hints
}

fn consume_contiguous_rows(
    contiguous_row_selections: &mut Vec<Selection<Point>>,
    selection: &Selection<Point>,
    display_map: &DisplaySnapshot,
    selections: &mut std::iter::Peekable<std::slice::Iter<Selection<Point>>>,
) -> (u32, u32) {
    contiguous_row_selections.push(selection.clone());
    let start_row = selection.start.row;
    let mut end_row = ending_row(selection, display_map);

    while let Some(next_selection) = selections.peek() {
        if next_selection.start.row <= end_row {
            end_row = ending_row(next_selection, display_map);
            contiguous_row_selections.push(selections.next().unwrap().clone());
        } else {
            break;
        }
    }
    (start_row, end_row)
}

fn ending_row(next_selection: &Selection<Point>, display_map: &DisplaySnapshot) -> u32 {
    if next_selection.end.column > 0 || next_selection.is_empty() {
        display_map.next_line_boundary(next_selection.end).0.row + 1
    } else {
        next_selection.end.row
    }
}

impl EditorSnapshot {
    pub fn language_at<T: ToOffset>(&self, position: T) -> Option<&Arc<Language>> {
        self.display_snapshot.buffer_snapshot.language_at(position)
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn placeholder_text(&self) -> Option<&Arc<str>> {
        self.placeholder_text.as_ref()
    }

    pub fn scroll_position(&self) -> Vector2F {
        self.scroll_anchor.scroll_position(&self.display_snapshot)
    }
}

impl Deref for EditorSnapshot {
    type Target = DisplaySnapshot;

    fn deref(&self) -> &Self::Target {
        &self.display_snapshot
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    InputIgnored {
        text: Arc<str>,
    },
    InputHandled {
        utf16_range_to_replace: Option<Range<isize>>,
        text: Arc<str>,
    },
    ExcerptsAdded {
        buffer: ModelHandle<Buffer>,
        predecessor: ExcerptId,
        excerpts: Vec<(ExcerptId, ExcerptRange<language::Anchor>)>,
    },
    ExcerptsRemoved {
        ids: Vec<ExcerptId>,
    },
    BufferEdited,
    Edited,
    Reparsed,
    Focused,
    Blurred,
    DirtyChanged,
    Saved,
    TitleChanged,
    DiffBaseChanged,
    SelectionsChanged {
        local: bool,
    },
    ScrollPositionChanged {
        local: bool,
        autoscroll: bool,
    },
    TransactionUndone {
        transaction_id: TransactionId,
    },
    Closed,
}

pub struct EditorFocused(pub ViewHandle<Editor>);
pub struct EditorBlurred(pub ViewHandle<Editor>);
pub struct EditorReleased(pub WeakViewHandle<Editor>);

impl Entity for Editor {
    type Event = Event;

    fn release(&mut self, cx: &mut AppContext) {
        cx.emit_global(EditorReleased(self.handle.clone()));
    }
}

impl View for Editor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let style = self.style(cx);
        let font_changed = self.display_map.update(cx, |map, cx| {
            map.set_fold_ellipses_color(style.folds.ellipses.text_color);
            map.set_font(style.text.font_id, style.text.font_size, cx)
        });

        if font_changed {
            cx.defer(move |editor, cx: &mut ViewContext<Editor>| {
                hide_hover(editor, cx);
                hide_link_definition(editor, cx);
            });
        }

        Stack::new()
            .with_child(EditorElement::new(style.clone()))
            .with_child(ChildView::new(&self.mouse_context_menu, cx))
            .into_any()
    }

    fn ui_name() -> &'static str {
        "Editor"
    }

    fn focus_in(&mut self, focused: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            let focused_event = EditorFocused(cx.handle());
            cx.emit(Event::Focused);
            cx.emit_global(focused_event);
        }
        if let Some(rename) = self.pending_rename.as_ref() {
            cx.focus(&rename.editor);
        } else if cx.is_self_focused() || !focused.is::<Editor>() {
            if !self.focused {
                self.blink_manager.update(cx, BlinkManager::enable);
            }
            self.focused = true;
            self.buffer.update(cx, |buffer, cx| {
                buffer.finalize_last_transaction(cx);
                if self.leader_replica_id.is_none() {
                    buffer.set_active_selections(
                        &self.selections.disjoint_anchors(),
                        self.selections.line_mode,
                        self.cursor_shape,
                        cx,
                    );
                }
            });
        }
    }

    fn focus_out(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        let blurred_event = EditorBlurred(cx.handle());
        cx.emit_global(blurred_event);
        self.focused = false;
        self.blink_manager.update(cx, BlinkManager::disable);
        self.buffer
            .update(cx, |buffer, cx| buffer.remove_active_selections(cx));
        self.hide_context_menu(cx);
        hide_hover(self, cx);
        cx.emit(Event::Blurred);
        cx.notify();
    }

    fn modifiers_changed(
        &mut self,
        event: &gpui::platform::ModifiersChangedEvent,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        let pending_selection = self.has_pending_selection();

        if let Some(point) = &self.link_go_to_definition_state.last_trigger_point {
            if event.cmd && !pending_selection {
                let point = point.clone();
                let snapshot = self.snapshot(cx);
                let kind = point.definition_kind(event.shift);

                show_link_definition(kind, self, point, snapshot, cx);
                return false;
            }
        }

        {
            if self.link_go_to_definition_state.symbol_range.is_some()
                || !self.link_go_to_definition_state.definitions.is_empty()
            {
                self.link_go_to_definition_state.symbol_range.take();
                self.link_go_to_definition_state.definitions.clear();
                cx.notify();
            }

            self.link_go_to_definition_state.task = None;

            self.clear_text_highlights::<LinkGoToDefinitionState>(cx);
        }

        false
    }

    fn update_keymap_context(&self, keymap: &mut KeymapContext, cx: &AppContext) {
        Self::reset_to_default_keymap_context(keymap);
        let mode = match self.mode {
            EditorMode::SingleLine => "single_line",
            EditorMode::AutoHeight { .. } => "auto_height",
            EditorMode::Full => "full",
        };
        keymap.add_key("mode", mode);
        if self.pending_rename.is_some() {
            keymap.add_identifier("renaming");
        }
        if self.context_menu_visible() {
            match self.context_menu.as_ref() {
                Some(ContextMenu::Completions(_)) => {
                    keymap.add_identifier("menu");
                    keymap.add_identifier("showing_completions")
                }
                Some(ContextMenu::CodeActions(_)) => {
                    keymap.add_identifier("menu");
                    keymap.add_identifier("showing_code_actions")
                }
                None => {}
            }
        }

        for layer in self.keymap_context_layers.values() {
            keymap.extend(layer);
        }

        if let Some(extension) = self
            .buffer
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file()?.path().extension()?.to_str())
        {
            keymap.add_key("extension", extension.to_string());
        }
    }

    fn text_for_range(&self, range_utf16: Range<usize>, cx: &AppContext) -> Option<String> {
        Some(
            self.buffer
                .read(cx)
                .read(cx)
                .text_for_range(OffsetUtf16(range_utf16.start)..OffsetUtf16(range_utf16.end))
                .collect(),
        )
    }

    fn selected_text_range(&self, cx: &AppContext) -> Option<Range<usize>> {
        // Prevent the IME menu from appearing when holding down an alphabetic key
        // while input is disabled.
        if !self.input_enabled {
            return None;
        }

        let range = self.selections.newest::<OffsetUtf16>(cx).range();
        Some(range.start.0..range.end.0)
    }

    fn marked_text_range(&self, cx: &AppContext) -> Option<Range<usize>> {
        let snapshot = self.buffer.read(cx).read(cx);
        let range = self.text_highlights::<InputComposition>(cx)?.1.get(0)?;
        let range = range.as_text_range()?;
        Some(range.start.to_offset_utf16(&snapshot).0..range.end.to_offset_utf16(&snapshot).0)
    }

    fn unmark_text(&mut self, cx: &mut ViewContext<Self>) {
        self.clear_text_highlights::<InputComposition>(cx);
        self.ime_transaction.take();
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.input_enabled {
            cx.emit(Event::InputIgnored { text: text.into() });
            return;
        }

        self.transact(cx, |this, cx| {
            let new_selected_ranges = if let Some(range_utf16) = range_utf16 {
                let range_utf16 = OffsetUtf16(range_utf16.start)..OffsetUtf16(range_utf16.end);
                Some(this.selection_replacement_ranges(range_utf16, cx))
            } else {
                this.marked_text_ranges(cx)
            };

            let range_to_replace = new_selected_ranges.as_ref().and_then(|ranges_to_replace| {
                let newest_selection_id = this.selections.newest_anchor().id;
                this.selections
                    .all::<OffsetUtf16>(cx)
                    .iter()
                    .zip(ranges_to_replace.iter())
                    .find_map(|(selection, range)| {
                        if selection.id == newest_selection_id {
                            Some(
                                (range.start.0 as isize - selection.head().0 as isize)
                                    ..(range.end.0 as isize - selection.head().0 as isize),
                            )
                        } else {
                            None
                        }
                    })
            });

            cx.emit(Event::InputHandled {
                utf16_range_to_replace: range_to_replace,
                text: text.into(),
            });

            if let Some(new_selected_ranges) = new_selected_ranges {
                this.change_selections(None, cx, |selections| {
                    selections.select_ranges(new_selected_ranges)
                });
            }

            this.handle_input(text, cx);
        });

        if let Some(transaction) = self.ime_transaction {
            self.buffer.update(cx, |buffer, cx| {
                buffer.group_until_transaction(transaction, cx);
            });
        }

        self.unmark_text(cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.input_enabled {
            cx.emit(Event::InputIgnored { text: text.into() });
            return;
        }

        let transaction = self.transact(cx, |this, cx| {
            let ranges_to_replace = if let Some(mut marked_ranges) = this.marked_text_ranges(cx) {
                let snapshot = this.buffer.read(cx).read(cx);
                if let Some(relative_range_utf16) = range_utf16.as_ref() {
                    for marked_range in &mut marked_ranges {
                        marked_range.end.0 = marked_range.start.0 + relative_range_utf16.end;
                        marked_range.start.0 += relative_range_utf16.start;
                        marked_range.start =
                            snapshot.clip_offset_utf16(marked_range.start, Bias::Left);
                        marked_range.end =
                            snapshot.clip_offset_utf16(marked_range.end, Bias::Right);
                    }
                }
                Some(marked_ranges)
            } else if let Some(range_utf16) = range_utf16 {
                let range_utf16 = OffsetUtf16(range_utf16.start)..OffsetUtf16(range_utf16.end);
                Some(this.selection_replacement_ranges(range_utf16, cx))
            } else {
                None
            };

            let range_to_replace = ranges_to_replace.as_ref().and_then(|ranges_to_replace| {
                let newest_selection_id = this.selections.newest_anchor().id;
                this.selections
                    .all::<OffsetUtf16>(cx)
                    .iter()
                    .zip(ranges_to_replace.iter())
                    .find_map(|(selection, range)| {
                        if selection.id == newest_selection_id {
                            Some(
                                (range.start.0 as isize - selection.head().0 as isize)
                                    ..(range.end.0 as isize - selection.head().0 as isize),
                            )
                        } else {
                            None
                        }
                    })
            });

            cx.emit(Event::InputHandled {
                utf16_range_to_replace: range_to_replace,
                text: text.into(),
            });

            if let Some(ranges) = ranges_to_replace {
                this.change_selections(None, cx, |s| s.select_ranges(ranges));
            }

            let marked_ranges = {
                let snapshot = this.buffer.read(cx).read(cx);
                this.selections
                    .disjoint_anchors()
                    .iter()
                    .map(|selection| {
                        selection.start.bias_left(&*snapshot)..selection.end.bias_right(&*snapshot)
                    })
                    .collect::<Vec<_>>()
            };

            if text.is_empty() {
                this.unmark_text(cx);
            } else {
                this.highlight_text::<InputComposition>(
                    marked_ranges.clone(),
                    this.style(cx).composition_mark,
                    cx,
                );
            }

            this.handle_input(text, cx);

            if let Some(new_selected_range) = new_selected_range_utf16 {
                let snapshot = this.buffer.read(cx).read(cx);
                let new_selected_ranges = marked_ranges
                    .into_iter()
                    .map(|marked_range| {
                        let insertion_start = marked_range.start.to_offset_utf16(&snapshot).0;
                        let new_start = OffsetUtf16(new_selected_range.start + insertion_start);
                        let new_end = OffsetUtf16(new_selected_range.end + insertion_start);
                        snapshot.clip_offset_utf16(new_start, Bias::Left)
                            ..snapshot.clip_offset_utf16(new_end, Bias::Right)
                    })
                    .collect::<Vec<_>>();

                drop(snapshot);
                this.change_selections(None, cx, |selections| {
                    selections.select_ranges(new_selected_ranges)
                });
            }
        });

        self.ime_transaction = self.ime_transaction.or(transaction);
        if let Some(transaction) = self.ime_transaction {
            self.buffer.update(cx, |buffer, cx| {
                buffer.group_until_transaction(transaction, cx);
            });
        }

        if self.text_highlights::<InputComposition>(cx).is_none() {
            self.ime_transaction.take();
        }
    }
}

fn build_style(
    settings: &ThemeSettings,
    get_field_editor_theme: Option<&GetFieldEditorTheme>,
    override_text_style: Option<&OverrideTextStyle>,
    cx: &AppContext,
) -> EditorStyle {
    let font_cache = cx.font_cache();
    let line_height_scalar = settings.line_height();
    let theme_id = settings.theme.meta.id;
    let mut theme = settings.theme.editor.clone();
    let mut style = if let Some(get_field_editor_theme) = get_field_editor_theme {
        let field_editor_theme = get_field_editor_theme(&settings.theme);
        theme.text_color = field_editor_theme.text.color;
        theme.selection = field_editor_theme.selection;
        theme.background = field_editor_theme
            .container
            .background_color
            .unwrap_or_default();
        EditorStyle {
            text: field_editor_theme.text,
            placeholder_text: field_editor_theme.placeholder_text,
            line_height_scalar,
            theme,
            theme_id,
        }
    } else {
        let font_family_id = settings.buffer_font_family;
        let font_family_name = cx.font_cache().family_name(font_family_id).unwrap();
        let font_properties = Default::default();
        let font_id = font_cache
            .select_font(font_family_id, &font_properties)
            .unwrap();
        let font_size = settings.buffer_font_size(cx);
        EditorStyle {
            text: TextStyle {
                color: settings.theme.editor.text_color,
                font_family_name,
                font_family_id,
                font_id,
                font_size,
                font_properties,
                underline: Default::default(),
                soft_wrap: false,
            },
            placeholder_text: None,
            line_height_scalar,
            theme,
            theme_id,
        }
    };

    if let Some(highlight_style) = override_text_style.and_then(|build_style| build_style(&style)) {
        if let Some(highlighted) = style
            .text
            .clone()
            .highlight(highlight_style, font_cache)
            .log_err()
        {
            style.text = highlighted;
        }
    }

    style
}

trait SelectionExt {
    fn offset_range(&self, buffer: &MultiBufferSnapshot) -> Range<usize>;
    fn point_range(&self, buffer: &MultiBufferSnapshot) -> Range<Point>;
    fn display_range(&self, map: &DisplaySnapshot) -> Range<DisplayPoint>;
    fn spanned_rows(&self, include_end_if_at_line_start: bool, map: &DisplaySnapshot)
        -> Range<u32>;
}

impl<T: ToPoint + ToOffset> SelectionExt for Selection<T> {
    fn point_range(&self, buffer: &MultiBufferSnapshot) -> Range<Point> {
        let start = self.start.to_point(buffer);
        let end = self.end.to_point(buffer);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    fn offset_range(&self, buffer: &MultiBufferSnapshot) -> Range<usize> {
        let start = self.start.to_offset(buffer);
        let end = self.end.to_offset(buffer);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    fn display_range(&self, map: &DisplaySnapshot) -> Range<DisplayPoint> {
        let start = self
            .start
            .to_point(&map.buffer_snapshot)
            .to_display_point(map);
        let end = self
            .end
            .to_point(&map.buffer_snapshot)
            .to_display_point(map);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    fn spanned_rows(
        &self,
        include_end_if_at_line_start: bool,
        map: &DisplaySnapshot,
    ) -> Range<u32> {
        let start = self.start.to_point(&map.buffer_snapshot);
        let mut end = self.end.to_point(&map.buffer_snapshot);
        if !include_end_if_at_line_start && start.row != end.row && end.column == 0 {
            end.row -= 1;
        }

        let buffer_start = map.prev_line_boundary(start).0;
        let buffer_end = map.next_line_boundary(end).0;
        buffer_start.row..buffer_end.row + 1
    }
}

impl<T: InvalidationRegion> InvalidationStack<T> {
    fn invalidate<S>(&mut self, selections: &[Selection<S>], buffer: &MultiBufferSnapshot)
    where
        S: Clone + ToOffset,
    {
        while let Some(region) = self.last() {
            let all_selections_inside_invalidation_ranges =
                if selections.len() == region.ranges().len() {
                    selections
                        .iter()
                        .zip(region.ranges().iter().map(|r| r.to_offset(buffer)))
                        .all(|(selection, invalidation_range)| {
                            let head = selection.head().to_offset(buffer);
                            invalidation_range.start <= head && invalidation_range.end >= head
                        })
                } else {
                    false
                };

            if all_selections_inside_invalidation_ranges {
                break;
            } else {
                self.pop();
            }
        }
    }
}

impl<T> Default for InvalidationStack<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Deref for InvalidationStack<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for InvalidationStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl InvalidationRegion for SnippetState {
    fn ranges(&self) -> &[Range<Anchor>] {
        &self.ranges[self.active_index]
    }
}

impl Deref for EditorStyle {
    type Target = theme::Editor;

    fn deref(&self) -> &Self::Target {
        &self.theme
    }
}

pub fn diagnostic_block_renderer(diagnostic: Diagnostic, is_valid: bool) -> RenderBlock {
    let mut highlighted_lines = Vec::new();

    for (index, line) in diagnostic.message.lines().enumerate() {
        let line = match &diagnostic.source {
            Some(source) if index == 0 => {
                let source_highlight = Vec::from_iter(0..source.len());
                highlight_diagnostic_message(source_highlight, &format!("{source}: {line}"))
            }

            _ => highlight_diagnostic_message(Vec::new(), line),
        };
        highlighted_lines.push(line);
    }
    let message = diagnostic.message;
    Arc::new(move |cx: &mut BlockContext| {
        let message = message.clone();
        let settings = settings::get::<ThemeSettings>(cx);
        let tooltip_style = settings.theme.tooltip.clone();
        let theme = &settings.theme.editor;
        let style = diagnostic_style(diagnostic.severity, is_valid, theme);
        let font_size = (style.text_scale_factor * settings.buffer_font_size(cx)).round();
        let anchor_x = cx.anchor_x;
        enum BlockContextToolip {}
        MouseEventHandler::new::<BlockContext, _>(cx.block_id, cx, |_, _| {
            Flex::column()
                .with_children(highlighted_lines.iter().map(|(line, highlights)| {
                    Label::new(
                        line.clone(),
                        style.message.clone().with_font_size(font_size),
                    )
                    .with_highlights(highlights.clone())
                    .contained()
                    .with_margin_left(anchor_x)
                }))
                .aligned()
                .left()
                .into_any()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, _, cx| {
            cx.write_to_clipboard(ClipboardItem::new(message.clone()));
        })
        // We really need to rethink this ID system...
        .with_tooltip::<BlockContextToolip>(
            cx.block_id,
            "Copy diagnostic message",
            None,
            tooltip_style,
            cx,
        )
        .into_any()
    })
}

pub fn highlight_diagnostic_message(
    initial_highlights: Vec<usize>,
    message: &str,
) -> (String, Vec<usize>) {
    let mut message_without_backticks = String::new();
    let mut prev_offset = 0;
    let mut inside_block = false;
    let mut highlights = initial_highlights;
    for (match_ix, (offset, _)) in message
        .match_indices('`')
        .chain([(message.len(), "")])
        .enumerate()
    {
        message_without_backticks.push_str(&message[prev_offset..offset]);
        if inside_block {
            highlights.extend(prev_offset - match_ix..offset - match_ix);
        }

        inside_block = !inside_block;
        prev_offset = offset + 1;
    }

    (message_without_backticks, highlights)
}

pub fn diagnostic_style(
    severity: DiagnosticSeverity,
    valid: bool,
    theme: &theme::Editor,
) -> DiagnosticStyle {
    match (severity, valid) {
        (DiagnosticSeverity::ERROR, true) => theme.error_diagnostic.clone(),
        (DiagnosticSeverity::ERROR, false) => theme.invalid_error_diagnostic.clone(),
        (DiagnosticSeverity::WARNING, true) => theme.warning_diagnostic.clone(),
        (DiagnosticSeverity::WARNING, false) => theme.invalid_warning_diagnostic.clone(),
        (DiagnosticSeverity::INFORMATION, true) => theme.information_diagnostic.clone(),
        (DiagnosticSeverity::INFORMATION, false) => theme.invalid_information_diagnostic.clone(),
        (DiagnosticSeverity::HINT, true) => theme.hint_diagnostic.clone(),
        (DiagnosticSeverity::HINT, false) => theme.invalid_hint_diagnostic.clone(),
        _ => theme.invalid_hint_diagnostic.clone(),
    }
}

pub fn combine_syntax_and_fuzzy_match_highlights(
    text: &str,
    default_style: HighlightStyle,
    syntax_ranges: impl Iterator<Item = (Range<usize>, HighlightStyle)>,
    match_indices: &[usize],
) -> Vec<(Range<usize>, HighlightStyle)> {
    let mut result = Vec::new();
    let mut match_indices = match_indices.iter().copied().peekable();

    for (range, mut syntax_highlight) in syntax_ranges.chain([(usize::MAX..0, Default::default())])
    {
        syntax_highlight.weight = None;

        // Add highlights for any fuzzy match characters before the next
        // syntax highlight range.
        while let Some(&match_index) = match_indices.peek() {
            if match_index >= range.start {
                break;
            }
            match_indices.next();
            let end_index = char_ix_after(match_index, text);
            let mut match_style = default_style;
            match_style.weight = Some(fonts::Weight::BOLD);
            result.push((match_index..end_index, match_style));
        }

        if range.start == usize::MAX {
            break;
        }

        // Add highlights for any fuzzy match characters within the
        // syntax highlight range.
        let mut offset = range.start;
        while let Some(&match_index) = match_indices.peek() {
            if match_index >= range.end {
                break;
            }

            match_indices.next();
            if match_index > offset {
                result.push((offset..match_index, syntax_highlight));
            }

            let mut end_index = char_ix_after(match_index, text);
            while let Some(&next_match_index) = match_indices.peek() {
                if next_match_index == end_index && next_match_index < range.end {
                    end_index = char_ix_after(next_match_index, text);
                    match_indices.next();
                } else {
                    break;
                }
            }

            let mut match_style = syntax_highlight;
            match_style.weight = Some(fonts::Weight::BOLD);
            result.push((match_index..end_index, match_style));
            offset = end_index;
        }

        if offset < range.end {
            result.push((offset..range.end, syntax_highlight));
        }
    }

    fn char_ix_after(ix: usize, text: &str) -> usize {
        ix + text[ix..].chars().next().unwrap().len_utf8()
    }

    result
}

pub fn styled_runs_for_code_label<'a>(
    label: &'a CodeLabel,
    syntax_theme: &'a theme::SyntaxTheme,
) -> impl 'a + Iterator<Item = (Range<usize>, HighlightStyle)> {
    let fade_out = HighlightStyle {
        fade_out: Some(0.35),
        ..Default::default()
    };

    let mut prev_end = label.filter_range.end;
    label
        .runs
        .iter()
        .enumerate()
        .flat_map(move |(ix, (range, highlight_id))| {
            let style = if let Some(style) = highlight_id.style(syntax_theme) {
                style
            } else {
                return Default::default();
            };
            let mut muted_style = style;
            muted_style.highlight(fade_out);

            let mut runs = SmallVec::<[(Range<usize>, HighlightStyle); 3]>::new();
            if range.start >= label.filter_range.end {
                if range.start > prev_end {
                    runs.push((prev_end..range.start, fade_out));
                }
                runs.push((range.clone(), muted_style));
            } else if range.end <= label.filter_range.end {
                runs.push((range.clone(), style));
            } else {
                runs.push((range.start..label.filter_range.end, style));
                runs.push((label.filter_range.end..range.end, muted_style));
            }
            prev_end = cmp::max(prev_end, range.end);

            if ix + 1 == label.runs.len() && label.text.len() > prev_end {
                runs.push((prev_end..label.text.len(), fade_out));
            }

            runs
        })
}

pub fn split_words<'a>(text: &'a str) -> impl std::iter::Iterator<Item = &'a str> + 'a {
    let mut index = 0;
    let mut codepoints = text.char_indices().peekable();

    std::iter::from_fn(move || {
        let start_index = index;
        while let Some((new_index, codepoint)) = codepoints.next() {
            index = new_index + codepoint.len_utf8();
            let current_upper = codepoint.is_uppercase();
            let next_upper = codepoints
                .peek()
                .map(|(_, c)| c.is_uppercase())
                .unwrap_or(false);

            if !current_upper && next_upper {
                return Some(&text[start_index..index]);
            }
        }

        index = text.len();
        if start_index < text.len() {
            return Some(&text[start_index..]);
        }
        None
    })
    .flat_map(|word| word.split_inclusive('_'))
    .flat_map(|word| word.split_inclusive('-'))
}

trait RangeToAnchorExt {
    fn to_anchors(self, snapshot: &MultiBufferSnapshot) -> Range<Anchor>;
}

impl<T: ToOffset> RangeToAnchorExt for Range<T> {
    fn to_anchors(self, snapshot: &MultiBufferSnapshot) -> Range<Anchor> {
        snapshot.anchor_after(self.start)..snapshot.anchor_before(self.end)
    }
}

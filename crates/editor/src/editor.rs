pub mod display_map;
mod element;
mod hover_popover;
pub mod items;
pub mod movement;
mod multi_buffer;
pub mod selections_collection;

#[cfg(any(test, feature = "test-support"))]
pub mod test;

use aho_corasick::AhoCorasick;
use anyhow::Result;
use clock::ReplicaId;
use collections::{BTreeMap, Bound, HashMap, HashSet, VecDeque};
pub use display_map::DisplayPoint;
use display_map::*;
pub use element::*;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    color::Color,
    elements::*,
    executor,
    fonts::{self, HighlightStyle, TextStyle},
    geometry::vector::{vec2f, Vector2F},
    impl_actions, impl_internal_actions,
    platform::CursorStyle,
    text_layout, AppContext, AsyncAppContext, ClipboardItem, Element, ElementBox, Entity,
    ModelHandle, MutableAppContext, RenderContext, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use hover_popover::{hide_hover, HoverState};
pub use language::{char_kind, CharKind};
use language::{
    BracketPair, Buffer, CodeAction, CodeLabel, Completion, Diagnostic, DiagnosticSeverity,
    IndentKind, IndentSize, Language, OffsetRangeExt, Point, Selection, SelectionGoal,
    TransactionId,
};
use multi_buffer::MultiBufferChunks;
pub use multi_buffer::{
    Anchor, AnchorRangeExt, ExcerptId, ExcerptRange, MultiBuffer, MultiBufferSnapshot, ToOffset,
    ToPoint,
};
use ordered_float::OrderedFloat;
use project::{Project, ProjectPath, ProjectTransaction};
use selections_collection::{resolve_multiple, MutableSelectionsCollection, SelectionsCollection};
use serde::{Deserialize, Serialize};
use settings::Settings;
use smallvec::SmallVec;
use smol::Timer;
use snippet::Snippet;
use std::{
    any::TypeId,
    borrow::Cow,
    cmp::{self, Ordering, Reverse},
    mem,
    ops::{Deref, DerefMut, Range, RangeInclusive},
    sync::Arc,
    time::{Duration, Instant},
};
pub use sum_tree::Bias;
use theme::{DiagnosticStyle, Theme};
use util::{post_inc, ResultExt, TryFutureExt};
use workspace::{ItemNavHistory, Workspace};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);
const MAX_LINE_LEN: usize = 1024;
const MIN_NAVIGATION_HISTORY_ROW_DELTA: i64 = 10;
const MAX_SELECTION_HISTORY_LEN: usize = 1024;

#[derive(Clone, Deserialize, PartialEq)]
pub struct SelectNext {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(Clone, PartialEq)]
pub struct GoToDiagnostic(pub Direction);

#[derive(Clone, PartialEq)]
pub struct Scroll(pub Vector2F);

#[derive(Clone, PartialEq)]
pub struct Select(pub SelectPhase);

#[derive(Clone, Debug, PartialEq)]
pub struct Jump {
    path: ProjectPath,
    position: Point,
    anchor: language::Anchor,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Input(pub String);

#[derive(Clone, Deserialize, PartialEq)]
pub struct SelectToBeginningOfLine {
    #[serde(default)]
    stop_at_soft_wraps: bool,
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

actions!(
    editor,
    [
        Cancel,
        Backspace,
        Delete,
        Newline,
        GoToNextDiagnostic,
        GoToPrevDiagnostic,
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
        Transpose,
        Cut,
        Copy,
        Paste,
        Undo,
        Redo,
        MoveUp,
        MoveDown,
        MoveLeft,
        MoveRight,
        MoveToPreviousWordStart,
        MoveToPreviousSubwordStart,
        MoveToNextWordEnd,
        MoveToNextSubwordEnd,
        MoveToBeginningOfLine,
        MoveToEndOfLine,
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
        SelectToBeginning,
        SelectToEnd,
        SelectAll,
        SelectLine,
        SplitSelectionIntoLines,
        AddSelectionAbove,
        AddSelectionBelow,
        Tab,
        TabPrev,
        ToggleComments,
        SelectLargerSyntaxNode,
        SelectSmallerSyntaxNode,
        GoToDefinition,
        MoveToEnclosingBracket,
        UndoSelection,
        RedoSelection,
        FindAllReferences,
        Rename,
        ConfirmRename,
        PageUp,
        PageDown,
        Fold,
        UnfoldLines,
        FoldSelectedRanges,
        ShowCompletions,
        OpenExcerpts,
        RestartLanguageServer,
        Hover,
    ]
);

impl_actions!(
    editor,
    [
        Input,
        SelectNext,
        SelectToBeginningOfLine,
        SelectToEndOfLine,
        ToggleCodeActions,
        ConfirmCompletion,
        ConfirmCodeAction,
    ]
);

impl_internal_actions!(editor, [Scroll, Select, Jump]);

enum DocumentHighlightRead {}
enum DocumentHighlightWrite {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Editor::new_file);
    cx.add_action(|this: &mut Editor, action: &Scroll, cx| this.set_scroll_position(action.0, cx));
    cx.add_action(Editor::select);
    cx.add_action(Editor::cancel);
    cx.add_action(Editor::handle_input);
    cx.add_action(Editor::newline);
    cx.add_action(Editor::backspace);
    cx.add_action(Editor::delete);
    cx.add_action(Editor::tab);
    cx.add_action(Editor::tab_prev);
    cx.add_action(Editor::indent);
    cx.add_action(Editor::outdent);
    cx.add_action(Editor::delete_line);
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
    cx.add_action(Editor::move_down);
    cx.add_action(Editor::move_left);
    cx.add_action(Editor::move_right);
    cx.add_action(Editor::move_to_previous_word_start);
    cx.add_action(Editor::move_to_previous_subword_start);
    cx.add_action(Editor::move_to_next_word_end);
    cx.add_action(Editor::move_to_next_subword_end);
    cx.add_action(Editor::move_to_beginning_of_line);
    cx.add_action(Editor::move_to_end_of_line);
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
    cx.add_action(Editor::select_to_beginning);
    cx.add_action(Editor::select_to_end);
    cx.add_action(Editor::select_all);
    cx.add_action(Editor::select_line);
    cx.add_action(Editor::split_selection_into_lines);
    cx.add_action(Editor::add_selection_above);
    cx.add_action(Editor::add_selection_below);
    cx.add_action(Editor::select_next);
    cx.add_action(Editor::toggle_comments);
    cx.add_action(Editor::select_larger_syntax_node);
    cx.add_action(Editor::select_smaller_syntax_node);
    cx.add_action(Editor::move_to_enclosing_bracket);
    cx.add_action(Editor::undo_selection);
    cx.add_action(Editor::redo_selection);
    cx.add_action(Editor::go_to_next_diagnostic);
    cx.add_action(Editor::go_to_prev_diagnostic);
    cx.add_action(Editor::go_to_definition);
    cx.add_action(Editor::page_up);
    cx.add_action(Editor::page_down);
    cx.add_action(Editor::fold);
    cx.add_action(Editor::unfold_lines);
    cx.add_action(Editor::fold_selected_ranges);
    cx.add_action(Editor::show_completions);
    cx.add_action(Editor::toggle_code_actions);
    cx.add_action(Editor::open_excerpts);
    cx.add_action(Editor::jump);
    cx.add_action(Editor::restart_language_server);
    cx.add_async_action(Editor::confirm_completion);
    cx.add_async_action(Editor::confirm_code_action);
    cx.add_async_action(Editor::rename);
    cx.add_async_action(Editor::confirm_rename);
    cx.add_async_action(Editor::find_all_references);

    hover_popover::init(cx);

    workspace::register_project_item::<Editor>(cx);
    workspace::register_followable_item::<Editor>(cx);
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
        overshoot: u32,
    },
    Extend {
        position: DisplayPoint,
        click_count: usize,
    },
    Update {
        position: DisplayPoint,
        overshoot: u32,
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

#[derive(PartialEq, Eq)]
pub enum Autoscroll {
    Fit,
    Center,
    Newest,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EditorMode {
    SingleLine,
    AutoHeight { max_lines: usize },
    Full,
}

#[derive(Clone)]
pub enum SoftWrap {
    None,
    EditorWidth,
    Column(u32),
}

#[derive(Clone)]
pub struct EditorStyle {
    pub text: TextStyle,
    pub placeholder_text: Option<TextStyle>,
    pub theme: theme::Editor,
}

type CompletionId = usize;

pub type GetFieldEditorTheme = fn(&theme::Theme) -> theme::FieldEditor;

type OverrideTextStyle = dyn Fn(&EditorStyle) -> Option<HighlightStyle>;

pub struct Editor {
    handle: WeakViewHandle<Self>,
    buffer: ModelHandle<MultiBuffer>,
    display_map: ModelHandle<DisplayMap>,
    pub selections: SelectionsCollection,
    columnar_selection_tail: Option<Anchor>,
    add_selections_state: Option<AddSelectionsState>,
    select_next_state: Option<SelectNextState>,
    selection_history: SelectionHistory,
    autoclose_stack: InvalidationStack<BracketPairState>,
    snippet_stack: InvalidationStack<SnippetState>,
    select_larger_syntax_node_stack: Vec<Box<[Selection<usize>]>>,
    active_diagnostics: Option<ActiveDiagnosticGroup>,
    scroll_position: Vector2F,
    scroll_top_anchor: Anchor,
    autoscroll_request: Option<(Autoscroll, bool)>,
    soft_wrap_mode_override: Option<settings::SoftWrap>,
    get_field_editor_theme: Option<GetFieldEditorTheme>,
    override_text_style: Option<Box<OverrideTextStyle>>,
    project: Option<ModelHandle<Project>>,
    focused: bool,
    show_local_cursors: bool,
    show_local_selections: bool,
    blink_epoch: usize,
    blinking_paused: bool,
    mode: EditorMode,
    vertical_scroll_margin: f32,
    placeholder_text: Option<Arc<str>>,
    highlighted_rows: Option<Range<u32>>,
    background_highlights: BTreeMap<TypeId, (fn(&Theme) -> Color, Vec<Range<Anchor>>)>,
    nav_history: Option<ItemNavHistory>,
    context_menu: Option<ContextMenu>,
    completion_tasks: Vec<(CompletionId, Task<Option<()>>)>,
    next_completion_id: CompletionId,
    available_code_actions: Option<(ModelHandle<Buffer>, Arc<[CodeAction]>)>,
    code_actions_task: Option<Task<()>>,
    document_highlights_task: Option<Task<()>>,
    pending_rename: Option<RenameState>,
    searchable: bool,
    cursor_shape: CursorShape,
    keymap_context_layers: BTreeMap<TypeId, gpui::keymap::Context>,
    input_enabled: bool,
    leader_replica_id: Option<u16>,
    hover_state: HoverState,
}

pub struct EditorSnapshot {
    pub mode: EditorMode,
    pub display_snapshot: DisplaySnapshot,
    pub placeholder_text: Option<Arc<str>>,
    is_focused: bool,
    scroll_position: Vector2F,
    scroll_top_anchor: Anchor,
}

#[derive(Clone, Debug)]
struct SelectionHistoryEntry {
    selections: Arc<[Selection<Anchor>]>,
    select_next_state: Option<SelectNextState>,
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

    fn transaction(
        &self,
        transaction_id: TransactionId,
    ) -> Option<&(Arc<[Selection<Anchor>]>, Option<Arc<[Selection<Anchor>]>>)> {
        self.selections_by_transaction.get(&transaction_id)
    }

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

struct BracketPairState {
    ranges: Vec<Range<Anchor>>,
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
        cx: &mut RenderContext<Editor>,
    ) -> (DisplayPoint, ElementBox) {
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
    completions: Arc<[Completion]>,
    match_candidates: Vec<StringMatchCandidate>,
    matches: Arc<[StringMatch]>,
    selected_item: usize,
    list: UniformListState,
}

impl CompletionsMenu {
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

    fn visible(&self) -> bool {
        !self.matches.is_empty()
    }

    fn render(&self, style: EditorStyle, cx: &mut RenderContext<Editor>) -> ElementBox {
        enum CompletionTag {}

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
                        MouseEventHandler::new::<CompletionTag, _, _>(
                            mat.candidate_id,
                            cx,
                            |state, _| {
                                let item_style = if item_ix == selected_item {
                                    style.autocomplete.selected_item
                                } else if state.hovered {
                                    style.autocomplete.hovered_item
                                } else {
                                    style.autocomplete.item
                                };

                                Text::new(completion.label.text.clone(), style.text.clone())
                                    .with_soft_wrap(false)
                                    .with_highlights(combine_syntax_and_fuzzy_match_highlights(
                                        &completion.label.text,
                                        style.text.color.into(),
                                        styled_runs_for_code_label(
                                            &completion.label,
                                            &style.syntax,
                                        ),
                                        &mat.positions,
                                    ))
                                    .contained()
                                    .with_style(item_style)
                                    .boxed()
                            },
                        )
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_mouse_down(move |_, cx| {
                            cx.dispatch_action(ConfirmCompletion {
                                item_ix: Some(item_ix),
                            });
                        })
                        .boxed(),
                    );
                }
            },
        )
        .with_width_from_item(
            self.matches
                .iter()
                .enumerate()
                .max_by_key(|(_, mat)| {
                    self.completions[mat.candidate_id]
                        .label
                        .text
                        .chars()
                        .count()
                })
                .map(|(ix, _)| ix),
        )
        .contained()
        .with_style(container_style)
        .boxed()
    }

    pub async fn filter(&mut self, query: Option<&str>, executor: Arc<executor::Background>) {
        let mut matches = if let Some(query) = query {
            fuzzy::match_strings(
                &self.match_candidates,
                query,
                false,
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
        matches.sort_unstable_by_key(|mat| {
            (
                Reverse(OrderedFloat(mat.score)),
                self.completions[mat.candidate_id].sort_key(),
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

    fn visible(&self) -> bool {
        !self.actions.is_empty()
    }

    fn render(
        &self,
        mut cursor_position: DisplayPoint,
        style: EditorStyle,
        cx: &mut RenderContext<Editor>,
    ) -> (DisplayPoint, ElementBox) {
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
                        MouseEventHandler::new::<ActionTag, _, _>(item_ix, cx, |state, _| {
                            let item_style = if item_ix == selected_item {
                                style.autocomplete.selected_item
                            } else if state.hovered {
                                style.autocomplete.hovered_item
                            } else {
                                style.autocomplete.item
                            };

                            Text::new(action.lsp_action.title.clone(), style.text.clone())
                                .with_soft_wrap(false)
                                .contained()
                                .with_style(item_style)
                                .boxed()
                        })
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_mouse_down(move |_, cx| {
                            cx.dispatch_action(ConfirmCodeAction {
                                item_ix: Some(item_ix),
                            });
                        })
                        .boxed(),
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
        .boxed();

        if self.deployed_from_indicator {
            *cursor_position.column_mut() = 0;
        }

        (cursor_position, element)
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
}

#[derive(Debug)]
pub struct NavigationData {
    // Matching offsets for anchor and scroll_top_anchor allows us to recreate the anchor if the buffer
    // has since been closed
    cursor_anchor: Anchor,
    cursor_position: Point,
    scroll_position: Vector2F,
    scroll_top_anchor: Anchor,
    scroll_top_row: u32,
}

pub struct EditorCreated(pub ViewHandle<Editor>);

impl Editor {
    pub fn single_line(
        field_editor_style: Option<GetFieldEditorTheme>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(
            EditorMode::SingleLine,
            buffer,
            None,
            field_editor_style,
            None,
            cx,
        )
    }

    pub fn auto_height(
        max_lines: usize,
        field_editor_style: Option<GetFieldEditorTheme>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(
            EditorMode::AutoHeight { max_lines },
            buffer,
            None,
            field_editor_style,
            None,
            cx,
        )
    }

    pub fn for_buffer(
        buffer: ModelHandle<Buffer>,
        project: Option<ModelHandle<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(EditorMode::Full, buffer, project, None, None, cx)
    }

    pub fn for_multibuffer(
        buffer: ModelHandle<MultiBuffer>,
        project: Option<ModelHandle<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::new(EditorMode::Full, buffer, project, None, None, cx)
    }

    pub fn clone(&self, cx: &mut ViewContext<Self>) -> Self {
        let mut clone = Self::new(
            self.mode,
            self.buffer.clone(),
            self.project.clone(),
            self.get_field_editor_theme,
            Some(self.selections.clone()),
            cx,
        );
        clone.scroll_position = self.scroll_position;
        clone.scroll_top_anchor = self.scroll_top_anchor.clone();
        clone.searchable = self.searchable;
        clone
    }

    fn new(
        mode: EditorMode,
        buffer: ModelHandle<MultiBuffer>,
        project: Option<ModelHandle<Project>>,
        get_field_editor_theme: Option<GetFieldEditorTheme>,
        selections: Option<SelectionsCollection>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let display_map = cx.add_model(|cx| {
            let settings = cx.global::<Settings>();
            let style = build_style(&*settings, get_field_editor_theme, None, cx);
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
        cx.observe(&buffer, Self::on_buffer_changed).detach();
        cx.subscribe(&buffer, Self::on_buffer_event).detach();
        cx.observe(&display_map, Self::on_display_map_changed)
            .detach();

        let selections = selections
            .unwrap_or_else(|| SelectionsCollection::new(display_map.clone(), buffer.clone()));

        let mut this = Self {
            handle: cx.weak_handle(),
            buffer,
            display_map,
            selections,
            columnar_selection_tail: None,
            add_selections_state: None,
            select_next_state: None,
            selection_history: Default::default(),
            autoclose_stack: Default::default(),
            snippet_stack: Default::default(),
            select_larger_syntax_node_stack: Vec::new(),
            active_diagnostics: None,
            soft_wrap_mode_override: None,
            get_field_editor_theme,
            project,
            scroll_position: Vector2F::zero(),
            scroll_top_anchor: Anchor::min(),
            autoscroll_request: None,
            focused: false,
            show_local_cursors: false,
            show_local_selections: true,
            blink_epoch: 0,
            blinking_paused: false,
            mode,
            vertical_scroll_margin: 3.0,
            placeholder_text: None,
            highlighted_rows: None,
            background_highlights: Default::default(),
            nav_history: None,
            context_menu: None,
            completion_tasks: Default::default(),
            next_completion_id: 0,
            available_code_actions: Default::default(),
            code_actions_task: Default::default(),

            document_highlights_task: Default::default(),
            pending_rename: Default::default(),
            searchable: true,
            override_text_style: None,
            cursor_shape: Default::default(),
            keymap_context_layers: Default::default(),
            input_enabled: true,
            leader_replica_id: None,
            hover_state: Default::default(),
        };
        this.end_selection(cx);

        let editor_created_event = EditorCreated(cx.handle());
        cx.emit_global(editor_created_event);

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

    pub fn title(&self, cx: &AppContext) -> String {
        self.buffer().read(cx).title(cx)
    }

    pub fn snapshot(&mut self, cx: &mut MutableAppContext) -> EditorSnapshot {
        EditorSnapshot {
            mode: self.mode,
            display_snapshot: self.display_map.update(cx, |map, cx| map.snapshot(cx)),
            scroll_position: self.scroll_position,
            scroll_top_anchor: self.scroll_top_anchor.clone(),
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
    ) -> Option<&'a Arc<Language>> {
        self.buffer.read(cx).language_at(point, cx)
    }

    fn style(&self, cx: &AppContext) -> EditorStyle {
        build_style(
            cx.global::<Settings>(),
            self.get_field_editor_theme,
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

    pub fn set_vertical_scroll_margin(&mut self, margin_rows: usize, cx: &mut ViewContext<Self>) {
        self.vertical_scroll_margin = margin_rows as f32;
        cx.notify();
    }

    pub fn set_scroll_position(&mut self, scroll_position: Vector2F, cx: &mut ViewContext<Self>) {
        self.set_scroll_position_internal(scroll_position, true, cx);
    }

    fn set_scroll_position_internal(
        &mut self,
        scroll_position: Vector2F,
        local: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if scroll_position.y() == 0. {
            self.scroll_top_anchor = Anchor::min();
            self.scroll_position = scroll_position;
        } else {
            let scroll_top_buffer_offset =
                DisplayPoint::new(scroll_position.y() as u32, 0).to_offset(&map, Bias::Right);
            let anchor = map
                .buffer_snapshot
                .anchor_at(scroll_top_buffer_offset, Bias::Right);
            self.scroll_position = vec2f(
                scroll_position.x(),
                scroll_position.y() - anchor.to_display_point(&map).row() as f32,
            );
            self.scroll_top_anchor = anchor;
        }

        self.autoscroll_request.take();
        hide_hover(self, cx);

        cx.emit(Event::ScrollPositionChanged { local });
        cx.notify();
    }

    fn set_scroll_top_anchor(
        &mut self,
        anchor: Anchor,
        position: Vector2F,
        cx: &mut ViewContext<Self>,
    ) {
        self.scroll_top_anchor = anchor;
        self.scroll_position = position;
        cx.emit(Event::ScrollPositionChanged { local: false });
        cx.notify();
    }

    pub fn set_cursor_shape(&mut self, cursor_shape: CursorShape, cx: &mut ViewContext<Self>) {
        self.cursor_shape = cursor_shape;
        cx.notify();
    }

    pub fn set_clip_at_line_ends(&mut self, clip: bool, cx: &mut ViewContext<Self>) {
        if self.display_map.read(cx).clip_at_line_ends != clip {
            self.display_map
                .update(cx, |map, _| map.clip_at_line_ends = clip);
        }
    }

    pub fn set_keymap_context_layer<Tag: 'static>(&mut self, context: gpui::keymap::Context) {
        self.keymap_context_layers
            .insert(TypeId::of::<Tag>(), context);
    }

    pub fn remove_keymap_context_layer<Tag: 'static>(&mut self) {
        self.keymap_context_layers.remove(&TypeId::of::<Tag>());
    }

    pub fn set_input_enabled(&mut self, input_enabled: bool) {
        self.input_enabled = input_enabled;
    }

    pub fn scroll_position(&self, cx: &mut ViewContext<Self>) -> Vector2F {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        compute_scroll_position(&display_map, self.scroll_position, &self.scroll_top_anchor)
    }

    pub fn clamp_scroll_left(&mut self, max: f32) -> bool {
        if max < self.scroll_position.x() {
            self.scroll_position.set_x(max);
            true
        } else {
            false
        }
    }

    pub fn autoscroll_vertically(
        &mut self,
        viewport_height: f32,
        line_height: f32,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        let visible_lines = viewport_height / line_height;
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut scroll_position =
            compute_scroll_position(&display_map, self.scroll_position, &self.scroll_top_anchor);
        let max_scroll_top = if matches!(self.mode, EditorMode::AutoHeight { .. }) {
            (display_map.max_point().row() as f32 - visible_lines + 1.).max(0.)
        } else {
            display_map.max_point().row().saturating_sub(1) as f32
        };
        if scroll_position.y() > max_scroll_top {
            scroll_position.set_y(max_scroll_top);
            self.set_scroll_position(scroll_position, cx);
        }

        let (autoscroll, local) = if let Some(autoscroll) = self.autoscroll_request.take() {
            autoscroll
        } else {
            return false;
        };

        let first_cursor_top;
        let last_cursor_bottom;
        if let Some(highlighted_rows) = &self.highlighted_rows {
            first_cursor_top = highlighted_rows.start as f32;
            last_cursor_bottom = first_cursor_top + 1.;
        } else if autoscroll == Autoscroll::Newest {
            let newest_selection = self.selections.newest::<Point>(cx);
            first_cursor_top = newest_selection.head().to_display_point(&display_map).row() as f32;
            last_cursor_bottom = first_cursor_top + 1.;
        } else {
            let selections = self.selections.all::<Point>(cx);
            first_cursor_top = selections
                .first()
                .unwrap()
                .head()
                .to_display_point(&display_map)
                .row() as f32;
            last_cursor_bottom = selections
                .last()
                .unwrap()
                .head()
                .to_display_point(&display_map)
                .row() as f32
                + 1.0;
        }

        let margin = if matches!(self.mode, EditorMode::AutoHeight { .. }) {
            0.
        } else {
            ((visible_lines - (last_cursor_bottom - first_cursor_top)) / 2.0).floor()
        };
        if margin < 0.0 {
            return false;
        }

        match autoscroll {
            Autoscroll::Fit | Autoscroll::Newest => {
                let margin = margin.min(self.vertical_scroll_margin);
                let target_top = (first_cursor_top - margin).max(0.0);
                let target_bottom = last_cursor_bottom + margin;
                let start_row = scroll_position.y();
                let end_row = start_row + visible_lines;

                if target_top < start_row {
                    scroll_position.set_y(target_top);
                    self.set_scroll_position_internal(scroll_position, local, cx);
                } else if target_bottom >= end_row {
                    scroll_position.set_y(target_bottom - visible_lines);
                    self.set_scroll_position_internal(scroll_position, local, cx);
                }
            }
            Autoscroll::Center => {
                scroll_position.set_y((first_cursor_top - margin).max(0.0));
                self.set_scroll_position_internal(scroll_position, local, cx);
            }
        }

        true
    }

    pub fn autoscroll_horizontally(
        &mut self,
        start_row: u32,
        viewport_width: f32,
        scroll_width: f32,
        max_glyph_width: f32,
        layouts: &[text_layout::Line],
        cx: &mut ViewContext<Self>,
    ) -> bool {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);

        let mut target_left;
        let mut target_right;

        if self.highlighted_rows.is_some() {
            target_left = 0.0_f32;
            target_right = 0.0_f32;
        } else {
            target_left = std::f32::INFINITY;
            target_right = 0.0_f32;
            for selection in selections {
                let head = selection.head().to_display_point(&display_map);
                if head.row() >= start_row && head.row() < start_row + layouts.len() as u32 {
                    let start_column = head.column().saturating_sub(3);
                    let end_column = cmp::min(display_map.line_len(head.row()), head.column() + 3);
                    target_left = target_left.min(
                        layouts[(head.row() - start_row) as usize]
                            .x_for_index(start_column as usize),
                    );
                    target_right = target_right.max(
                        layouts[(head.row() - start_row) as usize].x_for_index(end_column as usize)
                            + max_glyph_width,
                    );
                }
            }
        }

        target_right = target_right.min(scroll_width);

        if target_right - target_left > viewport_width {
            return false;
        }

        let scroll_left = self.scroll_position.x() * max_glyph_width;
        let scroll_right = scroll_left + viewport_width;

        if target_left < scroll_left {
            self.scroll_position.set_x(target_left / max_glyph_width);
            true
        } else if target_right > scroll_right {
            self.scroll_position
                .set_x((target_right - viewport_width) / max_glyph_width);
            true
        } else {
            false
        }
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
        self.select_larger_syntax_node_stack.clear();
        self.autoclose_stack
            .invalidate(&self.selections.disjoint_anchors(), buffer);
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
        }

        self.pause_cursor_blinking(cx);
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
        self.buffer.update(cx, |buffer, cx| buffer.edit(edits, cx));
    }

    pub fn edit_with_autoindent<I, S, T>(&mut self, edits: I, cx: &mut ViewContext<Self>)
    where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        self.buffer
            .update(cx, |buffer, cx| buffer.edit_with_autoindent(edits, cx));
    }

    fn select(&mut self, Select(phase): &Select, cx: &mut ViewContext<Self>) {
        self.hide_context_menu(cx);

        match phase {
            SelectPhase::Begin {
                position,
                add,
                click_count,
            } => self.begin_selection(*position, *add, *click_count, cx),
            SelectPhase::BeginColumnar {
                position,
                overshoot,
            } => self.begin_columnar_selection(*position, *overshoot, cx),
            SelectPhase::Extend {
                position,
                click_count,
            } => self.extend_selection(*position, *click_count, cx),
            SelectPhase::Update {
                position,
                overshoot,
                scroll_position,
            } => self.update_selection(*position, *overshoot, *scroll_position, cx),
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
            pending_selection.start = tail_anchor.clone();
        } else {
            pending_selection.end = tail_anchor.clone();
            pending_selection.reversed = true;
        }

        let mut pending_mode = self.selections.pending_mode().unwrap();
        match &mut pending_mode {
            SelectMode::Word(range) | SelectMode::Line(range) => {
                *range = tail_anchor.clone()..tail_anchor
            }
            _ => {}
        }

        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            cx.emit(Event::Activate);
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let newest_selection = self.selections.newest_anchor().clone();
        let position = display_map.clip_point(position, Bias::Left);

        let start;
        let end;
        let mode;
        match click_count {
            1 => {
                start = buffer.anchor_before(position.to_point(&display_map));
                end = start.clone();
                mode = SelectMode::Character;
            }
            2 => {
                let range = movement::surrounding_word(&display_map, position);
                start = buffer.anchor_before(range.start.to_point(&display_map));
                end = buffer.anchor_before(range.end.to_point(&display_map));
                mode = SelectMode::Word(start.clone()..end.clone());
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
            }
            _ => {
                start = buffer.anchor_before(0);
                end = buffer.anchor_before(buffer.len());
                mode = SelectMode::All;
            }
        }

        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            if !add {
                s.clear_disjoint();
            } else if click_count > 1 {
                s.delete(newest_selection.id)
            }

            s.set_pending_range(start..end, mode);
        });
    }

    fn begin_columnar_selection(
        &mut self,
        position: DisplayPoint,
        overshoot: u32,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.focused {
            cx.focus_self();
            cx.emit(Event::Activate);
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let tail = self.selections.newest::<Point>(cx).tail();
        self.columnar_selection_tail = Some(display_map.buffer_snapshot.anchor_before(tail));

        self.select_columns(
            tail.to_display_point(&display_map),
            position,
            overshoot,
            &display_map,
            cx,
        );
    }

    fn update_selection(
        &mut self,
        position: DisplayPoint,
        overshoot: u32,
        scroll_position: Vector2F,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if let Some(tail) = self.columnar_selection_tail.as_ref() {
            let tail = tail.to_display_point(&display_map);
            self.select_columns(tail, position, overshoot, &display_map, cx);
        } else if let Some(mut pending) = self.selections.pending_anchor().clone() {
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
            log::error!("update_selection dispatched with no pending selection");
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
        overshoot: u32,
        display_map: &DisplaySnapshot,
        cx: &mut ViewContext<Self>,
    ) {
        let start_row = cmp::min(tail.row(), head.row());
        let end_row = cmp::max(tail.row(), head.row());
        let start_column = cmp::min(tail.column(), head.column() + overshoot);
        let end_column = cmp::max(tail.column(), head.column() + overshoot);
        let reversed = start_column < tail.column();

        let selection_ranges = (start_row..=end_row)
            .filter_map(|row| {
                if start_column <= display_map.line_len(row) && !display_map.is_block_line(row) {
                    let start = display_map
                        .clip_point(DisplayPoint::new(row, start_column), Bias::Left)
                        .to_point(&display_map);
                    let end = display_map
                        .clip_point(DisplayPoint::new(row, end_column), Bias::Right)
                        .to_point(&display_map);
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

    pub fn is_selecting(&self) -> bool {
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

        if self.snippet_stack.pop().is_some() {
            return;
        }

        if self.mode == EditorMode::Full {
            if self.active_diagnostics.is_some() {
                self.dismiss_diagnostics(cx);
                return;
            }

            if self.change_selections(Some(Autoscroll::Fit), cx, |s| s.try_cancel()) {
                return;
            }
        }

        cx.propagate_action();
    }

    pub fn handle_input(&mut self, action: &Input, cx: &mut ViewContext<Self>) {
        if !self.input_enabled {
            cx.propagate_action();
            return;
        }

        let text = action.0.as_ref();
        if !self.skip_autoclose_end(text, cx) {
            self.transact(cx, |this, cx| {
                if !this.surround_with_bracket_pair(text, cx) {
                    this.insert(text, cx);
                    this.autoclose_bracket_pairs(cx);
                }
            });
            self.trigger_completion_on_input(text, cx);
        }
    }

    pub fn newline(&mut self, _: &Newline, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            let (edits, selection_fixup_info): (Vec<_>, Vec<_>) = {
                let selections = this.selections.all::<usize>(cx);

                let buffer = this.buffer.read(cx).snapshot(cx);
                selections
                    .iter()
                    .map(|selection| {
                        let start_point = selection.start.to_point(&buffer);
                        let mut indent = buffer.indent_size_for_line(start_point.row);
                        indent.len = cmp::min(indent.len, start_point.column);
                        let start = selection.start;
                        let end = selection.end;

                        let mut insert_extra_newline = false;
                        if let Some(language) = buffer.language() {
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

                            insert_extra_newline = language.brackets().iter().any(|pair| {
                                let pair_start = pair.start.trim_end();
                                let pair_end = pair.end.trim_start();

                                pair.newline
                                    && buffer
                                        .contains_str_at(end + trailing_whitespace_len, pair_end)
                                    && buffer.contains_str_at(
                                        (start - leading_whitespace_len)
                                            .saturating_sub(pair_start.len()),
                                        pair_start,
                                    )
                            });
                        }

                        let mut new_text = String::with_capacity(1 + indent.len as usize);
                        new_text.push('\n');
                        new_text.extend(indent.chars());
                        if insert_extra_newline {
                            new_text = new_text.repeat(2);
                        }

                        let anchor = buffer.anchor_after(end);
                        let new_selection = selection.map(|_| anchor.clone());
                        (
                            (start..end, new_text),
                            (insert_extra_newline, new_selection),
                        )
                    })
                    .unzip()
            };

            this.buffer.update(cx, |buffer, cx| {
                buffer.edit_with_autoindent(edits, cx);
            });
            let buffer = this.buffer.read(cx).snapshot(cx);
            let new_selections = selection_fixup_info
                .into_iter()
                .map(|(extra_newline_inserted, new_selection)| {
                    let mut cursor = new_selection.end.to_point(&buffer);
                    if extra_newline_inserted {
                        cursor.row -= 1;
                        cursor.column = buffer.line_len(cursor.row);
                    }
                    new_selection.map(|_| cursor.clone())
                })
                .collect();

            this.change_selections(Some(Autoscroll::Fit), cx, |s| s.select(new_selections));
        });
    }

    pub fn insert(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        let text: Arc<str> = text.into();
        self.transact(cx, |this, cx| {
            let old_selections = this.selections.all_adjusted(cx);
            let selection_anchors = this.buffer.update(cx, |buffer, cx| {
                let anchors = {
                    let snapshot = buffer.read(cx);
                    old_selections
                        .iter()
                        .map(|s| {
                            let anchor = snapshot.anchor_after(s.end);
                            s.map(|_| anchor.clone())
                        })
                        .collect::<Vec<_>>()
                };
                buffer.edit_with_autoindent(
                    old_selections
                        .iter()
                        .map(|s| (s.start..s.end, text.clone())),
                    cx,
                );
                anchors
            });

            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.select_anchors(selection_anchors);
            })
        });
    }

    fn trigger_completion_on_input(&mut self, text: &str, cx: &mut ViewContext<Self>) {
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

    fn surround_with_bracket_pair(&mut self, text: &str, cx: &mut ViewContext<Self>) -> bool {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        if let Some(pair) = snapshot
            .language()
            .and_then(|language| language.brackets().iter().find(|b| b.start == text))
            .cloned()
        {
            if self
                .selections
                .all::<usize>(cx)
                .iter()
                .any(|selection| selection.is_empty())
            {
                return false;
            }

            let mut selections = self.selections.disjoint_anchors().to_vec();
            for selection in &mut selections {
                selection.end = selection.end.bias_left(&snapshot);
            }
            drop(snapshot);

            self.buffer.update(cx, |buffer, cx| {
                let pair_start: Arc<str> = pair.start.clone().into();
                let pair_end: Arc<str> = pair.end.clone().into();
                buffer.edit(
                    selections
                        .iter()
                        .map(|s| (s.start.clone()..s.start.clone(), pair_start.clone()))
                        .chain(
                            selections
                                .iter()
                                .map(|s| (s.end.clone()..s.end.clone(), pair_end.clone())),
                        ),
                    cx,
                );
            });

            let snapshot = self.buffer.read(cx).read(cx);
            for selection in &mut selections {
                selection.end = selection.end.bias_right(&snapshot);
            }
            drop(snapshot);

            self.change_selections(None, cx, |s| s.select_anchors(selections));
            true
        } else {
            false
        }
    }

    fn autoclose_bracket_pairs(&mut self, cx: &mut ViewContext<Self>) {
        let selections = self.selections.all::<usize>(cx);
        let mut bracket_pair_state = None;
        let mut new_selections = None;
        self.buffer.update(cx, |buffer, cx| {
            let mut snapshot = buffer.snapshot(cx);
            let left_biased_selections = selections
                .iter()
                .map(|selection| selection.map(|p| snapshot.anchor_before(p)))
                .collect::<Vec<_>>();

            let autoclose_pair = snapshot.language().and_then(|language| {
                let first_selection_start = selections.first().unwrap().start;
                let pair = language.brackets().iter().find(|pair| {
                    snapshot.contains_str_at(
                        first_selection_start.saturating_sub(pair.start.len()),
                        &pair.start,
                    )
                });
                pair.and_then(|pair| {
                    let should_autoclose = selections.iter().all(|selection| {
                        // Ensure all selections are parked at the end of a pair start.
                        if snapshot.contains_str_at(
                            selection.start.saturating_sub(pair.start.len()),
                            &pair.start,
                        ) {
                            snapshot
                                .chars_at(selection.start)
                                .next()
                                .map_or(true, |c| language.should_autoclose_before(c))
                        } else {
                            false
                        }
                    });

                    if should_autoclose {
                        Some(pair.clone())
                    } else {
                        None
                    }
                })
            });

            if let Some(pair) = autoclose_pair {
                let selection_ranges = selections
                    .iter()
                    .map(|selection| {
                        let start = selection.start.to_offset(&snapshot);
                        start..start
                    })
                    .collect::<SmallVec<[_; 32]>>();

                let pair_end: Arc<str> = pair.end.clone().into();
                buffer.edit(
                    selection_ranges
                        .iter()
                        .map(|range| (range.clone(), pair_end.clone())),
                    cx,
                );
                snapshot = buffer.snapshot(cx);

                new_selections = Some(
                    resolve_multiple::<usize, _>(left_biased_selections.iter(), &snapshot)
                        .collect::<Vec<_>>(),
                );

                if pair.end.len() == 1 {
                    let mut delta = 0;
                    bracket_pair_state = Some(BracketPairState {
                        ranges: selections
                            .iter()
                            .map(move |selection| {
                                let offset = selection.start + delta;
                                delta += 1;
                                snapshot.anchor_before(offset)..snapshot.anchor_after(offset)
                            })
                            .collect(),
                        pair,
                    });
                }
            }
        });

        if let Some(new_selections) = new_selections {
            self.change_selections(None, cx, |s| {
                s.select(new_selections);
            });
        }
        if let Some(bracket_pair_state) = bracket_pair_state {
            self.autoclose_stack.push(bracket_pair_state);
        }
    }

    fn skip_autoclose_end(&mut self, text: &str, cx: &mut ViewContext<Self>) -> bool {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let old_selections = self.selections.all::<usize>(cx);
        let autoclose_pair = if let Some(autoclose_pair) = self.autoclose_stack.last() {
            autoclose_pair
        } else {
            return false;
        };
        if text != autoclose_pair.pair.end {
            return false;
        }

        debug_assert_eq!(old_selections.len(), autoclose_pair.ranges.len());

        if old_selections
            .iter()
            .zip(autoclose_pair.ranges.iter().map(|r| r.to_offset(&buffer)))
            .all(|(selection, autoclose_range)| {
                let autoclose_range_end = autoclose_range.end.to_offset(&buffer);
                selection.is_empty() && selection.start == autoclose_range_end
            })
        {
            let new_selections = old_selections
                .into_iter()
                .map(|selection| {
                    let cursor = selection.start + 1;
                    Selection {
                        id: selection.id,
                        start: cursor,
                        end: cursor,
                        reversed: false,
                        goal: SelectionGoal::None,
                    }
                })
                .collect();
            self.autoclose_stack.pop();
            self.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.select(new_selections);
            });
            true
        } else {
            false
        }
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
            project.completions(&buffer, buffer_position.clone(), cx)
        });

        let id = post_inc(&mut self.next_completion_id);
        let task = cx.spawn_weak(|this, mut cx| {
            async move {
                let completions = completions.await?;
                if completions.is_empty() {
                    return Ok(());
                }

                let mut menu = CompletionsMenu {
                    id,
                    initial_position: position,
                    match_candidates: completions
                        .iter()
                        .enumerate()
                        .map(|(id, completion)| {
                            StringMatchCandidate::new(
                                id,
                                completion.label.text[completion.label.filter_range.clone()].into(),
                            )
                        })
                        .collect(),
                    buffer,
                    completions: completions.into(),
                    matches: Vec::new().into(),
                    selected_item: 0,
                    list: Default::default(),
                };

                menu.filter(query.as_deref(), cx.background()).await;

                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        match this.context_menu.as_ref() {
                            None => {}
                            Some(ContextMenu::Completions(prev_menu)) => {
                                if prev_menu.id > menu.id {
                                    return;
                                }
                            }
                            _ => return,
                        }

                        this.completion_tasks.retain(|(id, _)| *id > menu.id);
                        if this.focused {
                            this.show_context_menu(ContextMenu::Completions(menu), cx);
                        }

                        cx.notify();
                    });
                }
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
        let old_range = completion.old_range.to_offset(&buffer);
        let old_text = buffer.text_for_range(old_range.clone()).collect::<String>();

        let newest_selection = self.selections.newest_anchor();
        if newest_selection.start.buffer_id != Some(buffer_handle.id()) {
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
        let mut ranges = Vec::new();
        for selection in &selections {
            if snapshot.contains_str_at(selection.start.saturating_sub(lookbehind), &old_text) {
                let start = selection.start.saturating_sub(lookbehind);
                let end = selection.end + lookahead;
                ranges.push(start + common_prefix_len..end);
            } else {
                common_prefix_len = 0;
                ranges.clear();
                ranges.extend(selections.iter().map(|s| {
                    if s.id == newest_selection.id {
                        old_range.clone()
                    } else {
                        s.start..s.end
                    }
                }));
                break;
            }
        }
        let text = &text[common_prefix_len..];

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
                    buffer
                        .edit_with_autoindent(ranges.iter().map(|range| (range.clone(), text)), cx);
                });
            }
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
        cx.spawn_weak(|this, mut cx| async move {
            while let Some(prev_task) = task {
                prev_task.await;
                task = this
                    .upgrade(&cx)
                    .and_then(|this| this.update(&mut cx, |this, _| this.code_actions_task.take()));
            }

            if let Some(this) = this.upgrade(&cx) {
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
                })
            }
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
        Some(cx.spawn(|workspace, cx| async move {
            let project_transaction = apply_code_actions.await?;
            Self::open_project_transaction(editor, workspace, project_transaction, title, cx).await
        }))
    }

    async fn open_project_transaction(
        this: ViewHandle<Editor>,
        workspace: ViewHandle<Workspace>,
        transaction: ProjectTransaction,
        title: String,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let replica_id = this.read_with(&cx, |this, cx| this.replica_id(cx));

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
                });
                if let Some((_, excerpted_buffer, excerpt_range)) = excerpt {
                    if excerpted_buffer == *buffer {
                        let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot());
                        let excerpt_range = excerpt_range.to_offset(&snapshot);
                        if snapshot
                            .edited_ranges_for_transaction(transaction)
                            .all(|range| {
                                excerpt_range.start <= range.start && excerpt_range.end >= range.end
                            })
                        {
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
            for (buffer, transaction) in &entries {
                let snapshot = buffer.read(cx).snapshot();
                ranges_to_highlight.extend(
                    multibuffer.push_excerpts_with_context_lines(
                        buffer.clone(),
                        snapshot
                            .edited_ranges_for_transaction::<usize>(transaction)
                            .collect(),
                        1,
                        cx,
                    ),
                );
            }
            multibuffer.push_transaction(entries.iter().map(|(b, t)| (b, t)));
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
        });

        Ok(())
    }

    fn refresh_code_actions(&mut self, cx: &mut ViewContext<Self>) -> Option<()> {
        let project = self.project.as_ref()?;
        let buffer = self.buffer.read(cx);
        let newest_selection = self.selections.newest_anchor().clone();
        let (start_buffer, start) = buffer.text_anchor_for_position(newest_selection.start, cx)?;
        let (end_buffer, end) = buffer.text_anchor_for_position(newest_selection.end, cx)?;
        if start_buffer != end_buffer {
            return None;
        }

        let actions = project.update(cx, |project, cx| {
            project.code_actions(&start_buffer, start..end, cx)
        });
        self.code_actions_task = Some(cx.spawn_weak(|this, mut cx| async move {
            let actions = actions.await;
            if let Some(this) = this.upgrade(&cx) {
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
            }
        }));
        None
    }

    fn refresh_document_highlights(&mut self, cx: &mut ViewContext<Self>) -> Option<()> {
        if self.pending_rename.is_some() {
            return None;
        }

        let project = self.project.as_ref()?;
        let buffer = self.buffer.read(cx);
        let newest_selection = self.selections.newest_anchor().clone();
        let cursor_position = newest_selection.head();
        let (cursor_buffer, cursor_buffer_position) =
            buffer.text_anchor_for_position(cursor_position.clone(), cx)?;
        let (tail_buffer, _) = buffer.text_anchor_for_position(newest_selection.tail(), cx)?;
        if cursor_buffer != tail_buffer {
            return None;
        }

        let highlights = project.update(cx, |project, cx| {
            project.document_highlights(&cursor_buffer, cursor_buffer_position, cx)
        });

        self.document_highlights_task = Some(cx.spawn_weak(|this, mut cx| async move {
            let highlights = highlights.log_err().await;
            if let Some((this, highlights)) = this.upgrade(&cx).zip(highlights) {
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
                });
            }
        }));
        None
    }

    pub fn render_code_actions_indicator(
        &self,
        style: &EditorStyle,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        if self.available_code_actions.is_some() {
            enum Tag {}
            Some(
                MouseEventHandler::new::<Tag, _, _>(0, cx, |_, _| {
                    Svg::new("icons/zap.svg")
                        .with_color(style.code_actions_indicator)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .with_padding(Padding::uniform(3.))
                .on_mouse_down(|_, cx| {
                    cx.dispatch_action(ToggleCodeActions {
                        deployed_from_indicator: true,
                    });
                })
                .boxed(),
            )
        } else {
            None
        }
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
        cx: &mut RenderContext<Editor>,
    ) -> Option<(DisplayPoint, ElementBox)> {
        self.context_menu
            .as_ref()
            .map(|menu| menu.render(cursor_position, style, cx))
    }

    fn show_context_menu(&mut self, menu: ContextMenu, cx: &mut ViewContext<Self>) {
        if !matches!(menu, ContextMenu::Completions(_)) {
            self.completion_tasks.clear();
        }
        self.context_menu = Some(menu);
        cx.notify();
    }

    fn hide_context_menu(&mut self, cx: &mut ViewContext<Self>) -> Option<ContextMenu> {
        cx.notify();
        self.completion_tasks.clear();
        self.context_menu.take()
    }

    pub fn insert_snippet(
        &mut self,
        insertion_ranges: &[Range<usize>],
        snippet: Snippet,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        let tabstops = self.buffer.update(cx, |buffer, cx| {
            let snippet_text: Arc<str> = snippet.text.clone().into();
            buffer.edit_with_autoindent(
                insertion_ranges
                    .iter()
                    .cloned()
                    .map(|range| (range, snippet_text.clone())),
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
                            let mut delta = 0 as isize;
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
            self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
                self.change_selections(Some(Autoscroll::Fit), cx, |s| {
                    s.select_anchor_ranges(current_ranges.into_iter().cloned())
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
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections.all::<Point>(cx);
        if !self.selections.line_mode {
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
                        let indent_size = buffer.indent_size_for_line(line_buffer_range.start.row);
                        let language_name = buffer.language().map(|language| language.name());
                        let indent_len = match indent_size.kind {
                            IndentKind::Space => {
                                cx.global::<Settings>().tab_size(language_name.as_deref())
                            }
                            IndentKind::Tab => 1,
                        };
                        if old_head.column <= indent_size.len && old_head.column > 0 {
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

        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::Fit), cx, |s| s.select(selections));
            this.insert("", cx);
        });
    }

    pub fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::right(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                })
            });
            this.insert(&"", cx);
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
        if selections.iter().all(|s| s.is_empty()) {
            self.transact(cx, |this, cx| {
                this.buffer.update(cx, |buffer, cx| {
                    for selection in &mut selections {
                        let language_name =
                            buffer.language_at(selection.start, cx).map(|l| l.name());
                        let settings = cx.global::<Settings>();
                        let tab_size = if settings.hard_tabs(language_name.as_deref()) {
                            IndentSize::tab()
                        } else {
                            let tab_size = settings.tab_size(language_name.as_deref());
                            let char_column = buffer
                                .read(cx)
                                .text_for_range(Point::new(selection.start.row, 0)..selection.start)
                                .flat_map(str::chars)
                                .count();
                            let chars_to_next_tab_stop = tab_size - (char_column as u32 % tab_size);
                            IndentSize::spaces(chars_to_next_tab_stop)
                        };
                        buffer.edit(
                            [(
                                selection.start..selection.start,
                                tab_size.chars().collect::<String>(),
                            )],
                            cx,
                        );
                        selection.start.column += tab_size.len;
                        selection.end = selection.start;
                    }
                });
                this.change_selections(Some(Autoscroll::Fit), cx, |s| {
                    s.select(selections);
                });
            });
        } else {
            self.indent(&Indent, cx);
        }
    }

    pub fn indent(&mut self, _: &Indent, cx: &mut ViewContext<Self>) {
        let mut selections = self.selections.all::<Point>(cx);
        self.transact(cx, |this, cx| {
            let mut last_indent = None;
            this.buffer.update(cx, |buffer, cx| {
                let snapshot = buffer.snapshot(cx);
                for selection in &mut selections {
                    let language_name = buffer.language_at(selection.start, cx).map(|l| l.name());
                    let settings = &cx.global::<Settings>();
                    let tab_size = settings.tab_size(language_name.as_deref());
                    let indent_kind = if settings.hard_tabs(language_name.as_deref()) {
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
                    if let Some((last_indent_row, last_indent_len)) = last_indent {
                        if last_indent_row == selection.start.row {
                            selection.start.column += last_indent_len;
                            start_row += 1;
                        }
                        if last_indent_row == selection.end.row {
                            selection.end.column += last_indent_len;
                        }
                    }

                    for row in start_row..end_row {
                        let current_indent = snapshot.indent_size_for_line(row);
                        let indent_delta = match (current_indent.kind, indent_kind) {
                            (IndentKind::Space, IndentKind::Space) => {
                                let columns_to_next_tab_stop =
                                    tab_size - (current_indent.len % tab_size);
                                IndentSize::spaces(columns_to_next_tab_stop)
                            }
                            (IndentKind::Tab, IndentKind::Space) => IndentSize::spaces(tab_size),
                            (_, IndentKind::Tab) => IndentSize::tab(),
                        };

                        let row_start = Point::new(row, 0);
                        buffer.edit(
                            [(
                                row_start..row_start,
                                indent_delta.chars().collect::<String>(),
                            )],
                            cx,
                        );

                        // Update this selection's endpoints to reflect the indentation.
                        if row == selection.start.row {
                            selection.start.column += indent_delta.len;
                        }
                        if row == selection.end.row {
                            selection.end.column += indent_delta.len as u32;
                        }

                        last_indent = Some((row, indent_delta.len));
                    }
                }
            });

            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.select(selections);
            });
        });
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
                let language_name = buffer.language_at(selection.start, cx).map(|l| l.name());
                let tab_size = cx.global::<Settings>().tab_size(language_name.as_deref());
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
                    cx,
                );
            });
            let selections = this.selections.all::<usize>(cx);
            this.change_selections(Some(Autoscroll::Fit), cx, |s| s.select(selections));
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
            let mut edit_start = Point::new(rows.start, 0).to_offset(&buffer);
            let edit_end;
            let cursor_buffer_row;
            if buffer.max_point().row >= rows.end {
                // If there's a line after the range, delete the \n from the end of the row range
                // and position the cursor on the next line.
                edit_end = Point::new(rows.end, 0).to_offset(&buffer);
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

            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.select(new_selections);
            });
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
                if next_rows.start <= rows.end - 1 {
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
                buffer.edit(edits, cx);
            });

            this.request_autoscroll(Autoscroll::Fit, cx);
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
            contiguous_row_selections.push(selection.clone());
            let start_row = selection.start.row;
            let mut end_row = if selection.end.column > 0 || selection.is_empty() {
                display_map.next_line_boundary(selection.end).0.row + 1
            } else {
                selection.end.row
            };

            while let Some(next_selection) = selections.peek() {
                if next_selection.start.row <= end_row {
                    end_row = if next_selection.end.column > 0 || next_selection.is_empty() {
                        display_map.next_line_boundary(next_selection.end).0.row + 1
                    } else {
                        next_selection.end.row
                    };
                    contiguous_row_selections.push(selections.next().unwrap().clone());
                } else {
                    break;
                }
            }

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
                    edits.push((insertion_anchor.clone()..insertion_anchor, text));

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
            new_selections.extend(contiguous_row_selections.drain(..));
        }

        self.transact(cx, |this, cx| {
            this.unfold_ranges(unfold_ranges, true, cx);
            this.buffer.update(cx, |buffer, cx| {
                for (range, text) in edits {
                    buffer.edit([(range, text)], cx);
                }
            });
            this.fold_ranges(refold_ranges, cx);
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            contiguous_row_selections.push(selection.clone());
            let start_row = selection.start.row;
            let mut end_row = if selection.end.column > 0 || selection.is_empty() {
                display_map.next_line_boundary(selection.end).0.row + 1
            } else {
                selection.end.row
            };

            while let Some(next_selection) = selections.peek() {
                if next_selection.start.row <= end_row {
                    end_row = if next_selection.end.column > 0 || next_selection.is_empty() {
                        display_map.next_line_boundary(next_selection.end).0.row + 1
                    } else {
                        next_selection.end.row
                    };
                    contiguous_row_selections.push(selections.next().unwrap().clone());
                } else {
                    break;
                }
            }

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
                    edits.push((insertion_anchor.clone()..insertion_anchor, text));

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
            new_selections.extend(contiguous_row_selections.drain(..));
        }

        self.transact(cx, |this, cx| {
            this.unfold_ranges(unfold_ranges, true, cx);
            this.buffer.update(cx, |buffer, cx| {
                for (range, text) in edits {
                    buffer.edit([(range, text)], cx);
                }
            });
            this.fold_ranges(refold_ranges, cx);
            this.change_selections(Some(Autoscroll::Fit), cx, |s| s.select(new_selections));
        });
    }

    pub fn transpose(&mut self, _: &Transpose, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            let edits = this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            this.buffer.update(cx, |buffer, cx| buffer.edit(edits, cx));
            let selections = this.selections.all::<usize>(cx);
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            for selection in &mut selections {
                let is_entire_line = selection.is_empty() || self.selections.line_mode;
                if is_entire_line {
                    selection.start = Point::new(selection.start.row, 0);
                    selection.end = cmp::min(max_point, Point::new(selection.end.row + 1, 0));
                    selection.goal = SelectionGoal::None;
                }
                let mut len = 0;
                for chunk in buffer.text_for_range(selection.start..selection.end) {
                    text.push_str(chunk);
                    len += chunk.len();
                }
                clipboard_selections.push(ClipboardSelection {
                    len,
                    is_entire_line,
                });
            }
        }

        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            for selection in selections.iter() {
                let mut start = selection.start;
                let mut end = selection.end;
                let is_entire_line = selection.is_empty() || self.selections.line_mode;
                if is_entire_line {
                    start = Point::new(start.row, 0);
                    end = cmp::min(max_point, Point::new(end.row + 1, 0));
                }
                let mut len = 0;
                for chunk in buffer.text_for_range(start..end) {
                    text.push_str(chunk);
                    len += chunk.len();
                }
                clipboard_selections.push(ClipboardSelection {
                    len,
                    is_entire_line,
                });
            }
        }

        cx.write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            if let Some(item) = cx.as_mut().read_from_clipboard() {
                let mut clipboard_text = Cow::Borrowed(item.text());
                if let Some(mut clipboard_selections) = item.metadata::<Vec<ClipboardSelection>>() {
                    let old_selections = this.selections.all::<usize>(cx);
                    let all_selections_were_entire_line =
                        clipboard_selections.iter().all(|s| s.is_entire_line);
                    if clipboard_selections.len() != old_selections.len() {
                        let mut newline_separated_text = String::new();
                        let mut clipboard_selections = clipboard_selections.drain(..).peekable();
                        let mut ix = 0;
                        while let Some(clipboard_selection) = clipboard_selections.next() {
                            newline_separated_text
                                .push_str(&clipboard_text[ix..ix + clipboard_selection.len]);
                            ix += clipboard_selection.len;
                            if clipboard_selections.peek().is_some() {
                                newline_separated_text.push('\n');
                            }
                        }
                        clipboard_text = Cow::Owned(newline_separated_text);
                    }

                    this.buffer.update(cx, |buffer, cx| {
                        let snapshot = buffer.read(cx);
                        let mut start_offset = 0;
                        let mut edits = Vec::new();
                        let line_mode = this.selections.line_mode;
                        for (ix, selection) in old_selections.iter().enumerate() {
                            let to_insert;
                            let entire_line;
                            if let Some(clipboard_selection) = clipboard_selections.get(ix) {
                                let end_offset = start_offset + clipboard_selection.len;
                                to_insert = &clipboard_text[start_offset..end_offset];
                                entire_line = clipboard_selection.is_entire_line;
                                start_offset = end_offset;
                            } else {
                                to_insert = clipboard_text.as_str();
                                entire_line = all_selections_were_entire_line;
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
                        }
                        drop(snapshot);
                        buffer.edit_with_autoindent(edits, cx);
                    });

                    let selections = this.selections.all::<usize>(cx);
                    this.change_selections(Some(Autoscroll::Fit), cx, |s| s.select(selections));
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
            self.request_autoscroll(Autoscroll::Fit, cx);
            cx.emit(Event::Edited);
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
            self.request_autoscroll(Autoscroll::Fit, cx);
            cx.emit(Event::Edited);
        }
    }

    pub fn finalize_last_transaction(&mut self, cx: &mut ViewContext<Self>) {
        self.buffer
            .update(cx, |buffer, cx| buffer.finalize_last_transaction(cx));
    }

    pub fn move_left(&mut self, _: &MoveLeft, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_heads_with(|map, head, _| (movement::left(map, head), SelectionGoal::None));
        })
    }

    pub fn move_right(&mut self, _: &MoveRight, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_heads_with(|map, head, _| (movement::right(map, head), SelectionGoal::None));
        })
    }

    pub fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if let Some(context_menu) = self.context_menu.as_mut() {
            if context_menu.select_prev(cx) {
                return;
            }
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::up(&map, selection.start, selection.goal, false);
                selection.collapse_to(cursor, goal);
            });
        })
    }

    pub fn select_up(&mut self, _: &SelectUp, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_heads_with(|map, head, goal| movement::up(map, head, goal, false))
        })
    }

    pub fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        self.take_rename(true, cx);

        if let Some(context_menu) = self.context_menu.as_mut() {
            if context_menu.select_next(cx) {
                return;
            }
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::down(&map, selection.end, selection.goal, false);
                selection.collapse_to(cursor, goal);
            });
        });
    }

    pub fn select_down(&mut self, _: &SelectDown, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_heads_with(|map, head, goal| movement::down(map, head, goal, false))
        });
    }

    pub fn move_to_previous_word_start(
        &mut self,
        _: &MoveToPreviousWordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (movement::next_subword_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn select_to_next_word_end(&mut self, _: &SelectToNextWordEnd, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (movement::next_subword_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn delete_to_next_word_end(&mut self, _: &DeleteToNextWordEnd, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (
                    movement::line_beginning(map, head, true),
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::line_beginning(map, head, action.stop_at_soft_wraps),
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
            this.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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

    pub fn move_to_beginning(&mut self, _: &MoveToBeginning, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.select_ranges(vec![0..0]);
        });
    }

    pub fn select_to_beginning(&mut self, _: &SelectToBeginning, cx: &mut ViewContext<Self>) {
        let mut selection = self.selections.last::<Point>(cx);
        selection.set_head(Point::zero(), SelectionGoal::None);

        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.select(vec![selection]);
        });
    }

    pub fn move_to_end(&mut self, _: &MoveToEnd, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let cursor = self.buffer.read(cx).read(cx).len();
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        &self,
        position: Anchor,
        new_position: Option<Point>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(nav_history) = &self.nav_history {
            let buffer = self.buffer.read(cx).read(cx);
            let point = position.to_point(&buffer);
            let scroll_top_row = self.scroll_top_anchor.to_point(&buffer).row;
            drop(buffer);

            if let Some(new_position) = new_position {
                let row_delta = (new_position.row as i64 - point.row as i64).abs();
                if row_delta < MIN_NAVIGATION_HISTORY_ROW_DELTA {
                    return;
                }
            }

            nav_history.push(Some(NavigationData {
                cursor_anchor: position,
                cursor_position: point,
                scroll_position: self.scroll_position,
                scroll_top_anchor: self.scroll_top_anchor.clone(),
                scroll_top_row,
            }));
        }
    }

    pub fn select_to_end(&mut self, _: &SelectToEnd, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selection = self.selections.first::<usize>(cx);
        selection.set_head(buffer.len(), SelectionGoal::None);
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.select(vec![selection]);
        });
    }

    pub fn select_all(&mut self, _: &SelectAll, cx: &mut ViewContext<Self>) {
        let end = self.buffer.read(cx).read(cx).len();
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        self.unfold_ranges(to_unfold, true, cx);
        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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

        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
                    self.unfold_ranges([next_selected_range.clone()], false, cx);
                    self.change_selections(Some(Autoscroll::Newest), cx, |s| {
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
                self.unfold_ranges([selection.start..selection.end], false, cx);
                self.change_selections(Some(Autoscroll::Newest), cx, |s| {
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

    pub fn toggle_comments(&mut self, _: &ToggleComments, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            let mut selections = this.selections.all::<Point>(cx);
            let mut all_selection_lines_are_comments = true;
            let mut edit_ranges = Vec::new();
            let mut last_toggled_row = None;
            this.buffer.update(cx, |buffer, cx| {
                // TODO: Handle selections that cross excerpts
                for selection in &mut selections {
                    // Get the line comment prefix. Split its trailing whitespace into a separate string,
                    // as that portion won't be used for detecting if a line is a comment.
                    let full_comment_prefix: Arc<str> = if let Some(prefix) = buffer
                        .language_at(selection.start, cx)
                        .and_then(|l| l.line_comment_prefix())
                    {
                        prefix.into()
                    } else {
                        return;
                    };
                    let comment_prefix = full_comment_prefix.trim_end_matches(' ');
                    let comment_prefix_whitespace = &full_comment_prefix[comment_prefix.len()..];
                    edit_ranges.clear();
                    let snapshot = buffer.snapshot(cx);

                    let end_row =
                        if selection.end.row > selection.start.row && selection.end.column == 0 {
                            selection.end.row
                        } else {
                            selection.end.row + 1
                        };

                    for row in selection.start.row..end_row {
                        // If multiple selections contain a given row, avoid processing that
                        // row more than once.
                        if last_toggled_row == Some(row) {
                            continue;
                        } else {
                            last_toggled_row = Some(row);
                        }

                        if snapshot.is_line_blank(row) {
                            continue;
                        }

                        let start = Point::new(row, snapshot.indent_size_for_line(row).len);
                        let mut line_bytes = snapshot
                            .bytes_in_range(start..snapshot.max_point())
                            .flatten()
                            .copied();

                        // If this line currently begins with the line comment prefix, then record
                        // the range containing the prefix.
                        if all_selection_lines_are_comments
                            && line_bytes
                                .by_ref()
                                .take(comment_prefix.len())
                                .eq(comment_prefix.bytes())
                        {
                            // Include any whitespace that matches the comment prefix.
                            let matching_whitespace_len = line_bytes
                                .zip(comment_prefix_whitespace.bytes())
                                .take_while(|(a, b)| a == b)
                                .count()
                                as u32;
                            let end = Point::new(
                                row,
                                start.column
                                    + comment_prefix.len() as u32
                                    + matching_whitespace_len,
                            );
                            edit_ranges.push(start..end);
                        }
                        // If this line does not begin with the line comment prefix, then record
                        // the position where the prefix should be inserted.
                        else {
                            all_selection_lines_are_comments = false;
                            edit_ranges.push(start..start);
                        }
                    }

                    if !edit_ranges.is_empty() {
                        if all_selection_lines_are_comments {
                            let empty_str: Arc<str> = "".into();
                            buffer.edit(
                                edit_ranges
                                    .iter()
                                    .cloned()
                                    .map(|range| (range, empty_str.clone())),
                                cx,
                            );
                        } else {
                            let min_column =
                                edit_ranges.iter().map(|r| r.start.column).min().unwrap();
                            let edits = edit_ranges.iter().map(|range| {
                                let position = Point::new(range.start.row, min_column);
                                (position..position, full_comment_prefix.clone())
                            });
                            buffer.edit(edits, cx);
                        }
                    }
                }
            });

            let selections = this.selections.all::<usize>(cx);
            this.change_selections(Some(Autoscroll::Fit), cx, |s| s.select(selections));
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
            self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
            self.change_selections(Some(Autoscroll::Fit), cx, |s| {
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
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selections = self.selections.all::<usize>(cx);
        for selection in &mut selections {
            if let Some((open_range, close_range)) =
                buffer.enclosing_bracket_ranges(selection.start..selection.end)
            {
                let close_range = close_range.to_inclusive();
                let destination = if close_range.contains(&selection.start)
                    && close_range.contains(&selection.end)
                {
                    open_range.end
                } else {
                    *close_range.start()
                };
                selection.start = destination;
                selection.end = destination;
            }
        }

        self.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.select(selections);
        });
    }

    pub fn undo_selection(&mut self, _: &UndoSelection, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        self.selection_history.mode = SelectionHistoryMode::Undoing;
        if let Some(entry) = self.selection_history.undo_stack.pop_back() {
            self.change_selections(None, cx, |s| s.select_anchors(entry.selections.to_vec()));
            self.select_next_state = entry.select_next_state;
            self.add_selections_state = entry.add_selections_state;
            self.request_autoscroll(Autoscroll::Newest, cx);
        }
        self.selection_history.mode = SelectionHistoryMode::Normal;
    }

    pub fn redo_selection(&mut self, _: &RedoSelection, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        self.selection_history.mode = SelectionHistoryMode::Redoing;
        if let Some(entry) = self.selection_history.redo_stack.pop_back() {
            self.change_selections(None, cx, |s| s.select_anchors(entry.selections.to_vec()));
            self.select_next_state = entry.select_next_state;
            self.add_selections_state = entry.add_selections_state;
            self.request_autoscroll(Autoscroll::Newest, cx);
        }
        self.selection_history.mode = SelectionHistoryMode::Normal;
    }

    fn go_to_next_diagnostic(&mut self, _: &GoToNextDiagnostic, cx: &mut ViewContext<Self>) {
        self.go_to_diagnostic(Direction::Next, cx)
    }

    fn go_to_prev_diagnostic(&mut self, _: &GoToPrevDiagnostic, cx: &mut ViewContext<Self>) {
        self.go_to_diagnostic(Direction::Prev, cx)
    }

    pub fn go_to_diagnostic(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let selection = self.selections.newest::<usize>(cx);
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
                self.activate_diagnostics(group_id, cx);
                self.change_selections(Some(Autoscroll::Center), cx, |s| {
                    s.select(vec![Selection {
                        id: selection.id,
                        start: primary_range.start,
                        end: primary_range.start,
                        reversed: false,
                        goal: SelectionGoal::None,
                    }]);
                });
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
                } else {
                    if search_start == 0 {
                        break;
                    } else {
                        search_start = 0;
                    }
                }
            }
        }
    }

    pub fn go_to_definition(
        workspace: &mut Workspace,
        _: &GoToDefinition,
        cx: &mut ViewContext<Workspace>,
    ) {
        let active_item = workspace.active_item(cx);
        let editor_handle = if let Some(editor) = active_item
            .as_ref()
            .and_then(|item| item.act_as::<Self>(cx))
        {
            editor
        } else {
            return;
        };

        let editor = editor_handle.read(cx);
        let buffer = editor.buffer.read(cx);
        let head = editor.selections.newest::<usize>(cx).head();
        let (buffer, head) = if let Some(text_anchor) = buffer.text_anchor_for_position(head, cx) {
            text_anchor
        } else {
            return;
        };

        let project = workspace.project().clone();
        let definitions = project.update(cx, |project, cx| project.definition(&buffer, head, cx));
        cx.spawn(|workspace, mut cx| async move {
            let definitions = definitions.await?;
            workspace.update(&mut cx, |workspace, cx| {
                let nav_history = workspace.active_pane().read(cx).nav_history().clone();
                for definition in definitions {
                    let range = definition.range.to_offset(definition.buffer.read(cx));

                    let target_editor_handle = workspace.open_project_item(definition.buffer, cx);
                    target_editor_handle.update(cx, |target_editor, cx| {
                        // When selecting a definition in a different buffer, disable the nav history
                        // to avoid creating a history entry at the previous cursor location.
                        if editor_handle != target_editor_handle {
                            nav_history.borrow_mut().disable();
                        }
                        target_editor.change_selections(Some(Autoscroll::Center), cx, |s| {
                            s.select_ranges([range]);
                        });

                        nav_history.borrow_mut().enable();
                    });
                }
            });

            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx);
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
        Some(cx.spawn(|workspace, mut cx| async move {
            let mut locations = references.await?;
            if locations.is_empty() {
                return Ok(());
            }

            locations.sort_by_key(|location| location.buffer.id());
            let mut locations = locations.into_iter().peekable();
            let mut ranges_to_highlight = Vec::new();

            let excerpt_buffer = cx.add_model(|cx| {
                let mut symbol_name = None;
                let mut multibuffer = MultiBuffer::new(replica_id);
                while let Some(location) = locations.next() {
                    let buffer = location.buffer.read(cx);
                    let mut ranges_for_buffer = Vec::new();
                    let range = location.range.to_offset(buffer);
                    ranges_for_buffer.push(range.clone());
                    if symbol_name.is_none() {
                        symbol_name = Some(buffer.text_for_range(range).collect::<String>());
                    }

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
                    ));
                }
                multibuffer.with_title(format!("References to `{}`", symbol_name.unwrap()))
            });

            workspace.update(&mut cx, |workspace, cx| {
                let editor =
                    cx.add_view(|cx| Editor::for_multibuffer(excerpt_buffer, Some(project), cx));
                editor.update(cx, |editor, cx| {
                    editor.highlight_background::<Self>(
                        ranges_to_highlight,
                        |theme| theme.editor.highlighted_line_background,
                        cx,
                    );
                });
                workspace.add_item(Box::new(editor), cx);
            });

            Ok(())
        }))
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
                this.read_with(&cx, |this, cx| {
                    let buffer = this.buffer.read(cx).snapshot(cx);
                    let mut buffer_highlights = this
                        .document_highlights_for_position(selection.head(), &buffer)
                        .filter(|highlight| {
                            highlight.start.excerpt_id() == selection.head().excerpt_id()
                                && highlight.end.excerpt_id() == selection.head().excerpt_id()
                        });
                    buffer_highlights
                        .next()
                        .map(|highlight| highlight.start.text_anchor..highlight.end.text_anchor)
                })
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
                        editor
                            .buffer
                            .update(cx, |buffer, cx| buffer.edit([(0..0, old_name.clone())], cx));
                        editor.select_all(&SelectAll, cx);
                        editor
                    });

                    let ranges = this
                        .clear_background_highlights::<DocumentHighlightWrite>(cx)
                        .into_iter()
                        .flat_map(|(_, ranges)| ranges)
                        .chain(
                            this.clear_background_highlights::<DocumentHighlightRead>(cx)
                                .into_iter()
                                .flat_map(|(_, ranges)| ranges),
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
                            position: range.start.clone(),
                            height: 1,
                            render: Arc::new({
                                let editor = rename_editor.clone();
                                move |cx: &mut BlockContext| {
                                    ChildView::new(editor.clone())
                                        .contained()
                                        .with_padding_left(cx.anchor_x)
                                        .boxed()
                                }
                            }),
                            disposition: BlockDisposition::Below,
                        }],
                        cx,
                    )[0];
                    this.pending_rename = Some(RenameState {
                        range,
                        old_name,
                        editor: rename_editor,
                        block_id,
                    });
                });
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
            project.perform_rename(
                buffer.clone(),
                range.start.clone(),
                new_name.clone(),
                true,
                cx,
            )
        });

        Some(cx.spawn(|workspace, mut cx| async move {
            let project_transaction = rename.await?;
            Self::open_project_transaction(
                editor.clone(),
                workspace,
                project_transaction,
                format!("Rename: {} → {}", old_name, new_name),
                cx.clone(),
            )
            .await?;

            editor.update(&mut cx, |editor, cx| {
                editor.refresh_document_highlights(cx);
            });
            Ok(())
        }))
    }

    fn take_rename(
        &mut self,
        moving_cursor: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<RenameState> {
        let rename = self.pending_rename.take()?;
        self.remove_blocks([rename.block_id].into_iter().collect(), cx);
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
        }

        Some(rename)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn pending_rename(&self) -> Option<&RenameState> {
        self.pending_rename.as_ref()
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

    fn activate_diagnostics(&mut self, group_id: usize, cx: &mut ViewContext<Self>) {
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
            let primary_range = primary_range.unwrap();
            let primary_message = primary_message.unwrap();
            let primary_range =
                buffer.anchor_after(primary_range.start)..buffer.anchor_before(primary_range.end);

            let blocks = display_map
                .insert_blocks(
                    diagnostic_group.iter().map(|entry| {
                        let diagnostic = entry.diagnostic.clone();
                        let message_height = diagnostic.message.lines().count() as u8;
                        BlockProperties {
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
        cx: &mut ViewContext<Self>,
    ) {
        let old_cursor_position = self.selections.newest_anchor().head();
        self.selections.change_with(cx, |s| {
            s.select_anchors(selections);
        });
        self.selections_did_change(false, &old_cursor_position, cx);
    }

    fn push_to_selection_history(&mut self) {
        self.selection_history.push(SelectionHistoryEntry {
            selections: self.selections.disjoint_anchors().clone(),
            select_next_state: self.select_next_state.clone(),
            add_selections_state: self.add_selections_state.clone(),
        });
    }

    pub fn request_autoscroll(&mut self, autoscroll: Autoscroll, cx: &mut ViewContext<Self>) {
        self.autoscroll_request = Some((autoscroll, true));
        cx.notify();
    }

    fn request_autoscroll_remotely(&mut self, autoscroll: Autoscroll, cx: &mut ViewContext<Self>) {
        self.autoscroll_request = Some((autoscroll, false));
        cx.notify();
    }

    pub fn transact(
        &mut self,
        cx: &mut ViewContext<Self>,
        update: impl FnOnce(&mut Self, &mut ViewContext<Self>),
    ) {
        self.start_transaction_at(Instant::now(), cx);
        update(self, cx);
        self.end_transaction_at(Instant::now(), cx);
    }

    fn start_transaction_at(&mut self, now: Instant, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        if let Some(tx_id) = self
            .buffer
            .update(cx, |buffer, cx| buffer.start_transaction_at(now, cx))
        {
            self.selection_history
                .insert_transaction(tx_id, self.selections.disjoint_anchors().clone());
        }
    }

    fn end_transaction_at(&mut self, now: Instant, cx: &mut ViewContext<Self>) {
        if let Some(tx_id) = self
            .buffer
            .update(cx, |buffer, cx| buffer.end_transaction_at(now, cx))
        {
            if let Some((_, end_selections)) = self.selection_history.transaction_mut(tx_id) {
                *end_selections = Some(self.selections.disjoint_anchors().clone());
            } else {
                log::error!("unexpectedly ended a transaction that wasn't started by this editor");
            }

            cx.emit(Event::Edited);
        }
    }

    pub fn page_up(&mut self, _: &PageUp, _: &mut ViewContext<Self>) {
        log::info!("Editor::page_up");
    }

    pub fn page_down(&mut self, _: &PageDown, _: &mut ViewContext<Self>) {
        log::info!("Editor::page_down");
    }

    pub fn fold(&mut self, _: &Fold, cx: &mut ViewContext<Self>) {
        let mut fold_ranges = Vec::new();

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);
        for selection in selections {
            let range = selection.display_range(&display_map).sorted();
            let buffer_start_row = range.start.to_point(&display_map).row;

            for row in (0..=range.end.row()).rev() {
                if self.is_line_foldable(&display_map, row) && !display_map.is_line_folded(row) {
                    let fold_range = self.foldable_range_for_line(&display_map, row);
                    if fold_range.end.row >= buffer_start_row {
                        fold_ranges.push(fold_range);
                        if row <= range.start.row() {
                            break;
                        }
                    }
                }
            }
        }

        self.fold_ranges(fold_ranges, cx);
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
        self.unfold_ranges(ranges, true, cx);
    }

    fn is_line_foldable(&self, display_map: &DisplaySnapshot, display_row: u32) -> bool {
        let max_point = display_map.max_point();
        if display_row >= max_point.row() {
            false
        } else {
            let (start_indent, is_blank) = display_map.line_indent(display_row);
            if is_blank {
                false
            } else {
                for display_row in display_row + 1..=max_point.row() {
                    let (indent, is_blank) = display_map.line_indent(display_row);
                    if !is_blank {
                        return indent > start_indent;
                    }
                }
                false
            }
        }
    }

    fn foldable_range_for_line(
        &self,
        display_map: &DisplaySnapshot,
        start_row: u32,
    ) -> Range<Point> {
        let max_point = display_map.max_point();

        let (start_indent, _) = display_map.line_indent(start_row);
        let start = DisplayPoint::new(start_row, display_map.line_len(start_row));
        let mut end = None;
        for row in start_row + 1..=max_point.row() {
            let (indent, is_blank) = display_map.line_indent(row);
            if !is_blank && indent <= start_indent {
                end = Some(DisplayPoint::new(row - 1, display_map.line_len(row - 1)));
                break;
            }
        }

        let end = end.unwrap_or(max_point);
        return start.to_point(display_map)..end.to_point(display_map);
    }

    pub fn fold_selected_ranges(&mut self, _: &FoldSelectedRanges, cx: &mut ViewContext<Self>) {
        let selections = self.selections.all::<Point>(cx);
        let ranges = selections.into_iter().map(|s| s.start..s.end);
        self.fold_ranges(ranges, cx);
    }

    pub fn fold_ranges<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &mut ViewContext<Self>,
    ) {
        let mut ranges = ranges.into_iter().peekable();
        if ranges.peek().is_some() {
            self.display_map.update(cx, |map, cx| map.fold(ranges, cx));
            self.request_autoscroll(Autoscroll::Fit, cx);
            cx.notify();
        }
    }

    pub fn unfold_ranges<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        inclusive: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let mut ranges = ranges.into_iter().peekable();
        if ranges.peek().is_some() {
            self.display_map
                .update(cx, |map, cx| map.unfold(ranges, inclusive, cx));
            self.request_autoscroll(Autoscroll::Fit, cx);
            cx.notify();
        }
    }

    pub fn insert_blocks(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<Anchor>>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<BlockId> {
        let blocks = self
            .display_map
            .update(cx, |display_map, cx| display_map.insert_blocks(blocks, cx));
        self.request_autoscroll(Autoscroll::Fit, cx);
        blocks
    }

    pub fn replace_blocks(
        &mut self,
        blocks: HashMap<BlockId, RenderBlock>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map
            .update(cx, |display_map, _| display_map.replace_blocks(blocks));
        self.request_autoscroll(Autoscroll::Fit, cx);
    }

    pub fn remove_blocks(&mut self, block_ids: HashSet<BlockId>, cx: &mut ViewContext<Self>) {
        self.display_map.update(cx, |display_map, cx| {
            display_map.remove_blocks(block_ids, cx)
        });
    }

    pub fn longest_row(&self, cx: &mut MutableAppContext) -> u32 {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .longest_row()
    }

    pub fn max_point(&self, cx: &mut MutableAppContext) -> DisplayPoint {
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

    pub fn display_text(&self, cx: &mut MutableAppContext) -> String {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .text()
    }

    pub fn soft_wrap_mode(&self, cx: &AppContext) -> SoftWrap {
        let language_name = self
            .buffer
            .read(cx)
            .as_singleton()
            .and_then(|singleton_buffer| singleton_buffer.read(cx).language())
            .map(|l| l.name());

        let settings = cx.global::<Settings>();
        let mode = self
            .soft_wrap_mode_override
            .unwrap_or_else(|| settings.soft_wrap(language_name.as_deref()));
        match mode {
            settings::SoftWrap::None => SoftWrap::None,
            settings::SoftWrap::EditorWidth => SoftWrap::EditorWidth,
            settings::SoftWrap::PreferredLineLength => {
                SoftWrap::Column(settings.preferred_line_length(language_name.as_deref()))
            }
        }
    }

    pub fn set_soft_wrap_mode(&mut self, mode: settings::SoftWrap, cx: &mut ViewContext<Self>) {
        self.soft_wrap_mode_override = Some(mode);
        cx.notify();
    }

    pub fn set_wrap_width(&self, width: Option<f32>, cx: &mut MutableAppContext) -> bool {
        self.display_map
            .update(cx, |map, cx| map.set_wrap_width(width, cx))
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
        self.background_highlights
            .insert(TypeId::of::<T>(), (color_fetcher, ranges));
        cx.notify();
    }

    pub fn clear_background_highlights<T: 'static>(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<(fn(&Theme) -> Color, Vec<Range<Anchor>>)> {
        cx.notify();
        self.background_highlights.remove(&TypeId::of::<T>())
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
        let theme = cx.global::<Settings>().theme.as_ref();
        self.background_highlights_in_range(start..end, &snapshot, theme)
    }

    fn document_highlights_for_position<'a>(
        &'a self,
        position: Anchor,
        buffer: &'a MultiBufferSnapshot,
    ) -> impl 'a + Iterator<Item = &Range<Anchor>> {
        let read_highlights = self
            .background_highlights
            .get(&TypeId::of::<DocumentHighlightRead>())
            .map(|h| &h.1);
        let write_highlights = self
            .background_highlights
            .get(&TypeId::of::<DocumentHighlightRead>())
            .map(|h| &h.1);
        let left_position = position.bias_left(buffer);
        let right_position = position.bias_right(buffer);
        read_highlights
            .into_iter()
            .chain(write_highlights)
            .flat_map(move |ranges| {
                let start_ix = match ranges.binary_search_by(|probe| {
                    let cmp = probe.end.cmp(&left_position, &buffer);
                    if cmp.is_ge() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }) {
                    Ok(i) | Err(i) => i,
                };

                let right_position = right_position.clone();
                ranges[start_ix..]
                    .iter()
                    .take_while(move |range| range.start.cmp(&right_position, &buffer).is_le())
            })
    }

    pub fn background_highlights_in_range(
        &self,
        search_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
        theme: &Theme,
    ) -> Vec<(Range<DisplayPoint>, Color)> {
        let mut results = Vec::new();
        let buffer = &display_snapshot.buffer_snapshot;
        for (color_fetcher, ranges) in self.background_highlights.values() {
            let color = color_fetcher(theme);
            let start_ix = match ranges.binary_search_by(|probe| {
                let cmp = probe.end.cmp(&search_range.start, &buffer);
                if cmp.is_gt() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }) {
                Ok(i) | Err(i) => i,
            };
            for range in &ranges[start_ix..] {
                if range.start.cmp(&search_range.end, &buffer).is_ge() {
                    break;
                }
                let start = range
                    .start
                    .to_point(buffer)
                    .to_display_point(display_snapshot);
                let end = range
                    .end
                    .to_point(buffer)
                    .to_display_point(display_snapshot);
                results.push((start..end, color))
            }
        }
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

    pub fn clear_text_highlights<T: 'static>(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<Arc<(HighlightStyle, Vec<Range<Anchor>>)>> {
        cx.notify();
        self.display_map
            .update(cx, |map, _| map.clear_text_highlights(TypeId::of::<T>()))
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    fn pause_cursor_blinking(&mut self, cx: &mut ViewContext<Self>) {
        if !self.focused {
            return;
        }

        self.show_local_cursors = true;
        cx.notify();

        let epoch = self.next_blink_epoch();
        cx.spawn(|this, mut cx| {
            let this = this.downgrade();
            async move {
                Timer::after(CURSOR_BLINK_INTERVAL).await;
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| this.resume_cursor_blinking(epoch, cx))
                }
            }
        })
        .detach();
    }

    fn resume_cursor_blinking(&mut self, epoch: usize, cx: &mut ViewContext<Self>) {
        if epoch == self.blink_epoch {
            self.blinking_paused = false;
            self.blink_cursors(epoch, cx);
        }
    }

    fn blink_cursors(&mut self, epoch: usize, cx: &mut ViewContext<Self>) {
        if epoch == self.blink_epoch && self.focused && !self.blinking_paused {
            self.show_local_cursors = !self.show_local_cursors;
            cx.notify();

            let epoch = self.next_blink_epoch();
            cx.spawn(|this, mut cx| {
                let this = this.downgrade();
                async move {
                    Timer::after(CURSOR_BLINK_INTERVAL).await;
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(&mut cx, |this, cx| this.blink_cursors(epoch, cx));
                    }
                }
            })
            .detach();
        }
    }

    pub fn show_local_cursors(&self) -> bool {
        self.show_local_cursors && self.focused
    }

    fn on_buffer_changed(&mut self, _: ModelHandle<MultiBuffer>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn on_buffer_event(
        &mut self,
        _: ModelHandle<MultiBuffer>,
        event: &language::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            language::Event::Edited => {
                self.refresh_active_diagnostics(cx);
                self.refresh_code_actions(cx);
                cx.emit(Event::BufferEdited);
            }
            language::Event::Reparsed => cx.emit(Event::Reparsed),
            language::Event::Dirtied => cx.emit(Event::Dirtied),
            language::Event::Saved => cx.emit(Event::Saved),
            language::Event::FileHandleChanged => cx.emit(Event::TitleChanged),
            language::Event::Reloaded => cx.emit(Event::TitleChanged),
            language::Event::Closed => cx.emit(Event::Closed),
            language::Event::DiagnosticsUpdated => {
                self.refresh_active_diagnostics(cx);
            }
            _ => {}
        }
    }

    fn on_display_map_changed(&mut self, _: ModelHandle<DisplayMap>, cx: &mut ViewContext<Self>) {
        cx.notify();
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
            for (buffer, mut range) in
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
        let nav_history = workspace.active_pane().read(cx).nav_history().clone();
        nav_history.borrow_mut().disable();

        // We defer the pane interaction because we ourselves are a workspace item
        // and activating a new item causes the pane to call a method on us reentrantly,
        // which panics if we're on the stack.
        cx.defer(move |workspace, cx| {
            for (buffer, ranges) in new_selections_by_buffer.into_iter() {
                let editor = workspace.open_project_item::<Self>(buffer, cx);
                editor.update(cx, |editor, cx| {
                    editor.change_selections(Some(Autoscroll::Newest), cx, |s| {
                        s.select_ranges(ranges);
                    });
                });
            }

            nav_history.borrow_mut().enable();
        });
    }

    fn jump(workspace: &mut Workspace, action: &Jump, cx: &mut ViewContext<Workspace>) {
        let editor = workspace.open_path(action.path.clone(), true, cx);
        let position = action.position;
        let anchor = action.anchor;
        cx.spawn_weak(|_, mut cx| async move {
            let editor = editor.await.log_err()?.downcast::<Editor>()?;
            editor.update(&mut cx, |editor, cx| {
                let buffer = editor.buffer().read(cx).as_singleton()?;
                let buffer = buffer.read(cx);
                let cursor = if buffer.can_resolve(&anchor) {
                    language::ToPoint::to_point(&anchor, buffer)
                } else {
                    buffer.clip_point(position, Bias::Left)
                };

                let nav_history = editor.nav_history.take();
                editor.change_selections(Some(Autoscroll::Newest), cx, |s| {
                    s.select_ranges([cursor..cursor]);
                });
                editor.nav_history = nav_history;

                Some(())
            })?;
            Some(())
        })
        .detach()
    }
}

impl EditorSnapshot {
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn placeholder_text(&self) -> Option<&Arc<str>> {
        self.placeholder_text.as_ref()
    }

    pub fn scroll_position(&self) -> Vector2F {
        compute_scroll_position(
            &self.display_snapshot,
            self.scroll_position,
            &self.scroll_top_anchor,
        )
    }
}

impl Deref for EditorSnapshot {
    type Target = DisplaySnapshot;

    fn deref(&self) -> &Self::Target {
        &self.display_snapshot
    }
}

fn compute_scroll_position(
    snapshot: &DisplaySnapshot,
    mut scroll_position: Vector2F,
    scroll_top_anchor: &Anchor,
) -> Vector2F {
    if *scroll_top_anchor != Anchor::min() {
        let scroll_top = scroll_top_anchor.to_display_point(snapshot).row() as f32;
        scroll_position.set_y(scroll_top + scroll_position.y());
    } else {
        scroll_position.set_y(0.);
    }
    scroll_position
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Activate,
    BufferEdited,
    Edited,
    Reparsed,
    Blurred,
    Dirtied,
    Saved,
    TitleChanged,
    SelectionsChanged { local: bool },
    ScrollPositionChanged { local: bool },
    Closed,
}

pub struct EditorFocused(pub ViewHandle<Editor>);
pub struct EditorBlurred(pub ViewHandle<Editor>);
pub struct EditorReleased(pub WeakViewHandle<Editor>);

impl Entity for Editor {
    type Event = Event;

    fn release(&mut self, cx: &mut MutableAppContext) {
        cx.emit_global(EditorReleased(self.handle.clone()));
    }
}

impl View for Editor {
    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let style = self.style(cx);
        let font_changed = self.display_map.update(cx, |map, cx| {
            map.set_font(style.text.font_id, style.text.font_size, cx)
        });

        // If the
        if font_changed {
            let handle = self.handle.clone();
            cx.defer(move |cx| {
                if let Some(editor) = handle.upgrade(cx) {
                    editor.update(cx, |editor, cx| {
                        hide_hover(editor, cx);
                    })
                }
            });
        }

        EditorElement::new(self.handle.clone(), style.clone(), self.cursor_shape).boxed()
    }

    fn ui_name() -> &'static str {
        "Editor"
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        let focused_event = EditorFocused(cx.handle());
        cx.emit_global(focused_event);
        if let Some(rename) = self.pending_rename.as_ref() {
            cx.focus(&rename.editor);
        } else {
            self.focused = true;
            self.blink_cursors(self.blink_epoch, cx);
            self.buffer.update(cx, |buffer, cx| {
                buffer.finalize_last_transaction(cx);
                if self.leader_replica_id.is_none() {
                    buffer.set_active_selections(
                        &self.selections.disjoint_anchors(),
                        self.selections.line_mode,
                        cx,
                    );
                }
            });
        }
    }

    fn on_blur(&mut self, cx: &mut ViewContext<Self>) {
        let blurred_event = EditorBlurred(cx.handle());
        cx.emit_global(blurred_event);
        self.focused = false;
        self.buffer
            .update(cx, |buffer, cx| buffer.remove_active_selections(cx));
        self.hide_context_menu(cx);
        cx.emit(Event::Blurred);
        cx.notify();
    }

    fn keymap_context(&self, _: &AppContext) -> gpui::keymap::Context {
        let mut context = Self::default_keymap_context();
        let mode = match self.mode {
            EditorMode::SingleLine => "single_line",
            EditorMode::AutoHeight { .. } => "auto_height",
            EditorMode::Full => "full",
        };
        context.map.insert("mode".into(), mode.into());
        if self.pending_rename.is_some() {
            context.set.insert("renaming".into());
        }
        match self.context_menu.as_ref() {
            Some(ContextMenu::Completions(_)) => {
                context.set.insert("showing_completions".into());
            }
            Some(ContextMenu::CodeActions(_)) => {
                context.set.insert("showing_code_actions".into());
            }
            None => {}
        }

        for layer in self.keymap_context_layers.values() {
            context.extend(layer);
        }

        context
    }
}

fn build_style(
    settings: &Settings,
    get_field_editor_theme: Option<GetFieldEditorTheme>,
    override_text_style: Option<&OverrideTextStyle>,
    cx: &AppContext,
) -> EditorStyle {
    let font_cache = cx.font_cache();

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
            theme,
        }
    } else {
        let font_family_id = settings.buffer_font_family;
        let font_family_name = cx.font_cache().family_name(font_family_id).unwrap();
        let font_properties = Default::default();
        let font_id = font_cache
            .select_font(font_family_id, &font_properties)
            .unwrap();
        let font_size = settings.buffer_font_size;
        EditorStyle {
            text: TextStyle {
                color: settings.theme.editor.text_color,
                font_family_name,
                font_family_id,
                font_id,
                font_size,
                font_properties,
                underline: Default::default(),
            },
            placeholder_text: None,
            theme,
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
                        .zip(region.ranges().iter().map(|r| r.to_offset(&buffer)))
                        .all(|(selection, invalidation_range)| {
                            let head = selection.head().to_offset(&buffer);
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

impl InvalidationRegion for BracketPairState {
    fn ranges(&self) -> &[Range<Anchor>] {
        &self.ranges
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
    for line in diagnostic.message.lines() {
        highlighted_lines.push(highlight_diagnostic_message(line));
    }

    Arc::new(move |cx: &mut BlockContext| {
        let settings = cx.global::<Settings>();
        let theme = &settings.theme.editor;
        let style = diagnostic_style(diagnostic.severity, is_valid, theme);
        let font_size = (style.text_scale_factor * settings.buffer_font_size).round();
        Flex::column()
            .with_children(highlighted_lines.iter().map(|(line, highlights)| {
                Label::new(
                    line.clone(),
                    style.message.clone().with_font_size(font_size),
                )
                .with_highlights(highlights.clone())
                .contained()
                .with_margin_left(cx.anchor_x)
                .boxed()
            }))
            .aligned()
            .left()
            .boxed()
    })
}

pub fn highlight_diagnostic_message(message: &str) -> (String, Vec<usize>) {
    let mut message_without_backticks = String::new();
    let mut prev_offset = 0;
    let mut inside_block = false;
    let mut highlights = Vec::new();
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
            let mut muted_style = style.clone();
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

#[cfg(test)]
mod tests {
    use crate::{
        hover_popover::{hover, hover_at, HoverAt, HOVER_DELAY_MILLIS, HOVER_GRACE_MILLIS},
        test::{
            assert_text_with_selections, build_editor, select_ranges, EditorLspTestContext,
            EditorTestContext,
        },
    };

    use super::*;
    use gpui::{
        geometry::rect::RectF,
        platform::{WindowBounds, WindowOptions},
    };
    use indoc::indoc;
    use language::{FakeLspAdapter, LanguageConfig};
    use lsp::FakeLanguageServer;
    use project::{FakeFs, HoverBlock};
    use settings::LanguageOverride;
    use smol::stream::StreamExt;
    use std::{cell::RefCell, rc::Rc, time::Instant};
    use text::Point;
    use unindent::Unindent;
    use util::{
        assert_set_eq,
        test::{marked_text_by, marked_text_ranges, marked_text_ranges_by, sample_text},
    };
    use workspace::{FollowableItem, ItemHandle};

    #[gpui::test]
    fn test_edit_events(cx: &mut MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = cx.add_model(|cx| language::Buffer::new(0, "123456", cx));

        let events = Rc::new(RefCell::new(Vec::new()));
        let (_, editor1) = cx.add_window(Default::default(), {
            let events = events.clone();
            |cx| {
                cx.subscribe(&cx.handle(), move |_, _, event, _| {
                    if matches!(event, Event::Edited | Event::BufferEdited | Event::Dirtied) {
                        events.borrow_mut().push(("editor1", *event));
                    }
                })
                .detach();
                Editor::for_buffer(buffer.clone(), None, cx)
            }
        });
        let (_, editor2) = cx.add_window(Default::default(), {
            let events = events.clone();
            |cx| {
                cx.subscribe(&cx.handle(), move |_, _, event, _| {
                    if matches!(event, Event::Edited | Event::BufferEdited | Event::Dirtied) {
                        events.borrow_mut().push(("editor2", *event));
                    }
                })
                .detach();
                Editor::for_buffer(buffer.clone(), None, cx)
            }
        });
        assert_eq!(mem::take(&mut *events.borrow_mut()), []);

        // Mutating editor 1 will emit an `Edited` event only for that editor.
        editor1.update(cx, |editor, cx| editor.insert("X", cx));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [
                ("editor1", Event::Edited),
                ("editor1", Event::BufferEdited),
                ("editor2", Event::BufferEdited),
                ("editor1", Event::Dirtied),
                ("editor2", Event::Dirtied)
            ]
        );

        // Mutating editor 2 will emit an `Edited` event only for that editor.
        editor2.update(cx, |editor, cx| editor.delete(&Delete, cx));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [
                ("editor2", Event::Edited),
                ("editor1", Event::BufferEdited),
                ("editor2", Event::BufferEdited),
            ]
        );

        // Undoing on editor 1 will emit an `Edited` event only for that editor.
        editor1.update(cx, |editor, cx| editor.undo(&Undo, cx));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [
                ("editor1", Event::Edited),
                ("editor1", Event::BufferEdited),
                ("editor2", Event::BufferEdited),
            ]
        );

        // Redoing on editor 1 will emit an `Edited` event only for that editor.
        editor1.update(cx, |editor, cx| editor.redo(&Redo, cx));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [
                ("editor1", Event::Edited),
                ("editor1", Event::BufferEdited),
                ("editor2", Event::BufferEdited),
            ]
        );

        // Undoing on editor 2 will emit an `Edited` event only for that editor.
        editor2.update(cx, |editor, cx| editor.undo(&Undo, cx));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [
                ("editor2", Event::Edited),
                ("editor1", Event::BufferEdited),
                ("editor2", Event::BufferEdited),
            ]
        );

        // Redoing on editor 2 will emit an `Edited` event only for that editor.
        editor2.update(cx, |editor, cx| editor.redo(&Redo, cx));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [
                ("editor2", Event::Edited),
                ("editor1", Event::BufferEdited),
                ("editor2", Event::BufferEdited),
            ]
        );

        // No event is emitted when the mutation is a no-op.
        editor2.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| s.select_ranges([0..0]));

            editor.backspace(&Backspace, cx);
        });
        assert_eq!(mem::take(&mut *events.borrow_mut()), []);
    }

    #[gpui::test]
    fn test_undo_redo_with_selection_restoration(cx: &mut MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let mut now = Instant::now();
        let buffer = cx.add_model(|cx| language::Buffer::new(0, "123456", cx));
        let group_interval = buffer.read(cx).transaction_group_interval();
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

        editor.update(cx, |editor, cx| {
            editor.start_transaction_at(now, cx);
            editor.change_selections(None, cx, |s| s.select_ranges([2..4]));

            editor.insert("cd", cx);
            editor.end_transaction_at(now, cx);
            assert_eq!(editor.text(cx), "12cd56");
            assert_eq!(editor.selections.ranges(cx), vec![4..4]);

            editor.start_transaction_at(now, cx);
            editor.change_selections(None, cx, |s| s.select_ranges([4..5]));
            editor.insert("e", cx);
            editor.end_transaction_at(now, cx);
            assert_eq!(editor.text(cx), "12cde6");
            assert_eq!(editor.selections.ranges(cx), vec![5..5]);

            now += group_interval + Duration::from_millis(1);
            editor.change_selections(None, cx, |s| s.select_ranges([2..2]));

            // Simulate an edit in another editor
            buffer.update(cx, |buffer, cx| {
                buffer.start_transaction_at(now, cx);
                buffer.edit([(0..1, "a")], cx);
                buffer.edit([(1..1, "b")], cx);
                buffer.end_transaction_at(now, cx);
            });

            assert_eq!(editor.text(cx), "ab2cde6");
            assert_eq!(editor.selections.ranges(cx), vec![3..3]);

            // Last transaction happened past the group interval in a different editor.
            // Undo it individually and don't restore selections.
            editor.undo(&Undo, cx);
            assert_eq!(editor.text(cx), "12cde6");
            assert_eq!(editor.selections.ranges(cx), vec![2..2]);

            // First two transactions happened within the group interval in this editor.
            // Undo them together and restore selections.
            editor.undo(&Undo, cx);
            editor.undo(&Undo, cx); // Undo stack is empty here, so this is a no-op.
            assert_eq!(editor.text(cx), "123456");
            assert_eq!(editor.selections.ranges(cx), vec![0..0]);

            // Redo the first two transactions together.
            editor.redo(&Redo, cx);
            assert_eq!(editor.text(cx), "12cde6");
            assert_eq!(editor.selections.ranges(cx), vec![5..5]);

            // Redo the last transaction on its own.
            editor.redo(&Redo, cx);
            assert_eq!(editor.text(cx), "ab2cde6");
            assert_eq!(editor.selections.ranges(cx), vec![6..6]);

            // Test empty transactions.
            editor.start_transaction_at(now, cx);
            editor.end_transaction_at(now, cx);
            editor.undo(&Undo, cx);
            assert_eq!(editor.text(cx), "12cde6");
        });
    }

    #[gpui::test]
    fn test_selection_with_mouse(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));

        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\nddddddd\n", cx);
        let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        editor.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(2, 2), false, 1, cx);
        });
        assert_eq!(
            editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
        );

        editor.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );

        editor.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
        );

        editor.update(cx, |view, cx| {
            view.end_selection(cx);
            view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
        );

        editor.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(3, 3), true, 1, cx);
            view.update_selection(DisplayPoint::new(0, 0), 0, Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
            [
                DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)
            ]
        );

        editor.update(cx, |view, cx| {
            view.end_selection(cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
            [DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)]
        );
    }

    #[gpui::test]
    fn test_canceling_pending_selection(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

        view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(2, 2), false, 1, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
            );
        });

        view.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
            );
        });
    }

    #[gpui::test]
    fn test_clone_with_selections(cx: &mut gpui::MutableAppContext) {
        let (text, selection_ranges) = marked_text_ranges(indoc! {"
            The qu[ick brown
            fox jum]ps over
            the lazy dog
        "});
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&text, cx);

        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

        let cloned_editor = view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| s.select_ranges(selection_ranges.clone()));
            view.clone(cx)
        });

        assert_set_eq!(cloned_editor.selections.ranges(cx), selection_ranges);
    }

    #[gpui::test]
    fn test_navigation_history(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        use workspace::Item;
        let nav_history = Rc::new(RefCell::new(workspace::NavHistory::default()));
        let buffer = MultiBuffer::build_simple(&sample_text(300, 5, 'a'), cx);

        cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(buffer.clone(), cx);
            editor.nav_history = Some(ItemNavHistory::new(nav_history.clone(), &cx.handle()));

            // Move the cursor a small distance.
            // Nothing is added to the navigation history.
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)])
            });
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0)])
            });
            assert!(nav_history.borrow_mut().pop_backward().is_none());

            // Move the cursor a large distance.
            // The history can jump back to the previous position.
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(13, 0)..DisplayPoint::new(13, 3)])
            });
            let nav_entry = nav_history.borrow_mut().pop_backward().unwrap();
            editor.navigate(nav_entry.data.unwrap(), cx);
            assert_eq!(nav_entry.item.id(), cx.view_id());
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0)]
            );
            assert!(nav_history.borrow_mut().pop_backward().is_none());

            // Move the cursor a small distance via the mouse.
            // Nothing is added to the navigation history.
            editor.begin_selection(DisplayPoint::new(5, 0), false, 1, cx);
            editor.end_selection(cx);
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(5, 0)..DisplayPoint::new(5, 0)]
            );
            assert!(nav_history.borrow_mut().pop_backward().is_none());

            // Move the cursor a large distance via the mouse.
            // The history can jump back to the previous position.
            editor.begin_selection(DisplayPoint::new(15, 0), false, 1, cx);
            editor.end_selection(cx);
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(15, 0)..DisplayPoint::new(15, 0)]
            );
            let nav_entry = nav_history.borrow_mut().pop_backward().unwrap();
            editor.navigate(nav_entry.data.unwrap(), cx);
            assert_eq!(nav_entry.item.id(), cx.view_id());
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(5, 0)..DisplayPoint::new(5, 0)]
            );
            assert!(nav_history.borrow_mut().pop_backward().is_none());

            // Set scroll position to check later
            editor.set_scroll_position(Vector2F::new(5.5, 5.5), cx);
            let original_scroll_position = editor.scroll_position;
            let original_scroll_top_anchor = editor.scroll_top_anchor.clone();

            // Jump to the end of the document and adjust scroll
            editor.move_to_end(&MoveToEnd, cx);
            editor.set_scroll_position(Vector2F::new(-2.5, -0.5), cx);
            assert_ne!(editor.scroll_position, original_scroll_position);
            assert_ne!(editor.scroll_top_anchor, original_scroll_top_anchor);

            let nav_entry = nav_history.borrow_mut().pop_backward().unwrap();
            editor.navigate(nav_entry.data.unwrap(), cx);
            assert_eq!(editor.scroll_position, original_scroll_position);
            assert_eq!(editor.scroll_top_anchor, original_scroll_top_anchor);

            // Ensure we don't panic when navigation data contains invalid anchors *and* points.
            let mut invalid_anchor = editor.scroll_top_anchor.clone();
            invalid_anchor.text_anchor.buffer_id = Some(999);
            let invalid_point = Point::new(9999, 0);
            editor.navigate(
                Box::new(NavigationData {
                    cursor_anchor: invalid_anchor.clone(),
                    cursor_position: invalid_point,
                    scroll_top_anchor: invalid_anchor.clone(),
                    scroll_top_row: invalid_point.row,
                    scroll_position: Default::default(),
                }),
                cx,
            );
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[editor.max_point(cx)..editor.max_point(cx)]
            );
            assert_eq!(
                editor.scroll_position(cx),
                vec2f(0., editor.max_point(cx).row() as f32)
            );

            editor
        });
    }

    #[gpui::test]
    fn test_cancel(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

        view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(3, 4), false, 1, cx);
            view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
            view.end_selection(cx);

            view.begin_selection(DisplayPoint::new(0, 1), true, 1, cx);
            view.update_selection(DisplayPoint::new(0, 3), 0, Vector2F::zero(), cx);
            view.end_selection(cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                [
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                [DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1)]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                [DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1)]
            );
        });
    }

    #[gpui::test]
    fn test_fold(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(
            &"
                impl Foo {
                    // Hello!

                    fn a() {
                        1
                    }

                    fn b() {
                        2
                    }

                    fn c() {
                        3
                    }
                }
            "
            .unindent(),
            cx,
        );
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(8, 0)..DisplayPoint::new(12, 0)]);
            });
            view.fold(&Fold, cx);
            assert_eq!(
                view.display_text(cx),
                "
                    impl Foo {
                        // Hello!

                        fn a() {
                            1
                        }

                        fn b() {…
                        }

                        fn c() {…
                        }
                    }
                "
                .unindent(),
            );

            view.fold(&Fold, cx);
            assert_eq!(
                view.display_text(cx),
                "
                    impl Foo {…
                    }
                "
                .unindent(),
            );

            view.unfold_lines(&UnfoldLines, cx);
            assert_eq!(
                view.display_text(cx),
                "
                    impl Foo {
                        // Hello!

                        fn a() {
                            1
                        }

                        fn b() {…
                        }

                        fn c() {…
                        }
                    }
                "
                .unindent(),
            );

            view.unfold_lines(&UnfoldLines, cx);
            assert_eq!(view.display_text(cx), buffer.read(cx).read(cx).text());
        });
    }

    #[gpui::test]
    fn test_move_cursor(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                vec![
                    (Point::new(1, 0)..Point::new(1, 0), "\t"),
                    (Point::new(1, 1)..Point::new(1, 1), "\t"),
                ],
                cx,
            );
        });

        view.update(cx, |view, cx| {
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );

            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4)]
            );

            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.move_to_end(&MoveToEnd, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(5, 6)..DisplayPoint::new(5, 6)]
            );

            view.move_to_beginning(&MoveToBeginning, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2)]);
            });
            view.select_to_beginning(&SelectToBeginning, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 0)]
            );

            view.select_to_end(&SelectToEnd, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(0, 1)..DisplayPoint::new(5, 6)]
            );
        });
    }

    #[gpui::test]
    fn test_move_cursor_multibyte(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcde\nαβγδε\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

        assert_eq!('ⓐ'.len_utf8(), 3);
        assert_eq!('α'.len_utf8(), 2);

        view.update(cx, |view, cx| {
            view.fold_ranges(
                vec![
                    Point::new(0, 6)..Point::new(0, 12),
                    Point::new(1, 2)..Point::new(1, 4),
                    Point::new(2, 4)..Point::new(2, 8),
                ],
                cx,
            );
            assert_eq!(view.display_text(cx), "ⓐⓑ…ⓔ\nab…e\nαβ…ε\n");

            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(0, "ⓐ".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(0, "ⓐⓑ".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(0, "ⓐⓑ…".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(1, "ab…".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(1, "ab".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(1, "a".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(2, "α".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(2, "αβ".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(2, "αβ…".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(2, "αβ…ε".len())]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(1, "ab…e".len())]
            );
            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(0, "ⓐⓑ…ⓔ".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(0, "ⓐⓑ…".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(0, "ⓐⓑ".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(0, "ⓐ".len())]
            );
        });
    }

    #[gpui::test]
    fn test_move_cursor_different_line_lengths(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcd\nαβγ\nabcd\nⓐⓑⓒⓓⓔ\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));
        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([empty_range(0, "ⓐⓑⓒⓓⓔ".len())]);
            });
            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(1, "abcd".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(2, "αβγ".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(3, "abcd".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(4, "ⓐⓑⓒⓓⓔ".len())]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(3, "abcd".len())]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[empty_range(2, "αβγ".len())]
            );
        });
    }

    #[gpui::test]
    fn test_beginning_end_of_line(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("abc\n  def", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
                ]);
            });
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_end_of_line(&MoveToEndOfLine, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
                ]
            );
        });

        // Moving to the end of line again is a no-op.
        view.update(cx, |view, cx| {
            view.move_to_end_of_line(&MoveToEndOfLine, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_left(&MoveLeft, cx);
            view.select_to_beginning_of_line(
                &SelectToBeginningOfLine {
                    stop_at_soft_wraps: true,
                },
                cx,
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_beginning_of_line(
                &SelectToBeginningOfLine {
                    stop_at_soft_wraps: true,
                },
                cx,
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_beginning_of_line(
                &SelectToBeginningOfLine {
                    stop_at_soft_wraps: true,
                },
                cx,
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_end_of_line(
                &SelectToEndOfLine {
                    stop_at_soft_wraps: true,
                },
                cx,
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 5),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.delete_to_end_of_line(&DeleteToEndOfLine, cx);
            assert_eq!(view.display_text(cx), "ab\n  de");
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.delete_to_beginning_of_line(&DeleteToBeginningOfLine, cx);
            assert_eq!(view.display_text(cx), "\n");
            assert_eq!(
                view.selections.display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_prev_next_word_boundary(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("use std::str::{foo, bar}\n\n  {baz.qux()}", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 11)..DisplayPoint::new(0, 11),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4),
                ])
            });

            view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
            assert_selection_ranges(
                "use std::<>str::{foo, bar}\n\n  {[]baz.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
            assert_selection_ranges(
                "use std<>::str::{foo, bar}\n\n  []{baz.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
            assert_selection_ranges(
                "use <>std::str::{foo, bar}\n\n[]  {baz.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
            assert_selection_ranges(
                "<>use std::str::{foo, bar}\n[]\n  {baz.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
            assert_selection_ranges(
                "<>use std::str::{foo, bar[]}\n\n  {baz.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.move_to_next_word_end(&MoveToNextWordEnd, cx);
            assert_selection_ranges(
                "use<> std::str::{foo, bar}[]\n\n  {baz.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.move_to_next_word_end(&MoveToNextWordEnd, cx);
            assert_selection_ranges(
                "use std<>::str::{foo, bar}\n[]\n  {baz.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.move_to_next_word_end(&MoveToNextWordEnd, cx);
            assert_selection_ranges(
                "use std::<>str::{foo, bar}\n\n  {[]baz.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.move_right(&MoveRight, cx);
            view.select_to_previous_word_start(&SelectToPreviousWordStart, cx);
            assert_selection_ranges(
                "use std::>s<tr::{foo, bar}\n\n  {]b[az.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.select_to_previous_word_start(&SelectToPreviousWordStart, cx);
            assert_selection_ranges(
                "use std>::s<tr::{foo, bar}\n\n  ]{b[az.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );

            view.select_to_next_word_end(&SelectToNextWordEnd, cx);
            assert_selection_ranges(
                "use std::>s<tr::{foo, bar}\n\n  {]b[az.qux()}",
                vec![('<', '>'), ('[', ']')],
                view,
                cx,
            );
        });
    }

    #[gpui::test]
    fn test_prev_next_word_bounds_with_soft_wrap(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("use one::{\n    two::three::four::five\n};", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

        view.update(cx, |view, cx| {
            view.set_wrap_width(Some(140.), cx);
            assert_eq!(
                view.display_text(cx),
                "use one::{\n    two::three::\n    four::five\n};"
            );

            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 7)..DisplayPoint::new(1, 7)]);
            });

            view.move_to_next_word_end(&MoveToNextWordEnd, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(1, 9)..DisplayPoint::new(1, 9)]
            );

            view.move_to_next_word_end(&MoveToNextWordEnd, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(1, 14)..DisplayPoint::new(1, 14)]
            );

            view.move_to_next_word_end(&MoveToNextWordEnd, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4)]
            );

            view.move_to_next_word_end(&MoveToNextWordEnd, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(2, 8)..DisplayPoint::new(2, 8)]
            );

            view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4)]
            );

            view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(1, 14)..DisplayPoint::new(1, 14)]
            );
        });
    }

    #[gpui::test]
    fn test_delete_to_beginning_of_line(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let (text, ranges) = marked_text_ranges("one [two three] four");
        let buffer = MultiBuffer::build_simple(&text, cx);

        let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| s.select_ranges(ranges));
            editor.delete_to_beginning_of_line(&DeleteToBeginningOfLine, cx);
            assert_eq!(editor.text(cx), " four");
        });
    }

    #[gpui::test]
    fn test_delete_to_word_boundary(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("one two three four", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    // an empty selection - the preceding word fragment is deleted
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    // characters selected - they are deleted
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 12),
                ])
            });
            view.delete_to_previous_word_start(&DeleteToPreviousWordStart, cx);
        });

        assert_eq!(buffer.read(cx).read(cx).text(), "e two te four");

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    // an empty selection - the following word fragment is deleted
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    // characters selected - they are deleted
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 10),
                ])
            });
            view.delete_to_next_word_end(&DeleteToNextWordEnd, cx);
        });

        assert_eq!(buffer.read(cx).read(cx).text(), "e t te our");
    }

    #[gpui::test]
    fn test_newline(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("aaaa\n    bbbb\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                    DisplayPoint::new(1, 6)..DisplayPoint::new(1, 6),
                ])
            });

            view.newline(&Newline, cx);
            assert_eq!(view.text(cx), "aa\naa\n  \n    bb\n    bb\n");
        });
    }

    #[gpui::test]
    fn test_newline_with_old_selections(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(
            "
                a
                b(
                    X
                )
                c(
                    X
                )
            "
            .unindent()
            .as_str(),
            cx,
        );

        let (_, editor) = cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(buffer.clone(), cx);
            editor.change_selections(None, cx, |s| {
                s.select_ranges([
                    Point::new(2, 4)..Point::new(2, 5),
                    Point::new(5, 4)..Point::new(5, 5),
                ])
            });
            editor
        });

        // Edit the buffer directly, deleting ranges surrounding the editor's selections
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    (Point::new(1, 2)..Point::new(3, 0), ""),
                    (Point::new(4, 2)..Point::new(6, 0), ""),
                ],
                cx,
            );
            assert_eq!(
                buffer.read(cx).text(),
                "
                    a
                    b()
                    c()
                "
                .unindent()
            );
        });

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.selections.ranges(cx),
                &[
                    Point::new(1, 2)..Point::new(1, 2),
                    Point::new(2, 2)..Point::new(2, 2),
                ],
            );

            editor.newline(&Newline, cx);
            assert_eq!(
                editor.text(cx),
                "
                    a
                    b(
                    )
                    c(
                    )
                "
                .unindent()
            );

            // The selections are moved after the inserted newlines
            assert_eq!(
                editor.selections.ranges(cx),
                &[
                    Point::new(2, 0)..Point::new(2, 0),
                    Point::new(4, 0)..Point::new(4, 0),
                ],
            );
        });
    }

    #[gpui::test]
    fn test_insert_with_old_selections(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("a( X ), b( Y ), c( Z )", cx);
        let (_, editor) = cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(buffer.clone(), cx);
            editor.change_selections(None, cx, |s| s.select_ranges([3..4, 11..12, 19..20]));
            editor
        });

        // Edit the buffer directly, deleting ranges surrounding the editor's selections
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(2..5, ""), (10..13, ""), (18..21, "")], cx);
            assert_eq!(buffer.read(cx).text(), "a(), b(), c()".unindent());
        });

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.selections.ranges(cx), &[2..2, 7..7, 12..12],);

            editor.insert("Z", cx);
            assert_eq!(editor.text(cx), "a(Z), b(Z), c(Z)");

            // The selections are moved after the inserted characters
            assert_eq!(editor.selections.ranges(cx), &[3..3, 9..9, 15..15],);
        });
    }

    #[gpui::test]
    async fn test_indent_outdent(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorTestContext::new(cx).await;

        cx.set_state(indoc! {"
              [one} [two}
            three
             four"});
        cx.update_editor(|e, cx| e.tab(&Tab, cx));
        cx.assert_editor_state(indoc! {"
                [one} [two}
            three
             four"});

        cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
        cx.assert_editor_state(indoc! {"
            [one} [two}
            three
             four"});

        // select across line ending
        cx.set_state(indoc! {"
            one two
            t[hree
            } four"});
        cx.update_editor(|e, cx| e.tab(&Tab, cx));
        cx.assert_editor_state(indoc! {"
            one two
                t[hree
            } four"});

        cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
        cx.assert_editor_state(indoc! {"
            one two
            t[hree
            } four"});

        // Ensure that indenting/outdenting works when the cursor is at column 0.
        cx.set_state(indoc! {"
            one two
            |three
                four"});
        cx.update_editor(|e, cx| e.tab(&Tab, cx));
        cx.assert_editor_state(indoc! {"
            one two
                |three
                four"});

        cx.set_state(indoc! {"
            one two
            |    three
             four"});
        cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
        cx.assert_editor_state(indoc! {"
            one two
            |three
             four"});
    }

    #[gpui::test]
    async fn test_indent_outdent_with_hard_tabs(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorTestContext::new(cx).await;
        cx.update(|cx| {
            cx.update_global::<Settings, _, _>(|settings, _| {
                settings.hard_tabs = true;
            });
        });

        // select two ranges on one line
        cx.set_state(indoc! {"
            [one} [two}
            three
            four"});
        cx.update_editor(|e, cx| e.tab(&Tab, cx));
        cx.assert_editor_state(indoc! {"
            \t[one} [two}
            three
            four"});
        cx.update_editor(|e, cx| e.tab(&Tab, cx));
        cx.assert_editor_state(indoc! {"
            \t\t[one} [two}
            three
            four"});
        cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
        cx.assert_editor_state(indoc! {"
            \t[one} [two}
            three
            four"});
        cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
        cx.assert_editor_state(indoc! {"
            [one} [two}
            three
            four"});

        // select across a line ending
        cx.set_state(indoc! {"
            one two
            t[hree
            }four"});
        cx.update_editor(|e, cx| e.tab(&Tab, cx));
        cx.assert_editor_state(indoc! {"
            one two
            \tt[hree
            }four"});
        cx.update_editor(|e, cx| e.tab(&Tab, cx));
        cx.assert_editor_state(indoc! {"
            one two
            \t\tt[hree
            }four"});
        cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
        cx.assert_editor_state(indoc! {"
            one two
            \tt[hree
            }four"});
        cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
        cx.assert_editor_state(indoc! {"
            one two
            t[hree
            }four"});

        // Ensure that indenting/outdenting works when the cursor is at column 0.
        cx.set_state(indoc! {"
            one two
            |three
            four"});
        cx.assert_editor_state(indoc! {"
            one two
            |three
            four"});
        cx.update_editor(|e, cx| e.tab(&Tab, cx));
        cx.assert_editor_state(indoc! {"
            one two
            \t|three
            four"});
        cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
        cx.assert_editor_state(indoc! {"
            one two
            |three
            four"});
    }

    #[gpui::test]
    fn test_indent_outdent_with_excerpts(cx: &mut gpui::MutableAppContext) {
        cx.set_global(
            Settings::test(cx)
                .with_overrides(
                    "TOML",
                    LanguageOverride {
                        tab_size: Some(2),
                        ..Default::default()
                    },
                )
                .with_overrides(
                    "Rust",
                    LanguageOverride {
                        tab_size: Some(4),
                        ..Default::default()
                    },
                ),
        );
        let toml_language = Arc::new(Language::new(
            LanguageConfig {
                name: "TOML".into(),
                ..Default::default()
            },
            None,
        ));
        let rust_language = Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                ..Default::default()
            },
            None,
        ));

        let toml_buffer = cx
            .add_model(|cx| Buffer::new(0, "a = 1\nb = 2\n", cx).with_language(toml_language, cx));
        let rust_buffer = cx.add_model(|cx| {
            Buffer::new(0, "const c: usize = 3;\n", cx).with_language(rust_language, cx)
        });
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            multibuffer.push_excerpts(
                toml_buffer.clone(),
                [ExcerptRange {
                    context: Point::new(0, 0)..Point::new(2, 0),
                    primary: None,
                }],
                cx,
            );
            multibuffer.push_excerpts(
                rust_buffer.clone(),
                [ExcerptRange {
                    context: Point::new(0, 0)..Point::new(1, 0),
                    primary: None,
                }],
                cx,
            );
            multibuffer
        });

        cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(multibuffer, cx);

            assert_eq!(
                editor.text(cx),
                indoc! {"
                    a = 1
                    b = 2

                    const c: usize = 3;
                "}
            );

            select_ranges(
                &mut editor,
                indoc! {"
                    [a] = 1
                    b = 2

                    [const c:] usize = 3;
                "},
                cx,
            );

            editor.tab(&Tab, cx);
            assert_text_with_selections(
                &mut editor,
                indoc! {"
                      [a] = 1
                    b = 2

                        [const c:] usize = 3;
                "},
                cx,
            );
            editor.tab_prev(&TabPrev, cx);
            assert_text_with_selections(
                &mut editor,
                indoc! {"
                    [a] = 1
                    b = 2

                    [const c:] usize = 3;
                "},
                cx,
            );

            editor
        });
    }

    #[gpui::test]
    async fn test_backspace(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorTestContext::new(cx).await;
        // Basic backspace
        cx.set_state(indoc! {"
            on|e two three
            fou[r} five six
            seven {eight nine
            ]ten"});
        cx.update_editor(|e, cx| e.backspace(&Backspace, cx));
        cx.assert_editor_state(indoc! {"
            o|e two three
            fou| five six
            seven |ten"});

        // Test backspace inside and around indents
        cx.set_state(indoc! {"
            zero
                |one
                    |two
                | | |  three
            |  |  four"});
        cx.update_editor(|e, cx| e.backspace(&Backspace, cx));
        cx.assert_editor_state(indoc! {"
            zero
            |one
                |two
            |  three|  four"});

        // Test backspace with line_mode set to true
        cx.update_editor(|e, _| e.selections.line_mode = true);
        cx.set_state(indoc! {"
            The |quick |brown
            fox jumps over
            the lazy dog
            |The qu[ick b}rown"});
        cx.update_editor(|e, cx| e.backspace(&Backspace, cx));
        cx.assert_editor_state(indoc! {"
            |fox jumps over
            the lazy dog|"});
    }

    #[gpui::test]
    async fn test_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorTestContext::new(cx).await;

        cx.set_state(indoc! {"
            on|e two three
            fou[r} five six
            seven {eight nine
            ]ten"});
        cx.update_editor(|e, cx| e.delete(&Delete, cx));
        cx.assert_editor_state(indoc! {"
            on| two three
            fou| five six
            seven |ten"});

        // Test backspace with line_mode set to true
        cx.update_editor(|e, _| e.selections.line_mode = true);
        cx.set_state(indoc! {"
            The |quick |brown
            fox {jum]ps over
            the lazy dog
            |The qu[ick b}rown"});
        cx.update_editor(|e, cx| e.backspace(&Backspace, cx));
        cx.assert_editor_state("|the lazy dog|");
    }

    #[gpui::test]
    fn test_delete_line(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                ])
            });
            view.delete_line(&DeleteLine, cx);
            assert_eq!(view.display_text(cx), "ghi");
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)
                ]
            );
        });

        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(2, 0)..DisplayPoint::new(0, 1)])
            });
            view.delete_line(&DeleteLine, cx);
            assert_eq!(view.display_text(cx), "ghi\n");
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)]
            );
        });
    }

    #[gpui::test]
    fn test_duplicate_line(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                ])
            });
            view.duplicate_line(&DuplicateLine, cx);
            assert_eq!(view.display_text(cx), "abc\nabc\ndef\ndef\nghi\n\n");
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(6, 0)..DisplayPoint::new(6, 0),
                ]
            );
        });

        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(2, 1),
                ])
            });
            view.duplicate_line(&DuplicateLine, cx);
            assert_eq!(view.display_text(cx), "abc\ndef\nghi\nabc\ndef\nghi\n");
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(3, 1)..DisplayPoint::new(4, 1),
                    DisplayPoint::new(4, 2)..DisplayPoint::new(5, 1),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_move_line_up_down(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.fold_ranges(
                vec![
                    Point::new(0, 2)..Point::new(1, 2),
                    Point::new(2, 3)..Point::new(4, 1),
                    Point::new(7, 0)..Point::new(8, 4),
                ],
                cx,
            );
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                    DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2),
                ])
            });
            assert_eq!(
                view.display_text(cx),
                "aa…bbb\nccc…eeee\nfffff\nggggg\n…i\njjjjj"
            );

            view.move_line_up(&MoveLineUp, cx);
            assert_eq!(
                view.display_text(cx),
                "aa…bbb\nccc…eeee\nggggg\n…i\njjjjj\nfffff"
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(4, 2)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_line_down(&MoveLineDown, cx);
            assert_eq!(
                view.display_text(cx),
                "ccc…eeee\naa…bbb\nfffff\nggggg\n…i\njjjjj"
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                    DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_line_down(&MoveLineDown, cx);
            assert_eq!(
                view.display_text(cx),
                "ccc…eeee\nfffff\naa…bbb\nggggg\n…i\njjjjj"
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                    DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_line_up(&MoveLineUp, cx);
            assert_eq!(
                view.display_text(cx),
                "ccc…eeee\naa…bbb\nggggg\n…i\njjjjj\nfffff"
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(4, 2)
                ]
            );
        });
    }

    #[gpui::test]
    fn test_move_line_up_down_with_blocks(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
        let snapshot = buffer.read(cx).snapshot(cx);
        let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        editor.update(cx, |editor, cx| {
            editor.insert_blocks(
                [BlockProperties {
                    position: snapshot.anchor_after(Point::new(2, 0)),
                    disposition: BlockDisposition::Below,
                    height: 1,
                    render: Arc::new(|_| Empty::new().boxed()),
                }],
                cx,
            );
            editor.change_selections(None, cx, |s| {
                s.select_ranges([Point::new(2, 0)..Point::new(2, 0)])
            });
            editor.move_line_down(&MoveLineDown, cx);
        });
    }

    #[gpui::test]
    fn test_transpose(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));

        cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(MultiBuffer::build_simple("abc", cx), cx);

            editor.change_selections(None, cx, |s| s.select_ranges([1..1]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bac");
            assert_eq!(editor.selections.ranges(cx), [2..2]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bca");
            assert_eq!(editor.selections.ranges(cx), [3..3]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bac");
            assert_eq!(editor.selections.ranges(cx), [3..3]);

            editor
        })
        .1;

        cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(MultiBuffer::build_simple("abc\nde", cx), cx);

            editor.change_selections(None, cx, |s| s.select_ranges([3..3]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "acb\nde");
            assert_eq!(editor.selections.ranges(cx), [3..3]);

            editor.change_selections(None, cx, |s| s.select_ranges([4..4]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "acbd\ne");
            assert_eq!(editor.selections.ranges(cx), [5..5]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "acbde\n");
            assert_eq!(editor.selections.ranges(cx), [6..6]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "acbd\ne");
            assert_eq!(editor.selections.ranges(cx), [6..6]);

            editor
        })
        .1;

        cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(MultiBuffer::build_simple("abc\nde", cx), cx);

            editor.change_selections(None, cx, |s| s.select_ranges([1..1, 2..2, 4..4]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bacd\ne");
            assert_eq!(editor.selections.ranges(cx), [2..2, 3..3, 5..5]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bcade\n");
            assert_eq!(editor.selections.ranges(cx), [3..3, 4..4, 6..6]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bcda\ne");
            assert_eq!(editor.selections.ranges(cx), [4..4, 6..6]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bcade\n");
            assert_eq!(editor.selections.ranges(cx), [4..4, 6..6]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bcaed\n");
            assert_eq!(editor.selections.ranges(cx), [5..5, 6..6]);

            editor
        })
        .1;

        cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(MultiBuffer::build_simple("🍐🏀✋", cx), cx);

            editor.change_selections(None, cx, |s| s.select_ranges([4..4]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "🏀🍐✋");
            assert_eq!(editor.selections.ranges(cx), [8..8]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "🏀✋🍐");
            assert_eq!(editor.selections.ranges(cx), [11..11]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "🏀🍐✋");
            assert_eq!(editor.selections.ranges(cx), [11..11]);

            editor
        })
        .1;
    }

    #[gpui::test]
    async fn test_clipboard(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorTestContext::new(cx).await;

        cx.set_state("[one✅ }two [three }four [five }six ");
        cx.update_editor(|e, cx| e.cut(&Cut, cx));
        cx.assert_editor_state("|two |four |six ");

        // Paste with three cursors. Each cursor pastes one slice of the clipboard text.
        cx.set_state("two |four |six |");
        cx.update_editor(|e, cx| e.paste(&Paste, cx));
        cx.assert_editor_state("two one✅ |four three |six five |");

        // Paste again but with only two cursors. Since the number of cursors doesn't
        // match the number of slices in the clipboard, the entire clipboard text
        // is pasted at each cursor.
        cx.set_state("|two one✅ four three six five |");
        cx.update_editor(|e, cx| {
            e.handle_input(&Input("( ".into()), cx);
            e.paste(&Paste, cx);
            e.handle_input(&Input(") ".into()), cx);
        });
        cx.assert_editor_state(indoc! {"
            ( one✅ 
            three 
            five ) |two one✅ four three six five ( one✅ 
            three 
            five ) |"});

        // Cut with three selections, one of which is full-line.
        cx.set_state(indoc! {"
            1[2}3
            4|567
            [8}9"});
        cx.update_editor(|e, cx| e.cut(&Cut, cx));
        cx.assert_editor_state(indoc! {"
            1|3
            |9"});

        // Paste with three selections, noticing how the copied selection that was full-line
        // gets inserted before the second cursor.
        cx.set_state(indoc! {"
            1|3
            9|
            [o}ne"});
        cx.update_editor(|e, cx| e.paste(&Paste, cx));
        cx.assert_editor_state(indoc! {"
            12|3
            4567
            9|
            8|ne"});

        // Copy with a single cursor only, which writes the whole line into the clipboard.
        cx.set_state(indoc! {"
            The quick brown
            fox ju|mps over
            the lazy dog"});
        cx.update_editor(|e, cx| e.copy(&Copy, cx));
        cx.assert_clipboard_content(Some("fox jumps over\n"));

        // Paste with three selections, noticing how the copied full-line selection is inserted
        // before the empty selections but replaces the selection that is non-empty.
        cx.set_state(indoc! {"
            T|he quick brown
            [fo}x jumps over
            t|he lazy dog"});
        cx.update_editor(|e, cx| e.paste(&Paste, cx));
        cx.assert_editor_state(indoc! {"
            fox jumps over
            T|he quick brown
            fox jumps over
            |x jumps over
            fox jumps over
            t|he lazy dog"});
    }

    #[gpui::test]
    fn test_select_all(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("abc\nde\nfgh", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.select_all(&SelectAll, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(2, 3)]
            );
        });
    }

    #[gpui::test]
    fn test_select_line(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&sample_text(6, 5, 'a'), cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                    DisplayPoint::new(4, 2)..DisplayPoint::new(4, 2),
                ])
            });
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(2, 0),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(5, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(5, 5),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![DisplayPoint::new(0, 0)..DisplayPoint::new(5, 5)]
            );
        });
    }

    #[gpui::test]
    fn test_split_selection_into_lines(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&sample_text(9, 5, 'a'), cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
        view.update(cx, |view, cx| {
            view.fold_ranges(
                vec![
                    Point::new(0, 2)..Point::new(1, 2),
                    Point::new(2, 3)..Point::new(4, 1),
                    Point::new(7, 0)..Point::new(8, 4),
                ],
                cx,
            );
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 4),
                ])
            });
            assert_eq!(view.display_text(cx), "aa…bbb\nccc…eeee\nfffff\nggggg\n…i");
        });

        view.update(cx, |view, cx| {
            view.split_selection_into_lines(&SplitSelectionIntoLines, cx);
            assert_eq!(
                view.display_text(cx),
                "aaaaa\nbbbbb\nccc…eeee\nfffff\nggggg\n…i"
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                [
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                    DisplayPoint::new(5, 4)..DisplayPoint::new(5, 4)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(5, 0)..DisplayPoint::new(0, 1)])
            });
            view.split_selection_into_lines(&SplitSelectionIntoLines, cx);
            assert_eq!(
                view.display_text(cx),
                "aaaaa\nbbbbb\nccccc\nddddd\neeeee\nfffff\nggggg\nhhhhh\niiiii"
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                [
                    DisplayPoint::new(0, 5)..DisplayPoint::new(0, 5),
                    DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
                    DisplayPoint::new(2, 5)..DisplayPoint::new(2, 5),
                    DisplayPoint::new(3, 5)..DisplayPoint::new(3, 5),
                    DisplayPoint::new(4, 5)..DisplayPoint::new(4, 5),
                    DisplayPoint::new(5, 5)..DisplayPoint::new(5, 5),
                    DisplayPoint::new(6, 5)..DisplayPoint::new(6, 5),
                    DisplayPoint::new(7, 0)..DisplayPoint::new(7, 0)
                ]
            );
        });
    }

    #[gpui::test]
    fn test_add_selection_above_below(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("abc\ndefghi\n\njk\nlmno\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)])
            });
        });
        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)]
            );

            view.undo_selection(&UndoSelection, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
                ]
            );

            view.redo_selection(&RedoSelection, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)])
            });
        });
        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(0, 1)..DisplayPoint::new(1, 4)])
            });
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
                    DisplayPoint::new(4, 1)..DisplayPoint::new(4, 4),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(4, 3)..DisplayPoint::new(1, 1)])
            });
        });
        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 1),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selections.display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 1),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_select_next(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));

        let (text, ranges) = marked_text_ranges("[abc]\n[abc] [abc]\ndefabc\n[abc]");
        let buffer = MultiBuffer::build_simple(&text, cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_ranges([ranges[1].start + 1..ranges[1].start + 1])
            });
            view.select_next(
                &SelectNext {
                    replace_newest: false,
                },
                cx,
            );
            assert_eq!(view.selections.ranges(cx), &ranges[1..2]);

            view.select_next(
                &SelectNext {
                    replace_newest: false,
                },
                cx,
            );
            assert_eq!(view.selections.ranges(cx), &ranges[1..3]);

            view.undo_selection(&UndoSelection, cx);
            assert_eq!(view.selections.ranges(cx), &ranges[1..2]);

            view.redo_selection(&RedoSelection, cx);
            assert_eq!(view.selections.ranges(cx), &ranges[1..3]);

            view.select_next(
                &SelectNext {
                    replace_newest: false,
                },
                cx,
            );
            assert_eq!(view.selections.ranges(cx), &ranges[1..4]);

            view.select_next(
                &SelectNext {
                    replace_newest: false,
                },
                cx,
            );
            assert_eq!(view.selections.ranges(cx), &ranges[0..4]);
        });
    }

    #[gpui::test]
    async fn test_select_larger_smaller_syntax_node(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));
        let language = Arc::new(Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::language()),
        ));

        let text = r#"
            use mod1::mod2::{mod3, mod4};

            fn fn_1(param1: bool, param2: &str) {
                let var1 = "text";
            }
        "#
        .unindent();

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, view) = cx.add_window(|cx| build_editor(buffer, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
            .await;

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                    DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                    DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
                ]);
            });
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| { view.selections.display_ranges(cx) }),
            &[
                DisplayPoint::new(0, 23)..DisplayPoint::new(0, 27),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 15)..DisplayPoint::new(3, 21),
            ]
        );

        view.update(cx, |view, cx| {
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selections.display_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(cx, |view, cx| {
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selections.display_ranges(cx)),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
        );

        // Trying to expand the selected syntax node one more time has no effect.
        view.update(cx, |view, cx| {
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selections.display_ranges(cx)),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
        );

        view.update(cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selections.display_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selections.display_ranges(cx)),
            &[
                DisplayPoint::new(0, 23)..DisplayPoint::new(0, 27),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 15)..DisplayPoint::new(3, 21),
            ]
        );

        view.update(cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selections.display_ranges(cx)),
            &[
                DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
            ]
        );

        // Trying to shrink the selected syntax node one more time has no effect.
        view.update(cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selections.display_ranges(cx)),
            &[
                DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
            ]
        );

        // Ensure that we keep expanding the selection if the larger selection starts or ends within
        // a fold.
        view.update(cx, |view, cx| {
            view.fold_ranges(
                vec![
                    Point::new(0, 21)..Point::new(0, 24),
                    Point::new(3, 20)..Point::new(3, 22),
                ],
                cx,
            );
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selections.display_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 4)..DisplayPoint::new(3, 23),
            ]
        );
    }

    #[gpui::test]
    async fn test_autoindent_selections(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));
        let language = Arc::new(
            Language::new(
                LanguageConfig {
                    brackets: vec![
                        BracketPair {
                            start: "{".to_string(),
                            end: "}".to_string(),
                            close: false,
                            newline: true,
                        },
                        BracketPair {
                            start: "(".to_string(),
                            end: ")".to_string(),
                            close: false,
                            newline: true,
                        },
                    ],
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
            )
            .with_indents_query(
                r#"
                (_ "(" ")" @end) @indent
                (_ "{" "}" @end) @indent
                "#,
            )
            .unwrap(),
        );

        let text = "fn a() {}";

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));
        editor
            .condition(&cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
            .await;

        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| s.select_ranges([5..5, 8..8, 9..9]));
            editor.newline(&Newline, cx);
            assert_eq!(editor.text(cx), "fn a(\n    \n) {\n    \n}\n");
            assert_eq!(
                editor.selections.ranges(cx),
                &[
                    Point::new(1, 4)..Point::new(1, 4),
                    Point::new(3, 4)..Point::new(3, 4),
                    Point::new(5, 0)..Point::new(5, 0)
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_autoclose_pairs(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));
        let language = Arc::new(Language::new(
            LanguageConfig {
                brackets: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "/*".to_string(),
                        end: " */".to_string(),
                        close: true,
                        newline: true,
                    },
                ],
                autoclose_before: "})]".to_string(),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ));

        let text = r#"
            a

            /

        "#
        .unindent();

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, view) = cx.add_window(|cx| build_editor(buffer, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
            .await;

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ])
            });

            view.handle_input(&Input("{".to_string()), cx);
            view.handle_input(&Input("{".to_string()), cx);
            view.handle_input(&Input("{".to_string()), cx);
            assert_eq!(
                view.text(cx),
                "
                {{{}}}
                {{{}}}
                /

                "
                .unindent()
            );

            view.move_right(&MoveRight, cx);
            view.handle_input(&Input("}".to_string()), cx);
            view.handle_input(&Input("}".to_string()), cx);
            view.handle_input(&Input("}".to_string()), cx);
            assert_eq!(
                view.text(cx),
                "
                {{{}}}}
                {{{}}}}
                /

                "
                .unindent()
            );

            view.undo(&Undo, cx);
            view.handle_input(&Input("/".to_string()), cx);
            view.handle_input(&Input("*".to_string()), cx);
            assert_eq!(
                view.text(cx),
                "
                /* */
                /* */
                /

                "
                .unindent()
            );

            view.undo(&Undo, cx);
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                ])
            });
            view.handle_input(&Input("*".to_string()), cx);
            assert_eq!(
                view.text(cx),
                "
                a

                /*
                *
                "
                .unindent()
            );

            // Don't autoclose if the next character isn't whitespace and isn't
            // listed in the language's "autoclose_before" section.
            view.finalize_last_transaction(cx);
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)])
            });
            view.handle_input(&Input("{".to_string()), cx);
            assert_eq!(
                view.text(cx),
                "
                {a

                /*
                *
                "
                .unindent()
            );

            view.undo(&Undo, cx);
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1)])
            });
            view.handle_input(&Input("{".to_string()), cx);
            assert_eq!(
                view.text(cx),
                "
                {a}

                /*
                *
                "
                .unindent()
            );
            assert_eq!(
                view.selections.display_ranges(cx),
                [DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2)]
            );
        });
    }

    #[gpui::test]
    async fn test_snippets(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));

        let (text, insertion_ranges) = marked_text_ranges(indoc! {"
            a.| b
            a.| b
            a.| b"});
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));
        let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));

        editor.update(cx, |editor, cx| {
            let snippet = Snippet::parse("f(${1:one}, ${2:two}, ${1:three})$0").unwrap();

            editor
                .insert_snippet(&insertion_ranges, snippet, cx)
                .unwrap();

            fn assert(editor: &mut Editor, cx: &mut ViewContext<Editor>, marked_text_ranges: &str) {
                let range_markers = ('<', '>');
                let (expected_text, mut selection_ranges_lookup) =
                    marked_text_ranges_by(marked_text_ranges, vec![range_markers.clone().into()]);
                let selection_ranges = selection_ranges_lookup
                    .remove(&range_markers.into())
                    .unwrap();
                assert_eq!(editor.text(cx), expected_text);
                assert_eq!(editor.selections.ranges::<usize>(cx), selection_ranges);
            }
            assert(
                editor,
                cx,
                indoc! {"
                    a.f(<one>, two, <three>) b
                    a.f(<one>, two, <three>) b
                    a.f(<one>, two, <three>) b"},
            );

            // Can't move earlier than the first tab stop
            assert!(!editor.move_to_prev_snippet_tabstop(cx));
            assert(
                editor,
                cx,
                indoc! {"
                    a.f(<one>, two, <three>) b
                    a.f(<one>, two, <three>) b
                    a.f(<one>, two, <three>) b"},
            );

            assert!(editor.move_to_next_snippet_tabstop(cx));
            assert(
                editor,
                cx,
                indoc! {"
                    a.f(one, <two>, three) b
                    a.f(one, <two>, three) b
                    a.f(one, <two>, three) b"},
            );

            editor.move_to_prev_snippet_tabstop(cx);
            assert(
                editor,
                cx,
                indoc! {"
                    a.f(<one>, two, <three>) b
                    a.f(<one>, two, <three>) b
                    a.f(<one>, two, <three>) b"},
            );

            assert!(editor.move_to_next_snippet_tabstop(cx));
            assert(
                editor,
                cx,
                indoc! {"
                    a.f(one, <two>, three) b
                    a.f(one, <two>, three) b
                    a.f(one, <two>, three) b"},
            );
            assert!(editor.move_to_next_snippet_tabstop(cx));
            assert(
                editor,
                cx,
                indoc! {"
                    a.f(one, two, three)<> b
                    a.f(one, two, three)<> b
                    a.f(one, two, three)<> b"},
            );

            // As soon as the last tab stop is reached, snippet state is gone
            editor.move_to_prev_snippet_tabstop(cx);
            assert(
                editor,
                cx,
                indoc! {"
                    a.f(one, two, three)<> b
                    a.f(one, two, three)<> b
                    a.f(one, two, three)<> b"},
            );
        });
    }

    #[gpui::test]
    async fn test_document_format_during_save(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        });

        let fs = FakeFs::new(cx.background().clone());
        fs.insert_file("/file.rs", Default::default()).await;

        let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages().add(Arc::new(language)));
        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
            .await
            .unwrap();

        cx.foreground().start_waiting();
        let fake_server = fake_servers.next().await.unwrap();

        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));
        editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
        assert!(cx.read(|cx| editor.is_dirty(cx)));

        let save = cx.update(|cx| editor.save(project.clone(), cx));
        fake_server
            .handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/file.rs").unwrap()
                );
                assert_eq!(params.options.tab_size, 4);
                Ok(Some(vec![lsp::TextEdit::new(
                    lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(1, 0)),
                    ", ".to_string(),
                )]))
            })
            .next()
            .await;
        cx.foreground().start_waiting();
        save.await.unwrap();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "one, two\nthree\n"
        );
        assert!(!cx.read(|cx| editor.is_dirty(cx)));

        editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
        assert!(cx.read(|cx| editor.is_dirty(cx)));

        // Ensure we can still save even if formatting hangs.
        fake_server.handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/file.rs").unwrap()
            );
            futures::future::pending::<()>().await;
            unreachable!()
        });
        let save = cx.update(|cx| editor.save(project.clone(), cx));
        cx.foreground().advance_clock(items::FORMAT_TIMEOUT);
        cx.foreground().start_waiting();
        save.await.unwrap();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "one\ntwo\nthree\n"
        );
        assert!(!cx.read(|cx| editor.is_dirty(cx)));

        // Set rust language override and assert overriden tabsize is sent to language server
        cx.update(|cx| {
            cx.update_global::<Settings, _, _>(|settings, _| {
                settings.language_overrides.insert(
                    "Rust".into(),
                    LanguageOverride {
                        tab_size: Some(8),
                        ..Default::default()
                    },
                );
            })
        });

        let save = cx.update(|cx| editor.save(project.clone(), cx));
        fake_server
            .handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/file.rs").unwrap()
                );
                assert_eq!(params.options.tab_size, 8);
                Ok(Some(vec![]))
            })
            .next()
            .await;
        cx.foreground().start_waiting();
        save.await.unwrap();
    }

    #[gpui::test]
    async fn test_range_format_during_save(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_range_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        });

        let fs = FakeFs::new(cx.background().clone());
        fs.insert_file("/file.rs", Default::default()).await;

        let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages().add(Arc::new(language)));
        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
            .await
            .unwrap();

        cx.foreground().start_waiting();
        let fake_server = fake_servers.next().await.unwrap();

        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));
        editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
        assert!(cx.read(|cx| editor.is_dirty(cx)));

        let save = cx.update(|cx| editor.save(project.clone(), cx));
        fake_server
            .handle_request::<lsp::request::RangeFormatting, _, _>(move |params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/file.rs").unwrap()
                );
                assert_eq!(params.options.tab_size, 4);
                Ok(Some(vec![lsp::TextEdit::new(
                    lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(1, 0)),
                    ", ".to_string(),
                )]))
            })
            .next()
            .await;
        cx.foreground().start_waiting();
        save.await.unwrap();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "one, two\nthree\n"
        );
        assert!(!cx.read(|cx| editor.is_dirty(cx)));

        editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
        assert!(cx.read(|cx| editor.is_dirty(cx)));

        // Ensure we can still save even if formatting hangs.
        fake_server.handle_request::<lsp::request::RangeFormatting, _, _>(
            move |params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/file.rs").unwrap()
                );
                futures::future::pending::<()>().await;
                unreachable!()
            },
        );
        let save = cx.update(|cx| editor.save(project.clone(), cx));
        cx.foreground().advance_clock(items::FORMAT_TIMEOUT);
        cx.foreground().start_waiting();
        save.await.unwrap();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "one\ntwo\nthree\n"
        );
        assert!(!cx.read(|cx| editor.is_dirty(cx)));

        // Set rust language override and assert overriden tabsize is sent to language server
        cx.update(|cx| {
            cx.update_global::<Settings, _, _>(|settings, _| {
                settings.language_overrides.insert(
                    "Rust".into(),
                    LanguageOverride {
                        tab_size: Some(8),
                        ..Default::default()
                    },
                );
            })
        });

        let save = cx.update(|cx| editor.save(project.clone(), cx));
        fake_server
            .handle_request::<lsp::request::RangeFormatting, _, _>(move |params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/file.rs").unwrap()
                );
                assert_eq!(params.options.tab_size, 8);
                Ok(Some(vec![]))
            })
            .next()
            .await;
        cx.foreground().start_waiting();
        save.await.unwrap();
    }

    #[gpui::test]
    async fn test_completion(cx: &mut gpui::TestAppContext) {
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        let text = "
            one
            two
            three
        "
        .unindent();

        let fs = FakeFs::new(cx.background().clone());
        fs.insert_file("/file.rs", text).await;

        let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages().add(Arc::new(language)));
        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
            .await
            .unwrap();
        let mut fake_server = fake_servers.next().await.unwrap();

        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.project = Some(project);
            editor.change_selections(None, cx, |s| {
                s.select_ranges([Point::new(0, 3)..Point::new(0, 3)])
            });
            editor.handle_input(&Input(".".to_string()), cx);
        });

        handle_completion_request(
            &mut fake_server,
            "/file.rs",
            Point::new(0, 4),
            vec![
                (Point::new(0, 4)..Point::new(0, 4), "first_completion"),
                (Point::new(0, 4)..Point::new(0, 4), "second_completion"),
            ],
        )
        .await;
        editor
            .condition(&cx, |editor, _| editor.context_menu_visible())
            .await;

        let apply_additional_edits = editor.update(cx, |editor, cx| {
            editor.move_down(&MoveDown, cx);
            let apply_additional_edits = editor
                .confirm_completion(&ConfirmCompletion::default(), cx)
                .unwrap();
            assert_eq!(
                editor.text(cx),
                "
                    one.second_completion
                    two
                    three
                "
                .unindent()
            );
            apply_additional_edits
        });

        handle_resolve_completion_request(
            &mut fake_server,
            Some((Point::new(2, 5)..Point::new(2, 5), "\nadditional edit")),
        )
        .await;
        apply_additional_edits.await.unwrap();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "
                one.second_completion
                two
                three
                additional edit
            "
            .unindent()
        );

        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_ranges([
                    Point::new(1, 3)..Point::new(1, 3),
                    Point::new(2, 5)..Point::new(2, 5),
                ])
            });

            editor.handle_input(&Input(" ".to_string()), cx);
            assert!(editor.context_menu.is_none());
            editor.handle_input(&Input("s".to_string()), cx);
            assert!(editor.context_menu.is_none());
        });

        handle_completion_request(
            &mut fake_server,
            "/file.rs",
            Point::new(2, 7),
            vec![
                (Point::new(2, 6)..Point::new(2, 7), "fourth_completion"),
                (Point::new(2, 6)..Point::new(2, 7), "fifth_completion"),
                (Point::new(2, 6)..Point::new(2, 7), "sixth_completion"),
            ],
        )
        .await;
        editor
            .condition(&cx, |editor, _| editor.context_menu_visible())
            .await;

        editor.update(cx, |editor, cx| {
            editor.handle_input(&Input("i".to_string()), cx);
        });

        handle_completion_request(
            &mut fake_server,
            "/file.rs",
            Point::new(2, 8),
            vec![
                (Point::new(2, 6)..Point::new(2, 8), "fourth_completion"),
                (Point::new(2, 6)..Point::new(2, 8), "fifth_completion"),
                (Point::new(2, 6)..Point::new(2, 8), "sixth_completion"),
            ],
        )
        .await;
        editor
            .condition(&cx, |editor, _| editor.context_menu_visible())
            .await;

        let apply_additional_edits = editor.update(cx, |editor, cx| {
            let apply_additional_edits = editor
                .confirm_completion(&ConfirmCompletion::default(), cx)
                .unwrap();
            assert_eq!(
                editor.text(cx),
                "
                    one.second_completion
                    two sixth_completion
                    three sixth_completion
                    additional edit
                "
                .unindent()
            );
            apply_additional_edits
        });
        handle_resolve_completion_request(&mut fake_server, None).await;
        apply_additional_edits.await.unwrap();

        async fn handle_completion_request(
            fake: &mut FakeLanguageServer,
            path: &'static str,
            position: Point,
            completions: Vec<(Range<Point>, &'static str)>,
        ) {
            fake.handle_request::<lsp::request::Completion, _, _>(move |params, _| {
                let completions = completions.clone();
                async move {
                    assert_eq!(
                        params.text_document_position.text_document.uri,
                        lsp::Url::from_file_path(path).unwrap()
                    );
                    assert_eq!(
                        params.text_document_position.position,
                        lsp::Position::new(position.row, position.column)
                    );
                    Ok(Some(lsp::CompletionResponse::Array(
                        completions
                            .iter()
                            .map(|(range, new_text)| lsp::CompletionItem {
                                label: new_text.to_string(),
                                text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                    range: lsp::Range::new(
                                        lsp::Position::new(range.start.row, range.start.column),
                                        lsp::Position::new(range.start.row, range.start.column),
                                    ),
                                    new_text: new_text.to_string(),
                                })),
                                ..Default::default()
                            })
                            .collect(),
                    )))
                }
            })
            .next()
            .await;
        }

        async fn handle_resolve_completion_request(
            fake: &mut FakeLanguageServer,
            edit: Option<(Range<Point>, &'static str)>,
        ) {
            fake.handle_request::<lsp::request::ResolveCompletionItem, _, _>(move |_, _| {
                let edit = edit.clone();
                async move {
                    Ok(lsp::CompletionItem {
                        additional_text_edits: edit.map(|(range, new_text)| {
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(
                                    lsp::Position::new(range.start.row, range.start.column),
                                    lsp::Position::new(range.end.row, range.end.column),
                                ),
                                new_text.to_string(),
                            )]
                        }),
                        ..Default::default()
                    })
                }
            })
            .next()
            .await;
        }
    }

    #[gpui::test]
    async fn test_hover_popover(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        // Basic hover delays and then pops without moving the mouse
        cx.set_state(indoc! {"
            fn |test()
                println!();"});
        let hover_point = cx.display_point(indoc! {"
            fn test()
                print|ln!();"});
        cx.update_editor(|editor, cx| {
            hover_at(
                editor,
                &HoverAt {
                    point: Some(hover_point),
                },
                cx,
            )
        });
        assert!(!cx.editor(|editor, _| editor.hover_state.visible()));

        // After delay, hover should be visible.
        let symbol_range = cx.lsp_range(indoc! {"
            fn test()
                [println!]();"});
        cx.handle_request::<lsp::request::HoverRequest, _>(move |_| {
            Some(lsp::Hover {
                contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                    kind: lsp::MarkupKind::Markdown,
                    value: indoc! {"
                        # Some basic docs
                        Some test documentation"}
                    .to_string(),
                }),
                range: Some(symbol_range),
            })
        })
        .await;
        cx.foreground()
            .advance_clock(Duration::from_millis(HOVER_DELAY_MILLIS + 100));

        cx.editor(|editor, _| {
            assert!(editor.hover_state.visible());
            assert_eq!(
                editor.hover_state.popover.clone().unwrap().contents,
                vec![
                    HoverBlock {
                        text: "Some basic docs".to_string(),
                        language: None
                    },
                    HoverBlock {
                        text: "Some test documentation".to_string(),
                        language: None
                    }
                ]
            )
        });

        // Mouse moved with no hover response dismisses
        let hover_point = cx.display_point(indoc! {"
            fn te|st()
                println!();"});
        cx.update_editor(|editor, cx| {
            hover_at(
                editor,
                &HoverAt {
                    point: Some(hover_point),
                },
                cx,
            )
        });
        cx.handle_request::<lsp::request::HoverRequest, _>(move |_| None)
            .await;
        cx.foreground().run_until_parked();
        cx.editor(|editor, _| {
            assert!(!editor.hover_state.visible());
        });
        cx.foreground()
            .advance_clock(Duration::from_millis(HOVER_GRACE_MILLIS + 100));

        // Hover with keyboard has no delay
        cx.set_state(indoc! {"
            f|n test()
                println!();"});
        cx.update_editor(|editor, cx| hover(editor, &hover_popover::Hover, cx));
        let symbol_range = cx.lsp_range(indoc! {"
            [fn] test()
                println!();"});
        cx.handle_request::<lsp::request::HoverRequest, _>(move |_| {
            Some(lsp::Hover {
                contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                    kind: lsp::MarkupKind::Markdown,
                    value: indoc! {"
                        # Some other basic docs
                        Some other test documentation"}
                    .to_string(),
                }),
                range: Some(symbol_range),
            })
        })
        .await;
        cx.foreground().run_until_parked();
        cx.editor(|editor, _| {
            assert!(editor.hover_state.visible());
            assert_eq!(
                editor.hover_state.popover.clone().unwrap().contents,
                vec![
                    HoverBlock {
                        text: "Some other basic docs".to_string(),
                        language: None
                    },
                    HoverBlock {
                        text: "Some other test documentation".to_string(),
                        language: None
                    }
                ]
            )
        });

        // Open hover popover disables delay
        let hover_point = cx.display_point(indoc! {"
            fn test()
                print|ln!();"});
        cx.update_editor(|editor, cx| {
            hover_at(
                editor,
                &HoverAt {
                    point: Some(hover_point),
                },
                cx,
            )
        });

        let symbol_range = cx.lsp_range(indoc! {"
            fn test()
                [println!]();"});
        cx.handle_request::<lsp::request::HoverRequest, _>(move |_| {
            Some(lsp::Hover {
                contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                    kind: lsp::MarkupKind::Markdown,
                    value: indoc! {"
                        # Some third basic docs
                        Some third test documentation"}
                    .to_string(),
                }),
                range: Some(symbol_range),
            })
        })
        .await;
        cx.foreground().run_until_parked();
        // No delay as the popover is already visible

        cx.editor(|editor, _| {
            assert!(editor.hover_state.visible());
            assert_eq!(
                editor.hover_state.popover.clone().unwrap().contents,
                vec![
                    HoverBlock {
                        text: "Some third basic docs".to_string(),
                        language: None
                    },
                    HoverBlock {
                        text: "Some third test documentation".to_string(),
                        language: None
                    }
                ]
            )
        });
    }

    #[gpui::test]
    async fn test_toggle_comment(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));
        let language = Arc::new(Language::new(
            LanguageConfig {
                line_comment: Some("// ".to_string()),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ));

        let text = "
            fn a() {
                //b();
                // c();
                //  d();
            }
        "
        .unindent();

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, view) = cx.add_window(|cx| build_editor(buffer, cx));

        view.update(cx, |editor, cx| {
            // If multiple selections intersect a line, the line is only
            // toggled once.
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(1, 3)..DisplayPoint::new(2, 3),
                    DisplayPoint::new(3, 5)..DisplayPoint::new(3, 6),
                ])
            });
            editor.toggle_comments(&ToggleComments, cx);
            assert_eq!(
                editor.text(cx),
                "
                    fn a() {
                        b();
                        c();
                         d();
                    }
                "
                .unindent()
            );

            // The comment prefix is inserted at the same column for every line
            // in a selection.
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 3)..DisplayPoint::new(3, 6)])
            });
            editor.toggle_comments(&ToggleComments, cx);
            assert_eq!(
                editor.text(cx),
                "
                    fn a() {
                        // b();
                        // c();
                        //  d();
                    }
                "
                .unindent()
            );

            // If a selection ends at the beginning of a line, that line is not toggled.
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(2, 0)..DisplayPoint::new(3, 0)])
            });
            editor.toggle_comments(&ToggleComments, cx);
            assert_eq!(
                editor.text(cx),
                "
                        fn a() {
                            // b();
                            c();
                            //  d();
                        }
                    "
                .unindent()
            );
        });
    }

    #[gpui::test]
    fn test_editing_disjoint_excerpts(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            multibuffer.push_excerpts(
                buffer.clone(),
                [
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(0, 4),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(1, 0)..Point::new(1, 4),
                        primary: None,
                    },
                ],
                cx,
            );
            multibuffer
        });

        assert_eq!(multibuffer.read(cx).read(cx).text(), "aaaa\nbbbb");

        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(multibuffer, cx));
        view.update(cx, |view, cx| {
            assert_eq!(view.text(cx), "aaaa\nbbbb");
            view.change_selections(None, cx, |s| {
                s.select_ranges([
                    Point::new(0, 0)..Point::new(0, 0),
                    Point::new(1, 0)..Point::new(1, 0),
                ])
            });

            view.handle_input(&Input("X".to_string()), cx);
            assert_eq!(view.text(cx), "Xaaaa\nXbbbb");
            assert_eq!(
                view.selections.ranges(cx),
                [
                    Point::new(0, 1)..Point::new(0, 1),
                    Point::new(1, 1)..Point::new(1, 1),
                ]
            )
        });
    }

    #[gpui::test]
    fn test_editing_overlapping_excerpts(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let (initial_text, excerpt_ranges) = marked_text_ranges(indoc! {"
                [aaaa
                (bbbb]
                cccc)"});
        let excerpt_ranges = excerpt_ranges.into_iter().map(|context| ExcerptRange {
            context,
            primary: None,
        });
        let buffer = cx.add_model(|cx| Buffer::new(0, initial_text, cx));
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            multibuffer.push_excerpts(buffer, excerpt_ranges, cx);
            multibuffer
        });

        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(multibuffer, cx));
        view.update(cx, |view, cx| {
            let (expected_text, selection_ranges) = marked_text_ranges(indoc! {"
                aaaa
                b|bbb
                b|bb|b
                cccc"});
            assert_eq!(view.text(cx), expected_text);
            view.change_selections(None, cx, |s| s.select_ranges(selection_ranges));

            view.handle_input(&Input("X".to_string()), cx);

            let (expected_text, expected_selections) = marked_text_ranges(indoc! {"
                aaaa
                bX|bbXb
                bX|bbX|b
                cccc"});
            assert_eq!(view.text(cx), expected_text);
            assert_eq!(view.selections.ranges(cx), expected_selections);

            view.newline(&Newline, cx);
            let (expected_text, expected_selections) = marked_text_ranges(indoc! {"
                aaaa
                bX
                |bbX
                b
                bX
                |bbX
                |b
                cccc"});
            assert_eq!(view.text(cx), expected_text);
            assert_eq!(view.selections.ranges(cx), expected_selections);
        });
    }

    #[gpui::test]
    fn test_refresh_selections(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
        let mut excerpt1_id = None;
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            excerpt1_id = multibuffer
                .push_excerpts(
                    buffer.clone(),
                    [
                        ExcerptRange {
                            context: Point::new(0, 0)..Point::new(1, 4),
                            primary: None,
                        },
                        ExcerptRange {
                            context: Point::new(1, 0)..Point::new(2, 4),
                            primary: None,
                        },
                    ],
                    cx,
                )
                .into_iter()
                .next();
            multibuffer
        });
        assert_eq!(
            multibuffer.read(cx).read(cx).text(),
            "aaaa\nbbbb\nbbbb\ncccc"
        );
        let (_, editor) = cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(multibuffer.clone(), cx);
            let snapshot = editor.snapshot(cx);
            editor.change_selections(None, cx, |s| {
                s.select_ranges([Point::new(1, 3)..Point::new(1, 3)])
            });
            editor.begin_selection(Point::new(2, 1).to_display_point(&snapshot), true, 1, cx);
            assert_eq!(
                editor.selections.ranges(cx),
                [
                    Point::new(1, 3)..Point::new(1, 3),
                    Point::new(2, 1)..Point::new(2, 1),
                ]
            );
            editor
        });

        // Refreshing selections is a no-op when excerpts haven't changed.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.refresh();
            });
            assert_eq!(
                editor.selections.ranges(cx),
                [
                    Point::new(1, 3)..Point::new(1, 3),
                    Point::new(2, 1)..Point::new(2, 1),
                ]
            );
        });

        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.remove_excerpts([&excerpt1_id.unwrap()], cx);
        });
        editor.update(cx, |editor, cx| {
            // Removing an excerpt causes the first selection to become degenerate.
            assert_eq!(
                editor.selections.ranges(cx),
                [
                    Point::new(0, 0)..Point::new(0, 0),
                    Point::new(0, 1)..Point::new(0, 1)
                ]
            );

            // Refreshing selections will relocate the first selection to the original buffer
            // location.
            editor.change_selections(None, cx, |s| {
                s.refresh();
            });
            assert_eq!(
                editor.selections.ranges(cx),
                [
                    Point::new(0, 1)..Point::new(0, 1),
                    Point::new(0, 3)..Point::new(0, 3)
                ]
            );
            assert!(editor.selections.pending_anchor().is_some());
        });
    }

    #[gpui::test]
    fn test_refresh_selections_while_selecting_with_mouse(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
        let mut excerpt1_id = None;
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            excerpt1_id = multibuffer
                .push_excerpts(
                    buffer.clone(),
                    [
                        ExcerptRange {
                            context: Point::new(0, 0)..Point::new(1, 4),
                            primary: None,
                        },
                        ExcerptRange {
                            context: Point::new(1, 0)..Point::new(2, 4),
                            primary: None,
                        },
                    ],
                    cx,
                )
                .into_iter()
                .next();
            multibuffer
        });
        assert_eq!(
            multibuffer.read(cx).read(cx).text(),
            "aaaa\nbbbb\nbbbb\ncccc"
        );
        let (_, editor) = cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(multibuffer.clone(), cx);
            let snapshot = editor.snapshot(cx);
            editor.begin_selection(Point::new(1, 3).to_display_point(&snapshot), false, 1, cx);
            assert_eq!(
                editor.selections.ranges(cx),
                [Point::new(1, 3)..Point::new(1, 3)]
            );
            editor
        });

        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.remove_excerpts([&excerpt1_id.unwrap()], cx);
        });
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.selections.ranges(cx),
                [Point::new(0, 0)..Point::new(0, 0)]
            );

            // Ensure we don't panic when selections are refreshed and that the pending selection is finalized.
            editor.change_selections(None, cx, |s| {
                s.refresh();
            });
            assert_eq!(
                editor.selections.ranges(cx),
                [Point::new(0, 3)..Point::new(0, 3)]
            );
            assert!(editor.selections.pending_anchor().is_some());
        });
    }

    #[gpui::test]
    async fn test_extra_newline_insertion(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));
        let language = Arc::new(Language::new(
            LanguageConfig {
                brackets: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "/* ".to_string(),
                        end: " */".to_string(),
                        close: true,
                        newline: true,
                    },
                ],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ));

        let text = concat!(
            "{   }\n",     // Suppress rustfmt
            "  x\n",       //
            "  /*   */\n", //
            "x\n",         //
            "{{} }\n",     //
        );

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, view) = cx.add_window(|cx| build_editor(buffer, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
            .await;

        view.update(cx, |view, cx| {
            view.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(2, 5)..DisplayPoint::new(2, 5),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 4),
                ])
            });
            view.newline(&Newline, cx);

            assert_eq!(
                view.buffer().read(cx).read(cx).text(),
                concat!(
                    "{ \n",    // Suppress rustfmt
                    "\n",      //
                    "}\n",     //
                    "  x\n",   //
                    "  /* \n", //
                    "  \n",    //
                    "  */\n",  //
                    "x\n",     //
                    "{{} \n",  //
                    "}\n",     //
                )
            );
        });
    }

    #[gpui::test]
    fn test_highlighted_ranges(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple(&sample_text(16, 8, 'a'), cx);

        cx.set_global(Settings::test(cx));
        let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

        editor.update(cx, |editor, cx| {
            struct Type1;
            struct Type2;

            let buffer = buffer.read(cx).snapshot(cx);

            let anchor_range = |range: Range<Point>| {
                buffer.anchor_after(range.start)..buffer.anchor_after(range.end)
            };

            editor.highlight_background::<Type1>(
                vec![
                    anchor_range(Point::new(2, 1)..Point::new(2, 3)),
                    anchor_range(Point::new(4, 2)..Point::new(4, 4)),
                    anchor_range(Point::new(6, 3)..Point::new(6, 5)),
                    anchor_range(Point::new(8, 4)..Point::new(8, 6)),
                ],
                |_| Color::red(),
                cx,
            );
            editor.highlight_background::<Type2>(
                vec![
                    anchor_range(Point::new(3, 2)..Point::new(3, 5)),
                    anchor_range(Point::new(5, 3)..Point::new(5, 6)),
                    anchor_range(Point::new(7, 4)..Point::new(7, 7)),
                    anchor_range(Point::new(9, 5)..Point::new(9, 8)),
                ],
                |_| Color::green(),
                cx,
            );

            let snapshot = editor.snapshot(cx);
            let mut highlighted_ranges = editor.background_highlights_in_range(
                anchor_range(Point::new(3, 4)..Point::new(7, 4)),
                &snapshot,
                cx.global::<Settings>().theme.as_ref(),
            );
            // Enforce a consistent ordering based on color without relying on the ordering of the
            // highlight's `TypeId` which is non-deterministic.
            highlighted_ranges.sort_unstable_by_key(|(_, color)| *color);
            assert_eq!(
                highlighted_ranges,
                &[
                    (
                        DisplayPoint::new(3, 2)..DisplayPoint::new(3, 5),
                        Color::green(),
                    ),
                    (
                        DisplayPoint::new(5, 3)..DisplayPoint::new(5, 6),
                        Color::green(),
                    ),
                    (
                        DisplayPoint::new(4, 2)..DisplayPoint::new(4, 4),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(6, 3)..DisplayPoint::new(6, 5),
                        Color::red(),
                    ),
                ]
            );
            assert_eq!(
                editor.background_highlights_in_range(
                    anchor_range(Point::new(5, 6)..Point::new(6, 4)),
                    &snapshot,
                    cx.global::<Settings>().theme.as_ref(),
                ),
                &[(
                    DisplayPoint::new(6, 3)..DisplayPoint::new(6, 5),
                    Color::red(),
                )]
            );
        });
    }

    #[gpui::test]
    fn test_following(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple(&sample_text(16, 8, 'a'), cx);

        cx.set_global(Settings::test(cx));

        let (_, leader) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));
        let (_, follower) = cx.add_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(RectF::from_points(vec2f(0., 0.), vec2f(10., 80.))),
                ..Default::default()
            },
            |cx| build_editor(buffer.clone(), cx),
        );

        let pending_update = Rc::new(RefCell::new(None));
        follower.update(cx, {
            let update = pending_update.clone();
            |_, cx| {
                cx.subscribe(&leader, move |_, leader, event, cx| {
                    leader
                        .read(cx)
                        .add_event_to_update_proto(event, &mut *update.borrow_mut(), cx);
                })
                .detach();
            }
        });

        // Update the selections only
        leader.update(cx, |leader, cx| {
            leader.change_selections(None, cx, |s| s.select_ranges([1..1]));
        });
        follower.update(cx, |follower, cx| {
            follower
                .apply_update_proto(pending_update.borrow_mut().take().unwrap(), cx)
                .unwrap();
        });
        assert_eq!(follower.read(cx).selections.ranges(cx), vec![1..1]);

        // Update the scroll position only
        leader.update(cx, |leader, cx| {
            leader.set_scroll_position(vec2f(1.5, 3.5), cx);
        });
        follower.update(cx, |follower, cx| {
            follower
                .apply_update_proto(pending_update.borrow_mut().take().unwrap(), cx)
                .unwrap();
        });
        assert_eq!(
            follower.update(cx, |follower, cx| follower.scroll_position(cx)),
            vec2f(1.5, 3.5)
        );

        // Update the selections and scroll position
        leader.update(cx, |leader, cx| {
            leader.change_selections(None, cx, |s| s.select_ranges([0..0]));
            leader.request_autoscroll(Autoscroll::Newest, cx);
            leader.set_scroll_position(vec2f(1.5, 3.5), cx);
        });
        follower.update(cx, |follower, cx| {
            let initial_scroll_position = follower.scroll_position(cx);
            follower
                .apply_update_proto(pending_update.borrow_mut().take().unwrap(), cx)
                .unwrap();
            assert_eq!(follower.scroll_position(cx), initial_scroll_position);
            assert!(follower.autoscroll_request.is_some());
        });
        assert_eq!(follower.read(cx).selections.ranges(cx), vec![0..0]);

        // Creating a pending selection that precedes another selection
        leader.update(cx, |leader, cx| {
            leader.change_selections(None, cx, |s| s.select_ranges([1..1]));
            leader.begin_selection(DisplayPoint::new(0, 0), true, 1, cx);
        });
        follower.update(cx, |follower, cx| {
            follower
                .apply_update_proto(pending_update.borrow_mut().take().unwrap(), cx)
                .unwrap();
        });
        assert_eq!(follower.read(cx).selections.ranges(cx), vec![0..0, 1..1]);

        // Extend the pending selection so that it surrounds another selection
        leader.update(cx, |leader, cx| {
            leader.extend_selection(DisplayPoint::new(0, 2), 1, cx);
        });
        follower.update(cx, |follower, cx| {
            follower
                .apply_update_proto(pending_update.borrow_mut().take().unwrap(), cx)
                .unwrap();
        });
        assert_eq!(follower.read(cx).selections.ranges(cx), vec![0..2]);
    }

    #[test]
    fn test_combine_syntax_and_fuzzy_match_highlights() {
        let string = "abcdefghijklmnop";
        let syntax_ranges = [
            (
                0..3,
                HighlightStyle {
                    color: Some(Color::red()),
                    ..Default::default()
                },
            ),
            (
                4..8,
                HighlightStyle {
                    color: Some(Color::green()),
                    ..Default::default()
                },
            ),
        ];
        let match_indices = [4, 6, 7, 8];
        assert_eq!(
            combine_syntax_and_fuzzy_match_highlights(
                &string,
                Default::default(),
                syntax_ranges.into_iter(),
                &match_indices,
            ),
            &[
                (
                    0..3,
                    HighlightStyle {
                        color: Some(Color::red()),
                        ..Default::default()
                    },
                ),
                (
                    4..5,
                    HighlightStyle {
                        color: Some(Color::green()),
                        weight: Some(fonts::Weight::BOLD),
                        ..Default::default()
                    },
                ),
                (
                    5..6,
                    HighlightStyle {
                        color: Some(Color::green()),
                        ..Default::default()
                    },
                ),
                (
                    6..8,
                    HighlightStyle {
                        color: Some(Color::green()),
                        weight: Some(fonts::Weight::BOLD),
                        ..Default::default()
                    },
                ),
                (
                    8..9,
                    HighlightStyle {
                        weight: Some(fonts::Weight::BOLD),
                        ..Default::default()
                    },
                ),
            ]
        );
    }

    fn empty_range(row: usize, column: usize) -> Range<DisplayPoint> {
        let point = DisplayPoint::new(row as u32, column as u32);
        point..point
    }

    fn assert_selection_ranges(
        marked_text: &str,
        selection_marker_pairs: Vec<(char, char)>,
        view: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) {
        let snapshot = view.snapshot(cx).display_snapshot;
        let mut marker_chars = Vec::new();
        for (start, end) in selection_marker_pairs.iter() {
            marker_chars.push(*start);
            marker_chars.push(*end);
        }
        let (_, markers) = marked_text_by(marked_text, marker_chars);
        let asserted_ranges: Vec<Range<DisplayPoint>> = selection_marker_pairs
            .iter()
            .map(|(start, end)| {
                let start = markers.get(start).unwrap()[0].to_display_point(&snapshot);
                let end = markers.get(end).unwrap()[0].to_display_point(&snapshot);
                start..end
            })
            .collect();
        assert_eq!(
            view.selections.display_ranges(cx),
            &asserted_ranges[..],
            "Assert selections are {}",
            marked_text
        );
    }
}

trait RangeExt<T> {
    fn sorted(&self) -> Range<T>;
    fn to_inclusive(&self) -> RangeInclusive<T>;
}

impl<T: Ord + Clone> RangeExt<T> for Range<T> {
    fn sorted(&self) -> Self {
        cmp::min(&self.start, &self.end).clone()..cmp::max(&self.start, &self.end).clone()
    }

    fn to_inclusive(&self) -> RangeInclusive<T> {
        self.start.clone()..=self.end.clone()
    }
}

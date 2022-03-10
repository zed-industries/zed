pub mod display_map;
mod element;
pub mod items;
pub mod movement;
mod multi_buffer;

#[cfg(test)]
mod test;

use aho_corasick::AhoCorasick;
use anyhow::Result;
use clock::ReplicaId;
use collections::{BTreeMap, Bound, HashMap, HashSet};
pub use display_map::DisplayPoint;
use display_map::*;
pub use element::*;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    action,
    color::Color,
    elements::*,
    executor,
    fonts::{self, HighlightStyle, TextStyle},
    geometry::vector::{vec2f, Vector2F},
    keymap::Binding,
    platform::CursorStyle,
    text_layout, AppContext, AsyncAppContext, ClipboardItem, Element, ElementBox, Entity,
    ModelHandle, MutableAppContext, RenderContext, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use items::{BufferItemHandle, MultiBufferItemHandle};
use itertools::Itertools as _;
pub use language::{char_kind, CharKind};
use language::{
    BracketPair, Buffer, CodeAction, CodeLabel, Completion, Diagnostic, DiagnosticSeverity,
    Language, OffsetRangeExt, Point, Selection, SelectionGoal, TransactionId,
};
use multi_buffer::MultiBufferChunks;
pub use multi_buffer::{
    Anchor, AnchorRangeExt, ExcerptId, MultiBuffer, MultiBufferSnapshot, ToOffset, ToPoint,
};
use ordered_float::OrderedFloat;
use postage::watch;
use project::{Project, ProjectTransaction};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smol::Timer;
use snippet::Snippet;
use std::{
    any::TypeId,
    cmp::{self, Ordering, Reverse},
    iter::{self, FromIterator},
    mem,
    ops::{Deref, DerefMut, Range, RangeInclusive, Sub},
    sync::Arc,
    time::{Duration, Instant},
};
pub use sum_tree::Bias;
use text::rope::TextDimension;
use theme::DiagnosticStyle;
use util::{post_inc, ResultExt, TryFutureExt};
use workspace::{settings, ItemNavHistory, PathOpener, Settings, Workspace};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);
const MAX_LINE_LEN: usize = 1024;
const MIN_NAVIGATION_HISTORY_ROW_DELTA: i64 = 10;

action!(Cancel);
action!(Backspace);
action!(Delete);
action!(Input, String);
action!(Newline);
action!(Tab);
action!(Outdent);
action!(DeleteLine);
action!(DeleteToPreviousWordBoundary);
action!(DeleteToNextWordBoundary);
action!(DeleteToBeginningOfLine);
action!(DeleteToEndOfLine);
action!(CutToEndOfLine);
action!(DuplicateLine);
action!(MoveLineUp);
action!(MoveLineDown);
action!(Cut);
action!(Copy);
action!(Paste);
action!(Undo);
action!(Redo);
action!(MoveUp);
action!(MoveDown);
action!(MoveLeft);
action!(MoveRight);
action!(MoveToPreviousWordBoundary);
action!(MoveToNextWordBoundary);
action!(MoveToBeginningOfLine);
action!(MoveToEndOfLine);
action!(MoveToBeginning);
action!(MoveToEnd);
action!(SelectUp);
action!(SelectDown);
action!(SelectLeft);
action!(SelectRight);
action!(SelectToPreviousWordBoundary);
action!(SelectToNextWordBoundary);
action!(SelectToBeginningOfLine, bool);
action!(SelectToEndOfLine, bool);
action!(SelectToBeginning);
action!(SelectToEnd);
action!(SelectAll);
action!(SelectLine);
action!(SplitSelectionIntoLines);
action!(AddSelectionAbove);
action!(AddSelectionBelow);
action!(SelectNext, bool);
action!(ToggleComments);
action!(SelectLargerSyntaxNode);
action!(SelectSmallerSyntaxNode);
action!(MoveToEnclosingBracket);
action!(ShowNextDiagnostic);
action!(GoToDefinition);
action!(FindAllReferences);
action!(Rename);
action!(ConfirmRename);
action!(PageUp);
action!(PageDown);
action!(Fold);
action!(Unfold);
action!(FoldSelectedRanges);
action!(Scroll, Vector2F);
action!(Select, SelectPhase);
action!(ShowCompletions);
action!(ToggleCodeActions, bool);
action!(ConfirmCompletion, Option<usize>);
action!(ConfirmCodeAction, Option<usize>);
action!(OpenExcerpts);

pub fn init(cx: &mut MutableAppContext, path_openers: &mut Vec<Box<dyn PathOpener>>) {
    path_openers.push(Box::new(items::BufferOpener));
    cx.add_bindings(vec![
        Binding::new("escape", Cancel, Some("Editor")),
        Binding::new("backspace", Backspace, Some("Editor")),
        Binding::new("ctrl-h", Backspace, Some("Editor")),
        Binding::new("delete", Delete, Some("Editor")),
        Binding::new("ctrl-d", Delete, Some("Editor")),
        Binding::new("enter", Newline, Some("Editor && mode == full")),
        Binding::new(
            "alt-enter",
            Input("\n".into()),
            Some("Editor && mode == auto_height"),
        ),
        Binding::new(
            "enter",
            ConfirmCompletion(None),
            Some("Editor && showing_completions"),
        ),
        Binding::new(
            "enter",
            ConfirmCodeAction(None),
            Some("Editor && showing_code_actions"),
        ),
        Binding::new("enter", ConfirmRename, Some("Editor && renaming")),
        Binding::new("tab", Tab, Some("Editor")),
        Binding::new(
            "tab",
            ConfirmCompletion(None),
            Some("Editor && showing_completions"),
        ),
        Binding::new("shift-tab", Outdent, Some("Editor")),
        Binding::new("ctrl-shift-K", DeleteLine, Some("Editor")),
        Binding::new(
            "alt-backspace",
            DeleteToPreviousWordBoundary,
            Some("Editor"),
        ),
        Binding::new("alt-h", DeleteToPreviousWordBoundary, Some("Editor")),
        Binding::new("alt-delete", DeleteToNextWordBoundary, Some("Editor")),
        Binding::new("alt-d", DeleteToNextWordBoundary, Some("Editor")),
        Binding::new("cmd-backspace", DeleteToBeginningOfLine, Some("Editor")),
        Binding::new("cmd-delete", DeleteToEndOfLine, Some("Editor")),
        Binding::new("ctrl-k", CutToEndOfLine, Some("Editor")),
        Binding::new("cmd-shift-D", DuplicateLine, Some("Editor")),
        Binding::new("ctrl-cmd-up", MoveLineUp, Some("Editor")),
        Binding::new("ctrl-cmd-down", MoveLineDown, Some("Editor")),
        Binding::new("cmd-x", Cut, Some("Editor")),
        Binding::new("cmd-c", Copy, Some("Editor")),
        Binding::new("cmd-v", Paste, Some("Editor")),
        Binding::new("cmd-z", Undo, Some("Editor")),
        Binding::new("cmd-shift-Z", Redo, Some("Editor")),
        Binding::new("up", MoveUp, Some("Editor")),
        Binding::new("down", MoveDown, Some("Editor")),
        Binding::new("left", MoveLeft, Some("Editor")),
        Binding::new("right", MoveRight, Some("Editor")),
        Binding::new("ctrl-p", MoveUp, Some("Editor")),
        Binding::new("ctrl-n", MoveDown, Some("Editor")),
        Binding::new("ctrl-b", MoveLeft, Some("Editor")),
        Binding::new("ctrl-f", MoveRight, Some("Editor")),
        Binding::new("alt-left", MoveToPreviousWordBoundary, Some("Editor")),
        Binding::new("alt-b", MoveToPreviousWordBoundary, Some("Editor")),
        Binding::new("alt-right", MoveToNextWordBoundary, Some("Editor")),
        Binding::new("alt-f", MoveToNextWordBoundary, Some("Editor")),
        Binding::new("cmd-left", MoveToBeginningOfLine, Some("Editor")),
        Binding::new("ctrl-a", MoveToBeginningOfLine, Some("Editor")),
        Binding::new("cmd-right", MoveToEndOfLine, Some("Editor")),
        Binding::new("ctrl-e", MoveToEndOfLine, Some("Editor")),
        Binding::new("cmd-up", MoveToBeginning, Some("Editor")),
        Binding::new("cmd-down", MoveToEnd, Some("Editor")),
        Binding::new("shift-up", SelectUp, Some("Editor")),
        Binding::new("ctrl-shift-P", SelectUp, Some("Editor")),
        Binding::new("shift-down", SelectDown, Some("Editor")),
        Binding::new("ctrl-shift-N", SelectDown, Some("Editor")),
        Binding::new("shift-left", SelectLeft, Some("Editor")),
        Binding::new("ctrl-shift-B", SelectLeft, Some("Editor")),
        Binding::new("shift-right", SelectRight, Some("Editor")),
        Binding::new("ctrl-shift-F", SelectRight, Some("Editor")),
        Binding::new(
            "alt-shift-left",
            SelectToPreviousWordBoundary,
            Some("Editor"),
        ),
        Binding::new("alt-shift-B", SelectToPreviousWordBoundary, Some("Editor")),
        Binding::new("alt-shift-right", SelectToNextWordBoundary, Some("Editor")),
        Binding::new("alt-shift-F", SelectToNextWordBoundary, Some("Editor")),
        Binding::new(
            "cmd-shift-left",
            SelectToBeginningOfLine(true),
            Some("Editor"),
        ),
        Binding::new(
            "ctrl-shift-A",
            SelectToBeginningOfLine(true),
            Some("Editor"),
        ),
        Binding::new("cmd-shift-right", SelectToEndOfLine(true), Some("Editor")),
        Binding::new("ctrl-shift-E", SelectToEndOfLine(true), Some("Editor")),
        Binding::new("cmd-shift-up", SelectToBeginning, Some("Editor")),
        Binding::new("cmd-shift-down", SelectToEnd, Some("Editor")),
        Binding::new("cmd-a", SelectAll, Some("Editor")),
        Binding::new("cmd-l", SelectLine, Some("Editor")),
        Binding::new("cmd-shift-L", SplitSelectionIntoLines, Some("Editor")),
        Binding::new("cmd-alt-up", AddSelectionAbove, Some("Editor")),
        Binding::new("cmd-ctrl-p", AddSelectionAbove, Some("Editor")),
        Binding::new("cmd-alt-down", AddSelectionBelow, Some("Editor")),
        Binding::new("cmd-ctrl-n", AddSelectionBelow, Some("Editor")),
        Binding::new("cmd-d", SelectNext(false), Some("Editor")),
        Binding::new("cmd-k cmd-d", SelectNext(true), Some("Editor")),
        Binding::new("cmd-/", ToggleComments, Some("Editor")),
        Binding::new("alt-up", SelectLargerSyntaxNode, Some("Editor")),
        Binding::new("ctrl-w", SelectLargerSyntaxNode, Some("Editor")),
        Binding::new("alt-down", SelectSmallerSyntaxNode, Some("Editor")),
        Binding::new("ctrl-shift-W", SelectSmallerSyntaxNode, Some("Editor")),
        Binding::new("f8", ShowNextDiagnostic, Some("Editor")),
        Binding::new("f2", Rename, Some("Editor")),
        Binding::new("f12", GoToDefinition, Some("Editor")),
        Binding::new("alt-shift-f12", FindAllReferences, Some("Editor")),
        Binding::new("ctrl-m", MoveToEnclosingBracket, Some("Editor")),
        Binding::new("pageup", PageUp, Some("Editor")),
        Binding::new("pagedown", PageDown, Some("Editor")),
        Binding::new("alt-cmd-[", Fold, Some("Editor")),
        Binding::new("alt-cmd-]", Unfold, Some("Editor")),
        Binding::new("alt-cmd-f", FoldSelectedRanges, Some("Editor")),
        Binding::new("ctrl-space", ShowCompletions, Some("Editor")),
        Binding::new("cmd-.", ToggleCodeActions(false), Some("Editor")),
        Binding::new("alt-enter", OpenExcerpts, Some("Editor")),
    ]);

    cx.add_action(Editor::open_new);
    cx.add_action(|this: &mut Editor, action: &Scroll, cx| this.set_scroll_position(action.0, cx));
    cx.add_action(Editor::select);
    cx.add_action(Editor::cancel);
    cx.add_action(Editor::handle_input);
    cx.add_action(Editor::newline);
    cx.add_action(Editor::backspace);
    cx.add_action(Editor::delete);
    cx.add_action(Editor::tab);
    cx.add_action(Editor::outdent);
    cx.add_action(Editor::delete_line);
    cx.add_action(Editor::delete_to_previous_word_boundary);
    cx.add_action(Editor::delete_to_next_word_boundary);
    cx.add_action(Editor::delete_to_beginning_of_line);
    cx.add_action(Editor::delete_to_end_of_line);
    cx.add_action(Editor::cut_to_end_of_line);
    cx.add_action(Editor::duplicate_line);
    cx.add_action(Editor::move_line_up);
    cx.add_action(Editor::move_line_down);
    cx.add_action(Editor::cut);
    cx.add_action(Editor::copy);
    cx.add_action(Editor::paste);
    cx.add_action(Editor::undo);
    cx.add_action(Editor::redo);
    cx.add_action(Editor::move_up);
    cx.add_action(Editor::move_down);
    cx.add_action(Editor::move_left);
    cx.add_action(Editor::move_right);
    cx.add_action(Editor::move_to_previous_word_boundary);
    cx.add_action(Editor::move_to_next_word_boundary);
    cx.add_action(Editor::move_to_beginning_of_line);
    cx.add_action(Editor::move_to_end_of_line);
    cx.add_action(Editor::move_to_beginning);
    cx.add_action(Editor::move_to_end);
    cx.add_action(Editor::select_up);
    cx.add_action(Editor::select_down);
    cx.add_action(Editor::select_left);
    cx.add_action(Editor::select_right);
    cx.add_action(Editor::select_to_previous_word_boundary);
    cx.add_action(Editor::select_to_next_word_boundary);
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
    cx.add_action(Editor::show_next_diagnostic);
    cx.add_action(Editor::go_to_definition);
    cx.add_action(Editor::page_up);
    cx.add_action(Editor::page_down);
    cx.add_action(Editor::fold);
    cx.add_action(Editor::unfold);
    cx.add_action(Editor::fold_selected_ranges);
    cx.add_action(Editor::show_completions);
    cx.add_action(Editor::toggle_code_actions);
    cx.add_action(Editor::open_excerpts);
    cx.add_async_action(Editor::confirm_completion);
    cx.add_async_action(Editor::confirm_code_action);
    cx.add_async_action(Editor::rename);
    cx.add_async_action(Editor::confirm_rename);
    cx.add_async_action(Editor::find_all_references);
}

trait SelectionExt {
    fn offset_range(&self, buffer: &MultiBufferSnapshot) -> Range<usize>;
    fn point_range(&self, buffer: &MultiBufferSnapshot) -> Range<Point>;
    fn display_range(&self, map: &DisplaySnapshot) -> Range<DisplayPoint>;
    fn spanned_rows(&self, include_end_if_at_line_start: bool, map: &DisplaySnapshot)
        -> Range<u32>;
}

trait InvalidationRegion {
    fn ranges(&self) -> &[Range<Anchor>];
}

#[derive(Clone, Debug)]
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

pub struct Editor {
    handle: WeakViewHandle<Self>,
    buffer: ModelHandle<MultiBuffer>,
    display_map: ModelHandle<DisplayMap>,
    next_selection_id: usize,
    selections: Arc<[Selection<Anchor>]>,
    pending_selection: Option<PendingSelection>,
    columnar_selection_tail: Option<Anchor>,
    add_selections_state: Option<AddSelectionsState>,
    select_next_state: Option<SelectNextState>,
    selection_history:
        HashMap<TransactionId, (Arc<[Selection<Anchor>]>, Option<Arc<[Selection<Anchor>]>>)>,
    autoclose_stack: InvalidationStack<BracketPairState>,
    snippet_stack: InvalidationStack<SnippetState>,
    select_larger_syntax_node_stack: Vec<Box<[Selection<usize>]>>,
    active_diagnostics: Option<ActiveDiagnosticGroup>,
    scroll_position: Vector2F,
    scroll_top_anchor: Option<Anchor>,
    autoscroll_request: Option<Autoscroll>,
    settings: watch::Receiver<Settings>,
    soft_wrap_mode_override: Option<settings::SoftWrap>,
    get_field_editor_theme: Option<GetFieldEditorTheme>,
    project: Option<ModelHandle<Project>>,
    focused: bool,
    show_local_cursors: bool,
    blink_epoch: usize,
    blinking_paused: bool,
    mode: EditorMode,
    vertical_scroll_margin: f32,
    placeholder_text: Option<Arc<str>>,
    highlighted_rows: Option<Range<u32>>,
    background_highlights: BTreeMap<TypeId, (Color, Vec<Range<Anchor>>)>,
    nav_history: Option<ItemNavHistory>,
    context_menu: Option<ContextMenu>,
    completion_tasks: Vec<(CompletionId, Task<Option<()>>)>,
    next_completion_id: CompletionId,
    available_code_actions: Option<(ModelHandle<Buffer>, Arc<[CodeAction]>)>,
    code_actions_task: Option<Task<()>>,
    document_highlights_task: Option<Task<()>>,
    pending_rename: Option<RenameState>,
    searchable: bool,
}

pub struct EditorSnapshot {
    pub mode: EditorMode,
    pub display_snapshot: DisplaySnapshot,
    pub placeholder_text: Option<Arc<str>>,
    is_focused: bool,
    scroll_position: Vector2F,
    scroll_top_anchor: Option<Anchor>,
}

#[derive(Clone)]
pub struct PendingSelection {
    selection: Selection<Anchor>,
    mode: SelectMode,
}

struct AddSelectionsState {
    above: bool,
    stack: Vec<usize>,
}

struct SelectNextState {
    query: AhoCorasick,
    wordwise: bool,
    done: bool,
}

struct BracketPairState {
    ranges: Vec<Range<Anchor>>,
    pair: BracketPair,
}

struct SnippetState {
    ranges: Vec<Vec<Range<Anchor>>>,
    active_index: usize,
}

pub struct RenameState {
    pub range: Range<Anchor>,
    pub old_name: String,
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
        cx: &AppContext,
    ) -> (DisplayPoint, ElementBox) {
        match self {
            ContextMenu::Completions(menu) => (cursor_position, menu.render(style, cx)),
            ContextMenu::CodeActions(menu) => menu.render(cursor_position, style),
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

    fn render(&self, style: EditorStyle, _: &AppContext) -> ElementBox {
        enum CompletionTag {}

        let completions = self.completions.clone();
        let matches = self.matches.clone();
        let selected_item = self.selected_item;
        let container_style = style.autocomplete.container;
        UniformList::new(self.list.clone(), matches.len(), move |range, items, cx| {
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
                                        style.text.color,
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
                    .on_mouse_down(move |cx| {
                        cx.dispatch_action(ConfirmCompletion(Some(item_ix)));
                    })
                    .boxed(),
                );
            }
        })
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
    ) -> (DisplayPoint, ElementBox) {
        enum ActionTag {}

        let container_style = style.autocomplete.container;
        let actions = self.actions.clone();
        let selected_item = self.selected_item;
        let element =
            UniformList::new(self.list.clone(), actions.len(), move |range, items, cx| {
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
                        .on_mouse_down(move |cx| {
                            cx.dispatch_action(ConfirmCodeAction(Some(item_ix)));
                        })
                        .boxed(),
                    );
                }
            })
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
struct ClipboardSelection {
    len: usize,
    is_entire_line: bool,
}

pub struct NavigationData {
    anchor: Anchor,
    offset: usize,
}

impl Editor {
    pub fn single_line(
        settings: watch::Receiver<Settings>,
        field_editor_style: Option<GetFieldEditorTheme>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(
            EditorMode::SingleLine,
            buffer,
            None,
            settings,
            field_editor_style,
            cx,
        )
    }

    pub fn auto_height(
        max_lines: usize,
        settings: watch::Receiver<Settings>,
        field_editor_style: Option<GetFieldEditorTheme>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(
            EditorMode::AutoHeight { max_lines },
            buffer,
            None,
            settings,
            field_editor_style,
            cx,
        )
    }

    pub fn for_buffer(
        buffer: ModelHandle<MultiBuffer>,
        project: Option<ModelHandle<Project>>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::new(EditorMode::Full, buffer, project, settings, None, cx)
    }

    pub fn clone(&self, nav_history: ItemNavHistory, cx: &mut ViewContext<Self>) -> Self {
        let mut clone = Self::new(
            self.mode,
            self.buffer.clone(),
            self.project.clone(),
            self.settings.clone(),
            self.get_field_editor_theme,
            cx,
        );
        clone.scroll_position = self.scroll_position;
        clone.scroll_top_anchor = self.scroll_top_anchor.clone();
        clone.nav_history = Some(nav_history);
        clone.searchable = self.searchable;
        clone
    }

    fn new(
        mode: EditorMode,
        buffer: ModelHandle<MultiBuffer>,
        project: Option<ModelHandle<Project>>,
        settings: watch::Receiver<Settings>,
        get_field_editor_theme: Option<GetFieldEditorTheme>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let display_map = cx.add_model(|cx| {
            let settings = settings.borrow();
            let style = build_style(&*settings, get_field_editor_theme, cx);
            DisplayMap::new(
                buffer.clone(),
                settings.tab_size,
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

        let mut this = Self {
            handle: cx.weak_handle(),
            buffer,
            display_map,
            selections: Arc::from([]),
            pending_selection: Some(PendingSelection {
                selection: Selection {
                    id: 0,
                    start: Anchor::min(),
                    end: Anchor::min(),
                    reversed: false,
                    goal: SelectionGoal::None,
                },
                mode: SelectMode::Character,
            }),
            columnar_selection_tail: None,
            next_selection_id: 1,
            add_selections_state: None,
            select_next_state: None,
            selection_history: Default::default(),
            autoclose_stack: Default::default(),
            snippet_stack: Default::default(),
            select_larger_syntax_node_stack: Vec::new(),
            active_diagnostics: None,
            settings,
            soft_wrap_mode_override: None,
            get_field_editor_theme,
            project,
            scroll_position: Vector2F::zero(),
            scroll_top_anchor: None,
            autoscroll_request: None,
            focused: false,
            show_local_cursors: false,
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
        };
        this.end_selection(cx);
        this
    }

    pub fn open_new(
        workspace: &mut Workspace,
        _: &workspace::OpenNew,
        cx: &mut ViewContext<Workspace>,
    ) {
        let project = workspace.project();
        if project.read(cx).is_remote() {
            cx.propagate_action();
        } else if let Some(buffer) = project
            .update(cx, |project, cx| project.create_buffer(cx))
            .log_err()
        {
            workspace.open_item(BufferItemHandle(buffer), cx);
        }
    }

    pub fn replica_id(&self, cx: &AppContext) -> ReplicaId {
        self.buffer.read(cx).replica_id()
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

    pub fn language<'a>(&self, cx: &'a AppContext) -> Option<&'a Arc<Language>> {
        self.buffer.read(cx).language(cx)
    }

    fn style(&self, cx: &AppContext) -> EditorStyle {
        build_style(&*self.settings.borrow(), self.get_field_editor_theme, cx)
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
        let map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if scroll_position.y() == 0. {
            self.scroll_top_anchor = None;
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
            self.scroll_top_anchor = Some(anchor);
        }

        cx.notify();
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

        let autoscroll = if let Some(autoscroll) = self.autoscroll_request.take() {
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
            let newest_selection =
                self.newest_selection_with_snapshot::<Point>(&display_map.buffer_snapshot);
            first_cursor_top = newest_selection.head().to_display_point(&display_map).row() as f32;
            last_cursor_bottom = first_cursor_top + 1.;
        } else {
            let selections = self.local_selections::<Point>(cx);
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
                    self.set_scroll_position(scroll_position, cx);
                } else if target_bottom >= end_row {
                    scroll_position.set_y(target_bottom - visible_lines);
                    self.set_scroll_position(scroll_position, cx);
                }
            }
            Autoscroll::Center => {
                scroll_position.set_y((first_cursor_top - margin).max(0.0));
                self.set_scroll_position(scroll_position, cx);
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
        let selections = self.local_selections::<Point>(cx);

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
        let tail = self
            .newest_selection_with_snapshot::<usize>(&display_map.buffer_snapshot)
            .tail();
        self.begin_selection(position, false, click_count, cx);

        let position = position.to_offset(&display_map, Bias::Left);
        let tail_anchor = display_map.buffer_snapshot.anchor_before(tail);
        let mut pending = self.pending_selection.clone().unwrap();

        if position >= tail {
            pending.selection.start = tail_anchor.clone();
        } else {
            pending.selection.end = tail_anchor.clone();
            pending.selection.reversed = true;
        }

        match &mut pending.mode {
            SelectMode::Word(range) | SelectMode::Line(range) => {
                *range = tail_anchor.clone()..tail_anchor
            }
            _ => {}
        }

        self.set_selections(self.selections.clone(), Some(pending), cx);
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
        let newest_selection = self.newest_anchor_selection().clone();

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

        self.push_to_nav_history(newest_selection.head(), Some(end.to_point(&buffer)), cx);

        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start,
            end,
            reversed: false,
            goal: SelectionGoal::None,
        };

        let mut selections;
        if add {
            selections = self.selections.clone();
            // Remove the newest selection if it was added due to a previous mouse up
            // within this multi-click.
            if click_count > 1 {
                selections = self
                    .selections
                    .iter()
                    .filter(|selection| selection.id != newest_selection.id)
                    .cloned()
                    .collect();
            }
        } else {
            selections = Arc::from([]);
        }
        self.set_selections(selections, Some(PendingSelection { selection, mode }), cx);

        cx.notify();
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
        let tail = self
            .newest_selection_with_snapshot::<Point>(&display_map.buffer_snapshot)
            .tail();
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
        } else if let Some(mut pending) = self.pending_selection.clone() {
            let buffer = self.buffer.read(cx).snapshot(cx);
            let head;
            let tail;
            match &pending.mode {
                SelectMode::Character => {
                    head = position.to_point(&display_map);
                    tail = pending.selection.tail().to_point(&buffer);
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
                pending.selection.start = buffer.anchor_before(head);
                pending.selection.end = buffer.anchor_before(tail);
                pending.selection.reversed = true;
            } else {
                pending.selection.start = buffer.anchor_before(tail);
                pending.selection.end = buffer.anchor_before(head);
                pending.selection.reversed = false;
            }
            self.set_selections(self.selections.clone(), Some(pending), cx);
        } else {
            log::error!("update_selection dispatched with no pending selection");
            return;
        }

        self.set_scroll_position(scroll_position, cx);
        cx.notify();
    }

    fn end_selection(&mut self, cx: &mut ViewContext<Self>) {
        self.columnar_selection_tail.take();
        if self.pending_selection.is_some() {
            let selections = self.local_selections::<usize>(cx);
            self.update_selections(selections, None, cx);
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

        let selections = (start_row..=end_row)
            .filter_map(|row| {
                if start_column <= display_map.line_len(row) && !display_map.is_block_line(row) {
                    let start = display_map
                        .clip_point(DisplayPoint::new(row, start_column), Bias::Left)
                        .to_point(&display_map);
                    let end = display_map
                        .clip_point(DisplayPoint::new(row, end_column), Bias::Right)
                        .to_point(&display_map);
                    Some(Selection {
                        id: post_inc(&mut self.next_selection_id),
                        start,
                        end,
                        reversed,
                        goal: SelectionGoal::None,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        self.update_selections(selections, None, cx);
        cx.notify();
    }

    pub fn is_selecting(&self) -> bool {
        self.pending_selection.is_some() || self.columnar_selection_tail.is_some()
    }

    pub fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if self.take_rename(cx).is_some() {
            return;
        }

        if self.hide_context_menu(cx).is_some() {
            return;
        }

        if self.snippet_stack.pop().is_some() {
            return;
        }

        if self.mode != EditorMode::Full {
            cx.propagate_action();
            return;
        }

        if self.active_diagnostics.is_some() {
            self.dismiss_diagnostics(cx);
        } else if let Some(pending) = self.pending_selection.clone() {
            let mut selections = self.selections.clone();
            if selections.is_empty() {
                selections = Arc::from([pending.selection]);
            }
            self.set_selections(selections, None, cx);
            self.request_autoscroll(Autoscroll::Fit, cx);
        } else {
            let mut oldest_selection = self.oldest_selection::<usize>(&cx);
            if self.selection_count() == 1 {
                if oldest_selection.is_empty() {
                    cx.propagate_action();
                    return;
                }

                oldest_selection.start = oldest_selection.head().clone();
                oldest_selection.end = oldest_selection.head().clone();
            }
            self.update_selections(vec![oldest_selection], Some(Autoscroll::Fit), cx);
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn selected_ranges<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &mut MutableAppContext,
    ) -> Vec<Range<D>> {
        self.local_selections::<D>(cx)
            .iter()
            .map(|s| {
                if s.reversed {
                    s.end.clone()..s.start.clone()
                } else {
                    s.start.clone()..s.end.clone()
                }
            })
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn selected_display_ranges(&self, cx: &mut MutableAppContext) -> Vec<Range<DisplayPoint>> {
        let display_map = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        self.selections
            .iter()
            .chain(
                self.pending_selection
                    .as_ref()
                    .map(|pending| &pending.selection),
            )
            .map(|s| {
                if s.reversed {
                    s.end.to_display_point(&display_map)..s.start.to_display_point(&display_map)
                } else {
                    s.start.to_display_point(&display_map)..s.end.to_display_point(&display_map)
                }
            })
            .collect()
    }

    pub fn select_ranges<I, T>(
        &mut self,
        ranges: I,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) where
        I: IntoIterator<Item = Range<T>>,
        T: ToOffset,
    {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let selections = ranges
            .into_iter()
            .map(|range| {
                let mut start = range.start.to_offset(&buffer);
                let mut end = range.end.to_offset(&buffer);
                let reversed = if start > end {
                    mem::swap(&mut start, &mut end);
                    true
                } else {
                    false
                };
                Selection {
                    id: post_inc(&mut self.next_selection_id),
                    start,
                    end,
                    reversed,
                    goal: SelectionGoal::None,
                }
            })
            .collect::<Vec<_>>();
        self.update_selections(selections, autoscroll, cx);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn select_display_ranges<'a, T>(&mut self, ranges: T, cx: &mut ViewContext<Self>)
    where
        T: IntoIterator<Item = &'a Range<DisplayPoint>>,
    {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = ranges
            .into_iter()
            .map(|range| {
                let mut start = range.start;
                let mut end = range.end;
                let reversed = if start > end {
                    mem::swap(&mut start, &mut end);
                    true
                } else {
                    false
                };
                Selection {
                    id: post_inc(&mut self.next_selection_id),
                    start: start.to_point(&display_map),
                    end: end.to_point(&display_map),
                    reversed,
                    goal: SelectionGoal::None,
                }
            })
            .collect();
        self.update_selections(selections, None, cx);
    }

    pub fn handle_input(&mut self, action: &Input, cx: &mut ViewContext<Self>) {
        let text = action.0.as_ref();
        if !self.skip_autoclose_end(text, cx) {
            self.start_transaction(cx);
            if !self.surround_with_bracket_pair(text, cx) {
                self.insert(text, cx);
                self.autoclose_bracket_pairs(cx);
            }
            self.end_transaction(cx);
            self.trigger_completion_on_input(text, cx);
        }
    }

    pub fn newline(&mut self, _: &Newline, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut old_selections = SmallVec::<[_; 32]>::new();
        {
            let selections = self.local_selections::<usize>(cx);
            let buffer = self.buffer.read(cx).snapshot(cx);
            for selection in selections.iter() {
                let start_point = selection.start.to_point(&buffer);
                let indent = buffer
                    .indent_column_for_line(start_point.row)
                    .min(start_point.column);
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
                            && buffer.contains_str_at(end + trailing_whitespace_len, pair_end)
                            && buffer.contains_str_at(
                                (start - leading_whitespace_len).saturating_sub(pair_start.len()),
                                pair_start,
                            )
                    });
                }

                old_selections.push((
                    selection.id,
                    buffer.anchor_after(end),
                    start..end,
                    indent,
                    insert_extra_newline,
                ));
            }
        }

        self.buffer.update(cx, |buffer, cx| {
            let mut delta = 0_isize;
            let mut pending_edit: Option<PendingEdit> = None;
            for (_, _, range, indent, insert_extra_newline) in &old_selections {
                if pending_edit.as_ref().map_or(false, |pending| {
                    pending.indent != *indent
                        || pending.insert_extra_newline != *insert_extra_newline
                }) {
                    let pending = pending_edit.take().unwrap();
                    let mut new_text = String::with_capacity(1 + pending.indent as usize);
                    new_text.push('\n');
                    new_text.extend(iter::repeat(' ').take(pending.indent as usize));
                    if pending.insert_extra_newline {
                        new_text = new_text.repeat(2);
                    }
                    buffer.edit_with_autoindent(pending.ranges, new_text, cx);
                    delta += pending.delta;
                }

                let start = (range.start as isize + delta) as usize;
                let end = (range.end as isize + delta) as usize;
                let mut text_len = *indent as usize + 1;
                if *insert_extra_newline {
                    text_len *= 2;
                }

                let pending = pending_edit.get_or_insert_with(Default::default);
                pending.delta += text_len as isize - (end - start) as isize;
                pending.indent = *indent;
                pending.insert_extra_newline = *insert_extra_newline;
                pending.ranges.push(start..end);
            }

            let pending = pending_edit.unwrap();
            let mut new_text = String::with_capacity(1 + pending.indent as usize);
            new_text.push('\n');
            new_text.extend(iter::repeat(' ').take(pending.indent as usize));
            if pending.insert_extra_newline {
                new_text = new_text.repeat(2);
            }
            buffer.edit_with_autoindent(pending.ranges, new_text, cx);

            let buffer = buffer.read(cx);
            self.selections = self
                .selections
                .iter()
                .cloned()
                .zip(old_selections)
                .map(
                    |(mut new_selection, (_, end_anchor, _, _, insert_extra_newline))| {
                        let mut cursor = end_anchor.to_point(&buffer);
                        if insert_extra_newline {
                            cursor.row -= 1;
                            cursor.column = buffer.line_len(cursor.row);
                        }
                        let anchor = buffer.anchor_after(cursor);
                        new_selection.start = anchor.clone();
                        new_selection.end = anchor;
                        new_selection
                    },
                )
                .collect();
        });

        self.request_autoscroll(Autoscroll::Fit, cx);
        self.end_transaction(cx);

        #[derive(Default)]
        struct PendingEdit {
            indent: u32,
            insert_extra_newline: bool,
            delta: isize,
            ranges: SmallVec<[Range<usize>; 32]>,
        }
    }

    pub fn insert(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let old_selections = self.local_selections::<usize>(cx);
        let selection_anchors = self.buffer.update(cx, |buffer, cx| {
            let anchors = {
                let snapshot = buffer.read(cx);
                old_selections
                    .iter()
                    .map(|s| (s.id, s.goal, snapshot.anchor_after(s.end)))
                    .collect::<Vec<_>>()
            };
            let edit_ranges = old_selections.iter().map(|s| s.start..s.end);
            buffer.edit_with_autoindent(edit_ranges, text, cx);
            anchors
        });

        let selections = {
            let snapshot = self.buffer.read(cx).read(cx);
            selection_anchors
                .into_iter()
                .map(|(id, goal, position)| {
                    let position = position.to_offset(&snapshot);
                    Selection {
                        id,
                        start: position,
                        end: position,
                        goal,
                        reversed: false,
                    }
                })
                .collect()
        };
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
        self.end_transaction(cx);
    }

    fn trigger_completion_on_input(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        let selection = self.newest_anchor_selection();
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
                .local_selections::<usize>(cx)
                .iter()
                .any(|selection| selection.is_empty())
            {
                false
            } else {
                let mut selections = self.selections.to_vec();
                for selection in &mut selections {
                    selection.end = selection.end.bias_left(&snapshot);
                }
                drop(snapshot);

                self.buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        selections.iter().map(|s| s.start.clone()..s.start.clone()),
                        &pair.start,
                        cx,
                    );
                    buffer.edit(
                        selections.iter().map(|s| s.end.clone()..s.end.clone()),
                        &pair.end,
                        cx,
                    );
                });

                let snapshot = self.buffer.read(cx).read(cx);
                for selection in &mut selections {
                    selection.end = selection.end.bias_right(&snapshot);
                }
                drop(snapshot);

                self.set_selections(selections.into(), None, cx);
                true
            }
        } else {
            false
        }
    }

    fn autoclose_bracket_pairs(&mut self, cx: &mut ViewContext<Self>) {
        let selections = self.local_selections::<usize>(cx);
        let mut bracket_pair_state = None;
        let mut new_selections = None;
        self.buffer.update(cx, |buffer, cx| {
            let mut snapshot = buffer.snapshot(cx);
            let left_biased_selections = selections
                .iter()
                .map(|selection| Selection {
                    id: selection.id,
                    start: snapshot.anchor_before(selection.start),
                    end: snapshot.anchor_before(selection.end),
                    reversed: selection.reversed,
                    goal: selection.goal,
                })
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
                            // Autoclose only if the next character is a whitespace or a pair end
                            // (possibly a different one from the pair we are inserting).
                            snapshot
                                .chars_at(selection.start)
                                .next()
                                .map_or(true, |ch| ch.is_whitespace())
                                || language.brackets().iter().any(|pair| {
                                    snapshot.contains_str_at(selection.start, &pair.end)
                                })
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

                buffer.edit(selection_ranges, &pair.end, cx);
                snapshot = buffer.snapshot(cx);

                new_selections = Some(
                    self.resolve_selections::<usize, _>(left_biased_selections.iter(), &snapshot)
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
            self.update_selections(new_selections, None, cx);
        }
        if let Some(bracket_pair_state) = bracket_pair_state {
            self.autoclose_stack.push(bracket_pair_state);
        }
    }

    fn skip_autoclose_end(&mut self, text: &str, cx: &mut ViewContext<Self>) -> bool {
        let old_selections = self.local_selections::<usize>(cx);
        let autoclose_pair = if let Some(autoclose_pair) = self.autoclose_stack.last() {
            autoclose_pair
        } else {
            return false;
        };
        if text != autoclose_pair.pair.end {
            return false;
        }

        debug_assert_eq!(old_selections.len(), autoclose_pair.ranges.len());

        let buffer = self.buffer.read(cx).snapshot(cx);
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
            self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
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

        let position = self.newest_anchor_selection().head();
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
        ConfirmCompletion(completion_ix): &ConfirmCompletion,
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
            .get(completion_ix.unwrap_or(completions_menu.selected_item))?;
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
        let buffer = buffer_handle.read(cx);
        let old_range = completion.old_range.to_offset(&buffer);
        let old_text = buffer.text_for_range(old_range.clone()).collect::<String>();

        let selections = self.local_selections::<usize>(cx);
        let newest_selection = self.newest_anchor_selection();
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

        self.start_transaction(cx);
        if let Some(mut snippet) = snippet {
            snippet.text = text.to_string();
            for tabstop in snippet.tabstops.iter_mut().flatten() {
                tabstop.start -= common_prefix_len as isize;
                tabstop.end -= common_prefix_len as isize;
            }

            self.insert_snippet(&ranges, snippet, cx).log_err();
        } else {
            self.buffer.update(cx, |buffer, cx| {
                buffer.edit_with_autoindent(ranges, text, cx);
            });
        }
        self.end_transaction(cx);

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

    pub fn toggle_code_actions(
        &mut self,
        &ToggleCodeActions(deployed_from_indicator): &ToggleCodeActions,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(
            self.context_menu.as_ref(),
            Some(ContextMenu::CodeActions(_))
        ) {
            self.context_menu.take();
            cx.notify();
            return;
        }

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
        ConfirmCodeAction(action_ix): &ConfirmCodeAction,
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
        let action_ix = action_ix.unwrap_or(actions_menu.selected_item);
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

        // If the code action's edits are all contained within this editor, then
        // avoid opening a new editor to display them.
        let mut entries = transaction.0.iter();
        if let Some((buffer, transaction)) = entries.next() {
            if entries.next().is_none() {
                let excerpt = this.read_with(&cx, |editor, cx| {
                    editor
                        .buffer()
                        .read(cx)
                        .excerpt_containing(editor.newest_anchor_selection().head(), cx)
                });
                if let Some((excerpted_buffer, excerpt_range)) = excerpt {
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
        }

        let mut ranges_to_highlight = Vec::new();
        let excerpt_buffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(replica_id).with_title(title);
            for (buffer, transaction) in &transaction.0 {
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
            multibuffer.push_transaction(&transaction.0);
            multibuffer
        });

        workspace.update(&mut cx, |workspace, cx| {
            let editor = workspace.open_item(MultiBufferItemHandle(excerpt_buffer), cx);
            if let Some(editor) = editor.act_as::<Self>(cx) {
                editor.update(cx, |editor, cx| {
                    let color = editor.style(cx).highlighted_line_background;
                    editor.highlight_background::<Self>(ranges_to_highlight, color, cx);
                });
            }
        });

        Ok(())
    }

    fn refresh_code_actions(&mut self, cx: &mut ViewContext<Self>) -> Option<()> {
        let project = self.project.as_ref()?;
        let buffer = self.buffer.read(cx);
        let newest_selection = self.newest_anchor_selection().clone();
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
        let project = self.project.as_ref()?;
        let buffer = self.buffer.read(cx);
        let newest_selection = self.newest_anchor_selection().clone();
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

        enum DocumentHighlightRead {}
        enum DocumentHighlightWrite {}

        self.document_highlights_task = Some(cx.spawn_weak(|this, mut cx| async move {
            let highlights = highlights.log_err().await;
            if let Some((this, highlights)) = this.upgrade(&cx).zip(highlights) {
                this.update(&mut cx, |this, cx| {
                    let buffer_id = cursor_position.buffer_id;
                    let excerpt_id = cursor_position.excerpt_id.clone();
                    let style = this.style(cx);
                    let read_background = style.document_highlight_read_background;
                    let write_background = style.document_highlight_write_background;
                    let buffer = this.buffer.read(cx);
                    if !buffer
                        .text_anchor_for_position(cursor_position, cx)
                        .map_or(false, |(buffer, _)| buffer == cursor_buffer)
                    {
                        return;
                    }

                    let mut write_ranges = Vec::new();
                    let mut read_ranges = Vec::new();
                    for highlight in highlights {
                        let range = Anchor {
                            buffer_id,
                            excerpt_id: excerpt_id.clone(),
                            text_anchor: highlight.range.start,
                        }..Anchor {
                            buffer_id,
                            excerpt_id: excerpt_id.clone(),
                            text_anchor: highlight.range.end,
                        };
                        if highlight.kind == lsp::DocumentHighlightKind::WRITE {
                            write_ranges.push(range);
                        } else {
                            read_ranges.push(range);
                        }
                    }

                    this.highlight_background::<DocumentHighlightRead>(
                        read_ranges,
                        read_background,
                        cx,
                    );
                    this.highlight_background::<DocumentHighlightWrite>(
                        write_ranges,
                        write_background,
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
        cx: &mut ViewContext<Self>,
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
                .on_mouse_down(|cx| {
                    cx.dispatch_action(ToggleCodeActions(true));
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
        cx: &AppContext,
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
            buffer.edit_with_autoindent(insertion_ranges.iter().cloned(), &snippet.text, cx);

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
                    tabstop_ranges
                        .sort_unstable_by(|a, b| a.start.cmp(&b.start, snapshot).unwrap());
                    tabstop_ranges
                })
                .collect::<Vec<_>>()
        });

        if let Some(tabstop) = tabstops.first() {
            self.select_ranges(tabstop.iter().cloned(), Some(Autoscroll::Fit), cx);
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

    pub fn move_to_prev_snippet_tabstop(&mut self, cx: &mut ViewContext<Self>) {
        self.move_to_snippet_tabstop(Bias::Left, cx);
    }

    pub fn move_to_snippet_tabstop(&mut self, bias: Bias, cx: &mut ViewContext<Self>) -> bool {
        let buffer = self.buffer.read(cx).snapshot(cx);

        if let Some(snippet) = self.snippet_stack.last_mut() {
            match bias {
                Bias::Left => {
                    if snippet.active_index > 0 {
                        snippet.active_index -= 1;
                    } else {
                        return false;
                    }
                }
                Bias::Right => {
                    if snippet.active_index + 1 < snippet.ranges.len() {
                        snippet.active_index += 1;
                    } else {
                        return false;
                    }
                }
            }
            if let Some(current_ranges) = snippet.ranges.get(snippet.active_index) {
                let new_selections = current_ranges
                    .iter()
                    .map(|new_range| {
                        let new_range = new_range.to_offset(&buffer);
                        Selection {
                            id: post_inc(&mut self.next_selection_id),
                            start: new_range.start,
                            end: new_range.end,
                            reversed: false,
                            goal: SelectionGoal::None,
                        }
                    })
                    .collect();

                // Remove the snippet state when moving to the last tabstop.
                if snippet.active_index + 1 == snippet.ranges.len() {
                    self.snippet_stack.pop();
                }

                self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
                return true;
            }
            self.snippet_stack.pop();
        }

        false
    }

    pub fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_all(&SelectAll, cx);
        self.insert("", cx);
        self.end_transaction(cx);
    }

    pub fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut selections = self.local_selections::<Point>(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        for selection in &mut selections {
            if selection.is_empty() {
                let head = selection.head().to_display_point(&display_map);
                let cursor = movement::left(&display_map, head)
                    .unwrap()
                    .to_point(&display_map);
                selection.set_head(cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
        self.insert("", cx);
        self.end_transaction(cx);
    }

    pub fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            if selection.is_empty() {
                let head = selection.head().to_display_point(&display_map);
                let cursor = movement::right(&display_map, head)
                    .unwrap()
                    .to_point(&display_map);
                selection.set_head(cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
        self.insert(&"", cx);
        self.end_transaction(cx);
    }

    pub fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        if self.move_to_next_snippet_tabstop(cx) {
            return;
        }

        self.start_transaction(cx);
        let tab_size = self.settings.borrow().tab_size;
        let mut selections = self.local_selections::<Point>(cx);
        let mut last_indent = None;
        self.buffer.update(cx, |buffer, cx| {
            for selection in &mut selections {
                if selection.is_empty() {
                    let char_column = buffer
                        .read(cx)
                        .text_for_range(Point::new(selection.start.row, 0)..selection.start)
                        .flat_map(str::chars)
                        .count();
                    let chars_to_next_tab_stop = tab_size - (char_column % tab_size);
                    buffer.edit(
                        [selection.start..selection.start],
                        " ".repeat(chars_to_next_tab_stop),
                        cx,
                    );
                    selection.start.column += chars_to_next_tab_stop as u32;
                    selection.end = selection.start;
                } else {
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
                        let indent_column = buffer.read(cx).indent_column_for_line(row) as usize;
                        let columns_to_next_tab_stop = tab_size - (indent_column % tab_size);
                        let row_start = Point::new(row, 0);
                        buffer.edit(
                            [row_start..row_start],
                            " ".repeat(columns_to_next_tab_stop),
                            cx,
                        );

                        // Update this selection's endpoints to reflect the indentation.
                        if row == selection.start.row {
                            selection.start.column += columns_to_next_tab_stop as u32;
                        }
                        if row == selection.end.row {
                            selection.end.column += columns_to_next_tab_stop as u32;
                        }

                        last_indent = Some((row, columns_to_next_tab_stop as u32));
                    }
                }
            }
        });

        self.update_selections(selections, Some(Autoscroll::Fit), cx);
        self.end_transaction(cx);
    }

    pub fn outdent(&mut self, _: &Outdent, cx: &mut ViewContext<Self>) {
        if !self.snippet_stack.is_empty() {
            self.move_to_prev_snippet_tabstop(cx);
            return;
        }

        self.start_transaction(cx);
        let tab_size = self.settings.borrow().tab_size;
        let selections = self.local_selections::<Point>(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut deletion_ranges = Vec::new();
        let mut last_outdent = None;
        {
            let buffer = self.buffer.read(cx).read(cx);
            for selection in &selections {
                let mut rows = selection.spanned_rows(false, &display_map);

                // Avoid re-outdenting a row that has already been outdented by a
                // previous selection.
                if let Some(last_row) = last_outdent {
                    if last_row == rows.start {
                        rows.start += 1;
                    }
                }

                for row in rows {
                    let column = buffer.indent_column_for_line(row) as usize;
                    if column > 0 {
                        let mut deletion_len = (column % tab_size) as u32;
                        if deletion_len == 0 {
                            deletion_len = tab_size as u32;
                        }
                        deletion_ranges.push(Point::new(row, 0)..Point::new(row, deletion_len));
                        last_outdent = Some(row);
                    }
                }
            }
        }
        self.buffer.update(cx, |buffer, cx| {
            buffer.edit(deletion_ranges, "", cx);
        });

        self.update_selections(
            self.local_selections::<usize>(cx),
            Some(Autoscroll::Fit),
            cx,
        );
        self.end_transaction(cx);
    }

    pub fn delete_line(&mut self, _: &DeleteLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let selections = self.local_selections::<Point>(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

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

        let buffer = self.buffer.update(cx, |buffer, cx| {
            buffer.edit(edit_ranges, "", cx);
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
        self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
        self.end_transaction(cx);
    }

    pub fn duplicate_line(&mut self, _: &DuplicateLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let selections = self.local_selections::<Point>(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;

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
            edits.push((start, text, rows.len() as u32));
        }

        self.buffer.update(cx, |buffer, cx| {
            for (point, text, _) in edits.into_iter().rev() {
                buffer.edit(Some(point..point), text, cx);
            }
        });

        self.request_autoscroll(Autoscroll::Fit, cx);
        self.end_transaction(cx);
    }

    pub fn move_line_up(&mut self, _: &MoveLineUp, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut edits = Vec::new();
        let mut unfold_ranges = Vec::new();
        let mut refold_ranges = Vec::new();

        let selections = self.local_selections::<Point>(cx);
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

        self.start_transaction(cx);
        self.unfold_ranges(unfold_ranges, cx);
        self.buffer.update(cx, |buffer, cx| {
            for (range, text) in edits {
                buffer.edit([range], text, cx);
            }
        });
        self.fold_ranges(refold_ranges, cx);
        self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
        self.end_transaction(cx);
    }

    pub fn move_line_down(&mut self, _: &MoveLineDown, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut edits = Vec::new();
        let mut unfold_ranges = Vec::new();
        let mut refold_ranges = Vec::new();

        let selections = self.local_selections::<Point>(cx);
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

        self.start_transaction(cx);
        self.unfold_ranges(unfold_ranges, cx);
        self.buffer.update(cx, |buffer, cx| {
            for (range, text) in edits {
                buffer.edit([range], text, cx);
            }
        });
        self.fold_ranges(refold_ranges, cx);
        self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
        self.end_transaction(cx);
    }

    pub fn cut(&mut self, _: &Cut, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut text = String::new();
        let mut selections = self.local_selections::<Point>(cx);
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let buffer = self.buffer.read(cx).read(cx);
            let max_point = buffer.max_point();
            for selection in &mut selections {
                let is_entire_line = selection.is_empty();
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
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
        self.insert("", cx);
        self.end_transaction(cx);

        cx.as_mut()
            .write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        let selections = self.local_selections::<Point>(cx);
        let mut text = String::new();
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let buffer = self.buffer.read(cx).read(cx);
            let max_point = buffer.max_point();
            for selection in selections.iter() {
                let mut start = selection.start;
                let mut end = selection.end;
                let is_entire_line = selection.is_empty();
                if is_entire_line {
                    start = Point::new(start.row, 0);
                    end = cmp::min(max_point, Point::new(start.row + 1, 0));
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

        cx.as_mut()
            .write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(item) = cx.as_mut().read_from_clipboard() {
            let clipboard_text = item.text();
            if let Some(mut clipboard_selections) = item.metadata::<Vec<ClipboardSelection>>() {
                let mut selections = self.local_selections::<usize>(cx);
                let all_selections_were_entire_line =
                    clipboard_selections.iter().all(|s| s.is_entire_line);
                if clipboard_selections.len() != selections.len() {
                    clipboard_selections.clear();
                }

                let mut delta = 0_isize;
                let mut start_offset = 0;
                for (i, selection) in selections.iter_mut().enumerate() {
                    let to_insert;
                    let entire_line;
                    if let Some(clipboard_selection) = clipboard_selections.get(i) {
                        let end_offset = start_offset + clipboard_selection.len;
                        to_insert = &clipboard_text[start_offset..end_offset];
                        entire_line = clipboard_selection.is_entire_line;
                        start_offset = end_offset
                    } else {
                        to_insert = clipboard_text.as_str();
                        entire_line = all_selections_were_entire_line;
                    }

                    selection.start = (selection.start as isize + delta) as usize;
                    selection.end = (selection.end as isize + delta) as usize;

                    self.buffer.update(cx, |buffer, cx| {
                        // If the corresponding selection was empty when this slice of the
                        // clipboard text was written, then the entire line containing the
                        // selection was copied. If this selection is also currently empty,
                        // then paste the line before the current line of the buffer.
                        let range = if selection.is_empty() && entire_line {
                            let column = selection.start.to_point(&buffer.read(cx)).column as usize;
                            let line_start = selection.start - column;
                            line_start..line_start
                        } else {
                            selection.start..selection.end
                        };

                        delta += to_insert.len() as isize - range.len() as isize;
                        buffer.edit([range], to_insert, cx);
                        selection.start += to_insert.len();
                        selection.end = selection.start;
                    });
                }
                self.update_selections(selections, Some(Autoscroll::Fit), cx);
            } else {
                self.insert(clipboard_text, cx);
            }
        }
    }

    pub fn undo(&mut self, _: &Undo, cx: &mut ViewContext<Self>) {
        if let Some(tx_id) = self.buffer.update(cx, |buffer, cx| buffer.undo(cx)) {
            if let Some((selections, _)) = self.selection_history.get(&tx_id).cloned() {
                self.set_selections(selections, None, cx);
            }
            self.request_autoscroll(Autoscroll::Fit, cx);
        }
    }

    pub fn redo(&mut self, _: &Redo, cx: &mut ViewContext<Self>) {
        if let Some(tx_id) = self.buffer.update(cx, |buffer, cx| buffer.redo(cx)) {
            if let Some((_, Some(selections))) = self.selection_history.get(&tx_id).cloned() {
                self.set_selections(selections, None, cx);
            }
            self.request_autoscroll(Autoscroll::Fit, cx);
        }
    }

    pub fn finalize_last_transaction(&mut self, cx: &mut ViewContext<Self>) {
        self.buffer
            .update(cx, |buffer, cx| buffer.finalize_last_transaction(cx));
    }

    pub fn move_left(&mut self, _: &MoveLeft, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let start = selection.start.to_display_point(&display_map);
            let end = selection.end.to_display_point(&display_map);

            if start != end {
                selection.end = selection.start.clone();
            } else {
                let cursor = movement::left(&display_map, start)
                    .unwrap()
                    .to_point(&display_map);
                selection.start = cursor.clone();
                selection.end = cursor;
            }
            selection.reversed = false;
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn select_left(&mut self, _: &SelectLeft, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let cursor = movement::left(&display_map, head)
                .unwrap()
                .to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn move_right(&mut self, _: &MoveRight, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let start = selection.start.to_display_point(&display_map);
            let end = selection.end.to_display_point(&display_map);

            if start != end {
                selection.start = selection.end.clone();
            } else {
                let cursor = movement::right(&display_map, end)
                    .unwrap()
                    .to_point(&display_map);
                selection.start = cursor;
                selection.end = cursor;
            }
            selection.reversed = false;
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn select_right(&mut self, _: &SelectRight, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let cursor = movement::right(&display_map, head)
                .unwrap()
                .to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if self.take_rename(cx).is_some() {
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

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let start = selection.start.to_display_point(&display_map);
            let end = selection.end.to_display_point(&display_map);
            if start != end {
                selection.goal = SelectionGoal::None;
            }

            let (start, goal) = movement::up(&display_map, start, selection.goal).unwrap();
            let cursor = start.to_point(&display_map);
            selection.start = cursor;
            selection.end = cursor;
            selection.goal = goal;
            selection.reversed = false;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn select_up(&mut self, _: &SelectUp, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let (head, goal) = movement::up(&display_map, head, selection.goal).unwrap();
            let cursor = head.to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = goal;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        self.take_rename(cx);

        if let Some(context_menu) = self.context_menu.as_mut() {
            if context_menu.select_next(cx) {
                return;
            }
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let start = selection.start.to_display_point(&display_map);
            let end = selection.end.to_display_point(&display_map);
            if start != end {
                selection.goal = SelectionGoal::None;
            }

            let (start, goal) = movement::down(&display_map, end, selection.goal).unwrap();
            let cursor = start.to_point(&display_map);
            selection.start = cursor;
            selection.end = cursor;
            selection.goal = goal;
            selection.reversed = false;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn select_down(&mut self, _: &SelectDown, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let (head, goal) = movement::down(&display_map, head, selection.goal).unwrap();
            let cursor = head.to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = goal;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn move_to_previous_word_boundary(
        &mut self,
        _: &MoveToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let cursor = movement::prev_word_boundary(&display_map, head).to_point(&display_map);
            selection.start = cursor.clone();
            selection.end = cursor;
            selection.reversed = false;
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn select_to_previous_word_boundary(
        &mut self,
        _: &SelectToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let cursor = movement::prev_word_boundary(&display_map, head).to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn delete_to_previous_word_boundary(
        &mut self,
        _: &DeleteToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        self.start_transaction(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            if selection.is_empty() {
                let head = selection.head().to_display_point(&display_map);
                let cursor =
                    movement::prev_word_boundary(&display_map, head).to_point(&display_map);
                selection.set_head(cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
        self.insert("", cx);
        self.end_transaction(cx);
    }

    pub fn move_to_next_word_boundary(
        &mut self,
        _: &MoveToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let cursor = movement::next_word_boundary(&display_map, head).to_point(&display_map);
            selection.start = cursor;
            selection.end = cursor;
            selection.reversed = false;
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn select_to_next_word_boundary(
        &mut self,
        _: &SelectToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let cursor = movement::next_word_boundary(&display_map, head).to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn delete_to_next_word_boundary(
        &mut self,
        _: &DeleteToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        self.start_transaction(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            if selection.is_empty() {
                let head = selection.head().to_display_point(&display_map);
                let cursor =
                    movement::next_word_boundary(&display_map, head).to_point(&display_map);
                selection.set_head(cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
        self.insert("", cx);
        self.end_transaction(cx);
    }

    pub fn move_to_beginning_of_line(
        &mut self,
        _: &MoveToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::line_beginning(&display_map, head, true);
            let cursor = new_head.to_point(&display_map);
            selection.start = cursor;
            selection.end = cursor;
            selection.reversed = false;
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn select_to_beginning_of_line(
        &mut self,
        SelectToBeginningOfLine(stop_at_soft_boundaries): &SelectToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::line_beginning(&display_map, head, *stop_at_soft_boundaries);
            selection.set_head(new_head.to_point(&display_map));
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn delete_to_beginning_of_line(
        &mut self,
        _: &DeleteToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.start_transaction(cx);
        self.select_to_beginning_of_line(&SelectToBeginningOfLine(false), cx);
        self.backspace(&Backspace, cx);
        self.end_transaction(cx);
    }

    pub fn move_to_end_of_line(&mut self, _: &MoveToEndOfLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map);
                let new_head = movement::line_end(&display_map, head, true);
                let anchor = new_head.to_point(&display_map);
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn select_to_end_of_line(
        &mut self,
        SelectToEndOfLine(stop_at_soft_boundaries): &SelectToEndOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::line_end(&display_map, head, *stop_at_soft_boundaries);
            selection.set_head(new_head.to_point(&display_map));
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn delete_to_end_of_line(&mut self, _: &DeleteToEndOfLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_end_of_line(&SelectToEndOfLine(false), cx);
        self.delete(&Delete, cx);
        self.end_transaction(cx);
    }

    pub fn cut_to_end_of_line(&mut self, _: &CutToEndOfLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_end_of_line(&SelectToEndOfLine(false), cx);
        self.cut(&Cut, cx);
        self.end_transaction(cx);
    }

    pub fn move_to_beginning(&mut self, _: &MoveToBeginning, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: 0,
            end: 0,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], Some(Autoscroll::Fit), cx);
    }

    pub fn select_to_beginning(&mut self, _: &SelectToBeginning, cx: &mut ViewContext<Self>) {
        let mut selection = self.local_selections::<Point>(cx).last().unwrap().clone();
        selection.set_head(Point::zero());
        self.update_selections(vec![selection], Some(Autoscroll::Fit), cx);
    }

    pub fn move_to_end(&mut self, _: &MoveToEnd, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let cursor = self.buffer.read(cx).read(cx).len();
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor,
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], Some(Autoscroll::Fit), cx);
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
            let offset = position.to_offset(&buffer);
            let point = position.to_point(&buffer);
            drop(buffer);

            if let Some(new_position) = new_position {
                let row_delta = (new_position.row as i64 - point.row as i64).abs();
                if row_delta < MIN_NAVIGATION_HISTORY_ROW_DELTA {
                    return;
                }
            }

            nav_history.push(Some(NavigationData {
                anchor: position,
                offset,
            }));
        }
    }

    pub fn select_to_end(&mut self, _: &SelectToEnd, cx: &mut ViewContext<Self>) {
        let mut selection = self.local_selections::<usize>(cx).first().unwrap().clone();
        selection.set_head(self.buffer.read(cx).read(cx).len());
        self.update_selections(vec![selection], Some(Autoscroll::Fit), cx);
    }

    pub fn select_all(&mut self, _: &SelectAll, cx: &mut ViewContext<Self>) {
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: 0,
            end: self.buffer.read(cx).read(cx).len(),
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], None, cx);
    }

    pub fn select_line(&mut self, _: &SelectLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        let max_point = display_map.buffer_snapshot.max_point();
        for selection in &mut selections {
            let rows = selection.spanned_rows(true, &display_map);
            selection.start = Point::new(rows.start, 0);
            selection.end = cmp::min(max_point, Point::new(rows.end, 0));
            selection.reversed = false;
        }
        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn split_selection_into_lines(
        &mut self,
        _: &SplitSelectionIntoLines,
        cx: &mut ViewContext<Self>,
    ) {
        let mut to_unfold = Vec::new();
        let mut new_selections = Vec::new();
        {
            let selections = self.local_selections::<Point>(cx);
            let buffer = self.buffer.read(cx).read(cx);
            for selection in selections {
                for row in selection.start.row..selection.end.row {
                    let cursor = Point::new(row, buffer.line_len(row));
                    new_selections.push(Selection {
                        id: post_inc(&mut self.next_selection_id),
                        start: cursor,
                        end: cursor,
                        reversed: false,
                        goal: SelectionGoal::None,
                    });
                }
                new_selections.push(Selection {
                    id: selection.id,
                    start: selection.end,
                    end: selection.end,
                    reversed: false,
                    goal: SelectionGoal::None,
                });
                to_unfold.push(selection.start..selection.end);
            }
        }
        self.unfold_ranges(to_unfold, cx);
        self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
    }

    pub fn add_selection_above(&mut self, _: &AddSelectionAbove, cx: &mut ViewContext<Self>) {
        self.add_selection(true, cx);
    }

    pub fn add_selection_below(&mut self, _: &AddSelectionBelow, cx: &mut ViewContext<Self>) {
        self.add_selection(false, cx);
    }

    fn add_selection(&mut self, above: bool, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.local_selections::<Point>(cx);
        let mut state = self.add_selections_state.take().unwrap_or_else(|| {
            let oldest_selection = selections.iter().min_by_key(|s| s.id).unwrap().clone();
            let range = oldest_selection.display_range(&display_map).sorted();
            let columns = cmp::min(range.start.column(), range.end.column())
                ..cmp::max(range.start.column(), range.end.column());

            selections.clear();
            let mut stack = Vec::new();
            for row in range.start.row()..=range.end.row() {
                if let Some(selection) = self.build_columnar_selection(
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

                        if let Some(new_selection) = self.build_columnar_selection(
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

        self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
        if state.stack.len() > 1 {
            self.add_selections_state = Some(state);
        }
    }

    pub fn select_next(&mut self, action: &SelectNext, cx: &mut ViewContext<Self>) {
        let replace_newest = action.0;
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let mut selections = self.local_selections::<usize>(cx);
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
                    if replace_newest {
                        if let Some(newest_id) =
                            selections.iter().max_by_key(|s| s.id).map(|s| s.id)
                        {
                            selections.retain(|s| s.id != newest_id);
                        }
                    }
                    selections.push(Selection {
                        id: post_inc(&mut self.next_selection_id),
                        start: next_selected_range.start,
                        end: next_selected_range.end,
                        reversed: false,
                        goal: SelectionGoal::None,
                    });
                    self.update_selections(selections, Some(Autoscroll::Newest), cx);
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
                self.update_selections(selections, Some(Autoscroll::Newest), cx);
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
        // Get the line comment prefix. Split its trailing whitespace into a separate string,
        // as that portion won't be used for detecting if a line is a comment.
        let full_comment_prefix =
            if let Some(prefix) = self.language(cx).and_then(|l| l.line_comment_prefix()) {
                prefix.to_string()
            } else {
                return;
            };
        let comment_prefix = full_comment_prefix.trim_end_matches(' ');
        let comment_prefix_whitespace = &full_comment_prefix[comment_prefix.len()..];

        self.start_transaction(cx);
        let mut selections = self.local_selections::<Point>(cx);
        let mut all_selection_lines_are_comments = true;
        let mut edit_ranges = Vec::new();
        let mut last_toggled_row = None;
        self.buffer.update(cx, |buffer, cx| {
            for selection in &mut selections {
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

                    let start = Point::new(row, snapshot.indent_column_for_line(row));
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
                            .count() as u32;
                        let end = Point::new(
                            row,
                            start.column + comment_prefix.len() as u32 + matching_whitespace_len,
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
                        buffer.edit(edit_ranges.iter().cloned(), "", cx);
                    } else {
                        let min_column = edit_ranges.iter().map(|r| r.start.column).min().unwrap();
                        let edit_ranges = edit_ranges.iter().map(|range| {
                            let position = Point::new(range.start.row, min_column);
                            position..position
                        });
                        buffer.edit(edit_ranges, &full_comment_prefix, cx);
                    }
                }
            }
        });

        self.update_selections(
            self.local_selections::<usize>(cx),
            Some(Autoscroll::Fit),
            cx,
        );
        self.end_transaction(cx);
    }

    pub fn select_larger_syntax_node(
        &mut self,
        _: &SelectLargerSyntaxNode,
        cx: &mut ViewContext<Self>,
    ) {
        let old_selections = self.local_selections::<usize>(cx).into_boxed_slice();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

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
            self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
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
            self.update_selections(selections.to_vec(), Some(Autoscroll::Fit), cx);
        }
        self.select_larger_syntax_node_stack = stack;
    }

    pub fn move_to_enclosing_bracket(
        &mut self,
        _: &MoveToEnclosingBracket,
        cx: &mut ViewContext<Self>,
    ) {
        let mut selections = self.local_selections::<usize>(cx);
        let buffer = self.buffer.read(cx).snapshot(cx);
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

        self.update_selections(selections, Some(Autoscroll::Fit), cx);
    }

    pub fn show_next_diagnostic(&mut self, _: &ShowNextDiagnostic, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let selection = self.newest_selection_with_snapshot::<usize>(&buffer);
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
            let next_group = buffer
                .diagnostics_in_range::<_, usize>(search_start..buffer.len())
                .find_map(|entry| {
                    if entry.diagnostic.is_primary
                        && !entry.range.is_empty()
                        && Some(entry.range.end) != active_primary_range.as_ref().map(|r| *r.end())
                    {
                        Some((entry.range, entry.diagnostic.group_id))
                    } else {
                        None
                    }
                });

            if let Some((primary_range, group_id)) = next_group {
                self.activate_diagnostics(group_id, cx);
                self.update_selections(
                    vec![Selection {
                        id: selection.id,
                        start: primary_range.start,
                        end: primary_range.start,
                        reversed: false,
                        goal: SelectionGoal::None,
                    }],
                    Some(Autoscroll::Center),
                    cx,
                );
                break;
            } else if search_start == 0 {
                break;
            } else {
                // Cycle around to the start of the buffer, potentially moving back to the start of
                // the currently active diagnostic.
                search_start = 0;
                active_primary_range.take();
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
        let head = editor.newest_selection::<usize>(cx).head();
        let (buffer, head) =
            if let Some(text_anchor) = editor.buffer.read(cx).text_anchor_for_position(head, cx) {
                text_anchor
            } else {
                return;
            };

        let definitions = workspace
            .project()
            .update(cx, |project, cx| project.definition(&buffer, head, cx));
        cx.spawn(|workspace, mut cx| async move {
            let definitions = definitions.await?;
            workspace.update(&mut cx, |workspace, cx| {
                let nav_history = workspace.active_pane().read(cx).nav_history().clone();
                for definition in definitions {
                    let range = definition.range.to_offset(definition.buffer.read(cx));
                    let target_editor_handle = workspace
                        .open_item(BufferItemHandle(definition.buffer), cx)
                        .downcast::<Self>()
                        .unwrap();

                    target_editor_handle.update(cx, |target_editor, cx| {
                        // When selecting a definition in a different buffer, disable the nav history
                        // to avoid creating a history entry at the previous cursor location.
                        if editor_handle != target_editor_handle {
                            nav_history.borrow_mut().disable();
                        }
                        target_editor.select_ranges([range], Some(Autoscroll::Center), cx);
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
        let head = editor.newest_selection::<usize>(cx).head();
        let (buffer, head) = editor.buffer.read(cx).text_anchor_for_position(head, cx)?;
        let replica_id = editor.replica_id(cx);

        let references = workspace
            .project()
            .update(cx, |project, cx| project.references(&buffer, head, cx));
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
                let editor = workspace.open_item(MultiBufferItemHandle(excerpt_buffer), cx);
                if let Some(editor) = editor.act_as::<Self>(cx) {
                    editor.update(cx, |editor, cx| {
                        let color = editor.style(cx).highlighted_line_background;
                        editor.highlight_background::<Self>(ranges_to_highlight, color, cx);
                    });
                }
            });

            Ok(())
        }))
    }

    pub fn rename(&mut self, _: &Rename, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        use language::ToOffset as _;

        let project = self.project.clone()?;
        let selection = self.newest_anchor_selection().clone();
        let (cursor_buffer, cursor_buffer_position) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(selection.head(), cx)?;
        let (tail_buffer, tail_buffer_position) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(selection.tail(), cx)?;
        if tail_buffer != cursor_buffer {
            return None;
        }

        let snapshot = cursor_buffer.read(cx).snapshot();
        let cursor_buffer_offset = cursor_buffer_position.to_offset(&snapshot);
        let tail_buffer_offset = tail_buffer_position.to_offset(&snapshot);
        let prepare_rename = project.update(cx, |project, cx| {
            project.prepare_rename(cursor_buffer, cursor_buffer_offset, cx)
        });

        Some(cx.spawn(|this, mut cx| async move {
            if let Some(rename_range) = prepare_rename.await? {
                let rename_buffer_range = rename_range.to_offset(&snapshot);
                let cursor_offset_in_rename_range =
                    cursor_buffer_offset.saturating_sub(rename_buffer_range.start);
                let tail_offset_in_rename_range =
                    tail_buffer_offset.saturating_sub(rename_buffer_range.start);

                this.update(&mut cx, |this, cx| {
                    this.take_rename(cx);
                    let style = this.style(cx);
                    let buffer = this.buffer.read(cx).read(cx);
                    let cursor_offset = selection.head().to_offset(&buffer);
                    let rename_start = cursor_offset.saturating_sub(cursor_offset_in_rename_range);
                    let rename_end = rename_start + rename_buffer_range.len();
                    let range = buffer.anchor_before(rename_start)..buffer.anchor_after(rename_end);
                    let old_name = buffer
                        .text_for_range(rename_start..rename_end)
                        .collect::<String>();
                    drop(buffer);

                    // Position the selection in the rename editor so that it matches the current selection.
                    let rename_editor = cx.add_view(|cx| {
                        let mut editor = Editor::single_line(this.settings.clone(), None, cx);
                        editor
                            .buffer
                            .update(cx, |buffer, cx| buffer.edit([0..0], &old_name, cx));
                        editor.select_ranges(
                            [tail_offset_in_rename_range..cursor_offset_in_rename_range],
                            None,
                            cx,
                        );
                        editor
                    });
                    this.highlight_text::<Rename>(
                        vec![range.clone()],
                        HighlightStyle {
                            color: Color::transparent_black(),
                            font_properties: todo!(),
                            underline: todo!(),
                            fade_out: todo!(),
                        },
                        cx,
                    );
                    this.update_selections(
                        vec![Selection {
                            id: selection.id,
                            start: rename_end,
                            end: rename_end,
                            reversed: false,
                            goal: SelectionGoal::None,
                        }],
                        None,
                        cx,
                    );
                    cx.focus(&rename_editor);
                    let block_id = this.insert_blocks(
                        [BlockProperties {
                            position: range.start.clone(),
                            height: 1,
                            render: Arc::new({
                                let editor = rename_editor.clone();
                                move |cx: &BlockContext| {
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
            let rename = editor.take_rename(cx)?;
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

        Some(cx.spawn(|workspace, cx| async move {
            let project_transaction = rename.await?;
            Self::open_project_transaction(
                editor,
                workspace,
                project_transaction,
                format!("Rename: {} → {}", old_name, new_name),
                cx,
            )
            .await
        }))
    }

    fn take_rename(&mut self, cx: &mut ViewContext<Self>) -> Option<RenameState> {
        let rename = self.pending_rename.take()?;
        self.remove_blocks([rename.block_id].into_iter().collect(), cx);
        self.clear_background_highlights::<Rename>(cx);

        let editor = rename.editor.read(cx);
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let selection = editor.newest_selection_with_snapshot::<usize>(&snapshot);

        // Update the selection to match the position of the selection inside
        // the rename editor.
        let rename_range = rename.range.to_offset(&snapshot);
        let start = snapshot
            .clip_offset(rename_range.start + selection.start, Bias::Left)
            .min(rename_range.end);
        let end = snapshot
            .clip_offset(rename_range.start + selection.end, Bias::Left)
            .min(rename_range.end);
        self.update_selections(
            vec![Selection {
                id: self.newest_anchor_selection().id,
                start,
                end,
                reversed: selection.reversed,
                goal: SelectionGoal::None,
            }],
            None,
            cx,
        );

        Some(rename)
    }

    fn invalidate_rename_range(
        &mut self,
        buffer: &MultiBufferSnapshot,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(rename) = self.pending_rename.as_ref() {
            if self.selections.len() == 1 {
                let head = self.selections[0].head().to_offset(buffer);
                let range = rename.range.to_offset(buffer).to_inclusive();
                if range.contains(&head) {
                    return;
                }
            }
            let rename = self.pending_rename.take().unwrap();
            self.remove_blocks([rename.block_id].into_iter().collect(), cx);
            self.clear_background_highlights::<Rename>(cx);
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn pending_rename(&self) -> Option<&RenameState> {
        self.pending_rename.as_ref()
    }

    fn refresh_active_diagnostics(&mut self, cx: &mut ViewContext<Editor>) {
        if let Some(active_diagnostics) = self.active_diagnostics.as_mut() {
            let buffer = self.buffer.read(cx).snapshot(cx);
            let primary_range_start = active_diagnostics.primary_range.start.to_offset(&buffer);
            let is_valid = buffer
                .diagnostics_in_range::<_, usize>(active_diagnostics.primary_range.clone())
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
                        diagnostic_block_renderer(
                            diagnostic.clone(),
                            is_valid,
                            self.settings.clone(),
                        ),
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
                            render: diagnostic_block_renderer(
                                diagnostic,
                                true,
                                self.settings.clone(),
                            ),
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

    fn build_columnar_selection(
        &mut self,
        display_map: &DisplaySnapshot,
        row: u32,
        columns: &Range<u32>,
        reversed: bool,
    ) -> Option<Selection<Point>> {
        let is_empty = columns.start == columns.end;
        let line_len = display_map.line_len(row);
        if columns.start < line_len || (is_empty && columns.start == line_len) {
            let start = DisplayPoint::new(row, columns.start);
            let end = DisplayPoint::new(row, cmp::min(columns.end, line_len));
            Some(Selection {
                id: post_inc(&mut self.next_selection_id),
                start: start.to_point(display_map),
                end: end.to_point(display_map),
                reversed,
                goal: SelectionGoal::ColumnRange {
                    start: columns.start,
                    end: columns.end,
                },
            })
        } else {
            None
        }
    }

    pub fn local_selections_in_range(
        &self,
        range: Range<Anchor>,
        display_map: &DisplaySnapshot,
    ) -> Vec<Selection<Point>> {
        let buffer = &display_map.buffer_snapshot;

        let start_ix = match self
            .selections
            .binary_search_by(|probe| probe.end.cmp(&range.start, &buffer).unwrap())
        {
            Ok(ix) | Err(ix) => ix,
        };
        let end_ix = match self
            .selections
            .binary_search_by(|probe| probe.start.cmp(&range.end, &buffer).unwrap())
        {
            Ok(ix) => ix + 1,
            Err(ix) => ix,
        };

        fn point_selection(
            selection: &Selection<Anchor>,
            buffer: &MultiBufferSnapshot,
        ) -> Selection<Point> {
            let start = selection.start.to_point(&buffer);
            let end = selection.end.to_point(&buffer);
            Selection {
                id: selection.id,
                start,
                end,
                reversed: selection.reversed,
                goal: selection.goal,
            }
        }

        self.selections[start_ix..end_ix]
            .iter()
            .chain(
                self.pending_selection
                    .as_ref()
                    .map(|pending| &pending.selection),
            )
            .map(|s| point_selection(s, &buffer))
            .collect()
    }

    pub fn local_selections<'a, D>(&self, cx: &'a AppContext) -> Vec<Selection<D>>
    where
        D: 'a + TextDimension + Ord + Sub<D, Output = D>,
    {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selections = self
            .resolve_selections::<D, _>(self.selections.iter(), &buffer)
            .peekable();

        let mut pending_selection = self.pending_selection::<D>(&buffer);

        iter::from_fn(move || {
            if let Some(pending) = pending_selection.as_mut() {
                while let Some(next_selection) = selections.peek() {
                    if pending.start <= next_selection.end && pending.end >= next_selection.start {
                        let next_selection = selections.next().unwrap();
                        if next_selection.start < pending.start {
                            pending.start = next_selection.start;
                        }
                        if next_selection.end > pending.end {
                            pending.end = next_selection.end;
                        }
                    } else if next_selection.end < pending.start {
                        return selections.next();
                    } else {
                        break;
                    }
                }

                pending_selection.take()
            } else {
                selections.next()
            }
        })
        .collect()
    }

    fn resolve_selections<'a, D, I>(
        &self,
        selections: I,
        snapshot: &MultiBufferSnapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
        I: 'a + IntoIterator<Item = &'a Selection<Anchor>>,
    {
        let (to_summarize, selections) = selections.into_iter().tee();
        let mut summaries = snapshot
            .summaries_for_anchors::<D, _>(to_summarize.flat_map(|s| [&s.start, &s.end]))
            .into_iter();
        selections.map(move |s| Selection {
            id: s.id,
            start: summaries.next().unwrap(),
            end: summaries.next().unwrap(),
            reversed: s.reversed,
            goal: s.goal,
        })
    }

    fn pending_selection<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        snapshot: &MultiBufferSnapshot,
    ) -> Option<Selection<D>> {
        self.pending_selection
            .as_ref()
            .map(|pending| self.resolve_selection(&pending.selection, &snapshot))
    }

    fn resolve_selection<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        selection: &Selection<Anchor>,
        buffer: &MultiBufferSnapshot,
    ) -> Selection<D> {
        Selection {
            id: selection.id,
            start: selection.start.summary::<D>(&buffer),
            end: selection.end.summary::<D>(&buffer),
            reversed: selection.reversed,
            goal: selection.goal,
        }
    }

    fn selection_count<'a>(&self) -> usize {
        let mut count = self.selections.len();
        if self.pending_selection.is_some() {
            count += 1;
        }
        count
    }

    pub fn oldest_selection<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &AppContext,
    ) -> Selection<D> {
        let snapshot = self.buffer.read(cx).read(cx);
        self.selections
            .iter()
            .min_by_key(|s| s.id)
            .map(|selection| self.resolve_selection(selection, &snapshot))
            .or_else(|| self.pending_selection(&snapshot))
            .unwrap()
    }

    pub fn newest_selection<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &AppContext,
    ) -> Selection<D> {
        self.resolve_selection(
            self.newest_anchor_selection(),
            &self.buffer.read(cx).read(cx),
        )
    }

    pub fn newest_selection_with_snapshot<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        snapshot: &MultiBufferSnapshot,
    ) -> Selection<D> {
        self.resolve_selection(self.newest_anchor_selection(), snapshot)
    }

    pub fn newest_anchor_selection(&self) -> &Selection<Anchor> {
        self.pending_selection
            .as_ref()
            .map(|s| &s.selection)
            .or_else(|| self.selections.iter().max_by_key(|s| s.id))
            .unwrap()
    }

    pub fn update_selections<T>(
        &mut self,
        mut selections: Vec<Selection<T>>,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) where
        T: ToOffset + ToPoint + Ord + std::marker::Copy + std::fmt::Debug,
    {
        let buffer = self.buffer.read(cx).snapshot(cx);
        selections.sort_unstable_by_key(|s| s.start);

        // Merge overlapping selections.
        let mut i = 1;
        while i < selections.len() {
            if selections[i - 1].end >= selections[i].start {
                let removed = selections.remove(i);
                if removed.start < selections[i - 1].start {
                    selections[i - 1].start = removed.start;
                }
                if removed.end > selections[i - 1].end {
                    selections[i - 1].end = removed.end;
                }
            } else {
                i += 1;
            }
        }

        if let Some(autoscroll) = autoscroll {
            self.request_autoscroll(autoscroll, cx);
        }

        self.set_selections(
            Arc::from_iter(selections.into_iter().map(|selection| {
                let end_bias = if selection.end > selection.start {
                    Bias::Left
                } else {
                    Bias::Right
                };
                Selection {
                    id: selection.id,
                    start: buffer.anchor_after(selection.start),
                    end: buffer.anchor_at(selection.end, end_bias),
                    reversed: selection.reversed,
                    goal: selection.goal,
                }
            })),
            None,
            cx,
        );
    }

    /// Compute new ranges for any selections that were located in excerpts that have
    /// since been removed.
    ///
    /// Returns a `HashMap` indicating which selections whose former head position
    /// was no longer present. The keys of the map are selection ids. The values are
    /// the id of the new excerpt where the head of the selection has been moved.
    pub fn refresh_selections(&mut self, cx: &mut ViewContext<Self>) -> HashMap<usize, ExcerptId> {
        let snapshot = self.buffer.read(cx).read(cx);
        let anchors_with_status = snapshot.refresh_anchors(
            self.selections
                .iter()
                .flat_map(|selection| [&selection.start, &selection.end]),
        );
        let offsets =
            snapshot.summaries_for_anchors::<usize, _>(anchors_with_status.iter().map(|a| &a.1));
        let offsets = offsets.chunks(2);
        let statuses = anchors_with_status
            .chunks(2)
            .map(|a| (a[0].0 / 2, a[0].2, a[1].2));

        let mut selections_with_lost_position = HashMap::default();
        let new_selections = offsets
            .zip(statuses)
            .map(|(offsets, (selection_ix, kept_start, kept_end))| {
                let selection = &self.selections[selection_ix];
                let kept_head = if selection.reversed {
                    kept_start
                } else {
                    kept_end
                };
                if !kept_head {
                    selections_with_lost_position
                        .insert(selection.id, selection.head().excerpt_id.clone());
                }

                Selection {
                    id: selection.id,
                    start: offsets[0],
                    end: offsets[1],
                    reversed: selection.reversed,
                    goal: selection.goal,
                }
            })
            .collect();
        drop(snapshot);
        self.update_selections(new_selections, Some(Autoscroll::Fit), cx);
        selections_with_lost_position
    }

    fn set_selections(
        &mut self,
        selections: Arc<[Selection<Anchor>]>,
        pending_selection: Option<PendingSelection>,
        cx: &mut ViewContext<Self>,
    ) {
        let old_cursor_position = self.newest_anchor_selection().head();

        self.selections = selections;
        self.pending_selection = pending_selection;
        if self.focused {
            self.buffer.update(cx, |buffer, cx| {
                buffer.set_active_selections(&self.selections, cx)
            });
        }

        let display_map = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        self.add_selections_state = None;
        self.select_next_state = None;
        self.select_larger_syntax_node_stack.clear();
        self.autoclose_stack.invalidate(&self.selections, &buffer);
        self.snippet_stack.invalidate(&self.selections, &buffer);
        self.invalidate_rename_range(&buffer, cx);

        let new_cursor_position = self.newest_anchor_selection().head();

        self.push_to_nav_history(
            old_cursor_position.clone(),
            Some(new_cursor_position.to_point(&buffer)),
            cx,
        );

        let completion_menu = match self.context_menu.as_mut() {
            Some(ContextMenu::Completions(menu)) => Some(menu),
            _ => {
                self.context_menu.take();
                None
            }
        };

        if let Some(completion_menu) = completion_menu {
            let cursor_position = new_cursor_position.to_offset(&buffer);
            let (word_range, kind) =
                buffer.surrounding_word(completion_menu.initial_position.clone());
            if kind == Some(CharKind::Word) && word_range.to_inclusive().contains(&cursor_position)
            {
                let query = Self::completion_query(&buffer, cursor_position);
                cx.background()
                    .block(completion_menu.filter(query.as_deref(), cx.background().clone()));
                self.show_completions(&ShowCompletions, cx);
            } else {
                self.hide_context_menu(cx);
            }
        }

        if old_cursor_position.to_display_point(&display_map).row()
            != new_cursor_position.to_display_point(&display_map).row()
        {
            self.available_code_actions.take();
        }
        self.refresh_code_actions(cx);
        self.refresh_document_highlights(cx);

        self.pause_cursor_blinking(cx);
        cx.emit(Event::SelectionsChanged);
    }

    pub fn request_autoscroll(&mut self, autoscroll: Autoscroll, cx: &mut ViewContext<Self>) {
        self.autoscroll_request = Some(autoscroll);
        cx.notify();
    }

    fn start_transaction(&mut self, cx: &mut ViewContext<Self>) {
        self.start_transaction_at(Instant::now(), cx);
    }

    fn start_transaction_at(&mut self, now: Instant, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        if let Some(tx_id) = self
            .buffer
            .update(cx, |buffer, cx| buffer.start_transaction_at(now, cx))
        {
            self.selection_history
                .insert(tx_id, (self.selections.clone(), None));
        }
    }

    fn end_transaction(&mut self, cx: &mut ViewContext<Self>) {
        self.end_transaction_at(Instant::now(), cx);
    }

    fn end_transaction_at(&mut self, now: Instant, cx: &mut ViewContext<Self>) {
        if let Some(tx_id) = self
            .buffer
            .update(cx, |buffer, cx| buffer.end_transaction_at(now, cx))
        {
            if let Some((_, end_selections)) = self.selection_history.get_mut(&tx_id) {
                *end_selections = Some(self.selections.clone());
            } else {
                log::error!("unexpectedly ended a transaction that wasn't started by this editor");
            }
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

        let selections = self.local_selections::<Point>(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
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

    pub fn unfold(&mut self, _: &Unfold, cx: &mut ViewContext<Self>) {
        let selections = self.local_selections::<Point>(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
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
        self.unfold_ranges(ranges, cx);
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
        let selections = self.local_selections::<Point>(cx);
        let ranges = selections.into_iter().map(|s| s.start..s.end);
        self.fold_ranges(ranges, cx);
    }

    fn fold_ranges<T: ToOffset>(
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

    fn unfold_ranges<T: ToOffset>(&mut self, ranges: Vec<Range<T>>, cx: &mut ViewContext<Self>) {
        if !ranges.is_empty() {
            self.display_map
                .update(cx, |map, cx| map.unfold(ranges, cx));
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

    pub fn set_text(&mut self, text: impl Into<String>, cx: &mut ViewContext<Self>) {
        self.buffer
            .read(cx)
            .as_singleton()
            .expect("you can only call set_text on editors for singleton buffers")
            .update(cx, |buffer, cx| buffer.set_text(text, cx));
    }

    pub fn display_text(&self, cx: &mut MutableAppContext) -> String {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .text()
    }

    pub fn soft_wrap_mode(&self, cx: &AppContext) -> SoftWrap {
        let language = self.language(cx);
        let settings = self.settings.borrow();
        let mode = self
            .soft_wrap_mode_override
            .unwrap_or_else(|| settings.soft_wrap(language));
        match mode {
            settings::SoftWrap::None => SoftWrap::None,
            settings::SoftWrap::EditorWidth => SoftWrap::EditorWidth,
            settings::SoftWrap::PreferredLineLength => {
                SoftWrap::Column(settings.preferred_line_length(language))
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
        color: Color,
        cx: &mut ViewContext<Self>,
    ) {
        self.background_highlights
            .insert(TypeId::of::<T>(), (color, ranges));
        cx.notify();
    }

    pub fn clear_background_highlights<T: 'static>(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<(Color, Vec<Range<Anchor>>)> {
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
        self.background_highlights_in_range(start..end, &snapshot)
    }

    pub fn background_highlights_for_type<T: 'static>(&self) -> Option<(Color, &[Range<Anchor>])> {
        self.background_highlights
            .get(&TypeId::of::<T>())
            .map(|(color, ranges)| (*color, ranges.as_slice()))
    }

    pub fn background_highlights_in_range(
        &self,
        search_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
    ) -> Vec<(Range<DisplayPoint>, Color)> {
        let mut results = Vec::new();
        let buffer = &display_snapshot.buffer_snapshot;
        for (color, ranges) in self.background_highlights.values() {
            let start_ix = match ranges.binary_search_by(|probe| {
                let cmp = probe.end.cmp(&search_range.start, &buffer).unwrap();
                if cmp.is_gt() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }) {
                Ok(i) | Err(i) => i,
            };
            for range in &ranges[start_ix..] {
                if range.start.cmp(&search_range.end, &buffer).unwrap().is_ge() {
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
                results.push((start..end, *color))
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

    pub fn clear_text_highlights<T: 'static>(&mut self, cx: &mut ViewContext<Self>) {
        self.display_map
            .update(cx, |map, _| map.clear_text_highlights(TypeId::of::<T>()));
        cx.notify();
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
        self.show_local_cursors
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
                cx.emit(Event::Edited);
            }
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
        for selection in editor.local_selections::<usize>(cx) {
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
            editor.push_to_nav_history(editor.newest_anchor_selection().head(), None, cx);
        });
        let nav_history = workspace.active_pane().read(cx).nav_history().clone();
        nav_history.borrow_mut().disable();

        // We defer the pane interaction because we ourselves are a workspace item
        // and activating a new item causes the pane to call a method on us reentrantly,
        // which panics if we're on the stack.
        cx.defer(move |workspace, cx| {
            for (ix, (buffer, ranges)) in new_selections_by_buffer.into_iter().enumerate() {
                let buffer = BufferItemHandle(buffer);
                if ix == 0 && !workspace.activate_pane_for_item(&buffer, cx) {
                    workspace.activate_next_pane(cx);
                }

                let editor = workspace
                    .open_item(buffer, cx)
                    .downcast::<Editor>()
                    .unwrap();

                editor.update(cx, |editor, cx| {
                    editor.select_ranges(ranges, Some(Autoscroll::Newest), cx);
                });
            }

            nav_history.borrow_mut().enable();
        });
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
    scroll_top_anchor: &Option<Anchor>,
) -> Vector2F {
    if let Some(anchor) = scroll_top_anchor {
        let scroll_top = anchor.to_display_point(snapshot).row() as f32;
        scroll_position.set_y(scroll_top + scroll_position.y());
    } else {
        scroll_position.set_y(0.);
    }
    scroll_position
}

#[derive(Copy, Clone)]
pub enum Event {
    Activate,
    Edited,
    Blurred,
    Dirtied,
    Saved,
    TitleChanged,
    SelectionsChanged,
    Closed,
}

impl Entity for Editor {
    type Event = Event;
}

impl View for Editor {
    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let style = self.style(cx);
        self.display_map.update(cx, |map, cx| {
            map.set_font(style.text.font_id, style.text.font_size, cx)
        });
        EditorElement::new(self.handle.clone(), style.clone()).boxed()
    }

    fn ui_name() -> &'static str {
        "Editor"
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.focused = true;
        self.blink_cursors(self.blink_epoch, cx);
        self.buffer.update(cx, |buffer, cx| {
            buffer.finalize_last_transaction(cx);
            buffer.set_active_selections(&self.selections, cx)
        });
    }

    fn on_blur(&mut self, cx: &mut ViewContext<Self>) {
        self.focused = false;
        self.show_local_cursors = false;
        self.buffer
            .update(cx, |buffer, cx| buffer.remove_active_selections(cx));
        self.hide_context_menu(cx);
        cx.emit(Event::Blurred);
        cx.notify();
    }

    fn keymap_context(&self, _: &AppContext) -> gpui::keymap::Context {
        let mut cx = Self::default_keymap_context();
        let mode = match self.mode {
            EditorMode::SingleLine => "single_line",
            EditorMode::AutoHeight { .. } => "auto_height",
            EditorMode::Full => "full",
        };
        cx.map.insert("mode".into(), mode.into());
        if self.pending_rename.is_some() {
            cx.set.insert("renaming".into());
        }
        match self.context_menu.as_ref() {
            Some(ContextMenu::Completions(_)) => {
                cx.set.insert("showing_completions".into());
            }
            Some(ContextMenu::CodeActions(_)) => {
                cx.set.insert("showing_code_actions".into());
            }
            None => {}
        }
        cx
    }
}

fn build_style(
    settings: &Settings,
    get_field_editor_theme: Option<GetFieldEditorTheme>,
    cx: &AppContext,
) -> EditorStyle {
    let mut theme = settings.theme.editor.clone();
    if let Some(get_field_editor_theme) = get_field_editor_theme {
        let field_editor_theme = get_field_editor_theme(&settings.theme);
        if let Some(background) = field_editor_theme.container.background_color {
            theme.background = background;
        }
        theme.text_color = field_editor_theme.text.color;
        theme.selection = field_editor_theme.selection;
        EditorStyle {
            text: field_editor_theme.text,
            placeholder_text: field_editor_theme.placeholder_text,
            theme,
        }
    } else {
        let font_cache = cx.font_cache();
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
                underline: None,
            },
            placeholder_text: None,
            theme,
        }
    }
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

pub fn diagnostic_block_renderer(
    diagnostic: Diagnostic,
    is_valid: bool,
    settings: watch::Receiver<Settings>,
) -> RenderBlock {
    let mut highlighted_lines = Vec::new();
    for line in diagnostic.message.lines() {
        highlighted_lines.push(highlight_diagnostic_message(line));
    }

    Arc::new(move |cx: &BlockContext| {
        let settings = settings.borrow();
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
        syntax_highlight.font_properties.weight(Default::default());

        // Add highlights for any fuzzy match characters before the next
        // syntax highlight range.
        while let Some(&match_index) = match_indices.peek() {
            if match_index >= range.start {
                break;
            }
            match_indices.next();
            let end_index = char_ix_after(match_index, text);
            let mut match_style = default_style;
            match_style.font_properties.weight(fonts::Weight::BOLD);
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
            match_style.font_properties.weight(fonts::Weight::BOLD);
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
    default_color: Color,
    syntax_theme: &'a theme::SyntaxTheme,
) -> impl 'a + Iterator<Item = (Range<usize>, HighlightStyle)> {
    const MUTED_OPACITY: usize = 165;

    let mut muted_default_style = HighlightStyle {
        color: default_color,
        ..Default::default()
    };
    muted_default_style.color.a = ((default_color.a as usize * MUTED_OPACITY) / 255) as u8;

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
            muted_style.color.a = ((style.color.a as usize * MUTED_OPACITY) / 255) as u8;

            let mut runs = SmallVec::<[(Range<usize>, HighlightStyle); 3]>::new();
            if range.start >= label.filter_range.end {
                if range.start > prev_end {
                    runs.push((prev_end..range.start, muted_default_style));
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
                runs.push((prev_end..label.text.len(), muted_default_style));
            }

            runs
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use language::{LanguageConfig, LanguageServerConfig};
    use lsp::FakeLanguageServer;
    use project::FakeFs;
    use smol::stream::StreamExt;
    use std::{cell::RefCell, rc::Rc, time::Instant};
    use text::Point;
    use unindent::Unindent;
    use util::test::sample_text;

    #[gpui::test]
    fn test_undo_redo_with_selection_restoration(cx: &mut MutableAppContext) {
        let mut now = Instant::now();
        let buffer = cx.add_model(|cx| language::Buffer::new(0, "123456", cx));
        let group_interval = buffer.read(cx).transaction_group_interval();
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let settings = Settings::test(cx);
        let (_, editor) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        editor.update(cx, |editor, cx| {
            editor.start_transaction_at(now, cx);
            editor.select_ranges([2..4], None, cx);
            editor.insert("cd", cx);
            editor.end_transaction_at(now, cx);
            assert_eq!(editor.text(cx), "12cd56");
            assert_eq!(editor.selected_ranges(cx), vec![4..4]);

            editor.start_transaction_at(now, cx);
            editor.select_ranges([4..5], None, cx);
            editor.insert("e", cx);
            editor.end_transaction_at(now, cx);
            assert_eq!(editor.text(cx), "12cde6");
            assert_eq!(editor.selected_ranges(cx), vec![5..5]);

            now += group_interval + Duration::from_millis(1);
            editor.select_ranges([2..2], None, cx);

            // Simulate an edit in another editor
            buffer.update(cx, |buffer, cx| {
                buffer.start_transaction_at(now, cx);
                buffer.edit([0..1], "a", cx);
                buffer.edit([1..1], "b", cx);
                buffer.end_transaction_at(now, cx);
            });

            assert_eq!(editor.text(cx), "ab2cde6");
            assert_eq!(editor.selected_ranges(cx), vec![3..3]);

            // Last transaction happened past the group interval in a different editor.
            // Undo it individually and don't restore selections.
            editor.undo(&Undo, cx);
            assert_eq!(editor.text(cx), "12cde6");
            assert_eq!(editor.selected_ranges(cx), vec![2..2]);

            // First two transactions happened within the group interval in this editor.
            // Undo them together and restore selections.
            editor.undo(&Undo, cx);
            editor.undo(&Undo, cx); // Undo stack is empty here, so this is a no-op.
            assert_eq!(editor.text(cx), "123456");
            assert_eq!(editor.selected_ranges(cx), vec![0..0]);

            // Redo the first two transactions together.
            editor.redo(&Redo, cx);
            assert_eq!(editor.text(cx), "12cde6");
            assert_eq!(editor.selected_ranges(cx), vec![5..5]);

            // Redo the last transaction on its own.
            editor.redo(&Redo, cx);
            assert_eq!(editor.text(cx), "ab2cde6");
            assert_eq!(editor.selected_ranges(cx), vec![6..6]);

            // Test empty transactions.
            editor.start_transaction_at(now, cx);
            editor.end_transaction_at(now, cx);
            editor.undo(&Undo, cx);
            assert_eq!(editor.text(cx), "12cde6");
        });
    }

    #[gpui::test]
    fn test_selection_with_mouse(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        let settings = Settings::test(cx);
        let (_, editor) =
            cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        editor.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(2, 2), false, 1, cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selected_display_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
        );

        editor.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selected_display_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );

        editor.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selected_display_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
        );

        editor.update(cx, |view, cx| {
            view.end_selection(cx);
            view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selected_display_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
        );

        editor.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(3, 3), true, 1, cx);
            view.update_selection(DisplayPoint::new(0, 0), 0, Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selected_display_ranges(cx)),
            [
                DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)
            ]
        );

        editor.update(cx, |view, cx| {
            view.end_selection(cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selected_display_ranges(cx)),
            [DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)]
        );
    }

    #[gpui::test]
    fn test_canceling_pending_selection(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        let settings = Settings::test(cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(2, 2), false, 1, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
            );
        });

        view.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
            );
        });
    }

    #[gpui::test]
    fn test_navigation_history(cx: &mut gpui::MutableAppContext) {
        cx.add_window(Default::default(), |cx| {
            use workspace::ItemView;
            let nav_history = Rc::new(RefCell::new(workspace::NavHistory::default()));
            let settings = Settings::test(&cx);
            let buffer = MultiBuffer::build_simple(&sample_text(30, 5, 'a'), cx);
            let mut editor = build_editor(buffer.clone(), settings, cx);
            editor.nav_history = Some(ItemNavHistory::new(nav_history.clone(), &cx.handle()));

            // Move the cursor a small distance.
            // Nothing is added to the navigation history.
            editor.select_display_ranges(&[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)], cx);
            editor.select_display_ranges(&[DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0)], cx);
            assert!(nav_history.borrow_mut().pop_backward().is_none());

            // Move the cursor a large distance.
            // The history can jump back to the previous position.
            editor.select_display_ranges(&[DisplayPoint::new(13, 0)..DisplayPoint::new(13, 3)], cx);
            let nav_entry = nav_history.borrow_mut().pop_backward().unwrap();
            editor.navigate(nav_entry.data.unwrap(), cx);
            assert_eq!(nav_entry.item_view.id(), cx.view_id());
            assert_eq!(
                editor.selected_display_ranges(cx),
                &[DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0)]
            );

            // Move the cursor a small distance via the mouse.
            // Nothing is added to the navigation history.
            editor.begin_selection(DisplayPoint::new(5, 0), false, 1, cx);
            editor.end_selection(cx);
            assert_eq!(
                editor.selected_display_ranges(cx),
                &[DisplayPoint::new(5, 0)..DisplayPoint::new(5, 0)]
            );
            assert!(nav_history.borrow_mut().pop_backward().is_none());

            // Move the cursor a large distance via the mouse.
            // The history can jump back to the previous position.
            editor.begin_selection(DisplayPoint::new(15, 0), false, 1, cx);
            editor.end_selection(cx);
            assert_eq!(
                editor.selected_display_ranges(cx),
                &[DisplayPoint::new(15, 0)..DisplayPoint::new(15, 0)]
            );
            let nav_entry = nav_history.borrow_mut().pop_backward().unwrap();
            editor.navigate(nav_entry.data.unwrap(), cx);
            assert_eq!(nav_entry.item_view.id(), cx.view_id());
            assert_eq!(
                editor.selected_display_ranges(cx),
                &[DisplayPoint::new(5, 0)..DisplayPoint::new(5, 0)]
            );

            editor
        });
    }

    #[gpui::test]
    fn test_cancel(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        let settings = Settings::test(cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(3, 4), false, 1, cx);
            view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
            view.end_selection(cx);

            view.begin_selection(DisplayPoint::new(0, 1), true, 1, cx);
            view.update_selection(DisplayPoint::new(0, 3), 0, Vector2F::zero(), cx);
            view.end_selection(cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                [
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                [DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1)]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                [DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1)]
            );
        });
    }

    #[gpui::test]
    fn test_fold(cx: &mut gpui::MutableAppContext) {
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
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(8, 0)..DisplayPoint::new(12, 0)], cx);
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

            view.unfold(&Unfold, cx);
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

            view.unfold(&Unfold, cx);
            assert_eq!(view.display_text(cx), buffer.read(cx).read(cx).text());
        });
    }

    #[gpui::test]
    fn test_move_cursor(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                vec![
                    Point::new(1, 0)..Point::new(1, 0),
                    Point::new(1, 1)..Point::new(1, 1),
                ],
                "\t",
                cx,
            );
        });

        view.update(cx, |view, cx| {
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );

            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4)]
            );

            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.move_to_end(&MoveToEnd, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(5, 6)..DisplayPoint::new(5, 6)]
            );

            view.move_to_beginning(&MoveToBeginning, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2)], cx);
            view.select_to_beginning(&SelectToBeginning, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 0)]
            );

            view.select_to_end(&SelectToEnd, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(0, 1)..DisplayPoint::new(5, 6)]
            );
        });
    }

    #[gpui::test]
    fn test_move_cursor_multibyte(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcde\nαβγδε\n", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

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
                view.selected_display_ranges(cx),
                &[empty_range(0, "ⓐ".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(0, "ⓐⓑ".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(0, "ⓐⓑ…".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(1, "ab…".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(1, "ab".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(1, "a".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(2, "α".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(2, "αβ".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(2, "αβ…".len())]
            );
            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(2, "αβ…ε".len())]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(1, "ab…e".len())]
            );
            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(0, "ⓐⓑ…ⓔ".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(0, "ⓐⓑ…".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(0, "ⓐⓑ".len())]
            );
            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(0, "ⓐ".len())]
            );
        });
    }

    #[gpui::test]
    fn test_move_cursor_different_line_lengths(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcd\nαβγ\nabcd\nⓐⓑⓒⓓⓔ\n", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[empty_range(0, "ⓐⓑⓒⓓⓔ".len())], cx);
            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(1, "abcd".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(2, "αβγ".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(3, "abcd".len())]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(4, "ⓐⓑⓒⓓⓔ".len())]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(3, "abcd".len())]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[empty_range(2, "αβγ".len())]
            );
        });
    }

    #[gpui::test]
    fn test_beginning_end_of_line(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("abc\n  def", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
                ],
                cx,
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_end_of_line(&MoveToEndOfLine, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_left(&MoveLeft, cx);
            view.select_to_beginning_of_line(&SelectToBeginningOfLine(true), cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_beginning_of_line(&SelectToBeginningOfLine(true), cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_beginning_of_line(&SelectToBeginningOfLine(true), cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_end_of_line(&SelectToEndOfLine(true), cx);
            assert_eq!(
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_prev_next_word_boundary(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("use std::str::{foo, bar}\n\n  {baz.qux()}", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 11)..DisplayPoint::new(0, 11),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4),
                ],
                cx,
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 3),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 4)..DisplayPoint::new(0, 4),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(0, 23)..DisplayPoint::new(0, 23),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(0, 24)..DisplayPoint::new(0, 24),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 3),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_right(&MoveRight, cx);
            view.select_to_previous_word_boundary(&SelectToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 3),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_previous_word_boundary(&SelectToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_next_word_boundary(&SelectToNextWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 3),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_prev_next_word_bounds_with_soft_wrap(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("use one::{\n    two::three::four::five\n};", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.set_wrap_width(Some(140.), cx);
            assert_eq!(
                view.display_text(cx),
                "use one::{\n    two::three::\n    four::five\n};"
            );

            view.select_display_ranges(&[DisplayPoint::new(1, 7)..DisplayPoint::new(1, 7)], cx);

            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 9)..DisplayPoint::new(1, 9)]
            );

            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 14)..DisplayPoint::new(1, 14)]
            );

            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4)]
            );

            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(2, 8)..DisplayPoint::new(2, 8)]
            );

            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4)]
            );

            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 14)..DisplayPoint::new(1, 14)]
            );
        });
    }

    #[gpui::test]
    fn test_delete_to_word_boundary(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("one two three four", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    // an empty selection - the preceding word fragment is deleted
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    // characters selected - they are deleted
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 12),
                ],
                cx,
            );
            view.delete_to_previous_word_boundary(&DeleteToPreviousWordBoundary, cx);
        });

        assert_eq!(buffer.read(cx).read(cx).text(), "e two te four");

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    // an empty selection - the following word fragment is deleted
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    // characters selected - they are deleted
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 10),
                ],
                cx,
            );
            view.delete_to_next_word_boundary(&DeleteToNextWordBoundary, cx);
        });

        assert_eq!(buffer.read(cx).read(cx).text(), "e t te our");
    }

    #[gpui::test]
    fn test_newline(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("aaaa\n    bbbb\n", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                    DisplayPoint::new(1, 6)..DisplayPoint::new(1, 6),
                ],
                cx,
            );

            view.newline(&Newline, cx);
            assert_eq!(view.text(cx), "aa\naa\n  \n    bb\n    bb\n");
        });
    }

    #[gpui::test]
    fn test_newline_with_old_selections(cx: &mut gpui::MutableAppContext) {
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

        let settings = Settings::test(&cx);
        let (_, editor) = cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(buffer.clone(), settings, cx);
            editor.select_ranges(
                [
                    Point::new(2, 4)..Point::new(2, 5),
                    Point::new(5, 4)..Point::new(5, 5),
                ],
                None,
                cx,
            );
            editor
        });

        // Edit the buffer directly, deleting ranges surrounding the editor's selections
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    Point::new(1, 2)..Point::new(3, 0),
                    Point::new(4, 2)..Point::new(6, 0),
                ],
                "",
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
                editor.selected_ranges(cx),
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
                editor.selected_ranges(cx),
                &[
                    Point::new(2, 0)..Point::new(2, 0),
                    Point::new(4, 0)..Point::new(4, 0),
                ],
            );
        });
    }

    #[gpui::test]
    fn test_insert_with_old_selections(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("a( X ), b( Y ), c( Z )", cx);

        let settings = Settings::test(&cx);
        let (_, editor) = cx.add_window(Default::default(), |cx| {
            let mut editor = build_editor(buffer.clone(), settings, cx);
            editor.select_ranges([3..4, 11..12, 19..20], None, cx);
            editor
        });

        // Edit the buffer directly, deleting ranges surrounding the editor's selections
        buffer.update(cx, |buffer, cx| {
            buffer.edit([2..5, 10..13, 18..21], "", cx);
            assert_eq!(buffer.read(cx).text(), "a(), b(), c()".unindent());
        });

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.selected_ranges(cx), &[2..2, 7..7, 12..12],);

            editor.insert("Z", cx);
            assert_eq!(editor.text(cx), "a(Z), b(Z), c(Z)");

            // The selections are moved after the inserted characters
            assert_eq!(editor.selected_ranges(cx), &[3..3, 9..9, 15..15],);
        });
    }

    #[gpui::test]
    fn test_indent_outdent(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("  one two\nthree\n four", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        view.update(cx, |view, cx| {
            // two selections on the same line
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 5),
                    DisplayPoint::new(0, 6)..DisplayPoint::new(0, 9),
                ],
                cx,
            );

            // indent from mid-tabstop to full tabstop
            view.tab(&Tab, cx);
            assert_eq!(view.text(cx), "    one two\nthree\n four");
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 4)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(0, 8)..DisplayPoint::new(0, 11),
                ]
            );

            // outdent from 1 tabstop to 0 tabstops
            view.outdent(&Outdent, cx);
            assert_eq!(view.text(cx), "one two\nthree\n four");
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(0, 4)..DisplayPoint::new(0, 7),
                ]
            );

            // select across line ending
            view.select_display_ranges(&[DisplayPoint::new(1, 1)..DisplayPoint::new(2, 0)], cx);

            // indent and outdent affect only the preceding line
            view.tab(&Tab, cx);
            assert_eq!(view.text(cx), "one two\n    three\n four");
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 5)..DisplayPoint::new(2, 0)]
            );
            view.outdent(&Outdent, cx);
            assert_eq!(view.text(cx), "one two\nthree\n four");
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 1)..DisplayPoint::new(2, 0)]
            );

            // Ensure that indenting/outdenting works when the cursor is at column 0.
            view.select_display_ranges(&[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)], cx);
            view.tab(&Tab, cx);
            assert_eq!(view.text(cx), "one two\n    three\n four");
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4)]
            );

            view.select_display_ranges(&[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)], cx);
            view.outdent(&Outdent, cx);
            assert_eq!(view.text(cx), "one two\nthree\n four");
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );
        });
    }

    #[gpui::test]
    fn test_backspace(cx: &mut gpui::MutableAppContext) {
        let buffer =
            MultiBuffer::build_simple("one two three\nfour five six\nseven eight nine\nten\n", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    // an empty selection - the preceding character is deleted
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    // one character selected - it is deleted
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                    // a line suffix selected - it is deleted
                    DisplayPoint::new(2, 6)..DisplayPoint::new(3, 0),
                ],
                cx,
            );
            view.backspace(&Backspace, cx);
        });

        assert_eq!(
            buffer.read(cx).read(cx).text(),
            "oe two three\nfou five six\nseven ten\n"
        );
    }

    #[gpui::test]
    fn test_delete(cx: &mut gpui::MutableAppContext) {
        let buffer =
            MultiBuffer::build_simple("one two three\nfour five six\nseven eight nine\nten\n", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    // an empty selection - the following character is deleted
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    // one character selected - it is deleted
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                    // a line suffix selected - it is deleted
                    DisplayPoint::new(2, 6)..DisplayPoint::new(3, 0),
                ],
                cx,
            );
            view.delete(&Delete, cx);
        });

        assert_eq!(
            buffer.read(cx).read(cx).text(),
            "on two three\nfou five six\nseven ten\n"
        );
    }

    #[gpui::test]
    fn test_delete_line(cx: &mut gpui::MutableAppContext) {
        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                ],
                cx,
            );
            view.delete_line(&DeleteLine, cx);
            assert_eq!(view.display_text(cx), "ghi");
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)
                ]
            );
        });

        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(2, 0)..DisplayPoint::new(0, 1)], cx);
            view.delete_line(&DeleteLine, cx);
            assert_eq!(view.display_text(cx), "ghi\n");
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)]
            );
        });
    }

    #[gpui::test]
    fn test_duplicate_line(cx: &mut gpui::MutableAppContext) {
        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                ],
                cx,
            );
            view.duplicate_line(&DuplicateLine, cx);
            assert_eq!(view.display_text(cx), "abc\nabc\ndef\ndef\nghi\n\n");
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(6, 0)..DisplayPoint::new(6, 0),
                ]
            );
        });

        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(2, 1),
                ],
                cx,
            );
            view.duplicate_line(&DuplicateLine, cx);
            assert_eq!(view.display_text(cx), "abc\ndef\nghi\nabc\ndef\nghi\n");
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(3, 1)..DisplayPoint::new(4, 1),
                    DisplayPoint::new(4, 2)..DisplayPoint::new(5, 1),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_move_line_up_down(cx: &mut gpui::MutableAppContext) {
        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.fold_ranges(
                vec![
                    Point::new(0, 2)..Point::new(1, 2),
                    Point::new(2, 3)..Point::new(4, 1),
                    Point::new(7, 0)..Point::new(8, 4),
                ],
                cx,
            );
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                    DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2),
                ],
                cx,
            );
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
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
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
        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
        let snapshot = buffer.read(cx).snapshot(cx);
        let (_, editor) =
            cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
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
            editor.select_ranges([Point::new(2, 0)..Point::new(2, 0)], None, cx);
            editor.move_line_down(&MoveLineDown, cx);
        });
    }

    #[gpui::test]
    fn test_clipboard(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("one✅ two three four five six ", cx);
        let settings = Settings::test(&cx);
        let view = cx
            .add_window(Default::default(), |cx| {
                build_editor(buffer.clone(), settings, cx)
            })
            .1;

        // Cut with three selections. Clipboard text is divided into three slices.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..7, 11..17, 22..27], None, cx);
            view.cut(&Cut, cx);
            assert_eq!(view.display_text(cx), "two four six ");
        });

        // Paste with three cursors. Each cursor pastes one slice of the clipboard text.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![4..4, 9..9, 13..13], None, cx);
            view.paste(&Paste, cx);
            assert_eq!(view.display_text(cx), "two one✅ four three six five ");
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 11)..DisplayPoint::new(0, 11),
                    DisplayPoint::new(0, 22)..DisplayPoint::new(0, 22),
                    DisplayPoint::new(0, 31)..DisplayPoint::new(0, 31)
                ]
            );
        });

        // Paste again but with only two cursors. Since the number of cursors doesn't
        // match the number of slices in the clipboard, the entire clipboard text
        // is pasted at each cursor.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..0, 31..31], None, cx);
            view.handle_input(&Input("( ".into()), cx);
            view.paste(&Paste, cx);
            view.handle_input(&Input(") ".into()), cx);
            assert_eq!(
                view.display_text(cx),
                "( one✅ three five ) two one✅ four three six five ( one✅ three five ) "
            );
        });

        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..0], None, cx);
            view.handle_input(&Input("123\n4567\n89\n".into()), cx);
            assert_eq!(
                view.display_text(cx),
                "123\n4567\n89\n( one✅ three five ) two one✅ four three six five ( one✅ three five ) "
            );
        });

        // Cut with three selections, one of which is full-line.
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 1),
                ],
                cx,
            );
            view.cut(&Cut, cx);
            assert_eq!(
                view.display_text(cx),
                "13\n9\n( one✅ three five ) two one✅ four three six five ( one✅ three five ) "
            );
        });

        // Paste with three selections, noticing how the copied selection that was full-line
        // gets inserted before the second cursor.
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(2, 3),
                ],
                cx,
            );
            view.paste(&Paste, cx);
            assert_eq!(
                view.display_text(cx),
                "123\n4567\n9\n( 8ne✅ three five ) two one✅ four three six five ( one✅ three five ) "
            );
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(3, 3)..DisplayPoint::new(3, 3),
                ]
            );
        });

        // Copy with a single cursor only, which writes the whole line into the clipboard.
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)], cx);
            view.copy(&Copy, cx);
        });

        // Paste with three selections, noticing how the copied full-line selection is inserted
        // before the empty selections but replaces the selection that is non-empty.
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 2),
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                ],
                cx,
            );
            view.paste(&Paste, cx);
            assert_eq!(
                view.display_text(cx),
                "123\n123\n123\n67\n123\n9\n( 8ne✅ three five ) two one✅ four three six five ( one✅ three five ) "
            );
            assert_eq!(
                view.selected_display_ranges(cx),
                &[
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(5, 1)..DisplayPoint::new(5, 1),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_select_all(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("abc\nde\nfgh", cx);
        let settings = Settings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_all(&SelectAll, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(2, 3)]
            );
        });
    }

    #[gpui::test]
    fn test_select_line(cx: &mut gpui::MutableAppContext) {
        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple(&sample_text(6, 5, 'a'), cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                    DisplayPoint::new(4, 2)..DisplayPoint::new(4, 2),
                ],
                cx,
            );
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(2, 0),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(5, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(5, 5),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![DisplayPoint::new(0, 0)..DisplayPoint::new(5, 5)]
            );
        });
    }

    #[gpui::test]
    fn test_split_selection_into_lines(cx: &mut gpui::MutableAppContext) {
        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple(&sample_text(9, 5, 'a'), cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.fold_ranges(
                vec![
                    Point::new(0, 2)..Point::new(1, 2),
                    Point::new(2, 3)..Point::new(4, 1),
                    Point::new(7, 0)..Point::new(8, 4),
                ],
                cx,
            );
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 4),
                ],
                cx,
            );
            assert_eq!(view.display_text(cx), "aa…bbb\nccc…eeee\nfffff\nggggg\n…i");
        });

        view.update(cx, |view, cx| {
            view.split_selection_into_lines(&SplitSelectionIntoLines, cx);
            assert_eq!(
                view.display_text(cx),
                "aaaaa\nbbbbb\nccc…eeee\nfffff\nggggg\n…i"
            );
            assert_eq!(
                view.selected_display_ranges(cx),
                [
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                    DisplayPoint::new(5, 4)..DisplayPoint::new(5, 4)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 1)], cx);
            view.split_selection_into_lines(&SplitSelectionIntoLines, cx);
            assert_eq!(
                view.display_text(cx),
                "aaaaa\nbbbbb\nccccc\nddddd\neeeee\nfffff\nggggg\nhhhhh\niiiii"
            );
            assert_eq!(
                view.selected_display_ranges(cx),
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
        let settings = Settings::test(&cx);
        let buffer = MultiBuffer::build_simple("abc\ndefghi\n\njk\nlmno\n", cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)], cx);
        });
        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)], cx);
        });
        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
                vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(1, 4)], cx);
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(4, 3)..DisplayPoint::new(1, 1)], cx);
        });
        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selected_display_ranges(cx),
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
                view.selected_display_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 1),
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_select_larger_smaller_syntax_node(cx: &mut gpui::TestAppContext) {
        let settings = cx.read(Settings::test);
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
        let (_, view) = cx.add_window(|cx| build_editor(buffer, settings, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
            .await;

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                    DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                    DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
                ],
                cx,
            );
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
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
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(cx, |view, cx| {
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
        );

        // Trying to expand the selected syntax node one more time has no effect.
        view.update(cx, |view, cx| {
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
        );

        view.update(cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
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
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
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
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
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
            view.update(cx, |view, cx| view.selected_display_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 4)..DisplayPoint::new(3, 23),
            ]
        );
    }

    #[gpui::test]
    async fn test_autoindent_selections(cx: &mut gpui::TestAppContext) {
        let settings = cx.read(Settings::test);
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
        let (_, editor) = cx.add_window(|cx| build_editor(buffer, settings, cx));
        editor
            .condition(&cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
            .await;

        editor.update(cx, |editor, cx| {
            editor.select_ranges([5..5, 8..8, 9..9], None, cx);
            editor.newline(&Newline, cx);
            assert_eq!(editor.text(cx), "fn a(\n    \n) {\n    \n}\n");
            assert_eq!(
                editor.selected_ranges(cx),
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
        let settings = cx.read(Settings::test);
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
        let (_, view) = cx.add_window(|cx| build_editor(buffer, settings, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
            .await;

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ],
                cx,
            );
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
            view.select_display_ranges(
                &[
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                ],
                cx,
            );
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

            view.finalize_last_transaction(cx);
            view.select_display_ranges(&[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)], cx);
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
            view.select_display_ranges(&[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1)], cx);
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
                view.selected_display_ranges(cx),
                [DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2)]
            );
        });
    }

    #[gpui::test]
    async fn test_snippets(cx: &mut gpui::TestAppContext) {
        let settings = cx.read(Settings::test);

        let text = "
            a. b
            a. b
            a. b
        "
        .unindent();
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));
        let (_, editor) = cx.add_window(|cx| build_editor(buffer, settings, cx));

        editor.update(cx, |editor, cx| {
            let buffer = &editor.snapshot(cx).buffer_snapshot;
            let snippet = Snippet::parse("f(${1:one}, ${2:two}, ${1:three})$0").unwrap();
            let insertion_ranges = [
                Point::new(0, 2).to_offset(buffer)..Point::new(0, 2).to_offset(buffer),
                Point::new(1, 2).to_offset(buffer)..Point::new(1, 2).to_offset(buffer),
                Point::new(2, 2).to_offset(buffer)..Point::new(2, 2).to_offset(buffer),
            ];

            editor
                .insert_snippet(&insertion_ranges, snippet, cx)
                .unwrap();
            assert_eq!(
                editor.text(cx),
                "
                    a.f(one, two, three) b
                    a.f(one, two, three) b
                    a.f(one, two, three) b
                "
                .unindent()
            );
            assert_eq!(
                editor.selected_ranges::<Point>(cx),
                &[
                    Point::new(0, 4)..Point::new(0, 7),
                    Point::new(0, 14)..Point::new(0, 19),
                    Point::new(1, 4)..Point::new(1, 7),
                    Point::new(1, 14)..Point::new(1, 19),
                    Point::new(2, 4)..Point::new(2, 7),
                    Point::new(2, 14)..Point::new(2, 19),
                ]
            );

            // Can't move earlier than the first tab stop
            editor.move_to_prev_snippet_tabstop(cx);
            assert_eq!(
                editor.selected_ranges::<Point>(cx),
                &[
                    Point::new(0, 4)..Point::new(0, 7),
                    Point::new(0, 14)..Point::new(0, 19),
                    Point::new(1, 4)..Point::new(1, 7),
                    Point::new(1, 14)..Point::new(1, 19),
                    Point::new(2, 4)..Point::new(2, 7),
                    Point::new(2, 14)..Point::new(2, 19),
                ]
            );

            assert!(editor.move_to_next_snippet_tabstop(cx));
            assert_eq!(
                editor.selected_ranges::<Point>(cx),
                &[
                    Point::new(0, 9)..Point::new(0, 12),
                    Point::new(1, 9)..Point::new(1, 12),
                    Point::new(2, 9)..Point::new(2, 12)
                ]
            );

            editor.move_to_prev_snippet_tabstop(cx);
            assert_eq!(
                editor.selected_ranges::<Point>(cx),
                &[
                    Point::new(0, 4)..Point::new(0, 7),
                    Point::new(0, 14)..Point::new(0, 19),
                    Point::new(1, 4)..Point::new(1, 7),
                    Point::new(1, 14)..Point::new(1, 19),
                    Point::new(2, 4)..Point::new(2, 7),
                    Point::new(2, 14)..Point::new(2, 19),
                ]
            );

            assert!(editor.move_to_next_snippet_tabstop(cx));
            assert!(editor.move_to_next_snippet_tabstop(cx));
            assert_eq!(
                editor.selected_ranges::<Point>(cx),
                &[
                    Point::new(0, 20)..Point::new(0, 20),
                    Point::new(1, 20)..Point::new(1, 20),
                    Point::new(2, 20)..Point::new(2, 20)
                ]
            );

            // As soon as the last tab stop is reached, snippet state is gone
            editor.move_to_prev_snippet_tabstop(cx);
            assert_eq!(
                editor.selected_ranges::<Point>(cx),
                &[
                    Point::new(0, 20)..Point::new(0, 20),
                    Point::new(1, 20)..Point::new(1, 20),
                    Point::new(2, 20)..Point::new(2, 20)
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_completion(cx: &mut gpui::TestAppContext) {
        let settings = cx.read(Settings::test);

        let (mut language_server_config, mut fake_servers) = LanguageServerConfig::fake();
        language_server_config.set_fake_capabilities(lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        });
        let language = Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                language_server: Some(language_server_config),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ));

        let text = "
            one
            two
            three
        "
        .unindent();

        let fs = FakeFs::new(cx.background().clone());
        fs.insert_file("/file.rs", text).await;

        let project = Project::test(fs, cx);
        project.update(cx, |project, _| project.languages().add(language));

        let worktree_id = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/file.rs", true, cx)
            })
            .await
            .unwrap()
            .0
            .read_with(cx, |tree, _| tree.id());
        let buffer = project
            .update(cx, |project, cx| project.open_buffer((worktree_id, ""), cx))
            .await
            .unwrap();
        let mut fake_server = fake_servers.next().await.unwrap();

        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let (_, editor) = cx.add_window(|cx| build_editor(buffer, settings, cx));

        editor.update(cx, |editor, cx| {
            editor.project = Some(project);
            editor.select_ranges([Point::new(0, 3)..Point::new(0, 3)], None, cx);
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
                .confirm_completion(&ConfirmCompletion(None), cx)
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
            editor.select_ranges(
                [
                    Point::new(1, 3)..Point::new(1, 3),
                    Point::new(2, 5)..Point::new(2, 5),
                ],
                None,
                cx,
            );

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
                .confirm_completion(&ConfirmCompletion(None), cx)
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
            fake.handle_request::<lsp::request::Completion, _>(move |params, _| {
                assert_eq!(
                    params.text_document_position.text_document.uri,
                    lsp::Url::from_file_path(path).unwrap()
                );
                assert_eq!(
                    params.text_document_position.position,
                    lsp::Position::new(position.row, position.column)
                );
                Some(lsp::CompletionResponse::Array(
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
                ))
            })
            .next()
            .await;
        }

        async fn handle_resolve_completion_request(
            fake: &mut FakeLanguageServer,
            edit: Option<(Range<Point>, &'static str)>,
        ) {
            fake.handle_request::<lsp::request::ResolveCompletionItem, _>(move |_, _| {
                lsp::CompletionItem {
                    additional_text_edits: edit.clone().map(|(range, new_text)| {
                        vec![lsp::TextEdit::new(
                            lsp::Range::new(
                                lsp::Position::new(range.start.row, range.start.column),
                                lsp::Position::new(range.end.row, range.end.column),
                            ),
                            new_text.to_string(),
                        )]
                    }),
                    ..Default::default()
                }
            })
            .next()
            .await;
        }
    }

    #[gpui::test]
    async fn test_toggle_comment(cx: &mut gpui::TestAppContext) {
        let settings = cx.read(Settings::test);
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
        let (_, view) = cx.add_window(|cx| build_editor(buffer, settings, cx));

        view.update(cx, |editor, cx| {
            // If multiple selections intersect a line, the line is only
            // toggled once.
            editor.select_display_ranges(
                &[
                    DisplayPoint::new(1, 3)..DisplayPoint::new(2, 3),
                    DisplayPoint::new(3, 5)..DisplayPoint::new(3, 6),
                ],
                cx,
            );
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
            editor.select_display_ranges(&[DisplayPoint::new(1, 3)..DisplayPoint::new(3, 6)], cx);
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
            editor.select_display_ranges(&[DisplayPoint::new(2, 0)..DisplayPoint::new(3, 0)], cx);
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
        let settings = Settings::test(cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            multibuffer.push_excerpts(
                buffer.clone(),
                [
                    Point::new(0, 0)..Point::new(0, 4),
                    Point::new(1, 0)..Point::new(1, 4),
                ],
                cx,
            );
            multibuffer
        });

        assert_eq!(multibuffer.read(cx).read(cx).text(), "aaaa\nbbbb");

        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(multibuffer, settings, cx)
        });
        view.update(cx, |view, cx| {
            assert_eq!(view.text(cx), "aaaa\nbbbb");
            view.select_ranges(
                [
                    Point::new(0, 0)..Point::new(0, 0),
                    Point::new(1, 0)..Point::new(1, 0),
                ],
                None,
                cx,
            );

            view.handle_input(&Input("X".to_string()), cx);
            assert_eq!(view.text(cx), "Xaaaa\nXbbbb");
            assert_eq!(
                view.selected_ranges(cx),
                [
                    Point::new(0, 1)..Point::new(0, 1),
                    Point::new(1, 1)..Point::new(1, 1),
                ]
            )
        });
    }

    #[gpui::test]
    fn test_editing_overlapping_excerpts(cx: &mut gpui::MutableAppContext) {
        let settings = Settings::test(cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            multibuffer.push_excerpts(
                buffer,
                [
                    Point::new(0, 0)..Point::new(1, 4),
                    Point::new(1, 0)..Point::new(2, 4),
                ],
                cx,
            );
            multibuffer
        });

        assert_eq!(
            multibuffer.read(cx).read(cx).text(),
            "aaaa\nbbbb\nbbbb\ncccc"
        );

        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(multibuffer, settings, cx)
        });
        view.update(cx, |view, cx| {
            view.select_ranges(
                [
                    Point::new(1, 1)..Point::new(1, 1),
                    Point::new(2, 3)..Point::new(2, 3),
                ],
                None,
                cx,
            );

            view.handle_input(&Input("X".to_string()), cx);
            assert_eq!(view.text(cx), "aaaa\nbXbbXb\nbXbbXb\ncccc");
            assert_eq!(
                view.selected_ranges(cx),
                [
                    Point::new(1, 2)..Point::new(1, 2),
                    Point::new(2, 5)..Point::new(2, 5),
                ]
            );

            view.newline(&Newline, cx);
            assert_eq!(view.text(cx), "aaaa\nbX\nbbX\nb\nbX\nbbX\nb\ncccc");
            assert_eq!(
                view.selected_ranges(cx),
                [
                    Point::new(2, 0)..Point::new(2, 0),
                    Point::new(6, 0)..Point::new(6, 0),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_refresh_selections(cx: &mut gpui::MutableAppContext) {
        let settings = Settings::test(cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
        let mut excerpt1_id = None;
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            excerpt1_id = multibuffer
                .push_excerpts(
                    buffer.clone(),
                    [
                        Point::new(0, 0)..Point::new(1, 4),
                        Point::new(1, 0)..Point::new(2, 4),
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
            let mut editor = build_editor(multibuffer.clone(), settings, cx);
            editor.select_ranges(
                [
                    Point::new(1, 3)..Point::new(1, 3),
                    Point::new(2, 1)..Point::new(2, 1),
                ],
                None,
                cx,
            );
            editor
        });

        // Refreshing selections is a no-op when excerpts haven't changed.
        editor.update(cx, |editor, cx| {
            editor.refresh_selections(cx);
            assert_eq!(
                editor.selected_ranges(cx),
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
                editor.selected_ranges(cx),
                [
                    Point::new(0, 0)..Point::new(0, 0),
                    Point::new(0, 1)..Point::new(0, 1)
                ]
            );

            // Refreshing selections will relocate the first selection to the original buffer
            // location.
            editor.refresh_selections(cx);
            assert_eq!(
                editor.selected_ranges(cx),
                [
                    Point::new(0, 1)..Point::new(0, 1),
                    Point::new(0, 3)..Point::new(0, 3)
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_extra_newline_insertion(cx: &mut gpui::TestAppContext) {
        let settings = cx.read(Settings::test);
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
        let (_, view) = cx.add_window(|cx| build_editor(buffer, settings, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
            .await;

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(2, 5)..DisplayPoint::new(2, 5),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 4),
                ],
                cx,
            );
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
        let settings = Settings::test(&cx);
        let (_, editor) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

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
                Color::red(),
                cx,
            );
            editor.highlight_background::<Type2>(
                vec![
                    anchor_range(Point::new(3, 2)..Point::new(3, 5)),
                    anchor_range(Point::new(5, 3)..Point::new(5, 6)),
                    anchor_range(Point::new(7, 4)..Point::new(7, 7)),
                    anchor_range(Point::new(9, 5)..Point::new(9, 8)),
                ],
                Color::green(),
                cx,
            );

            let snapshot = editor.snapshot(cx);
            let mut highlighted_ranges = editor.background_highlights_in_range(
                anchor_range(Point::new(3, 4)..Point::new(7, 4)),
                &snapshot,
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
                ),
                &[(
                    DisplayPoint::new(6, 3)..DisplayPoint::new(6, 5),
                    Color::red(),
                )]
            );
        });
    }

    #[test]
    fn test_combine_syntax_and_fuzzy_match_highlights() {
        let string = "abcdefghijklmnop";
        let default = HighlightStyle::default();
        let syntax_ranges = [
            (
                0..3,
                HighlightStyle {
                    color: Color::red(),
                    ..default
                },
            ),
            (
                4..8,
                HighlightStyle {
                    color: Color::green(),
                    ..default
                },
            ),
        ];
        let match_indices = [4, 6, 7, 8];
        assert_eq!(
            combine_syntax_and_fuzzy_match_highlights(
                &string,
                default,
                syntax_ranges.into_iter(),
                &match_indices,
            ),
            &[
                (
                    0..3,
                    HighlightStyle {
                        color: Color::red(),
                        ..default
                    },
                ),
                (
                    4..5,
                    HighlightStyle {
                        color: Color::green(),
                        font_properties: *fonts::Properties::default().weight(fonts::Weight::BOLD),
                        ..default
                    },
                ),
                (
                    5..6,
                    HighlightStyle {
                        color: Color::green(),
                        ..default
                    },
                ),
                (
                    6..8,
                    HighlightStyle {
                        color: Color::green(),
                        font_properties: *fonts::Properties::default().weight(fonts::Weight::BOLD),
                        ..default
                    },
                ),
                (
                    8..9,
                    HighlightStyle {
                        font_properties: *fonts::Properties::default().weight(fonts::Weight::BOLD),
                        ..default
                    },
                ),
            ]
        );
    }

    fn empty_range(row: usize, column: usize) -> Range<DisplayPoint> {
        let point = DisplayPoint::new(row as u32, column as u32);
        point..point
    }

    fn build_editor(
        buffer: ModelHandle<MultiBuffer>,
        settings: Settings,
        cx: &mut ViewContext<Editor>,
    ) -> Editor {
        let settings = watch::channel_with(settings);
        Editor::new(EditorMode::Full, buffer, None, settings.1, None, cx)
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

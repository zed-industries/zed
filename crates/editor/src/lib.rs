pub mod display_map;
mod element;
pub mod movement;

#[cfg(test)]
mod test;

use buffer::rope::TextDimension;
use clock::ReplicaId;
use display_map::*;
pub use display_map::{DisplayPoint, DisplayRow};
pub use element::*;
use gpui::{
    action,
    geometry::vector::{vec2f, Vector2F},
    keymap::Binding,
    text_layout, AppContext, ClipboardItem, Element, ElementBox, Entity, ModelHandle,
    MutableAppContext, RenderContext, View, ViewContext, WeakViewHandle,
};
use language::*;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smol::Timer;
use std::{
    cell::RefCell,
    cmp::{self, Ordering},
    collections::HashMap,
    iter, mem,
    ops::{Range, RangeInclusive},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use sum_tree::Bias;
use theme::{DiagnosticStyle, EditorStyle, SyntaxTheme};
use util::post_inc;

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);
const MAX_LINE_LEN: usize = 1024;

action!(Cancel);
action!(Backspace);
action!(Delete);
action!(Input, String);
action!(Newline);
action!(Tab);
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
action!(SelectToEndOfLine);
action!(SelectToBeginning);
action!(SelectToEnd);
action!(SelectAll);
action!(SelectLine);
action!(SplitSelectionIntoLines);
action!(AddSelectionAbove);
action!(AddSelectionBelow);
action!(SelectLargerSyntaxNode);
action!(SelectSmallerSyntaxNode);
action!(MoveToEnclosingBracket);
action!(ShowNextDiagnostic);
action!(PageUp);
action!(PageDown);
action!(Fold);
action!(Unfold);
action!(FoldSelectedRanges);
action!(Scroll, Vector2F);
action!(Select, SelectPhase);

pub fn init(cx: &mut MutableAppContext) {
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
        Binding::new("tab", Tab, Some("Editor")),
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
        Binding::new("cmd-shift-right", SelectToEndOfLine, Some("Editor")),
        Binding::new("ctrl-shift-E", SelectToEndOfLine, Some("Editor")),
        Binding::new("cmd-shift-up", SelectToBeginning, Some("Editor")),
        Binding::new("cmd-shift-down", SelectToEnd, Some("Editor")),
        Binding::new("cmd-a", SelectAll, Some("Editor")),
        Binding::new("cmd-l", SelectLine, Some("Editor")),
        Binding::new("cmd-shift-L", SplitSelectionIntoLines, Some("Editor")),
        Binding::new("cmd-alt-up", AddSelectionAbove, Some("Editor")),
        Binding::new("cmd-ctrl-p", AddSelectionAbove, Some("Editor")),
        Binding::new("cmd-alt-down", AddSelectionBelow, Some("Editor")),
        Binding::new("cmd-ctrl-n", AddSelectionBelow, Some("Editor")),
        Binding::new("alt-up", SelectLargerSyntaxNode, Some("Editor")),
        Binding::new("ctrl-w", SelectLargerSyntaxNode, Some("Editor")),
        Binding::new("alt-down", SelectSmallerSyntaxNode, Some("Editor")),
        Binding::new("ctrl-shift-W", SelectSmallerSyntaxNode, Some("Editor")),
        Binding::new("f8", ShowNextDiagnostic, Some("Editor")),
        Binding::new("ctrl-m", MoveToEnclosingBracket, Some("Editor")),
        Binding::new("pageup", PageUp, Some("Editor")),
        Binding::new("pagedown", PageDown, Some("Editor")),
        Binding::new("alt-cmd-[", Fold, Some("Editor")),
        Binding::new("alt-cmd-]", Unfold, Some("Editor")),
        Binding::new("alt-cmd-f", FoldSelectedRanges, Some("Editor")),
    ]);

    cx.add_action(|this: &mut Editor, action: &Scroll, cx| this.set_scroll_position(action.0, cx));
    cx.add_action(Editor::select);
    cx.add_action(Editor::cancel);
    cx.add_action(Editor::handle_input);
    cx.add_action(Editor::newline);
    cx.add_action(Editor::backspace);
    cx.add_action(Editor::delete);
    cx.add_action(Editor::tab);
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
    cx.add_action(Editor::select_larger_syntax_node);
    cx.add_action(Editor::select_smaller_syntax_node);
    cx.add_action(Editor::move_to_enclosing_bracket);
    cx.add_action(Editor::show_next_diagnostic);
    cx.add_action(Editor::page_up);
    cx.add_action(Editor::page_down);
    cx.add_action(Editor::fold);
    cx.add_action(Editor::unfold);
    cx.add_action(Editor::fold_selected_ranges);
}

trait SelectionExt {
    fn display_range(&self, map: &DisplayMapSnapshot) -> Range<DisplayPoint>;
    fn spanned_rows(
        &self,
        include_end_if_at_line_start: bool,
        map: &DisplayMapSnapshot,
    ) -> SpannedRows;
}

struct SpannedRows {
    buffer_rows: Range<u32>,
    display_rows: Range<u32>,
}

#[derive(Clone, Debug)]
pub enum SelectPhase {
    Begin {
        position: DisplayPoint,
        add: bool,
    },
    Update {
        position: DisplayPoint,
        scroll_position: Vector2F,
    },
    End,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EditorMode {
    SingleLine,
    AutoHeight { max_lines: usize },
    Full,
}

#[derive(Clone)]
pub struct EditorSettings {
    pub tab_size: usize,
    pub style: EditorStyle,
}

pub struct Editor {
    handle: WeakViewHandle<Self>,
    buffer: ModelHandle<Buffer>,
    display_map: ModelHandle<DisplayMap>,
    selection_set_id: SelectionSetId,
    pending_selection: Option<Selection<Anchor>>,
    next_selection_id: usize,
    add_selections_state: Option<AddSelectionsState>,
    autoclose_stack: Vec<BracketPairState>,
    select_larger_syntax_node_stack: Vec<Box<[Selection<usize>]>>,
    active_diagnostics: Option<ActiveDiagnosticGroup>,
    scroll_position: Vector2F,
    scroll_top_anchor: Anchor,
    autoscroll_requested: bool,
    build_settings: Rc<RefCell<dyn Fn(&AppContext) -> EditorSettings>>,
    focused: bool,
    show_local_cursors: bool,
    blink_epoch: usize,
    blinking_paused: bool,
    mode: EditorMode,
    placeholder_text: Option<Arc<str>>,
}

pub struct Snapshot {
    pub mode: EditorMode,
    pub display_snapshot: DisplayMapSnapshot,
    pub placeholder_text: Option<Arc<str>>,
    is_focused: bool,
    scroll_position: Vector2F,
    scroll_top_anchor: Anchor,
}

struct AddSelectionsState {
    above: bool,
    stack: Vec<usize>,
}

#[derive(Debug)]
struct BracketPairState {
    ranges: AnchorRangeSet,
    pair: BracketPair,
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

impl Editor {
    pub fn single_line(
        build_settings: impl 'static + Fn(&AppContext) -> EditorSettings,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let mut view = Self::for_buffer(buffer, build_settings, cx);
        view.mode = EditorMode::SingleLine;
        view
    }

    pub fn auto_height(
        max_lines: usize,
        build_settings: impl 'static + Fn(&AppContext) -> EditorSettings,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let mut view = Self::for_buffer(buffer, build_settings, cx);
        view.mode = EditorMode::AutoHeight { max_lines };
        view
    }

    pub fn for_buffer(
        buffer: ModelHandle<Buffer>,
        build_settings: impl 'static + Fn(&AppContext) -> EditorSettings,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::new(buffer, Rc::new(RefCell::new(build_settings)), cx)
    }

    pub fn clone(&self, cx: &mut ViewContext<Self>) -> Self {
        let mut clone = Self::new(self.buffer.clone(), self.build_settings.clone(), cx);
        clone.scroll_position = self.scroll_position;
        clone.scroll_top_anchor = self.scroll_top_anchor.clone();
        clone
    }

    pub fn new(
        buffer: ModelHandle<Buffer>,
        build_settings: Rc<RefCell<dyn Fn(&AppContext) -> EditorSettings>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let settings = build_settings.borrow_mut()(cx);
        let display_map = cx.add_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                settings.tab_size,
                settings.style.text.font_id,
                settings.style.text.font_size,
                None,
                cx,
            )
        });
        cx.observe(&buffer, Self::on_buffer_changed).detach();
        cx.subscribe(&buffer, Self::on_buffer_event).detach();
        cx.observe(&display_map, Self::on_display_map_changed)
            .detach();

        let mut next_selection_id = 0;
        let selection_set_id = buffer.update(cx, |buffer, cx| {
            buffer.add_selection_set(
                &[Selection {
                    id: post_inc(&mut next_selection_id),
                    start: 0,
                    end: 0,
                    reversed: false,
                    goal: SelectionGoal::None,
                }],
                cx,
            )
        });
        Self {
            handle: cx.handle().downgrade(),
            buffer,
            display_map,
            selection_set_id,
            pending_selection: None,
            next_selection_id,
            add_selections_state: None,
            autoclose_stack: Default::default(),
            select_larger_syntax_node_stack: Vec::new(),
            active_diagnostics: None,
            build_settings,
            scroll_position: Vector2F::zero(),
            scroll_top_anchor: Anchor::min(),
            autoscroll_requested: false,
            focused: false,
            show_local_cursors: false,
            blink_epoch: 0,
            blinking_paused: false,
            mode: EditorMode::Full,
            placeholder_text: None,
        }
    }

    pub fn replica_id(&self, cx: &AppContext) -> ReplicaId {
        self.buffer.read(cx).replica_id()
    }

    pub fn buffer(&self) -> &ModelHandle<Buffer> {
        &self.buffer
    }

    pub fn snapshot(&mut self, cx: &mut MutableAppContext) -> Snapshot {
        Snapshot {
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
        self.buffer.read(cx).language()
    }

    pub fn set_placeholder_text(
        &mut self,
        placeholder_text: impl Into<Arc<str>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.placeholder_text = Some(placeholder_text.into());
        cx.notify();
    }

    fn set_scroll_position(&mut self, scroll_position: Vector2F, cx: &mut ViewContext<Self>) {
        let map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let scroll_top_buffer_offset =
            DisplayPoint::new(scroll_position.y() as u32, 0).to_offset(&map, Bias::Right);
        self.scroll_top_anchor = self
            .buffer
            .read(cx)
            .anchor_at(scroll_top_buffer_offset, Bias::Right);
        self.scroll_position = vec2f(
            scroll_position.x(),
            scroll_position.y() - self.scroll_top_anchor.to_display_point(&map).row() as f32,
        );

        debug_assert_eq!(
            compute_scroll_position(&map, self.scroll_position, &self.scroll_top_anchor),
            scroll_position
        );

        cx.notify();
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

        if self.autoscroll_requested {
            self.autoscroll_requested = false;
        } else {
            return false;
        }

        let mut selections = self.selections::<Point>(cx).peekable();
        let first_cursor_top = selections
            .peek()
            .unwrap()
            .head()
            .to_display_point(&display_map)
            .row() as f32;
        let last_cursor_bottom = selections
            .last()
            .unwrap()
            .head()
            .to_display_point(&display_map)
            .row() as f32
            + 1.0;

        let margin = if matches!(self.mode, EditorMode::AutoHeight { .. }) {
            0.
        } else {
            ((visible_lines - (last_cursor_bottom - first_cursor_top)) / 2.0)
                .floor()
                .min(3.0)
        };
        if margin < 0.0 {
            return false;
        }

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
        let selections = self.selections::<Point>(cx);
        let mut target_left = std::f32::INFINITY;
        let mut target_right = 0.0_f32;
        for selection in selections {
            let head = selection.head().to_display_point(&display_map);
            let start_column = head.column().saturating_sub(3);
            let end_column = cmp::min(display_map.line_len(head.row()), head.column() + 3);
            target_left = target_left
                .min(layouts[(head.row() - start_row) as usize].x_for_index(start_column as usize));
            target_right = target_right.max(
                layouts[(head.row() - start_row) as usize].x_for_index(end_column as usize)
                    + max_glyph_width,
            );
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
        match phase {
            SelectPhase::Begin { position, add } => self.begin_selection(*position, *add, cx),
            SelectPhase::Update {
                position,
                scroll_position,
            } => self.update_selection(*position, *scroll_position, cx),
            SelectPhase::End => self.end_selection(cx),
        }
    }

    fn begin_selection(&mut self, position: DisplayPoint, add: bool, cx: &mut ViewContext<Self>) {
        if !self.focused {
            cx.focus_self();
            cx.emit(Event::Activate);
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);
        let cursor = buffer.anchor_before(position.to_point(&display_map));
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor.clone(),
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };

        if !add {
            self.update_selections::<usize>(Vec::new(), false, cx);
        }
        self.pending_selection = Some(selection);

        cx.notify();
    }

    fn update_selection(
        &mut self,
        position: DisplayPoint,
        scroll_position: Vector2F,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        if let Some(pending_selection) = self.pending_selection.as_mut() {
            let buffer = self.buffer.read(cx);
            let cursor = buffer.anchor_before(position.to_point(&display_map));
            if cursor.cmp(&pending_selection.tail(), buffer).unwrap() < Ordering::Equal {
                if !pending_selection.reversed {
                    pending_selection.end = pending_selection.start.clone();
                    pending_selection.reversed = true;
                }
                pending_selection.start = cursor;
            } else {
                if pending_selection.reversed {
                    pending_selection.start = pending_selection.end.clone();
                    pending_selection.reversed = false;
                }
                pending_selection.end = cursor;
            }
        } else {
            log::error!("update_selection dispatched with no pending selection");
            return;
        }

        self.set_scroll_position(scroll_position, cx);
        cx.notify();
    }

    fn end_selection(&mut self, cx: &mut ViewContext<Self>) {
        if self.pending_selection.is_some() {
            let selections = self.selections::<usize>(cx).collect::<Vec<_>>();
            self.update_selections(selections, false, cx);
        }
    }

    pub fn is_selecting(&self) -> bool {
        self.pending_selection.is_some()
    }

    pub fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if self.active_diagnostics.is_some() {
            self.dismiss_diagnostics(cx);
        } else if let Some(pending_selection) = self.pending_selection.take() {
            let buffer = self.buffer.read(cx);
            let pending_selection = Selection {
                id: pending_selection.id,
                start: pending_selection.start.to_point(buffer),
                end: pending_selection.end.to_point(buffer),
                reversed: pending_selection.reversed,
                goal: pending_selection.goal,
            };
            if self.selections::<Point>(cx).next().is_none() {
                self.update_selections(vec![pending_selection], true, cx);
            }
        } else {
            let mut oldest_selection = self.oldest_selection::<usize>(cx);
            if self.selection_count(cx) == 1 {
                oldest_selection.start = oldest_selection.head().clone();
                oldest_selection.end = oldest_selection.head().clone();
            }
            self.update_selections(vec![oldest_selection], true, cx);
        }
    }

    fn select_ranges<I, T>(&mut self, ranges: I, autoscroll: bool, cx: &mut ViewContext<Self>)
    where
        I: IntoIterator<Item = Range<T>>,
        T: ToOffset,
    {
        let buffer = self.buffer.read(cx);
        let selections = ranges
            .into_iter()
            .map(|range| {
                let mut start = range.start.to_offset(buffer);
                let mut end = range.end.to_offset(buffer);
                let reversed = if start > end {
                    mem::swap(&mut start, &mut end);
                    true
                } else {
                    false
                };
                Selection {
                    id: post_inc(&mut self.next_selection_id),
                    start: start,
                    end: end,
                    reversed,
                    goal: SelectionGoal::None,
                }
            })
            .collect();
        self.update_selections(selections, autoscroll, cx);
    }

    #[cfg(test)]
    fn select_display_ranges<'a, T>(
        &mut self,
        ranges: T,
        cx: &mut ViewContext<Self>,
    ) -> anyhow::Result<()>
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
        self.update_selections(selections, false, cx);
        Ok(())
    }

    pub fn handle_input(&mut self, action: &Input, cx: &mut ViewContext<Self>) {
        let text = action.0.as_ref();
        if !self.skip_autoclose_end(text, cx) {
            self.start_transaction(cx);
            self.insert(text, cx);
            self.autoclose_pairs(cx);
            self.end_transaction(cx);
        }
    }

    pub fn newline(&mut self, _: &Newline, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut old_selections = SmallVec::<[_; 32]>::new();
        {
            let selections = self.selections::<Point>(cx).collect::<Vec<_>>();
            let buffer = self.buffer.read(cx);
            for selection in selections.iter() {
                let start_point = selection.start;
                let indent = buffer
                    .indent_column_for_line(start_point.row)
                    .min(start_point.column);
                let start = selection.start.to_offset(buffer);
                let end = selection.end.to_offset(buffer);

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

                old_selections.push((selection.id, start..end, indent, insert_extra_newline));
            }
        }

        let mut new_selections = Vec::with_capacity(old_selections.len());
        self.buffer.update(cx, |buffer, cx| {
            let mut delta = 0_isize;
            let mut pending_edit: Option<PendingEdit> = None;
            for (_, range, indent, insert_extra_newline) in &old_selections {
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

            let mut delta = 0_isize;
            new_selections.extend(old_selections.into_iter().map(
                |(id, range, indent, insert_extra_newline)| {
                    let start = (range.start as isize + delta) as usize;
                    let end = (range.end as isize + delta) as usize;
                    let text_before_cursor_len = indent as usize + 1;
                    let cursor = start + text_before_cursor_len;
                    let text_len = if insert_extra_newline {
                        text_before_cursor_len * 2
                    } else {
                        text_before_cursor_len
                    };
                    delta += text_len as isize - (end - start) as isize;
                    Selection {
                        id,
                        start: cursor,
                        end: cursor,
                        reversed: false,
                        goal: SelectionGoal::None,
                    }
                },
            ))
        });

        self.update_selections(new_selections, true, cx);
        self.end_transaction(cx);

        #[derive(Default)]
        struct PendingEdit {
            indent: u32,
            insert_extra_newline: bool,
            delta: isize,
            ranges: SmallVec<[Range<usize>; 32]>,
        }
    }

    fn insert(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let old_selections = self.selections::<usize>(cx).collect::<SmallVec<[_; 32]>>();
        let mut new_selections = Vec::new();
        self.buffer.update(cx, |buffer, cx| {
            let edit_ranges = old_selections.iter().map(|s| s.start..s.end);
            buffer.edit_with_autoindent(edit_ranges, text, cx);
            let text_len = text.len() as isize;
            let mut delta = 0_isize;
            new_selections = old_selections
                .into_iter()
                .map(|selection| {
                    let start = selection.start as isize;
                    let end = selection.end as isize;
                    let cursor = (start + delta + text_len) as usize;
                    let deleted_count = end - start;
                    delta += text_len - deleted_count;
                    Selection {
                        id: selection.id,
                        start: cursor,
                        end: cursor,
                        reversed: false,
                        goal: SelectionGoal::None,
                    }
                })
                .collect();
        });

        self.update_selections(new_selections, true, cx);
        self.end_transaction(cx);
    }

    fn autoclose_pairs(&mut self, cx: &mut ViewContext<Self>) {
        let selections = self.selections::<usize>(cx).collect::<Vec<_>>();
        let new_autoclose_pair_state = self.buffer.update(cx, |buffer, cx| {
            let autoclose_pair = buffer.language().and_then(|language| {
                let first_selection_start = selections.first().unwrap().start;
                let pair = language.brackets().iter().find(|pair| {
                    buffer.contains_str_at(
                        first_selection_start.saturating_sub(pair.start.len()),
                        &pair.start,
                    )
                });
                pair.and_then(|pair| {
                    let should_autoclose = selections[1..].iter().all(|selection| {
                        buffer.contains_str_at(
                            selection.start.saturating_sub(pair.start.len()),
                            &pair.start,
                        )
                    });

                    if should_autoclose {
                        Some(pair.clone())
                    } else {
                        None
                    }
                })
            });

            autoclose_pair.and_then(|pair| {
                let selection_ranges = selections
                    .iter()
                    .map(|selection| {
                        let start = selection.start.to_offset(&*buffer);
                        start..start
                    })
                    .collect::<SmallVec<[_; 32]>>();

                buffer.edit(selection_ranges, &pair.end, cx);

                if pair.end.len() == 1 {
                    let mut delta = 0;
                    Some(BracketPairState {
                        ranges: buffer.anchor_range_set(
                            Bias::Left,
                            Bias::Right,
                            selections.iter().map(move |selection| {
                                let offset = selection.start + delta;
                                delta += 1;
                                offset..offset
                            }),
                        ),
                        pair,
                    })
                } else {
                    None
                }
            })
        });
        self.autoclose_stack.extend(new_autoclose_pair_state);
    }

    fn skip_autoclose_end(&mut self, text: &str, cx: &mut ViewContext<Self>) -> bool {
        let old_selections = self.selections::<usize>(cx).collect::<Vec<_>>();
        let autoclose_pair_state = if let Some(autoclose_pair_state) = self.autoclose_stack.last() {
            autoclose_pair_state
        } else {
            return false;
        };
        if text != autoclose_pair_state.pair.end {
            return false;
        }

        debug_assert_eq!(old_selections.len(), autoclose_pair_state.ranges.len());

        let buffer = self.buffer.read(cx);
        if old_selections
            .iter()
            .zip(autoclose_pair_state.ranges.ranges::<usize, _>(buffer))
            .all(|(selection, autoclose_range)| {
                let autoclose_range_end = autoclose_range.end.to_offset(buffer);
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
            self.update_selections(new_selections, true, cx);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_all(&SelectAll, cx);
        self.insert("", cx);
        self.end_transaction(cx);
    }

    pub fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
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
        self.update_selections(selections, true, cx);
        self.insert("", cx);
        self.end_transaction(cx);
    }

    pub fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
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
        self.update_selections(selections, true, cx);
        self.insert(&"", cx);
        self.end_transaction(cx);
    }

    pub fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let tab_size = self.build_settings.borrow()(cx).tab_size;
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        self.buffer.update(cx, |buffer, cx| {
            let mut last_indented_row = None;
            for selection in &mut selections {
                if selection.is_empty() {
                    let char_column = buffer
                        .chars_for_range(Point::new(selection.start.row, 0)..selection.start)
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
                    for row in selection.start.row..=selection.end.row {
                        if last_indented_row != Some(row) {
                            let char_column = buffer.indent_column_for_line(row) as usize;
                            let chars_to_next_tab_stop = tab_size - (char_column % tab_size);
                            let row_start = Point::new(row, 0);
                            buffer.edit(
                                [row_start..row_start],
                                " ".repeat(chars_to_next_tab_stop),
                                cx,
                            );
                            last_indented_row = Some(row);
                        }
                    }
                }
            }
        });

        self.update_selections(selections, true, cx);
        self.end_transaction(cx);
    }

    pub fn delete_line(&mut self, _: &DeleteLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);

        let mut row_delta = 0;
        let mut new_cursors = Vec::new();
        let mut edit_ranges = Vec::new();
        let mut selections = selections.iter().peekable();
        while let Some(selection) = selections.next() {
            let mut rows = selection.spanned_rows(false, &display_map).buffer_rows;
            let goal_display_column = selection.head().to_display_point(&display_map).column();

            // Accumulate contiguous regions of rows that we want to delete.
            while let Some(next_selection) = selections.peek() {
                let next_rows = next_selection.spanned_rows(false, &display_map).buffer_rows;
                if next_rows.start <= rows.end {
                    rows.end = next_rows.end;
                    selections.next().unwrap();
                } else {
                    break;
                }
            }

            let mut edit_start = Point::new(rows.start, 0).to_offset(buffer);
            let edit_end;
            let cursor_buffer_row;
            if buffer.max_point().row >= rows.end {
                // If there's a line after the range, delete the \n from the end of the row range
                // and position the cursor on the next line.
                edit_end = Point::new(rows.end, 0).to_offset(buffer);
                cursor_buffer_row = rows.start;
            } else {
                // If there isn't a line after the range, delete the \n from the line before the
                // start of the row range and position the cursor there.
                edit_start = edit_start.saturating_sub(1);
                edit_end = buffer.len();
                cursor_buffer_row = rows.start.saturating_sub(1);
            }

            let mut cursor =
                Point::new(cursor_buffer_row - row_delta, 0).to_display_point(&display_map);
            *cursor.column_mut() =
                cmp::min(goal_display_column, display_map.line_len(cursor.row()));
            row_delta += rows.len() as u32;

            new_cursors.push((selection.id, cursor.to_point(&display_map)));
            edit_ranges.push(edit_start..edit_end);
        }

        new_cursors.sort_unstable_by_key(|(_, point)| point.clone());
        let new_selections = new_cursors
            .into_iter()
            .map(|(id, cursor)| Selection {
                id,
                start: cursor,
                end: cursor,
                reversed: false,
                goal: SelectionGoal::None,
            })
            .collect();
        self.buffer
            .update(cx, |buffer, cx| buffer.edit(edit_ranges, "", cx));
        self.update_selections(new_selections, true, cx);
        self.end_transaction(cx);
    }

    pub fn duplicate_line(&mut self, _: &DuplicateLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);

        let mut edits = Vec::new();
        let mut selections_iter = selections.iter().peekable();
        while let Some(selection) = selections_iter.next() {
            // Avoid duplicating the same lines twice.
            let mut rows = selection.spanned_rows(false, &display_map).buffer_rows;

            while let Some(next_selection) = selections_iter.peek() {
                let next_rows = next_selection.spanned_rows(false, &display_map).buffer_rows;
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

        let mut edits_iter = edits.iter().peekable();
        let mut row_delta = 0;
        for selection in selections.iter_mut() {
            while let Some((point, _, line_count)) = edits_iter.peek() {
                if *point <= selection.start {
                    row_delta += line_count;
                    edits_iter.next();
                } else {
                    break;
                }
            }
            selection.start.row += row_delta;
            selection.end.row += row_delta;
        }

        self.buffer.update(cx, |buffer, cx| {
            for (point, text, _) in edits.into_iter().rev() {
                buffer.edit(Some(point..point), text, cx);
            }
        });

        self.update_selections(selections, true, cx);
        self.end_transaction(cx);
    }

    pub fn move_line_up(&mut self, _: &MoveLineUp, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);

        let mut edits = Vec::new();
        let mut new_selection_ranges = Vec::new();
        let mut old_folds = Vec::new();
        let mut new_folds = Vec::new();

        let mut selections = selections.iter().peekable();
        let mut contiguous_selections = Vec::new();
        while let Some(selection) = selections.next() {
            // Accumulate contiguous regions of rows that we want to move.
            contiguous_selections.push(selection.point_range(buffer));
            let SpannedRows {
                mut buffer_rows,
                mut display_rows,
            } = selection.spanned_rows(false, &display_map);

            while let Some(next_selection) = selections.peek() {
                let SpannedRows {
                    buffer_rows: next_buffer_rows,
                    display_rows: next_display_rows,
                } = next_selection.spanned_rows(false, &display_map);
                if next_buffer_rows.start <= buffer_rows.end {
                    buffer_rows.end = next_buffer_rows.end;
                    display_rows.end = next_display_rows.end;
                    contiguous_selections.push(next_selection.point_range(buffer));
                    selections.next().unwrap();
                } else {
                    break;
                }
            }

            // Cut the text from the selected rows and paste it at the start of the previous line.
            if display_rows.start != 0 {
                let start = Point::new(buffer_rows.start, 0).to_offset(buffer);
                let end = Point::new(buffer_rows.end - 1, buffer.line_len(buffer_rows.end - 1))
                    .to_offset(buffer);

                let prev_row_display_start = DisplayPoint::new(display_rows.start - 1, 0);
                let prev_row_buffer_start = display_map.prev_row_boundary(prev_row_display_start).1;
                let prev_row_buffer_start_offset = prev_row_buffer_start.to_offset(buffer);

                let mut text = String::new();
                text.extend(buffer.text_for_range(start..end));
                text.push('\n');
                edits.push((
                    prev_row_buffer_start_offset..prev_row_buffer_start_offset,
                    text,
                ));
                edits.push((start - 1..end, String::new()));

                let row_delta = buffer_rows.start - prev_row_buffer_start.row;

                // Move selections up.
                for range in &mut contiguous_selections {
                    range.start.row -= row_delta;
                    range.end.row -= row_delta;
                }

                // Move folds up.
                old_folds.push(start..end);
                for fold in display_map.folds_in_range(start..end) {
                    let mut start = fold.start.to_point(buffer);
                    let mut end = fold.end.to_point(buffer);
                    start.row -= row_delta;
                    end.row -= row_delta;
                    new_folds.push(start..end);
                }
            }

            new_selection_ranges.extend(contiguous_selections.drain(..));
        }

        self.unfold_ranges(old_folds, cx);
        self.buffer.update(cx, |buffer, cx| {
            for (range, text) in edits.into_iter().rev() {
                buffer.edit(Some(range), text, cx);
            }
        });
        self.fold_ranges(new_folds, cx);
        self.select_ranges(new_selection_ranges, true, cx);

        self.end_transaction(cx);
    }

    pub fn move_line_down(&mut self, _: &MoveLineDown, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);

        let mut edits = Vec::new();
        let mut new_selection_ranges = Vec::new();
        let mut old_folds = Vec::new();
        let mut new_folds = Vec::new();

        let mut selections = selections.iter().peekable();
        let mut contiguous_selections = Vec::new();
        while let Some(selection) = selections.next() {
            // Accumulate contiguous regions of rows that we want to move.
            contiguous_selections.push(selection.point_range(buffer));
            let SpannedRows {
                mut buffer_rows,
                mut display_rows,
            } = selection.spanned_rows(false, &display_map);
            while let Some(next_selection) = selections.peek() {
                let SpannedRows {
                    buffer_rows: next_buffer_rows,
                    display_rows: next_display_rows,
                } = next_selection.spanned_rows(false, &display_map);
                if next_buffer_rows.start <= buffer_rows.end {
                    buffer_rows.end = next_buffer_rows.end;
                    display_rows.end = next_display_rows.end;
                    contiguous_selections.push(next_selection.point_range(buffer));
                    selections.next().unwrap();
                } else {
                    break;
                }
            }

            // Cut the text from the selected rows and paste it at the end of the next line.
            if display_rows.end <= display_map.max_point().row() {
                let start = Point::new(buffer_rows.start, 0).to_offset(buffer);
                let end = Point::new(buffer_rows.end - 1, buffer.line_len(buffer_rows.end - 1))
                    .to_offset(buffer);

                let next_row_display_end =
                    DisplayPoint::new(display_rows.end, display_map.line_len(display_rows.end));
                let next_row_buffer_end = display_map.next_row_boundary(next_row_display_end).1;
                let next_row_buffer_end_offset = next_row_buffer_end.to_offset(buffer);

                let mut text = String::new();
                text.push('\n');
                text.extend(buffer.text_for_range(start..end));
                edits.push((start..end + 1, String::new()));
                edits.push((next_row_buffer_end_offset..next_row_buffer_end_offset, text));

                let row_delta = next_row_buffer_end.row - buffer_rows.end + 1;

                // Move selections down.
                for range in &mut contiguous_selections {
                    range.start.row += row_delta;
                    range.end.row += row_delta;
                }

                // Move folds down.
                old_folds.push(start..end);
                for fold in display_map.folds_in_range(start..end) {
                    let mut start = fold.start.to_point(buffer);
                    let mut end = fold.end.to_point(buffer);
                    start.row += row_delta;
                    end.row += row_delta;
                    new_folds.push(start..end);
                }
            }

            new_selection_ranges.extend(contiguous_selections.drain(..));
        }

        self.unfold_ranges(old_folds, cx);
        self.buffer.update(cx, |buffer, cx| {
            for (range, text) in edits.into_iter().rev() {
                buffer.edit(Some(range), text, cx);
            }
        });
        self.fold_ranges(new_folds, cx);
        self.select_ranges(new_selection_ranges, true, cx);

        self.end_transaction(cx);
    }

    pub fn cut(&mut self, _: &Cut, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut text = String::new();
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let buffer = self.buffer.read(cx);
            let max_point = buffer.max_point();
            for selection in &mut selections {
                let is_entire_line = selection.is_empty();
                if is_entire_line {
                    selection.start = Point::new(selection.start.row, 0);
                    selection.end = cmp::min(max_point, Point::new(selection.end.row + 1, 0));
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
        self.update_selections(selections, true, cx);
        self.insert("", cx);
        self.end_transaction(cx);

        cx.as_mut()
            .write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        let selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let buffer = self.buffer.read(cx);
        let max_point = buffer.max_point();
        let mut text = String::new();
        let mut clipboard_selections = Vec::with_capacity(selections.len());
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

        cx.as_mut()
            .write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(item) = cx.as_mut().read_from_clipboard() {
            let clipboard_text = item.text();
            if let Some(mut clipboard_selections) = item.metadata::<Vec<ClipboardSelection>>() {
                let mut selections = self.selections::<usize>(cx).collect::<Vec<_>>();
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
                            let column = selection.start.to_point(&*buffer).column as usize;
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
                self.update_selections(selections, true, cx);
            } else {
                self.insert(clipboard_text, cx);
            }
        }
    }

    pub fn undo(&mut self, _: &Undo, cx: &mut ViewContext<Self>) {
        self.buffer.update(cx, |buffer, cx| buffer.undo(cx));
        self.request_autoscroll(cx);
    }

    pub fn redo(&mut self, _: &Redo, cx: &mut ViewContext<Self>) {
        self.buffer.update(cx, |buffer, cx| buffer.redo(cx));
        self.request_autoscroll(cx);
    }

    pub fn move_left(&mut self, _: &MoveLeft, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
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
        self.update_selections(selections, true, cx);
    }

    pub fn select_left(&mut self, _: &SelectLeft, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let cursor = movement::left(&display_map, head)
                .unwrap()
                .to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_right(&mut self, _: &MoveRight, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
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
        self.update_selections(selections, true, cx);
    }

    pub fn select_right(&mut self, _: &SelectRight, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let cursor = movement::right(&display_map, head)
                .unwrap()
                .to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
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
        self.update_selections(selections, true, cx);
    }

    pub fn select_up(&mut self, _: &SelectUp, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let (head, goal) = movement::up(&display_map, head, selection.goal).unwrap();
            let cursor = head.to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = goal;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return;
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
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
        self.update_selections(selections, true, cx);
    }

    pub fn select_down(&mut self, _: &SelectDown, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let (head, goal) = movement::down(&display_map, head, selection.goal).unwrap();
            let cursor = head.to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = goal;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_to_previous_word_boundary(
        &mut self,
        _: &MoveToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::prev_word_boundary(&display_map, head).unwrap();
            let cursor = new_head.to_point(&display_map);
            selection.start = cursor.clone();
            selection.end = cursor;
            selection.reversed = false;
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_previous_word_boundary(
        &mut self,
        _: &SelectToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::prev_word_boundary(&display_map, head).unwrap();
            let cursor = new_head.to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_previous_word_boundary(
        &mut self,
        _: &DeleteToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        self.start_transaction(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            if selection.is_empty() {
                let head = selection.head().to_display_point(&display_map);
                let new_head = movement::prev_word_boundary(&display_map, head).unwrap();
                let cursor = new_head.to_point(&display_map);
                selection.set_head(cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
        self.insert("", cx);
        self.end_transaction(cx);
    }

    pub fn move_to_next_word_boundary(
        &mut self,
        _: &MoveToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::next_word_boundary(&display_map, head).unwrap();
            let cursor = new_head.to_point(&display_map);
            selection.start = cursor;
            selection.end = cursor;
            selection.reversed = false;
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_next_word_boundary(
        &mut self,
        _: &SelectToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::next_word_boundary(&display_map, head).unwrap();
            let cursor = new_head.to_point(&display_map);
            selection.set_head(cursor);
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_next_word_boundary(
        &mut self,
        _: &DeleteToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        self.start_transaction(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            if selection.is_empty() {
                let head = selection.head().to_display_point(&display_map);
                let new_head = movement::next_word_boundary(&display_map, head).unwrap();
                let cursor = new_head.to_point(&display_map);
                selection.set_head(cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
        self.insert("", cx);
        self.end_transaction(cx);
    }

    pub fn move_to_beginning_of_line(
        &mut self,
        _: &MoveToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::line_beginning(&display_map, head, true).unwrap();
            let cursor = new_head.to_point(&display_map);
            selection.start = cursor;
            selection.end = cursor;
            selection.reversed = false;
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_beginning_of_line(
        &mut self,
        SelectToBeginningOfLine(toggle_indent): &SelectToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::line_beginning(&display_map, head, *toggle_indent).unwrap();
            selection.set_head(new_head.to_point(&display_map));
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
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
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map);
                let new_head = movement::line_end(&display_map, head).unwrap();
                let anchor = new_head.to_point(&display_map);
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_end_of_line(&mut self, _: &SelectToEndOfLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        for selection in &mut selections {
            let head = selection.head().to_display_point(&display_map);
            let new_head = movement::line_end(&display_map, head).unwrap();
            selection.set_head(new_head.to_point(&display_map));
            selection.goal = SelectionGoal::None;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_end_of_line(&mut self, _: &DeleteToEndOfLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_end_of_line(&SelectToEndOfLine, cx);
        self.delete(&Delete, cx);
        self.end_transaction(cx);
    }

    pub fn cut_to_end_of_line(&mut self, _: &CutToEndOfLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_end_of_line(&SelectToEndOfLine, cx);
        self.cut(&Cut, cx);
        self.end_transaction(cx);
    }

    pub fn move_to_beginning(&mut self, _: &MoveToBeginning, cx: &mut ViewContext<Self>) {
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: 0,
            end: 0,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], true, cx);
    }

    pub fn select_to_beginning(&mut self, _: &SelectToBeginning, cx: &mut ViewContext<Self>) {
        let mut selection = self.selections::<Point>(cx).last().unwrap().clone();
        selection.set_head(Point::zero());
        self.update_selections(vec![selection], true, cx);
    }

    pub fn move_to_end(&mut self, _: &MoveToEnd, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        let cursor = buffer.len();
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor,
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], true, cx);
    }

    pub fn select_to_end(&mut self, _: &SelectToEnd, cx: &mut ViewContext<Self>) {
        let mut selection = self.selections::<usize>(cx).last().unwrap().clone();
        selection.set_head(self.buffer.read(cx).len());
        self.update_selections(vec![selection], true, cx);
    }

    pub fn select_all(&mut self, _: &SelectAll, cx: &mut ViewContext<Self>) {
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: 0,
            end: self.buffer.read(cx).len(),
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], false, cx);
    }

    pub fn select_line(&mut self, _: &SelectLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let buffer = self.buffer.read(cx);
        let max_point = buffer.max_point();
        for selection in &mut selections {
            let rows = selection.spanned_rows(true, &display_map).buffer_rows;
            selection.start = Point::new(rows.start, 0);
            selection.end = cmp::min(max_point, Point::new(rows.end, 0));
            selection.reversed = false;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn split_selection_into_lines(
        &mut self,
        _: &SplitSelectionIntoLines,
        cx: &mut ViewContext<Self>,
    ) {
        let selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let buffer = self.buffer.read(cx);

        let mut to_unfold = Vec::new();
        let mut new_selections = Vec::new();
        for selection in selections.iter() {
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
        self.unfold_ranges(to_unfold, cx);
        self.update_selections(new_selections, true, cx);
    }

    pub fn add_selection_above(&mut self, _: &AddSelectionAbove, cx: &mut ViewContext<Self>) {
        self.add_selection(true, cx);
    }

    pub fn add_selection_below(&mut self, _: &AddSelectionBelow, cx: &mut ViewContext<Self>) {
        self.add_selection(false, cx);
    }

    fn add_selection(&mut self, above: bool, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections::<Point>(cx).collect::<Vec<_>>();
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

        self.update_selections(new_selections, true, cx);
        if state.stack.len() > 1 {
            self.add_selections_state = Some(state);
        }
    }

    pub fn select_larger_syntax_node(
        &mut self,
        _: &SelectLargerSyntaxNode,
        cx: &mut ViewContext<Self>,
    ) {
        let old_selections = self.selections::<usize>(cx).collect::<Box<_>>();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);

        let mut stack = mem::take(&mut self.select_larger_syntax_node_stack);
        let mut selected_larger_node = false;
        let mut new_selections = old_selections
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
            new_selections.sort_unstable_by_key(|selection| selection.start);
            self.update_selections(new_selections, true, cx);
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
            self.update_selections(selections.to_vec(), true, cx);
        }
        self.select_larger_syntax_node_stack = stack;
    }

    pub fn move_to_enclosing_bracket(
        &mut self,
        _: &MoveToEnclosingBracket,
        cx: &mut ViewContext<Self>,
    ) {
        let mut selections = self.selections::<usize>(cx).collect::<Vec<_>>();
        let buffer = self.buffer.read(cx.as_ref());
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

        self.update_selections(selections, true, cx);
    }

    pub fn show_next_diagnostic(&mut self, _: &ShowNextDiagnostic, cx: &mut ViewContext<Self>) {
        let selection = self.newest_selection::<usize>(cx);
        let buffer = self.buffer.read(cx.as_ref());
        let active_primary_range = self.active_diagnostics.as_ref().map(|active_diagnostics| {
            active_diagnostics
                .primary_range
                .to_offset(buffer)
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
                .find_map(|(range, diagnostic)| {
                    if diagnostic.is_primary
                        && !range.is_empty()
                        && Some(range.end) != active_primary_range.as_ref().map(|r| *r.end())
                    {
                        Some((range, diagnostic.group_id))
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
                    true,
                    cx,
                );
                break;
            } else if search_start == 0 {
                break;
            } else {
                // Cycle around to the start of the buffer.
                search_start = 0;
            }
        }
    }

    fn refresh_active_diagnostics(&mut self, cx: &mut ViewContext<Editor>) {
        if let Some(active_diagnostics) = self.active_diagnostics.as_mut() {
            let buffer = self.buffer.read(cx);
            let primary_range_start = active_diagnostics.primary_range.start.to_offset(buffer);
            let is_valid = buffer
                .diagnostics_in_range::<_, usize>(active_diagnostics.primary_range.clone())
                .any(|(range, diagnostic)| {
                    diagnostic.is_primary
                        && !range.is_empty()
                        && range.start == primary_range_start
                        && diagnostic.message == active_diagnostics.primary_message
                });

            if is_valid != active_diagnostics.is_valid {
                active_diagnostics.is_valid = is_valid;
                let mut new_styles = HashMap::new();
                for (block_id, diagnostic) in &active_diagnostics.blocks {
                    let severity = diagnostic.severity;
                    let message_len = diagnostic.message.len();
                    new_styles.insert(
                        *block_id,
                        (
                            Some({
                                let build_settings = self.build_settings.clone();
                                move |cx: &AppContext| {
                                    let settings = build_settings.borrow()(cx);
                                    vec![(
                                        message_len,
                                        diagnostic_style(severity, is_valid, &settings.style)
                                            .text
                                            .into(),
                                    )]
                                }
                            }),
                            Some({
                                let build_settings = self.build_settings.clone();
                                move |cx: &AppContext| {
                                    let settings = build_settings.borrow()(cx);
                                    diagnostic_style(severity, is_valid, &settings.style).block
                                }
                            }),
                        ),
                    );
                }
                self.display_map
                    .update(cx, |display_map, _| display_map.restyle_blocks(new_styles));
            }
        }
    }

    fn activate_diagnostics(&mut self, group_id: usize, cx: &mut ViewContext<Self>) {
        self.dismiss_diagnostics(cx);
        self.active_diagnostics = self.display_map.update(cx, |display_map, cx| {
            let buffer = self.buffer.read(cx);

            let mut primary_range = None;
            let mut primary_message = None;
            let mut group_end = Point::zero();
            let diagnostic_group = buffer
                .diagnostic_group::<Point>(group_id)
                .map(|(range, diagnostic)| {
                    if range.end > group_end {
                        group_end = range.end;
                    }
                    if diagnostic.is_primary {
                        primary_range = Some(range.clone());
                        primary_message = Some(diagnostic.message.clone());
                    }
                    (range, diagnostic.clone())
                })
                .collect::<Vec<_>>();
            let primary_range = primary_range.unwrap();
            let primary_message = primary_message.unwrap();
            let primary_range =
                buffer.anchor_after(primary_range.start)..buffer.anchor_before(primary_range.end);

            let blocks = display_map
                .insert_blocks(
                    diagnostic_group.iter().map(|(range, diagnostic)| {
                        let build_settings = self.build_settings.clone();
                        let message_len = diagnostic.message.len();
                        let severity = diagnostic.severity;
                        BlockProperties {
                            position: range.start,
                            text: diagnostic.message.as_str(),
                            build_runs: Some(Arc::new({
                                let build_settings = build_settings.clone();
                                move |cx| {
                                    let settings = build_settings.borrow()(cx);
                                    vec![(
                                        message_len,
                                        diagnostic_style(severity, true, &settings.style)
                                            .text
                                            .into(),
                                    )]
                                }
                            })),
                            build_style: Some(Arc::new({
                                let build_settings = build_settings.clone();
                                move |cx| {
                                    let settings = build_settings.borrow()(cx);
                                    diagnostic_style(severity, true, &settings.style).block
                                }
                            })),
                            disposition: BlockDisposition::Below,
                        }
                    }),
                    cx,
                )
                .into_iter()
                .zip(
                    diagnostic_group
                        .into_iter()
                        .map(|(_, diagnostic)| diagnostic),
                )
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
        display_map: &DisplayMapSnapshot,
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

    pub fn active_selection_sets<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = SelectionSetId> {
        let buffer = self.buffer.read(cx);
        let replica_id = buffer.replica_id();
        buffer
            .selection_sets()
            .filter(move |(set_id, set)| {
                set.active && (set_id.replica_id != replica_id || **set_id == self.selection_set_id)
            })
            .map(|(set_id, _)| *set_id)
    }

    pub fn selections_in_range<'a>(
        &'a self,
        set_id: SelectionSetId,
        range: Range<DisplayPoint>,
        cx: &'a mut MutableAppContext,
    ) -> impl 'a + Iterator<Item = Range<DisplayPoint>> {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);
        let selections = self
            .buffer
            .read(cx)
            .selection_set(set_id)
            .unwrap()
            .selections::<Point, _>(buffer)
            .collect::<Vec<_>>();
        let start = range.start.to_point(&display_map);
        let start_index = self.selection_insertion_index(&selections, start);
        let pending_selection = if set_id == self.selection_set_id {
            self.pending_selection.as_ref().and_then(|pending| {
                let mut selection_start = pending.start.to_display_point(&display_map);
                let mut selection_end = pending.end.to_display_point(&display_map);
                if pending.reversed {
                    mem::swap(&mut selection_start, &mut selection_end);
                }
                if selection_start <= range.end || selection_end <= range.end {
                    Some(selection_start..selection_end)
                } else {
                    None
                }
            })
        } else {
            None
        };
        selections
            .into_iter()
            .skip(start_index)
            .map(move |s| s.display_range(&display_map))
            .take_while(move |r| r.start <= range.end || r.end <= range.end)
            .chain(pending_selection)
    }

    fn selection_insertion_index(&self, selections: &[Selection<Point>], start: Point) -> usize {
        match selections.binary_search_by_key(&start, |probe| probe.start) {
            Ok(index) => index,
            Err(index) => {
                if index > 0 && selections[index - 1].end > start {
                    index - 1
                } else {
                    index
                }
            }
        }
    }

    pub fn selections<'a, D>(&self, cx: &'a AppContext) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: 'a + TextDimension<'a> + Ord,
    {
        let buffer = self.buffer.read(cx);
        let mut selections = self.selection_set(cx).selections::<D, _>(buffer).peekable();
        let mut pending_selection = self.pending_selection(cx);
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
    }

    fn pending_selection<'a, D>(&self, cx: &'a AppContext) -> Option<Selection<D>>
    where
        D: 'a + TextDimension<'a>,
    {
        let buffer = self.buffer.read(cx);
        self.pending_selection.as_ref().map(|selection| Selection {
            id: selection.id,
            start: selection.start.summary::<D, _>(buffer),
            end: selection.end.summary::<D, _>(buffer),
            reversed: selection.reversed,
            goal: selection.goal,
        })
    }

    fn selection_count<'a>(&self, cx: &'a AppContext) -> usize {
        let mut selection_count = self.selection_set(cx).len();
        if self.pending_selection.is_some() {
            selection_count += 1;
        }
        selection_count
    }

    pub fn oldest_selection<'a, T>(&self, cx: &'a AppContext) -> Selection<T>
    where
        T: 'a + TextDimension<'a>,
    {
        let buffer = self.buffer.read(cx);
        self.selection_set(cx)
            .oldest_selection(buffer)
            .or_else(|| self.pending_selection(cx))
            .unwrap()
    }

    pub fn newest_selection<'a, T>(&self, cx: &'a AppContext) -> Selection<T>
    where
        T: 'a + TextDimension<'a>,
    {
        let buffer = self.buffer.read(cx);
        self.pending_selection(cx)
            .or_else(|| self.selection_set(cx).newest_selection(buffer))
            .unwrap()
    }

    fn selection_set<'a>(&self, cx: &'a AppContext) -> &'a SelectionSet {
        self.buffer
            .read(cx)
            .selection_set(self.selection_set_id)
            .unwrap()
    }

    fn update_selections<T>(
        &mut self,
        mut selections: Vec<Selection<T>>,
        autoscroll: bool,
        cx: &mut ViewContext<Self>,
    ) where
        T: ToOffset + ToPoint + Ord + std::marker::Copy + std::fmt::Debug,
    {
        // Merge overlapping selections.
        let buffer = self.buffer.read(cx);
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

        self.pending_selection = None;
        self.add_selections_state = None;
        self.select_larger_syntax_node_stack.clear();
        while let Some(autoclose_pair_state) = self.autoclose_stack.last() {
            let all_selections_inside_autoclose_ranges =
                if selections.len() == autoclose_pair_state.ranges.len() {
                    selections
                        .iter()
                        .zip(autoclose_pair_state.ranges.ranges::<Point, _>(buffer))
                        .all(|(selection, autoclose_range)| {
                            let head = selection.head().to_point(&*buffer);
                            autoclose_range.start <= head && autoclose_range.end >= head
                        })
                } else {
                    false
                };

            if all_selections_inside_autoclose_ranges {
                break;
            } else {
                self.autoclose_stack.pop();
            }
        }

        if autoscroll {
            self.request_autoscroll(cx);
        }
        self.pause_cursor_blinking(cx);

        self.buffer.update(cx, |buffer, cx| {
            buffer
                .update_selection_set(self.selection_set_id, &selections, cx)
                .unwrap();
        });
    }

    fn request_autoscroll(&mut self, cx: &mut ViewContext<Self>) {
        self.autoscroll_requested = true;
        cx.notify();
    }

    fn start_transaction(&mut self, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        self.buffer.update(cx, |buffer, _| {
            buffer
                .start_transaction(Some(self.selection_set_id))
                .unwrap()
        });
    }

    fn end_transaction(&self, cx: &mut ViewContext<Self>) {
        self.buffer.update(cx, |buffer, cx| {
            buffer
                .end_transaction(Some(self.selection_set_id), cx)
                .unwrap()
        });
    }

    pub fn page_up(&mut self, _: &PageUp, _: &mut ViewContext<Self>) {
        log::info!("Editor::page_up");
    }

    pub fn page_down(&mut self, _: &PageDown, _: &mut ViewContext<Self>) {
        log::info!("Editor::page_down");
    }

    pub fn fold(&mut self, _: &Fold, cx: &mut ViewContext<Self>) {
        let mut fold_ranges = Vec::new();

        let selections = self.selections::<Point>(cx).collect::<Vec<_>>();
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
        let selections = self.selections::<Point>(cx).collect::<Vec<_>>();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);
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

    fn is_line_foldable(&self, display_map: &DisplayMapSnapshot, display_row: u32) -> bool {
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
        display_map: &DisplayMapSnapshot,
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
        let selections = self.selections::<Point>(cx);
        let ranges = selections.map(|s| s.start..s.end).collect();
        self.fold_ranges(ranges, cx);
    }

    fn fold_ranges<T: ToOffset>(&mut self, ranges: Vec<Range<T>>, cx: &mut ViewContext<Self>) {
        if !ranges.is_empty() {
            self.display_map.update(cx, |map, cx| map.fold(ranges, cx));
            self.autoscroll_requested = true;
            cx.notify();
        }
    }

    fn unfold_ranges<T: ToOffset>(&mut self, ranges: Vec<Range<T>>, cx: &mut ViewContext<Self>) {
        if !ranges.is_empty() {
            self.display_map
                .update(cx, |map, cx| map.unfold(ranges, cx));
            self.autoscroll_requested = true;
            cx.notify();
        }
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
        self.buffer.read(cx).text()
    }

    pub fn display_text(&self, cx: &mut MutableAppContext) -> String {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .text()
    }

    // pub fn font_size(&self) -> f32 {
    //     self.settings.font_size
    // }

    pub fn set_wrap_width(&self, width: f32, cx: &mut MutableAppContext) -> bool {
        self.display_map
            .update(cx, |map, cx| map.set_wrap_width(Some(width), cx))
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    fn pause_cursor_blinking(&mut self, cx: &mut ViewContext<Self>) {
        self.show_local_cursors = true;
        cx.notify();

        let epoch = self.next_blink_epoch();
        cx.spawn(|this, mut cx| {
            let this = this.downgrade();
            async move {
                Timer::after(CURSOR_BLINK_INTERVAL).await;
                if let Some(this) = cx.read(|cx| this.upgrade(cx)) {
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
                    if let Some(this) = cx.read(|cx| this.upgrade(cx)) {
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

    fn on_buffer_changed(&mut self, _: ModelHandle<Buffer>, cx: &mut ViewContext<Self>) {
        self.refresh_active_diagnostics(cx);
        cx.notify();
    }

    fn on_buffer_event(
        &mut self,
        _: ModelHandle<Buffer>,
        event: &language::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            language::Event::Edited => cx.emit(Event::Edited),
            language::Event::Dirtied => cx.emit(Event::Dirtied),
            language::Event::Saved => cx.emit(Event::Saved),
            language::Event::FileHandleChanged => cx.emit(Event::FileHandleChanged),
            language::Event::Reloaded => cx.emit(Event::FileHandleChanged),
            language::Event::Closed => cx.emit(Event::Closed),
            language::Event::Reparsed => {}
        }
    }

    fn on_display_map_changed(&mut self, _: ModelHandle<DisplayMap>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}

impl Snapshot {
    pub fn is_empty(&self) -> bool {
        self.display_snapshot.is_empty()
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn placeholder_text(&self) -> Option<&Arc<str>> {
        self.placeholder_text.as_ref()
    }

    pub fn buffer_row_count(&self) -> u32 {
        self.display_snapshot.buffer_row_count()
    }

    pub fn buffer_rows<'a>(&'a self, start_row: u32, cx: &'a AppContext) -> BufferRows<'a> {
        self.display_snapshot.buffer_rows(start_row, Some(cx))
    }

    pub fn chunks<'a>(
        &'a self,
        display_rows: Range<u32>,
        theme: Option<&'a SyntaxTheme>,
        cx: &'a AppContext,
    ) -> display_map::Chunks<'a> {
        self.display_snapshot.chunks(display_rows, theme, cx)
    }

    pub fn scroll_position(&self) -> Vector2F {
        compute_scroll_position(
            &self.display_snapshot,
            self.scroll_position,
            &self.scroll_top_anchor,
        )
    }

    pub fn max_point(&self) -> DisplayPoint {
        self.display_snapshot.max_point()
    }

    pub fn longest_row(&self) -> u32 {
        self.display_snapshot.longest_row()
    }

    pub fn line_len(&self, display_row: u32) -> u32 {
        self.display_snapshot.line_len(display_row)
    }

    pub fn line(&self, display_row: u32) -> String {
        self.display_snapshot.line(display_row)
    }

    pub fn prev_row_boundary(&self, point: DisplayPoint) -> (DisplayPoint, Point) {
        self.display_snapshot.prev_row_boundary(point)
    }

    pub fn next_row_boundary(&self, point: DisplayPoint) -> (DisplayPoint, Point) {
        self.display_snapshot.next_row_boundary(point)
    }
}

impl EditorSettings {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &AppContext) -> Self {
        Self {
            tab_size: 4,
            style: {
                let font_cache: &gpui::FontCache = cx.font_cache();
                let font_family_name = Arc::from("Monaco");
                let font_properties = Default::default();
                let font_family_id = font_cache.load_family(&[&font_family_name]).unwrap();
                let font_id = font_cache
                    .select_font(font_family_id, &font_properties)
                    .unwrap();
                EditorStyle {
                    text: gpui::fonts::TextStyle {
                        font_family_name,
                        font_family_id,
                        font_id,
                        font_size: 14.,
                        color: gpui::color::Color::from_u32(0xff0000ff),
                        font_properties,
                        underline: None,
                    },
                    placeholder_text: None,
                    background: Default::default(),
                    gutter_background: Default::default(),
                    active_line_background: Default::default(),
                    line_number: Default::default(),
                    line_number_active: Default::default(),
                    selection: Default::default(),
                    guest_selections: Default::default(),
                    syntax: Default::default(),
                    error_diagnostic: Default::default(),
                    invalid_error_diagnostic: Default::default(),
                    warning_diagnostic: Default::default(),
                    invalid_warning_diagnostic: Default::default(),
                    information_diagnostic: Default::default(),
                    invalid_information_diagnostic: Default::default(),
                    hint_diagnostic: Default::default(),
                    invalid_hint_diagnostic: Default::default(),
                }
            },
        }
    }
}

fn compute_scroll_position(
    snapshot: &DisplayMapSnapshot,
    mut scroll_position: Vector2F,
    scroll_top_anchor: &Anchor,
) -> Vector2F {
    let scroll_top = scroll_top_anchor.to_display_point(snapshot).row() as f32;
    scroll_position.set_y(scroll_top + scroll_position.y());
    scroll_position
}

pub enum Event {
    Activate,
    Edited,
    Blurred,
    Dirtied,
    Saved,
    FileHandleChanged,
    Closed,
}

impl Entity for Editor {
    type Event = Event;

    fn release(&mut self, cx: &mut MutableAppContext) {
        self.buffer.update(cx, |buffer, cx| {
            buffer
                .remove_selection_set(self.selection_set_id, cx)
                .unwrap();
        });
    }
}

impl View for Editor {
    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let settings = self.build_settings.borrow_mut()(cx);
        self.display_map.update(cx, |map, cx| {
            map.set_font(
                settings.style.text.font_id,
                settings.style.text.font_size,
                cx,
            )
        });
        EditorElement::new(self.handle.clone(), settings).boxed()
    }

    fn ui_name() -> &'static str {
        "Editor"
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.focused = true;
        self.blink_cursors(self.blink_epoch, cx);
        self.buffer.update(cx, |buffer, cx| {
            buffer
                .set_active_selection_set(Some(self.selection_set_id), cx)
                .unwrap();
        });
    }

    fn on_blur(&mut self, cx: &mut ViewContext<Self>) {
        self.focused = false;
        self.show_local_cursors = false;
        self.buffer.update(cx, |buffer, cx| {
            buffer.set_active_selection_set(None, cx).unwrap();
        });
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
        cx
    }
}

impl SelectionExt for Selection<Point> {
    fn display_range(&self, map: &DisplayMapSnapshot) -> Range<DisplayPoint> {
        let start = self.start.to_display_point(map);
        let end = self.end.to_display_point(map);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    fn spanned_rows(
        &self,
        include_end_if_at_line_start: bool,
        map: &DisplayMapSnapshot,
    ) -> SpannedRows {
        let display_start = self.start.to_display_point(map);
        let mut display_end = self.end.to_display_point(map);
        if !include_end_if_at_line_start
            && display_end.row() != map.max_point().row()
            && display_start.row() != display_end.row()
            && display_end.column() == 0
        {
            *display_end.row_mut() -= 1;
        }

        let (display_start, buffer_start) = map.prev_row_boundary(display_start);
        let (display_end, buffer_end) = map.next_row_boundary(display_end);

        SpannedRows {
            buffer_rows: buffer_start.row..buffer_end.row + 1,
            display_rows: display_start.row()..display_end.row() + 1,
        }
    }
}

pub fn diagnostic_style(
    severity: DiagnosticSeverity,
    valid: bool,
    style: &EditorStyle,
) -> DiagnosticStyle {
    match (severity, valid) {
        (DiagnosticSeverity::ERROR, true) => style.error_diagnostic,
        (DiagnosticSeverity::ERROR, false) => style.invalid_error_diagnostic,
        (DiagnosticSeverity::WARNING, true) => style.warning_diagnostic,
        (DiagnosticSeverity::WARNING, false) => style.invalid_warning_diagnostic,
        (DiagnosticSeverity::INFORMATION, true) => style.information_diagnostic,
        (DiagnosticSeverity::INFORMATION, false) => style.invalid_information_diagnostic,
        (DiagnosticSeverity::HINT, true) => style.hint_diagnostic,
        (DiagnosticSeverity::HINT, false) => style.invalid_hint_diagnostic,
        _ => Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::sample_text;
    use buffer::Point;
    use unindent::Unindent;

    #[gpui::test]
    fn test_selection_with_mouse(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx));
        let settings = EditorSettings::test(cx);
        let (_, editor) =
            cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        editor.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(2, 2), false, cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selection_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
        );

        editor.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selection_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );

        editor.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(1, 1), Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selection_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
        );

        editor.update(cx, |view, cx| {
            view.end_selection(cx);
            view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selection_ranges(cx)),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
        );

        editor.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(3, 3), true, cx);
            view.update_selection(DisplayPoint::new(0, 0), Vector2F::zero(), cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selection_ranges(cx)),
            [
                DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)
            ]
        );

        editor.update(cx, |view, cx| {
            view.end_selection(cx);
        });

        assert_eq!(
            editor.update(cx, |view, cx| view.selection_ranges(cx)),
            [DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)]
        );
    }

    #[gpui::test]
    fn test_canceling_pending_selection(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx));
        let settings = EditorSettings::test(cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(2, 2), false, cx);
            assert_eq!(
                view.selection_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
            );
        });

        view.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), cx);
            assert_eq!(
                view.selection_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            view.update_selection(DisplayPoint::new(1, 1), Vector2F::zero(), cx);
            assert_eq!(
                view.selection_ranges(cx),
                [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
            );
        });
    }

    #[gpui::test]
    fn test_cancel(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx));
        let settings = EditorSettings::test(cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(3, 4), false, cx);
            view.update_selection(DisplayPoint::new(1, 1), Vector2F::zero(), cx);
            view.end_selection(cx);

            view.begin_selection(DisplayPoint::new(0, 1), true, cx);
            view.update_selection(DisplayPoint::new(0, 3), Vector2F::zero(), cx);
            view.end_selection(cx);
            assert_eq!(
                view.selection_ranges(cx),
                [
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            assert_eq!(
                view.selection_ranges(cx),
                [DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1)]
            );
        });

        view.update(cx, |view, cx| {
            view.cancel(&Cancel, cx);
            assert_eq!(
                view.selection_ranges(cx),
                [DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1)]
            );
        });
    }

    #[gpui::test]
    fn test_fold(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| {
            Buffer::new(
                0,
                "
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
            )
        });
        let settings = EditorSettings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(8, 0)..DisplayPoint::new(12, 0)], cx)
                .unwrap();
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
            assert_eq!(view.display_text(cx), buffer.read(cx).text());
        });
    }

    #[gpui::test]
    fn test_move_cursor(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6), cx));
        let settings = EditorSettings::test(&cx);
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
                view.selection_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.move_down(&MoveDown, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );

            view.move_right(&MoveRight, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4)]
            );

            view.move_left(&MoveLeft, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );

            view.move_up(&MoveUp, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.move_to_end(&MoveToEnd, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(5, 6)..DisplayPoint::new(5, 6)]
            );

            view.move_to_beginning(&MoveToBeginning, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2)], cx)
                .unwrap();
            view.select_to_beginning(&SelectToBeginning, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 0)]
            );

            view.select_to_end(&SelectToEnd, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(0, 1)..DisplayPoint::new(5, 6)]
            );
        });
    }

    #[gpui::test]
    fn test_move_cursor_multibyte(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "ⓐⓑⓒⓓⓔ\nabcde\nαβγδε\n", cx));
        let settings = EditorSettings::test(&cx);
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
            assert_eq!(view.selection_ranges(cx), &[empty_range(0, "ⓐ".len())]);
            view.move_right(&MoveRight, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(0, "ⓐⓑ".len())]);
            view.move_right(&MoveRight, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(0, "ⓐⓑ…".len())]);

            view.move_down(&MoveDown, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(1, "ab…".len())]);
            view.move_left(&MoveLeft, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(1, "ab".len())]);
            view.move_left(&MoveLeft, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(1, "a".len())]);

            view.move_down(&MoveDown, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(2, "α".len())]);
            view.move_right(&MoveRight, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(2, "αβ".len())]);
            view.move_right(&MoveRight, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(2, "αβ…".len())]);
            view.move_right(&MoveRight, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(2, "αβ…ε".len())]);

            view.move_up(&MoveUp, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(1, "ab…e".len())]);
            view.move_up(&MoveUp, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(0, "ⓐⓑ…ⓔ".len())]);
            view.move_left(&MoveLeft, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(0, "ⓐⓑ…".len())]);
            view.move_left(&MoveLeft, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(0, "ⓐⓑ".len())]);
            view.move_left(&MoveLeft, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(0, "ⓐ".len())]);
        });
    }

    #[gpui::test]
    fn test_move_cursor_different_line_lengths(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "ⓐⓑⓒⓓⓔ\nabcd\nαβγ\nabcd\nⓐⓑⓒⓓⓔ\n", cx));
        let settings = EditorSettings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer.clone(), settings, cx)
        });
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[empty_range(0, "ⓐⓑⓒⓓⓔ".len())], cx)
                .unwrap();

            view.move_down(&MoveDown, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(1, "abcd".len())]);

            view.move_down(&MoveDown, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(2, "αβγ".len())]);

            view.move_down(&MoveDown, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(3, "abcd".len())]);

            view.move_down(&MoveDown, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(4, "ⓐⓑⓒⓓⓔ".len())]);

            view.move_up(&MoveUp, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(3, "abcd".len())]);

            view.move_up(&MoveUp, cx);
            assert_eq!(view.selection_ranges(cx), &[empty_range(2, "αβγ".len())]);
        });
    }

    #[gpui::test]
    fn test_beginning_end_of_line(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\n  def", cx));
        let settings = EditorSettings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
                ],
                cx,
            )
            .unwrap();
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_end_of_line(&MoveToEndOfLine, cx);
            assert_eq!(
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_beginning_of_line(&SelectToBeginningOfLine(true), cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_beginning_of_line(&SelectToBeginningOfLine(true), cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_end_of_line(&SelectToEndOfLine, cx);
            assert_eq!(
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_prev_next_word_boundary(cx: &mut gpui::MutableAppContext) {
        let buffer =
            cx.add_model(|cx| Buffer::new(0, "use std::str::{foo, bar}\n\n  {baz.qux()}", cx));
        let settings = EditorSettings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 11)..DisplayPoint::new(0, 11),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4),
                ],
                cx,
            )
            .unwrap();
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 3),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 4)..DisplayPoint::new(0, 4),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(0, 23)..DisplayPoint::new(0, 23),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(0, 24)..DisplayPoint::new(0, 24),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 3),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_previous_word_boundary(&SelectToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_next_word_boundary(&SelectToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 4)..DisplayPoint::new(2, 3),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_prev_next_word_bounds_with_soft_wrap(cx: &mut gpui::MutableAppContext) {
        let buffer =
            cx.add_model(|cx| Buffer::new(0, "use one::{\n    two::three::four::five\n};", cx));
        let settings = EditorSettings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.set_wrap_width(140., cx);
            assert_eq!(
                view.display_text(cx),
                "use one::{\n    two::three::\n    four::five\n};"
            );

            view.select_display_ranges(&[DisplayPoint::new(1, 7)..DisplayPoint::new(1, 7)], cx)
                .unwrap();

            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(1, 9)..DisplayPoint::new(1, 9)]
            );

            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(1, 14)..DisplayPoint::new(1, 14)]
            );

            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4)]
            );

            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(2, 8)..DisplayPoint::new(2, 8)]
            );

            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4)]
            );

            view.move_to_previous_word_boundary(&MoveToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(1, 14)..DisplayPoint::new(1, 14)]
            );
        });
    }

    #[gpui::test]
    fn test_delete_to_word_boundary(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "one two three four", cx));
        let settings = EditorSettings::test(&cx);
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
            )
            .unwrap();
            view.delete_to_previous_word_boundary(&DeleteToPreviousWordBoundary, cx);
        });

        assert_eq!(buffer.read(cx).text(), "e two te four");

        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    // an empty selection - the following word fragment is deleted
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    // characters selected - they are deleted
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 10),
                ],
                cx,
            )
            .unwrap();
            view.delete_to_next_word_boundary(&DeleteToNextWordBoundary, cx);
        });

        assert_eq!(buffer.read(cx).text(), "e t te our");
    }

    #[gpui::test]
    fn test_newline(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaaa\n    bbbb\n", cx));
        let settings = EditorSettings::test(&cx);
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
            )
            .unwrap();

            view.newline(&Newline, cx);
            assert_eq!(view.text(cx), "aa\naa\n  \n    bb\n    bb\n");
        });
    }

    #[gpui::test]
    fn test_backspace(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| {
            Buffer::new(
                0,
                "one two three\nfour five six\nseven eight nine\nten\n",
                cx,
            )
        });
        let settings = EditorSettings::test(&cx);
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
            )
            .unwrap();
            view.backspace(&Backspace, cx);
        });

        assert_eq!(
            buffer.read(cx).text(),
            "oe two three\nfou five six\nseven ten\n"
        );
    }

    #[gpui::test]
    fn test_delete(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| {
            Buffer::new(
                0,
                "one two three\nfour five six\nseven eight nine\nten\n",
                cx,
            )
        });
        let settings = EditorSettings::test(&cx);
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
            )
            .unwrap();
            view.delete(&Delete, cx);
        });

        assert_eq!(
            buffer.read(cx).text(),
            "on two three\nfou five six\nseven ten\n"
        );
    }

    #[gpui::test]
    fn test_delete_line(cx: &mut gpui::MutableAppContext) {
        let settings = EditorSettings::test(&cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndef\nghi\n", cx));
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                ],
                cx,
            )
            .unwrap();
            view.delete_line(&DeleteLine, cx);
            assert_eq!(view.display_text(cx), "ghi");
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)
                ]
            );
        });

        let settings = EditorSettings::test(&cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndef\nghi\n", cx));
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(2, 0)..DisplayPoint::new(0, 1)], cx)
                .unwrap();
            view.delete_line(&DeleteLine, cx);
            assert_eq!(view.display_text(cx), "ghi\n");
            assert_eq!(
                view.selection_ranges(cx),
                vec![DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)]
            );
        });
    }

    #[gpui::test]
    fn test_duplicate_line(cx: &mut gpui::MutableAppContext) {
        let settings = EditorSettings::test(&cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndef\nghi\n", cx));
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
            )
            .unwrap();
            view.duplicate_line(&DuplicateLine, cx);
            assert_eq!(view.display_text(cx), "abc\nabc\ndef\ndef\nghi\n\n");
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(6, 0)..DisplayPoint::new(6, 0),
                ]
            );
        });

        let settings = EditorSettings::test(&cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndef\nghi\n", cx));
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(2, 1),
                ],
                cx,
            )
            .unwrap();
            view.duplicate_line(&DuplicateLine, cx);
            assert_eq!(view.display_text(cx), "abc\ndef\nghi\nabc\ndef\nghi\n");
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(3, 1)..DisplayPoint::new(4, 1),
                    DisplayPoint::new(4, 2)..DisplayPoint::new(5, 1),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_move_line_up_down(cx: &mut gpui::MutableAppContext) {
        let settings = EditorSettings::test(&cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(10, 5), cx));
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
            )
            .unwrap();
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
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
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
    fn test_clipboard(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "one✅ two three four five six ", cx));
        let settings = EditorSettings::test(&cx);
        let view = cx
            .add_window(Default::default(), |cx| {
                build_editor(buffer.clone(), settings, cx)
            })
            .1;

        // Cut with three selections. Clipboard text is divided into three slices.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..7, 11..17, 22..27], false, cx);
            view.cut(&Cut, cx);
            assert_eq!(view.display_text(cx), "two four six ");
        });

        // Paste with three cursors. Each cursor pastes one slice of the clipboard text.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![4..4, 9..9, 13..13], false, cx);
            view.paste(&Paste, cx);
            assert_eq!(view.display_text(cx), "two one✅ four three six five ");
            assert_eq!(
                view.selection_ranges(cx),
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
            view.select_ranges(vec![0..0, 31..31], false, cx);
            view.handle_input(&Input("( ".into()), cx);
            view.paste(&Paste, cx);
            view.handle_input(&Input(") ".into()), cx);
            assert_eq!(
                view.display_text(cx),
                "( one✅ three five ) two one✅ four three six five ( one✅ three five ) "
            );
        });

        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..0], false, cx);
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
            )
            .unwrap();
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
            )
            .unwrap();
            view.paste(&Paste, cx);
            assert_eq!(
                view.display_text(cx),
                "123\n4567\n9\n( 8ne✅ three five ) two one✅ four three six five ( one✅ three five ) "
            );
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(3, 3)..DisplayPoint::new(3, 3),
                ]
            );
        });

        // Copy with a single cursor only, which writes the whole line into the clipboard.
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)], cx)
                .unwrap();
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
            )
            .unwrap();
            view.paste(&Paste, cx);
            assert_eq!(
                view.display_text(cx),
                "123\n123\n123\n67\n123\n9\n( 8ne✅ three five ) two one✅ four three six five ( one✅ three five ) "
            );
            assert_eq!(
                view.selection_ranges(cx),
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
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\nde\nfgh", cx));
        let settings = EditorSettings::test(&cx);
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_all(&SelectAll, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(2, 3)]
            );
        });
    }

    #[gpui::test]
    fn test_select_line(cx: &mut gpui::MutableAppContext) {
        let settings = EditorSettings::test(&cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(6, 5), cx));
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
            )
            .unwrap();
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(2, 0),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(5, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(5, 5),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_line(&SelectLine, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![DisplayPoint::new(0, 0)..DisplayPoint::new(5, 5)]
            );
        });
    }

    #[gpui::test]
    fn test_split_selection_into_lines(cx: &mut gpui::MutableAppContext) {
        let settings = EditorSettings::test(&cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(9, 5), cx));
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
            )
            .unwrap();
            assert_eq!(view.display_text(cx), "aa…bbb\nccc…eeee\nfffff\nggggg\n…i");
        });

        view.update(cx, |view, cx| {
            view.split_selection_into_lines(&SplitSelectionIntoLines, cx);
            assert_eq!(
                view.display_text(cx),
                "aaaaa\nbbbbb\nccc…eeee\nfffff\nggggg\n…i"
            );
            assert_eq!(
                view.selection_ranges(cx),
                [
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                    DisplayPoint::new(5, 4)..DisplayPoint::new(5, 4)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 1)], cx)
                .unwrap();
            view.split_selection_into_lines(&SplitSelectionIntoLines, cx);
            assert_eq!(
                view.display_text(cx),
                "aaaaa\nbbbbb\nccccc\nddddd\neeeee\nfffff\nggggg\nhhhhh\niiiii"
            );
            assert_eq!(
                view.selection_ranges(cx),
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
        let settings = EditorSettings::test(&cx);
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndefghi\n\njk\nlmno\n", cx));
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)], cx)
                .unwrap();
        });
        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)], cx)
                .unwrap();
        });
        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selection_ranges(cx),
                vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
            );
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(1, 4)], cx)
                .unwrap();
            view.add_selection_below(&AddSelectionBelow, cx);
            assert_eq!(
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(4, 3)..DisplayPoint::new(1, 1)], cx)
                .unwrap();
        });
        view.update(cx, |view, cx| {
            view.add_selection_above(&AddSelectionAbove, cx);
            assert_eq!(
                view.selection_ranges(cx),
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
                view.selection_ranges(cx),
                vec![
                    DisplayPoint::new(1, 3)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(4, 3)..DisplayPoint::new(4, 1),
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_select_larger_smaller_syntax_node(mut cx: gpui::TestAppContext) {
        let settings = cx.read(EditorSettings::test);
        let language = Some(Arc::new(Language::new(
            LanguageConfig::default(),
            tree_sitter_rust::language(),
        )));

        let text = r#"
            use mod1::mod2::{mod3, mod4};

            fn fn_1(param1: bool, param2: &str) {
                let var1 = "text";
            }
        "#
        .unindent();

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, None, cx));
        let (_, view) = cx.add_window(|cx| build_editor(buffer, settings, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing())
            .await;

        view.update(&mut cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                    DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                    DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
                ],
                cx,
            )
            .unwrap();
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 23)..DisplayPoint::new(0, 27),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 15)..DisplayPoint::new(3, 21),
            ]
        );

        view.update(&mut cx, |view, cx| {
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(&mut cx, |view, cx| {
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
        );

        // Trying to expand the selected syntax node one more time has no effect.
        view.update(&mut cx, |view, cx| {
            view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
        );

        view.update(&mut cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(&mut cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 23)..DisplayPoint::new(0, 27),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 15)..DisplayPoint::new(3, 21),
            ]
        );

        view.update(&mut cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
            ]
        );

        // Trying to shrink the selected syntax node one more time has no effect.
        view.update(&mut cx, |view, cx| {
            view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
        });
        assert_eq!(
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
            ]
        );

        // Ensure that we keep expanding the selection if the larger selection starts or ends within
        // a fold.
        view.update(&mut cx, |view, cx| {
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
            view.update(&mut cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 4)..DisplayPoint::new(3, 23),
            ]
        );
    }

    #[gpui::test]
    async fn test_autoclose_pairs(mut cx: gpui::TestAppContext) {
        let settings = cx.read(EditorSettings::test);
        let language = Some(Arc::new(Language::new(
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
            tree_sitter_rust::language(),
        )));

        let text = r#"
            a

            /

        "#
        .unindent();

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, None, cx));
        let (_, view) = cx.add_window(|cx| build_editor(buffer, settings, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing())
            .await;

        view.update(&mut cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ],
                cx,
            )
            .unwrap();
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
            )
            .unwrap();
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
        });
    }

    #[gpui::test]
    async fn test_extra_newline_insertion(mut cx: gpui::TestAppContext) {
        let settings = cx.read(EditorSettings::test);
        let language = Some(Arc::new(Language::new(
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
            tree_sitter_rust::language(),
        )));

        let text = concat!(
            "{   }\n",     // Suppress rustfmt
            "  x\n",       //
            "  /*   */\n", //
            "x\n",         //
            "{{} }\n",     //
        );

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, None, cx));
        let (_, view) = cx.add_window(|cx| build_editor(buffer, settings, cx));
        view.condition(&cx, |view, cx| !view.buffer.read(cx).is_parsing())
            .await;

        view.update(&mut cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(2, 5)..DisplayPoint::new(2, 5),
                    DisplayPoint::new(4, 4)..DisplayPoint::new(4, 4),
                ],
                cx,
            )
            .unwrap();
            view.newline(&Newline, cx);

            assert_eq!(
                view.buffer().read(cx).text(),
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

    impl Editor {
        fn selection_ranges(&self, cx: &mut MutableAppContext) -> Vec<Range<DisplayPoint>> {
            self.selections_in_range(
                self.selection_set_id,
                DisplayPoint::zero()..self.max_point(cx),
                cx,
            )
            .collect::<Vec<_>>()
        }
    }

    fn empty_range(row: usize, column: usize) -> Range<DisplayPoint> {
        let point = DisplayPoint::new(row as u32, column as u32);
        point..point
    }

    fn build_editor(
        buffer: ModelHandle<Buffer>,
        settings: EditorSettings,
        cx: &mut ViewContext<Editor>,
    ) -> Editor {
        Editor::for_buffer(buffer, move |_| settings.clone(), cx)
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

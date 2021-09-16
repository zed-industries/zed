pub mod buffer;
pub mod display_map;
mod element;
pub mod movement;

use crate::{
    settings::{HighlightId, Settings},
    theme::Theme,
    time::ReplicaId,
    util::{post_inc, Bias},
    workspace,
    worktree::{File, Worktree},
};
use anyhow::Result;
pub use buffer::*;
pub use display_map::DisplayPoint;
use display_map::*;
pub use element::*;
use gpui::{
    action,
    color::Color,
    font_cache::FamilyId,
    fonts::{Properties as FontProperties, TextStyle},
    geometry::vector::Vector2F,
    keymap::Binding,
    text_layout::{self, RunStyle},
    AppContext, ClipboardItem, Element, ElementBox, Entity, FontCache, ModelHandle,
    MutableAppContext, RenderContext, Task, TextLayoutCache, View, ViewContext, WeakViewHandle,
};
use postage::watch;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smol::Timer;
use std::{
    cell::RefCell,
    cmp::{self, Ordering},
    collections::BTreeMap,
    fmt::Write,
    iter::FromIterator,
    mem,
    ops::{Range, RangeInclusive},
    path::Path,
    rc::Rc,
    sync::Arc,
    time::Duration,
};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);
const MAX_LINE_LEN: usize = 1024;

action!(Cancel);
action!(Backspace);
action!(Delete);
action!(Insert, String);
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
        Binding::new("enter", Insert("\n".into()), Some("Editor && mode == full")),
        Binding::new(
            "alt-enter",
            Insert("\n".into()),
            Some("Editor && mode == auto_height"),
        ),
        Binding::new("tab", Insert("\t".into()), Some("Editor")),
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
    cx.add_action(Editor::insert);
    cx.add_action(Editor::backspace);
    cx.add_action(Editor::delete);
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
    cx.add_action(Editor::page_up);
    cx.add_action(Editor::page_down);
    cx.add_action(Editor::fold);
    cx.add_action(Editor::unfold);
    cx.add_action(Editor::fold_selected_ranges);
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

#[derive(Clone, Deserialize)]
pub struct EditorStyle {
    pub text: TextStyle,
    #[serde(default)]
    pub placeholder_text: Option<TextStyle>,
    pub background: Color,
    pub selection: SelectionStyle,
    pub gutter_background: Color,
    pub active_line_background: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub guest_selections: Vec<SelectionStyle>,
}

#[derive(Clone, Copy, Default, Deserialize)]
pub struct SelectionStyle {
    pub cursor: Color,
    pub selection: Color,
}

pub struct Editor {
    handle: WeakViewHandle<Self>,
    buffer: ModelHandle<Buffer>,
    display_map: ModelHandle<DisplayMap>,
    selection_set_id: SelectionSetId,
    pending_selection: Option<Selection>,
    next_selection_id: usize,
    add_selections_state: Option<AddSelectionsState>,
    select_larger_syntax_node_stack: Vec<Vec<Selection>>,
    scroll_position: Vector2F,
    scroll_top_anchor: Anchor,
    autoscroll_requested: bool,
    build_style: Rc<RefCell<dyn FnMut(&mut MutableAppContext) -> EditorStyle>>,
    settings: watch::Receiver<Settings>,
    focused: bool,
    cursors_visible: bool,
    blink_epoch: usize,
    blinking_paused: bool,
    mode: EditorMode,
    placeholder_text: Option<Arc<str>>,
}

pub struct Snapshot {
    pub mode: EditorMode,
    pub display_snapshot: DisplayMapSnapshot,
    pub placeholder_text: Option<Arc<str>>,
    pub theme: Arc<Theme>,
    pub font_family: FamilyId,
    pub font_size: f32,
    is_focused: bool,
    scroll_position: Vector2F,
    scroll_top_anchor: Anchor,
}

struct AddSelectionsState {
    above: bool,
    stack: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
struct ClipboardSelection {
    len: usize,
    is_entire_line: bool,
}

impl Editor {
    pub fn single_line(
        settings: watch::Receiver<Settings>,
        build_style: impl 'static + FnMut(&mut MutableAppContext) -> EditorStyle,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let mut view = Self::for_buffer(buffer, settings, build_style, cx);
        view.mode = EditorMode::SingleLine;
        view
    }

    pub fn auto_height(
        max_lines: usize,
        settings: watch::Receiver<Settings>,
        build_style: impl 'static + FnMut(&mut MutableAppContext) -> EditorStyle,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let mut view = Self::for_buffer(buffer, settings, build_style, cx);
        view.mode = EditorMode::AutoHeight { max_lines };
        view
    }

    pub fn for_buffer(
        buffer: ModelHandle<Buffer>,
        settings: watch::Receiver<Settings>,
        build_style: impl 'static + FnMut(&mut MutableAppContext) -> EditorStyle,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::new(buffer, settings, Rc::new(RefCell::new(build_style)), cx)
    }

    fn new(
        buffer: ModelHandle<Buffer>,
        settings: watch::Receiver<Settings>,
        build_style: Rc<RefCell<dyn FnMut(&mut MutableAppContext) -> EditorStyle>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let display_map =
            cx.add_model(|cx| DisplayMap::new(buffer.clone(), settings.clone(), None, cx));
        cx.observe(&buffer, Self::on_buffer_changed).detach();
        cx.subscribe(&buffer, Self::on_buffer_event).detach();
        cx.observe(&display_map, Self::on_display_map_changed)
            .detach();

        let mut next_selection_id = 0;
        let selection_set_id = buffer.update(cx, |buffer, cx| {
            buffer.add_selection_set(
                vec![Selection {
                    id: post_inc(&mut next_selection_id),
                    start: buffer.anchor_before(0),
                    end: buffer.anchor_before(0),
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
            select_larger_syntax_node_stack: Vec::new(),
            build_style,
            scroll_position: Vector2F::zero(),
            scroll_top_anchor: Anchor::min(),
            autoscroll_requested: false,
            settings,
            focused: false,
            cursors_visible: false,
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
        let settings = self.settings.borrow();

        Snapshot {
            mode: self.mode,
            display_snapshot: self.display_map.update(cx, |map, cx| map.snapshot(cx)),
            scroll_position: self.scroll_position,
            scroll_top_anchor: self.scroll_top_anchor.clone(),
            theme: settings.theme.clone(),
            placeholder_text: self.placeholder_text.clone(),
            font_family: settings.buffer_font_family,
            font_size: settings.buffer_font_size,
            is_focused: self
                .handle
                .upgrade(cx)
                .map_or(false, |handle| handle.is_focused(cx)),
        }
    }

    pub fn set_placeholder_text(
        &mut self,
        placeholder_text: impl Into<Arc<str>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.placeholder_text = Some(placeholder_text.into());
        cx.notify();
    }

    fn set_scroll_position(&mut self, mut scroll_position: Vector2F, cx: &mut ViewContext<Self>) {
        let map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let scroll_top_buffer_offset =
            DisplayPoint::new(scroll_position.y() as u32, 0).to_buffer_offset(&map, Bias::Right);
        self.scroll_top_anchor = self
            .buffer
            .read(cx)
            .anchor_at(scroll_top_buffer_offset, Bias::Right);
        scroll_position.set_y(scroll_position.y().fract());
        self.scroll_position = scroll_position;
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

        let first_cursor_top = self
            .selections(cx)
            .first()
            .unwrap()
            .head()
            .to_display_point(&display_map, Bias::Left)
            .row() as f32;
        let last_cursor_bottom = self
            .selections(cx)
            .last()
            .unwrap()
            .head()
            .to_display_point(&display_map, Bias::Right)
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
        cx: &mut MutableAppContext,
    ) -> bool {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut target_left = std::f32::INFINITY;
        let mut target_right = 0.0_f32;
        for selection in self.selections(cx) {
            let head = selection.head().to_display_point(&display_map, Bias::Left);
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
        let cursor = display_map.anchor_before(position, Bias::Left);
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor.clone(),
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };

        if !add {
            self.update_selections(Vec::new(), false, cx);
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
        let buffer = self.buffer.read(cx);
        let cursor = display_map.anchor_before(position, Bias::Left);
        if let Some(selection) = self.pending_selection.as_mut() {
            selection.set_head(buffer, cursor);
        } else {
            log::error!("update_selection dispatched with no pending selection");
            return;
        }

        self.set_scroll_position(scroll_position, cx);
        cx.notify();
    }

    fn end_selection(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.pending_selection.take() {
            let mut selections = self.selections(cx.as_ref()).to_vec();
            let ix = self.selection_insertion_index(&selections, &selection.start, cx.as_ref());
            selections.insert(ix, selection);
            self.update_selections(selections, false, cx);
        } else {
            log::error!("end_selection dispatched with no pending selection");
        }
    }

    pub fn is_selecting(&self) -> bool {
        self.pending_selection.is_some()
    }

    pub fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        let selections = self.selections(cx.as_ref());
        if let Some(pending_selection) = self.pending_selection.take() {
            if selections.is_empty() {
                self.update_selections(vec![pending_selection], true, cx);
            }
        } else {
            let mut oldest_selection = selections.iter().min_by_key(|s| s.id).unwrap().clone();
            if selections.len() == 1 {
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
        let mut selections = Vec::new();
        for range in ranges {
            let mut start = range.start.to_offset(buffer);
            let mut end = range.end.to_offset(buffer);
            let reversed = if start > end {
                mem::swap(&mut start, &mut end);
                true
            } else {
                false
            };
            selections.push(Selection {
                id: post_inc(&mut self.next_selection_id),
                start: buffer.anchor_before(start),
                end: buffer.anchor_before(end),
                reversed,
                goal: SelectionGoal::None,
            });
        }
        self.update_selections(selections, autoscroll, cx);
    }

    #[cfg(test)]
    fn select_display_ranges<'a, T>(&mut self, ranges: T, cx: &mut ViewContext<Self>) -> Result<()>
    where
        T: IntoIterator<Item = &'a Range<DisplayPoint>>,
    {
        let mut selections = Vec::new();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        for range in ranges {
            let mut start = range.start;
            let mut end = range.end;
            let reversed = if start > end {
                mem::swap(&mut start, &mut end);
                true
            } else {
                false
            };

            selections.push(Selection {
                id: post_inc(&mut self.next_selection_id),
                start: display_map.anchor_before(start, Bias::Left),
                end: display_map.anchor_before(end, Bias::Left),
                reversed,
                goal: SelectionGoal::None,
            });
        }
        self.update_selections(selections, false, cx);
        Ok(())
    }

    pub fn insert(&mut self, action: &Insert, cx: &mut ViewContext<Self>) {
        let mut old_selections = SmallVec::<[_; 32]>::new();
        {
            let buffer = self.buffer.read(cx);
            for selection in self.selections(cx.as_ref()) {
                let start = selection.start.to_offset(buffer);
                let end = selection.end.to_offset(buffer);
                old_selections.push((selection.id, start..end));
            }
        }

        self.start_transaction(cx);
        let mut new_selections = Vec::new();
        self.buffer.update(cx, |buffer, cx| {
            let edit_ranges = old_selections.iter().map(|(_, range)| range.clone());
            buffer.edit(edit_ranges, action.0.as_str(), cx);
            let text_len = action.0.len() as isize;
            let mut delta = 0_isize;
            new_selections = old_selections
                .into_iter()
                .map(|(id, range)| {
                    let start = range.start as isize;
                    let end = range.end as isize;
                    let anchor = buffer.anchor_before((start + delta + text_len) as usize);
                    let deleted_count = end - start;
                    delta += text_len - deleted_count;
                    Selection {
                        id,
                        start: anchor.clone(),
                        end: anchor,
                        reversed: false,
                        goal: SelectionGoal::None,
                    }
                })
                .collect();
        });

        self.update_selections(new_selections, true, cx);
        self.end_transaction(cx);
    }

    pub fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_all(&SelectAll, cx);
        self.insert(&Insert(String::new()), cx);
        self.end_transaction(cx);
    }

    pub fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut selections = self.selections(cx.as_ref()).to_vec();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let range = selection.point_range(buffer);
                if range.start == range.end {
                    let head = selection.head().to_display_point(&display_map, Bias::Left);
                    let cursor = display_map
                        .anchor_before(movement::left(&display_map, head).unwrap(), Bias::Left);
                    selection.set_head(&buffer, cursor);
                    selection.goal = SelectionGoal::None;
                }
            }
        }

        self.update_selections(selections, true, cx);
        self.insert(&Insert(String::new()), cx);
        self.end_transaction(cx);
    }

    pub fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let range = selection.point_range(buffer);
                if range.start == range.end {
                    let head = selection.head().to_display_point(&display_map, Bias::Left);
                    let cursor = display_map
                        .anchor_before(movement::right(&display_map, head).unwrap(), Bias::Right);
                    selection.set_head(&buffer, cursor);
                    selection.goal = SelectionGoal::None;
                }
            }
        }

        self.update_selections(selections, true, cx);
        self.insert(&Insert(String::new()), cx);
        self.end_transaction(cx);
    }

    pub fn delete_line(&mut self, _: &DeleteLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let app = cx.as_ref();
        let buffer = self.buffer.read(app);

        let mut new_cursors = Vec::new();
        let mut edit_ranges = Vec::new();

        let mut selections = self.selections(app).iter().peekable();
        while let Some(selection) = selections.next() {
            let mut rows = selection.spanned_rows(false, &display_map).buffer_rows;
            let goal_display_column = selection
                .head()
                .to_display_point(&display_map, Bias::Left)
                .column();

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
                cursor_buffer_row = rows.end;
            } else {
                // If there isn't a line after the range, delete the \n from the line before the
                // start of the row range and position the cursor there.
                edit_start = edit_start.saturating_sub(1);
                edit_end = buffer.len();
                cursor_buffer_row = rows.start.saturating_sub(1);
            }

            let mut cursor =
                Point::new(cursor_buffer_row, 0).to_display_point(&display_map, Bias::Left);
            *cursor.column_mut() =
                cmp::min(goal_display_column, display_map.line_len(cursor.row()));

            new_cursors.push((
                selection.id,
                cursor.to_buffer_point(&display_map, Bias::Left),
            ));
            edit_ranges.push(edit_start..edit_end);
        }

        new_cursors.sort_unstable_by_key(|(_, range)| range.clone());
        let new_selections = new_cursors
            .into_iter()
            .map(|(id, cursor)| {
                let anchor = buffer.anchor_before(cursor);
                Selection {
                    id,
                    start: anchor.clone(),
                    end: anchor,
                    reversed: false,
                    goal: SelectionGoal::None,
                }
            })
            .collect();
        self.buffer
            .update(cx, |buffer, cx| buffer.edit(edit_ranges, "", cx));
        self.update_selections(new_selections, true, cx);
        self.end_transaction(cx);
    }

    pub fn duplicate_line(&mut self, _: &DuplicateLine, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            // Temporarily bias selections right to allow newly duplicate lines to push them down
            // when the selections are at the beginning of a line.
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                selection.start = selection.start.bias_right(buffer);
                selection.end = selection.end.bias_right(buffer);
            }
        }
        self.update_selections(selections.clone(), false, cx);

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);

        let mut edits = Vec::new();
        let mut selections_iter = selections.iter_mut().peekable();
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
            edits.push((start, text));
        }

        self.buffer.update(cx, |buffer, cx| {
            for (offset, text) in edits.into_iter().rev() {
                buffer.edit(Some(offset..offset), text, cx);
            }
        });

        // Restore bias on selections.
        let buffer = self.buffer.read(cx);
        for selection in &mut selections {
            selection.start = selection.start.bias_left(buffer);
            selection.end = selection.end.bias_left(buffer);
        }
        self.update_selections(selections, true, cx);

        self.end_transaction(cx);
    }

    pub fn move_line_up(&mut self, _: &MoveLineUp, cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let app = cx.as_ref();
        let buffer = self.buffer.read(cx);

        let mut edits = Vec::new();
        let mut new_selection_ranges = Vec::new();
        let mut old_folds = Vec::new();
        let mut new_folds = Vec::new();

        let mut selections = self.selections(app).iter().peekable();
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

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let app = cx.as_ref();
        let buffer = self.buffer.read(cx);

        let mut edits = Vec::new();
        let mut new_selection_ranges = Vec::new();
        let mut old_folds = Vec::new();
        let mut new_folds = Vec::new();

        let mut selections = self.selections(app).iter().peekable();
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
        let mut selections = self.selections(cx.as_ref()).to_vec();
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let buffer = self.buffer.read(cx);
            let max_point = buffer.max_point();
            for selection in &mut selections {
                let mut start = selection.start.to_point(buffer);
                let mut end = selection.end.to_point(buffer);
                let is_entire_line = start == end;
                if is_entire_line {
                    start = Point::new(start.row, 0);
                    end = cmp::min(max_point, Point::new(start.row + 1, 0));
                    selection.start = buffer.anchor_before(start);
                    selection.end = buffer.anchor_before(end);
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
        self.update_selections(selections, true, cx);
        self.insert(&Insert(String::new()), cx);
        self.end_transaction(cx);

        cx.as_mut()
            .write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        let max_point = buffer.max_point();
        let mut text = String::new();
        let selections = self.selections(cx.as_ref());
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        for selection in selections {
            let mut start = selection.start.to_point(buffer);
            let mut end = selection.end.to_point(buffer);
            let is_entire_line = start == end;
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
                let selections = self.selections(cx.as_ref()).to_vec();
                if clipboard_selections.len() != selections.len() {
                    let merged_selection = ClipboardSelection {
                        len: clipboard_selections.iter().map(|s| s.len).sum(),
                        is_entire_line: clipboard_selections.iter().all(|s| s.is_entire_line),
                    };
                    clipboard_selections.clear();
                    clipboard_selections.push(merged_selection);
                }

                self.start_transaction(cx);
                let mut new_selections = Vec::with_capacity(selections.len());
                let mut clipboard_chars = clipboard_text.chars().cycle();
                for (selection, clipboard_selection) in
                    selections.iter().zip(clipboard_selections.iter().cycle())
                {
                    let to_insert =
                        String::from_iter(clipboard_chars.by_ref().take(clipboard_selection.len));

                    self.buffer.update(cx, |buffer, cx| {
                        let selection_start = selection.start.to_point(&*buffer);
                        let selection_end = selection.end.to_point(&*buffer);

                        // If the corresponding selection was empty when this slice of the
                        // clipboard text was written, then the entire line containing the
                        // selection was copied. If this selection is also currently empty,
                        // then paste the line before the current line of the buffer.
                        let new_selection_start = selection.end.bias_right(buffer);
                        if selection_start == selection_end && clipboard_selection.is_entire_line {
                            let line_start = Point::new(selection_start.row, 0);
                            buffer.edit(Some(line_start..line_start), to_insert, cx);
                        } else {
                            buffer.edit(Some(&selection.start..&selection.end), to_insert, cx);
                        };

                        let new_selection_start = new_selection_start.bias_left(buffer);
                        new_selections.push(Selection {
                            id: selection.id,
                            start: new_selection_start.clone(),
                            end: new_selection_start,
                            reversed: false,
                            goal: SelectionGoal::None,
                        });
                    });
                }
                self.update_selections(new_selections, true, cx);
                self.end_transaction(cx);
            } else {
                self.insert(&Insert(clipboard_text.into()), cx);
            }
        }
    }

    pub fn undo(&mut self, _: &Undo, cx: &mut ViewContext<Self>) {
        self.buffer.update(cx, |buffer, cx| buffer.undo(cx));
    }

    pub fn redo(&mut self, _: &Redo, cx: &mut ViewContext<Self>) {
        self.buffer.update(cx, |buffer, cx| buffer.redo(cx));
    }

    pub fn move_left(&mut self, _: &MoveLeft, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let start = selection.start.to_display_point(&display_map, Bias::Left);
                let end = selection.end.to_display_point(&display_map, Bias::Left);

                if start != end {
                    selection.end = selection.start.clone();
                } else {
                    let cursor = display_map
                        .anchor_before(movement::left(&display_map, start).unwrap(), Bias::Left);
                    selection.start = cursor.clone();
                    selection.end = cursor;
                }
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_left(&mut self, _: &SelectLeft, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let cursor = display_map
                    .anchor_before(movement::left(&display_map, head).unwrap(), Bias::Left);
                selection.set_head(&buffer, cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_right(&mut self, _: &MoveRight, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            for selection in &mut selections {
                let start = selection.start.to_display_point(&display_map, Bias::Left);
                let end = selection.end.to_display_point(&display_map, Bias::Left);

                if start != end {
                    selection.start = selection.end.clone();
                } else {
                    let cursor = display_map
                        .anchor_before(movement::right(&display_map, end).unwrap(), Bias::Right);
                    selection.start = cursor.clone();
                    selection.end = cursor;
                }
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_right(&mut self, _: &SelectRight, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let app = cx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let cursor = display_map
                    .anchor_before(movement::right(&display_map, head).unwrap(), Bias::Right);
                selection.set_head(&buffer, cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
        } else {
            let mut selections = self.selections(cx.as_ref()).to_vec();
            {
                for selection in &mut selections {
                    let start = selection.start.to_display_point(&display_map, Bias::Left);
                    let end = selection.end.to_display_point(&display_map, Bias::Left);
                    if start != end {
                        selection.goal = SelectionGoal::None;
                    }

                    let (start, goal) = movement::up(&display_map, start, selection.goal).unwrap();
                    let cursor = display_map.anchor_before(start, Bias::Left);
                    selection.start = cursor.clone();
                    selection.end = cursor;
                    selection.goal = goal;
                    selection.reversed = false;
                }
            }
            self.update_selections(selections, true, cx);
        }
    }

    pub fn select_up(&mut self, _: &SelectUp, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let app = cx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let (head, goal) = movement::up(&display_map, head, selection.goal).unwrap();
                selection.set_head(&buffer, display_map.anchor_before(head, Bias::Left));
                selection.goal = goal;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
        } else {
            let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
            let mut selections = self.selections(cx.as_ref()).to_vec();
            {
                for selection in &mut selections {
                    let start = selection.start.to_display_point(&display_map, Bias::Left);
                    let end = selection.end.to_display_point(&display_map, Bias::Left);
                    if start != end {
                        selection.goal = SelectionGoal::None;
                    }

                    let (start, goal) = movement::down(&display_map, end, selection.goal).unwrap();
                    let cursor = display_map.anchor_before(start, Bias::Right);
                    selection.start = cursor.clone();
                    selection.end = cursor;
                    selection.goal = goal;
                    selection.reversed = false;
                }
            }
            self.update_selections(selections, true, cx);
        }
    }

    pub fn select_down(&mut self, _: &SelectDown, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx).to_vec();
        {
            let app = cx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let (head, goal) = movement::down(&display_map, head, selection.goal).unwrap();
                selection.set_head(&buffer, display_map.anchor_before(head, Bias::Right));
                selection.goal = goal;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_to_previous_word_boundary(
        &mut self,
        _: &MoveToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx).to_vec();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let new_head = movement::prev_word_boundary(&display_map, head).unwrap();
                let anchor = display_map.anchor_before(new_head, Bias::Left);
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_previous_word_boundary(
        &mut self,
        _: &SelectToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let new_head = movement::prev_word_boundary(&display_map, head).unwrap();
                let anchor = display_map.anchor_before(new_head, Bias::Left);
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_previous_word_boundary(
        &mut self,
        _: &DeleteToPreviousWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        self.start_transaction(cx);
        self.select_to_previous_word_boundary(&SelectToPreviousWordBoundary, cx);
        self.backspace(&Backspace, cx);
        self.end_transaction(cx);
    }

    pub fn move_to_next_word_boundary(
        &mut self,
        _: &MoveToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx).to_vec();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let new_head = movement::next_word_boundary(&display_map, head).unwrap();
                let anchor = display_map.anchor_before(new_head, Bias::Left);
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_next_word_boundary(
        &mut self,
        _: &SelectToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let new_head = movement::next_word_boundary(&display_map, head).unwrap();
                let anchor = display_map.anchor_before(new_head, Bias::Left);
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_next_word_boundary(
        &mut self,
        _: &DeleteToNextWordBoundary,
        cx: &mut ViewContext<Self>,
    ) {
        self.start_transaction(cx);
        self.select_to_next_word_boundary(&SelectToNextWordBoundary, cx);
        self.delete(&Delete, cx);
        self.end_transaction(cx);
    }

    pub fn move_to_beginning_of_line(
        &mut self,
        _: &MoveToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx).to_vec();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let new_head = movement::line_beginning(&display_map, head, true).unwrap();
                let anchor = display_map.anchor_before(new_head, Bias::Left);
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_beginning_of_line(
        &mut self,
        SelectToBeginningOfLine(toggle_indent): &SelectToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections(cx).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let new_head =
                    movement::line_beginning(&display_map, head, *toggle_indent).unwrap();
                let anchor = display_map.anchor_before(new_head, Bias::Left);
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
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
        let mut selections = self.selections(cx).to_vec();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let new_head = movement::line_end(&display_map, head).unwrap();
                let anchor = display_map.anchor_before(new_head, Bias::Left);
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
        let mut selections = self.selections(cx).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&display_map, Bias::Left);
                let new_head = movement::line_end(&display_map, head).unwrap();
                let anchor = display_map.anchor_before(new_head, Bias::Left);
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
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
        let buffer = self.buffer.read(cx);
        let cursor = buffer.anchor_before(Point::new(0, 0));
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor.clone(),
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], true, cx);
    }

    pub fn select_to_beginning(&mut self, _: &SelectToBeginning, cx: &mut ViewContext<Self>) {
        let mut selection = self.selections(cx.as_ref()).last().unwrap().clone();
        selection.set_head(self.buffer.read(cx), Anchor::min());
        self.update_selections(vec![selection], true, cx);
    }

    pub fn move_to_end(&mut self, _: &MoveToEnd, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        let cursor = buffer.anchor_before(buffer.max_point());
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor.clone(),
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], true, cx);
    }

    pub fn select_to_end(&mut self, _: &SelectToEnd, cx: &mut ViewContext<Self>) {
        let mut selection = self.selections(cx.as_ref()).last().unwrap().clone();
        selection.set_head(self.buffer.read(cx), Anchor::max());
        self.update_selections(vec![selection], true, cx);
    }

    pub fn select_all(&mut self, _: &SelectAll, cx: &mut ViewContext<Self>) {
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: Anchor::min(),
            end: Anchor::max(),
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], false, cx);
    }

    pub fn select_line(&mut self, _: &SelectLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);
        let mut selections = self.selections(cx).to_vec();
        let max_point = buffer.max_point();
        for selection in &mut selections {
            let rows = selection.spanned_rows(true, &display_map).buffer_rows;
            selection.start = buffer.anchor_before(Point::new(rows.start, 0));
            selection.end = buffer.anchor_before(cmp::min(max_point, Point::new(rows.end, 0)));
            selection.reversed = false;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn split_selection_into_lines(
        &mut self,
        _: &SplitSelectionIntoLines,
        cx: &mut ViewContext<Self>,
    ) {
        let app = cx.as_ref();
        let buffer = self.buffer.read(app);

        let mut to_unfold = Vec::new();
        let mut new_selections = Vec::new();
        for selection in self.selections(app) {
            let range = selection.point_range(buffer).sorted();
            if range.start.row != range.end.row {
                new_selections.push(Selection {
                    id: post_inc(&mut self.next_selection_id),
                    start: selection.start.clone(),
                    end: selection.start.clone(),
                    reversed: false,
                    goal: SelectionGoal::None,
                });
            }
            for row in range.start.row + 1..range.end.row {
                let cursor = buffer.anchor_before(Point::new(row, buffer.line_len(row)));
                new_selections.push(Selection {
                    id: post_inc(&mut self.next_selection_id),
                    start: cursor.clone(),
                    end: cursor,
                    reversed: false,
                    goal: SelectionGoal::None,
                });
            }
            new_selections.push(Selection {
                id: selection.id,
                start: selection.end.clone(),
                end: selection.end.clone(),
                reversed: false,
                goal: SelectionGoal::None,
            });
            to_unfold.push(range);
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
        let mut selections = self.selections(cx).to_vec();
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
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);

        let mut stack = mem::take(&mut self.select_larger_syntax_node_stack);
        let mut selected_larger_node = false;
        let old_selections = self.selections(cx).to_vec();
        let mut new_selection_ranges = Vec::new();
        for selection in &old_selections {
            let old_range = selection.start.to_offset(buffer)..selection.end.to_offset(buffer);
            let mut new_range = old_range.clone();
            while let Some(containing_range) = buffer.range_for_syntax_ancestor(new_range.clone()) {
                new_range = containing_range;
                if !display_map.intersects_fold(new_range.start)
                    && !display_map.intersects_fold(new_range.end)
                {
                    break;
                }
            }

            selected_larger_node |= new_range != old_range;
            new_selection_ranges.push((selection.id, new_range, selection.reversed));
        }

        if selected_larger_node {
            stack.push(old_selections);
            new_selection_ranges.sort_unstable_by_key(|(_, range, _)| range.start.clone());
            let new_selections = new_selection_ranges
                .into_iter()
                .map(|(id, range, reversed)| Selection {
                    id,
                    start: buffer.anchor_before(range.start),
                    end: buffer.anchor_before(range.end),
                    reversed,
                    goal: SelectionGoal::None,
                })
                .collect();
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
            self.update_selections(selections, true, cx);
        }
        self.select_larger_syntax_node_stack = stack;
    }

    pub fn move_to_enclosing_bracket(
        &mut self,
        _: &MoveToEnclosingBracket,
        cx: &mut ViewContext<Self>,
    ) {
        let buffer = self.buffer.read(cx.as_ref());
        let mut selections = self.selections(cx.as_ref()).to_vec();
        for selection in &mut selections {
            let selection_range = selection.offset_range(buffer);
            if let Some((open_range, close_range)) =
                buffer.enclosing_bracket_ranges(selection_range.clone())
            {
                let close_range = close_range.to_inclusive();
                let destination = if close_range.contains(&selection_range.start)
                    && close_range.contains(&selection_range.end)
                {
                    open_range.end
                } else {
                    *close_range.start()
                };
                selection.start = buffer.anchor_before(destination);
                selection.end = selection.start.clone();
            }
        }

        self.update_selections(selections, true, cx);
    }

    fn build_columnar_selection(
        &mut self,
        display_map: &DisplayMapSnapshot,
        row: u32,
        columns: &Range<u32>,
        reversed: bool,
    ) -> Option<Selection> {
        let is_empty = columns.start == columns.end;
        let line_len = display_map.line_len(row);
        if columns.start < line_len || (is_empty && columns.start == line_len) {
            let start = DisplayPoint::new(row, columns.start);
            let end = DisplayPoint::new(row, cmp::min(columns.end, line_len));
            Some(Selection {
                id: post_inc(&mut self.next_selection_id),
                start: display_map.anchor_before(start, Bias::Left),
                end: display_map.anchor_before(end, Bias::Left),
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
        let selections = &buffer.selection_set(set_id).unwrap().selections;
        let start = display_map.anchor_before(range.start, Bias::Left);
        let start_index = self.selection_insertion_index(selections, &start, cx);
        let pending_selection = if set_id.replica_id == self.buffer.read(cx).replica_id() {
            self.pending_selection.as_ref().and_then(|s| {
                let selection_range = s.display_range(&display_map);
                if selection_range.start <= range.end || selection_range.end <= range.end {
                    Some(selection_range)
                } else {
                    None
                }
            })
        } else {
            None
        };
        selections[start_index..]
            .iter()
            .map(move |s| s.display_range(&display_map))
            .take_while(move |r| r.start <= range.end || r.end <= range.end)
            .chain(pending_selection)
    }

    fn selection_insertion_index(
        &self,
        selections: &[Selection],
        start: &Anchor,
        cx: &AppContext,
    ) -> usize {
        let buffer = self.buffer.read(cx);
        match selections.binary_search_by(|probe| probe.start.cmp(&start, buffer).unwrap()) {
            Ok(index) => index,
            Err(index) => {
                if index > 0
                    && selections[index - 1].end.cmp(&start, buffer).unwrap() == Ordering::Greater
                {
                    index - 1
                } else {
                    index
                }
            }
        }
    }

    fn selections<'a>(&self, cx: &'a AppContext) -> &'a [Selection] {
        let buffer = self.buffer.read(cx);
        &buffer
            .selection_set(self.selection_set_id)
            .unwrap()
            .selections
    }

    fn update_selections(
        &mut self,
        mut selections: Vec<Selection>,
        autoscroll: bool,
        cx: &mut ViewContext<Self>,
    ) {
        // Merge overlapping selections.
        let buffer = self.buffer.read(cx);
        let mut i = 1;
        while i < selections.len() {
            if selections[i - 1]
                .end
                .cmp(&selections[i].start, buffer)
                .unwrap()
                >= Ordering::Equal
            {
                let removed = selections.remove(i);
                if removed.start.cmp(&selections[i - 1].start, buffer).unwrap() < Ordering::Equal {
                    selections[i - 1].start = removed.start;
                }
                if removed.end.cmp(&selections[i - 1].end, buffer).unwrap() > Ordering::Equal {
                    selections[i - 1].end = removed.end;
                }
            } else {
                i += 1;
            }
        }

        self.buffer.update(cx, |buffer, cx| {
            buffer
                .update_selection_set(self.selection_set_id, selections, cx)
                .unwrap();
        });
        self.pause_cursor_blinking(cx);

        if autoscroll {
            self.autoscroll_requested = true;
            cx.notify();
        }

        self.add_selections_state = None;
        self.select_larger_syntax_node_stack.clear();
    }

    fn start_transaction(&self, cx: &mut ViewContext<Self>) {
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

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        for selection in self.selections(cx) {
            let range = selection.display_range(&display_map).sorted();
            let buffer_start_row = range.start.to_buffer_point(&display_map, Bias::Left).row;

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
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx);
        let ranges = self
            .selections(cx)
            .iter()
            .map(|s| {
                let range = s.display_range(&display_map).sorted();
                let mut start = range.start.to_buffer_point(&display_map, Bias::Left);
                let mut end = range.end.to_buffer_point(&display_map, Bias::Left);
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
        return start.to_buffer_point(display_map, Bias::Left)
            ..end.to_buffer_point(display_map, Bias::Left);
    }

    pub fn fold_selected_ranges(&mut self, _: &FoldSelectedRanges, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        let ranges = self
            .selections(cx.as_ref())
            .iter()
            .map(|s| s.point_range(buffer).sorted())
            .collect();
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

    pub fn font_size(&self) -> f32 {
        self.settings.borrow().buffer_font_size
    }

    pub fn set_wrap_width(&self, width: f32, cx: &mut MutableAppContext) -> bool {
        self.display_map
            .update(cx, |map, cx| map.set_wrap_width(Some(width), cx))
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    fn pause_cursor_blinking(&mut self, cx: &mut ViewContext<Self>) {
        self.cursors_visible = true;
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
            self.cursors_visible = !self.cursors_visible;
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

    pub fn cursors_visible(&self) -> bool {
        self.cursors_visible
    }

    fn on_buffer_changed(&mut self, _: ModelHandle<Buffer>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn on_buffer_event(
        &mut self,
        _: ModelHandle<Buffer>,
        event: &buffer::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            buffer::Event::Edited => cx.emit(Event::Edited),
            buffer::Event::Dirtied => cx.emit(Event::Dirtied),
            buffer::Event::Saved => cx.emit(Event::Saved),
            buffer::Event::FileHandleChanged => cx.emit(Event::FileHandleChanged),
            buffer::Event::Reloaded => cx.emit(Event::FileHandleChanged),
            buffer::Event::Reparsed => {}
        }
    }

    fn on_display_map_changed(&mut self, _: ModelHandle<DisplayMap>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}

impl Snapshot {
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

    pub fn font_ascent(&self, font_cache: &FontCache) -> f32 {
        let font_id = font_cache.default_font(self.font_family);
        let ascent = font_cache.metric(font_id, |m| m.ascent);
        font_cache.scale_metric(ascent, font_id, self.font_size)
    }

    pub fn font_descent(&self, font_cache: &FontCache) -> f32 {
        let font_id = font_cache.default_font(self.font_family);
        let descent = font_cache.metric(font_id, |m| m.descent);
        font_cache.scale_metric(descent, font_id, self.font_size)
    }

    pub fn line_height(&self, font_cache: &FontCache) -> f32 {
        let font_id = font_cache.default_font(self.font_family);
        font_cache.line_height(font_id, self.font_size).ceil()
    }

    pub fn em_width(&self, font_cache: &FontCache) -> f32 {
        let font_id = font_cache.default_font(self.font_family);
        font_cache.em_width(font_id, self.font_size)
    }

    // TODO: Can we make this not return a result?
    pub fn max_line_number_width(
        &self,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
    ) -> Result<f32> {
        let font_size = self.font_size;
        let font_id = font_cache.select_font(self.font_family, &FontProperties::new())?;
        let digit_count = (self.display_snapshot.buffer_row_count() as f32)
            .log10()
            .floor() as usize
            + 1;

        Ok(layout_cache
            .layout_str(
                "1".repeat(digit_count).as_str(),
                font_size,
                &[(
                    digit_count,
                    RunStyle {
                        font_id,
                        color: Color::black(),
                        underline: false,
                    },
                )],
            )
            .width())
    }

    pub fn layout_line_numbers(
        &self,
        rows: Range<u32>,
        active_rows: &BTreeMap<u32, bool>,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        theme: &Theme,
    ) -> Result<Vec<Option<text_layout::Line>>> {
        let font_id = font_cache.select_font(self.font_family, &FontProperties::new())?;

        let mut layouts = Vec::with_capacity(rows.len());
        let mut line_number = String::new();
        for (ix, (buffer_row, soft_wrapped)) in self
            .display_snapshot
            .buffer_rows(rows.start)
            .take((rows.end - rows.start) as usize)
            .enumerate()
        {
            let display_row = rows.start + ix as u32;
            let color = if active_rows.contains_key(&display_row) {
                theme.editor.line_number_active
            } else {
                theme.editor.line_number
            };
            if soft_wrapped {
                layouts.push(None);
            } else {
                line_number.clear();
                write!(&mut line_number, "{}", buffer_row + 1).unwrap();
                layouts.push(Some(layout_cache.layout_str(
                    &line_number,
                    self.font_size,
                    &[(
                        line_number.len(),
                        RunStyle {
                            font_id,
                            color,
                            underline: false,
                        },
                    )],
                )));
            }
        }

        Ok(layouts)
    }

    pub fn layout_lines(
        &mut self,
        mut rows: Range<u32>,
        style: &EditorStyle,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
    ) -> Result<Vec<text_layout::Line>> {
        rows.end = cmp::min(rows.end, self.display_snapshot.max_point().row() + 1);
        if rows.start >= rows.end {
            return Ok(Vec::new());
        }

        // When the editor is empty and unfocused, then show the placeholder.
        if self.display_snapshot.is_empty() && !self.is_focused {
            let placeholder_lines = self
                .placeholder_text
                .as_ref()
                .map_or("", AsRef::as_ref)
                .split('\n')
                .skip(rows.start as usize)
                .take(rows.len());
            let font_id = font_cache
                .select_font(self.font_family, &style.placeholder_text().font_properties)?;
            return Ok(placeholder_lines
                .into_iter()
                .map(|line| {
                    layout_cache.layout_str(
                        line,
                        self.font_size,
                        &[(
                            line.len(),
                            RunStyle {
                                font_id,
                                color: style.placeholder_text().color,
                                underline: false,
                            },
                        )],
                    )
                })
                .collect());
        }

        let mut prev_font_properties = FontProperties::new();
        let mut prev_font_id = font_cache
            .select_font(self.font_family, &prev_font_properties)
            .unwrap();

        let mut layouts = Vec::with_capacity(rows.len());
        let mut line = String::new();
        let mut styles = Vec::new();
        let mut row = rows.start;
        let mut line_exceeded_max_len = false;
        let chunks = self
            .display_snapshot
            .highlighted_chunks_for_rows(rows.clone());

        'outer: for (chunk, style_ix) in chunks.chain(Some(("\n", HighlightId::default()))) {
            for (ix, mut line_chunk) in chunk.split('\n').enumerate() {
                if ix > 0 {
                    layouts.push(layout_cache.layout_str(&line, self.font_size, &styles));
                    line.clear();
                    styles.clear();
                    row += 1;
                    line_exceeded_max_len = false;
                    if row == rows.end {
                        break 'outer;
                    }
                }

                if !line_chunk.is_empty() && !line_exceeded_max_len {
                    let style = self
                        .theme
                        .syntax
                        .highlight_style(style_ix)
                        .unwrap_or(style.text.clone().into());
                    // Avoid a lookup if the font properties match the previous ones.
                    let font_id = if style.font_properties == prev_font_properties {
                        prev_font_id
                    } else {
                        font_cache.select_font(self.font_family, &style.font_properties)?
                    };

                    if line.len() + line_chunk.len() > MAX_LINE_LEN {
                        let mut chunk_len = MAX_LINE_LEN - line.len();
                        while !line_chunk.is_char_boundary(chunk_len) {
                            chunk_len -= 1;
                        }
                        line_chunk = &line_chunk[..chunk_len];
                        line_exceeded_max_len = true;
                    }

                    line.push_str(line_chunk);
                    styles.push((
                        line_chunk.len(),
                        RunStyle {
                            font_id,
                            color: style.color,
                            underline: style.underline,
                        },
                    ));
                    prev_font_id = font_id;
                    prev_font_properties = style.font_properties;
                }
            }
        }

        Ok(layouts)
    }

    pub fn layout_line(
        &self,
        row: u32,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
    ) -> Result<text_layout::Line> {
        let font_id = font_cache.select_font(self.font_family, &FontProperties::new())?;

        let mut line = self.display_snapshot.line(row);

        if line.len() > MAX_LINE_LEN {
            let mut len = MAX_LINE_LEN;
            while !line.is_char_boundary(len) {
                len -= 1;
            }
            line.truncate(len);
        }

        Ok(layout_cache.layout_str(
            &line,
            self.font_size,
            &[(
                self.display_snapshot.line_len(row) as usize,
                RunStyle {
                    font_id,
                    color: Color::black(),
                    underline: false,
                },
            )],
        ))
    }

    pub fn prev_row_boundary(&self, point: DisplayPoint) -> (DisplayPoint, Point) {
        self.display_snapshot.prev_row_boundary(point)
    }

    pub fn next_row_boundary(&self, point: DisplayPoint) -> (DisplayPoint, Point) {
        self.display_snapshot.next_row_boundary(point)
    }
}

impl EditorStyle {
    #[cfg(any(test, feature ="test-support"))]
    pub fn test(font_cache: &FontCache) -> Self {
        let font_family_name = "Monaco";
        let font_properties = Default::default();
        let family_id = font_cache.load_family(&[font_family_name]).unwrap();
        let font_id = font_cache.select_font(family_id, &font_properties).unwrap();
        Self {
            text: TextStyle {
                font_family_name: font_family_name.into(),
                font_id,
                font_size: 14.,
                color: Color::from_u32(0xff0000ff),
                font_properties,
                underline: false,
            },
            placeholder_text: None,
            background: Default::default(),
            gutter_background: Default::default(),
            active_line_background: Default::default(),
            line_number: Default::default(),
            line_number_active: Default::default(),
            selection: Default::default(),
            guest_selections: Default::default(),
        }
    }

    fn placeholder_text(&self) -> &TextStyle {
        self.placeholder_text.as_ref().unwrap_or(&self.text)
    }
}

fn compute_scroll_position(
    snapshot: &DisplayMapSnapshot,
    mut scroll_position: Vector2F,
    scroll_top_anchor: &Anchor,
) -> Vector2F {
    let scroll_top = scroll_top_anchor
        .to_display_point(snapshot, Bias::Left)
        .row() as f32;
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
        let style = self.build_style.borrow_mut()(cx);
        EditorElement::new(self.handle.clone(), style).boxed()
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
        self.cursors_visible = false;
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

impl workspace::Item for Buffer {
    type View = Editor;

    fn file(&self) -> Option<&File> {
        self.file()
    }

    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self::View>,
    ) -> Self::View {
        Editor::for_buffer(
            handle,
            settings.clone(),
            move |_| settings.borrow().theme.editor.clone(),
            cx,
        )
    }
}

impl workspace::ItemView for Editor {
    fn should_activate_item_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::Activate)
    }

    fn should_update_tab_on_event(event: &Self::Event) -> bool {
        matches!(
            event,
            Event::Saved | Event::Dirtied | Event::FileHandleChanged
        )
    }

    fn title(&self, cx: &AppContext) -> std::string::String {
        let filename = self
            .buffer
            .read(cx)
            .file()
            .and_then(|file| file.file_name(cx));
        if let Some(name) = filename {
            name.to_string_lossy().into()
        } else {
            "untitled".into()
        }
    }

    fn entry_id(&self, cx: &AppContext) -> Option<(usize, Arc<Path>)> {
        self.buffer.read(cx).file().map(|file| file.entry_id())
    }

    fn clone_on_split(&self, cx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut clone = Editor::new(
            self.buffer.clone(),
            self.settings.clone(),
            self.build_style.clone(),
            cx,
        );
        clone.scroll_position = self.scroll_position;
        clone.scroll_top_anchor = self.scroll_top_anchor.clone();
        Some(clone)
    }

    fn save(&mut self, cx: &mut ViewContext<Self>) -> Result<Task<Result<()>>> {
        let save = self.buffer.update(cx, |b, cx| b.save(cx))?;
        Ok(cx.spawn(|_, _| async move {
            save.await?;
            Ok(())
        }))
    }

    fn save_as(
        &mut self,
        worktree: &ModelHandle<Worktree>,
        path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.buffer
            .update(cx, |b, cx| b.save_as(worktree, path, cx))
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.buffer.read(cx).is_dirty()
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.buffer.read(cx).has_conflict()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Point, language::LanguageRegistry, settings, test::sample_text};
    use buffer::History;
    use unindent::Unindent;

    #[gpui::test]
    fn test_selection_with_mouse(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx));
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
    fn test_layout_line_numbers(cx: &mut gpui::MutableAppContext) {
        let layout_cache = TextLayoutCache::new(cx.platform().fonts());
        let font_cache = cx.font_cache().clone();

        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6), cx));

        let settings = settings::test(&cx).1;
        let (_, editor) = cx.add_window(Default::default(), |cx| {
            build_editor(buffer, settings.clone(), cx)
        });

        let layouts = editor.update(cx, |editor, cx| {
            editor
                .snapshot(cx)
                .layout_line_numbers(
                    0..6,
                    &Default::default(),
                    &font_cache,
                    &layout_cache,
                    &settings.borrow().theme,
                )
                .unwrap()
        });
        assert_eq!(layouts.len(), 6);
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
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
                    DisplayPoint::new(0, 24)..DisplayPoint::new(0, 24),
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
                    DisplayPoint::new(0, 4)..DisplayPoint::new(0, 4),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.move_to_next_word_boundary(&MoveToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
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
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_previous_word_boundary(&SelectToPreviousWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 0),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.select_to_next_word_boundary(&SelectToNextWordBoundary, cx);
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 2),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.delete_to_next_word_boundary(&DeleteToNextWordBoundary, cx);
            assert_eq!(
                view.display_text(cx),
                "use std::s::{foo, bar}\n\n  {az.qux()}"
            );
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 10),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 3),
                ]
            );
        });

        view.update(cx, |view, cx| {
            view.delete_to_previous_word_boundary(&DeleteToPreviousWordBoundary, cx);
            assert_eq!(
                view.display_text(cx),
                "use std::::{foo, bar}\n\n  az.qux()}"
            );
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
                ]
            );
        });
    }

    #[gpui::test]
    fn test_prev_next_word_bounds_with_soft_wrap(cx: &mut gpui::MutableAppContext) {
        let buffer =
            cx.add_model(|cx| Buffer::new(0, "use one::{\n    two::three::four::five\n};", cx));
        let settings = settings::test(&cx).1;
        let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.set_wrap_width(130., cx);
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
                &[DisplayPoint::new(1, 15)..DisplayPoint::new(1, 15)]
            );
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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

        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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

        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
        let buffer = cx.add_model(|cx| Buffer::new(0, "one two three four five six ", cx));
        let settings = settings::test(&cx).1;
        let view = cx
            .add_window(Default::default(), |cx| {
                build_editor(buffer.clone(), settings, cx)
            })
            .1;

        // Cut with three selections. Clipboard text is divided into three slices.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..4, 8..14, 19..24], false, cx);
            view.cut(&Cut, cx);
            assert_eq!(view.display_text(cx), "two four six ");
        });

        // Paste with three cursors. Each cursor pastes one slice of the clipboard text.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![4..4, 9..9, 13..13], false, cx);
            view.paste(&Paste, cx);
            assert_eq!(view.display_text(cx), "two one four three six five ");
            assert_eq!(
                view.selection_ranges(cx),
                &[
                    DisplayPoint::new(0, 8)..DisplayPoint::new(0, 8),
                    DisplayPoint::new(0, 19)..DisplayPoint::new(0, 19),
                    DisplayPoint::new(0, 28)..DisplayPoint::new(0, 28)
                ]
            );
        });

        // Paste again but with only two cursors. Since the number of cursors doesn't
        // match the number of slices in the clipboard, the entire clipboard text
        // is pasted at each cursor.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..0, 28..28], false, cx);
            view.insert(&Insert("( ".into()), cx);
            view.paste(&Paste, cx);
            view.insert(&Insert(") ".into()), cx);
            assert_eq!(
                view.display_text(cx),
                "( one three five ) two one four three six five ( one three five ) "
            );
        });

        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..0], false, cx);
            view.insert(&Insert("123\n4567\n89\n".into()), cx);
            assert_eq!(
                view.display_text(cx),
                "123\n4567\n89\n( one three five ) two one four three six five ( one three five ) "
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
                "13\n9\n( one three five ) two one four three six five ( one three five ) "
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
                "123\n4567\n9\n( 8ne three five ) two one four three six five ( one three five ) "
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
                "123\n123\n123\n67\n123\n9\n( 8ne three five ) two one four three six five ( one three five ) "
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
        let settings = settings::test(&cx).1;
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
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
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
        let settings = settings::test(&cx).1;
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
        let settings = cx.read(settings::test).1;
        let languages = LanguageRegistry::new();
        let lang = languages.select_language("z.rs");
        let text = r#"
            use mod1::mod2::{mod3, mod4};

            fn fn_1(param1: bool, param2: &str) {
                let var1 = "text";
            }
        "#
        .unindent();
        let buffer = cx.add_model(|cx| {
            let history = History::new(text.into());
            Buffer::from_history(0, history, None, lang.cloned(), cx)
        });
        let (_, view) = cx.add_window(|cx| build_editor(buffer, settings.clone(), cx));
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
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Editor>,
    ) -> Editor {
        Editor::for_buffer(
            buffer,
            settings,
            move |cx| EditorStyle::test(cx.font_cache()),
            cx,
        )
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

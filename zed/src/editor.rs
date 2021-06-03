mod buffer;
pub mod display_map;
mod element;
pub mod movement;

use crate::{
    settings::{Settings, StyleId},
    util::post_inc,
    workspace,
    worktree::FileHandle,
};
use anyhow::Result;
pub use buffer::*;
pub use display_map::DisplayPoint;
use display_map::*;
pub use element::*;
use gpui::{
    color::ColorU, fonts::Properties as FontProperties, geometry::vector::Vector2F,
    keymap::Binding, text_layout, AppContext, ClipboardItem, Element, ElementBox, Entity,
    FontCache, ModelHandle, MutableAppContext, Task, TextLayoutCache, View, ViewContext,
    WeakViewHandle,
};
use parking_lot::Mutex;
use postage::watch;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smol::Timer;
use std::{
    cmp::{self, Ordering},
    fmt::Write,
    iter::FromIterator,
    mem,
    ops::{Range, RangeInclusive},
    path::Path,
    sync::Arc,
    time::Duration,
};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings(vec![
        Binding::new("escape", "buffer:cancel", Some("BufferView")),
        Binding::new("backspace", "buffer:backspace", Some("BufferView")),
        Binding::new("ctrl-h", "buffer:backspace", Some("BufferView")),
        Binding::new("delete", "buffer:delete", Some("BufferView")),
        Binding::new("ctrl-d", "buffer:delete", Some("BufferView")),
        Binding::new("enter", "buffer:newline", Some("BufferView")),
        Binding::new("tab", "buffer:insert", Some("BufferView")).with_arg("\t".to_string()),
        Binding::new("ctrl-shift-K", "buffer:delete_line", Some("BufferView")),
        Binding::new(
            "alt-backspace",
            "buffer:delete_to_previous_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-h",
            "buffer:delete_to_previous_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-delete",
            "buffer:delete_to_next_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-d",
            "buffer:delete_to_next_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-backspace",
            "buffer:delete_to_beginning_of_line",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-delete",
            "buffer:delete_to_end_of_line",
            Some("BufferView"),
        ),
        Binding::new("ctrl-k", "buffer:cut_to_end_of_line", Some("BufferView")),
        Binding::new("cmd-shift-D", "buffer:duplicate_line", Some("BufferView")),
        Binding::new("ctrl-cmd-up", "buffer:move_line_up", Some("BufferView")),
        Binding::new("ctrl-cmd-down", "buffer:move_line_down", Some("BufferView")),
        Binding::new("cmd-x", "buffer:cut", Some("BufferView")),
        Binding::new("cmd-c", "buffer:copy", Some("BufferView")),
        Binding::new("cmd-v", "buffer:paste", Some("BufferView")),
        Binding::new("cmd-z", "buffer:undo", Some("BufferView")),
        Binding::new("cmd-shift-Z", "buffer:redo", Some("BufferView")),
        Binding::new("up", "buffer:move_up", Some("BufferView")),
        Binding::new("down", "buffer:move_down", Some("BufferView")),
        Binding::new("left", "buffer:move_left", Some("BufferView")),
        Binding::new("right", "buffer:move_right", Some("BufferView")),
        Binding::new("ctrl-p", "buffer:move_up", Some("BufferView")),
        Binding::new("ctrl-n", "buffer:move_down", Some("BufferView")),
        Binding::new("ctrl-b", "buffer:move_left", Some("BufferView")),
        Binding::new("ctrl-f", "buffer:move_right", Some("BufferView")),
        Binding::new(
            "alt-left",
            "buffer:move_to_previous_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-b",
            "buffer:move_to_previous_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-right",
            "buffer:move_to_next_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-f",
            "buffer:move_to_next_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-left",
            "buffer:move_to_beginning_of_line",
            Some("BufferView"),
        ),
        Binding::new(
            "ctrl-a",
            "buffer:move_to_beginning_of_line",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-right",
            "buffer:move_to_end_of_line",
            Some("BufferView"),
        ),
        Binding::new("ctrl-e", "buffer:move_to_end_of_line", Some("BufferView")),
        Binding::new("cmd-up", "buffer:move_to_beginning", Some("BufferView")),
        Binding::new("cmd-down", "buffer:move_to_end", Some("BufferView")),
        Binding::new("shift-up", "buffer:select_up", Some("BufferView")),
        Binding::new("ctrl-shift-P", "buffer:select_up", Some("BufferView")),
        Binding::new("shift-down", "buffer:select_down", Some("BufferView")),
        Binding::new("ctrl-shift-N", "buffer:select_down", Some("BufferView")),
        Binding::new("shift-left", "buffer:select_left", Some("BufferView")),
        Binding::new("ctrl-shift-B", "buffer:select_left", Some("BufferView")),
        Binding::new("shift-right", "buffer:select_right", Some("BufferView")),
        Binding::new("ctrl-shift-F", "buffer:select_right", Some("BufferView")),
        Binding::new(
            "alt-shift-left",
            "buffer:select_to_previous_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-shift-B",
            "buffer:select_to_previous_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-shift-right",
            "buffer:select_to_next_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-shift-F",
            "buffer:select_to_next_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-shift-left",
            "buffer:select_to_beginning_of_line",
            Some("BufferView"),
        )
        .with_arg(true),
        Binding::new(
            "ctrl-shift-A",
            "buffer:select_to_beginning_of_line",
            Some("BufferView"),
        )
        .with_arg(true),
        Binding::new(
            "cmd-shift-right",
            "buffer:select_to_end_of_line",
            Some("BufferView"),
        ),
        Binding::new(
            "ctrl-shift-E",
            "buffer:select_to_end_of_line",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-shift-up",
            "buffer:select_to_beginning",
            Some("BufferView"),
        ),
        Binding::new("cmd-shift-down", "buffer:select_to_end", Some("BufferView")),
        Binding::new("cmd-a", "buffer:select_all", Some("BufferView")),
        Binding::new("cmd-l", "buffer:select_line", Some("BufferView")),
        Binding::new(
            "cmd-shift-L",
            "buffer:split_selection_into_lines",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-alt-up",
            "buffer:add_selection_above",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-ctrl-p",
            "buffer:add_selection_above",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-alt-down",
            "buffer:add_selection_below",
            Some("BufferView"),
        ),
        Binding::new(
            "cmd-ctrl-n",
            "buffer:add_selection_below",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-up",
            "buffer:select_larger_syntax_node",
            Some("BufferView"),
        ),
        Binding::new(
            "ctrl-w",
            "buffer:select_larger_syntax_node",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-down",
            "buffer:select_smaller_syntax_node",
            Some("BufferView"),
        ),
        Binding::new(
            "ctrl-shift-W",
            "buffer:select_smaller_syntax_node",
            Some("BufferView"),
        ),
        Binding::new(
            "ctrl-m",
            "buffer:move_to_enclosing_bracket",
            Some("BufferView"),
        ),
        Binding::new("pageup", "buffer:page_up", Some("BufferView")),
        Binding::new("pagedown", "buffer:page_down", Some("BufferView")),
        Binding::new("alt-cmd-[", "buffer:fold", Some("BufferView")),
        Binding::new("alt-cmd-]", "buffer:unfold", Some("BufferView")),
        Binding::new(
            "alt-cmd-f",
            "buffer:fold_selected_ranges",
            Some("BufferView"),
        ),
    ]);

    cx.add_action("buffer:scroll", Editor::scroll);
    cx.add_action("buffer:select", Editor::select);
    cx.add_action("buffer:cancel", Editor::cancel);
    cx.add_action("buffer:insert", Editor::insert);
    cx.add_action("buffer:newline", Editor::newline);
    cx.add_action("buffer:backspace", Editor::backspace);
    cx.add_action("buffer:delete", Editor::delete);
    cx.add_action("buffer:delete_line", Editor::delete_line);
    cx.add_action(
        "buffer:delete_to_previous_word_boundary",
        Editor::delete_to_previous_word_boundary,
    );
    cx.add_action(
        "buffer:delete_to_next_word_boundary",
        Editor::delete_to_next_word_boundary,
    );
    cx.add_action(
        "buffer:delete_to_beginning_of_line",
        Editor::delete_to_beginning_of_line,
    );
    cx.add_action(
        "buffer:delete_to_end_of_line",
        Editor::delete_to_end_of_line,
    );
    cx.add_action("buffer:cut_to_end_of_line", Editor::cut_to_end_of_line);
    cx.add_action("buffer:duplicate_line", Editor::duplicate_line);
    cx.add_action("buffer:move_line_up", Editor::move_line_up);
    cx.add_action("buffer:move_line_down", Editor::move_line_down);
    cx.add_action("buffer:cut", Editor::cut);
    cx.add_action("buffer:copy", Editor::copy);
    cx.add_action("buffer:paste", Editor::paste);
    cx.add_action("buffer:undo", Editor::undo);
    cx.add_action("buffer:redo", Editor::redo);
    cx.add_action("buffer:move_up", Editor::move_up);
    cx.add_action("buffer:move_down", Editor::move_down);
    cx.add_action("buffer:move_left", Editor::move_left);
    cx.add_action("buffer:move_right", Editor::move_right);
    cx.add_action(
        "buffer:move_to_previous_word_boundary",
        Editor::move_to_previous_word_boundary,
    );
    cx.add_action(
        "buffer:move_to_next_word_boundary",
        Editor::move_to_next_word_boundary,
    );
    cx.add_action(
        "buffer:move_to_beginning_of_line",
        Editor::move_to_beginning_of_line,
    );
    cx.add_action("buffer:move_to_end_of_line", Editor::move_to_end_of_line);
    cx.add_action("buffer:move_to_beginning", Editor::move_to_beginning);
    cx.add_action("buffer:move_to_end", Editor::move_to_end);
    cx.add_action("buffer:select_up", Editor::select_up);
    cx.add_action("buffer:select_down", Editor::select_down);
    cx.add_action("buffer:select_left", Editor::select_left);
    cx.add_action("buffer:select_right", Editor::select_right);
    cx.add_action(
        "buffer:select_to_previous_word_boundary",
        Editor::select_to_previous_word_boundary,
    );
    cx.add_action(
        "buffer:select_to_next_word_boundary",
        Editor::select_to_next_word_boundary,
    );
    cx.add_action(
        "buffer:select_to_beginning_of_line",
        Editor::select_to_beginning_of_line,
    );
    cx.add_action(
        "buffer:select_to_end_of_line",
        Editor::select_to_end_of_line,
    );
    cx.add_action("buffer:select_to_beginning", Editor::select_to_beginning);
    cx.add_action("buffer:select_to_end", Editor::select_to_end);
    cx.add_action("buffer:select_all", Editor::select_all);
    cx.add_action("buffer:select_line", Editor::select_line);
    cx.add_action(
        "buffer:split_selection_into_lines",
        Editor::split_selection_into_lines,
    );
    cx.add_action("buffer:add_selection_above", Editor::add_selection_above);
    cx.add_action("buffer:add_selection_below", Editor::add_selection_below);
    cx.add_action(
        "buffer:select_larger_syntax_node",
        Editor::select_larger_syntax_node,
    );
    cx.add_action(
        "buffer:select_smaller_syntax_node",
        Editor::select_smaller_syntax_node,
    );
    cx.add_action(
        "buffer:move_to_enclosing_bracket",
        Editor::move_to_enclosing_bracket,
    );
    cx.add_action("buffer:page_up", Editor::page_up);
    cx.add_action("buffer:page_down", Editor::page_down);
    cx.add_action("buffer:fold", Editor::fold);
    cx.add_action("buffer:unfold", Editor::unfold);
    cx.add_action("buffer:fold_selected_ranges", Editor::fold_selected_ranges);
}

pub enum SelectAction {
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

pub struct Editor {
    handle: WeakViewHandle<Self>,
    buffer: ModelHandle<Buffer>,
    display_map: DisplayMap,
    selection_set_id: SelectionSetId,
    pending_selection: Option<Selection>,
    next_selection_id: usize,
    add_selections_state: Option<AddSelectionsState>,
    select_larger_syntax_node_stack: Vec<Vec<Selection>>,
    scroll_position: Mutex<Vector2F>,
    autoscroll_requested: Mutex<bool>,
    settings: watch::Receiver<Settings>,
    focused: bool,
    cursors_visible: bool,
    blink_epoch: usize,
    blinking_paused: bool,
    single_line: bool,
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
    pub fn single_line(settings: watch::Receiver<Settings>, cx: &mut ViewContext<Self>) -> Self {
        let buffer = cx.add_model(|cx| Buffer::new(0, String::new(), cx));
        let mut view = Self::for_buffer(buffer, settings, cx);
        view.single_line = true;
        view
    }

    pub fn for_buffer(
        buffer: ModelHandle<Buffer>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe_model(&buffer, Self::on_buffer_changed);
        cx.subscribe_to_model(&buffer, Self::on_buffer_event);
        let display_map = DisplayMap::new(buffer.clone(), settings.borrow().tab_size, cx.as_ref());

        let mut next_selection_id = 0;
        let (selection_set_id, _) = buffer.update(cx, |buffer, cx| {
            buffer.add_selection_set(
                vec![Selection {
                    id: post_inc(&mut next_selection_id),
                    start: buffer.anchor_before(0),
                    end: buffer.anchor_before(0),
                    reversed: false,
                    goal: SelectionGoal::None,
                }],
                Some(cx),
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
            scroll_position: Mutex::new(Vector2F::zero()),
            autoscroll_requested: Mutex::new(false),
            settings,
            focused: false,
            cursors_visible: false,
            blink_epoch: 0,
            blinking_paused: false,
            single_line: false,
        }
    }

    pub fn buffer(&self) -> &ModelHandle<Buffer> {
        &self.buffer
    }

    pub fn is_gutter_visible(&self) -> bool {
        !self.single_line
    }

    fn scroll(&mut self, scroll_position: &Vector2F, cx: &mut ViewContext<Self>) {
        *self.scroll_position.lock() = *scroll_position;
        cx.notify();
    }

    pub fn scroll_position(&self) -> Vector2F {
        *self.scroll_position.lock()
    }

    pub fn clamp_scroll_left(&self, max: f32) {
        let mut scroll_position = self.scroll_position.lock();
        let scroll_left = scroll_position.x();
        scroll_position.set_x(scroll_left.min(max));
    }

    pub fn autoscroll_vertically(
        &self,
        viewport_height: f32,
        line_height: f32,
        cx: &AppContext,
    ) -> bool {
        let mut scroll_position = self.scroll_position.lock();
        let scroll_top = scroll_position.y();
        scroll_position.set_y(scroll_top.min(self.max_point(cx).row().saturating_sub(1) as f32));

        let mut autoscroll_requested = self.autoscroll_requested.lock();
        if *autoscroll_requested {
            *autoscroll_requested = false;
        } else {
            return false;
        }

        let visible_lines = viewport_height / line_height;
        let first_cursor_top = self
            .selections(cx)
            .first()
            .unwrap()
            .head()
            .to_display_point(&self.display_map, cx)
            .row() as f32;
        let last_cursor_bottom = self
            .selections(cx)
            .last()
            .unwrap()
            .head()
            .to_display_point(&self.display_map, cx)
            .row() as f32
            + 1.0;

        let margin = ((visible_lines - (last_cursor_bottom - first_cursor_top)) / 2.0)
            .floor()
            .min(3.0);
        if margin < 0.0 {
            return false;
        }

        let target_top = (first_cursor_top - margin).max(0.0);
        let target_bottom = last_cursor_bottom + margin;
        let start_row = scroll_position.y();
        let end_row = start_row + visible_lines;

        if target_top < start_row {
            scroll_position.set_y(target_top);
        } else if target_bottom >= end_row {
            scroll_position.set_y(target_bottom - visible_lines);
        }

        true
    }

    pub fn autoscroll_horizontally(
        &self,
        start_row: u32,
        viewport_width: f32,
        scroll_width: f32,
        max_glyph_width: f32,
        layouts: &[text_layout::Line],
        cx: &AppContext,
    ) {
        let mut target_left = std::f32::INFINITY;
        let mut target_right = 0.0_f32;
        for selection in self.selections(cx) {
            let head = selection.head().to_display_point(&self.display_map, cx);
            let start_column = head.column().saturating_sub(3);
            let end_column = cmp::min(self.display_map.line_len(head.row(), cx), head.column() + 3);
            target_left = target_left
                .min(layouts[(head.row() - start_row) as usize].x_for_index(start_column as usize));
            target_right = target_right.max(
                layouts[(head.row() - start_row) as usize].x_for_index(end_column as usize)
                    + max_glyph_width,
            );
        }
        target_right = target_right.min(scroll_width);

        if target_right - target_left > viewport_width {
            return;
        }

        let mut scroll_position = self.scroll_position.lock();
        let scroll_left = scroll_position.x() * max_glyph_width;
        let scroll_right = scroll_left + viewport_width;

        if target_left < scroll_left {
            scroll_position.set_x(target_left / max_glyph_width);
        } else if target_right > scroll_right {
            scroll_position.set_x((target_right - viewport_width) / max_glyph_width);
        }
    }

    fn select(&mut self, arg: &SelectAction, cx: &mut ViewContext<Self>) {
        match arg {
            SelectAction::Begin { position, add } => self.begin_selection(*position, *add, cx),
            SelectAction::Update {
                position,
                scroll_position,
            } => self.update_selection(*position, *scroll_position, cx),
            SelectAction::End => self.end_selection(cx),
        }
    }

    fn begin_selection(&mut self, position: DisplayPoint, add: bool, cx: &mut ViewContext<Self>) {
        if !self.focused {
            cx.focus_self();
            cx.emit(Event::Activate);
        }

        let cursor = self
            .display_map
            .anchor_before(position, Bias::Left, cx.as_ref());
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
        let buffer = self.buffer.read(cx);
        let cursor = self
            .display_map
            .anchor_before(position, Bias::Left, cx.as_ref());
        if let Some(selection) = self.pending_selection.as_mut() {
            selection.set_head(buffer, cursor);
        } else {
            log::error!("update_selection dispatched with no pending selection");
            return;
        }

        *self.scroll_position.lock() = scroll_position;

        cx.notify();
    }

    fn end_selection(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.pending_selection.take() {
            let ix = self.selection_insertion_index(&selection.start, cx.as_ref());
            let mut selections = self.selections(cx.as_ref()).to_vec();
            selections.insert(ix, selection);
            self.update_selections(selections, false, cx);
        } else {
            log::error!("end_selection dispatched with no pending selection");
        }
    }

    pub fn is_selecting(&self) -> bool {
        self.pending_selection.is_some()
    }

    pub fn cancel(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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
                start: self
                    .display_map
                    .anchor_before(start, Bias::Left, cx.as_ref()),
                end: self.display_map.anchor_before(end, Bias::Left, cx.as_ref()),
                reversed,
                goal: SelectionGoal::None,
            });
        }
        self.update_selections(selections, false, cx);
        Ok(())
    }

    pub fn insert(&mut self, text: &String, cx: &mut ViewContext<Self>) {
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
            buffer.edit(edit_ranges, text.as_str(), Some(cx));
            let text_len = text.len() as isize;
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

    fn newline(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if self.single_line {
            cx.propagate_action();
        } else {
            self.insert(&"\n".into(), cx);
        }
    }

    pub fn backspace(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let range = selection.point_range(buffer);
                if range.start == range.end {
                    let head = selection
                        .head()
                        .to_display_point(&self.display_map, cx.as_ref());
                    let cursor = self.display_map.anchor_before(
                        movement::left(&self.display_map, head, cx.as_ref()).unwrap(),
                        Bias::Left,
                        cx.as_ref(),
                    );
                    selection.set_head(&buffer, cursor);
                    selection.goal = SelectionGoal::None;
                }
            }
        }

        self.update_selections(selections, true, cx);
        self.insert(&String::new(), cx);
        self.end_transaction(cx);
    }

    pub fn delete(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let range = selection.point_range(buffer);
                if range.start == range.end {
                    let head = selection
                        .head()
                        .to_display_point(&self.display_map, cx.as_ref());
                    let cursor = self.display_map.anchor_before(
                        movement::right(&self.display_map, head, cx.as_ref()).unwrap(),
                        Bias::Right,
                        cx.as_ref(),
                    );
                    selection.set_head(&buffer, cursor);
                    selection.goal = SelectionGoal::None;
                }
            }
        }

        self.update_selections(selections, true, cx);
        self.insert(&String::new(), cx);
        self.end_transaction(cx);
    }

    pub fn delete_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

        let app = cx.as_ref();
        let buffer = self.buffer.read(app);

        let mut new_cursors = Vec::new();
        let mut edit_ranges = Vec::new();

        let mut selections = self.selections(app).iter().peekable();
        while let Some(selection) = selections.next() {
            let (mut rows, _) =
                selection.buffer_rows_for_display_rows(false, &self.display_map, app);
            let goal_display_column = selection
                .head()
                .to_display_point(&self.display_map, app)
                .column();

            // Accumulate contiguous regions of rows that we want to delete.
            while let Some(next_selection) = selections.peek() {
                let (next_rows, _) =
                    next_selection.buffer_rows_for_display_rows(false, &self.display_map, app);
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
                Point::new(cursor_buffer_row, 0).to_display_point(&self.display_map, app);
            *cursor.column_mut() = cmp::min(
                goal_display_column,
                self.display_map.line_len(cursor.row(), app),
            );

            new_cursors.push((
                selection.id,
                cursor.to_buffer_point(&self.display_map, Bias::Left, app),
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
            .update(cx, |buffer, cx| buffer.edit(edit_ranges, "", Some(cx)))
            .unwrap();
        self.update_selections(new_selections, true, cx);
        self.end_transaction(cx);
    }

    pub fn duplicate_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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

        let app = cx.as_ref();
        let buffer = self.buffer.read(cx);

        let mut edits = Vec::new();
        let mut selections_iter = selections.iter_mut().peekable();
        while let Some(selection) = selections_iter.next() {
            // Avoid duplicating the same lines twice.
            let (mut rows, _) =
                selection.buffer_rows_for_display_rows(false, &self.display_map, app);
            while let Some(next_selection) = selections_iter.peek() {
                let (next_rows, _) =
                    next_selection.buffer_rows_for_display_rows(false, &self.display_map, app);
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
                buffer.edit(Some(offset..offset), text, Some(cx)).unwrap();
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

    pub fn move_line_up(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

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
            let (mut buffer_rows, mut display_rows) =
                selection.buffer_rows_for_display_rows(false, &self.display_map, app);
            while let Some(next_selection) = selections.peek() {
                let (next_buffer_rows, next_display_rows) =
                    next_selection.buffer_rows_for_display_rows(false, &self.display_map, app);
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
                let prev_row_start =
                    prev_row_display_start.to_buffer_offset(&self.display_map, Bias::Left, app);

                let mut text = String::new();
                text.extend(buffer.text_for_range(start..end));
                text.push('\n');
                edits.push((prev_row_start..prev_row_start, text));
                edits.push((start - 1..end, String::new()));

                let row_delta = buffer_rows.start
                    - prev_row_display_start
                        .to_buffer_point(&self.display_map, Bias::Left, app)
                        .row;

                // Move selections up.
                for range in &mut contiguous_selections {
                    range.start.row -= row_delta;
                    range.end.row -= row_delta;
                }

                // Move folds up.
                old_folds.push(start..end);
                for fold in self.display_map.folds_in_range(start..end, app) {
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
                buffer.edit(Some(range), text, Some(cx)).unwrap();
            }
        });
        self.fold_ranges(new_folds, cx);
        self.select_ranges(new_selection_ranges, true, cx);

        self.end_transaction(cx);
    }

    pub fn move_line_down(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);

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
            let (mut buffer_rows, mut display_rows) =
                selection.buffer_rows_for_display_rows(false, &self.display_map, app);
            while let Some(next_selection) = selections.peek() {
                let (next_buffer_rows, next_display_rows) =
                    next_selection.buffer_rows_for_display_rows(false, &self.display_map, app);
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
            if display_rows.end <= self.display_map.max_point(app).row() {
                let start = Point::new(buffer_rows.start, 0).to_offset(buffer);
                let end = Point::new(buffer_rows.end - 1, buffer.line_len(buffer_rows.end - 1))
                    .to_offset(buffer);

                let next_row_display_end = DisplayPoint::new(
                    display_rows.end,
                    self.display_map.line_len(display_rows.end, app),
                );
                let next_row_end =
                    next_row_display_end.to_buffer_offset(&self.display_map, Bias::Right, app);

                let mut text = String::new();
                text.push('\n');
                text.extend(buffer.text_for_range(start..end));
                edits.push((start..end + 1, String::new()));
                edits.push((next_row_end..next_row_end, text));

                let row_delta = next_row_display_end
                    .to_buffer_point(&self.display_map, Bias::Right, app)
                    .row
                    - buffer_rows.end
                    + 1;

                // Move selections down.
                for range in &mut contiguous_selections {
                    range.start.row += row_delta;
                    range.end.row += row_delta;
                }

                // Move folds down.
                old_folds.push(start..end);
                for fold in self.display_map.folds_in_range(start..end, app) {
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
                buffer.edit(Some(range), text, Some(cx)).unwrap();
            }
        });
        self.fold_ranges(new_folds, cx);
        self.select_ranges(new_selection_ranges, true, cx);

        self.end_transaction(cx);
    }

    pub fn cut(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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
        self.insert(&String::new(), cx);
        self.end_transaction(cx);

        cx.as_mut()
            .write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn copy(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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

    pub fn paste(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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
                        let selection_start = selection.start.to_point(buffer);
                        let selection_end = selection.end.to_point(buffer);

                        // If the corresponding selection was empty when this slice of the
                        // clipboard text was written, then the entire line containing the
                        // selection was copied. If this selection is also currently empty,
                        // then paste the line before the current line of the buffer.
                        let new_selection_start = selection.end.bias_right(buffer);
                        if selection_start == selection_end && clipboard_selection.is_entire_line {
                            let line_start = Point::new(selection_start.row, 0);
                            buffer
                                .edit(Some(line_start..line_start), to_insert, Some(cx))
                                .unwrap();
                        } else {
                            buffer
                                .edit(Some(&selection.start..&selection.end), to_insert, Some(cx))
                                .unwrap();
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
                self.insert(clipboard_text, cx);
            }
        }
    }

    pub fn undo(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.buffer.update(cx, |buffer, cx| buffer.undo(Some(cx)));
    }

    pub fn redo(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.buffer.update(cx, |buffer, cx| buffer.redo(Some(cx)));
    }

    pub fn move_left(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let start = selection.start.to_display_point(&self.display_map, app);
                let end = selection.end.to_display_point(&self.display_map, app);

                if start != end {
                    selection.end = selection.start.clone();
                } else {
                    let cursor = self.display_map.anchor_before(
                        movement::left(&self.display_map, start, app).unwrap(),
                        Bias::Left,
                        app,
                    );
                    selection.start = cursor.clone();
                    selection.end = cursor;
                }
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_left(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, cx.as_ref());
                let cursor = self.display_map.anchor_before(
                    movement::left(&self.display_map, head, cx.as_ref()).unwrap(),
                    Bias::Left,
                    cx.as_ref(),
                );
                selection.set_head(&buffer, cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_right(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let app = cx.as_ref();
            for selection in &mut selections {
                let start = selection.start.to_display_point(&self.display_map, app);
                let end = selection.end.to_display_point(&self.display_map, app);

                if start != end {
                    selection.start = selection.end.clone();
                } else {
                    let cursor = self.display_map.anchor_before(
                        movement::right(&self.display_map, end, app).unwrap(),
                        Bias::Right,
                        app,
                    );
                    selection.start = cursor.clone();
                    selection.end = cursor;
                }
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_right(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let app = cx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, cx.as_ref());
                let cursor = self.display_map.anchor_before(
                    movement::right(&self.display_map, head, app).unwrap(),
                    Bias::Right,
                    app,
                );
                selection.set_head(&buffer, cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_up(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if self.single_line {
            cx.propagate_action();
        } else {
            let mut selections = self.selections(cx.as_ref()).to_vec();
            {
                let app = cx.as_ref();
                for selection in &mut selections {
                    let start = selection.start.to_display_point(&self.display_map, app);
                    let end = selection.end.to_display_point(&self.display_map, app);
                    if start != end {
                        selection.goal = SelectionGoal::None;
                    }

                    let (start, goal) =
                        movement::up(&self.display_map, start, selection.goal, app).unwrap();
                    let cursor = self.display_map.anchor_before(start, Bias::Left, app);
                    selection.start = cursor.clone();
                    selection.end = cursor;
                    selection.goal = goal;
                    selection.reversed = false;
                }
            }
            self.update_selections(selections, true, cx);
        }
    }

    pub fn select_up(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let app = cx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let (head, goal) =
                    movement::up(&self.display_map, head, selection.goal, app).unwrap();
                selection.set_head(
                    &buffer,
                    self.display_map.anchor_before(head, Bias::Left, app),
                );
                selection.goal = goal;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_down(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if self.single_line {
            cx.propagate_action();
        } else {
            let mut selections = self.selections(cx.as_ref()).to_vec();
            {
                let app = cx.as_ref();
                for selection in &mut selections {
                    let start = selection.start.to_display_point(&self.display_map, app);
                    let end = selection.end.to_display_point(&self.display_map, app);
                    if start != end {
                        selection.goal = SelectionGoal::None;
                    }

                    let (start, goal) =
                        movement::down(&self.display_map, end, selection.goal, app).unwrap();
                    let cursor = self.display_map.anchor_before(start, Bias::Right, app);
                    selection.start = cursor.clone();
                    selection.end = cursor;
                    selection.goal = goal;
                    selection.reversed = false;
                }
            }
            self.update_selections(selections, true, cx);
        }
    }

    pub fn select_down(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selections = self.selections(cx.as_ref()).to_vec();
        {
            let app = cx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let (head, goal) =
                    movement::down(&self.display_map, head, selection.goal, app).unwrap();
                selection.set_head(
                    &buffer,
                    self.display_map.anchor_before(head, Bias::Right, app),
                );
                selection.goal = goal;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn move_to_previous_word_boundary(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let new_head = movement::prev_word_boundary(&self.display_map, head, app).unwrap();
                let anchor = self.display_map.anchor_before(new_head, Bias::Left, app);
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_previous_word_boundary(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let new_head = movement::prev_word_boundary(&self.display_map, head, app).unwrap();
                let anchor = self.display_map.anchor_before(new_head, Bias::Left, app);
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_previous_word_boundary(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_previous_word_boundary(&(), cx);
        self.backspace(&(), cx);
        self.end_transaction(cx);
    }

    pub fn move_to_next_word_boundary(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let new_head = movement::next_word_boundary(&self.display_map, head, app).unwrap();
                let anchor = self.display_map.anchor_before(new_head, Bias::Left, app);
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_next_word_boundary(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let new_head = movement::next_word_boundary(&self.display_map, head, app).unwrap();
                let anchor = self.display_map.anchor_before(new_head, Bias::Left, app);
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_next_word_boundary(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_next_word_boundary(&(), cx);
        self.delete(&(), cx);
        self.end_transaction(cx);
    }

    pub fn move_to_beginning_of_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let new_head =
                    movement::line_beginning(&self.display_map, head, true, app).unwrap();
                let anchor = self.display_map.anchor_before(new_head, Bias::Left, app);
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
        toggle_indent: &bool,
        cx: &mut ViewContext<Self>,
    ) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let new_head =
                    movement::line_beginning(&self.display_map, head, *toggle_indent, app).unwrap();
                let anchor = self.display_map.anchor_before(new_head, Bias::Left, app);
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_beginning_of_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_beginning_of_line(&false, cx);
        self.backspace(&(), cx);
        self.end_transaction(cx);
    }

    pub fn move_to_end_of_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let new_head = movement::line_end(&self.display_map, head, app).unwrap();
                let anchor = self.display_map.anchor_before(new_head, Bias::Left, app);
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn select_to_end_of_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            let buffer = self.buffer.read(cx);
            for selection in &mut selections {
                let head = selection.head().to_display_point(&self.display_map, app);
                let new_head = movement::line_end(&self.display_map, head, app).unwrap();
                let anchor = self.display_map.anchor_before(new_head, Bias::Left, app);
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, cx);
    }

    pub fn delete_to_end_of_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_end_of_line(&(), cx);
        self.delete(&(), cx);
        self.end_transaction(cx);
    }

    pub fn cut_to_end_of_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.start_transaction(cx);
        self.select_to_end_of_line(&(), cx);
        self.cut(&(), cx);
        self.end_transaction(cx);
    }

    pub fn move_to_beginning(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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

    pub fn select_to_beginning(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selection = self.selections(cx.as_ref()).last().unwrap().clone();
        selection.set_head(self.buffer.read(cx), Anchor::Start);
        self.update_selections(vec![selection], true, cx);
    }

    pub fn move_to_end(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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

    pub fn select_to_end(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selection = self.selections(cx.as_ref()).last().unwrap().clone();
        selection.set_head(self.buffer.read(cx), Anchor::End);
        self.update_selections(vec![selection], true, cx);
    }

    pub fn select_all(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: Anchor::Start,
            end: Anchor::End,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], false, cx);
    }

    pub fn select_line(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let buffer = self.buffer.read(app);
        let mut selections = self.selections(app).to_vec();
        let max_point = buffer.max_point();
        for selection in &mut selections {
            let (rows, _) = selection.buffer_rows_for_display_rows(true, &self.display_map, app);
            selection.start = buffer.anchor_before(Point::new(rows.start, 0));
            selection.end = buffer.anchor_before(cmp::min(max_point, Point::new(rows.end, 0)));
            selection.reversed = false;
        }
        self.update_selections(selections, true, cx);
    }

    pub fn split_selection_into_lines(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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

    pub fn add_selection_above(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.add_selection(true, cx);
    }

    pub fn add_selection_below(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        self.add_selection(false, cx);
    }

    fn add_selection(&mut self, above: bool, cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();

        let mut selections = self.selections(app).to_vec();
        let mut state = self.add_selections_state.take().unwrap_or_else(|| {
            let oldest_selection = selections.iter().min_by_key(|s| s.id).unwrap().clone();
            let range = oldest_selection
                .display_range(&self.display_map, app)
                .sorted();
            let columns = cmp::min(range.start.column(), range.end.column())
                ..cmp::max(range.start.column(), range.end.column());

            selections.clear();
            let mut stack = Vec::new();
            for row in range.start.row()..=range.end.row() {
                if let Some(selection) =
                    self.build_columnar_selection(row, &columns, oldest_selection.reversed, app)
                {
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
                self.display_map.max_point(app).row()
            };

            'outer: for selection in selections {
                if selection.id == last_added_selection {
                    let range = selection.display_range(&self.display_map, app).sorted();
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

                        if let Some(new_selection) =
                            self.build_columnar_selection(row, &columns, selection.reversed, app)
                        {
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

    pub fn select_larger_syntax_node(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let buffer = self.buffer.read(app);

        let mut stack = mem::take(&mut self.select_larger_syntax_node_stack);
        let mut selected_larger_node = false;
        let old_selections = self.selections(app).to_vec();
        let mut new_selection_ranges = Vec::new();
        for selection in &old_selections {
            let old_range = selection.start.to_offset(buffer)..selection.end.to_offset(buffer);
            let mut new_range = old_range.clone();
            while let Some(containing_range) = buffer.range_for_syntax_ancestor(new_range.clone()) {
                new_range = containing_range;
                if !self.display_map.intersects_fold(new_range.start, app)
                    && !self.display_map.intersects_fold(new_range.end, app)
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

    pub fn select_smaller_syntax_node(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut stack = mem::take(&mut self.select_larger_syntax_node_stack);
        if let Some(selections) = stack.pop() {
            self.update_selections(selections, true, cx);
        }
        self.select_larger_syntax_node_stack = stack;
    }

    pub fn move_to_enclosing_bracket(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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
        row: u32,
        columns: &Range<u32>,
        reversed: bool,
        cx: &AppContext,
    ) -> Option<Selection> {
        let is_empty = columns.start == columns.end;
        let line_len = self.display_map.line_len(row, cx);
        if columns.start < line_len || (is_empty && columns.start == line_len) {
            let start = DisplayPoint::new(row, columns.start);
            let end = DisplayPoint::new(row, cmp::min(columns.end, line_len));
            Some(Selection {
                id: post_inc(&mut self.next_selection_id),
                start: self.display_map.anchor_before(start, Bias::Left, cx),
                end: self.display_map.anchor_before(end, Bias::Left, cx),
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

    pub fn selections_in_range<'a>(
        &'a self,
        range: Range<DisplayPoint>,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = Range<DisplayPoint>> {
        let start = self.display_map.anchor_before(range.start, Bias::Left, cx);
        let start_index = self.selection_insertion_index(&start, cx);
        let pending_selection = self.pending_selection.as_ref().and_then(|s| {
            let selection_range = s.display_range(&self.display_map, cx);
            if selection_range.start <= range.end || selection_range.end <= range.end {
                Some(selection_range)
            } else {
                None
            }
        });
        self.selections(cx)[start_index..]
            .iter()
            .map(move |s| s.display_range(&self.display_map, cx))
            .take_while(move |r| r.start <= range.end || r.end <= range.end)
            .chain(pending_selection)
    }

    fn selection_insertion_index(&self, start: &Anchor, cx: &AppContext) -> usize {
        let buffer = self.buffer.read(cx);
        let selections = self.selections(cx);
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
        self.buffer
            .read(cx)
            .selections(self.selection_set_id)
            .unwrap()
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
                .update_selection_set(self.selection_set_id, selections, Some(cx))
                .unwrap()
        });
        self.pause_cursor_blinking(cx);

        if autoscroll {
            *self.autoscroll_requested.lock() = true;
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
                .end_transaction(Some(self.selection_set_id), Some(cx))
                .unwrap()
        });
    }

    pub fn page_up(&mut self, _: &(), _: &mut ViewContext<Self>) {
        log::info!("BufferView::page_up");
    }

    pub fn page_down(&mut self, _: &(), _: &mut ViewContext<Self>) {
        log::info!("BufferView::page_down");
    }

    pub fn fold(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut fold_ranges = Vec::new();

        let app = cx.as_ref();
        for selection in self.selections(app) {
            let range = selection.display_range(&self.display_map, app).sorted();
            let buffer_start_row = range
                .start
                .to_buffer_point(&self.display_map, Bias::Left, app)
                .row;

            for row in (0..=range.end.row()).rev() {
                if self.is_line_foldable(row, app)
                    && !self.display_map.is_line_folded(row, cx.as_ref())
                {
                    let fold_range = self.foldable_range_for_line(row, app);
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

    pub fn unfold(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let app = cx.as_ref();
        let buffer = self.buffer.read(app);
        let ranges = self
            .selections(app)
            .iter()
            .map(|s| {
                let range = s.display_range(&self.display_map, app).sorted();
                let mut start = range
                    .start
                    .to_buffer_point(&self.display_map, Bias::Left, app);
                let mut end = range
                    .end
                    .to_buffer_point(&self.display_map, Bias::Left, app);
                start.column = 0;
                end.column = buffer.line_len(end.row);
                start..end
            })
            .collect::<Vec<_>>();
        self.unfold_ranges(ranges, cx);
    }

    fn is_line_foldable(&self, display_row: u32, cx: &AppContext) -> bool {
        let max_point = self.max_point(cx);
        if display_row >= max_point.row() {
            false
        } else {
            let (start_indent, is_blank) = self.display_map.line_indent(display_row, cx);
            if is_blank {
                false
            } else {
                for display_row in display_row + 1..=max_point.row() {
                    let (indent, is_blank) = self.display_map.line_indent(display_row, cx);
                    if !is_blank {
                        return indent > start_indent;
                    }
                }
                false
            }
        }
    }

    fn foldable_range_for_line(&self, start_row: u32, cx: &AppContext) -> Range<Point> {
        let max_point = self.max_point(cx);

        let (start_indent, _) = self.display_map.line_indent(start_row, cx);
        let start = DisplayPoint::new(start_row, self.line_len(start_row, cx));
        let mut end = None;
        for row in start_row + 1..=max_point.row() {
            let (indent, is_blank) = self.display_map.line_indent(row, cx);
            if !is_blank && indent <= start_indent {
                end = Some(DisplayPoint::new(row - 1, self.line_len(row - 1, cx)));
                break;
            }
        }

        let end = end.unwrap_or(max_point);
        return start.to_buffer_point(&self.display_map, Bias::Left, cx)
            ..end.to_buffer_point(&self.display_map, Bias::Left, cx);
    }

    pub fn fold_selected_ranges(&mut self, _: &(), cx: &mut ViewContext<Self>) {
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
            self.display_map.fold(ranges, cx.as_ref());
            *self.autoscroll_requested.lock() = true;
            cx.notify();
        }
    }

    fn unfold_ranges<T: ToOffset>(&mut self, ranges: Vec<Range<T>>, cx: &mut ViewContext<Self>) {
        if !ranges.is_empty() {
            self.display_map.unfold(ranges, cx.as_ref());
            *self.autoscroll_requested.lock() = true;
            cx.notify();
        }
    }

    pub fn line(&self, display_row: u32, cx: &AppContext) -> String {
        self.display_map.line(display_row, cx)
    }

    pub fn line_len(&self, display_row: u32, cx: &AppContext) -> u32 {
        self.display_map.line_len(display_row, cx)
    }

    pub fn longest_row(&self, cx: &AppContext) -> u32 {
        self.display_map.longest_row(cx)
    }

    pub fn max_point(&self, cx: &AppContext) -> DisplayPoint {
        self.display_map.max_point(cx)
    }

    pub fn text(&self, cx: &AppContext) -> String {
        self.display_map.text(cx)
    }

    pub fn font_size(&self) -> f32 {
        self.settings.borrow().buffer_font_size
    }

    pub fn font_ascent(&self, font_cache: &FontCache) -> f32 {
        let settings = self.settings.borrow();
        let font_id = font_cache.default_font(settings.buffer_font_family);
        let ascent = font_cache.metric(font_id, |m| m.ascent);
        font_cache.scale_metric(ascent, font_id, settings.buffer_font_size)
    }

    pub fn font_descent(&self, font_cache: &FontCache) -> f32 {
        let settings = self.settings.borrow();
        let font_id = font_cache.default_font(settings.buffer_font_family);
        let ascent = font_cache.metric(font_id, |m| m.descent);
        font_cache.scale_metric(ascent, font_id, settings.buffer_font_size)
    }

    pub fn line_height(&self, font_cache: &FontCache) -> f32 {
        let settings = self.settings.borrow();
        let font_id = font_cache.default_font(settings.buffer_font_family);
        font_cache.line_height(font_id, settings.buffer_font_size)
    }

    pub fn em_width(&self, font_cache: &FontCache) -> f32 {
        let settings = self.settings.borrow();
        let font_id = font_cache.default_font(settings.buffer_font_family);
        font_cache.em_width(font_id, settings.buffer_font_size)
    }

    // TODO: Can we make this not return a result?
    pub fn max_line_number_width(
        &self,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        cx: &AppContext,
    ) -> Result<f32> {
        let settings = self.settings.borrow();
        let font_size = settings.buffer_font_size;
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;
        let digit_count = (self.buffer.read(cx).row_count() as f32).log10().floor() as usize + 1;

        Ok(layout_cache
            .layout_str(
                "1".repeat(digit_count).as_str(),
                font_size,
                &[(digit_count, font_id, ColorU::black())],
            )
            .width())
    }

    pub fn layout_line_numbers(
        &self,
        viewport_height: f32,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        cx: &AppContext,
    ) -> Result<Vec<text_layout::Line>> {
        let settings = self.settings.borrow();
        let font_size = settings.buffer_font_size;
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;

        let start_row = self.scroll_position().y() as usize;
        let end_row = cmp::min(
            self.max_point(cx).row() as usize,
            start_row + (viewport_height / self.line_height(font_cache)).ceil() as usize,
        );
        let line_count = end_row - start_row + 1;

        let mut layouts = Vec::with_capacity(line_count);
        let mut line_number = String::new();
        for buffer_row in self
            .display_map
            .snapshot(cx)
            .buffer_rows(start_row as u32)
            .take(line_count)
        {
            line_number.clear();
            write!(&mut line_number, "{}", buffer_row + 1).unwrap();
            layouts.push(layout_cache.layout_str(
                &line_number,
                font_size,
                &[(line_number.len(), font_id, ColorU::black())],
            ));
        }

        Ok(layouts)
    }

    pub fn layout_lines(
        &self,
        mut rows: Range<u32>,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        cx: &AppContext,
    ) -> Result<Vec<text_layout::Line>> {
        rows.end = cmp::min(rows.end, self.display_map.max_point(cx).row() + 1);
        if rows.start >= rows.end {
            return Ok(Vec::new());
        }

        let settings = self.settings.borrow();
        let font_size = settings.buffer_font_size;
        let font_family = settings.buffer_font_family;
        let mut prev_font_properties = FontProperties::new();
        let mut prev_font_id = font_cache
            .select_font(font_family, &prev_font_properties)
            .unwrap();

        let mut layouts = Vec::with_capacity(rows.len());
        let mut line = String::new();
        let mut styles = Vec::new();
        let mut row = rows.start;
        let mut snapshot = self.display_map.snapshot(cx);
        let chunks = snapshot.highlighted_chunks_for_rows(rows.clone());
        let theme = settings.theme.clone();

        'outer: for (chunk, style_ix) in chunks.chain(Some(("\n", StyleId::default()))) {
            for (ix, line_chunk) in chunk.split('\n').enumerate() {
                if ix > 0 {
                    layouts.push(layout_cache.layout_str(&line, font_size, &styles));
                    line.clear();
                    styles.clear();
                    row += 1;
                    if row == rows.end {
                        break 'outer;
                    }
                }

                if !line_chunk.is_empty() {
                    let (color, font_properties) = theme.syntax_style(style_ix);
                    // Avoid a lookup if the font properties match the previous ones.
                    let font_id = if font_properties == prev_font_properties {
                        prev_font_id
                    } else {
                        font_cache.select_font(font_family, &font_properties)?
                    };
                    line.push_str(line_chunk);
                    styles.push((line_chunk.len(), font_id, color));
                    prev_font_id = font_id;
                    prev_font_properties = font_properties;
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
        cx: &AppContext,
    ) -> Result<text_layout::Line> {
        let settings = self.settings.borrow();
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;

        let line = self.line(row, cx);

        Ok(layout_cache.layout_str(
            &line,
            settings.buffer_font_size,
            &[(self.line_len(row, cx) as usize, font_id, ColorU::black())],
        ))
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    fn pause_cursor_blinking(&mut self, cx: &mut ViewContext<Self>) {
        self.cursors_visible = true;
        cx.notify();

        let epoch = self.next_blink_epoch();
        cx.spawn(|this, mut cx| async move {
            Timer::after(CURSOR_BLINK_INTERVAL).await;
            this.update(&mut cx, |this, cx| {
                this.resume_cursor_blinking(epoch, cx);
            })
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
            cx.spawn(|this, mut cx| async move {
                Timer::after(CURSOR_BLINK_INTERVAL).await;
                this.update(&mut cx, |this, cx| this.blink_cursors(epoch, cx));
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
}

impl View for Editor {
    fn render<'a>(&self, _: &AppContext) -> ElementBox {
        EditorElement::new(self.handle.clone()).boxed()
    }

    fn ui_name() -> &'static str {
        "BufferView"
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.focused = true;
        self.blink_cursors(self.blink_epoch, cx);
    }

    fn on_blur(&mut self, cx: &mut ViewContext<Self>) {
        self.focused = false;
        self.cursors_visible = false;
        cx.emit(Event::Blurred);
        cx.notify();
    }
}

impl workspace::Item for Buffer {
    type View = Editor;

    fn file(&self) -> Option<&FileHandle> {
        self.file()
    }

    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self::View>,
    ) -> Self::View {
        Editor::for_buffer(handle, settings, cx)
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
        let clone = Editor::for_buffer(self.buffer.clone(), self.settings.clone(), cx);
        *clone.scroll_position.lock() = *self.scroll_position.lock();
        Some(clone)
    }

    fn save(
        &mut self,
        new_file: Option<FileHandle>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.buffer.update(cx, |b, cx| b.save(new_file, cx))
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
    use crate::{
        editor::Point,
        settings,
        test::{build_app_state, sample_text},
    };
    use buffer::History;
    use unindent::Unindent;

    #[gpui::test]
    fn test_selection_with_mouse(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, buffer_view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));

        buffer_view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(2, 2), false, cx);
        });

        let view = buffer_view.read(cx);
        let selections = view
            .selections_in_range(
                DisplayPoint::zero()..view.max_point(cx.as_ref()),
                cx.as_ref(),
            )
            .collect::<Vec<_>>();
        assert_eq!(
            selections,
            [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
        );

        buffer_view.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), cx);
        });

        let view = buffer_view.read(cx);
        let selections = view
            .selections_in_range(
                DisplayPoint::zero()..view.max_point(cx.as_ref()),
                cx.as_ref(),
            )
            .collect::<Vec<_>>();
        assert_eq!(
            selections,
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );

        buffer_view.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(1, 1), Vector2F::zero(), cx);
        });

        let view = buffer_view.read(cx);
        let selections = view
            .selections_in_range(
                DisplayPoint::zero()..view.max_point(cx.as_ref()),
                cx.as_ref(),
            )
            .collect::<Vec<_>>();
        assert_eq!(
            selections,
            [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
        );

        buffer_view.update(cx, |view, cx| {
            view.end_selection(cx);
            view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), cx);
        });

        let view = buffer_view.read(cx);
        let selections = view
            .selections_in_range(
                DisplayPoint::zero()..view.max_point(cx.as_ref()),
                cx.as_ref(),
            )
            .collect::<Vec<_>>();
        assert_eq!(
            selections,
            [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
        );

        buffer_view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(3, 3), true, cx);
            view.update_selection(DisplayPoint::new(0, 0), Vector2F::zero(), cx);
        });

        let view = buffer_view.read(cx);
        let selections = view
            .selections_in_range(
                DisplayPoint::zero()..view.max_point(cx.as_ref()),
                cx.as_ref(),
            )
            .collect::<Vec<_>>();
        assert_eq!(
            selections,
            [
                DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)
            ]
        );

        buffer_view.update(cx, |view, cx| {
            view.end_selection(cx);
        });

        let view = buffer_view.read(cx);
        let selections = view
            .selections_in_range(
                DisplayPoint::zero()..view.max_point(cx.as_ref()),
                cx.as_ref(),
            )
            .collect::<Vec<_>>();
        assert_eq!(
            selections,
            [DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)]
        );
    }

    #[gpui::test]
    fn test_canceling_pending_selection(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(2, 2), false, cx);
        });
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
        );

        view.update(cx, |view, cx| {
            view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), cx);
        });
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );

        view.update(cx, |view, cx| {
            view.cancel(&(), cx);
            view.update_selection(DisplayPoint::new(1, 1), Vector2F::zero(), cx);
        });
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );
    }

    #[gpui::test]
    fn test_cancel(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.begin_selection(DisplayPoint::new(3, 4), false, cx);
            view.update_selection(DisplayPoint::new(1, 1), Vector2F::zero(), cx);
            view.end_selection(cx);

            view.begin_selection(DisplayPoint::new(0, 1), true, cx);
            view.update_selection(DisplayPoint::new(0, 3), Vector2F::zero(), cx);
            view.end_selection(cx);
        });
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            [
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1),
            ]
        );

        view.update(cx, |view, cx| view.cancel(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            [DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1)]
        );

        view.update(cx, |view, cx| view.cancel(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            [DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1)]
        );
    }

    #[gpui::test]
    fn test_layout_line_numbers(cx: &mut gpui::MutableAppContext) {
        let layout_cache = TextLayoutCache::new(cx.platform().fonts());
        let font_cache = cx.font_cache().clone();

        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6), cx));

        let settings = settings::channel(&font_cache).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer.clone(), settings, cx));

        let layouts = view
            .read(cx)
            .layout_line_numbers(1000.0, &font_cache, &layout_cache, cx.as_ref())
            .unwrap();
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
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer.clone(), settings, cx));

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(8, 0)..DisplayPoint::new(12, 0)], cx)
                .unwrap();
            view.fold(&(), cx);
            assert_eq!(
                view.text(cx.as_ref()),
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

            view.fold(&(), cx);
            assert_eq!(
                view.text(cx.as_ref()),
                "
                    impl Foo {…
                    }
                "
                .unindent(),
            );

            view.unfold(&(), cx);
            assert_eq!(
                view.text(cx.as_ref()),
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

            view.unfold(&(), cx);
            assert_eq!(view.text(cx.as_ref()), buffer.read(cx).text());
        });
    }

    #[gpui::test]
    fn test_move_cursor(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6), cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer.clone(), settings, cx));

        buffer.update(cx, |buffer, cx| {
            buffer
                .edit(
                    vec![
                        Point::new(1, 0)..Point::new(1, 0),
                        Point::new(1, 1)..Point::new(1, 1),
                    ],
                    "\t",
                    Some(cx),
                )
                .unwrap();
        });

        view.update(cx, |view, cx| {
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.move_down(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );

            view.move_right(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4)]
            );

            view.move_left(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
            );

            view.move_up(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.move_to_end(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(5, 6)..DisplayPoint::new(5, 6)]
            );

            view.move_to_beginning(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
            );

            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2)], cx)
                .unwrap();
            view.select_to_beginning(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 0)]
            );

            view.select_to_end(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[DisplayPoint::new(0, 1)..DisplayPoint::new(5, 6)]
            );
        });
    }

    #[gpui::test]
    fn test_move_cursor_multibyte(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "ⓐⓑⓒⓓⓔ\nabcde\nαβγδε\n", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer.clone(), settings, cx));

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
            assert_eq!(view.text(cx.as_ref()), "ⓐⓑ…ⓔ\nab…e\nαβ…ε\n");

            view.move_right(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(0, "ⓐ".len())]
            );
            view.move_right(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(0, "ⓐⓑ".len())]
            );
            view.move_right(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(0, "ⓐⓑ…".len())]
            );

            view.move_down(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(1, "ab…".len())]
            );
            view.move_left(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(1, "ab".len())]
            );
            view.move_left(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(1, "a".len())]
            );

            view.move_down(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(2, "α".len())]
            );
            view.move_right(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(2, "αβ".len())]
            );
            view.move_right(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(2, "αβ…".len())]
            );
            view.move_right(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(2, "αβ…ε".len())]
            );

            view.move_up(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(1, "ab…e".len())]
            );
            view.move_up(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(0, "ⓐⓑ…ⓔ".len())]
            );
            view.move_left(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(0, "ⓐⓑ…".len())]
            );
            view.move_left(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(0, "ⓐⓑ".len())]
            );
            view.move_left(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(0, "ⓐ".len())]
            );
        });
    }

    #[gpui::test]
    fn test_move_cursor_different_line_lengths(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "ⓐⓑⓒⓓⓔ\nabcd\nαβγ\nabcd\nⓐⓑⓒⓓⓔ\n", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer.clone(), settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[empty_range(0, "ⓐⓑⓒⓓⓔ".len())], cx)
                .unwrap();

            view.move_down(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(1, "abcd".len())]
            );

            view.move_down(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(2, "αβγ".len())]
            );

            view.move_down(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(3, "abcd".len())]
            );

            view.move_down(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(4, "ⓐⓑⓒⓓⓔ".len())]
            );

            view.move_up(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(3, "abcd".len())]
            );

            view.move_up(&(), cx);
            assert_eq!(
                view.selection_ranges(cx.as_ref()),
                &[empty_range(2, "αβγ".len())]
            );
        });
    }

    #[gpui::test]
    fn test_beginning_end_of_line(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\n  def", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
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

        view.update(cx, |view, cx| view.move_to_beginning_of_line(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
            ]
        );

        view.update(cx, |view, cx| view.move_to_beginning_of_line(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
            ]
        );

        view.update(cx, |view, cx| view.move_to_beginning_of_line(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
            ]
        );

        view.update(cx, |view, cx| view.move_to_end_of_line(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
            ]
        );

        // Moving to the end of line again is a no-op.
        view.update(cx, |view, cx| view.move_to_end_of_line(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
            ]
        );

        view.update(cx, |view, cx| {
            view.move_left(&(), cx);
            view.select_to_beginning_of_line(&true, cx);
        });
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
            ]
        );

        view.update(cx, |view, cx| view.select_to_beginning_of_line(&true, cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 0),
            ]
        );

        view.update(cx, |view, cx| view.select_to_beginning_of_line(&true, cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
            ]
        );

        view.update(cx, |view, cx| view.select_to_end_of_line(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 5),
            ]
        );

        view.update(cx, |view, cx| view.delete_to_end_of_line(&(), cx));
        assert_eq!(view.read(cx).text(cx.as_ref()), "ab\n  de");
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
            ]
        );

        view.update(cx, |view, cx| view.delete_to_beginning_of_line(&(), cx));
        assert_eq!(view.read(cx).text(cx.as_ref()), "\n");
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
            ]
        );
    }

    #[gpui::test]
    fn test_prev_next_word_boundary(cx: &mut gpui::MutableAppContext) {
        let buffer =
            cx.add_model(|cx| Buffer::new(0, "use std::str::{foo, bar}\n\n  {baz.qux()}", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
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

        view.update(cx, |view, cx| view.move_to_previous_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                DisplayPoint::new(2, 3)..DisplayPoint::new(2, 3),
            ]
        );

        view.update(cx, |view, cx| view.move_to_previous_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
            ]
        );

        view.update(cx, |view, cx| view.move_to_previous_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 4)..DisplayPoint::new(0, 4),
                DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(cx, |view, cx| view.move_to_previous_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
            ]
        );

        view.update(cx, |view, cx| view.move_to_previous_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(0, 24)..DisplayPoint::new(0, 24),
            ]
        );

        view.update(cx, |view, cx| view.move_to_previous_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(0, 23)..DisplayPoint::new(0, 23),
            ]
        );

        view.update(cx, |view, cx| view.move_to_next_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(0, 24)..DisplayPoint::new(0, 24),
            ]
        );

        view.update(cx, |view, cx| view.move_to_next_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 4)..DisplayPoint::new(0, 4),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
            ]
        );

        view.update(cx, |view, cx| view.move_to_next_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(cx, |view, cx| view.move_to_next_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
            ]
        );

        view.update(cx, |view, cx| {
            view.move_right(&(), cx);
            view.select_to_previous_word_boundary(&(), cx);
        });
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                DisplayPoint::new(2, 3)..DisplayPoint::new(2, 2),
            ]
        );

        view.update(cx, |view, cx| {
            view.select_to_previous_word_boundary(&(), cx)
        });
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 10)..DisplayPoint::new(0, 7),
                DisplayPoint::new(2, 3)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(cx, |view, cx| view.select_to_next_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                DisplayPoint::new(2, 3)..DisplayPoint::new(2, 2),
            ]
        );

        view.update(cx, |view, cx| view.delete_to_next_word_boundary(&(), cx));
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "use std::s::{foo, bar}\n\n  {az.qux()}"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 10)..DisplayPoint::new(0, 10),
                DisplayPoint::new(2, 3)..DisplayPoint::new(2, 3),
            ]
        );

        view.update(cx, |view, cx| {
            view.delete_to_previous_word_boundary(&(), cx)
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "use std::::{foo, bar}\n\n  az.qux()}"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
            ]
        );
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
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer.clone(), settings, cx));

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
            view.backspace(&(), cx);
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
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer.clone(), settings, cx));

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
            view.delete(&(), cx);
        });

        assert_eq!(
            buffer.read(cx).text(),
            "on two three\nfou five six\nseven ten\n"
        );
    }

    #[gpui::test]
    fn test_delete_line(cx: &mut gpui::MutableAppContext) {
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndef\nghi\n", cx));
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
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
            view.delete_line(&(), cx);
        });
        assert_eq!(view.read(cx).text(cx.as_ref()), "ghi");
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)
            ]
        );

        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndef\nghi\n", cx));
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(2, 0)..DisplayPoint::new(0, 1)], cx)
                .unwrap();
            view.delete_line(&(), cx);
        });
        assert_eq!(view.read(cx).text(cx.as_ref()), "ghi\n");
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)]
        );
    }

    #[gpui::test]
    fn test_duplicate_line(cx: &mut gpui::MutableAppContext) {
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndef\nghi\n", cx));
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
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
            view.duplicate_line(&(), cx);
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "abc\nabc\ndef\ndef\nghi\n\n"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                DisplayPoint::new(6, 0)..DisplayPoint::new(6, 0),
            ]
        );

        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndef\nghi\n", cx));
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
        view.update(cx, |view, cx| {
            view.select_display_ranges(
                &[
                    DisplayPoint::new(0, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(2, 1),
                ],
                cx,
            )
            .unwrap();
            view.duplicate_line(&(), cx);
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "abc\ndef\nghi\nabc\ndef\nghi\n"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(3, 1)..DisplayPoint::new(4, 1),
                DisplayPoint::new(4, 2)..DisplayPoint::new(5, 1),
            ]
        );
    }

    #[gpui::test]
    fn test_move_line_up_down(cx: &mut gpui::MutableAppContext) {
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(10, 5), cx));
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
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
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "aa…bbb\nccc…eeee\nfffff\nggggg\n…i\njjjjj"
        );

        view.update(cx, |view, cx| view.move_line_up(&(), cx));
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "aa…bbb\nccc…eeee\nggggg\n…i\njjjjj\nfffff"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3),
                DisplayPoint::new(4, 0)..DisplayPoint::new(4, 2)
            ]
        );

        view.update(cx, |view, cx| view.move_line_down(&(), cx));
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "ccc…eeee\naa…bbb\nfffff\nggggg\n…i\njjjjj"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2)
            ]
        );

        view.update(cx, |view, cx| view.move_line_down(&(), cx));
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "ccc…eeee\nfffff\naa…bbb\nggggg\n…i\njjjjj"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2)
            ]
        );

        view.update(cx, |view, cx| view.move_line_up(&(), cx));
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "ccc…eeee\naa…bbb\nggggg\n…i\njjjjj\nfffff"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3),
                DisplayPoint::new(4, 0)..DisplayPoint::new(4, 2)
            ]
        );
    }

    #[gpui::test]
    fn test_clipboard(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "one two three four five six ", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let view = cx
            .add_window(|cx| Editor::for_buffer(buffer.clone(), settings, cx))
            .1;

        // Cut with three selections. Clipboard text is divided into three slices.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..4, 8..14, 19..24], false, cx);
            view.cut(&(), cx);
        });
        assert_eq!(view.read(cx).text(cx.as_ref()), "two four six ");

        // Paste with three cursors. Each cursor pastes one slice of the clipboard text.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![4..4, 9..9, 13..13], false, cx);
            view.paste(&(), cx);
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "two one four three six five "
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 8)..DisplayPoint::new(0, 8),
                DisplayPoint::new(0, 19)..DisplayPoint::new(0, 19),
                DisplayPoint::new(0, 28)..DisplayPoint::new(0, 28)
            ]
        );

        // Paste again but with only two cursors. Since the number of cursors doesn't
        // match the number of slices in the clipboard, the entire clipboard text
        // is pasted at each cursor.
        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..0, 28..28], false, cx);
            view.insert(&"( ".to_string(), cx);
            view.paste(&(), cx);
            view.insert(&") ".to_string(), cx);
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "( one three five ) two one four three six five ( one three five ) "
        );

        view.update(cx, |view, cx| {
            view.select_ranges(vec![0..0], false, cx);
            view.insert(&"123\n4567\n89\n".to_string(), cx);
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "123\n4567\n89\n( one three five ) two one four three six five ( one three five ) "
        );

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
            view.cut(&(), cx);
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "13\n9\n( one three five ) two one four three six five ( one three five ) "
        );

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
            view.paste(&(), cx);
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "123\n4567\n9\n( 8ne three five ) two one four three six five ( one three five ) "
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                DisplayPoint::new(3, 3)..DisplayPoint::new(3, 3),
            ]
        );

        // Copy with a single cursor only, which writes the whole line into the clipboard.
        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)], cx)
                .unwrap();
            view.copy(&(), cx);
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
            view.paste(&(), cx);
        });
        assert_eq!(
                view.read(cx).text(cx.as_ref()),
                "123\n123\n123\n67\n123\n9\n( 8ne three five ) two one four three six five ( one three five ) "
            );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                DisplayPoint::new(5, 1)..DisplayPoint::new(5, 1),
            ]
        );
    }

    #[gpui::test]
    fn test_select_all(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\nde\nfgh", cx));
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
        view.update(cx, |b, cx| b.select_all(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            &[DisplayPoint::new(0, 0)..DisplayPoint::new(2, 3)]
        );
    }

    #[gpui::test]
    fn test_select_line(cx: &mut gpui::MutableAppContext) {
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(6, 5), cx));
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
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
            view.select_line(&(), cx);
        });
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 0)..DisplayPoint::new(2, 0),
                DisplayPoint::new(4, 0)..DisplayPoint::new(5, 0),
            ]
        );

        view.update(cx, |view, cx| view.select_line(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 0)..DisplayPoint::new(3, 0),
                DisplayPoint::new(4, 0)..DisplayPoint::new(5, 5),
            ]
        );

        view.update(cx, |view, cx| view.select_line(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![DisplayPoint::new(0, 0)..DisplayPoint::new(5, 5)]
        );
    }

    #[gpui::test]
    fn test_split_selection_into_lines(cx: &mut gpui::MutableAppContext) {
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(9, 5), cx));
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));
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
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "aa…bbb\nccc…eeee\nfffff\nggggg\n…i"
        );

        view.update(cx, |view, cx| view.split_selection_into_lines(&(), cx));
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "aa…bbb\nccc…eeee\nfffff\nggggg\n…i"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            [
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                DisplayPoint::new(4, 4)..DisplayPoint::new(4, 4)
            ]
        );

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(4, 0)..DisplayPoint::new(0, 1)], cx)
                .unwrap();
            view.split_selection_into_lines(&(), cx);
        });
        assert_eq!(
            view.read(cx).text(cx.as_ref()),
            "aaaaa\nbbbbb\nccccc\nddddd\neeeee\nfffff\nggggg\n…i"
        );
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
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
    }

    #[gpui::test]
    fn test_add_selection_above_below(cx: &mut gpui::MutableAppContext) {
        let settings = settings::channel(&cx.font_cache()).unwrap().1;
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndefghi\n\njk\nlmno\n", cx));
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, settings, cx));

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)], cx)
                .unwrap();
        });
        view.update(cx, |view, cx| view.add_selection_above(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
            ]
        );

        view.update(cx, |view, cx| view.add_selection_above(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
            ]
        );

        view.update(cx, |view, cx| view.add_selection_below(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)]
        );

        view.update(cx, |view, cx| view.add_selection_below(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
            ]
        );

        view.update(cx, |view, cx| view.add_selection_below(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
            ]
        );

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)], cx)
                .unwrap();
        });
        view.update(cx, |view, cx| view.add_selection_below(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
            ]
        );

        view.update(cx, |view, cx| view.add_selection_below(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
            ]
        );

        view.update(cx, |view, cx| view.add_selection_above(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
        );

        view.update(cx, |view, cx| view.add_selection_above(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
        );

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(0, 1)..DisplayPoint::new(1, 4)], cx)
                .unwrap();
        });
        view.update(cx, |view, cx| view.add_selection_below(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
            ]
        );

        view.update(cx, |view, cx| view.add_selection_below(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
                DisplayPoint::new(4, 1)..DisplayPoint::new(4, 4),
            ]
        );

        view.update(cx, |view, cx| view.add_selection_above(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
            ]
        );

        view.update(cx, |view, cx| {
            view.select_display_ranges(&[DisplayPoint::new(4, 3)..DisplayPoint::new(1, 1)], cx)
                .unwrap();
        });
        view.update(cx, |view, cx| view.add_selection_above(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(3, 1),
                DisplayPoint::new(4, 3)..DisplayPoint::new(4, 1),
            ]
        );

        view.update(cx, |view, cx| view.add_selection_below(&(), cx));
        assert_eq!(
            view.read(cx).selection_ranges(cx.as_ref()),
            vec![
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(3, 1),
                DisplayPoint::new(4, 3)..DisplayPoint::new(4, 1),
            ]
        );
    }

    #[gpui::test]
    async fn test_select_larger_smaller_syntax_node(mut cx: gpui::TestAppContext) {
        let app_state = cx.read(build_app_state);
        let lang = app_state.language_registry.select_language("z.rs");
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
        let (_, view) = cx.add_window(|cx| Editor::for_buffer(buffer, app_state.settings, cx));
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
            view.select_larger_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 23)..DisplayPoint::new(0, 27),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 15)..DisplayPoint::new(3, 21),
            ]
        );

        view.update(&mut cx, |view, cx| {
            view.select_larger_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(&mut cx, |view, cx| {
            view.select_larger_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
        );

        // Trying to expand the selected syntax node one more time has no effect.
        view.update(&mut cx, |view, cx| {
            view.select_larger_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
        );

        view.update(&mut cx, |view, cx| {
            view.select_smaller_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
            ]
        );

        view.update(&mut cx, |view, cx| {
            view.select_smaller_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 23)..DisplayPoint::new(0, 27),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 15)..DisplayPoint::new(3, 21),
            ]
        );

        view.update(&mut cx, |view, cx| {
            view.select_smaller_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
            ]
        );

        // Trying to shrink the selected syntax node one more time has no effect.
        view.update(&mut cx, |view, cx| {
            view.select_smaller_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
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
            view.select_larger_syntax_node(&(), cx);
        });
        assert_eq!(
            view.read_with(&cx, |view, cx| view.selection_ranges(cx)),
            &[
                DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
                DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
                DisplayPoint::new(3, 4)..DisplayPoint::new(3, 23),
            ]
        );
    }

    impl Editor {
        fn selection_ranges(&self, cx: &AppContext) -> Vec<Range<DisplayPoint>> {
            self.selections_in_range(DisplayPoint::zero()..self.max_point(cx), cx)
                .collect::<Vec<_>>()
        }
    }

    fn empty_range(row: usize, column: usize) -> Range<DisplayPoint> {
        let point = DisplayPoint::new(row as u32, column as u32);
        point..point
    }
}

#[derive(Copy, Clone)]
pub enum Bias {
    Left,
    Right,
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

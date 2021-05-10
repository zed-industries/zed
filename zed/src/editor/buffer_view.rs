use super::{
    buffer, movement, Anchor, Bias, Buffer, BufferElement, DisplayMap, DisplayPoint, Point,
    Selection, SelectionGoal, SelectionSetId, ToOffset, ToPoint,
};
use crate::{settings::Settings, util::post_inc, workspace, worktree::FileHandle};
use anyhow::Result;
use futures_core::future::LocalBoxFuture;
use gpui::{
    fonts::Properties as FontProperties, geometry::vector::Vector2F, keymap::Binding, text_layout,
    AppContext, ClipboardItem, Element, ElementBox, Entity, FontCache, ModelHandle,
    MutableAppContext, TextLayoutCache, View, ViewContext, WeakViewHandle,
};
use parking_lot::Mutex;
use postage::watch;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smol::Timer;
use std::{
    cmp::{self, Ordering},
    collections::HashSet,
    fmt::Write,
    iter::FromIterator,
    mem,
    ops::Range,
    path::Path,
    sync::Arc,
    time::Duration,
};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

pub fn init(app: &mut MutableAppContext) {
    app.add_bindings(vec![
        Binding::new("escape", "buffer:cancel", Some("BufferView")),
        Binding::new("backspace", "buffer:backspace", Some("BufferView")),
        Binding::new("ctrl-h", "buffer:backspace", Some("BufferView")),
        Binding::new("delete", "buffer:delete", Some("BufferView")),
        Binding::new("ctrl-d", "buffer:delete", Some("BufferView")),
        Binding::new("enter", "buffer:newline", Some("BufferView")),
        Binding::new("ctrl-shift-K", "buffer:delete_line", Some("BufferView")),
        Binding::new(
            "alt-backspace",
            "buffer:delete_to_previous_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-delete",
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
            "alt-right",
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
        Binding::new("shift-down", "buffer:select_down", Some("BufferView")),
        Binding::new("shift-left", "buffer:select_left", Some("BufferView")),
        Binding::new("shift-right", "buffer:select_right", Some("BufferView")),
        Binding::new(
            "alt-shift-left",
            "buffer:select_to_previous_word_boundary",
            Some("BufferView"),
        ),
        Binding::new(
            "alt-shift-right",
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
            "ctrl-shift-up",
            "buffer:add_cursor_above",
            Some("BufferView"),
        ),
        Binding::new(
            "ctrl-shift-down",
            "buffer:add_cursor_below",
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

    app.add_action("buffer:scroll", BufferView::scroll);
    app.add_action("buffer:select", BufferView::select);
    app.add_action("buffer:cancel", BufferView::cancel);
    app.add_action("buffer:insert", BufferView::insert);
    app.add_action("buffer:newline", BufferView::newline);
    app.add_action("buffer:backspace", BufferView::backspace);
    app.add_action("buffer:delete", BufferView::delete);
    app.add_action("buffer:delete_line", BufferView::delete_line);
    app.add_action(
        "buffer:delete_to_previous_word_boundary",
        BufferView::delete_to_previous_word_boundary,
    );
    app.add_action(
        "buffer:delete_to_next_word_boundary",
        BufferView::delete_to_next_word_boundary,
    );
    app.add_action(
        "buffer:delete_to_beginning_of_line",
        BufferView::delete_to_beginning_of_line,
    );
    app.add_action(
        "buffer:delete_to_end_of_line",
        BufferView::delete_to_end_of_line,
    );
    app.add_action("buffer:duplicate_line", BufferView::duplicate_line);
    app.add_action("buffer:move_line_up", BufferView::move_line_up);
    app.add_action("buffer:move_line_down", BufferView::move_line_down);
    app.add_action("buffer:cut", BufferView::cut);
    app.add_action("buffer:copy", BufferView::copy);
    app.add_action("buffer:paste", BufferView::paste);
    app.add_action("buffer:undo", BufferView::undo);
    app.add_action("buffer:redo", BufferView::redo);
    app.add_action("buffer:move_up", BufferView::move_up);
    app.add_action("buffer:move_down", BufferView::move_down);
    app.add_action("buffer:move_left", BufferView::move_left);
    app.add_action("buffer:move_right", BufferView::move_right);
    app.add_action(
        "buffer:move_to_previous_word_boundary",
        BufferView::move_to_previous_word_boundary,
    );
    app.add_action(
        "buffer:move_to_next_word_boundary",
        BufferView::move_to_next_word_boundary,
    );
    app.add_action(
        "buffer:move_to_beginning_of_line",
        BufferView::move_to_beginning_of_line,
    );
    app.add_action(
        "buffer:move_to_end_of_line",
        BufferView::move_to_end_of_line,
    );
    app.add_action("buffer:move_to_beginning", BufferView::move_to_beginning);
    app.add_action("buffer:move_to_end", BufferView::move_to_end);
    app.add_action("buffer:select_up", BufferView::select_up);
    app.add_action("buffer:select_down", BufferView::select_down);
    app.add_action("buffer:select_left", BufferView::select_left);
    app.add_action("buffer:select_right", BufferView::select_right);
    app.add_action(
        "buffer:select_to_previous_word_boundary",
        BufferView::select_to_previous_word_boundary,
    );
    app.add_action(
        "buffer:select_to_next_word_boundary",
        BufferView::select_to_next_word_boundary,
    );
    app.add_action(
        "buffer:select_to_beginning_of_line",
        BufferView::select_to_beginning_of_line,
    );
    app.add_action(
        "buffer:select_to_end_of_line",
        BufferView::select_to_end_of_line,
    );
    app.add_action(
        "buffer:select_to_beginning",
        BufferView::select_to_beginning,
    );
    app.add_action("buffer:select_to_end", BufferView::select_to_end);
    app.add_action("buffer:select_all", BufferView::select_all);
    app.add_action("buffer:select_line", BufferView::select_line);
    app.add_action(
        "buffer:split_selection_into_lines",
        BufferView::split_selection_into_lines,
    );
    app.add_action("buffer:add_cursor_above", BufferView::add_cursor_above);
    app.add_action("buffer:add_cursor_below", BufferView::add_cursor_below);
    app.add_action("buffer:page_up", BufferView::page_up);
    app.add_action("buffer:page_down", BufferView::page_down);
    app.add_action("buffer:fold", BufferView::fold);
    app.add_action("buffer:unfold", BufferView::unfold);
    app.add_action(
        "buffer:fold_selected_ranges",
        BufferView::fold_selected_ranges,
    );
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

pub struct BufferView {
    handle: WeakViewHandle<Self>,
    buffer: ModelHandle<Buffer>,
    display_map: DisplayMap,
    selection_set_id: SelectionSetId,
    pending_selection: Option<Selection>,
    next_selection_id: usize,
    add_selections_state: Option<AddSelectionsState>,
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
    stack: Vec<HashSet<usize>>,
}

#[derive(Serialize, Deserialize)]
struct ClipboardSelection {
    len: usize,
    is_entire_line: bool,
}

impl BufferView {
    pub fn single_line(settings: watch::Receiver<Settings>, ctx: &mut ViewContext<Self>) -> Self {
        let buffer = ctx.add_model(|ctx| Buffer::new(0, String::new(), ctx));
        let mut view = Self::for_buffer(buffer, settings, ctx);
        view.single_line = true;
        view
    }

    pub fn for_buffer(
        buffer: ModelHandle<Buffer>,
        settings: watch::Receiver<Settings>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        ctx.observe_model(&buffer, Self::on_buffer_changed);
        ctx.subscribe_to_model(&buffer, Self::on_buffer_event);
        let display_map = DisplayMap::new(buffer.clone(), settings.borrow().tab_size, ctx.as_ref());

        let mut next_selection_id = 0;
        let (selection_set_id, _) = buffer.update(ctx, |buffer, ctx| {
            buffer.add_selection_set(
                vec![Selection {
                    id: post_inc(&mut next_selection_id),
                    start: buffer.anchor_before(0).unwrap(),
                    end: buffer.anchor_before(0).unwrap(),
                    reversed: false,
                    goal: SelectionGoal::None,
                }],
                Some(ctx),
            )
        });
        Self {
            handle: ctx.handle().downgrade(),
            buffer,
            display_map,
            selection_set_id,
            pending_selection: None,
            next_selection_id,
            add_selections_state: None,
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

    fn scroll(&mut self, scroll_position: &Vector2F, ctx: &mut ViewContext<Self>) {
        *self.scroll_position.lock() = *scroll_position;
        ctx.notify();
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
        app: &AppContext,
    ) -> bool {
        let mut scroll_position = self.scroll_position.lock();
        let scroll_top = scroll_position.y();
        scroll_position.set_y(scroll_top.min(self.max_point(app).row().saturating_sub(1) as f32));

        let mut autoscroll_requested = self.autoscroll_requested.lock();
        if *autoscroll_requested {
            *autoscroll_requested = false;
        } else {
            return false;
        }

        let visible_lines = viewport_height / line_height;
        let first_cursor_top = self
            .selections(app)
            .first()
            .unwrap()
            .head()
            .to_display_point(&self.display_map, app)
            .unwrap()
            .row() as f32;
        let last_cursor_bottom = self
            .selections(app)
            .last()
            .unwrap()
            .head()
            .to_display_point(&self.display_map, app)
            .unwrap()
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
        layouts: &[Arc<text_layout::Line>],
        ctx: &AppContext,
    ) {
        let mut target_left = std::f32::INFINITY;
        let mut target_right = 0.0_f32;
        for selection in self.selections(ctx) {
            let head = selection
                .head()
                .to_display_point(&self.display_map, ctx)
                .unwrap();
            let start_column = head.column().saturating_sub(3);
            let end_column = cmp::min(
                self.display_map.line_len(head.row(), ctx).unwrap(),
                head.column() + 3,
            );
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

    fn select(&mut self, arg: &SelectAction, ctx: &mut ViewContext<Self>) {
        match arg {
            SelectAction::Begin { position, add } => self.begin_selection(*position, *add, ctx),
            SelectAction::Update {
                position,
                scroll_position,
            } => self.update_selection(*position, *scroll_position, ctx),
            SelectAction::End => self.end_selection(ctx),
        }
    }

    fn begin_selection(&mut self, position: DisplayPoint, add: bool, ctx: &mut ViewContext<Self>) {
        if !self.focused {
            ctx.focus_self();
            ctx.emit(Event::Activate);
        }

        let cursor = self
            .display_map
            .anchor_before(position, Bias::Left, ctx.as_ref())
            .unwrap();
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor.clone(),
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };

        if !add {
            self.update_selections(Vec::new(), false, ctx);
        }
        self.pending_selection = Some(selection);

        ctx.notify();
    }

    fn update_selection(
        &mut self,
        position: DisplayPoint,
        scroll_position: Vector2F,
        ctx: &mut ViewContext<Self>,
    ) {
        let buffer = self.buffer.read(ctx);
        let cursor = self
            .display_map
            .anchor_before(position, Bias::Left, ctx.as_ref())
            .unwrap();
        if let Some(selection) = self.pending_selection.as_mut() {
            selection.set_head(buffer, cursor);
        } else {
            log::error!("update_selection dispatched with no pending selection");
            return;
        }

        *self.scroll_position.lock() = scroll_position;

        ctx.notify();
    }

    fn end_selection(&mut self, ctx: &mut ViewContext<Self>) {
        if let Some(selection) = self.pending_selection.take() {
            let ix = self.selection_insertion_index(&selection.start, ctx.as_ref());
            let mut selections = self.selections(ctx.as_ref()).to_vec();
            selections.insert(ix, selection);
            self.update_selections(selections, false, ctx);
        } else {
            log::error!("end_selection dispatched with no pending selection");
        }
    }

    pub fn is_selecting(&self) -> bool {
        self.pending_selection.is_some()
    }

    pub fn cancel(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let selections = self.selections(ctx.as_ref());
        if let Some(pending_selection) = self.pending_selection.take() {
            if selections.is_empty() {
                self.update_selections(vec![pending_selection], true, ctx);
            }
        } else {
            let mut oldest_selection = selections.iter().min_by_key(|s| s.id).unwrap().clone();
            if selections.len() == 1 {
                oldest_selection.start = oldest_selection.head().clone();
                oldest_selection.end = oldest_selection.head().clone();
            }
            self.update_selections(vec![oldest_selection], true, ctx);
        }
    }

    fn select_ranges<I, T>(&mut self, ranges: I, autoscroll: bool, ctx: &mut ViewContext<Self>)
    where
        I: IntoIterator<Item = Range<T>>,
        T: ToOffset,
    {
        let buffer = self.buffer.read(ctx);
        let mut selections = Vec::new();
        for range in ranges {
            let mut start = range.start.to_offset(buffer).unwrap();
            let mut end = range.end.to_offset(buffer).unwrap();
            let reversed = if start > end {
                mem::swap(&mut start, &mut end);
                true
            } else {
                false
            };
            selections.push(Selection {
                id: post_inc(&mut self.next_selection_id),
                start: buffer.anchor_before(start).unwrap(),
                end: buffer.anchor_before(end).unwrap(),
                reversed,
                goal: SelectionGoal::None,
            });
        }
        self.update_selections(selections, autoscroll, ctx);
    }

    #[cfg(test)]
    fn select_display_ranges<'a, T>(&mut self, ranges: T, ctx: &mut ViewContext<Self>) -> Result<()>
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
                    .anchor_before(start, Bias::Left, ctx.as_ref())?,
                end: self
                    .display_map
                    .anchor_before(end, Bias::Left, ctx.as_ref())?,
                reversed,
                goal: SelectionGoal::None,
            });
        }
        self.update_selections(selections, false, ctx);
        Ok(())
    }

    pub fn insert(&mut self, text: &String, ctx: &mut ViewContext<Self>) {
        let mut old_selections = SmallVec::<[_; 32]>::new();
        {
            let buffer = self.buffer.read(ctx);
            for selection in self.selections(ctx.as_ref()) {
                let start = selection.start.to_offset(buffer).unwrap();
                let end = selection.end.to_offset(buffer).unwrap();
                old_selections.push((selection.id, start..end));
            }
        }

        self.start_transaction(ctx);
        let mut new_selections = Vec::new();
        self.buffer.update(ctx, |buffer, ctx| {
            let edit_ranges = old_selections.iter().map(|(_, range)| range.clone());
            if let Err(error) = buffer.edit(edit_ranges, text.as_str(), Some(ctx)) {
                log::error!("error inserting text: {}", error);
            };
            let char_count = text.chars().count() as isize;
            let mut delta = 0_isize;
            new_selections = old_selections
                .into_iter()
                .map(|(id, range)| {
                    let start = range.start as isize;
                    let end = range.end as isize;
                    let anchor = buffer
                        .anchor_before((start + delta + char_count) as usize)
                        .unwrap();
                    let deleted_count = end - start;
                    delta += char_count - deleted_count;
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

        self.update_selections(new_selections, true, ctx);
        self.end_transaction(ctx);
    }

    fn newline(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if self.single_line {
            ctx.propagate_action();
        } else {
            self.insert(&"\n".into(), ctx);
        }
    }

    pub fn backspace(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);
        let mut selections = self.selections(ctx.as_ref()).to_vec();
        {
            let buffer = self.buffer.read(ctx);
            for selection in &mut selections {
                let range = selection.range(buffer);
                if range.start == range.end {
                    let head = selection
                        .head()
                        .to_display_point(&self.display_map, ctx.as_ref())
                        .unwrap();
                    let cursor = self
                        .display_map
                        .anchor_before(
                            movement::left(&self.display_map, head, ctx.as_ref()).unwrap(),
                            Bias::Left,
                            ctx.as_ref(),
                        )
                        .unwrap();
                    selection.set_head(&buffer, cursor);
                    selection.goal = SelectionGoal::None;
                }
            }
        }

        self.update_selections(selections, true, ctx);
        self.insert(&String::new(), ctx);
        self.end_transaction(ctx);
    }

    pub fn delete(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);
        let mut selections = self.selections(ctx.as_ref()).to_vec();
        {
            let buffer = self.buffer.read(ctx);
            for selection in &mut selections {
                let range = selection.range(buffer);
                if range.start == range.end {
                    let head = selection
                        .head()
                        .to_display_point(&self.display_map, ctx.as_ref())
                        .unwrap();
                    let cursor = self
                        .display_map
                        .anchor_before(
                            movement::right(&self.display_map, head, ctx.as_ref()).unwrap(),
                            Bias::Right,
                            ctx.as_ref(),
                        )
                        .unwrap();
                    selection.set_head(&buffer, cursor);
                    selection.goal = SelectionGoal::None;
                }
            }
        }

        self.update_selections(selections, true, ctx);
        self.insert(&String::new(), ctx);
        self.end_transaction(ctx);
    }

    pub fn delete_line(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);

        let app = ctx.as_ref();
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
                .unwrap()
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

            let mut edit_start = Point::new(rows.start, 0).to_offset(buffer).unwrap();
            let edit_end;
            let cursor_buffer_row;
            if let Ok(end_offset) = Point::new(rows.end, 0).to_offset(buffer) {
                // If there's a line after the range, delete the \n from the end of the row range
                // and position the cursor on the next line.
                edit_end = end_offset;
                cursor_buffer_row = rows.end;
            } else {
                // If there isn't a line after the range, delete the \n from the line before the
                // start of the row range and position the cursor there.
                edit_start = edit_start.saturating_sub(1);
                edit_end = buffer.len();
                cursor_buffer_row = rows.start.saturating_sub(1);
            }

            let mut cursor = Point::new(cursor_buffer_row, 0)
                .to_display_point(&self.display_map, app)
                .unwrap();
            *cursor.column_mut() = cmp::min(
                goal_display_column,
                self.display_map.line_len(cursor.row(), app).unwrap(),
            );

            new_cursors.push((
                selection.id,
                cursor
                    .to_buffer_point(&self.display_map, Bias::Left, app)
                    .unwrap(),
            ));
            edit_ranges.push(edit_start..edit_end);
        }

        new_cursors.sort_unstable_by_key(|(_, range)| range.clone());
        let new_selections = new_cursors
            .into_iter()
            .map(|(id, cursor)| {
                let anchor = buffer.anchor_before(cursor).unwrap();
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
            .update(ctx, |buffer, ctx| buffer.edit(edit_ranges, "", Some(ctx)))
            .unwrap();
        self.update_selections(new_selections, true, ctx);
        self.end_transaction(ctx);
    }

    pub fn duplicate_line(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);

        let mut selections = self.selections(ctx.as_ref()).to_vec();
        {
            // Temporarily bias selections right to allow newly duplicate lines to push them down
            // when the selections are at the beginning of a line.
            let buffer = self.buffer.read(ctx);
            for selection in &mut selections {
                selection.start = selection.start.bias_right(buffer).unwrap();
                selection.end = selection.end.bias_right(buffer).unwrap();
            }
        }
        self.update_selections(selections.clone(), false, ctx);

        let app = ctx.as_ref();
        let buffer = self.buffer.read(ctx);

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
            let end = Point::new(rows.end - 1, buffer.line_len(rows.end - 1).unwrap());
            let text = buffer
                .text_for_range(start..end)
                .unwrap()
                .chain(Some('\n'))
                .collect::<String>();
            edits.push((start, text));
        }

        self.buffer.update(ctx, |buffer, ctx| {
            for (offset, text) in edits.into_iter().rev() {
                buffer.edit(Some(offset..offset), text, Some(ctx)).unwrap();
            }
        });

        // Restore bias on selections.
        let buffer = self.buffer.read(ctx);
        for selection in &mut selections {
            selection.start = selection.start.bias_left(buffer).unwrap();
            selection.end = selection.end.bias_left(buffer).unwrap();
        }
        self.update_selections(selections, true, ctx);

        self.end_transaction(ctx);
    }

    pub fn move_line_up(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);

        let app = ctx.as_ref();
        let buffer = self.buffer.read(ctx);

        let mut edits = Vec::new();
        let mut new_selection_ranges = Vec::new();
        let mut old_folds = Vec::new();
        let mut new_folds = Vec::new();

        let mut selections = self.selections(app).iter().peekable();
        let mut contiguous_selections = Vec::new();
        while let Some(selection) = selections.next() {
            // Accumulate contiguous regions of rows that we want to move.
            contiguous_selections.push(selection.range(buffer));
            let (mut buffer_rows, mut display_rows) =
                selection.buffer_rows_for_display_rows(false, &self.display_map, app);
            while let Some(next_selection) = selections.peek() {
                let (next_buffer_rows, next_display_rows) =
                    next_selection.buffer_rows_for_display_rows(false, &self.display_map, app);
                if next_buffer_rows.start <= buffer_rows.end {
                    buffer_rows.end = next_buffer_rows.end;
                    display_rows.end = next_display_rows.end;
                    contiguous_selections.push(next_selection.range(buffer));
                    selections.next().unwrap();
                } else {
                    break;
                }
            }

            // Cut the text from the selected rows and paste it at the start of the previous line.
            if display_rows.start != 0 {
                let start = Point::new(buffer_rows.start, 0).to_offset(buffer).unwrap();
                let end = Point::new(
                    buffer_rows.end - 1,
                    buffer.line_len(buffer_rows.end - 1).unwrap(),
                )
                .to_offset(buffer)
                .unwrap();

                let prev_row_display_start = DisplayPoint::new(display_rows.start - 1, 0);
                let prev_row_start = prev_row_display_start
                    .to_buffer_offset(&self.display_map, Bias::Left, app)
                    .unwrap();

                let mut text = String::new();
                text.extend(buffer.text_for_range(start..end).unwrap());
                text.push('\n');
                edits.push((prev_row_start..prev_row_start, text));
                edits.push((start - 1..end, String::new()));

                let row_delta = buffer_rows.start
                    - prev_row_display_start
                        .to_buffer_point(&self.display_map, Bias::Left, app)
                        .unwrap()
                        .row;

                // Move selections up.
                for range in &mut contiguous_selections {
                    range.start.row -= row_delta;
                    range.end.row -= row_delta;
                }

                // Move folds up.
                old_folds.push(start..end);
                for fold in self.display_map.folds_in_range(start..end, app).unwrap() {
                    let mut start = fold.start.to_point(buffer).unwrap();
                    let mut end = fold.end.to_point(buffer).unwrap();
                    start.row -= row_delta;
                    end.row -= row_delta;
                    new_folds.push(start..end);
                }
            }

            new_selection_ranges.extend(contiguous_selections.drain(..));
        }

        self.unfold_ranges(old_folds, ctx);
        self.buffer.update(ctx, |buffer, ctx| {
            for (range, text) in edits.into_iter().rev() {
                buffer.edit(Some(range), text, Some(ctx)).unwrap();
            }
        });
        self.fold_ranges(new_folds, ctx);
        self.select_ranges(new_selection_ranges, true, ctx);

        self.end_transaction(ctx);
    }

    pub fn move_line_down(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);

        let app = ctx.as_ref();
        let buffer = self.buffer.read(ctx);

        let mut edits = Vec::new();
        let mut new_selection_ranges = Vec::new();
        let mut old_folds = Vec::new();
        let mut new_folds = Vec::new();

        let mut selections = self.selections(app).iter().peekable();
        let mut contiguous_selections = Vec::new();
        while let Some(selection) = selections.next() {
            // Accumulate contiguous regions of rows that we want to move.
            contiguous_selections.push(selection.range(buffer));
            let (mut buffer_rows, mut display_rows) =
                selection.buffer_rows_for_display_rows(false, &self.display_map, app);
            while let Some(next_selection) = selections.peek() {
                let (next_buffer_rows, next_display_rows) =
                    next_selection.buffer_rows_for_display_rows(false, &self.display_map, app);
                if next_buffer_rows.start <= buffer_rows.end {
                    buffer_rows.end = next_buffer_rows.end;
                    display_rows.end = next_display_rows.end;
                    contiguous_selections.push(next_selection.range(buffer));
                    selections.next().unwrap();
                } else {
                    break;
                }
            }

            // Cut the text from the selected rows and paste it at the end of the next line.
            if display_rows.end <= self.display_map.max_point(app).row() {
                let start = Point::new(buffer_rows.start, 0).to_offset(buffer).unwrap();
                let end = Point::new(
                    buffer_rows.end - 1,
                    buffer.line_len(buffer_rows.end - 1).unwrap(),
                )
                .to_offset(buffer)
                .unwrap();

                let next_row_display_end = DisplayPoint::new(
                    display_rows.end,
                    self.display_map.line_len(display_rows.end, app).unwrap(),
                );
                let next_row_end = next_row_display_end
                    .to_buffer_offset(&self.display_map, Bias::Right, app)
                    .unwrap();

                let mut text = String::new();
                text.push('\n');
                text.extend(buffer.text_for_range(start..end).unwrap());
                edits.push((start..end + 1, String::new()));
                edits.push((next_row_end..next_row_end, text));

                let row_delta = next_row_display_end
                    .to_buffer_point(&self.display_map, Bias::Right, app)
                    .unwrap()
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
                for fold in self.display_map.folds_in_range(start..end, app).unwrap() {
                    let mut start = fold.start.to_point(buffer).unwrap();
                    let mut end = fold.end.to_point(buffer).unwrap();
                    start.row += row_delta;
                    end.row += row_delta;
                    new_folds.push(start..end);
                }
            }

            new_selection_ranges.extend(contiguous_selections.drain(..));
        }

        self.unfold_ranges(old_folds, ctx);
        self.buffer.update(ctx, |buffer, ctx| {
            for (range, text) in edits.into_iter().rev() {
                buffer.edit(Some(range), text, Some(ctx)).unwrap();
            }
        });
        self.fold_ranges(new_folds, ctx);
        self.select_ranges(new_selection_ranges, true, ctx);

        self.end_transaction(ctx);
    }

    pub fn cut(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);
        let mut text = String::new();
        let mut selections = self.selections(ctx.as_ref()).to_vec();
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let buffer = self.buffer.read(ctx);
            let max_point = buffer.max_point();
            for selection in &mut selections {
                let mut start = selection.start.to_point(buffer).expect("invalid start");
                let mut end = selection.end.to_point(buffer).expect("invalid end");
                let is_entire_line = start == end;
                if is_entire_line {
                    start = Point::new(start.row, 0);
                    end = cmp::min(max_point, Point::new(start.row + 1, 0));
                    selection.start = buffer.anchor_before(start).unwrap();
                    selection.end = buffer.anchor_before(end).unwrap();
                }
                let mut len = 0;
                for ch in buffer.text_for_range(start..end).unwrap() {
                    text.push(ch);
                    len += 1;
                }
                clipboard_selections.push(ClipboardSelection {
                    len,
                    is_entire_line,
                });
            }
        }
        self.update_selections(selections, true, ctx);
        self.insert(&String::new(), ctx);
        self.end_transaction(ctx);

        ctx.as_mut()
            .write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn copy(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(ctx);
        let max_point = buffer.max_point();
        let mut text = String::new();
        let selections = self.selections(ctx.as_ref());
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        for selection in selections {
            let mut start = selection.start.to_point(buffer).expect("invalid start");
            let mut end = selection.end.to_point(buffer).expect("invalid end");
            let is_entire_line = start == end;
            if is_entire_line {
                start = Point::new(start.row, 0);
                end = cmp::min(max_point, Point::new(start.row + 1, 0));
            }
            let mut len = 0;
            for ch in buffer.text_for_range(start..end).unwrap() {
                text.push(ch);
                len += 1;
            }
            clipboard_selections.push(ClipboardSelection {
                len,
                is_entire_line,
            });
        }

        ctx.as_mut()
            .write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn paste(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if let Some(item) = ctx.as_mut().read_from_clipboard() {
            let clipboard_text = item.text();
            if let Some(mut clipboard_selections) = item.metadata::<Vec<ClipboardSelection>>() {
                let selections = self.selections(ctx.as_ref()).to_vec();
                if clipboard_selections.len() != selections.len() {
                    let merged_selection = ClipboardSelection {
                        len: clipboard_selections.iter().map(|s| s.len).sum(),
                        is_entire_line: clipboard_selections.iter().all(|s| s.is_entire_line),
                    };
                    clipboard_selections.clear();
                    clipboard_selections.push(merged_selection);
                }

                self.start_transaction(ctx);
                let mut new_selections = Vec::with_capacity(selections.len());
                let mut clipboard_chars = clipboard_text.chars().cycle();
                for (selection, clipboard_selection) in
                    selections.iter().zip(clipboard_selections.iter().cycle())
                {
                    let to_insert =
                        String::from_iter(clipboard_chars.by_ref().take(clipboard_selection.len));

                    self.buffer.update(ctx, |buffer, ctx| {
                        let selection_start = selection.start.to_point(buffer).unwrap();
                        let selection_end = selection.end.to_point(buffer).unwrap();

                        // If the corresponding selection was empty when this slice of the
                        // clipboard text was written, then the entire line containing the
                        // selection was copied. If this selection is also currently empty,
                        // then paste the line before the current line of the buffer.
                        let new_selection_start = selection.end.bias_right(buffer).unwrap();
                        if selection_start == selection_end && clipboard_selection.is_entire_line {
                            let line_start = Point::new(selection_start.row, 0);
                            buffer
                                .edit(Some(line_start..line_start), to_insert, Some(ctx))
                                .unwrap();
                        } else {
                            buffer
                                .edit(Some(&selection.start..&selection.end), to_insert, Some(ctx))
                                .unwrap();
                        };

                        let new_selection_start = new_selection_start.bias_left(buffer).unwrap();
                        new_selections.push(Selection {
                            id: selection.id,
                            start: new_selection_start.clone(),
                            end: new_selection_start,
                            reversed: false,
                            goal: SelectionGoal::None,
                        });
                    });
                }
                self.update_selections(new_selections, true, ctx);
                self.end_transaction(ctx);
            } else {
                self.insert(clipboard_text, ctx);
            }
        }
    }

    pub fn undo(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.buffer
            .update(ctx, |buffer, ctx| buffer.undo(Some(ctx)));
    }

    pub fn redo(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.buffer
            .update(ctx, |buffer, ctx| buffer.redo(Some(ctx)));
    }

    pub fn move_left(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let start = selection
                    .start
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let end = selection
                    .end
                    .to_display_point(&self.display_map, app)
                    .unwrap();

                if start != end {
                    selection.end = selection.start.clone();
                } else {
                    let cursor = self
                        .display_map
                        .anchor_before(
                            movement::left(&self.display_map, start, app).unwrap(),
                            Bias::Left,
                            app,
                        )
                        .unwrap();
                    selection.start = cursor.clone();
                    selection.end = cursor;
                }
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn select_left(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selections = self.selections(ctx.as_ref()).to_vec();
        {
            let buffer = self.buffer.read(ctx);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, ctx.as_ref())
                    .unwrap();
                let cursor = self
                    .display_map
                    .anchor_before(
                        movement::left(&self.display_map, head, ctx.as_ref()).unwrap(),
                        Bias::Left,
                        ctx.as_ref(),
                    )
                    .unwrap();
                selection.set_head(&buffer, cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn move_right(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selections = self.selections(ctx.as_ref()).to_vec();
        {
            let app = ctx.as_ref();
            for selection in &mut selections {
                let start = selection
                    .start
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let end = selection
                    .end
                    .to_display_point(&self.display_map, app)
                    .unwrap();

                if start != end {
                    selection.start = selection.end.clone();
                } else {
                    let cursor = self
                        .display_map
                        .anchor_before(
                            movement::right(&self.display_map, end, app).unwrap(),
                            Bias::Right,
                            app,
                        )
                        .unwrap();
                    selection.start = cursor.clone();
                    selection.end = cursor;
                }
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn select_right(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selections = self.selections(ctx.as_ref()).to_vec();
        {
            let app = ctx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, ctx.as_ref())
                    .unwrap();
                let cursor = self
                    .display_map
                    .anchor_before(
                        movement::right(&self.display_map, head, app).unwrap(),
                        Bias::Right,
                        app,
                    )
                    .unwrap();
                selection.set_head(&buffer, cursor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn move_up(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if self.single_line {
            ctx.propagate_action();
        } else {
            let mut selections = self.selections(ctx.as_ref()).to_vec();
            {
                let app = ctx.as_ref();
                for selection in &mut selections {
                    let start = selection
                        .start
                        .to_display_point(&self.display_map, app)
                        .unwrap();
                    let end = selection
                        .end
                        .to_display_point(&self.display_map, app)
                        .unwrap();
                    if start != end {
                        selection.goal = SelectionGoal::None;
                    }

                    let (start, goal) =
                        movement::up(&self.display_map, start, selection.goal, app).unwrap();
                    let cursor = self
                        .display_map
                        .anchor_before(start, Bias::Left, app)
                        .unwrap();
                    selection.start = cursor.clone();
                    selection.end = cursor;
                    selection.goal = goal;
                    selection.reversed = false;
                }
            }
            self.update_selections(selections, true, ctx);
        }
    }

    pub fn select_up(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selections = self.selections(ctx.as_ref()).to_vec();
        {
            let app = ctx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let (head, goal) =
                    movement::up(&self.display_map, head, selection.goal, app).unwrap();
                selection.set_head(
                    &buffer,
                    self.display_map
                        .anchor_before(head, Bias::Left, app)
                        .unwrap(),
                );
                selection.goal = goal;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn move_down(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if self.single_line {
            ctx.propagate_action();
        } else {
            let mut selections = self.selections(ctx.as_ref()).to_vec();
            {
                let app = ctx.as_ref();
                for selection in &mut selections {
                    let start = selection
                        .start
                        .to_display_point(&self.display_map, app)
                        .unwrap();
                    let end = selection
                        .end
                        .to_display_point(&self.display_map, app)
                        .unwrap();
                    if start != end {
                        selection.goal = SelectionGoal::None;
                    }

                    let (start, goal) =
                        movement::down(&self.display_map, end, selection.goal, app).unwrap();
                    let cursor = self
                        .display_map
                        .anchor_before(start, Bias::Right, app)
                        .unwrap();
                    selection.start = cursor.clone();
                    selection.end = cursor;
                    selection.goal = goal;
                    selection.reversed = false;
                }
            }
            self.update_selections(selections, true, ctx);
        }
    }

    pub fn select_down(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selections = self.selections(ctx.as_ref()).to_vec();
        {
            let app = ctx.as_ref();
            let buffer = self.buffer.read(app);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let (head, goal) =
                    movement::down(&self.display_map, head, selection.goal, app).unwrap();
                selection.set_head(
                    &buffer,
                    self.display_map
                        .anchor_before(head, Bias::Right, app)
                        .unwrap(),
                );
                selection.goal = goal;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn move_to_previous_word_boundary(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let new_head = movement::prev_word_boundary(&self.display_map, head, app).unwrap();
                let anchor = self
                    .display_map
                    .anchor_before(new_head, Bias::Left, app)
                    .unwrap();
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn select_to_previous_word_boundary(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            let buffer = self.buffer.read(ctx);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let new_head = movement::prev_word_boundary(&self.display_map, head, app).unwrap();
                let anchor = self
                    .display_map
                    .anchor_before(new_head, Bias::Left, app)
                    .unwrap();
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn delete_to_previous_word_boundary(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);
        self.select_to_previous_word_boundary(&(), ctx);
        self.backspace(&(), ctx);
        self.end_transaction(ctx);
    }

    pub fn move_to_next_word_boundary(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let new_head = movement::next_word_boundary(&self.display_map, head, app).unwrap();
                let anchor = self
                    .display_map
                    .anchor_before(new_head, Bias::Left, app)
                    .unwrap();
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn select_to_next_word_boundary(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            let buffer = self.buffer.read(ctx);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let new_head = movement::next_word_boundary(&self.display_map, head, app).unwrap();
                let anchor = self
                    .display_map
                    .anchor_before(new_head, Bias::Left, app)
                    .unwrap();
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn delete_to_next_word_boundary(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);
        self.select_to_next_word_boundary(&(), ctx);
        self.delete(&(), ctx);
        self.end_transaction(ctx);
    }

    pub fn move_to_beginning_of_line(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let new_head =
                    movement::line_beginning(&self.display_map, head, true, app).unwrap();
                let anchor = self
                    .display_map
                    .anchor_before(new_head, Bias::Left, app)
                    .unwrap();
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn select_to_beginning_of_line(
        &mut self,
        toggle_indent: &bool,
        ctx: &mut ViewContext<Self>,
    ) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            let buffer = self.buffer.read(ctx);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let new_head =
                    movement::line_beginning(&self.display_map, head, *toggle_indent, app).unwrap();
                let anchor = self
                    .display_map
                    .anchor_before(new_head, Bias::Left, app)
                    .unwrap();
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn delete_to_beginning_of_line(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);
        self.select_to_beginning_of_line(&false, ctx);
        self.backspace(&(), ctx);
        self.end_transaction(ctx);
    }

    pub fn move_to_end_of_line(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let new_head = movement::line_end(&self.display_map, head, app).unwrap();
                let anchor = self
                    .display_map
                    .anchor_before(new_head, Bias::Left, app)
                    .unwrap();
                selection.start = anchor.clone();
                selection.end = anchor;
                selection.reversed = false;
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn select_to_end_of_line(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let mut selections = self.selections(app).to_vec();
        {
            let buffer = self.buffer.read(ctx);
            for selection in &mut selections {
                let head = selection
                    .head()
                    .to_display_point(&self.display_map, app)
                    .unwrap();
                let new_head = movement::line_end(&self.display_map, head, app).unwrap();
                let anchor = self
                    .display_map
                    .anchor_before(new_head, Bias::Left, app)
                    .unwrap();
                selection.set_head(buffer, anchor);
                selection.goal = SelectionGoal::None;
            }
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn delete_to_end_of_line(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.start_transaction(ctx);
        self.select_to_end_of_line(&(), ctx);
        self.delete(&(), ctx);
        self.end_transaction(ctx);
    }

    pub fn move_to_beginning(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(ctx);
        let cursor = buffer.anchor_before(Point::new(0, 0)).unwrap();
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor.clone(),
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], true, ctx);
    }

    pub fn select_to_beginning(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selection = self.selections(ctx.as_ref()).last().unwrap().clone();
        selection.set_head(self.buffer.read(ctx), Anchor::Start);
        self.update_selections(vec![selection], true, ctx);
    }

    pub fn move_to_end(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(ctx);
        let cursor = buffer.anchor_before(buffer.max_point()).unwrap();
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: cursor.clone(),
            end: cursor,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], true, ctx);
    }

    pub fn select_to_end(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selection = self.selections(ctx.as_ref()).last().unwrap().clone();
        selection.set_head(self.buffer.read(ctx), Anchor::End);
        self.update_selections(vec![selection], true, ctx);
    }

    pub fn select_all(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let selection = Selection {
            id: post_inc(&mut self.next_selection_id),
            start: Anchor::Start,
            end: Anchor::End,
            reversed: false,
            goal: SelectionGoal::None,
        };
        self.update_selections(vec![selection], false, ctx);
    }

    pub fn select_line(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let app = ctx.as_ref();
        let buffer = self.buffer.read(app);
        let mut selections = self.selections(app).to_vec();
        let max_point = buffer.max_point();
        for selection in &mut selections {
            let (rows, _) = selection.buffer_rows_for_display_rows(true, &self.display_map, app);
            selection.start = buffer.anchor_before(Point::new(rows.start, 0)).unwrap();
            selection.end = buffer
                .anchor_before(cmp::min(max_point, Point::new(rows.end, 0)))
                .unwrap();
            selection.reversed = false;
        }
        self.update_selections(selections, true, ctx);
    }

    pub fn split_selection_into_lines(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        use super::RangeExt;

        let app = ctx.as_ref();
        let buffer = self.buffer.read(app);

        let mut to_unfold = Vec::new();
        let mut new_selections = Vec::new();
        for selection in self.selections(app) {
            let range = selection.range(buffer).sorted();
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
                let cursor = buffer
                    .anchor_before(Point::new(row, buffer.line_len(row).unwrap()))
                    .unwrap();
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
        self.unfold_ranges(to_unfold, ctx);
        self.update_selections(new_selections, true, ctx);
    }

    pub fn add_cursor_above(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.add_cursor(true, ctx);
    }

    pub fn add_cursor_below(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.add_cursor(false, ctx);
    }

    pub fn add_cursor(&mut self, above: bool, ctx: &mut ViewContext<Self>) {
        use super::RangeExt;

        let app = ctx.as_ref();
        let buffer = self.buffer.read(app);
        let selections = self.selections(app);
        let mut state = self
            .add_selections_state
            .take()
            .unwrap_or_else(|| AddSelectionsState {
                above,
                stack: vec![selections.iter().map(|s| s.id).collect()],
            });

        let last_added_selections = state.stack.last().unwrap();
        let mut new_selections = Vec::new();
        if above == state.above {
            let mut added_selections = HashSet::new();
            for selection in selections {
                if last_added_selections.contains(&selection.id) {
                    let range = selection.display_range(&self.display_map, app).sorted();

                    let mut row = range.start.row();
                    let start_column;
                    let end_column;
                    if let SelectionGoal::ColumnRange { start, end } = selection.goal {
                        start_column = start;
                        end_column = end;
                    } else {
                        start_column = cmp::min(range.start.column(), range.end.column());
                        end_column = cmp::max(range.start.column(), range.end.column());
                    }
                    let is_empty = start_column == end_column;

                    while row > 0 && row < self.display_map.max_point(app).row() {
                        if above {
                            row -= 1;
                        } else {
                            row += 1;
                        }

                        let line_len = self.display_map.line_len(row, app).unwrap();
                        if start_column < line_len || (is_empty && start_column == line_len) {
                            let id = post_inc(&mut self.next_selection_id);
                            let start = DisplayPoint::new(row, start_column);
                            let end = DisplayPoint::new(row, cmp::min(end_column, line_len));
                            new_selections.push(Selection {
                                id,
                                start: self
                                    .display_map
                                    .anchor_before(start, Bias::Left, app)
                                    .unwrap(),
                                end: self
                                    .display_map
                                    .anchor_before(end, Bias::Left, app)
                                    .unwrap(),
                                reversed: selection.reversed
                                    && range.start.row() == range.end.row(),
                                goal: SelectionGoal::ColumnRange {
                                    start: start_column,
                                    end: end_column,
                                },
                            });
                            added_selections.insert(id);
                            break;
                        }
                    }
                }

                new_selections.push(selection.clone());
            }

            if !added_selections.is_empty() {
                state.stack.push(added_selections);
            }
            new_selections.sort_unstable_by(|a, b| a.start.cmp(&b.start, buffer).unwrap());
        } else {
            new_selections.extend(
                selections
                    .into_iter()
                    .filter(|s| !last_added_selections.contains(&s.id))
                    .cloned(),
            );
            state.stack.pop();
        }

        self.update_selections(new_selections, true, ctx);
        if state.stack.len() > 1 {
            self.add_selections_state = Some(state);
        }
    }

    pub fn selections_in_range<'a>(
        &'a self,
        range: Range<DisplayPoint>,
        app: &'a AppContext,
    ) -> impl 'a + Iterator<Item = Range<DisplayPoint>> {
        let start = self
            .display_map
            .anchor_before(range.start, Bias::Left, app)
            .unwrap();
        let start_index = self.selection_insertion_index(&start, app);
        let pending_selection = self.pending_selection.as_ref().and_then(|s| {
            let selection_range = s.display_range(&self.display_map, app);
            if selection_range.start <= range.end || selection_range.end <= range.end {
                Some(selection_range)
            } else {
                None
            }
        });
        self.selections(app)[start_index..]
            .iter()
            .map(move |s| s.display_range(&self.display_map, app))
            .take_while(move |r| r.start <= range.end || r.end <= range.end)
            .chain(pending_selection)
    }

    fn selection_insertion_index(&self, start: &Anchor, app: &AppContext) -> usize {
        let buffer = self.buffer.read(app);
        let selections = self.selections(app);
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

    fn selections<'a>(&self, app: &'a AppContext) -> &'a [Selection] {
        self.buffer
            .read(app)
            .selections(self.selection_set_id)
            .unwrap()
    }

    fn update_selections(
        &mut self,
        mut selections: Vec<Selection>,
        autoscroll: bool,
        ctx: &mut ViewContext<Self>,
    ) {
        // Merge overlapping selections.
        let buffer = self.buffer.read(ctx);
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

        self.buffer.update(ctx, |buffer, ctx| {
            buffer
                .update_selection_set(self.selection_set_id, selections, Some(ctx))
                .unwrap()
        });
        self.pause_cursor_blinking(ctx);

        if autoscroll {
            *self.autoscroll_requested.lock() = true;
            ctx.notify();
        }

        self.add_selections_state = None;
    }

    fn start_transaction(&self, ctx: &mut ViewContext<Self>) {
        self.buffer.update(ctx, |buffer, _| {
            buffer
                .start_transaction(Some(self.selection_set_id))
                .unwrap()
        });
    }

    fn end_transaction(&self, ctx: &mut ViewContext<Self>) {
        self.buffer.update(ctx, |buffer, ctx| {
            buffer
                .end_transaction(Some(self.selection_set_id), Some(ctx))
                .unwrap()
        });
    }

    pub fn page_up(&mut self, _: &(), _: &mut ViewContext<Self>) {
        log::info!("BufferView::page_up");
    }

    pub fn page_down(&mut self, _: &(), _: &mut ViewContext<Self>) {
        log::info!("BufferView::page_down");
    }

    pub fn fold(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        use super::RangeExt;

        let mut fold_ranges = Vec::new();

        let app = ctx.as_ref();
        for selection in self.selections(app) {
            let range = selection.display_range(&self.display_map, app).sorted();
            let buffer_start_row = range
                .start
                .to_buffer_point(&self.display_map, Bias::Left, app)
                .unwrap()
                .row;

            for row in (0..=range.end.row()).rev() {
                if self.is_line_foldable(row, app)
                    && !self.display_map.is_line_folded(row, ctx.as_ref())
                {
                    let fold_range = self.foldable_range_for_line(row, app).unwrap();
                    if fold_range.end.row >= buffer_start_row {
                        fold_ranges.push(fold_range);
                        if row <= range.start.row() {
                            break;
                        }
                    }
                }
            }
        }

        self.fold_ranges(fold_ranges, ctx);
    }

    pub fn unfold(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        use super::RangeExt;

        let app = ctx.as_ref();
        let buffer = self.buffer.read(app);
        let ranges = self
            .selections(app)
            .iter()
            .map(|s| {
                let range = s.display_range(&self.display_map, app).sorted();
                let mut start = range
                    .start
                    .to_buffer_point(&self.display_map, Bias::Left, app)
                    .unwrap();
                let mut end = range
                    .end
                    .to_buffer_point(&self.display_map, Bias::Left, app)
                    .unwrap();
                start.column = 0;
                end.column = buffer.line_len(end.row).unwrap();
                start..end
            })
            .collect::<Vec<_>>();
        self.unfold_ranges(ranges, ctx);
    }

    fn is_line_foldable(&self, display_row: u32, app: &AppContext) -> bool {
        let max_point = self.max_point(app);
        if display_row >= max_point.row() {
            false
        } else {
            let (start_indent, is_blank) = self.display_map.line_indent(display_row, app).unwrap();
            if is_blank {
                false
            } else {
                for display_row in display_row + 1..=max_point.row() {
                    let (indent, is_blank) =
                        self.display_map.line_indent(display_row, app).unwrap();
                    if !is_blank {
                        return indent > start_indent;
                    }
                }
                false
            }
        }
    }

    fn foldable_range_for_line(&self, start_row: u32, app: &AppContext) -> Result<Range<Point>> {
        let max_point = self.max_point(app);

        let (start_indent, _) = self.display_map.line_indent(start_row, app)?;
        let start = DisplayPoint::new(start_row, self.line_len(start_row, app)?);
        let mut end = None;
        for row in start_row + 1..=max_point.row() {
            let (indent, is_blank) = self.display_map.line_indent(row, app)?;
            if !is_blank && indent <= start_indent {
                end = Some(DisplayPoint::new(row - 1, self.line_len(row - 1, app)?));
                break;
            }
        }

        let end = end.unwrap_or(max_point);
        return Ok(start.to_buffer_point(&self.display_map, Bias::Left, app)?
            ..end.to_buffer_point(&self.display_map, Bias::Left, app)?);
    }

    pub fn fold_selected_ranges(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        use super::RangeExt;

        let buffer = self.buffer.read(ctx);
        let ranges = self
            .selections(ctx.as_ref())
            .iter()
            .map(|s| s.range(buffer).sorted())
            .collect();
        self.fold_ranges(ranges, ctx);
    }

    fn fold_ranges<T: ToOffset>(&mut self, ranges: Vec<Range<T>>, ctx: &mut ViewContext<Self>) {
        if !ranges.is_empty() {
            self.display_map.fold(ranges, ctx.as_ref()).unwrap();
            *self.autoscroll_requested.lock() = true;
            ctx.notify();
        }
    }

    fn unfold_ranges<T: ToOffset>(&mut self, ranges: Vec<Range<T>>, ctx: &mut ViewContext<Self>) {
        if !ranges.is_empty() {
            self.display_map.unfold(ranges, ctx.as_ref()).unwrap();
            *self.autoscroll_requested.lock() = true;
            ctx.notify();
        }
    }

    pub fn line(&self, display_row: u32, ctx: &AppContext) -> Result<String> {
        self.display_map.line(display_row, ctx)
    }

    pub fn line_len(&self, display_row: u32, ctx: &AppContext) -> Result<u32> {
        self.display_map.line_len(display_row, ctx)
    }

    pub fn rightmost_point(&self, ctx: &AppContext) -> DisplayPoint {
        self.display_map.rightmost_point(ctx)
    }

    pub fn max_point(&self, ctx: &AppContext) -> DisplayPoint {
        self.display_map.max_point(ctx)
    }

    pub fn text(&self, ctx: &AppContext) -> String {
        self.display_map.text(ctx)
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
        app: &AppContext,
    ) -> Result<f32> {
        let settings = self.settings.borrow();
        let font_size = settings.buffer_font_size;
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;
        let digit_count = ((self.buffer.read(app).max_point().row + 1) as f32)
            .log10()
            .floor() as usize
            + 1;

        Ok(layout_cache
            .layout_str(
                "1".repeat(digit_count).as_str(),
                font_size,
                &[(0..digit_count, font_id)],
            )
            .width)
    }

    pub fn layout_line_numbers(
        &self,
        viewport_height: f32,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        ctx: &AppContext,
    ) -> Result<Vec<Arc<text_layout::Line>>> {
        let settings = self.settings.borrow();
        let font_size = settings.buffer_font_size;
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;

        let start_row = self.scroll_position().y() as usize;
        let end_row = cmp::min(
            self.max_point(ctx).row() as usize,
            start_row + (viewport_height / self.line_height(font_cache)).ceil() as usize,
        );
        let line_count = end_row - start_row + 1;

        let mut layouts = Vec::with_capacity(line_count);
        let mut line_number = String::new();
        for buffer_row in self
            .display_map
            .snapshot(ctx)
            .buffer_rows(start_row as u32)?
            .take(line_count)
        {
            line_number.clear();
            write!(&mut line_number, "{}", buffer_row + 1).unwrap();
            layouts.push(layout_cache.layout_str(
                &line_number,
                font_size,
                &[(0..line_number.len(), font_id)],
            ));
        }

        Ok(layouts)
    }

    pub fn layout_lines(
        &self,
        mut rows: Range<u32>,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        ctx: &AppContext,
    ) -> Result<Vec<Arc<text_layout::Line>>> {
        rows.end = cmp::min(rows.end, self.display_map.max_point(ctx).row() + 1);
        if rows.start >= rows.end {
            return Ok(Vec::new());
        }

        let settings = self.settings.borrow();
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;
        let font_size = settings.buffer_font_size;

        let mut layouts = Vec::with_capacity(rows.len());
        let mut line = String::new();
        let mut line_len = 0;
        let mut row = rows.start;
        let snapshot = self.display_map.snapshot(ctx);
        let chars = snapshot
            .chars_at(DisplayPoint::new(rows.start, 0), ctx)
            .unwrap();
        for char in chars.chain(Some('\n')) {
            if char == '\n' {
                layouts.push(layout_cache.layout_str(&line, font_size, &[(0..line_len, font_id)]));
                line.clear();
                line_len = 0;
                row += 1;
                if row == rows.end {
                    break;
                }
            } else {
                line_len += 1;
                line.push(char);
            }
        }

        Ok(layouts)
    }

    pub fn layout_line(
        &self,
        row: u32,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        app: &AppContext,
    ) -> Result<Arc<text_layout::Line>> {
        let settings = self.settings.borrow();
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;

        let line = self.line(row, app)?;

        Ok(layout_cache.layout_str(
            &line,
            settings.buffer_font_size,
            &[(0..self.line_len(row, app)? as usize, font_id)],
        ))
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    fn pause_cursor_blinking(&mut self, ctx: &mut ViewContext<Self>) {
        self.cursors_visible = true;
        ctx.notify();

        let epoch = self.next_blink_epoch();
        ctx.spawn(
            async move {
                Timer::after(CURSOR_BLINK_INTERVAL).await;
                epoch
            },
            Self::resume_cursor_blinking,
        )
        .detach();
    }

    fn resume_cursor_blinking(&mut self, epoch: usize, ctx: &mut ViewContext<Self>) {
        if epoch == self.blink_epoch {
            self.blinking_paused = false;
            self.blink_cursors(epoch, ctx);
        }
    }

    fn blink_cursors(&mut self, epoch: usize, ctx: &mut ViewContext<Self>) {
        if epoch == self.blink_epoch && self.focused && !self.blinking_paused {
            self.cursors_visible = !self.cursors_visible;
            ctx.notify();

            let epoch = self.next_blink_epoch();
            ctx.spawn(
                async move {
                    Timer::after(CURSOR_BLINK_INTERVAL).await;
                    epoch
                },
                Self::blink_cursors,
            )
            .detach();
        }
    }

    pub fn cursors_visible(&self) -> bool {
        self.cursors_visible
    }

    fn on_buffer_changed(&mut self, _: ModelHandle<Buffer>, ctx: &mut ViewContext<Self>) {
        ctx.notify();
    }

    fn on_buffer_event(
        &mut self,
        _: ModelHandle<Buffer>,
        event: &buffer::Event,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            buffer::Event::Edited => ctx.emit(Event::Edited),
            buffer::Event::Dirtied => ctx.emit(Event::Dirtied),
            buffer::Event::Saved => ctx.emit(Event::Saved),
            buffer::Event::FileHandleChanged => ctx.emit(Event::FileHandleChanged),
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

impl Entity for BufferView {
    type Event = Event;
}

impl View for BufferView {
    fn render<'a>(&self, _: &AppContext) -> ElementBox {
        BufferElement::new(self.handle.clone()).boxed()
    }

    fn ui_name() -> &'static str {
        "BufferView"
    }

    fn on_focus(&mut self, ctx: &mut ViewContext<Self>) {
        self.focused = true;
        self.blink_cursors(self.blink_epoch, ctx);
    }

    fn on_blur(&mut self, ctx: &mut ViewContext<Self>) {
        self.focused = false;
        self.cursors_visible = false;
        ctx.emit(Event::Blurred);
        ctx.notify();
    }
}

impl workspace::Item for Buffer {
    type View = BufferView;

    fn file(&self) -> Option<&FileHandle> {
        self.file()
    }

    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        ctx: &mut ViewContext<Self::View>,
    ) -> Self::View {
        BufferView::for_buffer(handle, settings, ctx)
    }
}

impl workspace::ItemView for BufferView {
    fn should_activate_item_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::Activate)
    }

    fn should_update_tab_on_event(event: &Self::Event) -> bool {
        matches!(
            event,
            Event::Saved | Event::Dirtied | Event::FileHandleChanged
        )
    }

    fn title(&self, app: &AppContext) -> std::string::String {
        let filename = self
            .buffer
            .read(app)
            .file()
            .and_then(|file| file.file_name(app));
        if let Some(name) = filename {
            name.to_string_lossy().into()
        } else {
            "untitled".into()
        }
    }

    fn entry_id(&self, ctx: &AppContext) -> Option<(usize, Arc<Path>)> {
        self.buffer.read(ctx).file().map(|file| file.entry_id())
    }

    fn clone_on_split(&self, ctx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        let clone = BufferView::for_buffer(self.buffer.clone(), self.settings.clone(), ctx);
        *clone.scroll_position.lock() = *self.scroll_position.lock();
        Some(clone)
    }

    fn save(
        &mut self,
        new_file: Option<FileHandle>,
        ctx: &mut ViewContext<Self>,
    ) -> LocalBoxFuture<'static, Result<()>> {
        self.buffer.update(ctx, |b, ctx| b.save(new_file, ctx))
    }

    fn is_dirty(&self, ctx: &AppContext) -> bool {
        self.buffer.read(ctx).is_dirty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Point, settings, test::sample_text};
    use gpui::App;
    use unindent::Unindent;

    #[test]
    fn test_selection_with_mouse() {
        App::test((), |app| {
            let buffer =
                app.add_model(|ctx| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n", ctx));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, buffer_view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));

            buffer_view.update(app, |view, ctx| {
                view.begin_selection(DisplayPoint::new(2, 2), false, ctx);
            });

            let view = buffer_view.read(app);
            let selections = view
                .selections_in_range(
                    DisplayPoint::zero()..view.max_point(app.as_ref()),
                    app.as_ref(),
                )
                .collect::<Vec<_>>();
            assert_eq!(
                selections,
                [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
            );

            buffer_view.update(app, |view, ctx| {
                view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), ctx);
            });

            let view = buffer_view.read(app);
            let selections = view
                .selections_in_range(
                    DisplayPoint::zero()..view.max_point(app.as_ref()),
                    app.as_ref(),
                )
                .collect::<Vec<_>>();
            assert_eq!(
                selections,
                [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
            );

            buffer_view.update(app, |view, ctx| {
                view.update_selection(DisplayPoint::new(1, 1), Vector2F::zero(), ctx);
            });

            let view = buffer_view.read(app);
            let selections = view
                .selections_in_range(
                    DisplayPoint::zero()..view.max_point(app.as_ref()),
                    app.as_ref(),
                )
                .collect::<Vec<_>>();
            assert_eq!(
                selections,
                [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
            );

            buffer_view.update(app, |view, ctx| {
                view.end_selection(ctx);
                view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), ctx);
            });

            let view = buffer_view.read(app);
            let selections = view
                .selections_in_range(
                    DisplayPoint::zero()..view.max_point(app.as_ref()),
                    app.as_ref(),
                )
                .collect::<Vec<_>>();
            assert_eq!(
                selections,
                [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
            );

            buffer_view.update(app, |view, ctx| {
                view.begin_selection(DisplayPoint::new(3, 3), true, ctx);
                view.update_selection(DisplayPoint::new(0, 0), Vector2F::zero(), ctx);
            });

            let view = buffer_view.read(app);
            let selections = view
                .selections_in_range(
                    DisplayPoint::zero()..view.max_point(app.as_ref()),
                    app.as_ref(),
                )
                .collect::<Vec<_>>();
            assert_eq!(
                selections,
                [
                    DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)
                ]
            );

            buffer_view.update(app, |view, ctx| {
                view.end_selection(ctx);
            });

            let view = buffer_view.read(app);
            let selections = view
                .selections_in_range(
                    DisplayPoint::zero()..view.max_point(app.as_ref()),
                    app.as_ref(),
                )
                .collect::<Vec<_>>();
            assert_eq!(
                selections,
                [DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)]
            );
        });
    }

    #[test]
    fn test_layout_line_numbers() {
        App::test((), |app| {
            let layout_cache = TextLayoutCache::new(app.platform().fonts());
            let font_cache = app.font_cache().clone();

            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(6, 6), ctx));

            let settings = settings::channel(&font_cache).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            let layouts = view
                .read(app)
                .layout_line_numbers(1000.0, &font_cache, &layout_cache, app.as_ref())
                .unwrap();
            assert_eq!(layouts.len(), 6);
        })
    }

    #[test]
    fn test_fold() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| {
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
                    ctx,
                )
            });
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[DisplayPoint::new(8, 0)..DisplayPoint::new(12, 0)],
                    ctx,
                )
                .unwrap();
                view.fold(&(), ctx);
                assert_eq!(
                    view.text(ctx.as_ref()),
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

                view.fold(&(), ctx);
                assert_eq!(
                    view.text(ctx.as_ref()),
                    "
                    impl Foo {…
                    }
                "
                    .unindent(),
                );

                view.unfold(&(), ctx);
                assert_eq!(
                    view.text(ctx.as_ref()),
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

                view.unfold(&(), ctx);
                assert_eq!(view.text(ctx.as_ref()), buffer.read(ctx).text());
            });
        });
    }

    #[test]
    fn test_move_cursor() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(6, 6), ctx));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            buffer.update(app, |buffer, ctx| {
                buffer
                    .edit(
                        vec![
                            Point::new(1, 0)..Point::new(1, 0),
                            Point::new(1, 1)..Point::new(1, 1),
                        ],
                        "\t",
                        Some(ctx),
                    )
                    .unwrap();
            });

            view.update(app, |view, ctx| {
                view.move_down(&(), ctx);
                assert_eq!(
                    view.selection_ranges(ctx.as_ref()),
                    &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
                );

                view.move_right(&(), ctx);
                assert_eq!(
                    view.selection_ranges(ctx.as_ref()),
                    &[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4)]
                );

                view.move_left(&(), ctx);
                assert_eq!(
                    view.selection_ranges(ctx.as_ref()),
                    &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
                );

                view.move_up(&(), ctx);
                assert_eq!(
                    view.selection_ranges(ctx.as_ref()),
                    &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
                );

                view.move_to_end(&(), ctx);
                assert_eq!(
                    view.selection_ranges(ctx.as_ref()),
                    &[DisplayPoint::new(5, 6)..DisplayPoint::new(5, 6)]
                );

                view.move_to_beginning(&(), ctx);
                assert_eq!(
                    view.selection_ranges(ctx.as_ref()),
                    &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
                );

                view.select_display_ranges(
                    &[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2)],
                    ctx,
                )
                .unwrap();
                view.select_to_beginning(&(), ctx);
                assert_eq!(
                    view.selection_ranges(ctx.as_ref()),
                    &[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 0)]
                );

                view.select_to_end(&(), ctx);
                assert_eq!(
                    view.selection_ranges(ctx.as_ref()),
                    &[DisplayPoint::new(0, 1)..DisplayPoint::new(5, 6)]
                );
            });
        });
    }

    #[test]
    fn test_beginning_end_of_line() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, "abc\n  def", ctx));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                        DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
                    ],
                    ctx,
                )
                .unwrap();
            });

            view.update(app, |view, ctx| view.move_to_beginning_of_line(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                ]
            );

            view.update(app, |view, ctx| view.move_to_beginning_of_line(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );

            view.update(app, |view, ctx| view.move_to_beginning_of_line(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                ]
            );

            view.update(app, |view, ctx| view.move_to_end_of_line(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
                ]
            );

            // Moving to the end of line again is a no-op.
            view.update(app, |view, ctx| view.move_to_end_of_line(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
                ]
            );

            view.update(app, |view, ctx| {
                view.move_left(&(), ctx);
                view.select_to_beginning_of_line(&true, ctx);
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
                ]
            );

            view.update(app, |view, ctx| {
                view.select_to_beginning_of_line(&true, ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 0),
                ]
            );

            view.update(app, |view, ctx| {
                view.select_to_beginning_of_line(&true, ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
                ]
            );

            view.update(app, |view, ctx| view.select_to_end_of_line(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 5),
                ]
            );

            view.update(app, |view, ctx| view.delete_to_end_of_line(&(), ctx));
            assert_eq!(view.read(app).text(app.as_ref()), "ab\n  de");
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
                ]
            );

            view.update(app, |view, ctx| view.delete_to_beginning_of_line(&(), ctx));
            assert_eq!(view.read(app).text(app.as_ref()), "\n");
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );
        });
    }

    #[test]
    fn test_prev_next_word_boundary() {
        App::test((), |app| {
            let buffer = app
                .add_model(|ctx| Buffer::new(0, "use std::str::{foo, bar}\n\n  {baz.qux()}", ctx));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 11)..DisplayPoint::new(0, 11),
                        DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4),
                    ],
                    ctx,
                )
                .unwrap();
            });

            view.update(app, |view, ctx| {
                view.move_to_previous_word_boundary(&(), ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 3),
                ]
            );

            view.update(app, |view, ctx| {
                view.move_to_previous_word_boundary(&(), ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
                ]
            );

            view.update(app, |view, ctx| {
                view.move_to_previous_word_boundary(&(), ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 4)..DisplayPoint::new(0, 4),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                ]
            );

            view.update(app, |view, ctx| {
                view.move_to_previous_word_boundary(&(), ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );

            view.update(app, |view, ctx| {
                view.move_to_previous_word_boundary(&(), ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(0, 24)..DisplayPoint::new(0, 24),
                ]
            );

            view.update(app, |view, ctx| {
                view.move_to_previous_word_boundary(&(), ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(0, 23)..DisplayPoint::new(0, 23),
                ]
            );

            view.update(app, |view, ctx| view.move_to_next_word_boundary(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                    DisplayPoint::new(0, 24)..DisplayPoint::new(0, 24),
                ]
            );

            view.update(app, |view, ctx| view.move_to_next_word_boundary(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 4)..DisplayPoint::new(0, 4),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                ]
            );

            view.update(app, |view, ctx| view.move_to_next_word_boundary(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 7)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                ]
            );

            view.update(app, |view, ctx| view.move_to_next_word_boundary(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
                ]
            );

            view.update(app, |view, ctx| {
                view.move_right(&(), ctx);
                view.select_to_previous_word_boundary(&(), ctx);
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 2),
                ]
            );

            view.update(app, |view, ctx| {
                view.select_to_previous_word_boundary(&(), ctx)
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 7),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 0),
                ]
            );

            view.update(app, |view, ctx| view.select_to_next_word_boundary(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 2),
                ]
            );

            view.update(app, |view, ctx| view.delete_to_next_word_boundary(&(), ctx));
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "use std::s::{foo, bar}\n\n  {az.qux()}"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 10)..DisplayPoint::new(0, 10),
                    DisplayPoint::new(2, 3)..DisplayPoint::new(2, 3),
                ]
            );

            view.update(app, |view, ctx| {
                view.delete_to_previous_word_boundary(&(), ctx)
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "use std::::{foo, bar}\n\n  az.qux()}"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 9)..DisplayPoint::new(0, 9),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2),
                ]
            );
        });
    }

    #[test]
    fn test_backspace() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| {
                Buffer::new(
                    0,
                    "one two three\nfour five six\nseven eight nine\nten\n",
                    ctx,
                )
            });
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        // an empty selection - the preceding character is deleted
                        DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                        // one character selected - it is deleted
                        DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                        // a line suffix selected - it is deleted
                        DisplayPoint::new(2, 6)..DisplayPoint::new(3, 0),
                    ],
                    ctx,
                )
                .unwrap();
                view.backspace(&(), ctx);
            });

            assert_eq!(
                buffer.read(app).text(),
                "oe two three\nfou five six\nseven ten\n"
            );
        })
    }

    #[test]
    fn test_delete() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| {
                Buffer::new(
                    0,
                    "one two three\nfour five six\nseven eight nine\nten\n",
                    ctx,
                )
            });
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        // an empty selection - the following character is deleted
                        DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                        // one character selected - it is deleted
                        DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                        // a line suffix selected - it is deleted
                        DisplayPoint::new(2, 6)..DisplayPoint::new(3, 0),
                    ],
                    ctx,
                )
                .unwrap();
                view.delete(&(), ctx);
            });

            assert_eq!(
                buffer.read(app).text(),
                "on two three\nfou five six\nseven ten\n"
            );
        })
    }

    #[test]
    fn test_delete_line() {
        App::test((), |app| {
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let buffer = app.add_model(|ctx| Buffer::new(0, "abc\ndef\nghi\n", ctx));
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                        DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                        DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                    ],
                    ctx,
                )
                .unwrap();
                view.delete_line(&(), ctx);
            });
            assert_eq!(view.read(app).text(app.as_ref()), "ghi");
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)
                ]
            );

            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let buffer = app.add_model(|ctx| Buffer::new(0, "abc\ndef\nghi\n", ctx));
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[DisplayPoint::new(2, 0)..DisplayPoint::new(0, 1)],
                    ctx,
                )
                .unwrap();
                view.delete_line(&(), ctx);
            });
            assert_eq!(view.read(app).text(app.as_ref()), "ghi\n");
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)]
            );
        });
    }

    #[test]
    fn test_duplicate_line() {
        App::test((), |app| {
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let buffer = app.add_model(|ctx| Buffer::new(0, "abc\ndef\nghi\n", ctx));
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                        DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                        DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                        DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                    ],
                    ctx,
                )
                .unwrap();
                view.duplicate_line(&(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "abc\nabc\ndef\ndef\nghi\n\n"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(6, 0)..DisplayPoint::new(6, 0),
                ]
            );

            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let buffer = app.add_model(|ctx| Buffer::new(0, "abc\ndef\nghi\n", ctx));
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 1)..DisplayPoint::new(1, 1),
                        DisplayPoint::new(1, 2)..DisplayPoint::new(2, 1),
                    ],
                    ctx,
                )
                .unwrap();
                view.duplicate_line(&(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "abc\ndef\nghi\nabc\ndef\nghi\n"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(3, 1)..DisplayPoint::new(4, 1),
                    DisplayPoint::new(4, 2)..DisplayPoint::new(5, 1),
                ]
            );
        });
    }

    #[test]
    fn test_move_line_up_down() {
        App::test((), |app| {
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(10, 5), ctx));
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.fold_ranges(
                    vec![
                        Point::new(0, 2)..Point::new(1, 2),
                        Point::new(2, 3)..Point::new(4, 1),
                        Point::new(7, 0)..Point::new(8, 4),
                    ],
                    ctx,
                );
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                        DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                        DisplayPoint::new(3, 2)..DisplayPoint::new(4, 2),
                        DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2),
                    ],
                    ctx,
                )
                .unwrap();
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "aa…bbb\nccc…eeee\nfffff\nggggg\n…i\njjjjj"
            );

            view.update(app, |view, ctx| view.move_line_up(&(), ctx));
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "aa…bbb\nccc…eeee\nggggg\n…i\njjjjj\nfffff"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(3, 2),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(4, 2)
                ]
            );

            view.update(app, |view, ctx| view.move_line_down(&(), ctx));
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "ccc…eeee\naa…bbb\nfffff\nggggg\n…i\njjjjj"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(4, 2),
                    DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2)
                ]
            );

            view.update(app, |view, ctx| view.move_line_down(&(), ctx));
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "ccc…eeee\nfffff\naa…bbb\nggggg\n…i\njjjjj"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                    DisplayPoint::new(3, 2)..DisplayPoint::new(4, 2),
                    DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2)
                ]
            );

            view.update(app, |view, ctx| view.move_line_up(&(), ctx));
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "ccc…eeee\naa…bbb\nggggg\n…i\njjjjj\nfffff"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(2, 2)..DisplayPoint::new(3, 2),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(4, 2)
                ]
            );
        });
    }

    #[test]
    fn test_clipboard() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, "one two three four five six ", ctx));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let view = app
                .add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx))
                .1;

            // Cut with three selections. Clipboard text is divided into three slices.
            view.update(app, |view, ctx| {
                view.select_ranges(vec![0..4, 8..14, 19..24], false, ctx);
                view.cut(&(), ctx);
            });
            assert_eq!(view.read(app).text(app.as_ref()), "two four six ");

            // Paste with three cursors. Each cursor pastes one slice of the clipboard text.
            view.update(app, |view, ctx| {
                view.select_ranges(vec![4..4, 9..9, 13..13], false, ctx);
                view.paste(&(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "two one four three six five "
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 8)..DisplayPoint::new(0, 8),
                    DisplayPoint::new(0, 19)..DisplayPoint::new(0, 19),
                    DisplayPoint::new(0, 28)..DisplayPoint::new(0, 28)
                ]
            );

            // Paste again but with only two cursors. Since the number of cursors doesn't
            // match the number of slices in the clipboard, the entire clipboard text
            // is pasted at each cursor.
            view.update(app, |view, ctx| {
                view.select_ranges(vec![0..0, 28..28], false, ctx);
                view.insert(&"( ".to_string(), ctx);
                view.paste(&(), ctx);
                view.insert(&") ".to_string(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "( one three five ) two one four three six five ( one three five ) "
            );

            view.update(app, |view, ctx| {
                view.select_ranges(vec![0..0], false, ctx);
                view.insert(&"123\n4567\n89\n".to_string(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "123\n4567\n89\n( one three five ) two one four three six five ( one three five ) "
            );

            // Cut with three selections, one of which is full-line.
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2),
                        DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                        DisplayPoint::new(2, 0)..DisplayPoint::new(2, 1),
                    ],
                    ctx,
                )
                .unwrap();
                view.cut(&(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "13\n9\n( one three five ) two one four three six five ( one three five ) "
            );

            // Paste with three selections, noticing how the copied selection that was full-line
            // gets inserted before the second cursor.
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                        DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                        DisplayPoint::new(2, 2)..DisplayPoint::new(2, 3),
                    ],
                    ctx,
                )
                .unwrap();
                view.paste(&(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "123\n4567\n9\n( 8ne three five ) two one four three six five ( one three five ) "
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    DisplayPoint::new(3, 3)..DisplayPoint::new(3, 3),
                ]
            );

            // Copy with a single cursor only, which writes the whole line into the clipboard.
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)],
                    ctx,
                )
                .unwrap();
                view.copy(&(), ctx);
            });

            // Paste with three selections, noticing how the copied full-line selection is inserted
            // before the empty selections but replaces the selection that is non-empty.
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                        DisplayPoint::new(1, 0)..DisplayPoint::new(1, 2),
                        DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                    ],
                    ctx,
                )
                .unwrap();
                view.paste(&(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "123\n123\n123\n67\n123\n9\n( 8ne three five ) two one four three six five ( one three five ) "
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[
                    DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                    DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(5, 1)..DisplayPoint::new(5, 1),
                ]
            );
        });
    }

    #[test]
    fn test_select_all() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, "abc\nde\nfgh", ctx));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |b, ctx| b.select_all(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                &[DisplayPoint::new(0, 0)..DisplayPoint::new(2, 3)]
            );
        });
    }

    #[test]
    fn test_select_line() {
        App::test((), |app| {
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(6, 5), ctx));
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                        DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                        DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                        DisplayPoint::new(4, 2)..DisplayPoint::new(4, 2),
                    ],
                    ctx,
                )
                .unwrap();
                view.select_line(&(), ctx);
            });
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(2, 0),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(5, 0),
                ]
            );

            view.update(app, |view, ctx| view.select_line(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![
                    DisplayPoint::new(0, 0)..DisplayPoint::new(3, 0),
                    DisplayPoint::new(4, 0)..DisplayPoint::new(5, 5),
                ]
            );

            view.update(app, |view, ctx| view.select_line(&(), ctx));
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                vec![DisplayPoint::new(0, 0)..DisplayPoint::new(5, 5)]
            );
        });
    }

    #[test]
    fn test_split_selection_into_lines() {
        App::test((), |app| {
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(9, 5), ctx));
            let (_, view) = app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));
            view.update(app, |view, ctx| {
                view.fold_ranges(
                    vec![
                        Point::new(0, 2)..Point::new(1, 2),
                        Point::new(2, 3)..Point::new(4, 1),
                        Point::new(7, 0)..Point::new(8, 4),
                    ],
                    ctx,
                );
                view.select_display_ranges(
                    &[
                        DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                        DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                        DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                        DisplayPoint::new(4, 2)..DisplayPoint::new(4, 2),
                    ],
                    ctx,
                )
                .unwrap();
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "aa…bbb\nccc…eeee\nfffff\nggggg\n…i"
            );

            view.update(app, |view, ctx| view.split_selection_into_lines(&(), ctx));
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "aa…bbb\nccc…eeee\nfffff\nggggg\n…i"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
                [
                    DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                    DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                    DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                    DisplayPoint::new(4, 2)..DisplayPoint::new(4, 2)
                ]
            );

            view.update(app, |view, ctx| {
                view.select_display_ranges(
                    &[DisplayPoint::new(4, 0)..DisplayPoint::new(0, 1)],
                    ctx,
                )
                .unwrap();
                view.split_selection_into_lines(&(), ctx);
            });
            assert_eq!(
                view.read(app).text(app.as_ref()),
                "aaaaa\nbbbbb\nccccc\nddddd\neeeee\nfffff\nggggg\n…i"
            );
            assert_eq!(
                view.read(app).selection_ranges(app.as_ref()),
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

    impl BufferView {
        fn selection_ranges(&self, app: &AppContext) -> Vec<Range<DisplayPoint>> {
            self.selections_in_range(DisplayPoint::zero()..self.max_point(app), app)
                .collect::<Vec<_>>()
        }
    }
}

use super::{
    buffer, movement, Anchor, Bias, Buffer, BufferElement, DisplayMap, DisplayPoint, Point,
    ToOffset, ToPoint,
};
use crate::{settings::Settings, watch, workspace};
use anyhow::Result;
use gpui::{
    fonts::Properties as FontProperties, keymap::Binding, text_layout, App, AppContext, Element,
    ElementBox, Entity, FontCache, ModelHandle, MutableAppContext, Task, View, ViewContext,
    WeakViewHandle,
};
use gpui::{geometry::vector::Vector2F, TextLayoutCache};
use parking_lot::Mutex;
use smallvec::SmallVec;
use smol::Timer;
use std::{
    cmp::{self, Ordering},
    fmt::Write,
    mem,
    ops::Range,
    sync::Arc,
    time::Duration,
};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

pub fn init(app: &mut App) {
    app.add_bindings(vec![
        Binding::new("backspace", "buffer:backspace", Some("BufferView")),
        Binding::new("enter", "buffer:newline", Some("BufferView")),
        Binding::new("up", "buffer:move_up", Some("BufferView")),
        Binding::new("down", "buffer:move_down", Some("BufferView")),
        Binding::new("left", "buffer:move_left", Some("BufferView")),
        Binding::new("right", "buffer:move_right", Some("BufferView")),
        Binding::new("shift-up", "buffer:select_up", Some("BufferView")),
        Binding::new("shift-down", "buffer:select_down", Some("BufferView")),
        Binding::new("shift-left", "buffer:select_left", Some("BufferView")),
        Binding::new("shift-right", "buffer:select_right", Some("BufferView")),
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
    app.add_action("buffer:insert", BufferView::insert);
    app.add_action("buffer:newline", BufferView::newline);
    app.add_action("buffer:backspace", BufferView::backspace);
    app.add_action("buffer:move_up", BufferView::move_up);
    app.add_action("buffer:move_down", BufferView::move_down);
    app.add_action("buffer:move_left", BufferView::move_left);
    app.add_action("buffer:move_right", BufferView::move_right);
    app.add_action("buffer:select_up", BufferView::select_up);
    app.add_action("buffer:select_down", BufferView::select_down);
    app.add_action("buffer:select_left", BufferView::select_left);
    app.add_action("buffer:select_right", BufferView::select_right);
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

// impl workspace::Item for Buffer {
//     type View = BufferView;

//     fn build_view(
//         buffer: ModelHandle<Self>,
//         settings: watch::Receiver<Settings>,
//         ctx: &mut ViewContext<Self::View>,
//     ) -> Self::View {
//         BufferView::for_buffer(buffer, settings, ctx)
//     }
// }

pub struct BufferView {
    handle: WeakViewHandle<Self>,
    buffer: ModelHandle<Buffer>,
    display_map: ModelHandle<DisplayMap>,
    selections: Vec<Selection>,
    pending_selection: Option<Selection>,
    scroll_position: Mutex<Vector2F>,
    autoscroll_requested: Mutex<bool>,
    settings: watch::Receiver<Settings>,
    focused: bool,
    cursors_visible: bool,
    blink_epoch: usize,
    blinking_paused: bool,
    single_line: bool,
}

impl BufferView {
    pub fn single_line(settings: watch::Receiver<Settings>, ctx: &mut ViewContext<Self>) -> Self {
        let buffer = ctx.add_model(|_| Buffer::new(0, String::new()));
        let mut view = Self::for_buffer(buffer, settings, ctx);
        view.single_line = true;
        view
    }

    pub fn for_buffer(
        buffer: ModelHandle<Buffer>,
        settings: watch::Receiver<Settings>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        settings.notify_view_on_change(ctx);

        ctx.observe(&buffer, Self::on_buffer_changed);
        ctx.subscribe_to_model(&buffer, Self::on_buffer_event);
        let display_map = ctx.add_model(|ctx| {
            DisplayMap::new(
                buffer.clone(),
                smol::block_on(settings.read()).tab_size,
                ctx,
            )
        });
        ctx.observe(&display_map, Self::on_display_map_changed);

        let buffer_ref = buffer.as_ref(ctx);
        Self {
            handle: ctx.handle().downgrade(),
            buffer,
            display_map,
            selections: vec![Selection {
                start: buffer_ref.anchor_before(0).unwrap(),
                end: buffer_ref.anchor_before(0).unwrap(),
                reversed: false,
                goal_column: None,
            }],
            pending_selection: None,
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

        let map = self.display_map.as_ref(app);
        let visible_lines = viewport_height / line_height;
        let first_cursor_top = self
            .selections
            .first()
            .unwrap()
            .head()
            .to_display_point(map, app)
            .unwrap()
            .row() as f32;
        let last_cursor_bottom = self
            .selections
            .last()
            .unwrap()
            .head()
            .to_display_point(map, app)
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
        app: &AppContext,
    ) {
        let map = self.display_map.as_ref(app);

        let mut target_left = std::f32::INFINITY;
        let mut target_right = 0.0_f32;
        for selection in &self.selections {
            let head = selection.head().to_display_point(map, app).unwrap();
            let start_column = head.column().saturating_sub(3);
            let end_column = cmp::min(map.line_len(head.row(), app).unwrap(), head.column() + 3);
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

        let display_map = self.display_map.as_ref(ctx);
        let cursor = display_map
            .anchor_before(position, Bias::Left, ctx.app())
            .unwrap();
        let selection = Selection {
            start: cursor.clone(),
            end: cursor,
            reversed: false,
            goal_column: None,
        };

        if !add {
            self.selections.clear();
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
        let buffer = self.buffer.as_ref(ctx);
        let map = self.display_map.as_ref(ctx);
        let cursor = map.anchor_before(position, Bias::Left, ctx.app()).unwrap();
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
            let ix = self.selection_insertion_index(&selection.start, ctx.app());
            self.selections.insert(ix, selection);
            self.merge_selections(ctx.app());
            ctx.notify();
        } else {
            log::error!("end_selection dispatched with no pending selection");
        }
    }

    pub fn is_selecting(&self) -> bool {
        self.pending_selection.is_some()
    }

    #[cfg(test)]
    fn select_ranges<'a, T>(&mut self, ranges: T, ctx: &mut ViewContext<Self>) -> Result<()>
    where
        T: IntoIterator<Item = &'a Range<DisplayPoint>>,
    {
        let buffer = self.buffer.as_ref(ctx);
        let map = self.display_map.as_ref(ctx);
        let mut selections = Vec::new();
        for range in ranges {
            selections.push(Selection {
                start: map.anchor_after(range.start, Bias::Left, ctx.app())?,
                end: map.anchor_before(range.end, Bias::Left, ctx.app())?,
                reversed: false,
                goal_column: None,
            });
        }
        selections.sort_unstable_by(|a, b| a.start.cmp(&b.start, buffer).unwrap());
        self.selections = selections;
        self.merge_selections(ctx.app());
        ctx.notify();
        Ok(())
    }

    fn insert(&mut self, text: &String, ctx: &mut ViewContext<Self>) {
        let buffer = self.buffer.as_ref(ctx);
        let mut offset_ranges = SmallVec::<[Range<usize>; 32]>::new();
        for selection in &self.selections {
            let start = selection.start.to_offset(buffer).unwrap();
            let end = selection.end.to_offset(buffer).unwrap();
            offset_ranges.push(start..end);
        }

        self.buffer.update(ctx, |buffer, ctx| {
            if let Err(error) = buffer.edit(offset_ranges.iter().cloned(), text.as_str(), Some(ctx))
            {
                log::error!("error inserting text: {}", error);
            };
        });

        let buffer = self.buffer.as_ref(ctx);
        let char_count = text.chars().count() as isize;
        let mut delta = 0_isize;
        self.selections = offset_ranges
            .into_iter()
            .map(|range| {
                let start = range.start as isize;
                let end = range.end as isize;
                let anchor = buffer
                    .anchor_before((start + delta + char_count) as usize)
                    .unwrap();
                let deleted_count = end - start;
                delta += char_count - deleted_count;
                Selection {
                    start: anchor.clone(),
                    end: anchor,
                    reversed: false,
                    goal_column: None,
                }
            })
            .collect();

        self.pause_cursor_blinking(ctx);
        *self.autoscroll_requested.lock() = true;
    }

    fn newline(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if self.single_line {
            ctx.propagate_action();
        } else {
            self.insert(&"\n".into(), ctx);
        }
    }

    pub fn backspace(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let buffer = self.buffer.as_ref(ctx);
        let map = self.display_map.as_ref(ctx);
        for selection in &mut self.selections {
            if selection.range(buffer).is_empty() {
                let head = selection.head().to_display_point(map, ctx.app()).unwrap();
                let cursor = map
                    .anchor_before(
                        movement::left(map, head, ctx.app()).unwrap(),
                        Bias::Left,
                        ctx.app(),
                    )
                    .unwrap();
                selection.set_head(&buffer, cursor);
                selection.goal_column = None;
            }
        }
        self.changed_selections(ctx);
        self.insert(&String::new(), ctx);
    }

    pub fn move_left(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        {
            let app = ctx.app();
            let map = self.display_map.as_ref(ctx);
            for selection in &mut self.selections {
                let start = selection.start.to_display_point(map, app).unwrap();
                let end = selection.end.to_display_point(map, app).unwrap();

                if start != end {
                    selection.end = selection.start.clone();
                } else {
                    let cursor = map
                        .anchor_before(movement::left(map, start, app).unwrap(), Bias::Left, app)
                        .unwrap();
                    selection.start = cursor.clone();
                    selection.end = cursor;
                }
                selection.reversed = false;
                selection.goal_column = None;
            }
        }
        self.changed_selections(ctx);
    }

    pub fn select_left(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        {
            let buffer = self.buffer.as_ref(ctx);
            let map = self.display_map.as_ref(ctx);
            for selection in &mut self.selections {
                let head = selection.head().to_display_point(map, ctx.app()).unwrap();
                let cursor = map
                    .anchor_before(
                        movement::left(map, head, ctx.app()).unwrap(),
                        Bias::Left,
                        ctx.app(),
                    )
                    .unwrap();
                selection.set_head(&buffer, cursor);
                selection.goal_column = None;
            }
        }
        self.changed_selections(ctx);
    }

    pub fn move_right(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        {
            let app = ctx.app();
            let map = self.display_map.as_ref(app);
            for selection in &mut self.selections {
                let start = selection.start.to_display_point(map, app).unwrap();
                let end = selection.end.to_display_point(map, app).unwrap();

                if start != end {
                    selection.start = selection.end.clone();
                } else {
                    let cursor = map
                        .anchor_before(movement::right(map, end, app).unwrap(), Bias::Right, app)
                        .unwrap();
                    selection.start = cursor.clone();
                    selection.end = cursor;
                }
                selection.reversed = false;
                selection.goal_column = None;
            }
        }
        self.changed_selections(ctx);
    }

    pub fn select_right(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        {
            let buffer = self.buffer.as_ref(ctx);
            let app = ctx.app();
            let map = self.display_map.as_ref(app);
            for selection in &mut self.selections {
                let head = selection.head().to_display_point(map, ctx.app()).unwrap();
                let cursor = map
                    .anchor_before(movement::right(map, head, app).unwrap(), Bias::Right, app)
                    .unwrap();
                selection.set_head(&buffer, cursor);
                selection.goal_column = None;
            }
        }
        self.changed_selections(ctx);
    }

    pub fn move_up(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if self.single_line {
            ctx.propagate_action();
        } else {
            let app = ctx.app();
            let map = self.display_map.as_ref(app);
            for selection in &mut self.selections {
                let start = selection.start.to_display_point(map, app).unwrap();
                let end = selection.end.to_display_point(map, app).unwrap();
                if start != end {
                    selection.goal_column = None;
                }

                let (start, goal_column) =
                    movement::up(map, start, selection.goal_column, app).unwrap();
                let cursor = map.anchor_before(start, Bias::Left, app).unwrap();
                selection.start = cursor.clone();
                selection.end = cursor;
                selection.goal_column = goal_column;
                selection.reversed = false;
            }
            self.changed_selections(ctx);
        }
    }

    pub fn select_up(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if self.single_line {
            ctx.propagate_action();
        } else {
            let app = ctx.app();
            let buffer = self.buffer.as_ref(app);
            let map = self.display_map.as_ref(app);
            for selection in &mut self.selections {
                let head = selection.head().to_display_point(map, app).unwrap();
                let (head, goal_column) =
                    movement::up(map, head, selection.goal_column, app).unwrap();
                selection.set_head(&buffer, map.anchor_before(head, Bias::Left, app).unwrap());
                selection.goal_column = goal_column;
            }
            self.changed_selections(ctx);
        }
    }

    pub fn move_down(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if self.single_line {
            ctx.propagate_action();
        } else {
            let app = ctx.app();
            let map = self.display_map.as_ref(app);
            for selection in &mut self.selections {
                let start = selection.start.to_display_point(map, app).unwrap();
                let end = selection.end.to_display_point(map, app).unwrap();
                if start != end {
                    selection.goal_column = None;
                }

                let (start, goal_column) =
                    movement::down(map, end, selection.goal_column, app).unwrap();
                let cursor = map.anchor_before(start, Bias::Right, app).unwrap();
                selection.start = cursor.clone();
                selection.end = cursor;
                selection.goal_column = goal_column;
                selection.reversed = false;
            }
            self.changed_selections(ctx);
        }
    }

    pub fn select_down(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if self.single_line {
            ctx.propagate_action();
        } else {
            let app = ctx.app();
            let buffer = self.buffer.as_ref(ctx);
            let map = self.display_map.as_ref(ctx);
            for selection in &mut self.selections {
                let head = selection.head().to_display_point(map, app).unwrap();
                let (head, goal_column) =
                    movement::down(map, head, selection.goal_column, app).unwrap();
                selection.set_head(&buffer, map.anchor_before(head, Bias::Right, app).unwrap());
                selection.goal_column = goal_column;
            }
            self.changed_selections(ctx);
        }
    }

    pub fn changed_selections(&mut self, ctx: &mut ViewContext<Self>) {
        self.merge_selections(ctx.app());
        self.pause_cursor_blinking(ctx);
        *self.autoscroll_requested.lock() = true;
        ctx.notify();
    }

    fn merge_selections(&mut self, ctx: &AppContext) {
        let buffer = self.buffer.as_ref(ctx);
        let mut i = 1;
        while i < self.selections.len() {
            if self.selections[i - 1]
                .end
                .cmp(&self.selections[i].start, buffer)
                .unwrap()
                >= Ordering::Equal
            {
                let removed = self.selections.remove(i);
                if removed
                    .start
                    .cmp(&self.selections[i - 1].start, buffer)
                    .unwrap()
                    < Ordering::Equal
                {
                    self.selections[i - 1].start = removed.start;
                }
                if removed
                    .end
                    .cmp(&self.selections[i - 1].end, buffer)
                    .unwrap()
                    > Ordering::Equal
                {
                    self.selections[i - 1].end = removed.end;
                }
            } else {
                i += 1;
            }
        }
    }

    pub fn first_selection(&self, app: &AppContext) -> Range<DisplayPoint> {
        self.selections
            .first()
            .unwrap()
            .display_range(self.display_map.as_ref(app), app)
    }

    pub fn last_selection(&self, app: &AppContext) -> Range<DisplayPoint> {
        self.selections
            .last()
            .unwrap()
            .display_range(self.display_map.as_ref(app), app)
    }

    pub fn selections_in_range<'a>(
        &'a self,
        range: Range<DisplayPoint>,
        app: &'a AppContext,
    ) -> impl 'a + Iterator<Item = Range<DisplayPoint>> {
        let map = self.display_map.as_ref(app);

        let start = map.anchor_before(range.start, Bias::Left, app).unwrap();
        let start_index = self.selection_insertion_index(&start, app);
        let pending_selection = self.pending_selection.as_ref().and_then(|s| {
            let selection_range = s.display_range(map, app);
            if selection_range.start <= range.end || selection_range.end <= range.end {
                Some(selection_range)
            } else {
                None
            }
        });
        self.selections[start_index..]
            .iter()
            .map(move |s| s.display_range(map, app))
            .take_while(move |r| r.start <= range.end || r.end <= range.end)
            .chain(pending_selection)
    }

    fn selection_insertion_index(&self, start: &Anchor, app: &AppContext) -> usize {
        let buffer = self.buffer.as_ref(app);

        match self
            .selections
            .binary_search_by(|probe| probe.start.cmp(&start, buffer).unwrap())
        {
            Ok(index) => index,
            Err(index) => {
                if index > 0
                    && self.selections[index - 1].end.cmp(&start, buffer).unwrap()
                        == Ordering::Greater
                {
                    index - 1
                } else {
                    index
                }
            }
        }
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

        let app = ctx.app();
        let map = self.display_map.as_ref(app);
        for selection in &self.selections {
            let (start, end) = selection.display_range(map, app).sorted();
            let buffer_start_row = start.to_buffer_point(map, Bias::Left, app).unwrap().row;

            for row in (0..=end.row()).rev() {
                if self.is_line_foldable(row, app) && !map.is_line_folded(row) {
                    let fold_range = self.foldable_range_for_line(row, app).unwrap();
                    if fold_range.end.row >= buffer_start_row {
                        fold_ranges.push(fold_range);
                        if row <= start.row() {
                            break;
                        }
                    }
                }
            }
        }

        if !fold_ranges.is_empty() {
            self.display_map.update(ctx, |map, ctx| {
                map.fold(fold_ranges, ctx).unwrap();
            });
            *self.autoscroll_requested.lock() = true;
        }
    }

    pub fn unfold(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        use super::RangeExt;

        let app = ctx.app();
        let map = self.display_map.as_ref(app);
        let buffer = self.buffer.as_ref(app);
        let ranges = self
            .selections
            .iter()
            .map(|s| {
                let (start, end) = s.display_range(map, app).sorted();
                let mut start = start.to_buffer_point(map, Bias::Left, app).unwrap();
                let mut end = end.to_buffer_point(map, Bias::Left, app).unwrap();
                start.column = 0;
                end.column = buffer.line_len(end.row).unwrap();
                start..end
            })
            .collect::<Vec<_>>();

        self.display_map.update(ctx, |map, ctx| {
            map.unfold(ranges, ctx).unwrap();
        });
        *self.autoscroll_requested.lock() = true;
    }

    fn is_line_foldable(&self, display_row: u32, app: &AppContext) -> bool {
        let max_point = self.max_point(app);
        if display_row >= max_point.row() {
            false
        } else {
            let (start_indent, is_blank) = self.line_indent(display_row, app).unwrap();
            if is_blank {
                false
            } else {
                for display_row in display_row + 1..=max_point.row() {
                    let (indent, is_blank) = self.line_indent(display_row, app).unwrap();
                    if !is_blank {
                        return indent > start_indent;
                    }
                }
                false
            }
        }
    }

    fn line_indent(&self, display_row: u32, app: &AppContext) -> Result<(usize, bool)> {
        let mut indent = 0;
        let mut is_blank = true;
        for c in self
            .display_map
            .as_ref(app)
            .chars_at(DisplayPoint::new(display_row, 0), app)?
        {
            if c == ' ' {
                indent += 1;
            } else {
                is_blank = c == '\n';
                break;
            }
        }
        Ok((indent, is_blank))
    }

    fn foldable_range_for_line(&self, start_row: u32, app: &AppContext) -> Result<Range<Point>> {
        let map = self.display_map.as_ref(app);
        let max_point = self.max_point(app);

        let (start_indent, _) = self.line_indent(start_row, app)?;
        let start = DisplayPoint::new(start_row, self.line_len(start_row, app)?);
        let mut end = None;
        for row in start_row + 1..=max_point.row() {
            let (indent, is_blank) = self.line_indent(row, app)?;
            if !is_blank && indent <= start_indent {
                end = Some(DisplayPoint::new(row - 1, self.line_len(row - 1, app)?));
                break;
            }
        }

        let end = end.unwrap_or(max_point);
        return Ok(start.to_buffer_point(map, Bias::Left, app)?
            ..end.to_buffer_point(map, Bias::Left, app)?);
    }

    pub fn fold_selected_ranges(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.display_map.update(ctx, |map, ctx| {
            let buffer = self.buffer.as_ref(ctx);
            let ranges = self
                .selections
                .iter()
                .map(|s| s.range(buffer))
                .collect::<Vec<_>>();
            map.fold(ranges, ctx).unwrap();
        });
    }

    pub fn line(&self, display_row: u32, app: &AppContext) -> Result<String> {
        self.display_map.as_ref(app).line(display_row, app)
    }

    pub fn line_len(&self, display_row: u32, app: &AppContext) -> Result<u32> {
        self.display_map.as_ref(app).line_len(display_row, app)
    }

    pub fn rightmost_point(&self, app: &AppContext) -> DisplayPoint {
        self.display_map.as_ref(app).rightmost_point()
    }

    pub fn max_point(&self, app: &AppContext) -> DisplayPoint {
        self.display_map.as_ref(app).max_point(app)
    }

    pub fn text(&self, app: &AppContext) -> String {
        self.display_map.as_ref(app).text(app)
    }

    pub fn font_size(&self) -> f32 {
        smol::block_on(self.settings.read()).buffer_font_size
    }

    pub fn font_ascent(&self, font_cache: &FontCache) -> f32 {
        let settings = smol::block_on(self.settings.read());
        let font_id = font_cache.default_font(settings.buffer_font_family);
        let ascent = font_cache.metric(font_id, |m| m.ascent);
        font_cache.scale_metric(ascent, font_id, settings.buffer_font_size)
    }

    pub fn font_descent(&self, font_cache: &FontCache) -> f32 {
        let settings = smol::block_on(self.settings.read());
        let font_id = font_cache.default_font(settings.buffer_font_family);
        let ascent = font_cache.metric(font_id, |m| m.descent);
        font_cache.scale_metric(ascent, font_id, settings.buffer_font_size)
    }

    pub fn line_height(&self, font_cache: &FontCache) -> f32 {
        let settings = smol::block_on(self.settings.read());
        let font_id = font_cache.default_font(settings.buffer_font_family);
        font_cache.line_height(font_id, settings.buffer_font_size)
    }

    pub fn em_width(&self, font_cache: &FontCache) -> f32 {
        let settings = smol::block_on(self.settings.read());
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
        let settings = smol::block_on(self.settings.read());
        let font_size = settings.buffer_font_size;
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;
        let digit_count = ((self.buffer.as_ref(app).max_point().row + 1) as f32)
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
        app: &AppContext,
    ) -> Result<Vec<Arc<text_layout::Line>>> {
        let display_map = self.display_map.as_ref(app);

        let settings = smol::block_on(self.settings.read());
        let font_size = settings.buffer_font_size;
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;

        let start_row = self.scroll_position().y() as usize;
        let end_row = cmp::min(
            self.max_point(app).row() as usize,
            start_row + (viewport_height / self.line_height(font_cache)).ceil() as usize,
        );
        let line_count = end_row - start_row + 1;

        let mut layouts = Vec::with_capacity(line_count);
        let mut line_number = String::new();
        for buffer_row in display_map.buffer_rows(start_row as u32)?.take(line_count) {
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
        app: &AppContext,
    ) -> Result<Vec<Arc<text_layout::Line>>> {
        let display_map = self.display_map.as_ref(app);

        rows.end = cmp::min(rows.end, display_map.max_point(app).row() + 1);
        if rows.start >= rows.end {
            return Ok(Vec::new());
        }

        let settings = smol::block_on(self.settings.read());
        let font_id =
            font_cache.select_font(settings.buffer_font_family, &FontProperties::new())?;
        let font_size = settings.buffer_font_size;

        let mut layouts = Vec::with_capacity(rows.len());
        let mut line = String::new();
        let mut line_len = 0;
        let mut row = rows.start;
        let chars = display_map
            .chars_at(DisplayPoint::new(rows.start, 0), app)
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
        let settings = smol::block_on(self.settings.read());
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

    fn on_display_map_changed(&mut self, _: ModelHandle<DisplayMap>, ctx: &mut ViewContext<Self>) {
        ctx.notify();
    }

    fn on_buffer_event(
        &mut self,
        _: ModelHandle<Buffer>,
        event: &buffer::Event,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            buffer::Event::Edited(_) => ctx.emit(Event::Edited),
        }
    }
}

struct Selection {
    start: Anchor,
    end: Anchor,
    reversed: bool,
    goal_column: Option<u32>,
}

pub enum Event {
    Activate,
    Edited,
    Blurred,
}

impl Entity for BufferView {
    type Event = Event;
}

impl View for BufferView {
    fn render<'a>(&self, app: &AppContext) -> ElementBox {
        BufferElement::new(self.handle.upgrade(app).unwrap()).boxed()
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

    fn build_view(
        buffer: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        ctx: &mut ViewContext<Self::View>,
    ) -> Self::View {
        BufferView::for_buffer(buffer, settings, ctx)
    }
}

impl workspace::ItemView for BufferView {
    fn is_activate_event(event: &Self::Event) -> bool {
        match event {
            Event::Activate => true,
            _ => false,
        }
    }

    fn title(&self, app: &AppContext) -> std::string::String {
        if let Some(path) = self.buffer.as_ref(app).path(app) {
            path.file_name()
                .expect("buffer's path is always to a file")
                .to_string_lossy()
                .into()
        } else {
            "untitled".into()
        }
    }

    fn entry_id(&self, app: &AppContext) -> Option<(usize, usize)> {
        self.buffer.as_ref(app).entry_id()
    }

    fn clone_on_split(&self, ctx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        let clone = BufferView::for_buffer(self.buffer.clone(), self.settings.clone(), ctx);
        *clone.scroll_position.lock() = *self.scroll_position.lock();
        Some(clone)
    }

    fn save(&self, ctx: &mut MutableAppContext) -> Option<Task<Result<()>>> {
        self.buffer.update(ctx, |buffer, ctx| buffer.save(ctx))
    }
}

impl Selection {
    fn head(&self) -> &Anchor {
        if self.reversed {
            &self.start
        } else {
            &self.end
        }
    }

    fn set_head(&mut self, buffer: &Buffer, cursor: Anchor) {
        if cursor.cmp(self.tail(), buffer).unwrap() < Ordering::Equal {
            if !self.reversed {
                mem::swap(&mut self.start, &mut self.end);
                self.reversed = true;
            }
            self.start = cursor;
        } else {
            if self.reversed {
                mem::swap(&mut self.start, &mut self.end);
                self.reversed = false;
            }
            self.end = cursor;
        }
    }

    fn tail(&self) -> &Anchor {
        if self.reversed {
            &self.end
        } else {
            &self.start
        }
    }

    fn range(&self, buffer: &Buffer) -> Range<Point> {
        let start = self.start.to_point(buffer).unwrap();
        let end = self.end.to_point(buffer).unwrap();
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    fn display_range(&self, map: &DisplayMap, app: &AppContext) -> Range<DisplayPoint> {
        let start = self.start.to_display_point(map, app).unwrap();
        let end = self.end.to_display_point(map, app).unwrap();
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Point, settings, test::sample_text};
    use anyhow::Error;
    use unindent::Unindent;

    #[test]
    fn test_selection_with_mouse() {
        App::test((), |mut app| async move {
            let buffer = app.add_model(|_| Buffer::new(0, "aaaaaa\nbbbbbb\ncccccc\ndddddd\n"));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, buffer_view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer, settings, ctx));

            buffer_view.update(&mut app, |view, ctx| {
                view.begin_selection(DisplayPoint::new(2, 2), false, ctx);
            });

            buffer_view.read(&app, |view, app| {
                let selections = view
                    .selections_in_range(DisplayPoint::zero()..view.max_point(app), app)
                    .collect::<Vec<_>>();
                assert_eq!(
                    selections,
                    [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
                );
            });

            buffer_view.update(&mut app, |view, ctx| {
                view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), ctx);
            });

            buffer_view.read(&app, |view, app| {
                let selections = view
                    .selections_in_range(DisplayPoint::zero()..view.max_point(app), app)
                    .collect::<Vec<_>>();
                assert_eq!(
                    selections,
                    [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
                );
            });

            buffer_view.update(&mut app, |view, ctx| {
                view.update_selection(DisplayPoint::new(1, 1), Vector2F::zero(), ctx);
            });

            buffer_view.read(&app, |view, app| {
                let selections = view
                    .selections_in_range(DisplayPoint::zero()..view.max_point(app), app)
                    .collect::<Vec<_>>();
                assert_eq!(
                    selections,
                    [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
                );
            });

            buffer_view.update(&mut app, |view, ctx| {
                view.end_selection(ctx);
                view.update_selection(DisplayPoint::new(3, 3), Vector2F::zero(), ctx);
            });

            buffer_view.read(&app, |view, app| {
                let selections = view
                    .selections_in_range(DisplayPoint::zero()..view.max_point(app), app)
                    .collect::<Vec<_>>();
                assert_eq!(
                    selections,
                    [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
                );
            });

            buffer_view.update(&mut app, |view, ctx| {
                view.begin_selection(DisplayPoint::new(3, 3), true, ctx);
                view.update_selection(DisplayPoint::new(0, 0), Vector2F::zero(), ctx);
            });

            buffer_view.read(&app, |view, app| {
                let selections = view
                    .selections_in_range(DisplayPoint::zero()..view.max_point(app), app)
                    .collect::<Vec<_>>();
                assert_eq!(
                    selections,
                    [
                        DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1),
                        DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)
                    ]
                );
            });

            buffer_view.update(&mut app, |view, ctx| {
                view.end_selection(ctx);
            });

            buffer_view.read(&app, |view, app| {
                let selections = view
                    .selections_in_range(DisplayPoint::zero()..view.max_point(app), app)
                    .collect::<Vec<_>>();
                assert_eq!(
                    selections,
                    [DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)]
                );
            });
        });
    }

    #[test]
    fn test_layout_line_numbers() -> Result<()> {
        App::test((), |mut app| async move {
            let layout_cache = TextLayoutCache::new(app.platform().fonts());
            let font_cache = app.font_cache();

            let buffer = app.add_model(|_| Buffer::new(0, sample_text(6, 6)));

            let settings = settings::channel(&font_cache).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            view.read(&app, |view, app| {
                let layouts = view.layout_line_numbers(1000.0, &font_cache, &layout_cache, app)?;
                assert_eq!(layouts.len(), 6);
                Result::<()>::Ok(())
            })?;

            Ok(())
        })
    }

    #[test]
    fn test_fold() -> Result<()> {
        App::test((), |mut app| async move {
            let buffer = app.add_model(|_| {
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
                )
            });
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            view.update(&mut app, |view, ctx| {
                view.select_ranges(&[DisplayPoint::new(8, 0)..DisplayPoint::new(12, 0)], ctx)?;
                view.fold(&(), ctx);
                assert_eq!(
                    view.text(ctx.app()),
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
                    view.text(ctx.app()),
                    "
                    impl Foo {…
                    }
                "
                    .unindent(),
                );

                view.unfold(&(), ctx);
                assert_eq!(
                    view.text(ctx.app()),
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
                assert_eq!(view.text(ctx.app()), buffer.as_ref(ctx).text());

                Ok::<(), Error>(())
            })?;

            Ok(())
        })
    }

    #[test]
    fn test_move_cursor() -> Result<()> {
        App::test((), |mut app| async move {
            let buffer = app.add_model(|_| Buffer::new(0, sample_text(6, 6)));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            buffer.update(&mut app, |buffer, ctx| {
                buffer.edit(
                    vec![
                        Point::new(1, 0)..Point::new(1, 0),
                        Point::new(1, 1)..Point::new(1, 1),
                    ],
                    "\t",
                    Some(ctx),
                )
            })?;

            view.update(&mut app, |view, ctx| {
                view.move_down(&(), ctx);
                assert_eq!(
                    view.selections(ctx.app()),
                    &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
                );
                view.move_right(&(), ctx);
                assert_eq!(
                    view.selections(ctx.app()),
                    &[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4)]
                );
                Ok::<(), Error>(())
            })?;

            Ok(())
        })
    }

    #[test]
    fn test_backspace() -> Result<()> {
        App::test((), |mut app| async move {
            let buffer = app.add_model(|_| {
                Buffer::new(0, "one two three\nfour five six\nseven eight nine\nten\n")
            });
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, view) =
                app.add_window(|ctx| BufferView::for_buffer(buffer.clone(), settings, ctx));

            view.update(&mut app, |view, ctx| -> Result<()> {
                view.select_ranges(
                    &[
                        // an empty selection - the preceding character is deleted
                        DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                        // one character selected - it is deleted
                        DisplayPoint::new(1, 3)..DisplayPoint::new(1, 4),
                        // a line suffix selected - it is deleted
                        DisplayPoint::new(2, 6)..DisplayPoint::new(3, 0),
                    ],
                    ctx,
                )?;
                view.backspace(&(), ctx);
                Ok(())
            })?;

            buffer.read(&mut app, |buffer, _| -> Result<()> {
                assert_eq!(buffer.text(), "oe two three\nfou five six\nseven ten\n");
                Ok(())
            })?;

            Ok(())
        })
    }

    impl BufferView {
        fn selections(&self, app: &AppContext) -> Vec<Range<DisplayPoint>> {
            self.selections_in_range(DisplayPoint::zero()..self.max_point(app), app)
                .collect::<Vec<_>>()
        }
    }
}

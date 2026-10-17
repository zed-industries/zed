mod actions;
pub(crate) mod autoscroll;
pub(crate) mod scroll_amount;

use crate::editor_settings::ScrollBeyondLastLine;
use crate::{
    Anchor, CursorShape, DisplayPoint, DisplayRow, Editor, EditorEvent, EditorMode, EditorSettings,
    MultiBufferSnapshot, RowExt, Selection, SelectionEffects, SizingBehavior, ToPoint,
    display_map::{DisplaySnapshot, PendingRowGeometry, RowLayout, ToDisplayPoint},
    element::{CursorLayout, LineWithInvisibles, SelectionGeometry},
    hover_popover::hide_hover,
    movement::TextLayoutDetails,
    persistence::EditorDb,
};
pub use autoscroll::{Autoscroll, AutoscrollStrategy};
use core::fmt::Debug;
use gpui::{
    Along, App, AppContext as _, Axis, Bounds, Context, Entity, EntityId, OngoingScroll, Pixels,
    Task, TouchPhase, Window, point,
};
use language::language_settings::{AllLanguageSettings, SoftWrap};
use language::{Bias, Point};
use multi_buffer::MultiBufferRow;
pub use scroll_amount::ScrollAmount;
use settings::Settings;
use std::{cmp::Ordering, ops::RangeInclusive, time::Duration};
use ui::scrollbars::ScrollbarAutoHide;
use unicode_segmentation::UnicodeSegmentation;
use util::ResultExt;
use workspace::{ItemId, WorkspaceId};

const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);

pub struct WasScrolled(pub(crate) bool);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SerializedScrollPosition {
    pub top_row: u32,
    pub offset: gpui::Point<ScrollOffset>,
    pub horizontal: Option<(Point, ScrollPixelOffset)>,
}

pub struct ScrollScreenDestination {
    pub top_row: DisplayRow,
    pub scroll_left: ScrollOffset,
    anchor: ScrollAnchor,
}

pub struct PaintedViewport<'a> {
    editor: &'a Editor,
    bounds: Bounds<Pixels>,
    clip_bounds: Bounds<Pixels>,
    content_origin: Pixels,
    scroll_left: ScrollPixelOffset,
    em_advance: Pixels,
    cursor_shape: CursorShape,
    placeholder: Option<DisplaySnapshot>,
}

impl PaintedViewport<'_> {
    pub fn with_cursor_shape(mut self, cursor_shape: CursorShape) -> Self {
        self.cursor_shape = cursor_shape;
        self
    }
    pub fn clip_selection(
        &self,
        snapshot: &DisplaySnapshot,
        selection: &Selection<DisplayPoint>,
        head: DisplayPoint,
        details: &TextLayoutDetails,
        window: &Window,
        cx: &App,
    ) -> Result<Option<DisplayPoint>, PendingRowGeometry> {
        self.clip_point_with(snapshot, head, details, window, cx, |head| {
            let mut selection = *selection;
            if self.editor.cursor_offset_on_selection && !selection.is_empty() {
                selection.set_head(head, selection.goal);
            } else {
                selection.collapse_to(head, selection.goal);
            }
            SelectionGeometry::new(
                selection.map(|point| point.to_point(snapshot)),
                self.editor.selections.line_mode(),
                self.editor.cursor_offset_on_selection,
                snapshot,
            )
            .head
        })
    }

    #[cfg(test)]
    fn clip_point(
        &self,
        snapshot: &DisplaySnapshot,
        point: DisplayPoint,
        details: &TextLayoutDetails,
        window: &Window,
        cx: &App,
    ) -> Result<Option<DisplayPoint>, PendingRowGeometry> {
        self.clip_point_with(snapshot, point, details, window, cx, |point| point)
    }

    fn clip_point_with(
        &self,
        snapshot: &DisplaySnapshot,
        point: DisplayPoint,
        details: &TextLayoutDetails,
        window: &Window,
        cx: &App,
        project: impl Fn(DisplayPoint) -> DisplayPoint,
    ) -> Result<Option<DisplayPoint>, PendingRowGeometry> {
        let point = snapshot.clip_point(point, Bias::Left);
        let cursor = project(point);
        let row = cursor.row();
        if row > snapshot.max_point().row() || self.clip_bounds.is_empty() {
            return Ok(None);
        }
        let clip = window.cover_bounds(self.clip_bounds);
        let clip_left = Pixels::from(clip.left().as_f32() / window.scale_factor());
        let clip_right = Pixels::from(clip.right().as_f32() / window.scale_factor());
        let layout = snapshot.layout_row(row, details)?;
        let cell_is_visible = |cursor: DisplayPoint| -> Result<bool, PendingRowGeometry> {
            if cursor.row() != row {
                return Ok(false);
            }
            let cell = self.cursor_x_range(snapshot, &layout, cursor, details, window, cx)?;
            Ok(*cell.start() < *cell.end()
                && *cell.start() >= clip_left
                && *cell.end() <= clip_right)
        };
        if cell_is_visible(cursor)? {
            return Ok(Some(point));
        }
        let line_len = snapshot.line_len(row) as usize;
        let last_column = line_len + usize::from(row < snapshot.max_point().row());
        let point_for_column = |column: usize| {
            if column > line_len {
                DisplayPoint::new(row.next_row(), 0)
            } else {
                DisplayPoint::new(row, column as u32)
            }
        };
        let column_for_point = |point: DisplayPoint| {
            if point.row() == row {
                Some(point.column() as usize)
            } else if point.row() == row.next_row() && point.column() == 0 {
                Some(line_len + 1)
            } else {
                None
            }
        };
        let alignment = LineWithInvisibles::alignment_offset_for_width(
            layout.width(),
            details.editor_style.text.text_align,
            self.bounds.size.width,
        );
        let column_for_edge = |edge: Pixels, inclusive: bool| {
            let mut start = 0;
            let mut end = last_column + 1;
            while start < end {
                let middle = start + (end - start) / 2;
                let cursor = project(snapshot.clip_point(point_for_column(middle), Bias::Left));
                let x = *self
                    .cell_x_range(
                        layout.x_for_index(cursor.column() as usize) + alignment,
                        Pixels::ZERO,
                        window,
                    )
                    .start();
                if cursor.row() < row
                    || (cursor.row() == row && (x < edge || (inclusive && x == edge)))
                {
                    start = middle + 1;
                } else {
                    end = middle;
                }
            }
            start
        };
        let start = column_for_edge(clip_left, false);
        let Some(end) = column_for_edge(clip_right, true).checked_sub(1) else {
            return Ok(None);
        };
        if start > end {
            return Ok(None);
        }
        let scan = |mut column: usize,
                    bias: Bias|
         -> Result<Option<DisplayPoint>, PendingRowGeometry> {
            loop {
                let candidate = snapshot.clip_point(point_for_column(column), bias);
                let Some(candidate_column) = column_for_point(candidate) else {
                    return Ok(None);
                };
                if candidate_column < start || candidate_column > end {
                    return Ok(None);
                }
                if cell_is_visible(project(candidate))? {
                    return Ok(Some(candidate));
                }
                let Some(next) = (match bias {
                    Bias::Left => candidate_column.checked_sub(1),
                    Bias::Right => candidate_column.checked_add(1),
                }) else {
                    return Ok(None);
                };
                if (bias == Bias::Left && next >= column) || (bias == Bias::Right && next <= column)
                {
                    return Ok(None);
                }
                column = next;
            }
        };
        let column = column_for_point(point).unwrap_or(last_column);
        if column < start {
            scan(start, Bias::Right)
        } else if let Some(point) = scan(column.min(end), Bias::Left)? {
            Ok(Some(point))
        } else {
            scan(column, Bias::Right)
        }
    }

    fn point_position(
        &self,
        snapshot: &DisplaySnapshot,
        point: DisplayPoint,
        details: &TextLayoutDetails,
        window: &Window,
        cx: &App,
    ) -> Result<Ordering, PendingRowGeometry> {
        let layout = snapshot.layout_row(point.row(), details)?;
        let cell = self.cursor_x_range(snapshot, &layout, point, details, window, cx)?;
        let clip = window.cover_bounds(self.clip_bounds);
        let left = Pixels::from(clip.left().as_f32() / window.scale_factor());
        let right = Pixels::from(clip.right().as_f32() / window.scale_factor());
        Ok(if *cell.end() <= left {
            Ordering::Less
        } else if self.clip_bounds.is_empty()
            || *cell.start() >= right
            || cell.start() == cell.end()
        {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }

    fn cursor_x_range(
        &self,
        snapshot: &DisplaySnapshot,
        layout: &RowLayout,
        cursor: DisplayPoint,
        details: &TextLayoutDetails,
        window: &Window,
        cx: &App,
    ) -> Result<RangeInclusive<Pixels>, PendingRowGeometry> {
        let placeholder = snapshot
            .buffer_snapshot()
            .is_empty()
            .then_some(self.placeholder.as_ref())
            .flatten();
        let placeholder_layout = placeholder
            .map(|snapshot| snapshot.layout_row(cursor.row(), details))
            .transpose()?;
        let layout = placeholder_layout.as_ref().unwrap_or(layout);
        let column = cursor.column() as usize;
        let alignment = LineWithInvisibles::alignment_offset_for_width(
            layout.width(),
            details.editor_style.text.text_align,
            self.bounds.size.width,
        );
        let x = layout.x_for_index(column) + alignment;
        let mut width = CursorLayout::cell_width(
            x,
            layout.x_for_index(column + 1) + alignment,
            self.em_advance,
        );
        if self.cursor_shape == CursorShape::Block {
            let buffer = snapshot.buffer_snapshot();
            let buffer_point = cursor.to_point(snapshot);
            let redacted = self
                .editor
                .redacted_ranges(
                    buffer.anchor_before(Point::new(buffer_point.row, 0))
                        ..buffer.anchor_after(Point::new(
                            buffer_point.row,
                            buffer.line_len(MultiBufferRow(buffer_point.row)),
                        )),
                    snapshot,
                    cx,
                )
                .iter()
                .any(|range| range.start <= cursor && cursor < range.end);
            if redacted {
                return Ok(self.cell_x_range(x, width, window));
            }
            if let RowLayout::Long(row) = layout {
                CursorLayout::long_row_width(
                    &mut width,
                    placeholder.unwrap_or(snapshot),
                    row,
                    cursor,
                    details
                        .editor_style
                        .text
                        .font_size
                        .to_pixels(details.rem_size),
                    &details.editor_style.text,
                    window,
                    cx,
                )?;
            } else if let Some(text) = snapshot.grapheme_at(cursor).or_else(|| {
                placeholder?
                    .text_chunks(DisplayRow(0))
                    .collect::<String>()
                    .graphemes(true)
                    .next()
                    .map(|text| gpui::SharedString::from(text.to_owned()))
            }) {
                let font_id = layout.cursor_font_id(
                    cursor.column(),
                    self.scroll_left + f64::from(self.bounds.left() - self.content_origin),
                    details.editor_style.text.text_align,
                    self.bounds.size.width,
                );
                CursorLayout::shape_text(
                    &mut width,
                    text,
                    font_id,
                    details
                        .editor_style
                        .text
                        .font_size
                        .to_pixels(details.rem_size),
                    None,
                    &details.editor_style.text,
                    window,
                    cx,
                );
            }
        }
        Ok(self.cell_x_range(x, width, window))
    }

    fn cell_x_range(
        &self,
        x: ScrollPixelOffset,
        width: Pixels,
        window: &Window,
    ) -> RangeInclusive<Pixels> {
        let bounds = window.pixel_snap_bounds(CursorLayout::shape_bounds(
            point(
                self.content_origin + Pixels::from(x - self.scroll_left),
                Pixels::ZERO,
            ),
            width,
            Pixels::ZERO,
            self.cursor_shape,
        ));
        window.pixel_snap(bounds.left())..=window.pixel_snap(bounds.right())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HorizontalScrollAnchor {
    anchor: Anchor,
    offset: ScrollPixelOffset,
    display_map_id: EntityId,
}

pub type ScrollOffset = f64;
pub type ScrollPixelOffset = f64;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollAnchor {
    pub offset: gpui::Point<ScrollOffset>,
    pub anchor: Anchor,
}

impl ScrollAnchor {
    pub(super) fn new() -> Self {
        Self {
            offset: gpui::Point::default(),
            anchor: Anchor::Min,
        }
    }

    pub fn scroll_position(&self, snapshot: &DisplaySnapshot) -> gpui::Point<ScrollOffset> {
        self.offset.apply_along(Axis::Vertical, |offset| {
            if self.anchor == Anchor::Min {
                0.
            } else {
                let scroll_top = self.anchor.to_display_point(snapshot).row().as_f64();
                (offset + scroll_top).max(0.)
            }
        })
    }

    pub fn top_row(&self, buffer: &MultiBufferSnapshot) -> u32 {
        self.anchor.to_point(buffer).row
    }
}

/// In the split diff view, the two sides share a ScrollAnchor using this struct.
/// Either side can set a ScrollAnchor that points to its own multibuffer, and we store the ID of the display map
/// that the last-written anchor came from so that we know how to resolve it to a DisplayPoint.
///
/// For normal editors, this just acts as a wrapper around a ScrollAnchor.
#[derive(Clone, Copy, Debug)]
pub struct SharedScrollAnchor {
    pub scroll_anchor: ScrollAnchor,
    pub display_map_id: Option<EntityId>,
    horizontal: Option<HorizontalScrollAnchor>,
    local: bool,
}

impl SharedScrollAnchor {
    pub fn scroll_position(&self, snapshot: &DisplaySnapshot) -> gpui::Point<ScrollOffset> {
        let snapshot = if let Some(display_map_id) = self.display_map_id
            && display_map_id != snapshot.display_map_id
        {
            let companion_snapshot = snapshot
                .companion_snapshot()
                .expect("shared scroll anchor references a non native display map, but snapshot has no companion");
            assert_eq!(
                companion_snapshot.display_map_id, display_map_id,
                "shared scroll anchor display map should match the snapshot's split companion"
            );
            companion_snapshot
        } else {
            snapshot
        };

        self.scroll_anchor.scroll_position(snapshot)
    }

    pub fn scroll_top_row(&self, snapshot: &DisplaySnapshot) -> DisplayRow {
        let snapshot = if let Some(display_map_id) = self.display_map_id
            && display_map_id != snapshot.display_map_id
        {
            let companion_snapshot = snapshot
                .companion_snapshot()
                .expect("shared scroll anchor references a non native display map, but snapshot has no companion");
            assert_eq!(
                companion_snapshot.display_map_id, display_map_id,
                "shared scroll anchor display map should match the snapshot's split companion"
            );
            companion_snapshot
        } else {
            snapshot
        };

        self.scroll_anchor.anchor.to_display_point(snapshot).row()
    }

    fn horizontal_scroll_left(
        &self,
        snapshot: &DisplaySnapshot,
    ) -> Result<Option<ScrollPixelOffset>, PendingRowGeometry> {
        let Some(horizontal) = self.horizontal else {
            return Ok(None);
        };
        let Some(point) = self.horizontal_display_point(snapshot) else {
            return Ok(None);
        };
        let Some(long_row) = snapshot.long_row(point.row())? else {
            return Ok(None);
        };
        let column = point.column().min(long_row.len());
        Ok(Some(long_row.x_for_column(column) - horizontal.offset))
    }

    fn native_horizontal(&self, snapshot: &DisplaySnapshot) -> Option<HorizontalScrollAnchor> {
        let horizontal = self.horizontal?;
        if horizontal.display_map_id == snapshot.display_map_id {
            return Some(horizontal);
        }
        let point = self.horizontal_display_point(snapshot)?;
        Some(HorizontalScrollAnchor {
            anchor: snapshot
                .buffer_snapshot()
                .anchor_before(point.to_offset(snapshot, Bias::Left)),
            offset: horizontal.offset,
            display_map_id: snapshot.display_map_id,
        })
    }

    fn horizontal_display_point(&self, snapshot: &DisplaySnapshot) -> Option<DisplayPoint> {
        let horizontal = self.horizontal?;
        if horizontal.display_map_id == snapshot.display_map_id {
            return Some(horizontal.anchor.to_display_point(snapshot));
        }
        let companion = snapshot
            .companion_snapshot()
            .filter(|companion| companion.display_map_id == horizontal.display_map_id)?;
        let companion_point = horizontal.anchor.to_point(companion.buffer_snapshot());
        if snapshot.is_block_line(companion_point.to_display_point(companion).row()) {
            return None;
        }
        let (point, past_line_end) =
            companion_display_point(snapshot, companion, companion_point, Bias::Left);
        (!past_line_end).then_some(point)
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum ScrollbarThumbState {
    #[default]
    Idle,
    Hovered,
    Dragging,
}

#[derive(PartialEq, Eq)]
pub struct ActiveScrollbarState {
    axis: Axis,
    thumb_state: ScrollbarThumbState,
}

impl ActiveScrollbarState {
    pub fn new(axis: Axis, thumb_state: ScrollbarThumbState) -> Self {
        ActiveScrollbarState { axis, thumb_state }
    }

    pub fn thumb_state_for_axis(&self, axis: Axis) -> Option<ScrollbarThumbState> {
        (self.axis == axis).then_some(self.thumb_state)
    }
}

pub struct ScrollManager {
    pub(crate) vertical_scroll_margin: ScrollOffset,
    anchor: Entity<SharedScrollAnchor>,
    /// Value to be used for clamping the x component of the SharedScrollAnchor's offset.
    ///
    /// We store this outside the SharedScrollAnchor so that the two sides of a split diff can share
    /// a horizontal scroll offset that may be out of range for one of the editors (when one side is wider than the other).
    /// Each side separately clamps the x component using its own scroll_max_x when reading from the SharedScrollAnchor.
    scroll_max_x: Option<f64>,
    ongoing: OngoingScroll,
    /// The second element indicates whether the autoscroll request is local
    /// (true) or remote (false). Local requests are initiated by user actions,
    /// while remote requests come from external sources.
    autoscroll_request: Option<(Autoscroll, bool)>,
    horizontal_scroll_x: Option<(HorizontalScrollAnchor, ScrollOffset)>,
    pending_horizontal_geometry: Option<(PendingRowGeometry, Task<()>)>,
    last_autoscroll: Option<(
        gpui::Point<ScrollOffset>,
        ScrollOffset,
        ScrollOffset,
        AutoscrollStrategy,
    )>,
    show_scrollbars: bool,
    hide_scrollbar_task: Option<Task<()>>,
    active_scrollbar: Option<ActiveScrollbarState>,
    visible_line_count: Option<f64>,
    visible_column_count: Option<f64>,
    forbid_vertical_scroll: bool,
    minimap_thumb_state: Option<ScrollbarThumbState>,
    persisted_position: Option<(Option<WorkspaceId>, SerializedScrollPosition)>,
    _save_scroll_position_task: Task<()>,
}

impl ScrollManager {
    pub fn new(cx: &mut Context<Editor>) -> Self {
        let anchor = cx.new(|_| SharedScrollAnchor {
            scroll_anchor: ScrollAnchor::new(),
            display_map_id: None,
            horizontal: None,
            local: true,
        });
        ScrollManager {
            vertical_scroll_margin: EditorSettings::get_global(cx).vertical_scroll_margin,
            anchor,
            scroll_max_x: None,
            ongoing: OngoingScroll::default(),
            autoscroll_request: None,
            horizontal_scroll_x: None,
            pending_horizontal_geometry: None,
            show_scrollbars: true,
            hide_scrollbar_task: None,
            active_scrollbar: None,
            last_autoscroll: None,
            visible_line_count: None,
            visible_column_count: None,
            forbid_vertical_scroll: false,
            minimap_thumb_state: None,
            persisted_position: None,
            _save_scroll_position_task: Task::ready(()),
        }
    }

    pub fn set_native_display_map_id(
        &mut self,
        display_map_id: EntityId,
        cx: &mut Context<Editor>,
    ) {
        self.anchor.update(cx, |shared, _| {
            if shared.display_map_id.is_none() {
                shared.display_map_id = Some(display_map_id);
            }
        });
    }

    pub fn clone_state(
        &mut self,
        other: &Self,
        other_snapshot: &DisplaySnapshot,
        my_snapshot: &DisplaySnapshot,
        cx: &mut Context<Editor>,
    ) {
        let native = other.native_shared_anchor(other_snapshot, cx);
        let cloned = SharedScrollAnchor {
            display_map_id: Some(my_snapshot.display_map_id),
            horizontal: native.horizontal.map(|horizontal| HorizontalScrollAnchor {
                display_map_id: my_snapshot.display_map_id,
                ..horizontal
            }),
            ..native
        };
        self.anchor.update(cx, |shared, _| *shared = cloned);
        self.ongoing = other.ongoing;
        self.cache_native_horizontal(&cloned);
    }

    pub fn offset(&self, cx: &App) -> gpui::Point<f64> {
        self.effective_offset(self.anchor.read(cx))
    }

    fn effective_offset(&self, shared: &SharedScrollAnchor) -> gpui::Point<f64> {
        let mut offset = shared.scroll_anchor.offset;
        if let Some((horizontal, x)) = self.horizontal_scroll_x
            && shared.horizontal == Some(horizontal)
        {
            offset.x = x;
        }
        if let Some(max_x) = self.scroll_max_x {
            offset.x = offset.x.min(max_x);
        }
        offset
    }

    /// Get a ScrollAnchor whose `anchor` field is guaranteed to point into the multibuffer for the provided snapshot.
    ///
    /// For normal editors, this just retrieves the internal ScrollAnchor and is lossless. When the editor is part of a split diff,
    /// we may need to translate the anchor to point to the "native" multibuffer first. That translation is lossy,
    /// so this method should be used sparingly---if you just need a scroll position or display point, call the appropriate helper method instead,
    /// since they can losslessly handle the case where the ScrollAnchor was last set from the other side.
    pub fn native_anchor(&self, snapshot: &DisplaySnapshot, cx: &App) -> ScrollAnchor {
        let shared = self.anchor.read(cx);

        let mut result = if let Some(display_map_id) = shared.display_map_id
            && display_map_id != snapshot.display_map_id
        {
            let companion_snapshot = snapshot
                .companion_snapshot()
                .expect("shared scroll anchor references a non native display map, but the snapshot has no companion");
            assert_eq!(
                companion_snapshot.display_map_id, display_map_id,
                "shared scroll anchor display map should match the companion used for native anchor conversion"
            );

            let mut display_point = shared
                .scroll_anchor
                .anchor
                .to_display_point(companion_snapshot);
            *display_point.column_mut() = 0;
            let buffer_point = snapshot.display_point_to_point(display_point, Bias::Left);
            let anchor = snapshot.buffer_snapshot().anchor_before(buffer_point);
            ScrollAnchor {
                anchor,
                offset: shared.scroll_anchor.offset,
            }
        } else {
            shared.scroll_anchor
        };

        result.offset = self.effective_offset(shared);
        result
    }

    pub fn shared_scroll_anchor(&self, cx: &App) -> SharedScrollAnchor {
        let mut shared = *self.anchor.read(cx);
        shared.scroll_anchor.offset = self.effective_offset(&shared);
        shared
    }

    pub fn scroll_top_row(&self, snapshot: &DisplaySnapshot, cx: &App) -> DisplayRow {
        self.anchor.read(cx).scroll_top_row(snapshot)
    }

    pub fn scroll_anchor_entity(&self) -> Entity<SharedScrollAnchor> {
        self.anchor.clone()
    }

    pub fn set_shared_scroll_anchor(&mut self, entity: Entity<SharedScrollAnchor>) {
        self.anchor = entity;
    }

    pub fn unshare_scroll_anchor(&mut self, snapshot: &DisplaySnapshot, cx: &mut Context<Editor>) {
        let native = self.native_shared_anchor(snapshot, cx);
        self.anchor = cx.new(|_| native);
        self.cache_native_horizontal(&native);
    }

    pub(crate) fn make_scroll_anchor_native(
        &mut self,
        snapshot: &DisplaySnapshot,
        cx: &mut Context<Editor>,
    ) {
        let native = self.native_shared_anchor(snapshot, cx);
        self.anchor.update(cx, |shared, _| *shared = native);
        self.cache_native_horizontal(&native);
    }

    fn native_shared_anchor(&self, snapshot: &DisplaySnapshot, cx: &App) -> SharedScrollAnchor {
        let shared = self.anchor.read(cx);
        SharedScrollAnchor {
            scroll_anchor: self.native_anchor(snapshot, cx),
            display_map_id: Some(snapshot.display_map_id),
            horizontal: shared.native_horizontal(snapshot),
            local: shared.local,
        }
    }

    fn cache_native_horizontal(&mut self, native: &SharedScrollAnchor) {
        self.horizontal_scroll_x = native
            .horizontal
            .map(|horizontal| (horizontal, native.scroll_anchor.offset.x));
    }

    pub fn filter_scroll_delta(
        &mut self,
        delta: &mut gpui::Point<Pixels>,
        touch_phase: TouchPhase,
    ) {
        self.ongoing.filter(delta, touch_phase);
    }

    pub fn scroll_position(
        &self,
        snapshot: &DisplaySnapshot,
        cx: &App,
    ) -> gpui::Point<ScrollOffset> {
        let shared = self.anchor.read(cx);
        let mut pos = shared.scroll_position(snapshot);
        pos.x = self.effective_offset(shared).x;
        pos
    }

    fn set_scroll_position(
        &mut self,
        scroll_position: gpui::Point<ScrollOffset>,
        map: &DisplaySnapshot,
        scroll_beyond_last_line: ScrollBeyondLastLine,
        local: bool,
        autoscroll: bool,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> WasScrolled {
        let anchor = self.anchor_for_scroll_position(scroll_position, map, scroll_beyond_last_line);
        self.set_anchor(anchor, map, local, autoscroll, window, cx)
    }

    fn anchor_for_scroll_position(
        &self,
        scroll_position: gpui::Point<ScrollOffset>,
        map: &DisplaySnapshot,
        scroll_beyond_last_line: ScrollBeyondLastLine,
    ) -> ScrollAnchor {
        let scroll_top = scroll_position.y.max(0.);
        let scroll_top = match scroll_beyond_last_line {
            ScrollBeyondLastLine::OnePage => scroll_top,
            ScrollBeyondLastLine::Off => {
                if let Some(height_in_lines) = self.visible_line_count {
                    let max_row = map.max_point().row().as_f64();
                    scroll_top.min(max_row - height_in_lines + 1.).max(0.)
                } else {
                    scroll_top
                }
            }
            ScrollBeyondLastLine::VerticalScrollMargin => {
                if let Some(height_in_lines) = self.visible_line_count {
                    let max_row = map.max_point().row().as_f64();
                    scroll_top
                        .min(max_row - height_in_lines + 1. + self.vertical_scroll_margin)
                        .max(0.)
                } else {
                    scroll_top
                }
            }
        };
        let scroll_top_row = DisplayRow(scroll_top as u32);
        let scroll_top_buffer_point = map
            .clip_point(
                DisplayPoint::new(scroll_top_row, scroll_position.x as u32),
                Bias::Left,
            )
            .to_point(map);
        let top_anchor = map.buffer_snapshot().anchor_before(scroll_top_buffer_point);

        ScrollAnchor {
            anchor: top_anchor,
            offset: point(
                scroll_position.x.max(0.),
                scroll_top - top_anchor.to_display_point(map).row().as_f64(),
            ),
        }
    }

    fn set_anchor(
        &mut self,
        anchor: ScrollAnchor,
        display_map: &DisplaySnapshot,
        local: bool,
        autoscroll: bool,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> WasScrolled {
        let adjusted_anchor = if self.forbid_vertical_scroll {
            let current = self.anchor.read(cx);
            ScrollAnchor {
                offset: gpui::Point::new(anchor.offset.x, current.scroll_anchor.offset.y),
                anchor: current.scroll_anchor.anchor,
            }
        } else {
            anchor
        };

        self.scroll_max_x.take();
        self.autoscroll_request.take();

        let current = self.anchor.read(cx);
        if current.scroll_anchor == adjusted_anchor {
            return WasScrolled(false);
        }

        self.anchor.update(cx, |shared, _| {
            shared.scroll_anchor = adjusted_anchor;
            shared.display_map_id = Some(display_map.display_map_id);
            shared.horizontal = None;
            shared.local = local;
        });
        self.horizontal_scroll_x = None;
        cx.emit(EditorEvent::ScrollPositionChanged { local, autoscroll });
        self.show_scrollbars(window, cx);
        cx.notify();

        WasScrolled(true)
    }

    pub(crate) fn serialized_position(
        &self,
        snapshot: &DisplaySnapshot,
        cx: &App,
    ) -> SerializedScrollPosition {
        let anchor = self.native_anchor(snapshot, cx);
        let buffer = snapshot.buffer_snapshot();
        SerializedScrollPosition {
            top_row: anchor.top_row(buffer),
            offset: anchor.offset,
            horizontal: self
                .anchor
                .read(cx)
                .native_horizontal(snapshot)
                .map(|horizontal| (horizontal.anchor.to_point(buffer), horizontal.offset)),
        }
    }

    fn persist_position(
        &mut self,
        position: SerializedScrollPosition,
        workspace_id: Option<WorkspaceId>,
        cx: &mut Context<Editor>,
    ) -> bool {
        let persisted = Some((workspace_id, position));
        if self.persisted_position == persisted {
            return false;
        }
        self.persisted_position = persisted;
        let Some(workspace_id) = workspace_id else {
            return true;
        };
        let item_id = cx.entity().entity_id().as_u64() as ItemId;
        let executor = cx.background_executor().clone();
        let db = EditorDb::global(cx);
        self._save_scroll_position_task = cx.background_executor().spawn(async move {
            executor.timer(Duration::from_millis(10)).await;
            log::debug!(
                "Saving scroll position for item {item_id:?} in workspace {workspace_id:?}"
            );
            db.save_scroll_position(
                item_id,
                workspace_id,
                position.top_row,
                position.offset.x,
                position.offset.y,
                position.horizontal.map(|(point, _)| point.row),
                position.horizontal.map(|(point, _)| point.column),
                position.horizontal.map(|(_, offset)| offset),
            )
            .await
            .log_err();
        });
        true
    }

    pub fn show_scrollbars(&mut self, window: &mut Window, cx: &mut Context<Editor>) {
        if !self.show_scrollbars {
            self.show_scrollbars = true;
            cx.notify();
        }

        if cx.default_global::<ScrollbarAutoHide>().should_hide() {
            self.hide_scrollbar_task = Some(cx.spawn_in(window, async move |editor, cx| {
                cx.background_executor()
                    .timer(SCROLLBAR_SHOW_INTERVAL)
                    .await;
                editor
                    .update(cx, |editor, cx| {
                        editor.scroll_manager.show_scrollbars = false;
                        cx.notify();
                    })
                    .log_err();
            }));
        } else {
            self.hide_scrollbar_task = None;
        }
    }

    pub fn scrollbars_visible(&self) -> bool {
        self.show_scrollbars
    }

    pub fn has_autoscroll_request(&self) -> bool {
        self.autoscroll_request.is_some()
    }

    pub fn take_autoscroll_request(&mut self) -> Option<(Autoscroll, bool)> {
        self.autoscroll_request.take()
    }

    pub(crate) fn restore_autoscroll_request(&mut self, request: (Autoscroll, bool)) {
        self.autoscroll_request = Some(request);
    }

    pub(crate) fn resolve_horizontal_anchor(
        &mut self,
        snapshot: &DisplaySnapshot,
        em_layout_width: Pixels,
        cx: &mut Context<Editor>,
    ) -> bool {
        let Some(is_native) = self.resolve_horizontal_scroll_x(snapshot, em_layout_width, cx)
        else {
            return false;
        };
        if is_native {
            cx.emit(EditorEvent::ScrollPositionChanged {
                local: self.is_local_position(cx),
                autoscroll: false,
            });
        }
        true
    }

    pub(crate) fn is_local_position(&self, cx: &App) -> bool {
        self.anchor.read(cx).local
    }

    fn resolve_horizontal_scroll_x(
        &mut self,
        snapshot: &DisplaySnapshot,
        em_layout_width: Pixels,
        cx: &mut Context<Editor>,
    ) -> Option<bool> {
        let em_layout_width = ScrollPixelOffset::from(em_layout_width);
        let shared = *self.anchor.read(cx);
        let Some(horizontal) = shared.horizontal else {
            self.pending_horizontal_geometry = None;
            return None;
        };
        if em_layout_width <= 0. {
            return None;
        }
        let previous_x = self.effective_offset(&shared).x;
        let is_native = horizontal.display_map_id == snapshot.display_map_id;
        let scroll_left = match shared.horizontal_scroll_left(snapshot) {
            Ok(Some(scroll_left)) => scroll_left,
            Ok(None) => {
                self.pending_horizontal_geometry = None;
                self.horizontal_scroll_x
                    .take_if(|(resolved, _)| *resolved == horizontal)?;
                return (self.effective_offset(&shared).x != previous_x).then_some(is_native);
            }
            Err(pending) => {
                if !self
                    .pending_horizontal_geometry
                    .as_ref()
                    .is_some_and(|(current, _)| {
                        current.row() == pending.row() && current.is_current(snapshot)
                    })
                {
                    let task = cx.spawn({
                        let pending = pending.clone();
                        async move |editor, cx| {
                            if editor
                                .update(cx, |editor, cx| {
                                    editor.display_map.update(cx, |map, cx| {
                                        map.request_row_geometry(pending.clone(), cx);
                                    });
                                })
                                .is_err()
                            {
                                return;
                            }
                            pending.wait().await;
                            editor
                                .update(cx, |editor, cx| {
                                    editor.scroll_manager.pending_horizontal_geometry = None;
                                    cx.notify();
                                })
                                .ok();
                        }
                    });
                    self.pending_horizontal_geometry = Some((pending, task));
                }
                return None;
            }
        };
        self.pending_horizontal_geometry = None;
        let x = (scroll_left / em_layout_width).max(0.);
        self.horizontal_scroll_x = Some((horizontal, x));
        if is_native && shared.scroll_anchor.offset.x != x {
            self.anchor
                .update(cx, |shared, _| shared.scroll_anchor.offset.x = x);
        }
        (self.effective_offset(&shared).x != previous_x).then_some(is_native)
    }

    pub(crate) fn store_horizontal_anchor(
        &mut self,
        anchor: Option<(Anchor, ScrollPixelOffset)>,
        snapshot: &DisplaySnapshot,
        cx: &mut Context<Editor>,
    ) {
        let shared = *self.anchor.read(cx);
        if shared
            .display_map_id
            .is_some_and(|writer| writer != snapshot.display_map_id)
        {
            return;
        }
        if let Some(current) = shared.horizontal {
            let resolved = self
                .horizontal_scroll_x
                .is_some_and(|(resolved, _)| resolved == current);
            if resolved || current.display_map_id != snapshot.display_map_id {
                return;
            }
        }
        let horizontal = anchor.map(|(anchor, offset)| HorizontalScrollAnchor {
            anchor,
            offset,
            display_map_id: snapshot.display_map_id,
        });
        if horizontal == shared.horizontal {
            return;
        }
        let offset = self.effective_offset(&shared);
        self.anchor
            .update(cx, |shared, _| shared.horizontal = horizontal);
        self.horizontal_scroll_x = horizontal.map(|horizontal| (horizontal, offset.x));
    }

    fn restore_anchor(
        &mut self,
        saved: SharedScrollAnchor,
        display_map: &DisplaySnapshot,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> WasScrolled {
        let is_resolvable = |display_map_id: EntityId| {
            display_map_id == display_map.display_map_id
                || display_map
                    .companion_snapshot()
                    .is_some_and(|companion| companion.display_map_id == display_map_id)
        };
        if saved
            .display_map_id
            .is_some_and(|display_map_id| !is_resolvable(display_map_id))
        {
            return WasScrolled(false);
        }
        let current = *self.anchor.read(cx);
        let mut restored = SharedScrollAnchor {
            horizontal: saved
                .horizontal
                .filter(|horizontal| is_resolvable(horizontal.display_map_id)),
            local: true,
            ..saved
        };
        if self.forbid_vertical_scroll {
            restored.scroll_anchor = ScrollAnchor {
                anchor: current.scroll_anchor.anchor,
                offset: point(saved.scroll_anchor.offset.x, current.scroll_anchor.offset.y),
            };
            restored.display_map_id = current.display_map_id;
        }

        self.scroll_max_x.take();
        self.autoscroll_request.take();

        if current.scroll_anchor == restored.scroll_anchor
            && current.display_map_id == restored.display_map_id
            && current.horizontal == restored.horizontal
        {
            return WasScrolled(false);
        }

        self.anchor.update(cx, |shared, _| *shared = restored);
        self.horizontal_scroll_x = None;
        cx.emit(EditorEvent::ScrollPositionChanged {
            local: true,
            autoscroll: false,
        });
        self.show_scrollbars(window, cx);
        cx.notify();

        WasScrolled(true)
    }

    pub fn active_scrollbar_state(&self) -> Option<&ActiveScrollbarState> {
        self.active_scrollbar.as_ref()
    }

    pub fn dragging_scrollbar_axis(&self) -> Option<Axis> {
        self.active_scrollbar
            .as_ref()
            .filter(|scrollbar| scrollbar.thumb_state == ScrollbarThumbState::Dragging)
            .map(|scrollbar| scrollbar.axis)
    }

    pub fn any_scrollbar_dragged(&self) -> bool {
        self.active_scrollbar
            .as_ref()
            .is_some_and(|scrollbar| scrollbar.thumb_state == ScrollbarThumbState::Dragging)
    }

    pub fn set_hovered_scroll_thumb_axis(&mut self, axis: Axis, cx: &mut Context<Editor>) {
        self.update_active_scrollbar_state(
            Some(ActiveScrollbarState::new(
                axis,
                ScrollbarThumbState::Hovered,
            )),
            cx,
        );
    }

    pub fn set_dragged_scroll_thumb_axis(&mut self, axis: Axis, cx: &mut Context<Editor>) {
        self.update_active_scrollbar_state(
            Some(ActiveScrollbarState::new(
                axis,
                ScrollbarThumbState::Dragging,
            )),
            cx,
        );
    }

    pub fn reset_scrollbar_state(&mut self, cx: &mut Context<Editor>) {
        self.update_active_scrollbar_state(None, cx);
    }

    fn update_active_scrollbar_state(
        &mut self,
        new_state: Option<ActiveScrollbarState>,
        cx: &mut Context<Editor>,
    ) {
        if self.active_scrollbar != new_state {
            self.active_scrollbar = new_state;
            cx.notify();
        }
    }

    pub fn set_is_hovering_minimap_thumb(&mut self, hovered: bool, cx: &mut Context<Editor>) {
        self.update_minimap_thumb_state(
            Some(if hovered {
                ScrollbarThumbState::Hovered
            } else {
                ScrollbarThumbState::Idle
            }),
            cx,
        );
    }

    pub fn set_is_dragging_minimap(&mut self, cx: &mut Context<Editor>) {
        self.update_minimap_thumb_state(Some(ScrollbarThumbState::Dragging), cx);
    }

    pub fn hide_minimap_thumb(&mut self, cx: &mut Context<Editor>) {
        self.update_minimap_thumb_state(None, cx);
    }

    pub fn is_dragging_minimap(&self) -> bool {
        self.minimap_thumb_state
            .is_some_and(|state| state == ScrollbarThumbState::Dragging)
    }

    fn update_minimap_thumb_state(
        &mut self,
        thumb_state: Option<ScrollbarThumbState>,
        cx: &mut Context<Editor>,
    ) {
        if self.minimap_thumb_state != thumb_state {
            self.minimap_thumb_state = thumb_state;
            cx.notify();
        }
    }

    pub fn minimap_thumb_state(&self) -> Option<ScrollbarThumbState> {
        self.minimap_thumb_state
    }

    pub fn clamp_scroll_left(&mut self, max: f64, cx: &App) -> bool {
        self.scroll_max_x = None;
        let current_x = self.effective_offset(self.anchor.read(cx)).x;
        self.scroll_max_x = Some(max);
        current_x > max
    }

    pub fn set_forbid_vertical_scroll(&mut self, forbid: bool) {
        self.forbid_vertical_scroll = forbid;
    }

    pub fn forbid_vertical_scroll(&self) -> bool {
        self.forbid_vertical_scroll
    }
}

impl Editor {
    pub fn has_autoscroll_request(&self) -> bool {
        self.scroll_manager.has_autoscroll_request()
    }

    pub fn set_forbid_vertical_scroll(&mut self, forbid: bool) {
        self.scroll_manager.set_forbid_vertical_scroll(forbid);
    }

    pub fn scroll_top_row(&self, snapshot: &DisplaySnapshot, cx: &App) -> DisplayRow {
        self.scroll_manager.scroll_top_row(snapshot, cx)
    }

    pub fn vertical_scroll_margin(&self) -> usize {
        self.scroll_manager.vertical_scroll_margin as usize
    }

    pub(crate) fn scroll_beyond_last_line(&self, cx: &App) -> ScrollBeyondLastLine {
        match self.mode {
            EditorMode::Minimap { .. }
            | EditorMode::Full {
                sizing_behavior: SizingBehavior::Default,
                ..
            } => EditorSettings::get_global(cx).scroll_beyond_last_line,

            EditorMode::Full { .. } | EditorMode::SingleLine | EditorMode::AutoHeight { .. } => {
                ScrollBeyondLastLine::Off
            }
        }
    }

    pub fn set_vertical_scroll_margin(&mut self, margin_rows: usize, cx: &mut Context<Self>) {
        self.scroll_manager.vertical_scroll_margin = margin_rows as f64;
        cx.notify();
    }

    pub fn visible_line_count(&self) -> Option<f64> {
        self.scroll_manager.visible_line_count
    }

    pub fn visible_row_count(&self) -> Option<u32> {
        self.visible_line_count()
            .map(|line_count| line_count as u32 - 1)
    }

    pub fn visible_column_count(&self) -> Option<f64> {
        self.scroll_manager.visible_column_count
    }

    pub(crate) fn set_visible_line_count(
        &mut self,
        lines: f64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.scroll_manager.visible_line_count.replace(lines) {
            None => self.update_data_on_scroll(false, window, cx),
            Some(previous_lines) if previous_lines < lines => {
                self.update_data_on_scroll(true, window, cx)
            }
            Some(_) => {}
        }
    }

    pub(crate) fn set_visible_column_count(&mut self, columns: f64) -> bool {
        self.scroll_manager
            .visible_column_count
            .replace(columns)
            .is_some_and(|previous_columns| previous_columns < columns)
    }

    pub fn apply_scroll_delta(
        &mut self,
        scroll_delta: gpui::Point<f32>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut delta = scroll_delta;
        if self.scroll_manager.forbid_vertical_scroll {
            delta.y = 0.0;
        }
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let position = self.scroll_manager.scroll_position(&display_map, cx) + delta.map(f64::from);
        self.set_scroll_position_taking_display_map(position, true, false, display_map, window, cx);
    }

    pub fn set_scroll_position(
        &mut self,
        scroll_position: gpui::Point<ScrollOffset>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> WasScrolled {
        let mut position = scroll_position;
        if self.scroll_manager.forbid_vertical_scroll {
            let current_position = self.scroll_position(cx);
            position.y = current_position.y;
        }
        self.set_scroll_position_internal(position, true, false, window, cx)
    }

    /// Scrolls so that `row` is at the top of the editor view.
    pub fn set_scroll_top_row(
        &mut self,
        row: DisplayRow,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let snapshot = self.snapshot(window, cx).display_snapshot;
        let new_screen_top = DisplayPoint::new(row, 0);
        let new_screen_top = new_screen_top.to_offset(&snapshot, Bias::Left);
        let new_anchor = snapshot.buffer_snapshot().anchor_before(new_screen_top);

        self.set_scroll_anchor(
            ScrollAnchor {
                anchor: new_anchor,
                offset: Default::default(),
            },
            window,
            cx,
        );
    }

    pub(crate) fn set_scroll_position_internal(
        &mut self,
        scroll_position: gpui::Point<ScrollOffset>,
        local: bool,
        autoscroll: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> WasScrolled {
        let map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let was_scrolled = self.set_scroll_position_taking_display_map(
            scroll_position,
            local,
            autoscroll,
            map,
            window,
            cx,
        );

        was_scrolled
    }

    fn set_scroll_position_taking_display_map(
        &mut self,
        scroll_position: gpui::Point<ScrollOffset>,
        local: bool,
        autoscroll: bool,
        display_map: DisplaySnapshot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> WasScrolled {
        hide_hover(self, cx);

        self.edit_prediction_preview
            .set_previous_scroll_position(None);

        let adjusted_position = if self.scroll_manager.forbid_vertical_scroll {
            let current_position = self.scroll_manager.scroll_position(&display_map, cx);
            gpui::Point::new(scroll_position.x, current_position.y)
        } else {
            scroll_position
        };
        let scroll_beyond_last_line = self.scroll_beyond_last_line(cx);
        self.scroll_manager.set_scroll_position(
            adjusted_position,
            &display_map,
            scroll_beyond_last_line,
            local,
            autoscroll,
            window,
            cx,
        )
    }

    pub fn scroll_position(&self, cx: &mut Context<Self>) -> gpui::Point<ScrollOffset> {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.scroll_manager.scroll_position(&display_map, cx)
    }

    pub fn set_scroll_anchor(
        &mut self,
        scroll_anchor: ScrollAnchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        hide_hover(self, cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.scroll_manager
            .set_anchor(scroll_anchor, &display_map, true, false, window, cx);
    }

    pub(crate) fn persist_scroll_position(
        &mut self,
        snapshot: &DisplaySnapshot,
        cx: &mut Context<Self>,
    ) {
        let position = self.scroll_manager.serialized_position(snapshot, cx);
        let workspace_id = self.workspace.as_ref().and_then(|workspace| workspace.1);
        if self
            .scroll_manager
            .persist_position(position, workspace_id, cx)
            && self.scroll_manager.is_local_position(cx)
        {
            self.update_restoration_data(cx, move |data| data.scroll_position = position);
        }
    }

    pub(crate) fn restore_scroll_position(
        &mut self,
        position: SerializedScrollPosition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let anchor = self
            .buffer()
            .read(cx)
            .snapshot(cx)
            .anchor_before(Point::new(position.top_row, 0));
        self.restore_native_scroll_position(
            ScrollAnchor {
                anchor,
                offset: position.offset,
            },
            position.horizontal,
            window,
            cx,
        );
    }

    pub(crate) fn restore_native_scroll_position(
        &mut self,
        scroll_anchor: ScrollAnchor,
        horizontal: Option<(Point, ScrollPixelOffset)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = self.buffer().read(cx).snapshot(cx);
        let display_map_id = self.display_map.entity_id();
        let saved = SharedScrollAnchor {
            scroll_anchor,
            display_map_id: Some(display_map_id),
            horizontal: horizontal.map(|(point, offset)| HorizontalScrollAnchor {
                anchor: buffer.anchor_before(buffer.clip_point(point, Bias::Left)),
                offset,
                display_map_id,
            }),
            local: true,
        };
        self.restore_scroll_anchor(saved, window, cx);
    }

    pub(crate) fn save_scroll_anchor(&self, cx: &App) -> SharedScrollAnchor {
        self.scroll_manager.shared_scroll_anchor(cx)
    }

    pub(crate) fn restore_scroll_anchor(
        &mut self,
        saved: SharedScrollAnchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> WasScrolled {
        hide_hover(self, cx);
        self.edit_prediction_preview
            .set_previous_scroll_position(None);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.scroll_manager
            .restore_anchor(saved, &display_map, window, cx)
    }

    pub(crate) fn set_scroll_anchor_remote(
        &mut self,
        scroll_anchor: ScrollAnchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        hide_hover(self, cx);
        let buffer_snapshot = self.buffer().read(cx).snapshot(cx);
        if !scroll_anchor.anchor.is_valid(&buffer_snapshot) {
            log::warn!("Invalid scroll anchor: {:?}", scroll_anchor);
            return;
        }
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.scroll_manager
            .set_anchor(scroll_anchor, &display_map, false, false, window, cx);
    }

    pub fn scroll_screen(
        &mut self,
        amount: &ScrollAmount,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }

        if self.take_rename(true, window, cx).is_some() {
            return;
        }

        let snapshot = self.display_snapshot(cx);
        if let Some(destination) = self.scroll_screen_destination(amount, &snapshot, cx) {
            hide_hover(self, cx);
            self.edit_prediction_preview
                .set_previous_scroll_position(None);
            self.scroll_manager
                .set_anchor(destination.anchor, &snapshot, true, false, window, cx);
        }
    }

    pub fn scroll_screen_destination(
        &self,
        amount: &ScrollAmount,
        snapshot: &DisplaySnapshot,
        cx: &App,
    ) -> Option<ScrollScreenDestination> {
        let mut current_position = self.scroll_manager.scroll_position(snapshot, cx);
        if self.mode == EditorMode::SingleLine || self.pending_rename.is_some() {
            return None;
        }
        let visible_line_count = self.visible_line_count()?;
        let mut visible_column_count = self.visible_column_count()?;
        if let Some((em_layout_width, em_advance)) = self.em_widths(cx) {
            visible_column_count *= em_advance / em_layout_width;
        }

        // If the user has a preferred line length, and has the editor
        // configured to wrap at the preferred line length, or bounded to it,
        // use that value over the visible column count. This was mostly done so
        // that tests could actually be written for vim's `z l`, `z h`, `z
        // shift-l` and `z shift-h` commands, as there wasn't a good way to
        // configure the editor to only display a certain number of columns. If
        // that ever happens, this could probably be removed.
        let settings = AllLanguageSettings::get_global(cx);
        if matches!(settings.defaults.soft_wrap, SoftWrap::Bounded)
            && (settings.defaults.preferred_line_length as f64) < visible_column_count
        {
            visible_column_count = settings.defaults.preferred_line_length as f64;
        }

        // If the scroll position is currently at the left edge of the document
        // (x == 0.0) and the intent is to scroll right, the gutter's margin
        // should first be added to the current position, otherwise the cursor
        // will end at the column position minus the margin, which looks off.
        if current_position.x == 0.0
            && amount.columns(visible_column_count) > 0.
            && let Some(last_position_map) = &self.last_position_map
        {
            current_position.x +=
                f64::from(self.gutter_dimensions.margin / last_position_map.em_layout_width);
        }
        let new_position = current_position
            + point(
                amount.columns(visible_column_count),
                amount.lines(visible_line_count),
            );
        let new_position = if self.scroll_manager.forbid_vertical_scroll {
            point(new_position.x, current_position.y)
        } else {
            new_position
        };
        let mut anchor = self.scroll_manager.anchor_for_scroll_position(
            new_position,
            snapshot,
            self.scroll_beyond_last_line(cx),
        );
        if self.scroll_manager.forbid_vertical_scroll {
            let current = self.scroll_manager.anchor.read(cx).scroll_anchor;
            anchor.anchor = current.anchor;
            anchor.offset.y = current.offset.y;
        }
        let current = self.scroll_manager.anchor.read(cx);
        let (top_row, scroll_left) = if current.scroll_anchor == anchor {
            (
                current.scroll_top_row(snapshot),
                self.scroll_manager.effective_offset(current).x,
            )
        } else {
            (
                anchor.anchor.to_display_point(snapshot).row(),
                anchor.offset.x,
            )
        };
        Some(ScrollScreenDestination {
            top_row,
            scroll_left,
            anchor,
        })
    }

    pub fn scroll_screen_with_cursor_margin(
        &mut self,
        amount: &ScrollAmount,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.scroll_screen(amount, window, cx);

        let Some(visible_line_count) = self.visible_line_count() else {
            return;
        };
        let display_snapshot = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let top = self.scroll_manager.scroll_top_row(&display_snapshot, cx);
        let vertical_scroll_margin =
            (self.vertical_scroll_margin() as u32).min(visible_line_count as u32 / 2);

        let max_point = display_snapshot.max_point();
        let min_row = if top.0 == 0 {
            DisplayRow(0)
        } else {
            DisplayRow(top.0 + vertical_scroll_margin)
        };
        let max_row = if top.0 + visible_line_count as u32 >= max_point.row().0 {
            max_point.row()
        } else {
            DisplayRow(
                (top.0 + visible_line_count as u32).saturating_sub(1 + vertical_scroll_margin),
            )
        };

        self.change_selections(
            SelectionEffects::no_scroll().nav_history(false),
            window,
            cx,
            |s| {
                s.move_with(&mut |map, selection| {
                    let head = selection.head();
                    let new_row = if head.row() < min_row {
                        min_row
                    } else if head.row() > max_row {
                        max_row
                    } else {
                        head.row()
                    };
                    if new_row != head.row() {
                        let new_head =
                            map.clip_point(DisplayPoint::new(new_row, head.column()), Bias::Left);
                        selection.collapse_to(new_head, selection.goal);
                    }
                })
            },
        );
    }

    /// Returns an ordering. The newest selection is:
    ///     Ordering::Equal => on screen
    ///     Ordering::Less => above or to the left of the screen
    ///     Ordering::Greater => below or to the right of the screen
    pub fn newest_selection_on_screen(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Ordering, PendingRowGeometry> {
        self.newest_selection_on_screen_with_cursor_shape(self.cursor_shape, window, cx)
    }

    pub fn newest_selection_on_screen_with_cursor_shape(
        &self,
        cursor_shape: CursorShape,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Ordering, PendingRowGeometry> {
        let snapshot = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let newest_head = SelectionGeometry::new(
            *self.selections.newest_anchor(),
            self.selections.line_mode(),
            self.cursor_offset_on_selection,
            &snapshot,
        )
        .head;
        let screen_top = self.scroll_manager.scroll_top_row(&snapshot, cx);
        if newest_head.row() < screen_top {
            return Ok(Ordering::Less);
        }
        let visible_lines = self.visible_line_count();
        if visible_lines
            .is_some_and(|lines| newest_head.row() > DisplayRow(screen_top.0 + lines as u32))
        {
            return Ok(Ordering::Greater);
        }
        let text_layout_details = self.text_layout_details(window, cx);
        let column = newest_head.column();
        let horizontal_position = |visible_columns: RangeInclusive<u32>| {
            if column < *visible_columns.start() {
                Ordering::Less
            } else if column > *visible_columns.end() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        };
        let horizontal = if let Some(viewport) = self.painted_viewport(&snapshot, window, cx) {
            viewport.with_cursor_shape(cursor_shape).point_position(
                &snapshot,
                newest_head,
                &text_layout_details,
                window,
                cx,
            )?
        } else {
            let left_column = self.scroll_manager.scroll_position(&snapshot, cx).x as u32;
            match self.visible_column_count() {
                Some(visible_column_count) => {
                    horizontal_position(left_column..=left_column + visible_column_count as u32)
                }
                None if column < left_column => Ordering::Less,
                None => Ordering::Greater,
            }
        };

        Ok(match visible_lines {
            Some(_) => horizontal,
            None if horizontal.is_lt() => Ordering::Less,
            None => Ordering::Greater,
        })
    }

    pub fn painted_viewport(
        &self,
        snapshot: &DisplaySnapshot,
        window: &Window,
        cx: &mut App,
    ) -> Option<PaintedViewport<'_>> {
        self.painted_viewport_at(
            self.scroll_manager.scroll_position(snapshot, cx).x,
            window,
            cx,
        )
    }

    pub fn painted_viewport_at(
        &self,
        scroll_x: ScrollOffset,
        window: &Window,
        cx: &mut App,
    ) -> Option<PaintedViewport<'_>> {
        let position_map = self.last_position_map.as_ref()?;
        Some(PaintedViewport {
            editor: self,
            bounds: position_map.text_hitbox.bounds,
            clip_bounds: position_map
                .text_hitbox
                .bounds
                .intersect(&position_map.text_hitbox.content_mask.bounds),
            content_origin: position_map.content_origin.x,
            scroll_left: window
                .pixel_snap_f64(scroll_x * ScrollPixelOffset::from(position_map.em_layout_width)),
            em_advance: position_map.em_advance,
            cursor_shape: self.cursor_shape,
            placeholder: self
                .placeholder_display_map
                .as_ref()
                .map(|map| map.update(cx, |map, cx| map.snapshot(cx))),
        })
    }

    pub(crate) fn visible_x_range(
        &self,
        snapshot: &DisplaySnapshot,
        cx: &App,
    ) -> Option<RangeInclusive<ScrollPixelOffset>> {
        let visible_columns = self.visible_column_count()?;
        let (em_layout_width, em_advance) = self.em_widths(cx)?;
        let left_x = self.scroll_manager.scroll_position(snapshot, cx).x * em_layout_width;
        Some(left_x..=left_x + em_advance * visible_columns)
    }

    pub(crate) fn horizontal_scroll_pixels(
        &self,
        scroll_x: ScrollOffset,
        window: &Window,
        cx: &App,
    ) -> ScrollPixelOffset {
        self.em_widths(cx).map_or(0., |(em_layout_width, _)| {
            window.pixel_snap_f64(scroll_x * em_layout_width)
        })
    }

    pub fn read_scroll_position_from_db(
        &mut self,
        item_id: u64,
        workspace_id: WorkspaceId,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let scroll_position = EditorDb::global(cx).get_scroll_position(item_id, workspace_id);
        if let Ok(Some((top_row, x, y, anchor_row, anchor_column, anchor_offset))) = scroll_position
        {
            let horizontal = match (anchor_row, anchor_column, anchor_offset) {
                (Some(row), Some(column), Some(offset)) => Some((Point::new(row, column), offset)),
                _ => None,
            };
            self.restore_scroll_position(
                SerializedScrollPosition {
                    top_row,
                    offset: gpui::Point::new(x, y),
                    horizontal,
                },
                window,
                cx,
            );
        }
    }

    fn em_widths(&self, cx: &App) -> Option<(ScrollPixelOffset, ScrollPixelOffset)> {
        let (font, font_size) = self.display_map.read(cx).font_with_size(cx);
        let text_system = cx.text_system();
        let font_id = text_system.resolve_font(font);
        let em_advance = text_system.em_advance(font_id, *font_size).log_err()?;
        Some((
            ScrollPixelOffset::from(text_system.layout_width(font_id, *font_size, 'm')),
            ScrollPixelOffset::from(em_advance),
        ))
    }
}

pub(crate) fn companion_display_point(
    snapshot: &DisplaySnapshot,
    companion: &DisplaySnapshot,
    companion_point: Point,
    bias: Bias,
) -> (DisplayPoint, bool) {
    let companion_display_point = companion.point_to_display_point(companion_point, bias);
    let row = companion_display_point.row();
    if !snapshot.is_long_row(row) || !companion.is_long_row(row) {
        return (snapshot.clip_point(companion_display_point, bias), false);
    }
    let buffer = snapshot.buffer_snapshot();
    let buffer_row = snapshot
        .display_point_to_point(DisplayPoint::new(row, 0), Bias::Left)
        .row;
    let line_len = buffer.line_len(MultiBufferRow(buffer_row));
    let point = buffer.clip_point(
        Point::new(buffer_row, companion_point.column.min(line_len)),
        bias,
    );
    (
        snapshot.point_to_display_point(point, bias),
        companion_point.column > line_len,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MultiBuffer,
        editor_tests::init_test,
        test::{ShapingTextSystem, editor_test_context::EditorTestContext},
    };
    use gpui::{TestAppContext, VisualTestContext, px, size};
    use std::sync::Arc;

    #[gpui::test]
    async fn test_painted_viewport_uses_snapped_scroll_and_content_origin(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        cx.set_state(&format!("ˇ{}", "x".repeat(2_000)));
        cx.update_editor(|editor, _, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_cursor_shape(CursorShape::Block, cx);
        });

        for scale in [1., 2.] {
            cx.simulate_scale_factor_change(scale);
            cx.run_until_parked();
            wait_for_row_geometry(&cx.editor.clone(), DisplayRow(0), &mut cx).await;
            cx.update_editor(|editor, window, cx| {
                let position_map = editor.last_position_map.as_ref().expect("painted editor");
                let em_width = f64::from(position_map.em_layout_width);
                let bounds = position_map.text_hitbox.bounds;
                let left = f64::from(bounds.left() - position_map.content_origin.x);
                let content_origin = position_map.content_origin.x;
                let glyph_width = position_map
                    .line_layouts
                    .first()
                    .expect("visible row")
                    .x_for_index(1);
                let scale = f64::from(scale);
                assert!(left < -1. / scale);

                for (scroll_pixels, snapped_pixels) in [
                    (0., 0.),
                    (10.24 / scale, 10. / scale),
                    (10.76 / scale, 11. / scale),
                    (4_294_967_296.25 / scale, 4_294_967_296. / scale),
                ] {
                    editor.set_scroll_position(point(scroll_pixels / em_width, 0.), window, cx);
                    let snapshot = editor.display_snapshot(cx);
                    let viewport = editor
                        .painted_viewport(&snapshot, window, cx)
                        .expect("painted editor");
                    assert_eq!(viewport.scroll_left, snapped_pixels);
                    assert_eq!(viewport.content_origin, content_origin);
                    assert_eq!(viewport.bounds, bounds);
                }

                editor.set_scroll_position(
                    point((glyph_width - left - 0.75 / scale) / em_width, 0.),
                    window,
                    cx,
                );
                assert_eq!(
                    editor
                        .newest_selection_on_screen(window, cx)
                        .expect("ready row geometry"),
                    Ordering::Equal
                );
                editor.set_scroll_position(
                    point((glyph_width - left + 1. / scale) / em_width, 0.),
                    window,
                    cx,
                );
                assert_eq!(
                    editor
                        .newest_selection_on_screen(window, cx)
                        .expect("ready row geometry"),
                    Ordering::Less
                );
                editor.set_scroll_position(point(0., 0.), window, cx);
            });
        }
    }

    #[gpui::test]
    async fn test_painted_cell_bounds_match_cursor_snapping(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        cx.update_editor(|editor, _, cx| editor.set_cursor_shape(CursorShape::Block, cx));
        for scale in [1., 1.5, 2.] {
            cx.simulate_scale_factor_change(scale);
            cx.run_until_parked();
            cx.update_editor(|editor, window, cx| {
                let snapshot = editor.display_snapshot(cx);
                let mut viewport = editor
                    .painted_viewport(&snapshot, window, cx)
                    .expect("painted editor");
                for (origin, scroll, x, width, expected_start, expected_end) in [
                    (151.73201, 193., 184.79999, 16.79999, 144., 161.),
                    (151.75, 193., 184.75, 16.75, 143., 160.),
                    (-148.26799, 193., 184.79999, 16.79999, -156., -139.),
                    (0.5, 0., 0., 1.5, 0., 1.),
                    (-0.5, 0., 0., 1.5, 0., 1.),
                    (-1.5, 0., 0., 0.5, -1., -1.),
                    (0.6, 0., 0., 0.6, 1., 2.),
                    (1.2, 0., 0., 0.1, 1., 1.),
                ] {
                    for offset in [0., 4_294_967_296.] {
                        viewport.content_origin = px(origin / scale);
                        viewport.scroll_left = (scroll + offset) / f64::from(scale);
                        assert_eq!(
                            viewport.cell_x_range(
                                (x + offset) / f64::from(scale),
                                px(width / scale),
                                window,
                            ),
                            px(expected_start / scale)..=px(expected_end / scale),
                        );
                    }
                }
            });
        }
    }

    #[gpui::test]
    async fn test_painted_viewport_clips_fully_visible_cells(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        cx.update_editor(|editor, _, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_cursor_shape(CursorShape::Block, cx);
        });
        for scale in [1., 1.5, 2.] {
            cx.simulate_scale_factor_change(scale);
            for (character, length) in [("x", 100), ("x", 2_000), ("é", 100), ("é", 2_000)] {
                cx.set_state(&format!("ˇ{}", character.repeat(length)));
                cx.run_until_parked();
                wait_for_row_geometry(&cx.editor.clone(), DisplayRow(0), &mut cx).await;
                cx.update_editor(|editor, window, cx| {
                    let snapshot = editor.display_snapshot(cx);
                    let details = editor.text_layout_details(window, cx);
                    let row = DisplayRow(0);
                    let stride = character.len() as u32;
                    let width = snapshot.x_for_display_point(DisplayPoint::new(row, stride), &details).expect("ready row geometry");
                    let line_len = snapshot.line_len(row);
                    let line_width = snapshot.x_for_display_point(DisplayPoint::new(row, line_len), &details).expect("ready row geometry");
                    let mut viewport = editor.painted_viewport(&snapshot, window, cx).expect("painted editor");
                    viewport.scroll_left = 0.;
                    for origin in [0.25, -100.25] {
                        viewport.content_origin = px(origin / scale);
                        for (left, right, expected) in [
                            (5.75 * width, 10.25 * width, Some((6 * stride, 9 * stride))),
                            (6. * width, 10. * width, Some((6 * stride, 9 * stride))),
                            (0.6 * width, 0.9 * width, None),
                            (-0.5 * width, width, Some((0, 0))),
                            (line_width, line_width + 1.25 * f64::from(viewport.em_advance), Some((line_len, line_len))),
                            (line_width, line_width + 0.1 * f64::from(viewport.em_advance), None),
                        ] {
                            viewport.clip_bounds = Bounds::from_corners(
                                point(viewport.content_origin + Pixels::from(left), Pixels::ZERO),
                                point(viewport.content_origin + Pixels::from(right), px(10.)),
                            );
                            for column in [0, 6 * stride, line_len] {
                                assert_eq!(
                                    viewport.clip_point(&snapshot, DisplayPoint::new(row, column), &details, window, cx).expect("ready row geometry"),
                                    expected.map(|(start, end)| DisplayPoint::new(row, column.clamp(start, end))),
                                    "{character:?}, {length}, {scale}, {origin}, {left}..{right}, {column}",
                                );
                            }
                        }

                        viewport.scroll_left = 96.5 / f64::from(scale);
                        let column = 11 * stride;
                        let x = snapshot.x_for_display_point(DisplayPoint::new(row, column), &details).expect("ready row geometry");
                        viewport.content_origin = px((origin + 71.75) / scale) - Pixels::from(x - viewport.scroll_left);
                        viewport.clip_bounds = Bounds::from_corners(
                            point(px((origin + 72.01601) / scale), Pixels::ZERO),
                            point(px((origin + 172.) / scale), px(10.)),
                        );
                        assert_eq!(
                            viewport.clip_point(&snapshot, DisplayPoint::new(row, 0), &details, window, cx).expect("ready row geometry"),
                            Some(DisplayPoint::new(row, column)),
                        );
                        viewport.scroll_left = 0.;
                    }
                });
            }
        }
    }

    #[gpui::test]
    async fn test_painted_viewport_uses_current_placeholder(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        cx.update_editor(|editor, window, cx| {
            editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
            editor.set_cursor_shape(CursorShape::Block, cx);
        });
        cx.run_until_parked();
        let pending = cx.update_editor(|editor, window, cx| {
            editor.set_placeholder_text("x", window, cx);
            assert_eq!(
                editor
                    .newest_selection_on_screen(window, cx)
                    .expect("ready placeholder"),
                Ordering::Equal,
            );
            editor.set_placeholder_text(&"y".repeat(2_000), window, cx);
            let style = editor.style(cx).clone();
            editor.set_style(style, window, cx);
            let pending = editor
                .newest_selection_on_screen(window, cx)
                .expect_err("new placeholder geometry is pending");
            assert_eq!(
                pending.display_map_id(),
                editor
                    .placeholder_display_map
                    .as_ref()
                    .expect("placeholder")
                    .entity_id(),
            );
            assert!(editor.request_row_geometry(pending.clone(), cx));
            pending
        });
        pending.wait().await;
        cx.update_editor(|editor, window, cx| {
            assert_eq!(
                editor
                    .newest_selection_on_screen(window, cx)
                    .expect("ready placeholder"),
                Ordering::Equal,
            );
        });
    }

    #[gpui::test]
    fn test_painted_cursor_footprint() {
        let mut cx = TestAppContext::build_with_text_system(
            gpui::TestDispatcher::new(0),
            None,
            Arc::new(ShapingTextSystem(gpui::NoopTextSystem)),
        );
        init_test(&mut cx, |_| {});
        for length in [12, 2_000] {
            let window = cx.add_window(|window, cx| {
                let buffer = MultiBuffer::build_simple(
                    &format!("xxxxxxxxxAV{}\nx", "x".repeat(length - 11)),
                    cx,
                );
                let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                editor.set_cursor_shape(CursorShape::Block, cx);
                editor.set_text_style_refinement(gpui::TextStyleRefinement {
                    font_size: Some(px(16.).into()),
                    ..gpui::TextStyleRefinement::default()
                });
                editor
            });
            let cx = &mut VisualTestContext::from_window(*window, &mut cx);
            let editor = window.root(cx).expect("editor");
            cx.simulate_scale_factor_change(1.);
            cx.run_until_parked();
            cx.foreground_executor
                .clone()
                .block_test(wait_for_row_geometry(&editor, DisplayRow(0), cx));
            editor.update_in(cx, |editor, window, cx| {
                editor.cursor_offset_on_selection = true;
                let mut details = editor.text_layout_details(window, cx);
                let snapshot = editor.display_snapshot(cx);
                let mut viewport = editor
                    .painted_viewport(&snapshot, window, cx)
                    .expect("painted editor");
                viewport.content_origin = Pixels::ZERO;
                viewport.scroll_left = 0.;
                viewport.em_advance = px(8.);
                viewport.clip_bounds =
                    Bounds::new(point(Pixels::ZERO, Pixels::ZERO), size(px(78.), px(20.)));
                let row = DisplayRow(0);
                let cursor = DisplayPoint::new(row, 9);
                let layout = snapshot
                    .layout_row(row, &details)
                    .expect("ready row geometry");
                assert_eq!(layout.x_for_index(9)..=layout.x_for_index(10), 72.0..=78.0);
                assert_eq!(
                    viewport
                        .cursor_x_range(&snapshot, &layout, cursor, &details, window, cx)
                        .expect("ready row geometry"),
                    px(72.)..=px(80.)
                );
                assert_eq!(
                    viewport
                        .clip_point(&snapshot, cursor, &details, window, cx)
                        .expect("ready row geometry"),
                    Some(DisplayPoint::new(row, 8))
                );

                if length == 12 {
                    for (start, end, reversed, left, right, expected) in [
                        ((0, 0), (0, 8), false, 0., 64., (0, 8)),
                        ((0, 0), (0, 10), false, 0., 64., (0, 8)),
                        ((0, 0), (0, 8), true, 16., 64., (0, 2)),
                        ((0, 9), (0, 11), false, 0., 64., (0, 7)),
                        ((0, 0), (1, 0), false, 94., 102., (1, 0)),
                        ((0, 0), (1, 0), false, 78., 94., (0, 12)),
                    ] {
                        let selection = Selection {
                            id: 0,
                            start: DisplayPoint::new(DisplayRow(start.0), start.1),
                            end: DisplayPoint::new(DisplayRow(end.0), end.1),
                            reversed,
                            goal: text::SelectionGoal::None,
                        };
                        viewport.clip_bounds = Bounds::from_corners(
                            point(px(left), Pixels::ZERO),
                            point(px(right), px(20.)),
                        );
                        assert_eq!(
                            viewport
                                .clip_selection(
                                    &snapshot,
                                    &selection,
                                    selection.head(),
                                    &details,
                                    window,
                                    cx
                                )
                                .expect("ready row geometry"),
                            Some(DisplayPoint::new(DisplayRow(expected.0), expected.1)),
                            "{selection:?}, {left}..{right}",
                        );
                    }

                    details.editor_style.text.font_size = px(19.5).into();
                    viewport.content_origin = px(15.625);
                    viewport.scroll_left = 15.;
                    viewport.clip_bounds =
                        Bounds::new(point(px(10.75), Pixels::ZERO), size(px(80.), px(20.)));
                    let cursor = DisplayPoint::new(row, 0);
                    let layout = snapshot
                        .layout_row(row, &details)
                        .expect("ready row geometry");
                    assert_eq!(layout.x_for_index(1), 9.75);
                    assert_eq!(
                        viewport
                            .cursor_x_range(&snapshot, &layout, cursor, &details, window, cx)
                            .expect("ready row geometry"),
                        px(1.)..=px(11.)
                    );
                    assert_eq!(
                        viewport
                            .point_position(&snapshot, cursor, &details, window, cx)
                            .expect("ready row geometry"),
                        Ordering::Equal
                    );
                    assert_eq!(
                        viewport
                            .clip_point(&snapshot, cursor, &details, window, cx)
                            .expect("ready row geometry"),
                        Some(DisplayPoint::new(row, 1))
                    );
                    viewport.clip_bounds.origin.x = px(11.);
                    assert_eq!(
                        viewport
                            .point_position(&snapshot, cursor, &details, window, cx)
                            .expect("ready row geometry"),
                        Ordering::Less
                    );
                }
            });
        }
    }

    #[gpui::test]
    fn test_horizontal_page_scroll_uses_layout_units() {
        let mut cx = TestAppContext::build_with_text_system(
            gpui::TestDispatcher::new(0),
            None,
            Arc::new(ShapingTextSystem(gpui::NoopTextSystem)),
        );
        init_test(&mut cx, |_| {});
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&"x".repeat(4_000), cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, &mut cx);
        let editor = window.root(cx).expect("editor");
        editor.update_in(cx, |editor, window, cx| {
            editor.scroll_manager.visible_line_count = Some(10.);
            editor.scroll_manager.visible_column_count = Some(100.5);
            for start in [10., 4_294_967_296.25] {
                editor.set_scroll_position(point(start, 0.), window, cx);
                editor.scroll_screen(&ScrollAmount::PageWidth(0.5), window, cx);
                assert_eq!(editor.scroll_position(cx), point(start + 60., 0.));
            }
        });
    }

    async fn wait_for_row_geometry(
        editor: &Entity<Editor>,
        row: DisplayRow,
        cx: &mut VisualTestContext,
    ) {
        loop {
            let pending = editor.update(cx, |editor, cx| {
                let snapshot = editor.display_snapshot(cx);
                match snapshot
                    .long_row(row)
                    .and_then(|_| snapshot.prepared_row_text(row))
                {
                    Ok(_) => None,
                    Err(pending) => {
                        editor.display_map.update(cx, |map, cx| {
                            map.request_row_geometry(pending.clone(), cx);
                        });
                        Some(pending)
                    }
                }
            });
            let Some(pending) = pending else {
                return;
            };
            pending.wait().await;
        }
    }
}

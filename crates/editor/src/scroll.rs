mod actions;
pub(crate) mod autoscroll;
pub(crate) mod scroll_amount;

use crate::editor_settings::ScrollBeyondLastLine;
use crate::{
    Anchor, DisplayPoint, DisplayRow, Editor, EditorEvent, EditorMode, EditorSettings,
    InlayHintRefreshReason, MultiBufferSnapshot, RowExt, ToPoint,
    display_map::{DisplaySnapshot, ToDisplayPoint},
    hover_popover::hide_hover,
    persistence::DB,
};
pub use autoscroll::{Autoscroll, AutoscrollStrategy};
use core::fmt::Debug;
use gpui::{Along, App, Axis, Context, Pixels, Task, Window, point, px};
use language::language_settings::{AllLanguageSettings, SoftWrap};
use language::{Bias, Point};
pub use scroll_amount::ScrollAmount;
use settings::Settings;
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};
use ui::scrollbars::ScrollbarAutoHide;
use util::ResultExt;
use workspace::{ItemId, WorkspaceId};

pub const SCROLL_EVENT_SEPARATION: Duration = Duration::from_millis(28);
const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);

pub struct WasScrolled(pub(crate) bool);

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
            anchor: Anchor::min(),
        }
    }

    pub fn scroll_position(&self, snapshot: &DisplaySnapshot) -> gpui::Point<ScrollOffset> {
        self.offset.apply_along(Axis::Vertical, |offset| {
            if self.anchor == Anchor::min() {
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

#[derive(Clone, Copy, Debug)]
pub struct OngoingScroll {
    last_event: Instant,
    axis: Option<Axis>,
}

impl OngoingScroll {
    fn new() -> Self {
        Self {
            last_event: Instant::now() - SCROLL_EVENT_SEPARATION,
            axis: None,
        }
    }

    pub fn filter(&self, delta: &mut gpui::Point<Pixels>) -> Option<Axis> {
        const UNLOCK_PERCENT: f32 = 1.9;
        const UNLOCK_LOWER_BOUND: Pixels = px(6.);
        let mut axis = self.axis;

        let x = delta.x.abs();
        let y = delta.y.abs();
        let duration = Instant::now().duration_since(self.last_event);
        if duration > SCROLL_EVENT_SEPARATION {
            //New ongoing scroll will start, determine axis
            axis = if x <= y {
                Some(Axis::Vertical)
            } else {
                Some(Axis::Horizontal)
            };
        } else if x.max(y) >= UNLOCK_LOWER_BOUND {
            //Check if the current ongoing will need to unlock
            match axis {
                Some(Axis::Vertical) => {
                    if x > y && x >= y * UNLOCK_PERCENT {
                        axis = None;
                    }
                }

                Some(Axis::Horizontal) => {
                    if y > x && y >= x * UNLOCK_PERCENT {
                        axis = None;
                    }
                }

                None => {}
            }
        }

        match axis {
            Some(Axis::Vertical) => {
                *delta = point(px(0.), delta.y);
            }
            Some(Axis::Horizontal) => {
                *delta = point(delta.x, px(0.));
            }
            None => {}
        }

        axis
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

#[derive(Clone, Copy, Debug)]
pub enum ScrollAnimationPhase {
    Intermediate,
    Final,
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollAnimationUpdate {
    pub position: gpui::Point<ScrollOffset>,
    pub phase: ScrollAnimationPhase,
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollAnimation {
    pub duration: Duration,
    pub start_position: gpui::Point<ScrollOffset>,
    pub target_position: gpui::Point<ScrollOffset>,
    pub start_time: Instant,
}

impl ScrollAnimation {
    pub fn progress(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let duration = self.duration.as_secs_f32();

        (elapsed / duration).min(1.0)
    }

    pub fn advance(&self) -> gpui::Point<ScrollOffset> {
        let progress = self.progress();
        let easing_fn = Self::easing_fn();
        let eased_progress = easing_fn(progress);

        let start = self.start_position;
        let target = self.target_position;

        let current_x = start.x + (target.x - start.x) * eased_progress as f64;
        let current_y = start.y + (target.y - start.y) * eased_progress as f64;

        point(current_x, current_y)
    }

    pub fn easing_fn() -> impl Fn(f32) -> f32 {
        gpui::ease_out_cubic()
    }
}

pub struct ScrollManager {
    pub(crate) vertical_scroll_margin: ScrollOffset,
    anchor: ScrollAnchor,
    ongoing: OngoingScroll,
    /// The second element indicates whether the autoscroll request is local
    /// (true) or remote (false). Local requests are initiated by user actions,
    /// while remote requests come from external sources.
    autoscroll_request: Option<(Autoscroll, bool)>,
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
    scroll_animation: Option<ScrollAnimation>,
    scroll_animation_duration: Duration,
}

impl ScrollManager {
    pub fn new(cx: &mut App) -> Self {
        let editor_settings = EditorSettings::get_global(cx);

        ScrollManager {
            vertical_scroll_margin: editor_settings.vertical_scroll_margin,
            anchor: ScrollAnchor::new(),
            ongoing: OngoingScroll::new(),
            autoscroll_request: None,
            show_scrollbars: true,
            hide_scrollbar_task: None,
            active_scrollbar: None,
            last_autoscroll: None,
            visible_line_count: None,
            visible_column_count: None,
            forbid_vertical_scroll: false,
            minimap_thumb_state: None,
            scroll_animation: None,
            // TODO: We are not listening for settings change
            scroll_animation_duration: Duration::from_secs_f32(
                editor_settings.smooth_scroll_duration,
            ),
        }
    }

    pub fn clone_state(&mut self, other: &Self) {
        self.anchor = other.anchor;
        self.ongoing = other.ongoing;
    }

    pub fn anchor(&self) -> ScrollAnchor {
        self.anchor
    }

    pub fn ongoing_scroll(&self) -> OngoingScroll {
        self.ongoing
    }

    pub fn update_ongoing_scroll(&mut self, axis: Option<Axis>) {
        self.ongoing.last_event = Instant::now();
        self.ongoing.axis = axis;
    }

    pub fn scroll_position(&self, snapshot: &DisplaySnapshot) -> gpui::Point<ScrollOffset> {
        self.anchor.scroll_position(snapshot)
    }

    pub fn set_scroll_position_visual(
        &mut self,
        scroll_position: gpui::Point<ScrollOffset>,
        map: &DisplaySnapshot,
        cx: &mut Context<Editor>,
    ) {
        let (anchor, _) = self.calculate_scroll_anchor(scroll_position, map, cx);
        self.anchor = anchor;
        cx.notify();
    }

    fn set_scroll_position(
        &mut self,
        scroll_position: gpui::Point<ScrollOffset>,
        map: &DisplaySnapshot,
        local: bool,
        autoscroll: bool,
        workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> WasScrolled {
        let (anchor, top_row) = self.calculate_scroll_anchor(scroll_position, map, cx);
        self.set_anchor(anchor, top_row, local, autoscroll, workspace_id, window, cx)
    }

    fn set_anchor(
        &mut self,
        anchor: ScrollAnchor,
        top_row: u32,
        local: bool,
        autoscroll: bool,
        workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> WasScrolled {
        let adjusted_anchor = if self.forbid_vertical_scroll {
            ScrollAnchor {
                offset: gpui::Point::new(anchor.offset.x, self.anchor.offset.y),
                anchor: self.anchor.anchor,
            }
        } else {
            anchor
        };

        self.autoscroll_request.take();
        if self.anchor == adjusted_anchor {
            return WasScrolled(false);
        }

        self.anchor = adjusted_anchor;
        cx.emit(EditorEvent::ScrollPositionChanged { local, autoscroll });
        self.show_scrollbars(window, cx);
        if let Some(workspace_id) = workspace_id {
            let item_id = cx.entity().entity_id().as_u64() as ItemId;

            cx.foreground_executor()
                .spawn(async move {
                    log::debug!(
                        "Saving scroll position for item {item_id:?} in workspace {workspace_id:?}"
                    );
                    DB.save_scroll_position(
                        item_id,
                        workspace_id,
                        top_row,
                        anchor.offset.x,
                        anchor.offset.y,
                    )
                    .await
                    .log_err()
                })
                .detach()
        }
        cx.notify();

        WasScrolled(true)
    }

    fn calculate_scroll_anchor(
        &self,
        scroll_position: gpui::Point<ScrollOffset>,
        map: &DisplaySnapshot,
        cx: &App,
    ) -> (ScrollAnchor, u32) {
        let scroll_top = scroll_position.y.max(0.);
        let scroll_top = match EditorSettings::get_global(cx).scroll_beyond_last_line {
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

        // Anchor the scroll position to the *left* of the first visible buffer point.
        //
        // This prevents the viewport from shifting down when blocks (e.g. expanded diff hunk
        // deletions) are inserted *above* the first buffer character in the file.
        let top_anchor = map.buffer_snapshot().anchor_before(scroll_top_buffer_point);

        let anchor = ScrollAnchor {
            anchor: top_anchor,
            offset: point(
                scroll_position.x.max(0.),
                scroll_top - top_anchor.to_display_point(map).row().as_f64(),
            ),
        };

        (anchor, scroll_top_buffer_point.row)
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

    pub fn take_autoscroll_request(&mut self) -> Option<(Autoscroll, bool)> {
        self.autoscroll_request.take()
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

    pub fn clamp_scroll_left(&mut self, max: f64) -> bool {
        if max < self.anchor.offset.x {
            self.anchor.offset.x = max;
            true
        } else {
            false
        }
    }

    pub fn set_forbid_vertical_scroll(&mut self, forbid: bool) {
        self.forbid_vertical_scroll = forbid;
    }

    pub fn forbid_vertical_scroll(&self) -> bool {
        self.forbid_vertical_scroll
    }

    pub fn scroll_animation(&self) -> Option<&ScrollAnimation> {
        self.scroll_animation.as_ref()
    }

    pub fn start_animation(
        &mut self,
        current_position: gpui::Point<ScrollOffset>,
        target_position: gpui::Point<ScrollOffset>,
    ) {
        // We advance the current animation and restart it with the last position
        let start_position = if let Some(animation) = &self.scroll_animation {
            animation.advance()
        } else {
            current_position
        };

        self.scroll_animation = Some(ScrollAnimation {
            start_position,
            target_position,
            start_time: Instant::now(),
            duration: self.scroll_animation_duration,
        });
    }

    pub fn cancel_animation(&mut self) {
        self.scroll_animation = None;
    }

    pub fn update_animation(&mut self) -> Option<ScrollAnimationUpdate> {
        let animation = self.scroll_animation?;

        if animation.progress() >= 1.0 {
            self.cancel_animation();

            Some(ScrollAnimationUpdate {
                position: animation.target_position,
                phase: ScrollAnimationPhase::Final,
            })
        } else {
            Some(ScrollAnimationUpdate {
                position: animation.advance(),
                phase: ScrollAnimationPhase::Intermediate,
            })
        }
    }

    pub fn animation_progress(&self) -> Option<f32> {
        self.scroll_animation
            .as_ref()
            .map(|animation| animation.progress())
    }
}

impl Editor {
    pub fn vertical_scroll_margin(&self) -> usize {
        self.scroll_manager.vertical_scroll_margin as usize
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
        let opened_first_time = self.scroll_manager.visible_line_count.is_none();
        self.scroll_manager.visible_line_count = Some(lines);
        if opened_first_time {
            self.post_scroll_update = cx.spawn_in(window, async move |editor, cx| {
                editor
                    .update_in(cx, |editor, window, cx| {
                        editor.register_visible_buffers(cx);
                        editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
                        editor.update_lsp_data(None, window, cx);
                        editor.colorize_brackets(false, cx);
                    })
                    .ok();
            });
        }
    }

    pub(crate) fn set_visible_column_count(&mut self, columns: f64) {
        self.scroll_manager.visible_column_count = Some(columns);
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
        let position =
            self.scroll_manager.anchor.scroll_position(&display_map) + delta.map(f64::from);
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
        self.set_scroll_position_taking_display_map(
            scroll_position,
            local,
            autoscroll,
            map,
            window,
            cx,
        )
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
        let workspace_id = self.workspace.as_ref().and_then(|workspace| workspace.1);

        self.edit_prediction_preview
            .set_previous_scroll_position(None);

        let adjusted_position = if self.scroll_manager.forbid_vertical_scroll {
            let current_position = self.scroll_manager.anchor.scroll_position(&display_map);
            gpui::Point::new(scroll_position.x, current_position.y)
        } else {
            scroll_position
        };

        self.scroll_manager.set_scroll_position(
            adjusted_position,
            &display_map,
            local,
            autoscroll,
            workspace_id,
            window,
            cx,
        )
    }

    pub fn scroll_position(&self, cx: &mut Context<Self>) -> gpui::Point<ScrollOffset> {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.scroll_manager.anchor.scroll_position(&display_map)
    }

    pub fn set_scroll_anchor(
        &mut self,
        scroll_anchor: ScrollAnchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        hide_hover(self, cx);
        let workspace_id = self.workspace.as_ref().and_then(|workspace| workspace.1);
        let top_row = scroll_anchor
            .anchor
            .to_point(&self.buffer().read(cx).snapshot(cx))
            .row;
        self.scroll_manager.set_anchor(
            scroll_anchor,
            top_row,
            true,
            false,
            workspace_id,
            window,
            cx,
        );
    }

    pub(crate) fn set_scroll_anchor_remote(
        &mut self,
        scroll_anchor: ScrollAnchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        hide_hover(self, cx);
        let workspace_id = self.workspace.as_ref().and_then(|workspace| workspace.1);
        let snapshot = &self.buffer().read(cx).snapshot(cx);
        if !scroll_anchor.anchor.is_valid(snapshot) {
            log::warn!("Invalid scroll anchor: {:?}", scroll_anchor);
            return;
        }
        let top_row = scroll_anchor.anchor.to_point(snapshot).row;
        self.scroll_manager.set_anchor(
            scroll_anchor,
            top_row,
            false,
            false,
            workspace_id,
            window,
            cx,
        );
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

        // Use animation target if animating, otherwise current position
        let mut current_position = self
            .scroll_manager
            .scroll_animation()
            .map(|a| a.target_position)
            .unwrap_or_else(|| self.scroll_position(cx));

        let Some(visible_line_count) = self.visible_line_count() else {
            return;
        };
        let Some(mut visible_column_count) = self.visible_column_count() else {
            return;
        };

        // If the user has a preferred line length, and has the editor
        // configured to wrap at the preferred line length, or bounded to it,
        // use that value over the visible column count. This was mostly done so
        // that tests could actually be written for vim's `z l`, `z h`, `z
        // shift-l` and `z shift-h` commands, as there wasn't a good way to
        // configure the editor to only display a certain number of columns. If
        // that ever happens, this could probably be removed.
        let settings = AllLanguageSettings::get_global(cx);
        if matches!(
            settings.defaults.soft_wrap,
            SoftWrap::PreferredLineLength | SoftWrap::Bounded
        ) && (settings.defaults.preferred_line_length as f64) < visible_column_count
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
                f64::from(self.gutter_dimensions.margin / last_position_map.em_advance);
        }
        let new_position = current_position
            + point(
                amount.columns(visible_column_count),
                amount.lines(visible_line_count),
            );

        self.scroll_animated(new_position, None, window, cx);
    }

    /// Returns an ordering. The newest selection is:
    ///     Ordering::Equal => on screen
    ///     Ordering::Less => above or to the left of the screen
    ///     Ordering::Greater => below or to the right of the screen
    pub fn newest_selection_on_screen(&self, cx: &mut App) -> Ordering {
        let snapshot = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let newest_head = self
            .selections
            .newest_anchor()
            .head()
            .to_display_point(&snapshot);
        let screen_top = self
            .scroll_manager
            .anchor
            .anchor
            .to_display_point(&snapshot);

        if screen_top > newest_head {
            return Ordering::Less;
        }

        if let (Some(visible_lines), Some(visible_columns)) =
            (self.visible_line_count(), self.visible_column_count())
            && newest_head.row() <= DisplayRow(screen_top.row().0 + visible_lines as u32)
            && newest_head.column() <= screen_top.column() + visible_columns as u32
        {
            return Ordering::Equal;
        }

        Ordering::Greater
    }

    pub fn read_scroll_position_from_db(
        &mut self,
        item_id: u64,
        workspace_id: WorkspaceId,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let scroll_position = DB.get_scroll_position(item_id, workspace_id);
        if let Ok(Some((top_row, x, y))) = scroll_position {
            let top_anchor = self
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_before(Point::new(top_row, 0));
            let scroll_anchor = ScrollAnchor {
                offset: gpui::Point::new(x, y),
                anchor: top_anchor,
            };
            self.set_scroll_anchor(scroll_anchor, window, cx);
        }
    }
}

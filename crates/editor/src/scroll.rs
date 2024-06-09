mod actions;
pub(crate) mod autoscroll;
pub(crate) mod scroll_amount;

use crate::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    hover_popover::hide_hover,
    persistence::DB,
    Anchor, DisplayPoint, DisplayRow, Editor, EditorEvent, EditorMode, EditorSettings,
    InlayHintRefreshReason, MultiBufferSnapshot, RowExt, ToPoint,
};
pub use autoscroll::{Autoscroll, AutoscrollStrategy};
use gpui::{point, px, AppContext, Entity, Global, Pixels, Task, ViewContext, WindowContext};
use language::{Bias, Point};
pub use scroll_amount::ScrollAmount;
use settings::Settings;
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};
use util::ResultExt;
use workspace::{ItemId, WorkspaceId};

pub const SCROLL_EVENT_SEPARATION: Duration = Duration::from_millis(28);
const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Default)]
pub struct ScrollbarAutoHide(pub bool);

impl Global for ScrollbarAutoHide {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollAnchor {
    pub offset: gpui::Point<f32>,
    pub anchor: Anchor,
}

impl ScrollAnchor {
    fn new() -> Self {
        Self {
            offset: gpui::Point::default(),
            anchor: Anchor::min(),
        }
    }

    pub fn scroll_position(&self, snapshot: &DisplaySnapshot) -> gpui::Point<f32> {
        let mut scroll_position = self.offset;
        if self.anchor == Anchor::min() {
            scroll_position.y = 0.;
        } else {
            let scroll_top = self.anchor.to_display_point(snapshot).row().as_f32();
            scroll_position.y = scroll_top + scroll_position.y;
        }
        scroll_position
    }

    pub fn top_row(&self, buffer: &MultiBufferSnapshot) -> u32 {
        self.anchor.to_point(buffer).row
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Axis {
    Vertical,
    Horizontal,
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

pub struct ScrollManager {
    pub(crate) vertical_scroll_margin: f32,
    anchor: ScrollAnchor,
    ongoing: OngoingScroll,
    autoscroll_request: Option<(Autoscroll, bool)>,
    last_autoscroll: Option<(gpui::Point<f32>, f32, f32, AutoscrollStrategy)>,
    show_scrollbars: bool,
    hide_scrollbar_task: Option<Task<()>>,
    dragging_scrollbar: bool,
    visible_line_count: Option<f32>,
    forbid_vertical_scroll: bool,
}

impl ScrollManager {
    pub fn new(cx: &mut WindowContext) -> Self {
        ScrollManager {
            vertical_scroll_margin: EditorSettings::get_global(cx).vertical_scroll_margin,
            anchor: ScrollAnchor::new(),
            ongoing: OngoingScroll::new(),
            autoscroll_request: None,
            show_scrollbars: true,
            hide_scrollbar_task: None,
            dragging_scrollbar: false,
            last_autoscroll: None,
            visible_line_count: None,
            forbid_vertical_scroll: false,
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

    pub fn scroll_position(&self, snapshot: &DisplaySnapshot) -> gpui::Point<f32> {
        self.anchor.scroll_position(snapshot)
    }

    fn set_scroll_position(
        &mut self,
        scroll_position: gpui::Point<f32>,
        map: &DisplaySnapshot,
        local: bool,
        autoscroll: bool,
        workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Editor>,
    ) {
        if self.forbid_vertical_scroll {
            return;
        }
        let (new_anchor, top_row) = if scroll_position.y <= 0. {
            (
                ScrollAnchor {
                    anchor: Anchor::min(),
                    offset: scroll_position.max(&gpui::Point::default()),
                },
                0,
            )
        } else {
            let scroll_top_buffer_point =
                DisplayPoint::new(DisplayRow(scroll_position.y as u32), 0).to_point(&map);
            let top_anchor = map
                .buffer_snapshot
                .anchor_at(scroll_top_buffer_point, Bias::Right);

            (
                ScrollAnchor {
                    anchor: top_anchor,
                    offset: point(
                        scroll_position.x.max(0.),
                        scroll_position.y - top_anchor.to_display_point(&map).row().as_f32(),
                    ),
                },
                scroll_top_buffer_point.row,
            )
        };

        self.set_anchor(new_anchor, top_row, local, autoscroll, workspace_id, cx);
    }

    fn set_anchor(
        &mut self,
        anchor: ScrollAnchor,
        top_row: u32,
        local: bool,
        autoscroll: bool,
        workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Editor>,
    ) {
        if self.forbid_vertical_scroll {
            return;
        }
        self.anchor = anchor;
        cx.emit(EditorEvent::ScrollPositionChanged { local, autoscroll });
        self.show_scrollbar(cx);
        self.autoscroll_request.take();
        if let Some(workspace_id) = workspace_id {
            let item_id = cx.view().entity_id().as_u64() as ItemId;

            cx.foreground_executor()
                .spawn(async move {
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
    }

    pub fn show_scrollbar(&mut self, cx: &mut ViewContext<Editor>) {
        if !self.show_scrollbars {
            self.show_scrollbars = true;
            cx.notify();
        }

        if cx.default_global::<ScrollbarAutoHide>().0 {
            self.hide_scrollbar_task = Some(cx.spawn(|editor, mut cx| async move {
                cx.background_executor()
                    .timer(SCROLLBAR_SHOW_INTERVAL)
                    .await;
                editor
                    .update(&mut cx, |editor, cx| {
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

    pub fn autoscroll_requested(&self) -> bool {
        self.autoscroll_request.is_some()
    }

    pub fn is_dragging_scrollbar(&self) -> bool {
        self.dragging_scrollbar
    }

    pub fn set_is_dragging_scrollbar(&mut self, dragging: bool, cx: &mut ViewContext<Editor>) {
        if dragging != self.dragging_scrollbar {
            self.dragging_scrollbar = dragging;
            cx.notify();
        }
    }

    pub fn clamp_scroll_left(&mut self, max: f32) -> bool {
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
}

impl Editor {
    pub fn vertical_scroll_margin(&self) -> usize {
        self.scroll_manager.vertical_scroll_margin as usize
    }

    pub fn set_vertical_scroll_margin(&mut self, margin_rows: usize, cx: &mut ViewContext<Self>) {
        self.scroll_manager.vertical_scroll_margin = margin_rows as f32;
        cx.notify();
    }

    pub fn visible_line_count(&self) -> Option<f32> {
        self.scroll_manager.visible_line_count
    }

    pub(crate) fn set_visible_line_count(&mut self, lines: f32, cx: &mut ViewContext<Self>) {
        let opened_first_time = self.scroll_manager.visible_line_count.is_none();
        self.scroll_manager.visible_line_count = Some(lines);
        if opened_first_time {
            cx.spawn(|editor, mut cx| async move {
                editor
                    .update(&mut cx, |editor, cx| {
                        editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx)
                    })
                    .ok()
            })
            .detach()
        }
    }

    pub fn apply_scroll_delta(
        &mut self,
        scroll_delta: gpui::Point<f32>,
        cx: &mut ViewContext<Self>,
    ) {
        if self.scroll_manager.forbid_vertical_scroll {
            return;
        }
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let position = self.scroll_manager.anchor.scroll_position(&display_map) + scroll_delta;
        self.set_scroll_position_taking_display_map(position, true, false, display_map, cx);
    }

    pub fn set_scroll_position(
        &mut self,
        scroll_position: gpui::Point<f32>,
        cx: &mut ViewContext<Self>,
    ) {
        if self.scroll_manager.forbid_vertical_scroll {
            return;
        }
        self.set_scroll_position_internal(scroll_position, true, false, cx);
    }

    pub(crate) fn set_scroll_position_internal(
        &mut self,
        scroll_position: gpui::Point<f32>,
        local: bool,
        autoscroll: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.set_scroll_position_taking_display_map(scroll_position, local, autoscroll, map, cx);
    }

    fn set_scroll_position_taking_display_map(
        &mut self,
        scroll_position: gpui::Point<f32>,
        local: bool,
        autoscroll: bool,
        display_map: DisplaySnapshot,
        cx: &mut ViewContext<Self>,
    ) {
        hide_hover(self, cx);
        let workspace_id = self.workspace.as_ref().and_then(|workspace| workspace.1);

        self.scroll_manager.set_scroll_position(
            scroll_position,
            &display_map,
            local,
            autoscroll,
            workspace_id,
            cx,
        );

        self.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
    }

    pub fn scroll_position(&self, cx: &mut ViewContext<Self>) -> gpui::Point<f32> {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.scroll_manager.anchor.scroll_position(&display_map)
    }

    pub fn set_scroll_anchor(&mut self, scroll_anchor: ScrollAnchor, cx: &mut ViewContext<Self>) {
        hide_hover(self, cx);
        let workspace_id = self.workspace.as_ref().and_then(|workspace| workspace.1);
        let top_row = scroll_anchor
            .anchor
            .to_point(&self.buffer().read(cx).snapshot(cx))
            .row;
        self.scroll_manager
            .set_anchor(scroll_anchor, top_row, true, false, workspace_id, cx);
    }

    pub(crate) fn set_scroll_anchor_remote(
        &mut self,
        scroll_anchor: ScrollAnchor,
        cx: &mut ViewContext<Self>,
    ) {
        hide_hover(self, cx);
        let workspace_id = self.workspace.as_ref().and_then(|workspace| workspace.1);
        let snapshot = &self.buffer().read(cx).snapshot(cx);
        if !scroll_anchor.anchor.is_valid(snapshot) {
            log::warn!("Invalid scroll anchor: {:?}", scroll_anchor);
            return;
        }
        let top_row = scroll_anchor.anchor.to_point(snapshot).row;
        self.scroll_manager
            .set_anchor(scroll_anchor, top_row, false, false, workspace_id, cx);
    }

    pub fn scroll_screen(&mut self, amount: &ScrollAmount, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }

        if self.take_rename(true, cx).is_some() {
            return;
        }

        let cur_position = self.scroll_position(cx);
        let new_pos = cur_position + point(0., amount.lines(self));
        self.set_scroll_position(new_pos, cx);
    }

    /// Returns an ordering. The newest selection is:
    ///     Ordering::Equal => on screen
    ///     Ordering::Less => above the screen
    ///     Ordering::Greater => below the screen
    pub fn newest_selection_on_screen(&self, cx: &mut AppContext) -> Ordering {
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

        if let Some(visible_lines) = self.visible_line_count() {
            if newest_head.row() < DisplayRow(screen_top.row().0 + visible_lines as u32) {
                return Ordering::Equal;
            }
        }

        Ordering::Greater
    }

    pub fn read_scroll_position_from_db(
        &mut self,
        item_id: u64,
        workspace_id: WorkspaceId,
        cx: &mut ViewContext<Editor>,
    ) {
        let scroll_position = DB.get_scroll_position(item_id, workspace_id);
        if let Ok(Some((top_row, x, y))) = scroll_position {
            let top_anchor = self
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_at(Point::new(top_row, 0), Bias::Left);
            let scroll_anchor = ScrollAnchor {
                offset: gpui::Point::new(x, y),
                anchor: top_anchor,
            };
            self.set_scroll_anchor(scroll_anchor, cx);
        }
    }
}

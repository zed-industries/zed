pub mod pdf_engine;
pub mod pdf_item;
pub mod persistence;
pub mod render_pool;
pub mod tile_cache;
#[cfg(test)]
mod tests;

use std::{cell::Cell, path::Path, rc::Rc};

use anyhow::Context as _;
use editor::{EditorSettings, items::entry_git_aware_label_color};
use file_icons::FileIcons;
use gpui::{
    AnyElement, App, Bounds, Context, Entity, EventEmitter, FocusHandle, Focusable, Font,
    InteractiveElement, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ParentElement, PinchEvent, Pixels, Point, Render, ScrollDelta, ScrollWheelEvent, SharedString,
    Styled, Subscription, Task, WeakEntity, Window, actions, canvas, div, img, point, px,
};
use persistence::PdfViewerDb;
pub use pdf_engine::*;
pub use pdf_item::*;
use project::{Project, ProjectPath};
pub use render_pool::{RenderPool, RenderResult};
use settings::Settings;
use theme_settings::ThemeSettings;
pub use tile_cache::{TileCache, TileKey};
use ui::{Button, ButtonCommon, Clickable, FluentBuilder, Icon, IconButton, IconName, Label, Tooltip, prelude::*};
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemLocation, Workspace, WorkspaceId,
    delete_unloaded_items,
    invalid_item_view::InvalidItemView,
    item::{HighlightedText, Item, ItemEvent, ProjectItem, SerializableItem, TabContentParams},
};

actions!(
    pdf_viewer,
    [
        /// Advance to the next PDF page.
        NextPage,
        /// Go back to the previous PDF page.
        PrevPage,
        /// Go to the first page.
        FirstPage,
        /// Go to the last page.
        LastPage,
        /// Zoom in on the current document.
        ZoomIn,
        /// Zoom out on the current document.
        ZoomOut,
        /// Reset zoom level to 100%.
        ResetZoom,
        /// Fit page to available container width.
        FitToWidth,
        /// Fit full page within container viewport.
        FitToPage,
        /// Rotate document 90 degrees clockwise.
        RotateClockwise,
        /// Rotate document 90 degrees counter-clockwise.
        RotateCounterclockwise,
    ]
);

const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 5.0;
const ZOOM_FACTOR: f32 = 1.2;
const PAGE_MARGIN: f32 = 16.0;

pub enum PdfViewEvent {
    PageChanged,
}

pub struct PdfView {
    pdf_item: Entity<PdfItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    total_pages: usize,
    current_page: usize,
    zoom_level: f32,
    rotation: u16,
    scroll_offset: Point<Pixels>,
    viewport_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    tile_cache: TileCache,
    render_pool: RenderPool,
    last_mouse_position: Option<Point<Pixels>>,
    is_panning: bool,
    _subscription: Option<Subscription>,
    _render_drain_task: Option<Task<()>>,
}

impl Focusable for PdfView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PdfViewEvent> for PdfView {}

impl PdfView {
    pub fn new(
        pdf_item: Entity<PdfItem>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let total_pages = pdf_item.read(cx).document().page_count().max(1);

        let subscription = cx.subscribe(&pdf_item, |this, _item, event: &PdfItemEvent, cx| {
            if *event == PdfItemEvent::Reloaded {
                this.tile_cache.clear();
                this.render_pool.increment_generation();
                cx.notify();
            }
        });

        // Background worker pool with 4 threads for tile and overview rasterization
        let render_pool = RenderPool::new(4);
        let tile_cache = TileCache::default();

        // Immediately request overview for initial pages so the document renders without delay
        let doc = pdf_item.read(cx).document().clone();
        for p in 0..total_pages.min(3) {
            render_pool.request_page_overview(p, 0, 0.75, doc.clone());
        }

        let mut view = Self {
            pdf_item,
            project,
            focus_handle,
            total_pages,
            current_page: 0,
            zoom_level: 1.0,
            rotation: 0,
            scroll_offset: point(px(0.0), px(0.0)),
            viewport_bounds: Rc::new(Cell::new(None)),
            tile_cache,
            render_pool,
            last_mouse_position: None,
            is_panning: false,
            _subscription: Some(subscription),
            _render_drain_task: None,
        };

        view.start_render_drain_task(window, cx);
        view
    }

    /// Drains any finished background tiles and overviews from the render worker pool.
    pub fn drain_render_results(&mut self) -> bool {
        let cur_gen = self.render_pool.generation();
        let mut received = false;
        while let Some(result) = self.render_pool.try_recv() {
            match result {
                RenderResult::Tile { key, generation, image } => {
                    if generation == cur_gen {
                        if let Ok(img) = image {
                            self.tile_cache.insert(key, img);
                            received = true;
                        }
                    }
                }
                RenderResult::PageOverview { page_index, rotation, generation, image } => {
                    if generation == cur_gen {
                        if let Ok(img) = image {
                            self.tile_cache.insert_page_overview(page_index, rotation, img);
                            received = true;
                        }
                    }
                }
            }
        }
        received
    }

    /// Spawns a lightweight background task loop to pump completed tiles and overviews into the cache.
    fn start_render_drain_task(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let task = cx.spawn_in(window, async move |this, cx| {
            loop {
                let update_res = this.update(cx, |view, cx| {
                    if view.drain_render_results() {
                        cx.notify();
                    }
                });

                if update_res.is_err() {
                    break;
                }

                // Poll every 16ms (~60fps)
                cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
            }
        });

        self._render_drain_task = Some(task);
    }

    pub fn current_page(&self) -> usize {
        self.current_page
    }

    pub fn total_pages(&self) -> usize {
        self.total_pages
    }

    pub fn zoom_level(&self) -> f32 {
        self.zoom_level
    }

    pub fn set_zoom_level(
        &mut self,
        zoom: f32,
        anchor: Option<Point<Pixels>>,
        cx: &mut Context<Self>,
    ) {
        let old_zoom = self.zoom_level;
        let new_zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        if (new_zoom - old_zoom).abs() < 0.001 {
            return;
        }

        self.zoom_level = new_zoom;
        self.render_pool.increment_generation();

        // If an anchor point in window pixels is given, keep the document position beneath it stationary
        if let Some((anchor_pt, vp)) = anchor.zip(self.viewport_bounds.get()) {
            let rel_x: f32 = (anchor_pt.x - vp.origin.x).into();
            let rel_y: f32 = (anchor_pt.y - vp.origin.y).into();

            let doc_x = rel_x - f32::from(self.scroll_offset.x);
            let doc_y = rel_y - f32::from(self.scroll_offset.y);

            let ratio = new_zoom / old_zoom;
            let new_doc_x = doc_x * ratio;
            let new_doc_y = doc_y * ratio;

            self.scroll_offset = point(
                px(rel_x - new_doc_x),
                px(rel_y - new_doc_y),
            );
        }

        self.clamp_scroll_bounds(cx);
        self.update_current_page(cx);
        cx.notify();
    }

    pub fn zoom_in(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let center = self.viewport_bounds.get().map(|b| b.center());
        self.set_zoom_level(self.zoom_level * ZOOM_FACTOR, center, cx);
    }

    pub fn zoom_out(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let center = self.viewport_bounds.get().map(|b| b.center());
        self.set_zoom_level(self.zoom_level / ZOOM_FACTOR, center, cx);
    }

    pub fn reset_zoom(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let center = self.viewport_bounds.get().map(|b| b.center());
        self.set_zoom_level(1.0, center, cx);
    }

    /// Returns the effective page dimensions taking document rotation into account.
    pub fn effective_page_size(&self, doc: &dyn PdfDocument, page_index: usize) -> (f32, f32) {
        let (w, h) = doc.page_size(page_index);
        if self.rotation == 90 || self.rotation == 270 {
            (h, w)
        } else {
            (w, h)
        }
    }

    pub fn rotation(&self) -> u16 {
        self.rotation
    }

    pub fn rotate_clockwise(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_rotation((self.rotation + 90) % 360, cx);
    }

    pub fn rotate_counterclockwise(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_rotation((self.rotation + 270) % 360, cx);
    }

    pub fn set_rotation(&mut self, rotation: u16, cx: &mut Context<Self>) {
        let normalized = (rotation / 90) * 90 % 360;
        if self.rotation == normalized {
            return;
        }
        self.rotation = normalized;
        self.render_pool.increment_generation();
        self.clamp_scroll_bounds(cx);
        self.update_current_page(cx);
        cx.notify();
    }

    pub fn fit_to_width(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(vp) = self.viewport_bounds.get() {
            let (page_w, _) = {
                let item = self.pdf_item.read(cx);
                self.effective_page_size(&*item.document(), self.current_page)
            };
            let avail_w: f32 = f32::from(vp.size.width) - (PAGE_MARGIN * 2.0);
            if page_w > 0.0 && avail_w > 0.0 {
                let target_zoom = (avail_w / page_w).clamp(MIN_ZOOM, MAX_ZOOM);
                self.zoom_level = target_zoom;
                self.render_pool.increment_generation();
                self.scroll_offset.x = px(0.0);
                self.clamp_scroll_bounds(cx);
                self.update_current_page(cx);
                cx.notify();
            }
        }
    }

    pub fn fit_to_page(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(vp) = self.viewport_bounds.get() {
            let (page_w, page_h) = {
                let item = self.pdf_item.read(cx);
                self.effective_page_size(&*item.document(), self.current_page)
            };
            let avail_w: f32 = f32::from(vp.size.width) - (PAGE_MARGIN * 2.0);
            let avail_h: f32 = f32::from(vp.size.height) - (PAGE_MARGIN * 2.0);
            if page_w > 0.0 && page_h > 0.0 && avail_w > 0.0 && avail_h > 0.0 {
                let scale_w = avail_w / page_w;
                let scale_h = avail_h / page_h;
                let target_zoom = scale_w.min(scale_h).clamp(MIN_ZOOM, MAX_ZOOM);
                self.zoom_level = target_zoom;
                self.render_pool.increment_generation();
                self.scroll_offset.x = px(0.0);
                self.clamp_scroll_bounds(cx);
                self.update_current_page(cx);
                cx.notify();
            }
        }
    }

    pub fn go_to_page(&mut self, page: usize, cx: &mut Context<Self>) {
        let target = page.clamp(0, self.total_pages.saturating_sub(1));
        self.scroll_to_page(target, cx);
    }

    pub fn next_page(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.current_page + 1 < self.total_pages {
            self.go_to_page(self.current_page + 1, cx);
        }
    }

    pub fn prev_page(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.current_page > 0 {
            self.go_to_page(self.current_page - 1, cx);
        }
    }

    pub fn first_page(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.go_to_page(0, cx);
    }

    pub fn last_page(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.go_to_page(self.total_pages.saturating_sub(1), cx);
    }

    /// Computes vertical offsets (top y position) for all pages at current zoom.
    fn compute_page_y_offsets(&self, doc: &dyn PdfDocument) -> Vec<f32> {
        let mut offsets = Vec::with_capacity(self.total_pages);
        let mut current_y = PAGE_MARGIN;

        for p in 0..self.total_pages {
            offsets.push(current_y);
            let (_, ph) = self.effective_page_size(doc, p);
            let scaled_h = ph * self.zoom_level;
            current_y += scaled_h + PAGE_MARGIN;
        }

        offsets
    }

    /// Determines the page index currently dominating the viewport.
    fn update_current_page(&mut self, cx: &mut Context<Self>) {
        let offsets = {
            let item = self.pdf_item.read(cx);
            self.compute_page_y_offsets(&*item.document())
        };
        let view_top = -f32::from(self.scroll_offset.y);

        let mut current = 0;
        for (idx, &top) in offsets.iter().enumerate() {
            if top <= view_top + 100.0 {
                current = idx;
            } else {
                break;
            }
        }

        if current != self.current_page {
            self.current_page = current;
            cx.emit(PdfViewEvent::PageChanged);
        }
    }

    /// Scrolls the viewport so that the given page is at the top.
    fn scroll_to_page(&mut self, page: usize, cx: &mut Context<Self>) {
        let offsets = {
            let item = self.pdf_item.read(cx);
            self.compute_page_y_offsets(&*item.document())
        };
        if let Some(&top) = offsets.get(page) {
            self.scroll_offset.y = px(-top + PAGE_MARGIN);
            self.current_page = page;
            self.clamp_scroll_bounds(cx);
            cx.emit(PdfViewEvent::PageChanged);
            cx.notify();
        }
    }

    // -----------------------------------------------------------------------
    // Mouse & Gesture Handlers
    // -----------------------------------------------------------------------

    fn handle_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.modifiers.control || event.modifiers.platform {
            // Zoom anchored to cursor
            let delta: f32 = match event.delta {
                ScrollDelta::Pixels(pixels) => pixels.y.into(),
                ScrollDelta::Lines(lines) => lines.y * 24.0,
            };
            let factor = if delta > 0.0 {
                1.0 + (delta.abs() * 0.008).min(0.3)
            } else {
                1.0 / (1.0 + (delta.abs() * 0.008).min(0.3))
            };
            self.set_zoom_level(self.zoom_level * factor, Some(event.position), cx);
        } else {
            // Fast responsive scroll: lines map to 80px (200px with Alt for rapid skimming)
            let line_height = if event.modifiers.alt {
                px(200.0)
            } else {
                px(80.0)
            };
            let mut delta = event.delta.pixel_delta(line_height);
            if event.modifiers.alt && matches!(event.delta, ScrollDelta::Pixels(_)) {
                delta = delta * 2.5;
            }

            // Lock horizontal scroll strictly to 0 when document fits in viewport
            if let Some(vp) = self.viewport_bounds.get() {
                let doc = self.pdf_item.read(cx).document();
                let mut max_scaled_w: f32 = 0.0;
                for p in 0..self.total_pages {
                    let (pw, _) = self.effective_page_size(&*doc, p);
                    max_scaled_w = max_scaled_w.max(pw * self.zoom_level);
                }
                if max_scaled_w <= f32::from(vp.size.width) {
                    delta.x = px(0.0);
                }
            }

            self.scroll_offset += delta;
            self.clamp_scroll_bounds(cx);
            self.update_current_page(cx);
            cx.notify();
        }
    }

    fn handle_pinch(
        &mut self,
        event: &PinchEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let factor = 1.0 + event.delta;
        self.set_zoom_level(self.zoom_level * factor, Some(event.position), cx);
    }

    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Only middle click or alt+left click initiates canvas panning
        if event.button == MouseButton::Middle
            || (event.button == MouseButton::Left && event.modifiers.alt)
        {
            self.last_mouse_position = Some(event.position);
            self.is_panning = true;
            cx.notify();
        }
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_panning {
            self.last_mouse_position = None;
            self.is_panning = false;
            cx.notify();
        }
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_panning {
            if let Some(last_pos) = self.last_mouse_position {
                let mut delta = event.position - last_pos;
                self.last_mouse_position = Some(event.position);

                if let Some(vp) = self.viewport_bounds.get() {
                    let doc = self.pdf_item.read(cx).document();
                    let mut max_scaled_w: f32 = 0.0;
                    for p in 0..self.total_pages {
                        let (pw, _) = self.effective_page_size(&*doc, p);
                        max_scaled_w = max_scaled_w.max(pw * self.zoom_level);
                    }
                    if max_scaled_w <= f32::from(vp.size.width) {
                        delta.x = px(0.0);
                    }
                }

                self.scroll_offset += delta;
                self.clamp_scroll_bounds(cx);
                self.update_current_page(cx);
                cx.notify();
            }
        }
    }

    /// Chrome-accurate scroll boundary clamping:
    /// - If page fits within viewport horizontally, clamp X strictly to 0 (centered).
    /// - If page is wider than viewport, allow horizontal pan within content margins.
    /// - Vertical scroll is clamped to valid document content bounds.
    fn clamp_scroll_bounds(&mut self, cx: &Context<Self>) {
        if let Some(vp) = self.viewport_bounds.get() {
            let item = self.pdf_item.read(cx);
            let doc = item.document();
            let offsets = self.compute_page_y_offsets(&*doc);
            let last_page_h = self.effective_page_size(&*doc, self.total_pages.saturating_sub(1)).1 * self.zoom_level;
            let total_doc_height = offsets.last().copied().unwrap_or(0.0) + last_page_h + PAGE_MARGIN;

            let vp_h = f32::from(vp.size.height);
            let vp_w = f32::from(vp.size.width);

            // Compute max page width
            let mut max_scaled_w: f32 = 0.0;
            for p in 0..self.total_pages {
                let (pw, _) = self.effective_page_size(&*doc, p);
                max_scaled_w = max_scaled_w.max(pw * self.zoom_level);
            }

            // Horizontal bounds:
            if max_scaled_w <= vp_w {
                // Completely fit horizontally: lock to center
                self.scroll_offset.x = px(0.0);
            } else {
                let max_overflow = (max_scaled_w - vp_w) / 2.0 + PAGE_MARGIN;
                let cur_x = f32::from(self.scroll_offset.x);
                self.scroll_offset.x = px(cur_x.clamp(-max_overflow, max_overflow));
            }

            // Vertical bounds:
            let min_y = if total_doc_height <= vp_h {
                0.0
            } else {
                -(total_doc_height - vp_h)
            };
            let max_y = 0.0;

            let cur_y = f32::from(self.scroll_offset.y);
            self.scroll_offset.y = px(cur_y.clamp(min_y, max_y));
        }
    }

    // -----------------------------------------------------------------------
    // Toolbar Rendering
    // -----------------------------------------------------------------------

    fn render_toolbar(&self, cx: &Context<Self>) -> impl IntoElement {
        let current_display_page = self.current_page + 1;
        let total = self.total_pages;
        let zoom_pct = (self.zoom_level * 100.0).round() as u32;

        h_flex()
            .w_full()
            .h_9()
            .px_3()
            .gap_2()
            .items_center()
            .justify_between()
            .bg(cx.theme().colors().toolbar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            // Left: Page Navigation
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        IconButton::new("first-page", IconName::ArrowLeft)
                            .tooltip(Tooltip::text("First Page (Home)"))
                            .disabled(self.current_page == 0)
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.first_page(window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("prev-page", IconName::ChevronLeft)
                            .tooltip(Tooltip::text("Previous Page (PageUp)"))
                            .disabled(self.current_page == 0)
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.prev_page(window, cx);
                            })),
                    )
                    .child(
                        Label::new(format!("Page {current_display_page} of {total}"))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        IconButton::new("next-page", IconName::ChevronRight)
                            .tooltip(Tooltip::text("Next Page (PageDown)"))
                            .disabled(self.current_page + 1 >= self.total_pages)
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.next_page(window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("last-page", IconName::ArrowRight)
                            .tooltip(Tooltip::text("Last Page (End)"))
                            .disabled(self.current_page + 1 >= self.total_pages)
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.last_page(window, cx);
                            })),
                    ),
            )
            // Right: Zoom Controls
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        IconButton::new("zoom-out", IconName::Dash)
                            .tooltip(Tooltip::text("Zoom Out (Cmd/Ctrl + -)"))
                            .disabled(self.zoom_level <= MIN_ZOOM)
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.zoom_out(window, cx);
                            })),
                    )
                    .child(
                        Button::new("zoom-reset", format!("{zoom_pct}%"))
                            .tooltip(Tooltip::text("Reset Zoom (Cmd/Ctrl + 0)"))
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.reset_zoom(window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("zoom-in", IconName::Plus)
                            .tooltip(Tooltip::text("Zoom In (Cmd/Ctrl + =)"))
                            .disabled(self.zoom_level >= MAX_ZOOM)
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.zoom_in(window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("fit-width", IconName::Maximize)
                            .tooltip(Tooltip::text("Fit to Width"))
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.fit_to_width(window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("fit-page", IconName::Screen)
                            .tooltip(Tooltip::text("Fit to Page"))
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.fit_to_page(window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("rotate-cw", IconName::RotateCw)
                            .tooltip(Tooltip::text("Rotate Clockwise (R)"))
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.rotate_clockwise(window, cx);
                            })),
                    ),
            )
    }

    // -----------------------------------------------------------------------
    // Document Canvas & Progressive Tile Rendering
    // -----------------------------------------------------------------------

    fn render_document_canvas(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        // Drain any pending completed tiles before computing canvas elements
        self.drain_render_results();

        let Some(vp) = self.viewport_bounds.get() else {
            return div().id("empty-canvas").into_any_element();
        };

        let vp_w = f32::from(vp.size.width);
        let vp_h = f32::from(vp.size.height);
        let scroll_x = f32::from(self.scroll_offset.x);
        let scroll_y = f32::from(self.scroll_offset.y);

        let pdf_item = self.pdf_item.clone();
        let item = pdf_item.read(cx);
        let doc = item.document();
        let y_offsets = self.compute_page_y_offsets(&*doc);
        let last_page_h = self.effective_page_size(&*doc, self.total_pages.saturating_sub(1)).1 * self.zoom_level;
        let total_doc_height = y_offsets.last().copied().unwrap_or(0.0) + last_page_h + PAGE_MARGIN;

        // If entire document fits vertically within viewport, center it nicely
        let doc_top_offset = if total_doc_height < vp_h {
            ((vp_h - total_doc_height) / 2.0).max(PAGE_MARGIN) - PAGE_MARGIN
        } else {
            0.0
        };

        let mut pages_container = div()
            .id("pdf-pages-canvas")
            .size_full()
            .relative();

        let mut min_visible_page = self.total_pages;
        let mut max_visible_page = 0;

        for (page_idx, &page_top) in y_offsets.iter().enumerate() {
            let (page_w, page_h) = self.effective_page_size(&*doc, page_idx);
            let scaled_w = page_w * self.zoom_level;
            let scaled_h = page_h * self.zoom_level;

            let page_screen_top = scroll_y + page_top + doc_top_offset;
            let page_screen_bottom = page_screen_top + scaled_h;

            // Viewport intersection test for page (with 800px buffer to prefetch smoothly)
            if page_screen_bottom < -800.0 || page_screen_top > vp_h + 800.0 {
                continue; // Cull distant page
            }

            min_visible_page = min_visible_page.min(page_idx);
            max_visible_page = max_visible_page.max(page_idx);

            // Chrome PDF Viewer centering:
            // When document width is less than viewport, center it precisely (no horizontal offset).
            // When zoomed wider than viewport, apply horizontal scroll offset.
            let page_screen_left = if scaled_w <= vp_w {
                (vp_w - scaled_w) / 2.0
            } else {
                (vp_w - scaled_w) / 2.0 + scroll_x
            };

            // Progressive background: check if low-res page overview is available
            let page_overview = self.tile_cache.get_page_overview(page_idx, self.rotation);
            if page_overview.is_none() {
                // Request fast low-res overview for smooth scrolling preview (0.75x scale)
                self.render_pool.request_page_overview(page_idx, self.rotation, 0.75, doc.clone());
            }

            // Chrome page aesthetic: pure white sheet with subtle realistic drop shadow
            let mut page_div = div()
                .id(("pdf-page", page_idx))
                .absolute()
                .left(px(page_screen_left))
                .top(px(page_screen_top))
                .w(px(scaled_w))
                .h(px(scaled_h))
                .bg(gpui::white())
                .shadow_lg()
                .border_1()
                .border_color(cx.theme().colors().border_variant);

            // If overview is cached, render it as underlying backdrop so there's never a blank or gray flash
            if let Some(overview) = page_overview {
                page_div = page_div.child(
                    div()
                        .size_full()
                        .absolute()
                        .top_0()
                        .left_0()
                        .child(img(overview).size_full()),
                );
            }

            // Compute tile grid for this page
            let tile_size = TileKey::TILE_SIZE as f32;
            let num_tiles_x = (scaled_w / tile_size).ceil() as u32;
            let num_tiles_y = (scaled_h / tile_size).ceil() as u32;

            for ty in 0..num_tiles_y {
                for tx in 0..num_tiles_x {
                    let tile_crop_x = (tx as f32 * tile_size) as u32;
                    let tile_crop_y = (ty as f32 * tile_size) as u32;
                    let cur_tile_w = (((tx + 1) as f32 * tile_size).min(scaled_w) - tile_crop_x as f32).round() as u32;
                    let cur_tile_h = (((ty + 1) as f32 * tile_size).min(scaled_h) - tile_crop_y as f32).round() as u32;

                    if cur_tile_w == 0 || cur_tile_h == 0 {
                        continue;
                    }

                    let tile_screen_left = page_screen_left + tile_crop_x as f32;
                    let tile_screen_top = page_screen_top + tile_crop_y as f32;

                    // Viewport intersection test for tile (with 200px buffer)
                    let outside_x = (tile_screen_left + cur_tile_w as f32) < -200.0 || tile_screen_left > vp_w + 200.0;
                    let outside_y = (tile_screen_top + cur_tile_h as f32) < -200.0 || tile_screen_top > vp_h + 200.0;
                    if outside_x || outside_y {
                        continue;
                    }

                    let tile_key = TileKey::new(page_idx, self.zoom_level, self.rotation, tx, ty);

                    let tile_element = if let Some(cached_img) = self.tile_cache.get(&tile_key) {
                        img(cached_img)
                            .size_full()
                            .into_any_element()
                    } else {
                        // Request background render for missing tile
                        self.render_pool.request_tile(
                            tile_key,
                            self.zoom_level,
                            tile_crop_x,
                            tile_crop_y,
                            cur_tile_w,
                            cur_tile_h,
                            doc.clone(),
                        );

                        // Progressive fallback: transparent so underlying overview image displays without being covered
                        div()
                            .size_full()
                            .into_any_element()
                    };

                    page_div = page_div.child(
                        div()
                            .absolute()
                            .left(px(tile_crop_x as f32))
                            .top(px(tile_crop_y as f32))
                            .w(px(cur_tile_w as f32))
                            .h(px(cur_tile_h as f32))
                            .child(tile_element),
                    );
                }
            }

            pages_container = pages_container.child(page_div);
        }

        // Prefetch page overviews for pages approaching the viewport
        if min_visible_page <= max_visible_page {
            let prefetch_start = min_visible_page.saturating_sub(1);
            let prefetch_end = (max_visible_page + 2).min(self.total_pages.saturating_sub(1));
            for p in prefetch_start..=prefetch_end {
                if self.tile_cache.get_page_overview(p, self.rotation).is_none() {
                    self.render_pool.request_page_overview(p, self.rotation, 0.75, doc.clone());
                }
            }

            let buffer_min = min_visible_page.saturating_sub(3);
            let buffer_max = max_visible_page + 3;
            self.tile_cache.evict_pages_outside(buffer_min, buffer_max);
        }

        pages_container.into_any_element()
    }
}

// ---------------------------------------------------------------------------
// Render implementation
// ---------------------------------------------------------------------------

impl Render for PdfView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport_bounds_ref = self.viewport_bounds.clone();

        v_flex()
            .key_context("PdfView")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(|this, _: &NextPage, window, cx| this.next_page(window, cx)))
            .on_action(cx.listener(|this, _: &PrevPage, window, cx| this.prev_page(window, cx)))
            .on_action(cx.listener(|this, _: &FirstPage, window, cx| this.first_page(window, cx)))
            .on_action(cx.listener(|this, _: &LastPage, window, cx| this.last_page(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomIn, window, cx| this.zoom_in(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomOut, window, cx| this.zoom_out(window, cx)))
            .on_action(cx.listener(|this, _: &ResetZoom, window, cx| this.reset_zoom(window, cx)))
            .on_action(cx.listener(|this, _: &FitToWidth, window, cx| this.fit_to_width(window, cx)))
            .on_action(cx.listener(|this, _: &FitToPage, window, cx| this.fit_to_page(window, cx)))
            .on_action(cx.listener(|this, _: &RotateClockwise, window, cx| this.rotate_clockwise(window, cx)))
            .on_action(cx.listener(|this, _: &RotateCounterclockwise, window, cx| this.rotate_counterclockwise(window, cx)))
            .child(self.render_toolbar(cx))
            // Viewport container capturing gestures and drawing visible tiles
            .child(
                div()
                    .id("pdf-viewport")
                    .flex_1()
                    .size_full()
                    .overflow_hidden()
                    .cursor(if self.is_panning {
                        gpui::CursorStyle::ClosedHand
                    } else {
                        gpui::CursorStyle::Arrow
                    })
                    .on_scroll_wheel(cx.listener(Self::handle_scroll_wheel))
                    .on_pinch(cx.listener(Self::handle_pinch))
                    .on_mouse_down(MouseButton::Middle, cx.listener(Self::handle_mouse_down))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
                    .on_mouse_up(MouseButton::Middle, cx.listener(Self::handle_mouse_up))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                    .on_mouse_move(cx.listener(Self::handle_mouse_move))
                    .child(
                        canvas(
                            move |_bounds, _window, _cx| {},
                            move |bounds, _, _window, _cx| {
                                viewport_bounds_ref.set(Some(bounds));
                            },
                        )
                        .size_full()
                        .absolute(),
                    )
                    .child(self.render_document_canvas(cx)),
            )
    }
}

// ---------------------------------------------------------------------------
// Item & ProjectItem implementation
// ---------------------------------------------------------------------------

impl Item for PdfView {
    type Event = PdfViewEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        match event {
            PdfViewEvent::PageChanged => {
                f(ItemEvent::UpdateBreadcrumbs);
            }
        }
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        f(self.pdf_item.entity_id(), self.pdf_item.read(cx))
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let abs_path = self.pdf_item.read(cx).abs_path()?;
        Some(abs_path.to_string_lossy().to_string().into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        let project_path = self.pdf_item.read(cx).project_path();

        let label_color = if ItemSettings::get_global(cx).git_status {
            let git_status = self
                .project
                .read(cx)
                .git_store()
                .read(cx)
                .display_status_for_project_path(project_path, cx)
                .map(|status| status.summary())
                .unwrap_or_default();

            self.project
                .read(cx)
                .entry_for_path(project_path, cx)
                .map(|entry| {
                    entry_git_aware_label_color(git_status, entry.is_ignored, params.selected)
                })
                .unwrap_or_else(|| params.text_color())
        } else {
            params.text_color()
        };

        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .single_line()
            .color(label_color)
            .when(params.preview, |this| this.italic())
            .into_any_element()
    }

    fn tab_content_text(&self, _: usize, cx: &App) -> SharedString {
        self.pdf_item.read(cx).file_name().to_string().into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = self.pdf_item.read(cx).abs_path()?;
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(path, cx))
            .flatten()
            .map(Icon::from_path)
    }

    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation {
        let show_breadcrumb = EditorSettings::get_global(cx).toolbar.breadcrumbs;
        if show_breadcrumb {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, cx: &App) -> Option<(Vec<HighlightedText>, Option<Font>)> {
        let file_name = self.pdf_item.read(cx).file_name().to_string();
        let page = self.current_page + 1;
        let total = self.total_pages;
        let font = ThemeSettings::get_global(cx).buffer_font.clone();

        Some((
            vec![
                HighlightedText {
                    text: file_name.into(),
                    highlights: vec![],
                },
                HighlightedText {
                    text: format!(" (Page {page}/{total})").into(),
                    highlights: vec![],
                },
            ],
            Some(font),
        ))
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        let pdf_item = self.pdf_item.clone();
        let project = self.project.clone();
        let current_page = self.current_page;
        let zoom_level = self.zoom_level;
        let rotation = self.rotation;
        let scroll_offset = self.scroll_offset;

        Task::ready(Some(cx.new(|cx| {
            let mut clone = Self::new(pdf_item, project, window, cx);
            clone.current_page = current_page;
            clone.zoom_level = zoom_level;
            clone.rotation = rotation;
            clone.scroll_offset = scroll_offset;
            clone
        })))
    }
}

impl ProjectItem for PdfView {
    type Item = PdfItem;

    fn for_project_item(
        project: Entity<Project>,
        _pane: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self::new(item, project, window, cx)
    }

    fn for_broken_project_item(
        abs_path: &Path,
        is_local: bool,
        e: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InvalidItemView>
    where
        Self: Sized,
    {
        Some(InvalidItemView::new(abs_path, is_local, e, window, cx))
    }
}

impl SerializableItem for PdfView {
    fn serialized_item_kind() -> &'static str {
        "PdfView"
    }

    fn deserialize(
        project: Entity<Project>,
        _workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<Entity<Self>>> {
        let db = PdfViewerDb::global(cx);
        window.spawn(cx, async move |cx| {
            let (pdf_path, saved_page, saved_zoom, scroll_x, scroll_y, saved_rotation) = db
                .get_pdf_state(item_id, workspace_id)?
                .context("No saved PDF state found")?;

            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(pdf_path.clone(), false, cx)
                })
                .await
                .context("Worktree path not found")?;

            let worktree_id = worktree.update(cx, |worktree, _cx| worktree.id());
            let project_path = ProjectPath {
                worktree_id,
                path: relative_path,
            };

            let pdf_item_task = cx.update(|_window, cx| {
                <PdfItem as project::ProjectItem>::try_open(&project, &project_path, cx)
            })?;
            let pdf_item = pdf_item_task
                .context("Failed to open PDF item")?
                .await?;

            cx.update(|window, cx| {
                Ok(cx.new(|cx| {
                    let mut view = Self::new(pdf_item, project, window, cx);
                    view.current_page = saved_page;
                    view.zoom_level = saved_zoom;
                    view.rotation = saved_rotation;
                    view.scroll_offset = point(px(scroll_x), px(scroll_y));
                    view
                }))
            })?
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<()>> {
        let db = PdfViewerDb::global(cx);
        delete_unloaded_items(alive_items, workspace_id, "pdf_viewers", &db, cx)
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<anyhow::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let pdf_path = self.pdf_item.read(cx).abs_path()?.clone();
        let page = self.current_page;
        let zoom = self.zoom_level;
        let rotation = self.rotation;
        let scroll_x: f32 = self.scroll_offset.x.into();
        let scroll_y: f32 = self.scroll_offset.y.into();

        let db = PdfViewerDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_pdf_state(item_id, workspace_id, pdf_path, page, zoom, scroll_x, scroll_y, rotation).await
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

/// Registers the PDF viewer with Zed's workspace and serializable item registries.
pub fn init(cx: &mut App) {
    workspace::register_project_item::<PdfView>(cx);
    workspace::register_serializable_item::<PdfView>(cx);
}

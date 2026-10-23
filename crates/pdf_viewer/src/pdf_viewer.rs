mod pdf_document;
mod pdf_item;

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use editor::{EditorSettings, RevealInFileManager, items::entry_git_aware_label_color};
use file_icons::FileIcons;
use gpui::{
    AnyElement, App, Bounds, ClipboardItem, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Font, Hsla, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    PinchEvent, Pixels, Point, Render, RenderImage, ScrollDelta, ScrollEvent, ScrollHandle, Size,
    Subscription, Task, WeakEntity, Window, actions, div, img, point, px, size,
};
use hayro::kurbo::Point as PagePoint;
use parking_lot::{Condvar, Mutex};
use persistence::PdfViewerDb;
use project::{Project, ProjectPath, git_store::GitStoreEvent};
use settings::Settings as _;
use smallvec::SmallVec;
use theme_settings::ThemeSettings;
use ui::{Scrollbars, Tooltip, WithScrollbar as _, prelude::*};
use util::{ResultExt as _, paths::PathExt as _};
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    WorkspaceId, delete_unloaded_items,
    invalid_item_view::InvalidItemView,
    item::{HighlightedText, Item, ItemHandle, ProjectItem, SerializableItem, TabContentParams},
};

use crate::pdf_document::{PageRenderer, PageText, PdfDocument};
pub use crate::pdf_item::{PdfItem, PdfItemEvent, is_pdf_file};

actions!(
    pdf_viewer,
    [
        /// Zoom in the PDF.
        ZoomIn,
        /// Zoom out the PDF.
        ZoomOut,
        /// Reset the zoom so pages fit the width of the view.
        ResetZoom,
        /// Fit the pages to the width of the view.
        FitToView,
        /// Zoom to the actual size of the pages (100%).
        ZoomToActualSize,
        /// Copy the selected text.
        Copy,
        /// Select the text of the whole document.
        SelectAll,
    ]
);

/// A zoom of 1.0 shows one PDF point as one logical pixel.
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 8.0;
const ZOOM_STEP: f32 = 1.1;
const SCROLL_ZOOM_SENSITIVITY: f32 = 0.01;
const SCROLL_LINE_HEIGHT: f32 = 20.0;
const KEY_SCROLL_STEP: Pixels = px(40.0);
const PAGE_GAP: Pixels = px(16.0);
const PAGE_MARGIN: Pixels = px(24.0);
/// Pages kept rendered above and below the visible ones, so short scrolls show no blank pages.
const BUFFER_PAGES: usize = 1;
/// Caps the memory of one rendered page at 64 MB. A page larger than this at the current zoom
/// is rendered at a lower resolution and scaled up.
const MAX_PAGE_PIXELS: f32 = 4096.0 * 4096.0;
/// While the zoom changes, pages keep their old rendering and are re-rendered after this delay.
const RENDER_DEBOUNCE: Duration = Duration::from_millis(150);
/// hayro interprets nested content streams recursively, which can exceed the default stack size
/// of GPUI's background threads.
const WORKER_STACK_SIZE: usize = 8 * 1024 * 1024;
const SELECTION_COLOR: u32 = 0x3584e44d;

pub struct PdfViewer {
    pdf_item: Entity<PdfItem>,
    project: Entity<Project>,
    document: Arc<PdfDocument>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    zoom: f32,
    fit_width: bool,
    zoom_changed_at: Option<Instant>,
    current_page: usize,
    kept_pages: Range<usize>,
    rendered_pages: HashMap<usize, RenderedPage>,
    requested_renders: HashMap<usize, f32>,
    failed_pages: HashSet<usize>,
    page_texts: HashMap<usize, PageText>,
    requested_texts: HashSet<usize>,
    selection: Option<Selection>,
    is_selecting: bool,
    worker: Worker,
    _render_debounce: Task<()>,
    _receive_responses: Task<()>,
    _subscriptions: Vec<Subscription>,
}

struct RenderedPage {
    scale: f32,
    image: Arc<RenderImage>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TextPosition {
    page: usize,
    caret: usize,
}

#[derive(Clone, Copy, Debug)]
struct Selection {
    anchor: TextPosition,
    head: TextPosition,
}

impl Selection {
    fn ordered(&self) -> (TextPosition, TextPosition) {
        if self.anchor <= self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// The selected caret range on `page`, or `None` when the selection does not reach it.
    fn range_on_page(&self, page: usize) -> Option<Range<usize>> {
        let (start, end) = self.ordered();
        if self.is_empty() || page < start.page || page > end.page {
            return None;
        }
        let range_start = if page == start.page { start.caret } else { 0 };
        let range_end = if page == end.page {
            end.caret
        } else {
            usize::MAX
        };
        Some(range_start..range_end)
    }
}

pub enum PdfViewEvent {
    TitleChanged,
}

impl PdfViewer {
    pub fn new(
        pdf_item: Entity<PdfItem>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let document = pdf_item.read(cx).document().clone();
        let (worker, receive_responses) = Self::spawn_worker(document.clone(), window, cx);
        let git_store = project.read(cx).git_store().clone();
        let subscriptions = vec![
            cx.subscribe_in(&pdf_item, window, Self::on_pdf_item_event),
            cx.subscribe(&git_store, |_, _, event, cx| {
                if matches!(event, GitStoreEvent::DiffBaseChanged(_)) {
                    cx.emit(PdfViewEvent::TitleChanged);
                }
            }),
        ];
        cx.on_release_in(window, |this, window, _| {
            for (_, page) in this.rendered_pages.drain() {
                window.drop_image(page.image).log_err();
            }
        })
        .detach();

        Self {
            pdf_item,
            project,
            document,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            zoom: 1.0,
            fit_width: true,
            zoom_changed_at: None,
            current_page: 0,
            kept_pages: 0..0,
            rendered_pages: HashMap::default(),
            requested_renders: HashMap::default(),
            failed_pages: HashSet::default(),
            page_texts: HashMap::default(),
            requested_texts: HashSet::default(),
            selection: None,
            is_selecting: false,
            worker,
            _render_debounce: Task::ready(()),
            _receive_responses: receive_responses,
            _subscriptions: subscriptions,
        }
    }

    fn on_pdf_item_event(
        &mut self,
        pdf_item: &Entity<PdfItem>,
        event: &PdfItemEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            PdfItemEvent::Reloaded => {
                let document = pdf_item.read(cx).document().clone();
                let (worker, receive_responses) = Self::spawn_worker(document.clone(), window, cx);
                self.worker = worker;
                self._receive_responses = receive_responses;
                self.document = document;
                for (_, page) in self.rendered_pages.drain() {
                    window.drop_image(page.image).log_err();
                }
                self.requested_renders.clear();
                self.failed_pages.clear();
                self.page_texts.clear();
                self.requested_texts.clear();
                self.selection = None;
                self.is_selecting = false;
                cx.notify();
            }
            PdfItemEvent::FileHandleChanged => {
                cx.emit(PdfViewEvent::TitleChanged);
                cx.notify();
            }
        }
    }

    fn spawn_worker(
        document: Arc<PdfDocument>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Worker, Task<()>) {
        let (worker, responses) = Worker::spawn(document);
        let receive_responses = cx.spawn_in(window, async move |this, cx| {
            while let Ok(response) = responses.recv().await {
                if this
                    .update_in(cx, |this, window, cx| {
                        this.handle_worker_response(response, window, cx)
                    })
                    .is_err()
                {
                    break;
                }
            }
        });
        (worker, receive_responses)
    }

    fn handle_worker_response(
        &mut self,
        response: WorkerResponse,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match response {
            WorkerResponse::Rendered { page, scale, image } => {
                if self.requested_renders.get(&page) == Some(&scale) {
                    self.requested_renders.remove(&page);
                }
                if self.kept_pages.contains(&page) {
                    if let Some(previous) = self
                        .rendered_pages
                        .insert(page, RenderedPage { scale, image })
                    {
                        window.drop_image(previous.image).log_err();
                    }
                    cx.notify();
                }
            }
            WorkerResponse::Text { page, text } => {
                self.requested_texts.remove(&page);
                if self.kept_pages.contains(&page) {
                    self.page_texts.insert(page, text);
                    cx.notify();
                }
            }
            WorkerResponse::Copied { text } => {
                cx.write_to_clipboard(ClipboardItem::new_string(text));
            }
            WorkerResponse::Failed { page, error } => {
                log::error!("Failed to render page {} of PDF: {error:#}", page + 1);
                self.requested_renders.remove(&page);
                self.failed_pages.insert(page);
                cx.notify();
            }
        }
    }

    fn layout(&self, viewport_width: Pixels) -> DocumentLayout {
        DocumentLayout::new(self.document.page_sizes(), self.zoom, viewport_width)
    }

    fn fit_width_zoom(&self, viewport_width: Pixels) -> f32 {
        let widest_page = self
            .document
            .page_sizes()
            .iter()
            .map(|page| page.width)
            .fold(0.0, f32::max);
        if widest_page <= 0.0 {
            return 1.0;
        }
        (f32::from(viewport_width - PAGE_MARGIN * 2.0) / widest_page).clamp(MIN_ZOOM, MAX_ZOOM)
    }

    /// Changes the zoom and keeps the document point under `anchor` (window coordinates) in
    /// place. Without an anchor, the center of the view stays in place.
    fn set_zoom(&mut self, zoom: f32, anchor: Option<Point<Pixels>>, cx: &mut Context<Self>) {
        let zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        self.fit_width = false;
        if (zoom - self.zoom).abs() < f32::EPSILON {
            return;
        }

        let viewport = self.scroll_handle.bounds();
        if viewport.size.width > Pixels::ZERO {
            let anchor = anchor.unwrap_or_else(|| viewport.center()) - viewport.origin;
            let content_point = anchor - self.scroll_handle.offset();
            let (page, page_point) = self
                .layout(viewport.size.width)
                .document_point(content_point, self.zoom);
            self.zoom = zoom;
            let new_content_point = self
                .layout(viewport.size.width)
                .content_point(page, page_point, self.zoom);
            let offset = anchor - new_content_point;
            self.scroll_handle.set_offset(point(
                offset.x.min(Pixels::ZERO),
                offset.y.min(Pixels::ZERO),
            ));
        } else {
            self.zoom = zoom;
        }

        self.start_render_debounce(cx);
        cx.notify();
    }

    fn start_render_debounce(&mut self, cx: &mut Context<Self>) {
        self.zoom_changed_at = Some(Instant::now());
        self._render_debounce = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(RENDER_DEBOUNCE).await;
            this.update(cx, |_, cx| cx.notify()).ok();
        });
    }

    fn zoom_in(&mut self, _: &ZoomIn, _: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom * ZOOM_STEP, None, cx);
    }

    fn zoom_out(&mut self, _: &ZoomOut, _: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom / ZOOM_STEP, None, cx);
    }

    fn reset_zoom(&mut self, _: &ResetZoom, window: &mut Window, cx: &mut Context<Self>) {
        self.fit_to_view(&FitToView, window, cx);
    }

    fn fit_to_view(&mut self, _: &FitToView, _: &mut Window, cx: &mut Context<Self>) {
        let viewport_width = self.scroll_handle.bounds().size.width;
        if viewport_width > Pixels::ZERO {
            self.set_zoom(self.fit_width_zoom(viewport_width), None, cx);
        }
        self.fit_width = true;
        cx.notify();
    }

    fn zoom_to_actual_size(
        &mut self,
        _: &ZoomToActualSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_zoom(1.0, None, cx);
    }

    fn reveal_in_file_manager(
        &mut self,
        _: &RevealInFileManager,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(path) = self.pdf_item.read(cx).abs_path(cx) {
            self.project
                .update(cx, |project, cx| project.reveal_path(&path, cx));
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, _: &mut Context<Self>) {
        if let Some(selection) = self.selection.filter(|selection| !selection.is_empty()) {
            let (start, end) = selection.ordered();
            self.worker.copy_text(start, end);
        }
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selection = Some(Selection {
            anchor: TextPosition { page: 0, caret: 0 },
            head: TextPosition {
                page: self.document.page_count() - 1,
                caret: usize::MAX,
            },
        });
        cx.notify();
    }

    fn handle_scroll(&mut self, event: &ScrollEvent, _: &mut Window, cx: &mut Context<Self>) {
        if !(event.modifiers.control || event.modifiers.platform) {
            return;
        }
        cx.stop_propagation();
        let delta = match event.delta {
            ScrollDelta::Pixels(pixels) => f32::from(pixels.y),
            ScrollDelta::Lines(lines) => lines.y * SCROLL_LINE_HEIGHT,
        };
        let factor = 1.0 + delta.abs() * SCROLL_ZOOM_SENSITIVITY;
        let zoom = if delta > 0.0 {
            self.zoom * factor
        } else {
            self.zoom / factor
        };
        self.set_zoom(zoom, Some(event.position), cx);
    }

    fn handle_pinch(&mut self, event: &PinchEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom * (1.0 + event.delta), Some(event.position), cx);
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let modifiers = &event.keystroke.modifiers;
        if modifiers.control || modifiers.platform || modifiers.alt {
            return;
        }
        let viewport = self.scroll_handle.bounds().size;
        let page_step = viewport.height * 0.9;
        let offset = self.scroll_handle.offset();
        let max_offset = self.scroll_handle.max_offset();
        let new_y = match (event.keystroke.key.as_str(), modifiers.shift) {
            ("up", _) => offset.y + KEY_SCROLL_STEP,
            ("down", _) => offset.y - KEY_SCROLL_STEP,
            ("pageup", _) | ("space", true) => offset.y + page_step,
            ("pagedown", _) | ("space", false) => offset.y - page_step,
            ("home", _) => Pixels::ZERO,
            ("end", _) => -max_offset.y,
            _ => return,
        };
        cx.stop_propagation();
        self.scroll_handle
            .set_offset(point(offset.x, new_y.clamp(-max_offset.y, Pixels::ZERO)));
        cx.notify();
    }

    /// Maps a window position to a caret in the page text nearest to it. Returns `None` when the
    /// text of that page is not extracted yet.
    fn text_position_at(&self, position: Point<Pixels>) -> Option<TextPosition> {
        let viewport = self.scroll_handle.bounds();
        if viewport.size.width <= Pixels::ZERO {
            return None;
        }
        let content_point = position - viewport.origin - self.scroll_handle.offset();
        let (page, page_point) = self
            .layout(viewport.size.width)
            .document_point(content_point, self.zoom);
        let caret = self.page_texts.get(&page)?.caret_at(page_point)?;
        Some(TextPosition { page, caret })
    }

    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus_handle.focus(window, cx);
        self.selection = self
            .text_position_at(event.position)
            .map(|position| Selection {
                anchor: position,
                head: position,
            });
        self.is_selecting = self.selection.is_some();
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_selecting {
            return;
        }
        if event.pressed_button != Some(MouseButton::Left) {
            self.is_selecting = false;
            return;
        }
        if let Some(position) = self.text_position_at(event.position)
            && let Some(selection) = self.selection.as_mut()
            && selection.head != position
        {
            selection.head = position;
            cx.notify();
        }
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.is_selecting = false;
        if self.selection.is_some_and(|selection| selection.is_empty()) {
            self.selection = None;
            cx.notify();
        }
    }

    /// Requests renders and text for the pages around the visible ones, and frees the rest.
    fn update_page_cache(&mut self, visible: Range<usize>, window: &mut Window) {
        let page_count = self.document.page_count();
        let kept = visible.start.saturating_sub(BUFFER_PAGES)
            ..(visible.end + BUFFER_PAGES).min(page_count);
        self.kept_pages = kept.clone();

        self.rendered_pages.retain(|page, rendered| {
            let keep = kept.contains(page);
            if !keep {
                window.drop_image(rendered.image.clone()).log_err();
            }
            keep
        });
        self.page_texts.retain(|page, _| kept.contains(page));
        self.requested_renders.retain(|page, _| kept.contains(page));
        self.requested_texts.retain(|page| kept.contains(page));

        let zoom_settling = self
            .zoom_changed_at
            .is_some_and(|changed_at| changed_at.elapsed() < RENDER_DEBOUNCE);
        let device_scale = self.zoom * window.scale_factor();
        let mut new_renders = Vec::new();
        let render_order = visible
            .clone()
            .chain(kept.start..visible.start)
            .chain(visible.end..kept.end);
        for page in render_order {
            if self.failed_pages.contains(&page) {
                continue;
            }
            let page_size = self.document.page_sizes()[page];
            let max_scale =
                (MAX_PAGE_PIXELS / (page_size.width * page_size.height).max(1.0)).sqrt();
            let scale = device_scale.min(max_scale);
            let is_close = |other: f32| (other - scale).abs() <= scale * 0.01;
            if self
                .rendered_pages
                .get(&page)
                .is_some_and(|rendered| zoom_settling || is_close(rendered.scale))
            {
                self.requested_renders.remove(&page);
                continue;
            }
            if self
                .requested_renders
                .get(&page)
                .is_some_and(|requested| is_close(*requested))
            {
                continue;
            }
            self.requested_renders.insert(page, scale);
            new_renders.push((page, scale));
        }

        let mut new_texts = Vec::new();
        for page in visible {
            if !self.page_texts.contains_key(&page) && self.requested_texts.insert(page) {
                new_texts.push(page);
            }
        }
        self.worker
            .sync(&self.requested_renders, new_renders, kept, new_texts);
    }

    fn render_page(&self, page: usize, bounds: Bounds<Pixels>) -> AnyElement {
        let highlight_color = Hsla::from(gpui::rgba(SELECTION_COLOR));
        let highlights = self
            .selection
            .and_then(|selection| selection.range_on_page(page))
            .zip(self.page_texts.get(&page))
            .map(|(range, text)| text.selection_rects(range))
            .unwrap_or_default()
            .into_iter()
            .map(|rect| {
                div()
                    .absolute()
                    .left(px(rect.x0 as f32 * self.zoom))
                    .top(px(rect.y0 as f32 * self.zoom))
                    .w(px(rect.width() as f32 * self.zoom))
                    .h(px(rect.height() as f32 * self.zoom))
                    .bg(highlight_color)
            });

        div()
            .absolute()
            .left(bounds.origin.x)
            .top(bounds.origin.y)
            .w(bounds.size.width)
            .h(bounds.size.height)
            .bg(gpui::white())
            .shadow_sm()
            .when_some(self.rendered_pages.get(&page), |this, rendered| {
                this.child(
                    img(rendered.image.clone())
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full(),
                )
            })
            .when(self.failed_pages.contains(&page), |this| {
                this.flex().items_center().justify_center().child(
                    Label::new(format!("Page {} could not be rendered", page + 1))
                        .color(Color::Error),
                )
            })
            .children(highlights)
            .into_any_element()
    }
}

/// Positions of all pages in the scrollable content, in logical pixels.
struct DocumentLayout {
    page_bounds: Vec<Bounds<Pixels>>,
    content_size: Size<Pixels>,
}

impl DocumentLayout {
    fn new(page_sizes: &[pdf_document::PageSize], zoom: f32, viewport_width: Pixels) -> Self {
        let widest_page = page_sizes.iter().map(|page| page.width).fold(0.0, f32::max);
        let content_width = (px(widest_page * zoom) + PAGE_MARGIN * 2.0).max(viewport_width);
        let mut top = PAGE_GAP;
        let page_bounds = page_sizes
            .iter()
            .map(|page| {
                let page_size = size(px(page.width * zoom), px(page.height * zoom));
                let bounds = Bounds::new(
                    point((content_width - page_size.width) / 2.0, top),
                    page_size,
                );
                top += page_size.height + PAGE_GAP;
                bounds
            })
            .collect();
        Self {
            page_bounds,
            content_size: size(content_width, top),
        }
    }

    fn pages_in(&self, top: Pixels, bottom: Pixels) -> Range<usize> {
        let start = self
            .page_bounds
            .partition_point(|bounds| bounds.bottom() <= top);
        let end = self
            .page_bounds
            .partition_point(|bounds| bounds.top() < bottom);
        start..end.max(start)
    }

    /// The page nearest to `content_point` and the point on it in PDF points.
    fn document_point(&self, content_point: Point<Pixels>, zoom: f32) -> (usize, PagePoint) {
        let page = self
            .page_bounds
            .partition_point(|bounds| bounds.bottom() + PAGE_GAP / 2.0 <= content_point.y)
            .min(self.page_bounds.len().saturating_sub(1));
        let local = content_point - self.page_bounds[page].origin;
        let page_point = PagePoint::new(
            f64::from(f32::from(local.x) / zoom),
            f64::from(f32::from(local.y) / zoom),
        );
        (page, page_point)
    }

    fn content_point(&self, page: usize, page_point: PagePoint, zoom: f32) -> Point<Pixels> {
        let origin = self.page_bounds[page].origin;
        origin
            + point(
                px(page_point.x as f32 * zoom),
                px(page_point.y as f32 * zoom),
            )
    }
}

impl EventEmitter<PdfViewEvent> for PdfViewer {}

impl Focusable for PdfViewer {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PdfViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport = self.scroll_handle.bounds();
        let content = if viewport.size.width > Pixels::ZERO {
            if self.fit_width {
                let zoom = self.fit_width_zoom(viewport.size.width);
                if (zoom - self.zoom).abs() > f32::EPSILON {
                    self.zoom = zoom;
                    self.start_render_debounce(cx);
                }
            }
            let layout = self.layout(viewport.size.width);
            // A scroll event moves the offset before layout clamps it to the content, so an
            // overscroll past the last page would otherwise render no pages.
            let max_scroll = (layout.content_size.height - viewport.size.height).max(Pixels::ZERO);
            let view_top = (-self.scroll_handle.offset().y)
                .max(Pixels::ZERO)
                .min(max_scroll);
            let view_bottom = view_top + viewport.size.height;
            let visible = layout.pages_in(view_top, view_bottom);
            self.current_page = layout
                .pages_in(
                    view_top + viewport.size.height / 2.0,
                    view_top + viewport.size.height / 2.0 + px(1.0),
                )
                .start
                .min(self.document.page_count() - 1);
            self.update_page_cache(visible.clone(), window);

            div()
                .relative()
                .w(layout.content_size.width)
                .h(layout.content_size.height)
                .children(visible.map(|page| self.render_page(page, layout.page_bounds[page])))
                .into_any_element()
        } else {
            // The viewport size is known after the first layout; fit the pages to it then.
            window.request_animation_frame();
            div().into_any_element()
        };

        div()
            .key_context("PdfViewer")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::zoom_in))
            .on_action(cx.listener(Self::zoom_out))
            .on_action(cx.listener(Self::reset_zoom))
            .on_action(cx.listener(Self::fit_to_view))
            .on_action(cx.listener(Self::zoom_to_actual_size))
            .on_action(cx.listener(Self::reveal_in_file_manager))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::select_all))
            .on_key_down(cx.listener(Self::handle_key_down))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                div()
                    .id("pdf-pages")
                    .size_full()
                    .overflow_scroll()
                    .track_scroll(&self.scroll_handle)
                    .cursor(gpui::CursorStyle::IBeam)
                    .on_scroll(cx.listener(Self::handle_scroll))
                    .on_pinch(cx.listener(Self::handle_pinch))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
                    .on_mouse_move(cx.listener(Self::handle_mouse_move))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                    .child(content)
                    .custom_scrollbars(
                        Scrollbars::new(ui::ScrollAxes::Both)
                            .tracked_scroll_handle(&self.scroll_handle)
                            .id("pdf-scrollbars"),
                        window,
                        cx,
                    ),
            )
    }
}

impl Item for PdfViewer {
    type Event = PdfViewEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        match event {
            PdfViewEvent::TitleChanged => {
                f(workspace::item::ItemEvent::UpdateTab);
                f(workspace::item::ItemEvent::UpdateBreadcrumbs);
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
        let abs_path = self.pdf_item.read(cx).abs_path(cx)?;
        Some(abs_path.compact().to_string_lossy().into_owned().into())
    }

    fn tab_content(&self, params: TabContentParams, _: &Window, cx: &App) -> AnyElement {
        let project_path = self.pdf_item.read(cx).project_path(cx);
        let label_color = if ItemSettings::get_global(cx).git_status {
            let project = self.project.read(cx);
            let git_status = project
                .git_store()
                .read(cx)
                .display_status_for_project_path(&project_path, cx)
                .map(|status| status.summary())
                .unwrap_or_default();
            project
                .entry_for_path(&project_path, cx)
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
        self.pdf_item.read(cx).file_name(cx).into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = self.pdf_item.read(cx).abs_path(cx)?;
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(&path, cx))
            .flatten()
            .map(Icon::from_path)
    }

    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation {
        if EditorSettings::get_global(cx).toolbar.breadcrumbs {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, cx: &App) -> Option<(Vec<HighlightedText>, Option<Font>)> {
        let project = self.project.read(cx);
        let pdf_item = self.pdf_item.read(cx);
        let mut path = pdf_item.file().path.to_rel_path_buf();
        if project.visible_worktrees(cx).count() > 1
            && let Some(worktree) =
                project.worktree_for_id(pdf_item.project_path(cx).worktree_id, cx)
        {
            path = worktree.read(cx).root_name().join(&path);
        }
        let text = path.display(project.path_style(cx)).to_string();
        Some((
            vec![HighlightedText {
                text: text.into(),
                highlights: vec![],
            }],
            Some(ThemeSettings::get_global(cx).buffer_font.clone()),
        ))
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        let pdf_item = self.pdf_item.clone();
        let project = self.project.clone();
        let zoom = self.zoom;
        let fit_width = self.fit_width;
        Task::ready(Some(cx.new(|cx| {
            let mut viewer = Self::new(pdf_item, project, window, cx);
            viewer.zoom = zoom;
            viewer.fit_width = fit_width;
            viewer
        })))
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.pdf_item.read(cx).file().disk_state.is_deleted()
    }

    fn buffer_kind(&self, _: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }
}

impl ProjectItem for PdfViewer {
    type Item = PdfItem;

    fn for_project_item(
        project: Entity<Project>,
        _: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(item, project, window, cx)
    }

    fn for_broken_project_item(
        abs_path: &Path,
        is_local: bool,
        error: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InvalidItemView> {
        Some(InvalidItemView::new(abs_path, is_local, error, window, cx))
    }
}

impl SerializableItem for PdfViewer {
    fn serialized_item_kind() -> &'static str {
        "PdfViewer"
    }

    fn deserialize(
        project: Entity<Project>,
        _: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let db = PdfViewerDb::global(cx);
        window.spawn(cx, async move |cx| {
            let pdf_path = db
                .get_pdf_path(item_id, workspace_id)?
                .context("No PDF path found")?;
            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(pdf_path, false, cx)
                })
                .await
                .context("Path not found")?;
            let project_path = ProjectPath {
                worktree_id: worktree.update(cx, |worktree, _| worktree.id()),
                path: relative_path,
            };
            let pdf_item = cx
                .update(|_, cx| PdfItem::load(&project, &project_path, cx))?
                .await?;
            cx.update(|window, cx| cx.new(|cx| Self::new(pdf_item, project, window, cx)))
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let db = PdfViewerDb::global(cx);
        delete_unloaded_items(alive_items, workspace_id, "pdf_viewers", &db, cx)
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let pdf_path = self.pdf_item.read(cx).abs_path(cx)?;
        let db = PdfViewerDb::global(cx);
        Some(cx.background_spawn(
            async move { db.save_pdf_path(item_id, workspace_id, pdf_path).await },
        ))
    }

    fn should_serialize(&self, _: &Self::Event) -> bool {
        false
    }
}

pub struct PdfViewToolbarControls {
    pdf_viewer: Option<WeakEntity<PdfViewer>>,
    _subscription: Option<Subscription>,
}

impl PdfViewToolbarControls {
    pub fn new() -> Self {
        Self {
            pdf_viewer: None,
            _subscription: None,
        }
    }
}

impl Render for PdfViewToolbarControls {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(pdf_viewer) = self.pdf_viewer.as_ref().and_then(|viewer| viewer.upgrade()) else {
            return div().into_any_element();
        };
        let viewer = pdf_viewer.read(cx);
        let page_label = format!(
            "Page {} of {}",
            viewer.current_page + 1,
            viewer.document.page_count()
        );
        let zoom_label = format!("{}%", (viewer.zoom * 100.0).round() as i32);
        let pdf_viewer = pdf_viewer.downgrade();
        let dispatch = move |action: fn(&mut PdfViewer, &mut Window, &mut Context<PdfViewer>)| {
            let pdf_viewer = pdf_viewer.clone();
            move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                pdf_viewer
                    .update(cx, |viewer, cx| action(viewer, window, cx))
                    .ok();
            }
        };

        h_flex()
            .gap_1()
            .child(
                Label::new(page_label)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                IconButton::new("zoom-out", IconName::Dash)
                    .icon_size(IconSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Zoom Out", &ZoomOut, cx))
                    .on_click(dispatch(|viewer, window, cx| {
                        viewer.zoom_out(&ZoomOut, window, cx)
                    })),
            )
            .child(
                Button::new("zoom-level", zoom_label)
                    .label_size(LabelSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Fit to Width", &FitToView, cx))
                    .on_click(dispatch(|viewer, window, cx| {
                        viewer.fit_to_view(&FitToView, window, cx)
                    })),
            )
            .child(
                IconButton::new("zoom-in", IconName::Plus)
                    .icon_size(IconSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Zoom In", &ZoomIn, cx))
                    .on_click(dispatch(|viewer, window, cx| {
                        viewer.zoom_in(&ZoomIn, window, cx)
                    })),
            )
            .into_any_element()
    }
}

impl EventEmitter<ToolbarItemEvent> for PdfViewToolbarControls {}

impl ToolbarItemView for PdfViewToolbarControls {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.pdf_viewer = None;
        self._subscription = None;

        if let Some(item) = active_pane_item.and_then(|item| item.downcast::<PdfViewer>()) {
            self._subscription = Some(cx.observe(&item, |_, _, cx| cx.notify()));
            self.pdf_viewer = Some(item.downgrade());
            cx.notify();
            return ToolbarItemLocation::PrimaryRight;
        }

        ToolbarItemLocation::Hidden
    }
}

enum WorkerResponse {
    Rendered {
        page: usize,
        scale: f32,
        image: Arc<RenderImage>,
    },
    Text {
        page: usize,
        text: PageText,
    },
    Copied {
        text: String,
    },
    Failed {
        page: usize,
        error: anyhow::Error,
    },
}

#[derive(Default)]
struct WorkQueue {
    renders: VecDeque<(usize, f32)>,
    texts: VecDeque<usize>,
    copy: Option<(TextPosition, TextPosition)>,
    closed: bool,
}

enum Job {
    Render {
        page: usize,
        scale: f32,
    },
    ExtractText {
        page: usize,
    },
    CopyText {
        start: TextPosition,
        end: TextPosition,
    },
}

/// A thread that renders pages and extracts text for one view. Requests replace each other, so
/// pages scrolled past before their turn are never rendered.
struct Worker {
    queue: Arc<(Mutex<WorkQueue>, Condvar)>,
}

impl Worker {
    fn spawn(document: Arc<PdfDocument>) -> (Self, smol::channel::Receiver<WorkerResponse>) {
        let queue = Arc::new((Mutex::new(WorkQueue::default()), Condvar::new()));
        let (responses_tx, responses_rx) = smol::channel::unbounded();
        let thread_queue = queue.clone();
        std::thread::Builder::new()
            .name("pdf-viewer".into())
            .stack_size(WORKER_STACK_SIZE)
            .spawn(move || run_worker(&document, &thread_queue, &responses_tx))
            .context("spawning PDF render thread")
            .log_err();
        (Self { queue }, responses_rx)
    }

    fn update(&self, f: impl FnOnce(&mut WorkQueue)) {
        let (queue, ready) = &*self.queue;
        f(&mut queue.lock());
        ready.notify_one();
    }

    /// Drops queued jobs the view no longer wants and queues new ones.
    fn sync(
        &self,
        wanted_renders: &HashMap<usize, f32>,
        new_renders: Vec<(usize, f32)>,
        kept_pages: Range<usize>,
        new_texts: Vec<usize>,
    ) {
        let (queue, ready) = &*self.queue;
        let mut queue = queue.lock();
        queue
            .renders
            .retain(|(page, scale)| wanted_renders.get(page) == Some(scale));
        queue.texts.retain(|page| kept_pages.contains(page));
        if new_renders.is_empty() && new_texts.is_empty() {
            return;
        }
        queue.renders.extend(new_renders);
        queue.texts.extend(new_texts);
        ready.notify_one();
    }

    fn copy_text(&self, start: TextPosition, end: TextPosition) {
        self.update(|queue| queue.copy = Some((start, end)));
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.update(|queue| queue.closed = true);
    }
}

fn run_worker(
    document: &PdfDocument,
    queue: &(Mutex<WorkQueue>, Condvar),
    responses: &smol::channel::Sender<WorkerResponse>,
) {
    let renderer = PageRenderer::new(document);
    loop {
        let job = {
            let (queue, ready) = queue;
            let mut queue = queue.lock();
            loop {
                if queue.closed {
                    return;
                }
                if let Some((start, end)) = queue.copy.take() {
                    break Job::CopyText { start, end };
                }
                if let Some((page, scale)) = queue.renders.pop_front() {
                    break Job::Render { page, scale };
                }
                if let Some(page) = queue.texts.pop_front() {
                    break Job::ExtractText { page };
                }
                ready.wait(&mut queue);
            }
        };

        let response = match job {
            Job::Render { page, scale } => match render_page_image(&renderer, page, scale) {
                Ok(image) => WorkerResponse::Rendered { page, scale, image },
                Err(error) => WorkerResponse::Failed { page, error },
            },
            Job::ExtractText { page } => match document.extract_text(page) {
                Ok(text) => WorkerResponse::Text { page, text },
                Err(error) => {
                    log::error!("Failed to extract text of PDF page {}: {error:#}", page + 1);
                    WorkerResponse::Text {
                        page,
                        text: PageText::default(),
                    }
                }
            },
            Job::CopyText { start, end } => {
                let mut text = String::new();
                for page in start.page..=end.page.min(document.page_count() - 1) {
                    let page_text = document.extract_text(page).log_err().unwrap_or_default();
                    let range_start = if page == start.page { start.caret } else { 0 };
                    let range_end = if page == end.page {
                        end.caret
                    } else {
                        usize::MAX
                    };
                    if page > start.page {
                        text.push('\n');
                    }
                    text.push_str(&page_text.text_in_range(range_start..range_end));
                }
                WorkerResponse::Copied { text }
            }
        };
        if responses.send_blocking(response).is_err() {
            return;
        }
    }
}

fn render_page_image(renderer: &PageRenderer, page: usize, scale: f32) -> Result<Arc<RenderImage>> {
    let pixels = renderer.render(page, scale)?;
    let buffer = image::RgbaImage::from_raw(pixels.width, pixels.height, pixels.bgra)
        .context("rendered page has an unexpected size")?;
    Ok(Arc::new(RenderImage::new(SmallVec::from_elem(
        image::Frame::new(buffer),
        1,
    ))))
}

pub fn init(cx: &mut App) {
    workspace::register_project_item::<PdfViewer>(cx);
    workspace::register_serializable_item::<PdfViewer>(cx);
}

mod persistence {
    use std::path::PathBuf;

    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct PdfViewerDb(ThreadSafeConnection);

    impl Domain for PdfViewerDb {
        const NAME: &str = stringify!(PdfViewerDb);

        const MIGRATIONS: &[&str] = &[sql!(
                CREATE TABLE pdf_viewers (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    pdf_path BLOB,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
        )];
    }

    db::static_connection!(PdfViewerDb, [WorkspaceDb]);

    impl PdfViewerDb {
        query! {
            pub async fn save_pdf_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                pdf_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO pdf_viewers(item_id, workspace_id, pdf_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_pdf_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT pdf_path
                FROM pdf_viewers
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fs::{FakeFs, Fs as _};
    use gpui::{TestAppContext, VisualTestContext};
    use settings::SettingsStore;
    use util::{path, rel_path::rel_path};
    use workspace::item::ItemHandle;

    use super::*;
    use crate::pdf_document::tests::build_pdf;

    fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.set_global(db::AppDatabase::test_new());
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            init(cx);
        });
        FakeFs::new(cx.executor())
    }

    async fn open_in_workspace<'a>(
        fs: Arc<FakeFs>,
        file_name: &str,
        cx: &'a mut TestAppContext,
    ) -> (Result<Box<dyn ItemHandle>>, &'a mut VisualTestContext) {
        let project = Project::test(fs, [Path::new(path!("/root"))], cx).await;
        let worktree_id = cx.update(|cx| {
            project
                .read(cx)
                .worktrees(cx)
                .next()
                .expect("test project should contain a worktree")
                .read(cx)
                .id()
        });
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let item = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path(file_name)), None, true, window, cx)
            })
            .await;
        cx.run_until_parked();
        (item, cx)
    }

    async fn insert_pdf(fs: &FakeFs, file_name: &str, lines: &[(f32, f32, &str)]) {
        fs.create_dir(Path::new(path!("/root")))
            .await
            .expect("test root should be created");
        fs.insert_file(Path::new(path!("/root")).join(file_name), build_pdf(lines))
            .await;
    }

    fn page_text(pdf_viewer: &Entity<PdfViewer>, cx: &VisualTestContext) -> String {
        cx.read(|cx| {
            let text = pdf_viewer
                .read(cx)
                .document
                .extract_text(0)
                .expect("text extraction should succeed");
            text.text_in_range(0..text.len())
        })
    }

    fn draw_window(cx: &mut VisualTestContext) {
        cx.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
        });
    }

    #[gpui::test]
    async fn test_pdf_files_open_in_the_pdf_viewer(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        insert_pdf(&fs, "Doc.PDF", &[(72.0, 720.0, "Hello")]).await;

        let (item, cx) = open_in_workspace(fs, "Doc.PDF", cx).await;
        let pdf_viewer = item
            .expect("PDF should open")
            .downcast::<PdfViewer>()
            .expect("PDF should open in the PDF viewer");

        pdf_viewer.read_with(cx, |pdf_viewer, cx| {
            assert_eq!(pdf_viewer.tab_content_text(0, cx), "Doc.PDF");
            assert_eq!(pdf_viewer.document.page_count(), 1);
        });
    }

    #[gpui::test]
    async fn test_invalid_pdf_shows_the_error(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        fs.create_dir(Path::new(path!("/root")))
            .await
            .expect("test root should be created");
        fs.insert_file(path!("/root/broken.pdf"), b"not a pdf".to_vec())
            .await;

        let (item, _) = open_in_workspace(fs, "broken.pdf", cx).await;
        let invalid_item_view = item
            .expect("the error view should open")
            .downcast::<InvalidItemView>()
            .expect("an invalid PDF should open in the error view");

        cx.read(|cx| {
            assert_eq!(
                invalid_item_view.read(cx).error,
                "The file is not a valid PDF document"
            );
        });
    }

    #[gpui::test]
    async fn test_rebuilt_pdf_is_reloaded(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        insert_pdf(&fs, "doc.pdf", &[(72.0, 720.0, "Draft")]).await;

        let (item, cx) = open_in_workspace(fs.clone(), "doc.pdf", cx).await;
        let pdf_viewer = item
            .expect("PDF should open")
            .downcast::<PdfViewer>()
            .expect("PDF should open in the PDF viewer");
        assert_eq!(page_text(&pdf_viewer, cx), "Draft");

        fs.insert_file(path!("/root/doc.pdf"), build_pdf(&[(72.0, 720.0, "Final")]))
            .await;
        cx.run_until_parked();
        assert_eq!(page_text(&pdf_viewer, cx), "Final");
    }

    #[gpui::test]
    async fn test_visible_page_is_rendered(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        insert_pdf(&fs, "doc.pdf", &[(72.0, 720.0, "Hello")]).await;

        let (item, cx) = open_in_workspace(fs, "doc.pdf", cx).await;
        let pdf_viewer = item
            .expect("PDF should open")
            .downcast::<PdfViewer>()
            .expect("PDF should open in the PDF viewer");

        // Pages render on the viewer's own thread.
        cx.executor().allow_parking();
        let mut rendered = false;
        for _ in 0..500 {
            draw_window(cx);
            rendered = pdf_viewer.read_with(cx, |pdf_viewer, _| {
                pdf_viewer.rendered_pages.contains_key(&0)
            });
            if rendered {
                break;
            }
            cx.executor().timer(Duration::from_millis(10)).await;
        }
        assert!(rendered, "the first page should be rendered");
    }

    #[gpui::test]
    async fn test_select_all_copies_the_document_text(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        insert_pdf(
            &fs,
            "doc.pdf",
            &[(72.0, 720.0, "Hello World"), (72.0, 700.0, "Second line")],
        )
        .await;

        let (item, cx) = open_in_workspace(fs, "doc.pdf", cx).await;
        let pdf_viewer = item
            .expect("PDF should open")
            .downcast::<PdfViewer>()
            .expect("PDF should open in the PDF viewer");

        // Text is extracted on the viewer's own thread.
        cx.executor().allow_parking();
        pdf_viewer.update_in(cx, |pdf_viewer, window, cx| {
            pdf_viewer.select_all(&SelectAll, window, cx);
            pdf_viewer.copy(&Copy, window, cx);
        });
        let mut copied = None;
        for _ in 0..500 {
            copied = cx.read_from_clipboard().and_then(|item| item.text());
            if copied.is_some() {
                break;
            }
            cx.executor().timer(Duration::from_millis(10)).await;
        }
        assert_eq!(copied.as_deref(), Some("Hello World\nSecond line"));
    }
}

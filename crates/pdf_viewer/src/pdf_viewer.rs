mod pdf_item;
mod pdf_renderer;
mod selection;
mod toolbar;

use hayro::vello_cpu::color::palette::css::WHITE;

pub use pdf_item::{PdfItem, is_pdf_file};
pub use toolbar::PdfViewToolbarControls;

use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use file_icons::FileIcons;
use gpui::{
    actions, div, img, point, px, AnyElement, App, Context, CursorStyle, Entity,
    EventEmitter, FocusHandle, Focusable, IntoElement, MouseButton,
    ParentElement, Pixels, Point, RenderImage, Render, ScrollDelta,
    ScrollHandle, ScrollWheelEvent, SharedString, Styled, Task, Window,
};
use project::Project;
use ui::prelude::*;
use ui::WithScrollbar;
use workspace::{
    Pane, ToolbarItemLocation,
    WorkspaceId,
    invalid_item_view::InvalidItemView,
    item::{BreadcrumbText, Item, ProjectItem, TabContentParams},
};

use pdf_renderer::{PageDimensions, PageTextLayout, PdfMetadata};

const PAGE_GAP_PX: f32 = 20.0;
const RENDER_BUFFER_PAGES: usize = 2;
const SCREEN_SCALE_FACTOR: f32 = 2.0;
// Metal texture atlas tiles cannot exceed 16384px in either dimension.
const MAX_GPU_TILE_PX: f32 = 16384.0;

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 20.0;
const ZOOM_STEP: f32 = 1.1;
const SCROLL_LINE_MULTIPLIER: f32 = 20.0;
const SCROLL_ZOOM_SENSITIVITY: f32 = 0.01;

const RENDER_DEBOUNCE: Duration = Duration::from_millis(200);

actions!(
    pdf_viewer,
    [
        /// Zoom in the PDF.
        ZoomIn,
        /// Zoom out the PDF.
        ZoomOut,
        /// Reset zoom to 100%.
        ResetZoom,
        /// Fit the PDF to the view width.
        FitToView,
        /// Zoom to actual size (100%).
        ZoomToActualSize,
        /// Copy all document text to the clipboard.
        CopyDocumentText,
    ]
);

pub enum PdfViewEvent {
    TitleChanged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TextPosition {
    page: usize,
    glyph_index: usize,
}

pub struct PdfViewer {
    pdf_item: Entity<PdfItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    metadata: Option<PdfMetadata>,
    rendered_pages: HashMap<usize, (f32, Arc<RenderImage>)>,
    pages_in_flight: HashSet<usize>,
    cancel_token: Arc<AtomicBool>,
    render_task: Task<()>,
    render_debounce: Task<()>,
    render_error: Option<SharedString>,
    zoom_level: f32,
    render_scale: f32,
    pan_x: Pixels,
    text_layouts: HashMap<usize, PageTextLayout>,
    text_extraction_task: Task<()>,
    selection_start: Option<TextPosition>,
    selection_end: Option<TextPosition>,
    is_selecting: bool,
}

impl PdfViewer {
    pub fn new(
        pdf_item: Entity<PdfItem>,
        project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            pdf_item,
            project,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            metadata: None,
            rendered_pages: HashMap::new(),
            pages_in_flight: HashSet::new(),
            cancel_token: Arc::new(AtomicBool::new(false)),
            render_task: Task::ready(()),
            render_debounce: Task::ready(()),
            render_error: None,
            zoom_level: 1.0,
            render_scale: SCREEN_SCALE_FACTOR,
            pan_x: px(0.0),
            text_layouts: HashMap::new(),
            text_extraction_task: Task::ready(()),
            selection_start: None,
            selection_end: None,
            is_selecting: false,
        };
        this.load_metadata(cx);
        this
    }

    fn load_metadata(&mut self, cx: &mut Context<Self>) {
        let pdf_bytes = self.pdf_item.read(cx).pdf_bytes().clone();

        let background_task =
            cx.background_spawn(async move { pdf_renderer::parse_metadata(&pdf_bytes) });

        self.render_task = cx.spawn(async move |this, cx| {
            let result = background_task.await;
            this.update(cx, |this, cx| match result {
                Ok(metadata) => {
                    log::debug!(
                        "pdf_viewer: loaded metadata — {} pages",
                        metadata.page_count
                    );
                    this.metadata = Some(metadata);
                    this.render_error = None;
                    log::debug!("pdf_viewer: kicking off text extraction");
                    this.extract_all_text(cx);
                    cx.notify();
                }
                Err(error) => {
                    log::error!("pdf_viewer: failed to load metadata: {error:#}");
                    this.render_error = Some(format!("{error:#}").into());
                }
            })
            .ok();
        });
    }

    fn page_count(&self) -> usize {
        self.metadata
            .as_ref()
            .map_or(0, |metadata| metadata.page_count)
    }

    fn page_dimensions(&self) -> &[PageDimensions] {
        self.metadata
            .as_ref()
            .map_or(&[], |metadata| &metadata.page_dimensions)
    }

    fn display_height(dim: &PageDimensions, container_width: f32, zoom: f32) -> f32 {
        if dim.width <= 0.0 {
            return dim.height * zoom;
        }
        let fit_scale = container_width / dim.width;
        dim.height * fit_scale * zoom
    }

    fn total_content_height(&self, container_width: f32) -> f32 {
        let dimensions = self.page_dimensions();
        let pages_height: f32 = dimensions
            .iter()
            .map(|dim| Self::display_height(dim, container_width, self.zoom_level))
            .sum();
        let gaps = dimensions.len().saturating_sub(1) as f32 * PAGE_GAP_PX;
        pages_height + gaps
    }

    /// Clamp horizontal pan so content edges never retreat past the viewport
    /// edges, matching macOS Preview behaviour. When content fits within the
    /// viewport pan is forced to zero (centered). When content overflows, pan
    /// is bounded so neither edge can slide past its corresponding viewport
    /// edge.
    fn clamp_pan(&mut self) {
        let viewport_width: f32 = self.scroll_handle.bounds().size.width.into();
        if viewport_width <= 0.0 {
            return;
        }
        // At any zoom, every page has display_width = viewport_width * zoom
        // (fit-to-view normalizes all pages to viewport_width at zoom 1.0).
        let content_width = viewport_width * self.zoom_level;
        let overflow = (content_width - viewport_width).max(0.0);
        let half_overflow = overflow / 2.0;
        self.pan_x = self.pan_x.clamp(px(-half_overflow), px(half_overflow));
    }

    fn visible_page_range(&self, viewport_height: f32, viewport_width: f32) -> Range<usize> {
        let dimensions = self.page_dimensions();
        if dimensions.is_empty() {
            return 0..0;
        }

        let scroll_y: f32 = self.scroll_handle.offset().y.into();
        let scroll_offset = scroll_y.abs();

        let mut y = 0.0_f32;
        let mut first_visible = None;
        let mut last_visible = 0;

        for (index, dim) in dimensions.iter().enumerate() {
            let page_height = Self::display_height(dim, viewport_width, self.zoom_level);
            let page_top = y;
            let page_bottom = y + page_height;

            if page_bottom > scroll_offset && first_visible.is_none() {
                first_visible = Some(index);
            }
            if page_top < scroll_offset + viewport_height {
                last_visible = index;
            }
            if page_top > scroll_offset + viewport_height {
                break;
            }

            y = page_bottom + PAGE_GAP_PX;
        }

        let first = first_visible.unwrap_or(0);
        let last = (last_visible + 1).min(dimensions.len());
        first..last
    }

    fn buffered_range(&self, visible: Range<usize>) -> Range<usize> {
        let start = visible.start.saturating_sub(RENDER_BUFFER_PAGES);
        let end = (visible.end + RENDER_BUFFER_PAGES).min(self.page_count());
        start..end
    }

    fn schedule_render_after_zoom(&mut self, cx: &mut Context<Self>) {
        self.render_debounce = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(RENDER_DEBOUNCE).await;
            this.update(cx, |this, cx| {
                this.update_render_scale_if_needed();
                this.request_visible_pages(cx);
            })
            .ok();
        });
    }

    fn update_render_scale_if_needed(&mut self) {
        let ideal_scale = self.zoom_level * SCREEN_SCALE_FACTOR;
        // Cap so the largest page dimension never exceeds the Metal atlas limit.
        let max_dim = self
            .page_dimensions()
            .iter()
            .map(|d| d.width.max(d.height))
            .fold(0.0_f32, f32::max);
        let needed_scale = if max_dim > 0.0 {
            ideal_scale.min(MAX_GPU_TILE_PX / max_dim)
        } else {
            ideal_scale
        };
        let scale_too_low = needed_scale > self.render_scale;
        let scale_wasteful = needed_scale < self.render_scale * 0.25;
        if scale_too_low || scale_wasteful {
            log::debug!(
                "pdf_viewer: render scale change {:.1} -> {:.1} (zoom={:.2})",
                self.render_scale,
                needed_scale,
                self.zoom_level
            );
            self.render_scale = needed_scale;
            self.cancel_token.store(true, Ordering::Relaxed);
            self.cancel_token = Arc::new(AtomicBool::new(false));
            self.pages_in_flight.clear();
        }
    }

    fn request_visible_pages(&mut self, cx: &mut Context<Self>) {
        if self.metadata.is_none() {
            return;
        }

        let bounds = self.scroll_handle.bounds();
        let viewport_height = {
            let h: f32 = bounds.size.height.into();
            if h > 0.0 { h } else { 800.0 }
        };
        let viewport_width = {
            let w: f32 = bounds.size.width.into();
            if w > 0.0 { w } else { 600.0 }
        };

        let visible = self.visible_page_range(viewport_height, viewport_width);
        let target_range = self.buffered_range(visible.clone());

        let current_scale = self.render_scale;
        let needs_render = |index: &usize| -> bool {
            !self.pages_in_flight.contains(index)
                && self
                    .rendered_pages
                    .get(index)
                    .map_or(true, |(scale, _)| *scale != current_scale)
        };

        let mut needed: Vec<usize> = visible.clone().filter(&needs_render).collect();
        let buffer_pages: Vec<usize> = target_range
            .filter(|index| !visible.contains(index) && needs_render(index))
            .collect();
        needed.extend(buffer_pages);

        if needed.is_empty() {
            return;
        }

        for &index in &needed {
            self.pages_in_flight.insert(index);
        }

        let needed_display: Vec<usize> = needed.iter().map(|i| i + 1).collect();
        log::debug!(
            "pdf_viewer: rendering pages {:?} at scale {:.1}",
            needed_display,
            self.render_scale
        );

        let pdf_bytes = self.pdf_item.read(cx).pdf_bytes().clone();
        let render_scale = self.render_scale;
        let cancel_token = self.cancel_token.clone();

        let (sender, receiver) = smol::channel::unbounded::<(usize, Arc<RenderImage>)>();

        if let Err(error) = std::thread::Builder::new()
            .name("pdf-renderer".into())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || {
                let pdf = match pdf_renderer::open_pdf(&pdf_bytes) {
                    Ok(pdf) => pdf,
                    Err(error) => {
                        log::error!(
                            "pdf_viewer: failed to open PDF for rendering: {error:#}"
                        );
                        return;
                    }
                };
                for page_index in needed {
                    if cancel_token.load(Ordering::Relaxed) {
                        log::debug!("pdf_viewer: render cancelled");
                        break;
                    }
                    log::debug!("pdf_viewer: rendering page {}...", page_index + 1);
                    match pdf_renderer::render_single_page(
                        &pdf,
                        page_index,
                        render_scale,
                        WHITE,
                    ) {
                        Ok(rendered) => {
                            log::debug!(
                                "pdf_viewer: page {} rendered ({}x{})",
                                page_index + 1,
                                rendered.page_width,
                                rendered.page_height
                            );
                            if sender.send_blocking((page_index, rendered.image)).is_err() {
                                break;
                            }
                        }
                        Err(error) => {
                            log::error!(
                                "pdf_viewer: failed to render page {}: {error:#}",
                                page_index + 1
                            );
                            break;
                        }
                    }
                }
            })
        {
            log::error!("pdf_viewer: failed to spawn render thread: {error:#}");
        }

        self.render_task = cx.spawn(async move |this, cx| {
            while let Ok((page_index, image)) = receiver.recv().await {
                let scale = render_scale;
                this.update(cx, |this, cx| {
                    this.pages_in_flight.remove(&page_index);
                    this.rendered_pages.insert(page_index, (scale, image));
                    cx.notify();
                })
                .ok();
            }
            this.update(cx, |this, _cx| {
                this.pages_in_flight.clear();
            })
            .ok();
        });
    }

    fn render_page_element(
        &self,
        index: usize,
        dimensions: &PageDimensions,
        container_width: f32,
    ) -> AnyElement {
        let fit_scale = if dimensions.width > 0.0 {
            container_width / dimensions.width
        } else {
            1.0
        };
        let scale = fit_scale * self.zoom_level;
        let display_width = px(dimensions.width * scale);
        let display_height = px(dimensions.height * scale);

        let page_content = if let Some((_scale, image)) = self.rendered_pages.get(&index) {
            img(image.clone())
                .id(("pdf-page", index))
                .w(display_width)
                .h(display_height)
                .into_any_element()
        } else {
            div()
                .id(("pdf-page-placeholder", index))
                .w(display_width)
                .h(display_height)
                .bg(gpui::rgb(0xf0f0f0))
                .border_1()
                .border_color(gpui::rgb(0xe0e0e0))
                .flex()
                .justify_center()
                .items_center()
                .child(Label::new(format!("Page {}", index + 1)).color(Color::Muted))
                .into_any_element()
        };

        let highlights = self.selection_highlights_for_page(index, fit_scale);

        div()
            .relative()
            .w(display_width)
            .h(display_height)
            .child(page_content)
            .children(highlights)
            .into_any_element()
    }

    // -- Zoom --

    fn set_zoom(
        &mut self,
        new_zoom: f32,
        zoom_center: Option<Point<Pixels>>,
        cx: &mut Context<Self>,
    ) {
        let old_zoom = self.zoom_level;
        self.zoom_level = new_zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        if (self.zoom_level - old_zoom).abs() > f32::EPSILON {
            if old_zoom > 0.0 {
                let ratio = self.zoom_level / old_zoom;
                self.pan_x *= ratio;
            }
            self.clamp_pan();

            if let Some(cursor) = zoom_center {
                let bounds = self.scroll_handle.bounds();
                let offset = self.scroll_handle.offset();

                let cursor_in_container = point(
                    cursor.x - bounds.origin.x,
                    cursor.y - bounds.origin.y,
                );

                let content_x = cursor_in_container.x - offset.x;
                let content_y = cursor_in_container.y - offset.y;

                let zoom_ratio = self.zoom_level / old_zoom;
                let new_content_x = content_x * zoom_ratio;
                let new_content_y = content_y * zoom_ratio;

                let new_offset = point(
                    cursor_in_container.x - new_content_x,
                    cursor_in_container.y - new_content_y,
                );
                self.scroll_handle.set_offset(new_offset);
            }

            cx.notify();
            self.schedule_render_after_zoom(cx);
        }
    }

    fn zoom_in(&mut self, _: &ZoomIn, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom_level * ZOOM_STEP, None, cx);
    }

    fn zoom_out(&mut self, _: &ZoomOut, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom_level / ZOOM_STEP, None, cx);
    }

    fn reset_zoom(&mut self, _: &ResetZoom, _window: &mut Window, cx: &mut Context<Self>) {
        self.pan_x = px(0.0);
        self.set_zoom(1.0, None, cx);
    }

    fn fit_to_view(&mut self, _: &FitToView, _window: &mut Window, cx: &mut Context<Self>) {
        self.pan_x = px(0.0);
        self.set_zoom(1.0, None, cx);
    }

    fn zoom_to_actual_size(
        &mut self,
        _: &ZoomToActualSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_zoom(1.0, None, cx);
    }

    // Handle horizontal scroll manually here because GPUI's ScrollHandler
    // only deals with vertical scrolling — horizontal delta from the
    // trackpad / scroll wheel is not forwarded through the scroll container.
    fn handle_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.modifiers.control || event.modifiers.platform {
            let delta: f32 = match event.delta {
                ScrollDelta::Pixels(pixels) => pixels.y.into(),
                ScrollDelta::Lines(lines) => lines.y * SCROLL_LINE_MULTIPLIER,
            };
            let zoom_factor = if delta > 0.0 {
                1.0 + delta.abs() * SCROLL_ZOOM_SENSITIVITY
            } else {
                1.0 / (1.0 + delta.abs() * SCROLL_ZOOM_SENSITIVITY)
            };
            self.set_zoom(self.zoom_level * zoom_factor, Some(event.position), cx);
        } else {
            let delta_x = match event.delta {
                ScrollDelta::Pixels(pixels) => pixels.x,
                ScrollDelta::Lines(lines) => px(lines.x * SCROLL_LINE_MULTIPLIER),
            };
            if delta_x != px(0.0) {
                self.pan_x += delta_x;
                self.clamp_pan();
            }
            self.request_visible_pages(cx);
            cx.notify();
        }
    }
}

impl EventEmitter<PdfViewEvent> for PdfViewer {}
impl EventEmitter<()> for PdfViewer {}

impl Focusable for PdfViewer {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PdfViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.request_visible_pages(cx);

        let content = if let Some(error) = &self.render_error {
            v_flex()
                .p_4()
                .gap_2()
                .child(Label::new("Failed to render PDF").color(Color::Error))
                .child(Label::new(error.clone()).size(LabelSize::Small))
                .into_any_element()
        } else if self.metadata.is_none() {
            v_flex()
                .p_4()
                .child(Label::new("Loading..."))
                .into_any_element()
        } else {
            let dimensions = self.page_dimensions().to_vec();
            let bounds = self.scroll_handle.bounds();
            let container_width: f32 = bounds.size.width.into();
            let container_width = if container_width > 0.0 {
                container_width
            } else {
                dimensions
                    .iter()
                    .map(|d| d.width)
                    .fold(0.0_f32, f32::max)
            };

            // When all pages fit inside the viewport, centre them vertically
            // with equal padding so they float in the middle. When content
            // exceeds the viewport, no padding is added and GPUI's scroll
            // container naturally bounds scrolling to the content height —
            // matching macOS Preview's behaviour.
            let total_content_h = self.total_content_height(container_width);
            let viewport_h: f32 = {
                let h: f32 = self.scroll_handle.bounds().size.height.into();
                if h > 0.0 { h } else { 800.0 }
            };

            let centering_pad = if total_content_h < viewport_h {
                (viewport_h - total_content_h) / 2.0
            } else {
                0.0
            };

            let mut pages_column = v_flex().gap(px(PAGE_GAP_PX)).items_center();
            if centering_pad > 0.0 {
                pages_column = pages_column.child(div().h(px(centering_pad)));
            }
            for (index, dim) in dimensions.iter().enumerate() {
                pages_column =
                    pages_column.child(self.render_page_element(index, dim, container_width));
            }
            if centering_pad > 0.0 {
                pages_column = pages_column.child(div().h(px(centering_pad)));
            }

            if self.pan_x != px(0.0) {
                div()
                    .relative()
                    .left(self.pan_x)
                    .child(pages_column)
                    .into_any_element()
            } else {
                pages_column.into_any_element()
            }
        };

        let bounds = self.scroll_handle.bounds();
        let vp_h: f32 = bounds.size.height.into();
        let vp_w: f32 = bounds.size.width.into();
        let vp_h = if vp_h > 0.0 { vp_h } else { 800.0 };
        let vp_w = if vp_w > 0.0 { vp_w } else { 600.0 };

        let zoom_percent = (self.zoom_level * 100.0).round() as u32;
        let page_info = if self.page_count() > 0 {
            let visible = self.visible_page_range(vp_h, vp_w);
            let first = visible.start + 1;
            let last = if visible.end > 0 { visible.end } else { 1 };
            if first == last || visible.end - visible.start <= 1 {
                format!(
                    "Page {} of {}  ·  {}%",
                    first,
                    self.page_count(),
                    zoom_percent,
                )
            } else {
                format!(
                    "Pages {}-{} of {}  ·  {}%",
                    first,
                    last,
                    self.page_count(),
                    zoom_percent,
                )
            }
        } else {
            "Loading...".to_string()
        };

        v_flex()
            .id("PdfViewer")
            .key_context("PdfViewer")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::zoom_in))
            .on_action(cx.listener(Self::zoom_out))
            .on_action(cx.listener(Self::reset_zoom))
            .on_action(cx.listener(Self::fit_to_view))
            .on_action(cx.listener(Self::zoom_to_actual_size))
            .on_action(cx.listener(Self::copy_document_text))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .px_3()
                    .py_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new(page_info).size(LabelSize::Small)),
            )
            .child(
                div()
                    .id("pdf-scroll-container")
                    .flex_1()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .on_scroll_wheel(cx.listener(Self::handle_scroll_wheel))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(Self::handle_mouse_down),
                    )
                    .on_mouse_move(cx.listener(Self::handle_mouse_move))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(Self::handle_mouse_up),
                    )
                    .cursor(CursorStyle::IBeam)
                    .child(content)
                    .vertical_scrollbar_for(&self.scroll_handle, window, cx),
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

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.pdf_item.read(cx).file_name().to_string().into()
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .single_line()
            .color(params.text_color())
            .into_any_element()
    }

    fn tab_icon(&self, _window: &Window, cx: &App) -> Option<Icon> {
        let path = self.pdf_item.read(cx).abs_path();
        FileIcons::get_icon(path, cx).map(Icon::from_path)
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        Some(
            self.pdf_item
                .read(cx)
                .abs_path()
                .display()
                .to_string()
                .into(),
        )
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        f(self.pdf_item.entity_id(), self.pdf_item.read(cx))
    }

    fn breadcrumb_location(&self, _cx: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, cx: &App) -> Option<Vec<BreadcrumbText>> {
        let text = self.pdf_item.read(cx).file_name().to_string();
        Some(vec![BreadcrumbText {
            text,
            highlights: None,
            font: None,
        }])
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| Self {
            pdf_item: self.pdf_item.clone(),
            project: self.project.clone(),
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            metadata: self.metadata.clone(),
            rendered_pages: self.rendered_pages.clone(),
            pages_in_flight: HashSet::new(),
            cancel_token: Arc::new(AtomicBool::new(false)),
            render_task: Task::ready(()),
            render_debounce: Task::ready(()),
            render_error: self.render_error.clone(),
            zoom_level: self.zoom_level,
            render_scale: self.render_scale,
            pan_x: self.pan_x,
            text_layouts: self.text_layouts.clone(),
            text_extraction_task: Task::ready(()),
            selection_start: None,
            selection_end: None,
            is_selecting: false,
        })))
    }

    fn buffer_kind(&self, _cx: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }
}

impl ProjectItem for PdfViewer {
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
        error: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InvalidItemView>
    where
        Self: Sized,
    {
        Some(InvalidItemView::new(abs_path, is_local, error, window, cx))
    }
}

pub fn init(cx: &mut App) {
    workspace::register_project_item::<PdfViewer>(cx);
}
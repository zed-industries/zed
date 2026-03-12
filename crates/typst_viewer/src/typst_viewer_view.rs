use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use anyhow::Context as _;
use async_tungstenite::tungstenite::Message;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{
    App, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render,
    RenderImage, ScrollHandle, SharedString, Task, Window,
};
use image::Frame;
use language::Buffer;
use multi_buffer::MultiBuffer;
use project::Project;
use settings::Settings as _;
use smallvec::SmallVec;
use ui::prelude::*;
use ui::WithScrollbar;
use workspace::item::Item;
use workspace::{Pane, Workspace};

use crate::svg_stream;
use crate::{OpenPreview, OpenPreviewToTheSide};



enum PreviewState {
    Connecting,
    Rendering { pages: Vec<Option<Arc<RenderImage>>> },
    Disconnected { reason: String },
    Error { message: String },
}

pub struct TypstPreviewView {
    source_buffer: Option<Entity<Buffer>>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    state: PreviewState,
    _connection_task: Task<()>,
    _lsp_subscriptions: Vec<lsp::Subscription>,
}

impl TypstPreviewView {
    pub fn new(
        active_buffer: Entity<MultiBuffer>,
        project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let source_buffer = active_buffer.read_with(cx, |buffer, _cx| buffer.as_singleton());
            let focus_handle = cx.focus_handle();

            let mut this = Self {
                source_buffer,
                project: project.clone(),
                focus_handle,
                scroll_handle: ScrollHandle::new(),
                state: PreviewState::Connecting,
                _connection_task: Task::ready(()),
                _lsp_subscriptions: Vec::new(),
            };

            this.start_connection(cx);
            this
        })
    }

    fn start_connection(&mut self, cx: &mut Context<Self>) {
        let project = self.project.clone();
        let source_buffer = self.source_buffer.clone();

        self._connection_task = cx.spawn(async move |this, cx| {
            if let Err(err) = Self::connect_and_receive(project, source_buffer, &this, cx).await {
                log::error!("typst_viewer: connection failed: {err:#}");
                this.update(cx, |this, cx| {
                    this.state = PreviewState::Error {
                        message: format!("{err:#}"),
                    };
                    cx.notify();
                })
                .ok();
            }
        });
    }

    /// Try to get a WebSocket URL by asking the tinymist LSP to start a
    /// preview. Returns `None` if tinymist isn't available.
    /// Also returns any LSP subscriptions that must be kept alive.
    async fn request_preview_url(
        project: &Entity<Project>,
        source_buffer: &Option<Entity<Buffer>>,
        cx: &mut gpui::AsyncApp,
    ) -> anyhow::Result<(String, Vec<lsp::Subscription>)> {
        let (server, request_timeout, entry_path) = project
            .read_with(cx, |project, cx| {
                let buffer = source_buffer.as_ref().map(|b| b.read(cx));
                let server_id =
                    crate::find_tinymist_server(project, buffer.as_deref(), cx)
                        .context("tinymist language server not found")?;
                let server = project
                    .lsp_store()
                    .read(cx)
                    .language_server_for_id(server_id)
                    .context("tinymist server not running")?;
                let request_timeout = project::project_settings::ProjectSettings::get_global(cx)
                    .global_lsp_settings
                    .get_request_timeout();
                let entry_path = source_buffer
                    .as_ref()
                    .and_then(|b| b.read(cx).file())
                    .and_then(|file| file.as_local())
                    .map(|file| file.abs_path(cx))
                    .context("buffer has no file path")?;
                anyhow::Ok((server, request_timeout, entry_path))
            })?;

        // Suppress "unhandled notification" log spam from tinymist.
        // Returns None if already registered for this server (safe on re-open).
        let subscription = crate::register_tinymist_notifications(&server);

        let url = crate::start_preview_via_lsp(server, &entry_path, request_timeout).await?;
        log::info!("typst_viewer: LSP provided WebSocket URL: {url}");
        let subscriptions: Vec<_> = subscription.into_iter().collect();
        Ok((url, subscriptions))
    }

    async fn connect_and_receive(
        project: Entity<Project>,
        source_buffer: Option<Entity<Buffer>>,
        this: &gpui::WeakEntity<Self>,
        cx: &mut gpui::AsyncApp,
    ) -> anyhow::Result<()> {
        // tinymist may dispose the preview immediately after starting it
        // (e.g. if a workspace/didChangeConfiguration triggers a project
        // reload).  Retry a few times with a delay to let it settle.
        let max_attempts = 3;
        for attempt in 0..max_attempts {
            match Self::try_connect_and_receive(&project, &source_buffer, this, cx).await {
                Ok(()) => return Ok(()),
                Err(err) => {
                    if attempt + 1 < max_attempts {
                        log::warn!(
                            "typst_viewer: attempt {}/{max_attempts} failed: {err:#}, retrying in 1s",
                            attempt + 1,
                        );
                        smol::Timer::after(std::time::Duration::from_secs(1)).await;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
        unreachable!()
    }

    async fn try_connect_and_receive(
        project: &Entity<Project>,
        source_buffer: &Option<Entity<Buffer>>,
        this: &gpui::WeakEntity<Self>,
        cx: &mut gpui::AsyncApp,
    ) -> anyhow::Result<()> {
        let (url, lsp_subscriptions) = Self::request_preview_url(project, source_buffer, cx)
            .await?;

        if !lsp_subscriptions.is_empty() {
            this.update(cx, |this, _cx| {
                this._lsp_subscriptions.extend(lsp_subscriptions);
            }).ok();
        }

        log::info!("typst_viewer: connecting to LSP-provided preview at {url}");
        let mut ws = svg_stream::connect(&url).await
            .with_context(|| format!("failed to connect to preview server at {url}"))?;

        // tinymist expects the client to send "current" to trigger a full render.
        log::info!("typst_viewer: sending 'current' to request initial render");
        ws.send(Message::text("current")).await?;

        Self::receive_loop(&mut ws, this, cx).await
    }



    /// Parse a single WebSocket text message into page metadata + SVG bytes,
    /// or return None for non-SVG messages (which are logged and skipped).
    fn parse_svg_message(text: &str) -> Option<(usize, usize, Vec<u8>)> {
        let (page_index, page_total, svg_text) =
            if let Some((header, svg)) = parse_page_header(text) {
                (header.index, header.total, svg)
            } else if text.contains("<svg") {
                (0, 1, text)
            } else {
                let preview = &text[..text.len().min(120)];
                log::info!(
                    "typst_viewer: received text message ({} bytes): {preview}",
                    text.len()
                );
                return None;
            };

        if !svg_text.contains("<svg") {
            log::warn!(
                "typst_viewer: page {page_index}/{page_total} has no <svg tag, skipping"
            );
            return None;
        }

        Some((page_index, page_total, svg_text.as_bytes().to_vec()))
    }

    async fn receive_loop(
        ws: &mut svg_stream::PreviewSocket,
        this: &gpui::WeakEntity<Self>,
        cx: &mut gpui::AsyncApp,
    ) -> anyhow::Result<()> {
        let mut cached_glyph_defs: HashMap<usize, String> = HashMap::new();

        while let Some(msg_result) = ws.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    // --- Phase 1: Parse the triggering message ---
                    let Some((page_index, page_total, svg_bytes)) =
                        Self::parse_svg_message(&text)
                    else {
                        continue;
                    };

                    // Collect the latest SVG for each page.  Start with
                    // the message we just received, then drain everything
                    // queued behind it so we only rasterize the freshest
                    // compile output.
                    let mut latest: BTreeMap<usize, Vec<u8>> = BTreeMap::new();
                    let mut latest_total = page_total;
                    latest.insert(page_index, svg_bytes);
                    let mut skipped = 0u64;

                    loop {
                        match ws.next().now_or_never() {
                            Some(Some(Ok(Message::Text(newer)))) => {
                                if let Some((pi, pt, bytes)) =
                                    Self::parse_svg_message(&newer)
                                {
                                    // If this starts a newer batch (page 0
                                    // with a possibly different total), clear
                                    // stale pages from the previous batch.
                                    if pi == 0 && pt != latest_total {
                                        latest.clear();
                                        latest_total = pt;
                                    }
                                    if latest.insert(pi, bytes).is_some() {
                                        skipped += 1;
                                    }
                                }
                            }
                            // Non-text or no more queued — stop draining.
                            _ => break,
                        }
                    }
                    if skipped > 0 {
                        log::info!(
                            "typst_viewer: frame drop — kept {} pages, \
                             skipped {skipped} stale SVGs",
                            latest.len(),
                        );
                    }

                    // --- Phase 2: Rasterize pages, visible first ---
                    // Read the current scroll offset to prioritise the page
                    // the user is actually looking at.
                    let visible_page = this.update(cx, |this, _cx| {
                        let scroll_y: f32 = this.scroll_handle.offset().y.abs().into();
                        // Each page is roughly the same height.  Estimate
                        // which page index is at the current scroll position.
                        let page_count = match &this.state {
                            PreviewState::Rendering { pages } => pages.len().max(1),
                            _ => latest_total,
                        };
                        // Use the first rendered page to get the height,
                        // or fall back to a reasonable default.
                        let page_height = match &this.state {
                            PreviewState::Rendering { pages } => {
                                pages.iter().find_map(|p| {
                                    let img = p.as_ref()?;
                                    let h = img.size(0).height.0 as f32 / 2.0;
                                    Some(h + 12.0) // display_h + page_gap
                                }).unwrap_or(1200.0)
                            }
                            _ => 1200.0,
                        };
                        let idx = (scroll_y / page_height) as usize;
                        idx.min(page_count.saturating_sub(1))
                    }).unwrap_or(0);

                    // Sort page indices: visible page first, then nearest
                    // neighbours expanding outward, then the rest.
                    let mut page_order: Vec<usize> = latest.keys().copied().collect();
                    page_order.sort_by_key(|&idx| {
                        let dist = (idx as isize - visible_page as isize).unsigned_abs();
                        dist
                    });

                    for &page_index in &page_order {
                        let Some(svg_bytes) = latest.get(&page_index) else { continue };
                        let mut svg_bytes = svg_bytes.clone();

                        // Glyph defs caching: first frame has full defs,
                        // subsequent frames may have them stripped by the
                        // server.  Inject cached defs when missing so
                        // usvg can resolve all <use> references.
                        let has_defs = std::str::from_utf8(&svg_bytes)
                            .map(|s| s.contains(GLYPH_DEFS_OPEN))
                            .unwrap_or(false);

                        if has_defs {
                            // Cache this page's defs for future frames
                            // where the server strips them.
                            let svg_str = String::from_utf8_lossy(&svg_bytes);
                            if let Some(start) = svg_str.find(GLYPH_DEFS_OPEN) {
                                if let Some(end_offset) = svg_str[start..].find(DEFS_CLOSE) {
                                    let defs_end = start + end_offset + DEFS_CLOSE.len();
                                    let defs = &svg_str[start..defs_end];
                                    cached_glyph_defs.insert(page_index, defs.to_string());
                                }
                            }
                        } else if let Some(defs) = cached_glyph_defs.get(&page_index) {
                            svg_bytes = inject_glyph_defs(&svg_bytes, defs);
                        } else {
                            log::warn!(
                                "typst_viewer: page {page_index} — no defs and no cache"
                            );
                        }

                        let raster_start = std::time::Instant::now();
                        let image_result = cx
                            .background_executor()
                            .spawn(async move {
                                rasterize_svg_to_image(&svg_bytes, 2.0)
                            })
                            .await;

                        let elapsed_ms = raster_start.elapsed().as_secs_f64() * 1000.0;

                        match image_result {
                            Ok(image) => {
                                log::info!(
                                    "typst_viewer: page {page_index}/{latest_total} \
                                     rasterized in {elapsed_ms:.0}ms"
                                );
                                this.update(cx, |this, cx| {
                                    let pages = match &mut this.state {
                                        PreviewState::Rendering { pages } => pages,
                                        _ => {
                                            this.state = PreviewState::Rendering {
                                                pages: vec![None; latest_total],
                                            };
                                            match &mut this.state {
                                                PreviewState::Rendering { pages } => pages,
                                                _ => unreachable!(),
                                            }
                                        }
                                    };
                                    pages.resize(latest_total, None);
                                    if page_index < pages.len() {
                                        pages[page_index] = Some(image);
                                    }
                                    cx.notify();
                                })?;
                            }
                            Err(err) => {
                                log::error!(
                                    "typst_viewer: rasterization failed in \
                                     {elapsed_ms:.0}ms: {err}"
                                );
                            }
                        }
                    }
                }
                Ok(Message::Binary(data)) => {
                    let magic = if data.len() >= 4 {
                        format!(
                            "{:02x} {:02x} {:02x} {:02x}",
                            data[0], data[1], data[2], data[3]
                        )
                    } else {
                        format!("{} bytes", data.len())
                    };
                    log::debug!(
                        "typst_viewer: received binary message ({} bytes, magic: {magic}), skipping",
                        data.len()
                    );
                }
                Ok(Message::Close(frame)) => {
                    log::info!("typst_viewer: server closed WebSocket: {frame:?}");
                    this.update(cx, |this, cx| {
                        this.state = PreviewState::Disconnected {
                            reason: "Server closed connection".into(),
                        };
                        cx.notify();
                    })?;
                    break;
                }
                Ok(Message::Ping(_)) => {
                    log::debug!("typst_viewer: received ping");
                }
                Ok(other) => {
                    log::debug!("typst_viewer: received other message: {other:?}");
                }
                Err(err) => {
                    log::error!("typst_viewer: WebSocket receive error: {err}");
                    this.update(cx, |this, cx| {
                        this.state = PreviewState::Disconnected {
                            reason: format!("WebSocket error: {err}"),
                        };
                        cx.notify();
                    })?;
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn resolve_active_item_as_typst_buffer(
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<MultiBuffer>> {
        workspace
            .active_item(cx)?
            .act_as::<MultiBuffer>(cx)
            .filter(|buffer| Self::is_typst_file(buffer, cx))
    }

    pub fn is_typst_file(buffer: &Entity<MultiBuffer>, cx: &App) -> bool {
        buffer
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
            .is_some_and(|file| {
                std::path::Path::new(file.file_name(cx))
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("typ"))
            })
    }

    fn find_existing_preview_item_idx(
        pane: &Pane,
        buffer: &Entity<MultiBuffer>,
        cx: &App,
    ) -> Option<usize> {
        let buffer_id = buffer.read(cx).as_singleton()?.entity_id();
        pane.items_of_type::<TypstPreviewView>()
            .find(|view| {
                view.read(cx)
                    .source_buffer
                    .as_ref()
                    .is_some_and(|buffer| buffer.entity_id() == buffer_id)
            })
            .and_then(|view| pane.index_for_item(&view))
    }

    pub fn register(
        workspace: &mut Workspace,
        _window: &mut Window,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(buffer) = Self::resolve_active_item_as_typst_buffer(workspace, cx)
                && Self::is_typst_file(&buffer, cx)
            {
                let project = workspace.project().clone();
                let view = TypstPreviewView::new(buffer.clone(), project, window, cx);
                workspace.active_pane().update(cx, |pane, cx| {
                    if let Some(existing_idx) =
                        Self::find_existing_preview_item_idx(pane, &buffer, cx)
                    {
                        pane.activate_item(existing_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view), true, true, None, window, cx);
                    }
                });
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenPreviewToTheSide, window, cx| {
            if let Some(buffer) = Self::resolve_active_item_as_typst_buffer(workspace, cx)
                && Self::is_typst_file(&buffer, cx)
            {
                let project = workspace.project().clone();
                let view = TypstPreviewView::new(buffer.clone(), project, window, cx);
                let pane = workspace
                    .find_pane_in_direction(workspace::SplitDirection::Right, cx)
                    .unwrap_or_else(|| {
                        workspace.split_pane(
                            workspace.active_pane().clone(),
                            workspace::SplitDirection::Right,
                            window,
                            cx,
                        )
                    });
                pane.update(cx, |pane, cx| {
                    if let Some(existing_idx) =
                        Self::find_existing_preview_item_idx(pane, &buffer, cx)
                    {
                        pane.activate_item(existing_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view), false, false, None, window, cx);
                    }
                });
                cx.notify();
            }
        });
    }
}

impl Render for TypstPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match &self.state {
            PreviewState::Connecting => div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(SharedString::from("Connecting to preview server…"))
                .into_any_element(),

            PreviewState::Rendering { pages } => {
                log::debug!("typst_viewer: [render] {} pages", pages.len());

                let page_gap = 12.0_f32;

                let mut pages_column = div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .pt(gpui::px(page_gap));

                for (i, page_opt) in pages.iter().enumerate() {
                    match page_opt {
                        Some(image) => {
                            let image_size = image.size(0);
                            let image_w = image_size.width.0 as f32;
                            let image_h = image_size.height.0 as f32;
                            let aspect = if image_w > 0.0 { image_h / image_w } else { 1.0 };
                            let display_w = image_w / 2.0;
                            let display_h = image_w / 2.0 * aspect;

                            pages_column = pages_column.child(
                                div()
                                    .pb(gpui::px(page_gap))
                                    .child(
                                        gpui::img(gpui::ImageSource::Render(image.clone()))
                                            .id(ElementId::Integer(image.id.0 as u64))
                                            .debug_selector(|| "TYPST_PREVIEW_IMG".into())
                                            .w(gpui::px(display_w))
                                            .h(gpui::px(display_h)),
                                    )
                            );
                        }
                        None => {
                            pages_column = pages_column.child(
                                div()
                                    .pb(gpui::px(page_gap))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .h(gpui::px(200.0))
                                    .child(SharedString::from(format!("Loading page {}…", i + 1)))
                            );
                        }
                    }
                }

                div()
                    .id("typst-viewer-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(pages_column)
                    .vertical_scrollbar_for(&self.scroll_handle, window, cx)
                    .into_any_element()
            }

            PreviewState::Disconnected { reason } => div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(SharedString::from(format!("Disconnected: {reason}")))
                .into_any_element(),

            PreviewState::Error { message } => div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(SharedString::from(format!("Error: {message}")))
                .into_any_element(),
        };

        div()
            .key_context("TypstPreview")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .flex()
            .flex_col()
            .bg(gpui::rgb(0xf0f0f0))
            .child(content)
    }
}

impl Focusable for TypstPreviewView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for TypstPreviewView {}

impl Item for TypstPreviewView {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.source_buffer
            .as_ref()
            .and_then(|buffer| buffer.read(cx).file())
            .map(|file| format!("Preview {}", file.file_name(cx)).into())
            .unwrap_or_else(|| "Typst Preview".into())
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Eye))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("typst preview: open")
    }

    fn to_item_events(_event: &Self::Event, _f: &mut dyn FnMut(workspace::item::ItemEvent)) {}
}

// =========================================================================
// Live-path helpers: glyph defs caching, page header parsing
// =========================================================================

pub(crate) const GLYPH_DEFS_OPEN: &str = r#"<defs id="glyph">"#;
pub(crate) const DEFS_CLOSE: &str = "</defs>";

pub(crate) struct PageHeader {
    pub(crate) index: usize,
    pub(crate) total: usize,
}

/// Parse a `page:{index}:{total}\n` prefix from a server message.
pub(crate) fn parse_page_header(text: &str) -> Option<(PageHeader, &str)> {
    let rest = text.strip_prefix("page:")?;
    let newline_pos = rest.find('\n')?;
    let header_str = &rest[..newline_pos];
    let svg = &rest[newline_pos + 1..];
    let (index_str, total_str) = header_str.split_once(':')?;
    let index: usize = index_str.parse().ok()?;
    let total: usize = total_str.parse().ok()?;
    Some((PageHeader { index, total }, svg))
}

/// Insert cached glyph defs into an SVG that had them stripped by the server.
/// Inserts right after the opening `<svg ...>` tag.
pub(crate) fn inject_glyph_defs(svg_bytes: &[u8], cached_defs: &str) -> Vec<u8> {
    let svg_str = String::from_utf8_lossy(svg_bytes);
    let s: &str = &svg_str;
    if let Some(close_bracket) = s.find('>') {
        let insert_pos = close_bracket + 1;
        let before = &s[..insert_pos];
        let after = &s[insert_pos..];
        let mut result = Vec::with_capacity(svg_bytes.len() + cached_defs.len());
        result.extend_from_slice(before.as_bytes());
        result.extend_from_slice(cached_defs.as_bytes());
        result.extend_from_slice(after.as_bytes());
        result
    } else {
        svg_bytes.to_vec()
    }
}





pub(crate) fn rasterize_svg_to_image(svg_bytes: &[u8], scale: f32) -> anyhow::Result<Arc<RenderImage>> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_bytes, &options)?;

    let size = tree.size();
    log::info!(
        "typst_viewer: full rasterize SVG size: {}x{} ({}x{} px at {scale}x)",
        size.width(), size.height(),
        (size.width() * scale).ceil() as u32,
        (size.height() * scale).ceil() as u32,
    );
    let width = (size.width() * scale).ceil() as u32;
    let height = (size.height() * scale).ceil() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| anyhow::anyhow!("failed to create {width}x{height} pixmap"))?;

    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    let mut buffer = image::ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
        .ok_or_else(|| anyhow::anyhow!("pixmap data didn't match expected buffer size"))?;

    // GPUI expects BGRA pixel format (Metal textures use BGRA8Unorm).
    // tiny-skia produces premultiplied RGBA. Swap R↔B channels.
    for pixel in buffer.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }

    let image = RenderImage::new(SmallVec::from_elem(Frame::new(buffer), 1))
        .with_scale_factor(scale);
    Ok(Arc::new(image))
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use gpui::{div, px, TestAppContext};

    /// A minimal view that displays a RenderImage the same way TypstPreviewView does.
    struct TestImageView {
        image: Arc<RenderImage>,
    }

    impl gpui::Render for TestImageView {
        fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
            let image_size = self.image.size(0);
            let image_w = image_size.width.0 as f32;
            let image_h = image_size.height.0 as f32;
            let aspect = if image_w > 0.0 { image_h / image_w } else { 1.0 };
            let display_w = px(image_w / 2.0);
            let display_h = px(image_w / 2.0 * aspect);

            div()
                .id("test-container")
                .size_full()
                .flex()
                .flex_col()
                .items_center()
                .overflow_y_scroll()
                .child(
                    gpui::img(gpui::ImageSource::Render(self.image.clone()))
                        .id(ElementId::Integer(self.image.id.0 as u64))
                        .debug_selector(|| "TEST_IMG".into())
                        .w(display_w)
                        .h(display_h),
                )
        }
    }

    fn make_test_image(width: u32, height: u32, scale: f32) -> Arc<RenderImage> {
        // Create a minimal BGRA pixmap.
        let data = vec![128u8; (width * height * 4) as usize];
        let buffer = image::ImageBuffer::from_raw(width, height, data)
            .expect("buffer size mismatch");
        Arc::new(
            RenderImage::new(SmallVec::from_elem(Frame::new(buffer), 1))
                .with_scale_factor(scale),
        )
    }

    #[gpui::test]
    async fn test_image_display_bounds_consistent_across_updates(cx: &mut TestAppContext) {
        // Create a window with a TestImageView showing image #1.
        let image1 = make_test_image(1119, 1588, 2.0);
        let (view, cx) = cx.add_window_view(|_window, _cx| TestImageView {
            image: image1.clone(),
        });

        // Read bounds of image #1.
        let bounds1 = cx.debug_bounds("TEST_IMG");
        assert!(
            bounds1.is_some(),
            "TEST_IMG element should exist after first render"
        );
        let bounds1 = bounds1.unwrap();
        assert!(
            bounds1.size.width.as_f32() > 100.0,
            "Image should have reasonable width, got {}",
            bounds1.size.width.as_f32(),
        );

        // Swap to image #2 (same pixel dimensions, different RenderImage instance).
        let image2 = make_test_image(1119, 1588, 2.0);
        assert_ne!(
            image1.id, image2.id,
            "Two RenderImages should have different IDs"
        );

        view.update_in(cx, |view, _window, cx| {
            view.image = image2;
            cx.notify();
        });
        cx.run_until_parked();

        // Read bounds of image #2.
        let bounds2 = cx.debug_bounds("TEST_IMG");
        assert!(
            bounds2.is_some(),
            "TEST_IMG element should exist after second render"
        );
        let bounds2 = bounds2.unwrap();

        // The display bounds must be identical.
        assert_eq!(
            bounds1.size.width.as_f32(), bounds2.size.width.as_f32(),
            "Width changed between image updates: {} -> {}",
            bounds1.size.width.as_f32(), bounds2.size.width.as_f32(),
        );
        assert_eq!(
            bounds1.size.height.as_f32(), bounds2.size.height.as_f32(),
            "Height changed between image updates: {} -> {}",
            bounds1.size.height.as_f32(), bounds2.size.height.as_f32(),
        );
    }

    #[gpui::test]
    async fn test_image_display_size_matches_expected_dimensions(cx: &mut TestAppContext) {
        // 1119x1588 pixels at 2x scale → 559.5x794 display points
        let image = make_test_image(1119, 1588, 2.0);
        let (_view, cx) = cx.add_window_view(|_window, _cx| TestImageView {
            image,
        });

        let bounds = cx.debug_bounds("TEST_IMG").expect("TEST_IMG should exist");

        // The display width should be pixel_width / scale = 1119 / 2 = 559.5
        let expected_w = 1119.0 / 2.0;
        let expected_h = 1588.0 / 2.0;
        assert!(
            (bounds.size.width.as_f32() - expected_w).abs() < 1.0,
            "Expected display width ~{expected_w}, got {}",
            bounds.size.width.as_f32(),
        );
        assert!(
            (bounds.size.height.as_f32() - expected_h).abs() < 1.0,
            "Expected display height ~{expected_h}, got {}",
            bounds.size.height.as_f32(),
        );
    }
}
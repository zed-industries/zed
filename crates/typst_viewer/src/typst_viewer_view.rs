use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use anyhow::Context as _;
use async_tungstenite::tungstenite::Message;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{
    App, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render,
    RenderImage, ScrollHandle, SharedString, Task, Window, actions,
};
use image::Frame;
use language::Buffer;
use multi_buffer::MultiBuffer;
use project::Project;
use settings::Settings as _;
use smallvec::SmallVec;
use ui::prelude::*;
use ui::{ScrollAxes, Scrollbars, TrackClickBehavior, WithScrollbar};
use workspace::item::Item;
use workspace::{Pane, Workspace};

use crate::svg_stream;
use crate::{OpenPreview, OpenPreviewToTheSide};

actions!(
    typst,
    [
        /// Scrolls the preview to the top of the next page.
        NextPage,
        /// Scrolls the preview to the top of the previous page.
        PreviousPage,
    ]
);

const PAGE_GAP: f32 = 12.0;
// Sizes for pages whose SVG source hasn't arrived yet, so placeholders keep
// roughly the right height and the scrollbar doesn't jump when they load.
// A4 portrait in points.
const DEFAULT_PAGE_WIDTH: f32 = 595.0;
const DEFAULT_PAGE_HEIGHT: f32 = 842.0;

/// A page's SVG source with its intrinsic display size, parsed once on
/// receipt.  The glyph defs are cached separately per page and re-injected
/// before rasterizing; stripping them keeps the cached source small.
pub(crate) struct PageSource {
    pub(crate) svg: Arc<str>,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

enum PreviewState {
    Connecting,
    Rendering {
        pages: Vec<Option<Arc<RenderImage>>>,
    },
    Disconnected {
        reason: String,
    },
    Error {
        message: String,
    },
    /// Compilation failed: show the reasons until a successful frame arrives.
    CompileError {
        errors: Vec<String>,
    },
}

pub struct TypstPreviewView {
    source_buffer: Option<Entity<Buffer>>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    state: PreviewState,
    /// Per-page SVG sources for on-demand rasterization (bitmap eviction
    /// window); grows with the document by design, per the memory trade-off.
    page_sources: BTreeMap<usize, PageSource>,
    /// Per-page glyph defs extracted from frames that carried them, injected
    /// back before rasterizing pages the server sent with defs stripped.
    glyph_defs: HashMap<usize, String>,
    /// Pages with a rasterization task in flight, to avoid duplicate work.
    pending_rasterize: HashSet<usize>,
    /// Last fallback top-up from render (throttled); see `top_up_rasterization`.
    last_top_up: std::time::Instant,
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
                page_sources: BTreeMap::new(),
                glyph_defs: HashMap::new(),
                pending_rasterize: HashSet::new(),
                last_top_up: std::time::Instant::now(),
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

    /// Reset and restart the connection for a preview that reached a
    /// terminal state. `start_connection` runs `start_preview_via_lsp`,
    /// which kills the old server-side task first, so this is safe even if
    /// a stale task is somehow still alive.
    fn restart_connection(&mut self, cx: &mut Context<Self>) {
        log::info!("typst_viewer: restarting preview connection after terminal state");
        self.page_sources.clear();
        self.glyph_defs.clear();
        self.pending_rasterize.clear();
        self.state = PreviewState::Connecting;
        self.start_connection(cx);
        cx.notify();
    }

    /// The task ID tinymist knows this preview by: the entry file's absolute
    /// path string, identical to the ID used when the preview was started.
    fn preview_task_id(&self, cx: &App) -> Option<String> {
        self.source_buffer
            .as_ref()
            .and_then(|buffer| buffer.read(cx).file())
            .and_then(|file| file.as_local())
            .map(|file| file.abs_path(cx))
            .and_then(|path| path.to_str().map(str::to_string))
    }

    /// Try to get a WebSocket URL by asking the tinymist LSP to start a
    /// preview. Returns `None` if tinymist isn't available.
    /// Also returns any LSP subscriptions that must be kept alive.
    async fn request_preview_url(
        project: &Entity<Project>,
        source_buffer: &Option<Entity<Buffer>>,
        cx: &mut gpui::AsyncApp,
    ) -> anyhow::Result<(String, Vec<lsp::Subscription>)> {
        let (server, request_timeout, entry_path) = project.read_with(cx, |project, cx| {
            let buffer = source_buffer.as_ref().map(|b| b.read(cx));
            let server_id = crate::find_tinymist_server(project, buffer.as_deref(), cx)
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
        // reload), and right after opening a .typ the language server may
        // not have spawned yet (spawn-to-ready takes ~4-6s). Retry with
        // growing delays so both settle before we give up.
        const MAX_CONNECT_ATTEMPTS: u32 = 5;
        for attempt in 0..MAX_CONNECT_ATTEMPTS {
            match Self::try_connect_and_receive(&project, &source_buffer, this, cx).await {
                Ok(()) => return Ok(()),
                Err(err) => {
                    if attempt + 1 < MAX_CONNECT_ATTEMPTS {
                        // 1s, 2s, 3s, 4s — 10s total window for the LSP to spawn.
                        let delay = std::time::Duration::from_secs(1 + attempt as u64);
                        log::warn!(
                            "typst_viewer: attempt {}/{MAX_CONNECT_ATTEMPTS} failed: {err:#}, \
                             retrying in {}s",
                            attempt + 1,
                            delay.as_secs(),
                        );
                        smol::Timer::after(delay).await;
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
        let (url, lsp_subscriptions) =
            Self::request_preview_url(project, source_buffer, cx).await?;

        if !lsp_subscriptions.is_empty() {
            this.update(cx, |this, _cx| {
                this._lsp_subscriptions.extend(lsp_subscriptions);
            })
            .ok();
        }

        log::info!("typst_viewer: connecting to LSP-provided preview at {url}");
        let mut ws = svg_stream::connect(&url)
            .await
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
            log::warn!("typst_viewer: page {page_index}/{page_total} has no <svg tag, skipping");
            return None;
        }

        Some((page_index, page_total, svg_text.as_bytes().to_vec()))
    }

    async fn receive_loop(
        ws: &mut svg_stream::PreviewSocket,
        this: &gpui::WeakEntity<Self>,
        cx: &mut gpui::AsyncApp,
    ) -> anyhow::Result<()> {
        while let Some(msg_result) = ws.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    // Compile-failure frame from tinymist: enter the error
                    // placeholder.  The next successful page frame switches
                    // the state back to Rendering.
                    if let Some(payload) = text.strip_prefix("error\n") {
                        let errors = parse_error_frame(payload);
                        log::info!("typst_viewer: compile error: {errors:?}");
                        this.update(cx, |this, cx| {
                            this.state = PreviewState::CompileError { errors };
                            cx.notify();
                        })?;
                        continue;
                    }

                    // --- Phase 1: Parse the triggering message ---
                    let Some((page_index, page_total, svg_bytes)) = Self::parse_svg_message(&text)
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
                                if let Some((pi, pt, bytes)) = Self::parse_svg_message(&newer) {
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

                    // --- Phase 2: Cache page sources, rasterize on demand ---
                    // Every page's SVG source is cached so bitmaps outside the
                    // viewport can be evicted and re-rasterized later; only
                    // pages near the current scroll position render now.
                    this.update(cx, |this, cx| {
                        match &mut this.state {
                            PreviewState::Rendering { pages } if pages.len() == latest_total => {}
                            _ => {
                                // First batch, or a structurally different
                                // document: drop stale sources and defs.
                                this.page_sources.clear();
                                this.glyph_defs.clear();
                                this.state = PreviewState::Rendering {
                                    pages: vec![None; latest_total],
                                };
                            }
                        }

                        for (page_index, svg_bytes) in &latest {
                            let svg_string = String::from_utf8_lossy(svg_bytes);
                            if let Some(defs) = extract_glyph_defs(&svg_string) {
                                this.glyph_defs.insert(*page_index, defs);
                            }
                            let stripped = strip_glyph_defs(&svg_string);
                            let Some((width, height)) = parse_svg_size(&stripped) else {
                                log::warn!(
                                    "typst_viewer: page {page_index}/{latest_total} \
                                     has no parsable size, skipping"
                                );
                                continue;
                            };
                            this.page_sources.insert(
                                *page_index,
                                PageSource {
                                    svg: Arc::from(stripped.as_ref()),
                                    width,
                                    height,
                                },
                            );
                            // The content changed, so any old bitmap for this
                            // page is stale — fall back to the placeholder.
                            if let PreviewState::Rendering { pages } = &mut this.state {
                                if *page_index < pages.len() {
                                    pages[*page_index] = None;
                                }
                            }
                        }

                        // Sources replaced by this batch may still have stale
                        // tasks in flight; those tasks discard their results
                        // by source identity, so allow re-queueing now.
                        this.pending_rasterize.clear();
                        this.request_rasterization(cx);
                        cx.notify();
                    })?;
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

    fn page_count(&self) -> usize {
        match &self.state {
            PreviewState::Rendering { pages } => pages.len(),
            _ => 0,
        }
    }

    /// Page indices intersecting the viewport, expanded by one page on each
    /// side — the rasterization window.  Pages whose source hasn't arrived
    /// yet are estimated at the default page height.
    fn visible_page_range(&self) -> std::ops::Range<usize> {
        let heights = self.page_display_heights();
        // Before the first layout the viewport size is unknown; assume one
        // page is visible so the initial rasterization window stays small.
        let bounds_height = self.scroll_handle.bounds().size.height.as_f32();
        let viewport_height = if bounds_height > 0.0 {
            bounds_height
        } else {
            heights.first().copied().unwrap_or(DEFAULT_PAGE_HEIGHT)
        };
        let scroll_y: f32 = self.scroll_handle.offset().y.abs().into();
        visible_page_range_at(scroll_y, viewport_height, &heights, PAGE_GAP)
    }

    /// Rasterize pages in and near the viewport whose bitmaps are missing,
    /// and evict bitmaps outside that window so memory doesn't grow with
    /// the document length.  Called from the receive loop after a batch and
    /// from render, which fires on scroll.
    fn request_rasterization(&mut self, cx: &mut Context<Self>) {
        if !matches!(self.state, PreviewState::Rendering { .. }) {
            return;
        }

        let range = self.visible_page_range();

        if let PreviewState::Rendering { pages } = &mut self.state {
            for (index, page) in pages.iter_mut().enumerate() {
                if !range.contains(&index) {
                    *page = None;
                }
            }
        }

        let center = (range.start + range.end) / 2;
        let mut wanted: Vec<usize> = self
            .page_sources
            .range(range.clone())
            .map(|(index, _)| *index)
            .filter(|index| {
                let has_bitmap = match &self.state {
                    PreviewState::Rendering { pages } => {
                        pages.get(*index).is_some_and(Option::is_some)
                    }
                    _ => false,
                };
                !has_bitmap && !self.pending_rasterize.contains(index)
            })
            .collect();
        // Render the page under the user's eyes first when several pages
        // queue up at once.
        wanted.sort_by_key(|index| (*index as isize - center as isize).unsigned_abs());

        for index in wanted {
            let Some(source) = self.page_sources.get(&index) else {
                continue;
            };
            let source_identity = source.svg.clone();
            let full_svg = match self.glyph_defs.get(&index) {
                Some(defs) => inject_glyph_defs(source.svg.as_bytes(), defs),
                None => source.svg.as_bytes().to_vec(),
            };
            self.pending_rasterize.insert(index);
            cx.spawn(async move |this, cx| {
                let rasterize_started = std::time::Instant::now();
                let result = cx
                    .background_executor()
                    .spawn(async move { rasterize_svg_to_image(&full_svg, 2.0) })
                    .await;
                let elapsed_ms = rasterize_started.elapsed().as_secs_f64() * 1000.0;
                this.update(cx, |this, cx| {
                    this.pending_rasterize.remove(&index);
                    // Discard a stale task's result: a newer compile batch
                    // replaced this page's source after the task was spawned.
                    let source_is_current = this
                        .page_sources
                        .get(&index)
                        .is_some_and(|source| Arc::ptr_eq(&source.svg, &source_identity));
                    if !source_is_current {
                        return;
                    }
                    match result {
                        Ok(image) => {
                            log::info!(
                                "typst_viewer: page {index}/{} rasterized in {elapsed_ms:.0}ms",
                                this.page_count(),
                            );
                            if let PreviewState::Rendering { pages } = &mut this.state {
                                if index < pages.len() {
                                    pages[index] = Some(image);
                                    cx.notify();
                                }
                            }
                        }
                        Err(err) => {
                            log::error!(
                                "typst_viewer: rasterization of page {index} failed \
                                 in {elapsed_ms:.0}ms: {err}"
                            );
                        }
                    }
                })
                .ok();
            })
            .detach();
        }
    }

    /// Placeholder size for a page whose SVG source hasn't arrived yet.
    ///
    /// Pages are requested on demand, so most placeholders have no source of
    /// their own. Typst documents overwhelmingly use one page size, so scale
    /// unknown pages from the first known one; the A4 constant only covers
    /// the window before any page has arrived. This keeps the laid-out
    /// content height close to the real one, so the scrollbar thumb and
    /// page-jump targets don't shift as pages stream in (a hard-coded A4
    /// fallback on a non-A4 document inflated the total height by half and
    /// dragged every offset with it).
    fn page_size(&self, index: usize) -> (f32, f32) {
        if let Some(source) = self.page_sources.get(&index) {
            (source.width, source.height)
        } else if let Some((_, source)) = self.page_sources.iter().next() {
            (source.width, source.height)
        } else {
            (DEFAULT_PAGE_WIDTH, DEFAULT_PAGE_HEIGHT)
        }
    }

    /// Display height per page in render order. Shared by the visible range
    /// calculation and page navigation.
    fn page_display_heights(&self) -> Vec<f32> {
        (0..self.page_count())
            .map(|index| self.page_size(index).1)
            .collect()
    }

    /// Index of the page at the top edge of the viewport.
    fn top_page_index(&self) -> usize {
        let scroll_y: f32 = self.scroll_handle.offset().y.abs().into();
        let mut page_top = PAGE_GAP;
        let mut top_page = 0;
        for (index, &height) in self.page_display_heights().iter().enumerate() {
            let page_bottom = page_top + height;
            if page_bottom > scroll_y {
                return index;
            }
            page_top = page_bottom + PAGE_GAP;
            top_page = index;
        }
        top_page
    }

    /// Scroll so the given page's top sits at the viewport's top edge.
    /// Scroll offsets are non-positive, hence the negation.
    fn scroll_to_page(&mut self, page_index: usize, cx: &mut Context<Self>) {
        let heights = self.page_display_heights();
        if heights.get(page_index).is_none() {
            return;
        }
        let mut page_top = PAGE_GAP;
        for &height in &heights[..page_index] {
            page_top += height + PAGE_GAP;
        }
        log::debug!(
            "typst_viewer: [page-nav] scroll_to_page({page_index}) page_top={page_top} \
             offset_before_y={}",
            self.scroll_handle.offset().y,
        );
        self.scroll_handle
            .set_offset(gpui::point(gpui::px(0.0), gpui::px(-page_top)));
        self.request_rasterization(cx);
        cx.notify();
    }

    fn next_page(&mut self, _: &NextPage, _window: &mut Window, cx: &mut Context<Self>) {
        log::debug!("typst_viewer: [page-nav] NextPage action fired");
        let target = (self.top_page_index() + 1).min(self.page_count().saturating_sub(1));
        self.scroll_to_page(target, cx);
    }

    fn previous_page(&mut self, _: &PreviousPage, _window: &mut Window, cx: &mut Context<Self>) {
        log::debug!("typst_viewer: [page-nav] PreviousPage action fired");
        let top = self.top_page_index();
        // At the page's top already → go one page up; otherwise first
        // return to this page's top (so a double-tap walks pages).
        let at_page_top = {
            let scroll_y: f32 = self.scroll_handle.offset().y.abs().into();
            let mut page_top = PAGE_GAP;
            for &height in &self.page_display_heights()[..top] {
                page_top += height + PAGE_GAP;
            }
            (scroll_y - page_top).abs() < 1.0
        };
        let target = if at_page_top {
            top.saturating_sub(1)
        } else {
            top
        };
        self.scroll_to_page(target, cx);
    }

    /// Fallback top-up for scroll paths that fire no event we can hook
    /// (e.g. dragging the scrollbar thumb): render calls this, but it only
    /// acts when a visible page is missing and at most once per interval,
    /// so the event-driven paths (wheel, page actions, frame batches) stay
    /// free of render-time side effects.
    fn top_up_rasterization(&mut self, cx: &mut Context<Self>) {
        if !matches!(self.state, PreviewState::Rendering { .. }) {
            return;
        }
        let range = self.visible_page_range();
        let missing = self
            .page_sources
            .range(range.clone())
            .filter(|&(index, _)| {
                let has_bitmap = match &self.state {
                    PreviewState::Rendering { pages } => {
                        pages.get(*index).is_some_and(Option::is_some)
                    }
                    _ => false,
                };
                !has_bitmap && !self.pending_rasterize.contains(index)
            })
            .count();
        if missing == 0 {
            return;
        }
        let now = std::time::Instant::now();
        if now.duration_since(self.last_top_up) >= std::time::Duration::from_millis(150) {
            log::debug!(
                "typst_viewer: [scroll-diag] missing={missing} range={:?} offset_y={} \
                 max_offset_y={} viewport_h={} pages={}",
                range,
                self.scroll_handle.offset().y.as_f32(),
                self.scroll_handle.max_offset().y.as_f32(),
                self.scroll_handle.bounds().size.height.as_f32(),
                self.page_count(),
            );
            self.last_top_up = now;
            self.request_rasterization(cx);
        }
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
    ) -> Option<(Entity<TypstPreviewView>, usize)> {
        let buffer_id = buffer.read(cx).as_singleton()?.entity_id();
        pane.items_of_type::<TypstPreviewView>()
            .find(|view| {
                view.read(cx)
                    .source_buffer
                    .as_ref()
                    .is_some_and(|buffer| buffer.entity_id() == buffer_id)
            })
            .and_then(|view| pane.index_for_item(&view).map(|idx| (view, idx)))
    }

    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(buffer) = Self::resolve_active_item_as_typst_buffer(workspace, cx)
                && Self::is_typst_file(&buffer, cx)
            {
                let project = workspace.project().clone();
                let pane = workspace.active_pane().clone();
                if let Some((existing, existing_idx)) =
                    Self::find_existing_preview_item_idx(pane.read(cx), &buffer, cx)
                {
                    pane.update(cx, |pane, cx| {
                        Self::revive_if_terminal(&existing, cx);
                        pane.activate_item(existing_idx, true, true, window, cx);
                    });
                } else {
                    // Only build the view when actually adding it: the
                    // constructor starts an LSP round-trip whose kill leg
                    // could tear down a reused preview.
                    let view = TypstPreviewView::new(buffer.clone(), project, window, cx);
                    pane.update(cx, |pane, cx| {
                        pane.add_item(Box::new(view), true, true, None, window, cx);
                    });
                }
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenPreviewToTheSide, window, cx| {
            if let Some(buffer) = Self::resolve_active_item_as_typst_buffer(workspace, cx)
                && Self::is_typst_file(&buffer, cx)
            {
                let project = workspace.project().clone();
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
                if let Some((existing, existing_idx)) =
                    Self::find_existing_preview_item_idx(pane.read(cx), &buffer, cx)
                {
                    pane.update(cx, |pane, cx| {
                        Self::revive_if_terminal(&existing, cx);
                        pane.activate_item(existing_idx, true, true, window, cx);
                    });
                } else {
                    let view = TypstPreviewView::new(buffer.clone(), project, window, cx);
                    pane.update(cx, |pane, cx| {
                        pane.add_item(Box::new(view), false, false, None, window, cx);
                    });
                }
                cx.notify();
            }
        });
    }

    /// Reconnect a reused preview that died: a dropped WebSocket or a
    /// failed connect can't recover on its own, and without this the
    /// keybinding would just focus a dead tab. Compile errors recover by
    /// themselves once a save succeeds, so they are left alone.
    fn revive_if_terminal(existing: &Entity<TypstPreviewView>, cx: &mut App) {
        existing.update(cx, |view, cx| {
            if Self::is_terminal_state(&view.state) {
                view.restart_connection(cx);
            }
        });
    }

    fn is_terminal_state(state: &PreviewState) -> bool {
        matches!(
            state,
            PreviewState::Disconnected { .. } | PreviewState::Error { .. }
        )
    }
}

impl Render for TypstPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Rasterization is event-driven (frame batches, scroll wheel, page
        // actions); this is only the throttled fallback for event-less
        // scrolling like dragging the scrollbar thumb.
        self.top_up_rasterization(cx);

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

                let mut pages_column = div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .pt(gpui::px(PAGE_GAP));

                for (index, page) in pages.iter().enumerate() {
                    let page_element = match page {
                        Some(image) => {
                            let image_size = image.size(0);
                            let image_width = image_size.width.0 as f32;
                            let image_height = image_size.height.0 as f32;
                            let aspect = if image_width > 0.0 {
                                image_height / image_width
                            } else {
                                1.0
                            };
                            let display_width = image_width / 2.0;
                            let display_height = image_width / 2.0 * aspect;

                            gpui::img(gpui::ImageSource::Render(image.clone()))
                                .id(ElementId::Integer(image.id.0 as u64))
                                .debug_selector(|| "TYPST_PREVIEW_IMG".into())
                                .w(gpui::px(display_width))
                                .h(gpui::px(display_height))
                                .into_any_element()
                        }
                        None => {
                            // Keep the placeholder at the page's real size so
                            // the scroll height doesn't jump when the bitmap
                            // arrives (which also keeps the scrollbar thumb
                            // stable). Unknown sizes scale from the first
                            // known page — see page_size().
                            let (display_width, display_height) = self.page_size(index);

                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(gpui::px(display_width))
                                .h(gpui::px(display_height))
                                .child(SharedString::from(format!("Loading page {}…", index + 1)))
                                .into_any_element()
                        }
                    };
                    pages_column =
                        pages_column.child(div().pb(gpui::px(PAGE_GAP)).child(page_element));
                }

                // The scrollbar must hang off a container that does not
                // scroll: custom_scrollbars() adds the scrollbar as a child of
                // this element, and every child of a scrolling container is
                // translated by its scroll offset. Attached to the scroller
                // itself, the thumb is drawn and hit-tested above the
                // viewport once the offset is non-zero — it disappears on the
                // first wheel tick, and thumb drags feed the shifted track
                // geometry back into the offset, running away to the last page.
                div()
                    .id("typst-viewer-viewport")
                    .flex_1()
                    // Without an explicit zero minimum, a non-scrolling flex
                    // item keeps its content's min size: this wrapper would
                    // grow to the full document height, the inner scrolling
                    // div would inherit it as its viewport, and "visible
                    // pages" would become the whole document — every page
                    // then gets rasterized (~14 MB each at 2x).
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .id("typst-viewer-scroll")
                            .flex_1()
                            .min_h_0()
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll_handle)
                            .on_scroll_wheel(cx.listener(|this, _event, _window, cx| {
                                this.request_rasterization(cx);
                            }))
                            .child(pages_column),
                    )
                    // always_visible: the thumb is the main navigation for
                    // long documents; the autohide default hides it 1s after
                    // scrolling stops, which reads as "scrollbar disappears".
                    // notify_content() so drag-to-scroll notifies this view;
                    // vertical_scrollbar_for() hides that hook, leaving the
                    // offset written but the content never re-laid-out.
                    // track_click_behavior: with 500+ pages the thumb sits at
                    // its 25px minimum, so track clicks usually miss it and
                    // the default jump-to-position teleports to the clicked
                    // ratio; page-scroll keeps track clicks to one screen.
                    .custom_scrollbars(
                        Scrollbars::always_visible(ScrollAxes::Vertical)
                            .tracked_scroll_handle(&self.scroll_handle)
                            .notify_content()
                            .track_click_behavior(TrackClickBehavior::PageScroll)
                            .id("typst-preview-scrollbar"),
                        window,
                        cx,
                    )
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

            PreviewState::CompileError { errors } => {
                let mut column = div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .child(SharedString::from("Compile error:"));
                for error in errors {
                    column = column.child(SharedString::from(error.clone()));
                }
                column.into_any_element()
            }
        };

        div()
            .key_context("TypstPreview")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::next_page))
            .on_action(cx.listener(Self::previous_page))
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().editor_background)
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

    /// The pane dropped this tab: dispose the matching server-side task so
    /// tinymist stops compiling a document nobody watches. Deliberately
    /// `on_removed` (fires once per real removal) and NOT `deactivated`,
    /// which also fires when switching the active tab and would kill the
    /// preview of a document the user merely navigated away from.
    fn on_removed(&self, cx: &mut Context<Self>) {
        let Some(task_id) = self.preview_task_id(cx) else {
            log::info!(
                "typst_viewer: preview tab removed without a file path; \
                 skipping tinymist.doKillPreview"
            );
            return;
        };

        let buffer = self.source_buffer.as_ref().map(|buffer| buffer.read(cx));
        let server_id = crate::find_tinymist_server(&self.project.read(cx), buffer, cx);
        let request_timeout = project::project_settings::ProjectSettings::get_global(cx)
            .global_lsp_settings
            .get_request_timeout();

        let server = server_id.and_then(|server_id| {
            self.project
                .read(cx)
                .lsp_store()
                .read(cx)
                .language_server_for_id(server_id)
        });
        match server {
            Some(server) => {
                // Detached on purpose: the view is going away, but this task
                // only touches the server and plain data, never the view.
                cx.spawn(async move |_this, _cx| {
                    if let Err(err) = crate::kill_preview_via_lsp(&server, &task_id, request_timeout)
                        .await
                    {
                        log::info!("typst_viewer: doKillPreview after tab close failed: {err:#}");
                    }
                })
                .detach();
            }
            None => log::info!(
                "typst_viewer: tinymist server not running; \
                 skipping doKillPreview for {task_id}"
            ),
        }
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

/// Parse the payload of an `error\n` frame sent on compile failure.  Falls
/// back to the raw payload when the JSON is malformed, so the reason is
/// never silently lost.
pub(crate) fn parse_error_frame(payload: &str) -> Vec<String> {
    let fallback = || vec![payload.trim().to_string()];
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(value) => value
            .get("errors")
            .and_then(|errors| errors.as_array())
            .map(|errors| {
                errors
                    .iter()
                    .filter_map(|error| error.as_str().map(str::to_string))
                    .collect::<Vec<_>>()
            })
            .filter(|errors| !errors.is_empty())
            .unwrap_or_else(fallback),
        Err(_) => fallback(),
    }
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

/// Compute which pages intersect the viewport from per-page display heights
/// and the top-padding/gap layout used by the render code, expanded by one
/// page on each side for the rasterization window.
pub(crate) fn visible_page_range_at(
    scroll_y: f32,
    viewport_height: f32,
    page_heights: &[f32],
    gap: f32,
) -> std::ops::Range<usize> {
    if page_heights.is_empty() {
        return 0..0;
    }
    let viewport_height = if viewport_height > 0.0 {
        viewport_height
    } else {
        page_heights[0].max(1.0)
    };
    let mut first_visible = None;
    let mut last_visible = None;
    let mut page_top = gap;
    for (index, &height) in page_heights.iter().enumerate() {
        let page_bottom = page_top + height;
        if page_bottom > scroll_y && page_top < scroll_y + viewport_height {
            first_visible.get_or_insert(index);
            last_visible = Some(index);
        }
        page_top = page_bottom + gap;
    }
    if first_visible.is_none() {
        // The scroll offset lies outside the content (e.g. mid-layout or a
        // mid-update clamp): pin the window to the start or the end rather
        // than rasterizing everything.
        if scroll_y <= 0.0 {
            return 0..2.min(page_heights.len());
        }
        return page_heights.len().saturating_sub(2)..page_heights.len();
    }
    let first = first_visible.unwrap_or(0);
    let last = last_visible.unwrap_or(page_heights.len() - 1);
    first.saturating_sub(1)..(last + 2).min(page_heights.len())
}

/// Extract the glyph defs block from an SVG frame that carries it, for
/// re-injection into later frames where the server strips the defs.
pub(crate) fn extract_glyph_defs(svg: &str) -> Option<String> {
    let start = svg.find(GLYPH_DEFS_OPEN)?;
    let end_offset = svg[start..].find(DEFS_CLOSE)?;
    let defs_end = start + end_offset + DEFS_CLOSE.len();
    Some(svg[start..defs_end].to_string())
}

/// Remove the glyph defs block from an SVG source so cached pages don't
/// repeat the (potentially large) shared defs in every entry.
pub(crate) fn strip_glyph_defs(svg: &str) -> std::borrow::Cow<'_, str> {
    match extract_glyph_defs(svg) {
        Some(defs) => std::borrow::Cow::Owned(svg.replacen(&defs, "", 1)),
        None => std::borrow::Cow::Borrowed(svg),
    }
}

/// Parse a page's intrinsic display size in points from the root `<svg>`
/// tag, preferring the width/height attributes and falling back to the
/// viewBox.  Typst emits both in points.
pub(crate) fn parse_svg_size(svg: &str) -> Option<(f32, f32)> {
    let tag_end = svg.find('>')?;
    let header = &svg[..tag_end];
    let width = svg_attribute(header, "width").and_then(parse_length_pt);
    let height = svg_attribute(header, "height").and_then(parse_length_pt);
    match (width, height) {
        (Some(width), Some(height)) => Some((width, height)),
        _ => {
            let view_box = svg_attribute(header, "viewBox")?;
            let mut parts = view_box.split_whitespace();
            let _min_x = parts.next()?;
            let _min_y = parts.next()?;
            let width: f32 = parts.next()?.parse().ok()?;
            let height: f32 = parts.next()?.parse().ok()?;
            Some((width, height))
        }
    }
}

fn svg_attribute<'a>(header: &'a str, name: &str) -> Option<&'a str> {
    let pattern = format!("{name}=\"");
    let value_start = header.find(&pattern)? + pattern.len();
    let value_end = value_start + header[value_start..].find('"')?;
    Some(&header[value_start..value_end])
}

fn parse_length_pt(value: &str) -> Option<f32> {
    value.strip_suffix("pt").unwrap_or(value).parse().ok()
}

pub(crate) fn rasterize_svg_to_image(
    svg_bytes: &[u8],
    scale: f32,
) -> anyhow::Result<Arc<RenderImage>> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_bytes, &options)?;

    let size = tree.size();
    log::info!(
        "typst_viewer: full rasterize SVG size: {}x{} ({}x{} px at {scale}x)",
        size.width(),
        size.height(),
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

    let mut buffer =
        image::ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
            .ok_or_else(|| anyhow::anyhow!("pixmap data didn't match expected buffer size"))?;

    // GPUI expects BGRA pixel format (Metal textures use BGRA8Unorm).
    // tiny-skia produces premultiplied RGBA. Swap R↔B channels.
    for pixel in buffer.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }

    let image =
        RenderImage::new(SmallVec::from_elem(Frame::new(buffer), 1)).with_scale_factor(scale);
    Ok(Arc::new(image))
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use gpui::{TestAppContext, div, px};

    /// A minimal view that displays a RenderImage the same way TypstPreviewView does.
    struct TestImageView {
        image: Arc<RenderImage>,
    }

    impl gpui::Render for TestImageView {
        fn render(
            &mut self,
            _window: &mut Window,
            _cx: &mut gpui::Context<Self>,
        ) -> impl IntoElement {
            let image_size = self.image.size(0);
            let image_w = image_size.width.0 as f32;
            let image_h = image_size.height.0 as f32;
            let aspect = if image_w > 0.0 {
                image_h / image_w
            } else {
                1.0
            };
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
        let buffer =
            image::ImageBuffer::from_raw(width, height, data).expect("buffer size mismatch");
        Arc::new(
            RenderImage::new(SmallVec::from_elem(Frame::new(buffer), 1)).with_scale_factor(scale),
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
            bounds1.size.width.as_f32(),
            bounds2.size.width.as_f32(),
            "Width changed between image updates: {} -> {}",
            bounds1.size.width.as_f32(),
            bounds2.size.width.as_f32(),
        );
        assert_eq!(
            bounds1.size.height.as_f32(),
            bounds2.size.height.as_f32(),
            "Height changed between image updates: {} -> {}",
            bounds1.size.height.as_f32(),
            bounds2.size.height.as_f32(),
        );
    }

    #[gpui::test]
    async fn test_image_display_size_matches_expected_dimensions(cx: &mut TestAppContext) {
        // 1119x1588 pixels at 2x scale → 559.5x794 display points
        let image = make_test_image(1119, 1588, 2.0);
        let (_view, cx) = cx.add_window_view(|_window, _cx| TestImageView { image });

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

#[cfg(test)]
mod error_frame_tests {
    use super::parse_error_frame;

    #[test]
    fn parse_error_frame_extracts_errors() {
        let json = r#"{"kind":"compileError","errors":["expected expression, found keyword: hi","unknown variable: x"]}"#;
        // The server terminates the frame with a newline, like page frames.
        let errors = parse_error_frame(&format!("{json}\n"));
        assert_eq!(
            errors,
            vec![
                "expected expression, found keyword: hi",
                "unknown variable: x",
            ]
        );
    }

    #[test]
    fn parse_error_frame_falls_back_to_raw_payload() {
        assert_eq!(parse_error_frame("not json"), vec!["not json"]);
        let empty = r#"{"kind":"compileError","errors":[]}"#;
        assert_eq!(parse_error_frame(empty), vec![empty]);
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use super::{PreviewState, TypstPreviewView};

    #[test]
    fn terminal_states_are_disconnected_and_error_only() {
        assert!(TypstPreviewView::is_terminal_state(&PreviewState::Disconnected {
            reason: "server closed".into()
        }));
        assert!(TypstPreviewView::is_terminal_state(&PreviewState::Error {
            message: "connect failed".into()
        }));
        // These recover by themselves (frames keep arriving / retries
        // pending), so reviving them would discard live state.
        assert!(!TypstPreviewView::is_terminal_state(&PreviewState::Connecting));
        assert!(!TypstPreviewView::is_terminal_state(&PreviewState::Rendering {
            pages: vec![None]
        }));
        assert!(!TypstPreviewView::is_terminal_state(&PreviewState::CompileError {
            errors: vec!["unknown variable: hi".into()]
        }));
    }
}

#[cfg(test)]
mod on_demand_tests {
    use super::{
        GLYPH_DEFS_OPEN, extract_glyph_defs, inject_glyph_defs, parse_svg_size, strip_glyph_defs,
        visible_page_range_at,
    };

    #[test]
    fn parse_svg_size_reads_pt_attributes() {
        let svg = r##"<svg class="typst-doc" viewBox="0 0 283.46 67.98" width="283.46pt" height="67.98pt" xmlns="http://www.w3.org/2000/svg"><path fill="#ffffff"/></svg>"##;
        let (width, height) = parse_svg_size(svg).expect("size should parse");
        assert!((width - 283.46).abs() < 0.01, "width: {width}");
        assert!((height - 67.98).abs() < 0.01, "height: {height}");
    }

    #[test]
    fn parse_svg_size_falls_back_to_view_box() {
        let svg = r##"<svg viewBox="0 0 595.28 841.89"><path fill="#ffffff"/></svg>"##;
        let (width, height) = parse_svg_size(svg).expect("size should parse");
        assert!((width - 595.28).abs() < 0.01, "width: {width}");
        assert!((height - 841.89).abs() < 0.01, "height: {height}");
    }

    #[test]
    fn parse_svg_size_returns_none_without_size() {
        assert!(parse_svg_size("<svg></svg>").is_none());
        assert!(parse_svg_size("").is_none());
    }

    #[test]
    fn glyph_defs_extract_strip_and_inject_roundtrip() {
        let defs = r#"<defs id="glyph"><symbol id="g1"><path d="M 0 0"/></symbol></defs>"#;
        let svg =
            format!(r#"<svg width="100pt" height="50pt">{defs}<g class="typst-text"/></svg>"#);
        assert_eq!(extract_glyph_defs(&svg).as_deref(), Some(defs));

        let stripped = strip_glyph_defs(&svg);
        assert!(!stripped.contains(GLYPH_DEFS_OPEN), "defs should be gone");
        assert!(stripped.contains(r#"<g class="typst-text"/>"#), "body kept");

        let reinjected = inject_glyph_defs(stripped.as_bytes(), defs);
        assert_eq!(
            reinjected,
            svg.as_bytes(),
            "roundtrip should restore the frame"
        );
    }

    #[test]
    fn visible_page_range_at_covers_viewport_plus_neighbors() {
        // 10 pages of 800 display px, gap 12: page i spans
        // [12 + i * 812, 12 + i * 812 + 800].
        let heights = [800.0_f32; 10];
        // Viewport [0, 1000): pages 0 and 1 visible, expand one page each way.
        assert_eq!(visible_page_range_at(0.0, 1000.0, &heights, 12.0), 0..3);
        // Viewport [2000, 3000): pages 2 and 3 visible.
        assert_eq!(visible_page_range_at(2000.0, 1000.0, &heights, 12.0), 1..5);
        // Viewport [7000, 8000): pages 8 and 9 visible, clamped at the end.
        assert_eq!(visible_page_range_at(7000.0, 1000.0, &heights, 12.0), 7..10);
    }

    #[test]
    fn visible_page_range_at_outside_content_pins_window() {
        let heights = [800.0_f32; 10];
        // Scrolled far beyond the content: pin to the last two pages.
        assert_eq!(
            visible_page_range_at(100_000.0, 1000.0, &heights, 12.0),
            8..10
        );
    }

    #[test]
    fn visible_page_range_at_empty_pages() {
        assert_eq!(visible_page_range_at(0.0, 1000.0, &[], 12.0), 0..0);
    }
}

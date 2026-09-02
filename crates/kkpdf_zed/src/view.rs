//! Core PDF viewer view state, interaction handlers, and GPUI layout calculations.

use anyhow::{Context as _, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::cache::{CacheKey, PageLruCache, RenderedPage, DEFAULT_MEMORY_BUDGET_BYTES};
use crate::document::{PageDimensions, PdfDocument};
use crate::pdfium::PdfiumEngine;
use crate::rasterizer::RasterizerOptions;
use crate::settings::{DefaultZoomPolicy, PageLayoutMode, PdfViewerSettings};
use crate::watcher::{PdfReloadDebouncer, ViewerStateSnapshot};

pub const MIN_ZOOM: f32 = 0.1;
pub const MAX_ZOOM: f32 = 20.0;
pub const ZOOM_STEP: f32 = 1.15;
pub const PAGE_SPACING_PX: f32 = 16.0;

/// Viewer event emitted to notify subscribers (e.g. tabs, breadcrumbs, toolbar).
#[derive(Debug, Clone, PartialEq)]
pub enum PdfViewEvent {
    /// Document title or filename changed.
    TitleChanged,
    /// Active page changed.
    PageChanged(usize),
    /// Active zoom level changed.
    ZoomChanged(f32),
    /// Document successfully reloaded from disk.
    Reloaded,
}

/// Core state representation for an interactive PDF viewing session.
pub struct PdfView {
    pub(crate) document: Option<PdfDocument>,
    pub(crate) engine: PdfiumEngine,
    pub(crate) cache: PageLruCache,
    pub(crate) settings: PdfViewerSettings,
    pub(crate) raw_bytes: Option<Arc<Vec<u8>>>,
    pub(crate) zoom_level: f32,
    pub(crate) pan_offset: (f32, f32),
    pub(crate) current_page: usize,
    pub(crate) dark_mode: bool,
    pub(crate) layout_mode: PageLayoutMode,
    pub(crate) last_mouse_pos: Option<(f32, f32)>,
    pub(crate) container_size: Option<(f32, f32)>,
    pub(crate) debouncer: Option<PdfReloadDebouncer>,
}

impl PdfView {
    /// Creates a new uninitialized PDF view with default settings.
    pub fn new(settings: PdfViewerSettings) -> Self {
        let cache_bytes = settings.cache_budget_mb * 1024 * 1024;
        Self {
            document: None,
            engine: PdfiumEngine::new(),
            cache: PageLruCache::new(cache_bytes.max(DEFAULT_MEMORY_BUDGET_BYTES)),
            zoom_level: 1.0,
            pan_offset: (0.0, 0.0),
            current_page: 0,
            dark_mode: settings.smart_dark_mode,
            layout_mode: settings.layout_mode,
            last_mouse_pos: None,
            container_size: None,
            debouncer: None,
            raw_bytes: None,
            settings,
        }
    }

    /// Loads a PDF document from file path.
    pub fn open_file(&mut self, path: &Path) -> Result<()> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read PDF file at {:?}", path))?;

        self.debouncer = Some(PdfReloadDebouncer::new(
            path.to_path_buf(),
            self.settings.reload_debounce_ms,
        ));

        self.load_from_bytes(bytes, Some(path.to_path_buf()))
    }

    /// Loads a PDF document from raw byte buffer.
    pub fn load_from_bytes(&mut self, bytes: Vec<u8>, path: Option<PathBuf>) -> Result<()> {
        let arc_bytes = Arc::new(bytes);
        let doc = self.engine.load_document_from_bytes(&arc_bytes, path)?;

        self.raw_bytes = Some(arc_bytes);
        self.document = Some(doc);
        self.cache.clear();
        self.current_page = 0;
        self.pan_offset = (0.0, 0.0);

        // Apply default zoom policy
        if let Some((width, height)) = self.container_size {
            self.apply_default_zoom_policy(width, height);
        }

        Ok(())
    }

    /// Reloads the document from disk while preserving the user's viewport state.
    pub fn reload_preserving_state(&mut self) -> Result<bool> {
        let path = match self.debouncer.as_ref().map(|d| d.path().to_path_buf()) {
            Some(p) => p,
            None => return Ok(false),
        };

        let snapshot = self.save_state_snapshot();
        self.open_file(&path)?;
        self.restore_state_snapshot(snapshot);

        if let Some(ref mut d) = self.debouncer {
            d.mark_reloaded();
        }

        Ok(true)
    }

    /// Captures the current navigation and viewport state.
    pub fn save_state_snapshot(&self) -> ViewerStateSnapshot {
        ViewerStateSnapshot {
            current_page: self.current_page,
            scroll_percent_y: 0.0,
            scroll_percent_x: 0.0,
            zoom_level: self.zoom_level,
            pan_x: self.pan_offset.0,
            pan_y: self.pan_offset.1,
        }
    }

    /// Restores a captured viewport state.
    pub fn restore_state_snapshot(&mut self, snapshot: ViewerStateSnapshot) {
        if let Some(ref doc) = self.document {
            if !doc.is_empty() {
                self.current_page = snapshot
                    .current_page
                    .min(doc.total_pages().saturating_sub(1));
            }
        }
        self.zoom_level = snapshot.zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
        self.pan_offset = (snapshot.pan_x, snapshot.pan_y);
    }

    /// Sets the viewport container size and updates fit-to-width/fit-to-page if on first layout.
    pub fn set_container_size(&mut self, width: f32, height: f32) {
        let first_layout = self.container_size.is_none();
        self.container_size = Some((width, height));
        if first_layout {
            self.apply_default_zoom_policy(width, height);
        }
    }

    /// Applies the default zoom policy based on container geometry and first page size.
    fn apply_default_zoom_policy(&mut self, container_width: f32, container_height: f32) {
        let first_page_size = self.document.as_ref().and_then(|d| d.page_size(0));
        let Some(page_dim) = first_page_size else {
            return;
        };

        match self.settings.default_zoom {
            DefaultZoomPolicy::FitWidth => {
                self.zoom_level = self.compute_fit_to_width_zoom(container_width, page_dim.width);
            }
            DefaultZoomPolicy::FitPage => {
                self.zoom_level =
                    self.compute_fit_to_page_zoom(container_width, container_height, page_dim);
            }
            DefaultZoomPolicy::ActualSize => {
                self.zoom_level = 1.0;
            }
        }
        self.pan_offset = (0.0, 0.0);
    }

    /// Computes zoom factor that fits page width to available container width.
    pub fn compute_fit_to_width_zoom(&self, container_width: f32, page_width_pts: f32) -> f32 {
        if page_width_pts <= 0.0 || container_width <= 0.0 {
            return 1.0;
        }
        let padding = 32.0; // Left and right gutter
        let available = (container_width - padding).max(100.0);
        let target_zoom = available / page_width_pts;
        target_zoom.clamp(MIN_ZOOM, MAX_ZOOM)
    }

    /// Computes zoom factor that fits entire page inside container bounds.
    pub fn compute_fit_to_page_zoom(
        &self,
        container_width: f32,
        container_height: f32,
        page_dim: PageDimensions,
    ) -> f32 {
        if page_dim.width <= 0.0 || page_dim.height <= 0.0 {
            return 1.0;
        }
        let padding = 32.0;
        let scale_x = (container_width - padding).max(100.0) / page_dim.width;
        let scale_y = (container_height - padding).max(100.0) / page_dim.height;
        scale_x.min(scale_y).clamp(MIN_ZOOM, MAX_ZOOM)
    }

    /// Zooms in by `ZOOM_STEP`.
    pub fn zoom_in(&mut self, center: Option<(f32, f32)>) {
        self.set_zoom(self.zoom_level * ZOOM_STEP, center);
    }

    /// Zooms out by `ZOOM_STEP`.
    pub fn zoom_out(&mut self, center: Option<(f32, f32)>) {
        self.set_zoom(self.zoom_level / ZOOM_STEP, center);
    }

    /// Resets zoom level to 100% (1.0).
    pub fn reset_zoom(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = (0.0, 0.0);
    }

    /// Adjusts zoom to fit the current page width.
    pub fn fit_to_width(&mut self) {
        if let Some((container_w, _)) = self.container_size {
            if let Some(dim) = self
                .document
                .as_ref()
                .and_then(|d| d.page_size(self.current_page))
            {
                self.zoom_level = self.compute_fit_to_width_zoom(container_w, dim.width);
                self.pan_offset = (0.0, 0.0);
            }
        }
    }

    /// Adjusts zoom to fit the current entire page in view.
    pub fn fit_to_page(&mut self) {
        if let Some((container_w, container_h)) = self.container_size {
            if let Some(dim) = self
                .document
                .as_ref()
                .and_then(|d| d.page_size(self.current_page))
            {
                self.zoom_level = self.compute_fit_to_page_zoom(container_w, container_h, dim);
                self.pan_offset = (0.0, 0.0);
            }
        }
    }

    /// Sets zoom with mouse focal point anchoring.
    pub fn set_zoom(&mut self, new_zoom: f32, focal_center: Option<(f32, f32)>) {
        let old_zoom = self.zoom_level;
        let clamped = new_zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        if (clamped - old_zoom).abs() < f32::EPSILON {
            return;
        }

        self.zoom_level = clamped;

        if let Some((center_x, center_y)) = focal_center {
            if let Some((cont_w, cont_h)) = self.container_size {
                let rel_x = center_x - cont_w / 2.0;
                let rel_y = center_y - cont_h / 2.0;

                let mouse_offset_x = rel_x - self.pan_offset.0;
                let mouse_offset_y = rel_y - self.pan_offset.1;

                let ratio = self.zoom_level / old_zoom;
                self.pan_offset.0 += mouse_offset_x * (1.0 - ratio);
                self.pan_offset.1 += mouse_offset_y * (1.0 - ratio);
            }
        }
    }

    /// Navigates to the next page.
    pub fn next_page(&mut self) -> bool {
        if let Some(ref doc) = self.document {
            if self.current_page + 1 < doc.total_pages() {
                self.current_page += 1;
                return true;
            }
        }
        false
    }

    /// Navigates to the previous page.
    pub fn previous_page(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            return true;
        }
        false
    }

    /// Jumps directly to target page index (0-indexed).
    pub fn go_to_page(&mut self, page_index: usize) -> bool {
        if let Some(ref doc) = self.document {
            if page_index < doc.total_pages() {
                self.current_page = page_index;
                return true;
            }
        }
        false
    }

    /// Returns true if dark mode luminosity mapping is active.
    pub fn dark_mode(&self) -> bool {
        self.dark_mode
    }

    /// Sets dark mode luminosity mapping.
    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        self.dark_mode = dark_mode;
    }

    /// Toggles smart dark mode luminosity mapping.
    pub fn toggle_dark_mode(&mut self) {
        self.dark_mode = !self.dark_mode;
    }

    /// Renders or fetches cached bitmap for a page.
    pub fn get_or_render_page(&mut self, page_index: usize) -> Result<RenderedPage> {
        let key = CacheKey::new(page_index, self.zoom_level, self.dark_mode);
        if let Some(cached) = self.cache.get(&key) {
            return Ok(cached);
        }

        let options = RasterizerOptions {
            target_dpi: self.settings.target_dpi,
            zoom_factor: self.zoom_level,
            dark_mode: self.dark_mode,
            saturation_threshold: self.settings.saturation_threshold,
        };

        let rendered = if let Some(ref raw) = self.raw_bytes {
            self.engine
                .render_page_from_bytes(raw, page_index, options)?
        } else {
            let dim = self
                .document
                .as_ref()
                .and_then(|d| d.page_size(page_index))
                .unwrap_or(PageDimensions::new(612.0, 792.0));
            crate::rasterizer::PageRasterizer::render_mock_page(page_index, dim, options)?
        };

        self.cache.insert(key, rendered.clone());
        Ok(rendered)
    }

    /// Returns the total number of pages in the loaded document.
    pub fn total_pages(&self) -> usize {
        self.document.as_ref().map(|d| d.total_pages()).unwrap_or(0)
    }

    /// Returns the active 1-indexed page number for UI display.
    pub fn display_page_number(&self) -> usize {
        self.current_page + 1
    }

    /// Returns the active page layout mode.
    pub fn layout_mode(&self) -> PageLayoutMode {
        self.layout_mode
    }

    /// Sets the active page layout mode.
    pub fn set_layout_mode(&mut self, layout_mode: PageLayoutMode) {
        self.layout_mode = layout_mode;
    }

    /// Handles mouse down event for interactive dragging/panning.
    pub fn handle_mouse_down(&mut self, position: (f32, f32)) {
        self.last_mouse_pos = Some(position);
    }

    /// Handles mouse up event to end dragging.
    pub fn handle_mouse_up(&mut self) {
        self.last_mouse_pos = None;
    }

    /// Handles mouse move during active drag to update pan offsets.
    pub fn handle_mouse_move(&mut self, position: (f32, f32)) {
        if let Some(last) = self.last_mouse_pos {
            let dx = position.0 - last.0;
            let dy = position.1 - last.1;
            self.pan_offset.0 += dx;
            self.pan_offset.1 += dy;
            self.last_mouse_pos = Some(position);
        }
    }

    /// Returns true if a drag/pan operation is currently active.
    pub fn is_dragging(&self) -> bool {
        self.last_mouse_pos.is_some()
    }

    /// Returns the current zoom level as percentage integer (e.g. 100 for 1.0x).
    pub fn zoom_percentage(&self) -> u32 {
        (self.zoom_level * 100.0).round().max(1.0) as u32
    }

    /// Returns the active 0-indexed page index.
    pub fn current_page(&self) -> usize {
        self.current_page
    }

    /// Returns the current zoom level factor.
    pub fn zoom_level(&self) -> f32 {
        self.zoom_level
    }

    /// Returns the current pan offset (x, y).
    pub fn pan_offset(&self) -> (f32, f32) {
        self.pan_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_VALID_PDF: &[u8] = b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\nxref\n0 4\n0000000000 65535 f \n0000000010 00000 n \n0000000060 00000 n \n0000000117 00000 n \ntrailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n185\n%%EOF\n";

    #[test]
    fn test_view_zoom_clamping() {
        let mut view = PdfView::new(PdfViewerSettings::default());
        view.set_zoom(0.01, None);
        assert_eq!(view.zoom_level, MIN_ZOOM);

        view.set_zoom(100.0, None);
        assert_eq!(view.zoom_level, MAX_ZOOM);
    }

    #[test]
    fn test_view_page_navigation() {
        let mut view = PdfView::new(PdfViewerSettings::default());
        view.load_from_bytes(SAMPLE_VALID_PDF.to_vec(), None)
            .expect("Load");

        assert_eq!(view.current_page, 0);
        assert_eq!(view.display_page_number(), 1);
        assert!(!view.previous_page());
        assert!(!view.next_page()); // Only 1 page in dummy doc
    }

    #[test]
    fn test_view_fit_to_width_calculation() {
        let view = PdfView::new(PdfViewerSettings::default());
        // Container width 832, page width 800 -> available 800 -> zoom = 1.0
        let zoom = view.compute_fit_to_width_zoom(832.0, 800.0);
        assert!((zoom - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_view_cache_and_render_cycle() {
        let mut view = PdfView::new(PdfViewerSettings::default());
        view.load_from_bytes(SAMPLE_VALID_PDF.to_vec(), None)
            .expect("Load");

        let page = view.get_or_render_page(0).expect("Render page");
        assert!(page.width > 0);
        assert_eq!(view.cache.len(), 1);

        // Fetching again should hit cache
        let cached = view.get_or_render_page(0).expect("Fetch cached");
        assert_eq!(cached.width, page.width);
    }

    #[test]
    fn test_view_mouse_drag_and_layout_mode() {
        let mut view = PdfView::new(PdfViewerSettings::default());
        assert_eq!(view.layout_mode(), PageLayoutMode::Continuous);
        view.set_layout_mode(PageLayoutMode::SinglePage);
        assert_eq!(view.layout_mode(), PageLayoutMode::SinglePage);

        assert!(!view.is_dragging());
        view.handle_mouse_down((100.0, 100.0));
        assert!(view.is_dragging());

        view.handle_mouse_move((120.0, 150.0));
        assert_eq!(view.pan_offset, (20.0, 50.0));

        view.handle_mouse_up();
        assert!(!view.is_dragging());
    }
}

//! High-level Pdfium engine abstraction and dynamic library loader.
//!
//! Provides thread-safe document rendering via Google Pdfium C++ engine,
//! with automated fallback and library path discovery.

use anyhow::{Context as _, Result};
use parking_lot::Mutex;
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::cache::RenderedPage;
use crate::document::{PageDimensions, PdfDocument};
use crate::rasterizer::{LuminosityToneMapper, PageRasterizer, RasterizerOptions};

/// Internal wrapper granting thread-safe synchronization to Pdfium handle.
struct SerializedPdfium(Option<Pdfium>);

// SAFETY: All calls to the internal Pdfium C++ FFI handle are strictly guarded
// by a Mutual Exclusion (Mutex) lock, guaranteeing serialized single-thread
// execution across asynchronous worker threads without race conditions.
unsafe impl Send for SerializedPdfium {}

// SAFETY: All calls to the internal Pdfium C++ FFI handle are strictly guarded
// by a Mutual Exclusion (Mutex) lock, guaranteeing synchronized multi-thread access.
unsafe impl Sync for SerializedPdfium {}

use std::sync::OnceLock;

static GLOBAL_PDFIUM: OnceLock<Arc<Mutex<SerializedPdfium>>> = OnceLock::new();

/// Maximum dimension (width or height) allowed when rasterizing a PDF page,
/// preventing out-of-memory denial of service attacks from malicious documents.
pub const MAX_PAGE_DIMENSION: f32 = 8192.0;

/// Thread-safe wrapper around a Pdfium instance.
#[derive(Clone)]
pub struct PdfiumEngine {
    inner: Arc<Mutex<SerializedPdfium>>,
}

impl Default for PdfiumEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfiumEngine {
    /// Creates a new engine handle, sharing the process-level singleton instance.
    pub fn new() -> Self {
        let inner = GLOBAL_PDFIUM
            .get_or_init(|| Arc::new(Mutex::new(SerializedPdfium(Self::init_pdfium()))))
            .clone();

        Self { inner }
    }

    /// Attempts to bind to dynamic `libpdfium.so`, `libpdfium.dylib`, or `pdfium.dll`.
    fn init_pdfium() -> Option<Pdfium> {
        // 1. Try explicit environment variable PDFIUM_LIB_PATH
        if let Ok(env_path) = std::env::var("PDFIUM_LIB_PATH") {
            let path = Path::new(&env_path);
            if path.exists() {
                if let Ok(bindings) = Pdfium::bind_to_library(path) {
                    log::info!("Successfully bound to Pdfium from environment at {}", env_path);
                    return Some(Pdfium::new(bindings));
                }
            }
        }

        // 2. Try standard system library search path
        if let Ok(bindings) = Pdfium::bind_to_system_library() {
            log::info!("Successfully initialized Pdfium from system dynamic library");
            return Some(Pdfium::new(bindings));
        }

        // 3. Try common Linux/macOS shared object locations
        let mut common_paths: Vec<PathBuf> = vec![
            PathBuf::from("lib/libpdfium.so"),
            PathBuf::from("./lib/libpdfium.so"),
            PathBuf::from("../lib/libpdfium.so"),
            PathBuf::from("../../lib/libpdfium.so"),
        ];

        if let Ok(home_dir) = std::env::var("HOME") {
            if !home_dir.trim().is_empty() {
                common_paths.push(PathBuf::from(format!("{home_dir}/.local/lib/libpdfium.so")));
                common_paths.push(PathBuf::from(format!("{home_dir}/.local/lib/libpdfium.dylib")));
            }
        }

        common_paths.extend([
            PathBuf::from("/usr/lib/libpdfium.so"),
            PathBuf::from("/usr/lib64/libpdfium.so"),
            PathBuf::from("/usr/local/lib/libpdfium.so"),
            PathBuf::from("/opt/homebrew/lib/libpdfium.dylib"),
            PathBuf::from("/usr/local/lib/libpdfium.dylib"),
        ]);

        for path in &common_paths {
            if path.exists() {
                if let Ok(bindings) = Pdfium::bind_to_library(path) {
                    log::info!("Successfully bound to Pdfium at {}", path.display());
                    return Some(Pdfium::new(bindings));
                }
            }
        }

        log::warn!("Dynamic libpdfium not found in standard system paths; engine will use synthetic fallback");
        None
    }

    /// True if native Pdfium C++ bindings are actively loaded.
    pub fn is_native_available(&self) -> bool {
        self.inner.lock().0.is_some()
    }

    /// Loads a PDF from disk path and returns document metadata.
    pub fn load_document_from_path(&self, path: &Path) -> Result<PdfDocument> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read PDF file at {:?}", path))?;
        self.load_document_from_bytes(&bytes, Some(path.to_path_buf()))
    }

    /// Loads a PDF from in-memory byte slice.
    pub fn load_document_from_bytes(
        &self,
        bytes: &[u8],
        path: Option<PathBuf>,
    ) -> Result<PdfDocument> {
        let guard = self.inner.lock();
        if let Some(ref pdfium) = guard.0 {
            let doc = pdfium
                .load_pdf_from_byte_slice(bytes, None)
                .context("Pdfium failed to parse PDF document byte slice")?;

            let mut pages = Vec::new();
            for page in doc.pages().iter() {
                let width = page.width().value;
                let height = page.height().value;
                pages.push(PageDimensions::new(width, height));
            }

            let title = doc
                .metadata()
                .get(PdfDocumentMetadataTagType::Title)
                .map(|s| s.value().to_string());

            Ok(PdfDocument::new(path, pages, title))
        } else {
            // Fallback scanner if native library is absent
            PdfDocument::from_bytes(bytes, path)
        }
    }

    /// Rasterizes a specific page from file bytes into an RGBA frame buffer.
    pub fn render_page_from_bytes(
        &self,
        bytes: &[u8],
        page_index: usize,
        options: RasterizerOptions,
    ) -> Result<RenderedPage> {
        let guard = self.inner.lock();
        if let Some(ref pdfium) = guard.0 {
            let doc = pdfium
                .load_pdf_from_byte_slice(bytes, None)
                .context("Failed to load PDF in rasterizer")?;

            let page = doc
                .pages()
                .get(page_index as u16)
                .context("Requested page index out of bounds")?;

            let target_width =
                (page.width().value * (options.target_dpi / 72.0) * options.zoom_factor)
                    .round()
                    .clamp(1.0, MAX_PAGE_DIMENSION) as i32;
            let target_height =
                (page.height().value * (options.target_dpi / 72.0) * options.zoom_factor)
                    .round()
                    .clamp(1.0, MAX_PAGE_DIMENSION) as i32;

            let render_config = PdfRenderConfig::new()
                .set_target_width(target_width)
                .set_target_height(target_height)
                .render_form_data(true)
                .render_annotations(true);

            let bitmap = page
                .render_with_config(&render_config)
                .context("Pdfium page render call failed")?;

            let mut rgba_buffer = bitmap.as_image().to_rgba8().into_raw();

            if options.dark_mode {
                LuminosityToneMapper::apply(&mut rgba_buffer, options.saturation_threshold);
            }

            Ok(RenderedPage::new(
                page_index,
                target_width as u32,
                target_height as u32,
                options.zoom_factor,
                options.dark_mode,
                rgba_buffer,
            ))
        } else {
            // Fallback mock rendering when native engine binary is not linked
            let dim = PageDimensions::new(612.0, 792.0);
            crate::rasterizer::PageRasterizer::render_mock_page(page_index, dim, options)
        }
    }
    /// Rasterizes all pages from file bytes into a continuous vertical layout,
    /// stitching them together with a configurable page spacing and margin.
    pub fn render_document_from_bytes(
        &self,
        bytes: &[u8],
        options: RasterizerOptions,
        page_gap: u32,
    ) -> Result<RenderedPage> {
        let guard = self.inner.lock();
        if let Some(ref pdfium) = guard.0 {
            let doc = pdfium
                .load_pdf_from_byte_slice(bytes, None)
                .context("Failed to load PDF in document rasterizer")?;

            let total_pages = doc.pages().len() as usize;
            if total_pages == 0 {
                anyhow::bail!("PDF document contains no pages");
            }

            let mut rendered_pages = Vec::with_capacity(total_pages);
            for (idx, page) in doc.pages().iter().enumerate() {
                let target_width =
                    (page.width().value * (options.target_dpi / 72.0) * options.zoom_factor)
                        .round()
                        .clamp(1.0, MAX_PAGE_DIMENSION) as i32;
                let target_height =
                    (page.height().value * (options.target_dpi / 72.0) * options.zoom_factor)
                        .round()
                        .clamp(1.0, MAX_PAGE_DIMENSION) as i32;

                let render_config = PdfRenderConfig::new()
                    .set_target_width(target_width)
                    .set_target_height(target_height)
                    .render_form_data(true)
                    .render_annotations(true);

                let bitmap = page
                    .render_with_config(&render_config)
                    .with_context(|| format!("Pdfium failed to render page {}", idx))?;

                let mut rgba_buffer = bitmap.as_image().to_rgba8().into_raw();

                if options.dark_mode {
                    LuminosityToneMapper::apply(&mut rgba_buffer, options.saturation_threshold);
                }

                rendered_pages.push((target_width as u32, target_height as u32, rgba_buffer));
            }

            let max_width = rendered_pages.iter().map(|(w, _, _)| *w).max().unwrap_or(1);
            let total_height: u32 = rendered_pages.iter().map(|(_, h, _)| *h).sum::<u32>()
                + ((total_pages as u32 - 1) * page_gap);

            let stride = (max_width as usize) * 4;
            let total_bytes = stride * (total_height as usize);

            let bg_color = if options.dark_mode {
                [24u8, 24, 37, 255]
            } else {
                [230u8, 233, 239, 255]
            };

            let mut composite = vec![0u8; total_bytes];
            for chunk in composite.as_chunks_mut::<4>().0 {
                *chunk = bg_color;
            }

            let mut y_offset = 0u32;
            for (w, h, page_buf) in rendered_pages {
                let x_offset = ((max_width - w) / 2) as usize;
                let page_stride = (w as usize) * 4;

                for y in 0..h {
                    let src_start = (y as usize) * page_stride;
                    let src_end = src_start + page_stride;
                    let src_slice = &page_buf[src_start..src_end];

                    let dst_y = (y_offset + y) as usize;
                    let dst_start = (dst_y * (max_width as usize) + x_offset) * 4;
                    let dst_end = dst_start + page_stride;

                    composite[dst_start..dst_end].copy_from_slice(src_slice);
                }

                y_offset += h + page_gap;
            }

            Ok(RenderedPage::new(
                0,
                max_width,
                total_height,
                options.zoom_factor,
                options.dark_mode,
                composite,
            ))
        } else {
            let doc = PdfDocument::from_bytes(bytes, None)?;
            let total_pages = doc.total_pages();
            if total_pages == 0 {
                anyhow::bail!("PDF document contains no pages");
            }
            if total_pages == 1 {
                let dim = doc.page_size(0).unwrap_or(PageDimensions::new(612.0, 792.0));
                return PageRasterizer::render_mock_page(0, dim, options);
            }

            let mut rendered_pages = Vec::with_capacity(total_pages);
            for i in 0..total_pages {
                let dim = doc.page_size(i).unwrap_or(PageDimensions::new(612.0, 792.0));
                let page = PageRasterizer::render_mock_page(i, dim, options)?;
                rendered_pages.push((page.width, page.height, page.rgba_buffer.as_ref().clone()));
            }

            let max_width = rendered_pages.iter().map(|(w, _, _)| *w).max().unwrap_or(1);
            let total_height: u32 = rendered_pages.iter().map(|(_, h, _)| *h).sum::<u32>()
                + ((total_pages as u32 - 1) * page_gap);

            let stride = (max_width as usize) * 4;
            let total_bytes = stride * (total_height as usize);

            let bg_color = if options.dark_mode {
                [24u8, 24, 37, 255]
            } else {
                [230u8, 233, 239, 255]
            };

            let mut composite = vec![0u8; total_bytes];
            for chunk in composite.as_chunks_mut::<4>().0 {
                *chunk = bg_color;
            }

            let mut y_offset = 0u32;
            for (w, h, page_buf) in rendered_pages {
                let x_offset = ((max_width - w) / 2) as usize;
                let page_stride = (w as usize) * 4;

                for y in 0..h {
                    let src_start = (y as usize) * page_stride;
                    let src_end = src_start + page_stride;
                    let src_slice = &page_buf[src_start..src_end];

                    let dst_y = (y_offset + y) as usize;
                    let dst_start = (dst_y * (max_width as usize) + x_offset) * 4;
                    let dst_end = dst_start + page_stride;

                    composite[dst_start..dst_end].copy_from_slice(src_slice);
                }

                y_offset += h + page_gap;
            }

            Ok(RenderedPage::new(
                0,
                max_width,
                total_height,
                options.zoom_factor,
                options.dark_mode,
                composite,
            ))
        }
    }

    /// Extracts text from a specific page (0-indexed) or the entire document if page_index is None.
    pub fn extract_text_from_bytes(
        &self,
        bytes: &[u8],
        page_index: Option<usize>,
    ) -> Result<String> {
        let guard = self.inner.lock();
        if let Some(ref pdfium) = guard.0 {
            let doc = pdfium
                .load_pdf_from_byte_slice(bytes, None)
                .context("Failed to load PDF for text extraction")?;

            if let Some(idx) = page_index {
                let page = doc
                    .pages()
                    .get(idx as u16)
                    .context("Requested page index out of bounds")?;
                let text_page = page.text().context("Failed to load page text")?;
                Ok(text_page.all())
            } else {
                let mut full_text = String::new();
                for (idx, page) in doc.pages().iter().enumerate() {
                    if let Ok(text_page) = page.text() {
                        let page_text = text_page.all();
                        if !page_text.is_empty() {
                            if idx > 0 {
                                full_text.push_str("\n\n--- Page ");
                                full_text.push_str(&(idx + 1).to_string());
                                full_text.push_str(" ---\n\n");
                            }
                            full_text.push_str(&page_text);
                        }
                    }
                }
                Ok(full_text)
            }
        } else {
            Ok(String::new())
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PdfLinkAnnotation {
    pub url: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PdfTextSegment {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Default)]
pub struct PdfPageDetails {
    pub page_index: usize,
    pub width_pt: f32,
    pub height_pt: f32,
    pub text: String,
    pub links: Vec<PdfLinkAnnotation>,
    pub text_segments: Vec<PdfTextSegment>,
}

#[derive(Debug, Clone, Default)]
pub struct PdfDocumentDetails {
    pub total_pages: usize,
    pub full_text: String,
    pub pages: Vec<PdfPageDetails>,
}

#[derive(Debug, Clone)]
pub struct PdfPageRenderResult {
    pub page_index: usize,
    pub width: u32,
    pub height: u32,
    pub rgba_buffer: Vec<u8>,
    pub text: String,
    pub text_segments: Vec<PdfTextSegment>,
    pub links: Vec<PdfLinkAnnotation>,
}

#[derive(Debug, Clone, Default)]
pub struct PdfDocumentRenderResult {
    pub total_pages: usize,
    pub full_text: String,
    pub pages: Vec<PdfPageRenderResult>,
}

fn extract_page_text_and_links(
    page: &pdfium_render::prelude::PdfPage,
    idx: usize,
    full_text: &mut String,
) -> (String, Vec<PdfTextSegment>, Vec<PdfLinkAnnotation>) {
    let page_w = page.width().value;
    let page_h = page.height().value;
    let mut page_text = String::new();
    let mut segments = Vec::new();
    let mut links = Vec::new();

    // 1. Text and Text Segments
    if let Ok(text_page) = page.text() {
        page_text = text_page.all();
        if !page_text.is_empty() {
            if idx > 0 {
                full_text.push_str("\n\n--- Page ");
                full_text.push_str(&(idx + 1).to_string());
                full_text.push_str(" ---\n\n");
            }
            full_text.push_str(&page_text);
        }

        let seg_coll = text_page.segments();
        for seg in seg_coll.iter() {
            let seg_str = seg.text();
            let seg_trimmed = seg_str.trim();
            if seg_trimmed.is_empty() {
                continue;
            }

            if !seg_trimmed.contains(' ') {
                let bounds = seg.bounds();
                let left = bounds.left().value.min(bounds.right().value);
                let right = bounds.left().value.max(bounds.right().value);
                let bottom = bounds.bottom().value.min(bounds.top().value);
                let top = bounds.bottom().value.max(bounds.top().value);

                let norm_x = if page_w > 0.0 { (left / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                let norm_y = if page_h > 0.0 { ((page_h - top) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };
                let norm_w = if page_w > 0.0 { ((right - left) / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                let norm_h = if page_h > 0.0 { ((top - bottom) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };

                if norm_w > 0.0 && norm_h > 0.0 {
                    segments.push(PdfTextSegment {
                        text: seg_trimmed.to_string(),
                        x: norm_x,
                        y: norm_y,
                        width: norm_w,
                        height: norm_h,
                    });
                }
                continue;
            }

            if let Ok(chars) = seg.chars() {
                let mut word_text = String::new();
                let mut min_left = f32::MAX;
                let mut max_right = f32::MIN;
                let mut min_bottom = f32::MAX;
                let mut max_top = f32::MIN;
                let mut has_word_char = false;

                for ch in chars.iter() {
                    let u_char = ch.unicode_char();
                    let is_ws = u_char.map(|c| c.is_whitespace()).unwrap_or(false);

                    if is_ws {
                        if has_word_char && !word_text.is_empty() {
                            let norm_x = if page_w > 0.0 { (min_left / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                            let norm_y = if page_h > 0.0 { ((page_h - max_top) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };
                            let norm_w = if page_w > 0.0 { ((max_right - min_left) / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                            let norm_h = if page_h > 0.0 { ((max_top - min_bottom) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };

                            if norm_w > 0.0 && norm_h > 0.0 {
                                segments.push(PdfTextSegment {
                                    text: std::mem::take(&mut word_text),
                                    x: norm_x,
                                    y: norm_y,
                                    width: norm_w,
                                    height: norm_h,
                                });
                            }
                            word_text.clear();
                            min_left = f32::MAX;
                            max_right = f32::MIN;
                            min_bottom = f32::MAX;
                            max_top = f32::MIN;
                            has_word_char = false;
                        }
                    } else if let Some(c) = u_char {
                        word_text.push(c);
                        if let Ok(bounds) = ch.loose_bounds().or_else(|_| ch.tight_bounds()) {
                            let l = bounds.left().value.min(bounds.right().value);
                            let r = bounds.left().value.max(bounds.right().value);
                            let b = bounds.bottom().value.min(bounds.top().value);
                            let t = bounds.bottom().value.max(bounds.top().value);

                            min_left = min_left.min(l);
                            max_right = max_right.max(r);
                            min_bottom = min_bottom.min(b);
                            max_top = max_top.max(t);
                            has_word_char = true;
                        }
                    }
                }

                if has_word_char && !word_text.is_empty() {
                    let norm_x = if page_w > 0.0 { (min_left / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                    let norm_y = if page_h > 0.0 { ((page_h - max_top) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };
                    let norm_w = if page_w > 0.0 { ((max_right - min_left) / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                    let norm_h = if page_h > 0.0 { ((max_top - min_bottom) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };

                    if norm_w > 0.0 && norm_h > 0.0 {
                        segments.push(PdfTextSegment {
                            text: word_text,
                            x: norm_x,
                            y: norm_y,
                            width: norm_w,
                            height: norm_h,
                        });
                    }
                }
            } else {
                let bounds = seg.bounds();
                let left = bounds.left().value.min(bounds.right().value);
                let right = bounds.left().value.max(bounds.right().value);
                let bottom = bounds.bottom().value.min(bounds.top().value);
                let top = bounds.bottom().value.max(bounds.top().value);

                let norm_x = if page_w > 0.0 { (left / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                let norm_y = if page_h > 0.0 { ((page_h - top) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };
                let norm_w = if page_w > 0.0 { ((right - left) / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                let norm_h = if page_h > 0.0 { ((top - bottom) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };

                segments.push(PdfTextSegment {
                    text: seg_trimmed.to_string(),
                    x: norm_x,
                    y: norm_y,
                    width: norm_w,
                    height: norm_h,
                });
            }
        }
    }

    // 2. Links from page.links()
    for link in page.links().iter() {
        if let Some(action) = link.action() {
            if let PdfAction::Uri(uri_action) = action {
                if let Ok(url) = uri_action.uri() {
                    let url_clean = url.trim().to_string();
                    if !url_clean.is_empty() {
                        if let Ok(rect) = link.rect() {
                            let left = rect.left().value.min(rect.right().value);
                            let right = rect.left().value.max(rect.right().value);
                            let bottom = rect.bottom().value.min(rect.top().value);
                            let top = rect.bottom().value.max(rect.top().value);

                            let norm_x = if page_w > 0.0 { (left / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                            let norm_y = if page_h > 0.0 { ((page_h - top) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };
                            let norm_w = if page_w > 0.0 { ((right - left) / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                            let norm_h = if page_h > 0.0 { ((top - bottom) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };

                            links.push(PdfLinkAnnotation {
                                url: url_clean,
                                x: norm_x,
                                y: norm_y,
                                width: norm_w,
                                height: norm_h,
                            });
                        }
                    }
                }
            }
        }
    }

    // 3. Links from page.annotations()
    for annot in page.annotations().iter() {
        if let Some(link_annot) = annot.as_link_annotation() {
            if let Ok(link) = link_annot.link() {
                if let Some(action) = link.action() {
                    if let PdfAction::Uri(uri_action) = action {
                        if let Ok(url) = uri_action.uri() {
                            let url_clean = url.trim().to_string();
                            if !url_clean.is_empty() {
                                if let Ok(rect) = link.rect() {
                                    let left = rect.left().value.min(rect.right().value);
                                    let right = rect.left().value.max(rect.right().value);
                                    let bottom = rect.bottom().value.min(rect.top().value);
                                    let top = rect.bottom().value.max(rect.top().value);

                                    let norm_x = if page_w > 0.0 { (left / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                                    let norm_y = if page_h > 0.0 { ((page_h - top) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };
                                    let norm_w = if page_w > 0.0 { ((right - left) / page_w).clamp(0.0, 1.0) as f32 } else { 0.0 };
                                    let norm_h = if page_h > 0.0 { ((top - bottom) / page_h).clamp(0.0, 1.0) as f32 } else { 0.0 };

                                    let exists = links.iter().any(|l| l.url == url_clean && (l.x - norm_x).abs() < 0.01 && (l.y - norm_y).abs() < 0.01);
                                    if !exists {
                                        links.push(PdfLinkAnnotation {
                                            url: url_clean,
                                            x: norm_x,
                                            y: norm_y,
                                            width: norm_w,
                                            height: norm_h,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 4. Text URLs from segments (heuristic for raw text URLs)
    for seg in &segments {
        let text = seg.text.trim();
        if text.starts_with("http://") || text.starts_with("https://") || text.starts_with("www.") {
            let url = if text.starts_with("www.") {
                format!("https://{}", text)
            } else {
                text.to_string()
            };
            let exists = links.iter().any(|l| (l.x - seg.x).abs() < 0.02 && (l.y - seg.y).abs() < 0.02);
            if !exists {
                links.push(PdfLinkAnnotation {
                    url,
                    x: seg.x,
                    y: seg.y,
                    width: seg.width,
                    height: seg.height,
                });
            }
        }
    }

    (page_text, segments, links)
}

impl PdfiumEngine {
    /// Extracts full document details including text, text segments with bounding boxes, and links.
    pub fn extract_document_details(&self, bytes: &[u8]) -> Result<PdfDocumentDetails> {
        let guard = self.inner.lock();
        if let Some(ref pdfium) = guard.0 {
            let doc = pdfium
                .load_pdf_from_byte_slice(bytes, None)
                .context("Failed to load PDF for metadata extraction")?;

            let mut pages_details = Vec::new();
            let mut full_text = String::new();

            for (idx, page) in doc.pages().iter().enumerate() {
                let page_w = page.width().value;
                let page_h = page.height().value;
                let (page_text, segments, links) = extract_page_text_and_links(&page, idx, &mut full_text);

                pages_details.push(PdfPageDetails {
                    page_index: idx,
                    width_pt: page_w,
                    height_pt: page_h,
                    text: page_text,
                    links,
                    text_segments: segments,
                });
            }

            Ok(PdfDocumentDetails {
                total_pages: pages_details.len(),
                full_text,
                pages: pages_details,
            })
        } else {
            Ok(PdfDocumentDetails::default())
        }
    }

    /// Renders all pages and extracts text/links in a single fast pass over the loaded document.
    pub fn render_and_extract_document_from_bytes(
        &self,
        bytes: &[u8],
        options: RasterizerOptions,
    ) -> Result<PdfDocumentRenderResult> {
        let guard = self.inner.lock();
        if let Some(ref pdfium) = guard.0 {
            let doc = pdfium
                .load_pdf_from_byte_slice(bytes, None)
                .context("Failed to load PDF for render and extraction")?;

            let total_pages = doc.pages().len() as usize;
            let mut full_text = String::new();
            let mut rendered_pages = Vec::with_capacity(total_pages);

            for (idx, page) in doc.pages().iter().enumerate() {
                let page_w = page.width().value;
                let page_h = page.height().value;

                let target_width =
                    (page_w * (options.target_dpi / 72.0) * options.zoom_factor)
                        .round()
                        .clamp(1.0, MAX_PAGE_DIMENSION) as i32;
                let target_height =
                    (page_h * (options.target_dpi / 72.0) * options.zoom_factor)
                        .round()
                        .clamp(1.0, MAX_PAGE_DIMENSION) as i32;

                let render_config = PdfRenderConfig::new()
                    .set_target_width(target_width)
                    .set_target_height(target_height)
                    .render_form_data(true)
                    .render_annotations(true);

                let bitmap = page
                    .render_with_config(&render_config)
                    .with_context(|| format!("Pdfium failed to render page {}", idx))?;

                let mut rgba_buffer = bitmap.as_image().to_rgba8().into_raw();

                if options.dark_mode {
                    LuminosityToneMapper::apply(&mut rgba_buffer, options.saturation_threshold);
                }

                let (text, text_segments, links) = extract_page_text_and_links(&page, idx, &mut full_text);

                rendered_pages.push(PdfPageRenderResult {
                    page_index: idx,
                    width: target_width as u32,
                    height: target_height as u32,
                    rgba_buffer,
                    text,
                    text_segments,
                    links,
                });
            }

            Ok(PdfDocumentRenderResult {
                total_pages: rendered_pages.len(),
                full_text,
                pages: rendered_pages,
            })
        } else {
            let dim = PageDimensions::new(612.0, 792.0);
            let page = crate::rasterizer::PageRasterizer::render_mock_page(0, dim, options)?;
            Ok(PdfDocumentRenderResult {
                total_pages: 1,
                full_text: String::new(),
                pages: vec![PdfPageRenderResult {
                    page_index: 0,
                    width: page.width,
                    height: page.height,
                    rgba_buffer: page.rgba_buffer.as_ref().clone(),
                    text: String::new(),
                    text_segments: Vec::new(),
                    links: Vec::new(),
                }],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization_safety() {
        let engine = PdfiumEngine::new();
        // Even without system libpdfium installed, instance must initialize safely
        let is_avail = engine.is_native_available();
        let _ = is_avail; // No panic
    }

    #[test]
    fn test_fallback_rendering_when_native_absent() {
        let engine = PdfiumEngine {
            inner: Arc::new(Mutex::new(SerializedPdfium(None))),
        };

        let dummy_pdf = b"%PDF-1.7\nSample";
        let doc = engine
            .load_document_from_bytes(dummy_pdf, None)
            .expect("Fallback parse failed");
        assert_eq!(doc.total_pages(), 1);

        let opts = RasterizerOptions::default();
        let page = engine
            .render_page_from_bytes(dummy_pdf, 0, opts)
            .expect("Fallback render failed");
        assert!(page.width > 0);
        assert!(page.height > 0);
        assert_eq!(
            page.rgba_buffer.len(),
            (page.width * page.height * 4) as usize
        );
    }

    #[test]
    fn test_render_document_multi_page() {
        let engine = PdfiumEngine {
            inner: Arc::new(Mutex::new(SerializedPdfium(None))),
        };

        let dummy_pdf = b"%PDF-1.7\nSample";
        let opts = RasterizerOptions::default();
        let rendered = engine
            .render_document_from_bytes(dummy_pdf, opts, 20)
            .expect("Render document failed");
        assert!(rendered.width > 0);
        assert!(rendered.height > 0);
        assert_eq!(
            rendered.rgba_buffer.len(),
            (rendered.width * rendered.height * 4) as usize
        );
    }

    #[test]
    fn test_extract_document_details() {
        let engine = PdfiumEngine {
            inner: Arc::new(Mutex::new(SerializedPdfium(None))),
        };
        let dummy_pdf = b"%PDF-1.7\nSample";
        let details = engine.extract_document_details(dummy_pdf);
        assert!(details.is_ok());
        let doc = details.unwrap();
        assert_eq!(doc.total_pages, 0);
    }

    #[test]
    fn test_render_and_extract_document_from_bytes() {
        let engine = PdfiumEngine {
            inner: Arc::new(Mutex::new(SerializedPdfium(None))),
        };
        let dummy_pdf = b"%PDF-1.7\nSample";
        let opts = RasterizerOptions::default();
        let res = engine.render_and_extract_document_from_bytes(dummy_pdf, opts);
        assert!(res.is_ok());
        let doc = res.unwrap();
        assert_eq!(doc.total_pages, 1);
        assert_eq!(doc.pages.len(), 1);
        assert!(doc.pages[0].width > 0);
        assert!(doc.pages[0].height > 0);
    }

    #[test]
    fn test_max_page_dimension_bounds() {
        assert_eq!(MAX_PAGE_DIMENSION, 8192.0);
        // Verify clamping logic for extreme/malicious dimensions
        let huge_dim: f32 = 100_000.0;
        let clamped = huge_dim.clamp(1.0, MAX_PAGE_DIMENSION);
        assert_eq!(clamped, 8192.0);

        let tiny_dim: f32 = -50.0;
        let clamped_tiny = tiny_dim.clamp(1.0, MAX_PAGE_DIMENSION);
        assert_eq!(clamped_tiny, 1.0);
    }
}

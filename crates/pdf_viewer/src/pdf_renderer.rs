use std::sync::Arc;

use anyhow::Result;
use gpui::RenderImage;
use image::Frame;
use smallvec::SmallVec;

/// Holds a parsed PDF document using hayro (rendering) and rpdfium (text extraction).
pub struct PdfDocument {
    hayro_pdf: hayro_syntax::Pdf,
    #[allow(dead_code)]
    rpdfium_library: rpdfium::ArcLibrary,
    rpdfium_document: Option<rpdfium::ArcDocument>,
}

// hayro_syntax::Pdf is not Send by default, but we need it for background rendering.
// SAFETY: We only access the Pdf from one thread at a time (render tasks are sequential per page).
unsafe impl Send for PdfDocument {}
unsafe impl Sync for PdfDocument {}

#[derive(Debug)]
pub enum PdfLoadError {
    ParseError(String),
    PasswordProtected,
}

impl std::fmt::Display for PdfLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfLoadError::ParseError(message) => write!(f, "Failed to parse PDF: {message}"),
            PdfLoadError::PasswordProtected => write!(f, "This PDF is password-protected"),
        }
    }
}

impl std::error::Error for PdfLoadError {}

impl PdfDocument {
    pub fn open(data: Vec<u8>) -> std::result::Result<Self, PdfLoadError> {
        // Parse with hayro for rendering
        let hayro_pdf = hayro_syntax::Pdf::new(Arc::new(data.clone()))
            .map_err(|error| PdfLoadError::ParseError(format!("{error:?}")))?;

        // Parse with rpdfium for text extraction (best-effort)
        let rpdfium_library = rpdfium::ArcLibrary::new();
        let rpdfium_document =
            rpdfium::ArcDocument::open(&rpdfium_library, data, &rpdfium::OpenOptions::default())
                .ok();

        Ok(Self {
            hayro_pdf,
            rpdfium_library,
            rpdfium_document,
        })
    }

    pub fn page_count(&self) -> u32 {
        self.hayro_pdf.pages().len() as u32
    }

    /// Page dimensions in points (72 points = 1 inch).
    pub fn page_dimensions(&self, page_index: u32) -> Result<(f64, f64)> {
        let pages = self.hayro_pdf.pages();
        let page = pages
            .get(page_index as usize)
            .ok_or_else(|| anyhow::anyhow!("page {page_index} out of range"))?;
        let (width, height) = page.render_dimensions();
        Ok((width as f64, height as f64))
    }

    /// Render a single page to RGBA pixels via hayro. CPU-intensive — call from background thread.
    pub fn render_page(&self, page_index: u32, dpi: f32) -> Result<RenderedPage> {
        let pages = self.hayro_pdf.pages();
        let page = pages
            .get(page_index as usize)
            .ok_or_else(|| anyhow::anyhow!("page {page_index} out of range"))?;

        let scale = dpi / 72.0;
        let render_settings = hayro::RenderSettings {
            x_scale: scale,
            y_scale: scale,
            ..hayro::RenderSettings::default()
        };
        let interp_settings = hayro_interpret::InterpreterSettings::default();

        let pixmap = hayro::render(page, &interp_settings, &render_settings);

        let width = pixmap.width() as u32;
        let height = pixmap.height() as u32;

        // Convert PremulRgba8 pixels to flat RGBA bytes
        let pixel_data = pixmap.data();
        let mut rgba_bytes: Vec<u8> = Vec::with_capacity(pixel_data.len() * 4);
        for pixel in pixel_data {
            rgba_bytes.push(pixel.r);
            rgba_bytes.push(pixel.g);
            rgba_bytes.push(pixel.b);
            rgba_bytes.push(pixel.a);
        }

        Ok(RenderedPage {
            width,
            height,
            data: rgba_bytes,
        })
    }

    /// Extract all text from a page via rpdfium.
    pub fn extract_page_text(&self, page_index: u32) -> Result<String> {
        let document = self
            .rpdfium_document
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("rpdfium document not available for text extraction"))?;
        let page = document.page(page_index)?;
        let text_page = page.text()?;
        Ok(text_page.all_page_text().to_string())
    }
}

/// A rendered page as raw RGBA pixels.
pub struct RenderedPage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl RenderedPage {
    pub fn into_render_image(self) -> Arc<RenderImage> {
        // Convert RGBA to BGRA (RenderImage expects BGRA format)
        let mut bgra_data = self.data;
        for pixel in bgra_data.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }

        let buffer = image::ImageBuffer::from_raw(self.width, self.height, bgra_data)
            .expect("pixel buffer dimensions should match");

        let frame = Frame::new(buffer);
        Arc::new(RenderImage::new(SmallVec::from_const([frame])))
    }
}

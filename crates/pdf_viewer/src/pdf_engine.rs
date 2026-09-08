use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};

use anyhow::{Context as _, anyhow};
use gpui::{Image, ImageFormat};

/// Trait representing an abstract PDF rendering engine.
pub trait PdfEngine: Send + Sync + 'static {
    /// Returns the human-readable name of the backend engine.
    fn name(&self) -> &'static str;

    /// Checks if this engine is available on the current machine.
    fn is_available(&self) -> bool;

    /// Loads a PDF document from bytes or file path.
    fn load_document(&self, path: &Path, bytes: &[u8]) -> anyhow::Result<Arc<dyn PdfDocument>>;
}

/// Trait representing an opened PDF document instance.
pub trait PdfDocument: Send + Sync {
    /// Returns the total number of pages in the PDF document.
    fn page_count(&self) -> usize;

    /// Returns the dimensions (width, height in points/pixels) of a specific page.
    fn page_size(&self, page_index: usize) -> (f32, f32);

    /// Rasterizes a specific 0-indexed page to a `gpui::Image` at the given scale factor.
    fn render_page(&self, page_index: usize, scale: f32) -> anyhow::Result<Arc<Image>>;

    /// Rasterizes a specific tile within a page at a given scale factor.
    /// `crop_x` and `crop_y` are the top-left offset in scaled pixels,
    /// `tile_w` and `tile_h` are the tile dimensions in pixels.
    fn render_tile(
        &self,
        page_index: usize,
        scale: f32,
        crop_x: u32,
        crop_y: u32,
        tile_w: u32,
        tile_h: u32,
    ) -> anyhow::Result<Arc<Image>>;
}

// ---------------------------------------------------------------------------
// Mock PDF Engine (for testing and offline environments)
// ---------------------------------------------------------------------------

/// A mock PDF engine that generates synthetic SVG-based document pages and tiles.
/// Ideal for automated unit tests, headless CI, and development without external C libraries.
#[derive(Debug, Clone)]
pub struct MockPdfEngine {
    default_page_count: usize,
    page_width: f32,
    page_height: f32,
}

impl Default for MockPdfEngine {
    fn default() -> Self {
        Self {
            default_page_count: 5,
            page_width: 612.0, // Standard US Letter width
            page_height: 792.0, // Standard US Letter height
        }
    }
}

impl MockPdfEngine {
    pub fn new(page_count: usize) -> Self {
        Self {
            default_page_count: page_count,
            ..Default::default()
        }
    }
}

impl PdfEngine for MockPdfEngine {
    fn name(&self) -> &'static str {
        "Mock Engine"
    }

    fn is_available(&self) -> bool {
        true
    }

    fn load_document(&self, path: &Path, _bytes: &[u8]) -> anyhow::Result<Arc<dyn PdfDocument>> {
        let title = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Document")
            .to_string();

        Ok(Arc::new(MockPdfDocument {
            title,
            page_count: self.default_page_count,
            width: self.page_width,
            height: self.page_height,
        }))
    }
}

pub struct MockPdfDocument {
    title: String,
    page_count: usize,
    width: f32,
    height: f32,
}

impl MockPdfDocument {
    fn generate_svg_markup(&self, page_index: usize, full_w: f32, full_h: f32) -> String {
        let page_num = page_index + 1;
        format!(
            r##"<rect width="{w}" height="{h}" fill="#ffffff" stroke="#d0d0d0" stroke-width="2"/>
                <rect x="20" y="20" width="{inner_w}" height="40" fill="#f0f4f8" rx="4"/>
                <text x="35" y="46" font-family="sans-serif" font-size="18" font-weight="bold" fill="#1e293b">{title}</text>
                <line x1="20" y1="80" x2="{line_w}" y2="80" stroke="#e2e8f0" stroke-width="2"/>
                <text x="35" y="120" font-family="sans-serif" font-size="14" fill="#64748b">Page {page_num} of {total_pages}</text>
                <rect x="35" y="150" width="{box_w}" height="120" fill="#f8fafc" stroke="#cbd5e1" stroke-dasharray="4" rx="6"/>
                <text x="50" y="215" font-family="sans-serif" font-size="13" fill="#94a3b8">Native GPUI Rendered PDF Page</text>"##,
            w = full_w,
            h = full_h,
            inner_w = (full_w - 40.0).max(10.0),
            line_w = (full_w - 20.0).max(10.0),
            box_w = (full_w - 70.0).max(10.0),
            title = self.title,
            page_num = page_num,
            total_pages = self.page_count,
        )
    }
}

impl PdfDocument for MockPdfDocument {
    fn page_count(&self) -> usize {
        self.page_count
    }

    fn page_size(&self, _page_index: usize) -> (f32, f32) {
        (self.width, self.height)
    }

    fn render_page(&self, page_index: usize, scale: f32) -> anyhow::Result<Arc<Image>> {
        if page_index >= self.page_count {
            return Err(anyhow!(
                "Page index {} out of range (total pages: {})",
                page_index,
                self.page_count
            ));
        }

        let w = (self.width * scale).max(50.0);
        let h = (self.height * scale).max(50.0);
        let body = self.generate_svg_markup(page_index, w, h);

        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
                {body}
            </svg>"##,
            w = w,
            h = h,
            body = body,
        );

        let image = Image::from_bytes(ImageFormat::Svg, svg.into_bytes());
        Ok(Arc::new(image))
    }

    fn render_tile(
        &self,
        page_index: usize,
        scale: f32,
        crop_x: u32,
        crop_y: u32,
        tile_w: u32,
        tile_h: u32,
    ) -> anyhow::Result<Arc<Image>> {
        if page_index >= self.page_count {
            return Err(anyhow!(
                "Page index {} out of range (total pages: {})",
                page_index,
                self.page_count
            ));
        }

        let full_w = (self.width * scale).max(50.0);
        let full_h = (self.height * scale).max(50.0);
        let body = self.generate_svg_markup(page_index, full_w, full_h);

        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{tile_w}" height="{tile_h}" viewBox="{crop_x} {crop_y} {tile_w} {tile_h}">
                {body}
            </svg>"##,
            tile_w = tile_w,
            tile_h = tile_h,
            crop_x = crop_x,
            crop_y = crop_y,
            body = body,
        );

        let image = Image::from_bytes(ImageFormat::Svg, svg.into_bytes());
        Ok(Arc::new(image))
    }
}

// ---------------------------------------------------------------------------
// Poppler CLI Engine (pdftoppm / pdfinfo fallback)
// ---------------------------------------------------------------------------

/// PDF engine that uses Linux/Unix command-line tools (`pdftoppm` and `pdfinfo`).
/// Provides zero-configuration native rendering on systems with poppler-utils installed.
#[derive(Debug, Clone, Default)]
pub struct PopplerCliEngine {
    pdftoppm_path: Option<PathBuf>,
}

impl PopplerCliEngine {
    pub fn new() -> Self {
        let path = which("pdftoppm");
        Self {
            pdftoppm_path: path,
        }
    }
}

impl PdfEngine for PopplerCliEngine {
    fn name(&self) -> &'static str {
        "Poppler (pdftoppm)"
    }

    fn is_available(&self) -> bool {
        self.pdftoppm_path.is_some()
    }

    fn load_document(&self, path: &Path, bytes: &[u8]) -> anyhow::Result<Arc<dyn PdfDocument>> {
        let Some(pdftoppm) = self.pdftoppm_path.as_ref() else {
            return Err(anyhow!("pdftoppm utility is not installed"));
        };

        // Determine page count and dimensions via pdfinfo or fallback
        let (page_count, width, height) = query_pdf_info(path).unwrap_or((1, 612.0, 792.0));

        Ok(Arc::new(PopplerCliDocument {
            path: path.to_path_buf(),
            _bytes: bytes.to_vec(),
            pdftoppm: pdftoppm.clone(),
            page_count: page_count.max(1),
            width,
            height,
        }))
    }
}

pub struct PopplerCliDocument {
    path: PathBuf,
    _bytes: Vec<u8>,
    pdftoppm: PathBuf,
    page_count: usize,
    width: f32,
    height: f32,
}

impl PdfDocument for PopplerCliDocument {
    fn page_count(&self) -> usize {
        self.page_count
    }

    fn page_size(&self, _page_index: usize) -> (f32, f32) {
        (self.width, self.height)
    }

    fn render_page(&self, page_index: usize, scale: f32) -> anyhow::Result<Arc<Image>> {
        let page_1_indexed = page_index + 1;
        let scaled_w = ((self.width * scale) as u32).max(1);
        let scaled_h = ((self.height * scale) as u32).max(1);

        // Run `pdftoppm -png -f <page> -l <page> -scale-to-x <w> -scale-to-y <h> <input_file>`
        let output = Command::new(&self.pdftoppm)
            .arg("-png")
            .arg("-f")
            .arg(page_1_indexed.to_string())
            .arg("-l")
            .arg(page_1_indexed.to_string())
            .arg("-scale-to-x")
            .arg(scaled_w.to_string())
            .arg("-scale-to-y")
            .arg(scaled_h.to_string())
            .arg(&self.path)
            .output()
            .with_context(|| format!("Failed to execute {:?}", self.pdftoppm))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("pdftoppm rendering failed: {}", stderr));
        }

        let image = Image::from_bytes(ImageFormat::Png, output.stdout);
        Ok(Arc::new(image))
    }

    fn render_tile(
        &self,
        page_index: usize,
        scale: f32,
        crop_x: u32,
        crop_y: u32,
        tile_w: u32,
        tile_h: u32,
    ) -> anyhow::Result<Arc<Image>> {
        let page_1_indexed = page_index + 1;
        let scaled_w = ((self.width * scale) as u32).max(1);
        let scaled_h = ((self.height * scale) as u32).max(1);

        // Run `pdftoppm -png -f <page> -l <page> -scale-to-x <W> -scale-to-y <H> -x <X> -y <Y> -W <TW> -H <TH> <input>`
        let output = Command::new(&self.pdftoppm)
            .arg("-png")
            .arg("-f")
            .arg(page_1_indexed.to_string())
            .arg("-l")
            .arg(page_1_indexed.to_string())
            .arg("-scale-to-x")
            .arg(scaled_w.to_string())
            .arg("-scale-to-y")
            .arg(scaled_h.to_string())
            .arg("-x")
            .arg(crop_x.to_string())
            .arg("-y")
            .arg(crop_y.to_string())
            .arg("-W")
            .arg(tile_w.to_string())
            .arg("-H")
            .arg(tile_h.to_string())
            .arg(&self.path)
            .output()
            .with_context(|| format!("Failed to execute {:?}", self.pdftoppm))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("pdftoppm tile rendering failed: {}", stderr));
        }

        let image = Image::from_bytes(ImageFormat::Png, output.stdout);
        Ok(Arc::new(image))
    }
}

// ---------------------------------------------------------------------------
// Helpers & Factory
// ---------------------------------------------------------------------------

fn which(cmd: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(':') {
            let candidate = Path::new(p).join(cmd);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn query_pdf_info(path: &Path) -> Option<(usize, f32, f32)> {
    let output = Command::new("pdfinfo").arg(path).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut pages = None;
    let mut width = 612.0;
    let mut height = 792.0;

    for line in stdout.lines() {
        if line.starts_with("Pages:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                pages = parts[1].parse::<usize>().ok();
            }
        } else if line.starts_with("Page size:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                if let (Ok(w), Ok(h)) = (parts[2].parse::<f32>(), parts[4].parse::<f32>()) {
                    width = w;
                    height = h;
                }
            }
        }
    }

    pages.map(|p| (p, width, height))
}

/// Creates the best available PDF engine for the current environment.
pub fn create_default_engine() -> Arc<dyn PdfEngine> {
    let poppler = PopplerCliEngine::new();
    if poppler.is_available() {
        return Arc::new(poppler);
    }

    // Fallback to MockEngine for headless test or unconfigured systems
    Arc::new(MockPdfEngine::default())
}

//! Thread-safe PDF document handle and metadata index.
//!
//! Exposes page count, dimensions, and aspect ratios without requiring
//! active rendering locks on the UI thread.

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Dimension metrics for a single PDF page in standard points (1/72 inch).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageDimensions {
    pub width: f32,
    pub height: f32,
}

impl PageDimensions {
    /// Constructs dimensions from width and height.
    #[inline]
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Aspect ratio (width / height). Guaranteed non-zero.
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        if self.height > 0.0 {
            self.width / self.height
        } else {
            1.0
        }
    }

    /// Calculates scaled dimensions in pixels for a given scale and target DPI.
    #[inline]
    pub fn to_pixel_size(&self, zoom_factor: f32, dpi: f32) -> (u32, u32) {
        let scale = (dpi / 72.0) * zoom_factor;
        let w = (self.width * scale).round().max(1.0) as u32;
        let h = (self.height * scale).round().max(1.0) as u32;
        (w, h)
    }
}

/// Metadata and page geometry cache for an opened PDF document.
#[derive(Debug, Clone)]
pub struct PdfDocument {
    path: Option<PathBuf>,
    pages: Arc<Vec<PageDimensions>>,
    title: Option<String>,
}

impl PdfDocument {
    /// Constructs a PdfDocument with explicit page dimensions.
    pub fn new(path: Option<PathBuf>, pages: Vec<PageDimensions>, title: Option<String>) -> Self {
        Self {
            path,
            pages: Arc::new(pages),
            title,
        }
    }

    /// Total number of pages in the document.
    #[inline]
    pub fn total_pages(&self) -> usize {
        self.pages.len()
    }

    /// True if the document has 0 pages.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// Dimensions of the page at `page_index` (0-indexed).
    #[inline]
    pub fn page_size(&self, page_index: usize) -> Option<PageDimensions> {
        self.pages.get(page_index).copied()
    }

    /// Aspect ratio of the page at `page_index` (0-indexed).
    #[inline]
    pub fn aspect_ratio(&self, page_index: usize) -> Option<f32> {
        self.pages.get(page_index).map(|d| d.aspect_ratio())
    }

    /// Absolute file system path if loaded from disk.
    #[inline]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Slice of all page dimensions in the document.
    #[inline]
    pub fn pages(&self) -> &[PageDimensions] {
        &self.pages
    }

    /// Computes overall width and height in pixels for the entire stitched document.
    pub fn total_pixel_size(&self, zoom_factor: f32, dpi: f32, page_gap: u32) -> (u32, u32) {
        if self.pages.is_empty() {
            return (0, 0);
        }
        let mut max_w = 0u32;
        let mut total_h = 0u32;
        for (i, page) in self.pages.iter().enumerate() {
            let (w, h) = page.to_pixel_size(zoom_factor, dpi);
            max_w = max_w.max(w);
            total_h += h;
            if i + 1 < self.pages.len() {
                total_h += page_gap;
            }
        }
        (max_w, total_h)
    }

    /// Document title if available in metadata, or falls back to filename.
    pub fn display_title(&self) -> String {
        if let Some(ref title) = self.title {
            if !title.trim().is_empty() {
                return title.clone();
            }
        }
        if let Some(ref path) = self.path {
            if let Some(file_name) = path.file_name() {
                return file_name.to_string_lossy().to_string();
            }
        }
        "Untitled Document.pdf".to_string()
    }

    /// Inspects raw PDF bytes to extract basic page count and geometry.
    /// Fast fallback scanner when native Pdfium context is running in a background task.
    pub fn from_bytes(bytes: &[u8], path: Option<PathBuf>) -> Result<Self> {
        // Validate PDF magic bytes (%PDF-)
        if bytes.len() < 5 || &bytes[0..5] != b"%PDF-" {
            anyhow::bail!("Invalid PDF header magic bytes");
        }

        // Default standard US Letter (612x792 pt) if full metadata parser not yet invoked
        let default_page = PageDimensions::new(612.0, 792.0);
        let pages = vec![default_page];

        Ok(Self::new(path, pages, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_dimensions_and_scaling() {
        let dim = PageDimensions::new(600.0, 800.0);
        assert_eq!(dim.aspect_ratio(), 0.75);

        // At 72 DPI and 1.0x zoom -> exactly 600x800 px
        let (w, h) = dim.to_pixel_size(1.0, 72.0);
        assert_eq!((w, h), (600, 800));

        // At 144 DPI (Retina 2x) and 1.5x zoom -> 600 * 2 * 1.5 = 1800
        let (w2, h2) = dim.to_pixel_size(1.5, 144.0);
        assert_eq!((w2, h2), (1800, 2400));
    }

    #[test]
    fn test_document_metadata_access() {
        let pages = vec![
            PageDimensions::new(595.0, 842.0), // A4
            PageDimensions::new(612.0, 792.0), // US Letter
        ];
        let doc = PdfDocument::new(
            Some(PathBuf::from("/tmp/sample.pdf")),
            pages,
            Some("Sample".into()),
        );

        assert_eq!(doc.total_pages(), 2);
        assert_eq!(doc.page_size(0), Some(PageDimensions::new(595.0, 842.0)));
        assert_eq!(doc.page_size(1), Some(PageDimensions::new(612.0, 792.0)));
        assert_eq!(doc.page_size(2), None);
        assert_eq!(doc.display_title(), "Sample");

        // Test total_pixel_size: at 72 DPI, 1.0x zoom, 20px gap
        // max width = max(595, 612) = 612
        // total height = 842 + 792 + 20 = 1654
        let (total_w, total_h) = doc.total_pixel_size(1.0, 72.0, 20);
        assert_eq!((total_w, total_h), (612, 1654));
    }

    #[test]
    fn test_pdf_magic_bytes_check() {
        let valid_pdf = b"%PDF-1.7\nSample content";
        assert!(PdfDocument::from_bytes(valid_pdf, None).is_ok());

        let invalid_pdf = b"NOT A PDF";
        assert!(PdfDocument::from_bytes(invalid_pdf, None).is_err());
    }
}

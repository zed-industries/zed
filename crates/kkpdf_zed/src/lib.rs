//! # `kkpdf-zed`
//!
//! High-performance native PDF viewing engine and GPUI workspace item for Zed Editor.
//!
//! Features:
//! - Sub-millisecond viewport rendering via bounded LRU memory caching
//! - Smart luminosity-threshold dark mode tone mapping preserving saturated colors
//! - Continuous multi-page scroll, single-page, and two-page spread layouts
//! - Precision focal-point zoom (10% to 2000%) with mouse and gesture support
//! - Debounced disk change watcher preserving exact scroll percentages and zoom levels
//! - Native Pdfium C++ engine bindings with robust fallback support

pub mod cache;
pub mod document;
pub mod pdfium;
pub mod rasterizer;
pub mod settings;
pub mod ui;
pub mod view;
pub mod watcher;

pub use cache::{CacheKey, PageLruCache, RenderedPage, DEFAULT_MEMORY_BUDGET_BYTES};
pub use document::{PageDimensions, PdfDocument};
pub use pdfium::{
    PdfDocumentDetails, PdfLinkAnnotation, PdfPageDetails, PdfTextSegment, PdfiumEngine,
};
pub use rasterizer::{LuminosityToneMapper, PageRasterizer, RasterizerOptions};
pub use settings::{DefaultZoomPolicy, PageLayoutMode, PdfViewerSettings};
pub use ui::{PdfToolbarAction, PdfToolbarState};
pub use view::{PdfView, PdfViewEvent, MAX_ZOOM, MIN_ZOOM, PAGE_SPACING_PX, ZOOM_STEP};
pub use watcher::{PdfReloadDebouncer, ViewerStateSnapshot};

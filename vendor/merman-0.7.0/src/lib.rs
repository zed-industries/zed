#![forbid(unsafe_code)]

//! Headless, parity-focused Mermaid parsing and rendering in Rust.
//!
//! `merman` is the public Rust facade for the project. It re-exports
//! [`merman_core`] for detection, parsing, metadata, semantic JSON, and typed
//! render models, then adds optional convenience modules for SVG, raster, and
//! terminal text output.
//!
//! The compatibility target is Mermaid `@11.15.0`. Upstream Mermaid behavior is
//! treated as the specification, including cases where the browser implementation
//! is surprising. The root README and `docs/alignment/STATUS.md` document the
//! current parity matrix, deferred residuals, and release gates.
//!
//! # Choosing an API
//!
//! | Goal | Feature | Start with |
//! | --- | --- | --- |
//! | Parse Mermaid or produce semantic JSON | none | [`Engine`] and [`ParseOptions`] |
//! | Render Mermaid-like SVG | `render` | `merman::render::HeadlessRenderer` |
//! | Prepare SVG for `usvg` / `resvg` / raster export | `render` | `HeadlessRenderer::render_svg_resvg_safe_sync` |
//! | Render terminal-friendly text | `ascii` | `merman::ascii::HeadlessAsciiRenderer` |
//! | Render PNG, JPG, or PDF from Rust | `raster` | `HeadlessRenderer::render_png_sync` and `render::raster::RasterOptions` |
//!
//! If you already know the diagram type, use the `*_with_type_sync` methods on
//! [`Engine`] to skip detection. If you need lower-level layout or SVG pipeline
//! control, use the re-exported types under `merman::render` or depend on
//! `merman-render` directly.
//!
//! # Features
//!
//! - `render`: layout plus SVG rendering through `merman::render`.
//! - `ascii`: ASCII/Unicode text rendering through `merman::ascii`.
//! - `raster`: PNG/JPG/PDF output through `merman::render::raster`; this implies
//!   `render`.
//! - `ratex-math`: pure-Rust math label rendering for the SVG path; this implies
//!   `render`.
//!
//! The default feature set is intentionally empty so parser-only users do not
//! pull in layout, SVG, raster, or text-output dependencies.
//!
//! # SVG quickstart
//!
//! ```no_run
//! # #[cfg(feature = "render")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use merman::render::HeadlessRenderer;
//!
//! let renderer = HeadlessRenderer::new().with_diagram_id("readme-example");
//! let svg = renderer
//!     .render_svg_sync("flowchart TD\nA[Start] --> B[Done]")?
//!     .expect("diagram detected");
//!
//! println!("{svg}");
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "render"))]
//! # fn main() {}
//! ```
//!
//! A fresh `HeadlessRenderer` keeps the Mermaid parity SVG contract for
//! `HeadlessRenderer::render_svg_sync`. Calling `with_host_theme` or
//! `with_svg_pipeline` installs a renderer-owned output pipeline for that
//! method. Use
//! `HeadlessRenderer::render_svg_readable_sync` when browser
//! `<foreignObject>` labels may need readable `<text>` fallbacks, and
//! `HeadlessRenderer::render_svg_resvg_safe_sync` when the output will
//! be consumed by `usvg`, `resvg`, or the built-in raster helpers.
//!
//! # ASCII quickstart
//!
//! ```no_run
//! # #[cfg(feature = "ascii")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use merman::ascii::{AsciiRenderOptions, HeadlessAsciiRenderer};
//!
//! let renderer = HeadlessAsciiRenderer::new()
//!     .with_strict_parsing()
//!     .with_ascii_options(AsciiRenderOptions::unicode());
//! let text = renderer
//!     .render_ascii_sync("sequenceDiagram\nA->>B: Hello")?
//!     .expect("diagram detected");
//!
//! println!("{text}");
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "ascii"))]
//! # fn main() {}
//! ```
//!
//! Text output is intentionally terminal-native rather than SVG-derived. The
//! currently supported public subset covers flowchart/graph, sequenceDiagram,
//! classDiagram, erDiagram, and xychart.
//!
//! # Raster output
//!
//! The `raster` feature renders SVG through the `resvg`-safe pipeline before
//! conversion. PNG and JPG use a default pixmap budget to avoid accidental huge
//! allocations from very large Mermaid `viewBox` values. For UI previews, pass a
//! visible target box through `RasterOptions::with_fit_to` and use
//! `RasterOptions::with_scale` for device-pixel ratio.

pub use merman_core::*;

#[cfg(feature = "ascii")]
pub mod ascii;

#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "render")]
pub use render::supported_host_theme_presets;

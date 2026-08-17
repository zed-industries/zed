//! Renderer-facing presentation theme roles.
//!
//! The implementation currently shares the SVG parity theme view because those CSS consumers were
//! the first migration slice. Non-SVG-layout consumers should import through this crate-level
//! module so the physical home can move without coupling diagram families to `svg::parity`.

pub(crate) use crate::svg::render_theme::{PresentationTheme, QuadrantChartTheme};

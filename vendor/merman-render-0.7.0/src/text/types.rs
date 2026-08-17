//! Shared text measurement types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WrapMode {
    #[default]
    SvgLike,
    /// SVG `<text>` behaves as a single shaping run (no whitespace-to-`<tspan>` tokenization).
    ///
    /// Mermaid uses this behavior in some diagrams (e.g. sequence message labels), where the
    /// resulting `getBBox()` width differs measurably from per-word `<tspan>` tokenization.
    SvgLikeSingleRun,
    HtmlLike,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: f64,
    pub font_weight: Option<String>,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_family: None,
            font_size: 16.0,
            font_weight: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TextMetrics {
    pub width: f64,
    pub height: f64,
    pub line_count: usize,
}

// for a very big json! macro
#![recursion_limit = "256"]

//! Crate for rendering Mermaid diagram strings to SVG strings.
//!
//! The entrypoint to this crate is [`render_to_svg`].
//!
//! It takes a `&str` and a [`MermaidTheme`]. The output is an SVG with the
//! following properties:
//! - The style matches the provided theme
//! - Nodes are given accent colors, even if none are provided in the mermaid
//!   source.
//! - The SVG has been tweaked based on the assumption that it will be rasterized
//!   using `usvg`/`resvg`. Some bugs/quirks of `usvg`/`resvg` are accounted for
//!   in this crate.
//!
//! This module uses the [`merman`] crate for rendering, rather than
//! `mermaid-rs`, which was used in the previous implementation of mermaid
//! rendering in Zed. Merman provides significantly more accurate rendering, and
//! seems to be somewhat faster, but by default has poor CSS, making diagrams
//! look weird without significant cleanup. This is made worse by the fact that
//! `usvg`/`resvg` doesn't support some features that [`merman`] relies on.
//!
//! As such, this crate is quite large. But the code is very self-contained, and
//! has few dependencies. In fact, the [`gpui`] dependency is only needed for
//! the [`Hsla`] and [`Rgba`] color types.
//!
//! The [`render_to_svg`] function operates in two stages:
//! - [`render`] the mermaid text to SVG using [`merman`].
//! - [`postprocess`] the SVG to clean incorrect output and add styling.
//!
//! The postprocessing is also split up into stages. We parse the generated SVG
//! using [`quick_xml`], which produces an iterator of
//! [`Event<'_>`](quick_xml::events::Event)s. This iterator is then repeatedly
//! transformed, and finally collected back into an SVG string.
//!
//! This approach:
//! - Avoids doing multiple expensive string insertions.
//! - Avoids parsing the SVG multiple times (without needing to put all the
//!   logic in one huge function).
//! - But is quite a bit more complex.
//!
//! I think this complexity is justified because of the drastic performance
//! impact, as well as the low-risk nature; this code cannot panic, and errors
//! in the output just produce weird-looking diagrams.
//!
//! ## Color handling
//!
//! We try to match the users theme, and also apply accent colors to diagrams to
//! make them more visually interesting. Accent colors are derived from the
//! `player_colors` in the Zed theme.
//!
//! There are three parts to color handling:
//!
//! 1. A [`merman::MermaidConfig`] is passed when initially rendering the
//!    diagram. This sets most "normal" colors (background, text, etc.). However,
//!    it's not possible to color nodes individually, and not all parts of the
//!    diagrams are correctly themed.
//! 2. `postprocess::accent_colors` injects custom CSS classes (e.g.
//!    `zed-accent-0`) to specific elements, based on the diagram type and
//!    node.
//! 3. `postprocess::inject_css` injects CSS rules for the classes applied by
//!    `accent_colors`

mod postprocess;
mod render;

use anyhow::Result;
use gpui::{Hsla, Rgba};

#[derive(Debug, Clone, Copy)]
pub struct AccentColor {
    pub foreground: Hsla,
    pub background: Hsla,
}

#[derive(Debug, Clone)]
pub struct MermaidTheme {
    pub dark_mode: bool,
    pub font_family: String,
    pub background: Hsla,
    pub primary_color: Hsla,
    pub primary_text_color: Hsla,
    pub primary_border_color: Hsla,
    pub secondary_color: Hsla,
    pub tertiary_color: Hsla,
    pub line_color: Hsla,
    pub text_color: Hsla,
    pub edge_label_background: Hsla,
    pub cluster_background: Hsla,
    pub cluster_border: Hsla,
    pub note_background: Hsla,
    pub note_border: Hsla,
    pub actor_background: Hsla,
    pub actor_border: Hsla,
    pub activation_background: Hsla,
    pub activation_border: Hsla,
    pub git_branch_colors: [Hsla; 8],
    pub git_branch_label_colors: [Hsla; 8],
    pub er_attr_bg_odd: Hsla,
    pub er_attr_bg_even: Hsla,
    pub error_color: Hsla,
    pub warning_color: Hsla,
    pub accent_colors: Vec<AccentColor>,
}

/// Default theme for testing.
#[cfg(any(test, feature = "test-support"))]
impl Default for MermaidTheme {
    fn default() -> Self {
        use gpui::{hsla, rgb};
        let git_branch_colors: [Hsla; 8] = [
            hsla(240.0 / 360.0, 1.0, 0.462_745_1, 1.0),
            hsla(60.0 / 360.0, 1.0, 0.435_294_12, 1.0),
            hsla(80.0 / 360.0, 1.0, 0.462_745_1, 1.0),
            hsla(210.0 / 360.0, 1.0, 0.462_745_1, 1.0),
            hsla(180.0 / 360.0, 1.0, 0.462_745_1, 1.0),
            hsla(150.0 / 360.0, 1.0, 0.462_745_1, 1.0),
            hsla(300.0 / 360.0, 1.0, 0.462_745_1, 1.0),
            hsla(0.0, 1.0, 0.462_745_1, 1.0),
        ];
        let git_branch_label_colors: [Hsla; 8] =
            git_branch_colors.map(crate::text_color_for_background);

        Self {
            dark_mode: false,
            font_family: "Inter, ui-sans-serif, system-ui, -apple-system, \"Segoe UI\", \"DejaVu Sans\", \"Liberation Sans\", sans-serif, \"Noto Color Emoji\", \"Apple Color Emoji\", \"Segoe UI Emoji\"".to_string(),
            background: rgb(0xFFFFFF).into(),
            primary_color: rgb(0xF8FAFC).into(),
            primary_text_color: rgb(0x0F172A).into(),
            primary_border_color: rgb(0x94A3B8).into(),
            secondary_color: rgb(0xE2E8F0).into(),
            tertiary_color: rgb(0xFFFFFF).into(),
            line_color: rgb(0x64748B).into(),
            text_color: rgb(0x0F172A).into(),
            edge_label_background: rgb(0xFFFFFF).into(),
            cluster_background: rgb(0xF1F5F9).into(),
            cluster_border: rgb(0xCBD5E1).into(),
            note_background: rgb(0xFFF7ED).into(),
            note_border: rgb(0xFDBA74).into(),
            actor_background: rgb(0xF8FAFC).into(),
            actor_border: rgb(0x94A3B8).into(),
            activation_background: rgb(0xE2E8F0).into(),
            activation_border: rgb(0x94A3B8).into(),
            git_branch_colors,
            git_branch_label_colors,
            er_attr_bg_odd: rgb(0x94A3B8).into(),
            er_attr_bg_even: rgb(0x0F172A).into(),
            error_color: rgb(0xDC2626).into(),
            warning_color: rgb(0xD97706).into(),
            accent_colors: Vec::new(),
        }
    }
}

/// Formats a color as a CSS hex color for embedding in SVG/CSS.
///
/// Emits `#rrggbb` for fully opaque colors and `#rrggbbaa` when the input
/// has any transparency, so translucent theme colors (e.g. `ghost_element_hover`
/// from Zed's UI palette) round-trip without silently losing their alpha.
pub(crate) fn css_color(color: Hsla) -> String {
    let rgba = Rgba::from(color);
    let r = (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (rgba.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (rgba.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = (rgba.a.clamp(0.0, 1.0) * 255.0).round() as u8;
    if a == 0xff {
        format!("#{r:02x}{g:02x}{b:02x}")
    } else {
        format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
    }
}

pub use postprocess::util::text_color_for_background;

/// See the [module-level docs][crate] for more info.
pub fn render_to_svg(source: &str, theme: &MermaidTheme) -> Result<String> {
    let svg = render::render_mermaid(source, theme)?;
    let svg = postprocess::postprocess(&svg, theme)?;
    Ok(svg)
}

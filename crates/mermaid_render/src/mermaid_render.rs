// for a very big json! macro
#![recursion_limit = "256"]

//! Crate for rendering Mermaid diagram strings to SVG strings.
//!
//! The entrypoint to this crate is [`render_to_svg`].
//!
//! It takes a `&str` and a [`MermaidTheme`]. The output is an SVG with the
//! following properties:
//! - The provided theme supplies defaults; explicit diagram colors take priority.
//! - Nodes are given accent colors, even if none are provided in the mermaid
//!   source, unless the diagram selects its own palette.
//! - The SVG has been tweaked based on the assumption that it will be rasterized
//!   using `usvg`/`resvg`. Some bugs/quirks of `usvg`/`resvg` are accounted for
//!   in this crate.
//!
//! This module uses the [`merman`] crate for rendering, rather than
//! `mermaid-rs`, which was used in the previous implementation of mermaid
//! rendering in Zed.
//!
//! Historically, this crate also carried generic `usvg`/`resvg` cleanup for SVG
//! constructs that merman's parity output could emit, such as HTML labels in
//! `<foreignObject>` and CSS/attribute forms that rasterizers do not handle.
//! Since merman 0.6, that generic cleanup is exposed as merman's raster-safe SVG
//! pipeline. Zed opts into that pipeline during rendering, then keeps
//! editor-specific theme and accent color rules in this crate. The [`gpui`]
//! dependency is only needed for the [`Hsla`] and [`Rgba`] color types.
//!
//! The [`render_to_svg`] function operates in two stages:
//! - [`render`] the mermaid text to raster-safe SVG using [`merman`].
//! - [`postprocess`] the SVG to add Zed theme and accent styling.
//!
//! Zed's postprocessing is split up into stages. We parse the generated SVG
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

/// Renders a diagram using host colors as defaults. `style`, `classDef` and
/// `linkStyle` colors retain their CSS priority. Front matter and init directives
/// may override color-valued `themeVariables`, `fontFamily`, pixel `fontSize`, and
/// boolean `darkMode`. Nonempty overrides disable host accent overlays, while
/// unspecified variables retain host defaults. Invalid values return an error.
/// Arbitrary `themeCSS` remains disabled by merman.
/// See the [module-level docs][crate] for more info.
#[ztracing::instrument(skip_all)]
pub fn render_to_svg(source: &str, theme: &MermaidTheme) -> Result<String> {
    let (svg, custom_palette) = render::render_mermaid(source, theme)?;
    postprocess::postprocess(&svg, theme, custom_palette)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rendered_path_colors(source: &str, theme: &MermaidTheme) -> Vec<usvg::Color> {
        fn collect(group: &usvg::Group, colors: &mut Vec<usvg::Color>) {
            for node in group.children() {
                match node {
                    usvg::Node::Group(group) => collect(group, colors),
                    usvg::Node::Path(path) => {
                        if let Some(fill) = path.fill()
                            && let usvg::Paint::Color(color) = fill.paint()
                        {
                            colors.push(*color);
                        }
                        if let Some(stroke) = path.stroke()
                            && let usvg::Paint::Color(color) = stroke.paint()
                        {
                            colors.push(*color);
                        }
                    }
                    _ => {}
                }
            }
        }
        let svg = render_to_svg(source, theme).expect("diagram renders");
        let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).expect("SVG parses");
        let mut colors = Vec::new();
        collect(tree.root(), &mut colors);
        colors
    }

    #[test]
    fn diagram_colors_override_host_defaults() {
        let themes = [
            MermaidTheme::default(),
            MermaidTheme {
                dark_mode: true,
                primary_color: gpui::rgb(0x202020).into(),
                background: gpui::rgb(0x101010).into(),
                accent_colors: vec![AccentColor {
                    foreground: gpui::rgb(0x345678).into(),
                    background: gpui::rgb(0x234567).into(),
                }],
                ..MermaidTheme::default()
            },
        ];
        for theme in themes {
            for source in [
                "flowchart TD\n A[Custom] --> B[Default]\n style A fill:#ff1234,stroke:#12ff34",
                "flowchart TD\n A[Custom]:::custom --> B[Default]\n classDef custom fill:#ff1234,stroke:#12ff34",
                "---\nconfig:\n  theme: base\n  themeVariables:\n    primaryColor: '#ff1234'\n    primaryBorderColor: '#12ff34'\n---\nflowchart TD\n A --> B",
                "%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#ff1234', 'primaryBorderColor': '#12ff34'}}}%%\nflowchart TD\n A --> B",
            ] {
                let colors = rendered_path_colors(source, &theme);
                assert!(
                    colors.contains(&usvg::Color::new_rgb(255, 18, 52)),
                    "custom fill lost: {source}: {colors:?}"
                );
                assert!(
                    colors.contains(&usvg::Color::new_rgb(18, 255, 52)),
                    "custom stroke lost: {source}: {colors:?}"
                );
            }
        }
    }

    #[test]
    fn host_defaults_change_while_individual_colors_stay_fixed() {
        let light = MermaidTheme::default();
        let dark = MermaidTheme {
            dark_mode: true,
            primary_color: gpui::rgb(0x202020).into(),
            background: gpui::rgb(0x101010).into(),
            ..light.clone()
        };
        let plain = "flowchart TD\n A --> B";
        let empty = "---\nconfig:\n  themeVariables: {}\n---\nflowchart TD\n A --> B";
        assert_eq!(
            rendered_path_colors(plain, &dark),
            rendered_path_colors(empty, &dark)
        );
        assert_ne!(
            rendered_path_colors(plain, &light),
            rendered_path_colors(plain, &dark)
        );
        let partial = "---\nconfig:\n  themeVariables:\n    lineColor: '#ff1234'\n---\nflowchart TD\n A --> B";
        assert!(rendered_path_colors(partial, &dark).contains(&usvg::Color::new_rgb(32, 32, 32)));
        assert!(rendered_path_colors(partial, &dark).contains(&usvg::Color::new_rgb(255, 18, 52)));
        let explicit = "flowchart TD\n A --> B\n style A fill:#000000\n linkStyle 0 stroke:#ff1234";
        let colors = rendered_path_colors(explicit, &dark);
        assert!(colors.contains(&usvg::Color::new_rgb(0, 0, 0)));
        assert!(colors.contains(&usvg::Color::new_rgb(255, 18, 52)));
        let preset = "---\nconfig:\n  theme: forest\n---\nflowchart TD\n A --> B";
        assert_eq!(
            rendered_path_colors(preset, &light),
            rendered_path_colors(preset, &dark)
        );
        assert_ne!(
            rendered_path_colors(preset, &dark),
            rendered_path_colors(plain, &dark)
        );
    }

    fn text_color(group: &usvg::Group, label: &str) -> Option<usvg::Color> {
        for node in group.children() {
            match node {
                usvg::Node::Group(group) => {
                    if let Some(color) = text_color(group, label) {
                        return Some(color);
                    }
                }
                usvg::Node::Text(text) => {
                    for chunk in text.chunks() {
                        if chunk.text() == label {
                            let fill = chunk.spans().first()?.fill()?;
                            if let usvg::Paint::Color(color) = fill.paint() {
                                return Some(*color);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    #[test]
    fn explicit_text_colors_survive_native_svg_fallback() {
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        for configuration in [
            "",
            "---\nconfig:\n  themeVariables:\n    primaryColor: '#eeeeee'\n---\n",
        ] {
            let source = format!(
                "{configuration}flowchart LR\n A[Orange] --> B[Green]\n style A fill:#ffddaa,color:#332200\n classDef custom fill:#225544,color:#ffffff\n class B custom"
            );
            let theme = MermaidTheme {
                background: gpui::rgb(0x101010).into(),
                ..MermaidTheme::default()
            };
            let svg = render_to_svg(&source, &theme).expect("render");
            assert!(
                svg.contains("background-color:#101010"),
                "host background lost: {}",
                &svg[..svg.find('>').expect("root tag")]
            );
            let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parses");
            assert_eq!(
                text_color(tree.root(), "Orange"),
                Some(usvg::Color::new_rgb(51, 34, 0))
            );
            assert_eq!(
                text_color(tree.root(), "Green"),
                Some(usvg::Color::new_rgb(255, 255, 255))
            );
        }
    }

    #[test]
    fn packet_front_matter_layout_and_colors_render() {
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        for dark_mode in [false, true] {
            let foreground = if dark_mode { 0xeeeeee } else { 0x222222 };
            let background = if dark_mode { 0x303030 } else { 0xdddddd };
            let theme = MermaidTheme {
                dark_mode,
                primary_color: gpui::rgb(background).into(),
                primary_text_color: gpui::rgb(foreground).into(),
                text_color: gpui::rgb(foreground).into(),
                ..MermaidTheme::default()
            };
            for custom_colors in [false, true] {
                let colors = if custom_colors {
                    "    labelColor: '#123456'\n    blockFillColor: '#fedcba'\n"
                } else {
                    ""
                };
                let source = format!(
                    "---\ntitle: Packet example\nconfig:\n  packet:\n    bitsPerRow: 64\n    bitWidth: 12\n{colors}---\npacket\n+8: \"Header\"\n+8: \"Kind\"\n+32: \"Identifier\"\n+16: \"Index\"\n+8: \"Length\"\n+432: \"Data\""
                );
                let svg = render_to_svg(&source, &theme).expect("packet render");
                let tree = usvg::Tree::from_str(&svg, &options).expect("native packet SVG");
                let expected_text = if custom_colors { 0x123456 } else { foreground };
                let expected_fill = if custom_colors { 0xfedcba } else { background };
                let color =
                    |rgb: u32| usvg::Color::new_rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8);
                assert_eq!(
                    text_color(tree.root(), "Header"),
                    Some(color(expected_text))
                );
                assert_eq!(text_color(tree.root(), "Data"), Some(color(expected_text)));
                assert_eq!(
                    text_color(tree.root(), "Packet example"),
                    Some(color(foreground))
                );
                assert!(rendered_path_colors(&source, &theme).contains(&color(expected_fill)));
                assert!(
                    svg.contains("width=\"91\""),
                    "8 bits at width 12 minus default padding 5"
                );
                assert!(
                    text_color(tree.root(), "503").is_some(),
                    "last bit of 63 bytes"
                );
            }
        }
    }

    #[test]
    fn newly_enabled_diagram_labels_follow_theme_and_nested_overrides() {
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let theme = MermaidTheme {
            dark_mode: true,
            background: gpui::rgb(0x282c33).into(),
            text_color: gpui::rgb(0xdce0e5).into(),
            ..MermaidTheme::default()
        };
        for (source, label, expected) in [
            ("treeView-beta\n \"Root\"", "Root", 0xdce0e5),
            (
                "---\nconfig:\n  themeVariables:\n    treeView:\n      labelColor: '#123456'\n---\ntreeView-beta\n \"Root\"",
                "Root",
                0x123456,
            ),
            (
                "treemap-beta\n\"Root\"\n \"A\": 30\n \"B\": 20",
                "B",
                0xdce0e5,
            ),
            ("eventmodeling\ntf 01 ui App.Form", "UI/A: App", 0xdce0e5),
            (
                "venn-beta\n set A[\"Alpha\"]:10\n set B[\"Beta\"]:8\n union A,B[\"Both\"]:3",
                "Both",
                0xdce0e5,
            ),
        ] {
            let svg = render_to_svg(source, &theme).expect("render");
            let tree = usvg::Tree::from_str(&svg, &options).expect("native SVG");
            assert_eq!(
                text_color(tree.root(), label),
                Some(usvg::Color::new_rgb(
                    (expected >> 16) as u8,
                    (expected >> 8) as u8,
                    expected as u8
                )),
                "{label}"
            );
        }
    }

    #[test]
    fn explicit_background_overrides_host_background() {
        let theme = MermaidTheme {
            background: gpui::rgb(0x101010).into(),
            ..MermaidTheme::default()
        };
        let white = "---\nconfig:\n  themeVariables:\n    background: '#ffffff'\n---\nflowchart TD\n A --> B";
        assert!(
            render_to_svg(white, &theme)
                .expect("white background")
                .contains("background-color:#ffffff")
        );
    }

    #[test]
    fn named_theme_uses_its_effective_background() {
        let source = "---\nconfig:\n  theme: dark\n---\npie\n  title Example\n  \"A\": 1";
        let svg = render_to_svg(source, &MermaidTheme::default()).expect("dark theme");

        assert!(svg.contains("background-color:#333"), "got: {svg}");
        assert!(!svg.contains("background-color:white"), "got: {svg}");
    }

    #[test]
    fn explicit_packet_colors_override_host_defaults() {
        let source = "---\ntitle: Packet example\nconfig:\n  packet:\n    labelColor: black\n    titleColor: white\n---\npacket\n+8: \"Header\"";
        let theme = MermaidTheme {
            primary_text_color: gpui::rgb(0x123456).into(),
            text_color: gpui::rgb(0x654321).into(),
            ..MermaidTheme::default()
        };
        let svg = render_to_svg(source, &theme).expect("packet colors");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("native packet SVG");

        assert_eq!(
            text_color(tree.root(), "Header"),
            Some(usvg::Color::new_rgb(0, 0, 0))
        );
        assert_eq!(
            text_color(tree.root(), "Packet example"),
            Some(usvg::Color::new_rgb(255, 255, 255))
        );
    }

    #[test]
    fn invalid_palette_values_cannot_become_site_css() {
        for value in [
            "red;stroke:blue",
            "url(https://example.com/a.svg)",
            "#fff}svg{fill:red",
        ] {
            let source = format!(
                "---\nconfig:\n  themeVariables:\n    primaryColor: '{value}'\n---\nflowchart TD\n A --> B"
            );
            assert!(render_to_svg(&source, &MermaidTheme::default()).is_err());
        }
    }

    #[test]
    fn mermaid_diagram_with_mixed_weight_combining_marks_does_not_panic() {
        const IBM_PLEX_REGULAR: &[u8] =
            include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");
        const IBM_PLEX_SEMIBOLD: &[u8] =
            include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf");

        let zalgo = "Ne\u{0301}\u{0302}\u{0303}\u{0304}\u{0306}\u{0307}\u{0308}\u{030a}d";
        let source = format!("flowchart TD\n  A[\"**{zalgo}** {zalgo}\"]");
        let svg = render_to_svg(&source, &MermaidTheme::default())
            .expect("mermaid diagram should render to SVG");

        let mut db = usvg::fontdb::Database::new();
        db.load_font_data(IBM_PLEX_REGULAR.to_vec());
        db.load_font_data(IBM_PLEX_SEMIBOLD.to_vec());
        db.set_sans_serif_family("IBM Plex Sans");
        let options = usvg::Options {
            fontdb: std::sync::Arc::new(db),
            ..Default::default()
        };

        usvg::Tree::from_data(svg.as_bytes(), &options)
            .expect("rasterizing mermaid text should not panic");
    }

    /// An ER diagram whose attribute-block tokens begin with a multibyte
    /// UTF-8 character (e.g. CJK type/field names) must not panic while the
    /// lexer probes for the two-character `PK`/`FK`/`UK` keys.
    #[test]
    fn er_multibyte_attribute_does_not_crash() {
        let source = "erDiagram\n顧客 {\n  文字列 名前\n}";
        let _ = render_to_svg(source, &MermaidTheme::default());
    }

    /// A flowchart with mutually nested subgraphs (`A` contains `B` and `B`
    /// contains `A`) is an invalid containment cycle. Rendering it must return
    /// gracefully rather than overflowing the stack and aborting the process.
    #[test]
    fn cyclic_subgraphs_do_not_crash() {
        let source = "flowchart TD\n  subgraph A\n    B\n  end\n  subgraph B\n    A\n  end";
        let result = render_to_svg(source, &MermaidTheme::default());
        if let Err(err) = result {
            let message = format!("{err:#}");
            assert!(
                message.contains("cycle"),
                "expected a cycle-related error, got: {message}"
            );
        }
    }
}

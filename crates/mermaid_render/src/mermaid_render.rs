#![recursion_limit = "256"]

mod postprocess;

use anyhow::{Context as _, Result, anyhow};
use gpui::{Hsla, Rgba};

#[derive(Debug, Clone, Copy)]
pub struct AccentColor {
    /// The accent stroke/border color.
    pub stroke: Hsla,
    /// The base background color from which fill and text colors are derived.
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
    pub accent_colors: Vec<AccentColor>,
}

/// A reasonable baseline theme for tests and benchmarks.
///
/// Production callers (the markdown crate, etc.) should always construct a
/// theme from Zed's active UI theme rather than relying on these defaults.
#[cfg(any(test, feature = "test-support"))]
impl Default for MermaidTheme {
    fn default() -> Self {
        use gpui::{hsla, rgb};
        // Values mirror the previous `mermaid_rs_renderer::Theme::modern()` defaults so existing
        // tests and call sites that relied on those colors continue to work.
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
        let git_branch_label_colors: [Hsla; 8] = {
            let w = Hsla::white();
            let k = Hsla::black();
            [w, k, k, w, k, k, k, k]
        };

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

pub fn render_to_svg(source: &str, theme: &MermaidTheme) -> Result<String> {
    render_with_merman(source, theme)
}

fn render_with_merman(source: &str, theme: &MermaidTheme) -> Result<String> {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let diagram_id = format!("merman-{id}");

    let config = to_merman_config(theme);
    let renderer = merman::render::HeadlessRenderer::new()
        .with_site_config(config)
        .with_vendored_text_measurer()
        .with_diagram_id(&diagram_id);

    let svg = renderer
        .render_svg_sync(source)
        .context("merman render failed")?
        .ok_or_else(|| anyhow!("merman returned no SVG for the given input"))?;

    postprocess::postprocess(&svg, theme)
}


fn to_merman_config(theme: &MermaidTheme) -> merman::MermaidConfig {
    let primary = css_color(theme.primary_color);
    let primary_text = css_color(theme.primary_text_color);
    let primary_border = css_color(theme.primary_border_color);
    let line = css_color(theme.line_color);
    let secondary = css_color(theme.secondary_color);
    let tertiary = css_color(theme.tertiary_color);
    let background = css_color(theme.background);
    let cluster_bg = css_color(theme.cluster_background);
    let cluster_border = css_color(theme.cluster_border);
    let edge_label_bg = css_color(theme.edge_label_background);
    let text = css_color(theme.text_color);
    let note_bg = css_color(theme.note_background);
    let note_border = css_color(theme.note_border);
    let actor_bg = css_color(theme.actor_background);
    let actor_border = css_color(theme.actor_border);
    let activation_bg = css_color(theme.activation_background);
    let activation_border = css_color(theme.activation_border);
    let er_odd = css_color(theme.er_attr_bg_odd);
    let er_even = css_color(theme.er_attr_bg_even);
    let git: [String; 8] = theme.git_branch_colors.map(css_color);
    let git_lbl: [String; 8] = theme.git_branch_label_colors.map(css_color);

    merman::MermaidConfig::from_value(serde_json::json!({
        "theme": "base",
        "darkMode": theme.dark_mode,
        "flowchart": {
            "padding": 16,
        },
        "themeVariables": {
            "primaryColor": primary,
            "primaryTextColor": primary_text,
            "primaryBorderColor": primary_border,
            "lineColor": line,
            "secondaryColor": secondary,
            "tertiaryColor": tertiary,
            "background": background,
            "mainBkg": primary,
            "nodeBorder": primary_border,
            "clusterBkg": cluster_bg,
            "clusterBorder": cluster_border,
            "titleColor": text,
            "edgeLabelBackground": edge_label_bg,
            "textColor": text,
            "fontFamily": theme.font_family,
            "noteBkgColor": note_bg,
            "noteBorderColor": note_border,
            "actorBkg": actor_bg,
            "actorBorder": actor_border,
            "actorTextColor": primary_text,
            "activationBkgColor": activation_bg,
            "activationBorderColor": activation_border,
            "attributeBackgroundColorOdd": er_odd,
            "attributeBackgroundColorEven": er_even,
            "cScale0": git[0],
            "cScale1": git[1],
            "cScale2": git[2],
            "cScale3": git[3],
            "cScale4": git[4],
            "cScale5": git[5],
            "cScale6": git[6],
            "cScale7": git[7],
            "cScaleLabel0": git_lbl[0],
            "cScaleLabel1": git_lbl[1],
            "cScaleLabel2": git_lbl[2],
            "cScaleLabel3": git_lbl[3],
            "cScaleLabel4": git_lbl[4],
            "cScaleLabel5": git_lbl[5],
            "cScaleLabel6": git_lbl[6],
            "cScaleLabel7": git_lbl[7],
            "pie1": git[0],
            "pie2": git[1],
            "pie3": git[2],
            "pie4": git[3],
            "pie5": git[4],
            "pie6": git[5],
            "pie7": git[6],
            "pie8": git[7],
            "pieTitleTextColor": text,
            "pieSectionTextColor": text,
            "pieLegendTextColor": text,
            "pieStrokeColor": primary_border,
            "pieOuterStrokeColor": primary_border,

            "quadrant1Fill": primary,
            "quadrant2Fill": primary,
            "quadrant3Fill": primary,
            "quadrant4Fill": primary,
            "quadrant1TextFill": text,
            "quadrant2TextFill": text,
            "quadrant3TextFill": text,
            "quadrant4TextFill": text,
            "quadrantPointFill": line,
            "quadrantPointTextFill": text,
            "quadrantTitleFill": text,
            "quadrantXAxisTextFill": text,
            "quadrantYAxisTextFill": text,
            "quadrantExternalBorderStrokeFill": primary_border,
            "quadrantInternalBorderStrokeFill": primary_border,
        }
    }))
}



use anyhow::{Context as _, Result, anyhow};

use crate::{MermaidTheme, css_color};

pub(super) fn render_mermaid(source: &str, theme: &MermaidTheme) -> Result<String> {
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

    Ok(svg)
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

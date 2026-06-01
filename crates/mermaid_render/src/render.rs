use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context as _, Result, anyhow};

use crate::{MermaidTheme, css_color};

pub(super) fn render_mermaid(source: &str, theme: &MermaidTheme) -> Result<String> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
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

    let mut theme_vars = serde_json::json!({
        "primaryColor": primary,
        "primaryTextColor": primary_text,
        "primaryBorderColor": primary_border,
        "lineColor": line,
        "secondaryColor": secondary,
        "secondaryTextColor": text,
        "tertiaryColor": tertiary,
        "tertiaryTextColor": text,
        "background": background,
        "mainBkg": primary,
        "nodeBorder": primary_border,
        "nodeTextColor": primary_text,
        "clusterBkg": cluster_bg,
        "clusterBorder": cluster_border,
        "titleColor": text,
        "edgeLabelBackground": edge_label_bg,
        "textColor": text,
        "fontFamily": theme.font_family,
        "noteBkgColor": note_bg,
        "noteBorderColor": note_border,
        "noteTextColor": text,
        "actorBkg": actor_bg,
        "actorBorder": actor_border,
        "actorTextColor": primary_text,
        "labelTextColor": text,
        "loopTextColor": text,
        "signalColor": text,
        "signalTextColor": text,
        "activationBkgColor": activation_bg,
        "activationBorderColor": activation_border,
        "classText": text,
        "labelColor": primary_text,
        "attributeBackgroundColorOdd": er_odd,
        "attributeBackgroundColorEven": er_even,
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
    });

    let map = theme_vars.as_object_mut().expect("just created as object");
    for i in 0..8 {
        map.insert(format!("cScale{i}"), git[i].clone().into());
        map.insert(format!("cScaleLabel{i}"), git_lbl[i].clone().into());
        map.insert(format!("pie{}", i + 1), git[i].clone().into());
    }

    merman::MermaidConfig::from_value(serde_json::json!({
        "theme": "base",
        "darkMode": theme.dark_mode,
        "fontFamily": theme.font_family,
        "flowchart": {
            "padding": 16,
        },
        "themeVariables": theme_vars,
    }))
}

use super::super::*;
use crate::sequence::{SequenceMathHeightMode, measure_sequence_math_label};

pub(super) struct SequenceKatexLabel {
    pub(super) html: String,
    pub(super) width: f64,
    pub(super) height: f64,
}

pub(super) fn sequence_katex_label(
    text: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    config: &merman_core::MermaidConfig,
    math_renderer: Option<&(dyn crate::math::MathRenderer + Send + Sync)>,
    height_mode: SequenceMathHeightMode,
) -> Option<SequenceKatexLabel> {
    if !text.contains("$$") {
        return None;
    }
    let renderer = math_renderer?;
    let (width, height) =
        measure_sequence_math_label(measurer, text, style, config, Some(renderer), height_mode)?;
    let html = renderer.render_sequence_html_label(text, config)?;
    let html = xhtml_fix_fragment(&merman_core::sanitize::sanitize_text(&html, config));
    Some(SequenceKatexLabel {
        html,
        width,
        height,
    })
}

pub(super) fn write_sequence_katex_foreign_object(
    out: &mut String,
    label: &SequenceKatexLabel,
    x: f64,
    y: f64,
) {
    let _ = write!(
        out,
        r#"<foreignObject height="{h}" width="{w}" x="{x}" y="{y}"><div style="width: fit-content;" xmlns="http://www.w3.org/1999/xhtml">{html}</div></foreignObject>"#,
        h = fmt(label.height),
        w = fmt(label.width),
        x = fmt(x),
        y = fmt(y),
        html = label.html,
    );
}

fn xhtml_fix_fragment(input: &str) -> String {
    input
        .replace("<br>", "<br />")
        .replace("<br/>", "<br />")
        .replace("<br >", "<br />")
        .replace("</br>", "<br />")
        .replace("</br/>", "<br />")
        .replace("</br />", "<br />")
        .replace("</br >", "<br />")
}

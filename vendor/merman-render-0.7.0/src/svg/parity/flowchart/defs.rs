//! Flowchart SVG defs and marker emission.

use std::fmt::Write as _;

use super::super::util::{escape_xml, escape_xml_display};
use super::{FlowchartRenderCtx, flowchart_resolve_stroke_for_marker};

pub(in crate::svg::parity::flowchart) struct FlowchartDefs<'a> {
    diagram_id: &'a str,
    extra_marker_colors: Vec<String>,
}

pub(in crate::svg::parity::flowchart) fn prepare_flowchart_defs<'a>(
    diagram_id: &'a str,
    ctx: &FlowchartRenderCtx<'_>,
) -> FlowchartDefs<'a> {
    FlowchartDefs {
        diagram_id,
        extra_marker_colors: collect_edge_marker_colors(ctx),
    }
}

impl FlowchartDefs<'_> {
    pub(in crate::svg::parity::flowchart) fn push_base_markers(&self, out: &mut String) {
        push_base_markers(out, self.diagram_id);
    }

    pub(in crate::svg::parity::flowchart) fn push_extra_markers(&self, out: &mut String) {
        push_extra_markers(out, self.diagram_id, &self.extra_marker_colors);
    }
}

fn push_base_markers(out: &mut String, diagram_id: &str) {
    let id = escape_xml(diagram_id);
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-pointEnd" class="marker flowchart-v2" viewBox="0 0 10 10" refX="5" refY="5" markerUnits="userSpaceOnUse" markerWidth="8" markerHeight="8" orient="auto"><path d="M 0 0 L 10 5 L 0 10 z" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-pointStart" class="marker flowchart-v2" viewBox="0 0 10 10" refX="4.5" refY="5" markerUnits="userSpaceOnUse" markerWidth="8" markerHeight="8" orient="auto"><path d="M 0 5 L 10 10 L 10 0 z" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-pointEnd-margin" class="marker flowchart-v2" viewBox="0 0 11.5 14" refX="11.5" refY="7" markerUnits="userSpaceOnUse" markerWidth="10.5" markerHeight="14" orient="auto"><path d="M 0 0 L 11.5 7 L 0 14 z" class="arrowMarkerPath" style="stroke-width: 0; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-pointStart-margin" class="marker flowchart-v2" viewBox="0 0 11.5 14" refX="1" refY="7" markerUnits="userSpaceOnUse" markerWidth="11.5" markerHeight="14" orient="auto"><polygon points="0,7 11.5,14 11.5,0" class="arrowMarkerPath" style="stroke-width: 0; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-circleEnd" class="marker flowchart-v2" viewBox="0 0 10 10" refX="11" refY="5" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><circle cx="5" cy="5" r="5" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-circleStart" class="marker flowchart-v2" viewBox="0 0 10 10" refX="-1" refY="5" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><circle cx="5" cy="5" r="5" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-circleEnd-margin" class="marker flowchart-v2" viewBox="0 0 10 10" refY="5" refX="12.25" markerUnits="userSpaceOnUse" markerWidth="14" markerHeight="14" orient="auto"><circle cx="5" cy="5" r="5" class="arrowMarkerPath" style="stroke-width: 0; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-circleStart-margin" class="marker flowchart-v2" viewBox="0 0 10 10" refX="-2" refY="5" markerUnits="userSpaceOnUse" markerWidth="14" markerHeight="14" orient="auto"><circle cx="5" cy="5" r="5" class="arrowMarkerPath" style="stroke-width: 0; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-crossEnd" class="marker cross flowchart-v2" viewBox="0 0 11 11" refX="12" refY="5.2" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><path d="M 1,1 l 9,9 M 10,1 l -9,9" class="arrowMarkerPath" style="stroke-width: 2; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-crossStart" class="marker cross flowchart-v2" viewBox="0 0 11 11" refX="-1" refY="5.2" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><path d="M 1,1 l 9,9 M 10,1 l -9,9" class="arrowMarkerPath" style="stroke-width: 2; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-crossEnd-margin" class="marker cross flowchart-v2" viewBox="0 0 15 15" refX="17.7" refY="7.5" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto"><path d="M 1,1 L 14,14 M 1,14 L 14,1" class="arrowMarkerPath" style="stroke-width: 2.5;"/></marker>"#,
        id.as_str()
    );
    let _ = write!(
        out,
        r#"<marker id="{}_flowchart-v2-crossStart-margin" class="marker cross flowchart-v2" viewBox="0 0 15 15" refX="-3.5" refY="7.5" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto"><path d="M 1,1 L 14,14 M 1,14 L 14,1" class="arrowMarkerPath" style="stroke-width: 2.5; stroke-dasharray: 1, 0;"/></marker>"#,
        id.as_str()
    );
}

fn marker_color_id(color: &str) -> String {
    // Mermaid's DOM marker id coloring logic (Mermaid@11.12.2) uses:
    // `strokeColor.replace(/[^\dA-Za-z]/g, '_')`
    //
    // Important: this does not trim whitespace. As a result, values like `" orange"` (leading
    // space captured from `style="...stroke: orange;..."`) produce a leading `_` in the color id,
    // which in turn yields a `__orange` suffix in the final marker id.
    let raw = color.trim_end_matches(';');
    if raw.trim().is_empty() {
        return String::new();
    }
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    out
}

#[inline]
pub(in crate::svg::parity::flowchart) fn write_flowchart_marker_id_xml(
    out: &mut String,
    diagram_id: &str,
    base: &str,
    color: Option<&str>,
) {
    let _ = write!(out, "{}", escape_xml_display(diagram_id));
    out.push('_');
    out.push_str(base);

    let Some(color) = color else {
        return;
    };
    let raw = color.trim_end_matches(';');
    if raw.trim().is_empty() {
        return;
    }
    out.push('_');
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
}

fn push_extra_markers(out: &mut String, diagram_id: &str, colors: &[String]) {
    for c in colors {
        let cid = marker_color_id(c);
        if cid.is_empty() {
            continue;
        }

        let _ = write!(
            out,
            r#"<marker id="{}_flowchart-v2-pointEnd_{}" class="marker flowchart-v2" viewBox="0 0 10 10" refX="5" refY="5" markerUnits="userSpaceOnUse" markerWidth="8" markerHeight="8" orient="auto"><path d="M 0 0 L 10 5 L 0 10 z" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;" stroke="{}" fill="{}"/></marker>"#,
            escape_xml(diagram_id),
            escape_xml(&cid),
            escape_xml_display(c.trim()),
            escape_xml_display(c.trim())
        );
    }
}

fn collect_edge_marker_colors(ctx: &FlowchartRenderCtx<'_>) -> Vec<String> {
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out: Vec<String> = Vec::new();

    for e in ctx.edges_by_id.values() {
        let mut found: Option<String> = None;
        for raw in ctx.default_edge_style.iter().chain(e.style.iter()) {
            // Mirror upstream behavior: `strokeColor` is extracted from `style="...stroke:...;..."`
            // without trimming, and then marker ids use `replace(/[^\dA-Za-z]/g, '_')`.
            //
            // Our style declarations may include a leading space (e.g. ` stroke: orange`), so we
            // only trim the key side.
            let s = raw.trim_start();
            let Some(rest) = s.strip_prefix("stroke:") else {
                continue;
            };
            let cid = marker_color_id(rest);
            if cid.is_empty() {
                continue;
            }
            if seen.insert(cid) {
                found = Some(rest.to_string());
            }
            break;
        }

        if found.is_none() && !e.classes.is_empty() {
            let stroke = flowchart_resolve_stroke_for_marker(
                ctx.class_defs,
                &e.classes,
                &ctx.default_edge_style,
                &e.style,
            );
            if let Some(stroke) = stroke {
                let cid = marker_color_id(&stroke);
                if !cid.is_empty() && seen.insert(cid) {
                    found = Some(stroke);
                }
            }
        }

        if let Some(v) = found {
            out.push(v);
        }
    }

    out.sort();
    out
}

//! Flowchart edge path renderer.

use super::super::defs::write_flowchart_marker_id_xml;
use super::super::*;

pub(in crate::svg::parity::flowchart) fn render_flowchart_edge_path(
    out: &mut String,
    ctx: &FlowchartRenderCtx<'_>,
    edge: &crate::flowchart::FlowEdge,
    origin_x: f64,
    origin_y: f64,
    scratch: &mut FlowchartEdgeDataPointsScratch,
    edge_cache: Option<&FxHashMap<&str, FlowchartEdgePathCacheEntry>>,
) {
    let trace_enabled = ctx
        .trace_edge_id
        .as_deref()
        .is_some_and(|id| id == edge.id.as_str());

    let cached_geom = (!trace_enabled)
        .then(|| {
            edge_cache
                .and_then(|m| m.get(edge.id.as_str()))
                .filter(|c| {
                    (c.origin_x - origin_x).abs() <= 1e-9 && (c.origin_y - origin_y).abs() <= 1e-9
                })
                .map(|c| &c.geom)
        })
        .flatten();

    let owned_geom = if cached_geom.is_none() {
        flowchart_compute_edge_path_geom(
            FlowchartEdgePathGeomRequest {
                ctx,
                edge,
                origin_x,
                origin_y,
                abs_top_transform: 0.0,
                trace_enabled,
                viewbox_current_bounds: None,
            },
            scratch,
        )
    } else {
        None
    };
    let (d, data_points_b64) = if let Some(g) = cached_geom {
        (g.d.as_str(), g.data_points_b64.as_str())
    } else {
        let Some(g) = owned_geom.as_ref() else {
            return;
        };
        (g.d.as_str(), g.data_points_b64.as_str())
    };

    let mut marker_color: Option<&str> = None;
    for raw in ctx.default_edge_style.iter().chain(edge.style.iter()) {
        // Mirror Mermaid@11.12.2: marker coloring uses the `stroke:` style capture without
        // trimming (see `edges.js` + `edgeMarker.ts`).
        let s = raw.trim_start();
        let Some(rest) = s.strip_prefix("stroke:") else {
            continue;
        };
        if !rest.trim().is_empty() {
            marker_color = Some(rest);
            break;
        }
    }

    // If no inline `stroke:` exists, Mermaid still colors markers based on class-derived stroke
    // styles (see `edges.js` `stylesFromClasses` + `edgeMarker.ts` `strokeColor` extraction).
    // We approximate this by compiling the edge styles using class defs and reusing the resulting
    // `stroke` value for the marker id suffix.
    let compiled_marker_color = if marker_color.is_none() && !edge.classes.is_empty() {
        flowchart_resolve_stroke_for_marker(
            ctx.class_defs,
            &edge.classes,
            &ctx.default_edge_style,
            &edge.style,
        )
    } else {
        None
    };
    if marker_color.is_none() {
        marker_color = compiled_marker_color.as_deref();
    }

    fn write_style_joined(out: &mut String, a: &[String], b: &[String]) {
        let mut first = true;
        for part in a.iter().chain(b.iter()) {
            if first {
                first = false;
            } else {
                out.push(';');
            }
            let _ = write!(out, "{}", escape_xml_display(part));
        }
    }

    let _ = write!(
        out,
        r#"<path d="{}" id="{}-{}" class=""#,
        d,
        escape_xml_display(ctx.diagram_id),
        escape_xml_display(&edge.id),
    );
    css::write_flowchart_edge_class_attr(out, edge);
    out.push_str(r#"" style=""#);
    if ctx.default_edge_style.is_empty() && edge.style.is_empty() {
        out.push(';');
    } else {
        scratch.style_escaped.clear();
        write_style_joined(
            &mut scratch.style_escaped,
            &ctx.default_edge_style,
            &edge.style,
        );
        out.push_str(&scratch.style_escaped);
        out.push_str(";;;");
        out.push_str(&scratch.style_escaped);
    }
    let _ = write!(
        out,
        r#"" data-edge="true" data-et="edge" data-id="{}" data-points="{}" data-look="{}""#,
        escape_xml_display(&edge.id),
        data_points_b64,
        escape_xml_display(flowchart_config_look(ctx.config)),
    );
    if let Some(base) = flowchart_edge_marker_start_base(edge) {
        out.push_str(r#" marker-start="url(#"#);
        write_flowchart_marker_id_xml(out, ctx.diagram_id, base, marker_color);
        out.push_str(r#")""#);
    }
    if let Some(base) = flowchart_edge_marker_end_base(edge) {
        out.push_str(r#" marker-end="url(#"#);
        write_flowchart_marker_id_xml(out, ctx.diagram_id, base, marker_color);
        out.push_str(r#")""#);
    }
    out.push_str(" />");
}

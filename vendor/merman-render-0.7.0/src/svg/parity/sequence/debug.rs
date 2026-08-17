use super::super::*;

pub(super) fn render_sequence_diagram_debug_svg(
    layout: &SequenceDiagramLayout,
    options: &SvgRenderOptions,
) -> String {
    let mut clusters = layout.clusters.clone();
    clusters.sort_by(|a, b| a.id.cmp(&b.id));

    let mut nodes = layout.nodes.clone();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut edges = layout.edges.clone();
    edges.sort_by(|a, b| a.id.cmp(&b.id));

    let bounds = compute_layout_bounds(&clusters, &nodes, &edges).unwrap_or(Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 100.0,
        max_y: 100.0,
    });
    let pad = options.viewbox_padding.max(0.0);
    let vb_min_x = bounds.min_x - pad;
    let vb_min_y = bounds.min_y - pad;
    let vb_w = (bounds.max_x - bounds.min_x) + pad * 2.0;
    let vb_h = (bounds.max_y - bounds.min_y) + pad * 2.0;

    let mut out = String::new();
    let _ = writeln!(
        &mut out,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}">"#,
        fmt(vb_min_x),
        fmt(vb_min_y),
        fmt(vb_w.max(1.0)),
        fmt(vb_h.max(1.0))
    );
    out.push_str(
        r#"<style>
 .cluster-box { fill: none; stroke: #4b5563; stroke-width: 1; }
 .cluster-title { fill: #111827; font-family: ui-sans-serif, system-ui, sans-serif; font-size: 12px; text-anchor: middle; dominant-baseline: middle; }
 .node-box { fill: none; stroke: #2563eb; stroke-width: 1; }
 .node-label { fill: #1f2937; font-family: ui-sans-serif, system-ui, sans-serif; font-size: 11px; text-anchor: middle; dominant-baseline: middle; }
 .edge { fill: none; stroke: #111827; stroke-width: 1; }
 .lifeline { stroke: #999; stroke-width: 0.5; }
 .message { stroke: #111827; stroke-width: 2; }
 .edge-label { fill: #111827; font-family: ui-sans-serif, system-ui, sans-serif; font-size: 11px; text-anchor: middle; dominant-baseline: middle; }
 .debug-cross { stroke: #ef4444; stroke-width: 1; }
</style>
"#,
    );
    out.push_str(
        r#"<defs><marker id="arrowhead" refX="7.9" refY="5" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto-start-reverse"><path d="M -1 0 L 10 5 L 0 10 z"/></marker></defs>
"#,
    );

    if options.include_clusters {
        out.push_str(r#"<g class="clusters">"#);
        for c in &clusters {
            render_cluster(&mut out, c, options.include_cluster_debug_markers);
        }
        out.push_str("</g>\n");
    }

    if options.include_edges {
        out.push_str(r#"<g class="edges">"#);
        for e in &edges {
            if e.points.len() >= 2 {
                if e.id.starts_with("lifeline-") && e.points.len() == 2 {
                    let p0 = &e.points[0];
                    let p1 = &e.points[1];
                    let _ = write!(
                        &mut out,
                        r#"<line class="edge lifeline" x1="{}" y1="{}" x2="{}" y2="{}" />"#,
                        fmt(p0.x),
                        fmt(p0.y),
                        fmt(p1.x),
                        fmt(p1.y)
                    );
                } else if e.id.starts_with("msg-") && e.points.len() == 2 {
                    let p0 = &e.points[0];
                    let p1 = &e.points[1];
                    let sign = if p1.x >= p0.x { 1.0 } else { -1.0 };
                    // Layout uses Mermaid-like endpoint offsets (to make arrowheads match later).
                    // For debug output, extend the line to the lifelines so it's easier to read.
                    let x1 = p0.x - sign * 1.0;
                    let x2 = p1.x + sign * 4.0;
                    let _ = write!(
                        &mut out,
                        r#"<line class="edge message" x1="{}" y1="{}" x2="{}" y2="{}" marker-end="url(#arrowhead)" />"#,
                        fmt(x1),
                        fmt(p0.y),
                        fmt(x2),
                        fmt(p1.y)
                    );
                } else {
                    out.push_str(r#"<polyline class="edge" points=""#);
                    push_points_attr(&mut out, &e.points);
                    out.push_str(r#"" />"#);
                }
            }
            if options.include_edge_id_labels {
                if let Some(lbl) = &e.label {
                    let _ = write!(
                        &mut out,
                        r#"<text class="edge-label" x="{}" y="{}">{}</text>"#,
                        fmt(lbl.x),
                        fmt(lbl.y),
                        escape_xml_display(&e.id)
                    );
                }
            }
        }
        out.push_str("</g>\n");
    }

    if options.include_nodes {
        out.push_str(r#"<g class="nodes">"#);
        for n in &nodes {
            if n.is_cluster {
                continue;
            }
            render_node(&mut out, n);
        }
        out.push_str("</g>\n");
    }

    out.push_str("</svg>\n");
    out
}

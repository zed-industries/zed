use super::super::*;
use super::geometry::node_left_top;

pub(super) struct ActorManBottomGlyphMetrics {
    pub(super) actor_height: f64,
    pub(super) label_box_height: f64,
}

struct ActorManGlyphContext<'a> {
    placement_class: &'a str,
    actor_type: &'a str,
    actor_id: &'a str,
    label: &'a str,
    idx: usize,
    actor_y: f64,
    cx: f64,
    cy: f64,
    label_y: f64,
    width: f64,
    height: f64,
    diagram_id: &'a str,
}

fn actor_man_participant_data_attrs(ctx: &ActorManGlyphContext<'_>) -> String {
    if ctx.placement_class != "actor-top" {
        return String::new();
    }
    format!(
        r#" data-et="participant" data-type="{}" data-id="{}""#,
        escape_attr(ctx.actor_type),
        escape_attr(ctx.actor_id)
    )
}

pub(super) fn write_actor_man_top_glyph(
    out: &mut String,
    actor_type: &str,
    actor_id: &str,
    label: &str,
    n: &LayoutNode,
    idx: usize,
    actor_height: f64,
    diagram_id: &str,
) {
    let (_, actor_y) = node_left_top(n);
    let cx = n.x;

    match actor_type {
        "actor" => write_stick_actor_glyph(
            out,
            &ActorManGlyphContext {
                placement_class: "actor-top",
                actor_type,
                actor_id,
                label,
                idx,
                actor_y,
                cx,
                cy: actor_y + 10.0,
                label_y: actor_y + actor_height + 2.5,
                width: n.width,
                height: actor_height,
                diagram_id,
            },
        ),
        "boundary" => {
            // drawTextCandidate adds rect.height/2. Top render uses the config height.
            let label_y = actor_y + 15.0 + (actor_height / 2.0);
            write_boundary_actor_glyph(
                out,
                &ActorManGlyphContext {
                    placement_class: "actor-top",
                    actor_type,
                    actor_id,
                    label,
                    idx,
                    actor_y,
                    cx,
                    cy: actor_y + 12.0,
                    label_y,
                    width: n.width,
                    height: actor_height,
                    diagram_id,
                },
            );
        }
        "control" => {
            let r = 22.0;
            let label_y = actor_y + r + 12.0 + (actor_height / 2.0);
            write_control_actor_glyph(
                out,
                &ActorManGlyphContext {
                    placement_class: "actor-top",
                    actor_type,
                    actor_id,
                    label,
                    idx,
                    actor_y,
                    cx,
                    cy: actor_y + 32.0,
                    label_y,
                    width: n.width,
                    height: actor_height,
                    diagram_id,
                },
            );
        }
        "entity" => {
            let cy = actor_y + 25.0;
            let label_y = actor_y + 30.0 + (actor_height / 2.0);
            write_entity_actor_glyph(
                out,
                &ActorManGlyphContext {
                    placement_class: "actor-top",
                    actor_type,
                    actor_id,
                    label,
                    idx,
                    actor_y,
                    cx,
                    cy,
                    label_y,
                    width: n.width,
                    height: actor_height,
                    diagram_id,
                },
            );
        }
        _ => {}
    }
}

pub(super) fn write_actor_man_bottom_glyph(
    out: &mut String,
    actor_type: &str,
    actor_id: &str,
    label: &str,
    n: &LayoutNode,
    idx: usize,
    metrics: ActorManBottomGlyphMetrics,
    diagram_id: &str,
) {
    let (_, actor_y) = node_left_top(n);
    let cx = n.x;

    match actor_type {
        "actor" => write_stick_actor_glyph(
            out,
            &ActorManGlyphContext {
                placement_class: "actor-bottom",
                actor_type,
                actor_id,
                label,
                idx,
                actor_y,
                cx,
                cy: actor_y + 10.0,
                label_y: actor_y + metrics.actor_height + 2.5,
                width: n.width,
                height: metrics.actor_height,
                diagram_id,
            },
        ),
        "boundary" => {
            let footer_h = 44.0 + metrics.label_box_height;
            let label_y = actor_y + 15.0 + (footer_h / 2.0);
            write_boundary_actor_glyph(
                out,
                &ActorManGlyphContext {
                    placement_class: "actor-bottom",
                    actor_type,
                    actor_id,
                    label,
                    idx,
                    actor_y,
                    cx,
                    cy: actor_y + 12.0,
                    label_y,
                    width: n.width,
                    height: footer_h,
                    diagram_id,
                },
            );
        }
        "control" => {
            let r = 22.0;
            let footer_h = 44.0 + 2.0 * metrics.label_box_height;
            let label_y = actor_y + r + 5.0 + (footer_h / 2.0);
            write_control_actor_glyph(
                out,
                &ActorManGlyphContext {
                    placement_class: "actor-bottom",
                    actor_type,
                    actor_id,
                    label,
                    idx,
                    actor_y,
                    cx,
                    cy: actor_y + 32.0,
                    label_y,
                    width: n.width,
                    height: footer_h,
                    diagram_id,
                },
            );
        }
        "entity" => {
            let cy = actor_y + 10.0;
            let footer_h = 44.0 + metrics.label_box_height;
            let label_y = actor_y + 15.0 + (footer_h / 2.0);
            write_entity_actor_glyph(
                out,
                &ActorManGlyphContext {
                    placement_class: "actor-bottom",
                    actor_type,
                    actor_id,
                    label,
                    idx,
                    actor_y,
                    cx,
                    cy,
                    label_y,
                    width: n.width,
                    height: footer_h,
                    diagram_id,
                },
            );
        }
        _ => {}
    }
}

fn write_stick_actor_glyph(out: &mut String, ctx: &ActorManGlyphContext<'_>) {
    let r = 15.0;
    let torso_top = ctx.cy + r;
    let torso_bottom = torso_top + 20.0;
    let arms_y = torso_top + 8.0;
    let arms_x1 = ctx.cx - 18.0;
    let arms_x2 = ctx.cx + 18.0;
    let leg_y = torso_bottom + 15.0;
    let data_attrs = actor_man_participant_data_attrs(ctx);
    let _ = write!(
        out,
        r##"<g class="actor-man {placement_class}" name="{name}"{data_attrs}><line id="actor-man-torso{idx}" x1="{cx}" y1="{y1}" x2="{cx}" y2="{y2}"/><line id="actor-man-arms{idx}" x1="{ax1}" y1="{ay}" x2="{ax2}" y2="{ay}"/><line x1="{ax1}" y1="{ly}" x2="{cx}" y2="{y2}"/><line x1="{cx}" y1="{y2}" x2="{lx2}" y2="{ly}"/><circle cx="{cx}" cy="{cy}" r="15" width="{w}" height="{h}"/><text x="{cx}" y="{ty}" dominant-baseline="central" alignment-baseline="central" class="actor actor-man" style="text-anchor: middle; font-size: 16px; font-weight: 400;"><tspan x="{cx}" dy="0">{label}</tspan></text></g>"##,
        placement_class = ctx.placement_class,
        name = escape_xml(ctx.actor_id),
        data_attrs = data_attrs,
        idx = ctx.idx,
        cx = fmt(ctx.cx),
        y1 = fmt(torso_top),
        y2 = fmt(torso_bottom),
        ax1 = fmt(arms_x1),
        ax2 = fmt(arms_x2),
        ay = fmt(arms_y),
        ly = fmt(leg_y),
        lx2 = fmt(ctx.cx + 16.0),
        cy = fmt(ctx.cy),
        w = fmt(ctx.width),
        h = fmt(ctx.height),
        ty = fmt(ctx.label_y),
        label = escape_xml(ctx.label)
    );
}

fn write_boundary_actor_glyph(out: &mut String, ctx: &ActorManGlyphContext<'_>) {
    let radius = 22.0;
    let x_left = ctx.cx - radius * 2.5;
    let data_attrs = actor_man_participant_data_attrs(ctx);
    let _ = write!(
        out,
        r##"<g class="actor-man {placement_class}" name="{name}" transform="translate(0,21)"{data_attrs}><line id="actor-man-torso{idx}" x1="{x1}" y1="{y_t}" x2="{x2}" y2="{y_t}"/><line id="actor-man-arms{idx}" x1="{x1}" y1="{y0}" x2="{x1}" y2="{y20}"/><circle cx="{cx}" cy="{cy}" r="22"/><text x="{cx}" y="{ty}" dominant-baseline="central" alignment-baseline="central" class="actor actor-man" style="text-anchor: middle; font-size: 16px; font-weight: 400;"><tspan x="{cx}" dy="0">{label}</tspan></text></g>"##,
        placement_class = ctx.placement_class,
        name = escape_xml(ctx.actor_id),
        data_attrs = data_attrs,
        idx = ctx.idx,
        x1 = fmt(x_left),
        x2 = fmt(ctx.cx - 15.0),
        y_t = fmt(ctx.actor_y + 10.0),
        y0 = fmt(ctx.actor_y + 0.0),
        y20 = fmt(ctx.actor_y + 20.0),
        cx = fmt(ctx.cx),
        cy = fmt(ctx.cy),
        ty = fmt(ctx.label_y),
        label = escape_xml(ctx.label)
    );
}

fn write_control_actor_glyph(out: &mut String, ctx: &ActorManGlyphContext<'_>) {
    let r = 22.0;
    let marker_id = scoped_svg_id(ctx.diagram_id, "filled-head-control");
    let marker_url = scoped_svg_url(ctx.diagram_id, "filled-head-control");
    let data_attrs = actor_man_participant_data_attrs(ctx);
    let _ = write!(
        out,
        r##"<g class="actor-man {placement_class}" name="{name}" style="stroke: rgb(147, 112, 219); fill: rgb(236, 236, 255);"{data_attrs}><defs><marker id="{marker_id}" refX="11" refY="5.8" markerWidth="20" markerHeight="28" orient="172.5" stroke-width="1.2"><path d="M 14.4 5.6 L 7.2 10.4 L 8.8 5.6 L 7.2 0.8 Z"/></marker></defs><circle cx="{cx}" cy="{cy}" r="22" filter=""/><line marker-end="{marker_url}" transform="translate({cx}, {ly})"/><text x="{cx}" y="{ty}" dominant-baseline="central" alignment-baseline="central" class="actor actor-man" style="text-anchor: middle; font-size: 16px; font-weight: 400;"><tspan x="{cx}" dy="0">{label}</tspan></text></g>"##,
        placement_class = ctx.placement_class,
        name = escape_xml(ctx.actor_id),
        data_attrs = data_attrs,
        marker_id = escape_attr(&marker_id),
        marker_url = escape_attr(&marker_url),
        cx = fmt(ctx.cx),
        cy = fmt(ctx.cy),
        ly = fmt(ctx.cy - r),
        ty = fmt(ctx.label_y),
        label = escape_xml(ctx.label)
    );
}

fn write_entity_actor_glyph(out: &mut String, ctx: &ActorManGlyphContext<'_>) {
    let r = 22.0;
    let transform_y = if ctx.placement_class == "actor-bottom" {
        22.0
    } else {
        6.0
    };
    let data_attrs = actor_man_participant_data_attrs(ctx);
    let _ = write!(
        out,
        r##"<g class="actor {placement_class}" name="{name}" transform="translate(0, {transform_y})"{data_attrs}><circle cx="{cx}" cy="{cy}" r="22" width="{w}" height="{h}"/><line x1="{x1}" x2="{x2}" y1="{y}" y2="{y}" stroke-width="2"/><text x="{cx}" y="{ty}" dominant-baseline="central" alignment-baseline="central" class="actor actor-man" style="text-anchor: middle; font-size: 16px; font-weight: 400;"><tspan x="{cx}" dy="0">{label}</tspan></text></g>"##,
        placement_class = ctx.placement_class,
        name = escape_xml(ctx.actor_id),
        data_attrs = data_attrs,
        transform_y = fmt(transform_y),
        cx = fmt(ctx.cx),
        cy = fmt(ctx.cy),
        w = fmt(ctx.width),
        h = fmt(ctx.height),
        x1 = fmt(ctx.cx - r),
        x2 = fmt(ctx.cx + r),
        y = fmt(ctx.cy + r),
        ty = fmt(ctx.label_y),
        label = escape_xml(ctx.label)
    );
}

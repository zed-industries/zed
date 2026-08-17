use super::super::*;
use super::block_collection::AltSection;
use super::block_geometry::{
    frame_x_from_message_ids, message_ids_y_range, section_message_y_range, section_separator_ys,
};
use super::block_text::{
    LoopTextRenderContext, display_block_label, wrap_svg_text_lines, write_loop_text_lines,
    write_section_title_lines,
};
use crate::sequence::sequence_text_line_step_px;
use rustc_hash::FxHashMap;

pub(super) struct SequenceBlockRenderContext<'a> {
    pub(super) default_frame_x1: f64,
    pub(super) default_frame_x2: f64,
    pub(super) msg_endpoints: &'a FxHashMap<&'a str, (&'a str, &'a str)>,
    pub(super) actor_nodes_by_id: &'a FxHashMap<&'a str, &'a LayoutNode>,
    pub(super) edges_by_id: &'a FxHashMap<&'a str, &'a crate::model::LayoutEdge>,
    pub(super) nodes_by_id: &'a FxHashMap<&'a str, &'a LayoutNode>,
    pub(super) label_box_height: f64,
    pub(super) box_text_margin: f64,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) loop_text_style: &'a TextStyle,
}

impl<'a> SequenceBlockRenderContext<'a> {
    fn loop_text_context(&self) -> LoopTextRenderContext<'_> {
        LoopTextRenderContext::new(self.measurer, self.loop_text_style)
    }
}

fn write_control_structure_group_open(out: &mut String, control_id: &str) {
    let _ = write!(
        out,
        r#"<g data-et="control-structure" data-id="i{id}">"#,
        id = escape_attr(control_id)
    );
}

pub(super) fn write_block_frame(
    out: &mut String,
    frame_x1: f64,
    frame_x2: f64,
    frame_y1: f64,
    frame_y2: f64,
) {
    let _ = write!(
        out,
        r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y1}" class="loopLine"/>"#,
        x1 = fmt(frame_x1),
        x2 = fmt(frame_x2),
        y1 = fmt(frame_y1)
    );
    let _ = write!(
        out,
        r#"<line x1="{x2}" y1="{y1}" x2="{x2}" y2="{y2}" class="loopLine"/>"#,
        x2 = fmt(frame_x2),
        y1 = fmt(frame_y1),
        y2 = fmt(frame_y2)
    );
    let _ = write!(
        out,
        r#"<line x1="{x1}" y1="{y2}" x2="{x2}" y2="{y2}" class="loopLine"/>"#,
        x1 = fmt(frame_x1),
        x2 = fmt(frame_x2),
        y2 = fmt(frame_y2)
    );
    let _ = write!(
        out,
        r#"<line x1="{x1}" y1="{y1}" x2="{x1}" y2="{y2}" class="loopLine"/>"#,
        x1 = fmt(frame_x1),
        y1 = fmt(frame_y1),
        y2 = fmt(frame_y2)
    );
}

pub(super) fn write_block_label_box(out: &mut String, frame_x1: f64, frame_y1: f64, label: &str) {
    let x1 = frame_x1;
    let y1 = frame_y1;
    let x2 = x1 + 50.0;
    let y2 = y1 + 13.0;
    let y3 = y1 + 20.0;
    let x3 = x2 - 8.4;
    let _ = write!(
        out,
        r#"<polygon points="{x1},{y1} {x2},{y1} {x2},{y2} {x3},{y3} {x1},{y3}" class="labelBox"/>"#,
        x1 = fmt(x1),
        y1 = fmt(y1),
        x2 = fmt(x2),
        y2 = fmt(y2),
        x3 = fmt(x3),
        y3 = fmt(y3)
    );
    let label_cx = (x1 + 25.0).round();
    let label_cy = y1 + 13.0;
    let _ = write!(
        out,
        r#"<text x="{x}" y="{y}" text-anchor="middle" dominant-baseline="middle" alignment-baseline="middle" class="labelText" style="font-size: 16px; font-weight: 400;">{label}</text>"#,
        x = fmt(label_cx),
        y = fmt(label_cy),
        label = escape_xml(label)
    );
}

pub(super) fn render_simple_sequence_block(
    out: &mut String,
    control_id: &str,
    block_label: &str,
    raw_label: &str,
    message_ids: &[&str],
    ctx: &SequenceBlockRenderContext<'_>,
) {
    let Some((min_y, max_y)) = message_ids_y_range(
        message_ids.iter().copied(),
        ctx.edges_by_id,
        ctx.nodes_by_id,
        ctx.msg_endpoints,
        false,
    ) else {
        return;
    };

    let (frame_x1, frame_x2, _min_left) = frame_x_from_message_ids(
        message_ids.iter().copied(),
        ctx.msg_endpoints,
        ctx.actor_nodes_by_id,
        ctx.edges_by_id,
        ctx.nodes_by_id,
    )
    .unwrap_or((ctx.default_frame_x1, ctx.default_frame_x2, f64::INFINITY));

    let header_offset = if block_label == "break" {
        93.0
    } else if raw_label.trim().is_empty() {
        (79.0 - ctx.label_box_height).max(0.0)
    } else {
        79.0
    };
    let frame_y1 = min_y - header_offset;
    let frame_y2 = max_y + 10.0;

    write_control_structure_group_open(out, control_id);
    write_block_frame(out, frame_x1, frame_x2, frame_y1, frame_y2);
    write_block_label_box(out, frame_x1, frame_y1, block_label);
    let label_box_right = frame_x1 + 50.0;
    let text_x = (label_box_right + frame_x2) / 2.0;
    let text_y = frame_y1 + 18.0;
    let label = display_block_label(raw_label, true).unwrap_or_else(|| "\u{200B}".to_string());
    let max_w = (frame_x2 - label_box_right).max(0.0);
    let loop_text_ctx = ctx.loop_text_context();
    write_loop_text_lines(
        out,
        &loop_text_ctx,
        text_x,
        text_y,
        Some(max_w),
        &label,
        true,
    );
    out.push_str("</g>");
}

pub(super) fn render_sectioned_sequence_block(
    out: &mut String,
    control_id: &str,
    block_label: &str,
    sections: &[AltSection<'_>],
    adjust_header_for_wrap: bool,
    ctx: &SequenceBlockRenderContext<'_>,
) {
    if sections.is_empty() {
        return;
    }

    let Some((min_y, max_y)) = section_message_y_range(
        sections,
        ctx.edges_by_id,
        ctx.nodes_by_id,
        ctx.msg_endpoints,
        false,
    ) else {
        return;
    };

    let (frame_x1, frame_x2, _min_left) = frame_x_from_message_ids(
        sections.iter().flat_map(|s| s.message_ids.iter().copied()),
        ctx.msg_endpoints,
        ctx.actor_nodes_by_id,
        ctx.edges_by_id,
        ctx.nodes_by_id,
    )
    .unwrap_or((ctx.default_frame_x1, ctx.default_frame_x2, f64::INFINITY));

    let header_offset =
        section_header_offset(sections, frame_x1, frame_x2, adjust_header_for_wrap, ctx);
    let frame_y1 = min_y - header_offset;
    let frame_y2 = max_y + 10.0;

    write_control_structure_group_open(out, control_id);

    // frame
    write_block_frame(out, frame_x1, frame_x2, frame_y1, frame_y2);

    // separators (dashed)
    // Keep separator endpoints identical to the frame endpoints to match upstream
    // Mermaid output and avoid sub-pixel gaps at the frame border.
    let dash_x1 = frame_x1;
    let dash_x2 = frame_x2;
    let sep_ys = section_separator_ys(
        sections,
        min_y,
        ctx.edges_by_id,
        ctx.nodes_by_id,
        ctx.msg_endpoints,
    );
    for y in &sep_ys {
        let _ = write!(
            out,
            r#"<line x1="{x1}" y1="{y}" x2="{x2}" y2="{y}" class="loopLine" style="stroke-dasharray: 3, 3;"/>"#,
            x1 = fmt(dash_x1),
            x2 = fmt(dash_x2),
            y = fmt(*y)
        );
    }

    // label box + label text
    write_block_label_box(out, frame_x1, frame_y1, block_label);

    // section labels
    let label_box_right = frame_x1 + 50.0;
    let main_text_x = (label_box_right + frame_x2) / 2.0;
    let center_text_x = (frame_x1 + frame_x2) / 2.0;
    for (i, sec) in sections.iter().enumerate() {
        let Some(label_text) = display_block_label(sec.raw_label, i == 0) else {
            continue;
        };
        if i == 0 {
            let y = frame_y1 + 18.0;
            let max_w = (frame_x2 - label_box_right).max(0.0);
            let loop_text_ctx = ctx.loop_text_context();
            write_loop_text_lines(
                out,
                &loop_text_ctx,
                main_text_x,
                y,
                Some(max_w),
                &label_text,
                true,
            );
            continue;
        }
        let y = sep_ys.get(i - 1).copied().unwrap_or(frame_y1) + 18.0;
        let loop_text_ctx = ctx.loop_text_context();
        write_section_title_lines(out, &loop_text_ctx, center_text_x, y, None, &label_text);
    }

    out.push_str("</g>");
}

pub(super) fn render_critical_sequence_block(
    out: &mut String,
    control_id: &str,
    sections: &[AltSection<'_>],
    ctx: &SequenceBlockRenderContext<'_>,
) {
    if sections.is_empty() {
        return;
    }

    let Some((min_y, max_y)) = section_message_y_range(
        sections,
        ctx.edges_by_id,
        ctx.nodes_by_id,
        ctx.msg_endpoints,
        false,
    ) else {
        return;
    };

    let (mut frame_x1, frame_x2, min_left) = frame_x_from_message_ids(
        sections.iter().flat_map(|s| s.message_ids.iter().copied()),
        ctx.msg_endpoints,
        ctx.actor_nodes_by_id,
        ctx.edges_by_id,
        ctx.nodes_by_id,
    )
    .unwrap_or((ctx.default_frame_x1, ctx.default_frame_x2, f64::INFINITY));
    if sections.len() > 1 && min_left.is_finite() {
        // Mermaid's `critical` w/ `option` sections widens the frame to the left.
        frame_x1 = frame_x1.min(min_left - 9.0);
    }

    let header_offset = if sections
        .first()
        .is_some_and(|s| s.raw_label.trim().is_empty())
    {
        (79.0 - ctx.label_box_height).max(0.0)
    } else if sections.len() > 1 {
        // Mermaid does not apply the wrap height adjustment for multi-section
        // `critical` blocks (those with one or more `option` sections).
        79.0
    } else {
        // Mermaid's `adjustLoopHeightForWrap(...)` expands the header height when the
        // section label wraps to multiple lines. This affects the frame's top y.
        let label_text = display_block_label(sections[0].raw_label, true)
            .unwrap_or_else(|| "\u{200B}".to_string());
        let label_box_right = frame_x1 + 50.0;
        let max_w = (frame_x2 - label_box_right).max(0.0);
        let wrapped =
            wrap_svg_text_lines(&label_text, ctx.measurer, ctx.loop_text_style, Some(max_w));
        let extra_lines = wrapped.len().saturating_sub(1) as f64;
        let extra_per_line = (sequence_text_line_step_px(ctx.loop_text_style.font_size)
            - ctx.box_text_margin)
            .max(0.0);
        79.0 + extra_lines * extra_per_line
    };
    let frame_y1 = min_y - header_offset;
    let frame_y2 = max_y + 10.0;

    write_control_structure_group_open(out, control_id);

    // frame
    write_block_frame(out, frame_x1, frame_x2, frame_y1, frame_y2);

    // separators (dashed)
    let dash_x1 = frame_x1;
    let dash_x2 = frame_x2;
    let sep_ys = section_separator_ys(
        sections,
        min_y,
        ctx.edges_by_id,
        ctx.nodes_by_id,
        ctx.msg_endpoints,
    );
    for y in &sep_ys {
        let _ = write!(
            out,
            r#"<line x1="{x1}" y1="{y}" x2="{x2}" y2="{y}" class="loopLine" style="stroke-dasharray: 3, 3;"/>"#,
            x1 = fmt(dash_x1),
            x2 = fmt(dash_x2),
            y = fmt(*y)
        );
    }

    // label box + label text
    write_block_label_box(out, frame_x1, frame_y1, "critical");

    // section labels
    let label_box_right = frame_x1 + 50.0;
    let main_text_x = (label_box_right + frame_x2) / 2.0;
    let center_text_x = (frame_x1 + frame_x2) / 2.0;
    for (i, sec) in sections.iter().enumerate() {
        let Some(label_text) = display_block_label(sec.raw_label, i == 0) else {
            continue;
        };
        if i == 0 {
            let y = frame_y1 + 18.0;
            let max_w = (frame_x2 - label_box_right).max(0.0);
            let loop_text_ctx = ctx.loop_text_context();
            write_loop_text_lines(
                out,
                &loop_text_ctx,
                main_text_x,
                y,
                Some(max_w),
                &label_text,
                true,
            );
            continue;
        }
        let y = sep_ys.get(i - 1).copied().unwrap_or(frame_y1) + 18.0;
        let loop_text_ctx = ctx.loop_text_context();
        write_section_title_lines(out, &loop_text_ctx, center_text_x, y, None, &label_text);
    }

    out.push_str("</g>");
}

fn section_header_offset(
    sections: &[AltSection<'_>],
    frame_x1: f64,
    frame_x2: f64,
    adjust_header_for_wrap: bool,
    ctx: &SequenceBlockRenderContext<'_>,
) -> f64 {
    if sections
        .first()
        .is_some_and(|s| s.raw_label.trim().is_empty())
    {
        return (79.0 - ctx.label_box_height).max(0.0);
    }
    if !adjust_header_for_wrap {
        return 79.0;
    }

    let base = 79.0;
    let label_box_right = frame_x1 + 50.0;
    let max_w = (frame_x2 - label_box_right).max(0.0);
    let label =
        display_block_label(sections[0].raw_label, true).unwrap_or_else(|| "\u{200B}".to_string());
    let wrapped = wrap_svg_text_lines(&label, ctx.measurer, ctx.loop_text_style, Some(max_w));
    let extra_lines = wrapped.len().saturating_sub(1) as f64;
    let extra_per_line =
        (sequence_text_line_step_px(ctx.loop_text_style.font_size) - ctx.box_text_margin).max(0.0);
    base + extra_lines * extra_per_line
}

use super::activation::SequenceActivationState;
use super::constants::SEQUENCE_MESSAGE_WRAP_SLACK_FACTOR;
use super::metrics::{SequenceMathHeightMode, measure_sequence_label_for_layout};
use crate::math::MathRenderer;
use crate::model::{LayoutEdge, LayoutLabel, LayoutPoint};
use crate::text::{
    TextMeasurer, TextStyle, split_html_br_lines, wrap_label_like_mermaid_lines_floored_bbox,
};
use merman_core::MermaidConfig;
use merman_core::diagrams::sequence::SequenceMessage;

const LINETYPE_BIDIRECTIONAL_SOLID: i32 = 33;
const LINETYPE_BIDIRECTIONAL_DOTTED: i32 = 34;
const LINETYPE_CENTRAL_CONNECTION_REVERSE: i32 = 60;
const LINETYPE_CENTRAL_CONNECTION_DUAL: i32 = 61;
const CENTRAL_CONNECTION_BASE_OFFSET: f64 = 4.0;
const CENTRAL_CONNECTION_BIDIRECTIONAL_OFFSET: f64 = 6.0;

pub(super) struct SequenceMessageLayoutContext<'a> {
    pub(super) actor_index: &'a std::collections::HashMap<&'a str, usize>,
    pub(super) actor_centers_x: &'a [f64],
    pub(super) actor_widths: &'a [f64],
    pub(super) activation_state: &'a SequenceActivationState<'a>,
    pub(super) msg_idx: usize,
    pub(super) actor_width_min: f64,
    pub(super) box_margin: f64,
    pub(super) wrap_padding: f64,
    pub(super) message_text_line_height: f64,
    pub(super) message_step: f64,
    pub(super) msg_label_offset: f64,
    pub(super) message_font_size: f64,
    pub(super) message_width_scale: f64,
    pub(super) cursor_y: f64,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) msg_text_style: &'a TextStyle,
    pub(super) math_config: &'a MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
    pub(super) created_actor_index: Option<usize>,
    pub(super) destroyed_from_index: Option<usize>,
    pub(super) destroyed_to_index: Option<usize>,
    pub(super) actor_is_type_width_limited: &'a dyn Fn(&str) -> bool,
}

pub(super) struct SequenceMessageLayout<'a> {
    pub(super) edge: LayoutEdge,
    pub(super) from: &'a str,
    pub(super) to: &'a str,
    pub(super) from_x: f64,
    pub(super) to_x: f64,
    pub(super) line_y: f64,
    pub(super) cursor_step: f64,
    pub(super) is_self: bool,
}

pub(super) fn layout_sequence_message<'a>(
    msg: &'a SequenceMessage,
    ctx: SequenceMessageLayoutContext<'a>,
) -> Option<SequenceMessageLayout<'a>> {
    let (Some(from), Some(to)) = (msg.from.as_deref(), msg.to.as_deref()) else {
        return None;
    };
    let (Some(fi), Some(ti)) = (
        ctx.actor_index.get(from).copied(),
        ctx.actor_index.get(to).copied(),
    ) else {
        return None;
    };
    let from_x = ctx.actor_centers_x[fi];
    let to_x = ctx.actor_centers_x[ti];

    let (mut startx, mut stopx, is_arrow_to_right, is_arrow_to_activation) =
        initial_message_endpoints(from, to, from_x, to_x, &ctx);
    let adjust_value = |v: f64| if is_arrow_to_right { -v } else { v };
    startx += central_connection_offset(msg, is_arrow_to_right);

    let is_self = from == to;
    if is_self {
        stopx = startx;
    } else {
        if msg.activate && !is_arrow_to_activation {
            stopx += adjust_value(ctx.activation_state.width() / 2.0 - 1.0);
        }

        if !matches!(msg.message_type, 5 | 6) {
            stopx += adjust_value(3.0);
        }

        if matches!(msg.message_type, 33 | 34) {
            startx -= adjust_value(3.0);
        }
    }

    if !is_self {
        adjust_created_destroyed_actor_endpoints(EndpointAdjustmentRequest {
            from,
            to,
            from_x,
            to_x,
            from_idx: fi,
            to_idx: ti,
            startx: &mut startx,
            stopx: &mut stopx,
            ctx: &ctx,
        });
    }

    let text = msg.message_text();
    let bounded_width = (startx - stopx).abs().max(0.0);
    let is_math_message = text.contains("$$");
    let wrapped_text = wrapped_message_text(text, msg.wrap, is_math_message, bounded_width, &ctx);
    let effective_text = wrapped_text.as_deref().unwrap_or(text);

    let (line_y, label_base_y, cursor_step) =
        message_vertical_geometry(effective_text, is_math_message, &ctx);

    let x1 = startx;
    let x2 = stopx;
    let label = message_label(effective_text, is_math_message, x1, x2, label_base_y, &ctx);

    Some(SequenceMessageLayout {
        edge: LayoutEdge {
            id: format!("msg-{}", msg.id),
            from: from.to_string(),
            to: to.to_string(),
            from_cluster: None,
            to_cluster: None,
            points: vec![
                LayoutPoint { x: x1, y: line_y },
                LayoutPoint { x: x2, y: line_y },
            ],
            label,
            start_label_left: None,
            start_label_right: None,
            end_label_left: None,
            end_label_right: None,
            start_marker: None,
            end_marker: None,
            stroke_dasharray: None,
        },
        from,
        to,
        from_x,
        to_x,
        line_y,
        cursor_step,
        is_self,
    })
}

fn central_connection_offset(msg: &SequenceMessage, is_arrow_to_right: bool) -> f64 {
    let mut offset = 0.0;
    if matches!(
        msg.central_connection,
        LINETYPE_CENTRAL_CONNECTION_REVERSE | LINETYPE_CENTRAL_CONNECTION_DUAL
    ) {
        offset += CENTRAL_CONNECTION_BASE_OFFSET;
    }

    if matches!(
        msg.central_connection,
        LINETYPE_CENTRAL_CONNECTION_REVERSE | LINETYPE_CENTRAL_CONNECTION_DUAL
    ) && matches!(
        msg.message_type,
        LINETYPE_BIDIRECTIONAL_SOLID | LINETYPE_BIDIRECTIONAL_DOTTED
    ) {
        offset += if is_arrow_to_right {
            0.0
        } else {
            -CENTRAL_CONNECTION_BIDIRECTIONAL_OFFSET
        };
    }

    offset
}

fn initial_message_endpoints(
    from: &str,
    to: &str,
    from_x: f64,
    to_x: f64,
    ctx: &SequenceMessageLayoutContext<'_>,
) -> (f64, f64, bool, bool) {
    let (from_left, from_right) = ctx.activation_state.actor_bounds(from, from_x);
    let (to_left, to_right) = ctx.activation_state.actor_bounds(to, to_x);

    let is_arrow_to_right = from_left <= to_left;
    let startx = if is_arrow_to_right {
        from_right
    } else {
        from_left
    };
    let stopx = if is_arrow_to_right { to_left } else { to_right };
    let is_arrow_to_activation = (to_left - to_right).abs() > 2.0;
    (startx, stopx, is_arrow_to_right, is_arrow_to_activation)
}

struct EndpointAdjustmentRequest<'a, 'b> {
    from: &'a str,
    to: &'a str,
    from_x: f64,
    to_x: f64,
    from_idx: usize,
    to_idx: usize,
    startx: &'b mut f64,
    stopx: &'b mut f64,
    ctx: &'b SequenceMessageLayoutContext<'a>,
}

fn adjust_created_destroyed_actor_endpoints(req: EndpointAdjustmentRequest<'_, '_>) {
    // Mermaid adjusts creating/destroying messages so arrowheads land outside the actor box.
    const ACTOR_TYPE_WIDTH_HALF: f64 = 18.0;

    if req.ctx.created_actor_index == Some(req.ctx.msg_idx) {
        let adjustment = if (req.ctx.actor_is_type_width_limited)(req.to) {
            ACTOR_TYPE_WIDTH_HALF + 3.0
        } else {
            req.ctx.actor_widths[req.to_idx] / 2.0 + 3.0
        };
        if req.to_x < req.from_x {
            *req.stopx += adjustment;
        } else {
            *req.stopx -= adjustment;
        }
    } else if req.ctx.destroyed_from_index == Some(req.ctx.msg_idx) {
        let adjustment = if (req.ctx.actor_is_type_width_limited)(req.from) {
            ACTOR_TYPE_WIDTH_HALF
        } else {
            req.ctx.actor_widths[req.from_idx] / 2.0
        };
        if req.from_x < req.to_x {
            *req.startx += adjustment;
        } else {
            *req.startx -= adjustment;
        }
    } else if req.ctx.destroyed_to_index == Some(req.ctx.msg_idx) {
        let adjustment = if (req.ctx.actor_is_type_width_limited)(req.to) {
            ACTOR_TYPE_WIDTH_HALF + 3.0
        } else {
            req.ctx.actor_widths[req.to_idx] / 2.0 + 3.0
        };
        if req.to_x < req.from_x {
            *req.stopx += adjustment;
        } else {
            *req.stopx -= adjustment;
        }
    }
}

fn wrapped_message_text(
    text: &str,
    should_wrap: bool,
    is_math_message: bool,
    bounded_width: f64,
    ctx: &SequenceMessageLayoutContext<'_>,
) -> Option<String> {
    if text.is_empty() || !should_wrap || is_math_message {
        return None;
    }

    // Upstream wraps message labels to `max(boundedWidth + 2*wrapPadding, conf.width)`.
    // Our vendored bbox widths are slightly conservative for Sequence prose, so use the
    // same calibrated slack as the SVG emitter to keep cursor height and rendered lines in
    // lockstep without adding fixture-specific text rows.
    let wrap_w = (bounded_width + SEQUENCE_MESSAGE_WRAP_SLACK_FACTOR * ctx.wrap_padding)
        .max(ctx.actor_width_min)
        .max(1.0);
    let lines =
        wrap_label_like_mermaid_lines_floored_bbox(text, ctx.measurer, ctx.msg_text_style, wrap_w);
    Some(lines.join("<br>"))
}

fn message_vertical_geometry(
    effective_text: &str,
    is_math_message: bool,
    ctx: &SequenceMessageLayoutContext<'_>,
) -> (f64, f64, f64) {
    if effective_text.is_empty() {
        // Mermaid's `boundMessage(...)` uses the measured text bbox height. For empty labels
        // (trailing colon `Alice->Bob:`) the bbox height becomes 0, collapsing the extra
        // vertical offset and producing a much earlier message line.
        //
        // Our cursor model uses `message_step` (a typical 1-line height) as the baseline.
        // Shift the line up and only advance by `boxMargin` to match the upstream footer actor
        // placement and overall viewBox height.
        let line_y = ctx.cursor_y - (ctx.message_step - ctx.box_margin);
        return (line_y, ctx.cursor_y, ctx.box_margin);
    }

    if is_math_message {
        // Mermaid's `boundMessage(...)` uses `calculateMathMLDimensions(...)` for KaTeX and
        // skips the extra ordinary-text line-height bump. Our cursor model keeps `cursor_y`
        // one `message_step` ahead of Mermaid's internal vertical position, so translate back
        // to that base before applying the KaTeX total offset.
        let (_w, h) = measure_sequence_label_for_layout(
            ctx.measurer,
            effective_text,
            ctx.msg_text_style,
            ctx.math_config,
            ctx.math_renderer,
            SequenceMathHeightMode::Bound,
        );
        let base_y = ctx.cursor_y - ctx.message_step;
        let line_y = base_y + ctx.box_margin + h;
        return (line_y, line_y, ctx.box_margin + h);
    }

    // Mermaid's `boundMessage(...)` uses `common.splitBreaks(message)` to derive a
    // `lines` count and adjusts the message line y-position and cursor increment by the
    // per-line height. This applies both to explicit `<br>` breaks and to `wrap: true`
    // labels (which are wrapped via `wrapLabel(...)` and stored with `<br/>` separators).
    let lines = split_html_br_lines(effective_text).len().max(1);
    // Mermaid's `calculateTextDimensions(...).height` is consistently ~2px smaller per
    // line than the rendered `drawText(...)` getBBox, so use a bbox-like per-line height
    // for the cursor math here.
    let extra = (lines.saturating_sub(1) as f64) * ctx.message_text_line_height;
    (ctx.cursor_y + extra, ctx.cursor_y, ctx.message_step + extra)
}

fn message_label(
    effective_text: &str,
    is_math_message: bool,
    x1: f64,
    x2: f64,
    label_base_y: f64,
    ctx: &SequenceMessageLayoutContext<'_>,
) -> Option<LayoutLabel> {
    if effective_text.is_empty() {
        // Mermaid renders an (empty) message text node even when the label is empty (e.g.
        // trailing colon `Alice->Bob:`). Keep a placeholder label to preserve DOM structure.
        return Some(LayoutLabel {
            x: ((x1 + x2) / 2.0).round(),
            y: (label_base_y - ctx.msg_label_offset).round(),
            width: 1.0,
            height: ctx.message_font_size.max(1.0),
        });
    }

    let (w, h) = measure_sequence_label_for_layout(
        ctx.measurer,
        effective_text,
        ctx.msg_text_style,
        ctx.math_config,
        ctx.math_renderer,
        if is_math_message {
            SequenceMathHeightMode::Draw
        } else {
            SequenceMathHeightMode::Bound
        },
    );
    Some(LayoutLabel {
        x: ((x1 + x2) / 2.0).round(),
        y: (label_base_y - ctx.msg_label_offset).round(),
        width: (w * ctx.message_width_scale).max(1.0),
        height: h.max(1.0),
    })
}

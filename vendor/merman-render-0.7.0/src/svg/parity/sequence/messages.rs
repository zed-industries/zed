use super::super::*;
use super::math_label::{sequence_katex_label, write_sequence_katex_foreign_object};
use super::model::{SequenceSvgMessagePayload, SequenceSvgModel};
use crate::sequence::{
    SEQUENCE_MESSAGE_WRAP_SLACK_FACTOR, SequenceMathHeightMode, sequence_activation_stack_bounds,
    sequence_activation_start_x, sequence_text_line_step_px,
};
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;

const LINETYPE_NOTE: i32 = 2;
const LINETYPE_ACTIVE_START: i32 = 17;
const LINETYPE_ACTIVE_END: i32 = 18;
const LINETYPE_AUTONUMBER: i32 = 26;
const LINETYPE_CENTRAL_CONNECTION: i32 = 59;
const LINETYPE_CENTRAL_CONNECTION_REVERSE: i32 = 60;
const LINETYPE_CENTRAL_CONNECTION_DUAL: i32 = 61;
const CENTRAL_CONNECTION_CIRCLE_OFFSET: f64 = 16.5;

pub(super) struct SequenceMessageRenderContext<'a> {
    pub(super) model: &'a SequenceSvgModel,
    pub(super) nodes_by_id: &'a FxHashMap<&'a str, &'a LayoutNode>,
    pub(super) edges_by_id: &'a FxHashMap<&'a str, &'a crate::model::LayoutEdge>,
    pub(super) sanitize_config: &'a merman_core::MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn crate::math::MathRenderer + Send + Sync)>,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) message_align: &'a str,
    pub(super) diagram_id: &'a str,
    pub(super) actor_height: f64,
    pub(super) actor_label_font_size: f64,
    pub(super) sequence_width: f64,
    pub(super) activation_width: f64,
    pub(super) wrap_padding: f64,
    pub(super) right_angles: bool,
    pub(super) loop_text_style: &'a TextStyle,
}

fn marker_attr(attr_name: &str, diagram_id: &str, local_id: &str) -> String {
    format!(
        r#" {attr_name}="{}""#,
        escape_attr(&scoped_svg_url(diagram_id, local_id))
    )
}

fn message_data_attrs(msg_id: &str, from: &str, to: &str) -> String {
    format!(
        r#" data-et="message" data-id="i{msg_id}" data-from="{}" data-to="{}""#,
        escape_attr(from),
        escape_attr(to)
    )
}

fn has_central_connection(msg: &merman_core::diagrams::sequence::SequenceMessage) -> bool {
    matches!(
        msg.central_connection,
        LINETYPE_CENTRAL_CONNECTION
            | LINETYPE_CENTRAL_CONNECTION_REVERSE
            | LINETYPE_CENTRAL_CONNECTION_DUAL
    )
}

fn is_reverse_arrow_type(msg_type: i32) -> bool {
    matches!(msg_type, 45 | 46 | 47 | 48 | 55 | 56 | 57 | 58)
}

fn actor_center_x(ctx: &SequenceMessageRenderContext<'_>, actor_id: &str) -> Option<f64> {
    ctx.nodes_by_id
        .get(format!("actor-top-{actor_id}").as_str())
        .map(|node| node.x)
}

struct SequenceAutonumberActivationBounds {
    width: f64,
    stacks: BTreeMap<String, Vec<f64>>,
}

impl SequenceAutonumberActivationBounds {
    fn new(width: f64) -> Self {
        Self {
            width,
            stacks: BTreeMap::new(),
        }
    }

    fn handle_directive(
        &mut self,
        msg: &merman_core::diagrams::sequence::SequenceMessage,
        ctx: &SequenceMessageRenderContext<'_>,
    ) -> bool {
        match msg.message_type {
            LINETYPE_ACTIVE_START => {
                let Some(actor_id) = msg.from.as_deref() else {
                    return true;
                };
                let Some(cx) = actor_center_x(ctx, actor_id) else {
                    return true;
                };
                let stack = self.stacks.entry(actor_id.to_string()).or_default();
                let stacked_size = stack.len();
                let startx = sequence_activation_start_x(cx, stacked_size, self.width);
                stack.push(startx);
                true
            }
            LINETYPE_ACTIVE_END => {
                let Some(actor_id) = msg.from.as_deref() else {
                    return true;
                };
                if let Some(stack) = self.stacks.get_mut(actor_id) {
                    let _ = stack.pop();
                }
                true
            }
            _ => false,
        }
    }

    fn actor_bounds(&self, actor_id: &str, center_x: f64) -> (f64, f64) {
        sequence_activation_stack_bounds(
            self.stacks
                .get(actor_id)
                .into_iter()
                .flat_map(|stack| stack.iter().copied()),
            center_x,
            self.width,
        )
    }
}

fn sequence_number_marker_x(
    activation_bounds: &SequenceAutonumberActivationBounds,
    ctx: &SequenceMessageRenderContext<'_>,
    msg: &merman_core::diagrams::sequence::SequenceMessage,
    from: &str,
    to: &str,
    startx: f64,
    stopx: f64,
) -> Option<f64> {
    let from_center = actor_center_x(ctx, from)?;
    let to_center = actor_center_x(ctx, to)?;
    let (from_left, from_right) = activation_bounds.actor_bounds(from, from_center);
    let (to_left, to_right) = activation_bounds.actor_bounds(to, to_center);
    let from_bounds = from_left.min(from_right).min(to_left).min(to_right);
    let to_bounds = from_left.max(from_right).max(to_left).max(to_right);
    let is_self_message = (startx - stopx).abs() <= f64::EPSILON;
    let is_left_to_right = startx <= stopx;

    Some(if is_self_message {
        from_bounds + 1.0
    } else if is_reverse_arrow_type(msg.message_type) {
        if is_left_to_right {
            to_bounds - 1.0
        } else {
            from_bounds + 1.0
        }
    } else if is_left_to_right {
        from_bounds + 1.0
    } else {
        to_bounds - 1.0
    })
}

fn write_central_connection_circles(
    out: &mut String,
    ctx: &SequenceMessageRenderContext<'_>,
    msg: &merman_core::diagrams::sequence::SequenceMessage,
    from: &str,
    to: &str,
    line_y: f64,
    sequence_number_visible: bool,
) {
    if !has_central_connection(msg) {
        return;
    }

    let (Some(mut from_center), Some(mut to_center)) =
        (actor_center_x(ctx, from), actor_center_x(ctx, to))
    else {
        return;
    };
    let is_left_to_right = from_center <= to_center;
    let is_reverse = is_reverse_arrow_type(msg.message_type);
    let circle_offset = |is_left_to_right: bool, is_reverse: bool| {
        let base_offset = if is_left_to_right {
            CENTRAL_CONNECTION_CIRCLE_OFFSET
        } else {
            -CENTRAL_CONNECTION_CIRCLE_OFFSET
        };
        if is_reverse {
            -base_offset
        } else {
            base_offset
        }
    };

    if sequence_number_visible {
        match msg.central_connection {
            LINETYPE_CENTRAL_CONNECTION => {
                if is_reverse {
                    to_center += circle_offset(is_left_to_right, true);
                }
            }
            LINETYPE_CENTRAL_CONNECTION_REVERSE => {
                if !is_reverse {
                    from_center += circle_offset(is_left_to_right, false);
                }
            }
            LINETYPE_CENTRAL_CONNECTION_DUAL => {
                if is_reverse {
                    to_center += circle_offset(is_left_to_right, true);
                } else {
                    from_center += circle_offset(is_left_to_right, false);
                }
            }
            _ => {}
        }
    }

    out.push_str("<g>");
    if matches!(
        msg.central_connection,
        LINETYPE_CENTRAL_CONNECTION_REVERSE | LINETYPE_CENTRAL_CONNECTION_DUAL
    ) {
        let _ = write!(
            out,
            r#"<circle cx="{cx}" cy="{cy}" r="5" width="10" height="10"/>"#,
            cx = fmt(from_center),
            cy = fmt(line_y)
        );
    }
    if matches!(
        msg.central_connection,
        LINETYPE_CENTRAL_CONNECTION | LINETYPE_CENTRAL_CONNECTION_DUAL
    ) {
        let _ = write!(
            out,
            r#"<circle cx="{cx}" cy="{cy}" r="5" width="10" height="10"/>"#,
            cx = fmt(to_center),
            cy = fmt(line_y)
        );
    }
    out.push_str("</g>");
}

pub(super) fn render_sequence_messages(out: &mut String, ctx: &SequenceMessageRenderContext<'_>) {
    let mut sequence_number_visible = false;
    let mut sequence_number = 1.0;
    let mut sequence_number_step = 1.0;
    let mut activation_bounds = SequenceAutonumberActivationBounds::new(ctx.activation_width);

    for _ in ctx.model.messages.iter().filter(|msg| {
        matches!(
            msg.message_type,
            LINETYPE_CENTRAL_CONNECTION | LINETYPE_CENTRAL_CONNECTION_REVERSE
        )
    }) {
        out.push_str("<g/>");
    }

    for msg in &ctx.model.messages {
        match msg.message_type {
            LINETYPE_AUTONUMBER => {
                if let SequenceSvgMessagePayload::Autonumber(autonumber) = &msg.message {
                    sequence_number_visible = autonumber.visible;
                    if let Some(start) = autonumber.start {
                        sequence_number = start;
                    }
                    if let Some(step) = autonumber.step {
                        sequence_number_step = step;
                    }
                }
                continue;
            }
            LINETYPE_ACTIVE_START | LINETYPE_ACTIVE_END => {
                let _ = activation_bounds.handle_directive(msg, ctx);
                continue;
            }
            LINETYPE_NOTE => continue,
            // CENTRAL_CONNECTION / CENTRAL_CONNECTION_REVERSE. Upstream routes these through
            // the activation drawing path, which leaves an empty group even without a visible
            // activation rectangle.
            LINETYPE_CENTRAL_CONNECTION | LINETYPE_CENTRAL_CONNECTION_REVERSE => continue,
            _ => {}
        }

        let (Some(from), Some(to)) = (msg.from.as_deref(), msg.to.as_deref()) else {
            continue;
        };
        let edge_id = format!("msg-{}", msg.id);
        let Some(edge) = ctx.edges_by_id.get(edge_id.as_str()).copied() else {
            continue;
        };
        if edge.points.len() < 2 {
            continue;
        }

        let p0 = &edge.points[0];
        let p1 = &edge.points[1];
        let sequence_number_x = if sequence_number_visible {
            sequence_number_marker_x(&activation_bounds, ctx, msg, from, to, p0.x, p1.x)
                .unwrap_or(p0.x)
        } else {
            p0.x
        };

        let text = msg.message_text();
        if let Some(lbl) = &edge.label {
            let line_step = sequence_text_line_step_px(ctx.actor_label_font_size);
            let bounded_width = (p0.x - p1.x).abs().max(0.0);
            // Mermaid aligns message label text based on `sequence.messageAlign`.
            let (label_x, label_anchor) = match ctx.message_align {
                "right" => (p1.x - 10.0, "end"),
                "left" => (p0.x + 10.0, "start"),
                _ => (lbl.x, "middle"),
            };
            if let Some(katex) = sequence_katex_label(
                text,
                ctx.measurer,
                ctx.loop_text_style,
                ctx.sanitize_config,
                ctx.math_renderer,
                SequenceMathHeightMode::Draw,
            ) {
                let center_x = (p0.x + p1.x) / 2.0;
                write_sequence_katex_foreign_object(
                    out,
                    &katex,
                    (center_x - katex.width / 2.0).round(),
                    (p0.y - katex.height).round(),
                );
            } else if msg.wrap && !text.is_empty() {
                // Mermaid's `wrapLabel(...)` uses DOM-backed SVG text bbox widths. Our headless
                // vendored metrics are close but can be slightly more conservative in some edge
                // cases; give message wrapping a bit of extra horizontal slack so line breaks match
                // upstream Cypress baselines.
                let wrap_w = (bounded_width
                    + SEQUENCE_MESSAGE_WRAP_SLACK_FACTOR * ctx.wrap_padding)
                    .max(ctx.sequence_width)
                    .max(1.0);
                let raw_lines = crate::text::wrap_label_like_mermaid_lines_floored_bbox(
                    text,
                    ctx.measurer,
                    ctx.loop_text_style,
                    wrap_w,
                );
                render_sequence_message_text_lines(
                    out,
                    raw_lines.iter().map(String::as_str),
                    lbl.y,
                    label_x,
                    label_anchor,
                    line_step,
                    ctx.actor_label_font_size,
                );
            } else {
                render_sequence_message_text_lines(
                    out,
                    crate::text::split_html_br_lines(text),
                    lbl.y,
                    label_x,
                    label_anchor,
                    line_step,
                    ctx.actor_label_font_size,
                );
            }
        }

        let class = match msg.message_type {
            1 | 4 | 6 | 25 | 34 => "messageLine1",
            _ => "messageLine0",
        };
        let style = match msg.message_type {
            1 | 4 | 6 | 25 | 34 => r#" style="stroke-dasharray: 3, 3; fill: none;""#,
            _ => r#" style="fill: none;""#,
        };

        let marker_start = match msg.message_type {
            33 | 34 => Some(marker_attr("marker-start", ctx.diagram_id, "arrowhead")),
            _ => None,
        };
        let marker_end = match msg.message_type {
            // open arrow variants: no marker.
            5 | 6 => None,
            // cross arrow variants
            3 | 4 => Some(marker_attr("marker-end", ctx.diagram_id, "crosshead")),
            // filled-head variants
            24 | 25 => Some(marker_attr("marker-end", ctx.diagram_id, "filled-head")),
            // default arrowhead variants
            _ => Some(marker_attr("marker-end", ctx.diagram_id, "arrowhead")),
        };
        let data_attrs = message_data_attrs(&msg.id, from, to);

        // Mermaid uses `stroke="none"` and assigns actual stroke via CSS.
        if from == to {
            let startx = p0.x;
            let y = p0.y;
            let d = if ctx.right_angles {
                let actor_w = ctx
                    .nodes_by_id
                    .get(format!("actor-top-{from}").as_str())
                    .map(|n| n.width)
                    .unwrap_or(ctx.actor_height);
                let text_dx = edge.label.as_ref().map(|l| l.width / 2.0).unwrap_or(0.0);
                let dx = (actor_w / 2.0).max(text_dx);
                format!(
                    "M  {x},{y} H {hx} V {vy} H {x}",
                    x = fmt(startx),
                    y = fmt(y),
                    hx = fmt(startx + dx),
                    vy = fmt(y + 25.0)
                )
            } else {
                format!(
                    "M {x},{y} C {x2},{y2} {x2},{y3} {x},{y4}",
                    x = fmt(startx),
                    y = fmt(y),
                    x2 = fmt(startx + 60.0),
                    y2 = fmt(y - 10.0),
                    y3 = fmt(y + 30.0),
                    y4 = fmt(y + 20.0)
                )
            };
            // Mermaid attaches an `x1` attribute to autonumbered self-reference paths even
            // though the geometry lives in the `d` attribute.
            let path_x1 = if sequence_number_visible {
                Some(if marker_start.is_some() {
                    p0.x + 6.0
                } else {
                    p0.x
                })
            } else {
                None
            };
            let _ = write!(
                out,
                r#"<path d="{d}" class="{class}"{data_attrs} stroke-width="2" stroke="none"{marker_start}{marker_end}{x1}{style}/>"#,
                d = d,
                class = class,
                data_attrs = data_attrs,
                marker_start = marker_start.as_deref().unwrap_or(""),
                marker_end = marker_end.as_deref().unwrap_or(""),
                x1 = path_x1
                    .map(|x1| format!(r#" x1="{x1}""#, x1 = fmt(x1)))
                    .unwrap_or_default(),
                style = style
            );
            write_central_connection_circles(out, ctx, msg, from, to, y, sequence_number_visible);
        } else {
            let _ = write!(
                out,
                r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="{class}"{data_attrs} stroke-width="2" stroke="none"{marker_start}{marker_end}{style}/>"#,
                x1 = fmt(p0.x),
                y1 = fmt(p0.y),
                x2 = fmt(p1.x),
                y2 = fmt(p1.y),
                class = class,
                data_attrs = data_attrs,
                marker_start = marker_start.as_deref().unwrap_or(""),
                marker_end = marker_end.as_deref().unwrap_or(""),
                style = style
            );
            write_central_connection_circles(
                out,
                ctx,
                msg,
                from,
                to,
                p0.y,
                sequence_number_visible,
            );
        }

        if sequence_number_visible {
            let sequence_number_text = format_sequence_number(sequence_number);
            let font_size = if sequence_number_text.len() > 5 {
                "7px"
            } else if sequence_number_text.len() > 3 {
                "9px"
            } else {
                "12px"
            };
            let x = sequence_number_x;
            let y = p0.y;
            let _ = write!(
                out,
                r#"<line x1="{x}" y1="{y}" x2="{x}" y2="{y}" stroke-width="0" marker-start="{marker_start}"/>"#,
                x = fmt(x),
                y = fmt(y),
                marker_start = escape_attr(&scoped_svg_url(ctx.diagram_id, "sequencenumber")),
            );
            let _ = write!(
                out,
                r#"<text x="{x}" y="{y}" font-family="sans-serif" font-size="{font_size}" text-anchor="middle" class="sequenceNumber">{n}</text>"#,
                x = fmt(x),
                y = fmt(y + 4.0),
                n = sequence_number_text,
            );
            sequence_number = round_sequence_number(sequence_number + sequence_number_step);
        }

        let _ = (from, to);
    }
}

fn round_sequence_number(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn format_sequence_number(value: f64) -> String {
    if value.is_finite() {
        value.to_string()
    } else {
        String::new()
    }
}

fn render_sequence_message_text_lines<'a>(
    out: &mut String,
    raw_lines: impl IntoIterator<Item = &'a str>,
    label_y: f64,
    label_x: f64,
    label_anchor: &str,
    line_step: f64,
    actor_label_font_size: f64,
) {
    for (i, raw) in raw_lines.into_iter().enumerate() {
        let y = label_y + (i as f64) * line_step;
        let decoded = merman_core::entities::decode_mermaid_entities_to_unicode(raw);
        let line = if decoded.as_ref().is_empty() {
            "\u{200B}"
        } else {
            decoded.as_ref()
        };
        let _ = write!(
            out,
            r#"<text x="{x}" y="{y}" text-anchor="{anchor}" dominant-baseline="middle" alignment-baseline="middle" class="messageText" dy="1em" style="font-size: {fs}px; font-weight: 400;">{text}</text>"#,
            x = fmt(label_x.round()),
            y = fmt(y),
            anchor = label_anchor,
            fs = fmt(actor_label_font_size),
            text = escape_xml(line)
        );
    }
}

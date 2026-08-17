use super::constants::{
    SEQUENCE_FRAME_GEOM_PAD_PX, SEQUENCE_LEFT_OF_NOTE_FINAL_WRAP_SLACK_PX,
    SEQUENCE_LEFT_OF_NOTE_WIDTH_OVERFLOW_PX, SEQUENCE_LEFT_OF_NOTE_WRAP_WIDTH_SLACK_PX,
    SEQUENCE_NOTE_WRAP_SLACK_PX,
};
use super::metrics::{
    SequenceMathHeightMode, measure_sequence_label_for_layout, measure_svg_like_with_html_br,
};
use crate::math::MathRenderer;
use crate::model::LayoutNode;
use crate::text::{TextMeasurer, TextStyle, wrap_label_like_mermaid_lines_floored_bbox};
use merman_core::MermaidConfig;
use merman_core::diagrams::sequence::SequenceMessage;
use std::collections::HashMap;

const NOTE_SIDE_OFFSET_PX: f64 = 25.0;

pub(super) struct SequenceNoteLayoutContext<'a> {
    pub(super) actor_index: &'a HashMap<&'a str, usize>,
    pub(super) actor_centers_x: &'a [f64],
    pub(super) actor_widths: &'a [f64],
    pub(super) note_default_width: f64,
    pub(super) note_text_pad_total: f64,
    pub(super) note_top_offset: f64,
    pub(super) note_gap: f64,
    pub(super) cursor_y: f64,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) note_text_style: &'a TextStyle,
    pub(super) math_config: &'a MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
}

pub(super) struct SequenceNoteLayout {
    pub(super) node: LayoutNode,
    pub(super) rect_min_x: f64,
    pub(super) rect_max_x: f64,
    pub(super) rect_max_y: f64,
    pub(super) cursor_step: f64,
}

pub(super) fn layout_sequence_note(
    msg: &SequenceMessage,
    ctx: SequenceNoteLayoutContext<'_>,
) -> Option<SequenceNoteLayout> {
    let (Some(from), Some(to)) = (msg.from.as_deref(), msg.to.as_deref()) else {
        return None;
    };
    let (Some(fi), Some(ti)) = (
        ctx.actor_index.get(from).copied(),
        ctx.actor_index.get(to).copied(),
    ) else {
        return None;
    };
    let fx = ctx.actor_centers_x[fi];
    let tx = ctx.actor_centers_x[ti];

    let placement = msg.placement.unwrap_or(2);
    let (mut note_x, mut note_w) = initial_note_x_and_width(msg, placement, fx, tx, fi, &ctx);
    let text = msg.message_text();
    let is_math_note = text.contains("$$");
    let (text_w, h) = measure_note_text(NoteTextMeasureRequest {
        msg,
        placement,
        note_x: &mut note_x,
        note_w: &mut note_w,
        fx,
        text,
        is_math_note,
        ctx: &ctx,
    });

    // Mermaid's `buildNoteModel(...)` widens the note box when the text would overflow the
    // configured default width. This is observable in strict SVG XML baselines when the
    // note contains literal `<br ...>` markup that is *not* treated as a line break.
    let padded_w = (text_w + ctx.note_text_pad_total).round().max(1.0);
    if !msg.wrap || is_math_note {
        match placement {
            // leftOf / rightOf notes clamp width to fit label text.
            0 | 1 => {
                note_w = note_w.max(padded_w);
                if placement == 0 {
                    note_x = fx - NOTE_SIDE_OFFSET_PX - note_w;
                }
            }
            // over: only clamp when the note is over a single actor (`from == to`).
            _ => {
                if (fx - tx).abs() < 0.0001 {
                    note_w = note_w.max(padded_w);
                    note_x = fx - note_w / 2.0;
                }
            }
        }
    }

    let note_h = (h + ctx.note_text_pad_total).round().max(1.0);
    let note_y = (ctx.cursor_y - ctx.note_top_offset).round();

    Some(SequenceNoteLayout {
        node: LayoutNode {
            id: format!("note-{}", msg.id),
            x: note_x + note_w / 2.0,
            y: note_y + note_h / 2.0,
            width: note_w.max(1.0),
            height: note_h,
            is_cluster: false,
            label_width: None,
            label_height: None,
        },
        rect_min_x: note_x - SEQUENCE_FRAME_GEOM_PAD_PX,
        rect_max_x: note_x + note_w + SEQUENCE_FRAME_GEOM_PAD_PX,
        rect_max_y: note_y + note_h,
        cursor_step: note_h + ctx.note_gap,
    })
}

fn initial_note_x_and_width(
    msg: &SequenceMessage,
    placement: i32,
    fx: f64,
    tx: f64,
    from_idx: usize,
    ctx: &SequenceNoteLayoutContext<'_>,
) -> (f64, f64) {
    match placement {
        // leftOf
        0 => (
            fx - NOTE_SIDE_OFFSET_PX - ctx.note_default_width,
            ctx.note_default_width,
        ),
        // rightOf
        1 => (fx + NOTE_SIDE_OFFSET_PX, ctx.note_default_width),
        // over
        _ => {
            if (fx - tx).abs() < 0.0001 {
                // Mermaid's `buildNoteModel(...)` widens "over self" notes when `wrap: true`:
                //   noteModel.width = max(conf.width, fromActor.width)
                //
                // This is observable in upstream SVG baselines for participants with
                // type-driven widths (e.g. `queue`), where the note box matches the actor
                // width rather than the configured default `conf.width`.
                let mut w = ctx.note_default_width;
                if msg.wrap {
                    w = w.max(
                        ctx.actor_widths
                            .get(from_idx)
                            .copied()
                            .unwrap_or(ctx.note_default_width),
                    );
                }
                (fx - (w / 2.0), w)
            } else {
                let left = fx.min(tx) - NOTE_SIDE_OFFSET_PX;
                let right = fx.max(tx) + NOTE_SIDE_OFFSET_PX;
                let w = (right - left).max(ctx.note_default_width);
                (left, w)
            }
        }
    }
}

struct NoteTextMeasureRequest<'a, 'b> {
    msg: &'a SequenceMessage,
    placement: i32,
    note_x: &'b mut f64,
    note_w: &'b mut f64,
    fx: f64,
    text: &'a str,
    is_math_note: bool,
    ctx: &'b SequenceNoteLayoutContext<'a>,
}

pub(super) struct SequenceNoteFinalTextMeasure<'a> {
    pub(super) text: &'a str,
    pub(super) placement: i32,
    pub(super) note_width: f64,
    pub(super) note_text_pad_total: f64,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) note_text_style: &'a TextStyle,
    pub(super) math_config: &'a MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
}

pub(crate) fn sequence_note_final_wrapped_lines(
    text: &str,
    placement: i32,
    note_width: f64,
    note_text_pad_total: f64,
    measurer: &dyn TextMeasurer,
    note_text_style: &TextStyle,
) -> Vec<String> {
    let wrap_w = (note_width - note_text_pad_total).max(1.0);
    let wrap_slack = if placement == 0 {
        SEQUENCE_LEFT_OF_NOTE_FINAL_WRAP_SLACK_PX
    } else {
        SEQUENCE_NOTE_WRAP_SLACK_PX
    };
    wrap_label_like_mermaid_lines_floored_bbox(
        text,
        measurer,
        note_text_style,
        (wrap_w + wrap_slack).max(1.0),
    )
}

pub(super) fn measure_sequence_note_final_text(
    req: SequenceNoteFinalTextMeasure<'_>,
) -> (f64, f64) {
    let lines = sequence_note_final_wrapped_lines(
        req.text,
        req.placement,
        req.note_width,
        req.note_text_pad_total,
        req.measurer,
        req.note_text_style,
    );
    let wrapped = lines.join("<br/>");
    let (w, h) = measure_sequence_label_for_layout(
        req.measurer,
        &wrapped,
        req.note_text_style,
        req.math_config,
        req.math_renderer,
        SequenceMathHeightMode::Bound,
    );
    (w.max(0.0), h.max(0.0))
}

fn measure_note_text(req: NoteTextMeasureRequest<'_, '_>) -> (f64, f64) {
    if req.is_math_note {
        return measure_sequence_label_for_layout(
            req.ctx.measurer,
            req.text,
            req.ctx.note_text_style,
            req.ctx.math_config,
            req.ctx.math_renderer,
            SequenceMathHeightMode::Bound,
        );
    }

    if req.msg.wrap {
        // Mermaid Sequence notes are wrapped via `wrapLabel(...)`, then measured via SVG
        // bbox probes (not HTML wrapping). Model this by producing wrapped `<br/>` lines
        // and then measuring them.
        //
        // Important: Mermaid widens *leftOf* wrapped notes based on the initially wrapped
        // text width (+ margins) before re-wrapping to the final width. In Mermaid 11.15
        // `sequenceRenderer.ts`, that first `wrapLabel(...)` call uses `conf.width`
        // exactly for notes, not `conf.width - 2*wrapPadding` like messages.
        // Chromium can still report a saturated wrapped line a few pixels wider in
        // `calculateTextDimensions(...)`; reflect that bounded bbox overflow before
        // adding note margins.
        if req.placement == 0 {
            let init_lines = wrap_label_like_mermaid_lines_floored_bbox(
                req.text,
                req.ctx.measurer,
                req.ctx.note_text_style,
                req.ctx.note_default_width.max(1.0),
            );
            let init_wrapped = init_lines.join("<br/>");
            let (w, _h) = measure_svg_like_with_html_br(
                req.ctx.measurer,
                &init_wrapped,
                req.ctx.note_text_style,
            );
            let mut w0 = (w + SEQUENCE_LEFT_OF_NOTE_WRAP_WIDTH_SLACK_PX).max(0.0);
            if w0 >= req.ctx.note_default_width {
                w0 = w0.max(req.ctx.note_default_width + SEQUENCE_LEFT_OF_NOTE_WIDTH_OVERFLOW_PX);
            }
            // Mermaid (LEFTOF + wrap): `noteModel.width = max(conf.width, textWidth + 2*noteMargin)`.
            // Our note padding total is `2*noteMargin`/`2*wrapPadding` in the default config.
            *req.note_w = req
                .note_w
                .max((w0 + req.ctx.note_text_pad_total).round().max(1.0));
            *req.note_x = req.fx - NOTE_SIDE_OFFSET_PX - *req.note_w;
        }

        return measure_sequence_note_final_text(SequenceNoteFinalTextMeasure {
            text: req.text,
            placement: req.placement,
            note_width: *req.note_w,
            note_text_pad_total: req.ctx.note_text_pad_total,
            measurer: req.ctx.measurer,
            note_text_style: req.ctx.note_text_style,
            math_config: req.ctx.math_config,
            math_renderer: req.ctx.math_renderer,
        });
    }

    measure_svg_like_with_html_br(req.ctx.measurer, req.text, req.ctx.note_text_style)
}

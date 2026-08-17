use super::constants::{
    SEQUENCE_FRAME_GEOM_PAD_PX, SEQUENCE_FRAME_SIDE_PAD_PX, sequence_text_line_step_px,
};
use super::metrics::{SequenceMathHeightMode, measure_sequence_label_for_layout};
use crate::math::MathRenderer;
use crate::text::{TextMeasurer, TextStyle, WrapMode};
use merman_core::MermaidConfig;
use merman_core::diagrams::sequence::{SequenceDiagramRenderModel, SequenceMessage};
use std::collections::HashMap;

pub(super) struct BlockStepPlanContext<'a> {
    pub(super) model: &'a SequenceDiagramRenderModel,
    pub(super) actor_index: &'a HashMap<&'a str, usize>,
    pub(super) actor_centers_x: &'a [f64],
    pub(super) actor_widths: &'a [f64],
    pub(super) message_margin: f64,
    pub(super) box_margin: f64,
    pub(super) box_text_margin: f64,
    pub(super) bottom_margin_adj: f64,
    pub(super) label_box_height: f64,
    pub(super) message_font_size: f64,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) msg_text_style: &'a TextStyle,
    pub(super) math_config: &'a MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
    pub(super) message_width_scale: f64,
}

pub(super) fn plan_sequence_directive_steps(ctx: BlockStepPlanContext<'_>) -> HashMap<String, f64> {
    // Mermaid advances the "cursor" for sequence blocks (loop/alt/opt/par/break/critical) even
    // though these directives are not message edges. The cursor increment depends on the wrapped
    // block label height; precompute these increments per directive message id.
    // `adjustLoopHeightForWrap(...)` advances the Mermaid bounds cursor by:
    // - `preMargin` (either `boxMargin` or `boxMargin + boxTextMargin`)
    // - plus `heightAdjust`, where `heightAdjust` is:
    //   - `postMargin` when the block label is empty
    //   - `postMargin + max(labelTextHeight, labelBoxHeight)` when the label is present
    //
    // For the common 1-line label case, this reduces to:
    //   preMargin + postMargin + labelBoxHeight
    //
    // We model this as a base step and subtract `labelBoxHeight` for empty labels.
    let block_base_step =
        (2.0 * ctx.box_margin + ctx.box_text_margin + ctx.label_box_height).max(0.0);
    let block_base_step_empty = (block_base_step - ctx.label_box_height).max(0.0);
    let line_step = sequence_text_line_step_px(ctx.message_font_size);
    let block_extra_per_line = (line_step - ctx.box_text_margin).max(0.0);
    let block_end_step = SEQUENCE_FRAME_GEOM_PAD_PX;

    let mut msg_by_id: HashMap<&str, &SequenceMessage> = HashMap::new();
    for msg in &ctx.model.messages {
        msg_by_id.insert(msg.id.as_str(), msg);
    }

    let frame_ctx = BlockFrameWidthContext {
        msg_by_id: &msg_by_id,
        actor_index: ctx.actor_index,
        actor_centers_x: ctx.actor_centers_x,
        actor_widths: ctx.actor_widths,
        message_margin: ctx.message_margin,
        box_text_margin: ctx.box_text_margin,
        bottom_margin_adj: ctx.bottom_margin_adj,
        measurer: ctx.measurer,
        msg_text_style: ctx.msg_text_style,
        math_config: ctx.math_config,
        math_renderer: ctx.math_renderer,
        message_width_scale: ctx.message_width_scale,
    };
    let step_ctx = BlockStepContext {
        block_base_step,
        block_base_step_empty,
        block_extra_per_line,
        block_end_step,
    };

    let mut directive_steps: HashMap<String, f64> = HashMap::new();
    let mut stack: Vec<BlockStackEntry> = Vec::new();

    for msg in &ctx.model.messages {
        let raw_label = msg.message_text();
        match msg.message_type {
            // loop start/end
            10 => stack.push(BlockStackEntry::Loop {
                start_id: msg.id.clone(),
                raw_label: raw_label.to_string(),
                messages: Vec::new(),
            }),
            11 => {
                if let Some(BlockStackEntry::Loop {
                    start_id,
                    raw_label,
                    messages,
                }) = stack.pop()
                {
                    let end_step = finish_single_block(
                        &mut directive_steps,
                        start_id,
                        raw_label,
                        messages,
                        frame_ctx,
                        step_ctx,
                    );
                    directive_steps.insert(msg.id.clone(), end_step);
                }
            }
            // opt start/end
            15 => stack.push(BlockStackEntry::Opt {
                start_id: msg.id.clone(),
                raw_label: raw_label.to_string(),
                messages: Vec::new(),
            }),
            16 => {
                let mut end_step = block_end_step;
                if let Some(BlockStackEntry::Opt {
                    start_id,
                    raw_label,
                    messages,
                }) = stack.pop()
                {
                    end_step = finish_single_block(
                        &mut directive_steps,
                        start_id,
                        raw_label,
                        messages,
                        frame_ctx,
                        step_ctx,
                    );
                }
                directive_steps.insert(msg.id.clone(), end_step);
            }
            // break start/end
            30 => stack.push(BlockStackEntry::Break {
                start_id: msg.id.clone(),
                raw_label: raw_label.to_string(),
                messages: Vec::new(),
            }),
            31 => {
                let mut end_step = block_end_step;
                if let Some(BlockStackEntry::Break {
                    start_id,
                    raw_label,
                    messages,
                }) = stack.pop()
                {
                    end_step = finish_single_block(
                        &mut directive_steps,
                        start_id,
                        raw_label,
                        messages,
                        frame_ctx,
                        step_ctx,
                    );
                }
                directive_steps.insert(msg.id.clone(), end_step);
            }
            // alt start/else/end
            12 => stack.push(BlockStackEntry::Alt {
                section_directives: vec![(msg.id.clone(), raw_label.to_string())],
                sections: vec![Vec::new()],
            }),
            13 => {
                if let Some(BlockStackEntry::Alt {
                    section_directives,
                    sections,
                }) = stack.last_mut()
                {
                    section_directives.push((msg.id.clone(), raw_label.to_string()));
                    sections.push(Vec::new());
                }
            }
            14 => {
                let mut end_step = block_end_step;
                if let Some(BlockStackEntry::Alt {
                    section_directives,
                    sections,
                }) = stack.pop()
                {
                    end_step = finish_sectioned_block(
                        &mut directive_steps,
                        section_directives,
                        sections,
                        frame_ctx,
                        step_ctx,
                    );
                }
                directive_steps.insert(msg.id.clone(), end_step);
            }
            // par start/and/end
            19 | 32 => stack.push(BlockStackEntry::Par {
                section_directives: vec![(msg.id.clone(), raw_label.to_string())],
                sections: vec![Vec::new()],
            }),
            20 => {
                if let Some(BlockStackEntry::Par {
                    section_directives,
                    sections,
                }) = stack.last_mut()
                {
                    section_directives.push((msg.id.clone(), raw_label.to_string()));
                    sections.push(Vec::new());
                }
            }
            21 => {
                let mut end_step = block_end_step;
                if let Some(BlockStackEntry::Par {
                    section_directives,
                    sections,
                }) = stack.pop()
                {
                    end_step = finish_sectioned_block(
                        &mut directive_steps,
                        section_directives,
                        sections,
                        frame_ctx,
                        step_ctx,
                    );
                }
                directive_steps.insert(msg.id.clone(), end_step);
            }
            // critical start/option/end
            27 => stack.push(BlockStackEntry::Critical {
                section_directives: vec![(msg.id.clone(), raw_label.to_string())],
                sections: vec![Vec::new()],
            }),
            28 => {
                if let Some(BlockStackEntry::Critical {
                    section_directives,
                    sections,
                }) = stack.last_mut()
                {
                    section_directives.push((msg.id.clone(), raw_label.to_string()));
                    sections.push(Vec::new());
                }
            }
            29 => {
                let mut end_step = block_end_step;
                if let Some(BlockStackEntry::Critical {
                    section_directives,
                    sections,
                }) = stack.pop()
                {
                    end_step = finish_sectioned_block(
                        &mut directive_steps,
                        section_directives,
                        sections,
                        frame_ctx,
                        step_ctx,
                    );
                }
                directive_steps.insert(msg.id.clone(), end_step);
            }
            _ => {
                // If this is a "real" message edge, attach it to all active block scopes so block
                // width computations can account for overflowing message labels.
                if msg.from.is_some() && msg.to.is_some() {
                    for entry in stack.iter_mut() {
                        push_message_to_active_block(entry, msg.id.clone());
                    }
                }
            }
        }
    }

    directive_steps
}

fn bracketize(s: &str) -> String {
    let t = s.trim();
    if t.is_empty() {
        return "\u{200B}".to_string();
    }
    if t.starts_with('[') && t.ends_with(']') {
        return t.to_string();
    }
    format!("[{t}]")
}

fn block_label_text(raw_label: &str) -> String {
    bracketize(raw_label)
}

fn finish_single_block(
    directive_steps: &mut HashMap<String, f64>,
    start_id: String,
    raw_label: String,
    messages: Vec<String>,
    frame_ctx: BlockFrameWidthContext<'_>,
    step_ctx: BlockStepContext,
) -> f64 {
    let has_self = messages
        .iter()
        .any(|msg_id| is_self_message_id(msg_id.as_str(), frame_ctx.msg_by_id));
    let start_step = block_start_step(&raw_label, &messages, frame_ctx, step_ctx);
    directive_steps.insert(start_id, start_step);
    if has_self {
        40.0
    } else {
        step_ctx.block_end_step
    }
}

fn finish_sectioned_block(
    directive_steps: &mut HashMap<String, f64>,
    section_directives: Vec<(String, String)>,
    sections: Vec<Vec<String>>,
    frame_ctx: BlockFrameWidthContext<'_>,
    step_ctx: BlockStepContext,
) -> f64 {
    let has_self = sections
        .iter()
        .flatten()
        .any(|msg_id| is_self_message_id(msg_id.as_str(), frame_ctx.msg_by_id));
    let mut message_ids: Vec<String> = Vec::new();
    for sec in &sections {
        message_ids.extend(sec.iter().cloned());
    }

    let frame_width = block_frame_width(&message_ids, frame_ctx);
    for (id, raw_label) in section_directives {
        let step = block_label_step(&raw_label, frame_width, frame_ctx, step_ctx);
        directive_steps.insert(id, step);
    }

    if has_self {
        40.0
    } else {
        step_ctx.block_end_step
    }
}

fn block_start_step(
    raw_label: &str,
    message_ids: &[String],
    frame_ctx: BlockFrameWidthContext<'_>,
    step_ctx: BlockStepContext,
) -> f64 {
    block_label_step(
        raw_label,
        block_frame_width(message_ids, frame_ctx),
        frame_ctx,
        step_ctx,
    )
}

fn block_label_step(
    raw_label: &str,
    frame_width: Option<f64>,
    frame_ctx: BlockFrameWidthContext<'_>,
    step_ctx: BlockStepContext,
) -> f64 {
    if raw_label.trim().is_empty() {
        return step_ctx.block_base_step_empty;
    }

    let Some(width) = frame_width else {
        return step_ctx.block_base_step;
    };

    let label = block_label_text(raw_label);
    let metrics = frame_ctx.measurer.measure_wrapped(
        &label,
        frame_ctx.msg_text_style,
        Some(width),
        WrapMode::SvgLikeSingleRun,
    );
    let extra = (metrics.line_count.saturating_sub(1) as f64) * step_ctx.block_extra_per_line;
    step_ctx.block_base_step + extra
}

fn is_self_message_id(msg_id: &str, msg_by_id: &HashMap<&str, &SequenceMessage>) -> bool {
    let Some(msg) = msg_by_id.get(msg_id).copied() else {
        return false;
    };
    // Notes can use `from==to` for `rightOf`/`leftOf`; do not treat them as self-messages.
    if msg.message_type == 2 {
        return false;
    }
    msg.from
        .as_deref()
        .is_some_and(|from| Some(from) == msg.to.as_deref())
}

#[derive(Clone, Copy)]
struct BlockFrameWidthContext<'a> {
    msg_by_id: &'a HashMap<&'a str, &'a SequenceMessage>,
    actor_index: &'a HashMap<&'a str, usize>,
    actor_centers_x: &'a [f64],
    actor_widths: &'a [f64],
    message_margin: f64,
    box_text_margin: f64,
    bottom_margin_adj: f64,
    measurer: &'a dyn TextMeasurer,
    msg_text_style: &'a TextStyle,
    math_config: &'a MermaidConfig,
    math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
    message_width_scale: f64,
}

#[derive(Clone, Copy)]
struct BlockStepContext {
    block_base_step: f64,
    block_base_step_empty: f64,
    block_extra_per_line: f64,
    block_end_step: f64,
}

fn message_span_x(msg: &SequenceMessage, ctx: BlockFrameWidthContext<'_>) -> Option<(f64, f64)> {
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
    let sign = if to_x >= from_x { 1.0 } else { -1.0 };
    let x1 = from_x + sign * 1.0;
    let x2 = if from == to { x1 } else { to_x - sign * 4.0 };
    let cx = (x1 + x2) / 2.0;

    let text = msg.message_text();
    let w = if text.is_empty() {
        1.0
    } else {
        let (w, _h) = measure_sequence_label_for_layout(
            ctx.measurer,
            text,
            ctx.msg_text_style,
            ctx.math_config,
            ctx.math_renderer,
            SequenceMathHeightMode::Bound,
        );
        (w * ctx.message_width_scale).max(1.0)
    };
    Some((cx - w / 2.0, cx + w / 2.0))
}

fn block_frame_width(message_ids: &[String], ctx: BlockFrameWidthContext<'_>) -> Option<f64> {
    let mut actor_idxs: Vec<usize> = Vec::new();
    for msg_id in message_ids {
        let Some(msg) = ctx.msg_by_id.get(msg_id.as_str()).copied() else {
            continue;
        };
        let (Some(from), Some(to)) = (msg.from.as_deref(), msg.to.as_deref()) else {
            continue;
        };
        if let Some(i) = ctx.actor_index.get(from).copied() {
            actor_idxs.push(i);
        }
        if let Some(i) = ctx.actor_index.get(to).copied() {
            actor_idxs.push(i);
        }
    }
    actor_idxs.sort();
    actor_idxs.dedup();
    if actor_idxs.is_empty() {
        return None;
    }

    if actor_idxs.len() == 1 {
        let i = actor_idxs[0];
        let actor_w = ctx.actor_widths.get(i).copied().unwrap_or(150.0);
        let half_width = actor_w / 2.0
            + (ctx.message_margin / 2.0)
            + ctx.box_text_margin
            + ctx.bottom_margin_adj;
        let w = (2.0 * half_width).max(1.0);
        return Some(w);
    }

    let min_i = actor_idxs.first().copied()?;
    let max_i = actor_idxs.last().copied()?;
    let mut x1 = ctx.actor_centers_x[min_i] - SEQUENCE_FRAME_SIDE_PAD_PX;
    let mut x2 = ctx.actor_centers_x[max_i] + SEQUENCE_FRAME_SIDE_PAD_PX;

    // Expand multi-actor blocks to include overflowing message labels (e.g. long self messages).
    for msg_id in message_ids {
        let Some(msg) = ctx.msg_by_id.get(msg_id.as_str()).copied() else {
            continue;
        };
        let Some((l, r)) = message_span_x(msg, ctx) else {
            continue;
        };
        if l < x1 {
            x1 = l.floor();
        }
        if r > x2 {
            x2 = r.ceil();
        }
    }

    Some((x2 - x1).max(1.0))
}

#[derive(Debug, Clone)]
enum BlockStackEntry {
    Loop {
        start_id: String,
        raw_label: String,
        messages: Vec<String>,
    },
    Opt {
        start_id: String,
        raw_label: String,
        messages: Vec<String>,
    },
    Break {
        start_id: String,
        raw_label: String,
        messages: Vec<String>,
    },
    Alt {
        section_directives: Vec<(String, String)>,
        sections: Vec<Vec<String>>,
    },
    Par {
        section_directives: Vec<(String, String)>,
        sections: Vec<Vec<String>>,
    },
    Critical {
        section_directives: Vec<(String, String)>,
        sections: Vec<Vec<String>>,
    },
}

fn push_message_to_active_block(entry: &mut BlockStackEntry, message_id: String) {
    match entry {
        BlockStackEntry::Alt { sections, .. }
        | BlockStackEntry::Par { sections, .. }
        | BlockStackEntry::Critical { sections, .. } => {
            if let Some(cur) = sections.last_mut() {
                cur.push(message_id);
            }
        }
        BlockStackEntry::Loop { messages, .. }
        | BlockStackEntry::Opt { messages, .. }
        | BlockStackEntry::Break { messages, .. } => {
            messages.push(message_id);
        }
    }
}

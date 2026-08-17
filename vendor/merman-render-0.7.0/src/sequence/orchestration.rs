use super::activation::SequenceActivationState;
use super::actors::{
    SequenceActorLifecycle, SequenceActorLifecycleContext, SequenceFooterActorContext,
    SequenceTopActorContext, append_sequence_footer_actors, append_sequence_top_actors,
    sequence_actor_is_type_width_limited,
};
use super::block_steps::{BlockStepPlanContext, plan_sequence_directive_steps};
use super::messages::{SequenceMessageLayoutContext, layout_sequence_message};
use super::notes::{SequenceNoteLayoutContext, layout_sequence_note};
use super::rect::SequenceRectOpen;
use super::{
    SEQUENCE_FRAME_GEOM_PAD_PX, SEQUENCE_FRAME_SIDE_PAD_PX, SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX,
};
use crate::math::MathRenderer;
use crate::model::{LayoutEdge, LayoutNode};
use crate::text::{TextMeasurer, TextStyle};
use merman_core::MermaidConfig;
use merman_core::diagrams::sequence::{SequenceDiagramRenderModel, SequenceMessage};
use std::collections::HashMap;

pub(super) struct SequenceLayoutGraphContext<'a> {
    pub(super) model: &'a SequenceDiagramRenderModel,
    pub(super) actor_index: &'a HashMap<&'a str, usize>,
    pub(super) actor_centers_x: &'a [f64],
    pub(super) actor_widths: &'a [f64],
    pub(super) actor_base_heights: &'a [f64],
    pub(super) actor_top_offset_y: f64,
    pub(super) max_actor_layout_height: f64,
    pub(super) actor_width_min: f64,
    pub(super) sequence_default_width: f64,
    pub(super) actor_height: f64,
    pub(super) message_margin: f64,
    pub(super) box_margin: f64,
    pub(super) box_text_margin: f64,
    pub(super) bottom_margin_adj: f64,
    pub(super) label_box_height: f64,
    pub(super) message_step: f64,
    pub(super) message_text_line_height: f64,
    pub(super) msg_label_offset: f64,
    pub(super) message_font_size: f64,
    pub(super) message_width_scale: f64,
    pub(super) wrap_padding: f64,
    pub(super) mirror_actors: bool,
    pub(super) activation_width: f64,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) msg_text_style: &'a TextStyle,
    pub(super) note_text_style: &'a TextStyle,
    pub(super) math_config: &'a MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
}

pub(super) struct SequenceLayoutGraph {
    pub(super) nodes: Vec<LayoutNode>,
    pub(super) edges: Vec<LayoutEdge>,
    pub(super) bottom_box_top_y: f64,
}

struct SequenceLayoutLoopState<'a> {
    cursor_y: f64,
    rect_stack: Vec<SequenceRectOpen>,
    activation_state: SequenceActivationState<'a>,
    actor_lifecycle: SequenceActorLifecycle<'a>,
}

impl<'a> SequenceLayoutLoopState<'a> {
    fn new(ctx: &SequenceLayoutGraphContext<'a>) -> Self {
        let activation_state = SequenceActivationState::new(ctx.activation_width);
        let actor_lifecycle = SequenceActorLifecycle::new(SequenceActorLifecycleContext {
            actor_index: ctx.actor_index,
            actor_base_heights: ctx.actor_base_heights,
            created_actors: &ctx.model.created_actors,
            destroyed_actors: &ctx.model.destroyed_actors,
            actor_height: ctx.actor_height,
        });

        Self {
            cursor_y: ctx.actor_top_offset_y + ctx.max_actor_layout_height + ctx.message_step,
            rect_stack: Vec::new(),
            activation_state,
            actor_lifecycle,
        }
    }
}

fn handle_sequence_directive<'a>(
    msg: &'a SequenceMessage,
    directive_steps: &HashMap<String, f64>,
    state: &mut SequenceLayoutLoopState<'a>,
    ctx: &SequenceLayoutGraphContext<'a>,
) -> bool {
    if state
        .activation_state
        .handle_directive(msg, ctx.actor_index, ctx.actor_centers_x)
    {
        return true;
    }

    if let Some(step) = directive_steps.get(msg.id.as_str()).copied() {
        state.cursor_y += step;
        return true;
    }

    false
}

fn handle_sequence_rect(
    msg: &SequenceMessage,
    state: &mut SequenceLayoutLoopState<'_>,
    note_top_offset: f64,
    rect_step_start: f64,
    rect_step_end: f64,
    actor_centers_x: &[f64],
    nodes: &mut Vec<LayoutNode>,
) -> bool {
    match msg.message_type {
        22 => {
            state.rect_stack.push(SequenceRectOpen::new(
                msg.id.clone(),
                state.cursor_y - note_top_offset,
            ));
            state.cursor_y += rect_step_start;
            true
        }
        23 => {
            if let Some(open) = state.rect_stack.pop() {
                let closed = open.close(actor_centers_x);
                nodes.push(closed.node);

                if let Some(parent) = state.rect_stack.last_mut() {
                    parent.include_min_max(
                        closed.left - SEQUENCE_FRAME_SIDE_PAD_PX,
                        closed.right + SEQUENCE_FRAME_SIDE_PAD_PX,
                        closed.bottom,
                    );
                }
            }
            state.cursor_y += rect_step_end;
            true
        }
        _ => false,
    }
}

fn handle_sequence_note(
    msg: &SequenceMessage,
    state: &mut SequenceLayoutLoopState<'_>,
    ctx: &SequenceLayoutGraphContext<'_>,
    nodes: &mut Vec<LayoutNode>,
    note_default_width: f64,
    note_top_offset: f64,
) -> bool {
    if msg.message_type != 2 {
        return false;
    }

    let note_gap = SEQUENCE_FRAME_GEOM_PAD_PX;
    let note_text_pad_total = 2.0 * note_gap;

    let Some(note) = layout_sequence_note(
        msg,
        SequenceNoteLayoutContext {
            actor_index: ctx.actor_index,
            actor_centers_x: ctx.actor_centers_x,
            actor_widths: ctx.actor_widths,
            note_default_width,
            note_text_pad_total,
            note_top_offset,
            note_gap,
            cursor_y: state.cursor_y,
            measurer: ctx.measurer,
            note_text_style: ctx.note_text_style,
            math_config: ctx.math_config,
            math_renderer: ctx.math_renderer,
        },
    ) else {
        return true;
    };

    include_rect_stack_bounds(
        &mut state.rect_stack,
        note.rect_min_x,
        note.rect_max_x,
        note.rect_max_y,
    );

    nodes.push(note.node);
    state.cursor_y += note.cursor_step;
    true
}

fn handle_sequence_message(
    msg: &SequenceMessage,
    msg_idx: usize,
    state: &mut SequenceLayoutLoopState<'_>,
    ctx: &SequenceLayoutGraphContext<'_>,
    edges: &mut Vec<LayoutEdge>,
    actor_is_type_width_limited: &dyn Fn(&str) -> bool,
) -> bool {
    let Some(message) = layout_sequence_message(
        msg,
        SequenceMessageLayoutContext {
            actor_index: ctx.actor_index,
            actor_centers_x: ctx.actor_centers_x,
            actor_widths: ctx.actor_widths,
            activation_state: &state.activation_state,
            msg_idx,
            actor_width_min: ctx.actor_width_min,
            box_margin: ctx.box_margin,
            wrap_padding: ctx.wrap_padding,
            message_text_line_height: ctx.message_text_line_height,
            message_step: ctx.message_step,
            msg_label_offset: ctx.msg_label_offset,
            message_font_size: ctx.message_font_size,
            message_width_scale: ctx.message_width_scale,
            cursor_y: state.cursor_y,
            measurer: ctx.measurer,
            msg_text_style: ctx.msg_text_style,
            math_config: ctx.math_config,
            math_renderer: ctx.math_renderer,
            created_actor_index: msg
                .to
                .as_deref()
                .and_then(|to| state.actor_lifecycle.created_actor_index(to)),
            destroyed_from_index: msg
                .from
                .as_deref()
                .and_then(|from| state.actor_lifecycle.destroyed_actor_index(from)),
            destroyed_to_index: msg
                .to
                .as_deref()
                .and_then(|to| state.actor_lifecycle.destroyed_actor_index(to)),
            actor_is_type_width_limited,
        },
    ) else {
        return false;
    };

    include_rect_stack_bounds(
        &mut state.rect_stack,
        message.from_x.min(message.to_x) - SEQUENCE_FRAME_SIDE_PAD_PX,
        message.from_x.max(message.to_x) + SEQUENCE_FRAME_SIDE_PAD_PX,
        message.line_y,
    );

    let from = message.from;
    let to = message.to;
    let line_y = message.line_y;
    state.cursor_y += message.cursor_step;
    if message.is_self {
        // Mermaid adds extra space for self-messages so the loop curve can fit cleanly.
        state.cursor_y += SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX / 2.0;
    }
    state.cursor_y += state
        .actor_lifecycle
        .apply_message_y_adjustment(msg_idx, from, to, line_y);
    edges.push(message.edge);
    true
}

pub(super) fn build_sequence_layout_graph(
    ctx: SequenceLayoutGraphContext<'_>,
) -> SequenceLayoutGraph {
    let mut nodes: Vec<LayoutNode> = Vec::new();
    let mut edges: Vec<LayoutEdge> = Vec::new();

    append_sequence_top_actors(
        &mut nodes,
        SequenceTopActorContext {
            actor_order: &ctx.model.actor_order,
            actors: &ctx.model.actors,
            actor_widths: ctx.actor_widths,
            actor_centers_x: ctx.actor_centers_x,
            actor_base_heights: ctx.actor_base_heights,
            actor_top_offset_y: ctx.actor_top_offset_y,
            label_box_height: ctx.label_box_height,
        },
    );

    let directive_steps = plan_sequence_directive_steps(BlockStepPlanContext {
        model: ctx.model,
        actor_index: ctx.actor_index,
        actor_centers_x: ctx.actor_centers_x,
        actor_widths: ctx.actor_widths,
        message_margin: ctx.message_margin,
        box_margin: ctx.box_margin,
        box_text_margin: ctx.box_text_margin,
        bottom_margin_adj: ctx.bottom_margin_adj,
        label_box_height: ctx.label_box_height,
        message_font_size: ctx.message_font_size,
        measurer: ctx.measurer,
        msg_text_style: ctx.msg_text_style,
        math_config: ctx.math_config,
        math_renderer: ctx.math_renderer,
        message_width_scale: ctx.message_width_scale,
    });

    let note_default_width = ctx.sequence_default_width;
    let rect_step_start = 2.0 * SEQUENCE_FRAME_GEOM_PAD_PX;
    let rect_step_end = SEQUENCE_FRAME_GEOM_PAD_PX;
    let note_gap = SEQUENCE_FRAME_GEOM_PAD_PX;
    let note_top_offset = ctx.message_step - note_gap;
    let mut state = SequenceLayoutLoopState::new(&ctx);
    let actor_is_type_width_limited = |actor_id: &str| -> bool {
        sequence_actor_is_type_width_limited(&ctx.model.actors, actor_id)
    };

    for (msg_idx, msg) in ctx.model.messages.iter().enumerate() {
        if handle_sequence_directive(msg, &directive_steps, &mut state, &ctx) {
            continue;
        }

        if handle_sequence_rect(
            msg,
            &mut state,
            note_top_offset,
            rect_step_start,
            rect_step_end,
            ctx.actor_centers_x,
            &mut nodes,
        ) {
            continue;
        }

        if handle_sequence_note(
            msg,
            &mut state,
            &ctx,
            &mut nodes,
            note_default_width,
            note_top_offset,
        ) {
            continue;
        }

        if handle_sequence_message(
            msg,
            msg_idx,
            &mut state,
            &ctx,
            &mut edges,
            &actor_is_type_width_limited,
        ) {
            continue;
        }
    }

    let bottom_margin = 2.0 * ctx.box_margin;
    let bottom_box_top_y = (state.cursor_y - ctx.message_step) + bottom_margin;

    state
        .actor_lifecycle
        .apply_created_top_actor_positions(&mut nodes);

    append_sequence_footer_actors(
        &mut nodes,
        &mut edges,
        SequenceFooterActorContext {
            actor_order: &ctx.model.actor_order,
            actors: &ctx.model.actors,
            actor_widths: ctx.actor_widths,
            actor_centers_x: ctx.actor_centers_x,
            actor_base_heights: ctx.actor_base_heights,
            actor_lifecycle: &state.actor_lifecycle,
            actor_top_offset_y: ctx.actor_top_offset_y,
            bottom_box_top_y,
            mirror_actors: ctx.mirror_actors,
            label_box_height: ctx.label_box_height,
            box_text_margin: ctx.box_text_margin,
        },
    );

    SequenceLayoutGraph {
        nodes,
        edges,
        bottom_box_top_y,
    }
}

fn include_rect_stack_bounds(
    rect_stack: &mut [SequenceRectOpen],
    min_x: f64,
    max_x: f64,
    max_y: f64,
) {
    for open in rect_stack.iter_mut() {
        open.include_min_max(min_x, max_x, max_y);
    }
}

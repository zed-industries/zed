use super::super::*;
use super::activation::{build_sequence_activation_plan, render_sequence_activation_group};
use super::block_collection::{SequenceBlock, collect_sequence_blocks};
use super::block_geometry::frame_x_from_actors;
use super::blocks::{
    SequenceBlockRenderContext, render_critical_sequence_block, render_sectioned_sequence_block,
    render_simple_sequence_block,
};
use super::model::*;
use super::notes::{SequenceNoteRenderContext, render_sequence_note};
use super::settings::SequenceRenderSettings;
use rustc_hash::FxHashMap;

pub(super) struct SequenceInteractionRenderContext<'a> {
    pub(super) model: &'a SequenceSvgModel,
    pub(super) nodes_by_id: &'a FxHashMap<&'a str, &'a LayoutNode>,
    pub(super) edges_by_id: &'a FxHashMap<&'a str, &'a crate::model::LayoutEdge>,
    pub(super) sanitize_config: &'a merman_core::MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn crate::math::MathRenderer + Send + Sync)>,
    pub(super) settings: &'a SequenceRenderSettings,
    pub(super) measurer: &'a dyn TextMeasurer,
}

pub(super) fn render_sequence_interaction_overlays(
    out: &mut String,
    ctx: &SequenceInteractionRenderContext<'_>,
) {
    // Mermaid creates activation placeholders at ACTIVE_START and inserts the `<rect>` once the
    // corresponding ACTIVE_END is encountered. We store the final rect geometry during this
    // first pass and remember which message id should emit which activation group.
    let activation_plan = build_sequence_activation_plan(
        ctx.model,
        ctx.nodes_by_id,
        ctx.edges_by_id,
        ctx.settings.activation_width,
    );

    let (blocks_by_end_id, blocks) = collect_sequence_blocks(ctx.model);

    let Some((frame_x1, frame_x2)) = frame_x_from_actors(ctx.model, ctx.nodes_by_id) else {
        return;
    };

    let mut actor_nodes_by_id: FxHashMap<&str, &LayoutNode> =
        FxHashMap::with_capacity_and_hasher(ctx.model.actors.len(), Default::default());
    for actor_id in &ctx.model.actor_order {
        let node_id = format!("actor-top-{actor_id}");
        let Some(n) = ctx.nodes_by_id.get(node_id.as_str()).copied() else {
            continue;
        };
        actor_nodes_by_id.insert(actor_id.as_str(), n);
    }

    let mut msg_endpoints: FxHashMap<&str, (&str, &str)> =
        FxHashMap::with_capacity_and_hasher(ctx.model.messages.len(), Default::default());
    for msg in &ctx.model.messages {
        let (Some(from), Some(to)) = (msg.from.as_deref(), msg.to.as_deref()) else {
            continue;
        };
        msg_endpoints.insert(msg.id.as_str(), (from, to));
    }

    let block_ctx = SequenceBlockRenderContext {
        default_frame_x1: frame_x1,
        default_frame_x2: frame_x2,
        msg_endpoints: &msg_endpoints,
        actor_nodes_by_id: &actor_nodes_by_id,
        edges_by_id: ctx.edges_by_id,
        nodes_by_id: ctx.nodes_by_id,
        label_box_height: ctx.settings.label_box_height,
        box_text_margin: ctx.settings.box_text_margin,
        measurer: ctx.measurer,
        loop_text_style: &ctx.settings.loop_text_style,
    };
    let note_ctx = SequenceNoteRenderContext {
        nodes_by_id: ctx.nodes_by_id,
        measurer: ctx.measurer,
        actor_label_font_size: ctx.settings.actor_label_font_size,
        wrap_padding: ctx.settings.wrap_padding,
        note_text_style: &ctx.settings.note_text_style,
        sanitize_config: ctx.sanitize_config,
        math_renderer: ctx.math_renderer,
    };

    for msg in &ctx.model.messages {
        render_sequence_activation_group(out, &activation_plan, &msg.id);
        render_sequence_note(out, msg, &note_ctx);

        let Some(idxs) = blocks_by_end_id.get(msg.id.as_str()) else {
            continue;
        };
        for idx in idxs {
            let Some(block) = blocks.get(*idx) else {
                continue;
            };
            match block {
                SequenceBlock::Alt {
                    control_id,
                    sections,
                } => {
                    render_sectioned_sequence_block(
                        out, control_id, "alt", sections, true, &block_ctx,
                    );
                }
                SequenceBlock::Par {
                    control_id,
                    sections,
                } => {
                    render_sectioned_sequence_block(
                        out, control_id, "par", sections, false, &block_ctx,
                    );
                }
                SequenceBlock::Loop {
                    control_id,
                    raw_label,
                    message_ids,
                } => {
                    render_simple_sequence_block(
                        out,
                        control_id,
                        "loop",
                        raw_label,
                        message_ids,
                        &block_ctx,
                    );
                }
                SequenceBlock::Opt {
                    control_id,
                    raw_label,
                    message_ids,
                } => {
                    render_simple_sequence_block(
                        out,
                        control_id,
                        "opt",
                        raw_label,
                        message_ids,
                        &block_ctx,
                    );
                }
                SequenceBlock::Break {
                    control_id,
                    raw_label,
                    message_ids,
                } => {
                    render_simple_sequence_block(
                        out,
                        control_id,
                        "break",
                        raw_label,
                        message_ids,
                        &block_ctx,
                    );
                }
                SequenceBlock::Critical {
                    control_id,
                    sections,
                } => {
                    render_critical_sequence_block(out, control_id, sections, &block_ctx);
                }
            }
        }
    }
}

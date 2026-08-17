use super::super::*;
use super::actor_shapes::{
    ActorLabelContext, is_actor_man_variant, write_actor_man_lifeline,
    write_collection_actor_shape, write_database_bottom_actor_shape,
    write_database_top_actor_shape, write_lifeline_root_open, write_queue_actor_shape,
    write_rect_actor_shape,
};
use super::geometry::node_left_top;
use super::model::SequenceSvgModel;
use rustc_hash::FxHashMap;

pub(super) struct SequenceActorRenderContext<'a> {
    pub(super) model: &'a SequenceSvgModel,
    pub(super) nodes_by_id: &'a FxHashMap<&'a str, &'a LayoutNode>,
    pub(super) edges_by_id: &'a FxHashMap<&'a str, &'a crate::model::LayoutEdge>,
    pub(super) sanitize_config: &'a merman_core::MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn crate::math::MathRenderer + Send + Sync)>,
    pub(super) actor_wrap_width: f64,
    pub(super) actor_height: f64,
    pub(super) label_box_height: f64,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) loop_text_style: &'a TextStyle,
}

pub(super) fn render_sequence_bottom_actors(
    out: &mut String,
    ctx: &SequenceActorRenderContext<'_>,
) {
    let label_ctx = ActorLabelContext::new(
        ctx.actor_wrap_width,
        ctx.measurer,
        ctx.loop_text_style,
        ctx.sanitize_config,
        ctx.math_renderer,
    );

    // Mermaid draws bottom actors first (reverse DOM order).
    for actor_id in ctx.model.actor_order.iter().rev() {
        let Some(actor) = ctx.model.actors.get(actor_id) else {
            continue;
        };
        let actor_type = actor.actor_type.as_str();
        let node_id = format!("actor-bottom-{actor_id}");
        let Some(n) = ctx.nodes_by_id.get(node_id.as_str()).copied() else {
            continue;
        };
        match actor_type {
            // Actor-man variants are drawn later (after `<defs>`), but Mermaid keeps stable
            // indices by emitting empty `<g/>` placeholders here.
            actor_type if is_actor_man_variant(actor_type) => {
                out.push_str("<g/>");
            }
            "collections" => {
                out.push_str("<g>");
                write_collection_actor_shape(out, n, actor_id, actor, "actor-bottom", &label_ctx);
                out.push_str("</g>");
            }
            "queue" => {
                out.push_str(r#"<g class="actor actor-bottom">"#);
                write_queue_actor_shape(out, n, actor, "actor-bottom", &label_ctx);
                out.push_str("</g>");
            }
            "database" => {
                out.push_str("<g>");
                write_database_bottom_actor_shape(out, n, actor, ctx.label_box_height, &label_ctx);
                out.push_str("</g>");
            }
            _ => {
                out.push_str("<g>");
                write_rect_actor_shape(out, n, actor_id, actor, "actor-bottom", &label_ctx);
                out.push_str("</g>");
            }
        }
    }
}

pub(super) fn render_sequence_top_actors_and_lifelines(
    out: &mut String,
    ctx: &SequenceActorRenderContext<'_>,
) {
    let label_ctx = ActorLabelContext::new(
        ctx.actor_wrap_width,
        ctx.measurer,
        ctx.loop_text_style,
        ctx.sanitize_config,
        ctx.math_renderer,
    );

    for (idx, actor_id) in ctx.model.actor_order.iter().enumerate().rev() {
        let Some(actor) = ctx.model.actors.get(actor_id) else {
            continue;
        };
        let actor_type = actor.actor_type.as_str();
        let node_top_id = format!("actor-top-{actor_id}");
        let node_bottom_id = format!("actor-bottom-{actor_id}");
        let Some(top) = ctx.nodes_by_id.get(node_top_id.as_str()).copied() else {
            continue;
        };
        let Some(bottom) = ctx.nodes_by_id.get(node_bottom_id.as_str()).copied() else {
            continue;
        };
        let (_, top_y) = node_left_top(top);
        let (_, bottom_y) = node_left_top(bottom);

        let (y1, y2) = ctx
            .edges_by_id
            .get(format!("lifeline-{actor_id}").as_str())
            .and_then(|e| Some((e.points.first()?.y, e.points.get(1)?.y)))
            .unwrap_or((top_y + top.height, bottom_y));

        match actor_type {
            actor_type if is_actor_man_variant(actor_type) => {
                write_actor_man_lifeline(out, idx, top.x, y1, y2, actor_id);
            }
            "collections" => {
                write_lifeline_root_open(out, idx, top.x, y1, y2, actor_id, actor_type);
                write_collection_actor_shape(out, top, actor_id, actor, "actor-top", &label_ctx);
                out.push_str("</g></g>");
            }
            "queue" => {
                write_lifeline_root_open(out, idx, top.x, y1, y2, actor_id, actor_type);
                write_queue_actor_shape(out, top, actor, "actor-top", &label_ctx);
                out.push_str("</g></g>");
            }
            "database" => {
                write_lifeline_root_open(out, idx, top.x, y1, y2, actor_id, actor_type);
                write_database_top_actor_shape(out, top, actor, ctx.actor_height, &label_ctx);
                out.push_str("</g></g>");
            }
            _ => {
                write_lifeline_root_open(out, idx, top.x, y1, y2, actor_id, actor_type);
                write_rect_actor_shape(out, top, actor_id, actor, "actor-top", &label_ctx);
                out.push_str("</g></g>");
            }
        }
    }
}

use super::super::*;
use super::actor_man::{render_sequence_actor_man_bottoms, render_sequence_actor_man_tops};
use super::actor_popup::render_sequence_actor_popup_menus;
use super::actors::{
    SequenceActorRenderContext, render_sequence_bottom_actors,
    render_sequence_top_actors_and_lifelines,
};
use super::frames::render_sequence_box_frames_and_rect_blocks;
use super::interactions::{SequenceInteractionRenderContext, render_sequence_interaction_overlays};
use super::messages::{SequenceMessageRenderContext, render_sequence_messages};
use super::root::write_sequence_svg_root_open;
use super::settings::SequenceRenderSettings;
use rustc_hash::FxHashMap;

use super::css::sequence_css;
use super::model::*;

const MERMAID_SEQUENCE_EXTRA_MARKER_DEFS_11_15_0: &str = r#"<defs><marker id="solidTopArrowHead" refX="7.9" refY="7.25" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto-start-reverse"><path d="M 0 0 L 10 8 L 0 8 z"/></marker></defs><defs><marker id="solidBottomArrowHead" refX="7.9" refY="0.75" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto-start-reverse"><path d="M 0 0 L 10 0 L 0 8 z"/></marker></defs><defs><marker id="stickTopArrowHead" refX="7.5" refY="7" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto-start-reverse"><path d="M 0 0 L 7 7" stroke="black" stroke-width="1.5" fill="none"/></marker></defs><defs><marker id="stickBottomArrowHead" refX="7.5" refY="0" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto-start-reverse"><path d="M 0 7 L 7 0" stroke="black" stroke-width="1.5" fill="none"/></marker></defs>"#;

fn scoped_sequence_base_defs(diagram_id: &str) -> String {
    let mut defs = MERMAID_SEQUENCE_BASE_DEFS_11_12_2.to_string();
    defs.push_str(MERMAID_SEQUENCE_EXTRA_MARKER_DEFS_11_15_0);
    for local_id in [
        "computer",
        "database",
        "clock",
        "arrowhead",
        "crosshead",
        "filled-head",
        "sequencenumber",
        "solidTopArrowHead",
        "solidBottomArrowHead",
        "stickTopArrowHead",
        "stickBottomArrowHead",
    ] {
        let bare = format!(r#"id="{local_id}""#);
        let scoped = format!(
            r#"id="{}""#,
            escape_attr(&scoped_svg_id(diagram_id, local_id))
        );
        defs = defs.replace(&bare, &scoped);
    }
    defs
}

pub(super) fn render_sequence_diagram_svg(
    layout: &SequenceDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let sanitize_config = merman_core::MermaidConfig::from_value(effective_config.clone());
    let model: SequenceSvgModel = crate::json::from_value_ref(semantic)?;
    render_sequence_diagram_svg_inner(
        layout,
        &model,
        effective_config,
        &sanitize_config,
        diagram_title,
        measurer,
        options,
    )
}

pub(super) fn render_sequence_diagram_svg_with_config(
    layout: &SequenceDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: SequenceSvgModel = crate::json::from_value_ref(semantic)?;
    render_sequence_diagram_svg_model_with_config(
        layout,
        &model,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub(super) fn render_sequence_diagram_svg_model_with_config(
    layout: &SequenceDiagramLayout,
    model: &SequenceSvgModel,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    render_sequence_diagram_svg_inner(
        layout,
        model,
        effective_config.as_value(),
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

fn render_sequence_diagram_svg_inner(
    layout: &SequenceDiagramLayout,
    model: &SequenceSvgModel,
    effective_config: &serde_json::Value,
    sanitize_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let effective_title =
        crate::sequence::sequence_render_title(model.title.as_deref(), diagram_title);

    let seq_cfg = effective_config
        .get("sequence")
        .unwrap_or(&serde_json::Value::Null);
    let settings = SequenceRenderSettings::from_config(effective_config, seq_cfg);

    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");
    let mut out = String::new();
    let root_metrics = write_sequence_svg_root_open(
        &mut out,
        layout,
        model,
        diagram_id,
        options.apply_root_overrides,
    );

    let mut nodes_by_id: FxHashMap<&str, &LayoutNode> =
        FxHashMap::with_capacity_and_hasher(layout.nodes.len(), Default::default());
    for n in &layout.nodes {
        nodes_by_id.insert(n.id.as_str(), n);
    }

    let mut edges_by_id: FxHashMap<&str, &crate::model::LayoutEdge> =
        FxHashMap::with_capacity_and_hasher(layout.edges.len(), Default::default());
    for e in &layout.edges {
        edges_by_id.insert(e.id.as_str(), e);
    }

    render_sequence_box_frames_and_rect_blocks(
        &mut out,
        model,
        &nodes_by_id,
        settings.actor_label_font_size,
        settings.box_margin,
        settings.box_text_margin,
    );

    let actor_ctx = SequenceActorRenderContext {
        model,
        nodes_by_id: &nodes_by_id,
        edges_by_id: &edges_by_id,
        sanitize_config,
        math_renderer: options.math_renderer.as_deref(),
        actor_wrap_width: settings.actor_wrap_width,
        actor_height: settings.actor_height,
        label_box_height: settings.label_box_height,
        measurer,
        loop_text_style: &settings.loop_text_style,
    };

    if settings.mirror_actors {
        render_sequence_bottom_actors(&mut out, &actor_ctx);
    }

    // Top actors + lifelines.
    render_sequence_top_actors_and_lifelines(&mut out, &actor_ctx);

    let _ = write!(
        &mut out,
        r#"<style>{}</style><g/>"#,
        sequence_css(diagram_id, settings.actor_label_font_size, effective_config)
    );

    // Mermaid's sequence output includes a shared set of <defs> for icons/markers.
    out.push_str(&scoped_sequence_base_defs(diagram_id));

    render_sequence_actor_man_tops(
        &mut out,
        model,
        &nodes_by_id,
        settings.actor_height,
        diagram_id,
    );

    let interaction_ctx = SequenceInteractionRenderContext {
        model,
        nodes_by_id: &nodes_by_id,
        edges_by_id: &edges_by_id,
        sanitize_config,
        math_renderer: options.math_renderer.as_deref(),
        settings: &settings,
        measurer,
    };
    render_sequence_interaction_overlays(&mut out, &interaction_ctx);

    let message_ctx = SequenceMessageRenderContext {
        model,
        nodes_by_id: &nodes_by_id,
        edges_by_id: &edges_by_id,
        sanitize_config,
        math_renderer: options.math_renderer.as_deref(),
        measurer,
        message_align: settings.message_align.as_str(),
        diagram_id,
        actor_height: settings.actor_height,
        actor_label_font_size: settings.actor_label_font_size,
        sequence_width: settings.sequence_width,
        activation_width: settings.activation_width,
        wrap_padding: settings.wrap_padding,
        right_angles: settings.right_angles,
        loop_text_style: &settings.loop_text_style,
    };
    render_sequence_messages(&mut out, &message_ctx);

    render_sequence_actor_popup_menus(
        &mut out,
        model,
        &nodes_by_id,
        sanitize_config,
        settings.force_menus,
        settings.mirror_actors,
        settings.actor_height,
    );

    if settings.mirror_actors {
        render_sequence_actor_man_bottoms(
            &mut out,
            model,
            &nodes_by_id,
            settings.actor_height,
            settings.label_box_height,
            diagram_id,
        );
    }

    if let Some(title) = effective_title {
        // Mermaid sequence titles are currently emitted as a plain `<text>` node.
        // Mermaid positions the title using the inner (content) box width:
        // `x = (box.stopx - box.startx) / 2 - 2 * diagramMarginX`.
        let title_x = ((root_metrics.viewbox_width - 2.0 * settings.diagram_margin_x) / 2.0)
            - 2.0 * settings.diagram_margin_x;
        let _ = write!(
            &mut out,
            r#"<text x="{x}" y="-25">{text}</text>"#,
            x = fmt(title_x),
            text = escape_xml(title)
        );
    }

    out.push_str("</svg>\n");
    Ok(out)
}

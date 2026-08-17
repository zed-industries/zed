use super::super::*;
use super::geometry::node_left_top;
use super::model::SequenceSvgModel;
use rustc_hash::FxHashMap;

pub(super) fn render_sequence_actor_popup_menus(
    out: &mut String,
    model: &SequenceSvgModel,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
    sanitize_config: &merman_core::MermaidConfig,
    force_menus: bool,
    mirror_actors: bool,
    actor_height: f64,
) {
    // Mermaid emits actor popup menus (links/link directives) as root-level
    // `<g class="actorPopupMenu">` groups after messages.
    for (actor_cnt, actor_id) in model.actor_order.iter().enumerate() {
        let Some(actor) = model.actors.get(actor_id) else {
            continue;
        };
        if actor.links.is_empty() {
            continue;
        }
        let actor_custom_class = actor
            .properties
            .get("class")
            .and_then(|v| v.as_str())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());
        let popup_display = if force_menus {
            "block !important"
        } else {
            "none"
        };
        let popup_fill = if actor_custom_class.is_some() {
            "#EDF2AE"
        } else {
            "#eaeaea"
        };
        let popup_actor_pos_class = if mirror_actors {
            "actor-bottom"
        } else {
            "actor-top"
        };
        let popup_panel_class = actor_custom_class
            .map(|c| format!("actorPopupMenuPanel {c} {popup_actor_pos_class}"))
            .unwrap_or_else(|| format!("actorPopupMenuPanel actor {popup_actor_pos_class}"));

        let node_id = format!("actor-top-{actor_id}");
        let Some(n) = nodes_by_id.get(node_id.as_str()).copied() else {
            continue;
        };
        let (x, _y) = node_left_top(n);

        let mut link_y: f64 = 20.0;
        let panel_height = crate::sequence::sequence_actor_popup_panel_height(actor.links.len());

        let _ = write!(
            out,
            r##"<g id="actor{idx}_popup" class="actorPopupMenu" display="{display}">"##,
            idx = actor_cnt,
            display = escape_attr(popup_display),
        );
        let _ = write!(
            out,
            r##"<rect class="{class}" x="{x}" y="{y}" fill="{fill}" stroke="#666" width="{w}" height="{h}" rx="3" ry="3"/>"##,
            class = escape_attr(&popup_panel_class),
            x = fmt(x),
            y = fmt(actor_height),
            w = fmt(n.width),
            h = fmt(panel_height),
            fill = escape_xml_display(popup_fill),
        );

        for (label, url) in &actor.links {
            let Some(href) = url.as_str() else {
                continue;
            };
            let href = url::Url::parse(href)
                .map(|u| u.to_string())
                .unwrap_or_else(|_| href.to_string());
            let href = merman_core::utils::format_url(&href, sanitize_config)
                .filter(|u| u.trim() != merman_core::utils::BLANK_URL);
            let text_x = x + 10.0;
            let text_y = actor_height + link_y + 10.0;
            if let Some(href) = href {
                let _ = write!(
                    out,
                    r##"<a xlink:href="{href}"><text x="{x}" y="{y}" dominant-baseline="central" alignment-baseline="central" class="actor" style="text-anchor: start; font-size: 16px; font-weight: 400;"><tspan x="{x}" dy="0">{label}</tspan></text></a>"##,
                    href = escape_xml(&href),
                    x = fmt(text_x),
                    y = fmt(text_y),
                    label = escape_xml(label)
                );
            } else {
                let _ = write!(
                    out,
                    r##"<a><text x="{x}" y="{y}" dominant-baseline="central" alignment-baseline="central" class="actor" style="text-anchor: start; font-size: 16px; font-weight: 400;"><tspan x="{x}" dy="0">{label}</tspan></text></a>"##,
                    x = fmt(text_x),
                    y = fmt(text_y),
                    label = escape_xml(label)
                );
            }
            link_y += 30.0;
        }

        out.push_str("</g>");
    }
}

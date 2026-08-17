use super::super::*;
use super::block_collection::AltSection;
use super::model::SequenceSvgModel;
use crate::sequence::{
    SEQUENCE_FRAME_GEOM_PAD_PX, SEQUENCE_FRAME_SIDE_PAD_PX, SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX,
};
use rustc_hash::FxHashMap;

pub(super) fn frame_x_from_actors(
    model: &SequenceSvgModel,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
) -> Option<(f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    for actor_id in &model.actor_order {
        let node_id = format!("actor-top-{actor_id}");
        let n = nodes_by_id.get(node_id.as_str()).copied()?;
        min_x = min_x.min(n.x);
        max_x = max_x.max(n.x);
    }
    if !min_x.is_finite() || !max_x.is_finite() {
        return None;
    }
    Some((
        min_x - SEQUENCE_FRAME_SIDE_PAD_PX,
        max_x + SEQUENCE_FRAME_SIDE_PAD_PX,
    ))
}

pub(super) fn frame_x_from_message_ids<'a>(
    message_ids: impl IntoIterator<Item = &'a str>,
    msg_endpoints: &FxHashMap<&str, (&str, &str)>,
    actor_nodes_by_id: &FxHashMap<&str, &LayoutNode>,
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
) -> Option<(f64, f64, f64)> {
    // For single-actor frames containing only self-messages, upstream Mermaid expands the
    // frame to cover at least the actor box width (plus a small asymmetric pad that leaves
    // room for the self-arrow loop on the right). Our deterministic layout edge points can
    // be too narrow for short self-message labels, which would over-wrap frame titles.
    let mut min_left = f64::INFINITY;
    let mut geom_min_x = f64::INFINITY;
    let mut geom_max_x = f64::NEG_INFINITY;
    let mut min_cx = f64::INFINITY;
    let mut max_cx = f64::NEG_INFINITY;
    let mut self_only_actor: Option<&str> = None;

    for msg_id in message_ids {
        // Notes are nodes (not edges); include their bounding boxes in frame extents.
        let note_node_id = format!("note-{msg_id}");
        if let Some(n) = nodes_by_id.get(note_node_id.as_str()).copied() {
            geom_min_x = geom_min_x.min(n.x - n.width / 2.0 - SEQUENCE_FRAME_GEOM_PAD_PX);
            geom_max_x = geom_max_x.max(n.x + n.width / 2.0 + SEQUENCE_FRAME_GEOM_PAD_PX);
        }

        let Some((from, to)) = msg_endpoints.get(msg_id).copied() else {
            continue;
        };
        if from == to {
            self_only_actor = match self_only_actor {
                None => Some(from),
                Some(prev) if prev == from => Some(prev),
                _ => Some(""),
            };
        } else {
            self_only_actor = Some("");
        }

        // Expand frames to cover message geometry and label overflow (especially important
        // for single-actor blocks containing long self-message labels).
        let edge_id = format!("msg-{msg_id}");
        if let Some(e) = edges_by_id.get(edge_id.as_str()).copied() {
            for p in &e.points {
                geom_min_x = geom_min_x.min(p.x);
                geom_max_x = geom_max_x.max(p.x);
            }
            if let Some(label) = e.label.as_ref() {
                geom_min_x =
                    geom_min_x.min(label.x - (label.width / 2.0) - SEQUENCE_FRAME_GEOM_PAD_PX);
                geom_max_x =
                    geom_max_x.max(label.x + (label.width / 2.0) + SEQUENCE_FRAME_GEOM_PAD_PX);
            }
        }
        for actor_id in [from, to] {
            let Some(n) = actor_nodes_by_id.get(actor_id).copied() else {
                continue;
            };
            min_cx = min_cx.min(n.x);
            max_cx = max_cx.max(n.x);
            min_left = min_left.min(n.x - n.width / 2.0);
        }
    }

    if !min_cx.is_finite() || !max_cx.is_finite() {
        return None;
    }
    let mut x1 = min_cx - SEQUENCE_FRAME_SIDE_PAD_PX;
    let mut x2 = max_cx + SEQUENCE_FRAME_SIDE_PAD_PX;
    if geom_min_x.is_finite() {
        x1 = x1.min(geom_min_x);
    }
    if geom_max_x.is_finite() {
        x2 = x2.max(geom_max_x);
    }
    if let Some(actor_id) = self_only_actor.filter(|id| !id.is_empty()) {
        if let Some(n) = actor_nodes_by_id.get(actor_id).copied() {
            let left = n.x - n.width / 2.0;
            let right = n.x + n.width / 2.0;
            let min_x1 = left - 5.0;
            let min_x2 = right + 15.0;
            // Only widen when the computed geometry is suspiciously narrow; avoid shifting
            // frames that already match upstream due to message label geometry.
            if (x2 - x1) < (min_x2 - min_x1) - 1.0 {
                x1 = x1.min(min_x1);
                x2 = x2.max(min_x2);
            }
        }
    }
    Some((x1, x2, min_left))
}

pub(super) fn item_y_range(
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
    msg_endpoints: &FxHashMap<&str, (&str, &str)>,
    item_id: &str,
    is_separator: bool,
) -> Option<(f64, f64)> {
    let msg_range = if is_separator {
        msg_y_range_for_separators(edges_by_id, msg_endpoints, item_id)
    } else {
        msg_y_range_for_frame(edges_by_id, msg_endpoints, item_id)
    };
    if let Some((y0, y1)) = msg_range {
        return Some((y0, y1));
    }
    let note_node_id = format!("note-{item_id}");
    let n = nodes_by_id.get(note_node_id.as_str()).copied()?;
    let top = n.y - n.height / 2.0;
    let bottom = n.y + n.height / 2.0;
    Some((top, bottom))
}

pub(super) fn message_ids_y_range<'a>(
    message_ids: impl IntoIterator<Item = &'a str>,
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
    msg_endpoints: &FxHashMap<&str, (&str, &str)>,
    is_separator: bool,
) -> Option<(f64, f64)> {
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for msg_id in message_ids {
        if let Some((y0, y1)) = item_y_range(
            edges_by_id,
            nodes_by_id,
            msg_endpoints,
            msg_id,
            is_separator,
        ) {
            min_y = min_y.min(y0);
            max_y = max_y.max(y1);
        }
    }
    if !min_y.is_finite() || !max_y.is_finite() {
        return None;
    }
    Some((min_y, max_y))
}

pub(super) fn section_message_y_range(
    sections: &[AltSection<'_>],
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
    msg_endpoints: &FxHashMap<&str, (&str, &str)>,
    is_separator: bool,
) -> Option<(f64, f64)> {
    message_ids_y_range(
        sections.iter().flat_map(|s| s.message_ids.iter().copied()),
        edges_by_id,
        nodes_by_id,
        msg_endpoints,
        is_separator,
    )
}

pub(super) fn section_separator_ys(
    sections: &[AltSection<'_>],
    min_y: f64,
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
    msg_endpoints: &FxHashMap<&str, (&str, &str)>,
) -> Vec<f64> {
    let mut section_max_ys: Vec<f64> = Vec::new();
    for sec in sections {
        let sec_max_y = message_ids_y_range(
            sec.message_ids.iter().copied(),
            edges_by_id,
            nodes_by_id,
            msg_endpoints,
            true,
        )
        .map(|(_y0, y1)| y1)
        .unwrap_or(min_y);
        section_max_ys.push(sec_max_y);
    }
    section_max_ys
        .iter()
        .take(section_max_ys.len().saturating_sub(1))
        .map(|sec_max_y| *sec_max_y + 15.0)
        .collect()
}

fn msg_line_y(
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    msg_id: &str,
) -> Option<f64> {
    let edge_id = format!("msg-{msg_id}");
    let e = edges_by_id.get(edge_id.as_str()).copied()?;
    Some(e.points.first()?.y)
}

fn msg_y_range_with_self_extra(
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    msg_endpoints: &FxHashMap<&str, (&str, &str)>,
    msg_id: &str,
    self_extra_y: f64,
) -> Option<(f64, f64)> {
    let y = msg_line_y(edges_by_id, msg_id)?;
    let extra = msg_endpoints
        .get(msg_id)
        .copied()
        .filter(|(from, to)| from == to)
        .map(|_| self_extra_y)
        .unwrap_or(0.0);
    Some((y, y + extra))
}

fn msg_y_range_for_frame(
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    msg_endpoints: &FxHashMap<&str, (&str, &str)>,
    msg_id: &str,
) -> Option<(f64, f64)> {
    // Mermaid's `boundMessage(...)` self-message branch expands the inserted bounds by 60px
    // below `lineStartY` (see the `+ 30 + totalOffset` bottom coordinate, where `totalOffset`
    // already includes a `+30` bump).
    msg_y_range_with_self_extra(
        edges_by_id,
        msg_endpoints,
        msg_id,
        SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX,
    )
}

fn msg_y_range_for_separators(
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    msg_endpoints: &FxHashMap<&str, (&str, &str)>,
    msg_id: &str,
) -> Option<(f64, f64)> {
    // The self-message loop curve itself extends ~30px below the message line.
    // Mermaid's dashed section separators follow the curve geometry, not the full `bounds.insert(...)`
    // envelope used for frame sizing.
    msg_y_range_with_self_extra(
        edges_by_id,
        msg_endpoints,
        msg_id,
        SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX / 2.0,
    )
}

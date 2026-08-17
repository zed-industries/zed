use super::super::*;
use super::model::SequenceSvgModel;
use crate::sequence::sequence_activation_start_x;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
struct SequenceActivationStart {
    startx: f64,
    starty: f64,
    group_index: usize,
}

#[derive(Debug, Clone)]
struct SequenceActivationRect {
    startx: f64,
    starty: f64,
    width: f64,
    height: f64,
    class_idx: usize,
}

#[derive(Debug, Clone)]
pub(super) struct SequenceActivationPlan<'a> {
    groups: Vec<Option<SequenceActivationRect>>,
    group_by_start_id: FxHashMap<&'a str, usize>,
    fill: String,
    stroke: String,
}

pub(super) fn build_sequence_activation_plan<'a>(
    model: &'a SequenceSvgModel,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    activation_width: f64,
) -> SequenceActivationPlan<'a> {
    // Mermaid 11.15 draws activation rectangles through `svgDraw.getNoteRect()`, whose SVG
    // attributes are hard-coded; theme `activationBkgColor` is emitted in CSS but does not change
    // the rect `fill` attribute in the baseline SVGs.
    let fill = "#EDF2AE".to_string();
    let stroke = "#666".to_string();

    let mut last_line_y: Option<f64> = None;
    let mut activation_stacks: std::collections::BTreeMap<&str, Vec<SequenceActivationStart>> =
        std::collections::BTreeMap::new();
    let mut groups: Vec<Option<SequenceActivationRect>> = Vec::new();
    let mut group_by_start_id: FxHashMap<&str, usize> =
        FxHashMap::with_capacity_and_hasher(model.messages.len(), Default::default());

    for msg in &model.messages {
        if let Some(y) = msg_line_y(edges_by_id, &msg.id) {
            last_line_y = Some(y);
        }

        match msg.message_type {
            // ACTIVE_START
            17 => {
                let Some(actor_id) = msg.from.as_deref() else {
                    continue;
                };
                let Some(cx) = actor_center_x(nodes_by_id, actor_id) else {
                    continue;
                };
                let has_any_activation = !activation_stacks.is_empty();
                let stack = activation_stacks.entry(actor_id).or_default();
                let stacked_size = stack.len();
                let startx = sequence_activation_start_x(cx, stacked_size, activation_width);

                let starty = last_line_y
                    .or_else(|| lifeline_y(edges_by_id, actor_id).map(|(y0, _y1)| y0))
                    .unwrap_or(0.0);
                let starty = if last_line_y.is_some() && has_any_activation {
                    starty + 2.0
                } else {
                    starty
                };

                let group_index = groups.len();
                groups.push(None);
                group_by_start_id.insert(msg.id.as_str(), group_index);
                stack.push(SequenceActivationStart {
                    startx,
                    starty,
                    group_index,
                });
            }
            // ACTIVE_END
            18 => {
                let Some(actor_id) = msg.from.as_deref() else {
                    continue;
                };
                let Some(stack) = activation_stacks.get_mut(actor_id) else {
                    continue;
                };
                let Some(start) = stack.pop() else {
                    continue;
                };

                let mut starty = start.starty;
                let mut vertical_pos = last_line_y.unwrap_or(starty);
                if starty + 18.0 > vertical_pos {
                    starty = vertical_pos - 6.0;
                    vertical_pos += 12.0;
                }

                let class_idx = stack.len() % 3;
                let rect = SequenceActivationRect {
                    startx: start.startx,
                    starty,
                    width: activation_width,
                    height: (vertical_pos - starty).max(0.0),
                    class_idx,
                };
                if let Some(slot) = groups.get_mut(start.group_index) {
                    *slot = Some(rect);
                }
            }
            _ => {}
        }

        let _ = msg.activate;
    }

    SequenceActivationPlan {
        groups,
        group_by_start_id,
        fill,
        stroke,
    }
}

pub(super) fn render_sequence_activation_group(
    out: &mut String,
    plan: &SequenceActivationPlan,
    message_id: &str,
) {
    let Some(group_index) = plan.group_by_start_id.get(message_id).copied() else {
        return;
    };

    // Mermaid creates a `<g>` placeholder at ACTIVE_START time and inserts the
    // `<rect class="activation{0..2}">` once ACTIVE_END is encountered.
    out.push_str("<g>");
    if let Some(Some(a)) = plan.groups.get(group_index) {
        let _ = write!(
            out,
            r##"<rect x="{x}" y="{y}" fill="{fill}" stroke="{stroke}" width="{w}" height="{h}" class="activation{idx}"/>"##,
            x = fmt(a.startx),
            y = fmt(a.starty),
            w = fmt(a.width),
            h = fmt(a.height),
            idx = a.class_idx,
            fill = escape_xml(&plan.fill),
            stroke = escape_xml(&plan.stroke),
        );
    }
    out.push_str("</g>");
}

fn actor_center_x(nodes_by_id: &FxHashMap<&str, &LayoutNode>, actor_id: &str) -> Option<f64> {
    let node_id = format!("actor-top-{actor_id}");
    nodes_by_id.get(node_id.as_str()).copied().map(|n| n.x)
}

fn lifeline_y(
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    actor_id: &str,
) -> Option<(f64, f64)> {
    let edge_id = format!("lifeline-{actor_id}");
    let e = edges_by_id.get(edge_id.as_str()).copied()?;
    let y0 = e.points.first()?.y;
    let y1 = e.points.last()?.y;
    Some((y0, y1))
}

fn msg_line_y(
    edges_by_id: &FxHashMap<&str, &crate::model::LayoutEdge>,
    msg_id: &str,
) -> Option<f64> {
    let edge_id = format!("msg-{msg_id}");
    let e = edges_by_id.get(edge_id.as_str()).copied()?;
    Some(e.points.first()?.y)
}

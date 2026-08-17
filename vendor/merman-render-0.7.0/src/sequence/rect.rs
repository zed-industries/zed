use super::constants::{SEQUENCE_FRAME_GEOM_PAD_PX, SEQUENCE_FRAME_SIDE_PAD_PX};
use crate::model::{LayoutEdge, LayoutNode};
use merman_core::diagrams::sequence::{SequenceDiagramRenderModel, SequenceMessage};
use merman_core::geom::Box2;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(super) struct SequenceRectOpen {
    start_id: String,
    top_y: f64,
    bounds: Option<Box2>,
}

impl SequenceRectOpen {
    pub(super) fn new(start_id: String, top_y: f64) -> Self {
        Self {
            start_id,
            top_y,
            bounds: None,
        }
    }

    pub(super) fn include_min_max(&mut self, min_x: f64, max_x: f64, max_y: f64) {
        let r = Box2::from_min_max(min_x, self.top_y, max_x, max_y);
        if let Some(ref mut cur) = self.bounds {
            cur.union(r);
        } else {
            self.bounds = Some(r);
        }
    }

    pub(super) fn close(self, actor_centers_x: &[f64]) -> ClosedSequenceRect {
        let rect_left = self.bounds.map(|b| b.min_x()).unwrap_or_else(|| {
            actor_centers_x
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min)
                - SEQUENCE_FRAME_SIDE_PAD_PX
        });
        let rect_right = self.bounds.map(|b| b.max_x()).unwrap_or_else(|| {
            actor_centers_x
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max)
                + SEQUENCE_FRAME_SIDE_PAD_PX
        });
        let rect_bottom = self
            .bounds
            .map(|b| b.max_y() + SEQUENCE_FRAME_GEOM_PAD_PX)
            .unwrap_or(self.top_y + SEQUENCE_FRAME_GEOM_PAD_PX);
        let rect_w = (rect_right - rect_left).max(1.0);
        let rect_h = (rect_bottom - self.top_y).max(1.0);

        ClosedSequenceRect {
            node: LayoutNode {
                id: format!("rect-{}", self.start_id),
                x: rect_left + rect_w / 2.0,
                y: self.top_y + rect_h / 2.0,
                width: rect_w,
                height: rect_h,
                is_cluster: false,
                label_width: None,
                label_height: None,
            },
            left: rect_left,
            right: rect_right,
            bottom: rect_bottom,
        }
    }
}

pub(super) struct ClosedSequenceRect {
    pub(super) node: LayoutNode,
    pub(super) left: f64,
    pub(super) right: f64,
    pub(super) bottom: f64,
}

pub(super) fn sequence_rect_stack_x_bounds(
    model: &SequenceDiagramRenderModel,
    actor_index: &HashMap<&str, usize>,
    actor_centers_x: &[f64],
    edges: &[LayoutEdge],
    nodes: &[LayoutNode],
    actor_width_min: f64,
    box_margin: f64,
) -> HashMap<String, (f64, f64)> {
    let edges_by_id: HashMap<&str, &LayoutEdge> =
        edges.iter().map(|e| (e.id.as_str(), e)).collect();
    let nodes_by_id: HashMap<&str, &LayoutNode> =
        nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    let mut stack: Vec<StackItem> = Vec::new();
    let mut rect_bounds: HashMap<String, (f64, f64)> = HashMap::new();

    for msg in &model.messages {
        match msg.message_type {
            10 | 12 | 15 | 19 | 27 | 30 | 32 => stack.push(StackItem::Control),
            11 | 14 | 16 | 21 | 29 | 31 => {
                let _ = stack.pop();
            }
            22 => stack.push(StackItem::Rect {
                start_id: msg.id.clone(),
                min_x: f64::INFINITY,
                max_x: f64::NEG_INFINITY,
            }),
            23 => {
                if let Some(StackItem::Rect {
                    start_id,
                    min_x,
                    max_x,
                }) = stack.pop()
                {
                    if min_x.is_finite() && max_x.is_finite() {
                        rect_bounds.insert(start_id, (min_x, max_x));
                    }
                }
            }
            _ => {
                if stack.is_empty() {
                    continue;
                }
                if let Some((x1, x2)) = message_x_range(
                    msg,
                    actor_index,
                    actor_centers_x,
                    &edges_by_id,
                    &nodes_by_id,
                    actor_width_min,
                ) {
                    update_stack(&mut stack, x1, x2, box_margin);
                }
            }
        }
    }

    rect_bounds
}

#[derive(Debug, Clone)]
enum StackItem {
    Rect {
        start_id: String,
        min_x: f64,
        max_x: f64,
    },
    Control,
}

fn update_stack(stack: &mut [StackItem], x1: f64, x2: f64, box_margin: f64) {
    let len = stack.len();
    for (idx, item) in stack.iter_mut().enumerate() {
        let n = (len - idx) as f64;
        if let StackItem::Rect { min_x, max_x, .. } = item {
            *min_x = min_x.min(x1 - n * box_margin);
            *max_x = max_x.max(x2 + n * box_margin);
        }
    }
}

fn message_x_range(
    msg: &SequenceMessage,
    actor_index: &HashMap<&str, usize>,
    actor_centers_x: &[f64],
    edges_by_id: &HashMap<&str, &LayoutEdge>,
    nodes_by_id: &HashMap<&str, &LayoutNode>,
    actor_width_min: f64,
) -> Option<(f64, f64)> {
    if msg.message_type == 2 {
        let note_id = format!("note-{}", msg.id);
        let n = nodes_by_id.get(note_id.as_str()).copied()?;
        return Some((n.x - n.width / 2.0, n.x + n.width / 2.0));
    }

    let (Some(from), Some(to)) = (msg.from.as_deref(), msg.to.as_deref()) else {
        return None;
    };
    let edge_id = format!("msg-{}", msg.id);
    let e = edges_by_id.get(edge_id.as_str()).copied()?;

    if from == to {
        let line_x = e
            .points
            .first()
            .map(|p| p.x)
            .or_else(|| actor_index.get(from).map(|&i| actor_centers_x[i] + 1.0))?;
        let label_width = e.label.as_ref().map(|label| label.width).unwrap_or(1.0);
        let dx = (label_width / 2.0).max(actor_width_min / 2.0);
        return Some((line_x - dx, line_x + dx));
    }

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    for p in &e.points {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
    }
    if !min_x.is_finite() || !max_x.is_finite() {
        return None;
    }
    Some((min_x, max_x))
}

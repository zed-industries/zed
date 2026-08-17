use super::constants::{
    SEQUENCE_FRAME_GEOM_PAD_PX, SEQUENCE_FRAME_SIDE_PAD_PX, SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX,
};
use crate::model::{LayoutEdge, LayoutNode};
use merman_core::diagrams::sequence::SequenceDiagramRenderModel;
use std::collections::HashMap;

const BLOCK_FRAME_TOP_OFFSET_PX: f64 = 79.0;
const BLOCK_CRITICAL_TOP_OFFSET_PX: f64 = 93.0;
const BLOCK_CRITICAL_LEFT_PAD_PX: f64 = 9.0;

#[derive(Debug, Clone, Copy)]
pub(super) struct SequenceBlockBounds {
    pub(super) min_x: f64,
    pub(super) max_x: f64,
    pub(super) max_y: f64,
}

pub(super) fn sequence_block_bounds(
    model: &SequenceDiagramRenderModel,
    nodes: &[LayoutNode],
    edges: &[LayoutEdge],
) -> Option<SequenceBlockBounds> {
    let refs = LayoutRefs::new(model, nodes, edges);
    let mut bounds = BlockBoundsAccumulator::new();
    let mut stack: Vec<BlockStackEntry> = Vec::new();

    for msg in &model.messages {
        match msg.message_type {
            10 => stack.push(BlockStackEntry::Loop { items: Vec::new() }),
            11 => {
                if let Some(BlockStackEntry::Loop { items }) = stack.pop() {
                    bounds.include_items(&refs, &items, BLOCK_FRAME_TOP_OFFSET_PX);
                }
            }
            15 => stack.push(BlockStackEntry::Opt { items: Vec::new() }),
            16 => {
                if let Some(BlockStackEntry::Opt { items }) = stack.pop() {
                    bounds.include_items(&refs, &items, BLOCK_FRAME_TOP_OFFSET_PX);
                }
            }
            30 => stack.push(BlockStackEntry::Break { items: Vec::new() }),
            31 => {
                if let Some(BlockStackEntry::Break { items }) = stack.pop() {
                    bounds.include_items(&refs, &items, BLOCK_CRITICAL_TOP_OFFSET_PX);
                }
            }
            12 => stack.push(BlockStackEntry::Alt {
                sections: vec![Vec::new()],
            }),
            13 => {
                if let Some(BlockStackEntry::Alt { sections }) = stack.last_mut() {
                    sections.push(Vec::new());
                }
            }
            14 => {
                if let Some(BlockStackEntry::Alt { sections }) = stack.pop() {
                    let items = flatten_sections(sections);
                    bounds.include_items(&refs, &items, BLOCK_FRAME_TOP_OFFSET_PX);
                }
            }
            19 | 32 => stack.push(BlockStackEntry::Par {
                sections: vec![Vec::new()],
            }),
            20 => {
                if let Some(BlockStackEntry::Par { sections }) = stack.last_mut() {
                    sections.push(Vec::new());
                }
            }
            21 => {
                if let Some(BlockStackEntry::Par { sections }) = stack.pop() {
                    let items = flatten_sections(sections);
                    bounds.include_items(&refs, &items, BLOCK_FRAME_TOP_OFFSET_PX);
                }
            }
            27 => stack.push(BlockStackEntry::Critical {
                sections: vec![Vec::new()],
            }),
            28 => {
                if let Some(BlockStackEntry::Critical { sections }) = stack.last_mut() {
                    sections.push(Vec::new());
                }
            }
            29 => {
                if let Some(BlockStackEntry::Critical { sections }) = stack.pop() {
                    let section_count = sections.len();
                    let items = flatten_sections(sections);
                    bounds.include_critical_items(&refs, &items, section_count);
                }
            }
            2 => push_item_to_active_blocks(&mut stack, msg.id.as_str()),
            _ => {
                if msg.from.is_some() && msg.to.is_some() {
                    push_item_to_active_blocks(&mut stack, msg.id.as_str());
                }
            }
        }
    }

    bounds.into_bounds()
}

struct LayoutRefs<'a> {
    nodes_by_id: HashMap<&'a str, &'a LayoutNode>,
    edges_by_id: HashMap<&'a str, &'a LayoutEdge>,
    msg_endpoints: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> LayoutRefs<'a> {
    fn new(
        model: &'a SequenceDiagramRenderModel,
        nodes: &'a [LayoutNode],
        edges: &'a [LayoutEdge],
    ) -> Self {
        let nodes_by_id = nodes.iter().map(|n| (n.id.as_str(), n)).collect();
        let edges_by_id = edges.iter().map(|e| (e.id.as_str(), e)).collect();

        let mut msg_endpoints = HashMap::new();
        for msg in &model.messages {
            let (Some(from), Some(to)) = (msg.from.as_deref(), msg.to.as_deref()) else {
                continue;
            };
            msg_endpoints.insert(msg.id.as_str(), (from, to));
        }

        Self {
            nodes_by_id,
            edges_by_id,
            msg_endpoints,
        }
    }

    fn item_y_range(&self, item_id: &str) -> Option<(f64, f64)> {
        // Mermaid's self-message branch expands bounds by 60px below the message line y
        // coordinate (see the `+ 30 + totalOffset` bottom coordinate, where `totalOffset`
        // already includes a `+30` bump).
        let edge_id = format!("msg-{item_id}");
        if let Some(e) = self.edges_by_id.get(edge_id.as_str()).copied() {
            let y = e.points.first()?.y;
            let extra = self
                .msg_endpoints
                .get(item_id)
                .copied()
                .filter(|(from, to)| from == to)
                .map(|_| SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX)
                .unwrap_or(0.0);
            return Some((y, y + extra));
        }

        let node_id = format!("note-{item_id}");
        let n = self.nodes_by_id.get(node_id.as_str()).copied()?;
        let top = n.y - n.height / 2.0;
        let bottom = n.y + n.height / 2.0;
        Some((top, bottom))
    }

    fn item_ids_y_range(&self, item_ids: &[String]) -> Option<(f64, f64)> {
        item_ids
            .iter()
            .filter_map(|id| self.item_y_range(id))
            .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1)))
    }

    fn frame_x_from_item_ids(&self, item_ids: &[String]) -> Option<FrameX> {
        let mut min_cx = f64::INFINITY;
        let mut max_cx = f64::NEG_INFINITY;
        let mut min_left = f64::INFINITY;
        let mut geom_min_x = f64::INFINITY;
        let mut geom_max_x = f64::NEG_INFINITY;

        for id in item_ids {
            // Notes contribute directly via their node bounds.
            let note_id = format!("note-{id}");
            if let Some(n) = self.nodes_by_id.get(note_id.as_str()).copied() {
                geom_min_x = geom_min_x.min(n.x - n.width / 2.0 - SEQUENCE_FRAME_GEOM_PAD_PX);
                geom_max_x = geom_max_x.max(n.x + n.width / 2.0 + SEQUENCE_FRAME_GEOM_PAD_PX);
            }

            let Some((from, to)) = self.msg_endpoints.get(id.as_str()).copied() else {
                continue;
            };
            for actor_id in [from, to] {
                let actor_node_id = format!("actor-top-{actor_id}");
                let Some(n) = self.nodes_by_id.get(actor_node_id.as_str()).copied() else {
                    continue;
                };
                min_cx = min_cx.min(n.x);
                max_cx = max_cx.max(n.x);
                min_left = min_left.min(n.x - n.width / 2.0);
            }

            // Message edges can overflow via label widths.
            let edge_id = format!("msg-{id}");
            if let Some(e) = self.edges_by_id.get(edge_id.as_str()).copied() {
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

        Some(FrameX {
            left: x1,
            right: x2,
            min_left,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct FrameX {
    left: f64,
    right: f64,
    min_left: f64,
}

struct BlockBoundsAccumulator {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl BlockBoundsAccumulator {
    fn new() -> Self {
        Self {
            min_x: f64::INFINITY,
            min_y: f64::INFINITY,
            max_x: f64::NEG_INFINITY,
            max_y: f64::NEG_INFINITY,
        }
    }

    fn include_items(&mut self, refs: &LayoutRefs<'_>, items: &[String], top_offset: f64) {
        let Some(frame_x) = refs.frame_x_from_item_ids(items) else {
            return;
        };
        let Some((y0, y1)) = refs.item_ids_y_range(items) else {
            return;
        };
        self.include_frame(
            frame_x.left,
            frame_x.right,
            y0 - top_offset,
            y1 + SEQUENCE_FRAME_GEOM_PAD_PX,
        );
    }

    fn include_critical_items(
        &mut self,
        refs: &LayoutRefs<'_>,
        items: &[String],
        section_count: usize,
    ) {
        let Some(mut frame_x) = refs.frame_x_from_item_ids(items) else {
            return;
        };
        let Some((y0, y1)) = refs.item_ids_y_range(items) else {
            return;
        };
        if frame_x.min_left.is_finite() && !items.is_empty() && section_count > 1 {
            frame_x.left = frame_x
                .left
                .min(frame_x.min_left - BLOCK_CRITICAL_LEFT_PAD_PX);
        }
        self.include_frame(
            frame_x.left,
            frame_x.right,
            y0 - BLOCK_FRAME_TOP_OFFSET_PX,
            y1 + SEQUENCE_FRAME_GEOM_PAD_PX,
        );
    }

    fn include_frame(&mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) {
        self.min_x = self.min_x.min(min_x);
        self.max_x = self.max_x.max(max_x);
        self.min_y = self.min_y.min(min_y);
        self.max_y = self.max_y.max(max_y);
    }

    fn into_bounds(self) -> Option<SequenceBlockBounds> {
        if self.min_x.is_finite() && self.min_y.is_finite() {
            Some(SequenceBlockBounds {
                min_x: self.min_x,
                max_x: self.max_x,
                max_y: self.max_y,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum BlockStackEntry {
    Loop { items: Vec<String> },
    Opt { items: Vec<String> },
    Break { items: Vec<String> },
    Alt { sections: Vec<Vec<String>> },
    Par { sections: Vec<Vec<String>> },
    Critical { sections: Vec<Vec<String>> },
}

fn push_item_to_active_blocks(stack: &mut [BlockStackEntry], item_id: &str) {
    for entry in stack.iter_mut() {
        match entry {
            BlockStackEntry::Alt { sections }
            | BlockStackEntry::Par { sections }
            | BlockStackEntry::Critical { sections } => {
                if let Some(cur) = sections.last_mut() {
                    cur.push(item_id.to_string());
                }
            }
            BlockStackEntry::Loop { items }
            | BlockStackEntry::Opt { items }
            | BlockStackEntry::Break { items } => {
                items.push(item_id.to_string());
            }
        }
    }
}

fn flatten_sections(sections: Vec<Vec<String>>) -> Vec<String> {
    sections.into_iter().flatten().collect()
}

//! Node ordering / crossing minimization.
//!
//! Ported from Dagre's `order` pipeline: barycenters, conflict resolution, and a sweep heuristic
//! that attempts to minimize edge crossings.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relationship {
    InEdges,
    OutEdges,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayerGraphLabel {
    pub root: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct WeightLabel {
    pub weight: f64,
}

pub trait OrderEdgeWeight {
    fn weight(&self) -> f64;
}

impl OrderEdgeWeight for WeightLabel {
    fn weight(&self) -> f64 {
        self.weight
    }
}

impl OrderEdgeWeight for crate::EdgeLabel {
    fn weight(&self) -> f64 {
        self.weight
    }
}

pub trait OrderNodeRange {
    fn rank(&self) -> Option<i32>;
    fn min_rank(&self) -> Option<i32>;
    fn max_rank(&self) -> Option<i32>;
    fn has_min_rank(&self) -> bool {
        self.min_rank().is_some()
    }
    fn border_left_at(&self, _rank: i32) -> Option<String> {
        None
    }
    fn border_right_at(&self, _rank: i32) -> Option<String> {
        None
    }
    fn subgraph_layer_label(&self, _rank: i32) -> Self
    where
        Self: Sized + Default,
    {
        Self::default()
    }
}

impl OrderNodeRange for crate::NodeLabel {
    fn rank(&self) -> Option<i32> {
        self.rank
    }

    fn min_rank(&self) -> Option<i32> {
        self.min_rank
    }

    fn max_rank(&self) -> Option<i32> {
        self.max_rank
    }

    fn has_min_rank(&self) -> bool {
        self.min_rank.is_some()
    }

    fn border_left_at(&self, rank: i32) -> Option<String> {
        self.border_left.get(rank as usize).cloned().unwrap_or(None)
    }

    fn border_right_at(&self, rank: i32) -> Option<String> {
        self.border_right
            .get(rank as usize)
            .cloned()
            .unwrap_or(None)
    }

    fn subgraph_layer_label(&self, rank: i32) -> Self {
        let left = self.border_left_at(rank);
        let right = self.border_right_at(rank);

        Self {
            border_left: vec![left],
            border_right: vec![right],
            ..Default::default()
        }
    }
}

pub trait OrderNodeLabel: OrderNodeRange {
    fn order(&self) -> Option<usize>;
    fn set_order(&mut self, order: usize);

    fn border_left(&self) -> Option<&str> {
        None
    }

    fn border_right(&self) -> Option<&str> {
        None
    }
}

impl OrderNodeLabel for crate::NodeLabel {
    fn order(&self) -> Option<usize> {
        self.order
    }

    fn set_order(&mut self, order: usize) {
        self.order = Some(order);
    }

    fn border_left(&self) -> Option<&str> {
        self.border_left.first().and_then(|v| v.as_deref())
    }

    fn border_right(&self) -> Option<&str> {
        self.border_right.first().and_then(|v| v.as_deref())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct OrderNodeLite {
    rank: Option<i32>,
    order: Option<usize>,
    min_rank: Option<i32>,
    max_rank: Option<i32>,
    border_left: Option<String>,
    border_right: Option<String>,
}

impl OrderNodeLite {
    pub fn from_node<N: OrderNodeLabel>(node: &N) -> Self {
        Self {
            rank: node.rank(),
            order: node.order(),
            min_rank: node.min_rank(),
            max_rank: node.max_rank(),
            border_left: None,
            border_right: None,
        }
    }

    pub fn subgraph_layer_label<N: OrderNodeRange>(node: &N, rank: i32) -> Self {
        Self {
            rank: None,
            order: None,
            min_rank: None,
            max_rank: None,
            border_left: node.border_left_at(rank),
            border_right: node.border_right_at(rank),
        }
    }
}

impl OrderNodeRange for OrderNodeLite {
    fn rank(&self) -> Option<i32> {
        self.rank
    }

    fn min_rank(&self) -> Option<i32> {
        self.min_rank
    }

    fn max_rank(&self) -> Option<i32> {
        self.max_rank
    }
}

impl OrderNodeLabel for OrderNodeLite {
    fn order(&self) -> Option<usize> {
        self.order
    }

    fn set_order(&mut self, order: usize) {
        self.order = Some(order);
    }

    fn border_left(&self) -> Option<&str> {
        self.border_left.as_deref()
    }

    fn border_right(&self) -> Option<&str> {
        self.border_right.as_deref()
    }
}

use crate::{Bounds, Half};
use std::{
    cmp,
    fmt::Debug,
    ops::{Add, Sub},
};

#[derive(Debug)]
pub struct BoundsTree<U, T>
where
    U: Default + Clone + Debug,
    T: Clone + Debug,
{
    root: Option<usize>,
    nodes: Vec<Node<U, T>>,
    stack: Vec<usize>,
}

impl<U, T> BoundsTree<U, T>
where
    U: Clone + Debug + PartialOrd + Add<U, Output = U> + Sub<Output = U> + Half + Default,
    T: Clone + Debug,
{
    pub fn clear(&mut self) {
        self.root = None;
        self.nodes.clear();
        self.stack.clear();
    }

    pub fn insert(&mut self, new_bounds: Bounds<U>, payload: T) -> u32 {
        // If the tree is empty, make the root the new leaf.
        if self.root.is_none() {
            let new_node = self.push_leaf(new_bounds, payload, 1);
            self.root = Some(new_node);
            return 1;
        }

        // Search for the best place to add the new leaf based on heuristics.
        let mut max_intersecting_ordering = 0;
        let mut index = self.root.unwrap();
        while let Node::Internal {
            left,
            right,
            bounds: node_bounds,
            ..
        } = self.node_mut(index)
        {
            let left = *left;
            let right = *right;
            *node_bounds = node_bounds.union(&new_bounds);
            self.stack.push(index);

            // Descend to the best-fit child, based on which one would increase
            // the surface area the least. This attempts to keep the tree balanced
            // in terms of surface area. If there is an intersection with the other child,
            // add its keys to the intersections vector.
            let left_cost = new_bounds.union(self.node(left).bounds()).half_perimeter();
            let right_cost = new_bounds.union(self.node(right).bounds()).half_perimeter();
            if left_cost < right_cost {
                max_intersecting_ordering =
                    self.find_max_ordering(right, &new_bounds, max_intersecting_ordering);
                index = left;
            } else {
                max_intersecting_ordering =
                    self.find_max_ordering(left, &new_bounds, max_intersecting_ordering);
                index = right;
            }
        }

        // We've found a leaf ('index' now refers to a leaf node).
        // We'll insert a new parent node above the leaf and attach our new leaf to it.
        let sibling = index;

        // Check for collision with the located leaf node
        let Node::Leaf {
            bounds: sibling_bounds,
            order: sibling_ordering,
            ..
        } = self.node(index)
        else {
            unreachable!();
        };
        if sibling_bounds.intersects(&new_bounds) {
            max_intersecting_ordering = cmp::max(max_intersecting_ordering, *sibling_ordering);
        }

        let ordering = max_intersecting_ordering + 1;
        let new_node = self.push_leaf(new_bounds, payload, ordering);
        let new_parent = self.push_internal(sibling, new_node);

        // If there was an old parent, we need to update its children indices.
        if let Some(old_parent) = self.stack.last().copied() {
            let Node::Internal { left, right, .. } = self.node_mut(old_parent) else {
                unreachable!();
            };

            if *left == sibling {
                *left = new_parent;
            } else {
                *right = new_parent;
            }
        } else {
            // If the old parent was the root, the new parent is the new root.
            self.root = Some(new_parent);
        }

        for node_index in self.stack.drain(..) {
            let Node::Internal { max_ordering, .. } = &mut self.nodes[node_index] else {
                unreachable!()
            };
            *max_ordering = cmp::max(*max_ordering, ordering);
        }

        ordering
    }

    fn find_max_ordering(&self, index: usize, bounds: &Bounds<U>, mut max_ordering: u32) -> u32 {
        match self.node(index) {
            Node::Leaf {
                bounds: node_bounds,
                order: ordering,
                ..
            } => {
                if bounds.intersects(node_bounds) {
                    max_ordering = cmp::max(*ordering, max_ordering);
                }
            }
            Node::Internal {
                left,
                right,
                bounds: node_bounds,
                max_ordering: node_max_ordering,
                ..
            } => {
                if bounds.intersects(node_bounds) && max_ordering < *node_max_ordering {
                    let left_max_ordering = self.node(*left).max_ordering();
                    let right_max_ordering = self.node(*right).max_ordering();
                    if left_max_ordering > right_max_ordering {
                        max_ordering = self.find_max_ordering(*left, bounds, max_ordering);
                        max_ordering = self.find_max_ordering(*right, bounds, max_ordering);
                    } else {
                        max_ordering = self.find_max_ordering(*right, bounds, max_ordering);
                        max_ordering = self.find_max_ordering(*left, bounds, max_ordering);
                    }
                }
            }
        }
        max_ordering
    }

    fn push_leaf(&mut self, bounds: Bounds<U>, payload: T, order: u32) -> usize {
        self.nodes.push(Node::Leaf {
            bounds,
            payload,
            order,
        });
        self.nodes.len() - 1
    }

    fn push_internal(&mut self, left: usize, right: usize) -> usize {
        let left_node = self.node(left);
        let right_node = self.node(right);
        let new_bounds = left_node.bounds().union(right_node.bounds());
        let max_ordering = cmp::max(left_node.max_ordering(), right_node.max_ordering());
        self.nodes.push(Node::Internal {
            bounds: new_bounds,
            left,
            right,
            max_ordering,
        });
        self.nodes.len() - 1
    }

    #[inline(always)]
    fn node(&self, index: usize) -> &Node<U, T> {
        &self.nodes[index]
    }

    #[inline(always)]
    fn node_mut(&mut self, index: usize) -> &mut Node<U, T> {
        &mut self.nodes[index]
    }
}

impl<U, T> Default for BoundsTree<U, T>
where
    U: Default + Clone + Debug,
    T: Clone + Debug,
{
    fn default() -> Self {
        BoundsTree {
            root: None,
            nodes: Vec::new(),
            stack: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Node<U, T>
where
    U: Clone + Default + Debug,
    T: Clone + Debug,
{
    Leaf {
        bounds: Bounds<U>,
        payload: T,
        order: u32,
    },
    Internal {
        left: usize,
        right: usize,
        bounds: Bounds<U>,
        max_ordering: u32,
    },
}

impl<U, T> Node<U, T>
where
    U: Clone + Default + Debug,
    T: Clone + Debug,
{
    fn bounds(&self) -> &Bounds<U> {
        match self {
            Node::Leaf { bounds, .. } => bounds,
            Node::Internal { bounds, .. } => bounds,
        }
    }

    fn max_ordering(&self) -> u32 {
        match self {
            Node::Leaf {
                order: ordering, ..
            } => *ordering,
            Node::Internal { max_ordering, .. } => *max_ordering,
        }
    }
}

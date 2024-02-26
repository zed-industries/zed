use crate::{Bounds, Half, Point};
use std::{
    cmp,
    fmt::Debug,
    ops::{Add, Sub},
};

#[derive(Debug)]
pub(crate) struct BoundsTree<U, T>
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
        } = &mut self.nodes[index]
        {
            let left = *left;
            let right = *right;
            *node_bounds = node_bounds.union(&new_bounds);
            self.stack.push(index);

            // Descend to the best-fit child, based on which one would increase
            // the surface area the least. This attempts to keep the tree balanced
            // in terms of surface area. If there is an intersection with the other child,
            // add its keys to the intersections vector.
            let left_cost = new_bounds
                .union(&self.nodes[left].bounds())
                .half_perimeter();
            let right_cost = new_bounds
                .union(&self.nodes[right].bounds())
                .half_perimeter();
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
        } = &self.nodes[index]
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
            let Node::Internal { left, right, .. } = &mut self.nodes[old_parent] else {
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
            let Node::Internal {
                max_order: max_ordering,
                ..
            } = &mut self.nodes[node_index]
            else {
                unreachable!()
            };
            *max_ordering = cmp::max(*max_ordering, ordering);
        }

        ordering
    }

    /// Finds all nodes whose bounds contain the given point and pushes their (bounds, payload) pairs onto the result vector.
    pub(crate) fn find_containing(
        &mut self,
        point: &Point<U>,
        result: &mut Vec<BoundsSearchResult<U, T>>,
    ) {
        if let Some(mut index) = self.root {
            self.stack.clear();
            self.stack.push(index);

            while let Some(current_index) = self.stack.pop() {
                match &self.nodes[current_index] {
                    Node::Leaf {
                        bounds,
                        order,
                        data,
                    } => {
                        if bounds.contains(point) {
                            result.push(BoundsSearchResult {
                                bounds: bounds.clone(),
                                order: *order,
                                data: data.clone(),
                            });
                        }
                    }
                    Node::Internal {
                        left,
                        right,
                        bounds,
                        ..
                    } => {
                        if bounds.contains(point) {
                            self.stack.push(*left);
                            self.stack.push(*right);
                        }
                    }
                }
            }
        }
    }

    fn find_max_ordering(&self, index: usize, bounds: &Bounds<U>, mut max_ordering: u32) -> u32 {
        match {
            let this = &self;
            &this.nodes[index]
        } {
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
                max_order: node_max_ordering,
                ..
            } => {
                if bounds.intersects(node_bounds) && max_ordering < *node_max_ordering {
                    let left_max_ordering = self.nodes[*left].max_ordering();
                    let right_max_ordering = self.nodes[*right].max_ordering();
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
            data: payload,
            order,
        });
        self.nodes.len() - 1
    }

    fn push_internal(&mut self, left: usize, right: usize) -> usize {
        let left_node = &self.nodes[left];
        let right_node = &self.nodes[right];
        let new_bounds = left_node.bounds().union(right_node.bounds());
        let max_ordering = cmp::max(left_node.max_ordering(), right_node.max_ordering());
        self.nodes.push(Node::Internal {
            bounds: new_bounds,
            left,
            right,
            max_order: max_ordering,
        });
        self.nodes.len() - 1
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
        order: u32,
        data: T,
    },
    Internal {
        left: usize,
        right: usize,
        bounds: Bounds<U>,
        max_order: u32,
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
            Node::Internal {
                max_order: max_ordering,
                ..
            } => *max_ordering,
        }
    }
}

pub(crate) struct BoundsSearchResult<U: Clone + Default + Debug, T> {
    pub bounds: Bounds<U>,
    pub order: u32,
    pub data: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bounds, Point, Size};

    #[test]
    fn test_insert_and_find_containing() {
        let mut tree = BoundsTree::<f32, String>::default();
        let bounds1 = Bounds {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        let bounds2 = Bounds {
            origin: Point { x: 5.0, y: 5.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        let bounds3 = Bounds {
            origin: Point { x: 10.0, y: 10.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };

        // Insert bounds into the tree
        tree.insert(bounds1.clone(), "Payload 1".to_string());
        tree.insert(bounds2.clone(), "Payload 2".to_string());
        tree.insert(bounds3.clone(), "Payload 3".to_string());

        // Points for testing
        let point_inside_bounds1 = Point { x: 1.0, y: 1.0 };
        let point_inside_bounds1_and_2 = Point { x: 6.0, y: 6.0 };
        let point_inside_bounds2_and_3 = Point { x: 12.0, y: 12.0 };
        let point_outside_all_bounds = Point { x: 21.0, y: 21.0 };

        assert!(!bounds1.contains(&point_inside_bounds2_and_3));
        assert!(!bounds1.contains(&point_outside_all_bounds));
        assert!(bounds2.contains(&point_inside_bounds1_and_2));
        assert!(bounds2.contains(&point_inside_bounds2_and_3));
        assert!(!bounds2.contains(&point_outside_all_bounds));
        assert!(!bounds3.contains(&point_inside_bounds1));
        assert!(bounds3.contains(&point_inside_bounds2_and_3));
        assert!(!bounds3.contains(&point_outside_all_bounds));

        // Test find_containing for different points
        let mut result = Vec::new();
        tree.find_containing(&point_inside_bounds1, &mut result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].data, "Payload 1");

        result.clear();
        tree.find_containing(&point_inside_bounds1_and_2, &mut result);
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|r| r.data == "Payload 1"));
        assert!(result.iter().any(|r| r.data == "Payload 2"));

        result.clear();
        tree.find_containing(&point_inside_bounds2_and_3, &mut result);
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|r| r.data == "Payload 2"));
        assert!(result.iter().any(|r| r.data == "Payload 3"));

        result.clear();
        tree.find_containing(&point_outside_all_bounds, &mut result);
        assert_eq!(result.len(), 0);
    }
}

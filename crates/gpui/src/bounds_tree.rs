use crate::{Bounds, Half};
use std::{
    cmp,
    fmt::Debug,
    ops::{Add, Sub},
    ptr::NonNull,
};

/// Maximum children per internal node (R-tree style branching factor).
/// Higher values = shorter tree = fewer cache misses, but more work per node.
const MAX_CHILDREN: usize = 12;

/// A spatial tree optimized for finding maximum ordering among intersecting bounds.
///
/// This is an R-tree variant specifically designed for the use case of assigning
/// z-order to overlapping UI elements. Key optimizations:
/// - Tracks the leaf with global max ordering for O(1) fast-path queries
/// - Uses higher branching factor (4) for lower tree height
/// - Aggressive pruning during search based on max_order metadata
#[derive(Debug)]
pub(crate) struct BoundsTree<U>
where
    U: Clone + Debug + Default + PartialEq,
{
    /// All nodes stored contiguously for cache efficiency.
    nodes: Vec<Node<U>>,
    // Leaves and search pruning need no child arrays. Keep them out of the node stride,
    // at the cost of a second allocation lane whose capacity also survives clear().
    internal_children: Vec<NodeChildren>,
    /// Index of the root node, if any.
    root: Option<usize>,
    /// Index of the leaf with the highest ordering (for fast-path lookups).
    max_leaf: Option<usize>,
    /// Reusable stack for tree traversal during insertion.
    insert_path: Vec<usize>,
    /// Reusable stack for search operations.
    search_stack: Vec<NonNull<Node<U>>>,
}

/// A node in the bounds tree.
#[derive(Debug, Clone)]
struct Node<U>
where
    U: Clone + Debug + Default + PartialEq,
{
    /// Bounding box containing this node and all descendants.
    bounds: Bounds<U>,
    /// Maximum ordering value in this subtree.
    max_order: u32,
    /// Node-specific data.
    kind: NodeKind,
}

#[derive(Debug, Clone)]
enum NodeKind {
    /// Leaf node containing actual bounds data.
    Leaf {
        /// The ordering assigned to this bounds.
        order: u32,
    },
    /// Internal node with children.
    Internal {
        /// Index into BoundsTree::internal_children.
        children: usize,
    },
}

/// Fixed-size array for child indices, avoiding heap allocation.
#[derive(Debug, Clone)]
struct NodeChildren {
    // Keeps an invariant where the max order child is always at the end
    indices: [usize; MAX_CHILDREN],
    len: u8,
}

impl NodeChildren {
    fn new() -> Self {
        Self {
            indices: [0; MAX_CHILDREN],
            len: 0,
        }
    }

    fn push(&mut self, index: usize) {
        debug_assert!((self.len as usize) < MAX_CHILDREN);
        self.indices[self.len as usize] = index;
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn as_slice(&self) -> &[usize] {
        &self.indices[..self.len as usize]
    }
}

impl<U> BoundsTree<U>
where
    U: Clone
        + Debug
        + PartialEq
        + PartialOrd
        + Add<U, Output = U>
        + Sub<Output = U>
        + Half
        + Default,
{
    /// Clears all nodes from the tree.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.internal_children.clear();
        self.root = None;
        self.max_leaf = None;
        self.insert_path.clear();
        self.search_stack.clear();
    }

    /// Inserts bounds into the tree and returns its assigned ordering.
    ///
    /// The ordering is one greater than the maximum ordering of any
    /// existing bounds that intersect with the new bounds.
    pub fn insert(&mut self, new_bounds: Bounds<U>) -> u32 {
        // Find maximum ordering among intersecting bounds
        let max_intersecting = self.find_max_ordering(&new_bounds);
        let ordering = max_intersecting + 1;

        // Insert the new leaf
        let new_leaf_idx = self.insert_leaf(new_bounds, ordering);

        // Update max_leaf tracking
        self.max_leaf = match self.max_leaf {
            None => Some(new_leaf_idx),
            Some(old_idx) if self.nodes[old_idx].max_order < ordering => Some(new_leaf_idx),
            some => some,
        };

        ordering
    }

    /// Finds the maximum ordering among all bounds that intersect with the query.
    fn find_max_ordering(&mut self, query: &Bounds<U>) -> u32 {
        let Some(root_idx) = self.root else {
            return 0;
        };

        // Fast path: check if the max-ordering leaf intersects
        if let Some(max_idx) = self.max_leaf {
            let max_node = &self.nodes[max_idx];
            if query.intersects(&max_node.bounds) {
                return max_node.max_order;
            }
        }

        // Slow path: search the tree
        self.search_stack.clear();
        self.search_stack.push(NonNull::from(&self.nodes[root_idx]));

        let mut max_found = 0u32;

        while let Some(node) = self.search_stack.pop() {
            // SAFETY: `node` is guaranteed to be valid as the `nodes` stack is unmodified in this function
            // and the `search_stack` only contains pointers from this function call.
            let node = unsafe { node.as_ref() };

            // Pruning: skip if this subtree can't improve our result
            if node.max_order <= max_found {
                continue;
            }

            // Spatial pruning: skip if bounds don't intersect
            if !query.intersects(&node.bounds) {
                continue;
            }

            match &node.kind {
                NodeKind::Leaf { order } => {
                    max_found = cmp::max(max_found, *order);
                }
                NodeKind::Internal { children } => {
                    // Children are maintained with highest max_order at the end.
                    // Push in forward order to highest (last) is popped first.
                    self.search_stack.extend(
                        self.internal_children[*children]
                            .as_slice()
                            .iter()
                            .map(|&child_idx| &self.nodes[child_idx])
                            .filter(|node| node.max_order > max_found)
                            .map(NonNull::from),
                    );
                }
            }
        }

        max_found
    }

    /// Inserts a leaf node with the given bounds and ordering.
    /// Returns the index of the new leaf.
    fn insert_leaf(&mut self, bounds: Bounds<U>, order: u32) -> usize {
        let new_leaf_idx = self.nodes.len();
        self.nodes.push(Node {
            bounds: bounds.clone(),
            max_order: order,
            kind: NodeKind::Leaf { order },
        });

        let Some(root_idx) = self.root else {
            // Tree is empty, new leaf becomes root
            self.root = Some(new_leaf_idx);
            return new_leaf_idx;
        };

        // If root is a leaf, create internal node with both
        if matches!(self.nodes[root_idx].kind, NodeKind::Leaf { .. }) {
            let root_bounds = self.nodes[root_idx].bounds.clone();
            let root_order = self.nodes[root_idx].max_order;

            let mut children = NodeChildren::new();
            // Max end invariant
            if order > root_order {
                children.push(root_idx);
                children.push(new_leaf_idx);
            } else {
                children.push(new_leaf_idx);
                children.push(root_idx);
            }

            let new_root_idx = self.nodes.len();
            let children_index = self.internal_children.len();
            self.internal_children.push(children);
            self.nodes.push(Node {
                bounds: root_bounds.union(&bounds),
                max_order: cmp::max(root_order, order),
                kind: NodeKind::Internal {
                    children: children_index,
                },
            });
            self.root = Some(new_root_idx);
            return new_leaf_idx;
        }

        // Descend to find the best internal node to insert into
        self.insert_path.clear();
        let mut current_idx = root_idx;

        loop {
            let current = &self.nodes[current_idx];
            let NodeKind::Internal { children } = &current.kind else {
                unreachable!("Should only traverse internal nodes");
            };
            let children = &self.internal_children[*children];

            self.insert_path.push(current_idx);

            // Find the best child to descend into
            let mut best_child_idx = children.as_slice()[0];
            let mut best_child_pos = 0;
            let mut best_cost = bounds
                .union(&self.nodes[best_child_idx].bounds)
                .half_perimeter();

            for (pos, &child_idx) in children.as_slice().iter().enumerate().skip(1) {
                let cost = bounds.union(&self.nodes[child_idx].bounds).half_perimeter();
                if cost < best_cost {
                    best_cost = cost;
                    best_child_idx = child_idx;
                    best_child_pos = pos;
                }
            }

            // Check if best child is a leaf or internal
            if matches!(self.nodes[best_child_idx].kind, NodeKind::Leaf { .. }) {
                // Best child is a leaf. Check if current node has room for another child.
                if children.len() < MAX_CHILDREN {
                    // Add new leaf directly to this node
                    let node = &mut self.nodes[current_idx];

                    if let NodeKind::Internal { children } = &mut node.kind {
                        let children = &mut self.internal_children[*children];
                        children.push(new_leaf_idx);
                        // Swap new leaf only if it has the highest max_order
                        if order <= node.max_order {
                            let last = children.len() - 1;
                            children.indices.swap(last - 1, last);
                        }
                    }

                    node.bounds = node.bounds.union(&bounds);
                    node.max_order = cmp::max(node.max_order, order);
                    break;
                } else {
                    // Node is full, create new internal with [best_leaf, new_leaf]
                    let sibling_bounds = self.nodes[best_child_idx].bounds.clone();
                    let sibling_order = self.nodes[best_child_idx].max_order;

                    let mut new_children = NodeChildren::new();
                    // Max end invariant
                    if order > sibling_order {
                        new_children.push(best_child_idx);
                        new_children.push(new_leaf_idx);
                    } else {
                        new_children.push(new_leaf_idx);
                        new_children.push(best_child_idx);
                    }

                    let new_internal_idx = self.nodes.len();
                    let new_internal_max = cmp::max(sibling_order, order);
                    let children_index = self.internal_children.len();
                    self.internal_children.push(new_children);
                    self.nodes.push(Node {
                        bounds: sibling_bounds.union(&bounds),
                        max_order: new_internal_max,
                        kind: NodeKind::Internal {
                            children: children_index,
                        },
                    });

                    // Replace the leaf with the new internal in parent
                    let parent = &mut self.nodes[current_idx];
                    if let NodeKind::Internal { children } = &mut parent.kind {
                        let children = &mut self.internal_children[*children];
                        let children_len = children.len();

                        children.indices[best_child_pos] = new_internal_idx;

                        // If new internal has highest max_order, swap it to the end
                        // to maintain sorting invariant
                        if new_internal_max > parent.max_order {
                            children.indices.swap(best_child_pos, children_len - 1);
                        }
                    }
                    break;
                }
            } else {
                // Best child is internal, continue descent
                current_idx = best_child_idx;
            }
        }

        // Propagate bounds and max_order updates up the tree
        let mut updated_child_idx = None;
        for &node_idx in self.insert_path.iter().rev() {
            let node = &mut self.nodes[node_idx];
            node.bounds = node.bounds.union(&bounds);

            if node.max_order < order {
                node.max_order = order;

                // Swap updated child to end (skip first iteration since the invariant is already handled by previous cases)
                if let Some(child_idx) = updated_child_idx {
                    if let NodeKind::Internal { children } = &mut node.kind {
                        let children = &mut self.internal_children[*children];
                        if let Some(pos) = children.as_slice().iter().position(|&c| c == child_idx)
                        {
                            let last = children.len() - 1;
                            if pos != last {
                                children.indices.swap(pos, last);
                            }
                        }
                    }
                }
            }

            updated_child_idx = Some(node_idx);
        }

        new_leaf_idx
    }
}

impl<U> Default for BoundsTree<U>
where
    U: Clone + Debug + Default + PartialEq,
{
    fn default() -> Self {
        BoundsTree {
            nodes: Vec::new(),
            internal_children: Vec::new(),
            root: None,
            max_leaf: None,
            insert_path: Vec::new(),
            search_stack: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bounds, Point, Size};
    use rand::{Rng, SeedableRng};

    fn rectangle(x: f32, y: f32, width: f32, height: f32) -> Bounds<f32> {
        Bounds {
            origin: Point { x, y },
            size: Size { width, height },
        }
    }

    fn assert_topology<U>(tree: &BoundsTree<U>, leaf_count: usize)
    where
        U: Clone + Debug + Default + PartialEq,
    {
        let mut visited = vec![false; tree.nodes.len()];
        let mut visited_children = vec![false; tree.internal_children.len()];
        let mut pending: Vec<_> = tree.root.into_iter().collect();
        let mut leaves = 0;
        while let Some(index) = pending.pop() {
            assert!(!visited[index], "node {index} is reachable twice");
            visited[index] = true;
            let node = &tree.nodes[index];
            match &node.kind {
                NodeKind::Leaf { order } => {
                    leaves += 1;
                    assert_eq!(node.max_order, *order);
                }
                NodeKind::Internal { children } => {
                    assert!(!visited_children[*children]);
                    visited_children[*children] = true;
                    let children = tree.internal_children[*children].as_slice();
                    assert!((2..=MAX_CHILDREN).contains(&children.len()));
                    let maximum = children
                        .iter()
                        .map(|&child| tree.nodes[child].max_order)
                        .max();
                    assert_eq!(maximum, Some(node.max_order));
                    assert_eq!(
                        children.last().map(|&child| tree.nodes[child].max_order),
                        maximum,
                        "maximum-order child must remain last",
                    );
                    pending.extend_from_slice(children);
                }
            }
        }
        assert_eq!(leaves, leaf_count);
        assert!(visited.into_iter().all(|visited| visited));
        assert!(visited_children.into_iter().all(|visited| visited));
        assert_eq!(tree.nodes.len(), leaves + tree.internal_children.len());
        if let Some(index) = tree.max_leaf {
            assert!(matches!(tree.nodes[index].kind, NodeKind::Leaf { .. }));
            assert_eq!(
                Some(tree.nodes[index].max_order),
                tree.nodes.iter().map(|node| node.max_order).max(),
            );
        } else {
            assert_eq!(leaf_count, 0);
        }
    }

    fn assert_orders<U>(tree: &mut BoundsTree<U>, bounds: &[Bounds<U>], label: &str)
    where
        U: Clone
            + Debug
            + Default
            + PartialEq
            + PartialOrd
            + Add<U, Output = U>
            + Sub<Output = U>
            + Half,
    {
        let mut original: Vec<(Bounds<U>, u32)> = Vec::new();
        for (index, bounds) in bounds.iter().enumerate() {
            // Derive every expected order from earlier oracle results, never tree results.
            let expected = original
                .iter()
                .filter_map(|(previous, order)| previous.intersects(bounds).then_some(*order))
                .max()
                .unwrap_or(0)
                + 1;
            assert_eq!(
                tree.insert(bounds.clone()),
                expected,
                "{label}: insertion {index}"
            );
            original.push((bounds.clone(), expected));
            assert_topology(tree, index + 1);
        }
        for (index, query) in bounds.iter().enumerate() {
            let expected = original
                .iter()
                .filter_map(|(previous, order)| previous.intersects(query).then_some(*order))
                .max()
                .unwrap_or(0);
            assert_eq!(
                tree.find_max_ordering(query),
                expected,
                "{label}: query {index}"
            );
        }
    }

    fn capacities(tree: &BoundsTree<f32>) -> (usize, usize, usize, usize) {
        (
            tree.nodes.capacity(),
            tree.internal_children.capacity(),
            tree.insert_path.capacity(),
            tree.search_stack.capacity(),
        )
    }

    fn report_layout(tree: &BoundsTree<f32>, label: &str) {
        let node_size = std::mem::size_of::<Node<f32>>();
        let children_size = std::mem::size_of::<NodeChildren>();
        eprintln!(
            "{label}: node_f32_bytes={node_size}, node_scaled_pixels_bytes={}, \
             kind_bytes={}, children_bytes={children_size}, \
             leaves={}, internals={}, nodes_len={}, nodes_capacity={}, \
             children_capacity={}, insert_path_capacity={}, search_stack_capacity={}, \
             lane_capacity_bytes={}",
            std::mem::size_of::<Node<crate::ScaledPixels>>(),
            std::mem::size_of::<NodeKind>(),
            tree.nodes.len() - tree.internal_children.len(),
            tree.internal_children.len(),
            tree.nodes.len(),
            tree.nodes.capacity(),
            tree.internal_children.capacity(),
            tree.insert_path.capacity(),
            tree.search_stack.capacity(),
            tree.nodes.capacity() * node_size + tree.internal_children.capacity() * children_size,
        );
    }

    #[test]
    fn test_oracle_patterns_and_clear_reuse() {
        let row: Vec<_> = (0..256)
            .map(|index| rectangle(index as f32 * 3.0, 0.0, 1.0, 1.0))
            .collect();
        let grid: Vec<_> = (0..256)
            .map(|index| {
                rectangle(
                    (index % 16) as f32 * 3.0,
                    (index / 16) as f32 * 3.0,
                    1.0,
                    1.0,
                )
            })
            .collect();
        let mut reversed_row = row.clone();
        reversed_row.reverse();
        let outside_in: Vec<_> = (0..128)
            .flat_map(|index| [row[index], row[255 - index]])
            .collect();
        let nested: Vec<_> = (0..256)
            .map(|index| {
                let inset = index as f32;
                rectangle(inset, inset, 1024.0 - inset * 2.0, 1024.0 - inset * 2.0)
            })
            .collect();
        let mut growing = nested.clone();
        growing.reverse();
        let alternating: Vec<_> = (0..512)
            .map(|index| {
                let cluster = (index % 2) as f32 * 10000.0;
                rectangle(cluster + (index % 17) as f32, (index % 13) as f32, 4.0, 4.0)
            })
            .collect();
        let overlay = rectangle(-100.0, -100.0, 2000.0, 2000.0);
        let mut overlay_first = vec![overlay];
        overlay_first.extend_from_slice(&grid);
        let mut overlay_last = grid.clone();
        overlay_last.push(overlay);
        let mut overlay_middle = grid.clone();
        overlay_middle.insert(128, overlay);
        let touching: Vec<_> = (0..256)
            .map(|index| rectangle(index as f32, 0.0, 1.0, 1.0))
            .collect();
        let degenerate: Vec<_> = (0..256)
            .map(|index| rectangle((index % 7) as f32, 0.0, (index % 3) as f32, 0.0))
            .collect();
        let patterns = [
            ("empty", Vec::new()),
            ("single", vec![overlay]),
            ("disjoint row", row),
            ("disjoint grid", grid),
            ("reversed row", reversed_row),
            ("outside in", outside_in),
            (
                "identical overlaps",
                vec![rectangle(0.0, 0.0, 5.0, 5.0); 256],
            ),
            ("nested shrinking", nested),
            ("nested growing", growing),
            ("alternating clusters", alternating),
            ("overlay first", overlay_first),
            ("overlay last", overlay_last),
            ("overlay middle", overlay_middle),
            ("touching edges", touching),
            ("degenerate", degenerate),
        ];
        let mut tree = BoundsTree::default();
        for (label, bounds) in patterns {
            tree.clear();
            assert_orders(&mut tree, &bounds, label);
            report_layout(&tree, label);
            let retained = capacities(&tree);
            tree.clear();
            assert_topology(&tree, 0);
            assert!(tree.root.is_none());
            assert!(tree.max_leaf.is_none());
            assert!(tree.insert_path.is_empty());
            assert!(tree.search_stack.is_empty());
            assert_eq!(capacities(&tree), retained);
            assert_eq!(tree.find_max_ordering(&overlay), 0);
            assert_orders(&mut tree, &bounds, label);
            assert_eq!(capacities(&tree), retained);
        }
    }

    #[test]
    fn test_oracle_seeded_sparse_and_dense_rectangles() {
        for seed in [0, 1, 44, 1000] {
            for extent in [50.0, 2000.0] {
                let mut random = rand::rngs::StdRng::seed_from_u64(seed);
                let bounds: Vec<_> = (0..1024)
                    .map(|_| {
                        rectangle(
                            random.random_range(-extent..extent),
                            random.random_range(-extent..extent),
                            random.random_range(0.0..50.0),
                            random.random_range(0.0..50.0),
                        )
                    })
                    .collect();
                let label = format!("seed={seed}, extent={extent}");
                let mut tree = BoundsTree::default();
                assert_orders(&mut tree, &bounds, &label);
                report_layout(&tree, &label);
            }
        }
    }

    #[test]
    fn test_oracle_scalar_types() {
        let bounds: Vec<_> = (0..256)
            .map(|index| {
                rectangle(
                    (index % 31) as f32 - 15.0,
                    (index % 17) as f32 - 8.0,
                    (index % 7) as f32,
                    (index % 11) as f32,
                )
            })
            .collect();
        let integer_bounds: Vec<_> = bounds
            .iter()
            .map(|bounds| bounds.map(|value| value as i32))
            .collect();
        assert_orders(&mut BoundsTree::default(), &integer_bounds, "i32");
        let scaled_bounds: Vec<_> = bounds
            .iter()
            .map(|bounds| bounds.map(crate::ScaledPixels::from))
            .collect();
        assert_orders(&mut BoundsTree::default(), &scaled_bounds, "ScaledPixels");
    }

    #[test]
    fn test_insert() {
        let mut tree = BoundsTree::<f32>::default();
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

        // Insert the bounds into the tree and verify the order is correct
        assert_eq!(tree.insert(bounds1), 1);
        assert_eq!(tree.insert(bounds2), 2);
        assert_eq!(tree.insert(bounds3), 3);

        // Insert non-overlapping bounds and verify they can reuse orders
        let bounds4 = Bounds {
            origin: Point { x: 20.0, y: 20.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        let bounds5 = Bounds {
            origin: Point { x: 40.0, y: 40.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        let bounds6 = Bounds {
            origin: Point { x: 25.0, y: 25.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        assert_eq!(tree.insert(bounds4), 1); // bounds4 does not overlap with bounds1, bounds2, or bounds3
        assert_eq!(tree.insert(bounds5), 1); // bounds5 does not overlap with any other bounds
        assert_eq!(tree.insert(bounds6), 2); // bounds6 overlaps with bounds4, so it should have a different order
    }

    #[test]
    fn test_random_iterations() {
        let max_bounds = 100;
        for seed in 1..=1000 {
            // let seed = 44;
            let mut tree = BoundsTree::default();
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
            let mut expected_quads: Vec<(Bounds<f32>, u32)> = Vec::new();

            // Insert a random number of random AABBs into the tree.
            let num_bounds = rng.random_range(1..=max_bounds);
            for _ in 0..num_bounds {
                let min_x: f32 = rng.random_range(-100.0..100.0);
                let min_y: f32 = rng.random_range(-100.0..100.0);
                let width: f32 = rng.random_range(0.0..50.0);
                let height: f32 = rng.random_range(0.0..50.0);
                let bounds = Bounds {
                    origin: Point { x: min_x, y: min_y },
                    size: Size { width, height },
                };

                let expected_ordering = expected_quads
                    .iter()
                    .filter_map(|quad| quad.0.intersects(&bounds).then_some(quad.1))
                    .max()
                    .unwrap_or(0)
                    + 1;
                expected_quads.push((bounds, expected_ordering));

                // Insert the AABB into the tree and collect intersections.
                let actual_ordering = tree.insert(bounds);
                assert_eq!(actual_ordering, expected_ordering);
            }
        }
    }
}

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
/// - Keeps leaves at equal depth by splitting overflowing internal nodes
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

        if matches!(self.nodes[root_idx].kind, NodeKind::Leaf { .. }) {
            let mut children = NodeChildren::new();
            children.push(root_idx);
            children.push(new_leaf_idx);
            self.root = Some(self.push_internal(children));
            return new_leaf_idx;
        }

        self.insert_path.clear();
        let mut current_idx = root_idx;
        loop {
            let current = &self.nodes[current_idx];
            let NodeKind::Internal { children } = &current.kind else {
                unreachable!("Should only traverse internal nodes");
            };
            let children = &self.internal_children[*children];

            self.insert_path.push(current_idx);
            let mut best_child_idx = children.as_slice()[0];
            if matches!(self.nodes[best_child_idx].kind, NodeKind::Leaf { .. }) {
                break;
            }
            let mut best_cost = bounds
                .union(&self.nodes[best_child_idx].bounds)
                .half_perimeter();
            for &child_idx in &children.as_slice()[1..] {
                let cost = bounds.union(&self.nodes[child_idx].bounds).half_perimeter();
                if cost < best_cost {
                    best_cost = cost;
                    best_child_idx = child_idx;
                }
            }
            current_idx = best_child_idx;
        }

        let mut pending = Some(new_leaf_idx);
        let mut updated_child = new_leaf_idx;
        while let Some(index) = self.insert_path.pop() {
            let NodeKind::Internal { children } = self.nodes[index].kind else {
                unreachable!("insertion path contains internal nodes");
            };
            if let Some(sibling) = pending.take() {
                if self.internal_children[children].len() == MAX_CHILDREN {
                    pending = Some(self.split(index, children, sibling));
                    updated_child = index;
                    continue;
                }
                self.internal_children[children].push(sibling);
                self.move_max_to_end(children);
            } else if self.nodes[index].max_order < order {
                let children = &mut self.internal_children[children];
                let position = children
                    .as_slice()
                    .iter()
                    .position(|&child| child == updated_child)
                    .expect("updated child belongs to its parent");
                let last = children.len() - 1;
                children.indices.swap(position, last);
            }
            let node = &mut self.nodes[index];
            node.bounds = node.bounds.union(&bounds);
            node.max_order = cmp::max(node.max_order, order);
            updated_child = index;
        }
        if let Some(sibling) = pending {
            let mut children = NodeChildren::new();
            children.push(root_idx);
            children.push(sibling);
            self.root = Some(self.push_internal(children));
        }
        new_leaf_idx
    }

    fn move_max_to_end(&mut self, children: usize) {
        let children = &mut self.internal_children[children];
        let mut maximum = 0;
        for position in 1..children.len() {
            if self.nodes[children.indices[position]].max_order
                > self.nodes[children.indices[maximum]].max_order
            {
                maximum = position;
            }
        }
        let last = children.len() - 1;
        children.indices.swap(maximum, last);
    }

    fn group_bounds(&self, indices: &[usize]) -> Bounds<U> {
        let mut bounds = self.nodes[indices[0]].bounds.clone();
        for &index in &indices[1..] {
            bounds = bounds.union(&self.nodes[index].bounds);
        }
        bounds
    }

    fn internal_node(&mut self, children: usize) -> Node<U> {
        self.move_max_to_end(children);
        let indices = self.internal_children[children].as_slice();
        Node {
            bounds: self.group_bounds(indices),
            max_order: self.nodes[indices[indices.len() - 1]].max_order,
            kind: NodeKind::Internal { children },
        }
    }

    fn push_internal(&mut self, children: NodeChildren) -> usize {
        let lane = self.internal_children.len();
        self.internal_children.push(children);
        let node = self.internal_node(lane);
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    fn split(&mut self, index: usize, children: usize, sibling: usize) -> usize {
        let mut indices = [sibling; MAX_CHILDREN + 1];
        indices[..MAX_CHILDREN].copy_from_slice(self.internal_children[children].as_slice());
        let mut best_indices = indices;
        let mut best_split = MAX_CHILDREN / 2;
        let mut best_key = None;

        // Compare overlap and the larger/smaller union perimeters lexicographically:
        // summing or subtracting costs would narrow the supported scalar domain.
        // Half-full groups bound height even for identical or nested bounds.
        for axis in 0..2 {
            for position in 1..indices.len() {
                let mut cursor = position;
                while cursor > 0 {
                    let left = &self.nodes[indices[cursor]].bounds.origin;
                    let right = &self.nodes[indices[cursor - 1]].bounds.origin;
                    let precedes = if axis == 0 {
                        left.x < right.x
                    } else {
                        left.y < right.y
                    };
                    if !precedes {
                        break;
                    }
                    indices.swap(cursor, cursor - 1);
                    cursor -= 1;
                }
            }
            for partition in MAX_CHILDREN / 2..=indices.len() - MAX_CHILDREN / 2 {
                let left = self.group_bounds(&indices[..partition]);
                let right = self.group_bounds(&indices[partition..]);
                let left_cost = left.half_perimeter();
                let right_cost = right.half_perimeter();
                let (larger, smaller) = if left_cost < right_cost {
                    (right_cost, left_cost)
                } else {
                    (left_cost, right_cost)
                };
                let key = (left.intersects(&right), larger, smaller);
                if best_key.as_ref().is_none_or(|best| key < *best) {
                    best_key = Some(key);
                    best_indices = indices;
                    best_split = partition;
                }
            }
        }
        let mut left = NodeChildren::new();
        let mut right = NodeChildren::new();
        for &child in &best_indices[..best_split] {
            left.push(child);
        }
        for &child in &best_indices[best_split..] {
            right.push(child);
        }
        self.internal_children[children] = left;
        self.nodes[index] = self.internal_node(children);
        self.push_internal(right)
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
        let mut pending: Vec<_> = tree.root.into_iter().map(|index| (index, 1)).collect();
        let mut leaves = 0;
        let mut leaf_depth = None;
        while let Some((index, depth)) = pending.pop() {
            assert!(!visited[index], "node {index} is reachable twice");
            visited[index] = true;
            let node = &tree.nodes[index];
            match &node.kind {
                NodeKind::Leaf { order } => {
                    leaves += 1;
                    assert_eq!(node.max_order, *order);
                    assert_eq!(*leaf_depth.get_or_insert(depth), depth);
                }
                NodeKind::Internal { children } => {
                    assert!(!visited_children[*children]);
                    visited_children[*children] = true;
                    let children = tree.internal_children[*children].as_slice();
                    let minimum = if Some(index) == tree.root {
                        2
                    } else {
                        MAX_CHILDREN / 2
                    };
                    assert!((minimum..=MAX_CHILDREN).contains(&children.len()));
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
                    pending.extend(children.iter().map(|&child| (child, depth + 1)));
                }
            }
        }
        assert_eq!(leaves, leaf_count);
        let mut power = 1usize;
        let mut height_bound = 1;
        while power < leaf_count {
            power *= MAX_CHILDREN / 2;
            height_bound += 1;
        }
        assert!(leaf_depth.unwrap_or(0) <= height_bound);
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
        let mut height = 0;
        let mut current = tree.root;
        while let Some(index) = current {
            height += 1;
            current = match tree.nodes[index].kind {
                NodeKind::Leaf { .. } => None,
                NodeKind::Internal { children } => {
                    tree.internal_children[children].as_slice().first().copied()
                }
            };
        }
        eprintln!(
            "{label}: height={height}, node_f32_bytes={node_size}, node_scaled_pixels_bytes={}, \
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

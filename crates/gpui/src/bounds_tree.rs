use crate::{Bounds, Half, scene::PrimitiveKind};
use std::{
    cmp::{self, Ordering},
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
/// z-order to overlapping UI elements. Each bounds has an ordering and a kind;
/// bounds sharing an ordering are drawn in the order of their kinds. Key
/// optimizations:
/// - Tracks the leaf with global max key for O(1) fast-path queries
/// - Uses higher branching factor (4) for lower tree height
/// - Aggressive pruning during search based on max_key metadata
#[derive(Debug)]
pub(crate) struct BoundsTree<U>
where
    U: Clone + Debug + Default + PartialEq,
{
    /// All nodes stored contiguously for cache efficiency.
    nodes: Vec<Node<U>>,
    /// Index of the root node, if any.
    root: Option<usize>,
    /// Index of the leaf with the highest key (for fast-path lookups).
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
    bounds: NodeBounds<U>,
    /// Maximum key in this subtree.
    max_key: Key,
    /// Node-specific data.
    kind: NodeKind,
}

/// Bounds stored as corners.
///
/// Storing a union as an origin and size would round its far corner, which
/// could then fall short of a child's. The corners are also not reordered for
/// negative sizes, so `intersects` agrees with [`Bounds::intersects`] for every
/// input.
#[derive(Debug, Clone)]
struct NodeBounds<U> {
    min_x: U,
    min_y: U,
    max_x: U,
    max_y: U,
}

impl<U> NodeBounds<U>
where
    U: Clone + Debug + Default + PartialEq + PartialOrd + Add<U, Output = U> + Sub<Output = U>,
{
    fn from_bounds(bounds: &Bounds<U>) -> Self {
        Self {
            min_x: bounds.origin.x.clone(),
            min_y: bounds.origin.y.clone(),
            max_x: bounds.origin.x.clone() + bounds.size.width.clone(),
            max_y: bounds.origin.y.clone() + bounds.size.height.clone(),
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
    }

    fn union(&self, other: &Self) -> Self {
        Self {
            min_x: min_value(self.min_x.clone(), other.min_x.clone()),
            min_y: min_value(self.min_y.clone(), other.min_y.clone()),
            max_x: max_value(self.max_x.clone(), other.max_x.clone()),
            max_y: max_value(self.max_y.clone(), other.max_y.clone()),
        }
    }

    fn half_perimeter(&self) -> U {
        (self.max_x.clone() - self.min_x.clone()) + (self.max_y.clone() - self.min_y.clone())
    }
}

/// A bounds with an unordered coordinate (such as an `f32` NaN) never intersects
/// anything, so unions skip such coordinates rather than letting them spread into
/// the bounds of every ancestor.
fn min_value<U: PartialOrd>(a: U, b: U) -> U {
    match a.partial_cmp(&b) {
        Some(Ordering::Greater) => b,
        Some(_) => a,
        None => {
            if a.partial_cmp(&a).is_some() {
                a
            } else {
                b
            }
        }
    }
}

fn max_value<U: PartialOrd>(a: U, b: U) -> U {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) => b,
        Some(_) => a,
        None => {
            if a.partial_cmp(&a).is_some() {
                a
            } else {
                b
            }
        }
    }
}

/// The ordering of a bounds, then its kind, which orders the bounds that share
/// an ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    ordering: u32,
    kind: PrimitiveKind,
}

#[derive(Debug, Clone)]
enum NodeKind {
    /// Leaf node containing actual bounds data.
    Leaf { key: Key },
    /// Internal node with children.
    Internal {
        /// Indices of child nodes (2 to MAX_CHILDREN).
        children: NodeChildren,
    },
}

/// Fixed-size array for child indices, avoiding heap allocation.
#[derive(Debug, Clone)]
struct NodeChildren {
    // Keeps an invariant where the max key child is always at the end
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
        self.root = None;
        self.max_leaf = None;
        self.insert_path.clear();
        self.search_stack.clear();
    }

    /// Inserts bounds of the given kind into the tree and returns its assigned
    /// ordering.
    ///
    /// The ordering is one greater than the maximum ordering of any existing
    /// bounds that intersect with the new bounds. The new bounds share that
    /// maximum ordering instead if every intersecting bounds there has a kind
    /// of at most `max_shared_kind`, since those are drawn first.
    pub fn insert(
        &mut self,
        new_bounds: Bounds<U>,
        kind: PrimitiveKind,
        max_shared_kind: PrimitiveKind,
    ) -> u32 {
        let new_bounds = NodeBounds::from_bounds(&new_bounds);
        let ordering = match self.find_max_key(&new_bounds) {
            None => 1,
            Some(max_key) if max_key.kind <= max_shared_kind => max_key.ordering,
            Some(max_key) => max_key.ordering + 1,
        };
        let key = Key { ordering, kind };

        // Insert the new leaf
        let new_leaf_idx = self.insert_leaf(new_bounds, key);

        // Update max_leaf tracking
        self.max_leaf = match self.max_leaf {
            None => Some(new_leaf_idx),
            Some(old_idx) if self.nodes[old_idx].max_key < key => Some(new_leaf_idx),
            some => some,
        };

        ordering
    }

    /// Finds the maximum key among all bounds that intersect with the query.
    fn find_max_key(&mut self, query: &NodeBounds<U>) -> Option<Key> {
        let root_idx = self.root?;

        // Fast path: check if the max-key leaf intersects
        if let Some(max_idx) = self.max_leaf {
            let max_node = &self.nodes[max_idx];
            if query.intersects(&max_node.bounds) {
                return Some(max_node.max_key);
            }
        }

        // Slow path: search the tree
        self.search_stack.clear();
        self.search_stack.push(NonNull::from(&self.nodes[root_idx]));

        // `None` compares below every key.
        let mut max_found = None;

        while let Some(node) = self.search_stack.pop() {
            // SAFETY: `node` is guaranteed to be valid as the `nodes` stack is unmodified in this function
            // and the `search_stack` only contains pointers from this function call.
            let node = unsafe { node.as_ref() };

            // Pruning: skip if this subtree can't improve our result
            if Some(node.max_key) <= max_found {
                continue;
            }

            // Spatial pruning: skip if bounds don't intersect
            if !query.intersects(&node.bounds) {
                continue;
            }

            match &node.kind {
                NodeKind::Leaf { key } => {
                    max_found = cmp::max(max_found, Some(*key));
                }
                NodeKind::Internal { children } => {
                    // Children are maintained with highest max_key at the end.
                    // Push in forward order to highest (last) is popped first.
                    self.search_stack.extend(
                        children
                            .as_slice()
                            .iter()
                            .map(|&child_idx| &self.nodes[child_idx])
                            .filter(|node| Some(node.max_key) > max_found)
                            .map(NonNull::from),
                    );
                }
            }
        }

        max_found
    }

    /// Inserts a leaf node with the given bounds and key.
    /// Returns the index of the new leaf.
    fn insert_leaf(&mut self, bounds: NodeBounds<U>, key: Key) -> usize {
        let new_leaf_idx = self.nodes.len();
        self.nodes.push(Node {
            bounds: bounds.clone(),
            max_key: key,
            kind: NodeKind::Leaf { key },
        });

        let Some(root_idx) = self.root else {
            // Tree is empty, new leaf becomes root
            self.root = Some(new_leaf_idx);
            return new_leaf_idx;
        };

        // If root is a leaf, create internal node with both
        if matches!(self.nodes[root_idx].kind, NodeKind::Leaf { .. }) {
            let root_bounds = self.nodes[root_idx].bounds.clone();
            let root_key = self.nodes[root_idx].max_key;

            let mut children = NodeChildren::new();
            // Max end invariant
            if key > root_key {
                children.push(root_idx);
                children.push(new_leaf_idx);
            } else {
                children.push(new_leaf_idx);
                children.push(root_idx);
            }

            let new_root_idx = self.nodes.len();
            self.nodes.push(Node {
                bounds: root_bounds.union(&bounds),
                max_key: cmp::max(root_key, key),
                kind: NodeKind::Internal { children },
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
                        children.push(new_leaf_idx);
                        // Swap new leaf only if it has the highest max_key
                        if key <= node.max_key {
                            let last = children.len() - 1;
                            children.indices.swap(last - 1, last);
                        }
                    }

                    node.bounds = node.bounds.union(&bounds);
                    node.max_key = cmp::max(node.max_key, key);
                    break;
                } else {
                    // Node is full, create new internal with [best_leaf, new_leaf]
                    let sibling_bounds = self.nodes[best_child_idx].bounds.clone();
                    let sibling_key = self.nodes[best_child_idx].max_key;

                    let mut new_children = NodeChildren::new();
                    // Max end invariant
                    if key > sibling_key {
                        new_children.push(best_child_idx);
                        new_children.push(new_leaf_idx);
                    } else {
                        new_children.push(new_leaf_idx);
                        new_children.push(best_child_idx);
                    }

                    let new_internal_idx = self.nodes.len();
                    let new_internal_max_key = cmp::max(sibling_key, key);
                    self.nodes.push(Node {
                        bounds: sibling_bounds.union(&bounds),
                        max_key: new_internal_max_key,
                        kind: NodeKind::Internal {
                            children: new_children,
                        },
                    });

                    // Replace the leaf with the new internal in parent
                    let parent = &mut self.nodes[current_idx];
                    if let NodeKind::Internal { children } = &mut parent.kind {
                        let children_len = children.len();

                        children.indices[best_child_pos] = new_internal_idx;

                        // If new internal has highest max_key, swap it to the end
                        // to maintain sorting invariant
                        if new_internal_max_key > parent.max_key {
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

        // Propagate bounds and max_key updates up the tree
        let mut updated_child_idx = None;
        for &node_idx in self.insert_path.iter().rev() {
            let node = &mut self.nodes[node_idx];
            node.bounds = node.bounds.union(&bounds);

            if node.max_key < key {
                node.max_key = key;

                // Swap updated child to end (skip first iteration since the invariant is already handled by previous cases)
                if let Some(child_idx) = updated_child_idx {
                    if let NodeKind::Internal { children } = &mut node.kind {
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
    use crate::{Bounds, Point, ScaledPixels, Size};
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_insert() {
        let mut tree = BoundsTree::<f32>::default();
        let (quad, shadow) = (PrimitiveKind::Quad, PrimitiveKind::Shadow);
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
        assert_eq!(tree.insert(bounds1, quad, shadow), 1);
        assert_eq!(tree.insert(bounds2, quad, shadow), 2);
        assert_eq!(tree.insert(bounds3, quad, shadow), 3);

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
        assert_eq!(tree.insert(bounds4, quad, shadow), 1); // bounds4 does not overlap with bounds1, bounds2, or bounds3
        assert_eq!(tree.insert(bounds5, quad, shadow), 1); // bounds5 does not overlap with any other bounds
        assert_eq!(tree.insert(bounds6, quad, shadow), 2); // bounds6 overlaps with bounds4, so it should have a different order
    }

    #[test]
    fn test_random_iterations() {
        let max_bounds = 100;
        for seed in 1..=1000 {
            let mut tree = BoundsTree::default();
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
            let mut expected_quads: Vec<(Bounds<f32>, u32, PrimitiveKind)> = Vec::new();
            let kinds = [
                PrimitiveKind::Shadow,
                PrimitiveKind::Quad,
                PrimitiveKind::Path,
                PrimitiveKind::Underline,
            ];

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
                let kind = kinds[rng.random_range(0..kinds.len())];
                let max_shared_kind = kinds[rng.random_range(0..kinds.len())];

                let max_intersecting = expected_quads
                    .iter()
                    .filter(|quad| quad.0.intersects(&bounds))
                    .map(|quad| (quad.1, quad.2))
                    .max();
                let expected_ordering = match max_intersecting {
                    None => 1,
                    Some((ordering, max_kind)) if max_kind <= max_shared_kind => ordering,
                    Some((ordering, _)) => ordering + 1,
                };
                expected_quads.push((bounds, expected_ordering, kind));

                let actual_ordering = tree.insert(bounds, kind, max_shared_kind);
                assert_eq!(actual_ordering, expected_ordering);
            }
        }
    }

    const ITEM_COUNT: usize = 2_500;
    const INVARIANT_CHECK_INTERVAL: usize = 97;

    const ALL_KINDS: [PrimitiveKind; 9] = [
        PrimitiveKind::Shadow,
        PrimitiveKind::Quad,
        PrimitiveKind::Path,
        PrimitiveKind::Underline,
        PrimitiveKind::MonochromeSprite,
        PrimitiveKind::SubpixelSprite,
        PrimitiveKind::PolychromeSprite,
        PrimitiveKind::Surface,
        PrimitiveKind::NativeView,
    ];

    #[derive(Clone, Copy, Debug)]
    enum KindMode {
        Shared,
        MonochromeSpriteUnderline,
        QuadShadow,
        AllKinds,
    }

    const KIND_MODES: [KindMode; 4] = [
        KindMode::Shared,
        KindMode::MonochromeSpriteUnderline,
        KindMode::QuadShadow,
        KindMode::AllKinds,
    ];

    impl KindMode {
        fn kinds(self, rng: &mut impl Rng) -> (PrimitiveKind, PrimitiveKind) {
            match self {
                KindMode::Shared => (PrimitiveKind::Quad, PrimitiveKind::Quad),
                KindMode::MonochromeSpriteUnderline => {
                    (PrimitiveKind::MonochromeSprite, PrimitiveKind::Underline)
                }
                KindMode::QuadShadow => (PrimitiveKind::Quad, PrimitiveKind::Shadow),
                KindMode::AllKinds => (
                    ALL_KINDS[rng.random_range(0..ALL_KINDS.len())],
                    ALL_KINDS[rng.random_range(0..ALL_KINDS.len())],
                ),
            }
        }
    }

    fn bounds<U>(x: U, y: U, width: U, height: U) -> Bounds<U>
    where
        U: Clone + Debug + Default + PartialEq,
    {
        Bounds {
            origin: Point { x, y },
            size: Size { width, height },
        }
    }

    fn expected_ordering<U>(
        inserted: &[(Bounds<U>, u32, PrimitiveKind)],
        new_bounds: &Bounds<U>,
        max_shared_kind: PrimitiveKind,
    ) -> u32
    where
        U: Clone + Debug + Default + PartialEq + PartialOrd + Add<U, Output = U>,
    {
        let max_intersecting = inserted
            .iter()
            .filter(|(bounds, _, _)| bounds.intersects(new_bounds))
            .map(|(_, ordering, kind)| (*ordering, *kind))
            .max();
        match max_intersecting {
            None => 1,
            Some((ordering, max_kind)) if max_kind <= max_shared_kind => ordering,
            Some((ordering, _)) => ordering + 1,
        }
    }

    /// Inserts every item into `tree` and a brute-force oracle, asserting that
    /// they agree on every ordering and that the tree stays well-formed.
    fn assert_matches_oracle<U>(
        tree: &mut BoundsTree<U>,
        items: &[(Bounds<U>, PrimitiveKind, PrimitiveKind)],
        label: &str,
    ) where
        U: Clone
            + Debug
            + PartialEq
            + PartialOrd
            + Add<U, Output = U>
            + Sub<Output = U>
            + Half
            + Default,
    {
        let mut inserted: Vec<(Bounds<U>, u32, PrimitiveKind)> = Vec::with_capacity(items.len());
        for (item_index, (new_bounds, kind, max_shared_kind)) in items.iter().enumerate() {
            let expected = expected_ordering(&inserted, new_bounds, *max_shared_kind);
            let actual = tree.insert(new_bounds.clone(), *kind, *max_shared_kind);
            assert_eq!(
                actual, expected,
                "{label}: item {item_index} {new_bounds:?} kind {kind:?} max shared {max_shared_kind:?}"
            );
            inserted.push((new_bounds.clone(), expected, *kind));
            if item_index % INVARIANT_CHECK_INTERVAL == 0 {
                assert_invariants(tree, inserted.len(), label);
            }
        }
        assert_invariants(tree, inserted.len(), label);
    }

    fn covers<U: PartialOrd>(outer: &NodeBounds<U>, inner: &NodeBounds<U>) -> bool {
        // Unordered coordinates never intersect anything, so ancestors skip them.
        let is_ordered = |value: &U| value.partial_cmp(value).is_some();
        (!is_ordered(&inner.min_x) || outer.min_x <= inner.min_x)
            && (!is_ordered(&inner.max_x) || inner.max_x <= outer.max_x)
            && (!is_ordered(&inner.min_y) || outer.min_y <= inner.min_y)
            && (!is_ordered(&inner.max_y) || inner.max_y <= outer.max_y)
    }

    fn assert_invariants<U>(tree: &BoundsTree<U>, inserted_count: usize, label: &str)
    where
        U: Clone + Debug + Default + PartialEq + PartialOrd,
    {
        let Some(root) = tree.root else {
            assert!(tree.nodes.is_empty(), "{label}");
            assert!(tree.max_leaf.is_none(), "{label}");
            assert_eq!(inserted_count, 0, "{label}");
            return;
        };

        let mut visited = vec![false; tree.nodes.len()];
        let mut leaf_count = 0;
        let mut stack = vec![root];
        while let Some(node_index) = stack.pop() {
            assert!(!visited[node_index], "{label}: node {node_index} reused");
            visited[node_index] = true;
            let node = &tree.nodes[node_index];
            match &node.kind {
                NodeKind::Leaf { key } => {
                    assert_eq!(node.max_key, *key, "{label}: leaf {node_index}");
                    leaf_count += 1;
                }
                NodeKind::Internal { children } => {
                    let children = children.as_slice();
                    assert!(
                        (2..=MAX_CHILDREN).contains(&children.len()),
                        "{label}: node {node_index} has {} children",
                        children.len()
                    );
                    let mut children_max_key = None;
                    for &child_index in children {
                        let child = &tree.nodes[child_index];
                        assert!(
                            covers(&node.bounds, &child.bounds),
                            "{label}: node {node_index} {:?} does not cover child {child_index} {:?}",
                            node.bounds,
                            child.bounds
                        );
                        children_max_key = cmp::max(children_max_key, Some(child.max_key));
                        stack.push(child_index);
                    }
                    assert_eq!(
                        Some(node.max_key),
                        children_max_key,
                        "{label}: node {node_index} max key"
                    );
                    if let Some(&last_child) = children.last() {
                        assert_eq!(
                            tree.nodes[last_child].max_key, node.max_key,
                            "{label}: node {node_index} last child doesn't hold the max key"
                        );
                    }
                }
            }
        }

        assert!(
            visited.iter().all(|visited| *visited),
            "{label}: unreachable node"
        );
        assert_eq!(leaf_count, inserted_count, "{label}");
        let Some(max_leaf) = tree.max_leaf else {
            panic!("{label}: missing max leaf");
        };
        assert!(
            matches!(tree.nodes[max_leaf].kind, NodeKind::Leaf { .. }),
            "{label}"
        );
        assert_eq!(
            tree.nodes[max_leaf].max_key, tree.nodes[root].max_key,
            "{label}: max leaf"
        );
    }

    fn log_uniform(rng: &mut impl Rng, min: f32, max: f32) -> f32 {
        rng.random_range(min.ln()..max.ln()).exp()
    }

    fn overlap_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|_| {
                bounds(
                    rng.random_range(0.0..200.0),
                    rng.random_range(0.0..200.0),
                    rng.random_range(10.0..100.0),
                    rng.random_range(10.0..100.0),
                )
            })
            .collect()
    }

    fn nested_shrinking_scenario() -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|item_index| {
                let inset = item_index as f32 * 0.2;
                bounds(inset, inset, 1_200.0 - inset * 2.0, 1_000.0 - inset * 2.0)
            })
            .collect()
    }

    fn nested_growing_scenario() -> Vec<Bounds<f32>> {
        let mut items = nested_shrinking_scenario();
        items.reverse();
        items
    }

    fn clusters_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        let centers: Vec<(f32, f32)> = (0..24)
            .map(|_| {
                (
                    rng.random_range(0.0..4_000.0),
                    rng.random_range(0.0..3_000.0),
                )
            })
            .collect();
        (0..ITEM_COUNT)
            .map(|_| {
                let (center_x, center_y) = centers[rng.random_range(0..centers.len())];
                bounds(
                    center_x + rng.random_range(-60.0..60.0),
                    center_y + rng.random_range(-60.0..60.0),
                    rng.random_range(1.0..40.0),
                    rng.random_range(1.0..40.0),
                )
            })
            .collect()
    }

    fn grid_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        let columns = 50;
        (0..ITEM_COUNT)
            .map(|item_index| {
                let column = (item_index % columns) as f32;
                let row = (item_index / columns) as f32;
                if rng.random_ratio(1, 10) {
                    bounds(
                        column * 20.0 - 5.0,
                        row * 20.0 - 5.0,
                        rng.random_range(20.0..90.0),
                        rng.random_range(20.0..90.0),
                    )
                } else {
                    bounds(column * 20.0, row * 20.0, 20.0, 20.0)
                }
            })
            .collect()
    }

    fn stripes_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|item_index| {
                let offset = (item_index % 200) as f32 * 18.0;
                if item_index % 2 == 0 {
                    bounds(0.0, offset, 3_600.0, rng.random_range(4.0..24.0))
                } else {
                    bounds(offset, 0.0, rng.random_range(4.0..24.0), 3_600.0)
                }
            })
            .collect()
    }

    fn overlay_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|item_index| {
                if item_index % 250 == 249 {
                    bounds(
                        rng.random_range(-50.0..50.0),
                        rng.random_range(-50.0..50.0),
                        rng.random_range(800.0..1_200.0),
                        rng.random_range(600.0..900.0),
                    )
                } else {
                    bounds(
                        rng.random_range(0.0..1_000.0),
                        rng.random_range(0.0..800.0),
                        rng.random_range(2.0..60.0),
                        rng.random_range(2.0..20.0),
                    )
                }
            })
            .collect()
    }

    fn random_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|_| {
                bounds(
                    rng.random_range(-500.0..3_000.0),
                    rng.random_range(-500.0..2_000.0),
                    log_uniform(rng, 0.5, 2_000.0),
                    log_uniform(rng, 0.5, 2_000.0),
                )
            })
            .collect()
    }

    fn with_kinds<U: Clone>(
        items: &[Bounds<U>],
        mode: KindMode,
        rng: &mut impl Rng,
    ) -> Vec<(Bounds<U>, PrimitiveKind, PrimitiveKind)>
    where
        U: Debug + Default + PartialEq,
    {
        items
            .iter()
            .map(|item| {
                let (kind, max_shared_kind) = mode.kinds(rng);
                (item.clone(), kind, max_shared_kind)
            })
            .collect()
    }

    fn assert_scenario_matches_oracle(name: &str, items: &[Bounds<f32>], seed: u64) {
        for mode in KIND_MODES {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let label = format!("{name} {mode:?} seed {seed}");
            let items = with_kinds(items, mode, &mut rng);
            assert_matches_oracle(&mut BoundsTree::<f32>::default(), &items, &label);

            let scaled_items: Vec<_> = items
                .iter()
                .map(|(item, kind, max_shared_kind)| {
                    (item.map(ScaledPixels), *kind, *max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::<ScaledPixels>::default(),
                &scaled_items,
                &format!("{label} scaled"),
            );
        }
    }

    #[test]
    fn test_oracle_overlap() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(1);
        assert_scenario_matches_oracle("overlap", &overlap_scenario(&mut rng), 1);
    }

    #[test]
    fn test_oracle_nested_shrinking() {
        assert_scenario_matches_oracle("nested shrinking", &nested_shrinking_scenario(), 2);
    }

    #[test]
    fn test_oracle_nested_growing() {
        assert_scenario_matches_oracle("nested growing", &nested_growing_scenario(), 3);
    }

    #[test]
    fn test_oracle_clusters() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(4);
        assert_scenario_matches_oracle("clusters", &clusters_scenario(&mut rng), 4);
    }

    #[test]
    fn test_oracle_grid() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(5);
        assert_scenario_matches_oracle("grid", &grid_scenario(&mut rng), 5);
    }

    #[test]
    fn test_oracle_stripes() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(6);
        assert_scenario_matches_oracle("stripes", &stripes_scenario(&mut rng), 6);
    }

    #[test]
    fn test_oracle_overlay() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(7);
        assert_scenario_matches_oracle("overlay", &overlay_scenario(&mut rng), 7);
    }

    #[test]
    fn test_oracle_random() {
        for seed in [8, 9, 10] {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            assert_scenario_matches_oracle("random", &random_scenario(&mut rng), seed);
        }
    }

    #[test]
    fn test_union_keeps_exact_corners() {
        let mut tree = BoundsTree::<f32>::default();
        let (quad, path, shadow) = (
            PrimitiveKind::Quad,
            PrimitiveKind::Path,
            PrimitiveKind::Shadow,
        );
        assert_eq!(tree.insert(bounds(0.0, 0.0, 0.1, 10.0), quad, shadow), 1);
        // Stored as an origin and size, the union of these would put its right
        // edge at `-1000.0 + 1000.1`, which rounds to just below `0.1`.
        assert_eq!(
            tree.insert(bounds(-1000.0, 0.0, 1.0, 10.0), path, shadow),
            1
        );
        assert_eq!(
            tree.insert(bounds(0.09999, 0.0, 1.0, 10.0), quad, shadow),
            2
        );
    }

    #[test]
    fn test_touching_edges() {
        let mut tree = BoundsTree::<f32>::default();
        let (quad, shadow) = (PrimitiveKind::Quad, PrimitiveKind::Shadow);
        assert_eq!(tree.insert(bounds(0.0, 0.0, 10.0, 10.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(10.0, 0.0, 10.0, 10.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(0.0, 10.0, 10.0, 10.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(10.0, 10.0, 10.0, 10.0), quad, shadow), 1);
        assert_eq!(
            tree.insert(bounds(-10.0, -10.0, 10.0, 10.0), quad, shadow),
            1
        );
        assert_eq!(tree.insert(bounds(9.0, 9.0, 2.0, 2.0), quad, shadow), 2);

        let mut rng = rand::rngs::StdRng::seed_from_u64(11);
        let items: Vec<_> = (0..ITEM_COUNT)
            .map(|_| {
                let (kind, max_shared_kind) = KindMode::AllKinds.kinds(&mut rng);
                let item = bounds(
                    rng.random_range(0..40) as f32 * 5.0,
                    rng.random_range(0..40) as f32 * 5.0,
                    rng.random_range(0..6) as f32 * 5.0,
                    rng.random_range(0..6) as f32 * 5.0,
                );
                (item, kind, max_shared_kind)
            })
            .collect();
        assert_matches_oracle(&mut BoundsTree::default(), &items, "touching");
    }

    #[test]
    fn test_zero_and_negative_sizes() {
        let mut tree = BoundsTree::<f32>::default();
        let (quad, shadow) = (PrimitiveKind::Quad, PrimitiveKind::Shadow);
        assert_eq!(tree.insert(bounds(0.0, 0.0, 100.0, 100.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(10.0, 10.0, 0.0, 0.0), quad, shadow), 2);
        assert_eq!(tree.insert(bounds(10.0, 10.0, 0.0, 10.0), quad, shadow), 2);
        assert_eq!(
            tree.insert(bounds(50.0, 50.0, -10.0, -10.0), quad, shadow),
            2
        );
        assert_eq!(tree.insert(bounds(50.0, 50.0, 10.0, 10.0), quad, shadow), 2);
        assert_eq!(tree.insert(bounds(200.0, 200.0, 0.0, 0.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(200.0, 200.0, 0.0, 0.0), quad, shadow), 1);
        assert_eq!(
            tree.insert(bounds(300.0, 300.0, -10.0, -10.0), quad, shadow),
            1
        );
        assert_eq!(
            tree.insert(bounds(300.0, 300.0, -10.0, -10.0), quad, shadow),
            1
        );

        let mut rng = rand::rngs::StdRng::seed_from_u64(12);
        for mode in KIND_MODES {
            let items: Vec<_> = (0..ITEM_COUNT)
                .map(|_| {
                    let (kind, max_shared_kind) = mode.kinds(&mut rng);
                    let item = bounds(
                        rng.random_range(-300.0..300.0),
                        rng.random_range(-300.0..300.0),
                        rng.random_range(-80.0..80.0),
                        rng.random_range(-80.0..80.0),
                    );
                    (item, kind, max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::default(),
                &items,
                &format!("negative sizes {mode:?}"),
            );
        }
    }

    #[test]
    fn test_negative_coordinates() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(13);
        let items: Vec<_> = (0..ITEM_COUNT)
            .map(|_| {
                let (kind, max_shared_kind) = KindMode::AllKinds.kinds(&mut rng);
                let item = bounds(
                    rng.random_range(-5_000.0..-1_000.0),
                    rng.random_range(-5_000.0..-1_000.0),
                    log_uniform(&mut rng, 1.0, 1_500.0),
                    log_uniform(&mut rng, 1.0, 1_500.0),
                );
                (item, kind, max_shared_kind)
            })
            .collect();
        assert_matches_oracle(&mut BoundsTree::default(), &items, "negative coordinates");
    }

    #[test]
    fn test_i32() {
        for mode in KIND_MODES {
            let mut rng = rand::rngs::StdRng::seed_from_u64(14);
            let items: Vec<_> = (0..ITEM_COUNT)
                .map(|_| {
                    let (kind, max_shared_kind) = mode.kinds(&mut rng);
                    let item = bounds(
                        rng.random_range(-1_000..1_000),
                        rng.random_range(-1_000..1_000),
                        rng.random_range(-20..200),
                        rng.random_range(-20..200),
                    );
                    (item, kind, max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::<i32>::default(),
                &items,
                &format!("i32 {mode:?}"),
            );
        }
    }

    #[test]
    fn test_clear_reuse() {
        let mut tree = BoundsTree::<f32>::default();
        for seed in 15..18 {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let items = with_kinds(&random_scenario(&mut rng), KindMode::AllKinds, &mut rng);
            tree.clear();
            assert_invariants(&tree, 0, "cleared");
            assert_matches_oracle(&mut tree, &items, &format!("clear reuse seed {seed}"));
        }
    }

    #[test]
    fn test_f32_special_values() {
        let special_values = [
            f32::NAN,
            -f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::MAX,
            f32::MIN,
            0.0,
            -0.0,
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(19);
        let value = |rng: &mut rand::rngs::StdRng, min: f32, max: f32| {
            if rng.random_ratio(1, 8) {
                special_values[rng.random_range(0..special_values.len())]
            } else {
                rng.random_range(min..max)
            }
        };
        for mode in KIND_MODES {
            let items: Vec<_> = (0..ITEM_COUNT)
                .map(|_| {
                    let (kind, max_shared_kind) = mode.kinds(&mut rng);
                    let item = bounds(
                        value(&mut rng, -200.0, 200.0),
                        value(&mut rng, -200.0, 200.0),
                        value(&mut rng, -20.0, 100.0),
                        value(&mut rng, -20.0, 100.0),
                    );
                    (item, kind, max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::<f32>::default(),
                &items,
                &format!("f32 special values {mode:?}"),
            );
        }
    }

    #[test]
    fn test_scaled_pixels_edge_values() {
        let edge_values = [
            -0.0,
            0.0,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::MAX,
            f32::MIN,
            f32::from_bits(0x7fc0_0001),
            f32::from_bits(0xffc0_0001),
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(20);
        let value = |rng: &mut rand::rngs::StdRng, min: f32, max: f32| {
            ScaledPixels(if rng.random_ratio(1, 6) {
                edge_values[rng.random_range(0..edge_values.len())]
            } else {
                rng.random_range(min..max)
            })
        };
        for mode in KIND_MODES {
            let items: Vec<_> = (0..ITEM_COUNT)
                .map(|_| {
                    let (kind, max_shared_kind) = mode.kinds(&mut rng);
                    let item = bounds(
                        value(&mut rng, -200.0, 200.0),
                        value(&mut rng, -200.0, 200.0),
                        value(&mut rng, -20.0, 100.0),
                        value(&mut rng, -20.0, 100.0),
                    );
                    (item, kind, max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::<ScaledPixels>::default(),
                &items,
                &format!("scaled pixels edge values {mode:?}"),
            );
        }
    }
}

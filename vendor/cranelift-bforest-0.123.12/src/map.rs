//! Forest of maps.

use super::{Comparator, Forest, INNER_SIZE, Node, NodeData, NodePool, Path};
use crate::packed_option::PackedOption;
#[cfg(test)]
use alloc::string::String;
#[cfg(test)]
use core::fmt;
use core::marker::PhantomData;

/// Tag type defining forest types for a map.
struct MapTypes<K, V>(PhantomData<(K, V)>);

impl<K, V> Forest for MapTypes<K, V>
where
    K: Copy,
    V: Copy,
{
    type Key = K;
    type Value = V;
    type LeafKeys = [K; INNER_SIZE - 1];
    type LeafValues = [V; INNER_SIZE - 1];

    fn splat_key(key: Self::Key) -> Self::LeafKeys {
        [key; INNER_SIZE - 1]
    }

    fn splat_value(value: Self::Value) -> Self::LeafValues {
        [value; INNER_SIZE - 1]
    }
}

/// Memory pool for a forest of `Map` instances.
pub struct MapForest<K, V>
where
    K: Copy,
    V: Copy,
{
    nodes: NodePool<MapTypes<K, V>>,
}

impl<K, V> MapForest<K, V>
where
    K: Copy,
    V: Copy,
{
    /// Create a new empty forest.
    pub fn new() -> Self {
        Self {
            nodes: NodePool::new(),
        }
    }

    /// Clear all maps in the forest.
    ///
    /// All `Map` instances belong to this forest are invalidated and should no longer be used.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}

/// B-tree mapping from `K` to `V`.
///
/// This is not a general-purpose replacement for `BTreeMap`. See the [module
/// documentation](index.html) for more information about design tradeoffs.
///
/// Maps can be cloned, but that operation should only be used as part of cloning the whole forest
/// they belong to. *Cloning a map does not allocate new memory for the clone*. It creates an alias
/// of the same memory.
#[derive(Clone)]
pub struct Map<K, V>
where
    K: Copy,
    V: Copy,
{
    root: PackedOption<Node>,
    unused: PhantomData<(K, V)>,
}

impl<K, V> Map<K, V>
where
    K: Copy,
    V: Copy,
{
    /// Make an empty map.
    pub fn new() -> Self {
        Self {
            root: None.into(),
            unused: PhantomData,
        }
    }

    /// Is this an empty map?
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Get the value stored for `key`.
    pub fn get<C: Comparator<K>>(&self, key: K, forest: &MapForest<K, V>, comp: &C) -> Option<V> {
        self.root
            .expand()
            .and_then(|root| Path::default().find(key, root, &forest.nodes, comp))
    }

    /// Look up the value stored for `key`.
    ///
    /// If it exists, return the stored key-value pair.
    ///
    /// Otherwise, return the last key-value pair with a key that is less than or equal to `key`.
    ///
    /// If no stored keys are less than or equal to `key`, return `None`.
    pub fn get_or_less<C: Comparator<K>>(
        &self,
        key: K,
        forest: &MapForest<K, V>,
        comp: &C,
    ) -> Option<(K, V)> {
        self.root.expand().and_then(|root| {
            let mut path = Path::default();
            match path.find(key, root, &forest.nodes, comp) {
                Some(v) => Some((key, v)),
                None => path.prev(root, &forest.nodes),
            }
        })
    }

    /// Insert `key, value` into the map and return the old value stored for `key`, if any.
    pub fn insert<C: Comparator<K>>(
        &mut self,
        key: K,
        value: V,
        forest: &mut MapForest<K, V>,
        comp: &C,
    ) -> Option<V> {
        self.cursor(forest, comp).insert(key, value)
    }

    /// Remove `key` from the map and return the removed value for `key`, if any.
    pub fn remove<C: Comparator<K>>(
        &mut self,
        key: K,
        forest: &mut MapForest<K, V>,
        comp: &C,
    ) -> Option<V> {
        let mut c = self.cursor(forest, comp);
        if c.goto(key).is_some() {
            c.remove()
        } else {
            None
        }
    }

    /// Remove all entries.
    pub fn clear(&mut self, forest: &mut MapForest<K, V>) {
        if let Some(root) = self.root.take() {
            forest.nodes.free_tree(root);
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// Remove all key-value pairs where the predicate returns false.
    ///
    /// The predicate is allowed to update the values stored in the map.
    pub fn retain<F>(&mut self, forest: &mut MapForest<K, V>, mut predicate: F)
    where
        F: FnMut(K, &mut V) -> bool,
    {
        let mut path = Path::default();
        if let Some(root) = self.root.expand() {
            path.first(root, &forest.nodes);
        }
        while let Some((node, entry)) = path.leaf_pos() {
            let keep = {
                let (ks, vs) = forest.nodes[node].unwrap_leaf_mut();
                predicate(ks[entry], &mut vs[entry])
            };
            if keep {
                path.next(&forest.nodes);
            } else {
                self.root = path.remove(&mut forest.nodes).into();
            }
        }
    }

    /// Create a cursor for navigating this map. The cursor is initially positioned off the end of
    /// the map.
    pub fn cursor<'a, C: Comparator<K>>(
        &'a mut self,
        forest: &'a mut MapForest<K, V>,
        comp: &'a C,
    ) -> MapCursor<'a, K, V, C> {
        MapCursor::new(self, forest, comp)
    }

    /// Create an iterator traversing this map. The iterator type is `(K, V)`.
    pub fn iter<'a>(&'a self, forest: &'a MapForest<K, V>) -> MapIter<'a, K, V> {
        MapIter {
            root: self.root,
            pool: &forest.nodes,
            path: Path::default(),
        }
    }
}

impl<K, V> Default for Map<K, V>
where
    K: Copy,
    V: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl<K, V> Map<K, V>
where
    K: Copy + fmt::Display,
    V: Copy,
{
    /// Verify consistency.
    fn verify<C: Comparator<K>>(&self, forest: &MapForest<K, V>, comp: &C)
    where
        NodeData<MapTypes<K, V>>: fmt::Display,
    {
        if let Some(root) = self.root.expand() {
            forest.nodes.verify_tree(root, comp);
        }
    }

    /// Get a text version of the path to `key`.
    fn tpath<C: Comparator<K>>(&self, key: K, forest: &MapForest<K, V>, comp: &C) -> String {
        use alloc::string::ToString;
        match self.root.expand() {
            None => "map(empty)".to_string(),
            Some(root) => {
                let mut path = Path::default();
                path.find(key, root, &forest.nodes, comp);
                path.to_string()
            }
        }
    }
}

/// A position in a `Map` used to navigate and modify the ordered map.
///
/// A cursor always points at a key-value pair in the map, or "off the end" which is a position
/// after the last entry in the map.
pub struct MapCursor<'a, K, V, C>
where
    K: 'a + Copy,
    V: 'a + Copy,
    C: 'a + Comparator<K>,
{
    root: &'a mut PackedOption<Node>,
    pool: &'a mut NodePool<MapTypes<K, V>>,
    comp: &'a C,
    path: Path<MapTypes<K, V>>,
}

impl<'a, K, V, C> MapCursor<'a, K, V, C>
where
    K: Copy,
    V: Copy,
    C: Comparator<K>,
{
    /// Create a cursor with a default (off-the-end) location.
    fn new(container: &'a mut Map<K, V>, forest: &'a mut MapForest<K, V>, comp: &'a C) -> Self {
        Self {
            root: &mut container.root,
            pool: &mut forest.nodes,
            comp,
            path: Path::default(),
        }
    }

    /// Is this cursor pointing to an empty map?
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Move cursor to the next key-value pair and return it.
    ///
    /// If the cursor reaches the end, return `None` and leave the cursor at the off-the-end
    /// position.
    pub fn next(&mut self) -> Option<(K, V)> {
        self.path.next(self.pool)
    }

    /// Move cursor to the previous key-value pair and return it.
    ///
    /// If the cursor is already pointing at the first entry, leave it there and return `None`.
    pub fn prev(&mut self) -> Option<(K, V)> {
        self.root
            .expand()
            .and_then(|root| self.path.prev(root, self.pool))
    }

    /// Get the current key, or `None` if the cursor is at the end.
    pub fn key(&self) -> Option<K> {
        self.path
            .leaf_pos()
            .and_then(|(node, entry)| self.pool[node].unwrap_leaf().0.get(entry).cloned())
    }

    /// Get the current value, or `None` if the cursor is at the end.
    pub fn value(&self) -> Option<V> {
        self.path
            .leaf_pos()
            .and_then(|(node, entry)| self.pool[node].unwrap_leaf().1.get(entry).cloned())
    }

    /// Get a mutable reference to the current value, or `None` if the cursor is at the end.
    pub fn value_mut(&mut self) -> Option<&mut V> {
        self.path
            .leaf_pos()
            .and_then(move |(node, entry)| self.pool[node].unwrap_leaf_mut().1.get_mut(entry))
    }

    /// Move this cursor to `key`.
    ///
    /// If `key` is in the map, place the cursor at `key` and return the corresponding value.
    ///
    /// If `key` is not in the set, place the cursor at the next larger element (or the end) and
    /// return `None`.
    pub fn goto(&mut self, elem: K) -> Option<V> {
        self.root.expand().and_then(|root| {
            let v = self.path.find(elem, root, self.pool, self.comp);
            if v.is_none() {
                self.path.normalize(self.pool);
            }
            v
        })
    }

    /// Move this cursor to the first element.
    pub fn goto_first(&mut self) -> Option<V> {
        self.root.map(|root| self.path.first(root, self.pool).1)
    }

    /// Insert `(key, value))` into the map and leave the cursor at the inserted pair.
    ///
    /// If the map did not contain `key`, return `None`.
    ///
    /// If `key` is already present, replace the existing with `value` and return the old value.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.root.expand() {
            None => {
                let root = self.pool.alloc_node(NodeData::leaf(key, value));
                *self.root = root.into();
                self.path.set_root_node(root);
                None
            }
            Some(root) => {
                // TODO: Optimize the case where `self.path` is already at the correct insert pos.
                let old = self.path.find(key, root, self.pool, self.comp);
                if old.is_some() {
                    *self.path.value_mut(self.pool) = value;
                } else {
                    *self.root = self.path.insert(key, value, self.pool).into();
                }
                old
            }
        }
    }

    /// Remove the current entry (if any) and return the mapped value.
    /// This advances the cursor to the next entry after the removed one.
    pub fn remove(&mut self) -> Option<V> {
        let value = self.value();
        if value.is_some() {
            *self.root = self.path.remove(self.pool).into();
        }
        value
    }
}

/// An iterator visiting the key-value pairs of a `Map`.
pub struct MapIter<'a, K, V>
where
    K: 'a + Copy,
    V: 'a + Copy,
{
    root: PackedOption<Node>,
    pool: &'a NodePool<MapTypes<K, V>>,
    path: Path<MapTypes<K, V>>,
}

impl<'a, K, V> Iterator for MapIter<'a, K, V>
where
    K: 'a + Copy,
    V: 'a + Copy,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        // We use `self.root` to indicate if we need to go to the first element. Reset to `None`
        // once we've returned the first element. This also works for an empty tree since the
        // `path.next()` call returns `None` when the path is empty. This also fuses the iterator.
        match self.root.take() {
            Some(root) => Some(self.path.first(root, self.pool)),
            None => self.path.next(self.pool),
        }
    }
}

#[cfg(test)]
impl<'a, K, V, C> MapCursor<'a, K, V, C>
where
    K: Copy + fmt::Display,
    V: Copy + fmt::Display,
    C: Comparator<K>,
{
    fn verify(&self) {
        self.path.verify(self.pool);
        self.root.map(|root| self.pool.verify_tree(root, self.comp));
    }

    /// Get a text version of the path to the current position.
    fn tpath(&self) -> String {
        use alloc::string::ToString;
        self.path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use core::mem;

    #[test]
    fn node_size() {
        // check that nodes are cache line sized when keys and values are 32 bits.
        type F = MapTypes<u32, u32>;
        assert_eq!(mem::size_of::<NodeData<F>>(), 64);
    }

    #[test]
    fn empty() {
        let mut f = MapForest::<u32, f32>::new();
        f.clear();

        let mut m = Map::<u32, f32>::new();
        assert!(m.is_empty());
        m.clear(&mut f);

        assert_eq!(m.get(7, &f, &()), None);
        assert_eq!(m.iter(&f).next(), None);
        assert_eq!(m.get_or_less(7, &f, &()), None);
        m.retain(&mut f, |_, _| unreachable!());

        let mut c = m.cursor(&mut f, &());
        assert!(c.is_empty());
        assert_eq!(c.key(), None);
        assert_eq!(c.value(), None);
        assert_eq!(c.next(), None);
        assert_eq!(c.prev(), None);
        c.verify();
        assert_eq!(c.tpath(), "<empty path>");
        assert_eq!(c.goto_first(), None);
        assert_eq!(c.tpath(), "<empty path>");
    }

    #[test]
    fn inserting() {
        let f = &mut MapForest::<u32, f32>::new();
        let mut m = Map::<u32, f32>::new();

        // The first seven values stay in a single leaf node.
        assert_eq!(m.insert(50, 5.0, f, &()), None);
        assert_eq!(m.insert(50, 5.5, f, &()), Some(5.0));
        assert_eq!(m.insert(20, 2.0, f, &()), None);
        assert_eq!(m.insert(80, 8.0, f, &()), None);
        assert_eq!(m.insert(40, 4.0, f, &()), None);
        assert_eq!(m.insert(60, 6.0, f, &()), None);
        assert_eq!(m.insert(90, 9.0, f, &()), None);
        assert_eq!(m.insert(200, 20.0, f, &()), None);

        m.verify(f, &());

        assert_eq!(
            m.iter(f).collect::<Vec<_>>(),
            [
                (20, 2.0),
                (40, 4.0),
                (50, 5.5),
                (60, 6.0),
                (80, 8.0),
                (90, 9.0),
                (200, 20.0),
            ]
        );

        assert_eq!(m.get(0, f, &()), None);
        assert_eq!(m.get(20, f, &()), Some(2.0));
        assert_eq!(m.get(30, f, &()), None);
        assert_eq!(m.get(40, f, &()), Some(4.0));
        assert_eq!(m.get(50, f, &()), Some(5.5));
        assert_eq!(m.get(60, f, &()), Some(6.0));
        assert_eq!(m.get(70, f, &()), None);
        assert_eq!(m.get(80, f, &()), Some(8.0));
        assert_eq!(m.get(100, f, &()), None);

        assert_eq!(m.get_or_less(0, f, &()), None);
        assert_eq!(m.get_or_less(20, f, &()), Some((20, 2.0)));
        assert_eq!(m.get_or_less(30, f, &()), Some((20, 2.0)));
        assert_eq!(m.get_or_less(40, f, &()), Some((40, 4.0)));
        assert_eq!(m.get_or_less(200, f, &()), Some((200, 20.0)));
        assert_eq!(m.get_or_less(201, f, &()), Some((200, 20.0)));

        {
            let mut c = m.cursor(f, &());
            assert_eq!(c.prev(), Some((200, 20.0)));
            assert_eq!(c.prev(), Some((90, 9.0)));
            assert_eq!(c.prev(), Some((80, 8.0)));
            assert_eq!(c.prev(), Some((60, 6.0)));
            assert_eq!(c.prev(), Some((50, 5.5)));
            assert_eq!(c.prev(), Some((40, 4.0)));
            assert_eq!(c.prev(), Some((20, 2.0)));
            assert_eq!(c.prev(), None);
        }

        // Test some removals where the node stays healthy.
        assert_eq!(m.tpath(50, f, &()), "node0[2]");
        assert_eq!(m.tpath(80, f, &()), "node0[4]");
        assert_eq!(m.tpath(200, f, &()), "node0[6]");

        assert_eq!(m.remove(80, f, &()), Some(8.0));
        assert_eq!(m.tpath(50, f, &()), "node0[2]");
        assert_eq!(m.tpath(80, f, &()), "node0[4]");
        assert_eq!(m.tpath(200, f, &()), "node0[5]");
        assert_eq!(m.remove(80, f, &()), None);
        m.verify(f, &());

        assert_eq!(m.remove(20, f, &()), Some(2.0));
        assert_eq!(m.tpath(50, f, &()), "node0[1]");
        assert_eq!(m.tpath(80, f, &()), "node0[3]");
        assert_eq!(m.tpath(200, f, &()), "node0[4]");
        assert_eq!(m.remove(20, f, &()), None);
        m.verify(f, &());

        // [ 40 50 60 90 200 ]

        {
            let mut c = m.cursor(f, &());
            assert_eq!(c.goto_first(), Some(4.0));
            assert_eq!(c.key(), Some(40));
            assert_eq!(c.value(), Some(4.0));
            assert_eq!(c.next(), Some((50, 5.5)));
            assert_eq!(c.next(), Some((60, 6.0)));
            assert_eq!(c.next(), Some((90, 9.0)));
            assert_eq!(c.next(), Some((200, 20.0)));
            c.verify();
            assert_eq!(c.next(), None);
            c.verify();
        }

        // Removals from the root leaf node beyond underflow.
        assert_eq!(m.remove(200, f, &()), Some(20.0));
        assert_eq!(m.remove(40, f, &()), Some(4.0));
        assert_eq!(m.remove(60, f, &()), Some(6.0));
        m.verify(f, &());
        assert_eq!(m.remove(50, f, &()), Some(5.5));
        m.verify(f, &());
        assert_eq!(m.remove(90, f, &()), Some(9.0));
        m.verify(f, &());
        assert!(m.is_empty());
    }

    #[test]
    fn split_level0_leaf() {
        // Various ways of splitting a full leaf node at level 0.
        let f = &mut MapForest::<u32, f32>::new();

        fn full_leaf(f: &mut MapForest<u32, f32>) -> Map<u32, f32> {
            let mut m = Map::new();
            for n in 1..8 {
                m.insert(n * 10, n as f32 * 1.1, f, &());
            }
            m
        }

        // Insert at front of leaf.
        let mut m = full_leaf(f);
        m.insert(5, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(5, f, &()), Some(4.2));

        // Retain even entries, with altered values.
        m.retain(f, |k, v| {
            *v = (k / 10) as f32;
            (k % 20) == 0
        });
        assert_eq!(
            m.iter(f).collect::<Vec<_>>(),
            [(20, 2.0), (40, 4.0), (60, 6.0)]
        );

        // Insert at back of leaf.
        let mut m = full_leaf(f);
        m.insert(80, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(80, f, &()), Some(4.2));

        // Insert before middle (40).
        let mut m = full_leaf(f);
        m.insert(35, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(35, f, &()), Some(4.2));

        // Insert after middle (40).
        let mut m = full_leaf(f);
        m.insert(45, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(45, f, &()), Some(4.2));

        m.clear(f);
        assert!(m.is_empty());
    }

    #[test]
    fn split_level1_leaf() {
        // Various ways of splitting a full leaf node at level 1.
        let f = &mut MapForest::<u32, f32>::new();

        // Return a map whose root node is a full inner node, and the leaf nodes are all full
        // containing:
        //
        // 110, 120, ..., 170
        // 210, 220, ..., 270
        // ...
        // 810, 820, ..., 870
        fn full(f: &mut MapForest<u32, f32>) -> Map<u32, f32> {
            let mut m = Map::new();

            // Start by inserting elements in order.
            // This should leave 8 leaf nodes with 4 elements in each.
            for row in 1..9 {
                for col in 1..5 {
                    m.insert(row * 100 + col * 10, row as f32 + col as f32 * 0.1, f, &());
                }
            }

            // Then top up the leaf nodes without splitting them.
            for row in 1..9 {
                for col in 5..8 {
                    m.insert(row * 100 + col * 10, row as f32 + col as f32 * 0.1, f, &());
                }
            }

            m
        }

        let mut m = full(f);
        // Verify geometry. Get get node2 as the root and leaves node0, 1, 3, ...
        m.verify(f, &());
        assert_eq!(m.tpath(110, f, &()), "node2[0]--node0[0]");
        assert_eq!(m.tpath(140, f, &()), "node2[0]--node0[3]");
        assert_eq!(m.tpath(210, f, &()), "node2[1]--node1[0]");
        assert_eq!(m.tpath(270, f, &()), "node2[1]--node1[6]");
        assert_eq!(m.tpath(310, f, &()), "node2[2]--node3[0]");
        assert_eq!(m.tpath(810, f, &()), "node2[7]--node8[0]");
        assert_eq!(m.tpath(870, f, &()), "node2[7]--node8[6]");

        {
            let mut c = m.cursor(f, &());
            assert_eq!(c.goto_first(), Some(1.1));
            assert_eq!(c.key(), Some(110));
        }

        // Front of first leaf.
        m.insert(0, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(0, f, &()), Some(4.2));

        // First leaf split 4-4 after appending to LHS.
        f.clear();
        m = full(f);
        m.insert(135, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(135, f, &()), Some(4.2));

        // First leaf split 4-4 after prepending to RHS.
        f.clear();
        m = full(f);
        m.insert(145, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(145, f, &()), Some(4.2));

        // First leaf split 4-4 after appending to RHS.
        f.clear();
        m = full(f);
        m.insert(175, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(175, f, &()), Some(4.2));

        // Left-middle leaf split, ins LHS.
        f.clear();
        m = full(f);
        m.insert(435, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(435, f, &()), Some(4.2));

        // Left-middle leaf split, ins RHS.
        f.clear();
        m = full(f);
        m.insert(445, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(445, f, &()), Some(4.2));

        // Right-middle leaf split, ins LHS.
        f.clear();
        m = full(f);
        m.insert(535, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(535, f, &()), Some(4.2));

        // Right-middle leaf split, ins RHS.
        f.clear();
        m = full(f);
        m.insert(545, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(545, f, &()), Some(4.2));

        // Last leaf split, ins LHS.
        f.clear();
        m = full(f);
        m.insert(835, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(835, f, &()), Some(4.2));

        // Last leaf split, ins RHS.
        f.clear();
        m = full(f);
        m.insert(845, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(845, f, &()), Some(4.2));

        // Front of last leaf.
        f.clear();
        m = full(f);
        m.insert(805, 4.2, f, &());
        m.verify(f, &());
        assert_eq!(m.get(805, f, &()), Some(4.2));

        m.clear(f);
        m.verify(f, &());
    }

    // Make a tree with two barely healthy leaf nodes:
    // [ 10 20 30 40 ] [ 50 60 70 80 ]
    fn two_leaf(f: &mut MapForest<u32, f32>) -> Map<u32, f32> {
        f.clear();
        let mut m = Map::new();
        for n in 1..9 {
            m.insert(n * 10, n as f32, f, &());
        }
        m
    }

    #[test]
    fn remove_level1() {
        let f = &mut MapForest::<u32, f32>::new();
        let mut m = two_leaf(f);

        // Verify geometry.
        m.verify(f, &());
        assert_eq!(m.tpath(10, f, &()), "node2[0]--node0[0]");
        assert_eq!(m.tpath(40, f, &()), "node2[0]--node0[3]");
        assert_eq!(m.tpath(49, f, &()), "node2[0]--node0[4]");
        assert_eq!(m.tpath(50, f, &()), "node2[1]--node1[0]");
        assert_eq!(m.tpath(80, f, &()), "node2[1]--node1[3]");

        // Remove the front entry from a node that stays healthy.
        assert_eq!(m.insert(55, 5.5, f, &()), None);
        assert_eq!(m.remove(50, f, &()), Some(5.0));
        m.verify(f, &());
        assert_eq!(m.tpath(49, f, &()), "node2[0]--node0[4]");
        assert_eq!(m.tpath(50, f, &()), "node2[0]--node0[4]");
        assert_eq!(m.tpath(55, f, &()), "node2[1]--node1[0]");

        // Remove the front entry from the first leaf node: No critical key to update.
        assert_eq!(m.insert(15, 1.5, f, &()), None);
        assert_eq!(m.remove(10, f, &()), Some(1.0));
        m.verify(f, &());

        // [ 15 20 30 40 ] [ 55 60 70 80 ]

        // Remove the front entry from a right-most node that underflows.
        // No rebalancing for the right-most node. Still need critical key update.
        assert_eq!(m.remove(55, f, &()), Some(5.5));
        m.verify(f, &());
        assert_eq!(m.tpath(55, f, &()), "node2[0]--node0[4]");
        assert_eq!(m.tpath(60, f, &()), "node2[1]--node1[0]");

        // [ 15 20 30 40 ] [ 60 70 80 ]

        // Replenish the right leaf.
        assert_eq!(m.insert(90, 9.0, f, &()), None);
        assert_eq!(m.insert(100, 10.0, f, &()), None);
        m.verify(f, &());
        assert_eq!(m.tpath(55, f, &()), "node2[0]--node0[4]");
        assert_eq!(m.tpath(60, f, &()), "node2[1]--node1[0]");

        // [ 15 20 30 40 ] [ 60 70 80 90 100 ]

        // Removing one entry from the left leaf should trigger a rebalancing from the right
        // sibling.
        assert_eq!(m.remove(20, f, &()), Some(2.0));
        m.verify(f, &());

        // [ 15 30 40 60 ] [ 70 80 90 100 ]
        // Check that the critical key was updated correctly.
        assert_eq!(m.tpath(50, f, &()), "node2[0]--node0[3]");
        assert_eq!(m.tpath(60, f, &()), "node2[0]--node0[3]");
        assert_eq!(m.tpath(70, f, &()), "node2[1]--node1[0]");

        // Remove front entry from the left-most leaf node, underflowing.
        // This should cause two leaf nodes to be merged and the root node to go away.
        assert_eq!(m.remove(15, f, &()), Some(1.5));
        m.verify(f, &());
    }

    #[test]
    fn remove_level1_rightmost() {
        let f = &mut MapForest::<u32, f32>::new();
        let mut m = two_leaf(f);

        // [ 10 20 30 40 ] [ 50 60 70 80 ]

        // Remove entries from the right leaf. This doesn't trigger a rebalancing.
        assert_eq!(m.remove(60, f, &()), Some(6.0));
        assert_eq!(m.remove(80, f, &()), Some(8.0));
        assert_eq!(m.remove(50, f, &()), Some(5.0));
        m.verify(f, &());

        // [ 10 20 30 40 ] [ 70 ]
        assert_eq!(m.tpath(50, f, &()), "node2[0]--node0[4]");
        assert_eq!(m.tpath(70, f, &()), "node2[1]--node1[0]");

        // Removing the last entry from the right leaf should cause a collapse.
        assert_eq!(m.remove(70, f, &()), Some(7.0));
        m.verify(f, &());
    }

    // Make a 3-level tree with barely healthy nodes.
    // 1 root, 8 inner nodes, 7*4+5=33 leaf nodes, 4 entries each.
    fn level3_sparse(f: &mut MapForest<u32, f32>) -> Map<u32, f32> {
        f.clear();
        let mut m = Map::new();
        for n in 1..133 {
            m.insert(n * 10, n as f32, f, &());
        }
        m
    }

    #[test]
    fn level3_removes() {
        let f = &mut MapForest::<u32, f32>::new();
        let mut m = level3_sparse(f);
        m.verify(f, &());

        // Check geometry.
        // Root: node11
        // [ node2 170 node10 330 node16 490 node21 650 node26 810 node31 970 node36 1130 node41 ]
        // L1: node11
        assert_eq!(m.tpath(0, f, &()), "node11[0]--node2[0]--node0[0]");
        assert_eq!(m.tpath(10000, f, &()), "node11[7]--node41[4]--node40[4]");

        // 650 is a critical key in the middle of the root.
        assert_eq!(m.tpath(640, f, &()), "node11[3]--node21[3]--node19[3]");
        assert_eq!(m.tpath(650, f, &()), "node11[4]--node26[0]--node20[0]");

        // Deleting 640 triggers a rebalance from node19 to node 20, cascading to n21 -> n26.
        assert_eq!(m.remove(640, f, &()), Some(64.0));
        m.verify(f, &());
        assert_eq!(m.tpath(650, f, &()), "node11[3]--node26[3]--node20[3]");

        // 1130 is in the first leaf of the last L1 node. Deleting it triggers a rebalance node35
        // -> node37, but no rebalance above where there is no right sibling.
        assert_eq!(m.tpath(1130, f, &()), "node11[6]--node41[0]--node35[0]");
        assert_eq!(m.tpath(1140, f, &()), "node11[6]--node41[0]--node35[1]");
        assert_eq!(m.remove(1130, f, &()), Some(113.0));
        m.verify(f, &());
        assert_eq!(m.tpath(1140, f, &()), "node11[6]--node41[0]--node37[0]");
    }

    #[test]
    fn insert_many() {
        let f = &mut MapForest::<u32, f32>::new();
        let mut m = Map::<u32, f32>::new();

        let mm = 4096;
        let mut x = 0;

        for n in 0..mm {
            assert_eq!(m.insert(x, n as f32, f, &()), None);
            m.verify(f, &());

            x = (x + n + 1) % mm;
        }

        x = 0;
        for n in 0..mm {
            assert_eq!(m.get(x, f, &()), Some(n as f32));
            x = (x + n + 1) % mm;
        }

        x = 0;
        for n in 0..mm {
            assert_eq!(m.remove(x, f, &()), Some(n as f32));
            m.verify(f, &());

            x = (x + n + 1) % mm;
        }

        assert!(m.is_empty());
    }
}

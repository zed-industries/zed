//! A path from the root of a B+-tree to a leaf node.

use super::node::Removed;
use super::{Comparator, Forest, MAX_PATH, Node, NodeData, NodePool, slice_insert, slice_shift};
use core::borrow::Borrow;
use core::marker::PhantomData;

#[cfg(test)]
use core::fmt;

pub(super) struct Path<F: Forest> {
    /// Number of path entries including the root and leaf nodes.
    size: usize,

    /// Path of node references from the root to a leaf node.
    node: [Node; MAX_PATH],

    /// Entry number in each node.
    entry: [u8; MAX_PATH],

    unused: PhantomData<F>,
}

impl<F: Forest> Default for Path<F> {
    fn default() -> Self {
        Self {
            size: 0,
            node: [Node(0); MAX_PATH],
            entry: [0; MAX_PATH],
            unused: PhantomData,
        }
    }
}

impl<F: Forest> Path<F> {
    /// Reset path by searching for `key` starting from `root`.
    ///
    /// If `key` is in the tree, returns the corresponding value and leaved the path pointing at
    /// the entry. Otherwise returns `None` and:
    ///
    /// - A key smaller than all stored keys returns a path to the first entry of the first leaf.
    /// - A key larger than all stored keys returns a path to one beyond the last element of the
    ///   last leaf.
    /// - A key between the stored keys of adjacent leaf nodes returns a path to one beyond the
    ///   last entry of the first of the leaf nodes.
    ///
    pub fn find(
        &mut self,
        key: F::Key,
        root: Node,
        pool: &NodePool<F>,
        comp: &dyn Comparator<F::Key>,
    ) -> Option<F::Value> {
        let mut node = root;
        for level in 0.. {
            self.size = level + 1;
            self.node[level] = node;
            match pool[node] {
                NodeData::Inner { size, keys, tree } => {
                    // Invariant: `tree[i]` contains keys smaller than
                    // `keys[i]`, greater or equal to `keys[i-1]`.
                    let i = match comp.search(key, &keys[0..size.into()]) {
                        // We hit an existing key, so follow the >= branch.
                        Ok(i) => i + 1,
                        // Key is less than `keys[i]`, so follow the < branch.
                        Err(i) => i,
                    };
                    self.entry[level] = i as u8;
                    node = tree[i];
                }
                NodeData::Leaf { size, keys, vals } => {
                    // For a leaf we want either the found key or an insert position.
                    return match comp.search(key, &keys.borrow()[0..size.into()]) {
                        Ok(i) => {
                            self.entry[level] = i as u8;
                            Some(vals.borrow()[i])
                        }
                        Err(i) => {
                            self.entry[level] = i as u8;
                            None
                        }
                    };
                }
                NodeData::Free { .. } => panic!("Free {} reached from {}", node, root),
            }
        }
        unreachable!();
    }

    /// Move path to the first entry of the tree starting at `root` and return it.
    pub fn first(&mut self, root: Node, pool: &NodePool<F>) -> (F::Key, F::Value) {
        let mut node = root;
        for level in 0.. {
            self.size = level + 1;
            self.node[level] = node;
            self.entry[level] = 0;
            match pool[node] {
                NodeData::Inner { tree, .. } => node = tree[0],
                NodeData::Leaf { keys, vals, .. } => return (keys.borrow()[0], vals.borrow()[0]),
                NodeData::Free { .. } => panic!("Free {} reached from {}", node, root),
            }
        }
        unreachable!();
    }

    /// Move this path to the next key-value pair and return it.
    pub fn next(&mut self, pool: &NodePool<F>) -> Option<(F::Key, F::Value)> {
        match self.leaf_pos() {
            None => return None,
            Some((node, entry)) => {
                let (keys, vals) = pool[node].unwrap_leaf();
                if entry + 1 < keys.len() {
                    self.entry[self.size - 1] += 1;
                    return Some((keys[entry + 1], vals[entry + 1]));
                }
            }
        }

        // The current leaf node is exhausted. Move to the next one.
        let leaf_level = self.size - 1;
        self.next_node(leaf_level, pool).map(|node| {
            let (keys, vals) = pool[node].unwrap_leaf();
            (keys[0], vals[0])
        })
    }

    /// Move this path to the previous key-value pair and return it.
    ///
    /// If the path is at the off-the-end position, go to the last key-value pair.
    ///
    /// If the path is already at the first key-value pair, leave it there and return `None`.
    pub fn prev(&mut self, root: Node, pool: &NodePool<F>) -> Option<(F::Key, F::Value)> {
        // We use `size == 0` as a generic off-the-end position.
        if self.size == 0 {
            self.goto_subtree_last(0, root, pool);
            let (node, entry) = self.leaf_pos().unwrap();
            let (keys, vals) = pool[node].unwrap_leaf();
            return Some((keys[entry], vals[entry]));
        }

        match self.leaf_pos() {
            None => return None,
            Some((node, entry)) => {
                if entry > 0 {
                    self.entry[self.size - 1] -= 1;
                    let (keys, vals) = pool[node].unwrap_leaf();
                    return Some((keys[entry - 1], vals[entry - 1]));
                }
            }
        }

        // The current leaf node is exhausted. Move to the previous one.
        self.prev_leaf(pool).map(|node| {
            let (keys, vals) = pool[node].unwrap_leaf();
            let e = self.leaf_entry();
            (keys[e], vals[e])
        })
    }

    /// Move path to the first entry of the next node at level, if one exists.
    ///
    /// Returns the new node if it exists.
    ///
    /// Reset the path to `size = 0` and return `None` if there is no next node.
    fn next_node(&mut self, level: usize, pool: &NodePool<F>) -> Option<Node> {
        match self.right_sibling_branch_level(level, pool) {
            None => {
                self.size = 0;
                None
            }
            Some(bl) => {
                let (_, bnodes) = pool[self.node[bl]].unwrap_inner();
                self.entry[bl] += 1;
                let mut node = bnodes[usize::from(self.entry[bl])];

                for l in bl + 1..level {
                    self.node[l] = node;
                    self.entry[l] = 0;
                    node = pool[node].unwrap_inner().1[0];
                }

                self.node[level] = node;
                self.entry[level] = 0;
                Some(node)
            }
        }
    }

    /// Move the path to the last entry of the previous leaf node, if one exists.
    ///
    /// Returns the new leaf node if it exists.
    ///
    /// Leave the path unchanged and returns `None` if we are already at the first leaf node.
    fn prev_leaf(&mut self, pool: &NodePool<F>) -> Option<Node> {
        self.left_sibling_branch_level(self.size - 1).map(|bl| {
            let entry = self.entry[bl] - 1;
            self.entry[bl] = entry;
            let (_, bnodes) = pool[self.node[bl]].unwrap_inner();
            self.goto_subtree_last(bl + 1, bnodes[usize::from(entry)], pool)
        })
    }

    /// Move this path to the last position for the sub-tree at `level, root`.
    fn goto_subtree_last(&mut self, level: usize, root: Node, pool: &NodePool<F>) -> Node {
        let mut node = root;
        for l in level.. {
            self.node[l] = node;
            match pool[node] {
                NodeData::Inner { size, ref tree, .. } => {
                    self.entry[l] = size;
                    node = tree[usize::from(size)];
                }
                NodeData::Leaf { size, .. } => {
                    self.entry[l] = size - 1;
                    self.size = l + 1;
                    break;
                }
                NodeData::Free { .. } => panic!("Free {} reached from {}", node, root),
            }
        }
        node
    }

    /// Set the root node and point the path at the first entry of the node.
    pub fn set_root_node(&mut self, root: Node) {
        self.size = 1;
        self.node[0] = root;
        self.entry[0] = 0;
    }

    /// Get the current leaf node and entry, if any.
    pub fn leaf_pos(&self) -> Option<(Node, usize)> {
        let i = self.size.wrapping_sub(1);
        self.node.get(i).map(|&n| (n, self.entry[i].into()))
    }

    /// Get the current leaf node.
    fn leaf_node(&self) -> Node {
        self.node[self.size - 1]
    }

    /// Get the current entry in the leaf node.
    fn leaf_entry(&self) -> usize {
        self.entry[self.size - 1].into()
    }

    /// Is this path pointing to the first entry in the tree?
    /// This corresponds to the smallest key.
    fn at_first_entry(&self) -> bool {
        self.entry[0..self.size].iter().all(|&i| i == 0)
    }

    /// Get a mutable reference to the current value.
    /// This assumes that there is a current value.
    pub fn value_mut<'a>(&self, pool: &'a mut NodePool<F>) -> &'a mut F::Value {
        &mut pool[self.leaf_node()].unwrap_leaf_mut().1[self.leaf_entry()]
    }

    /// Insert the key-value pair at the current position.
    /// The current position must be the correct insertion location for the key.
    /// This function does not check for duplicate keys. Use `find` or similar for that.
    /// Returns the new root node.
    pub fn insert(&mut self, key: F::Key, value: F::Value, pool: &mut NodePool<F>) -> Node {
        if !self.try_leaf_insert(key, value, pool) {
            self.split_and_insert(key, value, pool);
        }
        self.node[0]
    }

    /// Try to insert `key, value` at the current position, but fail and return false if the leaf
    /// node is full.
    fn try_leaf_insert(&self, key: F::Key, value: F::Value, pool: &mut NodePool<F>) -> bool {
        let index = self.leaf_entry();

        // The case `index == 0` should only ever happen when there are no earlier leaf nodes,
        // otherwise we should have appended to the previous leaf node instead. This invariant
        // means that we don't need to update keys stored in inner nodes here.
        debug_assert!(index > 0 || self.at_first_entry());

        pool[self.leaf_node()].try_leaf_insert(index, key, value)
    }

    /// Split the current leaf node and then insert `key, value`.
    /// This should only be used if `try_leaf_insert()` fails.
    fn split_and_insert(&mut self, mut key: F::Key, value: F::Value, pool: &mut NodePool<F>) {
        let orig_root = self.node[0];

        // Loop invariant: We need to split the node at `level` and then retry a failed insertion.
        // The items to insert are either `(key, ins_node)` or `(key, value)`.
        let mut ins_node = None;
        let mut split;
        for level in (0..self.size).rev() {
            // Split the current node.
            let mut node = self.node[level];
            let mut entry = self.entry[level].into();
            split = pool[node].split(entry);
            let rhs_node = pool.alloc_node(split.rhs_data);

            // Should the path be moved to the new RHS node?
            // Prefer the smaller node if we're right in the middle.
            // Prefer to append to LHS all other things being equal.
            //
            // When inserting into an inner node (`ins_node.is_some()`), we must point to a valid
            // entry in the current node since the new entry is inserted *after* the insert
            // location.
            if entry > split.lhs_entries
                || (entry == split.lhs_entries
                    && (split.lhs_entries > split.rhs_entries || ins_node.is_some()))
            {
                node = rhs_node;
                entry -= split.lhs_entries;
                self.node[level] = node;
                self.entry[level] = entry as u8;
            }

            // Now that we have a not-full node, it must be possible to insert.
            match ins_node {
                None => {
                    let inserted = pool[node].try_leaf_insert(entry, key, value);
                    debug_assert!(inserted);
                    // If we inserted at the front of the new rhs_node leaf, we need to propagate
                    // the inserted key as the critical key instead of the previous front key.
                    if entry == 0 && node == rhs_node {
                        split.crit_key = key;
                    }
                }
                Some(n) => {
                    let inserted = pool[node].try_inner_insert(entry, key, n);
                    debug_assert!(inserted);
                    // The lower level was moved to the new RHS node, so make sure that is
                    // reflected here.
                    if n == self.node[level + 1] {
                        self.entry[level] += 1;
                    }
                }
            }

            // We are now done with the current level, but `rhs_node` must be inserted in the inner
            // node above us. If we're already at level 0, the root node needs to be split.
            key = split.crit_key;
            ins_node = Some(rhs_node);
            if level > 0 {
                let pnode = &mut pool[self.node[level - 1]];
                let pentry = self.entry[level - 1].into();
                if pnode.try_inner_insert(pentry, key, rhs_node) {
                    // If this level level was moved to the new RHS node, update parent entry.
                    if node == rhs_node {
                        self.entry[level - 1] += 1;
                    }
                    return;
                }
            }
        }

        // If we get here we have split the original root node and need to add an extra level.
        let rhs_node = ins_node.expect("empty path");
        let root = pool.alloc_node(NodeData::inner(orig_root, key, rhs_node));
        let entry = if self.node[0] == rhs_node { 1 } else { 0 };
        self.size += 1;
        slice_insert(&mut self.node[0..self.size], 0, root);
        slice_insert(&mut self.entry[0..self.size], 0, entry);
    }

    /// Remove the key-value pair at the current position and advance the path to the next
    /// key-value pair, leaving the path in a normalized state.
    ///
    /// Return the new root node.
    pub fn remove(&mut self, pool: &mut NodePool<F>) -> Option<Node> {
        let e = self.leaf_entry();
        match pool[self.leaf_node()].leaf_remove(e) {
            Removed::Healthy => {
                if e == 0 {
                    self.update_crit_key(pool)
                }
                Some(self.node[0])
            }
            status => self.balance_nodes(status, pool),
        }
    }

    /// Get the critical key for the current node at `level`.
    ///
    /// The critical key is less than or equal to all keys in the sub-tree at `level` and greater
    /// than all keys to the left of the current node at `level`.
    ///
    /// The left-most node at any level does not have a critical key.
    fn current_crit_key(&self, level: usize, pool: &NodePool<F>) -> Option<F::Key> {
        // Find the level containing the critical key for the current node.
        self.left_sibling_branch_level(level).map(|bl| {
            let (keys, _) = pool[self.node[bl]].unwrap_inner();
            keys[usize::from(self.entry[bl]) - 1]
        })
    }

    /// Update the critical key after removing the front entry of the leaf node.
    fn update_crit_key(&mut self, pool: &mut NodePool<F>) {
        // Find the inner level containing the critical key for the current leaf node.
        let crit_level = match self.left_sibling_branch_level(self.size - 1) {
            None => return,
            Some(l) => l,
        };
        let crit_kidx = self.entry[crit_level] - 1;

        // Extract the new critical key from the leaf node.
        let crit_key = pool[self.leaf_node()].leaf_crit_key();
        let crit_node = self.node[crit_level];

        match pool[crit_node] {
            NodeData::Inner {
                size, ref mut keys, ..
            } => {
                debug_assert!(crit_kidx < size);
                keys[usize::from(crit_kidx)] = crit_key;
            }
            _ => panic!("Expected inner node"),
        }
    }

    /// Given that the current leaf node is in an unhealthy (underflowed or even empty) status,
    /// balance it with sibling nodes.
    ///
    /// Return the new root node.
    fn balance_nodes(&mut self, status: Removed, pool: &mut NodePool<F>) -> Option<Node> {
        // The current leaf node is not in a healthy state, and its critical key may have changed
        // too.
        //
        // Start by dealing with a changed critical key for the leaf level.
        if status != Removed::Empty && self.leaf_entry() == 0 {
            self.update_crit_key(pool);
        }

        let leaf_level = self.size - 1;
        if self.heal_level(status, leaf_level, pool) {
            // Tree has become empty.
            self.size = 0;
            return None;
        }

        // Discard the root node if it has shrunk to a single sub-tree.
        let mut ns = 0;
        while let NodeData::Inner {
            size: 0, ref tree, ..
        } = pool[self.node[ns]]
        {
            ns += 1;
            self.node[ns] = tree[0];
        }

        if ns > 0 {
            for l in 0..ns {
                pool.free_node(self.node[l]);
            }

            // Shift the whole array instead of just 0..size because `self.size` may be cleared
            // here if the path is pointing off-the-end.
            slice_shift(&mut self.node, ns);
            slice_shift(&mut self.entry, ns);

            if self.size > 0 {
                self.size -= ns;
            }
        }

        // Return the root node, even when `size=0` indicating that we're at the off-the-end
        // position.
        Some(self.node[0])
    }

    /// After removing an entry from the node at `level`, check its health and rebalance as needed.
    ///
    /// Leave the path up to and including `level` in a normalized state where all entries are in
    /// bounds.
    ///
    /// Returns true if the tree becomes empty.
    fn heal_level(&mut self, status: Removed, level: usize, pool: &mut NodePool<F>) -> bool {
        match status {
            Removed::Healthy => {}
            Removed::Rightmost => {
                // The rightmost entry was removed from the current node, so move the path so it
                // points at the first entry of the next node at this level.
                debug_assert_eq!(
                    usize::from(self.entry[level]),
                    pool[self.node[level]].entries()
                );
                self.next_node(level, pool);
            }
            Removed::Underflow => self.underflowed_node(level, pool),
            Removed::Empty => return self.empty_node(level, pool),
        }
        false
    }

    /// The current node at `level` has underflowed, meaning that it is below half capacity but
    /// not completely empty.
    ///
    /// Handle this by balancing entries with the right sibling node.
    ///
    /// Leave the path up to and including `level` in a valid state that points to the same entry.
    fn underflowed_node(&mut self, level: usize, pool: &mut NodePool<F>) {
        // Look for a right sibling node at this level. If none exists, we allow the underflowed
        // node to persist as the right-most node at its level.
        if let Some((crit_key, rhs_node)) = self.right_sibling(level, pool) {
            // New critical key for the updated right sibling node.
            let new_ck: Option<F::Key>;
            let empty;
            // Make a COPY of the sibling node to avoid fighting the borrow checker.
            let mut rhs = pool[rhs_node];
            match pool[self.node[level]].balance(crit_key, &mut rhs) {
                None => {
                    // Everything got moved to the RHS node.
                    new_ck = self.current_crit_key(level, pool);
                    empty = true;
                }
                Some(key) => {
                    // Entries moved from RHS node.
                    new_ck = Some(key);
                    empty = false;
                }
            }
            // Put back the updated RHS node data.
            pool[rhs_node] = rhs;
            // Update the critical key for the RHS node unless it has become a left-most
            // node.
            if let Some(ck) = new_ck {
                self.update_right_crit_key(level, ck, pool);
            }
            if empty {
                let empty_tree = self.empty_node(level, pool);
                debug_assert!(!empty_tree);
            }

            // Any Removed::Rightmost state must have been cleared above by merging nodes. If the
            // current entry[level] was one off the end of the node, it will now point at a proper
            // entry.
            debug_assert!(usize::from(self.entry[level]) < pool[self.node[level]].entries());
        } else if usize::from(self.entry[level]) >= pool[self.node[level]].entries() {
            // There's no right sibling at this level, so the node can't be rebalanced.
            // Check if we are in an off-the-end position.
            self.size = 0;
        }
    }

    /// The current node at `level` has become empty.
    ///
    /// Remove the node from its parent node and leave the path in a normalized state. This means
    /// that the path at this level will go through the right sibling of this node.
    ///
    /// If the current node has no right sibling, set `self.size = 0`.
    ///
    /// Returns true if the tree becomes empty.
    fn empty_node(&mut self, level: usize, pool: &mut NodePool<F>) -> bool {
        pool.free_node(self.node[level]);
        if level == 0 {
            // We just deleted the root node, so the tree is now empty.
            return true;
        }

        // Get the right sibling node before recursively removing nodes.
        let rhs_node = self.right_sibling(level, pool).map(|(_, n)| n);

        // Remove the current sub-tree from the parent node.
        let pl = level - 1;
        let pe = self.entry[pl].into();
        let status = pool[self.node[pl]].inner_remove(pe);
        self.heal_level(status, pl, pool);

        // Finally update the path at this level.
        match rhs_node {
            // We'll leave `self.entry[level]` unchanged. It can be non-zero after moving node
            // entries to the right sibling node.
            Some(rhs) => self.node[level] = rhs,
            // We have no right sibling, so we must have deleted the right-most
            // entry. The path should be moved to the "off-the-end" position.
            None => self.size = 0,
        }
        false
    }

    /// Find the level where the right sibling to the current node at `level` branches off.
    ///
    /// This will be an inner node with two adjacent sub-trees: In one the current node at level is
    /// a right-most node, in the other, the right sibling is a left-most node.
    ///
    /// Returns `None` if the current node is a right-most node so no right sibling exists.
    fn right_sibling_branch_level(&self, level: usize, pool: &NodePool<F>) -> Option<usize> {
        (0..level).rposition(|l| match pool[self.node[l]] {
            NodeData::Inner { size, .. } => self.entry[l] < size,
            _ => panic!("Expected inner node"),
        })
    }

    /// Find the level where the left sibling to the current node at `level` branches off.
    fn left_sibling_branch_level(&self, level: usize) -> Option<usize> {
        self.entry[0..level].iter().rposition(|&e| e != 0)
    }

    /// Get the right sibling node to the current node at `level`.
    /// Also return the critical key between the current node and the right sibling.
    fn right_sibling(&self, level: usize, pool: &NodePool<F>) -> Option<(F::Key, Node)> {
        // Find the critical level: The deepest level where two sibling subtrees contain the
        // current node and its right sibling.
        self.right_sibling_branch_level(level, pool).map(|bl| {
            // Extract the critical key and the `bl+1` node.
            let be = usize::from(self.entry[bl]);
            let crit_key;
            let mut node;
            {
                let (keys, tree) = pool[self.node[bl]].unwrap_inner();
                crit_key = keys[be];
                node = tree[be + 1];
            }

            // Follow left-most links back down to `level`.
            for _ in bl + 1..level {
                node = pool[node].unwrap_inner().1[0];
            }

            (crit_key, node)
        })
    }

    /// Update the critical key for the right sibling node at `level`.
    fn update_right_crit_key(&self, level: usize, crit_key: F::Key, pool: &mut NodePool<F>) {
        let bl = self
            .right_sibling_branch_level(level, pool)
            .expect("No right sibling exists");
        match pool[self.node[bl]] {
            NodeData::Inner { ref mut keys, .. } => {
                keys[usize::from(self.entry[bl])] = crit_key;
            }
            _ => panic!("Expected inner node"),
        }
    }

    /// Normalize the path position such that it is either pointing at a real entry or `size=0`
    /// indicating "off-the-end".
    pub fn normalize(&mut self, pool: &mut NodePool<F>) {
        if let Some((leaf, entry)) = self.leaf_pos() {
            if entry >= pool[leaf].entries() {
                let leaf_level = self.size - 1;
                self.next_node(leaf_level, pool);
            }
        }
    }
}

#[cfg(test)]
impl<F: Forest> Path<F> {
    /// Check the internal consistency of this path.
    pub fn verify(&self, pool: &NodePool<F>) {
        for level in 0..self.size {
            match pool[self.node[level]] {
                NodeData::Inner { size, tree, .. } => {
                    assert!(level < self.size - 1, "Expected leaf node at level {level}");
                    assert!(
                        self.entry[level] <= size,
                        "OOB inner entry {}/{} at level {}",
                        self.entry[level],
                        size,
                        level
                    );
                    assert_eq!(
                        self.node[level + 1],
                        tree[usize::from(self.entry[level])],
                        "Node mismatch at level {level}"
                    );
                }
                NodeData::Leaf { size, .. } => {
                    assert_eq!(level, self.size - 1, "Expected inner node");
                    assert!(
                        self.entry[level] <= size,
                        "OOB leaf entry {}/{}",
                        self.entry[level],
                        size,
                    );
                }
                NodeData::Free { .. } => {
                    panic!("Free {} in path", self.node[level]);
                }
            }
        }
    }
}

#[cfg(test)]
impl<F: Forest> fmt::Display for Path<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.size == 0 {
            write!(f, "<empty path>")
        } else {
            write!(f, "{}[{}]", self.node[0], self.entry[0])?;
            for i in 1..self.size {
                write!(f, "--{}[{}]", self.node[i], self.entry[i])?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cmp::Ordering;

    struct TC();

    impl Comparator<i32> for TC {
        fn cmp(&self, a: i32, b: i32) -> Ordering {
            a.cmp(&b)
        }
    }

    struct TF();

    impl Forest for TF {
        type Key = i32;
        type Value = char;
        type LeafKeys = [i32; 7];
        type LeafValues = [char; 7];

        fn splat_key(key: Self::Key) -> Self::LeafKeys {
            [key; 7]
        }

        fn splat_value(value: Self::Value) -> Self::LeafValues {
            [value; 7]
        }
    }

    #[test]
    fn search_single_leaf() {
        // Testing Path::new() for trees with a single leaf node.
        let mut pool = NodePool::<TF>::new();
        let root = pool.alloc_node(NodeData::leaf(10, 'a'));
        let mut p = Path::default();
        let comp = TC();

        // Search for key less than stored key.
        assert_eq!(p.find(5, root, &pool, &comp), None);
        assert_eq!(p.size, 1);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 0);

        // Search for stored key.
        assert_eq!(p.find(10, root, &pool, &comp), Some('a'));
        assert_eq!(p.size, 1);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 0);

        // Search for key greater than stored key.
        assert_eq!(p.find(15, root, &pool, &comp), None);
        assert_eq!(p.size, 1);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 1);

        // Modify leaf node to contain two values.
        match pool[root] {
            NodeData::Leaf {
                ref mut size,
                ref mut keys,
                ref mut vals,
            } => {
                *size = 2;
                keys[1] = 20;
                vals[1] = 'b';
            }
            _ => unreachable!(),
        }

        // Search for key between stored keys.
        assert_eq!(p.find(15, root, &pool, &comp), None);
        assert_eq!(p.size, 1);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 1);

        // Search for key greater than stored keys.
        assert_eq!(p.find(25, root, &pool, &comp), None);
        assert_eq!(p.size, 1);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 2);
    }

    #[test]
    fn search_single_inner() {
        // Testing Path::new() for trees with a single inner node and two leaves.
        let mut pool = NodePool::<TF>::new();
        let leaf1 = pool.alloc_node(NodeData::leaf(10, 'a'));
        let leaf2 = pool.alloc_node(NodeData::leaf(20, 'b'));
        let root = pool.alloc_node(NodeData::inner(leaf1, 20, leaf2));
        let mut p = Path::default();
        let comp = TC();

        // Search for key less than stored keys.
        assert_eq!(p.find(5, root, &pool, &comp), None);
        assert_eq!(p.size, 2);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 0);
        assert_eq!(p.node[1], leaf1);
        assert_eq!(p.entry[1], 0);

        assert_eq!(p.find(10, root, &pool, &comp), Some('a'));
        assert_eq!(p.size, 2);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 0);
        assert_eq!(p.node[1], leaf1);
        assert_eq!(p.entry[1], 0);

        // Midway between the two leaf nodes.
        assert_eq!(p.find(15, root, &pool, &comp), None);
        assert_eq!(p.size, 2);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 0);
        assert_eq!(p.node[1], leaf1);
        assert_eq!(p.entry[1], 1);

        assert_eq!(p.find(20, root, &pool, &comp), Some('b'));
        assert_eq!(p.size, 2);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 1);
        assert_eq!(p.node[1], leaf2);
        assert_eq!(p.entry[1], 0);

        assert_eq!(p.find(25, root, &pool, &comp), None);
        assert_eq!(p.size, 2);
        assert_eq!(p.node[0], root);
        assert_eq!(p.entry[0], 1);
        assert_eq!(p.node[1], leaf2);
        assert_eq!(p.entry[1], 1);
    }
}

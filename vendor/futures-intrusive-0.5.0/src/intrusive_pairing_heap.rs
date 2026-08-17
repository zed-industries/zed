//! Implements an intrusive priority queue based on a pairing heap.
//!
//! A [pairing heap] is a heap data structure (i.e. a tree whose nodes carry
//! values, with the property that every node's value is lesser or equal to its
//! children's) that supports the following operations:
//!
//! - finding a minimum element in `O(1)`
//!    - This is trivial: the heap property guarantees that the root is a
//!      minimum element.
//! - insertion of a new node in `O(1)`
//! - deletion in `O(log n)`, _amortized_
//!    - However, note that any _individual_ deletion may take `O(N)` time. For
//!      example, if we take an empty heap and insert N elements, the tree will
//!      have a very degenerate (shallow) shape. Then, deleting the root will
//!      take `O(N)` time, but it will also reorganize the tree to make
//!      successive deletes cheaper.
//!
//! [pairing heap]: https://en.wikipedia.org/wiki/Pairing_heap

use core::{
    marker::PhantomPinned,
    mem,
    ops::{Deref, DerefMut, Drop},
    ptr::NonNull,
};

/// Compares `a` and `b` without unwinding.
/// This is necessary to avoid reentrancy in the heap.
fn safe_lesser<T: Ord>(a: &T, b: &T) -> bool {
    struct DropBomb;
    impl Drop for DropBomb {
        fn drop(&mut self) {
            panic!("Panicked while comparing");
        }
    }
    // If `T::cmp` panics, force a double-panic (and therefore an abort).
    let bomb = DropBomb;
    let ordering = a < b;
    mem::forget(bomb);
    ordering
}

/// A node which carries data of type `T` and is stored in an intrusive heap.
///
/// Nodes will be compared based on `T`'s [`Ord`] impl. Those comparisons must
/// not panic - otherwise, the program will abort.
#[derive(Debug)]
pub struct HeapNode<T> {
    /// The parent. `None` if this is the root.
    parent: Option<NonNull<HeapNode<T>>>,
    /// The previous sibling. `None` if there is no previous sibling.
    prev: Option<NonNull<HeapNode<T>>>,
    /// The next sibling. `None` if there is no next sibling.
    next: Option<NonNull<HeapNode<T>>>,
    /// The first child. `None` if there are no children.
    first_child: Option<NonNull<HeapNode<T>>>,
    /// The data which is associated to this heap item.
    data: T,
    /// Prevents `HeapNode`s from being `Unpin`. They may never be moved, since
    /// the heap semantics require addresses to be stable.
    _pin: PhantomPinned,
}

impl<T> HeapNode<T> {
    /// Creates a new node with the associated data
    pub fn new(data: T) -> HeapNode<T> {
        HeapNode::<T> {
            parent: None,
            prev: None,
            next: None,
            first_child: None,
            data,
            _pin: PhantomPinned,
        }
    }

    fn is_root(&self) -> bool {
        if self.parent.is_none() {
            debug_assert_eq!(self.prev, None);
            debug_assert_eq!(self.next, None);
            true
        } else {
            false
        }
    }
}

impl<T> Deref for HeapNode<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for HeapNode<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

/// Add a child to a node.
unsafe fn add_child<T: Ord>(
    mut parent: NonNull<HeapNode<T>>,
    mut child: NonNull<HeapNode<T>>,
) {
    // require parent <= child
    debug_assert!(!safe_lesser(&child.as_ref().data, &parent.as_ref().data));
    if let Some(mut old_first_child) = parent.as_mut().first_child.take() {
        child.as_mut().next = Some(old_first_child);
        debug_assert_eq!(old_first_child.as_ref().prev, None);
        old_first_child.as_mut().prev = Some(child);
    }
    parent.as_mut().first_child = Some(child);
    child.as_mut().parent = Some(parent);
}

/// Merge two root heaps. Returns the new root.
unsafe fn meld<T: Ord>(
    left: NonNull<HeapNode<T>>,
    right: NonNull<HeapNode<T>>,
) -> NonNull<HeapNode<T>> {
    debug_assert!(left.as_ref().is_root());
    debug_assert!(right.as_ref().is_root());
    // The lesser node should become the root.
    if safe_lesser(&left.as_ref().data, &right.as_ref().data) {
        add_child(left, right);
        left
    } else {
        add_child(right, left);
        right
    }
}

/// Merge two root heaps, where the left might be empty. Returns the new root.
unsafe fn maybe_meld<T: Ord>(
    left: Option<NonNull<HeapNode<T>>>,
    right: NonNull<HeapNode<T>>,
) -> NonNull<HeapNode<T>> {
    if let Some(left) = left {
        meld(left, right)
    } else {
        right
    }
}

/// Given the first child in a child list, traverse and find the last child.
unsafe fn last_child<T>(
    first_child: NonNull<HeapNode<T>>,
) -> NonNull<HeapNode<T>> {
    let mut cur = first_child;
    while let Some(next) = cur.as_ref().next {
        cur = next;
    }
    cur
}

/// Given a pointer to the last node in a child list, unlink it and return the
/// previous node (which has become the last node in its list).
///
/// That is, given a list `A <-> B <-> C`, `unlink_prev(C)` will return `B` and
/// also unlink `C` to become `A <-> B    C`.
///
/// If the node was a lone child, returns `None`.
///
/// Parent/child pointers are untouched.
unsafe fn unlink_prev<T>(
    mut node: NonNull<HeapNode<T>>,
) -> Option<NonNull<HeapNode<T>>> {
    debug_assert_eq!(node.as_ref().next, None);
    let mut prev = node.as_mut().prev.take()?;
    debug_assert_eq!(prev.as_ref().next, Some(node));
    prev.as_mut().next = None;
    Some(prev)
}

/// Merge together a child list. Each child in the child list is expected to
/// have an equal `parent`. Returns the new merged root, whose `parent` will be unset.
unsafe fn merge_children<T: Ord>(
    first_child: NonNull<HeapNode<T>>,
) -> NonNull<HeapNode<T>> {
    let common_parent = first_child.as_ref().parent;
    debug_assert!(common_parent.is_some());

    // Traverse the children right-to-left. This is important for the analysis
    // to work. Reading: "Pairing heaps: the forward variant",
    // https://arxiv.org/pdf/1709.01152.pdf
    let mut node = last_child(first_child);
    let mut current = None;
    // Loop invariant: `node` is the first unprocessed child, `current`
    // is the merged result of all processed children.
    loop {
        // All nodes in the list should have the same parent.
        let node_parent = node.as_mut().parent.take();
        debug_assert_eq!(node_parent, common_parent);

        // Grab the last two unprocessed elements.
        let mut prev = if let Some(prev) = unlink_prev(node) {
            prev
        } else {
            // Odd case.
            return maybe_meld(current, node);
        };

        // All nodes in the list should have the same parent.
        let prev_parent = prev.as_mut().parent.take();
        debug_assert_eq!(prev_parent, common_parent);

        // Unlink `prev` from `prev.prev`.
        let prev_prev = unlink_prev(prev);

        // Meld the pair, then meld it into the accumulator.
        let cur = maybe_meld(current, meld(prev, node));

        if let Some(prev_prev) = prev_prev {
            node = prev_prev;
            current = Some(cur);
            continue;
        } else {
            // Even case.
            return cur;
        }
    }
}

/// An intrusive min-heap of nodes, where each node carries associated data
/// of type `T`.
#[derive(Debug)]
pub struct PairingHeap<T> {
    root: Option<NonNull<HeapNode<T>>>,
}

impl<T: Ord> PairingHeap<T> {
    /// Creates an empty heap
    pub fn new() -> Self {
        PairingHeap::<T> { root: None }
    }

    /// Adds a node to the heap.
    /// Safety: This function is only safe as long as `node` is guaranteed to
    /// get removed from the list before it gets moved or dropped.
    /// In addition to this `node` may not be added to another other heap before
    /// it is removed from the current one.
    pub unsafe fn insert(&mut self, node: &mut HeapNode<T>) {
        // The node should not already be in a heap.
        debug_assert!(node.is_root());
        debug_assert_eq!(node.first_child, None);

        if let Some(root) = self.root {
            self.root = Some(meld(root, node.into()));
        } else {
            self.root = Some(node.into());
        }
    }

    /// Returns the smallest element in the heap without removing it.
    /// The function is only safe as long as valid pointers are stored inside
    /// the heap.
    /// The returned pointer is only guaranteed to be valid as long as the heap
    /// is not mutated
    pub fn peek_min(&self) -> Option<NonNull<HeapNode<T>>> {
        self.root
    }

    /// Removes the given node from the heap.
    /// The node must be a member of this heap, and not a member of any other
    /// heap.
    pub unsafe fn remove(&mut self, node: &mut HeapNode<T>) {
        let parent = node.parent.take();
        if let Some(mut parent) = parent {
            // Unlink this node from its parent.
            if let Some(mut prev) = node.prev {
                prev.as_mut().next = node.next;
            } else {
                parent.as_mut().first_child = node.next;
            }
            if let Some(mut next) = node.next {
                next.as_mut().prev = node.prev;
            }
            node.next = None;
            node.prev = None;
        } else {
            debug_assert_eq!(node.next, None);
            debug_assert_eq!(node.prev, None);
            debug_assert_eq!(self.root, Some(node.into()));
            self.root = None;
        }
        if let Some(first_child) = node.first_child.take() {
            // Merge together the children.
            let children = merge_children(first_child);
            // Add the children back into the parent.
            if let Some(parent) = parent {
                // The heap property is preserved because we had `parent.data`
                // <= `node.data`, and `node.data` <= `child.data` for all
                // children.
                add_child(parent, children);
            } else {
                self.root = Some(children);
            }
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::{HeapNode, PairingHeap};
    use core::ptr::NonNull;

    // Recursively check the provided node and all descendants for:
    // - pointer consistency: parent pointers and next/prev
    // - the heap property: `node.data <= child.data` for all children
    unsafe fn validate_heap_node<T: Ord>(
        node: &HeapNode<T>,
        parent: Option<&HeapNode<T>>,
    ) {
        assert_eq!(node.parent, parent.map(NonNull::from));
        if let Some(p) = parent {
            assert!(p.data <= node.data);
        }
        if let Some(prev) = node.prev {
            assert_eq!(prev.as_ref().next, Some(node.into()));
        }
        if let Some(next) = node.next {
            assert_eq!(next.as_ref().prev, Some(node.into()));
        }
        let mut child = node.first_child;
        while let Some(c) = child {
            validate_heap_node(c.as_ref(), Some(node));
            child = c.as_ref().next;
        }
    }

    fn validate_heap<T: Ord>(heap: &PairingHeap<T>) {
        if let Some(root) = heap.root {
            // This is also sufficient to check that `heap.root` is indeed a
            // minimum element of the heap.
            unsafe {
                validate_heap_node(root.as_ref(), None);
            }
        }
    }

    #[test]
    fn insert_and_remove() {
        // This test exhaustively covers every possible schedule of inserting,
        // then removing, each of five different nodes from the heap.
        #[derive(Copy, Clone, Debug)]
        enum Action {
            Insert(u8),
            Remove(u8),
        }

        fn generate_schedules(
            current: &mut Vec<Action>,
            available: &mut Vec<Action>,
            f: fn(&[Action]),
        ) {
            for i in 0..available.len() {
                let action = available.swap_remove(i);
                current.push(action);
                f(current);
                if let Action::Insert(j) = action {
                    available.push(Action::Remove(j));
                }
                generate_schedules(current, available, f);
                if let Action::Insert(_) = action {
                    available.pop();
                }
                current.pop();
                // the opposite of `swap_remove`
                available.push(action);
                let len = available.len();
                available.swap(i, len - 1);
            }
        }
        let max = if cfg!(miri) {
            // Miri is really slow, make things easier.
            3
        } else {
            // 5 runs in a reasonable amount of time but still exercises
            // interesting cases.
            5
        };
        generate_schedules(
            &mut vec![],
            &mut (0..max).map(Action::Insert).collect(),
            |schedule| unsafe {
                let mut nodes = [
                    HeapNode::new(0u8),
                    HeapNode::new(1),
                    HeapNode::new(2),
                    HeapNode::new(3),
                    HeapNode::new(4),
                ];
                let mut heap = PairingHeap::new();
                for action in schedule {
                    match *action {
                        Action::Insert(n) => {
                            heap.insert(&mut nodes[n as usize]);
                            validate_heap(&heap);
                        }
                        Action::Remove(n) => {
                            heap.remove(&mut nodes[n as usize]);
                            assert!(nodes[n as usize].is_root());
                            assert_eq!(nodes[n as usize].first_child, None);
                            validate_heap(&heap);
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn equal_values() {
        // Check that things behave properly in the presence of equal values.
        unsafe {
            let mut nodes = [
                HeapNode::new(0u8),
                HeapNode::new(0),
                HeapNode::new(0),
                HeapNode::new(0),
                HeapNode::new(0),
            ];
            let mut heap = PairingHeap::new();
            for node in &mut nodes {
                heap.insert(node);
                validate_heap(&heap);
            }
            for _ in 0..5 {
                heap.remove(heap.peek_min().unwrap().as_mut());
                validate_heap(&heap);
            }
            assert_eq!(heap.peek_min(), None);
        }
    }
}

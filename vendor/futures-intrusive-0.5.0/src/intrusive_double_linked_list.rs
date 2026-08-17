//! An intrusive double linked list of data

use core::{
    marker::PhantomPinned,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// A node which carries data of type `T` and is stored in an intrusive list
#[derive(Debug)]
pub struct ListNode<T> {
    /// The previous node in the list. `None` if there is no previous node.
    prev: Option<NonNull<ListNode<T>>>,
    /// The next node in the list. `None` if there is no previous node.
    next: Option<NonNull<ListNode<T>>>,
    /// The data which is associated to this list item
    data: T,
    /// Prevents `ListNode`s from being `Unpin`. They may never be moved, since
    /// the list semantics require addresses to be stable.
    _pin: PhantomPinned,
}

impl<T> ListNode<T> {
    /// Creates a new node with the associated data
    pub fn new(data: T) -> ListNode<T> {
        ListNode::<T> {
            prev: None,
            next: None,
            data,
            _pin: PhantomPinned,
        }
    }
}

impl<T> Deref for ListNode<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for ListNode<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

/// An intrusive linked list of nodes, where each node carries associated data
/// of type `T`.
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<ListNode<T>>>,
    tail: Option<NonNull<ListNode<T>>>,
}

impl<T> LinkedList<T> {
    /// Creates an empty linked list
    pub fn new() -> Self {
        LinkedList::<T> {
            head: None,
            tail: None,
        }
    }

    /// Adds a node at the front of the linked list.
    /// Safety: This function is only safe as long as `node` is guaranteed to
    /// get removed from the list before it gets moved or dropped.
    /// In addition to this `node` may not be added to another other list before
    /// it is removed from the current one.
    pub unsafe fn add_front(&mut self, node: &mut ListNode<T>) {
        node.next = self.head;
        node.prev = None;
        match self.head {
            Some(mut head) => head.as_mut().prev = Some(node.into()),
            None => {}
        };
        self.head = Some(node.into());
        if self.tail.is_none() {
            self.tail = Some(node.into());
        }
    }

    /// Returns a reference to the first node in the linked list
    /// The function is only safe as long as valid pointers are stored inside
    /// the linked list.
    /// The returned pointer is only guaranteed to be valid as long as the list
    /// is not mutated
    pub fn peek_first(&self) -> Option<&ListNode<T>> {
        // Safety: When the node was inserted it was promised that it is alive
        // until it gets removed from the list.
        // The returned node has a pointer which constrains it to the lifetime
        // of the list. This is ok, since the Node is supposed to outlive
        // its insertion in the list.
        unsafe {
            self.head
                .map(|node| &*(node.as_ptr() as *const ListNode<T>))
        }
    }

    /// Returns a mutable reference to the first node in the linked list
    /// The function is only safe as long as valid pointers are stored inside
    /// the linked list.
    /// The returned pointer is only guaranteed to be valid as long as the list
    /// is not mutated
    pub fn peek_first_mut(&mut self) -> Option<&mut ListNode<T>> {
        // Safety: When the node was inserted it was promised that it is alive
        // until it gets removed from the list.
        // The returned node has a pointer which constrains it to the lifetime
        // of the list. This is ok, since the Node is supposed to outlive
        // its insertion in the list.
        unsafe {
            self.head
                .map(|mut node| &mut *(node.as_mut() as *mut ListNode<T>))
        }
    }

    /// Returns a reference to the last node in the linked list
    /// The function is only safe as long as valid pointers are stored inside
    /// the linked list.
    /// The returned pointer is only guaranteed to be valid as long as the list
    /// is not mutated
    pub fn peek_last(&self) -> Option<&ListNode<T>> {
        // Safety: When the node was inserted it was promised that it is alive
        // until it gets removed from the list.
        // The returned node has a pointer which constrains it to the lifetime
        // of the list. This is ok, since the Node is supposed to outlive
        // its insertion in the list.
        unsafe {
            self.tail
                .map(|node| &*(node.as_ptr() as *const ListNode<T>))
        }
    }

    /// Returns a mutable reference to the last node in the linked list
    /// The function is only safe as long as valid pointers are stored inside
    /// the linked list.
    /// The returned pointer is only guaranteed to be valid as long as the list
    /// is not mutated
    pub fn peek_last_mut(&mut self) -> Option<&mut ListNode<T>> {
        // Safety: When the node was inserted it was promised that it is alive
        // until it gets removed from the list.
        // The returned node has a pointer which constrains it to the lifetime
        // of the list. This is ok, since the Node is supposed to outlive
        // its insertion in the list.
        unsafe {
            self.tail
                .map(|mut node| &mut *(node.as_mut() as *mut ListNode<T>))
        }
    }

    /// Removes the first node from the linked list
    pub fn remove_first(&mut self) -> Option<&mut ListNode<T>> {
        // Safety: When the node was inserted it was promised that it is alive
        // until it gets removed from the list
        unsafe {
            let mut head = self.head?;
            self.head = head.as_mut().next;

            let first_ref = head.as_mut();
            match first_ref.next {
                None => {
                    // This was the only node in the list
                    debug_assert_eq!(Some(first_ref.into()), self.tail);
                    self.tail = None;
                }
                Some(mut next) => {
                    next.as_mut().prev = None;
                }
            }

            first_ref.prev = None;
            first_ref.next = None;
            Some(&mut *(first_ref as *mut ListNode<T>))
        }
    }

    /// Removes the last node from the linked list and returns it
    pub fn remove_last(&mut self) -> Option<&mut ListNode<T>> {
        // Safety: When the node was inserted it was promised that it is alive
        // until it gets removed from the list
        unsafe {
            let mut tail = self.tail?;
            self.tail = tail.as_mut().prev;

            let last_ref = tail.as_mut();
            match last_ref.prev {
                None => {
                    // This was the last node in the list
                    debug_assert_eq!(Some(last_ref.into()), self.head);
                    self.head = None;
                }
                Some(mut prev) => {
                    prev.as_mut().next = None;
                }
            }

            last_ref.prev = None;
            last_ref.next = None;
            Some(&mut *(last_ref as *mut ListNode<T>))
        }
    }

    /// Returns whether the linked list doesn not contain any node
    pub fn is_empty(&self) -> bool {
        if !self.head.is_none() {
            return false;
        }

        debug_assert!(self.tail.is_none());
        true
    }

    /// Removes the given `node` from the linked list.
    /// Returns whether the `node` was removed.
    /// It is also only save if it is known that the `node` is either part of this
    /// list, or of no list at all. If `node` is part of another list, the
    /// behavior is undefined.
    pub unsafe fn remove(&mut self, node: &mut ListNode<T>) -> bool {
        match node.prev {
            None => {
                // This might be the first node in the list. If it is not, the
                // node is not in the list at all. Since our precondition is that
                // the node must either be in this list or in no list, we check that
                // the node is really in no list.
                if self.head != Some(node.into()) {
                    debug_assert!(node.next.is_none());
                    return false;
                }
                self.head = node.next;
            }
            Some(mut prev) => {
                debug_assert_eq!(prev.as_ref().next, Some(node.into()));
                prev.as_mut().next = node.next;
            }
        }

        match node.next {
            None => {
                // This must be the last node in our list. Otherwise the list
                // is inconsistent.
                debug_assert_eq!(self.tail, Some(node.into()));
                self.tail = node.prev;
            }
            Some(mut next) => {
                debug_assert_eq!(next.as_mut().prev, Some(node.into()));
                next.as_mut().prev = node.prev;
            }
        }

        node.next = None;
        node.prev = None;

        true
    }

    /// Drains the list iby calling a callback on each list node
    ///
    /// The method does not return an iterator since stopping or deferring
    /// draining the list is not permitted. If the method would push nodes to
    /// an iterator we could not guarantee that the nodes do not get utilized
    /// after having been removed from the list anymore.
    pub fn drain<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut ListNode<T>),
    {
        let mut current = self.head;
        self.head = None;
        self.tail = None;

        while let Some(mut node) = current {
            // Safety: The nodes have not been removed from the list yet and must
            // therefore contain valid data. The nodes can also not be added to
            // the list again during iteration, since the list is mutably borrowed.
            unsafe {
                let node_ref = node.as_mut();
                current = node_ref.next;

                node_ref.next = None;
                node_ref.prev = None;

                // Note: We do not reset the pointers from the next element in the
                // list to the current one since we will iterate over the whole
                // list anyway, and therefore clean up all pointers.

                func(node_ref);
            }
        }
    }

    /// Drains the list in reverse order by calling a callback on each list node
    ///
    /// The method does not return an iterator since stopping or deferring
    /// draining the list is not permitted. If the method would push nodes to
    /// an iterator we could not guarantee that the nodes do not get utilized
    /// after having been removed from the list anymore.
    pub fn reverse_drain<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut ListNode<T>),
    {
        let mut current = self.tail;
        self.head = None;
        self.tail = None;

        while let Some(mut node) = current {
            // Safety: The nodes have not been removed from the list yet and must
            // therefore contain valid data. The nodes can also not be added to
            // the list again during iteration, since the list is mutably borrowed.
            unsafe {
                let node_ref = node.as_mut();
                current = node_ref.prev;

                node_ref.next = None;
                node_ref.prev = None;

                // Note: We do not reset the pointers from the next element in the
                // list to the current one since we will iterate over the whole
                // list anyway, and therefore clean up all pointers.

                func(node_ref);
            }
        }
    }
}

#[cfg(all(test, feature = "alloc"))] // Tests make use of Vec at the moment
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn collect_list<T: Copy>(mut list: LinkedList<T>) -> Vec<T> {
        let mut result = Vec::new();
        list.drain(|node| {
            result.push(**node);
        });
        result
    }

    fn collect_reverse_list<T: Copy>(mut list: LinkedList<T>) -> Vec<T> {
        let mut result = Vec::new();
        list.reverse_drain(|node| {
            result.push(**node);
        });
        result
    }

    unsafe fn add_nodes(
        list: &mut LinkedList<i32>,
        nodes: &mut [&mut ListNode<i32>],
    ) {
        for node in nodes.iter_mut() {
            list.add_front(node);
        }
    }

    unsafe fn assert_clean<T>(node: &mut ListNode<T>) {
        assert!(node.next.is_none());
        assert!(node.prev.is_none());
    }

    #[test]
    fn insert_and_iterate() {
        unsafe {
            let mut a = ListNode::new(5);
            let mut b = ListNode::new(7);
            let mut c = ListNode::new(31);

            let mut setup = |list: &mut LinkedList<i32>| {
                assert_eq!(true, list.is_empty());
                list.add_front(&mut c);
                assert_eq!(31, **list.peek_first().unwrap());
                assert_eq!(false, list.is_empty());
                list.add_front(&mut b);
                assert_eq!(7, **list.peek_first().unwrap());
                list.add_front(&mut a);
                assert_eq!(5, **list.peek_first().unwrap());
            };

            let mut list = LinkedList::new();
            setup(&mut list);
            let items: Vec<i32> = collect_list(list);
            assert_eq!([5, 7, 31].to_vec(), items);

            let mut list = LinkedList::new();
            setup(&mut list);
            let items: Vec<i32> = collect_reverse_list(list);
            assert_eq!([31, 7, 5].to_vec(), items);
        }
    }

    #[test]
    fn drain_and_collect() {
        unsafe {
            let mut a = ListNode::new(5);
            let mut b = ListNode::new(7);
            let mut c = ListNode::new(31);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);

            let taken_items: Vec<i32> = collect_list(list);
            assert_eq!([5, 7, 31].to_vec(), taken_items);
        }
    }

    #[test]
    fn peek_last() {
        unsafe {
            let mut a = ListNode::new(5);
            let mut b = ListNode::new(7);
            let mut c = ListNode::new(31);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);

            let last = list.peek_last();
            assert_eq!(31, **last.unwrap());
            list.remove_last();

            let last = list.peek_last();
            assert_eq!(7, **last.unwrap());
            list.remove_last();

            let last = list.peek_last();
            assert_eq!(5, **last.unwrap());
            list.remove_last();

            let last = list.peek_last();
            assert!(last.is_none());
        }
    }

    #[test]
    fn remove_first() {
        unsafe {
            // We iterate forward and backwards through the manipulated lists
            // to make sure pointers in both directions are still ok.
            let mut a = ListNode::new(5);
            let mut b = ListNode::new(7);
            let mut c = ListNode::new(31);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
            let removed = list.remove_first().unwrap();
            assert_clean(removed);
            assert!(!list.is_empty());
            let items: Vec<i32> = collect_list(list);
            assert_eq!([7, 31].to_vec(), items);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
            let removed = list.remove_first().unwrap();
            assert_clean(removed);
            assert!(!list.is_empty());
            let items: Vec<i32> = collect_reverse_list(list);
            assert_eq!([31, 7].to_vec(), items);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut b, &mut a]);
            let removed = list.remove_first().unwrap();
            assert_clean(removed);
            assert!(!list.is_empty());
            let items: Vec<i32> = collect_list(list);
            assert_eq!([7].to_vec(), items);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut b, &mut a]);
            let removed = list.remove_first().unwrap();
            assert_clean(removed);
            assert!(!list.is_empty());
            let items: Vec<i32> = collect_reverse_list(list);
            assert_eq!([7].to_vec(), items);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut a]);
            let removed = list.remove_first().unwrap();
            assert_clean(removed);
            assert!(list.is_empty());
            let items: Vec<i32> = collect_list(list);
            assert!(items.is_empty());

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut a]);
            let removed = list.remove_first().unwrap();
            assert_clean(removed);
            assert!(list.is_empty());
            let items: Vec<i32> = collect_reverse_list(list);
            assert!(items.is_empty());
        }
    }

    #[test]
    fn remove_last() {
        unsafe {
            // We iterate forward and backwards through the manipulated lists
            // to make sure pointers in both directions are still ok.
            let mut a = ListNode::new(5);
            let mut b = ListNode::new(7);
            let mut c = ListNode::new(31);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
            let removed = list.remove_last().unwrap();
            assert_clean(removed);
            assert!(!list.is_empty());
            let items: Vec<i32> = collect_list(list);
            assert_eq!([5, 7].to_vec(), items);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
            let removed = list.remove_last().unwrap();
            assert_clean(removed);
            assert!(!list.is_empty());
            let items: Vec<i32> = collect_reverse_list(list);
            assert_eq!([7, 5].to_vec(), items);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut b, &mut a]);
            let removed = list.remove_last().unwrap();
            assert_clean(removed);
            assert!(!list.is_empty());
            let items: Vec<i32> = collect_list(list);
            assert_eq!([5].to_vec(), items);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut b, &mut a]);
            let removed = list.remove_last().unwrap();
            assert_clean(removed);
            assert!(!list.is_empty());
            let items: Vec<i32> = collect_reverse_list(list);
            assert_eq!([5].to_vec(), items);

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut a]);
            let removed = list.remove_last().unwrap();
            assert_clean(removed);
            assert!(list.is_empty());
            let items: Vec<i32> = collect_list(list);
            assert!(items.is_empty());

            let mut list = LinkedList::new();
            add_nodes(&mut list, &mut [&mut a]);
            let removed = list.remove_last().unwrap();
            assert_clean(removed);
            assert!(list.is_empty());
            let items: Vec<i32> = collect_reverse_list(list);
            assert!(items.is_empty());
        }
    }

    #[test]
    fn remove_by_address() {
        unsafe {
            let mut a = ListNode::new(5);
            let mut b = ListNode::new(7);
            let mut c = ListNode::new(31);

            {
                // Remove first
                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
                assert_eq!(true, list.remove(&mut a));
                assert_clean((&mut a).into());
                // a should be no longer there and can't be removed twice
                assert_eq!(false, list.remove(&mut a));
                assert_eq!(Some((&mut b).into()), list.head);
                assert_eq!(Some((&mut c).into()), b.next);
                assert_eq!(Some((&mut b).into()), c.prev);
                let items: Vec<i32> = collect_list(list);
                assert_eq!([7, 31].to_vec(), items);

                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
                assert_eq!(true, list.remove(&mut a));
                assert_clean((&mut a).into());
                // a should be no longer there and can't be removed twice
                assert_eq!(false, list.remove(&mut a));
                assert_eq!(Some((&mut c).into()), b.next);
                assert_eq!(Some((&mut b).into()), c.prev);
                let items: Vec<i32> = collect_reverse_list(list);
                assert_eq!([31, 7].to_vec(), items);
            }

            {
                // Remove middle
                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
                assert_eq!(true, list.remove(&mut b));
                assert_clean((&mut b).into());
                assert_eq!(Some((&mut c).into()), a.next);
                assert_eq!(Some((&mut a).into()), c.prev);
                let items: Vec<i32> = collect_list(list);
                assert_eq!([5, 31].to_vec(), items);

                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
                assert_eq!(true, list.remove(&mut b));
                assert_clean((&mut b).into());
                assert_eq!(Some((&mut c).into()), a.next);
                assert_eq!(Some((&mut a).into()), c.prev);
                let items: Vec<i32> = collect_reverse_list(list);
                assert_eq!([31, 5].to_vec(), items);
            }

            {
                // Remove last
                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
                assert_eq!(true, list.remove(&mut c));
                assert_clean((&mut c).into());
                assert!(b.next.is_none());
                assert_eq!(Some((&mut b).into()), list.tail);
                let items: Vec<i32> = collect_list(list);
                assert_eq!([5, 7].to_vec(), items);

                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut c, &mut b, &mut a]);
                assert_eq!(true, list.remove(&mut c));
                assert_clean((&mut c).into());
                assert!(b.next.is_none());
                assert_eq!(Some((&mut b).into()), list.tail);
                let items: Vec<i32> = collect_reverse_list(list);
                assert_eq!([7, 5].to_vec(), items);
            }

            {
                // Remove first of two
                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut b, &mut a]);
                assert_eq!(true, list.remove(&mut a));
                assert_clean((&mut a).into());
                // a should be no longer there and can't be removed twice
                assert_eq!(false, list.remove(&mut a));
                assert_eq!(Some((&mut b).into()), list.head);
                assert_eq!(Some((&mut b).into()), list.tail);
                assert!(b.next.is_none());
                assert!(b.prev.is_none());
                let items: Vec<i32> = collect_list(list);
                assert_eq!([7].to_vec(), items);

                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut b, &mut a]);
                assert_eq!(true, list.remove(&mut a));
                assert_clean((&mut a).into());
                // a should be no longer there and can't be removed twice
                assert_eq!(false, list.remove(&mut a));
                assert_eq!(Some((&mut b).into()), list.head);
                assert_eq!(Some((&mut b).into()), list.tail);
                assert!(b.next.is_none());
                assert!(b.prev.is_none());
                let items: Vec<i32> = collect_reverse_list(list);
                assert_eq!([7].to_vec(), items);
            }

            {
                // Remove last of two
                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut b, &mut a]);
                assert_eq!(true, list.remove(&mut b));
                assert_clean((&mut b).into());
                assert_eq!(Some((&mut a).into()), list.head);
                assert_eq!(Some((&mut a).into()), list.tail);
                assert!(a.next.is_none());
                assert!(a.prev.is_none());
                let items: Vec<i32> = collect_list(list);
                assert_eq!([5].to_vec(), items);

                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut b, &mut a]);
                assert_eq!(true, list.remove(&mut b));
                assert_clean((&mut b).into());
                assert_eq!(Some((&mut a).into()), list.head);
                assert_eq!(Some((&mut a).into()), list.tail);
                assert!(a.next.is_none());
                assert!(a.prev.is_none());
                let items: Vec<i32> = collect_reverse_list(list);
                assert_eq!([5].to_vec(), items);
            }

            {
                // Remove last item
                let mut list = LinkedList::new();
                add_nodes(&mut list, &mut [&mut a]);
                assert_eq!(true, list.remove(&mut a));
                assert_clean((&mut a).into());
                assert!(list.head.is_none());
                assert!(list.tail.is_none());
                let items: Vec<i32> = collect_list(list);
                assert!(items.is_empty());
            }

            {
                // Remove missing
                let mut list = LinkedList::new();
                list.add_front(&mut b);
                list.add_front(&mut a);
                assert_eq!(false, list.remove(&mut c));
            }
        }
    }
}

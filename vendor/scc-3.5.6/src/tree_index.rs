//! [`TreeIndex`] is a read-optimized asynchronous/concurrent B-plus tree.

mod internal_node;
mod leaf;
mod leaf_node;
mod node;

use std::fmt;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;
use std::panic::UnwindSafe;
use std::pin::pin;
use std::sync::atomic::Ordering::{AcqRel, Acquire};

use sdd::{AtomicShared, Guard, Ptr, Shared, Tag};

use crate::Comparable;
use crate::async_helper::AsyncPager;
use leaf::Iter as LeafIter;
use leaf::RevIter as LeafRevIter;
use leaf::{InsertResult, Leaf, RemoveResult};
use node::Node;

/// Scalable asynchronous/concurrent B-plus tree.
///
/// [`TreeIndex`] is an asynchronous/concurrent B-plus tree variant optimized for read operations.
/// Read operations, such as read iteration over entries, are neither blocked nor interrupted by
/// other threads or tasks. Write operations, such as insert and remove, do not block if structural
/// changes are not required.
///
/// ## Note
///
/// [`TreeIndex`] methods are linearizable. However, its iterator methods are not; [`Iter`] and
/// [`Range`] are only guaranteed to observe events that happened before the first call to
/// [`Iterator::next`].
///
/// ## The key features of [`TreeIndex`]
///
/// * Lock-free read: read and scan operations are never blocked and do not modify shared data.
/// * Near lock-free write: write operations do not block unless a structural change is needed.
/// * No busy waiting: each node has a wait queue to avoid spinning.
/// * Immutability: the data in the container is immutable until it becomes unreachable.
///
/// ## The key statistics for [`TreeIndex`]
///
/// * The maximum number of entries that a leaf can contain: 14.
/// * The maximum number of leaves or child nodes a node can contain: 15.
///
/// ## Locking behavior
///
/// Read access is always lock-free and non-blocking. Write access to an entry is also lock-free and
/// non-blocking as long as no structural changes are required. However, when nodes are split or
/// merged by a write operation, other write operations on keys in the affected range are blocked.
///
/// ### Synchronous methods in an asynchronous code block
///
/// It is generally not recommended to use blocking methods, such as [`TreeIndex::insert_sync`], in
/// an asynchronous code block or [`poll`](std::future::Future::poll), since it may lead to
/// deadlocks or performance degradation.
///
/// ### Unwind safety
///
/// [`TreeIndex`] is impervious to out-of-memory errors and panics in user-specified code under one
/// condition; `K::drop` and `V::drop` must not panic.
pub struct TreeIndex<K, V> {
    root: AtomicShared<Node<K, V>>,
}

/// An iterator over the entries of a [`TreeIndex`].
///
/// An [`Iter`] iterates over all the entries that exist during the lifetime of the [`Iter`] in
/// monotonically increasing order.
pub struct Iter<'t, 'g, K, V> {
    root: &'t AtomicShared<Node<K, V>>,
    forward: Option<LeafIter<'g, K, V>>,
    backward: Option<LeafRevIter<'g, K, V>>,
    guard: &'g Guard,
}

/// An iterator over a sub-range of entries in a [`TreeIndex`].
pub struct Range<'t, 'g, K, V, Q: ?Sized, R: RangeBounds<Q>> {
    root: &'t AtomicShared<Node<K, V>>,
    forward: Option<LeafIter<'g, K, V>>,
    backward: Option<LeafRevIter<'g, K, V>>,
    bounds: R,
    check_upper_bound: bool,
    check_lower_bound: bool,
    guard: &'g Guard,
    query: PhantomData<fn() -> Q>,
}

/// Proximity of the [`Iter`] to the key passed to [`TreeIndex::locate`].
pub enum Proximity<'t, 'g, K, V> {
    /// [`Iter`] that points to the exact key.
    ///
    /// [`Iter::get`] returns the exact key.
    Exact(Iter<'t, 'g, K, V>),
    /// [`Iter`] that points to the closest smaller key and the closest larger key.
    ///
    /// [`Iter::get`] returns the closest smaller key, and [`Iter::get_back`] returns the closest
    /// larger key.
    Between(Iter<'t, 'g, K, V>),
    /// [`Iter`] that points to the closest smaller key.
    ///
    /// The [`TreeIndex`] does not contain any larger keys than the specified key, and
    /// [`Iter::get_back`] returns the closest smaller key.
    Smaller(Iter<'t, 'g, K, V>),
    /// [`Iter`] that points to the closest larger key.
    ///
    /// The [`TreeIndex`] does not contain any smaller keys than the specified key, and
    /// [`Iter::get`] returns the closest larger key.
    Larger(Iter<'t, 'g, K, V>),
    /// The [`TreeIndex`] is empty.
    Empty,
}

impl<K, V> TreeIndex<K, V> {
    /// Creates an empty [`TreeIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            root: AtomicShared::null(),
        }
    }

    /// Creates an empty [`TreeIndex`].
    #[cfg(feature = "loom")]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: AtomicShared::null(),
        }
    }

    /// Clears the [`TreeIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// treeindex.clear();
    /// assert_eq!(treeindex.len(), 0);
    /// ```
    #[inline]
    pub fn clear(&self) {
        self.root.swap((None, Tag::None), Acquire);
    }

    /// Returns the depth of the [`TreeIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    /// assert_eq!(treeindex.depth(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn depth(&self) -> usize {
        let guard = Guard::new();
        self.root
            .load(Acquire, &guard)
            .as_ref()
            .map_or(0, |root| root.depth(1, &guard))
    }
}

impl<K, V> TreeIndex<K, V>
where
    K: 'static + Clone + Ord,
    V: 'static,
{
    /// Inserts a key-value pair.
    ///
    /// # Errors
    ///
    /// Returns an error along with the supplied key-value pair if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    /// let future_insert = treeindex.insert_async(1, 10);
    /// ```
    #[inline]
    pub async fn insert_async(&self, mut key: K, mut val: V) -> Result<(), (K, V)> {
        let mut pinned_pager = pin!(AsyncPager::default());
        loop {
            {
                let guard = Guard::new();
                let root_ptr = self.root.load(Acquire, &guard);
                if let Some(root) = root_ptr.as_ref() {
                    match root.insert(key, val, &mut pinned_pager, &guard) {
                        Ok(r) => match r {
                            InsertResult::Success => return Ok(()),
                            InsertResult::Duplicate(k, v) | InsertResult::Frozen(k, v) => {
                                return Err((k, v));
                            }
                            InsertResult::Full(k, v) => {
                                key = k;
                                val = v;
                                Node::split_root(root_ptr, &self.root, &guard);
                                continue;
                            }
                        },
                        Err((k, v)) => {
                            key = k;
                            val = v;
                        }
                    }
                } else if let Err((Some(new_node), _)) = self.root.compare_exchange(
                    Ptr::null(),
                    (Some(Shared::new_with(Node::new_leaf_node)), Tag::None),
                    AcqRel,
                    Acquire,
                    &Guard::new(),
                ) {
                    unsafe {
                        let _: bool = new_node.drop_in_place();
                    }
                    continue;
                }
            };
            pinned_pager.wait().await;
        }
    }

    /// Inserts a key-value pair.
    ///
    /// # Errors
    ///
    /// Returns an error along with the supplied key-value pair if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 10).is_ok());
    /// assert_eq!(treeindex.insert_sync(1, 11).err().unwrap(), (1, 11));
    /// assert_eq!(treeindex.peek_with(&1, |k, v| *v).unwrap(), 10);
    /// ```
    #[inline]
    pub fn insert_sync(&self, mut key: K, mut val: V) -> Result<(), (K, V)> {
        loop {
            let guard = Guard::new();
            let root_ptr = self.root.load(Acquire, &guard);
            if let Some(root) = root_ptr.as_ref() {
                match root.insert(key, val, &mut (), &guard) {
                    Ok(r) => match r {
                        InsertResult::Success => return Ok(()),
                        InsertResult::Duplicate(k, v) | InsertResult::Frozen(k, v) => {
                            return Err((k, v));
                        }
                        InsertResult::Full(k, v) => {
                            key = k;
                            val = v;
                            Node::split_root(root_ptr, &self.root, &guard);
                        }
                    },
                    Err((k, v)) => {
                        key = k;
                        val = v;
                    }
                }
            } else if let Err((Some(new_node), _)) = self.root.compare_exchange(
                Ptr::null(),
                (Some(Shared::new_with(Node::new_leaf_node)), Tag::None),
                AcqRel,
                Acquire,
                &Guard::new(),
            ) {
                unsafe {
                    let _: bool = new_node.drop_in_place();
                }
            }
        }
    }

    /// Removes a key-value pair.
    ///
    /// Returns `false` if the key does not exist. Returns `true` if the key existed and the
    /// condition was met after marking the entry unreachable; the memory will be reclaimed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    /// let future_remove = treeindex.remove_async(&1);
    /// ```
    #[inline]
    pub async fn remove_async<Q>(&self, key: &Q) -> bool
    where
        Q: Comparable<K> + ?Sized,
    {
        self.remove_if_async(key, |_| true).await
    }

    /// Removes a key-value pair.
    ///
    /// Returns `false` if the key does not exist.
    ///
    /// Returns `true` if the key existed and the condition was met after marking the entry
    /// unreachable; the memory will be reclaimed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(!treeindex.remove_sync(&1));
    /// assert!(treeindex.insert_sync(1, 10).is_ok());
    /// assert!(treeindex.remove_sync(&1));
    /// ```
    #[inline]
    pub fn remove_sync<Q>(&self, key: &Q) -> bool
    where
        Q: Comparable<K> + ?Sized,
    {
        self.remove_if_sync(key, |_| true)
    }

    /// Removes a key-value pair if the given condition is met.
    ///
    /// Returns `false` if the key does not exist or the condition was not met. Returns `true` if
    /// the key existed and the condition was met after marking the entry unreachable; the memory
    /// will be reclaimed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    /// let future_remove = treeindex.remove_if_async(&1, |v| *v == 0);
    /// ```
    #[inline]
    pub async fn remove_if_async<Q, F: FnMut(&V) -> bool>(&self, key: &Q, mut condition: F) -> bool
    where
        Q: Comparable<K> + ?Sized,
    {
        let mut pinned_pager = pin!(AsyncPager::default());
        let mut removed = false;
        loop {
            {
                let guard = Guard::new();
                if let Some(root) = self.root.load(Acquire, &guard).as_ref() {
                    if let Ok(result) =
                        root.remove_if::<_, _, _>(key, &mut condition, &mut pinned_pager, &guard)
                    {
                        match result {
                            RemoveResult::Success => return true,
                            RemoveResult::Retired => {
                                if Node::cleanup_root(&self.root, &mut pinned_pager, &guard) {
                                    return true;
                                }
                                removed = true;
                            }
                            RemoveResult::Fail => {
                                if removed {
                                    if Node::cleanup_root(&self.root, &mut pinned_pager, &guard) {
                                        return true;
                                    }
                                } else {
                                    return false;
                                }
                            }
                            RemoveResult::Frozen => (),
                        }
                    }
                } else {
                    return removed;
                }
            }
            pinned_pager.wait().await;
        }
    }

    /// Removes a key-value pair if the given condition is met.
    ///
    /// Returns `false` if the key does not exist or the condition was not met.
    ///
    /// Returns `true` if the key existed and the condition was met after marking the entry
    /// unreachable; the memory will be reclaimed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 10).is_ok());
    /// assert!(!treeindex.remove_if_sync(&1, |v| *v == 0));
    /// assert!(treeindex.remove_if_sync(&1, |v| *v == 10));
    /// ```
    #[inline]
    pub fn remove_if_sync<Q, F: FnMut(&V) -> bool>(&self, key: &Q, mut condition: F) -> bool
    where
        Q: Comparable<K> + ?Sized,
    {
        let mut removed = false;
        loop {
            let guard = Guard::new();
            if let Some(root) = self.root.load(Acquire, &guard).as_ref() {
                if let Ok(result) = root.remove_if::<_, _, _>(key, &mut condition, &mut (), &guard)
                {
                    match result {
                        RemoveResult::Success => return true,
                        RemoveResult::Retired => {
                            if Node::cleanup_root(&self.root, &mut (), &guard) {
                                return true;
                            }
                            removed = true;
                        }
                        RemoveResult::Fail => {
                            if removed {
                                if Node::cleanup_root(&self.root, &mut (), &guard) {
                                    return true;
                                }
                            } else {
                                return false;
                            }
                        }
                        RemoveResult::Frozen => (),
                    }
                }
            } else {
                return removed;
            }
        }
    }

    /// Removes keys in the specified range.
    ///
    /// This method removes internal nodes that are definitely contained in the specified range
    /// first, and then removes remaining entries individually.
    ///
    /// # Note
    ///
    /// Internally, multiple internal node locks need to be acquired, thus making this method
    /// susceptible to lock starvation.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// for k in 2..8 {
    ///     assert!(treeindex.insert_sync(k, 1).is_ok());
    /// }
    ///
    /// let future_remove_range = treeindex.remove_range_async(3..8);
    /// ```
    #[inline]
    pub async fn remove_range_async<Q, R: RangeBounds<Q>>(&self, range: R)
    where
        Q: Comparable<K> + ?Sized,
    {
        let mut pinned_pager = pin!(AsyncPager::default());
        let start_unbounded = matches!(range.start_bound(), Unbounded);

        loop {
            {
                let guard = Guard::new();

                // Remove internal nodes, and individual entries in affected leaves.
                //
                // It takes O(N) to traverse sub-trees on the range border.
                if let Some(root) = self.root.load(Acquire, &guard).as_ref() {
                    if let Ok(num_children) = root.remove_range(
                        &range,
                        start_unbounded,
                        None,
                        None,
                        &mut pinned_pager,
                        &guard,
                    ) {
                        if num_children >= 2
                            || Node::cleanup_root(&self.root, &mut pinned_pager, &guard)
                        {
                            // Completed removal and cleaning up the root.
                            return;
                        }
                    } else {
                        // The entire root node may have been retired.
                        Node::cleanup_root(&self.root, &mut (), &guard);
                    }
                } else {
                    // Nothing to remove.
                    return;
                }
            }
            pinned_pager.wait().await;
        }
    }

    /// Removes keys in the specified range.
    ///
    /// This method removes internal nodes that are definitely contained in the specified range
    /// first, and then removes remaining entries individually.
    ///
    /// # Note
    ///
    /// Internally, multiple internal node locks need to be acquired, thus making this method
    /// susceptible to lock starvation.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// for k in 2..8 {
    ///     assert!(treeindex.insert_sync(k, 1).is_ok());
    /// }
    ///
    /// treeindex.remove_range_sync(3..8);
    ///
    /// assert!(treeindex.contains(&2));
    /// assert!(!treeindex.contains(&3));
    /// ```
    #[inline]
    pub fn remove_range_sync<Q, R: RangeBounds<Q>>(&self, range: R)
    where
        Q: Comparable<K> + ?Sized,
    {
        let start_unbounded = matches!(range.start_bound(), Unbounded);
        let guard = Guard::new();

        // Remove internal nodes, and individual entries in affected leaves.
        //
        // It takes O(N) to traverse sub-trees on the range border.
        while let Some(root) = self.root.load(Acquire, &guard).as_ref() {
            if let Ok(num_children) =
                root.remove_range(&range, start_unbounded, None, None, &mut (), &guard)
            {
                if num_children < 2 && !Node::cleanup_root(&self.root, &mut (), &guard) {
                    continue;
                }
                break;
            }
            // The entire root node may have been retired.
            Node::cleanup_root(&self.root, &mut (), &guard);
        }
    }

    /// Returns a guarded reference to the value for the specified key without acquiring locks.
    ///
    /// Returns `None` if the key does not exist. The returned reference can survive as long as the
    /// associated [`Guard`] is alive.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    ///
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<Arc<str>, u32> = TreeIndex::new();
    ///
    /// let guard = Guard::new();
    /// assert!(treeindex.peek("foo", &guard).is_none());
    ///
    /// treeindex.insert_sync("foo".into(), 1).expect("insert in empty TreeIndex");
    /// ```
    #[inline]
    pub fn peek<'g, Q>(&self, key: &Q, guard: &'g Guard) -> Option<&'g V>
    where
        Q: Comparable<K> + ?Sized,
    {
        if let Some(root) = self.root.load(Acquire, guard).as_ref() {
            return root.search_value(key, guard);
        }
        None
    }

    /// Peeks a key-value pair without acquiring locks.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<Arc<str>, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.peek_with("foo", |k, v| *v).is_none());
    ///
    /// treeindex.insert_sync("foo".into(), 1).expect("insert in empty TreeIndex");
    ///
    /// let key: Arc<str> = treeindex
    ///     .peek_with("foo", |k, _v| Arc::clone(k))
    ///     .expect("peek_with by borrowed key");
    /// ```
    #[inline]
    pub fn peek_with<Q, R, F: FnOnce(&K, &V) -> R>(&self, key: &Q, reader: F) -> Option<R>
    where
        Q: Comparable<K> + ?Sized,
    {
        let guard = Guard::new();
        self.peek_entry(key, &guard).map(|(k, v)| reader(k, v))
    }

    /// Returns a guarded reference to the key-value pair for the specified key without acquiring locks.
    ///
    /// Returns `None` if the key does not exist. The returned reference can survive as long as the
    /// associated [`Guard`] is alive.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    ///
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<Arc<str>, u32> = TreeIndex::new();
    ///
    /// let guard = Guard::new();
    /// assert!(treeindex.peek_entry("foo", &guard).is_none());
    ///
    /// treeindex.insert_sync("foo".into(), 1).expect("insert in empty TreeIndex");
    ///
    /// let key: Arc<str> = treeindex
    ///     .peek_entry("foo", &guard)
    ///     .map(|(k, _v)| Arc::clone(k))
    ///     .expect("peek_entry by borrowed key");
    /// ```
    #[inline]
    pub fn peek_entry<'g, Q>(&self, key: &Q, guard: &'g Guard) -> Option<(&'g K, &'g V)>
    where
        Q: Comparable<K> + ?Sized,
    {
        if let Some(root) = self.root.load(Acquire, guard).as_ref() {
            return root.search_entry(key, guard);
        }
        None
    }

    /// Returns `true` if the [`TreeIndex`] contains the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::default();
    ///
    /// assert!(!treeindex.contains(&1));
    /// assert!(treeindex.insert_sync(1, 0).is_ok());
    /// assert!(treeindex.contains(&1));
    /// ```
    #[inline]
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        Q: Comparable<K> + ?Sized,
    {
        self.peek(key, &Guard::new()).is_some()
    }

    /// Returns the size of the [`TreeIndex`].
    ///
    /// It internally scans all the leaf nodes, and therefore the time complexity is O(N).
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    /// assert_eq!(treeindex.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        let guard = Guard::new();
        self.iter(&guard).count()
    }

    /// Returns `true` if the [`TreeIndex`] is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        let guard = Guard::new();
        !self.iter(&guard).any(|_| true)
    }

    /// Returns an [`Iter`].
    ///
    /// The returned [`Iter`] is a [`DoubleEndedIterator`] that allows scanning in both ascending
    /// and descending order. [`Iter`] may miss newly inserted key-value pairs after the invocation
    /// of this method, because [`Self::iter`] is the linearization point whereas [`Iter::next`] and
    /// [`Iter::next_back`] are not.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 2).is_ok());
    /// assert!(treeindex.insert_sync(3, 4).is_ok());
    ///
    /// let guard = Guard::new();
    /// let mut iter = treeindex.iter(&guard);
    /// assert_eq!(iter.next(), Some((&1, &2)));
    /// assert_eq!(iter.next_back(), Some((&3, &4)));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.next_back(), None);
    /// ```
    #[inline]
    pub const fn iter<'t, 'g>(&'t self, guard: &'g Guard) -> Iter<'t, 'g, K, V> {
        Iter::new(&self.root, guard)
    }

    /// Returns a [`Range`] that scans keys in the given range.
    ///
    /// Key-value pairs in the range are scanned in ascending order, and key-value pairs that have
    /// existed since the invocation of the method are guaranteed to be visited if they are not
    /// removed. However, it is possible to visit removed key-value pairs momentarily.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// let guard = Guard::new();
    /// assert_eq!(treeindex.range(4..=8, &guard).count(), 0);
    /// ```
    #[inline]
    pub const fn range<'t, 'g, Q, R: RangeBounds<Q>>(
        &'t self,
        range: R,
        guard: &'g Guard,
    ) -> Range<'t, 'g, K, V, Q, R>
    where
        Q: Comparable<K> + ?Sized,
    {
        Range::new(&self.root, range, guard)
    }

    /// Returns a [`Proximity`] optionally containing an [`Iter`] that points to the entry with the
    /// specified key or the closest one if the [`TreeIndex`] is not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    /// use scc::tree_index::Proximity;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 1).is_ok());
    /// assert!(treeindex.insert_sync(3, 2).is_ok());
    /// assert!(treeindex.insert_sync(5, 3).is_ok());
    /// assert!(treeindex.insert_sync(7, 4).is_ok());
    ///
    /// let guard = Guard::new();
    ///
    /// let Proximity::Exact(iter) = treeindex.locate(&5, &guard) else {
    ///     unreachable!();
    /// };
    /// assert_eq!(iter.get(), Some((&5, &3)));
    ///
    /// let Proximity::Between(mut iter) = treeindex.locate(&2, &guard) else {
    ///     unreachable!();
    /// };
    /// assert_eq!(iter.get(), Some((&1, &1)));
    /// assert_eq!(iter.get_back(), Some((&3, &2)));
    /// assert!(iter.next().is_none());
    /// assert!(iter.next_back().is_none());
    ///
    /// let Proximity::Smaller(iter) = treeindex.locate(&8, &guard) else {
    ///     unreachable!();
    /// };
    /// assert_eq!(iter.get_back(), Some((&7, &4)));
    ///
    /// let Proximity::Larger(iter) = treeindex.locate(&0, &guard) else {
    ///     unreachable!();
    /// };
    /// assert_eq!(iter.get(), Some((&1, &1)));
    ///
    /// treeindex.clear();
    ///
    /// let Proximity::Empty = treeindex.locate(&3, &guard) else {
    ///     unreachable!();
    /// };
    /// ```
    pub fn locate<'t, 'g, Q>(&'t self, key: &Q, guard: &'g Guard) -> Proximity<'t, 'g, K, V>
    where
        Q: Comparable<K> + ?Sized,
    {
        if let Some(root) = self.root.load(Acquire, guard).as_ref() {
            if let Some(mut iter) = root.approximate::<_, true>(key, guard) {
                // Found a key that exactly matches the specified one or smaller.
                let mut prev_iter = iter.clone();
                while let Some((k, _)) = iter.get() {
                    let comparison = key.compare(k);
                    if comparison.is_eq() {
                        // Exact match found.
                        return Proximity::Exact(Iter {
                            root: &self.root,
                            forward: Some(iter),
                            backward: None,
                            guard,
                        });
                    } else if comparison.is_lt() {
                        // Just passed the key.
                        return Proximity::Between(Iter {
                            root: &self.root,
                            forward: Some(prev_iter),
                            backward: Some(iter.rev()),
                            guard,
                        });
                    }
                    prev_iter = iter.clone();
                    if iter.next().is_none() {
                        iter.jump(guard);
                    }
                }
                // No keys larger than or equal to the specified one found.
                if prev_iter.get().is_some() {
                    return Proximity::Smaller(Iter {
                        root: &self.root,
                        forward: None,
                        backward: Some(prev_iter.rev()),
                        guard,
                    });
                }
            }
            if let Some(iter) = root.approximate::<_, false>(key, guard) {
                // Found a key that exactly matches the specified one or greater.
                let mut rev_iter = iter.rev();
                let mut prev_rev_iter = rev_iter.clone();
                while let Some((k, _)) = rev_iter.get() {
                    let comparison = key.compare(k);
                    if comparison.is_eq() {
                        // Exact match found.
                        return Proximity::Exact(Iter {
                            root: &self.root,
                            forward: Some(rev_iter.rev()),
                            backward: None,
                            guard,
                        });
                    } else if comparison.is_gt() {
                        // Just passed the key.
                        return Proximity::Between(Iter {
                            root: &self.root,
                            forward: Some(rev_iter.rev()),
                            backward: Some(prev_rev_iter),
                            guard,
                        });
                    }
                    prev_rev_iter = rev_iter.clone();
                    if rev_iter.next().is_none() {
                        rev_iter.jump(guard);
                    }
                }
                // No keys smaller than or equal to the specified one found.
                if prev_rev_iter.get().is_some() {
                    return Proximity::Larger(Iter {
                        root: &self.root,
                        forward: Some(prev_rev_iter.rev()),
                        backward: None,
                        guard,
                    });
                }
            }
        }
        Proximity::Empty
    }
}

impl<K, V> Clone for TreeIndex<K, V>
where
    K: 'static + Clone + Ord,
    V: 'static + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        let self_clone = Self::default();
        for (k, v) in self.iter(&Guard::new()) {
            let _result: Result<(), (K, V)> = self_clone.insert_sync(k.clone(), v.clone());
        }
        self_clone
    }
}

impl<K, V> fmt::Debug for TreeIndex<K, V>
where
    K: 'static + Clone + fmt::Debug + Ord,
    V: 'static + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = Guard::new();
        f.write_str("TreeIndex { ")?;
        if let Some(root) = self.root.load(Acquire, &guard).as_ref() {
            f.write_str(" root: ")?;
            root.fmt(f)?;
        }
        f.write_str(" }")
    }
}

impl<K, V> Default for TreeIndex<K, V> {
    /// Creates a [`TreeIndex`] with the default parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::default();
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> PartialEq for TreeIndex<K, V>
where
    K: 'static + Clone + Ord,
    V: 'static + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // The key order is preserved, therefore comparing iterators suffices.
        let guard = Guard::new();
        Iterator::eq(self.iter(&guard), other.iter(&guard))
    }
}

impl<K, V> UnwindSafe for TreeIndex<K, V> {}

impl<'t, 'g, K, V> Iter<'t, 'g, K, V> {
    /// Returns the entry that the forward iterator points to.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 2).is_ok());
    ///
    /// let guard = Guard::new();
    /// let mut iter = treeindex.iter(&guard);
    /// assert_eq!(iter.next(), Some((&1, &2)));
    /// assert_eq!(iter.get(), Some((&1, &2)));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.get(), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> Option<(&'g K, &'g V)> {
        if let Some(iter) = self.forward.as_ref() {
            iter.get()
        } else {
            None
        }
    }

    /// Returns the entry that the backward iterator points to.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 2).is_ok());
    ///
    /// let guard = Guard::new();
    /// let mut iter = treeindex.iter(&guard);
    /// assert_eq!(iter.next_back(), Some((&1, &2)));
    /// assert_eq!(iter.get_back(), Some((&1, &2)));
    /// assert_eq!(iter.next_back(), None);
    /// assert_eq!(iter.get_back(), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_back(&self) -> Option<(&'g K, &'g V)> {
        if let Some(rev_iter) = self.backward.as_ref() {
            rev_iter.get()
        } else {
            None
        }
    }

    /// Changes the direction of the iterator if only one end of the iterator is open.
    ///
    /// Returns `false` if the iterator is already bidirectional or was exhausted.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 2).is_ok());
    /// assert!(treeindex.insert_sync(2, 2).is_ok());
    /// assert!(treeindex.insert_sync(3, 2).is_ok());
    ///
    /// let guard = Guard::new();
    /// let mut iter = treeindex.iter(&guard);
    ///
    /// assert_eq!(iter.next_back(), Some((&3, &2)));
    /// assert_eq!(iter.next_back(), Some((&2, &2)));
    /// assert!(iter.flip());
    /// assert_eq!(iter.next(), Some((&3, &2)));
    /// assert!(iter.flip());
    ///
    /// assert_eq!(iter.next_back(), Some((&2, &2)));
    /// assert_eq!(iter.next(), Some((&1, &2)));
    /// assert!(iter.next().is_none());
    /// assert!(!iter.flip());
    /// ```
    #[inline]
    pub const fn flip(&mut self) -> bool {
        if self.backward.is_none()
            && self.get().is_some()
            && let Some(forward) = self.forward.take()
        {
            self.backward = Some(forward.rev());
            return true;
        }
        if self.forward.is_none()
            && self.get_back().is_some()
            && let Some(backward) = self.backward.take()
        {
            self.forward = Some(backward.rev());
            return true;
        }
        self.forward.is_none() && self.backward.is_none()
    }

    #[inline]
    const fn new(root: &'t AtomicShared<Node<K, V>>, guard: &'g Guard) -> Iter<'t, 'g, K, V> {
        Iter::<'t, 'g, K, V> {
            root,
            forward: None,
            backward: None,
            guard,
        }
    }
}

impl<'g, K, V> Iter<'_, 'g, K, V>
where
    K: Ord,
{
    /// Checks if the both ends of the iterators collide.
    fn check_collision<const FORWARD: bool>(
        &self,
        entry: (&'g K, &'g V),
    ) -> Option<(&'g K, &'g V)> {
        let other_entry = if FORWARD {
            self.backward.as_ref().and_then(LeafRevIter::get)
        } else {
            self.forward.as_ref().and_then(LeafIter::get)
        };
        let Some(other_entry) = other_entry else {
            // The other iterator was exhausted.
            return None;
        };
        if (FORWARD && other_entry.0 > entry.0) || (!FORWARD && other_entry.0 < entry.0) {
            return Some(entry);
        }
        None
    }
}

impl<K, V> Clone for Iter<'_, '_, K, V>
where
    K: 'static + Clone + Ord,
    V: 'static,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            root: self.root,
            forward: self.forward.as_ref().map(LeafIter::clone),
            backward: self.backward.as_ref().map(LeafRevIter::clone),
            guard: self.guard,
        }
    }
}

impl<K, V> fmt::Debug for Iter<'_, '_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iter")
            .field("forward_iter", &self.forward)
            .field("backward_iter", &self.backward)
            .finish()
    }
}

impl<K, V> DoubleEndedIterator for Iter<'_, '_, K, V>
where
    K: 'static + Clone + Ord,
    V: 'static,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // Start iteration.
        if self.backward.is_none() {
            let root_ptr = self.root.load(Acquire, self.guard);
            if let Some(root) = root_ptr.as_ref() {
                if let Some(rev_iter) = root.max(self.guard) {
                    self.backward.replace(rev_iter);
                }
            } else {
                return None;
            }
        }

        // Go to the prev entry.
        if let Some(rev_iter) = self.backward.as_mut() {
            if let Some(entry) = rev_iter.next() {
                if self.forward.is_none() {
                    return Some(entry);
                }
                return self.check_collision::<false>(entry);
            }
            // Go to the prev leaf node.
            if let Some(entry) = rev_iter.jump(self.guard) {
                if self.forward.is_none() {
                    return Some(entry);
                }
                return self.check_collision::<false>(entry);
            }
        }

        None
    }
}

impl<'g, K, V> Iterator for Iter<'_, 'g, K, V>
where
    K: 'static + Clone + Ord,
    V: 'static,
{
    type Item = (&'g K, &'g V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Start iteration.
        if self.forward.is_none() {
            let root_ptr = self.root.load(Acquire, self.guard);
            if let Some(root) = root_ptr.as_ref() {
                if let Some(iter) = root.min(self.guard) {
                    self.forward.replace(iter);
                }
            } else {
                return None;
            }
        }

        // Go to the next entry.
        if let Some(iter) = self.forward.as_mut() {
            if let Some(entry) = iter.next() {
                if self.backward.is_none() {
                    return Some(entry);
                }
                return self.check_collision::<true>(entry);
            }
            // Go to the next leaf node.
            if let Some(entry) = iter.jump(self.guard) {
                if self.backward.is_none() {
                    return Some(entry);
                }
                return self.check_collision::<true>(entry);
            }
        }

        None
    }
}

impl<'t, 'g, K, V, Q, R> From<Range<'t, 'g, K, V, Q, R>> for Iter<'t, 'g, K, V>
where
    Q: Comparable<K> + ?Sized,
    R: RangeBounds<Q>,
{
    #[inline]
    fn from(range: Range<'t, 'g, K, V, Q, R>) -> Self {
        Self {
            root: range.root,
            forward: range.forward,
            backward: range.backward,
            guard: range.guard,
        }
    }
}

impl<K, V> FusedIterator for Iter<'_, '_, K, V>
where
    K: 'static + Clone + Ord,
    V: 'static,
{
}

impl<K, V> UnwindSafe for Iter<'_, '_, K, V> {}

impl<'t, 'g, K, V, Q: ?Sized, R: RangeBounds<Q>> Range<'t, 'g, K, V, Q, R> {
    #[inline]
    const fn new(
        root: &'t AtomicShared<Node<K, V>>,
        range: R,
        guard: &'g Guard,
    ) -> Range<'t, 'g, K, V, Q, R> {
        Range::<'t, 'g, K, V, Q, R> {
            root,
            forward: None,
            backward: None,
            bounds: range,
            check_upper_bound: false,
            check_lower_bound: false,
            guard,
            query: PhantomData,
        }
    }
}

impl<'g, K, V, Q, R> Range<'_, 'g, K, V, Q, R>
where
    K: 'static + Clone + Ord,
    V: 'static,
    Q: Comparable<K> + ?Sized,
    R: RangeBounds<Q>,
{
    /// Returns the entry that the forward iterator points to.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 2).is_ok());
    /// assert!(treeindex.insert_sync(2, 3).is_ok());
    /// assert!(treeindex.insert_sync(4, 6).is_ok());
    /// assert!(treeindex.insert_sync(8, 12).is_ok());
    ///
    /// let guard = Guard::new();
    /// let mut range = treeindex.range(3..=4, &guard);
    /// assert_eq!(range.next(), Some((&4, &6)));
    /// assert_eq!(range.get(), Some((&4, &6)));
    /// assert_eq!(range.next(), None);
    /// assert_eq!(range.get(), None);
    /// ```
    #[inline]
    pub fn get(&self) -> Option<(&'g K, &'g V)> {
        self.forward.as_ref().and_then(|iter| {
            if let Some(entry) = iter.get() {
                if !self.check_upper_bound || self.check_upper_bound(entry.0) {
                    return Some(entry);
                }
            }
            None
        })
    }

    /// Returns the entry that the backward iterator points to.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 2).is_ok());
    /// assert!(treeindex.insert_sync(2, 3).is_ok());
    /// assert!(treeindex.insert_sync(4, 6).is_ok());
    /// assert!(treeindex.insert_sync(8, 12).is_ok());
    ///
    /// let guard = Guard::new();
    /// let mut range = treeindex.range(3..=4, &guard);
    /// assert_eq!(range.next_back(), Some((&4, &6)));
    /// assert_eq!(range.get_back(), Some((&4, &6)));
    /// assert_eq!(range.next_back(), None);
    /// assert_eq!(range.get_back(), None);
    /// ```
    #[inline]
    pub fn get_back(&self) -> Option<(&'g K, &'g V)> {
        self.backward.as_ref().and_then(|rev_iter| {
            if let Some(entry) = rev_iter.get() {
                if !self.check_lower_bound || self.check_lower_bound(entry.0) {
                    return Some(entry);
                }
            }
            None
        })
    }

    /// Changes the direction of the range iterator if only one end of the iterator is open.
    ///
    /// Returns `false` if the range iterator is already bidirectional or was exhausted.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::TreeIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let treeindex: TreeIndex<u64, u32> = TreeIndex::new();
    ///
    /// assert!(treeindex.insert_sync(1, 2).is_ok());
    /// assert!(treeindex.insert_sync(2, 2).is_ok());
    /// assert!(treeindex.insert_sync(3, 2).is_ok());
    /// assert!(treeindex.insert_sync(4, 2).is_ok());
    ///
    /// let guard = Guard::new();
    /// let mut range = treeindex.range(1..4, &guard);
    ///
    /// assert_eq!(range.next_back(), Some((&3, &2)));
    /// assert_eq!(range.next_back(), Some((&2, &2)));
    /// assert!(range.flip());
    /// assert_eq!(range.next(), Some((&3, &2)));
    /// assert!(range.flip());
    ///
    /// assert_eq!(range.next_back(), Some((&2, &2)));
    /// assert_eq!(range.next(), Some((&1, &2)));
    /// assert!(range.next().is_none());
    /// assert!(!range.flip());
    /// ```
    #[inline]
    pub fn flip(&mut self) -> bool {
        if self.backward.is_none()
            && self.get().is_some()
            && let Some(forward) = self.forward.take()
        {
            let backward = forward.rev();
            let min_key = backward.min_key();
            self.backward = Some(backward);
            self.check_upper_bound = false;
            self.set_check_lower_bound(min_key);
            return true;
        }
        if self.forward.is_none()
            && self.get_back().is_some()
            && let Some(backward) = self.backward.take()
        {
            let forward = backward.rev();
            let max_key = forward.max_key();
            self.forward = Some(forward);
            self.check_lower_bound = false;
            self.set_check_upper_bound(max_key);
            return true;
        }
        self.forward.is_none() && self.backward.is_none()
    }

    /// Starts forward iteration.
    fn start_forward(&mut self) -> Option<(&'g K, &'g V)> {
        let root_ptr = self.root.load(Acquire, self.guard);
        if let Some(root) = root_ptr.as_ref() {
            let mut leaf_iter = match self.bounds.start_bound() {
                Excluded(k) | Included(k) => root.approximate::<_, true>(k, self.guard),
                Unbounded => None,
            };
            if leaf_iter.is_none() {
                if let Some(mut iter) = root.min(self.guard) {
                    iter.next();
                    leaf_iter.replace(iter);
                }
            }
            if let Some(mut leaf_iter) = leaf_iter {
                while let Some((k, v)) = leaf_iter.get() {
                    let check_failed = match self.bounds.start_bound() {
                        Excluded(key) => key.compare(k).is_ge(),
                        Included(key) => key.compare(k).is_gt(),
                        Unbounded => false,
                    };
                    if check_failed {
                        if leaf_iter.next().is_none() {
                            leaf_iter.jump(self.guard)?;
                        }
                        continue;
                    }

                    let max_key = leaf_iter.max_key();
                    self.set_check_upper_bound(max_key);
                    self.forward.replace(leaf_iter);
                    return Some((k, v));
                }
            }
        }
        None
    }

    /// Starts backward iteration.
    fn start_backward(&mut self) -> Option<(&'g K, &'g V)> {
        let root_ptr = self.root.load(Acquire, self.guard);
        if let Some(root) = root_ptr.as_ref() {
            let mut leaf_iter = match self.bounds.end_bound() {
                Excluded(k) | Included(k) => root
                    .approximate::<_, false>(k, self.guard)
                    .map(LeafIter::rev),
                Unbounded => None,
            };
            if leaf_iter.is_none() {
                if let Some(mut iter) = root.max(self.guard) {
                    iter.next();
                    leaf_iter.replace(iter);
                }
            }
            if let Some(mut leaf_iter) = leaf_iter {
                while let Some((k, v)) = leaf_iter.get() {
                    let check_failed = match self.bounds.end_bound() {
                        Excluded(key) => key.compare(k).is_le(),
                        Included(key) => key.compare(k).is_lt(),
                        Unbounded => false,
                    };
                    if check_failed {
                        if leaf_iter.next().is_none() {
                            leaf_iter.jump(self.guard)?;
                        }
                        continue;
                    }
                    let min_key = leaf_iter.min_key();
                    self.set_check_lower_bound(min_key);
                    self.backward.replace(leaf_iter);
                    return Some((k, v));
                }
            }
        }
        None
    }

    /// Moves to the next entry without checking the bounds.
    #[inline]
    fn forward_unbounded(&mut self) -> Option<(&'g K, &'g V)> {
        if self.forward.is_none() {
            return self.start_forward();
        }

        // Go to the next entry.
        if let Some(leaf_iter) = self.forward.as_mut() {
            if let Some(result) = leaf_iter.next() {
                return Some(result);
            }
            // Go to the next leaf node.
            if let Some(entry) = leaf_iter.jump(self.guard) {
                let max_key = leaf_iter.max_key();
                self.set_check_upper_bound(max_key);
                return Some(entry);
            }
        }

        None
    }

    /// Moves to the prev entry without checking the bounds.
    #[inline]
    fn backward_unbounded(&mut self) -> Option<(&'g K, &'g V)> {
        if self.backward.is_none() {
            return self.start_backward();
        }

        // Go to the next entry.
        if let Some(leaf_iter) = self.backward.as_mut() {
            if let Some(result) = leaf_iter.next() {
                return Some(result);
            }
            // Go to the next leaf node.
            if let Some(entry) = leaf_iter.jump(self.guard) {
                let min_key = leaf_iter.min_key();
                self.set_check_lower_bound(min_key);
                return Some(entry);
            }
        }

        None
    }

    /// Sets whether to check the upper bound.
    #[inline]
    fn set_check_upper_bound(&mut self, max_key: Option<&'g K>) {
        self.check_upper_bound = match self.bounds.end_bound() {
            Excluded(key) => max_key.is_some_and(|k| key.compare(k).is_le()),
            Included(key) => max_key.is_some_and(|k| key.compare(k).is_lt()),
            Unbounded => false,
        };
    }

    /// Sets whether to check the upper bound.
    #[inline]
    fn set_check_lower_bound(&mut self, min_key: Option<&'g K>) {
        self.check_lower_bound = match self.bounds.start_bound() {
            Excluded(key) => min_key.is_some_and(|k| key.compare(k).is_ge()),
            Included(key) => min_key.is_some_and(|k| key.compare(k).is_gt()),
            Unbounded => false,
        };
    }

    /// Checks if the both ends of the iterators collide.
    fn check_collision<const FORWARD: bool>(
        &self,
        entry: (&'g K, &'g V),
    ) -> Option<(&'g K, &'g V)> {
        let other_entry = if FORWARD {
            self.backward.as_ref().and_then(LeafRevIter::get)
        } else {
            self.forward.as_ref().and_then(LeafIter::get)
        };
        let Some(other_entry) = other_entry else {
            // The other iterator was exhausted.
            return None;
        };
        if (FORWARD && other_entry.0 > entry.0) || (!FORWARD && other_entry.0 < entry.0) {
            return Some(entry);
        }
        None
    }

    /// Checks the lower bound.
    fn check_lower_bound(&self, k: &K) -> bool {
        match self.bounds.start_bound() {
            Excluded(key) => key.compare(k).is_lt(),
            Included(key) => key.compare(k).is_le(),
            Unbounded => true,
        }
    }

    /// Checks the upper bound.
    fn check_upper_bound(&self, k: &K) -> bool {
        match self.bounds.end_bound() {
            Excluded(key) => key.compare(k).is_gt(),
            Included(key) => key.compare(k).is_ge(),
            Unbounded => true,
        }
    }
}

impl<K, V, Q, R> Clone for Range<'_, '_, K, V, Q, R>
where
    K: 'static + Clone + Ord,
    V: 'static,
    Q: Comparable<K> + ?Sized,
    R: Clone + RangeBounds<Q>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            root: self.root,
            forward: self.forward.as_ref().map(LeafIter::clone),
            backward: self.backward.as_ref().map(LeafRevIter::clone),
            bounds: self.bounds.clone(),
            check_upper_bound: self.check_upper_bound,
            check_lower_bound: self.check_lower_bound,
            guard: self.guard,
            query: PhantomData,
        }
    }
}

impl<K, V, Q: ?Sized, R: RangeBounds<Q>> fmt::Debug for Range<'_, '_, K, V, Q, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Range")
            .field("forward_iter", &self.forward)
            .field("backward_iter", &self.backward)
            .field("check_upper_bound", &self.check_upper_bound)
            .field("check_lower_bound", &self.check_upper_bound)
            .finish()
    }
}

impl<K, V, Q, R> DoubleEndedIterator for Range<'_, '_, K, V, Q, R>
where
    K: 'static + Clone + Ord,
    V: 'static,
    Q: Comparable<K> + ?Sized,
    R: RangeBounds<Q>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.backward_unbounded() {
            if self.check_lower_bound && !self.check_lower_bound(entry.0) {
                return None;
            }
            if self.forward.is_none() {
                return Some(entry);
            }
            return self.check_collision::<false>(entry);
        }
        None
    }
}
impl<'g, K, V, Q, R> Iterator for Range<'_, 'g, K, V, Q, R>
where
    K: 'static + Clone + Ord,
    V: 'static,
    Q: Comparable<K> + ?Sized,
    R: RangeBounds<Q>,
{
    type Item = (&'g K, &'g V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.forward_unbounded() {
            if self.check_upper_bound && !self.check_upper_bound(entry.0) {
                return None;
            }
            if self.backward.is_none() {
                return Some(entry);
            }
            return self.check_collision::<true>(entry);
        }
        None
    }
}

impl<K, V, Q, R> FusedIterator for Range<'_, '_, K, V, Q, R>
where
    K: 'static + Clone + Ord,
    V: 'static,
    Q: Comparable<K> + ?Sized,
    R: RangeBounds<Q>,
{
}

impl<K, V, Q, R> UnwindSafe for Range<'_, '_, K, V, Q, R>
where
    Q: ?Sized,
    R: RangeBounds<Q> + UnwindSafe,
{
}

impl<K, V> fmt::Debug for Proximity<'_, '_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(iter) => f.debug_tuple("Exact").field(iter).finish(),
            Self::Between(iter) => f.debug_tuple("Between").field(iter).finish(),
            Self::Smaller(iter) => f.debug_tuple("Smaller").field(iter).finish(),
            Self::Larger(iter) => f.debug_tuple("Larger").field(iter).finish(),
            Self::Empty => write!(f, "Empty"),
        }
    }
}

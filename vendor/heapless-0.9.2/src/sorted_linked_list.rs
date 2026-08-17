//! A fixed sorted priority linked list, similar to [`BinaryHeap`] but with different properties
//! on `push`, `pop`, etc.
//!
//! For example, the sorting of the list will never `memcpy` the underlying value, so having large
//! objects in the list will not cause a performance hit.
//!
//! # Examples
//!
//! ```
//! use heapless::sorted_linked_list::{Max, SortedLinkedList};
//! let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
//!
//! // The largest value will always be first
//! ll.push(1).unwrap();
//! assert_eq!(ll.peek(), Some(&1));
//!
//! ll.push(2).unwrap();
//! assert_eq!(ll.peek(), Some(&2));
//!
//! ll.push(3).unwrap();
//! assert_eq!(ll.peek(), Some(&3));
//!
//! // This will not fit in the queue.
//! assert_eq!(ll.push(4), Err(4));
//! ```
//!
//! [`BinaryHeap`]: `crate::binary_heap::BinaryHeap`

use core::{
    cmp::Ordering,
    fmt,
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr,
};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

mod storage {
    use super::{LenType, Node, SortedLinkedListInner, SortedLinkedListView};

    /// Trait defining how data for a container is stored.
    ///
    /// There's two implementations available:
    ///
    /// - [`OwnedSortedLinkedListStorage`]: stores the data in an array `[T; N]` whose size is known
    ///   at compile time.
    /// - [`ViewSortedLinkedListStorage`]: stores the data in an unsized `[T]`.
    ///
    /// This allows [`SortedLinkedList`] to be generic over either sized or unsized storage. The
    /// [`sorted_linked_list`](super) module contains a [`SortedLinkedListInner`] struct that's
    /// generic on [`SortedLinkedListStorage`], and two type aliases for convenience:
    ///
    /// - [`SortedLinkedList<T, Idx, N>`](super::SortedLinkedList) = `SortedLinkedListInner<T,
    ///   OwnedSortedLinkedListStorage<T, Idx, N>>`
    /// - [`SortedLinkedListView<T, Idx>`](super::SortedLinkedListView) = `SortedLinkedListInner<T,
    ///   ViewSortedLinkedListStorage<T, Idx>>`
    ///
    /// `SortedLinkedList` can be unsized into `SortedLinkedListView`, either by unsizing coercions
    /// such as `&mut SortedLinkedList -> &mut SortedLinkedListView` or `Box<SortedLinkedList>
    /// -> Box<SortedLinkedListView>`, or explicitly with
    /// [`.as_view()`](super::SortedLinkedList::as_view) or
    /// [`.as_mut_view()`](super::SortedLinkedList::as_mut_view).
    ///
    /// This trait is sealed, so you cannot implement it for your own types. You can only use
    /// the implementations provided by this crate.
    ///
    /// [`SortedLinkedListInner`]: super::SortedLinkedListInner
    /// [`SortedLinkedList`]: super::SortedLinkedList
    /// [`SortedLinkedListView`]: super::SortedLinkedListView
    #[allow(private_bounds)]
    pub trait SortedLinkedListStorage<T, Idx>: SortedLinkedListSealedStorage<T, Idx> {}

    pub trait SortedLinkedListSealedStorage<T, Idx> {
        // part of the sealed trait so that no trait is publicly implemented by
        // `OwnedSortedLinkedListStorage` besides `Storage`
        fn borrow(&self) -> &[Node<T, Idx>];
        fn borrow_mut(&mut self) -> &mut [Node<T, Idx>];
        fn as_view<K>(
            this: &SortedLinkedListInner<T, Idx, K, Self>,
        ) -> &SortedLinkedListView<T, K, Idx>
        where
            Idx: LenType,
            Self: SortedLinkedListStorage<T, Idx>;
        fn as_mut_view<K>(
            this: &mut SortedLinkedListInner<T, Idx, K, Self>,
        ) -> &mut SortedLinkedListView<T, K, Idx>
        where
            Idx: LenType,
            Self: SortedLinkedListStorage<T, Idx>;
    }

    // One sealed layer of indirection to hide the internal details (The MaybeUninit).
    pub struct SortedLinkedListStorageInner<T: ?Sized> {
        pub(crate) buffer: T,
    }

    /// Implementation of [`SortedLinkedListStorage`] that stores the data in an array `[T; N]`
    /// whose size is known at compile time.
    pub type OwnedSortedLinkedListStorage<T, Idx, const N: usize> =
        SortedLinkedListStorageInner<[Node<T, Idx>; N]>;
    /// Implementation of [`SortedLinkedListStorage`] that stores the data in an unsized `[T]`.
    pub type ViewSortedLinkedListStorage<T, Idx> = SortedLinkedListStorageInner<[Node<T, Idx>]>;

    impl<T, Idx, const N: usize> SortedLinkedListSealedStorage<T, Idx>
        for OwnedSortedLinkedListStorage<T, Idx, N>
    {
        fn borrow(&self) -> &[Node<T, Idx>] {
            &self.buffer
        }
        fn borrow_mut(&mut self) -> &mut [Node<T, Idx>] {
            &mut self.buffer
        }
        fn as_view<K>(
            this: &SortedLinkedListInner<T, Idx, K, Self>,
        ) -> &SortedLinkedListView<T, K, Idx>
        where
            Self: SortedLinkedListStorage<T, Idx>,
            Idx: LenType,
        {
            this
        }
        fn as_mut_view<K>(
            this: &mut SortedLinkedListInner<T, Idx, K, Self>,
        ) -> &mut SortedLinkedListView<T, K, Idx>
        where
            Self: SortedLinkedListStorage<T, Idx>,
            Idx: LenType,
        {
            this
        }
    }
    impl<T, Idx, const N: usize> SortedLinkedListStorage<T, Idx>
        for OwnedSortedLinkedListStorage<T, Idx, N>
    {
    }

    impl<T, Idx> SortedLinkedListSealedStorage<T, Idx> for ViewSortedLinkedListStorage<T, Idx> {
        fn borrow(&self) -> &[Node<T, Idx>] {
            &self.buffer
        }
        fn borrow_mut(&mut self) -> &mut [Node<T, Idx>] {
            &mut self.buffer
        }
        fn as_view<K>(
            this: &SortedLinkedListInner<T, Idx, K, Self>,
        ) -> &SortedLinkedListView<T, K, Idx>
        where
            Self: SortedLinkedListStorage<T, Idx>,
            Idx: LenType,
        {
            this
        }
        fn as_mut_view<K>(
            this: &mut SortedLinkedListInner<T, Idx, K, Self>,
        ) -> &mut SortedLinkedListView<T, K, Idx>
        where
            Self: SortedLinkedListStorage<T, Idx>,
            Idx: LenType,
        {
            this
        }
    }
    impl<T, Idx> SortedLinkedListStorage<T, Idx> for ViewSortedLinkedListStorage<T, Idx> {}
}
pub use storage::{
    OwnedSortedLinkedListStorage, SortedLinkedListStorage, ViewSortedLinkedListStorage,
};

use crate::len_type::LenType;

/// Marker for Min sorted [`SortedLinkedList`].
pub struct Min;

/// Marker for Max sorted [`SortedLinkedList`].
pub struct Max;

/// The linked list kind: min-list or max-list
pub trait Kind: private::Sealed {
    #[doc(hidden)]
    fn ordering() -> Ordering;
}

impl Kind for Min {
    fn ordering() -> Ordering {
        Ordering::Less
    }
}

impl Kind for Max {
    fn ordering() -> Ordering {
        Ordering::Greater
    }
}

/// Sealed traits
mod private {
    pub trait Sealed {}
}

impl private::Sealed for Max {}
impl private::Sealed for Min {}

/// A node in the [`SortedLinkedList`].
#[cfg_attr(feature = "zeroize", derive(Zeroize))]
pub struct Node<T, Idx> {
    val: MaybeUninit<T>,
    next: Idx,
}

/// Base struct for [`SortedLinkedList`] and [`SortedLinkedListView`], generic over the
/// [`SortedLinkedListStorage`].
///
/// In most cases you should use [`SortedLinkedList`] or [`SortedLinkedListView`] directly. Only use
/// this struct if you want to write code that's generic over both.
pub struct SortedLinkedListInner<T, Idx, K, S>
where
    Idx: LenType,
    S: SortedLinkedListStorage<T, Idx> + ?Sized,
{
    head: Idx,
    free: Idx,
    phantom: PhantomData<(K, T)>,
    list: S,
}

/// The linked list.
pub type SortedLinkedList<T, K, const N: usize, Idx = usize> =
    SortedLinkedListInner<T, Idx, K, OwnedSortedLinkedListStorage<T, Idx, N>>;

/// The linked list.
pub type SortedLinkedListView<T, K, Idx> =
    SortedLinkedListInner<T, Idx, K, ViewSortedLinkedListStorage<T, Idx>>;

macro_rules! impl_const_new {
    ($ty:ty, $new_name:ident) => {
        impl<T, K, const N: usize> SortedLinkedList<T, K, N, $ty> {
            /// Create a new linked list.
            pub const fn $new_name() -> Self {
                const {
                    assert!(
                        (<$ty>::MAX as usize) >= (N + 1),
                        "The capacity is larger than `LenT` can hold, increase the size of `LenT` or reduce the capacity"
                    );
                }

                let mut list = SortedLinkedList {
                    list: OwnedSortedLinkedListStorage {
                        buffer: [const {
                            Node {
                                val: MaybeUninit::uninit(),
                                next: <$ty>::MAX,
                            }
                        }; N],
                    },
                    head: <$ty>::MAX,
                    free: 0,
                    phantom: PhantomData,
                };

                if N == 0 {
                    list.free = <$ty>::MAX;
                    return list;
                }

                let mut free = 0;

                // Initialize indexes
                while free < N - 1 {
                    list.list.buffer[free].next = free as $ty + 1;
                    free += 1;
                }

                list
            }
        }
    };
}

impl_const_new!(u8, new_u8);
impl_const_new!(u16, new_u16);
impl_const_new!(usize, new_usize);

impl<T, Idx, K, S> SortedLinkedListInner<T, Idx, K, S>
where
    Idx: LenType,
    S: SortedLinkedListStorage<T, Idx> + ?Sized,
{
    /// Get a reference to the `SortedLinkedList`, erasing the `N` const-generic.
    pub fn as_view(&self) -> &SortedLinkedListView<T, K, Idx> {
        S::as_view(self)
    }

    /// Get a mutable reference to the `Vec`, erasing the `N` const-generic.
    pub fn as_mut_view(&mut self) -> &mut SortedLinkedListView<T, K, Idx> {
        S::as_mut_view(self)
    }

    /// Internal access helper
    #[inline(always)]
    fn node_at(&self, index: usize) -> &Node<T, Idx> {
        // Safety: The entire `self.list` is initialized in `new`, which makes this safe.
        unsafe { self.list.borrow().get_unchecked(index) }
    }

    /// Internal access helper
    #[inline(always)]
    fn node_at_mut(&mut self, index: usize) -> &mut Node<T, Idx> {
        // Safety: The entire `self.list` is initialized in `new`, which makes this safe.
        unsafe { self.list.borrow_mut().get_unchecked_mut(index) }
    }

    /// Internal access helper
    #[inline(always)]
    fn write_data_in_node_at(&mut self, index: usize, data: T) {
        // Safety: The entire `self.list` is initialized in `new`, which makes this safe.
        unsafe {
            self.node_at_mut(index).val.as_mut_ptr().write(data);
        }
    }

    /// Internal access helper
    #[inline(always)]
    fn read_data_in_node_at(&self, index: usize) -> &T {
        // Safety: The entire `self.list` is initialized in `new`, which makes this safe.
        unsafe { &*self.node_at(index).val.as_ptr() }
    }

    /// Internal access helper
    #[inline(always)]
    fn read_mut_data_in_node_at(&mut self, index: usize) -> &mut T {
        // Safety: The entire `self.list` is initialized in `new`, which makes this safe.
        unsafe { &mut *self.node_at_mut(index).val.as_mut_ptr() }
    }

    /// Internal access helper
    #[inline(always)]
    fn extract_data_in_node_at(&mut self, index: usize) -> T {
        // Safety: The entire `self.list` is initialized in `new`, which makes this safe.
        unsafe { self.node_at(index).val.as_ptr().read() }
    }
}

impl<T, Idx, K, S> SortedLinkedListInner<T, Idx, K, S>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
    S: SortedLinkedListStorage<T, Idx> + ?Sized,
{
    /// Pushes a value onto the list without checking if the list is full.
    ///
    /// Complexity is worst-case *O*(n).
    ///
    /// # Safety
    ///
    /// Assumes that the list is not full.
    pub unsafe fn push_unchecked(&mut self, value: T) {
        let new = self.free.into_usize();

        // Store the data and update the next free spot
        self.write_data_in_node_at(new, value);
        self.free = self.node_at(new).next;

        if let Some(head) = self.head.to_non_max() {
            // Check if we need to replace head
            if self
                .read_data_in_node_at(head)
                .cmp(self.read_data_in_node_at(new))
                == K::ordering()
            {
                // It's not head, search the list for the correct placement
                let mut current = head;

                while let Some(next) = self.node_at(current).next.to_non_max() {
                    if self
                        .read_data_in_node_at(next)
                        .cmp(self.read_data_in_node_at(new))
                        != K::ordering()
                    {
                        break;
                    }

                    current = next;
                }

                self.node_at_mut(new).next = self.node_at(current).next;
                self.node_at_mut(current).next = Idx::from_usize(new);
            } else {
                self.node_at_mut(new).next = self.head;
                self.head = Idx::from_usize(new);
            }
        } else {
            self.node_at_mut(new).next = self.head;
            self.head = Idx::from_usize(new);
        }
    }

    /// Pushes an element to the linked list and sorts it into place.
    ///
    /// Complexity is worst-case *O*(n).
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, SortedLinkedList};
    /// let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// // The largest value will always be first
    /// ll.push(1).unwrap();
    /// assert_eq!(ll.peek(), Some(&1));
    ///
    /// ll.push(2).unwrap();
    /// assert_eq!(ll.peek(), Some(&2));
    ///
    /// ll.push(3).unwrap();
    /// assert_eq!(ll.peek(), Some(&3));
    ///
    /// // This will not fit in the queue.
    /// assert_eq!(ll.push(4), Err(4));
    /// ```
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            Err(value)
        } else {
            unsafe { self.push_unchecked(value) }
            Ok(())
        }
    }

    /// Peek at the first element.
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, Min, SortedLinkedList};
    /// let mut ll_max: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// // The largest value will always be first
    /// ll_max.push(1).unwrap();
    /// assert_eq!(ll_max.peek(), Some(&1));
    /// ll_max.push(2).unwrap();
    /// assert_eq!(ll_max.peek(), Some(&2));
    /// ll_max.push(3).unwrap();
    /// assert_eq!(ll_max.peek(), Some(&3));
    ///
    /// let mut ll_min: SortedLinkedList<_, Min, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// // The Smallest value will always be first
    /// ll_min.push(3).unwrap();
    /// assert_eq!(ll_min.peek(), Some(&3));
    /// ll_min.push(2).unwrap();
    /// assert_eq!(ll_min.peek(), Some(&2));
    /// ll_min.push(1).unwrap();
    /// assert_eq!(ll_min.peek(), Some(&1));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head
            .to_non_max()
            .map(|head| self.read_data_in_node_at(head))
    }

    /// Pop an element from the list without checking so the list is not empty.
    ///
    /// # Safety
    ///
    /// Assumes that the list is not empty.
    pub unsafe fn pop_unchecked(&mut self) -> T {
        let head = self.head.into_usize();
        let current = head;
        self.head = self.node_at(head).next;
        self.node_at_mut(current).next = self.free;
        self.free = Idx::from_usize(current);

        self.extract_data_in_node_at(current)
    }

    /// Pops the first element in the list.
    ///
    /// Complexity is worst-case *O*(1).
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, SortedLinkedList};
    /// let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// ll.push(1).unwrap();
    /// ll.push(2).unwrap();
    ///
    /// assert_eq!(ll.pop(), Some(2));
    /// assert_eq!(ll.pop(), Some(1));
    /// assert_eq!(ll.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    /// Checks if the linked list is full.
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, SortedLinkedList};
    /// let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// assert_eq!(ll.is_full(), false);
    ///
    /// ll.push(1).unwrap();
    /// assert_eq!(ll.is_full(), false);
    /// ll.push(2).unwrap();
    /// assert_eq!(ll.is_full(), false);
    /// ll.push(3).unwrap();
    /// assert_eq!(ll.is_full(), true);
    /// ```
    #[inline]
    pub fn is_full(&self) -> bool {
        self.free.to_non_max().is_none()
    }

    /// Checks if the linked list is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, SortedLinkedList};
    /// let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// assert_eq!(ll.is_empty(), true);
    ///
    /// ll.push(1).unwrap();
    /// assert_eq!(ll.is_empty(), false);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.to_non_max().is_none()
    }
}

impl<T, Idx, K, S> SortedLinkedListInner<T, Idx, K, S>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
    S: SortedLinkedListStorage<T, Idx> + ?Sized,
{
    /// Get an iterator over the sorted list.
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, SortedLinkedList};
    /// let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// ll.push(1).unwrap();
    /// ll.push(2).unwrap();
    ///
    /// let mut iter = ll.iter();
    ///
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> IterView<'_, T, Idx, K> {
        IterView {
            list: S::as_view(self),
            index: self.head,
        }
    }

    /// Find an element in the list that can be changed and resorted.
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, SortedLinkedList};
    /// let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// ll.push(1).unwrap();
    /// ll.push(2).unwrap();
    /// ll.push(3).unwrap();
    ///
    /// // Find a value and update it
    /// let mut find = ll.find_mut(|v| *v == 2).unwrap();
    /// *find += 1000;
    /// find.finish();
    ///
    /// assert_eq!(ll.pop(), Some(1002));
    /// assert_eq!(ll.pop(), Some(3));
    /// assert_eq!(ll.pop(), Some(1));
    /// assert_eq!(ll.pop(), None);
    /// ```
    pub fn find_mut<F>(&mut self, mut f: F) -> Option<FindMutView<'_, T, Idx, K>>
    where
        F: FnMut(&T) -> bool,
    {
        let head = self.head.to_non_max()?;

        // Special-case, first element
        if f(self.read_data_in_node_at(head)) {
            return Some(FindMutView {
                is_head: true,
                prev_index: Idx::MAX,
                index: self.head,
                list: S::as_mut_view(self),
                maybe_changed: false,
            });
        }

        let mut current = head;

        while let Some(next) = self.node_at(current).next.to_non_max() {
            if f(self.read_data_in_node_at(next)) {
                return Some(FindMutView {
                    is_head: false,
                    prev_index: Idx::from_usize(current),
                    index: Idx::from_usize(next),
                    list: S::as_mut_view(self),
                    maybe_changed: false,
                });
            }

            current = next;
        }

        None
    }
}

/// Iterator for the linked list.
pub struct IterView<'a, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
{
    list: &'a SortedLinkedListInner<T, Idx, K, ViewSortedLinkedListStorage<T, Idx>>,
    index: Idx,
}

impl<'a, T, Idx, K> Iterator for IterView<'a, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.to_non_max()?;

        let node = self.list.node_at(index);
        self.index = node.next;

        Some(self.list.read_data_in_node_at(index))
    }
}

/// Comes from [`SortedLinkedList::find_mut`].
pub struct FindMutView<'a, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
{
    list: &'a mut SortedLinkedListView<T, K, Idx>,
    is_head: bool,
    prev_index: Idx,
    index: Idx,
    maybe_changed: bool,
}

impl<T, Idx, K> FindMutView<'_, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
{
    fn pop_internal(&mut self) -> T {
        if self.is_head {
            // If it is the head element, we can do a normal pop
            unsafe { self.list.pop_unchecked() }
        } else {
            // Somewhere in the list
            let prev = self.prev_index.into_usize();
            let curr = self.index.into_usize();

            // Re-point the previous index
            self.list.node_at_mut(prev).next = self.list.node_at_mut(curr).next;

            // Release the index into the free queue
            self.list.node_at_mut(curr).next = self.list.free;
            self.list.free = self.index;

            self.list.extract_data_in_node_at(curr)
        }
    }

    /// This will pop the element from the list.
    ///
    /// Complexity is worst-case *O*(1).
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, SortedLinkedList};
    /// let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// ll.push(1).unwrap();
    /// ll.push(2).unwrap();
    /// ll.push(3).unwrap();
    ///
    /// // Find a value and update it
    /// let mut find = ll.find_mut(|v| *v == 2).unwrap();
    /// find.pop();
    ///
    /// assert_eq!(ll.pop(), Some(3));
    /// assert_eq!(ll.pop(), Some(1));
    /// assert_eq!(ll.pop(), None);
    /// ```
    #[inline]
    pub fn pop(mut self) -> T {
        self.pop_internal()
    }

    /// This will resort the element into the correct position in the list if needed. The resorting
    /// will only happen if the element has been accessed mutably.
    ///
    /// Same as calling `drop`.
    ///
    /// Complexity is worst-case *O*(n).
    ///
    /// # Example
    ///
    /// ```
    /// use heapless::sorted_linked_list::{Max, SortedLinkedList};
    /// let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();
    ///
    /// ll.push(1).unwrap();
    /// ll.push(2).unwrap();
    /// ll.push(3).unwrap();
    ///
    /// let mut find = ll.find_mut(|v| *v == 2).unwrap();
    /// find.finish(); // No resort, we did not access the value.
    ///
    /// let mut find = ll.find_mut(|v| *v == 2).unwrap();
    /// *find += 1000;
    /// find.finish(); // Will resort, we accessed (and updated) the value.
    ///
    /// assert_eq!(ll.pop(), Some(1002));
    /// assert_eq!(ll.pop(), Some(3));
    /// assert_eq!(ll.pop(), Some(1));
    /// assert_eq!(ll.pop(), None);
    /// ```
    #[inline]
    pub fn finish(self) {
        drop(self);
    }
}

impl<T, Idx, K> Drop for FindMutView<'_, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
{
    fn drop(&mut self) {
        // Only resort the list if the element has changed
        if self.maybe_changed {
            let val = self.pop_internal();
            unsafe { self.list.push_unchecked(val) };
        }
    }
}

impl<T, Idx, K> Deref for FindMutView<'_, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.list.read_data_in_node_at(self.index.into_usize())
    }
}

impl<T, Idx, K> DerefMut for FindMutView<'_, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.maybe_changed = true;
        self.list.read_mut_data_in_node_at(self.index.into_usize())
    }
}

// /// Useful for debug during development.
// impl<T, Idx, K, const N: usize> fmt::Debug for FindMut<'_, T, Idx, K, N>
// where
//     T: Ord + core::fmt::Debug,
//     Idx: LenType,
//     K: Kind,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("FindMut")
//             .field("prev_index", &self.prev_index.to_non_max())
//             .field("index", &self.index.to_non_max())
//             .field(
//                 "prev_value",
//                 &self
//                     .list
//                     .read_data_in_node_at(self.prev_index.to_non_max().unwrap()),
//             )
//             .field(
//                 "value",
//                 &self.list.read_data_in_node_at(self.index.to_non_max().unwrap()),
//             )
//             .finish()
//     }
// }

impl<T, Idx, K, S> fmt::Debug for SortedLinkedListInner<T, Idx, K, S>
where
    T: Ord + core::fmt::Debug,
    Idx: LenType,
    K: Kind,
    S: ?Sized + SortedLinkedListStorage<T, Idx>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, Idx, K, S> Drop for SortedLinkedListInner<T, Idx, K, S>
where
    Idx: LenType,
    S: SortedLinkedListStorage<T, Idx> + ?Sized,
{
    fn drop(&mut self) {
        let mut index = self.head;

        while let Some(i) = index.to_non_max() {
            let node = self.node_at_mut(i);
            index = node.next;

            unsafe {
                ptr::drop_in_place(node.val.as_mut_ptr());
            }
        }
    }
}

#[cfg(feature = "zeroize")]
impl<T, Idx, K, S> Zeroize for SortedLinkedListInner<T, Idx, K, S>
where
    T: Ord + Zeroize,
    Idx: LenType + Zeroize,
    K: Kind,
    S: SortedLinkedListStorage<T, Idx> + ?Sized,
{
    fn zeroize(&mut self) {
        while let Some(mut item) = self.pop() {
            item.zeroize();
        }

        let buffer = self.list.borrow_mut();
        for elem in buffer {
            elem.zeroize();
        }
    }
}

#[cfg(test)]
mod tests {
    use static_assertions::assert_not_impl_any;

    use super::*;

    // Ensure a `SortedLinkedList` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(SortedLinkedList<*const (), (), 4>: Send);

    #[test]
    fn const_new() {
        static mut _V1: SortedLinkedList<u32, Max, 100, u8> = SortedLinkedList::new_u8();
        static mut _V2: SortedLinkedList<u32, Max, 10_000, u16> = SortedLinkedList::new_u16();
        static mut _V3: SortedLinkedList<u32, Max, 100_000, usize> = SortedLinkedList::new_usize();
    }

    #[test]
    fn test_peek() {
        let mut ll: SortedLinkedList<u32, Max, 3, u8> = SortedLinkedList::new_u8();

        ll.push(1).unwrap();
        assert_eq!(ll.peek().unwrap(), &1);

        ll.push(2).unwrap();
        assert_eq!(ll.peek().unwrap(), &2);

        ll.push(3).unwrap();
        assert_eq!(ll.peek().unwrap(), &3);

        let mut ll: SortedLinkedList<u32, Min, 3, u8> = SortedLinkedList::new_u8();

        ll.push(2).unwrap();
        assert_eq!(ll.peek().unwrap(), &2);

        ll.push(1).unwrap();
        assert_eq!(ll.peek().unwrap(), &1);

        ll.push(3).unwrap();
        assert_eq!(ll.peek().unwrap(), &1);
    }

    #[test]
    fn test_full() {
        let mut ll: SortedLinkedList<u32, Max, 3, u8> = SortedLinkedList::new_u8();
        ll.push(1).unwrap();
        ll.push(2).unwrap();
        ll.push(3).unwrap();

        assert!(ll.is_full());
    }

    #[test]
    fn test_empty() {
        let ll: SortedLinkedList<u32, Max, 3, u8> = SortedLinkedList::new_u8();

        assert!(ll.is_empty());
    }

    #[test]
    fn test_zero_size() {
        let ll: SortedLinkedList<u32, Max, 0, u8> = SortedLinkedList::new_u8();

        assert!(ll.is_empty());
        assert!(ll.is_full());
    }

    #[test]
    fn test_rejected_push() {
        let mut ll: SortedLinkedList<u32, Max, 3, u8> = SortedLinkedList::new_u8();
        ll.push(1).unwrap();
        ll.push(2).unwrap();
        ll.push(3).unwrap();

        // This won't fit
        let r = ll.push(4);

        assert_eq!(r, Err(4));
    }

    #[test]
    fn test_updating() {
        let mut ll: SortedLinkedList<u32, Max, 3, u8> = SortedLinkedList::new_u8();
        ll.push(1).unwrap();
        ll.push(2).unwrap();
        ll.push(3).unwrap();

        let mut find = ll.find_mut(|v| *v == 2).unwrap();

        *find += 1000;
        find.finish();

        assert_eq!(ll.peek().unwrap(), &1002);

        let mut find = ll.find_mut(|v| *v == 3).unwrap();

        *find += 1000;
        find.finish();

        assert_eq!(ll.peek().unwrap(), &1003);

        // Remove largest element
        ll.find_mut(|v| *v == 1003).unwrap().pop();

        assert_eq!(ll.peek().unwrap(), &1002);
    }

    #[test]
    fn test_updating_1() {
        let mut ll: SortedLinkedList<u32, Max, 3, u8> = SortedLinkedList::new_u8();
        ll.push(1).unwrap();

        let v = ll.pop().unwrap();

        assert_eq!(v, 1);
    }

    #[test]
    fn test_updating_2() {
        let mut ll: SortedLinkedList<u32, Max, 3, u8> = SortedLinkedList::new_u8();
        ll.push(1).unwrap();

        let mut find = ll.find_mut(|v| *v == 1).unwrap();

        *find += 1000;
        find.finish();

        assert_eq!(ll.peek().unwrap(), &1001);
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_sorted_linked_list_zeroize() {
        use zeroize::Zeroize;

        let mut list: SortedLinkedList<u8, Max, 8, u8> = SortedLinkedList::new_u8();
        for i in 1..=8 {
            list.push(i).unwrap();
        }

        assert_eq!(list.is_empty(), false);
        assert!(list.is_full());
        assert_eq!(list.peek(), Some(&8));

        list.pop();
        list.pop();
        list.push(100).unwrap();

        assert_eq!(list.peek(), Some(&100));

        list.zeroize();

        assert_eq!(list.peek(), None);
        assert!(list.is_empty());

        unsafe {
            for node in &list.list.buffer {
                assert_eq!(node.val.assume_init(), 0);
            }
        }
    }

    fn _test_variance<'a: 'b, 'b>(
        x: SortedLinkedList<&'a (), Max, 42, u8>,
    ) -> SortedLinkedList<&'b (), Max, 42, u8> {
        x
    }
    fn _test_variance_view<'a: 'b, 'b, 'c>(
        x: &'c SortedLinkedListView<&'a (), Max, u8>,
    ) -> &'c SortedLinkedListView<&'b (), Max, u8> {
        x
    }
}

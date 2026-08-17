//! [`LinkedList`] is a trait that implements lock-free concurrent singly linked list operations.

use std::fmt::{self, Debug, Display};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering::{self, AcqRel, Acquire, Relaxed};

use super::{AtomicShared, Guard, Ptr, Shared, Tag};

/// [`LinkedList`] is a trait that implements lock-free singly linked list operations.
pub trait LinkedList: Sized {
    /// Returns a reference to the forward link.
    ///
    /// The pointer value may be tagged if [`Self::mark`] or [`Self::delete_self`] has been
    /// invoked. The [`AtomicShared`] must only be updated through [`LinkedList`] in order to keep
    /// the linked list consistent.
    fn link_ref(&self) -> &AtomicShared<Self>;

    /// Returns `true` if `self` is reachable and not marked.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use sdd::{AtomicShared, LinkedList, Tag};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let head: L = L::default();
    /// assert!(head.is_clear(Relaxed));
    /// assert!(head.mark(Relaxed));
    /// assert!(!head.is_clear(Relaxed));
    /// assert!(head.delete_self(Relaxed));
    /// assert!(!head.is_clear(Relaxed));
    /// ```
    #[inline]
    fn is_clear(&self, order: Ordering) -> bool {
        self.link_ref().tag(order) == Tag::None
    }

    /// Marks `self` with an internal flag to denote that `self` is in a special state.
    ///
    /// Returns `false` if a flag has already been set on `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use sdd::{AtomicShared, LinkedList};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let head: L = L::default();
    /// assert!(head.mark(Relaxed));
    /// ```
    #[inline]
    fn mark(&self, order: Ordering) -> bool {
        self.link_ref()
            .update_tag_if(Tag::First, |ptr| ptr.tag() == Tag::None, order, Relaxed)
    }

    /// Removes any mark from `self`.
    ///
    /// Returns `false` if no flag has been set on `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use sdd::{AtomicShared, LinkedList};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let head: L = L::default();
    /// assert!(!head.unmark(Relaxed));
    /// assert!(head.mark(Relaxed));
    /// assert!(head.unmark(Relaxed));
    /// assert!(!head.is_marked(Relaxed));
    /// ```
    #[inline]
    fn unmark(&self, order: Ordering) -> bool {
        self.link_ref()
            .update_tag_if(Tag::None, |ptr| ptr.tag() == Tag::First, order, Relaxed)
    }

    /// Returns `true` if `self` has a mark on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use sdd::{AtomicShared, LinkedList};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let head: L = L::default();
    /// assert!(!head.is_marked(Relaxed));
    /// assert!(head.mark(Relaxed));
    /// assert!(head.is_marked(Relaxed));
    /// ```
    #[inline]
    fn is_marked(&self, order: Ordering) -> bool {
        self.link_ref().tag(order) == Tag::First
    }

    /// Deletes `self`.
    ///
    /// Returns `false` if `self` is already marked as deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use sdd::{AtomicShared, Guard, LinkedList, Shared};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let guard = Guard::new();
    ///
    /// let head: L = L::default();
    /// let tail: Shared<L> = Shared::new(L::default());
    /// assert!(head.push_back(tail.clone(), false, Relaxed, &guard).is_ok());
    ///
    /// tail.delete_self(Relaxed);
    /// assert!(head.next_ptr(Relaxed, &guard).as_ref().is_none());
    /// ```
    #[inline]
    fn delete_self(&self, order: Ordering) -> bool {
        self.link_ref().update_tag_if(
            Tag::Second,
            |ptr| {
                let tag = ptr.tag();
                tag == Tag::None || tag == Tag::First
            },
            order,
            Relaxed,
        )
    }

    /// Returns `true` if `self` has been deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use sdd::{AtomicShared, LinkedList};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let entry: L = L::default();
    /// assert!(!entry.is_deleted(Relaxed));
    /// entry.delete_self(Relaxed);
    /// assert!(entry.is_deleted(Relaxed));
    /// ```
    #[inline]
    fn is_deleted(&self, order: Ordering) -> bool {
        let tag = self.link_ref().tag(order);
        tag == Tag::Second || tag == Tag::Both
    }

    /// Appends the given entry to `self` and returns a pointer to the entry.
    ///
    /// If `mark` is `true`, it atomically sets an internal flag on `self` when updating
    /// the linked list, otherwise it removes the mark.
    ///
    /// # Errors
    ///
    /// Returns the supplied [`Shared`] when it finds `self` deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::{Relaxed, Release};
    ///
    /// use sdd::{AtomicShared, Guard, LinkedList, Shared};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let guard = Guard::new();
    ///
    /// let head: L = L::default();
    /// assert!(head.push_back(Shared::new(L::default()), true, Release, &guard).is_ok());
    /// assert!(head.is_marked(Relaxed));
    /// assert!(head.push_back(Shared::new(L::default()), false, Release, &guard).is_ok());
    /// assert!(!head.is_marked(Relaxed));
    ///
    /// head.delete_self(Relaxed);
    /// assert!(!head.is_marked(Relaxed));
    /// assert!(head.push_back(Shared::new(L::default()), false, Release, &guard).is_err());
    /// ```
    #[inline]
    fn push_back<'g>(
        &self,
        mut entry: Shared<Self>,
        mark: bool,
        order: Ordering,
        guard: &'g Guard,
    ) -> Result<Ptr<'g, Self>, Shared<Self>> {
        let new_tag = if mark { Tag::First } else { Tag::None };
        let mut next_ptr = self.link_ref().load(Relaxed, guard);

        loop {
            let tag = next_ptr.tag();
            if tag == Tag::Second || tag == Tag::Both {
                break;
            }

            entry
                .link_ref()
                .swap((next_ptr.get_shared(), Tag::None), Relaxed);
            match self.link_ref().compare_exchange_weak(
                next_ptr,
                (Some(entry), new_tag),
                order,
                Relaxed,
                guard,
            ) {
                Ok((_, updated)) => {
                    return Ok(updated);
                }
                Err((passed, actual)) => {
                    entry = unsafe { passed.unwrap_unchecked() };
                    next_ptr = actual;
                }
            }
        }

        // `current` has been deleted.
        Err(entry)
    }

    /// Returns the closest next valid entry.
    ///
    /// It unlinks deleted entries until it reaches a valid one.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
    ///
    /// use sdd::{AtomicShared, Guard, LinkedList, Shared};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let guard = Guard::new();
    ///
    /// let head: L = L::default();
    /// assert!(
    ///     head.push_back(Shared::new(L(AtomicShared::null(), 1)), false, Release, &guard).is_ok());
    /// head.mark(Relaxed);
    ///
    /// let next_ptr = head.next_ptr(Acquire, &guard);
    /// assert_eq!(next_ptr.as_ref().unwrap().1, 1);
    /// assert!(head.is_marked(Relaxed));
    /// ```
    #[inline]
    fn next_ptr<'g>(&self, order: Ordering, guard: &'g Guard) -> Ptr<'g, Self> {
        next_ptr_recursive(self, order, 64, guard)
    }

    /// Returns a [`Shared`] handle to the closest next valid entry.
    ///
    /// It unlinks deleted entries until it reaches a valid one.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
    ///
    /// use sdd::{AtomicShared, Guard, LinkedList, Shared};
    ///
    /// #[derive(Default)]
    /// struct L(AtomicShared<L>, usize);
    /// impl LinkedList for L {
    ///     fn link_ref(&self) -> &AtomicShared<L> {
    ///         &self.0
    ///     }
    /// }
    ///
    /// let guard = Guard::new();
    ///
    /// let head: L = L::default();
    /// assert!(
    ///     head.push_back(Shared::new(L(AtomicShared::null(), 1)), false, Release, &guard).is_ok());
    /// head.mark(Relaxed);
    ///
    /// let next_shared = head.next_shared(Acquire, &guard);
    /// assert_eq!(next_shared.unwrap().1, 1);
    /// assert!(head.is_marked(Relaxed));
    /// ```
    #[inline]
    fn next_shared(&self, order: Ordering, guard: &Guard) -> Option<Shared<Self>> {
        let mut next_ptr = self.next_ptr(order, guard);
        let mut next_entry = next_ptr.get_shared();
        while !next_ptr.is_null() && next_entry.is_none() {
            // The entry was released in the mean time.
            next_ptr = next_ptr
                .as_ref()
                .map_or_else(Ptr::null, |n| n.next_ptr(Acquire, guard));
            next_entry = next_ptr.get_shared();
        }
        next_entry
    }
}

/// [`LinkedEntry`] stores an instance of `T` and a link to the next entry.
pub struct LinkedEntry<T> {
    /// `instance` is always `Some` unless [`Self::take_inner`] is called.
    instance: Option<T>,
    /// `next` points to the next entry in a linked list.
    next: AtomicShared<Self>,
}

impl<T> LinkedEntry<T> {
    /// Extracts the inner instance of `T`.
    ///
    /// # Safety
    ///
    /// This method has to be called at most once per [`LinkedEntry`], and the caller needs to make
    /// sure that the [`LinkedEntry`] is not accessed via [`LinkedList`] methods.
    #[inline]
    pub(crate) unsafe fn take_inner(&mut self) -> T {
        unsafe { self.instance.take().unwrap_unchecked() }
    }

    #[inline]
    pub(super) fn new(val: T) -> Self {
        Self {
            instance: Some(val),
            next: AtomicShared::default(),
        }
    }

    /// Returns a reference to `next`.
    #[inline]
    pub(super) fn next(&self) -> &AtomicShared<Self> {
        &self.next
    }
}

impl<T> AsRef<T> for LinkedEntry<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        unsafe { self.instance.as_ref().unwrap_unchecked() }
    }
}

impl<T> AsMut<T> for LinkedEntry<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.instance.as_mut().unwrap_unchecked() }
    }
}

impl<T: Clone> Clone for LinkedEntry<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            instance: self.instance.clone(),
            next: AtomicShared::default(),
        }
    }
}

impl<T: Debug> Debug for LinkedEntry<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("instance", &self.instance)
            .field("next", &self.next)
            .field("marked", &self.is_marked(Relaxed))
            .field("removed", &self.is_deleted(Relaxed))
            .finish()
    }
}

impl<T> Deref for LinkedEntry<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.instance.as_ref().unwrap_unchecked() }
    }
}

impl<T> DerefMut for LinkedEntry<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.instance.as_mut().unwrap_unchecked() }
    }
}

impl<T> Drop for LinkedEntry<T> {
    #[inline]
    fn drop(&mut self) {
        if !self.next.is_null(Relaxed) {
            let guard = Guard::new();
            if let Some(next_entry) = self.next.load(Relaxed, &guard).as_ref() {
                next_ptr_recursive(next_entry, Relaxed, 64, &guard);
            }
        }
    }
}

impl<T: Display> Display for LinkedEntry<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(instance) = self.instance.as_ref() {
            write!(f, "Some({instance})")
        } else {
            write!(f, "None")
        }
    }
}

impl<T: Eq> Eq for LinkedEntry<T> {}

impl<T> LinkedList for LinkedEntry<T> {
    #[inline]
    fn link_ref(&self) -> &AtomicShared<Self> {
        &self.next
    }
}

impl<T: PartialEq> PartialEq for LinkedEntry<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.instance == other.instance
    }
}

/// Recursively cleans up the linked list starting from the supplied head.
fn next_ptr_recursive<'g, T: LinkedList>(
    head: &T,
    order: Ordering,
    mut depth: usize,
    guard: &'g Guard,
) -> Ptr<'g, T> {
    let mut head_next_ptr = head.link_ref().load(order, guard);
    let mut head_tag = head_next_ptr.tag();
    loop {
        let mut next_ptr = head_next_ptr;
        let next_valid_ptr = loop {
            if let Some(next_ref) = next_ptr.as_ref() {
                let next_next_ptr = next_ref.link_ref().load(order, guard);
                if next_next_ptr.tag() != Tag::Second {
                    break next_ptr;
                }
                if depth != 0 {
                    break next_ptr_recursive(next_ref, order, depth - 1, guard);
                }
                next_ptr = next_next_ptr;
            } else {
                break Ptr::null();
            }
        };

        // Do not allow a recursive call more than once.
        depth = 0;

        // Updates its link if there is an invalid entry between itself and the next valid one.
        if next_valid_ptr.with_tag(head_tag) != head_next_ptr {
            let next_valid_entry = next_valid_ptr.get_shared();
            if !next_valid_ptr.is_null() && next_valid_entry.is_none() {
                // The entry was unlinked in the meantime.
                head_next_ptr = head.link_ref().load(order, guard);
                head_tag = head_next_ptr.tag();
                continue;
            }
            match head.link_ref().compare_exchange(
                head_next_ptr,
                (next_valid_entry, head_tag),
                AcqRel,
                Acquire,
                guard,
            ) {
                Ok((prev, _)) => {
                    if let Some(removed) = prev {
                        let _: bool = removed.release();
                    }
                }
                Err((_, actual)) => {
                    head_next_ptr = actual;
                    head_tag = actual.tag();
                    continue;
                }
            }
        }

        return next_valid_ptr;
    }
}

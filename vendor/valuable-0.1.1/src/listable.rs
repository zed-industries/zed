use crate::*;

use core::fmt;

/// A list-like [`Valuable`] sub-type.
///
/// Implemented by [`Valuable`] types that have a list-like shape. This includes
/// [`Vec`] and other Rust [collection] types. `Listable` types may or may not
/// store items in contiguous memory. Any type that implements [`IntoIterator`]
/// may implement `Listable`. Values that implement `Listable` must return
/// [`Value::Listable`] from their [`Valuable::as_value`] implementation.
///
/// [collection]: https://doc.rust-lang.org/stable/std/collections/index.html
///
/// # Inspecting
///
/// Inspecting `Listable` items is done by visiting the collection. When
/// visiting a `Listable`, contained values are either passed one-by-one by
/// repeatedly calling [`visit_value()`] or all at once by calling
/// [`visit_primitive_slice()`]. The [`visit_primitive_slice()`] method has
/// lower overhead but can only be used when the `Listable` type contains
/// primitive values.
///
/// See [`Visit`] documentation for more details.
///
/// # Implementing
///
/// If the type stores values in slices internally, then those slices are passed
/// to [`Valuable::visit_slice`], which handles calling
/// [`visit_primitive_slice()`] if possible.
///
/// [`visit_value()`]: Visit::visit_value
/// [`visit_primitive_slice()`]: Visit::visit_primitive_slice
///
/// ```
/// use valuable::{Listable, Valuable, Value, Visit};
///
/// struct MyCollection<T> {
///     chunks: Vec<Vec<T>>,
/// }
///
/// impl<T: Valuable> Valuable for MyCollection<T> {
///     fn as_value(&self) -> Value<'_> {
///         Value::Listable(self)
///     }
///
///     fn visit(&self, visit: &mut dyn Visit) {
///         for chunk in &self.chunks {
///             // Handles visiting the slice
///             Valuable::visit_slice(chunk, visit);
///         }
///     }
/// }
///
/// impl<T: Valuable> Listable for MyCollection<T> {
///     fn size_hint(&self) -> (usize, Option<usize>) {
///         let len = self.chunks.iter().map(|chunk| chunk.len()).sum();
///         (len, Some(len))
///     }
/// }
/// ```
pub trait Listable: Valuable {
    /// Returns the bounds on the remaining length of the `Listable`.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element
    /// is the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an [`Option`]`<`[`usize`]`>`.
    /// A [`None`] here means that either there is no known upper bound, or the
    /// upper bound is larger than [`usize`].
    ///
    /// # Implementation notes
    ///
    /// It is not enforced that a `Listable` implementation yields the declared
    /// number of elements. A buggy iterator may yield less than the lower bound
    /// or more than the upper bound of elements.
    ///
    /// `size_hint()` is primarily intended to be used for optimizations such as
    /// reserving space for the elements of the `Listable`, but must not be
    /// trusted to e.g., omit bounds checks in unsafe code. An incorrect
    /// implementation of `size_hint()` should not lead to memory safety
    /// violations.
    ///
    /// That said, the implementation should provide a correct estimation,
    /// because otherwise it would be a violation of the trait's protocol.
    ///
    /// [`usize`]: type@usize
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use valuable::Listable;
    ///
    /// let a = vec![1, 2, 3];
    ///
    /// assert_eq!((3, Some(3)), a.size_hint());
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>);
}

macro_rules! deref {
    (
        $(
            $(#[$attrs:meta])*
            $ty:ty,
        )*
    ) => {
        $(
            $(#[$attrs])*
            impl<T: ?Sized + Listable> Listable for $ty {
                fn size_hint(&self) -> (usize, Option<usize>) {
                    T::size_hint(&**self)
                }
            }
        )*
    };
}

deref! {
    &T,
    &mut T,
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<T>,
    #[cfg(feature = "alloc")]
    alloc::rc::Rc<T>,
    #[cfg(not(valuable_no_atomic_cas))]
    #[cfg(feature = "alloc")]
    alloc::sync::Arc<T>,
}

macro_rules! slice {
    (
        $(
            $(#[$attrs:meta])*
            ($($generics:tt)*) $ty:ty,
        )*
    ) => {
        $(
            $(#[$attrs])*
            impl<$($generics)*> Valuable for $ty {
                fn as_value(&self) -> Value<'_> {
                    Value::Listable(self as &dyn Listable)
                }

                fn visit(&self, visit: &mut dyn Visit) {
                    T::visit_slice(self, visit);
                }
            }

            $(#[$attrs])*
            impl<$($generics)*> Listable for $ty {
                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.len(), Some(self.len()))
                }
            }
        )*
    };
}

slice! {
    (T: Valuable) &'_ [T],
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::boxed::Box<[T]>,
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::rc::Rc<[T]>,
    #[cfg(not(valuable_no_atomic_cas))]
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::sync::Arc<[T]>,
    (T: Valuable, const N: usize) [T; N],
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::vec::Vec<T>,
}

macro_rules! collection {
    (
        $(
            $(#[$attrs:meta])*
            ($($generics:tt)*) $ty:ty,
        )*
    ) => {
        $(
            $(#[$attrs])*
            impl<$($generics)*> Valuable for $ty {
                fn as_value(&self) -> Value<'_> {
                    Value::Listable(self as &dyn Listable)
                }

                fn visit(&self, visit: &mut dyn Visit) {
                    for value in self.iter() {
                        visit.visit_value(value.as_value());
                    }
                }
            }

            $(#[$attrs])*
            impl<$($generics)*> Listable for $ty {
                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.len(), Some(self.len()))
                }
            }
        )*
    };
}

collection! {
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::collections::LinkedList<T>,
    #[cfg(feature = "alloc")]
    (T: Valuable + Ord) alloc::collections::BinaryHeap<T>,
    #[cfg(feature = "alloc")]
    (T: Valuable + Ord) alloc::collections::BTreeSet<T>,
    #[cfg(feature = "std")]
    (T: Valuable + Eq + std::hash::Hash, H: std::hash::BuildHasher) std::collections::HashSet<T, H>,
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Valuable for alloc::collections::VecDeque<T> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self as &dyn Listable)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        let (first, second) = self.as_slices();
        T::visit_slice(first, visit);
        T::visit_slice(second, visit);
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Listable for alloc::collections::VecDeque<T> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl fmt::Debug for dyn Listable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugListable<'a, 'b> {
            fmt: fmt::DebugList<'a, 'b>,
        }

        impl Visit for DebugListable<'_, '_> {
            fn visit_value(&mut self, value: Value<'_>) {
                self.fmt.entry(&value);
            }
        }

        let mut debug = DebugListable {
            fmt: fmt.debug_list(),
        };

        self.visit(&mut debug);
        debug.fmt.finish()
    }
}

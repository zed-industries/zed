use crate::*;

use core::fmt;

/// A map-like [`Valuable`] sub-type.
///
/// Implemented by [`Valuable`] types that have a map-like shape. This includes
/// [`HashMap`] and other Rust [collection] types. Values that implement
/// `Mappable` must return [`Value::Mappable`] from their [`Value::as_value`]
/// implementation.
///
/// [collection]: https://doc.rust-lang.org/stable/std/collections/index.html
///
/// # Inspecting
///
/// Inspecting `Mappable` entries is done by visiting the value. When visiting a
/// `Mappable`, contained entries are passed one-by-one to the visitor by
/// repeatedly calling [`visit_entry()`].
///
/// See [`Visit`] documentation for more details.
///
/// [`visit_entry()`]: Visit::visit_entry
/// [`HashMap`]: std::collections::HashMap
///
/// # Implementing
///
/// Implementing `Mappable` for a custom map type. The map is represented using
/// a `Vec` of key/value pairs.
///
/// ```
/// use valuable::{Mappable, Valuable, Value, Visit};
///
/// struct MyMap<K, V> {
///     entries: Vec<(K, V)>,
/// }
///
/// impl<K: Valuable, V: Valuable> Valuable for MyMap<K, V> {
///     fn as_value(&self) -> Value<'_> {
///         Value::Mappable(self)
///     }
///
///     fn visit(&self, visit: &mut dyn Visit) {
///         for (k, v) in &self.entries {
///             visit.visit_entry(k.as_value(), v.as_value());
///         }
///     }
/// }
///
/// impl<K: Valuable, V: Valuable> Mappable for MyMap<K, V> {
///     fn size_hint(&self) -> (usize, Option<usize>) {
///         let len = self.entries.len();
///         (len, Some(len))
///     }
/// }
/// ```
pub trait Mappable: Valuable {
    /// Returns the bounds on the remaining length of the `Mappable`.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element is
    /// the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an
    /// [`Option`]`<`[`usize`]`>`. A [`None`] here means that either there is no
    /// known upper bound, or the upper bound is larger than [`usize`].
    ///
    /// # Implementation notes
    ///
    /// It is not enforced that a `Mappable` implementation yields the declared
    /// number of elements. A buggy implementation may yield less than the lower
    /// bound or more than the upper bound of elements.
    ///
    /// `size_hint()` is primarily intended to be used for optimizations such as
    /// reserving space for the elements of the `Mappable`, but must not be
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
    /// use valuable::Mappable;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("one", 1);
    /// map.insert("two", 2);
    /// map.insert("three", 3);
    ///
    /// assert_eq!((3, Some(3)), map.size_hint());
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
            impl<T: ?Sized + Mappable> Mappable for $ty {
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

#[cfg(feature = "std")]
impl<K: Valuable, V: Valuable, S> Valuable for std::collections::HashMap<K, V, S> {
    fn as_value(&self) -> Value<'_> {
        Value::Mappable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for (key, value) in self.iter() {
            visit.visit_entry(key.as_value(), value.as_value());
        }
    }
}

#[cfg(feature = "std")]
impl<K: Valuable, V: Valuable, S> Mappable for std::collections::HashMap<K, V, S> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter().size_hint()
    }
}

#[cfg(feature = "alloc")]
impl<K: Valuable, V: Valuable> Valuable for alloc::collections::BTreeMap<K, V> {
    fn as_value(&self) -> Value<'_> {
        Value::Mappable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for (key, value) in self.iter() {
            visit.visit_entry(key.as_value(), value.as_value());
        }
    }
}

#[cfg(feature = "alloc")]
impl<K: Valuable, V: Valuable> Mappable for alloc::collections::BTreeMap<K, V> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter().size_hint()
    }
}

impl fmt::Debug for dyn Mappable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugMappable<'a, 'b> {
            fmt: fmt::DebugMap<'a, 'b>,
        }

        impl Visit for DebugMappable<'_, '_> {
            fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
                self.fmt.entry(&key, &value);
            }

            fn visit_value(&mut self, _: Value<'_>) {}
        }

        let mut debug = DebugMappable {
            fmt: fmt.debug_map(),
        };
        self.visit(&mut debug);
        debug.fmt.finish()
    }
}

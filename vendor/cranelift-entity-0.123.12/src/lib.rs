//! Array-based data structures using densely numbered entity references as mapping keys.
//!
//! This crate defines a number of data structures based on arrays. The arrays are not indexed by
//! `usize` as usual, but by *entity references* which are integers wrapped in new-types. This has
//! a couple advantages:
//!
//! - Improved type safety. The various map and set types accept a specific key type, so there is
//!   no confusion about the meaning of an array index, as there is with plain arrays.
//! - Smaller indexes. The normal `usize` index is often 64 bits which is way too large for most
//!   purposes. The entity reference types can be smaller, allowing for more compact data
//!   structures.
//!
//! The `EntityRef` trait should be implemented by types to be used as indexed. The `entity_impl!`
//! macro provides convenient defaults for types wrapping `u32` which is common.
//!
//! - [`PrimaryMap`](struct.PrimaryMap.html) is used to keep track of a vector of entities,
//!   assigning a unique entity reference to each.
//! - [`SecondaryMap`](struct.SecondaryMap.html) is used to associate secondary information to an
//!   entity. The map is implemented as a simple vector, so it does not keep track of which
//!   entities have been inserted. Instead, any unknown entities map to the default value.
//! - [`SparseMap`](struct.SparseMap.html) is used to associate secondary information to a small
//!   number of entities. It tracks accurately which entities have been inserted. This is a
//!   specialized data structure which can use a lot of memory, so read the documentation before
//!   using it.
//! - [`EntitySet`](struct.EntitySet.html) is used to represent a secondary set of entities.
//!   The set is implemented as a simple vector, so it does not keep track of which entities have
//!   been inserted into the primary map. Instead, any unknown entities are not in the set.
//! - [`EntityList`](struct.EntityList.html) is a compact representation of lists of entity
//!   references allocated from an associated memory pool. It has a much smaller footprint than
//!   `Vec`.

#![deny(missing_docs)]
#![no_std]

extern crate alloc;

// Re-export core so that the macros works with both std and no_std crates
#[doc(hidden)]
pub extern crate core as __core;

use core::iter::FusedIterator;
use core::ops::Range;

/// A type wrapping a small integer index should implement `EntityRef` so it can be used as the key
/// of an `SecondaryMap` or `SparseMap`.
pub trait EntityRef: Copy + Eq {
    /// Create a new entity reference from a small integer.
    /// This should crash if the requested index is not representable.
    fn new(_: usize) -> Self;

    /// Get the index that was used to create this entity reference.
    fn index(self) -> usize;
}

/// Iterate over a `Range<E: EntityRef>`, yielding a sequence of `E` items.
#[inline]
pub fn iter_entity_range<E>(range: Range<E>) -> IterEntityRange<E>
where
    E: EntityRef,
{
    IterEntityRange {
        range: range.start.index()..range.end.index(),
        _phantom: core::marker::PhantomData,
    }
}

/// Iterator type returned by `iter_entity_range`.
pub struct IterEntityRange<E> {
    range: Range<usize>,
    _phantom: core::marker::PhantomData<E>,
}

impl<E> Iterator for IterEntityRange<E>
where
    E: EntityRef,
{
    type Item = E;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.range.next()?;
        Some(E::new(i))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<E> DoubleEndedIterator for IterEntityRange<E>
where
    E: EntityRef,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let i = self.range.next_back()?;
        Some(E::new(i))
    }
}

impl<E> FusedIterator for IterEntityRange<E>
where
    E: EntityRef,
    Range<usize>: FusedIterator,
{
}

impl<E> ExactSizeIterator for IterEntityRange<E>
where
    E: EntityRef,
    Range<usize>: ExactSizeIterator,
{
}

/// Macro which provides the common implementation of a 32-bit entity reference.
#[macro_export]
macro_rules! entity_impl {
    // Basic traits.
    ($entity:ident) => {
        impl $crate::EntityRef for $entity {
            #[inline]
            fn new(index: usize) -> Self {
                debug_assert!(index < ($crate::__core::u32::MAX as usize));
                $entity(index as u32)
            }

            #[inline]
            fn index(self) -> usize {
                self.0 as usize
            }
        }

        impl $crate::packed_option::ReservedValue for $entity {
            #[inline]
            fn reserved_value() -> $entity {
                $entity($crate::__core::u32::MAX)
            }

            #[inline]
            fn is_reserved_value(&self) -> bool {
                self.0 == $crate::__core::u32::MAX
            }
        }

        impl $entity {
            /// Create a new instance from a `u32`.
            #[allow(dead_code, reason = "macro-generated code")]
            #[inline]
            pub fn from_u32(x: u32) -> Self {
                debug_assert!(x < $crate::__core::u32::MAX);
                $entity(x)
            }

            /// Return the underlying index value as a `u32`.
            #[allow(dead_code, reason = "macro-generated code")]
            #[inline]
            pub fn as_u32(self) -> u32 {
                self.0
            }

            /// Return the raw bit encoding for this instance.
            ///
            /// __Warning__: the raw bit encoding is opaque and has no
            /// guaranteed correspondence to the entity's index. It encodes the
            /// entire state of this index value: either a valid index or an
            /// invalid-index sentinel. The value returned by this method should
            /// only be passed to `from_bits`.
            #[allow(dead_code, reason = "macro-generated code")]
            #[inline]
            pub fn as_bits(self) -> u32 {
                self.0
            }

            /// Create a new instance from the raw bit encoding.
            ///
            /// __Warning__: the raw bit encoding is opaque and has no
            /// guaranteed correspondence to the entity's index. It encodes the
            /// entire state of this index value: either a valid index or an
            /// invalid-index sentinel. The value returned by this method should
            /// only be given bits from `as_bits`.
            #[allow(dead_code, reason = "macro-generated code")]
            #[inline]
            pub fn from_bits(x: u32) -> Self {
                $entity(x)
            }
        }
    };

    // Include basic `Display` impl using the given display prefix.
    // Display a `Block` reference as "block12".
    ($entity:ident, $display_prefix:expr) => {
        $crate::entity_impl!($entity);

        impl $crate::__core::fmt::Display for $entity {
            fn fmt(&self, f: &mut $crate::__core::fmt::Formatter) -> $crate::__core::fmt::Result {
                write!(f, concat!($display_prefix, "{}"), self.0)
            }
        }

        impl $crate::__core::fmt::Debug for $entity {
            fn fmt(&self, f: &mut $crate::__core::fmt::Formatter) -> $crate::__core::fmt::Result {
                (self as &dyn $crate::__core::fmt::Display).fmt(f)
            }
        }
    };

    // Alternate form for tuples we can't directly construct; providing "to" and "from" expressions
    // to turn an index *into* an entity, or get an index *from* an entity.
    ($entity:ident, $display_prefix:expr, $arg:ident, $to_expr:expr, $from_expr:expr) => {
        impl $crate::EntityRef for $entity {
            #[inline]
            fn new(index: usize) -> Self {
                debug_assert!(index < ($crate::__core::u32::MAX as usize));
                let $arg = index as u32;
                $to_expr
            }

            #[inline]
            fn index(self) -> usize {
                let $arg = self;
                $from_expr as usize
            }
        }

        impl $crate::packed_option::ReservedValue for $entity {
            #[inline]
            fn reserved_value() -> $entity {
                $entity::from_u32($crate::__core::u32::MAX)
            }

            #[inline]
            fn is_reserved_value(&self) -> bool {
                self.as_u32() == $crate::__core::u32::MAX
            }
        }

        impl $entity {
            /// Create a new instance from a `u32`.
            #[allow(dead_code, reason = "macro-generated code")]
            #[inline]
            pub fn from_u32(x: u32) -> Self {
                debug_assert!(x < $crate::__core::u32::MAX);
                let $arg = x;
                $to_expr
            }

            /// Return the underlying index value as a `u32`.
            #[allow(dead_code, reason = "macro-generated code")]
            #[inline]
            pub fn as_u32(self) -> u32 {
                let $arg = self;
                $from_expr
            }
        }

        impl $crate::__core::fmt::Display for $entity {
            fn fmt(&self, f: &mut $crate::__core::fmt::Formatter) -> $crate::__core::fmt::Result {
                write!(f, concat!($display_prefix, "{}"), self.as_u32())
            }
        }

        impl $crate::__core::fmt::Debug for $entity {
            fn fmt(&self, f: &mut $crate::__core::fmt::Formatter) -> $crate::__core::fmt::Result {
                (self as &dyn $crate::__core::fmt::Display).fmt(f)
            }
        }
    };
}

pub mod packed_option;

mod boxed_slice;
mod iter;
mod keys;
mod list;
mod map;
mod primary;
mod set;
mod signed;
mod sparse;
mod unsigned;

pub use self::boxed_slice::BoxedSlice;
pub use self::iter::{Iter, IterMut};
pub use self::keys::Keys;
pub use self::list::{EntityList, ListPool};
pub use self::map::SecondaryMap;
pub use self::primary::PrimaryMap;
pub use self::set::{EntitySet, SetIter};
pub use self::signed::Signed;
pub use self::sparse::{SparseMap, SparseMapValue, SparseSet};
pub use self::unsigned::Unsigned;

/// A collection of tests to ensure that use of the different `entity_impl!` forms will generate
/// `EntityRef` implementations that behave the same way.
#[cfg(test)]
mod tests {
    /// A macro used to emit some basic tests to show that entities behave as we expect.
    macro_rules! entity_test {
        ($entity:ident) => {
            #[test]
            fn from_usize_to_u32() {
                let e = $entity::new(42);
                assert_eq!(e.as_u32(), 42_u32);
            }

            #[test]
            fn from_u32_to_usize() {
                let e = $entity::from_u32(42);
                assert_eq!(e.index(), 42_usize);
            }

            #[test]
            fn comparisons_work() {
                let a = $entity::from_u32(42);
                let b = $entity::new(42);
                assert_eq!(a, b);
            }

            #[should_panic]
            #[cfg(debug_assertions)]
            #[test]
            fn cannot_construct_from_reserved_u32() {
                use crate::packed_option::ReservedValue;
                let reserved = $entity::reserved_value().as_u32();
                let _ = $entity::from_u32(reserved); // panic
            }

            #[should_panic]
            #[cfg(debug_assertions)]
            #[test]
            fn cannot_construct_from_reserved_usize() {
                use crate::packed_option::ReservedValue;
                let reserved = $entity::reserved_value().index();
                let _ = $entity::new(reserved); // panic
            }
        };
    }

    /// Test cases for a plain ol' `EntityRef` implementation.
    mod basic_entity {
        use crate::EntityRef;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct BasicEntity(u32);
        entity_impl!(BasicEntity);
        entity_test!(BasicEntity);
    }

    /// Test cases for an `EntityRef` implementation that includes a display prefix.
    mod prefix_entity {
        use crate::EntityRef;
        #[derive(Clone, Copy, PartialEq, Eq)]
        struct PrefixEntity(u32);
        entity_impl!(PrefixEntity, "prefix-");
        entity_test!(PrefixEntity);

        #[test]
        fn display_prefix_works() {
            let e = PrefixEntity::new(0);
            assert_eq!(alloc::format!("{e}"), "prefix-0");
        }
    }

    /// Test cases for an `EntityRef` implementation for a type we can only construct through
    /// other means, such as calls to `core::convert::From<u32>`.
    mod other_entity {
        mod inner {
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct InnerEntity(u32);

            impl From<u32> for InnerEntity {
                fn from(x: u32) -> Self {
                    Self(x)
                }
            }

            impl From<InnerEntity> for u32 {
                fn from(x: InnerEntity) -> Self {
                    x.0
                }
            }
        }

        use {self::inner::InnerEntity, crate::EntityRef};
        entity_impl!(InnerEntity, "inner-", i, InnerEntity::from(i), u32::from(i));
        entity_test!(InnerEntity);

        #[test]
        fn display_prefix_works() {
            let e = InnerEntity::new(0);
            assert_eq!(alloc::format!("{e}"), "inner-0");
        }
    }
}

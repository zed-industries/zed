#![doc(html_root_url = "https://docs.rs/slotmap/1.0.7")]
#![crate_name = "slotmap"]
#![cfg_attr(all(nightly, feature = "unstable"), feature(try_reserve))]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(all(nightly, doc), feature(doc_cfg))]
#![warn(
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces
)]
#![deny(missing_docs, unaligned_references)]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        // Style differences.
        module_name_repetitions,
        redundant_closure_for_method_calls,
        unseparated_literal_suffix,

        // I know what I'm doing and want these.
        wildcard_imports,
        inline_always,
        cast_possible_truncation,
        needless_pass_by_value,

        // Very noisy.
        missing_errors_doc,
        must_use_candidate
    ))]

//! # slotmap
//!
//! This library provides a container with persistent unique keys to access
//! stored values, [`SlotMap`]. Upon insertion a key is returned that can be
//! used to later access or remove the values. Insertion, removal and access all
//! take O(1) time with low overhead. Great for storing collections of objects
//! that need stable, safe references but have no clear ownership otherwise,
//! such as game entities or graph nodes.
//!
//! The difference between a [`BTreeMap`] or [`HashMap`] and a slot map is
//! that the slot map generates and returns the key when inserting a value. A
//! key is always unique and will only refer to the value that was inserted.
//! A slot map's main purpose is to simply own things in a safe and efficient
//! manner.
//!
//! You can also create (multiple) secondary maps that can map the keys returned
//! by [`SlotMap`] to other values, to associate arbitrary data with objects
//! stored in slot maps, without hashing required - it's direct indexing under
//! the hood.
//!
//! The minimum required stable Rust version for this crate is 1.49.
//!
//! # Examples
//!
//! ```
//! # use slotmap::*;
//! let mut sm = SlotMap::new();
//! let foo = sm.insert("foo");  // Key generated on insert.
//! let bar = sm.insert("bar");
//! assert_eq!(sm[foo], "foo");
//! assert_eq!(sm[bar], "bar");
//!
//! sm.remove(bar);
//! let reuse = sm.insert("reuse");  // Space from bar reused.
//! assert_eq!(sm.contains_key(bar), false);  // After deletion a key stays invalid.
//!
//! let mut sec = SecondaryMap::new();
//! sec.insert(foo, "noun");  // We provide the key for secondary maps.
//! sec.insert(reuse, "verb");
//!
//! for (key, val) in sm {
//!     println!("{} is a {}", val, sec[key]);
//! }
//! ```
//!
//! # Serialization through [`serde`], [`no_std`] support and unstable features
//!
//! Both keys and the slot maps have full (de)seralization support through
//! the [`serde`] library. A key remains valid for a slot map even after one or
//! both have been serialized and deserialized! This makes storing or
//! transferring complicated referential structures and graphs a breeze. Care has
//! been taken such that deserializing keys and slot maps from untrusted sources
//! is safe. If you wish to use these features you must enable the `serde`
//! feature flag for `slotmap` in your `Cargo.toml`.
//!
//! ```text
//! slotmap = { version = "1.0", features = ["serde"] }
//! ```
//!
//! This crate also supports [`no_std`] environments, but does require the
//! [`alloc`] crate to be available. To enable this you have to disable the
//! `std` feature that is enabled by default:
//!
//! ```text
//! slotmap = { version = "1.0", default-features = false }
//! ```
//!
//! Unfortunately [`SparseSecondaryMap`] is not available in [`no_std`], because
//! it relies on [`HashMap`]. Finally the `unstable` feature can be defined to
//! enable the parts of `slotmap` that only work on nightly Rust.
//!
//! # Why not index a [`Vec`], or use [`slab`], [`stable-vec`], etc?
//!
//! Those solutions either can not reclaim memory from deleted elements or
//! suffer from the ABA problem. The keys returned by `slotmap` are versioned.
//! This means that once a key is removed, it stays removed, even if the
//! physical storage inside the slotmap is reused for new elements. The key is a
//! permanently unique<sup>*</sup> reference to the inserted value. Despite
//! supporting versioning, a [`SlotMap`] is often not (much) slower than the
//! alternative, by internally using carefully checked unsafe code. Finally,
//! `slotmap` simply has a lot of features that make your life easy.
//!
//! # Performance characteristics and implementation details
//!
//! Insertion, access and deletion is all O(1) with low overhead by storing the
//! elements inside a [`Vec`]. Unlike references or indices into a vector,
//! unless you remove a key it is never invalidated. Behind the scenes each
//! slot in the vector is a `(value, version)` tuple. After insertion the
//! returned key also contains a version. Only when the stored version and
//! version in a key match is a key valid. This allows us to reuse space in the
//! vector after deletion without letting removed keys point to spurious new
//! elements. <sup>*</sup>After 2<sup>31</sup> deletions and insertions to the
//! same underlying slot the version wraps around and such a spurious reference
//! could potentially occur. It is incredibly unlikely however, and in all
//! circumstances is the behavior safe. A slot map can hold up to
//! 2<sup>32</sup> - 2 elements at a time.
//!
//! The memory usage for each slot in [`SlotMap`] is `4 + max(sizeof(T), 4)`
//! rounded up to the alignment of `T`. Similarly it is `4 + max(sizeof(T), 12)`
//! for [`HopSlotMap`]. [`DenseSlotMap`] has an overhead of 8 bytes per element
//! and 8 bytes per slot.
//!
//! # Choosing [`SlotMap`], [`HopSlotMap`] or [`DenseSlotMap`]
//!
//! A [`SlotMap`] is the fastest for most operations, except iteration. It can
//! never shrink the size of its underlying storage, because it must remember
//! for each storage slot what the latest stored version was, even if the slot
//! is empty now. This means that iteration can be slow as it must iterate over
//! potentially a lot of empty slots.
//!
//! [`HopSlotMap`] solves this by maintaining more information on
//! insertion/removal, allowing it to iterate only over filled slots by 'hopping
//! over' contiguous blocks of vacant slots. This can give it significantly
//! better iteration speed.  If you expect to iterate over all elements in a
//! [`SlotMap`] a lot, and potentially have a lot of deleted elements, choose
//! [`HopSlotMap`]. The downside is that insertion and removal is roughly twice
//! as slow. Random access is the same speed for both.
//!
//! [`DenseSlotMap`] goes even further and stores all elements on a contiguous
//! block of memory. It uses two indirections per random access; the slots
//! contain indices used to access the contiguous memory. This means random
//! access is slower than both [`SlotMap`] and [`HopSlotMap`], but iteration is
//! significantly faster, as fast as a normal [`Vec`].
//!
//! # Choosing [`SecondaryMap`] or [`SparseSecondaryMap`]
//!
//! You want to associate extra data with objects stored in a slot map, so you
//! use (multiple) secondary maps to map keys to that data.
//!
//! A [`SecondaryMap`] is simply a [`Vec`] of slots like slot map is, and
//! essentially provides all the same guarantees as [`SlotMap`] does for its
//! operations (with the exception that you provide the keys as produced by the
//! primary slot map). This does mean that even if you associate data to only
//! a single element from the primary slot map, you could need and have to
//! initialize as much memory as the original.
//!
//! A [`SparseSecondaryMap`] is like a [`HashMap`] from keys to objects, however
//! it automatically removes outdated keys for slots that had their space
//! reused. You should use this variant if you expect to store some associated
//! data for only a small portion of the primary slot map.
//!
//! # Custom key types
//!
//! If you have multiple slot maps it's an error to use the key of one slot map
//! on another slot map. The result is safe, but unspecified, and can not be
//! detected at runtime, so it can lead to a hard to find bug.
//!
//! To prevent this, slot maps allow you to specify what the type is of the key
//! they return. You can construct new key types using the [`new_key_type!`]
//! macro. The resulting type behaves exactly like [`DefaultKey`], but is a
//! distinct type. So instead of simply using `SlotMap<DefaultKey, Player>` you
//! would use:
//!
//! ```
//! # use slotmap::*;
//! # #[derive(Copy, Clone)]
//! # struct Player;
//! new_key_type! { struct PlayerKey; }
//! let sm: SlotMap<PlayerKey, Player> = SlotMap::with_key();
//! ```
//!
//! You can write code generic over any key type using the [`Key`] trait.
//!
//! [`Vec`]: std::vec::Vec
//! [`BTreeMap`]: std::collections::BTreeMap
//! [`HashMap`]: std::collections::HashMap
//! [`serde`]: https://github.com/serde-rs/serde
//! [`slab`]: https://crates.io/crates/slab
//! [`stable-vec`]: https://crates.io/crates/stable-vec
//! [`no_std`]: https://doc.rust-lang.org/1.7.0/book/no-stdlib.html

extern crate alloc;

// So our macros can refer to these.
#[doc(hidden)]
pub mod __impl {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub use core::convert::From;
    pub use core::result::Result;
}

pub mod basic;
pub mod dense;
pub mod hop;
pub mod secondary;
#[cfg(feature = "std")]
pub mod sparse_secondary;
pub(crate) mod util;

use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::num::NonZeroU32;

#[doc(inline)]
pub use crate::basic::SlotMap;
#[doc(inline)]
pub use crate::dense::DenseSlotMap;
#[doc(inline)]
pub use crate::hop::HopSlotMap;
#[doc(inline)]
pub use crate::secondary::SecondaryMap;
#[cfg(feature = "std")]
#[doc(inline)]
pub use crate::sparse_secondary::SparseSecondaryMap;

// Keep Slottable for backwards compatibility, but warn about deprecation
// and hide from documentation.
#[doc(hidden)]
#[deprecated(
    since = "1.0.0",
    note = "Slottable is not necessary anymore, slotmap now supports all types on stable."
)]
pub trait Slottable {}

#[doc(hidden)]
#[allow(deprecated)]
impl<T> Slottable for T {}

/// The actual data stored in a [`Key`].
///
/// This implements [`Ord`](std::cmp::Ord) so keys can be stored in e.g.
/// [`BTreeMap`](std::collections::BTreeMap), but the order of keys is
/// unspecified.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyData {
    idx: u32,
    version: NonZeroU32,
}

impl KeyData {
    fn new(idx: u32, version: u32) -> Self {
        debug_assert!(version > 0);

        Self {
            idx,
            version: unsafe { NonZeroU32::new_unchecked(version | 1) },
        }
    }

    fn null() -> Self {
        Self::new(core::u32::MAX, 1)
    }

    fn is_null(self) -> bool {
        self.idx == core::u32::MAX
    }

    /// Returns the key data as a 64-bit integer. No guarantees about its value
    /// are made other than that passing it to [`from_ffi`](Self::from_ffi)
    /// will return a key equal to the original.
    ///
    /// With this you can easily pass slot map keys as opaque handles to foreign
    /// code. After you get them back you can confidently use them in your slot
    /// map without worrying about unsafe behavior as you would with passing and
    /// receiving back references or pointers.
    ///
    /// This is not a substitute for proper serialization, use [`serde`] for
    /// that. If you are not doing FFI, you almost surely do not need this
    /// function.
    ///
    /// [`serde`]: crate#serialization-through-serde-no_std-support-and-unstable-features
    pub fn as_ffi(self) -> u64 {
        (u64::from(self.version.get()) << 32) | u64::from(self.idx)
    }

    /// Iff `value` is a value received from `k.as_ffi()`, returns a key equal
    /// to `k`. Otherwise the behavior is safe but unspecified.
    pub fn from_ffi(value: u64) -> Self {
        let idx = value & 0xffff_ffff;
        let version = (value >> 32) | 1; // Ensure version is odd.
        Self::new(idx as u32, version as u32)
    }
}

impl Debug for KeyData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}v{}", self.idx, self.version.get())
    }
}

impl Default for KeyData {
    fn default() -> Self {
        Self::null()
    }
}

impl Hash for KeyData
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        // A derived Hash impl would call write_u32 twice. We call write_u64
        // once, which is beneficial if the hasher implements write_u64
        // explicitly.
        state.write_u64(self.as_ffi())
    }
}

/// Key used to access stored values in a slot map.
///
/// Do not use a key from one slot map in another. The behavior is safe but
/// non-sensical (and might panic in case of out-of-bounds).
///
/// To prevent this, it is suggested to have a unique key type for each slot
/// map. You can create new key types using [`new_key_type!`], which makes a
/// new type identical to [`DefaultKey`], just with a different name.
///
/// This trait is intended to be a thin wrapper around [`KeyData`], and all
/// methods must behave exactly as if we're operating on a [`KeyData`] directly.
/// The internal unsafe code relies on this, therefore this trait is `unsafe` to
/// implement. It is strongly suggested to simply use [`new_key_type!`] instead
/// of implementing this trait yourself.
pub unsafe trait Key:
    From<KeyData>
    + Copy
    + Clone
    + Default
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + core::hash::Hash
    + core::fmt::Debug
{
    /// Creates a new key that is always invalid and distinct from any non-null
    /// key. A null key can only be created through this method (or default
    /// initialization of keys made with [`new_key_type!`], which calls this
    /// method).
    ///
    /// A null key is always invalid, but an invalid key (that is, a key that
    /// has been removed from the slot map) does not become a null key. A null
    /// is safe to use with any safe method of any slot map instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let k = sm.insert(42);
    /// let nk = DefaultKey::null();
    /// assert!(nk.is_null());
    /// assert!(k != nk);
    /// assert_eq!(sm.get(nk), None);
    /// ```
    fn null() -> Self {
        KeyData::null().into()
    }

    /// Checks if a key is null. There is only a single null key, that is
    /// `a.is_null() && b.is_null()` implies `a == b`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// new_key_type! { struct MyKey; }
    /// let a = MyKey::null();
    /// let b = MyKey::default();
    /// assert_eq!(a, b);
    /// assert!(a.is_null());
    /// ```
    fn is_null(&self) -> bool {
        self.data().is_null()
    }

    /// Gets the [`KeyData`] stored in this key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// new_key_type! { struct MyKey; }
    /// let dk = DefaultKey::null();
    /// let mk = MyKey::null();
    /// assert_eq!(dk.data(), mk.data());
    /// ```
    fn data(&self) -> KeyData;
}

/// A helper macro to create new key types. If you use a new key type for each
/// slot map you create you can entirely prevent using the wrong key on the
/// wrong slot map.
///
/// The type constructed by this macro is defined exactly as [`DefaultKey`],
/// but is a distinct type for the type checker and does not implicitly convert.
///
/// # Examples
///
/// ```
/// # extern crate slotmap;
/// # use slotmap::*;
/// new_key_type! {
///     // A private key type.
///     struct RocketKey;
///
///     // A public key type with a doc comment.
///     /// Key for the user slot map.
///     pub struct UserKey;
/// }
///
/// fn main() {
///     let mut users = SlotMap::with_key();
///     let mut rockets = SlotMap::with_key();
///     let bob: UserKey = users.insert("bobby");
///     let apollo: RocketKey = rockets.insert("apollo");
///     // Now this is a type error because rockets.get expects an RocketKey:
///     // rockets.get(bob);
///
///     // If for some reason you do end up needing to convert (e.g. storing
///     // keys of multiple slot maps in the same data structure without
///     // boxing), you can use KeyData as an intermediate representation. This
///     // does mean that once again you are responsible for not using the wrong
///     // key on the wrong slot map.
///     let keys = vec![bob.data(), apollo.data()];
///     println!("{} likes rocket {}",
///              users[keys[0].into()], rockets[keys[1].into()]);
/// }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! new_key_type {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident; $($rest:tt)* ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Default,
                 Eq, PartialEq, Ord, PartialOrd,
                 Hash, Debug)]
        #[repr(transparent)]
        $vis struct $name($crate::KeyData);

        impl $crate::__impl::From<$crate::KeyData> for $name {
            fn from(k: $crate::KeyData) -> Self {
                $name(k)
            }
        }

        unsafe impl $crate::Key for $name {
            fn data(&self) -> $crate::KeyData {
                self.0
            }
        }

        $crate::__serialize_key!($name);

        $crate::new_key_type!($($rest)*);
    };

    () => {}
}

#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! __serialize_key {
    ( $name:ty ) => {
        impl $crate::__impl::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> $crate::__impl::Result<S::Ok, S::Error>
            where
                S: $crate::__impl::Serializer,
            {
                $crate::Key::data(self).serialize(serializer)
            }
        }

        impl<'de> $crate::__impl::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> $crate::__impl::Result<Self, D::Error>
            where
                D: $crate::__impl::Deserializer<'de>,
            {
                let key_data: $crate::KeyData =
                    $crate::__impl::Deserialize::deserialize(deserializer)?;
                Ok(key_data.into())
            }
        }
    };
}

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __serialize_key {
    ( $name:ty ) => {};
}

new_key_type! {
    /// The default slot map key type.
    pub struct DefaultKey;
}

// Serialization with serde.
#[cfg(feature = "serde")]
mod serialize {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct SerKey {
        idx: u32,
        version: u32,
    }

    impl Serialize for KeyData {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let ser_key = SerKey {
                idx: self.idx,
                version: self.version.get(),
            };
            ser_key.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for KeyData {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let mut ser_key: SerKey = Deserialize::deserialize(deserializer)?;

            // Ensure a.is_null() && b.is_null() implies a == b.
            if ser_key.idx == core::u32::MAX {
                ser_key.version = 1;
            }

            ser_key.version |= 1; // Ensure version is odd.
            Ok(Self::new(ser_key.idx, ser_key.version))
        }
    }
}

#[cfg(test)]
mod tests {
    // Intentionally no `use super::*;` because we want to test macro expansion
    // in the *users* scope, which might not have that.
    #[test]
    fn macro_expansion() {
        #![allow(dead_code)]
        use super::new_key_type;

        // Clobber namespace with clashing names - should still work.
        trait Serialize { }
        trait Deserialize { }
        trait Serializer { }
        trait Deserializer { }
        trait Key { }
        trait From { }
        struct Result;
        struct KeyData;

        new_key_type! {
            struct A;
            pub(crate) struct B;
            pub struct C;
        }
    }

    #[test]
    fn check_is_older_version() {
        use super::util::is_older_version;

        let is_older = |a, b| is_older_version(a, b);
        assert!(!is_older(42, 42));
        assert!(is_older(0, 1));
        assert!(is_older(0, 1 << 31));
        assert!(!is_older(0, (1 << 31) + 1));
        assert!(is_older(u32::MAX, 0));
    }

    #[test]
    fn iters_cloneable() {
        use super::*;

        struct NoClone;

        let mut sm = SlotMap::new();
        let mut hsm = HopSlotMap::new();
        let mut dsm = DenseSlotMap::new();
        let mut scm = SecondaryMap::new();
        let mut sscm = SparseSecondaryMap::new();
        scm.insert(sm.insert(NoClone), NoClone);
        sscm.insert(hsm.insert(NoClone), NoClone);
        dsm.insert(NoClone);

        let _ = sm.keys().clone();
        let _ = sm.values().clone();
        let _ = sm.iter().clone();
        let _ = hsm.keys().clone();
        let _ = hsm.values().clone();
        let _ = hsm.iter().clone();
        let _ = dsm.keys().clone();
        let _ = dsm.values().clone();
        let _ = dsm.iter().clone();
        let _ = scm.keys().clone();
        let _ = scm.values().clone();
        let _ = scm.iter().clone();
        let _ = sscm.keys().clone();
        let _ = sscm.values().clone();
        let _ = sscm.iter().clone();
    }

    #[cfg(feature = "serde")]
    #[test]
    fn key_serde() {
        use super::*;

        // Check round-trip through serde.
        let mut sm = SlotMap::new();
        let k = sm.insert(42);
        let ser = serde_json::to_string(&k).unwrap();
        let de: DefaultKey = serde_json::from_str(&ser).unwrap();
        assert_eq!(k, de);

        // Even if a malicious entity sends up even (unoccupied) versions in the
        // key, we make the version point to the occupied version.
        let malicious: KeyData = serde_json::from_str(&r#"{"idx":0,"version":4}"#).unwrap();
        assert_eq!(malicious.version.get(), 5);
    }
}

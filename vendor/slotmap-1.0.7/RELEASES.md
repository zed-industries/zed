Version 1.0.7
=============

 - Added `clone_from` implementations for all slot maps.
 - Added `try_insert_with_key` methods that accept a fallible closure.
 - Improved performance of insertion and key hashing.
 - Made `new_key_type` resistant to shadowing.
 - Made iterators clonable regardless of item type clonability.


Version 1.0.6
=============

 - Made `Key` trait unsafe, as it was erroneously safe to implement.


Version 1.0.5
=============

 - Added fuzzing for extra testing.
 - Fixed an issue that could cause a segfault when using `HopSlotMap::retain`
   that had the same underlying cause as the fix in 1.0.4 but was missed.


Version 1.0.4
=============

 - Fixed an issue that could cause a segfault when using `HopSlotMap::drain`.
   All versions 0.3+ are affected, and thus yanked.


Version 1.0.3
=============

 - Made `get_disjoint_mut` available on stable Rust 1.51 and up.
 - Added unchecked variants for the getters on `SparseSecondaryMap`.


Version 1.0.2
=============

 - Fixed the `new_key_type!` macro, it assumed the `Key` trait was in scope.
 - Updated code base with more stringent (clippy) warnings, and many small code
   quality and documentation changes.
 - Documented the minimum required stable Rust version, which is 1.49.


Version 1.0.1
=============

 - Fixed an instance where an uninitialized `[u32; N]` was created. The
   uninitialized values were never read - the code always initialized them
   before reading - but simply having the variable be uninitialized (despite all
   bit patterns being valid) is technically undefined behavior.


Version 1.0.0
=============

 - Removed all `Copy` trait restrictions of value types stable Rust! There are
   no longer any restrictions on the types you can store in any of the
   slot maps. For that reason `Slottable` was deprecated as well.

 - `no_std` support was added, use it by opting out of the default feature `std`.

 - Added `sm.get_disjoint_mut([k1, k2, ...])` which allows you to get mutable
   references from multiple disjoint keys at the same time. This requires
   `min-const-generics` to be stabilized, so until Rust 1.51 comes out this is
   only available on nightly by setting the `unstable` feature.

 - Added an `Entry` API to the secondary maps.

 - Added `derive(Clone)` for iterators where possible.

 - Replaced `Into<KeyData>` with `Key::data()`.

 - `SecondaryMap` now uses minimal space overhead. Each slot now uses
   `max(sizeof(T), 4)` bytes.
 
 - Moved `SlotMap` to the `basic` module.


Version 0.4.1
=============

 - Backport of fix made in 1.0.4.


Version 0.4.0
=============

 - Codebase moved to 2018 Edition.

 - Reintroduce `DenseSlotMap` - an overzealous removal in 0.3.0.
 
 - Added support for `try_reserve`.

 - Added support for custom hashers in `SparseSecondaryMap`.

 - `SparseSecondaryMap` and `SecondaryMap` can now be cloned.

 - Keys have a more terse debug output.

 - Fixed a bug that caused an overflowing left shift on 32-bit targets.


Version 0.3.0
=============

 - Massive rework, with a focus on secondary maps and custom keys to prevent
   cross-slotmap key usage.

 - Removed `DenseSlotMap` in favour of `HopSlotMap` as the latter performs
   better when secondary maps are in use.
   
 - Unfortunately due to the redesign the first slot in a slot map must now
   always be empty. This means some deserializations of slot maps serialized
   with a version before 0.3.0 can fail.

 - Added `SecondaryMap` and `SparseSecondaryMap`, which allows you to associate
   extra data with keys given by a slot map. 

 - Added `DefaultKey`, custom key types, and support for them on all slot maps
   and secondary maps. You must now always specify the key type you're using
   with a slot map, so `SlotMap<i32>` would be `SlotMap<DefaultKey, i32>`. It is
   recommended to make a custom key type with `new_key_type!` for any slot map
   you create, as this entirely prevents using the wrong key on the wrong slot
   map.

 - `KeyData` now has `as_ffi` and `from_ffi` functions that convert the data
   that makes up a key to/from an `u64`. This allows you to use slot map keys
   as opaque handles in FFI code.


Version 0.2.1
=============

 - Fixed a potential uninitialized memory vulnerability. No uninitialized memory
   was read or used directly, but Rust's assumptions could lead to it. Yanked
   all previous versions as they were all vulnerable.

 - Made a `Key` member non-zero so that `Option<Key>` is optimized.


Version 0.2.0
=============
Start of version history.

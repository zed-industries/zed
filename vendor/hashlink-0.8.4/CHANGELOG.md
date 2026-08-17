## [0.8.4]
- Now builds with `#![no_std]`.

## [0.8.3]
- bump hashbrown to 0.14

## [0.8.2]
- bump hashbrown to 0.13

## [0.8.1]
- Add `retain_with_order` methods, equivalent to `retain` but which iterate
  through the map in the proper linked list order

## [0.8.0]
- API incompatible change: No longer re-export hashbrown types so that bumping
  hashbrown is no longer an API compatible change.
- bump hashbrown to 0.12
- Fix implementation of `shrink_to_fit` to not panic when called on non-empty
  containers.

## [0.7.0]
- API incompatible change: depend on hashbrown 0.11, changes re-exported types.
- Fix `LinkedHashSet::back` to take `&self` not `&mut self`.
- API incompatible change: equality tests on `LinkedHashSet` are now *ordered*,
  similar to `LinkedHashMap`.
- Make the serde `Deserialize` implementations on `LinkedHashMap` and
  `LinkedHashSet` generic on the `BuildHasher` type.
- Add `to_back` and `to_front` methods for `LinkedHashMap` to control entry
  order.

## [0.6.0]
- API incompatible change: depend on hashbrown 0.9, re-export renamed
  hashbrown::TryReserveError type.
- Add a `Debug` impl to `LruCache` (thanks @thomcc!)
- Adjust trait bounds for `LinkedHashMap::retain`, `LinkedHashSet::default` to
  be less strict (to match hashbrown)
- Adjust trait bounds for all `Debug` impls to be less strict (to match
  hashbrown).
- Adjust trait bounds for all `IntoIterator` impls to be less strict (to match
  hashbrown).
- Adjust trait bounds for `LruCache::with_hasher`, `LruCache::capacity`,
  `LruCache::len`, `LruCache::is_empty`, `LruCache::clear`, `LruCache::iter`,
  `LruCache::iter_mut`, and `LruCache::drain` to be less strict
- Add optional serde support for `LinkedHashMap` and `LinkedHashSet`.
- Add `to_back` and `to_front` methods for LinkedHashSet to control entry order.

## [0.5.1]
- Add `LinkedHashMap::remove_entry` and `LruCache::remove_entry`
- Add `LruCache::new_unbounded` constructor that sets capacity to usize::MAX
- Add `LruCache::get` method to go with `LruCache::get_mut`
- Add `LruCache::peek` and `LruCache::peek_mut` to access the cache without
  moving the entry in the LRU list

## [0.5.0]
- API incompatible change: depend on hashbrown 0.7

## [0.4.0]
- API incompatible change: depend on hashbrown 0.6
- Passes miri

## [0.3.0]
- Add some *minimal* documentation for methods that change the internal ordering.
- Decide on a pattern for methods that change the internal ordering: the word
  "insert" means that it will move an existing entry to the back.
- Some methods have been renamed to conform to the above system.

## [0.2.1]
- Fix variance for LinkedHashMap (now covariant where appropriate)
- Add Debug impls to many more associated types
- Add LinkedHashSet
- Add `LinkedHashMap::retain`

## [0.2.0]
- Move `linked_hash_map` into its own module
- Add `LruCache` type ported from `lru-cache` crate into its own module
- Add `LruCache` entry and raw-entry API
- Add `linked_hash_map` `IntoIter` iterator that is different from `Drain` iterator
- Make `Drain` iterator recycle freed linked list nodes

## [0.1.0]
- Initial release

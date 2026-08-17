# hashlink -- HashMap-like containers that hold their key-value pairs in a user controllable order

[![Build Status](https://img.shields.io/circleci/project/github/kyren/hashlink.svg)](https://circleci.com/gh/kyren/hashlink)
[![Latest Version](https://img.shields.io/crates/v/hashlink.svg)](https://crates.io/crates/hashlink)
[![API Documentation](https://docs.rs/hashlink/badge.svg)](https://docs.rs/hashlink)

This crate is a fork of 
[linked-hash-map](https://github.com/contain-rs/linked-hash-map) that builds on
top of [hashbrown](https://github.com/rust-lang/hashbrown) to implement more up
to date versions of `LinkedHashMap` `LinkedHashSet`, and `LruCache`.

One important API change is that when a `LinkedHashMap` is used as a LRU cache,
it allows you to easily retrieve an entry and move it to the back OR produce a
new entry at the back without needlessly repeating key hashing and lookups:

``` rust
let mut lru_cache = LinkedHashMap::new();
let key = "key".to_owned();
// Try to find my expensive to construct and hash key
let _cached_val = match lru_cache.raw_entry_mut().from_key(&key) {
    RawEntryMut::Occupied(mut occupied) => {
        // Cache hit, move entry to the back.
        occupied.to_back();
        occupied.into_mut()
    }
    RawEntryMut::Vacant(vacant) => {
        // Insert expensive to construct key and expensive to compute value,
        // automatically inserted at the back.
        vacant.insert(key.clone(), 42).1
    }
};
```

Or, a simpler way to do the same thing:

``` rust
let mut lru_cache = LinkedHashMap::new();
let key = "key".to_owned();
let _cached_val = lru_cache
    .raw_entry_mut()
    .from_key(&key)
    .or_insert_with(|| (key.clone(), 42));
```

This crate contains a decent amount of unsafe code from handling its internal
linked list, and the unsafe code has diverged quite a lot from the original
`linked-hash-map` implementation.  It currently passes tests under miri and
sanitizers, but it should probably still receive more review and testing, and
check for test code coverage.

## Credit

There is a huge amount of code in this crate that is copied verbatim from
`linked-hash-map` and `hashbrown`, especially tests, associated types like
iterators, and things like `Debug` impls.

## License

This library is licensed the same as
[linked-hash-map](https://github.com/contain-rs/linked-hash-map) and
[hashbrown](https://github.com/rust-lang/hashbrown), it is licensed under either
of:

* MIT license [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
* Apache License 2.0 [LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/licenses/Apache-2.0

at your option.

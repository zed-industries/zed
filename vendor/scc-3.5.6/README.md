# Scalable Concurrent Containers

[![Cargo](https://img.shields.io/crates/v/scc)](https://crates.io/crates/scc)
![Crates.io](https://img.shields.io/crates/l/scc)

A collection of high-performance asynchronous/concurrent containers with both asynchronous and synchronous interfaces.

#### Features

- Provides both asynchronous and synchronous interfaces.
- SIMD lookup to scan multiple entries in parallel: requires `RUSTFLAGS='-C target_feature=+avx2'` on `x86_64`.
- [`Equivalent`](https://github.com/indexmap-rs/equivalent), [`Loom`](https://github.com/tokio-rs/loom) and [`Serde`](https://github.com/serde-rs/serde) support: `features = ["equivalent", "loom", "serde"]`.

#### Concurrent Containers

- [`HashMap`](#hashmap) is an asynchronous/concurrent hash map.
- [`HashSet`](#hashset) is an asynchronous/concurrent hash set.
- [`HashIndex`](#hashindex) is a read-optimized asynchronous/concurrent hash map.
- [`HashCache`](#hashcache) is a 32-way associative asynchronous/concurrent cache backed by [`HashMap`](#hashmap).
- [`TreeIndex`](#treeindex) is a read-optimized asynchronous/concurrent B-plus tree.

## `HashMap`

[`HashMap`](#hashmap) is an asynchronous/concurrent hash map optimized for highly parallel write-heavy workloads. [`HashMap`](#hashmap) is structured as a lock-free stack of entry bucket arrays. The entry bucket array is managed by [`sdd`](https://crates.io/crates/sdd), thus enabling lock-free access to it and non-blocking container resizing. Each bucket is a fixed-size array of entries, protected by a read-write lock that simultaneously provides blocking and asynchronous methods.

### Locking behavior

#### Entry access: fine-grained locking

Read/write access to an entry is serialized by the read-write lock in the bucket containing the entry. There are no container-level locks; therefore, the larger the container gets, the lower the chance of the bucket-level lock being contended.

#### Resize: lock-free

Resizing of a [`HashMap`](#hashmap) is entirely non-blocking and lock-free; resizing does not block any other read/write access to the container or resizing attempts. _Resizing is analogous to pushing a new bucket array into a lock-free stack_. Each entry in the old bucket array is incrementally relocated to the new bucket array upon future access to the container, and the old bucket array is eventually dropped after it becomes empty.

### Examples

Inserted entries can be updated, read, and removed synchronously or asynchronously.

```rust
use scc::HashMap;

let hashmap: HashMap<u64, u32> = HashMap::default();

assert!(hashmap.insert_sync(1, 0).is_ok());
assert!(hashmap.insert_sync(1, 1).is_err());
assert_eq!(hashmap.upsert_sync(1, 1).unwrap(), 0);
assert_eq!(hashmap.update_sync(&1, |_, v| { *v = 3; *v }).unwrap(), 3);
assert_eq!(hashmap.read_sync(&1, |_, v| *v).unwrap(), 3);
assert_eq!(hashmap.remove_sync(&1).unwrap(), (1, 3));

hashmap.entry_sync(7).or_insert(17);
assert_eq!(hashmap.read_sync(&7, |_, v| *v).unwrap(), 17);

let future_insert = hashmap.insert_async(2, 1);
let future_remove = hashmap.remove_async(&1);
```

The `Entry` API of [`HashMap`](#hashmap) is helpful for complicated workflows.

```rust
use scc::HashMap;

let hashmap: HashMap<u64, u32> = HashMap::default();

hashmap.entry_sync(3).or_insert(7);
assert_eq!(hashmap.read_sync(&3, |_, v| *v), Some(7));

hashmap.entry_sync(4).and_modify(|v| { *v += 1 }).or_insert(5);
assert_eq!(hashmap.read_sync(&4, |_, v| *v), Some(5));
```

[`HashMap`](#hashmap) does not provide an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) since it is impossible to confine the lifetime of [`Iterator::Item`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item) to the [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html). The limitation can be circumvented by relying on interior mutability, for example, letting the returned reference hold a lock. However, this may lead to a deadlock if not used correctly, and frequent acquisition of locks may impact performance. Therefore, [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) is not implemented; instead, [`HashMap`](#hashmap) provides several methods to iterate over entries synchronously or asynchronously: `iter_{async|sync}`, `iter_mut_{async|sync}`, `retain_{async|sync}`, `begin_{async|sync}`, `OccupiedEntry::next_{async|sync}`, and `OccupiedEntry::remove_and_{async|sync}`

```rust
use scc::HashMap;

let hashmap: HashMap<u64, u32> = HashMap::default();

assert!(hashmap.insert_sync(1, 0).is_ok());
assert!(hashmap.insert_sync(2, 1).is_ok());

// Entries can be modified or removed via `retain_sync`.
let mut acc = 0;
hashmap.retain_sync(|k, v_mut| {
    acc += *k;
    *v_mut = 2;
    true
});
assert_eq!(acc, 3);
assert_eq!(hashmap.read_sync(&1, |_, v| *v).unwrap(), 2);
assert_eq!(hashmap.read_sync(&2, |_, v| *v).unwrap(), 2);

// `iter_sync` returns `true` when all the entries satisfy the predicate.
assert!(hashmap.insert_sync(3, 2).is_ok());
assert!(!hashmap.iter_sync(|k, _| *k == 3));

// Multiple entries can be removed through `retain_sync`.
hashmap.retain_sync(|k, v| *k == 1 && *v == 2);

// `hash_map::OccupiedEntry` also can return the next closest occupied entry.
let first_entry = hashmap.begin_sync();
assert_eq!(*first_entry.as_ref().unwrap().key(), 1);
let second_entry = first_entry.and_then(|e| e.next_sync());
assert!(second_entry.is_none());

fn is_send<T: Send>(f: &T) -> bool {
    true
}

// Asynchronous iteration over entries using `iter_async`.
let future_scan = hashmap.iter_async(|k, v| {
    println!("{k} {v}");
    true
});
assert!(is_send(&future_scan));

// Asynchronous iteration over entries using the `Entry` API.
let future_iter = async {
    let mut iter = hashmap.begin_async().await;
    while let Some(entry) = iter {
        // `OccupiedEntry` can be sent across awaits and threads.
        assert!(is_send(&entry));
        assert_eq!(*entry.key(), 1);
        iter = entry.next_async().await;
    }
};
assert!(is_send(&future_iter));
```

## `HashSet`

[`HashSet`](#hashset) is a special version of [`HashMap`](#hashmap) where the value type is `()`.

### Examples

[`HashSet`](#hashset) methods are identical to those of [`HashMap`](#hashmap) except that they do not receive a value argument, and some [`HashMap`](#hashmap) methods for value modification are not implemented for [`HashSet`](#hashset).

```rust
use scc::HashSet;

let hashset: HashSet<u64> = HashSet::default();

assert!(hashset.read_sync(&1, |_| true).is_none());
assert!(hashset.insert_sync(1).is_ok());
assert!(hashset.read_sync(&1, |_| true).unwrap());

let future_insert = hashset.insert_async(2);
let future_remove = hashset.remove_async(&1);
```

## `HashIndex`

[`HashIndex`](#hashindex) is a read-optimized version of [`HashMap`](#hashmap). In a [`HashIndex`](#hashindex), not only is the memory of the bucket array managed by [`sdd`](https://crates.io/crates/sdd), but also that of entry buckets is protected by [`sdd`](https://crates.io/crates/sdd), enabling lock-free read access to individual entries.

### Entry lifetime

The `HashIndex` does not drop removed entries immediately; instead, they are dropped only when the bucket is accessed again after the [`sdd`](https://crates.io/crates/sdd) mechanism ensures there are no potential readers for those entries. This implies that a removed entry may persist as long as there are potential readers or the bucket remains unaccessed. As a result, `HashIndex` is not an optimal choice for write-heavy workloads that involve large entry sizes.

### Examples

The `peek` and `peek_with` methods are completely lock-free.

```rust
use scc::HashIndex;

let hashindex: HashIndex<u64, u32> = HashIndex::default();

assert!(hashindex.insert_sync(1, 0).is_ok());

// `peek` and `peek_with` are lock-free.
assert_eq!(hashindex.peek_with(&1, |_, v| *v).unwrap(), 0);

let future_insert = hashindex.insert_async(2, 1);
let future_remove = hashindex.remove_if_async(&1, |_| true);
```

The `Entry` API of [`HashIndex`](#hashindex) can update an existing entry.

```rust
use scc::HashIndex;

let hashindex: HashIndex<u64, u32> = HashIndex::default();
assert!(hashindex.insert_sync(1, 1).is_ok());

if let Some(mut o) = hashindex.get_sync(&1) {
    // Create a new version of the entry.
    o.update(2);
};

if let Some(mut o) = hashindex.get_sync(&1) {
    // Update the entry in place.
    unsafe { *o.get_mut() = 3; }
};
```

An [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) is implemented for [`HashIndex`](#hashindex).

```rust
use scc::HashIndex;

use sdd::Guard;

let hashindex: HashIndex<u64, u32> = HashIndex::default();

assert!(hashindex.insert_sync(1, 0).is_ok());

// Existing values can be replaced with a new one.
hashindex.get_sync(&1).unwrap().update(1);

let guard = Guard::new();

// An `Guard` has to be supplied to `iter`.
let mut iter = hashindex.iter(&guard);

let entry_ref = iter.next().unwrap();
assert_eq!(iter.next(), None);
```

## `HashCache`

[`HashCache`](#hashcache) is a 32-way associative asynchronous/concurrent cache based on the [`HashMap`](#hashmap) implementation. [`HashCache`](#hashcache) does not keep track of the least recently used entry in the entire cache. Instead, each bucket maintains a doubly linked list of occupied entries, which is updated on entry access.

### Examples

The LRU entry in a bucket is evicted when a new entry is inserted, and the bucket is full.

```rust
use scc::HashCache;

let hashcache: HashCache<u64, u32> = HashCache::with_capacity(100, 2000);

/// The capacity cannot exceed the maximum capacity.
assert_eq!(hashcache.capacity_range(), 128..=2048);

/// If the bucket corresponding to `1` or `2` is full, the LRU entry will be evicted.
assert!(hashcache.put_sync(1, 0).is_ok());
assert!(hashcache.put_sync(2, 0).is_ok());

/// `1` becomes the most recently accessed entry in the bucket.
assert!(hashcache.get_sync(&1).is_some());

/// An entry can be normally removed.
assert_eq!(hashcache.remove_sync(&2).unwrap(), (2, 0));
```

## `TreeIndex`

[`TreeIndex`](#treeindex) is a B-plus tree variant optimized for read operations. [`sdd`](https://crates.io/crates/sdd) protects the memory used by individual entries, thus enabling lock-free read access to them.

### Locking behavior

Read access is always lock-free and non-blocking. Write access to an entry is lock-free and non-blocking as long as no structural changes are required. However, when nodes are split or merged by a write operation, other write operations on keys in the affected range are blocked.

### Entry lifetime

`TreeIndex` does not drop removed entries immediately. Instead, they are dropped when the leaf node is cleared or split, making `TreeIndex` a suboptimal choice if the workload is write-heavy.

### Examples

Locks are acquired or awaited when internal nodes are split or merged, however blocking operations do not affect read operations.

```rust
use scc::TreeIndex;

let treeindex: TreeIndex<u64, u32> = TreeIndex::new();

assert!(treeindex.insert_sync(1, 2).is_ok());

// `peek` and `peek_with` are lock-free.
assert_eq!(treeindex.peek_with(&1, |_, v| *v).unwrap(), 2);
assert!(treeindex.remove_sync(&1));

let future_insert = treeindex.insert_async(2, 3);
let future_remove = treeindex.remove_if_async(&1, |v| *v == 2);
```

Entries can be scanned without acquiring any locks.

```rust
use scc::TreeIndex;

use sdd::Guard;

let treeindex: TreeIndex<u64, u32> = TreeIndex::new();

assert!(treeindex.insert_sync(1, 10).is_ok());
assert!(treeindex.insert_sync(2, 11).is_ok());
assert!(treeindex.insert_sync(3, 13).is_ok());

let guard = Guard::new();

// `iter` iterates over entries without acquiring a lock.
let mut iter = treeindex.iter(&guard);
assert_eq!(iter.next().unwrap(), (&1, &10));
assert_eq!(iter.next().unwrap(), (&2, &11));
assert_eq!(iter.next().unwrap(), (&3, &13));
assert!(iter.next().is_none());
```

A specific range of keys can be scanned.

```rust
use scc::TreeIndex;

use sdd::Guard;

let treeindex: TreeIndex<u64, u32> = TreeIndex::new();

for i in 0..10 {
    assert!(treeindex.insert_sync(i, 10).is_ok());
}

let guard = Guard::new();

assert_eq!(treeindex.range(1..1, &guard).count(), 0);
assert_eq!(treeindex.range(4..8, &guard).count(), 4);
assert_eq!(treeindex.range(4..=8, &guard).count(), 5);
```

## Performance

### SIMD support

[`HashMap`](#hashmap) is optimized for 256-bit SIMD instructions. Therefore, it is recommended to compile with `avx2` or equivalent options on `x86-64` targets, or with respective features on other platforms.

* Note that Apple M-series CPUs do not support the 256-bit SIMD instructions required for optimal performance.

### [`HashMap`](#hashmap) Tail Latency

The expected tail latency of a distribution of latencies of 1048576 insertion operations (`K = u64, V = u64`) is less than 50 microseconds on Apple M4 Pro.

### [`HashMap`](#hashmap), [`HashIndex`](#hashindex), and [`TreeIndex`](#treeindex) Multi-threaded Throughput

Multi-threaded throughput test results are available [here](https://codeberg.org/wvwwvwwv/scalable-concurrent-containers/src/branch/main/extended_benches/README.md).

## [Changelog](https://codeberg.org/wvwwvwwv/scalable-concurrent-containers/src/branch/main/CHANGELOG.md)

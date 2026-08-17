# Scalable Delayed Dealloc

[![Cargo](https://img.shields.io/crates/v/sdd)](https://crates.io/crates/sdd)
![Crates.io](https://img.shields.io/crates/l/sdd)

A scalable lock-free delayed memory reclaimer that emulates garbage collection by keeping track of memory reachability.

The delayed deallocation algorithm is based on a variant of epoch-based reclamation where retired memory chunks are temporarily kept in thread-local storage until they are no longer reachable. It is similar to [`crossbeam_epoch`](https://docs.rs/crossbeam-epoch/), however, users will find `sdd` more straightforward to use as `sdd` provides smart pointer types. For instance, `sdd::AtomicOwned`, `sdd::Owned`, `sdd::AtomicShared`, and `sdd::Shared` retire the contained value when the last reference is dropped.

## Features

* Lock-free epoch-based reclamation.
* [`Loom`](https://crates.io/crates/loom) support: `features = ["loom"]`.

## Examples

This crate can be used _without an `unsafe` block_.

```rust
use sdd::{suspend, AtomicOwned, AtomicShared, Guard, Owned, Ptr, Shared, Tag};
use std::sync::atomic::Ordering::Relaxed;

// `atomic_shared` holds a strong reference to `17`.
let atomic_shared: AtomicShared<usize> = AtomicShared::new(17);

// `atomic_owned` owns `19`.
let atomic_owned: AtomicOwned<usize> = AtomicOwned::new(19);

// `guard` prevents the garbage collector from dropping reachable instances.
let guard = Guard::new();

// `ptr` cannot outlive `guard`.
let mut ptr: Ptr<usize> = atomic_shared.load(Relaxed, &guard);
assert_eq!(*ptr.as_ref().unwrap(), 17);

// `atomic_shared` can be tagged.
atomic_shared.update_tag_if(Tag::First, |p| p.tag() == Tag::None, Relaxed, Relaxed);

// `ptr` is not tagged, so CAS fails.
assert!(atomic_shared.compare_exchange(
    ptr,
    (Some(Shared::new(18)), Tag::First),
    Relaxed,
    Relaxed,
    &guard).is_err());

// `ptr` can be tagged.
ptr.set_tag(Tag::First);

// The ownership of the contained instance is transferred to the return value of CAS.
let prev: Shared<usize> = atomic_shared.compare_exchange(
    ptr,
    (Some(Shared::new(19)), Tag::Second),
    Relaxed,
    Relaxed,
    &guard).unwrap().0.unwrap();
assert_eq!(*prev, 17);

// `17` will be garbage-collected later.
drop(prev);

// `sdd::AtomicShared` can be converted into `sdd::Shared`.
let shared: Shared<usize> = atomic_shared.into_shared(Relaxed).unwrap();
assert_eq!(*shared, 19);

// `18` and `19` will be garbage-collected later.
drop(shared);
drop(atomic_owned);

// `17` is still valid as `guard` keeps the garbage collector from dropping it.
assert_eq!(*ptr.as_ref().unwrap(), 17);

// Execution of a closure can be deferred until all the current readers are gone.
guard.defer_execute(|| println!("deferred"));
drop(guard);

// `sdd::Owned` and `sdd::Shared` can be nested.
let shared_nested: Shared<Owned<Shared<usize>>> = Shared::new(Owned::new(Shared::new(20)));
assert_eq!(***shared_nested, 20);

// If the thread is expected to lie dormant for a while, call `suspend()` to allow
// others to reclaim the memory.
suspend();
```

## Memory Overhead

Retired instances are stored in intrusive queues in thread-local storage, and therefore, additional space for `Option<NonNull<dyn Collectible>>` is allocated per instance.

## Performance

The average time taken to enter and exit a protected region: less than a nanosecond on Apple M4 Pro.

## Applications

[`sdd`](https://crates.io/crates/sdd) provides widely used lock-free concurrent data structures, including [`LinkedList`](#linkedlist), [`Bag`](#bag), [`Queue`](#queue), and [`Stack`](#stack).

### `LinkedList`

[`LinkedList`](#linkedlist) is a trait that implements lock-free concurrent singly linked list operations. It additionally provides a method for marking a linked list entry to denote a user-defined state.

```rust
use std::sync::atomic::Ordering::Relaxed;

use sdd::{AtomicShared, Guard, LinkedList, Shared};

#[derive(Default)]
struct L(AtomicShared<L>, usize);
impl LinkedList for L {
    fn link_ref(&self) -> &AtomicShared<L> {
        &self.0
    }
}

let guard = Guard::new();

let head: L = L::default();
let tail: Shared<L> = Shared::new(L(AtomicShared::null(), 1));

// A new entry is pushed.
assert!(head.push_back(tail.clone(), false, Relaxed, &guard).is_ok());
assert!(!head.is_marked(Relaxed));

// Users can mark a flag on an entry.
head.mark(Relaxed);
assert!(head.is_marked(Relaxed));

// `next_ptr` traverses the linked list.
let next_ptr = head.next_ptr(Relaxed, &guard);
assert_eq!(next_ptr.as_ref().unwrap().1, 1);

// Once `tail` is deleted, it becomes unreachable.
tail.delete_self(Relaxed);
assert!(head.next_ptr(Relaxed, &guard).is_null());
```

### `Bag`

[`Bag`](#bag) is a concurrent lock-free unordered container. [`Bag`](#bag) is completely opaque, disallowing access to contained instances until they are popped. [`Bag`](#bag) is especially efficient if the number of contained instances can be maintained under `ARRAY_LEN (default: usize::BITS / 2)`

```rust
use sdd::Bag;

let bag: Bag<usize> = Bag::default();

bag.push(1);
assert!(!bag.is_empty());
assert_eq!(bag.pop(), Some(1));
assert!(bag.is_empty());
```

### `Queue`

[`Queue`](#queue) is a concurrent lock-free first-in-first-out container.

```rust
use sdd::Queue;

let queue: Queue<usize> = Queue::default();

queue.push(1);
assert!(queue.push_if(2, |e| e.map_or(false, |x| **x == 1)).is_ok());
assert!(queue.push_if(3, |e| e.map_or(false, |x| **x == 1)).is_err());
assert_eq!(queue.pop().map(|e| **e), Some(1));
assert_eq!(queue.pop().map(|e| **e), Some(2));
assert!(queue.pop().is_none());
```

### `Stack`

[`Stack`](#stack) is a concurrent lock-free last-in-first-out container.

```rust
use sdd::Stack;

let stack: Stack<usize> = Stack::default();

stack.push(1);
stack.push(2);
assert_eq!(stack.pop().map(|e| **e), Some(2));
assert_eq!(stack.pop().map(|e| **e), Some(1));
assert!(stack.pop().is_none());
```

## [Changelog](https://codeberg.org/wvwwvwwv/scalable-delayed-dealloc/src/branch/main/CHANGELOG.md)

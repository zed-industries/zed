# Synchronous and Asynchronous Synchronization Primitives

[![Cargo](https://img.shields.io/crates/v/saa)](https://crates.io/crates/saa)
![Crates.io](https://img.shields.io/crates/l/saa)

Word-sized low-level synchronization primitives providing both asynchronous and synchronous interfaces.

## Features

- No heap allocation.
- No hidden global variables.
- Provides both asynchronous and synchronous interfaces.
- [`lock_api`](https://crates.io/crates/lock_api) support: `features = ["lock_api"]`.
- [`Loom`](https://github.com/tokio-rs/loom) support: `features = ["loom"]`.

## Lock

`saa::Lock` is a low-level shared-exclusive lock providing both asynchronous and synchronous interfaces. Synchronous locking methods such as `lock_sync` and `share_sync` can be used alongside their asynchronous counterparts `lock_async` and `share_async` simultaneously. `saa::Lock` implements an allocation-free fair wait queue shared between both synchronous and asynchronous methods.

```rust
use saa::Lock;

// At most `62` concurrent shared owners are allowed.
assert_eq!(Lock::MAX_SHARED_OWNERS, 62);

let lock = Lock::default();

assert!(lock.lock_sync());
assert!(!lock.try_lock());
assert!(!lock.try_share());

assert!(!lock.release_share());
assert!(lock.release_lock());

assert!(lock.lock_sync());

// `Lock` can be poisoned.
assert!(lock.poison_lock());
assert!(!lock.lock_sync());
assert!(lock.clear_poison());

async {
    assert!(lock.share_async().await);
    assert!(lock.release_share());
    assert!(lock.lock_async().await);
    assert!(lock.release_lock());
};
```

### [`lock_api`](https://crates.io/crates/lock_api) support

The `lock_api` feature is automatically disabled when the `loom` feature is enabled since `loom` atomic types cannot be instantiated in const contexts.

```rust
#[cfg(all(feature = "lock_api", not(feature = "loom")))]
use saa::{Mutex, RwLock, lock_async, read_async, write_async};

#[cfg(all(feature = "lock_api", not(feature = "loom")))]
fn example() {
    let mutex: Mutex<usize> = Mutex::new(0);
    let rwlock: RwLock<usize> = RwLock::new(0);

    let mut mutex_guard = mutex.lock();
    assert_eq!(*mutex_guard, 0);
    *mutex_guard += 1;
    assert_eq!(*mutex_guard, 1);
    drop(mutex_guard);

    let mut write_guard = rwlock.write();
    assert_eq!(*write_guard, 0);
    *write_guard += 1;
    drop(write_guard);

    let read_guard = rwlock.read();
    assert_eq!(*read_guard, 1);
    drop(read_guard);

    async {
        let mutex_guard = lock_async(&mutex).await;
        assert_eq!(*mutex_guard, 1);
        drop(mutex_guard);

        let mut write_guard = write_async(&rwlock).await;
        *write_guard += 1;
        drop(write_guard);

        let reader_guard = read_async(&rwlock).await;
        assert_eq!(*reader_guard, 2);
        drop(reader_guard);
    };
}
```

## Barrier

`saa::Barrier` is a synchronization primitive to enable a number of tasks to start execution at the same time.

```rust
use std::sync::Arc;
use std::thread;

use saa::Barrier;

// At most `63` concurrent tasks/threads can be synchronized.
assert_eq!(Barrier::MAX_TASKS, 63);

let barrier = Arc::new(Barrier::with_count(8));

let mut threads = Vec::new();

for _ in 0..8 {
    let barrier = barrier.clone();
    threads.push(thread::spawn(move || {
        for _ in 0..4 {
            barrier.wait_sync();
        }
    }));
}

for thread in threads {
    thread.join().unwrap();
}
```

## Semaphore

`saa::Semaphore` is a synchronization primitive that allows a fixed number of threads to access a resource concurrently.

```rust
use saa::Semaphore;

// At most `63` concurrent tasks/threads can be synchronized.
assert_eq!(Semaphore::MAX_PERMITS, 63);

let semaphore = Semaphore::default();

semaphore.acquire_many_sync(Semaphore::MAX_PERMITS - 1);

assert!(semaphore.try_acquire());
assert!(!semaphore.try_acquire());

assert!(semaphore.release());
assert!(!semaphore.release_many(Semaphore::MAX_PERMITS));
assert!(semaphore.release_many(Semaphore::MAX_PERMITS - 1));

async {
    semaphore.acquire_async().await;
    assert!(semaphore.release());
};
```

## Gate

`saa::Gate` is an unbounded barrier that can be opened or sealed manually as needed.

```rust
use std::sync::Arc;
use std::thread;

use saa::Gate;
use saa::gate::State;

let gate = Arc::new(Gate::default());

let mut threads = Vec::new();

for _ in 0..4 {
    let gate = gate.clone();
    threads.push(thread::spawn(move || {
        assert_eq!(gate.enter_sync(), Ok(State::Controlled));
    }));
}

let mut count = 0;
while count != 4 {
    if let Ok(n) = gate.permit() {
        count += n;
    }
}

for thread in threads {
    thread.join().unwrap();
}
```

## Pager

`saa::Pager` enables remotely waiting for a resource to become available.

```rust
use std::pin::pin;

use saa::{Gate, Pager};
use saa::gate::State;

let gate = Gate::default();

let mut pinned_pager = pin!(Pager::default());

assert!(gate.register_pager(&mut pinned_pager, true));
assert_eq!(gate.open().1, 1);

assert_eq!(pinned_pager.poll_sync(), Ok(State::Open));
```

## Notes

Using synchronous methods in an asynchronous context may lead to deadlocks. Consider a scenario where an asynchronous runtime uses two threads to execute three tasks.

* ThreadId(0): `task-0: share-waiting / pending` || `task-1: "synchronous"-lock-waiting`.
* ThreadId(1): `task-2: release-lock / ready: wake-up task-0` -> `task-2: lock-waiting / pending`.

In this example, `task-0` has logically acquired a shared lock transferred from `task-2`; however, it may remain in the task queue indefinitely depending on the task scheduling policy.

## [Changelog](https://codeberg.org/wvwwvwwv/synchronous-and-asynchronous/src/branch/main/CHANGELOG.md)

//! Synchronization primitives and utilities based on intrusive collections.
//!
//! This crate provides a variety of `Futures`-based and `async/await` compatible
//! types that are based on the idea of intrusive collections:
//! - Channels in a variety of flavors:
//!   - Oneshot
//!   - Multi-Producer Multi-Consumer (MPMC)
//!   - State Broadcast
//! - Synchronization Primitives:
//!   - Manual Reset Event
//!   - Mutex
//!   - Semaphore
//! - A timer
//!
//! ## Intrusive collections?
//!
//! In an intrusive collection, the elements that want to get stored inside the
//! collection provide the means to store themselves inside the collection.
//! E.g. in an intrusive linked list, each element that gets stored inside the
//! list contains a pointer field that points to the next list element. E.g.
//!
//! ```
//! // The element which is intended to be stored inside an intrusive container
//! struct ListElement {
//!    data: u32,
//!    next: *mut ListElement,
//! }
//!
//! // The intrusive container
//! struct List {
//!     head: *mut ListElement,
//! }
//! ```
//!
//! The advantage here is that the intrusive collection (here: the list) requires
//! only a fixed amount of memory. In this case it only needs a pointer to the
//! first element.
//!
//! The list container itself has a fixed size of a single pointer independent
//! of the number of stored elements.
//!
//! Intrusive lists are often used in low-level code like in operating system
//! kernels.  E.g. they can be used for storing elements that represent threads
//! that are blocked and waiting on queue.  In that case the stored elements can
//! be on the call stack of the caller of each blocked thread, since the
//! call stack won't change as long as the thread is blocked.
//!
//! ### Application in Futures
//!
//! This library brings this idea into the world of Rusts `Future`s. Due to the
//! addition of `Pin`ning, the address of a certain `Future` is not allowed to
//! change between the first call to `poll()` and when the `Future` is dropped.
//! This means the data inside the `Future` itself can be inserted into an
//! intrusive container. If the the call to `Future::poll()` is not immedately
//! ready, some parts of the `Future` itself are registered in the type which
//! yielded the `Future`. Each `Future` can store a `Waker`. When the original
//! type becomes ready, it can iterate through the list of registered `Future`s,
//! wakeup associated tasks, and potentially remove them from its queue.
//!
//! The result is that the future-yielding type is not required to copy an
//! arbitrary number of `Waker` objects into itself, and thereby does not require
//! dynamic memory for this task.
//!
//! When a `Future` gets destructed/dropped, it must make sure to remove itself
//! from any collections that refer to it to avoid invalid memory accesses.
//!
//! This library implements common synchronization primitives for the usage in
//! asychronous code based on this concept.
//!
//! The implementation requires the usage of a fair chunk of `unsafe`
//! annotations. However the provided user-level API is intended to be fully safe.
//!
//! ## Features of this library
//!
//! The following types are currently implemented:
//! - Channels (oneshot and multi-producer-multi-consumer)
//! - Synchronization primitives (async mutexes and events)
//! - Timers
//!
//! ## Design goals for the library
//!
//! - Provide implementations of common synchronization primitives in a platform
//!   independent fashion.
//! - Support `no-std` environments. As many types as possible are also provided
//!   for `no-std` environments. The library should boost the ability to use
//!   async Rust code in environments like:
//!   - Microcontrollers (RTOS and bare-metal)
//!   - Kernels
//!   - Drivers
//! - Avoid dynamic memory allocations at runtime.  After objects from this
//!   library have been created, they should not require allocation of any
//!   further memory at runtime.  E.g. they should not need to allocate memory
//!   for each call to an asynchronous function or each time a new task accesses
//!   the same object in parallel.
//! - Offer familiar APIs.
//!   The library tries to mimic the APIs of existing Rust libraries like the
//!   standard library and `futures-rs` as closely as possible.
//!
//! ## Non goals
//!
//! - Provide IO primitives (like sockets), or platform specific implementations.
//! - Reach the highest possible performance in terms of throughput and latency.
//!   While code in this library is optimized for performance, portability
//!   and deterministic memory usage are more important goals.
//! - Provide future wrappers for platform-specific APIs.
//!
//! ## Local, Non-local and shared flavors
//!
//! The library provides types in a variety of flavors:
//!
//! - A local flavor (e.g. [`channel::LocalChannel`])
//! - A non-local flavor (e.g. [`channel::Channel`])
//! - A shared flavor (e.g. [`channel::shared::Sender`])
//! - A generic flavor (e.g. [`channel::GenericChannel`] and
//!   [`channel::shared::GenericSender`])
//!
//! The difference between these types lie in their thread-safety. The non-local
//! flavors of types can be accessed from multiple threads (and thereby also
//! futures tasks) concurrently. This means they implement the `Sync` trait in
//! addition to the `Send` trait.
//! The local flavors only implement the `Send` trait.
//!
//! ### Local flavor
//!
//! The local flavors will require no internal synchronization (e.g. internal
//! Mutexes) and can therefore be provided for all platforms (including `no-std`).
//! Due the lack of required synchronization, they are also very fast.
//!
//! It might seem counter-intuitive to provide synchronization primitives that
//! only work within a single task. However there are a variety of applications
//! where these can be used to coordinate sub-tasks (futures that are polled on
//! a single task concurrently).
//!
//! The following example demonstrates this use-case:
//!
//! ```
//! # use futures::join;
//! # use futures_intrusive::sync::LocalManualResetEvent;
//! async fn async_fn() {
//!     let event = LocalManualResetEvent::new(false);
//!     let task_a = async {
//!         // Wait for the event
//!         event.wait().await;
//!         // Do something with the knowledge that task_b reached a certain state
//!     };
//!     let task_b = async {
//!         // Some complex asynchronous workflow here
//!         // ...
//!         // Signal task_a
//!         event.set();
//!     };
//!     join!(task_a, task_b);
//! }
//! ```
//!
//! ### Non-local flavor
//!
//! The non-local flavors can be used between arbitrary tasks and threads.  They
//! use internal synchronization for this in form of an embedded `Mutex` of
//! [`parking_lot::Mutex`] type.
//!
//! The non-local flavors are only available in `alloc` environments.
//!
//! ### Shared flavor
//!
//! For some types a shared flavor is provided. Non-local flavors of types are
//! `Sync`, but they still can only be shared by reference between various tasks.

//! Shared flavors are also `Sync`, but the types additionally implement the
//! `Clone` trait, which allows duplicating the object, and passing ownership of
//! it to a different task. These types allow avoiding references (and thereby
//! lifetimes) in some scenarios, which makes them more convenient to use.  The
//! types also return `Future`s which do not have an associated lifetime.  This
//! allows using those types as implementations of traits without the need for
//! generic associated types (GATs).
//!
//! Due to the requirement of atomic reference counting, these types are
//! currently only available for `alloc` environments.
//!
//! ### Generic flavor
//!
//! The generic flavors of provided types are parameterized around a
//! [`lock_api::RawMutex`] type. These form the base for the non-local and shared
//! flavors which simply parameterize the generic flavor in either a
//! non-thread-safe or thread-safe fashion.
//!
//! Users can directly use the generic flavors to adapt the provided thread-safe
//! types for use in `no-std` environments.
//!
//! E.g. by providing a custom [`lock_api::RawMutex`]
//! implementation, the following platforms can be supported:
//!
//! - For RTOS platforms, RTOS-specific mutexes can be wrapped.
//! - For kernel development, spinlock based mutexes can be created.
//! - For embedded development, mutexes which just disable interrupts can be
//!   utilized.
//!
//!
//! ## Relation to types in other libraries
//!
//! Other libraries (e.g. `futures-rs` and `tokio`) provide many primitives that
//! are comparable feature-wise to the types in this library.
//!
//! The most important differences are:
//! - This library has a bigger focus on `no-std` environments, and does not
//!   only try to provide an implementation for `alloc` or `std`.
//! - The types in this library do not require dynamic memory allocation for
//!   waking up an arbitrary number of tasks waiting on a particular
//!   `Future`. Other libraries typically require heap-allocated nodes of
//!   growing vectors for handling a varying number of tasks.
//! - The `Future`s produced by this library are all `!Unpin`, which might make
//!   them less ergonomic to use.
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, missing_debug_implementations)]
#![deny(bare_trait_objects)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod noop_lock;
use noop_lock::NoopLock;

pub mod buffer;

#[allow(dead_code)]
mod intrusive_double_linked_list;
mod intrusive_pairing_heap;

pub mod channel;
pub mod sync;
pub mod timer;

mod utils;

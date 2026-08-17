//! Types and traits for working with asynchronous tasks.
//!
//! This module is similar to [`std::thread`], except it uses asynchronous tasks in place of
//! threads.
//!
//! [`std::thread`]: https://doc.rust-lang.org/std/thread
//!
//! ## The task model
//!
//! An executing asynchronous Rust program consists of a collection of native OS threads, on top of
//! which multiple stackless coroutines are multiplexed. We refer to these as "tasks".  Tasks can
//! be named, and provide some built-in support for synchronization.
//!
//! Communication between tasks can be done through channels, Rust's message-passing types, along
//! with [other forms of tasks synchronization](../sync/index.html) and shared-memory data
//! structures. In particular, types that are guaranteed to be threadsafe are easily shared between
//! tasks using the atomically-reference-counted container, [`Arc`].
//!
//! Fatal logic errors in Rust cause *thread panic*, during which a thread will unwind the stack,
//! running destructors and freeing owned resources. If a panic occurs inside a task, there is no
//! meaningful way of recovering, so the panic will propagate through any thread boundaries all the
//! way to the root task. This is also known as a "panic = abort" model.
//!
//! ## Spawning a task
//!
//! A new task can be spawned using the [`task::spawn`][`spawn`] function:
//!
//! ```no_run
//! use async_std::task;
//!
//! task::spawn(async {
//!     // some work here
//! });
//! ```
//!
//! In this example, the spawned task is "detached" from the current task. This means that it can
//! outlive its parent (the task that spawned it), unless this parent is the root task.
//!
//! The root task can also wait on the completion of the child task; a call to [`spawn`] produces a
//! [`JoinHandle`], which implements `Future` and can be `await`ed:
//!
//! ```
//! use async_std::task;
//!
//! # async_std::task::block_on(async {
//! #
//! let child = task::spawn(async {
//!     // some work here
//! });
//! // some work here
//! let res = child.await;
//! #
//! # })
//! ```
//!
//! The `await` operator returns the final value produced by the child task.
//!
//! ## Configuring tasks
//!
//! A new task can be configured before it is spawned via the [`Builder`] type,
//! which currently allows you to set the name for the child task:
//!
//! ```
//! # #![allow(unused_must_use)]
//! use async_std::task;
//!
//! # async_std::task::block_on(async {
//! #
//! task::Builder::new().name("child1".to_string()).spawn(async {
//!     println!("Hello, world!");
//! });
//! #
//! # })
//! ```
//!
//! ## The `Task` type
//!
//! Tasks are represented via the [`Task`] type, which you can get in one of
//! two ways:
//!
//! * By spawning a new task, e.g., using the [`task::spawn`][`spawn`]
//!   function, and calling [`task`][`JoinHandle::task`] on the [`JoinHandle`].
//! * By requesting the current task, using the [`task::current`] function.
//!
//! ## Task-local storage
//!
//! This module also provides an implementation of task-local storage for Rust
//! programs. Task-local storage is a method of storing data into a global
//! variable that each task in the program will have its own copy of.
//! Tasks do not share this data, so accesses do not need to be synchronized.
//!
//! A task-local key owns the value it contains and will destroy the value when the
//! task exits. It is created with the [`task_local!`] macro and can contain any
//! value that is `'static` (no borrowed pointers). It provides an accessor function,
//! [`with`], that yields a shared reference to the value to the specified
//! closure. Task-local keys allow only shared access to values, as there would be no
//! way to guarantee uniqueness if mutable borrows were allowed.
//!
//! ## Naming tasks
//!
//! Tasks are able to have associated names for identification purposes. By default, spawned
//! tasks are unnamed. To specify a name for a task, build the task with [`Builder`] and pass
//! the desired task name to [`Builder::name`]. To retrieve the task name from within the
//! task, use [`Task::name`].
//!
//! [`Arc`]: ../sync/struct.Arc.html
//! [`spawn`]: fn.spawn.html
//! [`JoinHandle`]: struct.JoinHandle.html
//! [`JoinHandle::task`]: struct.JoinHandle.html#method.task
//! [`join`]: struct.JoinHandle.html#method.join
//! [`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
//! [`Builder`]: struct.Builder.html
//! [`Builder::name`]: struct.Builder.html#method.name
//! [`task::current`]: fn.current.html
//! [`Task`]: struct.Task.html
//! [`Task::name`]: struct.Task.html#method.name
//! [`task_local!`]: ../macro.task_local.html
//! [`with`]: struct.LocalKey.html#method.with

cfg_alloc! {
    #[doc(inline)]
    pub use core::task::{Context, Poll, Waker};
    pub use ready::ready;

    mod ready;
}

cfg_std! {
    pub use yield_now::yield_now;
    mod yield_now;
}

cfg_default! {
    pub use block_on::block_on;
    pub use builder::Builder;
    pub use current::{current, try_current};
    pub use task::Task;
    pub use task_id::TaskId;
    pub use join_handle::JoinHandle;
    pub use sleep::sleep;
    #[cfg(not(target_os = "unknown"))]
    pub use spawn::spawn;
    pub use task_local::{AccessError, LocalKey};

    pub(crate) use task_local::LocalsMap;
    pub(crate) use task_locals_wrapper::TaskLocalsWrapper;

    mod block_on;
    mod builder;
    mod current;
    mod join_handle;
    mod sleep;
    #[cfg(not(target_os = "unknown"))]
    mod spawn;
    #[cfg(not(target_os = "unknown"))]
    mod spawn_blocking;
    mod task;
    mod task_id;
    mod task_local;
    mod task_locals_wrapper;

    #[cfg(not(target_os = "unknown"))]
    pub use spawn_blocking::spawn_blocking;
}

cfg_unstable! {
    #[cfg(feature = "default")]
    pub use spawn_local::spawn_local;

    #[cfg(feature = "default")]
    mod spawn_local;
}

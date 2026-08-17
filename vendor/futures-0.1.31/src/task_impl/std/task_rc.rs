#![cfg(feature = "with-deprecated")]
#![allow(deprecated)]
#![deprecated(since = "0.1.4",
              note = "replaced with `BiLock` in many cases, otherwise slated \
                      for removal due to confusion")]

use std::prelude::v1::*;
use std::sync::Arc;
use std::cell::UnsafeCell;
use task_impl;

// One critical piece of this module's contents are the `TaskRc<A>` handles.
// The purpose of this is to conceptually be able to store data in a task,
// allowing it to be accessed within multiple futures at once. For example if
// you have some concurrent futures working, they may all want mutable access to
// some data. We already know that when the futures are being poll'd that we're
// entirely synchronized (aka `&mut Task`), so you shouldn't require an
// `Arc<Mutex<T>>` to share as the synchronization isn't necessary!
//
// So the idea here is that you insert data into a task via `Task::insert`, and
// a handle to that data is then returned to you. That handle can later get
// presented to the task itself to actually retrieve the underlying data. The
// invariant is that the data can only ever be accessed with the task present,
// and the lifetime of the actual data returned is connected to the lifetime of
// the task itself.
//
// Conceptually I at least like to think of this as "dynamically adding more
// struct fields to a `Task`". Each call to insert creates a new "name" for the
// struct field, a `TaskRc<A>`, and then you can access the fields of a struct
// with the struct itself (`Task`) as well as the name of the field
// (`TaskRc<A>`). If that analogy doesn't make sense then oh well, it at least
// helped me!
//
// So anyway, we do some interesting trickery here to actually get it to work.
// Each `TaskRc<A>` handle stores `Arc<UnsafeCell<A>>`. So it turns out, we're
// not even adding data to the `Task`! Each `TaskRc<A>` contains a reference
// to this `Arc`, and `TaskRc` handles can be cloned which just bumps the
// reference count on the `Arc` itself.
//
// As before, though, you can present the `Arc` to a `Task` and if they
// originated from the same place you're allowed safe access to the internals.
// We allow but shared and mutable access without the `Sync` bound on the data,
// crucially noting that a `Task` itself is not `Sync`.
//
// So hopefully I've convinced you of this point that the `get` and `get_mut`
// methods below are indeed safe. The data is always valid as it's stored in an
// `Arc`, and access is only allowed with the proof of the associated `Task`.
// One thing you might be asking yourself though is what exactly is this "proof
// of a task"? Right now it's a `usize` corresponding to the `Task`'s
// `TaskHandle` arc allocation.
//
// Wait a minute, isn't that the ABA problem! That is, we create a task A, add
// some data to it, destroy task A, do some work, create a task B, and then ask
// to get the data from task B. In this case though the point of the
// `task_inner` "proof" field is simply that there's some non-`Sync` token
// proving that you can get access to the data. So while weird, this case should
// still be safe, as the data's not stored in the task itself.

/// A reference to a piece of data that's accessible only within a specific
/// `Task`.
///
/// This data is `Send` even when `A` is not `Sync`, because the data stored
/// within is accessed in a single-threaded way. The thread accessing it may
/// change over time, if the task migrates, so `A` must be `Send`.
#[derive(Debug)]
pub struct TaskRc<A> {
    task: task_impl::Task,
    ptr: Arc<UnsafeCell<A>>,
}

// for safety here, see docs at the top of this module
unsafe impl<A: Send> Send for TaskRc<A> {}
unsafe impl<A: Sync> Sync for TaskRc<A> {}

impl<A> TaskRc<A> {
    /// Inserts a new piece of task-local data into this task, returning a
    /// reference to it.
    ///
    /// Ownership of the data will be transferred to the task, and the data will
    /// be destroyed when the task itself is destroyed. The returned value can
    /// be passed to the `with` method to get a reference back to the original
    /// data.
    ///
    /// Note that the returned handle is cloneable and copyable and can be sent
    /// to other futures which will be associated with the same task. All
    /// futures will then have access to this data when passed the reference
    /// back.
    ///
    /// # Panics
    ///
    /// This function will panic if a task is not currently running.
    pub fn new(a: A) -> TaskRc<A> {
        TaskRc {
            task: task_impl::park(),
            ptr: Arc::new(UnsafeCell::new(a)),
        }
    }

    /// Operate with a reference to the underlying data.
    ///
    /// This method should be passed a handle previously returned by
    /// `Task::insert`. That handle, when passed back into this method, will
    /// retrieve a reference to the original data.
    ///
    /// # Panics
    ///
    /// This method will panic if a task is not currently running or if `self`
    /// does not belong to the task that is currently running. That is, if
    /// another task generated the `data` handle passed in, this method will
    /// panic.
    pub fn with<F, R>(&self, f: F) -> R
        where F: FnOnce(&A) -> R
    {
        if !self.task.is_current() {
            panic!("TaskRc being accessed on task it does not belong to");
        }

        f(unsafe { &*self.ptr.get() })
    }
}

impl<A> Clone for TaskRc<A> {
    fn clone(&self) -> TaskRc<A> {
        TaskRc {
            task: self.task.clone(),
            ptr: self.ptr.clone(),
        }
    }
}

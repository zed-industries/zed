use crate::runtime::context;

use std::{fmt, num::NonZeroU64};

/// An opaque ID that uniquely identifies a task relative to all other currently
/// running tasks.
///
/// A task's ID may be re-used for another task only once *both* of the
/// following happen:
/// 1. The task itself exits.
/// 2. There is no active [`JoinHandle`] associated with this task.
///
/// A [`JoinHandle`] is considered active in the following situations:
/// - You are explicitly holding a [`JoinHandle`], [`AbortHandle`], or
///   `tokio_util::task::AbortOnDropHandle`.
/// - The task is being tracked by a [`JoinSet`] or `tokio_util::task::JoinMap`.
///
/// # Notes
///
/// - Task IDs are *not* sequential, and do not indicate the order in which
///   tasks are spawned, what runtime a task is spawned on, or any other data.
/// - The task ID of the currently running task can be obtained from inside the
///   task via the [`task::try_id()`](crate::task::try_id()) and
///   [`task::id()`](crate::task::id()) functions and from outside the task via
///   the [`JoinHandle::id()`](crate::task::JoinHandle::id()) function.
///
/// [`JoinHandle`]: crate::task::JoinHandle
/// [`AbortHandle`]: crate::task::AbortHandle
/// [`JoinSet`]: crate::task::JoinSet
#[cfg_attr(docsrs, doc(cfg(all(feature = "rt"))))]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Id(pub(crate) NonZeroU64);

/// Returns the [`Id`] of the currently running task.
///
/// # Panics
///
/// This function panics if called from outside a task. Please note that calls
/// to `block_on` do not have task IDs, so the method will panic if called from
/// within a call to `block_on`. For a version of this function that doesn't
/// panic, see [`task::try_id()`](crate::runtime::task::try_id()).
///
/// [task ID]: crate::task::Id
#[track_caller]
pub fn id() -> Id {
    context::current_task_id().expect("Can't get a task id when not inside a task")
}

/// Returns the [`Id`] of the currently running task, or `None` if called outside
/// of a task.
///
/// This function is similar to  [`task::id()`](crate::runtime::task::id()), except
/// that it returns `None` rather than panicking if called outside of a task
/// context.
///
/// [task ID]: crate::task::Id
#[track_caller]
pub fn try_id() -> Option<Id> {
    context::current_task_id()
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Id {
    pub(crate) fn next() -> Self {
        use crate::loom::sync::atomic::{AtomicU64, Ordering::Relaxed};

        #[cfg(all(test, loom))]
        crate::loom::lazy_static! {
            static ref NEXT_ID: AtomicU64 = AtomicU64::new(1);
        }

        #[cfg(not(all(test, loom)))]
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);

        loop {
            let id = NEXT_ID.fetch_add(1, Relaxed);
            if let Some(id) = NonZeroU64::new(id) {
                return Self(id);
            }
        }
    }

    pub(crate) fn as_u64(&self) -> u64 {
        self.0.get()
    }
}

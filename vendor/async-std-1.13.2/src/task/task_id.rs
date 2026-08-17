use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A unique identifier for a task.
///
/// # Examples
///
/// ```
/// use async_std::task;
///
/// task::block_on(async {
///     println!("id = {:?}", task::current().id());
/// })
/// ```
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct TaskId(pub(crate) usize);

impl TaskId {
    /// Generates a new `TaskId`.
    pub(crate) fn generate() -> TaskId {
        // TODO: find a good version to emulate u64 atomics on 32 bit systems.
        static COUNTER: AtomicUsize = AtomicUsize::new(1);

        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        if id > usize::max_value() / 2 {
            std::process::abort();
        }
        TaskId(id)
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

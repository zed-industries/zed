use crate::task::{Task, TaskLocalsWrapper};

/// Returns a handle to the current task.
///
/// # Panics
///
/// This function will panic if not called within the context of a task created by [`block_on`],
/// [`spawn`], or [`Builder::spawn`].
///
/// [`block_on`]: fn.block_on.html
/// [`spawn`]: fn.spawn.html
/// [`Builder::spawn`]: struct.Builder.html#method.spawn
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::task;
///
/// println!("The name of this task is {:?}", task::current().name());
/// #
/// # })
/// ```
pub fn current() -> Task {
    try_current().expect("`task::current()` called outside the context of a task")
}

/// Returns a handle to the current task if called within the context of a task created by [`block_on`],
/// [`spawn`], or [`Builder::spawn`], otherwise returns `None`.
///
/// [`block_on`]: fn.block_on.html
/// [`spawn`]: fn.spawn.html
/// [`Builder::spawn`]: struct.Builder.html#method.spawn
///
/// # Examples
///
/// ```
/// use async_std::task;
///
/// match task::try_current() {
///     Some(t) => println!("The name of this task is {:?}", t.name()),
///     None    => println!("Not inside a task!"),
/// }
/// ```
pub fn try_current() -> Option<Task> {
    TaskLocalsWrapper::get_current(|t| t.task().clone())
}
use std::future::Future;

use crate::task::Builder;

/// Spawns a task and blocks the current thread on its result.
///
/// Calling this function is similar to [spawning] a thread and immediately [joining] it, except an
/// asynchronous task will be spawned.
///
/// See also: [`task::spawn_blocking`].
///
/// [`task::spawn_blocking`]: fn.spawn_blocking.html
///
/// [spawning]: https://doc.rust-lang.org/std/thread/fn.spawn.html
/// [joining]: https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join
///
/// # Examples
///
/// ```no_run
/// use async_std::task;
///
/// task::block_on(async {
///     println!("Hello, world!");
/// })
/// ```
#[cfg(not(target_os = "unknown"))]
pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    Builder::new().blocking(future)
}

/// Spawns a task and waits for it to finish.
#[cfg(target_os = "unknown")]
pub fn block_on<F, T>(future: F)
where
    F: Future<Output = T> + 'static,
    T: 'static,
{
    Builder::new().local(future).unwrap();
}

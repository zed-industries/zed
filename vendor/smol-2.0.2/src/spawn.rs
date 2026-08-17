use std::future::Future;
use std::panic::catch_unwind;
use std::thread;

use async_executor::{Executor, Task};
use async_io::block_on;
use async_lock::OnceCell;
use futures_lite::future;

/// Spawns a task onto the global executor (single-threaded by default).
///
/// There is a global executor that gets lazily initialized on first use. It is included in this
/// library for convenience when writing unit tests and small programs, but it is otherwise
/// more advisable to create your own [`Executor`].
///
/// By default, the global executor is run by a single background thread, but you can also
/// configure the number of threads by setting the `SMOL_THREADS` environment variable.
///
/// Since the executor is kept around forever, `drop` is not called for tasks when the program
/// exits.
///
/// # Examples
///
/// ```
/// let task = smol::spawn(async {
///     1 + 2
/// });
///
/// smol::block_on(async {
///     assert_eq!(task.await, 3);
/// });
/// ```
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
    static GLOBAL: OnceCell<Executor<'_>> = OnceCell::new();

    fn global() -> &'static Executor<'static> {
        GLOBAL.get_or_init_blocking(|| {
            let num_threads = {
                // Parse SMOL_THREADS or default to 1.
                std::env::var("SMOL_THREADS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1)
            };

            for n in 1..=num_threads {
                thread::Builder::new()
                    .name(format!("smol-{}", n))
                    .spawn(|| loop {
                        catch_unwind(|| block_on(global().run(future::pending::<()>()))).ok();
                    })
                    .expect("cannot spawn executor thread");
            }

            // Prevent spawning another thread by running the process driver on this thread.
            let ex = Executor::new();
            #[cfg(not(target_os = "espidf"))]
            ex.spawn(async_process::driver()).detach();
            ex
        })
    }

    global().spawn(future)
}

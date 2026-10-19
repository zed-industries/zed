use std::{
    collections::VecDeque,
    mem,
    num::NonZeroUsize,
    panic::{self, AssertUnwindSafe},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use async_task::Runnable;
use parking_lot::{Condvar, Mutex};

use crate::RunnableMeta;

const DEFAULT_MAX_THREADS: NonZeroUsize = NonZeroUsize::new(32).unwrap();
const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_millis(500);
const WORKER_THREAD_NAME: &str = "blocking-worker";

/// Runs blocking work on lazily spawned, reusable OS threads.
///
/// This is the analogue of the thread pool behind `smol::unblock` and
/// `tokio::task::spawn_blocking`: workers are spawned on demand whenever queued
/// work exceeds the number of idle workers, up to a fixed limit, and retire
/// after sitting idle for a while. Work beyond the limit waits in a FIFO queue.
///
/// Every runnable is handed to the callback passed to [`BlockingPool::new`],
/// which must run it synchronously. Panics escaping the callback are contained
/// at the worker boundary so a single failing task cannot shrink the pool; they
/// are still reported through the panic hook.
///
/// Dropping the pool does not join its workers. Already queued runnables still
/// run, after which the workers exit.
///
/// # Examples
///
/// ```
/// let pool = gpui::BlockingPool::new(|runnable| {
///     runnable.run();
/// });
/// ```
pub struct BlockingPool {
    shared: Arc<Shared>,
}

struct Shared {
    state: Mutex<State>,
    work_available: Condvar,
    run: Box<dyn Fn(Runnable<RunnableMeta>) + Send + Sync>,
    max_threads: NonZeroUsize,
    idle_timeout: Duration,
}

struct State {
    queue: VecDeque<Runnable<RunnableMeta>>,
    workers: usize,
    idle_workers: usize,
    shutting_down: bool,
}

impl BlockingPool {
    /// Creates a pool with up to 32 workers that retire after 500 milliseconds of idleness.
    ///
    /// No threads are spawned until work is scheduled.
    ///
    /// # Examples
    ///
    /// ```
    /// let pool = gpui::BlockingPool::new(|runnable| {
    ///     runnable.run();
    /// });
    /// ```
    pub fn new(run: impl Fn(Runnable<RunnableMeta>) + Send + Sync + 'static) -> Self {
        Self::with_limits(DEFAULT_MAX_THREADS, DEFAULT_IDLE_TIMEOUT, run)
    }

    fn with_limits(
        max_threads: NonZeroUsize,
        idle_timeout: Duration,
        run: impl Fn(Runnable<RunnableMeta>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            shared: Arc::new(Shared {
                state: Mutex::new(State {
                    queue: VecDeque::new(),
                    workers: 0,
                    idle_workers: 0,
                    shutting_down: false,
                }),
                work_available: Condvar::new(),
                run: Box::new(run),
                max_threads,
                idle_timeout,
            }),
        }
    }

    /// Queues a runnable, spawning a worker if no idle worker can pick it up.
    ///
    /// # Panics
    ///
    /// Panics if a worker is needed but the OS refuses to spawn a thread. The
    /// runnable is dropped without being queued in that case, which cancels its
    /// task.
    ///
    /// # Examples
    ///
    /// ```
    /// let pool = gpui::BlockingPool::new(|runnable| {
    ///     runnable.run();
    /// });
    /// let (runnable, task) = async_task::Builder::new()
    ///     .metadata(gpui::RunnableMeta::new_with_callers_location())
    ///     .spawn(|_| async {}, |_| {});
    /// pool.schedule(runnable);
    /// task.detach();
    /// ```
    pub fn schedule(&self, runnable: Runnable<RunnableMeta>) {
        let mut state = self.shared.state.lock();
        let needs_worker = state.queue.len() >= state.idle_workers
            && state.workers < self.shared.max_threads.get();
        if needs_worker {
            let shared = self.shared.clone();
            let spawned = thread::Builder::new()
                .name(WORKER_THREAD_NAME.into())
                .spawn(move || shared.work());
            if let Err(error) = spawned {
                drop(state);
                panic!("failed to spawn blocking worker thread: {error}");
            }
            // The worker can't take the lock until we release it, so it's counted
            // as idle up front. This keeps concurrent `schedule` calls from
            // spawning another thread for work this worker is about to pick up.
            state.workers += 1;
            state.idle_workers += 1;
        }
        state.queue.push_back(runnable);
        drop(state);
        self.shared.work_available.notify_one();
    }
}

impl Drop for BlockingPool {
    fn drop(&mut self) {
        self.shared.state.lock().shutting_down = true;
        self.shared.work_available.notify_all();
    }
}

impl Shared {
    fn work(&self) {
        let mut state = self.state.lock();
        let mut idle_deadline = Instant::now() + self.idle_timeout;
        loop {
            if let Some(runnable) = state.queue.pop_front() {
                state.idle_workers -= 1;
                drop(state);
                self.run_contained(runnable);
                state = self.state.lock();
                state.idle_workers += 1;
                idle_deadline = Instant::now() + self.idle_timeout;
                continue;
            }
            if state.shutting_down || Instant::now() >= idle_deadline {
                break;
            }
            self.work_available.wait_until(&mut state, idle_deadline);
        }
        state.idle_workers -= 1;
        state.workers -= 1;
    }

    fn run_contained(&self, runnable: Runnable<RunnableMeta>) {
        // The panic hook has already reported the panic; catching it only keeps
        // this worker alive and the pool's bookkeeping intact.
        let Err(payload) = panic::catch_unwind(AssertUnwindSafe(|| (self.run)(runnable))) else {
            return;
        };
        // Dropping a payload runs arbitrary user code, which may itself panic.
        if let Err(payload) = panic::catch_unwind(AssertUnwindSafe(|| drop(payload))) {
            mem::forget(payload);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_task::Task;
    use flume::{Receiver, Sender};
    use std::sync::atomic::{AtomicUsize, Ordering};

    const TIMEOUT: Duration = Duration::from_secs(10);

    fn pool(max_threads: usize, idle_timeout: Duration) -> BlockingPool {
        BlockingPool::with_limits(
            NonZeroUsize::new(max_threads).expect("max_threads is non-zero"),
            idle_timeout,
            |runnable| {
                runnable.run();
            },
        )
    }

    fn runnable(
        work: impl FnOnce() + Send + 'static,
    ) -> (Runnable<RunnableMeta>, Task<(), RunnableMeta>) {
        async_task::Builder::new()
            .metadata(RunnableMeta::new_with_callers_location())
            .spawn(
                move |_| async move { work() },
                |_| panic!("blocking work should never be rescheduled"),
            )
    }

    fn submit(pool: &BlockingPool, work: impl FnOnce() + Send + 'static) {
        let (runnable, task) = runnable(work);
        task.detach();
        pool.schedule(runnable);
    }

    fn submit_reporting_thread(pool: &BlockingPool) -> Receiver<thread::ThreadId> {
        let (sender, receiver) = flume::bounded(1);
        submit(pool, move || {
            sender
                .send(thread::current().id())
                .expect("test is listening");
        });
        receiver
    }

    /// Occupies one worker until the returned sender is used.
    fn occupy_worker(pool: &BlockingPool) -> Sender<()> {
        let (started_sender, started_receiver) = flume::bounded(1);
        let (release_sender, release_receiver) = flume::bounded(1);
        submit(pool, move || {
            started_sender.send(()).expect("test is listening");
            release_receiver
                .recv_timeout(TIMEOUT)
                .expect("test releases the worker");
        });
        started_receiver
            .recv_timeout(TIMEOUT)
            .expect("worker started");
        release_sender
    }

    fn wait_for_state(shared: &Shared, predicate: impl Fn(&State) -> bool) {
        let deadline = Instant::now() + TIMEOUT;
        while !predicate(&shared.state.lock()) {
            assert!(
                Instant::now() < deadline,
                "timed out waiting for pool state"
            );
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn runs_work_on_named_worker_thread() {
        let pool = pool(1, TIMEOUT);
        let (sender, receiver) = flume::bounded(1);
        submit(&pool, move || {
            let thread = thread::current();
            sender
                .send((thread.id(), thread.name().map(str::to_owned)))
                .expect("test is listening");
        });
        let (thread_id, thread_name) = receiver.recv_timeout(TIMEOUT).expect("work ran");
        assert_ne!(thread_id, thread::current().id());
        assert_eq!(thread_name.as_deref(), Some(WORKER_THREAD_NAME));
    }

    #[test]
    fn spawns_lazily() {
        let pool = pool(4, TIMEOUT);
        assert_eq!(pool.shared.state.lock().workers, 0);
    }

    #[test]
    fn caps_workers_and_drains_backlog() {
        let pool = pool(2, TIMEOUT);
        let first = occupy_worker(&pool);
        let second = occupy_worker(&pool);
        let (sender, receiver) = flume::unbounded();
        for _ in 0..8 {
            let sender = sender.clone();
            submit(&pool, move || sender.send(()).expect("test is listening"));
        }
        {
            let state = pool.shared.state.lock();
            assert_eq!(state.workers, 2);
            assert_eq!(state.idle_workers, 0);
            assert_eq!(state.queue.len(), 8);
        }
        first.send(()).expect("worker is waiting");
        second.send(()).expect("worker is waiting");
        for _ in 0..8 {
            receiver.recv_timeout(TIMEOUT).expect("queued work ran");
        }
        assert_eq!(pool.shared.state.lock().workers, 2);
    }

    #[test]
    fn spawns_worker_when_existing_workers_are_busy() {
        let pool = pool(4, TIMEOUT);
        let release = occupy_worker(&pool);
        submit_reporting_thread(&pool)
            .recv_timeout(TIMEOUT)
            .expect("work ran while the other worker was busy");
        assert_eq!(pool.shared.state.lock().workers, 2);
        release.send(()).expect("worker is waiting");
    }

    #[test]
    fn reuses_idle_worker() {
        let pool = pool(4, TIMEOUT);
        let first = submit_reporting_thread(&pool)
            .recv_timeout(TIMEOUT)
            .expect("first work ran");
        wait_for_state(&pool.shared, |state| state.idle_workers == 1);
        let second = submit_reporting_thread(&pool)
            .recv_timeout(TIMEOUT)
            .expect("second work ran");
        assert_eq!(first, second);
        assert_eq!(pool.shared.state.lock().workers, 1);
    }

    #[test]
    fn retires_idle_workers_and_respawns_on_demand() {
        let pool = pool(1, Duration::from_millis(20));
        let first = submit_reporting_thread(&pool)
            .recv_timeout(TIMEOUT)
            .expect("first work ran");
        wait_for_state(&pool.shared, |state| state.workers == 0);
        assert_eq!(pool.shared.state.lock().idle_workers, 0);
        let second = submit_reporting_thread(&pool)
            .recv_timeout(TIMEOUT)
            .expect("second work ran");
        assert_ne!(first, second);
    }

    #[test]
    fn skips_cancelled_runnables() {
        let pool = pool(1, TIMEOUT);
        let release = occupy_worker(&pool);
        let calls = Arc::new(AtomicUsize::new(0));
        let (runnable, task) = runnable({
            let calls = calls.clone();
            move || {
                calls.fetch_add(1, Ordering::SeqCst);
            }
        });
        pool.schedule(runnable);
        drop(task);
        let after = submit_reporting_thread(&pool);
        release.send(()).expect("worker is waiting");
        after.recv_timeout(TIMEOUT).expect("later work ran");
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn worker_survives_panics() {
        struct PanicsOnDrop;

        impl Drop for PanicsOnDrop {
            fn drop(&mut self) {
                panic!("payload drop panic");
            }
        }

        let calls = Arc::new(AtomicUsize::new(0));
        let pool = BlockingPool::with_limits(NonZeroUsize::MIN, TIMEOUT, {
            let calls = calls.clone();
            move |runnable| match calls.fetch_add(1, Ordering::SeqCst) {
                0 => {
                    drop(runnable);
                    panic!("callback panic");
                }
                1 => {
                    drop(runnable);
                    panic::panic_any(PanicsOnDrop);
                }
                _ => {
                    runnable.run();
                }
            }
        });
        submit(&pool, || {});
        submit(&pool, || {});
        submit_reporting_thread(&pool)
            .recv_timeout(TIMEOUT)
            .expect("worker survived both panics");
        wait_for_state(&pool.shared, |state| state.idle_workers == 1);
        assert_eq!(pool.shared.state.lock().workers, 1);
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn drop_drains_queued_work_then_stops_workers() {
        let pool = pool(1, TIMEOUT);
        let shared = pool.shared.clone();
        let release = occupy_worker(&pool);
        let queued = submit_reporting_thread(&pool);
        drop(pool);
        release.send(()).expect("worker is waiting");
        queued.recv_timeout(TIMEOUT).expect("queued work ran");
        wait_for_state(&shared, |state| state.workers == 0);
    }
}

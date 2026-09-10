use std::{
    collections::BinaryHeap,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use parking_lot::{Condvar, Mutex};

use crate::{
    PlatformDispatcher, Priority, RunnableVariant, profiler,
    queue::{PriorityQueueReceiver, PriorityQueueSender},
};

const MIN_THREADS: usize = 2;

/// A multithreaded [`PlatformDispatcher`] for tests and benchmarks.
///
/// Background tasks run in parallel on a pool of worker threads and timers fire
/// in real time on a dedicated timer thread, mirroring the production
/// dispatchers (see `LinuxDispatcher`). Main-thread tasks are queued until the
/// creating thread drains them via [`Self::run_until_idle`], since there is no
/// platform run loop pumping them.
///
/// Unlike [`TestDispatcher`](crate::TestDispatcher), which runs everything on a
/// single thread with a virtual clock, work dispatched through this dispatcher
/// executes with production concurrency.
pub struct ThreadedDispatcher {
    background_sender: PriorityQueueSender<RunnableVariant>,
    main_sender: PriorityQueueSender<RunnableVariant>,
    main_receiver: Mutex<PriorityQueueReceiver<RunnableVariant>>,
    timers: Arc<TimerQueue>,
    idle: Arc<IdleTracker>,
    main_thread_id: thread::ThreadId,
}

/// Tracks how many background and timer runnables are queued or running so
/// [`ThreadedDispatcher::run_until_idle`] knows when to stop waiting.
#[derive(Default)]
struct IdleTracker {
    inflight: Mutex<usize>,
    condvar: Condvar,
}

impl IdleTracker {
    fn increment(&self) {
        *self.inflight.lock() += 1;
    }

    fn decrement(&self) {
        let mut inflight = self.inflight.lock();
        *inflight -= 1;
        if *inflight == 0 {
            self.condvar.notify_all();
        }
    }

    /// Returns a guard that decrements the in-flight count when dropped, so
    /// the count stays correct even if the runnable being executed panics.
    fn decrement_on_drop(&self) -> impl Drop + '_ {
        gpui_util::defer(|| self.decrement())
    }

    /// Notifies waiters while holding the in-flight lock. `run_until_idle`
    /// re-checks its wake conditions under this lock before waiting, so the
    /// notification can't slip between its check and its wait and be lost.
    fn notify_under_lock(&self) {
        let _inflight = self.inflight.lock();
        self.condvar.notify_all();
    }
}

struct TimerQueue {
    state: Mutex<TimerQueueState>,
    condvar: Condvar,
}

struct TimerQueueState {
    heap: BinaryHeap<TimerEntry>,
    next_seq: u64,
}

struct TimerEntry {
    due: Instant,
    seq: u64,
    runnable: RunnableVariant,
}

impl PartialEq for TimerEntry {
    fn eq(&self, other: &Self) -> bool {
        self.due == other.due && self.seq == other.seq
    }
}

impl Eq for TimerEntry {}

impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimerEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reversed so that the entry with the earliest due time (breaking ties
        // by insertion order) is at the top of the max-heap.
        other
            .due
            .cmp(&self.due)
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

impl Default for ThreadedDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadedDispatcher {
    /// Creates a dispatcher whose main thread is the calling thread.
    ///
    /// Worker and timer threads live for the lifetime of the process; the
    /// dispatcher is expected to be created once and reused.
    pub fn new() -> Self {
        let (background_sender, background_receiver) = PriorityQueueReceiver::new();
        let (main_sender, main_receiver) = PriorityQueueReceiver::new();
        let idle = Arc::new(IdleTracker::default());

        let thread_count =
            thread::available_parallelism().map_or(MIN_THREADS, |i| i.get().max(MIN_THREADS));
        for i in 0..thread_count {
            let mut receiver: PriorityQueueReceiver<RunnableVariant> = background_receiver.clone();
            let idle = idle.clone();
            thread::Builder::new()
                .name(format!("ThreadedDispatcherWorker-{i}"))
                .spawn(move || {
                    while let Ok(runnable) = receiver.pop() {
                        let _decrement = idle.decrement_on_drop();
                        let location = runnable.metadata().location;
                        let spawned = runnable.metadata().spawned;
                        profiler::update_running_task(spawned, location);
                        runnable.run();
                        profiler::save_task_timing();
                    }
                })
                .expect("failed to spawn threaded dispatcher worker");
        }
        drop(background_receiver);

        let timers = Arc::new(TimerQueue {
            state: Mutex::new(TimerQueueState {
                heap: BinaryHeap::new(),
                next_seq: 0,
            }),
            condvar: Condvar::new(),
        });
        {
            let timers = timers.clone();
            let idle = idle.clone();
            thread::Builder::new()
                .name("ThreadedDispatcherTimer".to_owned())
                .spawn(move || {
                    let mut state = timers.state.lock();
                    loop {
                        let Some(entry) = state.heap.peek() else {
                            timers.condvar.wait(&mut state);
                            continue;
                        };
                        let due = entry.due;
                        if due > Instant::now() {
                            timers.condvar.wait_until(&mut state, due);
                            continue;
                        }
                        let Some(entry) = state.heap.pop() else {
                            continue;
                        };
                        // Count the firing timer as in-flight before releasing
                        // the lock so it can spawn follow-up work that
                        // `run_until_idle` will wait for. Lock order is always
                        // timer state, then in-flight count; `run_until_idle`
                        // never takes them in the opposite order.
                        idle.increment();
                        drop(state);

                        {
                            let _decrement = idle.decrement_on_drop();
                            let location = entry.runnable.metadata().location;
                            let spawned = entry.runnable.metadata().spawned;
                            profiler::update_running_task(spawned, location);
                            entry.runnable.run();
                            profiler::save_task_timing();
                        }

                        state = timers.state.lock();
                    }
                })
                .expect("failed to spawn threaded dispatcher timer");
        }

        Self {
            background_sender,
            main_sender,
            main_receiver: Mutex::new(main_receiver),
            timers,
            idle,
            main_thread_id: thread::current().id(),
        }
    }

    /// Runs queued main thread tasks and waits until no background or timer
    /// work is queued, running, or already due.
    ///
    /// Timers that haven't reached their due time yet are *not* waited for:
    /// the dispatcher runs in real time and cannot skip ahead like the
    /// `TestDispatcher`'s virtual clock, so waiting on a future timer would
    /// block for its full real duration. Tasks sleeping on such timers are
    /// considered idle. Must be called on the thread that created this
    /// dispatcher.
    pub fn run_until_idle(&self) {
        assert!(
            self.is_main_thread(),
            "run_until_idle must be called on the threaded dispatcher's main thread"
        );
        loop {
            if self.drain_main_queue() {
                continue;
            }

            // Checked before taking the in-flight lock; the timer thread
            // locks them in the opposite order, so nesting would deadlock.
            if self.has_due_timer() {
                // Poll briefly: a firing timer leaves the heap just before it
                // registers as in-flight.
                let mut inflight = self.idle.inflight.lock();
                self.idle
                    .condvar
                    .wait_for(&mut inflight, Duration::from_millis(1));
                continue;
            }

            let mut inflight = self.idle.inflight.lock();
            // Re-checked under the lock that `dispatch_on_main_thread`
            // notifies under, so the notification can't be lost.
            if self.main_queue_has_work() {
                continue;
            }
            if *inflight == 0 {
                // Main-thread sends happen before in-flight decrements, and
                // decrements happen under this lock, so the check above
                // observed all completed work.
                return;
            }
            // Woken when main-thread work arrives or the in-flight count
            // reaches zero; both notify under this lock.
            self.idle.condvar.wait(&mut inflight);
        }
    }

    /// Drives main-thread work until `ready` returns a value.
    ///
    /// Unlike [`Self::run_until_idle`], this waits across temporary quiescence.
    /// This is required when completion can arrive from an external worker that
    /// is not represented in the dispatcher's in-flight count.
    ///
    /// Readiness is checked before every main-thread runnable, so this returns
    /// as soon as `ready` observes completion rather than after the queue
    /// drains — deferred work that re-queues itself (idle sweeps, pollers)
    /// must not extend a benchmark's measured interval past the completion it
    /// awaits.
    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn run_until<R>(&self, mut ready: impl FnMut() -> Option<R>) -> R {
        assert!(
            self.is_main_thread(),
            "run_until must be called on the threaded dispatcher's main thread"
        );
        loop {
            if let Some(result) = ready() {
                return result;
            }
            if self.run_one_main_task() {
                continue;
            }

            let mut inflight = self.idle.inflight.lock();
            if self.main_queue_has_work() {
                continue;
            }
            self.idle.condvar.wait(&mut inflight);
        }
    }

    /// Runs at most one queued main-thread task, returning whether one ran.
    ///
    /// [`Self::run_until`] steps tasks one at a time so it can observe
    /// readiness between them: a task that perpetually re-queues itself (like
    /// an idle-time sweep) would otherwise keep [`Self::drain_main_queue`]
    /// looping past the completion the caller is waiting for.
    #[cfg(any(test, feature = "bench-support"))]
    fn run_one_main_task(&self) -> bool {
        let runnable = self.main_receiver.lock().try_pop();
        match runnable {
            Ok(Some(runnable)) => {
                let location = runnable.metadata().location;
                let spawned = runnable.metadata().spawned;
                profiler::update_running_task(spawned, location);
                runnable.run();
                profiler::save_task_timing();
                true
            }
            Ok(None) | Err(_) => false,
        }
    }

    /// Runs the main-thread tasks that were queued when the call began,
    /// returning whether any ran. Tasks dispatched while running (e.g. a task
    /// re-queuing itself after yielding) are left for the next call, as on
    /// the platform run loops.
    pub fn run_ready_main_tasks(&self) -> bool {
        assert!(
            self.is_main_thread(),
            "run_ready_main_tasks must be called on the threaded dispatcher's main thread"
        );
        let pending = self.main_receiver.lock().len();
        let mut ran_any = false;
        for _ in 0..pending {
            let runnable = self.main_receiver.lock().try_pop();
            match runnable {
                Ok(Some(runnable)) => {
                    let location = runnable.metadata().location;
                    let spawned = runnable.metadata().spawned;
                    profiler::update_running_task(spawned, location);
                    runnable.run();
                    profiler::save_task_timing();
                    ran_any = true;
                }
                Ok(None) | Err(_) => break,
            }
        }
        ran_any
    }

    /// Cancels all pending timers so timers armed by one workload can't fire
    /// during a later workload sharing this process-lifetime dispatcher.
    ///
    /// Dropping a timer runnable drops its completion sender, waking the task
    /// awaiting the timer. Call [`Self::run_until_idle`] after this method to
    /// drain any work that cancellation unblocks.
    pub fn cancel_pending_timers(&self) -> usize {
        let timers = {
            let mut state = self.timers.state.lock();
            let timers: Vec<_> = state.heap.drain().collect();
            self.timers.condvar.notify_all();
            timers
        };
        let canceled = timers.len();
        drop(timers);
        canceled
    }

    /// Describes the dispatcher's idle-tracking state, for diagnosing
    /// workloads that fail to reach quiescence.
    pub fn debug_state(&self) -> String {
        let inflight = *self.idle.inflight.lock();
        let timers = self.timers.state.lock().heap.len();
        let main_queue_has_work = self.main_queue_has_work();
        format!(
            "ThreadedDispatcher {{ inflight: {inflight}, pending_timers: {timers}, \
             main_queue_has_work: {main_queue_has_work} }}"
        )
    }

    /// Whether no main-thread work is queued, no background or timer
    /// runnables are queued or running, and no armed timer is due. Timers
    /// that aren't due yet are ignored, as in [`Self::run_until_idle`].
    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn is_idle(&self) -> bool {
        !self.main_queue_has_work() && !self.has_due_timer() && *self.idle.inflight.lock() == 0
    }

    fn has_due_timer(&self) -> bool {
        let state = self.timers.state.lock();
        state
            .heap
            .peek()
            .is_some_and(|entry| entry.due <= Instant::now())
    }

    fn main_queue_has_work(&self) -> bool {
        !self.main_receiver.lock().is_empty()
    }

    fn drain_main_queue(&self) -> bool {
        let mut ran_any = false;
        loop {
            // Lock only around the pop so runnables can re-entrantly dispatch
            // more main-thread work through the sender while they run.
            let runnable = self.main_receiver.lock().try_pop();
            match runnable {
                Ok(Some(runnable)) => {
                    let location = runnable.metadata().location;
                    let spawned = runnable.metadata().spawned;
                    profiler::update_running_task(spawned, location);
                    runnable.run();
                    profiler::save_task_timing();
                    ran_any = true;
                }
                Ok(None) | Err(_) => return ran_any,
            }
        }
    }
}

impl PlatformDispatcher for ThreadedDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        self.idle.increment();
        self.background_sender
            .send(priority, runnable)
            .unwrap_or_else(|_| panic!("threaded dispatcher workers are no longer running"));
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority) {
        if let Err(error) = self.main_sender.send(priority, runnable) {
            // The main receiver lives as long as this dispatcher, so a failed
            // send means we're mid-teardown. The runnable may wrap a !Send
            // future, so forget it rather than dropping it on this thread
            // (mirrors LinuxDispatcher).
            std::mem::forget(error);
            return;
        }
        // Wake `run_until_idle` if it's waiting for main-thread work.
        self.idle.notify_under_lock();
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let mut state = self.timers.state.lock();
        let seq = state.next_seq;
        state.next_seq += 1;
        state.heap.push(TimerEntry {
            due: Instant::now() + duration,
            seq,
            runnable,
        });
        self.timers.condvar.notify_one();
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        // This dispatcher does not need realtime scheduling priority; a plain
        // thread keeps it portable.
        thread::Builder::new()
            .name("ThreadedDispatcherRealtime".to_owned())
            .spawn(f)
            .expect("failed to spawn threaded dispatcher realtime thread");
    }

    fn as_threaded(&self) -> Option<&ThreadedDispatcher> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use super::*;
    use crate::{BackgroundExecutor, ForegroundExecutor};

    #[test]
    fn is_idle_tracks_queued_work_but_ignores_undue_timers() {
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let foreground = ForegroundExecutor::new(dispatcher.clone());
        assert!(dispatcher.is_idle());

        foreground.spawn(async {}).detach();
        assert!(!dispatcher.is_idle());
        dispatcher.run_until_idle();
        assert!(dispatcher.is_idle());

        let background = BackgroundExecutor::new(dispatcher.clone());
        let timer = background.timer(Duration::from_secs(60));
        // The timer future's initial poll runs on a worker thread; wait for
        // it so only the armed, not-yet-due timer remains.
        dispatcher.run_until_idle();
        assert!(
            dispatcher.is_idle(),
            "a timer that is not due yet should not count as pending work"
        );
        drop(timer);
        dispatcher.cancel_pending_timers();
        dispatcher.run_until_idle();
        assert!(dispatcher.is_idle());
    }

    #[test]
    fn run_ready_main_tasks_does_not_wait_for_background_handoffs() {
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let background = BackgroundExecutor::new(dispatcher.clone());
        let foreground = ForegroundExecutor::new(dispatcher.clone());

        let (sender, receiver) = futures::channel::oneshot::channel();
        background
            .spawn(async move {
                thread::sleep(Duration::from_millis(10));
                sender.send(()).ok();
            })
            .detach();

        let completed = Arc::new(AtomicBool::new(false));
        foreground
            .spawn({
                let completed = completed.clone();
                async move {
                    receiver.await.ok();
                    completed.store(true, Ordering::SeqCst);
                }
            })
            .detach();

        assert!(dispatcher.run_ready_main_tasks());
        assert!(!completed.load(Ordering::SeqCst));

        dispatcher.run_until_idle();
        assert!(completed.load(Ordering::SeqCst));
    }

    #[test]
    fn run_until_idle_completes_background_to_main_handoffs() {
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let background = BackgroundExecutor::new(dispatcher.clone());
        let foreground = ForegroundExecutor::new(dispatcher.clone());

        let (sender, receiver) = futures::channel::oneshot::channel();
        background
            .spawn(async move {
                thread::sleep(Duration::from_millis(10));
                sender.send(()).ok();
            })
            .detach();

        let completed = Arc::new(AtomicBool::new(false));
        foreground
            .spawn({
                let completed = completed.clone();
                async move {
                    receiver.await.ok();
                    completed.store(true, Ordering::SeqCst);
                }
            })
            .detach();

        dispatcher.run_until_idle();
        assert!(completed.load(Ordering::SeqCst));
    }

    #[test]
    fn run_until_waits_for_untracked_external_wakes() {
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let foreground = ForegroundExecutor::new(dispatcher.clone());
        let (sender, receiver) = futures::channel::oneshot::channel();
        let sender_thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            sender
                .send(())
                .expect("foreground receiver should remain alive");
        });

        let completed = Arc::new(AtomicBool::new(false));
        foreground
            .spawn({
                let completed = completed.clone();
                async move {
                    receiver
                        .await
                        .expect("external sender should deliver its wake");
                    completed.store(true, Ordering::SeqCst);
                }
            })
            .detach();

        dispatcher.run_until(|| completed.load(Ordering::SeqCst).then_some(()));
        sender_thread.join().expect("sender thread should finish");
        assert!(completed.load(Ordering::SeqCst));
    }

    #[test]
    fn run_until_returns_at_readiness_despite_requeuing_main_work() {
        const REQUEUE_LIMIT: usize = 10_000;

        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let foreground = ForegroundExecutor::new(dispatcher.clone());

        // Mirrors main-thread work that yields and immediately re-queues
        // itself (e.g. an idle-time sweep): the main queue never drains until
        // such work finishes every iteration, so readiness must be observed
        // between runnables rather than only at quiescence.
        let iterations = Arc::new(AtomicUsize::new(0));
        foreground
            .spawn({
                let iterations = iterations.clone();
                async move {
                    for _ in 0..REQUEUE_LIMIT {
                        iterations.fetch_add(1, Ordering::SeqCst);
                        yield_once().await;
                    }
                }
            })
            .detach();

        let completed = Arc::new(AtomicBool::new(false));
        foreground
            .spawn({
                let completed = completed.clone();
                async move {
                    completed.store(true, Ordering::SeqCst);
                }
            })
            .detach();

        dispatcher.run_until(|| completed.load(Ordering::SeqCst).then_some(()));
        assert!(
            iterations.load(Ordering::SeqCst) < REQUEUE_LIMIT,
            "run_until should return at readiness instead of draining re-queued main work"
        );
    }

    /// Completes after one re-schedule: the poll returns `Pending` and wakes
    /// immediately, so the runnable re-enters the main queue.
    fn yield_once() -> impl Future<Output = ()> {
        let mut yielded = false;
        std::future::poll_fn(move |poll_context| {
            if yielded {
                std::task::Poll::Ready(())
            } else {
                yielded = true;
                poll_context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        })
    }

    #[test]
    fn run_ready_main_tasks_advances_requeuing_work_one_batch_per_call() {
        const REQUEUE_LIMIT: usize = 10_000;

        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let foreground = ForegroundExecutor::new(dispatcher.clone());

        let iterations = Arc::new(AtomicUsize::new(0));
        foreground
            .spawn({
                let iterations = iterations.clone();
                async move {
                    for _ in 0..REQUEUE_LIMIT {
                        iterations.fetch_add(1, Ordering::SeqCst);
                        yield_once().await;
                    }
                }
            })
            .detach();

        assert!(dispatcher.run_ready_main_tasks());
        assert_eq!(iterations.load(Ordering::SeqCst), 1);
        assert!(dispatcher.run_ready_main_tasks());
        assert_eq!(iterations.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn timers_fire_in_real_time() {
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let background = BackgroundExecutor::new(dispatcher);

        let fired = Arc::new(AtomicBool::new(false));
        let timer = background.timer(Duration::from_millis(10));
        background
            .spawn({
                let fired = fired.clone();
                async move {
                    timer.await;
                    fired.store(true, Ordering::SeqCst);
                }
            })
            .detach();

        let deadline = Instant::now() + Duration::from_secs(10);
        while !fired.load(Ordering::SeqCst) && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(1));
        }
        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    fn cancel_pending_timers_wakes_waiters_without_waiting_for_deadline() {
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let background = BackgroundExecutor::new(dispatcher.clone());

        let fired = Arc::new(AtomicBool::new(false));
        let timer = background.timer(Duration::from_secs(10));
        background
            .spawn({
                let fired = fired.clone();
                async move {
                    timer.await;
                    fired.store(true, Ordering::SeqCst);
                }
            })
            .detach();

        dispatcher.run_until_idle();
        assert_eq!(dispatcher.cancel_pending_timers(), 1);
        dispatcher.run_until_idle();

        assert!(fired.load(Ordering::SeqCst));
        assert_eq!(dispatcher.cancel_pending_timers(), 0);
    }
}

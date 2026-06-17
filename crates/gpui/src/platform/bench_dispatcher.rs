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

/// A multithreaded [`PlatformDispatcher`] for benchmarks.
///
/// Background tasks run in parallel on a pool of worker threads and timers fire
/// in real time on a dedicated timer thread, mirroring the production
/// dispatchers (see `LinuxDispatcher`). Main-thread tasks are queued until the
/// benchmark thread drains them via [`Self::run_until_idle`], since there is no
/// platform run loop pumping them.
///
/// Unlike [`TestDispatcher`](crate::TestDispatcher), which runs everything on a
/// single thread with a virtual clock, work dispatched through this dispatcher
/// executes with production concurrency, so wall-clock measurements reflect
/// real parallelism.
pub struct BenchDispatcher {
    background_sender: PriorityQueueSender<RunnableVariant>,
    main_sender: PriorityQueueSender<RunnableVariant>,
    main_receiver: Mutex<PriorityQueueReceiver<RunnableVariant>>,
    timers: Arc<TimerQueue>,
    idle: Arc<IdleTracker>,
    main_thread_id: thread::ThreadId,
}

/// Tracks how many background and timer runnables are queued or running so
/// [`BenchDispatcher::run_until_idle`] knows when to stop waiting.
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

impl Default for BenchDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchDispatcher {
    /// Creates a dispatcher whose main thread is the calling thread.
    ///
    /// Worker and timer threads live for the lifetime of the process; the
    /// dispatcher is expected to be created once and reused across benchmarks.
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
                .name(format!("BenchWorker-{i}"))
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
                .expect("failed to spawn benchmark worker thread");
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
                .name("BenchTimer".to_owned())
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
                .expect("failed to spawn benchmark timer thread");
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
            "run_until_idle must be called on the benchmark main thread"
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

    /// Cancels all pending timers so timers armed by one benchmark can't fire
    /// during a later benchmark sharing this process-lifetime dispatcher.
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
    /// benchmarks that fail to reach quiescence.
    pub fn debug_state(&self) -> String {
        let inflight = *self.idle.inflight.lock();
        let timers = self.timers.state.lock().heap.len();
        let main_queue_has_work = self.main_queue_has_work();
        format!(
            "BenchDispatcher {{ inflight: {inflight}, pending_timers: {timers}, \
             main_queue_has_work: {main_queue_has_work} }}"
        )
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

impl PlatformDispatcher for BenchDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        self.idle.increment();
        self.background_sender
            .send(priority, runnable)
            .unwrap_or_else(|_| panic!("benchmark worker threads are no longer running"));
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
        // Benchmarks don't need realtime scheduling priority; a plain thread
        // keeps this portable.
        thread::Builder::new()
            .name("BenchRealtime".to_owned())
            .spawn(f)
            .expect("failed to spawn benchmark realtime thread");
    }

    fn as_bench(&self) -> Option<&BenchDispatcher> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::*;
    use crate::{BackgroundExecutor, ForegroundExecutor};

    #[test]
    fn run_until_idle_completes_background_to_main_handoffs() {
        let dispatcher = Arc::new(BenchDispatcher::new());
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
    fn timers_fire_in_real_time() {
        let dispatcher = Arc::new(BenchDispatcher::new());
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
        let dispatcher = Arc::new(BenchDispatcher::new());
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

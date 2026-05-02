use std::{
    sync::{
        Arc, Condvar, Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use gpui::{
    GLOBAL_THREAD_TIMINGS, PlatformDispatcher, Priority, PriorityQueueReceiver,
    PriorityQueueSender, RunnableVariant, TaskTiming, ThreadTaskTimings, profiler,
};
use parking_lot::Mutex as PlMutex;

static MAIN_THREAD_ID: OnceLock<thread::ThreadId> = OnceLock::new();

/// Records the calling thread as the GPUI main thread. Idempotent — only the
/// first call wins. Should be invoked early in `android_main` (or whichever
/// thread will pump the foreground executor).
pub(crate) fn record_main_thread_id() {
    let _ = MAIN_THREAD_ID.set(thread::current().id());
}

const MIN_BACKGROUND_THREADS: usize = 2;

struct TimerEntry {
    deadline: Instant,
    runnable: RunnableVariant,
}

/// Mailbox of runnables destined for the main (UI) thread. The Android
/// foreground executor drains this mailbox each time the JNI side wakes us
/// (typically via an `ALooper` callback). For the initial scaffold we expose a
/// blocking `drain_blocking` helper so a basic `Platform::run` implementation
/// can pump tasks even before the JNI bridge is wired up.
pub(crate) struct MainThreadMailbox {
    sender: PriorityQueueSender<RunnableVariant>,
    receiver: PlMutex<PriorityQueueReceiver<RunnableVariant>>,
    cv: Condvar,
    cv_mutex: Mutex<()>,
    /// Set to `true` to ask the main loop to exit.
    stop: AtomicBool,
}

impl MainThreadMailbox {
    fn new() -> Arc<Self> {
        let (sender, receiver) = PriorityQueueReceiver::new();
        Arc::new(Self {
            sender,
            receiver: PlMutex::new(receiver),
            cv: Condvar::new(),
            cv_mutex: Mutex::new(()),
            stop: AtomicBool::new(false),
        })
    }

    fn send(&self, priority: Priority, runnable: RunnableVariant) {
        if self.sender.send(priority, runnable).is_err() {
            log::warn!("MainThreadMailbox::send failed: receiver disconnected");
        }
        self.notify();
    }

    fn notify(&self) {
        let _guard = self.cv_mutex.lock().unwrap_or_else(|e| e.into_inner());
        self.cv.notify_one();
    }

    /// Drain all currently queued runnables, executing them on the calling
    /// (main) thread. Returns the number drained.
    pub(crate) fn drain(&self) -> usize {
        let receiver = self.receiver.lock().clone();
        let mut count = 0;
        for runnable in receiver.try_iter().flatten() {
            run_with_timing(runnable);
            count += 1;
        }
        count
    }

    /// Block until at least one runnable is available, then drain. Returns
    /// `false` once `stop` has been signalled.
    pub(crate) fn drain_blocking(&self) -> bool {
        if self.stop.load(Ordering::Acquire) {
            return false;
        }
        if self.drain() > 0 {
            return true;
        }
        let mut guard = self.cv_mutex.lock().unwrap_or_else(|e| e.into_inner());
        while !self.stop.load(Ordering::Acquire) {
            let (next_guard, _timeout) = self
                .cv
                .wait_timeout(guard, Duration::from_millis(100))
                .unwrap_or_else(|e| {
                    let (g, t) = e.into_inner();
                    (g, t)
                });
            guard = next_guard;
            if self.drain() > 0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn signal_stop(&self) {
        self.stop.store(true, Ordering::Release);
        self.notify();
    }

    pub(crate) fn is_stopped(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}

fn run_with_timing(runnable: RunnableVariant) {
    let location = runnable.metadata().location;
    let start = Instant::now();
    let mut timing = TaskTiming {
        location,
        start,
        end: None,
    };
    profiler::add_task_timing(timing);
    runnable.run();
    timing.end = Some(Instant::now());
    profiler::add_task_timing(timing);
}

pub(crate) struct AndroidDispatcher {
    main_mailbox: Arc<MainThreadMailbox>,
    background_sender: PriorityQueueSender<RunnableVariant>,
    // `std::sync::mpsc::Sender` is `Send` but `!Sync`, while `PlatformDispatcher`
    // is `Sync`. Wrapping in a `Mutex` makes shared-`&self` sends legal.
    timer_sender: PlMutex<std::sync::mpsc::Sender<TimerEntry>>,
    _background_threads: Vec<thread::JoinHandle<()>>,
}

impl AndroidDispatcher {
    pub(crate) fn new() -> (Arc<Self>, Arc<MainThreadMailbox>) {
        record_main_thread_id();

        let main_mailbox = MainThreadMailbox::new();

        let (background_sender, background_receiver) = PriorityQueueReceiver::new();
        let thread_count = thread::available_parallelism()
            .map_or(MIN_BACKGROUND_THREADS, |n| n.get().max(MIN_BACKGROUND_THREADS));

        let mut background_threads: Vec<_> = (0..thread_count)
            .map(|i| {
                let receiver: PriorityQueueReceiver<RunnableVariant> = background_receiver.clone();
                thread::Builder::new()
                    .name(format!("gpui-android-worker-{i}"))
                    .spawn(move || {
                        for runnable in receiver.iter() {
                            run_with_timing(runnable);
                        }
                    })
                    .expect("failed to spawn android background worker")
            })
            .collect();

        let (timer_sender, timer_receiver) = std::sync::mpsc::channel::<TimerEntry>();
        let main_for_timer = main_mailbox.clone();
        let timer_thread = thread::Builder::new()
            .name("gpui-android-timer".to_owned())
            .spawn(move || timer_loop(timer_receiver, main_for_timer))
            .expect("failed to spawn android timer thread");
        background_threads.push(timer_thread);

        let dispatcher = Arc::new(Self {
            main_mailbox: main_mailbox.clone(),
            background_sender,
            timer_sender: PlMutex::new(timer_sender),
            _background_threads: background_threads,
        });

        (dispatcher, main_mailbox)
    }
}

fn timer_loop(
    rx: std::sync::mpsc::Receiver<TimerEntry>,
    main_mailbox: Arc<MainThreadMailbox>,
) {
    let mut pending: Vec<TimerEntry> = Vec::new();
    loop {
        let now = Instant::now();
        let mut i = 0;
        while i < pending.len() {
            if pending[i].deadline <= now {
                let entry = pending.swap_remove(i);
                main_mailbox.send(Priority::Medium, entry.runnable);
            } else {
                i += 1;
            }
        }

        let next_deadline = pending.iter().map(|e| e.deadline).min();
        let recv_result = match next_deadline {
            Some(deadline) => rx.recv_timeout(deadline.saturating_duration_since(now)),
            None => match rx.recv() {
                Ok(entry) => Ok(entry),
                Err(_) => return,
            },
        };

        match recv_result {
            Ok(entry) => pending.push(entry),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return,
        }
    }
}

impl PlatformDispatcher for AndroidDispatcher {
    fn get_all_timings(&self) -> Vec<ThreadTaskTimings> {
        let global_timings = GLOBAL_THREAD_TIMINGS.lock();
        ThreadTaskTimings::convert(&global_timings)
    }

    fn get_current_thread_timings(&self) -> ThreadTaskTimings {
        gpui::profiler::get_current_thread_task_timings()
    }

    fn is_main_thread(&self) -> bool {
        match MAIN_THREAD_ID.get() {
            Some(id) => *id == thread::current().id(),
            None => false,
        }
    }

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        if self.background_sender.send(priority, runnable).is_err() {
            log::warn!("AndroidDispatcher::dispatch: background queue disconnected");
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority) {
        self.main_mailbox.send(priority, runnable);
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let entry = TimerEntry {
            deadline: Instant::now() + duration,
            runnable,
        };
        if self.timer_sender.lock().send(entry).is_err() {
            log::warn!("AndroidDispatcher::dispatch_after: timer thread is gone");
        }
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        // Android does not expose realtime scheduling to apps without root,
        // so we spawn a normal high-priority thread.
        thread::Builder::new()
            .name("gpui-android-realtime".to_owned())
            .spawn(move || f())
            .expect("failed to spawn android realtime thread");
    }
}

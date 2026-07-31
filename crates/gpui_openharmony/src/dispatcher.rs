use gpui::{PlatformDispatcher, Priority, PriorityQueueReceiver, RunnableVariant};
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    mem::MaybeUninit,
    sync::Arc,
    thread::{self, ThreadId},
    time::Duration,
};

pub(crate) struct OpenHarmonyDispatcher {
    main_thread_id: ThreadId,
    main_queue: Arc<Mutex<VecDeque<(Priority, RunnableVariant)>>>,
    background_sender: gpui::PriorityQueueSender<RunnableVariant>,
    _background_threads: Vec<thread::JoinHandle<()>>,
    _timer_thread: thread::JoinHandle<()>,
    timer_sender: std::sync::mpsc::Sender<TimerAfter>,
    wake: Arc<dyn Fn() + Send + Sync>,
}

struct TimerAfter {
    duration: Duration,
    runnable: RunnableVariant,
}

const MIN_THREADS: usize = 2;

impl OpenHarmonyDispatcher {
    pub fn new(wake: Arc<dyn Fn() + Send + Sync>) -> Self {
        let main_thread_id = thread::current().id();
        let main_queue = Arc::new(Mutex::new(VecDeque::new()));

        let (background_sender, background_receiver) = PriorityQueueReceiver::new();
        let thread_count = std::thread::available_parallelism()
            .map_or(MIN_THREADS, |i| i.get().max(MIN_THREADS));

        let mut background_threads = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            let receiver: PriorityQueueReceiver<RunnableVariant> = background_receiver.clone();
            background_threads.push(
                thread::Builder::new()
                    .name("openharmony-worker".to_string())
                    .spawn(move || {
                        for runnable in receiver.iter() {
                            runnable.run();
                        }
                    })
                    .expect("failed to spawn background worker"),
            );
        }

        let (timer_sender, timer_receiver) = std::sync::mpsc::channel::<TimerAfter>();
        let main_queue_for_timer = main_queue.clone();
        let wake_for_timer = wake.clone();
        let timer_thread = thread::Builder::new()
            .name("openharmony-timer".to_string())
            .spawn(move || {
                for timer in timer_receiver {
                    thread::sleep(timer.duration);
                    main_queue_for_timer
                        .lock()
                        .push_back((Priority::Medium, timer.runnable));
                    wake_for_timer();
                }
            })
            .expect("failed to spawn timer thread");

        Self {
            main_thread_id,
            main_queue,
            background_sender,
            _background_threads: background_threads,
            _timer_thread: timer_thread,
            timer_sender,
            wake,
        }
    }

    pub fn process_main_thread_queue(&self) {
        while let Some((_, runnable)) = self.main_queue.lock().pop_front() {
            runnable.run();
        }
    }
}

impl PlatformDispatcher for OpenHarmonyDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        if self.background_sender.send(priority, runnable).is_err() {
            log::debug!("background dispatcher disconnected");
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority) {
        self.main_queue.lock().push_back((priority, runnable));
        (self.wake)();
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        if self.timer_sender.send(TimerAfter { duration, runnable }).is_err() {
            log::debug!("timer dispatcher disconnected");
        }
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        thread::spawn(move || {
            let thread_id = unsafe { libc::pthread_self() };
            let policy = libc::SCHED_FIFO;
            let sched_priority = 65;

            let mut sched_param =
                unsafe { MaybeUninit::<libc::sched_param>::zeroed().assume_init() };
            sched_param.sched_priority = sched_priority;
            let result = unsafe { libc::pthread_setschedparam(thread_id, policy, &sched_param) };
            if result != 0 {
                log::warn!("failed to set realtime thread priority");
            }

            f();
        });
    }
}

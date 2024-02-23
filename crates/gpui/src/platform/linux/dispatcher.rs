#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//todo!(linux): remove
#![allow(unused_variables)]

use crate::platform::linux::client_dispatcher::ClientDispatcher;
use crate::{PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use std::{
    panic,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

pub(crate) struct LinuxDispatcher {
    client_dispatcher: Arc<dyn ClientDispatcher + Send + Sync>,
    parker: Mutex<Parker>,
    timed_tasks: Mutex<Vec<(Instant, Runnable)>>,
    main_sender: flume::Sender<Runnable>,
    background_sender: flume::Sender<Runnable>,
    _background_thread: thread::JoinHandle<()>,
    main_thread_id: thread::ThreadId,
}

impl LinuxDispatcher {
    pub fn new(
        main_sender: flume::Sender<Runnable>,
        client_dispatcher: &Arc<dyn ClientDispatcher + Send + Sync>,
    ) -> Self {
        let (background_sender, background_receiver) = flume::unbounded::<Runnable>();
        let background_thread = thread::spawn(move || {
            profiling::register_thread!("background");
            for runnable in background_receiver {
                let _ignore_panic = panic::catch_unwind(|| runnable.run());
            }
        });
        Self {
            client_dispatcher: Arc::clone(client_dispatcher),
            parker: Mutex::new(Parker::new()),
            timed_tasks: Mutex::new(Vec::new()),
            main_sender,
            background_sender,
            _background_thread: background_thread,
            main_thread_id: thread::current().id(),
        }
    }
}

impl PlatformDispatcher for LinuxDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: Runnable, _: Option<TaskLabel>) {
        self.background_sender.send(runnable).unwrap();
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.main_sender.send(runnable).unwrap();
        self.client_dispatcher.dispatch_on_main_thread();
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        let moment = Instant::now() + duration;
        let mut timed_tasks = self.timed_tasks.lock();
        timed_tasks.push((moment, runnable));
        timed_tasks.sort_unstable_by(|(a, _), (b, _)| b.cmp(a));
    }

    fn tick(&self, background_only: bool) -> bool {
        let mut timed_tasks = self.timed_tasks.lock();
        let old_count = timed_tasks.len();
        while let Some(&(moment, _)) = timed_tasks.last() {
            if moment <= Instant::now() {
                let (_, runnable) = timed_tasks.pop().unwrap();
                runnable.run();
            } else {
                break;
            }
        }
        timed_tasks.len() != old_count
    }

    fn park(&self) {
        self.parker.lock().park()
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }
}

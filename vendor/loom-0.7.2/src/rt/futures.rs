use _futures::executor;
use rt::{self, ThreadHandle};

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

pub struct Notify {
    thread: ThreadHandle,
    flag: AtomicBool,
}

impl Notify {
    pub fn new() -> Notify {
        Notify {
            thread: ThreadHandle::current(),
            flag: AtomicBool::new(false),
        }
    }

    pub fn consume_notify(&self) -> bool {
        self.flag.swap(false, Relaxed)
    }
}

impl executor::Notify for Notify {
    fn notify(&self, _id: usize) {
        rt::branch();

        self.flag.store(true, Relaxed);
        self.thread.unpark();
    }
}

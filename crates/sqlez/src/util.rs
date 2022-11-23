use std::ops::Deref;
use std::sync::mpsc::Sender;

use parking_lot::Mutex;
use thread_local::ThreadLocal;

pub struct UnboundedSyncSender<T: Send> {
    clonable_sender: Mutex<Sender<T>>,
    local_senders: ThreadLocal<Sender<T>>,
}

impl<T: Send> UnboundedSyncSender<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self {
            clonable_sender: Mutex::new(sender),
            local_senders: ThreadLocal::new(),
        }
    }
}

impl<T: Send> Deref for UnboundedSyncSender<T> {
    type Target = Sender<T>;

    fn deref(&self) -> &Self::Target {
        self.local_senders
            .get_or(|| self.clonable_sender.lock().clone())
    }
}

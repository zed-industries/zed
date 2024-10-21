use std::ops::Deref;
use std::sync::mpsc::Sender;

use parking_lot::Mutex;
use thread_local::ThreadLocal;

/// Unbounded standard library sender which is stored per thread to get around
/// the lack of sync on the standard library version while still being unbounded
/// Note: this locks on the cloneable sender, but its done once per thread, so it
/// shouldn't result in too much contention
pub struct UnboundedSyncSender<T: Send> {
    cloneable_sender: Mutex<Sender<T>>,
    local_senders: ThreadLocal<Sender<T>>,
}

impl<T: Send> UnboundedSyncSender<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self {
            cloneable_sender: Mutex::new(sender),
            local_senders: ThreadLocal::new(),
        }
    }
}

impl<T: Send> Deref for UnboundedSyncSender<T> {
    type Target = Sender<T>;

    fn deref(&self) -> &Self::Target {
        self.local_senders
            .get_or(|| self.cloneable_sender.lock().clone())
    }
}

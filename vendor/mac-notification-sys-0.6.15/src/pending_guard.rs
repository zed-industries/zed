use crate::notification::NotificationResponse;
use std::{
    collections::HashMap,
    sync::{Arc, Condvar, LazyLock, Mutex, atomic::AtomicBool},
};

pub(crate) struct PendingEntry {
    pub(crate) result: Mutex<NotificationResponse>,

    /// Set to true once a response callback fires (first-wins).
    pub(crate) done: AtomicBool,

    /// Wakes `rust_wait_for_notification` when `done` becomes true.
    pub(crate) condvar: Condvar,

    /// Set to true once `didDeliverNotification:` fires.
    /// Kept in a separate Mutex+Condvar so the delivery path does not borrow
    /// `result`'s lock and the two wait conditions stay independent.
    pub(crate) delivered: Mutex<bool>,
    pub(crate) delivered_cv: Condvar,
}

pub(crate) static PENDING: LazyLock<Mutex<HashMap<[u8; 16], Arc<PendingEntry>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub(crate) fn pending() -> &'static Mutex<HashMap<[u8; 16], Arc<PendingEntry>>> {
    &PENDING
}

/// RAII guard — removes the `PENDING` entry for `id` on drop.
/// Ensures the entry is cleaned up even when `send_notification` panics between
/// the `insert` and the explicit `remove`.
pub(crate) struct PendingGuard {
    pub(crate) id: [u8; 16],
}

impl Drop for PendingGuard {
    fn drop(&mut self) {
        pending().lock().unwrap().remove(&self.id);
    }
}

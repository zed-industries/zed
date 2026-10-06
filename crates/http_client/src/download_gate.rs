use std::{
    collections::{BTreeMap, HashSet},
    mem,
    sync::Arc,
};

use futures::channel::{mpsc, oneshot};
use gpui_shared_string::SharedString;
use parking_lot::Mutex;

pub const NODE_RUNTIME: &str = "Node.js";
pub const PRETTIER: &str = "Prettier";
pub const COPILOT: &str = "GitHub Copilot";

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BinaryDownload {
    pub tool: SharedString,
}

impl BinaryDownload {
    pub fn new(tool: impl Into<SharedString>) -> Self {
        Self { tool: tool.into() }
    }
}

impl std::fmt::Display for BinaryDownload {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.tool)
    }
}

#[derive(Clone)]
pub struct DownloadGate(Arc<Mutex<GateState>>);

impl DownloadGate {
    pub fn new(
        allow_all: bool,
        allowed: impl IntoIterator<Item = BinaryDownload>,
    ) -> (Self, mpsc::UnboundedReceiver<()>) {
        let (changed, changes) = mpsc::unbounded();
        let gate = Self(Arc::new(Mutex::new(GateState {
            allow_all,
            allowed: allowed.into_iter().collect(),
            denied: HashSet::default(),
            pending: BTreeMap::new(),
            changed,
        })));
        (gate, changes)
    }

    pub fn allow_all() -> Self {
        Self::new(true, []).0
    }

    pub fn deny_all() -> Self {
        Self::new(false, []).0
    }

    pub fn is_allowed(&self, download: &BinaryDownload) -> bool {
        self.0.lock().is_allowed(download)
    }

    pub fn check(&self, download: BinaryDownload) -> bool {
        self.register(download, None)
    }

    pub async fn request(&self, download: BinaryDownload) -> bool {
        let (sender, receiver) = oneshot::channel();
        if self.register(download.clone(), Some(sender)) {
            return true;
        }
        let mut pending_request = PendingRequest {
            gate: self,
            download,
            receiver: Some(receiver),
        };
        match pending_request.receiver.as_mut() {
            Some(receiver) => receiver.await.is_ok(),
            None => false,
        }
    }

    pub async fn allowed(&self, download: BinaryDownload, installed: bool) -> bool {
        if installed {
            self.check(download)
        } else {
            self.request(download).await
        }
    }

    pub fn pending(&self) -> Vec<BinaryDownload> {
        self.0.lock().pending.keys().cloned().collect()
    }

    pub fn allow(&self, download: BinaryDownload) {
        let mut state = self.0.lock();
        let pending = state.pending.remove(&download);
        state.denied.remove(&download);
        state.allowed.insert(download);
        if let Some(pending) = pending {
            for waiter in pending.waiters {
                waiter.send(()).ok();
            }
            state.changed.unbounded_send(()).ok();
        }
    }

    pub fn deny(&self, download: BinaryDownload) {
        let mut state = self.0.lock();
        let had_waiters = state.pending.remove(&download).is_some();
        state.denied.insert(download);
        if had_waiters {
            state.changed.unbounded_send(()).ok();
        }
    }

    pub fn clear_allowed(&self) {
        self.0.lock().allowed.clear();
    }

    pub fn set_allow_all(&self, allow_all: bool) {
        let mut state = self.0.lock();
        if state.allow_all == allow_all {
            return;
        }
        state.allow_all = allow_all;
        if allow_all && !state.pending.is_empty() {
            for pending in mem::take(&mut state.pending).into_values() {
                for waiter in pending.waiters {
                    waiter.send(()).ok();
                }
            }
            state.changed.unbounded_send(()).ok();
        }
    }

    fn register(&self, download: BinaryDownload, waiter: Option<oneshot::Sender<()>>) -> bool {
        let mut state = self.0.lock();
        if state.is_allowed(&download) {
            return true;
        }
        if state.denied.contains(&download) || state.changed.is_closed() {
            return false;
        }
        let is_new = !state.pending.contains_key(&download);
        let pending = state.pending.entry(download).or_default();
        pending.waiters.retain(|waiter| !waiter.is_canceled());
        match waiter {
            Some(waiter) => pending.waiters.push(waiter),
            None => pending.update = true,
        }
        if is_new {
            state.changed.unbounded_send(()).ok();
        }
        false
    }

    fn forget_canceled(&self, download: &BinaryDownload) {
        let mut state = self.0.lock();
        let Some(pending) = state.pending.get_mut(download) else {
            return;
        };
        pending.waiters.retain(|waiter| !waiter.is_canceled());
        if pending.waiters.is_empty() && !pending.update {
            state.pending.remove(download);
            state.changed.unbounded_send(()).ok();
        }
    }
}

struct GateState {
    allow_all: bool,
    allowed: HashSet<BinaryDownload>,
    denied: HashSet<BinaryDownload>,
    pending: BTreeMap<BinaryDownload, PendingDownload>,
    changed: mpsc::UnboundedSender<()>,
}

#[derive(Default)]
struct PendingDownload {
    waiters: Vec<oneshot::Sender<()>>,
    update: bool,
}

struct PendingRequest<'a> {
    gate: &'a DownloadGate,
    download: BinaryDownload,
    receiver: Option<oneshot::Receiver<()>>,
}

impl Drop for PendingRequest<'_> {
    fn drop(&mut self) {
        self.receiver.take();
        self.gate.forget_canceled(&self.download);
    }
}

impl GateState {
    fn is_allowed(&self, download: &BinaryDownload) -> bool {
        self.allow_all || self.allowed.contains(download)
    }
}

#[cfg(test)]
mod tests {
    use futures::FutureExt as _;

    use super::*;

    #[test]
    fn dropped_request_stops_being_pending() {
        let (gate, _changes) = DownloadGate::new(false, []);
        let update = BinaryDownload::new("installed-tool");
        let missing = BinaryDownload::new("missing-tool");

        assert!(!gate.check(update.clone()));
        let mut first_request = Box::pin(gate.request(missing.clone()));
        let mut second_request = Box::pin(gate.request(missing.clone()));
        assert_eq!((&mut first_request).now_or_never(), None);
        assert_eq!((&mut second_request).now_or_never(), None);
        assert_eq!(gate.pending(), vec![update.clone(), missing.clone()]);

        drop(first_request);
        assert_eq!(gate.pending(), vec![update.clone(), missing]);
        drop(second_request);
        assert_eq!(gate.pending(), vec![update]);
    }
}

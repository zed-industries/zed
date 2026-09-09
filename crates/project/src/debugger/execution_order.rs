use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ExecutionToken(u64);

// Kept on Session, not in snapshots which are moved into history on each stop.
#[derive(Default)]
pub(super) struct ExecutionOrder {
    revision: u64,
    all_threads: u64,
    threads: BTreeMap<i64, u64>,
    position: u64,
}

impl ExecutionOrder {
    pub(super) fn thread_changed(&mut self, thread: i64) -> ExecutionToken {
        self.revision += 1;
        self.threads.insert(thread, self.revision);
        ExecutionToken(self.revision)
    }

    pub(super) fn all_threads_changed(&mut self) {
        self.revision += 1;
        self.all_threads = self.revision;
        self.threads.clear();
    }

    pub(super) fn position_changed(&mut self) {
        self.revision += 1;
        self.position = self.revision;
    }

    pub(super) fn begin(&mut self, thread: i64) -> ExecutionToken {
        self.position_changed();
        self.thread_changed(thread)
    }

    pub(super) fn allows_thread(&self, token: ExecutionToken, thread: i64) -> bool {
        self.all_threads <= token.0
            && self.threads.get(&thread).copied().unwrap_or_default() <= token.0
    }

    pub(super) fn allows_all_threads(&self, token: ExecutionToken) -> bool {
        self.all_threads <= token.0
    }

    pub(super) fn allows_position(&self, token: ExecutionToken) -> bool {
        self.position <= token.0
    }

    pub(super) fn apply_all_threads(&mut self, token: ExecutionToken) {
        self.all_threads = token.0;
        self.threads.retain(|_, revision| *revision > token.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stop_invalidates_old_continue_and_position() {
        let mut order = ExecutionOrder::default();
        let request = order.begin(1);
        order.all_threads_changed();
        order.position_changed();
        assert!(!order.allows_thread(request, 1));
        assert!(!order.allows_all_threads(request));
        assert!(!order.allows_position(request));
    }

    #[test]
    fn single_thread_event_does_not_discard_other_thread_response() {
        let mut order = ExecutionOrder::default();
        let request = order.begin(1);
        order.thread_changed(2);
        order.position_changed();
        assert!(order.allows_thread(request, 1));
        assert!(!order.allows_thread(request, 2));
        assert!(order.allows_all_threads(request));
        assert!(!order.allows_position(request));
        order.apply_all_threads(request);
        assert!(!order.allows_thread(request, 2));
        assert!(order.allows_thread(request, 3));
    }

    #[test]
    fn newer_command_invalidates_old_success_and_failure_on_same_thread() {
        let mut order = ExecutionOrder::default();
        let first = order.begin(1);
        let second = order.begin(1);
        assert!(!order.allows_thread(first, 1));
        assert!(!order.allows_position(first));
        assert!(order.allows_thread(second, 1));
    }

    #[test]
    fn reversed_cross_thread_responses_keep_their_causal_order() {
        let mut order = ExecutionOrder::default();
        let first = order.begin(1);
        let second = order.begin(2);
        assert!(order.allows_thread(first, 1));
        order.apply_all_threads(second);
        assert!(!order.allows_thread(first, 1));
        assert!(!order.allows_all_threads(first));
    }

    #[test]
    fn normal_response_can_update_state_and_position() {
        let mut order = ExecutionOrder::default();
        let request = order.begin(1);
        assert!(order.allows_thread(request, 1));
        assert!(order.allows_all_threads(request));
        assert!(order.allows_position(request));
    }

    #[test]
    fn thread_lifecycle_events_invalidate_pending_failure() {
        let mut order = ExecutionOrder::default();
        let request = order.begin(1);
        order.thread_changed(1);
        assert!(!order.allows_thread(request, 1));
        order.all_threads_changed();
        assert!(!order.allows_thread(request, 2));
    }
}

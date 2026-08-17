use crate::rt::{object, Access, Location, Synchronize, VersionVec};
use std::collections::VecDeque;
use std::sync::atomic::Ordering::{Acquire, Release};

#[derive(Debug)]
pub(crate) struct Channel {
    state: object::Ref<State>,
}

#[derive(Debug)]
pub(super) struct State {
    /// Count of messages in the channel.
    msg_cnt: usize,

    /// Last access that was a send operation.
    last_send_access: Option<Access>,
    /// Last access that was a receive operation.
    last_recv_access: Option<Access>,

    /// A synchronization point for synchronizing the sending threads and the
    /// channel.
    ///
    /// The `mpsc` channels have a guarantee that the messages will be received
    /// in the same order in which they were sent. Therefore, if thread `t1`
    /// managed to send `m1` before `t2` sent `m2`, the thread that received
    /// `m2` can be sure that `m1` was already sent and received. In other
    /// words, it is sound for the receiver of `m2` to know that `m1` happened
    /// before `m2`. That is why we have a single `sender_synchronize` for
    /// senders which we use to "timestamp" each message put in the channel.
    /// However, in our example, the receiver of `m1` does not know whether `m2`
    /// was already sent or not and, therefore, by reading from the channel it
    /// should not learn any facts about `happens_before(send(m2), recv(m1))`.
    /// That is why we cannot use single `Synchronize` for the entire channel
    /// and on the receiver side we need to use `Synchronize` per message.
    sender_synchronize: Synchronize,
    /// A synchronization point per message synchronizing the receiving thread
    /// with the channel state at the point when the received message was sent.
    receiver_synchronize: VecDeque<Synchronize>,

    created: Location,
}

/// Actions performed on the Channel.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Action {
    /// Send a message
    MsgSend,
    /// Receive a message
    MsgRecv,
}

impl Channel {
    pub(crate) fn new(location: Location) -> Self {
        super::execution(|execution| {
            let state = execution.objects.insert(State {
                msg_cnt: 0,
                last_send_access: None,
                last_recv_access: None,
                sender_synchronize: Synchronize::new(),
                receiver_synchronize: VecDeque::new(),
                created: location,
            });

            tracing::trace!(?state, %location, "mpsc::channel");
            Self { state }
        })
    }

    pub(crate) fn send(&self, location: Location) {
        self.state.branch_action(Action::MsgSend, location);
        super::execution(|execution| {
            let state = self.state.get_mut(&mut execution.objects);
            state.msg_cnt = state.msg_cnt.checked_add(1).expect("overflow");

            state
                .sender_synchronize
                .sync_store(&mut execution.threads, Release);
            state
                .receiver_synchronize
                .push_back(state.sender_synchronize);

            if state.msg_cnt == 1 {
                // Unblock all threads that are blocked waiting on this channel
                let thread_id = execution.threads.active_id();
                for (id, thread) in execution.threads.iter_mut() {
                    if id == thread_id {
                        continue;
                    }

                    let obj = thread
                        .operation
                        .as_ref()
                        .map(|operation| operation.object());

                    if obj == Some(self.state.erase()) {
                        thread.set_runnable();
                    }
                }
            }
        })
    }

    pub(crate) fn recv(&self, location: Location) {
        self.state
            .branch_disable(Action::MsgRecv, self.is_empty(), location);
        super::execution(|execution| {
            let state = self.state.get_mut(&mut execution.objects);
            let thread_id = execution.threads.active_id();
            state.msg_cnt = state
                .msg_cnt
                .checked_sub(1)
                .expect("expected to be able to read the message");
            let mut synchronize = state.receiver_synchronize.pop_front().unwrap();
            dbg!(synchronize.sync_load(&mut execution.threads, Acquire));
            if state.msg_cnt == 0 {
                // Block all **other** threads attempting to read from the channel
                for (id, thread) in execution.threads.iter_mut() {
                    if id == thread_id {
                        continue;
                    }

                    if let Some(operation) = thread.operation.as_ref() {
                        if operation.object() == self.state.erase()
                            && operation.action() == object::Action::Channel(Action::MsgRecv)
                        {
                            let location = operation.location();
                            thread.set_blocked(location);
                        }
                    }
                }
            }
        })
    }

    /// Returns `true` if the channel is currently empty
    pub(crate) fn is_empty(&self) -> bool {
        super::execution(|execution| self.get_state(&mut execution.objects).msg_cnt == 0)
    }

    fn get_state<'a>(&self, objects: &'a mut object::Store) -> &'a mut State {
        self.state.get_mut(objects)
    }
}

impl State {
    pub(super) fn check_for_leaks(&self, index: usize) {
        if self.msg_cnt != 0 {
            if self.created.is_captured() {
                panic!(
                    "Messages leaked.\n  \
                    Channel created: {}\n            \
                    Index: {}\n        \
                    Messages: {}",
                    self.created, index, self.msg_cnt
                );
            } else {
                panic!(
                    "Messages leaked.\n     Index: {}\n  Messages: {}",
                    index, self.msg_cnt
                );
            }
        }
    }

    pub(super) fn last_dependent_access(&self, action: Action) -> Option<&Access> {
        match action {
            Action::MsgSend => self.last_send_access.as_ref(),
            Action::MsgRecv => self.last_recv_access.as_ref(),
        }
    }

    pub(super) fn set_last_access(&mut self, action: Action, path_id: usize, version: &VersionVec) {
        match action {
            Action::MsgSend => Access::set_or_create(&mut self.last_send_access, path_id, version),
            Action::MsgRecv => Access::set_or_create(&mut self.last_recv_access, path_id, version),
        }
    }
}

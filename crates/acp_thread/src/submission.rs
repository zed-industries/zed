use crate::ReceiptSessionSubmissions;
use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use futures::{
    channel::oneshot,
    future::{BoxFuture, LocalBoxFuture},
};
use gpui::{SharedString, Task};
use std::collections::BTreeMap;
use std::{
    fmt,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubmissionId(pub(crate) u64);

impl SubmissionId {
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub enum SubmissionResponse {
    LegacyCompleted(acp_v1::PromptResponse),
    Accepted(acp_v2::PromptResponse),
}

/// The transport response and, for ordered transports, acknowledgement of model application.
/// The shared tracker applies the outcome before acknowledging; transports must not
/// wait for that acknowledgement before producing their response.
pub struct ReceiptSubmission {
    pub(crate) response: LocalBoxFuture<'static, anyhow::Result<acp_v2::PromptResponse>>,
    pub(crate) applied: Option<oneshot::Sender<bool>>,
}

impl ReceiptSubmission {
    /// The acknowledgement reports successful or failed model application.
    /// Dropping it instead means application was abandoned.
    pub fn ordered(
        response: LocalBoxFuture<'static, anyhow::Result<acp_v2::PromptResponse>>,
        applied: oneshot::Sender<bool>,
    ) -> Self {
        Self {
            response,
            applied: Some(applied),
        }
    }
}

impl From<Task<anyhow::Result<acp_v2::PromptResponse>>> for ReceiptSubmission {
    fn from(response: Task<anyhow::Result<acp_v2::PromptResponse>>) -> Self {
        Self {
            response: Box::pin(response),
            applied: None,
        }
    }
}

/// A local receipt withdrawn by cancellation before the message was accepted.
#[derive(Debug)]
pub struct SubmissionCancelled;

impl fmt::Display for SubmissionCancelled {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Submission cancelled before acceptance")
    }
}

impl std::error::Error for SubmissionCancelled {}

/// Local loss of a receipt waiter does not establish that the agent rejected the message.
#[derive(Debug)]
pub struct SubmissionDisconnected;

impl fmt::Display for SubmissionDisconnected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Agent connection lost before submission acceptance was confirmed")
    }
}

impl std::error::Error for SubmissionDisconnected {}

pub struct Submission {
    pub id: SubmissionId,
    pub response: BoxFuture<'static, anyhow::Result<Option<SubmissionResponse>>>,
}

impl Submission {
    pub(crate) fn new(
        id: SubmissionId,
        response: BoxFuture<'static, anyhow::Result<Option<SubmissionResponse>>>,
    ) -> Self {
        Self { id, response }
    }
}

impl Future for Submission {
    type Output = anyhow::Result<Option<SubmissionResponse>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().response.as_mut().poll(cx)
    }
}

#[derive(Debug)]
pub enum SubmissionState {
    Pending,
    Accepted {
        receipt: acp_v2::PromptResponse,
        echoed: bool,
    },
    Disconnected {
        /// Absence means acceptance is unknown, not that insertion failed.
        receipt: Option<acp_v2::PromptResponse>,
    },
    Completed,
    Failed(SharedString),
    Cancelled,
}

pub struct SubmissionRecord {
    pub content: Arc<[acp_v2::ContentBlock]>,
    pub state: SubmissionState,
    task: Option<Task<()>>,
    disconnect_sender: Option<oneshot::Sender<()>>,
}

impl fmt::Debug for SubmissionRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SubmissionRecord")
            .field("content", &self.content)
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

impl SubmissionRecord {
    fn is_unsettled(&self) -> bool {
        matches!(
            self.state,
            SubmissionState::Pending | SubmissionState::Accepted { echoed: false, .. }
        )
    }

    fn is_recoverable(&self) -> bool {
        self.is_unsettled()
            || matches!(
                self.state,
                SubmissionState::Failed(_)
                    | SubmissionState::Cancelled
                    | SubmissionState::Disconnected { .. }
            )
    }
}

pub(crate) struct SessionSubmissions {
    receipt_transport: Option<Rc<dyn ReceiptSessionSubmissions>>,
    records: BTreeMap<SubmissionId, SubmissionRecord>,
    next_id: u64,
    latest_id: Option<SubmissionId>,
}

impl SessionSubmissions {
    pub(crate) fn new(receipt_transport: Option<Rc<dyn ReceiptSessionSubmissions>>) -> Self {
        Self {
            receipt_transport,
            records: BTreeMap::default(),
            next_id: 0,
            latest_id: None,
        }
    }

    pub(crate) fn receipt_transport(&self) -> Option<&Rc<dyn ReceiptSessionSubmissions>> {
        self.receipt_transport.as_ref()
    }

    pub(crate) fn get(&self, id: SubmissionId) -> Option<&SubmissionRecord> {
        self.records.get(&id)
    }

    pub(crate) fn latest_id(&self) -> Option<SubmissionId> {
        self.latest_id
    }

    pub(crate) fn has_unsettled(&self) -> bool {
        self.records.values().any(SubmissionRecord::is_unsettled)
    }

    pub(crate) fn recoverable(&self) -> impl Iterator<Item = (SubmissionId, &SubmissionRecord)> {
        self.records.iter().filter_map(|(id, record)| {
            (self.receipt_transport.is_some() && record.is_recoverable()).then_some((*id, record))
        })
    }

    pub(crate) fn register(&mut self, content: Arc<[acp_v2::ContentBlock]>) -> SubmissionId {
        self.next_id += 1;
        let id = SubmissionId(self.next_id);
        self.latest_id = Some(id);
        self.prune();
        self.records.insert(
            id,
            SubmissionRecord {
                content,
                state: SubmissionState::Pending,
                task: None,
                disconnect_sender: None,
            },
        );
        id
    }

    pub(crate) fn track(
        &mut self,
        id: SubmissionId,
        task: Task<()>,
        disconnect_sender: Option<oneshot::Sender<()>>,
    ) {
        if let Some(record) = self.records.get_mut(&id) {
            record.task = Some(task);
            record.disconnect_sender = disconnect_sender;
        }
    }

    pub(crate) fn settle(&mut self, id: SubmissionId, state: SubmissionState) -> bool {
        let changed = self.records.get_mut(&id).is_some_and(|record| {
            drop(record.disconnect_sender.take());
            match &mut record.state {
                SubmissionState::Disconnected { receipt } => {
                    // A ready receipt can race with disconnect cleanup. Retain its
                    // acceptance evidence without reopening the disconnected record.
                    if receipt.is_none()
                        && let SubmissionState::Accepted {
                            receipt: accepted, ..
                        } = state
                    {
                        *receipt = Some(accepted);
                        return true;
                    }
                    false
                }
                _ => {
                    record.state = state;
                    true
                }
            }
        });
        self.prune();
        changed
    }

    pub(crate) fn disconnect(&mut self) -> Vec<SubmissionId> {
        if self.receipt_transport.is_none() {
            return Vec::new();
        }
        let mut changed = Vec::new();
        for (id, record) in &mut self.records {
            let receipt = match &record.state {
                SubmissionState::Pending => None,
                SubmissionState::Accepted {
                    receipt,
                    echoed: false,
                } => Some(receipt.clone()),
                _ => continue,
            };
            record.state = SubmissionState::Disconnected { receipt };
            if let Some(sender) = record.disconnect_sender.take()
                && sender.send(()).is_err()
            {
                log::debug!("Submission already finished before disconnect notification");
            }
            changed.push(*id);
        }
        changed
    }

    pub(crate) fn observe_echo(&mut self, message_id: &acp_v2::MessageId) -> Vec<SubmissionId> {
        let mut changed = Vec::new();
        for (id, record) in &mut self.records {
            if let SubmissionState::Accepted { receipt, echoed } = &mut record.state
                && &receipt.message_id == message_id
                && !*echoed
            {
                *echoed = true;
                changed.push(*id);
            }
        }
        self.prune();
        changed
    }

    pub(crate) fn forget(&mut self, id: SubmissionId) -> bool {
        if self
            .records
            .get(&id)
            .is_some_and(SubmissionRecord::is_unsettled)
        {
            return false;
        }
        if self.records.remove(&id).is_none() {
            return false;
        }
        if self.latest_id == Some(id) {
            self.latest_id = None;
        }
        true
    }

    fn prune(&mut self) {
        self.records.retain(|id, record| {
            Some(*id) == self.latest_id
                || record.is_unsettled()
                || (self.receipt_transport.is_some() && record.is_recoverable())
        });
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ForegroundActivity {
    Idle,
    Running,
    RequiresAction,
}

pub(crate) struct SessionActivity {
    state: acp_v2::StateUpdate,
    generation: u64,
    started_at: Option<Instant>,
    duration: Option<Duration>,
    first_entry_index: usize,
    cancel_waiters: Vec<oneshot::Sender<()>>,
    observation_closed: bool,
}

impl Default for SessionActivity {
    fn default() -> Self {
        Self {
            state: acp_v2::StateUpdate::Idle(acp_v2::IdleStateUpdate::new()),
            generation: 0,
            started_at: None,
            duration: None,
            first_entry_index: 0,
            cancel_waiters: Vec::new(),
            observation_closed: false,
        }
    }
}

impl SessionActivity {
    pub(crate) fn state(&self) -> &acp_v2::StateUpdate {
        &self.state
    }

    pub(crate) fn phase(&self) -> ForegroundActivity {
        match self.state {
            acp_v2::StateUpdate::Idle(_) => ForegroundActivity::Idle,
            acp_v2::StateUpdate::RequiresAction(_) => ForegroundActivity::RequiresAction,
            _ => ForegroundActivity::Running,
        }
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn started_at(&self) -> Option<Instant> {
        self.started_at
    }

    pub(crate) fn duration(&self) -> Option<Duration> {
        if self.phase() == ForegroundActivity::Idle {
            self.duration
        } else {
            self.started_at.map(|started| started.elapsed())
        }
    }

    pub(crate) fn first_entry_index(&self) -> usize {
        self.first_entry_index
    }

    pub(crate) fn update(&mut self, state: acp_v2::StateUpdate, first_entry_index: usize) {
        let previous = self.phase();
        self.state = state;
        let current = self.phase();
        if previous == ForegroundActivity::Idle && current != ForegroundActivity::Idle {
            self.generation += 1;
            self.started_at = Some(Instant::now());
            self.duration = None;
            self.first_entry_index = first_entry_index;
        } else if previous != ForegroundActivity::Idle && current == ForegroundActivity::Idle {
            self.duration = self.started_at.map(|started| started.elapsed());
            for waiter in self.cancel_waiters.drain(..) {
                if waiter.send(()).is_err() {
                    log::debug!("Foreground cancellation observer was dropped");
                }
            }
        }
    }

    pub(crate) fn ensure_observable(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            !self.observation_closed,
            "Agent connection lost before cancellation was confirmed"
        );
        Ok(())
    }

    pub(crate) fn close_observation(&mut self) {
        self.observation_closed = true;
        self.cancel_waiters.clear();
    }

    pub(crate) fn wait_for_idle(&mut self) -> anyhow::Result<Option<oneshot::Receiver<()>>> {
        self.ensure_observable()?;
        if self.phase() == ForegroundActivity::Idle {
            return Ok(None);
        }
        let (sender, receiver) = oneshot::channel();
        self.cancel_waiters.push(sender);
        Ok(Some(receiver))
    }
}

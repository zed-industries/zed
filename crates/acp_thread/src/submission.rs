use crate::ReceiptSessionSubmissions;
use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use futures::{channel::oneshot, future::BoxFuture};
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
    Completed,
    Failed(SharedString),
    Cancelled,
}

pub struct SubmissionRecord {
    pub content: Arc<[acp_v2::ContentBlock]>,
    pub state: SubmissionState,
    task: Option<Task<()>>,
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
                SubmissionState::Failed(_) | SubmissionState::Cancelled
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
            },
        );
        id
    }

    pub(crate) fn track(&mut self, id: SubmissionId, task: Task<()>) {
        if let Some(record) = self.records.get_mut(&id) {
            record.task = Some(task);
        }
    }

    pub(crate) fn settle(&mut self, id: SubmissionId, state: SubmissionState) {
        if let Some(record) = self.records.get_mut(&id) {
            record.state = state;
        }
        self.prune();
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

    pub(crate) fn wait_for_idle(&mut self) -> Option<oneshot::Receiver<()>> {
        if self.phase() == ForegroundActivity::Idle {
            return None;
        }
        let (sender, receiver) = oneshot::channel();
        self.cancel_waiters.push(sender);
        Some(receiver)
    }
}

use std::collections::VecDeque;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueueEntryId(usize);

pub struct QueueEntry {
    pub id: QueueEntryId,
    pub content: Vec<acp::ContentBlock>,
    pub tracked_buffers: Vec<Entity<Buffer>>,
    /// When true, this message interrupts the agent at the next turn boundary
    /// instead of waiting for generation to fully complete. Only the front
    /// entry's value matters, since messages are delivered in FIFO order.
    pub steer: bool,
    pub editor: Entity<MessageEditor>,
    pub _subscription: Subscription,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessingState {
    AutoProcess,
    Paused,
    // Sending a message out of turn cancelled the current generation; we must
    // absorb the Stopped event from that cancellation before resuming
    // auto-processing, otherwise the queue would double-send.
    AbsorbingCancel,
}

/// Holds follow-up messages typed while the agent is generating, along with
/// the state machine that decides when they're auto-sent.
pub struct MessageQueue {
    entries: VecDeque<QueueEntry>,
    processing_state: ProcessingState,
    can_fast_track: bool,
    next_id: usize,
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self {
            entries: VecDeque::new(),
            processing_state: ProcessingState::AutoProcess,
            can_fast_track: false,
            next_id: 0,
        }
    }
}

impl MessageQueue {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn first(&self) -> Option<&QueueEntry> {
        self.entries.front()
    }

    pub fn first_id(&self) -> Option<QueueEntryId> {
        self.entries.front().map(|entry| entry.id)
    }

    pub fn last_id(&self) -> Option<QueueEntryId> {
        self.entries.back().map(|entry| entry.id)
    }

    /// Whether the next message should interrupt the agent at the next turn
    /// boundary. Drives the native thread's boundary flag.
    pub fn front_wants_steer(&self) -> bool {
        self.entries.front().is_some_and(|entry| entry.steer)
    }

    pub fn toggle_steer(&mut self, id: QueueEntryId) {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
            entry.steer = !entry.steer;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &QueueEntry> {
        self.entries.iter()
    }

    pub fn can_fast_track(&self) -> bool {
        self.can_fast_track && !self.entries.is_empty()
    }

    pub fn entry_by_id(&self, id: QueueEntryId) -> Option<&QueueEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    pub fn entry_by_id_mut(&mut self, id: QueueEntryId) -> Option<&mut QueueEntry> {
        self.entries.iter_mut().find(|entry| entry.id == id)
    }

    /// Allocates a stable ID for a new entry. This is separate from `enqueue`
    /// because the editor event subscription must capture the ID before the
    /// `QueueEntry` (which owns that subscription) can be constructed.
    pub fn next_id(&mut self) -> QueueEntryId {
        let id = QueueEntryId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Queuing a message is active engagement, so it also resumes
    /// auto-processing if the queue was paused.
    pub fn enqueue(&mut self, entry: QueueEntry) {
        self.entries.push_back(entry);
        self.processing_state = ProcessingState::AutoProcess;
        self.can_fast_track = true;
    }

    pub fn remove(&mut self, id: QueueEntryId) -> Option<QueueEntry> {
        let index = self.entries.iter().position(|entry| entry.id == id)?;
        self.entries.remove(index)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.can_fast_track = false;
    }

    /// Pops the front entry if a fast-track send is allowed (the user just
    /// queued a message and pressed Enter on an empty main editor).
    ///
    /// This works even when paused — pressing Enter is an explicit user
    /// action, distinct from auto-processing. If a generation is in flight,
    /// the dispatch will cancel it, so we must absorb that cancellation's
    /// Stopped event to avoid double-sending the next entry.
    pub fn try_fast_track(&mut self, is_generating: bool) -> Option<QueueEntry> {
        if !self.can_fast_track {
            return None;
        }
        self.can_fast_track = false;
        let entry = self.entries.pop_front()?;
        self.processing_state = if is_generating {
            ProcessingState::AbsorbingCancel
        } else {
            ProcessingState::AutoProcess
        };
        Some(entry)
    }

    /// Handles a generation Stopped event, returning the entry to auto-send,
    /// if any.
    pub fn on_generation_stopped(&mut self, is_first_editor_focused: bool) -> Option<QueueEntry> {
        match self.processing_state {
            ProcessingState::AbsorbingCancel => {
                // This Stopped event came from a cancellation we initiated
                // ourselves (e.g. "Send Now"); swallow it and resume.
                self.processing_state = ProcessingState::AutoProcess;
                None
            }
            ProcessingState::Paused => None,
            ProcessingState::AutoProcess => {
                // Don't auto-send while the user is editing the next message.
                if is_first_editor_focused {
                    None
                } else {
                    self.entries.pop_front()
                }
            }
        }
    }

    /// Removes an entry for an explicit "Send Now". If a generation is in
    /// flight, the dispatch will cancel it, so we must absorb that
    /// cancellation's Stopped event.
    pub fn send_now(&mut self, id: QueueEntryId, is_generating: bool) -> Option<QueueEntry> {
        let entry = self.remove(id)?;
        if is_generating {
            self.processing_state = ProcessingState::AbsorbingCancel;
        }
        Some(entry)
    }

    /// Called when the user stops generation; queued messages stay put until
    /// the user re-engages.
    pub fn pause(&mut self) {
        self.processing_state = ProcessingState::Paused;
    }

    /// Called when the user sends a new message, re-enabling auto-processing.
    /// This is what un-freezes the queue after a manual stop.
    pub fn resume(&mut self) {
        self.processing_state = ProcessingState::AutoProcess;
    }
}

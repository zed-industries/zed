use alloc::{sync::Arc, vec::Vec};

use smallvec::SmallVec;
use thiserror::Error;

use crate::{
    device::{
        queue::{EncoderInFlight, SubmittedWorkDoneClosure, TempResource},
        DeviceError,
    },
    ray_tracing::BlasCompactReadyPendingClosure,
    resource::{Blas, Buffer, Texture, Trackable},
    snatch::SnatchGuard,
    SubmissionIndex,
};

/// A command submitted to the GPU for execution.
///
/// ## Keeping resources alive while the GPU is using them
///
/// [`wgpu_hal`] requires that, when a command is submitted to a queue, all the
/// resources it uses must remain alive until it has finished executing.
///
/// [`wgpu_hal`]: hal
struct ActiveSubmission {
    /// The index of the submission we track.
    ///
    /// When `Device::fence`'s value is greater than or equal to this, our queue
    /// submission has completed.
    index: SubmissionIndex,

    /// Buffers to be mapped once this submission has completed.
    mapped: Vec<Arc<Buffer>>,

    /// BLASes to have their compacted size read back once this submission has completed.
    compact_read_back: Vec<Arc<Blas>>,

    /// Command buffers used by this submission, and the encoder that owns them.
    ///
    /// [`wgpu_hal::Queue::submit`] requires the submitted command buffers to
    /// remain alive until the submission has completed execution. Command
    /// encoders double as allocation pools for command buffers, so holding them
    /// here and cleaning them up in [`LifetimeTracker::triage_submissions`]
    /// satisfies that requirement.
    ///
    /// Once this submission has completed, the command buffers are reset and
    /// the command encoder is recycled.
    ///
    /// [`wgpu_hal::Queue::submit`]: hal::Queue::submit
    encoders: Vec<EncoderInFlight>,

    /// List of queue "on_submitted_work_done" closures to be called once this
    /// submission has completed.
    work_done_closures: SmallVec<[SubmittedWorkDoneClosure; 1]>,
}

impl ActiveSubmission {
    /// Returns true if this submission contains the given buffer.
    ///
    /// This only uses constant-time operations.
    pub fn contains_buffer(&self, buffer: &Buffer) -> bool {
        for encoder in &self.encoders {
            // The ownership location of buffers depends on where the command encoder
            // came from. If it is the staging command encoder on the queue, it is
            // in the pending buffer list. If it came from a user command encoder,
            // it is in the tracker.

            if encoder.trackers.buffers.contains(buffer) {
                return true;
            }

            if encoder
                .pending_buffers
                .contains_key(&buffer.tracker_index())
            {
                return true;
            }
        }

        false
    }

    /// Returns true if this submission contains the given texture.
    ///
    /// This only uses constant-time operations.
    pub fn contains_texture(&self, texture: &Texture) -> bool {
        for encoder in &self.encoders {
            // The ownership location of textures depends on where the command encoder
            // came from. If it is the staging command encoder on the queue, it is
            // in the pending buffer list. If it came from a user command encoder,
            // it is in the tracker.

            if encoder.trackers.textures.contains(texture) {
                return true;
            }

            if encoder
                .pending_textures
                .contains_key(&texture.tracker_index())
            {
                return true;
            }
        }

        false
    }

    /// Returns true if this submission contains the given blas.
    ///
    /// This only uses constant-time operations.
    pub fn contains_blas(&self, blas: &Blas) -> bool {
        for encoder in &self.encoders {
            if encoder.trackers.blas_s.contains(blas) {
                return true;
            }

            if encoder.pending_blas_s.contains_key(&blas.tracker_index()) {
                return true;
            }
        }

        false
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum WaitIdleError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Tried to wait using a submission index ({0}) that has not been returned by a successful submission (last successful submission: {1})")]
    WrongSubmissionIndex(SubmissionIndex, SubmissionIndex),
    #[error("Timed out trying to wait for the given submission index.")]
    Timeout,
}

impl WaitIdleError {
    pub fn to_poll_error(&self) -> Option<wgt::PollError> {
        match self {
            WaitIdleError::Timeout => Some(wgt::PollError::Timeout),
            &WaitIdleError::WrongSubmissionIndex(a, b) => {
                Some(wgt::PollError::WrongSubmissionIndex(a, b))
            }
            _ => None,
        }
    }
}

/// Resource tracking for a device.
///
/// ## Host mapping buffers
///
/// A buffer cannot be mapped until all active queue submissions that use it
/// have completed. To that end:
///
/// -   Each buffer's `ResourceInfo::submission_index` records the index of the
///     most recent queue submission that uses that buffer.
///
/// -   When the device is polled, the following `LifetimeTracker` methods decide
///     what should happen next:
///
///     1)  `triage_submissions` moves entries in `self.active[i]` for completed
///         submissions to `self.ready_to_map`.  At this point, both
///         `self.active` and `self.ready_to_map` are up to date with the given
///         submission index.
///
///     2)  `handle_mapping` drains `self.ready_to_map` and actually maps the
///         buffers, collecting a list of notification closures to call.
///
/// Only calling `Global::buffer_map_async` clones a new `Arc` for the
/// buffer. This new `Arc` is only dropped by `handle_mapping`.
pub(crate) struct LifetimeTracker {
    /// Resources used by queue submissions still in flight. One entry per
    /// submission, with older submissions appearing before younger.
    ///
    /// Entries are added by `track_submission` and drained by
    /// `LifetimeTracker::triage_submissions`. Lots of methods contribute data
    /// to particular entries.
    active: Vec<ActiveSubmission>,

    /// Buffers the user has asked us to map, and which are not used by any
    /// queue submission still in flight.
    ready_to_map: Vec<Arc<Buffer>>,

    /// BLASes the user has asked us to prepare to compact, and which are not used by any
    /// queue submission still in flight.
    ready_to_compact: Vec<Arc<Blas>>,

    /// Queue "on_submitted_work_done" closures that were initiated for while there is no
    /// currently pending submissions. These cannot be immediately invoked as they
    /// must happen _after_ all mapped buffer callbacks are mapped, so we defer them
    /// here until the next time the device is maintained.
    work_done_closures: SmallVec<[SubmittedWorkDoneClosure; 1]>,
}

impl LifetimeTracker {
    pub fn new() -> Self {
        Self {
            active: Vec::new(),
            ready_to_map: Vec::new(),
            ready_to_compact: Vec::new(),
            work_done_closures: SmallVec::new(),
        }
    }

    /// Return true if there are no queue submissions still in flight.
    pub fn queue_empty(&self) -> bool {
        self.active.is_empty()
    }

    /// Start tracking resources associated with a new queue submission.
    pub fn track_submission(&mut self, index: SubmissionIndex, encoders: Vec<EncoderInFlight>) {
        self.active.push(ActiveSubmission {
            index,
            mapped: Vec::new(),
            compact_read_back: Vec::new(),
            encoders,
            work_done_closures: SmallVec::new(),
        });
    }

    pub(crate) fn map(&mut self, buffer: &Arc<Buffer>) -> Option<SubmissionIndex> {
        // Determine which buffers are ready to map, and which must wait for the GPU.
        let submission = self
            .active
            .iter_mut()
            .rev()
            .find(|a| a.contains_buffer(buffer));

        let maybe_submission_index = submission.as_ref().map(|s| s.index);

        submission
            .map_or(&mut self.ready_to_map, |a| &mut a.mapped)
            .push(buffer.clone());

        maybe_submission_index
    }

    pub(crate) fn prepare_compact(&mut self, blas: &Arc<Blas>) -> Option<SubmissionIndex> {
        // Determine which BLASes are ready to map, and which must wait for the GPU.
        let submission = self.active.iter_mut().rev().find(|a| a.contains_blas(blas));

        let maybe_submission_index = submission.as_ref().map(|s| s.index);

        submission
            .map_or(&mut self.ready_to_compact, |a| &mut a.compact_read_back)
            .push(blas.clone());

        maybe_submission_index
    }

    /// Returns the submission index of the most recent submission that uses the
    /// given buffer.
    pub fn get_buffer_latest_submission_index(&self, buffer: &Buffer) -> Option<SubmissionIndex> {
        // We iterate in reverse order, so that we can bail out early as soon
        // as we find a hit.
        self.active.iter().rev().find_map(|submission| {
            if submission.contains_buffer(buffer) {
                Some(submission.index)
            } else {
                None
            }
        })
    }

    /// Returns the submission index of the most recent submission that uses the
    /// given texture.
    pub fn get_texture_latest_submission_index(
        &self,
        texture: &Texture,
    ) -> Option<SubmissionIndex> {
        // We iterate in reverse order, so that we can bail out early as soon
        // as we find a hit.
        self.active.iter().rev().find_map(|submission| {
            if submission.contains_texture(texture) {
                Some(submission.index)
            } else {
                None
            }
        })
    }

    /// Sort out the consequences of completed submissions.
    ///
    /// Assume that all submissions up through `last_done` have completed.
    ///
    /// -   Buffers used by those submissions are now ready to map, if requested.
    ///     Add any buffers in the submission's [`mapped`] list to
    ///     [`self.ready_to_map`], where [`LifetimeTracker::handle_mapping`]
    ///     will find them.
    ///
    /// Return a list of [`SubmittedWorkDoneClosure`]s to run.
    ///
    /// [`mapped`]: ActiveSubmission::mapped
    /// [`self.ready_to_map`]: LifetimeTracker::ready_to_map
    /// [`SubmittedWorkDoneClosure`]: crate::device::queue::SubmittedWorkDoneClosure
    #[must_use]
    pub fn triage_submissions(
        &mut self,
        last_done: SubmissionIndex,
    ) -> SmallVec<[SubmittedWorkDoneClosure; 1]> {
        profiling::scope!("triage_submissions");

        //TODO: enable when `is_sorted_by_key` is stable
        //debug_assert!(self.active.is_sorted_by_key(|a| a.index));
        let done_count = self
            .active
            .iter()
            .position(|a| a.index > last_done)
            .unwrap_or(self.active.len());

        let mut work_done_closures: SmallVec<_> = self.work_done_closures.drain(..).collect();
        for a in self.active.drain(..done_count) {
            self.ready_to_map.extend(a.mapped);
            self.ready_to_compact.extend(a.compact_read_back);
            for encoder in a.encoders {
                // This involves actually decrementing the ref count of all command buffer
                // resources, so can be _very_ expensive.
                profiling::scope!("drop command buffer trackers");
                drop(encoder);
            }
            work_done_closures.extend(a.work_done_closures);
        }
        work_done_closures
    }

    pub fn schedule_resource_destruction(
        &mut self,
        temp_resource: TempResource,
        last_submit_index: SubmissionIndex,
    ) {
        let resources = self
            .active
            .iter_mut()
            .find(|a| a.index == last_submit_index)
            .map(|a| {
                // Because this resource's `last_submit_index` matches `a.index`,
                // we know that we must have done something with the resource,
                // so `a.encoders` should not be empty.
                &mut a.encoders.last_mut().unwrap().temp_resources
            });
        if let Some(resources) = resources {
            resources.push(temp_resource);
        }
    }

    pub fn add_work_done_closure(
        &mut self,
        closure: SubmittedWorkDoneClosure,
    ) -> Option<SubmissionIndex> {
        match self.active.last_mut() {
            Some(active) => {
                active.work_done_closures.push(closure);
                Some(active.index)
            }
            // We must defer the closure until all previously occurring map_async closures
            // have fired. This is required by the spec.
            None => {
                self.work_done_closures.push(closure);
                None
            }
        }
    }

    /// Map the buffers in `self.ready_to_map`.
    ///
    /// Return a list of mapping notifications to send.
    ///
    /// See the documentation for [`LifetimeTracker`] for details.
    #[must_use]
    pub(crate) fn handle_mapping(
        &mut self,
        snatch_guard: &SnatchGuard,
    ) -> Vec<super::BufferMapPendingClosure> {
        if self.ready_to_map.is_empty() {
            return Vec::new();
        }
        let mut pending_callbacks: Vec<super::BufferMapPendingClosure> =
            Vec::with_capacity(self.ready_to_map.len());

        for buffer in self.ready_to_map.drain(..) {
            match buffer.map(snatch_guard) {
                Some(cb) => pending_callbacks.push(cb),
                None => continue,
            }
        }
        pending_callbacks
    }
    /// Read back compact sizes from the BLASes in `self.ready_to_compact`.
    ///
    /// Return a list of mapping notifications to send.
    ///
    /// See the documentation for [`LifetimeTracker`] for details.
    #[must_use]
    pub(crate) fn handle_compact_read_back(&mut self) -> Vec<BlasCompactReadyPendingClosure> {
        if self.ready_to_compact.is_empty() {
            return Vec::new();
        }
        let mut pending_callbacks: Vec<BlasCompactReadyPendingClosure> =
            Vec::with_capacity(self.ready_to_compact.len());

        for blas in self.ready_to_compact.drain(..) {
            match blas.read_back_compact_size() {
                Some(cb) => pending_callbacks.push(cb),
                None => continue,
            }
        }
        pending_callbacks
    }
}

//! Definition of the [`SemaphoreList`] type.

use alloc::vec::Vec;
use ash::vk;
use core::mem::MaybeUninit;

#[derive(Debug, PartialEq)]
pub enum SemaphoreListMode {
    Wait,
    Signal,
}

/// A list of Vulkan semaphores to wait for or signal.
///
/// This represents a list of binary or timeline semaphores, together
/// with values for the timeline semaphores, and stage masks, if these
/// are used for waiting.
///
/// This type ensures that the array of semaphores to be signaled
/// stays aligned with the array of values for timeline semaphores
/// appearing in that list. The [`add_to_submit`] method prepares the
/// `vkQueueSubmit` arguments appropriately for whatever semaphores we
/// actually have.
///
/// [`add_to_submit`]: SemaphoreList::add_to_submit
#[derive(Debug)]
pub struct SemaphoreList {
    /// Mode of the semaphore list. Used for validation.
    mode: SemaphoreListMode,

    /// Semaphores to use.
    ///
    /// This can be a mix of binary and timeline semaphores.
    semaphores: Vec<vk::Semaphore>,

    /// Values for the timeline semaphores.
    ///
    /// If no timeline semaphores are present in [`semaphores`], this
    /// is empty. If any timeline semaphores are present, then this
    /// has the same length as [`semaphores`], with dummy !0 values
    /// in the elements corresponding to binary semaphores, since
    /// Vulkan ignores these.
    ///
    /// [`semaphores`]: Self::semaphores
    values: Vec<u64>,

    /// Stage masks for wait semaphores.
    ///
    /// This is only used if `mode` is `Wait`.
    pub stage_masks: Vec<vk::PipelineStageFlags>,
}

impl SemaphoreList {
    pub fn new(mode: SemaphoreListMode) -> Self {
        Self {
            mode,
            semaphores: Vec::new(),
            values: Vec::new(),
            stage_masks: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.semaphores.is_empty()
    }

    /// Add this list to the semaphores to be signalled by a `vkQueueSubmit` call.
    ///
    /// - Set `submit_info`'s `pSignalSemaphores` list to this list's
    ///   semaphores.
    ///
    /// - If this list contains any timeline semaphores, then initialize
    ///   `timeline_info`, set its `pSignalSemaphoreValues` to this
    ///   list's values, and add it to `submit_info`s extension chain.
    ///
    /// Return the revised `submit_info` value.
    pub fn add_to_submit<'info, 'semaphores: 'info>(
        wait_semaphores: &'semaphores mut Self,
        signal_semaphores: &'semaphores mut Self,
        submit_info: vk::SubmitInfo<'info>,
        timeline_info: &'info mut MaybeUninit<vk::TimelineSemaphoreSubmitInfo<'info>>,
    ) -> vk::SubmitInfo<'info> {
        wait_semaphores.check();
        signal_semaphores.check();

        assert!(matches!(wait_semaphores.mode, SemaphoreListMode::Wait));
        assert!(matches!(signal_semaphores.mode, SemaphoreListMode::Signal));

        let timeline_info = timeline_info.write(vk::TimelineSemaphoreSubmitInfo::default());

        let mut uses_timeline = false;

        if !wait_semaphores.values.is_empty() {
            *timeline_info = timeline_info.wait_semaphore_values(&wait_semaphores.values);
            uses_timeline = true;
        }

        if !signal_semaphores.values.is_empty() {
            *timeline_info = timeline_info.signal_semaphore_values(&signal_semaphores.values);
            uses_timeline = true;
        }

        let mut submit_info = submit_info
            .wait_semaphores(&wait_semaphores.semaphores)
            .wait_dst_stage_mask(&wait_semaphores.stage_masks)
            .signal_semaphores(&signal_semaphores.semaphores);

        if uses_timeline {
            submit_info = submit_info.push_next(timeline_info);
        }

        submit_info
    }

    /// Add a semaphore to be signaled. Panics if this is a list of semaphores to wait.
    pub fn push_signal(&mut self, semaphore: SemaphoreType) {
        assert!(matches!(self.mode, SemaphoreListMode::Signal));
        self.push_inner(semaphore);
    }

    /// Add a semaphore to be waited for. Panics if this is a list of semaphores to signal.
    pub fn push_wait(&mut self, semaphore: SemaphoreType, stage: vk::PipelineStageFlags) {
        assert!(matches!(self.mode, SemaphoreListMode::Wait));

        self.stage_masks.push(stage);
        self.push_inner(semaphore);
    }

    fn push_inner(&mut self, semaphore: SemaphoreType) {
        match semaphore {
            SemaphoreType::Binary(semaphore) => {
                self.semaphores.push(semaphore);
                // Push a dummy value if necessary.
                if !self.values.is_empty() {
                    self.values.push(!0);
                }
            }
            SemaphoreType::Timeline(semaphore, value) => {
                // We may be the first timeline semaphore, ensure that the values
                // array is filled with dummy values for existing binary semaphores.
                self.pad_values();
                self.semaphores.push(semaphore);
                self.values.push(value);
            }
        }

        self.check();
    }

    /// Append `other` to `self`, leaving `other` empty.
    pub fn append(&mut self, other: &mut Self) {
        assert_eq!(self.mode, other.mode);

        // If we're about to receive values, ensure we're aligned first.
        if !other.values.is_empty() {
            self.pad_values();
        }
        self.semaphores.append(&mut other.semaphores);
        self.values.append(&mut other.values);
        // If we had values, but `other` did not, re-align.
        if !self.values.is_empty() {
            self.pad_values();
        }
        self.stage_masks.append(&mut other.stage_masks);
        self.check();
    }

    /// Pad `self.values` with dummy values for binary semaphores,
    /// in preparation for adding a timeline semaphore value.
    ///
    /// This is a no-op if we already have values.
    fn pad_values(&mut self) {
        self.values.resize(self.semaphores.len(), !0);
    }

    #[track_caller]
    fn check(&self) {
        debug_assert!(self.values.is_empty() || self.values.len() == self.semaphores.len());
        match self.mode {
            SemaphoreListMode::Wait => {
                debug_assert!(
                    self.stage_masks.is_empty() || self.stage_masks.len() == self.semaphores.len()
                );
            }
            SemaphoreListMode::Signal => {
                debug_assert!(self.stage_masks.is_empty());
            }
        }
    }
}

pub enum SemaphoreType {
    Binary(vk::Semaphore),
    Timeline(vk::Semaphore, u64),
}

//! Buffer Trackers
//!
//! Buffers are represented by a single state for the whole resource,
//! a 16 bit bitflag of buffer usages. Because there is only ever
//! one subresource, they have no selector.

use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

use hal::BufferBarrier;
use wgt::{strict_assert, strict_assert_eq, BufferUses};

use super::{PendingTransition, TrackerIndex};
use crate::{
    resource::{Buffer, Trackable},
    snatch::SnatchGuard,
    track::{
        invalid_resource_state, skip_barrier, ResourceMetadata, ResourceMetadataProvider,
        ResourceUsageCompatibilityError, ResourceUses,
    },
};

impl ResourceUses for BufferUses {
    const EXCLUSIVE: Self = Self::EXCLUSIVE;

    type Selector = ();

    fn bits(self) -> u16 {
        Self::bits(&self)
    }

    fn any_exclusive(self) -> bool {
        self.intersects(Self::EXCLUSIVE)
    }
}

/// Stores a bind group's buffers + their usages (within the bind group).
#[derive(Debug)]
pub(crate) struct BufferBindGroupState {
    buffers: Vec<(Arc<Buffer>, BufferUses)>,
}
impl BufferBindGroupState {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
        }
    }

    /// Optimize the buffer bind group state by sorting it by ID.
    ///
    /// When this list of states is merged into a tracker, the memory
    /// accesses will be in a constant ascending order.
    pub(crate) fn optimize(&mut self) {
        self.buffers
            .sort_unstable_by_key(|(b, _)| b.tracker_index());
    }

    /// Returns a list of all buffers tracked. May contain duplicates.
    pub fn used_tracker_indices(&self) -> impl Iterator<Item = TrackerIndex> + '_ {
        self.buffers
            .iter()
            .map(|(b, _)| b.tracker_index())
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Adds the given resource with the given state.
    pub fn insert_single(&mut self, buffer: Arc<Buffer>, state: BufferUses) {
        self.buffers.push((buffer, state));
    }
}

/// Stores all buffer state within a single usage scope.
#[derive(Debug)]
pub(crate) struct BufferUsageScope {
    state: Vec<BufferUses>,
    metadata: ResourceMetadata<Arc<Buffer>>,
    ordered_uses_mask: BufferUses,
}

impl Default for BufferUsageScope {
    fn default() -> Self {
        Self {
            state: Vec::new(),
            metadata: ResourceMetadata::new(),
            ordered_uses_mask: BufferUses::empty(),
        }
    }
}

impl BufferUsageScope {
    fn tracker_assert_in_bounds(&self, index: usize) {
        strict_assert!(index < self.state.len());
        self.metadata.tracker_assert_in_bounds(index);
    }
    pub fn clear(&mut self) {
        self.state.clear();
        self.metadata.clear();
    }

    /// Sets the size of all the vectors inside the tracker.
    ///
    /// Must be called with the highest possible Buffer ID before
    /// all unsafe functions are called.
    pub fn set_size(&mut self, size: usize) {
        self.state.resize(size, BufferUses::empty());
        self.metadata.set_size(size);
    }

    pub fn set_ordered_uses_mask(&mut self, ordered_uses_mask: BufferUses) {
        self.ordered_uses_mask = ordered_uses_mask;
    }

    /// Extend the vectors to let the given index be valid.
    fn allow_index(&mut self, index: usize) {
        if index >= self.state.len() {
            self.set_size(index + 1);
        }
    }

    /// Merge the list of buffer states in the given bind group into this usage scope.
    ///
    /// If any of the resulting states is invalid, stops the merge and returns a usage
    /// conflict with the details of the invalid state.
    ///
    /// Because bind groups do not check if the union of all their states is valid,
    /// this method is allowed to return Err on the first bind group bound.
    ///
    /// # Safety
    ///
    /// [`Self::set_size`] must be called with the maximum possible Buffer ID before this
    /// method is called.
    pub unsafe fn merge_bind_group(
        &mut self,
        bind_group: &BufferBindGroupState,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        for &(ref resource, state) in bind_group.buffers.iter() {
            let index = resource.tracker_index().as_usize();

            unsafe {
                self.insert_or_merge(
                    index as _,
                    index,
                    BufferStateProvider::Direct { state },
                    ResourceMetadataProvider::Direct { resource },
                )?
            };
        }

        Ok(())
    }

    /// Merge the list of buffer states in the given usage scope into this UsageScope.
    ///
    /// If any of the resulting states is invalid, stops the merge and returns a usage
    /// conflict with the details of the invalid state.
    ///
    /// If the given tracker uses IDs higher than the length of internal vectors,
    /// the vectors will be extended. A call to set_size is not needed.
    pub fn merge_usage_scope(
        &mut self,
        scope: &Self,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        let incoming_size = scope.state.len();
        if incoming_size > self.state.len() {
            self.set_size(incoming_size);
        }

        for index in scope.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);
            scope.tracker_assert_in_bounds(index);

            unsafe {
                self.insert_or_merge(
                    index as u32,
                    index,
                    BufferStateProvider::Indirect {
                        state: &scope.state,
                    },
                    ResourceMetadataProvider::Indirect {
                        metadata: &scope.metadata,
                    },
                )?;
            };
        }

        Ok(())
    }

    /// Merge a single state into the UsageScope.
    ///
    /// If the resulting state is invalid, returns a usage
    /// conflict with the details of the invalid state.
    ///
    /// If the ID is higher than the length of internal vectors,
    /// the vectors will be extended. A call to set_size is not needed.
    pub fn merge_single(
        &mut self,
        buffer: &Arc<Buffer>,
        new_state: BufferUses,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        let index = buffer.tracker_index().as_usize();

        self.allow_index(index);

        self.tracker_assert_in_bounds(index);

        unsafe {
            self.insert_or_merge(
                index as _,
                index,
                BufferStateProvider::Direct { state: new_state },
                ResourceMetadataProvider::Direct { resource: buffer },
            )?;
        }

        Ok(())
    }

    /// Does an insertion operation if the index isn't tracked
    /// in the current metadata, otherwise merges the given state
    /// with the current state. If the merging would cause
    /// a conflict, returns that usage conflict.
    ///
    /// # Safety
    ///
    /// Indexes must be valid indexes into all arrays passed in
    /// to this function, either directly or via metadata or provider structs.
    #[inline(always)]
    unsafe fn insert_or_merge(
        &mut self,
        index32: u32,
        index: usize,
        state_provider: BufferStateProvider<'_>,
        metadata_provider: ResourceMetadataProvider<'_, Arc<Buffer>>,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        let currently_owned = unsafe { self.metadata.contains_unchecked(index) };

        if !currently_owned {
            unsafe {
                insert(
                    None,
                    &mut self.state,
                    &mut self.metadata,
                    index,
                    state_provider,
                    None,
                    metadata_provider,
                )
            };
            return Ok(());
        }

        unsafe {
            merge(
                &mut self.state,
                index32,
                index,
                state_provider,
                metadata_provider,
            )
        }
    }

    /// Removes the indicated usage from the scope.
    ///
    /// Note that multiple uses of the same type get merged. It is only
    /// safe to remove a usage if you are certain you aren't going to
    /// erase another usage you don't know about.
    pub fn remove_usage(&mut self, buffer: &Buffer, usage: BufferUses) {
        let index = buffer.tracker_index().as_usize();
        if self.metadata.contains(index) {
            // SAFETY: If the buffer is part of this usage scope, then the index
            // is in range.
            unsafe {
                *self.state.get_unchecked_mut(index) &= !usage;
            }
        }
    }
}

/// Stores all buffer state within a command buffer.
pub(crate) struct BufferTracker {
    start: Vec<BufferUses>,
    end: Vec<BufferUses>,

    metadata: ResourceMetadata<Arc<Buffer>>,

    temp: Vec<PendingTransition<BufferUses>>,

    ordered_uses_mask: BufferUses,
}

impl BufferTracker {
    pub fn new(ordered_uses_mask: BufferUses) -> Self {
        Self {
            start: Vec::new(),
            end: Vec::new(),

            metadata: ResourceMetadata::new(),

            temp: Vec::new(),

            ordered_uses_mask,
        }
    }

    fn tracker_assert_in_bounds(&self, index: usize) {
        strict_assert!(index < self.start.len());
        strict_assert!(index < self.end.len());
        self.metadata.tracker_assert_in_bounds(index);
    }

    /// Sets the size of all the vectors inside the tracker.
    ///
    /// Must be called with the highest possible Buffer ID before
    /// all unsafe functions are called.
    pub fn set_size(&mut self, size: usize) {
        self.start.resize(size, BufferUses::empty());
        self.end.resize(size, BufferUses::empty());

        self.metadata.set_size(size);
    }

    /// Extend the vectors to let the given index be valid.
    fn allow_index(&mut self, index: usize) {
        if index >= self.start.len() {
            self.set_size(index + 1);
        }
    }

    /// Returns true if the given buffer is tracked.
    pub fn contains(&self, buffer: &Buffer) -> bool {
        self.metadata.contains(buffer.tracker_index().as_usize())
    }

    /// Returns a list of all buffers tracked.
    pub fn used_resources(&self) -> impl Iterator<Item = &Arc<Buffer>> + '_ {
        self.metadata.owned_resources()
    }

    /// Drains all currently pending transitions.
    pub fn drain_transitions<'a, 'b: 'a>(
        &'b mut self,
        snatch_guard: &'a SnatchGuard<'a>,
    ) -> impl Iterator<Item = BufferBarrier<'a, dyn hal::DynBuffer>> {
        let buffer_barriers = self.temp.drain(..).map(|pending| {
            let buf = unsafe { self.metadata.get_resource_unchecked(pending.id as _) };
            pending.into_hal(buf, snatch_guard)
        });
        buffer_barriers
    }

    /// Sets the state of a single buffer.
    ///
    /// If a transition is needed to get the buffer into the given state, that transition
    /// is returned. No more than one transition is needed.
    ///
    /// If the ID is higher than the length of internal vectors,
    /// the vectors will be extended. A call to set_size is not needed.
    pub fn set_single(
        &mut self,
        buffer: &Arc<Buffer>,
        state: BufferUses,
    ) -> Option<PendingTransition<BufferUses>> {
        let index: usize = buffer.tracker_index().as_usize();

        self.allow_index(index);

        self.tracker_assert_in_bounds(index);

        unsafe {
            self.insert_or_barrier_update(
                index,
                BufferStateProvider::Direct { state },
                None,
                ResourceMetadataProvider::Direct { resource: buffer },
            )
        };

        strict_assert!(self.temp.len() <= 1);

        self.temp.pop()
    }

    /// Sets the given state for all buffers in the given tracker.
    ///
    /// If a transition is needed to get the buffers into the needed state,
    /// those transitions are stored within the tracker. A subsequent
    /// call to [`Self::drain_transitions`] is needed to get those transitions.
    ///
    /// If the ID is higher than the length of internal vectors,
    /// the vectors will be extended. A call to set_size is not needed.
    pub fn set_from_tracker(&mut self, tracker: &Self) {
        let incoming_size = tracker.start.len();
        if incoming_size > self.start.len() {
            self.set_size(incoming_size);
        }

        for index in tracker.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);
            tracker.tracker_assert_in_bounds(index);
            unsafe {
                self.insert_or_barrier_update(
                    index,
                    BufferStateProvider::Indirect {
                        state: &tracker.start,
                    },
                    Some(BufferStateProvider::Indirect {
                        state: &tracker.end,
                    }),
                    ResourceMetadataProvider::Indirect {
                        metadata: &tracker.metadata,
                    },
                )
            }
        }
    }

    /// Sets the given state for all buffers in the given UsageScope.
    ///
    /// If a transition is needed to get the buffers into the needed state,
    /// those transitions are stored within the tracker. A subsequent
    /// call to [`Self::drain_transitions`] is needed to get those transitions.
    ///
    /// If the ID is higher than the length of internal vectors,
    /// the vectors will be extended. A call to set_size is not needed.
    pub fn set_from_usage_scope(&mut self, scope: &BufferUsageScope) {
        let incoming_size = scope.state.len();
        if incoming_size > self.start.len() {
            self.set_size(incoming_size);
        }

        for index in scope.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);
            scope.tracker_assert_in_bounds(index);
            unsafe {
                self.insert_or_barrier_update(
                    index,
                    BufferStateProvider::Indirect {
                        state: &scope.state,
                    },
                    None,
                    ResourceMetadataProvider::Indirect {
                        metadata: &scope.metadata,
                    },
                )
            }
        }
    }

    /// Iterates through all buffers in the given bind group and adopts
    /// the state given for those buffers in the UsageScope. It also
    /// removes all touched buffers from the usage scope.
    ///
    /// If a transition is needed to get the buffers into the needed state,
    /// those transitions are stored within the tracker. A subsequent
    /// call to [`Self::drain_transitions`] is needed to get those transitions.
    ///
    /// This is a really funky method used by Compute Passes to generate
    /// barriers after a call to dispatch without needing to iterate
    /// over all elements in the usage scope. We use each the
    /// a given iterator of ids as a source of which IDs to look at.
    /// All the IDs must have first been added to the usage scope.
    ///
    /// # Panics
    ///
    /// If a resource identified by `index_source` is not found in the usage
    /// scope.
    pub fn set_and_remove_from_usage_scope_sparse(
        &mut self,
        scope: &mut BufferUsageScope,
        index_source: impl IntoIterator<Item = TrackerIndex>,
    ) {
        let incoming_size = scope.state.len();
        if incoming_size > self.start.len() {
            self.set_size(incoming_size);
        }

        for index in index_source {
            let index = index.as_usize();

            scope.tracker_assert_in_bounds(index);

            if unsafe { !scope.metadata.contains_unchecked(index) } {
                continue;
            }

            // SAFETY: we checked that the index is in bounds for the scope, and
            // called `set_size` to ensure it is valid for `self`.
            unsafe {
                self.insert_or_barrier_update(
                    index,
                    BufferStateProvider::Indirect {
                        state: &scope.state,
                    },
                    None,
                    ResourceMetadataProvider::Indirect {
                        metadata: &scope.metadata,
                    },
                )
            };

            unsafe { scope.metadata.remove(index) };
        }
    }

    /// If the resource isn't tracked
    /// - Inserts the given resource.
    /// - Uses the `start_state_provider` to populate `start_states`
    /// - Uses either `end_state_provider` or `start_state_provider`
    ///   to populate `current_states`.
    ///
    /// If the resource is tracked
    /// - Inserts barriers from the state in `current_states`
    ///   to the state provided by `start_state_provider`.
    /// - Updates the `current_states` with either the state from
    ///   `end_state_provider` or `start_state_provider`.
    ///
    /// Any barriers are added to the barrier vector.
    ///
    /// # Safety
    ///
    /// Indexes must be valid indexes into all arrays passed in
    /// to this function, either directly or via metadata or provider structs.
    #[inline(always)]
    unsafe fn insert_or_barrier_update(
        &mut self,
        index: usize,
        start_state_provider: BufferStateProvider<'_>,
        end_state_provider: Option<BufferStateProvider<'_>>,
        metadata_provider: ResourceMetadataProvider<'_, Arc<Buffer>>,
    ) {
        let currently_owned = unsafe { self.metadata.contains_unchecked(index) };

        if !currently_owned {
            unsafe {
                insert(
                    Some(&mut self.start),
                    &mut self.end,
                    &mut self.metadata,
                    index,
                    start_state_provider,
                    end_state_provider,
                    metadata_provider,
                )
            };
            return;
        }

        let update_state_provider =
            end_state_provider.unwrap_or_else(|| start_state_provider.clone());
        unsafe {
            barrier(
                &mut self.end,
                index,
                start_state_provider,
                &mut self.temp,
                self.ordered_uses_mask,
            )
        };

        unsafe { update(&mut self.end, index, update_state_provider) };
    }
}

/// Stores all buffer state within a device.
pub(crate) struct DeviceBufferTracker {
    current_states: Vec<BufferUses>,
    metadata: ResourceMetadata<Weak<Buffer>>,
    temp: Vec<PendingTransition<BufferUses>>,
    ordered_uses_mask: BufferUses,
}

impl DeviceBufferTracker {
    pub fn new(ordered_uses_mask: BufferUses) -> Self {
        Self {
            current_states: Vec::new(),
            metadata: ResourceMetadata::new(),
            temp: Vec::new(),
            ordered_uses_mask,
        }
    }

    fn tracker_assert_in_bounds(&self, index: usize) {
        strict_assert!(index < self.current_states.len());
        self.metadata.tracker_assert_in_bounds(index);
    }

    /// Extend the vectors to let the given index be valid.
    fn allow_index(&mut self, index: usize) {
        if index >= self.current_states.len() {
            self.current_states.resize(index + 1, BufferUses::empty());
            self.metadata.set_size(index + 1);
        }
    }

    /// Returns a list of all buffers tracked.
    pub fn used_resources(&self) -> impl Iterator<Item = &Weak<Buffer>> + '_ {
        self.metadata.owned_resources()
    }

    /// Inserts a single buffer and its state into the resource tracker.
    ///
    /// If the resource already exists in the tracker, it will be overwritten.
    pub fn insert_single(&mut self, buffer: &Arc<Buffer>, state: BufferUses) {
        let index = buffer.tracker_index().as_usize();

        self.allow_index(index);

        self.tracker_assert_in_bounds(index);

        unsafe {
            insert(
                None,
                &mut self.current_states,
                &mut self.metadata,
                index,
                BufferStateProvider::Direct { state },
                None,
                ResourceMetadataProvider::Direct {
                    resource: &Arc::downgrade(buffer),
                },
            )
        }
    }

    /// Sets the state of a single buffer.
    ///
    /// If a transition is needed to get the buffer into the given state, that transition
    /// is returned. No more than one transition is needed.
    pub fn set_single(
        &mut self,
        buffer: &Arc<Buffer>,
        state: BufferUses,
    ) -> Option<PendingTransition<BufferUses>> {
        let index: usize = buffer.tracker_index().as_usize();

        self.tracker_assert_in_bounds(index);

        let start_state_provider = BufferStateProvider::Direct { state };

        unsafe {
            barrier(
                &mut self.current_states,
                index,
                start_state_provider.clone(),
                &mut self.temp,
                self.ordered_uses_mask,
            )
        };
        unsafe { update(&mut self.current_states, index, start_state_provider) };

        strict_assert!(self.temp.len() <= 1);

        self.temp.pop()
    }

    /// Sets the given state for all buffers in the given tracker.
    ///
    /// If a transition is needed to get the buffers into the needed state,
    /// those transitions are returned.
    pub fn set_from_tracker_and_drain_transitions<'a, 'b: 'a>(
        &'a mut self,
        tracker: &'a BufferTracker,
        snatch_guard: &'b SnatchGuard<'b>,
    ) -> impl Iterator<Item = BufferBarrier<'a, dyn hal::DynBuffer>> {
        for index in tracker.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);

            let start_state_provider = BufferStateProvider::Indirect {
                state: &tracker.start,
            };
            let end_state_provider = BufferStateProvider::Indirect {
                state: &tracker.end,
            };
            unsafe {
                barrier(
                    &mut self.current_states,
                    index,
                    start_state_provider,
                    &mut self.temp,
                    self.ordered_uses_mask,
                )
            };
            unsafe { update(&mut self.current_states, index, end_state_provider) };
        }

        self.temp.drain(..).map(|pending| {
            let buf = unsafe { tracker.metadata.get_resource_unchecked(pending.id as _) };
            pending.into_hal(buf, snatch_guard)
        })
    }
}

/// Source of Buffer State.
#[derive(Debug, Clone)]
enum BufferStateProvider<'a> {
    /// Get a state that was provided directly.
    Direct { state: BufferUses },
    /// Get a state from an an array of states.
    Indirect { state: &'a [BufferUses] },
}
impl BufferStateProvider<'_> {
    /// Gets the state from the provider, given a resource ID index.
    ///
    /// # Safety
    ///
    /// Index must be in bounds for the indirect source iff this is in the indirect state.
    #[inline(always)]
    unsafe fn get_state(&self, index: usize) -> BufferUses {
        match *self {
            BufferStateProvider::Direct { state } => state,
            BufferStateProvider::Indirect { state } => {
                strict_assert!(index < state.len());
                *unsafe { state.get_unchecked(index) }
            }
        }
    }
}

#[inline(always)]
unsafe fn insert<T: Clone>(
    start_states: Option<&mut [BufferUses]>,
    current_states: &mut [BufferUses],
    resource_metadata: &mut ResourceMetadata<T>,
    index: usize,
    start_state_provider: BufferStateProvider<'_>,
    end_state_provider: Option<BufferStateProvider<'_>>,
    metadata_provider: ResourceMetadataProvider<'_, T>,
) {
    let new_start_state = unsafe { start_state_provider.get_state(index) };
    let new_end_state =
        end_state_provider.map_or(new_start_state, |p| unsafe { p.get_state(index) });

    // This should only ever happen with a wgpu bug, but let's just double
    // check that resource states don't have any conflicts.
    strict_assert_eq!(invalid_resource_state(new_start_state), false);
    strict_assert_eq!(invalid_resource_state(new_end_state), false);

    unsafe {
        if let Some(&mut ref mut start_state) = start_states {
            *start_state.get_unchecked_mut(index) = new_start_state;
        }
        *current_states.get_unchecked_mut(index) = new_end_state;

        let resource = metadata_provider.get(index);
        resource_metadata.insert(index, resource.clone());
    }
}

#[inline(always)]
unsafe fn merge(
    current_states: &mut [BufferUses],
    _index32: u32,
    index: usize,
    state_provider: BufferStateProvider<'_>,
    metadata_provider: ResourceMetadataProvider<'_, Arc<Buffer>>,
) -> Result<(), ResourceUsageCompatibilityError> {
    let current_state = unsafe { current_states.get_unchecked_mut(index) };
    let new_state = unsafe { state_provider.get_state(index) };

    let merged_state = *current_state | new_state;

    if invalid_resource_state(merged_state) {
        return Err(ResourceUsageCompatibilityError::from_buffer(
            unsafe { metadata_provider.get(index) },
            *current_state,
            new_state,
        ));
    }

    *current_state = merged_state;

    Ok(())
}

#[inline(always)]
unsafe fn barrier(
    current_states: &mut [BufferUses],
    index: usize,
    state_provider: BufferStateProvider<'_>,
    barriers: &mut Vec<PendingTransition<BufferUses>>,
    ordered_uses_mask: BufferUses,
) {
    let current_state = unsafe { *current_states.get_unchecked(index) };
    let new_state = unsafe { state_provider.get_state(index) };

    if skip_barrier(current_state, ordered_uses_mask, new_state) {
        return;
    }

    barriers.push(PendingTransition {
        id: index as _,
        selector: (),
        usage: hal::StateTransition {
            from: current_state,
            to: new_state,
        },
    });
}

#[inline(always)]
unsafe fn update(
    current_states: &mut [BufferUses],
    index: usize,
    state_provider: BufferStateProvider<'_>,
) {
    let current_state = unsafe { current_states.get_unchecked_mut(index) };
    let new_state = unsafe { state_provider.get_state(index) };

    *current_state = new_state;
}

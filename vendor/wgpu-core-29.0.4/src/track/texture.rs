//! Texture Trackers
//!
//! Texture trackers are significantly more complicated than
//! the buffer trackers because textures can be in a "complex"
//! state where each individual subresource can potentially be
//! in a different state from every other subtresource. These
//! complex states are stored separately from the simple states
//! because they are signifignatly more difficult to track and
//! most resources spend the vast majority of their lives in
//! simple states.
//!
//! There are two special texture usages: `UNKNOWN` and `UNINITIALIZED`.
//! - `UNKNOWN` is only used in complex states and is used to signify
//!   that the complex state does not know anything about those subresources.
//!   It cannot leak into transitions, it is invalid to transition into UNKNOWN
//!   state.
//! - `UNINITIALIZED` is used in both simple and complex states to mean the texture
//!   is known to be in some undefined state. Any transition away from UNINITIALIZED
//!   will treat the contents as junk.

use super::{range::RangedStates, PendingTransition, PendingTransitionList};
use crate::{
    resource::{RawResourceAccess, Texture, TextureInner, TextureView, Trackable},
    snatch::SnatchGuard,
    track::{
        invalid_resource_state, skip_barrier, ResourceMetadata, ResourceMetadataProvider,
        ResourceUsageCompatibilityError, ResourceUses,
    },
};
use hal::TextureBarrier;

use arrayvec::ArrayVec;
use naga::FastHashMap;

use wgt::{strict_assert, strict_assert_eq, TextureSelector, TextureUses};

use alloc::{
    sync::{Arc, Weak},
    vec::{Drain, Vec},
};
use core::iter;

impl ResourceUses for TextureUses {
    const EXCLUSIVE: Self = Self::EXCLUSIVE;

    type Selector = TextureSelector;

    fn bits(self) -> u16 {
        Self::bits(&self)
    }

    fn any_exclusive(self) -> bool {
        self.intersects(Self::EXCLUSIVE)
    }
}

/// Represents the complex state of textures where every subresource is potentially
/// in a different state.
#[derive(Clone, Debug, Default, PartialEq)]
struct ComplexTextureState {
    mips: ArrayVec<RangedStates<u32, TextureUses>, { hal::MAX_MIP_LEVELS as usize }>,
}

impl ComplexTextureState {
    /// Creates complex texture state for the given sizes.
    ///
    /// This state will be initialized with the UNKNOWN state, a special state
    /// which means the trakcer knows nothing about the state.
    fn new(mip_level_count: u32, array_layer_count: u32) -> Self {
        Self {
            mips: iter::repeat_with(|| {
                RangedStates::from_range(0..array_layer_count, TextureUses::UNKNOWN)
            })
            .take(mip_level_count as usize)
            .collect(),
        }
    }

    /// Initialize a complex state from a selector representing the full size of the texture
    /// and an iterator of a selector and a texture use, specifying a usage for a specific
    /// set of subresources.
    ///
    /// [`Self::to_selector_state_iter`] can be used to create such an iterator.
    ///
    /// # Safety
    ///
    /// All selectors in the iterator must be inside of the full_range selector.
    ///
    /// The full range selector must have mips and layers start at 0.
    unsafe fn from_selector_state_iter(
        full_range: TextureSelector,
        state_iter: impl Iterator<Item = (TextureSelector, TextureUses)>,
    ) -> Self {
        strict_assert_eq!(full_range.layers.start, 0);
        strict_assert_eq!(full_range.mips.start, 0);

        let mut complex =
            ComplexTextureState::new(full_range.mips.len() as u32, full_range.layers.len() as u32);
        for (selector, desired_state) in state_iter {
            strict_assert!(selector.layers.end <= full_range.layers.end);
            strict_assert!(selector.mips.end <= full_range.mips.end);

            // This should only ever happen with a wgpu bug, but let's just double
            // check that resource states don't have any conflicts.
            strict_assert_eq!(invalid_resource_state(desired_state), false);

            let mips = selector.mips.start as usize..selector.mips.end as usize;
            for mip in unsafe { complex.mips.get_unchecked_mut(mips) } {
                for &mut (_, ref mut state) in mip.isolate(&selector.layers, TextureUses::UNKNOWN) {
                    *state = desired_state;
                }
            }
        }
        complex
    }

    /// Convert a complex state into an iterator over all states stored.
    ///
    /// [`Self::from_selector_state_iter`] can be used to consume such an iterator.
    fn to_selector_state_iter(
        &self,
    ) -> impl Iterator<Item = (TextureSelector, TextureUses)> + Clone + '_ {
        self.mips.iter().enumerate().flat_map(|(mip, inner)| {
            let mip = mip as u32;
            {
                inner.iter().map(move |&(ref layers, inner)| {
                    (
                        TextureSelector {
                            mips: mip..mip + 1,
                            layers: layers.clone(),
                        },
                        inner,
                    )
                })
            }
        })
    }
}

/// Stores a bind group's texture views + their usages (within the bind group).
#[derive(Debug)]
pub(crate) struct TextureViewBindGroupState {
    views: Vec<(Arc<TextureView>, TextureUses)>,
}
impl TextureViewBindGroupState {
    pub fn new() -> Self {
        Self { views: Vec::new() }
    }

    /// Optimize the texture bind group state by sorting it by ID.
    ///
    /// When this list of states is merged into a tracker, the memory
    /// accesses will be in a constant ascending order.
    pub(crate) fn optimize(&mut self) {
        self.views
            .sort_unstable_by_key(|(view, _)| view.parent.tracker_index());
    }

    /// Adds the given resource with the given state.
    pub fn insert_single(&mut self, view: Arc<TextureView>, usage: TextureUses) {
        self.views.push((view, usage));
    }
}

/// Container for corresponding simple and complex texture states.
#[derive(Debug)]
pub(crate) struct TextureStateSet {
    simple: Vec<TextureUses>,
    complex: FastHashMap<usize, ComplexTextureState>,
}

impl TextureStateSet {
    fn new() -> Self {
        Self {
            simple: Vec::new(),
            complex: FastHashMap::default(),
        }
    }

    fn clear(&mut self) {
        self.simple.clear();
        self.complex.clear();
    }

    fn set_size(&mut self, size: usize) {
        self.simple.resize(size, TextureUses::UNINITIALIZED);
    }

    fn size(&self) -> usize {
        self.simple.len()
    }

    /// SAFETY: `index` must be in bounds.
    unsafe fn get_unchecked(
        &self,
        index: usize,
    ) -> SingleOrManyStates<TextureUses, &ComplexTextureState> {
        let simple = unsafe { *self.simple.get_unchecked(index) };
        if simple == TextureUses::COMPLEX {
            SingleOrManyStates::Many(unsafe { self.complex.get(&index).unwrap_unchecked() })
        } else {
            SingleOrManyStates::Single(simple)
        }
    }

    /// # Safety
    ///
    /// The `index` must be in bounds.
    unsafe fn get_mut_unchecked(
        &mut self,
        index: usize,
    ) -> SingleOrManyStates<&mut TextureUses, &mut ComplexTextureState> {
        let simple = unsafe { self.simple.get_unchecked_mut(index) };
        if *simple == TextureUses::COMPLEX {
            SingleOrManyStates::Many(unsafe { self.complex.get_mut(&index).unwrap_unchecked() })
        } else {
            SingleOrManyStates::Single(simple)
        }
    }

    /// # Safety
    ///
    /// The `index` must be in bounds.
    unsafe fn insert_simple_unchecked(&mut self, index: usize, simple: TextureUses) {
        unsafe { *self.simple.get_unchecked_mut(index) = simple };
    }

    /// # Safety
    ///
    /// The `index` must be in bounds.
    unsafe fn insert_complex_unchecked(&mut self, index: usize, complex: ComplexTextureState) {
        unsafe { *self.simple.get_unchecked_mut(index) = TextureUses::COMPLEX };
        self.complex.insert(index, complex);
    }

    /// # Safety
    ///
    /// The `index` must be in bounds.
    unsafe fn make_simple_unchecked(&mut self, index: usize, simple: TextureUses) {
        unsafe { *self.simple.get_unchecked_mut(index) = simple };
        unsafe { self.complex.remove(&index).unwrap_unchecked() };
    }

    /// # Safety
    ///
    /// The `index` must be in bounds.
    unsafe fn make_complex_unchecked(&mut self, index: usize, complex: ComplexTextureState) {
        unsafe { *self.simple.get_unchecked_mut(index) = TextureUses::COMPLEX };
        self.complex.insert(index, complex);
    }

    fn tracker_assert_in_bounds(&self, index: usize) {
        strict_assert!(index < self.size());
    }
}

/// Stores all texture state within a single usage scope.
#[derive(Debug)]
pub(crate) struct TextureUsageScope {
    set: TextureStateSet,
    metadata: ResourceMetadata<Arc<Texture>>,
    ordered_uses_mask: TextureUses,
}

impl Default for TextureUsageScope {
    fn default() -> Self {
        Self {
            set: TextureStateSet::new(),
            metadata: ResourceMetadata::new(),
            ordered_uses_mask: TextureUses::empty(),
        }
    }
}

impl TextureUsageScope {
    fn tracker_assert_in_bounds(&self, index: usize) {
        self.metadata.tracker_assert_in_bounds(index);
        self.set.tracker_assert_in_bounds(index);
    }

    pub fn clear(&mut self) {
        self.set.clear();
        self.metadata.clear();
    }

    /// Sets the size of all the vectors inside the tracker.
    ///
    /// Must be called with the highest possible Texture ID before
    /// all unsafe functions are called.
    pub fn set_size(&mut self, size: usize) {
        self.set.set_size(size);
        self.metadata.set_size(size);
    }

    pub fn set_ordered_uses_mask(&mut self, ordered_uses_mask: TextureUses) {
        self.ordered_uses_mask = ordered_uses_mask;
    }

    /// Returns true if the tracker owns no resources.
    ///
    /// This is a O(n) operation.
    pub(crate) fn is_empty(&self) -> bool {
        self.metadata.is_empty()
    }

    /// Merge the list of texture states in the given usage scope into this UsageScope.
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
        let incoming_size = scope.set.size();
        if incoming_size > self.set.size() {
            self.set_size(incoming_size);
        }

        for index in scope.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);
            scope.tracker_assert_in_bounds(index);

            let texture_selector =
                unsafe { &scope.metadata.get_resource_unchecked(index).full_range };
            unsafe {
                insert_or_merge(
                    texture_selector,
                    &mut self.set,
                    &mut self.metadata,
                    index,
                    TextureStateProvider::TextureSet { set: &scope.set },
                    ResourceMetadataProvider::Indirect {
                        metadata: &scope.metadata,
                    },
                )?
            };
        }

        Ok(())
    }

    /// Merge the list of texture states in the given bind group into this usage scope.
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
        bind_group: &TextureViewBindGroupState,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        for (view, usage) in bind_group.views.iter() {
            unsafe { self.merge_single(&view.parent, Some(view.selector.clone()), *usage)? };
        }

        Ok(())
    }

    /// Merge a single state into the UsageScope.
    ///
    /// If the resulting state is invalid, returns a usage
    /// conflict with the details of the invalid state.
    ///
    /// # Safety
    ///
    /// Unlike other trackers whose merge_single is safe, this method is only
    /// called where there is already other unsafe tracking functions active,
    /// so we can prove this unsafe "for free".
    ///
    /// [`Self::set_size`] must be called with the maximum possible Buffer ID before this
    /// method is called.
    pub unsafe fn merge_single(
        &mut self,
        texture: &Arc<Texture>,
        selector: Option<TextureSelector>,
        new_state: TextureUses,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        let index = texture.tracker_index().as_usize();

        self.tracker_assert_in_bounds(index);

        let texture_selector = &texture.full_range;
        unsafe {
            insert_or_merge(
                texture_selector,
                &mut self.set,
                &mut self.metadata,
                index,
                TextureStateProvider::from_option(selector, new_state),
                ResourceMetadataProvider::Direct { resource: texture },
            )?
        };

        Ok(())
    }
}

pub(crate) trait TextureTrackerSetSingle {
    fn set_single(
        &mut self,
        texture: &Arc<Texture>,
        selector: TextureSelector,
        new_state: TextureUses,
    ) -> Drain<'_, PendingTransition<TextureUses>>;
}

/// Stores all texture state within a command buffer.
pub(crate) struct TextureTracker {
    start_set: TextureStateSet,
    end_set: TextureStateSet,

    metadata: ResourceMetadata<Arc<Texture>>,

    temp: Vec<PendingTransition<TextureUses>>,

    ordered_uses_mask: TextureUses,
}

impl TextureTracker {
    pub fn new(ordered_uses_mask: TextureUses) -> Self {
        Self {
            start_set: TextureStateSet::new(),
            end_set: TextureStateSet::new(),

            metadata: ResourceMetadata::new(),

            temp: Vec::new(),

            ordered_uses_mask,
        }
    }

    fn tracker_assert_in_bounds(&self, index: usize) {
        self.metadata.tracker_assert_in_bounds(index);
        self.start_set.tracker_assert_in_bounds(index);
        self.end_set.tracker_assert_in_bounds(index);
    }

    /// Sets the size of all the vectors inside the tracker.
    ///
    /// Must be called with the highest possible Texture ID before
    /// all unsafe functions are called.
    pub fn set_size(&mut self, size: usize) {
        self.start_set.set_size(size);
        self.end_set.set_size(size);

        self.metadata.set_size(size);
    }

    /// Extend the vectors to let the given index be valid.
    fn allow_index(&mut self, index: usize) {
        if index >= self.start_set.size() {
            self.set_size(index + 1);
        }
    }

    /// Returns true if the tracker owns the given texture.
    pub fn contains(&self, texture: &Texture) -> bool {
        self.metadata.contains(texture.tracker_index().as_usize())
    }

    /// Returns a list of all textures tracked.
    pub fn used_resources(&self) -> impl Iterator<Item = &Arc<Texture>> + '_ {
        self.metadata.owned_resources()
    }
    /// Drain all currently pending transitions.
    pub fn drain_transitions<'a>(
        &'a mut self,
        snatch_guard: &'a SnatchGuard<'a>,
    ) -> (PendingTransitionList, Vec<Option<&'a TextureInner>>) {
        let mut textures = Vec::new();
        let transitions = self
            .temp
            .drain(..)
            .inspect(|pending| {
                let tex = unsafe { self.metadata.get_resource_unchecked(pending.id as _) };
                textures.push(tex.inner.get(snatch_guard));
            })
            .collect();
        (transitions, textures)
    }

    /// Sets the state of a single texture.
    ///
    /// If a transition is needed to get the texture into the given state, that transition
    /// is returned.
    ///
    /// If the ID is higher than the length of internal vectors,
    /// the vectors will be extended. A call to set_size is not needed.
    pub fn set_single(
        &mut self,
        texture: &Arc<Texture>,
        selector: TextureSelector,
        new_state: TextureUses,
    ) -> Drain<'_, PendingTransition<TextureUses>> {
        let index = texture.tracker_index().as_usize();

        self.allow_index(index);

        self.tracker_assert_in_bounds(index);

        unsafe {
            insert_or_barrier_update(
                &texture.full_range,
                Some(&mut self.start_set),
                &mut self.end_set,
                &mut self.metadata,
                index,
                TextureStateProvider::Selector {
                    selector,
                    state: new_state,
                },
                None,
                ResourceMetadataProvider::Direct { resource: texture },
                &mut self.temp,
                self.ordered_uses_mask,
            )
        }

        self.temp.drain(..)
    }

    /// Sets the given state for all texture in the given tracker.
    ///
    /// If a transition is needed to get the texture into the needed state,
    /// those transitions are stored within the tracker. A subsequent
    /// call to [`Self::drain_transitions`] is needed to get those transitions.
    ///
    /// If the ID is higher than the length of internal vectors,
    /// the vectors will be extended. A call to set_size is not needed.
    pub fn set_from_tracker(&mut self, tracker: &Self) {
        let incoming_size = tracker.start_set.size();
        if incoming_size > self.start_set.size() {
            self.set_size(incoming_size);
        }

        for index in tracker.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);
            tracker.tracker_assert_in_bounds(index);
            unsafe {
                let texture_selector = &tracker.metadata.get_resource_unchecked(index).full_range;
                insert_or_barrier_update(
                    texture_selector,
                    Some(&mut self.start_set),
                    &mut self.end_set,
                    &mut self.metadata,
                    index,
                    TextureStateProvider::TextureSet {
                        set: &tracker.start_set,
                    },
                    Some(TextureStateProvider::TextureSet {
                        set: &tracker.end_set,
                    }),
                    ResourceMetadataProvider::Indirect {
                        metadata: &tracker.metadata,
                    },
                    &mut self.temp,
                    self.ordered_uses_mask,
                );
            }
        }
    }

    /// Sets the given state for all textures in the given UsageScope.
    ///
    /// If a transition is needed to get the textures into the needed state,
    /// those transitions are stored within the tracker. A subsequent
    /// call to [`Self::drain_transitions`] is needed to get those transitions.
    ///
    /// If the ID is higher than the length of internal vectors,
    /// the vectors will be extended. A call to set_size is not needed.
    pub fn set_from_usage_scope(&mut self, scope: &TextureUsageScope) {
        let incoming_size = scope.set.size();
        if incoming_size > self.start_set.size() {
            self.set_size(incoming_size);
        }

        for index in scope.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);
            scope.tracker_assert_in_bounds(index);
            unsafe {
                let texture_selector = &scope.metadata.get_resource_unchecked(index).full_range;
                insert_or_barrier_update(
                    texture_selector,
                    Some(&mut self.start_set),
                    &mut self.end_set,
                    &mut self.metadata,
                    index,
                    TextureStateProvider::TextureSet { set: &scope.set },
                    None,
                    ResourceMetadataProvider::Indirect {
                        metadata: &scope.metadata,
                    },
                    &mut self.temp,
                    self.ordered_uses_mask,
                );
            }
        }
    }

    /// Iterates through all textures in the given bind group and adopts
    /// the state given for those textures in the UsageScope. It also
    /// removes all touched textures from the usage scope.
    ///
    /// If a transition is needed to get the textures into the needed state,
    /// those transitions are stored within the tracker. A subsequent
    /// call to [`Self::drain_transitions`] is needed to get those transitions.
    ///
    /// This is a really funky method used by Compute Passes to generate
    /// barriers after a call to dispatch without needing to iterate
    /// over all elements in the usage scope. We use each the
    /// bind group as a source of which IDs to look at. The bind groups
    /// must have first been added to the usage scope.
    ///
    /// # Panics
    ///
    /// If a resource in `bind_group_state` is not found in the usage scope.
    pub fn set_and_remove_from_usage_scope_sparse(
        &mut self,
        scope: &mut TextureUsageScope,
        bind_group_state: &TextureViewBindGroupState,
    ) {
        let incoming_size = scope.set.size();
        if incoming_size > self.start_set.size() {
            self.set_size(incoming_size);
        }

        for (view, _) in bind_group_state.views.iter() {
            let index = view.parent.tracker_index().as_usize();
            scope.tracker_assert_in_bounds(index);

            if unsafe { !scope.metadata.contains_unchecked(index) } {
                continue;
            }
            let texture_selector = &view.parent.full_range;
            // SAFETY: we checked that the index is in bounds for the scope, and
            // called `set_size` to ensure it is valid for `self`.
            unsafe {
                insert_or_barrier_update(
                    texture_selector,
                    Some(&mut self.start_set),
                    &mut self.end_set,
                    &mut self.metadata,
                    index,
                    TextureStateProvider::TextureSet { set: &scope.set },
                    None,
                    ResourceMetadataProvider::Indirect {
                        metadata: &scope.metadata,
                    },
                    &mut self.temp,
                    self.ordered_uses_mask,
                )
            };

            unsafe { scope.metadata.remove(index) };
        }
    }
}

impl TextureTrackerSetSingle for TextureTracker {
    fn set_single(
        &mut self,
        texture: &Arc<Texture>,
        selector: TextureSelector,
        new_state: TextureUses,
    ) -> Drain<'_, PendingTransition<TextureUses>> {
        self.set_single(texture, selector, new_state)
    }
}

/// Stores all texture state within a device.
pub(crate) struct DeviceTextureTracker {
    current_state_set: TextureStateSet,
    metadata: ResourceMetadata<Weak<Texture>>,
    temp: Vec<PendingTransition<TextureUses>>,
    ordered_uses_mask: TextureUses,
}

impl DeviceTextureTracker {
    pub fn new(ordered_uses_mask: TextureUses) -> Self {
        Self {
            current_state_set: TextureStateSet::new(),
            metadata: ResourceMetadata::new(),
            temp: Vec::new(),
            ordered_uses_mask,
        }
    }

    fn tracker_assert_in_bounds(&self, index: usize) {
        self.metadata.tracker_assert_in_bounds(index);
        self.current_state_set.tracker_assert_in_bounds(index);
    }

    /// Extend the vectors to let the given index be valid.
    fn allow_index(&mut self, index: usize) {
        if index >= self.current_state_set.size() {
            self.current_state_set.set_size(index + 1);
            self.metadata.set_size(index + 1);
        }
    }

    /// Returns a list of all textures tracked.
    pub fn used_resources(&self) -> impl Iterator<Item = &Weak<Texture>> + '_ {
        self.metadata.owned_resources()
    }

    /// Inserts a single texture and a state into the resource tracker.
    ///
    /// If the resource already exists in the tracker, it will be overwritten.
    pub fn insert_single(&mut self, texture: &Arc<Texture>, state: TextureUses) {
        let index = texture.tracker_index().as_usize();

        self.allow_index(index);

        self.tracker_assert_in_bounds(index);

        unsafe {
            insert(
                None,
                None,
                &mut self.current_state_set,
                &mut self.metadata,
                index,
                TextureStateProvider::KnownSingle { state },
                None,
                ResourceMetadataProvider::Direct {
                    resource: &Arc::downgrade(texture),
                },
            )
        };
    }

    /// Sets the state of a single texture.
    ///
    /// If a transition is needed to get the texture into the given state, that transition
    /// is returned.
    pub fn set_single(
        &mut self,
        texture: &Arc<Texture>,
        selector: TextureSelector,
        new_state: TextureUses,
    ) -> Drain<'_, PendingTransition<TextureUses>> {
        let index = texture.tracker_index().as_usize();

        self.allow_index(index);

        self.tracker_assert_in_bounds(index);

        let start_state_provider = TextureStateProvider::Selector {
            selector,
            state: new_state,
        };
        unsafe {
            barrier(
                &texture.full_range,
                &self.current_state_set,
                index,
                start_state_provider.clone(),
                &mut self.temp,
                self.ordered_uses_mask,
            )
        };
        unsafe {
            update(
                &texture.full_range,
                None,
                &mut self.current_state_set,
                index,
                start_state_provider,
            )
        };

        self.temp.drain(..)
    }

    /// Sets the given state for all texture in the given tracker.
    ///
    /// If a transition is needed to get the texture into the needed state,
    /// those transitions are returned.
    pub fn set_from_tracker_and_drain_transitions<'a, 'b: 'a>(
        &'a mut self,
        tracker: &'a TextureTracker,
        snatch_guard: &'b SnatchGuard<'b>,
    ) -> impl Iterator<Item = TextureBarrier<'a, dyn hal::DynTexture>> {
        for index in tracker.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);

            let start_state_provider = TextureStateProvider::TextureSet {
                set: &tracker.start_set,
            };
            let end_state_provider = TextureStateProvider::TextureSet {
                set: &tracker.end_set,
            };
            unsafe {
                let texture_selector = &tracker.metadata.get_resource_unchecked(index).full_range;
                barrier(
                    texture_selector,
                    &self.current_state_set,
                    index,
                    start_state_provider,
                    &mut self.temp,
                    self.ordered_uses_mask,
                );
                update(
                    texture_selector,
                    None,
                    &mut self.current_state_set,
                    index,
                    end_state_provider,
                );
            }
        }

        self.temp.drain(..).map(|pending| {
            let tex = unsafe { tracker.metadata.get_resource_unchecked(pending.id as _) };
            let tex = tex.try_raw(snatch_guard).unwrap();
            pending.into_hal(tex)
        })
    }

    /// Sets the given state for all textures in the given UsageScope.
    ///
    /// If a transition is needed to get the textures into the needed state,
    /// those transitions are returned.
    pub fn set_from_usage_scope_and_drain_transitions<'a, 'b: 'a>(
        &'a mut self,
        scope: &'a TextureUsageScope,
        snatch_guard: &'b SnatchGuard<'b>,
    ) -> impl Iterator<Item = TextureBarrier<'a, dyn hal::DynTexture>> {
        for index in scope.metadata.owned_indices() {
            self.tracker_assert_in_bounds(index);

            let start_state_provider = TextureStateProvider::TextureSet { set: &scope.set };
            unsafe {
                let texture_selector = &scope.metadata.get_resource_unchecked(index).full_range;
                barrier(
                    texture_selector,
                    &self.current_state_set,
                    index,
                    start_state_provider.clone(),
                    &mut self.temp,
                    self.ordered_uses_mask,
                );
                update(
                    texture_selector,
                    None,
                    &mut self.current_state_set,
                    index,
                    start_state_provider,
                );
            }
        }

        self.temp.drain(..).map(|pending| {
            let tex = unsafe { scope.metadata.get_resource_unchecked(pending.id as _) };
            let tex = tex.try_raw(snatch_guard).unwrap();
            pending.into_hal(tex)
        })
    }
}

impl TextureTrackerSetSingle for DeviceTextureTracker {
    fn set_single(
        &mut self,
        texture: &Arc<Texture>,
        selector: TextureSelector,
        new_state: TextureUses,
    ) -> Drain<'_, PendingTransition<TextureUses>> {
        self.set_single(texture, selector, new_state)
    }
}

/// An iterator adapter that can store two different iterator types.
#[derive(Clone)]
enum EitherIter<L, R> {
    Left(L),
    Right(R),
}

impl<L, R, D> Iterator for EitherIter<L, R>
where
    L: Iterator<Item = D>,
    R: Iterator<Item = D>,
{
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            EitherIter::Left(ref mut inner) => inner.next(),
            EitherIter::Right(ref mut inner) => inner.next(),
        }
    }
}

/// Container that signifies storing both different things
/// if there is a single state or many different states
/// involved in the operation.
#[derive(Debug, Clone)]
enum SingleOrManyStates<S, M> {
    Single(S),
    Many(M),
}

/// A source of texture state.
#[derive(Clone)]
enum TextureStateProvider<'a> {
    /// Comes directly from a single state.
    KnownSingle { state: TextureUses },
    /// Comes from a selector and a single state.
    Selector {
        selector: TextureSelector,
        state: TextureUses,
    },
    /// Comes from another texture set.
    TextureSet { set: &'a TextureStateSet },
}
impl<'a> TextureStateProvider<'a> {
    /// Convenience function turning `Option<Selector>` into this enum.
    fn from_option(selector: Option<TextureSelector>, state: TextureUses) -> Self {
        match selector {
            Some(selector) => Self::Selector { selector, state },
            None => Self::KnownSingle { state },
        }
    }

    /// Get the state provided by this.
    ///
    /// # Panics
    ///
    /// Panics if texture_selector is None and this uses a Selector source.
    ///
    /// # Safety
    ///
    /// - The index must be in bounds of the state set if this uses an TextureSet source.
    #[inline(always)]
    unsafe fn get_state(
        self,
        texture_selector: Option<&TextureSelector>,
        index: usize,
    ) -> SingleOrManyStates<
        TextureUses,
        impl Iterator<Item = (TextureSelector, TextureUses)> + Clone + 'a,
    > {
        match self {
            TextureStateProvider::KnownSingle { state } => SingleOrManyStates::Single(state),
            TextureStateProvider::Selector { selector, state } => {
                // We check if the selector given is actually for the full resource,
                // and if it is we promote to a simple state. This allows upstream
                // code to specify selectors willy nilly, and all that are really
                // single states are promoted here.
                if *texture_selector.unwrap() == selector {
                    SingleOrManyStates::Single(state)
                } else {
                    SingleOrManyStates::Many(EitherIter::Left(iter::once((selector, state))))
                }
            }
            TextureStateProvider::TextureSet { set } => match unsafe { set.get_unchecked(index) } {
                SingleOrManyStates::Single(single) => SingleOrManyStates::Single(single),
                SingleOrManyStates::Many(complex) => {
                    SingleOrManyStates::Many(EitherIter::Right(complex.to_selector_state_iter()))
                }
            },
        }
    }
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
    texture_selector: &TextureSelector,
    current_state_set: &mut TextureStateSet,
    resource_metadata: &mut ResourceMetadata<Arc<Texture>>,
    index: usize,
    state_provider: TextureStateProvider<'_>,
    metadata_provider: ResourceMetadataProvider<'_, Arc<Texture>>,
) -> Result<(), ResourceUsageCompatibilityError> {
    let currently_owned = unsafe { resource_metadata.contains_unchecked(index) };

    if !currently_owned {
        unsafe {
            insert(
                Some(texture_selector),
                None,
                current_state_set,
                resource_metadata,
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
            texture_selector,
            current_state_set,
            index,
            state_provider,
            metadata_provider,
        )
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
    texture_selector: &TextureSelector,
    start_state: Option<&mut TextureStateSet>,
    current_state_set: &mut TextureStateSet,
    resource_metadata: &mut ResourceMetadata<Arc<Texture>>,
    index: usize,
    start_state_provider: TextureStateProvider<'_>,
    end_state_provider: Option<TextureStateProvider<'_>>,
    metadata_provider: ResourceMetadataProvider<'_, Arc<Texture>>,
    barriers: &mut Vec<PendingTransition<TextureUses>>,
    ordered_uses_mask: TextureUses,
) {
    let currently_owned = unsafe { resource_metadata.contains_unchecked(index) };

    if !currently_owned {
        unsafe {
            insert(
                Some(texture_selector),
                start_state,
                current_state_set,
                resource_metadata,
                index,
                start_state_provider,
                end_state_provider,
                metadata_provider,
            )
        };
        return;
    }

    let update_state_provider = end_state_provider.unwrap_or_else(|| start_state_provider.clone());
    unsafe {
        barrier(
            texture_selector,
            current_state_set,
            index,
            start_state_provider,
            barriers,
            ordered_uses_mask,
        )
    };
    unsafe {
        update(
            texture_selector,
            start_state,
            current_state_set,
            index,
            update_state_provider,
        )
    };
}

#[inline(always)]
unsafe fn insert<T: Clone>(
    texture_selector: Option<&TextureSelector>,
    start_state: Option<&mut TextureStateSet>,
    end_state: &mut TextureStateSet,
    resource_metadata: &mut ResourceMetadata<T>,
    index: usize,
    start_state_provider: TextureStateProvider<'_>,
    end_state_provider: Option<TextureStateProvider<'_>>,
    metadata_provider: ResourceMetadataProvider<'_, T>,
) {
    let start_layers = unsafe { start_state_provider.get_state(texture_selector, index) };
    match start_layers {
        SingleOrManyStates::Single(state) => {
            // This should only ever happen with a wgpu bug, but let's just double
            // check that resource states don't have any conflicts.
            strict_assert_eq!(invalid_resource_state(state), false);

            if let Some(start_state) = start_state {
                unsafe { start_state.insert_simple_unchecked(index, state) };
            }

            // We only need to insert ourselves the end state if there is no end state provider.
            if end_state_provider.is_none() {
                unsafe { end_state.insert_simple_unchecked(index, state) };
            }
        }
        SingleOrManyStates::Many(state_iter) => {
            let full_range = texture_selector.unwrap().clone();

            let complex =
                unsafe { ComplexTextureState::from_selector_state_iter(full_range, state_iter) };

            if let Some(start_state) = start_state {
                unsafe { start_state.insert_complex_unchecked(index, complex.clone()) };
            }

            // We only need to insert ourselves the end state if there is no end state provider.
            if end_state_provider.is_none() {
                unsafe { end_state.insert_complex_unchecked(index, complex) };
            }
        }
    }

    if let Some(end_state_provider) = end_state_provider {
        match unsafe { end_state_provider.get_state(texture_selector, index) } {
            SingleOrManyStates::Single(state) => {
                // This should only ever happen with a wgpu bug, but let's just double
                // check that resource states don't have any conflicts.
                strict_assert_eq!(invalid_resource_state(state), false);

                // We only need to insert into the end, as there is guaranteed to be
                // a start state provider.
                unsafe { end_state.insert_simple_unchecked(index, state) };
            }
            SingleOrManyStates::Many(state_iter) => {
                let full_range = texture_selector.unwrap().clone();

                let complex = unsafe {
                    ComplexTextureState::from_selector_state_iter(full_range, state_iter)
                };

                // We only need to insert into the end, as there is guaranteed to be
                // a start state provider.
                unsafe { end_state.insert_complex_unchecked(index, complex) };
            }
        }
    }

    unsafe {
        let resource = metadata_provider.get(index);
        resource_metadata.insert(index, resource.clone());
    }
}

#[inline(always)]
unsafe fn merge(
    texture_selector: &TextureSelector,
    current_state_set: &mut TextureStateSet,
    index: usize,
    state_provider: TextureStateProvider<'_>,
    metadata_provider: ResourceMetadataProvider<'_, Arc<Texture>>,
) -> Result<(), ResourceUsageCompatibilityError> {
    let current_state = unsafe { current_state_set.get_mut_unchecked(index) };

    let new_state = unsafe { state_provider.get_state(Some(texture_selector), index) };

    match (current_state, new_state) {
        (SingleOrManyStates::Single(current_simple), SingleOrManyStates::Single(new_simple)) => {
            let merged_state = *current_simple | new_simple;

            if invalid_resource_state(merged_state) {
                return Err(ResourceUsageCompatibilityError::from_texture(
                    unsafe { metadata_provider.get(index) },
                    texture_selector.clone(),
                    *current_simple,
                    new_simple,
                ));
            }

            *current_simple = merged_state;
        }
        (SingleOrManyStates::Single(current_simple), SingleOrManyStates::Many(new_many)) => {
            // Because we are now demoting this simple state to a complex state,
            // we actually need to make a whole new complex state for us to use
            // as there wasn't one before.
            let mut new_complex = unsafe {
                ComplexTextureState::from_selector_state_iter(
                    texture_selector.clone(),
                    iter::once((texture_selector.clone(), *current_simple)),
                )
            };

            for (selector, new_state) in new_many {
                let merged_state = *current_simple | new_state;

                if invalid_resource_state(merged_state) {
                    return Err(ResourceUsageCompatibilityError::from_texture(
                        unsafe { metadata_provider.get(index) },
                        selector,
                        *current_simple,
                        new_state,
                    ));
                }

                for mip in
                    &mut new_complex.mips[selector.mips.start as usize..selector.mips.end as usize]
                {
                    for &mut (_, ref mut current_layer_state) in
                        mip.isolate(&selector.layers, TextureUses::UNKNOWN)
                    {
                        *current_layer_state = merged_state;
                    }

                    mip.coalesce();
                }
            }

            unsafe { current_state_set.make_complex_unchecked(index, new_complex) };
        }
        (SingleOrManyStates::Many(current_complex), SingleOrManyStates::Single(new_simple)) => {
            for (mip_id, mip) in current_complex.mips.iter_mut().enumerate() {
                let mip_id = mip_id as u32;

                for &mut (ref layers, ref mut current_layer_state) in mip.iter_mut() {
                    let merged_state = *current_layer_state | new_simple;

                    // Once we remove unknown, this will never be empty, as
                    // simple states are never unknown.
                    let merged_state = merged_state - TextureUses::UNKNOWN;

                    if invalid_resource_state(merged_state) {
                        return Err(ResourceUsageCompatibilityError::from_texture(
                            unsafe { metadata_provider.get(index) },
                            TextureSelector {
                                mips: mip_id..mip_id + 1,
                                layers: layers.clone(),
                            },
                            *current_layer_state,
                            new_simple,
                        ));
                    }

                    *current_layer_state = merged_state;
                }

                mip.coalesce();
            }
        }
        (SingleOrManyStates::Many(current_complex), SingleOrManyStates::Many(new_many)) => {
            for (selector, new_state) in new_many {
                for mip_id in selector.mips {
                    strict_assert!((mip_id as usize) < current_complex.mips.len());

                    let mip = unsafe { current_complex.mips.get_unchecked_mut(mip_id as usize) };

                    for &mut (ref layers, ref mut current_layer_state) in
                        mip.isolate(&selector.layers, TextureUses::UNKNOWN)
                    {
                        let merged_state = *current_layer_state | new_state;
                        let merged_state = merged_state - TextureUses::UNKNOWN;

                        if merged_state.is_empty() {
                            // We know nothing about this state, lets just move on.
                            continue;
                        }

                        if invalid_resource_state(merged_state) {
                            return Err(ResourceUsageCompatibilityError::from_texture(
                                unsafe { metadata_provider.get(index) },
                                TextureSelector {
                                    mips: mip_id..mip_id + 1,
                                    layers: layers.clone(),
                                },
                                *current_layer_state,
                                new_state,
                            ));
                        }
                        *current_layer_state = merged_state;
                    }

                    mip.coalesce();
                }
            }
        }
    }
    Ok(())
}

#[inline(always)]
unsafe fn barrier(
    texture_selector: &TextureSelector,
    current_state_set: &TextureStateSet,
    index: usize,
    state_provider: TextureStateProvider<'_>,
    barriers: &mut Vec<PendingTransition<TextureUses>>,
    ordered_uses_mask: TextureUses,
) {
    let current_state = unsafe { current_state_set.get_unchecked(index) };

    let new_state = unsafe { state_provider.get_state(Some(texture_selector), index) };

    match (current_state, new_state) {
        (SingleOrManyStates::Single(current_simple), SingleOrManyStates::Single(new_simple)) => {
            if skip_barrier(current_simple, ordered_uses_mask, new_simple) {
                return;
            }

            barriers.push(PendingTransition {
                id: index as _,
                selector: texture_selector.clone(),
                usage: hal::StateTransition {
                    from: current_simple,
                    to: new_simple,
                },
            });
        }
        (SingleOrManyStates::Single(current_simple), SingleOrManyStates::Many(new_many)) => {
            for (selector, new_state) in new_many {
                if new_state == TextureUses::UNKNOWN {
                    continue;
                }

                if skip_barrier(current_simple, ordered_uses_mask, new_state) {
                    continue;
                }

                barriers.push(PendingTransition {
                    id: index as _,
                    selector,
                    usage: hal::StateTransition {
                        from: current_simple,
                        to: new_state,
                    },
                });
            }
        }
        (SingleOrManyStates::Many(current_complex), SingleOrManyStates::Single(new_simple)) => {
            for (mip_id, mip) in current_complex.mips.iter().enumerate() {
                let mip_id = mip_id as u32;

                for &(ref layers, current_layer_state) in mip.iter() {
                    if current_layer_state == TextureUses::UNKNOWN {
                        continue;
                    }

                    if skip_barrier(current_layer_state, ordered_uses_mask, new_simple) {
                        continue;
                    }

                    barriers.push(PendingTransition {
                        id: index as _,
                        selector: TextureSelector {
                            mips: mip_id..mip_id + 1,
                            layers: layers.clone(),
                        },
                        usage: hal::StateTransition {
                            from: current_layer_state,
                            to: new_simple,
                        },
                    });
                }
            }
        }
        (SingleOrManyStates::Many(current_complex), SingleOrManyStates::Many(new_many)) => {
            for (selector, new_state) in new_many {
                for mip_id in selector.mips {
                    strict_assert!((mip_id as usize) < current_complex.mips.len());

                    let mip = unsafe { current_complex.mips.get_unchecked(mip_id as usize) };

                    for (layers, current_layer_state) in mip.iter_filter(&selector.layers) {
                        if *current_layer_state == TextureUses::UNKNOWN
                            || new_state == TextureUses::UNKNOWN
                        {
                            continue;
                        }

                        if skip_barrier(*current_layer_state, ordered_uses_mask, new_state) {
                            continue;
                        }

                        barriers.push(PendingTransition {
                            id: index as _,
                            selector: TextureSelector {
                                mips: mip_id..mip_id + 1,
                                layers,
                            },
                            usage: hal::StateTransition {
                                from: *current_layer_state,
                                to: new_state,
                            },
                        });
                    }
                }
            }
        }
    }
}

#[inline(always)]
unsafe fn update(
    texture_selector: &TextureSelector,
    start_state_set: Option<&mut TextureStateSet>,
    current_state_set: &mut TextureStateSet,
    index: usize,
    state_provider: TextureStateProvider<'_>,
) {
    // We only ever need to update the start state here if the state is complex.
    //
    // If the state is simple, the first insert to the tracker would cover it.
    let mut start_complex = start_state_set.and_then(|start_state_set| {
        match unsafe { start_state_set.get_mut_unchecked(index) } {
            SingleOrManyStates::Single(_) => None,
            SingleOrManyStates::Many(complex) => Some(complex),
        }
    });

    let current_state = unsafe { current_state_set.get_mut_unchecked(index) };

    let new_state = unsafe { state_provider.get_state(Some(texture_selector), index) };

    match (current_state, new_state) {
        (SingleOrManyStates::Single(current_simple), SingleOrManyStates::Single(new_simple)) => {
            *current_simple = new_simple;
        }
        (SingleOrManyStates::Single(current_simple), SingleOrManyStates::Many(new_many)) => {
            // Because we are now demoting this simple state to a complex state,
            // we actually need to make a whole new complex state for us to use
            // as there wasn't one before.
            let mut new_complex = unsafe {
                ComplexTextureState::from_selector_state_iter(
                    texture_selector.clone(),
                    iter::once((texture_selector.clone(), *current_simple)),
                )
            };

            for (selector, mut new_state) in new_many {
                if new_state == TextureUses::UNKNOWN {
                    new_state = *current_simple;
                }
                for mip in
                    &mut new_complex.mips[selector.mips.start as usize..selector.mips.end as usize]
                {
                    for &mut (_, ref mut current_layer_state) in
                        mip.isolate(&selector.layers, TextureUses::UNKNOWN)
                    {
                        *current_layer_state = new_state;
                    }

                    mip.coalesce();
                }
            }

            unsafe { current_state_set.make_complex_unchecked(index, new_complex) };
        }
        (SingleOrManyStates::Many(current_complex), SingleOrManyStates::Single(new_single)) => {
            for (mip_id, mip) in current_complex.mips.iter().enumerate() {
                for &(ref layers, current_layer_state) in mip.iter() {
                    // If this state is unknown, that means that the start is _also_ unknown.
                    if current_layer_state == TextureUses::UNKNOWN {
                        if let Some(&mut ref mut start_complex) = start_complex {
                            strict_assert!(mip_id < start_complex.mips.len());

                            let start_mip = unsafe { start_complex.mips.get_unchecked_mut(mip_id) };

                            for &mut (_, ref mut current_start_state) in
                                start_mip.isolate(layers, TextureUses::UNKNOWN)
                            {
                                strict_assert_eq!(*current_start_state, TextureUses::UNKNOWN);
                                *current_start_state = new_single;
                            }

                            start_mip.coalesce();
                        }
                    }
                }
            }

            unsafe { current_state_set.make_simple_unchecked(index, new_single) };
        }
        (SingleOrManyStates::Many(current_complex), SingleOrManyStates::Many(new_many)) => {
            for (selector, new_state) in new_many {
                if new_state == TextureUses::UNKNOWN {
                    // We know nothing new
                    continue;
                }

                for mip_id in selector.mips {
                    let mip_id = mip_id as usize;
                    strict_assert!(mip_id < current_complex.mips.len());

                    let mip = unsafe { current_complex.mips.get_unchecked_mut(mip_id) };

                    for &mut (ref layers, ref mut current_layer_state) in
                        mip.isolate(&selector.layers, TextureUses::UNKNOWN)
                    {
                        if *current_layer_state == TextureUses::UNKNOWN
                            && new_state != TextureUses::UNKNOWN
                        {
                            // We now know something about this subresource that
                            // we didn't before so we should go back and update
                            // the start state.
                            //
                            // We know we must have starter state be complex,
                            // otherwise we would know about this state.
                            strict_assert!(start_complex.is_some());

                            let start_complex =
                                unsafe { start_complex.as_deref_mut().unwrap_unchecked() };

                            strict_assert!(mip_id < start_complex.mips.len());

                            let start_mip = unsafe { start_complex.mips.get_unchecked_mut(mip_id) };

                            for &mut (_, ref mut current_start_state) in
                                start_mip.isolate(layers, TextureUses::UNKNOWN)
                            {
                                strict_assert_eq!(*current_start_state, TextureUses::UNKNOWN);
                                *current_start_state = new_state;
                            }

                            start_mip.coalesce();
                        }

                        *current_layer_state = new_state;
                    }

                    mip.coalesce();
                }
            }
        }
    }
}

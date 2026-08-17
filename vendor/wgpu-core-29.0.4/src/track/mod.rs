/*! Resource State and Lifetime Trackers

These structures are responsible for keeping track of resource state,
generating barriers where needednd making sure resources are kept
alive until the trackers die.

## General Architecture

Tracking is some of the hottest code in the entire codebase, so the trackers
are designed to be as cache efficient as possible. They store resource state
in flat vectors, storing metadata SOA style, one vector per type of metadata.

A lot of the tracker code is deeply unsafe, using unchecked accesses all over
to make performance as good as possible. However, for all unsafe accesses, there
is a corresponding debug assert the checks if that access is valid. This helps
get bugs caught fast, while still letting users not need to pay for the bounds
checks.

In wgpu, each resource ID includes a bitfield holding an index.
Indices are allocated and re-used, so they will always be as low as
reasonably possible. This allows us to use IDs to index into an array
of tracking information.

## Statefulness

There are two main types of trackers, stateful and stateless.

Stateful trackers are for buffers and textures. They both have
resource state attached to them which needs to be used to generate
automatic synchronization. Because of the different requirements of
buffers and textures, they have two separate tracking structures.

Stateless trackers only store metadata and own the given resource.

## Use Case

Within each type of tracker, the trackers are further split into 3 different
use cases, Bind Group, Usage Scopend a full Tracker.

Bind Group trackers are just a list of different resources, their refcount,
and how they are used. Textures are used via a selector and a usage type.
Buffers by just a usage type. Stateless resources don't have a usage type.

Usage Scope trackers are only for stateful resources. These trackers represent
a single [`UsageScope`] in the spec. When a use is added to a usage scope,
it is merged with all other uses of that resource in that scope. If there
is a usage conflict, merging will fail and an error will be reported.

Full trackers represent a before and after state of a resource. These
are used for tracking on the device and on command buffers. The before
state represents the state the resource is first used as in the command buffer,
the after state is the state the command buffer leaves the resource in.
These double ended buffers can then be used to generate the needed transitions
between command buffers.

## Dense Datastructure with Sparse Data

This tracking system is based on having completely dense data, but trackers do
not always contain every resource. Some resources (or even most resources) go
unused in any given command buffer. So to help speed up the process of iterating
through possibly thousands of resources, we use a bit vector to represent if
a resource is in the buffer or not. This allows us extremely efficient memory
utilizations well as being able to bail out of whole blocks of 32-64 resources
with a single usize comparison with zero. In practice this means that merging
partially resident buffers is extremely quick.

The main advantage of this dense datastructure is that we can do merging
of trackers in an extremely efficient fashion that results in us doing linear
scans down a couple of buffers. CPUs and their caches absolutely eat this up.

## Stateful Resource Operations

All operations on stateful trackers boil down to one of four operations:
- `insert(tracker, new_state)` adds a resource with a given state to the tracker
  for the first time.
- `merge(tracker, new_state)` merges this new state with the previous state, checking
  for usage conflicts.
- `barrier(tracker, new_state)` compares the given state to the existing state and
  generates the needed barriers.
- `update(tracker, new_state)` takes the given new state and overrides the old state.

This allows us to compose the operations to form the various kinds of tracker merges
that need to happen in the codebase. For each resource in the given merger, the following
operation applies:

```text
UsageScope <- Resource = insert(scope, usage) OR merge(scope, usage)
UsageScope <- UsageScope = insert(scope, scope) OR merge(scope, scope)
CommandBuffer <- UsageScope = insert(buffer.start, buffer.end, scope)
                              OR barrier(buffer.end, scope) + update(buffer.end, scope)
Device <- CommandBuffer = insert(device.start, device.end, buffer.start, buffer.end)
                          OR barrier(device.end, buffer.start) + update(device.end, buffer.end)
```

[`UsageScope`]: https://gpuweb.github.io/gpuweb/#programming-model-synchronization
*/

mod blas;
mod buffer;
mod metadata;
mod range;
mod stateless;
mod texture;

use crate::{
    binding_model, command,
    lock::{rank, Mutex},
    pipeline,
    resource::{self, Labeled, RawResourceAccess, ResourceErrorIdent},
    snatch::SnatchGuard,
    track::blas::BlasTracker,
};

use alloc::{sync::Arc, vec::Vec};
use bitflags::Flags;
use core::{fmt, mem, ops};

use thiserror::Error;

pub(crate) use buffer::{
    BufferBindGroupState, BufferTracker, BufferUsageScope, DeviceBufferTracker,
};
use metadata::{ResourceMetadata, ResourceMetadataProvider};
pub(crate) use stateless::StatelessTracker;
pub(crate) use texture::{
    DeviceTextureTracker, TextureTracker, TextureTrackerSetSingle, TextureUsageScope,
    TextureViewBindGroupState,
};
use wgt::{
    error::{ErrorType, WebGpuError},
    strict_assert_ne,
};

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct TrackerIndex(u32);

impl TrackerIndex {
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// wgpu-core internally use some array-like storage for tracking resources.
/// To that end, there needs to be a uniquely assigned index for each live resource
/// of a certain type. This index is separate from the resource ID for various reasons:
/// - There can be multiple resource IDs pointing the the same resource.
/// - IDs of dead handles can be recycled while resources are internally held alive (and tracked).
/// - The plan is to remove IDs in the long run
///   ([#5121](https://github.com/gfx-rs/wgpu/issues/5121)).
///
/// In order to produce these tracker indices, there is a shared TrackerIndexAllocator
/// per resource type. Indices have the same lifetime as the internal resource they
/// are associated to (alloc happens when creating the resource and free is called when
/// the resource is dropped).
struct TrackerIndexAllocator {
    unused: Vec<TrackerIndex>,
    next_index: TrackerIndex,
}

impl TrackerIndexAllocator {
    pub fn new() -> Self {
        TrackerIndexAllocator {
            unused: Vec::new(),
            next_index: TrackerIndex(0),
        }
    }

    pub fn alloc(&mut self) -> TrackerIndex {
        if let Some(index) = self.unused.pop() {
            return index;
        }

        let index = self.next_index;
        self.next_index.0 += 1;

        index
    }

    pub fn free(&mut self, index: TrackerIndex) {
        self.unused.push(index);
    }

    // This is used to pre-allocate the tracker storage.
    pub fn size(&self) -> usize {
        self.next_index.0 as usize
    }
}

impl fmt::Debug for TrackerIndexAllocator {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(())
    }
}

/// See TrackerIndexAllocator.
#[derive(Debug)]
pub(crate) struct SharedTrackerIndexAllocator {
    inner: Mutex<TrackerIndexAllocator>,
}

impl SharedTrackerIndexAllocator {
    pub fn new() -> Self {
        SharedTrackerIndexAllocator {
            inner: Mutex::new(
                rank::SHARED_TRACKER_INDEX_ALLOCATOR_INNER,
                TrackerIndexAllocator::new(),
            ),
        }
    }

    pub fn alloc(&self) -> TrackerIndex {
        self.inner.lock().alloc()
    }

    pub fn free(&self, index: TrackerIndex) {
        self.inner.lock().free(index);
    }

    pub fn size(&self) -> usize {
        self.inner.lock().size()
    }
}

pub(crate) struct TrackerIndexAllocators {
    pub buffers: Arc<SharedTrackerIndexAllocator>,
    pub textures: Arc<SharedTrackerIndexAllocator>,
    pub external_textures: Arc<SharedTrackerIndexAllocator>,
    pub samplers: Arc<SharedTrackerIndexAllocator>,
    pub bind_groups: Arc<SharedTrackerIndexAllocator>,
    pub compute_pipelines: Arc<SharedTrackerIndexAllocator>,
    pub render_pipelines: Arc<SharedTrackerIndexAllocator>,
    pub bundles: Arc<SharedTrackerIndexAllocator>,
    pub query_sets: Arc<SharedTrackerIndexAllocator>,
    pub blas_s: Arc<SharedTrackerIndexAllocator>,
    pub tlas_s: Arc<SharedTrackerIndexAllocator>,
}

impl TrackerIndexAllocators {
    pub fn new() -> Self {
        TrackerIndexAllocators {
            buffers: Arc::new(SharedTrackerIndexAllocator::new()),
            textures: Arc::new(SharedTrackerIndexAllocator::new()),
            external_textures: Arc::new(SharedTrackerIndexAllocator::new()),
            samplers: Arc::new(SharedTrackerIndexAllocator::new()),
            bind_groups: Arc::new(SharedTrackerIndexAllocator::new()),
            compute_pipelines: Arc::new(SharedTrackerIndexAllocator::new()),
            render_pipelines: Arc::new(SharedTrackerIndexAllocator::new()),
            bundles: Arc::new(SharedTrackerIndexAllocator::new()),
            query_sets: Arc::new(SharedTrackerIndexAllocator::new()),
            blas_s: Arc::new(SharedTrackerIndexAllocator::new()),
            tlas_s: Arc::new(SharedTrackerIndexAllocator::new()),
        }
    }
}

/// A structure containing all the information about a particular resource
/// transition. User code should be able to generate a pipeline barrier
/// based on the contents.
#[derive(Debug, PartialEq)]
pub(crate) struct PendingTransition<S: ResourceUses> {
    pub id: u32,
    pub selector: S::Selector,
    pub usage: hal::StateTransition<S>,
}

pub(crate) type PendingTransitionList = Vec<PendingTransition<wgt::TextureUses>>;

impl PendingTransition<wgt::BufferUses> {
    /// Produce the hal barrier corresponding to the transition.
    pub fn into_hal<'a>(
        self,
        buf: &'a resource::Buffer,
        snatch_guard: &'a SnatchGuard<'a>,
    ) -> hal::BufferBarrier<'a, dyn hal::DynBuffer> {
        let buffer = buf.raw(snatch_guard).expect("Buffer is destroyed");
        hal::BufferBarrier {
            buffer,
            usage: self.usage,
        }
    }
}

impl PendingTransition<wgt::TextureUses> {
    /// Produce the hal barrier corresponding to the transition.
    pub fn into_hal(
        self,
        texture: &dyn hal::DynTexture,
    ) -> hal::TextureBarrier<'_, dyn hal::DynTexture> {
        // These showing up in a barrier is always a bug
        strict_assert_ne!(self.usage.from, wgt::TextureUses::UNKNOWN);
        strict_assert_ne!(self.usage.to, wgt::TextureUses::UNKNOWN);

        let mip_count = self.selector.mips.end - self.selector.mips.start;
        strict_assert_ne!(mip_count, 0);
        let layer_count = self.selector.layers.end - self.selector.layers.start;
        strict_assert_ne!(layer_count, 0);

        hal::TextureBarrier {
            texture,
            range: wgt::ImageSubresourceRange {
                aspect: wgt::TextureAspect::All,
                base_mip_level: self.selector.mips.start,
                mip_level_count: Some(mip_count),
                base_array_layer: self.selector.layers.start,
                array_layer_count: Some(layer_count),
            },
            usage: self.usage,
        }
    }
}

/// The uses that a resource or subresource can be in.
pub(crate) trait ResourceUses:
    fmt::Debug + ops::BitAnd<Output = Self> + ops::BitOr<Output = Self> + PartialEq + Sized + Copy
{
    /// All flags that are exclusive.
    const EXCLUSIVE: Self;

    /// The selector used by this resource.
    type Selector: fmt::Debug;

    /// Turn the resource into a pile of bits.
    fn bits(self) -> u16;
    /// Returns true if any of the uses are exclusive.
    fn any_exclusive(self) -> bool;
}

/// Returns true if the given states violates the usage scope rule
/// of any(inclusive) XOR one(exclusive)
fn invalid_resource_state<T: ResourceUses>(state: T) -> bool {
    // Is power of two also means "is one bit set". We check for this as if
    // we're in any exclusive state, we must only be in a single state.
    state.any_exclusive() && !state.bits().is_power_of_two()
}

/// Returns true if the transition from one state to another does not require
/// a barrier.
fn skip_barrier<F: Flags>(old_state: F, ordered_uses_mask: F, new_state: F) -> bool {
    // If the state didn't change and all the usages are ordered, the hardware
    // will guarantee the order of accesses, so we do not need to issue a barrier at all
    old_state.bits() == new_state.bits() && ordered_uses_mask.contains(old_state)
}

#[derive(Clone, Debug, Error)]
pub enum ResourceUsageCompatibilityError {
    #[error("Attempted to use {res} with {invalid_use}.")]
    Buffer {
        res: ResourceErrorIdent,
        invalid_use: InvalidUse<wgt::BufferUses>,
    },
    #[error(
        "Attempted to use {res} (mips {mip_levels:?} layers {array_layers:?}) with {invalid_use}."
    )]
    Texture {
        res: ResourceErrorIdent,
        mip_levels: ops::Range<u32>,
        array_layers: ops::Range<u32>,
        invalid_use: InvalidUse<wgt::TextureUses>,
    },
}

impl WebGpuError for ResourceUsageCompatibilityError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

impl ResourceUsageCompatibilityError {
    fn from_buffer(
        buffer: &resource::Buffer,
        current_state: wgt::BufferUses,
        new_state: wgt::BufferUses,
    ) -> Self {
        Self::Buffer {
            res: buffer.error_ident(),
            invalid_use: InvalidUse {
                current_state,
                new_state,
            },
        }
    }

    fn from_texture(
        texture: &resource::Texture,
        selector: wgt::TextureSelector,
        current_state: wgt::TextureUses,
        new_state: wgt::TextureUses,
    ) -> Self {
        Self::Texture {
            res: texture.error_ident(),
            mip_levels: selector.mips,
            array_layers: selector.layers,
            invalid_use: InvalidUse {
                current_state,
                new_state,
            },
        }
    }
}

/// Pretty print helper that shows helpful descriptions of a conflicting usage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidUse<T> {
    current_state: T,
    new_state: T,
}

impl<T: ResourceUses> fmt::Display for InvalidUse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let current = self.current_state;
        let new = self.new_state;

        let current_exclusive = current & T::EXCLUSIVE;
        let new_exclusive = new & T::EXCLUSIVE;

        let exclusive = current_exclusive | new_exclusive;

        // The text starts with "tried to use X resource with {self}"
        write!(
            f,
            "conflicting usages. Current usage {current:?} and new usage {new:?}. \
            {exclusive:?} is an exclusive usage and cannot be used with any other \
            usages within the usage scope (renderpass or compute dispatch)"
        )
    }
}

/// All the usages that a bind group contains. The uses are not deduplicated in any way
/// and may include conflicting uses. This is fully compliant by the WebGPU spec.
///
/// All bind group states are sorted by their ID so that when adding to a tracker,
/// they are added in the most efficient order possible (ascending order).
#[derive(Debug)]
pub(crate) struct BindGroupStates {
    pub buffers: BufferBindGroupState,
    pub views: TextureViewBindGroupState,
    pub external_textures: StatelessTracker<resource::ExternalTexture>,
    pub samplers: StatelessTracker<resource::Sampler>,
    pub acceleration_structures: StatelessTracker<resource::Tlas>,
}

impl BindGroupStates {
    pub fn new() -> Self {
        Self {
            buffers: BufferBindGroupState::new(),
            views: TextureViewBindGroupState::new(),
            external_textures: StatelessTracker::new(),
            samplers: StatelessTracker::new(),
            acceleration_structures: StatelessTracker::new(),
        }
    }

    /// Optimize the bind group states by sorting them by ID.
    ///
    /// When this list of states is merged into a tracker, the memory
    /// accesses will be in a constant ascending order.
    pub fn optimize(&mut self) {
        self.buffers.optimize();
        // Views are stateless, however, `TextureViewBindGroupState`
        // is special as it will be merged with other texture trackers.
        self.views.optimize();
        // Samplers and Tlas's are stateless and don't need to be optimized
        // since the tracker is never merged with any other tracker.
    }
}

/// This is a render bundle specific usage scope. It includes stateless resources
/// that are not normally included in a usage scope, but are used by render bundles
/// and need to be owned by the render bundles.
#[derive(Debug)]
pub(crate) struct RenderBundleScope {
    pub buffers: BufferUsageScope,
    pub textures: TextureUsageScope,
    // Don't need to track views and samplers, they are never used directly, only by bind groups.
    pub bind_groups: StatelessTracker<binding_model::BindGroup>,
    pub render_pipelines: StatelessTracker<pipeline::RenderPipeline>,
}

impl RenderBundleScope {
    /// Create the render bundle scope and pull the maximum IDs from the hubs.
    pub fn new() -> Self {
        Self {
            buffers: BufferUsageScope::default(),
            textures: TextureUsageScope::default(),
            bind_groups: StatelessTracker::new(),
            render_pipelines: StatelessTracker::new(),
        }
    }

    /// Merge the inner contents of a bind group into the render bundle tracker.
    ///
    /// Only stateful things are merged in herell other resources are owned
    /// indirectly by the bind group.
    ///
    /// # Safety
    ///
    /// The maximum ID given by each bind group resource must be less than the
    /// length of the storage given at the call to `new`.
    pub unsafe fn merge_bind_group(
        &mut self,
        bind_group: &BindGroupStates,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        unsafe { self.buffers.merge_bind_group(&bind_group.buffers)? };
        unsafe { self.textures.merge_bind_group(&bind_group.views)? };

        Ok(())
    }
}

/// A pool for storing the memory used by [`UsageScope`]s. We take and store this memory when the
/// scope is dropped to avoid reallocating. The memory required only grows and allocation cost is
/// significant when a large number of resources have been used.
pub(crate) type UsageScopePool = Mutex<Vec<(BufferUsageScope, TextureUsageScope)>>;

/// A usage scope tracker. Only needs to store stateful resources as stateless
/// resources cannot possibly have a usage conflict.
#[derive(Debug)]
pub(crate) struct UsageScope<'a> {
    pub pool: &'a UsageScopePool,
    pub buffers: BufferUsageScope,
    pub textures: TextureUsageScope,
}

impl<'a> Drop for UsageScope<'a> {
    fn drop(&mut self) {
        // clear vecs and push into pool
        self.buffers.clear();
        self.textures.clear();
        self.pool
            .lock()
            .push((mem::take(&mut self.buffers), mem::take(&mut self.textures)));
    }
}

impl UsageScope<'static> {
    pub fn new_pooled<'d>(
        pool: &'d UsageScopePool,
        tracker_indices: &TrackerIndexAllocators,
        ordered_buffer_usages: wgt::BufferUses,
        ordered_texture_usages: wgt::TextureUses,
    ) -> UsageScope<'d> {
        let pooled = pool.lock().pop().unwrap_or_default();

        let mut scope = UsageScope::<'d> {
            pool,
            buffers: pooled.0,
            textures: pooled.1,
        };

        scope.buffers.set_size(tracker_indices.buffers.size());
        scope.buffers.set_ordered_uses_mask(ordered_buffer_usages);
        scope.textures.set_size(tracker_indices.textures.size());
        scope.textures.set_ordered_uses_mask(ordered_texture_usages);
        scope
    }
}

impl<'a> UsageScope<'a> {
    /// Merge the inner contents of a bind group into the usage scope.
    ///
    /// Only stateful things are merged in herell other resources are owned
    /// indirectly by the bind group.
    ///
    /// # Safety
    ///
    /// The maximum ID given by each bind group resource must be less than the
    /// length of the storage given at the call to `new`.
    pub unsafe fn merge_bind_group(
        &mut self,
        bind_group: &BindGroupStates,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        unsafe {
            self.buffers.merge_bind_group(&bind_group.buffers)?;
            self.textures.merge_bind_group(&bind_group.views)?;
        }

        Ok(())
    }

    /// Merge the inner contents of a bind group into the usage scope.
    ///
    /// Only stateful things are merged in herell other resources are owned
    /// indirectly by a bind group or are merged directly into the command buffer tracker.
    ///
    /// # Safety
    ///
    /// The maximum ID given by each bind group resource must be less than the
    /// length of the storage given at the call to `new`.
    pub unsafe fn merge_render_bundle(
        &mut self,
        render_bundle: &RenderBundleScope,
    ) -> Result<(), ResourceUsageCompatibilityError> {
        self.buffers.merge_usage_scope(&render_bundle.buffers)?;
        self.textures.merge_usage_scope(&render_bundle.textures)?;

        Ok(())
    }
}

/// A tracker used by Device.
pub(crate) struct DeviceTracker {
    pub buffers: DeviceBufferTracker,
    pub textures: DeviceTextureTracker,
}

impl DeviceTracker {
    pub fn new(
        ordered_buffer_usages: wgt::BufferUses,
        ordered_texture_usages: wgt::TextureUses,
    ) -> Self {
        Self {
            buffers: DeviceBufferTracker::new(ordered_buffer_usages),
            textures: DeviceTextureTracker::new(ordered_texture_usages),
        }
    }
}

/// A full double sided tracker used by CommandBuffers.
pub(crate) struct Tracker {
    /// Buffers used within this command buffer.
    ///
    /// For compute passes, this only includes buffers actually used by the
    /// pipeline (contrast with the `bind_groups` member).
    pub buffers: BufferTracker,

    /// Textures used within this command buffer.
    ///
    /// For compute passes, this only includes textures actually used by the
    /// pipeline (contrast with the `bind_groups` member).
    pub textures: TextureTracker,

    pub blas_s: BlasTracker,
    pub tlas_s: StatelessTracker<resource::Tlas>,
    pub views: StatelessTracker<resource::TextureView>,

    /// Contains all bind groups that were passed in any call to
    /// `set_bind_group` on the encoder.
    ///
    /// WebGPU requires that submission fails if any resource in any of these
    /// bind groups is destroyed, even if the resource is not actually used by
    /// the pipeline (e.g. because the pipeline does not use the bound slot, or
    /// because the bind group was replaced by a subsequent call to
    /// `set_bind_group`).
    pub bind_groups: StatelessTracker<binding_model::BindGroup>,

    pub compute_pipelines: StatelessTracker<pipeline::ComputePipeline>,
    pub render_pipelines: StatelessTracker<pipeline::RenderPipeline>,
    pub bundles: StatelessTracker<command::RenderBundle>,
    pub query_sets: StatelessTracker<resource::QuerySet>,
}

impl Tracker {
    pub fn new(
        ordered_buffer_usages: wgt::BufferUses,
        ordered_texture_usages: wgt::TextureUses,
    ) -> Self {
        Self {
            buffers: BufferTracker::new(ordered_buffer_usages),
            textures: TextureTracker::new(ordered_texture_usages),
            blas_s: BlasTracker::new(),
            tlas_s: StatelessTracker::new(),
            views: StatelessTracker::new(),
            bind_groups: StatelessTracker::new(),
            compute_pipelines: StatelessTracker::new(),
            render_pipelines: StatelessTracker::new(),
            bundles: StatelessTracker::new(),
            query_sets: StatelessTracker::new(),
        }
    }

    /// Iterates through all resources in the given bind group and adopts
    /// the state given for those resources in the UsageScope. It also
    /// removes all touched resources from the usage scope.
    ///
    /// If a transition is needed to get the resources into the needed
    /// state, those transitions are stored within the tracker. A
    /// subsequent call to [`BufferTracker::drain_transitions`] or
    /// [`TextureTracker::drain_transitions`] is needed to get those transitions.
    ///
    /// This is a really funky method used by Compute Passes to generate
    /// barriers after a call to dispatch without needing to iterate
    /// over all elements in the usage scope. We use each the
    /// bind group as a source of which IDs to look at. The bind groups
    /// must have first been added to the usage scope.
    ///
    /// Only stateful things are merged in here, all other resources are owned
    /// indirectly by the bind group.
    ///
    /// # Panics
    ///
    /// If a resource in the `bind_group` is not found in the usage scope.
    pub fn set_and_remove_from_usage_scope_sparse(
        &mut self,
        scope: &mut UsageScope,
        bind_group: &BindGroupStates,
    ) {
        self.buffers.set_and_remove_from_usage_scope_sparse(
            &mut scope.buffers,
            bind_group.buffers.used_tracker_indices(),
        );
        self.textures
            .set_and_remove_from_usage_scope_sparse(&mut scope.textures, &bind_group.views);
    }
}

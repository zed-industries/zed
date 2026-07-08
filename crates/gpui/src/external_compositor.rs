//! Backend-neutral external compositor integration.
//!
//! This module lets a window register an external, backend-specific compositor (for
//! example, a wgpu-based 3D renderer) that produces a texture to be composited into a
//! rectangular region of the GPUI scene. GPUI controls *when* the composition happens
//! (a controlled point within its own frame, before the frame is submitted), which
//! preserves draw ordering and lets a single backend own the one GPU submission for
//! the frame.
//!
//! No backend-specific types (e.g. `wgpu`) appear anywhere in this module's public
//! API, and this module has no notion of a "compose" callback at all: a slot's
//! compositor is stored as an opaque [`Box<dyn Any>`], entirely defined by the
//! backend that registered it. For example, the `gpui_wgpu` backend double-boxes a
//! `Box<dyn gpui_wgpu::WgpuExternalCompositor>` (`Box::new(Box::new(compositor) as
//! Box<dyn WgpuExternalCompositor>) as Box<dyn Any>`) so that a single
//! `downcast_mut::<Box<dyn WgpuExternalCompositor>>()` recovers a concrete,
//! frame-lifetime-safe trait object without ever needing to erase a non-`'static`
//! lifetime (e.g. a frame-scoped `&mut wgpu::CommandEncoder`) through `Any`.
//!
//! Backends resolve a registered slot's compositor via [`ExternalCompositorRegistry::take_compositor`]
//! / [`ExternalCompositorRegistry::put_back_compositor`] and call into it using their
//! own backend-specific trait; this module only tracks slot bookkeeping (liveness,
//! generations, frame-in-flight state).

use crate::{DevicePixels, Size};
use std::any::Any;
use std::fmt;

/// An opaque handle to a slot registered with an [`ExternalCompositorRegistry`].
///
/// Handles carry a generation counter to guard against ABA problems: once a slot is
/// freed and its index reused, handles referring to the previous occupant compare
/// unequal to the new handle and are rejected by the registry as stale.
/// `generation == 0` is reserved for [`ExternalSlotHandle::INVALID`] and can never be
/// produced by [`ExternalCompositorRegistry::register`].
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternalSlotHandle {
    index: u32,
    generation: u32,
}

impl ExternalSlotHandle {
    /// A handle that never refers to a valid slot.
    pub const INVALID: Self = Self {
        index: 0,
        generation: 0,
    };

    /// Returns `true` unless this handle is [`Self::INVALID`].
    ///
    /// This is a cheap, local check only: it does not guarantee that the slot is
    /// still registered (or still *live*, as opposed to stale) in any particular
    /// registry. Use [`ExternalCompositorRegistry::is_valid`] to check liveness.
    pub fn is_valid(self) -> bool {
        self.generation != 0
    }
}

impl Default for ExternalSlotHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Pixel format of an external compositor slot's target texture.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExternalSlotFormat {
    /// 8-bit RGBA, sRGB-encoded.
    Rgba8UnormSrgb,
    /// 8-bit BGRA, sRGB-encoded.
    Bgra8UnormSrgb,
    /// 16-bit float RGBA (linear).
    Rgba16Float,
}

/// How alpha is encoded in an external compositor slot's target texture.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AlphaMode {
    /// Color channels are already multiplied by alpha.
    PreMultiplied,
    /// Color channels are independent of alpha (straight/unassociated alpha).
    Straight,
}

/// Immutable metadata describing an external compositor slot, validated when the
/// slot is registered via [`ExternalCompositorRegistry::register`].
#[derive(Clone, Debug)]
pub struct ExternalSlotDescriptor {
    /// Pixel format of the slot's target texture.
    pub format: ExternalSlotFormat,
    /// Alpha encoding of the slot's target texture.
    pub alpha_mode: AlphaMode,
    /// Width of the slot's target texture, in texels.
    pub width: u32,
    /// Height of the slot's target texture, in texels.
    pub height: u32,
    /// Sample count of the slot's target texture. Must be `1`: multisample resolve,
    /// if any, is the external renderer's responsibility, not GPUI's.
    pub sample_count: u32,
    /// The graphics context generation this slot is being registered under. Must
    /// match [`ExternalCompositorRegistry::current_context_generation`] at the time
    /// of registration, or [`ExternalCompositorError::GenerationMismatch`] is
    /// returned.
    pub context_generation: u64,
}

/// Errors returned by [`ExternalCompositorRegistry`] operations.
#[derive(Debug)]
pub enum ExternalCompositorError {
    /// The handle does not refer to a currently registered slot: it was never
    /// registered, has already been unregistered and drained, or refers to a slot
    /// index that has since been reused by a different registration (an ABA
    /// mismatch — see [`ExternalCompositorRegistry::unregister`]).
    StaleHandle,
    /// The descriptor's `context_generation` did not match the registry's current
    /// generation.
    GenerationMismatch {
        /// The generation the caller supplied.
        got: u64,
        /// The registry's current generation.
        current: u64,
    },
    /// The requested format is not supported by the registry.
    UnsupportedFormat(ExternalSlotFormat),
    /// `sample_count` must be `1`; multisample resolve is the external renderer's
    /// responsibility.
    InvalidSampleCount(u32),
    /// `width` or `height` was zero, or exceeded `i32::MAX` (the limit every
    /// consumer of [`DevicePixels`] — a signed 32-bit type — can represent).
    InvalidDimensions {
        /// The width the caller supplied.
        width: u32,
        /// The height the caller supplied.
        height: u32,
    },
}

/// Outcome of a successful [`ExternalCompositorRegistry::unregister`] call.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnregisterOutcome {
    /// The slot was freed immediately.
    ///
    /// This includes the *context-recreation cleanup* case: unregistering a handle
    /// that [`ExternalCompositorRegistry::on_context_recreated`] previously marked
    /// stale always frees it immediately (see that method's docs) — there's nothing
    /// to defer, since a stale slot is never composed.
    Removed,
    /// The slot's most recently painted frame had not yet been composed, so
    /// freeing it was deferred until composition catches up (see
    /// [`ExternalCompositorRegistry::drain_pending_removals`]). The slot's
    /// resources (descriptor, compositor) remain live in the meantime.
    Deferred {
        /// The frame the slot was last painted in; composition of this frame must
        /// complete before the slot is actually freed.
        until_after_frame: u64,
    },
}

impl fmt::Display for ExternalCompositorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExternalCompositorError::StaleHandle => {
                write!(f, "external compositor handle is stale or unknown")
            }
            ExternalCompositorError::GenerationMismatch { got, current } => write!(
                f,
                "external compositor context generation mismatch: descriptor has {got}, registry is at {current}"
            ),
            ExternalCompositorError::UnsupportedFormat(format) => {
                write!(f, "unsupported external compositor slot format: {format:?}")
            }
            ExternalCompositorError::InvalidSampleCount(count) => write!(
                f,
                "invalid external compositor sample count {count}; only 1 is supported"
            ),
            ExternalCompositorError::InvalidDimensions { width, height } => write!(
                f,
                "external compositor slot dimensions {width}x{height} are invalid: width and \
                 height must both be non-zero and no greater than {}",
                i32::MAX
            ),
        }
    }
}

impl std::error::Error for ExternalCompositorError {}

/// A single slot's mutable bookkeeping. Not exposed outside the registry.
struct Slot {
    occupied: bool,
    generation: u32,
    /// Set by [`ExternalCompositorRegistry::on_context_recreated`]. A stale slot's
    /// resources (descriptor, compositor) are kept around — it is *not* freed — but
    /// [`ExternalCompositorRegistry::is_valid`] reports it as invalid, and backends
    /// must not compose it (see that method's docs for the full contract).
    context_stale: bool,
    pending_removal: bool,
    descriptor: Option<ExternalSlotDescriptor>,
    compositor: Option<Box<dyn Any>>,
    last_painted_frame: u64,
    /// The `last_painted_frame` value as of the most recent [`ExternalCompositorRegistry::mark_processed`]
    /// call: i.e. the last painted frame the backend has finished handling, in
    /// whatever way (composed and drew it, or skipped it for any reason).
    last_processed_frame: u64,
}

impl Slot {
    fn vacant() -> Self {
        Self {
            occupied: false,
            generation: 0,
            context_stale: false,
            pending_removal: false,
            descriptor: None,
            compositor: None,
            last_painted_frame: 0,
            last_processed_frame: 0,
        }
    }
}

/// Per-window registry of external compositor slots.
///
/// Shared as `Rc<RefCell<ExternalCompositorRegistry>>` (no `Send`/`Sync` bound: GPUI's
/// event loop is single-threaded). The [`Window`](crate::Window) paint path inserts a
/// neutral [`crate::ExternalCompositorPrimitive`] referencing a slot by handle; the
/// backend renderer later resolves that handle through this registry to reach the
/// opaque compositor stored in the slot (see [`Self::take_compositor`]).
///
/// Slot storage is a small inline slot map (index + generation) with no external
/// dependency: freed indices are recycled, and their generation is bumped on reuse so
/// stale handles from a previous occupant are rejected.
pub struct ExternalCompositorRegistry {
    slots: Vec<Slot>,
    free_indices: Vec<u32>,
    current_context_generation: u64,
}

impl Default for ExternalCompositorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalCompositorRegistry {
    /// Creates an empty registry. [`Self::current_context_generation`] starts at `1`,
    /// matching the convention used by backend graphics contexts (generation `0` is
    /// reserved to mean "invalid").
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_indices: Vec::new(),
            current_context_generation: 1,
        }
    }

    /// The current graphics context generation. Descriptors passed to
    /// [`Self::register`] must match this value.
    pub fn current_context_generation(&self) -> u64 {
        self.current_context_generation
    }

    /// Registers a new slot with the given descriptor and an opaque,
    /// backend-defined compositor.
    ///
    /// `compositor` is entirely defined by the backend that registers it — this
    /// module never calls into it. Backends downcast it back via
    /// [`Self::take_compositor`] using their own backend-specific type (e.g. the
    /// `gpui_wgpu` backend double-boxes a `Box<dyn WgpuExternalCompositor>`; see the
    /// module docs).
    ///
    /// Validates that `descriptor.context_generation` matches
    /// [`Self::current_context_generation`], `descriptor.sample_count == 1`, and
    /// that `descriptor.width`/`descriptor.height` are both non-zero and no
    /// greater than `i32::MAX` (the limit [`DevicePixels`], a signed 32-bit type,
    /// can represent). All [`ExternalSlotFormat`] variants are accepted.
    pub fn register(
        &mut self,
        descriptor: ExternalSlotDescriptor,
        compositor: Box<dyn Any>,
    ) -> Result<ExternalSlotHandle, ExternalCompositorError> {
        if descriptor.context_generation != self.current_context_generation {
            return Err(ExternalCompositorError::GenerationMismatch {
                got: descriptor.context_generation,
                current: self.current_context_generation,
            });
        }
        if descriptor.sample_count != 1 {
            return Err(ExternalCompositorError::InvalidSampleCount(
                descriptor.sample_count,
            ));
        }
        if descriptor.width == 0
            || descriptor.height == 0
            || descriptor.width > i32::MAX as u32
            || descriptor.height > i32::MAX as u32
        {
            return Err(ExternalCompositorError::InvalidDimensions {
                width: descriptor.width,
                height: descriptor.height,
            });
        }

        let index = if let Some(index) = self.free_indices.pop() {
            index
        } else {
            self.slots.push(Slot::vacant());
            (self.slots.len() - 1) as u32
        };

        let slot = &mut self.slots[index as usize];
        // Bump the generation on every (re)use so a handle from a previous occupant
        // of this index can never compare equal again. Skip 0, which is reserved for
        // `ExternalSlotHandle::INVALID`.
        let generation = slot.generation.wrapping_add(1).max(1);
        *slot = Slot {
            occupied: true,
            generation,
            context_stale: false,
            pending_removal: false,
            descriptor: Some(descriptor),
            compositor: Some(compositor),
            last_painted_frame: 0,
            last_processed_frame: 0,
        };

        Ok(ExternalSlotHandle { index, generation })
    }

    /// Unregisters a slot.
    ///
    /// Three distinct cases:
    ///
    /// - **Context-recreation cleanup**: `handle` refers to a slot previously marked
    ///   stale by [`Self::on_context_recreated`] (its index and generation still
    ///   match, but [`Self::is_valid`] reports `false`). Such a slot is never
    ///   composed by any backend, so there is nothing to wait for: it is freed
    ///   immediately and this returns `Ok(`[`UnregisterOutcome::Removed`]`)`.
    /// - **Frame in flight**: `handle` refers to a live (non-stale) slot whose most
    ///   recently painted frame has not yet been processed by the backend
    ///   (`last_painted_frame > last_processed_frame`; see [`Self::mark_processed`]).
    ///   Freeing it now would let the backend's texture disappear out from under a
    ///   still-pending composite, so
    ///   this marks the slot for deferred removal (drained automatically by
    ///   [`Self::drain_pending_removals`] once composition catches up) and returns
    ///   `Ok(`[`UnregisterOutcome::Deferred`]`)`; the slot's resources stay live in
    ///   the meantime, still resolvable by the same handle.
    /// - **Real ABA / unknown handle**: `handle`'s index is out of range, or is
    ///   occupied by a slot with a different generation (i.e. the index was already
    ///   freed and reused by an unrelated registration), or is not occupied at all.
    ///   Returns `Err(`[`ExternalCompositorError::StaleHandle`]`)`.
    pub fn unregister(
        &mut self,
        handle: ExternalSlotHandle,
    ) -> Result<UnregisterOutcome, ExternalCompositorError> {
        let index = handle.index as usize;
        let is_live_index = self
            .slots
            .get(index)
            .is_some_and(|slot| slot.occupied && slot.generation == handle.generation);
        if !is_live_index {
            return Err(ExternalCompositorError::StaleHandle);
        }

        if self.slots[index].context_stale {
            self.free_slot(handle.index);
            return Ok(UnregisterOutcome::Removed);
        }

        let slot = &mut self.slots[index];
        if slot.last_painted_frame > slot.last_processed_frame {
            slot.pending_removal = true;
            return Ok(UnregisterOutcome::Deferred {
                until_after_frame: slot.last_painted_frame,
            });
        }
        self.free_slot(handle.index);
        Ok(UnregisterOutcome::Removed)
    }

    /// The registered size of a slot's target texture, in device texels. This is
    /// *not* adjusted by the window's DPI scale factor: callers that need logical
    /// [`crate::Pixels`] (e.g. [`crate::elements::ExternalCompositorElement`]) divide
    /// by [`crate::Window::scale_factor`] themselves. Returns `None` if `handle` does
    /// not refer to a live slot.
    pub fn slot_size(&self, handle: ExternalSlotHandle) -> Option<Size<DevicePixels>> {
        let descriptor = self.descriptor(handle)?;
        Some(Size {
            width: DevicePixels(descriptor.width as i32),
            height: DevicePixels(descriptor.height as i32),
        })
    }

    /// The descriptor a slot was registered with, if it's still occupied (this
    /// includes slots marked stale by [`Self::on_context_recreated`] and slots
    /// pending removal, since their resources are still around; use [`Self::is_valid`]
    /// if you specifically need to know whether the slot is still *composable*).
    pub fn descriptor(&self, handle: ExternalSlotHandle) -> Option<&ExternalSlotDescriptor> {
        self.resolve(handle)?.descriptor.as_ref()
    }

    /// Returns `true` if `handle` refers to a slot that is occupied, *not* marked
    /// stale by a context recreation, and *not* pending removal (see
    /// [`Self::unregister`]'s frame-in-flight case). A slot pending removal is a
    /// zombie kept alive only long enough for the backend to catch up composing
    /// its last painted frame; it must never be reported valid, or a caller could
    /// mistake it for a live slot and keep painting into it. Application code that
    /// holds onto a handle across frames should poll this (e.g. once per frame) to
    /// discover it needs to unregister the old handle and register a fresh
    /// compositor.
    pub fn is_valid(&self, handle: ExternalSlotHandle) -> bool {
        self.resolve(handle)
            .is_some_and(|slot| !slot.context_stale && !slot.pending_removal)
    }

    /// Marks every currently occupied slot stale and advances
    /// [`Self::current_context_generation`] to `new_generation`. Called by the
    /// backend after recovering from a lost graphics context (e.g. device-lost on
    /// wgpu).
    ///
    /// This method is intentionally backend-neutral and does *not* touch any slot's
    /// compositor: this module has no way to call into an opaque `Box<dyn Any>`. It
    /// is the backend's responsibility to walk every slot returned by
    /// [`Self::occupied_handles`] (typically right after calling this method) and
    /// notify each compositor through its own backend-specific extension trait
    /// (e.g. `gpui_wgpu::WgpuExternalCompositor::on_context_recreated`).
    ///
    /// Marked-stale slots are *not* freed here: their descriptor and compositor stay
    /// live so the backend's notification walk (and any composition attempted before
    /// the app catches up) can still resolve them, and so
    /// [`Self::unregister`]'s context-recreation cleanup case can drop them once the
    /// app is done with them. [`Self::is_valid`] reports `false` for them in the
    /// meantime.
    ///
    /// This is a backend API (called by e.g. `gpui_wgpu`'s renderer after device
    /// recovery), not for application use.
    #[doc(hidden)]
    pub fn on_context_recreated(&mut self, new_generation: u64) {
        for slot in &mut self.slots {
            if slot.occupied {
                slot.context_stale = true;
            }
        }
        self.current_context_generation = new_generation;
    }

    /// Iterates the handles of every currently occupied slot, including ones marked
    /// stale by [`Self::on_context_recreated`] (their resources, and compositor, are
    /// still live until an explicit [`Self::unregister`] cleans them up). This is a
    /// backend API (e.g. used by `gpui_wgpu`'s renderer to walk every compositor
    /// after device recovery and notify it via a backend-specific extension trait),
    /// not for app use.
    #[doc(hidden)]
    pub fn occupied_handles(&self) -> impl Iterator<Item = ExternalSlotHandle> + '_ {
        self.slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.occupied)
            .map(|(index, slot)| ExternalSlotHandle {
                index: index as u32,
                generation: slot.generation,
            })
    }

    /// Records that `handle`'s slot was painted (i.e. its primitive was inserted into
    /// the scene) as part of `frame`. Called by [`crate::Window::paint_external_compositor`].
    ///
    /// A no-op on a slot pending removal (see [`Self::unregister`]'s frame-in-flight
    /// case): such a slot is a zombie waiting only for the backend to catch up on
    /// its *last* painted frame, and letting a further (stale) paint push
    /// `last_painted_frame` forward would extend its life indefinitely instead of
    /// letting [`Self::drain_pending_removals`] free it.
    pub(crate) fn mark_painted(&mut self, handle: ExternalSlotHandle, frame: u64) {
        if let Some(slot) = self.resolve_mut(handle)
            && !slot.pending_removal
        {
            slot.last_painted_frame = frame;
        }
    }

    /// Records that `handle`'s slot was processed this frame by the backend: its
    /// most recently painted frame was either composed and drawn, or skipped for
    /// any reason (not ready yet, a lost graphics context, a stale context
    /// generation, or any other non-fatal condition) — in every case, nothing about
    /// this slot is left in flight. Sets the slot's `last_processed_frame` to its
    /// current `last_painted_frame`.
    ///
    /// This intentionally takes no explicit frame number: the backend processes
    /// exactly the primitives painted into the scene it is currently drawing, so
    /// "processed" always means "caught up with the most recent paint", without
    /// requiring the backend to share gpui's [`crate::Window::frame_counter`]
    /// numbering. Backends must call this for every slot present in the scene being
    /// drawn, regardless of outcome — see [`Self::drain_pending_removals`], which
    /// depends on it to free slots pending removal. This is a backend API, not for
    /// app use.
    #[doc(hidden)]
    pub fn mark_processed(&mut self, handle: ExternalSlotHandle) {
        if let Some(slot) = self.resolve_mut(handle) {
            slot.last_processed_frame = slot.last_painted_frame;
        }
    }

    /// Removes and returns `handle`'s compositor, if any, so the backend renderer can
    /// call into it without holding a borrow of this registry (which may itself be
    /// reachable through the same `RefCell` the compositor needs to touch). Pair with
    /// [`Self::put_back_compositor`]. This is a backend API, not for app use.
    ///
    /// Resolves slots marked stale by [`Self::on_context_recreated`] too (their
    /// compositor is still live); backends that must not compose a stale slot are
    /// expected to check [`Self::is_valid`] (or compare
    /// [`Self::descriptor`]'s `context_generation`) themselves before calling this.
    #[doc(hidden)]
    pub fn take_compositor(&mut self, handle: ExternalSlotHandle) -> Option<Box<dyn Any>> {
        self.resolve_mut(handle)?.compositor.take()
    }

    /// Restores a compositor previously removed by [`Self::take_compositor`]. If the
    /// slot is no longer occupied (e.g. it was freed by [`Self::unregister`] in the
    /// interim), the compositor is dropped.
    /// This is a backend API, not for app use.
    #[doc(hidden)]
    pub fn put_back_compositor(&mut self, handle: ExternalSlotHandle, compositor: Box<dyn Any>) {
        if let Some(slot) = self.resolve_mut(handle) {
            slot.compositor = Some(compositor);
        }
    }

    /// Frees every slot marked for deferred removal whose last painted frame has
    /// since been processed (`last_processed_frame >= last_painted_frame`; see
    /// [`Self::mark_processed`]). Called by the backend renderer once per frame,
    /// after the frame that processed those slots actually landed (e.g. after the
    /// main encoder submit succeeded). This is a backend API, not for app use.
    #[doc(hidden)]
    pub fn drain_pending_removals(&mut self) {
        for index in 0..self.slots.len() {
            let slot = &self.slots[index];
            if slot.occupied
                && slot.pending_removal
                && slot.last_processed_frame >= slot.last_painted_frame
            {
                self.free_slot(index as u32);
            }
        }
    }

    fn resolve(&self, handle: ExternalSlotHandle) -> Option<&Slot> {
        let slot = self.slots.get(handle.index as usize)?;
        (slot.occupied && slot.generation == handle.generation).then_some(slot)
    }

    fn resolve_mut(&mut self, handle: ExternalSlotHandle) -> Option<&mut Slot> {
        let slot = self.slots.get_mut(handle.index as usize)?;
        (slot.occupied && slot.generation == handle.generation).then_some(slot)
    }

    fn free_slot(&mut self, index: u32) {
        if let Some(slot) = self.slots.get_mut(index as usize) {
            slot.occupied = false;
            slot.context_stale = false;
            slot.pending_removal = false;
            slot.descriptor = None;
            slot.compositor = None;
            self.free_indices.push(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_compositor() -> Box<dyn Any> {
        Box::new(())
    }

    fn descriptor(context_generation: u64) -> ExternalSlotDescriptor {
        ExternalSlotDescriptor {
            format: ExternalSlotFormat::Rgba8UnormSrgb,
            alpha_mode: AlphaMode::PreMultiplied,
            width: 64,
            height: 64,
            sample_count: 1,
            context_generation,
        }
    }

    #[test]
    fn register_and_unregister() {
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), dummy_compositor())
            .expect("register should succeed");
        assert!(handle.is_valid());
        assert!(registry.is_valid(handle));
        assert!(registry.descriptor(handle).is_some());

        registry
            .unregister(handle)
            .expect("unregister should succeed");
        assert!(registry.descriptor(handle).is_none());
    }

    #[test]
    fn generation_mismatch_is_rejected() {
        let mut registry = ExternalCompositorRegistry::new();
        let err = registry
            .register(descriptor(2), dummy_compositor())
            .unwrap_err();
        assert!(matches!(
            err,
            ExternalCompositorError::GenerationMismatch { got: 2, current: 1 }
        ));
    }

    #[test]
    fn invalid_descriptors_are_rejected() {
        let mut registry = ExternalCompositorRegistry::new();

        let mut bad_sample_count = descriptor(1);
        bad_sample_count.sample_count = 4;
        assert!(matches!(
            registry.register(bad_sample_count, dummy_compositor()),
            Err(ExternalCompositorError::InvalidSampleCount(4))
        ));

        let mut zero_width = descriptor(1);
        zero_width.width = 0;
        assert!(matches!(
            registry.register(zero_width, dummy_compositor()),
            Err(ExternalCompositorError::InvalidDimensions {
                width: 0,
                height: 64
            })
        ));
    }

    #[test]
    fn oversized_dimensions_are_rejected() {
        let mut registry = ExternalCompositorRegistry::new();

        let mut huge_width = descriptor(1);
        huge_width.width = i32::MAX as u32 + 1;
        assert!(matches!(
            registry.register(huge_width, dummy_compositor()),
            Err(ExternalCompositorError::InvalidDimensions { width, height: 64 })
                if width == i32::MAX as u32 + 1
        ));

        let mut huge_height = descriptor(1);
        huge_height.height = i32::MAX as u32 + 1;
        assert!(matches!(
            registry.register(huge_height, dummy_compositor()),
            Err(ExternalCompositorError::InvalidDimensions { width: 64, height })
                if height == i32::MAX as u32 + 1
        ));
    }

    #[test]
    fn context_recreation_marks_slots_stale_without_freeing() {
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), dummy_compositor())
            .unwrap();

        registry.on_context_recreated(2);
        assert_eq!(registry.current_context_generation(), 2);
        // Stale, but its resources (descriptor, compositor) are still around until
        // an explicit cleanup unregister.
        assert!(!registry.is_valid(handle));
        assert!(registry.descriptor(handle).is_some());
        assert_eq!(registry.occupied_handles().collect::<Vec<_>>(), [handle]);

        // Registering under the new generation succeeds and produces a fresh
        // handle that never compares equal to the stale one (it lands in a new
        // slot: the stale one hasn't been freed yet, so its index isn't reusable).
        let new_handle = registry
            .register(descriptor(2), dummy_compositor())
            .unwrap();
        assert_ne!(handle, new_handle);
        assert!(registry.is_valid(new_handle));

        // Cleanup unregister of the now-stale handle succeeds and frees it
        // immediately (no frame-in-flight deferral for stale slots).
        assert_eq!(
            registry.unregister(handle).unwrap(),
            UnregisterOutcome::Removed
        );
        assert!(registry.descriptor(handle).is_none());
    }

    #[test]
    fn unregister_after_real_aba_is_rejected() {
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), dummy_compositor())
            .unwrap();
        assert_eq!(
            registry.unregister(handle).unwrap(),
            UnregisterOutcome::Removed
        );

        // The freed index gets reused by this registration, with a bumped
        // generation, by an entirely unrelated caller.
        let reused = registry
            .register(descriptor(1), dummy_compositor())
            .unwrap();
        assert_ne!(handle, reused);

        // The original handle must not be treated as a valid cleanup target for
        // the slot now occupied by `reused`.
        assert!(matches!(
            registry.unregister(handle),
            Err(ExternalCompositorError::StaleHandle)
        ));
        assert!(registry.is_valid(reused));
    }

    #[test]
    fn frame_in_flight_defers_removal_until_processed() {
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), dummy_compositor())
            .unwrap();

        registry.mark_painted(handle, 5);
        let result = registry.unregister(handle);
        assert!(matches!(
            result,
            Ok(UnregisterOutcome::Deferred {
                until_after_frame: 5
            })
        ));
        // Marked for deferred removal, but still resolvable: its resources are
        // still needed until the backend catches up processing it.
        assert!(registry.descriptor(handle).is_some());

        registry.drain_pending_removals();
        // Not processed yet, so it must still be alive.
        assert!(registry.descriptor(handle).is_some());

        registry.mark_processed(handle);
        registry.drain_pending_removals();
        assert!(registry.descriptor(handle).is_none());
    }

    #[test]
    fn notready_outcome_still_drains_deferred_removal() {
        // A slot pending removal isn't only unblocked by a `Ready` compose: the
        // backend calls `mark_processed` for every outcome (NotReady, ContextLost,
        // a stale context generation, a downcast failure, ...), since in every
        // case there is nothing left in flight for that slot this frame.
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), dummy_compositor())
            .unwrap();

        registry.mark_painted(handle, 1);
        assert!(matches!(
            registry.unregister(handle),
            Ok(UnregisterOutcome::Deferred {
                until_after_frame: 1
            })
        ));

        registry.mark_processed(handle);
        registry.drain_pending_removals();
        assert!(registry.descriptor(handle).is_none());
    }

    #[test]
    fn pending_removal_slot_is_invalid_and_rejects_further_paints() {
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), dummy_compositor())
            .unwrap();

        registry.mark_painted(handle, 1);
        assert!(matches!(
            registry.unregister(handle),
            Ok(UnregisterOutcome::Deferred {
                until_after_frame: 1
            })
        ));
        assert!(!registry.is_valid(handle));

        // A stray paint on an already-pending-removal slot (e.g. a caller that
        // hasn't noticed `is_valid` went false yet) must not extend the zombie's
        // life by pushing `last_painted_frame` forward. Observe this indirectly
        // through `unregister`, which reports the slot's current
        // `last_painted_frame` as `until_after_frame`: if the stray paint had taken
        // effect, this would now report `99` instead of the original `1`.
        registry.mark_painted(handle, 99);
        assert!(matches!(
            registry.unregister(handle),
            Ok(UnregisterOutcome::Deferred {
                until_after_frame: 1
            })
        ));
    }

    #[test]
    fn unregister_without_frame_in_flight_frees_immediately() {
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), dummy_compositor())
            .unwrap();
        assert_eq!(
            registry.unregister(handle).unwrap(),
            UnregisterOutcome::Removed
        );
        assert!(registry.descriptor(handle).is_none());
    }

    #[test]
    fn unregister_unknown_handle_is_rejected() {
        let mut registry = ExternalCompositorRegistry::new();
        assert!(matches!(
            registry.unregister(ExternalSlotHandle {
                index: 0,
                generation: 1
            }),
            Err(ExternalCompositorError::StaleHandle)
        ));
    }

    #[test]
    fn take_and_put_back_compositor() {
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), Box::new(42_u32) as Box<dyn Any>)
            .unwrap();

        let mut compositor = registry
            .take_compositor(handle)
            .expect("compositor should be present");
        assert!(registry.take_compositor(handle).is_none());
        assert_eq!(compositor.downcast_mut::<u32>(), Some(&mut 42));

        registry.put_back_compositor(handle, compositor);
        assert!(registry.take_compositor(handle).is_some());
    }

    #[test]
    fn slot_size_reports_device_texels() {
        let mut registry = ExternalCompositorRegistry::new();
        let handle = registry
            .register(descriptor(1), dummy_compositor())
            .unwrap();
        let size = registry.slot_size(handle).unwrap();
        assert_eq!(size.width, DevicePixels(64));
        assert_eq!(size.height, DevicePixels(64));
    }
}

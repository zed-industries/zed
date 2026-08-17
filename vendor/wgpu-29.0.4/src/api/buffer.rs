use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{
    error, fmt,
    ops::{Bound, Deref, Range, RangeBounds},
};

use crate::util::Mutex;
use crate::*;

/// Handle to a GPU-accessible buffer.
///
/// A `Buffer` is a memory allocation for use by the GPU, somewhat analogous to
/// <code>[Box]&lt;[\[u8\]][primitive@slice]&gt;</code> in Rust.
/// The contents of buffers are untyped bytes; it is up to the application to
/// specify the interpretation of the bytes when the buffer is used, in ways
/// such as [`VertexBufferLayout`].
/// A single buffer can be used to hold multiple independent pieces of data at
/// different offsets (e.g. both vertices and indices for one or more meshes).
///
/// A `Buffer`'s bytes have "interior mutability": functions like
/// [`Queue::write_buffer`] or [mapping] a buffer for writing only require a
/// `&Buffer`, not a `&mut Buffer`, even though they modify its contents. `wgpu`
/// prevents simultaneous reads and writes of buffer contents using run-time
/// checks.
///
/// Created with [`Device::create_buffer()`] or
/// [`DeviceExt::create_buffer_init()`].
///
/// Corresponds to [WebGPU `GPUBuffer`](https://gpuweb.github.io/gpuweb/#buffer-interface).
///
/// [mapping]: Buffer#mapping-buffers
///
/// # How to get your data into a buffer
///
/// Every `Buffer` starts with all bytes zeroed.
/// There are many ways to load data into a `Buffer`:
///
/// - When creating a buffer, you may set the [`mapped_at_creation`][mac] flag,
///   then write to its [`get_mapped_range_mut()`][Buffer::get_mapped_range_mut].
///   This only works when the buffer is created and has not yet been used by
///   the GPU, but it is all you need for buffers whose contents do not change
///   after creation.
///   - You may use [`DeviceExt::create_buffer_init()`] as a convenient way to
///     do that and copy data from a `&[u8]` you provide.
/// - After creation, you may use [`Buffer::map_async()`] to map it again;
///   however, you then need to wait until the GPU is no longer using the buffer
///   before you begin writing.
/// - You may use [`CommandEncoder::copy_buffer_to_buffer()`] to copy data into
///   this buffer from another buffer.
/// - You may use [`Queue::write_buffer()`] to copy data into the buffer from a
///   `&[u8]`. This uses a temporary “staging” buffer managed by `wgpu` to hold
///   the data.
///   - [`Queue::write_buffer_with()`] allows you to write directly into temporary
///     storage instead of providing a slice you already prepared, which may
///     allow *your* code to save the allocation of a [`Vec`] or such.
/// - You may use [`util::StagingBelt`] to manage a set of temporary buffers.
///   This may be more efficient than [`Queue::write_buffer_with()`] when you
///   have many small copies to perform, but requires more steps to use, and
///   tuning of the belt buffer size.
/// - You may write your own staging buffer management customized to your
///   application, based on mapped buffers and
///   [`CommandEncoder::copy_buffer_to_buffer()`].
/// - A GPU computation’s results can be stored in a buffer:
///   - A [compute shader][ComputePipeline] may write to a buffer bound as a
///     [storage buffer][BufferBindingType::Storage].
///   - A render pass may render to a texture which is then copied to a buffer
///     using [`CommandEncoder::copy_texture_to_buffer()`].
///
/// # Mapping buffers
///
/// If a `Buffer` is created with the appropriate [`usage`], it can be *mapped*:
/// you can make its contents accessible to the CPU as an ordinary `&[u8]` or
/// `&mut [u8]` slice of bytes. Buffers created with the
/// [`mapped_at_creation`][mac] flag set are also mapped initially.
///
/// Depending on the hardware, the buffer could be memory shared between CPU and
/// GPU, so that the CPU has direct access to the same bytes the GPU will
/// consult; or it may be ordinary CPU memory, whose contents the system must
/// copy to/from the GPU as needed. This crate's API is designed to work the
/// same way in either case: at any given time, a buffer is either mapped and
/// available to the CPU, or unmapped and ready for use by the GPU, but never
/// both. This makes it impossible for either side to observe changes by the
/// other immediately, and any necessary transfers can be carried out when the
/// buffer transitions from one state to the other.
///
/// There are two ways to map a buffer:
///
/// - If [`BufferDescriptor::mapped_at_creation`] is `true`, then the entire
///   buffer is mapped when it is created. This is the easiest way to initialize
///   a new buffer. You can set `mapped_at_creation` on any kind of buffer,
///   regardless of its [`usage`] flags.
///
/// - If the buffer's [`usage`] includes the [`MAP_READ`] or [`MAP_WRITE`]
///   flags, then you can call `buffer.slice(range).map_async(mode, callback)`
///   to map the portion of `buffer` given by `range`. This waits for the GPU to
///   finish using the buffer, and invokes `callback` as soon as the buffer is
///   safe for the CPU to access.
///
/// Once a buffer is mapped:
///
/// - You can call `buffer.slice(range).get_mapped_range()` to obtain a
///   [`BufferView`], which dereferences to a `&[u8]` that you can use to read
///   the buffer's contents.
///
/// - Or, you can call `buffer.slice(range).get_mapped_range_mut()` to obtain a
///   [`BufferViewMut`], which dereferences to a `&mut [u8]` that you can use to
///   read and write the buffer's contents.
///
/// The given `range` must fall within the mapped portion of the buffer. If you
/// attempt to access overlapping ranges, even for shared access only, these
/// methods panic.
///
/// While a buffer is mapped, you may not submit any commands to the GPU that
/// access it. You may record command buffers that use the buffer, but if you
/// submit them while the buffer is mapped, submission will panic.
///
/// When you are done using the buffer on the CPU, you must call
/// [`Buffer::unmap`] to make it available for use by the GPU again. All
/// [`BufferView`] and [`BufferViewMut`] views referring to the buffer must be
/// dropped before you unmap it; otherwise, [`Buffer::unmap`] will panic.
///
/// # Example
///
/// If `buffer` was created with [`BufferUsages::MAP_WRITE`], we could fill it
/// with `f32` values like this:
///
/// ```
/// # #[cfg(feature = "noop")]
/// # let (device, _queue) = wgpu::Device::noop(&wgpu::DeviceDescriptor::default());
/// # #[cfg(not(feature = "noop"))]
/// # let device: wgpu::Device = { return; };
/// #
/// # let buffer = device.create_buffer(&wgpu::BufferDescriptor {
/// #     label: None,
/// #     size: 400,
/// #     usage: wgpu::BufferUsages::MAP_WRITE,
/// #     mapped_at_creation: false,
/// # });
/// let capturable = buffer.clone();
/// buffer.map_async(wgpu::MapMode::Write, .., move |result| {
///     if result.is_ok() {
///         let mut view = capturable.get_mapped_range_mut(..);
///         let mut floats: wgpu::WriteOnly<[[u8; 4]]> = view.slice(..).into_chunks::<4>().0;
///         floats.fill(42.0f32.to_ne_bytes());
///         drop(view);
///         capturable.unmap();
///     }
/// });
/// ```
///
/// This code takes the following steps:
///
/// - First, it makes a cloned handle to the buffer for capture by
///   the callback passed to [`map_async`]. Since a [`map_async`] callback may be
///   invoked from another thread, interaction between the callback and the
///   thread calling [`map_async`] generally requires some sort of shared heap
///   data like this. In real code, there might be an [`Arc`] to some larger
///   structure that itself owns `buffer`.
///
/// - Then, it calls [`Buffer::slice`] to make a [`BufferSlice`] referring to
///   the buffer's entire contents.
///
/// - Next, it calls [`BufferSlice::map_async`] to request that the bytes to
///   which the slice refers be made accessible to the CPU ("mapped"). This may
///   entail waiting for previously enqueued operations on `buffer` to finish.
///   Although [`map_async`] itself always returns immediately, it saves the
///   callback function to be invoked later.
///
/// - When some later call to [`Device::poll`] or [`Instance::poll_all`] (not
///   shown in this example) determines that the buffer is mapped and ready for
///   the CPU to use, it invokes the callback function.
///
/// - The callback function calls [`Buffer::slice`] and then
///   [`BufferSlice::get_mapped_range_mut`] to obtain a [`BufferViewMut`], which
///   dereferences to a `&mut [u8]` slice referring to the buffer's bytes.
///
/// - It then uses the [`bytemuck`] crate to turn the `&mut [u8]` into a `&mut
///   [f32]`, and calls the slice [`fill`] method to fill the buffer with a
///   useful value.
///
/// - Finally, the callback drops the view and calls [`Buffer::unmap`] to unmap
///   the buffer. In real code, the callback would also need to do some sort of
///   synchronization to let the rest of the program know that it has completed
///   its work.
///
/// If using [`map_async`] directly is awkward, you may find it more convenient to
/// use [`Queue::write_buffer`] and [`util::DownloadBuffer::read_buffer`].
/// However, those each have their own tradeoffs; the asynchronous nature of GPU
/// execution makes it hard to avoid friction altogether.
///
/// [`Arc`]: std::sync::Arc
/// [`map_async`]: BufferSlice::map_async
/// [`bytemuck`]: https://crates.io/crates/bytemuck
/// [`fill`]: slice::fill
///
/// ## Mapping buffers on the web
///
/// When compiled to WebAssembly and running in a browser content process,
/// `wgpu` implements its API in terms of the browser's WebGPU implementation.
/// In this context, `wgpu` is further isolated from the GPU:
///
/// - Depending on the browser's WebGPU implementation, mapping and unmapping
///   buffers probably entails copies between WebAssembly linear memory and the
///   graphics driver's buffers.
///
/// - All modern web browsers isolate web content in its own sandboxed process,
///   which can only interact with the GPU via interprocess communication (IPC).
///   Although most browsers' IPC systems use shared memory for large data
///   transfers, there will still probably need to be copies into and out of the
///   shared memory buffers.
///
/// All of these copies contribute to the cost of buffer mapping in this
/// configuration.
///
/// [`usage`]: BufferDescriptor::usage
/// [mac]: BufferDescriptor::mapped_at_creation
/// [`MAP_READ`]: BufferUsages::MAP_READ
/// [`MAP_WRITE`]: BufferUsages::MAP_WRITE
/// [`DeviceExt::create_buffer_init()`]: util::DeviceExt::create_buffer_init
#[derive(Debug, Clone)]
pub struct Buffer {
    pub(crate) inner: dispatch::DispatchBuffer,
    pub(crate) map_context: Arc<Mutex<MapContext>>,
    pub(crate) size: wgt::BufferAddress,
    pub(crate) usage: BufferUsages,
    // Todo: missing map_state https://www.w3.org/TR/webgpu/#dom-gpubuffer-mapstate
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(Buffer: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(Buffer => .inner);

impl Buffer {
    /// Return the binding view of the entire buffer.
    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        BindingResource::Buffer(self.as_entire_buffer_binding())
    }

    /// Return the binding view of the entire buffer.
    pub fn as_entire_buffer_binding(&self) -> BufferBinding<'_> {
        BufferBinding {
            buffer: self,
            offset: 0,
            size: None,
        }
    }

    /// Get the [`wgpu_hal`] buffer from this `Buffer`.
    ///
    /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
    /// and pass that struct to the to the `A` type parameter.
    ///
    /// Returns a guard that dereferences to the type of the hal backend
    /// which implements [`A::Buffer`].
    ///
    /// # Types
    ///
    /// The returned type depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("Buffer")]
    #[doc = crate::macros::hal_type_metal!("Buffer")]
    #[doc = crate::macros::hal_type_dx12!("Buffer")]
    #[doc = crate::macros::hal_type_gles!("Buffer")]
    ///
    /// # Deadlocks
    ///
    /// - The returned guard holds a read-lock on a device-local "destruction"
    ///   lock, which will cause all calls to `destroy` to block until the
    ///   guard is released.
    ///
    /// # Errors
    ///
    /// This method will return None if:
    /// - The buffer is not from the backend specified by `A`.
    /// - The buffer is from the `webgpu` or `custom` backend.
    /// - The buffer has had [`Self::destroy()`] called on it.
    ///
    /// # Safety
    ///
    /// - The returned resource must not be destroyed unless the guard
    ///   is the last reference to it and it is not in use by the GPU.
    ///   The guard and handle may be dropped at any time however.
    /// - All the safety requirements of wgpu-hal must be upheld.
    ///
    /// [`A::Buffer`]: hal::Api::Buffer
    #[cfg(wgpu_core)]
    pub unsafe fn as_hal<A: hal::Api>(
        &self,
    ) -> Option<impl Deref<Target = A::Buffer> + WasmNotSendSync> {
        let buffer = self.inner.as_core_opt()?;
        unsafe { buffer.context.buffer_as_hal::<A>(buffer) }
    }

    /// Returns a [`BufferSlice`] referring to the portion of `self`'s contents
    /// indicated by `bounds`. Regardless of what sort of data `self` stores,
    /// `bounds` start and end are given in bytes.
    ///
    /// A [`BufferSlice`] can be used to supply vertex and index data, or to map
    /// buffer contents for access from the CPU. See the [`BufferSlice`]
    /// documentation for details.
    ///
    /// The `range` argument can be half or fully unbounded: for example,
    /// `buffer.slice(..)` refers to the entire buffer, and `buffer.slice(n..)`
    /// refers to the portion starting at the `n`th byte and extending to the
    /// end of the buffer.
    ///
    /// # Panics
    ///
    /// - If `bounds` is outside of the bounds of `self`.
    /// - If `bounds` has a length less than 1.
    #[track_caller]
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'_> {
        let (offset, size) = range_to_offset_size(bounds, self.size);
        check_buffer_bounds(self.size, offset, size);
        BufferSlice {
            buffer: self,
            offset,
            size,
        }
    }

    /// Unmaps the buffer from host memory.
    ///
    /// This terminates the effect of all previous [`map_async()`](Self::map_async) operations and
    /// makes the buffer available for use by the GPU again.
    pub fn unmap(&self) {
        self.map_context.lock().reset();
        self.inner.unmap();
    }

    /// Destroy the associated native resources as soon as possible.
    pub fn destroy(&self) {
        self.inner.destroy();
    }

    /// Returns the length of the buffer allocation in bytes.
    ///
    /// This is always equal to the `size` that was specified when creating the buffer.
    pub fn size(&self) -> BufferAddress {
        self.size
    }

    /// Returns the allowed usages for this `Buffer`.
    ///
    /// This is always equal to the `usage` that was specified when creating the buffer.
    pub fn usage(&self) -> BufferUsages {
        self.usage
    }

    /// Map the buffer to host (CPU) memory, making it available for reading or writing via
    /// [`get_mapped_range()`](Self::get_mapped_range). The buffer becomes accessible once the
    /// `callback` is invoked with [`Ok`].
    ///
    /// Use this when you want to map the buffer immediately. If you need to submit GPU work that
    /// uses the buffer before mapping it, use `map_buffer_on_submit` on
    /// [`CommandEncoder`][CEmbos], [`CommandBuffer`][CBmbos], [`RenderPass`][RPmbos], or
    /// [`ComputePass`][CPmbos] to schedule the mapping after submission. This avoids extra calls to
    /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and lets you initiate mapping from a
    /// more convenient place.
    ///
    /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
    /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated into
    /// an event loop or run on a separate thread.
    ///
    /// The callback runs on the thread that first calls one of the above functions after the GPU work
    /// completes. There are no restrictions on the code you can run in the callback; however, on native
    /// the polling call will not return until the callback finishes, so keep callbacks short (set flags,
    /// send messages, etc.).
    ///
    /// While a buffer is mapped, it cannot be used by other commands; at any time, either the GPU or
    /// the CPU has exclusive access to the buffer’s contents.
    ///
    /// This can also be performed using [`BufferSlice::map_async()`].
    ///
    /// # Panics
    ///
    /// - If the buffer is already mapped.
    /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
    /// - If `bounds` is outside of the bounds of `self`.
    /// - If `bounds` does not start at a multiple of [`MAP_ALIGNMENT`].
    /// - If `bounds` has a length that is not a multiple of 4 greater than 0.
    ///
    /// [CEmbos]: CommandEncoder::map_buffer_on_submit
    /// [CBmbos]: CommandBuffer::map_buffer_on_submit
    /// [RPmbos]: RenderPass::map_buffer_on_submit
    /// [CPmbos]: ComputePass::map_buffer_on_submit
    /// [q::s]: Queue::submit
    /// [i::p_a]: Instance::poll_all
    /// [d::p]: Device::poll
    pub fn map_async<S: RangeBounds<BufferAddress>>(
        &self,
        mode: MapMode,
        bounds: S,
        callback: impl FnOnce(Result<(), BufferAsyncError>) + WasmNotSend + 'static,
    ) {
        self.slice(bounds).map_async(mode, callback)
    }

    /// Gain read-only access to the bytes of a [mapped] [`Buffer`].
    ///
    /// Returns a [`BufferView`] referring to the buffer range represented by
    /// `self`. See the documentation for [`BufferView`] for details.
    ///
    /// `bounds` may be less than the bounds passed to [`Self::map_async()`],
    /// and multiple views may be obtained and used simultaneously as long as they do not overlap.
    ///
    /// This can also be performed using [`BufferSlice::get_mapped_range()`].
    ///
    /// # Panics
    ///
    /// - If `bounds` is outside of the bounds of `self`.
    /// - If `bounds` does not start at a multiple of [`MAP_ALIGNMENT`].
    /// - If `bounds` has a length that is not a multiple of 4 greater than 0.
    /// - If the buffer to which `self` refers is not currently [mapped].
    /// - If you try to create a view which overlaps an existing [`BufferViewMut`].
    ///
    /// [mapped]: Buffer#mapping-buffers
    #[track_caller]
    pub fn get_mapped_range<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferView {
        self.slice(bounds).get_mapped_range()
    }

    /// Gain write access to the bytes of a [mapped] [`Buffer`].
    ///
    /// Returns a [`BufferViewMut`] referring to the buffer range represented by
    /// `self`. See the documentation for [`BufferViewMut`] for more details.
    ///
    /// `bounds` may be less than the bounds passed to [`Self::map_async()`],
    /// and multiple views may be obtained and used simultaneously as long as they do not overlap.
    ///
    /// This can also be performed using [`BufferSlice::get_mapped_range_mut()`].
    ///
    /// # Panics
    ///
    /// - If `bounds` is outside of the bounds of `self`.
    /// - If `bounds` does not start at a multiple of [`MAP_ALIGNMENT`].
    /// - If `bounds` has a length that is not a multiple of 4 greater than 0.
    /// - If the buffer to which `self` refers is not currently [mapped].
    /// - If you try to create a view which overlaps an existing [`BufferView`] or [`BufferViewMut`].
    ///
    /// [mapped]: Buffer#mapping-buffers
    #[track_caller]
    pub fn get_mapped_range_mut<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferViewMut {
        self.slice(bounds).get_mapped_range_mut()
    }

    #[cfg(custom)]
    /// Returns custom implementation of Buffer (if custom backend and is internally T)
    pub fn as_custom<T: custom::BufferInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// A slice of a [`Buffer`], to be mapped, used for vertex or index data, or the like.
///
/// You can create a `BufferSlice` by calling [`Buffer::slice`]:
///
/// ```no_run
/// # let buffer: wgpu::Buffer = todo!();
/// let slice = buffer.slice(10..20);
/// ```
///
/// This returns a slice referring to the second ten bytes of `buffer`. To get a
/// slice of the entire `Buffer`:
///
/// ```no_run
/// # let buffer: wgpu::Buffer = todo!();
/// let whole_buffer_slice = buffer.slice(..);
/// ```
///
/// You can pass buffer slices to methods like [`RenderPass::set_vertex_buffer`]
/// and [`RenderPass::set_index_buffer`] to indicate which portion of the buffer
/// a draw call should consult. You can also convert it to a [`BufferBinding`]
/// with `.into()`.
///
/// To access the slice's contents on the CPU, you must first [map] the buffer,
/// and then call [`BufferSlice::get_mapped_range`] or
/// [`BufferSlice::get_mapped_range_mut`] to obtain a view of the slice's
/// contents. See the documentation on [mapping][map] for more details,
/// including example code.
///
/// Unlike a Rust shared slice `&[T]`, whose existence guarantees that
/// nobody else is modifying the `T` values to which it refers, a
/// [`BufferSlice`] doesn't guarantee that the buffer's contents aren't
/// changing. You can still record and submit commands operating on the
/// buffer while holding a [`BufferSlice`]. A [`BufferSlice`] simply
/// represents a certain range of the buffer's bytes.
///
/// The `BufferSlice` type is unique to the Rust API of `wgpu`. In the WebGPU
/// specification, an offset and size are specified as arguments to each call
/// working with the [`Buffer`], instead.
///
/// [map]: Buffer#mapping-buffers
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BufferSlice<'a> {
    pub(crate) buffer: &'a Buffer,
    pub(crate) offset: BufferAddress,
    pub(crate) size: BufferSize,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(BufferSlice<'_>: Send, Sync);

impl<'a> BufferSlice<'a> {
    /// Return another [`BufferSlice`] referring to the portion of `self`'s contents
    /// indicated by `bounds`.
    ///
    /// The `range` argument can be half or fully unbounded: for example,
    /// `buffer.slice(..)` refers to the entire buffer, and `buffer.slice(n..)`
    /// refers to the portion starting at the `n`th byte and extending to the
    /// end of the buffer.
    ///
    /// # Panics
    ///
    /// - If `bounds` is outside of the bounds of `self`.
    /// - If `bounds` has a length less than 1.
    #[track_caller]
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'a> {
        let (offset, size) = range_to_offset_size(bounds, self.size.get());
        check_buffer_bounds(self.size.get(), offset, size);
        BufferSlice {
            buffer: self.buffer,
            offset: self.offset + offset, // check_buffer_bounds ensures this does not overflow
            size,                         // check_buffer_bounds ensures this is essentially min()
        }
    }

    /// Map the buffer to host (CPU) memory, making it available for reading or writing via
    /// [`get_mapped_range()`](Self::get_mapped_range). The buffer becomes accessible once the
    /// `callback` is invoked with [`Ok`].
    ///
    /// Use this when you want to map the buffer immediately. If you need to submit GPU work that
    /// uses the buffer before mapping it, use `map_buffer_on_submit` on
    /// [`CommandEncoder`][CEmbos], [`CommandBuffer`][CBmbos], [`RenderPass`][RPmbos], or
    /// [`ComputePass`][CPmbos] to schedule the mapping after submission. This avoids extra calls to
    /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and lets you initiate mapping from a
    /// more convenient place.
    ///
    /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
    /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated into
    /// an event loop or run on a separate thread.
    ///
    /// The callback runs on the thread that first calls one of the above functions after the GPU work
    /// completes. There are no restrictions on the code you can run in the callback; however, on native
    /// the polling call will not return until the callback finishes, so keep callbacks short (set flags,
    /// send messages, etc.).
    ///
    /// While a buffer is mapped, it cannot be used by other commands; at any time, either the GPU or
    /// the CPU has exclusive access to the buffer’s contents.
    ///
    /// This can also be performed using [`Buffer::map_async()`].
    ///
    /// # Panics
    ///
    /// - If the buffer is already mapped.
    /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
    /// - If the beginning of this slice is not aligned to [`MAP_ALIGNMENT`] within the buffer.
    /// - If the length of this slice is not a multiple of 4.
    ///
    /// [CEmbos]: CommandEncoder::map_buffer_on_submit
    /// [CBmbos]: CommandBuffer::map_buffer_on_submit
    /// [RPmbos]: RenderPass::map_buffer_on_submit
    /// [CPmbos]: ComputePass::map_buffer_on_submit
    /// [q::s]: Queue::submit
    /// [i::p_a]: Instance::poll_all
    /// [d::p]: Device::poll
    pub fn map_async(
        &self,
        mode: MapMode,
        callback: impl FnOnce(Result<(), BufferAsyncError>) + WasmNotSend + 'static,
    ) {
        let mut mc = self.buffer.map_context.lock();
        assert_eq!(mc.mapped_range, 0..0, "Buffer is already mapped");
        let end = self.offset + self.size.get();
        mc.mapped_range = self.offset..end;
        drop(mc); // release the lock of map_context as callback can call lock it again

        self.buffer
            .inner
            .map_async(mode, self.offset..end, Box::new(callback));
    }

    /// Gain read-only access to the bytes of a [mapped] [`Buffer`].
    ///
    /// Returns a [`BufferView`] referring to the buffer range represented by
    /// `self`. See the documentation for [`BufferView`] for details.
    ///
    /// Multiple views may be obtained and used simultaneously as long as they are from
    /// non-overlapping slices.
    ///
    /// This can also be performed using [`Buffer::get_mapped_range()`].
    ///
    /// # Panics
    ///
    /// - If the beginning of this slice is not aligned to [`MAP_ALIGNMENT`] within the buffer.
    /// - If the length of this slice is not a multiple of 4.
    /// - If the buffer to which `self` refers is not currently [mapped].
    /// - If you try to create a view which overlaps an existing [`BufferViewMut`].
    ///
    /// [mapped]: Buffer#mapping-buffers
    #[track_caller]
    pub fn get_mapped_range(&self) -> BufferView {
        let subrange = Subrange::new(self.offset, self.size, RangeMappingKind::Immutable);
        self.buffer
            .map_context
            .lock()
            .validate_and_add(subrange.clone());
        let range = self.buffer.inner.get_mapped_range(subrange.index);
        BufferView {
            buffer: self.buffer.clone(),
            size: self.size,
            offset: self.offset,
            inner: range,
        }
    }

    /// Gain write-only access to the bytes of a [mapped] [`Buffer`].
    ///
    /// Returns a [`BufferViewMut`] referring to the buffer range represented by
    /// `self`. See the documentation for [`BufferViewMut`] for more details.
    ///
    /// Multiple views may be obtained and used simultaneously as long as they are from
    /// non-overlapping slices.
    ///
    /// This can also be performed using [`Buffer::get_mapped_range_mut()`].
    ///
    /// # Panics
    ///
    /// - If the beginning of this slice is not aligned to [`MAP_ALIGNMENT`] within the buffer.
    /// - If the length of this slice is not a multiple of 4.
    /// - If the buffer to which `self` refers is not currently [mapped].
    /// - If you try to create a view which overlaps an existing [`BufferView`] or [`BufferViewMut`].
    ///
    /// [mapped]: Buffer#mapping-buffers
    #[track_caller]
    pub fn get_mapped_range_mut(&self) -> BufferViewMut {
        let subrange = Subrange::new(self.offset, self.size, RangeMappingKind::Mutable);
        self.buffer
            .map_context
            .lock()
            .validate_and_add(subrange.clone());
        let range = self.buffer.inner.get_mapped_range(subrange.index);
        BufferViewMut {
            buffer: self.buffer.clone(),
            size: self.size,
            offset: self.offset,
            inner: range,
        }
    }

    /// Returns the buffer this is a slice of.
    ///
    /// You should usually not need to call this, and if you received the buffer from code you
    /// do not control, you should refrain from accessing the buffer outside the bounds of the
    /// slice. Nevertheless, it’s possible to get this access, so this method makes it simple.
    pub fn buffer(&self) -> &'a Buffer {
        self.buffer
    }

    /// Returns the offset in [`Self::buffer()`] this slice starts at.
    pub fn offset(&self) -> BufferAddress {
        self.offset
    }

    /// Returns the size of this slice.
    pub fn size(&self) -> BufferSize {
        self.size
    }
}

impl<'a> From<BufferSlice<'a>> for crate::BufferBinding<'a> {
    /// Convert a [`BufferSlice`] to an equivalent [`BufferBinding`],
    /// provided that it will be used without a dynamic offset.
    fn from(value: BufferSlice<'a>) -> Self {
        BufferBinding {
            buffer: value.buffer,
            offset: value.offset,
            size: Some(value.size),
        }
    }
}

impl<'a> From<BufferSlice<'a>> for crate::BindingResource<'a> {
    /// Convert a [`BufferSlice`] to an equivalent [`BindingResource::Buffer`],
    /// provided that it will be used without a dynamic offset.
    fn from(value: BufferSlice<'a>) -> Self {
        crate::BindingResource::Buffer(crate::BufferBinding::from(value))
    }
}

fn range_overlaps(a: &Range<BufferAddress>, b: &Range<BufferAddress>) -> bool {
    a.start < b.end && b.start < a.end
}

fn range_contains(a: &Range<BufferAddress>, b: &Range<BufferAddress>) -> bool {
    a.start <= b.start && a.end >= b.end
}

#[derive(Debug, Copy, Clone)]
enum RangeMappingKind {
    Mutable,
    Immutable,
}

impl RangeMappingKind {
    /// Returns true if a range of this kind can touch the same bytes as a range of the other kind.
    ///
    /// This is Rust's Mutable XOR Shared rule.
    fn allowed_concurrently_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (RangeMappingKind::Immutable, RangeMappingKind::Immutable)
        )
    }
}

#[derive(Debug, Clone)]
struct Subrange {
    index: Range<BufferAddress>,
    kind: RangeMappingKind,
}

impl Subrange {
    fn new(offset: BufferAddress, size: BufferSize, kind: RangeMappingKind) -> Self {
        Self {
            index: offset..(offset + size.get()),
            kind,
        }
    }
}

impl fmt::Display for Subrange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}..{} ({:?})",
            self.index.start, self.index.end, self.kind
        )
    }
}

/// The mapped portion of a buffer, if any, and its outstanding views.
///
/// This ensures that views fall within the mapped range and don't overlap.
#[derive(Debug)]
pub(crate) struct MapContext {
    /// The range of the buffer that is mapped.
    ///
    /// This is `0..0` if the buffer is not mapped. This becomes non-empty when
    /// the buffer is mapped at creation time, and when you call `map_async` on
    /// some [`BufferSlice`] (so technically, it indicates the portion that is
    /// *or has been requested to be* mapped.)
    ///
    /// All [`BufferView`]s and [`BufferViewMut`]s must fall within this range.
    mapped_range: Range<BufferAddress>,

    /// The ranges covered by all outstanding [`BufferView`]s and
    /// [`BufferViewMut`]s. These are non-overlapping, and are all contained
    /// within `mapped_range`.
    sub_ranges: Vec<Subrange>,
}

impl MapContext {
    /// Creates a new `MapContext`.
    ///
    /// For [`mapped_at_creation`] buffers, pass the full buffer range in the
    /// `mapped_range` argument. For other buffers, pass `None`.
    ///
    /// [`mapped_at_creation`]: BufferDescriptor::mapped_at_creation
    pub(crate) fn new(mapped_range: Option<Range<BufferAddress>>) -> Self {
        Self {
            mapped_range: mapped_range.unwrap_or(0..0),
            sub_ranges: Vec::new(),
        }
    }

    /// Record that the buffer is no longer mapped.
    fn reset(&mut self) {
        self.mapped_range = 0..0;

        assert!(
            self.sub_ranges.is_empty(),
            "You cannot unmap a buffer that still has accessible mapped views"
        );
    }

    /// Record that the `size` bytes of the buffer at `offset` are now viewed.
    ///
    /// # Panics
    ///
    /// This panics if the given range is invalid.
    #[track_caller]
    fn validate_and_add(&mut self, new_sub: Subrange) {
        if self.mapped_range.is_empty() {
            panic!("tried to call get_mapped_range(_mut) on an unmapped buffer");
        }
        if !range_contains(&self.mapped_range, &new_sub.index) {
            panic!(
                "tried to call get_mapped_range(_mut) on a range that is not entirely mapped. \
                 Attempted to get range {}, but the mapped range is {}..{}",
                new_sub, self.mapped_range.start, self.mapped_range.end
            );
        }

        // This check is essential for avoiding undefined behavior: it is the
        // only thing that ensures that `&mut` references to the buffer's
        // contents don't alias anything else.
        for sub in self.sub_ranges.iter() {
            if range_overlaps(&sub.index, &new_sub.index)
                && !sub.kind.allowed_concurrently_with(new_sub.kind)
            {
                panic!(
                    "tried to call get_mapped_range(_mut) on a range that has already \
                     been mapped and would break Rust memory aliasing rules. Attempted \
                     to get range {}, and the conflicting range is {}",
                    new_sub, sub
                );
            }
        }
        self.sub_ranges.push(new_sub);
    }

    /// Record that the `size` bytes of the buffer at `offset` are no longer viewed.
    ///
    /// # Panics
    ///
    /// This panics if the given range does not exactly match one previously
    /// passed to [`MapContext::validate_and_add`].
    pub(crate) fn remove(&mut self, offset: BufferAddress, size: BufferSize) {
        let end = offset + size.get();

        let index = self
            .sub_ranges
            .iter()
            .position(|r| r.index == (offset..end))
            .expect("unable to remove range from map context");
        self.sub_ranges.swap_remove(index);
    }
}

/// Describes a [`Buffer`].
///
/// For use with [`Device::create_buffer`].
///
/// Corresponds to [WebGPU `GPUBufferDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpubufferdescriptor).
pub type BufferDescriptor<'a> = wgt::BufferDescriptor<Label<'a>>;
static_assertions::assert_impl_all!(BufferDescriptor<'_>: Send, Sync);

/// Error occurred when trying to async map a buffer.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BufferAsyncError;
static_assertions::assert_impl_all!(BufferAsyncError: Send, Sync);

impl fmt::Display for BufferAsyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occurred when trying to async map a buffer")
    }
}

impl error::Error for BufferAsyncError {}

/// Type of buffer mapping.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MapMode {
    /// Map only for reading
    Read,
    /// Map only for writing
    Write,
}
static_assertions::assert_impl_all!(MapMode: Send, Sync);

/// A read-only view of a mapped buffer's bytes.
///
/// To get a `BufferView`, first [map] the buffer, and then
/// call `buffer.slice(range).get_mapped_range()`.
///
/// `BufferView` dereferences to `&[u8]`, so you can use all the usual Rust
/// slice methods to access the buffer's contents. It also implements
/// `AsRef<[u8]>`, if that's more convenient.
///
/// Before the buffer can be unmapped, all `BufferView`s observing it
/// must be dropped. Otherwise, the call to [`Buffer::unmap`] will panic.
///
/// For example code, see the documentation on [mapping buffers][map].
///
/// [map]: Buffer#mapping-buffers
/// [`map_async`]: BufferSlice::map_async
#[derive(Debug)]
pub struct BufferView {
    // `buffer, offset, size` are similar to `BufferSlice`, except that they own the buffer.
    buffer: Buffer,
    offset: BufferAddress,
    size: BufferSize,
    inner: dispatch::DispatchBufferMappedRange,
}

/// A write-only view of a mapped buffer's bytes.
///
/// To get a `BufferViewMut`, first [map] the buffer, and then
/// call `buffer.slice(range).get_mapped_range_mut()`.
///
/// Because Rust has no write-only reference type
/// (`&[u8]` is read-only and `&mut [u8]` is read-write),
/// this type does not dereference to a slice in the way that [`BufferView`] does.
/// Instead, [`.slice()`][BufferViewMut::slice] returns a special [`WriteOnly`] pointer type,
/// and there are also a few convenience methods such as [`BufferViewMut::copy_from_slice()`].
///
/// Before the buffer can be unmapped, all `BufferViewMut`s observing it
/// must be dropped. Otherwise, the call to [`Buffer::unmap`] will panic.
///
/// For example code, see the documentation on [mapping buffers][map].
///
/// [map]: Buffer#mapping-buffers
#[derive(Debug)]
pub struct BufferViewMut {
    // `buffer, offset, size` are similar to `BufferSlice`, except that they own the buffer.
    buffer: Buffer,
    offset: BufferAddress,
    size: BufferSize,
    inner: dispatch::DispatchBufferMappedRange,
}

// `BufferView` simply dereferences. `BufferViewMut` cannot, because mapped memory may be
// write-combining memory <https://en.wikipedia.org/wiki/Write_combining>,
// and not support the expected behavior of atomic accesses.
// Further context: <https://github.com/gfx-rs/wgpu/issues/8897>

impl core::ops::Deref for BufferView {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        // SAFETY: this is a read mapping
        unsafe { self.inner.read_slice() }
    }
}

impl AsRef<[u8]> for BufferView {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl Drop for BufferView {
    fn drop(&mut self) {
        self.buffer
            .map_context
            .lock()
            .remove(self.offset, self.size);
    }
}

impl Drop for BufferViewMut {
    fn drop(&mut self) {
        self.buffer
            .map_context
            .lock()
            .remove(self.offset, self.size);
    }
}

#[cfg(webgpu)]
impl BufferView {
    /// Provides the same data as dereferencing the view, but as a `Uint8Array` in js.
    /// This can be MUCH faster than dereferencing the view which copies the data into
    /// the Rust / wasm heap.
    pub fn as_uint8array(&self) -> &js_sys::Uint8Array {
        self.inner.as_uint8array()
    }
}

/// These methods are equivalent to the methods of the same names on [`WriteOnly`].
impl BufferViewMut {
    /// Returns the length of this view; the number of bytes to be written.
    pub fn len(&self) -> usize {
        // cannot fail because we can't actually map more than isize::MAX bytes
        usize::try_from(self.size.get()).unwrap()
    }

    /// Returns `true` if the view has a length of 0.
    ///
    /// Note that this is currently impossible.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a [`WriteOnly`] reference to a portion of this.
    ///
    /// `.slice(..)` can be used to access the whole data.
    pub fn slice<'a, S: RangeBounds<usize>>(&'a mut self, bounds: S) -> WriteOnly<'a, [u8]> {
        // SAFETY: this is a write mapping
        unsafe { self.inner.write_slice() }.into_slice(bounds)
    }

    /// Copies all elements from src into `self`.
    ///
    /// The length of `src` must be the same as `self`.
    ///
    /// This method is equivalent to
    /// [`self.slice(..).copy_from_slice(src)`][WriteOnly::copy_from_slice].
    pub fn copy_from_slice(&mut self, src: &[u8]) {
        self.slice(..).copy_from_slice(src)
    }
}

#[track_caller]
fn check_buffer_bounds(
    buffer_size: BufferAddress,
    slice_offset: BufferAddress,
    slice_size: BufferSize,
) {
    // A slice of length 0 is invalid, so the offset must not be equal to or greater than the buffer size.
    if slice_offset >= buffer_size {
        panic!(
            "slice offset {} is out of range for buffer of size {}",
            slice_offset, buffer_size
        );
    }

    // Detect integer overflow.
    let end = slice_offset.checked_add(slice_size.get());
    if end.is_none_or(|end| end > buffer_size) {
        panic!(
            "slice offset {} size {} is out of range for buffer of size {}",
            slice_offset, slice_size, buffer_size
        );
    }
}

#[track_caller]
pub(crate) fn range_to_offset_size<S: RangeBounds<BufferAddress>>(
    bounds: S,
    whole_size: BufferAddress,
) -> (BufferAddress, BufferSize) {
    let offset = match bounds.start_bound() {
        Bound::Included(&bound) => bound,
        Bound::Excluded(&bound) => bound + 1,
        Bound::Unbounded => 0,
    };
    let size = BufferSize::new(match bounds.end_bound() {
        Bound::Included(&bound) => bound + 1 - offset,
        Bound::Excluded(&bound) => bound - offset,
        Bound::Unbounded => whole_size - offset,
    })
    .expect("buffer slices can not be empty");

    (offset, size)
}

#[cfg(test)]
mod tests {
    use super::{
        check_buffer_bounds, range_overlaps, range_to_offset_size, BufferAddress, BufferSize,
    };

    fn bs(value: BufferAddress) -> BufferSize {
        BufferSize::new(value).unwrap()
    }

    #[test]
    fn range_to_offset_size_works() {
        let whole = 100;

        assert_eq!(range_to_offset_size(0..2, whole), (0, bs(2)));
        assert_eq!(range_to_offset_size(2..5, whole), (2, bs(3)));
        assert_eq!(range_to_offset_size(.., whole), (0, bs(whole)));
        assert_eq!(range_to_offset_size(21.., whole), (21, bs(whole - 21)));
        assert_eq!(range_to_offset_size(0.., whole), (0, bs(whole)));
        assert_eq!(range_to_offset_size(..21, whole), (0, bs(21)));
    }

    #[test]
    #[should_panic = "buffer slices can not be empty"]
    fn range_to_offset_size_panics_for_empty_range() {
        range_to_offset_size(123..123, 200);
    }

    #[test]
    #[should_panic = "buffer slices can not be empty"]
    fn range_to_offset_size_panics_for_unbounded_empty_range() {
        range_to_offset_size(..0, 100);
    }

    #[test]
    fn check_buffer_bounds_works_for_end_in_range() {
        check_buffer_bounds(200, 100, bs(50));
        check_buffer_bounds(200, 100, bs(100));
        check_buffer_bounds(u64::MAX, u64::MAX - 100, bs(100));
        check_buffer_bounds(u64::MAX, 0, bs(u64::MAX));
        check_buffer_bounds(u64::MAX, 1, bs(u64::MAX - 1));
    }

    #[test]
    #[should_panic]
    fn check_buffer_bounds_panics_for_end_over_size() {
        check_buffer_bounds(200, 100, bs(101));
    }

    #[test]
    #[should_panic]
    fn check_buffer_bounds_panics_for_end_wraparound() {
        check_buffer_bounds(u64::MAX, 1, bs(u64::MAX));
    }

    #[test]
    fn range_overlapping() {
        // First range to the left
        assert_eq!(range_overlaps(&(0..1), &(1..3)), false);
        // First range overlaps left edge
        assert_eq!(range_overlaps(&(0..2), &(1..3)), true);
        // First range completely inside second
        assert_eq!(range_overlaps(&(1..2), &(0..3)), true);
        // First range completely surrounds second
        assert_eq!(range_overlaps(&(0..3), &(1..2)), true);
        // First range overlaps right edge
        assert_eq!(range_overlaps(&(1..3), &(0..2)), true);
        // First range entirely to the right
        assert_eq!(range_overlaps(&(2..3), &(0..2)), false);
    }
}

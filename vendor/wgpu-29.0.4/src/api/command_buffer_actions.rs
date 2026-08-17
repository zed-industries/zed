use alloc::{sync::Arc, vec::Vec};
use core::num::NonZeroU64;

use crate::{util::Mutex, *};

/// A deferred buffer mapping request captured during encoding (or a pass)
/// and executed later when the command buffer is submitted.
pub(crate) struct DeferredBufferMapping {
    pub buffer: api::Buffer,
    pub mode: MapMode,
    pub offset: u64,
    pub size: NonZeroU64,
    pub callback: dispatch::BufferMapCallback,
}

pub(super) type SharedDeferredCommandBufferActions = Arc<Mutex<DeferredCommandBufferActions>>;

/// Set of actions to take when the command buffer is submitted.
#[derive(Default)]
pub(crate) struct DeferredCommandBufferActions {
    pub buffer_mappings: Vec<DeferredBufferMapping>,
    pub on_submitted_work_done_callbacks: Vec<dispatch::BoxSubmittedWorkDoneCallback>,
}

impl DeferredCommandBufferActions {
    pub fn append(&mut self, other: &mut Self) {
        self.buffer_mappings.append(&mut other.buffer_mappings);
        self.on_submitted_work_done_callbacks
            .append(&mut other.on_submitted_work_done_callbacks);
    }

    pub fn execute(self, queue: &dispatch::DispatchQueue) {
        for mapping in self.buffer_mappings {
            mapping.buffer.map_async(
                mapping.mode,
                mapping.offset..mapping.offset + mapping.size.get(),
                mapping.callback,
            );
        }
        for callback in self.on_submitted_work_done_callbacks {
            queue.on_submitted_work_done(callback);
        }
    }
}

impl core::fmt::Debug for DeferredCommandBufferActions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DeferredCommandBufferActions")
            .field("buffer_mappings.len()", &self.buffer_mappings.len())
            .field(
                "on_submitted_work_done_callbacks.len()",
                &self.on_submitted_work_done_callbacks.len(),
            )
            .finish()
    }
}

// We can't just implement this on CommandEncoders as by default passes make it so that
// you can't call any commands on the encoder while this is happening. As such, we need
// to implement these methods on the passes too. Use a macro to avoid massive code duplication
macro_rules! impl_deferred_command_buffer_actions {
    () => {
        /// On submission, maps the buffer to host (CPU) memory, making it available
        /// for reading or writing via [`get_mapped_range()`](Buffer::get_mapped_range).
        /// The buffer becomes accessible once the `callback` is invoked with [`Ok`].
        ///
        /// Use this when you need to submit work that uses the buffer before mapping it.
        /// Because that submission must happen before calling `map_async`, this method
        /// schedules the mapping for after submission, avoiding extra calls to
        /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and letting you start
        /// the mapping from a more convenient place.
        ///
        /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
        /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
        /// into an event loop or run on a separate thread.
        ///
        /// The callback runs on the thread that first calls one of the above functions
        /// after the GPU work completes. There are no restrictions on the code you can run
        /// in the callback; however, on native the polling call will not return until the
        /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
        ///
        /// While a buffer is mapped, it cannot be used by other commands; at any time,
        /// either the GPU or the CPU has exclusive access to the buffer’s contents.
        ///
        /// # Panics
        ///
        /// - If `bounds` is outside the bounds of `buffer`.
        /// - If `bounds` has a length less than 1.
        ///
        /// # Panics During Submit
        ///
        /// - If the buffer is already mapped.
        /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
        /// - If `bounds` is outside of the bounds of `buffer`.
        /// - If `bounds` does not start at a multiple of [`MAP_ALIGNMENT`].
        /// - If `bounds` has a length that is not a multiple of 4 greater than 0.
        ///
        /// [q::s]: Queue::submit
        /// [i::p_a]: Instance::poll_all
        /// [d::p]: Device::poll
        /// [CEmbos]: CommandEncoder::map_buffer_on_submit
        /// [CBmbos]: CommandBuffer::map_buffer_on_submit
        /// [RPmbos]: RenderPass::map_buffer_on_submit
        /// [CPmbos]: ComputePass::map_buffer_on_submit
        pub fn map_buffer_on_submit<S: core::ops::RangeBounds<BufferAddress>>(
            &self,
            buffer: &api::Buffer,
            mode: MapMode,
            bounds: S,
            callback: impl FnOnce(Result<(), BufferAsyncError>) + WasmNotSend + 'static,
        ) {
            let (offset, size) = range_to_offset_size(bounds, buffer.size);
            self.actions.lock().buffer_mappings.push(
                crate::api::command_buffer_actions::DeferredBufferMapping {
                    buffer: buffer.clone(),
                    mode,
                    offset,
                    size,
                    callback: alloc::boxed::Box::new(callback),
                },
            );
        }

        /// Registers a callback that is invoked when this command buffer’s work finishes
        /// executing on the GPU. When this callback runs, all mapped-buffer callbacks
        /// registered for the same submission are guaranteed to have been called.
        ///
        /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
        /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
        /// into an event loop or run on a separate thread.
        ///
        /// The callback runs on the thread that first calls one of the above functions
        /// after the GPU work completes. There are no restrictions on the code you can run
        /// in the callback; however, on native the polling call will not return until the
        /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
        ///
        /// [q::s]: Queue::submit
        /// [i::p_a]: Instance::poll_all
        /// [d::p]: Device::poll
        pub fn on_submitted_work_done(&self, callback: impl FnOnce() + Send + 'static) {
            self.actions
                .lock()
                .on_submitted_work_done_callbacks
                .push(alloc::boxed::Box::new(callback));
        }
    };
}

pub(crate) use impl_deferred_command_buffer_actions;

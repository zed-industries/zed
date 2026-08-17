use crate::{
    api::{impl_deferred_command_buffer_actions, SharedDeferredCommandBufferActions},
    *,
};

/// Handle to a command buffer on the GPU.
///
/// A `CommandBuffer` represents a complete sequence of commands that may be submitted to a command
/// queue with [`Queue::submit`]. A `CommandBuffer` is obtained by recording a series of commands to
/// a [`CommandEncoder`] and then calling [`CommandEncoder::finish`].
///
/// Corresponds to [WebGPU `GPUCommandBuffer`](https://gpuweb.github.io/gpuweb/#command-buffer).
#[derive(Debug)]
pub struct CommandBuffer {
    pub(crate) buffer: dispatch::DispatchCommandBuffer,
    /// Deferred actions recorded at encode time, to run at Queue::submit.
    pub(crate) actions: SharedDeferredCommandBufferActions,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(CommandBuffer: Send, Sync);

impl CommandBuffer {
    #[cfg(custom)]
    /// Returns custom implementation of CommandBuffer (if custom backend and is internally T)
    pub fn as_custom<T: custom::CommandBufferInterface>(&self) -> Option<&T> {
        self.buffer.as_custom()
    }

    // Expose map_buffer_on_submit/on_submitted_work_done on CommandBuffer as well,
    // so callers can schedule after finishing encoding.
    impl_deferred_command_buffer_actions!();
}

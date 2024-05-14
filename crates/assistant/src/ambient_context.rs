mod recent_buffers;

pub use recent_buffers::*;

#[derive(Default)]
pub struct AmbientContext {
    pub recent_buffers: RecentBuffersContext,
}

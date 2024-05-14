mod current_project;
mod recent_buffers;

pub use current_project::*;
pub use recent_buffers::*;

#[derive(Default)]
pub struct AmbientContext {
    pub recent_buffers: RecentBuffersContext,
    pub current_project: CurrentProjectContext,
}

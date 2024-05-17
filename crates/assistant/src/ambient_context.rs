mod current_project;
mod recent_buffers;

pub use current_project::*;
pub use recent_buffers::*;

#[derive(Default)]
pub struct AmbientContext {
    pub recent_buffers: RecentBuffersContext,
    pub current_project: CurrentProjectContext,
}

impl AmbientContext {
    pub fn snapshot(&self) -> AmbientContextSnapshot {
        AmbientContextSnapshot {
            recent_buffers: self.recent_buffers.snapshot.clone(),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct AmbientContextSnapshot {
    pub recent_buffers: RecentBuffersSnapshot,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ContextUpdated {
    Updating,
    Disabled,
}

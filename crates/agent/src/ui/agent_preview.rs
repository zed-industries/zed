use collections::HashMap;
use component::ComponentId;
use gpui::{App, Entity, WeakEntity};
use linkme::distributed_slice;
use parking_lot::RwLock;
use ui::{AnyElement, Component, Window};
use workspace::Workspace;

use crate::{ActiveThread, ThreadStore};

/// Function type for creating agent component previews
pub type PreviewFn = fn(
    WeakEntity<Workspace>,
    Entity<ActiveThread>,
    WeakEntity<ThreadStore>,
    &mut Window,
    &mut App,
) -> Option<AnyElement>;

/// Structure to hold component ID and its preview function
pub struct PreviewEntry {
    pub id: ComponentId,
    pub preview_fn: PreviewFn,
}

/// Distributed slice that holds all agent preview data
#[distributed_slice]
pub static __ALL_AGENT_PREVIEWS: [PreviewEntry] = [..];

/// Trait that must be implemented by components that provide agent previews.
pub trait AgentPreview: Component {
    /// Static method to create a preview for this component type
    fn create_preview(
        workspace: WeakEntity<Workspace>,
        active_thread: Entity<ActiveThread>,
        thread_store: WeakEntity<ThreadStore>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement>
    where
        Self: Sized;
}

/// Register an agent preview for the given component type
#[macro_export]
macro_rules! register_agent_preview {
    ($type:ty) => {
        #[linkme::distributed_slice($crate::ui::agent_preview::__ALL_AGENT_PREVIEWS)]
        static __REGISTER_AGENT_PREVIEW: $crate::ui::agent_preview::PreviewEntry =
            $crate::ui::agent_preview::PreviewEntry {
                id: component::ComponentId(<$type>::name()),
                preview_fn: <$type as $crate::ui::agent_preview::AgentPreview>::create_preview,
            };
    };
}

/// Get a specific agent preview by component ID.
pub fn get_agent_preview(
    id: &ComponentId,
    workspace: WeakEntity<Workspace>,
    active_thread: Entity<ActiveThread>,
    thread_store: WeakEntity<ThreadStore>,
    window: &mut Window,
    cx: &mut App,
) -> Option<AnyElement> {
    __ALL_AGENT_PREVIEWS
        .iter()
        .find(|entry| &entry.id == id)
        .and_then(|entry| (entry.preview_fn)(workspace, active_thread, thread_store, window, cx))
}

/// Get all registered agent previews.
pub fn all_agent_previews() -> Vec<ComponentId> {
    __ALL_AGENT_PREVIEWS
        .iter()
        .map(|entry| entry.id.clone())
        .collect()
}

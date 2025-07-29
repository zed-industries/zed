use std::sync::OnceLock;

use collections::HashMap;
use component::ComponentId;
use gpui::{App, Entity, WeakEntity};
use ui::{AnyElement, Component, ComponentScope, Window};
use workspace::Workspace;

use crate::ActiveThread;

/// Function type for creating agent component previews
pub type PreviewFn =
    fn(WeakEntity<Workspace>, Entity<ActiveThread>, &mut Window, &mut App) -> Option<AnyElement>;

pub struct AgentPreviewFn(fn() -> (ComponentId, PreviewFn));

impl AgentPreviewFn {
    pub const fn new(f: fn() -> (ComponentId, PreviewFn)) -> Self {
        Self(f)
    }
}

inventory::collect!(AgentPreviewFn);

/// Trait that must be implemented by components that provide agent previews.
pub trait AgentPreview: Component + Sized {
    #[allow(unused)] // We can't know this is used due to the distributed slice
    fn scope(&self) -> ComponentScope {
        ComponentScope::Agent
    }

    /// Static method to create a preview for this component type
    fn agent_preview(
        workspace: WeakEntity<Workspace>,
        active_thread: Entity<ActiveThread>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement>;
}

/// Register an agent preview for the given component type
#[macro_export]
macro_rules! register_agent_preview {
    ($type:ty) => {
        inventory::submit! {
            $crate::ui::preview::AgentPreviewFn::new(|| {
                (
                    <$type as component::Component>::id(),
                    <$type as $crate::ui::preview::AgentPreview>::agent_preview,
                )
            })
        }
    };
}

/// Lazy initialized registry of preview functions
static AGENT_PREVIEW_REGISTRY: OnceLock<HashMap<ComponentId, PreviewFn>> = OnceLock::new();

/// Initialize the agent preview registry if needed
fn get_or_init_registry() -> &'static HashMap<ComponentId, PreviewFn> {
    AGENT_PREVIEW_REGISTRY.get_or_init(|| {
        let mut map = HashMap::default();
        for register_fn in inventory::iter::<AgentPreviewFn>() {
            let (id, preview_fn) = (register_fn.0)();
            map.insert(id, preview_fn);
        }
        map
    })
}

/// Get a specific agent preview by component ID.
pub fn get_agent_preview(
    id: &ComponentId,
    workspace: WeakEntity<Workspace>,
    active_thread: Entity<ActiveThread>,
    window: &mut Window,
    cx: &mut App,
) -> Option<AnyElement> {
    let registry = get_or_init_registry();
    registry
        .get(id)
        .and_then(|preview_fn| preview_fn(workspace, active_thread, window, cx))
}

/// Get all registered agent previews.
pub fn all_agent_previews() -> Vec<ComponentId> {
    let registry = get_or_init_registry();
    registry.keys().cloned().collect()
}

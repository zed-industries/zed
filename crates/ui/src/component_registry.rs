use crate::{prelude::*, traits::component::Component};
use gpui::{AnyElement, WindowContext};
use std::collections::HashMap;

/// The global component registry that tracks all registered components
pub struct ComponentRegistry {
    /// Map of scope -> (component_name -> component)
    components: HashMap<SharedString, HashMap<SharedString, Box<dyn Component>>>,
}

impl ComponentRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            components: HashMap::default(),
        }
    }

    /// Register a new component in the registry
    pub fn register<C: Component + 'static>(&mut self, component: C) {
        let scope = component.scope().into();
        let name = component.name().into();

        self.components
            .entry(scope)
            .or_default()
            .insert(name, Box::new(component));
    }

    /// Get all registered components and their previews
    pub fn get_all_component_previews(
        &self,
        cx: &WindowContext,
    ) -> HashMap<SharedString, Vec<(&str, AnyElement)>> {
        let mut previews = HashMap::new();

        for (scope, components) in &self.components {
            let mut scope_previews = Vec::new();

            for component in components.values() {
                if let Some(preview) = component.preview(cx) {
                    scope_previews.push((component.name(), preview));
                }
            }

            if !scope_previews.is_empty() {
                previews.insert(scope.clone(), scope_previews);
            }
        }

        previews
    }
}

// Make ComponentRegistry a singleton
lazy_static::lazy_static! {
    static ref REGISTRY: std::sync::Mutex<ComponentRegistry> = std::sync::Mutex::new(ComponentRegistry::new());
}

/// Register a component in the global registry
pub fn register_component<C: Component + 'static>(component: C) {
    REGISTRY.lock().unwrap().register(component);
}

/// Get a reference to the global component registry
pub fn get_registry() -> std::sync::MutexGuard<'static, ComponentRegistry> {
    REGISTRY.lock().unwrap()
}

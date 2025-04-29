use component::{Component, ComponentId, components};
use gpui::{AnyElement, AnyEntity, App, Entity, Window};
use std::any::TypeId;
use std::collections::HashMap;

/// Registry for storing and retrieving stateful component data
pub struct StatefulComponentRegistry {
    /// Component data is stored in `AnyEntity`ies due to cyclic references
    ///
    /// These can be accessed using the ComponentId as a key, and can then
    /// be downcasted to the appropriate type in the `component_preview` crate.
    entities: HashMap<ComponentId, AnyEntity>,

    types: HashMap<ComponentId, TypeId>,
}

impl StatefulComponentRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn stateful_component_ids(&self) -> Vec<ComponentId> {
        self.types.keys().cloned().collect()
    }

    /// Get an entity for a component, or create it if it doesn't exist
    pub fn get_or_create<T: 'static, F>(
        &mut self,
        component_id: &ComponentId,
        create_fn: F,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<T>
    where
        F: FnOnce(&mut Window, &mut App) -> Entity<T>,
    {
        if let Some(entity) = self.entities.get(component_id) {
            if let Ok(typed_entity) = entity.clone().downcast::<T>() {
                return typed_entity;
            }

            self.entities.remove(component_id);
        }

        let entity = create_fn(window, cx);

        self.entities
            .insert(component_id.clone(), entity.clone().into_any());
        self.types.insert(component_id.clone(), TypeId::of::<T>());

        entity
    }

    /// Get an entity if it exists
    pub fn get<T: 'static>(&self, component_id: &ComponentId) -> Option<Entity<T>> {
        self.entities
            .get(component_id)
            .and_then(|entity| entity.clone().downcast::<T>().ok())
    }

    /// Check if a component has an entity
    pub fn has_entity(&self, component_id: &ComponentId) -> bool {
        self.entities.contains_key(component_id)
    }

    /// Remove an entity
    pub fn remove(&mut self, component_id: &ComponentId) -> Option<AnyEntity> {
        let entity = self.entities.remove(component_id);
        if entity.is_some() {
            self.types.remove(component_id);
        }
        entity
    }

    /// Clear all entities
    pub fn clear(&mut self) {
        self.entities.clear();
        self.types.clear();
    }

    /// Get the number of stored entities
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

pub trait ComponentState: Component {
    /// The type of data stored for this component
    type Data: 'static;

    fn id() -> ComponentId {
        components()
            .id_by_name(Self::name())
            .expect("Couldn't get component ID")
    }

    /// Create the initial state data for this component
    fn data(window: &mut Window, cx: &mut App) -> Entity<Self::Data>;

    /// Render this component with its state data
    fn stateful_preview(
        data: Entity<Self::Data>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement>;

    fn __has_state() -> bool {
        true
    }

    /// Internal function to register the component's data with the
    /// [`StatefulComponentRegistry`].
    fn __register_data(
        state_registry: &mut StatefulComponentRegistry,
        window: &mut Window,
        cx: &mut App,
    ) {
        state_registry.get_or_create(&Self::id(), Self::data, window, cx);
    }
}

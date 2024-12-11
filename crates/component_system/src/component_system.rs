use collections::HashMap;
use gpui::{AnyElement, SharedString, WindowContext};
use linkme::distributed_slice;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub trait Component {
    fn scope() -> Option<&'static str>;
    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }
    fn description() -> Option<&'static str> {
        None
    }
}

pub trait ComponentPreview: Component {
    fn preview(_cx: &WindowContext) -> AnyElement;
}

#[distributed_slice]
pub static __ALL_COMPONENTS: [fn()] = [..];

#[distributed_slice]
pub static __ALL_PREVIEWS: [fn()] = [..];

pub static COMPONENT_DATA: Lazy<Mutex<ComponentData>> =
    Lazy::new(|| Mutex::new(ComponentData::new()));

pub struct ComponentData {
    components: Vec<(Option<&'static str>, &'static str, Option<&'static str>)>,
    previews: HashMap<&'static str, fn(&WindowContext) -> AnyElement>,
}

impl ComponentData {
    fn new() -> Self {
        ComponentData {
            components: Vec::new(),
            previews: HashMap::default(),
        }
    }
}

pub fn init() {
    let mut data = COMPONENT_DATA.lock().unwrap();
    for f in __ALL_COMPONENTS {
        f();
    }
    for f in __ALL_PREVIEWS {
        f();
    }
}

pub fn register_component<T: Component>() {
    let mut data = COMPONENT_DATA.lock().unwrap();
    data.components
        .push((T::scope(), T::name(), T::description()));
}

pub fn register_preview<T: ComponentPreview>() {
    let mut data = COMPONENT_DATA.lock().unwrap();
    data.previews.insert(T::name(), T::preview);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub &'static str);

pub struct ComponentMetadata {
    pub(crate) name: SharedString,
    pub(crate) scope: Option<SharedString>,
    pub(crate) description: Option<SharedString>,
    pub(crate) preview: Option<fn(&WindowContext) -> AnyElement>,
}

pub struct AllComponents(pub HashMap<ComponentId, ComponentMetadata>);

impl AllComponents {
    pub fn new() -> Self {
        AllComponents(HashMap::default())
    }

    pub fn add(&mut self, id: ComponentId, metadata: ComponentMetadata) {
        self.0.insert(id, metadata);
    }

    pub fn get(&self, id: &ComponentId) -> Option<&ComponentMetadata> {
        self.0.get(id)
    }

    /// Returns all components with previews
    pub fn all_previews(&self) -> Vec<&ComponentMetadata> {
        self.0.values().filter(|c| c.preview.is_some()).collect()
    }

    /// Returns all components
    pub fn all(&self) -> Vec<&ComponentMetadata> {
        self.0.values().collect()
    }
}

pub fn all_components() -> AllComponents {
    let data = COMPONENT_DATA.lock().unwrap();
    let mut all_components = AllComponents::new();

    for &(scope, name, description) in &data.components {
        let scope = scope.map(Into::into);
        let preview = data.previews.get(name).cloned();
        all_components.add(
            ComponentId(name),
            ComponentMetadata {
                name: name.into(),
                scope,
                description: description.map(Into::into),
                preview,
            },
        );
    }

    all_components
}

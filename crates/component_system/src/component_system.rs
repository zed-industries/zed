use collections::HashMap;
use gpui::{AnyElement, SharedString, WindowContext};
use linkme::distributed_slice;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

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

pub static COMPONENT_DATA: Lazy<RwLock<ComponentData>> =
    Lazy::new(|| RwLock::new(ComponentData::new()));

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
    let component_fns: Vec<_> = __ALL_COMPONENTS.iter().cloned().collect();
    let preview_fns: Vec<_> = __ALL_PREVIEWS.iter().cloned().collect();

    for f in component_fns {
        f();
    }
    for f in preview_fns {
        f();
    }
}

pub fn register_component<T: Component>() {
    let component_data = (T::scope(), T::name(), T::description());
    COMPONENT_DATA.write().components.push(component_data);
}

pub fn register_preview<T: ComponentPreview>() {
    let preview_data = (T::name(), T::preview as fn(&WindowContext) -> AnyElement);
    COMPONENT_DATA
        .write()
        .previews
        .insert(preview_data.0, preview_data.1);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub &'static str);

pub struct ComponentMetadata {
    name: SharedString,
    scope: Option<SharedString>,
    description: Option<SharedString>,
    preview: Option<fn(&WindowContext) -> AnyElement>,
}

impl ComponentMetadata {
    pub fn name(&self) -> SharedString {
        self.name.clone()
    }

    pub fn scope(&self) -> Option<SharedString> {
        self.scope.clone()
    }

    pub fn description(&self) -> Option<SharedString> {
        self.description.clone()
    }

    pub fn preview(&self) -> Option<fn(&WindowContext) -> AnyElement> {
        self.preview
    }
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

pub fn components() -> AllComponents {
    let data = COMPONENT_DATA.read();
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

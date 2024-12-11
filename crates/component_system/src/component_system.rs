use collections::HashMap;
use gpui::{AnyElement, SharedString, WindowContext};
use linkme::distributed_slice;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub trait Component {
    fn scope() -> &'static str;
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

pub static COMPONENTS: Lazy<Mutex<Vec<(&'static str, &'static str, Option<&'static str>)>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub static PREVIEWS: Lazy<Mutex<Vec<(&'static str, fn(&WindowContext) -> AnyElement)>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub fn init() {
    for f in __ALL_COMPONENTS {
        f();
    }
    for f in __ALL_PREVIEWS {
        f();
    }
}

fn components() -> Vec<(&'static str, &'static str, Option<&'static str>)> {
    COMPONENTS.lock().unwrap().clone()
}

fn component_previews() -> Vec<(&'static str, fn(&WindowContext) -> AnyElement)> {
    PREVIEWS.lock().unwrap().clone()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub &'static str);

pub struct ComponentMetadata {
    pub(crate) name: SharedString,
    pub(crate) scope: SharedString,
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
    let mut all_components = AllComponents::new();
    let components = components();
    let previews = component_previews();

    for (scope, name, description) in components {
        let preview = previews
            .iter()
            .find(|&(p_name, _)| *p_name == name)
            .map(|&(_, preview_fn)| preview_fn);
        all_components.add(
            ComponentId(name),
            ComponentMetadata {
                name: name.into(),
                scope: scope.into(),
                description: description.map(Into::into),
                preview,
            },
        );
    }

    all_components
}

pub fn register_preview<T: ComponentPreview>() {
    PREVIEWS
        .lock()
        .unwrap()
        .push((T::name(), |cx| T::preview(cx)));
}

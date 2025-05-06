use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;

use collections::HashMap;
use gpui::{AnyElement, App, SharedString, Window};
use linkme::distributed_slice;
use parking_lot::RwLock;

mod component_layout;

pub use component_layout::*;

/// Implement this trait to define a UI component. This will allow you to
/// derive `RegisterComponent` on it, in tutn allowing you to preview the
/// contents of the preview fn in `workspace: open component preview`.
///
/// This can be useful for visual debugging and testing, documenting UI
/// patterns, or simply showing all the variants of a component.
///
/// Generally you will want to implement at least `scope` and `preview`
/// from this trait, so you can preview the component, and it will show up
/// in a section that makes sense.
pub trait Component {
    /// The component's unique identifier.
    ///
    /// Used to access previews, or state for more
    /// complex, stateful components.
    fn id() -> ComponentId {
        ComponentId(Self::name())
    }
    /// Returns the scope of the component.
    ///
    /// This scope is used to determine how components and
    /// their previews are displayed and organized.
    fn scope() -> ComponentScope {
        ComponentScope::None
    }
    /// The name of the component.
    ///
    /// This name is used to identify the component
    /// and is usually derived from the component's type.
    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }
    /// Returns a name that the component should be sorted by.
    ///
    /// Implement this if the component should be sorted in an alternate order than its name.
    ///
    /// Example:
    ///
    /// For example, to group related components together when sorted:
    ///
    /// - Button      -> ButtonA
    /// - IconButton  -> ButtonBIcon
    /// - ToggleButton -> ButtonCToggle
    ///
    /// This naming scheme keeps these components together and allows them to /// be sorted in a logical order.
    fn sort_name() -> &'static str {
        Self::name()
    }
    /// An optional description of the component.
    ///
    /// This will be displayed in the component's preview. To show a
    /// component's doc comment as it's description, derive `Documented`.
    ///
    /// Example:
    ///
    /// ```
    /// /// This is a doc comment.
    /// #[derive(Documented)]
    /// struct MyComponent;
    ///
    /// impl MyComponent {
    ///     fn description() -> Option<&'static str> {
    ///         Some(Self::DOCS)
    ///     }
    /// }
    /// ```
    ///
    /// This will result in "This is a doc comment." being passed
    /// to the component's description.
    fn description() -> Option<&'static str> {
        None
    }
    /// The component's preview.
    ///
    /// An element returned here will be shown in the component's preview.
    ///
    /// Useful component helpers:
    /// - [`component::single_example`]
    /// - [`component::component_group`]
    /// - [`component::component_group_with_title`]
    ///
    /// Note: Any arbitrary element can be returned here.
    ///
    /// This is useful for displaying related UI to the component you are
    /// trying to preview, such as a button that opens a modal or shows a
    /// tooltip on hover, or a grid of icons showcasing all the icons available.
    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        None
    }
}

#[distributed_slice]
pub static __ALL_COMPONENTS: [fn()] = [..];

pub static COMPONENT_DATA: LazyLock<RwLock<ComponentRegistry>> =
    LazyLock::new(|| RwLock::new(ComponentRegistry::new()));

pub struct ComponentRegistry {
    components: Vec<(
        ComponentScope,
        // name
        &'static str,
        // sort name
        &'static str,
        // description
        Option<&'static str>,
    )>,
    previews: HashMap<&'static str, fn(&mut Window, &mut App) -> Option<AnyElement>>,
}

impl ComponentRegistry {
    fn new() -> Self {
        ComponentRegistry {
            components: Vec::new(),
            previews: HashMap::default(),
        }
    }
}

pub fn init() {
    let component_fns: Vec<_> = __ALL_COMPONENTS.iter().cloned().collect();
    for f in component_fns {
        f();
    }
}

pub fn register_component<T: Component>() {
    let component_data = (T::scope(), T::name(), T::sort_name(), T::description());
    let mut data = COMPONENT_DATA.write();
    data.components.push(component_data);
    data.previews.insert(T::id().0, T::preview);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub &'static str);

#[derive(Clone)]
pub struct ComponentMetadata {
    id: ComponentId,
    name: SharedString,
    sort_name: SharedString,
    scope: ComponentScope,
    description: Option<SharedString>,
    preview: Option<fn(&mut Window, &mut App) -> Option<AnyElement>>,
}

impl ComponentMetadata {
    pub fn id(&self) -> ComponentId {
        self.id.clone()
    }
    pub fn name(&self) -> SharedString {
        self.name.clone()
    }

    pub fn sort_name(&self) -> SharedString {
        self.sort_name.clone()
    }

    pub fn scopeless_name(&self) -> SharedString {
        self.name
            .clone()
            .split("::")
            .last()
            .unwrap_or(&self.name)
            .to_string()
            .into()
    }

    pub fn scope(&self) -> ComponentScope {
        self.scope.clone()
    }
    pub fn description(&self) -> Option<SharedString> {
        self.description.clone()
    }
    pub fn preview(&self) -> Option<fn(&mut Window, &mut App) -> Option<AnyElement>> {
        self.preview
    }
}

pub struct AllComponents(pub HashMap<ComponentId, ComponentMetadata>);

impl AllComponents {
    pub fn new() -> Self {
        AllComponents(HashMap::default())
    }
    pub fn all_previews(&self) -> Vec<&ComponentMetadata> {
        self.0.values().filter(|c| c.preview.is_some()).collect()
    }
    pub fn all_previews_sorted(&self) -> Vec<ComponentMetadata> {
        let mut previews: Vec<ComponentMetadata> =
            self.all_previews().into_iter().cloned().collect();
        previews.sort_by_key(|a| a.name());
        previews
    }
    pub fn all(&self) -> Vec<&ComponentMetadata> {
        self.0.values().collect()
    }
    pub fn all_sorted(&self) -> Vec<ComponentMetadata> {
        let mut components: Vec<ComponentMetadata> = self.all().into_iter().cloned().collect();
        components.sort_by_key(|a| a.name());
        components
    }
}

impl Deref for AllComponents {
    type Target = HashMap<ComponentId, ComponentMetadata>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AllComponents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn components() -> AllComponents {
    let data = COMPONENT_DATA.read();
    let mut all_components = AllComponents::new();
    for (scope, name, sort_name, description) in &data.components {
        let preview = data.previews.get(name).cloned();
        let component_name = SharedString::new_static(name);
        let sort_name = SharedString::new_static(sort_name);
        let id = ComponentId(name);
        all_components.insert(
            id.clone(),
            ComponentMetadata {
                id,
                name: component_name,
                sort_name,
                scope: scope.clone(),
                description: description.map(Into::into),
                preview,
            },
        );
    }
    all_components
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum ComponentStatus {
//     WorkInProgress,
//     EngineeringReady,
//     Live,
//     Deprecated,
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentScope {
    Agent,
    Collaboration,
    DataDisplay,
    Editor,
    Images,
    Input,
    Layout,
    Loading,
    Navigation,
    None,
    Notification,
    Overlays,
    Status,
    Typography,
    VersionControl,
}

impl Display for ComponentScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentScope::Agent => write!(f, "Agent"),
            ComponentScope::Collaboration => write!(f, "Collaboration"),
            ComponentScope::DataDisplay => write!(f, "Data Display"),
            ComponentScope::Editor => write!(f, "Editor"),
            ComponentScope::Images => write!(f, "Images & Icons"),
            ComponentScope::Input => write!(f, "Forms & Input"),
            ComponentScope::Layout => write!(f, "Layout & Structure"),
            ComponentScope::Loading => write!(f, "Loading & Progress"),
            ComponentScope::Navigation => write!(f, "Navigation"),
            ComponentScope::None => write!(f, "Unsorted"),
            ComponentScope::Notification => write!(f, "Notification"),
            ComponentScope::Overlays => write!(f, "Overlays & Layering"),
            ComponentScope::Status => write!(f, "Status"),
            ComponentScope::Typography => write!(f, "Typography"),
            ComponentScope::VersionControl => write!(f, "Version Control"),
        }
    }
}

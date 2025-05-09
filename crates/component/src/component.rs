//! # Component
//!
//! This module provides the Component trait, which is used to define
//! components for visual testing and debugging.
//!
//! Additionally, it includes layouts for rendering component examples
//! and example groups, as well as the distributed slice mechanism for
//! registering components.

mod component_layout;

pub use component_layout::*;

use std::sync::LazyLock;

use collections::HashMap;
use gpui::{AnyElement, App, SharedString, Window};
use linkme::distributed_slice;
use parking_lot::RwLock;
use strum::{Display, EnumString};

pub fn components() -> ComponentRegistry {
    COMPONENT_DATA.read().clone()
}

pub fn init() {
    let component_fns: Vec<_> = __ALL_COMPONENTS.iter().cloned().collect();
    for f in component_fns {
        f();
    }
}

pub fn register_component<T: Component>() {
    let id = T::id();
    let metadata = ComponentMetadata {
        id: id.clone(),
        description: T::description().map(Into::into),
        name: SharedString::new_static(T::name()),
        preview: Some(T::preview),
        scope: T::scope(),
        sort_name: SharedString::new_static(T::sort_name()),
        status: T::status(),
    };

    let mut data = COMPONENT_DATA.write();
    data.components.insert(id, metadata);
}

#[distributed_slice]
pub static __ALL_COMPONENTS: [fn()] = [..];

pub static COMPONENT_DATA: LazyLock<RwLock<ComponentRegistry>> =
    LazyLock::new(|| RwLock::new(ComponentRegistry::default()));

#[derive(Default, Clone)]
pub struct ComponentRegistry {
    components: HashMap<ComponentId, ComponentMetadata>,
}

impl ComponentRegistry {
    pub fn previews(&self) -> Vec<&ComponentMetadata> {
        self.components
            .values()
            .filter(|c| c.preview.is_some())
            .collect()
    }

    pub fn sorted_previews(&self) -> Vec<ComponentMetadata> {
        let mut previews: Vec<ComponentMetadata> = self.previews().into_iter().cloned().collect();
        previews.sort_by_key(|a| a.name());
        previews
    }

    pub fn components(&self) -> Vec<&ComponentMetadata> {
        self.components.values().collect()
    }

    pub fn sorted_components(&self) -> Vec<ComponentMetadata> {
        let mut components: Vec<ComponentMetadata> =
            self.components().into_iter().cloned().collect();
        components.sort_by_key(|a| a.name());
        components
    }

    pub fn component_map(&self) -> HashMap<ComponentId, ComponentMetadata> {
        self.components.clone()
    }

    pub fn get(&self, id: &ComponentId) -> Option<&ComponentMetadata> {
        self.components.get(id)
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub &'static str);

#[derive(Clone)]
pub struct ComponentMetadata {
    id: ComponentId,
    description: Option<SharedString>,
    name: SharedString,
    preview: Option<fn(&mut Window, &mut App) -> Option<AnyElement>>,
    scope: ComponentScope,
    sort_name: SharedString,
    status: ComponentStatus,
}

impl ComponentMetadata {
    pub fn id(&self) -> ComponentId {
        self.id.clone()
    }

    pub fn description(&self) -> Option<SharedString> {
        self.description.clone()
    }

    pub fn name(&self) -> SharedString {
        self.name.clone()
    }

    pub fn preview(&self) -> Option<fn(&mut Window, &mut App) -> Option<AnyElement>> {
        self.preview
    }

    pub fn scope(&self) -> ComponentScope {
        self.scope.clone()
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

    pub fn status(&self) -> ComponentStatus {
        self.status.clone()
    }
}

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
    /// The ready status of this component.
    ///
    /// Use this to mark when components are:
    /// - `WorkInProgress`: Still being designed or are partially implemented.
    /// - `EngineeringReady`: Ready to be implemented.
    /// - `Deprecated`: No longer recommended for use.
    ///
    /// Defaults to [`Live`](ComponentStatus::Live).
    fn status() -> ComponentStatus {
        ComponentStatus::Live
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

/// The ready status of this component.
///
/// Use this to mark when components are:
/// - `WorkInProgress`: Still being designed or are partially implemented.
/// - `EngineeringReady`: Ready to be implemented.
/// - `Deprecated`: No longer recommended for use.
///
/// Defaults to [`Live`](ComponentStatus::Live).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, EnumString)]
pub enum ComponentStatus {
    #[strum(serialize = "Work In Progress")]
    WorkInProgress,
    #[strum(serialize = "Ready To Build")]
    EngineeringReady,
    Live,
    Deprecated,
}

impl ComponentStatus {
    pub fn description(&self) -> &str {
        match self {
            ComponentStatus::WorkInProgress => {
                "These components are still being designed or refined. They shouldn't be used in the app yet."
            }
            ComponentStatus::EngineeringReady => {
                "These components are design complete or partially implemented, and are ready for an engineer to complete their implementation."
            }
            ComponentStatus::Live => "These components are ready for use in the app.",
            ComponentStatus::Deprecated => {
                "These components are no longer recommended for use in the app, and may be removed in a future release."
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, EnumString)]
pub enum ComponentScope {
    Agent,
    Collaboration,
    #[strum(serialize = "Data Display")]
    DataDisplay,
    Editor,
    #[strum(serialize = "Images & Icons")]
    Images,
    #[strum(serialize = "Forms & Input")]
    Input,
    #[strum(serialize = "Layout & Structure")]
    Layout,
    #[strum(serialize = "Loading & Progress")]
    Loading,
    Navigation,
    #[strum(serialize = "Unsorted")]
    None,
    Notification,
    #[strum(serialize = "Overlays & Layering")]
    Overlays,
    Status,
    Typography,
    #[strum(serialize = "Version Control")]
    VersionControl,
}

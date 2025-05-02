use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;

use collections::HashMap;
use gpui::{
    AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, pattern_slash, prelude::*,
    px, rems,
};
use linkme::distributed_slice;
use parking_lot::RwLock;
use theme::ActiveTheme;

pub trait Component {
    fn scope() -> ComponentScope {
        ComponentScope::None
    }
    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }
    fn id() -> ComponentId {
        ComponentId(Self::name())
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
    fn description() -> Option<&'static str> {
        None
    }
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

/// A single example of a component.
#[derive(IntoElement)]
pub struct ComponentExample {
    pub variant_name: SharedString,
    pub description: Option<SharedString>,
    pub element: AnyElement,
}

impl RenderOnce for ComponentExample {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .pt_2()
            .w_full()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .child(self.variant_name.clone())
                            .text_size(rems(1.0))
                            .text_color(cx.theme().colors().text),
                    )
                    .when_some(self.description, |this, description| {
                        this.child(
                            div()
                                .text_size(rems(0.875))
                                .text_color(cx.theme().colors().text_muted)
                                .child(description.clone()),
                        )
                    }),
            )
            .child(
                div()
                    .flex()
                    .w_full()
                    .rounded_xl()
                    .min_h(px(100.))
                    .justify_center()
                    .p_8()
                    .border_1()
                    .border_color(cx.theme().colors().border.opacity(0.5))
                    .bg(pattern_slash(
                        cx.theme().colors().surface_background.opacity(0.5),
                        12.0,
                        12.0,
                    ))
                    .shadow_sm()
                    .child(self.element),
            )
            .into_any_element()
    }
}

impl ComponentExample {
    pub fn new(variant_name: impl Into<SharedString>, element: AnyElement) -> Self {
        Self {
            variant_name: variant_name.into(),
            element,
            description: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// A group of component examples.
#[derive(IntoElement)]
pub struct ComponentExampleGroup {
    pub title: Option<SharedString>,
    pub examples: Vec<ComponentExample>,
    pub grow: bool,
    pub vertical: bool,
}

impl RenderOnce for ComponentExampleGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex_col()
            .text_sm()
            .text_color(cx.theme().colors().text_muted)
            .w_full()
            .when_some(self.title, |this, title| {
                this.gap_4().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .pb_1()
                        .child(div().h_px().w_4().bg(cx.theme().colors().border))
                        .child(
                            div()
                                .flex_none()
                                .text_size(px(10.))
                                .child(title.to_uppercase()),
                        )
                        .child(
                            div()
                                .h_px()
                                .w_full()
                                .flex_1()
                                .bg(cx.theme().colors().border),
                        ),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_start()
                    .w_full()
                    .gap_6()
                    .children(self.examples)
                    .into_any_element(),
            )
            .into_any_element()
    }
}

impl ComponentExampleGroup {
    pub fn new(examples: Vec<ComponentExample>) -> Self {
        Self {
            title: None,
            examples,
            grow: false,
            vertical: false,
        }
    }
    pub fn with_title(title: impl Into<SharedString>, examples: Vec<ComponentExample>) -> Self {
        Self {
            title: Some(title.into()),
            examples,
            grow: false,
            vertical: false,
        }
    }
    pub fn grow(mut self) -> Self {
        self.grow = true;
        self
    }
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
}

pub fn single_example(
    variant_name: impl Into<SharedString>,
    example: AnyElement,
) -> ComponentExample {
    ComponentExample::new(variant_name, example)
}

pub fn example_group(examples: Vec<ComponentExample>) -> ComponentExampleGroup {
    ComponentExampleGroup::new(examples)
}

pub fn example_group_with_title(
    title: impl Into<SharedString>,
    examples: Vec<ComponentExample>,
) -> ComponentExampleGroup {
    ComponentExampleGroup::with_title(title, examples)
}

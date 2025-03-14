use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;

use collections::HashMap;
use gpui::{div, prelude::*, px, AnyElement, App, IntoElement, RenderOnce, SharedString, Window};
pub use linkme::distributed_slice;
use parking_lot::RwLock;
use theme::ActiveTheme;

pub trait Component {
    fn scope() -> ComponentScope {
        ComponentScope::None
    }
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
    data.previews.insert(T::name(), T::preview);
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
    for (ref scope, name, sort_name, description) in &data.components {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentScope {
    Layout,
    Input,
    Notification,
    Editor,
    Collaboration,
    VersionControl,
    Unknown(SharedString),
    None,
}

impl Display for ComponentScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentScope::Layout => write!(f, "Layout"),
            ComponentScope::Input => write!(f, "Input"),
            ComponentScope::Notification => write!(f, "Notification"),
            ComponentScope::Editor => write!(f, "Editor"),
            ComponentScope::Collaboration => write!(f, "Collaboration"),
            ComponentScope::VersionControl => write!(f, "Version Control"),
            ComponentScope::Unknown(name) => write!(f, "Unknown: {}", name),
            ComponentScope::None => write!(f, "None"),
        }
    }
}

impl From<&str> for ComponentScope {
    fn from(value: &str) -> Self {
        match value {
            "Layout" => ComponentScope::Layout,
            "Input" => ComponentScope::Input,
            "Notification" => ComponentScope::Notification,
            "Editor" => ComponentScope::Editor,
            "Collaboration" => ComponentScope::Collaboration,
            "Version Control" | "VersionControl" => ComponentScope::VersionControl,
            "None" => ComponentScope::None,
            _ => ComponentScope::Unknown(SharedString::new(value)),
        }
    }
}

impl From<String> for ComponentScope {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Layout" => ComponentScope::Layout,
            "Input" => ComponentScope::Input,
            "Notification" => ComponentScope::Notification,
            "Editor" => ComponentScope::Editor,
            "Collaboration" => ComponentScope::Collaboration,
            "Version Control" | "VersionControl" => ComponentScope::VersionControl,
            "None" => ComponentScope::None,
            _ => ComponentScope::Unknown(SharedString::new(value)),
        }
    }
}

/// Which side of the preview to show labels on
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleLabelSide {
    Left,
    Right,
    #[default]
    Top,
    Bottom,
}

/// A single example of a component.
#[derive(IntoElement)]
pub struct ComponentExample {
    variant_name: SharedString,
    element: AnyElement,
    label_side: ExampleLabelSide,
    grow: bool,
}

impl RenderOnce for ComponentExample {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let base = div().flex();
        let base = match self.label_side {
            ExampleLabelSide::Right => base.flex_row(),
            ExampleLabelSide::Left => base.flex_row_reverse(),
            ExampleLabelSide::Bottom => base.flex_col(),
            ExampleLabelSide::Top => base.flex_col_reverse(),
        };
        base.gap_2()
            .p_2()
            .text_size(px(10.))
            .text_color(cx.theme().colors().text_muted)
            .when(self.grow, |this| this.flex_1())
            .when(!self.grow, |this| this.flex_none())
            .child(self.element)
            .child(self.variant_name)
            .into_any_element()
    }
}

impl ComponentExample {
    pub fn new(variant_name: impl Into<SharedString>, element: AnyElement) -> Self {
        Self {
            variant_name: variant_name.into(),
            element,
            label_side: ExampleLabelSide::default(),
            grow: false,
        }
    }
    pub fn grow(mut self) -> Self {
        self.grow = true;
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
            .when(self.grow, |this| this.w_full().flex_1())
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
                    .when(self.vertical, |this| this.flex_col())
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

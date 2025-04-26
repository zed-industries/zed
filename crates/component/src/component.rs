use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;

use collections::HashMap;
use gpui::{
    AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, pattern_slash, prelude::*,
    px, rems,
};
use itertools::Itertools;
use linkme::distributed_slice;
use parking_lot::RwLock;
use theme::ActiveTheme;

pub trait Component {
    type InitialState;

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
    /// State for `preview`, should be `()` for stateless components.
    fn initial_state(_cx: &mut App) -> Self::InitialState;
    /// Render the component.
    fn preview(
        _initial_state: &mut Self::InitialState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<AnyElement> {
        None
    }
}

#[distributed_slice]
pub static __ALL_COMPONENTS: [fn()] = [..];

pub static COMPONENT_DATA: LazyLock<RwLock<ComponentRegistry>> =
    LazyLock::new(|| RwLock::new(BTreeMap::default()));

pub type ComponentRegistry = BTreeMap<&'static str, ComponentRegistryItem>;

#[derive(Clone)]
pub struct ComponentRegistryItem {
    scope: ComponentScope,
    sort_name: &'static str,
    description: Option<&'static str>,
    preview_helper_creator: PreviewHelperCreator,
}

pub fn init() {
    let component_fns: Vec<_> = __ALL_COMPONENTS.iter().cloned().collect();
    for f in component_fns {
        f();
    }
}

pub fn register_component<T: Component + 'static>() {
    let component_data = ComponentRegistryItem {
        scope: T::scope(),
        sort_name: T::sort_name(),
        description: T::description(),
        preview_helper_creator: PreviewHelperCreator::new::<T>(),
    };

    COMPONENT_DATA.write().insert(T::name(), component_data);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub &'static str);

#[derive(Clone)]
struct PreviewHelperCreator {
    state_initializer: fn(&mut App) -> Box<dyn Any>,
    preview_fn: fn(Box<dyn Any>, &mut Window, &mut App) -> Option<AnyElement>,
}

impl PreviewHelperCreator {
    fn new<T: Component + Any>() -> Self {
        PreviewHelperCreator {
            state_initializer: state_initializer_type_erased::<T>,
            preview_fn: access_state_and_preview::<T>,
        }
    }
}

fn state_initializer_type_erased<T: Component + 'static>(cx: &mut App) -> Box<dyn Any> {
    Box::new(T::initial_state(cx))
}

fn access_state_and_preview<T: Component + 'static>(
    mut initial_state: Box<dyn Any>,
    window: &mut Window,
    cx: &mut App,
) -> Option<AnyElement> {
    let mut state = initial_state.downcast_mut::<T::InitialState>()?;
    T::preview(&mut state, window, cx)
}

impl PreviewHelperCreator {
    fn create(&self, cx: &mut App) -> PreviewHelper {
        PreviewHelper {
            state: (self.state_initializer)(cx),
            preview_fn: self.preview_fn,
        }
    }
}

struct PreviewHelper {
    state: Box<dyn Any>,
    preview_fn: fn(Box<dyn Any>, &mut Window, &mut App) -> Option<AnyElement>,
}

pub struct ComponentMetadata {
    id: ComponentId,
    name: SharedString,
    sort_name: SharedString,
    scope: ComponentScope,
    description: Option<SharedString>,
    preview_helper: PreviewHelper,
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
    // pub fn preview_helper(&self, cx: &mut App) -> PreviewHelper {
    //     self.preview_helper_creator.create(cx)
    // }
}

pub struct AllComponents(HashMap<ComponentId, ComponentMetadata>);

impl AllComponents {
    pub fn new(cx: &mut App) -> Self {
        let data = COMPONENT_DATA.read();
        let mut map = HashMap::new();
        for (name, item) in data.iter() {
            let ComponentRegistryItem {
                scope,
                sort_name,
                description,
                preview_helper_creator,
            } = item.clone();

            let id = ComponentId(name);

            map.insert(
                id.clone(),
                ComponentMetadata {
                    id,
                    name: SharedString::new(name.to_owned()),
                    sort_name: SharedString::new(sort_name.to_owned()),
                    scope,
                    description: description.map(Into::into),
                    preview_helper: preview_helper_creator.create(cx),
                },
            );
        }
        Self(map)
    }
    pub fn all_sorted(&self) -> Vec<&ComponentMetadata> {
        self.values().sorted_by_key(|a| a.name()).collect()
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

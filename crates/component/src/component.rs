use std::ops::{Deref, DerefMut};

use collections::HashMap;
use gpui::{div, prelude::*, px, AnyElement, App, IntoElement, RenderOnce, SharedString, Window};
use linkme::distributed_slice;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use theme::ActiveTheme;

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
    fn preview(_window: &mut Window, _cx: &App) -> AnyElement;
}

#[distributed_slice]
pub static __ALL_COMPONENTS: [fn()] = [..];

#[distributed_slice]
pub static __ALL_PREVIEWS: [fn()] = [..];

pub static COMPONENT_DATA: Lazy<RwLock<ComponentRegistry>> =
    Lazy::new(|| RwLock::new(ComponentRegistry::new()));

pub struct ComponentRegistry {
    components: Vec<(Option<&'static str>, &'static str, Option<&'static str>)>,
    previews: HashMap<&'static str, fn(&mut Window, &App) -> AnyElement>,
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
    let preview_data = (T::name(), T::preview as fn(&mut Window, &App) -> AnyElement);
    COMPONENT_DATA
        .write()
        .previews
        .insert(preview_data.0, preview_data.1);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub &'static str);

#[derive(Clone)]
pub struct ComponentMetadata {
    name: SharedString,
    scope: Option<SharedString>,
    description: Option<SharedString>,
    preview: Option<fn(&mut Window, &App) -> AnyElement>,
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

    pub fn preview(&self) -> Option<fn(&mut Window, &App) -> AnyElement> {
        self.preview
    }
}

pub struct AllComponents(pub HashMap<ComponentId, ComponentMetadata>);

impl AllComponents {
    pub fn new() -> Self {
        AllComponents(HashMap::default())
    }

    /// Returns all components with previews
    pub fn all_previews(&self) -> Vec<&ComponentMetadata> {
        self.0.values().filter(|c| c.preview.is_some()).collect()
    }

    /// Returns all components with previews sorted by name
    pub fn all_previews_sorted(&self) -> Vec<ComponentMetadata> {
        let mut previews: Vec<ComponentMetadata> =
            self.all_previews().into_iter().cloned().collect();
        previews.sort_by_key(|a| a.name());
        previews
    }

    /// Returns all components
    pub fn all(&self) -> Vec<&ComponentMetadata> {
        self.0.values().collect()
    }

    /// Returns all components sorted by name
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

    for &(scope, name, description) in &data.components {
        let scope = scope.map(Into::into);
        let preview = data.previews.get(name).cloned();
        all_components.insert(
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

/// Which side of the preview to show labels on
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleLabelSide {
    /// Left side
    Left,
    /// Right side
    Right,
    #[default]
    /// Top side
    Top,
    /// Bottom side
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

        base.gap_1()
            .p_2()
            .text_sm()
            .text_color(cx.theme().colors().text)
            .when(self.grow, |this| this.flex_1())
            .child(self.element)
            .child(self.variant_name)
            .into_any_element()
    }
}

impl ComponentExample {
    /// Create a new example with the given variant name and example value.
    pub fn new(variant_name: impl Into<SharedString>, element: AnyElement) -> Self {
        Self {
            variant_name: variant_name.into(),
            element,
            label_side: ExampleLabelSide::default(),
            grow: false,
        }
    }

    /// Set the example to grow to fill the available horizontal space.
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
}

impl RenderOnce for ComponentExampleGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex_col()
            .text_sm()
            .text_color(cx.theme().colors().text_muted)
            .when(self.grow, |this| this.w_full().flex_1())
            .when_some(self.title, |this, title| {
                this.gap_4().pb_5().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(div().h_px().w_4().bg(cx.theme().colors().border_variant))
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
                    .items_start()
                    .w_full()
                    .gap_8()
                    .children(self.examples)
                    .into_any_element(),
            )
            .into_any_element()
    }
}

impl ComponentExampleGroup {
    /// Create a new group of examples with the given title.
    pub fn new(examples: Vec<ComponentExample>) -> Self {
        Self {
            title: None,
            examples,
            grow: false,
        }
    }

    /// Create a new group of examples with the given title.
    pub fn with_title(title: impl Into<SharedString>, examples: Vec<ComponentExample>) -> Self {
        Self {
            title: Some(title.into()),
            examples,
            grow: false,
        }
    }

    /// Set the group to grow to fill the available horizontal space.
    pub fn grow(mut self) -> Self {
        self.grow = true;
        self
    }
}

/// Create a single example
pub fn single_example(
    variant_name: impl Into<SharedString>,
    example: AnyElement,
) -> ComponentExample {
    ComponentExample::new(variant_name, example)
}

/// Create a group of examples without a title
pub fn example_group(examples: Vec<ComponentExample>) -> ComponentExampleGroup {
    ComponentExampleGroup::new(examples)
}

/// Create a group of examples with a title
pub fn example_group_with_title(
    title: impl Into<SharedString>,
    examples: Vec<ComponentExample>,
) -> ComponentExampleGroup {
    ComponentExampleGroup::with_title(title, examples)
}

#![allow(missing_docs)]
use crate::prelude::*;
use gpui::{AnyElement, SharedString};

/// Implement this trait to enable rich UI previews with metadata in the Theme Preview tool.
pub trait ComponentPreview: IntoElement {
    fn title() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn description() -> impl Into<Option<&'static str>> {
        None
    }

    fn examples() -> Vec<ComponentExampleGroup<Self>>;

    fn component_previews() -> Vec<AnyElement> {
        Self::examples()
            .into_iter()
            .map(|example| Self::render_example_group(example))
            .collect()
    }

    fn render_component_previews(cx: &WindowContext) -> AnyElement {
        let title = Self::title();
        let (source, title) = title
            .rsplit_once("::")
            .map_or((None, title), |(s, t)| (Some(s), t));
        let description = Self::description().into();

        v_flex()
            .gap_3()
            .p_4()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(Headline::new(title).size(HeadlineSize::Small))
                            .when_some(source, |this, source| {
                                this.child(Label::new(format!("({})", source)).color(Color::Muted))
                            }),
                    )
                    .when_some(description, |this, description| {
                        this.child(
                            div()
                                .text_ui_sm(cx)
                                .text_color(cx.theme().colors().text_muted)
                                .max_w(px(600.0))
                                .child(description),
                        )
                    }),
            )
            .children(Self::component_previews())
            .into_any_element()
    }

    fn render_example_group(group: ComponentExampleGroup<Self>) -> AnyElement {
        v_flex()
            .gap_2()
            .child(Label::new(group.title).size(LabelSize::Small))
            .child(
                h_flex()
                    .gap_6()
                    .children(group.examples.into_iter().map(Self::render_example))
                    .into_any_element(),
            )
            .into_any_element()
    }

    fn render_example(example: ComponentExample<Self>) -> AnyElement {
        v_flex()
            .gap_1()
            .child(example.element)
            .child(
                Label::new(example.variant_name)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .into_any_element()
    }
}

/// A single example of a component.
pub struct ComponentExample<T> {
    variant_name: SharedString,
    element: T,
}

impl<T> ComponentExample<T> {
    /// Create a new example with the given variant name and example value.
    pub fn new(variant_name: impl Into<SharedString>, example: T) -> Self {
        Self {
            variant_name: variant_name.into(),
            element: example,
        }
    }
}

/// A group of component examples.
pub struct ComponentExampleGroup<T> {
    pub title: SharedString,
    pub examples: Vec<ComponentExample<T>>,
}

impl<T> ComponentExampleGroup<T> {
    /// Create a new group of examples with the given title.
    pub fn new(title: impl Into<SharedString>, examples: Vec<ComponentExample<T>>) -> Self {
        Self {
            title: title.into(),
            examples,
        }
    }
}

/// Create a single example
pub fn single_example<T>(variant_name: impl Into<SharedString>, example: T) -> ComponentExample<T> {
    ComponentExample::new(variant_name, example)
}

/// Create a group of examples
pub fn example_group<T>(
    title: impl Into<SharedString>,
    examples: Vec<ComponentExample<T>>,
) -> ComponentExampleGroup<T> {
    ComponentExampleGroup::new(title, examples)
}

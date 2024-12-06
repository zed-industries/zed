use crate::prelude::*;
use gpui::{AnyElement, IntoElement, Model, SharedString, WindowContext};

/// A trait that all components must implement
pub trait Component: IntoElement + Send + Sync {
    fn title() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn description() -> impl Into<Option<&'static str>> {
        None
    }

    /// The scope/category this component belongs to
    fn scope(&self) -> &'static str;

    fn preview(&self, cx: &WindowContext) -> Option<ComponentPreview<Self>> {
        None
    }

    fn render_component_previews(&self, cx: &WindowContext) -> Option<AnyElement> {
        let title = Self::title();
        let (source, title) = title
            .rsplit_once("::")
            .map_or((None, title), |(s, t)| (Some(s), t));
        let description = Self::description().into();

        let preview = v_flex()
            .w_full()
            .gap_6()
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
            // .when_some(Self::custom_example(cx).into(), |this, custom_example| {
            //     this.child(custom_example)
            // })
            // .children(Self::component_previews(cx))
            .into_any_element();

        // if let Some(preview) = self.preview(cx) {
        //     Some(preview.)
        // } else {
        //     None
        // }
    }

    // /// Render the component preview if it exists
    // fn preview(&self, cx: &WindowContext) -> Option<AnyElement> {
    //     if let Some(previews) = self.component_previews() {
    //         Some(previews(cx))
    //     } else {
    //         None
    //     }
    // }
}

/// Which side of the preview to show labels on
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewLabelSide {
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
pub struct ComponentPreviewVariant<T> {
    variant_name: SharedString,
    group: Option<SharedString>,
    element: T,
    grow: bool,
}

impl<T> ComponentPreviewVariant<T> {
    /// Create a new example with the given variant name and example value.
    pub fn new(variant_name: impl Into<SharedString>, example: T) -> Self {
        Self {
            variant_name: variant_name.into(),
            element: example,
            group: None,
            grow: false,
        }
    }

    /// Set the example to grow to fill the available horizontal space.
    pub fn grow(mut self) -> Self {
        self.grow = true;
        self
    }

    /// Set the example to be in the group
    pub fn group(mut self, group: impl Into<SharedString>) -> Self {
        self.group = Some(group.into());
        self
    }
}

/// A group of component examples.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ComponentPreviewGroup {
    pub title: Option<SharedString>,
    pub grow: bool,
}

pub struct ComponentPreview<T> {
    label_side: PreviewLabelSide,
    variants: Vec<ComponentPreviewVariant<T>>,
    groups: Vec<ComponentPreviewGroup>,
}

impl Default for ComponentPreview<AnyElement> {
    fn default() -> Self {
        Self {
            label_side: PreviewLabelSide::Bottom,
            variants: Vec::new(),
            groups: Vec::new(),
        }
    }
}

impl<T: IntoElement> ComponentPreview<T> {
    pub fn new(variants: Vec<ComponentPreviewVariant<T>>) -> Self {
        let groups: Vec<ComponentPreviewGroup> = variants
            .iter()
            .filter_map(|variant| variant.group.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .map(|group| ComponentPreviewGroup {
                title: Some(group),
                grow: false,
            })
            .collect();

        Self {
            label_side: PreviewLabelSide::Bottom,
            variants,
            groups,
        }
    }

    pub fn render_previews(&self, cx: &WindowContext) -> AnyElement {
        self.render_groups(cx)
    }

    fn render_groups(&self, cx: &WindowContext) -> AnyElement {
        v_flex()
            .gap_6()
            .children(self.groups.iter().map(|group| {
                let group_variants = self
                    .variants
                    .iter()
                    .filter(|variant| variant.group.as_ref() == group.title.as_ref())
                    .cloned();
                self.render_group(group.title.clone(), group_variants, cx)
            }))
            .into_any_element()
    }

    fn render_group(
        &self,
        group: Option<SharedString>,
        variants: Vec<ComponentPreviewVariant<T>>,
        cx: &WindowContext,
    ) -> AnyElement {
        v_flex()
            .gap_6()
            .when_some(group, |this, title| {
                this.child(Label::new(title).size(LabelSize::Small))
            })
            .child(
                h_flex()
                    .w_full()
                    .gap_6()
                    .children(
                        variants
                            .into_iter()
                            .map(|variant| self.render_variant(variant, cx)),
                    )
                    .into_any_element(),
            )
            .into_any_element()
    }

    fn render_variant(
        &self,
        variant: ComponentPreviewVariant<T>,
        cx: &WindowContext,
    ) -> AnyElement {
        let base = div().flex();

        let base = match self.label_side {
            PreviewLabelSide::Right => base.flex_row(),
            PreviewLabelSide::Left => base.flex_row_reverse(),
            PreviewLabelSide::Bottom => base.flex_col(),
            PreviewLabelSide::Top => base.flex_col_reverse(),
        };

        let grow = variant.grow.clone();
        let name = &variant.variant_name.clone();

        base.gap_1()
            .when(grow, |this| this.flex_1())
            .child(variant.element)
            .child(
                Label::new(name.clone())
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .into_any_element()
    }
}

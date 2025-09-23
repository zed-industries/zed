use gpui::{
    AnyElement, App, IntoElement, Pixels, RenderOnce, SharedString, Window, div, pattern_slash,
    prelude::*, px, rems,
};
use theme::ActiveTheme;

/// A single example of a component.
#[derive(IntoElement)]
pub struct ComponentExample {
    pub variant_name: SharedString,
    pub description: Option<SharedString>,
    pub element: AnyElement,
    pub width: Option<Pixels>,
}

impl RenderOnce for ComponentExample {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .pt_2()
            .map(|this| {
                if let Some(width) = self.width {
                    this.w(width)
                } else {
                    this.w_full()
                }
            })
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
                                .child(description),
                        )
                    }),
            )
            .child(
                div()
                    .min_h(px(100.))
                    .w_full()
                    .p_8()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_xl()
                    .border_1()
                    .border_color(cx.theme().colors().border.opacity(0.5))
                    .bg(pattern_slash(
                        cx.theme().colors().surface_background.opacity(0.25),
                        12.0,
                        12.0,
                    ))
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
            width: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

/// A group of component examples.
#[derive(IntoElement)]
pub struct ComponentExampleGroup {
    pub title: Option<SharedString>,
    pub examples: Vec<ComponentExample>,
    pub width: Option<Pixels>,
    pub grow: bool,
    pub vertical: bool,
}

impl RenderOnce for ComponentExampleGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex_col()
            .text_sm()
            .text_color(cx.theme().colors().text_muted)
            .map(|this| {
                if let Some(width) = self.width {
                    this.w(width)
                } else {
                    this.w_full()
                }
            })
            .when_some(self.title, |this, title| {
                this.gap_4().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .mt_4()
                        .mb_1()
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
            width: None,
            grow: false,
            vertical: false,
        }
    }
    pub fn with_title(title: impl Into<SharedString>, examples: Vec<ComponentExample>) -> Self {
        Self {
            title: Some(title.into()),
            examples,
            width: None,
            grow: false,
            vertical: false,
        }
    }
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
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

pub fn empty_example(variant_name: impl Into<SharedString>) -> ComponentExample {
    ComponentExample::new(variant_name, div().w_full().text_center().items_center().text_xs().opacity(0.4).child("This space is intentionally left blank. It indicates a case that should render nothing.").into_any_element())
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

use gpui::{div, prelude::*, Div, Element, ElementId, RenderOnce, Stateful, Styled, ViewContext};
use std::sync::Arc;
use theme2::ActiveTheme;

use crate::{Icon, IconElement, Selection, TextColor};

pub type CheckHandler<V> = Arc<dyn Fn(Selection, &mut V, &mut ViewContext<V>) + Send + Sync>;

/// # Checkbox
///
/// Checkboxes are used for multiple choices, not for mutually exclusive choices.
/// Each checkbox works independently from other checkboxes in the list,
/// therefore checking an additional box does not affect any other selections.
#[derive(RenderOnce)]
pub struct Checkbox<V: 'static> {
    id: ElementId,
    checked: Selection,
    disabled: bool,
    on_click: Option<CheckHandler<V>>,
}

impl<V: 'static> Component<V> for Checkbox<V> {
    type Rendered = Stateful<V, Div<V>>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
        let group_id = format!("checkbox_group_{:?}", self.id);

        let icon = match self.checked {
            // When selected, we show a checkmark.
            Selection::Selected => {
                Some(
                    IconElement::new(Icon::Check)
                        .size(crate::IconSize::Small)
                        .color(
                            // If the checkbox is disabled we change the color of the icon.
                            if self.disabled {
                                TextColor::Disabled
                            } else {
                                TextColor::Selected
                            },
                        ),
                )
            }
            // In an indeterminate state, we show a dash.
            Selection::Indeterminate => {
                Some(
                    IconElement::new(Icon::Dash)
                        .size(crate::IconSize::Small)
                        .color(
                            // If the checkbox is disabled we change the color of the icon.
                            if self.disabled {
                                TextColor::Disabled
                            } else {
                                TextColor::Selected
                            },
                        ),
                )
            }
            // When unselected, we show nothing.
            Selection::Unselected => None,
        };

        // A checkbox could be in an indeterminate state,
        // for example the indeterminate state could represent:
        //  - a group of options of which only some are selected
        //  - an enabled option that is no longer available
        //  - a previously agreed to license that has been updated
        //
        // For the sake of styles we treat the indeterminate state as selected,
        // but it's icon will be different.
        let selected =
            self.checked == Selection::Selected || self.checked == Selection::Indeterminate;

        // We could use something like this to make the checkbox background when selected:
        //
        // ~~~rust
        // ...
        // .when(selected, |this| {
        //     this.bg(cx.theme().colors().element_selected)
        // })
        // ~~~
        //
        // But we use a match instead here because the checkbox might be disabled,
        // and it could be disabled _while_ it is selected, as well as while it is not selected.
        let (bg_color, border_color) = match (self.disabled, selected) {
            (true, _) => (
                cx.theme().colors().ghost_element_disabled,
                cx.theme().colors().border_disabled,
            ),
            (false, true) => (
                cx.theme().colors().element_selected,
                cx.theme().colors().border,
            ),
            (false, false) => (
                cx.theme().colors().element_background,
                cx.theme().colors().border,
            ),
        };

        div()
            .id(self.id)
            // Rather than adding `px_1()` to add some space around the checkbox,
            // we use a larger parent element to create a slightly larger
            // click area for the checkbox.
            .size_5()
            // Because we've enlarged the click area, we need to create a
            // `group` to pass down interactivity events to the checkbox.
            .group(group_id.clone())
            .child(
                div()
                    .flex()
                    // This prevent the flex element from growing
                    // or shrinking in response to any size changes
                    .flex_none()
                    // The combo of `justify_center()` and `items_center()`
                    // is used frequently to center elements in a flex container.
                    //
                    // We use this to center the icon in the checkbox.
                    .justify_center()
                    .items_center()
                    .m_1()
                    .size_4()
                    .rounded_sm()
                    .bg(bg_color)
                    .border()
                    .border_color(border_color)
                    // We only want the interactivity states to fire when we
                    // are in a checkbox that isn't disabled.
                    .when(!self.disabled, |this| {
                        // Here instead of `hover()` we use `group_hover()`
                        // to pass it the group id.
                        this.group_hover(group_id.clone(), |el| {
                            el.bg(cx.theme().colors().element_hover)
                        })
                    })
                    .children(icon),
            )
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |view, _, cx| on_click(self.checked.inverse(), view, cx))
                },
            )
    }
}
impl<V: 'static> Checkbox<V> {
    pub fn new(id: impl Into<ElementId>, checked: Selection) -> Self {
        Self {
            id: id.into(),
            checked,
            disabled: false,
            on_click: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl 'static + Fn(Selection, &mut V, &mut ViewContext<V>) + Send + Sync,
    ) -> Self {
        self.on_click = Some(Arc::new(handler));
        self
    }

    pub fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let group_id = format!("checkbox_group_{:?}", self.id);

        let icon = match self.checked {
            // When selected, we show a checkmark.
            Selection::Selected => {
                Some(
                    IconElement::new(Icon::Check)
                        .size(crate::IconSize::Small)
                        .color(
                            // If the checkbox is disabled we change the color of the icon.
                            if self.disabled {
                                TextColor::Disabled
                            } else {
                                TextColor::Selected
                            },
                        ),
                )
            }
            // In an indeterminate state, we show a dash.
            Selection::Indeterminate => {
                Some(
                    IconElement::new(Icon::Dash)
                        .size(crate::IconSize::Small)
                        .color(
                            // If the checkbox is disabled we change the color of the icon.
                            if self.disabled {
                                TextColor::Disabled
                            } else {
                                TextColor::Selected
                            },
                        ),
                )
            }
            // When unselected, we show nothing.
            Selection::Unselected => None,
        };

        // A checkbox could be in an indeterminate state,
        // for example the indeterminate state could represent:
        //  - a group of options of which only some are selected
        //  - an enabled option that is no longer available
        //  - a previously agreed to license that has been updated
        //
        // For the sake of styles we treat the indeterminate state as selected,
        // but it's icon will be different.
        let selected =
            self.checked == Selection::Selected || self.checked == Selection::Indeterminate;

        // We could use something like this to make the checkbox background when selected:
        //
        // ~~~rust
        // ...
        // .when(selected, |this| {
        //     this.bg(cx.theme().colors().element_selected)
        // })
        // ~~~
        //
        // But we use a match instead here because the checkbox might be disabled,
        // and it could be disabled _while_ it is selected, as well as while it is not selected.
        let (bg_color, border_color) = match (self.disabled, selected) {
            (true, _) => (
                cx.theme().colors().ghost_element_disabled,
                cx.theme().colors().border_disabled,
            ),
            (false, true) => (
                cx.theme().colors().element_selected,
                cx.theme().colors().border,
            ),
            (false, false) => (
                cx.theme().colors().element_background,
                cx.theme().colors().border,
            ),
        };

        div()
            .id(self.id)
            // Rather than adding `px_1()` to add some space around the checkbox,
            // we use a larger parent element to create a slightly larger
            // click area for the checkbox.
            .size_5()
            // Because we've enlarged the click area, we need to create a
            // `group` to pass down interactivity events to the checkbox.
            .group(group_id.clone())
            .child(
                div()
                    .flex()
                    // This prevent the flex element from growing
                    // or shrinking in response to any size changes
                    .flex_none()
                    // The combo of `justify_center()` and `items_center()`
                    // is used frequently to center elements in a flex container.
                    //
                    // We use this to center the icon in the checkbox.
                    .justify_center()
                    .items_center()
                    .m_1()
                    .size_4()
                    .rounded_sm()
                    .bg(bg_color)
                    .border()
                    .border_color(border_color)
                    // We only want the interactivity states to fire when we
                    // are in a checkbox that isn't disabled.
                    .when(!self.disabled, |this| {
                        // Here instead of `hover()` we use `group_hover()`
                        // to pass it the group id.
                        this.group_hover(group_id.clone(), |el| {
                            el.bg(cx.theme().colors().element_hover)
                        })
                    })
                    .children(icon),
            )
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |view, _, cx| on_click(self.checked.inverse(), view, cx))
                },
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{h_stack, Story};
    use gpui::{Div, Render};

    pub struct CheckboxStory;

    impl Render<Self> for CheckboxStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Checkbox<Self>>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    h_stack()
                        .p_2()
                        .gap_2()
                        .rounded_md()
                        .border()
                        .border_color(cx.theme().colors().border)
                        .child(Checkbox::new("checkbox-enabled", Selection::Unselected))
                        .child(Checkbox::new(
                            "checkbox-intermediate",
                            Selection::Indeterminate,
                        ))
                        .child(Checkbox::new("checkbox-selected", Selection::Selected)),
                )
                .child(Story::label(cx, "Disabled"))
                .child(
                    h_stack()
                        .p_2()
                        .gap_2()
                        .rounded_md()
                        .border()
                        .border_color(cx.theme().colors().border)
                        .child(
                            Checkbox::new("checkbox-disabled", Selection::Unselected)
                                .disabled(true),
                        )
                        .child(
                            Checkbox::new(
                                "checkbox-disabled-intermediate",
                                Selection::Indeterminate,
                            )
                            .disabled(true),
                        )
                        .child(
                            Checkbox::new("checkbox-disabled-selected", Selection::Selected)
                                .disabled(true),
                        ),
                )
        }
    }
}

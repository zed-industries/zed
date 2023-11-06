///! # Checkbox
///!
///! Checkboxes are used for multiple choices, not for mutually exclusive choices.
///! Each checkbox works independently from other checkboxes in the list,
///! therefore checking an additional box does not affect any other selections.
use gpui2::{
    div, Component, ParentElement, SharedString, StatelessInteractive, Styled, ViewContext,
};
use theme2::ActiveTheme;

use crate::{Icon, IconColor, IconElement, Selected};

#[derive(Component)]
pub struct Checkbox {
    id: SharedString,
    checked: Selected,
    disabled: bool,
}

impl Checkbox {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            checked: Selected::Unselected,
            disabled: false,
        }
    }

    pub fn toggle(mut self) -> Self {
        self.checked = match self.checked {
            Selected::Selected => Selected::Unselected,
            Selected::Unselected => Selected::Selected,
            Selected::Indeterminate => Selected::Selected,
        };
        self
    }

    pub fn set_indeterminate(mut self) -> Self {
        self.checked = Selected::Indeterminate;
        self
    }

    pub fn set_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let group_id = format!("checkbox_group_{}", self.id);

        // The icon is different depending on the state of the checkbox.
        //
        // We need the match to return all the same type,
        // so we wrap the eatch result in a div.
        //
        // We are still exploring the best way to handle this.
        let icon = match self.checked {
            // When selected, we show a checkmark.
            Selected::Selected => {
                div().child(
                    IconElement::new(Icon::Check)
                        .size(crate::IconSize::Small)
                        .color(
                            // If the checkbox is disabled we change the color of the icon.
                            if self.disabled {
                                IconColor::Disabled
                            } else {
                                IconColor::Selected
                            },
                        ),
                )
            }
            // In an indeterminate state, we show a dash.
            Selected::Indeterminate => {
                div().child(
                    IconElement::new(Icon::Dash)
                        .size(crate::IconSize::Small)
                        .color(
                            // If the checkbox is disabled we change the color of the icon.
                            if self.disabled {
                                IconColor::Disabled
                            } else {
                                IconColor::Selected
                            },
                        ),
                )
            }
            // When unselected, we show nothing.
            Selected::Unselected => div(),
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
            self.checked == Selected::Selected || self.checked == Selected::Indeterminate;

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
            // Rather than adding `px_1()` to add some space around the checkbox,
            // we use a larger parent element to create a slightly larger
            // click area for the checkbox.
            .size_5()
            // Because we've enlarged the click area, we need to create a
            // `group` to pass down interaction events to the checkbox.
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
                    // We only want the interaction states to fire when we
                    // are in a checkbox that isn't disabled.
                    .when(!self.disabled, |this| {
                        // Here instead of `hover()` we use `group_hover()`
                        // to pass it the group id.
                        this.group_hover(group_id.clone(), |el| {
                            el.bg(cx.theme().colors().element_hover)
                        })
                    })
                    .child(icon),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{h_stack, Story};
    use gpui2::{Div, Render};

    pub struct CheckboxStory;

    impl Render for CheckboxStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Checkbox>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    h_stack()
                        .p_2()
                        .gap_2()
                        .rounded_md()
                        .border()
                        .border_color(cx.theme().colors().border)
                        .child(Checkbox::new("checkbox-enabled"))
                        .child(Checkbox::new("checkbox-intermediate").set_indeterminate())
                        .child(Checkbox::new("checkbox-selected").toggle()),
                )
                .child(Story::label(cx, "Disabled"))
                .child(
                    h_stack()
                        .p_2()
                        .gap_2()
                        .rounded_md()
                        .border()
                        .border_color(cx.theme().colors().border)
                        .child(Checkbox::new("checkbox-disabled").set_disabled(true))
                        .child(
                            Checkbox::new("checkbox-disabled-intermediate")
                                .set_disabled(true)
                                .set_indeterminate(),
                        )
                        .child(
                            Checkbox::new("checkbox-disabled-selected")
                                .set_disabled(true)
                                .toggle(),
                        ),
                )
        }
    }
}

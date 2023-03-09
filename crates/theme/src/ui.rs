use gpui::{
    color::Color,
    elements::{
        ConstrainedBox, Container, ContainerStyle, Empty, Flex, KeystrokeLabel, Label,
        MouseEventHandler, ParentElement, Svg,
    },
    Action, Element, ElementBox, EventContext, RenderContext, View,
};
use serde::Deserialize;

use crate::ContainedText;

#[derive(Clone, Deserialize, Default)]
pub struct CheckboxStyle {
    pub icon: IconStyle,
    pub label: ContainedText,
    pub default: ContainerStyle,
    pub checked: ContainerStyle,
    pub hovered: ContainerStyle,
    pub hovered_and_checked: ContainerStyle,
}

pub fn checkbox<T: 'static, V: View>(
    label: &'static str,
    style: &CheckboxStyle,
    checked: bool,
    cx: &mut RenderContext<V>,
    change: fn(checked: bool, cx: &mut EventContext) -> (),
) -> MouseEventHandler<T> {
    let label = Label::new(label, style.label.text.clone())
        .contained()
        .with_style(style.label.container)
        .boxed();

    checkbox_with_label(label, style, checked, cx, change)
}

pub fn checkbox_with_label<T: 'static, V: View>(
    label: ElementBox,
    style: &CheckboxStyle,
    checked: bool,
    cx: &mut RenderContext<V>,
    change: fn(checked: bool, cx: &mut EventContext) -> (),
) -> MouseEventHandler<T> {
    MouseEventHandler::<T>::new(0, cx, |state, _| {
        let indicator = if checked {
            icon(&style.icon)
        } else {
            Empty::new()
                .constrained()
                .with_width(style.icon.dimensions.width)
                .with_height(style.icon.dimensions.height)
        };

        Flex::row()
            .with_children([
                indicator
                    .contained()
                    .with_style(if checked {
                        if state.hovered() {
                            style.hovered_and_checked
                        } else {
                            style.checked
                        }
                    } else {
                        if state.hovered() {
                            style.hovered
                        } else {
                            style.default
                        }
                    })
                    .boxed(),
                label,
            ])
            .align_children_center()
            .boxed()
    })
    .on_click(gpui::MouseButton::Left, move |_, cx| change(!checked, cx))
    .with_cursor_style(gpui::CursorStyle::PointingHand)
}

#[derive(Clone, Deserialize, Default)]
pub struct IconStyle {
    pub color: Color,
    pub icon: String,
    pub dimensions: Dimensions,
}

#[derive(Clone, Deserialize, Default)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

pub fn icon(style: &IconStyle) -> ConstrainedBox {
    Svg::new(style.icon.clone())
        .with_color(style.color)
        .constrained()
        .with_width(style.dimensions.width)
        .with_height(style.dimensions.height)
}

pub fn keystroke_label<V: View>(
    label_text: &'static str,
    label_style: &ContainedText,
    keystroke_style: &ContainedText,
    action: Box<dyn Action>,
    cx: &mut RenderContext<V>,
) -> Container {
    // FIXME: Put the theme in it's own global so we can
    // query the keystroke style on our own
    keystroke_label_for(
        cx.window_id(),
        cx.handle().id(),
        label_text,
        label_style,
        keystroke_style,
        action,
    )
}

pub fn keystroke_label_for(
    window_id: usize,
    view_id: usize,
    label_text: &'static str,
    label_style: &ContainedText,
    keystroke_style: &ContainedText,
    action: Box<dyn Action>,
) -> Container {
    Flex::row()
        .with_child(
            Label::new(label_text, label_style.text.clone())
                .contained()
                .boxed(),
        )
        .with_child({
            KeystrokeLabel::new(
                window_id,
                view_id,
                action,
                keystroke_style.container,
                keystroke_style.text.clone(),
            )
            .flex_float()
            .boxed()
        })
        .contained()
        .with_style(label_style.container)
}

use std::borrow::Cow;

use gpui::{
    color::Color,
    elements::{
        ConstrainedBox, Container, ContainerStyle, Empty, Flex, KeystrokeLabel, Label,
        MouseEventHandler, ParentElement, Stack, Svg,
    },
    fonts::TextStyle,
    geometry::vector::{vec2f, Vector2F},
    scene::MouseClick,
    Action, Element, ElementBox, EventContext, MouseButton, MouseState, RenderContext, View,
};
use serde::Deserialize;

use crate::{ContainedText, Interactive};

#[derive(Clone, Deserialize, Default)]
pub struct CheckboxStyle {
    pub icon: SvgStyle,
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
            svg(&style.icon)
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
pub struct SvgStyle {
    pub color: Color,
    pub asset: String,
    pub dimensions: Dimensions,
}

#[derive(Clone, Deserialize, Default)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    pub fn to_vec(&self) -> Vector2F {
        vec2f(self.width, self.height)
    }
}

pub fn svg(style: &SvgStyle) -> ConstrainedBox {
    Svg::new(style.asset.clone())
        .with_color(style.color)
        .constrained()
        .with_width(style.dimensions.width)
        .with_height(style.dimensions.height)
}

#[derive(Clone, Deserialize, Default)]
pub struct IconStyle {
    icon: SvgStyle,
    container: ContainerStyle,
}

pub fn icon(style: &IconStyle) -> Container {
    svg(&style.icon).contained().with_style(style.container)
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

pub type ButtonStyle = Interactive<ContainedText>;

pub fn cta_button<L, A, V>(
    label: L,
    action: A,
    max_width: f32,
    style: &ButtonStyle,
    cx: &mut RenderContext<V>,
) -> ElementBox
where
    L: Into<Cow<'static, str>>,
    A: 'static + Action + Clone,
    V: View,
{
    cta_button_with_click(label, max_width, style, cx, move |_, cx| {
        cx.dispatch_action(action.clone())
    })
    .boxed()
}

pub fn cta_button_with_click<L, V, F>(
    label: L,
    max_width: f32,
    style: &ButtonStyle,
    cx: &mut RenderContext<V>,
    f: F,
) -> MouseEventHandler<F>
where
    L: Into<Cow<'static, str>>,
    V: View,
    F: Fn(MouseClick, &mut EventContext) + 'static,
{
    MouseEventHandler::<F>::new(0, cx, |state, _| {
        let style = style.style_for(state, false);
        Label::new(label, style.text.to_owned())
            .aligned()
            .contained()
            .with_style(style.container)
            .constrained()
            .with_max_width(max_width)
            .boxed()
    })
    .on_click(MouseButton::Left, f)
    .with_cursor_style(gpui::CursorStyle::PointingHand)
}

#[derive(Clone, Deserialize, Default)]
pub struct ModalStyle {
    close_icon: Interactive<IconStyle>,
    container: ContainerStyle,
    titlebar: ContainerStyle,
    title_text: Interactive<TextStyle>,
    dimensions: Dimensions,
}

impl ModalStyle {
    pub fn dimensions(&self) -> Vector2F {
        self.dimensions.to_vec()
    }
}

pub fn modal<V, I, F>(
    title: I,
    style: &ModalStyle,
    cx: &mut RenderContext<V>,
    build_modal: F,
) -> ElementBox
where
    V: View,
    I: Into<Cow<'static, str>>,
    F: FnOnce(&mut gpui::RenderContext<V>) -> ElementBox,
{
    const TITLEBAR_HEIGHT: f32 = 28.;
    // let active = cx.window_is_active(cx.window_id());

    Flex::column()
        .with_child(
            Stack::new()
                .with_children([
                    Label::new(
                        title,
                        style
                            .title_text
                            .style_for(&mut MouseState::default(), false)
                            .clone(),
                    )
                    .boxed(),
                    // FIXME: Get a better tag type
                    MouseEventHandler::<V>::new(999999, cx, |state, _cx| {
                        let style = style.close_icon.style_for(state, false);
                        icon(style).boxed()
                    })
                    .on_click(gpui::MouseButton::Left, move |_, cx| {
                        let window_id = cx.window_id();
                        cx.remove_window(window_id);
                    })
                    .with_cursor_style(gpui::CursorStyle::PointingHand)
                    .aligned()
                    .right()
                    .boxed(),
                ])
                .contained()
                .with_style(style.titlebar)
                .constrained()
                .with_height(TITLEBAR_HEIGHT)
                .boxed(),
        )
        .with_child(
            Container::new(build_modal(cx))
                .with_style(style.container)
                .constrained()
                .with_width(style.dimensions().x())
                .with_height(style.dimensions().y() - TITLEBAR_HEIGHT)
                .boxed(),
        )
        .constrained()
        .with_height(style.dimensions().y())
        .boxed()
}

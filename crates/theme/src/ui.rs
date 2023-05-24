use std::borrow::Cow;

use gpui::{
    color::Color,
    elements::{
        ConstrainedBox, Container, ContainerStyle, Empty, Flex, KeystrokeLabel, Label,
        MouseEventHandler, ParentElement, Stack, Svg,
    },
    fonts::TextStyle,
    geometry::vector::{vec2f, Vector2F},
    platform,
    platform::MouseButton,
    scene::MouseClick,
    Action, Element, EventContext, MouseState, View, ViewContext,
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

pub fn checkbox<Tag, V, F>(
    label: &'static str,
    style: &CheckboxStyle,
    checked: bool,
    id: usize,
    cx: &mut ViewContext<V>,
    change: F,
) -> MouseEventHandler<Tag, V>
where
    Tag: 'static,
    V: View,
    F: 'static + Fn(&mut V, bool, &mut EventContext<V>),
{
    let label = Label::new(label, style.label.text.clone())
        .contained()
        .with_style(style.label.container);
    checkbox_with_label(label, style, checked, id, cx, change)
}

pub fn checkbox_with_label<Tag, D, V, F>(
    label: D,
    style: &CheckboxStyle,
    checked: bool,
    id: usize,
    cx: &mut ViewContext<V>,
    change: F,
) -> MouseEventHandler<Tag, V>
where
    Tag: 'static,
    D: Element<V>,
    V: View,
    F: 'static + Fn(&mut V, bool, &mut EventContext<V>),
{
    MouseEventHandler::new(id, cx, |state, _| {
        let indicator = if checked {
            svg(&style.icon)
        } else {
            Empty::new()
                .constrained()
                .with_width(style.icon.dimensions.width)
                .with_height(style.icon.dimensions.height)
        };

        Flex::row()
            .with_child(indicator.contained().with_style(if checked {
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
            }))
            .with_child(label)
            .align_children_center()
    })
    .on_click(platform::MouseButton::Left, move |_, view, cx| {
        change(view, !checked, cx)
    })
    .with_cursor_style(platform::CursorStyle::PointingHand)
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

pub fn svg<V: View>(style: &SvgStyle) -> ConstrainedBox<V> {
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

pub fn icon<V: View>(style: &IconStyle) -> Container<V> {
    svg(&style.icon).contained().with_style(style.container)
}

pub fn keystroke_label<V: View>(
    label_text: &'static str,
    label_style: &ContainedText,
    keystroke_style: &ContainedText,
    action: Box<dyn Action>,
    cx: &mut ViewContext<V>,
) -> Container<V> {
    // FIXME: Put the theme in it's own global so we can
    // query the keystroke style on our own
    Flex::row()
        .with_child(Label::new(label_text, label_style.text.clone()).contained())
        .with_child(
            KeystrokeLabel::new(
                cx.view_id(),
                action,
                keystroke_style.container,
                keystroke_style.text.clone(),
            )
            .flex_float(),
        )
        .contained()
        .with_style(label_style.container)
}

pub type ButtonStyle = Interactive<ContainedText>;

pub fn cta_button<Tag, L, V, F>(
    label: L,
    max_width: f32,
    style: &ButtonStyle,
    cx: &mut ViewContext<V>,
    f: F,
) -> MouseEventHandler<Tag, V>
where
    Tag: 'static,
    L: Into<Cow<'static, str>>,
    V: View,
    F: Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
{
    MouseEventHandler::<Tag, V>::new(0, cx, |state, _| {
        let style = style.style_for(state, false);
        Label::new(label, style.text.to_owned())
            .aligned()
            .contained()
            .with_style(style.container)
            .constrained()
            .with_max_width(max_width)
    })
    .on_click(MouseButton::Left, f)
    .with_cursor_style(platform::CursorStyle::PointingHand)
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

pub fn modal<Tag, V, I, D, F>(
    title: I,
    style: &ModalStyle,
    cx: &mut ViewContext<V>,
    build_modal: F,
) -> impl Element<V>
where
    Tag: 'static,
    V: View,
    I: Into<Cow<'static, str>>,
    D: Element<V>,
    F: FnOnce(&mut gpui::ViewContext<V>) -> D,
{
    const TITLEBAR_HEIGHT: f32 = 28.;
    // let active = cx.window_is_active(cx.window_id());

    Flex::column()
        .with_child(
            Stack::new()
                .with_child(Label::new(
                    title,
                    style
                        .title_text
                        .style_for(&mut MouseState::default(), false)
                        .clone(),
                ))
                .with_child(
                    // FIXME: Get a better tag type
                    MouseEventHandler::<Tag, V>::new(999999, cx, |state, _cx| {
                        let style = style.close_icon.style_for(state, false);
                        icon(style)
                    })
                    .on_click(platform::MouseButton::Left, move |_, _, cx| {
                        cx.remove_window();
                    })
                    .with_cursor_style(platform::CursorStyle::PointingHand)
                    .aligned()
                    .right(),
                )
                .contained()
                .with_style(style.titlebar)
                .constrained()
                .with_height(TITLEBAR_HEIGHT),
        )
        .with_child(
            build_modal(cx)
                .contained()
                .with_style(style.container)
                .constrained()
                .with_width(style.dimensions().x())
                .with_height(style.dimensions().y() - TITLEBAR_HEIGHT),
        )
        .constrained()
        .with_height(style.dimensions().y())
}

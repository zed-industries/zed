use gpui::{elements::SafeStylable, Action};

use crate::{Interactive, Toggleable};

use self::{action_button::ButtonStyle, disclosure::Disclosable, svg::SvgStyle, toggle::Toggle};

pub type IconButtonStyle = Interactive<ButtonStyle<SvgStyle>>;
pub type ToggleIconButtonStyle = Toggleable<IconButtonStyle>;

pub trait ComponentExt<C: SafeStylable> {
    fn toggleable(self, active: bool) -> Toggle<C, ()>;
    fn disclosable(self, disclosed: Option<bool>, action: Box<dyn Action>) -> Disclosable<C, ()>;
}

impl<C: SafeStylable> ComponentExt<C> for C {
    fn toggleable(self, active: bool) -> Toggle<C, ()> {
        Toggle::new(self, active)
    }

    /// Some(True) => disclosed => content is visible
    /// Some(false) => closed => content is hidden
    /// None => No disclosure button, but reserve disclosure spacing
    fn disclosable(self, disclosed: Option<bool>, action: Box<dyn Action>) -> Disclosable<C, ()> {
        Disclosable::new(disclosed, self, action)
    }
}

pub mod disclosure {

    use gpui::{
        elements::{Component, ContainerStyle, Empty, Flex, ParentElement, SafeStylable},
        Action, Element,
    };
    use schemars::JsonSchema;
    use serde_derive::Deserialize;

    use super::{action_button::Button, svg::Svg, ComponentExt, IconButtonStyle};

    #[derive(Clone, Default, Deserialize, JsonSchema)]
    pub struct DisclosureStyle<S> {
        pub button: IconButtonStyle,
        #[serde(flatten)]
        pub container: ContainerStyle,
        pub spacing: f32,
        #[serde(flatten)]
        content: S,
    }

    impl<S> DisclosureStyle<S> {
        pub fn button_space(&self) -> f32 {
            self.spacing + self.button.button_width.unwrap()
        }
    }

    pub struct Disclosable<C, S> {
        disclosed: Option<bool>,
        action: Box<dyn Action>,
        id: usize,
        content: C,
        style: S,
    }

    impl Disclosable<(), ()> {
        pub fn new<C>(
            disclosed: Option<bool>,
            content: C,
            action: Box<dyn Action>,
        ) -> Disclosable<C, ()> {
            Disclosable {
                disclosed,
                content,
                action,
                id: 0,
                style: (),
            }
        }
    }

    impl<C> Disclosable<C, ()> {
        pub fn with_id(mut self, id: usize) -> Disclosable<C, ()> {
            self.id = id;
            self
        }
    }

    impl<C: SafeStylable> SafeStylable for Disclosable<C, ()> {
        type Style = DisclosureStyle<C::Style>;

        type Output = Disclosable<C, Self::Style>;

        fn with_style(self, style: Self::Style) -> Self::Output {
            Disclosable {
                disclosed: self.disclosed,
                action: self.action,
                content: self.content,
                id: self.id,
                style,
            }
        }
    }

    impl<C: SafeStylable> Component for Disclosable<C, DisclosureStyle<C::Style>> {
        fn render<V: gpui::View>(self, cx: &mut gpui::ViewContext<V>) -> gpui::AnyElement<V> {
            Flex::row()
                .with_spacing(self.style.spacing)
                .with_child(if let Some(disclosed) = self.disclosed {
                    Button::dynamic_action(self.action)
                        .with_id(self.id)
                        .with_contents(Svg::new(if disclosed {
                            "icons/file_icons/chevron_down.svg"
                        } else {
                            "icons/file_icons/chevron_right.svg"
                        }))
                        .with_style(self.style.button)
                        .element()
                        .into_any()
                } else {
                    Empty::new()
                        .into_any()
                        .constrained()
                        // TODO: Why is this optional at all?
                        .with_width(self.style.button.button_width.unwrap())
                        .into_any()
                })
                .with_child(
                    self.content
                        .with_style(self.style.content)
                        .render(cx)
                        .flex(1., true),
                )
                .align_children_center()
                .contained()
                .with_style(self.style.container)
                .into_any()
        }
    }
}

pub mod toggle {
    use gpui::elements::{Component, SafeStylable};

    use crate::Toggleable;

    pub struct Toggle<C, S> {
        style: S,
        active: bool,
        component: C,
    }

    impl<C: SafeStylable> Toggle<C, ()> {
        pub fn new(component: C, active: bool) -> Self {
            Toggle {
                active,
                component,
                style: (),
            }
        }
    }

    impl<C: SafeStylable> SafeStylable for Toggle<C, ()> {
        type Style = Toggleable<C::Style>;

        type Output = Toggle<C, Self::Style>;

        fn with_style(self, style: Self::Style) -> Self::Output {
            Toggle {
                active: self.active,
                component: self.component,
                style,
            }
        }
    }

    impl<C: SafeStylable> Component for Toggle<C, Toggleable<C::Style>> {
        fn render<V: gpui::View>(self, cx: &mut gpui::ViewContext<V>) -> gpui::AnyElement<V> {
            self.component
                .with_style(self.style.in_state(self.active).clone())
                .render(cx)
        }
    }
}

pub mod action_button {
    use std::borrow::Cow;

    use gpui::{
        elements::{Component, ContainerStyle, MouseEventHandler, SafeStylable, TooltipStyle},
        platform::{CursorStyle, MouseButton},
        Action, Element, EventContext, TypeTag, View,
    };
    use schemars::JsonSchema;
    use serde_derive::Deserialize;

    use crate::Interactive;

    #[derive(Clone, Deserialize, Default, JsonSchema)]
    pub struct ButtonStyle<C> {
        #[serde(flatten)]
        pub container: ContainerStyle,
        // TODO: These are incorrect for the intended usage of the buttons.
        // The size should be constant, but putting them here duplicates them
        // across the states the buttons can be in
        pub button_width: Option<f32>,
        pub button_height: Option<f32>,
        #[serde(flatten)]
        contents: C,
    }

    pub struct Button<C, S> {
        action: Box<dyn Action>,
        tooltip: Option<(Cow<'static, str>, TooltipStyle)>,
        tag: TypeTag,
        id: usize,
        contents: C,
        style: Interactive<S>,
    }

    impl Button<(), ()> {
        pub fn dynamic_action(action: Box<dyn Action>) -> Button<(), ()> {
            Self {
                contents: (),
                tag: action.type_tag(),
                action,
                style: Interactive::new_blank(),
                tooltip: None,
                id: 0,
            }
        }

        pub fn action<A: Action + Clone>(action: A) -> Self {
            Self::dynamic_action(Box::new(action))
        }

        pub fn with_tooltip(
            mut self,
            tooltip: impl Into<Cow<'static, str>>,
            tooltip_style: TooltipStyle,
        ) -> Self {
            self.tooltip = Some((tooltip.into(), tooltip_style));
            self
        }

        pub fn with_id(mut self, id: usize) -> Self {
            self.id = id;
            self
        }

        pub fn with_contents<C: SafeStylable>(self, contents: C) -> Button<C, ()> {
            Button {
                action: self.action,
                tag: self.tag,
                style: self.style,
                tooltip: self.tooltip,
                id: self.id,
                contents,
            }
        }
    }

    impl<C: SafeStylable> SafeStylable for Button<C, ()> {
        type Style = Interactive<ButtonStyle<C::Style>>;
        type Output = Button<C, ButtonStyle<C::Style>>;

        fn with_style(self, style: Self::Style) -> Self::Output {
            Button {
                action: self.action,
                tag: self.tag,
                contents: self.contents,
                tooltip: self.tooltip,
                id: self.id,
                style,
            }
        }
    }

    impl<C: SafeStylable> Component for Button<C, ButtonStyle<C::Style>> {
        fn render<V: View>(self, cx: &mut gpui::ViewContext<V>) -> gpui::AnyElement<V> {
            let mut button = MouseEventHandler::new_dynamic(self.tag, self.id, cx, |state, cx| {
                let style = self.style.style_for(state);
                let mut contents = self
                    .contents
                    .with_style(style.contents.to_owned())
                    .render(cx)
                    .contained()
                    .with_style(style.container)
                    .constrained();

                if let Some(height) = style.button_height {
                    contents = contents.with_height(height);
                }

                if let Some(width) = style.button_width {
                    contents = contents.with_width(width);
                }

                contents.into_any()
            })
            .on_click(MouseButton::Left, {
                let action = self.action.boxed_clone();
                move |_, _, cx: &mut EventContext<V>| {
                    let window = cx.window();
                    let view = cx.view_id();
                    let action = action.boxed_clone();
                    cx.spawn(|_, mut cx| async move {
                        window.dispatch_action(view, action.as_ref(), &mut cx)
                    })
                    .detach();
                }
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .into_any();

            if let Some((tooltip, style)) = self.tooltip {
                button = button
                    .with_dynamic_tooltip(self.tag, 0, tooltip, Some(self.action), style, cx)
                    .into_any()
            }

            button
        }
    }
}

pub mod svg {
    use std::borrow::Cow;

    use gpui::{
        elements::{Component, Empty, SafeStylable},
        Element,
    };
    use schemars::JsonSchema;
    use serde::Deserialize;

    #[derive(Clone, Default, JsonSchema)]
    pub struct SvgStyle {
        icon_width: f32,
        icon_height: f32,
        color: gpui::color::Color,
    }

    impl<'de> Deserialize<'de> for SvgStyle {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            #[serde(untagged)]
            pub enum IconSize {
                IconSize { icon_size: f32 },
                Dimensions { width: f32, height: f32 },
                IconDimensions { icon_width: f32, icon_height: f32 },
            }

            #[derive(Deserialize)]
            struct SvgStyleHelper {
                #[serde(flatten)]
                size: IconSize,
                color: gpui::color::Color,
            }

            let json = SvgStyleHelper::deserialize(deserializer)?;
            let color = json.color;

            let result = match json.size {
                IconSize::IconSize { icon_size } => SvgStyle {
                    icon_width: icon_size,
                    icon_height: icon_size,
                    color,
                },
                IconSize::Dimensions { width, height } => SvgStyle {
                    icon_width: width,
                    icon_height: height,
                    color,
                },
                IconSize::IconDimensions {
                    icon_width,
                    icon_height,
                } => SvgStyle {
                    icon_width,
                    icon_height,
                    color,
                },
            };

            Ok(result)
        }
    }

    pub struct Svg<S> {
        path: Option<Cow<'static, str>>,
        style: S,
    }

    impl Svg<()> {
        pub fn new(path: impl Into<Cow<'static, str>>) -> Self {
            Self {
                path: Some(path.into()),
                style: (),
            }
        }

        pub fn optional(path: Option<impl Into<Cow<'static, str>>>) -> Self {
            Self {
                path: path.map(Into::into),
                style: (),
            }
        }
    }

    impl SafeStylable for Svg<()> {
        type Style = SvgStyle;

        type Output = Svg<SvgStyle>;

        fn with_style(self, style: Self::Style) -> Self::Output {
            Svg {
                path: self.path,
                style,
            }
        }
    }

    impl Component for Svg<SvgStyle> {
        fn render<V: gpui::View>(self, _: &mut gpui::ViewContext<V>) -> gpui::AnyElement<V> {
            if let Some(path) = self.path {
                gpui::elements::Svg::new(path)
                    .with_color(self.style.color)
                    .constrained()
            } else {
                Empty::new().constrained()
            }
            .constrained()
            .with_width(self.style.icon_width)
            .with_height(self.style.icon_height)
            .into_any()
        }
    }
}

pub mod label {
    use std::borrow::Cow;

    use gpui::{
        elements::{Component, LabelStyle, SafeStylable},
        fonts::TextStyle,
        Element,
    };

    pub struct Label<S> {
        text: Cow<'static, str>,
        style: S,
    }

    impl Label<()> {
        pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
            Self {
                text: text.into(),
                style: (),
            }
        }
    }

    impl SafeStylable for Label<()> {
        type Style = TextStyle;

        type Output = Label<LabelStyle>;

        fn with_style(self, style: Self::Style) -> Self::Output {
            Label {
                text: self.text,
                style: style.into(),
            }
        }
    }

    impl Component for Label<LabelStyle> {
        fn render<V: gpui::View>(self, _: &mut gpui::ViewContext<V>) -> gpui::AnyElement<V> {
            gpui::elements::Label::new(self.text, self.style).into_any()
        }
    }
}

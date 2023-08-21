use gpui::elements::StyleableComponent;

use crate::{Interactive, Toggleable};

use self::{action_button::ButtonStyle, svg::SvgStyle, toggle::Toggle};

pub type ToggleIconButtonStyle = Toggleable<Interactive<ButtonStyle<SvgStyle>>>;

pub trait ComponentExt<C: StyleableComponent> {
    fn toggleable(self, active: bool) -> Toggle<C, ()>;
}

impl<C: StyleableComponent> ComponentExt<C> for C {
    fn toggleable(self, active: bool) -> Toggle<C, ()> {
        Toggle::new(self, active)
    }
}

pub mod toggle {
    use gpui::elements::{GeneralComponent, StyleableComponent};

    use crate::Toggleable;

    pub struct Toggle<C, S> {
        style: S,
        active: bool,
        component: C,
    }

    impl<C: StyleableComponent> Toggle<C, ()> {
        pub fn new(component: C, active: bool) -> Self {
            Toggle {
                active,
                component,
                style: (),
            }
        }
    }

    impl<C: StyleableComponent> StyleableComponent for Toggle<C, ()> {
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

    impl<C: StyleableComponent> GeneralComponent for Toggle<C, Toggleable<C::Style>> {
        fn render<V: gpui::View>(
            self,
            v: &mut V,
            cx: &mut gpui::ViewContext<V>,
        ) -> gpui::AnyElement<V> {
            self.component
                .with_style(self.style.in_state(self.active).clone())
                .render(v, cx)
        }
    }
}

pub mod action_button {
    use std::borrow::Cow;

    use gpui::{
        elements::{
            ContainerStyle, GeneralComponent, MouseEventHandler, StyleableComponent, TooltipStyle,
        },
        platform::{CursorStyle, MouseButton},
        Action, Element, TypeTag, View,
    };
    use schemars::JsonSchema;
    use serde_derive::Deserialize;

    use crate::Interactive;

    pub struct ActionButton<C, S> {
        action: Box<dyn Action>,
        tooltip: Cow<'static, str>,
        tooltip_style: TooltipStyle,
        tag: TypeTag,
        contents: C,
        style: Interactive<S>,
    }

    #[derive(Clone, Deserialize, Default, JsonSchema)]
    pub struct ButtonStyle<C> {
        #[serde(flatten)]
        container: ContainerStyle,
        button_width: Option<f32>,
        button_height: Option<f32>,
        #[serde(flatten)]
        contents: C,
    }

    impl ActionButton<(), ()> {
        pub fn new_dynamic(
            action: Box<dyn Action>,
            tooltip: impl Into<Cow<'static, str>>,
            tooltip_style: TooltipStyle,
        ) -> Self {
            Self {
                contents: (),
                tag: action.type_tag(),
                style: Interactive::new_blank(),
                tooltip: tooltip.into(),
                tooltip_style,
                action,
            }
        }

        pub fn new<A: Action + Clone>(
            action: A,
            tooltip: impl Into<Cow<'static, str>>,
            tooltip_style: TooltipStyle,
        ) -> Self {
            Self::new_dynamic(Box::new(action), tooltip, tooltip_style)
        }

        pub fn with_contents<C: StyleableComponent>(self, contents: C) -> ActionButton<C, ()> {
            ActionButton {
                action: self.action,
                tag: self.tag,
                style: self.style,
                tooltip: self.tooltip,
                tooltip_style: self.tooltip_style,
                contents,
            }
        }
    }

    impl<C: StyleableComponent> StyleableComponent for ActionButton<C, ()> {
        type Style = Interactive<ButtonStyle<C::Style>>;
        type Output = ActionButton<C, ButtonStyle<C::Style>>;

        fn with_style(self, style: Self::Style) -> Self::Output {
            ActionButton {
                action: self.action,
                tag: self.tag,
                contents: self.contents,
                tooltip: self.tooltip,
                tooltip_style: self.tooltip_style,
                style,
            }
        }
    }

    impl<C: StyleableComponent> GeneralComponent for ActionButton<C, ButtonStyle<C::Style>> {
        fn render<V: View>(self, v: &mut V, cx: &mut gpui::ViewContext<V>) -> gpui::AnyElement<V> {
            MouseEventHandler::new_dynamic(self.tag, 0, cx, |state, cx| {
                let style = self.style.style_for(state);
                let mut contents = self
                    .contents
                    .with_style(style.contents.to_owned())
                    .render(v, cx)
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
                move |_, _, cx| {
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
            .with_dynamic_tooltip(
                self.tag,
                0,
                self.tooltip,
                Some(self.action),
                self.tooltip_style,
                cx,
            )
            .into_any()
        }
    }
}

pub mod svg {
    use std::borrow::Cow;

    use gpui::{
        elements::{GeneralComponent, StyleableComponent},
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
            };

            Ok(result)
        }
    }

    pub struct Svg<S> {
        path: Cow<'static, str>,
        style: S,
    }

    impl Svg<()> {
        pub fn new(path: impl Into<Cow<'static, str>>) -> Self {
            Self {
                path: path.into(),
                style: (),
            }
        }
    }

    impl StyleableComponent for Svg<()> {
        type Style = SvgStyle;

        type Output = Svg<SvgStyle>;

        fn with_style(self, style: Self::Style) -> Self::Output {
            Svg {
                path: self.path,
                style,
            }
        }
    }

    impl GeneralComponent for Svg<SvgStyle> {
        fn render<V: gpui::View>(
            self,
            _: &mut V,
            _: &mut gpui::ViewContext<V>,
        ) -> gpui::AnyElement<V> {
            gpui::elements::Svg::new(self.path)
                .with_color(self.style.color)
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
        elements::{GeneralComponent, LabelStyle, StyleableComponent},
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

    impl StyleableComponent for Label<()> {
        type Style = LabelStyle;

        type Output = Label<LabelStyle>;

        fn with_style(self, style: Self::Style) -> Self::Output {
            Label {
                text: self.text,
                style,
            }
        }
    }

    impl GeneralComponent for Label<LabelStyle> {
        fn render<V: gpui::View>(
            self,
            _: &mut V,
            _: &mut gpui::ViewContext<V>,
        ) -> gpui::AnyElement<V> {
            gpui::elements::Label::new(self.text, self.style).into_any()
        }
    }
}

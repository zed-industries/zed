use gpui::{overlay, Action, AnyView, Overlay, Render, RenderOnce, VisualContext};
use settings2::Settings;
use theme2::{ActiveTheme, ThemeSettings};

use crate::prelude::*;
use crate::{h_stack, v_stack, Color, KeyBinding, Label, LabelSize, StyledExt};

pub struct Tooltip {
    title: SharedString,
    meta: Option<SharedString>,
    key_binding: Option<KeyBinding>,
}

impl Tooltip {
    pub fn text(title: impl Into<SharedString>, cx: &mut WindowContext) -> AnyView {
        cx.build_view(|cx| Self {
            title: title.into(),
            meta: None,
            key_binding: None,
        })
        .into()
    }

    pub fn for_action(
        title: impl Into<SharedString>,
        action: &dyn Action,
        cx: &mut WindowContext,
    ) -> AnyView {
        cx.build_view(|cx| Self {
            title: title.into(),
            meta: None,
            key_binding: KeyBinding::for_action(action, cx),
        })
        .into()
    }

    pub fn with_meta(
        title: impl Into<SharedString>,
        action: Option<&dyn Action>,
        meta: impl Into<SharedString>,
        cx: &mut WindowContext,
    ) -> AnyView {
        cx.build_view(|cx| Self {
            title: title.into(),
            meta: Some(meta.into()),
            key_binding: action.and_then(|action| KeyBinding::for_action(action, cx)),
        })
        .into()
    }

    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            meta: None,
            key_binding: None,
        }
    }

    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn key_binding(mut self, key_binding: impl Into<Option<KeyBinding>>) -> Self {
        self.key_binding = key_binding.into();
        self
    }
}

impl Render for Tooltip {
    type Element = Overlay;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let ui_font = ThemeSettings::get_global(cx).ui_font.family.clone();
        overlay().child(
            // padding to avoid mouse cursor
            div().pl_2().pt_2p5().child(
                v_stack()
                    .elevation_2(cx)
                    .font(ui_font)
                    .text_ui_sm()
                    .text_color(cx.theme().colors().text)
                    .py_1()
                    .px_2()
                    .child(
                        h_stack()
                            .child(self.title.clone())
                            .when_some(self.key_binding.clone(), |this, key_binding| {
                                this.justify_between().child(key_binding)
                            }),
                    )
                    .when_some(self.meta.clone(), |this, meta| {
                        this.child(Label::new(meta).size(LabelSize::Small).color(Color::Muted))
                    }),
            ),
        )
    }
}

use std::borrow::Borrow;
use std::rc::Rc;

use gpui::{Action, AnyElement, AnyView, AppContext, FocusHandle, IntoElement, Render};
use settings::Settings;
use theme::ThemeSettings;

use crate::prelude::*;
use crate::{Color, KeyBinding, Label, LabelSize, StyledExt, h_flex, v_flex};

#[derive(RegisterComponent)]
pub struct Tooltip {
    title: Title,
    meta: Option<SharedString>,
    key_binding: Option<KeyBinding>,
}

#[derive(Clone, IntoElement)]
enum Title {
    Str(SharedString),
    Callback(Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>),
}

impl From<SharedString> for Title {
    fn from(value: SharedString) -> Self {
        Title::Str(value)
    }
}

impl RenderOnce for Title {
    fn render(self, window: &mut Window, cx: &mut App) -> impl gpui::IntoElement {
        match self {
            Title::Str(title) => title.into_any_element(),
            Title::Callback(element) => element(window, cx),
        }
    }
}

impl Tooltip {
    pub fn simple(title: impl Into<SharedString>, cx: &mut App) -> AnyView {
        cx.new(|_| Self {
            title: Title::Str(title.into()),
            meta: None,
            key_binding: None,
        })
        .into()
    }

    pub fn text(title: impl Into<SharedString>) -> impl Fn(&mut Window, &mut App) -> AnyView {
        let title = title.into();
        move |_, cx| {
            cx.new(|_| Self {
                title: title.clone().into(),
                meta: None,
                key_binding: None,
            })
            .into()
        }
    }

    pub fn for_action_title<T: Into<SharedString>>(
        title: T,
        action: &dyn Action,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView + use<T> {
        let title = title.into();
        let action = action.boxed_clone();
        move |_, cx| {
            cx.new(|cx| Self {
                title: Title::Str(title.clone()),
                meta: None,
                key_binding: Some(KeyBinding::for_action(action.as_ref(), cx)),
            })
            .into()
        }
    }

    pub fn for_action_title_in<Str: Into<SharedString>>(
        title: Str,
        action: &dyn Action,
        focus_handle: &FocusHandle,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView + use<Str> {
        let title = title.into();
        let action = action.boxed_clone();
        let focus_handle = focus_handle.clone();
        move |_, cx| {
            cx.new(|cx| Self {
                title: Title::Str(title.clone()),
                meta: None,
                key_binding: Some(KeyBinding::for_action_in(
                    action.as_ref(),
                    &focus_handle,
                    cx,
                )),
            })
            .into()
        }
    }

    pub fn for_action(
        title: impl Into<SharedString>,
        action: &dyn Action,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| Self {
            title: Title::Str(title.into()),
            meta: None,
            key_binding: Some(KeyBinding::for_action(action, cx)),
        })
        .into()
    }

    pub fn for_action_in(
        title: impl Into<SharedString>,
        action: &dyn Action,
        focus_handle: &FocusHandle,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| Self {
            title: title.into().into(),
            meta: None,
            key_binding: Some(KeyBinding::for_action_in(action, focus_handle, cx)),
        })
        .into()
    }

    pub fn with_meta(
        title: impl Into<SharedString>,
        action: Option<&dyn Action>,
        meta: impl Into<SharedString>,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| Self {
            title: title.into().into(),
            meta: Some(meta.into()),
            key_binding: action.map(|action| KeyBinding::for_action(action, cx)),
        })
        .into()
    }

    pub fn with_meta_in(
        title: impl Into<SharedString>,
        action: Option<&dyn Action>,
        meta: impl Into<SharedString>,
        focus_handle: &FocusHandle,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| Self {
            title: title.into().into(),
            meta: Some(meta.into()),
            key_binding: action.map(|action| KeyBinding::for_action_in(action, focus_handle, cx)),
        })
        .into()
    }

    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into().into(),
            meta: None,
            key_binding: None,
        }
    }

    pub fn new_element(title: impl Fn(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        Self {
            title: Title::Callback(Rc::new(title)),
            meta: None,
            key_binding: None,
        }
    }

    pub fn element(
        title: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView {
        let title = Title::Callback(Rc::new(title));
        move |_, cx| {
            let title = title.clone();
            cx.new(|_| Self {
                title,
                meta: None,
                key_binding: None,
            })
            .into()
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(cx, |el, _| {
            el.child(
                h_flex()
                    .gap_4()
                    .child(div().max_w_72().child(self.title.clone()))
                    .when_some(self.key_binding.clone(), |this, key_binding| {
                        this.justify_between().child(key_binding)
                    }),
            )
            .when_some(self.meta.clone(), |this, meta| {
                this.child(
                    div()
                        .max_w_72()
                        .child(Label::new(meta).size(LabelSize::Small).color(Color::Muted)),
                )
            })
        })
    }
}

pub fn tooltip_container<C>(cx: &mut C, f: impl FnOnce(Div, &mut C) -> Div) -> impl IntoElement
where
    C: AppContext + Borrow<App>,
{
    let app = (*cx).borrow();
    let ui_font = ThemeSettings::get_global(app).ui_font.clone();

    // padding to avoid tooltip appearing right below the mouse cursor
    div().pl_2().pt_2p5().child(
        v_flex()
            .elevation_2(app)
            .font(ui_font)
            .text_ui(app)
            .text_color(app.theme().colors().text)
            .py_1()
            .px_2()
            .map(|el| f(el, cx)),
    )
}

pub struct LinkPreview {
    link: SharedString,
}

impl LinkPreview {
    pub fn new(url: &str, cx: &mut App) -> AnyView {
        let mut wrapped_url = String::new();
        for (i, ch) in url.chars().enumerate() {
            if i == 500 {
                wrapped_url.push('…');
                break;
            }
            if i % 100 == 0 && i != 0 {
                wrapped_url.push('\n');
            }
            wrapped_url.push(ch);
        }
        cx.new(|_| LinkPreview {
            link: wrapped_url.into(),
        })
        .into()
    }
}

impl Render for LinkPreview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(cx, |el, _| {
            el.child(
                Label::new(self.link.clone())
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
        })
    }
}

impl Component for Tooltip {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn description() -> Option<&'static str> {
        Some(
            "A tooltip that appears when hovering over an element, optionally showing a keybinding or additional metadata.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            example_group(vec![single_example(
                "Text only",
                Button::new("delete-example", "Delete")
                    .tooltip(Tooltip::text("This is a tooltip!"))
                    .into_any_element(),
            )])
            .into_any_element(),
        )
    }
}

use gpui::{
    point, App, AppContext, Context, Element, IntoElement, PlatformDisplay, Size, Window,
    WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowHandle, WindowKind,
    WindowOptions,
};
use pop_up::PopUp;
use release_channel::ReleaseChannel;
use std::{marker::PhantomData, rc::Rc};
use theme;
use ui::{prelude::*, Pixels, Render};
use util::ResultExt;

#[derive(Default)]
pub struct ToolReadyPopUp;

impl Render for ToolReadyPopUp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);

        h_flex()
            .text_ui(cx)
            .justify_between()
            .size_full()
            .overflow_hidden()
            .elevation_3(cx)
            .p_2()
            .gap_2()
            .font(ui_font)
            .child(
                v_flex()
                    .overflow_hidden()
                    .child(Label::new("Tool is ready to use")),
            )
            .child(
                v_flex()
                    .child(Button::new("open", "Open").on_click(cx.listener(
                        move |this, _event, _, cx| {
                            println!("TODO open");
                        },
                    )))
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        move |this, _event, _, cx| {
                            println!("TODO dismiss");
                        },
                    ))),
            )
    }
}

use gpui::{img, prelude::*, AnyElement, SharedUri};
use smallvec::SmallVec;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct CollabNotification {
    avatar_uri: SharedUri,
    accept_button: Button,
    dismiss_button: Button,
    children: SmallVec<[AnyElement; 2]>,
}

impl CollabNotification {
    pub fn new(
        avatar_uri: impl Into<SharedUri>,
        accept_button: Button,
        dismiss_button: Button,
    ) -> Self {
        Self {
            avatar_uri: avatar_uri.into(),
            accept_button,
            dismiss_button,
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for CollabNotification {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for CollabNotification {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .text_ui(cx)
            .justify_between()
            .size_full()
            .overflow_hidden()
            .elevation_3(cx)
            .p_2()
            .gap_2()
            .child(img(self.avatar_uri).w_12().h_12().rounded_full())
            .child(v_flex().overflow_hidden().children(self.children))
            .child(
                v_flex()
                    .child(self.accept_button)
                    .child(self.dismiss_button),
            )
    }
}

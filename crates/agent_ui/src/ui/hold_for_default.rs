use gpui::{App, IntoElement, Modifiers, RenderOnce, Window};
use ui::{prelude::*, render_modifiers};

#[derive(IntoElement)]
pub struct HoldForDefault {
    is_default: bool,
}

impl HoldForDefault {
    pub fn new(is_default: bool) -> Self {
        Self { is_default }
    }
}

impl RenderOnce for HoldForDefault {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .pt_1()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .gap_0p5()
            .text_sm()
            .text_color(Color::Muted.color(cx))
            .child("Hold")
            .child(h_flex().flex_shrink_0().children(render_modifiers(
                &Modifiers::secondary_key(),
                PlatformStyle::platform(),
                None,
                Some(TextSize::Default.rems(cx).into()),
                true,
            )))
            .child(div().map(|this| {
                if self.is_default {
                    this.child("to unset as default")
                } else {
                    this.child("to set as default")
                }
            }))
    }
}

use ui::{prelude::*, Avatar, IconButtonShape};

#[derive(IntoElement)]
pub struct ChatNotice {
    message: SharedString,
    meta: Option<SharedString>,
}

impl ChatNotice {
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            meta: None,
        }
    }

    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }
}

impl RenderOnce for ChatNotice {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .w_full()
            .items_start()
            .mt_4()
            .gap_3()
            .child(
                // TODO: Replace with question mark.
                Avatar::new("https://zed.dev/assistant_avatar.png").size(rems_from_px(20.)),
            )
            .child(
                v_flex()
                    .size_full()
                    .gap_1()
                    .pr_4()
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .justify_between()
                            .overflow_hidden()
                            .child(
                                h_flex()
                                    .flex_none()
                                    .overflow_hidden()
                                    .child(Label::new(self.message)),
                            )
                            .child(
                                h_flex()
                                    .flex_shrink_0()
                                    .gap_1()
                                    .child(Button::new("allow", "Allow"))
                                    .child(
                                        IconButton::new("deny", IconName::Close)
                                            .shape(IconButtonShape::Square)
                                            .icon_color(Color::Muted)
                                            .size(ButtonSize::None)
                                            .icon_size(IconSize::XSmall),
                                    ),
                            ),
                    )
                    .children(
                        self.meta.map(|meta| {
                            Label::new(meta).size(LabelSize::Small).color(Color::Muted)
                        }),
                    ),
            )
    }
}

use crate::{Avatar, prelude::*};
use gpui::{AnyElement, ImageSource, IntoElement};

#[derive(IntoElement, RegisterComponent)]
pub struct CollabOverlayControls {
    avatar: ImageSource,
    is_open: bool,
}

impl CollabOverlayControls {
    pub fn new(avatar: impl Into<ImageSource>) -> Self {
        Self {
            avatar: avatar.into(),
            is_open: false,
        }
    }

    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }
}

impl RenderOnce for CollabOverlayControls {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .py_1()
            .px_2()
            .w_full()
            .gap_1()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().surface_background)
            .child(
                h_flex().gap_1().child(Avatar::new(self.avatar)).child(
                    h_flex()
                        .child(IconButton::new("mic", IconName::Mic).icon_size(IconSize::Small))
                        .child(
                            IconButton::new("audio", IconName::AudioOn).icon_size(IconSize::Small),
                        )
                        .child(
                            IconButton::new("screen", IconName::Screen).icon_size(IconSize::Small),
                        ),
                ),
            )
            .child(
                Button::new("leave", "Leave")
                    .label_size(LabelSize::Small)
                    .icon(IconName::Exit)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted),
            )
    }
}

impl Component for CollabOverlayControls {
    fn scope() -> ComponentScope {
        ComponentScope::Collaboration
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let ex_container = h_flex()
            .w_80()
            .border_1()
            .border_color(cx.theme().colors().border);

        let examples = vec![single_example(
            "Default",
            ex_container
                .child(
                    CollabOverlayControls::new(
                        "https://avatars.githubusercontent.com/u/67129314?v=4",
                    )
                    .is_open(true),
                )
                .into_any_element(),
        )];

        Some(example_group(examples).vertical().into_any_element())
    }
}

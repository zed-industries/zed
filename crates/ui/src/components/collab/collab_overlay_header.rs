use crate::prelude::*;
use gpui::{AnyElement, IntoElement, ParentElement, Styled};

#[derive(IntoElement, RegisterComponent)]
pub struct CollabOverlayHeader {
    channel_name: SharedString,
    is_open: bool,
}

impl CollabOverlayHeader {
    pub fn new(channel_name: impl Into<SharedString>) -> Self {
        Self {
            channel_name: channel_name.into(),
            is_open: false,
        }
    }

    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }
}

impl RenderOnce for CollabOverlayHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let chevron = if self.is_open {
            IconName::ChevronDown
        } else {
            IconName::ChevronUp
        };

        h_flex()
            .py_1()
            .px_2()
            .w_full()
            .gap_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().surface_background)
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::AudioOn)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new(self.channel_name)),
            )
            .child(Icon::new(chevron).size(IconSize::Small).color(Color::Muted))
    }
}

impl Component for CollabOverlayHeader {
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
                .child(CollabOverlayHeader::new("Admin Dashboard v2").is_open(true))
                .into_any_element(),
        )];

        Some(example_group(examples).vertical().into_any_element())
    }
}

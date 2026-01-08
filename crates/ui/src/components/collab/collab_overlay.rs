use crate::{CollabOverlayControls, CollabOverlayHeader, ParticipantItem, prelude::*};
use gpui::{AnyElement, Empty, IntoElement};

#[derive(IntoElement, RegisterComponent)]
pub struct CollabOverlay {
    header: AnyElement,
    children: Vec<AnyElement>,
    controls: AnyElement,
}

impl CollabOverlay {
    pub fn new() -> Self {
        Self {
            header: Empty.into_any_element(),
            children: Vec::new(),
            controls: Empty.into_any_element(),
        }
    }

    pub fn header(mut self, element: impl IntoElement) -> Self {
        self.header = element.into_any_element();
        self
    }

    pub fn children(mut self, elements: Vec<AnyElement>) -> Self {
        self.children = elements;
        self
    }

    pub fn controls(mut self, element: impl IntoElement) -> Self {
        self.controls = element.into_any_element();
        self
    }
}

impl RenderOnce for CollabOverlay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .child(self.header)
            .children(self.children)
            .child(self.controls)
    }
}

impl Component for CollabOverlay {
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
                    CollabOverlay::new()
                        .header(CollabOverlayHeader::new("Admin Dashboard v2").is_open(true))
                        .children(vec![
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                            ParticipantItem::new("Matt").into_any_element(),
                        ])
                        .controls(
                            CollabOverlayControls::new(
                                "https://avatars.githubusercontent.com/u/67129314?v=4",
                            )
                            .is_open(true),
                        ),
                )
                .into_any_element(),
        )];

        Some(example_group(examples).vertical().into_any_element())
    }
}

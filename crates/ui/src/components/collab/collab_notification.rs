use gpui::{AnyElement, SharedUri, prelude::*};
use smallvec::SmallVec;

use crate::{Avatar, prelude::*};

#[derive(IntoElement, RegisterComponent)]
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
            .p_2()
            .size_full()
            .text_ui(cx)
            .justify_between()
            .overflow_hidden()
            .elevation_3(cx)
            .gap_1()
            .child(
                h_flex()
                    .min_w_0()
                    .gap_4()
                    .child(Avatar::new(self.avatar_uri).size(px(40.)))
                    .child(v_flex().truncate().children(self.children)),
            )
            .child(
                v_flex()
                    .items_center()
                    .child(self.accept_button)
                    .child(self.dismiss_button),
            )
    }
}

impl Component for CollabNotification {
    fn scope() -> ComponentScope {
        ComponentScope::Collaboration
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let avatar = "https://avatars.githubusercontent.com/u/67129314?v=4";
        let container = || div().h(px(72.)).w(px(400.)); // Size of the actual notification window

        let examples = vec![
            single_example(
                "Incoming Call",
                container()
                    .child(
                        CollabNotification::new(
                            avatar,
                            Button::new("accept", "Accept"),
                            Button::new("decline", "Decline"),
                        )
                        .child(Label::new("the user is inviting you to a call")),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Screen Share Request",
                container()
                    .child(
                        CollabNotification::new(
                            avatar,
                            Button::new("accept", "View"),
                            Button::new("decline", "Ignore"),
                        )
                        .child(Label::new("the user is sharing their screen")),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Project Shared",
                container()
                    .child(
                        CollabNotification::new(
                            avatar,
                            Button::new("accept", "Open"),
                            Button::new("decline", "Dismiss"),
                        )
                        .child(Label::new("the user is sharing a project"))
                        .child(Label::new("zed").color(Color::Muted)),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Overflowing Content",
                container()
                    .child(
                        CollabNotification::new(
                            avatar,
                            Button::new("accept", "Accept"),
                            Button::new("decline", "Decline"),
                        )
                        .child(Label::new(
                            "a_very_long_username_that_might_overflow is sharing a project in Zed:",
                        ))
                        .child(
                            Label::new("zed-cloud, zed, edit-prediction-bench, zed.dev")
                                .color(Color::Muted),
                        ),
                    )
                    .into_any_element(),
            ),
        ];

        Some(example_group(examples).vertical().into_any_element())
    }
}

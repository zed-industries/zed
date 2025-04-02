use gpui::{AnyElement, SharedUri, img, prelude::*};
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

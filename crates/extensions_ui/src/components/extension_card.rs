use gpui::{AnyElement, prelude::*};
use smallvec::SmallVec;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct ExtensionCard {
    overridden_by_dev_extension: bool,
    children: SmallVec<[AnyElement; 2]>,
}

impl ExtensionCard {
    pub fn new() -> Self {
        Self {
            overridden_by_dev_extension: false,
            children: SmallVec::new(),
        }
    }

    pub fn overridden_by_dev_extension(mut self, overridden: bool) -> Self {
        self.overridden_by_dev_extension = overridden;
        self
    }
}

impl ParentElement for ExtensionCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ExtensionCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().w_full().child(
            v_flex()
                .w_full()
                .h(rems(7.))
                .p_3()
                .mt_4()
                .gap_2()
                .bg(cx.theme().colors().elevated_surface_background)
                .border_1()
                .border_color(cx.theme().colors().border)
                .rounded_md()
                .children(self.children)
                .when(self.overridden_by_dev_extension, |card| {
                    card.child(
                        h_flex()
                            .absolute()
                            .top_0()
                            .left_0()
                            .block_mouse_down()
                            .cursor_default()
                            .size_full()
                            .items_center()
                            .justify_center()
                            .bg(cx.theme().colors().elevated_surface_background.alpha(0.8))
                            .child(Label::new("Overridden by dev extension.")),
                    )
                }),
        )
    }
}

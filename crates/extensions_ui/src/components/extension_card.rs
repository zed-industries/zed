use gpui::{prelude::*, AnyElement};
use smallvec::SmallVec;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct ExtensionCard {
    children: SmallVec<[AnyElement; 2]>,
}

impl ExtensionCard {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for ExtensionCard {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ExtensionCard {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
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
                .children(self.children),
        )
    }
}

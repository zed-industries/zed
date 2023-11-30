use gpui::{div, overlay, px, AnyElement, Div, ParentElement, RenderOnce, Styled, WindowContext};
use smallvec::SmallVec;

use crate::{prelude::*, Popover};

// 🚧 Under Construction

#[derive(IntoElement)]
pub struct PopoverMenu {
    trigger: AnyElement,
    children: SmallVec<[AnyElement; 2]>,
}

impl RenderOnce for PopoverMenu {
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        div()
            .bg(gpui::green())
            .relative()
            .child(div().bg(gpui::blue()).child(self.trigger))
            .child(
                overlay()
                    .position(gpui::Point {
                        x: px(100.),
                        y: px(100.),
                    })
                    .anchor(gpui::AnchorCorner::TopRight)
                    .child(Popover::new().children(self.children)),
            )
    }
}

impl PopoverMenu {
    pub fn new(trigger: AnyElement) -> Self {
        Self {
            trigger,
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for PopoverMenu {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

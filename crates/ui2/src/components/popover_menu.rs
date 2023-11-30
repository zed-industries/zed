use gpui::{div, overlay, AnyElement, Div, ParentElement, RenderOnce, Styled, WindowContext};
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
            .relative()
            .child(self.trigger)
            .child(overlay().child(Popover::new().children(self.children)))
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

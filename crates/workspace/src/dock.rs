use gpui::{elements::ChildView, Element, ElementBox, ViewContext, ViewHandle};
use theme::Theme;

use crate::{Pane, Workspace};

#[derive(PartialEq, Eq)]
pub enum DockPosition {
    Bottom,
    Right,
    Fullscreen,
    Hidden,
}

pub struct Dock {
    position: DockPosition,
    pane: ViewHandle<Pane>,
}

impl Dock {
    pub fn new(cx: &mut ViewContext<Workspace>) -> Self {
        let pane = cx.add_view(Pane::new);
        Self {
            pane,
            position: DockPosition::Bottom,
        }
    }

    pub fn render(&self, _theme: &Theme, position: DockPosition) -> Option<ElementBox> {
        if position == self.position {
            Some(ChildView::new(self.pane.clone()).boxed())
        } else {
            None
        }
    }
}

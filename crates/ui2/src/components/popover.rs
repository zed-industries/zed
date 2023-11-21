use gpui::{
    AnyElement, Component, Div, Element, ElementId, ParentElement, RenderOnce, Styled,
    WindowContext,
};
use smallvec::SmallVec;

use crate::{v_stack, StyledExt};

#[derive(RenderOnce)]
pub struct Popover {
    children: SmallVec<[AnyElement; 2]>,
    aside: Option<AnyElement>,
}

impl Component for Popover {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        v_stack()
            .relative()
            .elevation_2(cx)
            .p_1()
            .children(self.children)
            .when_some(self.aside, |this, aside| {
                // TODO: This will statically position the aside to the top right of the popover.
                // We should update this to avoid collisions with the window edges.
                this.child(
                    v_stack()
                        .top_0()
                        .neg_right_1()
                        .absolute()
                        .elevation_2(cx)
                        .p_1()
                        .child(aside),
                )
            })
    }
}

impl Popover {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
            aside: None,
        }
    }

    pub fn aside(mut self, aside: impl RenderOnce) -> Self
    where
        Self: Sized,
    {
        self.aside = Some(aside.render_once().into_any());
        self
    }
}

impl ParentElement for Popover {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

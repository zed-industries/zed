use gpui::{
    div, overlay, rems, AnchorCorner, AnyElement, Div, ParentElement, RenderOnce, Styled,
    WindowContext,
};
use smallvec::SmallVec;

use crate::{prelude::*, Popover};

#[derive(IntoElement)]
pub struct PopoverMenu {
    /// The element that triggers the popover menu when clicked
    /// Usually a button
    trigger: AnyElement,
    /// The content of the popover menu
    /// This will automatically be wrapped in a [Popover] element
    children: SmallVec<[AnyElement; 2]>,
    /// The direction the popover menu will open by default
    ///
    /// When not enough space is available in the default direction,
    /// the popover menu will follow the rules of [gpui2::elements::overlay]
    anchor: AnchorCorner,
    /// Whether the popover menu is currently open
    show_menu: bool,
}

impl RenderOnce for PopoverMenu {
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        // Default offset = 4px padding + 1px border
        let offset = 5. / 16.;

        let (top, right, bottom, left) = match self.anchor {
            AnchorCorner::TopRight => (None, Some(-offset), Some(-offset), None),
            AnchorCorner::TopLeft => (None, None, Some(-offset), Some(-offset)),
            AnchorCorner::BottomRight => (Some(-offset), Some(-offset), None, None),
            AnchorCorner::BottomLeft => (Some(-offset), None, None, Some(-offset)),
        };

        div()
            .flex()
            .flex_none()
            .bg(gpui::green())
            .relative()
            .child(
                div()
                    .flex_none()
                    .relative()
                    .bg(gpui::blue())
                    .child(self.trigger),
            )
            .when(self.show_menu, |this| {
                this.child(
                    div()
                        .absolute()
                        .size_0()
                        .when_some(top, |this, t| this.top(rems(t)))
                        .when_some(right, |this, r| this.right(rems(r)))
                        .when_some(bottom, |this, b| this.bottom(rems(b)))
                        .when_some(left, |this, l| this.left(rems(l)))
                        .child(
                            overlay()
                                .anchor(AnchorCorner::TopRight)
                                .child(Popover::new().children(self.children)),
                        ),
                )
            })
    }
}

impl PopoverMenu {
    pub fn new(trigger: AnyElement) -> Self {
        Self {
            trigger,
            children: SmallVec::new(),
            anchor: AnchorCorner::TopLeft,
            show_menu: false,
        }
    }

    pub fn anchor(mut self, anchor: AnchorCorner) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn show_menu(mut self, show_menu: bool) -> Self {
        self.show_menu = show_menu;
        self
    }
}

impl ParentElement for PopoverMenu {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

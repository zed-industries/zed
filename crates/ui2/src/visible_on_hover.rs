use gpui::{InteractiveElement, SharedString, Styled};

pub trait VisibleOnHover: InteractiveElement + Styled + Sized {
    /// Sets the element to only be visible when the specified group is hovered.
    ///
    /// Pass `""` as the `group_name` to use the global group.
    fn visible_on_hover(self, group_name: impl Into<SharedString>) -> Self {
        self.invisible()
            .group_hover(group_name, |style| style.visible())
    }
}

impl<E: InteractiveElement + Styled> VisibleOnHover for E {}

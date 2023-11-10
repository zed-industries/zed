use gpui::{Div, ElementInteractivity, KeyDispatch, Styled};

use crate::UITextSize;

/// Extends [`Styled`](gpui::Styled) with Zed specific styling methods.
pub trait StyledExt: Styled {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn h_flex(self) -> Self
    where
        Self: Sized,
    {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    ///
    /// Sets `flex()`, `flex_col()`
    fn v_flex(self) -> Self
    where
        Self: Sized,
    {
        self.flex().flex_col()
    }

    fn text_ui_size(self, size: UITextSize) -> Self
    where
        Self: Sized,
    {
        let size = size.rems();

        self.text_size(size)
    }

    /// The default size for UI text.
    ///
    /// `0.825rem` or `14px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use [`text_ui_sm`] for regular-sized text.
    fn text_ui(self) -> Self
    where
        Self: Sized,
    {
        let size = UITextSize::default().rems();

        self.text_size(size)
    }

    /// The small size for UI text.
    ///
    /// `0.75rem` or `12px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use [`text_ui`] for regular-sized text.
    fn text_ui_sm(self) -> Self
    where
        Self: Sized,
    {
        let size = UITextSize::Small.rems();

        self.text_size(size)
    }
}

impl<V, I, F> StyledExt for Div<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
}

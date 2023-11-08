use gpui::{Div, Styled};

use crate::UITextSize;

/// Extends [`Styled`](gpui::Styled) with Zed specific styling methods.
pub trait StyledExt {
    fn text_ui_size(self, size: UITextSize) -> Self;
    fn text_ui(self) -> Self;
    fn text_ui_sm(self) -> Self;
}

impl<V: 'static> StyledExt for Div<V> {
    fn text_ui_size(self, size: UITextSize) -> Self {
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
    fn text_ui(self) -> Self {
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
    fn text_ui_sm(self) -> Self {
        let size = UITextSize::Small.rems();

        self.text_size(size)
    }
}

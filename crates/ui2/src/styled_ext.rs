use gpui::Styled;

use crate::UITextSize;

pub trait StyledExt: Styled {
    fn text_ui_size(self, size: UITextSize) -> Self
    where
        Self: Sized,
    {
        let size = size.rems();

        self.text_size(size)
    }
    fn text_ui(self) -> Self
    where
        Self: Sized,
    {
        let size = UITextSize::default().rems();

        self.text_size(size)
    }
    fn text_ui_sm(self) -> Self
    where
        Self: Sized,
    {
        let size = UITextSize::Small.rems();

        self.text_size(size)
    }
}

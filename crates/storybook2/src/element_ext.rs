use crate::theme::{Theme, Themed};
use gpui3::Element;

pub trait ElementExt: Element {
    fn themed(self, theme: Theme) -> Themed<Self>
    where
        Self: Sized;
}

impl<E: Element> ElementExt for E {
    fn themed(self, theme: Theme) -> Themed<Self>
    where
        Self: Sized,
    {
        Themed { child: self, theme }
    }
}

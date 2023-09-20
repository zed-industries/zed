use crate::theme::{Theme, Themed};
use gpui3::Element;
use std::marker::PhantomData;

pub trait ElementExt: Element {
    fn themed(self, theme: Theme) -> Themed<V, Self>
    where
        Self: Sized;
}

impl<V: 'static, E: Element> ElementExt for E {
    fn themed(self, theme: Theme) -> Themed<V, Self>
    where
        Self: Sized,
    {
        Themed {
            child: self,
            theme,
            view_type: PhantomData,
        }
    }
}

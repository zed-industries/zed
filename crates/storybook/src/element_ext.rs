use crate::theme::{Theme, Themed};
use gpui2::Element;
use std::marker::PhantomData;

pub trait ElementExt<V: 'static>: Element<V> {
    fn themed(self, theme: Theme) -> Themed<V, Self>
    where
        Self: Sized;
}

impl<V: 'static, E: Element<V>> ElementExt<V> for E {
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

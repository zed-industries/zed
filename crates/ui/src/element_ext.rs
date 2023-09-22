use std::marker::PhantomData;

use gpui2::Element;

use crate::theme::{Theme, Themed};

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

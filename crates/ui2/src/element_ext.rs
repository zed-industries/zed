use gpui2::Element;

pub trait ElementExt<V: 'static>: Element<V> {
    // fn when(mut self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    // where
    //     Self: Sized,
    // {
    //     if condition {
    //         self = then(self);
    //     }
    //     self
    // }

    // fn when_some<T, U>(mut self, option: Option<T>, then: impl FnOnce(Self, T) -> U) -> U
    // where
    //     Self: Sized,
    // {
    //     if let Some(value) = option {
    //         self = then(self, value);
    //     }
    //     self
    // }
}

impl<S: 'static, E: Element<S>> ElementExt<S> for E {}

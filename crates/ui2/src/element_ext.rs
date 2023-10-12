use gpui3::Element;

pub trait ElementExt<S: 'static + Send + Sync>: Element<ViewState = S> {
    /// Applies a given function `then` to the current element if `condition` is true.
    /// This function is used to conditionally modify the element based on a given condition.
    /// If `condition` is false, it just returns the current element as it is.
    fn when(mut self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        if condition {
            self = then(self);
        }
        self
    }
}

impl<S: 'static + Send + Sync, E: Element<ViewState = S>> ElementExt<S> for E {}

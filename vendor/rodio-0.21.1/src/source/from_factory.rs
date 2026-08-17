use crate::source::{from_iter, FromIter};

/// Builds a source that chains sources built from a factory.
///
/// The `factory` parameter is a function that produces a source. The source is then played.
/// Whenever the source ends, `factory` is called again in order to produce the source that is
/// played next.
///
/// If the `factory` closure returns `None`, then the sound ends.
pub fn from_factory<F, S>(factory: F) -> FromIter<FromFactoryIter<F>>
where
    F: FnMut() -> Option<S>,
{
    from_iter(FromFactoryIter { factory })
}

/// Internal type used by `from_factory`.
pub struct FromFactoryIter<F> {
    factory: F,
}

impl<F, S> Iterator for FromFactoryIter<F>
where
    F: FnMut() -> Option<S>,
{
    type Item = S;

    #[inline]
    fn next(&mut self) -> Option<S> {
        (self.factory)()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

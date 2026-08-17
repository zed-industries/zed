/// An iterator over an Error and its sources.
///
/// If you want to omit the initial error and only process its sources, use `skip(1)`.
///
/// Can be created via [`ErrorCompat::iter_chain`][crate::ErrorCompat::iter_chain].
#[derive(Debug, Clone)]
pub struct ChainCompat<'a, 'b> {
    inner: Option<&'a (dyn crate::Error + 'b)>,
}

impl<'a, 'b> ChainCompat<'a, 'b> {
    /// Creates a new error chain iterator.
    pub fn new(error: &'a (dyn crate::Error + 'b)) -> Self {
        ChainCompat { inner: Some(error) }
    }
}

impl<'a, 'b> Iterator for ChainCompat<'a, 'b> {
    type Item = &'a (dyn crate::Error + 'b);

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner {
            None => None,
            Some(e) => {
                self.inner = e.source();
                Some(e)
            }
        }
    }
}

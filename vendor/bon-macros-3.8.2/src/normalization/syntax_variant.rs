/// Struct, that contains both the original syntax (unprocessed) and the normalized
/// version. This is useful for code that needs access to both versions of the syntax.
#[derive(Debug)]
pub(crate) struct SyntaxVariant<T> {
    /// Original syntax that was passed to the macro without any modifications.
    pub(crate) orig: T,

    /// The value that is equivalent to `orig`, but it underwent normalization.
    pub(crate) norm: T,
}

impl<T> SyntaxVariant<T> {
    pub(crate) fn apply_ref<'a, U>(&'a self, f: impl Fn(&'a T) -> U) -> SyntaxVariant<U> {
        let orig = f(&self.orig);
        let norm = f(&self.norm);
        SyntaxVariant { orig, norm }
    }

    pub(crate) fn into_iter(self) -> impl Iterator<Item = SyntaxVariant<T::Item>>
    where
        T: IntoIterator,
    {
        self.orig
            .into_iter()
            .zip(self.norm)
            .map(|(orig, norm)| SyntaxVariant { orig, norm })
    }
}

use crate::util::prelude::*;
use syn::punctuated::Punctuated;

pub(crate) trait PunctuatedExt<T, P> {
    /// Remove all elements from the [`Punctuated`] that do not satisfy the predicate.
    /// Also bail on the first error that the predicate returns.
    fn try_retain_mut(&mut self, f: impl FnMut(&mut T) -> Result<bool>) -> Result;
}

impl<T, P> PunctuatedExt<T, P> for Punctuated<T, P>
where
    P: Default,
{
    fn try_retain_mut(&mut self, mut try_predicate: impl FnMut(&mut T) -> Result<bool>) -> Result {
        // Unforunatelly, there is no builtin `retain` or `remove` in `Punctuated`
        // so we just re-create it from scratch.
        for mut pair in std::mem::take(self).into_pairs() {
            if try_predicate(pair.value_mut())? {
                self.extend([pair]);
            }
        }
        Ok(())
    }
}

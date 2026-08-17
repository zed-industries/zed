use crate::util::prelude::*;

pub(crate) trait VecExt<T> {
    /// Remove all elements from the [`Vec`] that do not satisfy the predicate.
    /// Also bail on the first error that the predicate returns.
    fn try_retain_mut(&mut self, f: impl FnMut(&mut T) -> Result<bool>) -> Result;
}

impl<T> VecExt<T> for Vec<T> {
    fn try_retain_mut(&mut self, mut try_predicate: impl FnMut(&mut T) -> Result<bool>) -> Result {
        let mut i = 0;
        while i < self.len() {
            if try_predicate(self.get_mut(i).expect("BUG: index must be valid"))? {
                i += 1;
            } else {
                self.remove(i);
            }
        }
        Ok(())
    }
}

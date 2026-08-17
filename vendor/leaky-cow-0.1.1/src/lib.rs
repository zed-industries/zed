// This library is released under the same terms as Rust itself.

//! Leaky cows: convert a clone-on-write reference into a plain reference. If it's just the
//! plain reference, just return that. If it's the owned structure, leak it and return a
//! reference to it.

extern crate leak;
use leak::Leak;
use std::borrow::{Borrow, Cow, ToOwned};

pub trait LeakyCow<'a, B: ?Sized> {
    fn leak(self) -> &'a B;
}

impl<'a, B: ?Sized> LeakyCow<'a, B> for Cow<'a, B> where B: 'a + ToOwned, B::Owned: Leak<B> {
    fn leak(self) -> &'a B {
        match self {
            Cow::Owned(x) => x.leak(),
            Cow::Borrowed(x) => x,
        }
    }
}

#[cfg(test)]
mod test {
    use super::LeakyCow;
    use std::borrow::Cow;
    #[test]
    fn test_string() {
        let _ = {
            let c = Cow::Borrowed("Let it go!");
            c.leak();
        };
    }
}

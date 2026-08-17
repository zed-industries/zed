use crate::intern::{self, InternedString};
use std::path::Path;

pub(crate) trait InternedVec<T>
where
    T: ?Sized,
{
    fn vec(&self) -> Vec<&'static T>;
}

impl<T> InternedVec<T> for Vec<InternedString>
where
    T: ?Sized + Element,
{
    fn vec(&self) -> Vec<&'static T> {
        self.iter().copied().map(Element::unintern).collect()
    }
}

pub(crate) fn intern<T>(elements: &[&T]) -> Vec<InternedString>
where
    T: ?Sized + Element,
{
    elements.iter().copied().map(Element::intern).collect()
}

pub(crate) trait Element {
    fn intern(&self) -> InternedString;
    fn unintern(_: InternedString) -> &'static Self;
}

impl Element for str {
    fn intern(&self) -> InternedString {
        intern::intern(self)
    }

    fn unintern(interned: InternedString) -> &'static Self {
        interned.str()
    }
}

impl Element for Path {
    fn intern(&self) -> InternedString {
        intern::intern(&self.to_string_lossy())
    }

    fn unintern(interned: InternedString) -> &'static Self {
        Path::new(interned.str())
    }
}

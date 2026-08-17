use crate::mapref;
use core::hash::Hash;
use core::ops::Deref;

pub struct RefMulti<'a, K> {
    inner: mapref::multiple::RefMulti<'a, K, ()>,
}

impl<'a, K: Eq + Hash> RefMulti<'a, K> {
    pub(crate) fn new(inner: mapref::multiple::RefMulti<'a, K, ()>) -> Self {
        Self { inner }
    }

    pub fn key(&self) -> &K {
        self.inner.key()
    }
}

impl<'a, K: Eq + Hash> Deref for RefMulti<'a, K> {
    type Target = K;

    fn deref(&self) -> &K {
        self.key()
    }
}

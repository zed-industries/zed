use crate::mapref;
use core::hash::Hash;
use core::ops::Deref;

pub struct Ref<'a, K> {
    inner: mapref::one::Ref<'a, K, ()>,
}

impl<'a, K: Eq + Hash> Ref<'a, K> {
    pub(crate) fn new(inner: mapref::one::Ref<'a, K, ()>) -> Self {
        Self { inner }
    }

    pub fn key(&self) -> &K {
        self.inner.key()
    }
}

impl<'a, K: Eq + Hash> Deref for Ref<'a, K> {
    type Target = K;

    fn deref(&self) -> &K {
        self.key()
    }
}

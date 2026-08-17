#[macro_use]
mod doc;

macro_rules! impl_clone {
    () => {
        #[inline]
        fn clone(&self) -> Self {
            Self { inner: self.inner, _marker: PhantomData }
        }
    };
}

macro_rules! impl_debug {
    ($type_name:expr) => {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let (ptr, tag) = self.decompose();
            f.debug_struct($type_name).field("ptr", &ptr).field("tag", &tag).finish()
        }
    };
}

macro_rules! impl_default {
    () => {
        #[inline]
        fn default() -> Self {
            Self::null()
        }
    };
}

macro_rules! impl_pointer {
    () => {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Pointer::fmt(&self.decompose_ptr(), f)
        }
    };
}

macro_rules! impl_partial_eq {
    () => {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.inner.eq(&other.inner)
        }
    };
}

macro_rules! impl_partial_ord {
    () => {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.inner.partial_cmp(&other.inner)
        }
    };
}

macro_rules! impl_ord {
    () => {
        #[inline]
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.inner.cmp(&other.inner)
        }
    };
}

macro_rules! impl_hash {
    () => {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.inner.hash(state)
        }
    };
}

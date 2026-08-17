use crate::runtime::vm::VMGcRef;
use crate::{
    ArrayRef, AsContext, AsContextMut, EqRef, GcRefImpl, HeapType, I31, ManuallyRooted, Result,
    Rooted, StructRef,
    store::{AutoAssertNoGc, StoreOpaque},
};

/// Support for `anyref` disabled at compile time because the `gc` cargo feature
/// was not enabled.
pub enum AnyRef {}

impl From<Rooted<EqRef>> for Rooted<AnyRef> {
    #[inline]
    fn from(s: Rooted<EqRef>) -> Self {
        match s.inner {}
    }
}

impl From<ManuallyRooted<EqRef>> for ManuallyRooted<AnyRef> {
    #[inline]
    fn from(s: ManuallyRooted<EqRef>) -> Self {
        match s.inner {}
    }
}

impl From<Rooted<StructRef>> for Rooted<AnyRef> {
    #[inline]
    fn from(s: Rooted<StructRef>) -> Self {
        match s.inner {}
    }
}

impl From<ManuallyRooted<StructRef>> for ManuallyRooted<AnyRef> {
    #[inline]
    fn from(s: ManuallyRooted<StructRef>) -> Self {
        match s.inner {}
    }
}

impl From<Rooted<ArrayRef>> for Rooted<AnyRef> {
    #[inline]
    fn from(s: Rooted<ArrayRef>) -> Self {
        match s.inner {}
    }
}

impl From<ManuallyRooted<ArrayRef>> for ManuallyRooted<AnyRef> {
    #[inline]
    fn from(s: ManuallyRooted<ArrayRef>) -> Self {
        match s.inner {}
    }
}

impl GcRefImpl for AnyRef {}

impl AnyRef {
    pub(crate) fn from_cloned_gc_ref(
        _store: &mut AutoAssertNoGc<'_>,
        _gc_ref: VMGcRef,
    ) -> Rooted<Self> {
        unreachable!()
    }

    pub fn from_raw(_store: impl AsContextMut, raw: u32) -> Option<Rooted<Self>> {
        assert_eq!(raw, 0);
        None
    }

    pub fn _from_raw(_store: &mut AutoAssertNoGc<'_>, raw: u32) -> Option<Rooted<Self>> {
        assert_eq!(raw, 0);
        None
    }

    pub fn to_raw(&self, _store: impl AsContextMut) -> Result<u32> {
        match *self {}
    }

    pub fn ty(&self, _store: impl AsContext) -> Result<HeapType> {
        match *self {}
    }

    pub(crate) fn _ty(&self, _store: &StoreOpaque) -> Result<HeapType> {
        match *self {}
    }

    pub fn matches_ty(&self, _store: impl AsContext, _ty: &HeapType) -> Result<bool> {
        match *self {}
    }

    pub fn is_eqref(&self, _store: impl AsContext) -> Result<bool> {
        match *self {}
    }

    pub(crate) fn _is_eqref(&self, _store: &StoreOpaque) -> Result<bool> {
        match *self {}
    }

    pub fn as_eqref(&self, _store: impl AsContext) -> Result<Option<EqRef>> {
        match *self {}
    }

    pub fn unwrap_eqref(&self, _store: impl AsContext) -> Result<EqRef> {
        match *self {}
    }

    pub fn is_i31(&self, _store: impl AsContext) -> Result<bool> {
        match *self {}
    }

    pub(crate) fn _is_i31(&self, _store: &StoreOpaque) -> Result<bool> {
        match *self {}
    }

    pub fn as_i31(&self, _store: impl AsContext) -> Result<Option<I31>> {
        match *self {}
    }

    pub fn unwrap_i31(&self, _store: impl AsContext) -> Result<I31> {
        match *self {}
    }

    pub fn is_struct(&self, _store: impl AsContext) -> Result<bool> {
        match *self {}
    }

    pub(crate) fn _is_struct(&self, _store: &StoreOpaque) -> Result<bool> {
        match *self {}
    }

    pub fn as_struct(&self, _store: impl AsContext) -> Result<Option<StructRef>> {
        match *self {}
    }

    pub(crate) fn _as_struct(&self, _store: &StoreOpaque) -> Result<Option<StructRef>> {
        match *self {}
    }

    pub fn unwrap_struct(&self, _store: impl AsContext) -> Result<StructRef> {
        match *self {}
    }

    pub fn is_array(&self, _store: impl AsContext) -> Result<bool> {
        match *self {}
    }

    pub(crate) fn _is_array(&self, _store: &StoreOpaque) -> Result<bool> {
        match *self {}
    }

    pub fn as_array(&self, _store: impl AsContext) -> Result<Option<ArrayRef>> {
        match *self {}
    }

    pub(crate) fn _as_array(&self, _store: &StoreOpaque) -> Result<Option<ArrayRef>> {
        match *self {}
    }

    pub fn unwrap_array(&self, _store: impl AsContext) -> Result<ArrayRef> {
        match *self {}
    }
}

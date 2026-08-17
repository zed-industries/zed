use crate::{
    ArrayRef, AsContext, GcRefImpl, HeapType, I31, ManuallyRooted, Result, Rooted, StructRef,
    store::StoreOpaque,
};

/// Support for `eqref` disabled at compile time because the `gc` cargo feature
/// was not enabled.
pub enum EqRef {}

impl From<Rooted<StructRef>> for Rooted<EqRef> {
    #[inline]
    fn from(s: Rooted<StructRef>) -> Self {
        match s.inner {}
    }
}

impl From<ManuallyRooted<StructRef>> for ManuallyRooted<EqRef> {
    #[inline]
    fn from(s: ManuallyRooted<StructRef>) -> Self {
        match s.inner {}
    }
}

impl From<Rooted<ArrayRef>> for Rooted<EqRef> {
    #[inline]
    fn from(s: Rooted<ArrayRef>) -> Self {
        match s.inner {}
    }
}

impl From<ManuallyRooted<ArrayRef>> for ManuallyRooted<EqRef> {
    #[inline]
    fn from(s: ManuallyRooted<ArrayRef>) -> Self {
        match s.inner {}
    }
}

impl GcRefImpl for EqRef {}

impl EqRef {
    pub fn ty(&self, _store: impl AsContext) -> Result<HeapType> {
        match *self {}
    }

    pub(crate) fn _ty(&self, _store: &StoreOpaque) -> Result<HeapType> {
        match *self {}
    }

    pub fn matches_ty(&self, _store: impl AsContext, _ty: &HeapType) -> Result<bool> {
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

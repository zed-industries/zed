//! `exnref` implementation stubs when GC is disabled.

use crate::{
    AsContext, AsContextMut, ExnType, GcRefImpl, HeapType, Result, Rooted, Tag, Val,
    store::{AutoAssertNoGc, StoreContextMut, StoreOpaque},
};

/// Support for `ExnRefPre` disabled at compile time because the `gc`
/// cargo feature was not enabled.
pub enum ExnRefPre {}

/// Support for `exnref` disabled at compile time because the `gc`
/// cargo feature was not enabled.
pub enum ExnRef {}

impl GcRefImpl for ExnRef {}

impl ExnRef {
    pub fn from_raw(_store: impl AsContextMut, _raw: u32) -> Option<Rooted<Self>> {
        None
    }

    pub(crate) fn _from_raw(_store: &mut AutoAssertNoGc, _raw: u32) -> Option<Rooted<Self>> {
        None
    }

    pub fn to_raw(&self, _store: impl AsContextMut) -> Result<u32> {
        Ok(0)
    }

    pub(crate) fn _to_raw(&self, _store: &mut AutoAssertNoGc<'_>) -> Result<u32> {
        Ok(0)
    }

    pub fn ty(&self, _store: impl AsContext) -> Result<ExnType> {
        match *self {}
    }

    pub(crate) fn _ty(&self, _store: &StoreOpaque) -> Result<ExnType> {
        match *self {}
    }

    pub fn matches_ty(&self, _store: impl AsContext, _ty: &HeapType) -> Result<bool> {
        match *self {}
    }

    pub(crate) fn _matches_ty(&self, _store: &StoreOpaque, _ty: &HeapType) -> Result<bool> {
        match *self {}
    }

    pub fn tag(&self, _store: impl AsContextMut) -> Result<Tag> {
        match *self {}
    }

    pub fn fields<'a, T: 'static>(
        &self,
        _store: impl Into<StoreContextMut<'a, T>>,
    ) -> Result<impl ExactSizeIterator<Item = Val> + 'a + '_> {
        match *self {}
        Ok([].into_iter())
    }

    pub fn field(&self, _store: impl AsContextMut, _index: usize) -> Result<Val> {
        match *self {}
    }

    pub fn set_field(&self, _store: impl AsContextMut, _index: usize, _value: Val) -> Result<()> {
        match *self {}
    }
}

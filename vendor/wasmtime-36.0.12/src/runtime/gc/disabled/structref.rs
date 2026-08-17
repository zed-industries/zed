use crate::{
    AsContext, AsContextMut, GcRefImpl, Result, StructType, Val,
    store::{StoreContextMut, StoreOpaque},
};

/// Support for `StructRefPre` disabled at compile time because the `gc` cargo
/// feature was not enabled.
pub enum StructRefPre {}

/// Support for `structref` disabled at compile time because the `gc` cargo feature
/// was not enabled.
pub enum StructRef {}

impl GcRefImpl for StructRef {}

impl StructRef {
    pub fn ty(&self, _store: impl AsContext) -> Result<StructType> {
        match *self {}
    }

    pub fn matches_ty(&self, _store: impl AsContext, _ty: &StructType) -> Result<bool> {
        match *self {}
    }

    pub(crate) fn _matches_ty(&self, _store: &StoreOpaque, _ty: &StructType) -> Result<bool> {
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

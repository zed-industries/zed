use crate::runtime::vm::VMGcRef;
use crate::{
    AsContextMut, GcRefImpl, Result, Rooted, StoreContext, StoreContextMut, store::AutoAssertNoGc,
};
use core::any::Any;

/// Support for `externref` disabled at compile time because the `gc` cargo
/// feature was not enabled.
pub enum ExternRef {}

impl GcRefImpl for ExternRef {}

impl ExternRef {
    pub(crate) fn from_cloned_gc_ref(
        _store: &mut AutoAssertNoGc<'_>,
        _gc_ref: VMGcRef,
    ) -> Rooted<Self> {
        unreachable!()
    }

    pub fn data<'a, T: 'static>(
        &self,
        _store: impl Into<StoreContext<'a, T>>,
    ) -> Result<&'a (dyn Any + Send + Sync)>
    where
        T: 'a,
    {
        match *self {}
    }

    pub fn data_mut<'a, T: 'static>(
        &self,
        _store: impl Into<StoreContextMut<'a, T>>,
    ) -> Result<&'a mut (dyn Any + Send + Sync)>
    where
        T: 'a,
    {
        match *self {}
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
}

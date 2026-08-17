use crate::Result;
use crate::runtime::types::TagType;
use crate::trampoline::generate_tag_export;
use crate::{
    AsContext, AsContextMut,
    store::{StoreInstanceId, StoreOpaque},
};
use wasmtime_environ::DefinedTagIndex;

#[cfg(feature = "gc")]
use crate::store::InstanceId;

/// A WebAssembly `tag`.
#[derive(Copy, Clone, Debug)]
#[repr(C)] // here for the C API in the future
pub struct Tag {
    instance: StoreInstanceId,
    index: DefinedTagIndex,
}

impl Tag {
    pub(crate) fn from_raw(instance: StoreInstanceId, index: DefinedTagIndex) -> Tag {
        Tag { instance, index }
    }

    /// Create a new tag instance from a given TagType.
    ///
    /// # Panics
    ///
    /// This function will panic when used with a [`Store`](`crate::Store`)
    /// which has a [`ResourceLimiterAsync`](`crate::ResourceLimiterAsync`)
    /// (see also: [`Store::limiter_async`](`crate::Store::limiter_async`).
    /// When using an async resource limiter, use [`Tag::new_async`]
    /// instead.
    pub fn new(mut store: impl AsContextMut, ty: &TagType) -> Result<Tag> {
        generate_tag_export(store.as_context_mut().0, ty)
    }

    /// Returns the underlying type of this `tag`.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this tag.
    pub fn ty(&self, store: impl AsContext) -> TagType {
        self._ty(store.as_context().0)
    }

    pub(crate) fn _ty(&self, store: &StoreOpaque) -> TagType {
        TagType::from_wasmtime_tag(store.engine(), self.wasmtime_ty(store))
    }

    pub(crate) fn wasmtime_ty<'a>(&self, store: &'a StoreOpaque) -> &'a wasmtime_environ::Tag {
        let module = store[self.instance].env_module();
        let index = module.tag_index(self.index);
        &module.tags[index]
    }

    pub(crate) fn vmimport(&self, store: &StoreOpaque) -> crate::runtime::vm::VMTagImport {
        let instance = &store[self.instance];
        crate::runtime::vm::VMTagImport {
            from: instance.tag_ptr(self.index).into(),
            vmctx: instance.vmctx().into(),
            index: self.index,
        }
    }

    pub(crate) fn comes_from_same_store(&self, store: &StoreOpaque) -> bool {
        store.id() == self.instance.store_id()
    }

    /// Determines whether this tag is reference equal to the other
    /// given tag in the given store.
    ///
    /// # Panics
    ///
    /// Panics if either tag do not belong to the given `store`.
    pub fn eq(a: &Tag, b: &Tag, store: impl AsContext) -> bool {
        // make sure both tags belong to the store
        let store = store.as_context();
        let _ = &store[a.instance];
        let _ = &store[b.instance];

        // then compare to see if they have the same definition
        a.instance == b.instance && a.index == b.index
    }

    /// Get the "index coordinates" for this `Tag`: the raw instance
    /// ID and defined-tag index within that instance. This can be
    /// used to "serialize" the tag as safe (tamper-proof,
    /// bounds-checked) values, e.g. within the GC store for an
    /// exception object.
    #[cfg(feature = "gc")]
    pub(crate) fn to_raw_indices(&self) -> (InstanceId, DefinedTagIndex) {
        (self.instance.instance(), self.index)
    }

    /// Create a new `Tag` from known raw indices as produced by
    /// `to_raw_indices()`.
    ///
    /// # Panics
    ///
    /// Panics if the indices are out-of-bounds in the given store.
    #[cfg(feature = "gc")]
    pub(crate) fn from_raw_indices(
        store: &StoreOpaque,
        instance: InstanceId,
        index: DefinedTagIndex,
    ) -> Tag {
        let instance = StoreInstanceId::new(store.id(), instance);
        Tag { instance, index }
    }
}

use crate::*;

/// Handle to a query set.
///
/// A `QuerySet` is an opaque, mutable storage location for the results of queries:
/// which are small pieces of information extracted from other operations such as render passes.
/// See [`QueryType`] for what types of information can be collected.
///
/// Each query writes data into one or more result slots in the `QuerySet`, which must be created
/// with a sufficient number of slots for that usage. Each result slot is a an unsigned 64-bit
/// number.
///
/// Using queries consists of the following steps:
///
/// 1. Create a `QuerySet` of the appropriate type and number of query result slots
///    using [`Device::create_query_set()`].
/// 2. Pass the `QuerySet` to the commands which will write to it.
///    See [`QueryType`] for the possible commands.
/// 3. Execute the command [`CommandEncoder::resolve_query_set()`].
///    This converts the opaque data stored in a `QuerySet` into [`u64`]s stored in a [`Buffer`].
/// 4. Make use of that buffer, such as by copying its contents to the CPU
///    or reading it from a compute shader.
///
/// Corresponds to [WebGPU `GPUQuerySet`](https://gpuweb.github.io/gpuweb/#queryset).
#[derive(Debug, Clone)]
pub struct QuerySet {
    pub(crate) inner: dispatch::DispatchQuerySet,
}
#[cfg(send_sync)]
#[cfg(send_sync)]
static_assertions::assert_impl_all!(QuerySet: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(QuerySet => .inner);

impl QuerySet {
    #[cfg(custom)]
    /// Returns custom implementation of QuerySet (if custom backend and is internally T)
    pub fn as_custom<T: custom::QuerySetInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// Describes a [`QuerySet`].
///
/// For use with [`Device::create_query_set`].
///
/// Corresponds to [WebGPU `GPUQuerySetDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuquerysetdescriptor).
pub type QuerySetDescriptor<'a> = wgt::QuerySetDescriptor<Label<'a>>;
static_assertions::assert_impl_all!(QuerySetDescriptor<'_>: Send, Sync);

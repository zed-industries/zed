//! GC-related methods for stores.

use super::*;
use crate::GcHeapOutOfMemory;

impl StoreOpaque {
    /// Collect garbage, potentially growing the GC heap.
    pub(crate) fn gc(&mut self, why: Option<&GcHeapOutOfMemory<()>>) {
        assert!(!self.async_support());
        unsafe {
            self.maybe_async_gc(None, why.map(|oom| oom.bytes_needed()))
                .expect("infallible when not async");
        }
    }

    /// Attempt to grow the GC heap by `bytes_needed` or, if that fails, perform
    /// a garbage collection.
    ///
    /// Cooperative, async-yielding (if configured) is completely transparent.
    ///
    /// Note that even when this function returns `Ok(())`, it is not guaranteed
    /// that a GC allocation of size `bytes_needed` will succeed. Growing the GC
    /// heap could fail, and then performing a collection could succeed but
    /// might not free up enough space. Therefore, callers should not assume
    /// that a retried allocation will always succeed.
    ///
    /// # Safety
    ///
    /// When async is enabled, it is the caller's responsibility to ensure that
    /// this is called on a fiber stack.
    pub(crate) unsafe fn maybe_async_gc(
        &mut self,
        root: Option<VMGcRef>,
        bytes_needed: Option<u64>,
    ) -> Result<Option<VMGcRef>> {
        let mut scope = crate::OpaqueRootScope::new(self);
        let store_id = scope.id();
        let root = root.map(|r| scope.gc_roots_mut().push_lifo_root(store_id, r));

        if scope.async_support() {
            #[cfg(feature = "async")]
            scope.block_on(|scope| Box::pin(scope.grow_or_collect_gc_heap_async(bytes_needed)))?;
        } else {
            scope.grow_or_collect_gc_heap(bytes_needed);
        }

        let root = match root {
            None => None,
            Some(r) => {
                let r = r
                    .get_gc_ref(&scope)
                    .expect("still in scope")
                    .unchecked_copy();
                Some(scope.gc_store_mut()?.clone_gc_ref(&r))
            }
        };

        Ok(root)
    }

    fn grow_or_collect_gc_heap(&mut self, bytes_needed: Option<u64>) {
        assert!(!self.async_support());
        if let Some(n) = bytes_needed {
            if unsafe { self.maybe_async_grow_gc_heap(n).is_ok() } {
                return;
            }
        }
        self.do_gc();
    }

    /// Attempt to grow the GC heap by `bytes_needed` bytes.
    ///
    /// Returns an error if growing the GC heap fails.
    ///
    /// # Safety
    ///
    /// When async is enabled, it is the caller's responsibility to ensure that
    /// this is called on a fiber stack.
    unsafe fn maybe_async_grow_gc_heap(&mut self, bytes_needed: u64) -> Result<()> {
        log::trace!("Attempting to grow the GC heap by {bytes_needed} bytes");
        assert!(bytes_needed > 0);

        // Take the GC heap's underlying memory out of the GC heap, attempt to
        // grow it, then replace it.
        let mut memory = unsafe { self.unwrap_gc_store_mut().gc_heap.take_memory() };
        let mut delta_bytes_grown = 0;
        let grow_result: Result<()> = (|| {
            let page_size = self.engine().tunables().gc_heap_memory_type().page_size();

            let current_size_in_bytes = u64::try_from(memory.byte_size()).unwrap();
            let current_size_in_pages = current_size_in_bytes / page_size;

            // Aim to double the heap size, amortizing the cost of growth.
            let doubled_size_in_pages = current_size_in_pages.saturating_mul(2);
            assert!(doubled_size_in_pages >= current_size_in_pages);
            let delta_pages_for_doubling = doubled_size_in_pages - current_size_in_pages;

            // When doubling our size, saturate at the maximum memory size in pages.
            //
            // TODO: we should consult the instance allocator for its configured
            // maximum memory size, if any, rather than assuming the index
            // type's maximum size.
            let max_size_in_bytes = 1 << 32;
            let max_size_in_pages = max_size_in_bytes / page_size;
            let delta_to_max_size_in_pages = max_size_in_pages - current_size_in_pages;
            let delta_pages_for_alloc = delta_pages_for_doubling.min(delta_to_max_size_in_pages);

            // But always make sure we are attempting to grow at least as many pages
            // as needed by the requested allocation. This must happen *after* the
            // max-size saturation, so that if we are at the max already, we do not
            // succeed in growing by zero delta pages, and then return successfully
            // to our caller, who would be assuming that there is now capacity for
            // their allocation.
            let pages_needed = bytes_needed.div_ceil(page_size);
            assert!(pages_needed > 0);
            let delta_pages_for_alloc = delta_pages_for_alloc.max(pages_needed);
            assert!(delta_pages_for_alloc > 0);

            // Safety: we pair growing the GC heap with updating its associated
            // `VMMemoryDefinition` in the `VMStoreContext` immediately
            // afterwards.
            unsafe {
                memory
                    .grow(delta_pages_for_alloc, Some(self.traitobj().as_mut()))?
                    .ok_or_else(|| anyhow!("failed to grow GC heap"))?;
            }
            self.vm_store_context.gc_heap = memory.vmmemory();

            let new_size_in_bytes = u64::try_from(memory.byte_size()).unwrap();
            assert!(new_size_in_bytes > current_size_in_bytes);
            delta_bytes_grown = new_size_in_bytes - current_size_in_bytes;
            let delta_bytes_for_alloc = delta_pages_for_alloc.checked_mul(page_size).unwrap();
            assert!(
                delta_bytes_grown >= delta_bytes_for_alloc,
                "{delta_bytes_grown} should be greater than or equal to {delta_bytes_for_alloc}"
            );
            Ok(())
        })();

        // Regardless whether growing succeeded or failed, place the memory back
        // inside the GC heap.
        unsafe {
            self.unwrap_gc_store_mut()
                .gc_heap
                .replace_memory(memory, delta_bytes_grown);
        }

        grow_result
    }

    /// Attempt an allocation, if it fails due to GC OOM, then do a GC and
    /// retry.
    pub(crate) fn retry_after_gc<T, U>(
        &mut self,
        value: T,
        alloc_func: impl Fn(&mut Self, T) -> Result<U>,
    ) -> Result<U>
    where
        T: Send + Sync + 'static,
    {
        assert!(
            !self.async_support(),
            "use the `*_async` versions of methods when async is configured"
        );
        match alloc_func(self, value) {
            Ok(x) => Ok(x),
            Err(e) => match e.downcast::<crate::GcHeapOutOfMemory<T>>() {
                Ok(oom) => {
                    let (value, oom) = oom.take_inner();
                    self.gc(Some(&oom));
                    alloc_func(self, value)
                }
                Err(e) => Err(e),
            },
        }
    }

    /// Like `retry_after_gc` but async yielding (if necessary) is transparent.
    ///
    /// # Safety
    ///
    /// When async is enabled, it is the caller's responsibility to ensure that
    /// this is called on a fiber stack.
    pub(crate) unsafe fn retry_after_gc_maybe_async<T, U>(
        &mut self,
        value: T,
        alloc_func: impl Fn(&mut Self, T) -> Result<U>,
    ) -> Result<U>
    where
        T: Send + Sync + 'static,
    {
        match alloc_func(self, value) {
            Ok(x) => Ok(x),
            Err(e) => match e.downcast::<crate::GcHeapOutOfMemory<T>>() {
                Ok(oom) => {
                    let (value, oom) = oom.take_inner();
                    // SAFETY: it's the caller's responsibility to ensure that
                    // this is on a fiber stack if necessary.
                    unsafe {
                        self.maybe_async_gc(None, Some(oom.bytes_needed()))?;
                    }
                    alloc_func(self, value)
                }
                Err(e) => Err(e),
            },
        }
    }
}

#[cfg(feature = "async")]
impl StoreOpaque {
    /// Asynchronously collect garbage, potentially growing the GC heap.
    pub(crate) async fn gc_async(&mut self, why: Option<&GcHeapOutOfMemory<()>>) -> Result<()> {
        assert!(self.async_support());
        self.on_fiber(|store| unsafe {
            store.maybe_async_gc(None, why.map(|oom| oom.bytes_needed()))
        })
        .await??;
        Ok(())
    }

    async fn grow_or_collect_gc_heap_async(&mut self, bytes_needed: Option<u64>) {
        assert!(self.async_support());
        if let Some(bytes_needed) = bytes_needed {
            if unsafe { self.maybe_async_grow_gc_heap(bytes_needed).is_ok() } {
                return;
            }
        }

        self.do_gc_async().await;
    }

    /// Attempt an allocation, if it fails due to GC OOM, then do a GC and
    /// retry.
    pub(crate) async fn retry_after_gc_async<T, U>(
        &mut self,
        value: T,
        alloc_func: impl Fn(&mut Self, T) -> Result<U>,
    ) -> Result<U>
    where
        T: Send + Sync + 'static,
    {
        assert!(
            self.async_support(),
            "you must configure async to use the `*_async` versions of methods"
        );
        match alloc_func(self, value) {
            Ok(x) => Ok(x),
            Err(e) => match e.downcast::<crate::GcHeapOutOfMemory<T>>() {
                Ok(oom) => {
                    let (value, oom) = oom.take_inner();
                    self.gc_async(Some(&oom)).await?;
                    alloc_func(self, value)
                }
                Err(e) => Err(e),
            },
        }
    }
}

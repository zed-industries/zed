use super::{
    TableAllocationIndex,
    index_allocator::{SimpleIndexAllocator, SlotId},
};
use crate::runtime::vm::sys::vm::commit_pages;
use crate::runtime::vm::{
    InstanceAllocationRequest, Mmap, PoolingInstanceAllocatorConfig, SendSyncPtr, Table,
    mmap::AlignedLength,
};
use crate::{prelude::*, vm::HostAlignedByteCount};
use std::ptr::NonNull;
use wasmtime_environ::{Module, Tunables};

/// Represents a pool of WebAssembly tables.
///
/// Each instance index into the pool returns an iterator over the base addresses
/// of the instance's tables.
#[derive(Debug)]
pub struct TablePool {
    index_allocator: SimpleIndexAllocator,
    mapping: Mmap<AlignedLength>,
    table_size: HostAlignedByteCount,
    max_total_tables: usize,
    tables_per_instance: usize,
    keep_resident: HostAlignedByteCount,
    nominal_table_elements: usize,
}

impl TablePool {
    /// Create a new `TablePool`.
    pub fn new(config: &PoolingInstanceAllocatorConfig) -> Result<Self> {
        let table_size = HostAlignedByteCount::new_rounded_up(
            crate::runtime::vm::table::NOMINAL_MAX_TABLE_ELEM_SIZE
                .checked_mul(config.limits.table_elements)
                .ok_or_else(|| anyhow!("table size exceeds addressable memory"))?,
        )?;

        let max_total_tables = usize::try_from(config.limits.total_tables).unwrap();
        let tables_per_instance = usize::try_from(config.limits.max_tables_per_module).unwrap();

        let allocation_size = table_size
            .checked_mul(max_total_tables)
            .context("total size of tables exceeds addressable memory")?;

        let mapping = Mmap::accessible_reserved(allocation_size, allocation_size)
            .context("failed to create table pool mapping")?;

        let keep_resident = HostAlignedByteCount::new_rounded_up(config.table_keep_resident)?;

        Ok(Self {
            index_allocator: SimpleIndexAllocator::new(config.limits.total_tables),
            mapping,
            table_size,
            max_total_tables,
            tables_per_instance,
            keep_resident,
            nominal_table_elements: config.limits.table_elements,
        })
    }

    /// Validate whether this module's tables are allocatable by this pool.
    pub fn validate(&self, module: &Module) -> Result<()> {
        let tables = module.num_defined_tables();

        if tables > self.tables_per_instance {
            bail!(
                "defined tables count of {} exceeds the per-instance limit of {}",
                tables,
                self.tables_per_instance,
            );
        }

        if tables > self.max_total_tables {
            bail!(
                "defined tables count of {} exceeds the total tables limit of {}",
                tables,
                self.max_total_tables,
            );
        }

        for (i, table) in module.tables.iter().skip(module.num_imported_tables) {
            if table.limits.min > u64::try_from(self.nominal_table_elements)? {
                bail!(
                    "table index {} has a minimum element size of {} which exceeds the limit of {}",
                    i.as_u32(),
                    table.limits.min,
                    self.nominal_table_elements,
                );
            }
        }
        Ok(())
    }

    /// Are there zero slots in use right now?
    pub fn is_empty(&self) -> bool {
        self.index_allocator.is_empty()
    }

    /// Get the base pointer of the given table allocation.
    fn get(&self, table_index: TableAllocationIndex) -> *mut u8 {
        assert!(table_index.index() < self.max_total_tables);

        unsafe {
            self.mapping
                .as_ptr()
                .add(
                    self.table_size
                        .checked_mul(table_index.index())
                        .expect(
                            "checked in constructor that table_size * table_index doesn't overflow",
                        )
                        .byte_count(),
                )
                .cast_mut()
        }
    }

    /// Returns the number of bytes occupied by table entry data
    ///
    /// This is typically just the `nominal_table_elements` multiplied by
    /// the size of the table's element type, but may be less in the case
    /// of types such as VMContRef for which less capacity will be available
    /// (maintaining a consistent table size in the pool).
    fn data_size(&self, table_type: crate::vm::table::TableElementType) -> usize {
        let element_size = table_type.element_size();
        let elements = self
            .nominal_table_elements
            .min(self.table_size.byte_count() / element_size);
        elements * element_size
    }

    /// Allocate a single table for the given instance allocation request.
    pub fn allocate(
        &self,
        request: &mut InstanceAllocationRequest,
        ty: &wasmtime_environ::Table,
        tunables: &Tunables,
    ) -> Result<(TableAllocationIndex, Table)> {
        let allocation_index = self
            .index_allocator
            .alloc()
            .map(|slot| TableAllocationIndex(slot.0))
            .ok_or_else(|| {
                super::PoolConcurrencyLimitError::new(self.max_total_tables, "tables")
            })?;

        match (|| {
            let base = self.get(allocation_index);
            let data_size = self.data_size(crate::vm::table::wasm_to_table_type(ty.ref_type));
            unsafe {
                commit_pages(base, data_size)?;
            }

            let ptr =
                NonNull::new(std::ptr::slice_from_raw_parts_mut(base.cast(), data_size)).unwrap();
            unsafe {
                Table::new_static(
                    ty,
                    tunables,
                    SendSyncPtr::new(ptr),
                    &mut *request.store.get().unwrap(),
                )
            }
        })() {
            Ok(table) => Ok((allocation_index, table)),
            Err(e) => {
                self.index_allocator.free(SlotId(allocation_index.0));
                Err(e)
            }
        }
    }

    /// Deallocate a previously-allocated table.
    ///
    /// # Safety
    ///
    /// The table must have been previously-allocated by this pool and assigned
    /// the given allocation index, it must currently be allocated, and it must
    /// never be used again.
    ///
    /// The caller must have already called `reset_table_pages_to_zero` on the
    /// memory and flushed any enqueued decommits for this table's memory.
    pub unsafe fn deallocate(&self, allocation_index: TableAllocationIndex, table: Table) {
        assert!(table.is_static());
        drop(table);
        self.index_allocator.free(SlotId(allocation_index.0));
    }

    /// Reset the given table's memory to zero.
    ///
    /// Invokes the given `decommit` function for each region of memory that
    /// needs to be decommitted. It is the caller's responsibility to actually
    /// perform that decommit before this table is reused.
    ///
    /// # Safety
    ///
    /// This table must not be in active use, and ready for returning to the
    /// table pool once it is zeroed and decommitted.
    pub unsafe fn reset_table_pages_to_zero(
        &self,
        allocation_index: TableAllocationIndex,
        table: &mut Table,
        mut decommit: impl FnMut(*mut u8, usize),
    ) {
        assert!(table.is_static());
        let base = self.get(allocation_index);
        let table_byte_size = table.size() * table.element_type().element_size();
        let table_byte_size_page_aligned = HostAlignedByteCount::new_rounded_up(table_byte_size)
            .expect("table entry size doesn't overflow");

        // `memset` the first `keep_resident` bytes.
        let size_to_memset = table_byte_size_page_aligned.min(self.keep_resident);

        // SAFETY: the contract of this function requires that the table is not
        // actively in use so it's safe to pave over its allocation with zero
        // bytes.
        unsafe {
            std::ptr::write_bytes(base, 0, size_to_memset.byte_count());
        }

        // And decommit the rest of it.
        decommit(
            // SAFETY: `size_to_memset` is less than the size of the allocation,
            // so it's safe to use the `add` intrinsic.
            unsafe { base.add(size_to_memset.byte_count()) },
            table_byte_size_page_aligned
                .checked_sub(size_to_memset)
                .expect("size_to_memset <= size")
                .byte_count(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::vm::InstanceLimits;

    #[test]
    fn test_table_pool() -> Result<()> {
        let pool = TablePool::new(&PoolingInstanceAllocatorConfig {
            limits: InstanceLimits {
                total_tables: 7,
                table_elements: 100,
                max_memory_size: 0,
                max_memories_per_module: 0,
                ..Default::default()
            },
            ..Default::default()
        })?;

        let host_page_size = HostAlignedByteCount::host_page_size();

        assert_eq!(pool.table_size, host_page_size);
        assert_eq!(pool.max_total_tables, 7);
        assert_eq!(pool.nominal_table_elements, 100);

        let base = pool.mapping.as_ptr() as usize;

        for i in 0..7 {
            let index = TableAllocationIndex(i);
            let ptr = pool.get(index);
            assert_eq!(
                ptr as usize - base,
                pool.table_size.checked_mul(i as usize).unwrap()
            );
        }

        Ok(())
    }

    #[test]
    fn test_table_pool_continuations_capacity() -> Result<()> {
        let mkpool = |table_elements: usize| -> Result<TablePool> {
            TablePool::new(&PoolingInstanceAllocatorConfig {
                limits: InstanceLimits {
                    table_elements,
                    total_tables: 7,
                    max_memory_size: 0,
                    max_memories_per_module: 0,
                    ..Default::default()
                },
                ..Default::default()
            })
        };

        let host_page_size = HostAlignedByteCount::host_page_size();
        let words_per_page = host_page_size.byte_count() / size_of::<*const u8>();
        let pool_big = mkpool(words_per_page - 1)?;
        let pool_small = mkpool(5)?;

        assert_eq!(pool_small.table_size, host_page_size);
        assert_eq!(pool_big.table_size, host_page_size);

        // table should store nominal_table_elements of data for func in both cases
        let func_table_type = crate::vm::table::TableElementType::Func;
        assert_eq!(
            pool_small.data_size(func_table_type),
            pool_small.nominal_table_elements * func_table_type.element_size()
        );
        assert_eq!(
            pool_big.data_size(func_table_type),
            pool_big.nominal_table_elements * func_table_type.element_size()
        );

        // In the "big" case, continuations should fill page size (capacity limited).
        // In the "small" case, continuations should fill only part of the page, capping
        // at the requested table size for nominal elements.
        let cont_table_type = crate::vm::table::TableElementType::Cont;
        assert_eq!(
            pool_small.data_size(cont_table_type),
            pool_small.nominal_table_elements * cont_table_type.element_size()
        );
        assert_eq!(pool_big.data_size(cont_table_type), host_page_size);

        Ok(())
    }
}

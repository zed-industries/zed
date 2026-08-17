//! Support for implementing the [`RuntimeLinearMemory`] trait in terms of a
//! platform memory allocation primitive (e.g. `malloc`)
//!
//! Note that memory is allocated here using `Vec::try_reserve` to explicitly
//! handle memory allocation failures.

use crate::prelude::*;
use crate::runtime::vm::SendSyncPtr;
use crate::runtime::vm::memory::{MemoryBase, RuntimeLinearMemory};
use core::mem;
use core::ptr::NonNull;
use wasmtime_environ::Tunables;

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct Align16(u128);

/// An instance of linear memory backed by the default system allocator.
pub struct MallocMemory {
    storage: Vec<Align16>,
    base_ptr: SendSyncPtr<u8>,
    byte_len: usize,
}

impl MallocMemory {
    pub fn new(
        _ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        minimum: usize,
    ) -> Result<Self> {
        if tunables.memory_guard_size > 0 {
            bail!("malloc memory is only compatible if guard pages aren't used");
        }
        if tunables.memory_reservation > 0 {
            bail!("malloc memory is only compatible with no ahead-of-time memory reservation");
        }
        if tunables.memory_init_cow {
            bail!("malloc memory cannot be used with CoW images");
        }

        let initial_allocation_byte_size = minimum
            .checked_add(tunables.memory_reservation_for_growth.try_into()?)
            .context("memory allocation size too large")?;

        let initial_allocation_len = byte_size_to_element_len(initial_allocation_byte_size);
        let mut storage = Vec::new();
        storage.try_reserve(initial_allocation_len)?;

        let initial_len = byte_size_to_element_len(minimum);
        if initial_len > 0 {
            grow_storage_to(&mut storage, initial_len);
        }
        Ok(MallocMemory {
            base_ptr: SendSyncPtr::new(NonNull::new(storage.as_mut_ptr()).unwrap()).cast(),
            storage,
            byte_len: minimum,
        })
    }
}

impl RuntimeLinearMemory for MallocMemory {
    fn byte_size(&self) -> usize {
        self.byte_len
    }

    fn byte_capacity(&self) -> usize {
        self.storage.capacity() * mem::size_of::<Align16>()
    }

    fn grow_to(&mut self, new_size: usize) -> Result<()> {
        let new_element_len = byte_size_to_element_len(new_size);
        if new_element_len > self.storage.len() {
            self.storage
                .try_reserve(new_element_len - self.storage.len())?;
            grow_storage_to(&mut self.storage, new_element_len);
            self.base_ptr =
                SendSyncPtr::new(NonNull::new(self.storage.as_mut_ptr()).unwrap()).cast();
        }
        self.byte_len = new_size;
        Ok(())
    }

    fn base(&self) -> MemoryBase {
        MemoryBase::Raw(self.base_ptr)
    }

    fn vmmemory(&self) -> crate::vm::VMMemoryDefinition {
        let base = self.base_ptr.as_non_null();
        crate::vm::VMMemoryDefinition {
            base: base.into(),
            current_length: self.byte_len.into(),
        }
    }
}

fn byte_size_to_element_len(byte_size: usize) -> usize {
    let align = mem::align_of::<Align16>();

    // Round up the requested byte size to the size of each vector element.
    let byte_size_rounded_up =
        byte_size.checked_add(align - 1).unwrap_or(usize::MAX) & !(align - 1);

    // Next divide this aligned size by the size of each element to get the
    // element length of our vector.
    byte_size_rounded_up / align
}

/// Helper that is the equivalent of `storage.resize(new_len, Align16(0))`
/// except it's also optimized to perform well in debug mode. Just using
/// `resize` leads to a per-element iteration which can be quite slow in debug
/// mode as it's not optimized to a memcpy, so it's manually optimized here
/// instead.
fn grow_storage_to(storage: &mut Vec<Align16>, new_len: usize) {
    debug_assert!(new_len > storage.len());
    assert!(new_len <= storage.capacity());
    let capacity_to_set = new_len - storage.len();
    let slice_to_initialize = &mut storage.spare_capacity_mut()[..capacity_to_set];
    let byte_size = mem::size_of_val(slice_to_initialize);

    // SAFETY: The `slice_to_initialize` is guaranteed to be in the capacity of
    // the vector via the slicing above, so it's all owned memory by the
    // vector. Additionally the `byte_size` is the exact size of the
    // `slice_to_initialize` itself, so this `memset` should be in-bounds.
    // Finally the `Align16` is a simple wrapper around `u128` for which 0
    // is a valid byte pattern. This should make the initial `write_bytes` safe.
    //
    // Afterwards the `set_len` call should also be safe because we've
    // initialized the tail end of the vector with zeros so it's safe to
    // consider it having a new length now.
    unsafe {
        core::ptr::write_bytes(slice_to_initialize.as_mut_ptr().cast::<u8>(), 0, byte_size);
        storage.set_len(new_len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is currently required by the constructor but otherwise ignored in
    // the creation of a `MallocMemory`, so just have a single one used in
    // tests below.
    const TY: wasmtime_environ::Memory = wasmtime_environ::Memory {
        idx_type: wasmtime_environ::IndexType::I32,
        limits: wasmtime_environ::Limits { min: 0, max: None },
        shared: false,
        page_size_log2: 16,
    };

    // Valid tunables that can be used to create a `MallocMemory`.
    const TUNABLES: Tunables = Tunables {
        memory_reservation: 0,
        memory_guard_size: 0,
        memory_init_cow: false,
        ..Tunables::default_miri()
    };

    #[test]
    fn simple() {
        let mut memory = MallocMemory::new(&TY, &TUNABLES, 10).unwrap();
        assert_eq!(memory.storage.len(), 1);
        assert_valid(&memory);

        memory.grow_to(11).unwrap();
        assert_eq!(memory.storage.len(), 1);
        assert_valid(&memory);

        memory.grow_to(16).unwrap();
        assert_eq!(memory.storage.len(), 1);
        assert_valid(&memory);

        memory.grow_to(17).unwrap();
        assert_eq!(memory.storage.len(), 2);
        assert_valid(&memory);

        memory.grow_to(65).unwrap();
        assert_eq!(memory.storage.len(), 5);
        assert_valid(&memory);
    }

    #[test]
    fn reservation_not_initialized() {
        let tunables = Tunables {
            memory_reservation_for_growth: 1 << 20,
            ..TUNABLES
        };
        let mut memory = MallocMemory::new(&TY, &tunables, 10).unwrap();
        assert_eq!(memory.storage.len(), 1);
        assert_eq!(
            memory.storage.capacity(),
            (tunables.memory_reservation_for_growth / 16) as usize + 1,
        );
        assert_valid(&memory);

        memory.grow_to(100).unwrap();
        assert_eq!(memory.storage.len(), 7);
        assert_eq!(
            memory.storage.capacity(),
            (tunables.memory_reservation_for_growth / 16) as usize + 1,
        );
        assert_valid(&memory);
    }

    fn assert_valid(mem: &MallocMemory) {
        assert_eq!(mem.storage.as_ptr().cast::<u8>(), mem.base_ptr.as_ptr());
        assert!(mem.byte_len <= mem.storage.len() * 16);
        for slot in mem.storage.iter() {
            assert_eq!(slot.0, 0);
        }
    }
}

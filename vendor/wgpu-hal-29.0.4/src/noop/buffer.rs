use alloc::vec::Vec;
use core::{cell::UnsafeCell, ops::Range, ptr};

cfg_if::cfg_if! {
    if #[cfg(supports_ptr_atomics)] {
        use alloc::sync::Arc;
    } else if #[cfg(feature = "portable-atomic")] {
        use portable_atomic_util::Arc;
    }
}

#[derive(Clone, Debug)]
pub struct Buffer {
    /// This data is potentially accessed mutably in arbitrary non-overlapping slices,
    /// so we must store it in `UnsafeCell` to avoid making any too-strong no-aliasing claims.
    storage: Arc<UnsafeCell<[u8]>>,

    /// Size of the allocation.
    ///
    /// This is redundant with `storage.get().len()`, but that method is not
    /// available until our MSRV is 1.79 or greater.
    size: usize,
}

/// SAFETY:
/// This shared mutable data will not be accessed in a way which causes data races;
/// the obligation to do so is on the caller of the HAL API.
/// For safe code, `wgpu-core` validation manages appropriate access.
unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl Buffer {
    pub(super) fn new(desc: &crate::BufferDescriptor) -> Result<Self, crate::DeviceError> {
        let &crate::BufferDescriptor {
            label: _,
            size,
            usage: _,
            memory_flags: _,
        } = desc;

        let size = usize::try_from(size).map_err(|_| crate::DeviceError::OutOfMemory)?;

        let mut vector: Vec<u8> = Vec::new();
        vector
            .try_reserve_exact(size)
            .map_err(|_| crate::DeviceError::OutOfMemory)?;
        vector.resize(size, 0);
        let storage: Arc<[u8]> = Arc::from(vector);
        debug_assert_eq!(storage.len(), size);

        // SAFETY: `UnsafeCell<[u8]>` and `[u8]` have the same layout.
        // This is just adding a wrapper type without changing any layout,
        // because there is not currently a safe language/`std` way to accomplish this.
        let storage: Arc<UnsafeCell<[u8]>> =
            unsafe { Arc::from_raw(Arc::into_raw(storage) as *mut UnsafeCell<[u8]>) };

        Ok(Buffer { storage, size })
    }

    /// Returns a pointer to the memory owned by this buffer within the given `range`.
    ///
    /// This may be used to create any number of simultaneous pointers;
    /// aliasing is only a concern when actually reading, writing, or converting the pointer
    /// to a reference.
    pub(super) fn get_slice_ptr(&self, range: crate::MemoryRange) -> *mut [u8] {
        let base_ptr = self.storage.get();
        let range = range_to_usize(range, self.size);

        // We must obtain a slice pointer without ever creating a slice reference
        // that could alias with another slice.
        ptr::slice_from_raw_parts_mut(
            // SAFETY: `range_to_usize` bounds checks this addition.
            unsafe { base_ptr.cast::<u8>().add(range.start) },
            range.len(),
        )
    }
}

/// Convert a [`crate::MemoryRange`] to `Range<usize>` and bounds check it.
fn range_to_usize(range: crate::MemoryRange, upper_bound: usize) -> Range<usize> {
    // Note: these assertions should be impossible to trigger from safe code.
    // We're doing them anyway since this entire backend is for testing
    // (except for when it is an unused placeholder)
    let start = usize::try_from(range.start).expect("range too large");
    let end = usize::try_from(range.end).expect("range too large");
    assert!(start <= end && end <= upper_bound, "range out of bounds");
    start..end
}

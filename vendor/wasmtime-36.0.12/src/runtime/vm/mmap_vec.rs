use crate::prelude::*;
use crate::runtime::vm::send_sync_ptr::SendSyncPtr;
#[cfg(has_virtual_memory)]
use crate::runtime::vm::{Mmap, mmap::UnalignedLength};
#[cfg(not(has_virtual_memory))]
use alloc::alloc::Layout;
use alloc::sync::Arc;
use core::ops::{Deref, Range};
use core::ptr::NonNull;
#[cfg(feature = "std")]
use std::fs::File;

/// A type which prefers to store backing memory in an OS-backed memory mapping
/// but can fall back to the regular memory allocator as well.
///
/// This type is used to store code in Wasmtime and manage read-only and
/// executable permissions of compiled images. This is created from either an
/// in-memory compilation or by deserializing an artifact from disk. Methods
/// are provided for managing VM permissions when the `signals-based-traps`
/// Cargo feature is enabled.
///
/// The length of an `MmapVec` is not guaranteed to be page-aligned. That means
/// that if the contents are not themselves page-aligned, which compiled images
/// are typically not, then the remaining bytes in the final page for
/// mmap-backed instances are unused.
///
/// Note that when `signals-based-traps` is disabled then this type is
/// backed by the regular memory allocator via `alloc` APIs. In such a
/// scenario this type does not support read-only or executable bits
/// and the methods are not available. However, the `CustomCodeMemory`
/// mechanism may be used by the embedder to set up and tear down
/// executable permissions on parts of this storage.
pub enum MmapVec {
    #[doc(hidden)]
    #[cfg(not(has_virtual_memory))]
    Alloc {
        base: SendSyncPtr<u8>,
        layout: Layout,
    },
    #[doc(hidden)]
    ExternallyOwned { memory: SendSyncPtr<[u8]> },
    #[doc(hidden)]
    #[cfg(has_virtual_memory)]
    Mmap {
        mmap: Mmap<UnalignedLength>,
        len: usize,
    },
}

impl MmapVec {
    /// Consumes an existing `mmap` and wraps it up into an `MmapVec`.
    ///
    /// The returned `MmapVec` will have the `size` specified, which can be
    /// smaller than the region mapped by the `Mmap`. The returned `MmapVec`
    /// will only have at most `size` bytes accessible.
    #[cfg(has_virtual_memory)]
    fn new_mmap<M>(mmap: M, len: usize) -> MmapVec
    where
        M: Into<Mmap<UnalignedLength>>,
    {
        let mmap = mmap.into();
        assert!(len <= mmap.len());
        MmapVec::Mmap { mmap, len }
    }

    #[cfg(not(has_virtual_memory))]
    fn new_alloc(len: usize, alignment: usize) -> MmapVec {
        let layout = Layout::from_size_align(len, alignment)
            .expect("Invalid size or alignment for MmapVec allocation");
        let base = SendSyncPtr::new(
            NonNull::new(unsafe { alloc::alloc::alloc_zeroed(layout.clone()) })
                .expect("Allocation of MmapVec storage failed"),
        );
        MmapVec::Alloc { base, layout }
    }

    fn new_externally_owned(memory: NonNull<[u8]>) -> MmapVec {
        let memory = SendSyncPtr::new(memory);
        MmapVec::ExternallyOwned { memory }
    }

    /// Creates a new zero-initialized `MmapVec` with the given `size`
    /// and `alignment`.
    ///
    /// This commit will return a new `MmapVec` suitably sized to hold `size`
    /// bytes. All bytes will be initialized to zero since this is a fresh OS
    /// page allocation.
    pub fn with_capacity_and_alignment(size: usize, alignment: usize) -> Result<MmapVec> {
        #[cfg(has_virtual_memory)]
        {
            assert!(alignment <= crate::runtime::vm::host_page_size());
            return Ok(MmapVec::new_mmap(Mmap::with_at_least(size)?, size));
        }
        #[cfg(not(has_virtual_memory))]
        {
            return Ok(MmapVec::new_alloc(size, alignment));
        }
    }

    /// Creates a new `MmapVec` from the contents of an existing `slice`.
    ///
    /// A new `MmapVec` is allocated to hold the contents of `slice` and then
    /// `slice` is copied into the new mmap. It's recommended to avoid this
    /// method if possible to avoid the need to copy data around.
    pub fn from_slice(slice: &[u8]) -> Result<MmapVec> {
        MmapVec::from_slice_with_alignment(slice, 1)
    }

    /// Creates a new `MmapVec` from an existing memory region
    ///
    /// This method avoids the copy performed by [`Self::from_slice`] by
    /// directly using the memory region provided. This must be done with
    /// extreme care, however, as any concurrent modification of the provided
    /// memory will cause undefined and likely very, very bad things to
    /// happen.
    ///
    /// The memory provided is guaranteed to not be mutated by the runtime.
    ///
    /// # Safety
    ///
    /// As there is no copy here, the runtime will be making direct readonly use
    /// of the provided memory. As such, outside writes to this memory region
    /// will result in undefined and likely very undesirable behavior.
    pub unsafe fn from_raw(memory: NonNull<[u8]>) -> Result<MmapVec> {
        Ok(MmapVec::new_externally_owned(memory))
    }

    /// Creates a new `MmapVec` from the contents of an existing
    /// `slice`, with a minimum alignment.
    ///
    /// `align` must be a power of two. This is useful when page
    /// alignment is required when the system otherwise does not use
    /// virtual memory but has a custom code publish handler.
    ///
    /// A new `MmapVec` is allocated to hold the contents of `slice` and then
    /// `slice` is copied into the new mmap. It's recommended to avoid this
    /// method if possible to avoid the need to copy data around.
    pub fn from_slice_with_alignment(slice: &[u8], align: usize) -> Result<MmapVec> {
        let mut result = MmapVec::with_capacity_and_alignment(slice.len(), align)?;
        // SAFETY: The mmap hasn't been made readonly yet so this should be
        // safe to call.
        unsafe {
            result.as_mut_slice().copy_from_slice(slice);
        }
        Ok(result)
    }

    /// Return `true` if the `MmapVec` suport virtual memory operations
    ///
    /// In some cases, such as when using externally owned memory, the underlying
    /// platform may support virtual memory but it still may not be legal
    /// to perform virtual memory operations on this memory.
    pub fn supports_virtual_memory(&self) -> bool {
        match self {
            #[cfg(has_virtual_memory)]
            MmapVec::Mmap { .. } => true,
            MmapVec::ExternallyOwned { .. } => false,
            #[cfg(not(has_virtual_memory))]
            MmapVec::Alloc { .. } => false,
        }
    }

    /// Return true if this `MmapVec` is always readonly
    ///
    /// Attempting to get access to mutate readonly memory via
    /// [`MmapVec::as_mut`] will result in a panic.  Note that this method
    /// does not change with runtime changes to portions of the code memory
    /// via `MmapVec::make_readonly` for platforms with virtual memory.
    pub fn is_always_readonly(&self) -> bool {
        match self {
            #[cfg(has_virtual_memory)]
            MmapVec::Mmap { .. } => false,
            MmapVec::ExternallyOwned { .. } => true,
            #[cfg(not(has_virtual_memory))]
            MmapVec::Alloc { .. } => false,
        }
    }

    /// Creates a new `MmapVec` which is the given `File` mmap'd into memory.
    ///
    /// This function will determine the file's size and map the full contents
    /// into memory. This will return an error if the file is too large to be
    /// fully mapped into memory.
    ///
    /// The file is mapped into memory with a "private mapping" meaning that
    /// changes are not persisted back to the file itself and are only visible
    /// within this process.
    #[cfg(feature = "std")]
    pub fn from_file(file: File) -> Result<MmapVec> {
        let file = Arc::new(file);
        let mmap = Mmap::from_file(Arc::clone(&file))
            .with_context(move || format!("failed to create mmap for file {file:?}"))?;
        let len = mmap.len();
        Ok(MmapVec::new_mmap(mmap, len))
    }

    /// Makes the specified `range` within this `mmap` to be read/execute.
    #[cfg(has_virtual_memory)]
    pub unsafe fn make_executable(
        &self,
        range: Range<usize>,
        enable_branch_protection: bool,
    ) -> Result<()> {
        let (mmap, len) = match self {
            MmapVec::Mmap { mmap, len } => (mmap, *len),
            MmapVec::ExternallyOwned { .. } => {
                bail!("Unable to make externally owned memory executable");
            }
        };
        assert!(range.start <= range.end);
        assert!(range.end <= len);
        unsafe { mmap.make_executable(range.start..range.end, enable_branch_protection) }
    }

    /// Makes the specified `range` within this `mmap` to be read-only.
    #[cfg(has_virtual_memory)]
    pub unsafe fn make_readonly(&self, range: Range<usize>) -> Result<()> {
        let (mmap, len) = match self {
            MmapVec::Mmap { mmap, len } => (mmap, *len),
            MmapVec::ExternallyOwned { .. } => {
                bail!("Unable to make externally owned memory readonly");
            }
        };
        assert!(range.start <= range.end);
        assert!(range.end <= len);
        unsafe { mmap.make_readonly(range.start..range.end) }
    }

    /// Returns the underlying file that this mmap is mapping, if present.
    #[cfg(feature = "std")]
    pub fn original_file(&self) -> Option<&Arc<File>> {
        match self {
            #[cfg(not(has_virtual_memory))]
            MmapVec::Alloc { .. } => None,
            MmapVec::ExternallyOwned { .. } => None,
            #[cfg(has_virtual_memory)]
            MmapVec::Mmap { mmap, .. } => mmap.original_file(),
        }
    }

    /// Returns the bounds, in host memory, of where this mmap
    /// image resides.
    pub fn image_range(&self) -> Range<*const u8> {
        let base = self.as_ptr();
        let len = self.len();
        base..base.wrapping_add(len)
    }

    /// Views this region of memory as a mutable slice.
    ///
    /// # Unsafety
    ///
    /// This method is only safe if `make_readonly` hasn't been called yet to
    /// ensure that the memory is indeed writable.  For a MmapVec created from
    /// a raw pointer using this memory as mutable is only safe if there are
    /// no outside reads or writes to the memory region.
    ///
    /// Externally owned code is implicitly considered to be readonly and this
    /// code will panic if called on externally owned memory.
    pub unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            #[cfg(not(has_virtual_memory))]
            MmapVec::Alloc { base, layout } => unsafe {
                core::slice::from_raw_parts_mut(base.as_mut(), layout.size())
            },
            MmapVec::ExternallyOwned { .. } => {
                panic!("Mutating externally owned memory is prohibited");
            }
            #[cfg(has_virtual_memory)]
            MmapVec::Mmap { mmap, len } => unsafe { mmap.slice_mut(0..*len) },
        }
    }
}

impl Deref for MmapVec {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        match self {
            #[cfg(not(has_virtual_memory))]
            MmapVec::Alloc { base, layout } => unsafe {
                core::slice::from_raw_parts(base.as_ptr(), layout.size())
            },
            MmapVec::ExternallyOwned { memory } => unsafe { memory.as_ref() },
            #[cfg(has_virtual_memory)]
            MmapVec::Mmap { mmap, len } => {
                // SAFETY: all bytes for this mmap, which is owned by
                // `MmapVec`, are always at least readable.
                unsafe { mmap.slice(0..*len) }
            }
        }
    }
}

impl Drop for MmapVec {
    fn drop(&mut self) {
        match self {
            #[cfg(not(has_virtual_memory))]
            MmapVec::Alloc { base, layout, .. } => unsafe {
                alloc::alloc::dealloc(base.as_mut(), layout.clone());
            },
            MmapVec::ExternallyOwned { .. } => {
                // Memory is allocated externally, nothing to do
            }
            #[cfg(has_virtual_memory)]
            MmapVec::Mmap { .. } => {
                // Drop impl on the `mmap` takes care of this case.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MmapVec;

    #[test]
    fn smoke() {
        let mut mmap = MmapVec::with_capacity_and_alignment(10, 1).unwrap();
        assert_eq!(mmap.len(), 10);
        assert_eq!(&mmap[..], &[0; 10]);

        unsafe {
            mmap.as_mut_slice()[0] = 1;
            mmap.as_mut_slice()[2] = 3;
        }
        assert!(mmap.get(10).is_none());
        assert_eq!(mmap[0], 1);
        assert_eq!(mmap[2], 3);
    }

    #[test]
    fn alignment() {
        let mmap = MmapVec::with_capacity_and_alignment(10, 4096).unwrap();
        let raw_ptr = &mmap[0] as *const _ as usize;
        assert_eq!(raw_ptr & (4096 - 1), 0);
    }
}

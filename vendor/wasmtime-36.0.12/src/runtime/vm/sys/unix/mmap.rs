use crate::prelude::*;
use crate::runtime::vm::sys::vm::MemoryImageSource;
use crate::runtime::vm::{HostAlignedByteCount, SendSyncPtr};
use rustix::mm::{MprotectFlags, mprotect};
use std::ops::Range;
use std::ptr::{self, NonNull};
#[cfg(feature = "std")]
use std::{fs::File, path::Path};

/// Open a file so that it can be mmap'd for executing.
#[cfg(feature = "std")]
pub fn open_file_for_mmap(path: &Path) -> Result<File> {
    File::open(path).context("failed to open file")
}

#[derive(Debug)]
pub struct Mmap {
    memory: SendSyncPtr<[u8]>,
}

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "illumos", target_os = "linux"))] {
        // On illumos, by default, mmap reserves what it calls "swap space" ahead of time, so that
        // memory accesses a`re guaranteed not to fail once mmap succeeds. NORESERVE is for cases
        // where that memory is never meant to be accessed -- e.g. memory that's used as guard
        // pages.
        //
        // This is less crucial on Linux because Linux tends to overcommit memory by default, but is
        // still a good idea to pass in for large allocations that don't need to be backed by
        // physical memory.
        pub(super) const MMAP_NORESERVE_FLAG: rustix::mm::MapFlags =
            rustix::mm::MapFlags::NORESERVE;
    } else {
        pub(super) const MMAP_NORESERVE_FLAG: rustix::mm::MapFlags = rustix::mm::MapFlags::empty();
    }
}

impl Mmap {
    pub fn new_empty() -> Mmap {
        Mmap {
            memory: crate::vm::sys::empty_mmap(),
        }
    }

    pub fn new(size: HostAlignedByteCount) -> Result<Self> {
        let ptr = unsafe {
            rustix::mm::mmap_anonymous(
                ptr::null_mut(),
                size.byte_count(),
                rustix::mm::ProtFlags::READ | rustix::mm::ProtFlags::WRITE,
                rustix::mm::MapFlags::PRIVATE | MMAP_NORESERVE_FLAG,
            )?
        };
        let memory = std::ptr::slice_from_raw_parts_mut(ptr.cast(), size.byte_count());
        let memory = SendSyncPtr::new(NonNull::new(memory).unwrap());
        Ok(Mmap { memory })
    }

    pub fn reserve(size: HostAlignedByteCount) -> Result<Self> {
        let ptr = unsafe {
            rustix::mm::mmap_anonymous(
                ptr::null_mut(),
                size.byte_count(),
                rustix::mm::ProtFlags::empty(),
                // Astute readers might be wondering why a function called "reserve" passes in a
                // NORESERVE flag. That's because "reserve" in this context means one of two
                // different things.
                //
                // * This method is used to allocate virtual memory that starts off in a state where
                //   it cannot be accessed (i.e. causes a segfault if accessed).
                // * NORESERVE is meant for virtual memory space for which backing physical/swap
                //   pages are reserved on first access.
                //
                // Virtual memory that cannot be accessed should not have a backing store reserved
                // for it. Hence, passing in NORESERVE is correct here.
                rustix::mm::MapFlags::PRIVATE | MMAP_NORESERVE_FLAG,
            )?
        };

        let memory = std::ptr::slice_from_raw_parts_mut(ptr.cast(), size.byte_count());
        let memory = SendSyncPtr::new(NonNull::new(memory).unwrap());
        Ok(Mmap { memory })
    }

    #[cfg(feature = "std")]
    pub fn from_file(file: &File) -> Result<Self> {
        let len = file
            .metadata()
            .context("failed to get file metadata")?
            .len();
        let len = usize::try_from(len).map_err(|_| anyhow::anyhow!("file too large to map"))?;
        let ptr = unsafe {
            rustix::mm::mmap(
                ptr::null_mut(),
                len,
                rustix::mm::ProtFlags::READ | rustix::mm::ProtFlags::WRITE,
                rustix::mm::MapFlags::PRIVATE,
                &file,
                0,
            )
            .context(format!("mmap failed to allocate {len:#x} bytes"))?
        };
        let memory = std::ptr::slice_from_raw_parts_mut(ptr.cast(), len);
        let memory = SendSyncPtr::new(NonNull::new(memory).unwrap());

        Ok(Mmap { memory })
    }

    pub unsafe fn make_accessible(
        &self,
        start: HostAlignedByteCount,
        len: HostAlignedByteCount,
    ) -> Result<()> {
        let ptr = self.memory.as_ptr();
        unsafe {
            mprotect(
                ptr.byte_add(start.byte_count()).cast(),
                len.byte_count(),
                MprotectFlags::READ | MprotectFlags::WRITE,
            )?;
        }

        Ok(())
    }

    #[inline]
    pub fn as_send_sync_ptr(&self) -> SendSyncPtr<u8> {
        self.memory.cast()
    }

    #[inline]
    pub fn len(&self) -> usize {
        // Note: while the start of memory is host page-aligned, the length might
        // not be, and in particular is not aligned for file-backed mmaps. Be
        // careful!
        self.memory.as_ptr().len()
    }

    pub unsafe fn make_executable(
        &self,
        range: Range<usize>,
        enable_branch_protection: bool,
    ) -> Result<()> {
        let base = unsafe { self.memory.as_ptr().byte_add(range.start).cast() };
        let len = range.end - range.start;

        let flags = MprotectFlags::READ | MprotectFlags::EXEC;
        let flags = if enable_branch_protection {
            #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
            if std::arch::is_aarch64_feature_detected!("bti") {
                MprotectFlags::from_bits_retain(flags.bits() | /* PROT_BTI */ 0x10)
            } else {
                flags
            }

            #[cfg(not(all(target_arch = "aarch64", target_os = "linux")))]
            flags
        } else {
            flags
        };

        unsafe {
            mprotect(base, len, flags)?;
        }

        Ok(())
    }

    pub unsafe fn make_readonly(&self, range: Range<usize>) -> Result<()> {
        let base = unsafe { self.memory.as_ptr().byte_add(range.start).cast() };
        let len = range.end - range.start;

        unsafe {
            mprotect(base, len, MprotectFlags::READ)?;
        }

        Ok(())
    }

    pub unsafe fn map_image_at(
        &self,
        image_source: &MemoryImageSource,
        source_offset: u64,
        memory_offset: HostAlignedByteCount,
        memory_len: HostAlignedByteCount,
    ) -> Result<()> {
        unsafe {
            let map_base = self.memory.as_ptr().byte_add(memory_offset.byte_count());
            let ptr = rustix::mm::mmap(
                map_base.cast(),
                memory_len.byte_count(),
                rustix::mm::ProtFlags::READ | rustix::mm::ProtFlags::WRITE,
                rustix::mm::MapFlags::PRIVATE | rustix::mm::MapFlags::FIXED,
                image_source.as_file(),
                source_offset,
            )?;
            assert_eq!(map_base.cast(), ptr);
        };
        Ok(())
    }
}

impl Drop for Mmap {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.memory.as_ptr().cast();
            let len = self.memory.as_ptr().len();
            if len == 0 {
                return;
            }
            rustix::mm::munmap(ptr, len).expect("munmap failed");
        }
    }
}

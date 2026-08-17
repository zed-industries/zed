//! A "dummy" implementation of mmaps for miri where "we do the best we can"
//!
//! Namely this uses `alloc` to allocate memory for the "mmap" specifically to
//! create page-aligned allocations. This allocation doesn't handle operations
//! like becoming executable or becoming readonly or being created from files,
//! but it's enough to get various tests running relying on memories and such.

use crate::prelude::*;
use crate::runtime::vm::sys::vm::MemoryImageSource;
use crate::runtime::vm::{HostAlignedByteCount, SendSyncPtr};
use std::alloc::{self, Layout};
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::ptr::NonNull;

pub fn open_file_for_mmap(path: &Path) -> Result<File> {
    let file = File::open(path)?;
    Ok(file)
}

#[derive(Debug)]
pub struct Mmap {
    memory: SendSyncPtr<[u8]>,
}

impl Mmap {
    pub fn new_empty() -> Mmap {
        Mmap {
            memory: crate::vm::sys::empty_mmap(),
        }
    }

    pub fn new(size: HostAlignedByteCount) -> Result<Self> {
        let ret = Mmap::reserve(size)?;
        // SAFETY: The memory was just created so no one else has access to it.
        unsafe {
            ret.make_accessible(HostAlignedByteCount::ZERO, size)?;
        }
        Ok(ret)
    }

    pub fn reserve(size: HostAlignedByteCount) -> Result<Self> {
        // Miri will abort execution on OOM instead of returning null from
        // `alloc::alloc` so detect "definitely too large" requests that the
        // test suite does and fail accordingly.
        if (size.byte_count() as u64) > 1 << 32 {
            bail!("failed to allocate memory");
        }
        let layout = make_layout(size.byte_count());
        let ptr = unsafe { alloc::alloc(layout) };
        if ptr.is_null() {
            bail!("failed to allocate memory");
        }

        let memory = std::ptr::slice_from_raw_parts_mut(ptr.cast(), size.byte_count());
        let memory = SendSyncPtr::new(NonNull::new(memory).unwrap());
        Ok(Mmap { memory })
    }

    pub fn from_file(mut file: &File) -> Result<Self> {
        // Read the file and copy it in to a fresh "mmap" to have allocation for
        // an mmap only in one location.
        let mut dst = Vec::new();
        file.read_to_end(&mut dst)?;
        let count = HostAlignedByteCount::new_rounded_up(dst.len())?;
        let result = Mmap::new(count)?;
        unsafe {
            std::ptr::copy_nonoverlapping(
                dst.as_ptr(),
                result.as_send_sync_ptr().as_ptr(),
                dst.len(),
            );
        }
        Ok(result)
    }

    pub unsafe fn make_accessible(
        &self,
        start: HostAlignedByteCount,
        len: HostAlignedByteCount,
    ) -> Result<()> {
        // The memory is technically always accessible but this marks it as
        // initialized for miri-level checking.
        unsafe {
            std::ptr::write_bytes(
                self.as_send_sync_ptr().as_ptr().add(start.byte_count()),
                0u8,
                len.byte_count(),
            );
        }
        Ok(())
    }

    #[inline]
    pub fn as_send_sync_ptr(&self) -> SendSyncPtr<u8> {
        self.memory.cast()
    }

    pub fn len(&self) -> usize {
        self.memory.as_ptr().len()
    }

    pub unsafe fn make_executable(
        &self,
        _range: Range<usize>,
        _enable_branch_protection: bool,
    ) -> Result<()> {
        Ok(())
    }

    pub unsafe fn make_readonly(&self, _range: Range<usize>) -> Result<()> {
        Ok(())
    }

    pub unsafe fn map_image_at(
        &self,
        image_source: &MemoryImageSource,
        _source_offset: u64,
        _memory_offset: HostAlignedByteCount,
        _memory_len: HostAlignedByteCount,
    ) -> Result<()> {
        match *image_source {}
    }
}

impl Drop for Mmap {
    fn drop(&mut self) {
        if self.len() == 0 {
            return;
        }
        unsafe {
            let layout = make_layout(self.len());
            alloc::dealloc(self.as_send_sync_ptr().as_ptr(), layout);
        }
    }
}

fn make_layout(size: usize) -> Layout {
    Layout::from_size_align(size, crate::runtime::vm::host_page_size()).unwrap()
}

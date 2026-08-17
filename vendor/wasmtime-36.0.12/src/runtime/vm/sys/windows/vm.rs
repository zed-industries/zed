use crate::vm::sys::DecommitBehavior;
use std::fs::File;
use std::io;
use std::mem::MaybeUninit;
use std::sync::Arc;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::System::SystemInformation::*;

pub unsafe fn expose_existing_mapping(ptr: *mut u8, len: usize) -> io::Result<()> {
    if len == 0 {
        return Ok(());
    }
    if unsafe { VirtualAlloc(ptr.cast(), len, MEM_COMMIT, PAGE_READWRITE).is_null() } {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub unsafe fn hide_existing_mapping(ptr: *mut u8, len: usize) -> io::Result<()> {
    unsafe { erase_existing_mapping(ptr, len) }
}

pub unsafe fn erase_existing_mapping(ptr: *mut u8, len: usize) -> io::Result<()> {
    if len == 0 {
        return Ok(());
    }
    if unsafe { VirtualFree(ptr.cast(), len, MEM_DECOMMIT) == 0 } {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(feature = "pooling-allocator")]
pub unsafe fn commit_pages(addr: *mut u8, len: usize) -> io::Result<()> {
    unsafe { expose_existing_mapping(addr, len) }
}

#[cfg(feature = "pooling-allocator")]
pub unsafe fn decommit_pages(addr: *mut u8, len: usize) -> io::Result<()> {
    unsafe { erase_existing_mapping(addr, len) }
}

pub fn get_page_size() -> usize {
    unsafe {
        let mut info = MaybeUninit::uninit();
        GetSystemInfo(info.as_mut_ptr());
        info.assume_init_ref().dwPageSize as usize
    }
}

pub fn decommit_behavior() -> DecommitBehavior {
    DecommitBehavior::Zero
}

#[derive(PartialEq, Debug)]
pub enum MemoryImageSource {}

impl MemoryImageSource {
    pub fn from_file(_file: &Arc<File>) -> Option<MemoryImageSource> {
        None
    }

    pub fn from_data(_data: &[u8]) -> io::Result<Option<MemoryImageSource>> {
        Ok(None)
    }

    pub unsafe fn remap_as_zeros_at(&self, _base: *mut u8, _len: usize) -> io::Result<()> {
        match *self {}
    }
}

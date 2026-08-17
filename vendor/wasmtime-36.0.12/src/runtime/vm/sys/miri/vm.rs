use crate::vm::sys::DecommitBehavior;
use std::fs::File;
use std::io;
use std::sync::Arc;

pub unsafe fn expose_existing_mapping(ptr: *mut u8, len: usize) -> io::Result<()> {
    unsafe {
        std::ptr::write_bytes(ptr, 0u8, len);
    }
    Ok(())
}

pub unsafe fn hide_existing_mapping(ptr: *mut u8, len: usize) -> io::Result<()> {
    unsafe {
        std::ptr::write_bytes(ptr, 0, len);
    }
    Ok(())
}

pub unsafe fn erase_existing_mapping(ptr: *mut u8, len: usize) -> io::Result<()> {
    unsafe {
        std::ptr::write_bytes(ptr, 0, len);
    }
    Ok(())
}

#[cfg(feature = "pooling-allocator")]
pub unsafe fn commit_pages(ptr: *mut u8, len: usize) -> io::Result<()> {
    unsafe {
        std::ptr::write_bytes(ptr, 0, len);
    }
    Ok(())
}

pub unsafe fn decommit_pages(ptr: *mut u8, len: usize) -> io::Result<()> {
    unsafe {
        std::ptr::write_bytes(ptr, 0, len);
    }
    Ok(())
}

pub fn get_page_size() -> usize {
    4096
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

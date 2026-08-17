use std::io;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

use windows::Win32::System::Memory::*;
use windows::Win32::System::SystemInformation::*;

use super::SysStack;

#[path = "overflow_windows.rs"]
pub mod overflow;

pub unsafe fn allocate_stack(size: usize) -> io::Result<SysStack> {
    let ptr = VirtualAlloc(
        Some(ptr::null()),
        size,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    );

    if ptr.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(SysStack::new(
            (ptr as usize + size) as *mut c_void,
            ptr as *mut c_void,
        ))
    }
}

pub unsafe fn protect_stack(stack: &SysStack) -> io::Result<SysStack> {
    let page_size = page_size();
    let mut old_prot = mem::zeroed();

    debug_assert!(stack.len() % page_size == 0 && stack.len() != 0);

    let ret = VirtualProtect(
        stack.bottom(),
        page_size,
        PAGE_READONLY | PAGE_GUARD,
        &mut old_prot,
    );

    if ret.is_err() {
        Err(io::Error::last_os_error())
    } else {
        let bottom = (stack.bottom() as usize + page_size) as *mut c_void;
        Ok(SysStack::new(stack.top(), bottom))
    }
}

pub unsafe fn deallocate_stack(ptr: *mut c_void, _: usize) {
    let _ = VirtualFree(ptr, 0, MEM_RELEASE);
}

pub fn page_size() -> usize {
    static PAGE_SIZE: AtomicUsize = AtomicUsize::new(0);

    let mut ret = PAGE_SIZE.load(Ordering::Relaxed);

    if ret == 0 {
        ret = unsafe {
            let mut info = mem::zeroed();
            GetSystemInfo(&mut info);
            info.dwPageSize as usize
        };

        PAGE_SIZE.store(ret, Ordering::Relaxed);
    }

    ret
}

// Windows does not seem to provide a stack limit API
pub fn min_stack_size() -> usize {
    page_size()
}

// Windows does not seem to provide a stack limit API
pub fn max_stack_size() -> usize {
    usize::MAX
}

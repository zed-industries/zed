//! This module corresponds to `mach/vm_sync.h`.

pub type vm_sync_t = ::libc::c_uint;

pub const VM_SYNC_ASYNCHRONOUS: vm_sync_t = 1;
pub const VM_SYNC_SYNCHRONOUS: vm_sync_t = 1 << 1;
pub const VM_SYNC_INVALIDATE: vm_sync_t = 1 << 2;
pub const VM_SYNC_KILLPAGES: vm_sync_t = 1 << 3;
pub const VM_SYNC_DEACTIVATE: vm_sync_t = 1 << 4;
pub const VM_SYNC_CONTIGUOUS: vm_sync_t = 1 << 5;
pub const VM_SYNC_REUSABLEPAGES: vm_sync_t = 1 << 6;

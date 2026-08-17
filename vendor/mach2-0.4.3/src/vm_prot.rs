//! This module corresponds to `mach/vm_prot.h`.

pub type vm_prot_t = ::libc::c_int;

pub const VM_PROT_NONE: vm_prot_t = 0;
pub const VM_PROT_READ: vm_prot_t = 1;
pub const VM_PROT_WRITE: vm_prot_t = 1 << 1;
pub const VM_PROT_EXECUTE: vm_prot_t = 1 << 2;
pub const VM_PROT_NO_CHANGE: vm_prot_t = 1 << 3;
pub const VM_PROT_COPY: vm_prot_t = 1 << 4;
pub const VM_PROT_WANTS_COPY: vm_prot_t = 1 << 4;
pub const VM_PROT_DEFAULT: vm_prot_t = VM_PROT_READ | VM_PROT_WRITE;
pub const VM_PROT_ALL: vm_prot_t = VM_PROT_READ | VM_PROT_WRITE | VM_PROT_EXECUTE;

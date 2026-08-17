//! This module corresponds to `mach/vm_inherit.h`.

pub type vm_inherit_t = ::libc::c_uint;

pub const VM_INHERIT_SHARE: vm_inherit_t = 0;
pub const VM_INHERIT_COPY: vm_inherit_t = 1;
pub const VM_INHERIT_NONE: vm_inherit_t = 2;
pub const VM_INHERIT_DONATE_COPY: vm_inherit_t = 3;
pub const VM_INHERIT_DEFAULT: vm_inherit_t = VM_INHERIT_COPY;
pub const VM_INHERIT_LAST_VALID: vm_inherit_t = VM_INHERIT_NONE;

//! This module roughly corresponds to `mach/task_info.h`.

use vm_types::{integer_t, mach_vm_address_t, mach_vm_size_t, natural_t};

pub const TASK_INFO_MAX: ::libc::c_uint = 1024;
pub const TASK_BASIC_INFO_32: ::libc::c_uint = 4;
pub const TASK_BASIC2_INFO_32: ::libc::c_uint = 6;
#[cfg(target_arch = "x86_64")]
pub const TASK_BASIC_INFO_64: ::libc::c_uint = 5;
#[cfg(target_arch = "aarch64")]
pub const TASK_BASIC_INFO_64: ::libc::c_uint = 18;
#[cfg(target_arch = "x86_64")]
pub const TASK_BASIC_INFO: ::libc::c_uint = 5;
#[cfg(target_arch = "aarch64")]
pub const TASK_BASIC_INFO: ::libc::c_uint = 18;
#[cfg(target_arch = "x86")]
pub const TASK_BASIC_INFO: ::libc::c_uint = 4;
pub const TASK_EVENTS_INFO: ::libc::c_uint = 2;
pub const TASK_THREAD_TIMES_INFO: ::libc::c_uint = 3;
pub const TASK_ABSOLUTETIME_INFO: ::libc::c_uint = 1;
pub const TASK_KERNELMEMORY_INFO: ::libc::c_uint = 7;
pub const TASK_SECURITY_TOKEN: ::libc::c_uint = 13;
pub const TASK_AUDIT_TOKEN: ::libc::c_uint = 15;
pub const TASK_AFFINITY_TAG_INFO: ::libc::c_uint = 16;
pub const TASK_DYLD_INFO: ::libc::c_uint = 17;
pub const TASK_DYLD_ALL_IMAGE_INFO_32: ::libc::c_uint = 0;
pub const TASK_DYLD_ALL_IMAGE_INFO_64: ::libc::c_uint = 1;
pub const TASK_EXTMOD_INFO: ::libc::c_uint = 19;
pub const MACH_TASK_BASIC_INFO: ::libc::c_uint = 20;
pub const TASK_POWER_INFO: ::libc::c_uint = 21;
pub const TASK_VM_INFO: ::libc::c_uint = 22;
pub const TASK_VM_INFO_PURGEABLE: ::libc::c_uint = 23;
pub const TASK_TRACE_MEMORY_INFO: ::libc::c_uint = 24;
pub const TASK_WAIT_STATE_INFO: ::libc::c_uint = 25;
pub const TASK_POWER_INFO_V2: ::libc::c_uint = 26;
pub const TASK_VM_INFO_PURGEABLE_ACCOUNT: ::libc::c_uint = 27;
pub const TASK_FLAGS_INFO: ::libc::c_uint = 28;
pub const TASK_DEBUG_INFO_INTERNAL: ::libc::c_uint = 29;

pub type task_flavor_t = natural_t;
pub type task_info_t = *mut integer_t;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_dyld_info {
    pub all_image_info_addr: mach_vm_address_t,
    pub all_image_info_size: mach_vm_size_t,
    pub all_image_info_format: integer_t,
}

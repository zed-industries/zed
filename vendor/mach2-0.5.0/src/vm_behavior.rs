//! This module corresponds to `mach/vm_behavior.h`.

pub type vm_behavior_t = ::libc::c_int;

pub const VM_BEHAVIOR_DEFAULT: vm_behavior_t = 0;
pub const VM_BEHAVIOR_RANDOM: vm_behavior_t = 1;
pub const VM_BEHAVIOR_SEQUENTIAL: vm_behavior_t = 2;
pub const VM_BEHAVIOR_RSEQNTL: vm_behavior_t = 3;
pub const VM_BEHAVIOR_WILLNEED: vm_behavior_t = 4;
pub const VM_BEHAVIOR_DONTNEED: vm_behavior_t = 5;
pub const VM_BEHAVIOR_FREE: vm_behavior_t = 6;
pub const VM_BEHAVIOR_ZERO_WIRED_PAGES: vm_behavior_t = 7;
pub const VM_BEHAVIOR_REUSABLE: vm_behavior_t = 8;
pub const VM_BEHAVIOR_REUSE: vm_behavior_t = 9;
pub const VM_BEHAVIOR_CAN_REUSE: vm_behavior_t = 10;

//! This module roughly corresponds to `mach/i386/vm_types.h` and `mach/arm/vm_types.h` on aarch64.

pub type natural_t = ::libc::c_uint;
pub type integer_t = ::libc::c_int;

pub type user_addr_t = u64;

pub type mach_vm_address_t = u64;
pub type mach_vm_offset_t = u64;
pub type mach_vm_size_t = u64;
pub type vm_map_offset_t = u64;
pub type vm_map_address_t = u64;
pub type vm_map_size_t = u64;
pub type vm_map_t = ::port::mach_port_t;
pub type vm_offset_t = ::libc::uintptr_t;
pub type vm_size_t = ::libc::uintptr_t;
pub type vm_address_t = vm_offset_t;

pub type mach_port_context_t = mach_vm_address_t;

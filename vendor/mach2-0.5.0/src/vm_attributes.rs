//! This module corresponds to `mach/vm_attributes.h`.

pub type vm_machine_attribute_t = ::libc::c_uint;

pub const MATTR_CACHE: vm_machine_attribute_t = 1;
pub const MATTR_MIGRATE: vm_machine_attribute_t = 1 << 1;
pub const MATTR_REPLICATE: vm_machine_attribute_t = 1 << 2;

pub type vm_machine_attribute_val_t = ::libc::c_int;

pub const MATTR_VAL_OFF: vm_machine_attribute_val_t = 0;
pub const MATTR_VAL_ON: vm_machine_attribute_val_t = 1;
pub const MATTR_VAL_GET: vm_machine_attribute_val_t = 2;
pub const MATTR_VAL_CACHE_FLUSH: vm_machine_attribute_val_t = 6;
pub const MATTR_VAL_DCACHE_FLUSH: vm_machine_attribute_val_t = 7;
pub const MATTR_VAL_ICACHE_FLUSH: vm_machine_attribute_val_t = 8;
pub const MATTR_VAL_CACHE_SYNC: vm_machine_attribute_val_t = 9;
pub const MATTR_VAL_GET_INFO: vm_machine_attribute_val_t = 10;

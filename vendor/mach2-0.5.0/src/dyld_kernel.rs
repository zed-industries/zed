//! This module roughly corresponds to `mach/dyld_kernel.h`.

use boolean::boolean_t;
use mach_types::{fsid_t, fsobj_id_t, uuid_t};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_kernel_image_info {
    pub uuid: uuid_t,
    pub fsobjid: fsobj_id_t,
    pub fsid: fsid_t,
    pub load_addr: u64,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_kernel_process_info {
    pub cache_image_info: dyld_kernel_image_info,
    pub timestamp: u64,
    pub imageCount: u32,
    pub initialImageCount: u32,
    pub dyldState: u8,
    pub no_cache: boolean_t,
    pub private_cache: boolean_t,
}

pub type dyld_kernel_image_info_t = dyld_kernel_image_info;
pub type dyld_kernel_process_info_t = dyld_kernel_process_info;
pub type dyld_kernel_image_info_array_t = *mut dyld_kernel_image_info_t;

//! This module roughly corresponds to `mach-o/dyld.h`.

use loader::mach_header;

extern "C" {
    pub fn _dyld_image_count() -> u32;
    pub fn _dyld_get_image_header(image_index: u32) -> *const mach_header;
    pub fn _dyld_get_image_vmaddr_slide(image_index: u32) -> ::libc::intptr_t;
    pub fn _dyld_get_image_name(image_index: u32) -> *const ::libc::c_char;
}

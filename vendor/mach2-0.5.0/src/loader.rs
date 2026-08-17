//! This module roughly corresponds to `mach-o/loader.h`.

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct mach_header {
    pub magic: u32,
    pub cputype: ::libc::cpu_type_t,
    pub cpusubtype: ::libc::cpu_subtype_t,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
}

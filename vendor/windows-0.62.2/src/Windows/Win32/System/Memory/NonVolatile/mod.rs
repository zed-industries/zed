#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlDrainNonVolatileFlush(nvtoken: *const core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlDrainNonVolatileFlush(nvtoken : *const core::ffi::c_void) -> u32);
    unsafe { RtlDrainNonVolatileFlush(nvtoken) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFillNonVolatileMemory(nvtoken: *const core::ffi::c_void, nvdestination: *mut core::ffi::c_void, size: usize, value: u8, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFillNonVolatileMemory(nvtoken : *const core::ffi::c_void, nvdestination : *mut core::ffi::c_void, size : usize, value : u8, flags : u32) -> u32);
    unsafe { RtlFillNonVolatileMemory(nvtoken, nvdestination as _, size, value, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemory(nvtoken: *const core::ffi::c_void, nvbuffer: *const core::ffi::c_void, size: usize, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFlushNonVolatileMemory(nvtoken : *const core::ffi::c_void, nvbuffer : *const core::ffi::c_void, size : usize, flags : u32) -> u32);
    unsafe { RtlFlushNonVolatileMemory(nvtoken, nvbuffer, size, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemoryRanges(nvtoken: *const core::ffi::c_void, nvranges: &[NV_MEMORY_RANGE], flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFlushNonVolatileMemoryRanges(nvtoken : *const core::ffi::c_void, nvranges : *const NV_MEMORY_RANGE, numranges : usize, flags : u32) -> u32);
    unsafe { RtlFlushNonVolatileMemoryRanges(nvtoken, core::mem::transmute(nvranges.as_ptr()), nvranges.len().try_into().unwrap(), flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFreeNonVolatileToken(nvtoken: *const core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeNonVolatileToken(nvtoken : *const core::ffi::c_void) -> u32);
    unsafe { RtlFreeNonVolatileToken(nvtoken) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlGetNonVolatileToken(nvbuffer: *const core::ffi::c_void, size: usize, nvtoken: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlGetNonVolatileToken(nvbuffer : *const core::ffi::c_void, size : usize, nvtoken : *mut *mut core::ffi::c_void) -> u32);
    unsafe { RtlGetNonVolatileToken(nvbuffer, size, nvtoken as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlWriteNonVolatileMemory(nvtoken: *const core::ffi::c_void, nvdestination: *mut core::ffi::c_void, source: *const core::ffi::c_void, size: usize, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlWriteNonVolatileMemory(nvtoken : *const core::ffi::c_void, nvdestination : *mut core::ffi::c_void, source : *const core::ffi::c_void, size : usize, flags : u32) -> u32);
    unsafe { RtlWriteNonVolatileMemory(nvtoken, nvdestination as _, source, size, flags) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut core::ffi::c_void,
    pub Length: usize,
}
impl Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

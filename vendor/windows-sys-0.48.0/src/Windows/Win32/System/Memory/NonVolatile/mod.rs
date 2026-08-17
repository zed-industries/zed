#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"] fn RtlDrainNonVolatileFlush ( nvtoken : *const ::core::ffi::c_void ) -> u32 );
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"] fn RtlFillNonVolatileMemory ( nvtoken : *const ::core::ffi::c_void , nvdestination : *mut ::core::ffi::c_void , size : usize , value : u8 , flags : u32 ) -> u32 );
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"] fn RtlFlushNonVolatileMemory ( nvtoken : *const ::core::ffi::c_void , nvbuffer : *const ::core::ffi::c_void , size : usize , flags : u32 ) -> u32 );
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"] fn RtlFlushNonVolatileMemoryRanges ( nvtoken : *const ::core::ffi::c_void , nvranges : *const NV_MEMORY_RANGE , numranges : usize , flags : u32 ) -> u32 );
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"] fn RtlFreeNonVolatileToken ( nvtoken : *const ::core::ffi::c_void ) -> u32 );
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"] fn RtlGetNonVolatileToken ( nvbuffer : *const ::core::ffi::c_void , size : usize , nvtoken : *mut *mut ::core::ffi::c_void ) -> u32 );
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"] fn RtlWriteNonVolatileMemory ( nvtoken : *const ::core::ffi::c_void , nvdestination : *mut ::core::ffi::c_void , source : *const ::core::ffi::c_void , size : usize , flags : u32 ) -> u32 );
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub Length: usize,
}
impl ::core::marker::Copy for NV_MEMORY_RANGE {}
impl ::core::clone::Clone for NV_MEMORY_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}

#[inline]
pub unsafe fn CloseCompressor(compressorhandle: COMPRESSOR_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn CloseCompressor(compressorhandle : COMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CloseCompressor(compressorhandle).ok() }
}
#[inline]
pub unsafe fn CloseDecompressor(decompressorhandle: DECOMPRESSOR_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn CloseDecompressor(decompressorhandle : DECOMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CloseDecompressor(decompressorhandle).ok() }
}
#[inline]
pub unsafe fn Compress(compressorhandle: COMPRESSOR_HANDLE, uncompresseddata: Option<*const core::ffi::c_void>, uncompresseddatasize: usize, compressedbuffer: Option<*mut core::ffi::c_void>, compressedbuffersize: usize, compresseddatasize: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn Compress(compressorhandle : COMPRESSOR_HANDLE, uncompresseddata : *const core::ffi::c_void, uncompresseddatasize : usize, compressedbuffer : *mut core::ffi::c_void, compressedbuffersize : usize, compresseddatasize : *mut usize) -> windows_core::BOOL);
    unsafe { Compress(compressorhandle, uncompresseddata.unwrap_or(core::mem::zeroed()) as _, uncompresseddatasize, compressedbuffer.unwrap_or(core::mem::zeroed()) as _, compressedbuffersize, compresseddatasize as _).ok() }
}
#[inline]
pub unsafe fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: Option<*const COMPRESS_ALLOCATION_ROUTINES>, compressorhandle: *mut COMPRESSOR_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn CreateCompressor(algorithm : COMPRESS_ALGORITHM, allocationroutines : *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle : *mut COMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CreateCompressor(algorithm, allocationroutines.unwrap_or(core::mem::zeroed()) as _, compressorhandle as _).ok() }
}
#[inline]
pub unsafe fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: Option<*const COMPRESS_ALLOCATION_ROUTINES>, decompressorhandle: *mut DECOMPRESSOR_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn CreateDecompressor(algorithm : COMPRESS_ALGORITHM, allocationroutines : *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle : *mut DECOMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CreateDecompressor(algorithm, allocationroutines.unwrap_or(core::mem::zeroed()) as _, decompressorhandle as _).ok() }
}
#[inline]
pub unsafe fn Decompress(decompressorhandle: DECOMPRESSOR_HANDLE, compresseddata: Option<*const core::ffi::c_void>, compresseddatasize: usize, uncompressedbuffer: Option<*mut core::ffi::c_void>, uncompressedbuffersize: usize, uncompresseddatasize: Option<*mut usize>) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn Decompress(decompressorhandle : DECOMPRESSOR_HANDLE, compresseddata : *const core::ffi::c_void, compresseddatasize : usize, uncompressedbuffer : *mut core::ffi::c_void, uncompressedbuffersize : usize, uncompresseddatasize : *mut usize) -> windows_core::BOOL);
    unsafe { Decompress(decompressorhandle, compresseddata.unwrap_or(core::mem::zeroed()) as _, compresseddatasize, uncompressedbuffer.unwrap_or(core::mem::zeroed()) as _, uncompressedbuffersize, uncompresseddatasize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn QueryCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut core::ffi::c_void, compressinformationsize: usize) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn QueryCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *mut core::ffi::c_void, compressinformationsize : usize) -> windows_core::BOOL);
    unsafe { QueryCompressorInformation(compressorhandle, compressinformationclass, compressinformation as _, compressinformationsize).ok() }
}
#[inline]
pub unsafe fn QueryDecompressorInformation(decompressorhandle: DECOMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut core::ffi::c_void, compressinformationsize: usize) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn QueryDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *mut core::ffi::c_void, compressinformationsize : usize) -> windows_core::BOOL);
    unsafe { QueryDecompressorInformation(decompressorhandle, compressinformationclass, compressinformation as _, compressinformationsize).ok() }
}
#[inline]
pub unsafe fn ResetCompressor(compressorhandle: COMPRESSOR_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn ResetCompressor(compressorhandle : COMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { ResetCompressor(compressorhandle).ok() }
}
#[inline]
pub unsafe fn ResetDecompressor(decompressorhandle: DECOMPRESSOR_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn ResetDecompressor(decompressorhandle : DECOMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { ResetDecompressor(decompressorhandle).ok() }
}
#[inline]
pub unsafe fn SetCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const core::ffi::c_void, compressinformationsize: usize) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn SetCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *const core::ffi::c_void, compressinformationsize : usize) -> windows_core::BOOL);
    unsafe { SetCompressorInformation(compressorhandle, compressinformationclass, compressinformation, compressinformationsize).ok() }
}
#[inline]
pub unsafe fn SetDecompressorInformation(decompressorhandle: DECOMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const core::ffi::c_void, compressinformationsize: usize) -> windows_core::Result<()> {
    windows_core::link!("cabinet.dll" "system" fn SetDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *const core::ffi::c_void, compressinformationsize : usize) -> windows_core::BOOL);
    unsafe { SetDecompressorInformation(decompressorhandle, compressinformationclass, compressinformation, compressinformationsize).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct COMPRESSOR_HANDLE(pub *mut core::ffi::c_void);
impl COMPRESSOR_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for COMPRESSOR_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("cabinet.dll" "system" fn CloseCompressor(compressorhandle : *mut core::ffi::c_void) -> i32);
            unsafe {
                CloseCompressor(self.0);
            }
        }
    }
}
impl Default for COMPRESSOR_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPRESS_ALGORITHM(pub u32);
pub const COMPRESS_ALGORITHM_INVALID: u32 = 0u32;
pub const COMPRESS_ALGORITHM_LZMS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(5u32);
pub const COMPRESS_ALGORITHM_MAX: u32 = 6u32;
pub const COMPRESS_ALGORITHM_MSZIP: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(2u32);
pub const COMPRESS_ALGORITHM_NULL: u32 = 1u32;
pub const COMPRESS_ALGORITHM_XPRESS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(3u32);
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct COMPRESS_ALLOCATION_ROUTINES {
    pub Allocate: PFN_COMPRESS_ALLOCATE,
    pub Free: PFN_COMPRESS_FREE,
    pub UserContext: *mut core::ffi::c_void,
}
impl Default for COMPRESS_ALLOCATION_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPRESS_INFORMATION_CLASS(pub i32);
pub const COMPRESS_INFORMATION_CLASS_BLOCK_SIZE: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(1i32);
pub const COMPRESS_INFORMATION_CLASS_INVALID: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(0i32);
pub const COMPRESS_INFORMATION_CLASS_LEVEL: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(2i32);
pub const COMPRESS_RAW: u32 = 536870912u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DECOMPRESSOR_HANDLE(pub *mut core::ffi::c_void);
impl DECOMPRESSOR_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for DECOMPRESSOR_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("cabinet.dll" "system" fn CloseDecompressor(decompressorhandle : *mut core::ffi::c_void) -> i32);
            unsafe {
                CloseDecompressor(self.0);
            }
        }
    }
}
impl Default for DECOMPRESSOR_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFN_COMPRESS_ALLOCATE = Option<unsafe extern "system" fn(usercontext: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type PFN_COMPRESS_FREE = Option<unsafe extern "system" fn(usercontext: *const core::ffi::c_void, memory: *const core::ffi::c_void)>;

#[inline]
pub unsafe fn CloseCompressor<P0>(compressorhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<COMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn CloseCompressor(compressorhandle : COMPRESSOR_HANDLE) -> super::super::Foundation:: BOOL);
    CloseCompressor(compressorhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseDecompressor<P0>(decompressorhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<DECOMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn CloseDecompressor(decompressorhandle : DECOMPRESSOR_HANDLE) -> super::super::Foundation:: BOOL);
    CloseDecompressor(decompressorhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn Compress<P0>(compressorhandle: P0, uncompresseddata: Option<*const core::ffi::c_void>, uncompresseddatasize: usize, compressedbuffer: Option<*mut core::ffi::c_void>, compressedbuffersize: usize, compresseddatasize: *mut usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<COMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn Compress(compressorhandle : COMPRESSOR_HANDLE, uncompresseddata : *const core::ffi::c_void, uncompresseddatasize : usize, compressedbuffer : *mut core::ffi::c_void, compressedbuffersize : usize, compresseddatasize : *mut usize) -> super::super::Foundation:: BOOL);
    Compress(compressorhandle.param().abi(), core::mem::transmute(uncompresseddata.unwrap_or(std::ptr::null())), uncompresseddatasize, core::mem::transmute(compressedbuffer.unwrap_or(std::ptr::null_mut())), compressedbuffersize, compresseddatasize).ok()
}
#[inline]
pub unsafe fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: Option<*const COMPRESS_ALLOCATION_ROUTINES>, compressorhandle: *mut COMPRESSOR_HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("cabinet.dll" "system" fn CreateCompressor(algorithm : COMPRESS_ALGORITHM, allocationroutines : *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle : *mut COMPRESSOR_HANDLE) -> super::super::Foundation:: BOOL);
    CreateCompressor(algorithm, core::mem::transmute(allocationroutines.unwrap_or(std::ptr::null())), compressorhandle).ok()
}
#[inline]
pub unsafe fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: Option<*const COMPRESS_ALLOCATION_ROUTINES>, decompressorhandle: *mut DECOMPRESSOR_HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("cabinet.dll" "system" fn CreateDecompressor(algorithm : COMPRESS_ALGORITHM, allocationroutines : *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle : *mut DECOMPRESSOR_HANDLE) -> super::super::Foundation:: BOOL);
    CreateDecompressor(algorithm, core::mem::transmute(allocationroutines.unwrap_or(std::ptr::null())), decompressorhandle).ok()
}
#[inline]
pub unsafe fn Decompress<P0>(decompressorhandle: P0, compresseddata: Option<*const core::ffi::c_void>, compresseddatasize: usize, uncompressedbuffer: Option<*mut core::ffi::c_void>, uncompressedbuffersize: usize, uncompresseddatasize: Option<*mut usize>) -> windows_core::Result<()>
where
    P0: windows_core::Param<DECOMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn Decompress(decompressorhandle : DECOMPRESSOR_HANDLE, compresseddata : *const core::ffi::c_void, compresseddatasize : usize, uncompressedbuffer : *mut core::ffi::c_void, uncompressedbuffersize : usize, uncompresseddatasize : *mut usize) -> super::super::Foundation:: BOOL);
    Decompress(decompressorhandle.param().abi(), core::mem::transmute(compresseddata.unwrap_or(std::ptr::null())), compresseddatasize, core::mem::transmute(uncompressedbuffer.unwrap_or(std::ptr::null_mut())), uncompressedbuffersize, core::mem::transmute(uncompresseddatasize.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn QueryCompressorInformation<P0>(compressorhandle: P0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut core::ffi::c_void, compressinformationsize: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<COMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn QueryCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *mut core::ffi::c_void, compressinformationsize : usize) -> super::super::Foundation:: BOOL);
    QueryCompressorInformation(compressorhandle.param().abi(), compressinformationclass, compressinformation, compressinformationsize).ok()
}
#[inline]
pub unsafe fn QueryDecompressorInformation<P0>(decompressorhandle: P0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut core::ffi::c_void, compressinformationsize: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<DECOMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn QueryDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *mut core::ffi::c_void, compressinformationsize : usize) -> super::super::Foundation:: BOOL);
    QueryDecompressorInformation(decompressorhandle.param().abi(), compressinformationclass, compressinformation, compressinformationsize).ok()
}
#[inline]
pub unsafe fn ResetCompressor<P0>(compressorhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<COMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn ResetCompressor(compressorhandle : COMPRESSOR_HANDLE) -> super::super::Foundation:: BOOL);
    ResetCompressor(compressorhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn ResetDecompressor<P0>(decompressorhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<DECOMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn ResetDecompressor(decompressorhandle : DECOMPRESSOR_HANDLE) -> super::super::Foundation:: BOOL);
    ResetDecompressor(decompressorhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn SetCompressorInformation<P0>(compressorhandle: P0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const core::ffi::c_void, compressinformationsize: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<COMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn SetCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *const core::ffi::c_void, compressinformationsize : usize) -> super::super::Foundation:: BOOL);
    SetCompressorInformation(compressorhandle.param().abi(), compressinformationclass, compressinformation, compressinformationsize).ok()
}
#[inline]
pub unsafe fn SetDecompressorInformation<P0>(decompressorhandle: P0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const core::ffi::c_void, compressinformationsize: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<DECOMPRESSOR_HANDLE>,
{
    windows_targets::link!("cabinet.dll" "system" fn SetDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *const core::ffi::c_void, compressinformationsize : usize) -> super::super::Foundation:: BOOL);
    SetDecompressorInformation(decompressorhandle.param().abi(), compressinformationclass, compressinformation, compressinformationsize).ok()
}
pub const COMPRESS_ALGORITHM_INVALID: u32 = 0u32;
pub const COMPRESS_ALGORITHM_LZMS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(5u32);
pub const COMPRESS_ALGORITHM_MAX: u32 = 6u32;
pub const COMPRESS_ALGORITHM_MSZIP: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(2u32);
pub const COMPRESS_ALGORITHM_NULL: u32 = 1u32;
pub const COMPRESS_ALGORITHM_XPRESS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(3u32);
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(4u32);
pub const COMPRESS_INFORMATION_CLASS_BLOCK_SIZE: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(1i32);
pub const COMPRESS_INFORMATION_CLASS_INVALID: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(0i32);
pub const COMPRESS_INFORMATION_CLASS_LEVEL: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(2i32);
pub const COMPRESS_RAW: u32 = 536870912u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPRESS_ALGORITHM(pub u32);
impl windows_core::TypeKind for COMPRESS_ALGORITHM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPRESS_ALGORITHM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPRESS_ALGORITHM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPRESS_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for COMPRESS_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPRESS_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPRESS_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct COMPRESSOR_HANDLE(pub *mut core::ffi::c_void);
impl COMPRESSOR_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for COMPRESSOR_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COMPRESSOR_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct COMPRESS_ALLOCATION_ROUTINES {
    pub Allocate: PFN_COMPRESS_ALLOCATE,
    pub Free: PFN_COMPRESS_FREE,
    pub UserContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for COMPRESS_ALLOCATION_ROUTINES {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPRESS_ALLOCATION_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
            _ = CloseDecompressor(*self);
        }
    }
}
impl Default for DECOMPRESSOR_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DECOMPRESSOR_HANDLE {
    type TypeKind = windows_core::CopyType;
}
pub type PFN_COMPRESS_ALLOCATE = Option<unsafe extern "system" fn(usercontext: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type PFN_COMPRESS_FREE = Option<unsafe extern "system" fn(usercontext: *const core::ffi::c_void, memory: *const core::ffi::c_void)>;

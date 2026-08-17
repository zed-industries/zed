#[cfg(feature = "Wdk_Storage_FileSystem_Minifilters")]
pub mod Minifilters;
#[inline]
pub unsafe fn ApplyControlToken(phcontext: *const SecHandle, pinput: *const SecBufferDesc) -> windows_core::Result<()> {
    windows_targets::link!("secur32.dll" "system" fn ApplyControlToken(phcontext : *const SecHandle, pinput : *const SecBufferDesc) -> windows_core::HRESULT);
    ApplyControlToken(phcontext, pinput).ok()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcAsyncCopyRead<P0, P1>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, wait: P0, buffer: *mut core::ffi::c_void, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, ioissuerthread: P1, asyncreadcontext: *const CC_ASYNC_READ_CONTEXT) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcAsyncCopyRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, ioissuerthread : super::super::Foundation:: PETHREAD, asyncreadcontext : *const CC_ASYNC_READ_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcAsyncCopyRead(fileobject, fileoffset, length, wait.param().abi(), buffer, iostatus, ioissuerthread.param().abi(), asyncreadcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcCanIWrite<P0>(fileobject: Option<*const super::super::Foundation::FILE_OBJECT>, bytestowrite: u32, wait: P0, retrying: u8) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcCanIWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, bytestowrite : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, retrying : u8) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcCanIWrite(core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), bytestowrite, wait.param().abi(), retrying)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CcCoherencyFlushAndPurgeCache(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, fileoffset: Option<*const i64>, length: u32, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, flags: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcCoherencyFlushAndPurgeCache(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, fileoffset : *const i64, length : u32, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, flags : u32));
    CcCoherencyFlushAndPurgeCache(sectionobjectpointer, core::mem::transmute(fileoffset.unwrap_or(std::ptr::null())), length, iostatus, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcCopyRead<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, wait: P0, buffer: *mut core::ffi::c_void, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcCopyRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcCopyRead(fileobject, fileoffset, length, wait.param().abi(), buffer, iostatus)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcCopyReadEx<P0, P1>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, wait: P0, buffer: *mut core::ffi::c_void, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, ioissuerthread: P1) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcCopyReadEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, ioissuerthread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcCopyReadEx(fileobject, fileoffset, length, wait.param().abi(), buffer, iostatus, ioissuerthread.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcCopyWrite<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, wait: P0, buffer: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcCopyWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcCopyWrite(fileobject, fileoffset, length, wait.param().abi(), buffer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcCopyWriteEx<P0, P1>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, wait: P0, buffer: *const core::ffi::c_void, ioissuerthread: P1) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcCopyWriteEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *const core::ffi::c_void, ioissuerthread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcCopyWriteEx(fileobject, fileoffset, length, wait.param().abi(), buffer, ioissuerthread.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcCopyWriteWontFlush(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: Option<*const i64>, length: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcCopyWriteWontFlush(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcCopyWriteWontFlush(fileobject, core::mem::transmute(fileoffset.unwrap_or(std::ptr::null())), length)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcDeferWrite<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, postroutine: PCC_POST_DEFERRED_WRITE, context1: *const core::ffi::c_void, context2: *const core::ffi::c_void, bytestowrite: u32, retrying: P0)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcDeferWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, postroutine : PCC_POST_DEFERRED_WRITE, context1 : *const core::ffi::c_void, context2 : *const core::ffi::c_void, bytestowrite : u32, retrying : super::super::super::Win32::Foundation:: BOOLEAN));
    CcDeferWrite(fileobject, postroutine, context1, context2, bytestowrite, retrying.param().abi())
}
#[inline]
pub unsafe fn CcErrorCallbackRoutine(context: *const CC_ERROR_CALLBACK_CONTEXT) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcErrorCallbackRoutine(context : *const CC_ERROR_CALLBACK_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    CcErrorCallbackRoutine(context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcFastCopyRead(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: u32, length: u32, pagecount: u32, buffer: *mut core::ffi::c_void, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcFastCopyRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : u32, length : u32, pagecount : u32, buffer : *mut core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK));
    CcFastCopyRead(fileobject, fileoffset, length, pagecount, buffer, iostatus)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcFastCopyWrite(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: u32, length: u32, buffer: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcFastCopyWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : u32, length : u32, buffer : *const core::ffi::c_void));
    CcFastCopyWrite(fileobject, fileoffset, length, buffer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CcFlushCache(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, fileoffset: Option<*const i64>, length: u32, iostatus: Option<*mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK>) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcFlushCache(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, fileoffset : *const i64, length : u32, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK));
    CcFlushCache(sectionobjectpointer, core::mem::transmute(fileoffset.unwrap_or(std::ptr::null())), length, core::mem::transmute(iostatus.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcGetDirtyPages(loghandle: *const core::ffi::c_void, dirtypageroutine: PDIRTY_PAGE_ROUTINE, context1: *const core::ffi::c_void, context2: *const core::ffi::c_void) -> i64 {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcGetDirtyPages(loghandle : *const core::ffi::c_void, dirtypageroutine : PDIRTY_PAGE_ROUTINE, context1 : *const core::ffi::c_void, context2 : *const core::ffi::c_void) -> i64);
    CcGetDirtyPages(loghandle, dirtypageroutine, context1, context2)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcGetFileObjectFromBcb(bcb: *const core::ffi::c_void) -> *mut super::super::Foundation::FILE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcGetFileObjectFromBcb(bcb : *const core::ffi::c_void) -> *mut super::super::Foundation:: FILE_OBJECT);
    CcGetFileObjectFromBcb(bcb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcGetFileObjectFromSectionPtrs(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS) -> *mut super::super::Foundation::FILE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcGetFileObjectFromSectionPtrs(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS) -> *mut super::super::Foundation:: FILE_OBJECT);
    CcGetFileObjectFromSectionPtrs(sectionobjectpointer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcGetFileObjectFromSectionPtrsRef(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS) -> *mut super::super::Foundation::FILE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcGetFileObjectFromSectionPtrsRef(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS) -> *mut super::super::Foundation:: FILE_OBJECT);
    CcGetFileObjectFromSectionPtrsRef(sectionobjectpointer)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn CcGetFlushedValidData<P0>(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, bcblistheld: P0) -> i64
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcGetFlushedValidData(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, bcblistheld : super::super::super::Win32::Foundation:: BOOLEAN) -> i64);
    CcGetFlushedValidData(sectionobjectpointer, bcblistheld.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcInitializeCacheMap<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, filesizes: *const CC_FILE_SIZES, pinaccess: P0, callbacks: *const CACHE_MANAGER_CALLBACKS, lazywritecontext: *const core::ffi::c_void)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcInitializeCacheMap(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesizes : *const CC_FILE_SIZES, pinaccess : super::super::super::Win32::Foundation:: BOOLEAN, callbacks : *const CACHE_MANAGER_CALLBACKS, lazywritecontext : *const core::ffi::c_void));
    CcInitializeCacheMap(fileobject, filesizes, pinaccess.param().abi(), callbacks, lazywritecontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcInitializeCacheMapEx<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, filesizes: *const CC_FILE_SIZES, pinaccess: P0, callbacks: *const CACHE_MANAGER_CALLBACKS, lazywritecontext: *const core::ffi::c_void, flags: u32)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcInitializeCacheMapEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesizes : *const CC_FILE_SIZES, pinaccess : super::super::super::Win32::Foundation:: BOOLEAN, callbacks : *const CACHE_MANAGER_CALLBACKS, lazywritecontext : *const core::ffi::c_void, flags : u32));
    CcInitializeCacheMapEx(fileobject, filesizes, pinaccess.param().abi(), callbacks, lazywritecontext, flags)
}
#[inline]
pub unsafe fn CcIsCacheManagerCallbackNeeded<P0>(status: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcIsCacheManagerCallbackNeeded(status : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcIsCacheManagerCallbackNeeded(status.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcIsThereDirtyData(vpb: *const super::super::Foundation::VPB) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcIsThereDirtyData(vpb : *const super::super::Foundation:: VPB) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcIsThereDirtyData(vpb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcIsThereDirtyDataEx(vpb: *const super::super::Foundation::VPB, numberofdirtypages: Option<*const u32>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcIsThereDirtyDataEx(vpb : *const super::super::Foundation:: VPB, numberofdirtypages : *const u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcIsThereDirtyDataEx(vpb, core::mem::transmute(numberofdirtypages.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcMapData(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, flags: u32, bcb: *mut *mut core::ffi::c_void, buffer: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcMapData(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, flags : u32, bcb : *mut *mut core::ffi::c_void, buffer : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcMapData(fileobject, fileoffset, length, flags, bcb, buffer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcMdlRead(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, mdlchain: *mut *mut super::super::Foundation::MDL, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcMdlRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, mdlchain : *mut *mut super::super::Foundation:: MDL, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK));
    CcMdlRead(fileobject, fileoffset, length, mdlchain, iostatus)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcMdlReadComplete(fileobject: *const super::super::Foundation::FILE_OBJECT, mdlchain: *const super::super::Foundation::MDL) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcMdlReadComplete(fileobject : *const super::super::Foundation:: FILE_OBJECT, mdlchain : *const super::super::Foundation:: MDL));
    CcMdlReadComplete(fileobject, mdlchain)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcMdlWriteAbort(fileobject: *const super::super::Foundation::FILE_OBJECT, mdlchain: *const super::super::Foundation::MDL) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcMdlWriteAbort(fileobject : *const super::super::Foundation:: FILE_OBJECT, mdlchain : *const super::super::Foundation:: MDL));
    CcMdlWriteAbort(fileobject, mdlchain)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcMdlWriteComplete(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, mdlchain: *const super::super::Foundation::MDL) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcMdlWriteComplete(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, mdlchain : *const super::super::Foundation:: MDL));
    CcMdlWriteComplete(fileobject, fileoffset, mdlchain)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcPinMappedData(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, flags: u32, bcb: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcPinMappedData(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, flags : u32, bcb : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcPinMappedData(fileobject, fileoffset, length, flags, bcb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcPinRead(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, flags: u32, bcb: *mut *mut core::ffi::c_void, buffer: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcPinRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, flags : u32, bcb : *mut *mut core::ffi::c_void, buffer : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcPinRead(fileobject, fileoffset, length, flags, bcb, buffer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcPrepareMdlWrite(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, mdlchain: *mut *mut super::super::Foundation::MDL, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcPrepareMdlWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, mdlchain : *mut *mut super::super::Foundation:: MDL, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK));
    CcPrepareMdlWrite(fileobject, fileoffset, length, mdlchain, iostatus)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcPreparePinWrite<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, zero: P0, flags: u32, bcb: *mut *mut core::ffi::c_void, buffer: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcPreparePinWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, zero : super::super::super::Win32::Foundation:: BOOLEAN, flags : u32, bcb : *mut *mut core::ffi::c_void, buffer : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcPreparePinWrite(fileobject, fileoffset, length, zero.param().abi(), flags, bcb, buffer)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn CcPurgeCacheSection(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, fileoffset: Option<*const i64>, length: u32, flags: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcPurgeCacheSection(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, fileoffset : *const i64, length : u32, flags : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcPurgeCacheSection(sectionobjectpointer, core::mem::transmute(fileoffset.unwrap_or(std::ptr::null())), length, flags)
}
#[inline]
pub unsafe fn CcRemapBcb(bcb: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcRemapBcb(bcb : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    CcRemapBcb(bcb)
}
#[inline]
pub unsafe fn CcRepinBcb(bcb: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcRepinBcb(bcb : *const core::ffi::c_void));
    CcRepinBcb(bcb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcScheduleReadAhead(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcScheduleReadAhead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32));
    CcScheduleReadAhead(fileobject, fileoffset, length)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcScheduleReadAheadEx<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, ioissuerthread: P0)
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcScheduleReadAheadEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, ioissuerthread : super::super::Foundation:: PETHREAD));
    CcScheduleReadAheadEx(fileobject, fileoffset, length, ioissuerthread.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcSetAdditionalCacheAttributes<P0, P1>(fileobject: *const super::super::Foundation::FILE_OBJECT, disablereadahead: P0, disablewritebehind: P1)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetAdditionalCacheAttributes(fileobject : *const super::super::Foundation:: FILE_OBJECT, disablereadahead : super::super::super::Win32::Foundation:: BOOLEAN, disablewritebehind : super::super::super::Win32::Foundation:: BOOLEAN));
    CcSetAdditionalCacheAttributes(fileobject, disablereadahead.param().abi(), disablewritebehind.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcSetAdditionalCacheAttributesEx(fileobject: *const super::super::Foundation::FILE_OBJECT, flags: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetAdditionalCacheAttributesEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, flags : u32));
    CcSetAdditionalCacheAttributesEx(fileobject, flags)
}
#[inline]
pub unsafe fn CcSetBcbOwnerPointer(bcb: *const core::ffi::c_void, ownerpointer: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetBcbOwnerPointer(bcb : *const core::ffi::c_void, ownerpointer : *const core::ffi::c_void));
    CcSetBcbOwnerPointer(bcb, ownerpointer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcSetDirtyPageThreshold(fileobject: *const super::super::Foundation::FILE_OBJECT, dirtypagethreshold: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetDirtyPageThreshold(fileobject : *const super::super::Foundation:: FILE_OBJECT, dirtypagethreshold : u32));
    CcSetDirtyPageThreshold(fileobject, dirtypagethreshold)
}
#[inline]
pub unsafe fn CcSetDirtyPinnedData(bcbvoid: *const core::ffi::c_void, lsn: Option<*const i64>) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetDirtyPinnedData(bcbvoid : *const core::ffi::c_void, lsn : *const i64));
    CcSetDirtyPinnedData(bcbvoid, core::mem::transmute(lsn.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcSetFileSizes(fileobject: *const super::super::Foundation::FILE_OBJECT, filesizes: *const CC_FILE_SIZES) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetFileSizes(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesizes : *const CC_FILE_SIZES));
    CcSetFileSizes(fileobject, filesizes)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcSetFileSizesEx(fileobject: *const super::super::Foundation::FILE_OBJECT, filesizes: *const CC_FILE_SIZES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetFileSizesEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesizes : *const CC_FILE_SIZES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    CcSetFileSizesEx(fileobject, filesizes)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcSetLogHandleForFile(fileobject: *const super::super::Foundation::FILE_OBJECT, loghandle: *const core::ffi::c_void, flushtolsnroutine: PFLUSH_TO_LSN) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetLogHandleForFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, loghandle : *const core::ffi::c_void, flushtolsnroutine : PFLUSH_TO_LSN));
    CcSetLogHandleForFile(fileobject, loghandle, flushtolsnroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcSetParallelFlushFile<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, enableparallelflush: P0)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetParallelFlushFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, enableparallelflush : super::super::super::Win32::Foundation:: BOOLEAN));
    CcSetParallelFlushFile(fileobject, enableparallelflush.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcSetReadAheadGranularity(fileobject: *const super::super::Foundation::FILE_OBJECT, granularity: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcSetReadAheadGranularity(fileobject : *const super::super::Foundation:: FILE_OBJECT, granularity : u32));
    CcSetReadAheadGranularity(fileobject, granularity)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcUninitializeCacheMap(fileobject: *const super::super::Foundation::FILE_OBJECT, truncatesize: Option<*const i64>, uninitializeevent: Option<*const CACHE_UNINITIALIZE_EVENT>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcUninitializeCacheMap(fileobject : *const super::super::Foundation:: FILE_OBJECT, truncatesize : *const i64, uninitializeevent : *const CACHE_UNINITIALIZE_EVENT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcUninitializeCacheMap(fileobject, core::mem::transmute(truncatesize.unwrap_or(std::ptr::null())), core::mem::transmute(uninitializeevent.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CcUnpinData(bcb: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcUnpinData(bcb : *const core::ffi::c_void));
    CcUnpinData(bcb)
}
#[inline]
pub unsafe fn CcUnpinDataForThread(bcb: *const core::ffi::c_void, resourcethreadid: usize) {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcUnpinDataForThread(bcb : *const core::ffi::c_void, resourcethreadid : usize));
    CcUnpinDataForThread(bcb, resourcethreadid)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CcUnpinRepinnedBcb<P0>(bcb: *const core::ffi::c_void, writethrough: P0) -> super::super::super::Win32::System::IO::IO_STATUS_BLOCK
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcUnpinRepinnedBcb(bcb : *const core::ffi::c_void, writethrough : super::super::super::Win32::Foundation:: BOOLEAN, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK));
    let mut result__ = core::mem::zeroed();
    CcUnpinRepinnedBcb(bcb, writethrough.param().abi(), &mut result__);
    result__
}
#[inline]
pub unsafe fn CcWaitForCurrentLazyWriterActivity() -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn CcWaitForCurrentLazyWriterActivity() -> super::super::super::Win32::Foundation:: NTSTATUS);
    CcWaitForCurrentLazyWriterActivity()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn CcZeroData<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, startoffset: *const i64, endoffset: *const i64, wait: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn CcZeroData(fileobject : *const super::super::Foundation:: FILE_OBJECT, startoffset : *const i64, endoffset : *const i64, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    CcZeroData(fileobject, startoffset, endoffset, wait.param().abi())
}
#[inline]
pub unsafe fn CompleteAuthToken(phcontext: *const SecHandle, ptoken: *const SecBufferDesc) -> windows_core::Result<()> {
    windows_targets::link!("secur32.dll" "system" fn CompleteAuthToken(phcontext : *const SecHandle, ptoken : *const SecBufferDesc) -> windows_core::HRESULT);
    CompleteAuthToken(phcontext, ptoken).ok()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn ExDisableResourceBoostLite(resource: *const super::super::Foundation::ERESOURCE) {
    windows_targets::link!("ntoskrnl.exe" "system" fn ExDisableResourceBoostLite(resource : *const super::super::Foundation:: ERESOURCE));
    ExDisableResourceBoostLite(resource)
}
#[inline]
pub unsafe fn ExQueryPoolBlockSize(poolblock: *const core::ffi::c_void, quotacharged: *mut super::super::super::Win32::Foundation::BOOLEAN) -> usize {
    windows_targets::link!("ntoskrnl.exe" "system" fn ExQueryPoolBlockSize(poolblock : *const core::ffi::c_void, quotacharged : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> usize);
    ExQueryPoolBlockSize(poolblock, quotacharged)
}
#[inline]
pub unsafe fn ExportSecurityContext(phcontext: *const SecHandle, fflags: u32, ppackedcontext: *mut SecBuffer, ptoken: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("secur32.dll" "system" fn ExportSecurityContext(phcontext : *const SecHandle, fflags : u32, ppackedcontext : *mut SecBuffer, ptoken : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    ExportSecurityContext(phcontext, fflags, ppackedcontext, ptoken).ok()
}
#[inline]
pub unsafe fn FsRtlAcknowledgeEcp(ecpcontext: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAcknowledgeEcp(ecpcontext : *const core::ffi::c_void));
    FsRtlAcknowledgeEcp(ecpcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlAcquireFileExclusive(fileobject: *const super::super::Foundation::FILE_OBJECT) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAcquireFileExclusive(fileobject : *const super::super::Foundation:: FILE_OBJECT));
    FsRtlAcquireFileExclusive(fileobject)
}
#[inline]
pub unsafe fn FsRtlAddBaseMcbEntry(mcb: *mut BASE_MCB, vbn: i64, lbn: i64, sectorcount: i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAddBaseMcbEntry(mcb : *mut BASE_MCB, vbn : i64, lbn : i64, sectorcount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlAddBaseMcbEntry(mcb, vbn, lbn, sectorcount)
}
#[inline]
pub unsafe fn FsRtlAddBaseMcbEntryEx(mcb: *mut BASE_MCB, vbn: i64, lbn: i64, sectorcount: i64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAddBaseMcbEntryEx(mcb : *mut BASE_MCB, vbn : i64, lbn : i64, sectorcount : i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlAddBaseMcbEntryEx(mcb, vbn, lbn, sectorcount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlAddLargeMcbEntry(mcb: *mut LARGE_MCB, vbn: i64, lbn: i64, sectorcount: i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAddLargeMcbEntry(mcb : *mut LARGE_MCB, vbn : i64, lbn : i64, sectorcount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlAddLargeMcbEntry(mcb, vbn, lbn, sectorcount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlAddMcbEntry(mcb: *mut MCB, vbn: u32, lbn: u32, sectorcount: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAddMcbEntry(mcb : *mut MCB, vbn : u32, lbn : u32, sectorcount : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlAddMcbEntry(mcb, vbn, lbn, sectorcount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlAddToTunnelCache<P0>(cache: *mut TUNNEL, directorykey: u64, shortname: *const super::super::super::Win32::Foundation::UNICODE_STRING, longname: *const super::super::super::Win32::Foundation::UNICODE_STRING, keybyshortname: P0, datalength: u32, data: *const core::ffi::c_void)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAddToTunnelCache(cache : *mut TUNNEL, directorykey : u64, shortname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, longname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, keybyshortname : super::super::super::Win32::Foundation:: BOOLEAN, datalength : u32, data : *const core::ffi::c_void));
    FsRtlAddToTunnelCache(cache, directorykey, shortname, longname, keybyshortname.param().abi(), datalength, data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlAddToTunnelCacheEx(cache: *mut TUNNEL, directorykey: u64, shortname: *const super::super::super::Win32::Foundation::UNICODE_STRING, longname: *const super::super::super::Win32::Foundation::UNICODE_STRING, flags: u32, datalength: u32, data: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAddToTunnelCacheEx(cache : *mut TUNNEL, directorykey : u64, shortname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, longname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, flags : u32, datalength : u32, data : *const core::ffi::c_void));
    FsRtlAddToTunnelCacheEx(cache, directorykey, shortname, longname, flags, datalength, data)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlAllocateAePushLock(pooltype: super::super::Foundation::POOL_TYPE, tag: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAllocateAePushLock(pooltype : super::super::Foundation:: POOL_TYPE, tag : u32) -> *mut core::ffi::c_void);
    FsRtlAllocateAePushLock(pooltype, tag)
}
#[inline]
pub unsafe fn FsRtlAllocateExtraCreateParameter(ecptype: *const windows_core::GUID, sizeofcontext: u32, flags: u32, cleanupcallback: PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, pooltag: u32, ecpcontext: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAllocateExtraCreateParameter(ecptype : *const windows_core::GUID, sizeofcontext : u32, flags : u32, cleanupcallback : PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, pooltag : u32, ecpcontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlAllocateExtraCreateParameter(ecptype, sizeofcontext, flags, cleanupcallback, pooltag, ecpcontext)
}
#[inline]
pub unsafe fn FsRtlAllocateExtraCreateParameterFromLookasideList(ecptype: *const windows_core::GUID, sizeofcontext: u32, flags: u32, cleanupcallback: PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, lookasidelist: *mut core::ffi::c_void, ecpcontext: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAllocateExtraCreateParameterFromLookasideList(ecptype : *const windows_core::GUID, sizeofcontext : u32, flags : u32, cleanupcallback : PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, lookasidelist : *mut core::ffi::c_void, ecpcontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlAllocateExtraCreateParameterFromLookasideList(ecptype, sizeofcontext, flags, cleanupcallback, lookasidelist, ecpcontext)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlAllocateExtraCreateParameterList(flags: u32, ecplist: *mut *mut super::super::Foundation::ECP_LIST) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAllocateExtraCreateParameterList(flags : u32, ecplist : *mut *mut super::super::Foundation:: ECP_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlAllocateExtraCreateParameterList(flags, ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlAllocateFileLock(completelockirproutine: PCOMPLETE_LOCK_IRP_ROUTINE, unlockroutine: PUNLOCK_ROUTINE) -> *mut FILE_LOCK {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAllocateFileLock(completelockirproutine : PCOMPLETE_LOCK_IRP_ROUTINE, unlockroutine : PUNLOCK_ROUTINE) -> *mut FILE_LOCK);
    FsRtlAllocateFileLock(completelockirproutine, unlockroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlAllocateResource() -> *mut super::super::Foundation::ERESOURCE {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAllocateResource() -> *mut super::super::Foundation:: ERESOURCE);
    FsRtlAllocateResource()
}
#[inline]
pub unsafe fn FsRtlAreNamesEqual<P0>(constantnamea: *const super::super::super::Win32::Foundation::UNICODE_STRING, constantnameb: *const super::super::super::Win32::Foundation::UNICODE_STRING, ignorecase: P0, upcasetable: Option<*const u16>) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAreNamesEqual(constantnamea : *const super::super::super::Win32::Foundation:: UNICODE_STRING, constantnameb : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ignorecase : super::super::super::Win32::Foundation:: BOOLEAN, upcasetable : *const u16) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlAreNamesEqual(constantnamea, constantnameb, ignorecase.param().abi(), core::mem::transmute(upcasetable.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlAreThereCurrentOrInProgressFileLocks(filelock: *const FILE_LOCK) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAreThereCurrentOrInProgressFileLocks(filelock : *const FILE_LOCK) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlAreThereCurrentOrInProgressFileLocks(filelock)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlAreThereWaitingFileLocks(filelock: *const FILE_LOCK) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAreThereWaitingFileLocks(filelock : *const FILE_LOCK) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlAreThereWaitingFileLocks(filelock)
}
#[inline]
pub unsafe fn FsRtlAreVolumeStartupApplicationsComplete() -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlAreVolumeStartupApplicationsComplete() -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlAreVolumeStartupApplicationsComplete()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlBalanceReads(targetdevice: *const super::super::Foundation::DEVICE_OBJECT) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlBalanceReads(targetdevice : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlBalanceReads(targetdevice)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCancellableWaitForMultipleObjects(objectarray: &[*const core::ffi::c_void], waittype: super::super::super::Win32::System::Kernel::WAIT_TYPE, timeout: Option<*const i64>, waitblockarray: Option<*const super::super::Foundation::KWAIT_BLOCK>, irp: Option<*const super::super::Foundation::IRP>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCancellableWaitForMultipleObjects(count : u32, objectarray : *const *const core::ffi::c_void, waittype : super::super::super::Win32::System::Kernel:: WAIT_TYPE, timeout : *const i64, waitblockarray : *const super::super::Foundation:: KWAIT_BLOCK, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlCancellableWaitForMultipleObjects(objectarray.len().try_into().unwrap(), core::mem::transmute(objectarray.as_ptr()), waittype, core::mem::transmute(timeout.unwrap_or(std::ptr::null())), core::mem::transmute(waitblockarray.unwrap_or(std::ptr::null())), core::mem::transmute(irp.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCancellableWaitForSingleObject(object: *const core::ffi::c_void, timeout: Option<*const i64>, irp: Option<*const super::super::Foundation::IRP>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCancellableWaitForSingleObject(object : *const core::ffi::c_void, timeout : *const i64, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlCancellableWaitForSingleObject(object, core::mem::transmute(timeout.unwrap_or(std::ptr::null())), core::mem::transmute(irp.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlChangeBackingFileObject(currentfileobject: Option<*const super::super::Foundation::FILE_OBJECT>, newfileobject: *const super::super::Foundation::FILE_OBJECT, changebackingtype: FSRTL_CHANGE_BACKING_TYPE, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlChangeBackingFileObject(currentfileobject : *const super::super::Foundation:: FILE_OBJECT, newfileobject : *const super::super::Foundation:: FILE_OBJECT, changebackingtype : FSRTL_CHANGE_BACKING_TYPE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlChangeBackingFileObject(core::mem::transmute(currentfileobject.unwrap_or(std::ptr::null())), newfileobject, changebackingtype, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCheckLockForOplockRequest(filelock: *const FILE_LOCK, allocationsize: *const i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCheckLockForOplockRequest(filelock : *const FILE_LOCK, allocationsize : *const i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlCheckLockForOplockRequest(filelock, allocationsize)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCheckLockForReadAccess(filelock: *const FILE_LOCK, irp: *const super::super::Foundation::IRP) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCheckLockForReadAccess(filelock : *const FILE_LOCK, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlCheckLockForReadAccess(filelock, irp)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCheckLockForWriteAccess(filelock: *const FILE_LOCK, irp: *const super::super::Foundation::IRP) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCheckLockForWriteAccess(filelock : *const FILE_LOCK, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlCheckLockForWriteAccess(filelock, irp)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCheckOplock(oplock: *const *const core::ffi::c_void, irp: *const super::super::Foundation::IRP, context: Option<*const core::ffi::c_void>, completionroutine: POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine: POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCheckOplock(oplock : *const *const core::ffi::c_void, irp : *const super::super::Foundation:: IRP, context : *const core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlCheckOplock(oplock, irp, core::mem::transmute(context.unwrap_or(std::ptr::null())), completionroutine, postirproutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCheckOplockEx(oplock: *const *const core::ffi::c_void, irp: *const super::super::Foundation::IRP, flags: u32, context: Option<*const core::ffi::c_void>, completionroutine: POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine: POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCheckOplockEx(oplock : *const *const core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, context : *const core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlCheckOplockEx(oplock, irp, flags, core::mem::transmute(context.unwrap_or(std::ptr::null())), completionroutine, postirproutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCheckOplockEx2(oplock: *const *const core::ffi::c_void, irp: *const super::super::Foundation::IRP, flags: u32, flagsex2: u32, completionroutinecontext: Option<*const core::ffi::c_void>, completionroutine: POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine: POPLOCK_FS_PREPOST_IRP, timeout: u64, notifycontext: Option<*const core::ffi::c_void>, notifyroutine: POPLOCK_NOTIFY_ROUTINE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCheckOplockEx2(oplock : *const *const core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, flagsex2 : u32, completionroutinecontext : *const core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP, timeout : u64, notifycontext : *const core::ffi::c_void, notifyroutine : POPLOCK_NOTIFY_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlCheckOplockEx2(oplock, irp, flags, flagsex2, core::mem::transmute(completionroutinecontext.unwrap_or(std::ptr::null())), completionroutine, postirproutine, timeout, core::mem::transmute(notifycontext.unwrap_or(std::ptr::null())), notifyroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCheckUpperOplock(oplock: *const *const core::ffi::c_void, newloweroplockstate: u32, completionroutinecontext: Option<*const core::ffi::c_void>, completionroutine: POPLOCK_WAIT_COMPLETE_ROUTINE, prependroutine: POPLOCK_FS_PREPOST_IRP, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCheckUpperOplock(oplock : *const *const core::ffi::c_void, newloweroplockstate : u32, completionroutinecontext : *const core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, prependroutine : POPLOCK_FS_PREPOST_IRP, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlCheckUpperOplock(oplock, newloweroplockstate, core::mem::transmute(completionroutinecontext.unwrap_or(std::ptr::null())), completionroutine, prependroutine, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCopyRead<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, wait: P0, lockkey: u32, buffer: *mut core::ffi::c_void, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const super::super::Foundation::DEVICE_OBJECT) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCopyRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, lockkey : u32, buffer : *mut core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlCopyRead(fileobject, fileoffset, length, wait.param().abi(), lockkey, buffer, iostatus, deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCopyWrite<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, wait: P0, lockkey: u32, buffer: *const core::ffi::c_void, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const super::super::Foundation::DEVICE_OBJECT) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCopyWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, lockkey : u32, buffer : *const core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlCopyWrite(fileobject, fileoffset, length, wait.param().abi(), lockkey, buffer, iostatus, deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlCreateSectionForDataScan(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, sectionobject: *mut *mut core::ffi::c_void, sectionfilesize: Option<*mut i64>, fileobject: *const super::super::Foundation::FILE_OBJECT, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, maximumsize: Option<*const i64>, sectionpageprotection: u32, allocationattributes: u32, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCreateSectionForDataScan(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, sectionobject : *mut *mut core::ffi::c_void, sectionfilesize : *mut i64, fileobject : *const super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlCreateSectionForDataScan(sectionhandle, sectionobject, core::mem::transmute(sectionfilesize.unwrap_or(std::ptr::null_mut())), fileobject, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), core::mem::transmute(maximumsize.unwrap_or(std::ptr::null())), sectionpageprotection, allocationattributes, flags)
}
#[inline]
pub unsafe fn FsRtlCurrentBatchOplock(oplock: *const *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCurrentBatchOplock(oplock : *const *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlCurrentBatchOplock(oplock)
}
#[inline]
pub unsafe fn FsRtlCurrentOplock(oplock: *const *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCurrentOplock(oplock : *const *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlCurrentOplock(oplock)
}
#[inline]
pub unsafe fn FsRtlCurrentOplockH(oplock: *const *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlCurrentOplockH(oplock : *const *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlCurrentOplockH(oplock)
}
#[inline]
pub unsafe fn FsRtlDeleteExtraCreateParameterLookasideList(lookaside: *mut core::ffi::c_void, flags: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDeleteExtraCreateParameterLookasideList(lookaside : *mut core::ffi::c_void, flags : u32));
    FsRtlDeleteExtraCreateParameterLookasideList(lookaside, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlDeleteKeyFromTunnelCache(cache: *mut TUNNEL, directorykey: u64) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDeleteKeyFromTunnelCache(cache : *mut TUNNEL, directorykey : u64));
    FsRtlDeleteKeyFromTunnelCache(cache, directorykey)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlDeleteTunnelCache(cache: *mut TUNNEL) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDeleteTunnelCache(cache : *mut TUNNEL));
    FsRtlDeleteTunnelCache(cache)
}
#[inline]
pub unsafe fn FsRtlDeregisterUncProvider<P0>(handle: P0)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDeregisterUncProvider(handle : super::super::super::Win32::Foundation:: HANDLE));
    FsRtlDeregisterUncProvider(handle.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlDismountComplete<P0>(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, dismountstatus: P0)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDismountComplete(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, dismountstatus : super::super::super::Win32::Foundation:: NTSTATUS));
    FsRtlDismountComplete(deviceobject, dismountstatus.param().abi())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn FsRtlDissectDbcs(path: super::super::super::Win32::System::Kernel::STRING, firstname: *mut super::super::super::Win32::System::Kernel::STRING, remainingname: *mut super::super::super::Win32::System::Kernel::STRING) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDissectDbcs(path : super::super::super::Win32::System::Kernel:: STRING, firstname : *mut super::super::super::Win32::System::Kernel:: STRING, remainingname : *mut super::super::super::Win32::System::Kernel:: STRING));
    FsRtlDissectDbcs(core::mem::transmute(path), firstname, remainingname)
}
#[inline]
pub unsafe fn FsRtlDissectName(path: super::super::super::Win32::Foundation::UNICODE_STRING, firstname: *mut super::super::super::Win32::Foundation::UNICODE_STRING, remainingname: *mut super::super::super::Win32::Foundation::UNICODE_STRING) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDissectName(path : super::super::super::Win32::Foundation:: UNICODE_STRING, firstname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, remainingname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING));
    FsRtlDissectName(core::mem::transmute(path), firstname, remainingname)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn FsRtlDoesDbcsContainWildCards(name: *const super::super::super::Win32::System::Kernel::STRING) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDoesDbcsContainWildCards(name : *const super::super::super::Win32::System::Kernel:: STRING) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlDoesDbcsContainWildCards(name)
}
#[inline]
pub unsafe fn FsRtlDoesNameContainWildCards(name: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlDoesNameContainWildCards(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlDoesNameContainWildCards(name)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlFastCheckLockForRead(filelock: *const FILE_LOCK, startingbyte: *const i64, length: *const i64, key: u32, fileobject: *const super::super::Foundation::FILE_OBJECT, processid: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFastCheckLockForRead(filelock : *const FILE_LOCK, startingbyte : *const i64, length : *const i64, key : u32, fileobject : *const super::super::Foundation:: FILE_OBJECT, processid : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlFastCheckLockForRead(filelock, startingbyte, length, key, fileobject, processid)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlFastCheckLockForWrite(filelock: *const FILE_LOCK, startingbyte: *const i64, length: *const i64, key: u32, fileobject: *const core::ffi::c_void, processid: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFastCheckLockForWrite(filelock : *const FILE_LOCK, startingbyte : *const i64, length : *const i64, key : u32, fileobject : *const core::ffi::c_void, processid : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlFastCheckLockForWrite(filelock, startingbyte, length, key, fileobject, processid)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlFastUnlockAll<P0>(filelock: *const FILE_LOCK, fileobject: *const super::super::Foundation::FILE_OBJECT, processid: P0, context: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFastUnlockAll(filelock : *const FILE_LOCK, fileobject : *const super::super::Foundation:: FILE_OBJECT, processid : super::super::Foundation:: PEPROCESS, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlFastUnlockAll(filelock, fileobject, processid.param().abi(), core::mem::transmute(context.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlFastUnlockAllByKey<P0>(filelock: *const FILE_LOCK, fileobject: *const super::super::Foundation::FILE_OBJECT, processid: P0, key: u32, context: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFastUnlockAllByKey(filelock : *const FILE_LOCK, fileobject : *const super::super::Foundation:: FILE_OBJECT, processid : super::super::Foundation:: PEPROCESS, key : u32, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlFastUnlockAllByKey(filelock, fileobject, processid.param().abi(), key, core::mem::transmute(context.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlFastUnlockSingle<P0, P1>(filelock: *const FILE_LOCK, fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: *const i64, processid: P0, key: u32, context: Option<*const core::ffi::c_void>, alreadysynchronized: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFastUnlockSingle(filelock : *const FILE_LOCK, fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : *const i64, processid : super::super::Foundation:: PEPROCESS, key : u32, context : *const core::ffi::c_void, alreadysynchronized : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlFastUnlockSingle(filelock, fileobject, fileoffset, length, processid.param().abi(), key, core::mem::transmute(context.unwrap_or(std::ptr::null())), alreadysynchronized.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlFindExtraCreateParameter(ecplist: *const super::super::Foundation::ECP_LIST, ecptype: *const windows_core::GUID, ecpcontext: Option<*mut *mut core::ffi::c_void>, ecpcontextsize: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFindExtraCreateParameter(ecplist : *const super::super::Foundation:: ECP_LIST, ecptype : *const windows_core::GUID, ecpcontext : *mut *mut core::ffi::c_void, ecpcontextsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlFindExtraCreateParameter(ecplist, ecptype, core::mem::transmute(ecpcontext.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ecpcontextsize.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlFindInTunnelCache(cache: *const TUNNEL, directorykey: u64, name: *const super::super::super::Win32::Foundation::UNICODE_STRING, shortname: *mut super::super::super::Win32::Foundation::UNICODE_STRING, longname: *mut super::super::super::Win32::Foundation::UNICODE_STRING, datalength: *mut u32, data: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFindInTunnelCache(cache : *const TUNNEL, directorykey : u64, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, shortname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, longname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, datalength : *mut u32, data : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlFindInTunnelCache(cache, directorykey, name, shortname, longname, datalength, data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlFindInTunnelCacheEx(cache: *const TUNNEL, directorykey: u64, name: *const super::super::super::Win32::Foundation::UNICODE_STRING, shortname: *mut super::super::super::Win32::Foundation::UNICODE_STRING, longname: *mut super::super::super::Win32::Foundation::UNICODE_STRING, flags: u32, datalength: *mut u32, data: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFindInTunnelCacheEx(cache : *const TUNNEL, directorykey : u64, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, shortname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, longname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, flags : u32, datalength : *mut u32, data : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlFindInTunnelCacheEx(cache, directorykey, name, shortname, longname, flags, datalength, data)
}
#[inline]
pub unsafe fn FsRtlFreeAePushLock(aepushlock: *mut core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFreeAePushLock(aepushlock : *mut core::ffi::c_void));
    FsRtlFreeAePushLock(aepushlock)
}
#[inline]
pub unsafe fn FsRtlFreeExtraCreateParameter(ecpcontext: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFreeExtraCreateParameter(ecpcontext : *const core::ffi::c_void));
    FsRtlFreeExtraCreateParameter(ecpcontext)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlFreeExtraCreateParameterList(ecplist: *const super::super::Foundation::ECP_LIST) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFreeExtraCreateParameterList(ecplist : *const super::super::Foundation:: ECP_LIST));
    FsRtlFreeExtraCreateParameterList(ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlFreeFileLock(filelock: *const FILE_LOCK) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlFreeFileLock(filelock : *const FILE_LOCK));
    FsRtlFreeFileLock(filelock)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn FsRtlGetCurrentProcessLoaderList() -> *mut super::super::super::Win32::System::Kernel::LIST_ENTRY {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetCurrentProcessLoaderList() -> *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY);
    FsRtlGetCurrentProcessLoaderList()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlGetEcpListFromIrp(irp: *const super::super::Foundation::IRP, ecplist: *mut *mut super::super::Foundation::ECP_LIST) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetEcpListFromIrp(irp : *const super::super::Foundation:: IRP, ecplist : *mut *mut super::super::Foundation:: ECP_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlGetEcpListFromIrp(irp, ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlGetFileSize(fileobject: *const super::super::Foundation::FILE_OBJECT, filesize: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetFileSize(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesize : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlGetFileSize(fileobject, filesize)
}
#[inline]
pub unsafe fn FsRtlGetNextBaseMcbEntry(mcb: *const BASE_MCB, runindex: u32, vbn: *mut i64, lbn: *mut i64, sectorcount: *mut i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetNextBaseMcbEntry(mcb : *const BASE_MCB, runindex : u32, vbn : *mut i64, lbn : *mut i64, sectorcount : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlGetNextBaseMcbEntry(mcb, runindex, vbn, lbn, sectorcount)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlGetNextExtraCreateParameter(ecplist: *const super::super::Foundation::ECP_LIST, currentecpcontext: Option<*const core::ffi::c_void>, nextecptype: Option<*mut windows_core::GUID>, nextecpcontext: Option<*mut *mut core::ffi::c_void>, nextecpcontextsize: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetNextExtraCreateParameter(ecplist : *const super::super::Foundation:: ECP_LIST, currentecpcontext : *const core::ffi::c_void, nextecptype : *mut windows_core::GUID, nextecpcontext : *mut *mut core::ffi::c_void, nextecpcontextsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlGetNextExtraCreateParameter(ecplist, core::mem::transmute(currentecpcontext.unwrap_or(std::ptr::null())), core::mem::transmute(nextecptype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(nextecpcontext.unwrap_or(std::ptr::null_mut())), core::mem::transmute(nextecpcontextsize.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlGetNextFileLock<P0>(filelock: *const FILE_LOCK, restart: P0) -> *mut FILE_LOCK_INFO
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetNextFileLock(filelock : *const FILE_LOCK, restart : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut FILE_LOCK_INFO);
    FsRtlGetNextFileLock(filelock, restart.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlGetNextLargeMcbEntry(mcb: *const LARGE_MCB, runindex: u32, vbn: *mut i64, lbn: *mut i64, sectorcount: *mut i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetNextLargeMcbEntry(mcb : *const LARGE_MCB, runindex : u32, vbn : *mut i64, lbn : *mut i64, sectorcount : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlGetNextLargeMcbEntry(mcb, runindex, vbn, lbn, sectorcount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlGetNextMcbEntry(mcb: *const MCB, runindex: u32, vbn: *mut u32, lbn: *mut u32, sectorcount: *mut u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetNextMcbEntry(mcb : *const MCB, runindex : u32, vbn : *mut u32, lbn : *mut u32, sectorcount : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlGetNextMcbEntry(mcb, runindex, vbn, lbn, sectorcount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlGetSectorSizeInformation(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, sectorsizeinfo: *mut FILE_FS_SECTOR_SIZE_INFORMATION) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetSectorSizeInformation(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, sectorsizeinfo : *mut FILE_FS_SECTOR_SIZE_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlGetSectorSizeInformation(deviceobject, sectorsizeinfo)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlGetSupportedFeatures(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, supportedfeatures: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetSupportedFeatures(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, supportedfeatures : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlGetSupportedFeatures(deviceobject, supportedfeatures)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlGetVirtualDiskNestingLevel(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, nestinglevel: *mut u32, nestingflags: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlGetVirtualDiskNestingLevel(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, nestinglevel : *mut u32, nestingflags : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlGetVirtualDiskNestingLevel(deviceobject, nestinglevel, core::mem::transmute(nestingflags.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FsRtlIncrementCcFastMdlReadWait() {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIncrementCcFastMdlReadWait());
    FsRtlIncrementCcFastMdlReadWait()
}
#[inline]
pub unsafe fn FsRtlIncrementCcFastReadNoWait() {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIncrementCcFastReadNoWait());
    FsRtlIncrementCcFastReadNoWait()
}
#[inline]
pub unsafe fn FsRtlIncrementCcFastReadNotPossible() {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIncrementCcFastReadNotPossible());
    FsRtlIncrementCcFastReadNotPossible()
}
#[inline]
pub unsafe fn FsRtlIncrementCcFastReadResourceMiss() {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIncrementCcFastReadResourceMiss());
    FsRtlIncrementCcFastReadResourceMiss()
}
#[inline]
pub unsafe fn FsRtlIncrementCcFastReadWait() {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIncrementCcFastReadWait());
    FsRtlIncrementCcFastReadWait()
}
#[inline]
pub unsafe fn FsRtlInitExtraCreateParameterLookasideList(lookaside: *mut core::ffi::c_void, flags: u32, size: usize, tag: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitExtraCreateParameterLookasideList(lookaside : *mut core::ffi::c_void, flags : u32, size : usize, tag : u32));
    FsRtlInitExtraCreateParameterLookasideList(lookaside, flags, size, tag)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlInitializeBaseMcb(mcb: *mut BASE_MCB, pooltype: super::super::Foundation::POOL_TYPE) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeBaseMcb(mcb : *mut BASE_MCB, pooltype : super::super::Foundation:: POOL_TYPE));
    FsRtlInitializeBaseMcb(mcb, pooltype)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlInitializeBaseMcbEx(mcb: *mut BASE_MCB, pooltype: super::super::Foundation::POOL_TYPE, flags: u16) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeBaseMcbEx(mcb : *mut BASE_MCB, pooltype : super::super::Foundation:: POOL_TYPE, flags : u16) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlInitializeBaseMcbEx(mcb, pooltype, flags)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlInitializeExtraCreateParameter(ecp: *mut super::super::Foundation::ECP_HEADER, ecpflags: u32, cleanupcallback: PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, totalsize: u32, ecptype: *const windows_core::GUID, listallocatedfrom: Option<*const core::ffi::c_void>) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeExtraCreateParameter(ecp : *mut super::super::Foundation:: ECP_HEADER, ecpflags : u32, cleanupcallback : PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, totalsize : u32, ecptype : *const windows_core::GUID, listallocatedfrom : *const core::ffi::c_void));
    FsRtlInitializeExtraCreateParameter(ecp, ecpflags, cleanupcallback, totalsize, ecptype, core::mem::transmute(listallocatedfrom.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlInitializeExtraCreateParameterList(ecplist: *mut super::super::Foundation::ECP_LIST) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeExtraCreateParameterList(ecplist : *mut super::super::Foundation:: ECP_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlInitializeExtraCreateParameterList(ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlInitializeFileLock(filelock: *mut FILE_LOCK, completelockirproutine: PCOMPLETE_LOCK_IRP_ROUTINE, unlockroutine: PUNLOCK_ROUTINE) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeFileLock(filelock : *mut FILE_LOCK, completelockirproutine : PCOMPLETE_LOCK_IRP_ROUTINE, unlockroutine : PUNLOCK_ROUTINE));
    FsRtlInitializeFileLock(filelock, completelockirproutine, unlockroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlInitializeLargeMcb(mcb: *mut LARGE_MCB, pooltype: super::super::Foundation::POOL_TYPE) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeLargeMcb(mcb : *mut LARGE_MCB, pooltype : super::super::Foundation:: POOL_TYPE));
    FsRtlInitializeLargeMcb(mcb, pooltype)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlInitializeMcb(mcb: *mut MCB, pooltype: super::super::Foundation::POOL_TYPE) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeMcb(mcb : *mut MCB, pooltype : super::super::Foundation:: POOL_TYPE));
    FsRtlInitializeMcb(mcb, pooltype)
}
#[inline]
pub unsafe fn FsRtlInitializeOplock(oplock: *mut *mut core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeOplock(oplock : *mut *mut core::ffi::c_void));
    FsRtlInitializeOplock(oplock)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlInitializeTunnelCache(cache: *mut TUNNEL) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInitializeTunnelCache(cache : *mut TUNNEL));
    FsRtlInitializeTunnelCache(cache)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlInsertExtraCreateParameter(ecplist: *mut super::super::Foundation::ECP_LIST, ecpcontext: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInsertExtraCreateParameter(ecplist : *mut super::super::Foundation:: ECP_LIST, ecpcontext : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlInsertExtraCreateParameter(ecplist, ecpcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlInsertPerFileContext(perfilecontextpointer: *const *const core::ffi::c_void, ptr: *const FSRTL_PER_FILE_CONTEXT) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInsertPerFileContext(perfilecontextpointer : *const *const core::ffi::c_void, ptr : *const FSRTL_PER_FILE_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlInsertPerFileContext(perfilecontextpointer, ptr)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlInsertPerFileObjectContext(fileobject: *const super::super::Foundation::FILE_OBJECT, ptr: *const FSRTL_PER_FILEOBJECT_CONTEXT) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInsertPerFileObjectContext(fileobject : *const super::super::Foundation:: FILE_OBJECT, ptr : *const FSRTL_PER_FILEOBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlInsertPerFileObjectContext(fileobject, ptr)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlInsertPerStreamContext(perstreamcontext: *const FSRTL_ADVANCED_FCB_HEADER, ptr: *const FSRTL_PER_STREAM_CONTEXT) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlInsertPerStreamContext(perstreamcontext : *const FSRTL_ADVANCED_FCB_HEADER, ptr : *const FSRTL_PER_STREAM_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlInsertPerStreamContext(perstreamcontext, ptr)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlIs32BitProcess<P0>(process: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIs32BitProcess(process : super::super::Foundation:: PEPROCESS) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIs32BitProcess(process.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlIsDaxVolume(fileobject: *const super::super::Foundation::FILE_OBJECT) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsDaxVolume(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsDaxVolume(fileobject)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn FsRtlIsDbcsInExpression(expression: *const super::super::super::Win32::System::Kernel::STRING, name: *const super::super::super::Win32::System::Kernel::STRING) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsDbcsInExpression(expression : *const super::super::super::Win32::System::Kernel:: STRING, name : *const super::super::super::Win32::System::Kernel:: STRING) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsDbcsInExpression(expression, name)
}
#[inline]
pub unsafe fn FsRtlIsEcpAcknowledged(ecpcontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsEcpAcknowledged(ecpcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsEcpAcknowledged(ecpcontext)
}
#[inline]
pub unsafe fn FsRtlIsEcpFromUserMode(ecpcontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsEcpFromUserMode(ecpcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsEcpFromUserMode(ecpcontext)
}
#[inline]
pub unsafe fn FsRtlIsExtentDangling(startpage: u32, numberofpages: u32, flags: u32) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsExtentDangling(startpage : u32, numberofpages : u32, flags : u32) -> u32);
    FsRtlIsExtentDangling(startpage, numberofpages, flags)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn FsRtlIsFatDbcsLegal<P0, P1, P2>(dbcsname: super::super::super::Win32::System::Kernel::STRING, wildcardspermissible: P0, pathnamepermissible: P1, leadingbackslashpermissible: P2) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsFatDbcsLegal(dbcsname : super::super::super::Win32::System::Kernel:: STRING, wildcardspermissible : super::super::super::Win32::Foundation:: BOOLEAN, pathnamepermissible : super::super::super::Win32::Foundation:: BOOLEAN, leadingbackslashpermissible : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsFatDbcsLegal(core::mem::transmute(dbcsname), wildcardspermissible.param().abi(), pathnamepermissible.param().abi(), leadingbackslashpermissible.param().abi())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn FsRtlIsHpfsDbcsLegal<P0, P1, P2>(dbcsname: super::super::super::Win32::System::Kernel::STRING, wildcardspermissible: P0, pathnamepermissible: P1, leadingbackslashpermissible: P2) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsHpfsDbcsLegal(dbcsname : super::super::super::Win32::System::Kernel:: STRING, wildcardspermissible : super::super::super::Win32::Foundation:: BOOLEAN, pathnamepermissible : super::super::super::Win32::Foundation:: BOOLEAN, leadingbackslashpermissible : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsHpfsDbcsLegal(core::mem::transmute(dbcsname), wildcardspermissible.param().abi(), pathnamepermissible.param().abi(), leadingbackslashpermissible.param().abi())
}
#[inline]
pub unsafe fn FsRtlIsMobileOS() -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsMobileOS() -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsMobileOS()
}
#[inline]
pub unsafe fn FsRtlIsNameInExpression<P0, P1>(expression: *const super::super::super::Win32::Foundation::UNICODE_STRING, name: *const super::super::super::Win32::Foundation::UNICODE_STRING, ignorecase: P0, upcasetable: P1) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsNameInExpression(expression : *const super::super::super::Win32::Foundation:: UNICODE_STRING, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ignorecase : super::super::super::Win32::Foundation:: BOOLEAN, upcasetable : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsNameInExpression(expression, name, ignorecase.param().abi(), upcasetable.param().abi())
}
#[inline]
pub unsafe fn FsRtlIsNameInUnUpcasedExpression<P0, P1>(expression: *const super::super::super::Win32::Foundation::UNICODE_STRING, name: *const super::super::super::Win32::Foundation::UNICODE_STRING, ignorecase: P0, upcasetable: P1) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsNameInUnUpcasedExpression(expression : *const super::super::super::Win32::Foundation:: UNICODE_STRING, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ignorecase : super::super::super::Win32::Foundation:: BOOLEAN, upcasetable : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsNameInUnUpcasedExpression(expression, name, ignorecase.param().abi(), upcasetable.param().abi())
}
#[inline]
pub unsafe fn FsRtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag)
}
#[inline]
pub unsafe fn FsRtlIsNtstatusExpected<P0>(exception: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsNtstatusExpected(exception : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlIsNtstatusExpected(exception.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlIsPagingFile(fileobject: *const super::super::Foundation::FILE_OBJECT) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsPagingFile(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> u32);
    FsRtlIsPagingFile(fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlIsSystemPagingFile(fileobject: *const super::super::Foundation::FILE_OBJECT) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsSystemPagingFile(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> u32);
    FsRtlIsSystemPagingFile(fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlIssueDeviceIoControl(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, ioctl: u32, flags: u8, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*const core::ffi::c_void>, outputbufferlength: u32, iosbinformation: Option<*mut usize>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIssueDeviceIoControl(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, ioctl : u32, flags : u8, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *const core::ffi::c_void, outputbufferlength : u32, iosbinformation : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlIssueDeviceIoControl(deviceobject, ioctl, flags, core::mem::transmute(inputbuffer.unwrap_or(std::ptr::null())), inputbufferlength, core::mem::transmute(outputbuffer.unwrap_or(std::ptr::null())), outputbufferlength, core::mem::transmute(iosbinformation.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlKernelFsControlFile(fileobject: *const super::super::Foundation::FILE_OBJECT, fscontrolcode: u32, inputbuffer: *const core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut core::ffi::c_void, outputbufferlength: u32, retoutputbuffersize: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlKernelFsControlFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32, retoutputbuffersize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlKernelFsControlFile(fileobject, fscontrolcode, inputbuffer, inputbufferlength, outputbuffer, outputbufferlength, retoutputbuffersize)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlLogCcFlushError<P0>(filename: *const super::super::super::Win32::Foundation::UNICODE_STRING, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, flusherror: P0, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLogCcFlushError(filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, flusherror : super::super::super::Win32::Foundation:: NTSTATUS, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlLogCcFlushError(filename, deviceobject, sectionobjectpointer, flusherror.param().abi(), flags)
}
#[inline]
pub unsafe fn FsRtlLookupBaseMcbEntry(mcb: *const BASE_MCB, vbn: i64, lbn: Option<*mut i64>, sectorcountfromlbn: Option<*mut i64>, startinglbn: Option<*mut i64>, sectorcountfromstartinglbn: Option<*mut i64>, index: Option<*mut u32>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupBaseMcbEntry(mcb : *const BASE_MCB, vbn : i64, lbn : *mut i64, sectorcountfromlbn : *mut i64, startinglbn : *mut i64, sectorcountfromstartinglbn : *mut i64, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlLookupBaseMcbEntry(mcb, vbn, core::mem::transmute(lbn.unwrap_or(std::ptr::null_mut())), core::mem::transmute(sectorcountfromlbn.unwrap_or(std::ptr::null_mut())), core::mem::transmute(startinglbn.unwrap_or(std::ptr::null_mut())), core::mem::transmute(sectorcountfromstartinglbn.unwrap_or(std::ptr::null_mut())), core::mem::transmute(index.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlLookupLargeMcbEntry(mcb: *const LARGE_MCB, vbn: i64, lbn: Option<*mut i64>, sectorcountfromlbn: Option<*mut i64>, startinglbn: Option<*mut i64>, sectorcountfromstartinglbn: Option<*mut i64>, index: Option<*mut u32>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupLargeMcbEntry(mcb : *const LARGE_MCB, vbn : i64, lbn : *mut i64, sectorcountfromlbn : *mut i64, startinglbn : *mut i64, sectorcountfromstartinglbn : *mut i64, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlLookupLargeMcbEntry(mcb, vbn, core::mem::transmute(lbn.unwrap_or(std::ptr::null_mut())), core::mem::transmute(sectorcountfromlbn.unwrap_or(std::ptr::null_mut())), core::mem::transmute(startinglbn.unwrap_or(std::ptr::null_mut())), core::mem::transmute(sectorcountfromstartinglbn.unwrap_or(std::ptr::null_mut())), core::mem::transmute(index.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FsRtlLookupLastBaseMcbEntry(mcb: *const BASE_MCB, vbn: *mut i64, lbn: *mut i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupLastBaseMcbEntry(mcb : *const BASE_MCB, vbn : *mut i64, lbn : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlLookupLastBaseMcbEntry(mcb, vbn, lbn)
}
#[inline]
pub unsafe fn FsRtlLookupLastBaseMcbEntryAndIndex(opaquemcb: *const BASE_MCB, largevbn: *mut i64, largelbn: *mut i64, index: *mut u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupLastBaseMcbEntryAndIndex(opaquemcb : *const BASE_MCB, largevbn : *mut i64, largelbn : *mut i64, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlLookupLastBaseMcbEntryAndIndex(opaquemcb, largevbn, largelbn, index)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlLookupLastLargeMcbEntry(mcb: *const LARGE_MCB, vbn: *mut i64, lbn: *mut i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupLastLargeMcbEntry(mcb : *const LARGE_MCB, vbn : *mut i64, lbn : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlLookupLastLargeMcbEntry(mcb, vbn, lbn)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlLookupLastLargeMcbEntryAndIndex(opaquemcb: *const LARGE_MCB, largevbn: *mut i64, largelbn: *mut i64, index: *mut u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupLastLargeMcbEntryAndIndex(opaquemcb : *const LARGE_MCB, largevbn : *mut i64, largelbn : *mut i64, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlLookupLastLargeMcbEntryAndIndex(opaquemcb, largevbn, largelbn, index)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlLookupLastMcbEntry(mcb: *const MCB, vbn: *mut u32, lbn: *mut u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupLastMcbEntry(mcb : *const MCB, vbn : *mut u32, lbn : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlLookupLastMcbEntry(mcb, vbn, lbn)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlLookupMcbEntry(mcb: *const MCB, vbn: u32, lbn: *mut u32, sectorcount: Option<*mut u32>, index: *mut u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupMcbEntry(mcb : *const MCB, vbn : u32, lbn : *mut u32, sectorcount : *mut u32, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlLookupMcbEntry(mcb, vbn, lbn, core::mem::transmute(sectorcount.unwrap_or(std::ptr::null_mut())), index)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlLookupPerFileContext(perfilecontextpointer: *const *const core::ffi::c_void, ownerid: Option<*const core::ffi::c_void>, instanceid: Option<*const core::ffi::c_void>) -> *mut FSRTL_PER_FILE_CONTEXT {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupPerFileContext(perfilecontextpointer : *const *const core::ffi::c_void, ownerid : *const core::ffi::c_void, instanceid : *const core::ffi::c_void) -> *mut FSRTL_PER_FILE_CONTEXT);
    FsRtlLookupPerFileContext(perfilecontextpointer, core::mem::transmute(ownerid.unwrap_or(std::ptr::null())), core::mem::transmute(instanceid.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlLookupPerFileObjectContext(fileobject: *const super::super::Foundation::FILE_OBJECT, ownerid: Option<*const core::ffi::c_void>, instanceid: Option<*const core::ffi::c_void>) -> *mut FSRTL_PER_FILEOBJECT_CONTEXT {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupPerFileObjectContext(fileobject : *const super::super::Foundation:: FILE_OBJECT, ownerid : *const core::ffi::c_void, instanceid : *const core::ffi::c_void) -> *mut FSRTL_PER_FILEOBJECT_CONTEXT);
    FsRtlLookupPerFileObjectContext(fileobject, core::mem::transmute(ownerid.unwrap_or(std::ptr::null())), core::mem::transmute(instanceid.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlLookupPerStreamContextInternal(streamcontext: *const FSRTL_ADVANCED_FCB_HEADER, ownerid: Option<*const core::ffi::c_void>, instanceid: Option<*const core::ffi::c_void>) -> *mut FSRTL_PER_STREAM_CONTEXT {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlLookupPerStreamContextInternal(streamcontext : *const FSRTL_ADVANCED_FCB_HEADER, ownerid : *const core::ffi::c_void, instanceid : *const core::ffi::c_void) -> *mut FSRTL_PER_STREAM_CONTEXT);
    FsRtlLookupPerStreamContextInternal(streamcontext, core::mem::transmute(ownerid.unwrap_or(std::ptr::null())), core::mem::transmute(instanceid.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlMdlReadCompleteDev(fileobject: *const super::super::Foundation::FILE_OBJECT, mdlchain: *const super::super::Foundation::MDL, deviceobject: Option<*const super::super::Foundation::DEVICE_OBJECT>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlMdlReadCompleteDev(fileobject : *const super::super::Foundation:: FILE_OBJECT, mdlchain : *const super::super::Foundation:: MDL, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlMdlReadCompleteDev(fileobject, mdlchain, core::mem::transmute(deviceobject.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlMdlReadDev(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut super::super::Foundation::MDL, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: Option<*const super::super::Foundation::DEVICE_OBJECT>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlMdlReadDev(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::Foundation:: MDL, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlMdlReadDev(fileobject, fileoffset, length, lockkey, mdlchain, iostatus, core::mem::transmute(deviceobject.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlMdlReadEx(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut super::super::Foundation::MDL, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlMdlReadEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::Foundation:: MDL, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlMdlReadEx(fileobject, fileoffset, length, lockkey, mdlchain, iostatus)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlMdlWriteCompleteDev(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, mdlchain: *const super::super::Foundation::MDL, deviceobject: Option<*const super::super::Foundation::DEVICE_OBJECT>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlMdlWriteCompleteDev(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, mdlchain : *const super::super::Foundation:: MDL, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlMdlWriteCompleteDev(fileobject, fileoffset, mdlchain, core::mem::transmute(deviceobject.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn FsRtlMupGetProviderIdFromName(pprovidername: *const super::super::super::Win32::Foundation::UNICODE_STRING, pproviderid: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlMupGetProviderIdFromName(pprovidername : *const super::super::super::Win32::Foundation:: UNICODE_STRING, pproviderid : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlMupGetProviderIdFromName(pprovidername, pproviderid)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlMupGetProviderInfoFromFileObject(pfileobject: *const super::super::Foundation::FILE_OBJECT, level: u32, pbuffer: *mut core::ffi::c_void, pbuffersize: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlMupGetProviderInfoFromFileObject(pfileobject : *const super::super::Foundation:: FILE_OBJECT, level : u32, pbuffer : *mut core::ffi::c_void, pbuffersize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlMupGetProviderInfoFromFileObject(pfileobject, level, pbuffer, pbuffersize)
}
#[inline]
pub unsafe fn FsRtlNormalizeNtstatus<P0, P1>(exception: P0, genericexception: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNormalizeNtstatus(exception : super::super::super::Win32::Foundation:: NTSTATUS, genericexception : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlNormalizeNtstatus(exception.param().abi(), genericexception.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlNotifyCleanup<P0>(notifysync: P0, notifylist: *const super::super::super::Win32::System::Kernel::LIST_ENTRY, fscontext: *const core::ffi::c_void)
where
    P0: windows_core::Param<super::super::Foundation::PNOTIFY_SYNC>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyCleanup(notifysync : super::super::Foundation:: PNOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fscontext : *const core::ffi::c_void));
    FsRtlNotifyCleanup(notifysync.param().abi(), notifylist, fscontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlNotifyCleanupAll<P0>(notifysync: P0, notifylist: *const super::super::super::Win32::System::Kernel::LIST_ENTRY)
where
    P0: windows_core::Param<super::super::Foundation::PNOTIFY_SYNC>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyCleanupAll(notifysync : super::super::Foundation:: PNOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY));
    FsRtlNotifyCleanupAll(notifysync.param().abi(), notifylist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlNotifyFilterChangeDirectory<P0, P1, P2>(notifysync: P0, notifylist: *const super::super::super::Win32::System::Kernel::LIST_ENTRY, fscontext: *const core::ffi::c_void, fulldirectoryname: *const super::super::super::Win32::System::Kernel::STRING, watchtree: P1, ignorebuffer: P2, completionfilter: u32, notifyirp: Option<*const super::super::Foundation::IRP>, traversecallback: PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext: Option<*const super::super::Foundation::SECURITY_SUBJECT_CONTEXT>, filtercallback: PFILTER_REPORT_CHANGE)
where
    P0: windows_core::Param<super::super::Foundation::PNOTIFY_SYNC>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyFilterChangeDirectory(notifysync : super::super::Foundation:: PNOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fscontext : *const core::ffi::c_void, fulldirectoryname : *const super::super::super::Win32::System::Kernel:: STRING, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, ignorebuffer : super::super::super::Win32::Foundation:: BOOLEAN, completionfilter : u32, notifyirp : *const super::super::Foundation:: IRP, traversecallback : PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, filtercallback : PFILTER_REPORT_CHANGE));
    FsRtlNotifyFilterChangeDirectory(notifysync.param().abi(), notifylist, fscontext, fulldirectoryname, watchtree.param().abi(), ignorebuffer.param().abi(), completionfilter, core::mem::transmute(notifyirp.unwrap_or(std::ptr::null())), traversecallback, core::mem::transmute(subjectcontext.unwrap_or(std::ptr::null())), filtercallback)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlNotifyFilterReportChange<P0>(notifysync: P0, notifylist: *const super::super::super::Win32::System::Kernel::LIST_ENTRY, fulltargetname: *const super::super::super::Win32::System::Kernel::STRING, targetnameoffset: u16, streamname: Option<*const super::super::super::Win32::System::Kernel::STRING>, normalizedparentname: Option<*const super::super::super::Win32::System::Kernel::STRING>, filtermatch: u32, action: u32, targetcontext: Option<*const core::ffi::c_void>, filtercontext: Option<*const core::ffi::c_void>)
where
    P0: windows_core::Param<super::super::Foundation::PNOTIFY_SYNC>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyFilterReportChange(notifysync : super::super::Foundation:: PNOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fulltargetname : *const super::super::super::Win32::System::Kernel:: STRING, targetnameoffset : u16, streamname : *const super::super::super::Win32::System::Kernel:: STRING, normalizedparentname : *const super::super::super::Win32::System::Kernel:: STRING, filtermatch : u32, action : u32, targetcontext : *const core::ffi::c_void, filtercontext : *const core::ffi::c_void));
    FsRtlNotifyFilterReportChange(notifysync.param().abi(), notifylist, fulltargetname, targetnameoffset, core::mem::transmute(streamname.unwrap_or(std::ptr::null())), core::mem::transmute(normalizedparentname.unwrap_or(std::ptr::null())), filtermatch, action, core::mem::transmute(targetcontext.unwrap_or(std::ptr::null())), core::mem::transmute(filtercontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlNotifyFullChangeDirectory<P0, P1, P2>(notifysync: P0, notifylist: *const super::super::super::Win32::System::Kernel::LIST_ENTRY, fscontext: *const core::ffi::c_void, fulldirectoryname: *mut super::super::super::Win32::System::Kernel::STRING, watchtree: P1, ignorebuffer: P2, completionfilter: u32, notifyirp: Option<*const super::super::Foundation::IRP>, traversecallback: PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext: Option<*const super::super::Foundation::SECURITY_SUBJECT_CONTEXT>)
where
    P0: windows_core::Param<super::super::Foundation::PNOTIFY_SYNC>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyFullChangeDirectory(notifysync : super::super::Foundation:: PNOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fscontext : *const core::ffi::c_void, fulldirectoryname : *mut super::super::super::Win32::System::Kernel:: STRING, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, ignorebuffer : super::super::super::Win32::Foundation:: BOOLEAN, completionfilter : u32, notifyirp : *const super::super::Foundation:: IRP, traversecallback : PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT));
    FsRtlNotifyFullChangeDirectory(notifysync.param().abi(), notifylist, fscontext, fulldirectoryname, watchtree.param().abi(), ignorebuffer.param().abi(), completionfilter, core::mem::transmute(notifyirp.unwrap_or(std::ptr::null())), traversecallback, core::mem::transmute(subjectcontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlNotifyFullReportChange<P0>(notifysync: P0, notifylist: *const super::super::super::Win32::System::Kernel::LIST_ENTRY, fulltargetname: *const super::super::super::Win32::System::Kernel::STRING, targetnameoffset: u16, streamname: Option<*const super::super::super::Win32::System::Kernel::STRING>, normalizedparentname: Option<*const super::super::super::Win32::System::Kernel::STRING>, filtermatch: u32, action: u32, targetcontext: Option<*const core::ffi::c_void>)
where
    P0: windows_core::Param<super::super::Foundation::PNOTIFY_SYNC>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyFullReportChange(notifysync : super::super::Foundation:: PNOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fulltargetname : *const super::super::super::Win32::System::Kernel:: STRING, targetnameoffset : u16, streamname : *const super::super::super::Win32::System::Kernel:: STRING, normalizedparentname : *const super::super::super::Win32::System::Kernel:: STRING, filtermatch : u32, action : u32, targetcontext : *const core::ffi::c_void));
    FsRtlNotifyFullReportChange(notifysync.param().abi(), notifylist, fulltargetname, targetnameoffset, core::mem::transmute(streamname.unwrap_or(std::ptr::null())), core::mem::transmute(normalizedparentname.unwrap_or(std::ptr::null())), filtermatch, action, core::mem::transmute(targetcontext.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlNotifyInitializeSync() -> super::super::Foundation::PNOTIFY_SYNC {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyInitializeSync(notifysync : *mut super::super::Foundation:: PNOTIFY_SYNC));
    let mut result__ = core::mem::zeroed();
    FsRtlNotifyInitializeSync(&mut result__);
    result__
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlNotifyUninitializeSync(notifysync: *mut super::super::Foundation::PNOTIFY_SYNC) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyUninitializeSync(notifysync : *mut super::super::Foundation:: PNOTIFY_SYNC));
    FsRtlNotifyUninitializeSync(notifysync)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlNotifyVolumeEvent(fileobject: *const super::super::Foundation::FILE_OBJECT, eventcode: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyVolumeEvent(fileobject : *const super::super::Foundation:: FILE_OBJECT, eventcode : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlNotifyVolumeEvent(fileobject, eventcode)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlNotifyVolumeEventEx(fileobject: *const super::super::Foundation::FILE_OBJECT, eventcode: u32, event: *const super::super::Foundation::TARGET_DEVICE_CUSTOM_NOTIFICATION) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNotifyVolumeEventEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, eventcode : u32, event : *const super::super::Foundation:: TARGET_DEVICE_CUSTOM_NOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlNotifyVolumeEventEx(fileobject, eventcode, event)
}
#[inline]
pub unsafe fn FsRtlNumberOfRunsInBaseMcb(mcb: *const BASE_MCB) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNumberOfRunsInBaseMcb(mcb : *const BASE_MCB) -> u32);
    FsRtlNumberOfRunsInBaseMcb(mcb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlNumberOfRunsInLargeMcb(mcb: *const LARGE_MCB) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNumberOfRunsInLargeMcb(mcb : *const LARGE_MCB) -> u32);
    FsRtlNumberOfRunsInLargeMcb(mcb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlNumberOfRunsInMcb(mcb: *const MCB) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlNumberOfRunsInMcb(mcb : *const MCB) -> u32);
    FsRtlNumberOfRunsInMcb(mcb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlOplockBreakH(oplock: *const *const core::ffi::c_void, irp: *const super::super::Foundation::IRP, flags: u32, context: Option<*const core::ffi::c_void>, completionroutine: POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine: POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockBreakH(oplock : *const *const core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, context : *const core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlOplockBreakH(oplock, irp, flags, core::mem::transmute(context.unwrap_or(std::ptr::null())), completionroutine, postirproutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlOplockBreakH2(oplock: *const *const core::ffi::c_void, irp: *const super::super::Foundation::IRP, flags: u32, context: Option<*const core::ffi::c_void>, completionroutine: POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine: POPLOCK_FS_PREPOST_IRP, grantedaccess: Option<*const u32>, shareaccess: Option<*const u16>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockBreakH2(oplock : *const *const core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, context : *const core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP, grantedaccess : *const u32, shareaccess : *const u16) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlOplockBreakH2(oplock, irp, flags, core::mem::transmute(context.unwrap_or(std::ptr::null())), completionroutine, postirproutine, core::mem::transmute(grantedaccess.unwrap_or(std::ptr::null())), core::mem::transmute(shareaccess.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlOplockBreakToNone(oplock: *mut *mut core::ffi::c_void, irpsp: Option<*const super::super::Foundation::IO_STACK_LOCATION>, irp: *const super::super::Foundation::IRP, context: Option<*const core::ffi::c_void>, completionroutine: POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine: POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockBreakToNone(oplock : *mut *mut core::ffi::c_void, irpsp : *const super::super::Foundation:: IO_STACK_LOCATION, irp : *const super::super::Foundation:: IRP, context : *const core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlOplockBreakToNone(oplock, core::mem::transmute(irpsp.unwrap_or(std::ptr::null())), irp, core::mem::transmute(context.unwrap_or(std::ptr::null())), completionroutine, postirproutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlOplockBreakToNoneEx(oplock: *mut *mut core::ffi::c_void, irp: *const super::super::Foundation::IRP, flags: u32, context: Option<*const core::ffi::c_void>, completionroutine: POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine: POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockBreakToNoneEx(oplock : *mut *mut core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, context : *const core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlOplockBreakToNoneEx(oplock, irp, flags, core::mem::transmute(context.unwrap_or(std::ptr::null())), completionroutine, postirproutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlOplockFsctrl(oplock: *const *const core::ffi::c_void, irp: *const super::super::Foundation::IRP, opencount: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockFsctrl(oplock : *const *const core::ffi::c_void, irp : *const super::super::Foundation:: IRP, opencount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlOplockFsctrl(oplock, irp, opencount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlOplockFsctrlEx(oplock: *const *const core::ffi::c_void, irp: *const super::super::Foundation::IRP, opencount: u32, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockFsctrlEx(oplock : *const *const core::ffi::c_void, irp : *const super::super::Foundation:: IRP, opencount : u32, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlOplockFsctrlEx(oplock, irp, opencount, flags)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlOplockGetAnyBreakOwnerProcess(oplock: *const *const core::ffi::c_void) -> super::super::Foundation::PEPROCESS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockGetAnyBreakOwnerProcess(oplock : *const *const core::ffi::c_void) -> super::super::Foundation:: PEPROCESS);
    FsRtlOplockGetAnyBreakOwnerProcess(oplock)
}
#[inline]
pub unsafe fn FsRtlOplockIsFastIoPossible(oplock: *const *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockIsFastIoPossible(oplock : *const *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlOplockIsFastIoPossible(oplock)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlOplockIsSharedRequest(irp: *const super::super::Foundation::IRP) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockIsSharedRequest(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlOplockIsSharedRequest(irp)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlOplockKeysEqual(fo1: Option<*const super::super::Foundation::FILE_OBJECT>, fo2: Option<*const super::super::Foundation::FILE_OBJECT>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlOplockKeysEqual(fo1 : *const super::super::Foundation:: FILE_OBJECT, fo2 : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlOplockKeysEqual(core::mem::transmute(fo1.unwrap_or(std::ptr::null())), core::mem::transmute(fo2.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlPostPagingFileStackOverflow(context: *const core::ffi::c_void, event: *const super::super::Foundation::KEVENT, stackoverflowroutine: PFSRTL_STACK_OVERFLOW_ROUTINE) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlPostPagingFileStackOverflow(context : *const core::ffi::c_void, event : *const super::super::Foundation:: KEVENT, stackoverflowroutine : PFSRTL_STACK_OVERFLOW_ROUTINE));
    FsRtlPostPagingFileStackOverflow(context, event, stackoverflowroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlPostStackOverflow(context: *const core::ffi::c_void, event: *const super::super::Foundation::KEVENT, stackoverflowroutine: PFSRTL_STACK_OVERFLOW_ROUTINE) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlPostStackOverflow(context : *const core::ffi::c_void, event : *const super::super::Foundation:: KEVENT, stackoverflowroutine : PFSRTL_STACK_OVERFLOW_ROUTINE));
    FsRtlPostStackOverflow(context, event, stackoverflowroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlPrepareMdlWriteDev(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut super::super::Foundation::MDL, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const super::super::Foundation::DEVICE_OBJECT) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlPrepareMdlWriteDev(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::Foundation:: MDL, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlPrepareMdlWriteDev(fileobject, fileoffset, length, lockkey, mdlchain, iostatus, deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlPrepareMdlWriteEx(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut super::super::Foundation::MDL, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlPrepareMdlWriteEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::Foundation:: MDL, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlPrepareMdlWriteEx(fileobject, fileoffset, length, lockkey, mdlchain, iostatus)
}
#[inline]
pub unsafe fn FsRtlPrepareToReuseEcp(ecpcontext: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlPrepareToReuseEcp(ecpcontext : *const core::ffi::c_void));
    FsRtlPrepareToReuseEcp(ecpcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlPrivateLock<P0, P1, P2, P3>(filelock: *const FILE_LOCK, fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: *const i64, processid: P0, key: u32, failimmediately: P1, exclusivelock: P2, iosb: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, irp: Option<*const super::super::Foundation::IRP>, context: Option<*const core::ffi::c_void>, alreadysynchronized: P3) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlPrivateLock(filelock : *const FILE_LOCK, fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : *const i64, processid : super::super::Foundation:: PEPROCESS, key : u32, failimmediately : super::super::super::Win32::Foundation:: BOOLEAN, exclusivelock : super::super::super::Win32::Foundation:: BOOLEAN, iosb : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, irp : *const super::super::Foundation:: IRP, context : *const core::ffi::c_void, alreadysynchronized : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlPrivateLock(filelock, fileobject, fileoffset, length, processid.param().abi(), key, failimmediately.param().abi(), exclusivelock.param().abi(), iosb, core::mem::transmute(irp.unwrap_or(std::ptr::null())), core::mem::transmute(context.unwrap_or(std::ptr::null())), alreadysynchronized.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlProcessFileLock(filelock: *const FILE_LOCK, irp: *const super::super::Foundation::IRP, context: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlProcessFileLock(filelock : *const FILE_LOCK, irp : *const super::super::Foundation:: IRP, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlProcessFileLock(filelock, irp, core::mem::transmute(context.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlQueryCachedVdl(fileobject: *const super::super::Foundation::FILE_OBJECT, vdl: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlQueryCachedVdl(fileobject : *const super::super::Foundation:: FILE_OBJECT, vdl : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlQueryCachedVdl(fileobject, vdl)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlQueryInformationFile(fileobject: *const super::super::Foundation::FILE_OBJECT, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: FILE_INFORMATION_CLASS, retfileinformationsize: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlQueryInformationFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS, retfileinformationsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlQueryInformationFile(fileobject, fileinformation, length, fileinformationclass, retfileinformationsize)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlQueryKernelEaFile<P0, P1>(fileobject: *const super::super::Foundation::FILE_OBJECT, returnedeadata: *mut core::ffi::c_void, length: u32, returnsingleentry: P0, ealist: Option<*const core::ffi::c_void>, ealistlength: u32, eaindex: Option<*const u32>, restartscan: P1, lengthreturned: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlQueryKernelEaFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, returnedeadata : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, ealist : *const core::ffi::c_void, ealistlength : u32, eaindex : *const u32, restartscan : super::super::super::Win32::Foundation:: BOOLEAN, lengthreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlQueryKernelEaFile(fileobject, returnedeadata, length, returnsingleentry.param().abi(), core::mem::transmute(ealist.unwrap_or(std::ptr::null())), ealistlength, core::mem::transmute(eaindex.unwrap_or(std::ptr::null())), restartscan.param().abi(), core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FsRtlQueryMaximumVirtualDiskNestingLevel() -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlQueryMaximumVirtualDiskNestingLevel() -> u32);
    FsRtlQueryMaximumVirtualDiskNestingLevel()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlRegisterFileSystemFilterCallbacks(filterdriverobject: *const super::super::Foundation::DRIVER_OBJECT, callbacks: *const FS_FILTER_CALLBACKS) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRegisterFileSystemFilterCallbacks(filterdriverobject : *const super::super::Foundation:: DRIVER_OBJECT, callbacks : *const FS_FILTER_CALLBACKS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlRegisterFileSystemFilterCallbacks(filterdriverobject, callbacks)
}
#[inline]
pub unsafe fn FsRtlRegisterUncProvider<P0>(muphandle: *mut super::super::super::Win32::Foundation::HANDLE, redirectordevicename: *const super::super::super::Win32::Foundation::UNICODE_STRING, mailslotssupported: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRegisterUncProvider(muphandle : *mut super::super::super::Win32::Foundation:: HANDLE, redirectordevicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, mailslotssupported : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlRegisterUncProvider(muphandle, redirectordevicename, mailslotssupported.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlRegisterUncProviderEx(muphandle: *mut super::super::super::Win32::Foundation::HANDLE, redirdevname: *const super::super::super::Win32::Foundation::UNICODE_STRING, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRegisterUncProviderEx(muphandle : *mut super::super::super::Win32::Foundation:: HANDLE, redirdevname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlRegisterUncProviderEx(muphandle, redirdevname, deviceobject, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlRegisterUncProviderEx2(redirdevname: *const super::super::super::Win32::Foundation::UNICODE_STRING, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, registration: *const FSRTL_UNC_PROVIDER_REGISTRATION, muphandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRegisterUncProviderEx2(redirdevname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, registration : *const FSRTL_UNC_PROVIDER_REGISTRATION, muphandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlRegisterUncProviderEx2(redirdevname, deviceobject, registration, muphandle)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlReleaseFile(fileobject: *const super::super::Foundation::FILE_OBJECT) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlReleaseFile(fileobject : *const super::super::Foundation:: FILE_OBJECT));
    FsRtlReleaseFile(fileobject)
}
#[inline]
pub unsafe fn FsRtlRemoveBaseMcbEntry(mcb: *mut BASE_MCB, vbn: i64, sectorcount: i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRemoveBaseMcbEntry(mcb : *mut BASE_MCB, vbn : i64, sectorcount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlRemoveBaseMcbEntry(mcb, vbn, sectorcount)
}
#[inline]
pub unsafe fn FsRtlRemoveDotsFromPath(originalstring: windows_core::PWSTR, pathlength: u16, newlength: *mut u16) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRemoveDotsFromPath(originalstring : windows_core::PWSTR, pathlength : u16, newlength : *mut u16) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlRemoveDotsFromPath(core::mem::transmute(originalstring), pathlength, newlength)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FsRtlRemoveExtraCreateParameter(ecplist: *mut super::super::Foundation::ECP_LIST, ecptype: *const windows_core::GUID, ecpcontext: *mut *mut core::ffi::c_void, ecpcontextsize: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRemoveExtraCreateParameter(ecplist : *mut super::super::Foundation:: ECP_LIST, ecptype : *const windows_core::GUID, ecpcontext : *mut *mut core::ffi::c_void, ecpcontextsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlRemoveExtraCreateParameter(ecplist, ecptype, ecpcontext, core::mem::transmute(ecpcontextsize.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlRemoveLargeMcbEntry(mcb: *mut LARGE_MCB, vbn: i64, sectorcount: i64) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRemoveLargeMcbEntry(mcb : *mut LARGE_MCB, vbn : i64, sectorcount : i64));
    FsRtlRemoveLargeMcbEntry(mcb, vbn, sectorcount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlRemoveMcbEntry(mcb: *mut MCB, vbn: u32, sectorcount: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRemoveMcbEntry(mcb : *mut MCB, vbn : u32, sectorcount : u32));
    FsRtlRemoveMcbEntry(mcb, vbn, sectorcount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlRemovePerFileContext(perfilecontextpointer: *const *const core::ffi::c_void, ownerid: Option<*const core::ffi::c_void>, instanceid: Option<*const core::ffi::c_void>) -> *mut FSRTL_PER_FILE_CONTEXT {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRemovePerFileContext(perfilecontextpointer : *const *const core::ffi::c_void, ownerid : *const core::ffi::c_void, instanceid : *const core::ffi::c_void) -> *mut FSRTL_PER_FILE_CONTEXT);
    FsRtlRemovePerFileContext(perfilecontextpointer, core::mem::transmute(ownerid.unwrap_or(std::ptr::null())), core::mem::transmute(instanceid.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlRemovePerFileObjectContext(fileobject: *const super::super::Foundation::FILE_OBJECT, ownerid: Option<*const core::ffi::c_void>, instanceid: Option<*const core::ffi::c_void>) -> *mut FSRTL_PER_FILEOBJECT_CONTEXT {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRemovePerFileObjectContext(fileobject : *const super::super::Foundation:: FILE_OBJECT, ownerid : *const core::ffi::c_void, instanceid : *const core::ffi::c_void) -> *mut FSRTL_PER_FILEOBJECT_CONTEXT);
    FsRtlRemovePerFileObjectContext(fileobject, core::mem::transmute(ownerid.unwrap_or(std::ptr::null())), core::mem::transmute(instanceid.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlRemovePerStreamContext(streamcontext: *const FSRTL_ADVANCED_FCB_HEADER, ownerid: Option<*const core::ffi::c_void>, instanceid: Option<*const core::ffi::c_void>) -> *mut FSRTL_PER_STREAM_CONTEXT {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlRemovePerStreamContext(streamcontext : *const FSRTL_ADVANCED_FCB_HEADER, ownerid : *const core::ffi::c_void, instanceid : *const core::ffi::c_void) -> *mut FSRTL_PER_STREAM_CONTEXT);
    FsRtlRemovePerStreamContext(streamcontext, core::mem::transmute(ownerid.unwrap_or(std::ptr::null())), core::mem::transmute(instanceid.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn FsRtlResetBaseMcb() -> BASE_MCB {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlResetBaseMcb(mcb : *mut BASE_MCB));
    let mut result__ = core::mem::zeroed();
    FsRtlResetBaseMcb(&mut result__);
    result__
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlResetLargeMcb<P0>(mcb: *mut LARGE_MCB, selfsynchronized: P0)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlResetLargeMcb(mcb : *mut LARGE_MCB, selfsynchronized : super::super::super::Win32::Foundation:: BOOLEAN));
    FsRtlResetLargeMcb(mcb, selfsynchronized.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlSetDriverBacking(driverobj: *const super::super::Foundation::DRIVER_OBJECT, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlSetDriverBacking(driverobj : *const super::super::Foundation:: DRIVER_OBJECT, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlSetDriverBacking(driverobj, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlSetEcpListIntoIrp(irp: *mut super::super::Foundation::IRP, ecplist: *const super::super::Foundation::ECP_LIST) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlSetEcpListIntoIrp(irp : *mut super::super::Foundation:: IRP, ecplist : *const super::super::Foundation:: ECP_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlSetEcpListIntoIrp(irp, ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlSetKernelEaFile(fileobject: *const super::super::Foundation::FILE_OBJECT, eabuffer: *const core::ffi::c_void, length: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlSetKernelEaFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, eabuffer : *const core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlSetKernelEaFile(fileobject, eabuffer, length)
}
#[inline]
pub unsafe fn FsRtlSplitBaseMcb(mcb: *mut BASE_MCB, vbn: i64, amount: i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlSplitBaseMcb(mcb : *mut BASE_MCB, vbn : i64, amount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlSplitBaseMcb(mcb, vbn, amount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlSplitLargeMcb(mcb: *mut LARGE_MCB, vbn: i64, amount: i64) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlSplitLargeMcb(mcb : *mut LARGE_MCB, vbn : i64, amount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    FsRtlSplitLargeMcb(mcb, vbn, amount)
}
#[inline]
pub unsafe fn FsRtlTeardownPerFileContexts(perfilecontextpointer: *const *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlTeardownPerFileContexts(perfilecontextpointer : *const *const core::ffi::c_void));
    FsRtlTeardownPerFileContexts(perfilecontextpointer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlTeardownPerStreamContexts(advancedheader: *const FSRTL_ADVANCED_FCB_HEADER) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlTeardownPerStreamContexts(advancedheader : *const FSRTL_ADVANCED_FCB_HEADER));
    FsRtlTeardownPerStreamContexts(advancedheader)
}
#[inline]
pub unsafe fn FsRtlTruncateBaseMcb(mcb: *mut BASE_MCB, vbn: i64) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlTruncateBaseMcb(mcb : *mut BASE_MCB, vbn : i64));
    FsRtlTruncateBaseMcb(mcb, vbn)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlTruncateLargeMcb(mcb: *mut LARGE_MCB, vbn: i64) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlTruncateLargeMcb(mcb : *mut LARGE_MCB, vbn : i64));
    FsRtlTruncateLargeMcb(mcb, vbn)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlTruncateMcb(mcb: *mut MCB, vbn: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlTruncateMcb(mcb : *mut MCB, vbn : u32));
    FsRtlTruncateMcb(mcb, vbn)
}
#[inline]
pub unsafe fn FsRtlUninitializeBaseMcb(mcb: *const BASE_MCB) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlUninitializeBaseMcb(mcb : *const BASE_MCB));
    FsRtlUninitializeBaseMcb(mcb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlUninitializeFileLock(filelock: *mut FILE_LOCK) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlUninitializeFileLock(filelock : *mut FILE_LOCK));
    FsRtlUninitializeFileLock(filelock)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlUninitializeLargeMcb(mcb: *mut LARGE_MCB) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlUninitializeLargeMcb(mcb : *mut LARGE_MCB));
    FsRtlUninitializeLargeMcb(mcb)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FsRtlUninitializeMcb(mcb: *mut MCB) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlUninitializeMcb(mcb : *mut MCB));
    FsRtlUninitializeMcb(mcb)
}
#[inline]
pub unsafe fn FsRtlUninitializeOplock(oplock: *mut *mut core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlUninitializeOplock(oplock : *mut *mut core::ffi::c_void));
    FsRtlUninitializeOplock(oplock)
}
#[inline]
pub unsafe fn FsRtlUpdateDiskCounters(bytesread: u64, byteswritten: u64) {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlUpdateDiskCounters(bytesread : u64, byteswritten : u64));
    FsRtlUpdateDiskCounters(bytesread, byteswritten)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlUpperOplockFsctrl(oplock: *const *const core::ffi::c_void, irp: *const super::super::Foundation::IRP, opencount: u32, loweroplockstate: u32, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlUpperOplockFsctrl(oplock : *const *const core::ffi::c_void, irp : *const super::super::Foundation:: IRP, opencount : u32, loweroplockstate : u32, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlUpperOplockFsctrl(oplock, irp, opencount, loweroplockstate, flags)
}
#[inline]
pub unsafe fn FsRtlValidateReparsePointBuffer(bufferlength: u32, reparsebuffer: *const REPARSE_DATA_BUFFER) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlValidateReparsePointBuffer(bufferlength : u32, reparsebuffer : *const REPARSE_DATA_BUFFER) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlValidateReparsePointBuffer(bufferlength, reparsebuffer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FsRtlVolumeDeviceToCorrelationId(volumedeviceobject: *const super::super::Foundation::DEVICE_OBJECT, guid: *mut windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlVolumeDeviceToCorrelationId(volumedeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, guid : *mut windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FsRtlVolumeDeviceToCorrelationId(volumedeviceobject, guid)
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[inline]
pub unsafe fn GetSecurityUserInfo(logonid: Option<*const super::super::super::Win32::Foundation::LUID>, flags: u32, userinformation: *mut *mut super::super::super::Win32::Security::Authentication::Identity::SECURITY_USER_DATA) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("secur32.dll" "system" fn GetSecurityUserInfo(logonid : *const super::super::super::Win32::Foundation:: LUID, flags : u32, userinformation : *mut *mut super::super::super::Win32::Security::Authentication::Identity:: SECURITY_USER_DATA) -> super::super::super::Win32::Foundation:: NTSTATUS);
    GetSecurityUserInfo(core::mem::transmute(logonid.unwrap_or(std::ptr::null())), flags, userinformation)
}
#[inline]
pub unsafe fn IoAcquireVpbSpinLock() -> u8 {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoAcquireVpbSpinLock(irql : *mut u8));
    let mut result__ = core::mem::zeroed();
    IoAcquireVpbSpinLock(&mut result__);
    result__
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn IoApplyPriorityInfoThread<P0>(inputpriorityinfo: *const IO_PRIORITY_INFO, outputpriorityinfo: Option<*mut IO_PRIORITY_INFO>, thread: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoApplyPriorityInfoThread(inputpriorityinfo : *const IO_PRIORITY_INFO, outputpriorityinfo : *mut IO_PRIORITY_INFO, thread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoApplyPriorityInfoThread(inputpriorityinfo, core::mem::transmute(outputpriorityinfo.unwrap_or(std::ptr::null_mut())), thread.param().abi())
}
#[inline]
pub unsafe fn IoCheckDesiredAccess(desiredaccess: *mut u32, grantedaccess: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckDesiredAccess(desiredaccess : *mut u32, grantedaccess : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoCheckDesiredAccess(desiredaccess, grantedaccess)
}
#[inline]
pub unsafe fn IoCheckEaBufferValidity(eabuffer: *const FILE_FULL_EA_INFORMATION, ealength: u32, erroroffset: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckEaBufferValidity(eabuffer : *const FILE_FULL_EA_INFORMATION, ealength : u32, erroroffset : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoCheckEaBufferValidity(eabuffer, ealength, erroroffset)
}
#[inline]
pub unsafe fn IoCheckFunctionAccess(grantedaccess: u32, majorfunction: u8, minorfunction: u8, iocontrolcode: u32, arg1: Option<*const core::ffi::c_void>, arg2: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckFunctionAccess(grantedaccess : u32, majorfunction : u8, minorfunction : u8, iocontrolcode : u32, arg1 : *const core::ffi::c_void, arg2 : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoCheckFunctionAccess(grantedaccess, majorfunction, minorfunction, iocontrolcode, core::mem::transmute(arg1.unwrap_or(std::ptr::null())), core::mem::transmute(arg2.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn IoCheckQuerySetFileInformation<P0>(fileinformationclass: FILE_INFORMATION_CLASS, length: u32, setoperation: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckQuerySetFileInformation(fileinformationclass : FILE_INFORMATION_CLASS, length : u32, setoperation : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoCheckQuerySetFileInformation(fileinformationclass, length, setoperation.param().abi())
}
#[inline]
pub unsafe fn IoCheckQuerySetVolumeInformation<P0>(fsinformationclass: FS_INFORMATION_CLASS, length: u32, setoperation: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckQuerySetVolumeInformation(fsinformationclass : FS_INFORMATION_CLASS, length : u32, setoperation : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoCheckQuerySetVolumeInformation(fsinformationclass, length, setoperation.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IoCheckQuotaBufferValidity(quotabuffer: *const FILE_QUOTA_INFORMATION, quotalength: u32, erroroffset: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckQuotaBufferValidity(quotabuffer : *const FILE_QUOTA_INFORMATION, quotalength : u32, erroroffset : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoCheckQuotaBufferValidity(quotabuffer, quotalength, erroroffset)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoClearFsTrackOffsetState(irp: *mut super::super::Foundation::IRP) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoClearFsTrackOffsetState(irp : *mut super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoClearFsTrackOffsetState(irp)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoCreateStreamFileObject(fileobject: Option<*const super::super::Foundation::FILE_OBJECT>, deviceobject: Option<*const super::super::Foundation::DEVICE_OBJECT>) -> *mut super::super::Foundation::FILE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateStreamFileObject(fileobject : *const super::super::Foundation:: FILE_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: FILE_OBJECT);
    IoCreateStreamFileObject(core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), core::mem::transmute(deviceobject.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoCreateStreamFileObjectEx(fileobject: Option<*const super::super::Foundation::FILE_OBJECT>, deviceobject: Option<*const super::super::Foundation::DEVICE_OBJECT>, filehandle: Option<*mut super::super::super::Win32::Foundation::HANDLE>) -> *mut super::super::Foundation::FILE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateStreamFileObjectEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, filehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> *mut super::super::Foundation:: FILE_OBJECT);
    IoCreateStreamFileObjectEx(core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), core::mem::transmute(deviceobject.unwrap_or(std::ptr::null())), core::mem::transmute(filehandle.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoCreateStreamFileObjectEx2(createoptions: *const IO_CREATE_STREAM_FILE_OPTIONS, fileobject: Option<*const super::super::Foundation::FILE_OBJECT>, deviceobject: Option<*const super::super::Foundation::DEVICE_OBJECT>, streamfileobject: *mut *mut super::super::Foundation::FILE_OBJECT, filehandle: Option<*mut super::super::super::Win32::Foundation::HANDLE>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateStreamFileObjectEx2(createoptions : *const IO_CREATE_STREAM_FILE_OPTIONS, fileobject : *const super::super::Foundation:: FILE_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, streamfileobject : *mut *mut super::super::Foundation:: FILE_OBJECT, filehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoCreateStreamFileObjectEx2(createoptions, core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), core::mem::transmute(deviceobject.unwrap_or(std::ptr::null())), streamfileobject, core::mem::transmute(filehandle.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoCreateStreamFileObjectLite(fileobject: Option<*const super::super::Foundation::FILE_OBJECT>, deviceobject: Option<*const super::super::Foundation::DEVICE_OBJECT>) -> *mut super::super::Foundation::FILE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateStreamFileObjectLite(fileobject : *const super::super::Foundation:: FILE_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: FILE_OBJECT);
    IoCreateStreamFileObjectLite(core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), core::mem::transmute(deviceobject.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoEnumerateDeviceObjectList(driverobject: *const super::super::Foundation::DRIVER_OBJECT, deviceobjectlist: Option<*mut *mut super::super::Foundation::DEVICE_OBJECT>, deviceobjectlistsize: u32, actualnumberdeviceobjects: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoEnumerateDeviceObjectList(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, deviceobjectlist : *mut *mut super::super::Foundation:: DEVICE_OBJECT, deviceobjectlistsize : u32, actualnumberdeviceobjects : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoEnumerateDeviceObjectList(driverobject, core::mem::transmute(deviceobjectlist.unwrap_or(std::ptr::null_mut())), deviceobjectlistsize, actualnumberdeviceobjects)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoEnumerateRegisteredFiltersList(driverobjectlist: Option<*mut *mut super::super::Foundation::DRIVER_OBJECT>, driverobjectlistsize: u32, actualnumberdriverobjects: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoEnumerateRegisteredFiltersList(driverobjectlist : *mut *mut super::super::Foundation:: DRIVER_OBJECT, driverobjectlistsize : u32, actualnumberdriverobjects : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoEnumerateRegisteredFiltersList(core::mem::transmute(driverobjectlist.unwrap_or(std::ptr::null_mut())), driverobjectlistsize, actualnumberdriverobjects)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn IoFastQueryNetworkAttributes(objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, desiredaccess: u32, openoptions: u32, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *mut FILE_NETWORK_OPEN_INFORMATION) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoFastQueryNetworkAttributes(objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, desiredaccess : u32, openoptions : u32, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut FILE_NETWORK_OPEN_INFORMATION) -> super::super::super::Win32::Foundation:: BOOLEAN);
    IoFastQueryNetworkAttributes(objectattributes, desiredaccess, openoptions, iostatus, buffer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetAttachedDevice(deviceobject: *const super::super::Foundation::DEVICE_OBJECT) -> *mut super::super::Foundation::DEVICE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetAttachedDevice(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
    IoGetAttachedDevice(deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetBaseFileSystemDeviceObject(fileobject: *const super::super::Foundation::FILE_OBJECT) -> *mut super::super::Foundation::DEVICE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetBaseFileSystemDeviceObject(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
    IoGetBaseFileSystemDeviceObject(fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetDeviceAttachmentBaseRef(deviceobject: *const super::super::Foundation::DEVICE_OBJECT) -> *mut super::super::Foundation::DEVICE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceAttachmentBaseRef(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
    IoGetDeviceAttachmentBaseRef(deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetDeviceToVerify<P0>(thread: P0) -> *mut super::super::Foundation::DEVICE_OBJECT
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceToVerify(thread : super::super::Foundation:: PETHREAD) -> *mut super::super::Foundation:: DEVICE_OBJECT);
    IoGetDeviceToVerify(thread.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetDiskDeviceObject(filesystemdeviceobject: *const super::super::Foundation::DEVICE_OBJECT, diskdeviceobject: *mut *mut super::super::Foundation::DEVICE_OBJECT) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDiskDeviceObject(filesystemdeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, diskdeviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoGetDiskDeviceObject(filesystemdeviceobject, diskdeviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetFsTrackOffsetState(irp: *const super::super::Foundation::IRP, retfstrackoffsetblob: *mut *mut super::super::super::Win32::System::Ioctl::IO_IRP_EXT_TRACK_OFFSET_HEADER, rettrackedoffset: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetFsTrackOffsetState(irp : *const super::super::Foundation:: IRP, retfstrackoffsetblob : *mut *mut super::super::super::Win32::System::Ioctl:: IO_IRP_EXT_TRACK_OFFSET_HEADER, rettrackedoffset : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoGetFsTrackOffsetState(irp, retfstrackoffsetblob, rettrackedoffset)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetLowerDeviceObject(deviceobject: *const super::super::Foundation::DEVICE_OBJECT) -> *mut super::super::Foundation::DEVICE_OBJECT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetLowerDeviceObject(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
    IoGetLowerDeviceObject(deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetOplockKeyContext(fileobject: *const super::super::Foundation::FILE_OBJECT) -> *mut OPLOCK_KEY_ECP_CONTEXT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetOplockKeyContext(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut OPLOCK_KEY_ECP_CONTEXT);
    IoGetOplockKeyContext(fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetOplockKeyContextEx(fileobject: *const super::super::Foundation::FILE_OBJECT) -> *mut OPLOCK_KEY_CONTEXT {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetOplockKeyContextEx(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut OPLOCK_KEY_CONTEXT);
    IoGetOplockKeyContextEx(fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetRequestorProcess(irp: *const super::super::Foundation::IRP) -> super::super::Foundation::PEPROCESS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetRequestorProcess(irp : *const super::super::Foundation:: IRP) -> super::super::Foundation:: PEPROCESS);
    IoGetRequestorProcess(irp)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetRequestorProcessId(irp: *const super::super::Foundation::IRP) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetRequestorProcessId(irp : *const super::super::Foundation:: IRP) -> u32);
    IoGetRequestorProcessId(irp)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoGetRequestorSessionId(irp: *const super::super::Foundation::IRP, psessionid: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoGetRequestorSessionId(irp : *const super::super::Foundation:: IRP, psessionid : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoGetRequestorSessionId(irp, psessionid)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoIrpHasFsTrackOffsetExtensionType(irp: *const super::super::Foundation::IRP) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoIrpHasFsTrackOffsetExtensionType(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
    IoIrpHasFsTrackOffsetExtensionType(irp)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoIsOperationSynchronous(irp: *const super::super::Foundation::IRP) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoIsOperationSynchronous(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
    IoIsOperationSynchronous(irp)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn IoIsSystemThread<P0>(thread: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoIsSystemThread(thread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
    IoIsSystemThread(thread.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoIsValidNameGraftingBuffer(irp: *const super::super::Foundation::IRP, reparsebuffer: *const REPARSE_DATA_BUFFER) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoIsValidNameGraftingBuffer(irp : *const super::super::Foundation:: IRP, reparsebuffer : *const REPARSE_DATA_BUFFER) -> super::super::super::Win32::Foundation:: BOOLEAN);
    IoIsValidNameGraftingBuffer(irp, reparsebuffer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoPageRead(fileobject: *const super::super::Foundation::FILE_OBJECT, memorydescriptorlist: *const super::super::Foundation::MDL, startingoffset: *const i64, event: *const super::super::Foundation::KEVENT, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoPageRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, memorydescriptorlist : *const super::super::Foundation:: MDL, startingoffset : *const i64, event : *const super::super::Foundation:: KEVENT, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoPageRead(fileobject, memorydescriptorlist, startingoffset, event, iostatusblock)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoQueryFileDosDeviceName(fileobject: *const super::super::Foundation::FILE_OBJECT, objectnameinformation: *mut *mut super::super::Foundation::OBJECT_NAME_INFORMATION) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoQueryFileDosDeviceName(fileobject : *const super::super::Foundation:: FILE_OBJECT, objectnameinformation : *mut *mut super::super::Foundation:: OBJECT_NAME_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoQueryFileDosDeviceName(fileobject, objectnameinformation)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoQueryFileInformation(fileobject: *const super::super::Foundation::FILE_OBJECT, fileinformationclass: FILE_INFORMATION_CLASS, length: u32, fileinformation: *mut core::ffi::c_void, returnedlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoQueryFileInformation(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileinformationclass : FILE_INFORMATION_CLASS, length : u32, fileinformation : *mut core::ffi::c_void, returnedlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoQueryFileInformation(fileobject, fileinformationclass, length, fileinformation, returnedlength)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoQueryVolumeInformation(fileobject: *const super::super::Foundation::FILE_OBJECT, fsinformationclass: FS_INFORMATION_CLASS, length: u32, fsinformation: *mut core::ffi::c_void, returnedlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoQueryVolumeInformation(fileobject : *const super::super::Foundation:: FILE_OBJECT, fsinformationclass : FS_INFORMATION_CLASS, length : u32, fsinformation : *mut core::ffi::c_void, returnedlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoQueryVolumeInformation(fileobject, fsinformationclass, length, fsinformation, returnedlength)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoQueueThreadIrp(irp: *const super::super::Foundation::IRP) {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoQueueThreadIrp(irp : *const super::super::Foundation:: IRP));
    IoQueueThreadIrp(irp)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoRegisterFileSystem(deviceobject: *const super::super::Foundation::DEVICE_OBJECT) {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterFileSystem(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
    IoRegisterFileSystem(deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoRegisterFsRegistrationChange(driverobject: *const super::super::Foundation::DRIVER_OBJECT, drivernotificationroutine: super::super::Foundation::DRIVER_FS_NOTIFICATION) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterFsRegistrationChange(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, drivernotificationroutine : super::super::Foundation:: DRIVER_FS_NOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoRegisterFsRegistrationChange(driverobject, drivernotificationroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoRegisterFsRegistrationChangeMountAware<P0>(driverobject: *const super::super::Foundation::DRIVER_OBJECT, drivernotificationroutine: super::super::Foundation::DRIVER_FS_NOTIFICATION, synchronizewithmounts: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterFsRegistrationChangeMountAware(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, drivernotificationroutine : super::super::Foundation:: DRIVER_FS_NOTIFICATION, synchronizewithmounts : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoRegisterFsRegistrationChangeMountAware(driverobject, drivernotificationroutine, synchronizewithmounts.param().abi())
}
#[inline]
pub unsafe fn IoReleaseVpbSpinLock(irql: u8) {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoReleaseVpbSpinLock(irql : u8));
    IoReleaseVpbSpinLock(irql)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoReplaceFileObjectName<P0>(fileobject: *const super::super::Foundation::FILE_OBJECT, newfilename: P0, filenamelength: u16) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoReplaceFileObjectName(fileobject : *const super::super::Foundation:: FILE_OBJECT, newfilename : windows_core::PCWSTR, filenamelength : u16) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoReplaceFileObjectName(fileobject, newfilename.param().abi(), filenamelength)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoRequestDeviceRemovalForReset(physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoRequestDeviceRemovalForReset(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoRequestDeviceRemovalForReset(physicaldeviceobject, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoRetrievePriorityInfo<P0>(irp: Option<*const super::super::Foundation::IRP>, fileobject: Option<*const super::super::Foundation::FILE_OBJECT>, thread: P0, priorityinfo: *mut IO_PRIORITY_INFO) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoRetrievePriorityInfo(irp : *const super::super::Foundation:: IRP, fileobject : *const super::super::Foundation:: FILE_OBJECT, thread : super::super::Foundation:: PETHREAD, priorityinfo : *mut IO_PRIORITY_INFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoRetrievePriorityInfo(core::mem::transmute(irp.unwrap_or(std::ptr::null())), core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), thread.param().abi(), priorityinfo)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoSetDeviceToVerify<P0>(thread: P0, deviceobject: Option<*const super::super::Foundation::DEVICE_OBJECT>)
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoSetDeviceToVerify(thread : super::super::Foundation:: PETHREAD, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
    IoSetDeviceToVerify(thread.param().abi(), core::mem::transmute(deviceobject.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoSetFsTrackOffsetState(irp: *mut super::super::Foundation::IRP, fstrackoffsetblob: *const super::super::super::Win32::System::Ioctl::IO_IRP_EXT_TRACK_OFFSET_HEADER, trackedoffset: i64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoSetFsTrackOffsetState(irp : *mut super::super::Foundation:: IRP, fstrackoffsetblob : *const super::super::super::Win32::System::Ioctl:: IO_IRP_EXT_TRACK_OFFSET_HEADER, trackedoffset : i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoSetFsTrackOffsetState(irp, fstrackoffsetblob, trackedoffset)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoSetInformation(fileobject: *const super::super::Foundation::FILE_OBJECT, fileinformationclass: FILE_INFORMATION_CLASS, length: u32, fileinformation: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoSetInformation(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileinformationclass : FILE_INFORMATION_CLASS, length : u32, fileinformation : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoSetInformation(fileobject, fileinformationclass, length, fileinformation)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoSynchronousPageWrite(fileobject: *const super::super::Foundation::FILE_OBJECT, memorydescriptorlist: *const super::super::Foundation::MDL, startingoffset: *const i64, event: *const super::super::Foundation::KEVENT, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoSynchronousPageWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, memorydescriptorlist : *const super::super::Foundation:: MDL, startingoffset : *const i64, event : *const super::super::Foundation:: KEVENT, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoSynchronousPageWrite(fileobject, memorydescriptorlist, startingoffset, event, iostatusblock)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn IoThreadToProcess<P0>(thread: P0) -> super::super::Foundation::PEPROCESS
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoThreadToProcess(thread : super::super::Foundation:: PETHREAD) -> super::super::Foundation:: PEPROCESS);
    IoThreadToProcess(thread.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoUnregisterFileSystem(deviceobject: *const super::super::Foundation::DEVICE_OBJECT) {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoUnregisterFileSystem(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
    IoUnregisterFileSystem(deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoUnregisterFsRegistrationChange(driverobject: *const super::super::Foundation::DRIVER_OBJECT, drivernotificationroutine: super::super::Foundation::DRIVER_FS_NOTIFICATION) {
    windows_targets::link!("ntoskrnl.exe" "system" fn IoUnregisterFsRegistrationChange(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, drivernotificationroutine : super::super::Foundation:: DRIVER_FS_NOTIFICATION));
    IoUnregisterFsRegistrationChange(driverobject, drivernotificationroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn IoVerifyVolume<P0>(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, allowrawmount: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn IoVerifyVolume(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, allowrawmount : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IoVerifyVolume(deviceobject, allowrawmount.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn KeAcquireQueuedSpinLock(number: super::super::Foundation::KSPIN_LOCK_QUEUE_NUMBER) -> u8 {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireQueuedSpinLock(number : super::super::Foundation:: KSPIN_LOCK_QUEUE_NUMBER) -> u8);
    KeAcquireQueuedSpinLock(number)
}
#[inline]
pub unsafe fn KeAcquireSpinLockRaiseToSynch(spinlock: *mut usize) -> u8 {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireSpinLockRaiseToSynch(spinlock : *mut usize) -> u8);
    KeAcquireSpinLockRaiseToSynch(spinlock)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn KeAttachProcess<P0>(process: P0)
where
    P0: windows_core::Param<super::super::Foundation::PRKPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn KeAttachProcess(process : super::super::Foundation:: PRKPROCESS));
    KeAttachProcess(process.param().abi())
}
#[inline]
pub unsafe fn KeDetachProcess() {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeDetachProcess());
    KeDetachProcess()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeInitializeMutant<P0>(mutant: *mut super::super::Foundation::KMUTANT, initialowner: P0)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeMutant(mutant : *mut super::super::Foundation:: KMUTANT, initialowner : super::super::super::Win32::Foundation:: BOOLEAN));
    KeInitializeMutant(mutant, initialowner.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeInitializeQueue(queue: *mut super::super::Foundation::KQUEUE, count: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeQueue(queue : *mut super::super::Foundation:: KQUEUE, count : u32));
    KeInitializeQueue(queue, count)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeInsertHeadQueue(queue: *mut super::super::Foundation::KQUEUE, entry: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY) -> i32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeInsertHeadQueue(queue : *mut super::super::Foundation:: KQUEUE, entry : *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY) -> i32);
    KeInsertHeadQueue(queue, entry)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeInsertQueue(queue: *mut super::super::Foundation::KQUEUE, entry: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY) -> i32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeInsertQueue(queue : *mut super::super::Foundation:: KQUEUE, entry : *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY) -> i32);
    KeInsertQueue(queue, entry)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeReadStateMutant(mutant: *const super::super::Foundation::KMUTANT) -> i32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeReadStateMutant(mutant : *const super::super::Foundation:: KMUTANT) -> i32);
    KeReadStateMutant(mutant)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeReadStateQueue(queue: *const super::super::Foundation::KQUEUE) -> i32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeReadStateQueue(queue : *const super::super::Foundation:: KQUEUE) -> i32);
    KeReadStateQueue(queue)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeReleaseMutant<P0, P1>(mutant: *mut super::super::Foundation::KMUTANT, increment: i32, abandoned: P0, wait: P1) -> i32
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseMutant(mutant : *mut super::super::Foundation:: KMUTANT, increment : i32, abandoned : super::super::super::Win32::Foundation:: BOOLEAN, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
    KeReleaseMutant(mutant, increment, abandoned.param().abi(), wait.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn KeReleaseQueuedSpinLock(number: super::super::Foundation::KSPIN_LOCK_QUEUE_NUMBER, oldirql: u8) {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseQueuedSpinLock(number : super::super::Foundation:: KSPIN_LOCK_QUEUE_NUMBER, oldirql : u8));
    KeReleaseQueuedSpinLock(number, oldirql)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeRemoveQueue(queue: *mut super::super::Foundation::KQUEUE, waitmode: i8, timeout: Option<*const i64>) -> *mut super::super::super::Win32::System::Kernel::LIST_ENTRY {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeRemoveQueue(queue : *mut super::super::Foundation:: KQUEUE, waitmode : i8, timeout : *const i64) -> *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY);
    KeRemoveQueue(queue, waitmode, core::mem::transmute(timeout.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeRemoveQueueEx<P0>(queue: *mut super::super::Foundation::KQUEUE, waitmode: i8, alertable: P0, timeout: Option<*const i64>, entryarray: &mut [*mut super::super::super::Win32::System::Kernel::LIST_ENTRY]) -> u32
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn KeRemoveQueueEx(queue : *mut super::super::Foundation:: KQUEUE, waitmode : i8, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *const i64, entryarray : *mut *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY, count : u32) -> u32);
    KeRemoveQueueEx(queue, waitmode, alertable.param().abi(), core::mem::transmute(timeout.unwrap_or(std::ptr::null())), core::mem::transmute(entryarray.as_ptr()), entryarray.len().try_into().unwrap())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeRundownQueue(queue: *mut super::super::Foundation::KQUEUE) -> *mut super::super::super::Win32::System::Kernel::LIST_ENTRY {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeRundownQueue(queue : *mut super::super::Foundation:: KQUEUE) -> *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY);
    KeRundownQueue(queue)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn KeSetIdealProcessorThread<P0>(thread: P0, processor: u8) -> u8
where
    P0: windows_core::Param<super::super::Foundation::PKTHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn KeSetIdealProcessorThread(thread : super::super::Foundation:: PKTHREAD, processor : u8) -> u8);
    KeSetIdealProcessorThread(thread.param().abi(), processor)
}
#[inline]
pub unsafe fn KeSetKernelStackSwapEnable<P0>(enable: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn KeSetKernelStackSwapEnable(enable : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    KeSetKernelStackSwapEnable(enable.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn KeStackAttachProcess<P0>(process: P0, apcstate: *mut KAPC_STATE)
where
    P0: windows_core::Param<super::super::Foundation::PRKPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn KeStackAttachProcess(process : super::super::Foundation:: PRKPROCESS, apcstate : *mut KAPC_STATE));
    KeStackAttachProcess(process.param().abi(), apcstate)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn KeTryToAcquireQueuedSpinLock(number: super::super::Foundation::KSPIN_LOCK_QUEUE_NUMBER, oldirql: *mut u8) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeTryToAcquireQueuedSpinLock(number : super::super::Foundation:: KSPIN_LOCK_QUEUE_NUMBER, oldirql : *mut u8) -> u32);
    KeTryToAcquireQueuedSpinLock(number, oldirql)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn KeUnstackDetachProcess(apcstate: *const KAPC_STATE) {
    windows_targets::link!("ntoskrnl.exe" "system" fn KeUnstackDetachProcess(apcstate : *const KAPC_STATE));
    KeUnstackDetachProcess(apcstate)
}
#[inline]
pub unsafe fn MakeSignature(phcontext: *const SecHandle, fqop: u32, pmessage: *const SecBufferDesc, messageseqno: u32) -> windows_core::Result<()> {
    windows_targets::link!("secur32.dll" "system" fn MakeSignature(phcontext : *const SecHandle, fqop : u32, pmessage : *const SecBufferDesc, messageseqno : u32) -> windows_core::HRESULT);
    MakeSignature(phcontext, fqop, pmessage, messageseqno).ok()
}
#[inline]
pub unsafe fn MapSecurityError(secstatus: windows_core::HRESULT) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ksecdd.sys" "system" fn MapSecurityError(secstatus : windows_core::HRESULT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    MapSecurityError(secstatus)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn MmCanFileBeTruncated(sectionpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, newfilesize: Option<*const i64>) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmCanFileBeTruncated(sectionpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, newfilesize : *const i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
    MmCanFileBeTruncated(sectionpointer, core::mem::transmute(newfilesize.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn MmDoesFileHaveUserWritableReferences(sectionpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmDoesFileHaveUserWritableReferences(sectionpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS) -> u32);
    MmDoesFileHaveUserWritableReferences(sectionpointer)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn MmFlushImageSection(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, flushtype: MMFLUSH_TYPE) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmFlushImageSection(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, flushtype : MMFLUSH_TYPE) -> super::super::super::Win32::Foundation:: BOOLEAN);
    MmFlushImageSection(sectionobjectpointer, flushtype)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn MmForceSectionClosed<P0>(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, delayclose: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn MmForceSectionClosed(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, delayclose : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    MmForceSectionClosed(sectionobjectpointer, delayclose.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn MmForceSectionClosedEx(sectionobjectpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, forcecloseflags: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmForceSectionClosedEx(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, forcecloseflags : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    MmForceSectionClosedEx(sectionobjectpointer, forcecloseflags)
}
#[inline]
pub unsafe fn MmGetMaximumFileSectionSize() -> u64 {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmGetMaximumFileSectionSize() -> u64);
    MmGetMaximumFileSectionSize()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn MmIsFileSectionActive(fssectionpointer: *const super::super::Foundation::SECTION_OBJECT_POINTERS, flags: u32, sectionisactive: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmIsFileSectionActive(fssectionpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, flags : u32, sectionisactive : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    MmIsFileSectionActive(fssectionpointer, flags, sectionisactive)
}
#[inline]
pub unsafe fn MmIsRecursiveIoFault() -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmIsRecursiveIoFault() -> super::super::super::Win32::Foundation:: BOOLEAN);
    MmIsRecursiveIoFault()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn MmMdlPagesAreZero(mdl: *const super::super::Foundation::MDL) -> u32 {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmMdlPagesAreZero(mdl : *const super::super::Foundation:: MDL) -> u32);
    MmMdlPagesAreZero(mdl)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn MmPrefetchPages(readlists: &[*const READ_LIST]) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmPrefetchPages(numberoflists : u32, readlists : *const *const READ_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
    MmPrefetchPages(readlists.len().try_into().unwrap(), core::mem::transmute(readlists.as_ptr()))
}
#[inline]
pub unsafe fn MmSetAddressRangeModified(address: *const core::ffi::c_void, length: usize) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn MmSetAddressRangeModified(address : *const core::ffi::c_void, length : usize) -> super::super::super::Win32::Foundation:: BOOLEAN);
    MmSetAddressRangeModified(address, length)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtAccessCheckAndAuditAlarm<P0, P1>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING, objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING, securitydescriptor: P0, desiredaccess: u32, genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING, objectcreation: P1, grantedaccess: *mut u32, accessstatus: *mut i32, generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtAccessCheckAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtAccessCheckAndAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename, objectname, securitydescriptor.param().abi(), desiredaccess, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, generateonclose)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtAccessCheckByTypeAndAuditAlarm<P0, P1, P2>(
    subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    handleid: Option<*const core::ffi::c_void>,
    objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    securitydescriptor: P0,
    principalselfsid: P1,
    desiredaccess: u32,
    audittype: super::super::super::Win32::Security::AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: Option<&[super::super::super::Win32::Security::OBJECT_TYPE_LIST]>,
    genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING,
    objectcreation: P2,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN,
) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtAccessCheckByTypeAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Security:: PSID, desiredaccess : u32, audittype : super::super::super::Win32::Security:: AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::super::super::Win32::Security:: OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtAccessCheckByTypeAndAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename, objectname, securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, generateonclose)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtAccessCheckByTypeResultListAndAuditAlarm<P0, P1, P2>(
    subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    handleid: Option<*const core::ffi::c_void>,
    objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    securitydescriptor: P0,
    principalselfsid: P1,
    desiredaccess: u32,
    audittype: super::super::super::Win32::Security::AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: Option<*const super::super::super::Win32::Security::OBJECT_TYPE_LIST>,
    objecttypelistlength: u32,
    genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING,
    objectcreation: P2,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN,
) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Security:: PSID, desiredaccess : u32, audittype : super::super::super::Win32::Security:: AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::super::super::Win32::Security:: OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename, objectname, securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null())), objecttypelistlength, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, generateonclose)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle<P0, P1, P2, P3>(
    subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    handleid: Option<*const core::ffi::c_void>,
    clienttoken: P0,
    objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    securitydescriptor: P1,
    principalselfsid: P2,
    desiredaccess: u32,
    audittype: super::super::super::Win32::Security::AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: Option<*const super::super::super::Win32::Security::OBJECT_TYPE_LIST>,
    objecttypelistlength: u32,
    genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING,
    objectcreation: P3,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN,
) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P2: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, clienttoken : super::super::super::Win32::Foundation:: HANDLE, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Security:: PSID, desiredaccess : u32, audittype : super::super::super::Win32::Security:: AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::super::super::Win32::Security:: OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), clienttoken.param().abi(), objecttypename, objectname, securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null())), objecttypelistlength, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, generateonclose)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtAdjustGroupsToken<P0, P1>(tokenhandle: P0, resettodefault: P1, newstate: Option<*const super::super::super::Win32::Security::TOKEN_GROUPS>, bufferlength: u32, previousstate: Option<*mut super::super::super::Win32::Security::TOKEN_GROUPS>, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtAdjustGroupsToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, resettodefault : super::super::super::Win32::Foundation:: BOOLEAN, newstate : *const super::super::super::Win32::Security:: TOKEN_GROUPS, bufferlength : u32, previousstate : *mut super::super::super::Win32::Security:: TOKEN_GROUPS, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtAdjustGroupsToken(tokenhandle.param().abi(), resettodefault.param().abi(), core::mem::transmute(newstate.unwrap_or(std::ptr::null())), bufferlength, core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())), returnlength)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtAdjustPrivilegesToken<P0, P1>(tokenhandle: P0, disableallprivileges: P1, newstate: Option<*const super::super::super::Win32::Security::TOKEN_PRIVILEGES>, bufferlength: u32, previousstate: Option<*mut super::super::super::Win32::Security::TOKEN_PRIVILEGES>, returnlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtAdjustPrivilegesToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, disableallprivileges : super::super::super::Win32::Foundation:: BOOLEAN, newstate : *const super::super::super::Win32::Security:: TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut super::super::super::Win32::Security:: TOKEN_PRIVILEGES, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtAdjustPrivilegesToken(tokenhandle.param().abi(), disableallprivileges.param().abi(), core::mem::transmute(newstate.unwrap_or(std::ptr::null())), bufferlength, core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NtAllocateVirtualMemory<P0>(processhandle: P0, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, regionsize: *mut usize, allocationtype: u32, protect: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtAllocateVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, regionsize : *mut usize, allocationtype : u32, protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtAllocateVirtualMemory(processhandle.param().abi(), baseaddress, zerobits, regionsize, allocationtype, protect)
}
#[cfg(feature = "Win32_System_Memory")]
#[inline]
pub unsafe fn NtAllocateVirtualMemoryEx<P0>(processhandle: P0, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::super::super::Win32::System::Memory::MEM_EXTENDED_PARAMETER]>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtAllocateVirtualMemoryEx(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::super::super::Win32::System::Memory:: MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtAllocateVirtualMemoryEx(processhandle.param().abi(), baseaddress, regionsize, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtCancelIoFileEx<P0>(filehandle: P0, iorequesttocancel: Option<*const super::super::super::Win32::System::IO::IO_STATUS_BLOCK>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtCancelIoFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, iorequesttocancel : *const super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCancelIoFileEx(filehandle.param().abi(), core::mem::transmute(iorequesttocancel.unwrap_or(std::ptr::null())), iostatusblock)
}
#[inline]
pub unsafe fn NtCloseObjectAuditAlarm<P0>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, generateonclose: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtCloseObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, generateonclose : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCloseObjectAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), generateonclose.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtCreateDirectoryObject(directoryhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtCreateDirectoryObject(directoryhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateDirectoryObject(directoryhandle, desiredaccess, objectattributes)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NtCreateEvent<P0>(eventhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, eventtype: super::super::super::Win32::System::Kernel::EVENT_TYPE, initialstate: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtCreateEvent(eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, eventtype : super::super::super::Win32::System::Kernel:: EVENT_TYPE, initialstate : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateEvent(eventhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), eventtype, initialstate.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NtCreateFile(filehandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: super::super::super::Win32::Storage::FileSystem::FILE_ACCESS_RIGHTS, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, allocationsize: Option<*const i64>, fileattributes: super::super::super::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, shareaccess: super::super::super::Win32::Storage::FileSystem::FILE_SHARE_MODE, createdisposition: NTCREATEFILE_CREATE_DISPOSITION, createoptions: NTCREATEFILE_CREATE_OPTIONS, eabuffer: Option<*const core::ffi::c_void>, ealength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtCreateFile(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : super::super::super::Win32::Storage::FileSystem:: FILE_ACCESS_RIGHTS, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : super::super::super::Win32::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, shareaccess : super::super::super::Win32::Storage::FileSystem:: FILE_SHARE_MODE, createdisposition : NTCREATEFILE_CREATE_DISPOSITION, createoptions : NTCREATEFILE_CREATE_OPTIONS, eabuffer : *const core::ffi::c_void, ealength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateFile(filehandle, desiredaccess, objectattributes, iostatusblock, core::mem::transmute(allocationsize.unwrap_or(std::ptr::null())), fileattributes, shareaccess, createdisposition, createoptions, core::mem::transmute(eabuffer.unwrap_or(std::ptr::null())), ealength)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtCreateSection<P0>(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, maximumsize: Option<*const i64>, sectionpageprotection: u32, allocationattributes: u32, filehandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtCreateSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateSection(sectionhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), core::mem::transmute(maximumsize.unwrap_or(std::ptr::null())), sectionpageprotection, allocationattributes, filehandle.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Memory"))]
#[inline]
pub unsafe fn NtCreateSectionEx<P0>(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, maximumsize: Option<*const i64>, sectionpageprotection: u32, allocationattributes: u32, filehandle: P0, extendedparameters: Option<&mut [super::super::super::Win32::System::Memory::MEM_EXTENDED_PARAMETER]>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtCreateSectionEx(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::super::super::Win32::Foundation:: HANDLE, extendedparameters : *mut super::super::super::Win32::System::Memory:: MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateSectionEx(sectionhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), core::mem::transmute(maximumsize.unwrap_or(std::ptr::null())), sectionpageprotection, allocationattributes, filehandle.param().abi(), core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtDeleteFile(objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtDeleteFile(objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDeleteFile(objectattributes)
}
#[inline]
pub unsafe fn NtDeleteObjectAuditAlarm<P0>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, generateonclose: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtDeleteObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, generateonclose : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDeleteObjectAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), generateonclose.param().abi())
}
#[inline]
pub unsafe fn NtDuplicateObject<P0, P1, P2>(sourceprocesshandle: P0, sourcehandle: P1, targetprocesshandle: P2, targethandle: Option<*mut super::super::super::Win32::Foundation::HANDLE>, desiredaccess: u32, handleattributes: u32, options: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtDuplicateObject(sourceprocesshandle : super::super::super::Win32::Foundation:: HANDLE, sourcehandle : super::super::super::Win32::Foundation:: HANDLE, targetprocesshandle : super::super::super::Win32::Foundation:: HANDLE, targethandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, handleattributes : u32, options : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDuplicateObject(sourceprocesshandle.param().abi(), sourcehandle.param().abi(), targetprocesshandle.param().abi(), core::mem::transmute(targethandle.unwrap_or(std::ptr::null_mut())), desiredaccess, handleattributes, options)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtDuplicateToken<P0, P1>(existingtokenhandle: P0, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, effectiveonly: P1, tokentype: super::super::super::Win32::Security::TOKEN_TYPE, newtokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtDuplicateToken(existingtokenhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, effectiveonly : super::super::super::Win32::Foundation:: BOOLEAN, tokentype : super::super::super::Win32::Security:: TOKEN_TYPE, newtokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDuplicateToken(existingtokenhandle.param().abi(), desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), effectiveonly.param().abi(), tokentype, newtokenhandle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtFilterToken<P0>(existingtokenhandle: P0, flags: u32, sidstodisable: Option<*const super::super::super::Win32::Security::TOKEN_GROUPS>, privilegestodelete: Option<*const super::super::super::Win32::Security::TOKEN_PRIVILEGES>, restrictedsids: Option<*const super::super::super::Win32::Security::TOKEN_GROUPS>, newtokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtFilterToken(existingtokenhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, sidstodisable : *const super::super::super::Win32::Security:: TOKEN_GROUPS, privilegestodelete : *const super::super::super::Win32::Security:: TOKEN_PRIVILEGES, restrictedsids : *const super::super::super::Win32::Security:: TOKEN_GROUPS, newtokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtFilterToken(existingtokenhandle.param().abi(), flags, core::mem::transmute(sidstodisable.unwrap_or(std::ptr::null())), core::mem::transmute(privilegestodelete.unwrap_or(std::ptr::null())), core::mem::transmute(restrictedsids.unwrap_or(std::ptr::null())), newtokenhandle)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtFlushBuffersFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtFlushBuffersFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtFlushBuffersFile(filehandle.param().abi(), iostatusblock)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtFlushBuffersFileEx<P0>(filehandle: P0, flags: u32, parameters: *const core::ffi::c_void, parameterssize: u32, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtFlushBuffersFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, parameters : *const core::ffi::c_void, parameterssize : u32, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtFlushBuffersFileEx(filehandle.param().abi(), flags, parameters, parameterssize, iostatusblock)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtFlushVirtualMemory<P0>(processhandle: P0, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtFlushVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtFlushVirtualMemory(processhandle.param().abi(), baseaddress, regionsize, iostatus)
}
#[inline]
pub unsafe fn NtFreeVirtualMemory<P0>(processhandle: P0, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, freetype: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtFreeVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, freetype : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtFreeVirtualMemory(processhandle.param().abi(), baseaddress, regionsize, freetype)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtFsControlFile<P0, P1>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fscontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtFsControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtFsControlFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, fscontrolcode, core::mem::transmute(inputbuffer.unwrap_or(std::ptr::null())), inputbufferlength, core::mem::transmute(outputbuffer.unwrap_or(std::ptr::null_mut())), outputbufferlength)
}
#[inline]
pub unsafe fn NtImpersonateAnonymousToken<P0>(threadhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtImpersonateAnonymousToken(threadhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtImpersonateAnonymousToken(threadhandle.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtLockFile<P0, P1, P2, P3>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, byteoffset: *const i64, length: *const i64, key: u32, failimmediately: P2, exclusivelock: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtLockFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32, failimmediately : super::super::super::Win32::Foundation:: BOOLEAN, exclusivelock : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtLockFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, byteoffset, length, key, failimmediately.param().abi(), exclusivelock.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtNotifyChangeKey<P0, P1, P2, P3>(keyhandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, completionfilter: u32, watchtree: P2, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, asynchronous: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtNotifyChangeKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, completionfilter : u32, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtNotifyChangeKey(keyhandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, completionfilter, watchtree.param().abi(), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, asynchronous.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenDirectoryObject(directoryhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenDirectoryObject(directoryhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenDirectoryObject(directoryhandle, desiredaccess, objectattributes)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NtOpenFile(filehandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenFile(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, shareaccess : u32, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenFile(filehandle, desiredaccess, objectattributes, iostatusblock, shareaccess, openoptions)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtOpenObjectAuditAlarm<P0, P1, P2, P3>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING, objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING, securitydescriptor: P0, clienttoken: P1, desiredaccess: u32, grantedaccess: u32, privileges: Option<*const super::super::super::Win32::Security::PRIVILEGE_SET>, objectcreation: P2, accessgranted: P3, generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtOpenObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, clienttoken : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenObjectAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename, objectname, securitydescriptor.param().abi(), clienttoken.param().abi(), desiredaccess, grantedaccess, core::mem::transmute(privileges.unwrap_or(std::ptr::null())), objectcreation.param().abi(), accessgranted.param().abi(), generateonclose)
}
#[inline]
pub unsafe fn NtOpenProcessToken<P0>(processhandle: P0, desiredaccess: u32, tokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtOpenProcessToken(processhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenProcessToken(processhandle.param().abi(), desiredaccess, tokenhandle)
}
#[inline]
pub unsafe fn NtOpenProcessTokenEx<P0>(processhandle: P0, desiredaccess: u32, handleattributes: u32, tokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtOpenProcessTokenEx(processhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, handleattributes : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenProcessTokenEx(processhandle.param().abi(), desiredaccess, handleattributes, tokenhandle)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenSymbolicLinkObject(linkhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenSymbolicLinkObject(linkhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenSymbolicLinkObject(linkhandle, desiredaccess, objectattributes)
}
#[inline]
pub unsafe fn NtOpenThreadToken<P0, P1>(threadhandle: P0, desiredaccess: u32, openasself: P1, tokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtOpenThreadToken(threadhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, openasself : super::super::super::Win32::Foundation:: BOOLEAN, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenThreadToken(threadhandle.param().abi(), desiredaccess, openasself.param().abi(), tokenhandle)
}
#[inline]
pub unsafe fn NtOpenThreadTokenEx<P0, P1>(threadhandle: P0, desiredaccess: u32, openasself: P1, handleattributes: u32, tokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtOpenThreadTokenEx(threadhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, openasself : super::super::super::Win32::Foundation:: BOOLEAN, handleattributes : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenThreadTokenEx(threadhandle.param().abi(), desiredaccess, openasself.param().abi(), handleattributes, tokenhandle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtPrivilegeCheck<P0>(clienttoken: P0, requiredprivileges: *mut super::super::super::Win32::Security::PRIVILEGE_SET, result: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtPrivilegeCheck(clienttoken : super::super::super::Win32::Foundation:: HANDLE, requiredprivileges : *mut super::super::super::Win32::Security:: PRIVILEGE_SET, result : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtPrivilegeCheck(clienttoken.param().abi(), requiredprivileges, result)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtPrivilegeObjectAuditAlarm<P0, P1>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, clienttoken: P0, desiredaccess: u32, privileges: *const super::super::super::Win32::Security::PRIVILEGE_SET, accessgranted: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtPrivilegeObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, clienttoken : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtPrivilegeObjectAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), clienttoken.param().abi(), desiredaccess, privileges, accessgranted.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtPrivilegedServiceAuditAlarm<P0, P1>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, servicename: *const super::super::super::Win32::Foundation::UNICODE_STRING, clienttoken: P0, privileges: *const super::super::super::Win32::Security::PRIVILEGE_SET, accessgranted: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtPrivilegedServiceAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, clienttoken : super::super::super::Win32::Foundation:: HANDLE, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtPrivilegedServiceAuditAlarm(subsystemname, servicename, clienttoken.param().abi(), privileges, accessgranted.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtQueryDirectoryFile<P0, P1, P2, P3>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: FILE_INFORMATION_CLASS, returnsingleentry: P2, filename: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, restartscan: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryDirectoryFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryDirectoryFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, fileinformation, length, fileinformationclass, returnsingleentry.param().abi(), core::mem::transmute(filename.unwrap_or(std::ptr::null())), restartscan.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtQueryDirectoryFileEx<P0, P1>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: FILE_INFORMATION_CLASS, queryflags: u32, filename: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryDirectoryFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS, queryflags : u32, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryDirectoryFileEx(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, fileinformation, length, fileinformationclass, queryflags, core::mem::transmute(filename.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn NtQueryDirectoryObject<P0, P1, P2>(directoryhandle: P0, buffer: Option<*mut core::ffi::c_void>, length: u32, returnsingleentry: P1, restartscan: P2, context: *mut u32, returnlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryDirectoryObject(directoryhandle : super::super::super::Win32::Foundation:: HANDLE, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, restartscan : super::super::super::Win32::Foundation:: BOOLEAN, context : *mut u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryDirectoryObject(directoryhandle.param().abi(), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), length, returnsingleentry.param().abi(), restartscan.param().abi(), context, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtQueryEaFile<P0, P1, P2>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: P1, ealist: Option<*const core::ffi::c_void>, ealistlength: u32, eaindex: Option<*const u32>, restartscan: P2) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryEaFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, ealist : *const core::ffi::c_void, ealistlength : u32, eaindex : *const u32, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryEaFile(filehandle.param().abi(), iostatusblock, buffer, length, returnsingleentry.param().abi(), core::mem::transmute(ealist.unwrap_or(std::ptr::null())), ealistlength, core::mem::transmute(eaindex.unwrap_or(std::ptr::null())), restartscan.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtQueryFullAttributesFile(objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, fileinformation: *mut FILE_NETWORK_OPEN_INFORMATION) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtQueryFullAttributesFile(objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, fileinformation : *mut FILE_NETWORK_OPEN_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryFullAttributesFile(objectattributes, fileinformation)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NtQueryInformationByName(objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationByName(objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryInformationByName(objectattributes, iostatusblock, fileinformation, length, fileinformationclass)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtQueryInformationFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryInformationFile(filehandle.param().abi(), iostatusblock, fileinformation, length, fileinformationclass)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtQueryInformationToken<P0>(tokenhandle: P0, tokeninformationclass: super::super::super::Win32::Security::TOKEN_INFORMATION_CLASS, tokeninformation: Option<*mut core::ffi::c_void>, tokeninformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, tokeninformationclass : super::super::super::Win32::Security:: TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryInformationToken(tokenhandle.param().abi(), tokeninformationclass, core::mem::transmute(tokeninformation.unwrap_or(std::ptr::null_mut())), tokeninformationlength, returnlength)
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NtQueryQuotaInformationFile<P0, P1, P2, P3>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: P1, sidlist: Option<*const core::ffi::c_void>, sidlistlength: u32, startsid: P2, restartscan: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryQuotaInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, sidlist : *const core::ffi::c_void, sidlistlength : u32, startsid : super::super::super::Win32::Security:: PSID, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryQuotaInformationFile(filehandle.param().abi(), iostatusblock, buffer, length, returnsingleentry.param().abi(), core::mem::transmute(sidlist.unwrap_or(std::ptr::null())), sidlistlength, startsid.param().abi(), restartscan.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtQuerySecurityObject<P0>(handle: P0, securityinformation: u32, securitydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, length: u32, lengthneeded: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQuerySecurityObject(handle : super::super::super::Win32::Foundation:: HANDLE, securityinformation : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, length : u32, lengthneeded : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQuerySecurityObject(handle.param().abi(), securityinformation, securitydescriptor, length, lengthneeded)
}
#[inline]
pub unsafe fn NtQuerySymbolicLinkObject<P0>(linkhandle: P0, linktarget: *mut super::super::super::Win32::Foundation::UNICODE_STRING, returnedlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQuerySymbolicLinkObject(linkhandle : super::super::super::Win32::Foundation:: HANDLE, linktarget : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, returnedlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQuerySymbolicLinkObject(linkhandle.param().abi(), linktarget, core::mem::transmute(returnedlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NtQueryVirtualMemory<P0>(processhandle: P0, baseaddress: Option<*const core::ffi::c_void>, memoryinformationclass: MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: Option<*mut usize>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, memoryinformationclass : MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationlength : usize, returnlength : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryVirtualMemory(processhandle.param().abi(), core::mem::transmute(baseaddress.unwrap_or(std::ptr::null())), memoryinformationclass, memoryinformation, memoryinformationlength, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtQueryVolumeInformationFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryVolumeInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryVolumeInformationFile(filehandle.param().abi(), iostatusblock, fsinformation, length, fsinformationclass)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtReadFile<P0, P1>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, byteoffset: Option<*const i64>, key: Option<*const u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtReadFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtReadFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, buffer, length, core::mem::transmute(byteoffset.unwrap_or(std::ptr::null())), core::mem::transmute(key.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtSetEaFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetEaFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetEaFile(filehandle.param().abi(), iostatusblock, buffer, length)
}
#[inline]
pub unsafe fn NtSetEvent<P0>(eventhandle: P0, previousstate: Option<*mut i32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetEvent(eventhandle : super::super::super::Win32::Foundation:: HANDLE, previousstate : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetEvent(eventhandle.param().abi(), core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtSetInformationFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *const core::ffi::c_void, length: u32, fileinformationclass: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetInformationFile(filehandle.param().abi(), iostatusblock, fileinformation, length, fileinformationclass)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtSetInformationToken<P0>(tokenhandle: P0, tokeninformationclass: super::super::super::Win32::Security::TOKEN_INFORMATION_CLASS, tokeninformation: *const core::ffi::c_void, tokeninformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetInformationToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, tokeninformationclass : super::super::super::Win32::Security:: TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetInformationToken(tokenhandle.param().abi(), tokeninformationclass, tokeninformation, tokeninformationlength)
}
#[inline]
pub unsafe fn NtSetInformationVirtualMemory<P0>(processhandle: P0, vminformationclass: VIRTUAL_MEMORY_INFORMATION_CLASS, virtualaddresses: &[MEMORY_RANGE_ENTRY], vminformation: *const core::ffi::c_void, vminformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetInformationVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, vminformationclass : VIRTUAL_MEMORY_INFORMATION_CLASS, numberofentries : usize, virtualaddresses : *const MEMORY_RANGE_ENTRY, vminformation : *const core::ffi::c_void, vminformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetInformationVirtualMemory(processhandle.param().abi(), vminformationclass, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), vminformation, vminformationlength)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtSetQuotaInformationFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetQuotaInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetQuotaInformationFile(filehandle.param().abi(), iostatusblock, buffer, length)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NtSetSecurityObject<P0, P1>(handle: P0, securityinformation: u32, securitydescriptor: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetSecurityObject(handle : super::super::super::Win32::Foundation:: HANDLE, securityinformation : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetSecurityObject(handle.param().abi(), securityinformation, securitydescriptor.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtSetVolumeInformationFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fsinformation: *const core::ffi::c_void, length: u32, fsinformationclass: FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetVolumeInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fsinformation : *const core::ffi::c_void, length : u32, fsinformationclass : FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetVolumeInformationFile(filehandle.param().abi(), iostatusblock, fsinformation, length, fsinformationclass)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtUnlockFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, byteoffset: *const i64, length: *const i64, key: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtUnlockFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtUnlockFile(filehandle.param().abi(), iostatusblock, byteoffset, length, key)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtWriteFile<P0, P1>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32, byteoffset: Option<*const i64>, key: Option<*const u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtWriteFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtWriteFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, buffer, length, core::mem::transmute(byteoffset.unwrap_or(std::ptr::null())), core::mem::transmute(key.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ObInsertObject(object: *const core::ffi::c_void, passedaccessstate: Option<*mut super::super::Foundation::ACCESS_STATE>, desiredaccess: u32, objectpointerbias: u32, newobject: Option<*mut *mut core::ffi::c_void>, handle: Option<*mut super::super::super::Win32::Foundation::HANDLE>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn ObInsertObject(object : *const core::ffi::c_void, passedaccessstate : *mut super::super::Foundation:: ACCESS_STATE, desiredaccess : u32, objectpointerbias : u32, newobject : *mut *mut core::ffi::c_void, handle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ObInsertObject(object, core::mem::transmute(passedaccessstate.unwrap_or(std::ptr::null_mut())), desiredaccess, objectpointerbias, core::mem::transmute(newobject.unwrap_or(std::ptr::null_mut())), core::mem::transmute(handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ObIsKernelHandle<P0>(handle: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn ObIsKernelHandle(handle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: BOOLEAN);
    ObIsKernelHandle(handle.param().abi())
}
#[inline]
pub unsafe fn ObMakeTemporaryObject(object: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn ObMakeTemporaryObject(object : *const core::ffi::c_void));
    ObMakeTemporaryObject(object)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ObOpenObjectByPointer<P0>(object: *const core::ffi::c_void, handleattributes: u32, passedaccessstate: Option<*const super::super::Foundation::ACCESS_STATE>, desiredaccess: u32, objecttype: P0, accessmode: i8, handle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::POBJECT_TYPE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn ObOpenObjectByPointer(object : *const core::ffi::c_void, handleattributes : u32, passedaccessstate : *const super::super::Foundation:: ACCESS_STATE, desiredaccess : u32, objecttype : super::super::Foundation:: POBJECT_TYPE, accessmode : i8, handle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ObOpenObjectByPointer(object, handleattributes, core::mem::transmute(passedaccessstate.unwrap_or(std::ptr::null())), desiredaccess, objecttype.param().abi(), accessmode, handle)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ObOpenObjectByPointerWithTag<P0>(object: *const core::ffi::c_void, handleattributes: u32, passedaccessstate: Option<*const super::super::Foundation::ACCESS_STATE>, desiredaccess: u32, objecttype: P0, accessmode: i8, tag: u32, handle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::POBJECT_TYPE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn ObOpenObjectByPointerWithTag(object : *const core::ffi::c_void, handleattributes : u32, passedaccessstate : *const super::super::Foundation:: ACCESS_STATE, desiredaccess : u32, objecttype : super::super::Foundation:: POBJECT_TYPE, accessmode : i8, tag : u32, handle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ObOpenObjectByPointerWithTag(object, handleattributes, core::mem::transmute(passedaccessstate.unwrap_or(std::ptr::null())), desiredaccess, objecttype.param().abi(), accessmode, tag, handle)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ObQueryNameString(object: *const core::ffi::c_void, objectnameinfo: Option<*mut super::super::Foundation::OBJECT_NAME_INFORMATION>, length: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn ObQueryNameString(object : *const core::ffi::c_void, objectnameinfo : *mut super::super::Foundation:: OBJECT_NAME_INFORMATION, length : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ObQueryNameString(object, core::mem::transmute(objectnameinfo.unwrap_or(std::ptr::null_mut())), length, returnlength)
}
#[inline]
pub unsafe fn ObQueryObjectAuditingByHandle<P0>(handle: P0, generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn ObQueryObjectAuditingByHandle(handle : super::super::super::Win32::Foundation:: HANDLE, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ObQueryObjectAuditingByHandle(handle.param().abi(), generateonclose)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn PfxFindPrefix(prefixtable: *const PREFIX_TABLE, fullname: *const super::super::super::Win32::System::Kernel::STRING) -> *mut PREFIX_TABLE_ENTRY {
    windows_targets::link!("ntdll.dll" "system" fn PfxFindPrefix(prefixtable : *const PREFIX_TABLE, fullname : *const super::super::super::Win32::System::Kernel:: STRING) -> *mut PREFIX_TABLE_ENTRY);
    PfxFindPrefix(prefixtable, fullname)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn PfxInitialize() -> PREFIX_TABLE {
    windows_targets::link!("ntdll.dll" "system" fn PfxInitialize(prefixtable : *mut PREFIX_TABLE));
    let mut result__ = core::mem::zeroed();
    PfxInitialize(&mut result__);
    result__
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn PfxInsertPrefix(prefixtable: *const PREFIX_TABLE, prefix: *const super::super::super::Win32::System::Kernel::STRING, prefixtableentry: *mut PREFIX_TABLE_ENTRY) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntdll.dll" "system" fn PfxInsertPrefix(prefixtable : *const PREFIX_TABLE, prefix : *const super::super::super::Win32::System::Kernel:: STRING, prefixtableentry : *mut PREFIX_TABLE_ENTRY) -> super::super::super::Win32::Foundation:: BOOLEAN);
    PfxInsertPrefix(prefixtable, prefix, prefixtableentry)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn PfxRemovePrefix(prefixtable: *const PREFIX_TABLE, prefixtableentry: *const PREFIX_TABLE_ENTRY) {
    windows_targets::link!("ntdll.dll" "system" fn PfxRemovePrefix(prefixtable : *const PREFIX_TABLE, prefixtableentry : *const PREFIX_TABLE_ENTRY));
    PfxRemovePrefix(prefixtable, prefixtableentry)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn PoQueueShutdownWorkItem(workitem: *mut super::super::Foundation::WORK_QUEUE_ITEM) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn PoQueueShutdownWorkItem(workitem : *mut super::super::Foundation:: WORK_QUEUE_ITEM) -> super::super::super::Win32::Foundation:: NTSTATUS);
    PoQueueShutdownWorkItem(workitem)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsAssignImpersonationToken<P0, P1>(thread: P0, token: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsAssignImpersonationToken(thread : super::super::Foundation:: PETHREAD, token : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    PsAssignImpersonationToken(thread.param().abi(), token.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsChargePoolQuota<P0>(process: P0, pooltype: super::super::Foundation::POOL_TYPE, amount: usize)
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsChargePoolQuota(process : super::super::Foundation:: PEPROCESS, pooltype : super::super::Foundation:: POOL_TYPE, amount : usize));
    PsChargePoolQuota(process.param().abi(), pooltype, amount)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsChargeProcessPoolQuota<P0>(process: P0, pooltype: super::super::Foundation::POOL_TYPE, amount: usize) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsChargeProcessPoolQuota(process : super::super::Foundation:: PEPROCESS, pooltype : super::super::Foundation:: POOL_TYPE, amount : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
    PsChargeProcessPoolQuota(process.param().abi(), pooltype, amount)
}
#[inline]
pub unsafe fn PsDereferenceImpersonationToken(impersonationtoken: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn PsDereferenceImpersonationToken(impersonationtoken : *const core::ffi::c_void));
    PsDereferenceImpersonationToken(impersonationtoken)
}
#[inline]
pub unsafe fn PsDereferencePrimaryToken(primarytoken: *const core::ffi::c_void) {
    windows_targets::link!("ntoskrnl.exe" "system" fn PsDereferencePrimaryToken(primarytoken : *const core::ffi::c_void));
    PsDereferencePrimaryToken(primarytoken)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn PsDisableImpersonation<P0>(thread: P0, impersonationstate: *mut super::super::super::Win32::Security::SE_IMPERSONATION_STATE) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsDisableImpersonation(thread : super::super::Foundation:: PETHREAD, impersonationstate : *mut super::super::super::Win32::Security:: SE_IMPERSONATION_STATE) -> super::super::super::Win32::Foundation:: BOOLEAN);
    PsDisableImpersonation(thread.param().abi(), impersonationstate)
}
#[inline]
pub unsafe fn PsGetProcessExitTime() -> i64 {
    windows_targets::link!("ntoskrnl.exe" "system" fn PsGetProcessExitTime() -> i64);
    PsGetProcessExitTime()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsGetThreadProcess<P0>(thread: P0) -> super::super::Foundation::PEPROCESS
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsGetThreadProcess(thread : super::super::Foundation:: PETHREAD) -> super::super::Foundation:: PEPROCESS);
    PsGetThreadProcess(thread.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn PsImpersonateClient<P0, P1, P2>(thread: P0, token: Option<*const core::ffi::c_void>, copyonopen: P1, effectiveonly: P2, impersonationlevel: super::super::super::Win32::Security::SECURITY_IMPERSONATION_LEVEL) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsImpersonateClient(thread : super::super::Foundation:: PETHREAD, token : *const core::ffi::c_void, copyonopen : super::super::super::Win32::Foundation:: BOOLEAN, effectiveonly : super::super::super::Win32::Foundation:: BOOLEAN, impersonationlevel : super::super::super::Win32::Security:: SECURITY_IMPERSONATION_LEVEL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    PsImpersonateClient(thread.param().abi(), core::mem::transmute(token.unwrap_or(std::ptr::null())), copyonopen.param().abi(), effectiveonly.param().abi(), impersonationlevel)
}
#[inline]
pub unsafe fn PsIsDiskCountersEnabled() -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn PsIsDiskCountersEnabled() -> super::super::super::Win32::Foundation:: BOOLEAN);
    PsIsDiskCountersEnabled()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsIsSystemThread<P0>(thread: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsIsSystemThread(thread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
    PsIsSystemThread(thread.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsIsThreadTerminating<P0>(thread: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsIsThreadTerminating(thread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
    PsIsThreadTerminating(thread.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsLookupProcessByProcessId<P0>(processid: P0, process: *mut super::super::Foundation::PEPROCESS) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsLookupProcessByProcessId(processid : super::super::super::Win32::Foundation:: HANDLE, process : *mut super::super::Foundation:: PEPROCESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    PsLookupProcessByProcessId(processid.param().abi(), process)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsLookupThreadByThreadId<P0>(threadid: P0, thread: *mut super::super::Foundation::PETHREAD) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsLookupThreadByThreadId(threadid : super::super::super::Win32::Foundation:: HANDLE, thread : *mut super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: NTSTATUS);
    PsLookupThreadByThreadId(threadid.param().abi(), thread)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn PsReferenceImpersonationToken<P0>(thread: P0, copyonopen: *mut super::super::super::Win32::Foundation::BOOLEAN, effectiveonly: *mut super::super::super::Win32::Foundation::BOOLEAN, impersonationlevel: *mut super::super::super::Win32::Security::SECURITY_IMPERSONATION_LEVEL) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsReferenceImpersonationToken(thread : super::super::Foundation:: PETHREAD, copyonopen : *mut super::super::super::Win32::Foundation:: BOOLEAN, effectiveonly : *mut super::super::super::Win32::Foundation:: BOOLEAN, impersonationlevel : *mut super::super::super::Win32::Security:: SECURITY_IMPERSONATION_LEVEL) -> *mut core::ffi::c_void);
    PsReferenceImpersonationToken(thread.param().abi(), copyonopen, effectiveonly, impersonationlevel)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsReferencePrimaryToken<P0>(process: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsReferencePrimaryToken(process : super::super::Foundation:: PEPROCESS) -> *mut core::ffi::c_void);
    PsReferencePrimaryToken(process.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn PsRestoreImpersonation<P0>(thread: P0, impersonationstate: *const super::super::super::Win32::Security::SE_IMPERSONATION_STATE)
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsRestoreImpersonation(thread : super::super::Foundation:: PETHREAD, impersonationstate : *const super::super::super::Win32::Security:: SE_IMPERSONATION_STATE));
    PsRestoreImpersonation(thread.param().abi(), impersonationstate)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsReturnPoolQuota<P0>(process: P0, pooltype: super::super::Foundation::POOL_TYPE, amount: usize)
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsReturnPoolQuota(process : super::super::Foundation:: PEPROCESS, pooltype : super::super::Foundation:: POOL_TYPE, amount : usize));
    PsReturnPoolQuota(process.param().abi(), pooltype, amount)
}
#[inline]
pub unsafe fn PsRevertToSelf() {
    windows_targets::link!("ntoskrnl.exe" "system" fn PsRevertToSelf());
    PsRevertToSelf()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn PsUpdateDiskCounters<P0>(process: P0, bytesread: u64, byteswritten: u64, readoperationcount: u32, writeoperationcount: u32, flushoperationcount: u32)
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn PsUpdateDiskCounters(process : super::super::Foundation:: PEPROCESS, bytesread : u64, byteswritten : u64, readoperationcount : u32, writeoperationcount : u32, flushoperationcount : u32));
    PsUpdateDiskCounters(process.param().abi(), bytesread, byteswritten, readoperationcount, writeoperationcount, flushoperationcount)
}
#[inline]
pub unsafe fn QuerySecurityContextToken(phcontext: *const SecHandle, token: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("secur32.dll" "system" fn QuerySecurityContextToken(phcontext : *const SecHandle, token : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    QuerySecurityContextToken(phcontext, token).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlAbsoluteToSelfRelativeSD<P0>(absolutesecuritydescriptor: P0, selfrelativesecuritydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, bufferlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, selfrelativesecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, bufferlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor.param().abi(), selfrelativesecuritydescriptor, bufferlength)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlAddAccessAllowedAce<P0>(acl: *mut super::super::super::Win32::Security::ACL, acerevision: u32, accessmask: u32, sid: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlAddAccessAllowedAce(acl : *mut super::super::super::Win32::Security:: ACL, acerevision : u32, accessmask : u32, sid : super::super::super::Win32::Security:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlAddAccessAllowedAce(acl, acerevision, accessmask, sid.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlAddAccessAllowedAceEx<P0>(acl: *mut super::super::super::Win32::Security::ACL, acerevision: u32, aceflags: u32, accessmask: u32, sid: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlAddAccessAllowedAceEx(acl : *mut super::super::super::Win32::Security:: ACL, acerevision : u32, aceflags : u32, accessmask : u32, sid : super::super::super::Win32::Security:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlAddAccessAllowedAceEx(acl, acerevision, aceflags, accessmask, sid.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlAddAce(acl: *mut super::super::super::Win32::Security::ACL, acerevision: u32, startingaceindex: u32, acelist: *const core::ffi::c_void, acelistlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlAddAce(acl : *mut super::super::super::Win32::Security:: ACL, acerevision : u32, startingaceindex : u32, acelist : *const core::ffi::c_void, acelistlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlAddAce(acl, acerevision, startingaceindex, acelist, acelistlength)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlAllocateAndInitializeSid(identifierauthority: *const super::super::super::Win32::Security::SID_IDENTIFIER_AUTHORITY, subauthoritycount: u8, subauthority0: u32, subauthority1: u32, subauthority2: u32, subauthority3: u32, subauthority4: u32, subauthority5: u32, subauthority6: u32, subauthority7: u32, sid: *mut super::super::super::Win32::Security::PSID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlAllocateAndInitializeSid(identifierauthority : *const super::super::super::Win32::Security:: SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, subauthority0 : u32, subauthority1 : u32, subauthority2 : u32, subauthority3 : u32, subauthority4 : u32, subauthority5 : u32, subauthority6 : u32, subauthority7 : u32, sid : *mut super::super::super::Win32::Security:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlAllocateAndInitializeSid(identifierauthority, subauthoritycount, subauthority0, subauthority1, subauthority2, subauthority3, subauthority4, subauthority5, subauthority6, subauthority7, sid)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlAllocateAndInitializeSidEx(identifierauthority: *const super::super::super::Win32::Security::SID_IDENTIFIER_AUTHORITY, subauthorities: &[u32], sid: *mut super::super::super::Win32::Security::PSID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlAllocateAndInitializeSidEx(identifierauthority : *const super::super::super::Win32::Security:: SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, subauthorities : *const u32, sid : *mut super::super::super::Win32::Security:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlAllocateAndInitializeSidEx(identifierauthority, subauthorities.len().try_into().unwrap(), core::mem::transmute(subauthorities.as_ptr()), sid)
}
#[inline]
pub unsafe fn RtlAllocateHeap(heaphandle: *const core::ffi::c_void, flags: u32, size: usize) -> *mut core::ffi::c_void {
    windows_targets::link!("ntdll.dll" "system" fn RtlAllocateHeap(heaphandle : *const core::ffi::c_void, flags : u32, size : usize) -> *mut core::ffi::c_void);
    RtlAllocateHeap(heaphandle, flags, size)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlAppendStringToString(destination: *mut super::super::super::Win32::System::Kernel::STRING, source: *const super::super::super::Win32::System::Kernel::STRING) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlAppendStringToString(destination : *mut super::super::super::Win32::System::Kernel:: STRING, source : *const super::super::super::Win32::System::Kernel:: STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlAppendStringToString(destination, source)
}
#[inline]
pub unsafe fn RtlCompareAltitudes(altitude1: *const super::super::super::Win32::Foundation::UNICODE_STRING, altitude2: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> i32 {
    windows_targets::link!("ntdll.dll" "system" fn RtlCompareAltitudes(altitude1 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, altitude2 : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> i32);
    RtlCompareAltitudes(altitude1, altitude2)
}
#[inline]
pub unsafe fn RtlCompareMemoryUlong(source: *const core::ffi::c_void, length: usize, pattern: u32) -> usize {
    windows_targets::link!("ntdll.dll" "system" fn RtlCompareMemoryUlong(source : *const core::ffi::c_void, length : usize, pattern : u32) -> usize);
    RtlCompareMemoryUlong(source, length, pattern)
}
#[inline]
pub unsafe fn RtlCompressBuffer(compressionformatandengine: u16, uncompressedbuffer: &[u8], compressedbuffer: &mut [u8], uncompressedchunksize: u32, finalcompressedsize: *mut u32, workspace: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlCompressBuffer(compressionformatandengine : u16, uncompressedbuffer : *const u8, uncompressedbuffersize : u32, compressedbuffer : *mut u8, compressedbuffersize : u32, uncompressedchunksize : u32, finalcompressedsize : *mut u32, workspace : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlCompressBuffer(compressionformatandengine, core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), uncompressedchunksize, finalcompressedsize, workspace)
}
#[inline]
pub unsafe fn RtlCompressChunks(uncompressedbuffer: &[u8], compressedbuffer: &mut [u8], compresseddatainfo: *mut COMPRESSED_DATA_INFO, compresseddatainfolength: u32, workspace: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlCompressChunks(uncompressedbuffer : *const u8, uncompressedbuffersize : u32, compressedbuffer : *mut u8, compressedbuffersize : u32, compresseddatainfo : *mut COMPRESSED_DATA_INFO, compresseddatainfolength : u32, workspace : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlCompressChunks(core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), compresseddatainfo, compresseddatainfolength, workspace)
}
#[inline]
pub unsafe fn RtlCopyLuid(destinationluid: *mut super::super::super::Win32::Foundation::LUID, sourceluid: *const super::super::super::Win32::Foundation::LUID) {
    windows_targets::link!("ntdll.dll" "system" fn RtlCopyLuid(destinationluid : *mut super::super::super::Win32::Foundation:: LUID, sourceluid : *const super::super::super::Win32::Foundation:: LUID));
    RtlCopyLuid(destinationluid, sourceluid)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlCopySid<P0>(destinationsidlength: u32, destinationsid: super::super::super::Win32::Security::PSID, sourcesid: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlCopySid(destinationsidlength : u32, destinationsid : super::super::super::Win32::Security:: PSID, sourcesid : super::super::super::Win32::Security:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlCopySid(destinationsidlength, destinationsid, sourcesid.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlCreateAcl(acl: *mut super::super::super::Win32::Security::ACL, acllength: u32, aclrevision: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlCreateAcl(acl : *mut super::super::super::Win32::Security:: ACL, acllength : u32, aclrevision : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlCreateAcl(acl, acllength, aclrevision)
}
#[inline]
pub unsafe fn RtlCreateHeap(flags: u32, heapbase: Option<*const core::ffi::c_void>, reservesize: usize, commitsize: usize, lock: Option<*const core::ffi::c_void>, parameters: Option<*const RTL_HEAP_PARAMETERS>) -> *mut core::ffi::c_void {
    windows_targets::link!("ntdll.dll" "system" fn RtlCreateHeap(flags : u32, heapbase : *const core::ffi::c_void, reservesize : usize, commitsize : usize, lock : *const core::ffi::c_void, parameters : *const RTL_HEAP_PARAMETERS) -> *mut core::ffi::c_void);
    RtlCreateHeap(flags, core::mem::transmute(heapbase.unwrap_or(std::ptr::null())), reservesize, commitsize, core::mem::transmute(lock.unwrap_or(std::ptr::null())), core::mem::transmute(parameters.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlCreateServiceSid(servicename: *const super::super::super::Win32::Foundation::UNICODE_STRING, servicesid: super::super::super::Win32::Security::PSID, servicesidlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlCreateServiceSid(servicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, servicesid : super::super::super::Win32::Security:: PSID, servicesidlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlCreateServiceSid(servicename, servicesid, servicesidlength)
}
#[inline]
pub unsafe fn RtlCreateSystemVolumeInformationFolder(volumerootpath: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlCreateSystemVolumeInformationFolder(volumerootpath : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlCreateSystemVolumeInformationFolder(volumerootpath)
}
#[inline]
pub unsafe fn RtlCreateUnicodeString<P0>(destinationstring: *mut super::super::super::Win32::Foundation::UNICODE_STRING, sourcestring: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlCreateUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlCreateUnicodeString(destinationstring, sourcestring.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlCreateVirtualAccountSid(name: *const super::super::super::Win32::Foundation::UNICODE_STRING, basesubauthority: u32, sid: super::super::super::Win32::Security::PSID, sidlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlCreateVirtualAccountSid(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, basesubauthority : u32, sid : super::super::super::Win32::Security:: PSID, sidlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlCreateVirtualAccountSid(name, basesubauthority, sid, sidlength)
}
#[inline]
pub unsafe fn RtlCustomCPToUnicodeN(customcp: *const CPTABLEINFO, unicodestring: windows_core::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: Option<*mut u32>, customcpstring: &[u8]) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlCustomCPToUnicodeN(customcp : *const CPTABLEINFO, unicodestring : windows_core::PWSTR, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, customcpstring : windows_core::PCSTR, bytesincustomcpstring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlCustomCPToUnicodeN(customcp, core::mem::transmute(unicodestring), maxbytesinunicodestring, core::mem::transmute(bytesinunicodestring.unwrap_or(std::ptr::null_mut())), core::mem::transmute(customcpstring.as_ptr()), customcpstring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn RtlDecompressBuffer(compressionformat: u16, uncompressedbuffer: &mut [u8], compressedbuffer: &[u8], finaluncompressedsize: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlDecompressBuffer(compressionformat : u16, uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, finaluncompressedsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDecompressBuffer(compressionformat, core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), finaluncompressedsize)
}
#[inline]
pub unsafe fn RtlDecompressBufferEx(compressionformat: u16, uncompressedbuffer: &mut [u8], compressedbuffer: &[u8], finaluncompressedsize: *mut u32, workspace: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlDecompressBufferEx(compressionformat : u16, uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, finaluncompressedsize : *mut u32, workspace : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDecompressBufferEx(compressionformat, core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), finaluncompressedsize, core::mem::transmute(workspace.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn RtlDecompressBufferEx2(compressionformat: u16, uncompressedbuffer: &mut [u8], compressedbuffer: &[u8], uncompressedchunksize: u32, finaluncompressedsize: *mut u32, workspace: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlDecompressBufferEx2(compressionformat : u16, uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, uncompressedchunksize : u32, finaluncompressedsize : *mut u32, workspace : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDecompressBufferEx2(compressionformat, core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), uncompressedchunksize, finaluncompressedsize, core::mem::transmute(workspace.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn RtlDecompressChunks(uncompressedbuffer: &mut [u8], compressedbuffer: &[u8], compressedtail: &[u8], compresseddatainfo: *const COMPRESSED_DATA_INFO) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlDecompressChunks(uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, compressedtail : *const u8, compressedtailsize : u32, compresseddatainfo : *const COMPRESSED_DATA_INFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDecompressChunks(core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedtail.as_ptr()), compressedtail.len().try_into().unwrap(), compresseddatainfo)
}
#[inline]
pub unsafe fn RtlDecompressFragment(compressionformat: u16, uncompressedfragment: &mut [u8], compressedbuffer: &[u8], fragmentoffset: u32, finaluncompressedsize: *mut u32, workspace: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlDecompressFragment(compressionformat : u16, uncompressedfragment : *mut u8, uncompressedfragmentsize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, fragmentoffset : u32, finaluncompressedsize : *mut u32, workspace : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDecompressFragment(compressionformat, core::mem::transmute(uncompressedfragment.as_ptr()), uncompressedfragment.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), fragmentoffset, finaluncompressedsize, workspace)
}
#[inline]
pub unsafe fn RtlDecompressFragmentEx(compressionformat: u16, uncompressedfragment: &mut [u8], compressedbuffer: &[u8], fragmentoffset: u32, uncompressedchunksize: u32, finaluncompressedsize: *mut u32, workspace: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlDecompressFragmentEx(compressionformat : u16, uncompressedfragment : *mut u8, uncompressedfragmentsize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, fragmentoffset : u32, uncompressedchunksize : u32, finaluncompressedsize : *mut u32, workspace : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDecompressFragmentEx(compressionformat, core::mem::transmute(uncompressedfragment.as_ptr()), uncompressedfragment.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), fragmentoffset, uncompressedchunksize, finaluncompressedsize, workspace)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlDeleteAce(acl: *mut super::super::super::Win32::Security::ACL, aceindex: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlDeleteAce(acl : *mut super::super::super::Win32::Security:: ACL, aceindex : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDeleteAce(acl, aceindex)
}
#[inline]
pub unsafe fn RtlDescribeChunk(compressionformat: u16, compressedbuffer: *mut *mut u8, endofcompressedbufferplus1: *const u8, chunkbuffer: *mut *mut u8, chunksize: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlDescribeChunk(compressionformat : u16, compressedbuffer : *mut *mut u8, endofcompressedbufferplus1 : *const u8, chunkbuffer : *mut *mut u8, chunksize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDescribeChunk(compressionformat, compressedbuffer, endofcompressedbufferplus1, chunkbuffer, chunksize)
}
#[inline]
pub unsafe fn RtlDestroyHeap(heaphandle: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    windows_targets::link!("ntdll.dll" "system" fn RtlDestroyHeap(heaphandle : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    RtlDestroyHeap(heaphandle)
}
#[inline]
pub unsafe fn RtlDowncaseUnicodeString<P0>(destinationstring: *mut super::super::super::Win32::Foundation::UNICODE_STRING, sourcestring: *const super::super::super::Win32::Foundation::UNICODE_STRING, allocatedestinationstring: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlDowncaseUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDowncaseUnicodeString(destinationstring, sourcestring, allocatedestinationstring.param().abi())
}
#[inline]
pub unsafe fn RtlDuplicateUnicodeString(flags: u32, stringin: *const super::super::super::Win32::Foundation::UNICODE_STRING, stringout: *mut super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlDuplicateUnicodeString(flags : u32, stringin : *const super::super::super::Win32::Foundation:: UNICODE_STRING, stringout : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlDuplicateUnicodeString(flags, stringin, stringout)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlEqualPrefixSid<P0, P1>(sid1: P0, sid2: P1) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlEqualPrefixSid(sid1 : super::super::super::Win32::Security:: PSID, sid2 : super::super::super::Win32::Security:: PSID) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlEqualPrefixSid(sid1.param().abi(), sid2.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlEqualSid<P0, P1>(sid1: P0, sid2: P1) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlEqualSid(sid1 : super::super::super::Win32::Security:: PSID, sid2 : super::super::super::Win32::Security:: PSID) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlEqualSid(sid1.param().abi(), sid2.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn RtlFindUnicodePrefix(prefixtable: *const UNICODE_PREFIX_TABLE, fullname: *const super::super::super::Win32::Foundation::UNICODE_STRING, caseinsensitiveindex: u32) -> *mut UNICODE_PREFIX_TABLE_ENTRY {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlFindUnicodePrefix(prefixtable : *const UNICODE_PREFIX_TABLE, fullname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, caseinsensitiveindex : u32) -> *mut UNICODE_PREFIX_TABLE_ENTRY);
    RtlFindUnicodePrefix(prefixtable, fullname, caseinsensitiveindex)
}
#[inline]
pub unsafe fn RtlFreeHeap(heaphandle: *const core::ffi::c_void, flags: u32, baseaddress: Option<*const core::ffi::c_void>) -> u32 {
    windows_targets::link!("ntdll.dll" "system" fn RtlFreeHeap(heaphandle : *const core::ffi::c_void, flags : u32, baseaddress : *const core::ffi::c_void) -> u32);
    RtlFreeHeap(heaphandle, flags, core::mem::transmute(baseaddress.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlFreeSid<P0>(sid: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlFreeSid(sid : super::super::super::Win32::Security:: PSID) -> *mut core::ffi::c_void);
    RtlFreeSid(sid.param().abi())
}
#[inline]
pub unsafe fn RtlGenerate8dot3Name<P0>(name: *const super::super::super::Win32::Foundation::UNICODE_STRING, allowextendedcharacters: P0, context: *mut GENERATE_NAME_CONTEXT, name8dot3: *mut super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlGenerate8dot3Name(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allowextendedcharacters : super::super::super::Win32::Foundation:: BOOLEAN, context : *mut GENERATE_NAME_CONTEXT, name8dot3 : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlGenerate8dot3Name(name, allowextendedcharacters.param().abi(), context, name8dot3)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlGetAce(acl: *const super::super::super::Win32::Security::ACL, aceindex: u32, ace: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlGetAce(acl : *const super::super::super::Win32::Security:: ACL, aceindex : u32, ace : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlGetAce(acl, aceindex, ace)
}
#[inline]
pub unsafe fn RtlGetCompressionWorkSpaceSize(compressionformatandengine: u16, compressbufferworkspacesize: *mut u32, compressfragmentworkspacesize: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlGetCompressionWorkSpaceSize(compressionformatandengine : u16, compressbufferworkspacesize : *mut u32, compressfragmentworkspacesize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlGetCompressionWorkSpaceSize(compressionformatandengine, compressbufferworkspacesize, compressfragmentworkspacesize)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlGetDaclSecurityDescriptor<P0>(securitydescriptor: P0, daclpresent: *mut super::super::super::Win32::Foundation::BOOLEAN, dacl: *mut *mut super::super::super::Win32::Security::ACL, dacldefaulted: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlGetDaclSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, daclpresent : *mut super::super::super::Win32::Foundation:: BOOLEAN, dacl : *mut *mut super::super::super::Win32::Security:: ACL, dacldefaulted : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlGetDaclSecurityDescriptor(securitydescriptor.param().abi(), daclpresent, dacl, dacldefaulted)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlGetGroupSecurityDescriptor<P0>(securitydescriptor: P0, group: *mut super::super::super::Win32::Security::PSID, groupdefaulted: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlGetGroupSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, group : *mut super::super::super::Win32::Security:: PSID, groupdefaulted : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlGetGroupSecurityDescriptor(securitydescriptor.param().abi(), group, groupdefaulted)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlGetOwnerSecurityDescriptor<P0>(securitydescriptor: P0, owner: *mut super::super::super::Win32::Security::PSID, ownerdefaulted: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlGetOwnerSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, owner : *mut super::super::super::Win32::Security:: PSID, ownerdefaulted : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlGetOwnerSecurityDescriptor(securitydescriptor.param().abi(), owner, ownerdefaulted)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlGetSaclSecurityDescriptor<P0>(securitydescriptor: P0, saclpresent: *mut super::super::super::Win32::Foundation::BOOLEAN, sacl: *mut *mut super::super::super::Win32::Security::ACL, sacldefaulted: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlGetSaclSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, saclpresent : *mut super::super::super::Win32::Foundation:: BOOLEAN, sacl : *mut *mut super::super::super::Win32::Security:: ACL, sacldefaulted : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlGetSaclSecurityDescriptor(securitydescriptor.param().abi(), saclpresent, sacl, sacldefaulted)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlIdentifierAuthoritySid<P0>(sid: P0) -> *mut super::super::super::Win32::Security::SID_IDENTIFIER_AUTHORITY
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlIdentifierAuthoritySid(sid : super::super::super::Win32::Security:: PSID) -> *mut super::super::super::Win32::Security:: SID_IDENTIFIER_AUTHORITY);
    RtlIdentifierAuthoritySid(sid.param().abi())
}
#[inline]
pub unsafe fn RtlIdnToAscii<P0>(flags: u32, sourcestring: P0, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: *mut i32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlIdnToAscii(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlIdnToAscii(flags, sourcestring.param().abi(), sourcestringlength, core::mem::transmute(destinationstring), destinationstringlength)
}
#[inline]
pub unsafe fn RtlIdnToNameprepUnicode<P0>(flags: u32, sourcestring: P0, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: *mut i32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlIdnToNameprepUnicode(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlIdnToNameprepUnicode(flags, sourcestring.param().abi(), sourcestringlength, core::mem::transmute(destinationstring), destinationstringlength)
}
#[inline]
pub unsafe fn RtlIdnToUnicode<P0>(flags: u32, sourcestring: P0, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: *mut i32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlIdnToUnicode(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlIdnToUnicode(flags, sourcestring.param().abi(), sourcestringlength, core::mem::transmute(destinationstring), destinationstringlength)
}
#[inline]
pub unsafe fn RtlInitCodePageTable(tablebase: Option<&[u16; 2]>, codepagetable: *mut CPTABLEINFO) {
    windows_targets::link!("ntdll.dll" "system" fn RtlInitCodePageTable(tablebase : *const u16, codepagetable : *mut CPTABLEINFO));
    RtlInitCodePageTable(core::mem::transmute(tablebase.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), codepagetable)
}
#[inline]
pub unsafe fn RtlInitUnicodeStringEx<P0>(destinationstring: *mut super::super::super::Win32::Foundation::UNICODE_STRING, sourcestring: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlInitUnicodeStringEx(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlInitUnicodeStringEx(destinationstring, sourcestring.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlInitializeSid(sid: super::super::super::Win32::Security::PSID, identifierauthority: *const super::super::super::Win32::Security::SID_IDENTIFIER_AUTHORITY, subauthoritycount: u8) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlInitializeSid(sid : super::super::super::Win32::Security:: PSID, identifierauthority : *const super::super::super::Win32::Security:: SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlInitializeSid(sid, identifierauthority, subauthoritycount)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlInitializeSidEx(sid: super::super::super::Win32::Security::PSID, identifierauthority: *const super::super::super::Win32::Security::SID_IDENTIFIER_AUTHORITY, subauthoritycount: u8) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "cdecl" fn RtlInitializeSidEx(sid : super::super::super::Win32::Security:: PSID, identifierauthority : *const super::super::super::Win32::Security:: SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlInitializeSidEx(sid, identifierauthority, subauthoritycount)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn RtlInitializeUnicodePrefix() -> UNICODE_PREFIX_TABLE {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlInitializeUnicodePrefix(prefixtable : *mut UNICODE_PREFIX_TABLE));
    let mut result__ = core::mem::zeroed();
    RtlInitializeUnicodePrefix(&mut result__);
    result__
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn RtlInsertUnicodePrefix(prefixtable: *const UNICODE_PREFIX_TABLE, prefix: *const super::super::super::Win32::Foundation::UNICODE_STRING, prefixtableentry: *mut UNICODE_PREFIX_TABLE_ENTRY) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlInsertUnicodePrefix(prefixtable : *const UNICODE_PREFIX_TABLE, prefix : *const super::super::super::Win32::Foundation:: UNICODE_STRING, prefixtableentry : *mut UNICODE_PREFIX_TABLE_ENTRY) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlInsertUnicodePrefix(prefixtable, prefix, prefixtableentry)
}
#[inline]
pub unsafe fn RtlIsCloudFilesPlaceholder(fileattributes: u32, reparsetag: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntdll.dll" "system" fn RtlIsCloudFilesPlaceholder(fileattributes : u32, reparsetag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlIsCloudFilesPlaceholder(fileattributes, reparsetag)
}
#[inline]
pub unsafe fn RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntdll.dll" "system" fn RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag)
}
#[inline]
pub unsafe fn RtlIsNormalizedString<P0>(normform: u32, sourcestring: P0, sourcestringlength: i32, normalized: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlIsNormalizedString(normform : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, normalized : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlIsNormalizedString(normform, sourcestring.param().abi(), sourcestringlength, normalized)
}
#[inline]
pub unsafe fn RtlIsPartialPlaceholder(fileattributes: u32, reparsetag: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholder(fileattributes : u32, reparsetag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlIsPartialPlaceholder(fileattributes, reparsetag)
}
#[inline]
pub unsafe fn RtlIsPartialPlaceholderFileHandle<P0>(filehandle: P0, ispartialplaceholder: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholderFileHandle(filehandle : super::super::super::Win32::Foundation:: HANDLE, ispartialplaceholder : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlIsPartialPlaceholderFileHandle(filehandle.param().abi(), ispartialplaceholder)
}
#[inline]
pub unsafe fn RtlIsPartialPlaceholderFileInfo(infobuffer: *const core::ffi::c_void, infoclass: FILE_INFORMATION_CLASS, ispartialplaceholder: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholderFileInfo(infobuffer : *const core::ffi::c_void, infoclass : FILE_INFORMATION_CLASS, ispartialplaceholder : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlIsPartialPlaceholderFileInfo(infobuffer, infoclass, ispartialplaceholder)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RtlIsSandboxedToken(context: Option<*const super::super::Foundation::SECURITY_SUBJECT_CONTEXT>, previousmode: i8) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlIsSandboxedToken(context : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, previousmode : i8) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlIsSandboxedToken(core::mem::transmute(context.unwrap_or(std::ptr::null())), previousmode)
}
#[inline]
pub unsafe fn RtlIsValidOemCharacter(char: windows_core::PWSTR) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlIsValidOemCharacter(char : windows_core::PWSTR) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlIsValidOemCharacter(core::mem::transmute(char))
}
#[inline]
pub unsafe fn RtlLengthRequiredSid(subauthoritycount: u32) -> u32 {
    windows_targets::link!("ntdll.dll" "system" fn RtlLengthRequiredSid(subauthoritycount : u32) -> u32);
    RtlLengthRequiredSid(subauthoritycount)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlLengthSid<P0>(sid: P0) -> u32
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlLengthSid(sid : super::super::super::Win32::Security:: PSID) -> u32);
    RtlLengthSid(sid.param().abi())
}
#[inline]
pub unsafe fn RtlMultiByteToUnicodeN(unicodestring: windows_core::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: Option<*mut u32>, multibytestring: &[u8]) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlMultiByteToUnicodeN(unicodestring : windows_core::PWSTR, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, multibytestring : windows_core::PCSTR, bytesinmultibytestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlMultiByteToUnicodeN(core::mem::transmute(unicodestring), maxbytesinunicodestring, core::mem::transmute(bytesinunicodestring.unwrap_or(std::ptr::null_mut())), core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn RtlMultiByteToUnicodeSize(bytesinunicodestring: *mut u32, multibytestring: &[u8]) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlMultiByteToUnicodeSize(bytesinunicodestring : *mut u32, multibytestring : windows_core::PCSTR, bytesinmultibytestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlMultiByteToUnicodeSize(bytesinunicodestring, core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn RtlNextUnicodePrefix<P0>(prefixtable: *const UNICODE_PREFIX_TABLE, restart: P0) -> *mut UNICODE_PREFIX_TABLE_ENTRY
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlNextUnicodePrefix(prefixtable : *const UNICODE_PREFIX_TABLE, restart : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut UNICODE_PREFIX_TABLE_ENTRY);
    RtlNextUnicodePrefix(prefixtable, restart.param().abi())
}
#[inline]
pub unsafe fn RtlNormalizeString<P0>(normform: u32, sourcestring: P0, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: *mut i32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlNormalizeString(normform : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlNormalizeString(normform, sourcestring.param().abi(), sourcestringlength, core::mem::transmute(destinationstring), destinationstringlength)
}
#[inline]
pub unsafe fn RtlNtStatusToDosErrorNoTeb<P0>(status: P0) -> u32
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlNtStatusToDosErrorNoTeb(status : super::super::super::Win32::Foundation:: NTSTATUS) -> u32);
    RtlNtStatusToDosErrorNoTeb(status.param().abi())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlOemStringToCountedUnicodeString<P0>(destinationstring: *mut super::super::super::Win32::Foundation::UNICODE_STRING, sourcestring: *const super::super::super::Win32::System::Kernel::STRING, allocatedestinationstring: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlOemStringToCountedUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::System::Kernel:: STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlOemStringToCountedUnicodeString(destinationstring, sourcestring, allocatedestinationstring.param().abi())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlOemStringToUnicodeString<P0>(destinationstring: *mut super::super::super::Win32::Foundation::UNICODE_STRING, sourcestring: *const super::super::super::Win32::System::Kernel::STRING, allocatedestinationstring: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlOemStringToUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::System::Kernel:: STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlOemStringToUnicodeString(destinationstring, sourcestring, allocatedestinationstring.param().abi())
}
#[inline]
pub unsafe fn RtlOemToUnicodeN(unicodestring: windows_core::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: Option<*mut u32>, oemstring: &[u8]) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlOemToUnicodeN(unicodestring : windows_core::PWSTR, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, oemstring : windows_core::PCSTR, bytesinoemstring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlOemToUnicodeN(core::mem::transmute(unicodestring), maxbytesinunicodestring, core::mem::transmute(bytesinunicodestring.unwrap_or(std::ptr::null_mut())), core::mem::transmute(oemstring.as_ptr()), oemstring.len().try_into().unwrap())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlPrefixString<P0>(string1: *const super::super::super::Win32::System::Kernel::STRING, string2: *const super::super::super::Win32::System::Kernel::STRING, caseinsensitive: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlPrefixString(string1 : *const super::super::super::Win32::System::Kernel:: STRING, string2 : *const super::super::super::Win32::System::Kernel:: STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlPrefixString(string1, string2, caseinsensitive.param().abi())
}
#[inline]
pub unsafe fn RtlQueryPackageIdentity(tokenobject: *const core::ffi::c_void, packagefullname: windows_core::PWSTR, packagesize: *mut usize, appid: windows_core::PWSTR, appidsize: Option<*mut usize>, packaged: Option<*mut super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlQueryPackageIdentity(tokenobject : *const core::ffi::c_void, packagefullname : windows_core::PWSTR, packagesize : *mut usize, appid : windows_core::PWSTR, appidsize : *mut usize, packaged : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlQueryPackageIdentity(tokenobject, core::mem::transmute(packagefullname), packagesize, core::mem::transmute(appid), core::mem::transmute(appidsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(packaged.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RtlQueryPackageIdentityEx(tokenobject: *const core::ffi::c_void, packagefullname: windows_core::PWSTR, packagesize: *mut usize, appid: windows_core::PWSTR, appidsize: Option<*mut usize>, dynamicid: Option<*mut windows_core::GUID>, flags: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlQueryPackageIdentityEx(tokenobject : *const core::ffi::c_void, packagefullname : windows_core::PWSTR, packagesize : *mut usize, appid : windows_core::PWSTR, appidsize : *mut usize, dynamicid : *mut windows_core::GUID, flags : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlQueryPackageIdentityEx(tokenobject, core::mem::transmute(packagefullname), packagesize, core::mem::transmute(appid), core::mem::transmute(appidsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(dynamicid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(flags.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RtlQueryProcessPlaceholderCompatibilityMode() -> i8 {
    windows_targets::link!("ntdll.dll" "system" fn RtlQueryProcessPlaceholderCompatibilityMode() -> i8);
    RtlQueryProcessPlaceholderCompatibilityMode()
}
#[inline]
pub unsafe fn RtlQueryThreadPlaceholderCompatibilityMode() -> i8 {
    windows_targets::link!("ntdll.dll" "system" fn RtlQueryThreadPlaceholderCompatibilityMode() -> i8);
    RtlQueryThreadPlaceholderCompatibilityMode()
}
#[inline]
pub unsafe fn RtlRandom(seed: *mut u32) -> u32 {
    windows_targets::link!("ntdll.dll" "system" fn RtlRandom(seed : *mut u32) -> u32);
    RtlRandom(seed)
}
#[inline]
pub unsafe fn RtlRandomEx(seed: *mut u32) -> u32 {
    windows_targets::link!("ntdll.dll" "system" fn RtlRandomEx(seed : *mut u32) -> u32);
    RtlRandomEx(seed)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn RtlRemoveUnicodePrefix(prefixtable: *const UNICODE_PREFIX_TABLE, prefixtableentry: *const UNICODE_PREFIX_TABLE_ENTRY) {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlRemoveUnicodePrefix(prefixtable : *const UNICODE_PREFIX_TABLE, prefixtableentry : *const UNICODE_PREFIX_TABLE_ENTRY));
    RtlRemoveUnicodePrefix(prefixtable, prefixtableentry)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlReplaceSidInSd<P0, P1>(securitydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, oldsid: P0, newsid: P1, numchanges: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlReplaceSidInSd(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, oldsid : super::super::super::Win32::Security:: PSID, newsid : super::super::super::Win32::Security:: PSID, numchanges : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlReplaceSidInSd(securitydescriptor, oldsid.param().abi(), newsid.param().abi(), numchanges)
}
#[inline]
pub unsafe fn RtlReserveChunk(compressionformat: u16, compressedbuffer: *mut *mut u8, endofcompressedbufferplus1: *const u8, chunkbuffer: *mut *mut u8, chunksize: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn RtlReserveChunk(compressionformat : u16, compressedbuffer : *mut *mut u8, endofcompressedbufferplus1 : *const u8, chunkbuffer : *mut *mut u8, chunksize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlReserveChunk(compressionformat, compressedbuffer, endofcompressedbufferplus1, chunkbuffer, chunksize)
}
#[inline]
pub unsafe fn RtlSecondsSince1970ToTime(elapsedseconds: u32) -> i64 {
    windows_targets::link!("ntdll.dll" "system" fn RtlSecondsSince1970ToTime(elapsedseconds : u32, time : *mut i64));
    let mut result__ = core::mem::zeroed();
    RtlSecondsSince1970ToTime(elapsedseconds, &mut result__);
    result__
}
#[inline]
pub unsafe fn RtlSecondsSince1980ToTime(elapsedseconds: u32) -> i64 {
    windows_targets::link!("ntdll.dll" "system" fn RtlSecondsSince1980ToTime(elapsedseconds : u32, time : *mut i64));
    let mut result__ = core::mem::zeroed();
    RtlSecondsSince1980ToTime(elapsedseconds, &mut result__);
    result__
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlSelfRelativeToAbsoluteSD<P0>(selfrelativesecuritydescriptor: P0, absolutesecuritydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, absolutesecuritydescriptorsize: *mut u32, dacl: Option<*mut super::super::super::Win32::Security::ACL>, daclsize: *mut u32, sacl: Option<*mut super::super::super::Win32::Security::ACL>, saclsize: *mut u32, owner: super::super::super::Win32::Security::PSID, ownersize: *mut u32, primarygroup: super::super::super::Win32::Security::PSID, primarygroupsize: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, absolutesecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, absolutesecuritydescriptorsize : *mut u32, dacl : *mut super::super::super::Win32::Security:: ACL, daclsize : *mut u32, sacl : *mut super::super::super::Win32::Security:: ACL, saclsize : *mut u32, owner : super::super::super::Win32::Security:: PSID, ownersize : *mut u32, primarygroup : super::super::super::Win32::Security:: PSID, primarygroupsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor.param().abi(), absolutesecuritydescriptor, absolutesecuritydescriptorsize, core::mem::transmute(dacl.unwrap_or(std::ptr::null_mut())), daclsize, core::mem::transmute(sacl.unwrap_or(std::ptr::null_mut())), saclsize, owner, ownersize, primarygroup, primarygroupsize)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlSetGroupSecurityDescriptor<P0, P1>(securitydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, group: P0, groupdefaulted: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlSetGroupSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, group : super::super::super::Win32::Security:: PSID, groupdefaulted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlSetGroupSecurityDescriptor(securitydescriptor, group.param().abi(), groupdefaulted.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlSetOwnerSecurityDescriptor<P0, P1>(securitydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, owner: P0, ownerdefaulted: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlSetOwnerSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, owner : super::super::super::Win32::Security:: PSID, ownerdefaulted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlSetOwnerSecurityDescriptor(securitydescriptor, owner.param().abi(), ownerdefaulted.param().abi())
}
#[inline]
pub unsafe fn RtlSetProcessPlaceholderCompatibilityMode(mode: i8) -> i8 {
    windows_targets::link!("ntdll.dll" "system" fn RtlSetProcessPlaceholderCompatibilityMode(mode : i8) -> i8);
    RtlSetProcessPlaceholderCompatibilityMode(mode)
}
#[inline]
pub unsafe fn RtlSetThreadPlaceholderCompatibilityMode(mode: i8) -> i8 {
    windows_targets::link!("ntdll.dll" "system" fn RtlSetThreadPlaceholderCompatibilityMode(mode : i8) -> i8);
    RtlSetThreadPlaceholderCompatibilityMode(mode)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlSubAuthorityCountSid<P0>(sid: P0) -> *mut u8
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlSubAuthorityCountSid(sid : super::super::super::Win32::Security:: PSID) -> *mut u8);
    RtlSubAuthorityCountSid(sid.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlSubAuthoritySid<P0>(sid: P0, subauthority: u32) -> *mut u32
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlSubAuthoritySid(sid : super::super::super::Win32::Security:: PSID, subauthority : u32) -> *mut u32);
    RtlSubAuthoritySid(sid.param().abi(), subauthority)
}
#[inline]
pub unsafe fn RtlTimeToSecondsSince1980(time: *const i64, elapsedseconds: *mut u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntdll.dll" "system" fn RtlTimeToSecondsSince1980(time : *const i64, elapsedseconds : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlTimeToSecondsSince1980(time, elapsedseconds)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlUnicodeStringToCountedOemString<P0>(destinationstring: *mut super::super::super::Win32::System::Kernel::STRING, sourcestring: *const super::super::super::Win32::Foundation::UNICODE_STRING, allocatedestinationstring: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeStringToCountedOemString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUnicodeStringToCountedOemString(destinationstring, sourcestring, allocatedestinationstring.param().abi())
}
#[inline]
pub unsafe fn RtlUnicodeToCustomCPN<P0>(customcp: *const CPTABLEINFO, customcpstring: &mut [u8], bytesincustomcpstring: Option<*mut u32>, unicodestring: P0, bytesinunicodestring: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeToCustomCPN(customcp : *const CPTABLEINFO, customcpstring : windows_core::PSTR, maxbytesincustomcpstring : u32, bytesincustomcpstring : *mut u32, unicodestring : windows_core::PCWSTR, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUnicodeToCustomCPN(customcp, core::mem::transmute(customcpstring.as_ptr()), customcpstring.len().try_into().unwrap(), core::mem::transmute(bytesincustomcpstring.unwrap_or(std::ptr::null_mut())), unicodestring.param().abi(), bytesinunicodestring)
}
#[inline]
pub unsafe fn RtlUnicodeToMultiByteN(multibytestring: &mut [u8], bytesinmultibytestring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeToMultiByteN(multibytestring : windows_core::PSTR, maxbytesinmultibytestring : u32, bytesinmultibytestring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUnicodeToMultiByteN(core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap(), core::mem::transmute(bytesinmultibytestring.unwrap_or(std::ptr::null_mut())), unicodestring, bytesinunicodestring)
}
#[inline]
pub unsafe fn RtlUnicodeToOemN(oemstring: &mut [u8], bytesinoemstring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeToOemN(oemstring : windows_core::PSTR, maxbytesinoemstring : u32, bytesinoemstring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUnicodeToOemN(core::mem::transmute(oemstring.as_ptr()), oemstring.len().try_into().unwrap(), core::mem::transmute(bytesinoemstring.unwrap_or(std::ptr::null_mut())), unicodestring, bytesinunicodestring)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlUpcaseUnicodeStringToCountedOemString<P0>(destinationstring: *mut super::super::super::Win32::System::Kernel::STRING, sourcestring: *const super::super::super::Win32::Foundation::UNICODE_STRING, allocatedestinationstring: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeStringToCountedOemString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUpcaseUnicodeStringToCountedOemString(destinationstring, sourcestring, allocatedestinationstring.param().abi())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlUpcaseUnicodeStringToOemString<P0>(destinationstring: *mut super::super::super::Win32::System::Kernel::STRING, sourcestring: *const super::super::super::Win32::Foundation::UNICODE_STRING, allocatedestinationstring: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeStringToOemString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUpcaseUnicodeStringToOemString(destinationstring, sourcestring, allocatedestinationstring.param().abi())
}
#[inline]
pub unsafe fn RtlUpcaseUnicodeToCustomCPN<P0>(customcp: *const CPTABLEINFO, customcpstring: &mut [u8], bytesincustomcpstring: Option<*mut u32>, unicodestring: P0, bytesinunicodestring: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToCustomCPN(customcp : *const CPTABLEINFO, customcpstring : windows_core::PSTR, maxbytesincustomcpstring : u32, bytesincustomcpstring : *mut u32, unicodestring : windows_core::PCWSTR, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUpcaseUnicodeToCustomCPN(customcp, core::mem::transmute(customcpstring.as_ptr()), customcpstring.len().try_into().unwrap(), core::mem::transmute(bytesincustomcpstring.unwrap_or(std::ptr::null_mut())), unicodestring.param().abi(), bytesinunicodestring)
}
#[inline]
pub unsafe fn RtlUpcaseUnicodeToMultiByteN(multibytestring: &mut [u8], bytesinmultibytestring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToMultiByteN(multibytestring : windows_core::PSTR, maxbytesinmultibytestring : u32, bytesinmultibytestring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUpcaseUnicodeToMultiByteN(core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap(), core::mem::transmute(bytesinmultibytestring.unwrap_or(std::ptr::null_mut())), unicodestring, bytesinunicodestring)
}
#[inline]
pub unsafe fn RtlUpcaseUnicodeToOemN(oemstring: &mut [u8], bytesinoemstring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToOemN(oemstring : windows_core::PSTR, maxbytesinoemstring : u32, bytesinoemstring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlUpcaseUnicodeToOemN(core::mem::transmute(oemstring.as_ptr()), oemstring.len().try_into().unwrap(), core::mem::transmute(bytesinoemstring.unwrap_or(std::ptr::null_mut())), unicodestring, bytesinunicodestring)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RtlValidSid<P0>(sid: P0) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlValidSid(sid : super::super::super::Win32::Security:: PSID) -> super::super::super::Win32::Foundation:: BOOLEAN);
    RtlValidSid(sid.param().abi())
}
#[inline]
pub unsafe fn RtlValidateUnicodeString(flags: u32, string: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn RtlValidateUnicodeString(flags : u32, string : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    RtlValidateUnicodeString(flags, string)
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlxOemStringToUnicodeSize(oemstring: *const super::super::super::Win32::System::Kernel::STRING) -> u32 {
    windows_targets::link!("ntdll.dll" "system" fn RtlxOemStringToUnicodeSize(oemstring : *const super::super::super::Win32::System::Kernel:: STRING) -> u32);
    RtlxOemStringToUnicodeSize(oemstring)
}
#[inline]
pub unsafe fn RtlxUnicodeStringToOemSize(unicodestring: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> u32 {
    windows_targets::link!("ntdll.dll" "system" fn RtlxUnicodeStringToOemSize(unicodestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> u32);
    RtlxUnicodeStringToOemSize(unicodestring)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeAccessCheckFromState<P0>(securitydescriptor: P0, primarytokeninformation: *const super::super::super::Win32::Security::TOKEN_ACCESS_INFORMATION, clienttokeninformation: Option<*const super::super::super::Win32::Security::TOKEN_ACCESS_INFORMATION>, desiredaccess: u32, previouslygrantedaccess: u32, privileges: Option<*mut *mut super::super::super::Win32::Security::PRIVILEGE_SET>, genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING, accessmode: i8, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAccessCheckFromState(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, primarytokeninformation : *const super::super::super::Win32::Security:: TOKEN_ACCESS_INFORMATION, clienttokeninformation : *const super::super::super::Win32::Security:: TOKEN_ACCESS_INFORMATION, desiredaccess : u32, previouslygrantedaccess : u32, privileges : *mut *mut super::super::super::Win32::Security:: PRIVILEGE_SET, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, accessmode : i8, grantedaccess : *mut u32, accessstatus : *mut i32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAccessCheckFromState(securitydescriptor.param().abi(), primarytokeninformation, core::mem::transmute(clienttokeninformation.unwrap_or(std::ptr::null())), desiredaccess, previouslygrantedaccess, core::mem::transmute(privileges.unwrap_or(std::ptr::null_mut())), genericmapping, accessmode, grantedaccess, accessstatus)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeAccessCheckFromStateEx<P0>(securitydescriptor: P0, primarytoken: *const core::ffi::c_void, clienttoken: Option<*const core::ffi::c_void>, desiredaccess: u32, previouslygrantedaccess: u32, privileges: Option<*mut *mut super::super::super::Win32::Security::PRIVILEGE_SET>, genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING, accessmode: i8, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAccessCheckFromStateEx(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, primarytoken : *const core::ffi::c_void, clienttoken : *const core::ffi::c_void, desiredaccess : u32, previouslygrantedaccess : u32, privileges : *mut *mut super::super::super::Win32::Security:: PRIVILEGE_SET, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, accessmode : i8, grantedaccess : *mut u32, accessstatus : *mut i32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAccessCheckFromStateEx(securitydescriptor.param().abi(), primarytoken, core::mem::transmute(clienttoken.unwrap_or(std::ptr::null())), desiredaccess, previouslygrantedaccess, core::mem::transmute(privileges.unwrap_or(std::ptr::null_mut())), genericmapping, accessmode, grantedaccess, accessstatus)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAdjustAccessStateForAccessConstraints<P0>(objecttype: *const core::ffi::c_void, securitydescriptor: P0, accessstate: *mut super::super::Foundation::ACCESS_STATE)
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAdjustAccessStateForAccessConstraints(objecttype : *const core::ffi::c_void, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *mut super::super::Foundation:: ACCESS_STATE));
    SeAdjustAccessStateForAccessConstraints(objecttype, securitydescriptor.param().abi(), accessstate)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAdjustAccessStateForTrustLabel<P0>(objecttype: *const core::ffi::c_void, securitydescriptor: P0, accessstate: *mut super::super::Foundation::ACCESS_STATE)
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAdjustAccessStateForTrustLabel(objecttype : *const core::ffi::c_void, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *mut super::super::Foundation:: ACCESS_STATE));
    SeAdjustAccessStateForTrustLabel(objecttype, securitydescriptor.param().abi(), accessstate)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAdjustObjectSecurity<P0, P1>(objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING, originaldescriptor: P0, proposeddescriptor: P1, subjectsecuritycontext: *const super::super::Foundation::SECURITY_SUBJECT_CONTEXT, adjusteddescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, applyadjusteddescriptor: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAdjustObjectSecurity(objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, originaldescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, proposeddescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, adjusteddescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, applyadjusteddescriptor : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeAdjustObjectSecurity(objectname, originaldescriptor.param().abi(), proposeddescriptor.param().abi(), subjectsecuritycontext, adjusteddescriptor, applyadjusteddescriptor)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAppendPrivileges(accessstate: *mut super::super::Foundation::ACCESS_STATE, privileges: *const super::super::super::Win32::Security::PRIVILEGE_SET) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAppendPrivileges(accessstate : *mut super::super::Foundation:: ACCESS_STATE, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeAppendPrivileges(accessstate, privileges)
}
#[inline]
pub unsafe fn SeAuditFipsCryptoSelftests<P0>(bsuccess: P0, selftestcode: u32)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditFipsCryptoSelftests(bsuccess : super::super::super::Win32::Foundation:: BOOLEAN, selftestcode : u32));
    SeAuditFipsCryptoSelftests(bsuccess.param().abi(), selftestcode)
}
#[inline]
pub unsafe fn SeAuditHardLinkCreation<P0>(filename: *const super::super::super::Win32::Foundation::UNICODE_STRING, linkname: *const super::super::super::Win32::Foundation::UNICODE_STRING, bsuccess: P0)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditHardLinkCreation(filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, linkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, bsuccess : super::super::super::Win32::Foundation:: BOOLEAN));
    SeAuditHardLinkCreation(filename, linkname, bsuccess.param().abi())
}
#[inline]
pub unsafe fn SeAuditHardLinkCreationWithTransaction<P0>(filename: *const super::super::super::Win32::Foundation::UNICODE_STRING, linkname: *const super::super::super::Win32::Foundation::UNICODE_STRING, bsuccess: P0, transactionid: Option<*const windows_core::GUID>)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditHardLinkCreationWithTransaction(filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, linkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, bsuccess : super::super::super::Win32::Foundation:: BOOLEAN, transactionid : *const windows_core::GUID));
    SeAuditHardLinkCreationWithTransaction(filename, linkname, bsuccess.param().abi(), core::mem::transmute(transactionid.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn SeAuditTransactionStateChange(transactionid: *const windows_core::GUID, resourcemanagerid: *const windows_core::GUID, newtransactionstate: u32) {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditTransactionStateChange(transactionid : *const windows_core::GUID, resourcemanagerid : *const windows_core::GUID, newtransactionstate : u32));
    SeAuditTransactionStateChange(transactionid, resourcemanagerid, newtransactionstate)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAuditingAnyFileEventsWithContext<P0>(securitydescriptor: P0, subjectsecuritycontext: Option<*const super::super::Foundation::SECURITY_SUBJECT_CONTEXT>) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditingAnyFileEventsWithContext(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAuditingAnyFileEventsWithContext(securitydescriptor.param().abi(), core::mem::transmute(subjectsecuritycontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAuditingAnyFileEventsWithContextEx<P0>(securitydescriptor: P0, subjectsecuritycontext: Option<*const super::super::Foundation::SECURITY_SUBJECT_CONTEXT>, stagingenabled: Option<*mut super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditingAnyFileEventsWithContextEx(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, stagingenabled : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAuditingAnyFileEventsWithContextEx(securitydescriptor.param().abi(), core::mem::transmute(subjectsecuritycontext.unwrap_or(std::ptr::null())), core::mem::transmute(stagingenabled.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeAuditingFileEvents<P0, P1>(accessgranted: P0, securitydescriptor: P1) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditingFileEvents(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAuditingFileEvents(accessgranted.param().abi(), securitydescriptor.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAuditingFileEventsWithContext<P0, P1>(accessgranted: P0, securitydescriptor: P1, subjectsecuritycontext: Option<*const super::super::Foundation::SECURITY_SUBJECT_CONTEXT>) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditingFileEventsWithContext(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAuditingFileEventsWithContext(accessgranted.param().abi(), securitydescriptor.param().abi(), core::mem::transmute(subjectsecuritycontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAuditingFileEventsWithContextEx<P0, P1>(accessgranted: P0, securitydescriptor: P1, subjectsecuritycontext: Option<*const super::super::Foundation::SECURITY_SUBJECT_CONTEXT>, stagingenabled: Option<*mut super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditingFileEventsWithContextEx(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, stagingenabled : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAuditingFileEventsWithContextEx(accessgranted.param().abi(), securitydescriptor.param().abi(), core::mem::transmute(subjectsecuritycontext.unwrap_or(std::ptr::null())), core::mem::transmute(stagingenabled.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAuditingFileOrGlobalEvents<P0, P1>(accessgranted: P0, securitydescriptor: P1, subjectsecuritycontext: *const super::super::Foundation::SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditingFileOrGlobalEvents(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAuditingFileOrGlobalEvents(accessgranted.param().abi(), securitydescriptor.param().abi(), subjectsecuritycontext)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeAuditingHardLinkEvents<P0, P1>(accessgranted: P0, securitydescriptor: P1) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditingHardLinkEvents(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAuditingHardLinkEvents(accessgranted.param().abi(), securitydescriptor.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeAuditingHardLinkEventsWithContext<P0, P1>(accessgranted: P0, securitydescriptor: P1, subjectsecuritycontext: Option<*const super::super::Foundation::SECURITY_SUBJECT_CONTEXT>) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeAuditingHardLinkEventsWithContext(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeAuditingHardLinkEventsWithContext(accessgranted.param().abi(), securitydescriptor.param().abi(), core::mem::transmute(subjectsecuritycontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeCaptureSubjectContextEx<P0, P1>(thread: P0, process: P1) -> super::super::Foundation::SECURITY_SUBJECT_CONTEXT
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
    P1: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeCaptureSubjectContextEx(thread : super::super::Foundation:: PETHREAD, process : super::super::Foundation:: PEPROCESS, subjectcontext : *mut super::super::Foundation:: SECURITY_SUBJECT_CONTEXT));
    let mut result__ = core::mem::zeroed();
    SeCaptureSubjectContextEx(thread.param().abi(), process.param().abi(), &mut result__);
    result__
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeCheckForCriticalAceRemoval<P0, P1>(currentdescriptor: P0, newdescriptor: P1, subjectsecuritycontext: *const super::super::Foundation::SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeCheckForCriticalAceRemoval(currentdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, newdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, aceremoved : *mut super::super::super::Win32::Foundation:: BOOLEAN));
    let mut result__ = core::mem::zeroed();
    SeCheckForCriticalAceRemoval(currentdescriptor.param().abi(), newdescriptor.param().abi(), subjectsecuritycontext, &mut result__);
    result__
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeCreateClientSecurity<P0, P1>(clientthread: P0, clientsecurityqos: *const super::super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE, remotesession: P1, clientcontext: *mut SECURITY_CLIENT_CONTEXT) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeCreateClientSecurity(clientthread : super::super::Foundation:: PETHREAD, clientsecurityqos : *const super::super::super::Win32::Security:: SECURITY_QUALITY_OF_SERVICE, remotesession : super::super::super::Win32::Foundation:: BOOLEAN, clientcontext : *mut SECURITY_CLIENT_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeCreateClientSecurity(clientthread.param().abi(), clientsecurityqos, remotesession.param().abi(), clientcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeCreateClientSecurityFromSubjectContext<P0>(subjectcontext: *const super::super::Foundation::SECURITY_SUBJECT_CONTEXT, clientsecurityqos: *const super::super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE, serverisremote: P0, clientcontext: *mut SECURITY_CLIENT_CONTEXT) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeCreateClientSecurityFromSubjectContext(subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, clientsecurityqos : *const super::super::super::Win32::Security:: SECURITY_QUALITY_OF_SERVICE, serverisremote : super::super::super::Win32::Foundation:: BOOLEAN, clientcontext : *mut SECURITY_CLIENT_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeCreateClientSecurityFromSubjectContext(subjectcontext, clientsecurityqos, serverisremote.param().abi(), clientcontext)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeDeleteClientSecurity(clientcontext: *mut SECURITY_CLIENT_CONTEXT) {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeDeleteClientSecurity(clientcontext : *mut SECURITY_CLIENT_CONTEXT));
    SeDeleteClientSecurity(clientcontext)
}
#[inline]
pub unsafe fn SeDeleteObjectAuditAlarm<P0>(object: *const core::ffi::c_void, handle: P0)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeDeleteObjectAuditAlarm(object : *const core::ffi::c_void, handle : super::super::super::Win32::Foundation:: HANDLE));
    SeDeleteObjectAuditAlarm(object, handle.param().abi())
}
#[inline]
pub unsafe fn SeDeleteObjectAuditAlarmWithTransaction<P0>(object: *const core::ffi::c_void, handle: P0, transactionid: Option<*const windows_core::GUID>)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeDeleteObjectAuditAlarmWithTransaction(object : *const core::ffi::c_void, handle : super::super::super::Win32::Foundation:: HANDLE, transactionid : *const windows_core::GUID));
    SeDeleteObjectAuditAlarmWithTransaction(object, handle.param().abi(), core::mem::transmute(transactionid.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeExamineSacl<P0>(sacl: *const super::super::super::Win32::Security::ACL, resourcesacl: *const super::super::super::Win32::Security::ACL, token: *const core::ffi::c_void, desiredaccess: u32, accessgranted: P0, generateaudit: *mut super::super::super::Win32::Foundation::BOOLEAN, generatealarm: *mut super::super::super::Win32::Foundation::BOOLEAN)
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeExamineSacl(sacl : *const super::super::super::Win32::Security:: ACL, resourcesacl : *const super::super::super::Win32::Security:: ACL, token : *const core::ffi::c_void, desiredaccess : u32, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, generateaudit : *mut super::super::super::Win32::Foundation:: BOOLEAN, generatealarm : *mut super::super::super::Win32::Foundation:: BOOLEAN));
    SeExamineSacl(sacl, resourcesacl, token, desiredaccess, accessgranted.param().abi(), generateaudit, generatealarm)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeFilterToken(existingtoken: *const core::ffi::c_void, flags: u32, sidstodisable: Option<*const super::super::super::Win32::Security::TOKEN_GROUPS>, privilegestodelete: Option<*const super::super::super::Win32::Security::TOKEN_PRIVILEGES>, restrictedsids: Option<*const super::super::super::Win32::Security::TOKEN_GROUPS>, filteredtoken: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeFilterToken(existingtoken : *const core::ffi::c_void, flags : u32, sidstodisable : *const super::super::super::Win32::Security:: TOKEN_GROUPS, privilegestodelete : *const super::super::super::Win32::Security:: TOKEN_PRIVILEGES, restrictedsids : *const super::super::super::Win32::Security:: TOKEN_GROUPS, filteredtoken : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeFilterToken(existingtoken, flags, core::mem::transmute(sidstodisable.unwrap_or(std::ptr::null())), core::mem::transmute(privilegestodelete.unwrap_or(std::ptr::null())), core::mem::transmute(restrictedsids.unwrap_or(std::ptr::null())), filteredtoken)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeFreePrivileges(privileges: *const super::super::super::Win32::Security::PRIVILEGE_SET) {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeFreePrivileges(privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET));
    SeFreePrivileges(privileges)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeImpersonateClient<P0>(clientcontext: *const SECURITY_CLIENT_CONTEXT, serverthread: P0)
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeImpersonateClient(clientcontext : *const SECURITY_CLIENT_CONTEXT, serverthread : super::super::Foundation:: PETHREAD));
    SeImpersonateClient(clientcontext, serverthread.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeImpersonateClientEx<P0>(clientcontext: *const SECURITY_CLIENT_CONTEXT, serverthread: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeImpersonateClientEx(clientcontext : *const SECURITY_CLIENT_CONTEXT, serverthread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeImpersonateClientEx(clientcontext, serverthread.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SeLocateProcessImageName<P0>(process: P0, pimagefilename: *mut *mut super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PEPROCESS>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeLocateProcessImageName(process : super::super::Foundation:: PEPROCESS, pimagefilename : *mut *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeLocateProcessImageName(process.param().abi(), pimagefilename)
}
#[inline]
pub unsafe fn SeMarkLogonSessionForTerminationNotification(logonid: *const super::super::super::Win32::Foundation::LUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeMarkLogonSessionForTerminationNotification(logonid : *const super::super::super::Win32::Foundation:: LUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeMarkLogonSessionForTerminationNotification(logonid)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SeMarkLogonSessionForTerminationNotificationEx<P0>(logonid: *const super::super::super::Win32::Foundation::LUID, pserversilo: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::PESILO>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeMarkLogonSessionForTerminationNotificationEx(logonid : *const super::super::super::Win32::Foundation:: LUID, pserversilo : super::super::Foundation:: PESILO) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeMarkLogonSessionForTerminationNotificationEx(logonid, pserversilo.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeOpenObjectAuditAlarm<P0, P1, P2>(objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING, object: Option<*const core::ffi::c_void>, absoluteobjectname: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, securitydescriptor: P0, accessstate: *const super::super::Foundation::ACCESS_STATE, objectcreated: P1, accessgranted: P2, accessmode: i8) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeOpenObjectAuditAlarm(objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, object : *const core::ffi::c_void, absoluteobjectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE, objectcreated : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, accessmode : i8, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN));
    let mut result__ = core::mem::zeroed();
    SeOpenObjectAuditAlarm(objecttypename, core::mem::transmute(object.unwrap_or(std::ptr::null())), core::mem::transmute(absoluteobjectname.unwrap_or(std::ptr::null())), securitydescriptor.param().abi(), accessstate, objectcreated.param().abi(), accessgranted.param().abi(), accessmode, &mut result__);
    result__
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeOpenObjectAuditAlarmWithTransaction<P0, P1, P2>(objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING, object: Option<*const core::ffi::c_void>, absoluteobjectname: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, securitydescriptor: P0, accessstate: *const super::super::Foundation::ACCESS_STATE, objectcreated: P1, accessgranted: P2, accessmode: i8, transactionid: Option<*const windows_core::GUID>) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeOpenObjectAuditAlarmWithTransaction(objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, object : *const core::ffi::c_void, absoluteobjectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE, objectcreated : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, accessmode : i8, transactionid : *const windows_core::GUID, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN));
    let mut result__ = core::mem::zeroed();
    SeOpenObjectAuditAlarmWithTransaction(objecttypename, core::mem::transmute(object.unwrap_or(std::ptr::null())), core::mem::transmute(absoluteobjectname.unwrap_or(std::ptr::null())), securitydescriptor.param().abi(), accessstate, objectcreated.param().abi(), accessgranted.param().abi(), accessmode, core::mem::transmute(transactionid.unwrap_or(std::ptr::null())), &mut result__);
    result__
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeOpenObjectForDeleteAuditAlarm<P0, P1, P2>(objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING, object: Option<*const core::ffi::c_void>, absoluteobjectname: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, securitydescriptor: P0, accessstate: *const super::super::Foundation::ACCESS_STATE, objectcreated: P1, accessgranted: P2, accessmode: i8) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeOpenObjectForDeleteAuditAlarm(objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, object : *const core::ffi::c_void, absoluteobjectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE, objectcreated : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, accessmode : i8, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN));
    let mut result__ = core::mem::zeroed();
    SeOpenObjectForDeleteAuditAlarm(objecttypename, core::mem::transmute(object.unwrap_or(std::ptr::null())), core::mem::transmute(absoluteobjectname.unwrap_or(std::ptr::null())), securitydescriptor.param().abi(), accessstate, objectcreated.param().abi(), accessgranted.param().abi(), accessmode, &mut result__);
    result__
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeOpenObjectForDeleteAuditAlarmWithTransaction<P0, P1, P2>(objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING, object: Option<*const core::ffi::c_void>, absoluteobjectname: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, securitydescriptor: P0, accessstate: *const super::super::Foundation::ACCESS_STATE, objectcreated: P1, accessgranted: P2, accessmode: i8, transactionid: Option<*const windows_core::GUID>) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeOpenObjectForDeleteAuditAlarmWithTransaction(objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, object : *const core::ffi::c_void, absoluteobjectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE, objectcreated : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, accessmode : i8, transactionid : *const windows_core::GUID, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN));
    let mut result__ = core::mem::zeroed();
    SeOpenObjectForDeleteAuditAlarmWithTransaction(objecttypename, core::mem::transmute(object.unwrap_or(std::ptr::null())), core::mem::transmute(absoluteobjectname.unwrap_or(std::ptr::null())), securitydescriptor.param().abi(), accessstate, objectcreated.param().abi(), accessgranted.param().abi(), accessmode, core::mem::transmute(transactionid.unwrap_or(std::ptr::null())), &mut result__);
    result__
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SePrivilegeCheck(requiredprivileges: *mut super::super::super::Win32::Security::PRIVILEGE_SET, subjectsecuritycontext: *const super::super::Foundation::SECURITY_SUBJECT_CONTEXT, accessmode: i8) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn SePrivilegeCheck(requiredprivileges : *mut super::super::super::Win32::Security:: PRIVILEGE_SET, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, accessmode : i8) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SePrivilegeCheck(requiredprivileges, subjectsecuritycontext, accessmode)
}
#[inline]
pub unsafe fn SeQueryAuthenticationIdToken(token: *const core::ffi::c_void, authenticationid: *mut super::super::super::Win32::Foundation::LUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeQueryAuthenticationIdToken(token : *const core::ffi::c_void, authenticationid : *mut super::super::super::Win32::Foundation:: LUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeQueryAuthenticationIdToken(token, authenticationid)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeQueryInformationToken(token: *const core::ffi::c_void, tokeninformationclass: super::super::super::Win32::Security::TOKEN_INFORMATION_CLASS, tokeninformation: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeQueryInformationToken(token : *const core::ffi::c_void, tokeninformationclass : super::super::super::Win32::Security:: TOKEN_INFORMATION_CLASS, tokeninformation : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeQueryInformationToken(token, tokeninformationclass, tokeninformation)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeQuerySecurityDescriptorInfo(securityinformation: *const u32, securitydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, length: *mut u32, objectssecuritydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeQuerySecurityDescriptorInfo(securityinformation : *const u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, length : *mut u32, objectssecuritydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeQuerySecurityDescriptorInfo(securityinformation, securitydescriptor, length, objectssecuritydescriptor)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SeQueryServerSiloToken(token: *const core::ffi::c_void, pserversilo: *mut super::super::Foundation::PESILO) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeQueryServerSiloToken(token : *const core::ffi::c_void, pserversilo : *mut super::super::Foundation:: PESILO) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeQueryServerSiloToken(token, pserversilo)
}
#[inline]
pub unsafe fn SeQuerySessionIdToken(token: *const core::ffi::c_void, sessionid: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeQuerySessionIdToken(token : *const core::ffi::c_void, sessionid : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeQuerySessionIdToken(token, sessionid)
}
#[inline]
pub unsafe fn SeQuerySessionIdTokenEx(token: *const core::ffi::c_void, sessionid: *mut u32, isservicesession: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeQuerySessionIdTokenEx(token : *const core::ffi::c_void, sessionid : *mut u32, isservicesession : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeQuerySessionIdTokenEx(token, sessionid, isservicesession)
}
#[inline]
pub unsafe fn SeRegisterLogonSessionTerminatedRoutine(callbackroutine: PSE_LOGON_SESSION_TERMINATED_ROUTINE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeRegisterLogonSessionTerminatedRoutine(callbackroutine : PSE_LOGON_SESSION_TERMINATED_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeRegisterLogonSessionTerminatedRoutine(callbackroutine)
}
#[inline]
pub unsafe fn SeRegisterLogonSessionTerminatedRoutineEx(callbackroutine: PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeRegisterLogonSessionTerminatedRoutineEx(callbackroutine : PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeRegisterLogonSessionTerminatedRoutineEx(callbackroutine, context)
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[inline]
pub unsafe fn SeReportSecurityEventWithSubCategory<P0>(flags: u32, sourcename: *const super::super::super::Win32::Foundation::UNICODE_STRING, usersid: P0, auditparameters: *const super::super::super::Win32::Security::Authentication::Identity::SE_ADT_PARAMETER_ARRAY, auditsubcategoryid: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeReportSecurityEventWithSubCategory(flags : u32, sourcename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, usersid : super::super::super::Win32::Security:: PSID, auditparameters : *const super::super::super::Win32::Security::Authentication::Identity:: SE_ADT_PARAMETER_ARRAY, auditsubcategoryid : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeReportSecurityEventWithSubCategory(flags, sourcename, usersid.param().abi(), auditparameters, auditsubcategoryid)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeSetAccessStateGenericMapping(accessstate: *mut super::super::Foundation::ACCESS_STATE, genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING) {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeSetAccessStateGenericMapping(accessstate : *mut super::super::Foundation:: ACCESS_STATE, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING));
    SeSetAccessStateGenericMapping(accessstate, genericmapping)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeSetSecurityDescriptorInfo<P0>(object: Option<*const core::ffi::c_void>, securityinformation: *const u32, modificationdescriptor: P0, objectssecuritydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, pooltype: super::super::Foundation::POOL_TYPE, genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeSetSecurityDescriptorInfo(object : *const core::ffi::c_void, securityinformation : *const u32, modificationdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, pooltype : super::super::Foundation:: POOL_TYPE, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeSetSecurityDescriptorInfo(core::mem::transmute(object.unwrap_or(std::ptr::null())), securityinformation, modificationdescriptor.param().abi(), objectssecuritydescriptor, pooltype, genericmapping)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeSetSecurityDescriptorInfoEx<P0>(object: Option<*const core::ffi::c_void>, securityinformation: *const u32, modificationdescriptor: P0, objectssecuritydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, autoinheritflags: u32, pooltype: super::super::Foundation::POOL_TYPE, genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeSetSecurityDescriptorInfoEx(object : *const core::ffi::c_void, securityinformation : *const u32, modificationdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, autoinheritflags : u32, pooltype : super::super::Foundation:: POOL_TYPE, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeSetSecurityDescriptorInfoEx(core::mem::transmute(object.unwrap_or(std::ptr::null())), securityinformation, modificationdescriptor.param().abi(), objectssecuritydescriptor, autoinheritflags, pooltype, genericmapping)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SeShouldCheckForAccessRightsFromParent<P0>(objecttype: *const core::ffi::c_void, childdescriptor: P0, accessstate: *const super::super::Foundation::ACCESS_STATE) -> super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntoskrnl.exe" "system" fn SeShouldCheckForAccessRightsFromParent(objecttype : *const core::ffi::c_void, childdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeShouldCheckForAccessRightsFromParent(objecttype, childdescriptor.param().abi(), accessstate)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeTokenFromAccessInformation(accessinformation: Option<*const super::super::super::Win32::Security::TOKEN_ACCESS_INFORMATION>, token: Option<*mut core::ffi::c_void>, length: u32, requiredlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeTokenFromAccessInformation(accessinformation : *const super::super::super::Win32::Security:: TOKEN_ACCESS_INFORMATION, token : *mut core::ffi::c_void, length : u32, requiredlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeTokenFromAccessInformation(core::mem::transmute(accessinformation.unwrap_or(std::ptr::null())), core::mem::transmute(token.unwrap_or(std::ptr::null_mut())), length, requiredlength)
}
#[inline]
pub unsafe fn SeTokenIsAdmin(token: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeTokenIsAdmin(token : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeTokenIsAdmin(token)
}
#[inline]
pub unsafe fn SeTokenIsRestricted(token: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeTokenIsRestricted(token : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeTokenIsRestricted(token)
}
#[inline]
pub unsafe fn SeTokenIsWriteRestricted(token: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeTokenIsWriteRestricted(token : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
    SeTokenIsWriteRestricted(token)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SeTokenType(token: *const core::ffi::c_void) -> super::super::super::Win32::Security::TOKEN_TYPE {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeTokenType(token : *const core::ffi::c_void) -> super::super::super::Win32::Security:: TOKEN_TYPE);
    SeTokenType(token)
}
#[inline]
pub unsafe fn SeUnregisterLogonSessionTerminatedRoutine(callbackroutine: PSE_LOGON_SESSION_TERMINATED_ROUTINE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeUnregisterLogonSessionTerminatedRoutine(callbackroutine : PSE_LOGON_SESSION_TERMINATED_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeUnregisterLogonSessionTerminatedRoutine(callbackroutine)
}
#[inline]
pub unsafe fn SeUnregisterLogonSessionTerminatedRoutineEx(callbackroutine: PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntoskrnl.exe" "system" fn SeUnregisterLogonSessionTerminatedRoutineEx(callbackroutine : PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SeUnregisterLogonSessionTerminatedRoutineEx(callbackroutine, context)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SecLookupAccountName(name: *const super::super::super::Win32::Foundation::UNICODE_STRING, sidsize: *mut u32, sid: super::super::super::Win32::Security::PSID, nameuse: *mut super::super::super::Win32::Security::SID_NAME_USE, domainsize: *mut u32, referenceddomain: Option<*mut super::super::super::Win32::Foundation::UNICODE_STRING>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ksecdd.sys" "system" fn SecLookupAccountName(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, sidsize : *mut u32, sid : super::super::super::Win32::Security:: PSID, nameuse : *mut super::super::super::Win32::Security:: SID_NAME_USE, domainsize : *mut u32, referenceddomain : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SecLookupAccountName(name, sidsize, sid, nameuse, domainsize, core::mem::transmute(referenceddomain.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SecLookupAccountSid<P0>(sid: P0, namesize: *mut u32, namebuffer: *mut super::super::super::Win32::Foundation::UNICODE_STRING, domainsize: *mut u32, domainbuffer: Option<*mut super::super::super::Win32::Foundation::UNICODE_STRING>, nameuse: *mut super::super::super::Win32::Security::SID_NAME_USE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSID>,
{
    windows_targets::link!("ksecdd.sys" "system" fn SecLookupAccountSid(sid : super::super::super::Win32::Security:: PSID, namesize : *mut u32, namebuffer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, domainsize : *mut u32, domainbuffer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, nameuse : *mut super::super::super::Win32::Security:: SID_NAME_USE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SecLookupAccountSid(sid.param().abi(), namesize, namebuffer, domainsize, core::mem::transmute(domainbuffer.unwrap_or(std::ptr::null_mut())), nameuse)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SecLookupWellKnownSid(sidtype: super::super::super::Win32::Security::WELL_KNOWN_SID_TYPE, sid: super::super::super::Win32::Security::PSID, sidbuffersize: u32, sidsize: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ksecdd.sys" "system" fn SecLookupWellKnownSid(sidtype : super::super::super::Win32::Security:: WELL_KNOWN_SID_TYPE, sid : super::super::super::Win32::Security:: PSID, sidbuffersize : u32, sidsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SecLookupWellKnownSid(sidtype, sid, sidbuffersize, core::mem::transmute(sidsize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SecMakeSPN<P0>(serviceclass: *mut super::super::super::Win32::Foundation::UNICODE_STRING, servicename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, instancename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, instanceport: u16, referrer: *mut super::super::super::Win32::Foundation::UNICODE_STRING, spn: *mut super::super::super::Win32::Foundation::UNICODE_STRING, length: *mut u32, allocate: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ksecdd.sys" "system" fn SecMakeSPN(serviceclass : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instanceport : u16, referrer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, spn : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, length : *mut u32, allocate : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SecMakeSPN(serviceclass, servicename, instancename, instanceport, referrer, spn, length, allocate.param().abi())
}
#[inline]
pub unsafe fn SecMakeSPNEx<P0>(serviceclass: *mut super::super::super::Win32::Foundation::UNICODE_STRING, servicename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, instancename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, instanceport: u16, referrer: *mut super::super::super::Win32::Foundation::UNICODE_STRING, targetinfo: *mut super::super::super::Win32::Foundation::UNICODE_STRING, spn: *mut super::super::super::Win32::Foundation::UNICODE_STRING, length: *mut u32, allocate: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ksecdd.sys" "system" fn SecMakeSPNEx(serviceclass : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instanceport : u16, referrer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, targetinfo : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, spn : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, length : *mut u32, allocate : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SecMakeSPNEx(serviceclass, servicename, instancename, instanceport, referrer, targetinfo, spn, length, allocate.param().abi())
}
#[inline]
pub unsafe fn SecMakeSPNEx2<P0, P1>(serviceclass: *mut super::super::super::Win32::Foundation::UNICODE_STRING, servicename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, instancename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, instanceport: u16, referrer: *mut super::super::super::Win32::Foundation::UNICODE_STRING, intargetinfo: *mut super::super::super::Win32::Foundation::UNICODE_STRING, spn: *mut super::super::super::Win32::Foundation::UNICODE_STRING, totalsize: *mut u32, allocate: P0, istargetinfomarshaled: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ksecdd.sys" "system" fn SecMakeSPNEx2(serviceclass : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instanceport : u16, referrer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, intargetinfo : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, spn : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, totalsize : *mut u32, allocate : super::super::super::Win32::Foundation:: BOOLEAN, istargetinfomarshaled : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SecMakeSPNEx2(serviceclass, servicename, instancename, instanceport, referrer, intargetinfo, spn, totalsize, allocate.param().abi(), istargetinfomarshaled.param().abi())
}
#[inline]
pub unsafe fn SetContextAttributesW(phcontext: *const SecHandle, ulattribute: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_targets::link!("secur32.dll" "system" fn SetContextAttributesW(phcontext : *const SecHandle, ulattribute : u32, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    SetContextAttributesW(phcontext, ulattribute, pbuffer, cbbuffer).ok()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiAcceptSecurityContextAsync(asynccontext: *mut super::super::Foundation::SspiAsyncContext, phcredential: Option<*const SecHandle>, phcontext: Option<*const SecHandle>, pinput: Option<*const SecBufferDesc>, fcontextreq: u32, targetdatarep: u32, phnewcontext: Option<*const SecHandle>, poutput: Option<*const SecBufferDesc>, pfcontextattr: *const u32, ptsexpiry: Option<*const i64>) -> windows_core::Result<()> {
    windows_targets::link!("ksecdd.sys" "system" fn SspiAcceptSecurityContextAsync(asynccontext : *mut super::super::Foundation:: SspiAsyncContext, phcredential : *const SecHandle, phcontext : *const SecHandle, pinput : *const SecBufferDesc, fcontextreq : u32, targetdatarep : u32, phnewcontext : *const SecHandle, poutput : *const SecBufferDesc, pfcontextattr : *const u32, ptsexpiry : *const i64) -> windows_core::HRESULT);
    SspiAcceptSecurityContextAsync(asynccontext, core::mem::transmute(phcredential.unwrap_or(std::ptr::null())), core::mem::transmute(phcontext.unwrap_or(std::ptr::null())), core::mem::transmute(pinput.unwrap_or(std::ptr::null())), fcontextreq, targetdatarep, core::mem::transmute(phnewcontext.unwrap_or(std::ptr::null())), core::mem::transmute(poutput.unwrap_or(std::ptr::null())), pfcontextattr, core::mem::transmute(ptsexpiry.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security_Authentication_Identity"))]
#[inline]
pub unsafe fn SspiAcquireCredentialsHandleAsyncA<P0, P1>(asynccontext: *mut super::super::Foundation::SspiAsyncContext, pszprincipal: P0, pszpackage: P1, fcredentialuse: u32, pvlogonid: Option<*const core::ffi::c_void>, pauthdata: Option<*const core::ffi::c_void>, pgetkeyfn: super::super::super::Win32::Security::Authentication::Identity::SEC_GET_KEY_FN, pvgetkeyargument: Option<*const core::ffi::c_void>, phcredential: *const SecHandle, ptsexpiry: Option<*const i64>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ksecdd.sys" "system" fn SspiAcquireCredentialsHandleAsyncA(asynccontext : *mut super::super::Foundation:: SspiAsyncContext, pszprincipal : windows_core::PCSTR, pszpackage : windows_core::PCSTR, fcredentialuse : u32, pvlogonid : *const core::ffi::c_void, pauthdata : *const core::ffi::c_void, pgetkeyfn : super::super::super::Win32::Security::Authentication::Identity:: SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, phcredential : *const SecHandle, ptsexpiry : *const i64) -> windows_core::HRESULT);
    SspiAcquireCredentialsHandleAsyncA(asynccontext, pszprincipal.param().abi(), pszpackage.param().abi(), fcredentialuse, core::mem::transmute(pvlogonid.unwrap_or(std::ptr::null())), core::mem::transmute(pauthdata.unwrap_or(std::ptr::null())), pgetkeyfn, core::mem::transmute(pvgetkeyargument.unwrap_or(std::ptr::null())), phcredential, core::mem::transmute(ptsexpiry.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security_Authentication_Identity"))]
#[inline]
pub unsafe fn SspiAcquireCredentialsHandleAsyncW(asynccontext: *mut super::super::Foundation::SspiAsyncContext, pszprincipal: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, pszpackage: *const super::super::super::Win32::Foundation::UNICODE_STRING, fcredentialuse: u32, pvlogonid: Option<*const core::ffi::c_void>, pauthdata: Option<*const core::ffi::c_void>, pgetkeyfn: super::super::super::Win32::Security::Authentication::Identity::SEC_GET_KEY_FN, pvgetkeyargument: Option<*const core::ffi::c_void>, phcredential: *const SecHandle, ptsexpiry: Option<*const i64>) -> windows_core::Result<()> {
    windows_targets::link!("ksecdd.sys" "system" fn SspiAcquireCredentialsHandleAsyncW(asynccontext : *mut super::super::Foundation:: SspiAsyncContext, pszprincipal : *const super::super::super::Win32::Foundation:: UNICODE_STRING, pszpackage : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fcredentialuse : u32, pvlogonid : *const core::ffi::c_void, pauthdata : *const core::ffi::c_void, pgetkeyfn : super::super::super::Win32::Security::Authentication::Identity:: SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, phcredential : *const SecHandle, ptsexpiry : *const i64) -> windows_core::HRESULT);
    SspiAcquireCredentialsHandleAsyncW(asynccontext, core::mem::transmute(pszprincipal.unwrap_or(std::ptr::null())), pszpackage, fcredentialuse, core::mem::transmute(pvlogonid.unwrap_or(std::ptr::null())), core::mem::transmute(pauthdata.unwrap_or(std::ptr::null())), pgetkeyfn, core::mem::transmute(pvgetkeyargument.unwrap_or(std::ptr::null())), phcredential, core::mem::transmute(ptsexpiry.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiCreateAsyncContext() -> *mut super::super::Foundation::SspiAsyncContext {
    windows_targets::link!("ksecdd.sys" "system" fn SspiCreateAsyncContext() -> *mut super::super::Foundation:: SspiAsyncContext);
    SspiCreateAsyncContext()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiDeleteSecurityContextAsync(asynccontext: *mut super::super::Foundation::SspiAsyncContext, phcontext: *const SecHandle) -> windows_core::Result<()> {
    windows_targets::link!("ksecdd.sys" "system" fn SspiDeleteSecurityContextAsync(asynccontext : *mut super::super::Foundation:: SspiAsyncContext, phcontext : *const SecHandle) -> windows_core::HRESULT);
    SspiDeleteSecurityContextAsync(asynccontext, phcontext).ok()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiFreeAsyncContext(handle: Option<*const super::super::Foundation::SspiAsyncContext>) {
    windows_targets::link!("ksecdd.sys" "system" fn SspiFreeAsyncContext(handle : *const super::super::Foundation:: SspiAsyncContext));
    SspiFreeAsyncContext(core::mem::transmute(handle.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiFreeCredentialsHandleAsync(asynccontext: *mut super::super::Foundation::SspiAsyncContext, phcredential: *const SecHandle) -> windows_core::Result<()> {
    windows_targets::link!("ksecdd.sys" "system" fn SspiFreeCredentialsHandleAsync(asynccontext : *mut super::super::Foundation:: SspiAsyncContext, phcredential : *const SecHandle) -> windows_core::HRESULT);
    SspiFreeCredentialsHandleAsync(asynccontext, phcredential).ok()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiGetAsyncCallStatus(handle: *const super::super::Foundation::SspiAsyncContext) -> windows_core::Result<()> {
    windows_targets::link!("ksecdd.sys" "system" fn SspiGetAsyncCallStatus(handle : *const super::super::Foundation:: SspiAsyncContext) -> windows_core::HRESULT);
    SspiGetAsyncCallStatus(handle).ok()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiInitializeSecurityContextAsyncA<P0>(asynccontext: *mut super::super::Foundation::SspiAsyncContext, phcredential: Option<*const SecHandle>, phcontext: Option<*const SecHandle>, psztargetname: P0, fcontextreq: u32, reserved1: u32, targetdatarep: u32, pinput: Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: Option<*const SecHandle>, poutput: Option<*const SecBufferDesc>, pfcontextattr: *const u32, ptsexpiry: Option<*const i64>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ksecdd.sys" "system" fn SspiInitializeSecurityContextAsyncA(asynccontext : *mut super::super::Foundation:: SspiAsyncContext, phcredential : *const SecHandle, phcontext : *const SecHandle, psztargetname : windows_core::PCSTR, fcontextreq : u32, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : *const SecHandle, poutput : *const SecBufferDesc, pfcontextattr : *const u32, ptsexpiry : *const i64) -> windows_core::HRESULT);
    SspiInitializeSecurityContextAsyncA(asynccontext, core::mem::transmute(phcredential.unwrap_or(std::ptr::null())), core::mem::transmute(phcontext.unwrap_or(std::ptr::null())), psztargetname.param().abi(), fcontextreq, reserved1, targetdatarep, core::mem::transmute(pinput.unwrap_or(std::ptr::null())), reserved2, core::mem::transmute(phnewcontext.unwrap_or(std::ptr::null())), core::mem::transmute(poutput.unwrap_or(std::ptr::null())), pfcontextattr, core::mem::transmute(ptsexpiry.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiInitializeSecurityContextAsyncW(asynccontext: *mut super::super::Foundation::SspiAsyncContext, phcredential: Option<*const SecHandle>, phcontext: Option<*const SecHandle>, psztargetname: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, fcontextreq: u32, reserved1: u32, targetdatarep: u32, pinput: Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: Option<*const SecHandle>, poutput: Option<*const SecBufferDesc>, pfcontextattr: *const u32, ptsexpiry: Option<*const i64>) -> windows_core::Result<()> {
    windows_targets::link!("ksecdd.sys" "system" fn SspiInitializeSecurityContextAsyncW(asynccontext : *mut super::super::Foundation:: SspiAsyncContext, phcredential : *const SecHandle, phcontext : *const SecHandle, psztargetname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fcontextreq : u32, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : *const SecHandle, poutput : *const SecBufferDesc, pfcontextattr : *const u32, ptsexpiry : *const i64) -> windows_core::HRESULT);
    SspiInitializeSecurityContextAsyncW(
        asynccontext,
        core::mem::transmute(phcredential.unwrap_or(std::ptr::null())),
        core::mem::transmute(phcontext.unwrap_or(std::ptr::null())),
        core::mem::transmute(psztargetname.unwrap_or(std::ptr::null())),
        fcontextreq,
        reserved1,
        targetdatarep,
        core::mem::transmute(pinput.unwrap_or(std::ptr::null())),
        reserved2,
        core::mem::transmute(phnewcontext.unwrap_or(std::ptr::null())),
        core::mem::transmute(poutput.unwrap_or(std::ptr::null())),
        pfcontextattr,
        core::mem::transmute(ptsexpiry.unwrap_or(std::ptr::null())),
    )
    .ok()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiReinitAsyncContext(handle: *mut super::super::Foundation::SspiAsyncContext) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ksecdd.sys" "system" fn SspiReinitAsyncContext(handle : *mut super::super::Foundation:: SspiAsyncContext) -> super::super::super::Win32::Foundation:: NTSTATUS);
    SspiReinitAsyncContext(handle)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn SspiSetAsyncNotifyCallback(context: *const super::super::Foundation::SspiAsyncContext, callback: SspiAsyncNotifyCallback, callbackdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("ksecdd.sys" "system" fn SspiSetAsyncNotifyCallback(context : *const super::super::Foundation:: SspiAsyncContext, callback : SspiAsyncNotifyCallback, callbackdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    SspiSetAsyncNotifyCallback(context, callback, core::mem::transmute(callbackdata.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn VerifySignature(phcontext: *const SecHandle, pmessage: *const SecBufferDesc, messageseqno: u32) -> windows_core::Result<u32> {
    windows_targets::link!("secur32.dll" "system" fn VerifySignature(phcontext : *const SecHandle, pmessage : *const SecBufferDesc, messageseqno : u32, pfqop : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VerifySignature(phcontext, pmessage, messageseqno, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwAccessCheckAndAuditAlarm<P0, P1>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING, objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING, securitydescriptor: P0, desiredaccess: u32, genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING, objectcreation: P1, grantedaccess: *mut u32, accessstatus: *mut i32, generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwAccessCheckAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwAccessCheckAndAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename, objectname, securitydescriptor.param().abi(), desiredaccess, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, generateonclose)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwAccessCheckByTypeAndAuditAlarm<P0, P1, P2>(
    subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    handleid: Option<*const core::ffi::c_void>,
    objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    securitydescriptor: P0,
    principalselfsid: P1,
    desiredaccess: u32,
    audittype: super::super::super::Win32::Security::AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: Option<&[super::super::super::Win32::Security::OBJECT_TYPE_LIST]>,
    genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING,
    objectcreation: P2,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN,
) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwAccessCheckByTypeAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Security:: PSID, desiredaccess : u32, audittype : super::super::super::Win32::Security:: AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::super::super::Win32::Security:: OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwAccessCheckByTypeAndAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename, objectname, securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, generateonclose)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwAccessCheckByTypeResultListAndAuditAlarm<P0, P1, P2>(
    subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    handleid: Option<*const core::ffi::c_void>,
    objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    securitydescriptor: P0,
    principalselfsid: P1,
    desiredaccess: u32,
    audittype: super::super::super::Win32::Security::AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: Option<*const super::super::super::Win32::Security::OBJECT_TYPE_LIST>,
    objecttypelistlength: u32,
    genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING,
    objectcreation: P2,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN,
) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwAccessCheckByTypeResultListAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Security:: PSID, desiredaccess : u32, audittype : super::super::super::Win32::Security:: AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::super::super::Win32::Security:: OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwAccessCheckByTypeResultListAndAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename, objectname, securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null())), objecttypelistlength, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, generateonclose)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwAccessCheckByTypeResultListAndAuditAlarmByHandle<P0, P1, P2, P3>(
    subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    handleid: Option<*const core::ffi::c_void>,
    clienttoken: P0,
    objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING,
    securitydescriptor: P1,
    principalselfsid: P2,
    desiredaccess: u32,
    audittype: super::super::super::Win32::Security::AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: Option<*const super::super::super::Win32::Security::OBJECT_TYPE_LIST>,
    objecttypelistlength: u32,
    genericmapping: *const super::super::super::Win32::Security::GENERIC_MAPPING,
    objectcreation: P3,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN,
) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P2: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, clienttoken : super::super::super::Win32::Foundation:: HANDLE, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Security:: PSID, desiredaccess : u32, audittype : super::super::super::Win32::Security:: AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::super::super::Win32::Security:: OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), clienttoken.param().abi(), objecttypename, objectname, securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null())), objecttypelistlength, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, generateonclose)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwAdjustGroupsToken<P0, P1>(tokenhandle: P0, resettodefault: P1, newstate: Option<*const super::super::super::Win32::Security::TOKEN_GROUPS>, bufferlength: u32, previousstate: Option<*mut super::super::super::Win32::Security::TOKEN_GROUPS>, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwAdjustGroupsToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, resettodefault : super::super::super::Win32::Foundation:: BOOLEAN, newstate : *const super::super::super::Win32::Security:: TOKEN_GROUPS, bufferlength : u32, previousstate : *mut super::super::super::Win32::Security:: TOKEN_GROUPS, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwAdjustGroupsToken(tokenhandle.param().abi(), resettodefault.param().abi(), core::mem::transmute(newstate.unwrap_or(std::ptr::null())), bufferlength, core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())), returnlength)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwAdjustPrivilegesToken<P0, P1>(tokenhandle: P0, disableallprivileges: P1, newstate: Option<*const super::super::super::Win32::Security::TOKEN_PRIVILEGES>, bufferlength: u32, previousstate: Option<*mut super::super::super::Win32::Security::TOKEN_PRIVILEGES>, returnlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwAdjustPrivilegesToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, disableallprivileges : super::super::super::Win32::Foundation:: BOOLEAN, newstate : *const super::super::super::Win32::Security:: TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut super::super::super::Win32::Security:: TOKEN_PRIVILEGES, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwAdjustPrivilegesToken(tokenhandle.param().abi(), disableallprivileges.param().abi(), core::mem::transmute(newstate.unwrap_or(std::ptr::null())), bufferlength, core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ZwAllocateVirtualMemory<P0>(processhandle: P0, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, regionsize: *mut usize, allocationtype: u32, protect: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwAllocateVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, regionsize : *mut usize, allocationtype : u32, protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwAllocateVirtualMemory(processhandle.param().abi(), baseaddress, zerobits, regionsize, allocationtype, protect)
}
#[cfg(feature = "Win32_System_Memory")]
#[inline]
pub unsafe fn ZwAllocateVirtualMemoryEx<P0>(processhandle: P0, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::super::super::Win32::System::Memory::MEM_EXTENDED_PARAMETER]>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwAllocateVirtualMemoryEx(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::super::super::Win32::System::Memory:: MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwAllocateVirtualMemoryEx(processhandle.param().abi(), baseaddress, regionsize, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwCancelIoFileEx<P0>(filehandle: P0, iorequesttocancel: Option<*const super::super::super::Win32::System::IO::IO_STATUS_BLOCK>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwCancelIoFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, iorequesttocancel : *const super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCancelIoFileEx(filehandle.param().abi(), core::mem::transmute(iorequesttocancel.unwrap_or(std::ptr::null())), iostatusblock)
}
#[inline]
pub unsafe fn ZwCloseObjectAuditAlarm<P0>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, generateonclose: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwCloseObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, generateonclose : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCloseObjectAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), generateonclose.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwCreateDirectoryObject(directoryhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwCreateDirectoryObject(directoryhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCreateDirectoryObject(directoryhandle, desiredaccess, objectattributes)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn ZwCreateEvent<P0>(eventhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, eventtype: super::super::super::Win32::System::Kernel::EVENT_TYPE, initialstate: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwCreateEvent(eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, eventtype : super::super::super::Win32::System::Kernel:: EVENT_TYPE, initialstate : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCreateEvent(eventhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), eventtype, initialstate.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Memory"))]
#[inline]
pub unsafe fn ZwCreateSectionEx<P0>(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, maximumsize: Option<*const i64>, sectionpageprotection: u32, allocationattributes: u32, filehandle: P0, extendedparameters: Option<&mut [super::super::super::Win32::System::Memory::MEM_EXTENDED_PARAMETER]>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwCreateSectionEx(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::super::super::Win32::Foundation:: HANDLE, extendedparameters : *mut super::super::super::Win32::System::Memory:: MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCreateSectionEx(sectionhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), core::mem::transmute(maximumsize.unwrap_or(std::ptr::null())), sectionpageprotection, allocationattributes, filehandle.param().abi(), core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwDeleteFile(objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwDeleteFile(objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwDeleteFile(objectattributes)
}
#[inline]
pub unsafe fn ZwDeleteObjectAuditAlarm<P0>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, generateonclose: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwDeleteObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, generateonclose : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwDeleteObjectAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), generateonclose.param().abi())
}
#[inline]
pub unsafe fn ZwDuplicateObject<P0, P1, P2>(sourceprocesshandle: P0, sourcehandle: P1, targetprocesshandle: P2, targethandle: Option<*mut super::super::super::Win32::Foundation::HANDLE>, desiredaccess: u32, handleattributes: u32, options: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwDuplicateObject(sourceprocesshandle : super::super::super::Win32::Foundation:: HANDLE, sourcehandle : super::super::super::Win32::Foundation:: HANDLE, targetprocesshandle : super::super::super::Win32::Foundation:: HANDLE, targethandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, handleattributes : u32, options : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwDuplicateObject(sourceprocesshandle.param().abi(), sourcehandle.param().abi(), targetprocesshandle.param().abi(), core::mem::transmute(targethandle.unwrap_or(std::ptr::null_mut())), desiredaccess, handleattributes, options)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwDuplicateToken<P0, P1>(existingtokenhandle: P0, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, effectiveonly: P1, tokentype: super::super::super::Win32::Security::TOKEN_TYPE, newtokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwDuplicateToken(existingtokenhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, effectiveonly : super::super::super::Win32::Foundation:: BOOLEAN, tokentype : super::super::super::Win32::Security:: TOKEN_TYPE, newtokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwDuplicateToken(existingtokenhandle.param().abi(), desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), effectiveonly.param().abi(), tokentype, newtokenhandle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwFilterToken<P0>(existingtokenhandle: P0, flags: u32, sidstodisable: Option<*const super::super::super::Win32::Security::TOKEN_GROUPS>, privilegestodelete: Option<*const super::super::super::Win32::Security::TOKEN_PRIVILEGES>, restrictedsids: Option<*const super::super::super::Win32::Security::TOKEN_GROUPS>, newtokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwFilterToken(existingtokenhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, sidstodisable : *const super::super::super::Win32::Security:: TOKEN_GROUPS, privilegestodelete : *const super::super::super::Win32::Security:: TOKEN_PRIVILEGES, restrictedsids : *const super::super::super::Win32::Security:: TOKEN_GROUPS, newtokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwFilterToken(existingtokenhandle.param().abi(), flags, core::mem::transmute(sidstodisable.unwrap_or(std::ptr::null())), core::mem::transmute(privilegestodelete.unwrap_or(std::ptr::null())), core::mem::transmute(restrictedsids.unwrap_or(std::ptr::null())), newtokenhandle)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwFlushBuffersFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwFlushBuffersFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwFlushBuffersFile(filehandle.param().abi(), iostatusblock)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwFlushBuffersFileEx<P0>(filehandle: P0, flags: u32, parameters: *const core::ffi::c_void, parameterssize: u32, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwFlushBuffersFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, parameters : *const core::ffi::c_void, parameterssize : u32, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwFlushBuffersFileEx(filehandle.param().abi(), flags, parameters, parameterssize, iostatusblock)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwFlushVirtualMemory<P0>(processhandle: P0, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, iostatus: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwFlushVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, iostatus : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwFlushVirtualMemory(processhandle.param().abi(), baseaddress, regionsize, iostatus)
}
#[inline]
pub unsafe fn ZwFreeVirtualMemory<P0>(processhandle: P0, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, freetype: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwFreeVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, freetype : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwFreeVirtualMemory(processhandle.param().abi(), baseaddress, regionsize, freetype)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwFsControlFile<P0, P1>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fscontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwFsControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwFsControlFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, fscontrolcode, core::mem::transmute(inputbuffer.unwrap_or(std::ptr::null())), inputbufferlength, core::mem::transmute(outputbuffer.unwrap_or(std::ptr::null_mut())), outputbufferlength)
}
#[inline]
pub unsafe fn ZwImpersonateAnonymousToken<P0>(threadhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwImpersonateAnonymousToken(threadhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwImpersonateAnonymousToken(threadhandle.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwLockFile<P0, P1, P2, P3>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, byteoffset: *const i64, length: *const i64, key: u32, failimmediately: P2, exclusivelock: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwLockFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32, failimmediately : super::super::super::Win32::Foundation:: BOOLEAN, exclusivelock : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwLockFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, byteoffset, length, key, failimmediately.param().abi(), exclusivelock.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwNotifyChangeKey<P0, P1, P2, P3>(keyhandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, completionfilter: u32, watchtree: P2, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, asynchronous: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwNotifyChangeKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, completionfilter : u32, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwNotifyChangeKey(keyhandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, completionfilter, watchtree.param().abi(), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, asynchronous.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenDirectoryObject(directoryhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenDirectoryObject(directoryhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenDirectoryObject(directoryhandle, desiredaccess, objectattributes)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwOpenObjectAuditAlarm<P0, P1, P2, P3>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: *const super::super::super::Win32::Foundation::UNICODE_STRING, objectname: *const super::super::super::Win32::Foundation::UNICODE_STRING, securitydescriptor: P0, clienttoken: P1, desiredaccess: u32, grantedaccess: u32, privileges: Option<*const super::super::super::Win32::Security::PRIVILEGE_SET>, objectcreation: P2, accessgranted: P3, generateonclose: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, clienttoken : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenObjectAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename, objectname, securitydescriptor.param().abi(), clienttoken.param().abi(), desiredaccess, grantedaccess, core::mem::transmute(privileges.unwrap_or(std::ptr::null())), objectcreation.param().abi(), accessgranted.param().abi(), generateonclose)
}
#[inline]
pub unsafe fn ZwOpenProcessToken<P0>(processhandle: P0, desiredaccess: u32, tokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenProcessToken(processhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenProcessToken(processhandle.param().abi(), desiredaccess, tokenhandle)
}
#[inline]
pub unsafe fn ZwOpenProcessTokenEx<P0>(processhandle: P0, desiredaccess: u32, handleattributes: u32, tokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenProcessTokenEx(processhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, handleattributes : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenProcessTokenEx(processhandle.param().abi(), desiredaccess, handleattributes, tokenhandle)
}
#[inline]
pub unsafe fn ZwOpenThreadToken<P0, P1>(threadhandle: P0, desiredaccess: u32, openasself: P1, tokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenThreadToken(threadhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, openasself : super::super::super::Win32::Foundation:: BOOLEAN, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenThreadToken(threadhandle.param().abi(), desiredaccess, openasself.param().abi(), tokenhandle)
}
#[inline]
pub unsafe fn ZwOpenThreadTokenEx<P0, P1>(threadhandle: P0, desiredaccess: u32, openasself: P1, handleattributes: u32, tokenhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenThreadTokenEx(threadhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, openasself : super::super::super::Win32::Foundation:: BOOLEAN, handleattributes : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenThreadTokenEx(threadhandle.param().abi(), desiredaccess, openasself.param().abi(), handleattributes, tokenhandle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwPrivilegeCheck<P0>(clienttoken: P0, requiredprivileges: *mut super::super::super::Win32::Security::PRIVILEGE_SET, result: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwPrivilegeCheck(clienttoken : super::super::super::Win32::Foundation:: HANDLE, requiredprivileges : *mut super::super::super::Win32::Security:: PRIVILEGE_SET, result : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwPrivilegeCheck(clienttoken.param().abi(), requiredprivileges, result)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwPrivilegeObjectAuditAlarm<P0, P1>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, clienttoken: P0, desiredaccess: u32, privileges: *const super::super::super::Win32::Security::PRIVILEGE_SET, accessgranted: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwPrivilegeObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const core::ffi::c_void, clienttoken : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwPrivilegeObjectAuditAlarm(subsystemname, core::mem::transmute(handleid.unwrap_or(std::ptr::null())), clienttoken.param().abi(), desiredaccess, privileges, accessgranted.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwPrivilegedServiceAuditAlarm<P0, P1>(subsystemname: *const super::super::super::Win32::Foundation::UNICODE_STRING, servicename: *const super::super::super::Win32::Foundation::UNICODE_STRING, clienttoken: P0, privileges: *const super::super::super::Win32::Security::PRIVILEGE_SET, accessgranted: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwPrivilegedServiceAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, clienttoken : super::super::super::Win32::Foundation:: HANDLE, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwPrivilegedServiceAuditAlarm(subsystemname, servicename, clienttoken.param().abi(), privileges, accessgranted.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwQueryDirectoryFile<P0, P1, P2, P3>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: FILE_INFORMATION_CLASS, returnsingleentry: P2, filename: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, restartscan: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryDirectoryFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryDirectoryFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, fileinformation, length, fileinformationclass, returnsingleentry.param().abi(), core::mem::transmute(filename.unwrap_or(std::ptr::null())), restartscan.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwQueryDirectoryFileEx<P0, P1>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: FILE_INFORMATION_CLASS, queryflags: u32, filename: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryDirectoryFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS, queryflags : u32, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryDirectoryFileEx(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, fileinformation, length, fileinformationclass, queryflags, core::mem::transmute(filename.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn ZwQueryDirectoryObject<P0, P1, P2>(directoryhandle: P0, buffer: Option<*mut core::ffi::c_void>, length: u32, returnsingleentry: P1, restartscan: P2, context: *mut u32, returnlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryDirectoryObject(directoryhandle : super::super::super::Win32::Foundation:: HANDLE, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, restartscan : super::super::super::Win32::Foundation:: BOOLEAN, context : *mut u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryDirectoryObject(directoryhandle.param().abi(), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), length, returnsingleentry.param().abi(), restartscan.param().abi(), context, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwQueryEaFile<P0, P1, P2>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: P1, ealist: Option<*const core::ffi::c_void>, ealistlength: u32, eaindex: Option<*const u32>, restartscan: P2) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryEaFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, ealist : *const core::ffi::c_void, ealistlength : u32, eaindex : *const u32, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryEaFile(filehandle.param().abi(), iostatusblock, buffer, length, returnsingleentry.param().abi(), core::mem::transmute(ealist.unwrap_or(std::ptr::null())), ealistlength, core::mem::transmute(eaindex.unwrap_or(std::ptr::null())), restartscan.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwQueryFullAttributesFile(objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, fileinformation: *mut FILE_NETWORK_OPEN_INFORMATION) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryFullAttributesFile(objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, fileinformation : *mut FILE_NETWORK_OPEN_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryFullAttributesFile(objectattributes, fileinformation)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwQueryInformationToken<P0>(tokenhandle: P0, tokeninformationclass: super::super::super::Win32::Security::TOKEN_INFORMATION_CLASS, tokeninformation: Option<*mut core::ffi::c_void>, tokeninformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, tokeninformationclass : super::super::super::Win32::Security:: TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryInformationToken(tokenhandle.param().abi(), tokeninformationclass, core::mem::transmute(tokeninformation.unwrap_or(std::ptr::null_mut())), tokeninformationlength, returnlength)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwQueryObject<P0>(handle: P0, objectinformationclass: super::super::Foundation::OBJECT_INFORMATION_CLASS, objectinformation: Option<*mut core::ffi::c_void>, objectinformationlength: u32, returnlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryObject(handle : super::super::super::Win32::Foundation:: HANDLE, objectinformationclass : super::super::Foundation:: OBJECT_INFORMATION_CLASS, objectinformation : *mut core::ffi::c_void, objectinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryObject(handle.param().abi(), objectinformationclass, core::mem::transmute(objectinformation.unwrap_or(std::ptr::null_mut())), objectinformationlength, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ZwQueryQuotaInformationFile<P0, P1, P2, P3>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: P1, sidlist: Option<*const core::ffi::c_void>, sidlistlength: u32, startsid: P2, restartscan: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::Win32::Security::PSID>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryQuotaInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, sidlist : *const core::ffi::c_void, sidlistlength : u32, startsid : super::super::super::Win32::Security:: PSID, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryQuotaInformationFile(filehandle.param().abi(), iostatusblock, buffer, length, returnsingleentry.param().abi(), core::mem::transmute(sidlist.unwrap_or(std::ptr::null())), sidlistlength, startsid.param().abi(), restartscan.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwQuerySecurityObject<P0>(handle: P0, securityinformation: u32, securitydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, length: u32, lengthneeded: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQuerySecurityObject(handle : super::super::super::Win32::Foundation:: HANDLE, securityinformation : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, length : u32, lengthneeded : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQuerySecurityObject(handle.param().abi(), securityinformation, securitydescriptor, length, lengthneeded)
}
#[inline]
pub unsafe fn ZwQueryVirtualMemory<P0>(processhandle: P0, baseaddress: Option<*const core::ffi::c_void>, memoryinformationclass: MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: Option<*mut usize>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, memoryinformationclass : MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationlength : usize, returnlength : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryVirtualMemory(processhandle.param().abi(), core::mem::transmute(baseaddress.unwrap_or(std::ptr::null())), memoryinformationclass, memoryinformation, memoryinformationlength, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwQueryVolumeInformationFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryVolumeInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryVolumeInformationFile(filehandle.param().abi(), iostatusblock, fsinformation, length, fsinformationclass)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwSetEaFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetEaFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetEaFile(filehandle.param().abi(), iostatusblock, buffer, length)
}
#[inline]
pub unsafe fn ZwSetEvent<P0>(eventhandle: P0, previousstate: Option<*mut i32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetEvent(eventhandle : super::super::super::Win32::Foundation:: HANDLE, previousstate : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetEvent(eventhandle.param().abi(), core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwSetInformationToken<P0>(tokenhandle: P0, tokeninformationclass: super::super::super::Win32::Security::TOKEN_INFORMATION_CLASS, tokeninformation: *const core::ffi::c_void, tokeninformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, tokeninformationclass : super::super::super::Win32::Security:: TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetInformationToken(tokenhandle.param().abi(), tokeninformationclass, tokeninformation, tokeninformationlength)
}
#[inline]
pub unsafe fn ZwSetInformationVirtualMemory<P0>(processhandle: P0, vminformationclass: VIRTUAL_MEMORY_INFORMATION_CLASS, virtualaddresses: &[MEMORY_RANGE_ENTRY], vminformation: *const core::ffi::c_void, vminformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, vminformationclass : VIRTUAL_MEMORY_INFORMATION_CLASS, numberofentries : usize, virtualaddresses : *const MEMORY_RANGE_ENTRY, vminformation : *const core::ffi::c_void, vminformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetInformationVirtualMemory(processhandle.param().abi(), vminformationclass, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), vminformation, vminformationlength)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwSetQuotaInformationFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetQuotaInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetQuotaInformationFile(filehandle.param().abi(), iostatusblock, buffer, length)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ZwSetSecurityObject<P0, P1>(handle: P0, securityinformation: u32, securitydescriptor: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetSecurityObject(handle : super::super::super::Win32::Foundation:: HANDLE, securityinformation : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetSecurityObject(handle.param().abi(), securityinformation, securitydescriptor.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwSetVolumeInformationFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fsinformation: *const core::ffi::c_void, length: u32, fsinformationclass: FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetVolumeInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fsinformation : *const core::ffi::c_void, length : u32, fsinformationclass : FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetVolumeInformationFile(filehandle.param().abi(), iostatusblock, fsinformation, length, fsinformationclass)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ZwUnlockFile<P0>(filehandle: P0, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, byteoffset: *const i64, length: *const i64, key: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwUnlockFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwUnlockFile(filehandle.param().abi(), iostatusblock, byteoffset, length, key)
}
pub const ATOMIC_CREATE_ECP_IN_FLAG_BEST_EFFORT: u32 = 256u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_EOF_SPECIFIED: u32 = 4u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_FILE_ATTRIBUTES_SPECIFIED: u32 = 32u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_GEN_FLAGS_SPECIFIED: u32 = 32768u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_MARK_USN_SOURCE_INFO: u32 = 2048u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_OPERATION_MASK: u32 = 255u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_OP_FLAGS_SPECIFIED: u32 = 128u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_REPARSE_POINT_SPECIFIED: u32 = 2u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SPARSE_SPECIFIED: u32 = 1u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_DIR_CHANGE_NOTIFY: u32 = 1024u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_FILE_ATTRIBUTE_INHERITANCE: u32 = 64u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_PARENT_TIMESTAMPS_UPDATE: u32 = 512u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_TIMESTAMPS_SPECIFIED: u32 = 16u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_VDL_SPECIFIED: u32 = 8u32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_WRITE_USN_CLOSE_RECORD: u32 = 4096u32;
pub const ATOMIC_CREATE_ECP_IN_OP_FLAG_CASE_SENSITIVE_FLAGS_SPECIFIED: u32 = 1u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_EOF_SET: u32 = 4u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTES_RETURNED: u32 = 512u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTES_SET: u32 = 32u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTE_INHERITANCE_SUPPRESSED: u32 = 64u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_OPERATION_MASK: u32 = 255u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_OP_FLAGS_HONORED: u32 = 128u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_REPARSE_POINT_SET: u32 = 2u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_SPARSE_SET: u32 = 1u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_TIMESTAMPS_RETURNED: u32 = 256u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_TIMESTAMPS_SET: u32 = 16u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_CLOSE_RECORD_WRITTEN: u32 = 2048u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_RETURNED: u32 = 4096u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_SOURCE_INFO_MARKED: u32 = 1024u32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_VDL_SET: u32 = 8u32;
pub const ATOMIC_CREATE_ECP_OUT_OP_FLAG_CASE_SENSITIVE_FLAGS_SET: u32 = 1u32;
pub const AuditAccessCheck: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(2i32);
pub const AuditCloseNonObject: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(9i32);
pub const AuditCloseObject: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(5i32);
pub const AuditDeleteObject: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(6i32);
pub const AuditHandleCreation: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(12i32);
pub const AuditObjectReference: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(11i32);
pub const AuditOpenNonObject: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(10i32);
pub const AuditOpenObject: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(3i32);
pub const AuditOpenObjectForDelete: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(7i32);
pub const AuditOpenObjectForDeleteWithTransaction: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(8i32);
pub const AuditOpenObjectWithTransaction: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(4i32);
pub const AuditPrivilegeObject: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(0i32);
pub const AuditPrivilegeService: SE_AUDIT_OPERATION = SE_AUDIT_OPERATION(1i32);
pub const CACHE_MANAGER_CALLBACKS_EX_V1: u32 = 1u32;
pub const CACHE_USE_DIRECT_ACCESS_MAPPING: u32 = 1u32;
pub const CACHE_VALID_FLAGS: u32 = 1u32;
pub const CC_ACQUIRE_DONT_WAIT: u32 = 1u32;
pub const CC_ACQUIRE_SUPPORTS_ASYNC_LAZYWRITE: u32 = 1u32;
pub const CC_AGGRESSIVE_UNMAP_BEHIND: u32 = 1u32;
pub const CC_DISABLE_DIRTY_PAGE_TRACKING: u32 = 8u32;
pub const CC_DISABLE_READ_AHEAD: u32 = 2u32;
pub const CC_DISABLE_UNMAP_BEHIND: u32 = 32u32;
pub const CC_DISABLE_WRITE_BEHIND: u32 = 4u32;
pub const CC_ENABLE_CPU_CACHE: u32 = 268435456u32;
pub const CC_ENABLE_DISK_IO_ACCOUNTING: u32 = 16u32;
pub const CC_FLUSH_AND_PURGE_GATHER_DIRTY_BITS: u32 = 2u32;
pub const CC_FLUSH_AND_PURGE_NO_PURGE: u32 = 1u32;
pub const CC_FLUSH_AND_PURGE_WRITEABLE_VIEWS_NOTSEEN: u32 = 4u32;
pub const COMPRESSION_ENGINE_MASK: u32 = 65280u32;
pub const COMPRESSION_ENGINE_MAX: u32 = 512u32;
pub const COMPRESSION_FORMAT_MASK: u32 = 255u32;
pub const COMPRESSION_FORMAT_MAX: u32 = 5u32;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_LAYER: u32 = 1u32;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_REGISTERED_LAYER: u32 = 4u32;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_REMOTE_LAYER: u32 = 8u32;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_SCRATCH: u32 = 2u32;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_USER_MODE: u32 = 16u32;
pub const CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
pub const ChangeDataControlArea: FSRTL_CHANGE_BACKING_TYPE = FSRTL_CHANGE_BACKING_TYPE(0i32);
pub const ChangeImageControlArea: FSRTL_CHANGE_BACKING_TYPE = FSRTL_CHANGE_BACKING_TYPE(1i32);
pub const ChangeSharedCacheMap: FSRTL_CHANGE_BACKING_TYPE = FSRTL_CHANGE_BACKING_TYPE(2i32);
pub const CsvCsvFsInternalFileObject: CSV_DOWN_LEVEL_FILE_TYPE = CSV_DOWN_LEVEL_FILE_TYPE(1i32);
pub const CsvDownLevelFileObject: CSV_DOWN_LEVEL_FILE_TYPE = CSV_DOWN_LEVEL_FILE_TYPE(0i32);
pub const DD_MUP_DEVICE_NAME: windows_core::PCWSTR = windows_core::w!("\\Device\\Mup");
pub const DEVICE_RESET_KEEP_STACK: u32 = 4u32;
pub const DEVICE_RESET_RESERVED_0: u32 = 1u32;
pub const DEVICE_RESET_RESERVED_1: u32 = 2u32;
pub const DO_BOOT_CRITICAL: u32 = 536870912u32;
pub const DO_BUFFERED_IO: u32 = 4u32;
pub const DO_BUS_ENUMERATED_DEVICE: u32 = 4096u32;
pub const DO_DAX_VOLUME: u32 = 268435456u32;
pub const DO_DEVICE_HAS_NAME: u32 = 64u32;
pub const DO_DEVICE_INITIALIZING: u32 = 128u32;
pub const DO_DEVICE_IRP_REQUIRES_EXTENSION: u32 = 134217728u32;
pub const DO_DEVICE_TO_BE_RESET: u32 = 67108864u32;
pub const DO_DIRECT_IO: u32 = 16u32;
pub const DO_DISALLOW_EXECUTE: u32 = 8388608u32;
pub const DO_EXCLUSIVE: u32 = 8u32;
pub const DO_FORCE_NEITHER_IO: u32 = 524288u32;
pub const DO_LONG_TERM_REQUESTS: u32 = 512u32;
pub const DO_LOW_PRIORITY_FILESYSTEM: u32 = 65536u32;
pub const DO_MAP_IO_BUFFER: u32 = 32u32;
pub const DO_NEVER_LAST_DEVICE: u32 = 1024u32;
pub const DO_NOT_PURGE_DIRTY_PAGES: u32 = 4u32;
pub const DO_NOT_RETRY_PURGE: u32 = 2u32;
pub const DO_POWER_INRUSH: u32 = 16384u32;
pub const DO_POWER_PAGABLE: u32 = 8192u32;
pub const DO_SHUTDOWN_REGISTERED: u32 = 2048u32;
pub const DO_SUPPORTS_PERSISTENT_ACLS: u32 = 131072u32;
pub const DO_SUPPORTS_TRANSACTIONS: u32 = 262144u32;
pub const DO_SYSTEM_BOOT_PARTITION: u32 = 256u32;
pub const DO_SYSTEM_CRITICAL_PARTITION: u32 = 4194304u32;
pub const DO_SYSTEM_SYSTEM_PARTITION: u32 = 2097152u32;
pub const DO_VERIFY_VOLUME: u32 = 2u32;
pub const DO_VOLUME_DEVICE_OBJECT: u32 = 1048576u32;
pub const DfsLinkTrackingInformation: LINK_TRACKING_INFORMATION_TYPE = LINK_TRACKING_INFORMATION_TYPE(1i32);
pub const EA_NAME_NETWORK_OPEN_ECP_INTEGRITY: windows_core::PCSTR = windows_core::s!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-INTEGRITY");
pub const EA_NAME_NETWORK_OPEN_ECP_INTEGRITY_U: windows_core::PCWSTR = windows_core::w!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-INTEGRITY");
pub const EA_NAME_NETWORK_OPEN_ECP_PRIVACY: windows_core::PCSTR = windows_core::s!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-PRIVACY");
pub const EA_NAME_NETWORK_OPEN_ECP_PRIVACY_U: windows_core::PCWSTR = windows_core::w!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-PRIVACY");
pub const ECP_OPEN_PARAMETERS_FLAG_FAIL_ON_CASE_SENSITIVE_DIR: u32 = 16u32;
pub const ECP_OPEN_PARAMETERS_FLAG_IGNORE_DIR_CASE_SENSITIVITY: u32 = 8u32;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_DELETE: u32 = 4u32;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_READ: u32 = 1u32;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_WRITE: u32 = 2u32;
pub const ECP_TYPE_CLFS_CREATE_CONTAINER: windows_core::GUID = windows_core::GUID::from_u128(0x8650c9fe_0cec_8bf6_bd1e_835956541090);
pub const ECP_TYPE_IO_STOP_ON_SYMLINK_FILTER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x940e5d56_1646_4d3c_87b6_577ec36a1466);
pub const ECP_TYPE_OPEN_REPARSE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x323eb6a8_affd_4d95_8230_863bce09d37a);
pub const EVENT_INCREMENT: u32 = 1u32;
pub const EqualTo: FSRTL_COMPARISON_RESULT = FSRTL_COMPARISON_RESULT(0i32);
pub const FILE_ACTION_ADDED_STREAM: u32 = 6u32;
pub const FILE_ACTION_ID_NOT_TUNNELLED: u32 = 10u32;
pub const FILE_ACTION_MODIFIED_STREAM: u32 = 8u32;
pub const FILE_ACTION_REMOVED_BY_DELETE: u32 = 9u32;
pub const FILE_ACTION_REMOVED_STREAM: u32 = 7u32;
pub const FILE_ACTION_TUNNELLED_ID_COLLISION: u32 = 11u32;
pub const FILE_CLEANUP_FILE_DELETED: u32 = 4u32;
pub const FILE_CLEANUP_FILE_REMAINS: u32 = 2u32;
pub const FILE_CLEANUP_LINK_DELETED: u32 = 8u32;
pub const FILE_CLEANUP_POSIX_STYLE_DELETE: u32 = 32u32;
pub const FILE_CLEANUP_STREAM_DELETED: u32 = 16u32;
pub const FILE_CLEANUP_UNKNOWN: u32 = 0u32;
pub const FILE_CLEANUP_WRONG_DEVICE: u32 = 1u32;
pub const FILE_COMPLETE_IF_OPLOCKED: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(256u32);
pub const FILE_CONTAINS_EXTENDED_CREATE_INFORMATION: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(268435456u32);
pub const FILE_CREATE: NTCREATEFILE_CREATE_DISPOSITION = NTCREATEFILE_CREATE_DISPOSITION(2u32);
pub const FILE_CREATE_TREE_CONNECTION: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(128u32);
pub const FILE_DELETE_ON_CLOSE: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(4096u32);
pub const FILE_DIRECTORY_FILE: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(1u32);
pub const FILE_DISALLOW_EXCLUSIVE: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(131072u32);
pub const FILE_DISPOSITION_DELETE: FILE_DISPOSITION_INFORMATION_EX_FLAGS = FILE_DISPOSITION_INFORMATION_EX_FLAGS(1u32);
pub const FILE_DISPOSITION_DO_NOT_DELETE: FILE_DISPOSITION_INFORMATION_EX_FLAGS = FILE_DISPOSITION_INFORMATION_EX_FLAGS(0u32);
pub const FILE_DISPOSITION_FORCE_IMAGE_SECTION_CHECK: FILE_DISPOSITION_INFORMATION_EX_FLAGS = FILE_DISPOSITION_INFORMATION_EX_FLAGS(4u32);
pub const FILE_DISPOSITION_IGNORE_READONLY_ATTRIBUTE: FILE_DISPOSITION_INFORMATION_EX_FLAGS = FILE_DISPOSITION_INFORMATION_EX_FLAGS(16u32);
pub const FILE_DISPOSITION_ON_CLOSE: FILE_DISPOSITION_INFORMATION_EX_FLAGS = FILE_DISPOSITION_INFORMATION_EX_FLAGS(8u32);
pub const FILE_DISPOSITION_POSIX_SEMANTICS: FILE_DISPOSITION_INFORMATION_EX_FLAGS = FILE_DISPOSITION_INFORMATION_EX_FLAGS(2u32);
pub const FILE_EA_TYPE_ASCII: u32 = 65533u32;
pub const FILE_EA_TYPE_ASN1: u32 = 65501u32;
pub const FILE_EA_TYPE_BINARY: u32 = 65534u32;
pub const FILE_EA_TYPE_BITMAP: u32 = 65531u32;
pub const FILE_EA_TYPE_EA: u32 = 65518u32;
pub const FILE_EA_TYPE_FAMILY_IDS: u32 = 65281u32;
pub const FILE_EA_TYPE_ICON: u32 = 65529u32;
pub const FILE_EA_TYPE_METAFILE: u32 = 65530u32;
pub const FILE_EA_TYPE_MVMT: u32 = 65503u32;
pub const FILE_EA_TYPE_MVST: u32 = 65502u32;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_OUTSIDE_TX: u32 = 4u32;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_TO_TX: u32 = 2u32;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_WRITELOCKED: u32 = 1u32;
pub const FILE_LINK_FORCE_RESIZE_SOURCE_SR: u32 = 256u32;
pub const FILE_LINK_FORCE_RESIZE_SR: u32 = 384u32;
pub const FILE_LINK_FORCE_RESIZE_TARGET_SR: u32 = 128u32;
pub const FILE_LINK_IGNORE_READONLY_ATTRIBUTE: u32 = 64u32;
pub const FILE_LINK_NO_DECREASE_AVAILABLE_SPACE: u32 = 32u32;
pub const FILE_LINK_NO_INCREASE_AVAILABLE_SPACE: u32 = 16u32;
pub const FILE_LINK_POSIX_SEMANTICS: u32 = 2u32;
pub const FILE_LINK_PRESERVE_AVAILABLE_SPACE: u32 = 48u32;
pub const FILE_LINK_REPLACE_IF_EXISTS: u32 = 1u32;
pub const FILE_LINK_SUPPRESS_STORAGE_RESERVE_INHERITANCE: u32 = 8u32;
pub const FILE_NEED_EA: u32 = 128u32;
pub const FILE_NON_DIRECTORY_FILE: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(64u32);
pub const FILE_NOTIFY_CHANGE_EA: u32 = 128u32;
pub const FILE_NOTIFY_CHANGE_NAME: u32 = 3u32;
pub const FILE_NOTIFY_CHANGE_STREAM_NAME: u32 = 512u32;
pub const FILE_NOTIFY_CHANGE_STREAM_SIZE: u32 = 1024u32;
pub const FILE_NOTIFY_CHANGE_STREAM_WRITE: u32 = 2048u32;
pub const FILE_NOTIFY_VALID_MASK: u32 = 4095u32;
pub const FILE_NO_COMPRESSION: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(32768u32);
pub const FILE_NO_EA_KNOWLEDGE: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(512u32);
pub const FILE_NO_INTERMEDIATE_BUFFERING: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(8u32);
pub const FILE_OPBATCH_BREAK_UNDERWAY: u32 = 9u32;
pub const FILE_OPEN: NTCREATEFILE_CREATE_DISPOSITION = NTCREATEFILE_CREATE_DISPOSITION(1u32);
pub const FILE_OPEN_BY_FILE_ID: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(8192u32);
pub const FILE_OPEN_FOR_BACKUP_INTENT: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(16384u32);
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(8388608u32);
pub const FILE_OPEN_IF: NTCREATEFILE_CREATE_DISPOSITION = NTCREATEFILE_CREATE_DISPOSITION(3u32);
pub const FILE_OPEN_NO_RECALL: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(4194304u32);
pub const FILE_OPEN_REPARSE_POINT: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(2097152u32);
pub const FILE_OPEN_REQUIRING_OPLOCK: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(65536u32);
pub const FILE_OPLOCK_BROKEN_TO_LEVEL_2: u32 = 7u32;
pub const FILE_OPLOCK_BROKEN_TO_NONE: u32 = 8u32;
pub const FILE_OVERWRITE: NTCREATEFILE_CREATE_DISPOSITION = NTCREATEFILE_CREATE_DISPOSITION(4u32);
pub const FILE_OVERWRITE_IF: NTCREATEFILE_CREATE_DISPOSITION = NTCREATEFILE_CREATE_DISPOSITION(5u32);
pub const FILE_PIPE_ACCEPT_REMOTE_CLIENTS: u32 = 0u32;
pub const FILE_PIPE_BYTE_STREAM_MODE: u32 = 0u32;
pub const FILE_PIPE_BYTE_STREAM_TYPE: u32 = 0u32;
pub const FILE_PIPE_CLIENT_END: u32 = 0u32;
pub const FILE_PIPE_CLOSING_STATE: u32 = 4u32;
pub const FILE_PIPE_COMPLETE_OPERATION: u32 = 1u32;
pub const FILE_PIPE_COMPUTER_NAME_LENGTH: u32 = 15u32;
pub const FILE_PIPE_CONNECTED_STATE: u32 = 3u32;
pub const FILE_PIPE_DISCONNECTED_STATE: u32 = 1u32;
pub const FILE_PIPE_FULL_DUPLEX: u32 = 2u32;
pub const FILE_PIPE_INBOUND: u32 = 0u32;
pub const FILE_PIPE_LISTENING_STATE: u32 = 2u32;
pub const FILE_PIPE_MESSAGE_MODE: u32 = 1u32;
pub const FILE_PIPE_MESSAGE_TYPE: u32 = 1u32;
pub const FILE_PIPE_OUTBOUND: u32 = 1u32;
pub const FILE_PIPE_QUEUE_OPERATION: u32 = 0u32;
pub const FILE_PIPE_READ_DATA: u32 = 0u32;
pub const FILE_PIPE_REJECT_REMOTE_CLIENTS: u32 = 2u32;
pub const FILE_PIPE_SERVER_END: u32 = 1u32;
pub const FILE_PIPE_SYMLINK_FLAG_GLOBAL: u32 = 1u32;
pub const FILE_PIPE_SYMLINK_FLAG_RELATIVE: u32 = 2u32;
pub const FILE_PIPE_TYPE_VALID_MASK: u32 = 3u32;
pub const FILE_PIPE_WRITE_SPACE: u32 = 1u32;
pub const FILE_RANDOM_ACCESS: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(2048u32);
pub const FILE_RENAME_FORCE_RESIZE_SOURCE_SR: u32 = 256u32;
pub const FILE_RENAME_FORCE_RESIZE_SR: u32 = 384u32;
pub const FILE_RENAME_FORCE_RESIZE_TARGET_SR: u32 = 128u32;
pub const FILE_RENAME_IGNORE_READONLY_ATTRIBUTE: u32 = 64u32;
pub const FILE_RENAME_NO_DECREASE_AVAILABLE_SPACE: u32 = 32u32;
pub const FILE_RENAME_NO_INCREASE_AVAILABLE_SPACE: u32 = 16u32;
pub const FILE_RENAME_POSIX_SEMANTICS: u32 = 2u32;
pub const FILE_RENAME_PRESERVE_AVAILABLE_SPACE: u32 = 48u32;
pub const FILE_RENAME_REPLACE_IF_EXISTS: u32 = 1u32;
pub const FILE_RENAME_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
pub const FILE_RENAME_SUPPRESS_STORAGE_RESERVE_INHERITANCE: u32 = 8u32;
pub const FILE_RESERVE_OPFILTER: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(1048576u32);
pub const FILE_SEQUENTIAL_ONLY: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(4u32);
pub const FILE_SESSION_AWARE: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(262144u32);
pub const FILE_SUPERSEDE: NTCREATEFILE_CREATE_DISPOSITION = NTCREATEFILE_CREATE_DISPOSITION(0u32);
pub const FILE_SYNCHRONOUS_IO_ALERT: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(16u32);
pub const FILE_SYNCHRONOUS_IO_NONALERT: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(32u32);
pub const FILE_VC_CONTENT_INDEX_DISABLED: u32 = 8u32;
pub const FILE_VC_LOG_QUOTA_LIMIT: u32 = 32u32;
pub const FILE_VC_LOG_QUOTA_THRESHOLD: u32 = 16u32;
pub const FILE_VC_LOG_VOLUME_LIMIT: u32 = 128u32;
pub const FILE_VC_LOG_VOLUME_THRESHOLD: u32 = 64u32;
pub const FILE_VC_QUOTAS_INCOMPLETE: u32 = 256u32;
pub const FILE_VC_QUOTAS_REBUILDING: u32 = 512u32;
pub const FILE_VC_QUOTA_ENFORCE: u32 = 2u32;
pub const FILE_VC_QUOTA_MASK: u32 = 3u32;
pub const FILE_VC_QUOTA_NONE: u32 = 0u32;
pub const FILE_VC_QUOTA_TRACK: u32 = 1u32;
pub const FILE_VC_VALID_MASK: u32 = 1023u32;
pub const FILE_WRITE_THROUGH: NTCREATEFILE_CREATE_OPTIONS = NTCREATEFILE_CREATE_OPTIONS(2u32);
pub const FLAGS_DELAY_REASONS_BITMAP_SCANNED: u32 = 2u32;
pub const FLAGS_DELAY_REASONS_LOG_FILE_FULL: u32 = 1u32;
pub const FLAGS_END_OF_FILE_INFO_EX_EXTEND_PAGING: u32 = 1u32;
pub const FLAGS_END_OF_FILE_INFO_EX_NO_EXTRA_PAGING_EXTEND: u32 = 2u32;
pub const FLAGS_END_OF_FILE_INFO_EX_TIME_CONSTRAINED: u32 = 4u32;
pub const FSCTL_LMR_GET_LINK_TRACKING_INFORMATION: u32 = 1310952u32;
pub const FSCTL_LMR_SET_LINK_TRACKING_INFORMATION: u32 = 1310956u32;
pub const FSCTL_MAILSLOT_PEEK: u32 = 802819u32;
pub const FSCTL_PIPE_ASSIGN_EVENT: u32 = 1114112u32;
pub const FSCTL_PIPE_CREATE_SYMLINK: u32 = 1114188u32;
pub const FSCTL_PIPE_DELETE_SYMLINK: u32 = 1114192u32;
pub const FSCTL_PIPE_DISABLE_IMPERSONATE: u32 = 1114180u32;
pub const FSCTL_PIPE_DISCONNECT: u32 = 1114116u32;
pub const FSCTL_PIPE_FLUSH: u32 = 1146944u32;
pub const FSCTL_PIPE_GET_CONNECTION_ATTRIBUTE: u32 = 1114160u32;
pub const FSCTL_PIPE_GET_HANDLE_ATTRIBUTE: u32 = 1114168u32;
pub const FSCTL_PIPE_GET_PIPE_ATTRIBUTE: u32 = 1114152u32;
pub const FSCTL_PIPE_IMPERSONATE: u32 = 1114140u32;
pub const FSCTL_PIPE_INTERNAL_READ: u32 = 1138676u32;
pub const FSCTL_PIPE_INTERNAL_READ_OVFLOW: u32 = 1138688u32;
pub const FSCTL_PIPE_INTERNAL_TRANSCEIVE: u32 = 1171455u32;
pub const FSCTL_PIPE_INTERNAL_WRITE: u32 = 1155064u32;
pub const FSCTL_PIPE_LISTEN: u32 = 1114120u32;
pub const FSCTL_PIPE_PEEK: u32 = 1130508u32;
pub const FSCTL_PIPE_QUERY_CLIENT_PROCESS: u32 = 1114148u32;
pub const FSCTL_PIPE_QUERY_CLIENT_PROCESS_V2: u32 = 1114196u32;
pub const FSCTL_PIPE_QUERY_EVENT: u32 = 1114128u32;
pub const FSCTL_PIPE_SET_CLIENT_PROCESS: u32 = 1114144u32;
pub const FSCTL_PIPE_SET_CONNECTION_ATTRIBUTE: u32 = 1114164u32;
pub const FSCTL_PIPE_SET_HANDLE_ATTRIBUTE: u32 = 1114172u32;
pub const FSCTL_PIPE_SET_PIPE_ATTRIBUTE: u32 = 1114156u32;
pub const FSCTL_PIPE_SILO_ARRIVAL: u32 = 1146952u32;
pub const FSCTL_PIPE_TRANSCEIVE: u32 = 1163287u32;
pub const FSCTL_PIPE_WAIT: u32 = 1114136u32;
pub const FSRTL_ADD_TC_CASE_SENSITIVE: u32 = 1u32;
pub const FSRTL_ADD_TC_KEY_BY_SHORT_NAME: u32 = 2u32;
pub const FSRTL_ALLOCATE_ECPLIST_FLAG_CHARGE_QUOTA: u32 = 1u32;
pub const FSRTL_ALLOCATE_ECP_FLAG_CHARGE_QUOTA: u32 = 1u32;
pub const FSRTL_ALLOCATE_ECP_FLAG_NONPAGED_POOL: u32 = 2u32;
pub const FSRTL_AUXILIARY_FLAG_DEALLOCATE: u32 = 1u32;
pub const FSRTL_CC_FLUSH_ERROR_FLAG_NO_HARD_ERROR: u32 = 1u32;
pub const FSRTL_CC_FLUSH_ERROR_FLAG_NO_LOG_ENTRY: u32 = 2u32;
pub const FSRTL_DRIVER_BACKING_FLAG_USE_PAGE_FILE: u32 = 1u32;
pub const FSRTL_ECP_LOOKASIDE_FLAG_NONPAGED_POOL: u32 = 2u32;
pub const FSRTL_FAT_LEGAL: u32 = 1u32;
pub const FSRTL_FCB_HEADER_V0: u32 = 0u32;
pub const FSRTL_FCB_HEADER_V1: u32 = 1u32;
pub const FSRTL_FCB_HEADER_V2: u32 = 2u32;
pub const FSRTL_FCB_HEADER_V3: u32 = 3u32;
pub const FSRTL_FCB_HEADER_V4: u32 = 4u32;
pub const FSRTL_FIND_TC_CASE_SENSITIVE: u32 = 1u32;
pub const FSRTL_FLAG2_BYPASSIO_STREAM_PAUSED: u32 = 32u32;
pub const FSRTL_FLAG2_DO_MODIFIED_WRITE: u32 = 1u32;
pub const FSRTL_FLAG2_IS_PAGING_FILE: u32 = 8u32;
pub const FSRTL_FLAG2_PURGE_WHEN_MAPPED: u32 = 4u32;
pub const FSRTL_FLAG2_SUPPORTS_FILTER_CONTEXTS: u32 = 2u32;
pub const FSRTL_FLAG2_WRITABLE_USER_MAPPED_FILE: u32 = 16u32;
pub const FSRTL_FLAG_ACQUIRE_MAIN_RSRC_EX: u32 = 8u32;
pub const FSRTL_FLAG_ACQUIRE_MAIN_RSRC_SH: u32 = 16u32;
pub const FSRTL_FLAG_ADVANCED_HEADER: u32 = 64u32;
pub const FSRTL_FLAG_EOF_ADVANCE_ACTIVE: u32 = 128u32;
pub const FSRTL_FLAG_FILE_LENGTH_CHANGED: u32 = 2u32;
pub const FSRTL_FLAG_FILE_MODIFIED: u32 = 1u32;
pub const FSRTL_FLAG_LIMIT_MODIFIED_PAGES: u32 = 4u32;
pub const FSRTL_FLAG_USER_MAPPED_FILE: u32 = 32u32;
pub const FSRTL_HPFS_LEGAL: u32 = 2u32;
pub const FSRTL_NTFS_LEGAL: u32 = 4u32;
pub const FSRTL_OLE_LEGAL: u32 = 16u32;
pub const FSRTL_UNC_HARDENING_CAPABILITIES_INTEGRITY: u32 = 2u32;
pub const FSRTL_UNC_HARDENING_CAPABILITIES_MUTUAL_AUTH: u32 = 1u32;
pub const FSRTL_UNC_HARDENING_CAPABILITIES_PRIVACY: u32 = 4u32;
pub const FSRTL_UNC_PROVIDER_FLAGS_CONTAINER_AWARE: u32 = 8u32;
pub const FSRTL_UNC_PROVIDER_FLAGS_CSC_ENABLED: u32 = 2u32;
pub const FSRTL_UNC_PROVIDER_FLAGS_DOMAIN_SVC_AWARE: u32 = 4u32;
pub const FSRTL_UNC_PROVIDER_FLAGS_MAILSLOTS_SUPPORTED: u32 = 1u32;
pub const FSRTL_UNC_REGISTRATION_CURRENT_VERSION: u32 = 513u32;
pub const FSRTL_UNC_REGISTRATION_VERSION_0200: u32 = 512u32;
pub const FSRTL_UNC_REGISTRATION_VERSION_0201: u32 = 513u32;
pub const FSRTL_VIRTDISK_FULLY_ALLOCATED: u32 = 1u32;
pub const FSRTL_VIRTDISK_NO_DRIVE_LETTER: u32 = 2u32;
pub const FSRTL_VOLUME_BACKGROUND_FORMAT: u32 = 14u32;
pub const FSRTL_VOLUME_CHANGE_SIZE: u32 = 13u32;
pub const FSRTL_VOLUME_DISMOUNT: u32 = 1u32;
pub const FSRTL_VOLUME_DISMOUNT_FAILED: u32 = 2u32;
pub const FSRTL_VOLUME_FORCED_CLOSED: u32 = 10u32;
pub const FSRTL_VOLUME_INFO_MAKE_COMPAT: u32 = 11u32;
pub const FSRTL_VOLUME_LOCK: u32 = 3u32;
pub const FSRTL_VOLUME_LOCK_FAILED: u32 = 4u32;
pub const FSRTL_VOLUME_MOUNT: u32 = 6u32;
pub const FSRTL_VOLUME_NEEDS_CHKDSK: u32 = 7u32;
pub const FSRTL_VOLUME_PREPARING_EJECT: u32 = 12u32;
pub const FSRTL_VOLUME_UNLOCK: u32 = 5u32;
pub const FSRTL_VOLUME_WEARING_OUT: u32 = 9u32;
pub const FSRTL_VOLUME_WORM_NEAR_FULL: u32 = 8u32;
pub const FSRTL_WILD_CHARACTER: u32 = 8u32;
pub const FS_FILTER_ACQUIRE_FOR_CC_FLUSH: u16 = 65531u16;
pub const FS_FILTER_ACQUIRE_FOR_MOD_WRITE: u16 = 65533u16;
pub const FS_FILTER_ACQUIRE_FOR_SECTION_SYNCHRONIZATION: u16 = 65535u16;
pub const FS_FILTER_QUERY_OPEN: u16 = 65529u16;
pub const FS_FILTER_RELEASE_FOR_CC_FLUSH: u16 = 65530u16;
pub const FS_FILTER_RELEASE_FOR_MOD_WRITE: u16 = 65532u16;
pub const FS_FILTER_RELEASE_FOR_SECTION_SYNCHRONIZATION: u16 = 65534u16;
pub const FS_FILTER_SECTION_SYNC_IMAGE_EXTENTS_ARE_NOT_RVA: u32 = 8u32;
pub const FS_FILTER_SECTION_SYNC_IN_FLAG_DONT_UPDATE_LAST_ACCESS: u32 = 1u32;
pub const FS_FILTER_SECTION_SYNC_IN_FLAG_DONT_UPDATE_LAST_WRITE: u32 = 2u32;
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_ASYNC_PARALLEL_IO: u32 = 1u32;
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_DIRECT_MAP_DATA: u32 = 2u32;
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_DIRECT_MAP_IMAGE: u32 = 4u32;
pub const FastIoIsNotPossible: FAST_IO_POSSIBLE = FAST_IO_POSSIBLE(0i32);
pub const FastIoIsPossible: FAST_IO_POSSIBLE = FAST_IO_POSSIBLE(1i32);
pub const FastIoIsQuestionable: FAST_IO_POSSIBLE = FAST_IO_POSSIBLE(2i32);
pub const FileAccessInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(8i32);
pub const FileAlignmentInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(17i32);
pub const FileAllInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(18i32);
pub const FileAllocationInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(19i32);
pub const FileAlternateNameInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(21i32);
pub const FileAttributeTagInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(35i32);
pub const FileBasicInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(4i32);
pub const FileBothDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(3i32);
pub const FileCaseSensitiveInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(71i32);
pub const FileCaseSensitiveInformationForceAccessCheck: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(75i32);
pub const FileCompletionInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(30i32);
pub const FileCompressionInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(28i32);
pub const FileDesiredStorageClassInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(67i32);
pub const FileDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(1i32);
pub const FileDispositionInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(13i32);
pub const FileDispositionInformationEx: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(64i32);
pub const FileEaInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(7i32);
pub const FileEndOfFileInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(20i32);
pub const FileFsAttributeInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(5i32);
pub const FileFsControlInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(6i32);
pub const FileFsDataCopyInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(12i32);
pub const FileFsDeviceInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(4i32);
pub const FileFsDriverPathInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(9i32);
pub const FileFsFullSizeInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(7i32);
pub const FileFsFullSizeInformationEx: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(14i32);
pub const FileFsLabelInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(2i32);
pub const FileFsMaximumInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(15i32);
pub const FileFsMetadataSizeInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(13i32);
pub const FileFsObjectIdInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(8i32);
pub const FileFsSectorSizeInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(11i32);
pub const FileFsSizeInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(3i32);
pub const FileFsVolumeFlagsInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(10i32);
pub const FileFsVolumeInformation: FS_INFORMATION_CLASS = FS_INFORMATION_CLASS(1i32);
pub const FileFullDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(2i32);
pub const FileFullEaInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(15i32);
pub const FileHardLinkFullIdInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(62i32);
pub const FileHardLinkInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(46i32);
pub const FileIdBothDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(37i32);
pub const FileIdExtdBothDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(63i32);
pub const FileIdExtdDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(60i32);
pub const FileIdFullDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(38i32);
pub const FileIdGlobalTxDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(50i32);
pub const FileIdInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(59i32);
pub const FileInternalInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(6i32);
pub const FileIoCompletionNotificationInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(41i32);
pub const FileIoPriorityHintInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(43i32);
pub const FileIoStatusBlockRangeInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(42i32);
pub const FileIsRemoteDeviceInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(51i32);
pub const FileKnownFolderInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(76i32);
pub const FileLinkInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(11i32);
pub const FileLinkInformationBypassAccessCheck: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(57i32);
pub const FileLinkInformationEx: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(72i32);
pub const FileLinkInformationExBypassAccessCheck: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(73i32);
pub const FileMailslotQueryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(26i32);
pub const FileMailslotSetInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(27i32);
pub const FileMaximumInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(77i32);
pub const FileMemoryPartitionInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(69i32);
pub const FileModeInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(16i32);
pub const FileMoveClusterInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(31i32);
pub const FileNameInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(9i32);
pub const FileNamesInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(12i32);
pub const FileNetworkOpenInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(34i32);
pub const FileNetworkPhysicalNameInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(49i32);
pub const FileNormalizedNameInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(48i32);
pub const FileNumaNodeInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(53i32);
pub const FileObjectIdInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(29i32);
pub const FilePipeInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(23i32);
pub const FilePipeLocalInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(24i32);
pub const FilePipeRemoteInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(25i32);
pub const FilePositionInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(14i32);
pub const FileProcessIdsUsingFileInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(47i32);
pub const FileQuotaInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(32i32);
pub const FileRemoteProtocolInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(55i32);
pub const FileRenameInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(10i32);
pub const FileRenameInformationBypassAccessCheck: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(56i32);
pub const FileRenameInformationEx: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(65i32);
pub const FileRenameInformationExBypassAccessCheck: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(66i32);
pub const FileReparsePointInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(33i32);
pub const FileReplaceCompletionInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(61i32);
pub const FileSfioReserveInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(44i32);
pub const FileSfioVolumeInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(45i32);
pub const FileShortNameInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(40i32);
pub const FileStandardInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(5i32);
pub const FileStandardLinkInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(54i32);
pub const FileStatInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(68i32);
pub const FileStatLxInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(70i32);
pub const FileStorageReserveIdInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(74i32);
pub const FileStreamInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(22i32);
pub const FileTrackingInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(36i32);
pub const FileUnusedInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(52i32);
pub const FileValidDataLengthInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(39i32);
pub const FileVolumeNameInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(58i32);
pub const GCR_ALLOW_LM: u32 = 4096u32;
pub const GCR_ALLOW_NO_TARGET: u32 = 8192u32;
pub const GCR_ALLOW_NTLM: u32 = 256u32;
pub const GCR_MACHINE_CREDENTIAL: u32 = 1024u32;
pub const GCR_NTLM3_PARMS: u32 = 32u32;
pub const GCR_TARGET_INFO: u32 = 64u32;
pub const GCR_USE_OEM_SET: u32 = 512u32;
pub const GCR_USE_OWF_PASSWORD: u32 = 2048u32;
pub const GCR_VSM_PROTECTED_PASSWORD: u32 = 16384u32;
pub const GENERATE_CLIENT_CHALLENGE: u32 = 16u32;
pub const GUID_ECP_ATOMIC_CREATE: windows_core::GUID = windows_core::GUID::from_u128(0x4720bd83_52ac_4104_a130_d1ec6a8cc8e5);
pub const GUID_ECP_CLOUDFILES_ATTRIBUTION: windows_core::GUID = windows_core::GUID::from_u128(0x2932ff52_8378_4fc1_8edb_6bdc8f602709);
pub const GUID_ECP_CREATE_REDIRECTION: windows_core::GUID = windows_core::GUID::from_u128(0x188d6bd6_a126_4fa8_bdf2_1ccdf896f3e0);
pub const GUID_ECP_CSV_DOWN_LEVEL_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0x4248be44_647f_488f_8be5_a08aaf70f028);
pub const GUID_ECP_CSV_QUERY_FILE_REVISION: windows_core::GUID = windows_core::GUID::from_u128(0x44aec90b_de65_4d46_8fbf_763f9d970b1d);
pub const GUID_ECP_CSV_QUERY_FILE_REVISION_FILE_ID_128: windows_core::GUID = windows_core::GUID::from_u128(0x7a3a4aa1_aa74_4bc6_b070_ab56a38c1fed);
pub const GUID_ECP_CSV_SET_HANDLE_PROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0x7a9fdd94_7b58_42bb_9740_3cb86983a615);
pub const GUID_ECP_DUAL_OPLOCK_KEY: windows_core::GUID = windows_core::GUID::from_u128(0x41621a14_b08b_4df1_b676_a05ffdf01bea);
pub const GUID_ECP_IO_DEVICE_HINT: windows_core::GUID = windows_core::GUID::from_u128(0xf315b732_ac6b_4d4d_be0c_b3126490e1a3);
pub const GUID_ECP_NETWORK_APP_INSTANCE: windows_core::GUID = windows_core::GUID::from_u128(0x6aa6bc45_a7ef_4af7_9008_fa462e144d74);
pub const GUID_ECP_NETWORK_APP_INSTANCE_VERSION: windows_core::GUID = windows_core::GUID::from_u128(0xb7d082b9_563b_4f07_a07b_524a8116a010);
pub const GUID_ECP_NETWORK_OPEN_CONTEXT: windows_core::GUID = windows_core::GUID::from_u128(0xc584edbf_00df_4d28_b884_35baca8911e8);
pub const GUID_ECP_NFS_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0xf326d30c_e5f8_4fe7_ab74_f5a3196d92db);
pub const GUID_ECP_OPEN_PARAMETERS: windows_core::GUID = windows_core::GUID::from_u128(0xcd0a93c3_3bb7_463d_accb_969d3435a5a5);
pub const GUID_ECP_OPLOCK_KEY: windows_core::GUID = windows_core::GUID::from_u128(0x48850596_3050_4be7_9863_fec350ce8d7f);
pub const GUID_ECP_PREFETCH_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0xe1777b21_847e_4837_aa45_64161d280655);
pub const GUID_ECP_QUERY_ON_CREATE: windows_core::GUID = windows_core::GUID::from_u128(0x1aca62e9_abb4_4ff2_bb5c_1c79025e417f);
pub const GUID_ECP_RKF_BYPASS: windows_core::GUID = windows_core::GUID::from_u128(0x02378cc6_f73c_489c_8282_564d1a99131b);
pub const GUID_ECP_SRV_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0xbebfaebc_aabf_489d_9d2c_e9e361102853);
pub const GreaterThan: FSRTL_COMPARISON_RESULT = FSRTL_COMPARISON_RESULT(1i32);
pub const HEAP_CLASS_0: u32 = 0u32;
pub const HEAP_CLASS_1: u32 = 4096u32;
pub const HEAP_CLASS_2: u32 = 8192u32;
pub const HEAP_CLASS_3: u32 = 12288u32;
pub const HEAP_CLASS_4: u32 = 16384u32;
pub const HEAP_CLASS_5: u32 = 20480u32;
pub const HEAP_CLASS_6: u32 = 24576u32;
pub const HEAP_CLASS_7: u32 = 28672u32;
pub const HEAP_CLASS_8: u32 = 32768u32;
pub const HEAP_CLASS_MASK: u32 = 61440u32;
pub const HEAP_CREATE_ALIGN_16: u32 = 65536u32;
pub const HEAP_CREATE_ENABLE_EXECUTE: u32 = 262144u32;
pub const HEAP_CREATE_ENABLE_TRACING: u32 = 131072u32;
pub const HEAP_CREATE_HARDENED: u32 = 512u32;
pub const HEAP_CREATE_SEGMENT_HEAP: u32 = 256u32;
pub const HEAP_DISABLE_COALESCE_ON_FREE: u32 = 128u32;
pub const HEAP_FREE_CHECKING_ENABLED: u32 = 64u32;
pub const HEAP_GENERATE_EXCEPTIONS: u32 = 4u32;
pub const HEAP_GLOBAL_TAG: u32 = 2048u32;
pub const HEAP_GROWABLE: u32 = 2u32;
pub const HEAP_MAXIMUM_TAG: u32 = 4095u32;
pub const HEAP_NO_SERIALIZE: u32 = 1u32;
pub const HEAP_PSEUDO_TAG_FLAG: u32 = 32768u32;
pub const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 16u32;
pub const HEAP_SETTABLE_USER_FLAG1: u32 = 512u32;
pub const HEAP_SETTABLE_USER_FLAG2: u32 = 1024u32;
pub const HEAP_SETTABLE_USER_FLAG3: u32 = 2048u32;
pub const HEAP_SETTABLE_USER_FLAGS: u32 = 3584u32;
pub const HEAP_SETTABLE_USER_VALUE: u32 = 256u32;
pub const HEAP_TAG_SHIFT: u32 = 18u32;
pub const HEAP_TAIL_CHECKING_ENABLED: u32 = 32u32;
pub const HEAP_ZERO_MEMORY: u32 = 8u32;
pub const HeapMemoryBasicInformation: HEAP_MEMORY_INFO_CLASS = HEAP_MEMORY_INFO_CLASS(0i32);
pub const INVALID_PROCESSOR_INDEX: u32 = 4294967295u32;
pub const IOCTL_LMR_ARE_FILE_OBJECTS_ON_SAME_SERVER: u32 = 1310960u32;
pub const IOCTL_REDIR_QUERY_PATH: u32 = 1311119u32;
pub const IOCTL_REDIR_QUERY_PATH_EX: u32 = 1311123u32;
pub const IOCTL_VOLSNAP_FLUSH_AND_HOLD_WRITES: u32 = 5488640u32;
pub const IO_CD_ROM_INCREMENT: u32 = 1u32;
pub const IO_CREATE_STREAM_FILE_LITE: u32 = 2u32;
pub const IO_CREATE_STREAM_FILE_RAISE_ON_ERROR: u32 = 1u32;
pub const IO_DISK_INCREMENT: u32 = 1u32;
pub const IO_FILE_OBJECT_NON_PAGED_POOL_CHARGE: u32 = 64u32;
pub const IO_FILE_OBJECT_PAGED_POOL_CHARGE: u32 = 1024u32;
pub const IO_IGNORE_READONLY_ATTRIBUTE: u32 = 64u32;
pub const IO_MAILSLOT_INCREMENT: u32 = 2u32;
pub const IO_MM_PAGING_FILE: u32 = 16u32;
pub const IO_NAMED_PIPE_INCREMENT: u32 = 2u32;
pub const IO_NETWORK_INCREMENT: u32 = 2u32;
pub const IO_NO_INCREMENT: u32 = 0u32;
pub const IO_OPEN_PAGING_FILE: u32 = 2u32;
pub const IO_OPEN_TARGET_DIRECTORY: u32 = 4u32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_0: i32 = 96i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_1: i32 = 97i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_2: i32 = 98i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_3: i32 = 99i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_4: i32 = 100i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_5: i32 = 101i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_6: i32 = 102i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_7: i32 = 103i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_8: i32 = 104i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_9: i32 = 105i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_A: i32 = 106i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_B: i32 = 107i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_C: i32 = 108i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_D: i32 = 109i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_E: i32 = 110i32;
pub const IO_REPARSE_TAG_ACRONIS_HSM_F: i32 = 111i32;
pub const IO_REPARSE_TAG_ACTIVISION_HSM: i32 = 71i32;
pub const IO_REPARSE_TAG_ADA_HSM: i32 = 38i32;
pub const IO_REPARSE_TAG_ADOBE_HSM: i32 = 69i32;
pub const IO_REPARSE_TAG_ALERTBOOT: i32 = 536870988i32;
pub const IO_REPARSE_TAG_ALTIRIS_HSM: i32 = 25i32;
pub const IO_REPARSE_TAG_APPXSTRM: i32 = -1073741804i32;
pub const IO_REPARSE_TAG_ARCO_BACKUP: i32 = 59i32;
pub const IO_REPARSE_TAG_ARKIVIO: i32 = 12i32;
pub const IO_REPARSE_TAG_AURISTOR_FS: i32 = 73i32;
pub const IO_REPARSE_TAG_AUTN_HSM: i32 = 39i32;
pub const IO_REPARSE_TAG_BRIDGEHEAD_HSM: i32 = 22i32;
pub const IO_REPARSE_TAG_C2CSYSTEMS_HSM: i32 = 49i32;
pub const IO_REPARSE_TAG_CARINGO_HSM: i32 = 52i32;
pub const IO_REPARSE_TAG_CARROLL_HSM: i32 = 60i32;
pub const IO_REPARSE_TAG_CITRIX_PM: i32 = 54i32;
pub const IO_REPARSE_TAG_COMMVAULT: i32 = 14i32;
pub const IO_REPARSE_TAG_COMMVAULT_HSM: i32 = 29i32;
pub const IO_REPARSE_TAG_COMTRADE_HSM: i32 = 61i32;
pub const IO_REPARSE_TAG_CTERA_HSM: i32 = 78i32;
pub const IO_REPARSE_TAG_DATAFIRST_HSM: i32 = 48i32;
pub const IO_REPARSE_TAG_DATAGLOBAL_HSM: i32 = 46i32;
pub const IO_REPARSE_TAG_DATASTOR_SIS: i32 = 30i32;
pub const IO_REPARSE_TAG_DFM: i32 = -2147483626i32;
pub const IO_REPARSE_TAG_DOR_HSM: i32 = 82i32;
pub const IO_REPARSE_TAG_DOUBLE_TAKE_HSM: i32 = 34i32;
pub const IO_REPARSE_TAG_DOUBLE_TAKE_SIS: i32 = 41i32;
pub const IO_REPARSE_TAG_DRIVE_EXTENDER: i32 = -2147483643i32;
pub const IO_REPARSE_TAG_DROPBOX_HSM: i32 = 68i32;
pub const IO_REPARSE_TAG_EASEFILTER_HSM: i32 = 87i32;
pub const IO_REPARSE_TAG_EASEVAULT_HSM: i32 = 62i32;
pub const IO_REPARSE_TAG_EDSI_HSM: i32 = 31i32;
pub const IO_REPARSE_TAG_ELTAN_HSM: i32 = 43i32;
pub const IO_REPARSE_TAG_EMC_HSM: i32 = 57i32;
pub const IO_REPARSE_TAG_ENIGMA_HSM: i32 = 17i32;
pub const IO_REPARSE_TAG_FILTER_MANAGER: i32 = -2147483637i32;
pub const IO_REPARSE_TAG_GLOBAL360_HSM: i32 = 24i32;
pub const IO_REPARSE_TAG_GOOGLE_HSM: i32 = 65i32;
pub const IO_REPARSE_TAG_GRAU_DATASTORAGE_HSM: i32 = 28i32;
pub const IO_REPARSE_TAG_HDS_HCP_HSM: i32 = 72i32;
pub const IO_REPARSE_TAG_HDS_HSM: i32 = 63i32;
pub const IO_REPARSE_TAG_HERMES_HSM: i32 = 26i32;
pub const IO_REPARSE_TAG_HP_BACKUP: i32 = 67i32;
pub const IO_REPARSE_TAG_HP_DATA_PROTECT: i32 = 70i32;
pub const IO_REPARSE_TAG_HP_HSM: i32 = 32i32;
pub const IO_REPARSE_TAG_HSAG_HSM: i32 = 37i32;
pub const IO_REPARSE_TAG_HUBSTOR_HSM: i32 = 85i32;
pub const IO_REPARSE_TAG_IFSTEST_CONGRUENT: i32 = 9i32;
pub const IO_REPARSE_TAG_IIS_CACHE: i32 = -1610612720i32;
pub const IO_REPARSE_TAG_IMANAGE_HSM: i32 = 536870998i32;
pub const IO_REPARSE_TAG_INTERCOPE_HSM: i32 = 19i32;
pub const IO_REPARSE_TAG_ITSTATION: i32 = 74i32;
pub const IO_REPARSE_TAG_KOM_NETWORKS_HSM: i32 = 20i32;
pub const IO_REPARSE_TAG_LX_BLK: i32 = -2147483610i32;
pub const IO_REPARSE_TAG_LX_CHR: i32 = -2147483611i32;
pub const IO_REPARSE_TAG_LX_FIFO: i32 = -2147483612i32;
pub const IO_REPARSE_TAG_LX_SYMLINK: i32 = -1610612707i32;
pub const IO_REPARSE_TAG_MAGINATICS_RDR: i32 = 64i32;
pub const IO_REPARSE_TAG_MAXISCALE_HSM: i32 = 536870965i32;
pub const IO_REPARSE_TAG_MEMORY_TECH_HSM: i32 = 21i32;
pub const IO_REPARSE_TAG_MIMOSA_HSM: i32 = 36i32;
pub const IO_REPARSE_TAG_MOONWALK_HSM: i32 = 10i32;
pub const IO_REPARSE_TAG_MTALOS: i32 = 77i32;
pub const IO_REPARSE_TAG_NEUSHIELD: i32 = 81i32;
pub const IO_REPARSE_TAG_NEXSAN_HSM: i32 = 40i32;
pub const IO_REPARSE_TAG_NIPPON_HSM: i32 = 79i32;
pub const IO_REPARSE_TAG_NVIDIA_UNIONFS: i32 = 536870996i32;
pub const IO_REPARSE_TAG_OPENAFS_DFS: i32 = 55i32;
pub const IO_REPARSE_TAG_OSR_SAMPLE: i32 = 536870935i32;
pub const IO_REPARSE_TAG_OVERTONE: i32 = 15i32;
pub const IO_REPARSE_TAG_POINTSOFT_HSM: i32 = 27i32;
pub const IO_REPARSE_TAG_QI_TECH_HSM: i32 = 536870959i32;
pub const IO_REPARSE_TAG_QUADDRA_HSM: i32 = 66i32;
pub const IO_REPARSE_TAG_QUEST_HSM: i32 = 45i32;
pub const IO_REPARSE_TAG_REDSTOR_HSM: i32 = 80i32;
pub const IO_REPARSE_TAG_RIVERBED_HSM: i32 = 51i32;
pub const IO_REPARSE_TAG_SER_HSM: i32 = 33i32;
pub const IO_REPARSE_TAG_SHX_BACKUP: i32 = 83i32;
pub const IO_REPARSE_TAG_SOLUTIONSOFT: i32 = 536870925i32;
pub const IO_REPARSE_TAG_SONY_HSM: i32 = 42i32;
pub const IO_REPARSE_TAG_SPHARSOFT: i32 = 75i32;
pub const IO_REPARSE_TAG_SYMANTEC_HSM: i32 = 18i32;
pub const IO_REPARSE_TAG_SYMANTEC_HSM2: i32 = 16i32;
pub const IO_REPARSE_TAG_TSINGHUA_UNIVERSITY_RESEARCH: i32 = 11i32;
pub const IO_REPARSE_TAG_UTIXO_HSM: i32 = 44i32;
pub const IO_REPARSE_TAG_VALID_VALUES: u32 = 4026597375u32;
pub const IO_REPARSE_TAG_VMWARE_PM: i32 = 58i32;
pub const IO_REPARSE_TAG_WATERFORD: i32 = 50i32;
pub const IO_REPARSE_TAG_WISDATA_HSM: i32 = 35i32;
pub const IO_REPARSE_TAG_ZLTI_HSM: i32 = 56i32;
pub const IO_STOP_ON_SYMLINK: u32 = 8u32;
pub const KnownFolderDesktop: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(1i32);
pub const KnownFolderDocuments: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(2i32);
pub const KnownFolderDownloads: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(3i32);
pub const KnownFolderMax: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(7i32);
pub const KnownFolderMusic: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(4i32);
pub const KnownFolderNone: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(0i32);
pub const KnownFolderOther: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(7i32);
pub const KnownFolderPictures: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(5i32);
pub const KnownFolderVideos: FILE_KNOWN_FOLDER_TYPE = FILE_KNOWN_FOLDER_TYPE(6i32);
pub const LCN_CHECKSUM_VALID: _LCN_WEAK_REFERENCE_STATE = _LCN_WEAK_REFERENCE_STATE(2i32);
pub const LCN_WEAK_REFERENCE_VALID: _LCN_WEAK_REFERENCE_STATE = _LCN_WEAK_REFERENCE_STATE(1i32);
pub const LX_FILE_CASE_SENSITIVE_DIR: u32 = 16u32;
pub const LX_FILE_METADATA_DEVICE_ID_EA_NAME: windows_core::PCSTR = windows_core::s!("$LXDEV");
pub const LX_FILE_METADATA_GID_EA_NAME: windows_core::PCSTR = windows_core::s!("$LXGID");
pub const LX_FILE_METADATA_HAS_DEVICE_ID: u32 = 8u32;
pub const LX_FILE_METADATA_HAS_GID: u32 = 2u32;
pub const LX_FILE_METADATA_HAS_MODE: u32 = 4u32;
pub const LX_FILE_METADATA_HAS_UID: u32 = 1u32;
pub const LX_FILE_METADATA_MODE_EA_NAME: windows_core::PCSTR = windows_core::s!("$LXMOD");
pub const LX_FILE_METADATA_UID_EA_NAME: windows_core::PCSTR = windows_core::s!("$LXUID");
pub const LessThan: FSRTL_COMPARISON_RESULT = FSRTL_COMPARISON_RESULT(-1i32);
pub const MAP_DISABLE_PAGEFAULT_CLUSTERING: u32 = 256u32;
pub const MAP_HIGH_PRIORITY: u32 = 64u32;
pub const MAP_NO_READ: u32 = 16u32;
pub const MAP_WAIT: u32 = 1u32;
pub const MAXIMUM_LEADBYTES: u32 = 12u32;
pub const MAX_UNICODE_STACK_BUFFER_LENGTH: u32 = 256u32;
pub const MCB_FLAG_RAISE_ON_ALLOCATION_FAILURE: u32 = 1u32;
pub const MM_FORCE_CLOSED_DATA: u32 = 1u32;
pub const MM_FORCE_CLOSED_IMAGE: u32 = 2u32;
pub const MM_FORCE_CLOSED_LATER_OK: u32 = 4u32;
pub const MM_IS_FILE_SECTION_ACTIVE_DATA: u32 = 2u32;
pub const MM_IS_FILE_SECTION_ACTIVE_IMAGE: u32 = 1u32;
pub const MM_IS_FILE_SECTION_ACTIVE_USER: u32 = 4u32;
pub const MemoryBasicInformation: MEMORY_INFORMATION_CLASS = MEMORY_INFORMATION_CLASS(0i32);
pub const MemoryType64KPage: RTL_MEMORY_TYPE = RTL_MEMORY_TYPE(2i32);
pub const MemoryTypeCustom: RTL_MEMORY_TYPE = RTL_MEMORY_TYPE(5i32);
pub const MemoryTypeHugePage: RTL_MEMORY_TYPE = RTL_MEMORY_TYPE(4i32);
pub const MemoryTypeLargePage: RTL_MEMORY_TYPE = RTL_MEMORY_TYPE(3i32);
pub const MemoryTypeMax: RTL_MEMORY_TYPE = RTL_MEMORY_TYPE(6i32);
pub const MemoryTypeNonPaged: RTL_MEMORY_TYPE = RTL_MEMORY_TYPE(1i32);
pub const MemoryTypePaged: RTL_MEMORY_TYPE = RTL_MEMORY_TYPE(0i32);
pub const MmFlushForDelete: MMFLUSH_TYPE = MMFLUSH_TYPE(0i32);
pub const MmFlushForWrite: MMFLUSH_TYPE = MMFLUSH_TYPE(1i32);
pub const MsvAvChannelBindings: MSV1_0_AVID = MSV1_0_AVID(10i32);
pub const MsvAvDnsComputerName: MSV1_0_AVID = MSV1_0_AVID(3i32);
pub const MsvAvDnsDomainName: MSV1_0_AVID = MSV1_0_AVID(4i32);
pub const MsvAvDnsTreeName: MSV1_0_AVID = MSV1_0_AVID(5i32);
pub const MsvAvEOL: MSV1_0_AVID = MSV1_0_AVID(0i32);
pub const MsvAvFlags: MSV1_0_AVID = MSV1_0_AVID(6i32);
pub const MsvAvNbComputerName: MSV1_0_AVID = MSV1_0_AVID(1i32);
pub const MsvAvNbDomainName: MSV1_0_AVID = MSV1_0_AVID(2i32);
pub const MsvAvRestrictions: MSV1_0_AVID = MSV1_0_AVID(8i32);
pub const MsvAvTargetName: MSV1_0_AVID = MSV1_0_AVID(9i32);
pub const MsvAvTimestamp: MSV1_0_AVID = MSV1_0_AVID(7i32);
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_HANDLE_COLLAPSING: u32 = 1u32;
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_HANDLE_DURABILITY: u32 = 2u32;
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_OPLOCKS: u32 = 4u32;
pub const NETWORK_OPEN_ECP_IN_FLAG_FORCE_BUFFERED_SYNCHRONOUS_IO_HACK: u32 = 2147483648u32;
pub const NETWORK_OPEN_ECP_IN_FLAG_FORCE_MAX_EOF_HACK: u32 = 1073741824u32;
pub const NETWORK_OPEN_ECP_IN_FLAG_REQ_MUTUAL_AUTH: u32 = 8u32;
pub const NETWORK_OPEN_ECP_OUT_FLAG_RET_MUTUAL_AUTH: u32 = 8u32;
pub const NO_8DOT3_NAME_PRESENT: u32 = 1u32;
pub const NetworkOpenIntegrityAny: NETWORK_OPEN_INTEGRITY_QUALIFIER = NETWORK_OPEN_INTEGRITY_QUALIFIER(0i32);
pub const NetworkOpenIntegrityEncrypted: NETWORK_OPEN_INTEGRITY_QUALIFIER = NETWORK_OPEN_INTEGRITY_QUALIFIER(3i32);
pub const NetworkOpenIntegrityMaximum: NETWORK_OPEN_INTEGRITY_QUALIFIER = NETWORK_OPEN_INTEGRITY_QUALIFIER(4i32);
pub const NetworkOpenIntegrityNone: NETWORK_OPEN_INTEGRITY_QUALIFIER = NETWORK_OPEN_INTEGRITY_QUALIFIER(1i32);
pub const NetworkOpenIntegritySigned: NETWORK_OPEN_INTEGRITY_QUALIFIER = NETWORK_OPEN_INTEGRITY_QUALIFIER(2i32);
pub const NetworkOpenLocationAny: NETWORK_OPEN_LOCATION_QUALIFIER = NETWORK_OPEN_LOCATION_QUALIFIER(0i32);
pub const NetworkOpenLocationLoopback: NETWORK_OPEN_LOCATION_QUALIFIER = NETWORK_OPEN_LOCATION_QUALIFIER(2i32);
pub const NetworkOpenLocationRemote: NETWORK_OPEN_LOCATION_QUALIFIER = NETWORK_OPEN_LOCATION_QUALIFIER(1i32);
pub const NotifyTypeCreate: FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = FS_FILTER_STREAM_FO_NOTIFICATION_TYPE(0i32);
pub const NotifyTypeRetired: FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = FS_FILTER_STREAM_FO_NOTIFICATION_TYPE(1i32);
pub const NtfsLinkTrackingInformation: LINK_TRACKING_INFORMATION_TYPE = LINK_TRACKING_INFORMATION_TYPE(0i32);
pub const OPEN_REPARSE_POINT_OVERRIDE_CREATE_OPTION: u32 = 64u32;
pub const OPEN_REPARSE_POINT_REPARSE_ALWAYS: u32 = 126u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_CHILD_EXISTS: u32 = 2u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_CHILD_NOT_EXISTS: u32 = 4u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_DIRECTORY_FINAL_COMPONENT: u32 = 8u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_DIRECTORY_FINAL_COMPONENT_ALWAYS: u32 = 72u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_FINAL_COMPONENT: u32 = 40u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_FINAL_COMPONENT_ALWAYS: u32 = 104u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_FINAL_COMPONENT: u32 = 32u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_FINAL_COMPONENT_ALWAYS: u32 = 96u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_NON_FINAL_COMPONENT: u32 = 16u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_NON_FINAL_COMPONENT_ALWAYS: u32 = 80u32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_FINAL_COMPONENT: u32 = 22u32;
pub const OPEN_REPARSE_POINT_RETURN_REPARSE_DATA_BUFFER: u32 = 128u32;
pub const OPEN_REPARSE_POINT_TAG_ENCOUNTERED: u32 = 1u32;
pub const OPEN_REPARSE_POINT_VERSION_EX: u32 = 2147483648u32;
pub const OPLOCK_FLAG_BACK_OUT_ATOMIC_OPLOCK: u32 = 4u32;
pub const OPLOCK_FLAG_BREAKING_FOR_SHARING_VIOLATION: u32 = 128u32;
pub const OPLOCK_FLAG_CLOSING_DELETE_ON_CLOSE: u32 = 32u32;
pub const OPLOCK_FLAG_COMPLETE_IF_OPLOCKED: u32 = 1u32;
pub const OPLOCK_FLAG_IGNORE_OPLOCK_KEYS: u32 = 8u32;
pub const OPLOCK_FLAG_OPLOCK_KEY_CHECK_ONLY: u32 = 2u32;
pub const OPLOCK_FLAG_PARENT_OBJECT: u32 = 16u32;
pub const OPLOCK_FLAG_REMOVING_FILE_OR_LINK: u32 = 64u32;
pub const OPLOCK_FSCTRL_FLAG_ALL_KEYS_MATCH: u32 = 1u32;
pub const OPLOCK_NOTIFY_BREAK_WAIT_INTERIM_TIMEOUT: OPLOCK_NOTIFY_REASON = OPLOCK_NOTIFY_REASON(0i32);
pub const OPLOCK_NOTIFY_BREAK_WAIT_TERMINATED: OPLOCK_NOTIFY_REASON = OPLOCK_NOTIFY_REASON(1i32);
pub const OPLOCK_UPPER_FLAG_CHECK_NO_BREAK: u32 = 65536u32;
pub const OPLOCK_UPPER_FLAG_NOTIFY_REFRESH_READ: u32 = 131072u32;
pub const PIN_CALLER_TRACKS_DIRTY_DATA: u32 = 32u32;
pub const PIN_EXCLUSIVE: u32 = 2u32;
pub const PIN_HIGH_PRIORITY: u32 = 64u32;
pub const PIN_IF_BCB: u32 = 8u32;
pub const PIN_NO_READ: u32 = 4u32;
pub const PIN_VERIFY_REQUIRED: u32 = 128u32;
pub const PIN_WAIT: u32 = 1u32;
pub const POLICY_AUDIT_SUBCATEGORY_COUNT: u32 = 59u32;
pub const PO_CB_AC_STATUS: u32 = 1u32;
pub const PO_CB_BUTTON_COLLISION: u32 = 2u32;
pub const PO_CB_LID_SWITCH_STATE: u32 = 4u32;
pub const PO_CB_PROCESSOR_POWER_POLICY: u32 = 5u32;
pub const PO_CB_SYSTEM_POWER_POLICY: u32 = 0u32;
pub const PO_CB_SYSTEM_STATE_LOCK: u32 = 3u32;
pub const PSMP_MAXIMUM_SYSAPP_CLAIM_VALUES: u32 = 4u32;
pub const PSMP_MINIMUM_SYSAPP_CLAIM_VALUES: u32 = 2u32;
pub const PURGE_WITH_ACTIVE_VIEWS: u32 = 8u32;
pub const QUERY_DIRECT_ACCESS_DATA_EXTENTS: u32 = 2u32;
pub const QUERY_DIRECT_ACCESS_IMAGE_EXTENTS: u32 = 1u32;
pub const QoCFileEaInformation: u32 = 4u32;
pub const QoCFileLxInformation: u32 = 2u32;
pub const QoCFileStatInformation: u32 = 1u32;
pub const REFS_COMPRESSION_FORMAT_LZ4: REFS_COMPRESSION_FORMATS = REFS_COMPRESSION_FORMATS(1i32);
pub const REFS_COMPRESSION_FORMAT_MAX: REFS_COMPRESSION_FORMATS = REFS_COMPRESSION_FORMATS(3i32);
pub const REFS_COMPRESSION_FORMAT_UNCOMPRESSED: REFS_COMPRESSION_FORMATS = REFS_COMPRESSION_FORMATS(0i32);
pub const REFS_COMPRESSION_FORMAT_ZSTD: REFS_COMPRESSION_FORMATS = REFS_COMPRESSION_FORMATS(2i32);
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_CAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = REFS_DEALLOCATE_RANGES_ALLOCATOR(2i32);
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_MAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = REFS_DEALLOCATE_RANGES_ALLOCATOR(3i32);
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_NONE: REFS_DEALLOCATE_RANGES_ALLOCATOR = REFS_DEALLOCATE_RANGES_ALLOCATOR(0i32);
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_SAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = REFS_DEALLOCATE_RANGES_ALLOCATOR(1i32);
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_COMPRESS_SYNC: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS(1i32);
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_MAX: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS(1i32);
pub const REFS_STREAM_EXTENT_PROPERTY_CRC32: _REFS_STREAM_EXTENT_PROPERTIES = _REFS_STREAM_EXTENT_PROPERTIES(128i32);
pub const REFS_STREAM_EXTENT_PROPERTY_CRC64: _REFS_STREAM_EXTENT_PROPERTIES = _REFS_STREAM_EXTENT_PROPERTIES(256i32);
pub const REFS_STREAM_EXTENT_PROPERTY_GHOSTED: _REFS_STREAM_EXTENT_PROPERTIES = _REFS_STREAM_EXTENT_PROPERTIES(512i32);
pub const REFS_STREAM_EXTENT_PROPERTY_READONLY: _REFS_STREAM_EXTENT_PROPERTIES = _REFS_STREAM_EXTENT_PROPERTIES(1024i32);
pub const REFS_STREAM_EXTENT_PROPERTY_SPARSE: _REFS_STREAM_EXTENT_PROPERTIES = _REFS_STREAM_EXTENT_PROPERTIES(8i32);
pub const REFS_STREAM_EXTENT_PROPERTY_STREAM_RESERVED: _REFS_STREAM_EXTENT_PROPERTIES = _REFS_STREAM_EXTENT_PROPERTIES(32i32);
pub const REFS_STREAM_EXTENT_PROPERTY_VALID: _REFS_STREAM_EXTENT_PROPERTIES = _REFS_STREAM_EXTENT_PROPERTIES(16i32);
pub const REFS_STREAM_SNAPSHOT_OPERATION_CLEAR_SHADOW_BTREE: REFS_STREAM_SNAPSHOT_OPERATION = REFS_STREAM_SNAPSHOT_OPERATION(6i32);
pub const REFS_STREAM_SNAPSHOT_OPERATION_CREATE: REFS_STREAM_SNAPSHOT_OPERATION = REFS_STREAM_SNAPSHOT_OPERATION(1i32);
pub const REFS_STREAM_SNAPSHOT_OPERATION_INVALID: REFS_STREAM_SNAPSHOT_OPERATION = REFS_STREAM_SNAPSHOT_OPERATION(0i32);
pub const REFS_STREAM_SNAPSHOT_OPERATION_LIST: REFS_STREAM_SNAPSHOT_OPERATION = REFS_STREAM_SNAPSHOT_OPERATION(2i32);
pub const REFS_STREAM_SNAPSHOT_OPERATION_MAX: REFS_STREAM_SNAPSHOT_OPERATION = REFS_STREAM_SNAPSHOT_OPERATION(6i32);
pub const REFS_STREAM_SNAPSHOT_OPERATION_QUERY_DELTAS: REFS_STREAM_SNAPSHOT_OPERATION = REFS_STREAM_SNAPSHOT_OPERATION(3i32);
pub const REFS_STREAM_SNAPSHOT_OPERATION_REVERT: REFS_STREAM_SNAPSHOT_OPERATION = REFS_STREAM_SNAPSHOT_OPERATION(4i32);
pub const REFS_STREAM_SNAPSHOT_OPERATION_SET_SHADOW_BTREE: REFS_STREAM_SNAPSHOT_OPERATION = REFS_STREAM_SNAPSHOT_OPERATION(5i32);
pub const REMOTE_PROTOCOL_FLAG_INTEGRITY: u32 = 16u32;
pub const REMOTE_PROTOCOL_FLAG_LOOPBACK: u32 = 1u32;
pub const REMOTE_PROTOCOL_FLAG_MUTUAL_AUTH: u32 = 32u32;
pub const REMOTE_PROTOCOL_FLAG_OFFLINE: u32 = 2u32;
pub const REMOTE_PROTOCOL_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
pub const REMOTE_PROTOCOL_FLAG_PRIVACY: u32 = 8u32;
pub const REMOVED_8DOT3_NAME: u32 = 2u32;
pub const REPARSE_DATA_EX_FLAG_GIVEN_TAG_OR_NONE: u32 = 1u32;
pub const RETURN_NON_NT_USER_SESSION_KEY: u32 = 8u32;
pub const RETURN_PRIMARY_LOGON_DOMAINNAME: u32 = 4u32;
pub const RETURN_PRIMARY_USERNAME: u32 = 2u32;
pub const RETURN_RESERVED_PARAMETER: u32 = 128u32;
pub const RPI_SMB2_SERVERCAP_DFS: u32 = 1u32;
pub const RPI_SMB2_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
pub const RPI_SMB2_SERVERCAP_ENCRYPTION_AWARE: u32 = 64u32;
pub const RPI_SMB2_SERVERCAP_LARGEMTU: u32 = 4u32;
pub const RPI_SMB2_SERVERCAP_LEASING: u32 = 2u32;
pub const RPI_SMB2_SERVERCAP_MULTICHANNEL: u32 = 8u32;
pub const RPI_SMB2_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
pub const RPI_SMB2_SHARECAP_ACCESS_BASED_DIRECTORY_ENUM: u32 = 256u32;
pub const RPI_SMB2_SHARECAP_ASYMMETRIC_SCALEOUT: u32 = 1024u32;
pub const RPI_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
pub const RPI_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
pub const RPI_SMB2_SHARECAP_DFS: u32 = 8u32;
pub const RPI_SMB2_SHARECAP_ENCRYPTED: u32 = 128u32;
pub const RPI_SMB2_SHARECAP_IDENTITY_REMOTING: u32 = 512u32;
pub const RPI_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
pub const RPI_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
pub const RPI_SMB2_SHARETYPE_DISK: u32 = 0u32;
pub const RPI_SMB2_SHARETYPE_PIPE: u32 = 1u32;
pub const RPI_SMB2_SHARETYPE_PRINT: u32 = 2u32;
pub const RTL_DUPLICATE_UNICODE_STRING_ALLOCATE_NULL_STRING: u32 = 2u32;
pub const RTL_DUPLICATE_UNICODE_STRING_NULL_TERMINATE: u32 = 1u32;
pub const RTL_HEAP_MEMORY_LIMIT_CURRENT_VERSION: u32 = 1u32;
pub const RTL_SYSTEM_VOLUME_INFORMATION_FOLDER: windows_core::PCWSTR = windows_core::w!("System Volume Information");
pub const SECURITY_ANONYMOUS_LOGON_RID: i32 = 7i32;
pub const SEGMENT_HEAP_FLG_USE_PAGE_HEAP: u32 = 1u32;
pub const SEGMENT_HEAP_PARAMETERS_VERSION: u32 = 3u32;
pub const SEGMENT_HEAP_PARAMS_VALID_FLAGS: u32 = 1u32;
pub const SEMAPHORE_INCREMENT: u32 = 1u32;
pub const SET_PURGE_FAILURE_MODE_DISABLED: u32 = 2u32;
pub const SE_BACKUP_PRIVILEGES_CHECKED: u32 = 256u32;
pub const SE_DACL_UNTRUSTED: u32 = 64u32;
pub const SE_SERVER_SECURITY: u32 = 128u32;
pub const SPECIAL_ENCRYPTED_OPEN: u32 = 262144u32;
pub const SRV_OPEN_ECP_CONTEXT_VERSION_2: u32 = 2u32;
pub const SUPPORTED_FS_FEATURES_BYPASS_IO: u32 = 8u32;
pub const SUPPORTED_FS_FEATURES_OFFLOAD_READ: u32 = 1u32;
pub const SUPPORTED_FS_FEATURES_OFFLOAD_WRITE: u32 = 2u32;
pub const SUPPORTED_FS_FEATURES_QUERY_OPEN: u32 = 4u32;
pub const SYMLINK_DIRECTORY: u32 = 2147483648u32;
pub const SYMLINK_FILE: u32 = 1073741824u32;
pub const SYMLINK_FLAG_RELATIVE: u32 = 1u32;
pub const SYMLINK_RESERVED_MASK: u32 = 4026531840u32;
pub const SYSTEM_PAGE_PRIORITY_BITS: u32 = 3u32;
pub const SharedVirtualDiskCDPSnapshotsSupported: SharedVirtualDiskSupportType = SharedVirtualDiskSupportType(7i32);
pub const SharedVirtualDiskHandleStateFileShared: SharedVirtualDiskHandleState = SharedVirtualDiskHandleState(1i32);
pub const SharedVirtualDiskHandleStateHandleShared: SharedVirtualDiskHandleState = SharedVirtualDiskHandleState(3i32);
pub const SharedVirtualDiskHandleStateNone: SharedVirtualDiskHandleState = SharedVirtualDiskHandleState(0i32);
pub const SharedVirtualDiskSnapshotsSupported: SharedVirtualDiskSupportType = SharedVirtualDiskSupportType(3i32);
pub const SharedVirtualDisksSupported: SharedVirtualDiskSupportType = SharedVirtualDiskSupportType(1i32);
pub const SharedVirtualDisksUnsupported: SharedVirtualDiskSupportType = SharedVirtualDiskSupportType(0i32);
pub const SrvInstanceTypeCsv: SRV_INSTANCE_TYPE = SRV_INSTANCE_TYPE(2i32);
pub const SrvInstanceTypePrimary: SRV_INSTANCE_TYPE = SRV_INSTANCE_TYPE(1i32);
pub const SrvInstanceTypeSBL: SRV_INSTANCE_TYPE = SRV_INSTANCE_TYPE(3i32);
pub const SrvInstanceTypeSR: SRV_INSTANCE_TYPE = SRV_INSTANCE_TYPE(4i32);
pub const SrvInstanceTypeUndefined: SRV_INSTANCE_TYPE = SRV_INSTANCE_TYPE(0i32);
pub const SrvInstanceTypeVSMB: SRV_INSTANCE_TYPE = SRV_INSTANCE_TYPE(5i32);
pub const SyncTypeCreateSection: FS_FILTER_SECTION_SYNC_TYPE = FS_FILTER_SECTION_SYNC_TYPE(1i32);
pub const SyncTypeOther: FS_FILTER_SECTION_SYNC_TYPE = FS_FILTER_SECTION_SYNC_TYPE(0i32);
pub const TOKEN_AUDIT_NO_CHILD_PROCESS: u32 = 2097152u32;
pub const TOKEN_AUDIT_REDIRECTION_TRUST: u32 = 8388608u32;
pub const TOKEN_DO_NOT_USE_GLOBAL_ATTRIBS_FOR_QUERY: u32 = 131072u32;
pub const TOKEN_ENFORCE_REDIRECTION_TRUST: u32 = 4194304u32;
pub const TOKEN_HAS_BACKUP_PRIVILEGE: u32 = 2u32;
pub const TOKEN_HAS_IMPERSONATE_PRIVILEGE: u32 = 128u32;
pub const TOKEN_HAS_OWN_CLAIM_ATTRIBUTES: u32 = 32768u32;
pub const TOKEN_HAS_RESTORE_PRIVILEGE: u32 = 4u32;
pub const TOKEN_HAS_TRAVERSE_PRIVILEGE: u32 = 1u32;
pub const TOKEN_IS_FILTERED: u32 = 2048u32;
pub const TOKEN_IS_RESTRICTED: u32 = 16u32;
pub const TOKEN_LEARNING_MODE_LOGGING: u32 = 16777216u32;
pub const TOKEN_LOWBOX: u32 = 16384u32;
pub const TOKEN_NOT_LOW: u32 = 8192u32;
pub const TOKEN_NO_CHILD_PROCESS: u32 = 524288u32;
pub const TOKEN_NO_CHILD_PROCESS_UNLESS_SECURE: u32 = 1048576u32;
pub const TOKEN_PERMISSIVE_LEARNING_MODE: u32 = 50331648u32;
pub const TOKEN_PRIVATE_NAMESPACE: u32 = 65536u32;
pub const TOKEN_SANDBOX_INERT: u32 = 64u32;
pub const TOKEN_SESSION_NOT_REFERENCED: u32 = 32u32;
pub const TOKEN_UIACCESS: u32 = 4096u32;
pub const TOKEN_VIRTUALIZE_ALLOWED: u32 = 512u32;
pub const TOKEN_VIRTUALIZE_ENABLED: u32 = 1024u32;
pub const TOKEN_WRITE_RESTRICTED: u32 = 8u32;
pub const UNINITIALIZE_CACHE_MAPS: u32 = 1u32;
pub const USE_PRIMARY_PASSWORD: u32 = 1u32;
pub const USN_DELETE_FLAG_DELETE: u32 = 1u32;
pub const VACB_MAPPING_GRANULARITY: u32 = 262144u32;
pub const VACB_OFFSET_SHIFT: u32 = 18u32;
pub const VALID_INHERIT_FLAGS: u32 = 31u32;
pub const VOLSNAPCONTROLTYPE: u32 = 83u32;
pub const VmPrefetchInformation: VIRTUAL_MEMORY_INFORMATION_CLASS = VIRTUAL_MEMORY_INFORMATION_CLASS(0i32);
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_LAYER: u32 = 1u32;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_REGISTERED_LAYER: u32 = 4u32;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_REMOTE_LAYER: u32 = 8u32;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_SCRATCH: u32 = 2u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSV_DOWN_LEVEL_FILE_TYPE(pub i32);
impl windows_core::TypeKind for CSV_DOWN_LEVEL_FILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSV_DOWN_LEVEL_FILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSV_DOWN_LEVEL_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FAST_IO_POSSIBLE(pub i32);
impl windows_core::TypeKind for FAST_IO_POSSIBLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FAST_IO_POSSIBLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FAST_IO_POSSIBLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_DISPOSITION_INFORMATION_EX_FLAGS(pub u32);
impl windows_core::TypeKind for FILE_DISPOSITION_INFORMATION_EX_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_DISPOSITION_INFORMATION_EX_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_DISPOSITION_INFORMATION_EX_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for FILE_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_KNOWN_FOLDER_TYPE(pub i32);
impl windows_core::TypeKind for FILE_KNOWN_FOLDER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_KNOWN_FOLDER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_KNOWN_FOLDER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FSRTL_CHANGE_BACKING_TYPE(pub i32);
impl windows_core::TypeKind for FSRTL_CHANGE_BACKING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FSRTL_CHANGE_BACKING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FSRTL_CHANGE_BACKING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FSRTL_COMPARISON_RESULT(pub i32);
impl windows_core::TypeKind for FSRTL_COMPARISON_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FSRTL_COMPARISON_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FSRTL_COMPARISON_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FS_FILTER_SECTION_SYNC_TYPE(pub i32);
impl windows_core::TypeKind for FS_FILTER_SECTION_SYNC_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FS_FILTER_SECTION_SYNC_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FS_FILTER_SECTION_SYNC_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FS_FILTER_STREAM_FO_NOTIFICATION_TYPE(pub i32);
impl windows_core::TypeKind for FS_FILTER_STREAM_FO_NOTIFICATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FS_FILTER_STREAM_FO_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FS_FILTER_STREAM_FO_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FS_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for FS_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FS_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FS_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HEAP_MEMORY_INFO_CLASS(pub i32);
impl windows_core::TypeKind for HEAP_MEMORY_INFO_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HEAP_MEMORY_INFO_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HEAP_MEMORY_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LINK_TRACKING_INFORMATION_TYPE(pub i32);
impl windows_core::TypeKind for LINK_TRACKING_INFORMATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LINK_TRACKING_INFORMATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LINK_TRACKING_INFORMATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MEMORY_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for MEMORY_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MEMORY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MEMORY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMFLUSH_TYPE(pub i32);
impl windows_core::TypeKind for MMFLUSH_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMFLUSH_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMFLUSH_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSV1_0_AVID(pub i32);
impl windows_core::TypeKind for MSV1_0_AVID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSV1_0_AVID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSV1_0_AVID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETWORK_OPEN_INTEGRITY_QUALIFIER(pub i32);
impl windows_core::TypeKind for NETWORK_OPEN_INTEGRITY_QUALIFIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETWORK_OPEN_INTEGRITY_QUALIFIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETWORK_OPEN_INTEGRITY_QUALIFIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETWORK_OPEN_LOCATION_QUALIFIER(pub i32);
impl windows_core::TypeKind for NETWORK_OPEN_LOCATION_QUALIFIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETWORK_OPEN_LOCATION_QUALIFIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETWORK_OPEN_LOCATION_QUALIFIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NTCREATEFILE_CREATE_DISPOSITION(pub u32);
impl windows_core::TypeKind for NTCREATEFILE_CREATE_DISPOSITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NTCREATEFILE_CREATE_DISPOSITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NTCREATEFILE_CREATE_DISPOSITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NTCREATEFILE_CREATE_OPTIONS(pub u32);
impl windows_core::TypeKind for NTCREATEFILE_CREATE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NTCREATEFILE_CREATE_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NTCREATEFILE_CREATE_OPTIONS").field(&self.0).finish()
    }
}
impl NTCREATEFILE_CREATE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NTCREATEFILE_CREATE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NTCREATEFILE_CREATE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NTCREATEFILE_CREATE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NTCREATEFILE_CREATE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NTCREATEFILE_CREATE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPLOCK_NOTIFY_REASON(pub i32);
impl windows_core::TypeKind for OPLOCK_NOTIFY_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPLOCK_NOTIFY_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPLOCK_NOTIFY_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REFS_COMPRESSION_FORMATS(pub i32);
impl windows_core::TypeKind for REFS_COMPRESSION_FORMATS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REFS_COMPRESSION_FORMATS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REFS_COMPRESSION_FORMATS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REFS_DEALLOCATE_RANGES_ALLOCATOR(pub i32);
impl windows_core::TypeKind for REFS_DEALLOCATE_RANGES_ALLOCATOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REFS_DEALLOCATE_RANGES_ALLOCATOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REFS_DEALLOCATE_RANGES_ALLOCATOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS(pub i32);
impl windows_core::TypeKind for REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REFS_STREAM_SNAPSHOT_OPERATION(pub i32);
impl windows_core::TypeKind for REFS_STREAM_SNAPSHOT_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REFS_STREAM_SNAPSHOT_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REFS_STREAM_SNAPSHOT_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTL_MEMORY_TYPE(pub i32);
impl windows_core::TypeKind for RTL_MEMORY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTL_MEMORY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTL_MEMORY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SE_AUDIT_OPERATION(pub i32);
impl windows_core::TypeKind for SE_AUDIT_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SE_AUDIT_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SE_AUDIT_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SRV_INSTANCE_TYPE(pub i32);
impl windows_core::TypeKind for SRV_INSTANCE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SRV_INSTANCE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SRV_INSTANCE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SharedVirtualDiskHandleState(pub i32);
impl windows_core::TypeKind for SharedVirtualDiskHandleState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SharedVirtualDiskHandleState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SharedVirtualDiskHandleState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SharedVirtualDiskSupportType(pub i32);
impl windows_core::TypeKind for SharedVirtualDiskSupportType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SharedVirtualDiskSupportType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SharedVirtualDiskSupportType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VIRTUAL_MEMORY_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for VIRTUAL_MEMORY_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VIRTUAL_MEMORY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VIRTUAL_MEMORY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _LCN_WEAK_REFERENCE_STATE(pub i32);
impl windows_core::TypeKind for _LCN_WEAK_REFERENCE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _LCN_WEAK_REFERENCE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_LCN_WEAK_REFERENCE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _REFS_STREAM_EXTENT_PROPERTIES(pub i32);
impl windows_core::TypeKind for _REFS_STREAM_EXTENT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _REFS_STREAM_EXTENT_PROPERTIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_REFS_STREAM_EXTENT_PROPERTIES").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: u16,
}
impl windows_core::TypeKind for ACE_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ATOMIC_CREATE_ECP_CONTEXT {
    pub Size: u16,
    pub InFlags: u16,
    pub OutFlags: u16,
    pub ReparseBufferLength: u16,
    pub ReparseBuffer: *mut REPARSE_DATA_BUFFER,
    pub FileSize: i64,
    pub ValidDataLength: i64,
    pub FileTimestamps: *mut FILE_TIMESTAMPS,
    pub FileAttributes: u32,
    pub UsnSourceInfo: u32,
    pub Usn: i64,
    pub SuppressFileAttributeInheritanceMask: u32,
    pub InOpFlags: u32,
    pub OutOpFlags: u32,
    pub InGenFlags: u32,
    pub OutGenFlags: u32,
    pub CaseSensitiveFlagsMask: u32,
    pub InCaseSensitiveFlags: u32,
    pub OutCaseSensitiveFlags: u32,
}
impl windows_core::TypeKind for ATOMIC_CREATE_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for ATOMIC_CREATE_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BASE_MCB {
    pub MaximumPairCount: u32,
    pub PairCount: u32,
    pub PoolType: u16,
    pub Flags: u16,
    pub Mapping: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for BASE_MCB {
    type TypeKind = windows_core::CopyType;
}
impl Default for BASE_MCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BOOT_AREA_INFO {
    pub BootSectorCount: u32,
    pub BootSectors: [BOOT_AREA_INFO_0; 2],
}
impl windows_core::TypeKind for BOOT_AREA_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for BOOT_AREA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BOOT_AREA_INFO_0 {
    pub Offset: i64,
}
impl windows_core::TypeKind for BOOT_AREA_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for BOOT_AREA_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CACHE_MANAGER_CALLBACKS {
    pub AcquireForLazyWrite: PACQUIRE_FOR_LAZY_WRITE,
    pub ReleaseFromLazyWrite: PRELEASE_FROM_LAZY_WRITE,
    pub AcquireForReadAhead: PACQUIRE_FOR_READ_AHEAD,
    pub ReleaseFromReadAhead: PRELEASE_FROM_READ_AHEAD,
}
impl windows_core::TypeKind for CACHE_MANAGER_CALLBACKS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CACHE_MANAGER_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CACHE_MANAGER_CALLBACKS_EX {
    pub Version: u16,
    pub Size: u16,
    pub Functions: CACHE_MANAGER_CALLBACK_FUNCTIONS,
}
impl windows_core::TypeKind for CACHE_MANAGER_CALLBACKS_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CACHE_MANAGER_CALLBACKS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CACHE_MANAGER_CALLBACK_FUNCTIONS {
    pub AcquireForLazyWriteEx: PACQUIRE_FOR_LAZY_WRITE_EX,
    pub ReleaseFromLazyWrite: PRELEASE_FROM_LAZY_WRITE,
    pub AcquireForReadAhead: PACQUIRE_FOR_READ_AHEAD,
    pub ReleaseFromReadAhead: PRELEASE_FROM_READ_AHEAD,
}
impl windows_core::TypeKind for CACHE_MANAGER_CALLBACK_FUNCTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CACHE_MANAGER_CALLBACK_FUNCTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct CACHE_UNINITIALIZE_EVENT {
    pub Next: *mut CACHE_UNINITIALIZE_EVENT,
    pub Event: super::super::Foundation::KEVENT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for CACHE_UNINITIALIZE_EVENT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for CACHE_UNINITIALIZE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug)]
pub struct CC_ASYNC_READ_CONTEXT {
    pub CompletionRoutine: PASYNC_READ_COMPLETION_CALLBACK,
    pub Context: *mut core::ffi::c_void,
    pub Mdl: *mut super::super::Foundation::MDL,
    pub RequestorMode: i8,
    pub NestingLevel: u32,
}
#[cfg(feature = "Wdk_Foundation")]
impl windows_core::TypeKind for CC_ASYNC_READ_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for CC_ASYNC_READ_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CC_ERROR_CALLBACK_CONTEXT {
    pub NodeByteSize: i16,
    pub ErrorCode: super::super::super::Win32::Foundation::NTSTATUS,
}
impl windows_core::TypeKind for CC_ERROR_CALLBACK_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CC_ERROR_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CC_FILE_SIZES {
    pub AllocationSize: i64,
    pub FileSize: i64,
    pub ValidDataLength: i64,
}
impl windows_core::TypeKind for CC_FILE_SIZES {
    type TypeKind = windows_core::CopyType;
}
impl Default for CC_FILE_SIZES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPRESSED_DATA_INFO {
    pub CompressionFormatAndEngine: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: u8,
    pub NumberOfChunks: u16,
    pub CompressedChunkSizes: [u32; 1],
}
impl windows_core::TypeKind for COMPRESSED_DATA_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPRESSED_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONTAINER_ROOT_INFO_INPUT {
    pub Flags: u32,
}
impl windows_core::TypeKind for CONTAINER_ROOT_INFO_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONTAINER_ROOT_INFO_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONTAINER_ROOT_INFO_OUTPUT {
    pub ContainerRootIdLength: u16,
    pub ContainerRootId: [u8; 1],
}
impl windows_core::TypeKind for CONTAINER_ROOT_INFO_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONTAINER_ROOT_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONTAINER_VOLUME_STATE {
    pub Flags: u32,
}
impl windows_core::TypeKind for CONTAINER_VOLUME_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONTAINER_VOLUME_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COPY_INFORMATION {
    pub SourceFileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub SourceFileOffset: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for COPY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for COPY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CPTABLEINFO {
    pub CodePage: u16,
    pub MaximumCharacterSize: u16,
    pub DefaultChar: u16,
    pub UniDefaultChar: u16,
    pub TransDefaultChar: u16,
    pub TransUniDefaultChar: u16,
    pub DBCSCodePage: u16,
    pub LeadByte: [u8; 12],
    pub MultiByteTable: *mut u16,
    pub WideCharTable: *mut core::ffi::c_void,
    pub DBCSRanges: *mut u16,
    pub DBCSOffsets: *mut u16,
}
impl windows_core::TypeKind for CPTABLEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CPTABLEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_REDIRECTION_ECP_CONTEXT {
    pub Size: u16,
    pub Flags: u16,
    pub FileId: super::super::super::Win32::Storage::FileSystem::FILE_ID_128,
    pub VolumeGuid: windows_core::GUID,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for CREATE_REDIRECTION_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CREATE_REDIRECTION_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_USN_JOURNAL_DATA {
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
}
impl windows_core::TypeKind for CREATE_USN_JOURNAL_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_USN_JOURNAL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT {
    pub Version: u32,
    pub IsResume: super::super::super::Win32::Foundation::BOOLEAN,
    pub FileType: CSV_DOWN_LEVEL_FILE_TYPE,
    pub SourceNodeId: u32,
    pub DestinationNodeId: u32,
}
impl windows_core::TypeKind for CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION_ECP_CONTEXT {
    pub FileId: i64,
    pub FileRevision: [i64; 3],
}
impl windows_core::TypeKind for CSV_QUERY_FILE_REVISION_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CSV_QUERY_FILE_REVISION_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    pub FileId: super::super::super::Win32::Storage::FileSystem::FILE_ID_128,
    pub FileRevision: [i64; 3],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT {
    pub Size: usize,
    pub PauseTimeoutInSeconds: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DUAL_OPLOCK_KEY_ECP_CONTEXT {
    pub ParentOplockKey: windows_core::GUID,
    pub TargetOplockKey: windows_core::GUID,
    pub ParentOplockKeySet: super::super::super::Win32::Foundation::BOOLEAN,
    pub TargetOplockKeySet: super::super::super::Win32::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for DUAL_OPLOCK_KEY_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DUAL_OPLOCK_KEY_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DUPLICATE_CLUSTER_DATA {
    pub SourceLcn: i64,
    pub TargetFileOffset: i64,
    pub DuplicationLimit: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for DUPLICATE_CLUSTER_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DUPLICATE_CLUSTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ECP_OPEN_PARAMETERS {
    pub Size: u16,
    pub Reserved: u16,
    pub Flags: u32,
}
impl windows_core::TypeKind for ECP_OPEN_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ECP_OPEN_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct EOF_WAIT_BLOCK {
    pub EofWaitLinks: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Event: super::super::Foundation::KEVENT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for EOF_WAIT_BLOCK {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for EOF_WAIT_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXTENT_READ_CACHE_INFO_BUFFER {
    pub AllocatedCache: i64,
    pub PopulatedCache: i64,
    pub InErrorCache: i64,
}
impl windows_core::TypeKind for EXTENT_READ_CACHE_INFO_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXTENT_READ_CACHE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ACCESS_INFORMATION {
    pub AccessFlags: u32,
}
impl windows_core::TypeKind for FILE_ACCESS_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ACCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ALIGNMENT_INFORMATION {
    pub AlignmentRequirement: u32,
}
impl windows_core::TypeKind for FILE_ALIGNMENT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ALIGNMENT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ALLOCATION_INFORMATION {
    pub AllocationSize: i64,
}
impl windows_core::TypeKind for FILE_ALLOCATION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ALL_INFORMATION {
    pub BasicInformation: FILE_BASIC_INFORMATION,
    pub StandardInformation: FILE_STANDARD_INFORMATION,
    pub InternalInformation: FILE_INTERNAL_INFORMATION,
    pub EaInformation: FILE_EA_INFORMATION,
    pub AccessInformation: FILE_ACCESS_INFORMATION,
    pub PositionInformation: FILE_POSITION_INFORMATION,
    pub ModeInformation: FILE_MODE_INFORMATION,
    pub AlignmentInformation: FILE_ALIGNMENT_INFORMATION,
    pub NameInformation: FILE_NAME_INFORMATION,
}
impl windows_core::TypeKind for FILE_ALL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ALL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_BASIC_INFORMATION {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl windows_core::TypeKind for FILE_BASIC_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_BOTH_DIR_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_CASE_SENSITIVE_INFORMATION {
    pub Flags: u32,
}
impl windows_core::TypeKind for FILE_CASE_SENSITIVE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_CASE_SENSITIVE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_COMPLETION_INFORMATION {
    pub Port: super::super::super::Win32::Foundation::HANDLE,
    pub Key: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for FILE_COMPLETION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_COMPLETION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_COMPRESSION_INFORMATION {
    pub CompressedFileSize: i64,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl windows_core::TypeKind for FILE_COMPRESSION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_COMPRESSION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_DIRECTORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_DIRECTORY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_DIRECTORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_DISPOSITION_INFORMATION {
    pub DeleteFile: super::super::super::Win32::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for FILE_DISPOSITION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_DISPOSITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_DISPOSITION_INFORMATION_EX {
    pub Flags: FILE_DISPOSITION_INFORMATION_EX_FLAGS,
}
impl windows_core::TypeKind for FILE_DISPOSITION_INFORMATION_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_DISPOSITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_EA_INFORMATION {
    pub EaSize: u32,
}
impl windows_core::TypeKind for FILE_EA_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_EA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_END_OF_FILE_INFORMATION_EX {
    pub EndOfFile: i64,
    pub PagingFileSizeInMM: i64,
    pub PagingFileMaxSize: i64,
    pub Flags: u32,
}
impl windows_core::TypeKind for FILE_END_OF_FILE_INFORMATION_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_END_OF_FILE_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FS_ATTRIBUTE_INFORMATION {
    pub FileSystemAttributes: u32,
    pub MaximumComponentNameLength: i32,
    pub FileSystemNameLength: u32,
    pub FileSystemName: [u16; 1],
}
impl windows_core::TypeKind for FILE_FS_ATTRIBUTE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FS_ATTRIBUTE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FS_CONTROL_INFORMATION {
    pub FreeSpaceStartFiltering: i64,
    pub FreeSpaceThreshold: i64,
    pub FreeSpaceStopFiltering: i64,
    pub DefaultQuotaThreshold: i64,
    pub DefaultQuotaLimit: i64,
    pub FileSystemControlFlags: u32,
}
impl windows_core::TypeKind for FILE_FS_CONTROL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FS_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FS_DATA_COPY_INFORMATION {
    pub NumberOfCopies: u32,
}
impl windows_core::TypeKind for FILE_FS_DATA_COPY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FS_DATA_COPY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FS_DRIVER_PATH_INFORMATION {
    pub DriverInPath: super::super::super::Win32::Foundation::BOOLEAN,
    pub DriverNameLength: u32,
    pub DriverName: [u16; 1],
}
impl windows_core::TypeKind for FILE_FS_DRIVER_PATH_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FS_DRIVER_PATH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FS_SECTOR_SIZE_INFORMATION {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
impl windows_core::TypeKind for FILE_FS_SECTOR_SIZE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FS_SECTOR_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FS_VOLUME_FLAGS_INFORMATION {
    pub Flags: u32,
}
impl windows_core::TypeKind for FILE_FS_VOLUME_FLAGS_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FS_VOLUME_FLAGS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_FULL_DIR_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FULL_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FULL_EA_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u8,
    pub EaNameLength: u8,
    pub EaValueLength: u16,
    pub EaName: [i8; 1],
}
impl windows_core::TypeKind for FILE_FULL_EA_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FULL_EA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_GET_EA_INFORMATION {
    pub NextEntryOffset: u32,
    pub EaNameLength: u8,
    pub EaName: [i8; 1],
}
impl windows_core::TypeKind for FILE_GET_EA_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_GET_EA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_GET_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub Sid: super::super::super::Win32::Security::SID,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for FILE_GET_QUOTA_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for FILE_GET_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_ID_BOTH_DIR_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::super::super::Win32::Storage::FileSystem::FILE_ID_128,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::super::super::Win32::Storage::FileSystem::FILE_ID_128,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for FILE_ID_EXTD_DIR_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for FILE_ID_EXTD_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_ID_FULL_DIR_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_FULL_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileId: i64,
    pub LockingTransactionId: windows_core::GUID,
    pub TxInfoFlags: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_INFORMATION {
    pub VolumeSerialNumber: u64,
    pub FileId: super::super::super::Win32::Storage::FileSystem::FILE_ID_128,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for FILE_ID_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for FILE_ID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_INFORMATION_DEFINITION {
    pub Class: FILE_INFORMATION_CLASS,
    pub NextEntryOffset: u32,
    pub FileNameLengthOffset: u32,
    pub FileNameOffset: u32,
}
impl windows_core::TypeKind for FILE_INFORMATION_DEFINITION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_INFORMATION_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_INTERNAL_INFORMATION {
    pub IndexNumber: i64,
}
impl windows_core::TypeKind for FILE_INTERNAL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_INTERNAL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_KNOWN_FOLDER_INFORMATION {
    pub Type: FILE_KNOWN_FOLDER_TYPE,
}
impl windows_core::TypeKind for FILE_KNOWN_FOLDER_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_KNOWN_FOLDER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LINKS_FULL_ID_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_FULL_ID_INFORMATION,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for FILE_LINKS_FULL_ID_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for FILE_LINKS_FULL_ID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LINKS_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_INFORMATION,
}
impl windows_core::TypeKind for FILE_LINKS_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_LINKS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: super::super::super::Win32::Storage::FileSystem::FILE_ID_128,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LINK_ENTRY_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_LINK_ENTRY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_LINK_ENTRY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_LINK_INFORMATION {
    pub Anonymous: FILE_LINK_INFORMATION_0,
    pub RootDirectory: super::super::super::Win32::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_LINK_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_LINK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_LINK_INFORMATION_0 {
    pub ReplaceIfExists: super::super::super::Win32::Foundation::BOOLEAN,
    pub Flags: u32,
}
impl windows_core::TypeKind for FILE_LINK_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_LINK_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug)]
pub struct FILE_LOCK {
    pub CompleteLockIrpRoutine: PCOMPLETE_LOCK_IRP_ROUTINE,
    pub UnlockRoutine: PUNLOCK_ROUTINE,
    pub FastIoIsQuestionable: super::super::super::Win32::Foundation::BOOLEAN,
    pub SpareC: [super::super::super::Win32::Foundation::BOOLEAN; 3],
    pub LockInformation: *mut core::ffi::c_void,
    pub LastReturnedLockInfo: FILE_LOCK_INFO,
    pub LastReturnedLock: *mut core::ffi::c_void,
    pub LockRequestsInProgress: i32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FILE_LOCK {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FILE_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LOCK_INFO {
    pub StartingByte: i64,
    pub Length: i64,
    pub ExclusiveLock: super::super::super::Win32::Foundation::BOOLEAN,
    pub Key: u32,
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub ProcessId: *mut core::ffi::c_void,
    pub EndingByte: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FILE_LOCK_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FILE_LOCK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_MAILSLOT_QUERY_INFORMATION {
    pub MaximumMessageSize: u32,
    pub MailslotQuota: u32,
    pub NextMessageSize: u32,
    pub MessagesAvailable: u32,
    pub ReadTimeout: i64,
}
impl windows_core::TypeKind for FILE_MAILSLOT_QUERY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_MAILSLOT_QUERY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_MAILSLOT_SET_INFORMATION {
    pub ReadTimeout: *mut i64,
}
impl windows_core::TypeKind for FILE_MAILSLOT_SET_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_MAILSLOT_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_MODE_INFORMATION {
    pub Mode: u32,
}
impl windows_core::TypeKind for FILE_MODE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_MODE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_MOVE_CLUSTER_INFORMATION {
    pub ClusterCount: u32,
    pub RootDirectory: super::super::super::Win32::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_MOVE_CLUSTER_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_MOVE_CLUSTER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NAMES_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_NAMES_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_NAMES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_NAME_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NETWORK_OPEN_INFORMATION {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
}
impl windows_core::TypeKind for FILE_NETWORK_OPEN_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_NETWORK_OPEN_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_OBJECTID_INFORMATION {
    pub FileReference: i64,
    pub ObjectId: [u8; 16],
    pub Anonymous: FILE_OBJECTID_INFORMATION_0,
}
impl windows_core::TypeKind for FILE_OBJECTID_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_OBJECTID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_OBJECTID_INFORMATION_0 {
    pub Anonymous: FILE_OBJECTID_INFORMATION_0_0,
    pub ExtendedInfo: [u8; 48],
}
impl windows_core::TypeKind for FILE_OBJECTID_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_OBJECTID_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_OBJECTID_INFORMATION_0_0 {
    pub BirthVolumeId: [u8; 16],
    pub BirthObjectId: [u8; 16],
    pub DomainId: [u8; 16],
}
impl windows_core::TypeKind for FILE_OBJECTID_INFORMATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_OBJECTID_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_ASSIGN_EVENT_BUFFER {
    pub EventHandle: super::super::super::Win32::Foundation::HANDLE,
    pub KeyValue: u32,
}
impl windows_core::TypeKind for FILE_PIPE_ASSIGN_EVENT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_ASSIGN_EVENT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER {
    pub ClientSession: *mut core::ffi::c_void,
    pub ClientProcess: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for FILE_PIPE_CLIENT_PROCESS_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    pub ClientSession: *mut core::ffi::c_void,
    pub ClientProcess: *mut core::ffi::c_void,
    pub ClientComputerNameLength: u16,
    pub ClientComputerBuffer: [u16; 16],
}
impl windows_core::TypeKind for FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    pub ClientSession: u64,
    pub ClientProcess: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_CREATE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub Flags: u32,
}
impl windows_core::TypeKind for FILE_PIPE_CREATE_SYMLINK_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_CREATE_SYMLINK_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_DELETE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
}
impl windows_core::TypeKind for FILE_PIPE_DELETE_SYMLINK_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_DELETE_SYMLINK_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_EVENT_BUFFER {
    pub NamedPipeState: u32,
    pub EntryType: u32,
    pub ByteCount: u32,
    pub KeyValue: u32,
    pub NumberRequests: u32,
}
impl windows_core::TypeKind for FILE_PIPE_EVENT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_EVENT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_INFORMATION {
    pub ReadMode: u32,
    pub CompletionMode: u32,
}
impl windows_core::TypeKind for FILE_PIPE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_LOCAL_INFORMATION {
    pub NamedPipeType: u32,
    pub NamedPipeConfiguration: u32,
    pub MaximumInstances: u32,
    pub CurrentInstances: u32,
    pub InboundQuota: u32,
    pub ReadDataAvailable: u32,
    pub OutboundQuota: u32,
    pub WriteQuotaAvailable: u32,
    pub NamedPipeState: u32,
    pub NamedPipeEnd: u32,
}
impl windows_core::TypeKind for FILE_PIPE_LOCAL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_LOCAL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_PEEK_BUFFER {
    pub NamedPipeState: u32,
    pub ReadDataAvailable: u32,
    pub NumberOfMessages: u32,
    pub MessageLength: u32,
    pub Data: [i8; 1],
}
impl windows_core::TypeKind for FILE_PIPE_PEEK_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_PEEK_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_REMOTE_INFORMATION {
    pub CollectDataTime: i64,
    pub MaximumCollectionCount: u32,
}
impl windows_core::TypeKind for FILE_PIPE_REMOTE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_REMOTE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_SILO_ARRIVAL_INPUT {
    pub JobHandle: super::super::super::Win32::Foundation::HANDLE,
}
impl windows_core::TypeKind for FILE_PIPE_SILO_ARRIVAL_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_SILO_ARRIVAL_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_WAIT_FOR_BUFFER {
    pub Timeout: i64,
    pub NameLength: u32,
    pub TimeoutSpecified: super::super::super::Win32::Foundation::BOOLEAN,
    pub Name: [u16; 1],
}
impl windows_core::TypeKind for FILE_PIPE_WAIT_FOR_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_PIPE_WAIT_FOR_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_POSITION_INFORMATION {
    pub CurrentByteOffset: i64,
}
impl windows_core::TypeKind for FILE_POSITION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_POSITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub ChangeTime: i64,
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
    pub Sid: super::super::super::Win32::Security::SID,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for FILE_QUOTA_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for FILE_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFORMATION_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFORMATION_1,
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_0 {
    pub Reserved: [u32; 8],
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFORMATION_1_0,
    pub Reserved: [u32; 16],
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1,
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFORMATION_1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0 {
    pub Capabilities: u32,
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1 {
    pub Capabilities: u32,
    pub ShareFlags: u32,
    pub ShareType: u8,
    pub Reserved0: [u8; 3],
    pub Reserved1: u32,
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_RENAME_INFORMATION {
    pub Anonymous: FILE_RENAME_INFORMATION_0,
    pub RootDirectory: super::super::super::Win32::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_RENAME_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_RENAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_RENAME_INFORMATION_0 {
    pub ReplaceIfExists: super::super::super::Win32::Foundation::BOOLEAN,
    pub Flags: u32,
}
impl windows_core::TypeKind for FILE_RENAME_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_RENAME_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REPARSE_POINT_INFORMATION {
    pub FileReference: i64,
    pub Tag: u32,
}
impl windows_core::TypeKind for FILE_REPARSE_POINT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REPARSE_POINT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STANDARD_INFORMATION {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: super::super::super::Win32::Foundation::BOOLEAN,
    pub Directory: super::super::super::Win32::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for FILE_STANDARD_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STANDARD_LINK_INFORMATION {
    pub NumberOfAccessibleLinks: u32,
    pub TotalNumberOfLinks: u32,
    pub DeletePending: super::super::super::Win32::Foundation::BOOLEAN,
    pub Directory: super::super::super::Win32::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for FILE_STANDARD_LINK_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_STANDARD_LINK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STAT_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub EffectiveAccess: u32,
}
impl windows_core::TypeKind for FILE_STAT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_STAT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STAT_LX_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub EffectiveAccess: u32,
    pub LxFlags: u32,
    pub LxUid: u32,
    pub LxGid: u32,
    pub LxMode: u32,
    pub LxDeviceIdMajor: u32,
    pub LxDeviceIdMinor: u32,
}
impl windows_core::TypeKind for FILE_STAT_LX_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_STAT_LX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ioctl")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STORAGE_RESERVE_ID_INFORMATION {
    pub StorageReserveId: super::super::super::Win32::System::Ioctl::STORAGE_RESERVE_ID,
}
#[cfg(feature = "Win32_System_Ioctl")]
impl windows_core::TypeKind for FILE_STORAGE_RESERVE_ID_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Ioctl")]
impl Default for FILE_STORAGE_RESERVE_ID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STREAM_INFORMATION {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl windows_core::TypeKind for FILE_STREAM_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_TIMESTAMPS {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
}
impl windows_core::TypeKind for FILE_TIMESTAMPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_TIMESTAMPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_TRACKING_INFORMATION {
    pub DestinationFile: super::super::super::Win32::Foundation::HANDLE,
    pub ObjectInformationLength: u32,
    pub ObjectInformation: [i8; 1],
}
impl windows_core::TypeKind for FILE_TRACKING_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_TRACKING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_VOLUME_NAME_INFORMATION {
    pub DeviceNameLength: u32,
    pub DeviceName: [u16; 1],
}
impl windows_core::TypeKind for FILE_VOLUME_NAME_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_VOLUME_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    pub FileOffset: i64,
    pub ByteCount: i64,
    pub RecallOwnerGuid: windows_core::GUID,
    pub RecallMetadataBufferSize: u32,
    pub RecallMetadataBuffer: [u8; 1],
}
impl windows_core::TypeKind for FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {
    pub FileOffset: i64,
    pub ByteCount: i64,
}
impl windows_core::TypeKind for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT {
    pub ExtentCount: u32,
    pub TotalExtentCount: u32,
    pub Extents: [u8; 1],
}
impl windows_core::TypeKind for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT {
    pub NumaNode: u32,
}
impl windows_core::TypeKind for FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_UNMAP_SPACE_INPUT_BUFFER {
    pub BytesToUnmap: i64,
}
impl windows_core::TypeKind for FSCTL_UNMAP_SPACE_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSCTL_UNMAP_SPACE_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_UNMAP_SPACE_OUTPUT {
    pub BytesUnmapped: i64,
}
impl windows_core::TypeKind for FSCTL_UNMAP_SPACE_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSCTL_UNMAP_SPACE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct FSRTL_ADVANCED_FCB_HEADER {
    pub Base: FSRTL_COMMON_FCB_HEADER,
    pub FastMutex: *mut super::super::Foundation::FAST_MUTEX,
    pub FilterContexts: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub PushLock: usize,
    pub FileContextSupportPointer: *mut *mut core::ffi::c_void,
    pub Anonymous: FSRTL_ADVANCED_FCB_HEADER_0,
    pub AePushLock: *mut core::ffi::c_void,
    pub BypassIoOpenCount: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for FSRTL_ADVANCED_FCB_HEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for FSRTL_ADVANCED_FCB_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub union FSRTL_ADVANCED_FCB_HEADER_0 {
    pub Oplock: *mut core::ffi::c_void,
    pub ReservedForRemote: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for FSRTL_ADVANCED_FCB_HEADER_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for FSRTL_ADVANCED_FCB_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSRTL_AUXILIARY_BUFFER {
    pub Buffer: *mut core::ffi::c_void,
    pub Length: u32,
    pub Flags: u32,
    pub Mdl: *mut super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl windows_core::TypeKind for FSRTL_AUXILIARY_BUFFER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FSRTL_AUXILIARY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSRTL_COMMON_FCB_HEADER {
    pub NodeTypeCode: i16,
    pub NodeByteSize: i16,
    pub Flags: u8,
    pub IsFastIoPossible: u8,
    pub Flags2: u8,
    pub _bitfield: u8,
    pub Resource: *mut super::super::Foundation::ERESOURCE,
    pub PagingIoResource: *mut super::super::Foundation::ERESOURCE,
    pub AllocationSize: i64,
    pub FileSize: i64,
    pub ValidDataLength: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for FSRTL_COMMON_FCB_HEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for FSRTL_COMMON_FCB_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSRTL_MUP_PROVIDER_INFO_LEVEL_1 {
    pub ProviderId: u32,
}
impl windows_core::TypeKind for FSRTL_MUP_PROVIDER_INFO_LEVEL_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSRTL_MUP_PROVIDER_INFO_LEVEL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSRTL_MUP_PROVIDER_INFO_LEVEL_2 {
    pub ProviderId: u32,
    pub ProviderName: super::super::super::Win32::Foundation::UNICODE_STRING,
}
impl windows_core::TypeKind for FSRTL_MUP_PROVIDER_INFO_LEVEL_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSRTL_MUP_PROVIDER_INFO_LEVEL_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSRTL_PER_FILEOBJECT_CONTEXT {
    pub Links: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for FSRTL_PER_FILEOBJECT_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for FSRTL_PER_FILEOBJECT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug)]
pub struct FSRTL_PER_FILE_CONTEXT {
    pub Links: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
    pub FreeCallback: super::super::Foundation::PFREE_FUNCTION,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for FSRTL_PER_FILE_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for FSRTL_PER_FILE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug)]
pub struct FSRTL_PER_STREAM_CONTEXT {
    pub Links: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
    pub FreeCallback: super::super::Foundation::PFREE_FUNCTION,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for FSRTL_PER_STREAM_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for FSRTL_PER_STREAM_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION {
    pub Size: u16,
    pub Version: u16,
    pub Anonymous1: FSRTL_UNC_PROVIDER_REGISTRATION_0,
    pub Anonymous2: FSRTL_UNC_PROVIDER_REGISTRATION_1,
}
impl windows_core::TypeKind for FSRTL_UNC_PROVIDER_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSRTL_UNC_PROVIDER_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FSRTL_UNC_PROVIDER_REGISTRATION_0 {
    pub ProviderFlags: u32,
    pub Anonymous: FSRTL_UNC_PROVIDER_REGISTRATION_0_0,
}
impl windows_core::TypeKind for FSRTL_UNC_PROVIDER_REGISTRATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSRTL_UNC_PROVIDER_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FSRTL_UNC_PROVIDER_REGISTRATION_1 {
    pub HardeningCapabilities: u32,
    pub Anonymous: FSRTL_UNC_PROVIDER_REGISTRATION_1_0,
}
impl windows_core::TypeKind for FSRTL_UNC_PROVIDER_REGISTRATION_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSRTL_UNC_PROVIDER_REGISTRATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_BPIO_INFO {
    pub ActiveBypassIoCount: u32,
    pub StorageDriverNameLen: u16,
    pub StorageDriverName: [u16; 32],
}
impl windows_core::TypeKind for FS_BPIO_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FS_BPIO_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ioctl")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_BPIO_INPUT {
    pub Operation: super::super::super::Win32::System::Ioctl::FS_BPIO_OPERATIONS,
    pub InFlags: super::super::super::Win32::System::Ioctl::FS_BPIO_INFLAGS,
    pub Reserved1: u64,
    pub Reserved2: u64,
}
#[cfg(feature = "Win32_System_Ioctl")]
impl windows_core::TypeKind for FS_BPIO_INPUT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Ioctl")]
impl Default for FS_BPIO_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug)]
pub struct FS_FILTER_CALLBACKS {
    pub SizeOfFsFilterCallbacks: u32,
    pub Reserved: u32,
    pub PreAcquireForSectionSynchronization: PFS_FILTER_CALLBACK,
    pub PostAcquireForSectionSynchronization: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForSectionSynchronization: PFS_FILTER_CALLBACK,
    pub PostReleaseForSectionSynchronization: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreAcquireForCcFlush: PFS_FILTER_CALLBACK,
    pub PostAcquireForCcFlush: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForCcFlush: PFS_FILTER_CALLBACK,
    pub PostReleaseForCcFlush: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreAcquireForModifiedPageWriter: PFS_FILTER_CALLBACK,
    pub PostAcquireForModifiedPageWriter: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForModifiedPageWriter: PFS_FILTER_CALLBACK,
    pub PostReleaseForModifiedPageWriter: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreQueryOpen: PFS_FILTER_CALLBACK,
    pub PostQueryOpen: PFS_FILTER_COMPLETION_CALLBACK,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FS_FILTER_CALLBACKS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FS_FILTER_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FS_FILTER_CALLBACK_DATA {
    pub SizeOfFsFilterCallbackData: u32,
    pub Operation: u8,
    pub Reserved: u8,
    pub DeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub Parameters: FS_FILTER_PARAMETERS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FS_FILTER_CALLBACK_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FS_FILTER_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FS_FILTER_PARAMETERS {
    pub AcquireForModifiedPageWriter: FS_FILTER_PARAMETERS_0,
    pub ReleaseForModifiedPageWriter: FS_FILTER_PARAMETERS_4,
    pub AcquireForSectionSynchronization: FS_FILTER_PARAMETERS_1,
    pub QueryOpen: FS_FILTER_PARAMETERS_3,
    pub Others: FS_FILTER_PARAMETERS_2,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FS_FILTER_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FS_FILTER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_0 {
    pub EndingOffset: *mut i64,
    pub ResourceToRelease: *mut *mut super::super::Foundation::ERESOURCE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FS_FILTER_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FS_FILTER_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_1 {
    pub SyncType: FS_FILTER_SECTION_SYNC_TYPE,
    pub PageProtection: u32,
    pub OutputInformation: *mut FS_FILTER_SECTION_SYNC_OUTPUT,
    pub Flags: u32,
    pub AllocationAttributes: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FS_FILTER_PARAMETERS_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FS_FILTER_PARAMETERS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_2 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
    pub Argument5: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FS_FILTER_PARAMETERS_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FS_FILTER_PARAMETERS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_3 {
    pub Irp: *mut super::super::Foundation::IRP,
    pub FileInformation: *mut core::ffi::c_void,
    pub Length: *mut u32,
    pub FileInformationClass: FILE_INFORMATION_CLASS,
    pub CompletionStatus: super::super::super::Win32::Foundation::NTSTATUS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FS_FILTER_PARAMETERS_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FS_FILTER_PARAMETERS_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_4 {
    pub ResourceToRelease: *mut super::super::Foundation::ERESOURCE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FS_FILTER_PARAMETERS_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FS_FILTER_PARAMETERS_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_FILTER_SECTION_SYNC_OUTPUT {
    pub StructureSize: u32,
    pub SizeReturned: u32,
    pub Flags: u32,
    pub DesiredReadAlignment: u32,
}
impl windows_core::TypeKind for FS_FILTER_SECTION_SYNC_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FS_FILTER_SECTION_SYNC_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GENERATE_NAME_CONTEXT {
    pub Checksum: u16,
    pub ChecksumInserted: super::super::super::Win32::Foundation::BOOLEAN,
    pub NameLength: u8,
    pub NameBuffer: [u16; 8],
    pub ExtensionLength: u32,
    pub ExtensionBuffer: [u16; 4],
    pub LastIndexValue: u32,
}
impl windows_core::TypeKind for GENERATE_NAME_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for GENERATE_NAME_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GHOSTED_FILE_EXTENT {
    pub FileOffset: i64,
    pub ByteCount: i64,
    pub RecallOwnerGuid: windows_core::GUID,
    pub NextEntryOffset: u32,
    pub RecallMetadataBufferSize: u32,
    pub RecallMetadataBuffer: [u8; 1],
}
impl windows_core::TypeKind for GHOSTED_FILE_EXTENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for GHOSTED_FILE_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IO_CREATE_STREAM_FILE_OPTIONS {
    pub Size: u16,
    pub Flags: u16,
    pub TargetDeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_CREATE_STREAM_FILE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_CREATE_STREAM_FILE_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IO_DEVICE_HINT_ECP_CONTEXT {
    pub TargetDevice: *mut super::super::Foundation::DEVICE_OBJECT,
    pub RemainingName: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_DEVICE_HINT_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_DEVICE_HINT_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IO_PRIORITY_INFO {
    pub Size: u32,
    pub ThreadPriority: u32,
    pub PagePriority: u32,
    pub IoPriority: super::super::Foundation::IO_PRIORITY_HINT,
}
#[cfg(feature = "Wdk_Foundation")]
impl windows_core::TypeKind for IO_PRIORITY_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for IO_PRIORITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IO_STOP_ON_SYMLINK_FILTER_ECP_v0 {
    pub Out: IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0,
}
impl windows_core::TypeKind for IO_STOP_ON_SYMLINK_FILTER_ECP_v0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IO_STOP_ON_SYMLINK_FILTER_ECP_v0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0 {
    pub ReparseCount: u32,
    pub RemainingPathLength: u32,
}
impl windows_core::TypeKind for IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KAPC_STATE {
    pub ApcListHead: [super::super::super::Win32::System::Kernel::LIST_ENTRY; 2],
    pub Process: *mut isize,
    pub Anonymous1: KAPC_STATE_0,
    pub KernelApcPending: super::super::super::Win32::Foundation::BOOLEAN,
    pub Anonymous2: KAPC_STATE_1,
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KAPC_STATE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KAPC_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union KAPC_STATE_0 {
    pub InProgressFlags: u8,
    pub Anonymous: KAPC_STATE_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KAPC_STATE_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KAPC_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KAPC_STATE_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KAPC_STATE_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KAPC_STATE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union KAPC_STATE_1 {
    pub UserApcPendingAll: super::super::super::Win32::Foundation::BOOLEAN,
    pub Anonymous: KAPC_STATE_1_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KAPC_STATE_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KAPC_STATE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KAPC_STATE_1_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KAPC_STATE_1_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KAPC_STATE_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LARGE_MCB {
    pub GuardedMutex: *mut super::super::Foundation::FAST_MUTEX,
    pub BaseMcb: BASE_MCB,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for LARGE_MCB {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for LARGE_MCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LCN_WEAK_REFERENCE_BUFFER {
    pub Lcn: i64,
    pub LengthInClusters: i64,
    pub ReferenceCount: u32,
    pub State: u16,
}
impl windows_core::TypeKind for LCN_WEAK_REFERENCE_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for LCN_WEAK_REFERENCE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER {
    pub Offset: i64,
    pub Length: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LINK_TRACKING_INFORMATION {
    pub Type: LINK_TRACKING_INFORMATION_TYPE,
    pub VolumeId: [u8; 16],
}
impl windows_core::TypeKind for LINK_TRACKING_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINK_TRACKING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MCB {
    pub DummyFieldThatSizesThisStructureCorrectly: LARGE_MCB,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for MCB {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for MCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl windows_core::TypeKind for MEMORY_RANGE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MFT_ENUM_DATA {
    pub StartFileReferenceNumber: u64,
    pub LowUsn: i64,
    pub HighUsn: i64,
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
impl windows_core::TypeKind for MFT_ENUM_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MFT_ENUM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MM_PREFETCH_FLAGS {
    pub Flags: MM_PREFETCH_FLAGS_0,
    pub AllFlags: u32,
}
impl windows_core::TypeKind for MM_PREFETCH_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MM_PREFETCH_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MM_PREFETCH_FLAGS_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for MM_PREFETCH_FLAGS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MM_PREFETCH_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_ENUMUSERS_REQUEST {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for MSV1_0_ENUMUSERS_REQUEST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for MSV1_0_ENUMUSERS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_ENUMUSERS_RESPONSE {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub NumberOfLoggedOnUsers: u32,
    pub LogonIds: *mut super::super::super::Win32::Foundation::LUID,
    pub EnumHandles: *mut u32,
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for MSV1_0_ENUMUSERS_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for MSV1_0_ENUMUSERS_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_REQUEST {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ParameterControl: u32,
    pub LogonId: super::super::super::Win32::Foundation::LUID,
    pub Password: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ServerName: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for MSV1_0_GETCHALLENRESP_REQUEST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for MSV1_0_GETCHALLENRESP_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ParameterControl: u32,
    pub LogonId: super::super::super::Win32::Foundation::LUID,
    pub Password: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_RESPONSE {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub CaseSensitiveChallengeResponse: super::super::super::Win32::System::Kernel::STRING,
    pub CaseInsensitiveChallengeResponse: super::super::super::Win32::System::Kernel::STRING,
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub UserSessionKey: [u8; 16],
    pub LanmanSessionKey: [u8; 8],
}
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for MSV1_0_GETCHALLENRESP_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_System_Kernel"))]
impl Default for MSV1_0_GETCHALLENRESP_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_GETUSERINFO_REQUEST {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Win32::Foundation::LUID,
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for MSV1_0_GETUSERINFO_REQUEST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for MSV1_0_GETUSERINFO_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_GETUSERINFO_RESPONSE {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub UserSid: super::super::super::Win32::Security::PSID,
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonType: super::super::super::Win32::Security::Authentication::Identity::SECURITY_LOGON_TYPE,
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for MSV1_0_GETUSERINFO_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for MSV1_0_GETUSERINFO_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_LM20_CHALLENGE_REQUEST {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for MSV1_0_LM20_CHALLENGE_REQUEST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for MSV1_0_LM20_CHALLENGE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_LM20_CHALLENGE_RESPONSE {
    pub MessageType: super::super::super::Win32::Security::Authentication::Identity::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ChallengeToClient: [u8; 8],
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for MSV1_0_LM20_CHALLENGE_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for MSV1_0_LM20_CHALLENGE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_APP_INSTANCE_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub AppInstanceID: windows_core::GUID,
}
impl windows_core::TypeKind for NETWORK_APP_INSTANCE_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_APP_INSTANCE_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub VersionHigh: u64,
    pub VersionLow: u64,
}
impl windows_core::TypeKind for NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub Anonymous: NETWORK_OPEN_ECP_CONTEXT_0,
}
impl windows_core::TypeKind for NETWORK_OPEN_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_OPEN_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0 {
    pub r#in: NETWORK_OPEN_ECP_CONTEXT_0_0,
    pub out: NETWORK_OPEN_ECP_CONTEXT_0_1,
}
impl windows_core::TypeKind for NETWORK_OPEN_ECP_CONTEXT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_OPEN_ECP_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0_0 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
    pub Flags: u32,
}
impl windows_core::TypeKind for NETWORK_OPEN_ECP_CONTEXT_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_OPEN_ECP_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0_1 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
    pub Flags: u32,
}
impl windows_core::TypeKind for NETWORK_OPEN_ECP_CONTEXT_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_OPEN_ECP_CONTEXT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0 {
    pub Size: u16,
    pub Reserved: u16,
    pub Anonymous: NETWORK_OPEN_ECP_CONTEXT_V0_0,
}
impl windows_core::TypeKind for NETWORK_OPEN_ECP_CONTEXT_V0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_OPEN_ECP_CONTEXT_V0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0 {
    pub r#in: NETWORK_OPEN_ECP_CONTEXT_V0_0_0,
    pub out: NETWORK_OPEN_ECP_CONTEXT_V0_0_1,
}
impl windows_core::TypeKind for NETWORK_OPEN_ECP_CONTEXT_V0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_OPEN_ECP_CONTEXT_V0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0_0 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
}
impl windows_core::TypeKind for NETWORK_OPEN_ECP_CONTEXT_V0_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_OPEN_ECP_CONTEXT_V0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0_1 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
}
impl windows_core::TypeKind for NETWORK_OPEN_ECP_CONTEXT_V0_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_OPEN_ECP_CONTEXT_V0_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NFS_OPEN_ECP_CONTEXT {
    pub ExportAlias: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ClientSocketAddress: *mut super::super::super::Win32::Networking::WinSock::SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for NFS_OPEN_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for NFS_OPEN_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLSTABLEINFO {
    pub OemTableInfo: CPTABLEINFO,
    pub AnsiTableInfo: CPTABLEINFO,
    pub UpperCaseTable: *mut u16,
    pub LowerCaseTable: *mut u16,
}
impl windows_core::TypeKind for NLSTABLEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for NLSTABLEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPEN_REPARSE_LIST {
    pub OpenReparseList: super::super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for OPEN_REPARSE_LIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for OPEN_REPARSE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPEN_REPARSE_LIST_ENTRY {
    pub OpenReparseListEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub ReparseTag: u32,
    pub Flags: u32,
    pub ReparseGuid: windows_core::GUID,
    pub Size: u16,
    pub RemainingLength: u16,
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for OPEN_REPARSE_LIST_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for OPEN_REPARSE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPLOCK_KEY_CONTEXT {
    pub Version: u16,
    pub Flags: u16,
    pub ParentOplockKey: windows_core::GUID,
    pub TargetOplockKey: windows_core::GUID,
    pub Reserved: u32,
}
impl windows_core::TypeKind for OPLOCK_KEY_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPLOCK_KEY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPLOCK_KEY_ECP_CONTEXT {
    pub OplockKey: windows_core::GUID,
    pub Reserved: u32,
}
impl windows_core::TypeKind for OPLOCK_KEY_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPLOCK_KEY_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPLOCK_NOTIFY_PARAMS {
    pub NotifyReason: OPLOCK_NOTIFY_REASON,
    pub NotifyContext: *mut core::ffi::c_void,
    pub Irp: *mut super::super::Foundation::IRP,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for OPLOCK_NOTIFY_PARAMS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for OPLOCK_NOTIFY_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_EXTENTS_DESCRIPTOR {
    pub NumberOfRuns: u32,
    pub NumberOfValidRuns: u32,
    pub Run: [PHYSICAL_MEMORY_RUN; 1],
}
impl windows_core::TypeKind for PHYSICAL_EXTENTS_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHYSICAL_EXTENTS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_MEMORY_DESCRIPTOR {
    pub NumberOfRuns: u32,
    pub NumberOfPages: u32,
    pub Run: [PHYSICAL_MEMORY_RUN; 1],
}
impl windows_core::TypeKind for PHYSICAL_MEMORY_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHYSICAL_MEMORY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_MEMORY_RUN {
    pub BasePage: u32,
    pub PageCount: u32,
}
impl windows_core::TypeKind for PHYSICAL_MEMORY_RUN {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHYSICAL_MEMORY_RUN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PREFETCH_OPEN_ECP_CONTEXT {
    pub Context: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for PREFETCH_OPEN_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PREFETCH_OPEN_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PREFIX_TABLE {
    pub NodeTypeCode: i16,
    pub NameLength: i16,
    pub NextPrefixTree: *mut PREFIX_TABLE_ENTRY,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for PREFIX_TABLE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for PREFIX_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: i16,
    pub NameLength: i16,
    pub NextPrefixTree: *mut PREFIX_TABLE_ENTRY,
    pub Links: super::super::Foundation::RTL_SPLAY_LINKS,
    pub Prefix: *mut super::super::super::Win32::System::Kernel::STRING,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for PREFIX_TABLE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for PREFIX_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PUBLIC_BCB {
    pub NodeTypeCode: i16,
    pub NodeByteSize: i16,
    pub MappedLength: u32,
    pub MappedFileOffset: i64,
}
impl windows_core::TypeKind for PUBLIC_BCB {
    type TypeKind = windows_core::CopyType;
}
impl Default for PUBLIC_BCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ioctl")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_BAD_RANGES_INPUT {
    pub Flags: u32,
    pub NumRanges: u32,
    pub Ranges: [super::super::super::Win32::System::Ioctl::QUERY_BAD_RANGES_INPUT_RANGE; 1],
}
#[cfg(feature = "Win32_System_Ioctl")]
impl windows_core::TypeKind for QUERY_BAD_RANGES_INPUT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Ioctl")]
impl Default for QUERY_BAD_RANGES_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_DIRECT_ACCESS_EXTENTS {
    pub FileOffset: i64,
    pub Length: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for QUERY_DIRECT_ACCESS_EXTENTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_DIRECT_ACCESS_EXTENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_ON_CREATE_EA_INFORMATION {
    pub EaBufferSize: u32,
    pub EaBuffer: *mut FILE_FULL_EA_INFORMATION,
}
impl windows_core::TypeKind for QUERY_ON_CREATE_EA_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_ON_CREATE_EA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_ON_CREATE_ECP_CONTEXT {
    pub RequestedClasses: u32,
    pub ClassesProcessed: u32,
    pub ClassesWithErrors: u32,
    pub ClassesWithNoData: u32,
    pub StatInformation: QUERY_ON_CREATE_FILE_STAT_INFORMATION,
    pub LxInformation: QUERY_ON_CREATE_FILE_LX_INFORMATION,
    pub EaInformation: QUERY_ON_CREATE_EA_INFORMATION,
}
impl windows_core::TypeKind for QUERY_ON_CREATE_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_ON_CREATE_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_ON_CREATE_FILE_LX_INFORMATION {
    pub EffectiveAccess: u32,
    pub LxFlags: u32,
    pub LxUid: u32,
    pub LxGid: u32,
    pub LxMode: u32,
    pub LxDeviceIdMajor: u32,
    pub LxDeviceIdMinor: u32,
}
impl windows_core::TypeKind for QUERY_ON_CREATE_FILE_LX_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_ON_CREATE_FILE_LX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_ON_CREATE_FILE_STAT_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
}
impl windows_core::TypeKind for QUERY_ON_CREATE_FILE_STAT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_ON_CREATE_FILE_STAT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_PATH_REQUEST {
    pub PathNameLength: u32,
    pub SecurityContext: *mut super::super::Foundation::IO_SECURITY_CONTEXT,
    pub FilePathName: [u16; 1],
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl windows_core::TypeKind for QUERY_PATH_REQUEST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for QUERY_PATH_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_PATH_REQUEST_EX {
    pub pSecurityContext: *mut super::super::Foundation::IO_SECURITY_CONTEXT,
    pub EaLength: u32,
    pub pEaBuffer: *mut core::ffi::c_void,
    pub PathName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub DomainServiceName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub EcpList: *mut super::super::Foundation::ECP_LIST,
    pub Silo: super::super::Foundation::PESILO,
    pub Reserved: usize,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl windows_core::TypeKind for QUERY_PATH_REQUEST_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for QUERY_PATH_REQUEST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_PATH_RESPONSE {
    pub LengthAccepted: u32,
}
impl windows_core::TypeKind for QUERY_PATH_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_PATH_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READ_AHEAD_PARAMETERS {
    pub NodeByteSize: i16,
    pub Granularity: u32,
    pub PipelinedRequestSize: u32,
    pub ReadAheadGrowthPercentage: u32,
}
impl windows_core::TypeKind for READ_AHEAD_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for READ_AHEAD_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct READ_LIST {
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub NumberOfEntries: u32,
    pub IsImage: u32,
    pub List: [super::super::super::Win32::Storage::FileSystem::FILE_SEGMENT_ELEMENT; 1],
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for READ_LIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for READ_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READ_USN_JOURNAL_DATA {
    pub StartUsn: i64,
    pub ReasonMask: u32,
    pub ReturnOnlyOnClose: u32,
    pub Timeout: u64,
    pub BytesToWaitFor: u64,
    pub UsnJournalID: u64,
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
impl windows_core::TypeKind for READ_USN_JOURNAL_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for READ_USN_JOURNAL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_DEALLOCATE_RANGES_INPUT_BUFFER {
    pub RangeCount: u32,
    pub Ranges: [REFS_DEALLOCATE_RANGES_RANGE; 1],
}
impl windows_core::TypeKind for REFS_DEALLOCATE_RANGES_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_DEALLOCATE_RANGES_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX {
    pub RangeCount: u32,
    pub Allocator: REFS_DEALLOCATE_RANGES_ALLOCATOR,
    pub StreamReserveUpdateCount: i64,
    pub OffsetToRanges: u32,
    pub OffsetToLeakCounts: u32,
    pub Reserved: [u64; 2],
}
impl windows_core::TypeKind for REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_DEALLOCATE_RANGES_RANGE {
    pub StartOfRange: u64,
    pub CountOfRange: u64,
}
impl windows_core::TypeKind for REFS_DEALLOCATE_RANGES_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_DEALLOCATE_RANGES_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {
    pub DefaultCompressionFormat: REFS_COMPRESSION_FORMATS,
    pub DefaultCompressionLevel: i16,
    pub DefaultCompressionChunkSizeBytes: u32,
    pub VolumeClusterSizeBytes: u32,
    pub TotalVolumeClusters: u64,
    pub TotalAllocatedClusters: u64,
    pub TotalCompressibleClustersAllocated: u64,
    pub TotalCompressibleClustersInUse: u64,
    pub TotalCompressedClusters: u64,
    pub Reserved: [u64; 6],
}
impl windows_core::TypeKind for REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_DEDUP_INFO_OUTPUT_BUFFER {
    pub Enabled: super::super::super::Win32::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for REFS_QUERY_VOLUME_DEDUP_INFO_OUTPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_QUERY_VOLUME_DEDUP_INFO_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_REMOVE_HARDLINK_BACKPOINTER {
    pub ParentDirectory: u64,
    pub Reserved: u64,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for REFS_REMOVE_HARDLINK_BACKPOINTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_REMOVE_HARDLINK_BACKPOINTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER {
    pub CompressionFormat: REFS_COMPRESSION_FORMATS,
    pub CompressionLevel: i16,
    pub CompressionChunkSizeBytes: u32,
    pub Flags: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS,
    pub Reserved: [u64; 8],
}
impl windows_core::TypeKind for REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_SET_VOLUME_DEDUP_INFO_INPUT_BUFFER {
    pub Enable: super::super::super::Win32::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for REFS_SET_VOLUME_DEDUP_INFO_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_SET_VOLUME_DEDUP_INFO_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_EXTENT {
    pub Vcn: i64,
    pub Lcn: i64,
    pub Length: i64,
    pub Properties: u16,
}
impl windows_core::TypeKind for REFS_STREAM_EXTENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_STREAM_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER {
    pub EntryCount: u32,
    pub BufferSizeRequiredForQuery: u32,
    pub Reserved: [u32; 2],
    pub Entries: [REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY; 1],
}
impl windows_core::TypeKind for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY {
    pub NextEntryOffset: u32,
    pub SnapshotNameLength: u16,
    pub SnapshotCreationTime: u64,
    pub StreamSize: u64,
    pub StreamAllocationSize: u64,
    pub Reserved: [u64; 2],
    pub SnapshotName: [u16; 1],
}
impl windows_core::TypeKind for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER {
    pub Operation: REFS_STREAM_SNAPSHOT_OPERATION,
    pub SnapshotNameLength: u16,
    pub OperationInputBufferLength: u16,
    pub Reserved: [u64; 2],
    pub NameAndInputBuffer: [u16; 1],
}
impl windows_core::TypeKind for REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER {
    pub StartingVcn: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER {
    pub ExtentCount: u32,
    pub Reserved: [u32; 2],
    pub Extents: [REFS_STREAM_EXTENT; 1],
}
impl windows_core::TypeKind for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER {
    pub ResetCounters: super::super::super::Win32::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_VOLUME_DATA_BUFFER {
    pub ByteCount: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BytesPerPhysicalSector: u32,
    pub VolumeSerialNumber: i64,
    pub NumberSectors: i64,
    pub TotalClusters: i64,
    pub FreeClusters: i64,
    pub TotalReserved: i64,
    pub BytesPerSector: u32,
    pub BytesPerCluster: u32,
    pub MaximumSizeOfResidentFile: i64,
    pub FastTierDataFillRatio: u16,
    pub SlowTierDataFillRatio: u16,
    pub DestagesFastTierToSlowTierRate: u32,
    pub Reserved: [i64; 9],
}
impl windows_core::TypeKind for REFS_VOLUME_DATA_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REFS_VOLUME_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REMOTE_LINK_TRACKING_INFORMATION {
    pub TargetFileObject: *mut core::ffi::c_void,
    pub TargetLinkTrackingInformationLength: u32,
    pub TargetLinkTrackingInformationBuffer: [u8; 1],
}
impl windows_core::TypeKind for REMOTE_LINK_TRACKING_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for REMOTE_LINK_TRACKING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub Anonymous: REPARSE_DATA_BUFFER_0,
}
impl windows_core::TypeKind for REPARSE_DATA_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPARSE_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union REPARSE_DATA_BUFFER_0 {
    pub SymbolicLinkReparseBuffer: REPARSE_DATA_BUFFER_0_2,
    pub MountPointReparseBuffer: REPARSE_DATA_BUFFER_0_1,
    pub GenericReparseBuffer: REPARSE_DATA_BUFFER_0_0,
}
impl windows_core::TypeKind for REPARSE_DATA_BUFFER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPARSE_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPARSE_DATA_BUFFER_0_0 {
    pub DataBuffer: [u8; 1],
}
impl windows_core::TypeKind for REPARSE_DATA_BUFFER_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPARSE_DATA_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPARSE_DATA_BUFFER_0_1 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub PathBuffer: [u16; 1],
}
impl windows_core::TypeKind for REPARSE_DATA_BUFFER_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPARSE_DATA_BUFFER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPARSE_DATA_BUFFER_0_2 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub Flags: u32,
    pub PathBuffer: [u16; 1],
}
impl windows_core::TypeKind for REPARSE_DATA_BUFFER_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPARSE_DATA_BUFFER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_EX {
    pub Flags: u32,
    pub ExistingReparseTag: u32,
    pub ExistingReparseGuid: windows_core::GUID,
    pub Reserved: u64,
    pub Anonymous: REPARSE_DATA_BUFFER_EX_0,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for REPARSE_DATA_BUFFER_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for REPARSE_DATA_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub union REPARSE_DATA_BUFFER_EX_0 {
    pub ReparseDataBuffer: REPARSE_DATA_BUFFER,
    pub ReparseGuidDataBuffer: super::super::super::Win32::Storage::FileSystem::REPARSE_GUID_DATA_BUFFER,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl windows_core::TypeKind for REPARSE_DATA_BUFFER_EX_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for REPARSE_DATA_BUFFER_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct REPARSE_INDEX_KEY {
    pub FileReparseTag: u32,
    pub FileId: i64,
}
impl windows_core::TypeKind for REPARSE_INDEX_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPARSE_INDEX_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    pub ExtentCount: u32,
    pub StartingVcn: i64,
    pub Extents: [RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0; 1],
}
impl windows_core::TypeKind for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    pub NextVcn: i64,
    pub Lcn: i64,
    pub ReferenceCount: u32,
}
impl windows_core::TypeKind for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RKF_BYPASS_ECP_CONTEXT {
    pub Reserved: i32,
    pub Version: i32,
}
impl windows_core::TypeKind for RKF_BYPASS_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for RKF_BYPASS_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTL_HEAP_MEMORY_LIMIT_DATA {
    pub CommitLimitBytes: usize,
    pub CommitLimitFailureCode: usize,
    pub MaxAllocationSizeBytes: usize,
    pub AllocationLimitFailureCode: usize,
}
impl windows_core::TypeKind for RTL_HEAP_MEMORY_LIMIT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_HEAP_MEMORY_LIMIT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTL_HEAP_MEMORY_LIMIT_INFO {
    pub Version: u32,
    pub Data: RTL_HEAP_MEMORY_LIMIT_DATA,
}
impl windows_core::TypeKind for RTL_HEAP_MEMORY_LIMIT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_HEAP_MEMORY_LIMIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RTL_HEAP_PARAMETERS {
    pub Length: u32,
    pub SegmentReserve: usize,
    pub SegmentCommit: usize,
    pub DeCommitFreeBlockThreshold: usize,
    pub DeCommitTotalFreeThreshold: usize,
    pub MaximumAllocationSize: usize,
    pub VirtualMemoryThreshold: usize,
    pub InitialCommit: usize,
    pub InitialReserve: usize,
    pub CommitRoutine: PRTL_HEAP_COMMIT_ROUTINE,
    pub Reserved: [usize; 2],
}
impl windows_core::TypeKind for RTL_HEAP_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_HEAP_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTL_NLS_STATE {
    pub DefaultAcpTableInfo: CPTABLEINFO,
    pub DefaultOemTableInfo: CPTABLEINFO,
    pub ActiveCodePageData: *mut u16,
    pub OemCodePageData: *mut u16,
    pub LeadByteInfo: *mut u16,
    pub OemLeadByteInfo: *mut u16,
    pub CaseMappingData: *mut u16,
    pub UnicodeUpcaseTable844: *mut u16,
    pub UnicodeLowercaseTable844: *mut u16,
}
impl windows_core::TypeKind for RTL_NLS_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_NLS_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    pub Flags: u32,
    pub MemoryTypeMask: u32,
    pub NumaNode: u32,
    pub Anonymous: RTL_SEGMENT_HEAP_MEMORY_SOURCE_0,
    pub Reserved: [usize; 2],
}
impl windows_core::TypeKind for RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    pub PartitionHandle: super::super::super::Win32::Foundation::HANDLE,
    pub Callbacks: *mut RTL_SEGMENT_HEAP_VA_CALLBACKS,
}
impl windows_core::TypeKind for RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_SEGMENT_HEAP_PARAMETERS {
    pub Version: u16,
    pub Size: u16,
    pub Flags: u32,
    pub MemorySource: RTL_SEGMENT_HEAP_MEMORY_SOURCE,
    pub Reserved: [usize; 4],
}
impl windows_core::TypeKind for RTL_SEGMENT_HEAP_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_SEGMENT_HEAP_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RTL_SEGMENT_HEAP_VA_CALLBACKS {
    pub CallbackContext: super::super::super::Win32::Foundation::HANDLE,
    pub AllocateVirtualMemory: PALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK,
    pub FreeVirtualMemory: PFREE_VIRTUAL_MEMORY_EX_CALLBACK,
    pub QueryVirtualMemory: PQUERY_VIRTUAL_MEMORY_CALLBACK,
}
impl windows_core::TypeKind for RTL_SEGMENT_HEAP_VA_CALLBACKS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_SEGMENT_HEAP_VA_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_CLIENT_CONTEXT {
    pub SecurityQos: super::super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE,
    pub ClientToken: *mut core::ffi::c_void,
    pub DirectlyAccessClientToken: super::super::super::Win32::Foundation::BOOLEAN,
    pub DirectAccessEffectiveOnly: super::super::super::Win32::Foundation::BOOLEAN,
    pub ServerIsRemote: super::super::super::Win32::Foundation::BOOLEAN,
    pub ClientTokenControl: super::super::super::Win32::Security::TOKEN_CONTROL,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SECURITY_CLIENT_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for SECURITY_CLIENT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEC_APPLICATION_PROTOCOLS {
    pub ProtocolListsSize: u32,
    pub ProtocolLists: [super::super::super::Win32::Security::Authentication::Identity::SEC_APPLICATION_PROTOCOL_LIST; 1],
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl windows_core::TypeKind for SEC_APPLICATION_PROTOCOLS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
impl Default for SEC_APPLICATION_PROTOCOLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEC_DTLS_MTU {
    pub PathMTU: u16,
}
impl windows_core::TypeKind for SEC_DTLS_MTU {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEC_DTLS_MTU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEC_FLAGS {
    pub Flags: u64,
}
impl windows_core::TypeKind for SEC_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEC_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEC_NEGOTIATION_INFO {
    pub Size: u32,
    pub NameLength: u32,
    pub Name: *mut u16,
    pub Reserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SEC_NEGOTIATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEC_NEGOTIATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEC_PRESHAREDKEY {
    pub KeySize: u16,
    pub Key: [u8; 1],
}
impl windows_core::TypeKind for SEC_PRESHAREDKEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEC_PRESHAREDKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEC_SRTP_MASTER_KEY_IDENTIFIER {
    pub MasterKeyIdentifierSize: u8,
    pub MasterKeyIdentifier: [u8; 1],
}
impl windows_core::TypeKind for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SET_CACHED_RUNS_STATE_INPUT_BUFFER {
    pub Enable: super::super::super::Win32::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SET_CACHED_RUNS_STATE_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for SET_CACHED_RUNS_STATE_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SE_AUDIT_INFO {
    pub Size: u32,
    pub AuditType: super::super::super::Win32::Security::AUDIT_EVENT_TYPE,
    pub AuditOperation: SE_AUDIT_OPERATION,
    pub AuditFlags: u32,
    pub SubsystemName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ObjectTypeName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ObjectName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub HandleId: *mut core::ffi::c_void,
    pub TransactionId: *mut windows_core::GUID,
    pub OperationId: *mut super::super::super::Win32::Foundation::LUID,
    pub ObjectCreation: super::super::super::Win32::Foundation::BOOLEAN,
    pub GenerateOnClose: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SE_AUDIT_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for SE_AUDIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SE_EXPORTS {
    pub SeCreateTokenPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeAssignPrimaryTokenPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeLockMemoryPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeIncreaseQuotaPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeUnsolicitedInputPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeTcbPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSecurityPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeTakeOwnershipPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeLoadDriverPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeCreatePagefilePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeIncreaseBasePriorityPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSystemProfilePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSystemtimePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeProfileSingleProcessPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeCreatePermanentPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeBackupPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeRestorePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeShutdownPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeDebugPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeAuditPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSystemEnvironmentPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeChangeNotifyPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeRemoteShutdownPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeNullSid: super::super::super::Win32::Security::PSID,
    pub SeWorldSid: super::super::super::Win32::Security::PSID,
    pub SeLocalSid: super::super::super::Win32::Security::PSID,
    pub SeCreatorOwnerSid: super::super::super::Win32::Security::PSID,
    pub SeCreatorGroupSid: super::super::super::Win32::Security::PSID,
    pub SeNtAuthoritySid: super::super::super::Win32::Security::PSID,
    pub SeDialupSid: super::super::super::Win32::Security::PSID,
    pub SeNetworkSid: super::super::super::Win32::Security::PSID,
    pub SeBatchSid: super::super::super::Win32::Security::PSID,
    pub SeInteractiveSid: super::super::super::Win32::Security::PSID,
    pub SeLocalSystemSid: super::super::super::Win32::Security::PSID,
    pub SeAliasAdminsSid: super::super::super::Win32::Security::PSID,
    pub SeAliasUsersSid: super::super::super::Win32::Security::PSID,
    pub SeAliasGuestsSid: super::super::super::Win32::Security::PSID,
    pub SeAliasPowerUsersSid: super::super::super::Win32::Security::PSID,
    pub SeAliasAccountOpsSid: super::super::super::Win32::Security::PSID,
    pub SeAliasSystemOpsSid: super::super::super::Win32::Security::PSID,
    pub SeAliasPrintOpsSid: super::super::super::Win32::Security::PSID,
    pub SeAliasBackupOpsSid: super::super::super::Win32::Security::PSID,
    pub SeAuthenticatedUsersSid: super::super::super::Win32::Security::PSID,
    pub SeRestrictedSid: super::super::super::Win32::Security::PSID,
    pub SeAnonymousLogonSid: super::super::super::Win32::Security::PSID,
    pub SeUndockPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSyncAgentPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeEnableDelegationPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeLocalServiceSid: super::super::super::Win32::Security::PSID,
    pub SeNetworkServiceSid: super::super::super::Win32::Security::PSID,
    pub SeManageVolumePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeImpersonatePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeCreateGlobalPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeTrustedCredManAccessPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeRelabelPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeIncreaseWorkingSetPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeTimeZonePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeCreateSymbolicLinkPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeIUserSid: super::super::super::Win32::Security::PSID,
    pub SeUntrustedMandatorySid: super::super::super::Win32::Security::PSID,
    pub SeLowMandatorySid: super::super::super::Win32::Security::PSID,
    pub SeMediumMandatorySid: super::super::super::Win32::Security::PSID,
    pub SeHighMandatorySid: super::super::super::Win32::Security::PSID,
    pub SeSystemMandatorySid: super::super::super::Win32::Security::PSID,
    pub SeOwnerRightsSid: super::super::super::Win32::Security::PSID,
    pub SeAllAppPackagesSid: super::super::super::Win32::Security::PSID,
    pub SeUserModeDriversSid: super::super::super::Win32::Security::PSID,
    pub SeProcTrustWinTcbSid: super::super::super::Win32::Security::PSID,
    pub SeTrustedInstallerSid: super::super::super::Win32::Security::PSID,
    pub SeDelegateSessionUserImpersonatePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeAppSiloSid: super::super::super::Win32::Security::PSID,
    pub SeAppSiloVolumeRootMinimalCapabilitySid: super::super::super::Win32::Security::PSID,
    pub SeAppSiloProfilesRootMinimalCapabilitySid: super::super::super::Win32::Security::PSID,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SE_EXPORTS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for SE_EXPORTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SRV_OPEN_ECP_CONTEXT {
    pub ShareName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub SocketAddress: *mut super::super::super::Win32::Networking::WinSock::SOCKADDR_STORAGE,
    pub OplockBlockState: super::super::super::Win32::Foundation::BOOLEAN,
    pub OplockAppState: super::super::super::Win32::Foundation::BOOLEAN,
    pub OplockFinalState: super::super::super::Win32::Foundation::BOOLEAN,
    pub Version: u16,
    pub InstanceType: SRV_INSTANCE_TYPE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for SRV_OPEN_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for SRV_OPEN_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_PROCESS_TRUST_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecBuffer {
    pub cbBuffer: u32,
    pub BufferType: u32,
    pub pvBuffer: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SecBuffer {
    type TypeKind = windows_core::CopyType;
}
impl Default for SecBuffer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecBufferDesc {
    pub ulVersion: u32,
    pub cBuffers: u32,
    pub pBuffers: *mut SecBuffer,
}
impl windows_core::TypeKind for SecBufferDesc {
    type TypeKind = windows_core::CopyType;
}
impl Default for SecBufferDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
impl windows_core::TypeKind for SecHandle {
    type TypeKind = windows_core::CopyType;
}
impl Default for SecHandle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct TUNNEL {
    pub Mutex: super::super::Foundation::FAST_MUTEX,
    pub Cache: *mut super::super::Foundation::RTL_SPLAY_LINKS,
    pub TimerQueue: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub NumEntries: u16,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for TUNNEL {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for TUNNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UNICODE_PREFIX_TABLE {
    pub NodeTypeCode: i16,
    pub NameLength: i16,
    pub NextPrefixTree: *mut UNICODE_PREFIX_TABLE_ENTRY,
    pub LastNextEntry: *mut UNICODE_PREFIX_TABLE_ENTRY,
}
#[cfg(feature = "Wdk_Foundation")]
impl windows_core::TypeKind for UNICODE_PREFIX_TABLE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for UNICODE_PREFIX_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UNICODE_PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: i16,
    pub NameLength: i16,
    pub NextPrefixTree: *mut UNICODE_PREFIX_TABLE_ENTRY,
    pub CaseMatch: *mut UNICODE_PREFIX_TABLE_ENTRY,
    pub Links: super::super::Foundation::RTL_SPLAY_LINKS,
    pub Prefix: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Wdk_Foundation")]
impl windows_core::TypeKind for UNICODE_PREFIX_TABLE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for UNICODE_PREFIX_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USN_JOURNAL_DATA {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
    pub MinSupportedMajorVersion: u16,
    pub MaxSupportedMajorVersion: u16,
}
impl windows_core::TypeKind for USN_JOURNAL_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for USN_JOURNAL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USN_RECORD {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub FileReferenceNumber: u64,
    pub ParentFileReferenceNumber: u64,
    pub Usn: i64,
    pub TimeStamp: i64,
    pub Reason: u32,
    pub SourceInfo: u32,
    pub SecurityId: u32,
    pub FileAttributes: u32,
    pub FileNameLength: u16,
    pub FileNameOffset: u16,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for USN_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for USN_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCN_RANGE_INPUT_BUFFER {
    pub StartingVcn: i64,
    pub ClusterCount: i64,
}
impl windows_core::TypeKind for VCN_RANGE_INPUT_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for VCN_RANGE_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_REFS_INFO_BUFFER {
    pub CacheSizeInBytes: i64,
    pub AllocatedCacheInBytes: i64,
    pub PopulatedCacheInBytes: i64,
    pub InErrorCacheInBytes: i64,
    pub MemoryUsedForCacheMetadata: i64,
    pub CacheLineSize: u32,
    pub CacheTransactionsOutstanding: i32,
    pub CacheLinesFree: i32,
    pub CacheLinesInError: i32,
    pub CacheHitsInBytes: i64,
    pub CacheMissesInBytes: i64,
    pub CachePopulationUpdatesInBytes: i64,
    pub CacheWriteThroughUpdatesInBytes: i64,
    pub CacheInvalidationsInBytes: i64,
    pub CacheOverReadsInBytes: i64,
    pub MetadataWrittenBytes: i64,
    pub CacheHitCounter: i32,
    pub CacheMissCounter: i32,
    pub CacheLineAllocationCounter: i32,
    pub CacheInvalidationsCounter: i32,
    pub CachePopulationUpdatesCounter: i32,
    pub CacheWriteThroughUpdatesCounter: i32,
    pub MaxCacheTransactionsOutstanding: i32,
    pub DataWritesReallocationCount: i32,
    pub DataInPlaceWriteCount: i32,
    pub MetadataAllocationsFastTierCount: i32,
    pub MetadataAllocationsSlowTierCount: i32,
    pub DataAllocationsFastTierCount: i32,
    pub DataAllocationsSlowTierCount: i32,
    pub DestagesSlowTierToFastTier: i32,
    pub DestagesFastTierToSlowTier: i32,
    pub SlowTierDataFillRatio: i32,
    pub FastTierDataFillRatio: i32,
    pub SlowTierMetadataFillRatio: i32,
    pub FastTierMetadataFillRatio: i32,
    pub SlowToFastDestageReadLatency: i32,
    pub SlowToFastDestageReadLatencyBase: i32,
    pub SlowToFastDestageWriteLatency: i32,
    pub SlowToFastDestageWriteLatencyBase: i32,
    pub FastToSlowDestageReadLatency: i32,
    pub FastToSlowDestageReadLatencyBase: i32,
    pub FastToSlowDestageWriteLatency: i32,
    pub FastToSlowDestageWriteLatencyBase: i32,
    pub SlowTierContainerFillRatio: i32,
    pub SlowTierContainerFillRatioBase: i32,
    pub FastTierContainerFillRatio: i32,
    pub FastTierContainerFillRatioBase: i32,
    pub TreeUpdateLatency: i32,
    pub TreeUpdateLatencyBase: i32,
    pub CheckpointLatency: i32,
    pub CheckpointLatencyBase: i32,
    pub TreeUpdateCount: i32,
    pub CheckpointCount: i32,
    pub LogWriteCount: i32,
    pub LogFillRatio: i32,
    pub ReadCacheInvalidationsForOverwrite: i32,
    pub ReadCacheInvalidationsForReuse: i32,
    pub ReadCacheInvalidationsGeneral: i32,
    pub ReadCacheChecksOnMount: i32,
    pub ReadCacheIssuesOnMount: i32,
    pub TrimLatency: i32,
    pub TrimLatencyBase: i32,
    pub DataCompactionCount: i32,
    pub CompactionReadLatency: i32,
    pub CompactionReadLatencyBase: i32,
    pub CompactionWriteLatency: i32,
    pub CompactionWriteLatencyBase: i32,
    pub DataInPlaceWriteClusterCount: i64,
    pub CompactionFailedDueToIneligibleContainer: i32,
    pub CompactionFailedDueToMaxFragmentation: i32,
    pub CompactedContainerFillRatio: i32,
    pub CompactedContainerFillRatioBase: i32,
    pub ContainerMoveRetryCount: i32,
    pub ContainerMoveFailedDueToIneligibleContainer: i32,
    pub CompactionFailureCount: i32,
    pub ContainerMoveFailureCount: i32,
    pub NumberOfDirtyMetadataPages: i64,
    pub NumberOfDirtyTableListEntries: i32,
    pub NumberOfDeleteQueueEntries: i32,
}
impl windows_core::TypeKind for VOLUME_REFS_INFO_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_REFS_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Memory")]
pub type ALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, allocationtype: u32, pageprotection: u32, extendedparameters: *mut super::super::super::Win32::System::Memory::MEM_EXTENDED_PARAMETER, extendedparametercount: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type FREE_VIRTUAL_MEMORY_EX_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, freetype: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PACQUIRE_FOR_LAZY_WRITE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, wait: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PACQUIRE_FOR_LAZY_WRITE_EX = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, inflags: u32, outflags: *mut u32) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PACQUIRE_FOR_READ_AHEAD = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, wait: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PASYNC_READ_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PCC_POST_DEFERRED_WRITE = Option<unsafe extern "system" fn(context1: *const core::ffi::c_void, context2: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
pub type PCHECK_FOR_TRAVERSE_ACCESS = Option<unsafe extern "system" fn(notifycontext: *const core::ffi::c_void, targetcontext: *const core::ffi::c_void, subjectcontext: *const super::super::Foundation::SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCOMPLETE_LOCK_IRP_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, irp: *const super::super::Foundation::IRP) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PDIRTY_PAGE_ROUTINE = Option<unsafe extern "system" fn(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, oldestlsn: *const i64, newestlsn: *const i64, context1: *const core::ffi::c_void, context2: *const core::ffi::c_void)>;
pub type PFILTER_REPORT_CHANGE = Option<unsafe extern "system" fn(notifycontext: *const core::ffi::c_void, filtercontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PFLUSH_TO_LSN = Option<unsafe extern "system" fn(loghandle: *const core::ffi::c_void, lsn: i64)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub type PFN_FSRTLTEARDOWNPERSTREAMCONTEXTS = Option<unsafe extern "system" fn(advancedheader: *const FSRTL_ADVANCED_FCB_HEADER)>;
pub type PFREE_VIRTUAL_MEMORY_EX_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(ecpcontext: *mut core::ffi::c_void, ecptype: *const windows_core::GUID)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub type PFSRTL_STACK_OVERFLOW_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, event: *const super::super::Foundation::KEVENT)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFS_FILTER_CALLBACK = Option<unsafe extern "system" fn(data: *const FS_FILTER_CALLBACK_DATA, completioncontext: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFS_FILTER_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(data: *const FS_FILTER_CALLBACK_DATA, operationstatus: super::super::super::Win32::Foundation::NTSTATUS, completioncontext: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type POPLOCK_FS_PREPOST_IRP = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, irp: *const super::super::Foundation::IRP)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type POPLOCK_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(notifyparams: *const OPLOCK_NOTIFY_PARAMS) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type POPLOCK_WAIT_COMPLETE_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, irp: *const super::super::Foundation::IRP)>;
pub type PQUERY_LOG_USAGE = Option<unsafe extern "system" fn(loghandle: *const core::ffi::c_void, percentagefull: *mut u16)>;
pub type PQUERY_VIRTUAL_MEMORY_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PRELEASE_FROM_LAZY_WRITE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PRELEASE_FROM_READ_AHEAD = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PRTL_ALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn() -> *mut core::ffi::c_void>;
pub type PRTL_FREE_STRING_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PRTL_HEAP_COMMIT_ROUTINE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PRTL_REALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn() -> *mut core::ffi::c_void>;
pub type PSE_LOGON_SESSION_TERMINATED_ROUTINE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PUNLOCK_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, filelockinfo: *const FILE_LOCK_INFO)>;
pub type QUERY_VIRTUAL_MEMORY_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *const core::ffi::c_void, memoryinformationclass: HEAP_MEMORY_INFO_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: *mut usize) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type RTL_ALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize) -> *mut core::ffi::c_void>;
pub type RTL_FREE_STRING_ROUTINE = Option<unsafe extern "system" fn(buffer: *const core::ffi::c_void)>;
pub type RTL_HEAP_COMMIT_ROUTINE = Option<unsafe extern "system" fn(base: *const core::ffi::c_void, commitaddress: *mut *mut core::ffi::c_void, commitsize: *mut usize) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type RTL_REALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize, buffer: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type SE_LOGON_SESSION_TERMINATED_ROUTINE = Option<unsafe extern "system" fn(logonid: *const super::super::super::Win32::Foundation::LUID) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type SE_LOGON_SESSION_TERMINATED_ROUTINE_EX = Option<unsafe extern "system" fn(logonid: *const super::super::super::Win32::Foundation::LUID, pserversilo: super::super::Foundation::PESILO, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type SspiAsyncNotifyCallback = Option<unsafe extern "system" fn(handle: *const super::super::Foundation::SspiAsyncContext, callbackdata: *const core::ffi::c_void)>;

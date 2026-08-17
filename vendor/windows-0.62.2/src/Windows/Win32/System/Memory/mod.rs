#[cfg(feature = "Win32_System_Memory_NonVolatile")]
pub mod NonVolatile;
#[inline]
pub unsafe fn AddSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn AddSecureMemoryCacheCallback(pfncallback : PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::BOOL);
    unsafe { AddSecureMemoryCacheCallback(pfncallback).ok() }
}
#[inline]
pub unsafe fn AllocateUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn AllocateUserPhysicalPages(hprocess : super::super::Foundation:: HANDLE, numberofpages : *mut usize, pagearray : *mut usize) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPages(hprocess, numberofpages as _, pagearray as _).ok() }
}
#[inline]
pub unsafe fn AllocateUserPhysicalPages2(objecthandle: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn AllocateUserPhysicalPages2(objecthandle : super::super::Foundation:: HANDLE, numberofpages : *mut usize, pagearray : *mut usize, extendedparameters : *mut MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPages2(objecthandle, numberofpages as _, pagearray as _, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn AllocateUserPhysicalPagesNuma(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, nndpreferred: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn AllocateUserPhysicalPagesNuma(hprocess : super::super::Foundation:: HANDLE, numberofpages : *mut usize, pagearray : *mut usize, nndpreferred : u32) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPagesNuma(hprocess, numberofpages as _, pagearray as _, nndpreferred).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMapping2<P6>(file: super::super::Foundation::HANDLE, securityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, desiredaccess: u32, pageprotection: PAGE_PROTECTION_FLAGS, allocationattributes: u32, maximumsize: u64, name: P6, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn CreateFileMapping2(file : super::super::Foundation:: HANDLE, securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, desiredaccess : u32, pageprotection : PAGE_PROTECTION_FLAGS, allocationattributes : u32, maximumsize : u64, name : windows_core::PCWSTR, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateFileMapping2(file, securityattributes.unwrap_or(core::mem::zeroed()) as _, desiredaccess, pageprotection, allocationattributes, maximumsize, name.param().abi(), core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingA<P5>(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingA(hfile : super::super::Foundation:: HANDLE, lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flprotect : PAGE_PROTECTION_FLAGS, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateFileMappingA(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingFromApp<P4>(hfile: super::super::Foundation::HANDLE, securityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, pageprotection: PAGE_PROTECTION_FLAGS, maximumsize: u64, name: P4) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingFromApp(hfile : super::super::Foundation:: HANDLE, securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, pageprotection : PAGE_PROTECTION_FLAGS, maximumsize : u64, name : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateFileMappingFromApp(hfile, securityattributes.unwrap_or(core::mem::zeroed()) as _, pageprotection, maximumsize, name.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingNumaA<P5>(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5, nndpreferred: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingNumaA(hfile : super::super::Foundation:: HANDLE, lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flprotect : PAGE_PROTECTION_FLAGS, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCSTR, nndpreferred : u32) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateFileMappingNumaA(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi(), nndpreferred) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingNumaW<P5>(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5, nndpreferred: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingNumaW(hfile : super::super::Foundation:: HANDLE, lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flprotect : PAGE_PROTECTION_FLAGS, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCWSTR, nndpreferred : u32) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateFileMappingNumaW(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi(), nndpreferred) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingW<P5>(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingW(hfile : super::super::Foundation:: HANDLE, lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flprotect : PAGE_PROTECTION_FLAGS, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateFileMappingW(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("kernel32.dll" "system" fn CreateMemoryResourceNotification(notificationtype : MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateMemoryResourceNotification(notificationtype) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn DiscardVirtualMemory(virtualaddress: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn DiscardVirtualMemory(virtualaddress : *mut core::ffi::c_void, size : usize) -> u32);
    unsafe { DiscardVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn FlushViewOfFile(lpbaseaddress: *const core::ffi::c_void, dwnumberofbytestoflush: usize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FlushViewOfFile(lpbaseaddress : *const core::ffi::c_void, dwnumberofbytestoflush : usize) -> windows_core::BOOL);
    unsafe { FlushViewOfFile(lpbaseaddress, dwnumberofbytestoflush).ok() }
}
#[inline]
pub unsafe fn FreeUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *const usize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FreeUserPhysicalPages(hprocess : super::super::Foundation:: HANDLE, numberofpages : *mut usize, pagearray : *const usize) -> windows_core::BOOL);
    unsafe { FreeUserPhysicalPages(hprocess, numberofpages as _, pagearray).ok() }
}
#[inline]
pub unsafe fn GetLargePageMinimum() -> usize {
    windows_core::link!("kernel32.dll" "system" fn GetLargePageMinimum() -> usize);
    unsafe { GetLargePageMinimum() }
}
#[inline]
pub unsafe fn GetMemoryErrorHandlingCapabilities(capabilities: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetMemoryErrorHandlingCapabilities(capabilities : *mut u32) -> windows_core::BOOL);
    unsafe { GetMemoryErrorHandlingCapabilities(capabilities as _).ok() }
}
#[inline]
pub unsafe fn GetProcessHeap() -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeap() -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { GetProcessHeap() };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetProcessHeaps(processheaps: &mut [super::super::Foundation::HANDLE]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeaps(numberofheaps : u32, processheaps : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { GetProcessHeaps(processheaps.len().try_into().unwrap(), core::mem::transmute(processheaps.as_ptr())) }
}
#[inline]
pub unsafe fn GetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize, flags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessWorkingSetSizeEx(hprocess : super::super::Foundation:: HANDLE, lpminimumworkingsetsize : *mut usize, lpmaximumworkingsetsize : *mut usize, flags : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessWorkingSetSizeEx(hprocess, lpminimumworkingsetsize as _, lpmaximumworkingsetsize as _, flags as _) }
}
#[inline]
pub unsafe fn GetSystemFileCacheSize(lpminimumfilecachesize: *mut usize, lpmaximumfilecachesize: *mut usize, lpflags: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetSystemFileCacheSize(lpminimumfilecachesize : *mut usize, lpmaximumfilecachesize : *mut usize, lpflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetSystemFileCacheSize(lpminimumfilecachesize as _, lpmaximumfilecachesize as _, lpflags as _).ok() }
}
#[inline]
pub unsafe fn GetWriteWatch(dwflags: u32, lpbaseaddress: *const core::ffi::c_void, dwregionsize: usize, lpaddresses: Option<*mut *mut core::ffi::c_void>, lpdwcount: Option<*mut usize>, lpdwgranularity: Option<*mut u32>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetWriteWatch(dwflags : u32, lpbaseaddress : *const core::ffi::c_void, dwregionsize : usize, lpaddresses : *mut *mut core::ffi::c_void, lpdwcount : *mut usize, lpdwgranularity : *mut u32) -> u32);
    unsafe { GetWriteWatch(dwflags, lpbaseaddress, dwregionsize, lpaddresses.unwrap_or(core::mem::zeroed()) as _, lpdwcount.unwrap_or(core::mem::zeroed()) as _, lpdwgranularity.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> windows_core::Result<super::super::Foundation::HGLOBAL> {
    windows_core::link!("kernel32.dll" "system" fn GlobalAlloc(uflags : GLOBAL_ALLOC_FLAGS, dwbytes : usize) -> super::super::Foundation:: HGLOBAL);
    let result__ = unsafe { GlobalAlloc(uflags, dwbytes) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GlobalFlags(hmem: super::super::Foundation::HGLOBAL) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GlobalFlags(hmem : super::super::Foundation:: HGLOBAL) -> u32);
    unsafe { GlobalFlags(hmem) }
}
#[inline]
pub unsafe fn GlobalHandle(pmem: *const core::ffi::c_void) -> windows_core::Result<super::super::Foundation::HGLOBAL> {
    windows_core::link!("kernel32.dll" "system" fn GlobalHandle(pmem : *const core::ffi::c_void) -> super::super::Foundation:: HGLOBAL);
    let result__ = unsafe { GlobalHandle(pmem) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GlobalLock(hmem: super::super::Foundation::HGLOBAL) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn GlobalLock(hmem : super::super::Foundation:: HGLOBAL) -> *mut core::ffi::c_void);
    unsafe { GlobalLock(hmem) }
}
#[inline]
pub unsafe fn GlobalReAlloc(hmem: super::super::Foundation::HGLOBAL, dwbytes: usize, uflags: u32) -> windows_core::Result<super::super::Foundation::HGLOBAL> {
    windows_core::link!("kernel32.dll" "system" fn GlobalReAlloc(hmem : super::super::Foundation:: HGLOBAL, dwbytes : usize, uflags : u32) -> super::super::Foundation:: HGLOBAL);
    let result__ = unsafe { GlobalReAlloc(hmem, dwbytes, uflags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GlobalSize(hmem: super::super::Foundation::HGLOBAL) -> usize {
    windows_core::link!("kernel32.dll" "system" fn GlobalSize(hmem : super::super::Foundation:: HGLOBAL) -> usize);
    unsafe { GlobalSize(hmem) }
}
#[inline]
pub unsafe fn GlobalUnlock(hmem: super::super::Foundation::HGLOBAL) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GlobalUnlock(hmem : super::super::Foundation:: HGLOBAL) -> windows_core::BOOL);
    unsafe { GlobalUnlock(hmem).ok() }
}
#[inline]
pub unsafe fn HeapAlloc(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn HeapAlloc(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut core::ffi::c_void);
    unsafe { HeapAlloc(hheap, dwflags, dwbytes) }
}
#[inline]
pub unsafe fn HeapCompact(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS) -> usize {
    windows_core::link!("kernel32.dll" "system" fn HeapCompact(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS) -> usize);
    unsafe { HeapCompact(hheap, dwflags) }
}
#[inline]
pub unsafe fn HeapCreate(floptions: HEAP_FLAGS, dwinitialsize: usize, dwmaximumsize: usize) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("kernel32.dll" "system" fn HeapCreate(floptions : HEAP_FLAGS, dwinitialsize : usize, dwmaximumsize : usize) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { HeapCreate(floptions, dwinitialsize, dwmaximumsize) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn HeapDestroy(hheap: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn HeapDestroy(hheap : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { HeapDestroy(hheap).ok() }
}
#[inline]
pub unsafe fn HeapFree(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn HeapFree(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { HeapFree(hheap as _, dwflags, lpmem.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HeapLock(hheap: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn HeapLock(hheap : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { HeapLock(hheap).ok() }
}
#[inline]
pub unsafe fn HeapQueryInformation(heaphandle: Option<super::super::Foundation::HANDLE>, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: Option<*mut core::ffi::c_void>, heapinformationlength: usize, returnlength: Option<*mut usize>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn HeapQueryInformation(heaphandle : super::super::Foundation:: HANDLE, heapinformationclass : HEAP_INFORMATION_CLASS, heapinformation : *mut core::ffi::c_void, heapinformationlength : usize, returnlength : *mut usize) -> windows_core::BOOL);
    unsafe { HeapQueryInformation(heaphandle.unwrap_or(core::mem::zeroed()) as _, heapinformationclass, heapinformation.unwrap_or(core::mem::zeroed()) as _, heapinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HeapReAlloc(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: Option<*const core::ffi::c_void>, dwbytes: usize) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn HeapReAlloc(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void, dwbytes : usize) -> *mut core::ffi::c_void);
    unsafe { HeapReAlloc(hheap as _, dwflags, lpmem.unwrap_or(core::mem::zeroed()) as _, dwbytes) }
}
#[inline]
pub unsafe fn HeapSetInformation(heaphandle: Option<super::super::Foundation::HANDLE>, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: Option<*const core::ffi::c_void>, heapinformationlength: usize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn HeapSetInformation(heaphandle : super::super::Foundation:: HANDLE, heapinformationclass : HEAP_INFORMATION_CLASS, heapinformation : *const core::ffi::c_void, heapinformationlength : usize) -> windows_core::BOOL);
    unsafe { HeapSetInformation(heaphandle.unwrap_or(core::mem::zeroed()) as _, heapinformationclass, heapinformation.unwrap_or(core::mem::zeroed()) as _, heapinformationlength).ok() }
}
#[inline]
pub unsafe fn HeapSize(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: *const core::ffi::c_void) -> usize {
    windows_core::link!("kernel32.dll" "system" fn HeapSize(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> usize);
    unsafe { HeapSize(hheap, dwflags, lpmem) }
}
#[inline]
pub unsafe fn HeapSummary(hheap: super::super::Foundation::HANDLE, dwflags: u32, lpsummary: *mut HEAP_SUMMARY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapSummary(hheap : super::super::Foundation:: HANDLE, dwflags : u32, lpsummary : *mut HEAP_SUMMARY) -> windows_core::BOOL);
    unsafe { HeapSummary(hheap, dwflags, lpsummary as _) }
}
#[inline]
pub unsafe fn HeapUnlock(hheap: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn HeapUnlock(hheap : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { HeapUnlock(hheap).ok() }
}
#[inline]
pub unsafe fn HeapValidate(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapValidate(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { HeapValidate(hheap, dwflags, lpmem.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HeapWalk(hheap: super::super::Foundation::HANDLE, lpentry: *mut PROCESS_HEAP_ENTRY) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn HeapWalk(hheap : super::super::Foundation:: HANDLE, lpentry : *mut PROCESS_HEAP_ENTRY) -> windows_core::BOOL);
    unsafe { HeapWalk(hheap, lpentry as _).ok() }
}
#[inline]
pub unsafe fn IsBadCodePtr(lpfn: super::super::Foundation::FARPROC) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn IsBadCodePtr(lpfn : super::super::Foundation:: FARPROC) -> windows_core::BOOL);
    unsafe { IsBadCodePtr(lpfn).ok() }
}
#[inline]
pub unsafe fn IsBadReadPtr(lp: Option<*const core::ffi::c_void>, ucb: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadReadPtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_core::BOOL);
    unsafe { IsBadReadPtr(lp.unwrap_or(core::mem::zeroed()) as _, ucb) }
}
#[inline]
pub unsafe fn IsBadStringPtrA<P0>(lpsz: P0, ucchmax: usize) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn IsBadStringPtrA(lpsz : windows_core::PCSTR, ucchmax : usize) -> windows_core::BOOL);
    unsafe { IsBadStringPtrA(lpsz.param().abi(), ucchmax) }
}
#[inline]
pub unsafe fn IsBadStringPtrW<P0>(lpsz: P0, ucchmax: usize) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn IsBadStringPtrW(lpsz : windows_core::PCWSTR, ucchmax : usize) -> windows_core::BOOL);
    unsafe { IsBadStringPtrW(lpsz.param().abi(), ucchmax) }
}
#[inline]
pub unsafe fn IsBadWritePtr(lp: Option<*const core::ffi::c_void>, ucb: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadWritePtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_core::BOOL);
    unsafe { IsBadWritePtr(lp.unwrap_or(core::mem::zeroed()) as _, ucb) }
}
#[inline]
pub unsafe fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> windows_core::Result<super::super::Foundation::HLOCAL> {
    windows_core::link!("kernel32.dll" "system" fn LocalAlloc(uflags : LOCAL_ALLOC_FLAGS, ubytes : usize) -> super::super::Foundation:: HLOCAL);
    let result__ = unsafe { LocalAlloc(uflags, ubytes) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn LocalFlags(hmem: super::super::Foundation::HLOCAL) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn LocalFlags(hmem : super::super::Foundation:: HLOCAL) -> u32);
    unsafe { LocalFlags(hmem) }
}
#[inline]
pub unsafe fn LocalHandle(pmem: *const core::ffi::c_void) -> windows_core::Result<super::super::Foundation::HLOCAL> {
    windows_core::link!("kernel32.dll" "system" fn LocalHandle(pmem : *const core::ffi::c_void) -> super::super::Foundation:: HLOCAL);
    let result__ = unsafe { LocalHandle(pmem) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn LocalLock(hmem: super::super::Foundation::HLOCAL) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn LocalLock(hmem : super::super::Foundation:: HLOCAL) -> *mut core::ffi::c_void);
    unsafe { LocalLock(hmem) }
}
#[inline]
pub unsafe fn LocalReAlloc(hmem: Option<super::super::Foundation::HLOCAL>, ubytes: usize, uflags: u32) -> windows_core::Result<super::super::Foundation::HLOCAL> {
    windows_core::link!("kernel32.dll" "system" fn LocalReAlloc(hmem : super::super::Foundation:: HLOCAL, ubytes : usize, uflags : u32) -> super::super::Foundation:: HLOCAL);
    let result__ = unsafe { LocalReAlloc(hmem.unwrap_or(core::mem::zeroed()) as _, ubytes, uflags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn LocalSize(hmem: super::super::Foundation::HLOCAL) -> usize {
    windows_core::link!("kernel32.dll" "system" fn LocalSize(hmem : super::super::Foundation:: HLOCAL) -> usize);
    unsafe { LocalSize(hmem) }
}
#[inline]
pub unsafe fn LocalUnlock(hmem: super::super::Foundation::HLOCAL) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn LocalUnlock(hmem : super::super::Foundation:: HLOCAL) -> windows_core::BOOL);
    unsafe { LocalUnlock(hmem).ok() }
}
#[inline]
pub unsafe fn MapUserPhysicalPages(virtualaddress: *const core::ffi::c_void, pagearray: Option<&[usize]>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn MapUserPhysicalPages(virtualaddress : *const core::ffi::c_void, numberofpages : usize, pagearray : *const usize) -> windows_core::BOOL);
    unsafe { MapUserPhysicalPages(virtualaddress, pagearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pagearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
}
#[inline]
pub unsafe fn MapUserPhysicalPagesScatter(virtualaddresses: *const *const core::ffi::c_void, numberofpages: usize, pagearray: Option<*const usize>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn MapUserPhysicalPagesScatter(virtualaddresses : *const *const core::ffi::c_void, numberofpages : usize, pagearray : *const usize) -> windows_core::BOOL);
    unsafe { MapUserPhysicalPagesScatter(virtualaddresses, numberofpages, pagearray.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn MapViewOfFile(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFile(hfilemappingobject : super::super::Foundation:: HANDLE, dwdesiredaccess : FILE_MAP, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize) -> MEMORY_MAPPED_VIEW_ADDRESS);
    unsafe { MapViewOfFile(hfilemappingobject, dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap) }
}
#[inline]
pub unsafe fn MapViewOfFile3(filemapping: super::super::Foundation::HANDLE, process: Option<super::super::Foundation::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3(filemapping : super::super::Foundation:: HANDLE, process : super::super::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, offset : u64, viewsize : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, pageprotection : u32, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> MEMORY_MAPPED_VIEW_ADDRESS);
    unsafe { MapViewOfFile3(filemapping, process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, offset, viewsize, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn MapViewOfFile3FromApp(filemapping: super::super::Foundation::HANDLE, process: Option<super::super::Foundation::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3FromApp(filemapping : super::super::Foundation:: HANDLE, process : super::super::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, offset : u64, viewsize : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, pageprotection : u32, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> MEMORY_MAPPED_VIEW_ADDRESS);
    unsafe { MapViewOfFile3FromApp(filemapping, process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, offset, viewsize, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn MapViewOfFileEx(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: Option<*const core::ffi::c_void>) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFileEx(hfilemappingobject : super::super::Foundation:: HANDLE, dwdesiredaccess : FILE_MAP, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize, lpbaseaddress : *const core::ffi::c_void) -> MEMORY_MAPPED_VIEW_ADDRESS);
    unsafe { MapViewOfFileEx(hfilemappingobject, dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap, lpbaseaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MapViewOfFileExNuma(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: Option<*const core::ffi::c_void>, nndpreferred: u32) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFileExNuma(hfilemappingobject : super::super::Foundation:: HANDLE, dwdesiredaccess : FILE_MAP, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize, lpbaseaddress : *const core::ffi::c_void, nndpreferred : u32) -> MEMORY_MAPPED_VIEW_ADDRESS);
    unsafe { MapViewOfFileExNuma(hfilemappingobject, dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap, lpbaseaddress.unwrap_or(core::mem::zeroed()) as _, nndpreferred) }
}
#[inline]
pub unsafe fn MapViewOfFileFromApp(hfilemappingobject: super::super::Foundation::HANDLE, desiredaccess: FILE_MAP, fileoffset: u64, numberofbytestomap: usize) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFileFromApp(hfilemappingobject : super::super::Foundation:: HANDLE, desiredaccess : FILE_MAP, fileoffset : u64, numberofbytestomap : usize) -> MEMORY_MAPPED_VIEW_ADDRESS);
    unsafe { MapViewOfFileFromApp(hfilemappingobject, desiredaccess, fileoffset, numberofbytestomap) }
}
#[inline]
pub unsafe fn MapViewOfFileNuma2(filemappinghandle: super::super::Foundation::HANDLE, processhandle: super::super::Foundation::HANDLE, offset: u64, baseaddress: Option<*const core::ffi::c_void>, viewsize: usize, allocationtype: u32, pageprotection: u32, preferrednode: u32) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn MapViewOfFileNuma2(filemappinghandle : super::super::Foundation:: HANDLE, processhandle : super::super::Foundation:: HANDLE, offset : u64, baseaddress : *const core::ffi::c_void, viewsize : usize, allocationtype : u32, pageprotection : u32, preferrednode : u32) -> MEMORY_MAPPED_VIEW_ADDRESS);
    unsafe { MapViewOfFileNuma2(filemappinghandle, processhandle, offset, baseaddress.unwrap_or(core::mem::zeroed()) as _, viewsize, allocationtype, pageprotection, preferrednode) }
}
#[inline]
pub unsafe fn OfferVirtualMemory(virtualaddress: &mut [u8], priority: OFFER_PRIORITY) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn OfferVirtualMemory(virtualaddress : *mut core::ffi::c_void, size : usize, priority : OFFER_PRIORITY) -> u32);
    unsafe { OfferVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap(), priority) }
}
#[inline]
pub unsafe fn OpenDedicatedMemoryPartition(partition: super::super::Foundation::HANDLE, dedicatedmemorytypeid: u64, desiredaccess: u32, inherithandle: bool) -> super::super::Foundation::HANDLE {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn OpenDedicatedMemoryPartition(partition : super::super::Foundation:: HANDLE, dedicatedmemorytypeid : u64, desiredaccess : u32, inherithandle : windows_core::BOOL) -> super::super::Foundation:: HANDLE);
    unsafe { OpenDedicatedMemoryPartition(partition, dedicatedmemorytypeid, desiredaccess, inherithandle.into()) }
}
#[inline]
pub unsafe fn OpenFileMappingA<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenFileMappingA(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { OpenFileMappingA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn OpenFileMappingFromApp<P2>(desiredaccess: u32, inherithandle: bool, name: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn OpenFileMappingFromApp(desiredaccess : u32, inherithandle : windows_core::BOOL, name : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { OpenFileMappingFromApp(desiredaccess, inherithandle.into(), name.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn OpenFileMappingW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenFileMappingW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { OpenFileMappingW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn PrefetchVirtualMemory(hprocess: super::super::Foundation::HANDLE, virtualaddresses: &[WIN32_MEMORY_RANGE_ENTRY], flags: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn PrefetchVirtualMemory(hprocess : super::super::Foundation:: HANDLE, numberofentries : usize, virtualaddresses : *const WIN32_MEMORY_RANGE_ENTRY, flags : u32) -> windows_core::BOOL);
    unsafe { PrefetchVirtualMemory(hprocess, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), flags).ok() }
}
#[inline]
pub unsafe fn QueryMemoryResourceNotification(resourcenotificationhandle: super::super::Foundation::HANDLE, resourcestate: *mut windows_core::BOOL) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn QueryMemoryResourceNotification(resourcenotificationhandle : super::super::Foundation:: HANDLE, resourcestate : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { QueryMemoryResourceNotification(resourcenotificationhandle, resourcestate as _).ok() }
}
#[inline]
pub unsafe fn QueryPartitionInformation(partition: super::super::Foundation::HANDLE, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: *mut core::ffi::c_void, partitioninformationlength: u32) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn QueryPartitionInformation(partition : super::super::Foundation:: HANDLE, partitioninformationclass : WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation : *mut core::ffi::c_void, partitioninformationlength : u32) -> windows_core::BOOL);
    unsafe { QueryPartitionInformation(partition, partitioninformationclass, partitioninformation as _, partitioninformationlength) }
}
#[inline]
pub unsafe fn QueryVirtualMemoryInformation(process: super::super::Foundation::HANDLE, virtualaddress: *const core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationsize: usize, returnsize: Option<*mut usize>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-memory-l1-1-4.dll" "system" fn QueryVirtualMemoryInformation(process : super::super::Foundation:: HANDLE, virtualaddress : *const core::ffi::c_void, memoryinformationclass : WIN32_MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationsize : usize, returnsize : *mut usize) -> windows_core::BOOL);
    unsafe { QueryVirtualMemoryInformation(process, virtualaddress, memoryinformationclass, memoryinformation as _, memoryinformationsize, returnsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ReclaimVirtualMemory(virtualaddress: &[u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn ReclaimVirtualMemory(virtualaddress : *const core::ffi::c_void, size : usize) -> u32);
    unsafe { ReclaimVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn RegisterBadMemoryNotification(callback: PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn RegisterBadMemoryNotification(callback : PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut core::ffi::c_void);
    unsafe { RegisterBadMemoryNotification(callback) }
}
#[inline]
pub unsafe fn RemoveSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RemoveSecureMemoryCacheCallback(pfncallback : PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::BOOL);
    unsafe { RemoveSecureMemoryCacheCallback(pfncallback) }
}
#[inline]
pub unsafe fn ResetWriteWatch(lpbaseaddress: *const core::ffi::c_void, dwregionsize: usize) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn ResetWriteWatch(lpbaseaddress : *const core::ffi::c_void, dwregionsize : usize) -> u32);
    unsafe { ResetWriteWatch(lpbaseaddress, dwregionsize) }
}
#[inline]
pub unsafe fn RtlCompareMemory(source1: *const core::ffi::c_void, source2: *const core::ffi::c_void, length: usize) -> usize {
    windows_core::link!("kernel32.dll" "system" fn RtlCompareMemory(source1 : *const core::ffi::c_void, source2 : *const core::ffi::c_void, length : usize) -> usize);
    unsafe { RtlCompareMemory(source1, source2, length) }
}
#[inline]
pub unsafe fn RtlCrc32(buffer: *const core::ffi::c_void, size: usize, initialcrc: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlCrc32(buffer : *const core::ffi::c_void, size : usize, initialcrc : u32) -> u32);
    unsafe { RtlCrc32(buffer, size, initialcrc) }
}
#[inline]
pub unsafe fn RtlCrc64(buffer: *const core::ffi::c_void, size: usize, initialcrc: u64) -> u64 {
    windows_core::link!("ntdll.dll" "system" fn RtlCrc64(buffer : *const core::ffi::c_void, size : usize, initialcrc : u64) -> u64);
    unsafe { RtlCrc64(buffer, size, initialcrc) }
}
#[inline]
pub unsafe fn RtlIsZeroMemory(buffer: *const core::ffi::c_void, length: usize) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlIsZeroMemory(buffer : *const core::ffi::c_void, length : usize) -> bool);
    unsafe { RtlIsZeroMemory(buffer, length) }
}
#[inline]
pub unsafe fn SetProcessValidCallTargets(hprocess: super::super::Foundation::HANDLE, virtualaddress: *const core::ffi::c_void, regionsize: usize, offsetinformation: &mut [CFG_CALL_TARGET_INFO]) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn SetProcessValidCallTargets(hprocess : super::super::Foundation:: HANDLE, virtualaddress : *const core::ffi::c_void, regionsize : usize, numberofoffsets : u32, offsetinformation : *mut CFG_CALL_TARGET_INFO) -> windows_core::BOOL);
    unsafe { SetProcessValidCallTargets(hprocess, virtualaddress, regionsize, offsetinformation.len().try_into().unwrap(), core::mem::transmute(offsetinformation.as_ptr())).ok() }
}
#[inline]
pub unsafe fn SetProcessValidCallTargetsForMappedView(process: super::super::Foundation::HANDLE, virtualaddress: *const core::ffi::c_void, regionsize: usize, offsetinformation: &mut [CFG_CALL_TARGET_INFO], section: super::super::Foundation::HANDLE, expectedfileoffset: u64) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn SetProcessValidCallTargetsForMappedView(process : super::super::Foundation:: HANDLE, virtualaddress : *const core::ffi::c_void, regionsize : usize, numberofoffsets : u32, offsetinformation : *mut CFG_CALL_TARGET_INFO, section : super::super::Foundation:: HANDLE, expectedfileoffset : u64) -> windows_core::BOOL);
    unsafe { SetProcessValidCallTargetsForMappedView(process, virtualaddress, regionsize, offsetinformation.len().try_into().unwrap(), core::mem::transmute(offsetinformation.as_ptr()), section, expectedfileoffset) }
}
#[inline]
pub unsafe fn SetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize, flags: SETPROCESSWORKINGSETSIZEEX_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetProcessWorkingSetSizeEx(hprocess : super::super::Foundation:: HANDLE, dwminimumworkingsetsize : usize, dwmaximumworkingsetsize : usize, flags : SETPROCESSWORKINGSETSIZEEX_FLAGS) -> windows_core::BOOL);
    unsafe { SetProcessWorkingSetSizeEx(hprocess, dwminimumworkingsetsize, dwmaximumworkingsetsize, flags).ok() }
}
#[inline]
pub unsafe fn SetSystemFileCacheSize(minimumfilecachesize: usize, maximumfilecachesize: usize, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetSystemFileCacheSize(minimumfilecachesize : usize, maximumfilecachesize : usize, flags : u32) -> windows_core::BOOL);
    unsafe { SetSystemFileCacheSize(minimumfilecachesize, maximumfilecachesize, flags).ok() }
}
#[inline]
pub unsafe fn UnmapViewOfFile(lpbaseaddress: MEMORY_MAPPED_VIEW_ADDRESS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn UnmapViewOfFile(lpbaseaddress : MEMORY_MAPPED_VIEW_ADDRESS) -> windows_core::BOOL);
    unsafe { UnmapViewOfFile(core::mem::transmute(lpbaseaddress)).ok() }
}
#[inline]
pub unsafe fn UnmapViewOfFile2(process: super::super::Foundation::HANDLE, baseaddress: MEMORY_MAPPED_VIEW_ADDRESS, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn UnmapViewOfFile2(process : super::super::Foundation:: HANDLE, baseaddress : MEMORY_MAPPED_VIEW_ADDRESS, unmapflags : UNMAP_VIEW_OF_FILE_FLAGS) -> windows_core::BOOL);
    unsafe { UnmapViewOfFile2(process, core::mem::transmute(baseaddress), unmapflags).ok() }
}
#[inline]
pub unsafe fn UnmapViewOfFileEx(baseaddress: MEMORY_MAPPED_VIEW_ADDRESS, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn UnmapViewOfFileEx(baseaddress : MEMORY_MAPPED_VIEW_ADDRESS, unmapflags : UNMAP_VIEW_OF_FILE_FLAGS) -> windows_core::BOOL);
    unsafe { UnmapViewOfFileEx(core::mem::transmute(baseaddress), unmapflags).ok() }
}
#[inline]
pub unsafe fn UnregisterBadMemoryNotification(registrationhandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn UnregisterBadMemoryNotification(registrationhandle : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { UnregisterBadMemoryNotification(registrationhandle).ok() }
}
#[inline]
pub unsafe fn VirtualAlloc(lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn VirtualAlloc(lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : VIRTUAL_ALLOCATION_TYPE, flprotect : PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void);
    unsafe { VirtualAlloc(lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect) }
}
#[inline]
pub unsafe fn VirtualAlloc2(process: Option<super::super::Foundation::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2(process : super::super::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, size : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, pageprotection : u32, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAlloc2(process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn VirtualAlloc2FromApp(process: Option<super::super::Foundation::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2FromApp(process : super::super::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, size : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, pageprotection : u32, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAlloc2FromApp(process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn VirtualAllocEx(hprocess: super::super::Foundation::HANDLE, lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn VirtualAllocEx(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : VIRTUAL_ALLOCATION_TYPE, flprotect : PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void);
    unsafe { VirtualAllocEx(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect) }
}
#[inline]
pub unsafe fn VirtualAllocExNuma(hprocess: super::super::Foundation::HANDLE, lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: u32, nndpreferred: u32) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn VirtualAllocExNuma(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : VIRTUAL_ALLOCATION_TYPE, flprotect : u32, nndpreferred : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAllocExNuma(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect, nndpreferred) }
}
#[inline]
pub unsafe fn VirtualAllocFromApp(baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, protection: u32) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualAllocFromApp(baseaddress : *const core::ffi::c_void, size : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, protection : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAllocFromApp(baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, protection) }
}
#[inline]
pub unsafe fn VirtualFree(lpaddress: *mut core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn VirtualFree(lpaddress : *mut core::ffi::c_void, dwsize : usize, dwfreetype : VIRTUAL_FREE_TYPE) -> windows_core::BOOL);
    unsafe { VirtualFree(lpaddress as _, dwsize, dwfreetype).ok() }
}
#[inline]
pub unsafe fn VirtualFreeEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *mut core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn VirtualFreeEx(hprocess : super::super::Foundation:: HANDLE, lpaddress : *mut core::ffi::c_void, dwsize : usize, dwfreetype : VIRTUAL_FREE_TYPE) -> windows_core::BOOL);
    unsafe { VirtualFreeEx(hprocess, lpaddress as _, dwsize, dwfreetype).ok() }
}
#[inline]
pub unsafe fn VirtualLock(lpaddress: *const core::ffi::c_void, dwsize: usize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn VirtualLock(lpaddress : *const core::ffi::c_void, dwsize : usize) -> windows_core::BOOL);
    unsafe { VirtualLock(lpaddress, dwsize).ok() }
}
#[inline]
pub unsafe fn VirtualProtect(lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn VirtualProtect(lpaddress : *const core::ffi::c_void, dwsize : usize, flnewprotect : PAGE_PROTECTION_FLAGS, lpfloldprotect : *mut PAGE_PROTECTION_FLAGS) -> windows_core::BOOL);
    unsafe { VirtualProtect(lpaddress, dwsize, flnewprotect, lpfloldprotect as _).ok() }
}
#[inline]
pub unsafe fn VirtualProtectEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn VirtualProtectEx(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flnewprotect : PAGE_PROTECTION_FLAGS, lpfloldprotect : *mut PAGE_PROTECTION_FLAGS) -> windows_core::BOOL);
    unsafe { VirtualProtectEx(hprocess, lpaddress, dwsize, flnewprotect, lpfloldprotect as _).ok() }
}
#[inline]
pub unsafe fn VirtualProtectFromApp(address: *const core::ffi::c_void, size: usize, newprotection: u32, oldprotection: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualProtectFromApp(address : *const core::ffi::c_void, size : usize, newprotection : u32, oldprotection : *mut u32) -> windows_core::BOOL);
    unsafe { VirtualProtectFromApp(address, size, newprotection, oldprotection as _).ok() }
}
#[inline]
pub unsafe fn VirtualQuery(lpaddress: Option<*const core::ffi::c_void>, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    windows_core::link!("kernel32.dll" "system" fn VirtualQuery(lpaddress : *const core::ffi::c_void, lpbuffer : *mut MEMORY_BASIC_INFORMATION, dwlength : usize) -> usize);
    unsafe { VirtualQuery(lpaddress.unwrap_or(core::mem::zeroed()) as _, lpbuffer as _, dwlength) }
}
#[inline]
pub unsafe fn VirtualQueryEx(hprocess: super::super::Foundation::HANDLE, lpaddress: Option<*const core::ffi::c_void>, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    windows_core::link!("kernel32.dll" "system" fn VirtualQueryEx(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, lpbuffer : *mut MEMORY_BASIC_INFORMATION, dwlength : usize) -> usize);
    unsafe { VirtualQueryEx(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, lpbuffer as _, dwlength) }
}
#[inline]
pub unsafe fn VirtualUnlock(lpaddress: *const core::ffi::c_void, dwsize: usize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn VirtualUnlock(lpaddress : *const core::ffi::c_void, dwsize : usize) -> windows_core::BOOL);
    unsafe { VirtualUnlock(lpaddress, dwsize).ok() }
}
#[inline]
pub unsafe fn VirtualUnlockEx(process: Option<super::super::Foundation::HANDLE>, address: *const core::ffi::c_void, size: usize) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn VirtualUnlockEx(process : super::super::Foundation:: HANDLE, address : *const core::ffi::c_void, size : usize) -> windows_core::BOOL);
    unsafe { VirtualUnlockEx(process.unwrap_or(core::mem::zeroed()) as _, address, size) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AtlThunkData_t(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CFG_CALL_TARGET_INFO {
    pub Offset: usize,
    pub Flags: usize,
}
pub const FILE_CACHE_MAX_HARD_DISABLE: u32 = 2u32;
pub const FILE_CACHE_MAX_HARD_ENABLE: u32 = 1u32;
pub const FILE_CACHE_MIN_HARD_DISABLE: u32 = 8u32;
pub const FILE_CACHE_MIN_HARD_ENABLE: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_MAP(pub u32);
impl FILE_MAP {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_MAP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_MAP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_MAP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_MAP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_MAP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FILE_MAP_ALL_ACCESS: FILE_MAP = FILE_MAP(983071u32);
pub const FILE_MAP_COPY: FILE_MAP = FILE_MAP(1u32);
pub const FILE_MAP_EXECUTE: FILE_MAP = FILE_MAP(32u32);
pub const FILE_MAP_LARGE_PAGES: FILE_MAP = FILE_MAP(536870912u32);
pub const FILE_MAP_READ: FILE_MAP = FILE_MAP(4u32);
pub const FILE_MAP_RESERVE: FILE_MAP = FILE_MAP(2147483648u32);
pub const FILE_MAP_TARGETS_INVALID: FILE_MAP = FILE_MAP(1073741824u32);
pub const FILE_MAP_WRITE: FILE_MAP = FILE_MAP(2u32);
pub const GHND: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(66u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBAL_ALLOC_FLAGS(pub u32);
impl GLOBAL_ALLOC_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GLOBAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GLOBAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GMEM_FIXED: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(0u32);
pub const GMEM_MOVEABLE: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(2u32);
pub const GMEM_ZEROINIT: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
pub const GPTR: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
pub const HEAP_CREATE_ALIGN_16: HEAP_FLAGS = HEAP_FLAGS(65536u32);
pub const HEAP_CREATE_ENABLE_EXECUTE: HEAP_FLAGS = HEAP_FLAGS(262144u32);
pub const HEAP_CREATE_ENABLE_TRACING: HEAP_FLAGS = HEAP_FLAGS(131072u32);
pub const HEAP_CREATE_HARDENED: HEAP_FLAGS = HEAP_FLAGS(512u32);
pub const HEAP_CREATE_SEGMENT_HEAP: HEAP_FLAGS = HEAP_FLAGS(256u32);
pub const HEAP_DISABLE_COALESCE_ON_FREE: HEAP_FLAGS = HEAP_FLAGS(128u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEAP_FLAGS(pub u32);
impl HEAP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const HEAP_FREE_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(64u32);
pub const HEAP_GENERATE_EXCEPTIONS: HEAP_FLAGS = HEAP_FLAGS(4u32);
pub const HEAP_GROWABLE: HEAP_FLAGS = HEAP_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEAP_INFORMATION_CLASS(pub i32);
pub const HEAP_MAXIMUM_TAG: HEAP_FLAGS = HEAP_FLAGS(4095u32);
pub const HEAP_NONE: HEAP_FLAGS = HEAP_FLAGS(0u32);
pub const HEAP_NO_SERIALIZE: HEAP_FLAGS = HEAP_FLAGS(1u32);
pub const HEAP_PSEUDO_TAG_FLAG: HEAP_FLAGS = HEAP_FLAGS(32768u32);
pub const HEAP_REALLOC_IN_PLACE_ONLY: HEAP_FLAGS = HEAP_FLAGS(16u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbReserved: usize,
    pub cbMaxReserve: usize,
}
pub const HEAP_TAG_SHIFT: HEAP_FLAGS = HEAP_FLAGS(18u32);
pub const HEAP_TAIL_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(32u32);
pub const HEAP_ZERO_MEMORY: HEAP_FLAGS = HEAP_FLAGS(8u32);
pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(0i32);
pub const HeapEnableTerminationOnCorruption: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(1i32);
pub const HeapOptimizeResources: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(3i32);
pub const HeapTag: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(7i32);
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = MEMORY_RESOURCE_NOTIFICATION_TYPE(1i32);
pub const LHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(66u32);
pub const LMEM_FIXED: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
pub const LMEM_MOVEABLE: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
pub const LMEM_ZEROINIT: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCAL_ALLOC_FLAGS(pub u32);
impl LOCAL_ALLOC_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LOCAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LOCAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = MEMORY_RESOURCE_NOTIFICATION_TYPE(0i32);
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(target_arch = "x86")]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION32 {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: u32,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION64 {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
    pub __alignment2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_MAPPED_VIEW_ADDRESS {
    pub Value: *mut core::ffi::c_void,
}
impl Default for MEMORY_MAPPED_VIEW_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    pub Type: MEM_DEDICATED_ATTRIBUTE_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub SizeOfInformation: u32,
    pub Flags: u32,
    pub AttributesOffset: u32,
    pub AttributeCount: u32,
    pub Reserved: u32,
    pub TypeId: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEMORY_RESOURCE_NOTIFICATION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEM_ADDRESS_REQUIREMENTS {
    pub LowestStartingAddress: *mut core::ffi::c_void,
    pub HighestEndingAddress: *mut core::ffi::c_void,
    pub Alignment: usize,
}
impl Default for MEM_ADDRESS_REQUIREMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(4096u32);
pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(16384u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEM_DEDICATED_ATTRIBUTE_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEM_EXTENDED_PARAMETER {
    pub Anonymous1: MEM_EXTENDED_PARAMETER_0,
    pub Anonymous2: MEM_EXTENDED_PARAMETER_1,
}
impl Default for MEM_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEM_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MEM_EXTENDED_PARAMETER_1 {
    pub ULong64: u64,
    pub Pointer: *mut core::ffi::c_void,
    pub Size: usize,
    pub Handle: super::super::Foundation::HANDLE,
    pub ULong: u32,
}
impl Default for MEM_EXTENDED_PARAMETER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEM_EXTENDED_PARAMETER_TYPE(pub i32);
pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(65536u32);
pub const MEM_IMAGE: PAGE_TYPE = PAGE_TYPE(16777216u32);
pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(536870912u32);
pub const MEM_MAPPED: PAGE_TYPE = PAGE_TYPE(262144u32);
pub const MEM_PRESERVE_PLACEHOLDER: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(2u32);
pub const MEM_PRIVATE: PAGE_TYPE = PAGE_TYPE(131072u32);
pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(32768u32);
pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16384u32);
pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(8192u32);
pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(262144u32);
pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(524288u32);
pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16777216u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEM_SECTION_EXTENDED_PARAMETER_TYPE(pub i32);
pub const MEM_UNMAP_NONE: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(0u32);
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(1u32);
pub const MemDedicatedAttributeMax: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(4i32);
pub const MemDedicatedAttributeReadBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(0i32);
pub const MemDedicatedAttributeReadLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(1i32);
pub const MemDedicatedAttributeWriteBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(2i32);
pub const MemDedicatedAttributeWriteLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(3i32);
pub const MemExtendedParameterAddressRequirements: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(1i32);
pub const MemExtendedParameterAttributeFlags: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(5i32);
pub const MemExtendedParameterImageMachine: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(6i32);
pub const MemExtendedParameterInvalidType: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(0i32);
pub const MemExtendedParameterMax: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(7i32);
pub const MemExtendedParameterNumaNode: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(2i32);
pub const MemExtendedParameterPartitionHandle: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(3i32);
pub const MemExtendedParameterUserPhysicalHandle: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(4i32);
pub const MemSectionExtendedParameterInvalidType: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(0i32);
pub const MemSectionExtendedParameterMax: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(4i32);
pub const MemSectionExtendedParameterNumaNode: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(2i32);
pub const MemSectionExtendedParameterSigningLevel: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(3i32);
pub const MemSectionExtendedParameterUserPhysicalFlags: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(1i32);
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = WIN32_MEMORY_PARTITION_INFORMATION_CLASS(1i32);
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = WIN32_MEMORY_PARTITION_INFORMATION_CLASS(0i32);
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = WIN32_MEMORY_INFORMATION_CLASS(0i32);
pub const NONZEROLHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
pub const NONZEROLPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFER_PRIORITY(pub i32);
pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435457u32);
pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435458u32);
pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(536870912u32);
pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16u32);
pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32u32);
pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(64u32);
pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(128u32);
pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(131072u32);
pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16384u32);
pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32768u32);
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(65536u32);
pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2048u32);
pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4096u32);
pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8192u32);
pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(256u32);
pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1u32);
pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(512u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAGE_PROTECTION_FLAGS(pub u32);
impl PAGE_PROTECTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PAGE_PROTECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PAGE_PROTECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2u32);
pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4u32);
pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAGE_TYPE(pub u32);
impl PAGE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PAGE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PAGE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PAGE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PAGE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PAGE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1024u32);
pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8u32);
pub type PBAD_MEMORY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: *mut core::ffi::c_void,
    pub cbData: u32,
    pub cbOverhead: u8,
    pub iRegionIndex: u8,
    pub wFlags: u16,
    pub Anonymous: PROCESS_HEAP_ENTRY_0,
}
impl Default for PROCESS_HEAP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_HEAP_ENTRY_0 {
    pub Block: PROCESS_HEAP_ENTRY_0_0,
    pub Region: PROCESS_HEAP_ENTRY_0_1,
}
impl Default for PROCESS_HEAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_HEAP_ENTRY_0_0 {
    pub hMem: super::super::Foundation::HANDLE,
    pub dwReserved: [u32; 3],
}
impl Default for PROCESS_HEAP_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_HEAP_ENTRY_0_1 {
    pub dwCommittedSize: u32,
    pub dwUnCommittedSize: u32,
    pub lpFirstBlock: *mut core::ffi::c_void,
    pub lpLastBlock: *mut core::ffi::c_void,
}
impl Default for PROCESS_HEAP_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PSECURE_MEMORY_CACHE_CALLBACK = Option<unsafe extern "system" fn(addr: *const core::ffi::c_void, range: usize) -> bool>;
pub const QUOTA_LIMITS_HARDWS_MAX_DISABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = SETPROCESSWORKINGSETSIZEEX_FLAGS(8u32);
pub const QUOTA_LIMITS_HARDWS_MAX_ENABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = SETPROCESSWORKINGSETSIZEEX_FLAGS(4u32);
pub const QUOTA_LIMITS_HARDWS_MIN_DISABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = SETPROCESSWORKINGSETSIZEEX_FLAGS(2u32);
pub const QUOTA_LIMITS_HARDWS_MIN_ENABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = SETPROCESSWORKINGSETSIZEEX_FLAGS(1u32);
pub const SECTION_ALL_ACCESS: SECTION_FLAGS = SECTION_FLAGS(983071u32);
pub const SECTION_EXTEND_SIZE: SECTION_FLAGS = SECTION_FLAGS(16u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECTION_FLAGS(pub u32);
impl SECTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SECTION_MAP_EXECUTE: SECTION_FLAGS = SECTION_FLAGS(8u32);
pub const SECTION_MAP_EXECUTE_EXPLICIT: SECTION_FLAGS = SECTION_FLAGS(32u32);
pub const SECTION_MAP_READ: SECTION_FLAGS = SECTION_FLAGS(4u32);
pub const SECTION_MAP_WRITE: SECTION_FLAGS = SECTION_FLAGS(2u32);
pub const SECTION_QUERY: SECTION_FLAGS = SECTION_FLAGS(1u32);
pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(524288u32);
pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(134217728u32);
pub const SEC_FILE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8388608u32);
pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16777216u32);
pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(285212672u32);
pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(33554432u32);
pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(67108864u32);
pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETPROCESSWORKINGSETSIZEEX_FLAGS(pub u32);
impl SETPROCESSWORKINGSETSIZEEX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNMAP_VIEW_OF_FILE_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_ALLOCATION_TYPE(pub u32);
impl VIRTUAL_ALLOCATION_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_FREE_TYPE(pub u32);
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = OFFER_PRIORITY(3i32);
pub const VmOfferPriorityLow: OFFER_PRIORITY = OFFER_PRIORITY(2i32);
pub const VmOfferPriorityNormal: OFFER_PRIORITY = OFFER_PRIORITY(4i32);
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = OFFER_PRIORITY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_MEMORY_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION {
    pub Flags: u32,
    pub NumaNode: u32,
    pub Channel: u32,
    pub NumberOfNumaNodes: u32,
    pub ResidentAvailablePages: u64,
    pub CommittedPages: u64,
    pub CommitLimit: u64,
    pub PeakCommitment: u64,
    pub TotalNumberOfPages: u64,
    pub AvailablePages: u64,
    pub ZeroPages: u64,
    pub FreePages: u64,
    pub StandbyPages: u64,
    pub Reserved: [u64; 16],
    pub MaximumCommitLimit: u64,
    pub Reserved2: u64,
    pub PartitionId: u32,
}
impl Default for WIN32_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl Default for WIN32_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIN32_MEMORY_REGION_INFORMATION {
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0,
    pub RegionSize: usize,
    pub CommitSize: usize,
}
impl Default for WIN32_MEMORY_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WIN32_MEMORY_REGION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0_0,
}
impl Default for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}

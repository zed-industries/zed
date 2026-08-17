#[inline]
pub unsafe fn BindIoCompletionCallback<P0>(filehandle: P0, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn BindIoCompletionCallback(filehandle : super::super::Foundation:: HANDLE, function : LPOVERLAPPED_COMPLETION_ROUTINE, flags : u32) -> super::super::Foundation:: BOOL);
    BindIoCompletionCallback(filehandle.param().abi(), function, flags).ok()
}
#[inline]
pub unsafe fn CancelIo<P0>(hfile: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CancelIo(hfile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CancelIo(hfile.param().abi()).ok()
}
#[inline]
pub unsafe fn CancelIoEx<P0>(hfile: P0, lpoverlapped: Option<*const OVERLAPPED>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CancelIoEx(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *const OVERLAPPED) -> super::super::Foundation:: BOOL);
    CancelIoEx(hfile.param().abi(), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CancelSynchronousIo<P0>(hthread: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CancelSynchronousIo(hthread : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CancelSynchronousIo(hthread.param().abi()).ok()
}
#[inline]
pub unsafe fn CreateIoCompletionPort<P0, P1>(filehandle: P0, existingcompletionport: P1, completionkey: usize, numberofconcurrentthreads: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateIoCompletionPort(filehandle : super::super::Foundation:: HANDLE, existingcompletionport : super::super::Foundation:: HANDLE, completionkey : usize, numberofconcurrentthreads : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateIoCompletionPort(filehandle.param().abi(), existingcompletionport.param().abi(), completionkey, numberofconcurrentthreads);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn DeviceIoControl<P0>(hdevice: P0, dwiocontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpoverlapped: Option<*mut OVERLAPPED>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeviceIoControl(hdevice : super::super::Foundation:: HANDLE, dwiocontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpoverlapped : *mut OVERLAPPED) -> super::super::Foundation:: BOOL);
    DeviceIoControl(hdevice.param().abi(), dwiocontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetOverlappedResult<P0, P1>(hfile: P0, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetOverlappedResult(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *const OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetOverlappedResult(hfile.param().abi(), lpoverlapped, lpnumberofbytestransferred, bwait.param().abi()).ok()
}
#[inline]
pub unsafe fn GetOverlappedResultEx<P0, P1>(hfile: P0, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetOverlappedResultEx(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *const OVERLAPPED, lpnumberofbytestransferred : *mut u32, dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetOverlappedResultEx(hfile.param().abi(), lpoverlapped, lpnumberofbytestransferred, dwmilliseconds, balertable.param().abi()).ok()
}
#[inline]
pub unsafe fn GetQueuedCompletionStatus<P0>(completionport: P0, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::super::Foundation:: HANDLE, lpnumberofbytestransferred : *mut u32, lpcompletionkey : *mut usize, lpoverlapped : *mut *mut OVERLAPPED, dwmilliseconds : u32) -> super::super::Foundation:: BOOL);
    GetQueuedCompletionStatus(completionport.param().abi(), lpnumberofbytestransferred, lpcompletionkey, lpoverlapped, dwmilliseconds).ok()
}
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx<P0, P1>(completionport: P0, lpcompletionportentries: &mut [OVERLAPPED_ENTRY], ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetQueuedCompletionStatusEx(completionport : super::super::Foundation:: HANDLE, lpcompletionportentries : *mut OVERLAPPED_ENTRY, ulcount : u32, ulnumentriesremoved : *mut u32, dwmilliseconds : u32, falertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetQueuedCompletionStatusEx(completionport.param().abi(), core::mem::transmute(lpcompletionportentries.as_ptr()), lpcompletionportentries.len().try_into().unwrap(), ulnumentriesremoved, dwmilliseconds, falertable.param().abi()).ok()
}
#[inline]
pub unsafe fn PostQueuedCompletionStatus<P0>(completionport: P0, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: Option<*const OVERLAPPED>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PostQueuedCompletionStatus(completionport : super::super::Foundation:: HANDLE, dwnumberofbytestransferred : u32, dwcompletionkey : usize, lpoverlapped : *const OVERLAPPED) -> super::super::Foundation:: BOOL);
    PostQueuedCompletionStatus(completionport.param().abi(), dwnumberofbytestransferred, dwcompletionkey, core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null()))).ok()
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: usize,
}
impl windows_core::TypeKind for IO_STATUS_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_STATUS_BLOCK_0 {
    pub Status: super::super::Foundation::NTSTATUS,
    pub Pointer: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for IO_STATUS_BLOCK_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IO_STATUS_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for OVERLAPPED {
    type TypeKind = windows_core::CopyType;
}
impl Default for OVERLAPPED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for OVERLAPPED_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
impl windows_core::TypeKind for OVERLAPPED_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OVERLAPPED_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: usize,
    pub lpOverlapped: *mut OVERLAPPED,
    pub Internal: usize,
    pub dwNumberOfBytesTransferred: u32,
}
impl windows_core::TypeKind for OVERLAPPED_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for OVERLAPPED_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPOVERLAPPED_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED)>;
pub type PIO_APC_ROUTINE = Option<unsafe extern "system" fn(apccontext: *mut core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, reserved: u32)>;

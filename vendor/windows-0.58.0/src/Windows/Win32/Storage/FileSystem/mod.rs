#[inline]
pub unsafe fn AddLogContainer<P0, P1>(hlog: P0, pcbcontainer: Option<*const u64>, pwszcontainerpath: P1, preserved: Option<*mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clfsw32.dll" "system" fn AddLogContainer(hlog : super::super::Foundation:: HANDLE, pcbcontainer : *const u64, pwszcontainerpath : windows_core::PCWSTR, preserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    AddLogContainer(hlog.param().abi(), core::mem::transmute(pcbcontainer.unwrap_or(std::ptr::null())), pwszcontainerpath.param().abi(), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn AddLogContainerSet<P0>(hlog: P0, pcbcontainer: Option<*const u64>, rgwszcontainerpath: &[windows_core::PCWSTR], preserved: Option<*mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn AddLogContainerSet(hlog : super::super::Foundation:: HANDLE, ccontainer : u16, pcbcontainer : *const u64, rgwszcontainerpath : *const windows_core::PCWSTR, preserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    AddLogContainerSet(hlog.param().abi(), rgwszcontainerpath.len().try_into().unwrap(), core::mem::transmute(pcbcontainer.unwrap_or(std::ptr::null())), core::mem::transmute(rgwszcontainerpath.as_ptr()), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn AddUsersToEncryptedFile<P0>(lpfilename: P0, pencryptioncertificates: *const ENCRYPTION_CERTIFICATE_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddUsersToEncryptedFile(lpfilename : windows_core::PCWSTR, pencryptioncertificates : *const ENCRYPTION_CERTIFICATE_LIST) -> u32);
    AddUsersToEncryptedFile(lpfilename.param().abi(), pencryptioncertificates)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn AdvanceLogBase(pvmarshal: *mut core::ffi::c_void, plsnbase: *mut CLS_LSN, fflags: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn AdvanceLogBase(pvmarshal : *mut core::ffi::c_void, plsnbase : *mut CLS_LSN, fflags : u32, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    AdvanceLogBase(pvmarshal, plsnbase, fflags, poverlapped).ok()
}
#[inline]
pub unsafe fn AlignReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, rgcbreservation: *mut i64, pcbalignreservation: *mut i64) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn AlignReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, rgcbreservation : *mut i64, pcbalignreservation : *mut i64) -> super::super::Foundation:: BOOL);
    AlignReservedLog(pvmarshal, creservedrecords, rgcbreservation, pcbalignreservation).ok()
}
#[inline]
pub unsafe fn AllocReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn AllocReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : *mut i64) -> super::super::Foundation:: BOOL);
    AllocReservedLog(pvmarshal, creservedrecords, pcbadjustment).ok()
}
#[inline]
pub unsafe fn AreFileApisANSI() -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn AreFileApisANSI() -> super::super::Foundation:: BOOL);
    AreFileApisANSI()
}
#[inline]
pub unsafe fn AreShortNamesEnabled<P0>(handle: P0, enabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AreShortNamesEnabled(handle : super::super::Foundation:: HANDLE, enabled : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    AreShortNamesEnabled(handle.param().abi(), enabled)
}
#[inline]
pub unsafe fn BackupRead<P0, P1, P2>(hfile: P0, lpbuffer: &mut [u8], lpnumberofbytesread: *mut u32, babort: P1, bprocesssecurity: P2, lpcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn BackupRead(hfile : super::super::Foundation:: HANDLE, lpbuffer : *mut u8, nnumberofbytestoread : u32, lpnumberofbytesread : *mut u32, babort : super::super::Foundation:: BOOL, bprocesssecurity : super::super::Foundation:: BOOL, lpcontext : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    BackupRead(hfile.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofbytesread, babort.param().abi(), bprocesssecurity.param().abi(), lpcontext).ok()
}
#[inline]
pub unsafe fn BackupSeek<P0>(hfile: P0, dwlowbytestoseek: u32, dwhighbytestoseek: u32, lpdwlowbyteseeked: *mut u32, lpdwhighbyteseeked: *mut u32, lpcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn BackupSeek(hfile : super::super::Foundation:: HANDLE, dwlowbytestoseek : u32, dwhighbytestoseek : u32, lpdwlowbyteseeked : *mut u32, lpdwhighbyteseeked : *mut u32, lpcontext : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    BackupSeek(hfile.param().abi(), dwlowbytestoseek, dwhighbytestoseek, lpdwlowbyteseeked, lpdwhighbyteseeked, lpcontext).ok()
}
#[inline]
pub unsafe fn BackupWrite<P0, P1, P2>(hfile: P0, lpbuffer: &[u8], lpnumberofbyteswritten: *mut u32, babort: P1, bprocesssecurity: P2, lpcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn BackupWrite(hfile : super::super::Foundation:: HANDLE, lpbuffer : *const u8, nnumberofbytestowrite : u32, lpnumberofbyteswritten : *mut u32, babort : super::super::Foundation:: BOOL, bprocesssecurity : super::super::Foundation:: BOOL, lpcontext : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    BackupWrite(hfile.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofbyteswritten, babort.param().abi(), bprocesssecurity.param().abi(), lpcontext).ok()
}
#[inline]
pub unsafe fn BuildIoRingCancelRequest<P0>(ioring: P0, file: IORING_HANDLE_REF, optocancel: usize, userdata: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn BuildIoRingCancelRequest(ioring : HIORING, file : IORING_HANDLE_REF, optocancel : usize, userdata : usize) -> windows_core::HRESULT);
    BuildIoRingCancelRequest(ioring.param().abi(), core::mem::transmute(file), optocancel, userdata).ok()
}
#[inline]
pub unsafe fn BuildIoRingFlushFile<P0>(ioring: P0, fileref: IORING_HANDLE_REF, flushmode: FILE_FLUSH_MODE, userdata: usize, sqeflags: IORING_SQE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("kernel32.dll" "system" fn BuildIoRingFlushFile(ioring : HIORING, fileref : IORING_HANDLE_REF, flushmode : FILE_FLUSH_MODE, userdata : usize, sqeflags : IORING_SQE_FLAGS) -> windows_core::HRESULT);
    BuildIoRingFlushFile(ioring.param().abi(), core::mem::transmute(fileref), flushmode, userdata, sqeflags).ok()
}
#[inline]
pub unsafe fn BuildIoRingReadFile<P0>(ioring: P0, fileref: IORING_HANDLE_REF, dataref: IORING_BUFFER_REF, numberofbytestoread: u32, fileoffset: u64, userdata: usize, sqeflags: IORING_SQE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn BuildIoRingReadFile(ioring : HIORING, fileref : IORING_HANDLE_REF, dataref : IORING_BUFFER_REF, numberofbytestoread : u32, fileoffset : u64, userdata : usize, sqeflags : IORING_SQE_FLAGS) -> windows_core::HRESULT);
    BuildIoRingReadFile(ioring.param().abi(), core::mem::transmute(fileref), core::mem::transmute(dataref), numberofbytestoread, fileoffset, userdata, sqeflags).ok()
}
#[inline]
pub unsafe fn BuildIoRingRegisterBuffers<P0>(ioring: P0, buffers: &[IORING_BUFFER_INFO], userdata: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn BuildIoRingRegisterBuffers(ioring : HIORING, count : u32, buffers : *const IORING_BUFFER_INFO, userdata : usize) -> windows_core::HRESULT);
    BuildIoRingRegisterBuffers(ioring.param().abi(), buffers.len().try_into().unwrap(), core::mem::transmute(buffers.as_ptr()), userdata).ok()
}
#[inline]
pub unsafe fn BuildIoRingRegisterFileHandles<P0>(ioring: P0, handles: &[super::super::Foundation::HANDLE], userdata: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn BuildIoRingRegisterFileHandles(ioring : HIORING, count : u32, handles : *const super::super::Foundation:: HANDLE, userdata : usize) -> windows_core::HRESULT);
    BuildIoRingRegisterFileHandles(ioring.param().abi(), handles.len().try_into().unwrap(), core::mem::transmute(handles.as_ptr()), userdata).ok()
}
#[inline]
pub unsafe fn BuildIoRingWriteFile<P0>(ioring: P0, fileref: IORING_HANDLE_REF, bufferref: IORING_BUFFER_REF, numberofbytestowrite: u32, fileoffset: u64, writeflags: FILE_WRITE_FLAGS, userdata: usize, sqeflags: IORING_SQE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("kernel32.dll" "system" fn BuildIoRingWriteFile(ioring : HIORING, fileref : IORING_HANDLE_REF, bufferref : IORING_BUFFER_REF, numberofbytestowrite : u32, fileoffset : u64, writeflags : FILE_WRITE_FLAGS, userdata : usize, sqeflags : IORING_SQE_FLAGS) -> windows_core::HRESULT);
    BuildIoRingWriteFile(ioring.param().abi(), core::mem::transmute(fileref), core::mem::transmute(bufferref), numberofbytestowrite, fileoffset, writeflags, userdata, sqeflags).ok()
}
#[inline]
pub unsafe fn CheckNameLegalDOS8Dot3A<P0>(lpname: P0, lpoemname: Option<&mut [u8]>, pbnamecontainsspaces: Option<*mut super::super::Foundation::BOOL>, pbnamelegal: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CheckNameLegalDOS8Dot3A(lpname : windows_core::PCSTR, lpoemname : windows_core::PSTR, oemnamesize : u32, pbnamecontainsspaces : *mut super::super::Foundation:: BOOL, pbnamelegal : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    CheckNameLegalDOS8Dot3A(lpname.param().abi(), core::mem::transmute(lpoemname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpoemname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbnamecontainsspaces.unwrap_or(std::ptr::null_mut())), pbnamelegal).ok()
}
#[inline]
pub unsafe fn CheckNameLegalDOS8Dot3W<P0>(lpname: P0, lpoemname: Option<&mut [u8]>, pbnamecontainsspaces: Option<*mut super::super::Foundation::BOOL>, pbnamelegal: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CheckNameLegalDOS8Dot3W(lpname : windows_core::PCWSTR, lpoemname : windows_core::PSTR, oemnamesize : u32, pbnamecontainsspaces : *mut super::super::Foundation:: BOOL, pbnamelegal : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    CheckNameLegalDOS8Dot3W(lpname.param().abi(), core::mem::transmute(lpoemname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpoemname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbnamecontainsspaces.unwrap_or(std::ptr::null_mut())), pbnamelegal).ok()
}
#[inline]
pub unsafe fn CloseAndResetLogFile<P0>(hlog: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn CloseAndResetLogFile(hlog : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CloseAndResetLogFile(hlog.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseEncryptedFileRaw(pvcontext: *const core::ffi::c_void) {
    windows_targets::link!("advapi32.dll" "system" fn CloseEncryptedFileRaw(pvcontext : *const core::ffi::c_void));
    CloseEncryptedFileRaw(pvcontext)
}
#[inline]
pub unsafe fn CloseIoRing<P0>(ioring: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn CloseIoRing(ioring : HIORING) -> windows_core::HRESULT);
    CloseIoRing(ioring.param().abi()).ok()
}
#[inline]
pub unsafe fn CommitComplete<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn CommitComplete(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    CommitComplete(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn CommitEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn CommitEnlistment(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    CommitEnlistment(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn CommitTransaction<P0>(transactionhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CommitTransaction(transactionhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn CommitTransactionAsync<P0>(transactionhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn CommitTransactionAsync(transactionhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CommitTransactionAsync(transactionhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn CompareFileTime(lpfiletime1: *const super::super::Foundation::FILETIME, lpfiletime2: *const super::super::Foundation::FILETIME) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn CompareFileTime(lpfiletime1 : *const super::super::Foundation:: FILETIME, lpfiletime2 : *const super::super::Foundation:: FILETIME) -> i32);
    CompareFileTime(lpfiletime1, lpfiletime2)
}
#[inline]
pub unsafe fn CopyFile2<P0, P1>(pwszexistingfilename: P0, pwsznewfilename: P1, pextendedparameters: Option<*const COPYFILE2_EXTENDED_PARAMETERS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CopyFile2(pwszexistingfilename : windows_core::PCWSTR, pwsznewfilename : windows_core::PCWSTR, pextendedparameters : *const COPYFILE2_EXTENDED_PARAMETERS) -> windows_core::HRESULT);
    CopyFile2(pwszexistingfilename.param().abi(), pwsznewfilename.param().abi(), core::mem::transmute(pextendedparameters.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CopyFileA<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn CopyFileA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, bfailifexists : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    CopyFileA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), bfailifexists.param().abi()).ok()
}
#[inline]
pub unsafe fn CopyFileExA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<*const core::ffi::c_void>, pbcancel: Option<*mut super::super::Foundation::BOOL>, dwcopyflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CopyFileExA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, pbcancel : *mut super::super::Foundation:: BOOL, dwcopyflags : u32) -> super::super::Foundation:: BOOL);
    CopyFileExA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), core::mem::transmute(pbcancel.unwrap_or(std::ptr::null_mut())), dwcopyflags).ok()
}
#[inline]
pub unsafe fn CopyFileExW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<*const core::ffi::c_void>, pbcancel: Option<*mut super::super::Foundation::BOOL>, dwcopyflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CopyFileExW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, pbcancel : *mut super::super::Foundation:: BOOL, dwcopyflags : u32) -> super::super::Foundation:: BOOL);
    CopyFileExW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), core::mem::transmute(pbcancel.unwrap_or(std::ptr::null_mut())), dwcopyflags).ok()
}
#[inline]
pub unsafe fn CopyFileFromAppW<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CopyFileFromAppW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, bfailifexists : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    CopyFileFromAppW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), bfailifexists.param().abi())
}
#[inline]
pub unsafe fn CopyFileTransactedA<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<*const core::ffi::c_void>, pbcancel: Option<*const super::super::Foundation::BOOL>, dwcopyflags: u32, htransaction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CopyFileTransactedA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, pbcancel : *const super::super::Foundation:: BOOL, dwcopyflags : u32, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CopyFileTransactedA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), core::mem::transmute(pbcancel.unwrap_or(std::ptr::null())), dwcopyflags, htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn CopyFileTransactedW<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<*const core::ffi::c_void>, pbcancel: Option<*const super::super::Foundation::BOOL>, dwcopyflags: u32, htransaction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CopyFileTransactedW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, pbcancel : *const super::super::Foundation:: BOOL, dwcopyflags : u32, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CopyFileTransactedW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), core::mem::transmute(pbcancel.unwrap_or(std::ptr::null())), dwcopyflags, htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn CopyFileW<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn CopyFileW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, bfailifexists : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    CopyFileW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), bfailifexists.param().abi()).ok()
}
#[inline]
pub unsafe fn CopyLZFile(hfsource: i32, hfdest: i32) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn CopyLZFile(hfsource : i32, hfdest : i32) -> i32);
    CopyLZFile(hfsource, hfdest)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateDirectoryA<P0>(lppathname: P0, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateDirectoryA(lppathname : windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    CreateDirectoryA(lppathname.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateDirectoryExA<P0, P1>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateDirectoryExA(lptemplatedirectory : windows_core::PCSTR, lpnewdirectory : windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    CreateDirectoryExA(lptemplatedirectory.param().abi(), lpnewdirectory.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateDirectoryExW<P0, P1>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateDirectoryExW(lptemplatedirectory : windows_core::PCWSTR, lpnewdirectory : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    CreateDirectoryExW(lptemplatedirectory.param().abi(), lpnewdirectory.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateDirectoryFromAppW<P0>(lppathname: P0, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateDirectoryFromAppW(lppathname : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    CreateDirectoryFromAppW(lppathname.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateDirectoryTransactedA<P0, P1, P2>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, htransaction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateDirectoryTransactedA(lptemplatedirectory : windows_core::PCSTR, lpnewdirectory : windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CreateDirectoryTransactedA(lptemplatedirectory.param().abi(), lpnewdirectory.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), htransaction.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateDirectoryTransactedW<P0, P1, P2>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, htransaction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateDirectoryTransactedW(lptemplatedirectory : windows_core::PCWSTR, lpnewdirectory : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CreateDirectoryTransactedW(lptemplatedirectory.param().abi(), lpnewdirectory.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), htransaction.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateDirectoryW<P0>(lppathname: P0, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateDirectoryW(lppathname : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    CreateDirectoryW(lppathname.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateEnlistment<P0, P1>(lpenlistmentattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, resourcemanagerhandle: P0, transactionhandle: P1, notificationmask: u32, createoptions: u32, enlistmentkey: *mut core::ffi::c_void) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn CreateEnlistment(lpenlistmentattributes : *mut super::super::Security:: SECURITY_ATTRIBUTES, resourcemanagerhandle : super::super::Foundation:: HANDLE, transactionhandle : super::super::Foundation:: HANDLE, notificationmask : u32, createoptions : u32, enlistmentkey : *mut core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    let result__ = CreateEnlistment(lpenlistmentattributes, resourcemanagerhandle.param().abi(), transactionhandle.param().abi(), notificationmask, createoptions, enlistmentkey);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFile2<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, dwcreationdisposition: FILE_CREATION_DISPOSITION, pcreateexparams: Option<*const CREATEFILE2_EXTENDED_PARAMETERS>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFile2(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, dwcreationdisposition : FILE_CREATION_DISPOSITION, pcreateexparams : *const CREATEFILE2_EXTENDED_PARAMETERS) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFile2(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, core::mem::transmute(pcreateexparams.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFile2FromAppW<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, dwcreationdisposition: u32, pcreateexparams: Option<*const CREATEFILE2_EXTENDED_PARAMETERS>) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFile2FromAppW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, dwcreationdisposition : u32, pcreateexparams : *const CREATEFILE2_EXTENDED_PARAMETERS) -> super::super::Foundation:: HANDLE);
    CreateFile2FromAppW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, core::mem::transmute(pcreateexparams.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileA<P0, P1>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileA(lpfilename : windows_core::PCSTR, dwdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwcreationdisposition : FILE_CREATION_DISPOSITION, dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES, htemplatefile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileA(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileFromAppW<P0, P1>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: P1) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFileFromAppW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    CreateFileFromAppW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileTransactedA<P0, P1, P2>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: P1, htransaction: P2, pusminiversion: Option<*const TXFS_MINIVERSION>, lpextendedparameter: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileTransactedA(lpfilename : windows_core::PCSTR, dwdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwcreationdisposition : FILE_CREATION_DISPOSITION, dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES, htemplatefile : super::super::Foundation:: HANDLE, htransaction : super::super::Foundation:: HANDLE, pusminiversion : *const TXFS_MINIVERSION, lpextendedparameter : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileTransactedA(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.param().abi(), htransaction.param().abi(), core::mem::transmute(pusminiversion.unwrap_or(std::ptr::null())), core::mem::transmute(lpextendedparameter.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileTransactedW<P0, P1, P2>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: P1, htransaction: P2, pusminiversion: Option<*const TXFS_MINIVERSION>, lpextendedparameter: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileTransactedW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwcreationdisposition : FILE_CREATION_DISPOSITION, dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES, htemplatefile : super::super::Foundation:: HANDLE, htransaction : super::super::Foundation:: HANDLE, pusminiversion : *const TXFS_MINIVERSION, lpextendedparameter : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileTransactedW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.param().abi(), htransaction.param().abi(), core::mem::transmute(pusminiversion.unwrap_or(std::ptr::null())), core::mem::transmute(lpextendedparameter.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileW<P0, P1>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwcreationdisposition : FILE_CREATION_DISPOSITION, dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES, htemplatefile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateHardLinkA<P0, P1>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateHardLinkA(lpfilename : windows_core::PCSTR, lpexistingfilename : windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    CreateHardLinkA(lpfilename.param().abi(), lpexistingfilename.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateHardLinkTransactedA<P0, P1, P2>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, htransaction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateHardLinkTransactedA(lpfilename : windows_core::PCSTR, lpexistingfilename : windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CreateHardLinkTransactedA(lpfilename.param().abi(), lpexistingfilename.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), htransaction.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateHardLinkTransactedW<P0, P1, P2>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, htransaction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateHardLinkTransactedW(lpfilename : windows_core::PCWSTR, lpexistingfilename : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CreateHardLinkTransactedW(lpfilename.param().abi(), lpexistingfilename.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), htransaction.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateHardLinkW<P0, P1>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateHardLinkW(lpfilename : windows_core::PCWSTR, lpexistingfilename : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    CreateHardLinkW(lpfilename.param().abi(), lpexistingfilename.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CreateIoRing(ioringversion: IORING_VERSION, flags: IORING_CREATE_FLAGS, submissionqueuesize: u32, completionqueuesize: u32) -> windows_core::Result<HIORING> {
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn CreateIoRing(ioringversion : IORING_VERSION, flags : IORING_CREATE_FLAGS, submissionqueuesize : u32, completionqueuesize : u32, h : *mut HIORING) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateIoRing(ioringversion, core::mem::transmute(flags), submissionqueuesize, completionqueuesize, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CreateLogContainerScanContext<P0>(hlog: P0, cfromcontainer: u32, ccontainers: u32, escanmode: u8, pcxscan: *mut CLS_SCAN_CONTEXT, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn CreateLogContainerScanContext(hlog : super::super::Foundation:: HANDLE, cfromcontainer : u32, ccontainers : u32, escanmode : u8, pcxscan : *mut CLS_SCAN_CONTEXT, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    CreateLogContainerScanContext(hlog.param().abi(), cfromcontainer, ccontainers, escanmode, pcxscan, poverlapped).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateLogFile<P0>(pszlogfilename: P0, fdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES, fcreatedisposition: FILE_CREATION_DISPOSITION, fflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clfsw32.dll" "system" fn CreateLogFile(pszlogfilename : windows_core::PCWSTR, fdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, psalogfile : *mut super::super::Security:: SECURITY_ATTRIBUTES, fcreatedisposition : FILE_CREATION_DISPOSITION, fflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = CreateLogFile(pszlogfilename.param().abi(), fdesiredaccess, dwsharemode, psalogfile, fcreatedisposition, fflagsandattributes);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateLogMarshallingArea<P0>(hlog: P0, pfnallocbuffer: CLFS_BLOCK_ALLOCATION, pfnfreebuffer: CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: *mut core::ffi::c_void, cbmarshallingbuffer: u32, cmaxwritebuffers: u32, cmaxreadbuffers: u32, ppvmarshal: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn CreateLogMarshallingArea(hlog : super::super::Foundation:: HANDLE, pfnallocbuffer : CLFS_BLOCK_ALLOCATION, pfnfreebuffer : CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : *mut core::ffi::c_void, cbmarshallingbuffer : u32, cmaxwritebuffers : u32, cmaxreadbuffers : u32, ppvmarshal : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CreateLogMarshallingArea(hlog.param().abi(), pfnallocbuffer, pfnfreebuffer, pvblockalloccontext, cbmarshallingbuffer, cmaxwritebuffers, cmaxreadbuffers, ppvmarshal).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateResourceManager<P0, P1>(lpresourcemanagerattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, resourcemanagerid: *mut windows_core::GUID, createoptions: u32, tmhandle: P0, description: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ktmw32.dll" "system" fn CreateResourceManager(lpresourcemanagerattributes : *mut super::super::Security:: SECURITY_ATTRIBUTES, resourcemanagerid : *mut windows_core::GUID, createoptions : u32, tmhandle : super::super::Foundation:: HANDLE, description : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateResourceManager(lpresourcemanagerattributes, resourcemanagerid, createoptions, tmhandle.param().abi(), description.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateSymbolicLinkA<P0, P1>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: SYMBOLIC_LINK_FLAGS) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateSymbolicLinkA(lpsymlinkfilename : windows_core::PCSTR, lptargetfilename : windows_core::PCSTR, dwflags : SYMBOLIC_LINK_FLAGS) -> super::super::Foundation:: BOOLEAN);
    CreateSymbolicLinkA(lpsymlinkfilename.param().abi(), lptargetfilename.param().abi(), dwflags)
}
#[inline]
pub unsafe fn CreateSymbolicLinkTransactedA<P0, P1, P2>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: SYMBOLIC_LINK_FLAGS, htransaction: P2) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateSymbolicLinkTransactedA(lpsymlinkfilename : windows_core::PCSTR, lptargetfilename : windows_core::PCSTR, dwflags : SYMBOLIC_LINK_FLAGS, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOLEAN);
    CreateSymbolicLinkTransactedA(lpsymlinkfilename.param().abi(), lptargetfilename.param().abi(), dwflags, htransaction.param().abi())
}
#[inline]
pub unsafe fn CreateSymbolicLinkTransactedW<P0, P1, P2>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: SYMBOLIC_LINK_FLAGS, htransaction: P2) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateSymbolicLinkTransactedW(lpsymlinkfilename : windows_core::PCWSTR, lptargetfilename : windows_core::PCWSTR, dwflags : SYMBOLIC_LINK_FLAGS, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOLEAN);
    CreateSymbolicLinkTransactedW(lpsymlinkfilename.param().abi(), lptargetfilename.param().abi(), dwflags, htransaction.param().abi())
}
#[inline]
pub unsafe fn CreateSymbolicLinkW<P0, P1>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: SYMBOLIC_LINK_FLAGS) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateSymbolicLinkW(lpsymlinkfilename : windows_core::PCWSTR, lptargetfilename : windows_core::PCWSTR, dwflags : SYMBOLIC_LINK_FLAGS) -> super::super::Foundation:: BOOLEAN);
    CreateSymbolicLinkW(lpsymlinkfilename.param().abi(), lptargetfilename.param().abi(), dwflags)
}
#[inline]
pub unsafe fn CreateTapePartition<P0>(hdevice: P0, dwpartitionmethod: CREATE_TAPE_PARTITION_METHOD, dwcount: u32, dwsize: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateTapePartition(hdevice : super::super::Foundation:: HANDLE, dwpartitionmethod : CREATE_TAPE_PARTITION_METHOD, dwcount : u32, dwsize : u32) -> u32);
    CreateTapePartition(hdevice.param().abi(), dwpartitionmethod, dwcount, dwsize)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateTransaction<P0>(lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, uow: *mut windows_core::GUID, createoptions: u32, isolationlevel: u32, isolationflags: u32, timeout: u32, description: P0) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : *mut super::super::Security:: SECURITY_ATTRIBUTES, uow : *mut windows_core::GUID, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateTransaction(lptransactionattributes, uow, createoptions, isolationlevel, isolationflags, timeout, description.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateTransactionManager<P0>(lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, logfilename: P0, createoptions: u32, commitstrength: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ktmw32.dll" "system" fn CreateTransactionManager(lptransactionattributes : *mut super::super::Security:: SECURITY_ATTRIBUTES, logfilename : windows_core::PCWSTR, createoptions : u32, commitstrength : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateTransactionManager(lptransactionattributes, logfilename.param().abi(), createoptions, commitstrength);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn DecryptFileA<P0>(lpfilename: P0, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn DecryptFileA(lpfilename : windows_core::PCSTR, dwreserved : u32) -> super::super::Foundation:: BOOL);
    DecryptFileA(lpfilename.param().abi(), dwreserved).ok()
}
#[inline]
pub unsafe fn DecryptFileW<P0>(lpfilename: P0, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn DecryptFileW(lpfilename : windows_core::PCWSTR, dwreserved : u32) -> super::super::Foundation:: BOOL);
    DecryptFileW(lpfilename.param().abi(), dwreserved).ok()
}
#[inline]
pub unsafe fn DefineDosDeviceA<P0, P1>(dwflags: DEFINE_DOS_DEVICE_FLAGS, lpdevicename: P0, lptargetpath: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn DefineDosDeviceA(dwflags : DEFINE_DOS_DEVICE_FLAGS, lpdevicename : windows_core::PCSTR, lptargetpath : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    DefineDosDeviceA(dwflags, lpdevicename.param().abi(), lptargetpath.param().abi()).ok()
}
#[inline]
pub unsafe fn DefineDosDeviceW<P0, P1>(dwflags: DEFINE_DOS_DEVICE_FLAGS, lpdevicename: P0, lptargetpath: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn DefineDosDeviceW(dwflags : DEFINE_DOS_DEVICE_FLAGS, lpdevicename : windows_core::PCWSTR, lptargetpath : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    DefineDosDeviceW(dwflags, lpdevicename.param().abi(), lptargetpath.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteFileA<P0>(lpfilename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteFileA(lpfilename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    DeleteFileA(lpfilename.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteFileFromAppW<P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn DeleteFileFromAppW(lpfilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    DeleteFileFromAppW(lpfilename.param().abi())
}
#[inline]
pub unsafe fn DeleteFileTransactedA<P0, P1>(lpfilename: P0, htransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteFileTransactedA(lpfilename : windows_core::PCSTR, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DeleteFileTransactedA(lpfilename.param().abi(), htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteFileTransactedW<P0, P1>(lpfilename: P0, htransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteFileTransactedW(lpfilename : windows_core::PCWSTR, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DeleteFileTransactedW(lpfilename.param().abi(), htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteFileW<P0>(lpfilename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteFileW(lpfilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    DeleteFileW(lpfilename.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteLogByHandle<P0>(hlog: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn DeleteLogByHandle(hlog : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DeleteLogByHandle(hlog.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteLogFile<P0>(pszlogfilename: P0, pvreserved: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clfsw32.dll" "system" fn DeleteLogFile(pszlogfilename : windows_core::PCWSTR, pvreserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    DeleteLogFile(pszlogfilename.param().abi(), pvreserved).ok()
}
#[inline]
pub unsafe fn DeleteLogMarshallingArea(pvmarshal: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn DeleteLogMarshallingArea(pvmarshal : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    DeleteLogMarshallingArea(pvmarshal).ok()
}
#[inline]
pub unsafe fn DeleteVolumeMountPointA<P0>(lpszvolumemountpoint: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteVolumeMountPointA(lpszvolumemountpoint : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    DeleteVolumeMountPointA(lpszvolumemountpoint.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteVolumeMountPointW<P0>(lpszvolumemountpoint: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteVolumeMountPointW(lpszvolumemountpoint : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    DeleteVolumeMountPointW(lpszvolumemountpoint.param().abi()).ok()
}
#[inline]
pub unsafe fn DeregisterManageableLogClient<P0>(hlog: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn DeregisterManageableLogClient(hlog : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DeregisterManageableLogClient(hlog.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DuplicateEncryptionInfoFile<P0, P1>(srcfilename: P0, dstfilename: P1, dwcreationdistribution: u32, dwattributes: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn DuplicateEncryptionInfoFile(srcfilename : windows_core::PCWSTR, dstfilename : windows_core::PCWSTR, dwcreationdistribution : u32, dwattributes : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> u32);
    DuplicateEncryptionInfoFile(srcfilename.param().abi(), dstfilename.param().abi(), dwcreationdistribution, dwattributes, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn EncryptFileA<P0>(lpfilename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn EncryptFileA(lpfilename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    EncryptFileA(lpfilename.param().abi()).ok()
}
#[inline]
pub unsafe fn EncryptFileW<P0>(lpfilename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn EncryptFileW(lpfilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    EncryptFileW(lpfilename.param().abi()).ok()
}
#[inline]
pub unsafe fn EncryptionDisable<P0, P1>(dirpath: P0, disable: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn EncryptionDisable(dirpath : windows_core::PCWSTR, disable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    EncryptionDisable(dirpath.param().abi(), disable.param().abi()).ok()
}
#[inline]
pub unsafe fn EraseTape<P0, P1>(hdevice: P0, dwerasetype: ERASE_TAPE_TYPE, bimmediate: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn EraseTape(hdevice : super::super::Foundation:: HANDLE, dwerasetype : ERASE_TAPE_TYPE, bimmediate : super::super::Foundation:: BOOL) -> u32);
    EraseTape(hdevice.param().abi(), dwerasetype, bimmediate.param().abi())
}
#[inline]
pub unsafe fn FileEncryptionStatusA<P0>(lpfilename: P0, lpstatus: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn FileEncryptionStatusA(lpfilename : windows_core::PCSTR, lpstatus : *mut u32) -> super::super::Foundation:: BOOL);
    FileEncryptionStatusA(lpfilename.param().abi(), lpstatus).ok()
}
#[inline]
pub unsafe fn FileEncryptionStatusW<P0>(lpfilename: P0, lpstatus: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn FileEncryptionStatusW(lpfilename : windows_core::PCWSTR, lpstatus : *mut u32) -> super::super::Foundation:: BOOL);
    FileEncryptionStatusW(lpfilename.param().abi(), lpstatus).ok()
}
#[inline]
pub unsafe fn FileTimeToLocalFileTime(lpfiletime: *const super::super::Foundation::FILETIME, lplocalfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FileTimeToLocalFileTime(lpfiletime : *const super::super::Foundation:: FILETIME, lplocalfiletime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    FileTimeToLocalFileTime(lpfiletime, lplocalfiletime).ok()
}
#[inline]
pub unsafe fn FindClose(hfindfile: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FindClose(hfindfile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FindClose(hfindfile).ok()
}
#[inline]
pub unsafe fn FindCloseChangeNotification<P0>(hchangehandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindCloseChangeNotification(hchangehandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FindCloseChangeNotification(hchangehandle.param().abi()).ok()
}
#[inline]
pub unsafe fn FindFirstChangeNotificationA<P0, P1>(lppathname: P0, bwatchsubtree: P1, dwnotifyfilter: FILE_NOTIFY_CHANGE) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstChangeNotificationA(lppathname : windows_core::PCSTR, bwatchsubtree : super::super::Foundation:: BOOL, dwnotifyfilter : FILE_NOTIFY_CHANGE) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstChangeNotificationA(lppathname.param().abi(), bwatchsubtree.param().abi(), dwnotifyfilter);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstChangeNotificationW<P0, P1>(lppathname: P0, bwatchsubtree: P1, dwnotifyfilter: FILE_NOTIFY_CHANGE) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstChangeNotificationW(lppathname : windows_core::PCWSTR, bwatchsubtree : super::super::Foundation:: BOOL, dwnotifyfilter : FILE_NOTIFY_CHANGE) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstChangeNotificationW(lppathname.param().abi(), bwatchsubtree.param().abi(), dwnotifyfilter);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstFileA<P0>(lpfilename: P0, lpfindfiledata: *mut WIN32_FIND_DATAA) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstFileA(lpfilename : windows_core::PCSTR, lpfindfiledata : *mut WIN32_FIND_DATAA) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstFileA(lpfilename.param().abi(), lpfindfiledata);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstFileExA<P0>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: Option<*const core::ffi::c_void>, dwadditionalflags: FIND_FIRST_EX_FLAGS) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstFileExA(lpfilename : windows_core::PCSTR, finfolevelid : FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : FIND_FIRST_EX_FLAGS) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstFileExA(lpfilename.param().abi(), finfolevelid, lpfindfiledata, fsearchop, core::mem::transmute(lpsearchfilter.unwrap_or(std::ptr::null())), dwadditionalflags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstFileExFromAppW<P0>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: Option<*const core::ffi::c_void>, dwadditionalflags: u32) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn FindFirstFileExFromAppW(lpfilename : windows_core::PCWSTR, finfolevelid : FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32) -> super::super::Foundation:: HANDLE);
    FindFirstFileExFromAppW(lpfilename.param().abi(), finfolevelid, lpfindfiledata, fsearchop, core::mem::transmute(lpsearchfilter.unwrap_or(std::ptr::null())), dwadditionalflags)
}
#[inline]
pub unsafe fn FindFirstFileExW<P0>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: Option<*const core::ffi::c_void>, dwadditionalflags: FIND_FIRST_EX_FLAGS) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstFileExW(lpfilename : windows_core::PCWSTR, finfolevelid : FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : FIND_FIRST_EX_FLAGS) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstFileExW(lpfilename.param().abi(), finfolevelid, lpfindfiledata, fsearchop, core::mem::transmute(lpsearchfilter.unwrap_or(std::ptr::null())), dwadditionalflags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstFileNameTransactedW<P0, P1>(lpfilename: P0, dwflags: u32, stringlength: *mut u32, linkname: windows_core::PWSTR, htransaction: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstFileNameTransactedW(lpfilename : windows_core::PCWSTR, dwflags : u32, stringlength : *mut u32, linkname : windows_core::PWSTR, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstFileNameTransactedW(lpfilename.param().abi(), dwflags, stringlength, core::mem::transmute(linkname), htransaction.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstFileNameW<P0>(lpfilename: P0, dwflags: u32, stringlength: *mut u32, linkname: windows_core::PWSTR) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstFileNameW(lpfilename : windows_core::PCWSTR, dwflags : u32, stringlength : *mut u32, linkname : windows_core::PWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstFileNameW(lpfilename.param().abi(), dwflags, stringlength, core::mem::transmute(linkname));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstFileTransactedA<P0, P1>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: Option<*const core::ffi::c_void>, dwadditionalflags: u32, htransaction: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstFileTransactedA(lpfilename : windows_core::PCSTR, finfolevelid : FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstFileTransactedA(lpfilename.param().abi(), finfolevelid, lpfindfiledata, fsearchop, core::mem::transmute(lpsearchfilter.unwrap_or(std::ptr::null())), dwadditionalflags, htransaction.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstFileTransactedW<P0, P1>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: Option<*const core::ffi::c_void>, dwadditionalflags: u32, htransaction: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstFileTransactedW(lpfilename : windows_core::PCWSTR, finfolevelid : FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstFileTransactedW(lpfilename.param().abi(), finfolevelid, lpfindfiledata, fsearchop, core::mem::transmute(lpsearchfilter.unwrap_or(std::ptr::null())), dwadditionalflags, htransaction.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstFileW<P0>(lpfilename: P0, lpfindfiledata: *mut WIN32_FIND_DATAW) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstFileW(lpfilename : windows_core::PCWSTR, lpfindfiledata : *mut WIN32_FIND_DATAW) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstFileW(lpfilename.param().abi(), lpfindfiledata);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstStreamTransactedW<P0, P1>(lpfilename: P0, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut core::ffi::c_void, dwflags: u32, htransaction: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstStreamTransactedW(lpfilename : windows_core::PCWSTR, infolevel : STREAM_INFO_LEVELS, lpfindstreamdata : *mut core::ffi::c_void, dwflags : u32, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstStreamTransactedW(lpfilename.param().abi(), infolevel, lpfindstreamdata, dwflags, htransaction.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstStreamW<P0>(lpfilename: P0, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut core::ffi::c_void, dwflags: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstStreamW(lpfilename : windows_core::PCWSTR, infolevel : STREAM_INFO_LEVELS, lpfindstreamdata : *mut core::ffi::c_void, dwflags : u32) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstStreamW(lpfilename.param().abi(), infolevel, lpfindstreamdata, dwflags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstVolumeA(lpszvolumename: &mut [u8]) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn FindFirstVolumeA(lpszvolumename : windows_core::PSTR, cchbufferlength : u32) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstVolumeA(core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len().try_into().unwrap());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstVolumeMountPointA<P0>(lpszrootpathname: P0, lpszvolumemountpoint: &mut [u8]) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstVolumeMountPointA(lpszrootpathname : windows_core::PCSTR, lpszvolumemountpoint : windows_core::PSTR, cchbufferlength : u32) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstVolumeMountPointA(lpszrootpathname.param().abi(), core::mem::transmute(lpszvolumemountpoint.as_ptr()), lpszvolumemountpoint.len().try_into().unwrap());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstVolumeMountPointW<P0>(lpszrootpathname: P0, lpszvolumemountpoint: &mut [u16]) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindFirstVolumeMountPointW(lpszrootpathname : windows_core::PCWSTR, lpszvolumemountpoint : windows_core::PWSTR, cchbufferlength : u32) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstVolumeMountPointW(lpszrootpathname.param().abi(), core::mem::transmute(lpszvolumemountpoint.as_ptr()), lpszvolumemountpoint.len().try_into().unwrap());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindFirstVolumeW(lpszvolumename: &mut [u16]) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn FindFirstVolumeW(lpszvolumename : windows_core::PWSTR, cchbufferlength : u32) -> super::super::Foundation:: HANDLE);
    let result__ = FindFirstVolumeW(core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len().try_into().unwrap());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindNextChangeNotification<P0>(hchangehandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindNextChangeNotification(hchangehandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FindNextChangeNotification(hchangehandle.param().abi()).ok()
}
#[inline]
pub unsafe fn FindNextFileA<P0>(hfindfile: P0, lpfindfiledata: *mut WIN32_FIND_DATAA) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindNextFileA(hfindfile : super::super::Foundation:: HANDLE, lpfindfiledata : *mut WIN32_FIND_DATAA) -> super::super::Foundation:: BOOL);
    FindNextFileA(hfindfile.param().abi(), lpfindfiledata).ok()
}
#[inline]
pub unsafe fn FindNextFileNameW<P0>(hfindstream: P0, stringlength: *mut u32, linkname: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindNextFileNameW(hfindstream : super::super::Foundation:: HANDLE, stringlength : *mut u32, linkname : windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    FindNextFileNameW(hfindstream.param().abi(), stringlength, core::mem::transmute(linkname)).ok()
}
#[inline]
pub unsafe fn FindNextFileW<P0>(hfindfile: P0, lpfindfiledata: *mut WIN32_FIND_DATAW) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindNextFileW(hfindfile : super::super::Foundation:: HANDLE, lpfindfiledata : *mut WIN32_FIND_DATAW) -> super::super::Foundation:: BOOL);
    FindNextFileW(hfindfile.param().abi(), lpfindfiledata).ok()
}
#[inline]
pub unsafe fn FindNextStreamW<P0>(hfindstream: P0, lpfindstreamdata: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindNextStreamW(hfindstream : super::super::Foundation:: HANDLE, lpfindstreamdata : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    FindNextStreamW(hfindstream.param().abi(), lpfindstreamdata).ok()
}
#[inline]
pub unsafe fn FindNextVolumeA(hfindvolume: super::super::Foundation::HANDLE, lpszvolumename: &mut [u8]) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FindNextVolumeA(hfindvolume : super::super::Foundation:: HANDLE, lpszvolumename : windows_core::PSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    FindNextVolumeA(hfindvolume, core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn FindNextVolumeMountPointA<P0>(hfindvolumemountpoint: P0, lpszvolumemountpoint: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindNextVolumeMountPointA(hfindvolumemountpoint : super::super::Foundation:: HANDLE, lpszvolumemountpoint : windows_core::PSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    FindNextVolumeMountPointA(hfindvolumemountpoint.param().abi(), core::mem::transmute(lpszvolumemountpoint.as_ptr()), lpszvolumemountpoint.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn FindNextVolumeMountPointW<P0>(hfindvolumemountpoint: P0, lpszvolumemountpoint: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindNextVolumeMountPointW(hfindvolumemountpoint : super::super::Foundation:: HANDLE, lpszvolumemountpoint : windows_core::PWSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    FindNextVolumeMountPointW(hfindvolumemountpoint.param().abi(), core::mem::transmute(lpszvolumemountpoint.as_ptr()), lpszvolumemountpoint.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn FindNextVolumeW(hfindvolume: super::super::Foundation::HANDLE, lpszvolumename: &mut [u16]) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FindNextVolumeW(hfindvolume : super::super::Foundation:: HANDLE, lpszvolumename : windows_core::PWSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    FindNextVolumeW(hfindvolume, core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn FindVolumeClose<P0>(hfindvolume: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindVolumeClose(hfindvolume : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FindVolumeClose(hfindvolume.param().abi()).ok()
}
#[inline]
pub unsafe fn FindVolumeMountPointClose<P0>(hfindvolumemountpoint: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindVolumeMountPointClose(hfindvolumemountpoint : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FindVolumeMountPointClose(hfindvolumemountpoint.param().abi()).ok()
}
#[inline]
pub unsafe fn FlushFileBuffers<P0>(hfile: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FlushFileBuffers(hfile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FlushFileBuffers(hfile.param().abi()).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn FlushLogBuffers(pvmarshal: *const core::ffi::c_void, poverlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn FlushLogBuffers(pvmarshal : *const core::ffi::c_void, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    FlushLogBuffers(pvmarshal, core::mem::transmute(poverlapped.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn FlushLogToLsn(pvmarshalcontext: *mut core::ffi::c_void, plsnflush: *mut CLS_LSN, plsnlastflushed: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn FlushLogToLsn(pvmarshalcontext : *mut core::ffi::c_void, plsnflush : *mut CLS_LSN, plsnlastflushed : *mut CLS_LSN, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    FlushLogToLsn(pvmarshalcontext, plsnflush, plsnlastflushed, poverlapped).ok()
}
#[inline]
pub unsafe fn FreeEncryptedFileMetadata(pbmetadata: *const u8) {
    windows_targets::link!("advapi32.dll" "system" fn FreeEncryptedFileMetadata(pbmetadata : *const u8));
    FreeEncryptedFileMetadata(pbmetadata)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FreeEncryptionCertificateHashList(pusers: *const ENCRYPTION_CERTIFICATE_HASH_LIST) {
    windows_targets::link!("advapi32.dll" "system" fn FreeEncryptionCertificateHashList(pusers : *const ENCRYPTION_CERTIFICATE_HASH_LIST));
    FreeEncryptionCertificateHashList(pusers)
}
#[inline]
pub unsafe fn FreeReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn FreeReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : *mut i64) -> super::super::Foundation:: BOOL);
    FreeReservedLog(pvmarshal, creservedrecords, pcbadjustment).ok()
}
#[inline]
pub unsafe fn GetBinaryTypeA<P0>(lpapplicationname: P0, lpbinarytype: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetBinaryTypeA(lpapplicationname : windows_core::PCSTR, lpbinarytype : *mut u32) -> super::super::Foundation:: BOOL);
    GetBinaryTypeA(lpapplicationname.param().abi(), lpbinarytype).ok()
}
#[inline]
pub unsafe fn GetBinaryTypeW<P0>(lpapplicationname: P0, lpbinarytype: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetBinaryTypeW(lpapplicationname : windows_core::PCWSTR, lpbinarytype : *mut u32) -> super::super::Foundation:: BOOL);
    GetBinaryTypeW(lpapplicationname.param().abi(), lpbinarytype).ok()
}
#[inline]
pub unsafe fn GetCompressedFileSizeA<P0>(lpfilename: P0, lpfilesizehigh: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCompressedFileSizeA(lpfilename : windows_core::PCSTR, lpfilesizehigh : *mut u32) -> u32);
    GetCompressedFileSizeA(lpfilename.param().abi(), core::mem::transmute(lpfilesizehigh.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCompressedFileSizeTransactedA<P0, P1>(lpfilename: P0, lpfilesizehigh: Option<*mut u32>, htransaction: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCompressedFileSizeTransactedA(lpfilename : windows_core::PCSTR, lpfilesizehigh : *mut u32, htransaction : super::super::Foundation:: HANDLE) -> u32);
    GetCompressedFileSizeTransactedA(lpfilename.param().abi(), core::mem::transmute(lpfilesizehigh.unwrap_or(std::ptr::null_mut())), htransaction.param().abi())
}
#[inline]
pub unsafe fn GetCompressedFileSizeTransactedW<P0, P1>(lpfilename: P0, lpfilesizehigh: Option<*mut u32>, htransaction: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCompressedFileSizeTransactedW(lpfilename : windows_core::PCWSTR, lpfilesizehigh : *mut u32, htransaction : super::super::Foundation:: HANDLE) -> u32);
    GetCompressedFileSizeTransactedW(lpfilename.param().abi(), core::mem::transmute(lpfilesizehigh.unwrap_or(std::ptr::null_mut())), htransaction.param().abi())
}
#[inline]
pub unsafe fn GetCompressedFileSizeW<P0>(lpfilename: P0, lpfilesizehigh: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCompressedFileSizeW(lpfilename : windows_core::PCWSTR, lpfilesizehigh : *mut u32) -> u32);
    GetCompressedFileSizeW(lpfilename.param().abi(), core::mem::transmute(lpfilesizehigh.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCurrentClockTransactionManager<P0>(transactionmanagerhandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn GetCurrentClockTransactionManager(transactionmanagerhandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    GetCurrentClockTransactionManager(transactionmanagerhandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn GetDiskFreeSpaceA<P0>(lprootpathname: P0, lpsectorspercluster: Option<*mut u32>, lpbytespersector: Option<*mut u32>, lpnumberoffreeclusters: Option<*mut u32>, lptotalnumberofclusters: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDiskFreeSpaceA(lprootpathname : windows_core::PCSTR, lpsectorspercluster : *mut u32, lpbytespersector : *mut u32, lpnumberoffreeclusters : *mut u32, lptotalnumberofclusters : *mut u32) -> super::super::Foundation:: BOOL);
    GetDiskFreeSpaceA(lprootpathname.param().abi(), core::mem::transmute(lpsectorspercluster.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpbytespersector.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpnumberoffreeclusters.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lptotalnumberofclusters.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetDiskFreeSpaceExA<P0>(lpdirectoryname: P0, lpfreebytesavailabletocaller: Option<*mut u64>, lptotalnumberofbytes: Option<*mut u64>, lptotalnumberoffreebytes: Option<*mut u64>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExA(lpdirectoryname : windows_core::PCSTR, lpfreebytesavailabletocaller : *mut u64, lptotalnumberofbytes : *mut u64, lptotalnumberoffreebytes : *mut u64) -> super::super::Foundation:: BOOL);
    GetDiskFreeSpaceExA(lpdirectoryname.param().abi(), core::mem::transmute(lpfreebytesavailabletocaller.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lptotalnumberofbytes.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lptotalnumberoffreebytes.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetDiskFreeSpaceExW<P0>(lpdirectoryname: P0, lpfreebytesavailabletocaller: Option<*mut u64>, lptotalnumberofbytes: Option<*mut u64>, lptotalnumberoffreebytes: Option<*mut u64>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExW(lpdirectoryname : windows_core::PCWSTR, lpfreebytesavailabletocaller : *mut u64, lptotalnumberofbytes : *mut u64, lptotalnumberoffreebytes : *mut u64) -> super::super::Foundation:: BOOL);
    GetDiskFreeSpaceExW(lpdirectoryname.param().abi(), core::mem::transmute(lpfreebytesavailabletocaller.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lptotalnumberofbytes.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lptotalnumberoffreebytes.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetDiskFreeSpaceW<P0>(lprootpathname: P0, lpsectorspercluster: Option<*mut u32>, lpbytespersector: Option<*mut u32>, lpnumberoffreeclusters: Option<*mut u32>, lptotalnumberofclusters: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDiskFreeSpaceW(lprootpathname : windows_core::PCWSTR, lpsectorspercluster : *mut u32, lpbytespersector : *mut u32, lpnumberoffreeclusters : *mut u32, lptotalnumberofclusters : *mut u32) -> super::super::Foundation:: BOOL);
    GetDiskFreeSpaceW(lprootpathname.param().abi(), core::mem::transmute(lpsectorspercluster.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpbytespersector.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpnumberoffreeclusters.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lptotalnumberofclusters.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetDiskSpaceInformationA<P0>(rootpath: P0, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDiskSpaceInformationA(rootpath : windows_core::PCSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT);
    GetDiskSpaceInformationA(rootpath.param().abi(), diskspaceinfo).ok()
}
#[inline]
pub unsafe fn GetDiskSpaceInformationW<P0>(rootpath: P0, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDiskSpaceInformationW(rootpath : windows_core::PCWSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT);
    GetDiskSpaceInformationW(rootpath.param().abi(), diskspaceinfo).ok()
}
#[inline]
pub unsafe fn GetDriveTypeA<P0>(lprootpathname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDriveTypeA(lprootpathname : windows_core::PCSTR) -> u32);
    GetDriveTypeA(lprootpathname.param().abi())
}
#[inline]
pub unsafe fn GetDriveTypeW<P0>(lprootpathname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDriveTypeW(lprootpathname : windows_core::PCWSTR) -> u32);
    GetDriveTypeW(lprootpathname.param().abi())
}
#[inline]
pub unsafe fn GetEncryptedFileMetadata<P0>(lpfilename: P0, pcbmetadata: *mut u32, ppbmetadata: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetEncryptedFileMetadata(lpfilename : windows_core::PCWSTR, pcbmetadata : *mut u32, ppbmetadata : *mut *mut u8) -> u32);
    GetEncryptedFileMetadata(lpfilename.param().abi(), pcbmetadata, ppbmetadata)
}
#[inline]
pub unsafe fn GetEnlistmentId<P0>(enlistmenthandle: P0, enlistmentid: *mut windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn GetEnlistmentId(enlistmenthandle : super::super::Foundation:: HANDLE, enlistmentid : *mut windows_core::GUID) -> super::super::Foundation:: BOOL);
    GetEnlistmentId(enlistmenthandle.param().abi(), enlistmentid).ok()
}
#[inline]
pub unsafe fn GetEnlistmentRecoveryInformation<P0>(enlistmenthandle: P0, buffersize: u32, buffer: *mut core::ffi::c_void, bufferused: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn GetEnlistmentRecoveryInformation(enlistmenthandle : super::super::Foundation:: HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void, bufferused : *mut u32) -> super::super::Foundation:: BOOL);
    GetEnlistmentRecoveryInformation(enlistmenthandle.param().abi(), buffersize, buffer, bufferused).ok()
}
#[inline]
pub unsafe fn GetExpandedNameA<P0>(lpszsource: P0, lpszbuffer: &mut [u8; 260]) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetExpandedNameA(lpszsource : windows_core::PCSTR, lpszbuffer : windows_core::PSTR) -> i32);
    GetExpandedNameA(lpszsource.param().abi(), core::mem::transmute(lpszbuffer.as_ptr()))
}
#[inline]
pub unsafe fn GetExpandedNameW<P0>(lpszsource: P0, lpszbuffer: &mut [u16; 260]) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetExpandedNameW(lpszsource : windows_core::PCWSTR, lpszbuffer : windows_core::PWSTR) -> i32);
    GetExpandedNameW(lpszsource.param().abi(), core::mem::transmute(lpszbuffer.as_ptr()))
}
#[inline]
pub unsafe fn GetFileAttributesA<P0>(lpfilename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileAttributesA(lpfilename : windows_core::PCSTR) -> u32);
    GetFileAttributesA(lpfilename.param().abi())
}
#[inline]
pub unsafe fn GetFileAttributesExA<P0>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileAttributesExA(lpfilename : windows_core::PCSTR, finfolevelid : GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    GetFileAttributesExA(lpfilename.param().abi(), finfolevelid, lpfileinformation).ok()
}
#[inline]
pub unsafe fn GetFileAttributesExFromAppW<P0>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn GetFileAttributesExFromAppW(lpfilename : windows_core::PCWSTR, finfolevelid : GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    GetFileAttributesExFromAppW(lpfilename.param().abi(), finfolevelid, lpfileinformation)
}
#[inline]
pub unsafe fn GetFileAttributesExW<P0>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileAttributesExW(lpfilename : windows_core::PCWSTR, finfolevelid : GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    GetFileAttributesExW(lpfilename.param().abi(), finfolevelid, lpfileinformation).ok()
}
#[inline]
pub unsafe fn GetFileAttributesTransactedA<P0, P1>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut core::ffi::c_void, htransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileAttributesTransactedA(lpfilename : windows_core::PCSTR, finfolevelid : GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    GetFileAttributesTransactedA(lpfilename.param().abi(), finfolevelid, lpfileinformation, htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn GetFileAttributesTransactedW<P0, P1>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut core::ffi::c_void, htransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileAttributesTransactedW(lpfilename : windows_core::PCWSTR, finfolevelid : GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    GetFileAttributesTransactedW(lpfilename.param().abi(), finfolevelid, lpfileinformation, htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn GetFileAttributesW<P0>(lpfilename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileAttributesW(lpfilename : windows_core::PCWSTR) -> u32);
    GetFileAttributesW(lpfilename.param().abi())
}
#[inline]
pub unsafe fn GetFileBandwidthReservation<P0>(hfile: P0, lpperiodmilliseconds: *mut u32, lpbytesperperiod: *mut u32, pdiscardable: *mut super::super::Foundation::BOOL, lptransfersize: *mut u32, lpnumoutstandingrequests: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileBandwidthReservation(hfile : super::super::Foundation:: HANDLE, lpperiodmilliseconds : *mut u32, lpbytesperperiod : *mut u32, pdiscardable : *mut super::super::Foundation:: BOOL, lptransfersize : *mut u32, lpnumoutstandingrequests : *mut u32) -> super::super::Foundation:: BOOL);
    GetFileBandwidthReservation(hfile.param().abi(), lpperiodmilliseconds, lpbytesperperiod, pdiscardable, lptransfersize, lpnumoutstandingrequests).ok()
}
#[inline]
pub unsafe fn GetFileInformationByHandle<P0>(hfile: P0, lpfileinformation: *mut BY_HANDLE_FILE_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileInformationByHandle(hfile : super::super::Foundation:: HANDLE, lpfileinformation : *mut BY_HANDLE_FILE_INFORMATION) -> super::super::Foundation:: BOOL);
    GetFileInformationByHandle(hfile.param().abi(), lpfileinformation).ok()
}
#[inline]
pub unsafe fn GetFileInformationByHandleEx<P0>(hfile: P0, fileinformationclass: FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *mut core::ffi::c_void, dwbuffersize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileInformationByHandleEx(hfile : super::super::Foundation:: HANDLE, fileinformationclass : FILE_INFO_BY_HANDLE_CLASS, lpfileinformation : *mut core::ffi::c_void, dwbuffersize : u32) -> super::super::Foundation:: BOOL);
    GetFileInformationByHandleEx(hfile.param().abi(), fileinformationclass, lpfileinformation, dwbuffersize).ok()
}
#[inline]
pub unsafe fn GetFileSize<P0>(hfile: P0, lpfilesizehigh: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileSize(hfile : super::super::Foundation:: HANDLE, lpfilesizehigh : *mut u32) -> u32);
    GetFileSize(hfile.param().abi(), core::mem::transmute(lpfilesizehigh.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetFileSizeEx<P0>(hfile: P0, lpfilesize: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileSizeEx(hfile : super::super::Foundation:: HANDLE, lpfilesize : *mut i64) -> super::super::Foundation:: BOOL);
    GetFileSizeEx(hfile.param().abi(), lpfilesize).ok()
}
#[inline]
pub unsafe fn GetFileTime<P0>(hfile: P0, lpcreationtime: Option<*mut super::super::Foundation::FILETIME>, lplastaccesstime: Option<*mut super::super::Foundation::FILETIME>, lplastwritetime: Option<*mut super::super::Foundation::FILETIME>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileTime(hfile : super::super::Foundation:: HANDLE, lpcreationtime : *mut super::super::Foundation:: FILETIME, lplastaccesstime : *mut super::super::Foundation:: FILETIME, lplastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    GetFileTime(hfile.param().abi(), core::mem::transmute(lpcreationtime.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplastaccesstime.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplastwritetime.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetFileType<P0>(hfile: P0) -> FILE_TYPE
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileType(hfile : super::super::Foundation:: HANDLE) -> FILE_TYPE);
    GetFileType(hfile.param().abi())
}
#[inline]
pub unsafe fn GetFileVersionInfoA<P0>(lptstrfilename: P0, dwhandle: u32, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("version.dll" "system" fn GetFileVersionInfoA(lptstrfilename : windows_core::PCSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    GetFileVersionInfoA(lptstrfilename.param().abi(), dwhandle, dwlen, lpdata).ok()
}
#[inline]
pub unsafe fn GetFileVersionInfoExA<P0>(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: P0, dwhandle: u32, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("version.dll" "system" fn GetFileVersionInfoExA(dwflags : GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename : windows_core::PCSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    GetFileVersionInfoExA(dwflags, lpwstrfilename.param().abi(), dwhandle, dwlen, lpdata).ok()
}
#[inline]
pub unsafe fn GetFileVersionInfoExW<P0>(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: P0, dwhandle: u32, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("version.dll" "system" fn GetFileVersionInfoExW(dwflags : GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename : windows_core::PCWSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    GetFileVersionInfoExW(dwflags, lpwstrfilename.param().abi(), dwhandle, dwlen, lpdata).ok()
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeA<P0>(lptstrfilename: P0, lpdwhandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("version.dll" "system" fn GetFileVersionInfoSizeA(lptstrfilename : windows_core::PCSTR, lpdwhandle : *mut u32) -> u32);
    GetFileVersionInfoSizeA(lptstrfilename.param().abi(), core::mem::transmute(lpdwhandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeExA<P0>(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: P0, lpdwhandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("version.dll" "system" fn GetFileVersionInfoSizeExA(dwflags : GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename : windows_core::PCSTR, lpdwhandle : *mut u32) -> u32);
    GetFileVersionInfoSizeExA(dwflags, lpwstrfilename.param().abi(), lpdwhandle)
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeExW<P0>(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: P0, lpdwhandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("version.dll" "system" fn GetFileVersionInfoSizeExW(dwflags : GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename : windows_core::PCWSTR, lpdwhandle : *mut u32) -> u32);
    GetFileVersionInfoSizeExW(dwflags, lpwstrfilename.param().abi(), lpdwhandle)
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeW<P0>(lptstrfilename: P0, lpdwhandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("version.dll" "system" fn GetFileVersionInfoSizeW(lptstrfilename : windows_core::PCWSTR, lpdwhandle : *mut u32) -> u32);
    GetFileVersionInfoSizeW(lptstrfilename.param().abi(), core::mem::transmute(lpdwhandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetFileVersionInfoW<P0>(lptstrfilename: P0, dwhandle: u32, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("version.dll" "system" fn GetFileVersionInfoW(lptstrfilename : windows_core::PCWSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    GetFileVersionInfoW(lptstrfilename.param().abi(), dwhandle, dwlen, lpdata).ok()
}
#[inline]
pub unsafe fn GetFinalPathNameByHandleA<P0>(hfile: P0, lpszfilepath: &mut [u8], dwflags: GETFINALPATHNAMEBYHANDLE_FLAGS) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleA(hfile : super::super::Foundation:: HANDLE, lpszfilepath : windows_core::PSTR, cchfilepath : u32, dwflags : GETFINALPATHNAMEBYHANDLE_FLAGS) -> u32);
    GetFinalPathNameByHandleA(hfile.param().abi(), core::mem::transmute(lpszfilepath.as_ptr()), lpszfilepath.len().try_into().unwrap(), dwflags)
}
#[inline]
pub unsafe fn GetFinalPathNameByHandleW<P0>(hfile: P0, lpszfilepath: &mut [u16], dwflags: GETFINALPATHNAMEBYHANDLE_FLAGS) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleW(hfile : super::super::Foundation:: HANDLE, lpszfilepath : windows_core::PWSTR, cchfilepath : u32, dwflags : GETFINALPATHNAMEBYHANDLE_FLAGS) -> u32);
    GetFinalPathNameByHandleW(hfile.param().abi(), core::mem::transmute(lpszfilepath.as_ptr()), lpszfilepath.len().try_into().unwrap(), dwflags)
}
#[inline]
pub unsafe fn GetFullPathNameA<P0>(lpfilename: P0, lpbuffer: Option<&mut [u8]>, lpfilepart: Option<*mut windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFullPathNameA(lpfilename : windows_core::PCSTR, nbufferlength : u32, lpbuffer : windows_core::PSTR, lpfilepart : *mut windows_core::PSTR) -> u32);
    GetFullPathNameA(lpfilename.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(lpfilepart.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetFullPathNameTransactedA<P0, P1>(lpfilename: P0, lpbuffer: Option<&mut [u8]>, lpfilepart: Option<*mut windows_core::PSTR>, htransaction: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFullPathNameTransactedA(lpfilename : windows_core::PCSTR, nbufferlength : u32, lpbuffer : windows_core::PSTR, lpfilepart : *mut windows_core::PSTR, htransaction : super::super::Foundation:: HANDLE) -> u32);
    GetFullPathNameTransactedA(lpfilename.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(lpfilepart.unwrap_or(std::ptr::null_mut())), htransaction.param().abi())
}
#[inline]
pub unsafe fn GetFullPathNameTransactedW<P0, P1>(lpfilename: P0, lpbuffer: Option<&mut [u16]>, lpfilepart: Option<*mut windows_core::PWSTR>, htransaction: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFullPathNameTransactedW(lpfilename : windows_core::PCWSTR, nbufferlength : u32, lpbuffer : windows_core::PWSTR, lpfilepart : *mut windows_core::PWSTR, htransaction : super::super::Foundation:: HANDLE) -> u32);
    GetFullPathNameTransactedW(lpfilename.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(lpfilepart.unwrap_or(std::ptr::null_mut())), htransaction.param().abi())
}
#[inline]
pub unsafe fn GetFullPathNameW<P0>(lpfilename: P0, lpbuffer: Option<&mut [u16]>, lpfilepart: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFullPathNameW(lpfilename : windows_core::PCWSTR, nbufferlength : u32, lpbuffer : windows_core::PWSTR, lpfilepart : *mut windows_core::PWSTR) -> u32);
    GetFullPathNameW(lpfilename.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(lpfilepart.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetIoRingInfo<P0>(ioring: P0, info: *mut IORING_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn GetIoRingInfo(ioring : HIORING, info : *mut IORING_INFO) -> windows_core::HRESULT);
    GetIoRingInfo(ioring.param().abi(), info).ok()
}
#[inline]
pub unsafe fn GetLogContainerName<P0, P1>(hlog: P0, cidlogicalcontainer: u32, pwstrcontainername: P1, clencontainername: u32, pcactuallencontainername: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clfsw32.dll" "system" fn GetLogContainerName(hlog : super::super::Foundation:: HANDLE, cidlogicalcontainer : u32, pwstrcontainername : windows_core::PCWSTR, clencontainername : u32, pcactuallencontainername : *mut u32) -> super::super::Foundation:: BOOL);
    GetLogContainerName(hlog.param().abi(), cidlogicalcontainer, pwstrcontainername.param().abi(), clencontainername, pcactuallencontainername).ok()
}
#[inline]
pub unsafe fn GetLogFileInformation<P0>(hlog: P0, pinfobuffer: *mut CLS_INFORMATION, cbbuffer: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn GetLogFileInformation(hlog : super::super::Foundation:: HANDLE, pinfobuffer : *mut CLS_INFORMATION, cbbuffer : *mut u32) -> super::super::Foundation:: BOOL);
    GetLogFileInformation(hlog.param().abi(), pinfobuffer, cbbuffer).ok()
}
#[inline]
pub unsafe fn GetLogIoStatistics<P0>(hlog: P0, pvstatsbuffer: *mut core::ffi::c_void, cbstatsbuffer: u32, estatsclass: CLFS_IOSTATS_CLASS, pcbstatswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn GetLogIoStatistics(hlog : super::super::Foundation:: HANDLE, pvstatsbuffer : *mut core::ffi::c_void, cbstatsbuffer : u32, estatsclass : CLFS_IOSTATS_CLASS, pcbstatswritten : *mut u32) -> super::super::Foundation:: BOOL);
    GetLogIoStatistics(hlog.param().abi(), pvstatsbuffer, cbstatsbuffer, estatsclass, pcbstatswritten).ok()
}
#[inline]
pub unsafe fn GetLogReservationInfo(pvmarshal: *const core::ffi::c_void, pcbrecordnumber: *mut u32, pcbuserreservation: *mut i64, pcbcommitreservation: *mut i64) -> super::super::Foundation::BOOL {
    windows_targets::link!("clfsw32.dll" "system" fn GetLogReservationInfo(pvmarshal : *const core::ffi::c_void, pcbrecordnumber : *mut u32, pcbuserreservation : *mut i64, pcbcommitreservation : *mut i64) -> super::super::Foundation:: BOOL);
    GetLogReservationInfo(pvmarshal, pcbrecordnumber, pcbuserreservation, pcbcommitreservation)
}
#[inline]
pub unsafe fn GetLogicalDriveStringsA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetLogicalDriveStringsA(nbufferlength : u32, lpbuffer : windows_core::PSTR) -> u32);
    GetLogicalDriveStringsA(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetLogicalDriveStringsW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetLogicalDriveStringsW(nbufferlength : u32, lpbuffer : windows_core::PWSTR) -> u32);
    GetLogicalDriveStringsW(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetLogicalDrives() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetLogicalDrives() -> u32);
    GetLogicalDrives()
}
#[inline]
pub unsafe fn GetLongPathNameA<P0>(lpszshortpath: P0, lpszlongpath: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetLongPathNameA(lpszshortpath : windows_core::PCSTR, lpszlongpath : windows_core::PSTR, cchbuffer : u32) -> u32);
    GetLongPathNameA(lpszshortpath.param().abi(), core::mem::transmute(lpszlongpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszlongpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetLongPathNameTransactedA<P0, P1>(lpszshortpath: P0, lpszlongpath: Option<&mut [u8]>, htransaction: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetLongPathNameTransactedA(lpszshortpath : windows_core::PCSTR, lpszlongpath : windows_core::PSTR, cchbuffer : u32, htransaction : super::super::Foundation:: HANDLE) -> u32);
    GetLongPathNameTransactedA(lpszshortpath.param().abi(), core::mem::transmute(lpszlongpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszlongpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), htransaction.param().abi())
}
#[inline]
pub unsafe fn GetLongPathNameTransactedW<P0, P1>(lpszshortpath: P0, lpszlongpath: Option<&mut [u16]>, htransaction: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetLongPathNameTransactedW(lpszshortpath : windows_core::PCWSTR, lpszlongpath : windows_core::PWSTR, cchbuffer : u32, htransaction : super::super::Foundation:: HANDLE) -> u32);
    GetLongPathNameTransactedW(lpszshortpath.param().abi(), core::mem::transmute(lpszlongpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszlongpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), htransaction.param().abi())
}
#[inline]
pub unsafe fn GetLongPathNameW<P0>(lpszshortpath: P0, lpszlongpath: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetLongPathNameW(lpszshortpath : windows_core::PCWSTR, lpszlongpath : windows_core::PWSTR, cchbuffer : u32) -> u32);
    GetLongPathNameW(lpszshortpath.param().abi(), core::mem::transmute(lpszlongpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszlongpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetNextLogArchiveExtent(pvarchivecontext: *mut core::ffi::c_void, rgadextent: *mut CLS_ARCHIVE_DESCRIPTOR, cdescriptors: u32, pcdescriptorsreturned: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn GetNextLogArchiveExtent(pvarchivecontext : *mut core::ffi::c_void, rgadextent : *mut CLS_ARCHIVE_DESCRIPTOR, cdescriptors : u32, pcdescriptorsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    GetNextLogArchiveExtent(pvarchivecontext, rgadextent, cdescriptors, pcdescriptorsreturned).ok()
}
#[inline]
pub unsafe fn GetNotificationResourceManager<P0>(resourcemanagerhandle: P0, transactionnotification: *mut TRANSACTION_NOTIFICATION, notificationlength: u32, dwmilliseconds: u32, returnlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn GetNotificationResourceManager(resourcemanagerhandle : super::super::Foundation:: HANDLE, transactionnotification : *mut TRANSACTION_NOTIFICATION, notificationlength : u32, dwmilliseconds : u32, returnlength : *mut u32) -> super::super::Foundation:: BOOL);
    GetNotificationResourceManager(resourcemanagerhandle.param().abi(), transactionnotification, notificationlength, dwmilliseconds, returnlength).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn GetNotificationResourceManagerAsync<P0>(resourcemanagerhandle: P0, transactionnotification: *mut TRANSACTION_NOTIFICATION, transactionnotificationlength: u32, returnlength: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn GetNotificationResourceManagerAsync(resourcemanagerhandle : super::super::Foundation:: HANDLE, transactionnotification : *mut TRANSACTION_NOTIFICATION, transactionnotificationlength : u32, returnlength : *mut u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    GetNotificationResourceManagerAsync(resourcemanagerhandle.param().abi(), transactionnotification, transactionnotificationlength, returnlength, lpoverlapped).ok()
}
#[inline]
pub unsafe fn GetShortPathNameA<P0>(lpszlongpath: P0, lpszshortpath: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetShortPathNameA(lpszlongpath : windows_core::PCSTR, lpszshortpath : windows_core::PSTR, cchbuffer : u32) -> u32);
    GetShortPathNameA(lpszlongpath.param().abi(), core::mem::transmute(lpszshortpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszshortpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetShortPathNameW<P0>(lpszlongpath: P0, lpszshortpath: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetShortPathNameW(lpszlongpath : windows_core::PCWSTR, lpszshortpath : windows_core::PWSTR, cchbuffer : u32) -> u32);
    GetShortPathNameW(lpszlongpath.param().abi(), core::mem::transmute(lpszshortpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszshortpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetTapeParameters<P0>(hdevice: P0, dwoperation: GET_TAPE_DRIVE_PARAMETERS_OPERATION, lpdwsize: *mut u32, lptapeinformation: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetTapeParameters(hdevice : super::super::Foundation:: HANDLE, dwoperation : GET_TAPE_DRIVE_PARAMETERS_OPERATION, lpdwsize : *mut u32, lptapeinformation : *mut core::ffi::c_void) -> u32);
    GetTapeParameters(hdevice.param().abi(), dwoperation, lpdwsize, lptapeinformation)
}
#[inline]
pub unsafe fn GetTapePosition<P0>(hdevice: P0, dwpositiontype: TAPE_POSITION_TYPE, lpdwpartition: *mut u32, lpdwoffsetlow: *mut u32, lpdwoffsethigh: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetTapePosition(hdevice : super::super::Foundation:: HANDLE, dwpositiontype : TAPE_POSITION_TYPE, lpdwpartition : *mut u32, lpdwoffsetlow : *mut u32, lpdwoffsethigh : *mut u32) -> u32);
    GetTapePosition(hdevice.param().abi(), dwpositiontype, lpdwpartition, lpdwoffsetlow, lpdwoffsethigh)
}
#[inline]
pub unsafe fn GetTapeStatus<P0>(hdevice: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetTapeStatus(hdevice : super::super::Foundation:: HANDLE) -> u32);
    GetTapeStatus(hdevice.param().abi())
}
#[inline]
pub unsafe fn GetTempFileNameA<P0, P1>(lppathname: P0, lpprefixstring: P1, uunique: u32, lptempfilename: &mut [u8; 260]) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetTempFileNameA(lppathname : windows_core::PCSTR, lpprefixstring : windows_core::PCSTR, uunique : u32, lptempfilename : windows_core::PSTR) -> u32);
    GetTempFileNameA(lppathname.param().abi(), lpprefixstring.param().abi(), uunique, core::mem::transmute(lptempfilename.as_ptr()))
}
#[inline]
pub unsafe fn GetTempFileNameW<P0, P1>(lppathname: P0, lpprefixstring: P1, uunique: u32, lptempfilename: &mut [u16; 260]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetTempFileNameW(lppathname : windows_core::PCWSTR, lpprefixstring : windows_core::PCWSTR, uunique : u32, lptempfilename : windows_core::PWSTR) -> u32);
    GetTempFileNameW(lppathname.param().abi(), lpprefixstring.param().abi(), uunique, core::mem::transmute(lptempfilename.as_ptr()))
}
#[inline]
pub unsafe fn GetTempPath2A(buffer: Option<&mut [u8]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetTempPath2A(bufferlength : u32, buffer : windows_core::PSTR) -> u32);
    GetTempPath2A(buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetTempPath2W(buffer: Option<&mut [u16]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetTempPath2W(bufferlength : u32, buffer : windows_core::PWSTR) -> u32);
    GetTempPath2W(buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetTempPathA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetTempPathA(nbufferlength : u32, lpbuffer : windows_core::PSTR) -> u32);
    GetTempPathA(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetTempPathW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetTempPathW(nbufferlength : u32, lpbuffer : windows_core::PWSTR) -> u32);
    GetTempPathW(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetTransactionId<P0>(transactionhandle: P0, transactionid: *mut windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn GetTransactionId(transactionhandle : super::super::Foundation:: HANDLE, transactionid : *mut windows_core::GUID) -> super::super::Foundation:: BOOL);
    GetTransactionId(transactionhandle.param().abi(), transactionid).ok()
}
#[inline]
pub unsafe fn GetTransactionInformation<P0>(transactionhandle: P0, outcome: *mut u32, isolationlevel: *mut u32, isolationflags: *mut u32, timeout: *mut u32, description: Option<&mut [u16]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn GetTransactionInformation(transactionhandle : super::super::Foundation:: HANDLE, outcome : *mut u32, isolationlevel : *mut u32, isolationflags : *mut u32, timeout : *mut u32, bufferlength : u32, description : windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    GetTransactionInformation(transactionhandle.param().abi(), outcome, isolationlevel, isolationflags, timeout, description.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(description.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[inline]
pub unsafe fn GetTransactionManagerId<P0>(transactionmanagerhandle: P0, transactionmanagerid: *mut windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn GetTransactionManagerId(transactionmanagerhandle : super::super::Foundation:: HANDLE, transactionmanagerid : *mut windows_core::GUID) -> super::super::Foundation:: BOOL);
    GetTransactionManagerId(transactionmanagerhandle.param().abi(), transactionmanagerid).ok()
}
#[inline]
pub unsafe fn GetVolumeInformationA<P0>(lprootpathname: P0, lpvolumenamebuffer: Option<&mut [u8]>, lpvolumeserialnumber: Option<*mut u32>, lpmaximumcomponentlength: Option<*mut u32>, lpfilesystemflags: Option<*mut u32>, lpfilesystemnamebuffer: Option<&mut [u8]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumeInformationA(lprootpathname : windows_core::PCSTR, lpvolumenamebuffer : windows_core::PSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_core::PSTR, nfilesystemnamesize : u32) -> super::super::Foundation:: BOOL);
    GetVolumeInformationA(
        lprootpathname.param().abi(),
        core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(lpvolumeserialnumber.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpmaximumcomponentlength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpfilesystemflags.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
    .ok()
}
#[inline]
pub unsafe fn GetVolumeInformationByHandleW<P0>(hfile: P0, lpvolumenamebuffer: Option<&mut [u16]>, lpvolumeserialnumber: Option<*mut u32>, lpmaximumcomponentlength: Option<*mut u32>, lpfilesystemflags: Option<*mut u32>, lpfilesystemnamebuffer: Option<&mut [u16]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumeInformationByHandleW(hfile : super::super::Foundation:: HANDLE, lpvolumenamebuffer : windows_core::PWSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_core::PWSTR, nfilesystemnamesize : u32) -> super::super::Foundation:: BOOL);
    GetVolumeInformationByHandleW(
        hfile.param().abi(),
        core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(lpvolumeserialnumber.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpmaximumcomponentlength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpfilesystemflags.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
    .ok()
}
#[inline]
pub unsafe fn GetVolumeInformationW<P0>(lprootpathname: P0, lpvolumenamebuffer: Option<&mut [u16]>, lpvolumeserialnumber: Option<*mut u32>, lpmaximumcomponentlength: Option<*mut u32>, lpfilesystemflags: Option<*mut u32>, lpfilesystemnamebuffer: Option<&mut [u16]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumeInformationW(lprootpathname : windows_core::PCWSTR, lpvolumenamebuffer : windows_core::PWSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_core::PWSTR, nfilesystemnamesize : u32) -> super::super::Foundation:: BOOL);
    GetVolumeInformationW(
        lprootpathname.param().abi(),
        core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(lpvolumeserialnumber.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpmaximumcomponentlength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpfilesystemflags.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
    .ok()
}
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointA<P0>(lpszvolumemountpoint: P0, lpszvolumename: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumeNameForVolumeMountPointA(lpszvolumemountpoint : windows_core::PCSTR, lpszvolumename : windows_core::PSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    GetVolumeNameForVolumeMountPointA(lpszvolumemountpoint.param().abi(), core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointW<P0>(lpszvolumemountpoint: P0, lpszvolumename: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint : windows_core::PCWSTR, lpszvolumename : windows_core::PWSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint.param().abi(), core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn GetVolumePathNameA<P0>(lpszfilename: P0, lpszvolumepathname: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumePathNameA(lpszfilename : windows_core::PCSTR, lpszvolumepathname : windows_core::PSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    GetVolumePathNameA(lpszfilename.param().abi(), core::mem::transmute(lpszvolumepathname.as_ptr()), lpszvolumepathname.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn GetVolumePathNameW<P0>(lpszfilename: P0, lpszvolumepathname: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumePathNameW(lpszfilename : windows_core::PCWSTR, lpszvolumepathname : windows_core::PWSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    GetVolumePathNameW(lpszfilename.param().abi(), core::mem::transmute(lpszvolumepathname.as_ptr()), lpszvolumepathname.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameA<P0>(lpszvolumename: P0, lpszvolumepathnames: Option<&mut [u8]>, lpcchreturnlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumePathNamesForVolumeNameA(lpszvolumename : windows_core::PCSTR, lpszvolumepathnames : windows_core::PSTR, cchbufferlength : u32, lpcchreturnlength : *mut u32) -> super::super::Foundation:: BOOL);
    GetVolumePathNamesForVolumeNameA(lpszvolumename.param().abi(), core::mem::transmute(lpszvolumepathnames.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszvolumepathnames.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpcchreturnlength).ok()
}
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameW<P0>(lpszvolumename: P0, lpszvolumepathnames: Option<&mut [u16]>, lpcchreturnlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetVolumePathNamesForVolumeNameW(lpszvolumename : windows_core::PCWSTR, lpszvolumepathnames : windows_core::PWSTR, cchbufferlength : u32, lpcchreturnlength : *mut u32) -> super::super::Foundation:: BOOL);
    GetVolumePathNamesForVolumeNameW(lpszvolumename.param().abi(), core::mem::transmute(lpszvolumepathnames.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszvolumepathnames.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpcchreturnlength).ok()
}
#[inline]
pub unsafe fn HandleLogFull<P0>(hlog: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn HandleLogFull(hlog : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    HandleLogFull(hlog.param().abi()).ok()
}
#[inline]
pub unsafe fn InstallLogPolicy<P0>(hlog: P0, ppolicy: *mut CLFS_MGMT_POLICY) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn InstallLogPolicy(hlog : super::super::Foundation:: HANDLE, ppolicy : *mut CLFS_MGMT_POLICY) -> super::super::Foundation:: BOOL);
    InstallLogPolicy(hlog.param().abi(), ppolicy).ok()
}
#[inline]
pub unsafe fn IsIoRingOpSupported<P0>(ioring: P0, op: IORING_OP_CODE) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn IsIoRingOpSupported(ioring : HIORING, op : IORING_OP_CODE) -> super::super::Foundation:: BOOL);
    IsIoRingOpSupported(ioring.param().abi(), op)
}
#[inline]
pub unsafe fn LZClose(hfile: i32) {
    windows_targets::link!("kernel32.dll" "system" fn LZClose(hfile : i32));
    LZClose(hfile)
}
#[inline]
pub unsafe fn LZCopy(hfsource: i32, hfdest: i32) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn LZCopy(hfsource : i32, hfdest : i32) -> i32);
    LZCopy(hfsource, hfdest)
}
#[inline]
pub unsafe fn LZDone() {
    windows_targets::link!("kernel32.dll" "system" fn LZDone());
    LZDone()
}
#[inline]
pub unsafe fn LZInit(hfsource: i32) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn LZInit(hfsource : i32) -> i32);
    LZInit(hfsource)
}
#[inline]
pub unsafe fn LZOpenFileA<P0>(lpfilename: P0, lpreopenbuf: *mut OFSTRUCT, wstyle: LZOPENFILE_STYLE) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn LZOpenFileA(lpfilename : windows_core::PCSTR, lpreopenbuf : *mut OFSTRUCT, wstyle : LZOPENFILE_STYLE) -> i32);
    LZOpenFileA(lpfilename.param().abi(), lpreopenbuf, wstyle)
}
#[inline]
pub unsafe fn LZOpenFileW<P0>(lpfilename: P0, lpreopenbuf: *mut OFSTRUCT, wstyle: LZOPENFILE_STYLE) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn LZOpenFileW(lpfilename : windows_core::PCWSTR, lpreopenbuf : *mut OFSTRUCT, wstyle : LZOPENFILE_STYLE) -> i32);
    LZOpenFileW(lpfilename.param().abi(), lpreopenbuf, wstyle)
}
#[inline]
pub unsafe fn LZRead(hfile: i32, lpbuffer: &mut [u8]) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn LZRead(hfile : i32, lpbuffer : windows_core::PSTR, cbread : i32) -> i32);
    LZRead(hfile, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn LZSeek(hfile: i32, loffset: i32, iorigin: i32) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn LZSeek(hfile : i32, loffset : i32, iorigin : i32) -> i32);
    LZSeek(hfile, loffset, iorigin)
}
#[inline]
pub unsafe fn LZStart() -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn LZStart() -> i32);
    LZStart()
}
#[inline]
pub unsafe fn LocalFileTimeToFileTime(lplocalfiletime: *const super::super::Foundation::FILETIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn LocalFileTimeToFileTime(lplocalfiletime : *const super::super::Foundation:: FILETIME, lpfiletime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    LocalFileTimeToFileTime(lplocalfiletime, lpfiletime).ok()
}
#[inline]
pub unsafe fn LockFile<P0>(hfile: P0, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn LockFile(hfile : super::super::Foundation:: HANDLE, dwfileoffsetlow : u32, dwfileoffsethigh : u32, nnumberofbytestolocklow : u32, nnumberofbytestolockhigh : u32) -> super::super::Foundation:: BOOL);
    LockFile(hfile.param().abi(), dwfileoffsetlow, dwfileoffsethigh, nnumberofbytestolocklow, nnumberofbytestolockhigh).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn LockFileEx<P0>(hfile: P0, dwflags: LOCK_FILE_FLAGS, dwreserved: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn LockFileEx(hfile : super::super::Foundation:: HANDLE, dwflags : LOCK_FILE_FLAGS, dwreserved : u32, nnumberofbytestolocklow : u32, nnumberofbytestolockhigh : u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    LockFileEx(hfile.param().abi(), dwflags, dwreserved, nnumberofbytestolocklow, nnumberofbytestolockhigh, lpoverlapped).ok()
}
#[inline]
pub unsafe fn LogTailAdvanceFailure<P0>(hlog: P0, dwreason: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn LogTailAdvanceFailure(hlog : super::super::Foundation:: HANDLE, dwreason : u32) -> super::super::Foundation:: BOOL);
    LogTailAdvanceFailure(hlog.param().abi(), dwreason).ok()
}
#[inline]
pub unsafe fn LsnBlockOffset(plsn: *const CLS_LSN) -> u32 {
    windows_targets::link!("clfsw32.dll" "system" fn LsnBlockOffset(plsn : *const CLS_LSN) -> u32);
    LsnBlockOffset(plsn)
}
#[inline]
pub unsafe fn LsnContainer(plsn: *const CLS_LSN) -> u32 {
    windows_targets::link!("clfsw32.dll" "system" fn LsnContainer(plsn : *const CLS_LSN) -> u32);
    LsnContainer(plsn)
}
#[inline]
pub unsafe fn LsnCreate(cidcontainer: u32, offblock: u32, crecord: u32) -> CLS_LSN {
    windows_targets::link!("clfsw32.dll" "system" fn LsnCreate(cidcontainer : u32, offblock : u32, crecord : u32) -> CLS_LSN);
    LsnCreate(cidcontainer, offblock, crecord)
}
#[inline]
pub unsafe fn LsnEqual(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("clfsw32.dll" "system" fn LsnEqual(plsn1 : *const CLS_LSN, plsn2 : *const CLS_LSN) -> super::super::Foundation:: BOOLEAN);
    LsnEqual(plsn1, plsn2)
}
#[inline]
pub unsafe fn LsnGreater(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("clfsw32.dll" "system" fn LsnGreater(plsn1 : *const CLS_LSN, plsn2 : *const CLS_LSN) -> super::super::Foundation:: BOOLEAN);
    LsnGreater(plsn1, plsn2)
}
#[inline]
pub unsafe fn LsnIncrement(plsn: *const CLS_LSN) -> CLS_LSN {
    windows_targets::link!("clfsw32.dll" "system" fn LsnIncrement(plsn : *const CLS_LSN) -> CLS_LSN);
    LsnIncrement(plsn)
}
#[inline]
pub unsafe fn LsnInvalid(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("clfsw32.dll" "system" fn LsnInvalid(plsn : *const CLS_LSN) -> super::super::Foundation:: BOOLEAN);
    LsnInvalid(plsn)
}
#[inline]
pub unsafe fn LsnLess(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("clfsw32.dll" "system" fn LsnLess(plsn1 : *const CLS_LSN, plsn2 : *const CLS_LSN) -> super::super::Foundation:: BOOLEAN);
    LsnLess(plsn1, plsn2)
}
#[inline]
pub unsafe fn LsnNull(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("clfsw32.dll" "system" fn LsnNull(plsn : *const CLS_LSN) -> super::super::Foundation:: BOOLEAN);
    LsnNull(plsn)
}
#[inline]
pub unsafe fn LsnRecordSequence(plsn: *const CLS_LSN) -> u32 {
    windows_targets::link!("clfsw32.dll" "system" fn LsnRecordSequence(plsn : *const CLS_LSN) -> u32);
    LsnRecordSequence(plsn)
}
#[inline]
pub unsafe fn MoveFileA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn MoveFileA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    MoveFileA(lpexistingfilename.param().abi(), lpnewfilename.param().abi()).ok()
}
#[inline]
pub unsafe fn MoveFileExA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: MOVE_FILE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn MoveFileExA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, dwflags : MOVE_FILE_FLAGS) -> super::super::Foundation:: BOOL);
    MoveFileExA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), dwflags).ok()
}
#[inline]
pub unsafe fn MoveFileExW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: MOVE_FILE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn MoveFileExW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, dwflags : MOVE_FILE_FLAGS) -> super::super::Foundation:: BOOL);
    MoveFileExW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), dwflags).ok()
}
#[inline]
pub unsafe fn MoveFileFromAppW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn MoveFileFromAppW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    MoveFileFromAppW(lpexistingfilename.param().abi(), lpnewfilename.param().abi())
}
#[inline]
pub unsafe fn MoveFileTransactedA<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<*const core::ffi::c_void>, dwflags: MOVE_FILE_FLAGS, htransaction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn MoveFileTransactedA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, dwflags : MOVE_FILE_FLAGS, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    MoveFileTransactedA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), dwflags, htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn MoveFileTransactedW<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<*const core::ffi::c_void>, dwflags: MOVE_FILE_FLAGS, htransaction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn MoveFileTransactedW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, dwflags : MOVE_FILE_FLAGS, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    MoveFileTransactedW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), dwflags, htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn MoveFileW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn MoveFileW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    MoveFileW(lpexistingfilename.param().abi(), lpnewfilename.param().abi()).ok()
}
#[inline]
pub unsafe fn MoveFileWithProgressA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<*const core::ffi::c_void>, dwflags: MOVE_FILE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn MoveFileWithProgressA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, dwflags : MOVE_FILE_FLAGS) -> super::super::Foundation:: BOOL);
    MoveFileWithProgressA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), dwflags).ok()
}
#[inline]
pub unsafe fn MoveFileWithProgressW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<*const core::ffi::c_void>, dwflags: MOVE_FILE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn MoveFileWithProgressW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, dwflags : MOVE_FILE_FLAGS) -> super::super::Foundation:: BOOL);
    MoveFileWithProgressW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), dwflags).ok()
}
#[inline]
pub unsafe fn NetConnectionEnum<P0, P1>(servername: P0, qualifier: P1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetConnectionEnum(servername : windows_core::PCWSTR, qualifier : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetConnectionEnum(servername.param().abi(), qualifier.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetFileClose<P0>(servername: P0, fileid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetFileClose(servername : windows_core::PCWSTR, fileid : u32) -> u32);
    NetFileClose(servername.param().abi(), fileid)
}
#[inline]
pub unsafe fn NetFileEnum<P0, P1, P2>(servername: P0, basepath: P1, username: P2, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetFileEnum(servername : windows_core::PCWSTR, basepath : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut usize) -> u32);
    NetFileEnum(servername.param().abi(), basepath.param().abi(), username.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetFileGetInfo<P0>(servername: P0, fileid: u32, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetFileGetInfo(servername : windows_core::PCWSTR, fileid : u32, level : u32, bufptr : *mut *mut u8) -> u32);
    NetFileGetInfo(servername.param().abi(), fileid, level, bufptr)
}
#[inline]
pub unsafe fn NetServerAliasAdd<P0>(servername: P0, level: u32, buf: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerAliasAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8) -> u32);
    NetServerAliasAdd(servername.param().abi(), level, buf)
}
#[inline]
pub unsafe fn NetServerAliasDel<P0>(servername: P0, level: u32, buf: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerAliasDel(servername : windows_core::PCWSTR, level : u32, buf : *const u8) -> u32);
    NetServerAliasDel(servername.param().abi(), level, buf)
}
#[inline]
pub unsafe fn NetServerAliasEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerAliasEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    NetServerAliasEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resumehandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetSessionDel<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetSessionDel(servername : windows_core::PCWSTR, uncclientname : windows_core::PCWSTR, username : windows_core::PCWSTR) -> u32);
    NetSessionDel(servername.param().abi(), uncclientname.param().abi(), username.param().abi())
}
#[inline]
pub unsafe fn NetSessionEnum<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetSessionEnum(servername : windows_core::PCWSTR, uncclientname : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetSessionEnum(servername.param().abi(), uncclientname.param().abi(), username.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetSessionGetInfo<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetSessionGetInfo(servername : windows_core::PCWSTR, uncclientname : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetSessionGetInfo(servername.param().abi(), uncclientname.param().abi(), username.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetShareAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetShareAdd(servername.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetShareCheck<P0, P1>(servername: P0, device: P1, r#type: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareCheck(servername : windows_core::PCWSTR, device : windows_core::PCWSTR, r#type : *mut u32) -> u32);
    NetShareCheck(servername.param().abi(), device.param().abi(), r#type)
}
#[inline]
pub unsafe fn NetShareDel<P0, P1>(servername: P0, netname: P1, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareDel(servername : windows_core::PCWSTR, netname : windows_core::PCWSTR, reserved : u32) -> u32);
    NetShareDel(servername.param().abi(), netname.param().abi(), reserved)
}
#[inline]
pub unsafe fn NetShareDelEx<P0>(servername: P0, level: u32, buf: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareDelEx(servername : windows_core::PCWSTR, level : u32, buf : *const u8) -> u32);
    NetShareDelEx(servername.param().abi(), level, buf)
}
#[inline]
pub unsafe fn NetShareDelSticky<P0, P1>(servername: P0, netname: P1, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareDelSticky(servername : windows_core::PCWSTR, netname : windows_core::PCWSTR, reserved : u32) -> u32);
    NetShareDelSticky(servername.param().abi(), netname.param().abi(), reserved)
}
#[inline]
pub unsafe fn NetShareEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetShareEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetShareEnumSticky<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareEnumSticky(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetShareEnumSticky(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetShareGetInfo<P0, P1>(servername: P0, netname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareGetInfo(servername : windows_core::PCWSTR, netname : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetShareGetInfo(servername.param().abi(), netname.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetShareSetInfo<P0, P1>(servername: P0, netname: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetShareSetInfo(servername : windows_core::PCWSTR, netname : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetShareSetInfo(servername.param().abi(), netname.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetStatisticsGet(servername: *const i8, service: *const i8, level: u32, options: u32, buffer: *mut *mut u8) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetStatisticsGet(servername : *const i8, service : *const i8, level : u32, options : u32, buffer : *mut *mut u8) -> u32);
    NetStatisticsGet(servername, service, level, options, buffer)
}
#[inline]
pub unsafe fn OpenEncryptedFileRawA<P0>(lpfilename: P0, ulflags: u32, pvcontext: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenEncryptedFileRawA(lpfilename : windows_core::PCSTR, ulflags : u32, pvcontext : *mut *mut core::ffi::c_void) -> u32);
    OpenEncryptedFileRawA(lpfilename.param().abi(), ulflags, pvcontext)
}
#[inline]
pub unsafe fn OpenEncryptedFileRawW<P0>(lpfilename: P0, ulflags: u32, pvcontext: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenEncryptedFileRawW(lpfilename : windows_core::PCWSTR, ulflags : u32, pvcontext : *mut *mut core::ffi::c_void) -> u32);
    OpenEncryptedFileRawW(lpfilename.param().abi(), ulflags, pvcontext)
}
#[inline]
pub unsafe fn OpenEnlistment<P0>(dwdesiredaccess: u32, resourcemanagerhandle: P0, enlistmentid: *mut windows_core::GUID) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn OpenEnlistment(dwdesiredaccess : u32, resourcemanagerhandle : super::super::Foundation:: HANDLE, enlistmentid : *mut windows_core::GUID) -> super::super::Foundation:: HANDLE);
    let result__ = OpenEnlistment(dwdesiredaccess, resourcemanagerhandle.param().abi(), enlistmentid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenFile<P0>(lpfilename: P0, lpreopenbuff: *mut OFSTRUCT, ustyle: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenFile(lpfilename : windows_core::PCSTR, lpreopenbuff : *mut OFSTRUCT, ustyle : u32) -> i32);
    OpenFile(lpfilename.param().abi(), lpreopenbuff, ustyle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenFileById<P0>(hvolumehint: P0, lpfileid: *const FILE_ID_DESCRIPTOR, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenFileById(hvolumehint : super::super::Foundation:: HANDLE, lpfileid : *const FILE_ID_DESCRIPTOR, dwdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = OpenFileById(hvolumehint.param().abi(), lpfileid, dwdesiredaccess, dwsharemode, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), dwflagsandattributes);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenResourceManager<P0>(dwdesiredaccess: u32, tmhandle: P0, resourcemanagerid: *mut windows_core::GUID) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn OpenResourceManager(dwdesiredaccess : u32, tmhandle : super::super::Foundation:: HANDLE, resourcemanagerid : *mut windows_core::GUID) -> super::super::Foundation:: HANDLE);
    let result__ = OpenResourceManager(dwdesiredaccess, tmhandle.param().abi(), resourcemanagerid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenTransaction(dwdesiredaccess: u32, transactionid: *mut windows_core::GUID) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("ktmw32.dll" "system" fn OpenTransaction(dwdesiredaccess : u32, transactionid : *mut windows_core::GUID) -> super::super::Foundation:: HANDLE);
    let result__ = OpenTransaction(dwdesiredaccess, transactionid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenTransactionManager<P0>(logfilename: P0, desiredaccess: u32, openoptions: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ktmw32.dll" "system" fn OpenTransactionManager(logfilename : windows_core::PCWSTR, desiredaccess : u32, openoptions : u32) -> super::super::Foundation:: HANDLE);
    let result__ = OpenTransactionManager(logfilename.param().abi(), desiredaccess, openoptions);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenTransactionManagerById(transactionmanagerid: *const windows_core::GUID, desiredaccess: u32, openoptions: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("ktmw32.dll" "system" fn OpenTransactionManagerById(transactionmanagerid : *const windows_core::GUID, desiredaccess : u32, openoptions : u32) -> super::super::Foundation:: HANDLE);
    let result__ = OpenTransactionManagerById(transactionmanagerid, desiredaccess, openoptions);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn PopIoRingCompletion<P0>(ioring: P0, cqe: *mut IORING_CQE) -> windows_core::HRESULT
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn PopIoRingCompletion(ioring : HIORING, cqe : *mut IORING_CQE) -> windows_core::HRESULT);
    PopIoRingCompletion(ioring.param().abi(), cqe)
}
#[inline]
pub unsafe fn PrePrepareComplete<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn PrePrepareComplete(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    PrePrepareComplete(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn PrePrepareEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn PrePrepareEnlistment(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    PrePrepareEnlistment(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn PrepareComplete<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn PrepareComplete(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    PrepareComplete(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn PrepareEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn PrepareEnlistment(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    PrepareEnlistment(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn PrepareLogArchive<P0>(hlog: P0, pszbaselogfilename: &mut [u16], plsnlow: Option<*const CLS_LSN>, plsnhigh: Option<*const CLS_LSN>, pcactuallength: Option<*mut u32>, poffbaselogfiledata: *mut u64, pcbbaselogfilelength: *mut u64, plsnbase: *mut CLS_LSN, plsnlast: *mut CLS_LSN, plsncurrentarchivetail: *mut CLS_LSN, ppvarchivecontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn PrepareLogArchive(hlog : super::super::Foundation:: HANDLE, pszbaselogfilename : windows_core::PWSTR, clen : u32, plsnlow : *const CLS_LSN, plsnhigh : *const CLS_LSN, pcactuallength : *mut u32, poffbaselogfiledata : *mut u64, pcbbaselogfilelength : *mut u64, plsnbase : *mut CLS_LSN, plsnlast : *mut CLS_LSN, plsncurrentarchivetail : *mut CLS_LSN, ppvarchivecontext : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    PrepareLogArchive(hlog.param().abi(), core::mem::transmute(pszbaselogfilename.as_ptr()), pszbaselogfilename.len().try_into().unwrap(), core::mem::transmute(plsnlow.unwrap_or(std::ptr::null())), core::mem::transmute(plsnhigh.unwrap_or(std::ptr::null())), core::mem::transmute(pcactuallength.unwrap_or(std::ptr::null_mut())), poffbaselogfiledata, pcbbaselogfilelength, plsnbase, plsnlast, plsncurrentarchivetail, ppvarchivecontext).ok()
}
#[inline]
pub unsafe fn PrepareTape<P0, P1>(hdevice: P0, dwoperation: PREPARE_TAPE_OPERATION, bimmediate: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn PrepareTape(hdevice : super::super::Foundation:: HANDLE, dwoperation : PREPARE_TAPE_OPERATION, bimmediate : super::super::Foundation:: BOOL) -> u32);
    PrepareTape(hdevice.param().abi(), dwoperation, bimmediate.param().abi())
}
#[inline]
pub unsafe fn QueryDosDeviceA<P0>(lpdevicename: P0, lptargetpath: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryDosDeviceA(lpdevicename : windows_core::PCSTR, lptargetpath : windows_core::PSTR, ucchmax : u32) -> u32);
    QueryDosDeviceA(lpdevicename.param().abi(), core::mem::transmute(lptargetpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptargetpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn QueryDosDeviceW<P0>(lpdevicename: P0, lptargetpath: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryDosDeviceW(lpdevicename : windows_core::PCWSTR, lptargetpath : windows_core::PWSTR, ucchmax : u32) -> u32);
    QueryDosDeviceW(lpdevicename.param().abi(), core::mem::transmute(lptargetpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptargetpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn QueryIoRingCapabilities() -> windows_core::Result<IORING_CAPABILITIES> {
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn QueryIoRingCapabilities(capabilities : *mut IORING_CAPABILITIES) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    QueryIoRingCapabilities(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn QueryLogPolicy<P0>(hlog: P0, epolicytype: CLFS_MGMT_POLICY_TYPE, ppolicybuffer: *mut CLFS_MGMT_POLICY, pcbpolicybuffer: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn QueryLogPolicy(hlog : super::super::Foundation:: HANDLE, epolicytype : CLFS_MGMT_POLICY_TYPE, ppolicybuffer : *mut CLFS_MGMT_POLICY, pcbpolicybuffer : *mut u32) -> super::super::Foundation:: BOOL);
    QueryLogPolicy(hlog.param().abi(), epolicytype, ppolicybuffer, pcbpolicybuffer).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryRecoveryAgentsOnEncryptedFile<P0>(lpfilename: P0, precoveryagents: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryRecoveryAgentsOnEncryptedFile(lpfilename : windows_core::PCWSTR, precoveryagents : *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
    QueryRecoveryAgentsOnEncryptedFile(lpfilename.param().abi(), precoveryagents)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryUsersOnEncryptedFile<P0>(lpfilename: P0, pusers: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryUsersOnEncryptedFile(lpfilename : windows_core::PCWSTR, pusers : *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
    QueryUsersOnEncryptedFile(lpfilename.param().abi(), pusers)
}
#[inline]
pub unsafe fn ReOpenFile<P0>(horiginalfile: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReOpenFile(horiginalfile : super::super::Foundation:: HANDLE, dwdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = ReOpenFile(horiginalfile.param().abi(), dwdesiredaccess, dwsharemode, dwflagsandattributes);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadDirectoryChangesExW<P0, P1>(hdirectory: P0, lpbuffer: *mut core::ffi::c_void, nbufferlength: u32, bwatchsubtree: P1, dwnotifyfilter: FILE_NOTIFY_CHANGE, lpbytesreturned: Option<*mut u32>, lpoverlapped: Option<*mut super::super::System::IO::OVERLAPPED>, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE, readdirectorynotifyinformationclass: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadDirectoryChangesExW(hdirectory : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nbufferlength : u32, bwatchsubtree : super::super::Foundation:: BOOL, dwnotifyfilter : FILE_NOTIFY_CHANGE, lpbytesreturned : *mut u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED, lpcompletionroutine : super::super::System::IO:: LPOVERLAPPED_COMPLETION_ROUTINE, readdirectorynotifyinformationclass : READ_DIRECTORY_NOTIFY_INFORMATION_CLASS) -> super::super::Foundation:: BOOL);
    ReadDirectoryChangesExW(hdirectory.param().abi(), lpbuffer, nbufferlength, bwatchsubtree.param().abi(), dwnotifyfilter, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut())), lpcompletionroutine, readdirectorynotifyinformationclass).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadDirectoryChangesW<P0, P1>(hdirectory: P0, lpbuffer: *mut core::ffi::c_void, nbufferlength: u32, bwatchsubtree: P1, dwnotifyfilter: FILE_NOTIFY_CHANGE, lpbytesreturned: Option<*mut u32>, lpoverlapped: Option<*mut super::super::System::IO::OVERLAPPED>, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadDirectoryChangesW(hdirectory : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nbufferlength : u32, bwatchsubtree : super::super::Foundation:: BOOL, dwnotifyfilter : FILE_NOTIFY_CHANGE, lpbytesreturned : *mut u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED, lpcompletionroutine : super::super::System::IO:: LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation:: BOOL);
    ReadDirectoryChangesW(hdirectory.param().abi(), lpbuffer, nbufferlength, bwatchsubtree.param().abi(), dwnotifyfilter, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut())), lpcompletionroutine).ok()
}
#[inline]
pub unsafe fn ReadEncryptedFileRaw(pfexportcallback: PFE_EXPORT_FUNC, pvcallbackcontext: Option<*const core::ffi::c_void>, pvcontext: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn ReadEncryptedFileRaw(pfexportcallback : PFE_EXPORT_FUNC, pvcallbackcontext : *const core::ffi::c_void, pvcontext : *const core::ffi::c_void) -> u32);
    ReadEncryptedFileRaw(pfexportcallback, core::mem::transmute(pvcallbackcontext.unwrap_or(std::ptr::null())), pvcontext)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadFile<P0>(hfile: P0, lpbuffer: Option<&mut [u8]>, lpnumberofbytesread: Option<*mut u32>, lpoverlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadFile(hfile : super::super::Foundation:: HANDLE, lpbuffer : *mut u8, nnumberofbytestoread : u32, lpnumberofbytesread : *mut u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReadFile(hfile.param().abi(), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpnumberofbytesread.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadFileEx<P0>(hfile: P0, lpbuffer: Option<&mut [u8]>, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadFileEx(hfile : super::super::Foundation:: HANDLE, lpbuffer : *mut u8, nnumberofbytestoread : u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED, lpcompletionroutine : super::super::System::IO:: LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation:: BOOL);
    ReadFileEx(hfile.param().abi(), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpoverlapped, lpcompletionroutine).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadFileScatter<P0>(hfile: P0, asegmentarray: *const FILE_SEGMENT_ELEMENT, nnumberofbytestoread: u32, lpreserved: Option<*const u32>, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadFileScatter(hfile : super::super::Foundation:: HANDLE, asegmentarray : *const FILE_SEGMENT_ELEMENT, nnumberofbytestoread : u32, lpreserved : *const u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReadFileScatter(hfile.param().abi(), asegmentarray, nnumberofbytestoread, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), lpoverlapped).ok()
}
#[inline]
pub unsafe fn ReadLogArchiveMetadata(pvarchivecontext: *mut core::ffi::c_void, cboffset: u32, cbbytestoread: u32, pbreadbuffer: *mut u8, pcbbytesread: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn ReadLogArchiveMetadata(pvarchivecontext : *mut core::ffi::c_void, cboffset : u32, cbbytestoread : u32, pbreadbuffer : *mut u8, pcbbytesread : *mut u32) -> super::super::Foundation:: BOOL);
    ReadLogArchiveMetadata(pvarchivecontext, cboffset, cbbytestoread, pbreadbuffer, pcbbytesread).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadLogNotification<P0>(hlog: P0, pnotification: *mut CLFS_MGMT_NOTIFICATION, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn ReadLogNotification(hlog : super::super::Foundation:: HANDLE, pnotification : *mut CLFS_MGMT_NOTIFICATION, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReadLogNotification(hlog.param().abi(), pnotification, lpoverlapped).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadLogRecord(pvmarshal: *mut core::ffi::c_void, plsnfirst: *mut CLS_LSN, econtextmode: CLFS_CONTEXT_MODE, ppvreadbuffer: *mut *mut core::ffi::c_void, pcbreadbuffer: *mut u32, perecordtype: *mut u8, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, ppvreadcontext: *mut *mut core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn ReadLogRecord(pvmarshal : *mut core::ffi::c_void, plsnfirst : *mut CLS_LSN, econtextmode : CLFS_CONTEXT_MODE, ppvreadbuffer : *mut *mut core::ffi::c_void, pcbreadbuffer : *mut u32, perecordtype : *mut u8, plsnundonext : *mut CLS_LSN, plsnprevious : *mut CLS_LSN, ppvreadcontext : *mut *mut core::ffi::c_void, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReadLogRecord(pvmarshal, plsnfirst, econtextmode, ppvreadbuffer, pcbreadbuffer, perecordtype, plsnundonext, plsnprevious, ppvreadcontext, poverlapped).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadLogRestartArea(pvmarshal: *mut core::ffi::c_void, ppvrestartbuffer: *mut *mut core::ffi::c_void, pcbrestartbuffer: *mut u32, plsn: *mut CLS_LSN, ppvcontext: *mut *mut core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn ReadLogRestartArea(pvmarshal : *mut core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : *mut u32, plsn : *mut CLS_LSN, ppvcontext : *mut *mut core::ffi::c_void, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReadLogRestartArea(pvmarshal, ppvrestartbuffer, pcbrestartbuffer, plsn, ppvcontext, poverlapped).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadNextLogRecord(pvreadcontext: *mut core::ffi::c_void, ppvbuffer: *mut *mut core::ffi::c_void, pcbbuffer: *mut u32, perecordtype: *mut u8, plsnuser: *mut CLS_LSN, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, plsnrecord: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn ReadNextLogRecord(pvreadcontext : *mut core::ffi::c_void, ppvbuffer : *mut *mut core::ffi::c_void, pcbbuffer : *mut u32, perecordtype : *mut u8, plsnuser : *mut CLS_LSN, plsnundonext : *mut CLS_LSN, plsnprevious : *mut CLS_LSN, plsnrecord : *mut CLS_LSN, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReadNextLogRecord(pvreadcontext, ppvbuffer, pcbbuffer, perecordtype, plsnuser, plsnundonext, plsnprevious, plsnrecord, poverlapped).ok()
}
#[inline]
pub unsafe fn ReadOnlyEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn ReadOnlyEnlistment(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    ReadOnlyEnlistment(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReadPreviousLogRestartArea(pvreadcontext: *mut core::ffi::c_void, ppvrestartbuffer: *mut *mut core::ffi::c_void, pcbrestartbuffer: *mut u32, plsnrestart: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn ReadPreviousLogRestartArea(pvreadcontext : *mut core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : *mut u32, plsnrestart : *mut CLS_LSN, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReadPreviousLogRestartArea(pvreadcontext, ppvrestartbuffer, pcbrestartbuffer, plsnrestart, poverlapped).ok()
}
#[inline]
pub unsafe fn RecoverEnlistment<P0>(enlistmenthandle: P0, enlistmentkey: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RecoverEnlistment(enlistmenthandle : super::super::Foundation:: HANDLE, enlistmentkey : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    RecoverEnlistment(enlistmenthandle.param().abi(), enlistmentkey).ok()
}
#[inline]
pub unsafe fn RecoverResourceManager<P0>(resourcemanagerhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RecoverResourceManager(resourcemanagerhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    RecoverResourceManager(resourcemanagerhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn RecoverTransactionManager<P0>(transactionmanagerhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RecoverTransactionManager(transactionmanagerhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    RecoverTransactionManager(transactionmanagerhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn RegisterForLogWriteNotification<P0, P1>(hlog: P0, cbthreshold: u32, fenable: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("clfsw32.dll" "system" fn RegisterForLogWriteNotification(hlog : super::super::Foundation:: HANDLE, cbthreshold : u32, fenable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    RegisterForLogWriteNotification(hlog.param().abi(), cbthreshold, fenable.param().abi()).ok()
}
#[inline]
pub unsafe fn RegisterManageableLogClient<P0>(hlog: P0, pcallbacks: *mut LOG_MANAGEMENT_CALLBACKS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn RegisterManageableLogClient(hlog : super::super::Foundation:: HANDLE, pcallbacks : *mut LOG_MANAGEMENT_CALLBACKS) -> super::super::Foundation:: BOOL);
    RegisterManageableLogClient(hlog.param().abi(), pcallbacks).ok()
}
#[inline]
pub unsafe fn RemoveDirectoryA<P0>(lppathname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn RemoveDirectoryA(lppathname : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    RemoveDirectoryA(lppathname.param().abi()).ok()
}
#[inline]
pub unsafe fn RemoveDirectoryFromAppW<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn RemoveDirectoryFromAppW(lppathname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    RemoveDirectoryFromAppW(lppathname.param().abi())
}
#[inline]
pub unsafe fn RemoveDirectoryTransactedA<P0, P1>(lppathname: P0, htransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn RemoveDirectoryTransactedA(lppathname : windows_core::PCSTR, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    RemoveDirectoryTransactedA(lppathname.param().abi(), htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn RemoveDirectoryTransactedW<P0, P1>(lppathname: P0, htransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn RemoveDirectoryTransactedW(lppathname : windows_core::PCWSTR, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    RemoveDirectoryTransactedW(lppathname.param().abi(), htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn RemoveDirectoryW<P0>(lppathname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn RemoveDirectoryW(lppathname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    RemoveDirectoryW(lppathname.param().abi()).ok()
}
#[inline]
pub unsafe fn RemoveLogContainer<P0, P1, P2>(hlog: P0, pwszcontainerpath: P1, fforce: P2, preserved: Option<*mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("clfsw32.dll" "system" fn RemoveLogContainer(hlog : super::super::Foundation:: HANDLE, pwszcontainerpath : windows_core::PCWSTR, fforce : super::super::Foundation:: BOOL, preserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    RemoveLogContainer(hlog.param().abi(), pwszcontainerpath.param().abi(), fforce.param().abi(), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RemoveLogContainerSet<P0, P1>(hlog: P0, rgwszcontainerpath: &[windows_core::PCWSTR], fforce: P1, preserved: Option<*mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("clfsw32.dll" "system" fn RemoveLogContainerSet(hlog : super::super::Foundation:: HANDLE, ccontainer : u16, rgwszcontainerpath : *const windows_core::PCWSTR, fforce : super::super::Foundation:: BOOL, preserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    RemoveLogContainerSet(hlog.param().abi(), rgwszcontainerpath.len().try_into().unwrap(), core::mem::transmute(rgwszcontainerpath.as_ptr()), fforce.param().abi(), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RemoveLogPolicy<P0>(hlog: P0, epolicytype: CLFS_MGMT_POLICY_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn RemoveLogPolicy(hlog : super::super::Foundation:: HANDLE, epolicytype : CLFS_MGMT_POLICY_TYPE) -> super::super::Foundation:: BOOL);
    RemoveLogPolicy(hlog.param().abi(), epolicytype).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RemoveUsersFromEncryptedFile<P0>(lpfilename: P0, phashes: *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RemoveUsersFromEncryptedFile(lpfilename : windows_core::PCWSTR, phashes : *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
    RemoveUsersFromEncryptedFile(lpfilename.param().abi(), phashes)
}
#[inline]
pub unsafe fn RenameTransactionManager<P0>(logfilename: P0, existingtransactionmanagerguid: *mut windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RenameTransactionManager(logfilename : windows_core::PCWSTR, existingtransactionmanagerguid : *mut windows_core::GUID) -> super::super::Foundation:: BOOL);
    RenameTransactionManager(logfilename.param().abi(), existingtransactionmanagerguid).ok()
}
#[inline]
pub unsafe fn ReplaceFileA<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: REPLACE_FILE_FLAGS, lpexclude: Option<*const core::ffi::c_void>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReplaceFileA(lpreplacedfilename : windows_core::PCSTR, lpreplacementfilename : windows_core::PCSTR, lpbackupfilename : windows_core::PCSTR, dwreplaceflags : REPLACE_FILE_FLAGS, lpexclude : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ReplaceFileA(lpreplacedfilename.param().abi(), lpreplacementfilename.param().abi(), lpbackupfilename.param().abi(), dwreplaceflags, core::mem::transmute(lpexclude.unwrap_or(std::ptr::null())), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ReplaceFileFromAppW<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: u32, lpexclude: Option<*const core::ffi::c_void>, lpreserved: Option<*const core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn ReplaceFileFromAppW(lpreplacedfilename : windows_core::PCWSTR, lpreplacementfilename : windows_core::PCWSTR, lpbackupfilename : windows_core::PCWSTR, dwreplaceflags : u32, lpexclude : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ReplaceFileFromAppW(lpreplacedfilename.param().abi(), lpreplacementfilename.param().abi(), lpbackupfilename.param().abi(), dwreplaceflags, core::mem::transmute(lpexclude.unwrap_or(std::ptr::null())), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn ReplaceFileW<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: REPLACE_FILE_FLAGS, lpexclude: Option<*const core::ffi::c_void>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReplaceFileW(lpreplacedfilename : windows_core::PCWSTR, lpreplacementfilename : windows_core::PCWSTR, lpbackupfilename : windows_core::PCWSTR, dwreplaceflags : REPLACE_FILE_FLAGS, lpexclude : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ReplaceFileW(lpreplacedfilename.param().abi(), lpreplacementfilename.param().abi(), lpbackupfilename.param().abi(), dwreplaceflags, core::mem::transmute(lpexclude.unwrap_or(std::ptr::null())), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReserveAndAppendLog(pvmarshal: *mut core::ffi::c_void, rgwriteentries: *mut CLS_WRITE_ENTRY, cwriteentries: u32, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, creserverecords: u32, rgcbreservation: *mut i64, fflags: CLFS_FLAG, plsn: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn ReserveAndAppendLog(pvmarshal : *mut core::ffi::c_void, rgwriteentries : *mut CLS_WRITE_ENTRY, cwriteentries : u32, plsnundonext : *mut CLS_LSN, plsnprevious : *mut CLS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : CLFS_FLAG, plsn : *mut CLS_LSN, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReserveAndAppendLog(pvmarshal, rgwriteentries, cwriteentries, plsnundonext, plsnprevious, creserverecords, rgcbreservation, fflags, plsn, poverlapped).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ReserveAndAppendLogAligned(pvmarshal: *mut core::ffi::c_void, rgwriteentries: *mut CLS_WRITE_ENTRY, cwriteentries: u32, cbentryalignment: u32, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, creserverecords: u32, rgcbreservation: *mut i64, fflags: CLFS_FLAG, plsn: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn ReserveAndAppendLogAligned(pvmarshal : *mut core::ffi::c_void, rgwriteentries : *mut CLS_WRITE_ENTRY, cwriteentries : u32, cbentryalignment : u32, plsnundonext : *mut CLS_LSN, plsnprevious : *mut CLS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : CLFS_FLAG, plsn : *mut CLS_LSN, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ReserveAndAppendLogAligned(pvmarshal, rgwriteentries, cwriteentries, cbentryalignment, plsnundonext, plsnprevious, creserverecords, rgcbreservation, fflags, plsn, poverlapped).ok()
}
#[inline]
pub unsafe fn RollbackComplete<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RollbackComplete(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    RollbackComplete(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn RollbackEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RollbackEnlistment(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    RollbackEnlistment(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn RollbackTransaction<P0>(transactionhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RollbackTransaction(transactionhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    RollbackTransaction(transactionhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn RollbackTransactionAsync<P0>(transactionhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RollbackTransactionAsync(transactionhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    RollbackTransactionAsync(transactionhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn RollforwardTransactionManager<P0>(transactionmanagerhandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn RollforwardTransactionManager(transactionmanagerhandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    RollforwardTransactionManager(transactionmanagerhandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn ScanLogContainers(pcxscan: *mut CLS_SCAN_CONTEXT, escanmode: u8, preserved: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn ScanLogContainers(pcxscan : *mut CLS_SCAN_CONTEXT, escanmode : u8, preserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ScanLogContainers(pcxscan, escanmode, preserved).ok()
}
#[inline]
pub unsafe fn SearchPathA<P0, P1, P2>(lppath: P0, lpfilename: P1, lpextension: P2, lpbuffer: Option<&mut [u8]>, lpfilepart: Option<*mut windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SearchPathA(lppath : windows_core::PCSTR, lpfilename : windows_core::PCSTR, lpextension : windows_core::PCSTR, nbufferlength : u32, lpbuffer : windows_core::PSTR, lpfilepart : *mut windows_core::PSTR) -> u32);
    SearchPathA(lppath.param().abi(), lpfilename.param().abi(), lpextension.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(lpfilepart.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SearchPathW<P0, P1, P2>(lppath: P0, lpfilename: P1, lpextension: P2, lpbuffer: Option<&mut [u16]>, lpfilepart: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SearchPathW(lppath : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR, lpextension : windows_core::PCWSTR, nbufferlength : u32, lpbuffer : windows_core::PWSTR, lpfilepart : *mut windows_core::PWSTR) -> u32);
    SearchPathW(lppath.param().abi(), lpfilename.param().abi(), lpextension.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(lpfilepart.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetEncryptedFileMetadata<P0>(lpfilename: P0, pboldmetadata: Option<*const u8>, pbnewmetadata: *const u8, pownerhash: *const ENCRYPTION_CERTIFICATE_HASH, dwoperation: u32, pcertificatesadded: Option<*const ENCRYPTION_CERTIFICATE_HASH_LIST>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetEncryptedFileMetadata(lpfilename : windows_core::PCWSTR, pboldmetadata : *const u8, pbnewmetadata : *const u8, pownerhash : *const ENCRYPTION_CERTIFICATE_HASH, dwoperation : u32, pcertificatesadded : *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
    SetEncryptedFileMetadata(lpfilename.param().abi(), core::mem::transmute(pboldmetadata.unwrap_or(std::ptr::null())), pbnewmetadata, pownerhash, dwoperation, core::mem::transmute(pcertificatesadded.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn SetEndOfFile<P0>(hfile: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetEndOfFile(hfile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetEndOfFile(hfile.param().abi()).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn SetEndOfLog<P0>(hlog: P0, plsnend: *mut CLS_LSN, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn SetEndOfLog(hlog : super::super::Foundation:: HANDLE, plsnend : *mut CLS_LSN, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    SetEndOfLog(hlog.param().abi(), plsnend, lpoverlapped).ok()
}
#[inline]
pub unsafe fn SetEnlistmentRecoveryInformation<P0>(enlistmenthandle: P0, buffersize: u32, buffer: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn SetEnlistmentRecoveryInformation(enlistmenthandle : super::super::Foundation:: HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SetEnlistmentRecoveryInformation(enlistmenthandle.param().abi(), buffersize, buffer).ok()
}
#[inline]
pub unsafe fn SetFileApisToANSI() {
    windows_targets::link!("kernel32.dll" "system" fn SetFileApisToANSI());
    SetFileApisToANSI()
}
#[inline]
pub unsafe fn SetFileApisToOEM() {
    windows_targets::link!("kernel32.dll" "system" fn SetFileApisToOEM());
    SetFileApisToOEM()
}
#[inline]
pub unsafe fn SetFileAttributesA<P0>(lpfilename: P0, dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileAttributesA(lpfilename : windows_core::PCSTR, dwfileattributes : FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    SetFileAttributesA(lpfilename.param().abi(), dwfileattributes).ok()
}
#[inline]
pub unsafe fn SetFileAttributesFromAppW<P0>(lpfilename: P0, dwfileattributes: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn SetFileAttributesFromAppW(lpfilename : windows_core::PCWSTR, dwfileattributes : u32) -> super::super::Foundation:: BOOL);
    SetFileAttributesFromAppW(lpfilename.param().abi(), dwfileattributes)
}
#[inline]
pub unsafe fn SetFileAttributesTransactedA<P0, P1>(lpfilename: P0, dwfileattributes: u32, htransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileAttributesTransactedA(lpfilename : windows_core::PCSTR, dwfileattributes : u32, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetFileAttributesTransactedA(lpfilename.param().abi(), dwfileattributes, htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn SetFileAttributesTransactedW<P0, P1>(lpfilename: P0, dwfileattributes: u32, htransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileAttributesTransactedW(lpfilename : windows_core::PCWSTR, dwfileattributes : u32, htransaction : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetFileAttributesTransactedW(lpfilename.param().abi(), dwfileattributes, htransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn SetFileAttributesW<P0>(lpfilename: P0, dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileAttributesW(lpfilename : windows_core::PCWSTR, dwfileattributes : FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    SetFileAttributesW(lpfilename.param().abi(), dwfileattributes).ok()
}
#[inline]
pub unsafe fn SetFileBandwidthReservation<P0, P1>(hfile: P0, nperiodmilliseconds: u32, nbytesperperiod: u32, bdiscardable: P1, lptransfersize: *mut u32, lpnumoutstandingrequests: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileBandwidthReservation(hfile : super::super::Foundation:: HANDLE, nperiodmilliseconds : u32, nbytesperperiod : u32, bdiscardable : super::super::Foundation:: BOOL, lptransfersize : *mut u32, lpnumoutstandingrequests : *mut u32) -> super::super::Foundation:: BOOL);
    SetFileBandwidthReservation(hfile.param().abi(), nperiodmilliseconds, nbytesperperiod, bdiscardable.param().abi(), lptransfersize, lpnumoutstandingrequests).ok()
}
#[inline]
pub unsafe fn SetFileCompletionNotificationModes<P0>(filehandle: P0, flags: u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileCompletionNotificationModes(filehandle : super::super::Foundation:: HANDLE, flags : u8) -> super::super::Foundation:: BOOL);
    SetFileCompletionNotificationModes(filehandle.param().abi(), flags).ok()
}
#[inline]
pub unsafe fn SetFileInformationByHandle<P0>(hfile: P0, fileinformationclass: FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *const core::ffi::c_void, dwbuffersize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileInformationByHandle(hfile : super::super::Foundation:: HANDLE, fileinformationclass : FILE_INFO_BY_HANDLE_CLASS, lpfileinformation : *const core::ffi::c_void, dwbuffersize : u32) -> super::super::Foundation:: BOOL);
    SetFileInformationByHandle(hfile.param().abi(), fileinformationclass, lpfileinformation, dwbuffersize).ok()
}
#[inline]
pub unsafe fn SetFileIoOverlappedRange<P0>(filehandle: P0, overlappedrangestart: *const u8, length: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileIoOverlappedRange(filehandle : super::super::Foundation:: HANDLE, overlappedrangestart : *const u8, length : u32) -> super::super::Foundation:: BOOL);
    SetFileIoOverlappedRange(filehandle.param().abi(), overlappedrangestart, length).ok()
}
#[inline]
pub unsafe fn SetFilePointer<P0>(hfile: P0, ldistancetomove: i32, lpdistancetomovehigh: Option<*mut i32>, dwmovemethod: SET_FILE_POINTER_MOVE_METHOD) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFilePointer(hfile : super::super::Foundation:: HANDLE, ldistancetomove : i32, lpdistancetomovehigh : *mut i32, dwmovemethod : SET_FILE_POINTER_MOVE_METHOD) -> u32);
    SetFilePointer(hfile.param().abi(), ldistancetomove, core::mem::transmute(lpdistancetomovehigh.unwrap_or(std::ptr::null_mut())), dwmovemethod)
}
#[inline]
pub unsafe fn SetFilePointerEx<P0>(hfile: P0, lidistancetomove: i64, lpnewfilepointer: Option<*mut i64>, dwmovemethod: SET_FILE_POINTER_MOVE_METHOD) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFilePointerEx(hfile : super::super::Foundation:: HANDLE, lidistancetomove : i64, lpnewfilepointer : *mut i64, dwmovemethod : SET_FILE_POINTER_MOVE_METHOD) -> super::super::Foundation:: BOOL);
    SetFilePointerEx(hfile.param().abi(), lidistancetomove, core::mem::transmute(lpnewfilepointer.unwrap_or(std::ptr::null_mut())), dwmovemethod).ok()
}
#[inline]
pub unsafe fn SetFileShortNameA<P0, P1>(hfile: P0, lpshortname: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileShortNameA(hfile : super::super::Foundation:: HANDLE, lpshortname : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetFileShortNameA(hfile.param().abi(), lpshortname.param().abi()).ok()
}
#[inline]
pub unsafe fn SetFileShortNameW<P0, P1>(hfile: P0, lpshortname: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileShortNameW(hfile : super::super::Foundation:: HANDLE, lpshortname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetFileShortNameW(hfile.param().abi(), lpshortname.param().abi()).ok()
}
#[inline]
pub unsafe fn SetFileTime<P0>(hfile: P0, lpcreationtime: Option<*const super::super::Foundation::FILETIME>, lplastaccesstime: Option<*const super::super::Foundation::FILETIME>, lplastwritetime: Option<*const super::super::Foundation::FILETIME>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileTime(hfile : super::super::Foundation:: HANDLE, lpcreationtime : *const super::super::Foundation:: FILETIME, lplastaccesstime : *const super::super::Foundation:: FILETIME, lplastwritetime : *const super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    SetFileTime(hfile.param().abi(), core::mem::transmute(lpcreationtime.unwrap_or(std::ptr::null())), core::mem::transmute(lplastaccesstime.unwrap_or(std::ptr::null())), core::mem::transmute(lplastwritetime.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn SetFileValidData<P0>(hfile: P0, validdatalength: i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetFileValidData(hfile : super::super::Foundation:: HANDLE, validdatalength : i64) -> super::super::Foundation:: BOOL);
    SetFileValidData(hfile.param().abi(), validdatalength).ok()
}
#[inline]
pub unsafe fn SetIoRingCompletionEvent<P0, P1>(ioring: P0, hevent: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn SetIoRingCompletionEvent(ioring : HIORING, hevent : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    SetIoRingCompletionEvent(ioring.param().abi(), hevent.param().abi()).ok()
}
#[inline]
pub unsafe fn SetLogArchiveMode<P0>(hlog: P0, emode: CLFS_LOG_ARCHIVE_MODE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn SetLogArchiveMode(hlog : super::super::Foundation:: HANDLE, emode : CLFS_LOG_ARCHIVE_MODE) -> super::super::Foundation:: BOOL);
    SetLogArchiveMode(hlog.param().abi(), emode).ok()
}
#[inline]
pub unsafe fn SetLogArchiveTail<P0>(hlog: P0, plsnarchivetail: *mut CLS_LSN, preserved: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn SetLogArchiveTail(hlog : super::super::Foundation:: HANDLE, plsnarchivetail : *mut CLS_LSN, preserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SetLogArchiveTail(hlog.param().abi(), plsnarchivetail, preserved).ok()
}
#[inline]
pub unsafe fn SetLogFileSizeWithPolicy<P0>(hlog: P0, pdesiredsize: *const u64, presultingsize: *mut u64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clfsw32.dll" "system" fn SetLogFileSizeWithPolicy(hlog : super::super::Foundation:: HANDLE, pdesiredsize : *const u64, presultingsize : *mut u64) -> super::super::Foundation:: BOOL);
    SetLogFileSizeWithPolicy(hlog.param().abi(), pdesiredsize, presultingsize).ok()
}
#[inline]
pub unsafe fn SetResourceManagerCompletionPort<P0, P1>(resourcemanagerhandle: P0, iocompletionporthandle: P1, completionkey: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn SetResourceManagerCompletionPort(resourcemanagerhandle : super::super::Foundation:: HANDLE, iocompletionporthandle : super::super::Foundation:: HANDLE, completionkey : usize) -> super::super::Foundation:: BOOL);
    SetResourceManagerCompletionPort(resourcemanagerhandle.param().abi(), iocompletionporthandle.param().abi(), completionkey).ok()
}
#[inline]
pub unsafe fn SetSearchPathMode(flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetSearchPathMode(flags : u32) -> super::super::Foundation:: BOOL);
    SetSearchPathMode(flags).ok()
}
#[inline]
pub unsafe fn SetTapeParameters<P0>(hdevice: P0, dwoperation: TAPE_INFORMATION_TYPE, lptapeinformation: *const core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetTapeParameters(hdevice : super::super::Foundation:: HANDLE, dwoperation : TAPE_INFORMATION_TYPE, lptapeinformation : *const core::ffi::c_void) -> u32);
    SetTapeParameters(hdevice.param().abi(), dwoperation, lptapeinformation)
}
#[inline]
pub unsafe fn SetTapePosition<P0, P1>(hdevice: P0, dwpositionmethod: TAPE_POSITION_METHOD, dwpartition: u32, dwoffsetlow: u32, dwoffsethigh: u32, bimmediate: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetTapePosition(hdevice : super::super::Foundation:: HANDLE, dwpositionmethod : TAPE_POSITION_METHOD, dwpartition : u32, dwoffsetlow : u32, dwoffsethigh : u32, bimmediate : super::super::Foundation:: BOOL) -> u32);
    SetTapePosition(hdevice.param().abi(), dwpositionmethod, dwpartition, dwoffsetlow, dwoffsethigh, bimmediate.param().abi())
}
#[inline]
pub unsafe fn SetTransactionInformation<P0, P1>(transactionhandle: P0, isolationlevel: u32, isolationflags: u32, timeout: u32, description: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ktmw32.dll" "system" fn SetTransactionInformation(transactionhandle : super::super::Foundation:: HANDLE, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetTransactionInformation(transactionhandle.param().abi(), isolationlevel, isolationflags, timeout, description.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetUserFileEncryptionKey(pencryptioncertificate: Option<*const ENCRYPTION_CERTIFICATE>) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn SetUserFileEncryptionKey(pencryptioncertificate : *const ENCRYPTION_CERTIFICATE) -> u32);
    SetUserFileEncryptionKey(core::mem::transmute(pencryptioncertificate.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetUserFileEncryptionKeyEx(pencryptioncertificate: Option<*const ENCRYPTION_CERTIFICATE>, dwcapabilities: u32, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn SetUserFileEncryptionKeyEx(pencryptioncertificate : *const ENCRYPTION_CERTIFICATE, dwcapabilities : u32, dwflags : u32, pvreserved : *const core::ffi::c_void) -> u32);
    SetUserFileEncryptionKeyEx(core::mem::transmute(pencryptioncertificate.unwrap_or(std::ptr::null())), dwcapabilities, dwflags, core::mem::transmute(pvreserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn SetVolumeLabelA<P0, P1>(lprootpathname: P0, lpvolumename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetVolumeLabelA(lprootpathname : windows_core::PCSTR, lpvolumename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetVolumeLabelA(lprootpathname.param().abi(), lpvolumename.param().abi()).ok()
}
#[inline]
pub unsafe fn SetVolumeLabelW<P0, P1>(lprootpathname: P0, lpvolumename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetVolumeLabelW(lprootpathname : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetVolumeLabelW(lprootpathname.param().abi(), lpvolumename.param().abi()).ok()
}
#[inline]
pub unsafe fn SetVolumeMountPointA<P0, P1>(lpszvolumemountpoint: P0, lpszvolumename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetVolumeMountPointA(lpszvolumemountpoint : windows_core::PCSTR, lpszvolumename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetVolumeMountPointA(lpszvolumemountpoint.param().abi(), lpszvolumename.param().abi()).ok()
}
#[inline]
pub unsafe fn SetVolumeMountPointW<P0, P1>(lpszvolumemountpoint: P0, lpszvolumename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetVolumeMountPointW(lpszvolumemountpoint : windows_core::PCWSTR, lpszvolumename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetVolumeMountPointW(lpszvolumemountpoint.param().abi(), lpszvolumename.param().abi()).ok()
}
#[inline]
pub unsafe fn SinglePhaseReject<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ktmw32.dll" "system" fn SinglePhaseReject(enlistmenthandle : super::super::Foundation:: HANDLE, tmvirtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    SinglePhaseReject(enlistmenthandle.param().abi(), tmvirtualclock).ok()
}
#[inline]
pub unsafe fn SubmitIoRing<P0>(ioring: P0, waitoperations: u32, milliseconds: u32, submittedentries: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<HIORING>,
{
    windows_targets::link!("api-ms-win-core-ioring-l1-1-0.dll" "system" fn SubmitIoRing(ioring : HIORING, waitoperations : u32, milliseconds : u32, submittedentries : *mut u32) -> windows_core::HRESULT);
    SubmitIoRing(ioring.param().abi(), waitoperations, milliseconds, core::mem::transmute(submittedentries.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn TerminateLogArchive(pvarchivecontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn TerminateLogArchive(pvarchivecontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    TerminateLogArchive(pvarchivecontext).ok()
}
#[inline]
pub unsafe fn TerminateReadLog(pvcursorcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn TerminateReadLog(pvcursorcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    TerminateReadLog(pvcursorcontext).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn TruncateLog(pvmarshal: *const core::ffi::c_void, plsnend: *const CLS_LSN, lpoverlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn TruncateLog(pvmarshal : *const core::ffi::c_void, plsnend : *const CLS_LSN, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    TruncateLog(pvmarshal, plsnend, core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn TxfGetThreadMiniVersionForCreate() -> u16 {
    windows_targets::link!("txfw32.dll" "system" fn TxfGetThreadMiniVersionForCreate(miniversion : *mut u16));
    let mut result__ = core::mem::zeroed();
    TxfGetThreadMiniVersionForCreate(&mut result__);
    result__
}
#[inline]
pub unsafe fn TxfLogCreateFileReadContext<P0>(logpath: P0, beginninglsn: CLS_LSN, endinglsn: CLS_LSN, txffileid: *const TXF_ID, txflogcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("txfw32.dll" "system" fn TxfLogCreateFileReadContext(logpath : windows_core::PCWSTR, beginninglsn : CLS_LSN, endinglsn : CLS_LSN, txffileid : *const TXF_ID, txflogcontext : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    TxfLogCreateFileReadContext(logpath.param().abi(), core::mem::transmute(beginninglsn), core::mem::transmute(endinglsn), txffileid, txflogcontext).ok()
}
#[inline]
pub unsafe fn TxfLogCreateRangeReadContext<P0>(logpath: P0, beginninglsn: CLS_LSN, endinglsn: CLS_LSN, beginningvirtualclock: *const i64, endingvirtualclock: *const i64, recordtypemask: u32, txflogcontext: *mut *mut core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("txfw32.dll" "system" fn TxfLogCreateRangeReadContext(logpath : windows_core::PCWSTR, beginninglsn : CLS_LSN, endinglsn : CLS_LSN, beginningvirtualclock : *const i64, endingvirtualclock : *const i64, recordtypemask : u32, txflogcontext : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    TxfLogCreateRangeReadContext(logpath.param().abi(), core::mem::transmute(beginninglsn), core::mem::transmute(endinglsn), beginningvirtualclock, endingvirtualclock, recordtypemask, txflogcontext)
}
#[inline]
pub unsafe fn TxfLogDestroyReadContext(txflogcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("txfw32.dll" "system" fn TxfLogDestroyReadContext(txflogcontext : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    TxfLogDestroyReadContext(txflogcontext).ok()
}
#[inline]
pub unsafe fn TxfLogReadRecords(txflogcontext: *const core::ffi::c_void, bufferlength: u32, buffer: *mut core::ffi::c_void, bytesused: *mut u32, recordcount: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("txfw32.dll" "system" fn TxfLogReadRecords(txflogcontext : *const core::ffi::c_void, bufferlength : u32, buffer : *mut core::ffi::c_void, bytesused : *mut u32, recordcount : *mut u32) -> super::super::Foundation:: BOOL);
    TxfLogReadRecords(txflogcontext, bufferlength, buffer, bytesused, recordcount).ok()
}
#[inline]
pub unsafe fn TxfLogRecordGetFileName(recordbuffer: *const core::ffi::c_void, recordbufferlengthinbytes: u32, namebuffer: windows_core::PWSTR, namebufferlengthinbytes: *mut u32, txfid: Option<*mut TXF_ID>) -> super::super::Foundation::BOOL {
    windows_targets::link!("txfw32.dll" "system" fn TxfLogRecordGetFileName(recordbuffer : *const core::ffi::c_void, recordbufferlengthinbytes : u32, namebuffer : windows_core::PWSTR, namebufferlengthinbytes : *mut u32, txfid : *mut TXF_ID) -> super::super::Foundation:: BOOL);
    TxfLogRecordGetFileName(recordbuffer, recordbufferlengthinbytes, core::mem::transmute(namebuffer), namebufferlengthinbytes, core::mem::transmute(txfid.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn TxfLogRecordGetGenericType(recordbuffer: *const core::ffi::c_void, recordbufferlengthinbytes: u32, generictype: *mut u32, virtualclock: Option<*mut i64>) -> super::super::Foundation::BOOL {
    windows_targets::link!("txfw32.dll" "system" fn TxfLogRecordGetGenericType(recordbuffer : *const core::ffi::c_void, recordbufferlengthinbytes : u32, generictype : *mut u32, virtualclock : *mut i64) -> super::super::Foundation:: BOOL);
    TxfLogRecordGetGenericType(recordbuffer, recordbufferlengthinbytes, generictype, core::mem::transmute(virtualclock.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn TxfReadMetadataInfo<P0>(filehandle: P0, txffileid: *mut TXF_ID, lastlsn: *mut CLS_LSN, transactionstate: *mut u32, lockingtransaction: *mut windows_core::GUID) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("txfw32.dll" "system" fn TxfReadMetadataInfo(filehandle : super::super::Foundation:: HANDLE, txffileid : *mut TXF_ID, lastlsn : *mut CLS_LSN, transactionstate : *mut u32, lockingtransaction : *mut windows_core::GUID) -> super::super::Foundation:: BOOL);
    TxfReadMetadataInfo(filehandle.param().abi(), txffileid, lastlsn, transactionstate, lockingtransaction)
}
#[inline]
pub unsafe fn TxfSetThreadMiniVersionForCreate(miniversion: u16) {
    windows_targets::link!("txfw32.dll" "system" fn TxfSetThreadMiniVersionForCreate(miniversion : u16));
    TxfSetThreadMiniVersionForCreate(miniversion)
}
#[inline]
pub unsafe fn UnlockFile<P0>(hfile: P0, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn UnlockFile(hfile : super::super::Foundation:: HANDLE, dwfileoffsetlow : u32, dwfileoffsethigh : u32, nnumberofbytestounlocklow : u32, nnumberofbytestounlockhigh : u32) -> super::super::Foundation:: BOOL);
    UnlockFile(hfile.param().abi(), dwfileoffsetlow, dwfileoffsethigh, nnumberofbytestounlocklow, nnumberofbytestounlockhigh).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn UnlockFileEx<P0>(hfile: P0, dwreserved: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn UnlockFileEx(hfile : super::super::Foundation:: HANDLE, dwreserved : u32, nnumberofbytestounlocklow : u32, nnumberofbytestounlockhigh : u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    UnlockFileEx(hfile.param().abi(), dwreserved, nnumberofbytestounlocklow, nnumberofbytestounlockhigh, lpoverlapped).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ValidateLog<P0>(pszlogfilename: P0, psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES, pinfobuffer: *mut CLS_INFORMATION, pcbbuffer: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clfsw32.dll" "system" fn ValidateLog(pszlogfilename : windows_core::PCWSTR, psalogfile : *mut super::super::Security:: SECURITY_ATTRIBUTES, pinfobuffer : *mut CLS_INFORMATION, pcbbuffer : *mut u32) -> super::super::Foundation:: BOOL);
    ValidateLog(pszlogfilename.param().abi(), psalogfile, pinfobuffer, pcbbuffer).ok()
}
#[inline]
pub unsafe fn VerFindFileA<P0, P1, P2>(uflags: VER_FIND_FILE_FLAGS, szfilename: P0, szwindir: P1, szappdir: P2, szcurdir: windows_core::PSTR, pucurdirlen: *mut u32, szdestdir: windows_core::PSTR, pudestdirlen: *mut u32) -> VER_FIND_FILE_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("version.dll" "system" fn VerFindFileA(uflags : VER_FIND_FILE_FLAGS, szfilename : windows_core::PCSTR, szwindir : windows_core::PCSTR, szappdir : windows_core::PCSTR, szcurdir : windows_core::PSTR, pucurdirlen : *mut u32, szdestdir : windows_core::PSTR, pudestdirlen : *mut u32) -> VER_FIND_FILE_STATUS);
    VerFindFileA(uflags, szfilename.param().abi(), szwindir.param().abi(), szappdir.param().abi(), core::mem::transmute(szcurdir), pucurdirlen, core::mem::transmute(szdestdir), pudestdirlen)
}
#[inline]
pub unsafe fn VerFindFileW<P0, P1, P2>(uflags: VER_FIND_FILE_FLAGS, szfilename: P0, szwindir: P1, szappdir: P2, szcurdir: windows_core::PWSTR, pucurdirlen: *mut u32, szdestdir: windows_core::PWSTR, pudestdirlen: *mut u32) -> VER_FIND_FILE_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("version.dll" "system" fn VerFindFileW(uflags : VER_FIND_FILE_FLAGS, szfilename : windows_core::PCWSTR, szwindir : windows_core::PCWSTR, szappdir : windows_core::PCWSTR, szcurdir : windows_core::PWSTR, pucurdirlen : *mut u32, szdestdir : windows_core::PWSTR, pudestdirlen : *mut u32) -> VER_FIND_FILE_STATUS);
    VerFindFileW(uflags, szfilename.param().abi(), szwindir.param().abi(), szappdir.param().abi(), core::mem::transmute(szcurdir), pucurdirlen, core::mem::transmute(szdestdir), pudestdirlen)
}
#[inline]
pub unsafe fn VerInstallFileA<P0, P1, P2, P3, P4>(uflags: VER_INSTALL_FILE_FLAGS, szsrcfilename: P0, szdestfilename: P1, szsrcdir: P2, szdestdir: P3, szcurdir: P4, sztmpfile: windows_core::PSTR, putmpfilelen: *mut u32) -> VER_INSTALL_FILE_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("version.dll" "system" fn VerInstallFileA(uflags : VER_INSTALL_FILE_FLAGS, szsrcfilename : windows_core::PCSTR, szdestfilename : windows_core::PCSTR, szsrcdir : windows_core::PCSTR, szdestdir : windows_core::PCSTR, szcurdir : windows_core::PCSTR, sztmpfile : windows_core::PSTR, putmpfilelen : *mut u32) -> VER_INSTALL_FILE_STATUS);
    VerInstallFileA(uflags, szsrcfilename.param().abi(), szdestfilename.param().abi(), szsrcdir.param().abi(), szdestdir.param().abi(), szcurdir.param().abi(), core::mem::transmute(sztmpfile), putmpfilelen)
}
#[inline]
pub unsafe fn VerInstallFileW<P0, P1, P2, P3, P4>(uflags: VER_INSTALL_FILE_FLAGS, szsrcfilename: P0, szdestfilename: P1, szsrcdir: P2, szdestdir: P3, szcurdir: P4, sztmpfile: windows_core::PWSTR, putmpfilelen: *mut u32) -> VER_INSTALL_FILE_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("version.dll" "system" fn VerInstallFileW(uflags : VER_INSTALL_FILE_FLAGS, szsrcfilename : windows_core::PCWSTR, szdestfilename : windows_core::PCWSTR, szsrcdir : windows_core::PCWSTR, szdestdir : windows_core::PCWSTR, szcurdir : windows_core::PCWSTR, sztmpfile : windows_core::PWSTR, putmpfilelen : *mut u32) -> VER_INSTALL_FILE_STATUS);
    VerInstallFileW(uflags, szsrcfilename.param().abi(), szdestfilename.param().abi(), szsrcdir.param().abi(), szdestdir.param().abi(), szcurdir.param().abi(), core::mem::transmute(sztmpfile), putmpfilelen)
}
#[inline]
pub unsafe fn VerLanguageNameA(wlang: u32, szlang: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn VerLanguageNameA(wlang : u32, szlang : windows_core::PSTR, cchlang : u32) -> u32);
    VerLanguageNameA(wlang, core::mem::transmute(szlang.as_ptr()), szlang.len().try_into().unwrap())
}
#[inline]
pub unsafe fn VerLanguageNameW(wlang: u32, szlang: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn VerLanguageNameW(wlang : u32, szlang : windows_core::PWSTR, cchlang : u32) -> u32);
    VerLanguageNameW(wlang, core::mem::transmute(szlang.as_ptr()), szlang.len().try_into().unwrap())
}
#[inline]
pub unsafe fn VerQueryValueA<P0>(pblock: *const core::ffi::c_void, lpsubblock: P0, lplpbuffer: *mut *mut core::ffi::c_void, pulen: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("version.dll" "system" fn VerQueryValueA(pblock : *const core::ffi::c_void, lpsubblock : windows_core::PCSTR, lplpbuffer : *mut *mut core::ffi::c_void, pulen : *mut u32) -> super::super::Foundation:: BOOL);
    VerQueryValueA(pblock, lpsubblock.param().abi(), lplpbuffer, pulen)
}
#[inline]
pub unsafe fn VerQueryValueW<P0>(pblock: *const core::ffi::c_void, lpsubblock: P0, lplpbuffer: *mut *mut core::ffi::c_void, pulen: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("version.dll" "system" fn VerQueryValueW(pblock : *const core::ffi::c_void, lpsubblock : windows_core::PCWSTR, lplpbuffer : *mut *mut core::ffi::c_void, pulen : *mut u32) -> super::super::Foundation:: BOOL);
    VerQueryValueW(pblock, lpsubblock.param().abi(), lplpbuffer, pulen)
}
#[inline]
pub unsafe fn WofEnumEntries<P0>(volumename: P0, provider: u32, enumproc: WofEnumEntryProc, userdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofEnumEntries(volumename : windows_core::PCWSTR, provider : u32, enumproc : WofEnumEntryProc, userdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    WofEnumEntries(volumename.param().abi(), provider, enumproc, core::mem::transmute(userdata.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn WofFileEnumFiles<P0>(volumename: P0, algorithm: u32, enumproc: WofEnumFilesProc, userdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofFileEnumFiles(volumename : windows_core::PCWSTR, algorithm : u32, enumproc : WofEnumFilesProc, userdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    WofFileEnumFiles(volumename.param().abi(), algorithm, enumproc, core::mem::transmute(userdata.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn WofGetDriverVersion<P0>(fileorvolumehandle: P0, provider: u32) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofGetDriverVersion(fileorvolumehandle : super::super::Foundation:: HANDLE, provider : u32, wofversion : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WofGetDriverVersion(fileorvolumehandle.param().abi(), provider, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WofIsExternalFile<P0>(filepath: P0, isexternalfile: Option<*mut super::super::Foundation::BOOL>, provider: Option<*mut u32>, externalfileinfo: Option<*mut core::ffi::c_void>, bufferlength: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofIsExternalFile(filepath : windows_core::PCWSTR, isexternalfile : *mut super::super::Foundation:: BOOL, provider : *mut u32, externalfileinfo : *mut core::ffi::c_void, bufferlength : *mut u32) -> windows_core::HRESULT);
    WofIsExternalFile(filepath.param().abi(), core::mem::transmute(isexternalfile.unwrap_or(std::ptr::null_mut())), core::mem::transmute(provider.unwrap_or(std::ptr::null_mut())), core::mem::transmute(externalfileinfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(bufferlength.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WofSetFileDataLocation<P0>(filehandle: P0, provider: u32, externalfileinfo: *const core::ffi::c_void, length: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofSetFileDataLocation(filehandle : super::super::Foundation:: HANDLE, provider : u32, externalfileinfo : *const core::ffi::c_void, length : u32) -> windows_core::HRESULT);
    WofSetFileDataLocation(filehandle.param().abi(), provider, externalfileinfo, length).ok()
}
#[inline]
pub unsafe fn WofShouldCompressBinaries<P0>(volume: P0, algorithm: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofShouldCompressBinaries(volume : windows_core::PCWSTR, algorithm : *mut u32) -> super::super::Foundation:: BOOL);
    WofShouldCompressBinaries(volume.param().abi(), algorithm)
}
#[inline]
pub unsafe fn WofWimAddEntry<P0, P1>(volumename: P0, wimpath: P1, wimtype: u32, wimindex: u32) -> windows_core::Result<i64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofWimAddEntry(volumename : windows_core::PCWSTR, wimpath : windows_core::PCWSTR, wimtype : u32, wimindex : u32, datasourceid : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WofWimAddEntry(volumename.param().abi(), wimpath.param().abi(), wimtype, wimindex, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WofWimEnumFiles<P0>(volumename: P0, datasourceid: i64, enumproc: WofEnumFilesProc, userdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofWimEnumFiles(volumename : windows_core::PCWSTR, datasourceid : i64, enumproc : WofEnumFilesProc, userdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    WofWimEnumFiles(volumename.param().abi(), datasourceid, enumproc, core::mem::transmute(userdata.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn WofWimRemoveEntry<P0>(volumename: P0, datasourceid: i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofWimRemoveEntry(volumename : windows_core::PCWSTR, datasourceid : i64) -> windows_core::HRESULT);
    WofWimRemoveEntry(volumename.param().abi(), datasourceid).ok()
}
#[inline]
pub unsafe fn WofWimSuspendEntry<P0>(volumename: P0, datasourceid: i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofWimSuspendEntry(volumename : windows_core::PCWSTR, datasourceid : i64) -> windows_core::HRESULT);
    WofWimSuspendEntry(volumename.param().abi(), datasourceid).ok()
}
#[inline]
pub unsafe fn WofWimUpdateEntry<P0, P1>(volumename: P0, datasourceid: i64, newwimpath: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wofutil.dll" "system" fn WofWimUpdateEntry(volumename : windows_core::PCWSTR, datasourceid : i64, newwimpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    WofWimUpdateEntry(volumename.param().abi(), datasourceid, newwimpath.param().abi()).ok()
}
#[inline]
pub unsafe fn Wow64DisableWow64FsRedirection(oldvalue: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn Wow64DisableWow64FsRedirection(oldvalue : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    Wow64DisableWow64FsRedirection(oldvalue).ok()
}
#[inline]
pub unsafe fn Wow64EnableWow64FsRedirection<P0>(wow64fsenableredirection: P0) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("kernel32.dll" "system" fn Wow64EnableWow64FsRedirection(wow64fsenableredirection : super::super::Foundation:: BOOLEAN) -> super::super::Foundation:: BOOLEAN);
    Wow64EnableWow64FsRedirection(wow64fsenableredirection.param().abi())
}
#[inline]
pub unsafe fn Wow64RevertWow64FsRedirection(olvalue: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn Wow64RevertWow64FsRedirection(olvalue : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    Wow64RevertWow64FsRedirection(olvalue).ok()
}
#[inline]
pub unsafe fn WriteEncryptedFileRaw(pfimportcallback: PFE_IMPORT_FUNC, pvcallbackcontext: Option<*const core::ffi::c_void>, pvcontext: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn WriteEncryptedFileRaw(pfimportcallback : PFE_IMPORT_FUNC, pvcallbackcontext : *const core::ffi::c_void, pvcontext : *const core::ffi::c_void) -> u32);
    WriteEncryptedFileRaw(pfimportcallback, core::mem::transmute(pvcallbackcontext.unwrap_or(std::ptr::null())), pvcontext)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn WriteFile<P0>(hfile: P0, lpbuffer: Option<&[u8]>, lpnumberofbyteswritten: Option<*mut u32>, lpoverlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteFile(hfile : super::super::Foundation:: HANDLE, lpbuffer : *const u8, nnumberofbytestowrite : u32, lpnumberofbyteswritten : *mut u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    WriteFile(hfile.param().abi(), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpnumberofbyteswritten.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn WriteFileEx<P0>(hfile: P0, lpbuffer: Option<&[u8]>, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteFileEx(hfile : super::super::Foundation:: HANDLE, lpbuffer : *const u8, nnumberofbytestowrite : u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED, lpcompletionroutine : super::super::System::IO:: LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation:: BOOL);
    WriteFileEx(hfile.param().abi(), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpoverlapped, lpcompletionroutine).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn WriteFileGather<P0>(hfile: P0, asegmentarray: *const FILE_SEGMENT_ELEMENT, nnumberofbytestowrite: u32, lpreserved: Option<*const u32>, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteFileGather(hfile : super::super::Foundation:: HANDLE, asegmentarray : *const FILE_SEGMENT_ELEMENT, nnumberofbytestowrite : u32, lpreserved : *const u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    WriteFileGather(hfile.param().abi(), asegmentarray, nnumberofbytestowrite, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), lpoverlapped).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn WriteLogRestartArea(pvmarshal: *mut core::ffi::c_void, pvrestartbuffer: *mut core::ffi::c_void, cbrestartbuffer: u32, plsnbase: *mut CLS_LSN, fflags: CLFS_FLAG, pcbwritten: *mut u32, plsnnext: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("clfsw32.dll" "system" fn WriteLogRestartArea(pvmarshal : *mut core::ffi::c_void, pvrestartbuffer : *mut core::ffi::c_void, cbrestartbuffer : u32, plsnbase : *mut CLS_LSN, fflags : CLFS_FLAG, pcbwritten : *mut u32, plsnnext : *mut CLS_LSN, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    WriteLogRestartArea(pvmarshal, pvrestartbuffer, cbrestartbuffer, plsnbase, fflags, pcbwritten, plsnnext, poverlapped).ok()
}
#[inline]
pub unsafe fn WriteTapemark<P0, P1>(hdevice: P0, dwtapemarktype: TAPEMARK_TYPE, dwtapemarkcount: u32, bimmediate: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteTapemark(hdevice : super::super::Foundation:: HANDLE, dwtapemarktype : TAPEMARK_TYPE, dwtapemarkcount : u32, bimmediate : super::super::Foundation:: BOOL) -> u32);
    WriteTapemark(hdevice.param().abi(), dwtapemarktype, dwtapemarkcount, bimmediate.param().abi())
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiskQuotaControl, IDiskQuotaControl_Vtbl, 0x7988b572_ec89_11cf_9c00_00aa00a14f56);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiskQuotaControl {
    type Target = super::super::System::Com::IConnectionPointContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiskQuotaControl, windows_core::IUnknown, super::super::System::Com::IConnectionPointContainer);
#[cfg(feature = "Win32_System_Com")]
impl IDiskQuotaControl {
    pub unsafe fn Initialize<P0, P1>(&self, pszpath: P0, breadwrite: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pszpath.param().abi(), breadwrite.param().abi()).ok()
    }
    pub unsafe fn SetQuotaState(&self, dwstate: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuotaState)(windows_core::Interface::as_raw(self), dwstate).ok()
    }
    pub unsafe fn GetQuotaState(&self, pdwstate: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuotaState)(windows_core::Interface::as_raw(self), pdwstate).ok()
    }
    pub unsafe fn SetQuotaLogFlags(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuotaLogFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetQuotaLogFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuotaLogFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetDefaultQuotaThreshold(&self, llthreshold: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultQuotaThreshold)(windows_core::Interface::as_raw(self), llthreshold).ok()
    }
    pub unsafe fn GetDefaultQuotaThreshold(&self, pllthreshold: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultQuotaThreshold)(windows_core::Interface::as_raw(self), pllthreshold).ok()
    }
    pub unsafe fn GetDefaultQuotaThresholdText<P0>(&self, psztext: P0, cchtext: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDefaultQuotaThresholdText)(windows_core::Interface::as_raw(self), psztext.param().abi(), cchtext).ok()
    }
    pub unsafe fn SetDefaultQuotaLimit(&self, lllimit: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultQuotaLimit)(windows_core::Interface::as_raw(self), lllimit).ok()
    }
    pub unsafe fn GetDefaultQuotaLimit(&self, plllimit: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultQuotaLimit)(windows_core::Interface::as_raw(self), plllimit).ok()
    }
    pub unsafe fn GetDefaultQuotaLimitText<P0>(&self, psztext: P0, cchtext: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDefaultQuotaLimitText)(windows_core::Interface::as_raw(self), psztext.param().abi(), cchtext).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn AddUserSid<P0>(&self, pusersid: P0, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> windows_core::Result<IDiskQuotaUser>
    where
        P0: windows_core::Param<super::super::Security::PSID>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddUserSid)(windows_core::Interface::as_raw(self), pusersid.param().abi(), fnameresolution, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddUserName<P0>(&self, pszlogonname: P0, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> windows_core::Result<IDiskQuotaUser>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddUserName)(windows_core::Interface::as_raw(self), pszlogonname.param().abi(), fnameresolution, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteUser<P0>(&self, puser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiskQuotaUser>,
    {
        (windows_core::Interface::vtable(self).DeleteUser)(windows_core::Interface::as_raw(self), puser.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn FindUserSid<P0>(&self, pusersid: P0, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> windows_core::Result<IDiskQuotaUser>
    where
        P0: windows_core::Param<super::super::Security::PSID>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindUserSid)(windows_core::Interface::as_raw(self), pusersid.param().abi(), fnameresolution, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindUserName<P0>(&self, pszlogonname: P0) -> windows_core::Result<IDiskQuotaUser>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindUserName)(windows_core::Interface::as_raw(self), pszlogonname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn CreateEnumUsers(&self, rgpusersids: *mut super::super::Security::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut Option<IEnumDiskQuotaUsers>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateEnumUsers)(windows_core::Interface::as_raw(self), rgpusersids, cpsids, fnameresolution, core::mem::transmute(ppenum)).ok()
    }
    pub unsafe fn CreateUserBatch(&self) -> windows_core::Result<IDiskQuotaUserBatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateUserBatch)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InvalidateSidNameCache(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InvalidateSidNameCache)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GiveUserNameResolutionPriority<P0>(&self, puser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiskQuotaUser>,
    {
        (windows_core::Interface::vtable(self).GiveUserNameResolutionPriority)(windows_core::Interface::as_raw(self), puser.param().abi()).ok()
    }
    pub unsafe fn ShutdownNameResolution(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutdownNameResolution)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiskQuotaControl_Vtbl {
    pub base__: super::super::System::Com::IConnectionPointContainer_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetQuotaState: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetQuotaState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetQuotaLogFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetQuotaLogFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDefaultQuotaThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetDefaultQuotaThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetDefaultQuotaThresholdText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetDefaultQuotaLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetDefaultQuotaLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetDefaultQuotaLimitText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub AddUserSid: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Security::PSID, DISKQUOTA_USERNAME_RESOLVE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    AddUserSid: usize,
    pub AddUserName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DISKQUOTA_USERNAME_RESOLVE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub FindUserSid: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Security::PSID, DISKQUOTA_USERNAME_RESOLVE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    FindUserSid: usize,
    pub FindUserName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub CreateEnumUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID, u32, DISKQUOTA_USERNAME_RESOLVE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    CreateEnumUsers: usize,
    pub CreateUserBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvalidateSidNameCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GiveUserNameResolutionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownNameResolution: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiskQuotaEvents, IDiskQuotaEvents_Vtbl, 0x7988b579_ec89_11cf_9c00_00aa00a14f56);
impl core::ops::Deref for IDiskQuotaEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiskQuotaEvents, windows_core::IUnknown);
impl IDiskQuotaEvents {
    pub unsafe fn OnUserNameChanged<P0>(&self, puser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiskQuotaUser>,
    {
        (windows_core::Interface::vtable(self).OnUserNameChanged)(windows_core::Interface::as_raw(self), puser.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDiskQuotaEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUserNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiskQuotaUser, IDiskQuotaUser_Vtbl, 0x7988b574_ec89_11cf_9c00_00aa00a14f56);
impl core::ops::Deref for IDiskQuotaUser {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiskQuotaUser, windows_core::IUnknown);
impl IDiskQuotaUser {
    pub unsafe fn GetID(&self, pulid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), pulid).ok()
    }
    pub unsafe fn GetName<P0, P1, P2>(&self, pszaccountcontainer: P0, cchaccountcontainer: u32, pszlogonname: P1, cchlogonname: u32, pszdisplayname: P2, cchdisplayname: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), pszaccountcontainer.param().abi(), cchaccountcontainer, pszlogonname.param().abi(), cchlogonname, pszdisplayname.param().abi(), cchdisplayname).ok()
    }
    pub unsafe fn GetSidLength(&self, pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSidLength)(windows_core::Interface::as_raw(self), pdwlength).ok()
    }
    pub unsafe fn GetSid(&self, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSid)(windows_core::Interface::as_raw(self), pbsidbuffer, cbsidbuffer).ok()
    }
    pub unsafe fn GetQuotaThreshold(&self, pllthreshold: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuotaThreshold)(windows_core::Interface::as_raw(self), pllthreshold).ok()
    }
    pub unsafe fn GetQuotaThresholdText<P0>(&self, psztext: P0, cchtext: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetQuotaThresholdText)(windows_core::Interface::as_raw(self), psztext.param().abi(), cchtext).ok()
    }
    pub unsafe fn GetQuotaLimit(&self, plllimit: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuotaLimit)(windows_core::Interface::as_raw(self), plllimit).ok()
    }
    pub unsafe fn GetQuotaLimitText<P0>(&self, psztext: P0, cchtext: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetQuotaLimitText)(windows_core::Interface::as_raw(self), psztext.param().abi(), cchtext).ok()
    }
    pub unsafe fn GetQuotaUsed(&self, pllused: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuotaUsed)(windows_core::Interface::as_raw(self), pllused).ok()
    }
    pub unsafe fn GetQuotaUsedText<P0>(&self, psztext: P0, cchtext: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetQuotaUsedText)(windows_core::Interface::as_raw(self), psztext.param().abi(), cchtext).ok()
    }
    pub unsafe fn GetQuotaInformation(&self, pbquotainfo: *mut core::ffi::c_void, cbquotainfo: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuotaInformation)(windows_core::Interface::as_raw(self), pbquotainfo, cbquotainfo).ok()
    }
    pub unsafe fn SetQuotaThreshold<P0>(&self, llthreshold: i64, fwritethrough: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetQuotaThreshold)(windows_core::Interface::as_raw(self), llthreshold, fwritethrough.param().abi()).ok()
    }
    pub unsafe fn SetQuotaLimit<P0>(&self, lllimit: i64, fwritethrough: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetQuotaLimit)(windows_core::Interface::as_raw(self), lllimit, fwritethrough.param().abi()).ok()
    }
    pub unsafe fn Invalidate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invalidate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAccountStatus(&self, pdwstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAccountStatus)(windows_core::Interface::as_raw(self), pdwstatus).ok()
    }
}
#[repr(C)]
pub struct IDiskQuotaUser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetSidLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
    pub GetQuotaThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetQuotaThresholdText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetQuotaLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetQuotaLimitText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetQuotaUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetQuotaUsedText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetQuotaInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetQuotaThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, i64, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetQuotaLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i64, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAccountStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiskQuotaUserBatch, IDiskQuotaUserBatch_Vtbl, 0x7988b576_ec89_11cf_9c00_00aa00a14f56);
impl core::ops::Deref for IDiskQuotaUserBatch {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiskQuotaUserBatch, windows_core::IUnknown);
impl IDiskQuotaUserBatch {
    pub unsafe fn Add<P0>(&self, puser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiskQuotaUser>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), puser.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, puser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiskQuotaUser>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), puser.param().abi()).ok()
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FlushToDisk(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FlushToDisk)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDiskQuotaUserBatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushToDisk: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumDiskQuotaUsers, IEnumDiskQuotaUsers_Vtbl, 0x7988b577_ec89_11cf_9c00_00aa00a14f56);
impl core::ops::Deref for IEnumDiskQuotaUsers {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumDiskQuotaUsers, windows_core::IUnknown);
impl IEnumDiskQuotaUsers {
    pub unsafe fn Next(&self, cusers: u32, rgusers: *mut Option<IDiskQuotaUser>, pcusersfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cusers, core::mem::transmute(rgusers), pcusersfetched).ok()
    }
    pub unsafe fn Skip(&self, cusers: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cusers).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDiskQuotaUsers> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumDiskQuotaUsers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const ACCESS_ALL: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32768u32);
pub const ACCESS_ATRIB: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32u32);
pub const ACCESS_CREATE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(4u32);
pub const ACCESS_DELETE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(16u32);
pub const ACCESS_EXEC: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(8u32);
pub const ACCESS_PERM: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(64u32);
pub const ACCESS_READ: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(1u32);
pub const ACCESS_WRITE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(2u32);
pub const BACKUP_ALTERNATE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(4u32);
pub const BACKUP_DATA: WIN_STREAM_ID = WIN_STREAM_ID(1u32);
pub const BACKUP_EA_DATA: WIN_STREAM_ID = WIN_STREAM_ID(2u32);
pub const BACKUP_LINK: WIN_STREAM_ID = WIN_STREAM_ID(5u32);
pub const BACKUP_OBJECT_ID: WIN_STREAM_ID = WIN_STREAM_ID(7u32);
pub const BACKUP_PROPERTY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(6u32);
pub const BACKUP_REPARSE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(8u32);
pub const BACKUP_SECURITY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(3u32);
pub const BACKUP_SPARSE_BLOCK: WIN_STREAM_ID = WIN_STREAM_ID(9u32);
pub const BACKUP_TXFS_DATA: WIN_STREAM_ID = WIN_STREAM_ID(10u32);
pub const BusType1394: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(4i32);
pub const BusTypeAta: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(3i32);
pub const BusTypeAtapi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(2i32);
pub const BusTypeFibre: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(6i32);
pub const BusTypeFileBackedVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(15i32);
pub const BusTypeMax: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(20i32);
pub const BusTypeMaxReserved: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(127i32);
pub const BusTypeMmc: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(13i32);
pub const BusTypeNvme: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(17i32);
pub const BusTypeRAID: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(8i32);
pub const BusTypeSCM: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(18i32);
pub const BusTypeSas: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(10i32);
pub const BusTypeSata: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(11i32);
pub const BusTypeScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(1i32);
pub const BusTypeSd: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(12i32);
pub const BusTypeSpaces: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(16i32);
pub const BusTypeSsa: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(5i32);
pub const BusTypeUfs: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(19i32);
pub const BusTypeUnknown: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(0i32);
pub const BusTypeUsb: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(7i32);
pub const BusTypeVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(14i32);
pub const BusTypeiScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(9i32);
pub const CALLBACK_CHUNK_FINISHED: LPPROGRESS_ROUTINE_CALLBACK_REASON = LPPROGRESS_ROUTINE_CALLBACK_REASON(0u32);
pub const CALLBACK_STREAM_SWITCH: LPPROGRESS_ROUTINE_CALLBACK_REASON = LPPROGRESS_ROUTINE_CALLBACK_REASON(1u32);
pub const CLFS_BASELOG_EXTENSION: windows_core::PCWSTR = windows_core::w!(".blf");
pub const CLFS_CONTAINER_RELATIVE_PREFIX: windows_core::PCWSTR = windows_core::w!("%BLF%\\");
pub const CLFS_CONTAINER_STREAM_PREFIX: windows_core::PCWSTR = windows_core::w!("%BLF%:");
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: u32 = 16u32;
pub const CLFS_FLAG_FILTER_TOP_LEVEL: u32 = 32u32;
pub const CLFS_FLAG_FORCE_APPEND: CLFS_FLAG = CLFS_FLAG(1u32);
pub const CLFS_FLAG_FORCE_FLUSH: CLFS_FLAG = CLFS_FLAG(2u32);
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: u32 = 512u32;
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: u32 = 64u32;
pub const CLFS_FLAG_MINIFILTER_LEVEL: u32 = 256u32;
pub const CLFS_FLAG_NON_REENTRANT_FILTER: u32 = 16u32;
pub const CLFS_FLAG_NO_FLAGS: CLFS_FLAG = CLFS_FLAG(0u32);
pub const CLFS_FLAG_READ_IN_PROGRESS: u32 = 128u32;
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: u32 = 8u32;
pub const CLFS_FLAG_REENTRANT_FILTER: u32 = 32u32;
pub const CLFS_FLAG_USE_RESERVATION: CLFS_FLAG = CLFS_FLAG(4u32);
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: u32 = 1u32;
pub const CLFS_MARSHALLING_FLAG_NONE: u32 = 0u32;
pub const CLFS_MAX_CONTAINER_INFO: u32 = 256u32;
pub const CLFS_MGMT_CLIENT_REGISTRATION_VERSION: u32 = 1u32;
pub const CLFS_MGMT_POLICY_VERSION: u32 = 1u32;
pub const CLFS_SCAN_BACKWARD: u8 = 4u8;
pub const CLFS_SCAN_BUFFERED: u8 = 32u8;
pub const CLFS_SCAN_CLOSE: u8 = 8u8;
pub const CLFS_SCAN_FORWARD: u8 = 2u8;
pub const CLFS_SCAN_INIT: u8 = 1u8;
pub const CLFS_SCAN_INITIALIZED: u8 = 16u8;
pub const CLSID_DiskQuotaControl: windows_core::GUID = windows_core::GUID::from_u128(0x7988b571_ec89_11cf_9c00_00aa00a14f56);
pub const COMPRESSION_FORMAT_DEFAULT: COMPRESSION_FORMAT = COMPRESSION_FORMAT(1u16);
pub const COMPRESSION_FORMAT_LZNT1: COMPRESSION_FORMAT = COMPRESSION_FORMAT(2u16);
pub const COMPRESSION_FORMAT_NONE: COMPRESSION_FORMAT = COMPRESSION_FORMAT(0u16);
pub const COMPRESSION_FORMAT_XP10: COMPRESSION_FORMAT = COMPRESSION_FORMAT(5u16);
pub const COMPRESSION_FORMAT_XPRESS: COMPRESSION_FORMAT = COMPRESSION_FORMAT(3u16);
pub const COMPRESSION_FORMAT_XPRESS_HUFF: COMPRESSION_FORMAT = COMPRESSION_FORMAT(4u16);
pub const COPYFILE2_CALLBACK_CHUNK_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(2i32);
pub const COPYFILE2_CALLBACK_CHUNK_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(1i32);
pub const COPYFILE2_CALLBACK_ERROR: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(6i32);
pub const COPYFILE2_CALLBACK_MAX: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(7i32);
pub const COPYFILE2_CALLBACK_NONE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(0i32);
pub const COPYFILE2_CALLBACK_POLL_CONTINUE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(5i32);
pub const COPYFILE2_CALLBACK_STREAM_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(4i32);
pub const COPYFILE2_CALLBACK_STREAM_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(3i32);
pub const COPYFILE2_PHASE_MAX: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(7i32);
pub const COPYFILE2_PHASE_NAMEGRAFT_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(6i32);
pub const COPYFILE2_PHASE_NONE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(0i32);
pub const COPYFILE2_PHASE_PREPARE_DEST: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(2i32);
pub const COPYFILE2_PHASE_PREPARE_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(1i32);
pub const COPYFILE2_PHASE_READ_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(3i32);
pub const COPYFILE2_PHASE_SERVER_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(5i32);
pub const COPYFILE2_PHASE_WRITE_DESTINATION: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(4i32);
pub const COPYFILE2_PROGRESS_CANCEL: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(1i32);
pub const COPYFILE2_PROGRESS_CONTINUE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(0i32);
pub const COPYFILE2_PROGRESS_PAUSE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(4i32);
pub const COPYFILE2_PROGRESS_QUIET: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(3i32);
pub const COPYFILE2_PROGRESS_STOP: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(2i32);
pub const CREATE_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(2u32);
pub const CREATE_NEW: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(1u32);
pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: u32 = 2u32;
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: u32 = 1u32;
pub const CRM_PROTOCOL_MAXIMUM_OPTION: u32 = 3u32;
pub const CSC_CACHE_AUTO_REINT: u32 = 16u32;
pub const CSC_CACHE_MANUAL_REINT: u32 = 0u32;
pub const CSC_CACHE_NONE: u32 = 48u32;
pub const CSC_CACHE_VDO: u32 = 32u32;
pub const CSC_MASK: u32 = 48u32;
pub const CSC_MASK_EXT: u32 = 8240u32;
pub const CSV_BLOCK_AND_FILE_CACHE_CALLBACK_VERSION: u32 = 2u32;
pub const CSV_BLOCK_CACHE_CALLBACK_VERSION: u32 = 1u32;
pub const ClfsClientRecord: u8 = 3u8;
pub const ClfsContainerActive: u32 = 4u32;
pub const ClfsContainerActivePendingDelete: u32 = 8u32;
pub const ClfsContainerInactive: u32 = 2u32;
pub const ClfsContainerInitializing: u32 = 1u32;
pub const ClfsContainerPendingArchive: u32 = 16u32;
pub const ClfsContainerPendingArchiveAndDelete: u32 = 32u32;
pub const ClfsContextForward: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(3i32);
pub const ClfsContextNone: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(0i32);
pub const ClfsContextPrevious: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(2i32);
pub const ClfsContextUndoNext: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(1i32);
pub const ClfsDataRecord: u8 = 1u8;
pub const ClfsIoStatsDefault: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(0i32);
pub const ClfsIoStatsMax: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(65535i32);
pub const ClfsLogArchiveDisabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(2i32);
pub const ClfsLogArchiveEnabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(1i32);
pub const ClfsLogBasicInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(0i32);
pub const ClfsLogBasicInformationPhysical: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(1i32);
pub const ClfsLogPhysicalLsnInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(5i32);
pub const ClfsLogPhysicalNameInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(2i32);
pub const ClfsLogStreamIdentifierInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(3i32);
pub const ClfsLogSystemMarkingInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(4i32);
pub const ClfsMgmtAdvanceTailNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(0i32);
pub const ClfsMgmtLogFullHandlerNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(1i32);
pub const ClfsMgmtLogUnpinnedNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(2i32);
pub const ClfsMgmtLogWriteNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(3i32);
pub const ClfsMgmtPolicyAutoGrow: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(6i32);
pub const ClfsMgmtPolicyAutoShrink: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(5i32);
pub const ClfsMgmtPolicyGrowthRate: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(3i32);
pub const ClfsMgmtPolicyInvalid: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(10i32);
pub const ClfsMgmtPolicyLogTail: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(4i32);
pub const ClfsMgmtPolicyMaximumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(0i32);
pub const ClfsMgmtPolicyMinimumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(1i32);
pub const ClfsMgmtPolicyNewContainerExtension: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(9i32);
pub const ClfsMgmtPolicyNewContainerPrefix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(7i32);
pub const ClfsMgmtPolicyNewContainerSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(2i32);
pub const ClfsMgmtPolicyNewContainerSuffix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(8i32);
pub const ClfsNullRecord: u8 = 0u8;
pub const ClfsRestartRecord: u8 = 2u8;
pub const ClsContainerActive: u32 = 4u32;
pub const ClsContainerActivePendingDelete: u32 = 8u32;
pub const ClsContainerInactive: u32 = 2u32;
pub const ClsContainerInitializing: u32 = 1u32;
pub const ClsContainerPendingArchive: u32 = 16u32;
pub const ClsContainerPendingArchiveAndDelete: u32 = 32u32;
pub const ClsContextForward: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(3i32);
pub const ClsContextNone: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(0i32);
pub const ClsContextPrevious: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(2i32);
pub const ClsContextUndoNext: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(1i32);
pub const ClsIoStatsDefault: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(0i32);
pub const ClsIoStatsMax: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(65535i32);
pub const DDD_EXACT_MATCH_ON_REMOVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(4u32);
pub const DDD_LUID_BROADCAST_DRIVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(16u32);
pub const DDD_NO_BROADCAST_SYSTEM: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(8u32);
pub const DDD_RAW_TARGET_PATH: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(1u32);
pub const DDD_REMOVE_DEFINITION: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(2u32);
pub const DELETE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(65536u32);
pub const DISKQUOTA_FILESTATE_INCOMPLETE: u32 = 256u32;
pub const DISKQUOTA_FILESTATE_MASK: u32 = 768u32;
pub const DISKQUOTA_FILESTATE_REBUILDING: u32 = 512u32;
pub const DISKQUOTA_LOGFLAG_USER_LIMIT: u32 = 2u32;
pub const DISKQUOTA_LOGFLAG_USER_THRESHOLD: u32 = 1u32;
pub const DISKQUOTA_STATE_DISABLED: u32 = 0u32;
pub const DISKQUOTA_STATE_ENFORCE: u32 = 2u32;
pub const DISKQUOTA_STATE_MASK: u32 = 3u32;
pub const DISKQUOTA_STATE_TRACK: u32 = 1u32;
pub const DISKQUOTA_USERNAME_RESOLVE_ASYNC: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(2u32);
pub const DISKQUOTA_USERNAME_RESOLVE_NONE: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(0u32);
pub const DISKQUOTA_USERNAME_RESOLVE_SYNC: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(1u32);
pub const DISKQUOTA_USER_ACCOUNT_DELETED: u32 = 2u32;
pub const DISKQUOTA_USER_ACCOUNT_INVALID: u32 = 3u32;
pub const DISKQUOTA_USER_ACCOUNT_RESOLVED: u32 = 0u32;
pub const DISKQUOTA_USER_ACCOUNT_UNAVAILABLE: u32 = 1u32;
pub const DISKQUOTA_USER_ACCOUNT_UNKNOWN: u32 = 4u32;
pub const DISKQUOTA_USER_ACCOUNT_UNRESOLVED: u32 = 5u32;
pub const EA_CONTAINER_NAME: windows_core::PCSTR = windows_core::s!("ContainerName");
pub const EA_CONTAINER_SIZE: windows_core::PCSTR = windows_core::s!("ContainerSize");
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5u32;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6u32;
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1u32;
pub const EFS_METADATA_ADD_USER: u32 = 1u32;
pub const EFS_METADATA_GENERAL_OP: u32 = 8u32;
pub const EFS_METADATA_REMOVE_USER: u32 = 2u32;
pub const EFS_METADATA_REPLACE_USER: u32 = 4u32;
pub const EFS_PFILE_SUBVER_APPX: u32 = 3u32;
pub const EFS_PFILE_SUBVER_RMS: u32 = 2u32;
pub const EFS_SUBVER_UNKNOWN: u32 = 0u32;
pub const ENLISTMENT_MAXIMUM_OPTION: u32 = 1u32;
pub const ENLISTMENT_OBJECT_PATH: windows_core::PCWSTR = windows_core::w!("\\Enlistment\\");
pub const ENLISTMENT_SUPERIOR: u32 = 1u32;
pub const ExtendedFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(2i32);
pub const FILE_ACTION_ADDED: FILE_ACTION = FILE_ACTION(1u32);
pub const FILE_ACTION_MODIFIED: FILE_ACTION = FILE_ACTION(3u32);
pub const FILE_ACTION_REMOVED: FILE_ACTION = FILE_ACTION(2u32);
pub const FILE_ACTION_RENAMED_NEW_NAME: FILE_ACTION = FILE_ACTION(5u32);
pub const FILE_ACTION_RENAMED_OLD_NAME: FILE_ACTION = FILE_ACTION(4u32);
pub const FILE_ADD_FILE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(2u32);
pub const FILE_ADD_SUBDIRECTORY: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(4u32);
pub const FILE_ALL_ACCESS: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(2032127u32);
pub const FILE_APPEND_DATA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(4u32);
pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(32u32);
pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2048u32);
pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(64u32);
pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16u32);
pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16384u32);
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(32768u32);
pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(128u32);
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(8192u32);
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(131072u32);
pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4096u32);
pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4194304u32);
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1024u32);
pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(512u32);
pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4u32);
pub const FILE_ATTRIBUTE_TEMPORARY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(256u32);
pub const FILE_ATTRIBUTE_UNPINNED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const FILE_ATTRIBUTE_VIRTUAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
pub const FILE_BEGIN: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(0u32);
pub const FILE_CREATE_PIPE_INSTANCE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(4u32);
pub const FILE_CURRENT: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(1u32);
pub const FILE_DELETE_CHILD: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(64u32);
pub const FILE_DEVICE_CD_ROM: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(2u32);
pub const FILE_DEVICE_DISK: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(7u32);
pub const FILE_DEVICE_DVD: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(51u32);
pub const FILE_DEVICE_TAPE: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(31u32);
pub const FILE_DISPOSITION_FLAG_DELETE: FILE_DISPOSITION_INFO_EX_FLAGS = FILE_DISPOSITION_INFO_EX_FLAGS(1u32);
pub const FILE_DISPOSITION_FLAG_DO_NOT_DELETE: FILE_DISPOSITION_INFO_EX_FLAGS = FILE_DISPOSITION_INFO_EX_FLAGS(0u32);
pub const FILE_DISPOSITION_FLAG_FORCE_IMAGE_SECTION_CHECK: FILE_DISPOSITION_INFO_EX_FLAGS = FILE_DISPOSITION_INFO_EX_FLAGS(4u32);
pub const FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE: FILE_DISPOSITION_INFO_EX_FLAGS = FILE_DISPOSITION_INFO_EX_FLAGS(16u32);
pub const FILE_DISPOSITION_FLAG_ON_CLOSE: FILE_DISPOSITION_INFO_EX_FLAGS = FILE_DISPOSITION_INFO_EX_FLAGS(8u32);
pub const FILE_DISPOSITION_FLAG_POSIX_SEMANTICS: FILE_DISPOSITION_INFO_EX_FLAGS = FILE_DISPOSITION_INFO_EX_FLAGS(2u32);
pub const FILE_END: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(2u32);
pub const FILE_EXECUTE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(32u32);
pub const FILE_FLAG_BACKUP_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(33554432u32);
pub const FILE_FLAG_DELETE_ON_CLOSE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(67108864u32);
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const FILE_FLAG_NO_BUFFERING: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(536870912u32);
pub const FILE_FLAG_OPEN_NO_RECALL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const FILE_FLAG_OPEN_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2097152u32);
pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1073741824u32);
pub const FILE_FLAG_POSIX_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16777216u32);
pub const FILE_FLAG_RANDOM_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(268435456u32);
pub const FILE_FLAG_SEQUENTIAL_SCAN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(134217728u32);
pub const FILE_FLAG_SESSION_AWARE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(8388608u32);
pub const FILE_FLAG_WRITE_THROUGH: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2147483648u32);
pub const FILE_FLUSH_DATA: FILE_FLUSH_MODE = FILE_FLUSH_MODE(1i32);
pub const FILE_FLUSH_DEFAULT: FILE_FLUSH_MODE = FILE_FLUSH_MODE(0i32);
pub const FILE_FLUSH_MIN_METADATA: FILE_FLUSH_MODE = FILE_FLUSH_MODE(2i32);
pub const FILE_FLUSH_NO_SYNC: FILE_FLUSH_MODE = FILE_FLUSH_MODE(3i32);
pub const FILE_GENERIC_EXECUTE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1179808u32);
pub const FILE_GENERIC_READ: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1179785u32);
pub const FILE_GENERIC_WRITE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1179926u32);
pub const FILE_LIST_DIRECTORY: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1u32);
pub const FILE_NAME_NORMALIZED: GETFINALPATHNAMEBYHANDLE_FLAGS = GETFINALPATHNAMEBYHANDLE_FLAGS(0u32);
pub const FILE_NAME_OPENED: GETFINALPATHNAMEBYHANDLE_FLAGS = GETFINALPATHNAMEBYHANDLE_FLAGS(8u32);
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(4u32);
pub const FILE_NOTIFY_CHANGE_CREATION: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(64u32);
pub const FILE_NOTIFY_CHANGE_DIR_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(2u32);
pub const FILE_NOTIFY_CHANGE_FILE_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(1u32);
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(32u32);
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(16u32);
pub const FILE_NOTIFY_CHANGE_SECURITY: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(256u32);
pub const FILE_NOTIFY_CHANGE_SIZE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(8u32);
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
pub const FILE_READ_ATTRIBUTES: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(128u32);
pub const FILE_READ_DATA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1u32);
pub const FILE_READ_EA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(8u32);
pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = FILE_SHARE_MODE(4u32);
pub const FILE_SHARE_NONE: FILE_SHARE_MODE = FILE_SHARE_MODE(0u32);
pub const FILE_SHARE_READ: FILE_SHARE_MODE = FILE_SHARE_MODE(1u32);
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = FILE_SHARE_MODE(2u32);
pub const FILE_TRAVERSE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(32u32);
pub const FILE_TYPE_CHAR: FILE_TYPE = FILE_TYPE(2u32);
pub const FILE_TYPE_DISK: FILE_TYPE = FILE_TYPE(1u32);
pub const FILE_TYPE_PIPE: FILE_TYPE = FILE_TYPE(3u32);
pub const FILE_TYPE_REMOTE: FILE_TYPE = FILE_TYPE(32768u32);
pub const FILE_TYPE_UNKNOWN: FILE_TYPE = FILE_TYPE(0u32);
pub const FILE_VER_GET_LOCALISED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(1u32);
pub const FILE_VER_GET_NEUTRAL: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(2u32);
pub const FILE_VER_GET_PREFETCHED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(4u32);
pub const FILE_WRITE_ATTRIBUTES: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(256u32);
pub const FILE_WRITE_DATA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(2u32);
pub const FILE_WRITE_EA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(16u32);
pub const FILE_WRITE_FLAGS_NONE: FILE_WRITE_FLAGS = FILE_WRITE_FLAGS(0i32);
pub const FILE_WRITE_FLAGS_WRITE_THROUGH: FILE_WRITE_FLAGS = FILE_WRITE_FLAGS(1i32);
pub const FIND_FIRST_EX_CASE_SENSITIVE: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(1u32);
pub const FIND_FIRST_EX_LARGE_FETCH: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(2u32);
pub const FIND_FIRST_EX_ON_DISK_ENTRIES_ONLY: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(4u32);
pub const FileAlignmentInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(17i32);
pub const FileAllocationInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(5i32);
pub const FileAttributeTagInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(9i32);
pub const FileBasicInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(0i32);
pub const FileCaseSensitiveInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(23i32);
pub const FileCompressionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(8i32);
pub const FileDispositionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(4i32);
pub const FileDispositionInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(21i32);
pub const FileEndOfFileInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(6i32);
pub const FileFullDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(14i32);
pub const FileFullDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(15i32);
pub const FileIdBothDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(10i32);
pub const FileIdBothDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(11i32);
pub const FileIdExtdDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(19i32);
pub const FileIdExtdDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(20i32);
pub const FileIdInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(18i32);
pub const FileIdType: FILE_ID_TYPE = FILE_ID_TYPE(0i32);
pub const FileIoPriorityHintInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(12i32);
pub const FileNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(2i32);
pub const FileNormalizedNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(24i32);
pub const FileRemoteProtocolInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(13i32);
pub const FileRenameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(3i32);
pub const FileRenameInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(22i32);
pub const FileStandardInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(1i32);
pub const FileStorageInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(16i32);
pub const FileStreamInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(7i32);
pub const FindExInfoBasic: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(1i32);
pub const FindExInfoMaxInfoLevel: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(2i32);
pub const FindExInfoStandard: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(0i32);
pub const FindExSearchLimitToDevices: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(2i32);
pub const FindExSearchLimitToDirectories: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(1i32);
pub const FindExSearchMaxSearchOp: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(3i32);
pub const FindExSearchNameMatch: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(0i32);
pub const FindStreamInfoMaxInfoLevel: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(1i32);
pub const FindStreamInfoStandard: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(0i32);
pub const GET_TAPE_DRIVE_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION = GET_TAPE_DRIVE_PARAMETERS_OPERATION(1u32);
pub const GET_TAPE_MEDIA_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION = GET_TAPE_DRIVE_PARAMETERS_OPERATION(0u32);
pub const GetFileExInfoStandard: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(0i32);
pub const GetFileExMaxInfoLevel: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(1i32);
pub const INVALID_FILE_ATTRIBUTES: u32 = 4294967295u32;
pub const INVALID_FILE_SIZE: u32 = 4294967295u32;
pub const INVALID_SET_FILE_POINTER: u32 = 4294967295u32;
pub const IOCTL_VOLUME_ALLOCATE_BC_STREAM: u32 = 5685312u32;
pub const IOCTL_VOLUME_BASE: u32 = 86u32;
pub const IOCTL_VOLUME_BC_VERSION: u32 = 1u32;
pub const IOCTL_VOLUME_FREE_BC_STREAM: u32 = 5685316u32;
pub const IOCTL_VOLUME_GET_BC_PROPERTIES: u32 = 5652540u32;
pub const IOCTL_VOLUME_GET_CSVBLOCKCACHE_CALLBACK: u32 = 5685352u32;
pub const IOCTL_VOLUME_GET_GPT_ATTRIBUTES: u32 = 5636152u32;
pub const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: u32 = 5636096u32;
pub const IOCTL_VOLUME_IS_CLUSTERED: u32 = 5636144u32;
pub const IOCTL_VOLUME_IS_CSV: u32 = 5636192u32;
pub const IOCTL_VOLUME_IS_DYNAMIC: u32 = 5636168u32;
pub const IOCTL_VOLUME_IS_IO_CAPABLE: u32 = 5636116u32;
pub const IOCTL_VOLUME_IS_OFFLINE: u32 = 5636112u32;
pub const IOCTL_VOLUME_IS_PARTITION: u32 = 5636136u32;
pub const IOCTL_VOLUME_LOGICAL_TO_PHYSICAL: u32 = 5636128u32;
pub const IOCTL_VOLUME_OFFLINE: u32 = 5685260u32;
pub const IOCTL_VOLUME_ONLINE: u32 = 5685256u32;
pub const IOCTL_VOLUME_PHYSICAL_TO_LOGICAL: u32 = 5636132u32;
pub const IOCTL_VOLUME_POST_ONLINE: u32 = 5685348u32;
pub const IOCTL_VOLUME_PREPARE_FOR_CRITICAL_IO: u32 = 5685324u32;
pub const IOCTL_VOLUME_PREPARE_FOR_SHRINK: u32 = 5685340u32;
pub const IOCTL_VOLUME_QUERY_ALLOCATION_HINT: u32 = 5652562u32;
pub const IOCTL_VOLUME_QUERY_FAILOVER_SET: u32 = 5636120u32;
pub const IOCTL_VOLUME_QUERY_MINIMUM_SHRINK_SIZE: u32 = 5652568u32;
pub const IOCTL_VOLUME_QUERY_VOLUME_NUMBER: u32 = 5636124u32;
pub const IOCTL_VOLUME_READ_PLEX: u32 = 5652526u32;
pub const IOCTL_VOLUME_SET_GPT_ATTRIBUTES: u32 = 5636148u32;
pub const IOCTL_VOLUME_SUPPORTS_ONLINE_OFFLINE: u32 = 5636100u32;
pub const IOCTL_VOLUME_UPDATE_PROPERTIES: u32 = 5636180u32;
pub const IORING_CREATE_ADVISORY_FLAGS_NONE: IORING_CREATE_ADVISORY_FLAGS = IORING_CREATE_ADVISORY_FLAGS(0i32);
pub const IORING_CREATE_REQUIRED_FLAGS_NONE: IORING_CREATE_REQUIRED_FLAGS = IORING_CREATE_REQUIRED_FLAGS(0i32);
pub const IORING_FEATURE_FLAGS_NONE: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(0i32);
pub const IORING_FEATURE_SET_COMPLETION_EVENT: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(2i32);
pub const IORING_FEATURE_UM_EMULATION: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(1i32);
pub const IORING_OP_CANCEL: IORING_OP_CODE = IORING_OP_CODE(4i32);
pub const IORING_OP_FLUSH: IORING_OP_CODE = IORING_OP_CODE(6i32);
pub const IORING_OP_NOP: IORING_OP_CODE = IORING_OP_CODE(0i32);
pub const IORING_OP_READ: IORING_OP_CODE = IORING_OP_CODE(1i32);
pub const IORING_OP_REGISTER_BUFFERS: IORING_OP_CODE = IORING_OP_CODE(3i32);
pub const IORING_OP_REGISTER_FILES: IORING_OP_CODE = IORING_OP_CODE(2i32);
pub const IORING_OP_WRITE: IORING_OP_CODE = IORING_OP_CODE(5i32);
pub const IORING_REF_RAW: IORING_REF_KIND = IORING_REF_KIND(0i32);
pub const IORING_REF_REGISTERED: IORING_REF_KIND = IORING_REF_KIND(1i32);
pub const IORING_VERSION_1: IORING_VERSION = IORING_VERSION(1i32);
pub const IORING_VERSION_2: IORING_VERSION = IORING_VERSION(2i32);
pub const IORING_VERSION_3: IORING_VERSION = IORING_VERSION(300i32);
pub const IORING_VERSION_INVALID: IORING_VERSION = IORING_VERSION(0i32);
pub const IOSQE_FLAGS_DRAIN_PRECEDING_OPS: IORING_SQE_FLAGS = IORING_SQE_FLAGS(1i32);
pub const IOSQE_FLAGS_NONE: IORING_SQE_FLAGS = IORING_SQE_FLAGS(0i32);
pub const IoPriorityHintLow: PRIORITY_HINT = PRIORITY_HINT(1i32);
pub const IoPriorityHintNormal: PRIORITY_HINT = PRIORITY_HINT(2i32);
pub const IoPriorityHintVeryLow: PRIORITY_HINT = PRIORITY_HINT(0i32);
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: u32 = 1u32;
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: u32 = 1u32;
pub const LOCKFILE_EXCLUSIVE_LOCK: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(2u32);
pub const LOCKFILE_FAIL_IMMEDIATELY: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(1u32);
pub const LOG_POLICY_OVERWRITE: u32 = 1u32;
pub const LOG_POLICY_PERSIST: u32 = 2u32;
pub const LZERROR_BADINHANDLE: i32 = -1i32;
pub const LZERROR_BADOUTHANDLE: i32 = -2i32;
pub const LZERROR_BADVALUE: i32 = -7i32;
pub const LZERROR_GLOBALLOC: i32 = -5i32;
pub const LZERROR_GLOBLOCK: i32 = -6i32;
pub const LZERROR_READ: i32 = -3i32;
pub const LZERROR_UNKNOWNALG: i32 = -8i32;
pub const LZERROR_WRITE: i32 = -4i32;
pub const MAXIMUM_REPARSE_DATA_BUFFER_SIZE: u32 = 16384u32;
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: u32 = 64u32;
pub const MAX_SID_SIZE: u32 = 256u32;
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: u32 = 64u32;
pub const MOVEFILE_COPY_ALLOWED: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(2u32);
pub const MOVEFILE_CREATE_HARDLINK: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(16u32);
pub const MOVEFILE_DELAY_UNTIL_REBOOT: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(4u32);
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(32u32);
pub const MOVEFILE_REPLACE_EXISTING: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(1u32);
pub const MOVEFILE_WRITE_THROUGH: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(8u32);
pub const MaximumFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(3i32);
pub const MaximumFileInfoByHandleClass: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(25i32);
pub const MaximumIoPriorityHintType: PRIORITY_HINT = PRIORITY_HINT(3i32);
pub const NTMSMLI_MAXAPPDESCR: u32 = 256u32;
pub const NTMSMLI_MAXIDSIZE: u32 = 256u32;
pub const NTMSMLI_MAXTYPE: u32 = 64u32;
pub const NTMS_ALLOCATE_ERROR_IF_UNAVAILABLE: NtmsAllocateOptions = NtmsAllocateOptions(4i32);
pub const NTMS_ALLOCATE_FROMSCRATCH: NtmsAllocationPolicy = NtmsAllocationPolicy(1i32);
pub const NTMS_ALLOCATE_NEW: NtmsAllocateOptions = NtmsAllocateOptions(1i32);
pub const NTMS_ALLOCATE_NEXT: NtmsAllocateOptions = NtmsAllocateOptions(2i32);
pub const NTMS_APPLICATIONNAME_LENGTH: u32 = 64u32;
pub const NTMS_ASYNCOP_MOUNT: NtmsAsyncOperations = NtmsAsyncOperations(1i32);
pub const NTMS_ASYNCSTATE_COMPLETE: NtmsAsyncStatus = NtmsAsyncStatus(4i32);
pub const NTMS_ASYNCSTATE_INPROCESS: NtmsAsyncStatus = NtmsAsyncStatus(3i32);
pub const NTMS_ASYNCSTATE_QUEUED: NtmsAsyncStatus = NtmsAsyncStatus(0i32);
pub const NTMS_ASYNCSTATE_WAIT_OPERATOR: NtmsAsyncStatus = NtmsAsyncStatus(2i32);
pub const NTMS_ASYNCSTATE_WAIT_RESOURCE: NtmsAsyncStatus = NtmsAsyncStatus(1i32);
pub const NTMS_BARCODESTATE_OK: NtmsBarCodeState = NtmsBarCodeState(1i32);
pub const NTMS_BARCODESTATE_UNREADABLE: NtmsBarCodeState = NtmsBarCodeState(2i32);
pub const NTMS_BARCODE_LENGTH: u32 = 64u32;
pub const NTMS_CHANGER: NtmsObjectsTypes = NtmsObjectsTypes(2i32);
pub const NTMS_CHANGER_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(3i32);
pub const NTMS_COMPUTER: NtmsObjectsTypes = NtmsObjectsTypes(4i32);
pub const NTMS_COMPUTERNAME_LENGTH: u32 = 64u32;
pub const NTMS_CONTROL_ACCESS: NtmsAccessMask = NtmsAccessMask(4i32);
pub const NTMS_CREATE_NEW: NtmsCreateOptions = NtmsCreateOptions(2i32);
pub const NTMS_DEALLOCATE_TOSCRATCH: NtmsDeallocationPolicy = NtmsDeallocationPolicy(1i32);
pub const NTMS_DESCRIPTION_LENGTH: u32 = 127u32;
pub const NTMS_DEVICENAME_LENGTH: u32 = 64u32;
pub const NTMS_DISMOUNT_DEFERRED: NtmsDismountOptions = NtmsDismountOptions(1i32);
pub const NTMS_DISMOUNT_IMMEDIATE: NtmsDismountOptions = NtmsDismountOptions(2i32);
pub const NTMS_DOORSTATE_CLOSED: NtmsDoorState = NtmsDoorState(1i32);
pub const NTMS_DOORSTATE_OPEN: NtmsDoorState = NtmsDoorState(2i32);
pub const NTMS_DOORSTATE_UNKNOWN: NtmsDoorState = NtmsDoorState(0i32);
pub const NTMS_DRIVE: NtmsObjectsTypes = NtmsObjectsTypes(5i32);
pub const NTMS_DRIVESTATE_BEING_CLEANED: NtmsDriveState = NtmsDriveState(6i32);
pub const NTMS_DRIVESTATE_DISMOUNTABLE: NtmsDriveState = NtmsDriveState(7i32);
pub const NTMS_DRIVESTATE_DISMOUNTED: NtmsDriveState = NtmsDriveState(0i32);
pub const NTMS_DRIVESTATE_LOADED: NtmsDriveState = NtmsDriveState(2i32);
pub const NTMS_DRIVESTATE_MOUNTED: NtmsDriveState = NtmsDriveState(1i32);
pub const NTMS_DRIVESTATE_UNLOADED: NtmsDriveState = NtmsDriveState(5i32);
pub const NTMS_DRIVE_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(6i32);
pub const NTMS_EJECT_ASK_USER: NtmsEjectOperation = NtmsEjectOperation(5i32);
pub const NTMS_EJECT_FORCE: NtmsEjectOperation = NtmsEjectOperation(3i32);
pub const NTMS_EJECT_IMMEDIATE: NtmsEjectOperation = NtmsEjectOperation(4i32);
pub const NTMS_EJECT_QUEUE: NtmsEjectOperation = NtmsEjectOperation(2i32);
pub const NTMS_EJECT_START: NtmsEjectOperation = NtmsEjectOperation(0i32);
pub const NTMS_EJECT_STOP: NtmsEjectOperation = NtmsEjectOperation(1i32);
pub const NTMS_ENUM_DEFAULT: NtmsEnumerateOption = NtmsEnumerateOption(0i32);
pub const NTMS_ENUM_ROOTPOOL: NtmsEnumerateOption = NtmsEnumerateOption(1i32);
pub const NTMS_ERROR_ON_DUPLICATE: NtmsCreateNtmsMediaOptions = NtmsCreateNtmsMediaOptions(1i32);
pub const NTMS_EVENT_COMPLETE: NtmsNotificationOperations = NtmsNotificationOperations(5i32);
pub const NTMS_EVENT_SIGNAL: NtmsNotificationOperations = NtmsNotificationOperations(4i32);
pub const NTMS_I1_MESSAGE_LENGTH: u32 = 127u32;
pub const NTMS_IEDOOR: NtmsObjectsTypes = NtmsObjectsTypes(7i32);
pub const NTMS_IEPORT: NtmsObjectsTypes = NtmsObjectsTypes(8i32);
pub const NTMS_INITIALIZING: NtmsOperationalState = NtmsOperationalState(10i32);
pub const NTMS_INJECT_RETRACT: NtmsInjectOperation = NtmsInjectOperation(2i32);
pub const NTMS_INJECT_START: NtmsInjectOperation = NtmsInjectOperation(0i32);
pub const NTMS_INJECT_STARTMANY: NtmsInjectOperation = NtmsInjectOperation(3i32);
pub const NTMS_INJECT_STOP: NtmsInjectOperation = NtmsInjectOperation(1i32);
pub const NTMS_INVENTORY_DEFAULT: NtmsInventoryMethod = NtmsInventoryMethod(3i32);
pub const NTMS_INVENTORY_FAST: NtmsInventoryMethod = NtmsInventoryMethod(1i32);
pub const NTMS_INVENTORY_MAX: NtmsInventoryMethod = NtmsInventoryMethod(6i32);
pub const NTMS_INVENTORY_NONE: NtmsInventoryMethod = NtmsInventoryMethod(0i32);
pub const NTMS_INVENTORY_OMID: NtmsInventoryMethod = NtmsInventoryMethod(2i32);
pub const NTMS_INVENTORY_SLOT: NtmsInventoryMethod = NtmsInventoryMethod(4i32);
pub const NTMS_INVENTORY_STOP: NtmsInventoryMethod = NtmsInventoryMethod(5i32);
pub const NTMS_LIBRARY: NtmsObjectsTypes = NtmsObjectsTypes(9i32);
pub const NTMS_LIBRARYFLAG_AUTODETECTCHANGE: NtmsLibraryFlags = NtmsLibraryFlags(4i32);
pub const NTMS_LIBRARYFLAG_CLEANERPRESENT: NtmsLibraryFlags = NtmsLibraryFlags(2i32);
pub const NTMS_LIBRARYFLAG_FIXEDOFFLINE: NtmsLibraryFlags = NtmsLibraryFlags(1i32);
pub const NTMS_LIBRARYFLAG_IGNORECLEANERUSESREMAINING: NtmsLibraryFlags = NtmsLibraryFlags(8i32);
pub const NTMS_LIBRARYFLAG_RECOGNIZECLEANERBARCODE: NtmsLibraryFlags = NtmsLibraryFlags(16i32);
pub const NTMS_LIBRARYTYPE_OFFLINE: NtmsLibraryType = NtmsLibraryType(1i32);
pub const NTMS_LIBRARYTYPE_ONLINE: NtmsLibraryType = NtmsLibraryType(2i32);
pub const NTMS_LIBRARYTYPE_STANDALONE: NtmsLibraryType = NtmsLibraryType(3i32);
pub const NTMS_LIBRARYTYPE_UNKNOWN: NtmsLibraryType = NtmsLibraryType(0i32);
pub const NTMS_LIBREQFLAGS_NOAUTOPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(1i32);
pub const NTMS_LIBREQFLAGS_NOFAILEDPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(2i32);
pub const NTMS_LIBREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(10i32);
pub const NTMS_LM_CANCELLED: NtmsLmState = NtmsLmState(7i32);
pub const NTMS_LM_CLASSIFY: NtmsLmOperation = NtmsLmOperation(19i32);
pub const NTMS_LM_CLEANDRIVE: NtmsLmOperation = NtmsLmOperation(15i32);
pub const NTMS_LM_DEFERRED: NtmsLmState = NtmsLmState(6i32);
pub const NTMS_LM_DEFFERED: NtmsLmState = NtmsLmState(6i32);
pub const NTMS_LM_DISABLECHANGER: NtmsLmOperation = NtmsLmOperation(1i32);
pub const NTMS_LM_DISABLEDRIVE: NtmsLmOperation = NtmsLmOperation(3i32);
pub const NTMS_LM_DISABLELIBRARY: NtmsLmOperation = NtmsLmOperation(1i32);
pub const NTMS_LM_DISABLEMEDIA: NtmsLmOperation = NtmsLmOperation(5i32);
pub const NTMS_LM_DISMOUNT: NtmsLmOperation = NtmsLmOperation(16i32);
pub const NTMS_LM_DOORACCESS: NtmsLmOperation = NtmsLmOperation(9i32);
pub const NTMS_LM_EJECT: NtmsLmOperation = NtmsLmOperation(10i32);
pub const NTMS_LM_EJECTCLEANER: NtmsLmOperation = NtmsLmOperation(11i32);
pub const NTMS_LM_ENABLECHANGER: NtmsLmOperation = NtmsLmOperation(2i32);
pub const NTMS_LM_ENABLEDRIVE: NtmsLmOperation = NtmsLmOperation(4i32);
pub const NTMS_LM_ENABLELIBRARY: NtmsLmOperation = NtmsLmOperation(2i32);
pub const NTMS_LM_ENABLEMEDIA: NtmsLmOperation = NtmsLmOperation(6i32);
pub const NTMS_LM_FAILED: NtmsLmState = NtmsLmState(3i32);
pub const NTMS_LM_INJECT: NtmsLmOperation = NtmsLmOperation(12i32);
pub const NTMS_LM_INJECTCLEANER: NtmsLmOperation = NtmsLmOperation(13i32);
pub const NTMS_LM_INPROCESS: NtmsLmState = NtmsLmState(1i32);
pub const NTMS_LM_INVALID: NtmsLmState = NtmsLmState(4i32);
pub const NTMS_LM_INVENTORY: NtmsLmOperation = NtmsLmOperation(8i32);
pub const NTMS_LM_MAXWORKITEM: NtmsLmOperation = NtmsLmOperation(22i32);
pub const NTMS_LM_MOUNT: NtmsLmOperation = NtmsLmOperation(17i32);
pub const NTMS_LM_PASSED: NtmsLmState = NtmsLmState(2i32);
pub const NTMS_LM_PROCESSOMID: NtmsLmOperation = NtmsLmOperation(14i32);
pub const NTMS_LM_QUEUED: NtmsLmState = NtmsLmState(0i32);
pub const NTMS_LM_RELEASECLEANER: NtmsLmOperation = NtmsLmOperation(21i32);
pub const NTMS_LM_REMOVE: NtmsLmOperation = NtmsLmOperation(0i32);
pub const NTMS_LM_RESERVECLEANER: NtmsLmOperation = NtmsLmOperation(20i32);
pub const NTMS_LM_STOPPED: NtmsLmState = NtmsLmState(8i32);
pub const NTMS_LM_UPDATEOMID: NtmsLmOperation = NtmsLmOperation(7i32);
pub const NTMS_LM_WAITING: NtmsLmState = NtmsLmState(5i32);
pub const NTMS_LM_WRITESCRATCH: NtmsLmOperation = NtmsLmOperation(18i32);
pub const NTMS_LOGICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(11i32);
pub const NTMS_MAXATTR_LENGTH: u32 = 65536u32;
pub const NTMS_MAXATTR_NAMELEN: u32 = 32u32;
pub const NTMS_MEDIARW_READONLY: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(3i32);
pub const NTMS_MEDIARW_REWRITABLE: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(1i32);
pub const NTMS_MEDIARW_UNKNOWN: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(0i32);
pub const NTMS_MEDIARW_WRITEONCE: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(2i32);
pub const NTMS_MEDIASTATE_IDLE: NtmsMediaState = NtmsMediaState(0i32);
pub const NTMS_MEDIASTATE_INUSE: NtmsMediaState = NtmsMediaState(1i32);
pub const NTMS_MEDIASTATE_LOADED: NtmsMediaState = NtmsMediaState(3i32);
pub const NTMS_MEDIASTATE_MOUNTED: NtmsMediaState = NtmsMediaState(2i32);
pub const NTMS_MEDIASTATE_OPERROR: NtmsMediaState = NtmsMediaState(5i32);
pub const NTMS_MEDIASTATE_OPREQ: NtmsMediaState = NtmsMediaState(6i32);
pub const NTMS_MEDIASTATE_UNLOADED: NtmsMediaState = NtmsMediaState(4i32);
pub const NTMS_MEDIA_POOL: NtmsObjectsTypes = NtmsObjectsTypes(12i32);
pub const NTMS_MEDIA_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(13i32);
pub const NTMS_MESSAGE_LENGTH: u32 = 256u32;
pub const NTMS_MODIFY_ACCESS: NtmsAccessMask = NtmsAccessMask(2i32);
pub const NTMS_MOUNT_ERROR_IF_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
pub const NTMS_MOUNT_ERROR_IF_UNAVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
pub const NTMS_MOUNT_ERROR_NOT_AVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
pub const NTMS_MOUNT_ERROR_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
pub const NTMS_MOUNT_NOWAIT: NtmsMountOptions = NtmsMountOptions(32i32);
pub const NTMS_MOUNT_READ: NtmsMountOptions = NtmsMountOptions(1i32);
pub const NTMS_MOUNT_SPECIFIC_DRIVE: NtmsMountOptions = NtmsMountOptions(16i32);
pub const NTMS_MOUNT_WRITE: NtmsMountOptions = NtmsMountOptions(2i32);
pub const NTMS_NEEDS_SERVICE: NtmsOperationalState = NtmsOperationalState(20i32);
pub const NTMS_NOT_PRESENT: NtmsOperationalState = NtmsOperationalState(21i32);
pub const NTMS_NUMBER_OF_OBJECT_TYPES: NtmsObjectsTypes = NtmsObjectsTypes(19i32);
pub const NTMS_OBJECT: NtmsObjectsTypes = NtmsObjectsTypes(1i32);
pub const NTMS_OBJECTNAME_LENGTH: u32 = 64u32;
pub const NTMS_OBJ_DELETE: NtmsNotificationOperations = NtmsNotificationOperations(3i32);
pub const NTMS_OBJ_INSERT: NtmsNotificationOperations = NtmsNotificationOperations(2i32);
pub const NTMS_OBJ_UPDATE: NtmsNotificationOperations = NtmsNotificationOperations(1i32);
pub const NTMS_OMIDLABELID_LENGTH: u32 = 255u32;
pub const NTMS_OMIDLABELINFO_LENGTH: u32 = 256u32;
pub const NTMS_OMIDLABELTYPE_LENGTH: u32 = 64u32;
pub const NTMS_OMID_TYPE_FILESYSTEM_INFO: NTMS_OMID_TYPE = NTMS_OMID_TYPE(2u32);
pub const NTMS_OMID_TYPE_RAW_LABEL: NTMS_OMID_TYPE = NTMS_OMID_TYPE(1u32);
pub const NTMS_OPEN_ALWAYS: NtmsCreateOptions = NtmsCreateOptions(3i32);
pub const NTMS_OPEN_EXISTING: NtmsCreateOptions = NtmsCreateOptions(1i32);
pub const NTMS_OPREQFLAGS_NOALERTS: NtmsOpRequestFlags = NtmsOpRequestFlags(16i32);
pub const NTMS_OPREQFLAGS_NOAUTOPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(1i32);
pub const NTMS_OPREQFLAGS_NOFAILEDPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(2i32);
pub const NTMS_OPREQFLAGS_NOTRAYICON: NtmsOpRequestFlags = NtmsOpRequestFlags(32i32);
pub const NTMS_OPREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(17i32);
pub const NTMS_OPREQ_CLEANER: NtmsOpreqCommand = NtmsOpreqCommand(2i32);
pub const NTMS_OPREQ_DEVICESERVICE: NtmsOpreqCommand = NtmsOpreqCommand(3i32);
pub const NTMS_OPREQ_MESSAGE: NtmsOpreqCommand = NtmsOpreqCommand(5i32);
pub const NTMS_OPREQ_MOVEMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(4i32);
pub const NTMS_OPREQ_NEWMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(1i32);
pub const NTMS_OPREQ_UNKNOWN: NtmsOpreqCommand = NtmsOpreqCommand(0i32);
pub const NTMS_OPSTATE_ACTIVE: NtmsOpreqState = NtmsOpreqState(2i32);
pub const NTMS_OPSTATE_COMPLETE: NtmsOpreqState = NtmsOpreqState(5i32);
pub const NTMS_OPSTATE_INPROGRESS: NtmsOpreqState = NtmsOpreqState(3i32);
pub const NTMS_OPSTATE_REFUSED: NtmsOpreqState = NtmsOpreqState(4i32);
pub const NTMS_OPSTATE_SUBMITTED: NtmsOpreqState = NtmsOpreqState(1i32);
pub const NTMS_OPSTATE_UNKNOWN: NtmsOpreqState = NtmsOpreqState(0i32);
pub const NTMS_PARTITION: NtmsObjectsTypes = NtmsObjectsTypes(14i32);
pub const NTMS_PARTSTATE_ALLOCATED: NtmsPartitionState = NtmsPartitionState(5i32);
pub const NTMS_PARTSTATE_AVAILABLE: NtmsPartitionState = NtmsPartitionState(4i32);
pub const NTMS_PARTSTATE_COMPLETE: NtmsPartitionState = NtmsPartitionState(6i32);
pub const NTMS_PARTSTATE_DECOMMISSIONED: NtmsPartitionState = NtmsPartitionState(3i32);
pub const NTMS_PARTSTATE_FOREIGN: NtmsPartitionState = NtmsPartitionState(7i32);
pub const NTMS_PARTSTATE_IMPORT: NtmsPartitionState = NtmsPartitionState(8i32);
pub const NTMS_PARTSTATE_INCOMPATIBLE: NtmsPartitionState = NtmsPartitionState(2i32);
pub const NTMS_PARTSTATE_RESERVED: NtmsPartitionState = NtmsPartitionState(9i32);
pub const NTMS_PARTSTATE_UNKNOWN: NtmsPartitionState = NtmsPartitionState(0i32);
pub const NTMS_PARTSTATE_UNPREPARED: NtmsPartitionState = NtmsPartitionState(1i32);
pub const NTMS_PHYSICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(15i32);
pub const NTMS_POOLHIERARCHY_LENGTH: u32 = 512u32;
pub const NTMS_POOLPOLICY_KEEPOFFLINEIMPORT: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(2i32);
pub const NTMS_POOLPOLICY_PURGEOFFLINESCRATCH: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(1i32);
pub const NTMS_POOLTYPE_APPLICATION: NtmsPoolType = NtmsPoolType(1000i32);
pub const NTMS_POOLTYPE_FOREIGN: NtmsPoolType = NtmsPoolType(2i32);
pub const NTMS_POOLTYPE_IMPORT: NtmsPoolType = NtmsPoolType(3i32);
pub const NTMS_POOLTYPE_SCRATCH: NtmsPoolType = NtmsPoolType(1i32);
pub const NTMS_POOLTYPE_UNKNOWN: NtmsPoolType = NtmsPoolType(0i32);
pub const NTMS_PORTCONTENT_EMPTY: NtmsPortContent = NtmsPortContent(2i32);
pub const NTMS_PORTCONTENT_FULL: NtmsPortContent = NtmsPortContent(1i32);
pub const NTMS_PORTCONTENT_UNKNOWN: NtmsPortContent = NtmsPortContent(0i32);
pub const NTMS_PORTPOSITION_EXTENDED: NtmsPortPosition = NtmsPortPosition(1i32);
pub const NTMS_PORTPOSITION_RETRACTED: NtmsPortPosition = NtmsPortPosition(2i32);
pub const NTMS_PORTPOSITION_UNKNOWN: NtmsPortPosition = NtmsPortPosition(0i32);
pub const NTMS_PRIORITY_DEFAULT: NtmsMountPriority = NtmsMountPriority(0i32);
pub const NTMS_PRIORITY_HIGH: NtmsMountPriority = NtmsMountPriority(7i32);
pub const NTMS_PRIORITY_HIGHEST: NtmsMountPriority = NtmsMountPriority(15i32);
pub const NTMS_PRIORITY_LOW: NtmsMountPriority = NtmsMountPriority(-7i32);
pub const NTMS_PRIORITY_LOWEST: NtmsMountPriority = NtmsMountPriority(-15i32);
pub const NTMS_PRIORITY_NORMAL: NtmsMountPriority = NtmsMountPriority(0i32);
pub const NTMS_PRODUCTNAME_LENGTH: u32 = 128u32;
pub const NTMS_READY: NtmsOperationalState = NtmsOperationalState(0i32);
pub const NTMS_REVISION_LENGTH: u32 = 32u32;
pub const NTMS_SEQUENCE_LENGTH: u32 = 32u32;
pub const NTMS_SERIALNUMBER_LENGTH: u32 = 32u32;
pub const NTMS_SESSION_QUERYEXPEDITE: NtmsSessionOptions = NtmsSessionOptions(1i32);
pub const NTMS_SLOTSTATE_EMPTY: NtmsSlotState = NtmsSlotState(2i32);
pub const NTMS_SLOTSTATE_FULL: NtmsSlotState = NtmsSlotState(1i32);
pub const NTMS_SLOTSTATE_NEEDSINVENTORY: NtmsSlotState = NtmsSlotState(4i32);
pub const NTMS_SLOTSTATE_NOTPRESENT: NtmsSlotState = NtmsSlotState(3i32);
pub const NTMS_SLOTSTATE_UNKNOWN: NtmsSlotState = NtmsSlotState(0i32);
pub const NTMS_STORAGESLOT: NtmsObjectsTypes = NtmsObjectsTypes(16i32);
pub const NTMS_UIDEST_ADD: NtmsUIOperations = NtmsUIOperations(1i32);
pub const NTMS_UIDEST_DELETE: NtmsUIOperations = NtmsUIOperations(2i32);
pub const NTMS_UIDEST_DELETEALL: NtmsUIOperations = NtmsUIOperations(3i32);
pub const NTMS_UIOPERATION_MAX: NtmsUIOperations = NtmsUIOperations(4i32);
pub const NTMS_UITYPE_ERR: NtmsUITypes = NtmsUITypes(3i32);
pub const NTMS_UITYPE_INFO: NtmsUITypes = NtmsUITypes(1i32);
pub const NTMS_UITYPE_INVALID: NtmsUITypes = NtmsUITypes(0i32);
pub const NTMS_UITYPE_MAX: NtmsUITypes = NtmsUITypes(4i32);
pub const NTMS_UITYPE_REQ: NtmsUITypes = NtmsUITypes(2i32);
pub const NTMS_UI_DESTINATION: NtmsObjectsTypes = NtmsObjectsTypes(18i32);
pub const NTMS_UNKNOWN: NtmsObjectsTypes = NtmsObjectsTypes(0i32);
pub const NTMS_UNKNOWN_DRIVE: NtmsDriveType = NtmsDriveType(0i32);
pub const NTMS_USERNAME_LENGTH: u32 = 64u32;
pub const NTMS_USE_ACCESS: NtmsAccessMask = NtmsAccessMask(1i32);
pub const NTMS_VENDORNAME_LENGTH: u32 = 128u32;
pub const OF_CANCEL: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2048u16);
pub const OF_CREATE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(4096u16);
pub const OF_DELETE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(512u16);
pub const OF_EXIST: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16384u16);
pub const OF_PARSE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(256u16);
pub const OF_PROMPT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(8192u16);
pub const OF_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u16);
pub const OF_READWRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2u16);
pub const OF_REOPEN: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32768u16);
pub const OF_SHARE_COMPAT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u16);
pub const OF_SHARE_DENY_NONE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(64u16);
pub const OF_SHARE_DENY_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(48u16);
pub const OF_SHARE_DENY_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32u16);
pub const OF_SHARE_EXCLUSIVE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16u16);
pub const OF_VERIFY: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1024u16);
pub const OF_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1u16);
pub const OPEN_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(4u32);
pub const OPEN_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(3u32);
pub const ObjectIdType: FILE_ID_TYPE = FILE_ID_TYPE(1i32);
pub const PARTITION_BASIC_DATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xebd0a0a2_b9e5_4433_87c0_68b6b72699c7);
pub const PARTITION_BSP_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_4df9_45b9_8e9e_2370f006457c);
pub const PARTITION_CLUSTER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdb97dba9_0840_4bae_97f0_ffb9a327c7e1);
pub const PARTITION_DPP_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_94cb_43f0_a533_d73c10cfa57d);
pub const PARTITION_ENTRY_UNUSED_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const PARTITION_LDM_DATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaf9b60a0_1431_4f62_bc68_3311714a69ad);
pub const PARTITION_LDM_METADATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5808c8aa_7e8f_42e0_85d2_e1e90434cfb3);
pub const PARTITION_LEGACY_BL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x424ca0e2_7cb2_4fb9_8143_c52a99398bc6);
pub const PARTITION_LEGACY_BL_GUID_BACKUP: windows_core::GUID = windows_core::GUID::from_u128(0x424c3e6c_d79f_49cb_935d_36d71467a288);
pub const PARTITION_MAIN_OS_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_8f45_405e_8a23_186d8a4330d3);
pub const PARTITION_MSFT_RECOVERY_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xde94bba4_06d1_4d40_a16a_bfd50179d6ac);
pub const PARTITION_MSFT_RESERVED_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe3c9e316_0b5c_4db8_817d_f92df00215ae);
pub const PARTITION_MSFT_SNAPSHOT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcaddebf1_4400_4de8_b103_12117dcf3ccf);
pub const PARTITION_OS_DATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_23f2_44d5_a830_67bbdaa609f9);
pub const PARTITION_PATCH_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8967a686_96aa_6aa8_9589_a84256541090);
pub const PARTITION_PRE_INSTALLED_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_7fe0_4196_9b42_427b51643484);
pub const PARTITION_SBL_CACHE_HDD_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x03aaa829_ebfc_4e7e_aac9_c4d76c63b24b);
pub const PARTITION_SBL_CACHE_SSD_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeeff8352_dd2a_44db_ae83_bee1cf7481dc);
pub const PARTITION_SBL_CACHE_SSD_RESERVED_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdcc0c7c1_55ad_4f17_9d43_4bc776e0117e);
pub const PARTITION_SERVICING_FILES_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_432e_4014_ae4c_8deaa9c0006a);
pub const PARTITION_SERVICING_METADATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_c691_4a05_bb4e_703dafd229ce);
pub const PARTITION_SERVICING_RESERVE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_4b81_460b_a319_ffb6fe136d14);
pub const PARTITION_SERVICING_STAGING_ROOT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_e84d_4e84_aaf3_ecbbbd04b9df);
pub const PARTITION_SPACES_DATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe7addcb4_dc34_4539_9a76_ebbd07be6f7e);
pub const PARTITION_SPACES_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe75caf8f_f680_4cee_afa3_b001e56efc2d);
pub const PARTITION_SYSTEM_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc12a7328_f81f_11d2_ba4b_00a0c93ec93b);
pub const PARTITION_WINDOWS_SYSTEM_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x57434f53_e3e3_4631_a5c5_26d2243873aa);
pub const PERM_FILE_CREATE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(4u32);
pub const PERM_FILE_READ: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(1u32);
pub const PERM_FILE_WRITE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(2u32);
pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(3u32);
pub const PIPE_ACCESS_INBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
pub const PIPE_ACCESS_OUTBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
pub const QUIC: SERVER_CERTIFICATE_TYPE = SERVER_CERTIFICATE_TYPE(0i32);
pub const READ_CONTROL: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(131072u32);
pub const REPLACEFILE_IGNORE_ACL_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(4u32);
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(2u32);
pub const REPLACEFILE_WRITE_THROUGH: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(1u32);
pub const RESOURCE_MANAGER_COMMUNICATION: u32 = 2u32;
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: u32 = 3u32;
pub const RESOURCE_MANAGER_OBJECT_PATH: windows_core::PCWSTR = windows_core::w!("\\ResourceManager\\");
pub const RESOURCE_MANAGER_VOLATILE: u32 = 1u32;
pub const ReadDirectoryNotifyExtendedInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(2i32);
pub const ReadDirectoryNotifyFullInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(3i32);
pub const ReadDirectoryNotifyInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(1i32);
pub const ReadDirectoryNotifyMaximumInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(4i32);
pub const SECURITY_ANONYMOUS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(0u32);
pub const SECURITY_CONTEXT_TRACKING: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const SECURITY_DELEGATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(196608u32);
pub const SECURITY_EFFECTIVE_ONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const SECURITY_IDENTIFICATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
pub const SECURITY_IMPERSONATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(131072u32);
pub const SECURITY_SQOS_PRESENT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const SECURITY_VALID_SQOS_FLAGS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2031616u32);
pub const SESI1_NUM_ELEMENTS: u32 = 8u32;
pub const SESI2_NUM_ELEMENTS: u32 = 9u32;
pub const SESS_GUEST: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(1u32);
pub const SESS_NOENCRYPTION: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(2u32);
pub const SET_TAPE_DRIVE_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(1u32);
pub const SET_TAPE_MEDIA_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(0u32);
pub const SHARE_CURRENT_USES_PARMNUM: u32 = 7u32;
pub const SHARE_FILE_SD_PARMNUM: u32 = 501u32;
pub const SHARE_MAX_USES_PARMNUM: u32 = 6u32;
pub const SHARE_NETNAME_PARMNUM: u32 = 1u32;
pub const SHARE_PASSWD_PARMNUM: u32 = 9u32;
pub const SHARE_PATH_PARMNUM: u32 = 8u32;
pub const SHARE_PERMISSIONS_PARMNUM: u32 = 5u32;
pub const SHARE_QOS_POLICY_PARMNUM: u32 = 504u32;
pub const SHARE_REMARK_PARMNUM: u32 = 4u32;
pub const SHARE_SERVER_PARMNUM: u32 = 503u32;
pub const SHARE_TYPE_PARMNUM: u32 = 3u32;
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: u32 = 2048u32;
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: u32 = 1024u32;
pub const SHI1005_FLAGS_CLUSTER_MANAGED: u32 = 524288u32;
pub const SHI1005_FLAGS_COMPRESS_DATA: u32 = 1048576u32;
pub const SHI1005_FLAGS_DFS: u32 = 1u32;
pub const SHI1005_FLAGS_DFS_ROOT: u32 = 2u32;
pub const SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING: u32 = 131072u32;
pub const SHI1005_FLAGS_DISABLE_DIRECTORY_HANDLE_LEASING: u32 = 4194304u32;
pub const SHI1005_FLAGS_ENABLE_CA: u32 = 16384u32;
pub const SHI1005_FLAGS_ENABLE_HASH: u32 = 8192u32;
pub const SHI1005_FLAGS_ENCRYPT_DATA: u32 = 32768u32;
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: u32 = 4096u32;
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: u32 = 512u32;
pub const SHI1005_FLAGS_IDENTITY_REMOTING: u32 = 262144u32;
pub const SHI1005_FLAGS_ISOLATED_TRANSPORT: u32 = 2097152u32;
pub const SHI1005_FLAGS_RESERVED: u32 = 65536u32;
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: u32 = 256u32;
pub const SHI1_NUM_ELEMENTS: u32 = 4u32;
pub const SHI2_NUM_ELEMENTS: u32 = 10u32;
pub const SHI_USES_UNLIMITED: u32 = 4294967295u32;
pub const SPECIFIC_RIGHTS_ALL: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(65535u32);
pub const STANDARD_RIGHTS_ALL: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(2031616u32);
pub const STANDARD_RIGHTS_EXECUTE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(131072u32);
pub const STANDARD_RIGHTS_READ: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(131072u32);
pub const STANDARD_RIGHTS_REQUIRED: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(983040u32);
pub const STANDARD_RIGHTS_WRITE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(131072u32);
pub const STATSOPT_CLR: u32 = 1u32;
pub const STYPE_DEVICE: SHARE_TYPE = SHARE_TYPE(2u32);
pub const STYPE_DISKTREE: SHARE_TYPE = SHARE_TYPE(0u32);
pub const STYPE_IPC: SHARE_TYPE = SHARE_TYPE(3u32);
pub const STYPE_MASK: SHARE_TYPE = SHARE_TYPE(255u32);
pub const STYPE_PRINTQ: SHARE_TYPE = SHARE_TYPE(1u32);
pub const STYPE_RESERVED1: u32 = 16777216u32;
pub const STYPE_RESERVED2: u32 = 33554432u32;
pub const STYPE_RESERVED3: u32 = 67108864u32;
pub const STYPE_RESERVED4: u32 = 134217728u32;
pub const STYPE_RESERVED5: u32 = 1048576u32;
pub const STYPE_RESERVED_ALL: u32 = 1073741568u32;
pub const STYPE_SPECIAL: SHARE_TYPE = SHARE_TYPE(2147483648u32);
pub const STYPE_TEMPORARY: SHARE_TYPE = SHARE_TYPE(1073741824u32);
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: SYMBOLIC_LINK_FLAGS = SYMBOLIC_LINK_FLAGS(2u32);
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: SYMBOLIC_LINK_FLAGS = SYMBOLIC_LINK_FLAGS(1u32);
pub const SYNCHRONIZE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1048576u32);
pub const TAPE_ABSOLUTE_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(1u32);
pub const TAPE_ABSOLUTE_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(0u32);
pub const TAPE_ERASE_LONG: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(1u32);
pub const TAPE_ERASE_SHORT: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(0u32);
pub const TAPE_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(1u32);
pub const TAPE_FIXED_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(0u32);
pub const TAPE_FORMAT: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(5u32);
pub const TAPE_INITIATOR_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(2u32);
pub const TAPE_LOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(0u32);
pub const TAPE_LOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(3u32);
pub const TAPE_LOGICAL_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(2u32);
pub const TAPE_LOGICAL_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(1u32);
pub const TAPE_LONG_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(3u32);
pub const TAPE_REWIND: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(0u32);
pub const TAPE_SELECT_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(1u32);
pub const TAPE_SETMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(0u32);
pub const TAPE_SHORT_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(2u32);
pub const TAPE_SPACE_END_OF_DATA: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(4u32);
pub const TAPE_SPACE_FILEMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(6u32);
pub const TAPE_SPACE_RELATIVE_BLOCKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(5u32);
pub const TAPE_SPACE_SEQUENTIAL_FMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(7u32);
pub const TAPE_SPACE_SEQUENTIAL_SMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(9u32);
pub const TAPE_SPACE_SETMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(8u32);
pub const TAPE_TENSION: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(2u32);
pub const TAPE_UNLOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(1u32);
pub const TAPE_UNLOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(4u32);
pub const TRANSACTIONMANAGER_OBJECT_PATH: windows_core::PCWSTR = windows_core::w!("\\TransactionManager\\");
pub const TRANSACTION_DO_NOT_PROMOTE: u32 = 1u32;
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: u32 = 0u32;
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: u32 = 8u32;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: u32 = 4u32;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: u32 = 2u32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: u32 = 32u32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: u32 = 16u32;
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: u32 = 63u32;
pub const TRANSACTION_MANAGER_VOLATILE: u32 = 1u32;
pub const TRANSACTION_MAXIMUM_OPTION: u32 = 1u32;
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: u32 = 1u32;
pub const TRANSACTION_NOTIFY_COMMIT: u32 = 4u32;
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: u32 = 64u32;
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: u32 = 1073741824u32;
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: u32 = 67108864u32;
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: u32 = 1024u32;
pub const TRANSACTION_NOTIFY_ENLIST_MASK: u32 = 262144u32;
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: u32 = 4096u32;
pub const TRANSACTION_NOTIFY_INDOUBT: u32 = 16384u32;
pub const TRANSACTION_NOTIFY_LAST_RECOVER: u32 = 8192u32;
pub const TRANSACTION_NOTIFY_MARSHAL: u32 = 131072u32;
pub const TRANSACTION_NOTIFY_MASK: u32 = 1073741823u32;
pub const TRANSACTION_NOTIFY_PREPARE: u32 = 2u32;
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: u32 = 32u32;
pub const TRANSACTION_NOTIFY_PREPREPARE: u32 = 1u32;
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: u32 = 16u32;
pub const TRANSACTION_NOTIFY_PROMOTE: u32 = 134217728u32;
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: u32 = 268435456u32;
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: u32 = 32768u32;
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: u32 = 65536u32;
pub const TRANSACTION_NOTIFY_RECOVER: u32 = 256u32;
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: u32 = 2048u32;
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: u32 = 536870912u32;
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: u32 = 16777216u32;
pub const TRANSACTION_NOTIFY_ROLLBACK: u32 = 8u32;
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: u32 = 128u32;
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: u32 = 512u32;
pub const TRANSACTION_NOTIFY_TM_ONLINE: u32 = 33554432u32;
pub const TRANSACTION_OBJECT_PATH: windows_core::PCWSTR = windows_core::w!("\\Transaction\\");
pub const TRUNCATE_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(5u32);
pub const TXFS_MINIVERSION_COMMITTED_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(0u32);
pub const TXFS_MINIVERSION_DEFAULT_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65534u32);
pub const TXFS_MINIVERSION_DIRTY_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65535u32);
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: u32 = 2u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: u32 = 1u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: u32 = 8u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: u32 = 4u32;
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(4u16);
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(2u16);
pub const TXF_LOG_RECORD_TYPE_WRITE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(1u16);
pub const TransactionOutcomeAborted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(3i32);
pub const TransactionOutcomeCommitted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(2i32);
pub const TransactionOutcomeUndetermined: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(1i32);
pub const VFFF_ISSHAREDFILE: VER_FIND_FILE_FLAGS = VER_FIND_FILE_FLAGS(1u32);
pub const VFF_BUFFTOOSMALL: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(4u32);
pub const VFF_CURNEDEST: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(1u32);
pub const VFF_FILEINUSE: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(2u32);
pub const VFT2_DRV_COMM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(10i32);
pub const VFT2_DRV_DISPLAY: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(4i32);
pub const VFT2_DRV_INPUTMETHOD: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(11i32);
pub const VFT2_DRV_INSTALLABLE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(8i32);
pub const VFT2_DRV_KEYBOARD: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
pub const VFT2_DRV_LANGUAGE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
pub const VFT2_DRV_MOUSE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(5i32);
pub const VFT2_DRV_NETWORK: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(6i32);
pub const VFT2_DRV_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
pub const VFT2_DRV_SOUND: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(9i32);
pub const VFT2_DRV_SYSTEM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(7i32);
pub const VFT2_DRV_VERSIONED_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(12i32);
pub const VFT2_FONT_RASTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
pub const VFT2_FONT_TRUETYPE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
pub const VFT2_FONT_VECTOR: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
pub const VFT2_UNKNOWN: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(0i32);
pub const VFT_APP: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(1i32);
pub const VFT_DLL: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(2i32);
pub const VFT_DRV: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(3i32);
pub const VFT_FONT: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(4i32);
pub const VFT_STATIC_LIB: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(7i32);
pub const VFT_UNKNOWN: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(0i32);
pub const VFT_VXD: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(5i32);
pub const VIFF_DONTDELETEOLD: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(2u32);
pub const VIFF_FORCEINSTALL: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(1u32);
pub const VIF_ACCESSVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(512u32);
pub const VIF_BUFFTOOSMALL: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(262144u32);
pub const VIF_CANNOTCREATE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2048u32);
pub const VIF_CANNOTDELETE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4096u32);
pub const VIF_CANNOTDELETECUR: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16384u32);
pub const VIF_CANNOTLOADCABINET: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1048576u32);
pub const VIF_CANNOTLOADLZ32: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(524288u32);
pub const VIF_CANNOTREADDST: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(131072u32);
pub const VIF_CANNOTREADSRC: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(65536u32);
pub const VIF_CANNOTRENAME: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8192u32);
pub const VIF_DIFFCODEPG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16u32);
pub const VIF_DIFFLANG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8u32);
pub const VIF_DIFFTYPE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32u32);
pub const VIF_FILEINUSE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(128u32);
pub const VIF_MISMATCH: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2u32);
pub const VIF_OUTOFMEMORY: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32768u32);
pub const VIF_OUTOFSPACE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(256u32);
pub const VIF_SHARINGVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1024u32);
pub const VIF_SRCOLD: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4u32);
pub const VIF_TEMPFILE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1u32);
pub const VIF_WRITEPROT: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(64u32);
pub const VOLUME_NAME_DOS: GETFINALPATHNAMEBYHANDLE_FLAGS = GETFINALPATHNAMEBYHANDLE_FLAGS(0u32);
pub const VOLUME_NAME_GUID: GETFINALPATHNAMEBYHANDLE_FLAGS = GETFINALPATHNAMEBYHANDLE_FLAGS(1u32);
pub const VOLUME_NAME_NONE: GETFINALPATHNAMEBYHANDLE_FLAGS = GETFINALPATHNAMEBYHANDLE_FLAGS(4u32);
pub const VOLUME_NAME_NT: GETFINALPATHNAMEBYHANDLE_FLAGS = GETFINALPATHNAMEBYHANDLE_FLAGS(2u32);
pub const VOS_DOS: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65536u32);
pub const VOS_DOS_WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65537u32);
pub const VOS_DOS_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65540u32);
pub const VOS_NT: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262144u32);
pub const VOS_NT_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262148u32);
pub const VOS_OS216: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131072u32);
pub const VOS_OS216_PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131074u32);
pub const VOS_OS232: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196608u32);
pub const VOS_OS232_PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196611u32);
pub const VOS_UNKNOWN: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0u32);
pub const VOS_WINCE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(327680u32);
pub const VOS__BASE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0u32);
pub const VOS__PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(2u32);
pub const VOS__PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(3u32);
pub const VOS__WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(1u32);
pub const VOS__WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(4u32);
pub const VS_FFI_FILEFLAGSMASK: i32 = 63i32;
pub const VS_FFI_SIGNATURE: i32 = -17890115i32;
pub const VS_FFI_STRUCVERSION: i32 = 65536i32;
pub const VS_FF_DEBUG: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(1u32);
pub const VS_FF_INFOINFERRED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(16u32);
pub const VS_FF_PATCHED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(4u32);
pub const VS_FF_PRERELEASE: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(2u32);
pub const VS_FF_PRIVATEBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(8u32);
pub const VS_FF_SPECIALBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(32u32);
pub const VS_USER_DEFINED: u32 = 100u32;
pub const VS_VERSION_INFO: u32 = 1u32;
pub const WIM_BOOT_NOT_OS_WIM: u32 = 0u32;
pub const WIM_BOOT_OS_WIM: u32 = 1u32;
pub const WIM_ENTRY_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_ENTRY_FLAG_SUSPENDED: u32 = 2u32;
pub const WIM_EXTERNAL_FILE_INFO_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_EXTERNAL_FILE_INFO_FLAG_SUSPENDED: u32 = 2u32;
pub const WIM_PROVIDER_HASH_SIZE: u32 = 20u32;
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1u32;
pub const WOF_PROVIDER_FILE: u32 = 2u32;
pub const WOF_PROVIDER_WIM: u32 = 1u32;
pub const WRITE_DAC: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(262144u32);
pub const WRITE_OWNER: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(524288u32);
pub const _FT_TYPES_DEFINITION_: u32 = 1u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLFS_CONTEXT_MODE(pub i32);
impl windows_core::TypeKind for CLFS_CONTEXT_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLFS_CONTEXT_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLFS_CONTEXT_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLFS_FLAG(pub u32);
impl windows_core::TypeKind for CLFS_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLFS_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLFS_FLAG").field(&self.0).finish()
    }
}
impl CLFS_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CLFS_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CLFS_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CLFS_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CLFS_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CLFS_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLFS_IOSTATS_CLASS(pub i32);
impl windows_core::TypeKind for CLFS_IOSTATS_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLFS_IOSTATS_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLFS_IOSTATS_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLFS_LOG_ARCHIVE_MODE(pub i32);
impl windows_core::TypeKind for CLFS_LOG_ARCHIVE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLFS_LOG_ARCHIVE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLFS_LOG_ARCHIVE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLFS_MGMT_NOTIFICATION_TYPE(pub i32);
impl windows_core::TypeKind for CLFS_MGMT_NOTIFICATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLFS_MGMT_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLFS_MGMT_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLFS_MGMT_POLICY_TYPE(pub i32);
impl windows_core::TypeKind for CLFS_MGMT_POLICY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLFS_MGMT_POLICY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLFS_MGMT_POLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLS_CONTEXT_MODE(pub i32);
impl windows_core::TypeKind for CLS_CONTEXT_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLS_CONTEXT_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLS_CONTEXT_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLS_IOSTATS_CLASS(pub i32);
impl windows_core::TypeKind for CLS_IOSTATS_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLS_IOSTATS_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLS_IOSTATS_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLS_LOG_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for CLS_LOG_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLS_LOG_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLS_LOG_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPRESSION_FORMAT(pub u16);
impl windows_core::TypeKind for COMPRESSION_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPRESSION_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPRESSION_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COPYFILE2_COPY_PHASE(pub i32);
impl windows_core::TypeKind for COPYFILE2_COPY_PHASE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COPYFILE2_COPY_PHASE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COPYFILE2_COPY_PHASE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COPYFILE2_MESSAGE_ACTION(pub i32);
impl windows_core::TypeKind for COPYFILE2_MESSAGE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COPYFILE2_MESSAGE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COPYFILE2_MESSAGE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COPYFILE2_MESSAGE_TYPE(pub i32);
impl windows_core::TypeKind for COPYFILE2_MESSAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COPYFILE2_MESSAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COPYFILE2_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CREATE_TAPE_PARTITION_METHOD(pub u32);
impl windows_core::TypeKind for CREATE_TAPE_PARTITION_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CREATE_TAPE_PARTITION_METHOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CREATE_TAPE_PARTITION_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEFINE_DOS_DEVICE_FLAGS(pub u32);
impl windows_core::TypeKind for DEFINE_DOS_DEVICE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEFINE_DOS_DEVICE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEFINE_DOS_DEVICE_FLAGS").field(&self.0).finish()
    }
}
impl DEFINE_DOS_DEVICE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISKQUOTA_USERNAME_RESOLVE(pub u32);
impl windows_core::TypeKind for DISKQUOTA_USERNAME_RESOLVE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISKQUOTA_USERNAME_RESOLVE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISKQUOTA_USERNAME_RESOLVE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ERASE_TAPE_TYPE(pub u32);
impl windows_core::TypeKind for ERASE_TAPE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ERASE_TAPE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ERASE_TAPE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_ACCESS_RIGHTS(pub u32);
impl windows_core::TypeKind for FILE_ACCESS_RIGHTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl FILE_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_ACTION(pub u32);
impl windows_core::TypeKind for FILE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_CREATION_DISPOSITION(pub u32);
impl windows_core::TypeKind for FILE_CREATION_DISPOSITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_CREATION_DISPOSITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_CREATION_DISPOSITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_DEVICE_TYPE(pub u32);
impl windows_core::TypeKind for FILE_DEVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_DEVICE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_DISPOSITION_INFO_EX_FLAGS(pub u32);
impl windows_core::TypeKind for FILE_DISPOSITION_INFO_EX_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_DISPOSITION_INFO_EX_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_DISPOSITION_INFO_EX_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_FLAGS_AND_ATTRIBUTES(pub u32);
impl windows_core::TypeKind for FILE_FLAGS_AND_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_FLAGS_AND_ATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_FLAGS_AND_ATTRIBUTES").field(&self.0).finish()
    }
}
impl FILE_FLAGS_AND_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_FLUSH_MODE(pub i32);
impl windows_core::TypeKind for FILE_FLUSH_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_FLUSH_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_FLUSH_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_ID_TYPE(pub i32);
impl windows_core::TypeKind for FILE_ID_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_ID_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_ID_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_INFO_BY_HANDLE_CLASS(pub i32);
impl windows_core::TypeKind for FILE_INFO_BY_HANDLE_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_INFO_BY_HANDLE_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_INFO_BY_HANDLE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_INFO_FLAGS_PERMISSIONS(pub u32);
impl windows_core::TypeKind for FILE_INFO_FLAGS_PERMISSIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_INFO_FLAGS_PERMISSIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_INFO_FLAGS_PERMISSIONS").field(&self.0).finish()
    }
}
impl FILE_INFO_FLAGS_PERMISSIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_NOTIFY_CHANGE(pub u32);
impl windows_core::TypeKind for FILE_NOTIFY_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_NOTIFY_CHANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_NOTIFY_CHANGE").field(&self.0).finish()
    }
}
impl FILE_NOTIFY_CHANGE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_NOTIFY_CHANGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_NOTIFY_CHANGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_SHARE_MODE(pub u32);
impl windows_core::TypeKind for FILE_SHARE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_SHARE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_SHARE_MODE").field(&self.0).finish()
    }
}
impl FILE_SHARE_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_SHARE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_SHARE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_SHARE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_SHARE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_SHARE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_TYPE(pub u32);
impl windows_core::TypeKind for FILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILE_WRITE_FLAGS(pub i32);
impl windows_core::TypeKind for FILE_WRITE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILE_WRITE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILE_WRITE_FLAGS").field(&self.0).finish()
    }
}
impl FILE_WRITE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_WRITE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_WRITE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_WRITE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_WRITE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_WRITE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FINDEX_INFO_LEVELS(pub i32);
impl windows_core::TypeKind for FINDEX_INFO_LEVELS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FINDEX_INFO_LEVELS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FINDEX_INFO_LEVELS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FINDEX_SEARCH_OPS(pub i32);
impl windows_core::TypeKind for FINDEX_SEARCH_OPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FINDEX_SEARCH_OPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FINDEX_SEARCH_OPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FIND_FIRST_EX_FLAGS(pub u32);
impl windows_core::TypeKind for FIND_FIRST_EX_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FIND_FIRST_EX_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FIND_FIRST_EX_FLAGS").field(&self.0).finish()
    }
}
impl FIND_FIRST_EX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FIND_FIRST_EX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FIND_FIRST_EX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GETFINALPATHNAMEBYHANDLE_FLAGS(pub u32);
impl windows_core::TypeKind for GETFINALPATHNAMEBYHANDLE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GETFINALPATHNAMEBYHANDLE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GETFINALPATHNAMEBYHANDLE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_FILEEX_INFO_LEVELS(pub i32);
impl windows_core::TypeKind for GET_FILEEX_INFO_LEVELS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_FILEEX_INFO_LEVELS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_FILEEX_INFO_LEVELS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_FILE_VERSION_INFO_FLAGS(pub u32);
impl windows_core::TypeKind for GET_FILE_VERSION_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_FILE_VERSION_INFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_FILE_VERSION_INFO_FLAGS").field(&self.0).finish()
    }
}
impl GET_FILE_VERSION_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_TAPE_DRIVE_PARAMETERS_OPERATION(pub u32);
impl windows_core::TypeKind for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_TAPE_DRIVE_PARAMETERS_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IORING_CREATE_ADVISORY_FLAGS(pub i32);
impl windows_core::TypeKind for IORING_CREATE_ADVISORY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IORING_CREATE_ADVISORY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IORING_CREATE_ADVISORY_FLAGS").field(&self.0).finish()
    }
}
impl IORING_CREATE_ADVISORY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IORING_CREATE_ADVISORY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IORING_CREATE_ADVISORY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IORING_CREATE_ADVISORY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IORING_CREATE_ADVISORY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IORING_CREATE_ADVISORY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IORING_CREATE_REQUIRED_FLAGS(pub i32);
impl windows_core::TypeKind for IORING_CREATE_REQUIRED_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IORING_CREATE_REQUIRED_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IORING_CREATE_REQUIRED_FLAGS").field(&self.0).finish()
    }
}
impl IORING_CREATE_REQUIRED_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IORING_CREATE_REQUIRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IORING_CREATE_REQUIRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IORING_CREATE_REQUIRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IORING_CREATE_REQUIRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IORING_CREATE_REQUIRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IORING_FEATURE_FLAGS(pub i32);
impl windows_core::TypeKind for IORING_FEATURE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IORING_FEATURE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IORING_FEATURE_FLAGS").field(&self.0).finish()
    }
}
impl IORING_FEATURE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IORING_FEATURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IORING_FEATURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IORING_FEATURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IORING_FEATURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IORING_FEATURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IORING_OP_CODE(pub i32);
impl windows_core::TypeKind for IORING_OP_CODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IORING_OP_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IORING_OP_CODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IORING_REF_KIND(pub i32);
impl windows_core::TypeKind for IORING_REF_KIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IORING_REF_KIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IORING_REF_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IORING_SQE_FLAGS(pub i32);
impl windows_core::TypeKind for IORING_SQE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IORING_SQE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IORING_SQE_FLAGS").field(&self.0).finish()
    }
}
impl IORING_SQE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IORING_SQE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IORING_SQE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IORING_SQE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IORING_SQE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IORING_SQE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IORING_VERSION(pub i32);
impl windows_core::TypeKind for IORING_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IORING_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IORING_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOCK_FILE_FLAGS(pub u32);
impl windows_core::TypeKind for LOCK_FILE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOCK_FILE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOCK_FILE_FLAGS").field(&self.0).finish()
    }
}
impl LOCK_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LOCK_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LOCK_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LOCK_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LPPROGRESS_ROUTINE_CALLBACK_REASON(pub u32);
impl windows_core::TypeKind for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LPPROGRESS_ROUTINE_CALLBACK_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LZOPENFILE_STYLE(pub u16);
impl windows_core::TypeKind for LZOPENFILE_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LZOPENFILE_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LZOPENFILE_STYLE").field(&self.0).finish()
    }
}
impl LZOPENFILE_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LZOPENFILE_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LZOPENFILE_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LZOPENFILE_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MOVE_FILE_FLAGS(pub u32);
impl windows_core::TypeKind for MOVE_FILE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MOVE_FILE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MOVE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl MOVE_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MOVE_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MOVE_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MOVE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NTMS_OMID_TYPE(pub u32);
impl windows_core::TypeKind for NTMS_OMID_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NTMS_OMID_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NTMS_OMID_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsAccessMask(pub i32);
impl windows_core::TypeKind for NtmsAccessMask {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsAccessMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsAccessMask").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsAllocateOptions(pub i32);
impl windows_core::TypeKind for NtmsAllocateOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsAllocateOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsAllocateOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsAllocationPolicy(pub i32);
impl windows_core::TypeKind for NtmsAllocationPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsAllocationPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsAllocationPolicy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsAsyncOperations(pub i32);
impl windows_core::TypeKind for NtmsAsyncOperations {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsAsyncOperations {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsAsyncOperations").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsAsyncStatus(pub i32);
impl windows_core::TypeKind for NtmsAsyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsAsyncStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsAsyncStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsBarCodeState(pub i32);
impl windows_core::TypeKind for NtmsBarCodeState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsBarCodeState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsBarCodeState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsCreateNtmsMediaOptions(pub i32);
impl windows_core::TypeKind for NtmsCreateNtmsMediaOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsCreateNtmsMediaOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsCreateNtmsMediaOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsCreateOptions(pub i32);
impl windows_core::TypeKind for NtmsCreateOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsCreateOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsCreateOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsDeallocationPolicy(pub i32);
impl windows_core::TypeKind for NtmsDeallocationPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsDeallocationPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsDeallocationPolicy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsDismountOptions(pub i32);
impl windows_core::TypeKind for NtmsDismountOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsDismountOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsDismountOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsDoorState(pub i32);
impl windows_core::TypeKind for NtmsDoorState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsDoorState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsDoorState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsDriveState(pub i32);
impl windows_core::TypeKind for NtmsDriveState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsDriveState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsDriveState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsDriveType(pub i32);
impl windows_core::TypeKind for NtmsDriveType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsDriveType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsDriveType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsEjectOperation(pub i32);
impl windows_core::TypeKind for NtmsEjectOperation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsEjectOperation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsEjectOperation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsEnumerateOption(pub i32);
impl windows_core::TypeKind for NtmsEnumerateOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsEnumerateOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsEnumerateOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsInjectOperation(pub i32);
impl windows_core::TypeKind for NtmsInjectOperation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsInjectOperation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsInjectOperation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsInventoryMethod(pub i32);
impl windows_core::TypeKind for NtmsInventoryMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsInventoryMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsInventoryMethod").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsLibRequestFlags(pub i32);
impl windows_core::TypeKind for NtmsLibRequestFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsLibRequestFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsLibRequestFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsLibraryFlags(pub i32);
impl windows_core::TypeKind for NtmsLibraryFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsLibraryFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsLibraryFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsLibraryType(pub i32);
impl windows_core::TypeKind for NtmsLibraryType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsLibraryType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsLibraryType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsLmOperation(pub i32);
impl windows_core::TypeKind for NtmsLmOperation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsLmOperation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsLmOperation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsLmState(pub i32);
impl windows_core::TypeKind for NtmsLmState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsLmState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsLmState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsMediaPoolPolicy(pub i32);
impl windows_core::TypeKind for NtmsMediaPoolPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsMediaPoolPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsMediaPoolPolicy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsMediaState(pub i32);
impl windows_core::TypeKind for NtmsMediaState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsMediaState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsMediaState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsMountOptions(pub i32);
impl windows_core::TypeKind for NtmsMountOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsMountOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsMountOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsMountPriority(pub i32);
impl windows_core::TypeKind for NtmsMountPriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsMountPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsMountPriority").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsNotificationOperations(pub i32);
impl windows_core::TypeKind for NtmsNotificationOperations {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsNotificationOperations {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsNotificationOperations").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsObjectsTypes(pub i32);
impl windows_core::TypeKind for NtmsObjectsTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsObjectsTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsObjectsTypes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsOpRequestFlags(pub i32);
impl windows_core::TypeKind for NtmsOpRequestFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsOpRequestFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsOpRequestFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsOperationalState(pub i32);
impl windows_core::TypeKind for NtmsOperationalState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsOperationalState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsOperationalState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsOpreqCommand(pub i32);
impl windows_core::TypeKind for NtmsOpreqCommand {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsOpreqCommand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsOpreqCommand").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsOpreqState(pub i32);
impl windows_core::TypeKind for NtmsOpreqState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsOpreqState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsOpreqState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsPartitionState(pub i32);
impl windows_core::TypeKind for NtmsPartitionState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsPartitionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsPartitionState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsPoolType(pub i32);
impl windows_core::TypeKind for NtmsPoolType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsPoolType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsPoolType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsPortContent(pub i32);
impl windows_core::TypeKind for NtmsPortContent {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsPortContent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsPortContent").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsPortPosition(pub i32);
impl windows_core::TypeKind for NtmsPortPosition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsPortPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsPortPosition").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsReadWriteCharacteristics(pub i32);
impl windows_core::TypeKind for NtmsReadWriteCharacteristics {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsReadWriteCharacteristics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsReadWriteCharacteristics").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsSessionOptions(pub i32);
impl windows_core::TypeKind for NtmsSessionOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsSessionOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsSessionOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsSlotState(pub i32);
impl windows_core::TypeKind for NtmsSlotState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsSlotState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsSlotState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsUIOperations(pub i32);
impl windows_core::TypeKind for NtmsUIOperations {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsUIOperations {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsUIOperations").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NtmsUITypes(pub i32);
impl windows_core::TypeKind for NtmsUITypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NtmsUITypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NtmsUITypes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PREPARE_TAPE_OPERATION(pub u32);
impl windows_core::TypeKind for PREPARE_TAPE_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PREPARE_TAPE_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PREPARE_TAPE_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRIORITY_HINT(pub i32);
impl windows_core::TypeKind for PRIORITY_HINT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRIORITY_HINT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRIORITY_HINT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("READ_DIRECTORY_NOTIFY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REPLACE_FILE_FLAGS(pub u32);
impl windows_core::TypeKind for REPLACE_FILE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REPLACE_FILE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REPLACE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl REPLACE_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REPLACE_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REPLACE_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVER_CERTIFICATE_TYPE(pub i32);
impl windows_core::TypeKind for SERVER_CERTIFICATE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVER_CERTIFICATE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVER_CERTIFICATE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SESSION_INFO_USER_FLAGS(pub u32);
impl windows_core::TypeKind for SESSION_INFO_USER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SESSION_INFO_USER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SESSION_INFO_USER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SET_FILE_POINTER_MOVE_METHOD(pub u32);
impl windows_core::TypeKind for SET_FILE_POINTER_MOVE_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SET_FILE_POINTER_MOVE_METHOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SET_FILE_POINTER_MOVE_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SHARE_INFO_PERMISSIONS(pub u32);
impl windows_core::TypeKind for SHARE_INFO_PERMISSIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SHARE_INFO_PERMISSIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SHARE_INFO_PERMISSIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SHARE_TYPE(pub u32);
impl windows_core::TypeKind for SHARE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SHARE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SHARE_TYPE").field(&self.0).finish()
    }
}
impl SHARE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SHARE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SHARE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SHARE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SHARE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SHARE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STORAGE_BUS_TYPE(pub i32);
impl windows_core::TypeKind for STORAGE_BUS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STORAGE_BUS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STORAGE_BUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STREAM_INFO_LEVELS(pub i32);
impl windows_core::TypeKind for STREAM_INFO_LEVELS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STREAM_INFO_LEVELS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STREAM_INFO_LEVELS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYMBOLIC_LINK_FLAGS(pub u32);
impl windows_core::TypeKind for SYMBOLIC_LINK_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYMBOLIC_LINK_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYMBOLIC_LINK_FLAGS").field(&self.0).finish()
    }
}
impl SYMBOLIC_LINK_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SYMBOLIC_LINK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SYMBOLIC_LINK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPEMARK_TYPE(pub u32);
impl windows_core::TypeKind for TAPEMARK_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPEMARK_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPEMARK_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPE_INFORMATION_TYPE(pub u32);
impl windows_core::TypeKind for TAPE_INFORMATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPE_INFORMATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPE_INFORMATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPE_POSITION_METHOD(pub u32);
impl windows_core::TypeKind for TAPE_POSITION_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPE_POSITION_METHOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPE_POSITION_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPE_POSITION_TYPE(pub u32);
impl windows_core::TypeKind for TAPE_POSITION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPE_POSITION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPE_POSITION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TRANSACTION_OUTCOME(pub i32);
impl windows_core::TypeKind for TRANSACTION_OUTCOME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TRANSACTION_OUTCOME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TRANSACTION_OUTCOME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TXFS_MINIVERSION(pub u32);
impl windows_core::TypeKind for TXFS_MINIVERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TXFS_MINIVERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TXFS_MINIVERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TXF_LOG_RECORD_TYPE(pub u16);
impl windows_core::TypeKind for TXF_LOG_RECORD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TXF_LOG_RECORD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TXF_LOG_RECORD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VER_FIND_FILE_FLAGS(pub u32);
impl windows_core::TypeKind for VER_FIND_FILE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VER_FIND_FILE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VER_FIND_FILE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VER_FIND_FILE_STATUS(pub u32);
impl windows_core::TypeKind for VER_FIND_FILE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VER_FIND_FILE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VER_FIND_FILE_STATUS").field(&self.0).finish()
    }
}
impl VER_FIND_FILE_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VER_FIND_FILE_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VER_FIND_FILE_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VER_INSTALL_FILE_FLAGS(pub u32);
impl windows_core::TypeKind for VER_INSTALL_FILE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VER_INSTALL_FILE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VER_INSTALL_FILE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VER_INSTALL_FILE_STATUS(pub u32);
impl windows_core::TypeKind for VER_INSTALL_FILE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VER_INSTALL_FILE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VER_INSTALL_FILE_STATUS").field(&self.0).finish()
    }
}
impl VER_INSTALL_FILE_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VER_INSTALL_FILE_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VER_INSTALL_FILE_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VS_FIXEDFILEINFO_FILE_FLAGS(pub u32);
impl windows_core::TypeKind for VS_FIXEDFILEINFO_FILE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_FLAGS").field(&self.0).finish()
    }
}
impl VS_FIXEDFILEINFO_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VS_FIXEDFILEINFO_FILE_OS(pub u32);
impl windows_core::TypeKind for VS_FIXEDFILEINFO_FILE_OS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VS_FIXEDFILEINFO_FILE_OS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_OS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VS_FIXEDFILEINFO_FILE_SUBTYPE(pub i32);
impl windows_core::TypeKind for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_SUBTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VS_FIXEDFILEINFO_FILE_TYPE(pub i32);
impl windows_core::TypeKind for VS_FIXEDFILEINFO_FILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VS_FIXEDFILEINFO_FILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WIN_STREAM_ID(pub u32);
impl windows_core::TypeKind for WIN_STREAM_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WIN_STREAM_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WIN_STREAM_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}
impl windows_core::TypeKind for BY_HANDLE_FILE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for BY_HANDLE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_LOG_NAME_INFORMATION {
    pub NameLengthInBytes: u16,
    pub Name: [u16; 1],
}
impl windows_core::TypeKind for CLFS_LOG_NAME_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_LOG_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_NOTIFICATION {
    pub Notification: CLFS_MGMT_NOTIFICATION_TYPE,
    pub Lsn: CLS_LSN,
    pub LogIsPinned: u16,
}
impl windows_core::TypeKind for CLFS_MGMT_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLFS_MGMT_POLICY {
    pub Version: u32,
    pub LengthInBytes: u32,
    pub PolicyFlags: u32,
    pub PolicyType: CLFS_MGMT_POLICY_TYPE,
    pub PolicyParameters: CLFS_MGMT_POLICY_0,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLFS_MGMT_POLICY_0 {
    pub MaximumSize: CLFS_MGMT_POLICY_0_4,
    pub MinimumSize: CLFS_MGMT_POLICY_0_5,
    pub NewContainerSize: CLFS_MGMT_POLICY_0_8,
    pub GrowthRate: CLFS_MGMT_POLICY_0_2,
    pub LogTail: CLFS_MGMT_POLICY_0_3,
    pub AutoShrink: CLFS_MGMT_POLICY_0_1,
    pub AutoGrow: CLFS_MGMT_POLICY_0_0,
    pub NewContainerPrefix: CLFS_MGMT_POLICY_0_7,
    pub NewContainerSuffix: CLFS_MGMT_POLICY_0_9,
    pub NewContainerExtension: CLFS_MGMT_POLICY_0_6,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_0 {
    pub Enabled: u32,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_1 {
    pub Percentage: u32,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_2 {
    pub AbsoluteGrowthInContainers: u32,
    pub RelativeGrowthPercentage: u32,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_3 {
    pub MinimumAvailablePercentage: u32,
    pub MinimumAvailableContainers: u32,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_4 {
    pub Containers: u32,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_5 {
    pub Containers: u32,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_6 {
    pub ExtensionLengthInBytes: u16,
    pub ExtensionString: [u16; 1],
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_7 {
    pub PrefixLengthInBytes: u16,
    pub PrefixString: [u16; 1],
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_7 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_8 {
    pub SizeInBytes: u32,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_8 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_9 {
    pub NextContainerSuffix: u64,
}
impl windows_core::TypeKind for CLFS_MGMT_POLICY_0_9 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_MGMT_POLICY_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_NODE_ID {
    pub cType: u32,
    pub cbNode: u32,
}
impl windows_core::TypeKind for CLFS_NODE_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_NODE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_PHYSICAL_LSN_INFORMATION {
    pub StreamIdentifier: u8,
    pub VirtualLsn: CLS_LSN,
    pub PhysicalLsn: CLS_LSN,
}
impl windows_core::TypeKind for CLFS_PHYSICAL_LSN_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_PHYSICAL_LSN_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLFS_STREAM_ID_INFORMATION {
    pub StreamIdentifier: u8,
}
impl windows_core::TypeKind for CLFS_STREAM_ID_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLFS_STREAM_ID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLS_ARCHIVE_DESCRIPTOR {
    pub coffLow: u64,
    pub coffHigh: u64,
    pub infoContainer: CLS_CONTAINER_INFORMATION,
}
impl windows_core::TypeKind for CLS_ARCHIVE_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLS_ARCHIVE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLS_CONTAINER_INFORMATION {
    pub FileAttributes: u32,
    pub CreationTime: u64,
    pub LastAccessTime: u64,
    pub LastWriteTime: u64,
    pub ContainerSize: i64,
    pub FileNameActualLength: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 256],
    pub State: u32,
    pub PhysicalContainerId: u32,
    pub LogicalContainerId: u32,
}
impl windows_core::TypeKind for CLS_CONTAINER_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLS_INFORMATION {
    pub TotalAvailable: i64,
    pub CurrentAvailable: i64,
    pub TotalReservation: i64,
    pub BaseFileSize: u64,
    pub ContainerSize: u64,
    pub TotalContainers: u32,
    pub FreeContainers: u32,
    pub TotalClients: u32,
    pub Attributes: u32,
    pub FlushThreshold: u32,
    pub SectorSize: u32,
    pub MinArchiveTailLsn: CLS_LSN,
    pub BaseLsn: CLS_LSN,
    pub LastFlushedLsn: CLS_LSN,
    pub LastLsn: CLS_LSN,
    pub RestartLsn: CLS_LSN,
    pub Identity: windows_core::GUID,
}
impl windows_core::TypeKind for CLS_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLS_IO_STATISTICS {
    pub hdrIoStats: CLS_IO_STATISTICS_HEADER,
    pub cFlush: u64,
    pub cbFlush: u64,
    pub cMetaFlush: u64,
    pub cbMetaFlush: u64,
}
impl windows_core::TypeKind for CLS_IO_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLS_IO_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLS_IO_STATISTICS_HEADER {
    pub ubMajorVersion: u8,
    pub ubMinorVersion: u8,
    pub eStatsClass: CLFS_IOSTATS_CLASS,
    pub cbLength: u16,
    pub coffData: u32,
}
impl windows_core::TypeKind for CLS_IO_STATISTICS_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLS_IO_STATISTICS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLS_LSN {
    pub Internal: u64,
}
impl windows_core::TypeKind for CLS_LSN {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLS_LSN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::super::Foundation::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: u8,
    pub pinfoContainer: *mut CLS_CONTAINER_INFORMATION,
}
impl windows_core::TypeKind for CLS_SCAN_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLS_WRITE_ENTRY {
    pub Buffer: *mut core::ffi::c_void,
    pub ByteLength: u32,
}
impl windows_core::TypeKind for CLS_WRITE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONNECTION_INFO_0 {
    pub coni0_id: u32,
}
impl windows_core::TypeKind for CONNECTION_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONNECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONNECTION_INFO_1 {
    pub coni1_id: u32,
    pub coni1_type: SHARE_TYPE,
    pub coni1_num_opens: u32,
    pub coni1_num_users: u32,
    pub coni1_time: u32,
    pub coni1_username: windows_core::PWSTR,
    pub coni1_netname: windows_core::PWSTR,
}
impl windows_core::TypeKind for CONNECTION_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONNECTION_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut super::super::Foundation::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for COPYFILE2_EXTENDED_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2 {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut super::super::Foundation::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: *mut core::ffi::c_void,
    pub dwCopyFlagsV2: u32,
    pub ioDesiredSize: u32,
    pub ioDesiredRate: u32,
    pub reserved: [*mut core::ffi::c_void; 8],
}
impl windows_core::TypeKind for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE {
    pub Type: COPYFILE2_MESSAGE_TYPE,
    pub dwPadding: u32,
    pub Info: COPYFILE2_MESSAGE_0,
}
impl windows_core::TypeKind for COPYFILE2_MESSAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union COPYFILE2_MESSAGE_0 {
    pub ChunkStarted: COPYFILE2_MESSAGE_0_1,
    pub ChunkFinished: COPYFILE2_MESSAGE_0_0,
    pub StreamStarted: COPYFILE2_MESSAGE_0_5,
    pub StreamFinished: COPYFILE2_MESSAGE_0_4,
    pub PollContinue: COPYFILE2_MESSAGE_0_3,
    pub Error: COPYFILE2_MESSAGE_0_2,
}
impl windows_core::TypeKind for COPYFILE2_MESSAGE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_MESSAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COPYFILE2_MESSAGE_0_0 {
    pub dwStreamNumber: u32,
    pub dwFlags: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
impl windows_core::TypeKind for COPYFILE2_MESSAGE_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_MESSAGE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COPYFILE2_MESSAGE_0_1 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
impl windows_core::TypeKind for COPYFILE2_MESSAGE_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_MESSAGE_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COPYFILE2_MESSAGE_0_2 {
    pub CopyPhase: COPYFILE2_COPY_PHASE,
    pub dwStreamNumber: u32,
    pub hrFailure: windows_core::HRESULT,
    pub dwReserved: u32,
    pub uliChunkNumber: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
impl windows_core::TypeKind for COPYFILE2_MESSAGE_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_MESSAGE_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COPYFILE2_MESSAGE_0_3 {
    pub dwReserved: u32,
}
impl windows_core::TypeKind for COPYFILE2_MESSAGE_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_MESSAGE_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COPYFILE2_MESSAGE_0_4 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
impl windows_core::TypeKind for COPYFILE2_MESSAGE_0_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_MESSAGE_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COPYFILE2_MESSAGE_0_5 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
impl windows_core::TypeKind for COPYFILE2_MESSAGE_0_5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYFILE2_MESSAGE_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pub hTemplateFile: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for CREATEFILE2_EXTENDED_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for CREATEFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DISKQUOTA_USER_INFORMATION {
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
}
impl windows_core::TypeKind for DISKQUOTA_USER_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DISKQUOTA_USER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DISK_SPACE_INFORMATION {
    pub ActualTotalAllocationUnits: u64,
    pub ActualAvailableAllocationUnits: u64,
    pub ActualPoolUnavailableAllocationUnits: u64,
    pub CallerTotalAllocationUnits: u64,
    pub CallerAvailableAllocationUnits: u64,
    pub CallerPoolUnavailableAllocationUnits: u64,
    pub UsedAllocationUnits: u64,
    pub TotalReservedAllocationUnits: u64,
    pub VolumeStorageReserveAllocationUnits: u64,
    pub AvailableCommittedAllocationUnits: u64,
    pub PoolAvailableAllocationUnits: u64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl windows_core::TypeKind for DISK_SPACE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DISK_SPACE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: u32,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl windows_core::TypeKind for EFS_CERTIFICATE_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFS_CERTIFICATE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: u32,
}
impl windows_core::TypeKind for EFS_COMPATIBILITY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFS_COMPATIBILITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: u32,
    pub dwHashOffset: u32,
    pub cbHash: u32,
}
impl windows_core::TypeKind for EFS_DECRYPTION_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFS_DECRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: super::super::Foundation::BOOL,
    pub dwEncryptionError: u32,
}
impl windows_core::TypeKind for EFS_ENCRYPTION_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFS_ENCRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_HASH_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl windows_core::TypeKind for EFS_HASH_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFS_HASH_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_KEY_INFO {
    pub dwVersion: u32,
    pub Entropy: u32,
    pub Algorithm: super::super::Security::Cryptography::ALG_ID,
    pub KeyLength: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for EFS_KEY_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for EFS_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_PIN_BLOB {
    pub cbPadding: u32,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl windows_core::TypeKind for EFS_PIN_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFS_PIN_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_RPC_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl windows_core::TypeKind for EFS_RPC_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFS_RPC_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFS_VERSION_INFO {
    pub EfsVersion: u32,
    pub SubVersion: u32,
}
impl windows_core::TypeKind for EFS_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFS_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: u32,
    pub pCertificatesAdded: *mut ENCRYPTION_CERTIFICATE_HASH_LIST,
    pub pEncryptionCertificate: *mut ENCRYPTION_CERTIFICATE,
    pub pEfsStreamSignature: *mut EFS_RPC_BLOB,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for ENCRYPTED_FILE_METADATA_SIGNATURE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub pCertBlob: *mut EFS_CERTIFICATE_BLOB,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for ENCRYPTION_CERTIFICATE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for ENCRYPTION_CERTIFICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub pHash: *mut EFS_HASH_BLOB,
    pub lpDisplayInformation: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for ENCRYPTION_CERTIFICATE_HASH {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for ENCRYPTION_CERTIFICATE_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: u32,
    pub pUsers: *mut *mut ENCRYPTION_CERTIFICATE_HASH,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for ENCRYPTION_CERTIFICATE_HASH_LIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: u32,
    pub pUsers: *mut *mut ENCRYPTION_CERTIFICATE,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for ENCRYPTION_CERTIFICATE_LIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for ENCRYPTION_CERTIFICATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub lpProtectorDescriptor: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for ENCRYPTION_PROTECTOR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for ENCRYPTION_PROTECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTION_PROTECTOR_LIST {
    pub nProtectors: u32,
    pub pProtectors: *mut *mut ENCRYPTION_PROTECTOR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for ENCRYPTION_PROTECTOR_LIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for ENCRYPTION_PROTECTOR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FH_OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Offset: u32,
    pub OffsetHigh: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub pfnCompletion: PFN_IO_COMPLETION,
    pub Reserved1: usize,
    pub Reserved2: usize,
    pub Reserved3: usize,
    pub Reserved4: usize,
}
impl windows_core::TypeKind for FH_OVERLAPPED {
    type TypeKind = windows_core::CopyType;
}
impl Default for FH_OVERLAPPED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ALIGNMENT_INFO {
    pub AlignmentRequirement: u32,
}
impl windows_core::TypeKind for FILE_ALIGNMENT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ALIGNMENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ALLOCATION_INFO {
    pub AllocationSize: i64,
}
impl windows_core::TypeKind for FILE_ALLOCATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ALLOCATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ATTRIBUTE_TAG_INFO {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
impl windows_core::TypeKind for FILE_ATTRIBUTE_TAG_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ATTRIBUTE_TAG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_BASIC_INFO {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl windows_core::TypeKind for FILE_BASIC_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_BASIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_COMPRESSION_INFO {
    pub CompressedFileSize: i64,
    pub CompressionFormat: COMPRESSION_FORMAT,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl windows_core::TypeKind for FILE_COMPRESSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_DISPOSITION_INFO {
    pub DeleteFile: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for FILE_DISPOSITION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_DISPOSITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_DISPOSITION_INFO_EX {
    pub Flags: FILE_DISPOSITION_INFO_EX_FLAGS,
}
impl windows_core::TypeKind for FILE_DISPOSITION_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_DISPOSITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_END_OF_FILE_INFO {
    pub EndOfFile: i64,
}
impl windows_core::TypeKind for FILE_END_OF_FILE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_END_OF_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_EXTENT {
    pub VolumeOffset: u64,
    pub ExtentLength: u64,
}
impl windows_core::TypeKind for FILE_EXTENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FULL_DIR_INFO {
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
impl windows_core::TypeKind for FILE_FULL_DIR_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_FULL_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_128 {
    pub Identifier: [u8; 16],
}
impl windows_core::TypeKind for FILE_ID_128 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_BOTH_DIR_INFO {
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
impl windows_core::TypeKind for FILE_ID_BOTH_DIR_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_BOTH_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_ID_DESCRIPTOR {
    pub dwSize: u32,
    pub Type: FILE_ID_TYPE,
    pub Anonymous: FILE_ID_DESCRIPTOR_0,
}
impl windows_core::TypeKind for FILE_ID_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_ID_DESCRIPTOR_0 {
    pub FileId: i64,
    pub ObjectId: windows_core::GUID,
    pub ExtendedFileId: FILE_ID_128,
}
impl windows_core::TypeKind for FILE_ID_DESCRIPTOR_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_EXTD_DIR_INFO {
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
    pub FileId: FILE_ID_128,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_ID_EXTD_DIR_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_EXTD_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_ID_INFO {
    pub VolumeSerialNumber: u64,
    pub FileId: FILE_ID_128,
}
impl windows_core::TypeKind for FILE_ID_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_ID_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_INFO_2 {
    pub fi2_id: u32,
}
impl windows_core::TypeKind for FILE_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_INFO_3 {
    pub fi3_id: u32,
    pub fi3_permissions: FILE_INFO_FLAGS_PERMISSIONS,
    pub fi3_num_locks: u32,
    pub fi3_pathname: windows_core::PWSTR,
    pub fi3_username: windows_core::PWSTR,
}
impl windows_core::TypeKind for FILE_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_IO_PRIORITY_HINT_INFO {
    pub PriorityHint: PRIORITY_HINT,
}
impl windows_core::TypeKind for FILE_IO_PRIORITY_HINT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_IO_PRIORITY_HINT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NAME_INFO {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_NAME_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_NAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NOTIFY_EXTENDED_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub Anonymous: FILE_NOTIFY_EXTENDED_INFORMATION_0,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_NOTIFY_EXTENDED_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    pub ReparsePointTag: u32,
    pub EaSize: u32,
}
impl windows_core::TypeKind for FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_NOTIFY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_NOTIFY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFO {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFO_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFO_1,
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFO_0 {
    pub Reserved: [u32; 8],
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_REMOTE_PROTOCOL_INFO_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFO_1_0,
    pub Reserved: [u32; 16],
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFO_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFO_1_0_1,
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    pub Capabilities: u32,
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    pub Capabilities: u32,
    pub ShareFlags: u32,
}
impl windows_core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_RENAME_INFO {
    pub Anonymous: FILE_RENAME_INFO_0,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl windows_core::TypeKind for FILE_RENAME_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_RENAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_RENAME_INFO_0 {
    pub ReplaceIfExists: super::super::Foundation::BOOLEAN,
    pub Flags: u32,
}
impl windows_core::TypeKind for FILE_RENAME_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_RENAME_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_SEGMENT_ELEMENT {
    pub Buffer: *mut core::ffi::c_void,
    pub Alignment: u64,
}
impl windows_core::TypeKind for FILE_SEGMENT_ELEMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_SEGMENT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STANDARD_INFO {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: super::super::Foundation::BOOLEAN,
    pub Directory: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for FILE_STANDARD_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_STANDARD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STORAGE_INFO {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
impl windows_core::TypeKind for FILE_STORAGE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_STORAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STREAM_INFO {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl windows_core::TypeKind for FILE_STREAM_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILE_STREAM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FIO_CONTEXT {
    pub m_dwTempHack: u32,
    pub m_dwSignature: u32,
    pub m_hFile: super::super::Foundation::HANDLE,
    pub m_dwLinesOffset: u32,
    pub m_dwHeaderLength: u32,
}
impl windows_core::TypeKind for FIO_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FIO_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIORING(pub *mut core::ffi::c_void);
impl HIORING {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HIORING {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = CloseIoRing(*self);
        }
    }
}
impl Default for HIORING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HIORING {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IORING_BUFFER_INFO {
    pub Address: *mut core::ffi::c_void,
    pub Length: u32,
}
impl windows_core::TypeKind for IORING_BUFFER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_BUFFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IORING_BUFFER_REF {
    pub Kind: IORING_REF_KIND,
    pub Buffer: IORING_BUFFER_REF_0,
}
impl windows_core::TypeKind for IORING_BUFFER_REF {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_BUFFER_REF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IORING_BUFFER_REF_0 {
    pub Address: *mut core::ffi::c_void,
    pub IndexAndOffset: IORING_REGISTERED_BUFFER,
}
impl windows_core::TypeKind for IORING_BUFFER_REF_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_BUFFER_REF_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IORING_CAPABILITIES {
    pub MaxVersion: IORING_VERSION,
    pub MaxSubmissionQueueSize: u32,
    pub MaxCompletionQueueSize: u32,
    pub FeatureFlags: IORING_FEATURE_FLAGS,
}
impl windows_core::TypeKind for IORING_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IORING_CQE {
    pub UserData: usize,
    pub ResultCode: windows_core::HRESULT,
    pub Information: usize,
}
impl windows_core::TypeKind for IORING_CQE {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_CQE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IORING_CREATE_FLAGS {
    pub Required: IORING_CREATE_REQUIRED_FLAGS,
    pub Advisory: IORING_CREATE_ADVISORY_FLAGS,
}
impl windows_core::TypeKind for IORING_CREATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_CREATE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IORING_HANDLE_REF {
    pub Kind: IORING_REF_KIND,
    pub Handle: IORING_HANDLE_REF_0,
}
impl windows_core::TypeKind for IORING_HANDLE_REF {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_HANDLE_REF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IORING_HANDLE_REF_0 {
    pub Handle: super::super::Foundation::HANDLE,
    pub Index: u32,
}
impl windows_core::TypeKind for IORING_HANDLE_REF_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_HANDLE_REF_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IORING_INFO {
    pub IoRingVersion: IORING_VERSION,
    pub Flags: IORING_CREATE_FLAGS,
    pub SubmissionQueueSize: u32,
    pub CompletionQueueSize: u32,
}
impl windows_core::TypeKind for IORING_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IORING_REGISTERED_BUFFER {
    pub BufferIndex: u32,
    pub Offset: u32,
}
impl windows_core::TypeKind for IORING_REGISTERED_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for IORING_REGISTERED_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KCRM_MARSHAL_HEADER {
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub NumProtocols: u32,
    pub Unused: u32,
}
impl windows_core::TypeKind for KCRM_MARSHAL_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for KCRM_MARSHAL_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KCRM_PROTOCOL_BLOB {
    pub ProtocolId: windows_core::GUID,
    pub StaticInfoLength: u32,
    pub TransactionIdInfoLength: u32,
    pub Unused1: u32,
    pub Unused2: u32,
}
impl windows_core::TypeKind for KCRM_PROTOCOL_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for KCRM_PROTOCOL_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KCRM_TRANSACTION_BLOB {
    pub UOW: windows_core::GUID,
    pub TmIdentity: windows_core::GUID,
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: u32,
    pub Description: [u16; 64],
}
impl windows_core::TypeKind for KCRM_TRANSACTION_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for KCRM_TRANSACTION_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct LOG_MANAGEMENT_CALLBACKS {
    pub CallbackContext: *mut core::ffi::c_void,
    pub AdvanceTailCallback: PLOG_TAIL_ADVANCE_CALLBACK,
    pub LogFullHandlerCallback: PLOG_FULL_HANDLER_CALLBACK,
    pub LogUnpinnedCallback: PLOG_UNPINNED_CALLBACK,
}
impl windows_core::TypeKind for LOG_MANAGEMENT_CALLBACKS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOG_MANAGEMENT_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MediaLabelInfo {
    pub LabelType: [u16; 64],
    pub LabelIDSize: u32,
    pub LabelID: [u8; 256],
    pub LabelAppDescr: [u16; 256],
}
impl windows_core::TypeKind for MediaLabelInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for MediaLabelInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NAME_CACHE_CONTEXT {
    pub m_dwSignature: u32,
}
impl windows_core::TypeKind for NAME_CACHE_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for NAME_CACHE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_ALLOCATION_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: *mut core::ffi::c_void,
    pub AllocatedFrom: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_ALLOCATION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_ASYNC_IO {
    pub OperationId: windows_core::GUID,
    pub EventId: windows_core::GUID,
    pub dwOperationType: u32,
    pub dwResult: u32,
    pub dwAsyncState: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub bOnStateChange: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for NTMS_ASYNC_IO {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_ASYNC_IO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_CHANGERINFORMATIONA {
    pub Number: u32,
    pub ChangerType: windows_core::GUID,
    pub szSerialNumber: [i8; 32],
    pub szRevision: [i8; 32],
    pub szDeviceName: [i8; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_CHANGERINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_CHANGERINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_CHANGERINFORMATIONW {
    pub Number: u32,
    pub ChangerType: windows_core::GUID,
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub szDeviceName: [u16; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_CHANGERINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_CHANGERINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_CHANGERTYPEINFORMATIONA {
    pub szVendor: [i8; 128],
    pub szProduct: [i8; 128],
    pub DeviceType: u32,
}
impl windows_core::TypeKind for NTMS_CHANGERTYPEINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_CHANGERTYPEINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_CHANGERTYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub DeviceType: u32,
}
impl windows_core::TypeKind for NTMS_CHANGERTYPEINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_CHANGERTYPEINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_COMPUTERINFORMATION {
    pub dwLibRequestPurgeTime: u32,
    pub dwOpRequestPurgeTime: u32,
    pub dwLibRequestFlags: u32,
    pub dwOpRequestFlags: u32,
    pub dwMediaPoolPolicy: u32,
}
impl windows_core::TypeKind for NTMS_COMPUTERINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_COMPUTERINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_DRIVEINFORMATIONA {
    pub Number: u32,
    pub State: u32,
    pub DriveType: windows_core::GUID,
    pub szDeviceName: [i8; 64],
    pub szSerialNumber: [i8; 32],
    pub szRevision: [i8; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: windows_core::GUID,
    pub Library: windows_core::GUID,
    pub Reserved: windows_core::GUID,
    pub dwDeferDismountDelay: u32,
}
impl windows_core::TypeKind for NTMS_DRIVEINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_DRIVEINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_DRIVEINFORMATIONW {
    pub Number: u32,
    pub State: u32,
    pub DriveType: windows_core::GUID,
    pub szDeviceName: [u16; 64],
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: windows_core::GUID,
    pub Library: windows_core::GUID,
    pub Reserved: windows_core::GUID,
    pub dwDeferDismountDelay: u32,
}
impl windows_core::TypeKind for NTMS_DRIVEINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_DRIVEINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_DRIVETYPEINFORMATIONA {
    pub szVendor: [i8; 128],
    pub szProduct: [i8; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl windows_core::TypeKind for NTMS_DRIVETYPEINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_DRIVETYPEINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_DRIVETYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl windows_core::TypeKind for NTMS_DRIVETYPEINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_DRIVETYPEINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_FILESYSTEM_INFO {
    pub FileSystemType: [u16; 64],
    pub VolumeName: [u16; 256],
    pub SerialNumber: u32,
}
impl windows_core::TypeKind for NTMS_FILESYSTEM_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_FILESYSTEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_LIBRARYINFORMATION {
    pub LibraryType: u32,
    pub CleanerSlot: windows_core::GUID,
    pub CleanerSlotDefault: windows_core::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: u32,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_I1_LIBRARYINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_LIBRARYINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_LIBREQUESTINFORMATIONA {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: windows_core::GUID,
    pub DriveId: windows_core::GUID,
    pub PhysMediaId: windows_core::GUID,
    pub Library: windows_core::GUID,
    pub SlotId: windows_core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [i8; 64],
    pub szUser: [i8; 64],
    pub szComputer: [i8; 64],
}
impl windows_core::TypeKind for NTMS_I1_LIBREQUESTINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_LIBREQUESTINFORMATIONW {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: windows_core::GUID,
    pub DriveId: windows_core::GUID,
    pub PhysMediaId: windows_core::GUID,
    pub Library: windows_core::GUID,
    pub SlotId: windows_core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
impl windows_core::TypeKind for NTMS_I1_LIBREQUESTINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NTMS_I1_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: windows_core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [i8; 64],
    pub szDescription: [i8; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONA_0,
}
impl windows_core::TypeKind for NTMS_I1_OBJECTINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_OBJECTINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NTMS_I1_OBJECTINFORMATIONA_0 {
    pub Drive: NTMS_DRIVEINFORMATIONA,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONA,
    pub Library: NTMS_I1_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONA,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONA,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_I1_PMIDINFORMATIONA,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_I1_PARTITIONINFORMATIONA,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_I1_LIBREQUESTINFORMATIONA,
    pub OpRequest: NTMS_I1_OPREQUESTINFORMATIONA,
}
impl windows_core::TypeKind for NTMS_I1_OBJECTINFORMATIONA_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NTMS_I1_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: windows_core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONW_0,
}
impl windows_core::TypeKind for NTMS_I1_OBJECTINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_OBJECTINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NTMS_I1_OBJECTINFORMATIONW_0 {
    pub Drive: NTMS_DRIVEINFORMATIONW,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONW,
    pub Library: NTMS_I1_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONW,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONW,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_I1_PMIDINFORMATIONW,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_I1_PARTITIONINFORMATIONW,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_I1_LIBREQUESTINFORMATIONW,
    pub OpRequest: NTMS_I1_OPREQUESTINFORMATIONW,
}
impl windows_core::TypeKind for NTMS_I1_OBJECTINFORMATIONW_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_OPREQUESTINFORMATIONA {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [i8; 127],
    pub Arg1Type: u32,
    pub Arg1: windows_core::GUID,
    pub Arg2Type: u32,
    pub Arg2: windows_core::GUID,
    pub szApplication: [i8; 64],
    pub szUser: [i8; 64],
    pub szComputer: [i8; 64],
}
impl windows_core::TypeKind for NTMS_I1_OPREQUESTINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_OPREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_OPREQUESTINFORMATIONW {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [u16; 127],
    pub Arg1Type: u32,
    pub Arg1: windows_core::GUID,
    pub Arg2Type: u32,
    pub Arg2: windows_core::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
impl windows_core::TypeKind for NTMS_I1_OPREQUESTINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_OPREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_PARTITIONINFORMATIONA {
    pub PhysicalMedia: windows_core::GUID,
    pub LogicalMedia: windows_core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [i8; 64],
    pub szOmidLabelInfo: [i8; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
impl windows_core::TypeKind for NTMS_I1_PARTITIONINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_PARTITIONINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_PARTITIONINFORMATIONW {
    pub PhysicalMedia: windows_core::GUID,
    pub LogicalMedia: windows_core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
impl windows_core::TypeKind for NTMS_I1_PARTITIONINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_PARTITIONINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_PMIDINFORMATIONA {
    pub CurrentLibrary: windows_core::GUID,
    pub MediaPool: windows_core::GUID,
    pub Location: windows_core::GUID,
    pub LocationType: u32,
    pub MediaType: windows_core::GUID,
    pub HomeSlot: windows_core::GUID,
    pub szBarCode: [i8; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [i8; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
impl windows_core::TypeKind for NTMS_I1_PMIDINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_PMIDINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_I1_PMIDINFORMATIONW {
    pub CurrentLibrary: windows_core::GUID,
    pub MediaPool: windows_core::GUID,
    pub Location: windows_core::GUID,
    pub LocationType: u32,
    pub MediaType: windows_core::GUID,
    pub HomeSlot: windows_core::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
impl windows_core::TypeKind for NTMS_I1_PMIDINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_I1_PMIDINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_IEDOORINFORMATION {
    pub Number: u32,
    pub State: u32,
    pub MaxOpenSecs: u16,
    pub Library: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_IEDOORINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_IEDOORINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_IEPORTINFORMATION {
    pub Number: u32,
    pub Content: u32,
    pub Position: u32,
    pub MaxExtendSecs: u16,
    pub Library: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_IEPORTINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_IEPORTINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_LIBRARYINFORMATION {
    pub LibraryType: u32,
    pub CleanerSlot: windows_core::GUID,
    pub CleanerSlotDefault: windows_core::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: u32,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: windows_core::GUID,
    pub AutoRecovery: super::super::Foundation::BOOL,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for NTMS_LIBRARYINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_LIBRARYINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_LIBREQUESTINFORMATIONA {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: windows_core::GUID,
    pub DriveId: windows_core::GUID,
    pub PhysMediaId: windows_core::GUID,
    pub Library: windows_core::GUID,
    pub SlotId: windows_core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [i8; 64],
    pub szUser: [i8; 64],
    pub szComputer: [i8; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: windows_core::GUID,
    pub dwPriority: u32,
}
impl windows_core::TypeKind for NTMS_LIBREQUESTINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_LIBREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_LIBREQUESTINFORMATIONW {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: windows_core::GUID,
    pub DriveId: windows_core::GUID,
    pub PhysMediaId: windows_core::GUID,
    pub Library: windows_core::GUID,
    pub SlotId: windows_core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: windows_core::GUID,
    pub dwPriority: u32,
}
impl windows_core::TypeKind for NTMS_LIBREQUESTINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_LIBREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_LMIDINFORMATION {
    pub MediaPool: windows_core::GUID,
    pub dwNumberOfPartitions: u32,
}
impl windows_core::TypeKind for NTMS_LMIDINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_LMIDINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_MEDIAPOOLINFORMATION {
    pub PoolType: u32,
    pub MediaType: windows_core::GUID,
    pub Parent: windows_core::GUID,
    pub AllocationPolicy: u32,
    pub DeallocationPolicy: u32,
    pub dwMaxAllocates: u32,
    pub dwNumberOfPhysicalMedia: u32,
    pub dwNumberOfLogicalMedia: u32,
    pub dwNumberOfMediaPools: u32,
}
impl windows_core::TypeKind for NTMS_MEDIAPOOLINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_MEDIAPOOLINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_MEDIATYPEINFORMATION {
    pub MediaType: u32,
    pub NumberOfSides: u32,
    pub ReadWriteCharacteristics: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl windows_core::TypeKind for NTMS_MEDIATYPEINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_MEDIATYPEINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_MOUNT_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for NTMS_MOUNT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_MOUNT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_NOTIFICATIONINFORMATION {
    pub dwOperation: u32,
    pub ObjectId: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_NOTIFICATIONINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_NOTIFICATIONINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NTMS_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: windows_core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [i8; 64],
    pub szDescription: [i8; 127],
    pub Info: NTMS_OBJECTINFORMATIONA_0,
}
impl windows_core::TypeKind for NTMS_OBJECTINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_OBJECTINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NTMS_OBJECTINFORMATIONA_0 {
    pub Drive: NTMS_DRIVEINFORMATIONA,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONA,
    pub Library: NTMS_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONA,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONA,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_PMIDINFORMATIONA,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_PARTITIONINFORMATIONA,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_LIBREQUESTINFORMATIONA,
    pub OpRequest: NTMS_OPREQUESTINFORMATIONA,
    pub Computer: NTMS_COMPUTERINFORMATION,
}
impl windows_core::TypeKind for NTMS_OBJECTINFORMATIONA_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_OBJECTINFORMATIONA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NTMS_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: windows_core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_OBJECTINFORMATIONW_0,
}
impl windows_core::TypeKind for NTMS_OBJECTINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_OBJECTINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NTMS_OBJECTINFORMATIONW_0 {
    pub Drive: NTMS_DRIVEINFORMATIONW,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONW,
    pub Library: NTMS_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONW,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONW,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_PMIDINFORMATIONW,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_PARTITIONINFORMATIONW,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_LIBREQUESTINFORMATIONW,
    pub OpRequest: NTMS_OPREQUESTINFORMATIONW,
    pub Computer: NTMS_COMPUTERINFORMATION,
}
impl windows_core::TypeKind for NTMS_OBJECTINFORMATIONW_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_OBJECTINFORMATIONW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_OPREQUESTINFORMATIONA {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [i8; 256],
    pub Arg1Type: u32,
    pub Arg1: windows_core::GUID,
    pub Arg2Type: u32,
    pub Arg2: windows_core::GUID,
    pub szApplication: [i8; 64],
    pub szUser: [i8; 64],
    pub szComputer: [i8; 64],
}
impl windows_core::TypeKind for NTMS_OPREQUESTINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_OPREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_OPREQUESTINFORMATIONW {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [u16; 256],
    pub Arg1Type: u32,
    pub Arg1: windows_core::GUID,
    pub Arg2Type: u32,
    pub Arg2: windows_core::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
impl windows_core::TypeKind for NTMS_OPREQUESTINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_OPREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_PARTITIONINFORMATIONA {
    pub PhysicalMedia: windows_core::GUID,
    pub LogicalMedia: windows_core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [i8; 64],
    pub szOmidLabelInfo: [i8; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
impl windows_core::TypeKind for NTMS_PARTITIONINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_PARTITIONINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_PARTITIONINFORMATIONW {
    pub PhysicalMedia: windows_core::GUID,
    pub LogicalMedia: windows_core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
impl windows_core::TypeKind for NTMS_PARTITIONINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_PARTITIONINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_PMIDINFORMATIONA {
    pub CurrentLibrary: windows_core::GUID,
    pub MediaPool: windows_core::GUID,
    pub Location: windows_core::GUID,
    pub LocationType: u32,
    pub MediaType: windows_core::GUID,
    pub HomeSlot: windows_core::GUID,
    pub szBarCode: [i8; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [i8; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_PMIDINFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_PMIDINFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_PMIDINFORMATIONW {
    pub CurrentLibrary: windows_core::GUID,
    pub MediaPool: windows_core::GUID,
    pub Location: windows_core::GUID,
    pub LocationType: u32,
    pub MediaType: windows_core::GUID,
    pub HomeSlot: windows_core::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_PMIDINFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_PMIDINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTMS_STORAGESLOTINFORMATION {
    pub Number: u32,
    pub State: u32,
    pub Library: windows_core::GUID,
}
impl windows_core::TypeKind for NTMS_STORAGESLOTINFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTMS_STORAGESLOTINFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OFSTRUCT {
    pub cBytes: u8,
    pub fFixedDisk: u8,
    pub nErrCode: u16,
    pub Reserved1: u16,
    pub Reserved2: u16,
    pub szPathName: [i8; 128],
}
impl windows_core::TypeKind for OFSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for OFSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPARSE_GUID_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub ReparseGuid: windows_core::GUID,
    pub GenericReparseBuffer: REPARSE_GUID_DATA_BUFFER_0,
}
impl windows_core::TypeKind for REPARSE_GUID_DATA_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPARSE_GUID_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPARSE_GUID_DATA_BUFFER_0 {
    pub DataBuffer: [u8; 1],
}
impl windows_core::TypeKind for REPARSE_GUID_DATA_BUFFER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPARSE_GUID_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_ALIAS_INFO_0 {
    pub srvai0_alias: windows_core::PWSTR,
    pub srvai0_target: windows_core::PWSTR,
    pub srvai0_default: super::super::Foundation::BOOLEAN,
    pub srvai0_reserved: u32,
}
impl windows_core::TypeKind for SERVER_ALIAS_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_ALIAS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_CERTIFICATE_INFO_0 {
    pub srvci0_name: windows_core::PWSTR,
    pub srvci0_subject: windows_core::PWSTR,
    pub srvci0_issuer: windows_core::PWSTR,
    pub srvci0_thumbprint: windows_core::PWSTR,
    pub srvci0_friendlyname: windows_core::PWSTR,
    pub srvci0_notbefore: windows_core::PWSTR,
    pub srvci0_notafter: windows_core::PWSTR,
    pub srvci0_storelocation: windows_core::PWSTR,
    pub srvci0_storename: windows_core::PWSTR,
    pub srvci0_renewalchain: windows_core::PWSTR,
    pub srvci0_type: u32,
    pub srvci0_flags: u32,
    pub srvci0_mapping_status: u32,
}
impl windows_core::TypeKind for SERVER_CERTIFICATE_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_CERTIFICATE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SESSION_INFO_0 {
    pub sesi0_cname: windows_core::PWSTR,
}
impl windows_core::TypeKind for SESSION_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SESSION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SESSION_INFO_1 {
    pub sesi1_cname: windows_core::PWSTR,
    pub sesi1_username: windows_core::PWSTR,
    pub sesi1_num_opens: u32,
    pub sesi1_time: u32,
    pub sesi1_idle_time: u32,
    pub sesi1_user_flags: SESSION_INFO_USER_FLAGS,
}
impl windows_core::TypeKind for SESSION_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SESSION_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SESSION_INFO_10 {
    pub sesi10_cname: windows_core::PWSTR,
    pub sesi10_username: windows_core::PWSTR,
    pub sesi10_time: u32,
    pub sesi10_idle_time: u32,
}
impl windows_core::TypeKind for SESSION_INFO_10 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SESSION_INFO_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SESSION_INFO_2 {
    pub sesi2_cname: windows_core::PWSTR,
    pub sesi2_username: windows_core::PWSTR,
    pub sesi2_num_opens: u32,
    pub sesi2_time: u32,
    pub sesi2_idle_time: u32,
    pub sesi2_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi2_cltype_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for SESSION_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SESSION_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SESSION_INFO_502 {
    pub sesi502_cname: windows_core::PWSTR,
    pub sesi502_username: windows_core::PWSTR,
    pub sesi502_num_opens: u32,
    pub sesi502_time: u32,
    pub sesi502_idle_time: u32,
    pub sesi502_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi502_cltype_name: windows_core::PWSTR,
    pub sesi502_transport: windows_core::PWSTR,
}
impl windows_core::TypeKind for SESSION_INFO_502 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SESSION_INFO_502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_0 {
    pub shi0_netname: windows_core::PWSTR,
}
impl windows_core::TypeKind for SHARE_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHARE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_1 {
    pub shi1_netname: windows_core::PWSTR,
    pub shi1_type: SHARE_TYPE,
    pub shi1_remark: windows_core::PWSTR,
}
impl windows_core::TypeKind for SHARE_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHARE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_1004 {
    pub shi1004_remark: windows_core::PWSTR,
}
impl windows_core::TypeKind for SHARE_INFO_1004 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHARE_INFO_1004 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_1005 {
    pub shi1005_flags: u32,
}
impl windows_core::TypeKind for SHARE_INFO_1005 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHARE_INFO_1005 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_1006 {
    pub shi1006_max_uses: u32,
}
impl windows_core::TypeKind for SHARE_INFO_1006 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHARE_INFO_1006 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_1501 {
    pub shi1501_reserved: u32,
    pub shi1501_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SHARE_INFO_1501 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for SHARE_INFO_1501 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_1503 {
    pub shi1503_sharefilter: windows_core::GUID,
}
impl windows_core::TypeKind for SHARE_INFO_1503 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHARE_INFO_1503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_2 {
    pub shi2_netname: windows_core::PWSTR,
    pub shi2_type: SHARE_TYPE,
    pub shi2_remark: windows_core::PWSTR,
    pub shi2_permissions: SHARE_INFO_PERMISSIONS,
    pub shi2_max_uses: u32,
    pub shi2_current_uses: u32,
    pub shi2_path: windows_core::PWSTR,
    pub shi2_passwd: windows_core::PWSTR,
}
impl windows_core::TypeKind for SHARE_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHARE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_501 {
    pub shi501_netname: windows_core::PWSTR,
    pub shi501_type: SHARE_TYPE,
    pub shi501_remark: windows_core::PWSTR,
    pub shi501_flags: u32,
}
impl windows_core::TypeKind for SHARE_INFO_501 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHARE_INFO_501 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_502 {
    pub shi502_netname: windows_core::PWSTR,
    pub shi502_type: SHARE_TYPE,
    pub shi502_remark: windows_core::PWSTR,
    pub shi502_permissions: SHARE_INFO_PERMISSIONS,
    pub shi502_max_uses: u32,
    pub shi502_current_uses: u32,
    pub shi502_path: windows_core::PWSTR,
    pub shi502_passwd: windows_core::PWSTR,
    pub shi502_reserved: u32,
    pub shi502_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SHARE_INFO_502 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for SHARE_INFO_502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHARE_INFO_503 {
    pub shi503_netname: windows_core::PWSTR,
    pub shi503_type: SHARE_TYPE,
    pub shi503_remark: windows_core::PWSTR,
    pub shi503_permissions: SHARE_INFO_PERMISSIONS,
    pub shi503_max_uses: u32,
    pub shi503_current_uses: u32,
    pub shi503_path: windows_core::PWSTR,
    pub shi503_passwd: windows_core::PWSTR,
    pub shi503_servername: windows_core::PWSTR,
    pub shi503_reserved: u32,
    pub shi503_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SHARE_INFO_503 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for SHARE_INFO_503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STAT_SERVER_0 {
    pub sts0_start: u32,
    pub sts0_fopens: u32,
    pub sts0_devopens: u32,
    pub sts0_jobsqueued: u32,
    pub sts0_sopens: u32,
    pub sts0_stimedout: u32,
    pub sts0_serrorout: u32,
    pub sts0_pwerrors: u32,
    pub sts0_permerrors: u32,
    pub sts0_syserrors: u32,
    pub sts0_bytessent_low: u32,
    pub sts0_bytessent_high: u32,
    pub sts0_bytesrcvd_low: u32,
    pub sts0_bytesrcvd_high: u32,
    pub sts0_avresponse: u32,
    pub sts0_reqbufneed: u32,
    pub sts0_bigbufneed: u32,
}
impl windows_core::TypeKind for STAT_SERVER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STAT_SERVER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STAT_WORKSTATION_0 {
    pub StatisticsStartTime: i64,
    pub BytesReceived: i64,
    pub SmbsReceived: i64,
    pub PagingReadBytesRequested: i64,
    pub NonPagingReadBytesRequested: i64,
    pub CacheReadBytesRequested: i64,
    pub NetworkReadBytesRequested: i64,
    pub BytesTransmitted: i64,
    pub SmbsTransmitted: i64,
    pub PagingWriteBytesRequested: i64,
    pub NonPagingWriteBytesRequested: i64,
    pub CacheWriteBytesRequested: i64,
    pub NetworkWriteBytesRequested: i64,
    pub InitiallyFailedOperations: u32,
    pub FailedCompletionOperations: u32,
    pub ReadOperations: u32,
    pub RandomReadOperations: u32,
    pub ReadSmbs: u32,
    pub LargeReadSmbs: u32,
    pub SmallReadSmbs: u32,
    pub WriteOperations: u32,
    pub RandomWriteOperations: u32,
    pub WriteSmbs: u32,
    pub LargeWriteSmbs: u32,
    pub SmallWriteSmbs: u32,
    pub RawReadsDenied: u32,
    pub RawWritesDenied: u32,
    pub NetworkErrors: u32,
    pub Sessions: u32,
    pub FailedSessions: u32,
    pub Reconnects: u32,
    pub CoreConnects: u32,
    pub Lanman20Connects: u32,
    pub Lanman21Connects: u32,
    pub LanmanNtConnects: u32,
    pub ServerDisconnects: u32,
    pub HungSessions: u32,
    pub UseCount: u32,
    pub FailedUseCount: u32,
    pub CurrentCommands: u32,
}
impl windows_core::TypeKind for STAT_WORKSTATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STAT_WORKSTATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TAPE_ERASE {
    pub Type: ERASE_TAPE_TYPE,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for TAPE_ERASE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAPE_ERASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TAPE_GET_POSITION {
    pub Type: TAPE_POSITION_TYPE,
    pub Partition: u32,
    pub Offset: i64,
}
impl windows_core::TypeKind for TAPE_GET_POSITION {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAPE_GET_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TAPE_PREPARE {
    pub Operation: PREPARE_TAPE_OPERATION,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for TAPE_PREPARE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAPE_PREPARE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TAPE_SET_POSITION {
    pub Method: TAPE_POSITION_METHOD,
    pub Partition: u32,
    pub Offset: i64,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for TAPE_SET_POSITION {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAPE_SET_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TAPE_WRITE_MARKS {
    pub Type: TAPEMARK_TYPE,
    pub Count: u32,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for TAPE_WRITE_MARKS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAPE_WRITE_MARKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION {
    pub TransactionKey: *mut core::ffi::c_void,
    pub TransactionNotification: u32,
    pub TmVirtualClock: i64,
    pub ArgumentLength: u32,
}
impl windows_core::TypeKind for TRANSACTION_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSACTION_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    pub MarshalCookie: u32,
    pub UOW: windows_core::GUID,
}
impl windows_core::TypeKind for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    pub PropagationCookie: u32,
    pub UOW: windows_core::GUID,
    pub TmIdentity: windows_core::GUID,
    pub BufferLength: u32,
}
impl windows_core::TypeKind for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    pub EnlistmentId: windows_core::GUID,
    pub UOW: windows_core::GUID,
}
impl windows_core::TypeKind for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    pub SavepointId: u32,
}
impl windows_core::TypeKind for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    pub TmIdentity: windows_core::GUID,
    pub Flags: u32,
}
impl windows_core::TypeKind for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct TXF_ID {
    pub Anonymous: TXF_ID_0,
}
impl windows_core::TypeKind for TXF_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for TXF_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct TXF_ID_0 {
    pub LowPart: i64,
    pub HighPart: i64,
}
impl windows_core::TypeKind for TXF_ID_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for TXF_ID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct TXF_LOG_RECORD_AFFECTED_FILE {
    pub Version: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_core::GUID,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl windows_core::TypeKind for TXF_LOG_RECORD_AFFECTED_FILE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TXF_LOG_RECORD_AFFECTED_FILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct TXF_LOG_RECORD_BASE {
    pub Version: u16,
    pub RecordType: TXF_LOG_RECORD_TYPE,
    pub RecordLength: u32,
}
impl windows_core::TypeKind for TXF_LOG_RECORD_BASE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TXF_LOG_RECORD_BASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct TXF_LOG_RECORD_TRUNCATE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_core::GUID,
    pub NewFileSize: i64,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl windows_core::TypeKind for TXF_LOG_RECORD_TRUNCATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TXF_LOG_RECORD_TRUNCATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct TXF_LOG_RECORD_WRITE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_core::GUID,
    pub ByteOffsetInFile: i64,
    pub NumBytesWritten: u32,
    pub ByteOffsetInStructure: u32,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl windows_core::TypeKind for TXF_LOG_RECORD_WRITE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TXF_LOG_RECORD_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT {
    pub Version: u32,
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: super::super::Foundation::BOOLEAN,
    pub Discardable: super::super::Foundation::BOOLEAN,
    pub Reserved1: [super::super::Foundation::BOOLEAN; 2],
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl windows_core::TypeKind for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: u64,
    pub NumOutStandingRequests: u32,
}
impl windows_core::TypeKind for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_ALLOCATION_HINT_INPUT {
    pub ClusterSize: u32,
    pub NumberOfClusters: u32,
    pub StartingClusterNumber: i64,
}
impl windows_core::TypeKind for VOLUME_ALLOCATION_HINT_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_ALLOCATION_HINT_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_ALLOCATION_HINT_OUTPUT {
    pub Bitmap: [u32; 1],
}
impl windows_core::TypeKind for VOLUME_ALLOCATION_HINT_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_CRITICAL_IO {
    pub AccessType: u32,
    pub ExtentsCount: u32,
    pub Extents: [FILE_EXTENT; 1],
}
impl windows_core::TypeKind for VOLUME_CRITICAL_IO {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_CRITICAL_IO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_FAILOVER_SET {
    pub NumberOfDisks: u32,
    pub DiskNumbers: [u32; 1],
}
impl windows_core::TypeKind for VOLUME_FAILOVER_SET {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_FAILOVER_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_GET_BC_PROPERTIES_INPUT {
    pub Version: u32,
    pub Reserved1: u32,
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl windows_core::TypeKind for VOLUME_GET_BC_PROPERTIES_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MaximumRequestSize: u64,
    pub EstimatedTimePerRequest: u32,
    pub NumOutStandingRequests: u32,
    pub RequestSize: u64,
}
impl windows_core::TypeKind for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_LOGICAL_OFFSET {
    pub LogicalOffset: i64,
}
impl windows_core::TypeKind for VOLUME_LOGICAL_OFFSET {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_LOGICAL_OFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_NUMBER {
    pub VolumeNumber: u32,
    pub VolumeManagerName: [u16; 8],
}
impl windows_core::TypeKind for VOLUME_NUMBER {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_PHYSICAL_OFFSET {
    pub DiskNumber: u32,
    pub Offset: i64,
}
impl windows_core::TypeKind for VOLUME_PHYSICAL_OFFSET {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_PHYSICAL_OFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_PHYSICAL_OFFSETS {
    pub NumberOfPhysicalOffsets: u32,
    pub PhysicalOffset: [VOLUME_PHYSICAL_OFFSET; 1],
}
impl windows_core::TypeKind for VOLUME_PHYSICAL_OFFSETS {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_PHYSICAL_OFFSETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_READ_PLEX_INPUT {
    pub ByteOffset: i64,
    pub Length: u32,
    pub PlexNumber: u32,
}
impl windows_core::TypeKind for VOLUME_READ_PLEX_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_READ_PLEX_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: u64,
    pub RevertOnClose: super::super::Foundation::BOOLEAN,
    pub ApplyToAllConnectedVolumes: super::super::Foundation::BOOLEAN,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl windows_core::TypeKind for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_SHRINK_INFO {
    pub VolumeSize: u64,
}
impl windows_core::TypeKind for VOLUME_SHRINK_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUME_SHRINK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VS_FIXEDFILEINFO {
    pub dwSignature: u32,
    pub dwStrucVersion: u32,
    pub dwFileVersionMS: u32,
    pub dwFileVersionLS: u32,
    pub dwProductVersionMS: u32,
    pub dwProductVersionLS: u32,
    pub dwFileFlagsMask: u32,
    pub dwFileFlags: VS_FIXEDFILEINFO_FILE_FLAGS,
    pub dwFileOS: VS_FIXEDFILEINFO_FILE_OS,
    pub dwFileType: u32,
    pub dwFileSubtype: u32,
    pub dwFileDateMS: u32,
    pub dwFileDateLS: u32,
}
impl windows_core::TypeKind for VS_FIXEDFILEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for VS_FIXEDFILEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIM_ENTRY_INFO {
    pub WimEntryInfoSize: u32,
    pub WimType: u32,
    pub DataSourceId: i64,
    pub WimGuid: windows_core::GUID,
    pub WimPath: windows_core::PCWSTR,
    pub WimIndex: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for WIM_ENTRY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WIM_ENTRY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIM_EXTERNAL_FILE_INFO {
    pub DataSourceId: i64,
    pub ResourceHash: [u8; 20],
    pub Flags: u32,
}
impl windows_core::TypeKind for WIM_EXTERNAL_FILE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WIM_EXTERNAL_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
}
impl windows_core::TypeKind for WIN32_FILE_ATTRIBUTE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WIN32_FILE_ATTRIBUTE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN32_FIND_DATAA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [i8; 260],
    pub cAlternateFileName: [i8; 14],
}
impl windows_core::TypeKind for WIN32_FIND_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WIN32_FIND_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [u16; 260],
    pub cAlternateFileName: [u16; 14],
}
impl windows_core::TypeKind for WIN32_FIND_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WIN32_FIND_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN32_FIND_STREAM_DATA {
    pub StreamSize: i64,
    pub cStreamName: [u16; 296],
}
impl windows_core::TypeKind for WIN32_FIND_STREAM_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WIN32_FIND_STREAM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN32_STREAM_ID {
    pub dwStreamId: WIN_STREAM_ID,
    pub dwStreamAttributes: u32,
    pub Size: i64,
    pub dwStreamNameSize: u32,
    pub cStreamName: [u16; 1],
}
impl windows_core::TypeKind for WIN32_STREAM_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for WIN32_STREAM_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WOF_FILE_COMPRESSION_INFO_V0 {
    pub Algorithm: u32,
}
impl windows_core::TypeKind for WOF_FILE_COMPRESSION_INFO_V0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WOF_FILE_COMPRESSION_INFO_V0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WOF_FILE_COMPRESSION_INFO_V1 {
    pub Algorithm: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for WOF_FILE_COMPRESSION_INFO_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WOF_FILE_COMPRESSION_INFO_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
pub type CACHE_ACCESS_CHECK = Option<unsafe extern "system" fn(psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, hclienttoken: super::super::Foundation::HANDLE, dwdesiredaccess: u32, genericmapping: *mut super::super::Security::GENERIC_MAPPING, privilegeset: *mut super::super::Security::PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
pub type CACHE_DESTROY_CALLBACK = Option<unsafe extern "system" fn(cb: u32, lpb: *mut u8)>;
pub type CACHE_KEY_COMPARE = Option<unsafe extern "system" fn(cbkey1: u32, lpbkey1: *mut u8, cbkey2: u32, lpbkey2: *mut u8) -> i32>;
pub type CACHE_KEY_HASH = Option<unsafe extern "system" fn(lpbkey: *mut u8, cbkey: u32) -> u32>;
pub type CACHE_READ_CALLBACK = Option<unsafe extern "system" fn(cb: u32, lpb: *mut u8, lpvcontext: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type CLAIMMEDIALABEL = Option<unsafe extern "system" fn(pbuffer: *const u8, nbuffersize: u32, plabelinfo: *mut MediaLabelInfo) -> u32>;
pub type CLAIMMEDIALABELEX = Option<unsafe extern "system" fn(pbuffer: *const u8, nbuffersize: u32, plabelinfo: *mut MediaLabelInfo, labelguid: *mut windows_core::GUID) -> u32>;
pub type CLFS_BLOCK_ALLOCATION = Option<unsafe extern "system" fn(cbbufferlength: u32, pvusercontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type CLFS_BLOCK_DEALLOCATION = Option<unsafe extern "system" fn(pvbuffer: *mut core::ffi::c_void, pvusercontext: *mut core::ffi::c_void)>;
pub type FCACHE_CREATE_CALLBACK = Option<unsafe extern "system" fn(lpstrname: windows_core::PCSTR, lpvdata: *mut core::ffi::c_void, cbfilesize: *mut u32, cbfilesizehigh: *mut u32) -> super::super::Foundation::HANDLE>;
pub type FCACHE_RICHCREATE_CALLBACK = Option<unsafe extern "system" fn(lpstrname: windows_core::PCSTR, lpvdata: *mut core::ffi::c_void, cbfilesize: *mut u32, cbfilesizehigh: *mut u32, pfdidwescanit: *mut super::super::Foundation::BOOL, pfisstuffed: *mut super::super::Foundation::BOOL, pfstoredwithdots: *mut super::super::Foundation::BOOL, pfstoredwithterminatingdot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE>;
pub type LPPROGRESS_ROUTINE = Option<unsafe extern "system" fn(totalfilesize: i64, totalbytestransferred: i64, streamsize: i64, streambytestransferred: i64, dwstreamnumber: u32, dwcallbackreason: LPPROGRESS_ROUTINE_CALLBACK_REASON, hsourcefile: super::super::Foundation::HANDLE, hdestinationfile: super::super::Foundation::HANDLE, lpdata: *const core::ffi::c_void) -> u32>;
pub type MAXMEDIALABEL = Option<unsafe extern "system" fn(pmaxsize: *mut u32) -> u32>;
pub type PCLFS_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pvoverlapped: *mut core::ffi::c_void, ulreserved: u32)>;
pub type PCOPYFILE2_PROGRESS_ROUTINE = Option<unsafe extern "system" fn(pmessage: *const COPYFILE2_MESSAGE, pvcallbackcontext: *const core::ffi::c_void) -> COPYFILE2_MESSAGE_ACTION>;
pub type PFE_EXPORT_FUNC = Option<unsafe extern "system" fn(pbdata: *const u8, pvcallbackcontext: *const core::ffi::c_void, ullength: u32) -> u32>;
pub type PFE_IMPORT_FUNC = Option<unsafe extern "system" fn(pbdata: *mut u8, pvcallbackcontext: *const core::ffi::c_void, ullength: *mut u32) -> u32>;
pub type PFN_IO_COMPLETION = Option<unsafe extern "system" fn(pcontext: *mut FIO_CONTEXT, lpo: *mut FH_OVERLAPPED, cb: u32, dwcompletionstatus: u32)>;
pub type PLOG_FULL_HANDLER_CALLBACK = Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, dwerror: u32, flogispinned: super::super::Foundation::BOOL, pvclientcontext: *mut core::ffi::c_void)>;
pub type PLOG_TAIL_ADVANCE_CALLBACK = Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, lsntarget: CLS_LSN, pvclientcontext: *mut core::ffi::c_void)>;
pub type PLOG_UNPINNED_CALLBACK = Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, pvclientcontext: *mut core::ffi::c_void)>;
pub type WofEnumEntryProc = Option<unsafe extern "system" fn(entryinfo: *const core::ffi::c_void, userdata: *const core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type WofEnumFilesProc = Option<unsafe extern "system" fn(filepath: windows_core::PCWSTR, externalfileinfo: *const core::ffi::c_void, userdata: *const core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
core::include!("impl.rs");

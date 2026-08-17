#[inline]
pub unsafe fn AddAtomA<P0>(lpstring: P0) -> u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddAtomA(lpstring : windows_core::PCSTR) -> u16);
    AddAtomA(lpstring.param().abi())
}
#[inline]
pub unsafe fn AddAtomW<P0>(lpstring: P0) -> u16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddAtomW(lpstring : windows_core::PCWSTR) -> u16);
    AddAtomW(lpstring.param().abi())
}
#[inline]
pub unsafe fn AddClipboardFormatListener<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn AddClipboardFormatListener(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    AddClipboardFormatListener(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn ChangeClipboardChain<P0, P1>(hwndremove: P0, hwndnewnext: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ChangeClipboardChain(hwndremove : super::super::Foundation:: HWND, hwndnewnext : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    ChangeClipboardChain(hwndremove.param().abi(), hwndnewnext.param().abi())
}
#[inline]
pub unsafe fn CloseClipboard() -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn CloseClipboard() -> super::super::Foundation:: BOOL);
    CloseClipboard().ok()
}
#[inline]
pub unsafe fn CountClipboardFormats() -> i32 {
    windows_targets::link!("user32.dll" "system" fn CountClipboardFormats() -> i32);
    CountClipboardFormats()
}
#[inline]
pub unsafe fn DdeAbandonTransaction<P0>(idinst: u32, hconv: P0, idtransaction: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HCONV>,
{
    windows_targets::link!("user32.dll" "system" fn DdeAbandonTransaction(idinst : u32, hconv : HCONV, idtransaction : u32) -> super::super::Foundation:: BOOL);
    DdeAbandonTransaction(idinst, hconv.param().abi(), idtransaction)
}
#[inline]
pub unsafe fn DdeAccessData<P0>(hdata: P0, pcbdatasize: Option<*mut u32>) -> *mut u8
where
    P0: windows_core::Param<HDDEDATA>,
{
    windows_targets::link!("user32.dll" "system" fn DdeAccessData(hdata : HDDEDATA, pcbdatasize : *mut u32) -> *mut u8);
    DdeAccessData(hdata.param().abi(), core::mem::transmute(pcbdatasize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DdeAddData<P0>(hdata: P0, psrc: &[u8], cboff: u32) -> HDDEDATA
where
    P0: windows_core::Param<HDDEDATA>,
{
    windows_targets::link!("user32.dll" "system" fn DdeAddData(hdata : HDDEDATA, psrc : *const u8, cb : u32, cboff : u32) -> HDDEDATA);
    DdeAddData(hdata.param().abi(), core::mem::transmute(psrc.as_ptr()), psrc.len().try_into().unwrap(), cboff)
}
#[inline]
pub unsafe fn DdeClientTransaction<P0, P1>(pdata: Option<*const u8>, cbdata: u32, hconv: P0, hszitem: P1, wfmt: u32, wtype: DDE_CLIENT_TRANSACTION_TYPE, dwtimeout: u32, pdwresult: Option<*mut u32>) -> HDDEDATA
where
    P0: windows_core::Param<HCONV>,
    P1: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeClientTransaction(pdata : *const u8, cbdata : u32, hconv : HCONV, hszitem : HSZ, wfmt : u32, wtype : DDE_CLIENT_TRANSACTION_TYPE, dwtimeout : u32, pdwresult : *mut u32) -> HDDEDATA);
    DdeClientTransaction(core::mem::transmute(pdata.unwrap_or(std::ptr::null())), cbdata, hconv.param().abi(), hszitem.param().abi(), wfmt, wtype, dwtimeout, core::mem::transmute(pdwresult.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DdeCmpStringHandles<P0, P1>(hsz1: P0, hsz2: P1) -> i32
where
    P0: windows_core::Param<HSZ>,
    P1: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeCmpStringHandles(hsz1 : HSZ, hsz2 : HSZ) -> i32);
    DdeCmpStringHandles(hsz1.param().abi(), hsz2.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DdeConnect<P0, P1>(idinst: u32, hszservice: P0, hsztopic: P1, pcc: Option<*const CONVCONTEXT>) -> HCONV
where
    P0: windows_core::Param<HSZ>,
    P1: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeConnect(idinst : u32, hszservice : HSZ, hsztopic : HSZ, pcc : *const CONVCONTEXT) -> HCONV);
    DdeConnect(idinst, hszservice.param().abi(), hsztopic.param().abi(), core::mem::transmute(pcc.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DdeConnectList<P0, P1, P2>(idinst: u32, hszservice: P0, hsztopic: P1, hconvlist: P2, pcc: Option<*const CONVCONTEXT>) -> HCONVLIST
where
    P0: windows_core::Param<HSZ>,
    P1: windows_core::Param<HSZ>,
    P2: windows_core::Param<HCONVLIST>,
{
    windows_targets::link!("user32.dll" "system" fn DdeConnectList(idinst : u32, hszservice : HSZ, hsztopic : HSZ, hconvlist : HCONVLIST, pcc : *const CONVCONTEXT) -> HCONVLIST);
    DdeConnectList(idinst, hszservice.param().abi(), hsztopic.param().abi(), hconvlist.param().abi(), core::mem::transmute(pcc.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn DdeCreateDataHandle<P0>(idinst: u32, psrc: Option<&[u8]>, cboff: u32, hszitem: P0, wfmt: u32, afcmd: u32) -> HDDEDATA
where
    P0: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeCreateDataHandle(idinst : u32, psrc : *const u8, cb : u32, cboff : u32, hszitem : HSZ, wfmt : u32, afcmd : u32) -> HDDEDATA);
    DdeCreateDataHandle(idinst, core::mem::transmute(psrc.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psrc.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), cboff, hszitem.param().abi(), wfmt, afcmd)
}
#[inline]
pub unsafe fn DdeCreateStringHandleA<P0>(idinst: u32, psz: P0, icodepage: i32) -> HSZ
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn DdeCreateStringHandleA(idinst : u32, psz : windows_core::PCSTR, icodepage : i32) -> HSZ);
    DdeCreateStringHandleA(idinst, psz.param().abi(), icodepage)
}
#[inline]
pub unsafe fn DdeCreateStringHandleW<P0>(idinst: u32, psz: P0, icodepage: i32) -> HSZ
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn DdeCreateStringHandleW(idinst : u32, psz : windows_core::PCWSTR, icodepage : i32) -> HSZ);
    DdeCreateStringHandleW(idinst, psz.param().abi(), icodepage)
}
#[inline]
pub unsafe fn DdeDisconnect<P0>(hconv: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HCONV>,
{
    windows_targets::link!("user32.dll" "system" fn DdeDisconnect(hconv : HCONV) -> super::super::Foundation:: BOOL);
    DdeDisconnect(hconv.param().abi())
}
#[inline]
pub unsafe fn DdeDisconnectList<P0>(hconvlist: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HCONVLIST>,
{
    windows_targets::link!("user32.dll" "system" fn DdeDisconnectList(hconvlist : HCONVLIST) -> super::super::Foundation:: BOOL);
    DdeDisconnectList(hconvlist.param().abi())
}
#[inline]
pub unsafe fn DdeEnableCallback<P0>(idinst: u32, hconv: P0, wcmd: DDE_ENABLE_CALLBACK_CMD) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HCONV>,
{
    windows_targets::link!("user32.dll" "system" fn DdeEnableCallback(idinst : u32, hconv : HCONV, wcmd : DDE_ENABLE_CALLBACK_CMD) -> super::super::Foundation:: BOOL);
    DdeEnableCallback(idinst, hconv.param().abi(), wcmd)
}
#[inline]
pub unsafe fn DdeFreeDataHandle<P0>(hdata: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HDDEDATA>,
{
    windows_targets::link!("user32.dll" "system" fn DdeFreeDataHandle(hdata : HDDEDATA) -> super::super::Foundation:: BOOL);
    DdeFreeDataHandle(hdata.param().abi())
}
#[inline]
pub unsafe fn DdeFreeStringHandle<P0>(idinst: u32, hsz: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeFreeStringHandle(idinst : u32, hsz : HSZ) -> super::super::Foundation:: BOOL);
    DdeFreeStringHandle(idinst, hsz.param().abi())
}
#[inline]
pub unsafe fn DdeGetData<P0>(hdata: P0, pdst: Option<&mut [u8]>, cboff: u32) -> u32
where
    P0: windows_core::Param<HDDEDATA>,
{
    windows_targets::link!("user32.dll" "system" fn DdeGetData(hdata : HDDEDATA, pdst : *mut u8, cbmax : u32, cboff : u32) -> u32);
    DdeGetData(hdata.param().abi(), core::mem::transmute(pdst.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdst.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), cboff)
}
#[inline]
pub unsafe fn DdeGetLastError(idinst: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn DdeGetLastError(idinst : u32) -> u32);
    DdeGetLastError(idinst)
}
#[inline]
pub unsafe fn DdeImpersonateClient<P0>(hconv: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HCONV>,
{
    windows_targets::link!("user32.dll" "system" fn DdeImpersonateClient(hconv : HCONV) -> super::super::Foundation:: BOOL);
    DdeImpersonateClient(hconv.param().abi()).ok()
}
#[inline]
pub unsafe fn DdeInitializeA(pidinst: *mut u32, pfncallback: PFNCALLBACK, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn DdeInitializeA(pidinst : *mut u32, pfncallback : PFNCALLBACK, afcmd : DDE_INITIALIZE_COMMAND, ulres : u32) -> u32);
    DdeInitializeA(pidinst, pfncallback, afcmd, ulres)
}
#[inline]
pub unsafe fn DdeInitializeW(pidinst: *mut u32, pfncallback: PFNCALLBACK, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn DdeInitializeW(pidinst : *mut u32, pfncallback : PFNCALLBACK, afcmd : DDE_INITIALIZE_COMMAND, ulres : u32) -> u32);
    DdeInitializeW(pidinst, pfncallback, afcmd, ulres)
}
#[inline]
pub unsafe fn DdeKeepStringHandle<P0>(idinst: u32, hsz: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeKeepStringHandle(idinst : u32, hsz : HSZ) -> super::super::Foundation:: BOOL);
    DdeKeepStringHandle(idinst, hsz.param().abi())
}
#[inline]
pub unsafe fn DdeNameService<P0, P1>(idinst: u32, hsz1: P0, hsz2: P1, afcmd: DDE_NAME_SERVICE_CMD) -> HDDEDATA
where
    P0: windows_core::Param<HSZ>,
    P1: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeNameService(idinst : u32, hsz1 : HSZ, hsz2 : HSZ, afcmd : DDE_NAME_SERVICE_CMD) -> HDDEDATA);
    DdeNameService(idinst, hsz1.param().abi(), hsz2.param().abi(), afcmd)
}
#[inline]
pub unsafe fn DdePostAdvise<P0, P1>(idinst: u32, hsztopic: P0, hszitem: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HSZ>,
    P1: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdePostAdvise(idinst : u32, hsztopic : HSZ, hszitem : HSZ) -> super::super::Foundation:: BOOL);
    DdePostAdvise(idinst, hsztopic.param().abi(), hszitem.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DdeQueryConvInfo<P0>(hconv: P0, idtransaction: u32, pconvinfo: *mut CONVINFO) -> u32
where
    P0: windows_core::Param<HCONV>,
{
    windows_targets::link!("user32.dll" "system" fn DdeQueryConvInfo(hconv : HCONV, idtransaction : u32, pconvinfo : *mut CONVINFO) -> u32);
    DdeQueryConvInfo(hconv.param().abi(), idtransaction, pconvinfo)
}
#[inline]
pub unsafe fn DdeQueryNextServer<P0, P1>(hconvlist: P0, hconvprev: P1) -> HCONV
where
    P0: windows_core::Param<HCONVLIST>,
    P1: windows_core::Param<HCONV>,
{
    windows_targets::link!("user32.dll" "system" fn DdeQueryNextServer(hconvlist : HCONVLIST, hconvprev : HCONV) -> HCONV);
    DdeQueryNextServer(hconvlist.param().abi(), hconvprev.param().abi())
}
#[inline]
pub unsafe fn DdeQueryStringA<P0>(idinst: u32, hsz: P0, psz: Option<&mut [u8]>, icodepage: i32) -> u32
where
    P0: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeQueryStringA(idinst : u32, hsz : HSZ, psz : windows_core::PSTR, cchmax : u32, icodepage : i32) -> u32);
    DdeQueryStringA(idinst, hsz.param().abi(), core::mem::transmute(psz.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), icodepage)
}
#[inline]
pub unsafe fn DdeQueryStringW<P0>(idinst: u32, hsz: P0, psz: Option<&mut [u16]>, icodepage: i32) -> u32
where
    P0: windows_core::Param<HSZ>,
{
    windows_targets::link!("user32.dll" "system" fn DdeQueryStringW(idinst : u32, hsz : HSZ, psz : windows_core::PWSTR, cchmax : u32, icodepage : i32) -> u32);
    DdeQueryStringW(idinst, hsz.param().abi(), core::mem::transmute(psz.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), icodepage)
}
#[inline]
pub unsafe fn DdeReconnect<P0>(hconv: P0) -> HCONV
where
    P0: windows_core::Param<HCONV>,
{
    windows_targets::link!("user32.dll" "system" fn DdeReconnect(hconv : HCONV) -> HCONV);
    DdeReconnect(hconv.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DdeSetQualityOfService<P0>(hwndclient: P0, pqosnew: *const super::super::Security::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::super::Security::SECURITY_QUALITY_OF_SERVICE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn DdeSetQualityOfService(hwndclient : super::super::Foundation:: HWND, pqosnew : *const super::super::Security:: SECURITY_QUALITY_OF_SERVICE, pqosprev : *mut super::super::Security:: SECURITY_QUALITY_OF_SERVICE) -> super::super::Foundation:: BOOL);
    DdeSetQualityOfService(hwndclient.param().abi(), pqosnew, pqosprev).ok()
}
#[inline]
pub unsafe fn DdeSetUserHandle<P0>(hconv: P0, id: u32, huser: usize) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HCONV>,
{
    windows_targets::link!("user32.dll" "system" fn DdeSetUserHandle(hconv : HCONV, id : u32, huser : usize) -> super::super::Foundation:: BOOL);
    DdeSetUserHandle(hconv.param().abi(), id, huser)
}
#[inline]
pub unsafe fn DdeUnaccessData<P0>(hdata: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HDDEDATA>,
{
    windows_targets::link!("user32.dll" "system" fn DdeUnaccessData(hdata : HDDEDATA) -> super::super::Foundation:: BOOL);
    DdeUnaccessData(hdata.param().abi())
}
#[inline]
pub unsafe fn DdeUninitialize(idinst: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn DdeUninitialize(idinst : u32) -> super::super::Foundation:: BOOL);
    DdeUninitialize(idinst)
}
#[inline]
pub unsafe fn DeleteAtom(natom: u16) -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn DeleteAtom(natom : u16) -> u16);
    DeleteAtom(natom)
}
#[inline]
pub unsafe fn EmptyClipboard() -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn EmptyClipboard() -> super::super::Foundation:: BOOL);
    EmptyClipboard().ok()
}
#[inline]
pub unsafe fn EnumClipboardFormats(format: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn EnumClipboardFormats(format : u32) -> u32);
    EnumClipboardFormats(format)
}
#[inline]
pub unsafe fn FindAtomA<P0>(lpstring: P0) -> u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindAtomA(lpstring : windows_core::PCSTR) -> u16);
    FindAtomA(lpstring.param().abi())
}
#[inline]
pub unsafe fn FindAtomW<P0>(lpstring: P0) -> u16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindAtomW(lpstring : windows_core::PCWSTR) -> u16);
    FindAtomW(lpstring.param().abi())
}
#[inline]
pub unsafe fn FreeDDElParam<P0>(msg: u32, lparam: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn FreeDDElParam(msg : u32, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    FreeDDElParam(msg, lparam.param().abi())
}
#[inline]
pub unsafe fn GetAtomNameA(natom: u16, lpbuffer: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetAtomNameA(natom : u16, lpbuffer : windows_core::PSTR, nsize : i32) -> u32);
    GetAtomNameA(natom, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetAtomNameW(natom: u16, lpbuffer: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetAtomNameW(natom : u16, lpbuffer : windows_core::PWSTR, nsize : i32) -> u32);
    GetAtomNameW(natom, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetClipboardData(uformat: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("user32.dll" "system" fn GetClipboardData(uformat : u32) -> super::super::Foundation:: HANDLE);
    let result__ = GetClipboardData(uformat);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetClipboardFormatNameA(format: u32, lpszformatname: &mut [u8]) -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetClipboardFormatNameA(format : u32, lpszformatname : windows_core::PSTR, cchmaxcount : i32) -> i32);
    GetClipboardFormatNameA(format, core::mem::transmute(lpszformatname.as_ptr()), lpszformatname.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetClipboardFormatNameW(format: u32, lpszformatname: &mut [u16]) -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetClipboardFormatNameW(format : u32, lpszformatname : windows_core::PWSTR, cchmaxcount : i32) -> i32);
    GetClipboardFormatNameW(format, core::mem::transmute(lpszformatname.as_ptr()), lpszformatname.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetClipboardOwner() -> windows_core::Result<super::super::Foundation::HWND> {
    windows_targets::link!("user32.dll" "system" fn GetClipboardOwner() -> super::super::Foundation:: HWND);
    let result__ = GetClipboardOwner();
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetClipboardSequenceNumber() -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetClipboardSequenceNumber() -> u32);
    GetClipboardSequenceNumber()
}
#[inline]
pub unsafe fn GetClipboardViewer() -> windows_core::Result<super::super::Foundation::HWND> {
    windows_targets::link!("user32.dll" "system" fn GetClipboardViewer() -> super::super::Foundation:: HWND);
    let result__ = GetClipboardViewer();
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetOpenClipboardWindow() -> windows_core::Result<super::super::Foundation::HWND> {
    windows_targets::link!("user32.dll" "system" fn GetOpenClipboardWindow() -> super::super::Foundation:: HWND);
    let result__ = GetOpenClipboardWindow();
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetPriorityClipboardFormat(paformatprioritylist: &[u32]) -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetPriorityClipboardFormat(paformatprioritylist : *const u32, cformats : i32) -> i32);
    GetPriorityClipboardFormat(core::mem::transmute(paformatprioritylist.as_ptr()), paformatprioritylist.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetUpdatedClipboardFormats(lpuiformats: &mut [u32], pcformatsout: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetUpdatedClipboardFormats(lpuiformats : *mut u32, cformats : u32, pcformatsout : *mut u32) -> super::super::Foundation:: BOOL);
    GetUpdatedClipboardFormats(core::mem::transmute(lpuiformats.as_ptr()), lpuiformats.len().try_into().unwrap(), pcformatsout).ok()
}
#[inline]
pub unsafe fn GlobalAddAtomA<P0>(lpstring: P0) -> u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GlobalAddAtomA(lpstring : windows_core::PCSTR) -> u16);
    GlobalAddAtomA(lpstring.param().abi())
}
#[inline]
pub unsafe fn GlobalAddAtomExA<P0>(lpstring: P0, flags: u32) -> u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GlobalAddAtomExA(lpstring : windows_core::PCSTR, flags : u32) -> u16);
    GlobalAddAtomExA(lpstring.param().abi(), flags)
}
#[inline]
pub unsafe fn GlobalAddAtomExW<P0>(lpstring: P0, flags: u32) -> u16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GlobalAddAtomExW(lpstring : windows_core::PCWSTR, flags : u32) -> u16);
    GlobalAddAtomExW(lpstring.param().abi(), flags)
}
#[inline]
pub unsafe fn GlobalAddAtomW<P0>(lpstring: P0) -> u16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GlobalAddAtomW(lpstring : windows_core::PCWSTR) -> u16);
    GlobalAddAtomW(lpstring.param().abi())
}
#[inline]
pub unsafe fn GlobalDeleteAtom(natom: u16) -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn GlobalDeleteAtom(natom : u16) -> u16);
    GlobalDeleteAtom(natom)
}
#[inline]
pub unsafe fn GlobalFindAtomA<P0>(lpstring: P0) -> u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GlobalFindAtomA(lpstring : windows_core::PCSTR) -> u16);
    GlobalFindAtomA(lpstring.param().abi())
}
#[inline]
pub unsafe fn GlobalFindAtomW<P0>(lpstring: P0) -> u16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GlobalFindAtomW(lpstring : windows_core::PCWSTR) -> u16);
    GlobalFindAtomW(lpstring.param().abi())
}
#[inline]
pub unsafe fn GlobalGetAtomNameA(natom: u16, lpbuffer: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GlobalGetAtomNameA(natom : u16, lpbuffer : windows_core::PSTR, nsize : i32) -> u32);
    GlobalGetAtomNameA(natom, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GlobalGetAtomNameW(natom: u16, lpbuffer: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GlobalGetAtomNameW(natom : u16, lpbuffer : windows_core::PWSTR, nsize : i32) -> u32);
    GlobalGetAtomNameW(natom, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn ImpersonateDdeClientWindow<P0, P1>(hwndclient: P0, hwndserver: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ImpersonateDdeClientWindow(hwndclient : super::super::Foundation:: HWND, hwndserver : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    ImpersonateDdeClientWindow(hwndclient.param().abi(), hwndserver.param().abi()).ok()
}
#[inline]
pub unsafe fn InitAtomTable(nsize: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn InitAtomTable(nsize : u32) -> super::super::Foundation:: BOOL);
    InitAtomTable(nsize)
}
#[inline]
pub unsafe fn IsClipboardFormatAvailable(format: u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn IsClipboardFormatAvailable(format : u32) -> super::super::Foundation:: BOOL);
    IsClipboardFormatAvailable(format).ok()
}
#[inline]
pub unsafe fn OpenClipboard<P0>(hwndnewowner: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn OpenClipboard(hwndnewowner : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    OpenClipboard(hwndnewowner.param().abi()).ok()
}
#[inline]
pub unsafe fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM {
    windows_targets::link!("user32.dll" "system" fn PackDDElParam(msg : u32, uilo : usize, uihi : usize) -> super::super::Foundation:: LPARAM);
    PackDDElParam(msg, uilo, uihi)
}
#[inline]
pub unsafe fn RegisterClipboardFormatA<P0>(lpszformat: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterClipboardFormatA(lpszformat : windows_core::PCSTR) -> u32);
    RegisterClipboardFormatA(lpszformat.param().abi())
}
#[inline]
pub unsafe fn RegisterClipboardFormatW<P0>(lpszformat: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterClipboardFormatW(lpszformat : windows_core::PCWSTR) -> u32);
    RegisterClipboardFormatW(lpszformat.param().abi())
}
#[inline]
pub unsafe fn RemoveClipboardFormatListener<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn RemoveClipboardFormatListener(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    RemoveClipboardFormatListener(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn ReuseDDElParam<P0>(lparam: P0, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn ReuseDDElParam(lparam : super::super::Foundation:: LPARAM, msgin : u32, msgout : u32, uilo : usize, uihi : usize) -> super::super::Foundation:: LPARAM);
    ReuseDDElParam(lparam.param().abi(), msgin, msgout, uilo, uihi)
}
#[inline]
pub unsafe fn SetClipboardData<P0>(uformat: u32, hmem: P0) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn SetClipboardData(uformat : u32, hmem : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = SetClipboardData(uformat, hmem.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn SetClipboardViewer<P0>(hwndnewviewer: P0) -> windows_core::Result<super::super::Foundation::HWND>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetClipboardViewer(hwndnewviewer : super::super::Foundation:: HWND) -> super::super::Foundation:: HWND);
    let result__ = SetClipboardViewer(hwndnewviewer.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetWinMetaFileBits<P0>(lpmeta16data: &[u8], hdcref: P0, lpmfp: Option<*const METAFILEPICT>) -> super::super::Graphics::Gdi::HENHMETAFILE
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn SetWinMetaFileBits(nsize : u32, lpmeta16data : *const u8, hdcref : super::super::Graphics::Gdi:: HDC, lpmfp : *const METAFILEPICT) -> super::super::Graphics::Gdi:: HENHMETAFILE);
    SetWinMetaFileBits(lpmeta16data.len().try_into().unwrap(), core::mem::transmute(lpmeta16data.as_ptr()), hdcref.param().abi(), core::mem::transmute(lpmfp.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn UnpackDDElParam<P0>(msg: u32, lparam: P0, puilo: *mut usize, puihi: *mut usize) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn UnpackDDElParam(msg : u32, lparam : super::super::Foundation:: LPARAM, puilo : *mut usize, puihi : *mut usize) -> super::super::Foundation:: BOOL);
    UnpackDDElParam(msg, lparam.param().abi(), puilo, puihi)
}
pub const APPCLASS_MASK: i32 = 15i32;
pub const APPCLASS_MONITOR: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1u32);
pub const APPCLASS_STANDARD: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(0u32);
pub const APPCMD_CLIENTONLY: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16u32);
pub const APPCMD_FILTERINITS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(32u32);
pub const APPCMD_MASK: i32 = 4080i32;
pub const CADV_LATEACK: u32 = 65535u32;
pub const CBF_FAIL_ADVISES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16384u32);
pub const CBF_FAIL_ALLSVRXACTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(258048u32);
pub const CBF_FAIL_CONNECTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(8192u32);
pub const CBF_FAIL_EXECUTES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(32768u32);
pub const CBF_FAIL_POKES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(65536u32);
pub const CBF_FAIL_REQUESTS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(131072u32);
pub const CBF_FAIL_SELFCONNECTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(4096u32);
pub const CBF_SKIP_ALLNOTIFICATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(3932160u32);
pub const CBF_SKIP_CONNECT_CONFIRMS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(262144u32);
pub const CBF_SKIP_DISCONNECTS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(2097152u32);
pub const CBF_SKIP_REGISTRATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(524288u32);
pub const CBF_SKIP_UNREGISTRATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1048576u32);
pub const CP_WINANSI: i32 = 1004i32;
pub const CP_WINNEUTRAL: i32 = 1200i32;
pub const CP_WINUNICODE: i32 = 1200i32;
pub const DDE_FACK: u32 = 32768u32;
pub const DDE_FACKREQ: u32 = 32768u32;
pub const DDE_FAPPSTATUS: u32 = 255u32;
pub const DDE_FBUSY: u32 = 16384u32;
pub const DDE_FDEFERUPD: u32 = 16384u32;
pub const DDE_FNOTPROCESSED: u32 = 0u32;
pub const DDE_FRELEASE: u32 = 8192u32;
pub const DDE_FREQUESTED: u32 = 4096u32;
pub const DMLERR_ADVACKTIMEOUT: u32 = 16384u32;
pub const DMLERR_BUSY: u32 = 16385u32;
pub const DMLERR_DATAACKTIMEOUT: u32 = 16386u32;
pub const DMLERR_DLL_NOT_INITIALIZED: u32 = 16387u32;
pub const DMLERR_DLL_USAGE: u32 = 16388u32;
pub const DMLERR_EXECACKTIMEOUT: u32 = 16389u32;
pub const DMLERR_FIRST: u32 = 16384u32;
pub const DMLERR_INVALIDPARAMETER: u32 = 16390u32;
pub const DMLERR_LAST: u32 = 16401u32;
pub const DMLERR_LOW_MEMORY: u32 = 16391u32;
pub const DMLERR_MEMORY_ERROR: u32 = 16392u32;
pub const DMLERR_NOTPROCESSED: u32 = 16393u32;
pub const DMLERR_NO_CONV_ESTABLISHED: u32 = 16394u32;
pub const DMLERR_NO_ERROR: u32 = 0u32;
pub const DMLERR_POKEACKTIMEOUT: u32 = 16395u32;
pub const DMLERR_POSTMSG_FAILED: u32 = 16396u32;
pub const DMLERR_REENTRANCY: u32 = 16397u32;
pub const DMLERR_SERVER_DIED: u32 = 16398u32;
pub const DMLERR_SYS_ERROR: u32 = 16399u32;
pub const DMLERR_UNADVACKTIMEOUT: u32 = 16400u32;
pub const DMLERR_UNFOUND_QUEUE_ID: u32 = 16401u32;
pub const DNS_FILTEROFF: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(8u32);
pub const DNS_FILTERON: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(4u32);
pub const DNS_REGISTER: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(1u32);
pub const DNS_UNREGISTER: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(2u32);
pub const EC_DISABLE: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(8u32);
pub const EC_ENABLEALL: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(0u32);
pub const EC_ENABLEONE: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(128u32);
pub const EC_QUERYWAITING: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(2u32);
pub const HDATA_APPOWNED: u32 = 1u32;
pub const MAX_MONITORS: u32 = 4u32;
pub const MF_CALLBACKS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(134217728u32);
pub const MF_CONV: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1073741824u32);
pub const MF_ERRORS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(268435456u32);
pub const MF_HSZ_INFO: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16777216u32);
pub const MF_LINKS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(536870912u32);
pub const MF_MASK: u32 = 4278190080u32;
pub const MF_POSTMSGS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(67108864u32);
pub const MF_SENDMSGS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(33554432u32);
pub const MH_CLEANUP: u32 = 4u32;
pub const MH_CREATE: u32 = 1u32;
pub const MH_DELETE: u32 = 3u32;
pub const MH_KEEP: u32 = 2u32;
pub const MSGF_DDEMGR: u32 = 32769u32;
pub const QID_SYNC: u32 = 4294967295u32;
pub const ST_ADVISE: CONVINFO_STATUS = CONVINFO_STATUS(2u32);
pub const ST_BLOCKED: CONVINFO_STATUS = CONVINFO_STATUS(8u32);
pub const ST_BLOCKNEXT: CONVINFO_STATUS = CONVINFO_STATUS(128u32);
pub const ST_CLIENT: CONVINFO_STATUS = CONVINFO_STATUS(16u32);
pub const ST_CONNECTED: CONVINFO_STATUS = CONVINFO_STATUS(1u32);
pub const ST_INLIST: CONVINFO_STATUS = CONVINFO_STATUS(64u32);
pub const ST_ISLOCAL: CONVINFO_STATUS = CONVINFO_STATUS(4u32);
pub const ST_ISSELF: CONVINFO_STATUS = CONVINFO_STATUS(256u32);
pub const ST_TERMINATED: CONVINFO_STATUS = CONVINFO_STATUS(32u32);
pub const SZDDESYS_ITEM_FORMATS: windows_core::PCWSTR = windows_core::w!("Formats");
pub const SZDDESYS_ITEM_HELP: windows_core::PCWSTR = windows_core::w!("Help");
pub const SZDDESYS_ITEM_RTNMSG: windows_core::PCWSTR = windows_core::w!("ReturnMessage");
pub const SZDDESYS_ITEM_STATUS: windows_core::PCWSTR = windows_core::w!("Status");
pub const SZDDESYS_ITEM_SYSITEMS: windows_core::PCWSTR = windows_core::w!("SysItems");
pub const SZDDESYS_ITEM_TOPICS: windows_core::PCWSTR = windows_core::w!("Topics");
pub const SZDDESYS_TOPIC: windows_core::PCWSTR = windows_core::w!("System");
pub const SZDDE_ITEM_ITEMLIST: windows_core::PCWSTR = windows_core::w!("TopicItemList");
pub const TIMEOUT_ASYNC: u32 = 4294967295u32;
pub const WM_DDE_ACK: u32 = 996u32;
pub const WM_DDE_ADVISE: u32 = 994u32;
pub const WM_DDE_DATA: u32 = 997u32;
pub const WM_DDE_EXECUTE: u32 = 1000u32;
pub const WM_DDE_FIRST: u32 = 992u32;
pub const WM_DDE_INITIATE: u32 = 992u32;
pub const WM_DDE_LAST: u32 = 1000u32;
pub const WM_DDE_POKE: u32 = 999u32;
pub const WM_DDE_REQUEST: u32 = 998u32;
pub const WM_DDE_TERMINATE: u32 = 993u32;
pub const WM_DDE_UNADVISE: u32 = 995u32;
pub const XCLASS_BOOL: u32 = 4096u32;
pub const XCLASS_DATA: u32 = 8192u32;
pub const XCLASS_FLAGS: u32 = 16384u32;
pub const XCLASS_MASK: u32 = 64512u32;
pub const XCLASS_NOTIFICATION: u32 = 32768u32;
pub const XST_ADVACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(13u32);
pub const XST_ADVDATAACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(16u32);
pub const XST_ADVDATASENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(15u32);
pub const XST_ADVSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(11u32);
pub const XST_CONNECTED: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(2u32);
pub const XST_DATARCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(6u32);
pub const XST_EXECACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(10u32);
pub const XST_EXECSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(9u32);
pub const XST_INCOMPLETE: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(1u32);
pub const XST_INIT1: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(3u32);
pub const XST_INIT2: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(4u32);
pub const XST_NULL: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(0u32);
pub const XST_POKEACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(8u32);
pub const XST_POKESENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(7u32);
pub const XST_REQSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(5u32);
pub const XST_UNADVACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(14u32);
pub const XST_UNADVSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(12u32);
pub const XTYPF_ACKREQ: u32 = 8u32;
pub const XTYPF_NOBLOCK: u32 = 2u32;
pub const XTYPF_NODATA: u32 = 4u32;
pub const XTYP_ADVDATA: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16400u32);
pub const XTYP_ADVREQ: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8226u32);
pub const XTYP_ADVSTART: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(4144u32);
pub const XTYP_ADVSTOP: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32832u32);
pub const XTYP_CONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(4194u32);
pub const XTYP_CONNECT_CONFIRM: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32882u32);
pub const XTYP_DISCONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32962u32);
pub const XTYP_EXECUTE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16464u32);
pub const XTYP_MASK: u32 = 240u32;
pub const XTYP_MONITOR: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(33010u32);
pub const XTYP_POKE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16528u32);
pub const XTYP_REGISTER: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32930u32);
pub const XTYP_REQUEST: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8368u32);
pub const XTYP_SHIFT: u32 = 4u32;
pub const XTYP_UNREGISTER: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32978u32);
pub const XTYP_WILDCONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8418u32);
pub const XTYP_XACT_COMPLETE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32896u32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONVINFO_CONVERSATION_STATE(pub u32);
impl windows_core::TypeKind for CONVINFO_CONVERSATION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONVINFO_CONVERSATION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONVINFO_CONVERSATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONVINFO_STATUS(pub u32);
impl windows_core::TypeKind for CONVINFO_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONVINFO_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONVINFO_STATUS").field(&self.0).finish()
    }
}
impl CONVINFO_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CONVINFO_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CONVINFO_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CONVINFO_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CONVINFO_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CONVINFO_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DDE_CLIENT_TRANSACTION_TYPE(pub u32);
impl windows_core::TypeKind for DDE_CLIENT_TRANSACTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DDE_CLIENT_TRANSACTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DDE_CLIENT_TRANSACTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DDE_ENABLE_CALLBACK_CMD(pub u32);
impl windows_core::TypeKind for DDE_ENABLE_CALLBACK_CMD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DDE_ENABLE_CALLBACK_CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DDE_ENABLE_CALLBACK_CMD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DDE_INITIALIZE_COMMAND(pub u32);
impl windows_core::TypeKind for DDE_INITIALIZE_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DDE_INITIALIZE_COMMAND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DDE_INITIALIZE_COMMAND").field(&self.0).finish()
    }
}
impl DDE_INITIALIZE_COMMAND {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DDE_INITIALIZE_COMMAND {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DDE_INITIALIZE_COMMAND {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DDE_NAME_SERVICE_CMD(pub u32);
impl windows_core::TypeKind for DDE_NAME_SERVICE_CMD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DDE_NAME_SERVICE_CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DDE_NAME_SERVICE_CMD").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONVCONTEXT {
    pub cb: u32,
    pub wFlags: u32,
    pub wCountryID: u32,
    pub iCodePage: i32,
    pub dwLangID: u32,
    pub dwSecurity: u32,
    pub qos: super::super::Security::SECURITY_QUALITY_OF_SERVICE,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for CONVCONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for CONVCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONVINFO {
    pub cb: u32,
    pub hUser: usize,
    pub hConvPartner: HCONV,
    pub hszSvcPartner: HSZ,
    pub hszServiceReq: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub wType: DDE_CLIENT_TRANSACTION_TYPE,
    pub wStatus: CONVINFO_STATUS,
    pub wConvst: CONVINFO_CONVERSATION_STATE,
    pub wLastError: u32,
    pub hConvList: HCONVLIST,
    pub ConvCtxt: CONVCONTEXT,
    pub hwnd: super::super::Foundation::HWND,
    pub hwndPartner: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for CONVINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for CONVINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COPYDATASTRUCT {
    pub dwData: usize,
    pub cbData: u32,
    pub lpData: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for COPYDATASTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for COPYDATASTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEACK {
    pub _bitfield: u16,
}
impl windows_core::TypeKind for DDEACK {
    type TypeKind = windows_core::CopyType;
}
impl Default for DDEACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEADVISE {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl windows_core::TypeKind for DDEADVISE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DDEADVISE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEDATA {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl windows_core::TypeKind for DDEDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DDEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDELN {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl windows_core::TypeKind for DDELN {
    type TypeKind = windows_core::CopyType;
}
impl Default for DDELN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEML_MSG_HOOK_DATA {
    pub uiLo: usize,
    pub uiHi: usize,
    pub cbData: u32,
    pub Data: [u32; 8],
}
impl windows_core::TypeKind for DDEML_MSG_HOOK_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DDEML_MSG_HOOK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEPOKE {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl windows_core::TypeKind for DDEPOKE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DDEPOKE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEUP {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub rgb: [u8; 1],
}
impl windows_core::TypeKind for DDEUP {
    type TypeKind = windows_core::CopyType;
}
impl Default for DDEUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCONV(pub *mut core::ffi::c_void);
impl HCONV {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HCONV {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = DdeDisconnect(*self);
        }
    }
}
impl Default for HCONV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCONV {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCONVLIST(pub *mut core::ffi::c_void);
impl HCONVLIST {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HCONVLIST {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = DdeDisconnectList(*self);
        }
    }
}
impl Default for HCONVLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCONVLIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDDEDATA(pub *mut core::ffi::c_void);
impl HDDEDATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for HDDEDATA {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = DdeFreeDataHandle(*self);
        }
    }
}
impl Default for HDDEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDDEDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSZ(pub *mut core::ffi::c_void);
impl HSZ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HSZ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HSZ {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSZPAIR {
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
}
impl windows_core::TypeKind for HSZPAIR {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSZPAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct METAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub hMF: super::super::Graphics::Gdi::HMETAFILE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for METAFILEPICT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for METAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONCBSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub dwRet: u32,
    pub wType: u32,
    pub wFmt: u32,
    pub hConv: HCONV,
    pub hsz1: HSZ,
    pub hsz2: HSZ,
    pub hData: HDDEDATA,
    pub dwData1: usize,
    pub dwData2: usize,
    pub cc: CONVCONTEXT,
    pub cbData: u32,
    pub Data: [u32; 8],
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for MONCBSTRUCT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for MONCBSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONCONVSTRUCT {
    pub cb: u32,
    pub fConnect: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hConvClient: HCONV,
    pub hConvServer: HCONV,
}
impl windows_core::TypeKind for MONCONVSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONCONVSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONERRSTRUCT {
    pub cb: u32,
    pub wLastError: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for MONERRSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONERRSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONHSZSTRUCTA {
    pub cb: u32,
    pub fsAction: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::super::Foundation::HANDLE,
    pub str: [i8; 1],
}
impl windows_core::TypeKind for MONHSZSTRUCTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONHSZSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONHSZSTRUCTW {
    pub cb: u32,
    pub fsAction: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::super::Foundation::HANDLE,
    pub str: [u16; 1],
}
impl windows_core::TypeKind for MONHSZSTRUCTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONHSZSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONLINKSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub fEstablished: super::super::Foundation::BOOL,
    pub fNoData: super::super::Foundation::BOOL,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub fServer: super::super::Foundation::BOOL,
    pub hConvServer: HCONV,
    pub hConvClient: HCONV,
}
impl windows_core::TypeKind for MONLINKSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONLINKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONMSGSTRUCT {
    pub cb: u32,
    pub hwndTo: super::super::Foundation::HWND,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub wMsg: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub dmhd: DDEML_MSG_HOOK_DATA,
}
impl windows_core::TypeKind for MONMSGSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONMSGSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFNCALLBACK = Option<unsafe extern "system" fn(wtype: u32, wfmt: u32, hconv: HCONV, hsz1: HSZ, hsz2: HSZ, hdata: HDDEDATA, dwdata1: usize, dwdata2: usize) -> HDDEDATA>;

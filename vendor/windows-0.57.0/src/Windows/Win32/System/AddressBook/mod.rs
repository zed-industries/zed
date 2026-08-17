#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn BuildDisplayTable<P0, P1>(lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpmalloc: P0, hinstance: P1, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut Option<IMAPITable>, lpptbldata: *mut Option<ITableData>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IMalloc>,
    P1: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("mapi32.dll" "system" fn BuildDisplayTable(lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpmalloc : * mut core::ffi::c_void, hinstance : super::super::Foundation:: HINSTANCE, cpages : u32, lppage : *mut DTPAGE, ulflags : u32, lpptable : *mut * mut core::ffi::c_void, lpptbldata : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    BuildDisplayTable(lpallocatebuffer, lpallocatemore, lpfreebuffer, lpmalloc.param().abi(), hinstance.param().abi(), cpages, lppage, ulflags, core::mem::transmute(lpptable), core::mem::transmute(lpptbldata)).ok()
}
#[inline]
pub unsafe fn ChangeIdleRoutine(ftg: *mut core::ffi::c_void, lpfnidle: PFNIDLE, lpvidleparam: *mut core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16) {
    windows_targets::link!("mapi32.dll" "system" fn ChangeIdleRoutine(ftg : *mut core::ffi::c_void, lpfnidle : PFNIDLE, lpvidleparam : *mut core::ffi::c_void, priidle : i16, csecidle : u32, iroidle : u16, ircidle : u16));
    ChangeIdleRoutine(ftg, lpfnidle, lpvidleparam, priidle, csecidle, iroidle, ircidle)
}
#[inline]
pub unsafe fn CreateIProp(lpinterface: *mut windows_core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut core::ffi::c_void, lpppropdata: *mut Option<IPropData>) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn CreateIProp(lpinterface : *mut windows_core::GUID, lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpvreserved : *mut core::ffi::c_void, lpppropdata : *mut * mut core::ffi::c_void) -> i32);
    CreateIProp(lpinterface, lpallocatebuffer, lpallocatemore, lpfreebuffer, lpvreserved, core::mem::transmute(lpppropdata))
}
#[inline]
pub unsafe fn CreateTable(lpinterface: *mut windows_core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut Option<ITableData>) -> i32 {
    windows_targets::link!("rtm.dll" "system" fn CreateTable(lpinterface : *mut windows_core::GUID, lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpvreserved : *mut core::ffi::c_void, ultabletype : u32, ulproptagindexcolumn : u32, lpsproptagarraycolumns : *mut SPropTagArray, lpptabledata : *mut * mut core::ffi::c_void) -> i32);
    CreateTable(lpinterface, lpallocatebuffer, lpallocatemore, lpfreebuffer, lpvreserved, ultabletype, ulproptagindexcolumn, lpsproptagarraycolumns, core::mem::transmute(lpptabledata))
}
#[inline]
pub unsafe fn DeinitMapiUtil() {
    windows_targets::link!("mapi32.dll" "system" fn DeinitMapiUtil());
    DeinitMapiUtil()
}
#[inline]
pub unsafe fn DeregisterIdleRoutine(ftg: *mut core::ffi::c_void) {
    windows_targets::link!("mapi32.dll" "system" fn DeregisterIdleRoutine(ftg : *mut core::ffi::c_void));
    DeregisterIdleRoutine(ftg)
}
#[inline]
pub unsafe fn EnableIdleRoutine<P0>(ftg: *mut core::ffi::c_void, fenable: P0)
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mapi32.dll" "system" fn EnableIdleRoutine(ftg : *mut core::ffi::c_void, fenable : super::super::Foundation:: BOOL));
    EnableIdleRoutine(ftg, fenable.param().abi())
}
#[inline]
pub unsafe fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> super::super::Foundation::BOOL {
    windows_targets::link!("mapi32.dll" "system" fn FEqualNames(lpname1 : *mut MAPINAMEID, lpname2 : *mut MAPINAMEID) -> super::super::Foundation:: BOOL);
    FEqualNames(lpname1, lpname2)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> super::super::Foundation::BOOL {
    windows_targets::link!("mapi32.dll" "system" fn FPropCompareProp(lpspropvalue1 : *mut SPropValue, ulrelop : u32, lpspropvalue2 : *mut SPropValue) -> super::super::Foundation:: BOOL);
    FPropCompareProp(lpspropvalue1, ulrelop, lpspropvalue2)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mapi32.dll" "system" fn FPropContainsProp(lpspropvaluedst : *mut SPropValue, lpspropvaluesrc : *mut SPropValue, ulfuzzylevel : u32) -> super::super::Foundation:: BOOL);
    FPropContainsProp(lpspropvaluedst, lpspropvaluesrc, ulfuzzylevel)
}
#[inline]
pub unsafe fn FPropExists<P0>(lpmapiprop: P0, ulproptag: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<IMAPIProp>,
{
    windows_targets::link!("mapi32.dll" "system" fn FPropExists(lpmapiprop : * mut core::ffi::c_void, ulproptag : u32) -> super::super::Foundation:: BOOL);
    FPropExists(lpmapiprop.param().abi(), ulproptag)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn FreePadrlist(lpadrlist: *mut ADRLIST) {
    windows_targets::link!("mapi32.dll" "system" fn FreePadrlist(lpadrlist : *mut ADRLIST));
    FreePadrlist(lpadrlist)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn FreeProws(lprows: *mut SRowSet) {
    windows_targets::link!("mapi32.dll" "system" fn FreeProws(lprows : *mut SRowSet));
    FreeProws(lprows)
}
#[inline]
pub unsafe fn FtAddFt(ftaddend1: super::super::Foundation::FILETIME, ftaddend2: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    windows_targets::link!("mapi32.dll" "system" fn FtAddFt(ftaddend1 : super::super::Foundation:: FILETIME, ftaddend2 : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
    FtAddFt(core::mem::transmute(ftaddend1), core::mem::transmute(ftaddend2))
}
#[inline]
pub unsafe fn FtMulDw(ftmultiplier: u32, ftmultiplicand: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    windows_targets::link!("mapi32.dll" "system" fn FtMulDw(ftmultiplier : u32, ftmultiplicand : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
    FtMulDw(ftmultiplier, core::mem::transmute(ftmultiplicand))
}
#[inline]
pub unsafe fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME {
    windows_targets::link!("mapi32.dll" "system" fn FtMulDwDw(ftmultiplicand : u32, ftmultiplier : u32) -> super::super::Foundation:: FILETIME);
    FtMulDwDw(ftmultiplicand, ftmultiplier)
}
#[inline]
pub unsafe fn FtNegFt(ft: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    windows_targets::link!("mapi32.dll" "system" fn FtNegFt(ft : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
    FtNegFt(core::mem::transmute(ft))
}
#[inline]
pub unsafe fn FtSubFt(ftminuend: super::super::Foundation::FILETIME, ftsubtrahend: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    windows_targets::link!("mapi32.dll" "system" fn FtSubFt(ftminuend : super::super::Foundation:: FILETIME, ftsubtrahend : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
    FtSubFt(core::mem::transmute(ftminuend), core::mem::transmute(ftsubtrahend))
}
#[inline]
pub unsafe fn FtgRegisterIdleRoutine(lpfnidle: PFNIDLE, lpvidleparam: *mut core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut core::ffi::c_void {
    windows_targets::link!("mapi32.dll" "system" fn FtgRegisterIdleRoutine(lpfnidle : PFNIDLE, lpvidleparam : *mut core::ffi::c_void, priidle : i16, csecidle : u32, iroidle : u16) -> *mut core::ffi::c_void);
    FtgRegisterIdleRoutine(lpfnidle, lpvidleparam, priidle, csecidle, iroidle)
}
#[inline]
pub unsafe fn HrAddColumns<P0>(lptbl: P0, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPITable>,
{
    windows_targets::link!("mapi32.dll" "system" fn HrAddColumns(lptbl : * mut core::ffi::c_void, lpproptagcolumnsnew : *mut SPropTagArray, lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER) -> windows_core::HRESULT);
    HrAddColumns(lptbl.param().abi(), lpproptagcolumnsnew, lpallocatebuffer, lpfreebuffer).ok()
}
#[inline]
pub unsafe fn HrAddColumnsEx<P0>(lptbl: P0, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, lpfnfiltercolumns: isize) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPITable>,
{
    windows_targets::link!("mapi32.dll" "system" fn HrAddColumnsEx(lptbl : * mut core::ffi::c_void, lpproptagcolumnsnew : *mut SPropTagArray, lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER, lpfnfiltercolumns : isize) -> windows_core::HRESULT);
    HrAddColumnsEx(lptbl.param().abi(), lpproptagcolumnsnew, lpallocatebuffer, lpfreebuffer, lpfnfiltercolumns).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn HrAllocAdviseSink(lpfncallback: LPNOTIFCALLBACK, lpvcontext: *mut core::ffi::c_void, lppadvisesink: *mut Option<IMAPIAdviseSink>) -> windows_core::Result<()> {
    windows_targets::link!("mapi32.dll" "system" fn HrAllocAdviseSink(lpfncallback : LPNOTIFCALLBACK, lpvcontext : *mut core::ffi::c_void, lppadvisesink : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    HrAllocAdviseSink(lpfncallback, lpvcontext, core::mem::transmute(lppadvisesink)).ok()
}
#[inline]
pub unsafe fn HrDispatchNotifications(ulflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("mapi32.dll" "system" fn HrDispatchNotifications(ulflags : u32) -> windows_core::HRESULT);
    HrDispatchNotifications(ulflags).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn HrGetOneProp<P0>(lpmapiprop: P0, ulproptag: u32, lppprop: *mut *mut SPropValue) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPIProp>,
{
    windows_targets::link!("mapi32.dll" "system" fn HrGetOneProp(lpmapiprop : * mut core::ffi::c_void, ulproptag : u32, lppprop : *mut *mut SPropValue) -> windows_core::HRESULT);
    HrGetOneProp(lpmapiprop.param().abi(), ulproptag, lppprop).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn HrIStorageFromStream<P0>(lpunkin: P0, lpinterface: *mut windows_core::GUID, ulflags: u32, lppstorageout: *mut Option<super::Com::StructuredStorage::IStorage>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("mapi32.dll" "system" fn HrIStorageFromStream(lpunkin : * mut core::ffi::c_void, lpinterface : *mut windows_core::GUID, ulflags : u32, lppstorageout : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    HrIStorageFromStream(lpunkin.param().abi(), lpinterface, ulflags, core::mem::transmute(lppstorageout)).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn HrQueryAllRows<P0>(lptable: P0, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPITable>,
{
    windows_targets::link!("mapi32.dll" "system" fn HrQueryAllRows(lptable : * mut core::ffi::c_void, lpproptags : *mut SPropTagArray, lprestriction : *mut SRestriction, lpsortorderset : *mut SSortOrderSet, crowsmax : i32, lpprows : *mut *mut SRowSet) -> windows_core::HRESULT);
    HrQueryAllRows(lptable.param().abi(), lpproptags, lprestriction, lpsortorderset, crowsmax, lpprows).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn HrSetOneProp<P0>(lpmapiprop: P0, lpprop: *mut SPropValue) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPIProp>,
{
    windows_targets::link!("mapi32.dll" "system" fn HrSetOneProp(lpmapiprop : * mut core::ffi::c_void, lpprop : *mut SPropValue) -> windows_core::HRESULT);
    HrSetOneProp(lpmapiprop.param().abi(), lpprop).ok()
}
#[inline]
pub unsafe fn HrThisThreadAdviseSink<P0>(lpadvisesink: P0) -> windows_core::Result<IMAPIAdviseSink>
where
    P0: windows_core::Param<IMAPIAdviseSink>,
{
    windows_targets::link!("mapi32.dll" "system" fn HrThisThreadAdviseSink(lpadvisesink : * mut core::ffi::c_void, lppadvisesink : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    HrThisThreadAdviseSink(lpadvisesink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn LPropCompareProp(lpspropvaluea : *mut SPropValue, lpspropvalueb : *mut SPropValue) -> i32);
    LPropCompareProp(lpspropvaluea, lpspropvalueb)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue {
    windows_targets::link!("mapi32.dll" "system" fn LpValFindProp(ulproptag : u32, cvalues : u32, lpproparray : *mut SPropValue) -> *mut SPropValue);
    LpValFindProp(ulproptag, cvalues, lpproparray)
}
#[inline]
pub unsafe fn MAPIDeinitIdle() {
    windows_targets::link!("mapi32.dll" "system" fn MAPIDeinitIdle());
    MAPIDeinitIdle()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn MAPIGetDefaultMalloc() -> Option<super::Com::IMalloc> {
    windows_targets::link!("mapi32.dll" "system" fn MAPIGetDefaultMalloc() -> Option < super::Com:: IMalloc >);
    MAPIGetDefaultMalloc()
}
#[inline]
pub unsafe fn MAPIInitIdle(lpvreserved: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn MAPIInitIdle(lpvreserved : *mut core::ffi::c_void) -> i32);
    MAPIInitIdle(lpvreserved)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenStreamOnFile(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: Option<*const i8>) -> windows_core::Result<super::Com::IStream> {
    windows_targets::link!("mapi32.dll" "system" fn OpenStreamOnFile(lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER, ulflags : u32, lpszfilename : *const i8, lpszprefix : *const i8, lppstream : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OpenStreamOnFile(lpallocatebuffer, lpfreebuffer, ulflags, lpszfilename, core::mem::transmute(lpszprefix.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue {
    windows_targets::link!("mapi32.dll" "system" fn PpropFindProp(lpproparray : *mut SPropValue, cvalues : u32, ulproptag : u32) -> *mut SPropValue);
    PpropFindProp(lpproparray, cvalues, ulproptag)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: LPALLOCATEMORE, lpvobject: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn PropCopyMore(lpspropvaluedest : *mut SPropValue, lpspropvaluesrc : *mut SPropValue, lpfallocmore : LPALLOCATEMORE, lpvobject : *mut core::ffi::c_void) -> i32);
    PropCopyMore(lpspropvaluedest, lpspropvaluesrc, lpfallocmore, lpvobject)
}
#[inline]
pub unsafe fn RTFSync<P0>(lpmessage: P0, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMessage>,
{
    windows_targets::link!("mapi32.dll" "system" fn RTFSync(lpmessage : * mut core::ffi::c_void, ulflags : u32, lpfmessageupdated : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    RTFSync(lpmessage.param().abi(), ulflags, lpfmessageupdated).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut core::ffi::c_void, lpcb: *mut u32) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScCopyNotifications(cnotification : i32, lpnotifications : *mut NOTIFICATION, lpvdst : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
    ScCopyNotifications(cnotification, lpnotifications, lpvdst, lpcb)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut core::ffi::c_void, lpcb: *mut u32) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScCopyProps(cvalues : i32, lpproparray : *mut SPropValue, lpvdst : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
    ScCopyProps(cvalues, lpproparray, lpvdst, lpcb)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScCountNotifications(cnotifications : i32, lpnotifications : *mut NOTIFICATION, lpcb : *mut u32) -> i32);
    ScCountNotifications(cnotifications, lpnotifications, lpcb)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScCountProps(cvalues : i32, lpproparray : *mut SPropValue, lpcb : *mut u32) -> i32);
    ScCountProps(cvalues, lpproparray, lpcb)
}
#[inline]
pub unsafe fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScCreateConversationIndex(cbparent : u32, lpbparent : *mut u8, lpcbconvindex : *mut u32, lppbconvindex : *mut *mut u8) -> i32);
    ScCreateConversationIndex(cbparent, lpbparent, lpcbconvindex, lppbconvindex)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: LPALLOCATEBUFFER, lppproparray: *mut *mut SPropValue) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScDupPropset(cvalues : i32, lpproparray : *mut SPropValue, lpallocatebuffer : LPALLOCATEBUFFER, lppproparray : *mut *mut SPropValue) -> i32);
    ScDupPropset(cvalues, lpproparray, lpallocatebuffer, lppproparray)
}
#[inline]
pub unsafe fn ScInitMapiUtil(ulflags: u32) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScInitMapiUtil(ulflags : u32) -> i32);
    ScInitMapiUtil(ulflags)
}
#[inline]
pub unsafe fn ScLocalPathFromUNC<P0>(lpszunc: P0, lpszlocal: &[u8]) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mapi32.dll" "system" fn ScLocalPathFromUNC(lpszunc : windows_core::PCSTR, lpszlocal : windows_core::PCSTR, cchlocal : u32) -> i32);
    ScLocalPathFromUNC(lpszunc.param().abi(), core::mem::transmute(lpszlocal.as_ptr()), lpszlocal.len().try_into().unwrap())
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut core::ffi::c_void, lpvbasenew: *mut core::ffi::c_void, lpcb: *mut u32) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScRelocNotifications(cnotification : i32, lpnotifications : *mut NOTIFICATION, lpvbaseold : *mut core::ffi::c_void, lpvbasenew : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
    ScRelocNotifications(cnotification, lpnotifications, lpvbaseold, lpvbasenew, lpcb)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut core::ffi::c_void, lpvbasenew: *mut core::ffi::c_void, lpcb: *mut u32) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn ScRelocProps(cvalues : i32, lpproparray : *mut SPropValue, lpvbaseold : *mut core::ffi::c_void, lpvbasenew : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
    ScRelocProps(cvalues, lpproparray, lpvbaseold, lpvbasenew, lpcb)
}
#[inline]
pub unsafe fn ScUNCFromLocalPath<P0>(lpszlocal: P0, lpszunc: &[u8]) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mapi32.dll" "system" fn ScUNCFromLocalPath(lpszlocal : windows_core::PCSTR, lpszunc : windows_core::PCSTR, cchunc : u32) -> i32);
    ScUNCFromLocalPath(lpszlocal.param().abi(), core::mem::transmute(lpszunc.as_ptr()), lpszunc.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8 {
    windows_targets::link!("mapi32.dll" "system" fn SzFindCh(lpsz : *mut i8, ch : u16) -> *mut i8);
    SzFindCh(lpsz, ch)
}
#[inline]
pub unsafe fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8 {
    windows_targets::link!("mapi32.dll" "system" fn SzFindLastCh(lpsz : *mut i8, ch : u16) -> *mut i8);
    SzFindLastCh(lpsz, ch)
}
#[inline]
pub unsafe fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8 {
    windows_targets::link!("mapi32.dll" "system" fn SzFindSz(lpsz : *mut i8, lpszkey : *mut i8) -> *mut i8);
    SzFindSz(lpsz, lpszkey)
}
#[inline]
pub unsafe fn UFromSz(lpsz: *mut i8) -> u32 {
    windows_targets::link!("mapi32.dll" "system" fn UFromSz(lpsz : *mut i8) -> u32);
    UFromSz(lpsz)
}
#[inline]
pub unsafe fn UlAddRef(lpunk: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("mapi32.dll" "system" fn UlAddRef(lpunk : *mut core::ffi::c_void) -> u32);
    UlAddRef(lpunk)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32 {
    windows_targets::link!("mapi32.dll" "system" fn UlPropSize(lpspropvalue : *mut SPropValue) -> u32);
    UlPropSize(lpspropvalue)
}
#[inline]
pub unsafe fn UlRelease(lpunk: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("mapi32.dll" "system" fn UlRelease(lpunk : *mut core::ffi::c_void) -> u32);
    UlRelease(lpunk)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WrapCompressedRTFStream<P0>(lpcompressedrtfstream: P0, ulflags: u32) -> windows_core::Result<super::Com::IStream>
where
    P0: windows_core::Param<super::Com::IStream>,
{
    windows_targets::link!("mapi32.dll" "system" fn WrapCompressedRTFStream(lpcompressedrtfstream : * mut core::ffi::c_void, ulflags : u32, lpuncompressedrtfstream : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WrapCompressedRTFStream(lpcompressedrtfstream.param().abi(), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> windows_core::Result<()> {
    windows_targets::link!("mapi32.dll" "system" fn WrapStoreEntryID(ulflags : u32, lpszdllname : *const i8, cborigentry : u32, lporigentry : *const ENTRYID, lpcbwrappedentry : *mut u32, lppwrappedentry : *mut *mut ENTRYID) -> windows_core::HRESULT);
    WrapStoreEntryID(ulflags, lpszdllname, cborigentry, lporigentry, lpcbwrappedentry, lppwrappedentry).ok()
}
windows_core::imp::define_interface!(IABContainer, IABContainer_Vtbl, 0);
impl core::ops::Deref for IABContainer {
    type Target = IMAPIContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IABContainer, windows_core::IUnknown, IMAPIProp, IMAPIContainer);
impl IABContainer {
    pub unsafe fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> windows_core::Result<IMAPIProp> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulcreateflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CopyEntries<P0>(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).CopyEntries)(windows_core::Interface::as_raw(self), lpentries, uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
    pub unsafe fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteEntries)(windows_core::Interface::as_raw(self), lpentries, ulflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResolveNames(&self, lpproptagarray: Option<*const SPropTagArray>, ulflags: u32, lpadrlist: *const ADRLIST) -> windows_core::Result<FlagList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResolveNames)(windows_core::Interface::as_raw(self), core::mem::transmute(lpproptagarray.unwrap_or(std::ptr::null())), ulflags, lpadrlist, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IABContainer_Vtbl {
    pub base__: IMAPIContainer_Vtbl,
    pub CreateEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const SBinaryArray, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const SBinaryArray, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResolveNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPropTagArray, u32, *const ADRLIST, *mut FlagList) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResolveNames: usize,
}
windows_core::imp::define_interface!(IAddrBook, IAddrBook_Vtbl, 0);
impl core::ops::Deref for IAddrBook {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAddrBook, windows_core::IUnknown, IMAPIProp);
impl IAddrBook {
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, lpinterface, ulflags, lpulobjtype, core::mem::transmute(lppunk)).ok()
    }
    pub unsafe fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CompareEntryIDs)(windows_core::Interface::as_raw(self), cbentryid1, lpentryid1, cbentryid2, lpentryid2, ulflags, lpulresult).ok()
    }
    pub unsafe fn Advise<P0>(&self, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: P0, lpulconnection: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIAdviseSink>,
    {
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, uleventmask, lpadvisesink.param().abi(), lpulconnection).ok()
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), ulconnection).ok()
    }
    pub unsafe fn CreateOneOff(&self, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateOneOff)(windows_core::Interface::as_raw(self), lpszname, lpszadrtype, lpszaddress, ulflags, lpcbentryid, lppentryid).ok()
    }
    pub unsafe fn NewEntry(&self, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NewEntry)(windows_core::Interface::as_raw(self), uluiparam, ulflags, cbeidcontainer, lpeidcontainer, cbeidnewentrytpl, lpeidnewentrytpl, lpcbeidnewentry, lppeidnewentry).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResolveName(&self, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResolveName)(windows_core::Interface::as_raw(self), uluiparam, ulflags, lpsznewentrytitle, lpadrlist).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), lpuluiparam, lpadrparms, lppadrlist).ok()
    }
    pub unsafe fn Details(&self, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Details)(windows_core::Interface::as_raw(self), lpuluiparam, lpfndismiss, lpvdismisscontext, cbentryid, lpentryid, lpfbuttoncallback, lpvbuttoncontext, lpszbuttontext, ulflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecipOptions(&self, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecipOptions)(windows_core::Interface::as_raw(self), uluiparam, ulflags, lprecip).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDefaultRecipOpt(&self, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryDefaultRecipOpt)(windows_core::Interface::as_raw(self), lpszadrtype, ulflags, lpcvalues, lppoptions).ok()
    }
    pub unsafe fn GetPAB(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPAB)(windows_core::Interface::as_raw(self), lpcbentryid, lppentryid).ok()
    }
    pub unsafe fn SetPAB(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPAB)(windows_core::Interface::as_raw(self), cbentryid, lpentryid).ok()
    }
    pub unsafe fn GetDefaultDir(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultDir)(windows_core::Interface::as_raw(self), lpcbentryid, lppentryid).ok()
    }
    pub unsafe fn SetDefaultDir(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultDir)(windows_core::Interface::as_raw(self), cbentryid, lpentryid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSearchPath(&self, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSearchPath)(windows_core::Interface::as_raw(self), ulflags, lppsearchpath).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSearchPath(&self, ulflags: u32, lpsearchpath: *mut SRowSet) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSearchPath)(windows_core::Interface::as_raw(self), ulflags, lpsearchpath).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRecips(&self, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrepareRecips)(windows_core::Interface::as_raw(self), ulflags, lpproptagarray, lpreciplist).ok()
    }
}
#[repr(C)]
pub struct IAddrBook_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub OpenEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ENTRYID, *mut windows_core::GUID, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompareEntryIDs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ENTRYID, u32, *mut ENTRYID, u32, *mut u32) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ENTRYID, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateOneOff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i8, *mut i8, *mut i8, u32, *mut u32, *mut *mut ENTRYID) -> windows_core::HRESULT,
    pub NewEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut ENTRYID, u32, *mut ENTRYID, *mut u32, *mut *mut ENTRYID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResolveName: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut i8, *mut ADRLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResolveName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut ADRPARM, *mut *mut ADRLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub Details: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, LPFNDISMISS, *mut core::ffi::c_void, u32, *mut ENTRYID, LPFNBUTTON, *mut core::ffi::c_void, *mut i8, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RecipOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut ADRENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecipOptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDefaultRecipOpt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i8, u32, *mut u32, *mut *mut SPropValue) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDefaultRecipOpt: usize,
    pub GetPAB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut ENTRYID) -> windows_core::HRESULT,
    pub SetPAB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ENTRYID) -> windows_core::HRESULT,
    pub GetDefaultDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut ENTRYID) -> windows_core::HRESULT,
    pub SetDefaultDir: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ENTRYID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSearchPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut SRowSet) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSearchPath: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSearchPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SRowSet) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSearchPath: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRecips: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SPropTagArray, *mut ADRLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRecips: usize,
}
windows_core::imp::define_interface!(IAttach, IAttach_Vtbl, 0);
impl core::ops::Deref for IAttach {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAttach, windows_core::IUnknown, IMAPIProp);
impl IAttach {}
#[repr(C)]
pub struct IAttach_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
windows_core::imp::define_interface!(IDistList, IDistList_Vtbl, 0);
impl core::ops::Deref for IDistList {
    type Target = IMAPIContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDistList, windows_core::IUnknown, IMAPIProp, IMAPIContainer);
impl IDistList {
    pub unsafe fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> windows_core::Result<IMAPIProp> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulcreateflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CopyEntries<P0>(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).CopyEntries)(windows_core::Interface::as_raw(self), lpentries, uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
    pub unsafe fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteEntries)(windows_core::Interface::as_raw(self), lpentries, ulflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResolveNames(&self, lpproptagarray: Option<*const SPropTagArray>, ulflags: u32, lpadrlist: *const ADRLIST) -> windows_core::Result<FlagList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResolveNames)(windows_core::Interface::as_raw(self), core::mem::transmute(lpproptagarray.unwrap_or(std::ptr::null())), ulflags, lpadrlist, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDistList_Vtbl {
    pub base__: IMAPIContainer_Vtbl,
    pub CreateEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const SBinaryArray, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const SBinaryArray, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResolveNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPropTagArray, u32, *const ADRLIST, *mut FlagList) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResolveNames: usize,
}
windows_core::imp::define_interface!(IMAPIAdviseSink, IMAPIAdviseSink_Vtbl, 0);
impl core::ops::Deref for IMAPIAdviseSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPIAdviseSink, windows_core::IUnknown);
impl IMAPIAdviseSink {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnNotify(&self, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32 {
        (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), cnotif, lpnotifications)
    }
}
#[repr(C)]
pub struct IMAPIAdviseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut NOTIFICATION) -> u32,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnNotify: usize,
}
windows_core::imp::define_interface!(IMAPIContainer, IMAPIContainer_Vtbl, 0);
impl core::ops::Deref for IMAPIContainer {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPIContainer, windows_core::IUnknown, IMAPIProp);
impl IMAPIContainer {
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentsTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHierarchyTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, lpinterface, ulflags, lpulobjtype, core::mem::transmute(lppunk)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: Option<*const SRestriction>, lpcontainerlist: Option<*const SBinaryArray>, ulsearchflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSearchCriteria)(windows_core::Interface::as_raw(self), core::mem::transmute(lprestriction.unwrap_or(std::ptr::null())), core::mem::transmute(lpcontainerlist.unwrap_or(std::ptr::null())), ulsearchflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSearchCriteria)(windows_core::Interface::as_raw(self), ulflags, lpprestriction, lppcontainerlist, core::mem::transmute(lpulsearchstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMAPIContainer_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub GetContentsTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHierarchyTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, *mut windows_core::GUID, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSearchCriteria: unsafe extern "system" fn(*mut core::ffi::c_void, *const SRestriction, *const SBinaryArray, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSearchCriteria: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSearchCriteria: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut SRestriction, *mut *mut SBinaryArray, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSearchCriteria: usize,
}
windows_core::imp::define_interface!(IMAPIControl, IMAPIControl_Vtbl, 0);
impl core::ops::Deref for IMAPIControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPIControl, windows_core::IUnknown);
impl IMAPIControl {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32) -> windows_core::Result<*mut MAPIERROR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, &mut result__).map(|| result__)
    }
    pub unsafe fn Activate(&self, ulflags: u32, uluiparam: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), ulflags, uluiparam).ok()
    }
    pub unsafe fn GetState(&self, ulflags: u32, lpulstate: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), ulflags, lpulstate).ok()
    }
}
#[repr(C)]
pub struct IMAPIControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *mut *mut MAPIERROR) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMAPIFolder, IMAPIFolder_Vtbl, 0);
impl core::ops::Deref for IMAPIFolder {
    type Target = IMAPIContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPIFolder, windows_core::IUnknown, IMAPIProp, IMAPIContainer);
impl IMAPIFolder {
    pub unsafe fn CreateMessage(&self, lpinterface: *mut windows_core::GUID, ulflags: u32, lppmessage: *mut Option<IMessage>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateMessage)(windows_core::Interface::as_raw(self), lpinterface, ulflags, core::mem::transmute(lppmessage)).ok()
    }
    pub unsafe fn CopyMessages<P0>(&self, lpmsglist: *const SBinaryArray, lpinterface: Option<*const windows_core::GUID>, lpdestfolder: *const core::ffi::c_void, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).CopyMessages)(windows_core::Interface::as_raw(self), lpmsglist, core::mem::transmute(lpinterface.unwrap_or(std::ptr::null())), lpdestfolder, uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
    pub unsafe fn DeleteMessages<P0>(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).DeleteMessages)(windows_core::Interface::as_raw(self), lpmsglist, uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
    pub unsafe fn CreateFolder(&self, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: Option<*const i8>, lpinterface: Option<*const windows_core::GUID>, ulflags: u32) -> windows_core::Result<IMAPIFolder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFolder)(windows_core::Interface::as_raw(self), ulfoldertype, lpszfoldername, core::mem::transmute(lpszfoldercomment.unwrap_or(std::ptr::null())), core::mem::transmute(lpinterface.unwrap_or(std::ptr::null())), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CopyFolder<P0>(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: Option<*const windows_core::GUID>, lpdestfolder: *const core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).CopyFolder)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, core::mem::transmute(lpinterface.unwrap_or(std::ptr::null())), lpdestfolder, lpsznewfoldername, uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
    pub unsafe fn DeleteFolder<P0>(&self, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).DeleteFolder)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
    pub unsafe fn SetReadFlags<P0>(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).SetReadFlags)(windows_core::Interface::as_raw(self), lpmsglist, uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
    pub unsafe fn GetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMessageStatus)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulflags, &mut result__).map(|| result__)
    }
    pub unsafe fn SetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetMessageStatus)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulnewstatus, ulnewstatusmask, &mut result__).map(|| result__)
    }
    pub unsafe fn SaveContentsSort(&self, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveContentsSort)(windows_core::Interface::as_raw(self), lpsortcriteria, ulflags).ok()
    }
    pub unsafe fn EmptyFolder<P0>(&self, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).EmptyFolder)(windows_core::Interface::as_raw(self), uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
}
#[repr(C)]
pub struct IMAPIFolder_Vtbl {
    pub base__: IMAPIContainer_Vtbl,
    pub CreateMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *const SBinaryArray, *const windows_core::GUID, *const core::ffi::c_void, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *const SBinaryArray, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateFolder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i8, *const i8, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyFolder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, *const windows_core::GUID, *const core::ffi::c_void, *const i8, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteFolder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetReadFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *const SBinaryArray, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMessageStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, u32, *mut u32) -> windows_core::HRESULT,
    pub SetMessageStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SaveContentsSort: unsafe extern "system" fn(*mut core::ffi::c_void, *const SSortOrderSet, u32) -> windows_core::HRESULT,
    pub EmptyFolder: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMAPIProgress, IMAPIProgress_Vtbl, 0);
impl core::ops::Deref for IMAPIProgress {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPIProgress, windows_core::IUnknown);
impl IMAPIProgress {
    pub unsafe fn Progress(&self, ulvalue: u32, ulcount: u32, ultotal: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), ulvalue, ulcount, ultotal).ok()
    }
    pub unsafe fn GetFlags(&self, lpulflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), lpulflags).ok()
    }
    pub unsafe fn GetMax(&self, lpulmax: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMax)(windows_core::Interface::as_raw(self), lpulmax).ok()
    }
    pub unsafe fn GetMin(&self, lpulmin: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMin)(windows_core::Interface::as_raw(self), lpulmin).ok()
    }
    pub unsafe fn SetLimits(&self, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLimits)(windows_core::Interface::as_raw(self), lpulmin, lpulmax, lpulflags).ok()
    }
}
#[repr(C)]
pub struct IMAPIProgress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMAPIProp, IMAPIProp_Vtbl, 0);
impl core::ops::Deref for IMAPIProp {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPIProp, windows_core::IUnknown);
impl IMAPIProp {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProps)(windows_core::Interface::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropList)(windows_core::Interface::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut windows_core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenProperty)(windows_core::Interface::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, core::mem::transmute(lppunk)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProps)(windows_core::Interface::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteProps)(windows_core::Interface::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut windows_core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.param().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).CopyProps)(windows_core::Interface::as_raw(self), lpincludeprops, uluiparam, lpprogress.param().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut windows_core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNamesFromIDs)(windows_core::Interface::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIDsFromNames)(windows_core::Interface::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
#[repr(C)]
pub struct IMAPIProp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *mut *mut MAPIERROR) -> windows_core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropTagArray, u32, *mut u32, *mut *mut SPropValue) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProps: usize,
    pub GetPropList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut SPropTagArray) -> windows_core::HRESULT,
    pub OpenProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SPropValue, *mut *mut SPropProblemArray) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetProps: usize,
    pub DeleteProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropTagArray, *mut *mut SPropProblemArray) -> windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut SPropTagArray, usize, *mut core::ffi::c_void, *mut windows_core::GUID, *mut core::ffi::c_void, u32, *mut *mut SPropProblemArray) -> windows_core::HRESULT,
    pub CopyProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropTagArray, usize, *mut core::ffi::c_void, *mut windows_core::GUID, *mut core::ffi::c_void, u32, *mut *mut SPropProblemArray) -> windows_core::HRESULT,
    pub GetNamesFromIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SPropTagArray, *mut windows_core::GUID, u32, *mut u32, *mut *mut *mut MAPINAMEID) -> windows_core::HRESULT,
    pub GetIDsFromNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut MAPINAMEID, u32, *mut *mut SPropTagArray) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMAPIStatus, IMAPIStatus_Vtbl, 0);
impl core::ops::Deref for IMAPIStatus {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPIStatus, windows_core::IUnknown, IMAPIProp);
impl IMAPIStatus {
    pub unsafe fn ValidateState(&self, uluiparam: usize, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ValidateState)(windows_core::Interface::as_raw(self), uluiparam, ulflags).ok()
    }
    pub unsafe fn SettingsDialog(&self, uluiparam: usize, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SettingsDialog)(windows_core::Interface::as_raw(self), uluiparam, ulflags).ok()
    }
    pub unsafe fn ChangePassword(&self, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangePassword)(windows_core::Interface::as_raw(self), lpoldpass, lpnewpass, ulflags).ok()
    }
    pub unsafe fn FlushQueues(&self, uluiparam: usize, lptargettransport: Option<&[ENTRYID]>, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FlushQueues)(windows_core::Interface::as_raw(self), uluiparam, lptargettransport.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lptargettransport.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ulflags).ok()
    }
}
#[repr(C)]
pub struct IMAPIStatus_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub ValidateState: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub SettingsDialog: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, *const i8, u32) -> windows_core::HRESULT,
    pub FlushQueues: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const ENTRYID, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMAPITable, IMAPITable_Vtbl, 0);
impl core::ops::Deref for IMAPITable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPITable, windows_core::IUnknown);
impl IMAPITable {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn Advise<P0>(&self, uleventmask: u32, lpadvisesink: P0, lpulconnection: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIAdviseSink>,
    {
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), uleventmask, lpadvisesink.param().abi(), lpulconnection).ok()
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), ulconnection).ok()
    }
    pub unsafe fn GetStatus(&self, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), lpultablestatus, lpultabletype).ok()
    }
    pub unsafe fn SetColumns(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColumns)(windows_core::Interface::as_raw(self), lpproptagarray, ulflags).ok()
    }
    pub unsafe fn QueryColumns(&self, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryColumns)(windows_core::Interface::as_raw(self), ulflags, lpproptagarray).ok()
    }
    pub unsafe fn GetRowCount(&self, ulflags: u32, lpulcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRowCount)(windows_core::Interface::as_raw(self), ulflags, lpulcount).ok()
    }
    pub unsafe fn SeekRow(&self, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SeekRow)(windows_core::Interface::as_raw(self), bkorigin, lrowcount, lplrowssought).ok()
    }
    pub unsafe fn SeekRowApprox(&self, ulnumerator: u32, uldenominator: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SeekRowApprox)(windows_core::Interface::as_raw(self), ulnumerator, uldenominator).ok()
    }
    pub unsafe fn QueryPosition(&self, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryPosition)(windows_core::Interface::as_raw(self), lpulrow, lpulnumerator, lpuldenominator).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindRow(&self, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindRow)(windows_core::Interface::as_raw(self), lprestriction, bkorigin, ulflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Restrict(&self, lprestriction: *mut SRestriction, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Restrict)(windows_core::Interface::as_raw(self), lprestriction, ulflags).ok()
    }
    pub unsafe fn CreateBookmark(&self, lpbkposition: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateBookmark)(windows_core::Interface::as_raw(self), lpbkposition).ok()
    }
    pub unsafe fn FreeBookmark(&self, bkposition: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeBookmark)(windows_core::Interface::as_raw(self), bkposition).ok()
    }
    pub unsafe fn SortTable(&self, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SortTable)(windows_core::Interface::as_raw(self), lpsortcriteria, ulflags).ok()
    }
    pub unsafe fn QuerySortOrder(&self, lppsortcriteria: *mut *mut SSortOrderSet) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QuerySortOrder)(windows_core::Interface::as_raw(self), lppsortcriteria).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryRows(&self, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryRows)(windows_core::Interface::as_raw(self), lrowcount, ulflags, lpprows).ok()
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExpandRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExpandRow)(windows_core::Interface::as_raw(self), cbinstancekey, pbinstancekey, ulrowcount, ulflags, lpprows, lpulmorerows).ok()
    }
    pub unsafe fn CollapseRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CollapseRow)(windows_core::Interface::as_raw(self), cbinstancekey, pbinstancekey, ulflags, lpulrowcount).ok()
    }
    pub unsafe fn WaitForCompletion(&self, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), ulflags, ultimeout, lpultablestatus).ok()
    }
    pub unsafe fn GetCollapseState(&self, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCollapseState)(windows_core::Interface::as_raw(self), ulflags, cbinstancekey, lpbinstancekey, lpcbcollapsestate, lppbcollapsestate).ok()
    }
    pub unsafe fn SetCollapseState(&self, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCollapseState)(windows_core::Interface::as_raw(self), ulflags, cbcollapsestate, pbcollapsestate, lpbklocation).ok()
    }
}
#[repr(C)]
pub struct IMAPITable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *mut *mut MAPIERROR) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetColumns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropTagArray, u32) -> windows_core::HRESULT,
    pub QueryColumns: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut SPropTagArray) -> windows_core::HRESULT,
    pub GetRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SeekRow: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut i32) -> windows_core::HRESULT,
    pub SeekRowApprox: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub QueryPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FindRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SRestriction, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindRow: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Restrict: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SRestriction, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Restrict: usize,
    pub CreateBookmark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreeBookmark: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SortTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SSortOrderSet, u32) -> windows_core::HRESULT,
    pub QuerySortOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SSortOrderSet) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryRows: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut SRowSet) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryRows: usize,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpandRow: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, u32, u32, *mut *mut SRowSet, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpandRow: usize,
    pub CollapseRow: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMailUser, IMailUser_Vtbl, 0);
impl core::ops::Deref for IMailUser {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMailUser, windows_core::IUnknown, IMAPIProp);
impl IMailUser {}
#[repr(C)]
pub struct IMailUser_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
windows_core::imp::define_interface!(IMessage, IMessage_Vtbl, 0);
impl core::ops::Deref for IMessage {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMessage, windows_core::IUnknown, IMAPIProp);
impl IMessage {
    pub unsafe fn GetAttachmentTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttachmentTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenAttach(&self, ulattachmentnum: u32, lpinterface: Option<*const windows_core::GUID>, ulflags: u32) -> windows_core::Result<IAttach> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenAttach)(windows_core::Interface::as_raw(self), ulattachmentnum, core::mem::transmute(lpinterface.unwrap_or(std::ptr::null())), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAttach(&self, lpinterface: Option<*const windows_core::GUID>, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut Option<IAttach>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateAttach)(windows_core::Interface::as_raw(self), core::mem::transmute(lpinterface.unwrap_or(std::ptr::null())), ulflags, lpulattachmentnum, core::mem::transmute(lppattach)).ok()
    }
    pub unsafe fn DeleteAttach<P0>(&self, ulattachmentnum: u32, uluiparam: usize, lpprogress: P0, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMAPIProgress>,
    {
        (windows_core::Interface::vtable(self).DeleteAttach)(windows_core::Interface::as_raw(self), ulattachmentnum, uluiparam, lpprogress.param().abi(), ulflags).ok()
    }
    pub unsafe fn GetRecipientTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecipientTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModifyRecipients(&self, ulflags: u32, lpmods: *const ADRLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModifyRecipients)(windows_core::Interface::as_raw(self), ulflags, lpmods).ok()
    }
    pub unsafe fn SubmitMessage(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SubmitMessage)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn SetReadFlag(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReadFlag)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
}
#[repr(C)]
pub struct IMessage_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub GetAttachmentTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenAttach: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAttach: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAttach: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetRecipientTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifyRecipients: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ADRLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifyRecipients: usize,
    pub SubmitMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetReadFlag: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMsgStore, IMsgStore_Vtbl, 0);
impl core::ops::Deref for IMsgStore {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMsgStore, windows_core::IUnknown, IMAPIProp);
impl IMsgStore {
    pub unsafe fn Advise<P0>(&self, cbentryid: u32, lpentryid: Option<*const ENTRYID>, uleventmask: u32, lpadvisesink: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IMAPIAdviseSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), cbentryid, core::mem::transmute(lpentryid.unwrap_or(std::ptr::null())), uleventmask, lpadvisesink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), ulconnection).ok()
    }
    pub unsafe fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareEntryIDs)(windows_core::Interface::as_raw(self), cbentryid1, lpentryid1, cbentryid2, lpentryid2, ulflags, &mut result__).map(|| result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: Option<*const windows_core::GUID>, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, core::mem::transmute(lpinterface.unwrap_or(std::ptr::null())), ulflags, lpulobjtype, core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn SetReceiveFolder(&self, lpszmessageclass: Option<*const i8>, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReceiveFolder)(windows_core::Interface::as_raw(self), core::mem::transmute(lpszmessageclass.unwrap_or(std::ptr::null())), ulflags, cbentryid, lpentryid).ok()
    }
    pub unsafe fn GetReceiveFolder(&self, lpszmessageclass: Option<*const i8>, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReceiveFolder)(windows_core::Interface::as_raw(self), core::mem::transmute(lpszmessageclass.unwrap_or(std::ptr::null())), ulflags, lpcbentryid, lppentryid, lppszexplicitclass).ok()
    }
    pub unsafe fn GetReceiveFolderTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReceiveFolderTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StoreLogoff(&self, lpulflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StoreLogoff)(windows_core::Interface::as_raw(self), lpulflags).ok()
    }
    pub unsafe fn AbortSubmit(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AbortSubmit)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulflags).ok()
    }
    pub unsafe fn GetOutgoingQueue(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutgoingQueue)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLockState<P0>(&self, lpmessage: P0, ullockstate: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMessage>,
    {
        (windows_core::Interface::vtable(self).SetLockState)(windows_core::Interface::as_raw(self), lpmessage.param().abi(), ullockstate).ok()
    }
    pub unsafe fn FinishedMsg(&self, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FinishedMsg)(windows_core::Interface::as_raw(self), ulflags, cbentryid, lpentryid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NotifyNewMail(&self, lpnotification: *const NOTIFICATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyNewMail)(windows_core::Interface::as_raw(self), lpnotification).ok()
    }
}
#[repr(C)]
pub struct IMsgStore_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CompareEntryIDs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, u32, *const ENTRYID, u32, *mut u32) -> windows_core::HRESULT,
    pub OpenEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, *const windows_core::GUID, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReceiveFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, u32, u32, *const ENTRYID) -> windows_core::HRESULT,
    pub GetReceiveFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, u32, *mut u32, *mut *mut ENTRYID, *mut *mut i8) -> windows_core::HRESULT,
    pub GetReceiveFolderTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StoreLogoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AbortSubmit: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ENTRYID, u32) -> windows_core::HRESULT,
    pub GetOutgoingQueue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLockState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FinishedMsg: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const ENTRYID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NotifyNewMail: unsafe extern "system" fn(*mut core::ffi::c_void, *const NOTIFICATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NotifyNewMail: usize,
}
windows_core::imp::define_interface!(IProfSect, IProfSect_Vtbl, 0);
impl core::ops::Deref for IProfSect {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProfSect, windows_core::IUnknown, IMAPIProp);
impl IProfSect {}
#[repr(C)]
pub struct IProfSect_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
windows_core::imp::define_interface!(IPropData, IPropData_Vtbl, 0);
impl core::ops::Deref for IPropData {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropData, windows_core::IUnknown, IMAPIProp);
impl IPropData {
    pub unsafe fn HrSetObjAccess(&self, ulaccess: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrSetObjAccess)(windows_core::Interface::as_raw(self), ulaccess).ok()
    }
    pub unsafe fn HrSetPropAccess(&self, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrSetPropAccess)(windows_core::Interface::as_raw(self), lpproptagarray, rgulaccess).ok()
    }
    pub unsafe fn HrGetPropAccess(&self, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrGetPropAccess)(windows_core::Interface::as_raw(self), lppproptagarray, lprgulaccess).ok()
    }
    pub unsafe fn HrAddObjProps(&self, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrAddObjProps)(windows_core::Interface::as_raw(self), lppproptagarray, lprgulaccess).ok()
    }
}
#[repr(C)]
pub struct IPropData_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub HrSetObjAccess: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HrSetPropAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropTagArray, *mut u32) -> windows_core::HRESULT,
    pub HrGetPropAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SPropTagArray, *mut *mut u32) -> windows_core::HRESULT,
    pub HrAddObjProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropTagArray, *mut *mut SPropProblemArray) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProviderAdmin, IProviderAdmin_Vtbl, 0);
impl core::ops::Deref for IProviderAdmin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProviderAdmin, windows_core::IUnknown);
impl IProviderAdmin {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32) -> windows_core::Result<*mut MAPIERROR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetProviderTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateProvider(&self, lpszprovider: *const i8, lpprops: &[SPropValue], uluiparam: usize, ulflags: u32) -> windows_core::Result<MAPIUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateProvider)(windows_core::Interface::as_raw(self), lpszprovider, lpprops.len().try_into().unwrap(), core::mem::transmute(lpprops.as_ptr()), uluiparam, ulflags, &mut result__).map(|| result__)
    }
    pub unsafe fn DeleteProvider(&self, lpuid: *const MAPIUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteProvider)(windows_core::Interface::as_raw(self), lpuid).ok()
    }
    pub unsafe fn OpenProfileSection(&self, lpuid: Option<*const MAPIUID>, lpinterface: Option<*const windows_core::GUID>, ulflags: u32) -> windows_core::Result<IProfSect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenProfileSection)(windows_core::Interface::as_raw(self), core::mem::transmute(lpuid.unwrap_or(std::ptr::null())), core::mem::transmute(lpinterface.unwrap_or(std::ptr::null())), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IProviderAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *mut *mut MAPIERROR) -> windows_core::HRESULT,
    pub GetProviderTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, u32, *const SPropValue, usize, u32, *mut MAPIUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateProvider: usize,
    pub DeleteProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const MAPIUID) -> windows_core::HRESULT,
    pub OpenProfileSection: unsafe extern "system" fn(*mut core::ffi::c_void, *const MAPIUID, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITableData, ITableData_Vtbl, 0);
impl core::ops::Deref for ITableData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableData, windows_core::IUnknown);
impl ITableData {
    pub unsafe fn HrGetView(&self, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut Option<IMAPITable>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrGetView)(windows_core::Interface::as_raw(self), lpssortorderset, lpfcallerrelease, ulcallerdata, core::mem::transmute(lppmapitable)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrModifyRow(&self, param0: *mut SRow) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrModifyRow)(windows_core::Interface::as_raw(self), param0).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrDeleteRow(&self, lpspropvalue: *mut SPropValue) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrDeleteRow)(windows_core::Interface::as_raw(self), lpspropvalue).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrQueryRow(&self, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrQueryRow)(windows_core::Interface::as_raw(self), lpspropvalue, lppsrow, lpulirow).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrEnumRow(&self, ulrownumber: u32, lppsrow: *mut *mut SRow) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrEnumRow)(windows_core::Interface::as_raw(self), ulrownumber, lppsrow).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrNotify(&self, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrNotify)(windows_core::Interface::as_raw(self), ulflags, cvalues, lpspropvalue).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrInsertRow(&self, ulirow: u32, lpsrow: *mut SRow) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrInsertRow)(windows_core::Interface::as_raw(self), ulirow, lpsrow).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrModifyRows(&self, ulflags: u32, lpsrowset: *mut SRowSet) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrModifyRows)(windows_core::Interface::as_raw(self), ulflags, lpsrowset).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrDeleteRows(&self, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HrDeleteRows)(windows_core::Interface::as_raw(self), ulflags, lprowsettodelete, crowsdeleted).ok()
    }
}
#[repr(C)]
pub struct ITableData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HrGetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SSortOrderSet, *mut CALLERRELEASE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub HrModifyRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SRow) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HrModifyRow: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HrDeleteRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropValue) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HrDeleteRow: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HrQueryRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropValue, *mut *mut SRow, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HrQueryRow: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HrEnumRow: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut SRow) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HrEnumRow: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HrNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut SPropValue) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HrNotify: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HrInsertRow: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SRow) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HrInsertRow: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HrModifyRows: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SRowSet) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HrModifyRows: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HrDeleteRows: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SRowSet, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HrDeleteRows: usize,
}
windows_core::imp::define_interface!(IWABExtInit, IWABExtInit_Vtbl, 0xea22ebf0_87a4_11d1_9acf_00a0c91f9c8b);
impl core::ops::Deref for IWABExtInit {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWABExtInit, windows_core::IUnknown);
impl IWABExtInit {
    pub unsafe fn Initialize(&self, lpwabextdisplay: *mut WABEXTDISPLAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), lpwabextdisplay).ok()
    }
}
#[repr(C)]
pub struct IWABExtInit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WABEXTDISPLAY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWABObject, IWABObject_Vtbl, 0);
impl core::ops::Deref for IWABObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWABObject, windows_core::IUnknown);
impl IWABObject {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn AllocateBuffer(&self, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateBuffer)(windows_core::Interface::as_raw(self), cbsize, lppbuffer).ok()
    }
    pub unsafe fn AllocateMore(&self, cbsize: u32, lpobject: *const core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateMore)(windows_core::Interface::as_raw(self), cbsize, lpobject, lppbuffer).ok()
    }
    pub unsafe fn FreeBuffer(&self, lpbuffer: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), lpbuffer).ok()
    }
    pub unsafe fn Backup<P0>(&self, lpfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).Backup)(windows_core::Interface::as_raw(self), lpfilename.param().abi()).ok()
    }
    pub unsafe fn Import<P0>(&self, lpwip: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), lpwip.param().abi()).ok()
    }
    pub unsafe fn Find<P0, P1>(&self, lpiab: P0, hwnd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Find)(windows_core::Interface::as_raw(self), lpiab.param().abi(), hwnd.param().abi()).ok()
    }
    pub unsafe fn VCardDisplay<P0, P1, P2>(&self, lpiab: P0, hwnd: P1, lpszfilename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).VCardDisplay)(windows_core::Interface::as_raw(self), lpiab.param().abi(), hwnd.param().abi(), lpszfilename.param().abi()).ok()
    }
    pub unsafe fn LDAPUrl<P0, P1, P2>(&self, lpiab: P0, hwnd: P1, ulflags: u32, lpszurl: P2) -> windows_core::Result<IMailUser>
    where
        P0: windows_core::Param<IAddrBook>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LDAPUrl)(windows_core::Interface::as_raw(self), lpiab.param().abi(), hwnd.param().abi(), ulflags, lpszurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VCardCreate<P0, P1, P2>(&self, lpiab: P0, ulflags: u32, lpszvcard: P1, lpmailuser: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<IMailUser>,
    {
        (windows_core::Interface::vtable(self).VCardCreate)(windows_core::Interface::as_raw(self), lpiab.param().abi(), ulflags, lpszvcard.param().abi(), lpmailuser.param().abi()).ok()
    }
    pub unsafe fn VCardRetrieve<P0, P1>(&self, lpiab: P0, ulflags: u32, lpszvcard: P1) -> windows_core::Result<IMailUser>
    where
        P0: windows_core::Param<IAddrBook>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VCardRetrieve)(windows_core::Interface::as_raw(self), lpiab.param().abi(), ulflags, lpszvcard.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMe<P0, P1>(&self, lpiab: P0, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).GetMe)(windows_core::Interface::as_raw(self), lpiab.param().abi(), ulflags, lpdwaction, lpsbeid, hwnd.param().abi()).ok()
    }
    pub unsafe fn SetMe<P0, P1>(&self, lpiab: P0, ulflags: u32, sbeid: SBinary, hwnd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SetMe)(windows_core::Interface::as_raw(self), lpiab.param().abi(), ulflags, core::mem::transmute(sbeid), hwnd.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWABObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *mut *mut MAPIERROR) -> windows_core::HRESULT,
    pub AllocateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateMore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Backup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> windows_core::HRESULT,
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> windows_core::HRESULT,
    pub Find: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub VCardDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCSTR) -> windows_core::HRESULT,
    pub LDAPUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, u32, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VCardCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VCardRetrieve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32, *mut SBinary, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub SetMe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, SBinary, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub const E_IMAPI_BURN_VERIFICATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0007_u32 as _);
pub const E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0408_u32 as _);
pub const E_IMAPI_DF2DATA_INVALID_MEDIA_STATE: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0402_u32 as _);
pub const E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0406_u32 as _);
pub const E_IMAPI_DF2DATA_MEDIA_NOT_BLANK: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0405_u32 as _);
pub const E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0407_u32 as _);
pub const E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0403_u32 as _);
pub const E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0404_u32 as _);
pub const E_IMAPI_DF2DATA_WRITE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0400_u32 as _);
pub const E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0401_u32 as _);
pub const E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0604_u32 as _);
pub const E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA060E_u32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0606_u32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0602_u32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0607_u32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_PREPARED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0603_u32 as _);
pub const E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0609_u32 as _);
pub const E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA060A_u32 as _);
pub const E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0610_u32 as _);
pub const E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT: windows_core::HRESULT = windows_core::HRESULT(0xC0AA060F_u32 as _);
pub const E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA060D_u32 as _);
pub const E_IMAPI_DF2RAW_WRITE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0600_u32 as _);
pub const E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0601_u32 as _);
pub const E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID: windows_core::HRESULT = windows_core::HRESULT(0xC0AA050F_u32 as _);
pub const E_IMAPI_DF2TAO_INVALID_ISRC: windows_core::HRESULT = windows_core::HRESULT(0xC0AA050B_u32 as _);
pub const E_IMAPI_DF2TAO_INVALID_MCN: windows_core::HRESULT = windows_core::HRESULT(0xC0AA050C_u32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0506_u32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0502_u32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0507_u32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_PREPARED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0503_u32 as _);
pub const E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0509_u32 as _);
pub const E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA050A_u32 as _);
pub const E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0504_u32 as _);
pub const E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA050E_u32 as _);
pub const E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA050D_u32 as _);
pub const E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0505_u32 as _);
pub const E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0508_u32 as _);
pub const E_IMAPI_DF2TAO_WRITE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0500_u32 as _);
pub const E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0501_u32 as _);
pub const E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID: windows_core::HRESULT = windows_core::HRESULT(0xC0AA090B_u32 as _);
pub const E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80AA0902_u32 as _);
pub const E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND: windows_core::HRESULT = windows_core::HRESULT(0x80AA0905_u32 as _);
pub const E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND: windows_core::HRESULT = windows_core::HRESULT(0x80AA0908_u32 as _);
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE: windows_core::HRESULT = windows_core::HRESULT(0x80AA0904_u32 as _);
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0909_u32 as _);
pub const E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80AA0903_u32 as _);
pub const E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80AA0901_u32 as _);
pub const E_IMAPI_ERASE_RECORDER_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80AA0900_u32 as _);
pub const E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA090A_u32 as _);
pub const E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR: windows_core::HRESULT = windows_core::HRESULT(0x80AA0906_u32 as _);
pub const E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE: windows_core::HRESULT = windows_core::HRESULT(0x80AA0907_u32 as _);
pub const E_IMAPI_LOSS_OF_STREAMING: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0300_u32 as _);
pub const E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A05_u32 as _);
pub const E_IMAPI_RAW_IMAGE_IS_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A00_u32 as _);
pub const E_IMAPI_RAW_IMAGE_NO_TRACKS: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A03_u32 as _);
pub const E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A02_u32 as _);
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A01_u32 as _);
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A06_u32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A04_u32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A07_u32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A09_u32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX: windows_core::HRESULT = windows_core::HRESULT(0x80AA0A0A_u32 as _);
pub const E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0211_u32 as _);
pub const E_IMAPI_RECORDER_COMMAND_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0xC0AA020D_u32 as _);
pub const E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0xC0AA020E_u32 as _);
pub const E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT: windows_core::HRESULT = windows_core::HRESULT(0xC0AA020B_u32 as _);
pub const E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA020C_u32 as _);
pub const E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0208_u32 as _);
pub const E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0xC0AA02FF_u32 as _);
pub const E_IMAPI_RECORDER_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0210_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_BECOMING_READY: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0205_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_BUSY: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0207_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0206_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0203_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0212_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_NO_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0202_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC0AA020F_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0204_u32 as _);
pub const E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0209_u32 as _);
pub const E_IMAPI_RECORDER_NO_SUCH_FEATURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AA020A_u32 as _);
pub const E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0201_u32 as _);
pub const E_IMAPI_RECORDER_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0003_u32 as _);
pub const E_IMAPI_REQUEST_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0002_u32 as _);
pub const E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0xC0AA0301_u32 as _);
pub const FACILITY_IMAPI2: u32 = 170u32;
pub const IMAPI_E_BAD_MULTISESSION_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB162_u32 as _);
pub const IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB14A_u32 as _);
pub const IMAPI_E_BOOT_IMAGE_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB148_u32 as _);
pub const IMAPI_E_BOOT_OBJECT_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB149_u32 as _);
pub const IMAPI_E_DATA_STREAM_CREATE_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB12A_u32 as _);
pub const IMAPI_E_DATA_STREAM_INCONSISTENCY: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB128_u32 as _);
pub const IMAPI_E_DATA_STREAM_READ_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB129_u32 as _);
pub const IMAPI_E_DATA_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB132_u32 as _);
pub const IMAPI_E_DIRECTORY_READ_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB12B_u32 as _);
pub const IMAPI_E_DIR_NOT_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB10A_u32 as _);
pub const IMAPI_E_DIR_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB11A_u32 as _);
pub const IMAPI_E_DISC_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB158_u32 as _);
pub const IMAPI_E_DUP_NAME: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB112_u32 as _);
pub const IMAPI_E_EMPTY_DISC: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB150_u32 as _);
pub const IMAPI_E_FILE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB119_u32 as _);
pub const IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB163_u32 as _);
pub const IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB154_u32 as _);
pub const IMAPI_E_FILE_SYSTEM_NOT_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB106_u32 as _);
pub const IMAPI_E_FILE_SYSTEM_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB152_u32 as _);
pub const IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB153_u32 as _);
pub const IMAPI_E_FSI_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB100_u32 as _);
pub const IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB200_u32 as _);
pub const IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB203_u32 as _);
pub const IMAPI_E_IMAGEMANAGER_NO_IMAGE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB202_u32 as _);
pub const IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB201_u32 as _);
pub const IMAPI_E_IMAGE_SIZE_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB120_u32 as _);
pub const IMAPI_E_IMAGE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB121_u32 as _);
pub const IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB159_u32 as _);
pub const IMAPI_E_IMPORT_READ_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB157_u32 as _);
pub const IMAPI_E_IMPORT_SEEK_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB156_u32 as _);
pub const IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB15E_u32 as _);
pub const IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB155_u32 as _);
pub const IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB15B_u32 as _);
pub const IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB133_u32 as _);
pub const IMAPI_E_INVALID_DATE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB105_u32 as _);
pub const IMAPI_E_INVALID_PARAM: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB101_u32 as _);
pub const IMAPI_E_INVALID_PATH: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB110_u32 as _);
pub const IMAPI_E_INVALID_VOLUME_NAME: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB104_u32 as _);
pub const IMAPI_E_INVALID_WORKING_DIRECTORY: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB140_u32 as _);
pub const IMAPI_E_ISO9660_LEVELS: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB131_u32 as _);
pub const IMAPI_E_ITEM_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB118_u32 as _);
pub const IMAPI_E_MULTISESSION_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB15D_u32 as _);
pub const IMAPI_E_NOT_DIR: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB109_u32 as _);
pub const IMAPI_E_NOT_FILE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB108_u32 as _);
pub const IMAPI_E_NOT_IN_FILE_SYSTEM: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB10B_u32 as _);
pub const IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB15C_u32 as _);
pub const IMAPI_E_NO_OUTPUT: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB103_u32 as _);
pub const IMAPI_E_NO_SUPPORTED_FILE_SYSTEM: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB151_u32 as _);
pub const IMAPI_E_NO_UNIQUE_NAME: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB113_u32 as _);
pub const IMAPI_E_PROPERTY_NOT_ACCESSIBLE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB160_u32 as _);
pub const IMAPI_E_READONLY: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB102_u32 as _);
pub const IMAPI_E_RESTRICTED_NAME_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB111_u32 as _);
pub const IMAPI_E_STASHFILE_MOVE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB142_u32 as _);
pub const IMAPI_E_STASHFILE_OPEN_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB138_u32 as _);
pub const IMAPI_E_STASHFILE_READ_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB13B_u32 as _);
pub const IMAPI_E_STASHFILE_SEEK_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB139_u32 as _);
pub const IMAPI_E_STASHFILE_WRITE_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB13A_u32 as _);
pub const IMAPI_E_TOO_MANY_DIRS: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB130_u32 as _);
pub const IMAPI_E_UDF_NOT_WRITE_COMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB15A_u32 as _);
pub const IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB161_u32 as _);
pub const IMAPI_E_WORKING_DIRECTORY_SPACE: windows_core::HRESULT = windows_core::HRESULT(0xC0AAB141_u32 as _);
pub const IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xAAB15F_u32 as _);
pub const MAPI_COMPOUND: u32 = 128u32;
pub const MAPI_DIM: u32 = 1u32;
pub const MAPI_ERROR_VERSION: i32 = 0i32;
pub const MAPI_E_CALL_FAILED: i32 = -2147467259i32;
pub const MAPI_E_INTERFACE_NOT_SUPPORTED: i32 = -2147467262i32;
pub const MAPI_E_INVALID_PARAMETER: i32 = -2147024809i32;
pub const MAPI_E_NOT_ENOUGH_MEMORY: i32 = -2147024882i32;
pub const MAPI_E_NO_ACCESS: i32 = -2147024891i32;
pub const MAPI_NOTRECIP: u32 = 64u32;
pub const MAPI_NOTRESERVED: u32 = 8u32;
pub const MAPI_NOW: u32 = 16u32;
pub const MAPI_ONE_OFF_NO_RICH_INFO: u32 = 1u32;
pub const MAPI_P1: u32 = 268435456u32;
pub const MAPI_SHORTTERM: u32 = 128u32;
pub const MAPI_SUBMITTED: u32 = 2147483648u32;
pub const MAPI_THISSESSION: u32 = 32u32;
pub const MAPI_USE_DEFAULT: u32 = 64u32;
pub const MNID_ID: u32 = 0u32;
pub const MNID_STRING: u32 = 1u32;
pub const MV_FLAG: u32 = 4096u32;
pub const MV_INSTANCE: u32 = 8192u32;
pub const OPENSTREAMONFILE: windows_core::PCSTR = windows_core::s!("OpenStreamOnFile");
pub const PRIHIGHEST: u32 = 32767u32;
pub const PRILOWEST: i32 = -32768i32;
pub const PRIUSER: u32 = 0u32;
pub const PROP_ID_INVALID: u32 = 65535u32;
pub const PROP_ID_NULL: u32 = 0u32;
pub const PROP_ID_SECURE_MAX: u32 = 26623u32;
pub const PROP_ID_SECURE_MIN: u32 = 26608u32;
pub const SERVICE_UI_ALLOWED: u32 = 16u32;
pub const SERVICE_UI_ALWAYS: u32 = 2u32;
pub const S_IMAPI_BOTHADJUSTED: windows_core::HRESULT = windows_core::HRESULT(0xAA0006_u32 as _);
pub const S_IMAPI_COMMAND_HAS_SENSE_DATA: windows_core::HRESULT = windows_core::HRESULT(0xAA0200_u32 as _);
pub const S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xAA0A08_u32 as _);
pub const S_IMAPI_ROTATIONADJUSTED: windows_core::HRESULT = windows_core::HRESULT(0xAA0005_u32 as _);
pub const S_IMAPI_SPEEDADJUSTED: windows_core::HRESULT = windows_core::HRESULT(0xAA0004_u32 as _);
pub const S_IMAPI_WRITE_NOT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xAA0302_u32 as _);
pub const TABLE_CHANGED: u32 = 1u32;
pub const TABLE_ERROR: u32 = 2u32;
pub const TABLE_RELOAD: u32 = 9u32;
pub const TABLE_RESTRICT_DONE: u32 = 7u32;
pub const TABLE_ROW_ADDED: u32 = 3u32;
pub const TABLE_ROW_DELETED: u32 = 4u32;
pub const TABLE_ROW_MODIFIED: u32 = 5u32;
pub const TABLE_SETCOL_DONE: u32 = 8u32;
pub const TABLE_SORT_DONE: u32 = 6u32;
pub const TAD_ALL_ROWS: u32 = 1u32;
pub const UI_CURRENT_PROVIDER_FIRST: u32 = 4u32;
pub const UI_SERVICE: u32 = 2u32;
pub const WABOBJECT_LDAPURL_RETURN_MAILUSER: u32 = 1u32;
pub const WABOBJECT_ME_NEW: u32 = 1u32;
pub const WABOBJECT_ME_NOCREATE: u32 = 2u32;
pub const WAB_CONTEXT_ADRLIST: u32 = 2u32;
pub const WAB_DISPLAY_ISNTDS: u32 = 4u32;
pub const WAB_DISPLAY_LDAPURL: u32 = 1u32;
pub const WAB_DLL_NAME: windows_core::PCWSTR = windows_core::w!("WAB32.DLL");
pub const WAB_DLL_PATH_KEY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\WAB\\DLLPath");
pub const WAB_ENABLE_PROFILES: u32 = 4194304u32;
pub const WAB_IGNORE_PROFILES: u32 = 8388608u32;
pub const WAB_LOCAL_CONTAINERS: u32 = 1048576u32;
pub const WAB_PROFILE_CONTENTS: u32 = 2097152u32;
pub const WAB_USE_OE_SENDMAIL: u32 = 1u32;
pub const WAB_VCARD_FILE: u32 = 0u32;
pub const WAB_VCARD_STREAM: u32 = 1u32;
pub const cchProfileNameMax: u32 = 64u32;
pub const cchProfilePassMax: u32 = 64u32;
pub const fMapiUnicode: u32 = 0u32;
pub const genderFemale: Gender = Gender(1i32);
pub const genderMale: Gender = Gender(2i32);
pub const genderUnspecified: Gender = Gender(0i32);
pub const hrSuccess: u32 = 0u32;
pub const szHrDispatchNotifications: windows_core::PCSTR = windows_core::s!("HrDispatchNotifications");
pub const szMAPINotificationMsg: windows_core::PCSTR = windows_core::s!("MAPI Notify window message");
pub const szScCreateConversationIndex: windows_core::PCSTR = windows_core::s!("ScCreateConversationIndex");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Gender(pub i32);
impl windows_core::TypeKind for Gender {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Gender {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Gender").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADRENTRY {
    pub ulReserved1: u32,
    pub cValues: u32,
    pub rgPropVals: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for ADRENTRY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for ADRENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADRLIST {
    pub cEntries: u32,
    pub aEntries: [ADRENTRY; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for ADRLIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for ADRLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug)]
pub struct ADRPARM {
    pub cbABContEntryID: u32,
    pub lpABContEntryID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpReserved: *mut core::ffi::c_void,
    pub ulHelpContext: u32,
    pub lpszHelpFileName: *mut i8,
    pub lpfnABSDI: LPFNABSDI,
    pub lpfnDismiss: LPFNDISMISS,
    pub lpvDismissContext: *mut core::ffi::c_void,
    pub lpszCaption: *mut i8,
    pub lpszNewEntryTitle: *mut i8,
    pub lpszDestWellsTitle: *mut i8,
    pub cDestFields: u32,
    pub nDestFieldFocus: u32,
    pub lppszDestTitles: *mut *mut i8,
    pub lpulDestComps: *mut u32,
    pub lpContRestriction: *mut SRestriction,
    pub lpHierRestriction: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for ADRPARM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for ADRPARM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRControl: u32,
}
impl windows_core::TypeKind for DTBLBUTTON {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLCHECKBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRPropertyName: u32,
}
impl windows_core::TypeKind for DTBLCHECKBOX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLCHECKBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLCOMBOBOX {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPRPropertyName: u32,
    pub ulPRTableName: u32,
}
impl windows_core::TypeKind for DTBLCOMBOBOX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLCOMBOBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLDDLBX {
    pub ulFlags: u32,
    pub ulPRDisplayProperty: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
impl windows_core::TypeKind for DTBLDDLBX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLDDLBX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLEDIT {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPropTag: u32,
}
impl windows_core::TypeKind for DTBLEDIT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLEDIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLGROUPBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
}
impl windows_core::TypeKind for DTBLGROUPBOX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLGROUPBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLLABEL {
    pub ulbLpszLabelName: u32,
    pub ulFlags: u32,
}
impl windows_core::TypeKind for DTBLLABEL {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLLABEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLLBX {
    pub ulFlags: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
impl windows_core::TypeKind for DTBLLBX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLLBX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLMVDDLBX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
impl windows_core::TypeKind for DTBLMVDDLBX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLMVDDLBX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLMVLISTBOX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
impl windows_core::TypeKind for DTBLMVLISTBOX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLMVLISTBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLPAGE {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulbLpszComponent: u32,
    pub ulContext: u32,
}
impl windows_core::TypeKind for DTBLPAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLPAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DTBLRADIOBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulcButtons: u32,
    pub ulPropTag: u32,
    pub lReturnValue: i32,
}
impl windows_core::TypeKind for DTBLRADIOBUTTON {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTBLRADIOBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTCTL {
    pub ulCtlType: u32,
    pub ulCtlFlags: u32,
    pub lpbNotif: *mut u8,
    pub cbNotif: u32,
    pub lpszFilter: *mut i8,
    pub ulItemID: u32,
    pub ctl: DTCTL_0,
}
impl windows_core::TypeKind for DTCTL {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTCTL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DTCTL_0 {
    pub lpv: *mut core::ffi::c_void,
    pub lplabel: *mut DTBLLABEL,
    pub lpedit: *mut DTBLEDIT,
    pub lplbx: *mut DTBLLBX,
    pub lpcombobox: *mut DTBLCOMBOBOX,
    pub lpddlbx: *mut DTBLDDLBX,
    pub lpcheckbox: *mut DTBLCHECKBOX,
    pub lpgroupbox: *mut DTBLGROUPBOX,
    pub lpbutton: *mut DTBLBUTTON,
    pub lpradiobutton: *mut DTBLRADIOBUTTON,
    pub lpmvlbx: *mut DTBLMVLISTBOX,
    pub lpmvddlbx: *mut DTBLMVDDLBX,
    pub lppage: *mut DTBLPAGE,
}
impl windows_core::TypeKind for DTCTL_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTCTL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTPAGE {
    pub cctl: u32,
    pub lpszResourceName: *mut i8,
    pub Anonymous: DTPAGE_0,
    pub lpctl: *mut DTCTL,
}
impl windows_core::TypeKind for DTPAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTPAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DTPAGE_0 {
    pub lpszComponent: *mut i8,
    pub ulItemID: u32,
}
impl windows_core::TypeKind for DTPAGE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTPAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENTRYID {
    pub abFlags: [u8; 4],
    pub ab: [u8; 1],
}
impl windows_core::TypeKind for ENTRYID {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENTRYID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ERROR_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub scode: i32,
    pub ulFlags: u32,
    pub lpMAPIError: *mut MAPIERROR,
}
impl windows_core::TypeKind for ERROR_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for ERROR_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXTENDED_NOTIFICATION {
    pub ulEvent: u32,
    pub cb: u32,
    pub pbEventParameters: *mut u8,
}
impl windows_core::TypeKind for EXTENDED_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXTENDED_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLATENTRY {
    pub cb: u32,
    pub abEntry: [u8; 1],
}
impl windows_core::TypeKind for FLATENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLATENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLATENTRYLIST {
    pub cEntries: u32,
    pub cbEntries: u32,
    pub abEntries: [u8; 1],
}
impl windows_core::TypeKind for FLATENTRYLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLATENTRYLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLATMTSIDLIST {
    pub cMTSIDs: u32,
    pub cbMTSIDs: u32,
    pub abMTSIDs: [u8; 1],
}
impl windows_core::TypeKind for FLATMTSIDLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLATMTSIDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FlagList {
    pub cFlags: u32,
    pub ulFlag: [u32; 1],
}
impl windows_core::TypeKind for FlagList {
    type TypeKind = windows_core::CopyType;
}
impl Default for FlagList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWABACTIONITEM(pub isize);
impl Default for LPWABACTIONITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LPWABACTIONITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MAPIERROR {
    pub ulVersion: u32,
    pub lpszError: *mut i8,
    pub lpszComponent: *mut i8,
    pub ulLowLevelError: u32,
    pub ulContext: u32,
}
impl windows_core::TypeKind for MAPIERROR {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPIERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAPINAMEID {
    pub lpguid: *mut windows_core::GUID,
    pub ulKind: u32,
    pub Kind: MAPINAMEID_0,
}
impl windows_core::TypeKind for MAPINAMEID {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPINAMEID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MAPINAMEID_0 {
    pub lID: i32,
    pub lpwstrName: windows_core::PWSTR,
}
impl windows_core::TypeKind for MAPINAMEID_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPINAMEID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MAPIUID {
    pub ab: [u8; 16],
}
impl windows_core::TypeKind for MAPIUID {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPIUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MTSID {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl windows_core::TypeKind for MTSID {
    type TypeKind = windows_core::CopyType;
}
impl Default for MTSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NEWMAIL_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpszMessageClass: *mut i8,
    pub ulMessageFlags: u32,
}
impl windows_core::TypeKind for NEWMAIL_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for NEWMAIL_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct NOTIFICATION {
    pub ulEventType: u32,
    pub ulAlignPad: u32,
    pub info: NOTIFICATION_0,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union NOTIFICATION_0 {
    pub err: ERROR_NOTIFICATION,
    pub newmail: NEWMAIL_NOTIFICATION,
    pub obj: OBJECT_NOTIFICATION,
    pub tab: TABLE_NOTIFICATION,
    pub ext: EXTENDED_NOTIFICATION,
    pub statobj: STATUS_OBJECT_NOTIFICATION,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for NOTIFICATION_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NOTIFKEY {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl windows_core::TypeKind for NOTIFKEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for NOTIFKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub ulObjType: u32,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub cbOldID: u32,
    pub lpOldID: *mut ENTRYID,
    pub cbOldParentID: u32,
    pub lpOldParentID: *mut ENTRYID,
    pub lpPropTagArray: *mut SPropTagArray,
}
impl windows_core::TypeKind for OBJECT_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAndRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SAndRestriction {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAndRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAppTimeArray {
    pub cValues: u32,
    pub lpat: *mut f64,
}
impl windows_core::TypeKind for SAppTimeArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAppTimeArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SBinary {
    pub cb: u32,
    pub lpb: *mut u8,
}
impl windows_core::TypeKind for SBinary {
    type TypeKind = windows_core::CopyType;
}
impl Default for SBinary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SBinaryArray {
    pub cValues: u32,
    pub lpbin: *mut SBinary,
}
impl windows_core::TypeKind for SBinaryArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SBinaryArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SBitMaskRestriction {
    pub relBMR: u32,
    pub ulPropTag: u32,
    pub ulMask: u32,
}
impl windows_core::TypeKind for SBitMaskRestriction {
    type TypeKind = windows_core::CopyType;
}
impl Default for SBitMaskRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCommentRestriction {
    pub cValues: u32,
    pub lpRes: *mut SRestriction,
    pub lpProp: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SCommentRestriction {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SCommentRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SComparePropsRestriction {
    pub relop: u32,
    pub ulPropTag1: u32,
    pub ulPropTag2: u32,
}
impl windows_core::TypeKind for SComparePropsRestriction {
    type TypeKind = windows_core::CopyType;
}
impl Default for SComparePropsRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SContentRestriction {
    pub ulFuzzyLevel: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SContentRestriction {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SContentRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCurrencyArray {
    pub cValues: u32,
    pub lpcur: *mut super::Com::CY,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SCurrencyArray {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SCurrencyArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SDateTimeArray {
    pub cValues: u32,
    pub lpft: *mut super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for SDateTimeArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SDateTimeArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SDoubleArray {
    pub cValues: u32,
    pub lpdbl: *mut f64,
}
impl windows_core::TypeKind for SDoubleArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SDoubleArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SExistRestriction {
    pub ulReserved1: u32,
    pub ulPropTag: u32,
    pub ulReserved2: u32,
}
impl windows_core::TypeKind for SExistRestriction {
    type TypeKind = windows_core::CopyType;
}
impl Default for SExistRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SGuidArray {
    pub cValues: u32,
    pub lpguid: *mut windows_core::GUID,
}
impl windows_core::TypeKind for SGuidArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SGuidArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLPSTRArray {
    pub cValues: u32,
    pub lppszA: *mut windows_core::PSTR,
}
impl windows_core::TypeKind for SLPSTRArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SLPSTRArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLargeIntegerArray {
    pub cValues: u32,
    pub lpli: *mut i64,
}
impl windows_core::TypeKind for SLargeIntegerArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SLargeIntegerArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLongArray {
    pub cValues: u32,
    pub lpl: *mut i32,
}
impl windows_core::TypeKind for SLongArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SLongArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SNotRestriction {
    pub ulReserved: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SNotRestriction {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SNotRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOrRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SOrRestriction {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SOrRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPropProblem {
    pub ulIndex: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
impl windows_core::TypeKind for SPropProblem {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPropProblem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPropProblemArray {
    pub cProblem: u32,
    pub aProblem: [SPropProblem; 1],
}
impl windows_core::TypeKind for SPropProblemArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPropProblemArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPropTagArray {
    pub cValues: u32,
    pub aulPropTag: [u32; 1],
}
impl windows_core::TypeKind for SPropTagArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPropTagArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SPropValue {
    pub ulPropTag: u32,
    pub dwAlignPad: u32,
    pub Value: __UPV,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SPropValue {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SPropValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPropertyRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SPropertyRestriction {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SPropertyRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SRealArray {
    pub cValues: u32,
    pub lpflt: *mut f32,
}
impl windows_core::TypeKind for SRealArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SRealArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SRestriction {
    pub rt: u32,
    pub res: SRestriction_0,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SRestriction {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union SRestriction_0 {
    pub resCompareProps: SComparePropsRestriction,
    pub resAnd: SAndRestriction,
    pub resOr: SOrRestriction,
    pub resNot: SNotRestriction,
    pub resContent: SContentRestriction,
    pub resProperty: SPropertyRestriction,
    pub resBitMask: SBitMaskRestriction,
    pub resSize: SSizeRestriction,
    pub resExist: SExistRestriction,
    pub resSub: SSubRestriction,
    pub resComment: SCommentRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SRestriction_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SRestriction_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SRow {
    pub ulAdrEntryPad: u32,
    pub cValues: u32,
    pub lpProps: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SRow {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SRow {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SRowSet {
    pub cRows: u32,
    pub aRow: [SRow; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SRowSet {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SRowSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SShortArray {
    pub cValues: u32,
    pub lpi: *mut i16,
}
impl windows_core::TypeKind for SShortArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SShortArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSizeRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub cb: u32,
}
impl windows_core::TypeKind for SSizeRestriction {
    type TypeKind = windows_core::CopyType;
}
impl Default for SSizeRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSortOrder {
    pub ulPropTag: u32,
    pub ulOrder: u32,
}
impl windows_core::TypeKind for SSortOrder {
    type TypeKind = windows_core::CopyType;
}
impl Default for SSortOrder {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSortOrderSet {
    pub cSorts: u32,
    pub cCategories: u32,
    pub cExpanded: u32,
    pub aSort: [SSortOrder; 1],
}
impl windows_core::TypeKind for SSortOrderSet {
    type TypeKind = windows_core::CopyType;
}
impl Default for SSortOrderSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSubRestriction {
    pub ulSubObject: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SSubRestriction {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSubRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STATUS_OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cValues: u32,
    pub lpPropVals: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for STATUS_OBJECT_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for STATUS_OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SWStringArray {
    pub cValues: u32,
    pub lppszW: *mut windows_core::PWSTR,
}
impl windows_core::TypeKind for SWStringArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWStringArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct TABLE_NOTIFICATION {
    pub ulTableEvent: u32,
    pub hResult: windows_core::HRESULT,
    pub propIndex: SPropValue,
    pub propPrior: SPropValue,
    pub row: SRow,
    pub ulPad: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for TABLE_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for TABLE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct WABEXTDISPLAY {
    pub cbSize: u32,
    pub lpWABObject: core::mem::ManuallyDrop<Option<IWABObject>>,
    pub lpAdrBook: core::mem::ManuallyDrop<Option<IAddrBook>>,
    pub lpPropObj: core::mem::ManuallyDrop<Option<IMAPIProp>>,
    pub fReadOnly: super::super::Foundation::BOOL,
    pub fDataChanged: super::super::Foundation::BOOL,
    pub ulFlags: u32,
    pub lpv: *mut core::ffi::c_void,
    pub lpsz: *mut i8,
}
impl Clone for WABEXTDISPLAY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WABEXTDISPLAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for WABEXTDISPLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct WABIMPORTPARAM {
    pub cbSize: u32,
    pub lpAdrBook: core::mem::ManuallyDrop<Option<IAddrBook>>,
    pub hWnd: super::super::Foundation::HWND,
    pub ulFlags: u32,
    pub lpszFileName: windows_core::PSTR,
}
impl Clone for WABIMPORTPARAM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WABIMPORTPARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for WABIMPORTPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WAB_PARAM {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub szFileName: windows_core::PSTR,
    pub ulFlags: u32,
    pub guidPSExt: windows_core::GUID,
}
impl windows_core::TypeKind for WAB_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for WAB_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union __UPV {
    pub i: i16,
    pub l: i32,
    pub ul: u32,
    pub flt: f32,
    pub dbl: f64,
    pub b: u16,
    pub cur: super::Com::CY,
    pub at: f64,
    pub ft: super::super::Foundation::FILETIME,
    pub lpszA: windows_core::PSTR,
    pub bin: SBinary,
    pub lpszW: windows_core::PWSTR,
    pub lpguid: *mut windows_core::GUID,
    pub li: i64,
    pub MVi: SShortArray,
    pub MVl: SLongArray,
    pub MVflt: SRealArray,
    pub MVdbl: SDoubleArray,
    pub MVcur: SCurrencyArray,
    pub MVat: SAppTimeArray,
    pub MVft: SDateTimeArray,
    pub MVbin: SBinaryArray,
    pub MVszA: SLPSTRArray,
    pub MVszW: SWStringArray,
    pub MVguid: SGuidArray,
    pub MVli: SLargeIntegerArray,
    pub err: i32,
    pub x: i32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for __UPV {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for __UPV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CALLERRELEASE = Option<unsafe extern "system" fn(ulcallerdata: u32, lptbldata: Option<ITableData>, lpvue: Option<IMAPITable>)>;
pub type LPALLOCATEBUFFER = Option<unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPALLOCATEMORE = Option<unsafe extern "system" fn(cbsize: u32, lpobject: *mut core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPCREATECONVERSATIONINDEX = Option<unsafe extern "system" fn(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32>;
pub type LPDISPATCHNOTIFICATIONS = Option<unsafe extern "system" fn(ulflags: u32) -> windows_core::HRESULT>;
pub type LPFNABSDI = Option<unsafe extern "system" fn(uluiparam: usize, lpvmsg: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type LPFNBUTTON = Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut core::ffi::c_void, cbentryid: u32, lpselection: *mut ENTRYID, ulflags: u32) -> i32>;
pub type LPFNDISMISS = Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut core::ffi::c_void)>;
pub type LPFREEBUFFER = Option<unsafe extern "system" fn(lpbuffer: *mut core::ffi::c_void) -> u32>;
#[cfg(feature = "Win32_System_Com")]
pub type LPNOTIFCALLBACK = Option<unsafe extern "system" fn(lpvcontext: *mut core::ffi::c_void, cnotification: u32, lpnotifications: *mut NOTIFICATION) -> i32>;
#[cfg(feature = "Win32_System_Com")]
pub type LPOPENSTREAMONFILE = Option<unsafe extern "system" fn(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut Option<super::Com::IStream>) -> windows_core::HRESULT>;
pub type LPWABALLOCATEBUFFER = Option<unsafe extern "system" fn(lpwabobject: Option<IWABObject>, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPWABALLOCATEMORE = Option<unsafe extern "system" fn(lpwabobject: Option<IWABObject>, cbsize: u32, lpobject: *mut core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPWABFREEBUFFER = Option<unsafe extern "system" fn(lpwabobject: Option<IWABObject>, lpbuffer: *mut core::ffi::c_void) -> u32>;
pub type LPWABOPEN = Option<unsafe extern "system" fn(lppadrbook: *mut Option<IAddrBook>, lppwabobject: *mut Option<IWABObject>, lpwp: *mut WAB_PARAM, reserved2: u32) -> windows_core::HRESULT>;
pub type LPWABOPENEX = Option<unsafe extern "system" fn(lppadrbook: *mut Option<IAddrBook>, lppwabobject: *mut Option<IWABObject>, lpwp: *mut WAB_PARAM, reserved: u32, fnallocatebuffer: LPALLOCATEBUFFER, fnallocatemore: LPALLOCATEMORE, fnfreebuffer: LPFREEBUFFER) -> windows_core::HRESULT>;
pub type PFNIDLE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
core::include!("impl.rs");

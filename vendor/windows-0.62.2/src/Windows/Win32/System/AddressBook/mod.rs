#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn BuildDisplayTable<P3>(lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpmalloc: P3, hinstance: super::super::Foundation::HINSTANCE, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut Option<IMAPITable>, lpptbldata: *mut Option<ITableData>) -> windows_core::Result<()>
where
    P3: windows_core::Param<super::Com::IMalloc>,
{
    windows_core::link!("mapi32.dll" "system" fn BuildDisplayTable(lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpmalloc : * mut core::ffi::c_void, hinstance : super::super::Foundation:: HINSTANCE, cpages : u32, lppage : *mut DTPAGE, ulflags : u32, lpptable : *mut * mut core::ffi::c_void, lpptbldata : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { BuildDisplayTable(lpallocatebuffer, lpallocatemore, lpfreebuffer, lpmalloc.param().abi(), hinstance, cpages, lppage as _, ulflags, core::mem::transmute(lpptable), core::mem::transmute(lpptbldata)).ok() }
}
#[inline]
pub unsafe fn ChangeIdleRoutine(ftg: *mut core::ffi::c_void, lpfnidle: PFNIDLE, lpvidleparam: *mut core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16) {
    windows_core::link!("mapi32.dll" "system" fn ChangeIdleRoutine(ftg : *mut core::ffi::c_void, lpfnidle : PFNIDLE, lpvidleparam : *mut core::ffi::c_void, priidle : i16, csecidle : u32, iroidle : u16, ircidle : u16));
    unsafe { ChangeIdleRoutine(ftg as _, lpfnidle, lpvidleparam as _, priidle, csecidle, iroidle, ircidle) }
}
#[inline]
pub unsafe fn CreateIProp(lpinterface: *mut windows_core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut core::ffi::c_void, lpppropdata: *mut Option<IPropData>) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn CreateIProp(lpinterface : *mut windows_core::GUID, lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpvreserved : *mut core::ffi::c_void, lpppropdata : *mut * mut core::ffi::c_void) -> i32);
    unsafe { CreateIProp(lpinterface as _, lpallocatebuffer, lpallocatemore, lpfreebuffer, lpvreserved as _, core::mem::transmute(lpppropdata)) }
}
#[inline]
pub unsafe fn CreateTable(lpinterface: *mut windows_core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut Option<ITableData>) -> i32 {
    windows_core::link!("rtm.dll" "system" fn CreateTable(lpinterface : *mut windows_core::GUID, lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpvreserved : *mut core::ffi::c_void, ultabletype : u32, ulproptagindexcolumn : u32, lpsproptagarraycolumns : *mut SPropTagArray, lpptabledata : *mut * mut core::ffi::c_void) -> i32);
    unsafe { CreateTable(lpinterface as _, lpallocatebuffer, lpallocatemore, lpfreebuffer, lpvreserved as _, ultabletype, ulproptagindexcolumn, lpsproptagarraycolumns as _, core::mem::transmute(lpptabledata)) }
}
#[inline]
pub unsafe fn DeinitMapiUtil() {
    windows_core::link!("mapi32.dll" "system" fn DeinitMapiUtil());
    unsafe { DeinitMapiUtil() }
}
#[inline]
pub unsafe fn DeregisterIdleRoutine(ftg: *mut core::ffi::c_void) {
    windows_core::link!("mapi32.dll" "system" fn DeregisterIdleRoutine(ftg : *mut core::ffi::c_void));
    unsafe { DeregisterIdleRoutine(ftg as _) }
}
#[inline]
pub unsafe fn EnableIdleRoutine(ftg: *mut core::ffi::c_void, fenable: bool) {
    windows_core::link!("mapi32.dll" "system" fn EnableIdleRoutine(ftg : *mut core::ffi::c_void, fenable : windows_core::BOOL));
    unsafe { EnableIdleRoutine(ftg as _, fenable.into()) }
}
#[inline]
pub unsafe fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> windows_core::BOOL {
    windows_core::link!("mapi32.dll" "system" fn FEqualNames(lpname1 : *mut MAPINAMEID, lpname2 : *mut MAPINAMEID) -> windows_core::BOOL);
    unsafe { FEqualNames(lpname1 as _, lpname2 as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> windows_core::BOOL {
    windows_core::link!("mapi32.dll" "system" fn FPropCompareProp(lpspropvalue1 : *mut SPropValue, ulrelop : u32, lpspropvalue2 : *mut SPropValue) -> windows_core::BOOL);
    unsafe { FPropCompareProp(lpspropvalue1 as _, ulrelop, lpspropvalue2 as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> windows_core::BOOL {
    windows_core::link!("mapi32.dll" "system" fn FPropContainsProp(lpspropvaluedst : *mut SPropValue, lpspropvaluesrc : *mut SPropValue, ulfuzzylevel : u32) -> windows_core::BOOL);
    unsafe { FPropContainsProp(lpspropvaluedst as _, lpspropvaluesrc as _, ulfuzzylevel) }
}
#[inline]
pub unsafe fn FPropExists<P0>(lpmapiprop: P0, ulproptag: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<IMAPIProp>,
{
    windows_core::link!("mapi32.dll" "system" fn FPropExists(lpmapiprop : * mut core::ffi::c_void, ulproptag : u32) -> windows_core::BOOL);
    unsafe { FPropExists(lpmapiprop.param().abi(), ulproptag) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn FreePadrlist(lpadrlist: *mut ADRLIST) {
    windows_core::link!("mapi32.dll" "system" fn FreePadrlist(lpadrlist : *mut ADRLIST));
    unsafe { FreePadrlist(lpadrlist as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn FreeProws(lprows: *mut SRowSet) {
    windows_core::link!("mapi32.dll" "system" fn FreeProws(lprows : *mut SRowSet));
    unsafe { FreeProws(lprows as _) }
}
#[inline]
pub unsafe fn FtAddFt(ftaddend1: super::super::Foundation::FILETIME, ftaddend2: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    windows_core::link!("mapi32.dll" "system" fn FtAddFt(ftaddend1 : super::super::Foundation:: FILETIME, ftaddend2 : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
    unsafe { FtAddFt(core::mem::transmute(ftaddend1), core::mem::transmute(ftaddend2)) }
}
#[inline]
pub unsafe fn FtMulDw(ftmultiplier: u32, ftmultiplicand: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    windows_core::link!("mapi32.dll" "system" fn FtMulDw(ftmultiplier : u32, ftmultiplicand : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
    unsafe { FtMulDw(ftmultiplier, core::mem::transmute(ftmultiplicand)) }
}
#[inline]
pub unsafe fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME {
    windows_core::link!("mapi32.dll" "system" fn FtMulDwDw(ftmultiplicand : u32, ftmultiplier : u32) -> super::super::Foundation:: FILETIME);
    unsafe { FtMulDwDw(ftmultiplicand, ftmultiplier) }
}
#[inline]
pub unsafe fn FtNegFt(ft: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    windows_core::link!("mapi32.dll" "system" fn FtNegFt(ft : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
    unsafe { FtNegFt(core::mem::transmute(ft)) }
}
#[inline]
pub unsafe fn FtSubFt(ftminuend: super::super::Foundation::FILETIME, ftsubtrahend: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    windows_core::link!("mapi32.dll" "system" fn FtSubFt(ftminuend : super::super::Foundation:: FILETIME, ftsubtrahend : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
    unsafe { FtSubFt(core::mem::transmute(ftminuend), core::mem::transmute(ftsubtrahend)) }
}
#[inline]
pub unsafe fn FtgRegisterIdleRoutine(lpfnidle: PFNIDLE, lpvidleparam: *mut core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut core::ffi::c_void {
    windows_core::link!("mapi32.dll" "system" fn FtgRegisterIdleRoutine(lpfnidle : PFNIDLE, lpvidleparam : *mut core::ffi::c_void, priidle : i16, csecidle : u32, iroidle : u16) -> *mut core::ffi::c_void);
    unsafe { FtgRegisterIdleRoutine(lpfnidle, lpvidleparam as _, priidle, csecidle, iroidle) }
}
#[inline]
pub unsafe fn HrAddColumns<P0>(lptbl: P0, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPITable>,
{
    windows_core::link!("mapi32.dll" "system" fn HrAddColumns(lptbl : * mut core::ffi::c_void, lpproptagcolumnsnew : *mut SPropTagArray, lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER) -> windows_core::HRESULT);
    unsafe { HrAddColumns(lptbl.param().abi(), lpproptagcolumnsnew as _, lpallocatebuffer, lpfreebuffer).ok() }
}
#[inline]
pub unsafe fn HrAddColumnsEx<P0>(lptbl: P0, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, lpfnfiltercolumns: isize) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPITable>,
{
    windows_core::link!("mapi32.dll" "system" fn HrAddColumnsEx(lptbl : * mut core::ffi::c_void, lpproptagcolumnsnew : *mut SPropTagArray, lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER, lpfnfiltercolumns : isize) -> windows_core::HRESULT);
    unsafe { HrAddColumnsEx(lptbl.param().abi(), lpproptagcolumnsnew as _, lpallocatebuffer, lpfreebuffer, lpfnfiltercolumns).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn HrAllocAdviseSink(lpfncallback: LPNOTIFCALLBACK, lpvcontext: *mut core::ffi::c_void, lppadvisesink: *mut Option<IMAPIAdviseSink>) -> windows_core::Result<()> {
    windows_core::link!("mapi32.dll" "system" fn HrAllocAdviseSink(lpfncallback : LPNOTIFCALLBACK, lpvcontext : *mut core::ffi::c_void, lppadvisesink : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HrAllocAdviseSink(lpfncallback, lpvcontext as _, core::mem::transmute(lppadvisesink)).ok() }
}
#[inline]
pub unsafe fn HrDispatchNotifications(ulflags: u32) -> windows_core::Result<()> {
    windows_core::link!("mapi32.dll" "system" fn HrDispatchNotifications(ulflags : u32) -> windows_core::HRESULT);
    unsafe { HrDispatchNotifications(ulflags).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn HrGetOneProp<P0>(lpmapiprop: P0, ulproptag: u32, lppprop: *mut *mut SPropValue) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPIProp>,
{
    windows_core::link!("mapi32.dll" "system" fn HrGetOneProp(lpmapiprop : * mut core::ffi::c_void, ulproptag : u32, lppprop : *mut *mut SPropValue) -> windows_core::HRESULT);
    unsafe { HrGetOneProp(lpmapiprop.param().abi(), ulproptag, lppprop as _).ok() }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn HrIStorageFromStream<P0>(lpunkin: P0, lpinterface: *mut windows_core::GUID, ulflags: u32, lppstorageout: *mut Option<super::Com::StructuredStorage::IStorage>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mapi32.dll" "system" fn HrIStorageFromStream(lpunkin : * mut core::ffi::c_void, lpinterface : *mut windows_core::GUID, ulflags : u32, lppstorageout : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HrIStorageFromStream(lpunkin.param().abi(), lpinterface as _, ulflags, core::mem::transmute(lppstorageout)).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn HrQueryAllRows<P0>(lptable: P0, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPITable>,
{
    windows_core::link!("mapi32.dll" "system" fn HrQueryAllRows(lptable : * mut core::ffi::c_void, lpproptags : *mut SPropTagArray, lprestriction : *mut SRestriction, lpsortorderset : *mut SSortOrderSet, crowsmax : i32, lpprows : *mut *mut SRowSet) -> windows_core::HRESULT);
    unsafe { HrQueryAllRows(lptable.param().abi(), lpproptags as _, lprestriction as _, lpsortorderset as _, crowsmax, lpprows as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn HrSetOneProp<P0>(lpmapiprop: P0, lpprop: *mut SPropValue) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMAPIProp>,
{
    windows_core::link!("mapi32.dll" "system" fn HrSetOneProp(lpmapiprop : * mut core::ffi::c_void, lpprop : *mut SPropValue) -> windows_core::HRESULT);
    unsafe { HrSetOneProp(lpmapiprop.param().abi(), lpprop as _).ok() }
}
#[inline]
pub unsafe fn HrThisThreadAdviseSink<P0>(lpadvisesink: P0) -> windows_core::Result<IMAPIAdviseSink>
where
    P0: windows_core::Param<IMAPIAdviseSink>,
{
    windows_core::link!("mapi32.dll" "system" fn HrThisThreadAdviseSink(lpadvisesink : * mut core::ffi::c_void, lppadvisesink : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HrThisThreadAdviseSink(lpadvisesink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn LPropCompareProp(lpspropvaluea : *mut SPropValue, lpspropvalueb : *mut SPropValue) -> i32);
    unsafe { LPropCompareProp(lpspropvaluea as _, lpspropvalueb as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue {
    windows_core::link!("mapi32.dll" "system" fn LpValFindProp(ulproptag : u32, cvalues : u32, lpproparray : *mut SPropValue) -> *mut SPropValue);
    unsafe { LpValFindProp(ulproptag, cvalues, lpproparray as _) }
}
#[inline]
pub unsafe fn MAPIDeinitIdle() {
    windows_core::link!("mapi32.dll" "system" fn MAPIDeinitIdle());
    unsafe { MAPIDeinitIdle() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn MAPIGetDefaultMalloc() -> Option<super::Com::IMalloc> {
    windows_core::link!("mapi32.dll" "system" fn MAPIGetDefaultMalloc() -> Option < super::Com:: IMalloc >);
    unsafe { MAPIGetDefaultMalloc() }
}
#[inline]
pub unsafe fn MAPIInitIdle(lpvreserved: *mut core::ffi::c_void) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn MAPIInitIdle(lpvreserved : *mut core::ffi::c_void) -> i32);
    unsafe { MAPIInitIdle(lpvreserved as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenStreamOnFile(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: Option<*const i8>) -> windows_core::Result<super::Com::IStream> {
    windows_core::link!("mapi32.dll" "system" fn OpenStreamOnFile(lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER, ulflags : u32, lpszfilename : *const i8, lpszprefix : *const i8, lppstream : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OpenStreamOnFile(lpallocatebuffer, lpfreebuffer, ulflags, lpszfilename, lpszprefix.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue {
    windows_core::link!("mapi32.dll" "system" fn PpropFindProp(lpproparray : *mut SPropValue, cvalues : u32, ulproptag : u32) -> *mut SPropValue);
    unsafe { PpropFindProp(lpproparray as _, cvalues, ulproptag) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: LPALLOCATEMORE, lpvobject: *mut core::ffi::c_void) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn PropCopyMore(lpspropvaluedest : *mut SPropValue, lpspropvaluesrc : *mut SPropValue, lpfallocmore : LPALLOCATEMORE, lpvobject : *mut core::ffi::c_void) -> i32);
    unsafe { PropCopyMore(lpspropvaluedest as _, lpspropvaluesrc as _, lpfallocmore, lpvobject as _) }
}
#[inline]
pub unsafe fn RTFSync<P0>(lpmessage: P0, ulflags: u32, lpfmessageupdated: *mut windows_core::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMessage>,
{
    windows_core::link!("mapi32.dll" "system" fn RTFSync(lpmessage : * mut core::ffi::c_void, ulflags : u32, lpfmessageupdated : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RTFSync(lpmessage.param().abi(), ulflags, lpfmessageupdated as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut core::ffi::c_void, lpcb: *mut u32) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScCopyNotifications(cnotification : i32, lpnotifications : *mut NOTIFICATION, lpvdst : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
    unsafe { ScCopyNotifications(cnotification, lpnotifications as _, lpvdst as _, lpcb as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut core::ffi::c_void, lpcb: *mut u32) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScCopyProps(cvalues : i32, lpproparray : *mut SPropValue, lpvdst : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
    unsafe { ScCopyProps(cvalues, lpproparray as _, lpvdst as _, lpcb as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScCountNotifications(cnotifications : i32, lpnotifications : *mut NOTIFICATION, lpcb : *mut u32) -> i32);
    unsafe { ScCountNotifications(cnotifications, lpnotifications as _, lpcb as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScCountProps(cvalues : i32, lpproparray : *mut SPropValue, lpcb : *mut u32) -> i32);
    unsafe { ScCountProps(cvalues, lpproparray as _, lpcb as _) }
}
#[inline]
pub unsafe fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScCreateConversationIndex(cbparent : u32, lpbparent : *mut u8, lpcbconvindex : *mut u32, lppbconvindex : *mut *mut u8) -> i32);
    unsafe { ScCreateConversationIndex(cbparent, lpbparent as _, lpcbconvindex as _, lppbconvindex as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: LPALLOCATEBUFFER, lppproparray: *mut *mut SPropValue) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScDupPropset(cvalues : i32, lpproparray : *mut SPropValue, lpallocatebuffer : LPALLOCATEBUFFER, lppproparray : *mut *mut SPropValue) -> i32);
    unsafe { ScDupPropset(cvalues, lpproparray as _, lpallocatebuffer, lppproparray as _) }
}
#[inline]
pub unsafe fn ScInitMapiUtil(ulflags: u32) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScInitMapiUtil(ulflags : u32) -> i32);
    unsafe { ScInitMapiUtil(ulflags) }
}
#[inline]
pub unsafe fn ScLocalPathFromUNC<P0>(lpszunc: P0, lpszlocal: &[u8]) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mapi32.dll" "system" fn ScLocalPathFromUNC(lpszunc : windows_core::PCSTR, lpszlocal : windows_core::PCSTR, cchlocal : u32) -> i32);
    unsafe { ScLocalPathFromUNC(lpszunc.param().abi(), core::mem::transmute(lpszlocal.as_ptr()), lpszlocal.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut core::ffi::c_void, lpvbasenew: *mut core::ffi::c_void, lpcb: *mut u32) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScRelocNotifications(cnotification : i32, lpnotifications : *mut NOTIFICATION, lpvbaseold : *mut core::ffi::c_void, lpvbasenew : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
    unsafe { ScRelocNotifications(cnotification, lpnotifications as _, lpvbaseold as _, lpvbasenew as _, lpcb as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut core::ffi::c_void, lpvbasenew: *mut core::ffi::c_void, lpcb: *mut u32) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn ScRelocProps(cvalues : i32, lpproparray : *mut SPropValue, lpvbaseold : *mut core::ffi::c_void, lpvbasenew : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
    unsafe { ScRelocProps(cvalues, lpproparray as _, lpvbaseold as _, lpvbasenew as _, lpcb as _) }
}
#[inline]
pub unsafe fn ScUNCFromLocalPath<P0>(lpszlocal: P0, lpszunc: &[u8]) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mapi32.dll" "system" fn ScUNCFromLocalPath(lpszlocal : windows_core::PCSTR, lpszunc : windows_core::PCSTR, cchunc : u32) -> i32);
    unsafe { ScUNCFromLocalPath(lpszlocal.param().abi(), core::mem::transmute(lpszunc.as_ptr()), lpszunc.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8 {
    windows_core::link!("mapi32.dll" "system" fn SzFindCh(lpsz : *mut i8, ch : u16) -> *mut i8);
    unsafe { SzFindCh(lpsz as _, ch) }
}
#[inline]
pub unsafe fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8 {
    windows_core::link!("mapi32.dll" "system" fn SzFindLastCh(lpsz : *mut i8, ch : u16) -> *mut i8);
    unsafe { SzFindLastCh(lpsz as _, ch) }
}
#[inline]
pub unsafe fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8 {
    windows_core::link!("mapi32.dll" "system" fn SzFindSz(lpsz : *mut i8, lpszkey : *mut i8) -> *mut i8);
    unsafe { SzFindSz(lpsz as _, lpszkey as _) }
}
#[inline]
pub unsafe fn UFromSz(lpsz: *mut i8) -> u32 {
    windows_core::link!("mapi32.dll" "system" fn UFromSz(lpsz : *mut i8) -> u32);
    unsafe { UFromSz(lpsz as _) }
}
#[inline]
pub unsafe fn UlAddRef(lpunk: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mapi32.dll" "system" fn UlAddRef(lpunk : *mut core::ffi::c_void) -> u32);
    unsafe { UlAddRef(lpunk as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32 {
    windows_core::link!("mapi32.dll" "system" fn UlPropSize(lpspropvalue : *mut SPropValue) -> u32);
    unsafe { UlPropSize(lpspropvalue as _) }
}
#[inline]
pub unsafe fn UlRelease(lpunk: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mapi32.dll" "system" fn UlRelease(lpunk : *mut core::ffi::c_void) -> u32);
    unsafe { UlRelease(lpunk as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WrapCompressedRTFStream<P0>(lpcompressedrtfstream: P0, ulflags: u32) -> windows_core::Result<super::Com::IStream>
where
    P0: windows_core::Param<super::Com::IStream>,
{
    windows_core::link!("mapi32.dll" "system" fn WrapCompressedRTFStream(lpcompressedrtfstream : * mut core::ffi::c_void, ulflags : u32, lpuncompressedrtfstream : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WrapCompressedRTFStream(lpcompressedrtfstream.param().abi(), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> windows_core::Result<()> {
    windows_core::link!("mapi32.dll" "system" fn WrapStoreEntryID(ulflags : u32, lpszdllname : *const i8, cborigentry : u32, lporigentry : *const ENTRYID, lpcbwrappedentry : *mut u32, lppwrappedentry : *mut *mut ENTRYID) -> windows_core::HRESULT);
    unsafe { WrapStoreEntryID(ulflags, lpszdllname, cborigentry, lporigentry, lpcbwrappedentry as _, lppwrappedentry as _).ok() }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ADRENTRY {
    pub ulReserved1: u32,
    pub cValues: u32,
    pub rgPropVals: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for ADRENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ADRLIST {
    pub cEntries: u32,
    pub aEntries: [ADRENTRY; 1],
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
impl Default for ADRPARM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CALLERRELEASE = Option<unsafe extern "system" fn(ulcallerdata: u32, lptbldata: windows_core::Ref<ITableData>, lpvue: windows_core::Ref<IMAPITable>)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRControl: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLCHECKBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRPropertyName: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLCOMBOBOX {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPRPropertyName: u32,
    pub ulPRTableName: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLDDLBX {
    pub ulFlags: u32,
    pub ulPRDisplayProperty: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLEDIT {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPropTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLGROUPBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLLABEL {
    pub ulbLpszLabelName: u32,
    pub ulFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLLBX {
    pub ulFlags: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLMVDDLBX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLMVLISTBOX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLPAGE {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulbLpszComponent: u32,
    pub ulContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBLRADIOBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulcButtons: u32,
    pub ulPropTag: u32,
    pub lReturnValue: i32,
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
impl Default for DTPAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENTRYID {
    pub abFlags: [u8; 4],
    pub ab: [u8; 1],
}
impl Default for ENTRYID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ERROR_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub scode: i32,
    pub ulFlags: u32,
    pub lpMAPIError: *mut MAPIERROR,
}
impl Default for ERROR_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTENDED_NOTIFICATION {
    pub ulEvent: u32,
    pub cb: u32,
    pub pbEventParameters: *mut u8,
}
impl Default for EXTENDED_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLATENTRY {
    pub cb: u32,
    pub abEntry: [u8; 1],
}
impl Default for FLATENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLATENTRYLIST {
    pub cEntries: u32,
    pub cbEntries: u32,
    pub abEntries: [u8; 1],
}
impl Default for FLATENTRYLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLATMTSIDLIST {
    pub cMTSIDs: u32,
    pub cbMTSIDs: u32,
    pub abMTSIDs: [u8; 1],
}
impl Default for FLATMTSIDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlagList {
    pub cFlags: u32,
    pub ulFlag: [u32; 1],
}
impl Default for FlagList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Gender(pub i32);
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulcreateflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyEntries<P2>(&self, lpentries: *const SBinaryArray, uluiparam: Option<usize>, lpprogress: P2, ulflags: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyEntries)(windows_core::Interface::as_raw(self), lpentries, uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
    pub unsafe fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteEntries)(windows_core::Interface::as_raw(self), lpentries, ulflags).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResolveNames(&self, lpproptagarray: Option<*const SPropTagArray>, ulflags: u32, lpadrlist: *const ADRLIST) -> windows_core::Result<FlagList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResolveNames)(windows_core::Interface::as_raw(self), lpproptagarray.unwrap_or(core::mem::zeroed()) as _, ulflags, lpadrlist, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IABContainer_Impl: IMAPIContainer_Impl {
    fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> windows_core::Result<IMAPIProp>;
    fn CopyEntries(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::Result<()>;
    fn ResolveNames(&self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> windows_core::Result<FlagList>;
}
#[cfg(feature = "Win32_System_Com")]
impl IABContainer_Vtbl {
    pub const fn new<Identity: IABContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEntry<Identity: IABContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IABContainer_Impl::CreateEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulcreateflags)) {
                    Ok(ok__) => {
                        lppmapipropentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyEntries<Identity: IABContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IABContainer_Impl::CopyEntries(this, core::mem::transmute_copy(&lpentries), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn DeleteEntries<Identity: IABContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IABContainer_Impl::DeleteEntries(this, core::mem::transmute_copy(&lpentries), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn ResolveNames<Identity: IABContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut FlagList) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IABContainer_Impl::ResolveNames(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpadrlist)) {
                    Ok(ok__) => {
                        lpflaglist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMAPIContainer_Vtbl::new::<Identity, OFFSET>(),
            CreateEntry: CreateEntry::<Identity, OFFSET>,
            CopyEntries: CopyEntries::<Identity, OFFSET>,
            DeleteEntries: DeleteEntries::<Identity, OFFSET>,
            ResolveNames: ResolveNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IABContainer as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID || iid == &<IMAPIContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IABContainer {}
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
        unsafe { (windows_core::Interface::vtable(self).OpenEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid as _, lpinterface as _, ulflags, lpulobjtype as _, core::mem::transmute(lppunk)).ok() }
    }
    pub unsafe fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CompareEntryIDs)(windows_core::Interface::as_raw(self), cbentryid1, lpentryid1 as _, cbentryid2, lpentryid2 as _, ulflags, lpulresult as _).ok() }
    }
    pub unsafe fn Advise<P3>(&self, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: P3, lpulconnection: *mut u32) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IMAPIAdviseSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), cbentryid, lpentryid as _, uleventmask, lpadvisesink.param().abi(), lpulconnection as _).ok() }
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), ulconnection).ok() }
    }
    pub unsafe fn CreateOneOff(&self, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateOneOff)(windows_core::Interface::as_raw(self), lpszname as _, lpszadrtype as _, lpszaddress as _, ulflags, lpcbentryid as _, lppentryid as _).ok() }
    }
    pub unsafe fn NewEntry(&self, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NewEntry)(windows_core::Interface::as_raw(self), uluiparam, ulflags, cbeidcontainer, lpeidcontainer as _, cbeidnewentrytpl, lpeidnewentrytpl as _, lpcbeidnewentry as _, lppeidnewentry as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResolveName(&self, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResolveName)(windows_core::Interface::as_raw(self), uluiparam, ulflags, lpsznewentrytitle as _, lpadrlist as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), lpuluiparam as _, lpadrparms as _, lppadrlist as _).ok() }
    }
    pub unsafe fn Details(&self, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Details)(windows_core::Interface::as_raw(self), lpuluiparam as _, lpfndismiss, lpvdismisscontext as _, cbentryid, lpentryid as _, lpfbuttoncallback, lpvbuttoncontext as _, lpszbuttontext as _, ulflags).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecipOptions(&self, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecipOptions)(windows_core::Interface::as_raw(self), uluiparam, ulflags, lprecip as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDefaultRecipOpt(&self, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryDefaultRecipOpt)(windows_core::Interface::as_raw(self), lpszadrtype as _, ulflags, lpcvalues as _, lppoptions as _).ok() }
    }
    pub unsafe fn GetPAB(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPAB)(windows_core::Interface::as_raw(self), lpcbentryid as _, lppentryid as _).ok() }
    }
    pub unsafe fn SetPAB(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPAB)(windows_core::Interface::as_raw(self), cbentryid, lpentryid as _).ok() }
    }
    pub unsafe fn GetDefaultDir(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDefaultDir)(windows_core::Interface::as_raw(self), lpcbentryid as _, lppentryid as _).ok() }
    }
    pub unsafe fn SetDefaultDir(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultDir)(windows_core::Interface::as_raw(self), cbentryid, lpentryid as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSearchPath(&self, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSearchPath)(windows_core::Interface::as_raw(self), ulflags, lppsearchpath as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSearchPath(&self, ulflags: u32, lpsearchpath: *mut SRowSet) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSearchPath)(windows_core::Interface::as_raw(self), ulflags, lpsearchpath as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRecips(&self, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrepareRecips)(windows_core::Interface::as_raw(self), ulflags, lpproptagarray as _, lpreciplist as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IAddrBook_Impl: IMAPIProp_Impl {
    fn OpenEntry(&self, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> windows_core::Result<()>;
    fn Advise(&self, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: windows_core::Ref<IMAPIAdviseSink>, lpulconnection: *mut u32) -> windows_core::Result<()>;
    fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()>;
    fn CreateOneOff(&self, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()>;
    fn NewEntry(&self, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> windows_core::Result<()>;
    fn ResolveName(&self, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> windows_core::Result<()>;
    fn Address(&self, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> windows_core::Result<()>;
    fn Details(&self, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> windows_core::Result<()>;
    fn RecipOptions(&self, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> windows_core::Result<()>;
    fn QueryDefaultRecipOpt(&self, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> windows_core::Result<()>;
    fn GetPAB(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()>;
    fn SetPAB(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::Result<()>;
    fn GetDefaultDir(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()>;
    fn SetDefaultDir(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::Result<()>;
    fn GetSearchPath(&self, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> windows_core::Result<()>;
    fn SetSearchPath(&self, ulflags: u32, lpsearchpath: *mut SRowSet) -> windows_core::Result<()>;
    fn PrepareRecips(&self, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAddrBook_Vtbl {
    pub const fn new<Identity: IAddrBook_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenEntry<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::OpenEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulobjtype), core::mem::transmute_copy(&lppunk)).into()
            }
        }
        unsafe extern "system" fn CompareEntryIDs<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::CompareEntryIDs(this, core::mem::transmute_copy(&cbentryid1), core::mem::transmute_copy(&lpentryid1), core::mem::transmute_copy(&cbentryid2), core::mem::transmute_copy(&lpentryid2), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulresult)).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: *mut core::ffi::c_void, lpulconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::Advise(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&uleventmask), core::mem::transmute_copy(&lpadvisesink), core::mem::transmute_copy(&lpulconnection)).into()
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::Unadvise(this, core::mem::transmute_copy(&ulconnection)).into()
            }
        }
        unsafe extern "system" fn CreateOneOff<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::CreateOneOff(this, core::mem::transmute_copy(&lpszname), core::mem::transmute_copy(&lpszadrtype), core::mem::transmute_copy(&lpszaddress), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcbentryid), core::mem::transmute_copy(&lppentryid)).into()
            }
        }
        unsafe extern "system" fn NewEntry<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::NewEntry(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbeidcontainer), core::mem::transmute_copy(&lpeidcontainer), core::mem::transmute_copy(&cbeidnewentrytpl), core::mem::transmute_copy(&lpeidnewentrytpl), core::mem::transmute_copy(&lpcbeidnewentry), core::mem::transmute_copy(&lppeidnewentry)).into()
            }
        }
        unsafe extern "system" fn ResolveName<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::ResolveName(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpsznewentrytitle), core::mem::transmute_copy(&lpadrlist)).into()
            }
        }
        unsafe extern "system" fn Address<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::Address(this, core::mem::transmute_copy(&lpuluiparam), core::mem::transmute_copy(&lpadrparms), core::mem::transmute_copy(&lppadrlist)).into()
            }
        }
        unsafe extern "system" fn Details<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::Details(this, core::mem::transmute_copy(&lpuluiparam), core::mem::transmute_copy(&lpfndismiss), core::mem::transmute_copy(&lpvdismisscontext), core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpfbuttoncallback), core::mem::transmute_copy(&lpvbuttoncontext), core::mem::transmute_copy(&lpszbuttontext), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn RecipOptions<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::RecipOptions(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lprecip)).into()
            }
        }
        unsafe extern "system" fn QueryDefaultRecipOpt<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::QueryDefaultRecipOpt(this, core::mem::transmute_copy(&lpszadrtype), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcvalues), core::mem::transmute_copy(&lppoptions)).into()
            }
        }
        unsafe extern "system" fn GetPAB<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::GetPAB(this, core::mem::transmute_copy(&lpcbentryid), core::mem::transmute_copy(&lppentryid)).into()
            }
        }
        unsafe extern "system" fn SetPAB<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::SetPAB(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid)).into()
            }
        }
        unsafe extern "system" fn GetDefaultDir<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::GetDefaultDir(this, core::mem::transmute_copy(&lpcbentryid), core::mem::transmute_copy(&lppentryid)).into()
            }
        }
        unsafe extern "system" fn SetDefaultDir<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::SetDefaultDir(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid)).into()
            }
        }
        unsafe extern "system" fn GetSearchPath<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::GetSearchPath(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppsearchpath)).into()
            }
        }
        unsafe extern "system" fn SetSearchPath<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpsearchpath: *mut SRowSet) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::SetSearchPath(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpsearchpath)).into()
            }
        }
        unsafe extern "system" fn PrepareRecips<Identity: IAddrBook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrBook_Impl::PrepareRecips(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&lpreciplist)).into()
            }
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            OpenEntry: OpenEntry::<Identity, OFFSET>,
            CompareEntryIDs: CompareEntryIDs::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            CreateOneOff: CreateOneOff::<Identity, OFFSET>,
            NewEntry: NewEntry::<Identity, OFFSET>,
            ResolveName: ResolveName::<Identity, OFFSET>,
            Address: Address::<Identity, OFFSET>,
            Details: Details::<Identity, OFFSET>,
            RecipOptions: RecipOptions::<Identity, OFFSET>,
            QueryDefaultRecipOpt: QueryDefaultRecipOpt::<Identity, OFFSET>,
            GetPAB: GetPAB::<Identity, OFFSET>,
            SetPAB: SetPAB::<Identity, OFFSET>,
            GetDefaultDir: GetDefaultDir::<Identity, OFFSET>,
            SetDefaultDir: SetDefaultDir::<Identity, OFFSET>,
            GetSearchPath: GetSearchPath::<Identity, OFFSET>,
            SetSearchPath: SetSearchPath::<Identity, OFFSET>,
            PrepareRecips: PrepareRecips::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAddrBook as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAddrBook {}
windows_core::imp::define_interface!(IAttach, IAttach_Vtbl, 0);
impl core::ops::Deref for IAttach {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAttach, windows_core::IUnknown, IMAPIProp);
#[repr(C)]
#[doc(hidden)]
pub struct IAttach_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAttach_Impl: IMAPIProp_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl IAttach_Vtbl {
    pub const fn new<Identity: IAttach_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAttach as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAttach {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulcreateflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyEntries<P2>(&self, lpentries: *const SBinaryArray, uluiparam: Option<usize>, lpprogress: P2, ulflags: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyEntries)(windows_core::Interface::as_raw(self), lpentries, uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
    pub unsafe fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteEntries)(windows_core::Interface::as_raw(self), lpentries, ulflags).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResolveNames(&self, lpproptagarray: Option<*const SPropTagArray>, ulflags: u32, lpadrlist: *const ADRLIST) -> windows_core::Result<FlagList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResolveNames)(windows_core::Interface::as_raw(self), lpproptagarray.unwrap_or(core::mem::zeroed()) as _, ulflags, lpadrlist, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IDistList_Impl: IMAPIContainer_Impl {
    fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> windows_core::Result<IMAPIProp>;
    fn CopyEntries(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::Result<()>;
    fn ResolveNames(&self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> windows_core::Result<FlagList>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDistList_Vtbl {
    pub const fn new<Identity: IDistList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEntry<Identity: IDistList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDistList_Impl::CreateEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulcreateflags)) {
                    Ok(ok__) => {
                        lppmapipropentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyEntries<Identity: IDistList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDistList_Impl::CopyEntries(this, core::mem::transmute_copy(&lpentries), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn DeleteEntries<Identity: IDistList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDistList_Impl::DeleteEntries(this, core::mem::transmute_copy(&lpentries), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn ResolveNames<Identity: IDistList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut FlagList) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDistList_Impl::ResolveNames(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpadrlist)) {
                    Ok(ok__) => {
                        lpflaglist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMAPIContainer_Vtbl::new::<Identity, OFFSET>(),
            CreateEntry: CreateEntry::<Identity, OFFSET>,
            CopyEntries: CopyEntries::<Identity, OFFSET>,
            DeleteEntries: DeleteEntries::<Identity, OFFSET>,
            ResolveNames: ResolveNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDistList as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID || iid == &<IMAPIContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDistList {}
windows_core::imp::define_interface!(IMAPIAdviseSink, IMAPIAdviseSink_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IMAPIAdviseSink, windows_core::IUnknown);
impl IMAPIAdviseSink {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnNotify(&self, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), cnotif, lpnotifications as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIAdviseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut NOTIFICATION) -> u32,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnNotify: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIAdviseSink_Impl: windows_core::IUnknownImpl {
    fn OnNotify(&self, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32;
}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIAdviseSink_Vtbl {
    pub const fn new<Identity: IMAPIAdviseSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNotify<Identity: IMAPIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIAdviseSink_Impl::OnNotify(this, core::mem::transmute_copy(&cnotif), core::mem::transmute_copy(&lpnotifications))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIAdviseSink {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentsTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHierarchyTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, lpinterface as _, ulflags, lpulobjtype as _, core::mem::transmute(lppunk)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: Option<*const SRestriction>, lpcontainerlist: Option<*const SBinaryArray>, ulsearchflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSearchCriteria)(windows_core::Interface::as_raw(self), lprestriction.unwrap_or(core::mem::zeroed()) as _, lpcontainerlist.unwrap_or(core::mem::zeroed()) as _, ulsearchflags).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSearchCriteria)(windows_core::Interface::as_raw(self), ulflags, lpprestriction as _, lppcontainerlist as _, lpulsearchstate.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIContainer_Impl: IMAPIProp_Impl {
    fn GetContentsTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn GetHierarchyTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> windows_core::Result<()>;
    fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIContainer_Vtbl {
    pub const fn new<Identity: IMAPIContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContentsTable<Identity: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMAPIContainer_Impl::GetContentsTable(this, core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHierarchyTable<Identity: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMAPIContainer_Impl::GetHierarchyTable(this, core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenEntry<Identity: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIContainer_Impl::OpenEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulobjtype), core::mem::transmute_copy(&lppunk)).into()
            }
        }
        unsafe extern "system" fn SetSearchCriteria<Identity: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIContainer_Impl::SetSearchCriteria(this, core::mem::transmute_copy(&lprestriction), core::mem::transmute_copy(&lpcontainerlist), core::mem::transmute_copy(&ulsearchflags)).into()
            }
        }
        unsafe extern "system" fn GetSearchCriteria<Identity: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIContainer_Impl::GetSearchCriteria(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpprestriction), core::mem::transmute_copy(&lppcontainerlist), core::mem::transmute_copy(&lpulsearchstate)).into()
            }
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            GetContentsTable: GetContentsTable::<Identity, OFFSET>,
            GetHierarchyTable: GetHierarchyTable::<Identity, OFFSET>,
            OpenEntry: OpenEntry::<Identity, OFFSET>,
            SetSearchCriteria: SetSearchCriteria::<Identity, OFFSET>,
            GetSearchCriteria: GetSearchCriteria::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIContainer as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIContainer {}
windows_core::imp::define_interface!(IMAPIControl, IMAPIControl_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IMAPIControl, windows_core::IUnknown);
impl IMAPIControl {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32) -> windows_core::Result<*mut MAPIERROR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Activate(&self, ulflags: u32, uluiparam: Option<usize>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), ulflags, uluiparam.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetState(&self, ulflags: u32, lpulstate: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), ulflags, lpulstate as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *mut *mut MAPIERROR) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IMAPIControl_Impl: windows_core::IUnknownImpl {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32) -> windows_core::Result<*mut MAPIERROR>;
    fn Activate(&self, ulflags: u32, uluiparam: usize) -> windows_core::Result<()>;
    fn GetState(&self, ulflags: u32, lpulstate: *mut u32) -> windows_core::Result<()>;
}
impl IMAPIControl_Vtbl {
    pub const fn new<Identity: IMAPIControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLastError<Identity: IMAPIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMAPIControl_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lppmapierror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Activate<Identity: IMAPIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, uluiparam: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIControl_Impl::Activate(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&uluiparam)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IMAPIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpulstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIControl_Impl::GetState(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulstate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMAPIControl {}
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
        unsafe { (windows_core::Interface::vtable(self).CreateMessage)(windows_core::Interface::as_raw(self), lpinterface as _, ulflags, core::mem::transmute(lppmessage)).ok() }
    }
    pub unsafe fn CopyMessages<P4>(&self, lpmsglist: *const SBinaryArray, lpinterface: Option<*const windows_core::GUID>, lpdestfolder: *const core::ffi::c_void, uluiparam: Option<usize>, lpprogress: P4, ulflags: u32) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyMessages)(windows_core::Interface::as_raw(self), lpmsglist, lpinterface.unwrap_or(core::mem::zeroed()) as _, lpdestfolder, uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
    pub unsafe fn DeleteMessages<P2>(&self, lpmsglist: *const SBinaryArray, uluiparam: Option<usize>, lpprogress: P2, ulflags: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteMessages)(windows_core::Interface::as_raw(self), lpmsglist, uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
    pub unsafe fn CreateFolder(&self, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: Option<*const i8>, lpinterface: Option<*const windows_core::GUID>, ulflags: u32) -> windows_core::Result<IMAPIFolder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFolder)(windows_core::Interface::as_raw(self), ulfoldertype, lpszfoldername, lpszfoldercomment.unwrap_or(core::mem::zeroed()) as _, lpinterface.unwrap_or(core::mem::zeroed()) as _, ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyFolder<P6>(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: Option<*const windows_core::GUID>, lpdestfolder: *const core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: Option<usize>, lpprogress: P6, ulflags: u32) -> windows_core::Result<()>
    where
        P6: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyFolder)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, lpinterface.unwrap_or(core::mem::zeroed()) as _, lpdestfolder, lpsznewfoldername, uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
    pub unsafe fn DeleteFolder<P3>(&self, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: Option<usize>, lpprogress: P3, ulflags: u32) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteFolder)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
    pub unsafe fn SetReadFlags<P2>(&self, lpmsglist: *const SBinaryArray, uluiparam: Option<usize>, lpprogress: P2, ulflags: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReadFlags)(windows_core::Interface::as_raw(self), lpmsglist, uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
    pub unsafe fn GetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessageStatus)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetMessageStatus)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulnewstatus, ulnewstatusmask, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SaveContentsSort(&self, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveContentsSort)(windows_core::Interface::as_raw(self), lpsortcriteria, ulflags).ok() }
    }
    pub unsafe fn EmptyFolder<P1>(&self, uluiparam: Option<usize>, lpprogress: P1, ulflags: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).EmptyFolder)(windows_core::Interface::as_raw(self), uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIFolder_Impl: IMAPIContainer_Impl {
    fn CreateMessage(&self, lpinterface: *mut windows_core::GUID, ulflags: u32, lppmessage: windows_core::OutRef<IMessage>) -> windows_core::Result<()>;
    fn CopyMessages(&self, lpmsglist: *const SBinaryArray, lpinterface: *const windows_core::GUID, lpdestfolder: *const core::ffi::c_void, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn DeleteMessages(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn CreateFolder(&self, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const windows_core::GUID, ulflags: u32) -> windows_core::Result<IMAPIFolder>;
    fn CopyFolder(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const windows_core::GUID, lpdestfolder: *const core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn DeleteFolder(&self, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn SetReadFlags(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn GetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::Result<u32>;
    fn SetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32) -> windows_core::Result<u32>;
    fn SaveContentsSort(&self, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> windows_core::Result<()>;
    fn EmptyFolder(&self, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIFolder_Vtbl {
    pub const fn new<Identity: IMAPIFolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMessage<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpinterface: *mut windows_core::GUID, ulflags: u32, lppmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIFolder_Impl::CreateMessage(this, core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppmessage)).into()
            }
        }
        unsafe extern "system" fn CopyMessages<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsglist: *const SBinaryArray, lpinterface: *const windows_core::GUID, lpdestfolder: *const core::ffi::c_void, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIFolder_Impl::CopyMessages(this, core::mem::transmute_copy(&lpmsglist), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&lpdestfolder), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn DeleteMessages<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIFolder_Impl::DeleteMessages(this, core::mem::transmute_copy(&lpmsglist), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn CreateFolder<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const windows_core::GUID, ulflags: u32, lppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMAPIFolder_Impl::CreateFolder(this, core::mem::transmute_copy(&ulfoldertype), core::mem::transmute_copy(&lpszfoldername), core::mem::transmute_copy(&lpszfoldercomment), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyFolder<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const windows_core::GUID, lpdestfolder: *const core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIFolder_Impl::CopyFolder(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&lpdestfolder), core::mem::transmute_copy(&lpsznewfoldername), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn DeleteFolder<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIFolder_Impl::DeleteFolder(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn SetReadFlags<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIFolder_Impl::SetReadFlags(this, core::mem::transmute_copy(&lpmsglist), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn GetMessageStatus<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32, lpulmessagestatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMAPIFolder_Impl::GetMessageStatus(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpulmessagestatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageStatus<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32, lpuloldstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMAPIFolder_Impl::SetMessageStatus(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulnewstatus), core::mem::transmute_copy(&ulnewstatusmask)) {
                    Ok(ok__) => {
                        lpuloldstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SaveContentsSort<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIFolder_Impl::SaveContentsSort(this, core::mem::transmute_copy(&lpsortcriteria), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn EmptyFolder<Identity: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIFolder_Impl::EmptyFolder(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        Self {
            base__: IMAPIContainer_Vtbl::new::<Identity, OFFSET>(),
            CreateMessage: CreateMessage::<Identity, OFFSET>,
            CopyMessages: CopyMessages::<Identity, OFFSET>,
            DeleteMessages: DeleteMessages::<Identity, OFFSET>,
            CreateFolder: CreateFolder::<Identity, OFFSET>,
            CopyFolder: CopyFolder::<Identity, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, OFFSET>,
            SetReadFlags: SetReadFlags::<Identity, OFFSET>,
            GetMessageStatus: GetMessageStatus::<Identity, OFFSET>,
            SetMessageStatus: SetMessageStatus::<Identity, OFFSET>,
            SaveContentsSort: SaveContentsSort::<Identity, OFFSET>,
            EmptyFolder: EmptyFolder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIFolder as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID || iid == &<IMAPIContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIFolder {}
windows_core::imp::define_interface!(IMAPIProgress, IMAPIProgress_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IMAPIProgress, windows_core::IUnknown);
impl IMAPIProgress {
    pub unsafe fn Progress(&self, ulvalue: u32, ulcount: u32, ultotal: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), ulvalue, ulcount, ultotal).ok() }
    }
    pub unsafe fn GetFlags(&self, lpulflags: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), lpulflags as _).ok() }
    }
    pub unsafe fn GetMax(&self, lpulmax: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMax)(windows_core::Interface::as_raw(self), lpulmax as _).ok() }
    }
    pub unsafe fn GetMin(&self, lpulmin: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMin)(windows_core::Interface::as_raw(self), lpulmin as _).ok() }
    }
    pub unsafe fn SetLimits(&self, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLimits)(windows_core::Interface::as_raw(self), lpulmin as _, lpulmax as _, lpulflags as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIProgress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IMAPIProgress_Impl: windows_core::IUnknownImpl {
    fn Progress(&self, ulvalue: u32, ulcount: u32, ultotal: u32) -> windows_core::Result<()>;
    fn GetFlags(&self, lpulflags: *mut u32) -> windows_core::Result<()>;
    fn GetMax(&self, lpulmax: *mut u32) -> windows_core::Result<()>;
    fn GetMin(&self, lpulmin: *mut u32) -> windows_core::Result<()>;
    fn SetLimits(&self, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> windows_core::Result<()>;
}
impl IMAPIProgress_Vtbl {
    pub const fn new<Identity: IMAPIProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Progress<Identity: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulvalue: u32, ulcount: u32, ultotal: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProgress_Impl::Progress(this, core::mem::transmute_copy(&ulvalue), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ultotal)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProgress_Impl::GetFlags(this, core::mem::transmute_copy(&lpulflags)).into()
            }
        }
        unsafe extern "system" fn GetMax<Identity: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulmax: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProgress_Impl::GetMax(this, core::mem::transmute_copy(&lpulmax)).into()
            }
        }
        unsafe extern "system" fn GetMin<Identity: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulmin: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProgress_Impl::GetMin(this, core::mem::transmute_copy(&lpulmin)).into()
            }
        }
        unsafe extern "system" fn SetLimits<Identity: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProgress_Impl::SetLimits(this, core::mem::transmute_copy(&lpulmin), core::mem::transmute_copy(&lpulmax), core::mem::transmute_copy(&lpulflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Progress: Progress::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetMax: GetMax::<Identity, OFFSET>,
            GetMin: GetMin::<Identity, OFFSET>,
            SetLimits: SetLimits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIProgress as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMAPIProgress {}
windows_core::imp::define_interface!(IMAPIProp, IMAPIProp_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IMAPIProp, windows_core::IUnknown);
impl IMAPIProp {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, lppmapierror as _).ok() }
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self), ulflags).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProps)(windows_core::Interface::as_raw(self), lpproptagarray as _, ulflags, lpcvalues as _, lppproparray as _).ok() }
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropList)(windows_core::Interface::as_raw(self), ulflags, lppproptagarray as _).ok() }
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut windows_core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenProperty)(windows_core::Interface::as_raw(self), ulproptag, lpiid as _, ulinterfaceoptions, ulflags, core::mem::transmute(lppunk)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProps)(windows_core::Interface::as_raw(self), cvalues, lpproparray as _, lppproblems as _).ok() }
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteProps)(windows_core::Interface::as_raw(self), lpproptagarray as _, lppproblems as _).ok() }
    }
    pub unsafe fn CopyTo<P4>(&self, ciidexclude: u32, rgiidexclude: *mut windows_core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P4, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), ciidexclude, rgiidexclude as _, lpexcludeprops as _, uluiparam, lpprogress.param().abi(), lpinterface as _, lpdestobj as _, ulflags, lppproblems as _).ok() }
    }
    pub unsafe fn CopyProps<P2>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P2, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyProps)(windows_core::Interface::as_raw(self), lpincludeprops as _, uluiparam, lpprogress.param().abi(), lpinterface as _, lpdestobj as _, ulflags, lppproblems as _).ok() }
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut windows_core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNamesFromIDs)(windows_core::Interface::as_raw(self), lppproptags as _, lppropsetguid as _, ulflags, lpcpropnames as _, lppppropnames as _).ok() }
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIDsFromNames)(windows_core::Interface::as_raw(self), cpropnames, lpppropnames as _, ulflags, lppproptags as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIProp_Impl: windows_core::IUnknownImpl {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()>;
    fn SaveChanges(&self, ulflags: u32) -> windows_core::Result<()>;
    fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> windows_core::Result<()>;
    fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> windows_core::Result<()>;
    fn OpenProperty(&self, ulproptag: u32, lpiid: *mut windows_core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
    fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
    fn CopyTo(&self, ciidexclude: u32, rgiidexclude: *mut windows_core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
    fn CopyProps(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
    fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut windows_core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> windows_core::Result<()>;
    fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIProp_Vtbl {
    pub const fn new<Identity: IMAPIProp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLastError<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppmapierror)).into()
            }
        }
        unsafe extern "system" fn SaveChanges<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::SaveChanges(this, core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn GetProps<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::GetProps(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcvalues), core::mem::transmute_copy(&lppproparray)).into()
            }
        }
        unsafe extern "system" fn GetPropList<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::GetPropList(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppproptagarray)).into()
            }
        }
        unsafe extern "system" fn OpenProperty<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulproptag: u32, lpiid: *mut windows_core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::OpenProperty(this, core::mem::transmute_copy(&ulproptag), core::mem::transmute_copy(&lpiid), core::mem::transmute_copy(&ulinterfaceoptions), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppunk)).into()
            }
        }
        unsafe extern "system" fn SetProps<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::SetProps(this, core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&lpproparray), core::mem::transmute_copy(&lppproblems)).into()
            }
        }
        unsafe extern "system" fn DeleteProps<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::DeleteProps(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&lppproblems)).into()
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ciidexclude: u32, rgiidexclude: *mut windows_core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::CopyTo(this, core::mem::transmute_copy(&ciidexclude), core::mem::transmute_copy(&rgiidexclude), core::mem::transmute_copy(&lpexcludeprops), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&lpdestobj), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppproblems)).into()
            }
        }
        unsafe extern "system" fn CopyProps<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::CopyProps(this, core::mem::transmute_copy(&lpincludeprops), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&lpdestobj), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppproblems)).into()
            }
        }
        unsafe extern "system" fn GetNamesFromIDs<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut windows_core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::GetNamesFromIDs(this, core::mem::transmute_copy(&lppproptags), core::mem::transmute_copy(&lppropsetguid), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcpropnames), core::mem::transmute_copy(&lppppropnames)).into()
            }
        }
        unsafe extern "system" fn GetIDsFromNames<Identity: IMAPIProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIProp_Impl::GetIDsFromNames(this, core::mem::transmute_copy(&cpropnames), core::mem::transmute_copy(&lpppropnames), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppproptags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            GetProps: GetProps::<Identity, OFFSET>,
            GetPropList: GetPropList::<Identity, OFFSET>,
            OpenProperty: OpenProperty::<Identity, OFFSET>,
            SetProps: SetProps::<Identity, OFFSET>,
            DeleteProps: DeleteProps::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            CopyProps: CopyProps::<Identity, OFFSET>,
            GetNamesFromIDs: GetNamesFromIDs::<Identity, OFFSET>,
            GetIDsFromNames: GetIDsFromNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIProp {}
windows_core::imp::define_interface!(IMAPIStatus, IMAPIStatus_Vtbl, 0);
impl core::ops::Deref for IMAPIStatus {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMAPIStatus, windows_core::IUnknown, IMAPIProp);
impl IMAPIStatus {
    pub unsafe fn ValidateState(&self, uluiparam: Option<usize>, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ValidateState)(windows_core::Interface::as_raw(self), uluiparam.unwrap_or(core::mem::zeroed()) as _, ulflags).ok() }
    }
    pub unsafe fn SettingsDialog(&self, uluiparam: Option<usize>, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SettingsDialog)(windows_core::Interface::as_raw(self), uluiparam.unwrap_or(core::mem::zeroed()) as _, ulflags).ok() }
    }
    pub unsafe fn ChangePassword(&self, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ChangePassword)(windows_core::Interface::as_raw(self), lpoldpass, lpnewpass, ulflags).ok() }
    }
    pub unsafe fn FlushQueues(&self, uluiparam: Option<usize>, lptargettransport: Option<&[ENTRYID]>, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FlushQueues)(windows_core::Interface::as_raw(self), uluiparam.unwrap_or(core::mem::zeroed()) as _, lptargettransport.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lptargettransport.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ulflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIStatus_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub ValidateState: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub SettingsDialog: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, *const i8, u32) -> windows_core::HRESULT,
    pub FlushQueues: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const ENTRYID, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIStatus_Impl: IMAPIProp_Impl {
    fn ValidateState(&self, uluiparam: usize, ulflags: u32) -> windows_core::Result<()>;
    fn SettingsDialog(&self, uluiparam: usize, ulflags: u32) -> windows_core::Result<()>;
    fn ChangePassword(&self, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> windows_core::Result<()>;
    fn FlushQueues(&self, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIStatus_Vtbl {
    pub const fn new<Identity: IMAPIStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ValidateState<Identity: IMAPIStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIStatus_Impl::ValidateState(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn SettingsDialog<Identity: IMAPIStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIStatus_Impl::SettingsDialog(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn ChangePassword<Identity: IMAPIStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIStatus_Impl::ChangePassword(this, core::mem::transmute_copy(&lpoldpass), core::mem::transmute_copy(&lpnewpass), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn FlushQueues<Identity: IMAPIStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPIStatus_Impl::FlushQueues(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&cbtargettransport), core::mem::transmute_copy(&lptargettransport), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            ValidateState: ValidateState::<Identity, OFFSET>,
            SettingsDialog: SettingsDialog::<Identity, OFFSET>,
            ChangePassword: ChangePassword::<Identity, OFFSET>,
            FlushQueues: FlushQueues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIStatus as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIStatus {}
windows_core::imp::define_interface!(IMAPITable, IMAPITable_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IMAPITable, windows_core::IUnknown);
impl IMAPITable {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, lppmapierror as _).ok() }
    }
    pub unsafe fn Advise<P1>(&self, uleventmask: u32, lpadvisesink: P1, lpulconnection: *mut u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IMAPIAdviseSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), uleventmask, lpadvisesink.param().abi(), lpulconnection as _).ok() }
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), ulconnection).ok() }
    }
    pub unsafe fn GetStatus(&self, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), lpultablestatus as _, lpultabletype as _).ok() }
    }
    pub unsafe fn SetColumns(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColumns)(windows_core::Interface::as_raw(self), lpproptagarray as _, ulflags).ok() }
    }
    pub unsafe fn QueryColumns(&self, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryColumns)(windows_core::Interface::as_raw(self), ulflags, lpproptagarray as _).ok() }
    }
    pub unsafe fn GetRowCount(&self, ulflags: u32, lpulcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRowCount)(windows_core::Interface::as_raw(self), ulflags, lpulcount as _).ok() }
    }
    pub unsafe fn SeekRow(&self, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SeekRow)(windows_core::Interface::as_raw(self), bkorigin, lrowcount, lplrowssought as _).ok() }
    }
    pub unsafe fn SeekRowApprox(&self, ulnumerator: u32, uldenominator: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SeekRowApprox)(windows_core::Interface::as_raw(self), ulnumerator, uldenominator).ok() }
    }
    pub unsafe fn QueryPosition(&self, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryPosition)(windows_core::Interface::as_raw(self), lpulrow as _, lpulnumerator as _, lpuldenominator as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindRow(&self, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FindRow)(windows_core::Interface::as_raw(self), lprestriction as _, bkorigin, ulflags).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Restrict(&self, lprestriction: *mut SRestriction, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Restrict)(windows_core::Interface::as_raw(self), lprestriction as _, ulflags).ok() }
    }
    pub unsafe fn CreateBookmark(&self, lpbkposition: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateBookmark)(windows_core::Interface::as_raw(self), lpbkposition as _).ok() }
    }
    pub unsafe fn FreeBookmark(&self, bkposition: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeBookmark)(windows_core::Interface::as_raw(self), bkposition).ok() }
    }
    pub unsafe fn SortTable(&self, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SortTable)(windows_core::Interface::as_raw(self), lpsortcriteria as _, ulflags).ok() }
    }
    pub unsafe fn QuerySortOrder(&self, lppsortcriteria: *mut *mut SSortOrderSet) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QuerySortOrder)(windows_core::Interface::as_raw(self), lppsortcriteria as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryRows(&self, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryRows)(windows_core::Interface::as_raw(self), lrowcount, ulflags, lpprows as _).ok() }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExpandRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExpandRow)(windows_core::Interface::as_raw(self), cbinstancekey, pbinstancekey as _, ulrowcount, ulflags, lpprows as _, lpulmorerows as _).ok() }
    }
    pub unsafe fn CollapseRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CollapseRow)(windows_core::Interface::as_raw(self), cbinstancekey, pbinstancekey as _, ulflags, lpulrowcount as _).ok() }
    }
    pub unsafe fn WaitForCompletion(&self, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), ulflags, ultimeout, lpultablestatus as _).ok() }
    }
    pub unsafe fn GetCollapseState(&self, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCollapseState)(windows_core::Interface::as_raw(self), ulflags, cbinstancekey, lpbinstancekey as _, lpcbcollapsestate as _, lppbcollapsestate as _).ok() }
    }
    pub unsafe fn SetCollapseState(&self, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCollapseState)(windows_core::Interface::as_raw(self), ulflags, cbcollapsestate, pbcollapsestate as _, lpbklocation as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPITable_Impl: windows_core::IUnknownImpl {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()>;
    fn Advise(&self, uleventmask: u32, lpadvisesink: windows_core::Ref<IMAPIAdviseSink>, lpulconnection: *mut u32) -> windows_core::Result<()>;
    fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()>;
    fn GetStatus(&self, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> windows_core::Result<()>;
    fn SetColumns(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> windows_core::Result<()>;
    fn QueryColumns(&self, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> windows_core::Result<()>;
    fn GetRowCount(&self, ulflags: u32, lpulcount: *mut u32) -> windows_core::Result<()>;
    fn SeekRow(&self, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> windows_core::Result<()>;
    fn SeekRowApprox(&self, ulnumerator: u32, uldenominator: u32) -> windows_core::Result<()>;
    fn QueryPosition(&self, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> windows_core::Result<()>;
    fn FindRow(&self, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> windows_core::Result<()>;
    fn Restrict(&self, lprestriction: *mut SRestriction, ulflags: u32) -> windows_core::Result<()>;
    fn CreateBookmark(&self, lpbkposition: *mut u32) -> windows_core::Result<()>;
    fn FreeBookmark(&self, bkposition: u32) -> windows_core::Result<()>;
    fn SortTable(&self, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> windows_core::Result<()>;
    fn QuerySortOrder(&self, lppsortcriteria: *mut *mut SSortOrderSet) -> windows_core::Result<()>;
    fn QueryRows(&self, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn ExpandRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> windows_core::Result<()>;
    fn CollapseRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> windows_core::Result<()>;
    fn GetCollapseState(&self, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> windows_core::Result<()>;
    fn SetCollapseState(&self, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMAPITable_Vtbl {
    pub const fn new<Identity: IMAPITable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLastError<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppmapierror)).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uleventmask: u32, lpadvisesink: *mut core::ffi::c_void, lpulconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::Advise(this, core::mem::transmute_copy(&uleventmask), core::mem::transmute_copy(&lpadvisesink), core::mem::transmute_copy(&lpulconnection)).into()
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::Unadvise(this, core::mem::transmute_copy(&ulconnection)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::GetStatus(this, core::mem::transmute_copy(&lpultablestatus), core::mem::transmute_copy(&lpultabletype)).into()
            }
        }
        unsafe extern "system" fn SetColumns<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::SetColumns(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn QueryColumns<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::QueryColumns(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpproptagarray)).into()
            }
        }
        unsafe extern "system" fn GetRowCount<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpulcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::GetRowCount(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulcount)).into()
            }
        }
        unsafe extern "system" fn SeekRow<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::SeekRow(this, core::mem::transmute_copy(&bkorigin), core::mem::transmute_copy(&lrowcount), core::mem::transmute_copy(&lplrowssought)).into()
            }
        }
        unsafe extern "system" fn SeekRowApprox<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulnumerator: u32, uldenominator: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::SeekRowApprox(this, core::mem::transmute_copy(&ulnumerator), core::mem::transmute_copy(&uldenominator)).into()
            }
        }
        unsafe extern "system" fn QueryPosition<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::QueryPosition(this, core::mem::transmute_copy(&lpulrow), core::mem::transmute_copy(&lpulnumerator), core::mem::transmute_copy(&lpuldenominator)).into()
            }
        }
        unsafe extern "system" fn FindRow<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::FindRow(this, core::mem::transmute_copy(&lprestriction), core::mem::transmute_copy(&bkorigin), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn Restrict<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprestriction: *mut SRestriction, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::Restrict(this, core::mem::transmute_copy(&lprestriction), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn CreateBookmark<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbkposition: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::CreateBookmark(this, core::mem::transmute_copy(&lpbkposition)).into()
            }
        }
        unsafe extern "system" fn FreeBookmark<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bkposition: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::FreeBookmark(this, core::mem::transmute_copy(&bkposition)).into()
            }
        }
        unsafe extern "system" fn SortTable<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::SortTable(this, core::mem::transmute_copy(&lpsortcriteria), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn QuerySortOrder<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppsortcriteria: *mut *mut SSortOrderSet) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::QuerySortOrder(this, core::mem::transmute_copy(&lppsortcriteria)).into()
            }
        }
        unsafe extern "system" fn QueryRows<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::QueryRows(this, core::mem::transmute_copy(&lrowcount), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpprows)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn ExpandRow<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::ExpandRow(this, core::mem::transmute_copy(&cbinstancekey), core::mem::transmute_copy(&pbinstancekey), core::mem::transmute_copy(&ulrowcount), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpprows), core::mem::transmute_copy(&lpulmorerows)).into()
            }
        }
        unsafe extern "system" fn CollapseRow<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::CollapseRow(this, core::mem::transmute_copy(&cbinstancekey), core::mem::transmute_copy(&pbinstancekey), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulrowcount)).into()
            }
        }
        unsafe extern "system" fn WaitForCompletion<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::WaitForCompletion(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ultimeout), core::mem::transmute_copy(&lpultablestatus)).into()
            }
        }
        unsafe extern "system" fn GetCollapseState<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::GetCollapseState(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbinstancekey), core::mem::transmute_copy(&lpbinstancekey), core::mem::transmute_copy(&lpcbcollapsestate), core::mem::transmute_copy(&lppbcollapsestate)).into()
            }
        }
        unsafe extern "system" fn SetCollapseState<Identity: IMAPITable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMAPITable_Impl::SetCollapseState(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbcollapsestate), core::mem::transmute_copy(&pbcollapsestate), core::mem::transmute_copy(&lpbklocation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetColumns: SetColumns::<Identity, OFFSET>,
            QueryColumns: QueryColumns::<Identity, OFFSET>,
            GetRowCount: GetRowCount::<Identity, OFFSET>,
            SeekRow: SeekRow::<Identity, OFFSET>,
            SeekRowApprox: SeekRowApprox::<Identity, OFFSET>,
            QueryPosition: QueryPosition::<Identity, OFFSET>,
            FindRow: FindRow::<Identity, OFFSET>,
            Restrict: Restrict::<Identity, OFFSET>,
            CreateBookmark: CreateBookmark::<Identity, OFFSET>,
            FreeBookmark: FreeBookmark::<Identity, OFFSET>,
            SortTable: SortTable::<Identity, OFFSET>,
            QuerySortOrder: QuerySortOrder::<Identity, OFFSET>,
            QueryRows: QueryRows::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            ExpandRow: ExpandRow::<Identity, OFFSET>,
            CollapseRow: CollapseRow::<Identity, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, OFFSET>,
            GetCollapseState: GetCollapseState::<Identity, OFFSET>,
            SetCollapseState: SetCollapseState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPITable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPITable {}
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
windows_core::imp::define_interface!(IMailUser, IMailUser_Vtbl, 0);
impl core::ops::Deref for IMailUser {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMailUser, windows_core::IUnknown, IMAPIProp);
#[repr(C)]
#[doc(hidden)]
pub struct IMailUser_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMailUser_Impl: IMAPIProp_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl IMailUser_Vtbl {
    pub const fn new<Identity: IMailUser_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMailUser as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMailUser {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttachmentTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenAttach(&self, ulattachmentnum: u32, lpinterface: Option<*const windows_core::GUID>, ulflags: u32) -> windows_core::Result<IAttach> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenAttach)(windows_core::Interface::as_raw(self), ulattachmentnum, lpinterface.unwrap_or(core::mem::zeroed()) as _, ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAttach(&self, lpinterface: Option<*const windows_core::GUID>, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut Option<IAttach>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateAttach)(windows_core::Interface::as_raw(self), lpinterface.unwrap_or(core::mem::zeroed()) as _, ulflags, lpulattachmentnum as _, core::mem::transmute(lppattach)).ok() }
    }
    pub unsafe fn DeleteAttach<P2>(&self, ulattachmentnum: u32, uluiparam: Option<usize>, lpprogress: P2, ulflags: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IMAPIProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttach)(windows_core::Interface::as_raw(self), ulattachmentnum, uluiparam.unwrap_or(core::mem::zeroed()) as _, lpprogress.param().abi(), ulflags).ok() }
    }
    pub unsafe fn GetRecipientTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecipientTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModifyRecipients(&self, ulflags: u32, lpmods: *const ADRLIST) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModifyRecipients)(windows_core::Interface::as_raw(self), ulflags, lpmods).ok() }
    }
    pub unsafe fn SubmitMessage(&self, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SubmitMessage)(windows_core::Interface::as_raw(self), ulflags).ok() }
    }
    pub unsafe fn SetReadFlag(&self, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReadFlag)(windows_core::Interface::as_raw(self), ulflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IMessage_Impl: IMAPIProp_Impl {
    fn GetAttachmentTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn OpenAttach(&self, ulattachmentnum: u32, lpinterface: *const windows_core::GUID, ulflags: u32) -> windows_core::Result<IAttach>;
    fn CreateAttach(&self, lpinterface: *const windows_core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: windows_core::OutRef<IAttach>) -> windows_core::Result<()>;
    fn DeleteAttach(&self, ulattachmentnum: u32, uluiparam: usize, lpprogress: windows_core::Ref<IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn GetRecipientTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn ModifyRecipients(&self, ulflags: u32, lpmods: *const ADRLIST) -> windows_core::Result<()>;
    fn SubmitMessage(&self, ulflags: u32) -> windows_core::Result<()>;
    fn SetReadFlag(&self, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMessage_Vtbl {
    pub const fn new<Identity: IMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAttachmentTable<Identity: IMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMessage_Impl::GetAttachmentTable(this, core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenAttach<Identity: IMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulattachmentnum: u32, lpinterface: *const windows_core::GUID, ulflags: u32, lppattach: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMessage_Impl::OpenAttach(this, core::mem::transmute_copy(&ulattachmentnum), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lppattach.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAttach<Identity: IMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpinterface: *const windows_core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessage_Impl::CreateAttach(this, core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulattachmentnum), core::mem::transmute_copy(&lppattach)).into()
            }
        }
        unsafe extern "system" fn DeleteAttach<Identity: IMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulattachmentnum: u32, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessage_Impl::DeleteAttach(this, core::mem::transmute_copy(&ulattachmentnum), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn GetRecipientTable<Identity: IMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMessage_Impl::GetRecipientTable(this, core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifyRecipients<Identity: IMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpmods: *const ADRLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessage_Impl::ModifyRecipients(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpmods)).into()
            }
        }
        unsafe extern "system" fn SubmitMessage<Identity: IMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessage_Impl::SubmitMessage(this, core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn SetReadFlag<Identity: IMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessage_Impl::SetReadFlag(this, core::mem::transmute_copy(&ulflags)).into()
            }
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            GetAttachmentTable: GetAttachmentTable::<Identity, OFFSET>,
            OpenAttach: OpenAttach::<Identity, OFFSET>,
            CreateAttach: CreateAttach::<Identity, OFFSET>,
            DeleteAttach: DeleteAttach::<Identity, OFFSET>,
            GetRecipientTable: GetRecipientTable::<Identity, OFFSET>,
            ModifyRecipients: ModifyRecipients::<Identity, OFFSET>,
            SubmitMessage: SubmitMessage::<Identity, OFFSET>,
            SetReadFlag: SetReadFlag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessage as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMessage {}
windows_core::imp::define_interface!(IMsgStore, IMsgStore_Vtbl, 0);
impl core::ops::Deref for IMsgStore {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMsgStore, windows_core::IUnknown, IMAPIProp);
impl IMsgStore {
    pub unsafe fn Advise<P3>(&self, cbentryid: u32, lpentryid: Option<*const ENTRYID>, uleventmask: u32, lpadvisesink: P3) -> windows_core::Result<u32>
    where
        P3: windows_core::Param<IMAPIAdviseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), cbentryid, lpentryid.unwrap_or(core::mem::zeroed()) as _, uleventmask, lpadvisesink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), ulconnection).ok() }
    }
    pub unsafe fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareEntryIDs)(windows_core::Interface::as_raw(self), cbentryid1, lpentryid1, cbentryid2, lpentryid2, ulflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: Option<*const windows_core::GUID>, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenEntry)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, lpinterface.unwrap_or(core::mem::zeroed()) as _, ulflags, lpulobjtype as _, core::mem::transmute(ppunk)).ok() }
    }
    pub unsafe fn SetReceiveFolder(&self, lpszmessageclass: Option<*const i8>, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReceiveFolder)(windows_core::Interface::as_raw(self), lpszmessageclass.unwrap_or(core::mem::zeroed()) as _, ulflags, cbentryid, lpentryid).ok() }
    }
    pub unsafe fn GetReceiveFolder(&self, lpszmessageclass: Option<*const i8>, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetReceiveFolder)(windows_core::Interface::as_raw(self), lpszmessageclass.unwrap_or(core::mem::zeroed()) as _, ulflags, lpcbentryid as _, lppentryid as _, lppszexplicitclass as _).ok() }
    }
    pub unsafe fn GetReceiveFolderTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReceiveFolderTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn StoreLogoff(&self, lpulflags: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StoreLogoff)(windows_core::Interface::as_raw(self), lpulflags as _).ok() }
    }
    pub unsafe fn AbortSubmit(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AbortSubmit)(windows_core::Interface::as_raw(self), cbentryid, lpentryid, ulflags).ok() }
    }
    pub unsafe fn GetOutgoingQueue(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutgoingQueue)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetLockState<P0>(&self, lpmessage: P0, ullockstate: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMessage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLockState)(windows_core::Interface::as_raw(self), lpmessage.param().abi(), ullockstate).ok() }
    }
    pub unsafe fn FinishedMsg(&self, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FinishedMsg)(windows_core::Interface::as_raw(self), ulflags, cbentryid, lpentryid).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NotifyNewMail(&self, lpnotification: *const NOTIFICATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyNewMail)(windows_core::Interface::as_raw(self), lpnotification).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IMsgStore_Impl: IMAPIProp_Impl {
    fn Advise(&self, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: windows_core::Ref<IMAPIAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()>;
    fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32) -> windows_core::Result<u32>;
    fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetReceiveFolder(&self, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::Result<()>;
    fn GetReceiveFolder(&self, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> windows_core::Result<()>;
    fn GetReceiveFolderTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn StoreLogoff(&self, lpulflags: *mut u32) -> windows_core::Result<()>;
    fn AbortSubmit(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::Result<()>;
    fn GetOutgoingQueue(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn SetLockState(&self, lpmessage: windows_core::Ref<IMessage>, ullockstate: u32) -> windows_core::Result<()>;
    fn FinishedMsg(&self, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::Result<()>;
    fn NotifyNewMail(&self, lpnotification: *const NOTIFICATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMsgStore_Vtbl {
    pub const fn new<Identity: IMsgStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Advise<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: *mut core::ffi::c_void, lpulconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsgStore_Impl::Advise(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&uleventmask), core::mem::transmute_copy(&lpadvisesink)) {
                    Ok(ok__) => {
                        lpulconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::Unadvise(this, core::mem::transmute_copy(&ulconnection)).into()
            }
        }
        unsafe extern "system" fn CompareEntryIDs<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32, lpulresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsgStore_Impl::CompareEntryIDs(this, core::mem::transmute_copy(&cbentryid1), core::mem::transmute_copy(&lpentryid1), core::mem::transmute_copy(&cbentryid2), core::mem::transmute_copy(&lpentryid2), core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpulresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenEntry<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::OpenEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulobjtype), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn SetReceiveFolder<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::SetReceiveFolder(this, core::mem::transmute_copy(&lpszmessageclass), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid)).into()
            }
        }
        unsafe extern "system" fn GetReceiveFolder<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::GetReceiveFolder(this, core::mem::transmute_copy(&lpszmessageclass), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcbentryid), core::mem::transmute_copy(&lppentryid), core::mem::transmute_copy(&lppszexplicitclass)).into()
            }
        }
        unsafe extern "system" fn GetReceiveFolderTable<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsgStore_Impl::GetReceiveFolderTable(this, core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StoreLogoff<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::StoreLogoff(this, core::mem::transmute_copy(&lpulflags)).into()
            }
        }
        unsafe extern "system" fn AbortSubmit<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::AbortSubmit(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn GetOutgoingQueue<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsgStore_Impl::GetOutgoingQueue(this, core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLockState<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmessage: *mut core::ffi::c_void, ullockstate: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::SetLockState(this, core::mem::transmute_copy(&lpmessage), core::mem::transmute_copy(&ullockstate)).into()
            }
        }
        unsafe extern "system" fn FinishedMsg<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::FinishedMsg(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid)).into()
            }
        }
        unsafe extern "system" fn NotifyNewMail<Identity: IMsgStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnotification: *const NOTIFICATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsgStore_Impl::NotifyNewMail(this, core::mem::transmute_copy(&lpnotification)).into()
            }
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            CompareEntryIDs: CompareEntryIDs::<Identity, OFFSET>,
            OpenEntry: OpenEntry::<Identity, OFFSET>,
            SetReceiveFolder: SetReceiveFolder::<Identity, OFFSET>,
            GetReceiveFolder: GetReceiveFolder::<Identity, OFFSET>,
            GetReceiveFolderTable: GetReceiveFolderTable::<Identity, OFFSET>,
            StoreLogoff: StoreLogoff::<Identity, OFFSET>,
            AbortSubmit: AbortSubmit::<Identity, OFFSET>,
            GetOutgoingQueue: GetOutgoingQueue::<Identity, OFFSET>,
            SetLockState: SetLockState::<Identity, OFFSET>,
            FinishedMsg: FinishedMsg::<Identity, OFFSET>,
            NotifyNewMail: NotifyNewMail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsgStore as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsgStore {}
windows_core::imp::define_interface!(IProfSect, IProfSect_Vtbl, 0);
impl core::ops::Deref for IProfSect {
    type Target = IMAPIProp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProfSect, windows_core::IUnknown, IMAPIProp);
#[repr(C)]
#[doc(hidden)]
pub struct IProfSect_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProfSect_Impl: IMAPIProp_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl IProfSect_Vtbl {
    pub const fn new<Identity: IProfSect_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProfSect as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProfSect {}
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
        unsafe { (windows_core::Interface::vtable(self).HrSetObjAccess)(windows_core::Interface::as_raw(self), ulaccess).ok() }
    }
    pub unsafe fn HrSetPropAccess(&self, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrSetPropAccess)(windows_core::Interface::as_raw(self), lpproptagarray as _, rgulaccess as _).ok() }
    }
    pub unsafe fn HrGetPropAccess(&self, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrGetPropAccess)(windows_core::Interface::as_raw(self), lppproptagarray as _, lprgulaccess as _).ok() }
    }
    pub unsafe fn HrAddObjProps(&self, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrAddObjProps)(windows_core::Interface::as_raw(self), lppproptagarray as _, lprgulaccess as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropData_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub HrSetObjAccess: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HrSetPropAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropTagArray, *mut u32) -> windows_core::HRESULT,
    pub HrGetPropAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SPropTagArray, *mut *mut u32) -> windows_core::HRESULT,
    pub HrAddObjProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPropTagArray, *mut *mut SPropProblemArray) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPropData_Impl: IMAPIProp_Impl {
    fn HrSetObjAccess(&self, ulaccess: u32) -> windows_core::Result<()>;
    fn HrSetPropAccess(&self, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> windows_core::Result<()>;
    fn HrGetPropAccess(&self, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> windows_core::Result<()>;
    fn HrAddObjProps(&self, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPropData_Vtbl {
    pub const fn new<Identity: IPropData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HrSetObjAccess<Identity: IPropData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulaccess: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropData_Impl::HrSetObjAccess(this, core::mem::transmute_copy(&ulaccess)).into()
            }
        }
        unsafe extern "system" fn HrSetPropAccess<Identity: IPropData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropData_Impl::HrSetPropAccess(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&rgulaccess)).into()
            }
        }
        unsafe extern "system" fn HrGetPropAccess<Identity: IPropData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropData_Impl::HrGetPropAccess(this, core::mem::transmute_copy(&lppproptagarray), core::mem::transmute_copy(&lprgulaccess)).into()
            }
        }
        unsafe extern "system" fn HrAddObjProps<Identity: IPropData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropData_Impl::HrAddObjProps(this, core::mem::transmute_copy(&lppproptagarray), core::mem::transmute_copy(&lprgulaccess)).into()
            }
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            HrSetObjAccess: HrSetObjAccess::<Identity, OFFSET>,
            HrSetPropAccess: HrSetPropAccess::<Identity, OFFSET>,
            HrGetPropAccess: HrGetPropAccess::<Identity, OFFSET>,
            HrAddObjProps: HrAddObjProps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropData as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPropData {}
windows_core::imp::define_interface!(IProviderAdmin, IProviderAdmin_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IProviderAdmin, windows_core::IUnknown);
impl IProviderAdmin {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32) -> windows_core::Result<*mut MAPIERROR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProviderTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderTable)(windows_core::Interface::as_raw(self), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateProvider(&self, lpszprovider: *const i8, lpprops: &[SPropValue], uluiparam: Option<usize>, ulflags: u32) -> windows_core::Result<MAPIUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProvider)(windows_core::Interface::as_raw(self), lpszprovider, lpprops.len().try_into().unwrap(), core::mem::transmute(lpprops.as_ptr()), uluiparam.unwrap_or(core::mem::zeroed()) as _, ulflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeleteProvider(&self, lpuid: *const MAPIUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteProvider)(windows_core::Interface::as_raw(self), lpuid).ok() }
    }
    pub unsafe fn OpenProfileSection(&self, lpuid: Option<*const MAPIUID>, lpinterface: Option<*const windows_core::GUID>, ulflags: u32) -> windows_core::Result<IProfSect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenProfileSection)(windows_core::Interface::as_raw(self), lpuid.unwrap_or(core::mem::zeroed()) as _, lpinterface.unwrap_or(core::mem::zeroed()) as _, ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IProviderAdmin_Impl: windows_core::IUnknownImpl {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32) -> windows_core::Result<*mut MAPIERROR>;
    fn GetProviderTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn CreateProvider(&self, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32) -> windows_core::Result<MAPIUID>;
    fn DeleteProvider(&self, lpuid: *const MAPIUID) -> windows_core::Result<()>;
    fn OpenProfileSection(&self, lpuid: *const MAPIUID, lpinterface: *const windows_core::GUID, ulflags: u32) -> windows_core::Result<IProfSect>;
}
#[cfg(feature = "Win32_System_Com")]
impl IProviderAdmin_Vtbl {
    pub const fn new<Identity: IProviderAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLastError<Identity: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProviderAdmin_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lppmapierror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderTable<Identity: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProviderAdmin_Impl::GetProviderTable(this, core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateProvider<Identity: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32, lpuid: *mut MAPIUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProviderAdmin_Impl::CreateProvider(this, core::mem::transmute_copy(&lpszprovider), core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&lpprops), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lpuid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteProvider<Identity: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpuid: *const MAPIUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProviderAdmin_Impl::DeleteProvider(this, core::mem::transmute_copy(&lpuid)).into()
            }
        }
        unsafe extern "system" fn OpenProfileSection<Identity: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpuid: *const MAPIUID, lpinterface: *const windows_core::GUID, ulflags: u32, lppprofsect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProviderAdmin_Impl::OpenProfileSection(this, core::mem::transmute_copy(&lpuid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags)) {
                    Ok(ok__) => {
                        lppprofsect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            GetProviderTable: GetProviderTable::<Identity, OFFSET>,
            CreateProvider: CreateProvider::<Identity, OFFSET>,
            DeleteProvider: DeleteProvider::<Identity, OFFSET>,
            OpenProfileSection: OpenProfileSection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProviderAdmin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProviderAdmin {}
windows_core::imp::define_interface!(ITableData, ITableData_Vtbl, 0);
windows_core::imp::interface_hierarchy!(ITableData, windows_core::IUnknown);
impl ITableData {
    pub unsafe fn HrGetView(&self, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut Option<IMAPITable>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrGetView)(windows_core::Interface::as_raw(self), lpssortorderset as _, lpfcallerrelease as _, ulcallerdata, core::mem::transmute(lppmapitable)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrModifyRow(&self, param0: *mut SRow) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrModifyRow)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrDeleteRow(&self, lpspropvalue: *mut SPropValue) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrDeleteRow)(windows_core::Interface::as_raw(self), lpspropvalue as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrQueryRow(&self, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrQueryRow)(windows_core::Interface::as_raw(self), lpspropvalue as _, lppsrow as _, lpulirow as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrEnumRow(&self, ulrownumber: u32, lppsrow: *mut *mut SRow) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrEnumRow)(windows_core::Interface::as_raw(self), ulrownumber, lppsrow as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrNotify(&self, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrNotify)(windows_core::Interface::as_raw(self), ulflags, cvalues, lpspropvalue as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrInsertRow(&self, ulirow: u32, lpsrow: *mut SRow) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrInsertRow)(windows_core::Interface::as_raw(self), ulirow, lpsrow as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrModifyRows(&self, ulflags: u32, lpsrowset: *mut SRowSet) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrModifyRows)(windows_core::Interface::as_raw(self), ulflags, lpsrowset as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HrDeleteRows(&self, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HrDeleteRows)(windows_core::Interface::as_raw(self), ulflags, lprowsettodelete as _, crowsdeleted as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait ITableData_Impl: windows_core::IUnknownImpl {
    fn HrGetView(&self, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: windows_core::OutRef<IMAPITable>) -> windows_core::Result<()>;
    fn HrModifyRow(&self, param0: *mut SRow) -> windows_core::Result<()>;
    fn HrDeleteRow(&self, lpspropvalue: *mut SPropValue) -> windows_core::Result<()>;
    fn HrQueryRow(&self, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> windows_core::Result<()>;
    fn HrEnumRow(&self, ulrownumber: u32, lppsrow: *mut *mut SRow) -> windows_core::Result<()>;
    fn HrNotify(&self, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> windows_core::Result<()>;
    fn HrInsertRow(&self, ulirow: u32, lpsrow: *mut SRow) -> windows_core::Result<()>;
    fn HrModifyRows(&self, ulflags: u32, lpsrowset: *mut SRowSet) -> windows_core::Result<()>;
    fn HrDeleteRows(&self, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITableData_Vtbl {
    pub const fn new<Identity: ITableData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HrGetView<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrGetView(this, core::mem::transmute_copy(&lpssortorderset), core::mem::transmute_copy(&lpfcallerrelease), core::mem::transmute_copy(&ulcallerdata), core::mem::transmute_copy(&lppmapitable)).into()
            }
        }
        unsafe extern "system" fn HrModifyRow<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut SRow) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrModifyRow(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn HrDeleteRow<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpspropvalue: *mut SPropValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrDeleteRow(this, core::mem::transmute_copy(&lpspropvalue)).into()
            }
        }
        unsafe extern "system" fn HrQueryRow<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrQueryRow(this, core::mem::transmute_copy(&lpspropvalue), core::mem::transmute_copy(&lppsrow), core::mem::transmute_copy(&lpulirow)).into()
            }
        }
        unsafe extern "system" fn HrEnumRow<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrownumber: u32, lppsrow: *mut *mut SRow) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrEnumRow(this, core::mem::transmute_copy(&ulrownumber), core::mem::transmute_copy(&lppsrow)).into()
            }
        }
        unsafe extern "system" fn HrNotify<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrNotify(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&lpspropvalue)).into()
            }
        }
        unsafe extern "system" fn HrInsertRow<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulirow: u32, lpsrow: *mut SRow) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrInsertRow(this, core::mem::transmute_copy(&ulirow), core::mem::transmute_copy(&lpsrow)).into()
            }
        }
        unsafe extern "system" fn HrModifyRows<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpsrowset: *mut SRowSet) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrModifyRows(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpsrowset)).into()
            }
        }
        unsafe extern "system" fn HrDeleteRows<Identity: ITableData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableData_Impl::HrDeleteRows(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lprowsettodelete), core::mem::transmute_copy(&crowsdeleted)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HrGetView: HrGetView::<Identity, OFFSET>,
            HrModifyRow: HrModifyRow::<Identity, OFFSET>,
            HrDeleteRow: HrDeleteRow::<Identity, OFFSET>,
            HrQueryRow: HrQueryRow::<Identity, OFFSET>,
            HrEnumRow: HrEnumRow::<Identity, OFFSET>,
            HrNotify: HrNotify::<Identity, OFFSET>,
            HrInsertRow: HrInsertRow::<Identity, OFFSET>,
            HrModifyRows: HrModifyRows::<Identity, OFFSET>,
            HrDeleteRows: HrDeleteRows::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITableData {}
windows_core::imp::define_interface!(IWABExtInit, IWABExtInit_Vtbl, 0xea22ebf0_87a4_11d1_9acf_00a0c91f9c8b);
windows_core::imp::interface_hierarchy!(IWABExtInit, windows_core::IUnknown);
impl IWABExtInit {
    pub unsafe fn Initialize(&self, lpwabextdisplay: *mut WABEXTDISPLAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(lpwabextdisplay)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWABExtInit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WABEXTDISPLAY) -> windows_core::HRESULT,
}
pub trait IWABExtInit_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, lpwabextdisplay: *mut WABEXTDISPLAY) -> windows_core::Result<()>;
}
impl IWABExtInit_Vtbl {
    pub const fn new<Identity: IWABExtInit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWABExtInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpwabextdisplay: *mut WABEXTDISPLAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABExtInit_Impl::Initialize(this, core::mem::transmute_copy(&lpwabextdisplay)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWABExtInit as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWABExtInit {}
windows_core::imp::define_interface!(IWABObject, IWABObject_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IWABObject, windows_core::IUnknown);
impl IWABObject {
    pub unsafe fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), hresult, ulflags, lppmapierror as _).ok() }
    }
    pub unsafe fn AllocateBuffer(&self, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AllocateBuffer)(windows_core::Interface::as_raw(self), cbsize, lppbuffer as _).ok() }
    }
    pub unsafe fn AllocateMore(&self, cbsize: u32, lpobject: *const core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AllocateMore)(windows_core::Interface::as_raw(self), cbsize, lpobject, lppbuffer as _).ok() }
    }
    pub unsafe fn FreeBuffer(&self, lpbuffer: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), lpbuffer).ok() }
    }
    pub unsafe fn Backup<P0>(&self, lpfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Backup)(windows_core::Interface::as_raw(self), lpfilename.param().abi()).ok() }
    }
    pub unsafe fn Import<P0>(&self, lpwip: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), lpwip.param().abi()).ok() }
    }
    pub unsafe fn Find<P0>(&self, lpiab: P0, hwnd: Option<super::super::Foundation::HWND>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
    {
        unsafe { (windows_core::Interface::vtable(self).Find)(windows_core::Interface::as_raw(self), lpiab.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn VCardDisplay<P0, P2>(&self, lpiab: P0, hwnd: Option<super::super::Foundation::HWND>, lpszfilename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).VCardDisplay)(windows_core::Interface::as_raw(self), lpiab.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _, lpszfilename.param().abi()).ok() }
    }
    pub unsafe fn LDAPUrl<P0, P3>(&self, lpiab: P0, hwnd: Option<super::super::Foundation::HWND>, ulflags: u32, lpszurl: P3) -> windows_core::Result<IMailUser>
    where
        P0: windows_core::Param<IAddrBook>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LDAPUrl)(windows_core::Interface::as_raw(self), lpiab.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _, ulflags, lpszurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn VCardCreate<P0, P2, P3>(&self, lpiab: P0, ulflags: u32, lpszvcard: P2, lpmailuser: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
        P2: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<IMailUser>,
    {
        unsafe { (windows_core::Interface::vtable(self).VCardCreate)(windows_core::Interface::as_raw(self), lpiab.param().abi(), ulflags, lpszvcard.param().abi(), lpmailuser.param().abi()).ok() }
    }
    pub unsafe fn VCardRetrieve<P0, P2>(&self, lpiab: P0, ulflags: u32, lpszvcard: P2) -> windows_core::Result<IMailUser>
    where
        P0: windows_core::Param<IAddrBook>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VCardRetrieve)(windows_core::Interface::as_raw(self), lpiab.param().abi(), ulflags, lpszvcard.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMe<P0>(&self, lpiab: P0, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: Option<super::super::Foundation::HWND>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMe)(windows_core::Interface::as_raw(self), lpiab.param().abi(), ulflags, lpdwaction as _, lpsbeid as _, hwnd.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetMe<P0>(&self, lpiab: P0, ulflags: u32, sbeid: SBinary, hwnd: Option<super::super::Foundation::HWND>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAddrBook>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMe)(windows_core::Interface::as_raw(self), lpiab.param().abi(), ulflags, core::mem::transmute(sbeid), hwnd.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IWABObject_Impl: windows_core::IUnknownImpl {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()>;
    fn AllocateBuffer(&self, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateMore(&self, cbsize: u32, lpobject: *const core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeBuffer(&self, lpbuffer: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Backup(&self, lpfilename: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn Import(&self, lpwip: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn Find(&self, lpiab: windows_core::Ref<IAddrBook>, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn VCardDisplay(&self, lpiab: windows_core::Ref<IAddrBook>, hwnd: super::super::Foundation::HWND, lpszfilename: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn LDAPUrl(&self, lpiab: windows_core::Ref<IAddrBook>, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: &windows_core::PCSTR) -> windows_core::Result<IMailUser>;
    fn VCardCreate(&self, lpiab: windows_core::Ref<IAddrBook>, ulflags: u32, lpszvcard: &windows_core::PCSTR, lpmailuser: windows_core::Ref<IMailUser>) -> windows_core::Result<()>;
    fn VCardRetrieve(&self, lpiab: windows_core::Ref<IAddrBook>, ulflags: u32, lpszvcard: &windows_core::PCSTR) -> windows_core::Result<IMailUser>;
    fn GetMe(&self, lpiab: windows_core::Ref<IAddrBook>, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn SetMe(&self, lpiab: windows_core::Ref<IAddrBook>, ulflags: u32, sbeid: &SBinary, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl IWABObject_Vtbl {
    pub const fn new<Identity: IWABObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLastError<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppmapierror)).into()
            }
        }
        unsafe extern "system" fn AllocateBuffer<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::AllocateBuffer(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&lppbuffer)).into()
            }
        }
        unsafe extern "system" fn AllocateMore<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, lpobject: *const core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::AllocateMore(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&lpobject), core::mem::transmute_copy(&lppbuffer)).into()
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::FreeBuffer(this, core::mem::transmute_copy(&lpbuffer)).into()
            }
        }
        unsafe extern "system" fn Backup<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpfilename: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::Backup(this, core::mem::transmute(&lpfilename)).into()
            }
        }
        unsafe extern "system" fn Import<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpwip: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::Import(this, core::mem::transmute(&lpwip)).into()
            }
        }
        unsafe extern "system" fn Find<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::Find(this, core::mem::transmute_copy(&lpiab), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn VCardDisplay<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, lpszfilename: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::VCardDisplay(this, core::mem::transmute_copy(&lpiab), core::mem::transmute_copy(&hwnd), core::mem::transmute(&lpszfilename)).into()
            }
        }
        unsafe extern "system" fn LDAPUrl<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: windows_core::PCSTR, lppmailuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWABObject_Impl::LDAPUrl(this, core::mem::transmute_copy(&lpiab), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&ulflags), core::mem::transmute(&lpszurl)) {
                    Ok(ok__) => {
                        lppmailuser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VCardCreate<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, ulflags: u32, lpszvcard: windows_core::PCSTR, lpmailuser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::VCardCreate(this, core::mem::transmute_copy(&lpiab), core::mem::transmute_copy(&ulflags), core::mem::transmute(&lpszvcard), core::mem::transmute_copy(&lpmailuser)).into()
            }
        }
        unsafe extern "system" fn VCardRetrieve<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, ulflags: u32, lpszvcard: windows_core::PCSTR, lppmailuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWABObject_Impl::VCardRetrieve(this, core::mem::transmute_copy(&lpiab), core::mem::transmute_copy(&ulflags), core::mem::transmute(&lpszvcard)) {
                    Ok(ok__) => {
                        lppmailuser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMe<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::GetMe(this, core::mem::transmute_copy(&lpiab), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpdwaction), core::mem::transmute_copy(&lpsbeid), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn SetMe<Identity: IWABObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWABObject_Impl::SetMe(this, core::mem::transmute_copy(&lpiab), core::mem::transmute_copy(&ulflags), core::mem::transmute(&sbeid), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, OFFSET>,
            AllocateMore: AllocateMore::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
            Backup: Backup::<Identity, OFFSET>,
            Import: Import::<Identity, OFFSET>,
            Find: Find::<Identity, OFFSET>,
            VCardDisplay: VCardDisplay::<Identity, OFFSET>,
            LDAPUrl: LDAPUrl::<Identity, OFFSET>,
            VCardCreate: VCardCreate::<Identity, OFFSET>,
            VCardRetrieve: VCardRetrieve::<Identity, OFFSET>,
            GetMe: GetMe::<Identity, OFFSET>,
            SetMe: SetMe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWABObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWABObject {}
pub type LPALLOCATEBUFFER = Option<unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPALLOCATEMORE = Option<unsafe extern "system" fn(cbsize: u32, lpobject: *mut core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPCREATECONVERSATIONINDEX = Option<unsafe extern "system" fn(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32>;
pub type LPDISPATCHNOTIFICATIONS = Option<unsafe extern "system" fn(ulflags: u32) -> windows_core::HRESULT>;
pub type LPFNABSDI = Option<unsafe extern "system" fn(uluiparam: usize, lpvmsg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPFNBUTTON = Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut core::ffi::c_void, cbentryid: u32, lpselection: *mut ENTRYID, ulflags: u32) -> i32>;
pub type LPFNDISMISS = Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut core::ffi::c_void)>;
pub type LPFREEBUFFER = Option<unsafe extern "system" fn(lpbuffer: *mut core::ffi::c_void) -> u32>;
#[cfg(feature = "Win32_System_Com")]
pub type LPNOTIFCALLBACK = Option<unsafe extern "system" fn(lpvcontext: *mut core::ffi::c_void, cnotification: u32, lpnotifications: *mut NOTIFICATION) -> i32>;
#[cfg(feature = "Win32_System_Com")]
pub type LPOPENSTREAMONFILE = Option<unsafe extern "system" fn(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: windows_core::OutRef<super::Com::IStream>) -> windows_core::HRESULT>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPWABACTIONITEM(pub isize);
pub type LPWABALLOCATEBUFFER = Option<unsafe extern "system" fn(lpwabobject: windows_core::Ref<IWABObject>, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPWABALLOCATEMORE = Option<unsafe extern "system" fn(lpwabobject: windows_core::Ref<IWABObject>, cbsize: u32, lpobject: *mut core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPWABFREEBUFFER = Option<unsafe extern "system" fn(lpwabobject: windows_core::Ref<IWABObject>, lpbuffer: *mut core::ffi::c_void) -> u32>;
pub type LPWABOPEN = Option<unsafe extern "system" fn(lppadrbook: windows_core::OutRef<IAddrBook>, lppwabobject: windows_core::OutRef<IWABObject>, lpwp: *mut WAB_PARAM, reserved2: u32) -> windows_core::HRESULT>;
pub type LPWABOPENEX = Option<unsafe extern "system" fn(lppadrbook: windows_core::OutRef<IAddrBook>, lppwabobject: windows_core::OutRef<IWABObject>, lpwp: *mut WAB_PARAM, reserved: u32, fnallocatebuffer: LPALLOCATEBUFFER, fnallocatemore: LPALLOCATEMORE, fnfreebuffer: LPFREEBUFFER) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAPIERROR {
    pub ulVersion: u32,
    pub lpszError: *mut i8,
    pub lpszComponent: *mut i8,
    pub ulLowLevelError: u32,
    pub ulContext: u32,
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
impl Default for MAPINAMEID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAPIUID {
    pub ab: [u8; 16],
}
impl Default for MAPIUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTSID {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl Default for MTSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MV_FLAG: u32 = 4096u32;
pub const MV_INSTANCE: u32 = 8192u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NEWMAIL_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpszMessageClass: *mut i8,
    pub ulMessageFlags: u32,
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
impl Default for NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NOTIFKEY {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl Default for NOTIFKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPENSTREAMONFILE: windows_core::PCSTR = windows_core::s!("OpenStreamOnFile");
pub type PFNIDLE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub const PRIHIGHEST: u32 = 32767u32;
pub const PRILOWEST: i32 = -32768i32;
pub const PRIUSER: u32 = 0u32;
pub const PROP_ID_INVALID: u32 = 65535u32;
pub const PROP_ID_NULL: u32 = 0u32;
pub const PROP_ID_SECURE_MAX: u32 = 26623u32;
pub const PROP_ID_SECURE_MIN: u32 = 26608u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAndRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAndRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAppTimeArray {
    pub cValues: u32,
    pub lpat: *mut f64,
}
impl Default for SAppTimeArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SBinary {
    pub cb: u32,
    pub lpb: *mut u8,
}
impl Default for SBinary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SBinaryArray {
    pub cValues: u32,
    pub lpbin: *mut SBinary,
}
impl Default for SBinaryArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SBitMaskRestriction {
    pub relBMR: u32,
    pub ulPropTag: u32,
    pub ulMask: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCommentRestriction {
    pub cValues: u32,
    pub lpRes: *mut SRestriction,
    pub lpProp: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SCommentRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SComparePropsRestriction {
    pub relop: u32,
    pub ulPropTag1: u32,
    pub ulPropTag2: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SContentRestriction {
    pub ulFuzzyLevel: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SContentRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCurrencyArray {
    pub cValues: u32,
    pub lpcur: *mut super::Com::CY,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SCurrencyArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SDateTimeArray {
    pub cValues: u32,
    pub lpft: *mut super::super::Foundation::FILETIME,
}
impl Default for SDateTimeArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SDoubleArray {
    pub cValues: u32,
    pub lpdbl: *mut f64,
}
impl Default for SDoubleArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_UI_ALLOWED: u32 = 16u32;
pub const SERVICE_UI_ALWAYS: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SExistRestriction {
    pub ulReserved1: u32,
    pub ulPropTag: u32,
    pub ulReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SGuidArray {
    pub cValues: u32,
    pub lpguid: *mut windows_core::GUID,
}
impl Default for SGuidArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SLPSTRArray {
    pub cValues: u32,
    pub lppszA: *mut windows_core::PSTR,
}
impl Default for SLPSTRArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SLargeIntegerArray {
    pub cValues: u32,
    pub lpli: *mut i64,
}
impl Default for SLargeIntegerArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SLongArray {
    pub cValues: u32,
    pub lpl: *mut i32,
}
impl Default for SLongArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SNotRestriction {
    pub ulReserved: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SNotRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOrRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SOrRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPropProblem {
    pub ulIndex: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPropProblemArray {
    pub cProblem: u32,
    pub aProblem: [SPropProblem; 1],
}
impl Default for SPropProblemArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPropTagArray {
    pub cValues: u32,
    pub aulPropTag: [u32; 1],
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
impl Default for SPropValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPropertyRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SPropertyRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SRealArray {
    pub cValues: u32,
    pub lpflt: *mut f32,
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
impl Default for SRestriction_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SRow {
    pub ulAdrEntryPad: u32,
    pub cValues: u32,
    pub lpProps: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SRow {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SRowSet {
    pub cRows: u32,
    pub aRow: [SRow; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SRowSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SShortArray {
    pub cValues: u32,
    pub lpi: *mut i16,
}
impl Default for SShortArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSizeRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub cb: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSortOrder {
    pub ulPropTag: u32,
    pub ulOrder: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SSortOrderSet {
    pub cSorts: u32,
    pub cCategories: u32,
    pub cExpanded: u32,
    pub aSort: [SSortOrder; 1],
}
impl Default for SSortOrderSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SSubRestriction {
    pub ulSubObject: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSubRestriction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STATUS_OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cValues: u32,
    pub lpPropVals: *mut SPropValue,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for STATUS_OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWStringArray {
    pub cValues: u32,
    pub lppszW: *mut windows_core::PWSTR,
}
impl Default for SWStringArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const S_IMAPI_BOTHADJUSTED: windows_core::HRESULT = windows_core::HRESULT(0xAA0006_u32 as _);
pub const S_IMAPI_COMMAND_HAS_SENSE_DATA: windows_core::HRESULT = windows_core::HRESULT(0xAA0200_u32 as _);
pub const S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xAA0A08_u32 as _);
pub const S_IMAPI_ROTATIONADJUSTED: windows_core::HRESULT = windows_core::HRESULT(0xAA0005_u32 as _);
pub const S_IMAPI_SPEEDADJUSTED: windows_core::HRESULT = windows_core::HRESULT(0xAA0004_u32 as _);
pub const S_IMAPI_WRITE_NOT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xAA0302_u32 as _);
pub const TABLE_CHANGED: u32 = 1u32;
pub const TABLE_ERROR: u32 = 2u32;
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
impl Default for TABLE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct WABEXTDISPLAY {
    pub cbSize: u32,
    pub lpWABObject: core::mem::ManuallyDrop<Option<IWABObject>>,
    pub lpAdrBook: core::mem::ManuallyDrop<Option<IAddrBook>>,
    pub lpPropObj: core::mem::ManuallyDrop<Option<IMAPIProp>>,
    pub fReadOnly: windows_core::BOOL,
    pub fDataChanged: windows_core::BOOL,
    pub ulFlags: u32,
    pub lpv: *mut core::ffi::c_void,
    pub lpsz: *mut i8,
}
impl Default for WABEXTDISPLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WABIMPORTPARAM {
    pub cbSize: u32,
    pub lpAdrBook: core::mem::ManuallyDrop<Option<IAddrBook>>,
    pub hWnd: super::super::Foundation::HWND,
    pub ulFlags: u32,
    pub lpszFileName: windows_core::PSTR,
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WAB_PARAM {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub szFileName: windows_core::PSTR,
    pub ulFlags: u32,
    pub guidPSExt: windows_core::GUID,
}
pub const WAB_PROFILE_CONTENTS: u32 = 2097152u32;
pub const WAB_USE_OE_SENDMAIL: u32 = 1u32;
pub const WAB_VCARD_FILE: u32 = 0u32;
pub const WAB_VCARD_STREAM: u32 = 1u32;
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
impl Default for __UPV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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

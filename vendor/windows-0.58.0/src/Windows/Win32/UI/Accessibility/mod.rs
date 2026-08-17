#[inline]
pub unsafe fn AccNotifyTouchInteraction<P0, P1>(hwndapp: P0, hwndtarget: P1, pttarget: super::super::Foundation::POINT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("oleacc.dll" "system" fn AccNotifyTouchInteraction(hwndapp : super::super::Foundation:: HWND, hwndtarget : super::super::Foundation:: HWND, pttarget : super::super::Foundation:: POINT) -> windows_core::HRESULT);
    AccNotifyTouchInteraction(hwndapp.param().abi(), hwndtarget.param().abi(), core::mem::transmute(pttarget)).ok()
}
#[inline]
pub unsafe fn AccSetRunningUtilityState<P0>(hwndapp: P0, dwutilitystatemask: u32, dwutilitystate: ACC_UTILITY_STATE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("oleacc.dll" "system" fn AccSetRunningUtilityState(hwndapp : super::super::Foundation:: HWND, dwutilitystatemask : u32, dwutilitystate : ACC_UTILITY_STATE_FLAGS) -> windows_core::HRESULT);
    AccSetRunningUtilityState(hwndapp.param().abi(), dwutilitystatemask, dwutilitystate).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn AccessibleChildren<P0>(pacccontainer: P0, ichildstart: i32, rgvarchildren: &mut [windows_core::VARIANT], pcobtained: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<IAccessible>,
{
    windows_targets::link!("oleacc.dll" "system" fn AccessibleChildren(pacccontainer : * mut core::ffi::c_void, ichildstart : i32, cchildren : i32, rgvarchildren : *mut core::mem::MaybeUninit < windows_core::VARIANT >, pcobtained : *mut i32) -> windows_core::HRESULT);
    AccessibleChildren(pacccontainer.param().abi(), ichildstart, rgvarchildren.len().try_into().unwrap(), core::mem::transmute(rgvarchildren.as_ptr()), pcobtained).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn AccessibleObjectFromEvent<P0>(hwnd: P0, dwid: u32, dwchildid: u32, ppacc: *mut Option<IAccessible>, pvarchild: *mut windows_core::VARIANT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("oleacc.dll" "system" fn AccessibleObjectFromEvent(hwnd : super::super::Foundation:: HWND, dwid : u32, dwchildid : u32, ppacc : *mut * mut core::ffi::c_void, pvarchild : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    AccessibleObjectFromEvent(hwnd.param().abi(), dwid, dwchildid, core::mem::transmute(ppacc), core::mem::transmute(pvarchild)).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn AccessibleObjectFromPoint(ptscreen: super::super::Foundation::POINT, ppacc: *mut Option<IAccessible>, pvarchild: *mut windows_core::VARIANT) -> windows_core::Result<()> {
    windows_targets::link!("oleacc.dll" "system" fn AccessibleObjectFromPoint(ptscreen : super::super::Foundation:: POINT, ppacc : *mut * mut core::ffi::c_void, pvarchild : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    AccessibleObjectFromPoint(core::mem::transmute(ptscreen), core::mem::transmute(ppacc), core::mem::transmute(pvarchild)).ok()
}
#[inline]
pub unsafe fn AccessibleObjectFromWindow<P0>(hwnd: P0, dwid: u32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("oleacc.dll" "system" fn AccessibleObjectFromWindow(hwnd : super::super::Foundation:: HWND, dwid : u32, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    AccessibleObjectFromWindow(hwnd.param().abi(), dwid, riid, ppvobject).ok()
}
#[inline]
pub unsafe fn CreateStdAccessibleObject<P0>(hwnd: P0, idobject: i32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("oleacc.dll" "system" fn CreateStdAccessibleObject(hwnd : super::super::Foundation:: HWND, idobject : i32, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CreateStdAccessibleObject(hwnd.param().abi(), idobject, riid, ppvobject).ok()
}
#[inline]
pub unsafe fn CreateStdAccessibleProxyA<P0, P1>(hwnd: P0, pclassname: P1, idobject: i32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("oleacc.dll" "system" fn CreateStdAccessibleProxyA(hwnd : super::super::Foundation:: HWND, pclassname : windows_core::PCSTR, idobject : i32, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CreateStdAccessibleProxyA(hwnd.param().abi(), pclassname.param().abi(), idobject, riid, ppvobject).ok()
}
#[inline]
pub unsafe fn CreateStdAccessibleProxyW<P0, P1>(hwnd: P0, pclassname: P1, idobject: i32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleacc.dll" "system" fn CreateStdAccessibleProxyW(hwnd : super::super::Foundation:: HWND, pclassname : windows_core::PCWSTR, idobject : i32, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CreateStdAccessibleProxyW(hwnd.param().abi(), pclassname.param().abi(), idobject, riid, ppvobject).ok()
}
#[inline]
pub unsafe fn DockPattern_SetDockPosition<P0>(hobj: P0, dockposition: DockPosition) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn DockPattern_SetDockPosition(hobj : HUIAPATTERNOBJECT, dockposition : DockPosition) -> windows_core::HRESULT);
    DockPattern_SetDockPosition(hobj.param().abi(), dockposition).ok()
}
#[inline]
pub unsafe fn ExpandCollapsePattern_Collapse<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn ExpandCollapsePattern_Collapse(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    ExpandCollapsePattern_Collapse(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn ExpandCollapsePattern_Expand<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn ExpandCollapsePattern_Expand(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    ExpandCollapsePattern_Expand(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn GetOleaccVersionInfo(pver: *mut u32, pbuild: *mut u32) {
    windows_targets::link!("oleacc.dll" "system" fn GetOleaccVersionInfo(pver : *mut u32, pbuild : *mut u32));
    GetOleaccVersionInfo(pver, pbuild)
}
#[inline]
pub unsafe fn GetRoleTextA(lrole: u32, lpszrole: Option<&mut [u8]>) -> u32 {
    windows_targets::link!("oleacc.dll" "system" fn GetRoleTextA(lrole : u32, lpszrole : windows_core::PSTR, cchrolemax : u32) -> u32);
    GetRoleTextA(lrole, core::mem::transmute(lpszrole.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszrole.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetRoleTextW(lrole: u32, lpszrole: Option<&mut [u16]>) -> u32 {
    windows_targets::link!("oleacc.dll" "system" fn GetRoleTextW(lrole : u32, lpszrole : windows_core::PWSTR, cchrolemax : u32) -> u32);
    GetRoleTextW(lrole, core::mem::transmute(lpszrole.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszrole.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetStateTextA(lstatebit: u32, lpszstate: Option<&mut [u8]>) -> u32 {
    windows_targets::link!("oleacc.dll" "system" fn GetStateTextA(lstatebit : u32, lpszstate : windows_core::PSTR, cchstate : u32) -> u32);
    GetStateTextA(lstatebit, core::mem::transmute(lpszstate.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszstate.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetStateTextW(lstatebit: u32, lpszstate: Option<&mut [u16]>) -> u32 {
    windows_targets::link!("oleacc.dll" "system" fn GetStateTextW(lstatebit : u32, lpszstate : windows_core::PWSTR, cchstate : u32) -> u32);
    GetStateTextW(lstatebit, core::mem::transmute(lpszstate.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszstate.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GridPattern_GetItem<P0>(hobj: P0, row: i32, column: i32, presult: *mut HUIANODE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn GridPattern_GetItem(hobj : HUIAPATTERNOBJECT, row : i32, column : i32, presult : *mut HUIANODE) -> windows_core::HRESULT);
    GridPattern_GetItem(hobj.param().abi(), row, column, presult).ok()
}
#[inline]
pub unsafe fn InvokePattern_Invoke<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn InvokePattern_Invoke(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    InvokePattern_Invoke(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn IsWinEventHookInstalled(event: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn IsWinEventHookInstalled(event : u32) -> super::super::Foundation:: BOOL);
    IsWinEventHookInstalled(event)
}
#[inline]
pub unsafe fn ItemContainerPattern_FindItemByProperty<P0, P1, P2>(hobj: P0, hnodestartafter: P1, propertyid: i32, value: P2, pfound: *mut HUIANODE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
    P1: windows_core::Param<HUIANODE>,
    P2: windows_core::Param<windows_core::VARIANT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn ItemContainerPattern_FindItemByProperty(hobj : HUIAPATTERNOBJECT, hnodestartafter : HUIANODE, propertyid : i32, value : core::mem::MaybeUninit < windows_core::VARIANT >, pfound : *mut HUIANODE) -> windows_core::HRESULT);
    ItemContainerPattern_FindItemByProperty(hobj.param().abi(), hnodestartafter.param().abi(), propertyid, value.param().abi(), pfound).ok()
}
#[inline]
pub unsafe fn LegacyIAccessiblePattern_DoDefaultAction<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn LegacyIAccessiblePattern_DoDefaultAction(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    LegacyIAccessiblePattern_DoDefaultAction(hobj.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LegacyIAccessiblePattern_GetIAccessible<P0>(hobj: P0) -> windows_core::Result<IAccessible>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn LegacyIAccessiblePattern_GetIAccessible(hobj : HUIAPATTERNOBJECT, paccessible : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    LegacyIAccessiblePattern_GetIAccessible(hobj.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn LegacyIAccessiblePattern_Select<P0>(hobj: P0, flagsselect: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn LegacyIAccessiblePattern_Select(hobj : HUIAPATTERNOBJECT, flagsselect : i32) -> windows_core::HRESULT);
    LegacyIAccessiblePattern_Select(hobj.param().abi(), flagsselect).ok()
}
#[inline]
pub unsafe fn LegacyIAccessiblePattern_SetValue<P0, P1>(hobj: P0, szvalue: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn LegacyIAccessiblePattern_SetValue(hobj : HUIAPATTERNOBJECT, szvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    LegacyIAccessiblePattern_SetValue(hobj.param().abi(), szvalue.param().abi()).ok()
}
#[inline]
pub unsafe fn LresultFromObject<P0, P1>(riid: *const windows_core::GUID, wparam: P0, punk: P1) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::WPARAM>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("oleacc.dll" "system" fn LresultFromObject(riid : *const windows_core::GUID, wparam : super::super::Foundation:: WPARAM, punk : * mut core::ffi::c_void) -> super::super::Foundation:: LRESULT);
    LresultFromObject(riid, wparam.param().abi(), punk.param().abi())
}
#[inline]
pub unsafe fn MultipleViewPattern_GetViewName<P0>(hobj: P0, viewid: i32, ppstr: *mut windows_core::BSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn MultipleViewPattern_GetViewName(hobj : HUIAPATTERNOBJECT, viewid : i32, ppstr : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    MultipleViewPattern_GetViewName(hobj.param().abi(), viewid, core::mem::transmute(ppstr)).ok()
}
#[inline]
pub unsafe fn MultipleViewPattern_SetCurrentView<P0>(hobj: P0, viewid: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn MultipleViewPattern_SetCurrentView(hobj : HUIAPATTERNOBJECT, viewid : i32) -> windows_core::HRESULT);
    MultipleViewPattern_SetCurrentView(hobj.param().abi(), viewid).ok()
}
#[inline]
pub unsafe fn NotifyWinEvent<P0>(event: u32, hwnd: P0, idobject: i32, idchild: i32)
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn NotifyWinEvent(event : u32, hwnd : super::super::Foundation:: HWND, idobject : i32, idchild : i32));
    NotifyWinEvent(event, hwnd.param().abi(), idobject, idchild)
}
#[inline]
pub unsafe fn ObjectFromLresult<P0, P1>(lresult: P0, riid: *const windows_core::GUID, wparam: P1, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::LRESULT>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
{
    windows_targets::link!("oleacc.dll" "system" fn ObjectFromLresult(lresult : super::super::Foundation:: LRESULT, riid : *const windows_core::GUID, wparam : super::super::Foundation:: WPARAM, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    ObjectFromLresult(lresult.param().abi(), riid, wparam.param().abi(), ppvobject).ok()
}
#[inline]
pub unsafe fn RangeValuePattern_SetValue<P0>(hobj: P0, val: f64) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn RangeValuePattern_SetValue(hobj : HUIAPATTERNOBJECT, val : f64) -> windows_core::HRESULT);
    RangeValuePattern_SetValue(hobj.param().abi(), val).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterPointerInputTarget<P0>(hwnd: P0, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterPointerInputTarget(hwnd : super::super::Foundation:: HWND, pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE) -> super::super::Foundation:: BOOL);
    RegisterPointerInputTarget(hwnd.param().abi(), pointertype).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterPointerInputTargetEx<P0, P1>(hwnd: P0, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, fobserve: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterPointerInputTargetEx(hwnd : super::super::Foundation:: HWND, pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE, fobserve : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    RegisterPointerInputTargetEx(hwnd.param().abi(), pointertype, fobserve.param().abi())
}
#[inline]
pub unsafe fn ScrollItemPattern_ScrollIntoView<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn ScrollItemPattern_ScrollIntoView(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    ScrollItemPattern_ScrollIntoView(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn ScrollPattern_Scroll<P0>(hobj: P0, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn ScrollPattern_Scroll(hobj : HUIAPATTERNOBJECT, horizontalamount : ScrollAmount, verticalamount : ScrollAmount) -> windows_core::HRESULT);
    ScrollPattern_Scroll(hobj.param().abi(), horizontalamount, verticalamount).ok()
}
#[inline]
pub unsafe fn ScrollPattern_SetScrollPercent<P0>(hobj: P0, horizontalpercent: f64, verticalpercent: f64) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn ScrollPattern_SetScrollPercent(hobj : HUIAPATTERNOBJECT, horizontalpercent : f64, verticalpercent : f64) -> windows_core::HRESULT);
    ScrollPattern_SetScrollPercent(hobj.param().abi(), horizontalpercent, verticalpercent).ok()
}
#[inline]
pub unsafe fn SelectionItemPattern_AddToSelection<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn SelectionItemPattern_AddToSelection(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    SelectionItemPattern_AddToSelection(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn SelectionItemPattern_RemoveFromSelection<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn SelectionItemPattern_RemoveFromSelection(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    SelectionItemPattern_RemoveFromSelection(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn SelectionItemPattern_Select<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn SelectionItemPattern_Select(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    SelectionItemPattern_Select(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn SetWinEventHook<P0>(eventmin: u32, eventmax: u32, hmodwineventproc: P0, pfnwineventproc: WINEVENTPROC, idprocess: u32, idthread: u32, dwflags: u32) -> HWINEVENTHOOK
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("user32.dll" "system" fn SetWinEventHook(eventmin : u32, eventmax : u32, hmodwineventproc : super::super::Foundation:: HMODULE, pfnwineventproc : WINEVENTPROC, idprocess : u32, idthread : u32, dwflags : u32) -> HWINEVENTHOOK);
    SetWinEventHook(eventmin, eventmax, hmodwineventproc.param().abi(), pfnwineventproc, idprocess, idthread, dwflags)
}
#[inline]
pub unsafe fn SynchronizedInputPattern_Cancel<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn SynchronizedInputPattern_Cancel(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    SynchronizedInputPattern_Cancel(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn SynchronizedInputPattern_StartListening<P0>(hobj: P0, inputtype: SynchronizedInputType) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn SynchronizedInputPattern_StartListening(hobj : HUIAPATTERNOBJECT, inputtype : SynchronizedInputType) -> windows_core::HRESULT);
    SynchronizedInputPattern_StartListening(hobj.param().abi(), inputtype).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn TextPattern_GetSelection<P0>(hobj: P0, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextPattern_GetSelection(hobj : HUIAPATTERNOBJECT, pretval : *mut *mut super::super::System::Com:: SAFEARRAY) -> windows_core::HRESULT);
    TextPattern_GetSelection(hobj.param().abi(), pretval).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn TextPattern_GetVisibleRanges<P0>(hobj: P0, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextPattern_GetVisibleRanges(hobj : HUIAPATTERNOBJECT, pretval : *mut *mut super::super::System::Com:: SAFEARRAY) -> windows_core::HRESULT);
    TextPattern_GetVisibleRanges(hobj.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextPattern_RangeFromChild<P0, P1>(hobj: P0, hnodechild: P1, pretval: *mut HUIATEXTRANGE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
    P1: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextPattern_RangeFromChild(hobj : HUIAPATTERNOBJECT, hnodechild : HUIANODE, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    TextPattern_RangeFromChild(hobj.param().abi(), hnodechild.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextPattern_RangeFromPoint<P0>(hobj: P0, point: UiaPoint, pretval: *mut HUIATEXTRANGE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextPattern_RangeFromPoint(hobj : HUIAPATTERNOBJECT, point : UiaPoint, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    TextPattern_RangeFromPoint(hobj.param().abi(), core::mem::transmute(point), pretval).ok()
}
#[inline]
pub unsafe fn TextPattern_get_DocumentRange<P0>(hobj: P0, pretval: *mut HUIATEXTRANGE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextPattern_get_DocumentRange(hobj : HUIAPATTERNOBJECT, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    TextPattern_get_DocumentRange(hobj.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextPattern_get_SupportedTextSelection<P0>(hobj: P0, pretval: *mut SupportedTextSelection) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextPattern_get_SupportedTextSelection(hobj : HUIAPATTERNOBJECT, pretval : *mut SupportedTextSelection) -> windows_core::HRESULT);
    TextPattern_get_SupportedTextSelection(hobj.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextRange_AddToSelection<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_AddToSelection(hobj : HUIATEXTRANGE) -> windows_core::HRESULT);
    TextRange_AddToSelection(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn TextRange_Clone<P0>(hobj: P0, pretval: *mut HUIATEXTRANGE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_Clone(hobj : HUIATEXTRANGE, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    TextRange_Clone(hobj.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextRange_Compare<P0, P1>(hobj: P0, range: P1, pretval: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
    P1: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_Compare(hobj : HUIATEXTRANGE, range : HUIATEXTRANGE, pretval : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    TextRange_Compare(hobj.param().abi(), range.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextRange_CompareEndpoints<P0, P1>(hobj: P0, endpoint: TextPatternRangeEndpoint, targetrange: P1, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
    P1: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_CompareEndpoints(hobj : HUIATEXTRANGE, endpoint : TextPatternRangeEndpoint, targetrange : HUIATEXTRANGE, targetendpoint : TextPatternRangeEndpoint, pretval : *mut i32) -> windows_core::HRESULT);
    TextRange_CompareEndpoints(hobj.param().abi(), endpoint, targetrange.param().abi(), targetendpoint, pretval).ok()
}
#[inline]
pub unsafe fn TextRange_ExpandToEnclosingUnit<P0>(hobj: P0, unit: TextUnit) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_ExpandToEnclosingUnit(hobj : HUIATEXTRANGE, unit : TextUnit) -> windows_core::HRESULT);
    TextRange_ExpandToEnclosingUnit(hobj.param().abi(), unit).ok()
}
#[inline]
pub unsafe fn TextRange_FindAttribute<P0, P1, P2>(hobj: P0, attributeid: i32, val: P1, backward: P2, pretval: *mut HUIATEXTRANGE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
    P1: windows_core::Param<windows_core::VARIANT>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_FindAttribute(hobj : HUIATEXTRANGE, attributeid : i32, val : core::mem::MaybeUninit < windows_core::VARIANT >, backward : super::super::Foundation:: BOOL, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    TextRange_FindAttribute(hobj.param().abi(), attributeid, val.param().abi(), backward.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextRange_FindText<P0, P1, P2, P3>(hobj: P0, text: P1, backward: P2, ignorecase: P3, pretval: *mut HUIATEXTRANGE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
    P1: windows_core::Param<windows_core::BSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_FindText(hobj : HUIATEXTRANGE, text : core::mem::MaybeUninit < windows_core::BSTR >, backward : super::super::Foundation:: BOOL, ignorecase : super::super::Foundation:: BOOL, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    TextRange_FindText(hobj.param().abi(), text.param().abi(), backward.param().abi(), ignorecase.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextRange_GetAttributeValue<P0>(hobj: P0, attributeid: i32, pretval: *mut windows_core::VARIANT) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_GetAttributeValue(hobj : HUIATEXTRANGE, attributeid : i32, pretval : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    TextRange_GetAttributeValue(hobj.param().abi(), attributeid, core::mem::transmute(pretval)).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn TextRange_GetBoundingRectangles<P0>(hobj: P0, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_GetBoundingRectangles(hobj : HUIATEXTRANGE, pretval : *mut *mut super::super::System::Com:: SAFEARRAY) -> windows_core::HRESULT);
    TextRange_GetBoundingRectangles(hobj.param().abi(), pretval).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn TextRange_GetChildren<P0>(hobj: P0, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_GetChildren(hobj : HUIATEXTRANGE, pretval : *mut *mut super::super::System::Com:: SAFEARRAY) -> windows_core::HRESULT);
    TextRange_GetChildren(hobj.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextRange_GetEnclosingElement<P0>(hobj: P0, pretval: *mut HUIANODE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_GetEnclosingElement(hobj : HUIATEXTRANGE, pretval : *mut HUIANODE) -> windows_core::HRESULT);
    TextRange_GetEnclosingElement(hobj.param().abi(), pretval).ok()
}
#[inline]
pub unsafe fn TextRange_GetText<P0>(hobj: P0, maxlength: i32, pretval: *mut windows_core::BSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_GetText(hobj : HUIATEXTRANGE, maxlength : i32, pretval : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    TextRange_GetText(hobj.param().abi(), maxlength, core::mem::transmute(pretval)).ok()
}
#[inline]
pub unsafe fn TextRange_Move<P0>(hobj: P0, unit: TextUnit, count: i32, pretval: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_Move(hobj : HUIATEXTRANGE, unit : TextUnit, count : i32, pretval : *mut i32) -> windows_core::HRESULT);
    TextRange_Move(hobj.param().abi(), unit, count, pretval).ok()
}
#[inline]
pub unsafe fn TextRange_MoveEndpointByRange<P0, P1>(hobj: P0, endpoint: TextPatternRangeEndpoint, targetrange: P1, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
    P1: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_MoveEndpointByRange(hobj : HUIATEXTRANGE, endpoint : TextPatternRangeEndpoint, targetrange : HUIATEXTRANGE, targetendpoint : TextPatternRangeEndpoint) -> windows_core::HRESULT);
    TextRange_MoveEndpointByRange(hobj.param().abi(), endpoint, targetrange.param().abi(), targetendpoint).ok()
}
#[inline]
pub unsafe fn TextRange_MoveEndpointByUnit<P0>(hobj: P0, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_MoveEndpointByUnit(hobj : HUIATEXTRANGE, endpoint : TextPatternRangeEndpoint, unit : TextUnit, count : i32, pretval : *mut i32) -> windows_core::HRESULT);
    TextRange_MoveEndpointByUnit(hobj.param().abi(), endpoint, unit, count, pretval).ok()
}
#[inline]
pub unsafe fn TextRange_RemoveFromSelection<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_RemoveFromSelection(hobj : HUIATEXTRANGE) -> windows_core::HRESULT);
    TextRange_RemoveFromSelection(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn TextRange_ScrollIntoView<P0, P1>(hobj: P0, aligntotop: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_ScrollIntoView(hobj : HUIATEXTRANGE, aligntotop : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    TextRange_ScrollIntoView(hobj.param().abi(), aligntotop.param().abi()).ok()
}
#[inline]
pub unsafe fn TextRange_Select<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TextRange_Select(hobj : HUIATEXTRANGE) -> windows_core::HRESULT);
    TextRange_Select(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn TogglePattern_Toggle<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TogglePattern_Toggle(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    TogglePattern_Toggle(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn TransformPattern_Move<P0>(hobj: P0, x: f64, y: f64) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TransformPattern_Move(hobj : HUIAPATTERNOBJECT, x : f64, y : f64) -> windows_core::HRESULT);
    TransformPattern_Move(hobj.param().abi(), x, y).ok()
}
#[inline]
pub unsafe fn TransformPattern_Resize<P0>(hobj: P0, width: f64, height: f64) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TransformPattern_Resize(hobj : HUIAPATTERNOBJECT, width : f64, height : f64) -> windows_core::HRESULT);
    TransformPattern_Resize(hobj.param().abi(), width, height).ok()
}
#[inline]
pub unsafe fn TransformPattern_Rotate<P0>(hobj: P0, degrees: f64) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn TransformPattern_Rotate(hobj : HUIAPATTERNOBJECT, degrees : f64) -> windows_core::HRESULT);
    TransformPattern_Rotate(hobj.param().abi(), degrees).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaAddEvent<P0>(hnode: P0, eventid: i32, pcallback: *mut UiaEventCallback, scope: TreeScope, pproperties: *mut i32, cproperties: i32, prequest: *mut UiaCacheRequest, phevent: *mut HUIAEVENT) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaAddEvent(hnode : HUIANODE, eventid : i32, pcallback : *mut UiaEventCallback, scope : TreeScope, pproperties : *mut i32, cproperties : i32, prequest : *mut UiaCacheRequest, phevent : *mut HUIAEVENT) -> windows_core::HRESULT);
    UiaAddEvent(hnode.param().abi(), eventid, pcallback, scope, pproperties, cproperties, prequest, phevent).ok()
}
#[inline]
pub unsafe fn UiaClientsAreListening() -> super::super::Foundation::BOOL {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaClientsAreListening() -> super::super::Foundation:: BOOL);
    UiaClientsAreListening()
}
#[inline]
pub unsafe fn UiaDisconnectAllProviders() -> windows_core::Result<()> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaDisconnectAllProviders() -> windows_core::HRESULT);
    UiaDisconnectAllProviders().ok()
}
#[inline]
pub unsafe fn UiaDisconnectProvider<P0>(pprovider: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaDisconnectProvider(pprovider : * mut core::ffi::c_void) -> windows_core::HRESULT);
    UiaDisconnectProvider(pprovider.param().abi()).ok()
}
#[inline]
pub unsafe fn UiaEventAddWindow<P0, P1>(hevent: P0, hwnd: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAEVENT>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaEventAddWindow(hevent : HUIAEVENT, hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    UiaEventAddWindow(hevent.param().abi(), hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn UiaEventRemoveWindow<P0, P1>(hevent: P0, hwnd: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAEVENT>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaEventRemoveWindow(hevent : HUIAEVENT, hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    UiaEventRemoveWindow(hevent.param().abi(), hwnd.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaFind<P0>(hnode: P0, pparams: *mut UiaFindParams, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, ppoffsets: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructures: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaFind(hnode : HUIANODE, pparams : *mut UiaFindParams, prequest : *mut UiaCacheRequest, pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY, ppoffsets : *mut *mut super::super::System::Com:: SAFEARRAY, pptreestructures : *mut *mut super::super::System::Com:: SAFEARRAY) -> windows_core::HRESULT);
    UiaFind(hnode.param().abi(), pparams, prequest, pprequesteddata, ppoffsets, pptreestructures).ok()
}
#[inline]
pub unsafe fn UiaGetErrorDescription(pdescription: *mut windows_core::BSTR) -> super::super::Foundation::BOOL {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaGetErrorDescription(pdescription : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> super::super::Foundation:: BOOL);
    UiaGetErrorDescription(core::mem::transmute(pdescription))
}
#[inline]
pub unsafe fn UiaGetPatternProvider<P0>(hnode: P0, patternid: i32, phobj: *mut HUIAPATTERNOBJECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaGetPatternProvider(hnode : HUIANODE, patternid : i32, phobj : *mut HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    UiaGetPatternProvider(hnode.param().abi(), patternid, phobj).ok()
}
#[inline]
pub unsafe fn UiaGetPropertyValue<P0>(hnode: P0, propertyid: i32, pvalue: *mut windows_core::VARIANT) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaGetPropertyValue(hnode : HUIANODE, propertyid : i32, pvalue : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    UiaGetPropertyValue(hnode.param().abi(), propertyid, core::mem::transmute(pvalue)).ok()
}
#[inline]
pub unsafe fn UiaGetReservedMixedAttributeValue() -> windows_core::Result<windows_core::IUnknown> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaGetReservedMixedAttributeValue(punkmixedattributevalue : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    UiaGetReservedMixedAttributeValue(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn UiaGetReservedNotSupportedValue() -> windows_core::Result<windows_core::IUnknown> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaGetReservedNotSupportedValue(punknotsupportedvalue : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    UiaGetReservedNotSupportedValue(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn UiaGetRootNode(phnode: *mut HUIANODE) -> windows_core::Result<()> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaGetRootNode(phnode : *mut HUIANODE) -> windows_core::HRESULT);
    UiaGetRootNode(phnode).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaGetRuntimeId<P0>(hnode: P0, pruntimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaGetRuntimeId(hnode : HUIANODE, pruntimeid : *mut *mut super::super::System::Com:: SAFEARRAY) -> windows_core::HRESULT);
    UiaGetRuntimeId(hnode.param().abi(), pruntimeid).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaGetUpdatedCache<P0>(hnode: P0, prequest: *mut UiaCacheRequest, normalizestate: NormalizeState, pnormalizecondition: *mut UiaCondition, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut windows_core::BSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaGetUpdatedCache(hnode : HUIANODE, prequest : *mut UiaCacheRequest, normalizestate : NormalizeState, pnormalizecondition : *mut UiaCondition, pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY, pptreestructure : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    UiaGetUpdatedCache(hnode.param().abi(), prequest, normalizestate, pnormalizecondition, pprequesteddata, core::mem::transmute(pptreestructure)).ok()
}
#[inline]
pub unsafe fn UiaHPatternObjectFromVariant(pvar: *mut windows_core::VARIANT, phobj: *mut HUIAPATTERNOBJECT) -> windows_core::Result<()> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaHPatternObjectFromVariant(pvar : *mut core::mem::MaybeUninit < windows_core::VARIANT >, phobj : *mut HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    UiaHPatternObjectFromVariant(core::mem::transmute(pvar), phobj).ok()
}
#[inline]
pub unsafe fn UiaHTextRangeFromVariant(pvar: *mut windows_core::VARIANT, phtextrange: *mut HUIATEXTRANGE) -> windows_core::Result<()> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaHTextRangeFromVariant(pvar : *mut core::mem::MaybeUninit < windows_core::VARIANT >, phtextrange : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    UiaHTextRangeFromVariant(core::mem::transmute(pvar), phtextrange).ok()
}
#[inline]
pub unsafe fn UiaHUiaNodeFromVariant(pvar: *mut windows_core::VARIANT, phnode: *mut HUIANODE) -> windows_core::Result<()> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaHUiaNodeFromVariant(pvar : *mut core::mem::MaybeUninit < windows_core::VARIANT >, phnode : *mut HUIANODE) -> windows_core::HRESULT);
    UiaHUiaNodeFromVariant(core::mem::transmute(pvar), phnode).ok()
}
#[inline]
pub unsafe fn UiaHasServerSideProvider<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaHasServerSideProvider(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    UiaHasServerSideProvider(hwnd.param().abi())
}
#[inline]
pub unsafe fn UiaHostProviderFromHwnd<P0>(hwnd: P0) -> windows_core::Result<IRawElementProviderSimple>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaHostProviderFromHwnd(hwnd : super::super::Foundation:: HWND, ppprovider : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    UiaHostProviderFromHwnd(hwnd.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaIAccessibleFromProvider<P0>(pprovider: P0, dwflags: u32, ppaccessible: *mut Option<IAccessible>, pvarchild: *mut windows_core::VARIANT) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaIAccessibleFromProvider(pprovider : * mut core::ffi::c_void, dwflags : u32, ppaccessible : *mut * mut core::ffi::c_void, pvarchild : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    UiaIAccessibleFromProvider(pprovider.param().abi(), dwflags, core::mem::transmute(ppaccessible), core::mem::transmute(pvarchild)).ok()
}
#[inline]
pub unsafe fn UiaLookupId(r#type: AutomationIdentifierType, pguid: *const windows_core::GUID) -> i32 {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaLookupId(r#type : AutomationIdentifierType, pguid : *const windows_core::GUID) -> i32);
    UiaLookupId(r#type, pguid)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaNavigate<P0>(hnode: P0, direction: NavigateDirection, pcondition: *mut UiaCondition, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut windows_core::BSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaNavigate(hnode : HUIANODE, direction : NavigateDirection, pcondition : *mut UiaCondition, prequest : *mut UiaCacheRequest, pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY, pptreestructure : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    UiaNavigate(hnode.param().abi(), direction, pcondition, prequest, pprequesteddata, core::mem::transmute(pptreestructure)).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaNodeFromFocus(prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut windows_core::BSTR) -> windows_core::Result<()> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaNodeFromFocus(prequest : *mut UiaCacheRequest, pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY, pptreestructure : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    UiaNodeFromFocus(prequest, pprequesteddata, core::mem::transmute(pptreestructure)).ok()
}
#[inline]
pub unsafe fn UiaNodeFromHandle<P0>(hwnd: P0, phnode: *mut HUIANODE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaNodeFromHandle(hwnd : super::super::Foundation:: HWND, phnode : *mut HUIANODE) -> windows_core::HRESULT);
    UiaNodeFromHandle(hwnd.param().abi(), phnode).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaNodeFromPoint(x: f64, y: f64, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut windows_core::BSTR) -> windows_core::Result<()> {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaNodeFromPoint(x : f64, y : f64, prequest : *mut UiaCacheRequest, pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY, pptreestructure : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    UiaNodeFromPoint(x, y, prequest, pprequesteddata, core::mem::transmute(pptreestructure)).ok()
}
#[inline]
pub unsafe fn UiaNodeFromProvider<P0>(pprovider: P0, phnode: *mut HUIANODE) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaNodeFromProvider(pprovider : * mut core::ffi::c_void, phnode : *mut HUIANODE) -> windows_core::HRESULT);
    UiaNodeFromProvider(pprovider.param().abi(), phnode).ok()
}
#[inline]
pub unsafe fn UiaNodeRelease<P0>(hnode: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaNodeRelease(hnode : HUIANODE) -> super::super::Foundation:: BOOL);
    UiaNodeRelease(hnode.param().abi())
}
#[inline]
pub unsafe fn UiaPatternRelease<P0>(hobj: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaPatternRelease(hobj : HUIAPATTERNOBJECT) -> super::super::Foundation:: BOOL);
    UiaPatternRelease(hobj.param().abi())
}
#[inline]
pub unsafe fn UiaProviderForNonClient<P0>(hwnd: P0, idobject: i32, idchild: i32) -> windows_core::Result<IRawElementProviderSimple>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaProviderForNonClient(hwnd : super::super::Foundation:: HWND, idobject : i32, idchild : i32, ppprovider : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    UiaProviderForNonClient(hwnd.param().abi(), idobject, idchild, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaProviderFromIAccessible<P0>(paccessible: P0, idchild: i32, dwflags: u32) -> windows_core::Result<IRawElementProviderSimple>
where
    P0: windows_core::Param<IAccessible>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaProviderFromIAccessible(paccessible : * mut core::ffi::c_void, idchild : i32, dwflags : u32, ppprovider : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    UiaProviderFromIAccessible(paccessible.param().abi(), idchild, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn UiaRaiseActiveTextPositionChangedEvent<P0, P1>(provider: P0, textrange: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
    P1: windows_core::Param<ITextRangeProvider>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRaiseActiveTextPositionChangedEvent(provider : * mut core::ffi::c_void, textrange : * mut core::ffi::c_void) -> windows_core::HRESULT);
    UiaRaiseActiveTextPositionChangedEvent(provider.param().abi(), textrange.param().abi()).ok()
}
#[inline]
pub unsafe fn UiaRaiseAsyncContentLoadedEvent<P0>(pprovider: P0, asynccontentloadedstate: AsyncContentLoadedState, percentcomplete: f64) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRaiseAsyncContentLoadedEvent(pprovider : * mut core::ffi::c_void, asynccontentloadedstate : AsyncContentLoadedState, percentcomplete : f64) -> windows_core::HRESULT);
    UiaRaiseAsyncContentLoadedEvent(pprovider.param().abi(), asynccontentloadedstate, percentcomplete).ok()
}
#[inline]
pub unsafe fn UiaRaiseAutomationEvent<P0>(pprovider: P0, id: UIA_EVENT_ID) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRaiseAutomationEvent(pprovider : * mut core::ffi::c_void, id : UIA_EVENT_ID) -> windows_core::HRESULT);
    UiaRaiseAutomationEvent(pprovider.param().abi(), id).ok()
}
#[inline]
pub unsafe fn UiaRaiseAutomationPropertyChangedEvent<P0, P1, P2>(pprovider: P0, id: UIA_PROPERTY_ID, oldvalue: P1, newvalue: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
    P1: windows_core::Param<windows_core::VARIANT>,
    P2: windows_core::Param<windows_core::VARIANT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRaiseAutomationPropertyChangedEvent(pprovider : * mut core::ffi::c_void, id : UIA_PROPERTY_ID, oldvalue : core::mem::MaybeUninit < windows_core::VARIANT >, newvalue : core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    UiaRaiseAutomationPropertyChangedEvent(pprovider.param().abi(), id, oldvalue.param().abi(), newvalue.param().abi()).ok()
}
#[inline]
pub unsafe fn UiaRaiseChangesEvent<P0>(pprovider: P0, eventidcount: i32, puiachanges: *mut UiaChangeInfo) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRaiseChangesEvent(pprovider : * mut core::ffi::c_void, eventidcount : i32, puiachanges : *mut UiaChangeInfo) -> windows_core::HRESULT);
    UiaRaiseChangesEvent(pprovider.param().abi(), eventidcount, puiachanges).ok()
}
#[inline]
pub unsafe fn UiaRaiseNotificationEvent<P0, P1, P2>(provider: P0, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: P1, activityid: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
    P1: windows_core::Param<windows_core::BSTR>,
    P2: windows_core::Param<windows_core::BSTR>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRaiseNotificationEvent(provider : * mut core::ffi::c_void, notificationkind : NotificationKind, notificationprocessing : NotificationProcessing, displaystring : core::mem::MaybeUninit < windows_core::BSTR >, activityid : core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    UiaRaiseNotificationEvent(provider.param().abi(), notificationkind, notificationprocessing, displaystring.param().abi(), activityid.param().abi()).ok()
}
#[inline]
pub unsafe fn UiaRaiseStructureChangedEvent<P0>(pprovider: P0, structurechangetype: StructureChangeType, pruntimeid: *mut i32, cruntimeidlen: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRaiseStructureChangedEvent(pprovider : * mut core::ffi::c_void, structurechangetype : StructureChangeType, pruntimeid : *mut i32, cruntimeidlen : i32) -> windows_core::HRESULT);
    UiaRaiseStructureChangedEvent(pprovider.param().abi(), structurechangetype, pruntimeid, cruntimeidlen).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaRaiseTextEditTextChangedEvent<P0>(pprovider: P0, texteditchangetype: TextEditChangeType, pchangeddata: *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRaiseTextEditTextChangedEvent(pprovider : * mut core::ffi::c_void, texteditchangetype : TextEditChangeType, pchangeddata : *mut super::super::System::Com:: SAFEARRAY) -> windows_core::HRESULT);
    UiaRaiseTextEditTextChangedEvent(pprovider.param().abi(), texteditchangetype, pchangeddata).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaRegisterProviderCallback(pcallback: *mut UiaProviderCallback) {
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRegisterProviderCallback(pcallback : *mut UiaProviderCallback));
    UiaRegisterProviderCallback(pcallback)
}
#[inline]
pub unsafe fn UiaRemoveEvent<P0>(hevent: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAEVENT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaRemoveEvent(hevent : HUIAEVENT) -> windows_core::HRESULT);
    UiaRemoveEvent(hevent.param().abi()).ok()
}
#[inline]
pub unsafe fn UiaReturnRawElementProvider<P0, P1, P2, P3>(hwnd: P0, wparam: P1, lparam: P2, el: P3) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
    P3: windows_core::Param<IRawElementProviderSimple>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaReturnRawElementProvider(hwnd : super::super::Foundation:: HWND, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, el : * mut core::ffi::c_void) -> super::super::Foundation:: LRESULT);
    UiaReturnRawElementProvider(hwnd.param().abi(), wparam.param().abi(), lparam.param().abi(), el.param().abi())
}
#[inline]
pub unsafe fn UiaSetFocus<P0>(hnode: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIANODE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaSetFocus(hnode : HUIANODE) -> windows_core::HRESULT);
    UiaSetFocus(hnode.param().abi()).ok()
}
#[inline]
pub unsafe fn UiaTextRangeRelease<P0>(hobj: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HUIATEXTRANGE>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn UiaTextRangeRelease(hobj : HUIATEXTRANGE) -> super::super::Foundation:: BOOL);
    UiaTextRangeRelease(hobj.param().abi())
}
#[inline]
pub unsafe fn UnhookWinEvent<P0>(hwineventhook: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HWINEVENTHOOK>,
{
    windows_targets::link!("user32.dll" "system" fn UnhookWinEvent(hwineventhook : HWINEVENTHOOK) -> super::super::Foundation:: BOOL);
    UnhookWinEvent(hwineventhook.param().abi())
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn UnregisterPointerInputTarget<P0>(hwnd: P0, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn UnregisterPointerInputTarget(hwnd : super::super::Foundation:: HWND, pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE) -> super::super::Foundation:: BOOL);
    UnregisterPointerInputTarget(hwnd.param().abi(), pointertype).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn UnregisterPointerInputTargetEx<P0>(hwnd: P0, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn UnregisterPointerInputTargetEx(hwnd : super::super::Foundation:: HWND, pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE) -> super::super::Foundation:: BOOL);
    UnregisterPointerInputTargetEx(hwnd.param().abi(), pointertype)
}
#[inline]
pub unsafe fn ValuePattern_SetValue<P0, P1>(hobj: P0, pval: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn ValuePattern_SetValue(hobj : HUIAPATTERNOBJECT, pval : windows_core::PCWSTR) -> windows_core::HRESULT);
    ValuePattern_SetValue(hobj.param().abi(), pval.param().abi()).ok()
}
#[inline]
pub unsafe fn VirtualizedItemPattern_Realize<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn VirtualizedItemPattern_Realize(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    VirtualizedItemPattern_Realize(hobj.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WindowFromAccessibleObject<P0>(param0: P0, phwnd: Option<*mut super::super::Foundation::HWND>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IAccessible>,
{
    windows_targets::link!("oleacc.dll" "system" fn WindowFromAccessibleObject(param0 : * mut core::ffi::c_void, phwnd : *mut super::super::Foundation:: HWND) -> windows_core::HRESULT);
    WindowFromAccessibleObject(param0.param().abi(), core::mem::transmute(phwnd.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WindowPattern_Close<P0>(hobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn WindowPattern_Close(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    WindowPattern_Close(hobj.param().abi()).ok()
}
#[inline]
pub unsafe fn WindowPattern_SetWindowVisualState<P0>(hobj: P0, state: WindowVisualState) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn WindowPattern_SetWindowVisualState(hobj : HUIAPATTERNOBJECT, state : WindowVisualState) -> windows_core::HRESULT);
    WindowPattern_SetWindowVisualState(hobj.param().abi(), state).ok()
}
#[inline]
pub unsafe fn WindowPattern_WaitForInputIdle<P0>(hobj: P0, milliseconds: i32, presult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<HUIAPATTERNOBJECT>,
{
    windows_targets::link!("uiautomationcore.dll" "system" fn WindowPattern_WaitForInputIdle(hobj : HUIAPATTERNOBJECT, milliseconds : i32, presult : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    WindowPattern_WaitForInputIdle(hobj.param().abi(), milliseconds, presult).ok()
}
windows_core::imp::define_interface!(IAccIdentity, IAccIdentity_Vtbl, 0x7852b78d_1cfd_41c1_a615_9c0c85960b5f);
impl core::ops::Deref for IAccIdentity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccIdentity, windows_core::IUnknown);
impl IAccIdentity {
    pub unsafe fn GetIdentityString(&self, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdentityString)(windows_core::Interface::as_raw(self), dwidchild, ppidstring, pdwidstringlen).ok()
    }
}
#[repr(C)]
pub struct IAccIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccPropServer, IAccPropServer_Vtbl, 0x76c0dbbb_15e0_4e7b_b61b_20eeea2001e0);
impl core::ops::Deref for IAccPropServer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccPropServer, windows_core::IUnknown);
impl IAccPropServer {
    pub unsafe fn GetPropValue(&self, pidstring: &[u8], idprop: windows_core::GUID, pvarvalue: *mut windows_core::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pidstring.as_ptr()), pidstring.len().try_into().unwrap(), core::mem::transmute(idprop), core::mem::transmute(pvarvalue), pfhasprop).ok()
    }
}
#[repr(C)]
pub struct IAccPropServer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccPropServices, IAccPropServices_Vtbl, 0x6e26e776_04f0_495d_80e4_3330352e3169);
impl core::ops::Deref for IAccPropServices {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccPropServices, windows_core::IUnknown);
impl IAccPropServices {
    pub unsafe fn SetPropValue<P0>(&self, pidstring: &[u8], idprop: windows_core::GUID, var: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetPropValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pidstring.as_ptr()), pidstring.len().try_into().unwrap(), core::mem::transmute(idprop), var.param().abi()).ok()
    }
    pub unsafe fn SetPropServer<P0>(&self, pidstring: &[u8], paprops: &[windows_core::GUID], pserver: P0, annoscope: AnnoScope) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAccPropServer>,
    {
        (windows_core::Interface::vtable(self).SetPropServer)(windows_core::Interface::as_raw(self), core::mem::transmute(pidstring.as_ptr()), pidstring.len().try_into().unwrap(), core::mem::transmute(paprops.as_ptr()), paprops.len().try_into().unwrap(), pserver.param().abi(), annoscope).ok()
    }
    pub unsafe fn ClearProps(&self, pidstring: &[u8], paprops: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearProps)(windows_core::Interface::as_raw(self), core::mem::transmute(pidstring.as_ptr()), pidstring.len().try_into().unwrap(), core::mem::transmute(paprops.as_ptr()), paprops.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetHwndProp<P0, P1>(&self, hwnd: P0, idobject: u32, idchild: u32, idprop: windows_core::GUID, var: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetHwndProp)(windows_core::Interface::as_raw(self), hwnd.param().abi(), idobject, idchild, core::mem::transmute(idprop), var.param().abi()).ok()
    }
    pub unsafe fn SetHwndPropStr<P0, P1>(&self, hwnd: P0, idobject: u32, idchild: u32, idprop: windows_core::GUID, str: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHwndPropStr)(windows_core::Interface::as_raw(self), hwnd.param().abi(), idobject, idchild, core::mem::transmute(idprop), str.param().abi()).ok()
    }
    pub unsafe fn SetHwndPropServer<P0, P1>(&self, hwnd: P0, idobject: u32, idchild: u32, paprops: &[windows_core::GUID], pserver: P1, annoscope: AnnoScope) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<IAccPropServer>,
    {
        (windows_core::Interface::vtable(self).SetHwndPropServer)(windows_core::Interface::as_raw(self), hwnd.param().abi(), idobject, idchild, core::mem::transmute(paprops.as_ptr()), paprops.len().try_into().unwrap(), pserver.param().abi(), annoscope).ok()
    }
    pub unsafe fn ClearHwndProps<P0>(&self, hwnd: P0, idobject: u32, idchild: u32, paprops: &[windows_core::GUID]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ClearHwndProps)(windows_core::Interface::as_raw(self), hwnd.param().abi(), idobject, idchild, core::mem::transmute(paprops.as_ptr()), paprops.len().try_into().unwrap()).ok()
    }
    pub unsafe fn ComposeHwndIdentityString<P0>(&self, hwnd: P0, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ComposeHwndIdentityString)(windows_core::Interface::as_raw(self), hwnd.param().abi(), idobject, idchild, ppidstring, pdwidstringlen).ok()
    }
    pub unsafe fn DecomposeHwndIdentityString(&self, pidstring: &[u8], phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DecomposeHwndIdentityString)(windows_core::Interface::as_raw(self), core::mem::transmute(pidstring.as_ptr()), pidstring.len().try_into().unwrap(), phwnd, pidobject, pidchild).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetHmenuProp<P0, P1>(&self, hmenu: P0, idchild: u32, idprop: windows_core::GUID, var: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::WindowsAndMessaging::HMENU>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetHmenuProp)(windows_core::Interface::as_raw(self), hmenu.param().abi(), idchild, core::mem::transmute(idprop), var.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetHmenuPropStr<P0, P1>(&self, hmenu: P0, idchild: u32, idprop: windows_core::GUID, str: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::WindowsAndMessaging::HMENU>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHmenuPropStr)(windows_core::Interface::as_raw(self), hmenu.param().abi(), idchild, core::mem::transmute(idprop), str.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetHmenuPropServer<P0, P1>(&self, hmenu: P0, idchild: u32, paprops: &[windows_core::GUID], pserver: P1, annoscope: AnnoScope) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::WindowsAndMessaging::HMENU>,
        P1: windows_core::Param<IAccPropServer>,
    {
        (windows_core::Interface::vtable(self).SetHmenuPropServer)(windows_core::Interface::as_raw(self), hmenu.param().abi(), idchild, core::mem::transmute(paprops.as_ptr()), paprops.len().try_into().unwrap(), pserver.param().abi(), annoscope).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ClearHmenuProps<P0>(&self, hmenu: P0, idchild: u32, paprops: &[windows_core::GUID]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::WindowsAndMessaging::HMENU>,
    {
        (windows_core::Interface::vtable(self).ClearHmenuProps)(windows_core::Interface::as_raw(self), hmenu.param().abi(), idchild, core::mem::transmute(paprops.as_ptr()), paprops.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ComposeHmenuIdentityString<P0>(&self, hmenu: P0, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::WindowsAndMessaging::HMENU>,
    {
        (windows_core::Interface::vtable(self).ComposeHmenuIdentityString)(windows_core::Interface::as_raw(self), hmenu.param().abi(), idchild, ppidstring, pdwidstringlen).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn DecomposeHmenuIdentityString(&self, pidstring: &[u8], phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DecomposeHmenuIdentityString)(windows_core::Interface::as_raw(self), core::mem::transmute(pidstring.as_ptr()), pidstring.len().try_into().unwrap(), phmenu, pidchild).ok()
    }
}
#[repr(C)]
pub struct IAccPropServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPropValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, windows_core::GUID, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetPropServer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const windows_core::GUID, i32, *mut core::ffi::c_void, AnnoScope) -> windows_core::HRESULT,
    pub ClearProps: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const windows_core::GUID, i32) -> windows_core::HRESULT,
    pub SetHwndProp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, u32, windows_core::GUID, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetHwndPropStr: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, u32, windows_core::GUID, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHwndPropServer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, u32, *const windows_core::GUID, i32, *mut core::ffi::c_void, AnnoScope) -> windows_core::HRESULT,
    pub ClearHwndProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, u32, *const windows_core::GUID, i32) -> windows_core::HRESULT,
    pub ComposeHwndIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub DecomposeHwndIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut super::super::Foundation::HWND, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetHmenuProp: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowsAndMessaging::HMENU, u32, windows_core::GUID, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetHmenuProp: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetHmenuPropStr: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowsAndMessaging::HMENU, u32, windows_core::GUID, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetHmenuPropStr: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetHmenuPropServer: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowsAndMessaging::HMENU, u32, *const windows_core::GUID, i32, *mut core::ffi::c_void, AnnoScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetHmenuPropServer: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub ClearHmenuProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowsAndMessaging::HMENU, u32, *const windows_core::GUID, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    ClearHmenuProps: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub ComposeHmenuIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowsAndMessaging::HMENU, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    ComposeHmenuIdentityString: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub DecomposeHmenuIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut super::WindowsAndMessaging::HMENU, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    DecomposeHmenuIdentityString: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAccessible, IAccessible_Vtbl, 0x618736e0_3c3d_11cf_810c_00aa00389b71);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAccessible {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAccessible, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAccessible {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn accParent(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).accParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn accChildCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).accChildCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_accChild<P0>(&self, varchild: P0) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accChild)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_accName<P0>(&self, varchild: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accName)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_accValue<P0>(&self, varchild: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accValue)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_accDescription<P0>(&self, varchild: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accDescription)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_accRole<P0>(&self, varchild: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accRole)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_accState<P0>(&self, varchild: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accState)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_accHelp<P0>(&self, varchild: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accHelp)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_accHelpTopic<P0>(&self, pszhelpfile: *mut windows_core::BSTR, varchild: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accHelpTopic)(windows_core::Interface::as_raw(self), core::mem::transmute(pszhelpfile), varchild.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn get_accKeyboardShortcut<P0>(&self, varchild: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accKeyboardShortcut)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn accFocus(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).accFocus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn accSelection(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).accSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_accDefaultAction<P0>(&self, varchild: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_accDefaultAction)(windows_core::Interface::as_raw(self), varchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn accSelect<P0>(&self, flagsselect: i32, varchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).accSelect)(windows_core::Interface::as_raw(self), flagsselect, varchild.param().abi()).ok()
    }
    pub unsafe fn accLocation<P0>(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).accLocation)(windows_core::Interface::as_raw(self), pxleft, pytop, pcxwidth, pcyheight, varchild.param().abi()).ok()
    }
    pub unsafe fn accNavigate<P0>(&self, navdir: i32, varstart: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).accNavigate)(windows_core::Interface::as_raw(self), navdir, varstart.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn accHitTest(&self, xleft: i32, ytop: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).accHitTest)(windows_core::Interface::as_raw(self), xleft, ytop, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn accDoDefaultAction<P0>(&self, varchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).accDoDefaultAction)(windows_core::Interface::as_raw(self), varchild.param().abi()).ok()
    }
    pub unsafe fn put_accName<P0, P1>(&self, varchild: P0, szname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_accName)(windows_core::Interface::as_raw(self), varchild.param().abi(), szname.param().abi()).ok()
    }
    pub unsafe fn put_accValue<P0, P1>(&self, varchild: P0, szvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_accValue)(windows_core::Interface::as_raw(self), varchild.param().abi(), szvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAccessible_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub accParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    accParent: usize,
    pub accChildCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_accChild: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_accChild: usize,
    pub get_accName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_accValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_accDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_accRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub get_accState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub get_accHelp: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_accHelpTopic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut i32) -> windows_core::HRESULT,
    pub get_accKeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub accFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub accSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub get_accDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub accSelect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub accLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub accNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub accHitTest: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub accDoDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub put_accName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_accValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccessibleEx, IAccessibleEx_Vtbl, 0xf8b80ada_2c44_48d0_89be_5ff23c9cd875);
impl core::ops::Deref for IAccessibleEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccessibleEx, windows_core::IUnknown);
impl IAccessibleEx {
    pub unsafe fn GetObjectForChild(&self, idchild: i32) -> windows_core::Result<IAccessibleEx> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectForChild)(windows_core::Interface::as_raw(self), idchild, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetIAccessiblePair(&self, ppacc: *mut Option<IAccessible>, pidchild: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIAccessiblePair)(windows_core::Interface::as_raw(self), core::mem::transmute(ppacc), pidchild).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRuntimeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ConvertReturnedElement<P0>(&self, pin: P0) -> windows_core::Result<IAccessibleEx>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConvertReturnedElement)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAccessibleEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectForChild: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetIAccessiblePair: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetIAccessiblePair: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRuntimeId: usize,
    pub ConvertReturnedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccessibleHandler, IAccessibleHandler_Vtbl, 0x03022430_abc4_11d0_bde2_00aa001a1953);
impl core::ops::Deref for IAccessibleHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccessibleHandler, windows_core::IUnknown);
impl IAccessibleHandler {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AccessibleObjectFromID(&self, hwnd: i32, lobjectid: i32) -> windows_core::Result<IAccessible> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AccessibleObjectFromID)(windows_core::Interface::as_raw(self), hwnd, lobjectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAccessibleHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AccessibleObjectFromID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AccessibleObjectFromID: usize,
}
windows_core::imp::define_interface!(IAccessibleHostingElementProviders, IAccessibleHostingElementProviders_Vtbl, 0x33ac331b_943e_4020_b295_db37784974a3);
impl core::ops::Deref for IAccessibleHostingElementProviders {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccessibleHostingElementProviders, windows_core::IUnknown);
impl IAccessibleHostingElementProviders {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEmbeddedFragmentRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetObjectIdForProvider<P0>(&self, pprovider: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectIdForProvider)(windows_core::Interface::as_raw(self), pprovider.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAccessibleHostingElementProviders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEmbeddedFragmentRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEmbeddedFragmentRoots: usize,
    pub GetObjectIdForProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccessibleWindowlessSite, IAccessibleWindowlessSite_Vtbl, 0xbf3abd9c_76da_4389_9eb6_1427d25abab7);
impl core::ops::Deref for IAccessibleWindowlessSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccessibleWindowlessSite, windows_core::IUnknown);
impl IAccessibleWindowlessSite {
    pub unsafe fn AcquireObjectIdRange<P0>(&self, rangesize: i32, prangeowner: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IAccessibleHandler>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AcquireObjectIdRange)(windows_core::Interface::as_raw(self), rangesize, prangeowner.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ReleaseObjectIdRange<P0>(&self, rangebase: i32, prangeowner: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAccessibleHandler>,
    {
        (windows_core::Interface::vtable(self).ReleaseObjectIdRange)(windows_core::Interface::as_raw(self), rangebase, prangeowner.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryObjectIdRanges<P0>(&self, prangesowner: P0) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>
    where
        P0: windows_core::Param<IAccessibleHandler>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryObjectIdRanges)(windows_core::Interface::as_raw(self), prangesowner.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetParentAccessible(&self) -> windows_core::Result<IAccessible> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParentAccessible)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAccessibleWindowlessSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AcquireObjectIdRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReleaseObjectIdRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryObjectIdRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryObjectIdRanges: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParentAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParentAccessible: usize,
}
windows_core::imp::define_interface!(IAnnotationProvider, IAnnotationProvider_Vtbl, 0xf95c7e80_bd63_4601_9782_445ebff011fc);
impl core::ops::Deref for IAnnotationProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAnnotationProvider, windows_core::IUnknown);
impl IAnnotationProvider {
    pub unsafe fn AnnotationTypeId(&self) -> windows_core::Result<UIA_ANNOTATIONTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Author(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Author)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DateTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Target(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Target)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAnnotationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_ANNOTATIONTYPE) -> windows_core::HRESULT,
    pub AnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Target: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICustomNavigationProvider, ICustomNavigationProvider_Vtbl, 0x2062a28a_8c07_4b94_8e12_7037c622aeb8);
impl core::ops::Deref for ICustomNavigationProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICustomNavigationProvider, windows_core::IUnknown);
impl ICustomNavigationProvider {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICustomNavigationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDockProvider, IDockProvider_Vtbl, 0x159bc72c_4ad3_485e_9637_d7052edf0146);
impl core::ops::Deref for IDockProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDockProvider, windows_core::IUnknown);
impl IDockProvider {
    pub unsafe fn SetDockPosition(&self, dockposition: DockPosition) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDockPosition)(windows_core::Interface::as_raw(self), dockposition).ok()
    }
    pub unsafe fn DockPosition(&self) -> windows_core::Result<DockPosition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDockProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, DockPosition) -> windows_core::HRESULT,
    pub DockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DockPosition) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDragProvider, IDragProvider_Vtbl, 0x6aa7bbbb_7ff9_497d_904f_d20b897929d8);
impl core::ops::Deref for IDragProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDragProvider, windows_core::IUnknown);
impl IDragProvider {
    pub unsafe fn IsGrabbed(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DropEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DropEffects(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGrabbedItems(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGrabbedItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDragProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DropEffects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGrabbedItems: usize,
}
windows_core::imp::define_interface!(IDropTargetProvider, IDropTargetProvider_Vtbl, 0xbae82bfd_358a_481c_85a0_d8b4d90a5d61);
impl core::ops::Deref for IDropTargetProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDropTargetProvider, windows_core::IUnknown);
impl IDropTargetProvider {
    pub unsafe fn DropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DropTargetEffects(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDropTargetProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DropTargetEffects: usize,
}
windows_core::imp::define_interface!(IExpandCollapseProvider, IExpandCollapseProvider_Vtbl, 0xd847d3a5_cab0_4a98_8c32_ecb45c59ad24);
impl core::ops::Deref for IExpandCollapseProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExpandCollapseProvider, windows_core::IUnknown);
impl IExpandCollapseProvider {
    pub unsafe fn Expand(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Collapse(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Collapse)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExpandCollapseState(&self) -> windows_core::Result<ExpandCollapseState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IExpandCollapseProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExpandCollapseState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGridItemProvider, IGridItemProvider_Vtbl, 0xd02541f1_fb81_4d64_ae32_f520f8a6dbd1);
impl core::ops::Deref for IGridItemProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGridItemProvider, windows_core::IUnknown);
impl IGridItemProvider {
    pub unsafe fn Row(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Row)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Column(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Column)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RowSpan(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ColumnSpan(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ContainingGrid(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IGridItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Row: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Column: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGridProvider, IGridProvider_Vtbl, 0xb17d6187_0907_464b_a168_0ef17a1572b1);
impl core::ops::Deref for IGridProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGridProvider, windows_core::IUnknown);
impl IGridProvider {
    pub unsafe fn GetItem(&self, row: i32, column: i32) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), row, column, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RowCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ColumnCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGridProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInvokeProvider, IInvokeProvider_Vtbl, 0x54fcb24b_e18e_47a2_b4d3_eccbe77599a2);
impl core::ops::Deref for IInvokeProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInvokeProvider, windows_core::IUnknown);
impl IInvokeProvider {
    pub unsafe fn Invoke(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInvokeProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IItemContainerProvider, IItemContainerProvider_Vtbl, 0xe747770b_39ce_4382_ab30_d8fb3f336f24);
impl core::ops::Deref for IItemContainerProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IItemContainerProvider, windows_core::IUnknown);
impl IItemContainerProvider {
    pub unsafe fn FindItemByProperty<P0, P1>(&self, pstartafter: P0, propertyid: UIA_PROPERTY_ID, value: P1) -> windows_core::Result<IRawElementProviderSimple>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindItemByProperty)(windows_core::Interface::as_raw(self), pstartafter.param().abi(), propertyid, value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IItemContainerProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindItemByProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UIA_PROPERTY_ID, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILegacyIAccessibleProvider, ILegacyIAccessibleProvider_Vtbl, 0xe44c3566_915d_4070_99c6_047bff5a08f5);
impl core::ops::Deref for ILegacyIAccessibleProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILegacyIAccessibleProvider, windows_core::IUnknown);
impl ILegacyIAccessibleProvider {
    pub unsafe fn Select(&self, flagsselect: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self), flagsselect).ok()
    }
    pub unsafe fn DoDefaultAction(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoDefaultAction)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetValue<P0>(&self, szvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), szvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetIAccessible(&self) -> windows_core::Result<IAccessible> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIAccessible)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ChildId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Role(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Role)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn State(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Help(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Help)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn KeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ILegacyIAccessibleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DoDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetIAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetIAccessible: usize,
    pub ChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Help: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub KeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection: usize,
    pub DefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultipleViewProvider, IMultipleViewProvider_Vtbl, 0x6278cab1_b556_4a1a_b4e0_418acc523201);
impl core::ops::Deref for IMultipleViewProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultipleViewProvider, windows_core::IUnknown);
impl IMultipleViewProvider {
    pub unsafe fn GetViewName(&self, viewid: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewName)(windows_core::Interface::as_raw(self), viewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCurrentView(&self, viewid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCurrentView)(windows_core::Interface::as_raw(self), viewid).ok()
    }
    pub unsafe fn CurrentView(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSupportedViews(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSupportedViews)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMultipleViewProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetViewName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSupportedViews: usize,
}
windows_core::imp::define_interface!(IObjectModelProvider, IObjectModelProvider_Vtbl, 0x3ad86ebd_f5ef_483d_bb18_b1042a475d64);
impl core::ops::Deref for IObjectModelProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectModelProvider, windows_core::IUnknown);
impl IObjectModelProvider {
    pub unsafe fn GetUnderlyingObjectModel(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUnderlyingObjectModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IObjectModelProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProxyProviderWinEventHandler, IProxyProviderWinEventHandler_Vtbl, 0x89592ad4_f4e0_43d5_a3b6_bad7e111b435);
impl core::ops::Deref for IProxyProviderWinEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProxyProviderWinEventHandler, windows_core::IUnknown);
impl IProxyProviderWinEventHandler {
    pub unsafe fn RespondToWinEvent<P0, P1>(&self, idwinevent: u32, hwnd: P0, idobject: i32, idchild: i32, psink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<IProxyProviderWinEventSink>,
    {
        (windows_core::Interface::vtable(self).RespondToWinEvent)(windows_core::Interface::as_raw(self), idwinevent, hwnd.param().abi(), idobject, idchild, psink.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IProxyProviderWinEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RespondToWinEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HWND, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProxyProviderWinEventSink, IProxyProviderWinEventSink_Vtbl, 0x4fd82b78_a43e_46ac_9803_0a6969c7c183);
impl core::ops::Deref for IProxyProviderWinEventSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProxyProviderWinEventSink, windows_core::IUnknown);
impl IProxyProviderWinEventSink {
    pub unsafe fn AddAutomationPropertyChangedEvent<P0, P1>(&self, pprovider: P0, id: UIA_PROPERTY_ID, newvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddAutomationPropertyChangedEvent)(windows_core::Interface::as_raw(self), pprovider.param().abi(), id, newvalue.param().abi()).ok()
    }
    pub unsafe fn AddAutomationEvent<P0>(&self, pprovider: P0, id: UIA_EVENT_ID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        (windows_core::Interface::vtable(self).AddAutomationEvent)(windows_core::Interface::as_raw(self), pprovider.param().abi(), id).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddStructureChangedEvent<P0>(&self, pprovider: P0, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        (windows_core::Interface::vtable(self).AddStructureChangedEvent)(windows_core::Interface::as_raw(self), pprovider.param().abi(), structurechangetype, runtimeid).ok()
    }
}
#[repr(C)]
pub struct IProxyProviderWinEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAutomationPropertyChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UIA_PROPERTY_ID, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UIA_EVENT_ID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddStructureChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, StructureChangeType, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddStructureChangedEvent: usize,
}
windows_core::imp::define_interface!(IRangeValueProvider, IRangeValueProvider_Vtbl, 0x36dc7aef_33e6_4691_afe1_2be7274b3d33);
impl core::ops::Deref for IRangeValueProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRangeValueProvider, windows_core::IUnknown);
impl IRangeValueProvider {
    pub unsafe fn SetValue(&self, val: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsReadOnly(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Maximum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Maximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Minimum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Minimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LargeChange(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SmallChange(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRangeValueProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Maximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Minimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawElementProviderAdviseEvents, IRawElementProviderAdviseEvents_Vtbl, 0xa407b27b_0f6d_4427_9292_473c7bf93258);
impl core::ops::Deref for IRawElementProviderAdviseEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderAdviseEvents, windows_core::IUnknown);
impl IRawElementProviderAdviseEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdviseEventAdded(&self, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AdviseEventAdded)(windows_core::Interface::as_raw(self), eventid, propertyids).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdviseEventRemoved(&self, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AdviseEventRemoved)(windows_core::Interface::as_raw(self), eventid, propertyids).ok()
    }
}
#[repr(C)]
pub struct IRawElementProviderAdviseEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AdviseEventAdded: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_EVENT_ID, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdviseEventAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdviseEventRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_EVENT_ID, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdviseEventRemoved: usize,
}
windows_core::imp::define_interface!(IRawElementProviderFragment, IRawElementProviderFragment_Vtbl, 0xf7063da8_8359_439c_9297_bbc5299a7d87);
impl core::ops::Deref for IRawElementProviderFragment {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderFragment, windows_core::IUnknown);
impl IRawElementProviderFragment {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderFragment> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRuntimeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BoundingRectangle(&self) -> windows_core::Result<UiaRect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEmbeddedFragmentRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFocus(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FragmentRoot(&self) -> windows_core::Result<IRawElementProviderFragmentRoot> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FragmentRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRawElementProviderFragment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRuntimeId: usize,
    pub BoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UiaRect) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEmbeddedFragmentRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEmbeddedFragmentRoots: usize,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FragmentRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawElementProviderFragmentRoot, IRawElementProviderFragmentRoot_Vtbl, 0x620ce2a5_ab8f_40a9_86cb_de3c75599b58);
impl core::ops::Deref for IRawElementProviderFragmentRoot {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderFragmentRoot, windows_core::IUnknown);
impl IRawElementProviderFragmentRoot {
    pub unsafe fn ElementProviderFromPoint(&self, x: f64, y: f64) -> windows_core::Result<IRawElementProviderFragment> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElementProviderFromPoint)(windows_core::Interface::as_raw(self), x, y, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFocus(&self) -> windows_core::Result<IRawElementProviderFragment> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRawElementProviderFragmentRoot_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ElementProviderFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawElementProviderHostingAccessibles, IRawElementProviderHostingAccessibles_Vtbl, 0x24be0b07_d37d_487a_98cf_a13ed465e9b3);
impl core::ops::Deref for IRawElementProviderHostingAccessibles {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderHostingAccessibles, windows_core::IUnknown);
impl IRawElementProviderHostingAccessibles {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEmbeddedAccessibles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEmbeddedAccessibles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRawElementProviderHostingAccessibles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEmbeddedAccessibles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEmbeddedAccessibles: usize,
}
windows_core::imp::define_interface!(IRawElementProviderHwndOverride, IRawElementProviderHwndOverride_Vtbl, 0x1d5df27c_8947_4425_b8d9_79787bb460b8);
impl core::ops::Deref for IRawElementProviderHwndOverride {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderHwndOverride, windows_core::IUnknown);
impl IRawElementProviderHwndOverride {
    pub unsafe fn GetOverrideProviderForHwnd<P0>(&self, hwnd: P0) -> windows_core::Result<IRawElementProviderSimple>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOverrideProviderForHwnd)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRawElementProviderHwndOverride_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOverrideProviderForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawElementProviderSimple, IRawElementProviderSimple_Vtbl, 0xd6dd68d1_86fd_4332_8666_9abedea2d24c);
impl core::ops::Deref for IRawElementProviderSimple {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderSimple, windows_core::IUnknown);
impl IRawElementProviderSimple {
    pub unsafe fn ProviderOptions(&self) -> windows_core::Result<ProviderOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPatternProvider(&self, patternid: UIA_PATTERN_ID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPatternProvider)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HostRawElementProvider(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HostRawElementProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRawElementProviderSimple_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProviderOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderOptions) -> windows_core::HRESULT,
    pub GetPatternProvider: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PATTERN_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub HostRawElementProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawElementProviderSimple2, IRawElementProviderSimple2_Vtbl, 0xa0a839a9_8da1_4a82_806a_8e0d44e79f56);
impl core::ops::Deref for IRawElementProviderSimple2 {
    type Target = IRawElementProviderSimple;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderSimple2, windows_core::IUnknown, IRawElementProviderSimple);
impl IRawElementProviderSimple2 {
    pub unsafe fn ShowContextMenu(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRawElementProviderSimple2_Vtbl {
    pub base__: IRawElementProviderSimple_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawElementProviderSimple3, IRawElementProviderSimple3_Vtbl, 0xfcf5d820_d7ec_4613_bdf6_42a84ce7daaf);
impl core::ops::Deref for IRawElementProviderSimple3 {
    type Target = IRawElementProviderSimple2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderSimple3, windows_core::IUnknown, IRawElementProviderSimple, IRawElementProviderSimple2);
impl IRawElementProviderSimple3 {
    pub unsafe fn GetMetadataValue(&self, targetid: i32, metadataid: UIA_METADATA_ID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataValue)(windows_core::Interface::as_raw(self), targetid, metadataid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRawElementProviderSimple3_Vtbl {
    pub base__: IRawElementProviderSimple2_Vtbl,
    pub GetMetadataValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, UIA_METADATA_ID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawElementProviderWindowlessSite, IRawElementProviderWindowlessSite_Vtbl, 0x0a2a93cc_bfad_42ac_9b2e_0991fb0d3ea0);
impl core::ops::Deref for IRawElementProviderWindowlessSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderWindowlessSite, windows_core::IUnknown);
impl IRawElementProviderWindowlessSite {
    pub unsafe fn GetAdjacentFragment(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderFragment> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAdjacentFragment)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeIdPrefix(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRuntimeIdPrefix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRawElementProviderWindowlessSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAdjacentFragment: unsafe extern "system" fn(*mut core::ffi::c_void, NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRuntimeIdPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRuntimeIdPrefix: usize,
}
windows_core::imp::define_interface!(IRichEditUiaInformation, IRichEditUiaInformation_Vtbl, 0);
impl core::ops::Deref for IRichEditUiaInformation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRichEditUiaInformation, windows_core::IUnknown);
impl IRichEditUiaInformation {
    pub unsafe fn GetBoundaryRectangle(&self, puiarect: *mut UiaRect) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBoundaryRectangle)(windows_core::Interface::as_raw(self), puiarect).ok()
    }
    pub unsafe fn IsVisible(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsVisible)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRichEditUiaInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBoundaryRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UiaRect) -> windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRicheditWindowlessAccessibility, IRicheditWindowlessAccessibility_Vtbl, 0);
impl core::ops::Deref for IRicheditWindowlessAccessibility {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRicheditWindowlessAccessibility, windows_core::IUnknown);
impl IRicheditWindowlessAccessibility {
    pub unsafe fn CreateProvider<P0>(&self, psite: P0) -> windows_core::Result<IRawElementProviderSimple>
    where
        P0: windows_core::Param<IRawElementProviderWindowlessSite>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateProvider)(windows_core::Interface::as_raw(self), psite.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRicheditWindowlessAccessibility_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScrollItemProvider, IScrollItemProvider_Vtbl, 0x2360c714_4bf1_4b26_ba65_9b21316127eb);
impl core::ops::Deref for IScrollItemProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScrollItemProvider, windows_core::IUnknown);
impl IScrollItemProvider {
    pub unsafe fn ScrollIntoView(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IScrollItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScrollProvider, IScrollProvider_Vtbl, 0xb38b8077_1fc3_42a5_8cae_d40c2215055a);
impl core::ops::Deref for IScrollProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScrollProvider, windows_core::IUnknown);
impl IScrollProvider {
    pub unsafe fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), horizontalamount, verticalamount).ok()
    }
    pub unsafe fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScrollPercent)(windows_core::Interface::as_raw(self), horizontalpercent, verticalpercent).ok()
    }
    pub unsafe fn HorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn VerticalScrollPercent(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn HorizontalViewSize(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn VerticalViewSize(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn HorizontallyScrollable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn VerticallyScrollable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IScrollProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, ScrollAmount, ScrollAmount) -> windows_core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub HorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub VerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub HorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub VerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub HorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub VerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISelectionItemProvider, ISelectionItemProvider_Vtbl, 0x2acad808_b2d4_452d_a407_91ff1ad167b2);
impl core::ops::Deref for ISelectionItemProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISelectionItemProvider, windows_core::IUnknown);
impl ISelectionItemProvider {
    pub unsafe fn Select(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddToSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveFromSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsSelected(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SelectionContainer(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISelectionItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISelectionProvider, ISelectionProvider_Vtbl, 0xfb8b03af_3bdf_48d4_bd36_1a65793be168);
impl core::ops::Deref for ISelectionProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISelectionProvider, windows_core::IUnknown);
impl ISelectionProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CanSelectMultiple(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSelectionRequired(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISelectionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection: usize,
    pub CanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISelectionProvider2, ISelectionProvider2_Vtbl, 0x14f68475_ee1c_44f6_a869_d239381f0fe7);
impl core::ops::Deref for ISelectionProvider2 {
    type Target = ISelectionProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISelectionProvider2, windows_core::IUnknown, ISelectionProvider);
impl ISelectionProvider2 {
    pub unsafe fn FirstSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ItemCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISelectionProvider2_Vtbl {
    pub base__: ISelectionProvider_Vtbl,
    pub FirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpreadsheetItemProvider, ISpreadsheetItemProvider_Vtbl, 0xeaed4660_7b3d_4879_a2e6_365ce603f3d0);
impl core::ops::Deref for ISpreadsheetItemProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpreadsheetItemProvider, windows_core::IUnknown);
impl ISpreadsheetItemProvider {
    pub unsafe fn Formula(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Formula)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAnnotationObjects(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAnnotationTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpreadsheetItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Formula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAnnotationObjects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAnnotationTypes: usize,
}
windows_core::imp::define_interface!(ISpreadsheetProvider, ISpreadsheetProvider_Vtbl, 0x6f6b5d35_5525_4f80_b758_85473832ffc7);
impl core::ops::Deref for ISpreadsheetProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpreadsheetProvider, windows_core::IUnknown);
impl ISpreadsheetProvider {
    pub unsafe fn GetItemByName<P0>(&self, name: P0) -> windows_core::Result<IRawElementProviderSimple>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemByName)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpreadsheetProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStylesProvider, IStylesProvider_Vtbl, 0x19b6b649_f5d7_4a6d_bdcb_129252be588a);
impl core::ops::Deref for IStylesProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStylesProvider, windows_core::IUnknown);
impl IStylesProvider {
    pub unsafe fn StyleId(&self) -> windows_core::Result<UIA_STYLE_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StyleName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FillColor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Shape(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Shape)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FillPatternColor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IStylesProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_STYLE_ID) -> windows_core::HRESULT,
    pub StyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Shape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISynchronizedInputProvider, ISynchronizedInputProvider_Vtbl, 0x29db1a06_02ce_4cf7_9b42_565d4fab20ee);
impl core::ops::Deref for ISynchronizedInputProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronizedInputProvider, windows_core::IUnknown);
impl ISynchronizedInputProvider {
    pub unsafe fn StartListening(&self, inputtype: SynchronizedInputType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartListening)(windows_core::Interface::as_raw(self), inputtype).ok()
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISynchronizedInputProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartListening: unsafe extern "system" fn(*mut core::ffi::c_void, SynchronizedInputType) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITableItemProvider, ITableItemProvider_Vtbl, 0xb9734fa6_771f_4d78_9c90_2517999349cd);
impl core::ops::Deref for ITableItemProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableItemProvider, windows_core::IUnknown);
impl ITableItemProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRowHeaderItems(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetColumnHeaderItems(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITableItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRowHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRowHeaderItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetColumnHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetColumnHeaderItems: usize,
}
windows_core::imp::define_interface!(ITableProvider, ITableProvider_Vtbl, 0x9c860395_97b3_490a_b52a_858cc22af166);
impl core::ops::Deref for ITableProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableProvider, windows_core::IUnknown);
impl ITableProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRowHeaders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetColumnHeaders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RowOrColumnMajor(&self) -> windows_core::Result<RowOrColumnMajor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITableProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRowHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRowHeaders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetColumnHeaders: usize,
    pub RowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RowOrColumnMajor) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextChildProvider, ITextChildProvider_Vtbl, 0x4c2de2b9_c88f_4f88_a111_f1d336b7d1a9);
impl core::ops::Deref for ITextChildProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextChildProvider, windows_core::IUnknown);
impl ITextChildProvider {
    pub unsafe fn TextContainer(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TextContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TextRange(&self) -> windows_core::Result<ITextRangeProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TextRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITextChildProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TextContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TextRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextEditProvider, ITextEditProvider_Vtbl, 0xea3605b4_3a05_400e_b5f9_4e91b40f6176);
impl core::ops::Deref for ITextEditProvider {
    type Target = ITextProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextEditProvider, windows_core::IUnknown, ITextProvider);
impl ITextEditProvider {
    pub unsafe fn GetActiveComposition(&self) -> windows_core::Result<ITextRangeProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveComposition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConversionTarget(&self) -> windows_core::Result<ITextRangeProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConversionTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITextEditProvider_Vtbl {
    pub base__: ITextProvider_Vtbl,
    pub GetActiveComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextProvider, ITextProvider_Vtbl, 0x3589c92c_63f3_4367_99bb_ada653b77cf2);
impl core::ops::Deref for ITextProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextProvider, windows_core::IUnknown);
impl ITextProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleRanges(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisibleRanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RangeFromChild<P0>(&self, childelement: P0) -> windows_core::Result<ITextRangeProvider>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RangeFromChild)(windows_core::Interface::as_raw(self), childelement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RangeFromPoint(&self, point: UiaPoint) -> windows_core::Result<ITextRangeProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RangeFromPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(point), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DocumentRange(&self) -> windows_core::Result<ITextRangeProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DocumentRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SupportedTextSelection(&self) -> windows_core::Result<SupportedTextSelection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedTextSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITextProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVisibleRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVisibleRanges: usize,
    pub RangeFromChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, UiaPoint, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportedTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SupportedTextSelection) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextProvider2, ITextProvider2_Vtbl, 0x0dc5e6ed_3e16_4bf1_8f9a_a979878bc195);
impl core::ops::Deref for ITextProvider2 {
    type Target = ITextProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextProvider2, windows_core::IUnknown, ITextProvider);
impl ITextProvider2 {
    pub unsafe fn RangeFromAnnotation<P0>(&self, annotationelement: P0) -> windows_core::Result<ITextRangeProvider>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RangeFromAnnotation)(windows_core::Interface::as_raw(self), annotationelement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCaretRange(&self, isactive: *mut super::super::Foundation::BOOL) -> windows_core::Result<ITextRangeProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCaretRange)(windows_core::Interface::as_raw(self), isactive, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITextProvider2_Vtbl {
    pub base__: ITextProvider_Vtbl,
    pub RangeFromAnnotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaretRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextRangeProvider, ITextRangeProvider_Vtbl, 0x5347ad7b_c355_46f8_aff5_909033582f63);
impl core::ops::Deref for ITextRangeProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextRangeProvider, windows_core::IUnknown);
impl ITextRangeProvider {
    pub unsafe fn Clone(&self) -> windows_core::Result<ITextRangeProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Compare<P0>(&self, range: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITextRangeProvider>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), range.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CompareEndpoints<P0>(&self, endpoint: TextPatternRangeEndpoint, targetrange: P0, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextRangeProvider>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareEndpoints)(windows_core::Interface::as_raw(self), endpoint, targetrange.param().abi(), targetendpoint, &mut result__).map(|| result__)
    }
    pub unsafe fn ExpandToEnclosingUnit(&self, unit: TextUnit) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExpandToEnclosingUnit)(windows_core::Interface::as_raw(self), unit).ok()
    }
    pub unsafe fn FindAttribute<P0, P1>(&self, attributeid: UIA_TEXTATTRIBUTE_ID, val: P0, backward: P1) -> windows_core::Result<ITextRangeProvider>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindAttribute)(windows_core::Interface::as_raw(self), attributeid, val.param().abi(), backward.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindText<P0, P1, P2>(&self, text: P0, backward: P1, ignorecase: P2) -> windows_core::Result<ITextRangeProvider>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindText)(windows_core::Interface::as_raw(self), text.param().abi(), backward.param().abi(), ignorecase.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAttributeValue(&self, attributeid: UIA_TEXTATTRIBUTE_ID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttributeValue)(windows_core::Interface::as_raw(self), attributeid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBoundingRectangles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEnclosingElement(&self) -> windows_core::Result<IRawElementProviderSimple> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnclosingElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), maxlength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Move(&self, unit: TextUnit, count: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
    }
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveEndpointByUnit)(windows_core::Interface::as_raw(self), endpoint, unit, count, &mut result__).map(|| result__)
    }
    pub unsafe fn MoveEndpointByRange<P0>(&self, endpoint: TextPatternRangeEndpoint, targetrange: P0, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRangeProvider>,
    {
        (windows_core::Interface::vtable(self).MoveEndpointByRange)(windows_core::Interface::as_raw(self), endpoint, targetrange.param().abi(), targetendpoint).ok()
    }
    pub unsafe fn Select(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddToSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveFromSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ScrollIntoView<P0>(&self, aligntotop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self), aligntotop.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChildren(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITextRangeProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CompareEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, *mut core::ffi::c_void, TextPatternRangeEndpoint, *mut i32) -> windows_core::HRESULT,
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(*mut core::ffi::c_void, TextUnit) -> windows_core::HRESULT,
    pub FindAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_TEXTATTRIBUTE_ID, core::mem::MaybeUninit<windows_core::VARIANT>, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_TEXTATTRIBUTE_ID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBoundingRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBoundingRectangles: usize,
    pub GetEnclosingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEndpointByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEndpointByRange: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, *mut core::ffi::c_void, TextPatternRangeEndpoint) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChildren: usize,
}
windows_core::imp::define_interface!(ITextRangeProvider2, ITextRangeProvider2_Vtbl, 0x9bbce42c_1921_4f18_89ca_dba1910a0386);
impl core::ops::Deref for ITextRangeProvider2 {
    type Target = ITextRangeProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextRangeProvider2, windows_core::IUnknown, ITextRangeProvider);
impl ITextRangeProvider2 {
    pub unsafe fn ShowContextMenu(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITextRangeProvider2_Vtbl {
    pub base__: ITextRangeProvider_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToggleProvider, IToggleProvider_Vtbl, 0x56d00bd0_c4f4_433c_a836_1a52a57e0892);
impl core::ops::Deref for IToggleProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IToggleProvider, windows_core::IUnknown);
impl IToggleProvider {
    pub unsafe fn Toggle(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Toggle)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ToggleState(&self) -> windows_core::Result<ToggleState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IToggleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Toggle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToggleState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransformProvider, ITransformProvider_Vtbl, 0x6829ddc4_4f91_4ffa_b86f_bd3e2987cb4c);
impl core::ops::Deref for ITransformProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransformProvider, windows_core::IUnknown);
impl ITransformProvider {
    pub unsafe fn Move(&self, x: f64, y: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), x, y).ok()
    }
    pub unsafe fn Resize(&self, width: f64, height: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Rotate)(windows_core::Interface::as_raw(self), degrees).ok()
    }
    pub unsafe fn CanMove(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CanResize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CanRotate(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITransformProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransformProvider2, ITransformProvider2_Vtbl, 0x4758742f_7ac2_460c_bc48_09fc09308a93);
impl core::ops::Deref for ITransformProvider2 {
    type Target = ITransformProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransformProvider2, windows_core::IUnknown, ITransformProvider);
impl ITransformProvider2 {
    pub unsafe fn Zoom(&self, zoom: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Zoom)(windows_core::Interface::as_raw(self), zoom).ok()
    }
    pub unsafe fn CanZoom(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ZoomLevel(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ZoomMinimum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ZoomMaximum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ZoomByUnit(&self, zoomunit: ZoomUnit) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ZoomByUnit)(windows_core::Interface::as_raw(self), zoomunit).ok()
    }
}
#[repr(C)]
pub struct ITransformProvider2_Vtbl {
    pub base__: ITransformProvider_Vtbl,
    pub Zoom: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, ZoomUnit) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomation, IUIAutomation_Vtbl, 0x30cbe57d_d9d0_452a_ab13_7ac5ac4825ee);
impl core::ops::Deref for IUIAutomation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation, windows_core::IUnknown);
impl IUIAutomation {
    pub unsafe fn CompareElements<P0, P1>(&self, el1: P0, el2: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareElements)(windows_core::Interface::as_raw(self), el1.param().abi(), el2.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareRuntimeIds)(windows_core::Interface::as_raw(self), runtimeid1, runtimeid2, &mut result__).map(|| result__)
    }
    pub unsafe fn GetRootElement(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRootElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ElementFromHandle<P0>(&self, hwnd: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElementFromHandle)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ElementFromPoint(&self, pt: super::super::Foundation::POINT) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElementFromPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(pt), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFocusedElement(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRootElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ElementFromHandleBuildCache<P0, P1>(&self, hwnd: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElementFromHandleBuildCache)(windows_core::Interface::as_raw(self), hwnd.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ElementFromPointBuildCache<P0>(&self, pt: super::super::Foundation::POINT, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElementFromPointBuildCache)(windows_core::Interface::as_raw(self), core::mem::transmute(pt), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFocusedElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> windows_core::Result<IUIAutomationTreeWalker>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTreeWalker)(windows_core::Interface::as_raw(self), pcondition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ControlViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ControlViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ContentViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContentViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RawViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RawViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RawViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RawViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ControlViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ControlViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ContentViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContentViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCacheRequest(&self) -> windows_core::Result<IUIAutomationCacheRequest> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCacheRequest)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTrueCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTrueCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFalseCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFalseCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePropertyCondition<P0>(&self, propertyid: UIA_PROPERTY_ID, value: P0) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePropertyCondition)(windows_core::Interface::as_raw(self), propertyid, value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePropertyConditionEx<P0>(&self, propertyid: UIA_PROPERTY_ID, value: P0, flags: PropertyConditionFlags) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePropertyConditionEx)(windows_core::Interface::as_raw(self), propertyid, value.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAndCondition)(windows_core::Interface::as_raw(self), condition1.param().abi(), condition2.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAndConditionFromArray)(windows_core::Interface::as_raw(self), conditions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: &[Option<IUIAutomationCondition>]) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAndConditionFromNativeArray)(windows_core::Interface::as_raw(self), core::mem::transmute(conditions.as_ptr()), conditions.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateOrCondition)(windows_core::Interface::as_raw(self), condition1.param().abi(), condition2.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateOrConditionFromArray)(windows_core::Interface::as_raw(self), conditions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: &[Option<IUIAutomationCondition>]) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateOrConditionFromNativeArray)(windows_core::Interface::as_raw(self), core::mem::transmute(conditions.as_ptr()), conditions.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateNotCondition)(windows_core::Interface::as_raw(self), condition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1, P2>(&self, eventid: UIA_EVENT_ID, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn RemoveAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, element: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationEventHandler>,
    {
        (windows_core::Interface::vtable(self).RemoveAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, element.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: &[UIA_PROPERTY_ID]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddPropertyChangedEventHandlerNativeArray)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi(), core::mem::transmute(propertyarray.as_ptr()), propertyarray.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddPropertyChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi(), propertyarray).ok()
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).RemovePropertyChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddStructureChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).RemoveStructureChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(&self, cacherequest: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationFocusChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddFocusChangedEventHandler)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationFocusChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).RemoveFocusChangedEventHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAllEventHandlers)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: &[i32]) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IntNativeArrayToSafeArray)(windows_core::Interface::as_raw(self), core::mem::transmute(array.as_ptr()), array.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IntSafeArrayToNativeArray)(windows_core::Interface::as_raw(self), intarray, array, &mut result__).map(|| result__)
    }
    pub unsafe fn RectToVariant(&self, rc: super::super::Foundation::RECT) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RectToVariant)(windows_core::Interface::as_raw(self), core::mem::transmute(rc), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VariantToRect<P0>(&self, var: P0) -> windows_core::Result<super::super::Foundation::RECT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VariantToRect)(windows_core::Interface::as_raw(self), var.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SafeArrayToRectNativeArray)(windows_core::Interface::as_raw(self), rects, rectarray, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(&self, factory: P0) -> windows_core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: windows_core::Param<IUIAutomationProxyFactory>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateProxyFactoryEntry)(windows_core::Interface::as_raw(self), factory.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProxyFactoryMapping(&self) -> windows_core::Result<IUIAutomationProxyFactoryMapping> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProxyFactoryMapping)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyProgrammaticName(&self, property: UIA_PROPERTY_ID) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyProgrammaticName)(windows_core::Interface::as_raw(self), property, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: UIA_PATTERN_ID) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPatternProgrammaticName)(windows_core::Interface::as_raw(self), pattern, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        (windows_core::Interface::vtable(self).PollForPotentialSupportedPatterns)(windows_core::Interface::as_raw(self), pelement.param().abi(), patternids, patternnames).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        (windows_core::Interface::vtable(self).PollForPotentialSupportedProperties)(windows_core::Interface::as_raw(self), pelement.param().abi(), propertyids, propertynames).ok()
    }
    pub unsafe fn CheckNotSupported<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckNotSupported)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReservedNotSupportedValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReservedMixedAttributeValue(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReservedMixedAttributeValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IAccessible>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElementFromIAccessible)(windows_core::Interface::as_raw(self), accessible.param().abi(), childid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P1>(&self, accessible: P0, childid: i32, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IAccessible>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElementFromIAccessibleBuildCache)(windows_core::Interface::as_raw(self), accessible.param().abi(), childid, cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompareElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareRuntimeIds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *const super::super::System::Com::SAFEARRAY, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareRuntimeIds: usize,
    pub GetRootElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ElementFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ElementFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ElementFromHandleBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ElementFromPointBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::POINT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocusedElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTreeWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ControlViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RawViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RawViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ControlViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCacheRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTrueCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFalseCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePropertyCondition: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePropertyConditionEx: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID, core::mem::MaybeUninit<windows_core::VARIANT>, PropertyConditionFlags, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAndCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAndConditionFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAndConditionFromArray: usize,
    pub CreateAndConditionFromNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOrCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateOrConditionFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateOrConditionFromArray: usize,
    pub CreateOrConditionFromNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNotCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_EVENT_ID, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_EVENT_ID, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPropertyChangedEventHandlerNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const UIA_PROPERTY_ID, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPropertyChangedEventHandler: usize,
    pub RemovePropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddFocusChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFocusChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllEventHandlers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IntNativeArrayToSafeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, i32, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IntNativeArrayToSafeArray: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IntSafeArrayToNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IntSafeArrayToNativeArray: usize,
    pub RectToVariant: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::RECT, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub VariantToRect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SafeArrayToRectNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut *mut super::super::Foundation::RECT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SafeArrayToRectNativeArray: usize,
    pub CreateProxyFactoryEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyFactoryMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyProgrammaticName: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPatternProgrammaticName: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PATTERN_ID, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PollForPotentialSupportedPatterns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PollForPotentialSupportedPatterns: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PollForPotentialSupportedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PollForPotentialSupportedProperties: usize,
    pub CheckNotSupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ReservedNotSupportedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReservedMixedAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ElementFromIAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ElementFromIAccessible: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ElementFromIAccessibleBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ElementFromIAccessibleBuildCache: usize,
}
windows_core::imp::define_interface!(IUIAutomation2, IUIAutomation2_Vtbl, 0x34723aff_0c9d_49d0_9896_7ab52df8cd8a);
impl core::ops::Deref for IUIAutomation2 {
    type Target = IUIAutomation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation2, windows_core::IUnknown, IUIAutomation);
impl IUIAutomation2 {
    pub unsafe fn AutoSetFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoSetFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoSetFocus<P0>(&self, autosetfocus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoSetFocus)(windows_core::Interface::as_raw(self), autosetfocus.param().abi()).ok()
    }
    pub unsafe fn ConnectionTimeout(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectionTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetConnectionTimeout(&self, timeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConnectionTimeout)(windows_core::Interface::as_raw(self), timeout).ok()
    }
    pub unsafe fn TransactionTimeout(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTransactionTimeout(&self, timeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransactionTimeout)(windows_core::Interface::as_raw(self), timeout).ok()
    }
}
#[repr(C)]
pub struct IUIAutomation2_Vtbl {
    pub base__: IUIAutomation_Vtbl,
    pub AutoSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAutoSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ConnectionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetConnectionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TransactionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTransactionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomation3, IUIAutomation3_Vtbl, 0x73d768da_9b51_4b89_936e_c209290973e7);
impl core::ops::Deref for IUIAutomation3 {
    type Target = IUIAutomation2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation3, windows_core::IUnknown, IUIAutomation, IUIAutomation2);
impl IUIAutomation3 {
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: P1, handler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationTextEditTextChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddTextEditTextChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, texteditchangetype, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn RemoveTextEditTextChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationTextEditTextChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).RemoveTextEditTextChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomation3_Vtbl {
    pub base__: IUIAutomation2_Vtbl,
    pub AddTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, TextEditChangeType, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomation4, IUIAutomation4_Vtbl, 0x1189c02a_05f8_4319_8e21_e817e3db2860);
impl core::ops::Deref for IUIAutomation4 {
    type Target = IUIAutomation3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation4, windows_core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3);
impl IUIAutomation4 {
    pub unsafe fn AddChangesEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, changetypes: &[i32], pcacherequest: P1, handler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationChangesEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddChangesEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, core::mem::transmute(changetypes.as_ptr()), changetypes.len().try_into().unwrap(), pcacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn RemoveChangesEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationChangesEventHandler>,
    {
        (windows_core::Interface::vtable(self).RemoveChangesEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomation4_Vtbl {
    pub base__: IUIAutomation3_Vtbl,
    pub AddChangesEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *const i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveChangesEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomation5, IUIAutomation5_Vtbl, 0x25f700c8_d816_4057_a9dc_3cbdee77e256);
impl core::ops::Deref for IUIAutomation5 {
    type Target = IUIAutomation4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation5, windows_core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3, IUIAutomation4);
impl IUIAutomation5 {
    pub unsafe fn AddNotificationEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationNotificationEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddNotificationEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn RemoveNotificationEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationNotificationEventHandler>,
    {
        (windows_core::Interface::vtable(self).RemoveNotificationEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomation5_Vtbl {
    pub base__: IUIAutomation4_Vtbl,
    pub AddNotificationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveNotificationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomation6, IUIAutomation6_Vtbl, 0xaae072da_29e3_413d_87a7_192dbf81ed10);
impl core::ops::Deref for IUIAutomation6 {
    type Target = IUIAutomation5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation6, windows_core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3, IUIAutomation4, IUIAutomation5);
impl IUIAutomation6 {
    pub unsafe fn CreateEventHandlerGroup(&self) -> windows_core::Result<IUIAutomationEventHandlerGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEventHandlerGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddEventHandlerGroup<P0, P1>(&self, element: P0, handlergroup: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationEventHandlerGroup>,
    {
        (windows_core::Interface::vtable(self).AddEventHandlerGroup)(windows_core::Interface::as_raw(self), element.param().abi(), handlergroup.param().abi()).ok()
    }
    pub unsafe fn RemoveEventHandlerGroup<P0, P1>(&self, element: P0, handlergroup: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationEventHandlerGroup>,
    {
        (windows_core::Interface::vtable(self).RemoveEventHandlerGroup)(windows_core::Interface::as_raw(self), element.param().abi(), handlergroup.param().abi()).ok()
    }
    pub unsafe fn ConnectionRecoveryBehavior(&self) -> windows_core::Result<ConnectionRecoveryBehaviorOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectionRecoveryBehavior)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetConnectionRecoveryBehavior(&self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConnectionRecoveryBehavior)(windows_core::Interface::as_raw(self), connectionrecoverybehavioroptions).ok()
    }
    pub unsafe fn CoalesceEvents(&self) -> windows_core::Result<CoalesceEventsOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CoalesceEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCoalesceEvents(&self, coalesceeventsoptions: CoalesceEventsOptions) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCoalesceEvents)(windows_core::Interface::as_raw(self), coalesceeventsoptions).ok()
    }
    pub unsafe fn AddActiveTextPositionChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationActiveTextPositionChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddActiveTextPositionChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn RemoveActiveTextPositionChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationActiveTextPositionChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).RemoveActiveTextPositionChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomation6_Vtbl {
    pub base__: IUIAutomation5_Vtbl,
    pub CreateEventHandlerGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddEventHandlerGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveEventHandlerGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionRecoveryBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT,
    pub SetConnectionRecoveryBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT,
    pub CoalesceEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoalesceEventsOptions) -> windows_core::HRESULT,
    pub SetCoalesceEvents: unsafe extern "system" fn(*mut core::ffi::c_void, CoalesceEventsOptions) -> windows_core::HRESULT,
    pub AddActiveTextPositionChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveActiveTextPositionChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationActiveTextPositionChangedEventHandler, IUIAutomationActiveTextPositionChangedEventHandler_Vtbl, 0xf97933b0_8dae_4496_8997_5ba015fe0d82);
impl core::ops::Deref for IUIAutomationActiveTextPositionChangedEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationActiveTextPositionChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationActiveTextPositionChangedEventHandler {
    pub unsafe fn HandleActiveTextPositionChangedEvent<P0, P1>(&self, sender: P0, range: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationTextRange>,
    {
        (windows_core::Interface::vtable(self).HandleActiveTextPositionChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), range.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleActiveTextPositionChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationAndCondition, IUIAutomationAndCondition_Vtbl, 0xa7d0af36_b912_45fe_9855_091ddc174aec);
impl core::ops::Deref for IUIAutomationAndCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationAndCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationAndCondition {
    pub unsafe fn ChildCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ChildCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChildrenAsNativeArray)(windows_core::Interface::as_raw(self), childarray, childarraycount).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChildren(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationAndCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub ChildCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetChildrenAsNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut Option<IUIAutomationCondition>, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChildren: usize,
}
windows_core::imp::define_interface!(IUIAutomationAnnotationPattern, IUIAutomationAnnotationPattern_Vtbl, 0x9a175b21_339e_41b1_8e8b_623f6b681098);
impl core::ops::Deref for IUIAutomationAnnotationPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationAnnotationPattern, windows_core::IUnknown);
impl IUIAutomationAnnotationPattern {
    pub unsafe fn CurrentAnnotationTypeId(&self) -> windows_core::Result<UIA_ANNOTATIONTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentAuthor(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAuthor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentDateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDateTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentTarget(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedAnnotationTypeId(&self) -> windows_core::Result<UIA_ANNOTATIONTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedAuthor(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAuthor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedDateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDateTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedTarget(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationAnnotationPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentAnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_ANNOTATIONTYPE) -> windows_core::HRESULT,
    pub CurrentAnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedAnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_ANNOTATIONTYPE) -> windows_core::HRESULT,
    pub CachedAnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationBoolCondition, IUIAutomationBoolCondition_Vtbl, 0x1b4e1f2e_75eb_4d0b_8952_5a69988e2307);
impl core::ops::Deref for IUIAutomationBoolCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationBoolCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationBoolCondition {
    pub unsafe fn BooleanValue(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BooleanValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationBoolCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub BooleanValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationCacheRequest, IUIAutomationCacheRequest_Vtbl, 0xb32a92b5_bc25_4078_9c08_d7ee95c48e03);
impl core::ops::Deref for IUIAutomationCacheRequest {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationCacheRequest, windows_core::IUnknown);
impl IUIAutomationCacheRequest {
    pub unsafe fn AddProperty(&self, propertyid: UIA_PROPERTY_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddProperty)(windows_core::Interface::as_raw(self), propertyid).ok()
    }
    pub unsafe fn AddPattern(&self, patternid: UIA_PATTERN_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddPattern)(windows_core::Interface::as_raw(self), patternid).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IUIAutomationCacheRequest> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TreeScope(&self) -> windows_core::Result<TreeScope> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TreeScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTreeScope(&self, scope: TreeScope) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTreeScope)(windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn TreeFilter(&self) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TreeFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTreeFilter<P0>(&self, filter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        (windows_core::Interface::vtable(self).SetTreeFilter)(windows_core::Interface::as_raw(self), filter.param().abi()).ok()
    }
    pub unsafe fn AutomationElementMode(&self) -> windows_core::Result<AutomationElementMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutomationElementMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutomationElementMode(&self, mode: AutomationElementMode) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutomationElementMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationCacheRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddProperty: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID) -> windows_core::HRESULT,
    pub AddPattern: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PATTERN_ID) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TreeScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TreeScope) -> windows_core::HRESULT,
    pub SetTreeScope: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope) -> windows_core::HRESULT,
    pub TreeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTreeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AutomationElementMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomationElementMode) -> windows_core::HRESULT,
    pub SetAutomationElementMode: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationElementMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationChangesEventHandler, IUIAutomationChangesEventHandler_Vtbl, 0x58edca55_2c3e_4980_b1b9_56c17f27a2a0);
impl core::ops::Deref for IUIAutomationChangesEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationChangesEventHandler, windows_core::IUnknown);
impl IUIAutomationChangesEventHandler {
    pub unsafe fn HandleChangesEvent<P0>(&self, sender: P0, uiachanges: &[UiaChangeInfo]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        (windows_core::Interface::vtable(self).HandleChangesEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), core::mem::transmute(uiachanges.as_ptr()), uiachanges.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationChangesEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleChangesEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const UiaChangeInfo, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationCondition, IUIAutomationCondition_Vtbl, 0x352ffba8_0973_437c_a61f_f64cafd81df9);
impl core::ops::Deref for IUIAutomationCondition {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationCondition, windows_core::IUnknown);
impl IUIAutomationCondition {}
#[repr(C)]
pub struct IUIAutomationCondition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IUIAutomationCustomNavigationPattern, IUIAutomationCustomNavigationPattern_Vtbl, 0x01ea217a_1766_47ed_a6cc_acf492854b1f);
impl core::ops::Deref for IUIAutomationCustomNavigationPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationCustomNavigationPattern, windows_core::IUnknown);
impl IUIAutomationCustomNavigationPattern {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationCustomNavigationPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationDockPattern, IUIAutomationDockPattern_Vtbl, 0xfde5ef97_1464_48f6_90bf_43d0948e86ec);
impl core::ops::Deref for IUIAutomationDockPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationDockPattern, windows_core::IUnknown);
impl IUIAutomationDockPattern {
    pub unsafe fn SetDockPosition(&self, dockpos: DockPosition) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDockPosition)(windows_core::Interface::as_raw(self), dockpos).ok()
    }
    pub unsafe fn CurrentDockPosition(&self) -> windows_core::Result<DockPosition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedDockPosition(&self) -> windows_core::Result<DockPosition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationDockPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, DockPosition) -> windows_core::HRESULT,
    pub CurrentDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DockPosition) -> windows_core::HRESULT,
    pub CachedDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DockPosition) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationDragPattern, IUIAutomationDragPattern_Vtbl, 0x1dc7b570_1f54_4bad_bcda_d36a722fb7bd);
impl core::ops::Deref for IUIAutomationDragPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationDragPattern, windows_core::IUnknown);
impl IUIAutomationDragPattern {
    pub unsafe fn CurrentIsGrabbed(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsGrabbed(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentDropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDropEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedDropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDropEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentDropEffects(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedDropEffects(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentGrabbedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedGrabbedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationDragPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentIsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentDropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentDropEffects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CachedDropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CachedDropEffects: usize,
    pub GetCurrentGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationDropTargetPattern, IUIAutomationDropTargetPattern_Vtbl, 0x69a095f7_eee4_430e_a46b_fb73b1ae39a5);
impl core::ops::Deref for IUIAutomationDropTargetPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationDropTargetPattern, windows_core::IUnknown);
impl IUIAutomationDropTargetPattern {
    pub unsafe fn CurrentDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentDropTargetEffects(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedDropTargetEffects(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationDropTargetPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentDropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedDropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentDropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentDropTargetEffects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CachedDropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CachedDropTargetEffects: usize,
}
windows_core::imp::define_interface!(IUIAutomationElement, IUIAutomationElement_Vtbl, 0xd22108aa_8ac5_49a5_837b_37bbb3d7591e);
impl core::ops::Deref for IUIAutomationElement {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement, windows_core::IUnknown);
impl IUIAutomationElement {
    pub unsafe fn SetFocus(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRuntimeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindFirst)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> windows_core::Result<IUIAutomationElementArray>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindAll)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindFirstBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElementArray>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindAllBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BuildUpdatedCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentPropertyValueEx)(windows_core::Interface::as_raw(self), propertyid, ignoredefaultvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedPropertyValueEx)(windows_core::Interface::as_raw(self), propertyid, ignoredefaultvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetCurrentPatternAs)(windows_core::Interface::as_raw(self), patternid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetCachedPatternAs)(windows_core::Interface::as_raw(self), patternid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentPattern)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedPattern)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedParent(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedChildren(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedChildren)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentProcessId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentControlType(&self) -> windows_core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLocalizedControlType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAcceleratorKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentAccessKey(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAccessKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentHasKeyboardFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsKeyboardFocusable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAutomationId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentClassName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentHelpText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentHelpText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentCulture(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCulture)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsControlElement(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsControlElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsContentElement(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsContentElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsPassword(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentNativeWindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentNativeWindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentItemType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentItemType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentIsOffscreen(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsOffscreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> windows_core::Result<OrientationType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFrameworkId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentIsRequiredForForm(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsRequiredForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentItemStatus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentBoundingRectangle(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentBoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLabeledBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentAriaRole(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAriaRole)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentAriaProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAriaProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentIsDataValidForForm(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsDataValidForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentControllerFor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDescribedBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFlowsTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentProviderDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentProviderDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedProcessId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedControlType(&self) -> windows_core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedLocalizedControlType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAcceleratorKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedAccessKey(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAccessKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedHasKeyboardFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedHasKeyboardFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsKeyboardFocusable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAutomationId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedClassName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedHelpText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedHelpText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedCulture(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCulture)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsControlElement(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsControlElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsContentElement(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsContentElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsPassword(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedNativeWindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedNativeWindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedItemType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedItemType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedIsOffscreen(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsOffscreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedOrientation(&self) -> windows_core::Result<OrientationType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFrameworkId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedIsRequiredForForm(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsRequiredForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedItemStatus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedBoundingRectangle(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedBoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedLabeledBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedAriaRole(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAriaRole)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedAriaProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAriaProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedIsDataValidForForm(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsDataValidForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedControllerFor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDescribedBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFlowsTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedProviderDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedProviderDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClickablePoint)(windows_core::Interface::as_raw(self), clickable, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationElement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRuntimeId: usize,
    pub FindFirst: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAll: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BuildUpdatedCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCurrentPropertyValueEx: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID, super::super::Foundation::BOOL, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCachedPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCachedPropertyValueEx: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PROPERTY_ID, super::super::Foundation::BOOL, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCurrentPatternAs: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PATTERN_ID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedPatternAs: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PATTERN_ID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentPattern: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PATTERN_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedPattern: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_PATTERN_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_CONTROLTYPE_ID) -> windows_core::HRESULT,
    pub CurrentLocalizedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentAcceleratorKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentHasKeyboardFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentIsKeyboardFocusable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentAutomationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentHelpText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentCulture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentIsControlElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentIsContentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentIsPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentNativeWindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub CurrentItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentIsOffscreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OrientationType) -> windows_core::HRESULT,
    pub CurrentFrameworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentIsRequiredForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentItemStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentBoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub CurrentLabeledBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAriaRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentAriaProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentIsDataValidForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentControllerFor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDescribedBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentFlowsTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentProviderDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_CONTROLTYPE_ID) -> windows_core::HRESULT,
    pub CachedLocalizedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedAcceleratorKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedHasKeyboardFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsKeyboardFocusable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedAutomationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedHelpText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedCulture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedIsControlElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsContentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedNativeWindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub CachedItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedIsOffscreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OrientationType) -> windows_core::HRESULT,
    pub CachedFrameworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedIsRequiredForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedItemStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedBoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub CachedLabeledBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedAriaRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedAriaProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedIsDataValidForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedControllerFor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedDescribedBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedFlowsTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedProviderDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetClickablePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::POINT, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElement2, IUIAutomationElement2_Vtbl, 0x6749c683_f70d_4487_a698_5f79d55290d6);
impl core::ops::Deref for IUIAutomationElement2 {
    type Target = IUIAutomationElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement2, windows_core::IUnknown, IUIAutomationElement);
impl IUIAutomationElement2 {
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentOptimizeForVisualContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedOptimizeForVisualContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> windows_core::Result<LiveSetting> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLiveSetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> windows_core::Result<LiveSetting> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedLiveSetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFlowsFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFlowsFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationElement2_Vtbl {
    pub base__: IUIAutomationElement_Vtbl,
    pub CurrentOptimizeForVisualContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedOptimizeForVisualContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentLiveSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LiveSetting) -> windows_core::HRESULT,
    pub CachedLiveSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LiveSetting) -> windows_core::HRESULT,
    pub CurrentFlowsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedFlowsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElement3, IUIAutomationElement3_Vtbl, 0x8471df34_aee0_4a01_a7de_7db9af12c296);
impl core::ops::Deref for IUIAutomationElement3 {
    type Target = IUIAutomationElement2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement3, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2);
impl IUIAutomationElement3 {
    pub unsafe fn ShowContextMenu(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CurrentIsPeripheral(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsPeripheral)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsPeripheral(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsPeripheral)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationElement3_Vtbl {
    pub base__: IUIAutomationElement2_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentIsPeripheral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsPeripheral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElement4, IUIAutomationElement4_Vtbl, 0x3b6e233c_52fb_4063_a4c9_77c075c2a06b);
impl core::ops::Deref for IUIAutomationElement4 {
    type Target = IUIAutomationElement3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement4, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3);
impl IUIAutomationElement4 {
    pub unsafe fn CurrentPositionInSet(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentPositionInSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentSizeOfSet(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentSizeOfSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedPositionInSet(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedPositionInSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedSizeOfSet(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedSizeOfSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedAnnotationTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationElement4_Vtbl {
    pub base__: IUIAutomationElement3_Vtbl,
    pub CurrentPositionInSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentSizeOfSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentAnnotationTypes: usize,
    pub CurrentAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedPositionInSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedSizeOfSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CachedAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CachedAnnotationTypes: usize,
    pub CachedAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElement5, IUIAutomationElement5_Vtbl, 0x98141c1d_0d0e_4175_bbe2_6bff455842a7);
impl core::ops::Deref for IUIAutomationElement5 {
    type Target = IUIAutomationElement4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement5, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4);
impl IUIAutomationElement5 {
    pub unsafe fn CurrentLandmarkType(&self) -> windows_core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLocalizedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedLandmarkType(&self) -> windows_core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedLocalizedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationElement5_Vtbl {
    pub base__: IUIAutomationElement4_Vtbl,
    pub CurrentLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_LANDMARKTYPE_ID) -> windows_core::HRESULT,
    pub CurrentLocalizedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_LANDMARKTYPE_ID) -> windows_core::HRESULT,
    pub CachedLocalizedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElement6, IUIAutomationElement6_Vtbl, 0x4780d450_8bca_4977_afa5_a4a517f555e3);
impl core::ops::Deref for IUIAutomationElement6 {
    type Target = IUIAutomationElement5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement6, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5);
impl IUIAutomationElement6 {
    pub unsafe fn CurrentFullDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFullDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedFullDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFullDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationElement6_Vtbl {
    pub base__: IUIAutomationElement5_Vtbl,
    pub CurrentFullDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedFullDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElement7, IUIAutomationElement7_Vtbl, 0x204e8572_cfc3_4c11_b0c8_7da7420750b7);
impl core::ops::Deref for IUIAutomationElement7 {
    type Target = IUIAutomationElement6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement7, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6);
impl IUIAutomationElement7 {
    pub unsafe fn FindFirstWithOptions<P0, P1>(&self, scope: TreeScope, condition: P0, traversaloptions: TreeTraversalOptions, root: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindFirstWithOptions)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindAllWithOptions<P0, P1>(&self, scope: TreeScope, condition: P0, traversaloptions: TreeTraversalOptions, root: P1) -> windows_core::Result<IUIAutomationElementArray>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindAllWithOptions)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindFirstWithOptionsBuildCache<P0, P1, P2>(&self, scope: TreeScope, condition: P0, cacherequest: P1, traversaloptions: TreeTraversalOptions, root: P2) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindFirstWithOptionsBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindAllWithOptionsBuildCache<P0, P1, P2>(&self, scope: TreeScope, condition: P0, cacherequest: P1, traversaloptions: TreeTraversalOptions, root: P2) -> windows_core::Result<IUIAutomationElementArray>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindAllWithOptionsBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: UIA_METADATA_ID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentMetadataValue)(windows_core::Interface::as_raw(self), targetid, metadataid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationElement7_Vtbl {
    pub base__: IUIAutomationElement6_Vtbl,
    pub FindFirstWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, TreeTraversalOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, TreeTraversalOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstWithOptionsBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, TreeTraversalOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllWithOptionsBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, TreeTraversalOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentMetadataValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, UIA_METADATA_ID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElement8, IUIAutomationElement8_Vtbl, 0x8c60217d_5411_4cde_bcc0_1ceda223830c);
impl core::ops::Deref for IUIAutomationElement8 {
    type Target = IUIAutomationElement7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement8, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6, IUIAutomationElement7);
impl IUIAutomationElement8 {
    pub unsafe fn CurrentHeadingLevel(&self) -> windows_core::Result<UIA_HEADINGLEVEL_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentHeadingLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedHeadingLevel(&self) -> windows_core::Result<UIA_HEADINGLEVEL_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedHeadingLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationElement8_Vtbl {
    pub base__: IUIAutomationElement7_Vtbl,
    pub CurrentHeadingLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_HEADINGLEVEL_ID) -> windows_core::HRESULT,
    pub CachedHeadingLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_HEADINGLEVEL_ID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElement9, IUIAutomationElement9_Vtbl, 0x39325fac_039d_440e_a3a3_5eb81a5cecc3);
impl core::ops::Deref for IUIAutomationElement9 {
    type Target = IUIAutomationElement8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement9, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6, IUIAutomationElement7, IUIAutomationElement8);
impl IUIAutomationElement9 {
    pub unsafe fn CurrentIsDialog(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsDialog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsDialog(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsDialog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationElement9_Vtbl {
    pub base__: IUIAutomationElement8_Vtbl,
    pub CurrentIsDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationElementArray, IUIAutomationElementArray_Vtbl, 0x14314595_b4bc_4055_95f2_58f2e42c9855);
impl core::ops::Deref for IUIAutomationElementArray {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElementArray, windows_core::IUnknown);
impl IUIAutomationElementArray {
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationElementArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationEventHandler, IUIAutomationEventHandler_Vtbl, 0x146c3c17_f12e_4e22_8c27_f894b9b79c69);
impl core::ops::Deref for IUIAutomationEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationEventHandler, windows_core::IUnknown);
impl IUIAutomationEventHandler {
    pub unsafe fn HandleAutomationEvent<P0>(&self, sender: P0, eventid: UIA_EVENT_ID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        (windows_core::Interface::vtable(self).HandleAutomationEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), eventid).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UIA_EVENT_ID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationEventHandlerGroup, IUIAutomationEventHandlerGroup_Vtbl, 0xc9ee12f2_c13b_4408_997c_639914377f4e);
impl core::ops::Deref for IUIAutomationEventHandlerGroup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationEventHandlerGroup, windows_core::IUnknown);
impl IUIAutomationEventHandlerGroup {
    pub unsafe fn AddActiveTextPositionChangedEventHandler<P0, P1>(&self, scope: TreeScope, cacherequest: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationActiveTextPositionChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddActiveTextPositionChangedEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, scope: TreeScope, cacherequest: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, scope, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn AddChangesEventHandler<P0, P1>(&self, scope: TreeScope, changetypes: &[i32], cacherequest: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationChangesEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddChangesEventHandler)(windows_core::Interface::as_raw(self), scope, core::mem::transmute(changetypes.as_ptr()), changetypes.len().try_into().unwrap(), cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn AddNotificationEventHandler<P0, P1>(&self, scope: TreeScope, cacherequest: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationNotificationEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddNotificationEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1>(&self, scope: TreeScope, cacherequest: P0, handler: P1, propertyarray: &[UIA_PROPERTY_ID]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddPropertyChangedEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi(), core::mem::transmute(propertyarray.as_ptr()), propertyarray.len().try_into().unwrap()).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1>(&self, scope: TreeScope, cacherequest: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddStructureChangedEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi()).ok()
    }
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P1>(&self, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationTextEditTextChangedEventHandler>,
    {
        (windows_core::Interface::vtable(self).AddTextEditTextChangedEventHandler)(windows_core::Interface::as_raw(self), scope, texteditchangetype, cacherequest.param().abi(), handler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationEventHandlerGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddActiveTextPositionChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_EVENT_ID, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddChangesEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *const i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddNotificationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const UIA_PROPERTY_ID, i32) -> windows_core::HRESULT,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, TextEditChangeType, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationExpandCollapsePattern, IUIAutomationExpandCollapsePattern_Vtbl, 0x619be086_1f4e_4ee4_bafa_210128738730);
impl core::ops::Deref for IUIAutomationExpandCollapsePattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationExpandCollapsePattern, windows_core::IUnknown);
impl IUIAutomationExpandCollapsePattern {
    pub unsafe fn Expand(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Collapse(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Collapse)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CurrentExpandCollapseState(&self) -> windows_core::Result<ExpandCollapseState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedExpandCollapseState(&self) -> windows_core::Result<ExpandCollapseState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationExpandCollapsePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExpandCollapseState) -> windows_core::HRESULT,
    pub CachedExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExpandCollapseState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationFocusChangedEventHandler, IUIAutomationFocusChangedEventHandler_Vtbl, 0xc270f6b5_5c69_4290_9745_7a7f97169468);
impl core::ops::Deref for IUIAutomationFocusChangedEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationFocusChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationFocusChangedEventHandler {
    pub unsafe fn HandleFocusChangedEvent<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        (windows_core::Interface::vtable(self).HandleFocusChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationFocusChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleFocusChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationGridItemPattern, IUIAutomationGridItemPattern_Vtbl, 0x78f8ef57_66c3_4e09_bd7c_e79b2004894d);
impl core::ops::Deref for IUIAutomationGridItemPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationGridItemPattern, windows_core::IUnknown);
impl IUIAutomationGridItemPattern {
    pub unsafe fn CurrentContainingGrid(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentRow(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentRow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentColumn(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentRowSpan(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentRowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentColumnSpan(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedContainingGrid(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedRow(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedRow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedColumn(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedRowSpan(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedRowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedColumnSpan(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationGridItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentRowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedRowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationGridPattern, IUIAutomationGridPattern_Vtbl, 0x414c3cdc_856b_4f5b_8538_3131c6302550);
impl core::ops::Deref for IUIAutomationGridPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationGridPattern, windows_core::IUnknown);
impl IUIAutomationGridPattern {
    pub unsafe fn GetItem(&self, row: i32, column: i32) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), row, column, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentRowCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentRowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentColumnCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedRowCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedRowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedColumnCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationGridPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationInvokePattern, IUIAutomationInvokePattern_Vtbl, 0xfb377fbe_8ea6_46d5_9c73_6499642d3059);
impl core::ops::Deref for IUIAutomationInvokePattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationInvokePattern, windows_core::IUnknown);
impl IUIAutomationInvokePattern {
    pub unsafe fn Invoke(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationInvokePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationItemContainerPattern, IUIAutomationItemContainerPattern_Vtbl, 0xc690fdb2_27a8_423c_812d_429773c9084e);
impl core::ops::Deref for IUIAutomationItemContainerPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationItemContainerPattern, windows_core::IUnknown);
impl IUIAutomationItemContainerPattern {
    pub unsafe fn FindItemByProperty<P0, P1>(&self, pstartafter: P0, propertyid: UIA_PROPERTY_ID, value: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindItemByProperty)(windows_core::Interface::as_raw(self), pstartafter.param().abi(), propertyid, value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationItemContainerPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindItemByProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UIA_PROPERTY_ID, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationLegacyIAccessiblePattern, IUIAutomationLegacyIAccessiblePattern_Vtbl, 0x828055ad_355b_4435_86d5_3b51c14a9b1b);
impl core::ops::Deref for IUIAutomationLegacyIAccessiblePattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationLegacyIAccessiblePattern, windows_core::IUnknown);
impl IUIAutomationLegacyIAccessiblePattern {
    pub unsafe fn Select(&self, flagsselect: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self), flagsselect).ok()
    }
    pub unsafe fn DoDefaultAction(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoDefaultAction)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetValue<P0>(&self, szvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), szvalue.param().abi()).ok()
    }
    pub unsafe fn CurrentChildId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentValue(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentRole(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentState(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentHelp(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentHelp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentKeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentDefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDefaultAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedChildId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedValue(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedRole(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedState(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedHelp(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedHelp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedKeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedDefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedDefaultAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetIAccessible(&self) -> windows_core::Result<IAccessible> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIAccessible)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationLegacyIAccessiblePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DoDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CurrentChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentHelp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentKeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCurrentSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CachedState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CachedHelp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedKeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCachedSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetIAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetIAccessible: usize,
}
windows_core::imp::define_interface!(IUIAutomationMultipleViewPattern, IUIAutomationMultipleViewPattern_Vtbl, 0x8d253c91_1dc5_4bb5_b18f_ade16fa495e8);
impl core::ops::Deref for IUIAutomationMultipleViewPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationMultipleViewPattern, windows_core::IUnknown);
impl IUIAutomationMultipleViewPattern {
    pub unsafe fn GetViewName(&self, view: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewName)(windows_core::Interface::as_raw(self), view, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCurrentView(&self, view: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCurrentView)(windows_core::Interface::as_raw(self), view).ok()
    }
    pub unsafe fn CurrentCurrentView(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCurrentSupportedViews(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentSupportedViews)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedCurrentView(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCachedSupportedViews(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedSupportedViews)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationMultipleViewPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetViewName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CurrentCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCurrentSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCurrentSupportedViews: usize,
    pub CachedCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCachedSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCachedSupportedViews: usize,
}
windows_core::imp::define_interface!(IUIAutomationNotCondition, IUIAutomationNotCondition_Vtbl, 0xf528b657_847b_498c_8896_d52b565407a1);
impl core::ops::Deref for IUIAutomationNotCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationNotCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationNotCondition {
    pub unsafe fn GetChild(&self) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChild)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationNotCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub GetChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationNotificationEventHandler, IUIAutomationNotificationEventHandler_Vtbl, 0xc7cb2637_e6c2_4d0c_85de_4948c02175c7);
impl core::ops::Deref for IUIAutomationNotificationEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationNotificationEventHandler, windows_core::IUnknown);
impl IUIAutomationNotificationEventHandler {
    pub unsafe fn HandleNotificationEvent<P0, P1, P2>(&self, sender: P0, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: P1, activityid: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).HandleNotificationEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), notificationkind, notificationprocessing, displaystring.param().abi(), activityid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationNotificationEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleNotificationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, NotificationKind, NotificationProcessing, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationObjectModelPattern, IUIAutomationObjectModelPattern_Vtbl, 0x71c284b3_c14d_4d14_981e_19751b0d756d);
impl core::ops::Deref for IUIAutomationObjectModelPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationObjectModelPattern, windows_core::IUnknown);
impl IUIAutomationObjectModelPattern {
    pub unsafe fn GetUnderlyingObjectModel(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUnderlyingObjectModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationObjectModelPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationOrCondition, IUIAutomationOrCondition_Vtbl, 0x8753f032_3db1_47b5_a1fc_6e34a266c712);
impl core::ops::Deref for IUIAutomationOrCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationOrCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationOrCondition {
    pub unsafe fn ChildCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ChildCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChildrenAsNativeArray)(windows_core::Interface::as_raw(self), childarray, childarraycount).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChildren(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationOrCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub ChildCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetChildrenAsNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut Option<IUIAutomationCondition>, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChildren: usize,
}
windows_core::imp::define_interface!(IUIAutomationPatternHandler, IUIAutomationPatternHandler_Vtbl, 0xd97022f3_a947_465e_8b2a_ac4315fa54e8);
impl core::ops::Deref for IUIAutomationPatternHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationPatternHandler, windows_core::IUnknown);
impl IUIAutomationPatternHandler {
    pub unsafe fn CreateClientWrapper<P0>(&self, ppatterninstance: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IUIAutomationPatternInstance>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateClientWrapper)(windows_core::Interface::as_raw(self), ppatterninstance.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Dispatch<P0>(&self, ptarget: P0, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Dispatch)(windows_core::Interface::as_raw(self), ptarget.param().abi(), index, pparams, cparams).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationPatternHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateClientWrapper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const UIAutomationParameter, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationPatternInstance, IUIAutomationPatternInstance_Vtbl, 0xc03a7fe4_9431_409f_bed8_ae7c2299bc8d);
impl core::ops::Deref for IUIAutomationPatternInstance {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationPatternInstance, windows_core::IUnknown);
impl IUIAutomationPatternInstance {
    pub unsafe fn GetProperty<P0>(&self, index: u32, cached: P0, r#type: UIAutomationType, pptr: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), index, cached.param().abi(), r#type, pptr).ok()
    }
    pub unsafe fn CallMethod(&self, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CallMethod)(windows_core::Interface::as_raw(self), index, pparams, cparams).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationPatternInstance_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL, UIAutomationType, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CallMethod: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const UIAutomationParameter, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationPropertyChangedEventHandler, IUIAutomationPropertyChangedEventHandler_Vtbl, 0x40cd37d4_c756_4b0c_8c6f_bddfeeb13b50);
impl core::ops::Deref for IUIAutomationPropertyChangedEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationPropertyChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationPropertyChangedEventHandler {
    pub unsafe fn HandlePropertyChangedEvent<P0, P1>(&self, sender: P0, propertyid: UIA_PROPERTY_ID, newvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).HandlePropertyChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), propertyid, newvalue.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandlePropertyChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UIA_PROPERTY_ID, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationPropertyCondition, IUIAutomationPropertyCondition_Vtbl, 0x99ebf2cb_5578_4267_9ad4_afd6ea77e94b);
impl core::ops::Deref for IUIAutomationPropertyCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationPropertyCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationPropertyCondition {
    pub unsafe fn PropertyId(&self) -> windows_core::Result<UIA_PROPERTY_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PropertyValue(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PropertyConditionFlags(&self) -> windows_core::Result<PropertyConditionFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyConditionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationPropertyCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub PropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_PROPERTY_ID) -> windows_core::HRESULT,
    pub PropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PropertyConditionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PropertyConditionFlags) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationProxyFactory, IUIAutomationProxyFactory_Vtbl, 0x85b94ecd_849d_42b6_b94d_d6db23fdf5a4);
impl core::ops::Deref for IUIAutomationProxyFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactory, windows_core::IUnknown);
impl IUIAutomationProxyFactory {
    pub unsafe fn CreateProvider<P0>(&self, hwnd: P0, idobject: i32, idchild: i32) -> windows_core::Result<IRawElementProviderSimple>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateProvider)(windows_core::Interface::as_raw(self), hwnd.param().abi(), idobject, idchild, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProxyFactoryId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProxyFactoryId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationProxyFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyFactoryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationProxyFactoryEntry, IUIAutomationProxyFactoryEntry_Vtbl, 0xd50e472e_b64b_490c_bca1_d30696f9f289);
impl core::ops::Deref for IUIAutomationProxyFactoryEntry {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactoryEntry, windows_core::IUnknown);
impl IUIAutomationProxyFactoryEntry {
    pub unsafe fn ProxyFactory(&self) -> windows_core::Result<IUIAutomationProxyFactory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProxyFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ImageName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImageName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AllowSubstringMatch(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowSubstringMatch)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CanCheckBaseClass(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanCheckBaseClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NeedsAdviseEvents(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NeedsAdviseEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClassName<P0>(&self, classname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetClassName)(windows_core::Interface::as_raw(self), classname.param().abi()).ok()
    }
    pub unsafe fn SetImageName<P0>(&self, imagename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetImageName)(windows_core::Interface::as_raw(self), imagename.param().abi()).ok()
    }
    pub unsafe fn SetAllowSubstringMatch<P0>(&self, allowsubstringmatch: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowSubstringMatch)(windows_core::Interface::as_raw(self), allowsubstringmatch.param().abi()).ok()
    }
    pub unsafe fn SetCanCheckBaseClass<P0>(&self, cancheckbaseclass: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCanCheckBaseClass)(windows_core::Interface::as_raw(self), cancheckbaseclass.param().abi()).ok()
    }
    pub unsafe fn SetNeedsAdviseEvents<P0>(&self, adviseevents: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNeedsAdviseEvents)(windows_core::Interface::as_raw(self), adviseevents.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWinEventsForAutomationEvent(&self, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWinEventsForAutomationEvent)(windows_core::Interface::as_raw(self), eventid, propertyid, winevents).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWinEventsForAutomationEvent(&self, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWinEventsForAutomationEvent)(windows_core::Interface::as_raw(self), eventid, propertyid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationProxyFactoryEntry_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProxyFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ImageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AllowSubstringMatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CanCheckBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub NeedsAdviseEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetImageName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetAllowSubstringMatch: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCanCheckBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetNeedsAdviseEvents: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWinEventsForAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_EVENT_ID, UIA_PROPERTY_ID, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWinEventsForAutomationEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWinEventsForAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_EVENT_ID, UIA_PROPERTY_ID, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWinEventsForAutomationEvent: usize,
}
windows_core::imp::define_interface!(IUIAutomationProxyFactoryMapping, IUIAutomationProxyFactoryMapping_Vtbl, 0x09e31e18_872d_4873_93d1_1e541ec133fd);
impl core::ops::Deref for IUIAutomationProxyFactoryMapping {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactoryMapping, windows_core::IUnknown);
impl IUIAutomationProxyFactoryMapping {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTable(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEntry(&self, index: u32) -> windows_core::Result<IUIAutomationProxyFactoryEntry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEntry)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTable(&self, factorylist: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTable)(windows_core::Interface::as_raw(self), factorylist).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEntries(&self, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertEntries)(windows_core::Interface::as_raw(self), before, factorylist).ok()
    }
    pub unsafe fn InsertEntry<P0>(&self, before: u32, factory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationProxyFactoryEntry>,
    {
        (windows_core::Interface::vtable(self).InsertEntry)(windows_core::Interface::as_raw(self), before, factory.param().abi()).ok()
    }
    pub unsafe fn RemoveEntry(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveEntry)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn ClearTable(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearTable)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RestoreDefaultTable(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestoreDefaultTable)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationProxyFactoryMapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTable: usize,
    pub GetEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEntries: usize,
    pub InsertEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ClearTable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestoreDefaultTable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationRangeValuePattern, IUIAutomationRangeValuePattern_Vtbl, 0x59213f4f_7346_49e5_b120_80555987a148);
impl core::ops::Deref for IUIAutomationRangeValuePattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationRangeValuePattern, windows_core::IUnknown);
impl IUIAutomationRangeValuePattern {
    pub unsafe fn SetValue(&self, val: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn CurrentValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsReadOnly(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentMaximum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentMinimum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentLargeChange(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentSmallChange(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentSmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsReadOnly(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedMaximum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedMinimum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedLargeChange(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedLargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedSmallChange(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedSmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationRangeValuePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentLargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentSmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedLargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedSmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationRegistrar, IUIAutomationRegistrar_Vtbl, 0x8609c4ec_4a1a_4d88_a357_5a66e060e1cf);
impl core::ops::Deref for IUIAutomationRegistrar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationRegistrar, windows_core::IUnknown);
impl IUIAutomationRegistrar {
    pub unsafe fn RegisterProperty(&self, property: *const UIAutomationPropertyInfo) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterProperty)(windows_core::Interface::as_raw(self), property, &mut result__).map(|| result__)
    }
    pub unsafe fn RegisterEvent(&self, event: *const UIAutomationEventInfo) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterEvent)(windows_core::Interface::as_raw(self), event, &mut result__).map(|| result__)
    }
    pub unsafe fn RegisterPattern(&self, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, ppropertyids: &mut [i32], peventids: &mut [i32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterPattern)(windows_core::Interface::as_raw(self), pattern, ppatternid, ppatternavailablepropertyid, ppropertyids.len().try_into().unwrap(), core::mem::transmute(ppropertyids.as_ptr()), peventids.len().try_into().unwrap(), core::mem::transmute(peventids.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationPropertyInfo, *mut i32) -> windows_core::HRESULT,
    pub RegisterEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationEventInfo, *mut i32) -> windows_core::HRESULT,
    pub RegisterPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationPatternInfo, *mut i32, *mut i32, u32, *mut i32, u32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationScrollItemPattern, IUIAutomationScrollItemPattern_Vtbl, 0xb488300f_d015_4f19_9c29_bb595e3645ef);
impl core::ops::Deref for IUIAutomationScrollItemPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationScrollItemPattern, windows_core::IUnknown);
impl IUIAutomationScrollItemPattern {
    pub unsafe fn ScrollIntoView(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationScrollItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationScrollPattern, IUIAutomationScrollPattern_Vtbl, 0x88f4d42a_e881_459d_a77c_73bbbb7e02dc);
impl core::ops::Deref for IUIAutomationScrollPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationScrollPattern, windows_core::IUnknown);
impl IUIAutomationScrollPattern {
    pub unsafe fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), horizontalamount, verticalamount).ok()
    }
    pub unsafe fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScrollPercent)(windows_core::Interface::as_raw(self), horizontalpercent, verticalpercent).ok()
    }
    pub unsafe fn CurrentHorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentHorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentVerticalScrollPercent(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentVerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentHorizontalViewSize(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentHorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentVerticalViewSize(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentVerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentHorizontallyScrollable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentHorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentVerticallyScrollable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentVerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedHorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedHorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedVerticalScrollPercent(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedVerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedHorizontalViewSize(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedHorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedVerticalViewSize(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedVerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedHorizontallyScrollable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedHorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedVerticallyScrollable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedVerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationScrollPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, ScrollAmount, ScrollAmount) -> windows_core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub CurrentHorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentVerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentHorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentVerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentHorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentVerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedHorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedVerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedHorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedVerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedHorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedVerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationSelectionItemPattern, IUIAutomationSelectionItemPattern_Vtbl, 0xa8efa66a_0fda_421a_9194_38021f3578ea);
impl core::ops::Deref for IUIAutomationSelectionItemPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationSelectionItemPattern, windows_core::IUnknown);
impl IUIAutomationSelectionItemPattern {
    pub unsafe fn Select(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddToSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveFromSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CurrentIsSelected(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentSelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedIsSelected(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedSelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationSelectionItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentSelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedSelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationSelectionPattern, IUIAutomationSelectionPattern_Vtbl, 0x5ed5202e_b2ac_47a6_b638_4b0bf140d78e);
impl core::ops::Deref for IUIAutomationSelectionPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationSelectionPattern, windows_core::IUnknown);
impl IUIAutomationSelectionPattern {
    pub unsafe fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentCanSelectMultiple(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsSelectionRequired(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedCanSelectMultiple(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsSelectionRequired(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationSelectionPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentCanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentIsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCachedSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedCanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationSelectionPattern2, IUIAutomationSelectionPattern2_Vtbl, 0x0532bfae_c011_4e32_a343_6d642d798555);
impl core::ops::Deref for IUIAutomationSelectionPattern2 {
    type Target = IUIAutomationSelectionPattern;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationSelectionPattern2, windows_core::IUnknown, IUIAutomationSelectionPattern);
impl IUIAutomationSelectionPattern2 {
    pub unsafe fn CurrentFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentItemCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedLastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedItemCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationSelectionPattern2_Vtbl {
    pub base__: IUIAutomationSelectionPattern_Vtbl,
    pub CurrentFirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentLastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentCurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedFirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedLastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedCurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationSpreadsheetItemPattern, IUIAutomationSpreadsheetItemPattern_Vtbl, 0x7d4fb86c_8d34_40e1_8e83_62c15204e335);
impl core::ops::Deref for IUIAutomationSpreadsheetItemPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationSpreadsheetItemPattern, windows_core::IUnknown);
impl IUIAutomationSpreadsheetItemPattern {
    pub unsafe fn CurrentFormula(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFormula)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedFormula(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFormula)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCachedAnnotationTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationSpreadsheetItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentFormula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCurrentAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCurrentAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCurrentAnnotationTypes: usize,
    pub CachedFormula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCachedAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCachedAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCachedAnnotationTypes: usize,
}
windows_core::imp::define_interface!(IUIAutomationSpreadsheetPattern, IUIAutomationSpreadsheetPattern_Vtbl, 0x7517a7c8_faae_4de9_9f08_29b91e8595c1);
impl core::ops::Deref for IUIAutomationSpreadsheetPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationSpreadsheetPattern, windows_core::IUnknown);
impl IUIAutomationSpreadsheetPattern {
    pub unsafe fn GetItemByName<P0>(&self, name: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemByName)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationSpreadsheetPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationStructureChangedEventHandler, IUIAutomationStructureChangedEventHandler_Vtbl, 0xe81d1b4e_11c5_42f8_9754_e7036c79f054);
impl core::ops::Deref for IUIAutomationStructureChangedEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationStructureChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationStructureChangedEventHandler {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleStructureChangedEvent<P0>(&self, sender: P0, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        (windows_core::Interface::vtable(self).HandleStructureChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), changetype, runtimeid).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationStructureChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HandleStructureChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, StructureChangeType, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HandleStructureChangedEvent: usize,
}
windows_core::imp::define_interface!(IUIAutomationStylesPattern, IUIAutomationStylesPattern_Vtbl, 0x85b5f0a2_bd79_484a_ad2b_388c9838d5fb);
impl core::ops::Deref for IUIAutomationStylesPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationStylesPattern, windows_core::IUnknown);
impl IUIAutomationStylesPattern {
    pub unsafe fn CurrentStyleId(&self) -> windows_core::Result<UIA_STYLE_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentStyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentStyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentStyleName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentFillColor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentShape(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentShape)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentFillPatternColor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentExtendedPropertiesAsArray)(windows_core::Interface::as_raw(self), propertyarray, propertycount).ok()
    }
    pub unsafe fn CachedStyleId(&self) -> windows_core::Result<UIA_STYLE_ID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedStyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedStyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedStyleName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedFillColor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedShape(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedShape)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedFillPatternColor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedFillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCachedExtendedPropertiesAsArray)(windows_core::Interface::as_raw(self), propertyarray, propertycount).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationStylesPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentStyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_STYLE_ID) -> windows_core::HRESULT,
    pub CurrentStyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentFillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentFillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentShape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentFillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCurrentExtendedPropertiesAsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut ExtendedProperty, *mut i32) -> windows_core::HRESULT,
    pub CachedStyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_STYLE_ID) -> windows_core::HRESULT,
    pub CachedStyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedFillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedFillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedShape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedFillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCachedExtendedPropertiesAsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut ExtendedProperty, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationSynchronizedInputPattern, IUIAutomationSynchronizedInputPattern_Vtbl, 0x2233be0b_afb7_448b_9fda_3b378aa5eae1);
impl core::ops::Deref for IUIAutomationSynchronizedInputPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationSynchronizedInputPattern, windows_core::IUnknown);
impl IUIAutomationSynchronizedInputPattern {
    pub unsafe fn StartListening(&self, inputtype: SynchronizedInputType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartListening)(windows_core::Interface::as_raw(self), inputtype).ok()
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationSynchronizedInputPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartListening: unsafe extern "system" fn(*mut core::ffi::c_void, SynchronizedInputType) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTableItemPattern, IUIAutomationTableItemPattern_Vtbl, 0x0b964eb3_ef2e_4464_9c79_61d61737a27e);
impl core::ops::Deref for IUIAutomationTableItemPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTableItemPattern, windows_core::IUnknown);
impl IUIAutomationTableItemPattern {
    pub unsafe fn GetCurrentRowHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentColumnHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentColumnHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedRowHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedColumnHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedColumnHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationTableItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentRowHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentColumnHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedRowHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedColumnHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTablePattern, IUIAutomationTablePattern_Vtbl, 0x620e691c_ea96_4710_a850_754b24ce2417);
impl core::ops::Deref for IUIAutomationTablePattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTablePattern, windows_core::IUnknown);
impl IUIAutomationTablePattern {
    pub unsafe fn GetCurrentRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentRowOrColumnMajor(&self) -> windows_core::Result<RowOrColumnMajor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentRowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCachedRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCachedColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedRowOrColumnMajor(&self) -> windows_core::Result<RowOrColumnMajor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedRowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationTablePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentRowHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentRowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RowOrColumnMajor) -> windows_core::HRESULT,
    pub GetCachedRowHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedRowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RowOrColumnMajor) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTextChildPattern, IUIAutomationTextChildPattern_Vtbl, 0x6552b038_ae05_40c8_abfd_aa08352aab86);
impl core::ops::Deref for IUIAutomationTextChildPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextChildPattern, windows_core::IUnknown);
impl IUIAutomationTextChildPattern {
    pub unsafe fn TextContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TextContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TextRange(&self) -> windows_core::Result<IUIAutomationTextRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TextRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationTextChildPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TextContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TextRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTextEditPattern, IUIAutomationTextEditPattern_Vtbl, 0x17e21576_996c_4870_99d9_bff323380c06);
impl core::ops::Deref for IUIAutomationTextEditPattern {
    type Target = IUIAutomationTextPattern;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextEditPattern, windows_core::IUnknown, IUIAutomationTextPattern);
impl IUIAutomationTextEditPattern {
    pub unsafe fn GetActiveComposition(&self) -> windows_core::Result<IUIAutomationTextRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveComposition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConversionTarget(&self) -> windows_core::Result<IUIAutomationTextRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConversionTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationTextEditPattern_Vtbl {
    pub base__: IUIAutomationTextPattern_Vtbl,
    pub GetActiveComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTextEditTextChangedEventHandler, IUIAutomationTextEditTextChangedEventHandler_Vtbl, 0x92faa680_e704_4156_931a_e32d5bb38f3f);
impl core::ops::Deref for IUIAutomationTextEditTextChangedEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextEditTextChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationTextEditTextChangedEventHandler {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleTextEditTextChangedEvent<P0>(&self, sender: P0, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        (windows_core::Interface::vtable(self).HandleTextEditTextChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), texteditchangetype, eventstrings).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationTextEditTextChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HandleTextEditTextChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TextEditChangeType, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HandleTextEditTextChangedEvent: usize,
}
windows_core::imp::define_interface!(IUIAutomationTextPattern, IUIAutomationTextPattern_Vtbl, 0x32eba289_3583_42c9_9c59_3b6d9a1e9b6a);
impl core::ops::Deref for IUIAutomationTextPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextPattern, windows_core::IUnknown);
impl IUIAutomationTextPattern {
    pub unsafe fn RangeFromPoint(&self, pt: super::super::Foundation::POINT) -> windows_core::Result<IUIAutomationTextRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RangeFromPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(pt), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RangeFromChild<P0>(&self, child: P0) -> windows_core::Result<IUIAutomationTextRange>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RangeFromChild)(windows_core::Interface::as_raw(self), child.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSelection(&self) -> windows_core::Result<IUIAutomationTextRangeArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetVisibleRanges(&self) -> windows_core::Result<IUIAutomationTextRangeArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisibleRanges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DocumentRange(&self) -> windows_core::Result<IUIAutomationTextRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DocumentRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SupportedTextSelection(&self) -> windows_core::Result<SupportedTextSelection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedTextSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationTextPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RangeFromChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVisibleRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportedTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SupportedTextSelection) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTextPattern2, IUIAutomationTextPattern2_Vtbl, 0x506a921a_fcc9_409f_b23b_37eb74106872);
impl core::ops::Deref for IUIAutomationTextPattern2 {
    type Target = IUIAutomationTextPattern;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextPattern2, windows_core::IUnknown, IUIAutomationTextPattern);
impl IUIAutomationTextPattern2 {
    pub unsafe fn RangeFromAnnotation<P0>(&self, annotation: P0) -> windows_core::Result<IUIAutomationTextRange>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RangeFromAnnotation)(windows_core::Interface::as_raw(self), annotation.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCaretRange(&self, isactive: *mut super::super::Foundation::BOOL) -> windows_core::Result<IUIAutomationTextRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCaretRange)(windows_core::Interface::as_raw(self), isactive, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationTextPattern2_Vtbl {
    pub base__: IUIAutomationTextPattern_Vtbl,
    pub RangeFromAnnotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaretRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTextRange, IUIAutomationTextRange_Vtbl, 0xa543cc6a_f4ae_494b_8239_c814481187a8);
impl core::ops::Deref for IUIAutomationTextRange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextRange, windows_core::IUnknown);
impl IUIAutomationTextRange {
    pub unsafe fn Clone(&self) -> windows_core::Result<IUIAutomationTextRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Compare<P0>(&self, range: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<IUIAutomationTextRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), range.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CompareEndpoints<P0>(&self, srcendpoint: TextPatternRangeEndpoint, range: P0, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IUIAutomationTextRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareEndpoints)(windows_core::Interface::as_raw(self), srcendpoint, range.param().abi(), targetendpoint, &mut result__).map(|| result__)
    }
    pub unsafe fn ExpandToEnclosingUnit(&self, textunit: TextUnit) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExpandToEnclosingUnit)(windows_core::Interface::as_raw(self), textunit).ok()
    }
    pub unsafe fn FindAttribute<P0, P1>(&self, attr: UIA_TEXTATTRIBUTE_ID, val: P0, backward: P1) -> windows_core::Result<IUIAutomationTextRange>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindAttribute)(windows_core::Interface::as_raw(self), attr, val.param().abi(), backward.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindText<P0, P1, P2>(&self, text: P0, backward: P1, ignorecase: P2) -> windows_core::Result<IUIAutomationTextRange>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindText)(windows_core::Interface::as_raw(self), text.param().abi(), backward.param().abi(), ignorecase.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAttributeValue(&self, attr: UIA_TEXTATTRIBUTE_ID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttributeValue)(windows_core::Interface::as_raw(self), attr, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBoundingRectangles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEnclosingElement(&self) -> windows_core::Result<IUIAutomationElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnclosingElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), maxlength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Move(&self, unit: TextUnit, count: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
    }
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveEndpointByUnit)(windows_core::Interface::as_raw(self), endpoint, unit, count, &mut result__).map(|| result__)
    }
    pub unsafe fn MoveEndpointByRange<P0>(&self, srcendpoint: TextPatternRangeEndpoint, range: P0, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAutomationTextRange>,
    {
        (windows_core::Interface::vtable(self).MoveEndpointByRange)(windows_core::Interface::as_raw(self), srcendpoint, range.param().abi(), targetendpoint).ok()
    }
    pub unsafe fn Select(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddToSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveFromSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ScrollIntoView<P0>(&self, aligntotop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self), aligntotop.param().abi()).ok()
    }
    pub unsafe fn GetChildren(&self) -> windows_core::Result<IUIAutomationElementArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationTextRange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CompareEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, *mut core::ffi::c_void, TextPatternRangeEndpoint, *mut i32) -> windows_core::HRESULT,
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(*mut core::ffi::c_void, TextUnit) -> windows_core::HRESULT,
    pub FindAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_TEXTATTRIBUTE_ID, core::mem::MaybeUninit<windows_core::VARIANT>, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_TEXTATTRIBUTE_ID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBoundingRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBoundingRectangles: usize,
    pub GetEnclosingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEndpointByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEndpointByRange: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, *mut core::ffi::c_void, TextPatternRangeEndpoint) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTextRange2, IUIAutomationTextRange2_Vtbl, 0xbb9b40e0_5e04_46bd_9be0_4b601b9afad4);
impl core::ops::Deref for IUIAutomationTextRange2 {
    type Target = IUIAutomationTextRange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextRange2, windows_core::IUnknown, IUIAutomationTextRange);
impl IUIAutomationTextRange2 {
    pub unsafe fn ShowContextMenu(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationTextRange2_Vtbl {
    pub base__: IUIAutomationTextRange_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTextRange3, IUIAutomationTextRange3_Vtbl, 0x6a315d69_5512_4c2e_85f0_53fce6dd4bc2);
impl core::ops::Deref for IUIAutomationTextRange3 {
    type Target = IUIAutomationTextRange2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextRange3, windows_core::IUnknown, IUIAutomationTextRange, IUIAutomationTextRange2);
impl IUIAutomationTextRange3 {
    pub unsafe fn GetEnclosingElementBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnclosingElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetChildrenBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElementArray>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChildrenBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAttributeValues(&self, attributeids: &[UIA_TEXTATTRIBUTE_ID]) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttributeValues)(windows_core::Interface::as_raw(self), core::mem::transmute(attributeids.as_ptr()), attributeids.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationTextRange3_Vtbl {
    pub base__: IUIAutomationTextRange2_Vtbl,
    pub GetEnclosingElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChildrenBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAttributeValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIA_TEXTATTRIBUTE_ID, i32, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAttributeValues: usize,
}
windows_core::imp::define_interface!(IUIAutomationTextRangeArray, IUIAutomationTextRangeArray_Vtbl, 0xce4ae76a_e717_4c98_81ea_47371d028eb6);
impl core::ops::Deref for IUIAutomationTextRangeArray {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextRangeArray, windows_core::IUnknown);
impl IUIAutomationTextRangeArray {
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationTextRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationTextRangeArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTogglePattern, IUIAutomationTogglePattern_Vtbl, 0x94cf8058_9b8d_4ab9_8bfd_4cd0a33c8c70);
impl core::ops::Deref for IUIAutomationTogglePattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTogglePattern, windows_core::IUnknown);
impl IUIAutomationTogglePattern {
    pub unsafe fn Toggle(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Toggle)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CurrentToggleState(&self) -> windows_core::Result<ToggleState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedToggleState(&self) -> windows_core::Result<ToggleState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationTogglePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Toggle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToggleState) -> windows_core::HRESULT,
    pub CachedToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToggleState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTransformPattern, IUIAutomationTransformPattern_Vtbl, 0xa9b55844_a55d_4ef0_926d_569c16ff89bb);
impl core::ops::Deref for IUIAutomationTransformPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTransformPattern, windows_core::IUnknown);
impl IUIAutomationTransformPattern {
    pub unsafe fn Move(&self, x: f64, y: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), x, y).ok()
    }
    pub unsafe fn Resize(&self, width: f64, height: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Rotate)(windows_core::Interface::as_raw(self), degrees).ok()
    }
    pub unsafe fn CurrentCanMove(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentCanResize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentCanRotate(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedCanMove(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedCanResize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedCanRotate(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationTransformPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CurrentCanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentCanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentCanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedCanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedCanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedCanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTransformPattern2, IUIAutomationTransformPattern2_Vtbl, 0x6d74d017_6ecb_4381_b38b_3c17a48ff1c2);
impl core::ops::Deref for IUIAutomationTransformPattern2 {
    type Target = IUIAutomationTransformPattern;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTransformPattern2, windows_core::IUnknown, IUIAutomationTransformPattern);
impl IUIAutomationTransformPattern2 {
    pub unsafe fn Zoom(&self, zoomvalue: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Zoom)(windows_core::Interface::as_raw(self), zoomvalue).ok()
    }
    pub unsafe fn ZoomByUnit(&self, zoomunit: ZoomUnit) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ZoomByUnit)(windows_core::Interface::as_raw(self), zoomunit).ok()
    }
    pub unsafe fn CurrentCanZoom(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedCanZoom(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentZoomLevel(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedZoomLevel(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentZoomMinimum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedZoomMinimum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentZoomMaximum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedZoomMaximum(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationTransformPattern2_Vtbl {
    pub base__: IUIAutomationTransformPattern_Vtbl,
    pub Zoom: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, ZoomUnit) -> windows_core::HRESULT,
    pub CurrentCanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedCanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationTreeWalker, IUIAutomationTreeWalker_Vtbl, 0x4042c624_389c_4afc_a630_9df854a541fc);
impl core::ops::Deref for IUIAutomationTreeWalker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTreeWalker, windows_core::IUnknown);
impl IUIAutomationTreeWalker {
    pub unsafe fn GetParentElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParentElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFirstChildElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFirstChildElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLastChildElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastChildElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNextSiblingElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextSiblingElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPreviousSiblingElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreviousSiblingElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NormalizeElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NormalizeElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetParentElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParentElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFirstChildElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFirstChildElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLastChildElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastChildElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNextSiblingElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextSiblingElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPreviousSiblingElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreviousSiblingElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NormalizeElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NormalizeElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Condition(&self) -> windows_core::Result<IUIAutomationCondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Condition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAutomationTreeWalker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetParentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstChildElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastChildElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextSiblingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviousSiblingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NormalizeElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParentElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstChildElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastChildElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextSiblingElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviousSiblingElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NormalizeElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationValuePattern, IUIAutomationValuePattern_Vtbl, 0xa94cd8b1_0844_4cd6_9d2d_640537ab39e9);
impl core::ops::Deref for IUIAutomationValuePattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationValuePattern, windows_core::IUnknown);
impl IUIAutomationValuePattern {
    pub unsafe fn SetValue<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn CurrentValue(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentIsReadOnly(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedValue(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CachedIsReadOnly(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationValuePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CachedIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationVirtualizedItemPattern, IUIAutomationVirtualizedItemPattern_Vtbl, 0x6ba3d7a6_04cf_4f11_8793_a8d1cde9969f);
impl core::ops::Deref for IUIAutomationVirtualizedItemPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationVirtualizedItemPattern, windows_core::IUnknown);
impl IUIAutomationVirtualizedItemPattern {
    pub unsafe fn Realize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Realize)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAutomationVirtualizedItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Realize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIAutomationWindowPattern, IUIAutomationWindowPattern_Vtbl, 0x0faef453_9208_43ef_bbb2_3b485177864f);
impl core::ops::Deref for IUIAutomationWindowPattern {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationWindowPattern, windows_core::IUnknown);
impl IUIAutomationWindowPattern {
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForInputIdle(&self, milliseconds: i32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitForInputIdle)(windows_core::Interface::as_raw(self), milliseconds, &mut result__).map(|| result__)
    }
    pub unsafe fn SetWindowVisualState(&self, state: WindowVisualState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWindowVisualState)(windows_core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn CurrentCanMaximize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentCanMinimize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsModal(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentIsTopmost(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentIsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentWindowVisualState(&self) -> windows_core::Result<WindowVisualState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentWindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentWindowInteractionState(&self) -> windows_core::Result<WindowInteractionState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentWindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedCanMaximize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedCanMinimize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedCanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsModal(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedIsTopmost(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedIsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedWindowVisualState(&self) -> windows_core::Result<WindowVisualState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedWindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CachedWindowInteractionState(&self) -> windows_core::Result<WindowInteractionState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CachedWindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAutomationWindowPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForInputIdle: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, WindowVisualState) -> windows_core::HRESULT,
    pub CurrentCanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentCanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentIsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentIsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CurrentWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowVisualState) -> windows_core::HRESULT,
    pub CurrentWindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowInteractionState) -> windows_core::HRESULT,
    pub CachedCanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedCanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedIsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CachedWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowVisualState) -> windows_core::HRESULT,
    pub CachedWindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowInteractionState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IValueProvider, IValueProvider_Vtbl, 0xc7935180_6fb3_4201_b174_7df73adbf64a);
impl core::ops::Deref for IValueProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IValueProvider, windows_core::IUnknown);
impl IValueProvider {
    pub unsafe fn SetValue<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsReadOnly(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IValueProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVirtualizedItemProvider, IVirtualizedItemProvider_Vtbl, 0xcb98b665_2d35_4fac_ad35_f3c60d0c0b8b);
impl core::ops::Deref for IVirtualizedItemProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVirtualizedItemProvider, windows_core::IUnknown);
impl IVirtualizedItemProvider {
    pub unsafe fn Realize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Realize)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IVirtualizedItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Realize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowProvider, IWindowProvider_Vtbl, 0x987df77b_db06_4d77_8f8a_86a9c3bb90b9);
impl core::ops::Deref for IWindowProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWindowProvider, windows_core::IUnknown);
impl IWindowProvider {
    pub unsafe fn SetVisualState(&self, state: WindowVisualState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVisualState)(windows_core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForInputIdle(&self, milliseconds: i32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitForInputIdle)(windows_core::Interface::as_raw(self), milliseconds, &mut result__).map(|| result__)
    }
    pub unsafe fn CanMaximize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CanMinimize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsModal(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WindowVisualState(&self) -> windows_core::Result<WindowVisualState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WindowInteractionState(&self) -> windows_core::Result<WindowInteractionState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsTopmost(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWindowProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, WindowVisualState) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForInputIdle: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub WindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowVisualState) -> windows_core::HRESULT,
    pub WindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowInteractionState) -> windows_core::HRESULT,
    pub IsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub const ANNO_CONTAINER: AnnoScope = AnnoScope(1i32);
pub const ANNO_THIS: AnnoScope = AnnoScope(0i32);
pub const ANRUS_ON_SCREEN_KEYBOARD_ACTIVE: ACC_UTILITY_STATE_FLAGS = ACC_UTILITY_STATE_FLAGS(1u32);
pub const ANRUS_PRIORITY_AUDIO_ACTIVE: ACC_UTILITY_STATE_FLAGS = ACC_UTILITY_STATE_FLAGS(4u32);
pub const ANRUS_PRIORITY_AUDIO_ACTIVE_NODUCK: ACC_UTILITY_STATE_FLAGS = ACC_UTILITY_STATE_FLAGS(8u32);
pub const ANRUS_PRIORITY_AUDIO_DYNAMIC_DUCK: u32 = 16u32;
pub const ANRUS_TOUCH_MODIFICATION_ACTIVE: ACC_UTILITY_STATE_FLAGS = ACC_UTILITY_STATE_FLAGS(2u32);
pub const AcceleratorKey_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x514865df_2557_4cb9_aeed_6ced084ce52c);
pub const AccessKey_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x06827b12_a7f9_4a15_917c_ffa5ad3eb0a7);
pub const ActiveEnd_End: ActiveEnd = ActiveEnd(2i32);
pub const ActiveEnd_None: ActiveEnd = ActiveEnd(0i32);
pub const ActiveEnd_Start: ActiveEnd = ActiveEnd(1i32);
pub const ActiveTextPositionChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa5c09e9c_c77d_4f25_b491_e5bb7017cbd4);
pub const AnimationStyle_BlinkingBackground: AnimationStyle = AnimationStyle(2i32);
pub const AnimationStyle_LasVegasLights: AnimationStyle = AnimationStyle(1i32);
pub const AnimationStyle_MarchingBlackAnts: AnimationStyle = AnimationStyle(4i32);
pub const AnimationStyle_MarchingRedAnts: AnimationStyle = AnimationStyle(5i32);
pub const AnimationStyle_None: AnimationStyle = AnimationStyle(0i32);
pub const AnimationStyle_Other: AnimationStyle = AnimationStyle(-1i32);
pub const AnimationStyle_Shimmer: AnimationStyle = AnimationStyle(6i32);
pub const AnimationStyle_SparkleText: AnimationStyle = AnimationStyle(3i32);
pub const AnnotationObjects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x310910c8_7c6e_4f20_becd_4aaf6d191156);
pub const AnnotationType_AdvancedProofingIssue: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60020i32);
pub const AnnotationType_Author: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60019i32);
pub const AnnotationType_CircularReferenceError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60022i32);
pub const AnnotationType_Comment: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60003i32);
pub const AnnotationType_ConflictingChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60018i32);
pub const AnnotationType_DataValidationError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60021i32);
pub const AnnotationType_DeletionChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60012i32);
pub const AnnotationType_EditingLockedChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60016i32);
pub const AnnotationType_Endnote: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60009i32);
pub const AnnotationType_ExternalChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60017i32);
pub const AnnotationType_Footer: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60007i32);
pub const AnnotationType_Footnote: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60010i32);
pub const AnnotationType_FormatChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60014i32);
pub const AnnotationType_FormulaError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60004i32);
pub const AnnotationType_GrammarError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60002i32);
pub const AnnotationType_Header: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60006i32);
pub const AnnotationType_Highlighted: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60008i32);
pub const AnnotationType_InsertionChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60011i32);
pub const AnnotationType_Mathematics: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60023i32);
pub const AnnotationType_MoveChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60013i32);
pub const AnnotationType_Sensitive: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60024i32);
pub const AnnotationType_SpellingError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60001i32);
pub const AnnotationType_TrackChanges: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60005i32);
pub const AnnotationType_Unknown: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60000i32);
pub const AnnotationType_UnsyncedChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60015i32);
pub const AnnotationTypes_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x64b71f76_53c4_4696_a219_20e940c9a176);
pub const Annotation_AdvancedProofingIssue_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdac7b72c_c0f2_4b84_b90d_5fafc0f0ef1c);
pub const Annotation_AnnotationTypeId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x20ae484f_69ef_4c48_8f5b_c4938b206ac7);
pub const Annotation_AnnotationTypeName_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9b818892_5ac9_4af9_aa96_f58a77b058e3);
pub const Annotation_Author_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf161d3a7_f81b_4128_b17f_71f690914520);
pub const Annotation_Author_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7a528462_9c5c_4a03_a974_8b307a9937f2);
pub const Annotation_CircularReferenceError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x25bd9cf4_1745_4659_ba67_727f0318c616);
pub const Annotation_Comment_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfd2fda30_26b3_4c06_8bc7_98f1532e46fd);
pub const Annotation_ConflictingChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x98af8802_517c_459f_af13_016d3fab877e);
pub const Annotation_Custom_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9ec82750_3931_4952_85bc_1dbff78a43e3);
pub const Annotation_DataValidationError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc8649fa8_9775_437e_ad46_e709d93c2343);
pub const Annotation_DateTime_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x99b5ca5d_1acf_414b_a4d0_6b350b047578);
pub const Annotation_DeletionChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbe3d5b05_951d_42e7_901d_adc8c2cf34d0);
pub const Annotation_EditingLockedChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc31f3e1c_7423_4dac_8348_41f099ff6f64);
pub const Annotation_Endnote_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7565725c_2d99_4839_960d_33d3b866aba5);
pub const Annotation_ExternalChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x75a05b31_5f11_42fd_887d_dfa010db2392);
pub const Annotation_Footer_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcceab046_1833_47aa_8080_701ed0b0c832);
pub const Annotation_Footnote_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3de10e21_4125_42db_8620_be8083080624);
pub const Annotation_FormatChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeb247345_d4f1_41ce_8e52_f79b69635e48);
pub const Annotation_FormulaError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x95611982_0cab_46d5_a2f0_e30d1905f8bf);
pub const Annotation_GrammarError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x757a048d_4518_41c6_854c_dc009b7cfb53);
pub const Annotation_Header_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x867b409b_b216_4472_a219_525e310681f8);
pub const Annotation_Highlighted_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x757c884e_8083_4081_8b9c_e87f5072f0e4);
pub const Annotation_InsertionChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0dbeb3a6_df15_4164_a3c0_e21a8ce931c4);
pub const Annotation_Mathematics_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeaab634b_26d0_40c1_8073_57ca1c633c9b);
pub const Annotation_MoveChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9da587eb_23e5_4490_b385_1a22ddc8b187);
pub const Annotation_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf6c72ad7_356c_4850_9291_316f608a8c84);
pub const Annotation_Sensitive_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x37f4c04f_0f12_4464_929c_828fd15292e3);
pub const Annotation_SpellingError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xae85567e_9ece_423f_81b7_96c43d53e50e);
pub const Annotation_Target_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb71b302d_2104_44ad_9c5c_092b4907d70f);
pub const Annotation_TrackChanges_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x21e6e888_dc14_4016_ac27_190553c8c470);
pub const Annotation_UnsyncedChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1851116a_0e47_4b30_8cb5_d7dae4fbcd1b);
pub const AppBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6114908d_cc02_4d37_875b_b530c7139554);
pub const AriaProperties_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4213678c_e025_4922_beb5_e43ba08e6221);
pub const AriaRole_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdd207b95_be4a_4e0d_b727_63ace94b6916);
pub const Assertive: LiveSetting = LiveSetting(2i32);
pub const AsyncContentLoadedState_Beginning: AsyncContentLoadedState = AsyncContentLoadedState(0i32);
pub const AsyncContentLoadedState_Completed: AsyncContentLoadedState = AsyncContentLoadedState(2i32);
pub const AsyncContentLoadedState_Progress: AsyncContentLoadedState = AsyncContentLoadedState(1i32);
pub const AsyncContentLoaded_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5fdee11c_d2fa_4fb9_904e_5cbee894d5ef);
pub const AutomationElementMode_Full: AutomationElementMode = AutomationElementMode(1i32);
pub const AutomationElementMode_None: AutomationElementMode = AutomationElementMode(0i32);
pub const AutomationFocusChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb68a1f17_f60d_41a7_a3cc_b05292155fe0);
pub const AutomationId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc82c0500_b60e_4310_a267_303c531f8ee5);
pub const AutomationIdentifierType_Annotation: AutomationIdentifierType = AutomationIdentifierType(6i32);
pub const AutomationIdentifierType_Changes: AutomationIdentifierType = AutomationIdentifierType(7i32);
pub const AutomationIdentifierType_ControlType: AutomationIdentifierType = AutomationIdentifierType(3i32);
pub const AutomationIdentifierType_Event: AutomationIdentifierType = AutomationIdentifierType(2i32);
pub const AutomationIdentifierType_LandmarkType: AutomationIdentifierType = AutomationIdentifierType(5i32);
pub const AutomationIdentifierType_Pattern: AutomationIdentifierType = AutomationIdentifierType(1i32);
pub const AutomationIdentifierType_Property: AutomationIdentifierType = AutomationIdentifierType(0i32);
pub const AutomationIdentifierType_Style: AutomationIdentifierType = AutomationIdentifierType(8i32);
pub const AutomationIdentifierType_TextAttribute: AutomationIdentifierType = AutomationIdentifierType(4i32);
pub const AutomationPropertyChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2527fba1_8d7a_4630_a4cc_e66315942f52);
pub const BoundingRectangle_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7bbfe8b2_3bfc_48dd_b729_c794b846e9a1);
pub const BulletStyle_DashBullet: BulletStyle = BulletStyle(5i32);
pub const BulletStyle_FilledRoundBullet: BulletStyle = BulletStyle(2i32);
pub const BulletStyle_FilledSquareBullet: BulletStyle = BulletStyle(4i32);
pub const BulletStyle_HollowRoundBullet: BulletStyle = BulletStyle(1i32);
pub const BulletStyle_HollowSquareBullet: BulletStyle = BulletStyle(3i32);
pub const BulletStyle_None: BulletStyle = BulletStyle(0i32);
pub const BulletStyle_Other: BulletStyle = BulletStyle(-1i32);
pub const Button_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5a78e369_c6a1_4f33_a9d7_79f20d0c788e);
pub const CLSID_AccPropServices: windows_core::GUID = windows_core::GUID::from_u128(0xb5f8350b_0548_48b1_a6ee_88bd00b4a5e7);
pub const Calendar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8913eb88_00e5_46bc_8e4e_14a786e165a1);
pub const CapStyle_AllCap: CapStyle = CapStyle(2i32);
pub const CapStyle_AllPetiteCaps: CapStyle = CapStyle(3i32);
pub const CapStyle_None: CapStyle = CapStyle(0i32);
pub const CapStyle_Other: CapStyle = CapStyle(-1i32);
pub const CapStyle_PetiteCaps: CapStyle = CapStyle(4i32);
pub const CapStyle_SmallCap: CapStyle = CapStyle(1i32);
pub const CapStyle_Titling: CapStyle = CapStyle(6i32);
pub const CapStyle_Unicase: CapStyle = CapStyle(5i32);
pub const CaretBidiMode_LTR: CaretBidiMode = CaretBidiMode(0i32);
pub const CaretBidiMode_RTL: CaretBidiMode = CaretBidiMode(1i32);
pub const CaretPosition_BeginningOfLine: CaretPosition = CaretPosition(2i32);
pub const CaretPosition_EndOfLine: CaretPosition = CaretPosition(1i32);
pub const CaretPosition_Unknown: CaretPosition = CaretPosition(0i32);
pub const CenterPoint_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0cb00c08_540c_4edb_9445_26359ea69785);
pub const Changes_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7df26714_614f_4e05_9488_716c5ba19436);
pub const Changes_Summary_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x313d65a6_e60f_4d62_9861_55afd728d207);
pub const CheckBox_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfb50f922_a3db_49c0_8bc3_06dad55778e2);
pub const ClassName_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x157b7215_894f_4b65_84e2_aac0da08b16b);
pub const ClickablePoint_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0196903b_b203_4818_a9f3_f08e675f2341);
pub const CoalesceEventsOptions_Disabled: CoalesceEventsOptions = CoalesceEventsOptions(0i32);
pub const CoalesceEventsOptions_Enabled: CoalesceEventsOptions = CoalesceEventsOptions(1i32);
pub const ComboBox_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x54cb426c_2f33_4fff_aaa1_aef60dac5deb);
pub const ConditionType_And: ConditionType = ConditionType(3i32);
pub const ConditionType_False: ConditionType = ConditionType(1i32);
pub const ConditionType_Not: ConditionType = ConditionType(5i32);
pub const ConditionType_Or: ConditionType = ConditionType(4i32);
pub const ConditionType_Property: ConditionType = ConditionType(2i32);
pub const ConditionType_True: ConditionType = ConditionType(0i32);
pub const ConnectionRecoveryBehaviorOptions_Disabled: ConnectionRecoveryBehaviorOptions = ConnectionRecoveryBehaviorOptions(0i32);
pub const ConnectionRecoveryBehaviorOptions_Enabled: ConnectionRecoveryBehaviorOptions = ConnectionRecoveryBehaviorOptions(1i32);
pub const ControlType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xca774fea_28ac_4bc2_94ca_acec6d6c10a3);
pub const ControllerFor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x51124c8a_a5d2_4f13_9be6_7fa8ba9d3a90);
pub const Culture_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe2d74f27_3d79_4dc2_b88b_3044963a8afb);
pub const CustomNavigation_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xafea938a_621e_4054_bb2c_2f46114dac3f);
pub const Custom_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf29ea0c3_adb7_430a_ba90_e52c7313e6ed);
pub const DISPID_ACC_CHILD: i32 = -5002i32;
pub const DISPID_ACC_CHILDCOUNT: i32 = -5001i32;
pub const DISPID_ACC_DEFAULTACTION: i32 = -5013i32;
pub const DISPID_ACC_DESCRIPTION: i32 = -5005i32;
pub const DISPID_ACC_DODEFAULTACTION: i32 = -5018i32;
pub const DISPID_ACC_FOCUS: i32 = -5011i32;
pub const DISPID_ACC_HELP: i32 = -5008i32;
pub const DISPID_ACC_HELPTOPIC: i32 = -5009i32;
pub const DISPID_ACC_HITTEST: i32 = -5017i32;
pub const DISPID_ACC_KEYBOARDSHORTCUT: i32 = -5010i32;
pub const DISPID_ACC_LOCATION: i32 = -5015i32;
pub const DISPID_ACC_NAME: i32 = -5003i32;
pub const DISPID_ACC_NAVIGATE: i32 = -5016i32;
pub const DISPID_ACC_PARENT: i32 = -5000i32;
pub const DISPID_ACC_ROLE: i32 = -5006i32;
pub const DISPID_ACC_SELECT: i32 = -5014i32;
pub const DISPID_ACC_SELECTION: i32 = -5012i32;
pub const DISPID_ACC_STATE: i32 = -5007i32;
pub const DISPID_ACC_VALUE: i32 = -5004i32;
pub const DataGrid_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x84b783af_d103_4b0a_8415_e73942410f4b);
pub const DataItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa0177842_d94f_42a5_814b_6068addc8da5);
pub const DescribedBy_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7c5865b8_9992_40fd_8db0_6bf1d317f998);
pub const DockPosition_Bottom: DockPosition = DockPosition(2i32);
pub const DockPosition_Fill: DockPosition = DockPosition(4i32);
pub const DockPosition_Left: DockPosition = DockPosition(1i32);
pub const DockPosition_None: DockPosition = DockPosition(5i32);
pub const DockPosition_Right: DockPosition = DockPosition(3i32);
pub const DockPosition_Top: DockPosition = DockPosition(0i32);
pub const Dock_DockPosition_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6d67f02e_c0b0_4b10_b5b9_18d6ecf98760);
pub const Dock_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9cbaa846_83c8_428d_827f_7e6063fe0620);
pub const Document_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3cd6bb6f_6f08_4562_b229_e4e2fc7a9eb4);
pub const Drag_DragCancel_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc3ede6fa_3451_4e0f_9e71_df9c280a4657);
pub const Drag_DragComplete_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x38e96188_ef1f_463e_91ca_3a7792c29caf);
pub const Drag_DragStart_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x883a480b_3aa9_429d_95e4_d9c8d011f0dd);
pub const Drag_DropEffect_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x646f2779_48d3_4b23_8902_4bf100005df3);
pub const Drag_DropEffects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf5d61156_7ce6_49be_a836_9269dcec920f);
pub const Drag_GrabbedItems_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x77c1562c_7b86_4b21_9ed7_3cefda6f4c43);
pub const Drag_IsGrabbed_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x45f206f3_75cc_4cca_a9b9_fcdfb982d8a2);
pub const Drag_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc0bee21f_ccb3_4fed_995b_114f6e3d2728);
pub const DropTarget_DragEnter_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaad9319b_032c_4a88_961d_1cf579581e34);
pub const DropTarget_DragLeave_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0f82eb15_24a2_4988_9217_de162aee272b);
pub const DropTarget_DropTargetEffect_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8bb75975_a0ca_4981_b818_87fc66e9509d);
pub const DropTarget_DropTargetEffects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbc1dd4ed_cb89_45f1_a592_e03b08ae790f);
pub const DropTarget_Dropped_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x622cead8_1edb_4a3d_abbc_be2211ff68b5);
pub const DropTarget_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0bcbec56_bd34_4b7b_9fd5_2659905ea3dc);
pub const Edit_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6504a5c8_2c86_4f87_ae7b_1abddc810cf9);
pub const EventArgsType_ActiveTextPositionChanged: EventArgsType = EventArgsType(8i32);
pub const EventArgsType_AsyncContentLoaded: EventArgsType = EventArgsType(3i32);
pub const EventArgsType_Changes: EventArgsType = EventArgsType(6i32);
pub const EventArgsType_Notification: EventArgsType = EventArgsType(7i32);
pub const EventArgsType_PropertyChanged: EventArgsType = EventArgsType(1i32);
pub const EventArgsType_Simple: EventArgsType = EventArgsType(0i32);
pub const EventArgsType_StructureChanged: EventArgsType = EventArgsType(2i32);
pub const EventArgsType_StructuredMarkup: EventArgsType = EventArgsType(9i32);
pub const EventArgsType_TextEditTextChanged: EventArgsType = EventArgsType(5i32);
pub const EventArgsType_WindowClosed: EventArgsType = EventArgsType(4i32);
pub const ExpandCollapseState_Collapsed: ExpandCollapseState = ExpandCollapseState(0i32);
pub const ExpandCollapseState_Expanded: ExpandCollapseState = ExpandCollapseState(1i32);
pub const ExpandCollapseState_LeafNode: ExpandCollapseState = ExpandCollapseState(3i32);
pub const ExpandCollapseState_PartiallyExpanded: ExpandCollapseState = ExpandCollapseState(2i32);
pub const ExpandCollapse_ExpandCollapseState_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x275a4c48_85a7_4f69_aba0_af157610002b);
pub const ExpandCollapse_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xae05efa2_f9d1_428a_834c_53a5c52f9b8b);
pub const FillColor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6e0ec4d0_e2a8_4a56_9de7_953389933b39);
pub const FillType_Color: FillType = FillType(1i32);
pub const FillType_Gradient: FillType = FillType(2i32);
pub const FillType_None: FillType = FillType(0i32);
pub const FillType_Pattern: FillType = FillType(4i32);
pub const FillType_Picture: FillType = FillType(3i32);
pub const FillType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc6fc74e4_8cb9_429c_a9e1_9bc4ac372b62);
pub const FlowDirections_BottomToTop: FlowDirections = FlowDirections(2i32);
pub const FlowDirections_Default: FlowDirections = FlowDirections(0i32);
pub const FlowDirections_RightToLeft: FlowDirections = FlowDirections(1i32);
pub const FlowDirections_Vertical: FlowDirections = FlowDirections(4i32);
pub const FlowsFrom_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x05c6844f_19de_48f8_95fa_880d5b0fd615);
pub const FlowsTo_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe4f33d20_559a_47fb_a830_f9cb4ff1a70a);
pub const FrameworkId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdbfd9900_7e1a_4f58_b61b_7063120f773b);
pub const FullDescription_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0d4450ff_6aef_4f33_95dd_7befa72a4391);
pub const GridItem_ColumnSpan_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x583ea3f5_86d0_4b08_a6ec_2c5463ffc109);
pub const GridItem_Column_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc774c15c_62c0_4519_8bdc_47be573c8ad5);
pub const GridItem_Parent_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9d912252_b97f_4ecc_8510_ea0e33427c72);
pub const GridItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf2d5c877_a462_4957_a2a5_2c96b303bc63);
pub const GridItem_RowSpan_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4582291c_466b_4e93_8e83_3d1715ec0c5e);
pub const GridItem_Row_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6223972a_c945_4563_9329_fdc974af2553);
pub const Grid_ColumnCount_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfe96f375_44aa_4536_ac7a_2a75d71a3efc);
pub const Grid_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x260a2ccb_93a8_4e44_a4c1_3df397f2b02b);
pub const Grid_RowCount_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2a9505bf_c2eb_4fb6_b356_8245ae53703e);
pub const Group_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xad50aa1c_e8c8_4774_ae1b_dd86df0b3bdc);
pub const HCF_AVAILABLE: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(2u32);
pub const HCF_CONFIRMHOTKEY: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(8u32);
pub const HCF_HIGHCONTRASTON: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(1u32);
pub const HCF_HOTKEYACTIVE: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(4u32);
pub const HCF_HOTKEYAVAILABLE: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(64u32);
pub const HCF_HOTKEYSOUND: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(16u32);
pub const HCF_INDICATOR: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(32u32);
pub const HCF_OPTION_NOTHEMECHANGE: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(4096u32);
pub const HasKeyboardFocus_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcf8afd39_3f46_4800_9656_b2bf12529905);
pub const HeaderItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe6bc12cb_7c8e_49cf_b168_4a93a32bebb0);
pub const Header_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5b90cbce_78fb_4614_82b6_554d74718e67);
pub const HeadingLevel1: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80051i32);
pub const HeadingLevel2: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80052i32);
pub const HeadingLevel3: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80053i32);
pub const HeadingLevel4: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80054i32);
pub const HeadingLevel5: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80055i32);
pub const HeadingLevel6: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80056i32);
pub const HeadingLevel7: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80057i32);
pub const HeadingLevel8: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80058i32);
pub const HeadingLevel9: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80059i32);
pub const HeadingLevel_None: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80050i32);
pub const HeadingLevel_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x29084272_aaaf_4a30_8796_3c12f62b6bbb);
pub const HelpText_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x08555685_0977_45c7_a7a6_abaf5684121a);
pub const HorizontalTextAlignment_Centered: HorizontalTextAlignment = HorizontalTextAlignment(1i32);
pub const HorizontalTextAlignment_Justified: HorizontalTextAlignment = HorizontalTextAlignment(3i32);
pub const HorizontalTextAlignment_Left: HorizontalTextAlignment = HorizontalTextAlignment(0i32);
pub const HorizontalTextAlignment_Right: HorizontalTextAlignment = HorizontalTextAlignment(2i32);
pub const HostedFragmentRootsInvalidated_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe6bdb03e_0921_4ec5_8dcf_eae877b0426b);
pub const Hyperlink_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8a56022c_b00d_4d15_8ff0_5b6b266e5e02);
pub const IIS_ControlAccessible: windows_core::GUID = windows_core::GUID::from_u128(0x38c682a6_9731_43f2_9fae_e901e641b101);
pub const IIS_IsOleaccProxy: windows_core::GUID = windows_core::GUID::from_u128(0x902697fa_80e4_4560_802a_a13f22a64709);
pub const Image_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2d3736e4_6b16_4c57_a962_f93260a75243);
pub const InputDiscarded_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7f36c367_7b18_417c_97e3_9d58ddc944ab);
pub const InputReachedOtherElement_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xed201d8a_4e6c_415e_a874_2460c9b66ba8);
pub const InputReachedTarget_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x93ed549a_0549_40f0_bedb_28e44f7de2a3);
pub const Invoke_Invoked_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdfd699f0_c915_49dd_b422_dde785c3d24b);
pub const Invoke_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd976c2fc_66ea_4a6e_b28f_c24c7546ad37);
pub const IsAnnotationPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0b5b3238_6d5c_41b6_bcc4_5e807f6551c4);
pub const IsContentElement_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4bda64a8_f5d8_480b_8155_ef2e89adb672);
pub const IsControlElement_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x95f35085_abcc_4afd_a5f4_dbb46c230fdb);
pub const IsCustomNavigationPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8f8e80d4_2351_48e0_874a_54aa7313889a);
pub const IsDataValidForForm_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x445ac684_c3fc_4dd9_acf8_845a579296ba);
pub const IsDialog_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9d0dfb9b_8436_4501_bbbb_e534a4fb3b3f);
pub const IsDockPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2600a4c4_2ff8_4c96_ae31_8fe619a13c6c);
pub const IsDragPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe997a7b7_1d39_4ca7_be0f_277fcf5605cc);
pub const IsDropTargetPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0686b62e_8e19_4aaf_873d_384f6d3b92be);
pub const IsEnabled_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2109427f_da60_4fed_bf1b_264bdce6eb3a);
pub const IsExpandCollapsePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x929d3806_5287_4725_aa16_222afc63d595);
pub const IsGridItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5a43e524_f9a2_4b12_84c8_b48a3efedd34);
pub const IsGridPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5622c26c_f0ef_4f3b_97cb_714c0868588b);
pub const IsInvokePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4e725738_8364_4679_aa6c_f3f41931f750);
pub const IsItemContainerPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x624b5ca7_fe40_4957_a019_20c4cf11920f);
pub const IsKeyboardFocusable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf7b8552a_0859_4b37_b9cb_51e72092f29f);
pub const IsLegacyIAccessiblePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd8ebd0c7_929a_4ee7_8d3a_d3d94413027b);
pub const IsMultipleViewPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xff0a31eb_8e25_469d_8d6e_e771a27c1b90);
pub const IsObjectModelPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6b21d89b_2841_412f_8ef2_15ca952318ba);
pub const IsOffscreen_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x03c3d160_db79_42db_a2ef_1c231eede507);
pub const IsPassword_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe8482eb1_687c_497b_bebc_03be53ec1454);
pub const IsPeripheral_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xda758276_7ed5_49d4_8e68_ecc9a2d300dd);
pub const IsRangeValuePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfda4244a_eb4d_43ff_b5ad_ed36d373ec4c);
pub const IsRequiredForForm_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4f5f43cf_59fb_4bde_a270_602e5e1141e9);
pub const IsScrollItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1cad1a05_0927_4b76_97e1_0fcdb209b98a);
pub const IsScrollPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3ebb7b4a_828a_4b57_9d22_2fea1632ed0d);
pub const IsSelectionItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8becd62d_0bc3_4109_bee2_8e6715290e68);
pub const IsSelectionPattern2Available_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x490806fb_6e89_4a47_8319_d266e511f021);
pub const IsSelectionPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf588acbe_c769_4838_9a60_2686dc1188c4);
pub const IsSpreadsheetItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9fe79b2a_2f94_43fd_996b_549e316f4acd);
pub const IsSpreadsheetPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6ff43732_e4b4_4555_97bc_ecdbbc4d1888);
pub const IsStructuredMarkupPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb0d4c196_2c0b_489c_b165_a405928c6f3d);
pub const IsStylesPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x27f353d3_459c_4b59_a490_50611dacafb5);
pub const IsSynchronizedInputPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x75d69cc5_d2bf_4943_876e_b45b62a6cc66);
pub const IsTableItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeb36b40d_8ea4_489b_a013_e60d5951fe34);
pub const IsTablePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcb83575f_45c2_4048_9c76_159715a139df);
pub const IsTextChildPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x559e65df_30ff_43b5_b5ed_5b283b80c7e9);
pub const IsTextEditPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7843425c_8b32_484c_9ab5_e3200571ffda);
pub const IsTextPattern2Available_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x41cf921d_e3f1_4b22_9c81_e1c3ed331c22);
pub const IsTextPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfbe2d69d_aff6_4a45_82e2_fc92a82f5917);
pub const IsTogglePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x78686d53_fcd0_4b83_9b78_5832ce63bb5b);
pub const IsTransformPattern2Available_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x25980b4b_be04_4710_ab4a_fda31dbd2895);
pub const IsTransformPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa7f78804_d68b_4077_a5c6_7a5ea1ac31c5);
pub const IsValuePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0b5020a7_2119_473b_be37_5ceb98bbfb22);
pub const IsVirtualizedItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x302cb151_2ac8_45d6_977b_d2b3a5a53f20);
pub const IsWindowPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe7a57bb1_5888_4155_98dc_b422fd57f2bc);
pub const ItemContainer_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3d13da0f_8b9a_4a99_85fa_c5c9a69f1ed4);
pub const ItemStatus_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x51de0321_3973_43e7_8913_0b08e813c37f);
pub const ItemType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcdda434d_6222_413b_a68a_325dd1d40f39);
pub const LIBID_Accessibility: windows_core::GUID = windows_core::GUID::from_u128(0x1ea4dbf0_3c3b_11cf_810c_00aa00389b71);
pub const LabeledBy_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe5b8924b_fc8a_4a35_8031_cf78ac43e55e);
pub const LandmarkType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x454045f2_6f61_49f7_a4f8_b5f0cf82da1e);
pub const LayoutInvalidated_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xed7d6544_a6bd_4595_9bae_3d28946cc715);
pub const LegacyIAccessible_ChildId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9a191b5d_9ef2_4787_a459_dcde885dd4e8);
pub const LegacyIAccessible_DefaultAction_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3b331729_eaad_4502_b85f_92615622913c);
pub const LegacyIAccessible_Description_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x46448418_7d70_4ea9_9d27_b7e775cf2ad7);
pub const LegacyIAccessible_Help_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x94402352_161c_4b77_a98d_a872cc33947a);
pub const LegacyIAccessible_KeyboardShortcut_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8f6909ac_00b8_4259_a41c_966266d43a8a);
pub const LegacyIAccessible_Name_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcaeb063d_40ae_4869_aa5a_1b8e5d666739);
pub const LegacyIAccessible_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x54cc0a9f_3395_48af_ba8d_73f85690f3e0);
pub const LegacyIAccessible_Role_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6856e59f_cbaf_4e31_93e8_bcbf6f7e491c);
pub const LegacyIAccessible_Selection_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8aa8b1e0_0891_40cc_8b06_90d7d4166219);
pub const LegacyIAccessible_State_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdf985854_2281_4340_ab9c_c60e2c5803f6);
pub const LegacyIAccessible_Value_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb5c5b0b6_8217_4a77_97a5_190a85ed0156);
pub const Level_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x242ac529_cd36_400f_aad9_7876ef3af627);
pub const ListItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7b3717f2_44d1_4a58_98a8_f12a9b8f78e2);
pub const List_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9b149ee1_7cca_4cfc_9af1_cac7bddd3031);
pub const LiveRegionChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x102d5e90_e6a9_41b6_b1c5_a9b1929d9510);
pub const LiveSetting_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc12bcd8e_2a8e_4950_8ae7_3625111d58eb);
pub const LocalizedControlType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8763404f_a1bd_452a_89c4_3f01d3833806);
pub const LocalizedLandmarkType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7ac81980_eafb_4fb2_bf91_f485bef5e8e1);
pub const MSAA_MENU_SIG: i32 = -1441927155i32;
pub const MenuBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcc384250_0e7b_4ae8_95ae_a08f261b52ee);
pub const MenuClosed_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3cf1266e_1582_4041_acd7_88a35a965297);
pub const MenuItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf45225d3_d0a0_49d8_9834_9a000d2aeddc);
pub const MenuModeEnd_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9ecd4c9f_80dd_47b8_8267_5aec06bb2cff);
pub const MenuModeStart_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x18d7c631_166a_4ac9_ae3b_ef4b5420e681);
pub const MenuOpened_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xebe2e945_66ca_4ed1_9ff8_2ad7df0a1b08);
pub const Menu_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2e9b1440_0ea8_41fd_b374_c1ea6f503cd1);
pub const MultipleView_CurrentView_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7a81a67a_b94f_4875_918b_65c8d2f998e5);
pub const MultipleView_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x547a6ae4_113f_47c4_850f_db4dfa466b1d);
pub const MultipleView_SupportedViews_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8d5db9fd_ce3c_4ae7_b788_400a3c645547);
pub const NAVDIR_DOWN: u32 = 2u32;
pub const NAVDIR_FIRSTCHILD: u32 = 7u32;
pub const NAVDIR_LASTCHILD: u32 = 8u32;
pub const NAVDIR_LEFT: u32 = 3u32;
pub const NAVDIR_MAX: u32 = 9u32;
pub const NAVDIR_MIN: u32 = 0u32;
pub const NAVDIR_NEXT: u32 = 5u32;
pub const NAVDIR_PREVIOUS: u32 = 6u32;
pub const NAVDIR_RIGHT: u32 = 4u32;
pub const NAVDIR_UP: u32 = 1u32;
pub const Name_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc3a6921b_4a99_44f1_bca6_61187052c431);
pub const NavigateDirection_FirstChild: NavigateDirection = NavigateDirection(3i32);
pub const NavigateDirection_LastChild: NavigateDirection = NavigateDirection(4i32);
pub const NavigateDirection_NextSibling: NavigateDirection = NavigateDirection(1i32);
pub const NavigateDirection_Parent: NavigateDirection = NavigateDirection(0i32);
pub const NavigateDirection_PreviousSibling: NavigateDirection = NavigateDirection(2i32);
pub const NewNativeWindowHandle_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5196b33b_380a_4982_95e1_91f3ef60e024);
pub const NormalizeState_Custom: NormalizeState = NormalizeState(2i32);
pub const NormalizeState_None: NormalizeState = NormalizeState(0i32);
pub const NormalizeState_View: NormalizeState = NormalizeState(1i32);
pub const NotificationKind_ActionAborted: NotificationKind = NotificationKind(3i32);
pub const NotificationKind_ActionCompleted: NotificationKind = NotificationKind(2i32);
pub const NotificationKind_ItemAdded: NotificationKind = NotificationKind(0i32);
pub const NotificationKind_ItemRemoved: NotificationKind = NotificationKind(1i32);
pub const NotificationKind_Other: NotificationKind = NotificationKind(4i32);
pub const NotificationProcessing_All: NotificationProcessing = NotificationProcessing(2i32);
pub const NotificationProcessing_CurrentThenMostRecent: NotificationProcessing = NotificationProcessing(4i32);
pub const NotificationProcessing_ImportantAll: NotificationProcessing = NotificationProcessing(0i32);
pub const NotificationProcessing_ImportantMostRecent: NotificationProcessing = NotificationProcessing(1i32);
pub const NotificationProcessing_MostRecent: NotificationProcessing = NotificationProcessing(3i32);
pub const Notification_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x72c5a2f7_9788_480f_b8eb_4dee00f6186f);
pub const ObjectModel_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3e04acfe_08fc_47ec_96bc_353fa3b34aa7);
pub const Off: LiveSetting = LiveSetting(0i32);
pub const OptimizeForVisualContent_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6a852250_c75a_4e5d_b858_e381b0f78861);
pub const OrientationType_Horizontal: OrientationType = OrientationType(1i32);
pub const OrientationType_None: OrientationType = OrientationType(0i32);
pub const OrientationType_Vertical: OrientationType = OrientationType(2i32);
pub const Orientation_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa01eee62_3884_4415_887e_678ec21e39ba);
pub const OutlineColor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc395d6c0_4b55_4762_a073_fd303a634f52);
pub const OutlineStyles_Embossed: OutlineStyles = OutlineStyles(8i32);
pub const OutlineStyles_Engraved: OutlineStyles = OutlineStyles(4i32);
pub const OutlineStyles_None: OutlineStyles = OutlineStyles(0i32);
pub const OutlineStyles_Outline: OutlineStyles = OutlineStyles(1i32);
pub const OutlineStyles_Shadow: OutlineStyles = OutlineStyles(2i32);
pub const OutlineThickness_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x13e67cc7_dac2_4888_bdd3_375c62fa9618);
pub const PROPID_ACC_DEFAULTACTION: windows_core::GUID = windows_core::GUID::from_u128(0x180c072b_c27f_43c7_9922_f63562a4632b);
pub const PROPID_ACC_DESCRIPTION: windows_core::GUID = windows_core::GUID::from_u128(0x4d48dfe4_bd3f_491f_a648_492d6f20c588);
pub const PROPID_ACC_DESCRIPTIONMAP: windows_core::GUID = windows_core::GUID::from_u128(0x1ff1435f_8a14_477b_b226_a0abe279975d);
pub const PROPID_ACC_DODEFAULTACTION: windows_core::GUID = windows_core::GUID::from_u128(0x1ba09523_2e3b_49a6_a059_59682a3c48fd);
pub const PROPID_ACC_FOCUS: windows_core::GUID = windows_core::GUID::from_u128(0x6eb335df_1c29_4127_b12c_dee9fd157f2b);
pub const PROPID_ACC_HELP: windows_core::GUID = windows_core::GUID::from_u128(0xc831e11f_44db_4a99_9768_cb8f978b7231);
pub const PROPID_ACC_HELPTOPIC: windows_core::GUID = windows_core::GUID::from_u128(0x787d1379_8ede_440b_8aec_11f7bf9030b3);
pub const PROPID_ACC_KEYBOARDSHORTCUT: windows_core::GUID = windows_core::GUID::from_u128(0x7d9bceee_7d1e_4979_9382_5180f4172c34);
pub const PROPID_ACC_NAME: windows_core::GUID = windows_core::GUID::from_u128(0x608d3df8_8128_4aa7_a428_f55e49267291);
pub const PROPID_ACC_NAV_DOWN: windows_core::GUID = windows_core::GUID::from_u128(0x031670ed_3cdf_48d2_9613_138f2dd8a668);
pub const PROPID_ACC_NAV_FIRSTCHILD: windows_core::GUID = windows_core::GUID::from_u128(0xcfd02558_557b_4c67_84f9_2a09fce40749);
pub const PROPID_ACC_NAV_LASTCHILD: windows_core::GUID = windows_core::GUID::from_u128(0x302ecaa5_48d5_4f8d_b671_1a8d20a77832);
pub const PROPID_ACC_NAV_LEFT: windows_core::GUID = windows_core::GUID::from_u128(0x228086cb_82f1_4a39_8705_dcdc0fff92f5);
pub const PROPID_ACC_NAV_NEXT: windows_core::GUID = windows_core::GUID::from_u128(0x1cdc5455_8cd9_4c92_a371_3939a2fe3eee);
pub const PROPID_ACC_NAV_PREV: windows_core::GUID = windows_core::GUID::from_u128(0x776d3891_c73b_4480_b3f6_076a16a15af6);
pub const PROPID_ACC_NAV_RIGHT: windows_core::GUID = windows_core::GUID::from_u128(0xcd211d9f_e1cb_4fe5_a77c_920b884d095b);
pub const PROPID_ACC_NAV_UP: windows_core::GUID = windows_core::GUID::from_u128(0x016e1a2b_1a4e_4767_8612_3386f66935ec);
pub const PROPID_ACC_PARENT: windows_core::GUID = windows_core::GUID::from_u128(0x474c22b6_ffc2_467a_b1b5_e958b4657330);
pub const PROPID_ACC_ROLE: windows_core::GUID = windows_core::GUID::from_u128(0xcb905ff2_7bd1_4c05_b3c8_e6c241364d70);
pub const PROPID_ACC_ROLEMAP: windows_core::GUID = windows_core::GUID::from_u128(0xf79acda2_140d_4fe6_8914_208476328269);
pub const PROPID_ACC_SELECTION: windows_core::GUID = windows_core::GUID::from_u128(0xb99d073c_d731_405b_9061_d95e8f842984);
pub const PROPID_ACC_STATE: windows_core::GUID = windows_core::GUID::from_u128(0xa8d4d5b0_0a21_42d0_a5c0_514e984f457b);
pub const PROPID_ACC_STATEMAP: windows_core::GUID = windows_core::GUID::from_u128(0x43946c5e_0ac0_4042_b525_07bbdbe17fa7);
pub const PROPID_ACC_VALUE: windows_core::GUID = windows_core::GUID::from_u128(0x123fe443_211a_4615_9527_c45a7e93717a);
pub const PROPID_ACC_VALUEMAP: windows_core::GUID = windows_core::GUID::from_u128(0xda1c3d79_fc5c_420e_b399_9d1533549e75);
pub const Pane_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5c2b3f5b_9182_42a3_8dec_8c04c1ee634d);
pub const Polite: LiveSetting = LiveSetting(1i32);
pub const PositionInSet_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x33d1dc54_641e_4d76_a6b1_13f341c1f896);
pub const ProcessId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x40499998_9c31_4245_a403_87320e59eaf6);
pub const ProgressBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x228c9f86_c36c_47bb_9fb6_a5834bfc53a4);
pub const PropertyConditionFlags_IgnoreCase: PropertyConditionFlags = PropertyConditionFlags(1i32);
pub const PropertyConditionFlags_MatchSubstring: PropertyConditionFlags = PropertyConditionFlags(2i32);
pub const PropertyConditionFlags_None: PropertyConditionFlags = PropertyConditionFlags(0i32);
pub const ProviderDescription_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdca5708a_c16b_4cd9_b889_beb16a804904);
pub const ProviderOptions_ClientSideProvider: ProviderOptions = ProviderOptions(1i32);
pub const ProviderOptions_HasNativeIAccessible: ProviderOptions = ProviderOptions(128i32);
pub const ProviderOptions_NonClientAreaProvider: ProviderOptions = ProviderOptions(4i32);
pub const ProviderOptions_OverrideProvider: ProviderOptions = ProviderOptions(8i32);
pub const ProviderOptions_ProviderOwnsSetFocus: ProviderOptions = ProviderOptions(16i32);
pub const ProviderOptions_RefuseNonClientSupport: ProviderOptions = ProviderOptions(64i32);
pub const ProviderOptions_ServerSideProvider: ProviderOptions = ProviderOptions(2i32);
pub const ProviderOptions_UseClientCoordinates: ProviderOptions = ProviderOptions(256i32);
pub const ProviderOptions_UseComThreading: ProviderOptions = ProviderOptions(32i32);
pub const ProviderType_BaseHwnd: ProviderType = ProviderType(0i32);
pub const ProviderType_NonClientArea: ProviderType = ProviderType(2i32);
pub const ProviderType_Proxy: ProviderType = ProviderType(1i32);
pub const ROLE_SYSTEM_ALERT: u32 = 8u32;
pub const ROLE_SYSTEM_ANIMATION: u32 = 54u32;
pub const ROLE_SYSTEM_APPLICATION: u32 = 14u32;
pub const ROLE_SYSTEM_BORDER: u32 = 19u32;
pub const ROLE_SYSTEM_BUTTONDROPDOWN: u32 = 56u32;
pub const ROLE_SYSTEM_BUTTONDROPDOWNGRID: u32 = 58u32;
pub const ROLE_SYSTEM_BUTTONMENU: u32 = 57u32;
pub const ROLE_SYSTEM_CARET: u32 = 7u32;
pub const ROLE_SYSTEM_CELL: u32 = 29u32;
pub const ROLE_SYSTEM_CHARACTER: u32 = 32u32;
pub const ROLE_SYSTEM_CHART: u32 = 17u32;
pub const ROLE_SYSTEM_CHECKBUTTON: u32 = 44u32;
pub const ROLE_SYSTEM_CLIENT: u32 = 10u32;
pub const ROLE_SYSTEM_CLOCK: u32 = 61u32;
pub const ROLE_SYSTEM_COLUMN: u32 = 27u32;
pub const ROLE_SYSTEM_COLUMNHEADER: u32 = 25u32;
pub const ROLE_SYSTEM_COMBOBOX: u32 = 46u32;
pub const ROLE_SYSTEM_CURSOR: u32 = 6u32;
pub const ROLE_SYSTEM_DIAGRAM: u32 = 53u32;
pub const ROLE_SYSTEM_DIAL: u32 = 49u32;
pub const ROLE_SYSTEM_DIALOG: u32 = 18u32;
pub const ROLE_SYSTEM_DOCUMENT: u32 = 15u32;
pub const ROLE_SYSTEM_DROPLIST: u32 = 47u32;
pub const ROLE_SYSTEM_EQUATION: u32 = 55u32;
pub const ROLE_SYSTEM_GRAPHIC: u32 = 40u32;
pub const ROLE_SYSTEM_GRIP: u32 = 4u32;
pub const ROLE_SYSTEM_GROUPING: u32 = 20u32;
pub const ROLE_SYSTEM_HELPBALLOON: u32 = 31u32;
pub const ROLE_SYSTEM_HOTKEYFIELD: u32 = 50u32;
pub const ROLE_SYSTEM_INDICATOR: u32 = 39u32;
pub const ROLE_SYSTEM_IPADDRESS: u32 = 63u32;
pub const ROLE_SYSTEM_LINK: u32 = 30u32;
pub const ROLE_SYSTEM_LIST: u32 = 33u32;
pub const ROLE_SYSTEM_LISTITEM: u32 = 34u32;
pub const ROLE_SYSTEM_MENUBAR: u32 = 2u32;
pub const ROLE_SYSTEM_MENUITEM: u32 = 12u32;
pub const ROLE_SYSTEM_MENUPOPUP: u32 = 11u32;
pub const ROLE_SYSTEM_OUTLINE: u32 = 35u32;
pub const ROLE_SYSTEM_OUTLINEBUTTON: u32 = 64u32;
pub const ROLE_SYSTEM_OUTLINEITEM: u32 = 36u32;
pub const ROLE_SYSTEM_PAGETAB: u32 = 37u32;
pub const ROLE_SYSTEM_PAGETABLIST: u32 = 60u32;
pub const ROLE_SYSTEM_PANE: u32 = 16u32;
pub const ROLE_SYSTEM_PROGRESSBAR: u32 = 48u32;
pub const ROLE_SYSTEM_PROPERTYPAGE: u32 = 38u32;
pub const ROLE_SYSTEM_PUSHBUTTON: u32 = 43u32;
pub const ROLE_SYSTEM_RADIOBUTTON: u32 = 45u32;
pub const ROLE_SYSTEM_ROW: u32 = 28u32;
pub const ROLE_SYSTEM_ROWHEADER: u32 = 26u32;
pub const ROLE_SYSTEM_SCROLLBAR: u32 = 3u32;
pub const ROLE_SYSTEM_SEPARATOR: u32 = 21u32;
pub const ROLE_SYSTEM_SLIDER: u32 = 51u32;
pub const ROLE_SYSTEM_SOUND: u32 = 5u32;
pub const ROLE_SYSTEM_SPINBUTTON: u32 = 52u32;
pub const ROLE_SYSTEM_SPLITBUTTON: u32 = 62u32;
pub const ROLE_SYSTEM_STATICTEXT: u32 = 41u32;
pub const ROLE_SYSTEM_STATUSBAR: u32 = 23u32;
pub const ROLE_SYSTEM_TABLE: u32 = 24u32;
pub const ROLE_SYSTEM_TEXT: u32 = 42u32;
pub const ROLE_SYSTEM_TITLEBAR: u32 = 1u32;
pub const ROLE_SYSTEM_TOOLBAR: u32 = 22u32;
pub const ROLE_SYSTEM_TOOLTIP: u32 = 13u32;
pub const ROLE_SYSTEM_WHITESPACE: u32 = 59u32;
pub const ROLE_SYSTEM_WINDOW: u32 = 9u32;
pub const RadioButton_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3bdb49db_fe2c_4483_b3e1_e57f219440c6);
pub const RangeValue_IsReadOnly_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x25fa1055_debf_4373_a79e_1f1a1908d3c4);
pub const RangeValue_LargeChange_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa1f96325_3a3d_4b44_8e1f_4a46d9844019);
pub const RangeValue_Maximum_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x19319914_f979_4b35_a1a6_d37e05433473);
pub const RangeValue_Minimum_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x78cbd3b2_684d_4860_af93_d1f95cb022fd);
pub const RangeValue_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x18b00d87_b1c9_476a_bfbd_5f0bdb926f63);
pub const RangeValue_SmallChange_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x81c2c457_3941_4107_9975_139760f7c072);
pub const RangeValue_Value_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x131f5d98_c50c_489d_abe5_ae220898c5f7);
pub const Rotation_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x767cdc7d_aec0_4110_ad32_30edd403492e);
pub const RowOrColumnMajor_ColumnMajor: RowOrColumnMajor = RowOrColumnMajor(1i32);
pub const RowOrColumnMajor_Indeterminate: RowOrColumnMajor = RowOrColumnMajor(2i32);
pub const RowOrColumnMajor_RowMajor: RowOrColumnMajor = RowOrColumnMajor(0i32);
pub const RuntimeId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa39eebfa_7fba_4c89_b4d4_b99e2de7d160);
pub const SELFLAG_ADDSELECTION: u32 = 8u32;
pub const SELFLAG_EXTENDSELECTION: u32 = 4u32;
pub const SELFLAG_NONE: u32 = 0u32;
pub const SELFLAG_REMOVESELECTION: u32 = 16u32;
pub const SELFLAG_TAKEFOCUS: u32 = 1u32;
pub const SELFLAG_TAKESELECTION: u32 = 2u32;
pub const SELFLAG_VALID: u32 = 31u32;
pub const SERKF_AVAILABLE: SERIALKEYS_FLAGS = SERIALKEYS_FLAGS(2u32);
pub const SERKF_INDICATOR: SERIALKEYS_FLAGS = SERIALKEYS_FLAGS(4u32);
pub const SERKF_SERIALKEYSON: SERIALKEYS_FLAGS = SERIALKEYS_FLAGS(1u32);
pub const SID_ControlElementProvider: windows_core::GUID = windows_core::GUID::from_u128(0xf4791d68_e254_4ba3_9a53_26a5c5497946);
pub const SID_IsUIAutomationObject: windows_core::GUID = windows_core::GUID::from_u128(0xb96fdb85_7204_4724_842b_c7059dedb9d0);
pub const SKF_AUDIBLEFEEDBACK: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(64u32);
pub const SKF_AVAILABLE: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(2u32);
pub const SKF_CONFIRMHOTKEY: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(8u32);
pub const SKF_HOTKEYACTIVE: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(4u32);
pub const SKF_HOTKEYSOUND: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(16u32);
pub const SKF_INDICATOR: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(32u32);
pub const SKF_LALTLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(268435456u32);
pub const SKF_LALTLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(1048576u32);
pub const SKF_LCTLLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(67108864u32);
pub const SKF_LCTLLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(262144u32);
pub const SKF_LSHIFTLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(16777216u32);
pub const SKF_LSHIFTLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(65536u32);
pub const SKF_LWINLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(1073741824u32);
pub const SKF_LWINLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(4194304u32);
pub const SKF_RALTLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(536870912u32);
pub const SKF_RALTLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(2097152u32);
pub const SKF_RCTLLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(134217728u32);
pub const SKF_RCTLLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(524288u32);
pub const SKF_RSHIFTLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(33554432u32);
pub const SKF_RSHIFTLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(131072u32);
pub const SKF_RWINLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(2147483648u32);
pub const SKF_RWINLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(8388608u32);
pub const SKF_STICKYKEYSON: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(1u32);
pub const SKF_TRISTATE: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(128u32);
pub const SKF_TWOKEYSOFF: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(256u32);
pub const SSF_AVAILABLE: SOUNDSENTRY_FLAGS = SOUNDSENTRY_FLAGS(2u32);
pub const SSF_INDICATOR: SOUNDSENTRY_FLAGS = SOUNDSENTRY_FLAGS(4u32);
pub const SSF_SOUNDSENTRYON: SOUNDSENTRY_FLAGS = SOUNDSENTRY_FLAGS(1u32);
pub const SSGF_DISPLAY: SOUND_SENTRY_GRAPHICS_EFFECT = SOUND_SENTRY_GRAPHICS_EFFECT(3u32);
pub const SSGF_NONE: SOUND_SENTRY_GRAPHICS_EFFECT = SOUND_SENTRY_GRAPHICS_EFFECT(0u32);
pub const SSTF_BORDER: SOUNDSENTRY_TEXT_EFFECT = SOUNDSENTRY_TEXT_EFFECT(2u32);
pub const SSTF_CHARS: SOUNDSENTRY_TEXT_EFFECT = SOUNDSENTRY_TEXT_EFFECT(1u32);
pub const SSTF_DISPLAY: SOUNDSENTRY_TEXT_EFFECT = SOUNDSENTRY_TEXT_EFFECT(3u32);
pub const SSTF_NONE: SOUNDSENTRY_TEXT_EFFECT = SOUNDSENTRY_TEXT_EFFECT(0u32);
pub const SSWF_CUSTOM: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(4u32);
pub const SSWF_DISPLAY: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(3u32);
pub const SSWF_NONE: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(0u32);
pub const SSWF_TITLE: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(1u32);
pub const SSWF_WINDOW: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(2u32);
pub const STATE_SYSTEM_HASPOPUP: u32 = 1073741824u32;
pub const STATE_SYSTEM_NORMAL: u32 = 0u32;
pub const SayAsInterpretAs_Address: SayAsInterpretAs = SayAsInterpretAs(11i32);
pub const SayAsInterpretAs_Alphanumeric: SayAsInterpretAs = SayAsInterpretAs(12i32);
pub const SayAsInterpretAs_Cardinal: SayAsInterpretAs = SayAsInterpretAs(2i32);
pub const SayAsInterpretAs_Currency: SayAsInterpretAs = SayAsInterpretAs(8i32);
pub const SayAsInterpretAs_Date: SayAsInterpretAs = SayAsInterpretAs(5i32);
pub const SayAsInterpretAs_Date_DayMonth: SayAsInterpretAs = SayAsInterpretAs(20i32);
pub const SayAsInterpretAs_Date_DayMonthYear: SayAsInterpretAs = SayAsInterpretAs(16i32);
pub const SayAsInterpretAs_Date_MonthDay: SayAsInterpretAs = SayAsInterpretAs(21i32);
pub const SayAsInterpretAs_Date_MonthDayYear: SayAsInterpretAs = SayAsInterpretAs(15i32);
pub const SayAsInterpretAs_Date_MonthYear: SayAsInterpretAs = SayAsInterpretAs(19i32);
pub const SayAsInterpretAs_Date_Year: SayAsInterpretAs = SayAsInterpretAs(22i32);
pub const SayAsInterpretAs_Date_YearMonth: SayAsInterpretAs = SayAsInterpretAs(18i32);
pub const SayAsInterpretAs_Date_YearMonthDay: SayAsInterpretAs = SayAsInterpretAs(17i32);
pub const SayAsInterpretAs_Media: SayAsInterpretAs = SayAsInterpretAs(14i32);
pub const SayAsInterpretAs_Name: SayAsInterpretAs = SayAsInterpretAs(13i32);
pub const SayAsInterpretAs_Net: SayAsInterpretAs = SayAsInterpretAs(9i32);
pub const SayAsInterpretAs_None: SayAsInterpretAs = SayAsInterpretAs(0i32);
pub const SayAsInterpretAs_Number: SayAsInterpretAs = SayAsInterpretAs(4i32);
pub const SayAsInterpretAs_Ordinal: SayAsInterpretAs = SayAsInterpretAs(3i32);
pub const SayAsInterpretAs_Spell: SayAsInterpretAs = SayAsInterpretAs(1i32);
pub const SayAsInterpretAs_Telephone: SayAsInterpretAs = SayAsInterpretAs(7i32);
pub const SayAsInterpretAs_Time: SayAsInterpretAs = SayAsInterpretAs(6i32);
pub const SayAsInterpretAs_Time_HoursMinutes12: SayAsInterpretAs = SayAsInterpretAs(24i32);
pub const SayAsInterpretAs_Time_HoursMinutes24: SayAsInterpretAs = SayAsInterpretAs(26i32);
pub const SayAsInterpretAs_Time_HoursMinutesSeconds12: SayAsInterpretAs = SayAsInterpretAs(23i32);
pub const SayAsInterpretAs_Time_HoursMinutesSeconds24: SayAsInterpretAs = SayAsInterpretAs(25i32);
pub const SayAsInterpretAs_Url: SayAsInterpretAs = SayAsInterpretAs(10i32);
pub const ScrollAmount_LargeDecrement: ScrollAmount = ScrollAmount(0i32);
pub const ScrollAmount_LargeIncrement: ScrollAmount = ScrollAmount(3i32);
pub const ScrollAmount_NoAmount: ScrollAmount = ScrollAmount(2i32);
pub const ScrollAmount_SmallDecrement: ScrollAmount = ScrollAmount(1i32);
pub const ScrollAmount_SmallIncrement: ScrollAmount = ScrollAmount(4i32);
pub const ScrollBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdaf34b36_5065_4946_b22f_92595fc0751a);
pub const ScrollItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4591d005_a803_4d5c_b4d5_8d2800f906a7);
pub const Scroll_HorizontalScrollPercent_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc7c13c0e_eb21_47ff_acc4_b5a3350f5191);
pub const Scroll_HorizontalViewSize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x70c2e5d4_fcb0_4713_a9aa_af92ff79e4cd);
pub const Scroll_HorizontallyScrollable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8b925147_28cd_49ae_bd63_f44118d2e719);
pub const Scroll_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x895fa4b4_759d_4c50_8e15_03460672003c);
pub const Scroll_VerticalScrollPercent_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6c8d7099_b2a8_4948_bff7_3cf9058bfefb);
pub const Scroll_VerticalViewSize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xde6a2e22_d8c7_40c5_83ba_e5f681d53108);
pub const Scroll_VerticallyScrollable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x89164798_0068_4315_b89a_1e7cfbbc3dfc);
pub const Selection2_CurrentSelectedItem_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x34257c26_83b5_41a6_939c_ae841c136236);
pub const Selection2_FirstSelectedItem_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcc24ea67_369c_4e55_9ff7_38da69540c29);
pub const Selection2_ItemCount_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbb49eb9f_456d_4048_b591_9c2026b84636);
pub const Selection2_LastSelectedItem_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcf7bda90_2d83_49f8_860c_9ce394cf89b4);
pub const SelectionItem_ElementAddedToSelectionEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3c822dd1_c407_4dba_91dd_79d4aed0aec6);
pub const SelectionItem_ElementRemovedFromSelectionEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x097fa8a9_7079_41af_8b9c_0934d8305e5c);
pub const SelectionItem_ElementSelectedEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb9c7dbfb_4ebe_4532_aaf4_008cf647233c);
pub const SelectionItem_IsSelected_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf122835f_cd5f_43df_b79d_4b849e9e6020);
pub const SelectionItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9bc64eeb_87c7_4b28_94bb_4d9fa437b6ef);
pub const SelectionItem_SelectionContainer_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa4365b6e_9c1e_4b63_8b53_c2421dd1e8fb);
pub const Selection_CanSelectMultiple_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x49d73da5_c883_4500_883d_8fcf8daf6cbe);
pub const Selection_InvalidatedEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcac14904_16b4_4b53_8e47_4cb1df267bb7);
pub const Selection_IsSelectionRequired_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb1ae4422_63fe_44e7_a5a5_a738c829b19a);
pub const Selection_Pattern2_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfba25cab_ab98_49f7_a7dc_fe539dc15be7);
pub const Selection_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x66e3b7e8_d821_4d25_8761_435d2c8b253f);
pub const Selection_Selection_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaa6dc2a2_0e2b_4d38_96d5_34e470b81853);
pub const SemanticZoom_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5fd34a43_061e_42c8_b589_9dccf74bc43a);
pub const Separator_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8767eba3_2a63_4ab0_ac8d_aa50e23de978);
pub const SizeOfSet_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1600d33c_3b9f_4369_9431_aa293f344cf1);
pub const Size_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2b5f761d_f885_4404_973f_9b1d98e36d8f);
pub const Slider_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb033c24b_3b35_4cea_b609_763682fa660b);
pub const Spinner_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x60cc4b38_3cb1_4161_b442_c6b726c17825);
pub const SplitButton_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7011f01f_4ace_4901_b461_920a6f1ca650);
pub const SpreadsheetItem_AnnotationObjects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa3194c38_c9bc_4604_9396_ae3f9f457f7b);
pub const SpreadsheetItem_AnnotationTypes_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc70c51d0_d602_4b45_afbc_b4712b96d72b);
pub const SpreadsheetItem_Formula_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe602e47d_1b47_4bea_87cf_3b0b0b5c15b6);
pub const SpreadsheetItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x32cf83ff_f1a8_4a8c_8658_d47ba74e20ba);
pub const Spreadsheet_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6a5b24c9_9d1e_4b85_9e44_c02e3169b10b);
pub const StatusBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd45e7d1b_5873_475f_95a4_0433e1f1b00a);
pub const StructureChangeType_ChildAdded: StructureChangeType = StructureChangeType(0i32);
pub const StructureChangeType_ChildRemoved: StructureChangeType = StructureChangeType(1i32);
pub const StructureChangeType_ChildrenBulkAdded: StructureChangeType = StructureChangeType(3i32);
pub const StructureChangeType_ChildrenBulkRemoved: StructureChangeType = StructureChangeType(4i32);
pub const StructureChangeType_ChildrenInvalidated: StructureChangeType = StructureChangeType(2i32);
pub const StructureChangeType_ChildrenReordered: StructureChangeType = StructureChangeType(5i32);
pub const StructureChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x59977961_3edd_4b11_b13b_676b2a2a6ca9);
pub const StructuredMarkup_CompositionComplete_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc48a3c17_677a_4047_a68d_fc1257528aef);
pub const StructuredMarkup_Deleted_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf9d0a020_e1c1_4ecf_b9aa_52efde7e41e1);
pub const StructuredMarkup_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xabbd0878_8665_4f5c_94fc_36e7d8bb706b);
pub const StructuredMarkup_SelectionChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa7c815f7_ff9f_41c7_a3a7_ab6cbfdb4903);
pub const StyleId_BulletedList: UIA_STYLE_ID = UIA_STYLE_ID(70015i32);
pub const StyleId_BulletedList_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5963ed64_6426_4632_8caf_a32ad402d91a);
pub const StyleId_Custom: UIA_STYLE_ID = UIA_STYLE_ID(70000i32);
pub const StyleId_Custom_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xef2edd3e_a999_4b7c_a378_09bbd52a3516);
pub const StyleId_Emphasis: UIA_STYLE_ID = UIA_STYLE_ID(70013i32);
pub const StyleId_Emphasis_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xca6e7dbe_355e_4820_95a0_925f041d3470);
pub const StyleId_Heading1: UIA_STYLE_ID = UIA_STYLE_ID(70001i32);
pub const StyleId_Heading1_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7f7e8f69_6866_4621_930c_9a5d0ca5961c);
pub const StyleId_Heading2: UIA_STYLE_ID = UIA_STYLE_ID(70002i32);
pub const StyleId_Heading2_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbaa9b241_5c69_469d_85ad_474737b52b14);
pub const StyleId_Heading3: UIA_STYLE_ID = UIA_STYLE_ID(70003i32);
pub const StyleId_Heading3_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbf8be9d2_d8b8_4ec5_8c52_9cfb0d035970);
pub const StyleId_Heading4: UIA_STYLE_ID = UIA_STYLE_ID(70004i32);
pub const StyleId_Heading4_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8436ffc0_9578_45fc_83a4_ff40053315dd);
pub const StyleId_Heading5: UIA_STYLE_ID = UIA_STYLE_ID(70005i32);
pub const StyleId_Heading5_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x909f424d_0dbf_406e_97bb_4e773d9798f7);
pub const StyleId_Heading6: UIA_STYLE_ID = UIA_STYLE_ID(70006i32);
pub const StyleId_Heading6_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x89d23459_5d5b_4824_a420_11d3ed82e40f);
pub const StyleId_Heading7: UIA_STYLE_ID = UIA_STYLE_ID(70007i32);
pub const StyleId_Heading7_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa3790473_e9ae_422d_b8e3_3b675c6181a4);
pub const StyleId_Heading8: UIA_STYLE_ID = UIA_STYLE_ID(70008i32);
pub const StyleId_Heading8_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2bc14145_a40c_4881_84ae_f2235685380c);
pub const StyleId_Heading9: UIA_STYLE_ID = UIA_STYLE_ID(70009i32);
pub const StyleId_Heading9_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc70d9133_bb2a_43d3_8ac6_33657884b0f0);
pub const StyleId_Normal: UIA_STYLE_ID = UIA_STYLE_ID(70012i32);
pub const StyleId_Normal_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcd14d429_e45e_4475_a1c5_7f9e6be96eba);
pub const StyleId_NumberedList: UIA_STYLE_ID = UIA_STYLE_ID(70016i32);
pub const StyleId_NumberedList_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1e96dbd5_64c3_43d0_b1ee_b53b06e3eddf);
pub const StyleId_Quote: UIA_STYLE_ID = UIA_STYLE_ID(70014i32);
pub const StyleId_Quote_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5d1c21ea_8195_4f6c_87ea_5dabece64c1d);
pub const StyleId_Subtitle: UIA_STYLE_ID = UIA_STYLE_ID(70011i32);
pub const StyleId_Subtitle_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb5d9fc17_5d6f_4420_b439_7cb19ad434e2);
pub const StyleId_Title: UIA_STYLE_ID = UIA_STYLE_ID(70010i32);
pub const StyleId_Title_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x15d8201a_ffcf_481f_b0a1_30b63be98f07);
pub const Styles_ExtendedProperties_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf451cda0_ba0a_4681_b0b0_0dbdb53e58f3);
pub const Styles_FillColor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x63eff97a_a1c5_4b1d_84eb_b765f2edd632);
pub const Styles_FillPatternColor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x939a59fe_8fbd_4e75_a271_ac4595195163);
pub const Styles_FillPatternStyle_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x81cf651f_482b_4451_a30a_e1545e554fb8);
pub const Styles_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1ae62655_da72_4d60_a153_e5aa6988e3bf);
pub const Styles_Shape_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc71a23f8_778c_400d_8458_3b543e526984);
pub const Styles_StyleId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xda82852f_3817_4233_82af_02279e72cc77);
pub const Styles_StyleName_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1c12b035_05d1_4f55_9e8e_1489f3ff550d);
pub const SupportedTextSelection_Multiple: SupportedTextSelection = SupportedTextSelection(2i32);
pub const SupportedTextSelection_None: SupportedTextSelection = SupportedTextSelection(0i32);
pub const SupportedTextSelection_Single: SupportedTextSelection = SupportedTextSelection(1i32);
pub const SynchronizedInputType_KeyDown: SynchronizedInputType = SynchronizedInputType(2i32);
pub const SynchronizedInputType_KeyUp: SynchronizedInputType = SynchronizedInputType(1i32);
pub const SynchronizedInputType_LeftMouseDown: SynchronizedInputType = SynchronizedInputType(8i32);
pub const SynchronizedInputType_LeftMouseUp: SynchronizedInputType = SynchronizedInputType(4i32);
pub const SynchronizedInputType_RightMouseDown: SynchronizedInputType = SynchronizedInputType(32i32);
pub const SynchronizedInputType_RightMouseUp: SynchronizedInputType = SynchronizedInputType(16i32);
pub const SynchronizedInput_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x05c288a6_c47b_488b_b653_33977a551b8b);
pub const SystemAlert_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd271545d_7a3a_47a7_8474_81d29a2451c9);
pub const TabItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2c6a634f_921b_4e6e_b26e_08fcb0798f4c);
pub const Tab_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x38cd1f2d_337a_4bd2_a5e3_adb469e30bd3);
pub const TableItem_ColumnHeaderItems_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x967a56a3_74b6_431e_8de6_99c411031c58);
pub const TableItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdf1343bd_1888_4a29_a50c_b92e6de37f6f);
pub const TableItem_RowHeaderItems_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb3f853a0_0574_4cd8_bcd7_ed5923572d97);
pub const Table_ColumnHeaders_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaff1d72b_968d_42b1_b459_150b299da664);
pub const Table_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x773bfa0e_5bc4_4deb_921b_de7b3206229e);
pub const Table_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc415218e_a028_461e_aa92_8f925cf79351);
pub const Table_RowHeaders_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd9e35b87_6eb8_4562_aac6_a8a9075236a8);
pub const Table_RowOrColumnMajor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x83be75c3_29fe_4a30_85e1_2a6277fd106e);
pub const TextChild_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7533cab7_3bfe_41ef_9e85_e2638cbe169e);
pub const TextDecorationLineStyle_Dash: TextDecorationLineStyle = TextDecorationLineStyle(5i32);
pub const TextDecorationLineStyle_DashDot: TextDecorationLineStyle = TextDecorationLineStyle(6i32);
pub const TextDecorationLineStyle_DashDotDot: TextDecorationLineStyle = TextDecorationLineStyle(7i32);
pub const TextDecorationLineStyle_Dot: TextDecorationLineStyle = TextDecorationLineStyle(4i32);
pub const TextDecorationLineStyle_Double: TextDecorationLineStyle = TextDecorationLineStyle(3i32);
pub const TextDecorationLineStyle_DoubleWavy: TextDecorationLineStyle = TextDecorationLineStyle(11i32);
pub const TextDecorationLineStyle_LongDash: TextDecorationLineStyle = TextDecorationLineStyle(13i32);
pub const TextDecorationLineStyle_None: TextDecorationLineStyle = TextDecorationLineStyle(0i32);
pub const TextDecorationLineStyle_Other: TextDecorationLineStyle = TextDecorationLineStyle(-1i32);
pub const TextDecorationLineStyle_Single: TextDecorationLineStyle = TextDecorationLineStyle(1i32);
pub const TextDecorationLineStyle_ThickDash: TextDecorationLineStyle = TextDecorationLineStyle(14i32);
pub const TextDecorationLineStyle_ThickDashDot: TextDecorationLineStyle = TextDecorationLineStyle(15i32);
pub const TextDecorationLineStyle_ThickDashDotDot: TextDecorationLineStyle = TextDecorationLineStyle(16i32);
pub const TextDecorationLineStyle_ThickDot: TextDecorationLineStyle = TextDecorationLineStyle(17i32);
pub const TextDecorationLineStyle_ThickLongDash: TextDecorationLineStyle = TextDecorationLineStyle(18i32);
pub const TextDecorationLineStyle_ThickSingle: TextDecorationLineStyle = TextDecorationLineStyle(9i32);
pub const TextDecorationLineStyle_ThickWavy: TextDecorationLineStyle = TextDecorationLineStyle(12i32);
pub const TextDecorationLineStyle_Wavy: TextDecorationLineStyle = TextDecorationLineStyle(8i32);
pub const TextDecorationLineStyle_WordsOnly: TextDecorationLineStyle = TextDecorationLineStyle(2i32);
pub const TextEditChangeType_AutoComplete: TextEditChangeType = TextEditChangeType(4i32);
pub const TextEditChangeType_AutoCorrect: TextEditChangeType = TextEditChangeType(1i32);
pub const TextEditChangeType_Composition: TextEditChangeType = TextEditChangeType(2i32);
pub const TextEditChangeType_CompositionFinalized: TextEditChangeType = TextEditChangeType(3i32);
pub const TextEditChangeType_None: TextEditChangeType = TextEditChangeType(0i32);
pub const TextEdit_ConversionTargetChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3388c183_ed4f_4c8b_9baa_364d51d8847f);
pub const TextEdit_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x69f3ff89_5af9_4c75_9340_f2de292e4591);
pub const TextEdit_TextChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x120b0308_ec22_4eb8_9c98_9867cda1b165);
pub const TextPatternRangeEndpoint_End: TextPatternRangeEndpoint = TextPatternRangeEndpoint(1i32);
pub const TextPatternRangeEndpoint_Start: TextPatternRangeEndpoint = TextPatternRangeEndpoint(0i32);
pub const TextUnit_Character: TextUnit = TextUnit(0i32);
pub const TextUnit_Document: TextUnit = TextUnit(6i32);
pub const TextUnit_Format: TextUnit = TextUnit(1i32);
pub const TextUnit_Line: TextUnit = TextUnit(3i32);
pub const TextUnit_Page: TextUnit = TextUnit(5i32);
pub const TextUnit_Paragraph: TextUnit = TextUnit(4i32);
pub const TextUnit_Word: TextUnit = TextUnit(2i32);
pub const Text_AfterParagraphSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x588cbb38_e62f_497c_b5d1_ccdf0ee823d8);
pub const Text_AfterSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x588cbb38_e62f_497c_b5d1_ccdf0ee823d8);
pub const Text_AnimationStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x628209f0_7c9a_4d57_be64_1f1836571ff5);
pub const Text_AnnotationObjects_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xff41cf68_e7ab_40b9_8c72_72a8ed94017d);
pub const Text_AnnotationTypes_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xad2eb431_ee4e_4be1_a7ba_5559155a73ef);
pub const Text_BackgroundColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfdc49a07_583d_4f17_ad27_77fc832a3c0b);
pub const Text_BeforeParagraphSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbe7b0ab1_c822_4a24_85e9_c8f2650fc79c);
pub const Text_BeforeSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbe7b0ab1_c822_4a24_85e9_c8f2650fc79c);
pub const Text_BulletStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc1097c90_d5c4_4237_9781_3bec8ba54e48);
pub const Text_CapStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfb059c50_92cc_49a5_ba8f_0aa872bba2f3);
pub const Text_CaretBidiMode_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x929ee7a6_51d3_4715_96dc_b694fa24a168);
pub const Text_CaretPosition_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb227b131_9889_4752_a91b_733efdc5c5a0);
pub const Text_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xae9772dc_d331_4f09_be20_7e6dfaf07b0a);
pub const Text_Culture_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc2025af9_a42d_4ced_a1fb_c6746315222e);
pub const Text_FontName_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x64e63ba8_f2e5_476e_a477_1734feaaf726);
pub const Text_FontSize_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdc5eeeff_0506_4673_93f2_377e4a8e01f1);
pub const Text_FontWeight_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6fc02359_b316_4f5f_b401_f1ce55741853);
pub const Text_ForegroundColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x72d1c95d_5e60_471a_96b1_6c1b3b77a436);
pub const Text_HorizontalTextAlignment_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x04ea6161_fba3_477a_952a_bb326d026a5b);
pub const Text_IndentationFirstLine_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x206f9ad5_c1d3_424a_8182_6da9a7f3d632);
pub const Text_IndentationLeading_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5cf66bac_2d45_4a4b_b6c9_f7221d2815b0);
pub const Text_IndentationTrailing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x97ff6c0f_1ce4_408a_b67b_94d83eb69bf2);
pub const Text_IsActive_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf5a4e533_e1b8_436b_935d_b57aa3f558c4);
pub const Text_IsHidden_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x360182fb_bdd7_47f6_ab69_19e33f8a3344);
pub const Text_IsItalic_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfce12a56_1336_4a34_9663_1bab47239320);
pub const Text_IsReadOnly_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa738156b_ca3e_495e_9514_833c440feb11);
pub const Text_IsSubscript_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf0ead858_8f53_413c_873f_1a7d7f5e0de4);
pub const Text_IsSuperscript_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xda706ee4_b3aa_4645_a41f_cd25157dea76);
pub const Text_LineSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x63ff70ae_d943_4b47_8ab7_a7a033d3214b);
pub const Text_Link_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb38ef51d_9e8d_4e46_9144_56ebe177329b);
pub const Text_MarginBottom_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7ee593c4_72b4_4cac_9271_3ed24b0e4d42);
pub const Text_MarginLeading_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9e9242d0_5ed0_4900_8e8a_eecc03835afc);
pub const Text_MarginTop_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x683d936f_c9b9_4a9a_b3d9_d20d33311e2a);
pub const Text_MarginTrailing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaf522f98_999d_40af_a5b2_0169d0342002);
pub const Text_OutlineStyles_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5b675b27_db89_46fe_970c_614d523bb97d);
pub const Text_OverlineColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x83ab383a_fd43_40da_ab3e_ecf8165cbb6d);
pub const Text_OverlineStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0a234d66_617e_427f_871d_e1ff1e0c213f);
pub const Text_Pattern2_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x498479a2_5b22_448d_b6e4_647490860698);
pub const Text_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8615f05d_7de5_44fd_a679_2ca4b46033a8);
pub const Text_SayAsInterpretAs_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb38ad6ac_eee1_4b6e_88cc_014cefa93fcb);
pub const Text_SelectionActiveEnd_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1f668cc3_9bbf_416b_b0a2_f89f86f6612c);
pub const Text_StrikethroughColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbfe15a18_8c41_4c5a_9a0b_04af0e07f487);
pub const Text_StrikethroughStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x72913ef1_da00_4f01_899c_ac5a8577a307);
pub const Text_StyleId_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x14c300de_c32b_449b_ab7c_b0e0789aea5d);
pub const Text_StyleName_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x22c9e091_4d66_45d8_a828_737bab4c98a7);
pub const Text_Tabs_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2e68d00b_92fe_42d8_899a_a784aa4454a1);
pub const Text_TextChangedEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4a342082_f483_48c4_ac11_a84b435e2a84);
pub const Text_TextFlowDirections_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8bdf8739_f420_423e_af77_20a5d973a907);
pub const Text_TextSelectionChangedEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x918edaa1_71b3_49ae_9741_79beb8d358f3);
pub const Text_UnderlineColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbfa12c73_fde2_4473_bf64_1036d6aa0f45);
pub const Text_UnderlineStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5f3b21c0_ede4_44bd_9c36_3853038cbfeb);
pub const Thumb_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x701ca877_e310_4dd6_b644_797e4faea213);
pub const TitleBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x98aa55bf_3bb0_4b65_836e_2ea30dbc171f);
pub const ToggleState_Indeterminate: ToggleState = ToggleState(2i32);
pub const ToggleState_Off: ToggleState = ToggleState(0i32);
pub const ToggleState_On: ToggleState = ToggleState(1i32);
pub const Toggle_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0b419760_e2f4_43ff_8c5f_9457c82b56e9);
pub const Toggle_ToggleState_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb23cdc52_22c2_4c6c_9ded_f5c422479ede);
pub const ToolBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8f06b751_e182_4e98_8893_2284543a7dce);
pub const ToolTipClosed_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x276d71ef_24a9_49b6_8e97_da98b401bbcd);
pub const ToolTipOpened_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3f4b97ff_2edc_451d_bca4_95a3188d5b03);
pub const ToolTip_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x05ddc6d1_2137_4768_98ea_73f52f7134f3);
pub const Tranform_Pattern2_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8afcfd07_a369_44de_988b_2f7ff49fb8a8);
pub const Transform2_CanZoom_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf357e890_a756_4359_9ca6_86702bf8f381);
pub const Transform2_ZoomLevel_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeee29f1a_f4a2_4b5b_ac65_95cf93283387);
pub const Transform2_ZoomMaximum_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x42ab6b77_ceb0_4eca_b82a_6cfa5fa1fc08);
pub const Transform2_ZoomMinimum_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x742ccc16_4ad1_4e07_96fe_b122c6e6b22b);
pub const Transform_CanMove_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1b75824d_208b_4fdf_bccd_f1f4e5741f4f);
pub const Transform_CanResize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbb98dca5_4c1a_41d4_a4f6_ebc128644180);
pub const Transform_CanRotate_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x10079b48_3849_476f_ac96_44a95c8440d9);
pub const Transform_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x24b46fdb_587e_49f1_9c4a_d8e98b664b7b);
pub const TreeItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x62c9feb9_8ffc_4878_a3a4_96b030315c18);
pub const TreeScope_Ancestors: TreeScope = TreeScope(16i32);
pub const TreeScope_Children: TreeScope = TreeScope(2i32);
pub const TreeScope_Descendants: TreeScope = TreeScope(4i32);
pub const TreeScope_Element: TreeScope = TreeScope(1i32);
pub const TreeScope_None: TreeScope = TreeScope(0i32);
pub const TreeScope_Parent: TreeScope = TreeScope(8i32);
pub const TreeScope_Subtree: TreeScope = TreeScope(7i32);
pub const TreeTraversalOptions_Default: TreeTraversalOptions = TreeTraversalOptions(0i32);
pub const TreeTraversalOptions_LastToFirstOrder: TreeTraversalOptions = TreeTraversalOptions(2i32);
pub const TreeTraversalOptions_PostOrder: TreeTraversalOptions = TreeTraversalOptions(1i32);
pub const Tree_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7561349c_d241_43f4_9908_b5f091bee611);
pub const UIA_AcceleratorKeyPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30006i32);
pub const UIA_AccessKeyPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30007i32);
pub const UIA_ActiveTextPositionChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20036i32);
pub const UIA_AfterParagraphSpacingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40042i32);
pub const UIA_AnimationStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40000i32);
pub const UIA_AnnotationAnnotationTypeIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30113i32);
pub const UIA_AnnotationAnnotationTypeNamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30114i32);
pub const UIA_AnnotationAuthorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30115i32);
pub const UIA_AnnotationDateTimePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30116i32);
pub const UIA_AnnotationObjectsAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40032i32);
pub const UIA_AnnotationObjectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30156i32);
pub const UIA_AnnotationPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10023i32);
pub const UIA_AnnotationTargetPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30117i32);
pub const UIA_AnnotationTypesAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40031i32);
pub const UIA_AnnotationTypesPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30155i32);
pub const UIA_AppBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50040i32);
pub const UIA_AriaPropertiesPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30102i32);
pub const UIA_AriaRolePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30101i32);
pub const UIA_AsyncContentLoadedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20006i32);
pub const UIA_AutomationFocusChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20005i32);
pub const UIA_AutomationIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30011i32);
pub const UIA_AutomationPropertyChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20004i32);
pub const UIA_BackgroundColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40001i32);
pub const UIA_BeforeParagraphSpacingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40041i32);
pub const UIA_BoundingRectanglePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30001i32);
pub const UIA_BulletStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40002i32);
pub const UIA_ButtonControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50000i32);
pub const UIA_CalendarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50001i32);
pub const UIA_CapStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40003i32);
pub const UIA_CaretBidiModeAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40039i32);
pub const UIA_CaretPositionAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40038i32);
pub const UIA_CenterPointPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30165i32);
pub const UIA_ChangesEventId: UIA_EVENT_ID = UIA_EVENT_ID(20034i32);
pub const UIA_CheckBoxControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50002i32);
pub const UIA_ClassNamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30012i32);
pub const UIA_ClickablePointPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30014i32);
pub const UIA_ComboBoxControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50003i32);
pub const UIA_ControlTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30003i32);
pub const UIA_ControllerForPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30104i32);
pub const UIA_CultureAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40004i32);
pub const UIA_CulturePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30015i32);
pub const UIA_CustomControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50025i32);
pub const UIA_CustomLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80000i32);
pub const UIA_CustomNavigationPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10033i32);
pub const UIA_DataGridControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50028i32);
pub const UIA_DataItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50029i32);
pub const UIA_DescribedByPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30105i32);
pub const UIA_DockDockPositionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30069i32);
pub const UIA_DockPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10011i32);
pub const UIA_DocumentControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50030i32);
pub const UIA_DragDropEffectPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30139i32);
pub const UIA_DragDropEffectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30140i32);
pub const UIA_DragGrabbedItemsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30144i32);
pub const UIA_DragIsGrabbedPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30138i32);
pub const UIA_DragPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10030i32);
pub const UIA_Drag_DragCancelEventId: UIA_EVENT_ID = UIA_EVENT_ID(20027i32);
pub const UIA_Drag_DragCompleteEventId: UIA_EVENT_ID = UIA_EVENT_ID(20028i32);
pub const UIA_Drag_DragStartEventId: UIA_EVENT_ID = UIA_EVENT_ID(20026i32);
pub const UIA_DropTargetDropTargetEffectPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30142i32);
pub const UIA_DropTargetDropTargetEffectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30143i32);
pub const UIA_DropTargetPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10031i32);
pub const UIA_DropTarget_DragEnterEventId: UIA_EVENT_ID = UIA_EVENT_ID(20029i32);
pub const UIA_DropTarget_DragLeaveEventId: UIA_EVENT_ID = UIA_EVENT_ID(20030i32);
pub const UIA_DropTarget_DroppedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20031i32);
pub const UIA_E_ELEMENTNOTAVAILABLE: u32 = 2147746305u32;
pub const UIA_E_ELEMENTNOTENABLED: u32 = 2147746304u32;
pub const UIA_E_INVALIDOPERATION: u32 = 2148734217u32;
pub const UIA_E_NOCLICKABLEPOINT: u32 = 2147746306u32;
pub const UIA_E_NOTSUPPORTED: u32 = 2147746308u32;
pub const UIA_E_PROXYASSEMBLYNOTLOADED: u32 = 2147746307u32;
pub const UIA_E_TIMEOUT: u32 = 2148734213u32;
pub const UIA_EditControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50004i32);
pub const UIA_ExpandCollapseExpandCollapseStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30070i32);
pub const UIA_ExpandCollapsePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10005i32);
pub const UIA_FillColorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30160i32);
pub const UIA_FillTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30162i32);
pub const UIA_FlowsFromPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30148i32);
pub const UIA_FlowsToPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30106i32);
pub const UIA_FontNameAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40005i32);
pub const UIA_FontSizeAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40006i32);
pub const UIA_FontWeightAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40007i32);
pub const UIA_ForegroundColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40008i32);
pub const UIA_FormLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80001i32);
pub const UIA_FrameworkIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30024i32);
pub const UIA_FullDescriptionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30159i32);
pub const UIA_GridColumnCountPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30063i32);
pub const UIA_GridItemColumnPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30065i32);
pub const UIA_GridItemColumnSpanPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30067i32);
pub const UIA_GridItemContainingGridPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30068i32);
pub const UIA_GridItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10007i32);
pub const UIA_GridItemRowPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30064i32);
pub const UIA_GridItemRowSpanPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30066i32);
pub const UIA_GridPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10006i32);
pub const UIA_GridRowCountPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30062i32);
pub const UIA_GroupControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50026i32);
pub const UIA_HasKeyboardFocusPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30008i32);
pub const UIA_HeaderControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50034i32);
pub const UIA_HeaderItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50035i32);
pub const UIA_HeadingLevelPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30173i32);
pub const UIA_HelpTextPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30013i32);
pub const UIA_HorizontalTextAlignmentAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40009i32);
pub const UIA_HostedFragmentRootsInvalidatedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20025i32);
pub const UIA_HyperlinkControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50005i32);
pub const UIA_IAFP_DEFAULT: u32 = 0u32;
pub const UIA_IAFP_UNWRAP_BRIDGE: u32 = 1u32;
pub const UIA_ImageControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50006i32);
pub const UIA_IndentationFirstLineAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40010i32);
pub const UIA_IndentationLeadingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40011i32);
pub const UIA_IndentationTrailingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40012i32);
pub const UIA_InputDiscardedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20022i32);
pub const UIA_InputReachedOtherElementEventId: UIA_EVENT_ID = UIA_EVENT_ID(20021i32);
pub const UIA_InputReachedTargetEventId: UIA_EVENT_ID = UIA_EVENT_ID(20020i32);
pub const UIA_InvokePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10000i32);
pub const UIA_Invoke_InvokedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20009i32);
pub const UIA_IsActiveAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40036i32);
pub const UIA_IsAnnotationPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30118i32);
pub const UIA_IsContentElementPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30017i32);
pub const UIA_IsControlElementPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30016i32);
pub const UIA_IsCustomNavigationPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30151i32);
pub const UIA_IsDataValidForFormPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30103i32);
pub const UIA_IsDialogPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30174i32);
pub const UIA_IsDockPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30027i32);
pub const UIA_IsDragPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30137i32);
pub const UIA_IsDropTargetPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30141i32);
pub const UIA_IsEnabledPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30010i32);
pub const UIA_IsExpandCollapsePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30028i32);
pub const UIA_IsGridItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30029i32);
pub const UIA_IsGridPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30030i32);
pub const UIA_IsHiddenAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40013i32);
pub const UIA_IsInvokePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30031i32);
pub const UIA_IsItalicAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40014i32);
pub const UIA_IsItemContainerPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30108i32);
pub const UIA_IsKeyboardFocusablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30009i32);
pub const UIA_IsLegacyIAccessiblePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30090i32);
pub const UIA_IsMultipleViewPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30032i32);
pub const UIA_IsObjectModelPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30112i32);
pub const UIA_IsOffscreenPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30022i32);
pub const UIA_IsPasswordPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30019i32);
pub const UIA_IsPeripheralPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30150i32);
pub const UIA_IsRangeValuePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30033i32);
pub const UIA_IsReadOnlyAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40015i32);
pub const UIA_IsRequiredForFormPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30025i32);
pub const UIA_IsScrollItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30035i32);
pub const UIA_IsScrollPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30034i32);
pub const UIA_IsSelectionItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30036i32);
pub const UIA_IsSelectionPattern2AvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30168i32);
pub const UIA_IsSelectionPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30037i32);
pub const UIA_IsSpreadsheetItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30132i32);
pub const UIA_IsSpreadsheetPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30128i32);
pub const UIA_IsStylesPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30127i32);
pub const UIA_IsSubscriptAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40016i32);
pub const UIA_IsSuperscriptAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40017i32);
pub const UIA_IsSynchronizedInputPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30110i32);
pub const UIA_IsTableItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30039i32);
pub const UIA_IsTablePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30038i32);
pub const UIA_IsTextChildPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30136i32);
pub const UIA_IsTextEditPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30149i32);
pub const UIA_IsTextPattern2AvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30119i32);
pub const UIA_IsTextPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30040i32);
pub const UIA_IsTogglePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30041i32);
pub const UIA_IsTransformPattern2AvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30134i32);
pub const UIA_IsTransformPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30042i32);
pub const UIA_IsValuePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30043i32);
pub const UIA_IsVirtualizedItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30109i32);
pub const UIA_IsWindowPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30044i32);
pub const UIA_ItemContainerPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10019i32);
pub const UIA_ItemStatusPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30026i32);
pub const UIA_ItemTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30021i32);
pub const UIA_LabeledByPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30018i32);
pub const UIA_LandmarkTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30157i32);
pub const UIA_LayoutInvalidatedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20008i32);
pub const UIA_LegacyIAccessibleChildIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30091i32);
pub const UIA_LegacyIAccessibleDefaultActionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30100i32);
pub const UIA_LegacyIAccessibleDescriptionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30094i32);
pub const UIA_LegacyIAccessibleHelpPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30097i32);
pub const UIA_LegacyIAccessibleKeyboardShortcutPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30098i32);
pub const UIA_LegacyIAccessibleNamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30092i32);
pub const UIA_LegacyIAccessiblePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10018i32);
pub const UIA_LegacyIAccessibleRolePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30095i32);
pub const UIA_LegacyIAccessibleSelectionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30099i32);
pub const UIA_LegacyIAccessibleStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30096i32);
pub const UIA_LegacyIAccessibleValuePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30093i32);
pub const UIA_LevelPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30154i32);
pub const UIA_LineSpacingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40040i32);
pub const UIA_LinkAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40035i32);
pub const UIA_ListControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50008i32);
pub const UIA_ListItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50007i32);
pub const UIA_LiveRegionChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20024i32);
pub const UIA_LiveSettingPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30135i32);
pub const UIA_LocalizedControlTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30004i32);
pub const UIA_LocalizedLandmarkTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30158i32);
pub const UIA_MainLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80002i32);
pub const UIA_MarginBottomAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40018i32);
pub const UIA_MarginLeadingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40019i32);
pub const UIA_MarginTopAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40020i32);
pub const UIA_MarginTrailingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40021i32);
pub const UIA_MenuBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50010i32);
pub const UIA_MenuClosedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20007i32);
pub const UIA_MenuControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50009i32);
pub const UIA_MenuItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50011i32);
pub const UIA_MenuModeEndEventId: UIA_EVENT_ID = UIA_EVENT_ID(20019i32);
pub const UIA_MenuModeStartEventId: UIA_EVENT_ID = UIA_EVENT_ID(20018i32);
pub const UIA_MenuOpenedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20003i32);
pub const UIA_MultipleViewCurrentViewPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30071i32);
pub const UIA_MultipleViewPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10008i32);
pub const UIA_MultipleViewSupportedViewsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30072i32);
pub const UIA_NamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30005i32);
pub const UIA_NativeWindowHandlePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30020i32);
pub const UIA_NavigationLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80003i32);
pub const UIA_NotificationEventId: UIA_EVENT_ID = UIA_EVENT_ID(20035i32);
pub const UIA_ObjectModelPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10022i32);
pub const UIA_OptimizeForVisualContentPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30111i32);
pub const UIA_OrientationPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30023i32);
pub const UIA_OutlineColorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30161i32);
pub const UIA_OutlineStylesAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40022i32);
pub const UIA_OutlineThicknessPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30164i32);
pub const UIA_OverlineColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40023i32);
pub const UIA_OverlineStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40024i32);
pub const UIA_PFIA_DEFAULT: u32 = 0u32;
pub const UIA_PFIA_UNWRAP_BRIDGE: u32 = 1u32;
pub const UIA_PaneControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50033i32);
pub const UIA_PositionInSetPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30152i32);
pub const UIA_ProcessIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30002i32);
pub const UIA_ProgressBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50012i32);
pub const UIA_ProviderDescriptionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30107i32);
pub const UIA_RadioButtonControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50013i32);
pub const UIA_RangeValueIsReadOnlyPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30048i32);
pub const UIA_RangeValueLargeChangePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30051i32);
pub const UIA_RangeValueMaximumPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30050i32);
pub const UIA_RangeValueMinimumPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30049i32);
pub const UIA_RangeValuePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10003i32);
pub const UIA_RangeValueSmallChangePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30052i32);
pub const UIA_RangeValueValuePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30047i32);
pub const UIA_RotationPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30166i32);
pub const UIA_RuntimeIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30000i32);
pub const UIA_SayAsInterpretAsAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40043i32);
pub const UIA_SayAsInterpretAsMetadataId: UIA_METADATA_ID = UIA_METADATA_ID(100000i32);
pub const UIA_ScrollBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50014i32);
pub const UIA_ScrollHorizontalScrollPercentPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30053i32);
pub const UIA_ScrollHorizontalViewSizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30054i32);
pub const UIA_ScrollHorizontallyScrollablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30057i32);
pub const UIA_ScrollItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10017i32);
pub const UIA_ScrollPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10004i32);
pub const UIA_ScrollPatternNoScroll: f64 = -1f64;
pub const UIA_ScrollVerticalScrollPercentPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30055i32);
pub const UIA_ScrollVerticalViewSizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30056i32);
pub const UIA_ScrollVerticallyScrollablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30058i32);
pub const UIA_SearchLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80004i32);
pub const UIA_Selection2CurrentSelectedItemPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30171i32);
pub const UIA_Selection2FirstSelectedItemPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30169i32);
pub const UIA_Selection2ItemCountPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30172i32);
pub const UIA_Selection2LastSelectedItemPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30170i32);
pub const UIA_SelectionActiveEndAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40037i32);
pub const UIA_SelectionCanSelectMultiplePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30060i32);
pub const UIA_SelectionIsSelectionRequiredPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30061i32);
pub const UIA_SelectionItemIsSelectedPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30079i32);
pub const UIA_SelectionItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10010i32);
pub const UIA_SelectionItemSelectionContainerPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30080i32);
pub const UIA_SelectionItem_ElementAddedToSelectionEventId: UIA_EVENT_ID = UIA_EVENT_ID(20010i32);
pub const UIA_SelectionItem_ElementRemovedFromSelectionEventId: UIA_EVENT_ID = UIA_EVENT_ID(20011i32);
pub const UIA_SelectionItem_ElementSelectedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20012i32);
pub const UIA_SelectionPattern2Id: UIA_PATTERN_ID = UIA_PATTERN_ID(10034i32);
pub const UIA_SelectionPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10001i32);
pub const UIA_SelectionSelectionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30059i32);
pub const UIA_Selection_InvalidatedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20013i32);
pub const UIA_SemanticZoomControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50039i32);
pub const UIA_SeparatorControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50038i32);
pub const UIA_SizeOfSetPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30153i32);
pub const UIA_SizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30167i32);
pub const UIA_SliderControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50015i32);
pub const UIA_SpinnerControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50016i32);
pub const UIA_SplitButtonControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50031i32);
pub const UIA_SpreadsheetItemAnnotationObjectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30130i32);
pub const UIA_SpreadsheetItemAnnotationTypesPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30131i32);
pub const UIA_SpreadsheetItemFormulaPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30129i32);
pub const UIA_SpreadsheetItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10027i32);
pub const UIA_SpreadsheetPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10026i32);
pub const UIA_StatusBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50017i32);
pub const UIA_StrikethroughColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40025i32);
pub const UIA_StrikethroughStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40026i32);
pub const UIA_StructureChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20002i32);
pub const UIA_StyleIdAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40034i32);
pub const UIA_StyleNameAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40033i32);
pub const UIA_StylesExtendedPropertiesPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30126i32);
pub const UIA_StylesFillColorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30122i32);
pub const UIA_StylesFillPatternColorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30125i32);
pub const UIA_StylesFillPatternStylePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30123i32);
pub const UIA_StylesPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10025i32);
pub const UIA_StylesShapePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30124i32);
pub const UIA_StylesStyleIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30120i32);
pub const UIA_StylesStyleNamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30121i32);
pub const UIA_SummaryChangeId: UIA_CHANGE_ID = UIA_CHANGE_ID(90000i32);
pub const UIA_SynchronizedInputPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10021i32);
pub const UIA_SystemAlertEventId: UIA_EVENT_ID = UIA_EVENT_ID(20023i32);
pub const UIA_TabControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50018i32);
pub const UIA_TabItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50019i32);
pub const UIA_TableColumnHeadersPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30082i32);
pub const UIA_TableControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50036i32);
pub const UIA_TableItemColumnHeaderItemsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30085i32);
pub const UIA_TableItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10013i32);
pub const UIA_TableItemRowHeaderItemsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30084i32);
pub const UIA_TablePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10012i32);
pub const UIA_TableRowHeadersPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30081i32);
pub const UIA_TableRowOrColumnMajorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30083i32);
pub const UIA_TabsAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40027i32);
pub const UIA_TextChildPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10029i32);
pub const UIA_TextControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50020i32);
pub const UIA_TextEditPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10032i32);
pub const UIA_TextEdit_ConversionTargetChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20033i32);
pub const UIA_TextEdit_TextChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20032i32);
pub const UIA_TextFlowDirectionsAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40028i32);
pub const UIA_TextPattern2Id: UIA_PATTERN_ID = UIA_PATTERN_ID(10024i32);
pub const UIA_TextPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10014i32);
pub const UIA_Text_TextChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20015i32);
pub const UIA_Text_TextSelectionChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20014i32);
pub const UIA_ThumbControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50027i32);
pub const UIA_TitleBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50037i32);
pub const UIA_TogglePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10015i32);
pub const UIA_ToggleToggleStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30086i32);
pub const UIA_ToolBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50021i32);
pub const UIA_ToolTipClosedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20001i32);
pub const UIA_ToolTipControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50022i32);
pub const UIA_ToolTipOpenedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20000i32);
pub const UIA_Transform2CanZoomPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30133i32);
pub const UIA_Transform2ZoomLevelPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30145i32);
pub const UIA_Transform2ZoomMaximumPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30147i32);
pub const UIA_Transform2ZoomMinimumPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30146i32);
pub const UIA_TransformCanMovePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30087i32);
pub const UIA_TransformCanResizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30088i32);
pub const UIA_TransformCanRotatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30089i32);
pub const UIA_TransformPattern2Id: UIA_PATTERN_ID = UIA_PATTERN_ID(10028i32);
pub const UIA_TransformPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10016i32);
pub const UIA_TreeControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50023i32);
pub const UIA_TreeItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50024i32);
pub const UIA_UnderlineColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40029i32);
pub const UIA_UnderlineStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40030i32);
pub const UIA_ValueIsReadOnlyPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30046i32);
pub const UIA_ValuePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10002i32);
pub const UIA_ValueValuePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30045i32);
pub const UIA_VirtualizedItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10020i32);
pub const UIA_VisualEffectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30163i32);
pub const UIA_WindowCanMaximizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30073i32);
pub const UIA_WindowCanMinimizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30074i32);
pub const UIA_WindowControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50032i32);
pub const UIA_WindowIsModalPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30077i32);
pub const UIA_WindowIsTopmostPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30078i32);
pub const UIA_WindowPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10009i32);
pub const UIA_WindowWindowInteractionStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30076i32);
pub const UIA_WindowWindowVisualStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30075i32);
pub const UIA_Window_WindowClosedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20017i32);
pub const UIA_Window_WindowOpenedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20016i32);
pub const UIAutomationType_Array: UIAutomationType = UIAutomationType(65536i32);
pub const UIAutomationType_Bool: UIAutomationType = UIAutomationType(2i32);
pub const UIAutomationType_BoolArray: UIAutomationType = UIAutomationType(65538i32);
pub const UIAutomationType_Double: UIAutomationType = UIAutomationType(4i32);
pub const UIAutomationType_DoubleArray: UIAutomationType = UIAutomationType(65540i32);
pub const UIAutomationType_Element: UIAutomationType = UIAutomationType(7i32);
pub const UIAutomationType_ElementArray: UIAutomationType = UIAutomationType(65543i32);
pub const UIAutomationType_Int: UIAutomationType = UIAutomationType(1i32);
pub const UIAutomationType_IntArray: UIAutomationType = UIAutomationType(65537i32);
pub const UIAutomationType_Out: UIAutomationType = UIAutomationType(131072i32);
pub const UIAutomationType_OutBool: UIAutomationType = UIAutomationType(131074i32);
pub const UIAutomationType_OutBoolArray: UIAutomationType = UIAutomationType(196610i32);
pub const UIAutomationType_OutDouble: UIAutomationType = UIAutomationType(131076i32);
pub const UIAutomationType_OutDoubleArray: UIAutomationType = UIAutomationType(196612i32);
pub const UIAutomationType_OutElement: UIAutomationType = UIAutomationType(131079i32);
pub const UIAutomationType_OutElementArray: UIAutomationType = UIAutomationType(196615i32);
pub const UIAutomationType_OutInt: UIAutomationType = UIAutomationType(131073i32);
pub const UIAutomationType_OutIntArray: UIAutomationType = UIAutomationType(196609i32);
pub const UIAutomationType_OutPoint: UIAutomationType = UIAutomationType(131077i32);
pub const UIAutomationType_OutPointArray: UIAutomationType = UIAutomationType(196613i32);
pub const UIAutomationType_OutRect: UIAutomationType = UIAutomationType(131078i32);
pub const UIAutomationType_OutRectArray: UIAutomationType = UIAutomationType(196614i32);
pub const UIAutomationType_OutString: UIAutomationType = UIAutomationType(131075i32);
pub const UIAutomationType_OutStringArray: UIAutomationType = UIAutomationType(196611i32);
pub const UIAutomationType_Point: UIAutomationType = UIAutomationType(5i32);
pub const UIAutomationType_PointArray: UIAutomationType = UIAutomationType(65541i32);
pub const UIAutomationType_Rect: UIAutomationType = UIAutomationType(6i32);
pub const UIAutomationType_RectArray: UIAutomationType = UIAutomationType(65542i32);
pub const UIAutomationType_String: UIAutomationType = UIAutomationType(3i32);
pub const UIAutomationType_StringArray: UIAutomationType = UIAutomationType(65539i32);
pub const UiaAppendRuntimeId: u32 = 3u32;
pub const UiaRootObjectId: i32 = -25i32;
pub const Value_IsReadOnly_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeb090f30_e24c_4799_a705_0d247bc037f8);
pub const Value_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x17faad9e_c877_475b_b933_77332779b637);
pub const Value_Value_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe95f5e64_269f_4a85_ba99_4092c3ea2986);
pub const VirtualizedItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf510173e_2e71_45e9_a6e5_62f6ed8289d5);
pub const VisualEffects_Bevel: VisualEffects = VisualEffects(16i32);
pub const VisualEffects_Glow: VisualEffects = VisualEffects(4i32);
pub const VisualEffects_None: VisualEffects = VisualEffects(0i32);
pub const VisualEffects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe61a8565_aad9_46d7_9e70_4e8a8420d420);
pub const VisualEffects_Reflection: VisualEffects = VisualEffects(2i32);
pub const VisualEffects_Shadow: VisualEffects = VisualEffects(1i32);
pub const VisualEffects_SoftEdges: VisualEffects = VisualEffects(8i32);
pub const WindowInteractionState_BlockedByModalWindow: WindowInteractionState = WindowInteractionState(3i32);
pub const WindowInteractionState_Closing: WindowInteractionState = WindowInteractionState(1i32);
pub const WindowInteractionState_NotResponding: WindowInteractionState = WindowInteractionState(4i32);
pub const WindowInteractionState_ReadyForUserInteraction: WindowInteractionState = WindowInteractionState(2i32);
pub const WindowInteractionState_Running: WindowInteractionState = WindowInteractionState(0i32);
pub const WindowVisualState_Maximized: WindowVisualState = WindowVisualState(1i32);
pub const WindowVisualState_Minimized: WindowVisualState = WindowVisualState(2i32);
pub const WindowVisualState_Normal: WindowVisualState = WindowVisualState(0i32);
pub const Window_CanMaximize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x64fff53f_635d_41c1_950c_cb5adfbe28e3);
pub const Window_CanMinimize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb73b4625_5988_4b97_b4c2_a6fe6e78c8c6);
pub const Window_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe13a7242_f462_4f4d_aec1_53b28d6c3290);
pub const Window_IsModal_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xff4e6892_37b9_4fca_8532_ffe674ecfeed);
pub const Window_IsTopmost_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xef7d85d3_0937_4962_9241_b62345f24041);
pub const Window_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x27901735_c760_4994_ad11_5919e606b110);
pub const Window_WindowClosed_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xedf141f8_fa67_4e22_bbf7_944e05735ee2);
pub const Window_WindowInteractionState_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4fed26a4_0455_4fa2_b21c_c4da2db1ff9c);
pub const Window_WindowOpened_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd3e81d06_de45_4f2f_9633_de9e02fb65af);
pub const Window_WindowVisualState_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4ab7905f_e860_453e_a30a_f6431e5daad5);
pub const ZoomUnit_LargeDecrement: ZoomUnit = ZoomUnit(1i32);
pub const ZoomUnit_LargeIncrement: ZoomUnit = ZoomUnit(3i32);
pub const ZoomUnit_NoAmount: ZoomUnit = ZoomUnit(0i32);
pub const ZoomUnit_SmallDecrement: ZoomUnit = ZoomUnit(2i32);
pub const ZoomUnit_SmallIncrement: ZoomUnit = ZoomUnit(4i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACC_UTILITY_STATE_FLAGS(pub u32);
impl windows_core::TypeKind for ACC_UTILITY_STATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACC_UTILITY_STATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACC_UTILITY_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ACC_UTILITY_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ACC_UTILITY_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ACC_UTILITY_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ACC_UTILITY_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ACC_UTILITY_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ACC_UTILITY_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActiveEnd(pub i32);
impl windows_core::TypeKind for ActiveEnd {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActiveEnd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActiveEnd").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AnimationStyle(pub i32);
impl windows_core::TypeKind for AnimationStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AnimationStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AnimationStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AnnoScope(pub i32);
impl windows_core::TypeKind for AnnoScope {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AnnoScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AnnoScope").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AsyncContentLoadedState(pub i32);
impl windows_core::TypeKind for AsyncContentLoadedState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AsyncContentLoadedState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AsyncContentLoadedState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutomationElementMode(pub i32);
impl windows_core::TypeKind for AutomationElementMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutomationElementMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutomationElementMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutomationIdentifierType(pub i32);
impl windows_core::TypeKind for AutomationIdentifierType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutomationIdentifierType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutomationIdentifierType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BulletStyle(pub i32);
impl windows_core::TypeKind for BulletStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BulletStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BulletStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CapStyle(pub i32);
impl windows_core::TypeKind for CapStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CapStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CapStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CaretBidiMode(pub i32);
impl windows_core::TypeKind for CaretBidiMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CaretBidiMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CaretBidiMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CaretPosition(pub i32);
impl windows_core::TypeKind for CaretPosition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CaretPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CaretPosition").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoalesceEventsOptions(pub i32);
impl windows_core::TypeKind for CoalesceEventsOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoalesceEventsOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoalesceEventsOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConditionType(pub i32);
impl windows_core::TypeKind for ConditionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConditionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConditionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConnectionRecoveryBehaviorOptions(pub i32);
impl windows_core::TypeKind for ConnectionRecoveryBehaviorOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConnectionRecoveryBehaviorOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConnectionRecoveryBehaviorOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DockPosition(pub i32);
impl windows_core::TypeKind for DockPosition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DockPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DockPosition").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EventArgsType(pub i32);
impl windows_core::TypeKind for EventArgsType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EventArgsType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EventArgsType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ExpandCollapseState(pub i32);
impl windows_core::TypeKind for ExpandCollapseState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ExpandCollapseState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ExpandCollapseState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FillType(pub i32);
impl windows_core::TypeKind for FillType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FillType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FillType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FlowDirections(pub i32);
impl windows_core::TypeKind for FlowDirections {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FlowDirections {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FlowDirections").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HIGHCONTRASTW_FLAGS(pub u32);
impl windows_core::TypeKind for HIGHCONTRASTW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HIGHCONTRASTW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HIGHCONTRASTW_FLAGS").field(&self.0).finish()
    }
}
impl HIGHCONTRASTW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HIGHCONTRASTW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HIGHCONTRASTW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HIGHCONTRASTW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HIGHCONTRASTW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HIGHCONTRASTW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HorizontalTextAlignment(pub i32);
impl windows_core::TypeKind for HorizontalTextAlignment {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HorizontalTextAlignment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HorizontalTextAlignment").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LiveSetting(pub i32);
impl windows_core::TypeKind for LiveSetting {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LiveSetting {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LiveSetting").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NavigateDirection(pub i32);
impl windows_core::TypeKind for NavigateDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NavigateDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NavigateDirection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NormalizeState(pub i32);
impl windows_core::TypeKind for NormalizeState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NormalizeState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NormalizeState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NotificationKind(pub i32);
impl windows_core::TypeKind for NotificationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NotificationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NotificationKind").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NotificationProcessing(pub i32);
impl windows_core::TypeKind for NotificationProcessing {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NotificationProcessing {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NotificationProcessing").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OrientationType(pub i32);
impl windows_core::TypeKind for OrientationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OrientationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OrientationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OutlineStyles(pub i32);
impl windows_core::TypeKind for OutlineStyles {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OutlineStyles {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OutlineStyles").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PropertyConditionFlags(pub i32);
impl windows_core::TypeKind for PropertyConditionFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PropertyConditionFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PropertyConditionFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ProviderOptions(pub i32);
impl windows_core::TypeKind for ProviderOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ProviderOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ProviderOptions").field(&self.0).finish()
    }
}
impl ProviderOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ProviderOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ProviderOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ProviderOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ProviderOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ProviderOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ProviderType(pub i32);
impl windows_core::TypeKind for ProviderType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ProviderType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ProviderType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RowOrColumnMajor(pub i32);
impl windows_core::TypeKind for RowOrColumnMajor {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RowOrColumnMajor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RowOrColumnMajor").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERIALKEYS_FLAGS(pub u32);
impl windows_core::TypeKind for SERIALKEYS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERIALKEYS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERIALKEYS_FLAGS").field(&self.0).finish()
    }
}
impl SERIALKEYS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SERIALKEYS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SERIALKEYS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SERIALKEYS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SERIALKEYS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SERIALKEYS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SOUNDSENTRY_FLAGS(pub u32);
impl windows_core::TypeKind for SOUNDSENTRY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SOUNDSENTRY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SOUNDSENTRY_FLAGS").field(&self.0).finish()
    }
}
impl SOUNDSENTRY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SOUNDSENTRY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SOUNDSENTRY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SOUNDSENTRY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SOUNDSENTRY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SOUNDSENTRY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SOUNDSENTRY_TEXT_EFFECT(pub u32);
impl windows_core::TypeKind for SOUNDSENTRY_TEXT_EFFECT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SOUNDSENTRY_TEXT_EFFECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SOUNDSENTRY_TEXT_EFFECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SOUNDSENTRY_WINDOWS_EFFECT(pub u32);
impl windows_core::TypeKind for SOUNDSENTRY_WINDOWS_EFFECT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SOUNDSENTRY_WINDOWS_EFFECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SOUNDSENTRY_WINDOWS_EFFECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SOUND_SENTRY_GRAPHICS_EFFECT(pub u32);
impl windows_core::TypeKind for SOUND_SENTRY_GRAPHICS_EFFECT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SOUND_SENTRY_GRAPHICS_EFFECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SOUND_SENTRY_GRAPHICS_EFFECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STICKYKEYS_FLAGS(pub u32);
impl windows_core::TypeKind for STICKYKEYS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STICKYKEYS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STICKYKEYS_FLAGS").field(&self.0).finish()
    }
}
impl STICKYKEYS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for STICKYKEYS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for STICKYKEYS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for STICKYKEYS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for STICKYKEYS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for STICKYKEYS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SayAsInterpretAs(pub i32);
impl windows_core::TypeKind for SayAsInterpretAs {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SayAsInterpretAs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SayAsInterpretAs").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ScrollAmount(pub i32);
impl windows_core::TypeKind for ScrollAmount {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ScrollAmount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ScrollAmount").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StructureChangeType(pub i32);
impl windows_core::TypeKind for StructureChangeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StructureChangeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StructureChangeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SupportedTextSelection(pub i32);
impl windows_core::TypeKind for SupportedTextSelection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SupportedTextSelection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SupportedTextSelection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SynchronizedInputType(pub i32);
impl windows_core::TypeKind for SynchronizedInputType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SynchronizedInputType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SynchronizedInputType").field(&self.0).finish()
    }
}
impl SynchronizedInputType {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SynchronizedInputType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SynchronizedInputType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SynchronizedInputType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SynchronizedInputType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SynchronizedInputType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TextDecorationLineStyle(pub i32);
impl windows_core::TypeKind for TextDecorationLineStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TextDecorationLineStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TextDecorationLineStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TextEditChangeType(pub i32);
impl windows_core::TypeKind for TextEditChangeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TextEditChangeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TextEditChangeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TextPatternRangeEndpoint(pub i32);
impl windows_core::TypeKind for TextPatternRangeEndpoint {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TextPatternRangeEndpoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TextPatternRangeEndpoint").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TextUnit(pub i32);
impl windows_core::TypeKind for TextUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TextUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TextUnit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ToggleState(pub i32);
impl windows_core::TypeKind for ToggleState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ToggleState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ToggleState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TreeScope(pub i32);
impl windows_core::TypeKind for TreeScope {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TreeScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TreeScope").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TreeTraversalOptions(pub i32);
impl windows_core::TypeKind for TreeTraversalOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TreeTraversalOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TreeTraversalOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_ANNOTATIONTYPE(pub i32);
impl windows_core::TypeKind for UIA_ANNOTATIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_ANNOTATIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_ANNOTATIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_CHANGE_ID(pub i32);
impl windows_core::TypeKind for UIA_CHANGE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_CHANGE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_CHANGE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_CONTROLTYPE_ID(pub i32);
impl windows_core::TypeKind for UIA_CONTROLTYPE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_CONTROLTYPE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_CONTROLTYPE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_EVENT_ID(pub i32);
impl windows_core::TypeKind for UIA_EVENT_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_EVENT_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_EVENT_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_HEADINGLEVEL_ID(pub i32);
impl windows_core::TypeKind for UIA_HEADINGLEVEL_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_HEADINGLEVEL_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_HEADINGLEVEL_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_LANDMARKTYPE_ID(pub i32);
impl windows_core::TypeKind for UIA_LANDMARKTYPE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_LANDMARKTYPE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_LANDMARKTYPE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_METADATA_ID(pub i32);
impl windows_core::TypeKind for UIA_METADATA_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_METADATA_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_METADATA_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_PATTERN_ID(pub i32);
impl windows_core::TypeKind for UIA_PATTERN_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_PATTERN_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_PATTERN_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for UIA_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_STYLE_ID(pub i32);
impl windows_core::TypeKind for UIA_STYLE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_STYLE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_STYLE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIA_TEXTATTRIBUTE_ID(pub i32);
impl windows_core::TypeKind for UIA_TEXTATTRIBUTE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIA_TEXTATTRIBUTE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIA_TEXTATTRIBUTE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIAutomationType(pub i32);
impl windows_core::TypeKind for UIAutomationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIAutomationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIAutomationType").field(&self.0).finish()
    }
}
impl UIAutomationType {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UIAutomationType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UIAutomationType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UIAutomationType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UIAutomationType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UIAutomationType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VisualEffects(pub i32);
impl windows_core::TypeKind for VisualEffects {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VisualEffects {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VisualEffects").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WindowInteractionState(pub i32);
impl windows_core::TypeKind for WindowInteractionState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WindowInteractionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WindowInteractionState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WindowVisualState(pub i32);
impl windows_core::TypeKind for WindowVisualState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WindowVisualState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WindowVisualState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ZoomUnit(pub i32);
impl windows_core::TypeKind for ZoomUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ZoomUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ZoomUnit").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESSTIMEOUT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iTimeOutMSec: u32,
}
impl windows_core::TypeKind for ACCESSTIMEOUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESSTIMEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CAccPropServices: windows_core::GUID = windows_core::GUID::from_u128(0xb5f8350b_0548_48b1_a6ee_88bd00b4a5e7);
pub const CUIAutomation: windows_core::GUID = windows_core::GUID::from_u128(0xff48dba4_60ef_4201_aa87_54103eef594e);
pub const CUIAutomation8: windows_core::GUID = windows_core::GUID::from_u128(0xe22ad333_b25f_460c_83d0_0581107395c9);
pub const CUIAutomationRegistrar: windows_core::GUID = windows_core::GUID::from_u128(0x6e29fabf_9977_42d1_8d0e_ca7e61ad87e6);
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct ExtendedProperty {
    pub PropertyName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub PropertyValue: core::mem::ManuallyDrop<windows_core::BSTR>,
}
impl Clone for ExtendedProperty {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for ExtendedProperty {
    type TypeKind = windows_core::CopyType;
}
impl Default for ExtendedProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTERKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iWaitMSec: u32,
    pub iDelayMSec: u32,
    pub iRepeatMSec: u32,
    pub iBounceMSec: u32,
}
impl windows_core::TypeKind for FILTERKEYS {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTERKEYS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HIGHCONTRASTA {
    pub cbSize: u32,
    pub dwFlags: HIGHCONTRASTW_FLAGS,
    pub lpszDefaultScheme: windows_core::PSTR,
}
impl windows_core::TypeKind for HIGHCONTRASTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for HIGHCONTRASTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HIGHCONTRASTW {
    pub cbSize: u32,
    pub dwFlags: HIGHCONTRASTW_FLAGS,
    pub lpszDefaultScheme: windows_core::PWSTR,
}
impl windows_core::TypeKind for HIGHCONTRASTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for HIGHCONTRASTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HUIAEVENT(pub *mut core::ffi::c_void);
impl HUIAEVENT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HUIAEVENT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = UiaRemoveEvent(*self);
        }
    }
}
impl Default for HUIAEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HUIAEVENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HUIANODE(pub *mut core::ffi::c_void);
impl HUIANODE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HUIANODE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = UiaNodeRelease(*self);
        }
    }
}
impl Default for HUIANODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HUIANODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HUIAPATTERNOBJECT(pub *mut core::ffi::c_void);
impl HUIAPATTERNOBJECT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HUIAPATTERNOBJECT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = UiaPatternRelease(*self);
        }
    }
}
impl Default for HUIAPATTERNOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HUIAPATTERNOBJECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HUIATEXTRANGE(pub *mut core::ffi::c_void);
impl HUIATEXTRANGE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HUIATEXTRANGE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = UiaTextRangeRelease(*self);
        }
    }
}
impl Default for HUIATEXTRANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HUIATEXTRANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWINEVENTHOOK(pub *mut core::ffi::c_void);
impl HWINEVENTHOOK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HWINEVENTHOOK {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = UnhookWinEvent(*self);
        }
    }
}
impl Default for HWINEVENTHOOK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HWINEVENTHOOK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MOUSEKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iMaxSpeed: u32,
    pub iTimeToMaxSpeed: u32,
    pub iCtrlSpeed: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for MOUSEKEYS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MOUSEKEYS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSAAMENUINFO {
    pub dwMSAASignature: u32,
    pub cchWText: u32,
    pub pszWText: windows_core::PWSTR,
}
impl windows_core::TypeKind for MSAAMENUINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MSAAMENUINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERIALKEYSA {
    pub cbSize: u32,
    pub dwFlags: SERIALKEYS_FLAGS,
    pub lpszActivePort: windows_core::PSTR,
    pub lpszPort: windows_core::PSTR,
    pub iBaudRate: u32,
    pub iPortState: u32,
    pub iActive: u32,
}
impl windows_core::TypeKind for SERIALKEYSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERIALKEYSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERIALKEYSW {
    pub cbSize: u32,
    pub dwFlags: SERIALKEYS_FLAGS,
    pub lpszActivePort: windows_core::PWSTR,
    pub lpszPort: windows_core::PWSTR,
    pub iBaudRate: u32,
    pub iPortState: u32,
    pub iActive: u32,
}
impl windows_core::TypeKind for SERIALKEYSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERIALKEYSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOUNDSENTRYA {
    pub cbSize: u32,
    pub dwFlags: SOUNDSENTRY_FLAGS,
    pub iFSTextEffect: SOUNDSENTRY_TEXT_EFFECT,
    pub iFSTextEffectMSec: u32,
    pub iFSTextEffectColorBits: u32,
    pub iFSGrafEffect: SOUND_SENTRY_GRAPHICS_EFFECT,
    pub iFSGrafEffectMSec: u32,
    pub iFSGrafEffectColor: u32,
    pub iWindowsEffect: SOUNDSENTRY_WINDOWS_EFFECT,
    pub iWindowsEffectMSec: u32,
    pub lpszWindowsEffectDLL: windows_core::PSTR,
    pub iWindowsEffectOrdinal: u32,
}
impl windows_core::TypeKind for SOUNDSENTRYA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SOUNDSENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOUNDSENTRYW {
    pub cbSize: u32,
    pub dwFlags: SOUNDSENTRY_FLAGS,
    pub iFSTextEffect: SOUNDSENTRY_TEXT_EFFECT,
    pub iFSTextEffectMSec: u32,
    pub iFSTextEffectColorBits: u32,
    pub iFSGrafEffect: SOUND_SENTRY_GRAPHICS_EFFECT,
    pub iFSGrafEffectMSec: u32,
    pub iFSGrafEffectColor: u32,
    pub iWindowsEffect: SOUNDSENTRY_WINDOWS_EFFECT,
    pub iWindowsEffectMSec: u32,
    pub lpszWindowsEffectDLL: windows_core::PWSTR,
    pub iWindowsEffectOrdinal: u32,
}
impl windows_core::TypeKind for SOUNDSENTRYW {
    type TypeKind = windows_core::CopyType;
}
impl Default for SOUNDSENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STICKYKEYS {
    pub cbSize: u32,
    pub dwFlags: STICKYKEYS_FLAGS,
}
impl windows_core::TypeKind for STICKYKEYS {
    type TypeKind = windows_core::CopyType;
}
impl Default for STICKYKEYS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOGGLEKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for TOGGLEKEYS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOGGLEKEYS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIAutomationEventInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for UIAutomationEventInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for UIAutomationEventInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIAutomationMethodInfo {
    pub pProgrammaticName: windows_core::PCWSTR,
    pub doSetFocus: super::super::Foundation::BOOL,
    pub cInParameters: u32,
    pub cOutParameters: u32,
    pub pParameterTypes: *mut UIAutomationType,
    pub pParameterNames: *const windows_core::PCWSTR,
}
impl windows_core::TypeKind for UIAutomationMethodInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for UIAutomationMethodInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIAutomationParameter {
    pub r#type: UIAutomationType,
    pub pData: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for UIAutomationParameter {
    type TypeKind = windows_core::CopyType;
}
impl Default for UIAutomationParameter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct UIAutomationPatternInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
    pub providerInterfaceId: windows_core::GUID,
    pub clientInterfaceId: windows_core::GUID,
    pub cProperties: u32,
    pub pProperties: *mut UIAutomationPropertyInfo,
    pub cMethods: u32,
    pub pMethods: *mut UIAutomationMethodInfo,
    pub cEvents: u32,
    pub pEvents: *mut UIAutomationEventInfo,
    pub pPatternHandler: core::mem::ManuallyDrop<Option<IUIAutomationPatternHandler>>,
}
impl Clone for UIAutomationPatternInfo {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for UIAutomationPatternInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for UIAutomationPatternInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIAutomationPropertyInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
    pub r#type: UIAutomationType,
}
impl windows_core::TypeKind for UIAutomationPropertyInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for UIAutomationPropertyInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaAndOrCondition {
    pub ConditionType: ConditionType,
    pub ppConditions: *mut *mut UiaCondition,
    pub cConditions: i32,
}
impl windows_core::TypeKind for UiaAndOrCondition {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaAndOrCondition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiaAsyncContentLoadedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub AsyncContentLoadedState: AsyncContentLoadedState,
    pub PercentComplete: f64,
}
impl windows_core::TypeKind for UiaAsyncContentLoadedEventArgs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaAsyncContentLoadedEventArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaCacheRequest {
    pub pViewCondition: *mut UiaCondition,
    pub Scope: TreeScope,
    pub pProperties: *mut i32,
    pub cProperties: i32,
    pub pPatterns: *mut i32,
    pub cPatterns: i32,
    pub automationElementMode: AutomationElementMode,
}
impl windows_core::TypeKind for UiaCacheRequest {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaCacheRequest {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct UiaChangeInfo {
    pub uiaId: i32,
    pub payload: core::mem::ManuallyDrop<windows_core::VARIANT>,
    pub extraInfo: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
impl Clone for UiaChangeInfo {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for UiaChangeInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaChangeInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaChangesEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub EventIdCount: i32,
    pub pUiaChanges: *mut UiaChangeInfo,
}
impl windows_core::TypeKind for UiaChangesEventArgs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaChangesEventArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaCondition {
    pub ConditionType: ConditionType,
}
impl windows_core::TypeKind for UiaCondition {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaCondition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
}
impl windows_core::TypeKind for UiaEventArgs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaEventArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaFindParams {
    pub MaxDepth: i32,
    pub FindFirst: super::super::Foundation::BOOL,
    pub ExcludeRoot: super::super::Foundation::BOOL,
    pub pFindCondition: *mut UiaCondition,
}
impl windows_core::TypeKind for UiaFindParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaFindParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaNotCondition {
    pub ConditionType: ConditionType,
    pub pCondition: *mut UiaCondition,
}
impl windows_core::TypeKind for UiaNotCondition {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaNotCondition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiaPoint {
    pub x: f64,
    pub y: f64,
}
impl windows_core::TypeKind for UiaPoint {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct UiaPropertyChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: UIA_EVENT_ID,
    pub PropertyId: i32,
    pub OldValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
    pub NewValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
impl Clone for UiaPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for UiaPropertyChangedEventArgs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaPropertyChangedEventArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct UiaPropertyCondition {
    pub ConditionType: ConditionType,
    pub PropertyId: UIA_PROPERTY_ID,
    pub Value: core::mem::ManuallyDrop<windows_core::VARIANT>,
    pub Flags: PropertyConditionFlags,
}
impl Clone for UiaPropertyCondition {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for UiaPropertyCondition {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaPropertyCondition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiaRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}
impl windows_core::TypeKind for UiaRect {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaRect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaStructureChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub StructureChangeType: StructureChangeType,
    pub pRuntimeId: *mut i32,
    pub cRuntimeIdLen: i32,
}
impl windows_core::TypeKind for UiaStructureChangedEventArgs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaStructureChangedEventArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaTextEditTextChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub TextEditChangeType: TextEditChangeType,
    pub pTextChange: *mut super::super::System::Com::SAFEARRAY,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for UiaTextEditTextChangedEventArgs {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for UiaTextEditTextChangedEventArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiaWindowClosedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub pRuntimeId: *mut i32,
    pub cRuntimeIdLen: i32,
}
impl windows_core::TypeKind for UiaWindowClosedEventArgs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiaWindowClosedEventArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
pub type LPFNACCESSIBLECHILDREN = Option<unsafe extern "system" fn(pacccontainer: Option<IAccessible>, ichildstart: i32, cchildren: i32, rgvarchildren: *mut windows_core::VARIANT, pcobtained: *mut i32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_Com")]
pub type LPFNACCESSIBLEOBJECTFROMPOINT = Option<unsafe extern "system" fn(ptscreen: super::super::Foundation::POINT, ppacc: *mut Option<IAccessible>, pvarchild: *mut windows_core::VARIANT) -> windows_core::HRESULT>;
pub type LPFNACCESSIBLEOBJECTFROMWINDOW = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, dwid: u32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type LPFNCREATESTDACCESSIBLEOBJECT = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, idobject: i32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type LPFNLRESULTFROMOBJECT = Option<unsafe extern "system" fn(riid: *const windows_core::GUID, wparam: super::super::Foundation::WPARAM, punk: Option<windows_core::IUnknown>) -> super::super::Foundation::LRESULT>;
pub type LPFNOBJECTFROMLRESULT = Option<unsafe extern "system" fn(lresult: super::super::Foundation::LRESULT, riid: *const windows_core::GUID, wparam: super::super::Foundation::WPARAM, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_Com")]
pub type UiaEventCallback = Option<unsafe extern "system" fn(pargs: *mut UiaEventArgs, prequesteddata: *mut super::super::System::Com::SAFEARRAY, ptreestructure: windows_core::BSTR)>;
#[cfg(feature = "Win32_System_Com")]
pub type UiaProviderCallback = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, providertype: ProviderType) -> *mut super::super::System::Com::SAFEARRAY>;
pub type WINEVENTPROC = Option<unsafe extern "system" fn(hwineventhook: HWINEVENTHOOK, event: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, ideventthread: u32, dwmseventtime: u32)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");

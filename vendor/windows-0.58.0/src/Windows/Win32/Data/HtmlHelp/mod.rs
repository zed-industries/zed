#[inline]
pub unsafe fn HtmlHelpA<P0, P1>(hwndcaller: P0, pszfile: P1, ucommand: HTML_HELP_COMMAND, dwdata: usize) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("hhctrl.ocx" "system" fn HtmlHelpA(hwndcaller : super::super::Foundation:: HWND, pszfile : windows_core::PCSTR, ucommand : u32, dwdata : usize) -> super::super::Foundation:: HWND);
    HtmlHelpA(hwndcaller.param().abi(), pszfile.param().abi(), ucommand.0 as _, dwdata)
}
#[inline]
pub unsafe fn HtmlHelpW<P0, P1>(hwndcaller: P0, pszfile: P1, ucommand: HTML_HELP_COMMAND, dwdata: usize) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("hhctrl.ocx" "system" fn HtmlHelpW(hwndcaller : super::super::Foundation:: HWND, pszfile : windows_core::PCWSTR, ucommand : u32, dwdata : usize) -> super::super::Foundation:: HWND);
    HtmlHelpW(hwndcaller.param().abi(), pszfile.param().abi(), ucommand.0 as _, dwdata)
}
windows_core::imp::define_interface!(IITDatabase, IITDatabase_Vtbl, 0x8fa0d5a2_dedf_11d0_9a61_00c04fb68bf7);
impl core::ops::Deref for IITDatabase {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IITDatabase, windows_core::IUnknown);
impl IITDatabase {
    pub unsafe fn Open<P0, P1>(&self, lpszhost: P0, lpszmoniker: P1, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), lpszhost.param().abi(), lpszmoniker.param().abi(), dwflags).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateObject(&self, rclsid: *const windows_core::GUID, pdwobjinstance: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateObject)(windows_core::Interface::as_raw(self), rclsid, pdwobjinstance).ok()
    }
    pub unsafe fn GetObject(&self, dwobjinstance: u32, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), dwobjinstance, riid, ppvobj).ok()
    }
    pub unsafe fn GetObjectPersistence<P0, P1>(&self, lpwszobject: P0, dwobjinstance: u32, ppvpersistence: *mut *mut core::ffi::c_void, fstream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetObjectPersistence)(windows_core::Interface::as_raw(self), lpwszobject.param().abi(), dwobjinstance, ppvpersistence, fstream.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IITDatabase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectPersistence: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IITPropList, IITPropList_Vtbl, 0x1f403bb1_9997_11d0_a850_00aa006c7d01);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IITPropList {
    type Target = super::super::System::Com::IPersistStreamInit;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IITPropList, windows_core::IUnknown, super::super::System::Com::IPersist, super::super::System::Com::IPersistStreamInit);
#[cfg(feature = "Win32_System_Com")]
impl IITPropList {
    pub unsafe fn Set<P0>(&self, propid: u32, lpszwstring: P0, dwoperation: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), propid, lpszwstring.param().abi(), dwoperation).ok()
    }
    pub unsafe fn Set2(&self, propid: u32, lpvdata: *mut core::ffi::c_void, cbdata: u32, dwoperation: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set2)(windows_core::Interface::as_raw(self), propid, lpvdata, cbdata, dwoperation).ok()
    }
    pub unsafe fn Set3(&self, propid: u32, dwdata: u32, dwoperation: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set3)(windows_core::Interface::as_raw(self), propid, dwdata, dwoperation).ok()
    }
    pub unsafe fn Add(&self, prop: *mut CProperty) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), prop).ok()
    }
    pub unsafe fn Get(&self, propid: u32, property: *mut CProperty) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), propid, property).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPersist<P0>(&self, fpersist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPersist)(windows_core::Interface::as_raw(self), fpersist.param().abi()).ok()
    }
    pub unsafe fn SetPersist2<P0>(&self, propid: u32, fpersist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPersist2)(windows_core::Interface::as_raw(self), propid, fpersist.param().abi()).ok()
    }
    pub unsafe fn GetFirst(&self, property: *mut CProperty) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFirst)(windows_core::Interface::as_raw(self), property).ok()
    }
    pub unsafe fn GetNext(&self, property: *mut CProperty) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNext)(windows_core::Interface::as_raw(self), property).ok()
    }
    pub unsafe fn GetPropCount(&self, cprop: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropCount)(windows_core::Interface::as_raw(self), cprop).ok()
    }
    pub unsafe fn SaveHeader(&self, lpvdata: *mut core::ffi::c_void, dwhdrsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveHeader)(windows_core::Interface::as_raw(self), lpvdata, dwhdrsize).ok()
    }
    pub unsafe fn SaveData(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveData)(windows_core::Interface::as_raw(self), lpvheader, dwhdrsize, lpvdata, dwbufsize).ok()
    }
    pub unsafe fn GetHeaderSize(&self, dwhdrsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHeaderSize)(windows_core::Interface::as_raw(self), dwhdrsize).ok()
    }
    pub unsafe fn GetDataSize(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self), lpvheader, dwhdrsize, dwdatasize).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveDataToStream<P0>(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SaveDataToStream)(windows_core::Interface::as_raw(self), lpvheader, dwhdrsize, pstream.param().abi()).ok()
    }
    pub unsafe fn LoadFromMem(&self, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadFromMem)(windows_core::Interface::as_raw(self), lpvdata, dwbufsize).ok()
    }
    pub unsafe fn SaveToMem(&self, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveToMem)(windows_core::Interface::as_raw(self), lpvdata, dwbufsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IITPropList_Vtbl {
    pub base__: super::super::System::Com::IPersistStreamInit_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Set2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Set3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CProperty) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CProperty) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPersist: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetPersist2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFirst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CProperty) -> windows_core::HRESULT,
    pub GetNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CProperty) -> windows_core::HRESULT,
    pub GetPropCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SaveHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetHeaderSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveDataToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveDataToStream: usize,
    pub LoadFromMem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveToMem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IITResultSet, IITResultSet_Vtbl, 0x3bb91d41_998b_11d0_a850_00aa006c7d01);
impl core::ops::Deref for IITResultSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IITResultSet, windows_core::IUnknown);
impl IITResultSet {
    pub unsafe fn SetColumnPriority(&self, lcolumnindex: i32, columnpriority: PRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColumnPriority)(windows_core::Interface::as_raw(self), lcolumnindex, columnpriority).ok()
    }
    pub unsafe fn SetColumnHeap(&self, lcolumnindex: i32, lpvheap: *mut core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColumnHeap)(windows_core::Interface::as_raw(self), lcolumnindex, lpvheap, pfncolheapfree).ok()
    }
    pub unsafe fn SetKeyProp(&self, propid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKeyProp)(windows_core::Interface::as_raw(self), propid).ok()
    }
    pub unsafe fn Add(&self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), propid, dwdefaultdata, priority).ok()
    }
    pub unsafe fn Add2<P0>(&self, propid: u32, lpszwdefault: P0, priority: PRIORITY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Add2)(windows_core::Interface::as_raw(self), propid, lpszwdefault.param().abi(), priority).ok()
    }
    pub unsafe fn Add3(&self, propid: u32, lpvdefaultdata: *mut core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add3)(windows_core::Interface::as_raw(self), propid, lpvdefaultdata, cbdata, priority).ok()
    }
    pub unsafe fn Add4(&self, lpvhdr: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add4)(windows_core::Interface::as_raw(self), lpvhdr).ok()
    }
    pub unsafe fn Append(&self, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), lpvhdr, lpvdata).ok()
    }
    pub unsafe fn Set(&self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), lrowindex, lcolumnindex, lpvdata, cbdata).ok()
    }
    pub unsafe fn Set2<P0>(&self, lrowindex: i32, lcolumnindex: i32, lpwstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Set2)(windows_core::Interface::as_raw(self), lrowindex, lcolumnindex, lpwstr.param().abi()).ok()
    }
    pub unsafe fn Set3(&self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set3)(windows_core::Interface::as_raw(self), lrowindex, lcolumnindex, dwdata).ok()
    }
    pub unsafe fn Set4(&self, lrowindex: i32, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set4)(windows_core::Interface::as_raw(self), lrowindex, lpvhdr, lpvdata).ok()
    }
    pub unsafe fn Copy<P0>(&self, prscopy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IITResultSet>,
    {
        (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), prscopy.param().abi()).ok()
    }
    pub unsafe fn AppendRows<P0>(&self, pressrc: P0, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IITResultSet>,
    {
        (windows_core::Interface::vtable(self).AppendRows)(windows_core::Interface::as_raw(self), pressrc.param().abi(), lrowsrcfirst, csrcrows, lrowfirstdest).ok()
    }
    pub unsafe fn Get(&self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), lrowindex, lcolumnindex, prop).ok()
    }
    pub unsafe fn GetKeyProp(&self, keypropid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetKeyProp)(windows_core::Interface::as_raw(self), keypropid).ok()
    }
    pub unsafe fn GetColumnPriority(&self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnPriority)(windows_core::Interface::as_raw(self), lcolumnindex, columnpriority).ok()
    }
    pub unsafe fn GetRowCount(&self, lnumberofrows: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRowCount)(windows_core::Interface::as_raw(self), lnumberofrows).ok()
    }
    pub unsafe fn GetColumnCount(&self, lnumberofcolumns: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnCount)(windows_core::Interface::as_raw(self), lnumberofcolumns).ok()
    }
    pub unsafe fn GetColumn(&self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), lcolumnindex, propid, dwtype, lpvdefaultvalue, cbsize, columnpriority).ok()
    }
    pub unsafe fn GetColumn2(&self, lcolumnindex: i32, propid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumn2)(windows_core::Interface::as_raw(self), lcolumnindex, propid).ok()
    }
    pub unsafe fn GetColumnFromPropID(&self, propid: u32, lcolumnindex: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnFromPropID)(windows_core::Interface::as_raw(self), propid, lcolumnindex).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ClearRows(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearRows)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Free(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsCompleted(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsCompleted)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause<P0>(&self, fpause: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self), fpause.param().abi()).ok()
    }
    pub unsafe fn GetRowStatus(&self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRowStatus)(windows_core::Interface::as_raw(self), lrowfirst, crows, lprowstatus).ok()
    }
    pub unsafe fn GetColumnStatus(&self, lpcolstatus: *mut COLUMNSTATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnStatus)(windows_core::Interface::as_raw(self), lpcolstatus).ok()
    }
}
#[repr(C)]
pub struct IITResultSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetColumnPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32, PRIORITY) -> windows_core::HRESULT,
    pub SetColumnHeap: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, PFNCOLHEAPFREE) -> windows_core::HRESULT,
    pub SetKeyProp: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, PRIORITY) -> windows_core::HRESULT,
    pub Add2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, PRIORITY) -> windows_core::HRESULT,
    pub Add3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, PRIORITY) -> windows_core::HRESULT,
    pub Add4: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Set2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Set3: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, usize) -> windows_core::HRESULT,
    pub Set4: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendRows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut CProperty) -> windows_core::HRESULT,
    pub GetKeyProp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetColumnPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut PRIORITY) -> windows_core::HRESULT,
    pub GetRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32, *mut PRIORITY) -> windows_core::HRESULT,
    pub GetColumn2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    pub GetColumnFromPropID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearRows: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetRowStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut ROWSTATUS) -> windows_core::HRESULT,
    pub GetColumnStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COLUMNSTATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStemSink, IStemSink_Vtbl, 0xfe77c330_7f42_11ce_be57_00aa0051fe20);
impl core::ops::Deref for IStemSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStemSink, windows_core::IUnknown);
impl IStemSink {
    pub unsafe fn PutAltWord<P0>(&self, pwcinbuf: P0, cwc: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutAltWord)(windows_core::Interface::as_raw(self), pwcinbuf.param().abi(), cwc).ok()
    }
    pub unsafe fn PutWord<P0>(&self, pwcinbuf: P0, cwc: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutWord)(windows_core::Interface::as_raw(self), pwcinbuf.param().abi(), cwc).ok()
    }
}
#[repr(C)]
pub struct IStemSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutAltWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub PutWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStemmerConfig, IStemmerConfig_Vtbl, 0x8fa0d5a7_dedf_11d0_9a61_00c04fb68bf7);
impl core::ops::Deref for IStemmerConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStemmerConfig, windows_core::IUnknown);
impl IStemmerConfig {
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLocaleInfo)(windows_core::Interface::as_raw(self), dwcodepageid, lcid).ok()
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocaleInfo)(windows_core::Interface::as_raw(self), pdwcodepageid, plcid).ok()
    }
    pub unsafe fn SetControlInfo(&self, grfstemflags: u32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetControlInfo)(windows_core::Interface::as_raw(self), grfstemflags, dwreserved).ok()
    }
    pub unsafe fn GetControlInfo(&self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetControlInfo)(windows_core::Interface::as_raw(self), pgrfstemflags, pdwreserved).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalStemmerData<P0>(&self, pstream: P0, dwextdatatype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).LoadExternalStemmerData)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwextdatatype).ok()
    }
}
#[repr(C)]
pub struct IStemmerConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLocaleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetLocaleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadExternalStemmerData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadExternalStemmerData: usize,
}
windows_core::imp::define_interface!(IWordBreakerConfig, IWordBreakerConfig_Vtbl, 0x8fa0d5a6_dedf_11d0_9a61_00c04fb68bf7);
impl core::ops::Deref for IWordBreakerConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWordBreakerConfig, windows_core::IUnknown);
impl IWordBreakerConfig {
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLocaleInfo)(windows_core::Interface::as_raw(self), dwcodepageid, lcid).ok()
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocaleInfo)(windows_core::Interface::as_raw(self), pdwcodepageid, plcid).ok()
    }
    pub unsafe fn SetBreakWordType(&self, dwbreakwordtype: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBreakWordType)(windows_core::Interface::as_raw(self), dwbreakwordtype).ok()
    }
    pub unsafe fn GetBreakWordType(&self, pdwbreakwordtype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBreakWordType)(windows_core::Interface::as_raw(self), pdwbreakwordtype).ok()
    }
    pub unsafe fn SetControlInfo(&self, grfbreakflags: u32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetControlInfo)(windows_core::Interface::as_raw(self), grfbreakflags, dwreserved).ok()
    }
    pub unsafe fn GetControlInfo(&self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetControlInfo)(windows_core::Interface::as_raw(self), pgrfbreakflags, pdwreserved).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalBreakerData<P0>(&self, pstream: P0, dwextdatatype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).LoadExternalBreakerData)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwextdatatype).ok()
    }
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn SetWordStemmer<P0>(&self, rclsid: *const windows_core::GUID, pstemmer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Search::IStemmer>,
    {
        (windows_core::Interface::vtable(self).SetWordStemmer)(windows_core::Interface::as_raw(self), rclsid, pstemmer.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn GetWordStemmer(&self) -> windows_core::Result<super::super::System::Search::IStemmer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWordStemmer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWordBreakerConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLocaleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetLocaleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetBreakWordType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetBreakWordType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadExternalBreakerData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadExternalBreakerData: usize,
    #[cfg(feature = "Win32_System_Search")]
    pub SetWordStemmer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))]
    SetWordStemmer: usize,
    #[cfg(feature = "Win32_System_Search")]
    pub GetWordStemmer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))]
    GetWordStemmer: usize,
}
pub const CLSID_IITCmdInt: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa2_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITDatabase: windows_core::GUID = windows_core::GUID::from_u128(0x66673452_8c23_11d0_a84e_00aa006c7d01);
pub const CLSID_IITDatabaseLocal: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa9_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITGroupUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa4_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITIndexBuild: windows_core::GUID = windows_core::GUID::from_u128(0x8fa0d5aa_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_IITPropList: windows_core::GUID = windows_core::GUID::from_u128(0x4662daae_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITResultSet: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa7_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITSvMgr: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa3_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITWWFilterBuild: windows_core::GUID = windows_core::GUID::from_u128(0x8fa0d5ab_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_IITWordWheel: windows_core::GUID = windows_core::GUID::from_u128(0xd73725c2_8c12_11d0_a84e_00aa006c7d01);
pub const CLSID_IITWordWheelLocal: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa8_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITWordWheelUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa5_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_ITEngStemmer: windows_core::GUID = windows_core::GUID::from_u128(0x8fa0d5a8_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_ITStdBreaker: windows_core::GUID = windows_core::GUID::from_u128(0x4662daaf_d393_11d0_9a56_00c04fb68bf7);
pub const E_ALL_WILD: windows_core::HRESULT = windows_core::HRESULT(0x80001055_u32 as _);
pub const E_ALREADYINIT: windows_core::HRESULT = windows_core::HRESULT(0x80001083_u32 as _);
pub const E_ALREADYOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80001013_u32 as _);
pub const E_ASSERT: windows_core::HRESULT = windows_core::HRESULT(0x80001006_u32 as _);
pub const E_BADBREAKER: windows_core::HRESULT = windows_core::HRESULT(0x80001053_u32 as _);
pub const E_BADFILE: windows_core::HRESULT = windows_core::HRESULT(0x80001003_u32 as _);
pub const E_BADFILTERSIZE: windows_core::HRESULT = windows_core::HRESULT(0x80001018_u32 as _);
pub const E_BADFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80001004_u32 as _);
pub const E_BADINDEXFLAGS: windows_core::HRESULT = windows_core::HRESULT(0x80001060_u32 as _);
pub const E_BADPARAM: windows_core::HRESULT = windows_core::HRESULT(0x80001011_u32 as _);
pub const E_BADRANGEOP: windows_core::HRESULT = windows_core::HRESULT(0x8000105D_u32 as _);
pub const E_BADVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80001054_u32 as _);
pub const E_BADVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80001002_u32 as _);
pub const E_CANTFINDDLL: windows_core::HRESULT = windows_core::HRESULT(0x8000100E_u32 as _);
pub const E_DISKFULL: windows_core::HRESULT = windows_core::HRESULT(0x80001038_u32 as _);
pub const E_DUPLICATE: windows_core::HRESULT = windows_core::HRESULT(0x80001001_u32 as _);
pub const E_EXPECTEDTERM: windows_core::HRESULT = windows_core::HRESULT(0x80001057_u32 as _);
pub const E_FILECLOSE: windows_core::HRESULT = windows_core::HRESULT(0x80001031_u32 as _);
pub const E_FILECREATE: windows_core::HRESULT = windows_core::HRESULT(0x80001030_u32 as _);
pub const E_FILEDELETE: windows_core::HRESULT = windows_core::HRESULT(0x80001035_u32 as _);
pub const E_FILEINVALID: windows_core::HRESULT = windows_core::HRESULT(0x80001036_u32 as _);
pub const E_FILENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80001037_u32 as _);
pub const E_FILEREAD: windows_core::HRESULT = windows_core::HRESULT(0x80001032_u32 as _);
pub const E_FILESEEK: windows_core::HRESULT = windows_core::HRESULT(0x80001033_u32 as _);
pub const E_FILEWRITE: windows_core::HRESULT = windows_core::HRESULT(0x80001034_u32 as _);
pub const E_GETLASTERROR: windows_core::HRESULT = windows_core::HRESULT(0x80001010_u32 as _);
pub const E_GROUPIDTOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x8000100A_u32 as _);
pub const E_INTERRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80001007_u32 as _);
pub const E_INVALIDSTATE: windows_core::HRESULT = windows_core::HRESULT(0x80001012_u32 as _);
pub const E_MISSINGPROP: windows_core::HRESULT = windows_core::HRESULT(0x80001080_u32 as _);
pub const E_MISSLPAREN: windows_core::HRESULT = windows_core::HRESULT(0x80001058_u32 as _);
pub const E_MISSQUOTE: windows_core::HRESULT = windows_core::HRESULT(0x8000105A_u32 as _);
pub const E_MISSRPAREN: windows_core::HRESULT = windows_core::HRESULT(0x80001059_u32 as _);
pub const E_NAMETOOLONG: windows_core::HRESULT = windows_core::HRESULT(0x80001020_u32 as _);
pub const E_NOHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x8000100F_u32 as _);
pub const E_NOKEYPROP: windows_core::HRESULT = windows_core::HRESULT(0x80001087_u32 as _);
pub const E_NOMERGEDDATA: windows_core::HRESULT = windows_core::HRESULT(0x8000100C_u32 as _);
pub const E_NOPERMISSION: windows_core::HRESULT = windows_core::HRESULT(0x80001005_u32 as _);
pub const E_NOSTEMMER: windows_core::HRESULT = windows_core::HRESULT(0x80001062_u32 as _);
pub const E_NOTEXIST: windows_core::HRESULT = windows_core::HRESULT(0x80001000_u32 as _);
pub const E_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8000100D_u32 as _);
pub const E_NOTINIT: windows_core::HRESULT = windows_core::HRESULT(0x80001084_u32 as _);
pub const E_NOTOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80001013_u32 as _);
pub const E_NOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80001008_u32 as _);
pub const E_NULLQUERY: windows_core::HRESULT = windows_core::HRESULT(0x8000105B_u32 as _);
pub const E_OUTOFRANGE: windows_core::HRESULT = windows_core::HRESULT(0x80001009_u32 as _);
pub const E_PROPLISTEMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80001082_u32 as _);
pub const E_PROPLISTNOTEMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80001081_u32 as _);
pub const E_RESULTSETEMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80001085_u32 as _);
pub const E_STOPWORD: windows_core::HRESULT = windows_core::HRESULT(0x8000105C_u32 as _);
pub const E_TOODEEP: windows_core::HRESULT = windows_core::HRESULT(0x80001056_u32 as _);
pub const E_TOOMANYCOLUMNS: windows_core::HRESULT = windows_core::HRESULT(0x80001086_u32 as _);
pub const E_TOOMANYDUPS: windows_core::HRESULT = windows_core::HRESULT(0x80001051_u32 as _);
pub const E_TOOMANYOBJECTS: windows_core::HRESULT = windows_core::HRESULT(0x80001019_u32 as _);
pub const E_TOOMANYTITLES: windows_core::HRESULT = windows_core::HRESULT(0x8000100B_u32 as _);
pub const E_TOOMANYTOPICS: windows_core::HRESULT = windows_core::HRESULT(0x80001050_u32 as _);
pub const E_TREETOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x80001052_u32 as _);
pub const E_UNKNOWN_TRANSPORT: windows_core::HRESULT = windows_core::HRESULT(0x80001016_u32 as _);
pub const E_UNMATCHEDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x8000105E_u32 as _);
pub const E_UNSUPPORTED_TRANSPORT: windows_core::HRESULT = windows_core::HRESULT(0x80001017_u32 as _);
pub const E_WILD_IN_DTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80001061_u32 as _);
pub const E_WORDTOOLONG: windows_core::HRESULT = windows_core::HRESULT(0x8000105F_u32 as _);
pub const HHACT_BACK: i32 = 7i32;
pub const HHACT_CONTRACT: i32 = 6i32;
pub const HHACT_CUSTOMIZE: i32 = 16i32;
pub const HHACT_EXPAND: i32 = 5i32;
pub const HHACT_FORWARD: i32 = 8i32;
pub const HHACT_HIGHLIGHT: i32 = 15i32;
pub const HHACT_HOME: i32 = 11i32;
pub const HHACT_JUMP1: i32 = 17i32;
pub const HHACT_JUMP2: i32 = 18i32;
pub const HHACT_LAST_ENUM: i32 = 23i32;
pub const HHACT_NOTES: i32 = 22i32;
pub const HHACT_OPTIONS: i32 = 13i32;
pub const HHACT_PRINT: i32 = 14i32;
pub const HHACT_REFRESH: i32 = 10i32;
pub const HHACT_STOP: i32 = 9i32;
pub const HHACT_SYNC: i32 = 12i32;
pub const HHACT_TAB_CONTENTS: i32 = 0i32;
pub const HHACT_TAB_FAVORITES: i32 = 4i32;
pub const HHACT_TAB_HISTORY: i32 = 3i32;
pub const HHACT_TAB_INDEX: i32 = 1i32;
pub const HHACT_TAB_SEARCH: i32 = 2i32;
pub const HHACT_TOC_NEXT: i32 = 20i32;
pub const HHACT_TOC_PREV: i32 = 21i32;
pub const HHACT_ZOOM: i32 = 19i32;
pub const HHN_FIRST: u32 = 4294966436u32;
pub const HHN_LAST: u32 = 4294966417u32;
pub const HHN_NAVCOMPLETE: u32 = 4294966436u32;
pub const HHN_TRACK: u32 = 4294966435u32;
pub const HHN_WINDOW_CREATE: u32 = 4294966434u32;
pub const HHWIN_BUTTON_BACK: u32 = 4u32;
pub const HHWIN_BUTTON_BROWSE_BCK: u32 = 256u32;
pub const HHWIN_BUTTON_BROWSE_FWD: u32 = 128u32;
pub const HHWIN_BUTTON_CONTENTS: u32 = 1024u32;
pub const HHWIN_BUTTON_EXPAND: u32 = 2u32;
pub const HHWIN_BUTTON_FAVORITES: u32 = 131072u32;
pub const HHWIN_BUTTON_FORWARD: u32 = 8u32;
pub const HHWIN_BUTTON_HISTORY: u32 = 65536u32;
pub const HHWIN_BUTTON_HOME: u32 = 64u32;
pub const HHWIN_BUTTON_INDEX: u32 = 16384u32;
pub const HHWIN_BUTTON_JUMP1: u32 = 262144u32;
pub const HHWIN_BUTTON_JUMP2: u32 = 524288u32;
pub const HHWIN_BUTTON_NOTES: u32 = 512u32;
pub const HHWIN_BUTTON_OPTIONS: u32 = 4096u32;
pub const HHWIN_BUTTON_PRINT: u32 = 8192u32;
pub const HHWIN_BUTTON_REFRESH: u32 = 32u32;
pub const HHWIN_BUTTON_SEARCH: u32 = 32768u32;
pub const HHWIN_BUTTON_STOP: u32 = 16u32;
pub const HHWIN_BUTTON_SYNC: u32 = 2048u32;
pub const HHWIN_BUTTON_TOC_NEXT: u32 = 2097152u32;
pub const HHWIN_BUTTON_TOC_PREV: u32 = 4194304u32;
pub const HHWIN_BUTTON_ZOOM: u32 = 1048576u32;
pub const HHWIN_NAVTAB_BOTTOM: i32 = 2i32;
pub const HHWIN_NAVTAB_LEFT: i32 = 1i32;
pub const HHWIN_NAVTAB_TOP: i32 = 0i32;
pub const HHWIN_NAVTYPE_AUTHOR: i32 = 5i32;
pub const HHWIN_NAVTYPE_CUSTOM_FIRST: i32 = 11i32;
pub const HHWIN_NAVTYPE_FAVORITES: i32 = 3i32;
pub const HHWIN_NAVTYPE_HISTORY: i32 = 4i32;
pub const HHWIN_NAVTYPE_INDEX: i32 = 1i32;
pub const HHWIN_NAVTYPE_SEARCH: i32 = 2i32;
pub const HHWIN_NAVTYPE_TOC: i32 = 0i32;
pub const HHWIN_PARAM_CUR_TAB: u32 = 8192u32;
pub const HHWIN_PARAM_EXPANSION: u32 = 512u32;
pub const HHWIN_PARAM_EXSTYLES: u32 = 8u32;
pub const HHWIN_PARAM_HISTORY_COUNT: u32 = 4096u32;
pub const HHWIN_PARAM_INFOTYPES: u32 = 128u32;
pub const HHWIN_PARAM_NAV_WIDTH: u32 = 32u32;
pub const HHWIN_PARAM_PROPERTIES: u32 = 2u32;
pub const HHWIN_PARAM_RECT: u32 = 16u32;
pub const HHWIN_PARAM_SHOWSTATE: u32 = 64u32;
pub const HHWIN_PARAM_STYLES: u32 = 4u32;
pub const HHWIN_PARAM_TABORDER: u32 = 2048u32;
pub const HHWIN_PARAM_TABPOS: u32 = 1024u32;
pub const HHWIN_PARAM_TB_FLAGS: u32 = 256u32;
pub const HHWIN_PROP_AUTO_SYNC: u32 = 256u32;
pub const HHWIN_PROP_CHANGE_TITLE: u32 = 8192u32;
pub const HHWIN_PROP_MENU: u32 = 65536u32;
pub const HHWIN_PROP_NAV_ONLY_WIN: u32 = 16384u32;
pub const HHWIN_PROP_NODEF_EXSTYLES: u32 = 16u32;
pub const HHWIN_PROP_NODEF_STYLES: u32 = 8u32;
pub const HHWIN_PROP_NOTB_TEXT: u32 = 64u32;
pub const HHWIN_PROP_NOTITLEBAR: u32 = 4u32;
pub const HHWIN_PROP_NO_TOOLBAR: u32 = 32768u32;
pub const HHWIN_PROP_ONTOP: u32 = 2u32;
pub const HHWIN_PROP_POST_QUIT: u32 = 128u32;
pub const HHWIN_PROP_TAB_ADVSEARCH: u32 = 131072u32;
pub const HHWIN_PROP_TAB_AUTOHIDESHOW: u32 = 1u32;
pub const HHWIN_PROP_TAB_CUSTOM1: u32 = 524288u32;
pub const HHWIN_PROP_TAB_CUSTOM2: u32 = 1048576u32;
pub const HHWIN_PROP_TAB_CUSTOM3: u32 = 2097152u32;
pub const HHWIN_PROP_TAB_CUSTOM4: u32 = 4194304u32;
pub const HHWIN_PROP_TAB_CUSTOM5: u32 = 8388608u32;
pub const HHWIN_PROP_TAB_CUSTOM6: u32 = 16777216u32;
pub const HHWIN_PROP_TAB_CUSTOM7: u32 = 33554432u32;
pub const HHWIN_PROP_TAB_CUSTOM8: u32 = 67108864u32;
pub const HHWIN_PROP_TAB_CUSTOM9: u32 = 134217728u32;
pub const HHWIN_PROP_TAB_FAVORITES: u32 = 4096u32;
pub const HHWIN_PROP_TAB_HISTORY: u32 = 2048u32;
pub const HHWIN_PROP_TAB_SEARCH: u32 = 1024u32;
pub const HHWIN_PROP_TRACKING: u32 = 512u32;
pub const HHWIN_PROP_TRI_PANE: u32 = 32u32;
pub const HHWIN_PROP_USER_POS: u32 = 262144u32;
pub const HHWIN_TB_MARGIN: u32 = 268435456u32;
pub const HH_ALINK_LOOKUP: HTML_HELP_COMMAND = HTML_HELP_COMMAND(19i32);
pub const HH_CLOSE_ALL: HTML_HELP_COMMAND = HTML_HELP_COMMAND(18i32);
pub const HH_DISPLAY_INDEX: HTML_HELP_COMMAND = HTML_HELP_COMMAND(2i32);
pub const HH_DISPLAY_SEARCH: HTML_HELP_COMMAND = HTML_HELP_COMMAND(3i32);
pub const HH_DISPLAY_TEXT_POPUP: HTML_HELP_COMMAND = HTML_HELP_COMMAND(14i32);
pub const HH_DISPLAY_TOC: HTML_HELP_COMMAND = HTML_HELP_COMMAND(1i32);
pub const HH_DISPLAY_TOPIC: HTML_HELP_COMMAND = HTML_HELP_COMMAND(0i32);
pub const HH_ENUM_CATEGORY: HTML_HELP_COMMAND = HTML_HELP_COMMAND(21i32);
pub const HH_ENUM_CATEGORY_IT: HTML_HELP_COMMAND = HTML_HELP_COMMAND(22i32);
pub const HH_ENUM_INFO_TYPE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(7i32);
pub const HH_FTS_DEFAULT_PROXIMITY: HTML_HELP_COMMAND = HTML_HELP_COMMAND(-1i32);
pub const HH_GET_LAST_ERROR: HTML_HELP_COMMAND = HTML_HELP_COMMAND(20i32);
pub const HH_GET_WIN_HANDLE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(6i32);
pub const HH_GET_WIN_TYPE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(5i32);
pub const HH_GPROPID_CONTENT_LANGUAGE: HH_GPROPID = HH_GPROPID(5i32);
pub const HH_GPROPID_CURRENT_SUBSET: HH_GPROPID = HH_GPROPID(4i32);
pub const HH_GPROPID_SINGLETHREAD: HH_GPROPID = HH_GPROPID(1i32);
pub const HH_GPROPID_TOOLBAR_MARGIN: HH_GPROPID = HH_GPROPID(2i32);
pub const HH_GPROPID_UI_LANGUAGE: HH_GPROPID = HH_GPROPID(3i32);
pub const HH_HELP_CONTEXT: HTML_HELP_COMMAND = HTML_HELP_COMMAND(15i32);
pub const HH_HELP_FINDER: HTML_HELP_COMMAND = HTML_HELP_COMMAND(0i32);
pub const HH_INITIALIZE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(28i32);
pub const HH_KEYWORD_LOOKUP: HTML_HELP_COMMAND = HTML_HELP_COMMAND(13i32);
pub const HH_MAX_TABS: HTML_HELP_COMMAND = HTML_HELP_COMMAND(19i32);
pub const HH_MAX_TABS_CUSTOM: HTML_HELP_COMMAND = HTML_HELP_COMMAND(9i32);
pub const HH_PRETRANSLATEMESSAGE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(253i32);
pub const HH_RESERVED1: HTML_HELP_COMMAND = HTML_HELP_COMMAND(10i32);
pub const HH_RESERVED2: HTML_HELP_COMMAND = HTML_HELP_COMMAND(11i32);
pub const HH_RESERVED3: HTML_HELP_COMMAND = HTML_HELP_COMMAND(12i32);
pub const HH_RESET_IT_FILTER: HTML_HELP_COMMAND = HTML_HELP_COMMAND(23i32);
pub const HH_SAFE_DISPLAY_TOPIC: HTML_HELP_COMMAND = HTML_HELP_COMMAND(32i32);
pub const HH_SET_EXCLUSIVE_FILTER: HTML_HELP_COMMAND = HTML_HELP_COMMAND(25i32);
pub const HH_SET_GLOBAL_PROPERTY: HTML_HELP_COMMAND = HTML_HELP_COMMAND(252i32);
pub const HH_SET_INCLUSIVE_FILTER: HTML_HELP_COMMAND = HTML_HELP_COMMAND(24i32);
pub const HH_SET_INFO_TYPE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(8i32);
pub const HH_SET_QUERYSERVICE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(30i32);
pub const HH_SET_WIN_TYPE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(4i32);
pub const HH_SYNC: HTML_HELP_COMMAND = HTML_HELP_COMMAND(9i32);
pub const HH_TAB_AUTHOR: i32 = 5i32;
pub const HH_TAB_CONTENTS: i32 = 0i32;
pub const HH_TAB_CUSTOM_FIRST: i32 = 11i32;
pub const HH_TAB_CUSTOM_LAST: i32 = 19i32;
pub const HH_TAB_FAVORITES: i32 = 3i32;
pub const HH_TAB_HISTORY: i32 = 4i32;
pub const HH_TAB_INDEX: i32 = 1i32;
pub const HH_TAB_SEARCH: i32 = 2i32;
pub const HH_TP_HELP_CONTEXTMENU: HTML_HELP_COMMAND = HTML_HELP_COMMAND(16i32);
pub const HH_TP_HELP_WM_HELP: HTML_HELP_COMMAND = HTML_HELP_COMMAND(17i32);
pub const HH_UNINITIALIZE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(29i32);
pub const IDTB_BACK: u32 = 204u32;
pub const IDTB_BROWSE_BACK: u32 = 212u32;
pub const IDTB_BROWSE_FWD: u32 = 211u32;
pub const IDTB_CONTENTS: u32 = 213u32;
pub const IDTB_CONTRACT: u32 = 201u32;
pub const IDTB_CUSTOMIZE: u32 = 221u32;
pub const IDTB_EXPAND: u32 = 200u32;
pub const IDTB_FAVORITES: u32 = 217u32;
pub const IDTB_FORWARD: u32 = 209u32;
pub const IDTB_HISTORY: u32 = 216u32;
pub const IDTB_HOME: u32 = 205u32;
pub const IDTB_INDEX: u32 = 214u32;
pub const IDTB_JUMP1: u32 = 218u32;
pub const IDTB_JUMP2: u32 = 219u32;
pub const IDTB_NOTES: u32 = 210u32;
pub const IDTB_OPTIONS: u32 = 208u32;
pub const IDTB_PRINT: u32 = 207u32;
pub const IDTB_REFRESH: u32 = 203u32;
pub const IDTB_SEARCH: u32 = 215u32;
pub const IDTB_STOP: u32 = 202u32;
pub const IDTB_SYNC: u32 = 206u32;
pub const IDTB_TOC_NEXT: u32 = 223u32;
pub const IDTB_TOC_PREV: u32 = 224u32;
pub const IDTB_ZOOM: u32 = 222u32;
pub const IITWBC_BREAK_ACCEPT_WILDCARDS: u32 = 1u32;
pub const IITWBC_BREAK_AND_STEM: u32 = 2u32;
pub const ITWW_CBKEY_MAX: u32 = 1024u32;
pub const ITWW_OPEN_NOCONNECT: u32 = 1u32;
pub const IT_EXCLUSIVE: i32 = 1i32;
pub const IT_HIDDEN: i32 = 2i32;
pub const IT_INCLUSIVE: i32 = 0i32;
pub const MAX_COLUMNS: u32 = 256u32;
pub const PRIORITY_HIGH: PRIORITY = PRIORITY(2i32);
pub const PRIORITY_LOW: PRIORITY = PRIORITY(0i32);
pub const PRIORITY_NORMAL: PRIORITY = PRIORITY(1i32);
pub const PROP_ADD: u32 = 0u32;
pub const PROP_DELETE: u32 = 1u32;
pub const PROP_UPDATE: u32 = 2u32;
pub const STDPROP_DISPLAYKEY: u32 = 101u32;
pub const STDPROP_INDEX_BREAK: u32 = 204u32;
pub const STDPROP_INDEX_DTYPE: u32 = 202u32;
pub const STDPROP_INDEX_LENGTH: u32 = 203u32;
pub const STDPROP_INDEX_TERM: u32 = 210u32;
pub const STDPROP_INDEX_TERM_RAW_LENGTH: u32 = 211u32;
pub const STDPROP_INDEX_TEXT: u32 = 200u32;
pub const STDPROP_INDEX_VFLD: u32 = 201u32;
pub const STDPROP_KEY: u32 = 4u32;
pub const STDPROP_SORTKEY: u32 = 100u32;
pub const STDPROP_SORTORDINAL: u32 = 102u32;
pub const STDPROP_TITLE: u32 = 2u32;
pub const STDPROP_UID: u32 = 1u32;
pub const STDPROP_USERDATA: u32 = 3u32;
pub const STDPROP_USERPROP_BASE: u32 = 65536u32;
pub const STDPROP_USERPROP_MAX: u32 = 2147483647u32;
pub const SZ_WWDEST_GLOBAL: windows_core::PCWSTR = windows_core::w!("GLOBAL");
pub const SZ_WWDEST_KEY: windows_core::PCWSTR = windows_core::w!("KEY");
pub const SZ_WWDEST_OCC: windows_core::PCWSTR = windows_core::w!("OCC");
pub const TYPE_POINTER: u32 = 1u32;
pub const TYPE_STRING: u32 = 2u32;
pub const TYPE_VALUE: u32 = 0u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HH_GPROPID(pub i32);
impl windows_core::TypeKind for HH_GPROPID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HH_GPROPID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HH_GPROPID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTML_HELP_COMMAND(pub i32);
impl windows_core::TypeKind for HTML_HELP_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTML_HELP_COMMAND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTML_HELP_COMMAND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRIORITY(pub i32);
impl windows_core::TypeKind for PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRIORITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COLUMNSTATUS {
    pub cPropCount: i32,
    pub cPropsLoaded: i32,
}
impl windows_core::TypeKind for COLUMNSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for COLUMNSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CProperty {
    pub dwPropID: u32,
    pub cbData: u32,
    pub dwType: u32,
    pub Anonymous: CProperty_0,
    pub fPersist: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CProperty {
    type TypeKind = windows_core::CopyType;
}
impl Default for CProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CProperty_0 {
    pub lpszwData: windows_core::PWSTR,
    pub lpvData: *mut core::ffi::c_void,
    pub dwValue: u32,
}
impl windows_core::TypeKind for CProperty_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CProperty_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HHNTRACK {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszCurUrl: windows_core::PCSTR,
    pub idAction: i32,
    pub phhWinType: *mut HH_WINTYPE,
}
#[cfg(feature = "Win32_UI_Controls")]
impl windows_core::TypeKind for HHNTRACK {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Controls")]
impl Default for HHNTRACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HHN_NOTIFY {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszUrl: windows_core::PCSTR,
}
#[cfg(feature = "Win32_UI_Controls")]
impl windows_core::TypeKind for HHN_NOTIFY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Controls")]
impl Default for HHN_NOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HH_AKLINK {
    pub cbStruct: i32,
    pub fReserved: super::super::Foundation::BOOL,
    pub pszKeywords: *mut i8,
    pub pszUrl: *mut i8,
    pub pszMsgText: *mut i8,
    pub pszMsgTitle: *mut i8,
    pub pszWindow: *mut i8,
    pub fIndexOnFail: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for HH_AKLINK {
    type TypeKind = windows_core::CopyType;
}
impl Default for HH_AKLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HH_ENUM_CAT {
    pub cbStruct: i32,
    pub pszCatName: windows_core::PCSTR,
    pub pszCatDescription: windows_core::PCSTR,
}
impl windows_core::TypeKind for HH_ENUM_CAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for HH_ENUM_CAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HH_ENUM_IT {
    pub cbStruct: i32,
    pub iType: i32,
    pub pszCatName: windows_core::PCSTR,
    pub pszITName: windows_core::PCSTR,
    pub pszITDescription: windows_core::PCSTR,
}
impl windows_core::TypeKind for HH_ENUM_IT {
    type TypeKind = windows_core::CopyType;
}
impl Default for HH_ENUM_IT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HH_FTS_QUERY {
    pub cbStruct: i32,
    pub fUniCodeStrings: super::super::Foundation::BOOL,
    pub pszSearchQuery: *mut i8,
    pub iProximity: i32,
    pub fStemmedSearch: super::super::Foundation::BOOL,
    pub fTitleOnly: super::super::Foundation::BOOL,
    pub fExecute: super::super::Foundation::BOOL,
    pub pszWindow: *mut i8,
}
impl windows_core::TypeKind for HH_FTS_QUERY {
    type TypeKind = windows_core::CopyType;
}
impl Default for HH_FTS_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct HH_GLOBAL_PROPERTY {
    pub id: HH_GPROPID,
    pub var: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
impl Clone for HH_GLOBAL_PROPERTY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for HH_GLOBAL_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl Default for HH_GLOBAL_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HH_POPUP {
    pub cbStruct: i32,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub idString: u32,
    pub pszText: *mut i8,
    pub pt: super::super::Foundation::POINT,
    pub clrForeground: super::super::Foundation::COLORREF,
    pub clrBackground: super::super::Foundation::COLORREF,
    pub rcMargins: super::super::Foundation::RECT,
    pub pszFont: *mut i8,
}
impl windows_core::TypeKind for HH_POPUP {
    type TypeKind = windows_core::CopyType;
}
impl Default for HH_POPUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HH_SET_INFOTYPE {
    pub cbStruct: i32,
    pub pszCatName: windows_core::PCSTR,
    pub pszInfoTypeName: windows_core::PCSTR,
}
impl windows_core::TypeKind for HH_SET_INFOTYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for HH_SET_INFOTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HH_WINTYPE {
    pub cbStruct: i32,
    pub fUniCodeStrings: super::super::Foundation::BOOL,
    pub pszType: *mut i8,
    pub fsValidMembers: u32,
    pub fsWinProperties: u32,
    pub pszCaption: *mut i8,
    pub dwStyles: u32,
    pub dwExStyles: u32,
    pub rcWindowPos: super::super::Foundation::RECT,
    pub nShowState: i32,
    pub hwndHelp: super::super::Foundation::HWND,
    pub hwndCaller: super::super::Foundation::HWND,
    pub paInfoTypes: *mut u32,
    pub hwndToolBar: super::super::Foundation::HWND,
    pub hwndNavigation: super::super::Foundation::HWND,
    pub hwndHTML: super::super::Foundation::HWND,
    pub iNavWidth: i32,
    pub rcHTML: super::super::Foundation::RECT,
    pub pszToc: *mut i8,
    pub pszIndex: *mut i8,
    pub pszFile: *mut i8,
    pub pszHome: *mut i8,
    pub fsToolBarFlags: u32,
    pub fNotExpanded: super::super::Foundation::BOOL,
    pub curNavType: i32,
    pub tabpos: i32,
    pub idNotify: i32,
    pub tabOrder: [u8; 20],
    pub cHistory: i32,
    pub pszJump1: *mut i8,
    pub pszJump2: *mut i8,
    pub pszUrlJump1: *mut i8,
    pub pszUrlJump2: *mut i8,
    pub rcMinSize: super::super::Foundation::RECT,
    pub cbInfoTypes: i32,
    pub pszCustomTabs: *mut i8,
}
impl windows_core::TypeKind for HH_WINTYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for HH_WINTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ROWSTATUS {
    pub lRowFirst: i32,
    pub cRows: i32,
    pub cProperties: i32,
    pub cRowsTotal: i32,
}
impl windows_core::TypeKind for ROWSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ROWSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFNCOLHEAPFREE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> i32>;
#[cfg(feature = "implement")]
core::include!("impl.rs");

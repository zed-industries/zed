#[inline]
pub unsafe fn CloseIMsgSession<P0>(lpmsgsess: P0)
where
    P0: windows_core::Param<LPMSGSESS>,
{
    windows_targets::link!("mapi32.dll" "system" fn CloseIMsgSession(lpmsgsess : LPMSGSESS));
    CloseIMsgSession(lpmsgsess.param().abi())
}
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn GetAttribIMsgOnIStg(lpobject: *mut core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> windows_core::Result<()> {
    windows_targets::link!("mapi32.dll" "system" fn GetAttribIMsgOnIStg(lpobject : *mut core::ffi::c_void, lpproptagarray : *mut super::super::System::AddressBook:: SPropTagArray, lpppropattrarray : *mut *mut SPropAttrArray) -> windows_core::HRESULT);
    GetAttribIMsgOnIStg(lpobject, lpproptagarray, lpppropattrarray).ok()
}
#[inline]
pub unsafe fn MapStorageSCode(stgscode: i32) -> i32 {
    windows_targets::link!("mapi32.dll" "system" fn MapStorageSCode(stgscode : i32) -> i32);
    MapStorageSCode(stgscode)
}
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OpenIMsgOnIStg<P0, P1, P2>(lpmsgsess: P0, lpallocatebuffer: super::super::System::AddressBook::LPALLOCATEBUFFER, lpallocatemore: super::super::System::AddressBook::LPALLOCATEMORE, lpfreebuffer: super::super::System::AddressBook::LPFREEBUFFER, lpmalloc: P1, lpmapisup: *mut core::ffi::c_void, lpstg: P2, lpfmsgcallrelease: *mut MSGCALLRELEASE, ulcallerdata: u32, ulflags: u32, lppmsg: *mut Option<super::super::System::AddressBook::IMessage>) -> i32
where
    P0: windows_core::Param<LPMSGSESS>,
    P1: windows_core::Param<super::super::System::Com::IMalloc>,
    P2: windows_core::Param<super::super::System::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("mapi32.dll" "system" fn OpenIMsgOnIStg(lpmsgsess : LPMSGSESS, lpallocatebuffer : super::super::System::AddressBook:: LPALLOCATEBUFFER, lpallocatemore : super::super::System::AddressBook:: LPALLOCATEMORE, lpfreebuffer : super::super::System::AddressBook:: LPFREEBUFFER, lpmalloc : * mut core::ffi::c_void, lpmapisup : *mut core::ffi::c_void, lpstg : * mut core::ffi::c_void, lpfmsgcallrelease : *mut MSGCALLRELEASE, ulcallerdata : u32, ulflags : u32, lppmsg : *mut * mut core::ffi::c_void) -> i32);
    OpenIMsgOnIStg(lpmsgsess.param().abi(), lpallocatebuffer, lpallocatemore, lpfreebuffer, lpmalloc.param().abi(), lpmapisup, lpstg.param().abi(), lpfmsgcallrelease, ulcallerdata, ulflags, core::mem::transmute(lppmsg))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenIMsgSession<P0>(lpmalloc: P0, ulflags: u32, lppmsgsess: *mut LPMSGSESS) -> i32
where
    P0: windows_core::Param<super::super::System::Com::IMalloc>,
{
    windows_targets::link!("mapi32.dll" "system" fn OpenIMsgSession(lpmalloc : * mut core::ffi::c_void, ulflags : u32, lppmsgsess : *mut LPMSGSESS) -> i32);
    OpenIMsgSession(lpmalloc.param().abi(), ulflags, lppmsgsess)
}
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn SetAttribIMsgOnIStg(lpobject: *mut core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> windows_core::Result<()> {
    windows_targets::link!("mapi32.dll" "system" fn SetAttribIMsgOnIStg(lpobject : *mut core::ffi::c_void, lpproptags : *mut super::super::System::AddressBook:: SPropTagArray, lppropattrs : *mut SPropAttrArray, lpppropproblems : *mut *mut super::super::System::AddressBook:: SPropProblemArray) -> windows_core::HRESULT);
    SetAttribIMsgOnIStg(lpobject, lpproptags, lppropattrs, lpppropproblems).ok()
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscFormat2DataEvents, DDiscFormat2DataEvents_Vtbl, 0x2735413c_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscFormat2DataEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscFormat2DataEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2DataEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscFormat2DataEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscFormat2EraseEvents, DDiscFormat2EraseEvents_Vtbl, 0x2735413a_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscFormat2EraseEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscFormat2EraseEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2EraseEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0>(&self, object: P0, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), elapsedseconds, estimatedtotalseconds).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscFormat2EraseEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscFormat2RawCDEvents, DDiscFormat2RawCDEvents_Vtbl, 0x27354142_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscFormat2RawCDEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscFormat2RawCDEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2RawCDEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscFormat2RawCDEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscFormat2TrackAtOnceEvents, DDiscFormat2TrackAtOnceEvents_Vtbl, 0x2735413f_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscFormat2TrackAtOnceEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscFormat2TrackAtOnceEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2TrackAtOnceEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscMaster2Events, DDiscMaster2Events_Vtbl, 0x27354131_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscMaster2Events {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscMaster2Events, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscMaster2Events {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NotifyDeviceAdded<P0, P1>(&self, object: P0, uniqueid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).NotifyDeviceAdded)(windows_core::Interface::as_raw(self), object.param().abi(), uniqueid.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NotifyDeviceRemoved<P0, P1>(&self, object: P0, uniqueid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).NotifyDeviceRemoved)(windows_core::Interface::as_raw(self), object.param().abi(), uniqueid.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscMaster2Events_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub NotifyDeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NotifyDeviceAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NotifyDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NotifyDeviceRemoved: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DFileSystemImageEvents, DFileSystemImageEvents_Vtbl, 0x2c941fdf_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DFileSystemImageEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DFileSystemImageEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, currentfile: P1, copiedsectors: i32, totalsectors: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), currentfile.param().abi(), copiedsectors, totalsectors).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DFileSystemImageEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DFileSystemImageImportEvents, DFileSystemImageImportEvents_Vtbl, 0xd25c30f9_4087_4366_9e24_e55be286424b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DFileSystemImageImportEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DFileSystemImageImportEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageImportEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateImport<P0, P1>(&self, object: P0, filesystem: FsiFileSystems, currentitem: P1, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).UpdateImport)(windows_core::Interface::as_raw(self), object.param().abi(), filesystem, currentitem.param().abi(), importeddirectoryitems, totaldirectoryitems, importedfileitems, totalfileitems).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DFileSystemImageImportEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateImport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsiFileSystems, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateImport: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DWriteEngine2Events, DWriteEngine2Events_Vtbl, 0x27354137_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DWriteEngine2Events {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DWriteEngine2Events, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DWriteEngine2Events {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DWriteEngine2Events_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBlockRange, IBlockRange_Vtbl, 0xb507ca25_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBlockRange {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBlockRange, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBlockRange {
    pub unsafe fn StartLba(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EndLba(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBlockRange_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EndLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBlockRangeList, IBlockRangeList_Vtbl, 0xb507ca26_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBlockRangeList {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBlockRangeList, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBlockRangeList {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockRanges(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BlockRanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBlockRangeList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BlockRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BlockRanges: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBootOptions, IBootOptions_Vtbl, 0x2c941fd4_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBootOptions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBootOptions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBootOptions {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImage(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BootImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Manufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Manufacturer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetManufacturer<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetManufacturer)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn PlatformId(&self) -> windows_core::Result<PlatformId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PlatformId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPlatformId(&self, newval: PlatformId) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPlatformId)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn Emulation(&self) -> windows_core::Result<EmulationType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Emulation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEmulation(&self, newval: EmulationType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEmulation)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ImageSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImageSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AssignBootImage<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AssignBootImage)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBootOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImage: usize,
    pub Manufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PlatformId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlatformId) -> windows_core::HRESULT,
    pub SetPlatformId: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformId) -> windows_core::HRESULT,
    pub Emulation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EmulationType) -> windows_core::HRESULT,
    pub SetEmulation: unsafe extern "system" fn(*mut core::ffi::c_void, EmulationType) -> windows_core::HRESULT,
    pub ImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AssignBootImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AssignBootImage: usize,
}
windows_core::imp::define_interface!(IBurnVerification, IBurnVerification_Vtbl, 0xd2ffd834_958b_426d_8470_2a13879c6a91);
impl core::ops::Deref for IBurnVerification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBurnVerification, windows_core::IUnknown);
impl IBurnVerification {
    pub unsafe fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBurnVerificationLevel)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BurnVerificationLevel(&self) -> windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BurnVerificationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBurnVerification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBurnVerificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT,
    pub BurnVerificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2, IDiscFormat2_Vtbl, 0x27354152_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRecorderSupported)(windows_core::Interface::as_raw(self), recorder.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCurrentMediaSupported)(windows_core::Interface::as_raw(self), recorder.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn MediaPhysicallyBlank(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaPhysicallyBlank)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MediaHeuristicallyBlank(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaHeuristicallyBlank)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedMediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub IsRecorderSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsRecorderSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IsCurrentMediaSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsCurrentMediaSupported: usize,
    pub MediaPhysicallyBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MediaHeuristicallyBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedMediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedMediaTypes: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2Data, IDiscFormat2Data_Vtbl, 0x27354153_9f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2Data {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2Data, windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Data {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPostgapAlreadyInImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPostgapAlreadyInImage)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn PostgapAlreadyInImage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PostgapAlreadyInImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentMediaStatus(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentMediaStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WriteProtectStatus(&self) -> windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteProtectStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NextWritableAddress(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextWritableAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetForceMediaToBeClosed<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetForceMediaToBeClosed)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ForceMediaToBeClosed(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ForceMediaToBeClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisableConsumerDvdCompatibilityMode<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDisableConsumerDvdCompatibilityMode)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn DisableConsumerDvdCompatibilityMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisableConsumerDvdCompatibilityMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClientName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetForceOverwrite<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetForceOverwrite)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ForceOverwrite(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ForceOverwrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MultisessionInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), data.param().abi()).ok()
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWriteSpeed<P0>(&self, requestedsectorspersecond: i32, rotationtypeispurecav: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2Data_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPostgapAlreadyInImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PostgapAlreadyInImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentMediaStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> windows_core::HRESULT,
    pub WriteProtectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetForceMediaToBeClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ForceMediaToBeClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
    pub SetForceOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ForceOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MultisessionInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2DataEventArgs, IDiscFormat2DataEventArgs_Vtbl, 0x2735413d_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2DataEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2DataEventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2DataEventArgs {
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2DataEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2Erase, IDiscFormat2Erase_Vtbl, 0x27354156_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2Erase {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2Erase, windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Erase {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFullErase<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFullErase)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn FullErase(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FullErase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClientName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EraseMedia(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EraseMedia)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2Erase_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetFullErase: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FullErase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EraseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2RawCD, IDiscFormat2RawCD_Vtbl, 0x27354155_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2RawCD {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2RawCD, windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCD {
    pub unsafe fn PrepareMedia(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrepareMedia)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteMedia<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).WriteMedia)(windows_core::Interface::as_raw(self), data.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteMedia2<P0>(&self, data: P0, streamleadinsectors: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).WriteMedia2)(windows_core::Interface::as_raw(self), data.param().abi(), streamleadinsectors).ok()
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseMedia(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseMedia)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWriteSpeed<P0>(&self, requestedsectorspersecond: i32, rotationtypeispurecav: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StartOfNextSession(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartOfNextSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastPossibleStartOfLeadout(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastPossibleStartOfLeadout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedSectorTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedSectorTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRequestedSectorType)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn RequestedSectorType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestedSectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClientName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2RawCD_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteMedia2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteMedia2: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub StartOfNextSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastPossibleStartOfLeadout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedSectorTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedSectorTypes: usize,
    pub SetRequestedSectorType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub RequestedSectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2RawCDEventArgs, IDiscFormat2RawCDEventArgs_Vtbl, 0x27354143_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2RawCDEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2RawCDEventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCDEventArgs {
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2RawCDEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2TrackAtOnce, IDiscFormat2TrackAtOnce_Vtbl, 0x27354154_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2TrackAtOnce {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnce, windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnce {
    pub unsafe fn PrepareMedia(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrepareMedia)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddAudioTrack<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddAudioTrack)(windows_core::Interface::as_raw(self), data.param().abi()).ok()
    }
    pub unsafe fn CancelAddTrack(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelAddTrack)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseMedia(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseMedia)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWriteSpeed<P0>(&self, requestedsectorspersecond: i32, rotationtypeispurecav: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfExistingTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UsedSectorsOnMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UsedSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDoNotFinalizeMedia<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDoNotFinalizeMedia)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn DoNotFinalizeMedia(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoNotFinalizeMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExpectedTableOfContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClientName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2TrackAtOnce_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAudioTrack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAudioTrack: usize,
    pub CancelAddTrack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub NumberOfExistingTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UsedSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDoNotFinalizeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DoNotFinalizeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpectedTableOfContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpectedTableOfContents: usize,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2TrackAtOnceEventArgs, IDiscFormat2TrackAtOnceEventArgs_Vtbl, 0x27354140_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2TrackAtOnceEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnceEventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnceEventArgs {
    pub unsafe fn CurrentTrackNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentTrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiscMaster, IDiscMaster_Vtbl, 0x520cca62_51a5_11d3_9144_00104ba11c5e);
impl core::ops::Deref for IDiscMaster {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiscMaster, windows_core::IUnknown);
impl IDiscMaster {
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumDiscMasterFormats(&self) -> windows_core::Result<IEnumDiscMasterFormats> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDiscMasterFormats)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetActiveDiscMasterFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveDiscMasterFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetActiveDiscMasterFormat(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActiveDiscMasterFormat)(windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn EnumDiscRecorders(&self) -> windows_core::Result<IEnumDiscRecorders> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDiscRecorders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetActiveDiscRecorder(&self) -> windows_core::Result<IDiscRecorder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveDiscRecorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetActiveDiscRecorder<P0>(&self, precorder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder>,
    {
        (windows_core::Interface::vtable(self).SetActiveDiscRecorder)(windows_core::Interface::as_raw(self), precorder.param().abi()).ok()
    }
    pub unsafe fn ClearFormatContent(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearFormatContent)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProgressAdvise<P0>(&self, pevents: P0) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<IDiscMasterProgressEvents>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProgressAdvise)(windows_core::Interface::as_raw(self), pevents.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ProgressUnadvise(&self, vcookie: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProgressUnadvise)(windows_core::Interface::as_raw(self), vcookie).ok()
    }
    pub unsafe fn RecordDisc(&self, bsimulate: u8, bejectafterburn: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecordDisc)(windows_core::Interface::as_raw(self), bsimulate, bejectafterburn).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDiscMaster_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDiscMasterFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveDiscMasterFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetActiveDiscMasterFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDiscRecorders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActiveDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearFormatContent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProgressAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub ProgressUnadvise: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub RecordDisc: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscMaster2, IDiscMaster2_Vtbl, 0x27354130_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscMaster2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscMaster2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDiscMaster2 {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSupportedEnvironment(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSupportedEnvironment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscMaster2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsSupportedEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiscMasterProgressEvents, IDiscMasterProgressEvents_Vtbl, 0xec9e51c1_4e5d_11d3_9144_00104ba11c5e);
impl core::ops::Deref for IDiscMasterProgressEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiscMasterProgressEvents, windows_core::IUnknown);
impl IDiscMasterProgressEvents {
    pub unsafe fn QueryCancel(&self) -> windows_core::Result<u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryCancel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NotifyPnPActivity(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyPnPActivity)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotifyAddProgress(&self, ncompletedsteps: i32, ntotalsteps: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyAddProgress)(windows_core::Interface::as_raw(self), ncompletedsteps, ntotalsteps).ok()
    }
    pub unsafe fn NotifyBlockProgress(&self, ncompleted: i32, ntotal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyBlockProgress)(windows_core::Interface::as_raw(self), ncompleted, ntotal).ok()
    }
    pub unsafe fn NotifyTrackProgress(&self, ncurrenttrack: i32, ntotaltracks: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyTrackProgress)(windows_core::Interface::as_raw(self), ncurrenttrack, ntotaltracks).ok()
    }
    pub unsafe fn NotifyPreparingBurn(&self, nestimatedseconds: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyPreparingBurn)(windows_core::Interface::as_raw(self), nestimatedseconds).ok()
    }
    pub unsafe fn NotifyClosingDisc(&self, nestimatedseconds: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyClosingDisc)(windows_core::Interface::as_raw(self), nestimatedseconds).ok()
    }
    pub unsafe fn NotifyBurnComplete(&self, status: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyBurnComplete)(windows_core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn NotifyEraseComplete(&self, status: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyEraseComplete)(windows_core::Interface::as_raw(self), status).ok()
    }
}
#[repr(C)]
pub struct IDiscMasterProgressEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryCancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub NotifyPnPActivity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyAddProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub NotifyBlockProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub NotifyTrackProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub NotifyPreparingBurn: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NotifyClosingDisc: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NotifyBurnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub NotifyEraseComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiscRecorder, IDiscRecorder_Vtbl, 0x85ac9776_ca88_4cf2_894e_09598c078a41);
impl core::ops::Deref for IDiscRecorder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiscRecorder, windows_core::IUnknown);
impl IDiscRecorder {
    pub unsafe fn Init(&self, pbyuniqueid: &[u8], nuldrivenumber: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), core::mem::transmute(pbyuniqueid.as_ptr()), pbyuniqueid.len().try_into().unwrap(), nuldrivenumber).ok()
    }
    pub unsafe fn GetRecorderGUID(&self, pbyuniqueid: Option<&mut [u8]>, pulreturnsizerequired: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRecorderGUID)(windows_core::Interface::as_raw(self), core::mem::transmute(pbyuniqueid.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbyuniqueid.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pulreturnsizerequired).ok()
    }
    pub unsafe fn GetRecorderType(&self) -> windows_core::Result<RECORDER_TYPES> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecorderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDisplayNames(&self, pbstrvendorid: Option<*mut windows_core::BSTR>, pbstrproductid: Option<*mut windows_core::BSTR>, pbstrrevision: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayNames)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrvendorid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pbstrproductid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pbstrrevision.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetBasePnPID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBasePnPID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetRecorderProperties(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecorderProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetRecorderProperties<P0>(&self, ppropstg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::StructuredStorage::IPropertyStorage>,
    {
        (windows_core::Interface::vtable(self).SetRecorderProperties)(windows_core::Interface::as_raw(self), ppropstg.param().abi()).ok()
    }
    pub unsafe fn GetRecorderState(&self) -> windows_core::Result<DISC_RECORDER_STATE_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecorderState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OpenExclusive(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenExclusive)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryMediaType(&self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryMediaType)(windows_core::Interface::as_raw(self), fmediatype, fmediaflags).ok()
    }
    pub unsafe fn QueryMediaInfo(&self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryMediaInfo)(windows_core::Interface::as_raw(self), pbsessions, pblasttrack, ulstartaddress, ulnextwritable, ulfreeblocks).ok()
    }
    pub unsafe fn Eject(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Eject)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Erase(&self, bfullerase: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Erase)(windows_core::Interface::as_raw(self), bfullerase).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDiscRecorder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32) -> windows_core::HRESULT,
    pub GetRecorderGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetRecorderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RECORDER_TYPES) -> windows_core::HRESULT,
    pub GetDisplayNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetBasePnPID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetRecorderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetRecorderProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetRecorderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetRecorderProperties: usize,
    pub GetRecorderState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DISC_RECORDER_STATE_FLAGS) -> windows_core::HRESULT,
    pub OpenExclusive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MEDIA_TYPES, *mut MEDIA_FLAGS) -> windows_core::HRESULT,
    pub QueryMediaInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u8, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Eject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Erase: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscRecorder2, IDiscRecorder2_Vtbl, 0x27354133_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscRecorder2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscRecorder2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDiscRecorder2 {
    pub unsafe fn EjectMedia(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EjectMedia)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseTray(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseTray)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AcquireExclusiveAccess<P0, P1>(&self, force: P0, __midl__idiscrecorder20000: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AcquireExclusiveAccess)(windows_core::Interface::as_raw(self), force.param().abi(), __midl__idiscrecorder20000.param().abi()).ok()
    }
    pub unsafe fn ReleaseExclusiveAccess(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseExclusiveAccess)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableMcn(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableMcn)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableMcn(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableMcn)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InitializeDiscRecorder<P0>(&self, recorderuniqueid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDiscRecorder)(windows_core::Interface::as_raw(self), recorderuniqueid.param().abi()).ok()
    }
    pub unsafe fn ActiveDiscRecorder(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActiveDiscRecorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VendorId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VendorId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProductId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProductId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProductRevision(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProductRevision)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn VolumePathNames(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumePathNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceCanLoadMedia(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceCanLoadMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LegacyDeviceNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LegacyDeviceNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedFeaturePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedFeaturePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentFeaturePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentFeaturePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedProfiles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedProfiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentProfiles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentProfiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedModePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedModePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ExclusiveAccessOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExclusiveAccessOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscRecorder2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EjectMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseTray: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcquireExclusiveAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReleaseExclusiveAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableMcn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableMcn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ActiveDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProductRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub VolumePathNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VolumePathNames: usize,
    pub DeviceCanLoadMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LegacyDeviceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedFeaturePages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentFeaturePages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedProfiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentProfiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedModePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedModePages: usize,
    pub ExclusiveAccessOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiscRecorder2Ex, IDiscRecorder2Ex_Vtbl, 0x27354132_7f64_5b0f_8f00_5d77afbe261e);
impl core::ops::Deref for IDiscRecorder2Ex {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiscRecorder2Ex, windows_core::IUnknown);
impl IDiscRecorder2Ex {
    pub unsafe fn SendCommandNoData(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendCommandNoData)(windows_core::Interface::as_raw(self), core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), core::mem::transmute(sensebuffer.as_ptr()), timeout).ok()
    }
    pub unsafe fn SendCommandSendDataToDevice(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32, buffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendCommandSendDataToDevice)(windows_core::Interface::as_raw(self), core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), core::mem::transmute(sensebuffer.as_ptr()), timeout, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SendCommandGetDataFromDevice(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32, buffer: &mut [u8], bufferfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendCommandGetDataFromDevice)(windows_core::Interface::as_raw(self), core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), core::mem::transmute(sensebuffer.as_ptr()), timeout, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), bufferfetched).ok()
    }
    pub unsafe fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadDvdStructure)(windows_core::Interface::as_raw(self), format, address, layer, agid, data, count).ok()
    }
    pub unsafe fn SendDvdStructure(&self, format: u32, data: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendDvdStructure)(windows_core::Interface::as_raw(self), format, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAdapterDescriptor)(windows_core::Interface::as_raw(self), data, bytesize).ok()
    }
    pub unsafe fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceDescriptor)(windows_core::Interface::as_raw(self), data, bytesize).ok()
    }
    pub unsafe fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDiscInformation)(windows_core::Interface::as_raw(self), discinformation, bytesize).ok()
    }
    pub unsafe fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTrackInformation)(windows_core::Interface::as_raw(self), address, addresstype, trackinformation, bytesize).ok()
    }
    pub unsafe fn GetFeaturePage<P0>(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: P0, featuredata: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
    {
        (windows_core::Interface::vtable(self).GetFeaturePage)(windows_core::Interface::as_raw(self), requestedfeature, currentfeatureonly.param().abi(), featuredata, bytesize).ok()
    }
    pub unsafe fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetModePage)(windows_core::Interface::as_raw(self), requestedmodepage, requesttype, modepagedata, bytesize).ok()
    }
    pub unsafe fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetModePage)(windows_core::Interface::as_raw(self), requesttype, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetSupportedFeaturePages<P0>(&self, currentfeatureonly: P0, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
    {
        (windows_core::Interface::vtable(self).GetSupportedFeaturePages)(windows_core::Interface::as_raw(self), currentfeatureonly.param().abi(), featuredata, bytesize).ok()
    }
    pub unsafe fn GetSupportedProfiles<P0>(&self, currentonly: P0, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
    {
        (windows_core::Interface::vtable(self).GetSupportedProfiles)(windows_core::Interface::as_raw(self), currentonly.param().abi(), profiletypes, validprofiles).ok()
    }
    pub unsafe fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSupportedModePages)(windows_core::Interface::as_raw(self), requesttype, modepagetypes, validpages).ok()
    }
    pub unsafe fn GetByteAlignmentMask(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetByteAlignmentMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaximumNonPageAlignedTransferSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaximumNonPageAlignedTransferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaximumPageAlignedTransferSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaximumPageAlignedTransferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDiscRecorder2Ex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendCommandNoData: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32) -> windows_core::HRESULT,
    pub SendCommandSendDataToDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32, *const u8, u32) -> windows_core::HRESULT,
    pub SendCommandGetDataFromDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub ReadDvdStructure: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SendDvdStructure: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32) -> windows_core::HRESULT,
    pub GetAdapterDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetDiscInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetTrackInformation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, IMAPI_READ_TRACK_ADDRESS_TYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetFeaturePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FEATURE_PAGE_TYPE, super::super::Foundation::BOOLEAN, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetModePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_TYPE, IMAPI_MODE_PAGE_REQUEST_TYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetModePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_REQUEST_TYPE, *const u8, u32) -> windows_core::HRESULT,
    pub GetSupportedFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOLEAN, *mut *mut IMAPI_FEATURE_PAGE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOLEAN, *mut *mut IMAPI_PROFILE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedModePages: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_REQUEST_TYPE, *mut *mut IMAPI_MODE_PAGE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub GetByteAlignmentMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaximumNonPageAlignedTransferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaximumPageAlignedTransferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumDiscMasterFormats, IEnumDiscMasterFormats_Vtbl, 0xddf445e1_54ba_11d3_9144_00104ba11c5e);
impl core::ops::Deref for IEnumDiscMasterFormats {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumDiscMasterFormats, windows_core::IUnknown);
impl IEnumDiscMasterFormats {
    pub unsafe fn Next(&self, lpiidformatid: &mut [windows_core::GUID], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lpiidformatid.len().try_into().unwrap(), core::mem::transmute(lpiidformatid.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cformats: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cformats).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDiscMasterFormats> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumDiscMasterFormats_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumDiscRecorders, IEnumDiscRecorders_Vtbl, 0x9b1921e1_54ac_11d3_9144_00104ba11c5e);
impl core::ops::Deref for IEnumDiscRecorders {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumDiscRecorders, windows_core::IUnknown);
impl IEnumDiscRecorders {
    pub unsafe fn Next(&self, pprecorder: &mut [Option<IDiscRecorder>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pprecorder.len().try_into().unwrap(), core::mem::transmute(pprecorder.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, crecorders: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), crecorders).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDiscRecorders> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumDiscRecorders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumFsiItems, IEnumFsiItems_Vtbl, 0x2c941fda_975b_59be_a960_9a2a262853a5);
impl core::ops::Deref for IEnumFsiItems {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumFsiItems, windows_core::IUnknown);
impl IEnumFsiItems {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [Option<IFsiItem>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumFsiItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumFsiItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumProgressItems, IEnumProgressItems_Vtbl, 0x2c941fd6_975b_59be_a960_9a2a262853a5);
impl core::ops::Deref for IEnumProgressItems {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumProgressItems, windows_core::IUnknown);
impl IEnumProgressItems {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [Option<IProgressItem>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumProgressItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumProgressItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImage, IFileSystemImage_Vtbl, 0x2c941fe1_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImage {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImage, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Root(&self) -> windows_core::Result<IFsiDirectoryItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Root)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SessionStartBlock(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionStartBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSessionStartBlock)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FreeMediaBlocks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FreeMediaBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFreeMediaBlocks)(windows_core::Interface::as_raw(self), newval).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<P0>(&self, discrecorder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        (windows_core::Interface::vtable(self).SetMaxMediaBlocksFromDevice)(windows_core::Interface::as_raw(self), discrecorder.param().abi()).ok()
    }
    pub unsafe fn UsedBlocks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UsedBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetVolumeName<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetVolumeName)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn ImportedVolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportedVolumeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptions(&self) -> windows_core::Result<IBootOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BootImageOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptions<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBootOptions>,
    {
        (windows_core::Interface::vtable(self).SetBootImageOptions)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn FileCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DirectoryCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DirectoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetWorkingDirectory<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn ChangePoint(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ChangePoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StrictFileSystemCompliance(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrictFileSystemCompliance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrictFileSystemCompliance<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStrictFileSystemCompliance)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn UseRestrictedCharacterSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UseRestrictedCharacterSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUseRestrictedCharacterSet<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseRestrictedCharacterSet)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn FileSystemsToCreate(&self) -> windows_core::Result<FsiFileSystems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileSystemsToCreate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileSystemsToCreate)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsSupported(&self) -> windows_core::Result<FsiFileSystems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileSystemsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUDFRevision)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UDFRevision(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UDFRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UDFRevisionsSupported(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UDFRevisionsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChooseImageDefaults<P0>(&self, discrecorder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        (windows_core::Interface::vtable(self).ChooseImageDefaults)(windows_core::Interface::as_raw(self), discrecorder.param().abi()).ok()
    }
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChooseImageDefaultsForMediaType)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetISO9660InterchangeLevel)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ISO9660InterchangeLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ISO9660InterchangeLevelsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> windows_core::Result<IFileSystemImageResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateResultImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Exists<P0>(&self, fullpath: P0) -> windows_core::Result<FsiItemType>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Exists)(windows_core::Interface::as_raw(self), fullpath.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CalculateDiscIdentifier(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CalculateDiscIdentifier)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdentifyFileSystemsOnDisc<P0>(&self, discrecorder: P0) -> windows_core::Result<FsiFileSystems>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IdentifyFileSystemsOnDisc)(windows_core::Interface::as_raw(self), discrecorder.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> windows_core::Result<FsiFileSystems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultFileSystemForImport)(windows_core::Interface::as_raw(self), filesystems, &mut result__).map(|| result__)
    }
    pub unsafe fn ImportFileSystem(&self) -> windows_core::Result<FsiFileSystems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportFileSystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ImportSpecificFileSystem)(windows_core::Interface::as_raw(self), filesystemtouse).ok()
    }
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RollbackToChangePoint)(windows_core::Interface::as_raw(self), changepoint).ok()
    }
    pub unsafe fn LockInChangePoint(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockInChangePoint)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDirectoryItem<P0>(&self, name: P0) -> windows_core::Result<IFsiDirectoryItem>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDirectoryItem)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileItem<P0>(&self, name: P0) -> windows_core::Result<IFsiFileItem>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFileItem)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VolumeNameUDF(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeNameUDF)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VolumeNameJoliet(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeNameJoliet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VolumeNameISO9660(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeNameISO9660)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StageFiles(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StageFiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStageFiles<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStageFiles)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MultisessionInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMultisessionInterfaces)(windows_core::Interface::as_raw(self), newval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Root: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Root: usize,
    pub SessionStartBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSessionStartBlock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FreeMediaBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFreeMediaBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMaxMediaBlocksFromDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMaxMediaBlocksFromDevice: usize,
    pub UsedBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ImportedVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImageOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImageOptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBootImageOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBootImageOptions: usize,
    pub FileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DirectoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StrictFileSystemCompliance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStrictFileSystemCompliance: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseRestrictedCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseRestrictedCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FileSystemsToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub SetFileSystemsToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems) -> windows_core::HRESULT,
    pub FileSystemsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub SetUDFRevision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UDFRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UDFRevisionsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UDFRevisionsSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ChooseImageDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ChooseImageDefaults: usize,
    pub ChooseImageDefaultsForMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetISO9660InterchangeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ISO9660InterchangeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ISO9660InterchangeLevelsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ISO9660InterchangeLevelsSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultImage: usize,
    pub Exists: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut FsiItemType) -> windows_core::HRESULT,
    pub CalculateDiscIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IdentifyFileSystemsOnDisc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IdentifyFileSystemsOnDisc: usize,
    pub GetDefaultFileSystemForImport: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub ImportFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub ImportSpecificFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems) -> windows_core::HRESULT,
    pub RollbackToChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LockInChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDirectoryItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDirectoryItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileItem: usize,
    pub VolumeNameUDF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VolumeNameJoliet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VolumeNameISO9660: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MultisessionInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMultisessionInterfaces: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImage2, IFileSystemImage2_Vtbl, 0xd7644b2c_1537_4767_b62f_f1387b02ddfd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImage2 {
    type Target = IFileSystemImage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImage2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImage);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptionsArray(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BootImageOptionsArray)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBootImageOptionsArray)(windows_core::Interface::as_raw(self), newval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImage2_Vtbl {
    pub base__: IFileSystemImage_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImageOptionsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImageOptionsArray: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBootImageOptionsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBootImageOptionsArray: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImage3, IFileSystemImage3_Vtbl, 0x7cff842c_7e97_4807_8304_910dd8f7c051);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImage3 {
    type Target = IFileSystemImage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImage3, windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImage, IFileSystemImage2);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage3 {
    pub unsafe fn CreateRedundantUdfMetadataFiles(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRedundantUdfMetadataFiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCreateRedundantUdfMetadataFiles<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCreateRedundantUdfMetadataFiles)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProbeSpecificFileSystem)(windows_core::Interface::as_raw(self), filesystemtoprobe, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImage3_Vtbl {
    pub base__: IFileSystemImage2_Vtbl,
    pub CreateRedundantUdfMetadataFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetCreateRedundantUdfMetadataFiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ProbeSpecificFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImageResult, IFileSystemImageResult_Vtbl, 0x2c941fd8_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImageResult {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImageResult, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImageStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImageStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItems(&self) -> windows_core::Result<IProgressItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProgressItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TotalBlocks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BlockSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DiscId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DiscId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImageResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ImageStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImageStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItems: usize,
    pub TotalBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DiscId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImageResult2, IFileSystemImageResult2_Vtbl, 0xb507ca29_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImageResult2 {
    type Target = IFileSystemImageResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImageResult2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImageResult);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModifiedBlocks(&self) -> windows_core::Result<IBlockRangeList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModifiedBlocks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImageResult2_Vtbl {
    pub base__: IFileSystemImageResult_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifiedBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifiedBlocks: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiDirectoryItem, IFsiDirectoryItem_Vtbl, 0x2c941fdc_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiDirectoryItem {
    type Target = IFsiItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiDirectoryItem, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem);
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, path: P0) -> windows_core::Result<IFsiItem>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumFsiItems(&self) -> windows_core::Result<IEnumFsiItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFsiItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddDirectory<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddDirectory)(windows_core::Interface::as_raw(self), path.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddFile<P0, P1>(&self, path: P0, filedata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddFile)(windows_core::Interface::as_raw(self), path.param().abi(), filedata.param().abi()).ok()
    }
    pub unsafe fn AddTree<P0, P1>(&self, sourcedirectory: P0, includebasedirectory: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).AddTree)(windows_core::Interface::as_raw(self), sourcedirectory.param().abi(), includebasedirectory.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, item: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsiItem>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), item.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), path.param().abi()).ok()
    }
    pub unsafe fn RemoveTree<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveTree)(windows_core::Interface::as_raw(self), path.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiDirectoryItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumFsiItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddFile: usize,
    pub AddTree: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveTree: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiDirectoryItem2, IFsiDirectoryItem2_Vtbl, 0xf7fb4b9b_6d96_4d7b_9115_201b144811ef);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiDirectoryItem2 {
    type Target = IFsiDirectoryItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiDirectoryItem2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem, IFsiDirectoryItem);
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem2 {
    pub unsafe fn AddTreeWithNamedStreams<P0, P1>(&self, sourcedirectory: P0, includebasedirectory: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).AddTreeWithNamedStreams)(windows_core::Interface::as_raw(self), sourcedirectory.param().abi(), includebasedirectory.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiDirectoryItem2_Vtbl {
    pub base__: IFsiDirectoryItem_Vtbl,
    pub AddTreeWithNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiFileItem, IFsiFileItem_Vtbl, 0x2c941fdb_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiFileItem {
    type Target = IFsiItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiFileItem, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem);
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem {
    pub unsafe fn DataSize(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DataSize32BitLow(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataSize32BitLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DataSize32BitHigh(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataSize32BitHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Data(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetData<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiFileItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    pub DataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub DataSize32BitLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DataSize32BitHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Data: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetData: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiFileItem2, IFsiFileItem2_Vtbl, 0x199d0c19_11e1_40eb_8ec2_c8c822a07792);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiFileItem2 {
    type Target = IFsiFileItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiFileItem2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem, IFsiFileItem);
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FsiNamedStreams(&self) -> windows_core::Result<IFsiNamedStreams> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FsiNamedStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsNamedStream(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsNamedStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddStream<P0, P1>(&self, name: P0, streamdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), name.param().abi(), streamdata.param().abi()).ok()
    }
    pub unsafe fn RemoveStream<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRealTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsRealTime<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsRealTime)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiFileItem2_Vtbl {
    pub base__: IFsiFileItem_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FsiNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FsiNamedStreams: usize,
    pub IsNamedStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddStream: usize,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiItem, IFsiItem_Vtbl, 0x2c941fd9_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiItem {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiItem, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsiItem {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FullPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FullPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreationTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCreationTime)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastAccessedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLastAccessedTime)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastModifiedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLastModifiedTime)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn IsHidden(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsHidden<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsHidden)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileSystemName)(windows_core::Interface::as_raw(self), filesystem, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileSystemPath)(windows_core::Interface::as_raw(self), filesystem, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FullPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetCreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LastAccessedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLastAccessedTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FileSystemName: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FileSystemPath: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiNamedStreams, IFsiNamedStreams_Vtbl, 0xed79ba56_5294_4250_8d46_f9aecee23459);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiNamedStreams {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiNamedStreams, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsiNamedStreams {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IFsiFileItem2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumNamedStreams(&self) -> windows_core::Result<IEnumFsiItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumNamedStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiNamedStreams_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IIsoImageManager, IIsoImageManager_Vtbl, 0x6ca38be5_fbbb_4800_95a1_a438865eb0d4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IIsoImageManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IIsoImageManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IIsoImageManager {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Stream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPath<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetStream<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SetStream)(windows_core::Interface::as_raw(self), data.param().abi()).ok()
    }
    pub unsafe fn Validate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIsoImageManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetStream: usize,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IJolietDiscMaster, IJolietDiscMaster_Vtbl, 0xe3bc42ce_4e5c_11d3_9144_00104ba11c5e);
impl core::ops::Deref for IJolietDiscMaster {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IJolietDiscMaster, windows_core::IUnknown);
impl IJolietDiscMaster {
    pub unsafe fn GetTotalDataBlocks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTotalDataBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUsedDataBlocks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUsedDataBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataBlockSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataBlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn AddData<P0>(&self, pstorage: P0, lfileoverwrite: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::StructuredStorage::IStorage>,
    {
        (windows_core::Interface::vtable(self).AddData)(windows_core::Interface::as_raw(self), pstorage.param().abi(), lfileoverwrite).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetJolietProperties(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJolietProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetJolietProperties<P0>(&self, ppropstg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::StructuredStorage::IPropertyStorage>,
    {
        (windows_core::Interface::vtable(self).SetJolietProperties)(windows_core::Interface::as_raw(self), ppropstg.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IJolietDiscMaster_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTotalDataBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetUsedDataBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDataBlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub AddData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    AddData: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetJolietProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetJolietProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetJolietProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetJolietProperties: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMultisession, IMultisession_Vtbl, 0x27354150_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMultisession {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMultisession, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMultisession {
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSupportedOnCurrentMediaState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInUse<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetInUse)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn InUse(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InUse)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> windows_core::Result<IDiscRecorder2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportRecorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMultisession_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IsSupportedOnCurrentMediaState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetInUse: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub InUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportRecorder: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMultisessionRandomWrite, IMultisessionRandomWrite_Vtbl, 0xb507ca23_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMultisessionRandomWrite {
    type Target = IMultisession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMultisessionRandomWrite, windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionRandomWrite {
    pub unsafe fn WriteUnitSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteUnitSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastWrittenAddress(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastWrittenAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMultisessionRandomWrite_Vtbl {
    pub base__: IMultisession_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMultisessionSequential, IMultisessionSequential_Vtbl, 0x27354151_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMultisessionSequential {
    type Target = IMultisession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMultisessionSequential, windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential {
    pub unsafe fn IsFirstDataSession(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFirstDataSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NextWritableAddress(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextWritableAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMultisessionSequential_Vtbl {
    pub base__: IMultisession_Vtbl,
    pub IsFirstDataSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMultisessionSequential2, IMultisessionSequential2_Vtbl, 0xb507ca22_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMultisessionSequential2 {
    type Target = IMultisessionSequential;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMultisessionSequential2, windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession, IMultisessionSequential);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential2 {
    pub unsafe fn WriteUnitSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteUnitSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMultisessionSequential2_Vtbl {
    pub base__: IMultisessionSequential_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IProgressItem, IProgressItem_Vtbl, 0x2c941fd5_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IProgressItem {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IProgressItem, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IProgressItem {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FirstBlock(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirstBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastBlock(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BlockCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BlockCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IProgressItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FirstBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LastBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub BlockCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IProgressItems, IProgressItems_Vtbl, 0x2c941fd7_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IProgressItems {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IProgressItems, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IProgressItems {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IProgressItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItemFromBlock(&self, block: u32) -> windows_core::Result<IProgressItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProgressItemFromBlock)(windows_core::Interface::as_raw(self), block, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItemFromDescription<P0>(&self, description: P0) -> windows_core::Result<IProgressItem>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProgressItemFromDescription)(windows_core::Interface::as_raw(self), description.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumProgressItems(&self) -> windows_core::Result<IEnumProgressItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumProgressItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IProgressItems_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItemFromBlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItemFromBlock: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItemFromDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItemFromDescription: usize,
    pub EnumProgressItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRawCDImageCreator, IRawCDImageCreator_Vtbl, 0x25983550_9d65_49ce_b335_40630d901227);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRawCDImageCreator {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRawCDImageCreator, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageCreator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateResultImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddTrack<P0>(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddTrack)(windows_core::Interface::as_raw(self), datatype, data.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddSpecialPregap<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddSpecialPregap)(windows_core::Interface::as_raw(self), data.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddSubcodeRWGenerator<P0>(&self, subcode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddSubcodeRWGenerator)(windows_core::Interface::as_raw(self), subcode.param().abi()).ok()
    }
    pub unsafe fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResultingImageType)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ResultingImageType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultingImageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StartOfLeadout(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartOfLeadout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStartOfLeadoutLimit(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartOfLeadoutLimit)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartOfLeadoutLimit(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartOfLeadoutLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisableGaplessAudio<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDisableGaplessAudio)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn DisableGaplessAudio(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisableGaplessAudio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMediaCatalogNumber<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMediaCatalogNumber)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn MediaCatalogNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaCatalogNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStartingTrackNumber(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartingTrackNumber)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartingTrackNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartingTrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_TrackInfo(&self, trackindex: i32) -> windows_core::Result<IRawCDImageTrackInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_TrackInfo)(windows_core::Interface::as_raw(self), trackindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfExistingTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastUsedUserSectorInImage(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastUsedUserSectorInImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExpectedTableOfContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRawCDImageCreator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultImage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddTrack: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_CD_SECTOR_TYPE, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddTrack: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSpecialPregap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSpecialPregap: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSubcodeRWGenerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSubcodeRWGenerator: usize,
    pub SetResultingImageType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub ResultingImageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub StartOfLeadout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStartOfLeadoutLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartOfLeadoutLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDisableGaplessAudio: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisableGaplessAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMediaCatalogNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MediaCatalogNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetStartingTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartingTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_TrackInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_TrackInfo: usize,
    pub NumberOfExistingTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastUsedUserSectorInImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpectedTableOfContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpectedTableOfContents: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRawCDImageTrackInfo, IRawCDImageTrackInfo_Vtbl, 0x25983551_9d65_49ce_b335_40630d901227);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRawCDImageTrackInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRawCDImageTrackInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageTrackInfo {
    pub unsafe fn StartingLba(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartingLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SectorCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TrackNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SectorType(&self) -> windows_core::Result<IMAPI_CD_SECTOR_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ISRC(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ISRC)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetISRC<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetISRC)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn DigitalAudioCopySetting(&self) -> windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DigitalAudioCopySetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDigitalAudioCopySetting)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn AudioHasPreemphasis(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioHasPreemphasis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAudioHasPreemphasis<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAudioHasPreemphasis)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TrackIndexes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TrackIndexes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AddTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddTrackIndex)(windows_core::Interface::as_raw(self), lbaoffset).ok()
    }
    pub unsafe fn ClearTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearTrackIndex)(windows_core::Interface::as_raw(self), lbaoffset).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRawCDImageTrackInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartingLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_CD_SECTOR_TYPE) -> windows_core::HRESULT,
    pub ISRC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetISRC: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DigitalAudioCopySetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT,
    pub SetDigitalAudioCopySetting: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT,
    pub AudioHasPreemphasis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAudioHasPreemphasis: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TrackIndexes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TrackIndexes: usize,
    pub AddTrackIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClearTrackIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRedbookDiscMaster, IRedbookDiscMaster_Vtbl, 0xe3bc42cd_4e5c_11d3_9144_00104ba11c5e);
impl core::ops::Deref for IRedbookDiscMaster {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRedbookDiscMaster, windows_core::IUnknown);
impl IRedbookDiscMaster {
    pub unsafe fn GetTotalAudioTracks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTotalAudioTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTotalAudioBlocks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTotalAudioBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUsedAudioBlocks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUsedAudioBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAvailableAudioTrackBlocks(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAvailableAudioTrackBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAudioBlockSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAudioBlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateAudioTrack(&self, nblocks: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateAudioTrack)(windows_core::Interface::as_raw(self), nblocks).ok()
    }
    pub unsafe fn AddAudioTrackBlocks(&self, pby: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddAudioTrackBlocks)(windows_core::Interface::as_raw(self), core::mem::transmute(pby.as_ptr()), pby.len().try_into().unwrap()).ok()
    }
    pub unsafe fn CloseAudioTrack(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseAudioTrack)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRedbookDiscMaster_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTotalAudioTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetTotalAudioBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetUsedAudioBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetAvailableAudioTrackBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetAudioBlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CreateAudioTrack: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddAudioTrackBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32) -> windows_core::HRESULT,
    pub CloseAudioTrack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStreamConcatenate, IStreamConcatenate_Vtbl, 0x27354146_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStreamConcatenate {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStreamConcatenate, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl IStreamConcatenate {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1>(&self, stream1: P0, stream2: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), stream1.param().abi(), stream2.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize2(&self, streams: &[Option<super::super::System::Com::IStream>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize2)(windows_core::Interface::as_raw(self), core::mem::transmute(streams.as_ptr()), streams.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Append<P0>(&self, stream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), stream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Append2(&self, streams: &[Option<super::super::System::Com::IStream>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Append2)(windows_core::Interface::as_raw(self), core::mem::transmute(streams.as_ptr()), streams.len().try_into().unwrap()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStreamConcatenate_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize2: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Append2: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append2: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStreamInterleave, IStreamInterleave_Vtbl, 0x27354147_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStreamInterleave {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStreamInterleave, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl IStreamInterleave {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize(&self, streams: *const Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(streams), interleavesizes, streamcount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStreamInterleave_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStreamPseudoRandomBased, IStreamPseudoRandomBased_Vtbl, 0x27354145_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStreamPseudoRandomBased {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStreamPseudoRandomBased, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl IStreamPseudoRandomBased {
    pub unsafe fn SetSeed(&self, value: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSeed)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Seed(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Seed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn put_ExtendedSeed(&self, values: &[u32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_ExtendedSeed)(windows_core::Interface::as_raw(self), core::mem::transmute(values.as_ptr()), values.len().try_into().unwrap()).ok()
    }
    pub unsafe fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).get_ExtendedSeed)(windows_core::Interface::as_raw(self), values, ecount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStreamPseudoRandomBased_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    pub SetSeed: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Seed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_ExtendedSeed: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
    pub get_ExtendedSeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWriteEngine2, IWriteEngine2_Vtbl, 0x27354135_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWriteEngine2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWriteEngine2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteSection<P0>(&self, data: P0, startingblockaddress: i32, numberofblocks: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).WriteSection)(windows_core::Interface::as_raw(self), data.param().abi(), startingblockaddress, numberofblocks).ok()
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2Ex>,
    {
        (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2Ex> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetUseStreamingWrite12<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseStreamingWrite12)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn UseStreamingWrite12(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UseStreamingWrite12)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStartingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartingSectorsPerSecond)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartingSectorsPerSecond(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartingSectorsPerSecond)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEndingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEndingSectorsPerSecond)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn EndingSectorsPerSecond(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndingSectorsPerSecond)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBytesPerSector(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBytesPerSector)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BytesPerSector(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BytesPerSector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WriteInProgress(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteInProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWriteEngine2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteSection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteSection: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUseStreamingWrite12: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseStreamingWrite12: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStartingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEndingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EndingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBytesPerSector: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BytesPerSector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub WriteInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWriteEngine2EventArgs, IWriteEngine2EventArgs_Vtbl, 0x27354136_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWriteEngine2EventArgs {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWriteEngine2EventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2EventArgs {
    pub unsafe fn StartLba(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SectorCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastReadLba(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastReadLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastWrittenLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UsedSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FreeSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWriteEngine2EventArgs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastReadLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UsedSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWriteSpeedDescriptor, IWriteSpeedDescriptor_Vtbl, 0x27354144_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWriteSpeedDescriptor {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWriteSpeedDescriptor, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWriteSpeedDescriptor {
    pub unsafe fn MediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WriteSpeed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWriteSpeedDescriptor_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub RotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub WriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub const CATID_SMTP_DNSRESOLVERRECORDSINK: windows_core::GUID = windows_core::GUID::from_u128(0xbd0b4366_8e03_11d2_94f6_00c04f79f1d6);
pub const CATID_SMTP_DSN: windows_core::GUID = windows_core::GUID::from_u128(0x22b55731_f5f8_4d23_bd8f_87b52371a73a);
pub const CATID_SMTP_GET_AUX_DOMAIN_INFO_FLAGS: windows_core::GUID = windows_core::GUID::from_u128(0x84ff368a_fab3_43d7_bcdf_692c5b46e6b1);
pub const CATID_SMTP_LOG: windows_core::GUID = windows_core::GUID::from_u128(0x93d0a538_2c1e_4b68_a7c9_d73a8aa6ee97);
pub const CATID_SMTP_MAXMSGSIZE: windows_core::GUID = windows_core::GUID::from_u128(0xebf159de_a67e_11d2_94f7_00c04f79f1d6);
pub const CATID_SMTP_MSGTRACKLOG: windows_core::GUID = windows_core::GUID::from_u128(0xc6df52aa_7db0_11d2_94f4_00c04f79f1d6);
pub const CATID_SMTP_ON_BEFORE_DATA: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c92_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_INBOUND_COMMAND: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c8d_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_MESSAGE_START: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c90_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_PER_RECIPIENT: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c91_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SERVER_RESPONSE: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c8e_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SESSION_END: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c93_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SESSION_START: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c8f_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_STORE_DRIVER: windows_core::GUID = windows_core::GUID::from_u128(0x59175850_e533_11d1_aa67_00c04fa345f6);
pub const CATID_SMTP_TRANSPORT_CATEGORIZE: windows_core::GUID = windows_core::GUID::from_u128(0x960252a3_0a3a_11d2_9e00_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_POSTCATEGORIZE: windows_core::GUID = windows_core::GUID::from_u128(0x76719654_05a6_11d2_9dfd_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_PRECATEGORIZE: windows_core::GUID = windows_core::GUID::from_u128(0xa3acfb0d_83ff_11d2_9e14_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_ROUTER: windows_core::GUID = windows_core::GUID::from_u128(0x283430c9_1850_11d2_9e03_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_SUBMISSION: windows_core::GUID = windows_core::GUID::from_u128(0xff3caa23_00b9_11d2_9dfb_00c04fa322ba);
pub const CLSID_SmtpCat: windows_core::GUID = windows_core::GUID::from_u128(0xb23c35b7_9219_11d2_9e17_00c04fa322ba);
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256u32;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257u32;
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256u32;
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257u32;
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256u32;
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256u32;
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257u32;
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257u32;
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513u32;
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275u32;
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270u32;
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269u32;
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279u32;
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265u32;
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268u32;
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280u32;
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266u32;
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260u32;
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274u32;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273u32;
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514u32;
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267u32;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278u32;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277u32;
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264u32;
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512u32;
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263u32;
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259u32;
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513u32;
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257u32;
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258u32;
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515u32;
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269u32;
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260u32;
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512u32;
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267u32;
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517u32;
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271u32;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513u32;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772u32;
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513u32;
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514u32;
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273u32;
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263u32;
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266u32;
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515u32;
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261u32;
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259u32;
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512u32;
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272u32;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271u32;
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516u32;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276u32;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275u32;
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260u32;
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262u32;
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793u32;
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792u32;
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049u32;
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048u32;
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794u32;
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258u32;
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0u32;
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257u32;
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521u32;
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523u32;
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518u32;
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260u32;
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256u32;
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261u32;
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525u32;
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262u32;
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519u32;
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514u32;
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515u32;
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522u32;
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513u32;
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516u32;
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517u32;
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512u32;
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516u32;
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258u32;
pub const DISPID_IMULTISESSION_INUSE: u32 = 257u32;
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514u32;
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518u32;
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515u32;
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519u32;
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513u32;
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256u32;
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513u32;
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512u32;
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259u32;
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265u32;
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260u32;
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263u32;
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258u32;
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262u32;
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264u32;
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262u32;
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261u32;
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260u32;
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257u32;
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259u32;
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256u32;
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263u32;
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260u32;
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513u32;
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256u32;
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259u32;
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258u32;
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257u32;
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261u32;
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512u32;
pub const Emulation12MFloppy: EmulationType = EmulationType(1i32);
pub const Emulation144MFloppy: EmulationType = EmulationType(2i32);
pub const Emulation288MFloppy: EmulationType = EmulationType(3i32);
pub const EmulationHardDisk: EmulationType = EmulationType(4i32);
pub const EmulationNone: EmulationType = EmulationType(0i32);
pub const FsiFileSystemISO9660: FsiFileSystems = FsiFileSystems(1i32);
pub const FsiFileSystemJoliet: FsiFileSystems = FsiFileSystems(2i32);
pub const FsiFileSystemNone: FsiFileSystems = FsiFileSystems(0i32);
pub const FsiFileSystemUDF: FsiFileSystems = FsiFileSystems(4i32);
pub const FsiFileSystemUnknown: FsiFileSystems = FsiFileSystems(1073741824i32);
pub const FsiItemDirectory: FsiItemType = FsiItemType(1i32);
pub const FsiItemFile: FsiItemType = FsiItemType(2i32);
pub const FsiItemNotFound: FsiItemType = FsiItemType(0i32);
pub const GUID_SMTPSVC_SOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x1b3c0666_e470_11d1_aa67_00c04fa345f6);
pub const GUID_SMTP_SOURCE_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xfb65c4dc_e468_11d1_aa67_00c04fa345f6);
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32u32;
pub const IMAPI2FS_FullVersion_STR: windows_core::PCSTR = windows_core::s!("1.0");
pub const IMAPI2FS_FullVersion_WSTR: windows_core::PCWSTR = windows_core::w!("1.0");
pub const IMAPI2FS_MajorVersion: u32 = 1u32;
pub const IMAPI2FS_MinorVersion: u32 = 0u32;
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10u32;
pub const IMAPILib2_MajorVersion: u32 = 1u32;
pub const IMAPILib2_MinorVersion: u32 = 0u32;
pub const IMAPI_BURN_VERIFICATION_FULL: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(2i32);
pub const IMAPI_BURN_VERIFICATION_NONE: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(0i32);
pub const IMAPI_BURN_VERIFICATION_QUICK: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(1i32);
pub const IMAPI_CD_SECTOR_AUDIO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(0i32);
pub const IMAPI_CD_SECTOR_MODE1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(2i32);
pub const IMAPI_CD_SECTOR_MODE1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(6i32);
pub const IMAPI_CD_SECTOR_MODE2FORM0: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(3i32);
pub const IMAPI_CD_SECTOR_MODE2FORM0RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(7i32);
pub const IMAPI_CD_SECTOR_MODE2FORM1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(4i32);
pub const IMAPI_CD_SECTOR_MODE2FORM1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(8i32);
pub const IMAPI_CD_SECTOR_MODE2FORM2: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(5i32);
pub const IMAPI_CD_SECTOR_MODE2FORM2RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(9i32);
pub const IMAPI_CD_SECTOR_MODE_ZERO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(1i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(0i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(1i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_SCMS: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(2i32);
pub const IMAPI_E_ALREADYOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80040222_u32 as _);
pub const IMAPI_E_BADJOLIETNAME: windows_core::HRESULT = windows_core::HRESULT(0x8004021D_u32 as _);
pub const IMAPI_E_BOOTIMAGE_AND_NONBLANK_DISC: windows_core::HRESULT = windows_core::HRESULT(0x8004022E_u32 as _);
pub const IMAPI_E_CANNOT_WRITE_TO_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x8004022C_u32 as _);
pub const IMAPI_E_COMPRESSEDSTASH: windows_core::HRESULT = windows_core::HRESULT(0x80040228_u32 as _);
pub const IMAPI_E_DEVICE_INVALIDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040214_u32 as _);
pub const IMAPI_E_DEVICE_NOPROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0x80040211_u32 as _);
pub const IMAPI_E_DEVICE_NOTACCESSIBLE: windows_core::HRESULT = windows_core::HRESULT(0x80040212_u32 as _);
pub const IMAPI_E_DEVICE_NOTPRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80040213_u32 as _);
pub const IMAPI_E_DEVICE_STILL_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80040226_u32 as _);
pub const IMAPI_E_DISCFULL: windows_core::HRESULT = windows_core::HRESULT(0x8004021C_u32 as _);
pub const IMAPI_E_DISCINFO: windows_core::HRESULT = windows_core::HRESULT(0x80040219_u32 as _);
pub const IMAPI_E_ENCRYPTEDSTASH: windows_core::HRESULT = windows_core::HRESULT(0x80040229_u32 as _);
pub const IMAPI_E_FILEACCESS: windows_core::HRESULT = windows_core::HRESULT(0x80040218_u32 as _);
pub const IMAPI_E_FILEEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80040224_u32 as _);
pub const IMAPI_E_FILESYSTEM: windows_core::HRESULT = windows_core::HRESULT(0x80040217_u32 as _);
pub const IMAPI_E_GENERIC: windows_core::HRESULT = windows_core::HRESULT(0x8004020E_u32 as _);
pub const IMAPI_E_INITIALIZE_ENDWRITE: windows_core::HRESULT = windows_core::HRESULT(0x80040216_u32 as _);
pub const IMAPI_E_INITIALIZE_WRITE: windows_core::HRESULT = windows_core::HRESULT(0x80040215_u32 as _);
pub const IMAPI_E_INVALIDIMAGE: windows_core::HRESULT = windows_core::HRESULT(0x8004021E_u32 as _);
pub const IMAPI_E_LOSS_OF_STREAMING: windows_core::HRESULT = windows_core::HRESULT(0x80040227_u32 as _);
pub const IMAPI_E_MEDIUM_INVALIDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040210_u32 as _);
pub const IMAPI_E_MEDIUM_NOTPRESENT: windows_core::HRESULT = windows_core::HRESULT(0x8004020F_u32 as _);
pub const IMAPI_E_NOACTIVEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004021F_u32 as _);
pub const IMAPI_E_NOACTIVERECORDER: windows_core::HRESULT = windows_core::HRESULT(0x80040220_u32 as _);
pub const IMAPI_E_NOTENOUGHDISKFORSTASH: windows_core::HRESULT = windows_core::HRESULT(0x8004022A_u32 as _);
pub const IMAPI_E_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8004020C_u32 as _);
pub const IMAPI_E_NOTOPENED: windows_core::HRESULT = windows_core::HRESULT(0x8004020B_u32 as _);
pub const IMAPI_E_REMOVABLESTASH: windows_core::HRESULT = windows_core::HRESULT(0x8004022B_u32 as _);
pub const IMAPI_E_STASHINUSE: windows_core::HRESULT = windows_core::HRESULT(0x80040225_u32 as _);
pub const IMAPI_E_TRACKNOTOPEN: windows_core::HRESULT = windows_core::HRESULT(0x8004021A_u32 as _);
pub const IMAPI_E_TRACKOPEN: windows_core::HRESULT = windows_core::HRESULT(0x8004021B_u32 as _);
pub const IMAPI_E_TRACK_NOT_BIG_ENOUGH: windows_core::HRESULT = windows_core::HRESULT(0x8004022D_u32 as _);
pub const IMAPI_E_USERABORT: windows_core::HRESULT = windows_core::HRESULT(0x8004020D_u32 as _);
pub const IMAPI_E_WRONGDISC: windows_core::HRESULT = windows_core::HRESULT(0x80040223_u32 as _);
pub const IMAPI_E_WRONGFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80040221_u32 as _);
pub const IMAPI_FEATURE_PAGE_TYPE_AACS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(269i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_PSEUDO_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(56i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(64i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(65i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CDRW_CAV_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(39i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_ANALOG_PLAY: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(259i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MASTERING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(46i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MULTIREAD: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(29i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(30i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_RW_MEDIA_WRITE_SUPPORT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(55i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_TRACK_AT_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(45i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CORE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(1i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DISC_CONTROL_BLOCKS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(266i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(48i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_RW_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(50i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_R_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(49i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CPRM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(267i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CSS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(262i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_DASH_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(47i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(43i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_RW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(42i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R_DUAL_LAYER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(59i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(31i32);
pub const IMAPI_FEATURE_PAGE_TYPE_EMBEDDED_CHANGER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(258i32);
pub const IMAPI_FEATURE_PAGE_TYPE_ENHANCED_DEFECT_REPORTING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(41i32);
pub const IMAPI_FEATURE_PAGE_TYPE_FIRMWARE_INFORMATION: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(268i32);
pub const IMAPI_FEATURE_PAGE_TYPE_FORMATTABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(35i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HARDWARE_DEFECT_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(36i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(80i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(81i32);
pub const IMAPI_FEATURE_PAGE_TYPE_INCREMENTAL_STREAMING_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(33i32);
pub const IMAPI_FEATURE_PAGE_TYPE_LAYER_JUMP_RECORDING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(51i32);
pub const IMAPI_FEATURE_PAGE_TYPE_LOGICAL_UNIT_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(264i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MEDIA_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(265i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MICROCODE_UPDATE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(260i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MORPHING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(2i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MRW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(40i32);
pub const IMAPI_FEATURE_PAGE_TYPE_POWER_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(256i32);
pub const IMAPI_FEATURE_PAGE_TYPE_PROFILE_LIST: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(0i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_READABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(16i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(32i32);
pub const IMAPI_FEATURE_PAGE_TYPE_REAL_TIME_STREAMING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(263i32);
pub const IMAPI_FEATURE_PAGE_TYPE_REMOVABLE_MEDIUM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(3i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(38i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RIGID_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(44i32);
pub const IMAPI_FEATURE_PAGE_TYPE_SECTOR_ERASABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(34i32);
pub const IMAPI_FEATURE_PAGE_TYPE_SMART: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(257i32);
pub const IMAPI_FEATURE_PAGE_TYPE_TIMEOUT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(261i32);
pub const IMAPI_FEATURE_PAGE_TYPE_VCPS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(272i32);
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(37i32);
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_PROTECT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(4i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_APPENDABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_BLANK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_DAMAGED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1024i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_ERASE_REQUIRED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2048i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINALIZED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(16384i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINAL_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_INFORMATIONAL_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(15i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_NON_EMPTY_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4096i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_OVERWRITE_ONLY: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_RANDOMLY_WRITABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNKNOWN: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(0i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(64512i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MEDIA: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(32768i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_WRITE_PROTECTED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8192i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_CALIBRATING_POWER: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_COMPLETED: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(6i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FINALIZATION: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(5i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FORMATTING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_INITIALIZING_HARDWARE: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VALIDATING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(7i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_WRITING_DATA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(4i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(2i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(3i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(1i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(4i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(2i32);
pub const IMAPI_MEDIA_TYPE_BDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(18i32);
pub const IMAPI_MEDIA_TYPE_BDRE: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
pub const IMAPI_MEDIA_TYPE_BDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(17i32);
pub const IMAPI_MEDIA_TYPE_CDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(2i32);
pub const IMAPI_MEDIA_TYPE_CDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(1i32);
pub const IMAPI_MEDIA_TYPE_CDRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(3i32);
pub const IMAPI_MEDIA_TYPE_DISK: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(12i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(9i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(10i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(11i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(6i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(7i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(13i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(8i32);
pub const IMAPI_MEDIA_TYPE_DVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(5i32);
pub const IMAPI_MEDIA_TYPE_DVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(4i32);
pub const IMAPI_MEDIA_TYPE_HDDVDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(15i32);
pub const IMAPI_MEDIA_TYPE_HDDVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(16i32);
pub const IMAPI_MEDIA_TYPE_HDDVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(14i32);
pub const IMAPI_MEDIA_TYPE_MAX: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
pub const IMAPI_MEDIA_TYPE_UNKNOWN: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(0i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(1i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(0i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(2i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(3i32);
pub const IMAPI_MODE_PAGE_TYPE_CACHING: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(8i32);
pub const IMAPI_MODE_PAGE_TYPE_INFORMATIONAL_EXCEPTIONS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(28i32);
pub const IMAPI_MODE_PAGE_TYPE_LEGACY_CAPABILITIES: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(42i32);
pub const IMAPI_MODE_PAGE_TYPE_MRW: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(3i32);
pub const IMAPI_MODE_PAGE_TYPE_POWER_CONDITION: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(26i32);
pub const IMAPI_MODE_PAGE_TYPE_READ_WRITE_ERROR_RECOVERY: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(1i32);
pub const IMAPI_MODE_PAGE_TYPE_TIMEOUT_AND_PROTECT: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(29i32);
pub const IMAPI_MODE_PAGE_TYPE_WRITE_PARAMETERS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(5i32);
pub const IMAPI_PROFILE_TYPE_AS_MO: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(5i32);
pub const IMAPI_PROFILE_TYPE_BD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(67i32);
pub const IMAPI_PROFILE_TYPE_BD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(64i32);
pub const IMAPI_PROFILE_TYPE_BD_R_RANDOM_RECORDING: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(66i32);
pub const IMAPI_PROFILE_TYPE_BD_R_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65i32);
pub const IMAPI_PROFILE_TYPE_CDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(8i32);
pub const IMAPI_PROFILE_TYPE_CD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(9i32);
pub const IMAPI_PROFILE_TYPE_CD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(10i32);
pub const IMAPI_PROFILE_TYPE_DDCDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(32i32);
pub const IMAPI_PROFILE_TYPE_DDCD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(33i32);
pub const IMAPI_PROFILE_TYPE_DDCD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(34i32);
pub const IMAPI_PROFILE_TYPE_DVDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(16i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(17i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(19i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RW_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(20i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_LAYER_JUMP: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(22i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(21i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(27i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(26i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(42i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(43i32);
pub const IMAPI_PROFILE_TYPE_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(18i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(82i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(81i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(80i32);
pub const IMAPI_PROFILE_TYPE_INVALID: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(0i32);
pub const IMAPI_PROFILE_TYPE_MO_ERASABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(3i32);
pub const IMAPI_PROFILE_TYPE_MO_WRITE_ONCE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(4i32);
pub const IMAPI_PROFILE_TYPE_NON_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(1i32);
pub const IMAPI_PROFILE_TYPE_NON_STANDARD: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65535i32);
pub const IMAPI_PROFILE_TYPE_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(2i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_LBA: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(0i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(2i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(1i32);
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568u32;
pub const IMAPI_SECTOR_SIZE: u32 = 2048u32;
pub const IMAPI_S_BUFFER_TO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x40201_u32 as _);
pub const IMAPI_S_PROPERTIESIGNORED: windows_core::HRESULT = windows_core::HRESULT(0x40200_u32 as _);
pub const IMAPI_WRITEPROTECTED_BY_CARTRIDGE: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(2i32);
pub const IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16i32);
pub const IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(4i32);
pub const IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(8i32);
pub const IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16384i32);
pub const IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(1i32);
pub const IMMPID_CPV_AFTER__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32769i32);
pub const IMMPID_CPV_BEFORE__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32767i32);
pub const IMMPID_CP_START: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32768i32);
pub const IMMPID_MPV_AFTER__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12294i32);
pub const IMMPID_MPV_BEFORE__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12287i32);
pub const IMMPID_MPV_MESSAGE_CREATION_FLAGS: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12289i32);
pub const IMMPID_MPV_MESSAGE_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12290i32);
pub const IMMPID_MPV_STORE_DRIVER_HANDLE: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12288i32);
pub const IMMPID_MPV_TOTAL_OPEN_CONTENT_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12293i32);
pub const IMMPID_MPV_TOTAL_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12291i32);
pub const IMMPID_MPV_TOTAL_OPEN_PROPERTY_STREAM_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12292i32);
pub const IMMPID_MP_AFTER__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4153i32);
pub const IMMPID_MP_ARRIVAL_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4121i32);
pub const IMMPID_MP_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4115i32);
pub const IMMPID_MP_AUTHENTICATED_USER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4104i32);
pub const IMMPID_MP_BEFORE__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4095i32);
pub const IMMPID_MP_BINARYMIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4109i32);
pub const IMMPID_MP_CHUNKING_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4108i32);
pub const IMMPID_MP_CLIENT_AUTH_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4149i32);
pub const IMMPID_MP_CLIENT_AUTH_USER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4148i32);
pub const IMMPID_MP_CONNECTION_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4105i32);
pub const IMMPID_MP_CONNECTION_SERVER_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4134i32);
pub const IMMPID_MP_CONNECTION_SERVER_PORT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4147i32);
pub const IMMPID_MP_CONTENT_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4097i32);
pub const IMMPID_MP_CONTENT_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4145i32);
pub const IMMPID_MP_CRC_GLOBAL: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4150i32);
pub const IMMPID_MP_CRC_RECIPS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4151i32);
pub const IMMPID_MP_DEFERRED_DELIVERY_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4141i32);
pub const IMMPID_MP_DOMAIN_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4102i32);
pub const IMMPID_MP_DSN_ENVID_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4112i32);
pub const IMMPID_MP_DSN_RET_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4113i32);
pub const IMMPID_MP_EIGHTBIT_MIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4107i32);
pub const IMMPID_MP_ENCRYPTION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4146i32);
pub const IMMPID_MP_ERROR_CODE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4111i32);
pub const IMMPID_MP_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4117i32);
pub const IMMPID_MP_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4118i32);
pub const IMMPID_MP_FOUND_EMBEDDED_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4126i32);
pub const IMMPID_MP_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4139i32);
pub const IMMPID_MP_HELO_DOMAIN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4106i32);
pub const IMMPID_MP_HR_CAT_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4122i32);
pub const IMMPID_MP_INBOUND_MAIL_FROM_AUTH: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4152i32);
pub const IMMPID_MP_LOCAL_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4119i32);
pub const IMMPID_MP_LOCAL_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4120i32);
pub const IMMPID_MP_MESSAGE_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4116i32);
pub const IMMPID_MP_MSGCLASS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4144i32);
pub const IMMPID_MP_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4123i32);
pub const IMMPID_MP_MSG_SIZE_HINT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4127i32);
pub const IMMPID_MP_NUM_RECIPIENTS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4137i32);
pub const IMMPID_MP_ORIGINAL_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4143i32);
pub const IMMPID_MP_PICKUP_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4103i32);
pub const IMMPID_MP_RECIPIENT_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4096i32);
pub const IMMPID_MP_REMOTE_AUTHENTICATION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4110i32);
pub const IMMPID_MP_REMOTE_SERVER_DSN_CAPABLE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4114i32);
pub const IMMPID_MP_RFC822_BCC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4133i32);
pub const IMMPID_MP_RFC822_CC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4132i32);
pub const IMMPID_MP_RFC822_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4130i32);
pub const IMMPID_MP_RFC822_MSG_ID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4128i32);
pub const IMMPID_MP_RFC822_MSG_SUBJECT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4129i32);
pub const IMMPID_MP_RFC822_TO_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4131i32);
pub const IMMPID_MP_SCANNED_FOR_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4125i32);
pub const IMMPID_MP_SENDER_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4140i32);
pub const IMMPID_MP_SENDER_ADDRESS_LEGACY_EX_DN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4101i32);
pub const IMMPID_MP_SENDER_ADDRESS_OTHER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4142i32);
pub const IMMPID_MP_SENDER_ADDRESS_SMTP: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4098i32);
pub const IMMPID_MP_SENDER_ADDRESS_X400: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4100i32);
pub const IMMPID_MP_SENDER_ADDRESS_X500: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4099i32);
pub const IMMPID_MP_SERVER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4135i32);
pub const IMMPID_MP_SERVER_VERSION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4136i32);
pub const IMMPID_MP_SUPERSEDES_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4124i32);
pub const IMMPID_MP_X_PRIORITY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4138i32);
pub const IMMPID_NMP_AFTER__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24585i32);
pub const IMMPID_NMP_BEFORE__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24575i32);
pub const IMMPID_NMP_HEADERS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24582i32);
pub const IMMPID_NMP_NEWSGROUP_LIST: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24581i32);
pub const IMMPID_NMP_NNTP_APPROVED_HEADER: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24584i32);
pub const IMMPID_NMP_NNTP_PROCESSING: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24583i32);
pub const IMMPID_NMP_POST_TOKEN: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24580i32);
pub const IMMPID_NMP_PRIMARY_ARTID: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24579i32);
pub const IMMPID_NMP_PRIMARY_GROUP: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24578i32);
pub const IMMPID_NMP_SECONDARY_ARTNUM: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24577i32);
pub const IMMPID_NMP_SECONDARY_GROUPS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24576i32);
pub const IMMPID_RPV_AFTER__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16386i32);
pub const IMMPID_RPV_BEFORE__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16383i32);
pub const IMMPID_RPV_DONT_DELIVER: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16384i32);
pub const IMMPID_RPV_NO_NAME_COLLISIONS: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16385i32);
pub const IMMPID_RP_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8195i32);
pub const IMMPID_RP_ADDRESS_OTHER: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8211i32);
pub const IMMPID_RP_ADDRESS_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8201i32);
pub const IMMPID_RP_ADDRESS_TYPE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8194i32);
pub const IMMPID_RP_ADDRESS_TYPE_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8196i32);
pub const IMMPID_RP_ADDRESS_X400: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8202i32);
pub const IMMPID_RP_ADDRESS_X500: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8203i32);
pub const IMMPID_RP_AFTER__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8213i32);
pub const IMMPID_RP_BEFORE__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8191i32);
pub const IMMPID_RP_DISPLAY_NAME: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8212i32);
pub const IMMPID_RP_DOMAIN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8210i32);
pub const IMMPID_RP_DSN_NOTIFY_INVALID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8193i32);
pub const IMMPID_RP_DSN_NOTIFY_SUCCESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8192i32);
pub const IMMPID_RP_DSN_NOTIFY_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8199i32);
pub const IMMPID_RP_DSN_ORCPT_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8200i32);
pub const IMMPID_RP_DSN_PRE_CAT_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8207i32);
pub const IMMPID_RP_ERROR_CODE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8197i32);
pub const IMMPID_RP_ERROR_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8198i32);
pub const IMMPID_RP_LEGACY_EX_DN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8204i32);
pub const IMMPID_RP_MDB_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8208i32);
pub const IMMPID_RP_RECIPIENT_FLAGS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8205i32);
pub const IMMPID_RP_SMTP_STATUS_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8206i32);
pub const IMMPID_RP_USER_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8209i32);
pub const MEDIA_BLANK: MEDIA_FLAGS = MEDIA_FLAGS(1i32);
pub const MEDIA_CDDA_CDROM: MEDIA_TYPES = MEDIA_TYPES(1i32);
pub const MEDIA_CD_EXTRA: MEDIA_TYPES = MEDIA_TYPES(4i32);
pub const MEDIA_CD_I: MEDIA_TYPES = MEDIA_TYPES(3i32);
pub const MEDIA_CD_OTHER: MEDIA_TYPES = MEDIA_TYPES(5i32);
pub const MEDIA_CD_ROM_XA: MEDIA_TYPES = MEDIA_TYPES(2i32);
pub const MEDIA_FORMAT_UNUSABLE_BY_IMAPI: MEDIA_FLAGS = MEDIA_FLAGS(8i32);
pub const MEDIA_RW: MEDIA_FLAGS = MEDIA_FLAGS(2i32);
pub const MEDIA_SPECIAL: MEDIA_TYPES = MEDIA_TYPES(6i32);
pub const MEDIA_WRITABLE: MEDIA_FLAGS = MEDIA_FLAGS(4i32);
pub const MPV_INBOUND_CUTOFF_EXCEEDED: u32 = 1u32;
pub const MPV_WRITE_CONTENT: u32 = 2u32;
pub const MP_MSGCLASS_DELIVERY_REPORT: u32 = 3u32;
pub const MP_MSGCLASS_NONDELIVERY_REPORT: u32 = 4u32;
pub const MP_MSGCLASS_REPLICATION: u32 = 2u32;
pub const MP_MSGCLASS_SYSTEM: u32 = 1u32;
pub const MP_STATUS_ABANDON_DELIVERY: u32 = 6u32;
pub const MP_STATUS_ABORT_DELIVERY: u32 = 2u32;
pub const MP_STATUS_BAD_MAIL: u32 = 3u32;
pub const MP_STATUS_CATEGORIZED: u32 = 5u32;
pub const MP_STATUS_RETRY: u32 = 1u32;
pub const MP_STATUS_SUBMITTED: u32 = 4u32;
pub const MP_STATUS_SUCCESS: u32 = 0u32;
pub const NMP_PROCESS_CONTROL: u32 = 2u32;
pub const NMP_PROCESS_MODERATOR: u32 = 4u32;
pub const NMP_PROCESS_POST: u32 = 1u32;
pub const PlatformEFI: PlatformId = PlatformId(239i32);
pub const PlatformMac: PlatformId = PlatformId(2i32);
pub const PlatformPowerPC: PlatformId = PlatformId(1i32);
pub const PlatformX86: PlatformId = PlatformId(0i32);
pub const RECORDER_BURNING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(2u32);
pub const RECORDER_CDR: RECORDER_TYPES = RECORDER_TYPES(1i32);
pub const RECORDER_CDRW: RECORDER_TYPES = RECORDER_TYPES(2i32);
pub const RECORDER_DOING_NOTHING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(0u32);
pub const RECORDER_OPENED: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(1u32);
pub const RP_DELIVERED: u32 = 272u32;
pub const RP_DSN_HANDLED: u32 = 64u32;
pub const RP_DSN_NOTIFY_DELAY: u32 = 67108864u32;
pub const RP_DSN_NOTIFY_FAILURE: u32 = 33554432u32;
pub const RP_DSN_NOTIFY_INVALID: u32 = 0u32;
pub const RP_DSN_NOTIFY_MASK: u32 = 251658240u32;
pub const RP_DSN_NOTIFY_NEVER: u32 = 134217728u32;
pub const RP_DSN_NOTIFY_SUCCESS: u32 = 16777216u32;
pub const RP_DSN_SENT_DELAYED: u32 = 16384u32;
pub const RP_DSN_SENT_DELIVERED: u32 = 131136u32;
pub const RP_DSN_SENT_EXPANDED: u32 = 32832u32;
pub const RP_DSN_SENT_NDR: u32 = 1104u32;
pub const RP_DSN_SENT_RELAYED: u32 = 65600u32;
pub const RP_ENPANDED: u32 = 8208u32;
pub const RP_ERROR_CONTEXT_CAT: u32 = 2097152u32;
pub const RP_ERROR_CONTEXT_MTA: u32 = 4194304u32;
pub const RP_ERROR_CONTEXT_STORE: u32 = 1048576u32;
pub const RP_EXPANDED: u32 = 8208u32;
pub const RP_FAILED: u32 = 2096u32;
pub const RP_GENERAL_FAILURE: u32 = 32u32;
pub const RP_HANDLED: u32 = 16u32;
pub const RP_RECIP_FLAGS_RESERVED: u32 = 15u32;
pub const RP_REMOTE_MTA_NO_DSN: u32 = 524288u32;
pub const RP_UNRESOLVED: u32 = 4144u32;
pub const RP_VOLATILE_FLAGS_MASK: u32 = 4026531840u32;
pub const SZ_PROGID_SMTPCAT: windows_core::PCSTR = windows_core::s!("Smtp.Cat");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISC_RECORDER_STATE_FLAGS(pub u32);
impl windows_core::TypeKind for DISC_RECORDER_STATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISC_RECORDER_STATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISC_RECORDER_STATE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EmulationType(pub i32);
impl windows_core::TypeKind for EmulationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EmulationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EmulationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsiFileSystems(pub i32);
impl windows_core::TypeKind for FsiFileSystems {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsiFileSystems {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsiFileSystems").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsiItemType(pub i32);
impl windows_core::TypeKind for FsiItemType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsiItemType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsiItemType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_BURN_VERIFICATION_LEVEL(pub i32);
impl windows_core::TypeKind for IMAPI_BURN_VERIFICATION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_BURN_VERIFICATION_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_BURN_VERIFICATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_CD_SECTOR_TYPE(pub i32);
impl windows_core::TypeKind for IMAPI_CD_SECTOR_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_CD_SECTOR_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_CD_SECTOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(pub i32);
impl windows_core::TypeKind for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_CD_TRACK_DIGITAL_COPY_SETTING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_FEATURE_PAGE_TYPE(pub i32);
impl windows_core::TypeKind for IMAPI_FEATURE_PAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_FEATURE_PAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_FEATURE_PAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_FORMAT2_DATA_MEDIA_STATE(pub i32);
impl windows_core::TypeKind for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_DATA_MEDIA_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_FORMAT2_DATA_WRITE_ACTION(pub i32);
impl windows_core::TypeKind for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_DATA_WRITE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(pub i32);
impl windows_core::TypeKind for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(pub i32);
impl windows_core::TypeKind for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_RAW_CD_WRITE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_FORMAT2_TAO_WRITE_ACTION(pub i32);
impl windows_core::TypeKind for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_TAO_WRITE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_MEDIA_PHYSICAL_TYPE(pub i32);
impl windows_core::TypeKind for IMAPI_MEDIA_PHYSICAL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_MEDIA_PHYSICAL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_MEDIA_WRITE_PROTECT_STATE(pub i32);
impl windows_core::TypeKind for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_MEDIA_WRITE_PROTECT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_MODE_PAGE_REQUEST_TYPE(pub i32);
impl windows_core::TypeKind for IMAPI_MODE_PAGE_REQUEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_MODE_PAGE_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_MODE_PAGE_TYPE(pub i32);
impl windows_core::TypeKind for IMAPI_MODE_PAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_MODE_PAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_MODE_PAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_PROFILE_TYPE(pub i32);
impl windows_core::TypeKind for IMAPI_PROFILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_PROFILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_PROFILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAPI_READ_TRACK_ADDRESS_TYPE(pub i32);
impl windows_core::TypeKind for IMAPI_READ_TRACK_ADDRESS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAPI_READ_TRACK_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMMPID_CPV_ENUM(pub i32);
impl windows_core::TypeKind for IMMPID_CPV_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMMPID_CPV_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMMPID_CPV_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMMPID_MPV_ENUM(pub i32);
impl windows_core::TypeKind for IMMPID_MPV_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMMPID_MPV_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMMPID_MPV_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMMPID_MP_ENUM(pub i32);
impl windows_core::TypeKind for IMMPID_MP_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMMPID_MP_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMMPID_MP_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMMPID_NMP_ENUM(pub i32);
impl windows_core::TypeKind for IMMPID_NMP_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMMPID_NMP_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMMPID_NMP_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMMPID_RPV_ENUM(pub i32);
impl windows_core::TypeKind for IMMPID_RPV_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMMPID_RPV_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMMPID_RPV_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMMPID_RP_ENUM(pub i32);
impl windows_core::TypeKind for IMMPID_RP_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMMPID_RP_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMMPID_RP_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MEDIA_FLAGS(pub i32);
impl windows_core::TypeKind for MEDIA_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MEDIA_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MEDIA_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MEDIA_TYPES(pub i32);
impl windows_core::TypeKind for MEDIA_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MEDIA_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MEDIA_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlatformId(pub i32);
impl windows_core::TypeKind for PlatformId {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlatformId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlatformId").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RECORDER_TYPES(pub i32);
impl windows_core::TypeKind for RECORDER_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RECORDER_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RECORDER_TYPES").field(&self.0).finish()
    }
}
pub const BlockRange: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca27_2204_11dd_966a_001aa01bbc58);
pub const BlockRangeList: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca28_2204_11dd_966a_001aa01bbc58);
pub const BootOptions: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fce_975b_59be_a960_9a2a262853a5);
pub const EnumFsiItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc6_975b_59be_a960_9a2a262853a5);
pub const EnumProgressItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fca_975b_59be_a960_9a2a262853a5);
pub const FileSystemImageResult: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcc_975b_59be_a960_9a2a262853a5);
pub const FsiDirectoryItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc8_975b_59be_a960_9a2a262853a5);
pub const FsiFileItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc7_975b_59be_a960_9a2a262853a5);
pub const FsiNamedStreams: windows_core::GUID = windows_core::GUID::from_u128(0xc6b6f8ed_6d19_44b4_b539_b159b793a32d);
pub const FsiStream: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcd_975b_59be_a960_9a2a262853a5);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMMP_MPV_STORE_DRIVER_HANDLE {
    pub guidSignature: windows_core::GUID,
}
impl windows_core::TypeKind for IMMP_MPV_STORE_DRIVER_HANDLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMSGSESS(pub isize);
impl Default for LPMSGSESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LPMSGSESS {
    type TypeKind = windows_core::CopyType;
}
pub const MSDiscMasterObj: windows_core::GUID = windows_core::GUID::from_u128(0x520cca63_51a5_11d3_9144_00104ba11c5e);
pub const MSDiscRecorderObj: windows_core::GUID = windows_core::GUID::from_u128(0x520cca61_51a5_11d3_9144_00104ba11c5e);
pub const MSEnumDiscRecordersObj: windows_core::GUID = windows_core::GUID::from_u128(0x8a03567a_63cb_4ba8_baf6_52119816d1ef);
pub const MsftDiscFormat2Data: windows_core::GUID = windows_core::GUID::from_u128(0x2735412a_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2Erase: windows_core::GUID = windows_core::GUID::from_u128(0x2735412b_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2RawCD: windows_core::GUID = windows_core::GUID::from_u128(0x27354128_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2TrackAtOnce: windows_core::GUID = windows_core::GUID::from_u128(0x27354129_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscMaster2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412e_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscRecorder2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412d_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftFileSystemImage: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc5_975b_59be_a960_9a2a262853a5);
pub const MsftIsoImageManager: windows_core::GUID = windows_core::GUID::from_u128(0xceee3b62_8f56_4056_869b_ef16917e3efc);
pub const MsftMultisessionRandomWrite: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca24_2204_11dd_966a_001aa01bbc58);
pub const MsftMultisessionSequential: windows_core::GUID = windows_core::GUID::from_u128(0x27354122_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftRawCDImageCreator: windows_core::GUID = windows_core::GUID::from_u128(0x25983561_9d65_49ce_b335_40630d901227);
pub const MsftStreamConcatenate: windows_core::GUID = windows_core::GUID::from_u128(0x27354125_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamInterleave: windows_core::GUID = windows_core::GUID::from_u128(0x27354124_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamPrng001: windows_core::GUID = windows_core::GUID::from_u128(0x27354126_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamZero: windows_core::GUID = windows_core::GUID::from_u128(0x27354127_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteEngine2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412c_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteSpeedDescriptor: windows_core::GUID = windows_core::GUID::from_u128(0x27354123_7f64_5b0f_8f00_5d77afbe261e);
pub const ProgressItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcb_975b_59be_a960_9a2a262853a5);
pub const ProgressItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc9_975b_59be_a960_9a2a262853a5);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPropAttrArray {
    pub cValues: u32,
    pub aPropAttr: [u32; 1],
}
impl windows_core::TypeKind for SPropAttrArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPropAttrArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const tagIMMPID_CPV_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0xa2a76b2a_e52d_11d1_aa64_00c04fa35b82);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct tagIMMPID_GUIDLIST_ITEM {
    pub pguid: *const windows_core::GUID,
    pub dwStart: u32,
    pub dwLast: u32,
}
impl windows_core::TypeKind for tagIMMPID_GUIDLIST_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for tagIMMPID_GUIDLIST_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const tagIMMPID_MPV_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0xcbe69706_c9bd_11d1_9ff2_00c04fa37348);
pub const tagIMMPID_MP_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0x13384cf0_b3c4_11d1_aa92_00aa006bc80b);
pub const tagIMMPID_NMP_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0x7433a9aa_20e2_11d2_94d6_00c04fa379f1);
pub const tagIMMPID_RPV_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0x79e82049_d320_11d1_9ff4_00c04fa37348);
pub const tagIMMPID_RP_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0x79e82048_d320_11d1_9ff4_00c04fa37348);
#[cfg(feature = "Win32_System_AddressBook")]
pub type MSGCALLRELEASE = Option<unsafe extern "system" fn(ulcallerdata: u32, lpmessage: Option<super::super::System::AddressBook::IMessage>)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
